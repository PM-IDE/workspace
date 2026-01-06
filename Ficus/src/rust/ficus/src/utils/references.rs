use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
  fmt::{Debug, Display, Formatter},
  hash::{Hash, Hasher},
  ops::Deref,
  rc::Rc,
};

pub enum ReferenceOrOwned<'a, T> {
  Ref(&'a T),
  Owned(T),
}

impl<'a, T> Deref for ReferenceOrOwned<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match self {
      ReferenceOrOwned::Ref(reference) => reference,
      ReferenceOrOwned::Owned(value) => &value,
    }
  }
}

pub enum HeapedOrOwned<T> {
  Heaped(Rc<Box<T>>),
  Owned(T),
}

impl<T: Serialize> Serialize for HeapedOrOwned<T> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      HeapedOrOwned::Heaped(heaped) => heaped.serialize(serializer),
      HeapedOrOwned::Owned(owned) => owned.serialize(serializer),
    }
  }
}

impl<'a, T: Deserialize<'a>> Deserialize<'a> for HeapedOrOwned<T> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>,
  {
    Ok(HeapedOrOwned::Heaped(Rc::new(Box::new(T::deserialize(deserializer)?))))
  }
}

pub fn owned<T>(t: T) -> HeapedOrOwned<T> {
  HeapedOrOwned::Owned(t)
}

pub fn heaped<T>(t: T) -> HeapedOrOwned<T> {
  HeapedOrOwned::Heaped(Rc::new(Box::new(t)))
}

impl<T> Deref for HeapedOrOwned<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match self {
      HeapedOrOwned::Heaped(ptr) => ptr.as_ref().as_ref(),
      HeapedOrOwned::Owned(value) => &value,
    }
  }
}

impl<T> Display for HeapedOrOwned<T>
where
  T: ToString,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let str = self.deref().to_string();
    write!(f, "{}", str)
  }
}

impl<T> Hash for HeapedOrOwned<T>
where
  T: Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.deref().hash(state);
  }
}

impl<T> PartialEq for HeapedOrOwned<T>
where
  T: Eq,
{
  fn eq(&self, other: &Self) -> bool {
    self.deref().eq(other.deref())
  }
}

impl<T> Eq for HeapedOrOwned<T> where T: Eq {}

impl<T> Clone for HeapedOrOwned<T>
where
  T: Clone,
{
  fn clone(&self) -> Self {
    match self {
      HeapedOrOwned::Heaped(heaped) => HeapedOrOwned::Heaped(heaped.clone()),
      HeapedOrOwned::Owned(owned) => HeapedOrOwned::Owned(owned.clone()),
    }
  }
}

impl<T> Debug for HeapedOrOwned<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}
