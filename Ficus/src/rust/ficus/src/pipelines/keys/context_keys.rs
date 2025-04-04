use std::ops::Deref;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::analysis::patterns::pattern_info::UnderlyingPatternInfo;
use crate::features::clustering::activities::activities_params::ActivityRepresentationSource;
use crate::features::clustering::traces::traces_params::TracesRepresentationSource;
use crate::features::discovery::petri_net::annotations::TimeAnnotationKind;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::root_sequence::discovery::RootSequenceKind;
use crate::features::discovery::root_sequence::models::{ActivityStartEndTimeData, CorrespondingTraceData, NodeAdditionalDataContainer};
use crate::features::discovery::timeline::abstraction::SoftwareData;
use crate::features::discovery::timeline::discovery::LogTimelineDiagram;
use crate::grpc::events::events_handler::CaseName;
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::keys::context_key::{ContextKey, DefaultContextKey};
use crate::pipelines::multithreading::FeatureCountKindDto;
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::utils::colors::ColorsEventLog;
use crate::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use crate::utils::distance::distance::FicusDistance;
use crate::utils::graph::graph::DefaultGraph;
use crate::utils::log_serialization_format::LogSerializationFormat;
use crate::{
  event_log::xes::xes_event_log::XesEventLogImpl,
  features::analysis::patterns::{
    activity_instances::{ActivityInTraceInfo, AdjustingMode},
    contexts::PatternsDiscoveryStrategy,
    repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
    tandem_arrays::SubArrayInTraceInfo,
  },
  pipelines::pipelines::Pipeline,
  utils::colors::ColorsHolder,
};
use bxes::models::system_models::SystemMetadata;
use lazy_static::lazy_static;
use uuid::Uuid;

pub const CASE_NAME_STR: &'static str = "case_name";
pub const PROCESS_NAME_STR: &'static str = "process_name";
pub const SUBSCRIPTION_ID_STR: &'static str = "subscription_id";
pub const SUBSCRIPTION_NAME_STR: &'static str = "subscription_name";
pub const PIPELINE_ID_STR: &'static str = "pipeline_id";
pub const PIPELINE_NAME_STR: &'static str = "pipeline_name";
pub const UNSTRUCTURED_METADATA_STR: &'static str = "unstructured_metadata";
pub const PATH: &'static str = "path";
pub const TANDEM_ARRAY_LENGTH: &'static str = "tandem_array_length";
pub const ACTIVITY_LEVEL: &'static str = "activity_level";
pub const NARROW_ACTIVITIES: &'static str = "narrow_activities";
pub const EVENT_NAME: &'static str = "event_name";
pub const REGEX: &'static str = "regex";
pub const PATTERNS_DISCOVERY_STRATEGY: &'static str = "patterns_discovery_strategy";
pub const OUTPUT_STRING: &'static str = "output_string";
pub const EVENT_LOG_INFO: &'static str = "event_log_info";
pub const UNDERLYING_EVENTS_COUNT: &'static str = "underlying_events_count";
pub const EVENTS_COUNT: &'static str = "events_count";
pub const EVENT_CLASSES_REGEXES: &'static str = "event_classes_regexes";
pub const ADJUSTING_MODE: &'static str = "adjusting_mode";
pub const EVENT_CLASS_REGEX: &'static str = "event_class_regex";
pub const PATTERNS_KIND: &'static str = "patterns_kind";
pub const PIPELINE: &'static str = "pipeline";
pub const MIN_ACTIVITY_LENGTH: &'static str = "min_activity_length";
pub const UNDEF_ACTIVITY_HANDLING_STRATEGY: &'static str = "undef_activity_handling_strategy";
pub const ACTIVITY_IN_TRACE_FILTER_KIND: &'static str = "activity_in_trace_filter_kind";
pub const ACTIVITIES_LOGS_SOURCE: &'static str = "activities_logs_source";
pub const PNML_USE_NAMES_AS_IDS: &'static str = "pnml_use_names_as_ids";
pub const DEPENDENCY_THRESHOLD: &'static str = "dependency_threshold";
pub const POSITIVE_OBSERVATIONS_THRESHOLD: &'static str = "positive_observations_threshold";
pub const RELATIVE_TO_BEST_THRESHOLD: &'static str = "relative_to_best_threshold";
pub const AND_THRESHOLD: &'static str = "and_threshold";
pub const LOOP_LENGTH_TWO_THRESHOLD: &'static str = "loop_length_two_threshold";
pub const UNARY_FREQUENCY_THRESHOLD: &'static str = "unary_frequency_threshold";
pub const BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD: &'static str = "binary_frequency_significance_threshold";
pub const PRESERVE_THRESHOLD: &'static str = "preserve_threshold";
pub const RATIO_THRESHOLD: &'static str = "ratio_threshold";
pub const UTILITY_RATE: &'static str = "utility_rate";
pub const EDGE_CUTOFF_THRESHOLD: &'static str = "edge_cutoff_threshold";
pub const NODE_CUTOFF_THRESHOLD: &'static str = "node_cutoff_threshold";
pub const TERMINATE_ON_UNREPLAYABLE_TRACES: &'static str = "terminate_on_unreplayable_traces";
pub const CLUSTERS_COUNT: &'static str = "clusters_count";
pub const LEARNING_ITERATIONS_COUNT: &'static str = "learning_iterations_count";
pub const TOLERANCE: &'static str = "tolerance";
pub const MIN_EVENTS_IN_CLUSTERS_COUNT: &'static str = "min_events_in_cluster_count";
pub const EVENT_LOG_NAME: &'static str = "event_log_name";
pub const BYTES: &'static str = "bytes";
pub const START_CASE_REGEX_STR: &str = "start_case_regex";
pub const END_CASE_REGEX_STR: &str = "end_case_regex";
pub const INLINE_INNER_CASES_STR: &str = "inline_inner_cases";

pub const EVENT_LOG: &'static str = "event_log";
pub const ACTIVITIES: &'static str = "activities";
pub const REPEAT_SETS: &'static str = "repeat_sets";
pub const TRACE_ACTIVITIES: &'static str = "trace_activities";
pub const PATTERNS: &'static str = "patterns";
pub const PETRI_NET: &'static str = "petri_net";
pub const ACTIVITIES_TO_LOGS: &'static str = "activities_to_logs";
pub const ACTIVITY_NAME: &'static str = "activity_name";
pub const HASHES_EVENT_LOG: &'static str = "hashes_event_log";
pub const NAMES_EVENT_LOG: &'static str = "names_event_log";
pub const COLORS_EVENT_LOG: &'static str = "colors_event_log";
pub const COLORS_HOLDER: &'static str = "colors_holder";
pub const GRAPH: &'static str = "graph";
pub const PETRI_NET_COUNT_ANNOTATION: &'static str = "petri_net_count_annotation";
pub const PETRI_NET_FREQUENCY_ANNOTATION: &'static str = "petri_net_frequency_annotation";
pub const PETRI_NET_TRACE_FREQUENCY_ANNOTATION: &'static str = "petri_net_trace_frequency_annotation";
pub const TRACES_ACTIVITIES_DATASET: &'static str = "traces_activities_dataset";
pub const LABELED_TRACES_ACTIVITIES_DATASET: &'static str = "labeled_traces_activities_dataset";
pub const ACTIVITIES_REPR_SOURCE: &'static str = "activities_repr_source";
pub const DISTANCE: &'static str = "distance";
pub const EXECUTE_ONLY_ON_LAST_EXTRACTION: &'static str = "execute_only_on_last_extraction";
pub const LABELED_LOG_TRACES_DATASET: &'static str = "labeled_log_traces_dataset";
pub const LOG_TRACES_DATASET: &'static str = "log_traces_dataset";
pub const TRACES_REPR_SOURCE: &'static str = "traces_repr_source";
pub const SYSTEM_METADATA: &'static str = "system_metadata";
pub const LOG_SERIALIZATION_FORMAT: &'static str = "log_serialization_format";
pub const GRAPH_TIME_ANNOTATION: &'static str = "graph_time_annotation";
pub const ATTRIBUTE: &'static str = "attribute";
pub const TIME_ANNOTATION_KIND: &'static str = "time_annotation_kind";
pub const ATTRIBUTES: &'static str = "attributes";
pub const PATHS: &'static str = "paths";
pub const LOG_THREADS_DIAGRAM: &'static str = "log_threads_diagram";
pub const THREAD_ATTRIBUTE: &'static str = "thread_attribute";
pub const TIME_ATTRIBUTE: &'static str = "time_attribute";
pub const TIME_DELTA: &'static str = "time_delta";
pub const FEATURE_COUNT_KIND: &'static str = "feature_count_kind";
pub const PERCENT_FROM_MAX_VALUE: &'static str = "percent_from_max_value";
pub const TOLERANCES: &'static str = "tolerances";
pub const MIN_POINTS_IN_CLUSTER_ARRAY: &'static str = "min_points_in_cluster_array";
pub const EXECUTION_ID: &'static str = "execution_id";
pub const ROOT_SEQUENCE_KIND: &'static str = "root_sequence_kind";
pub const SOFTWARE_DATA: &'static str = "software_data";
pub const CORRESPONDING_TRACE_DATA: &'static str = "corresponding_trace_data";
pub const INNER_GRAPH: &'static str = "inner_graph";
pub const START_END_ACTIVITY_TIME: &'static str = "start_end_activity_time";
pub const START_END_ACTIVITIES_TIMES: &'static str = "start_end_activities_times";
pub const MERGE_SEQUENCES_OF_EVENTS: &'static str = "merge_sequences_of_events";
pub const UNDERLYING_PATTERNS_INFOS: &'static str = "underlying_patterns_infos";

#[rustfmt::skip]
lazy_static!(
     pub static ref EVENT_LOG_KEY: DefaultContextKey<XesEventLogImpl> = DefaultContextKey::new(EVENT_LOG);
     pub static ref ACTIVITIES_KEY: DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>> = DefaultContextKey::new(ACTIVITIES);
     pub static ref REPEAT_SETS_KEY: DefaultContextKey<Vec<SubArrayWithTraceIndex>> = DefaultContextKey::new(REPEAT_SETS);
     pub static ref TRACE_ACTIVITIES_KEY: DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>> = DefaultContextKey::new(TRACE_ACTIVITIES);
     pub static ref PATTERNS_KEY: DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>> = DefaultContextKey::new(PATTERNS);
     pub static ref PETRI_NET_KEY: DefaultContextKey<DefaultPetriNet> = DefaultContextKey::new(PETRI_NET);
     pub static ref ACTIVITIES_TO_LOGS_KEY: DefaultContextKey<HashMap<String, XesEventLogImpl>> = DefaultContextKey::new(ACTIVITIES_TO_LOGS);
     pub static ref ACTIVITY_NAME_KEY: DefaultContextKey<String> = DefaultContextKey::new(ACTIVITY_NAME);
     pub static ref HASHES_EVENT_LOG_KEY: DefaultContextKey<Vec<Vec<u64>>> = DefaultContextKey::new(HASHES_EVENT_LOG);
     pub static ref NAMES_EVENT_LOG_KEY: DefaultContextKey<Vec<Vec<String>>> = DefaultContextKey::new(NAMES_EVENT_LOG);
     pub static ref TANDEM_ARRAY_LENGTH_KEY: DefaultContextKey<u32> = DefaultContextKey::new(TANDEM_ARRAY_LENGTH);
     pub static ref ACTIVITY_LEVEL_KEY: DefaultContextKey<u32> = DefaultContextKey::new(ACTIVITY_LEVEL);
     pub static ref NARROW_ACTIVITIES_KEY: DefaultContextKey<ActivityNarrowingKind> = DefaultContextKey::new(NARROW_ACTIVITIES);
     pub static ref EVENT_NAME_KEY: DefaultContextKey<String> = DefaultContextKey::new(EVENT_NAME);
     pub static ref REGEX_KEY: DefaultContextKey<String> = DefaultContextKey::new(REGEX);
     pub static ref COLORS_EVENT_LOG_KEY: DefaultContextKey<ColorsEventLog> = DefaultContextKey::new(COLORS_EVENT_LOG);
     pub static ref COLORS_HOLDER_KEY: DefaultContextKey<ColorsHolder> = DefaultContextKey::new(COLORS_HOLDER);
     pub static ref PATTERNS_DISCOVERY_STRATEGY_KEY: DefaultContextKey<PatternsDiscoveryStrategy> = DefaultContextKey::new(PATTERNS_DISCOVERY_STRATEGY);
     pub static ref OUTPUT_STRING_KEY: DefaultContextKey<String> = DefaultContextKey::new(OUTPUT_STRING);
     pub static ref EVENT_LOG_INFO_KEY: DefaultContextKey<OfflineEventLogInfo> = DefaultContextKey::new(EVENT_LOG_INFO);
     pub static ref UNDERLYING_EVENTS_COUNT_KEY: DefaultContextKey<usize> = DefaultContextKey::new(UNDERLYING_EVENTS_COUNT);
     pub static ref EVENTS_COUNT_KEY: DefaultContextKey<u32> = DefaultContextKey::new(EVENTS_COUNT);
     pub static ref EVENT_CLASSES_REGEXES_KEY: DefaultContextKey<Vec<String>> = DefaultContextKey::new(EVENT_CLASSES_REGEXES);
     pub static ref ADJUSTING_MODE_KEY: DefaultContextKey<AdjustingMode> = DefaultContextKey::new(ADJUSTING_MODE);
     pub static ref EVENT_CLASS_REGEX_KEY: DefaultContextKey<String> = DefaultContextKey::new(EVENT_CLASS_REGEX);
     pub static ref PATTERNS_KIND_KEY: DefaultContextKey<PatternsKindDto> = DefaultContextKey::new(PATTERNS_KIND);
     pub static ref PIPELINE_KEY: DefaultContextKey<Pipeline> = DefaultContextKey::new(PIPELINE);
     pub static ref MIN_ACTIVITY_LENGTH_KEY: DefaultContextKey<u32> = DefaultContextKey::new(MIN_ACTIVITY_LENGTH);
     pub static ref UNDEF_ACTIVITY_HANDLING_STRATEGY_KEY: DefaultContextKey<UndefActivityHandlingStrategyDto> = DefaultContextKey::new(UNDEF_ACTIVITY_HANDLING_STRATEGY);
     pub static ref ACTIVITY_IN_TRACE_FILTER_KIND_KEY: DefaultContextKey<ActivityInTraceFilterKind> = DefaultContextKey::new(ACTIVITY_IN_TRACE_FILTER_KIND);
     pub static ref ACTIVITIES_LOGS_SOURCE_KEY: DefaultContextKey<ActivitiesLogsSourceDto> = DefaultContextKey::new(ACTIVITIES_LOGS_SOURCE);
     pub static ref PNML_USE_NAMES_AS_IDS_KEY: DefaultContextKey<bool> = DefaultContextKey::new(PNML_USE_NAMES_AS_IDS);
     pub static ref GRAPH_KEY: DefaultContextKey<DefaultGraph> = DefaultContextKey::new(GRAPH);
     pub static ref DEPENDENCY_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(DEPENDENCY_THRESHOLD);
     pub static ref POSITIVE_OBSERVATIONS_THRESHOLD_KEY: DefaultContextKey<u32> = DefaultContextKey::new(POSITIVE_OBSERVATIONS_THRESHOLD);
     pub static ref RELATIVE_TO_BEST_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(RELATIVE_TO_BEST_THRESHOLD);
     pub static ref AND_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(AND_THRESHOLD);
     pub static ref LOOP_LENGTH_TWO_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(LOOP_LENGTH_TWO_THRESHOLD);
     pub static ref UNARY_FREQUENCY_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(UNARY_FREQUENCY_THRESHOLD);
     pub static ref BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD);
);

#[rustfmt::skip]
lazy_static!(
     pub static ref PRESERVE_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(PRESERVE_THRESHOLD);
     pub static ref RATIO_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(RATIO_THRESHOLD);
     pub static ref UTILITY_RATE_KEY: DefaultContextKey<f64> = DefaultContextKey::new(UTILITY_RATE);
     pub static ref EDGE_CUTOFF_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(EDGE_CUTOFF_THRESHOLD);
     pub static ref NODE_CUTOFF_THRESHOLD_KEY: DefaultContextKey<f64> = DefaultContextKey::new(NODE_CUTOFF_THRESHOLD);
     pub static ref PETRI_NET_COUNT_ANNOTATION_KEY: DefaultContextKey<HashMap<u64, usize>> = DefaultContextKey::new(PETRI_NET_COUNT_ANNOTATION);
     pub static ref PETRI_NET_FREQUENCY_ANNOTATION_KEY: DefaultContextKey<HashMap<u64, f64>> = DefaultContextKey::new(PETRI_NET_FREQUENCY_ANNOTATION);
     pub static ref PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY: DefaultContextKey<HashMap<u64, f64>> = DefaultContextKey::new(PETRI_NET_TRACE_FREQUENCY_ANNOTATION);
     pub static ref TERMINATE_ON_UNREPLAYABLE_TRACES_KEY: DefaultContextKey<bool> = DefaultContextKey::new(TERMINATE_ON_UNREPLAYABLE_TRACES);
     pub static ref CLUSTERS_COUNT_KEY: DefaultContextKey<u32> = DefaultContextKey::new(CLUSTERS_COUNT);
     pub static ref LEARNING_ITERATIONS_COUNT_KEY: DefaultContextKey<u32> = DefaultContextKey::new(LEARNING_ITERATIONS_COUNT);
     pub static ref TOLERANCE_KEY: DefaultContextKey<f64> = DefaultContextKey::new(TOLERANCE);
     pub static ref MIN_EVENTS_IN_CLUSTERS_COUNT_KEY: DefaultContextKey<u32> = DefaultContextKey::new(MIN_EVENTS_IN_CLUSTERS_COUNT);
     pub static ref TRACES_ACTIVITIES_DATASET_KEY: DefaultContextKey<FicusDataset> = DefaultContextKey::new(TRACES_ACTIVITIES_DATASET);
     pub static ref LABELED_TRACES_ACTIVITIES_DATASET_KEY: DefaultContextKey<LabeledDataset> = DefaultContextKey::new(LABELED_TRACES_ACTIVITIES_DATASET);
     pub static ref ACTIVITIES_REPR_SOURCE_KEY: DefaultContextKey<ActivityRepresentationSource> = DefaultContextKey::new(ACTIVITIES_REPR_SOURCE);
     pub static ref DISTANCE_KEY: DefaultContextKey<FicusDistance> = DefaultContextKey::new(DISTANCE);
     pub static ref EXECUTE_ONLY_ON_LAST_EXTRACTION_KEY: DefaultContextKey<bool> = DefaultContextKey::new(EXECUTE_ONLY_ON_LAST_EXTRACTION);
     pub static ref EVENT_LOG_NAME_KEY: DefaultContextKey<String> = DefaultContextKey::new(EVENT_LOG_NAME);
     pub static ref LOG_TRACES_DATASET_KEY: DefaultContextKey<FicusDataset> = DefaultContextKey::new(LOG_TRACES_DATASET);
     pub static ref LABELED_LOG_TRACES_DATASET_KEY: DefaultContextKey<LabeledDataset> = DefaultContextKey::new(LABELED_LOG_TRACES_DATASET);
     pub static ref TRACES_REPRESENTATION_SOURCE_KEY: DefaultContextKey<TracesRepresentationSource> = DefaultContextKey::new(TRACES_REPR_SOURCE);
     pub static ref SYSTEM_METADATA_KEY: DefaultContextKey<SystemMetadata> = DefaultContextKey::new(SYSTEM_METADATA);
     pub static ref LOG_SERIALIZATION_FORMAT_KEY: DefaultContextKey<LogSerializationFormat> = DefaultContextKey::new(LOG_SERIALIZATION_FORMAT);
     pub static ref BYTES_KEY: DefaultContextKey<Vec<u8>> = DefaultContextKey::new(BYTES);
     pub static ref PATH_KEY: DefaultContextKey<String> = DefaultContextKey::new(PATH);
     pub static ref CASE_NAME: DefaultContextKey<CaseName> = DefaultContextKey::new(CASE_NAME_STR);
     pub static ref PROCESS_NAME: DefaultContextKey<String> = DefaultContextKey::new(PROCESS_NAME_STR);
     pub static ref PIPELINE_NAME: DefaultContextKey<String> = DefaultContextKey::new(PIPELINE_NAME_STR);
     pub static ref PIPELINE_ID: DefaultContextKey<Uuid> = DefaultContextKey::new(PIPELINE_ID_STR);
     pub static ref SUBSCRIPTION_NAME: DefaultContextKey<String> = DefaultContextKey::new(SUBSCRIPTION_NAME_STR);
     pub static ref SUBSCRIPTION_ID: DefaultContextKey<Uuid> = DefaultContextKey::new(SUBSCRIPTION_ID_STR);
     pub static ref UNSTRUCTURED_METADATA: DefaultContextKey<Vec<(String, String)>> = DefaultContextKey::new(PROCESS_NAME_STR);
     pub static ref START_CASE_REGEX: DefaultContextKey<String> = DefaultContextKey::new(START_CASE_REGEX_STR);
     pub static ref END_CASE_REGEX: DefaultContextKey<String> = DefaultContextKey::new(END_CASE_REGEX_STR);
     pub static ref INLINE_INNER_CASES: DefaultContextKey<bool> = DefaultContextKey::new(INLINE_INNER_CASES_STR);
     pub static ref GRAPH_TIME_ANNOTATION_KEY: DefaultContextKey<HashMap<u64, f64>> = DefaultContextKey::new(GRAPH_TIME_ANNOTATION);
     pub static ref ATTRIBUTE_KEY: DefaultContextKey<String> = DefaultContextKey::new(ATTRIBUTE);
     pub static ref TIME_ANNOTATION_KIND_KEY: DefaultContextKey<TimeAnnotationKind> = DefaultContextKey::new(TIME_ANNOTATION_KIND);
     pub static ref ATTRIBUTES_KEY: DefaultContextKey<Vec<String>> = DefaultContextKey::new(ATTRIBUTES);
     pub static ref PATHS_KEY: DefaultContextKey<Vec<String>> = DefaultContextKey::new(PATHS);
     pub static ref LOG_THREADS_DIAGRAM_KEY: DefaultContextKey<LogTimelineDiagram> = DefaultContextKey::new(LOG_THREADS_DIAGRAM);
     pub static ref THREAD_ATTRIBUTE_KEY: DefaultContextKey<String> = DefaultContextKey::new(THREAD_ATTRIBUTE);
     pub static ref TIME_ATTRIBUTE_KEY: DefaultContextKey<String> = DefaultContextKey::new(TIME_ATTRIBUTE);
     pub static ref TIME_DELTA_KEY: DefaultContextKey<u32> = DefaultContextKey::new(TIME_DELTA);
     pub static ref FEATURE_COUNT_KIND_KEY: DefaultContextKey<FeatureCountKindDto> = DefaultContextKey::new(FEATURE_COUNT_KIND);
     pub static ref PERCENT_FROM_MAX_VALUE_KEY: DefaultContextKey<f64> = DefaultContextKey::new(PERCENT_FROM_MAX_VALUE);
     pub static ref TOLERANCES_KEY: DefaultContextKey<Vec<f64>> = DefaultContextKey::new(TOLERANCES);
     pub static ref MIN_POINTS_IN_CLUSTER_ARRAY_KEY: DefaultContextKey<Vec<u64>> = DefaultContextKey::new(MIN_POINTS_IN_CLUSTER_ARRAY);
     pub static ref EXECUTION_ID_KEY: DefaultContextKey<Uuid> = DefaultContextKey::new(EXECUTION_ID);
     pub static ref ROOT_SEQUENCE_KIND_KEY: DefaultContextKey<RootSequenceKind> = DefaultContextKey::new(ROOT_SEQUENCE_KIND);
     pub static ref MERGE_SEQUENCES_OF_EVENTS_KEY: DefaultContextKey<bool> = DefaultContextKey::new(START_END_ACTIVITIES_TIMES);
);

lazy_static!(
     pub static ref SOFTWARE_DATA_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<SoftwareData>>> = DefaultContextKey::new(SOFTWARE_DATA);
     pub static ref CORRESPONDING_TRACE_DATA_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<CorrespondingTraceData>>> = DefaultContextKey::new(CORRESPONDING_TRACE_DATA);
     pub static ref INNER_GRAPH_KEY: DefaultContextKey<DefaultGraph> = DefaultContextKey::new(SOFTWARE_DATA);
     pub static ref START_END_ACTIVITY_TIME_KEY: DefaultContextKey<NodeAdditionalDataContainer<ActivityStartEndTimeData>> = DefaultContextKey::new(START_END_ACTIVITY_TIME);
     pub static ref START_END_ACTIVITIES_TIMES_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<ActivityStartEndTimeData>>> = DefaultContextKey::new(START_END_ACTIVITIES_TIMES);
     pub static ref UNDERLYING_PATTERNS_INFOS_KEY: DefaultContextKey<Vec<NodeAdditionalDataContainer<UnderlyingPatternInfo>>> = DefaultContextKey::new(UNDERLYING_PATTERNS_INFOS);
);

pub fn find_context_key(name: &str) -> Option<&dyn ContextKey> {
  match name {
    PATH => Some(PATH_KEY.deref() as &dyn ContextKey),
    TANDEM_ARRAY_LENGTH => Some(TANDEM_ARRAY_LENGTH_KEY.deref() as &dyn ContextKey),
    ACTIVITY_LEVEL => Some(ACTIVITY_LEVEL_KEY.deref() as &dyn ContextKey),
    NARROW_ACTIVITIES => Some(NARROW_ACTIVITIES_KEY.deref() as &dyn ContextKey),
    EVENT_NAME => Some(EVENT_NAME_KEY.deref() as &dyn ContextKey),
    REGEX => Some(REGEX_KEY.deref() as &dyn ContextKey),
    PATTERNS_DISCOVERY_STRATEGY => Some(PATTERNS_DISCOVERY_STRATEGY_KEY.deref() as &dyn ContextKey),
    OUTPUT_STRING => Some(OUTPUT_STRING_KEY.deref() as &dyn ContextKey),
    EVENT_LOG_INFO => Some(EVENT_LOG_INFO_KEY.deref() as &dyn ContextKey),
    UNDERLYING_EVENTS_COUNT => Some(UNDERLYING_EVENTS_COUNT_KEY.deref() as &dyn ContextKey),
    EVENTS_COUNT => Some(EVENTS_COUNT_KEY.deref() as &dyn ContextKey),
    EVENT_CLASSES_REGEXES => Some(EVENT_CLASSES_REGEXES_KEY.deref() as &dyn ContextKey),
    ADJUSTING_MODE => Some(ADJUSTING_MODE_KEY.deref() as &dyn ContextKey),
    EVENT_CLASS_REGEX => Some(EVENT_CLASS_REGEX_KEY.deref() as &dyn ContextKey),
    PATTERNS_KIND => Some(PATTERNS_KIND_KEY.deref() as &dyn ContextKey),
    PIPELINE => Some(PIPELINE_KEY.deref() as &dyn ContextKey),
    MIN_ACTIVITY_LENGTH => Some(MIN_ACTIVITY_LENGTH_KEY.deref() as &dyn ContextKey),
    UNDEF_ACTIVITY_HANDLING_STRATEGY => Some(UNDEF_ACTIVITY_HANDLING_STRATEGY_KEY.deref() as &dyn ContextKey),
    ACTIVITY_IN_TRACE_FILTER_KIND => Some(ACTIVITY_IN_TRACE_FILTER_KIND_KEY.deref() as &dyn ContextKey),
    ACTIVITIES_LOGS_SOURCE => Some(ACTIVITIES_LOGS_SOURCE_KEY.deref() as &dyn ContextKey),
    PNML_USE_NAMES_AS_IDS => Some(PNML_USE_NAMES_AS_IDS_KEY.deref() as &dyn ContextKey),
    DEPENDENCY_THRESHOLD => Some(DEPENDENCY_THRESHOLD_KEY.deref() as &dyn ContextKey),
    POSITIVE_OBSERVATIONS_THRESHOLD => Some(POSITIVE_OBSERVATIONS_THRESHOLD_KEY.deref() as &dyn ContextKey),
    RELATIVE_TO_BEST_THRESHOLD => Some(RELATIVE_TO_BEST_THRESHOLD_KEY.deref() as &dyn ContextKey),
    AND_THRESHOLD => Some(AND_THRESHOLD_KEY.deref() as &dyn ContextKey),
    LOOP_LENGTH_TWO_THRESHOLD => Some(LOOP_LENGTH_TWO_THRESHOLD_KEY.deref() as &dyn ContextKey),
    UNARY_FREQUENCY_THRESHOLD => Some(UNARY_FREQUENCY_THRESHOLD_KEY.deref() as &dyn ContextKey),
    BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD => Some(BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD_KEY.deref() as &dyn ContextKey),
    PRESERVE_THRESHOLD => Some(PRESERVE_THRESHOLD_KEY.deref() as &dyn ContextKey),
    RATIO_THRESHOLD => Some(RATIO_THRESHOLD_KEY.deref() as &dyn ContextKey),
    UTILITY_RATE => Some(UTILITY_RATE_KEY.deref() as &dyn ContextKey),
    EDGE_CUTOFF_THRESHOLD => Some(EDGE_CUTOFF_THRESHOLD_KEY.deref() as &dyn ContextKey),
    NODE_CUTOFF_THRESHOLD => Some(NODE_CUTOFF_THRESHOLD_KEY.deref() as &dyn ContextKey),
    TERMINATE_ON_UNREPLAYABLE_TRACES => Some(TERMINATE_ON_UNREPLAYABLE_TRACES_KEY.deref() as &dyn ContextKey),
    CLUSTERS_COUNT => Some(CLUSTERS_COUNT_KEY.deref() as &dyn ContextKey),
    LEARNING_ITERATIONS_COUNT => Some(LEARNING_ITERATIONS_COUNT_KEY.deref() as &dyn ContextKey),
    TOLERANCE => Some(TOLERANCE_KEY.deref() as &dyn ContextKey),
    MIN_EVENTS_IN_CLUSTERS_COUNT => Some(MIN_EVENTS_IN_CLUSTERS_COUNT_KEY.deref() as &dyn ContextKey),
    EVENT_LOG_NAME => Some(EVENT_LOG_NAME_KEY.deref() as &dyn ContextKey),
    BYTES => Some(BYTES_KEY.deref() as &dyn ContextKey),
    EVENT_LOG => Some(EVENT_LOG_KEY.deref() as &dyn ContextKey),
    ACTIVITIES => Some(ACTIVITIES_KEY.deref() as &dyn ContextKey),
    REPEAT_SETS => Some(REPEAT_SETS_KEY.deref() as &dyn ContextKey),
    TRACE_ACTIVITIES => Some(TRACE_ACTIVITIES_KEY.deref() as &dyn ContextKey),
    PATTERNS => Some(PATTERNS_KEY.deref() as &dyn ContextKey),
    PETRI_NET => Some(PETRI_NET_KEY.deref() as &dyn ContextKey),
    ACTIVITIES_TO_LOGS => Some(ACTIVITIES_TO_LOGS_KEY.deref() as &dyn ContextKey),
    ACTIVITY_NAME => Some(ACTIVITY_NAME_KEY.deref() as &dyn ContextKey),
    HASHES_EVENT_LOG => Some(HASHES_EVENT_LOG_KEY.deref() as &dyn ContextKey),
    NAMES_EVENT_LOG => Some(NAMES_EVENT_LOG_KEY.deref() as &dyn ContextKey),
    COLORS_EVENT_LOG => Some(COLORS_EVENT_LOG_KEY.deref() as &dyn ContextKey),
    COLORS_HOLDER => Some(COLORS_HOLDER_KEY.deref() as &dyn ContextKey),
    GRAPH => Some(GRAPH_KEY.deref() as &dyn ContextKey),
    PETRI_NET_COUNT_ANNOTATION => Some(PETRI_NET_COUNT_ANNOTATION_KEY.deref() as &dyn ContextKey),
    PETRI_NET_FREQUENCY_ANNOTATION => Some(PETRI_NET_FREQUENCY_ANNOTATION_KEY.deref() as &dyn ContextKey),
    PETRI_NET_TRACE_FREQUENCY_ANNOTATION => Some(PETRI_NET_TRACE_FREQUENCY_ANNOTATION_KEY.deref() as &dyn ContextKey),
    TRACES_ACTIVITIES_DATASET => Some(TRACES_ACTIVITIES_DATASET_KEY.deref() as &dyn ContextKey),
    LABELED_TRACES_ACTIVITIES_DATASET => Some(LABELED_TRACES_ACTIVITIES_DATASET_KEY.deref() as &dyn ContextKey),
    ACTIVITIES_REPR_SOURCE => Some(ACTIVITIES_REPR_SOURCE_KEY.deref() as &dyn ContextKey),
    DISTANCE => Some(DISTANCE_KEY.deref() as &dyn ContextKey),
    EXECUTE_ONLY_ON_LAST_EXTRACTION => Some(EXECUTE_ONLY_ON_LAST_EXTRACTION_KEY.deref() as &dyn ContextKey),
    LABELED_LOG_TRACES_DATASET => Some(LABELED_LOG_TRACES_DATASET_KEY.deref() as &dyn ContextKey),
    LOG_TRACES_DATASET => Some(LOG_TRACES_DATASET_KEY.deref() as &dyn ContextKey),
    TRACES_REPR_SOURCE => Some(TRACES_REPRESENTATION_SOURCE_KEY.deref() as &dyn ContextKey),
    SYSTEM_METADATA => Some(SYSTEM_METADATA_KEY.deref() as &dyn ContextKey),
    LOG_SERIALIZATION_FORMAT => Some(LOG_SERIALIZATION_FORMAT_KEY.deref() as &dyn ContextKey),
    START_CASE_REGEX_STR => Some(START_CASE_REGEX.deref() as &dyn ContextKey),
    END_CASE_REGEX_STR => Some(END_CASE_REGEX.deref() as &dyn ContextKey),
    INLINE_INNER_CASES_STR => Some(INLINE_INNER_CASES.deref() as &dyn ContextKey),
    GRAPH_TIME_ANNOTATION => Some(GRAPH_TIME_ANNOTATION_KEY.deref() as &dyn ContextKey),
    ATTRIBUTE => Some(ATTRIBUTE_KEY.deref() as &dyn ContextKey),
    TIME_ANNOTATION_KIND => Some(TIME_ANNOTATION_KIND_KEY.deref() as &dyn ContextKey),
    ATTRIBUTES => Some(ATTRIBUTES_KEY.deref() as &dyn ContextKey),
    PATHS => Some(PATHS_KEY.deref() as &dyn ContextKey),
    LOG_THREADS_DIAGRAM => Some(LOG_THREADS_DIAGRAM_KEY.deref() as &dyn ContextKey),
    THREAD_ATTRIBUTE => Some(THREAD_ATTRIBUTE_KEY.deref() as &dyn ContextKey),
    TIME_ATTRIBUTE => Some(TIME_ATTRIBUTE_KEY.deref() as &dyn ContextKey),
    TIME_DELTA => Some(TIME_DELTA_KEY.deref() as &dyn ContextKey),
    FEATURE_COUNT_KIND => Some(FEATURE_COUNT_KIND_KEY.deref() as &dyn ContextKey),
    PERCENT_FROM_MAX_VALUE => Some(PERCENT_FROM_MAX_VALUE_KEY.deref() as &dyn ContextKey),
    TOLERANCES => Some(TOLERANCES_KEY.deref() as &dyn ContextKey),
    MIN_POINTS_IN_CLUSTER_ARRAY => Some(MIN_POINTS_IN_CLUSTER_ARRAY_KEY.deref() as &dyn ContextKey),
    EXECUTION_ID => Some(EXECUTION_ID_KEY.deref() as &dyn ContextKey),
    ROOT_SEQUENCE_KIND => Some(ROOT_SEQUENCE_KIND_KEY.deref() as &dyn ContextKey),
    SOFTWARE_DATA => Some(SOFTWARE_DATA_KEY.deref() as &dyn ContextKey),
    CORRESPONDING_TRACE_DATA => Some(CORRESPONDING_TRACE_DATA_KEY.deref() as &dyn ContextKey),
    INNER_GRAPH => Some(INNER_GRAPH_KEY.deref() as &dyn ContextKey),
    START_END_ACTIVITY_TIME => Some(START_END_ACTIVITY_TIME_KEY.deref() as &dyn ContextKey),
    START_END_ACTIVITIES_TIMES => Some(START_END_ACTIVITIES_TIMES_KEY.deref() as &dyn ContextKey),
    MERGE_SEQUENCES_OF_EVENTS => Some(MERGE_SEQUENCES_OF_EVENTS_KEY.deref() as &dyn ContextKey),
    UNDERLYING_PATTERNS_INFOS => Some(UNDERLYING_PATTERNS_INFOS_KEY.deref() as &dyn ContextKey),
    _ => None,
  }
}
