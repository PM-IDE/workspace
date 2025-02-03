use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;
use std::time::Duration;
use chrono::{DateTime, Utc};

#[derive(Clone)]
struct SlidingWindowEntry<TValue> {
    value: TValue,
    timestamp: DateTime<Utc>,
}

impl<TValue> SlidingWindowEntry<TValue> {
    pub fn new(value: TValue, timestamp: DateTime<Utc>) -> Self {
        Self { value, timestamp }
    }
}

#[derive(PartialEq)]
pub enum InvalidationResult {
    Invalidate,
    Retain,
}

pub type Invalidator<TValue> = Rc<Box<dyn Fn(&TValue, &DateTime<Utc>) -> InvalidationResult>>;

#[derive(Clone)]
pub struct SlidingWindowProcessor<TKey: Hash + Eq, TValue> {
    storage: HashMap<TKey, SlidingWindowEntry<TValue>>,
    invalidator: Invalidator<TValue>,
}

unsafe impl<TKey: Hash + Eq, TValue> Sync for SlidingWindowProcessor<TKey, TValue> {}
unsafe impl<TKey: Hash + Eq, TValue> Send for SlidingWindowProcessor<TKey, TValue> {}

impl<TKey: Hash + Eq, TValue> SlidingWindowProcessor<TKey, TValue> {
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

    pub fn add_current_stamp(&mut self, key: TKey, value: TValue) {
        self.add(key, value, Utc::now());
    }

    pub fn add(&mut self, key: TKey, value: TValue, stamp: DateTime<Utc>) {
        self.storage.insert(key, SlidingWindowEntry::new(value, stamp));
    }

    pub fn get(&self, key: &TKey) -> Option<&TValue> {
        match self.storage.get(key) {
            None => None,
            Some(entry) => Some(&entry.value),
        }
    }

    pub fn invalidate(&mut self) {
        let invalidator = self.invalidator.clone();
        self.storage
            .retain(|_, value| invalidator(&value.value, &value.timestamp) == InvalidationResult::Retain)
    }
}
