use crate::features::streaming::counters::core::ValueUpdateKind;
use chrono::{DateTime, Utc};
use num_traits::Num;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone)]
struct SlidingWindowEntry<TValue> {
    value: Option<TValue>,
    timestamp: DateTime<Utc>,
}

impl<TValue> SlidingWindowEntry<TValue> {
    pub fn new(value: Option<TValue>, timestamp: DateTime<Utc>) -> Self {
        Self { value, timestamp }
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

    pub fn add_current_stamp(&mut self, key: TKey, value: ValueUpdateKind<TValue>) {
        self.add(key, value, Utc::now());
    }

    pub fn add(&mut self, key: TKey, value: ValueUpdateKind<TValue>, stamp: DateTime<Utc>) {
        let value = match value {
            ValueUpdateKind::Replace(new_value) => Some(new_value),
            ValueUpdateKind::DoNothing => match self.storage.get(&key) {
                None => None,
                Some(value) => value.value.clone()
            }
        };

        self.storage.insert(key, SlidingWindowEntry::new(value, stamp));
    }

    pub fn get(&self, key: &TKey) -> Option<&TValue> {
        match self.storage.get(key) {
            None => None,
            Some(entry) => entry.value.as_ref(),
        }
    }

    pub fn all(&self) -> Vec<(&TKey, Option<&TValue>)> {
        self.storage.iter().map(|p| (p.0, p.1.value.as_ref())).collect()
    }

    pub fn replace_current_stamp(&mut self, key: TKey, value_factory: impl Fn(Option<&TValue>) -> TValue) {
        let new_value = value_factory(match self.storage.get(&key) {
            None => None,
            Some(value) => value.value.as_ref(),
        });

        self.add_current_stamp(key, ValueUpdateKind::Replace(new_value));
    }

    pub fn invalidate(&mut self) {
        let invalidator = self.invalidator.clone();
        self.storage
            .retain(|_, value| invalidator(value.value.as_ref(), &value.timestamp) == InvalidationResult::Retain)
    }

    pub fn to_count_map(&self) -> HashMap<TKey, Option<TValue>> {
        self.all().into_iter().map(|(k, v)| (k.clone(), match v {
            None => None,
            Some(v) => Some(v.clone())
        })).collect()
    }
}

impl<TKey: Hash + Eq + Clone, TValue: Num + Clone> SlidingWindow<TKey, TValue> {
    pub fn increment_current_stamp(&mut self, key: TKey) {
        self.replace_current_stamp(key, |old| match old {
            None => TValue::one(),
            Some(old) => TValue::add(old.clone(), TValue::one()),
        });
    }
}
