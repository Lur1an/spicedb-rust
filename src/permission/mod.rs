mod requests;

use self::requests::DeleteRelationshipsRequest;
use crate::entity::{Resource};
use crate::grpc_auth::AuthenticatedChannel;
use crate::spicedb::permissions_service_client::PermissionsServiceClient;
use requests::WriteRelationshipsRequest;

#[derive(Clone, Debug)]
pub struct PermissionServiceClient {
    inner: PermissionsServiceClient<AuthenticatedChannel>,
}

impl PermissionServiceClient {
    pub async fn create_relationships(&self) -> WriteRelationshipsRequest {
        WriteRelationshipsRequest::new(self.clone())
    }

    pub fn delete_relationships<R>(&self) -> DeleteRelationshipsRequest<R>
    where
        R: Resource,
    {
        DeleteRelationshipsRequest::new(self.clone())
    }
}
