use crate::permission::PermissionServiceClient;
use crate::{spicedb, Caveat, Relation, Resource, Subject};
use crate::{Entity, RelationshipOperation};

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
        // no Into type for resource_id otherwise it would need to be specified every time we pass in `None`
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

    pub fn add_relationship<S, R>(
        &mut self,
        operation: RelationshipOperation,
        subject_id: impl Into<S::Id>,
        subject_relation: Option<S::Relations>,
        resource_id: impl Into<R::Id>,
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
                    object_id: Into::<R::Id>::into(resource_id).into(),
                }),
                relation: relation.name().to_owned(),
                subject: Some(spicedb::SubjectReference {
                    object: Some(spicedb::ObjectReference {
                        object_type: S::object_type().to_owned(),
                        object_id: Into::<S::Id>::into(subject_id).into(),
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
        operation: RelationshipOperation,
        subject_id: impl Into<S::Id>,
        subject_relation: Option<S::Relations>,
        resource_id: impl Into<R::Id>,
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
                    object_id: Into::<R::Id>::into(resource_id).into(),
                }),
                relation: relation.name().to_owned(),
                subject: Some(spicedb::SubjectReference {
                    object: Some(spicedb::ObjectReference {
                        object_type: S::object_type().to_owned(),
                        object_id: Into::<S::Id>::into(subject_id).into(),
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
