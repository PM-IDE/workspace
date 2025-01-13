use std::collections::HashMap;
use std::hash::Hash;

struct State {
    freq: u64,
    delta: f64,
}

impl State {
    pub fn new(delta: f64) -> Self {
        Self {
            freq: 1,
            delta
        }
    }
}

pub struct LossyCount<T> where T: Hash + Eq {
    state: HashMap<T, State>,
    batch_size: u64,
    observed_items_count: u64
}

impl<T> LossyCount<T> where T: Hash + Eq + Clone {
    pub fn new(error: f64) -> Self {
        Self {
            state: HashMap::new(),
            batch_size: (1f64 / error).ceil() as u64,
            observed_items_count: 0,
        }
    }

    pub fn observe(&mut self, e: T) {
        self.observed_items_count += 1;
        let bucket_number = self.observed_items_count as f64 / self.batch_size as f64;

        if self.state.contains_key(&e) {
            self.state.get_mut(&e).unwrap().freq += 1;
        } else {
            self.state.insert(e, State::new(bucket_number));
        }

        if self.observed_items_count % self.batch_size == 0 {
            self.prune(bucket_number);
        }
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