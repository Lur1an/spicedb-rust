use tokio_stream::{Stream, StreamExt};

use crate::grpc::GrpcResult;
use crate::permission::PermissionServiceClient;
use crate::spicedb;
use crate::spicedb::wrappers::{Consistency, ReadRelationshipsResponse};

#[derive(Clone, Debug)]
pub struct ReadRelationshipsRequest {
    client: PermissionServiceClient,
    request: spicedb::ReadRelationshipsRequest,
}

impl ReadRelationshipsRequest {
    pub fn new(client: PermissionServiceClient) -> Self {
        let request = spicedb::ReadRelationshipsRequest {
            ..Default::default()
        };
        ReadRelationshipsRequest { client, request }
    }

    pub fn with_limit(&mut self, limit: u32) -> &mut Self {
        self.request.optional_limit = limit;
        self
    }

    pub fn with_cursor(&mut self, cursor: spicedb::Cursor) -> &mut Self {
        self.request.optional_cursor = Some(cursor);
        self
    }

    pub fn with_relationship_filter(&mut self, filter: spicedb::RelationshipFilter) -> &mut Self {
        self.request.relationship_filter = Some(filter);
        self
    }

    pub fn with_consistency(&mut self, consistency: Consistency) -> &mut Self {
        self.request.consistency = Some(consistency.into());
        self
    }

    pub async fn send(
        mut self,
    ) -> GrpcResult<impl Stream<Item = GrpcResult<ReadRelationshipsResponse>>> {
        let resp = self
            .client
            .inner
            .read_relationships(self.request)
            .await?
            .into_inner();
        Ok(resp.map(|r| r.map(Into::into)))
    }
}
