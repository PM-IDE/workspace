use crate::utils::hash_utils::compare_based_on_hashes;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    id: u64,
    set: BTreeSet<T>,
}

static ONE_SET_NEXT_ID: AtomicU64 = AtomicU64::new(0);

impl<T> OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    pub fn empty() -> Self {
        Self {
            id: ONE_SET_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            set: BTreeSet::new(),
        }
    }

    pub fn new(el: T) -> Self {
        Self {
            id: ONE_SET_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            set: BTreeSet::from_iter(vec![el]),
        }
    }

    pub fn new_two_elements(first: T, second: T) -> Self {
        Self {
            id: ONE_SET_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            set: BTreeSet::from_iter(vec![first, second]),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            id: ONE_SET_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            set: self.set.iter().chain(other.set.iter()).map(|el| el.clone()).collect(),
        }
    }

    pub fn set(&self) -> &BTreeSet<T> {
        &self.set
    }

    pub fn insert(&mut self, item: T) {
        self.set.insert(item);
    }

    pub fn id(&self) -> &u64 {
        &self.id
    }
}

impl<T> Hash for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for el in &self.set {
            el.hash(state);
        }
    }
}

impl<T> PartialEq for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        compare_based_on_hashes(self, other)
    }
}

impl<T> Eq for OneSet<T> where T: Hash + Eq + Ord + Clone {}

impl<T> Clone for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: ONE_SET_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            set: self.set.clone(),
        }
    }
}
