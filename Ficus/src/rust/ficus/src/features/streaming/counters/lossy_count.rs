use crate::features::streaming::counters::core::{StreamingCounter, StreamingCounterEntry, ValueUpdateKind};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
struct LossyCountState<TValue> {
    value: Option<TValue>,
    freq: u64,
    delta: f64,
}

impl<TValue> LossyCountState<TValue> {
    pub fn new(value: Option<TValue>, delta: f64) -> Self {
        Self { value, freq: 1, delta }
    }
}

#[derive(Clone)]
pub struct LossyCount<T, TValue>
where
    T: Hash + Eq,
{
    state: HashMap<T, LossyCountState<TValue>>,
    batch_size: u64,
    observed_items_count: u64,
}

impl<TKey, TValue> StreamingCounter<TKey, TValue> for LossyCount<TKey, TValue>
where
    TKey: Hash + Eq + Clone,
    TValue: Clone,
{
    fn observe(&mut self, element: TKey, value: ValueUpdateKind<TValue>) {
        self.observed_items_count += 1;
        let bucket_number = self.observed_items_count / self.batch_size + 1;

        let value = match value {
            ValueUpdateKind::Replace(value) => Some(value),
            ValueUpdateKind::DoNothing => None,
        };

        if self.state.contains_key(&element) {
            let element_state = self.state.get_mut(&element).unwrap();

            element_state.freq += 1;
            if let Some(value) = value {
                element_state.value = Some(value);
            }
        } else {
            self.state.insert(element, LossyCountState::new(value, (bucket_number - 1) as f64));
        }

        if self.observed_items_count % self.batch_size == 0 {
            self.prune(bucket_number as f64);
        }
    }

    fn get(&self, key: &TKey) -> Option<StreamingCounterEntry<TKey, TValue>> {
        match self.state.get(key) {
            None => None,
            Some(entry) => Some(self.to_streaming_counter_entry((key, entry))),
        }
    }

    fn above_threshold(&self, threshold: f64) -> Vec<StreamingCounterEntry<TKey, TValue>> {
        self.state
            .iter()
            .filter(|s| s.1.freq as f64 >= (threshold - s.1.delta) * (self.observed_items_count as f64))
            .map(|s| self.to_streaming_counter_entry(s))
            .collect()
    }
}

impl<TKey, TValue> LossyCount<TKey, TValue>
where
    TKey: Hash + Eq + Clone,
    TValue: Clone,
{
    pub fn new(error: f64) -> Self {
        Self {
            state: HashMap::new(),
            batch_size: (1f64 / error).ceil() as u64,
            observed_items_count: 0,
        }
    }

    fn to_streaming_counter_entry(&self, pair: (&TKey, &LossyCountState<TValue>)) -> StreamingCounterEntry<TKey, TValue> {
        StreamingCounterEntry::new(
            pair.0.clone(),
            pair.1.value.clone(),
            pair.1.freq as f64 / (self.observed_items_count as f64),
            pair.1.freq
        )
    }

    fn prune(&mut self, bucket_number: f64) {
        self.state.retain(|_, value| value.freq as f64 + value.delta > bucket_number);
    }
}
