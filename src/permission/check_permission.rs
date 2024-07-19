use crate::grpc::GrpcResult;
use crate::permission::SpiceDBPermissionClient;
use crate::spicedb::wrappers::Consistency;
use crate::{spicedb, Actor, Permission, Resource};

#[derive(Debug)]
pub struct CheckPermissionRequest<R> {
    client: SpiceDBPermissionClient,
    request: spicedb::CheckPermissionRequest,
    _phantom: std::marker::PhantomData<R>,
}

impl<R> CheckPermissionRequest<R>
where
    R: Resource,
{
    pub fn new(client: SpiceDBPermissionClient) -> Self {
        let request = spicedb::CheckPermissionRequest {
            ..Default::default()
        };
        CheckPermissionRequest {
            client,
            request,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn permission(&mut self, permission: R::Permissions) -> &mut Self {
        permission.name().clone_into(&mut self.request.permission);
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

    pub fn actor(&mut self, actor: &impl Actor) -> &mut Self {
        self.subject(actor.to_subject())
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
            .check_permission(self.request)
            .await?
            .into_inner();
        Ok(resp)
    }
}
