use ficus::features::discovery::lcs::discovery::{adjust_lcs_graph_with_traces, initialize_lcs_graph_with_root_sequence};
use ficus::utils::graph::graph::DefaultGraph;
use ficus::utils::references::HeapedOrOwned;
use ficus::vecs;
use termgraph::{Config, DirectedGraph, ValueFormatter};

#[test]
pub fn simple_test() {
  execute_lcs_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "B", "D", "E"]
    ], 
    vecs!["A", "E"]
  );
}

fn execute_lcs_discovery_test(traces: Vec<Vec<String>>, root_sequence: Vec<String>) {
  let mut graph = DefaultGraph::empty();
  let name_extractor = |s: &String| HeapedOrOwned::Owned(s.to_string());

  let root_node_ids = initialize_lcs_graph_with_root_sequence(&root_sequence, &mut graph, name_extractor);
  adjust_lcs_graph_with_traces(&traces, &root_sequence, &root_node_ids, &mut graph, name_extractor);

  let mut tgraph = DirectedGraph::new();
  tgraph.add_nodes(graph.all_nodes().into_iter().map(|n| (*n.id(), n.data().unwrap().as_str().to_owned())));
  tgraph.add_edges(graph.all_edges().into_iter().map(|e| (*e.from_node(), *e.to_node())));

  let tconfig = Config::new(ValueFormatter::new(), 10).default_colors();
  termgraph::display(&tgraph, &tconfig);
}