use crate::grpc::BearerTokenInterceptor;

#[derive(Clone, Debug)]
pub struct SpiceDBClient {
    schema_service_client: crate::schema::SchemaServiceClient,
    permission_service_client: crate::permission::PermissionServiceClient,
}

impl SpiceDBClient {
    /// Reads the following env variables:
    /// - `ZED_TOKEN`
    /// - `ZED_ENDPOINT`
    pub async fn from_env() -> anyhow::Result<Self> {
        let token = std::env::var("ZED_TOKEN")?;
        let addr = std::env::var("ZED_ENDPOINT")?;
        Self::new(addr, token).await
    }

    pub async fn new(addr: impl Into<String>, token: impl AsRef<str>) -> anyhow::Result<Self> {
        let token = format!("Bearer {}", token.as_ref()).parse()?;
        let interceptor = BearerTokenInterceptor::new(token);
        let channel = tonic::transport::Channel::from_shared(addr.into())?
            .connect()
            .await?;
        Ok(SpiceDBClient {
            #[cfg(feature = "schema")]
            schema_service_client: crate::schema::SchemaServiceClient::new(
                channel.clone(),
                interceptor.clone(),
            ),
            #[cfg(feature = "permission")]
            permission_service_client: crate::permission::PermissionServiceClient::new(
                channel.clone(),
                interceptor,
            ),
        })
    }

    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    #[inline]
    pub fn schema_client(&self) -> &crate::schema::SchemaServiceClient {
        &self.schema_service_client
    }

    pub fn into_schema_client(self) -> crate::schema::SchemaServiceClient {
        self.schema_service_client
    }

    #[inline]
    pub fn permission_client(&self) -> &crate::permission::PermissionServiceClient {
        &self.permission_service_client
    }

    pub fn into_permission_client(self) -> crate::permission::PermissionServiceClient {
        self.permission_service_client
    }
}
