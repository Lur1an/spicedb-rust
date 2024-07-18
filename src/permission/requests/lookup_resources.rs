use futures::TryStreamExt;
use tokio_stream::{Stream, StreamExt};

use crate::grpc::GrpcResult;
use crate::permission::PermissionServiceClient;
use crate::spicedb::wrappers::{Consistency, LookupResourcesResponse};
use crate::{spicedb, Actor, Permission, Resource};

#[derive(Clone, Debug)]
pub struct LookupResourcesRequest<R> {
    client: PermissionServiceClient,
    request: spicedb::LookupResourcesRequest,
    _phantom: std::marker::PhantomData<R>,
}

impl<R> LookupResourcesRequest<R>
where
    R: Resource,
{
    pub fn new(client: PermissionServiceClient) -> Self {
        let request = spicedb::LookupResourcesRequest {
            resource_object_type: R::object_type().into(),
            ..Default::default()
        };
        LookupResourcesRequest {
            client,
            request,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn permission(&mut self, permission: R::Permissions) -> &mut Self {
        self.request.permission = permission.name().into();
        self
    }

    pub fn actor(&mut self, actor: &impl Actor) -> &mut Self {
        self.request.subject = Some(actor.to_subject());
        self
    }

    pub fn subject(&mut self, subject: spicedb::SubjectReference) -> &mut Self {
        self.request.subject = Some(subject);
        self
    }

    pub fn with_context(&mut self, context: impl Into<prost_types::Struct>) -> &mut Self {
        self.request.context = Some(context.into());
        self
    }

    pub fn with_consistency(&mut self, consistency: Consistency) -> &mut Self {
        self.request.consistency = Some(consistency.into());
        self
    }

    pub fn with_limit(&mut self, limit: u32) -> &mut Self {
        self.request.optional_limit = limit;
        self
    }

    pub fn with_cursor(&mut self, cursor: spicedb::Cursor) -> &mut Self {
        self.request.optional_cursor = Some(cursor);
        self
    }

    /// Sends the request and collects all the IDs from the response objects
    pub async fn send_collect_ids(self) -> GrpcResult<Vec<R::Id>> {
        self.send_stream()
            .await?
            .map(|resp| resp.map(|r| r.id))
            .try_collect()
            .await
    }

    pub async fn send_stream(
        mut self,
    ) -> GrpcResult<impl Stream<Item = GrpcResult<LookupResourcesResponse<R::Id>>>> {
        if self.request.permission.is_empty() {
            return Err(tonic::Status::invalid_argument("permission is required"));
        }
        if self.request.subject.is_none() {
            return Err(tonic::Status::invalid_argument("subject is required"));
        }
        let resp = self
            .client
            .inner
            .lookup_resources(self.request)
            .await?
            .into_inner();
        Ok(resp.map(|r| {
            r.and_then(|r| {
                let id = r.resource_object_id.parse().map_err(|_| {
                    let expected_type = std::any::type_name::<R::Id>();
                    tonic::Status::internal(format!(
                        "Could not parse Id: {} from LookupResourcesResponse, expected a value to be parsed as {}",
                        r.resource_object_id, expected_type
                    ))
                })?;
                let missing_caveats = r
                    .partial_caveat_info
                    .map(|p| p.missing_required_context)
                    .unwrap_or_default();
                let response = LookupResourcesResponse::<R::Id> {
                    id,
                    looked_up_at: r.looked_up_at,
                    permissionship: spicedb::LookupPermissionship::try_from(r.permissionship)
                        .unwrap(),
                    missing_caveats,
                    after_result_cursor: r.after_result_cursor,
                };
                Ok(response)
            })
        }))
    }
}
