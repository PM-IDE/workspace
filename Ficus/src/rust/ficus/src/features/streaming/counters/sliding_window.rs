use crate::features::streaming::counters::core::{StreamingCounter, StreamingCounterEntry, ValueUpdateKind};
use chrono::{DateTime, Utc};
use num_traits::Num;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone)]
struct SlidingWindowEntry<TValue: Clone> {
    value: Option<TValue>,
    count: u64,
    timestamp: DateTime<Utc>,
}

impl<TValue: Clone> SlidingWindowEntry<TValue> {
    pub fn new(value: Option<TValue>, timestamp: DateTime<Utc>) -> Self {
        Self { value, timestamp, count: 1 }
    }

    pub fn to_streaming_counter_entry<TKey>(&self, key: TKey, approx_freq: f64) -> StreamingCounterEntry<TKey, TValue> {
        StreamingCounterEntry::new(key, self.value.clone(), approx_freq, self.count)
    }
}

#[derive(PartialEq)]
pub enum InvalidationResult {
    Invalidate,
    Retain,
}

pub type Invalidator<TValue> = Rc<Box<dyn Fn(Option<&TValue>, &DateTime<Utc>) -> InvalidationResult>>;

#[derive(Clone)]
pub struct SlidingWindow<TKey: Hash + Eq + Clone, TValue: Clone> {
    storage: HashMap<TKey, SlidingWindowEntry<TValue>>,
    invalidator: Invalidator<TValue>,
}

unsafe impl<TKey: Hash + Eq + Clone, TValue: Clone> Sync for SlidingWindow<TKey, TValue> {}
unsafe impl<TKey: Hash + Eq + Clone, TValue: Clone> Send for SlidingWindow<TKey, TValue> {}

impl<TKey: Hash + Eq + Clone, TValue: Clone> StreamingCounter<TKey, TValue> for SlidingWindow<TKey, TValue> {
    fn observe(&mut self, key: TKey, value: ValueUpdateKind<TValue>) {
        self.add_current_stamp(key, value);
    }

    fn get(&self, key: &TKey) -> Option<StreamingCounterEntry<TKey, TValue>> {
        match self.storage.get(key) {
            None => None,
            Some(entry) => Some(entry.to_streaming_counter_entry(key.clone(), entry.count as f64 / self.counts_sum() as f64))
        }
    }

    fn above_threshold(&self, threshold: f64) -> Vec<StreamingCounterEntry<TKey, TValue>> {
        let all_count: u64 = self.counts_sum();

        self.storage
            .iter()
            .filter(|(_, v)| v.count as f64 > threshold)
            .map(|(k, v)| StreamingCounterEntry::new(k.clone(), v.value.clone(), v.count as f64 / all_count as f64, v.count))
            .collect()
    }
}


impl<TKey: Hash + Eq + Clone, TValue: Clone> SlidingWindow<TKey, TValue> {
    pub fn new(invalidator: Invalidator<TValue>) -> Self {
        Self {
            storage: HashMap::new(),
            invalidator,
        }
    }

    pub fn new_time(invalidation_duration: Duration) -> Self {
        Self {
            storage: HashMap::new(),
            invalidator: Rc::new(Box::new(move |_, stamp| match stamp.add(invalidation_duration) < Utc::now() {
                true => InvalidationResult::Invalidate,
                false => InvalidationResult::Retain,
            })),
        }
    }

    fn add_current_stamp(&mut self, key: TKey, value: ValueUpdateKind<TValue>) {
        self.add(key, value, Utc::now());
    }

    fn counts_sum(&self) -> u64 {
        self.storage.iter().map(|(_, v)| v.count).sum()
    }

    pub fn add(&mut self, key: TKey, value: ValueUpdateKind<TValue>, stamp: DateTime<Utc>) {
        if let Some(entry) = self.storage.get_mut(&key) {
            entry.count += 1;
            entry.timestamp = stamp;
            if let ValueUpdateKind::Replace(new_value) = value {
                entry.value = Some(new_value);
            }

            return;
        }

        let value = match value {
            ValueUpdateKind::Replace(new_value) => Some(new_value),
            ValueUpdateKind::DoNothing => None
        };

        self.storage.insert(key, SlidingWindowEntry::new(value, stamp));
    }

    pub fn invalidate(&mut self) {
        let invalidator = self.invalidator.clone();
        self.storage
            .retain(|_, value| invalidator(value.value.as_ref(), &value.timestamp) == InvalidationResult::Retain)
    }
}
