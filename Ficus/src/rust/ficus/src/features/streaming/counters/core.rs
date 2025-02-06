use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct StreamingCounterEntry<TKey, TValue> {
    key: TKey,
    value: Option<TValue>,
    approx_frequency: f64,
    absolute_count: u64,
}

impl<TKey, TValue> StreamingCounterEntry<TKey, TValue> {
    pub fn new(key: TKey, value: Option<TValue>, approx_frequency: f64, absolute_count: u64) -> Self {
        Self {
            key,
            value,
            approx_frequency,
            absolute_count,
        }
    }

    pub fn value(&self) -> Option<&TValue> {
        self.value.as_ref()
    }
    pub fn key(&self) -> &TKey {
        &self.key
    }
    pub fn approx_frequency(&self) -> f64 {
        self.approx_frequency
    }
    pub fn absolute_count(&self) -> u64 {
        self.absolute_count
    }
}

pub enum ValueUpdateKind<TValue> {
    Replace(TValue),
    DoNothing,
}

pub trait StreamingCounter<TKey: Hash + Eq + Clone, TValue: Clone> {
    fn observe(&mut self, key: TKey, value: ValueUpdateKind<TValue>);
    fn get(&self, key: &TKey) -> Option<StreamingCounterEntry<TKey, TValue>>;
    fn above_threshold(&self, threshold: f64) -> Vec<StreamingCounterEntry<TKey, TValue>>;
    fn invalidate(&mut self);

    fn all_frequencies(&self) -> Vec<StreamingCounterEntry<TKey, TValue>> {
        self.above_threshold(0.0)
    }

    fn to_count_map(&self) -> HashMap<TKey, f64> {
        self.all_frequencies()
            .into_iter()
            .map(|entry| (entry.key().clone(), entry.approx_frequency()))
            .collect()
    }
}
