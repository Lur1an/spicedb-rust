use crate::permission::SpiceDBPermissionClient;
use crate::spicedb;

#[derive(Clone, Debug)]
pub struct LookupSubjectsRequest {
    client: SpiceDBPermissionClient,
    request: spicedb::LookupSubjectsRequest,
}

impl LookupSubjectsRequest {
    pub fn new(client: SpiceDBPermissionClient) -> Self {
        let request = spicedb::LookupSubjectsRequest {
            ..Default::default()
        };
        LookupSubjectsRequest { client, request }
    }
}
