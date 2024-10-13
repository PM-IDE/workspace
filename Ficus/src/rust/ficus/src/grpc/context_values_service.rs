use crate::ficus_proto::grpc_context_values_service_server::GrpcContextValuesService;
use crate::ficus_proto::{GrpcContextKey, GrpcContextKeyValue, GrpcContextValuePart, GrpcDropContextValuesRequest, GrpcGuid};
use crate::grpc::converters::context_value_from_bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

pub struct ContextValueService {
    context_values: Mutex<HashMap<String, GrpcContextKeyValue>>,
}

impl ContextValueService {
    pub fn new() -> Self {
        Self {
            context_values: Mutex::new(HashMap::new()),
        }
    }

    pub fn reclaim_context_values(&self, ids: &Vec<GrpcGuid>) -> Result<Vec<GrpcContextKeyValue>, String> {
        let mut values = vec![];

        let mut context_values = self.context_values.lock();
        let context_values = context_values.as_mut().expect("Must acquire lock");

        for id in ids {
            if let Some((_, context_key_value)) = context_values.remove_entry(&id.guid) {
                values.push(context_key_value);
            } else {
                return Err(id.guid.clone());
            }
        }

        Ok(values)
    }

    pub fn put_context_value(&self, key: String, value: GrpcContextKeyValue) {
        let mut context_values = self.context_values.lock();
        let context_values = context_values.as_mut().expect("Must acquire lock");

        context_values.insert(key, value);
    }

    pub fn prune_context_values(&self, ids: &Vec<GrpcGuid>) {
        let mut context_values = self.context_values.lock();
        let context_values = context_values.as_mut().expect("Must acquire lock");

        for id in ids {
            context_values.remove(&id.guid);
        }
    }
}

pub struct GrpcContextValueService {
    cv_service: Arc<Mutex<ContextValueService>>,
}

impl GrpcContextValueService {
    pub fn new(cv_service: Arc<Mutex<ContextValueService>>) -> Self {
        Self { cv_service }
    }
}

#[tonic::async_trait]
impl GrpcContextValuesService for GrpcContextValueService {
    async fn set_context_value(&self, request: Request<Streaming<GrpcContextValuePart>>) -> Result<Response<GrpcGuid>, Status> {
        let context_value_id = Uuid::new_v4();

        let mut stream = request.into_inner();
        let mut bytes = vec![];
        let mut key = None;
        while let Some(part) = stream.message().await? {
            bytes.extend(part.bytes);
            key = Some(part.key);
        }

        let context_value = match context_value_from_bytes(bytes.as_slice()) {
            Ok(context_value) => context_value,
            Err(_) => return Err(Status::invalid_argument("Failed to deserialize context value from bytes")),
        };

        let mut cv_service = self.cv_service.lock();
        let cv_service = cv_service.as_mut().expect("Should acquire mut lock");

        cv_service.put_context_value(
            context_value_id.to_string(),
            GrpcContextKeyValue {
                key: Some(GrpcContextKey { name: key.unwrap() }),
                value: Some(context_value),
            },
        );

        Ok(Response::new(GrpcGuid {
            guid: context_value_id.to_string(),
        }))
    }

    async fn drop_context_values(&self, request: Request<GrpcDropContextValuesRequest>) -> Result<Response<()>, Status> {
        let cv_service = self.cv_service.lock();
        let cv_service = cv_service.as_ref().expect("Should acquire mut lock");

        cv_service.prune_context_values(&request.get_ref().ids);

        Ok(Response::new(()))
    }
}
