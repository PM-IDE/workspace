use std::vec;

use crate::utils::suffix_tree::{
    suffix_tree_patterns::SuffixTree,
    suffix_tree_slice::{MultipleWordsSuffixTreeSlice, SingleWordSuffixTreeSlice},
};

use super::{contexts::PatternsDiscoveryStrategy, tandem_arrays::SubArrayInTraceInfo};

pub fn find_maximal_repeats(log: &Vec<Vec<u64>>, strategy: &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>> {
    find_repeats(log, strategy, |tree| tree.find_maximal_repeats())
}

fn find_repeats<TRepeatsFinder>(
    log: &Vec<Vec<u64>>,
    strategy: &PatternsDiscoveryStrategy,
    finder: TRepeatsFinder,
) -> Vec<Vec<SubArrayInTraceInfo>>
where
    TRepeatsFinder: Fn(&SuffixTree<u64>) -> Vec<(usize, usize)>,
{
    let mut repeats = vec![];

    let mut push_repeats = |patterns: &[(usize, usize)]| {
        repeats.push(
            patterns
                .into_iter()
                .map(|repeat| SubArrayInTraceInfo::new(repeat.0, repeat.1 - repeat.0))
                .collect(),
        );
    };

    match strategy {
        PatternsDiscoveryStrategy::FromAllTraces => {
            find_from_all_traces(log, &finder, &mut push_repeats);
        }
        PatternsDiscoveryStrategy::FromSingleMergedTrace => {
            find_from_single_merged_trace(log, &finder, &mut push_repeats);
        }
    }

    repeats
}

fn find_from_all_traces<TFinder, TRepeatsPusher>(log: &Vec<Vec<u64>>, finder: &TFinder, pusher: &mut TRepeatsPusher)
where
    TFinder: Fn(&SuffixTree<u64>) -> Vec<(usize, usize)>,
    TRepeatsPusher: FnMut(&[(usize, usize)]) -> (),
{
    for trace in log {
        let slice = SingleWordSuffixTreeSlice::new(trace.as_slice());
        let mut tree = SuffixTree::new(&slice);
        tree.build_tree();
        pusher(finder(&tree).as_slice());
    }
}

fn find_from_single_merged_trace<TFinder, TRepeatsPusher>(log: &Vec<Vec<u64>>, finder: &TFinder, pusher: &mut TRepeatsPusher)
where
    TFinder: Fn(&SuffixTree<u64>) -> Vec<(usize, usize)>,
    TRepeatsPusher: FnMut(&[(usize, usize)]) -> (),
{
    let mut single_trace = vec![];
    for trace in log {
        single_trace.push(trace.as_slice());
    }

    let slice = MultipleWordsSuffixTreeSlice::new(single_trace.clone());
    let mut tree = SuffixTree::new(&slice);

    tree.build_tree();

    let mut patterns = finder(&tree);
    let mut traces_patterns = vec![vec![]; log.len()];

    for pattern in &mut patterns {
        let first_index_info = slice.get_slice_info_for(pattern.0).unwrap();
        let trace_index = first_index_info.0;

        let first_index = first_index_info.1.unwrap();
        let second_index = match slice.get_slice_info_for(pattern.1).unwrap().1 {
            Some(index) => index,
            None => slice.get_slice_part_len(trace_index),
        };

        pattern.0 = first_index;
        pattern.1 = second_index;
        traces_patterns[trace_index].push((pattern.0, pattern.1));
    }

    for trace_patterns in traces_patterns {
        pusher(&trace_patterns);
    }
}

pub fn find_super_maximal_repeats(log: &Vec<Vec<u64>>, strategy: &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>> {
    find_repeats(log, strategy, |tree| tree.find_super_maximal_repeats())
}

pub fn find_near_super_maximal_repeats(log: &Vec<Vec<u64>>, strategy: &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>> {
    find_repeats(log, strategy, |tree| tree.find_near_super_maximal_repeats())
}
