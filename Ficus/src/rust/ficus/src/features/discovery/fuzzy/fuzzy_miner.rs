use super::fuzzy_metrics_provider::FuzzyMetricsProvider;
use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use crate::utils::graph::graph::{Graph, NodesConnectionData};
use crate::utils::sets::one_set::OneSet;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub type FuzzyGraph = Graph<String, f64>;

pub fn discover_graph_fuzzy(
  log: &impl EventLog,
  unary_frequency_threshold: f64,
  binary_frequency_significance_threshold: f64,
  preserve_threshold: f64,
  ratio_threshold: f64,
  utility_rate: f64,
  edge_cutoff_threshold: f64,
  node_cutoff_threshold: f64,
) -> FuzzyGraph {
  let mut graph = FuzzyGraph::empty();

  let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
  let mut provider = FuzzyMetricsProvider::new(log, &info);
  let mut classes_to_ids = HashMap::new();

  initialize_fuzzy_graph(
    &mut graph,
    &provider,
    &mut classes_to_ids,
    unary_frequency_threshold,
    binary_frequency_significance_threshold,
  );

  resolve_conflicts(&classes_to_ids, &provider, &mut graph, preserve_threshold, ratio_threshold);
  filter_edges(&mut provider, &mut graph, utility_rate, edge_cutoff_threshold);
  discover_clusters(&mut provider, &mut graph, node_cutoff_threshold);

  graph
}

fn initialize_fuzzy_graph<TLog: EventLog>(
  graph: &mut FuzzyGraph,
  provider: &FuzzyMetricsProvider<TLog>,
  classes_to_ids: &mut HashMap<String, u64>,
  unary_frequency_threshold: f64,
  binary_frequency_significance_threshold: f64,
) {
  for class in provider.log_info().all_event_classes() {
    if provider.unary_frequency_significance(class) > unary_frequency_threshold {
      let node_id = graph.add_node(Some(class.to_owned()));
      classes_to_ids.insert(class.to_owned(), node_id);
    }
  }

  for first_class in classes_to_ids.keys() {
    for second_class in classes_to_ids.keys() {
      let bin_freq_sig = provider.binary_frequency_significance(first_class, second_class);
      if bin_freq_sig > binary_frequency_significance_threshold {
        let first_id = classes_to_ids.get(first_class).unwrap();
        let second_id = classes_to_ids.get(second_class).unwrap();
        let connection_data = NodesConnectionData::new(Some(bin_freq_sig), bin_freq_sig, None);

        graph.connect_nodes(first_id, second_id, connection_data);
      }
    }
  }
}

fn resolve_conflicts<TLog: EventLog>(
  classes_to_ids: &HashMap<String, u64>,
  provider: &FuzzyMetricsProvider<TLog>,
  graph: &mut FuzzyGraph,
  preserve_threshold: f64,
  ratio_threshold: f64,
) {
  for first_name in classes_to_ids.keys() {
    for second_name in classes_to_ids.keys() {
      let first_id = classes_to_ids.get(first_name).unwrap();
      let second_id = classes_to_ids.get(second_name).unwrap();

      if are_nodes_bi_connected(&graph, first_id, second_id) {
        let first_second_sig = provider.relative_significance(first_name, second_name, &graph);
        let second_first_sig = provider.relative_significance(second_name, first_name, &graph);

        if first_second_sig < preserve_threshold || second_first_sig < preserve_threshold {
          let offset = (first_second_sig - second_first_sig).abs();

          if offset > ratio_threshold {
            if first_second_sig < second_first_sig {
              graph.disconnect_nodes(first_id, second_id);
            } else {
              graph.disconnect_nodes(second_id, first_id);
            }
          } else {
            graph.disconnect_nodes(first_id, second_id);
            graph.disconnect_nodes(second_id, first_id);
          }
        }
      }
    }
  }
}

fn filter_edges<TLog: EventLog>(
  provider: &mut FuzzyMetricsProvider<TLog>,
  graph: &mut FuzzyGraph,
  utility_rate: f64,
  edge_cutoff_threshold: f64,
) {
  let edges: Vec<(u64, u64)> = graph.all_edges().iter().map(|edge| (*edge.from_node(), *edge.to_node())).collect();
  let mut node_to_incoming_nodes: HashMap<u64, HashSet<u64>> = HashMap::new();
  for (from_node_id, to_node_id) in edges {
    if let Some(set) = node_to_incoming_nodes.get_mut(&to_node_id) {
      set.insert(from_node_id);
    } else {
      node_to_incoming_nodes.insert(to_node_id, HashSet::from_iter(vec![from_node_id]));
    }
  }

  for (node_id, incoming_nodes_ids) in node_to_incoming_nodes {
    if incoming_nodes_ids.len() == 0 {
      continue;
    }

    let incoming_nodes: Vec<u64> = incoming_nodes_ids.iter().map(|c| *c).collect();
    let mut utility_measures = vec![0.0; incoming_nodes.len()];
    let second = graph.node(&node_id).unwrap().data().unwrap();

    for i in 0..incoming_nodes.len() {
      let first = graph.node(incoming_nodes.get(i).unwrap()).unwrap().data().unwrap();
      utility_measures[i] = provider.utility_measure(first, second, utility_rate);
    }

    let min = utility_measures.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = utility_measures.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    for i in 0..utility_measures.len() {
      let normalized_measure = if max != min {
        (utility_measures[i] - min) / (max - min)
      } else {
        1.0
      };

      if normalized_measure < edge_cutoff_threshold {
        graph.disconnect_nodes(&incoming_nodes[i], &node_id);
      }
    }
  }
}

type ClustersMap = HashMap<u64, Rc<RefCell<OneSet<u64>>>>;

fn discover_clusters<TLog: EventLog>(provider: &mut FuzzyMetricsProvider<TLog>, graph: &mut FuzzyGraph, node_cutoff_threshold: f64) {
  let mut nodes_to_clusters: HashMap<u64, u64> = HashMap::new();
  let mut clusters = ClustersMap::new();

  find_initial_clusters(graph, provider, node_cutoff_threshold, &mut clusters, &mut nodes_to_clusters);
  merge_clusters(graph, provider, &mut nodes_to_clusters, &mut clusters);
  merge_nodes(graph, &clusters);
}

fn find_initial_clusters<TLog: EventLog>(
  graph: &mut FuzzyGraph,
  provider: &mut FuzzyMetricsProvider<TLog>,
  node_cutoff_threshold: f64,
  clusters: &mut ClustersMap,
  nodes_to_clusters: &mut HashMap<u64, u64>,
) {
  for node in graph.all_nodes() {
    let this_node_name = node.data().unwrap();
    let node_significance = provider.unary_frequency_significance(this_node_name);
    if node_significance >= node_cutoff_threshold {
      continue;
    }

    let connected_nodes = graph.all_connected_nodes(node.id());
    let mut max_corr = f64::NEG_INFINITY;
    let mut max_corr_node_id = None;

    for connected_node_id in connected_nodes {
      if connected_node_id != node.id() {
        let connected_node_name = graph.node(connected_node_id).unwrap().data().unwrap();
        let correlation = provider.proximity_correlation(this_node_name, connected_node_name);
        if correlation > max_corr {
          max_corr_node_id = Some(*connected_node_id);
          max_corr = correlation;
        }
      }
    }

    if let Some(max_corr_node_id) = max_corr_node_id {
      if let Some(cluster_id) = nodes_to_clusters.get(&max_corr_node_id) {
        clusters.get_mut(cluster_id).unwrap().borrow_mut().insert(*node.id());
        nodes_to_clusters.insert(*node.id(), *cluster_id);
      } else {
        let mut new_cluster = OneSet::empty();
        new_cluster.insert(*node.id());
        nodes_to_clusters.insert(*node.id(), *new_cluster.id());
        clusters.insert(*new_cluster.id(), Rc::new(RefCell::new(new_cluster)));
      }
    }
  }
}

fn merge_clusters<TLog: EventLog>(
  graph: &mut FuzzyGraph,
  provider: &mut FuzzyMetricsProvider<TLog>,
  nodes_to_clusters: &mut HashMap<u64, u64>,
  clusters: &mut ClustersMap,
) {
  'merging_clusters: loop {
    let current_clusters: Vec<u64> = clusters.iter().map(|c| *c.0).collect();

    for i in 0..current_clusters.len() {
      let cluster_id = current_clusters.get(i).unwrap();
      let cluster = clusters.get(cluster_id).unwrap().clone();

      let outgoing_nodes: HashSet<&u64> = cluster.borrow().set().iter().flat_map(|id| graph.outgoing_nodes(id)).collect();
      if try_merge_clusters(provider, graph, nodes_to_clusters, &outgoing_nodes, clusters, cluster.clone()) {
        continue 'merging_clusters;
      }

      let incoming_nodes: HashSet<&u64> = cluster.borrow().set().iter().flat_map(|id| graph.incoming_edges(id)).collect();
      if try_merge_clusters(provider, graph, nodes_to_clusters, &incoming_nodes, clusters, cluster.clone()) {
        continue 'merging_clusters;
      }
    }

    break;
  }
}

fn merge_nodes(graph: &mut FuzzyGraph, clusters: &ClustersMap) {
  for cluster in clusters.values() {
    let cluster = cluster.borrow();
    graph.merge_nodes_into_one(
      &cluster.set().iter().map(|id| *id).collect(),
      |nodes_data| {
        let mut cluster_data = String::new();
        cluster_data.push_str("Cluster[");
        for data in &nodes_data {
          if let Some(data) = data {
            cluster_data.push_str(data);
            cluster_data.push(',');
          }
        }

        if cluster_data.ends_with(',') {
          cluster_data.remove(cluster_data.len() - 1);
        }

        cluster_data.push_str("]");

        Some(cluster_data)
      },
      |edges_data| {
        edges_data.iter().fold(NodesConnectionData::empty(), |first, second| {
          NodesConnectionData::new(
            Some(*first.data().unwrap_or(&0.0) + second.data().unwrap_or(0.0)),
            first.weight() + second.weight,
            None,
          )
        })
      },
    );
  }
}

fn try_merge_clusters<TLog: EventLog>(
  provider: &mut FuzzyMetricsProvider<TLog>,
  graph: &FuzzyGraph,
  nodes_to_clusters: &mut HashMap<u64, u64>,
  nodes: &HashSet<&u64>,
  clusters: &mut HashMap<u64, Rc<RefCell<OneSet<u64>>>>,
  cluster: Rc<RefCell<OneSet<u64>>>,
) -> bool {
  let mut all_clusters = true;
  let mut outgoing_clusters = HashSet::new();
  for node_id in nodes {
    if !cluster.borrow().set().contains(node_id) && !nodes_to_clusters.contains_key(node_id) {
      all_clusters = false;
      break;
    }

    if cluster.borrow().set().contains(node_id) {
      continue;
    }

    let cluster_id = nodes_to_clusters.get(node_id).unwrap();
    outgoing_clusters.insert(cluster_id);
  }

  if all_clusters {
    let most_corr_cluster = find_most_correlated_cluster(provider, graph, &cluster.borrow(), &outgoing_clusters, clusters);
    if let Some(most_correlated_cluster) = most_corr_cluster {
      let new_cluster = cluster.borrow().merge(&most_correlated_cluster.borrow());
      for node in new_cluster.set() {
        if let Some(value) = nodes_to_clusters.get_mut(node) {
          *value = *new_cluster.id()
        } else {
          nodes_to_clusters.insert(*node, *new_cluster.id());
        }
      }

      clusters.remove(cluster.borrow().id());
      clusters.remove(most_correlated_cluster.borrow().id());
      clusters.insert(*new_cluster.id(), Rc::new(RefCell::new(new_cluster)));
      return true;
    }
  }

  false
}

fn clusters_correlation<TLog: EventLog>(
  provider: &mut FuzzyMetricsProvider<TLog>,
  graph: &FuzzyGraph,
  first_cluster: &OneSet<u64>,
  second_cluster: &OneSet<u64>,
) -> f64 {
  let mut corr = 0.0;
  let mut count = 0;
  for first_el in first_cluster.set() {
    for second_el in second_cluster.set() {
      let first_name = graph.node(first_el).unwrap().data().unwrap();
      let second_name = graph.node(second_el).unwrap().data().unwrap();
      corr += provider.proximity_correlation(first_name, second_name);
      count += 1;
    }
  }

  if count == 0 {
    0.0
  } else {
    corr / count as f64
  }
}

fn find_most_correlated_cluster<TLog: EventLog>(
  provider: &mut FuzzyMetricsProvider<TLog>,
  graph: &FuzzyGraph,
  cluster: &OneSet<u64>,
  candidates: &HashSet<&u64>,
  clusters: &HashMap<u64, Rc<RefCell<OneSet<u64>>>>,
) -> Option<Rc<RefCell<OneSet<u64>>>> {
  let mut max_corr = None;
  let mut most_correlated_cluster = None;
  for outgoing_cluster_id in candidates {
    let outgoing_cluster = clusters.get(outgoing_cluster_id).unwrap();
    let clusters_corr = clusters_correlation(provider, graph, cluster, &outgoing_cluster.borrow());
    if max_corr.is_none() || clusters_corr > max_corr.unwrap() {
      max_corr = Some(clusters_corr);
      most_correlated_cluster = Some(outgoing_cluster.clone());
    }
  }

  most_correlated_cluster
}

fn are_nodes_bi_connected(graph: &FuzzyGraph, first_node_id: &u64, second_node_id: &u64) -> bool {
  graph.are_nodes_connected(first_node_id, second_node_id) && graph.are_nodes_connected(second_node_id, first_node_id)
}
