use crate::entity::{Caveat, Relation, Resource, Subject};
use crate::{grpc, spicedb};

use self::spicedb::relationship_update::Operation;

#[derive(Clone, Debug)]
pub struct PermissionServiceClient {
    inner:
        spicedb::permissions_service_client::PermissionsServiceClient<grpc::AuthenticatedChannel>,
}

#[derive(Clone, Debug)]
pub struct WriteRelationshipsRequest {
    client: PermissionServiceClient,
    request: spicedb::WriteRelationshipsRequest,
}

impl WriteRelationshipsRequest {
    pub fn add_relationship<S, R>(
        mut self,
        operation: Operation,
        subject_id: S::Id,
        subject_relation: Option<S::Relations>,
        resource_id: R::Id,
        relation: R::Relations,
    ) -> Self
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
        mut self,
        operation: Operation,
        subject_id: S::Id,
        subject_relation: Option<S::Relations>,
        resource_id: R::Id,
        relation: R::Relations,
        caveat_context: C::ContextStruct,
    ) -> Self
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
    pub fn with_resource_id(mut self, resource_id: R::Id) -> Self {
        match self.request.relationship_filter.as_mut() {
            Some(rf) => resource_id.into().clone_into(&mut rf.optional_resource_id),
            None => unreachable!(),
        }
        self
    }
    pub fn with_relation(mut self, relation: R::Relations) -> Self {
        match self.request.relationship_filter.as_mut() {
            Some(rf) => relation.name().clone_into(&mut rf.optional_relation),
            None => unreachable!(),
        }
        self
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
}
