use ficus::features::discovery::root_sequence::discovery::{discover_root_sequence_graph, discover_root_sequence, DiscoveryContext, RootSequenceKind};
use ficus::utils::references::HeapedOrOwned;
use ficus::utils::user_data::user_data::UserDataImpl;
use ficus::vecs;
use termgraph::{Config, DirectedGraph, ValueFormatter};

#[test]
pub fn test_root_sequence_graph_1() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "B", "D", "E"]
    ],
    vecs!["START", "A", "B", "C", "D", "E", "END"],
    vec![
      "[A]--[B]",
      "[B]--[C]",
      "[B]--[D]",
      "[C]--[D]",
      "[D]--[E]",
      "[E]--[END]",
      "[START]--[A]"
    ],
  );
}

#[test]
pub fn test_root_sequence_graph_2() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "X", "Y", "E"]
    ],
    vecs!["START", "A", "B", "C", "D", "E", "END"],
    vec![
      "[A]--[B]",
      "[A]--[X]",
      "[B]--[C]",
      "[C]--[D]",
      "[D]--[E]",
      "[E]--[END]",
      "[START]--[A]",
      "[X]--[Y]",
      "[Y]--[E]"
    ],
  );
}

#[test]
pub fn test_root_sequence_graph_3() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A"],
      vecs!["B"],
      vecs!["C"],
      vecs!["D"],
      vecs!["E"],
    ],
    vecs!["START", "END"],
    vec![
      "[A]--[END]",
      "[B]--[END]",
      "[C]--[END]",
      "[D]--[END]",
      "[E]--[END]",
      "[START]--[A]",
      "[START]--[B]",
      "[START]--[C]",
      "[START]--[D]",
      "[START]--[E]",
    ],
  )
}

#[test]
pub fn test_root_sequence_graph_4() {
  execute_root_sequence_discovery_test(
    vec![],
    vec![],
    vec![],
  )
}

#[test]
pub fn test_root_sequence_graph_5() {
  execute_root_sequence_discovery_test(
    vec![
      vecs![]
    ],
    vecs!["START", "END"],
    vec![
      "[START]--[END]"
    ],
  )
}

#[test]
pub fn test_root_sequence_graph_6() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A", "X", "B", "Y", "C", "Z", "D", "W", "E"],
      vecs!["X", "A", "Y", "B", "Z", "C", "W", "D"],
    ],
    vecs!["START", "A", "X", "B", "Y", "C", "Z", "D", "W", "E", "END"],
    vec![
      "[A]--[X]",
      "[A]--[Y]",
      "[B]--[Y]",
      "[B]--[Z]",
      "[C]--[W]",
      "[C]--[Z]",
      "[D]--[END]",
      "[D]--[W]",
      "[E]--[END]",
      "[START]--[A]",
      "[START]--[X]",
      "[W]--[D]",
      "[W]--[E]",
      "[X]--[A]",
      "[X]--[B]",
      "[Y]--[B]",
      "[Y]--[C]",
      "[Z]--[C]",
      "[Z]--[D]",
    ],
  )
}

#[test]
pub fn test_root_sequence_graph_7() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["X", "A", "Y", "B", "Z", "C", "W", "D", "Z", "E"],
      vecs!["A", "B", "C", "D", "E"]
    ],
    vecs!["START", "X", "A", "Y", "B", "Z", "C", "W", "D", "Z", "E", "END"],
    vec![
      "[A]--[B]",
      "[A]--[Y]",
      "[B]--[C]",
      "[B]--[Z]",
      "[C]--[D]",
      "[C]--[W]",
      "[D]--[E]",
      "[D]--[Z]",
      "[E]--[END]",
      "[START]--[A]",
      "[START]--[X]",
      "[W]--[D]",
      "[X]--[A]",
      "[Y]--[B]",
      "[Z]--[C]",
      "[Z]--[E]",
    ],
  );
}

#[test]
pub fn test_root_sequence_graph_8() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "X", "B", "C", "D", "E"],
      vecs!["A", "X", "C", "D", "E"],
      vecs!["A", "X", "D", "E"],
    ],
    vecs!["START", "A", "X", "B", "C", "D", "E", "END"],
    vec![
      "[A]--[B]",
      "[A]--[X]",
      "[B]--[C]",
      "[C]--[D]",
      "[D]--[E]",
      "[E]--[END]",
      "[START]--[A]",
      "[X]--[B]",
      "[X]--[C]",
      "[X]--[D]",
    ],
  );
}

#[test]
pub fn test_root_sequence_graph_9() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "X", "Y", "Z", "W", "B", "C", "D", "E"],
      vecs!["A", "Y", "Z", "W", "B", "C", "D", "E"],
      vecs!["A", "Z", "W", "B", "C", "D", "E"],
      vecs!["A", "X", "B", "C", "D", "E"],
    ],
    vecs!["START", "A", "Z", "W", "B", "C", "D", "E", "END"],
    vec![
      "[A]--[B]",
      "[A]--[X]",
      "[A]--[X]",
      "[A]--[Y]",
      "[A]--[Z]",
      "[B]--[C]",
      "[C]--[D]",
      "[D]--[E]",
      "[E]--[END]",
      "[START]--[A]",
      "[W]--[B]",
      "[X]--[B]",
      "[X]--[Y]",
      "[Y]--[Z]",
      "[Z]--[W]",
    ],
  )
}

#[test]
pub fn test_root_sequence_graph_10() {
  execute_root_sequence_discovery_test(
    vec![
      vecs!["A", "B", "C", "D", "E"],
      vecs!["A", "X", "Y", "Z", "W", "B", "C", "D", "E"],
      vecs!["A", "y", "z", "w", "B", "C", "D", "E"],
      vecs!["A", "V", "B", "C", "D", "E"],
    ],
    vecs!["START", "A", "B", "C", "D", "E", "END"],
    vec![
      "[A]--[B]",
      "[A]--[V]",
      "[A]--[X]",
      "[A]--[y]",
      "[B]--[C]",
      "[C]--[D]",
      "[D]--[E]",
      "[E]--[END]",
      "[START]--[A]",
      "[V]--[B]",
      "[W]--[B]",
      "[X]--[Y]",
      "[Y]--[Z]",
      "[Z]--[W]",
      "[w]--[B]",
      "[y]--[z]",
      "[z]--[w]",
    ],
  )
}

fn execute_root_sequence_discovery_test(mut traces: Vec<Vec<String>>, gold_root_sequence: Vec<String>, gold_graph_edges: Vec<&str>) {
  let factory = || (
    "START".to_string(),
    "END".to_string()
  );

  for trace in &mut traces {
    let (art_start, art_end) = factory();
    trace.push(art_end);
    trace.insert(0, art_start);
  }

  let root_sequence_kind = RootSequenceKind::FindBest;
  let root_sequence = discover_root_sequence(&traces, root_sequence_kind);
  assert_eq!(root_sequence, gold_root_sequence);

  let name_extractor = |s: &String| HeapedOrOwned::Owned(s.to_string());

  let to_node_data_transfer = |_: &String, _: &mut UserDataImpl| {};

  let context = DiscoveryContext::new(&name_extractor, &factory, root_sequence_kind, &to_node_data_transfer);

  let graph = discover_root_sequence_graph(&traces, &context);
  let test_result = graph.serialize_edges_deterministic();

  let gold = gold_graph_edges.join("\n");

  if test_result != gold {
    let mut tgraph = DirectedGraph::new();
    tgraph.add_nodes(graph.all_nodes().into_iter().map(|n| (*n.id(), n.data().unwrap().as_str().to_owned())));
    tgraph.add_edges(graph.all_edges().into_iter().map(|e| (*e.from_node(), *e.to_node())));

    let tconfig = Config::new(ValueFormatter::new(), 10).default_colors();

    termgraph::display(&tgraph, &tconfig);

    println!("GOLD:");
    println!("{}", gold);

    println!("TEST RESULT:");
    println!("{}", test_result);

    assert!(false);
  }
}