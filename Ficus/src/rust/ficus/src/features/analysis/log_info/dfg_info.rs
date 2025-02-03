use std::collections::{HashMap, HashSet};

pub trait DfgInfo {
    fn get_directly_follows_count(&self, first: &String, second: &String) -> usize;
    fn is_in_directly_follows_relation(&self, left: &str, right: &str) -> bool;
    fn get_followed_events(&self, event_class: &String) -> Option<&HashMap<String, usize>>;
    fn get_precedes_events(&self, event_class: &String) -> Option<&HashMap<String, usize>>;
    fn is_event_with_single_follower(&self, event_class: &String) -> bool;
}

#[derive(Debug)]
pub struct OfflineDfgInfo {
    pub(super) dfg_pairs: HashMap<String, HashMap<String, usize>>,
    pub(super) followed_events: HashMap<String, HashMap<String, usize>>,
    pub(super) precedes_events: HashMap<String, HashMap<String, usize>>,
    pub(super) events_with_single_follower: HashSet<String>,
}

impl DfgInfo for OfflineDfgInfo {
    fn get_directly_follows_count(&self, first: &String, second: &String) -> usize {
        if let Some(values) = self.dfg_pairs.get(first) {
            if let Some(dfg_count) = values.get(second) {
                return *dfg_count;
            }
        }

        0
    }

    fn is_in_directly_follows_relation(&self, left: &str, right: &str) -> bool {
        if let Some(values) = self.dfg_pairs.get(left) {
            values.contains_key(right)
        } else {
            false
        }
    }

    fn get_followed_events(&self, event_class: &String) -> Option<&HashMap<String, usize>> {
        match self.followed_events.get(event_class) {
            Some(followers_counts) => Some(followers_counts),
            None => None,
        }
    }

    fn get_precedes_events(&self, event_class: &String) -> Option<&HashMap<String, usize>> {
        match self.precedes_events.get(event_class) {
            Some(followers_counts) => Some(followers_counts),
            None => None,
        }
    }

    fn is_event_with_single_follower(&self, event_class: &String) -> bool {
        self.events_with_single_follower.contains(event_class)
    }
}
