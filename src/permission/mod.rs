mod check_permission;
mod delete_relationships;
mod lookup_resources;
mod lookup_subjects;
mod read_relationships;
mod write_relationships;

use crate::grpc::AuthenticatedChannel;
use crate::spicedb;

pub type SpiceDBPermissionClient =
    spicedb::permissions_service_client::PermissionsServiceClient<AuthenticatedChannel>;

pub use check_permission::CheckPermissionRequest;
pub use delete_relationships::DeleteRelationshipsRequest;
pub use lookup_resources::LookupResourcesRequest;
pub use lookup_subjects::LookupSubjectsRequest;
pub use read_relationships::ReadRelationshipsRequest;
pub use write_relationships::WriteRelationshipsRequest;
