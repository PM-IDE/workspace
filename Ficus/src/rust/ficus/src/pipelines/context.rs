use super::errors::pipeline_errors::PipelinePartExecutionError;
use crate::utils::{
  performance::performance_cookie::PerformanceLogger,
  user_data::{
    keys::{DefaultKey, Key},
    user_data::{UserData, UserDataImpl},
  },
};
use getset::Getters;
use std::{any::Any, sync::Arc};

pub trait LogMessageHandler: Send + Sync {
  fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError>;
}

pub struct PipelineInfrastructure {
  log_message_handler: Option<Arc<dyn LogMessageHandler>>,
}

impl PerformanceLogger<PipelinePartExecutionError> for PipelineInfrastructure {
  fn log(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
    self.log(message)?;
    Ok(())
  }
}

impl PipelineInfrastructure {
  pub fn new(log_message_handler: Option<Arc<dyn LogMessageHandler>>) -> Self {
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

#[derive(Clone, Getters)]
pub struct PipelineContext {
  user_data: UserDataImpl,
}

impl PipelineContext {
  pub fn empty() -> Self {
    Self {
      user_data: Default::default(),
    }
  }
}

impl UserData for PipelineContext {
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

  fn items(&self) -> Option<Vec<(Box<dyn Key>, &dyn Any)>> {
    self.user_data.items()
  }
}

impl PipelineContext {
  pub fn devastate_user_data(self) -> UserDataImpl {
    self.user_data
  }
}
