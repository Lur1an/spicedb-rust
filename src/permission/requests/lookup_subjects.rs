use crate::permission::PermissionServiceClient;
use crate::spicedb;

#[derive(Clone, Debug)]
pub struct LookupSubjectsRequest {
    client: PermissionServiceClient,
    request: spicedb::LookupSubjectsRequest,
}

impl LookupSubjectsRequest {
    pub fn new(client: PermissionServiceClient) -> Self {
        let request = spicedb::LookupSubjectsRequest {
            ..Default::default()
        };
        LookupSubjectsRequest { client, request }
    }
}
