mod requests;

use self::requests::DeleteRelationshipsRequest;
use crate::entity::Resource;
use crate::grpc::{AuthenticatedChannel, BearerTokenInterceptor};
use crate::spicedb::permissions_service_client::PermissionsServiceClient;
use requests::WriteRelationshipsRequest;
use tonic::transport::Channel;

#[derive(Clone, Debug)]
pub struct PermissionServiceClient {
    inner: PermissionsServiceClient<AuthenticatedChannel>,
}

impl PermissionServiceClient {
    pub fn new(channel: Channel, interceptor: BearerTokenInterceptor) -> Self {
        let inner = PermissionsServiceClient::with_interceptor(channel, interceptor);
        PermissionServiceClient { inner }
    }
    pub fn create_relationships(&self) -> WriteRelationshipsRequest {
        WriteRelationshipsRequest::new(self.clone())
    }

    pub fn delete_relationships<R>(&self) -> DeleteRelationshipsRequest<R>
    where
        R: Resource,
    {
        DeleteRelationshipsRequest::new(self.clone())
    }
}
