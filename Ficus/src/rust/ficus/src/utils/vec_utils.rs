pub fn sort_by_first<TValue>(vec: &mut Vec<(&String, TValue)>) {
  vec.sort_by(|first, second| first.0.partial_cmp(second.0).unwrap());
}

pub trait VectorOptionExtensions<T> {
  fn is_non_empty(&self) -> Option<&Vec<T>>;
}

impl<T> VectorOptionExtensions<T> for Option<Vec<T>> {
  fn is_non_empty(&self) -> Option<&Vec<T>> {
    match self.as_ref() {
      None => None,
      Some(v) => {
        if !v.is_empty() {
          self.as_ref()
        } else {
          None
        }
      }
    }
  }
}
