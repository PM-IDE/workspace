use std::{env, fs, path::PathBuf};

pub fn get_test_data_path() -> PathBuf {
  let current_dir = env::current_dir().ok().unwrap();
  let root_dir = current_dir.parent().unwrap().parent().unwrap().parent().unwrap();
  root_dir.join("test_data")
}

pub fn get_test_data_sources_path() -> PathBuf {
  get_test_data_path().join("source")
}

pub fn get_test_data_rust_gold_path() -> PathBuf {
  get_test_data_path().join("gold").join("rust")
}

pub fn get_example_logs_gold_path() -> PathBuf {
  get_test_data_rust_gold_path().join("example_logs")
}

pub fn get_suffix_trees_gold_path() -> PathBuf {
  get_test_data_rust_gold_path().join("suffix_trees")
}

pub fn get_paths_to_example_logs() -> Vec<PathBuf> {
  let example_logs_dir = get_test_data_sources_path().join("example_logs");

  let mut logs = Vec::new();
  for path in fs::read_dir(example_logs_dir).unwrap() {
    let candidate_path = path.unwrap().path();
    if candidate_path.extension().unwrap().to_str().unwrap() == "xes" {
      logs.push(candidate_path);
    }
  }

  logs
}

pub fn create_example_log_gold_file_path(log_name: &str) -> PathBuf {
  get_example_logs_gold_path().join(log_name.to_owned() + ".gold")
}

pub fn get_paths_to_suffix_tree_string() -> Vec<PathBuf> {
  let example_logs_dir = get_test_data_sources_path().join("suffix_tree_strings");

  let mut paths = Vec::new();
  for path in fs::read_dir(example_logs_dir).unwrap() {
    let candidate_path = path.unwrap().path();
    if candidate_path.extension().unwrap().to_str().unwrap() == "txt" {
      paths.push(candidate_path);
    }
  }

  paths
}

pub fn create_suffix_tree_gold_file_path(file_name: &str) -> PathBuf {
  get_suffix_trees_gold_path().join(file_name.to_owned() + ".gold")
}

pub fn get_serialized_petri_nets_gold_path(test_name: &str) -> PathBuf {
  get_test_data_rust_gold_path()
    .join("pnml_petri_nets")
    .join(format!("{}.gold", test_name))
}
