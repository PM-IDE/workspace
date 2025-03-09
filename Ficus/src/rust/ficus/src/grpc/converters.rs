use std::collections::HashMap;
use std::fmt::Display;
use std::{any::Any, str::FromStr};

use crate::features::analysis::log_info::event_log_info::{EventLogInfo, OfflineEventLogInfo};
use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::discovery::timeline::discovery::{LogPoint, LogTimelineDiagram};
use crate::features::clustering::activities::activities_params::ActivityRepresentationSource;
use crate::features::clustering::traces::traces_params::TracesRepresentationSource;
use crate::features::discovery::petri_net::annotations::TimeAnnotationKind;
use crate::features::discovery::petri_net::arc::Arc;
use crate::features::discovery::petri_net::marking::{Marking, SingleMarking};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::ficus_proto::grpc_annotation::Annotation::{CountAnnotation, FrequencyAnnotation, TimeAnnotation};
use crate::ficus_proto::grpc_context_value::ContextValue::Annotation;
use crate::ficus_proto::{GrpcAnnotation, GrpcBytes, GrpcColorsEventLogMapping, GrpcCountAnnotation, GrpcDataset, GrpcEntityCountAnnotation, GrpcEntityFrequencyAnnotation, GrpcEntityTimeAnnotation, GrpcFrequenciesAnnotation, GrpcGraph, GrpcGraphEdge, GrpcGraphNode, GrpcLabeledDataset, GrpcLogPoint, GrpcLogTimelineDiagram, GrpcMatrix, GrpcMatrixRow, GrpcPetriNet, GrpcPetriNetArc, GrpcPetriNetMarking, GrpcPetriNetPlace, GrpcPetriNetSinglePlaceMarking, GrpcPetriNetTransition, GrpcThread, GrpcThreadEvent, GrpcTimePerformanceAnnotation, GrpcTimeSpan, GrpcTimelineTraceEventsGroup, GrpcTraceTimelineDiagram};
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::keys::context_keys::{
  BYTES_KEY, COLORS_EVENT_LOG_KEY, EVENT_LOG_INFO_KEY, GRAPH_KEY, GRAPH_TIME_ANNOTATION_KEY, HASHES_EVENT_LOG_KEY,
  LABELED_LOG_TRACES_DATASET_KEY, LABELED_TRACES_ACTIVITIES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, LOG_TRACES_DATASET_KEY,
  NAMES_EVENT_LOG_KEY, PATH_KEY, PATTERNS_KEY, PETRI_NET_COUNT_ANNOTATION_KEY, PETRI_NET_FREQUENCY_ANNOTATION_KEY, PETRI_NET_KEY,
  PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY, REPEAT_SETS_KEY, TRACES_ACTIVITIES_DATASET_KEY,
};
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::utils::colors::ColorsEventLog;
use crate::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use crate::utils::distance::distance::FicusDistance;
use crate::utils::graph::graph::{DefaultGraph, Graph};
use crate::utils::graph::graph_edge::GraphEdge;
use crate::utils::graph::graph_node::GraphNode;
use crate::utils::log_serialization_format::LogSerializationFormat;
use crate::{
  features::analysis::patterns::{
    activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy, repeat_sets::SubArrayWithTraceIndex,
    tandem_arrays::SubArrayInTraceInfo,
  },
  ficus_proto::{
    grpc_context_value::ContextValue, GrpcColor, GrpcColoredRectangle, GrpcColorsEventLog, GrpcColorsTrace, GrpcContextValue,
    GrpcEventLogInfo, GrpcEventLogTraceSubArraysContextValue, GrpcHashesEventLog, GrpcHashesEventLogContextValue, GrpcHashesLogTrace,
    GrpcNamesEventLog, GrpcNamesEventLogContextValue, GrpcNamesTrace, GrpcSubArrayWithTraceIndex,
    GrpcSubArraysWithTraceIndexContextValue, GrpcTraceSubArray, GrpcTraceSubArrays,
  },
  pipelines::{keys::context_key::ContextKey, pipelines::Pipeline},
  utils::{
    colors::{Color, ColoredRectangle},
    user_data::{keys::Key, user_data::UserData},
  },
};
use nameof::name_of_type;
use prost::{DecodeError, Message};
use crate::event_log::core::event::event::Event;

pub(super) fn context_value_from_bytes(bytes: &[u8]) -> Result<GrpcContextValue, DecodeError> {
  GrpcContextValue::decode(bytes)
}

pub(super) fn put_into_user_data(
  key: &dyn Key,
  value: &ContextValue,
  user_data: &mut impl UserData,
  context: &ServicePipelineExecutionContext,
) {
  match value {
    ContextValue::String(string) => user_data.put_any::<String>(key, string.clone()),
    ContextValue::HashesLog(_) => todo!(),
    ContextValue::NamesLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
    ContextValue::Uint32(number) => user_data.put_any::<u32>(key, number.clone()),
    ContextValue::TracesSubArrays(_) => todo!(),
    ContextValue::TraceIndexSubArrays(_) => todo!(),
    ContextValue::Bool(bool) => user_data.put_any::<bool>(key, bool.clone()),
    ContextValue::XesEventLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
    ContextValue::ColorsLog(_) => {}
    ContextValue::Enum(grpc_enum) => {
      let enum_name = &grpc_enum.enum_type;
      if enum_name == name_of_type!(PatternsDiscoveryStrategy) {
        parse_grpc_enum::<PatternsDiscoveryStrategy>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(AdjustingMode) {
        parse_grpc_enum::<AdjustingMode>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(PatternsKindDto) {
        parse_grpc_enum::<PatternsKindDto>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(UndefActivityHandlingStrategyDto) {
        parse_grpc_enum::<UndefActivityHandlingStrategyDto>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(ActivityNarrowingKind) {
        parse_grpc_enum::<ActivityNarrowingKind>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(ActivityInTraceFilterKind) {
        parse_grpc_enum::<ActivityInTraceFilterKind>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(ActivitiesLogsSourceDto) {
        parse_grpc_enum::<ActivitiesLogsSourceDto>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(ActivityRepresentationSource) {
        parse_grpc_enum::<ActivityRepresentationSource>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(FicusDistance) {
        parse_grpc_enum::<FicusDistance>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(TracesRepresentationSource) {
        parse_grpc_enum::<TracesRepresentationSource>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(LogSerializationFormat) {
        parse_grpc_enum::<LogSerializationFormat>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(TimeAnnotationKind) {
        parse_grpc_enum::<TimeAnnotationKind>(user_data, key, &grpc_enum.value);
      }
    }
    ContextValue::EventLogInfo(_) => todo!(),
    ContextValue::Strings(strings) => user_data.put_any::<Vec<String>>(key, strings.strings.clone()),
    ContextValue::Pipeline(pipeline) => {
      let pipeline = context.with_pipeline(pipeline).to_pipeline();
      user_data.put_any::<Pipeline>(key, pipeline);
    }
    ContextValue::PetriNet(_) => todo!(),
    ContextValue::Graph(_) => todo!(),
    ContextValue::Float(value) => user_data.put_any::<f64>(key, *value as f64),
    ContextValue::Annotation(_) => todo!(),
    ContextValue::Dataset(_) => todo!(),
    ContextValue::LabeledDataset(_) => todo!(),
    ContextValue::Bytes(grpc_bytes) => user_data.put_any::<Vec<u8>>(key, grpc_bytes.bytes.clone()),
    ContextValue::LogTimelineDiagram(_) => todo!(),
  }
}

fn parse_grpc_enum<TEnum: FromStr + 'static>(user_data: &mut impl UserData, key: &dyn Key, raw_enum: &str) {
  if let Ok(parsed_value) = TEnum::from_str(raw_enum) {
    user_data.put_any::<TEnum>(key, parsed_value);
  }
}

fn put_names_log_to_context(key: &dyn Key, grpc_log: &GrpcNamesEventLogContextValue, user_data: &mut impl UserData) {
  let grpc_log = grpc_log.log.as_ref().unwrap();
  let mut names_log = vec![];
  for grpc_trace in &grpc_log.traces {
    let mut trace = vec![];
    for grpc_event in &grpc_trace.events {
      trace.push(grpc_event.clone());
    }

    names_log.push(trace);
  }

  user_data.put_any::<Vec<Vec<String>>>(key, names_log);
}

pub fn convert_to_grpc_context_value(key: &dyn ContextKey, value: &dyn Any) -> Option<GrpcContextValue> {
  if PATH_KEY.eq_other(key) {
    try_convert_to_string_context_value(value)
  } else if HASHES_EVENT_LOG_KEY.eq_other(key) {
    try_convert_to_hashes_event_log(value)
  } else if NAMES_EVENT_LOG_KEY.eq_other(key) {
    try_convert_to_names_event_log(value)
  } else if PATTERNS_KEY.eq_other(key) {
    try_convert_to_grpc_traces_sub_arrays(value)
  } else if REPEAT_SETS_KEY.eq_other(key) {
    try_convert_to_grpc_sub_arrays_with_index(value)
  } else if COLORS_EVENT_LOG_KEY.eq_other(key) {
    try_convert_to_grpc_colors_event_log(value)
  } else if EVENT_LOG_INFO_KEY.eq_other(key) {
    try_convert_to_grpc_event_log_info(value)
  } else if PETRI_NET_KEY.eq_other(key) {
    try_convert_to_grpc_petri_net(value)
  } else if GRAPH_KEY.eq_other(key) {
    try_convert_to_grpc_graph(value)
  } else if PETRI_NET_COUNT_ANNOTATION_KEY.eq_other(key) {
    try_convert_to_grpc_petri_net_count_annotation(value)
  } else if PETRI_NET_FREQUENCY_ANNOTATION_KEY.eq_other(key) {
    try_convert_to_grpc_petri_net_frequency_annotation(value)
  } else if PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY.eq_other(key) {
    try_convert_to_grpc_petri_net_frequency_annotation(value)
  } else if GRAPH_TIME_ANNOTATION_KEY.eq_other(key) {
    try_convert_to_grpc_graph_time_annotation(value)
  } else if TRACES_ACTIVITIES_DATASET_KEY.eq_other(key) {
    try_convert_to_grpc_dataset(value)
  } else if LABELED_TRACES_ACTIVITIES_DATASET_KEY.eq_other(key) {
    try_convert_to_grpc_labeled_dataset(value)
  } else if LABELED_LOG_TRACES_DATASET_KEY.eq_other(key) {
    try_convert_to_grpc_labeled_dataset(value)
  } else if LOG_TRACES_DATASET_KEY.eq_other(key) {
    try_convert_to_grpc_dataset(value)
  } else if BYTES_KEY.eq_other(key) {
    try_convert_to_grpc_bytes(value)
  } else if LOG_THREADS_DIAGRAM_KEY.eq_other(key) {
    try_convert_to_grpc_log_threads_diagram(value)
  } else {
    None
  }
}

fn try_convert_to_grpc_bytes(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<Vec<u8>>() {
    None
  } else {
    let value = value.downcast_ref::<Vec<u8>>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(ContextValue::Bytes(GrpcBytes { bytes: value.clone() })),
    })
  }
}

fn try_convert_to_grpc_petri_net_count_annotation(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<HashMap<u64, usize>>() {
    None
  } else {
    let value = value.downcast_ref::<HashMap<u64, usize>>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(Annotation(GrpcAnnotation {
        annotation: Some(CountAnnotation(convert_to_grpc_count_annotation(value))),
      })),
    })
  }
}

fn try_convert_to_grpc_labeled_dataset(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<LabeledDataset>() {
    None
  } else {
    let value = value.downcast_ref::<LabeledDataset>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(ContextValue::LabeledDataset(convert_to_labeled_grpc_dataset(value))),
    })
  }
}

fn try_convert_to_grpc_petri_net_frequency_annotation(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<HashMap<u64, f64>>() {
    None
  } else {
    let value = value.downcast_ref::<HashMap<u64, f64>>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(Annotation(GrpcAnnotation {
        annotation: Some(FrequencyAnnotation(convert_to_grpc_frequency_annotation(value))),
      })),
    })
  }
}

fn try_convert_to_grpc_graph_time_annotation(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<HashMap<u64, f64>>() {
    None
  } else {
    let value = value.downcast_ref::<HashMap<u64, f64>>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(Annotation(GrpcAnnotation {
        annotation: Some(TimeAnnotation(convert_to_grpc_time_annotation(value))),
      })),
    })
  }
}

fn try_convert_to_string_context_value(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<String>() {
    None
  } else {
    Some(GrpcContextValue {
      context_value: Some(ContextValue::String(value.downcast_ref::<String>().unwrap().clone())),
    })
  }
}

fn try_convert_to_hashes_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<Vec<Vec<u64>>>() {
    None
  } else {
    let vec = value.downcast_ref::<Vec<Vec<u64>>>().unwrap();
    let mut traces = vec![];
    for trace in vec {
      let mut events = vec![];
      for event in trace {
        events.push(*event);
      }

      traces.push(GrpcHashesLogTrace { events });
    }

    Some(GrpcContextValue {
      context_value: Some(ContextValue::HashesLog(GrpcHashesEventLogContextValue {
        log: Some(GrpcHashesEventLog { traces }),
      })),
    })
  }
}

fn try_convert_to_names_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<Vec<Vec<String>>>() {
    None
  } else {
    let vec = value.downcast_ref::<Vec<Vec<String>>>().unwrap();
    let mut traces = vec![];
    for trace in vec {
      let mut events = vec![];
      for event in trace {
        events.push(event.clone());
      }

      traces.push(GrpcNamesTrace { events });
    }

    Some(GrpcContextValue {
      context_value: Some(ContextValue::NamesLog(GrpcNamesEventLogContextValue {
        log: Some(GrpcNamesEventLog { traces }),
      })),
    })
  }
}

fn try_convert_to_grpc_traces_sub_arrays(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<Vec<Vec<SubArrayInTraceInfo>>>() {
    None
  } else {
    let vec = value.downcast_ref::<Vec<Vec<SubArrayInTraceInfo>>>().unwrap();
    let mut traces = vec![];
    for trace in vec {
      let mut sub_arrays = vec![];
      for array in trace {
        sub_arrays.push(GrpcTraceSubArray {
          start: array.start_index as u32,
          end: (array.start_index + array.length) as u32,
        })
      }

      traces.push(GrpcTraceSubArrays { sub_arrays })
    }

    Some(GrpcContextValue {
      context_value: Some(ContextValue::TracesSubArrays(GrpcEventLogTraceSubArraysContextValue {
        traces_sub_arrays: traces,
      })),
    })
  }
}

fn try_convert_to_grpc_sub_arrays_with_index(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<Vec<SubArrayWithTraceIndex>>() {
    None
  } else {
    let vec = value.downcast_ref::<Vec<SubArrayWithTraceIndex>>().unwrap();
    let mut sub_arrays = vec![];

    for array in vec {
      sub_arrays.push(GrpcSubArrayWithTraceIndex {
        sub_array: Some(GrpcTraceSubArray {
          start: array.sub_array.start_index as u32,
          end: (array.sub_array.start_index + array.sub_array.length) as u32,
        }),
        trace_index: array.trace_index as u32,
      })
    }

    Some(GrpcContextValue {
      context_value: Some(ContextValue::TraceIndexSubArrays(GrpcSubArraysWithTraceIndexContextValue {
        sub_arrays,
      })),
    })
  }
}

fn try_convert_to_grpc_colors_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<ColorsEventLog>() {
    None
  } else {
    let colors_log = value.downcast_ref::<ColorsEventLog>().unwrap();
    let mut grpc_traces = vec![];
    let mut mapping = HashMap::new();
    let mut grpc_mapping = vec![];
    for (key, color) in colors_log.mapping.iter() {
      mapping.insert(key.to_owned(), grpc_mapping.len());
      grpc_mapping.push(GrpcColorsEventLogMapping {
        name: key.to_owned(),
        color: Some(convert_to_grpc_color(color)),
      });
    }

    for trace in &colors_log.traces {
      let mut grpc_trace = vec![];
      let mut last_width = None;
      let mut same_width = true;

      for colored_rect in trace {
        if same_width {
          if let Some(last_width) = last_width {
            if last_width != colored_rect.len() {
              same_width = false;
            }
          }

          last_width = Some(colored_rect.len())
        }

        let index = *mapping.get(colored_rect.name()).unwrap();
        grpc_trace.push(convert_to_grpc_colored_rect(colored_rect, index))
      }

      grpc_traces.push(GrpcColorsTrace {
        event_colors: grpc_trace,
        constant_width: same_width,
      })
    }

    Some(GrpcContextValue {
      context_value: Some(ContextValue::ColorsLog(GrpcColorsEventLog {
        mapping: grpc_mapping,
        traces: grpc_traces,
        adjustments: vec![],
      })),
    })
  }
}

fn convert_to_grpc_colored_rect(colored_rect: &ColoredRectangle, color_index: usize) -> GrpcColoredRectangle {
  GrpcColoredRectangle {
    color_index: color_index as u32,
    start_x: colored_rect.start_x(),
    length: colored_rect.len(),
  }
}

fn convert_to_grpc_color(color: &Color) -> GrpcColor {
  GrpcColor {
    red: color.red() as u32,
    green: color.green() as u32,
    blue: color.blue() as u32,
  }
}

fn try_convert_to_grpc_event_log_info(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<OfflineEventLogInfo>() {
    None
  } else {
    let log_info = value.downcast_ref::<OfflineEventLogInfo>().unwrap();
    if log_info.counts().is_none() {
      return None;
    }

    Some(GrpcContextValue {
      context_value: Some(ContextValue::EventLogInfo(GrpcEventLogInfo {
        events_count: log_info.counts().unwrap().events_count() as u32,
        traces_count: log_info.counts().unwrap().traces_count() as u32,
        event_classes_count: log_info.event_classes_count() as u32,
      })),
    })
  }
}

fn try_convert_to_grpc_petri_net(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<DefaultPetriNet>() {
    None
  } else {
    let petri_net = value.downcast_ref::<DefaultPetriNet>().unwrap();
    let grpc_places: Vec<GrpcPetriNetPlace> = petri_net.all_places().iter().map(|place| convert_to_grpc_place(place)).collect();

    let grpc_transitions: Vec<GrpcPetriNetTransition> = petri_net
      .all_transitions()
      .iter()
      .map(|transition| convert_to_grpc_transition(transition))
      .collect();

    Some(GrpcContextValue {
      context_value: Some(ContextValue::PetriNet(GrpcPetriNet {
        places: grpc_places,
        transitions: grpc_transitions,
        initial_marking: try_convert_to_grpc_marking(petri_net.initial_marking()),
        final_marking: try_convert_to_grpc_marking(petri_net.final_marking()),
      })),
    })
  }
}

fn convert_to_grpc_place(place: &Place) -> GrpcPetriNetPlace {
  GrpcPetriNetPlace {
    id: place.id() as i64,
    name: place.name().to_owned(),
  }
}

fn convert_to_grpc_transition<TTransitionData, TArcData>(transition: &Transition<TTransitionData, TArcData>) -> GrpcPetriNetTransition
where
  TTransitionData: ToString,
{
  let incoming_arcs = transition
    .incoming_arcs()
    .iter()
    .map(|arc| convert_to_grpc_arc(arc))
    .collect::<Vec<GrpcPetriNetArc>>();

  let outgoing_arcs = transition
    .outgoing_arcs()
    .iter()
    .map(|arc| convert_to_grpc_arc(arc))
    .collect::<Vec<GrpcPetriNetArc>>();

  GrpcPetriNetTransition {
    id: transition.id() as i64,
    incoming_arcs,
    outgoing_arcs,
    data: match transition.data() {
      None => "".to_string(),
      Some(data) => data.to_string(),
    },
  }
}

fn convert_to_grpc_arc<TArcData>(arc: &Arc<TArcData>) -> GrpcPetriNetArc {
  GrpcPetriNetArc {
    id: arc.id() as i64,
    place_id: arc.place_id() as i64,
    tokens_count: *arc.tokens_count() as i64,
  }
}

fn try_convert_to_grpc_marking(marking: Option<&Marking>) -> Option<GrpcPetriNetMarking> {
  match marking {
    None => None,
    Some(marking) => Some(GrpcPetriNetMarking {
      markings: marking
        .active_places()
        .iter()
        .map(|single_marking| convert_to_grpc_single_marking(single_marking))
        .collect(),
    }),
  }
}

fn convert_to_grpc_single_marking(marking: &SingleMarking) -> GrpcPetriNetSinglePlaceMarking {
  GrpcPetriNetSinglePlaceMarking {
    place_id: marking.place_id() as i64,
    tokens_count: marking.tokens_count() as i64,
  }
}

fn try_convert_to_grpc_graph(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<DefaultGraph>() {
    None
  } else {
    let graph = value.downcast_ref::<DefaultGraph>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(ContextValue::Graph(convert_to_grpc_graph(graph))),
    })
  }
}

fn convert_to_grpc_graph<TNodeData, TEdgeData>(graph: &Graph<TNodeData, TEdgeData>) -> GrpcGraph
where
  TNodeData: ToString,
  TEdgeData: ToString + Display,
{
  let nodes: Vec<GrpcGraphNode> = graph.all_nodes().iter().map(|node| convert_to_grpc_graph_node(*node)).collect();
  let edges: Vec<GrpcGraphEdge> = graph.all_edges().iter().map(|edge| convert_to_grpc_graph_edge(edge)).collect();

  GrpcGraph { edges, nodes }
}

fn convert_to_grpc_graph_node<TNodeData>(node: &GraphNode<TNodeData>) -> GrpcGraphNode
where
  TNodeData: ToString,
{
  GrpcGraphNode {
    id: *node.id(),
    data: match node.data() {
      None => "".to_string(),
      Some(data) => data.to_string(),
    },
  }
}

fn convert_to_grpc_graph_edge<TEdgeData>(edge: &GraphEdge<TEdgeData>) -> GrpcGraphEdge
where
  TEdgeData: ToString,
{
  GrpcGraphEdge {
    id: *edge.id(),
    from_node: *edge.from_node(),
    to_node: *edge.to_node(),
    weight: edge.weight,
    data: match edge.data() {
      None => "".to_string(),
      Some(data) => data.to_string(),
    },
  }
}

fn convert_to_grpc_count_annotation(annotation: &HashMap<u64, usize>) -> GrpcCountAnnotation {
  let annotations = annotation
    .iter()
    .map(|pair| GrpcEntityCountAnnotation {
      entity_id: *pair.0 as i64,
      count: *pair.1 as i64,
    })
    .collect();

  GrpcCountAnnotation { annotations }
}

fn convert_to_grpc_frequency_annotation(annotation: &HashMap<u64, f64>) -> GrpcFrequenciesAnnotation {
  let annotations = annotation
    .iter()
    .map(|pair| GrpcEntityFrequencyAnnotation {
      entity_id: *pair.0 as i64,
      frequency: *pair.1 as f32,
    })
    .collect();

  GrpcFrequenciesAnnotation { annotations }
}

fn convert_to_grpc_time_annotation(annotation: &HashMap<u64, f64>) -> GrpcTimePerformanceAnnotation {
  let annotations = annotation
    .iter()
    .map(|pair| GrpcEntityTimeAnnotation {
      entity_id: *pair.0 as i64,
      interval: Some(GrpcTimeSpan {
        nanoseconds: *pair.1 as u64,
      }),
    })
    .collect();

  GrpcTimePerformanceAnnotation { annotations }
}

fn try_convert_to_grpc_dataset(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<FicusDataset>() {
    None
  } else {
    Some(GrpcContextValue {
      context_value: Some(ContextValue::Dataset(convert_to_grpc_dataset(
        value.downcast_ref::<FicusDataset>().unwrap(),
      ))),
    })
  }
}

fn convert_to_grpc_dataset(dataset: &FicusDataset) -> GrpcDataset {
  let rows = dataset
    .values()
    .iter()
    .map(|x| GrpcMatrixRow {
      values: x.iter().map(|x| *x as f32).collect(),
    })
    .collect();

  let matrix = GrpcMatrix { rows };

  GrpcDataset {
    matrix: Some(matrix),
    columns_names: dataset.columns_names().clone(),
    row_names: dataset.row_names().clone(),
  }
}

fn convert_to_labeled_grpc_dataset(dataset: &LabeledDataset) -> GrpcLabeledDataset {
  let grpc_dataset = convert_to_grpc_dataset(dataset.dataset());
  let labels = dataset.labels().iter().map(|x| *x as i32).collect();
  let labels_colors = dataset.colors().iter().map(|x| convert_to_grpc_color(x)).collect();

  GrpcLabeledDataset {
    dataset: Some(grpc_dataset),
    labels,
    labels_colors,
  }
}

fn try_convert_to_grpc_log_threads_diagram(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<LogTimelineDiagram>() {
    None
  } else {
    Some(GrpcContextValue {
      context_value: Some(ContextValue::LogTimelineDiagram(convert_to_grpc_log_threads_diagram(
        value.downcast_ref::<LogTimelineDiagram>().unwrap(),
      ))),
    })
  }
}

fn convert_to_grpc_log_threads_diagram(diagram: &LogTimelineDiagram) -> GrpcLogTimelineDiagram {
  GrpcLogTimelineDiagram {
    traces: diagram
      .traces()
      .iter()
      .map(|t| GrpcTraceTimelineDiagram {
        events_groups: t.events_groups().iter().map(|g| GrpcTimelineTraceEventsGroup {
          start_point: Some(convert_to_grpc_log_point(g.start_point())),
          end_point: Some(convert_to_grpc_log_point(g.end_point())),
        }).collect(),
        threads: t
          .threads()
          .iter()
          .map(|t| GrpcThread {
            events: t
              .events()
              .iter()
              .map(|e| GrpcThreadEvent {
                name: e.original_event().borrow().name().to_owned(),
                stamp: e.stamp(),
              })
              .collect(),
          })
          .collect(),
      })
      .collect(),
  }
}

fn convert_to_grpc_log_point(point: &LogPoint) -> GrpcLogPoint {
  GrpcLogPoint {
    trace_index: point.trace_index() as u64,
    event_index: point.event_index() as u64,
  }
}