use std::collections::HashMap;
use std::hash::Hash;
use crate::features::streaming::counters::core::{StreamingCounter, StreamingCounterEntry};

struct LossyCountState {
    freq: u64,
    delta: f64,
}

impl LossyCountState {
    pub fn new(delta: f64) -> Self {
        Self {
            freq: 1,
            delta
        }
    }
}

pub struct LossyCount<T> where T: Hash + Eq {
    state: HashMap<T, LossyCountState>,
    batch_size: u64,
    observed_items_count: u64
}

impl<T> StreamingCounter<T> for LossyCount<T> where T: Hash + Eq + Clone {
    fn observe(&mut self, element: T) {
        self.observed_items_count += 1;
        let bucket_number = self.observed_items_count / self.batch_size + 1;

        if self.state.contains_key(&element) {
            self.state.get_mut(&element).unwrap().freq += 1;
        } else {
            self.state.insert(element, LossyCountState::new((bucket_number - 1) as f64));
        }

        if self.observed_items_count % self.batch_size == 0 {
            self.prune(bucket_number as f64);
        }
    }

    fn frequency(&self, element: &T) -> Option<StreamingCounterEntry<T>> {
        match self.state.get(element) {
            None => None,
            Some(entry) => Some(self.to_streaming_counter_entry((element, entry)))
        }
    }

    fn above_threshold(&self, threshold: f64) -> Vec<StreamingCounterEntry<T>> {
        self.state
            .iter()
            .filter(|s| s.1.freq as f64 >= (threshold - s.1.delta) * (self.observed_items_count as f64))
            .map(|s| self.to_streaming_counter_entry(s))
            .collect()
    }
}

impl<T> LossyCount<T> where T: Hash + Eq + Clone {
    pub fn new(error: f64) -> Self {
        Self {
            state: HashMap::new(),
            batch_size: (1f64 / error).ceil() as u64,
            observed_items_count: 0,
        }
    }

    fn to_streaming_counter_entry(&self, pair: (&T, &LossyCountState)) -> StreamingCounterEntry<T> {
        StreamingCounterEntry::new(pair.0.clone(), pair.1.freq as f64 / (self.observed_items_count as f64))
    }

    fn prune(&mut self, bucket_number: f64) {
        let keys_to_remove: Vec<T> = self.state
            .iter()
            .filter(|s| s.1.freq as f64 + s.1.delta <= bucket_number)
            .map(|s| s.0.clone())
            .collect();

        for key in keys_to_remove {
            self.state.remove(&key);
        }
    }
}