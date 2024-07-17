pub use crate::generated::authzed::api::v1::*;
use crate::{Relation, Resource, Subject};

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

pub fn relationship_filter<R>(
    resource_id: Option<R::Id>,
    resource_id_prefix: Option<String>,
    relation: Option<R::Relations>,
    subject_filter: Option<SubjectFilter>,
) -> RelationshipFilter
where
    R: Resource,
{
    RelationshipFilter {
        resource_type: R::object_type().into(),
        optional_resource_id: resource_id.map(Into::into).unwrap_or_default(),
        optional_resource_id_prefix: resource_id_prefix.unwrap_or_default(),
        optional_relation: relation.map(|r| r.name().to_owned()).unwrap_or_default(),
        optional_subject_filter: subject_filter,
    }
}
