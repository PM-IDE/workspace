use crate::ficus_proto::{
  grpc_annotation::Annotation::{CountAnnotation, FrequencyAnnotation, TimeAnnotation},
  grpc_context_value::{ContextValue, ContextValue::Annotation},
  grpc_event_attribute, grpc_graph_edge_additional_data,
  grpc_node_additional_data::Data,
  grpc_ocel_data, GrpcActivityDurationData, GrpcActivityStartEndData, GrpcAnnotation, GrpcBytes, GrpcColor, GrpcColoredRectangle,
  GrpcColorsEventLog, GrpcColorsEventLogMapping, GrpcColorsTrace, GrpcContextValue, GrpcCountAnnotation, GrpcDataset, GrpcDurationKind,
  GrpcEdgeExecutionInfo, GrpcEntityCountAnnotation, GrpcEntityFrequencyAnnotation, GrpcEntityTimeAnnotation, GrpcEvent, GrpcEventAttribute,
  GrpcEventCoordinates, GrpcEventLogInfo, GrpcEventLogTraceSubArraysContextValue, GrpcFrequenciesAnnotation, GrpcGeneralHistogramData,
  GrpcGenericEnhancementBase, GrpcGraph, GrpcGraphEdge, GrpcGraphEdgeAdditionalData, GrpcGraphKind, GrpcGraphNode, GrpcGuid,
  GrpcHashesEventLog, GrpcHashesEventLogContextValue, GrpcHashesLogTrace, GrpcHistogramEntry, GrpcLabeledDataset, GrpcLogPoint,
  GrpcLogTimelineDiagram, GrpcMatrix, GrpcMatrixRow, GrpcModelElementOcelAnnotation, GrpcMultithreadedFragment, GrpcNamesEventLog,
  GrpcNamesEventLogContextValue, GrpcNamesTrace, GrpcNodeAdditionalData, GrpcNodeCorrespondingTraceData, GrpcOcelAllocateMerge,
  GrpcOcelConsumeProduce, GrpcOcelData, GrpcOcelModelAnnotation, GrpcOcelObjectTypeData, GrpcOcelObjectTypeState, GrpcOcelProducedObject,
  GrpcOcelState, GrpcOcelStateObjectRelation, GrpcPetriNet, GrpcPetriNetArc, GrpcPetriNetMarking, GrpcPetriNetPlace,
  GrpcPetriNetSinglePlaceMarking, GrpcPetriNetTransition, GrpcSimpleCounterData, GrpcSimpleEventLog, GrpcSimpleTrace, GrpcSoftwareData,
  GrpcSubArrayWithTraceIndex, GrpcSubArraysWithTraceIndexContextValue, GrpcThread, GrpcThreadEvent, GrpcTimePerformanceAnnotation,
  GrpcTimeSpan, GrpcTimelineDiagramFragment, GrpcTimelineTraceEventsGroup, GrpcTraceSubArray, GrpcTraceSubArrays, GrpcTraceTimelineDiagram,
  GrpcUnderlyingPatternInfo, GrpcUnderlyingPatternKind,
};
use chrono::{DateTime, Utc};
use ficus::{
  event_log::{
    core::{
      event::event::{Event, EventPayloadValue},
      event_log::EventLog,
      trace::trace::Trace,
    },
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
  },
  features::{
    analysis::{
      log_info::event_log_info::{EventLogInfo, OfflineEventLogInfo},
      patterns::{
        activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind, AdjustingMode},
        contexts::PatternsDiscoveryStrategy,
        pattern_info::{UnderlyingPatternGraphInfo, UnderlyingPatternKind},
        repeat_sets::SubArrayWithTraceIndex,
        tandem_arrays::SubArrayInTraceInfo,
      },
    },
    clustering::{activities::activities_params::ActivityRepresentationSource, traces::traces_params::TracesRepresentationSource},
    discovery::{
      ecfg::{
        context_keys::{
          EDGE_SOFTWARE_DATA_KEY, EDGE_START_END_ACTIVITIES_TIMES_KEY, EDGE_TRACE_EXECUTION_INFO_KEY, NODE_CORRESPONDING_TRACE_DATA_KEY,
          NODE_INNER_GRAPH_KEY, NODE_MULTITHREADED_FRAGMENT_LOG_KEY, NODE_SOFTWARE_DATA_KEY, NODE_START_END_ACTIVITIES_TIMES_KEY,
          NODE_START_END_ACTIVITY_TIME_KEY, NODE_UNDERLYING_PATTERNS_GRAPHS_INFO_KEY,
        },
        models::{
          ActivityStartEndTimeData, CorrespondingTraceData, EdgeTraceExecutionInfo, EventCoordinates, NodeAdditionalDataContainer,
          RootSequenceKind,
        },
      },
      ocel::graph_annotation::{NodeObjectsState, OcelAnnotation, OcelObjectRelations},
      petri_net::{
        annotations::TimeAnnotationKind,
        arc::PetriNetArc,
        marking::{Marking, SingleMarking},
        petri_net::DefaultPetriNet,
        place::Place,
        transition::Transition,
      },
      timeline::{
        discovery::{LogPoint, LogTimelineDiagram, TraceThread},
        software_data::models::{
          ActivityDurationData, DurationKind, GenericEnhancementBase, HistogramData, OcelData, OcelObjectAction, SimpleCounterData,
          SoftwareData,
        },
      },
    },
  },
  pipelines::{
    activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto},
    keys::context_keys::{
      BYTES_KEY, COLORS_EVENT_LOG_KEY, EVENT_LOG_INFO_KEY, EVENT_LOG_KEY, GRAPH_KEY, GRAPH_TIME_ANNOTATION_KEY, HASHES_EVENT_LOG_KEY,
      LABELED_LOG_TRACES_DATASET_KEY, LABELED_TRACES_ACTIVITIES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, LOG_TRACES_DATASET_KEY,
      NAMES_EVENT_LOG_KEY, OCEL_ANNOTATION_KEY, PATH_KEY, PATTERNS_KEY, PETRI_NET_COUNT_ANNOTATION_KEY, PETRI_NET_FREQUENCY_ANNOTATION_KEY,
      PETRI_NET_KEY, PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY, REPEAT_SETS_KEY, SOFTWARE_DATA_EXTRACTION_CONFIG_KEY,
      TRACES_ACTIVITIES_DATASET_KEY,
    },
    multithreading::FeatureCountKindDto,
    patterns_parts::PatternsKindDto,
    pipelines::Pipeline,
  },
  utils::{
    colors::{Color, ColoredRectangle, ColorsEventLog},
    context_key::ContextKey,
    dataset::dataset::{FicusDataset, LabeledDataset},
    distance::distance::FicusDistance,
    graph::{
      graph::{DefaultGraph, Graph, GraphKind},
      graph_edge::GraphEdge,
      graph_node::GraphNode,
    },
    log_serialization_format::LogSerializationFormat,
    user_data::{
      keys::Key,
      user_data::{UserData, UserDataImpl},
    },
  },
};
use log::error;
use nameof::name_of_type;
use prost::{DecodeError, Message};
use prost_types::Timestamp;
use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc, str::FromStr};
use std::sync::Arc;
use uuid::Uuid;

use super::pipeline_executor::ServicePipelineExecutionContext;

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
    ContextValue::String(string) => user_data.put_any::<Arc<str>>(key, string.clone().into()),
    ContextValue::HashesLog(_) => todo!(),
    ContextValue::NamesLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
    ContextValue::Uint32(number) => user_data.put_any::<u32>(key, *number),
    ContextValue::TracesSubArrays(_) => todo!(),
    ContextValue::TraceIndexSubArrays(_) => todo!(),
    ContextValue::Bool(bool) => user_data.put_any::<bool>(key, *bool),
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
    ContextValue::Strings(strings) => user_data.put_any::<Vec<Arc<str>>>(key, strings.strings.iter().map(|s| s.clone().into()).collect()),
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
        user_data.put_concrete(
          SOFTWARE_DATA_EXTRACTION_CONFIG_KEY.key(),
          match serde_json::from_str(json_string) {
            Ok(config) => config,
            Err(err) => {
              error!("Failed to deserialize, error: {}, string: {}", err, json_string);
              return;
            }
          },
        )
      }
    }
    ContextValue::EventLog(log) => {
      let mut xes_log = XesEventLogImpl::default();

      for trace in &log.traces {
        let mut xes_trace = XesTraceImpl::default();
        for event in &trace.events {
          let date: DateTime<Utc> = match event.stamp.as_ref() {
            None => DateTime::<Utc>::MIN_UTC,
            Some(stamp) => convert_timestamp_to_datetime(stamp),
          };

          let mut xes_event = XesEventImpl::new(event.name.clone().into(), date);
          for attribute in &event.attributes {
            let payload_value = attribute
              .value
              .as_ref()
              .map(convert_grpc_event_attribute_to_xes_event_payload_value);

            if let Some(xes_attribute) = payload_value {
              xes_event.add_or_update_payload(attribute.key.clone().into(), xes_attribute)
            }
          }

          xes_trace.push(Rc::new(RefCell::new(xes_event)));
        }

        xes_log.push(Rc::new(RefCell::new(xes_trace)));
      }

      user_data.put_concrete(EVENT_LOG_KEY.key(), xes_log);
    }
    ContextValue::OcelAnnotation(_) => todo!(),
  }
}

fn convert_grpc_event_attribute_to_xes_event_payload_value(attribute_value: &grpc_event_attribute::Value) -> EventPayloadValue {
  match attribute_value {
    grpc_event_attribute::Value::Int(v) => EventPayloadValue::Int64(*v),
    grpc_event_attribute::Value::String(v) => EventPayloadValue::String(v.to_owned().into()),
    grpc_event_attribute::Value::Bool(v) => EventPayloadValue::Boolean(*v),
    grpc_event_attribute::Value::Double(v) => EventPayloadValue::Float64(*v),
    grpc_event_attribute::Value::Guid(v) => EventPayloadValue::Guid(Uuid::parse_str(v.guid.as_str()).unwrap()),
    grpc_event_attribute::Value::Null(..) => EventPayloadValue::Null,
    grpc_event_attribute::Value::Stamp(v) => EventPayloadValue::Date(convert_timestamp_to_datetime(v)),
    grpc_event_attribute::Value::Uint(v) => EventPayloadValue::Uint64(*v),
  }
}

fn convert_timestamp_to_datetime(stamp: &Timestamp) -> DateTime<Utc> {
  DateTime::from_timestamp(stamp.seconds, stamp.nanos as u32).unwrap()
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
      trace.push(grpc_event.clone().into());
    }

    names_log.push(trace);
  }

  user_data.put_any::<Vec<Vec<Arc<str>>>>(key, names_log);
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
  } else if EVENT_LOG_KEY.eq_other(key) {
    try_convert_to_grpc_simple_log(value)
  } else if OCEL_ANNOTATION_KEY.eq_other(key) {
    try_convert_to_grpc_ocel_annotation(value)
  } else {
    None
  }
}

fn try_convert_to_grpc_ocel_annotation(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<OcelAnnotation>() {
    None
  } else {
    let value = value.downcast_ref::<OcelAnnotation>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(ContextValue::OcelAnnotation(convert_to_grpc_ocel_annotation(value))),
    })
  }
}

fn convert_to_grpc_ocel_annotation(annotation: &OcelAnnotation) -> GrpcOcelModelAnnotation {
  GrpcOcelModelAnnotation {
    annotations: annotation
      .nodes_to_states()
      .iter()
      .map(|s| GrpcModelElementOcelAnnotation {
        element_id: *s.0,
        final_state: Some(convert_to_grpc_ocel_node_state(s.1.final_objects())),
        initial_state: s.1.initial_objects().as_ref().map(convert_to_grpc_ocel_node_state),
        relations: s
          .1
          .incoming_objects_relations()
          .iter()
          .map(convert_to_grpc_ocel_object_relation)
          .collect(),
      })
      .collect(),
  }
}

fn convert_to_grpc_ocel_object_relation(relations: &OcelObjectRelations) -> GrpcOcelStateObjectRelation {
  GrpcOcelStateObjectRelation {
    element_id: relations.from_element_id().to_owned(),
    related_objects_ids: relations.related_objects_ids().iter().map(|id| id.to_string()).collect(),
    object_id: relations.object_id().to_string(),
  }
}

fn convert_to_grpc_ocel_node_state(state: &NodeObjectsState) -> GrpcOcelState {
  GrpcOcelState {
    type_states: state
      .map()
      .iter()
      .map(|t| GrpcOcelObjectTypeState {
        r#type: t.0.to_string(),
        object_ids: t.1.iter().map(|s| s.to_string()).collect(),
      })
      .collect(),
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
  if !value.is::<Arc<str>>() {
    None
  } else {
    Some(GrpcContextValue {
      context_value: Some(ContextValue::String(value.downcast_ref::<Arc<str>>().unwrap().to_string())),
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
  if !value.is::<Vec<Vec<Arc<str>>>>() {
    None
  } else {
    let vec = value.downcast_ref::<Vec<Vec<Arc<str>>>>().unwrap();
    let mut traces = vec![];
    for trace in vec {
      let mut events = vec![];
      for event in trace {
        events.push(event.to_string());
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
        name: key.to_string(),
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
    log_info.counts()?;

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

fn convert_to_grpc_arc<TArcData>(arc: &PetriNetArc<TArcData>) -> GrpcPetriNetArc {
  GrpcPetriNetArc {
    id: arc.id() as i64,
    place_id: arc.place_id() as i64,
    tokens_count: arc.tokens_count() as i64,
  }
}

fn try_convert_to_grpc_marking(marking: Option<&Marking>) -> Option<GrpcPetriNetMarking> {
  marking.map(|marking| GrpcPetriNetMarking {
    markings: marking.active_places().iter().map(convert_to_grpc_single_marking).collect(),
  })
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

  GrpcGraph {
    edges,
    nodes,
    kind: kind.into(),
  }
}

fn convert_to_grpc_graph_kind(kind: Option<&GraphKind>) -> GrpcGraphKind {
  match kind {
    None => GrpcGraphKind::None,
    Some(kind) => match kind {
      GraphKind::Dag => GrpcGraphKind::Dag,
      GraphKind::DagLCS => GrpcGraphKind::DagLcs,
    },
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
    inner_graph: node.user_data.concrete(NODE_INNER_GRAPH_KEY.key()).map(convert_to_grpc_graph),
  }
}

fn convert_to_grpc_graph_node_additional_data(user_data: &UserDataImpl) -> Vec<GrpcNodeAdditionalData> {
  let mut additional_data = vec![];
  if let Some(software_data) = user_data.concrete(NODE_SOFTWARE_DATA_KEY.key()) {
    additional_data.extend(software_data.iter().map(convert_to_grpc_graph_node_software_data));
  }

  if let Some(trace_data) = user_data.concrete(NODE_CORRESPONDING_TRACE_DATA_KEY.key()) {
    additional_data.extend(trace_data.iter().map(convert_to_grpc_corresponding_trace_data));
  }

  if let Some(activity_start_end_data) = user_data.concrete(NODE_START_END_ACTIVITY_TIME_KEY.key()) {
    additional_data.push(convert_to_grpc_node_activity_start_end_data(activity_start_end_data))
  }

  if let Some(activities_start_end_data) = user_data.concrete(NODE_START_END_ACTIVITIES_TIMES_KEY.key()) {
    additional_data.extend(activities_start_end_data.iter().map(convert_to_grpc_node_activity_start_end_data))
  }

  if let Some(underlying_patterns_infos) = user_data.concrete(NODE_UNDERLYING_PATTERNS_GRAPHS_INFO_KEY.key()) {
    additional_data.extend(
      underlying_patterns_infos
        .iter()
        .map(convert_to_grpc_underlying_pattern_info_additional_data),
    )
  }

  if let Some(multithreaded_logs) = user_data.concrete(NODE_MULTITHREADED_FRAGMENT_LOG_KEY.key()) {
    additional_data.extend(
      multithreaded_logs
        .iter()
        .map(convert_to_grpc_node_multithreaded_log_additional_data),
    )
  }

  additional_data
}

fn convert_to_grpc_node_multithreaded_log_additional_data(info: &NodeAdditionalDataContainer<XesEventLogImpl>) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(info.original_event_coordinates())),
    data: Some(Data::MultithreadedFragment(GrpcMultithreadedFragment {
      multithreaded_log: Some(convert_to_grpc_simple_log(info.value())),
    })),
  }
}

fn convert_to_grpc_underlying_pattern_info_additional_data(
  info: &NodeAdditionalDataContainer<UnderlyingPatternGraphInfo>,
) -> GrpcNodeAdditionalData {
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
    })
    .into(),
    base_sequence: match info.base_pattern() {
      None => vec![],
      Some(base_pattern) => base_pattern.clone(),
    },
    graph: Some(convert_to_grpc_graph(info.graph())),
  }
}

fn try_convert_to_grpc_simple_log(value: &dyn Any) -> Option<GrpcContextValue> {
  if !value.is::<XesEventLogImpl>() {
    None
  } else {
    let log = value.downcast_ref::<XesEventLogImpl>().unwrap();
    Some(GrpcContextValue {
      context_value: Some(ContextValue::EventLog(convert_to_grpc_simple_log(log))),
    })
  }
}

fn convert_to_grpc_simple_log(log: &XesEventLogImpl) -> GrpcSimpleEventLog {
  GrpcSimpleEventLog {
    traces: log
      .traces()
      .iter()
      .map(|t| convert_to_grpc_simple_trace(t.borrow().events()))
      .collect(),
  }
}

fn convert_to_grpc_simple_trace(trace: &Vec<Rc<RefCell<XesEventImpl>>>) -> GrpcSimpleTrace {
  GrpcSimpleTrace {
    events: trace
      .iter()
      .map(|e| GrpcEvent {
        name: e.borrow().name().to_owned(),
        stamp: Some(convert_to_grpc_timestamp(e.borrow().timestamp())),
        attributes: if let Some(payload) = e.borrow().payload_map() {
          payload
            .iter()
            .map(|(k, v)| GrpcEventAttribute {
              key: k.to_string(),
              value: convert_to_grpc_attribute_value(v),
            })
            .collect()
        } else {
          vec![]
        },
      })
      .collect(),
  }
}

fn convert_to_grpc_attribute_value(value: &EventPayloadValue) -> Option<grpc_event_attribute::Value> {
  match value {
    EventPayloadValue::Null => Some(grpc_event_attribute::Value::Null(())),
    EventPayloadValue::Date(date) => Some(grpc_event_attribute::Value::Stamp(convert_to_grpc_timestamp(date))),
    EventPayloadValue::String(string) => Some(grpc_event_attribute::Value::String(string.as_ref().to_owned())),
    EventPayloadValue::Boolean(bool) => Some(grpc_event_attribute::Value::Bool(bool.to_owned())),
    EventPayloadValue::Int32(int) => Some(grpc_event_attribute::Value::Int(*int as i64)),
    EventPayloadValue::Int64(int) => Some(grpc_event_attribute::Value::Int(*int)),
    EventPayloadValue::Float32(float) => Some(grpc_event_attribute::Value::Double(*float as f64)),
    EventPayloadValue::Float64(float) => Some(grpc_event_attribute::Value::Double(*float)),
    EventPayloadValue::Uint32(uint) => Some(grpc_event_attribute::Value::Uint(*uint as u64)),
    EventPayloadValue::Uint64(uint) => Some(grpc_event_attribute::Value::Uint(*uint)),
    EventPayloadValue::Guid(guid) => Some(grpc_event_attribute::Value::Guid(GrpcGuid { guid: guid.to_string() })),
    EventPayloadValue::Timestamp(_) => None,
    EventPayloadValue::Lifecycle(_) => None,
    EventPayloadValue::Artifact(_) => None,
    EventPayloadValue::Drivers(_) => None,
    EventPayloadValue::SoftwareEvent(_) => None,
  }
}

fn convert_to_grpc_timestamp(stamp: &DateTime<Utc>) -> Timestamp {
  Timestamp::from_str(stamp.to_rfc3339().as_str()).unwrap()
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
    start_time: *data.start_time(),
    end_time: *data.end_time(),
  }
}

fn convert_to_grpc_corresponding_trace_data(
  corresponding_trace_data: &NodeAdditionalDataContainer<CorrespondingTraceData>,
) -> GrpcNodeAdditionalData {
  GrpcNodeAdditionalData {
    original_event_coordinates: Some(convert_to_event_coordinates(corresponding_trace_data.original_event_coordinates())),
    data: Some(Data::TraceData(GrpcNodeCorrespondingTraceData {
      belongs_to_root_sequence: corresponding_trace_data.value().belongs_to_root_sequence(),
    })),
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
    histogram: convert_to_grpc_histogram_entries(software_data.event_classes()),
    histogram_data: software_data.histograms().iter().map(convert_to_grpc_histogram_data).collect(),

    simple_counter_data: software_data
      .simple_counters()
      .iter()
      .map(convert_to_grpc_simple_counter_data)
      .collect(),

    activities_durations_data: software_data
      .activities_durations()
      .iter()
      .map(convert_to_grpc_activity_duration)
      .collect(),

    ocel_data: software_data.ocel_data().iter().map(convert_to_grpc_ocel_data).collect(),

    timeline_diagram_fragment: Some(GrpcTimelineDiagramFragment {
      threads: convert_to_grpc_threads(software_data.thread_diagram_fragment()),
    }),
  }
}

fn convert_to_grpc_ocel_data(data: &OcelData) -> GrpcOcelData {
  GrpcOcelData {
    object_id: data.object_id().to_string(),
    action: Some(match data.action() {
      OcelObjectAction::Allocate(data) => grpc_ocel_data::Action::Allocate(GrpcOcelObjectTypeData {
        r#type: data.r#type().as_ref().map(|t| t.to_string()),
      }),
      OcelObjectAction::Consume(data) => grpc_ocel_data::Action::Consume(GrpcOcelObjectTypeData {
        r#type: data.r#type().as_ref().map(|t| t.to_string()),
      }),
      OcelObjectAction::AllocateMerged(data) => grpc_ocel_data::Action::MergedObjectAllocation(GrpcOcelAllocateMerge {
        r#type: data.r#type().as_ref().map(|t| t.to_string()),
        merged_objects_ids: data.data().iter().map(|id| id.to_string()).collect(),
      }),
      OcelObjectAction::ConsumeWithProduce(data) => grpc_ocel_data::Action::ProduceObjectConsumption(GrpcOcelConsumeProduce {
        produced_objects: data
          .iter()
          .map(|x| GrpcOcelProducedObject {
            id: x.id().to_string(),
            r#type: x.r#type().as_ref().map(|t| t.to_string()),
          })
          .collect(),
      }),
    }),
  }
}

fn convert_to_grpc_activity_duration(data: &ActivityDurationData) -> GrpcActivityDurationData {
  GrpcActivityDurationData {
    base: Some(convert_to_grpc_generic_enhancement_base(data.base())),
    duration: *data.duration(),
    kind: (match data.kind() {
      DurationKind::Unknown => GrpcDurationKind::Unspecified,
      DurationKind::Nanos => GrpcDurationKind::Nanos,
      DurationKind::Micros => GrpcDurationKind::Micros,
      DurationKind::Millis => GrpcDurationKind::Millis,
      DurationKind::Seconds => GrpcDurationKind::Seconds,
      DurationKind::Minutes => GrpcDurationKind::Minutes,
      DurationKind::Hours => GrpcDurationKind::Hours,
      DurationKind::Days => GrpcDurationKind::Days,
    }) as i32,
  }
}

fn convert_to_grpc_generic_enhancement_base(base: &GenericEnhancementBase) -> GrpcGenericEnhancementBase {
  GrpcGenericEnhancementBase {
    name: base.name().to_string(),
    units: base.units().to_string(),
    group: base.group().as_ref().map(|g| g.to_string()),
  }
}

fn convert_to_grpc_histogram_data(data: &HistogramData) -> GrpcGeneralHistogramData {
  GrpcGeneralHistogramData {
    base: Some(convert_to_grpc_generic_enhancement_base(data.base())),
    entries: data
      .entries()
      .iter()
      .map(|e| GrpcHistogramEntry {
        name: e.name().to_string(),
        count: *e.value(),
      })
      .collect(),
  }
}

fn convert_to_grpc_simple_counter_data(data: &SimpleCounterData) -> GrpcSimpleCounterData {
  GrpcSimpleCounterData {
    base: Some(convert_to_grpc_generic_enhancement_base(data.base())),
    count: *data.value(),
  }
}

fn convert_to_grpc_histogram_entries(histogram: &HashMap<Arc<str>, usize>) -> Vec<GrpcHistogramEntry> {
  histogram
    .iter()
    .map(|(key, value)| GrpcHistogramEntry {
      name: key.to_string(),
      count: *value as f64,
    })
    .collect()
}

fn convert_to_grpc_graph_edge<TEdgeData>(edge: &GraphEdge<TEdgeData>) -> GrpcGraphEdge
where
  TEdgeData: ToString,
{
  GrpcGraphEdge {
    id: *edge.id(),
    from_node: *edge.from_node(),
    to_node: *edge.to_node(),
    weight: *edge.weight(),
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
      trace_id: *info.trace_id(),
    })),
  }
}

fn convert_to_grpc_edge_software_additional_data(software_data: &SoftwareData) -> GrpcGraphEdgeAdditionalData {
  GrpcGraphEdgeAdditionalData {
    data: Some(grpc_graph_edge_additional_data::Data::SoftwareData(convert_to_grpc_software_data(
      software_data,
    ))),
  }
}

fn convert_grpc_edge_activity_start_end_time(activity_data: &ActivityStartEndTimeData) -> GrpcGraphEdgeAdditionalData {
  GrpcGraphEdgeAdditionalData {
    data: Some(grpc_graph_edge_additional_data::Data::TimeData(
      convert_to_grpc_activity_start_end_data(activity_data),
    )),
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
  let labels_colors = dataset.colors().iter().map(convert_to_grpc_color).collect();

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
        events_groups: t
          .events_groups()
          .iter()
          .map(|g| GrpcTimelineTraceEventsGroup {
            start_point: Some(convert_to_grpc_log_point(g.start_point())),
            end_point: Some(convert_to_grpc_log_point(g.end_point())),
          })
          .collect(),
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
          stamp: *e.stamp(),
        })
        .collect(),
    })
    .collect()
}

fn convert_to_grpc_log_point(point: &LogPoint) -> GrpcLogPoint {
  GrpcLogPoint {
    trace_index: *point.trace_index() as u64,
    event_index: *point.event_index() as u64,
  }
}
