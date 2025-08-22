use crate::ficus_proto::grpc_context_values_service_server::GrpcContextValuesService;
use crate::ficus_proto::{GrpcContextKey, GrpcContextKeyValue, GrpcContextValue, GrpcContextValuePart, GrpcDropContextValuesRequest, GrpcGuid};
use crate::grpc::converters::context_value_from_bytes;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use prost::Message;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status, Streaming};
use tonic::codegen::futures_core::Stream;
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

  pub fn get_context_value_bytes(&self, key: &str) -> Option<(String, Vec<u8>)> {
    let context_values = self.context_values.lock();
    let context_values = context_values.as_ref().expect("Must acquire lock");

    match context_values.get(key) {
      None => None,
      Some(value) => Some((value.key.as_ref().unwrap().name.clone(), value.encode_to_vec()))
    }
  }
}

pub struct GrpcContextValueService {
  cv_service: Arc<ContextValueService>,
}

impl GrpcContextValueService {
  pub fn new(cv_service: Arc<ContextValueService>) -> Self {
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

    self.cv_service.put_context_value(
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

  type GetContextValueStream = Pin<Box<dyn Stream<Item=Result<GrpcContextValuePart, Status>> + Send + 'static>>;

  async fn get_context_value(&self, request: Request<GrpcGuid>) -> Result<Response<Self::GetContextValueStream>, Status> {
    let (sender, receiver) = mpsc::channel(4);

    let id = request.get_ref().guid.as_str();
    match self.cv_service.get_context_value_bytes(id) {
      None => Err(Status::new(Code::NotFound, format!("Context value for id {} is not found", id))),
      Some((key, bytes)) => {
        tokio::spawn(async move {
          for chunk in bytes.chunks(1024) {
            let part = GrpcContextValuePart {
              key: key.clone(),
              bytes: chunk.to_vec(),
            };

            match sender.send(Ok(part)).await {
              Ok(_) => {},
              Err(err) => {
                log::error!("Failed to send context value part {}, error {}", key, err.to_string());
                break
              }
            }
          }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
      }
    }
  }

  async fn drop_context_values(&self, request: Request<GrpcDropContextValuesRequest>) -> Result<Response<()>, Status> {
    self.cv_service.prune_context_values(&request.get_ref().ids);

    Ok(Response::new(()))
  }
}
