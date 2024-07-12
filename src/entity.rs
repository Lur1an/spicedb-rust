pub trait Entity {
    type Relations: Relation;
    type Id: TryFrom<String> + Into<String>;

    fn object_type() -> &'static str;
}

pub trait Relation {
    fn name(&self) -> &'static str;
}

pub trait Subject: Entity {}

pub trait Resource: Entity {
    type Permissions: Permission;
}

pub trait Permission {
    fn name() -> &'static str;
}

pub trait Caveat {
    type ContextStruct: Into<prost_types::Struct>;
    fn name() -> &'static str;
}

pub struct NoCaveat;

impl Caveat for NoCaveat {
    type ContextStruct = prost_types::Struct;

    fn name() -> &'static str {
        unreachable!()
    }
}
