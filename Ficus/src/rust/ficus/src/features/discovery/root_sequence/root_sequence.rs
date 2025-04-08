use std::fmt::Debug;
use crate::features::discovery::root_sequence::models::RootSequenceKind;
use crate::utils::distance::distance::calculate_lcs_distance;
use crate::utils::lcs::{find_longest_common_subsequence, find_longest_common_subsequence_length};

pub fn discover_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>, root_sequence_kind: RootSequenceKind) -> Vec<T> {
  if log.is_empty() {
    return vec![];
  }

  match root_sequence_kind {
    RootSequenceKind::FindBest => {
      let (root_trace_index, root_distance) = find_trace_candidate_for_root_sequence(log);
      let (indices, root_pair_wise_lcs_distance) = find_traces_pairwise_lcs_candidate_for_root_sequence(log);
      let (lcs, root_lcs_distance) = find_lcs_candidate_for_root_sequence(log);

      let min_distance = root_distance.min(root_pair_wise_lcs_distance).min(root_lcs_distance);
      if root_distance == min_distance {
        log.get(root_trace_index).unwrap().iter().map(|c| c.clone()).collect()
      } else if root_pair_wise_lcs_distance == min_distance {
        create_root_sequence_from_lcs(log, indices)
      } else {
        lcs
      }
    }
    RootSequenceKind::LCS => find_lcs_candidate_for_root_sequence(log).0,
    RootSequenceKind::PairwiseLCS => create_root_sequence_from_lcs(log, find_traces_pairwise_lcs_candidate_for_root_sequence(log).0),
    RootSequenceKind::Trace => log.get(find_trace_candidate_for_root_sequence(log).0).unwrap().iter().map(|c| c.clone()).collect()
  }
}

fn find_trace_candidate_for_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>) -> (usize, f64) {
  let mut root_trace_index = 0;
  let mut root_distance = f64::MAX;
  for (index, trace_events) in log.iter().enumerate() {
    let mut summed_distance = 0.;
    for other_trace_events in log.iter() {
      let lcs = find_longest_common_subsequence_length(trace_events, other_trace_events, trace_events.len(), other_trace_events.len());
      let distance = calculate_lcs_distance(lcs, trace_events.len(), other_trace_events.len());

      summed_distance += distance;
    }

    if summed_distance < root_distance {
      root_distance = summed_distance;
      root_trace_index = index;
    }
  }

  (root_trace_index, root_distance)
}

fn find_traces_pairwise_lcs_candidate_for_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>) -> ((usize, usize), f64) {
  let mut root_lcs_distance = f64::MAX;
  let mut indices = (0, 0);
  for (first_index, first_trace) in log.iter().enumerate() {
    for (second_index, second_trace) in log.iter().enumerate() {
      if first_index == second_index {
        continue;
      }

      let lcs = find_longest_common_subsequence(first_trace, second_trace, first_trace.len(), second_trace.len())
        .lcs().into_iter().map(|c| (*c).clone()).collect::<Vec<T>>();

      let mut distance = 0.;
      for trace in log.iter() {
        let lcs_length = find_longest_common_subsequence_length(&lcs, trace, lcs.len(), trace.len());
        distance += calculate_lcs_distance(lcs_length, lcs.len(), trace.len());
      }

      if distance < root_lcs_distance {
        root_lcs_distance = distance;
        indices = (first_index, second_index);
      }
    }
  }

  (indices, root_lcs_distance)
}

fn find_lcs_candidate_for_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>) -> (Vec<T>, f64) {
  let mut lcs = log.first().unwrap().into_iter().map(|e| (*e).clone()).collect();

  for trace in log.iter().skip(1) {
    lcs = find_longest_common_subsequence(&lcs, trace, lcs.len(), trace.len()).lcs().into_iter().map(|e| (*e).clone()).collect();
  }

  let mut distance = 0.;
  for trace in log {
    distance += calculate_lcs_distance(lcs.len(), lcs.len(), trace.len());
  }

  (lcs, distance)
}

fn create_root_sequence_from_lcs<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>, indices: (usize, usize)) -> Vec<T> {
  let first_trace = log.get(indices.0).unwrap();
  let second_trace = log.get(indices.1).unwrap();

  let first_trace_len = first_trace.len();
  let second_trace_len = second_trace.len();

  find_longest_common_subsequence(first_trace, second_trace, first_trace_len, second_trace_len)
    .lcs()
    .into_iter()
    .map(|c| (*c).clone())
    .collect::<Vec<T>>()
}