use crate::utils::graph::graph::DefaultGraph;
use std::collections::HashMap;

pub fn serialize(graph: &DefaultGraph) -> String {
  let mut result = String::default();

  result.push_str(&graph.all_nodes().len().to_string());
  result.push_str("\n");

  let mut ids_to_indices = HashMap::new();
  for (idx, node) in graph.all_nodes().iter().enumerate() {
    ids_to_indices.insert(node.id, idx);

    result.push_str(node.data.as_ref().expect("must contain name").as_ref());
    result.push_str("\n");
  }

  result.push_str("0\n0\n");

  for edge in graph.all_edges() {
    let from_idx = ids_to_indices[&edge.from_node];
    let to_idx = ids_to_indices[&edge.to_node];

    result.push_str(format!("{from_idx}>{to_idx}x{}", edge.weight as i64).as_str());
    result.push_str("\n");
  }

  result
}
