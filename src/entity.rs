use std::str::FromStr;

use crate::spicedb;

pub trait Entity {
    type Relations: Relation;
    type Id: FromStr + Into<String>;

    fn object_type() -> &'static str;
}

pub trait Relation: FromStr {
    fn name(&self) -> &'static str;
}

pub trait Subject: Entity {}

pub trait Resource: Entity {
    type Permissions: Permission;
}

pub trait Permission {
    fn name(&self) -> &'static str;
}

pub trait Caveat {
    type ContextStruct: Into<prost_types::Struct>;
    fn name() -> &'static str;
}

pub trait Actor {
    fn to_subject(&self) -> spicedb::SubjectReference;
}

pub struct NoCaveat;

impl Caveat for NoCaveat {
    type ContextStruct = prost_types::Struct;

    fn name() -> &'static str {
        unreachable!()
    }
}

pub struct NoRelations;

impl Relation for NoRelations {
    fn name(&self) -> &'static str {
        unreachable!()
    }
}

impl FromStr for NoRelations {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unreachable!()
    }
}
