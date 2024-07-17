use crate::grpc::{AuthenticatedChannel, BearerTokenInterceptor};
use crate::spicedb;
use crate::spicedb::schema_service_client::SchemaServiceClient;
use tonic::transport::Channel;

#[derive(Clone, Debug)]
pub struct Client {
    inner: SchemaServiceClient<AuthenticatedChannel>,
}

#[derive(Clone, Debug)]
pub struct ReadSchemaResponse {
    pub schema_text: String,
    pub read_at: spicedb::ZedToken,
}

impl Client {
    pub fn new(channel: Channel, interceptor: BearerTokenInterceptor) -> Self {
        let inner = SchemaServiceClient::with_interceptor(channel, interceptor);
        Client { inner }
    }

    pub fn raw(&self) -> SchemaServiceClient<AuthenticatedChannel> {
        self.inner.clone()
    }

    pub async fn write_schema(
        &self,
        schema: impl Into<String>,
    ) -> Result<spicedb::ZedToken, tonic::Status> {
        let resp = self
            .inner
            .clone()
            .write_schema(spicedb::WriteSchemaRequest {
                schema: schema.into(),
            })
            .await?
            .into_inner();
        resp.written_at
            .ok_or_else(|| tonic::Status::internal("ZedToken can't be null"))
    }

    pub async fn read_schema(&self) -> Result<ReadSchemaResponse, tonic::Status> {
        let resp = self
            .inner
            .clone()
            .read_schema(spicedb::ReadSchemaRequest {})
            .await?
            .into_inner();
        let zed_token = resp
            .read_at
            .ok_or_else(|| tonic::Status::internal("Invalid ZedToken"))?;
        Ok(ReadSchemaResponse {
            schema_text: resp.schema_text,
            read_at: zed_token,
        })
    }
}
