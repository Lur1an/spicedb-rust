use crate::permission::PermissionServiceClient;
use crate::spicedb::{object_reference, subject_reference, subject_reference_raw};
use crate::{spicedb, Caveat, Entity, Relation, RelationshipOperation, Resource, WildCardId};

use self::spicedb::precondition::Operation;
use self::spicedb::Precondition;

#[derive(Clone, Debug)]
pub struct WriteRelationshipsRequest {
    client: PermissionServiceClient,
    request: spicedb::WriteRelationshipsRequest,
}

impl WriteRelationshipsRequest {
    pub fn new(client: PermissionServiceClient) -> Self {
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
        let subject = subject_reference_raw(WildCardId, S::object_type(), None::<String>);
        let resource = object_reference::<R>(Into::<R::Id>::into(resource_id));
        self.request.updates.push(spicedb::RelationshipUpdate {
            operation: operation as i32,
            relationship: Some(spicedb::Relationship {
                resource: Some(resource),
                relation: relation.name().to_owned(),
                subject: Some(subject),
                optional_caveat: None,
            }),
        });
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
        let subject = subject_reference::<S>(Into::<S::Id>::into(subject_id), subject_relation);
        let resource = object_reference::<R>(Into::<R::Id>::into(resource_id));
        self.request.updates.push(spicedb::RelationshipUpdate {
            operation: operation as i32,
            relationship: Some(spicedb::Relationship {
                resource: Some(resource),
                relation: relation.name().to_owned(),
                subject: Some(subject),
                optional_caveat: None,
            }),
        });
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
        let subject = subject_reference::<S>(Into::<S::Id>::into(subject_id), subject_relation);
        let resource = object_reference::<R>(Into::<R::Id>::into(resource_id));
        self.request.updates.push(spicedb::RelationshipUpdate {
            operation: operation as i32,
            relationship: Some(spicedb::Relationship {
                resource: Some(resource),
                relation: relation.name().to_owned(),
                subject: Some(subject),
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
