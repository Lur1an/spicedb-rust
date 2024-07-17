#[allow(clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}

mod client;
mod entity;
mod grpc;
pub mod spicedb;

#[cfg(feature = "permission")]
mod permission;

#[cfg(feature = "schema")]
mod schema;

pub use client::*;
pub use entity::*;
pub use spicedb::relationship_update::Operation as RelationshipOperation;
