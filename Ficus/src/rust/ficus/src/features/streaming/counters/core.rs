use std::hash::Hash;

#[derive(Debug)]
pub struct StreamingCounterEntry<TKey, TValue> {
    key: TKey,
    value: Option<TValue>,
    approx_frequency: f64,
}

impl<TKey, TValue> StreamingCounterEntry<TKey, TValue> {
    pub fn new(key: TKey, value: Option<TValue>, approx_frequency: f64) -> Self {
        Self {
            key,
            value,
            approx_frequency,
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
}

pub enum ValueUpdateKind<TValue> {
    Replace(TValue),
    DoNothing,
}

pub trait StreamingCounter<TKey, TValue>
where
    TKey: Hash + Eq + Clone,
    TValue: Clone,
{
    fn observe(&mut self, element: TKey, value: ValueUpdateKind<TValue>);
    fn frequency(&self, element: &TKey) -> Option<StreamingCounterEntry<TKey, TValue>>;
    fn above_threshold(&self, threshold: f64) -> Vec<StreamingCounterEntry<TKey, TValue>>;

    fn all_frequencies(&self) -> Vec<StreamingCounterEntry<TKey, TValue>> {
        self.above_threshold(0.0)
    }
}
