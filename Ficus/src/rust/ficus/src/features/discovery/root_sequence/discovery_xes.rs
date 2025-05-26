use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::patterns::activity_instances::{create_vector_of_underlying_events, try_get_base_pattern};
use crate::features::analysis::patterns::pattern_info::{UnderlyingPatternGraphInfo, UnderlyingPatternInfo, UNDERLYING_PATTERN_KIND_KEY};
use crate::features::discovery::petri_net::annotations::create_performance_map;
use crate::features::discovery::root_sequence::context::DiscoveryContext;
use crate::features::discovery::root_sequence::context_keys::{EDGE_SOFTWARE_DATA_KEY, EDGE_START_END_ACTIVITIES_TIMES_KEY, EDGE_TRACE_EXECUTION_INFO_KEY, NODE_CORRESPONDING_TRACE_DATA_KEY, NODE_MULTITHREADED_FRAGMENT_LOG_KEY, NODE_SOFTWARE_DATA_KEY, NODE_START_END_ACTIVITIES_TIMES_KEY, NODE_UNDERLYING_PATTERNS_GRAPHS_INFOS_KEY, NODE_UNDERLYING_PATTERNS_INFOS_KEY};
use crate::features::discovery::root_sequence::discovery::{create_new_graph_node, discover_root_sequence_graph};
use crate::features::discovery::root_sequence::models::{CorrespondingTraceData, DiscoverRootSequenceGraphError, EventCoordinates, EventWithUniqueId, NodeAdditionalDataContainer, RootSequenceKind};
use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use crate::utils::context_key::DefaultContextKey;
use crate::utils::display_name::get_display_name;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::user_data::user_data::{UserData, UserDataImpl, UserDataOwner};
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

pub fn discover_root_sequence_graph_from_event_log(
  log: &XesEventLogImpl,
  root_sequence_kind: RootSequenceKind,
  merge_sequences_of_events: bool,
) -> Result<DefaultGraph, DiscoverRootSequenceGraphError> {
  assert_all_traces_have_artificial_start_end_events(log)?;
  adjust_log_user_data(log);

  let name_extractor = |e: &Rc<RefCell<XesEventImpl>>| get_display_name(&e.borrow());

  let artificial_start_end_events_factory = || (
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(ARTIFICIAL_START_EVENT_NAME.to_string()))),
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(ARTIFICIAL_END_EVENT_NAME.to_string()))),
  );

  let event_to_node_data_transfer = |event: &Rc<RefCell<XesEventImpl>>, user_data_impl: &mut UserDataImpl, belongs_to_root_sequence: bool| {
    transfer_data_from_event_to_node_user_data(event, user_data_impl, belongs_to_root_sequence);
  };

  let event_to_edge_data_transfer = |event: &Rc<RefCell<XesEventImpl>>, user_data_impl: &mut UserDataImpl| {
    transfer_data_from_event_to_edge_user_data(event, user_data_impl);
  };

  let context = DiscoveryContext::new(
    &name_extractor,
    &artificial_start_end_events_factory,
    root_sequence_kind,
    &event_to_node_data_transfer,
    &event_to_edge_data_transfer
  );

  let performance_map = create_performance_map(log);

  let log = log.traces().iter().map(|t| t.borrow().events().clone()).collect();
  initialize_patterns_infos(&log);

  let log = log.into_iter().map(|t| t.into_iter().map(|e| EventWithUniqueId::new(e)).collect()).collect();

  let mut result = discover_root_sequence_graph(&log, &context, merge_sequences_of_events, Some(performance_map))?;
  discover_graphs_for_patterns(result.graph_mut(), &context);

  Ok(result.graph_move())
}

fn transfer_data_from_event_to_edge_user_data(event: &Rc<RefCell<XesEventImpl>>, user_data: &mut UserDataImpl) {
  let underlying_events = create_vector_of_underlying_events::<XesEventLogImpl>(event);
  let event = if underlying_events.is_empty() {
    event
  } else {
    underlying_events.last().unwrap()
  };

  transfer_vector_like_user_data(event, &EDGE_SOFTWARE_DATA_KEY, user_data);
  transfer_vector_like_user_data(event, &EDGE_TRACE_EXECUTION_INFO_KEY, user_data);
  transfer_vector_like_user_data(event, &EDGE_START_END_ACTIVITIES_TIMES_KEY, user_data);
}

fn transfer_data_from_event_to_node_user_data(event: &Rc<RefCell<XesEventImpl>>, user_data_impl: &mut UserDataImpl, belongs_to_root_sequence: bool) {
  transfer_vector_like_user_data(event, &NODE_SOFTWARE_DATA_KEY, user_data_impl);
  transfer_vector_like_user_data(event, &NODE_START_END_ACTIVITIES_TIMES_KEY, user_data_impl);
  transfer_vector_like_user_data(event, &NODE_UNDERLYING_PATTERNS_INFOS_KEY, user_data_impl);
  transfer_vector_like_user_data(event, &NODE_MULTITHREADED_FRAGMENT_LOG_KEY, user_data_impl);

  if let Some(corresponding_trace_data) = event.borrow().user_data().concrete(NODE_CORRESPONDING_TRACE_DATA_KEY.key()) {
    let new_trace_data = corresponding_trace_data.iter().map(|d| {
      let mut data = d.clone();
      data.value_mut().set_belongs_to_root_sequence(belongs_to_root_sequence);
      data
    }).collect();

    if let Some(existing_trace_data) = user_data_impl.concrete_mut(NODE_CORRESPONDING_TRACE_DATA_KEY.key()) {
      existing_trace_data.extend(new_trace_data);
    } else {
      user_data_impl.put_concrete(NODE_CORRESPONDING_TRACE_DATA_KEY.key(), new_trace_data);
    }
  }
}

fn initialize_patterns_infos(log: &Vec<Vec<Rc<RefCell<XesEventImpl>>>>) {
  for (trace_id, trace) in log.iter().enumerate() {
    for (event_index, event) in trace.iter().enumerate() {
      let pattern_kind = event.borrow().user_data().concrete(UNDERLYING_PATTERN_KIND_KEY.key()).cloned();
      if let Some(pattern_kind) = pattern_kind {
        let underlying_events = create_vector_of_underlying_events::<XesEventLogImpl>(event);
        let base_pattern = try_get_base_pattern::<XesEventLogImpl>(event);

        let pattern_info = UnderlyingPatternInfo::new(pattern_kind, underlying_events, base_pattern);

        let event_coordinates = EventCoordinates::new(trace_id as u64, event_index as u64);

        let patterns = vec![NodeAdditionalDataContainer::new(pattern_info, event_coordinates)];

        event.borrow_mut().user_data_mut().put_concrete(NODE_UNDERLYING_PATTERNS_INFOS_KEY.key(), patterns);
      }
    }
  }
}

fn discover_graphs_for_patterns(graph: &mut DefaultGraph, context: &DiscoveryContext<Rc<RefCell<XesEventImpl>>>) {
  for node in graph.all_nodes_mut() {
    let user_data = node.user_data_mut();

    let mut pattern_graph_infos = vec![];
    if let Some(patterns) = user_data.concrete(NODE_UNDERLYING_PATTERNS_INFOS_KEY.key()).cloned() {
      if patterns.len() == 0 {
        continue;
      }

      for pattern in &patterns {
        let mut graph = DefaultGraph::empty();
        let mut prev_node_id = None;

        for event in pattern.value().underlying_sequence() {
          let current_node_id = create_new_graph_node(&mut graph, &EventWithUniqueId::new(event.clone()), false, context, true);
          if let Some(prev_node) = prev_node_id {
            graph.connect_nodes(&prev_node, &current_node_id, NodesConnectionData::empty());
            transfer_data_from_event_to_edge_user_data(event, graph.edge_mut(&prev_node, &current_node_id).unwrap().user_data_mut());
          }

          prev_node_id = Some(current_node_id);
        }

        let graph = Rc::new(Box::new(graph));

        let base_sequence = match pattern.value().base_pattern() {
          None => None,
          Some(base_pattern) => Some(base_pattern.iter().map(|e| e.borrow().name().to_owned()).collect())
        };

        let pattern_graph_info = UnderlyingPatternGraphInfo::new(pattern.value().pattern_kind().clone(), base_sequence, graph);
        let pattern_graph_info = NodeAdditionalDataContainer::new(pattern_graph_info, pattern.original_event_coordinates().clone());

        pattern_graph_infos.push(pattern_graph_info);
      }

      user_data.put_concrete(NODE_UNDERLYING_PATTERNS_GRAPHS_INFOS_KEY.key(), pattern_graph_infos);
    }
  }
}

fn transfer_vector_like_user_data<T: Clone>(
  event: &Rc<RefCell<XesEventImpl>>,
  key: &DefaultContextKey<Vec<T>>,
  user_data_impl: &mut UserDataImpl,
) {
  if let Some(data) = event.borrow().user_data().concrete(key.key()) {
    if let Some(existing_data) = user_data_impl.concrete_mut(key.key()) {
      existing_data.extend(data.clone().into_iter());
    } else {
      user_data_impl.put_concrete(key.key(), data.clone());
    }
  }
}

fn adjust_log_user_data(log: &XesEventLogImpl) {
  for (trace_index, trace) in log.traces().iter().enumerate() {
    for (event_index, event) in trace.borrow().events().iter().enumerate() {
      let coordinates = EventCoordinates::new(trace_index as u64, event_index as u64);
      event.borrow_mut().user_data_mut().put_concrete(NODE_CORRESPONDING_TRACE_DATA_KEY.key(), vec![
        NodeAdditionalDataContainer::new(CorrespondingTraceData::new(false), coordinates)
      ]);

      adjust_event_coordinates(event, coordinates, &NODE_SOFTWARE_DATA_KEY);
      adjust_event_coordinates(event, coordinates, &NODE_START_END_ACTIVITIES_TIMES_KEY);
    }
  }
}

fn adjust_event_coordinates<T: Clone + Debug>(
  event: &Rc<RefCell<XesEventImpl>>,
  event_coordinates: EventCoordinates,
  key: &DefaultContextKey<Vec<NodeAdditionalDataContainer<T>>>,
) {
  if let Some(data) = event.borrow_mut().user_data_mut().concrete_mut(key.key()) {
    for data in data {
      data.set_new_event_coordinates(event_coordinates);
    }
  }
}

fn assert_all_traces_have_artificial_start_end_events(log: &XesEventLogImpl) -> Result<(), DiscoverRootSequenceGraphError> {
  for trace in log.traces().iter().map(|t| t.borrow()) {
    if !check_trace_have_artificial_start_end_events(trace.deref()) {
      return Err(DiscoverRootSequenceGraphError::NoArtificialStartEndEvents);
    }
  }

  Ok(())
}

fn check_trace_have_artificial_start_end_events(trace: &XesTraceImpl) -> bool {
  trace.events().len() >= 2 &&
    trace.events().first().unwrap().borrow().name().as_str() == ARTIFICIAL_START_EVENT_NAME &&
    trace.events().last().unwrap().borrow().name().as_str() == ARTIFICIAL_END_EVENT_NAME
}