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
    pub(super) followed_events: HashMap<String, HashMap<String, usize>>,
    pub(super) precedes_events: HashMap<String, HashMap<String, usize>>,
    pub(super) events_with_single_follower: HashSet<String>,
}

impl OfflineDfgInfo {
    pub fn create_from_relations(relations: &HashMap<(String, String), u64>) -> OfflineDfgInfo {
        let mut followed_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut precedes_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut events_with_single_follower = HashSet::new();

        for (relation, count) in relations {
            if let Some(followers_map) = followed_events.get_mut(&relation.0) {
                *followers_map.entry(relation.1.to_owned()).or_insert(0) += 1;
            } else {
                followed_events.insert(relation.0.to_owned(), HashMap::from_iter([(relation.1.to_owned(), 1)]));
            }

            if let Some(precedes_map) = precedes_events.get_mut(&relation.1) {
                *precedes_map.entry(relation.0.to_owned()).or_insert(0) += 1;
            } else {
                precedes_events.insert(relation.1.to_owned(), HashMap::from_iter([(relation.0.to_owned(), 1)]));
            }

            if *count == 1 {
                events_with_single_follower.insert(relation.0.to_owned());
            }
        }

        OfflineDfgInfo {
            followed_events,
            precedes_events,
            events_with_single_follower
        }
    }
}

impl DfgInfo for OfflineDfgInfo {
    fn get_directly_follows_count(&self, first: &String, second: &String) -> usize {
        if let Some(values) = self.followed_events.get(first) {
            if let Some(dfg_count) = values.get(second) {
                return *dfg_count;
            }
        }

        0
    }

    fn is_in_directly_follows_relation(&self, left: &str, right: &str) -> bool {
        if let Some(values) = self.followed_events.get(left) {
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
