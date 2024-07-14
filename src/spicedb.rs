pub use crate::generated::authzed::api::v1::*;
use crate::{Relation, Subject};

use self::subject_filter::RelationFilter;

pub fn subject_filter<S>(id: Option<S::Id>, relation: Option<S::Relations>) -> SubjectFilter
where
    S: Subject,
{
    SubjectFilter {
        subject_type: S::object_type().into(),
        optional_subject_id: id.map(Into::into).unwrap_or_default(),
        optional_relation: relation.map(|r| RelationFilter {
            relation: r.name().to_owned(),
        }),
    }
}
