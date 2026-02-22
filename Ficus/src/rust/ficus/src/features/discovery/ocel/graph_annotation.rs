use crate::{
  features::discovery::{ecfg::context_keys::EDGE_SOFTWARE_DATA_KEY, timeline::software_data::models::OcelObjectAction},
  utils::{graph::graph::DefaultGraph, references::HeapedOrOwned, user_data::user_data::UserData},
};
use derive_new::new;
use enum_display::EnumDisplay;
use getset::Getters;
use lazy_static::lazy_static;
use std::{
  collections::{HashMap, HashSet, VecDeque},
  rc::Rc,
};

#[derive(new, Getters)]
pub struct OcelAnnotation {
  #[get = "pub"]
  nodes_to_states: HashMap<u64, ProcessNodesStates>,
}

#[derive(EnumDisplay)]
pub enum OcelAnnotationCreationError {
  UnsupportedGraphKind,
  FailedToFindStartNode,
  ObjectAlreadyExistsInNodeState,
  ConsumeNotExistingObject,
  OneOfMergedObjetsDoesNotExist,
}

#[derive(Getters)]
#[derive(Default)]
pub struct NodeObjectsState {
  #[getset(get = "pub")]
  map: HashMap<HeapedOrOwned<String>, HashSet<HeapedOrOwned<String>>>,
}


impl NodeObjectsState {
  pub fn add_allocated_object(
    &mut self,
    object_type: HeapedOrOwned<String>,
    object_id: HeapedOrOwned<String>,
  ) -> Result<(), OcelAnnotationCreationError> {
    if self.contains_object(&object_type, &object_id) {
      return Err(OcelAnnotationCreationError::ObjectAlreadyExistsInNodeState);
    }

    self.type_set_mut(&object_type).insert(object_id);
    Ok(())
  }

  pub fn remove_object(&mut self, object_type: &HeapedOrOwned<String>, id: &HeapedOrOwned<String>) {
    let Some(set) = self.map.get_mut(object_type) else { return };

    set.remove(id);
  }

  pub fn remove_unknown_object(&mut self, id: &HeapedOrOwned<String>) {
    for (_, set) in self.map.iter_mut() {
      if set.remove(id) {
        return;
      }
    }
  }

  pub fn contains_object(&self, object_type: &HeapedOrOwned<String>, object_id: &HeapedOrOwned<String>) -> bool {
    if let Some(type_objects) = self.map.get(object_type) {
      type_objects.contains(object_id)
    } else {
      false
    }
  }

  pub fn contains_unknown_object(&self, object_id: &HeapedOrOwned<String>) -> bool {
    self.map.values().any(|set| set.contains(object_id))
  }

  fn type_set_mut(&mut self, object_type: &HeapedOrOwned<String>) -> &mut HashSet<HeapedOrOwned<String>> {
    if !self.map.contains_key(object_type) {
      self.map.insert(object_type.clone(), HashSet::new());
    }

    self.map.get_mut(object_type).unwrap()
  }

  pub fn add_state_from(&mut self, other: &NodeObjectsState) {
    for (obj_type, ids) in other.map.iter() {
      let set = self.map.entry(obj_type.to_owned()).or_default();

      for id in ids {
        set.insert(id.clone());
      }
    }
  }
}

#[derive(new, Getters)]
pub struct ProcessNodesStates {
  #[get = "pub"]
  initial_objects: Option<NodeObjectsState>,
  #[get = "pub"]
  final_objects: NodeObjectsState,
  #[get = "pub"]
  incoming_objects_relations: Vec<OcelObjectRelations>,
}

#[derive(new, Getters)]
pub struct OcelObjectRelations {
  #[get = "pub"]
  object_id: HeapedOrOwned<String>,
  #[get = "pub"]
  from_element_id: u64,
  #[get = "pub"]
  related_objects_ids: Vec<HeapedOrOwned<String>>,
}

lazy_static! {
  pub static ref UNKNOWN_TYPE: Box<String> = Box::new("UNKNOWN".to_string());
}

pub fn create_ocel_annotation_for_dag(graph: &DefaultGraph) -> Result<OcelAnnotation, OcelAnnotationCreationError> {
  let Some(kind) = graph.kind.as_ref() else {
    return Err(OcelAnnotationCreationError::UnsupportedGraphKind);
  };
  if !kind.is_dag() {
    return Err(OcelAnnotationCreationError::UnsupportedGraphKind);
  }

  let mut q = VecDeque::new();
  let start_node_id = graph
    .all_nodes()
    .iter()
    .find(|n| graph.incoming_edges(n.id()).is_empty())
    .map(|n| n.id().to_owned());

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

    let mut new_node_state: NodeObjectsState = Default::default();
    let mut new_node_objects_relations = vec![];

    let fallback_type = HeapedOrOwned::Heaped(Rc::new(UNKNOWN_TYPE.clone()));

    for incoming_node in incoming_nodes.iter() {
      let prev_state: &ProcessNodesStates = process_nodes_states.get(*incoming_node).as_ref().unwrap();
      let edge = graph.edge(incoming_node, &node);
      let edge = edge.as_ref().unwrap();

      new_node_state.add_state_from(prev_state.final_objects());

      if let Some(edge_software_data) = edge.user_data().concrete(EDGE_SOFTWARE_DATA_KEY.key()) {
        for data in edge_software_data {
          for ocel_data in data.ocel_data() {
            let obj_id = ocel_data.object_id();

            match ocel_data.action() {
              OcelObjectAction::Allocate(data) => {
                let obj_type = data.r#type().as_ref().unwrap_or(&fallback_type);
                new_node_state.add_allocated_object(obj_type.to_owned(), obj_id.to_owned())?;
              }
              OcelObjectAction::Consume(data) => {
                let obj_type = data.r#type().as_ref().unwrap_or(&fallback_type);
                if prev_state.final_objects.contains_object(obj_type, obj_id) {
                  return Err(OcelAnnotationCreationError::ConsumeNotExistingObject);
                }

                new_node_state.remove_object(obj_type, obj_id);
              }
              OcelObjectAction::AllocateMerged(data) => {
                let mut related_objects = vec![];

                for id in data.data() {
                  if !prev_state.final_objects.contains_unknown_object(id) {
                    return Err(OcelAnnotationCreationError::OneOfMergedObjetsDoesNotExist);
                  }

                  new_node_state.remove_unknown_object(id);
                  related_objects.push(id.clone());
                }

                let relations = OcelObjectRelations::new(obj_id.to_owned(), **incoming_node, related_objects);
                new_node_objects_relations.push(relations);

                let obj_type = data.r#type().as_ref().unwrap_or(&fallback_type);
                new_node_state.add_allocated_object(obj_type.clone(), obj_id.clone())?;
              }
              OcelObjectAction::ConsumeWithProduce(data) => {
                if !prev_state.final_objects.contains_unknown_object(obj_id) {
                  return Err(OcelAnnotationCreationError::ConsumeNotExistingObject);
                }

                for produced_obj in data.iter() {
                  let obj_type = produced_obj.r#type().as_ref().unwrap_or(&fallback_type);
                  let id = produced_obj.id();
                  new_node_state.add_allocated_object(obj_type.clone(), id.clone())?;

                  let relations = OcelObjectRelations::new(id.clone(), **incoming_node, vec![obj_id.clone()]);
                  new_node_objects_relations.push(relations);
                }

                new_node_state.remove_unknown_object(obj_id);
              }
            }
          }
        }
      }
    }

    process_nodes_states.insert(node, ProcessNodesStates::new(None, new_node_state, new_node_objects_relations));

    for outgoing_node in graph.outgoing_nodes(&node) {
      q.push_back(*outgoing_node);
    }
  }

  Ok(OcelAnnotation::new(process_nodes_states))
}
