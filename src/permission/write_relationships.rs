use crate::grpc::GrpcResult;
use crate::permission::SpiceDBPermissionClient;
use crate::spicedb::{caveated_relationship_update, wildcard_relationship_update};
use crate::{spicedb, Caveat, Entity, RelationshipOperation, Resource};

use self::spicedb::precondition::Operation;
use self::spicedb::{relationship_update, Precondition};

#[derive(Clone, Debug)]
pub struct WriteRelationshipsRequest {
    client: SpiceDBPermissionClient,
    request: spicedb::WriteRelationshipsRequest,
}

impl WriteRelationshipsRequest {
    pub fn new(client: SpiceDBPermissionClient) -> Self {
        let request = spicedb::WriteRelationshipsRequest {
            updates: vec![],
            optional_preconditions: vec![],
        };
        WriteRelationshipsRequest { client, request }
    }

    pub fn add_precondition_raw(&mut self, precondition: Precondition) -> &mut Self {
        self.request.optional_preconditions.push(precondition);
        self
    }

    pub fn add_relationship_raw(&mut self, relationship: spicedb::RelationshipUpdate) -> &mut Self {
        self.request.updates.push(relationship);
        self
    }

    pub fn add_precondition<R>(
        &mut self,
        operation: Operation,
        resource_id: Option<R::Id>,
        resource_id_prefix: Option<String>,
        relation: Option<R::Relations>,
        subject_filter: Option<spicedb::SubjectFilter>,
    ) -> &mut Self
    where
        R: Resource,
    {
        let precondition = spicedb::precondition::<R>(
            operation,
            resource_id,
            resource_id_prefix,
            relation,
            subject_filter,
        );
        self.request.optional_preconditions.push(precondition);
        self
    }

    pub fn add_wildcard_relationship<S, R>(
        &mut self,
        operation: RelationshipOperation,
        resource_id: impl Into<R::Id>,
        relation: R::Relations,
    ) -> &mut Self
    where
        S: Entity,
        R: Resource,
    {
        self.request
            .updates
            .push(wildcard_relationship_update::<S, R>(
                operation,
                resource_id,
                relation,
            ));
        self
    }

    pub fn add_relationship<S, R>(
        &mut self,
        operation: RelationshipOperation,
        subject_id: impl Into<S::Id>,
        subject_relation: Option<S::Relations>,
        resource_id: impl Into<R::Id>,
        relation: R::Relations,
    ) -> &mut Self
    where
        S: Entity,
        R: Resource,
    {
        self.request.updates.push(relationship_update::<S, R>(
            operation,
            subject_id,
            subject_relation,
            resource_id,
            relation,
        ));
        self
    }

    pub fn add_caveated_relationship<S, R, C>(
        &mut self,
        operation: RelationshipOperation,
        subject_id: impl Into<S::Id>,
        subject_relation: Option<S::Relations>,
        resource_id: impl Into<R::Id>,
        relation: R::Relations,
        caveat_context: C::ContextStruct,
    ) -> &mut Self
    where
        S: Entity,
        R: Resource,
        C: Caveat,
    {
        self.request
            .updates
            .push(caveated_relationship_update::<S, R, C>(
                operation,
                subject_id,
                subject_relation,
                resource_id,
                relation,
                caveat_context,
            ));
        self
    }

    pub async fn send(mut self) -> GrpcResult<spicedb::ZedToken> {
        let resp = self
            .client
            .write_relationships(self.request)
            .await?
            .into_inner();
        resp.written_at
            .ok_or_else(|| tonic::Status::internal("Invalid ZedToken"))
    }
}
