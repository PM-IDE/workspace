use crate::utils::hash_map_utils::{compare_maps_by_keys, increase_in_map_by};

use super::suffix_tree_patterns::SuffixTree;
use std::collections::VecDeque;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

enum RepeatType {
    MaximalRepeat,
    SuperMaximalRepeat,
    NearSuperMaximalRepeat,
}

impl<'a, TElement> SuffixTree<'a, TElement>
where
    TElement: Eq + PartialEq + Hash + Copy,
{
    //docs: http://vis.usal.es/rodrigo/documentos/bioinfo/avanzada/soluciones/12-suffixtrees2.pdf
    pub fn find_maximal_repeats(&self) -> Vec<(usize, usize)> {
        self.find_repeats(RepeatType::MaximalRepeat)
    }

    pub fn find_super_maximal_repeats(&self) -> Vec<(usize, usize)> {
        self.find_repeats(RepeatType::SuperMaximalRepeat)
    }

    pub fn find_near_super_maximal_repeats(&self) -> Vec<(usize, usize)> {
        self.find_repeats(RepeatType::NearSuperMaximalRepeat)
    }

    fn find_repeats(&self, repeat_type: RepeatType) -> Vec<(usize, usize)> {
        let maximal_repeats = self.find_repeats_internal(&repeat_type);

        let mut maximal_repeats: Vec<(usize, usize)> = maximal_repeats.into_iter().collect();
        maximal_repeats.sort();

        let mut seen = HashSet::new();
        let mut filtered_repeats = Vec::new();
        for repeat in &maximal_repeats {
            if let Some(sub_slice) = self.slice.sub_slice(repeat.0, repeat.1) {
                if seen.contains(sub_slice) {
                    continue;
                }

                seen.insert(sub_slice);
                filtered_repeats.push(*repeat);
            }
        }

        filtered_repeats
    }

    fn find_repeats_internal(&self, repeat_type: &RepeatType) -> HashSet<(usize, usize)> {
        let queue = self.create_from_bottom_to_top_nodes_queue();
        self.find_repeats_from_queue(repeat_type, queue)
    }

    fn create_from_bottom_to_top_nodes_queue(&self) -> VecDeque<(usize, usize)> {
        let nodes = self.nodes.borrow();
        let start_node = nodes.get(0).unwrap();
        let mut queue = VecDeque::from_iter([(0, start_node.edge_len())]);

        let mut queue_index = 0;
        loop {
            let current_queue_len = queue.len();
            let mut queue_extended = false;

            for i in queue_index..current_queue_len {
                let (node_index, suffix_length) = queue.get(i).cloned().unwrap();
                let node = nodes.get(node_index).unwrap();

                for (_, child_index) in &node.children {
                    let child_node = nodes.get(*child_index).unwrap();
                    let child_suffix_length = suffix_length + child_node.edge_len();
                    queue.push_back((*child_index, child_suffix_length));
                    queue_extended = true;
                }
            }

            if !queue_extended {
                break;
            }

            queue_index = current_queue_len;
        }

        queue
    }

    fn find_repeats_from_queue(&self, repeat_type: &RepeatType, mut queue: VecDeque<(usize, usize)>) -> HashSet<(usize, usize)> {
        let nodes = self.nodes.borrow();
        let mut nodes_to_awc = HashMap::new();
        let mut nodes_to_any_suffix_len = HashMap::new();
        let mut maximal_repeats = HashSet::new();

        while let Some((node_index, suffix_length)) = queue.pop_back() {
            let node = nodes.get(node_index).unwrap();

            if node.is_leaf() {
                let element = self.get_element_for_suffix(suffix_length);
                nodes_to_any_suffix_len.insert(node_index, suffix_length);
                nodes_to_awc.insert(node_index, HashMap::from_iter(vec![(element, 1)]));
                continue;
            }

            let mut child_set = HashMap::new();
            for (_, child_index) in &node.children {
                for (element, count) in nodes_to_awc.get(&child_index).unwrap() {
                    increase_in_map_by(&mut child_set, element, *count);
                }
            }

            nodes_to_awc.insert(node_index, child_set);

            let children: Vec<&usize> = node.children.values().into_iter().collect();

            let child_suffix_len = nodes_to_any_suffix_len[children.iter().min().unwrap()];
            nodes_to_any_suffix_len.insert(node_index, child_suffix_len);

            if suffix_length != 0 {
                self.add_repeats(
                    repeat_type,
                    &node_index,
                    &children,
                    suffix_length,
                    &nodes_to_awc,
                    &nodes_to_any_suffix_len,
                    &mut maximal_repeats,
                );
            }
        }

        maximal_repeats
    }

    fn add_repeats(
        &self,
        repeat_type: &RepeatType,
        node_index: &usize,
        children: &Vec<&usize>,
        suffix_length: usize,
        nodes_to_awc: &HashMap<usize, HashMap<Option<TElement>, usize>>,
        nodes_to_any_suffix_len: &HashMap<usize, usize>,
        repeats: &mut HashSet<(usize, usize)>,
    ) {
        match repeat_type {
            RepeatType::MaximalRepeat => {}
            RepeatType::SuperMaximalRepeat => {
                let nodes = &self.nodes.borrow();

                for (_, child_index) in &nodes.get(*node_index).unwrap().children {
                    let child_node = nodes.get(*child_index).unwrap();
                    if !child_node.is_leaf() {
                        return;
                    }

                    if child_node.is_leaf() {
                        let element = self.get_element_for_suffix(nodes_to_any_suffix_len[child_index]);
                        if element != None && nodes_to_awc[node_index][&element] != 1 {
                            return;
                        }
                    }
                }
            }
            RepeatType::NearSuperMaximalRepeat => {
                let mut witnesses_near_supermaximality = false;
                let nodes = &self.nodes.borrow();
                for (_, child_index) in &nodes.get(*node_index).unwrap().children {
                    let child_node = nodes.get(*child_index).unwrap();
                    if child_node.is_leaf() {
                        let element = self.get_element_for_suffix(nodes_to_any_suffix_len[child_index]);

                        if element != None && nodes_to_awc[node_index][&element] == 1 {
                            witnesses_near_supermaximality = true;
                        }
                    }
                }

                if !witnesses_near_supermaximality {
                    return;
                }
            }
        }

        for first_child in children {
            for second_child in children {
                if first_child == second_child {
                    continue;
                }

                let first_map = nodes_to_awc.get(first_child).unwrap();
                let second_map = nodes_to_awc.get(second_child).unwrap();

                if !compare_maps_by_keys(first_map, second_map, HashSet::from_iter([None])) {
                    let first_child_suffix_len = nodes_to_any_suffix_len[first_child];
                    let start = self.slice.len() - first_child_suffix_len;

                    repeats.insert((start, start + suffix_length));
                }
            }
        }
    }
}
