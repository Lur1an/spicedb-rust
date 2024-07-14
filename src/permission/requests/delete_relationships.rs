use self::spicedb::delete_relationships_response::DeletionProgress;
use crate::entity::{Relation, Resource};
use crate::grpc_auth::AuthenticatedChannel;
use crate::permission::PermissionServiceClient;
use crate::spicedb;
use crate::spicedb::permissions_service_client::PermissionsServiceClient;

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
    pub fn new(client: PermissionServiceClient) -> Self {
        Self {
            client,
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
