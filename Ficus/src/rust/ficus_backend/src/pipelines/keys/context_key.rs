use crate::utils::user_data::keys::{DefaultKey, Key};
use std::hash::{Hash, Hasher};

pub trait ContextKey {
    fn key(&self) -> &dyn Key;
}

pub struct DefaultContextKey<T>
where
    T: 'static,
{
    key: DefaultKey<T>,
}

impl<T> ContextKey for DefaultContextKey<T> {
    fn key(&self) -> &dyn Key {
        &self.key
    }
}

impl<T> Clone for DefaultContextKey<T> {
    fn clone(&self) -> Self {
        Self { key: self.key.clone() }
    }
}

impl<T> DefaultContextKey<T>
where
    T: 'static,
{
    pub fn new(name: &str) -> Self {
        Self {
            key: DefaultKey::new(name.to_owned()),
        }
    }

    pub fn key(&self) -> &DefaultKey<T> {
        &self.key
    }
}

impl<T> PartialEq for DefaultContextKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for DefaultContextKey<T> {}

impl<T> Hash for DefaultContextKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
