use crate::grpc::GrpcResult;
use crate::permission::SpiceDBPermissionClient;
use crate::spicedb::object_reference;
use crate::spicedb::wrappers::Consistency;
use crate::{spicedb, Entity, Permission, Relation, Resource};
use futures::TryStreamExt;
use tokio_stream::{Stream, StreamExt};

use self::spicedb::LookupSubjectsResponse;

#[derive(Clone, Debug)]
pub struct LookupSubjectsRequest<S, R> {
    client: SpiceDBPermissionClient,
    request: spicedb::LookupSubjectsRequest,
    _phantom: std::marker::PhantomData<(S, R)>,
}

impl<S, R> LookupSubjectsRequest<S, R>
where
    S: Entity,
    R: Resource,
{
    pub fn new(client: SpiceDBPermissionClient) -> Self {
        let request = spicedb::LookupSubjectsRequest {
            subject_object_type: S::object_type().into(),
            ..Default::default()
        };
        LookupSubjectsRequest {
            client,
            request,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_consistency(&mut self, consistency: Consistency) -> &mut Self {
        self.request.consistency = Some(consistency.into());
        self
    }

    pub fn resource(&mut self, id: R::Id, permission: R::Permissions) -> &mut Self
    where
        R: Resource,
    {
        self.request.resource = Some(object_reference::<R>(id));
        self.request.permission = permission.name().into();
        self
    }

    pub fn subject_relation(&mut self, relation: Option<S::Relations>) -> &mut Self
    where
        S: Entity,
    {
        self.request.optional_subject_relation =
            relation.map(|r| r.name().to_owned()).unwrap_or_default();
        self
    }

    pub fn with_context(&mut self, context: impl Into<prost_types::Struct>) -> &mut Self {
        self.request.context = Some(context.into());
        self
    }

    pub fn with_concrete_limit(&mut self, limit: u32) -> &mut Self {
        self.request.optional_concrete_limit = limit;
        self
    }

    pub fn wildcards_allowed(&mut self, wildcards: bool) -> &mut Self {
        if wildcards {
            self.request.wildcard_option =
                spicedb::lookup_subjects_request::WildcardOption::IncludeWildcards as i32;
        } else {
            self.request.wildcard_option =
                spicedb::lookup_subjects_request::WildcardOption::ExcludeWildcards as i32;
        }
        self
    }

    pub async fn send_collect_ids(self) -> GrpcResult<Vec<S::Id>> {
        if self.request.wildcard_option
            == spicedb::lookup_subjects_request::WildcardOption::IncludeWildcards as i32
        {
            return Err(tonic::Status::invalid_argument(
                "Cannot call send_collect_ids on a lookup_subjects request with wildcards enabled",
            ));
        }
        self.send_stream()
            .await?
            .filter_map(|r| if let Ok(ref resp) = r {
                if resp.subject.is_none() {
                    None
                } else {
                    Some(r)
                }
            } else {
                Some(r)
            })
            .map(|resp| resp.and_then(|r| {
                let subject = r.subject.unwrap();
                let id = subject.subject_object_id.parse().map_err(|_| {
                    let expected_type = std::any::type_name::<S::Id>();
                    tonic::Status::internal(format!(
                        "Could not parse Id: {} from LookupResourcesResponse, expected a value to be parsed as {}",
                        subject.subject_object_id, expected_type
                    ))
                })?;
                Ok(id)
            }))
            .try_collect()
            .await
    }

    pub async fn send_stream(
        mut self,
    ) -> GrpcResult<impl Stream<Item = GrpcResult<LookupSubjectsResponse>>> {
        if self.request.resource.is_none() {
            return Err(tonic::Status::invalid_argument("resource is required"));
        }
        if self.request.permission.is_empty() {
            return Err(tonic::Status::invalid_argument("permission is required"));
        }
        let resp = self
            .client
            .lookup_subjects(self.request)
            .await?
            .into_inner();
        Ok(resp)
    }
}
