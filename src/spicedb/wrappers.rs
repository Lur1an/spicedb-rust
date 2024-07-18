use super::consistency::Requirement;

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

/// Relationship structure without the stupid optional types due to proto3
#[derive(Clone, Debug, PartialEq)]
pub struct Relationship {
    pub resource: super::ObjectReference,
    pub relation: String,
    pub subject: SubjectReference,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReadRelationshipsResponse {
    pub zed_token: super::ZedToken,
    pub relationships: Vec<Relationship>,
    pub after_result_cursor: super::Cursor,
}
