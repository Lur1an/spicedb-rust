use crate::grpc::AuthenticatedChannel;
use crate::spicedb;

pub type SpiceDBSchemaClient =
    spicedb::schema_service_client::SchemaServiceClient<AuthenticatedChannel>;
