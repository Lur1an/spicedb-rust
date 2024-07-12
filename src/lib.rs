mod generated {
    include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}

pub mod spicedb {
    pub use crate::generated::authzed::api::v1::*;
}

pub mod entity;
pub mod grpc;
pub mod permission;
pub mod spicedb_ext;
