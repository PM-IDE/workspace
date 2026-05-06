use crate::{
  ficus_proto::{
    GrpcContextKey, GrpcContextKeyValue, GrpcContextValuePart, GrpcDropContextValuesRequest, GrpcGetAllContextValuesResult,
    GrpcGetContextValueRequest, GrpcGuid, grpc_context_values_service_server::GrpcContextValuesService,
  },
  grpc::converters::context_value_from_bytes,
};
use ficus::pipelines::keys::context_keys::find_context_key;
use futures::Stream;
use prost::Message;
use std::{
  collections::HashMap,
  pin::Pin,
  sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status, Streaming};
use uuid::Uuid;

pub struct ContextValueService {
  context_values: Mutex<HashMap<String, GrpcContextKeyValue>>,
  contexts: Mutex<HashMap<String, HashMap<String, Uuid>>>,
}

impl ContextValueService {
  pub fn new() -> Self {
    Self {
      context_values: Default::default(),
      contexts: Default::default(),
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

    context_values.get(key).map(|value| {
      (
        value.key.as_ref().unwrap().name.clone(),
        value.value.as_ref().unwrap().encode_to_vec(),
      )
    })
  }

  pub fn insert_cv_to_ids(&self, id: Uuid, keys_to_ids: HashMap<String, Uuid>) {
    self.contexts.lock().as_mut().unwrap().insert(id.to_string(), keys_to_ids);
  }

  pub fn drop_execution_result(&self, execution_id: &str) -> Result<Response<()>, Status> {
    let mut contexts = self.contexts.lock();
    let contexts = contexts.as_mut().ok().unwrap();

    contexts.remove(execution_id).map_or_else(
      || Err(Status::not_found(format!("The session for {} does not exist", execution_id))),
      |_| Ok(Response::new(())),
    )
  }

  pub fn get_all_context_values(&self, execution_id: &str) -> Result<Vec<String>, Status> {
    self.contexts.lock().as_ref().unwrap().get(execution_id).map_or_else(
      || Err(Status::not_found("The context values for supplied execution id are not found")),
      |ids| Ok(ids.values().map(|id| id.to_string()).collect()),
    )
  }

  pub fn get_context_value(&self, execution_id: &str, key: &str) -> Result<Uuid, Status> {
    match self.contexts.lock().unwrap().get_mut(execution_id) {
      None => Err(Status::not_found("Failed to get context for guid".to_string())),
      Some(keys_to_cv_ids) => match keys_to_cv_ids.get(key) {
        None => Err(Status::not_found("Failed to get context for guid".to_string())),
        Some(id) => Ok(*id),
      },
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

  type GetContextValueStream = Pin<Box<dyn Stream<Item = Result<GrpcContextValuePart, Status>> + Send + 'static>>;

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
              Ok(_) => {}
              Err(err) => {
                log::error!("Failed to send context value part {}, error {}", key, err);
                break;
              }
            }
          }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
      }
    }
  }

  async fn get_context_value_id(&self, request: Request<GrpcGetContextValueRequest>) -> Result<Response<GrpcGuid>, Status> {
    let key_name = &request.get_ref().key.as_ref().unwrap().name;
    match find_context_key(key_name) {
      None => Err(Status::not_found(format!("Failed to find key for key name {}", key_name))),
      Some(key) => {
        let id = request.get_ref().execution_id.as_ref().unwrap();

        self
          .cv_service
          .get_context_value(&id.guid, key.key().name().as_str())
          .map(|id| Response::new(GrpcGuid { guid: id.to_string() }))
      }
    }
  }

  async fn get_all_context_values_ids(&self, request: Request<GrpcGuid>) -> Result<Response<GrpcGetAllContextValuesResult>, Status> {
    self.cv_service.get_all_context_values(&request.get_ref().guid).map(|ids| {
      Response::new(GrpcGetAllContextValuesResult {
        context_values: ids.into_iter().map(|id| GrpcGuid { guid: id.to_string() }).collect(),
      })
    })
  }

  async fn drop_context_values(&self, request: Request<GrpcDropContextValuesRequest>) -> Result<Response<()>, Status> {
    self.cv_service.prune_context_values(&request.get_ref().ids);

    Ok(Response::new(()))
  }
}
