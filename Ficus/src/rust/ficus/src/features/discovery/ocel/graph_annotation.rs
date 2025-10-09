use crate::utils::graph::graph::DefaultGraph;
use derive_new::new;
use std::collections::{HashMap, HashSet, VecDeque};
use getset::Getters;
use crate::features::discovery::root_sequence::context_keys::EDGE_SOFTWARE_DATA_KEY;
use crate::features::discovery::timeline::software_data::models::OcelObjectAction;
use crate::utils::user_data::user_data::UserData;

#[derive(new, Getters)]
pub struct OcelAnnotation {
  #[get = "pub"]
  nodes_to_states: HashMap<u64, ProcessNodesStates>
}

pub enum OcelAnnotationCreationError {
  FailedToFindStartNode,
  ObjectAlreadyExistsInNodeState,
  ConsumeNotExistingObject,
  OneOfMergedObjetsDoesNotExist
}

#[derive(Getters)]
pub struct NodeObjectsState {
  #[getset(get = "pub")]
  map: HashMap<String, HashSet<String>>
}

impl NodeObjectsState {
  pub fn new() -> Self {
    Self {
      map: HashMap::new()
    }
  }

  pub fn add_allocated_object(&mut self, object_type: String, object_id: String) -> Result<(), OcelAnnotationCreationError> {
    if self.contains_object(object_type.as_str(), object_id.as_str()) {
      return Err(OcelAnnotationCreationError::ObjectAlreadyExistsInNodeState)
    }

    self.type_set_mut(object_type.as_str()).insert(object_id);
    Ok(())
  }

  pub fn contains_object(&self, object_type: &str, object_id: &str) -> bool {
    if let Some(type_objects) = self.map.get(object_type) {
      type_objects.contains(object_id)
    } else {
      false
    }
  }

  pub fn contains_unknown_object(&self, object_id: &str) -> bool {
    self.map.values().any(|set| set.contains(object_id))
  }

  fn type_set_mut(&mut self, object_type: &str) -> &mut HashSet<String> {
    if !self.map.contains_key(object_type) {
      self.map.insert(object_type.to_string(), HashSet::new());
    }

    self.map.get_mut(object_type).unwrap()
  }
}

#[derive(new, Getters)]
pub struct ProcessNodesStates {
  #[get = "pub"]
  initial_objects: Option<NodeObjectsState>,
  #[get = "pub"]
  final_objects: NodeObjectsState
}

pub fn create_ocel_annotation_for_dag(graph: &DefaultGraph) -> Result<OcelAnnotation, OcelAnnotationCreationError> {
  let mut q = VecDeque::new();
  let start_node_id = graph.all_nodes().iter().find(|n| graph.incoming_edges(n.id()).len() == 0).map(|n| n.id().to_owned());

  if start_node_id.is_none() {
    return Err(OcelAnnotationCreationError::FailedToFindStartNode);
  }

  q.push_back(start_node_id.unwrap());

  let mut process_nodes_states = HashMap::new();

  'main_loop: loop {
    if q.is_empty() {
      break;
    }

    let node = q.pop_front().unwrap();

    let incoming_nodes = graph.incoming_edges(&node);
    for incoming_node in incoming_nodes.iter() {
      if !process_nodes_states.contains_key(*incoming_node) {
        q.push_back(node);
        continue 'main_loop;
      }
    }

    let mut new_node_state = NodeObjectsState::new();

    for incoming_node in incoming_nodes.iter() {
      let prev_state: &ProcessNodesStates = *process_nodes_states.get(*incoming_node).as_ref().unwrap();
      let edge = graph.edge(*incoming_node, &node);
      let edge = edge.as_ref().unwrap();

      if let Some(edge_software_data) = edge.user_data().concrete(EDGE_SOFTWARE_DATA_KEY.key()) {
        for data in edge_software_data {
          for ocel_data in data.ocel_data() {
            let obj_type = ocel_data.object_type();
            let obj_id = ocel_data.object_id();
            match ocel_data.action() {
              OcelObjectAction::Allocate => {
                new_node_state.add_allocated_object(obj_type.to_string(), obj_id.to_string())?;
              }
              OcelObjectAction::Consume => {
                if prev_state.final_objects.contains_object(obj_type, obj_id) {
                  return Err(OcelAnnotationCreationError::ConsumeNotExistingObject)
                }
              }
              OcelObjectAction::AllocateMerged(ids) => {
                for id in ids {
                  if !prev_state.final_objects.contains_unknown_object(id) {
                    return Err(OcelAnnotationCreationError::OneOfMergedObjetsDoesNotExist)
                  }
                }
              }
              OcelObjectAction::ConsumeWithProduce(_) => {
                if !prev_state.final_objects.contains_object(obj_type, obj_id) {
                  return Err(OcelAnnotationCreationError::ConsumeNotExistingObject)
                }
              }
            }
          }
        }
      }
    }

    process_nodes_states.insert(node, ProcessNodesStates::new(None, new_node_state));
  }

  Ok(OcelAnnotation::new(process_nodes_states))
}