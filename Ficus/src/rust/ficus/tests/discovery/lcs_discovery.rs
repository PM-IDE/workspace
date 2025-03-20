use ficus::features::discovery::lcs::discovery::{discover_lcs_graph};
use ficus::utils::graph::graph::DefaultGraph;
use ficus::utils::references::HeapedOrOwned;
use ficus::vecs;
use termgraph::{Config, DirectedGraph, ValueFormatter};

#[test]
pub fn test_lcs_graph_1() {
  execute_lcs_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "B", "D", "E"]
    ],
  );
}

#[test]
pub fn test_lcs_graph_2() {
  execute_lcs_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "X", "Y", "E"]
    ],
  );
}

fn execute_lcs_discovery_test(mut traces: Vec<Vec<String>>) {
  let name_extractor = |s: &String| HeapedOrOwned::Owned(s.to_string());

  let factory = || (
    "START".to_string(),
    "END".to_string()
  );

  for trace in &mut traces {
    let (art_start, art_end) = factory();
    trace.push(art_end);
    trace.insert(0, art_start);
  }

  let graph = discover_lcs_graph(&traces, &name_extractor, &factory);

  let mut tgraph = DirectedGraph::new();
  tgraph.add_nodes(graph.all_nodes().into_iter().map(|n| (*n.id(), n.data().unwrap().as_str().to_owned())));
  tgraph.add_edges(graph.all_edges().into_iter().map(|e| (*e.from_node(), *e.to_node())));

  let tconfig = Config::new(ValueFormatter::new(), 10).default_colors();

  termgraph::display(&tgraph, &tconfig);
}