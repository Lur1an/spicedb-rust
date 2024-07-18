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

//#[cfg(feature = "mock")]
//pub use client::MockSpiceDBClient;
//#[cfg(not(feature = "mock"))]
//
pub use client::SpiceDBClient;

pub use entity::*;
pub use spicedb::relationship_update::Operation as RelationshipOperation;

#[cfg(all(test, feature = "mock"))]
mod test {
    use super::*;

    #[tokio::test]
    async fn ensure_mock_works() -> anyhow::Result<()> {
        let client = SpiceDBClient::default();
        Ok(())
    }
}
