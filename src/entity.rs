use std::str::FromStr;

use crate::spicedb;

/// An entity is any object in your SpiceDB system
/// The Id type represents whatever rust type you're using internally, since SpiceDB only uses
/// Strings to avoid having to convert and deal with errors everywhere we use trait bounds `FromStr` and `Into<String>`
/// which a lot of common Id types like `Uuid` or `u32` already implement.
pub trait Entity: 'static {
    type Relations: Relation;
    type Id: FromStr + Into<String>;

    fn object_type() -> &'static str;
}

pub trait Relation {
    fn name(&self) -> &'static str;
}

/// A resource is any `Entity` that also has `Permissions` associated
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

/// Implement the Actor trait for any struct that will represent someone/something taking action in
/// your system. it could for example be an enum wrapping User/Organization/Service if those are
/// entities that can take action.
/// ```rust
/// pub enum SystemActor {
///     User(Uuid),
///     Organization(String),
///     Service(Uuid),
/// }
/// ```
pub trait Actor: 'static {
    fn to_subject(&self) -> spicedb::SubjectReference;
}

pub struct NoCaveat;

impl Caveat for NoCaveat {
    type ContextStruct = prost_types::Struct;

    fn name() -> &'static str {
        unreachable!()
    }
}

/// Use this time for `Relations` when implementing `Entity` for something that will never have a
/// any relation to.
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

/// Use this type to build your wildcard entity types.
/// Often there already are wildcard shortcuts like `add_wildcard_relationship` in the
/// `write_relationships_request` builder that can be used instead of this.
/// However for more niche usecases where you need to pass in a WildCard user for an entity in an
/// operation that currently doesn't have a shortcut, this can be implemented and used.
/// ```rust
/// pub struct WildCardUser;
///
/// impl Entity for WildCardUser {
///     type Id = WildCardId;
///     type Relations = NoRelations;
///
///     fn object_type() -> &'static str {
///         User::object_type()
///     }
/// }
/// ```
pub struct WildCardId;

impl FromStr for WildCardId {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unreachable!()
    }
}

impl From<WildCardId> for String {
    fn from(_: WildCardId) -> Self {
        "*".into()
    }
}
