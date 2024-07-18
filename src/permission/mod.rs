mod requests;

use self::requests::{
    CheckPermissionRequest, DeleteRelationshipsRequest, LookupResourcesRequest,
    ReadRelationshipsRequest, WriteRelationshipsRequest,
};
use crate::entity::Resource;
use crate::grpc::{AuthenticatedChannel, BearerTokenInterceptor, GrpcResult};
use crate::spicedb::permissions_service_client::PermissionsServiceClient;
use crate::spicedb::{self, object_reference};
use crate::{Actor, Permission};
use tonic::transport::Channel;

#[derive(Clone, Debug)]
pub struct PermissionServiceClient {
    inner: PermissionsServiceClient<AuthenticatedChannel>,
}

impl PermissionServiceClient {
    pub fn new(channel: Channel, interceptor: BearerTokenInterceptor) -> Self {
        let inner = PermissionsServiceClient::with_interceptor(channel, interceptor);
        PermissionServiceClient { inner }
    }

    pub fn raw(&self) -> PermissionsServiceClient<AuthenticatedChannel> {
        self.inner.clone()
    }

    pub fn create_relationships_request(&self) -> WriteRelationshipsRequest {
        WriteRelationshipsRequest::new(self.clone())
    }

    pub fn delete_relationships_reuqest<R>(&self) -> DeleteRelationshipsRequest<R>
    where
        R: Resource,
    {
        DeleteRelationshipsRequest::new(self.clone())
    }

    pub fn read_relationships_request(&self) -> ReadRelationshipsRequest {
        ReadRelationshipsRequest::new(self.clone())
    }

    pub fn check_permission_request(&self) -> CheckPermissionRequest {
        CheckPermissionRequest::new(self.clone())
    }

    pub fn lookup_resources_request<R>(&self) -> LookupResourcesRequest<R>
    where
        R: Resource,
    {
        LookupResourcesRequest::new(self.clone())
    }

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
        request.collect_ids().await
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
        let mut request = self.check_permission_request();
        request.subject(actor.to_subject());
        request.resource(object_reference::<R>(resource_id.into()));
        request.permission(permission.name());
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
        let mut request = self.check_permission_request();
        request.subject(actor.to_subject());
        request.resource(object_reference::<R>(resource_id.into()));
        request.permission(permission.name());
        request.consistency(spicedb::wrappers::Consistency::AtLeastAsFresh(token));
        let resp = request.send().await?;
        Ok(resp.permissionship
            == spicedb::check_permission_response::Permissionship::HasPermission as i32)
    }
}
