use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::performance::performance_cookie::PerformanceLogger;
use std::{any::Any, sync::Arc};

use crate::utils::user_data::{
  keys::{DefaultKey, Key},
  user_data::{UserData, UserDataImpl},
};

use super::errors::pipeline_errors::PipelinePartExecutionError;

pub trait LogMessageHandler: Send + Sync {
  fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError>;
}

pub struct PipelineInfrastructure {
  log_message_handler: Option<Arc<Box<dyn LogMessageHandler>>>,
}

impl PerformanceLogger<PipelinePartExecutionError> for PipelineInfrastructure {
  fn log(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
    self.log(message)?;
    Ok(())
  }
}

impl PipelineInfrastructure {
  pub fn new(log_message_handler: Option<Arc<Box<dyn LogMessageHandler>>>) -> Self {
    Self { log_message_handler }
  }

  pub fn log(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
    if let Some(handler) = self.log_message_handler.as_ref() {
      handler.handle(message)
    } else {
      Ok(())
    }
  }
}

#[derive(Clone)]
pub struct PipelineContext<'a> {
  user_data: UserDataImpl,
  pipeline_parts: Option<&'a PipelineParts>,
}

impl<'a> PipelineContext<'a> {
  pub fn new_with_logging(parts: &'a PipelineParts) -> Self {
    Self {
      user_data: UserDataImpl::new(),
      pipeline_parts: Some(parts),
    }
  }

  pub fn empty() -> Self {
    Self {
      user_data: UserDataImpl::new(),
      pipeline_parts: None,
    }
  }

  pub fn empty_from(other: &'a PipelineContext) -> Self {
    Self {
      user_data: UserDataImpl::new(),
      pipeline_parts: other.pipeline_parts.clone(),
    }
  }
}

impl<'a> UserData for PipelineContext<'a> {
  fn len(&self) -> usize {
    self.user_data.len()
  }

  fn put_concrete<T: 'static>(&mut self, key: &DefaultKey<T>, value: T) {
    self.user_data.put_concrete(key, value)
  }

  fn put_any<T: 'static>(&mut self, key: &dyn Key, value: T) {
    self.user_data.put_any(key, value)
  }

  fn concrete<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&T> {
    self.user_data.concrete(key)
  }

  fn any(&self, key: &dyn Key) -> Option<&dyn Any> {
    self.user_data.any(key)
  }

  fn concrete_mut<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&mut T> {
    self.user_data.concrete_mut(key)
  }

  fn remove_concrete<T: 'static>(&mut self, key: &DefaultKey<T>) {
    self.user_data.remove_concrete(key)
  }

  fn remove_any<T: 'static>(&mut self, key: &dyn Key) {
    self.user_data.remove_any::<T>(key)
  }
}

impl<'a> PipelineContext<'a> {
  pub fn pipeline_parts(&self) -> Option<&PipelineParts> {
    self.pipeline_parts
  }

  pub fn devastate_user_data(self) -> UserDataImpl {
    self.user_data
  }
}
