use super::consistency::Requirement;
use super::{LookupPermissionship, ResolvedSubject};

/// Wrapper enum to shorten the expressions needed to construct the gRPC `Consistency` type
#[derive(Clone, Debug, PartialEq)]
pub enum Consistency {
    MinimizeLatency,
    AtLeastAsFresh(super::ZedToken),
    AtExactSnapshot(super::ZedToken),
    FullyConsistent,
}

impl From<Consistency> for super::Consistency {
    fn from(consistency: Consistency) -> Self {
        let requirement = match consistency {
            Consistency::MinimizeLatency => Requirement::MinimizeLatency(true),
            Consistency::AtLeastAsFresh(token) => Requirement::AtLeastAsFresh(token),
            Consistency::AtExactSnapshot(token) => Requirement::AtExactSnapshot(token),
            Consistency::FullyConsistent => Requirement::FullyConsistent(true),
        };
        super::Consistency {
            requirement: Some(requirement),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubjectReference {
    pub object: super::ObjectReference,
    pub optional_relation: Option<String>,
}

impl From<super::SubjectReference> for SubjectReference {
    fn from(subject: super::SubjectReference) -> Self {
        SubjectReference {
            object: subject.object.unwrap(),
            optional_relation: if subject.optional_relation.is_empty() {
                None
            } else {
                Some(subject.optional_relation)
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Relationship {
    pub resource: super::ObjectReference,
    pub relation: String,
    pub subject: SubjectReference,
    pub optional_caveat: Option<super::ContextualizedCaveat>,
}

impl From<super::Relationship> for Relationship {
    fn from(rel: super::Relationship) -> Self {
        Relationship {
            resource: rel.resource.unwrap(),
            relation: rel.relation,
            subject: rel.subject.unwrap().into(),
            optional_caveat: rel.optional_caveat,
        }
    }
}

/// Response struct without the stupid optional types due to proto3 and a `From` impl that
/// assumes the `validate` rules defined in the proto file to be upheld, otherwise it panics.
#[derive(Clone, Debug, PartialEq)]
pub struct ReadRelationshipsResponse {
    pub read_at: super::ZedToken,
    pub relationships: Vec<Relationship>,
    pub after_result_cursor: Option<super::Cursor>,
}

impl From<super::ReadRelationshipsResponse> for ReadRelationshipsResponse {
    fn from(resp: super::ReadRelationshipsResponse) -> Self {
        ReadRelationshipsResponse {
            read_at: resp.read_at.unwrap(),
            relationships: resp.relationship.into_iter().map(|r| r.into()).collect(),
            after_result_cursor: resp.after_result_cursor,
        }
    }
}

/// Wrapper struct for the LookupResourcesResponse, since it looks up all resources of a specific
/// type we can be sure that all Ids are also of the same type.
pub struct LookupResourcesResponse<Id> {
    pub id: Id,
    pub looked_up_at: Option<super::ZedToken>,
    pub permissionship: LookupPermissionship,
    pub missing_caveats: Vec<String>,
    pub after_result_cursor: Option<super::Cursor>,
}

/// Wrapper struct for the ReadSchemaResponse, with validation presuppositions applied
pub struct ReadSchemaResponse {
    pub schema_text: String,
    pub read_at: super::ZedToken,
}

impl From<super::ReadSchemaResponse> for ReadSchemaResponse {
    fn from(resp: super::ReadSchemaResponse) -> Self {
        ReadSchemaResponse {
            schema_text: resp.schema_text,
            read_at: resp.read_at.unwrap(),
        }
    }
}
