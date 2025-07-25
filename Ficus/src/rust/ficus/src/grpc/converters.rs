use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::analysis::log_info::event_log_info::{EventLogInfo, OfflineEventLogInfo};
use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::analysis::patterns::pattern_info::{UnderlyingPatternGraphInfo, UnderlyingPatternKind};
use crate::features::clustering::activities::activities_params::ActivityRepresentationSource;
use crate::features::clustering::traces::traces_params::TracesRepresentationSource;
use crate::features::discovery::petri_net::annotations::TimeAnnotationKind;
use crate::features::discovery::petri_net::arc::Arc;
use crate::features::discovery::petri_net::marking::{Marking, SingleMarking};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::features::discovery::root_sequence::context_keys::{EDGE_SOFTWARE_DATA_KEY, EDGE_START_END_ACTIVITIES_TIMES_KEY, EDGE_TRACE_EXECUTION_INFO_KEY, NODE_CORRESPONDING_TRACE_DATA_KEY, NODE_INNER_GRAPH_KEY, NODE_MULTITHREADED_FRAGMENT_LOG_KEY, NODE_SOFTWARE_DATA_KEY, NODE_START_END_ACTIVITIES_TIMES_KEY, NODE_START_END_ACTIVITY_TIME_KEY, NODE_UNDERLYING_PATTERNS_GRAPHS_INFOS_KEY};
use crate::features::discovery::root_sequence::models::{ActivityStartEndTimeData, CorrespondingTraceData, EdgeTraceExecutionInfo, EventCoordinates, NodeAdditionalDataContainer, RootSequenceKind};
use crate::features::discovery::timeline::discovery::{LogPoint, LogTimelineDiagram, TraceThread};
use crate::features::discovery::timeline::software_data::models::{AllocationEvent, ArrayPoolEvent, ArrayPoolEventKind, ContentionEvent, ExceptionEvent, ExecutionSuspensionEvent, HTTPEvent, HistogramData, MethodInliningData, MethodInliningEvent, MethodLoadUnloadEvent, MethodNameParts, SimpleCounterData, SocketEvent, SoftwareData, ThreadEvent, ThreadEventKind};
use crate::ficus_proto::grpc_annotation::Annotation::{CountAnnotation, FrequencyAnnotation, TimeAnnotation};
use crate::ficus_proto::grpc_context_value::ContextValue::Annotation;
use crate::ficus_proto::grpc_event_stamp::Stamp;
use crate::ficus_proto::grpc_node_additional_data::Data;
use crate::ficus_proto::{grpc_array_pool_event, grpc_graph_edge_additional_data, grpc_method_inlining_event, grpc_method_load_unload_event, grpc_socket_event, grpc_thread_event_info, GrpcActivityStartEndData, GrpcAllocationInfo, GrpcAnnotation, GrpcArrayPoolEvent, GrpcBytes, GrpcColorsEventLogMapping, GrpcContentionEvent, GrpcCountAnnotation, GrpcDataset, GrpcEdgeExecutionInfo, GrpcEntityCountAnnotation, GrpcEntityFrequencyAnnotation, GrpcEntityTimeAnnotation, GrpcEvent, GrpcEventCoordinates, GrpcEventStamp, GrpcExceptionEvent, GrpcExecutionSuspensionInfo, GrpcFrequenciesAnnotation, GrpcGeneralHistogramData, GrpcGraph, GrpcGraphEdge, GrpcGraphEdgeAdditionalData, GrpcGraphKind, GrpcGraphNode, GrpcHistogramEntry, GrpcHttpEvent, GrpcLabeledDataset, GrpcLogPoint, GrpcLogTimelineDiagram, GrpcMatrix, GrpcMatrixRow, GrpcMethodInliningEvent, GrpcMethodInliningFailedEvent, GrpcMethodInliningInfo, GrpcMethodLoadUnloadEvent, GrpcMethodNameParts, GrpcMultithreadedFragment, GrpcNodeAdditionalData, GrpcNodeCorrespondingTraceData, GrpcPetriNet, GrpcPetriNetArc, GrpcPetriNetMarking, GrpcPetriNetPlace, GrpcPetriNetSinglePlaceMarking, GrpcPetriNetTransition, GrpcSimpleCounterData, GrpcSimpleEventLog, GrpcSimpleTrace, GrpcSocketAcceptFailed, GrpcSocketAcceptStart, GrpcSocketAcceptStop, GrpcSocketConnectFailed, GrpcSocketConnectStart, GrpcSocketConnectStop, GrpcSocketEvent, GrpcSoftwareData, GrpcThread, GrpcThreadEvent, GrpcThreadEventInfo, GrpcThreadEventKind, GrpcTimePerformanceAnnotation, GrpcTimeSpan, GrpcTimelineDiagramFragment, GrpcTimelineTraceEventsGroup, GrpcTraceTimelineDiagram, GrpcUnderlyingPatternInfo, GrpcUnderlyingPatternKind};
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::keys::context_keys::{BYTES_KEY, COLORS_EVENT_LOG_KEY, EVENT_LOG_INFO_KEY, GRAPH_KEY, GRAPH_TIME_ANNOTATION_KEY, HASHES_EVENT_LOG_KEY, LABELED_LOG_TRACES_DATASET_KEY, LABELED_TRACES_ACTIVITIES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, LOG_TRACES_DATASET_KEY, NAMES_EVENT_LOG_KEY, PATH_KEY, PATTERNS_KEY, PETRI_NET_COUNT_ANNOTATION_KEY, PETRI_NET_FREQUENCY_ANNOTATION_KEY, PETRI_NET_KEY, PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY, REPEAT_SETS_KEY, SOFTWARE_DATA_EXTRACTION_CONFIG_KEY, TRACES_ACTIVITIES_DATASET_KEY};
use crate::pipelines::multithreading::FeatureCountKindDto;
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::utils::colors::ColorsEventLog;
use crate::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use crate::utils::distance::distance::FicusDistance;
use crate::utils::graph::graph::{DefaultGraph, Graph, GraphKind};
use crate::utils::graph::graph_edge::GraphEdge;
use crate::utils::graph::graph_node::GraphNode;
use crate::utils::log_serialization_format::LogSerializationFormat;
use crate::utils::user_data::user_data::UserDataImpl;
use crate::{features::analysis::patterns::{
  activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy, repeat_sets::SubArrayWithTraceIndex,
  tandem_arrays::SubArrayInTraceInfo,
}, ficus_proto::{
  grpc_context_value::ContextValue, GrpcColor, GrpcColoredRectangle, GrpcColorsEventLog, GrpcColorsTrace, GrpcContextValue,
  GrpcEventLogInfo, GrpcEventLogTraceSubArraysContextValue, GrpcHashesEventLog, GrpcHashesEventLogContextValue, GrpcHashesLogTrace,
  GrpcNamesEventLog, GrpcNamesEventLogContextValue, GrpcNamesTrace, GrpcSubArrayWithTraceIndex,
  GrpcSubArraysWithTraceIndexContextValue, GrpcTraceSubArray, GrpcTraceSubArrays,
}, pipelines::pipelines::Pipeline, utils::{
  colors::{Color, ColoredRectangle},
  user_data::{keys::Key, user_data::UserData},
}, vecs};
use log::error;
use nameof::name_of_type;
use prost::{DecodeError, Message};
use prost_types::Timestamp;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::{any::Any, str::FromStr};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::utils::context_key::ContextKey;

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
      } else if enum_name == name_of_type!(FeatureCountKindDto) {
        parse_grpc_enum::<FeatureCountKindDto>(user_data, key, &grpc_enum.value);
      } else if enum_name == name_of_type!(RootSequenceKind) {
        parse_grpc_enum::<RootSequenceKind>(user_data, key, &grpc_enum.value);
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
    ContextValue::FloatArray(float_array) => user_data.put_any::<Vec<f64>>(key, float_array.items.clone()),
    ContextValue::IntArray(int_array) => user_data.put_any::<Vec<i64>>(key, int_array.items.clone()),
    ContextValue::UintArray(uint_array) => user_data.put_any::<Vec<u64>>(key, uint_array.items.clone()),
    ContextValue::Json(json_string) => {
      if key.id() == SOFTWARE_DATA_EXTRACTION_CONFIG_KEY.key().id() {
        user_data.put_concrete(SOFTWARE_DATA_EXTRACTION_CONFIG_KEY.key(), match serde_json::from_str(json_string) {
          Ok(config) => config,
          Err(err) => {
            error!("Failed to deserialize, error: {}, string: {}", err.to_string(), json_string);
            return;
          }
        })
      }
    }
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
  let kind = convert_to_grpc_graph_kind(graph.kind().as_ref());

  GrpcGraph { edges, nodes, kind: kind.into() }
}

fn convert_to_grpc_graph_kind(kind: Option<&GraphKind>) -> GrpcGraphKind {
  match kind {
    None => GrpcGraphKind::None,
    Some(kind) => match kind {
      GraphKind::Dag => GrpcGraphKind::Dag
    }
  }
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
    additional_data: convert_to_grpc_graph_node_additional_data(node.user_data()),
    inner_graph: if let Some(inner_graph) = node.user_data.concrete(NODE_INNER_GRAPH_KEY.key()) {
      Some(convert_to_grpc_graph(inner_graph))
    } else {
      None
    },
  }
}

fn convert_to_grpc_graph_node_additional_data(user_data: &UserDataImpl) -> Vec<GrpcNodeAdditionalData> {
  let mut additional_data = vec![];
  if let Some(software_data) = user_data.concrete(NODE_SOFTWARE_DATA_KEY.key()) {
    additional_data.extend(software_data.iter().map(|s| convert_to_grpc_graph_node_software_data(s)));
  }

  if let Some(trace_data) = user_data.concrete(NODE_CORRESPONDING_TRACE_DATA_KEY.key()) {
    additional_data.extend(trace_data.iter().map(|t| convert_to_grpc_corresponding_trace_data(t)));
  }

  if let Some(activity_start_end_data) = user_data.concrete(NODE_START_END_ACTIVITY_TIME_KEY.key()) {
    additional_data.push(convert_to_grpc_node_activity_start_end_data(activity_start_end_data))
  }

  if let Some(activities_start_end_data) = user_data.concrete(NODE_START_END_ACTIVITIES_TIMES_KEY.key()) {
    additional_data.extend(activities_start_end_data.iter().map(|d| convert_to_grpc_node_activity_start_end_data(d)))
  }

  if let Some(underlying_patterns_infos) = user_data.concrete(NODE_UNDERLYING_PATTERNS_GRAPHS_INFOS_KEY.key()) {
    additional_data.extend(underlying_patterns_infos.iter().map(|info| convert_to_grpc_underlying_pattern_info_additional_data(info)))
  }

  if let Some(multithreaded_logs) = user_data.concrete(NODE_MULTITHREADED_FRAGMENT_LOG_KEY.key()) {
    additional_data.extend(multithreaded_logs.iter().map(|info| convert_to_grpc_node_multithreaded_log_additional_data(info)))
  }

  additional_data
}

fn convert_to_grpc_node_multithreaded_log_additional_data(info: &NodeAdditionalDataContainer<XesEventLogImpl>) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(info.original_event_coordinates())),
    data: Some(Data::MultithreadedFragment(GrpcMultithreadedFragment { multithreaded_log: Some(convert_to_grpc_simple_log(info.value())) })),
  }
}

fn convert_to_grpc_underlying_pattern_info_additional_data(info: &NodeAdditionalDataContainer<UnderlyingPatternGraphInfo>) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(info.original_event_coordinates())),
    data: Some(Data::PatternInfo(convert_to_grpc_underlying_pattern_info(info.value()))),
  }
}

fn convert_to_grpc_underlying_pattern_info(info: &UnderlyingPatternGraphInfo) -> GrpcUnderlyingPatternInfo {
  GrpcUnderlyingPatternInfo {
    pattern_kind: (match info.pattern_kind() {
      UnderlyingPatternKind::StrictLoop => GrpcUnderlyingPatternKind::StrictLoop,
      UnderlyingPatternKind::PrimitiveTandemArray => GrpcUnderlyingPatternKind::PrimitiveTandemArray,
      UnderlyingPatternKind::MaximalTandemArray => GrpcUnderlyingPatternKind::MaximalTandemArray,
      UnderlyingPatternKind::MaximalRepeat => GrpcUnderlyingPatternKind::MaximalRepeat,
      UnderlyingPatternKind::SuperMaximalRepeat => GrpcUnderlyingPatternKind::SuperMaximalRepeat,
      UnderlyingPatternKind::NearSuperMaximalRepeat => GrpcUnderlyingPatternKind::NearSuperMaximalRepeat,
      UnderlyingPatternKind::Unknown => GrpcUnderlyingPatternKind::Unknown,
    }).into(),
    base_sequence: match info.base_pattern() {
      None => vec![],
      Some(base_pattern) => base_pattern.clone()
    },
    graph: Some(convert_to_grpc_graph(info.graph().as_ref())),
  }
}

fn convert_to_grpc_simple_log(log: &XesEventLogImpl) -> GrpcSimpleEventLog {
  GrpcSimpleEventLog {
    traces: log.traces().iter().map(|t| convert_to_grpc_simple_trace(t.borrow().events())).collect()
  }
}

fn convert_to_grpc_simple_trace(trace: &Vec<Rc<RefCell<XesEventImpl>>>) -> GrpcSimpleTrace {
  GrpcSimpleTrace {
    events: trace.iter().map(|e| GrpcEvent {
      name: e.borrow().name().to_owned(),
      stamp: Some(GrpcEventStamp {
        stamp: Some(Stamp::Date(Timestamp::from_str(e.borrow().timestamp().to_rfc3339().as_str()).unwrap()))
      }),
    }).collect()
  }
}

fn convert_to_event_coordinates(event_coordinates: &EventCoordinates) -> GrpcEventCoordinates {
  GrpcEventCoordinates {
    trace_id: event_coordinates.trace_id(),
    event_index: event_coordinates.event_index(),
  }
}

fn convert_to_grpc_node_activity_start_end_data(data: &NodeAdditionalDataContainer<ActivityStartEndTimeData>) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(data.original_event_coordinates())),
    data: Some(Data::TimeData(convert_to_grpc_activity_start_end_data(data.value()))),
  }
}

fn convert_to_grpc_activity_start_end_data(data: &ActivityStartEndTimeData) -> GrpcActivityStartEndData {
  GrpcActivityStartEndData {
    start_time: data.start_time(),
    end_time: data.end_time(),
  }
}

fn convert_to_grpc_corresponding_trace_data(corresponding_trace_data: &NodeAdditionalDataContainer<CorrespondingTraceData>) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(corresponding_trace_data.original_event_coordinates())),
    data: Some(Data::TraceData(
      GrpcNodeCorrespondingTraceData {
        belongs_to_root_sequence: corresponding_trace_data.value().belongs_to_root_sequence(),
      }
    )),
  }
}

fn convert_to_grpc_graph_node_software_data(software_data: &NodeAdditionalDataContainer<SoftwareData>) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(software_data.original_event_coordinates())),
    data: Some(Data::SoftwareData(convert_to_grpc_software_data(software_data.value()))),
  }
}

fn convert_to_grpc_software_data(software_data: &SoftwareData) -> GrpcSoftwareData {
  GrpcSoftwareData {
    allocations_info: convert_to_grpc_allocation(software_data.allocation_events()),
    histogram: convert_to_grpc_histogram_entries(software_data.event_classes()),
    contention_events: convert_to_grpc_contention_events(software_data.contention_events()),
    exception_events: convert_to_grpc_exception_events(software_data.exception_events()),
    execution_suspension_info: convert_to_grpc_suspensions(software_data.suspensions()),
    thread_events: convert_to_grpc_threads_events(software_data.thread_events()),
    methods_inlining_events: convert_to_grpc_methods_events(software_data.method_inlinings_events()),
    array_pool_events: convert_to_grpc_array_pool_event(software_data.pool_events()),
    http_events: convert_to_grpc_http_events(software_data.http_events()),
    socket_event: convert_to_grpc_socket_events(software_data.socket_events()),
    timeline_diagram_fragment: Some(GrpcTimelineDiagramFragment {
      threads: convert_to_grpc_threads(software_data.thread_diagram_fragment())
    }),
    methods_load_unload_events: convert_to_grpc_method_load_unload_events(software_data.method_load_unload_events()),
    histogram_data: software_data.histograms().iter().map(|h| convert_to_grpc_histogram_data(h)).collect(),
    simple_counter_data: software_data.simple_counters().iter().map(|c| convert_to_grpc_simple_counter_data(c)).collect(),
  }
}

fn convert_to_grpc_histogram_data(data: &HistogramData) -> GrpcGeneralHistogramData {
  GrpcGeneralHistogramData {
    name: data.name().to_string(),
    units: data.units().to_owned(),
    entries: data.entries().iter().map(|e| GrpcHistogramEntry {
      name: e.name().to_string(),
      count: *e.value(),
    }).collect(),
  }
}

fn convert_to_grpc_simple_counter_data(data: &SimpleCounterData) -> GrpcSimpleCounterData {
  GrpcSimpleCounterData {
    name: data.name().to_string(),
    count: *data.value(),
    units: data.units().to_owned(),
  }
}

fn convert_to_grpc_method_load_unload_events(events: &Vec<MethodLoadUnloadEvent>) -> Vec<GrpcMethodLoadUnloadEvent> {
  events.iter().map(|e| match e {
    MethodLoadUnloadEvent::Load(load) => GrpcMethodLoadUnloadEvent {
      method_name_parts: Some(convert_to_grpc_method_name_parts(load)),
      event: Some(grpc_method_load_unload_event::Event::Load(())),
    },
    MethodLoadUnloadEvent::Unload(load) => GrpcMethodLoadUnloadEvent {
      method_name_parts: Some(convert_to_grpc_method_name_parts(load)),
      event: Some(grpc_method_load_unload_event::Event::Unload(())),
    }
  }).collect()
}

fn convert_to_grpc_allocation(allocations: &Vec<AllocationEvent>) -> Vec<GrpcAllocationInfo> {
  allocations.iter().map(|a| GrpcAllocationInfo {
    type_name: a.type_name().to_owned(),
    allocated_bytes: *a.allocated_bytes() as u64,
    allocated_objects_count: *a.objects_count() as u64,
  }).collect()
}

fn convert_to_grpc_socket_events(events: &Vec<SocketEvent>) -> Vec<GrpcSocketEvent> {
  events.iter().map(|e| GrpcSocketEvent {
    event: Some(match e {
      SocketEvent::ConnectStart(e) => grpc_socket_event::Event::ConnectStart(GrpcSocketConnectStart {
        address: e.address().to_owned()
      }),
      SocketEvent::ConnectStop => grpc_socket_event::Event::ConnectStop(GrpcSocketConnectStop {}),
      SocketEvent::AcceptStart(e) => grpc_socket_event::Event::AcceptStart(GrpcSocketAcceptStart {
        address: e.address().to_owned(),
      }),
      SocketEvent::AcceptStop => grpc_socket_event::Event::AcceptStop(GrpcSocketAcceptStop {}),
      SocketEvent::ConnectFailed(e) => grpc_socket_event::Event::ConnectFailed(GrpcSocketConnectFailed {
        error_code: e.error_code().to_owned(),
        error_message: e.error_message().to_owned(),
      }),
      SocketEvent::AcceptFailed(e) => grpc_socket_event::Event::AcceptFailed(GrpcSocketAcceptFailed {
        error_code: e.error_code().to_owned(),
        error_message: e.error_message().to_owned(),
      }),
    })
  }).collect()
}

fn convert_to_grpc_http_events(events: &Vec<HTTPEvent>) -> Vec<GrpcHttpEvent> {
  events.iter().map(|h| GrpcHttpEvent {
    host: h.host().to_owned(),
    port: h.port().to_owned(),
    scheme: h.scheme().to_owned(),
    path_and_query: h.path_and_query().to_owned(),
  }).collect()
}

fn convert_to_grpc_array_pool_event(events: &Vec<ArrayPoolEvent>) -> Vec<GrpcArrayPoolEvent> {
  events.iter().map(|a| GrpcArrayPoolEvent {
    buffer_id: a.buffer_id().clone(),
    buffer_size_bytes: a.buffer_size_bytes().clone(),
    event: Some(match a.event_kind() {
      ArrayPoolEventKind::Created => grpc_array_pool_event::Event::BufferAllocated(()),
      ArrayPoolEventKind::Rented => grpc_array_pool_event::Event::BufferRented(()),
      ArrayPoolEventKind::Returned => grpc_array_pool_event::Event::BufferReturned(()),
      ArrayPoolEventKind::Trimmed => grpc_array_pool_event::Event::BufferTrimmed(()),
    }),
  }).collect()
}

fn convert_to_grpc_methods_events(events: &Vec<MethodInliningEvent>) -> Vec<GrpcMethodInliningEvent> {
  events.iter().map(|m| match m {
    MethodInliningEvent::InliningSuccess(method) => GrpcMethodInliningEvent {
      inlining_info: Some(convert_to_grpc_inlining_info(method)),
      event: Some(grpc_method_inlining_event::Event::Succeeded(())),
    },
    MethodInliningEvent::InliningFailed(method, reason) => GrpcMethodInliningEvent {
      inlining_info: Some(convert_to_grpc_inlining_info(method)),
      event: Some(grpc_method_inlining_event::Event::Failed(GrpcMethodInliningFailedEvent {
        reason: reason.clone()
      })),
    }
  }).collect()
}

fn convert_to_grpc_inlining_info(method: &MethodInliningData) -> GrpcMethodInliningInfo {
  GrpcMethodInliningInfo {
    inlinee_info: Some(convert_to_grpc_method_name_parts(method.inlinee_info())),
    inliner_info: Some(convert_to_grpc_method_name_parts(method.inliner_info())),
  }
}

fn convert_to_grpc_method_name_parts(method: &MethodNameParts) -> GrpcMethodNameParts {
  GrpcMethodNameParts {
    name: method.name().to_owned(),
    namespace: method.namespace().to_owned(),
    signature: method.signature().to_owned(),
  }
}

fn convert_to_grpc_threads_events(events: &Vec<ThreadEvent>) -> Vec<GrpcThreadEventInfo> {
  events.iter().map(|t| GrpcThreadEventInfo {
    thread_id: t.thread_id().clone(),
    event: Some(match t.kind() {
      ThreadEventKind::Created => grpc_thread_event_info::Event::Created(()),
      ThreadEventKind::Terminated => grpc_thread_event_info::Event::Terminated(()),
    }),
  }).collect()
}

fn convert_to_grpc_suspensions(events: &Vec<ExecutionSuspensionEvent>) -> Vec<GrpcExecutionSuspensionInfo> {
  events.iter().map(|e| GrpcExecutionSuspensionInfo {
    start_time: e.start_time().clone(),
    end_time: e.end_time().clone(),
    reason: e.reason().to_owned(),
  }).collect()
}

fn convert_to_grpc_exception_events(events: &Vec<ExceptionEvent>) -> Vec<GrpcExceptionEvent> {
  events.iter().map(|c| GrpcExceptionEvent {
    exception_type: c.exception_type().to_owned()
  }).collect()
}

fn convert_to_grpc_contention_events(events: &Vec<ContentionEvent>) -> Vec<GrpcContentionEvent> {
  events.iter().map(|c| GrpcContentionEvent {
    start_time: c.start_time().clone(),
    end_time: c.end_time().clone(),
  }).collect()
}

fn convert_to_grpc_histogram_entries(histogram: &HashMap<String, usize>) -> Vec<GrpcHistogramEntry> {
  histogram.iter().map(|(key, value)| {
    GrpcHistogramEntry {
      name: key.to_owned(),
      count: *value as f64,
    }
  }).collect()
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
    additional_data: convert_to_grpc_edge_additional_data(edge.user_data()),
    data: match edge.data() {
      None => "".to_string(),
      Some(data) => data.to_string(),
    },
  }
}

fn convert_to_grpc_edge_additional_data(user_data: &UserDataImpl) -> Vec<GrpcGraphEdgeAdditionalData> {
  let mut result = vec![];
  if let Some(edge_software_data) = user_data.concrete(EDGE_SOFTWARE_DATA_KEY.key()) {
    for data in edge_software_data {
      result.push(convert_to_grpc_edge_software_additional_data(data));
    }
  }

  if let Some(activities_start_end_data) = user_data.concrete(EDGE_START_END_ACTIVITIES_TIMES_KEY.key()) {
    for data in activities_start_end_data {
      result.push(convert_grpc_edge_activity_start_end_time(data));
    }
  }

  if let Some(trace_execution_infos) = user_data.concrete(EDGE_TRACE_EXECUTION_INFO_KEY.key()) {
    for info in trace_execution_infos {
      result.push(convert_to_grpc_edge_execution_info_additional_data(info));
    }
  }

  result
}

fn convert_to_grpc_edge_execution_info_additional_data(info: &EdgeTraceExecutionInfo) -> GrpcGraphEdgeAdditionalData {
  GrpcGraphEdgeAdditionalData {
    data: Some(grpc_graph_edge_additional_data::Data::ExecutionInfo(GrpcEdgeExecutionInfo {
      trace_id: info.trace_id().clone()
    }))
  }
}

fn convert_to_grpc_edge_software_additional_data(software_data: &SoftwareData) -> GrpcGraphEdgeAdditionalData {
  GrpcGraphEdgeAdditionalData {
    data: Some(grpc_graph_edge_additional_data::Data::SoftwareData(convert_to_grpc_software_data(software_data)))
  }
}

fn convert_grpc_edge_activity_start_end_time(activity_data: &ActivityStartEndTimeData) -> GrpcGraphEdgeAdditionalData {
  GrpcGraphEdgeAdditionalData {
    data: Some(grpc_graph_edge_additional_data::Data::TimeData(convert_to_grpc_activity_start_end_data(activity_data)))
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
        threads: convert_to_grpc_threads(t.threads()),
      })
      .collect(),
  }
}

fn convert_to_grpc_threads(threads: &Vec<TraceThread>) -> Vec<GrpcThread> {
  threads
    .iter()
    .map(|t| GrpcThread {
      events: t
        .events()
        .iter()
        .map(|e| GrpcThreadEvent {
          name: e.original_event().borrow().name().to_owned(),
          stamp: e.stamp().clone(),
        })
        .collect(),
    })
    .collect()
}

fn convert_to_grpc_log_point(point: &LogPoint) -> GrpcLogPoint {
  GrpcLogPoint {
    trace_index: point.trace_index().clone() as u64,
    event_index: point.event_index().clone() as u64,
  }
}