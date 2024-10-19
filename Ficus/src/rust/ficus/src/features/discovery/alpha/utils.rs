use std::collections::HashSet;
use std::hash::Hash;

pub fn maximize<TElement: Hash + Eq + Clone>(
    elements: HashSet<TElement>,
    mut merger: impl FnMut(&TElement, &TElement) -> Option<TElement>,
) -> HashSet<TElement> {
    let mut current = elements;

    loop {
        let vec: Vec<&TElement> = current.iter().collect();
        let mut new_elements = HashSet::new();
        let mut any_change = false;
        let mut merged_indices = HashSet::new();

        for i in 0..vec.len() {
            for j in 0..vec.len() {
                if i != j {
                    if let Some(merged) = merger(vec.get(i).unwrap(), vec.get(j).unwrap()) {
                        any_change = true;
                        new_elements.insert(merged);
                        merged_indices.insert(i);
                        merged_indices.insert(j);
                    }
                }
            }
        }

        if !any_change {
            break;
        }

        for i in 0..vec.len() {
            if !merged_indices.contains(&i) {
                new_elements.insert(vec[i].clone());
            }
        }

        current = new_elements;
    }

    current
}
