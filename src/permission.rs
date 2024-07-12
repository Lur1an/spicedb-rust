use crate::entity::{Caveat, Relation, Resource, Subject};
use crate::grpc::AuthenticatedChannel;
use crate::spicedb;
use crate::spicedb::permissions_service_client::PermissionsServiceClient;

use self::spicedb::delete_relationships_response::DeletionProgress;
use self::spicedb::relationship_update::Operation;

#[derive(Clone, Debug)]
pub struct PermissionServiceClient {
    inner: PermissionsServiceClient<AuthenticatedChannel>,
}

#[derive(Clone, Debug)]
pub struct WriteRelationshipsRequest {
    client: PermissionServiceClient,
    request: spicedb::WriteRelationshipsRequest,
}

impl WriteRelationshipsRequest {
    pub fn add_relationship<S, R>(
        &mut self,
        operation: Operation,
        subject_id: S::Id,
        subject_relation: Option<S::Relations>,
        resource_id: R::Id,
        relation: R::Relations,
    ) -> &mut Self
    where
        S: Subject,
        R: Resource,
    {
        self.request.updates.push(spicedb::RelationshipUpdate {
            operation: operation as i32,
            relationship: Some(spicedb::Relationship {
                resource: Some(spicedb::ObjectReference {
                    object_type: R::object_type().to_owned(),
                    object_id: resource_id.into(),
                }),
                relation: relation.name().to_owned(),
                subject: Some(spicedb::SubjectReference {
                    object: Some(spicedb::ObjectReference {
                        object_type: S::object_type().to_owned(),
                        object_id: subject_id.into(),
                    }),
                    optional_relation: subject_relation
                        .map(|r| r.name().to_owned())
                        .unwrap_or_default(),
                }),
                optional_caveat: None,
            }),
        });
        self
    }

    pub fn add_caveated_relationship<S, R, C>(
        &mut self,
        operation: Operation,
        subject_id: S::Id,
        subject_relation: Option<S::Relations>,
        resource_id: R::Id,
        relation: R::Relations,
        caveat_context: C::ContextStruct,
    ) -> &mut Self
    where
        S: Subject,
        R: Resource,
        C: Caveat,
    {
        self.request.updates.push(spicedb::RelationshipUpdate {
            operation: operation as i32,
            relationship: Some(spicedb::Relationship {
                resource: Some(spicedb::ObjectReference {
                    object_type: R::object_type().to_owned(),
                    object_id: resource_id.into(),
                }),
                relation: relation.name().to_owned(),
                subject: Some(spicedb::SubjectReference {
                    object: Some(spicedb::ObjectReference {
                        object_type: S::object_type().to_owned(),
                        object_id: subject_id.into(),
                    }),
                    optional_relation: subject_relation
                        .map(|r| r.name().to_owned())
                        .unwrap_or_default(),
                }),
                optional_caveat: Some(spicedb::ContextualizedCaveat {
                    caveat_name: C::name().to_owned(),
                    context: Some(caveat_context.into()),
                }),
            }),
        });
        self
    }

    pub async fn send(mut self) -> Result<spicedb::ZedToken, tonic::Status> {
        let resp = self
            .client
            .inner
            .write_relationships(self.request)
            .await?
            .into_inner();
        resp.written_at
            .ok_or_else(|| tonic::Status::internal("Invalid ZedToken"))
    }
}

#[derive(Clone, Debug)]
pub struct DeleteRelationshipsRequest<R>
where
    R: Resource,
{
    client: PermissionServiceClient,
    request: spicedb::DeleteRelationshipsRequest,
    _phantom: std::marker::PhantomData<R>,
}

impl<R> DeleteRelationshipsRequest<R>
where
    R: Resource,
{
    pub fn with_id(&mut self, resource_id: R::Id) -> &mut Self {
        match self.request.relationship_filter.as_mut() {
            Some(rf) => resource_id.into().clone_into(&mut rf.optional_resource_id),
            None => unreachable!(),
        }
        self
    }

    pub fn with_id_prefix(&mut self, id_prefix: String) -> &mut Self {
        match self.request.relationship_filter.as_mut() {
            Some(rf) => id_prefix.clone_into(&mut rf.optional_resource_id_prefix),
            None => unreachable!(),
        }
        self
    }

    pub fn with_relation(&mut self, relation: R::Relations) -> &mut Self {
        match self.request.relationship_filter.as_mut() {
            Some(rf) => relation.name().clone_into(&mut rf.optional_relation),
            None => unreachable!(),
        }
        self
    }

    pub fn allow_partial_deletions(&mut self) -> &mut Self {
        self.request.optional_allow_partial_deletions = true;
        self
    }

    pub fn with_limit(&mut self, limit: u32) -> &mut Self {
        self.request.optional_limit = limit;
        self
    }

    pub fn with_subject_filter(&mut self, subject_filter: spicedb::SubjectFilter) -> &mut Self {
        match self.request.relationship_filter.as_mut() {
            Some(rf) => rf.optional_subject_filter = Some(subject_filter),
            None => unreachable!(),
        }
        self
    }

    pub async fn send(
        mut self,
    ) -> Result<
        (
            spicedb::ZedToken,
            spicedb::delete_relationships_response::DeletionProgress,
        ),
        tonic::Status,
    > {
        let resp = self
            .client
            .inner
            .delete_relationships(self.request)
            .await?
            .into_inner();
        let zed_token = resp
            .deleted_at
            .ok_or_else(|| tonic::Status::internal("Invalid ZedToken"))?;
        let deletion_progress = DeletionProgress::from_i32(resp.deletion_progress)
            .ok_or_else(|| tonic::Status::internal("Invalid i32 value for DeletionProgress"))?;
        Ok((zed_token, deletion_progress))
    }
}

impl PermissionServiceClient {
    pub async fn create_relationships(&self) -> WriteRelationshipsRequest {
        WriteRelationshipsRequest {
            client: self.clone(),
            request: spicedb::WriteRelationshipsRequest {
                updates: vec![],
                optional_preconditions: vec![],
            },
        }
    }

    pub async fn delete_relationships<R>(&self) -> DeleteRelationshipsRequest<R>
    where
        R: Resource,
    {
        DeleteRelationshipsRequest {
            client: self.clone(),
            request: spicedb::DeleteRelationshipsRequest {
                relationship_filter: Some(spicedb::RelationshipFilter {
                    ..Default::default()
                }),
                optional_preconditions: vec![],
                optional_limit: 0,
                optional_allow_partial_deletions: false,
            },
            _phantom: std::marker::PhantomData,
        }
    }
}
