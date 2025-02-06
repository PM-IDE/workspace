use num_traits::Num;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn increase_in_map<TKey, TNum: Num + Clone>(map: &mut HashMap<TKey, TNum>, key: &TKey)
where
    TKey: Hash + Eq + PartialEq + Clone,
{
    if let Some(value) = map.get_mut(key) {
        *value = value.clone().add(TNum::one());
    } else {
        map.insert(key.clone(), TNum::one());
    }
}

pub fn increase_in_map_by<TKey, TNum: Num + Clone>(map: &mut HashMap<TKey, TNum>, key: &TKey, increase_value: TNum)
where
    TKey: Hash + Eq + PartialEq + Clone,
{
    if let Some(value) = map.get_mut(key) {
        *value = value.clone().add(increase_value);
    } else {
        map.insert(key.clone(), increase_value);
    }
}

pub fn add_to_list_in_map<TKey, TValue>(map: &mut HashMap<TKey, Vec<TValue>>, key: &TKey, value: TValue)
where
    TKey: Hash + Eq + PartialEq + Clone,
{
    if let Some(list) = map.get_mut(key) {
        list.push(value);
    } else {
        map.insert(key.clone(), vec![value]);
    }
}

pub fn compare_maps_by_keys<TKey, TValue>(
    first_map: &HashMap<TKey, TValue>,
    second_map: &HashMap<TKey, TValue>,
    unique_keys: HashSet<TKey>,
) -> bool
where
    TKey: Hash + Eq + PartialEq + Clone,
{
    for key in first_map.keys() {
        if unique_keys.contains(key) || !second_map.contains_key(key) {
            return false;
        }
    }

    for key in second_map.keys() {
        if unique_keys.contains(key) || !first_map.contains_key(key) {
            return false;
        }
    }

    true
}

pub fn reverse_map<TKey, TValue>(map: &HashMap<TKey, TValue>) -> HashMap<TValue, TKey>
where
    TKey: Hash + Eq + PartialEq + Clone,
    TValue: Hash + Eq + PartialEq + Clone,
{
    let mut reversed_map = HashMap::new();
    for (key, value) in map {
        reversed_map.insert(value.clone(), key.clone());
    }

    reversed_map
}
