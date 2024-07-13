use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;

pub type AuthenticatedChannel = InterceptedService<Channel, BearerTokenInterceptor>;

#[derive(Clone, Debug)]
pub struct BearerTokenInterceptor {
    token: MetadataValue<Ascii>,
}

impl BearerTokenInterceptor {
    pub fn new(token: MetadataValue<Ascii>) -> Self {
        Self { token }
    }
}

impl Interceptor for BearerTokenInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        request
            .metadata_mut()
            .insert("authorization", self.token.clone());
        Ok(request)
    }
}
