#[allow(clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}

mod client;
mod entity;
mod grpc;
pub mod spicedb;

mod permission;
mod schema;

pub use client::SpiceDBClient;

pub type Error = tonic::Status;

pub use strum::IntoStaticStr;

pub use entity::*;
pub use spicedb::relationship_update::Operation as RelationshipOperation;
