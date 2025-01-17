use std::hash::Hash;

#[derive(Debug)]
pub struct StreamingCounterEntry<T> {
    key: T,
    approx_frequency: f64,
}

impl<T> StreamingCounterEntry<T> {
    pub fn new(key: T, approx_frequency: f64) -> Self {
        Self { key, approx_frequency }
    }

    pub fn key(&self) -> &T {
        &self.key
    }
    pub fn approx_frequency(&self) -> f64 {
        self.approx_frequency
    }
}

pub trait StreamingCounter<T>
where
    T: Hash + Eq + Clone,
{
    fn observe(&mut self, element: T);
    fn frequency(&self, element: &T) -> Option<StreamingCounterEntry<T>>;
    fn above_threshold(&self, threshold: f64) -> Vec<StreamingCounterEntry<T>>;

    fn all_frequencies(&self) -> Vec<StreamingCounterEntry<T>> {
        self.above_threshold(0.0)
    }
}
