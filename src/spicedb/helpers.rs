use crate::{Entity, Relation, Resource, Subject};

use super::subject_filter::RelationFilter;
use super::{ObjectReference, Precondition, RelationshipFilter, SubjectFilter, SubjectReference};

pub fn subject_filter<S>(id: Option<S::Id>, relation: Option<S::Relations>) -> SubjectFilter
where
    S: Subject,
{
    subject_filter_raw(S::object_type(), id, relation.map(|r| r.name()))
}

pub fn subject_filter_raw(
    subject_type: impl Into<String>,
    id: Option<impl Into<String>>,
    relation: Option<impl Into<String>>,
) -> SubjectFilter {
    SubjectFilter {
        subject_type: subject_type.into(),
        optional_subject_id: id.map(Into::into).unwrap_or_default(),
        optional_relation: relation.map(|r| RelationFilter { relation: r.into() }),
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
    relationship_filter_raw(
        R::object_type(),
        resource_id,
        resource_id_prefix,
        relation.map(|r| r.name()),
        subject_filter,
    )
}

pub fn relationship_filter_raw(
    resource_type: impl Into<String>,
    resource_id: Option<impl Into<String>>,
    resource_id_prefix: Option<impl Into<String>>,
    relation: Option<impl Into<String>>,
    subject_filter: Option<SubjectFilter>,
) -> RelationshipFilter {
    RelationshipFilter {
        resource_type: resource_type.into(),
        optional_resource_id: resource_id.map(Into::into).unwrap_or_default(),
        optional_resource_id_prefix: resource_id_prefix.map(Into::into).unwrap_or_default(),
        optional_relation: relation.map(Into::into).unwrap_or_default(),
        optional_subject_filter: subject_filter,
    }
}

pub fn precondition<R>(
    operation: super::precondition::Operation,
    resource_id: Option<R::Id>,
    resource_id_prefix: Option<String>,
    relation: Option<R::Relations>,
    subject_filter: Option<SubjectFilter>,
) -> Precondition
where
    R: Resource,
{
    precondition_raw(
        operation,
        R::object_type(),
        resource_id,
        resource_id_prefix,
        relation.map(|r| r.name()),
        subject_filter,
    )
}

pub fn precondition_raw(
    operation: super::precondition::Operation,
    resource_type: impl Into<String>,
    resource_id: Option<impl Into<String>>,
    resource_id_prefix: Option<impl Into<String>>,
    relation: Option<impl Into<String>>,
    subject_filter: Option<SubjectFilter>,
) -> Precondition {
    Precondition {
        operation: operation as i32,
        filter: Some(relationship_filter_raw(
            resource_type,
            resource_id,
            resource_id_prefix,
            relation,
            subject_filter,
        )),
    }
}

pub fn subject_reference<S>(id: S::Id, relation: Option<S::Relations>) -> SubjectReference
where
    S: Subject,
{
    subject_reference_raw(id, S::object_type(), relation.map(|r| r.name()))
}

pub fn subject_reference_raw(
    id: impl Into<String>,
    object_type: impl Into<String>,
    relation: Option<impl Into<String>>,
) -> SubjectReference {
    SubjectReference {
        object: Some(ObjectReference {
            object_type: object_type.into(),
            object_id: id.into(),
        }),
        optional_relation: relation.map(Into::into).unwrap_or_default(),
    }
}

pub fn object_reference<E>(id: E::Id) -> ObjectReference
where
    E: Entity,
{
    ObjectReference {
        object_type: E::object_type().into(),
        object_id: id.into(),
    }
}
