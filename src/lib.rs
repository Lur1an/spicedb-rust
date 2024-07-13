mod generated {
    include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}

mod entity;
mod grpc_auth;
pub mod permission;
pub mod spicedb;

pub use entity::*;
