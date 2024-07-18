use crate::grpc::GrpcResult;
use crate::permission::PermissionServiceClient;
use crate::spicedb;
use crate::spicedb::wrappers::Consistency;

#[derive(Clone, Debug)]
pub struct CheckPermissionRequest {
    client: PermissionServiceClient,
    request: spicedb::CheckPermissionRequest,
}

impl CheckPermissionRequest {
    pub fn new(client: PermissionServiceClient) -> Self {
        let request = spicedb::CheckPermissionRequest {
            ..Default::default()
        };
        CheckPermissionRequest { client, request }
    }

    pub fn permission(&mut self, permission: impl Into<String>) -> &mut Self {
        self.request.permission = permission.into();
        self
    }

    pub fn enable_tracing(&mut self) -> &mut Self {
        self.request.with_tracing = true;
        self
    }

    pub fn context(&mut self, context: impl Into<prost_types::Struct>) -> &mut Self {
        self.request.context = Some(context.into());
        self
    }

    pub fn subject(&mut self, subject: spicedb::SubjectReference) -> &mut Self {
        self.request.subject = Some(subject);
        self
    }

    pub fn resource(&mut self, resource: spicedb::ObjectReference) -> &mut Self {
        self.request.resource = Some(resource);
        self
    }

    pub fn consistency(&mut self, consistency: Consistency) -> &mut Self {
        self.request.consistency = Some(consistency.into());
        self
    }

    pub async fn send(mut self) -> GrpcResult<spicedb::CheckPermissionResponse> {
        if self.request.resource.is_none() {
            return Err(tonic::Status::invalid_argument("resource is required"));
        }
        if self.request.permission.is_empty() {
            return Err(tonic::Status::invalid_argument("permission is required"));
        }
        if self.request.subject.is_none() {
            return Err(tonic::Status::invalid_argument("subject is required"));
        }
        let resp = self
            .client
            .inner
            .check_permission(self.request)
            .await?
            .into_inner();
        Ok(resp)
    }
}
