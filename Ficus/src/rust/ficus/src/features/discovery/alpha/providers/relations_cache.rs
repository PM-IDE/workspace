use std::collections::HashMap;

pub struct RelationsCache<T> {
  cache: HashMap<String, HashMap<String, T>>,
}

impl<T> RelationsCache<T> {
  pub fn empty() -> Self {
    Self { cache: HashMap::new() }
  }

  pub fn try_get(&self, first: &str, second: &str) -> Option<&T> {
    if let Some(map) = self.cache.get(first) {
      if let Some(value) = map.get(second) {
        return Some(value);
      }
    }

    None
  }

  pub fn put(&mut self, first: &str, second: &str, value: T) {
    if !self.cache.contains_key(first) {
      self.cache.insert(first.to_owned(), HashMap::new());
    }

    let map = self.cache.get_mut(first).unwrap();
    if map.contains_key(second) {
      return;
    }

    map.insert(second.to_owned(), value);
  }
}

pub struct RelationsCaches<T> {
  caches: HashMap<&'static str, RelationsCache<T>>,
}

impl<T> RelationsCaches<T> {
  pub fn new(caches_names: &'static [&'static str]) -> Self {
    let mut caches = HashMap::new();
    for name in caches_names {
      caches.insert(*name, RelationsCache::empty());
    }

    Self { caches }
  }

  pub fn cache(&self, cache_name: &'static str) -> &RelationsCache<T> {
    self.caches.get(cache_name).unwrap()
  }

  pub fn cache_mut(&mut self, cache_name: &'static str) -> &mut RelationsCache<T> {
    self.caches.get_mut(cache_name).unwrap()
  }
}
