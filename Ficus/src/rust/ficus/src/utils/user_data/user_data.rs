use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use super::keys::{DefaultKey, Key};

pub trait UserDataOwner {
  fn user_data(&self) -> &UserDataImpl;
  fn user_data_mut(&mut self) -> &mut UserDataImpl;
}

pub trait ExecuteWithUserData {
  fn execute_with_user_data(&self, func: &mut dyn FnMut(&UserDataImpl) -> ());
  fn execute_with_user_data_mut(&mut self, func: &mut dyn FnMut(&mut UserDataImpl));
}

#[derive(Debug)]
pub struct UserDataImpl {
  values_map: Option<HashMap<u64, Rc<RefCell<dyn Any>>>>,
}

unsafe impl Send for UserDataImpl {}

pub trait UserData {
  fn len(&self) -> usize;

  fn put_concrete<T: 'static>(&mut self, key: &DefaultKey<T>, value: T);
  fn put_any<T: 'static>(&mut self, key: &dyn Key, value: T);
  fn concrete<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&T>;
  fn any(&self, key: &dyn Key) -> Option<&dyn Any>;
  fn concrete_mut<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&mut T>;
  fn remove_concrete<T: 'static>(&mut self, key: &DefaultKey<T>);
  fn remove_any<T: 'static>(&mut self, key: &dyn Key);
}

impl UserData for UserDataImpl {
  fn len(&self) -> usize {
    if let Some(map) = self.values_map.as_ref() {
      map.len()
    } else {
      0
    }
  }

  fn put_concrete<T: 'static>(&mut self, key: &DefaultKey<T>, value: T) {
    self.put_any(key, value)
  }

  fn put_any<T: 'static>(&mut self, key: &dyn Key, value: T) {
    self.initialize_values_map();

    let values_map = self.values_map.as_mut().unwrap();
    values_map.insert(key.id(), Rc::new(RefCell::new(value)));
  }

  fn concrete<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&T> {
    self.get(key)
  }

  fn any(&self, key: &dyn Key) -> Option<&dyn Any> {
    if self.values_map.is_none() {
      return None;
    }

    let values_map = self.values_map.as_ref().unwrap();
    if let Some(value) = values_map.get(&key.id()) {
      unsafe { Some(value.as_ref().try_borrow_unguarded().ok().unwrap()) }
    } else {
      None
    }
  }

  fn concrete_mut<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&mut T> {
    self.get_mut(key)
  }

  fn remove_concrete<T: 'static>(&mut self, key: &DefaultKey<T>) {
    if let Some(values_map) = self.values_map.as_mut() {
      values_map.remove(&key.id());
    }
  }

  fn remove_any<T: 'static>(&mut self, key: &dyn Key) {
    if let Some(values_map) = self.values_map.as_mut() {
      values_map.remove(&key.id());
    }
  }
}

impl UserDataImpl {
  pub fn new() -> Self {
    Self { values_map: None }
  }

  fn initialize_values_map(&mut self) {
    if self.values_map.is_some() {
      return;
    }

    self.values_map = Some(HashMap::new());
  }

  pub fn remove(&mut self, key: &impl Key) {
    if self.values_map.is_none() {
      return;
    }

    self.values_map.as_mut().unwrap().remove(&key.id());
  }

  pub fn get<T: 'static>(&self, key: &impl Key) -> Option<&T> {
    match self.any(key) {
      None => None,
      Some(any) => Some(any.downcast_ref::<T>().unwrap()),
    }
  }

  pub fn get_mut<T: 'static>(&self, key: &impl Key) -> Option<&mut T> {
    if self.values_map.is_none() {
      return None;
    }

    let values_map = self.values_map.as_ref().unwrap();
    if let Some(value) = values_map.get(&key.id()) {
      unsafe { Some(value.as_ptr().as_mut().unwrap().downcast_mut::<T>().unwrap()) }
    } else {
      None
    }
  }
}

impl Clone for UserDataImpl {
  fn clone(&self) -> Self {
    match self.values_map.as_ref() {
      None => Self { values_map: None },
      Some(map) => {
        let mut new_map = HashMap::new();
        for (key, value) in map {
          new_map.insert(key.clone(), Rc::clone(value));
        }

        Self { values_map: Some(new_map) }
      }
    }
  }
}

#[derive(Debug)]
pub struct UserDataHolder {
  user_data: Option<UserDataImpl>,
}

impl UserDataHolder {
  pub fn new() -> Self {
    Self { user_data: None }
  }

  pub fn get_mut(&mut self) -> &mut UserDataImpl {
    if self.user_data.is_none() {
      self.user_data = Some(UserDataImpl::new());
    }

    self.user_data.as_mut().unwrap()
  }
}

impl Clone for UserDataHolder {
  fn clone(&self) -> Self {
    match self.user_data.as_ref() {
      None => Self { user_data: None },
      Some(user_data) => Self {
        user_data: Some(user_data.clone()),
      },
    }
  }
}
