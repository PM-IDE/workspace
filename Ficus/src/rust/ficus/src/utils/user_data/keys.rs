use std::hash::Hash;
use std::{
  hash::Hasher,
  marker::PhantomData,
  sync::atomic::{AtomicU64, Ordering},
};

pub trait Key {
  fn id(&self) -> u64;
  fn name(&self) -> &String;
}

pub struct DefaultKey<T> {
  name: String,
  _phantom_data: PhantomData<T>,
  _hash: u64,
}

impl<T> Key for DefaultKey<T>
where
  T: 'static,
{
  fn id(&self) -> u64 {
    self._hash
  }

  fn name(&self) -> &String {
    &self.name
  }
}

impl<T> Clone for DefaultKey<T> {
  fn clone(&self) -> Self {
    Self {
      name: self.name.clone(),
      _phantom_data: self._phantom_data.clone(),
      _hash: self._hash.clone(),
    }
  }
}

impl<T> DefaultKey<T>
where
  T: 'static,
{
  pub fn new(name: String) -> Self {
    static CURRENT_HASH: AtomicU64 = AtomicU64::new(0);

    Self {
      name: name.to_owned(),
      _phantom_data: PhantomData,
      _hash: CURRENT_HASH.fetch_add(1, Ordering::SeqCst),
    }
  }
}

impl<T> PartialEq for DefaultKey<T> {
  fn eq(&self, other: &Self) -> bool {
    self._hash == other._hash
  }
}

impl<T> Hash for DefaultKey<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u64(self._hash)
  }
}
