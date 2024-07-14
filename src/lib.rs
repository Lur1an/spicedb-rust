#[allow(clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}

mod client;
mod entity;
mod grpc;

#[cfg(feature = "permission")]
mod permission;
#[cfg(feature = "schema")]
mod schema;

pub mod spicedb;
pub use client::*;
pub use entity::*;
pub use spicedb::relationship_update::Operation as RelationshipOperation;
