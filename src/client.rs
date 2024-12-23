use crate::grpc::{BearerTokenInterceptor, GrpcResult};
use crate::permission::{
    CheckPermissionRequest, DeleteRelationshipsRequest, LookupResourcesRequest,
    LookupSubjectsRequest, ReadRelationshipsRequest, SpiceDBPermissionClient,
    WriteRelationshipsRequest,
};
use crate::schema::SpiceDBSchemaClient;
use crate::spicedb::wrappers::{Consistency, ReadSchemaResponse};
use crate::spicedb::{self, object_reference};
use crate::{Actor, Entity, Resource};

#[derive(Clone, Debug)]
pub struct SpiceDBClient {
    schema_service_client: SpiceDBSchemaClient,
    permission_service_client: SpiceDBPermissionClient,
}

impl SpiceDBClient {
    /// Reads the following env variables:
    /// - `ZED_TOKEN`
    /// - `ZED_ENDPOINT`
    pub async fn from_env() -> anyhow::Result<Self> {
        let token = std::env::var("SPICEDB_TOKEN")?;
        let addr = std::env::var("SPICEDB_ENDPOINT")?;
        Self::new(addr, &token).await
    }

    #[cfg(any(feature = "integration-test", test))]
    pub async fn new_isolated(addr: impl Into<String>) -> anyhow::Result<Self> {
        let token = uuid::Uuid::new_v4().to_string();
        Self::new(addr, token).await
    }

    pub async fn new(addr: impl Into<String>, token: impl AsRef<str>) -> anyhow::Result<Self> {
        let token = format!("Bearer {}", token.as_ref()).parse()?;
        let interceptor = BearerTokenInterceptor::new(token);
        let channel = tonic::transport::Channel::from_shared(addr.into())?
            .connect()
            .await?;
        Ok(SpiceDBClient {
            schema_service_client:
                spicedb::schema_service_client::SchemaServiceClient::with_interceptor(
                    channel.clone(),
                    interceptor.clone(),
                ),
            permission_service_client:
                spicedb::permissions_service_client::PermissionsServiceClient::with_interceptor(
                    channel,
                    interceptor,
                ),
        })
    }

    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    pub fn schema_service_client(&self) -> SpiceDBSchemaClient {
        self.schema_service_client.clone()
    }

    pub fn permission_service_client(&self) -> SpiceDBPermissionClient {
        self.permission_service_client.clone()
    }

    pub fn create_relationships_request(&self) -> WriteRelationshipsRequest {
        WriteRelationshipsRequest::new(self.permission_service_client())
    }

    pub fn delete_relationships_request<R>(&self) -> DeleteRelationshipsRequest<R>
    where
        R: Resource,
    {
        DeleteRelationshipsRequest::new(self.permission_service_client())
    }

    pub fn read_relationships_request(&self) -> ReadRelationshipsRequest {
        ReadRelationshipsRequest::new(self.permission_service_client())
    }

    pub fn check_permission_request<R>(&self) -> CheckPermissionRequest<R>
    where
        R: Resource,
    {
        CheckPermissionRequest::new(self.permission_service_client())
    }

    pub fn lookup_resources_request<R>(&self) -> LookupResourcesRequest<R>
    where
        R: Resource,
    {
        LookupResourcesRequest::new(self.permission_service_client())
    }

    pub fn lookup_subjects_request<S, R>(&self) -> LookupSubjectsRequest<S, R>
    where
        S: Entity,
        R: Resource,
    {
        LookupSubjectsRequest::new(self.permission_service_client())
    }

    pub async fn delete_relationships<R>(
        &self,
        id: Option<R::Id>,
        relation: Option<R::Relations>,
        subject_filter: Option<spicedb::SubjectFilter>,
    ) -> GrpcResult<spicedb::ZedToken>
    where
        R: Resource,
    {
        let mut request = self.delete_relationships_request::<R>();
        if let Some(id) = id {
            request.with_id(id);
        }
        if let Some(relation) = relation {
            request.with_relation(relation);
        }
        if let Some(subject_filter) = subject_filter {
            request.with_subject_filter(subject_filter);
        }
        request.send().await.map(|resp| resp.0)
    }

    pub async fn create_relationships<R, P>(
        &self,
        relationships: R,
        preconditions: P,
    ) -> GrpcResult<spicedb::ZedToken>
    where
        R: IntoIterator<Item = spicedb::RelationshipUpdate>,
        P: IntoIterator<Item = spicedb::Precondition>,
    {
        let mut request = self.create_relationships_request();
        for precondition in preconditions {
            request.add_precondition_raw(precondition);
        }
        for relationship in relationships {
            request.add_relationship_raw(relationship);
        }
        request.send().await
    }

    /// Shortcut for the most common use case of looking up resources, to quickly collect all ID's
    /// returned in one call.
    pub async fn lookup_resources<R>(
        &self,
        actor: &impl Actor,
        permission: R::Permissions,
    ) -> GrpcResult<Vec<R::Id>>
    where
        R: Resource,
    {
        let mut request = self.lookup_resources_request::<R>();
        request.permission(permission);
        request.actor(actor);
        request.send_collect_ids().await
    }

    pub async fn lookup_resources_at<R>(
        &self,
        actor: &impl Actor,
        permission: R::Permissions,
        token: spicedb::ZedToken,
    ) -> GrpcResult<Vec<R::Id>>
    where
        R: Resource,
    {
        let mut request = self.lookup_resources_request::<R>();
        request.permission(permission);
        request.actor(actor);
        request.with_consistency(Consistency::AtLeastAsFresh(token));
        request.send_collect_ids().await
    }

    pub async fn lookup_subjects<S, R>(
        &self,
        id: impl Into<R::Id>,
        permission: R::Permissions,
    ) -> GrpcResult<Vec<S::Id>>
    where
        R: Resource,
        S: Entity,
    {
        let mut request = self.lookup_subjects_request::<S, R>();
        request.resource(id, permission);
        request.send_collect_ids().await
    }

    pub async fn lookup_subjects_at<S, R>(
        &self,
        id: impl Into<R::Id>,
        permission: R::Permissions,
        token: spicedb::ZedToken,
    ) -> GrpcResult<Vec<S::Id>>
    where
        S: Entity,
        R: Resource,
    {
        let mut request = self.lookup_subjects_request::<S, R>();
        request.resource(id, permission);
        request.with_consistency(Consistency::AtLeastAsFresh(token));
        request.send_collect_ids().await
    }

    /// Shortcut for the most common use case of checking a permission for an actor in the system
    /// on a specific resource `R` with default consistency.
    pub async fn check_permission<R>(
        &self,
        actor: &impl Actor,
        resource_id: impl Into<R::Id>,
        permission: R::Permissions,
    ) -> GrpcResult<bool>
    where
        R: Resource,
    {
        let mut request = self.check_permission_request::<R>();
        request.subject(actor.to_subject());
        request.resource(object_reference::<R>(resource_id.into()));
        request.permission(permission);
        let resp = request.send().await?;
        Ok(resp.permissionship
            == spicedb::check_permission_response::Permissionship::HasPermission as i32)
    }

    pub async fn check_permission_at<R>(
        &self,
        actor: &impl Actor,
        resource_id: impl Into<R::Id>,
        permission: R::Permissions,
        token: spicedb::ZedToken,
    ) -> GrpcResult<bool>
    where
        R: Resource,
    {
        let mut request = self.check_permission_request::<R>();
        request.subject(actor.to_subject());
        request.resource(object_reference::<R>(resource_id.into()));
        request.permission(permission);
        request.consistency(Consistency::AtLeastAsFresh(token));
        let resp = request.send().await?;
        Ok(resp.permissionship
            == spicedb::check_permission_response::Permissionship::HasPermission as i32)
    }

    pub async fn write_schema(&self, schema: String) -> Result<spicedb::ZedToken, tonic::Status> {
        let resp = self
            .schema_service_client()
            .write_schema(spicedb::WriteSchemaRequest { schema })
            .await?
            .into_inner();
        resp.written_at
            .ok_or_else(|| tonic::Status::internal("ZedToken expected"))
    }

    pub async fn read_schema(&self) -> Result<ReadSchemaResponse, tonic::Status> {
        let resp = self
            .schema_service_client()
            .read_schema(spicedb::ReadSchemaRequest {})
            .await?
            .into_inner()
            .into();
        Ok(resp)
    }
}
