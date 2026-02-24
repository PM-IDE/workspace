use crate::{
  event_log::xes::xes_event_log::XesEventLogImpl,
  features::{
    analysis::{
      log_info::event_log_info::OfflineEventLogInfo,
      patterns::{
        activity_instances::{ActivityInTraceFilterKind, ActivityInTraceInfo, ActivityNarrowingKind, AdjustingMode},
        contexts::PatternsDiscoveryStrategy,
        repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
        tandem_arrays::SubArrayInTraceInfo,
      },
    },
    cases::CaseName,
    clustering::{activities::activities_params::ActivityRepresentationSource, traces::traces_params::TracesRepresentationSource},
    discovery::{
      ecfg::models::RootSequenceKind,
      ocel::graph_annotation::OcelAnnotation,
      petri_net::{annotations::TimeAnnotationKind, petri_net::DefaultPetriNet},
      timeline::{discovery::LogTimelineDiagram, software_data::extraction_config::SoftwareDataExtractionConfig},
    },
  },
  pipelines::{
    activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto},
    multithreading::FeatureCountKindDto,
    patterns_parts::PatternsKindDto,
    pipelines::Pipeline,
  },
  utils::{
    colors::{ColorsEventLog, ColorsHolder},
    context_key::ContextKey,
    dataset::dataset::{FicusDataset, LabeledDataset},
    distance::distance::FicusDistance,
    graph::graph::DefaultGraph,
    log_serialization_format::LogSerializationFormat,
  },
};
use bxes::models::system_models::SystemMetadata;
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};
use uuid::Uuid;

pub const CASE_NAME: &str = "case_name";
pub const PROCESS_NAME: &str = "process_name";
pub const SUBSCRIPTION_ID: &str = "subscription_id";
pub const SUBSCRIPTION_NAME: &str = "subscription_name";
pub const PIPELINE_ID: &str = "pipeline_id";
pub const PIPELINE_NAME: &str = "pipeline_name";
pub const UNSTRUCTURED_METADATA: &str = "unstructured_metadata";
pub const PATH: &str = "path";
pub const TANDEM_ARRAY_LENGTH: &str = "tandem_array_length";
pub const ACTIVITY_LEVEL: &str = "activity_level";
pub const NARROW_ACTIVITIES: &str = "narrow_activities";
pub const EVENT_NAME: &str = "event_name";
pub const REGEX: &str = "regex";
pub const PATTERNS_DISCOVERY_STRATEGY: &str = "patterns_discovery_strategy";
pub const OUTPUT_STRING: &str = "output_string";
pub const EVENT_LOG_INFO: &str = "event_log_info";
pub const UNDERLYING_EVENTS_COUNT: &str = "underlying_events_count";
pub const EVENTS_COUNT: &str = "events_count";
pub const REGEXES: &str = "regexes";
pub const ADJUSTING_MODE: &str = "adjusting_mode";
pub const EVENT_CLASS_REGEX: &str = "event_class_regex";
pub const PATTERNS_KIND: &str = "patterns_kind";
pub const PIPELINE: &str = "pipeline";
pub const MIN_ACTIVITY_LENGTH: &str = "min_activity_length";
pub const UNDEF_ACTIVITY_HANDLING_STRATEGY: &str = "undef_activity_handling_strategy";
pub const ACTIVITY_IN_TRACE_FILTER_KIND: &str = "activity_in_trace_filter_kind";
pub const ACTIVITIES_LOGS_SOURCE: &str = "activities_logs_source";
pub const PNML_USE_NAMES_AS_IDS: &str = "pnml_use_names_as_ids";
pub const DEPENDENCY_THRESHOLD: &str = "dependency_threshold";
pub const POSITIVE_OBSERVATIONS_THRESHOLD: &str = "positive_observations_threshold";
pub const RELATIVE_TO_BEST_THRESHOLD: &str = "relative_to_best_threshold";
pub const AND_THRESHOLD: &str = "and_threshold";
pub const LOOP_LENGTH_TWO_THRESHOLD: &str = "loop_length_two_threshold";
pub const UNARY_FREQUENCY_THRESHOLD: &str = "unary_frequency_threshold";
pub const BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD: &str = "binary_frequency_significance_threshold";
pub const PRESERVE_THRESHOLD: &str = "preserve_threshold";
pub const RATIO_THRESHOLD: &str = "ratio_threshold";
pub const UTILITY_RATE: &str = "utility_rate";
pub const EDGE_CUTOFF_THRESHOLD: &str = "edge_cutoff_threshold";
pub const NODE_CUTOFF_THRESHOLD: &str = "node_cutoff_threshold";
pub const TERMINATE_ON_UNREPLAYABLE_TRACES: &str = "terminate_on_unreplayable_traces";
pub const CLUSTERS_COUNT: &str = "clusters_count";
pub const LEARNING_ITERATIONS_COUNT: &str = "learning_iterations_count";
pub const TOLERANCE: &str = "tolerance";
pub const MIN_EVENTS_IN_CLUSTERS_COUNT: &str = "min_events_in_cluster_count";
pub const EVENT_LOG_NAME: &str = "event_log_name";
pub const BYTES: &str = "bytes";
pub const START_CASE_REGEX: &str = "start_case_regex";
pub const END_CASE_REGEX: &str = "end_case_regex";
pub const INLINE_INNER_CASES: &str = "inline_inner_cases";

pub const EVENT_LOG: &str = "event_log";
pub const ACTIVITIES: &str = "activities";
pub const REPEAT_SETS: &str = "repeat_sets";
pub const TRACE_ACTIVITIES: &str = "trace_activities";
pub const PATTERNS: &str = "patterns";
pub const PETRI_NET: &str = "petri_net";
pub const ACTIVITIES_TO_LOGS: &str = "activities_to_logs";
pub const ACTIVITY_NAME: &str = "activity_name";
pub const HASHES_EVENT_LOG: &str = "hashes_event_log";
pub const NAMES_EVENT_LOG: &str = "names_event_log";
pub const COLORS_EVENT_LOG: &str = "colors_event_log";
pub const COLORS_HOLDER: &str = "colors_holder";
pub const GRAPH: &str = "graph";
pub const GRAPHS: &str = "graphs";
pub const PETRI_NET_COUNT_ANNOTATION: &str = "petri_net_count_annotation";
pub const PETRI_NET_FREQUENCY_ANNOTATION: &str = "petri_net_frequency_annotation";
pub const PETRI_NET_TRACE_FREQUENCY_ANNOTATION: &str = "petri_net_trace_frequency_annotation";
pub const TRACES_ACTIVITIES_DATASET: &str = "traces_activities_dataset";
pub const LABELED_TRACES_ACTIVITIES_DATASET: &str = "labeled_traces_activities_dataset";
pub const ACTIVITIES_REPR_SOURCE: &str = "activities_repr_source";
pub const DISTANCE: &str = "distance";
pub const EXECUTE_ONLY_ON_LAST_EXTRACTION: &str = "execute_only_on_last_extraction";
pub const LABELED_LOG_TRACES_DATASET: &str = "labeled_log_traces_dataset";
pub const LOG_TRACES_DATASET: &str = "log_traces_dataset";
pub const TRACES_REPR_SOURCE: &str = "traces_repr_source";
pub const SYSTEM_METADATA: &str = "system_metadata";
pub const LOG_SERIALIZATION_FORMAT: &str = "log_serialization_format";
pub const GRAPH_TIME_ANNOTATION: &str = "graph_time_annotation";
pub const ATTRIBUTE: &str = "attribute";
pub const TIME_ANNOTATION_KIND: &str = "time_annotation_kind";
pub const ATTRIBUTES: &str = "attributes";
pub const PATHS: &str = "paths";
pub const LOG_THREADS_DIAGRAM: &str = "log_threads_diagram";
pub const THREAD_ATTRIBUTE: &str = "thread_attribute";
pub const TIME_ATTRIBUTE: &str = "time_attribute";
pub const TIME_DELTA: &str = "time_delta";
pub const FEATURE_COUNT_KIND: &str = "feature_count_kind";
pub const PERCENT_FROM_MAX_VALUE: &str = "percent_from_max_value";
pub const TOLERANCES: &str = "tolerances";
pub const MIN_POINTS_IN_CLUSTER_ARRAY: &str = "min_points_in_cluster_array";
pub const EXECUTION_ID: &str = "execution_id";
pub const ROOT_SEQUENCE_KIND: &str = "root_sequence_kind";
pub const MERGE_SEQUENCES_OF_EVENTS: &str = "merge_sequences_of_events";
pub const DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE: &str = "discover_events_groups_in_each_trace";
pub const SOFTWARE_DATA_EXTRACTION_CONFIG: &str = "software_data_extraction_config";
pub const DISCOVER_ACTIVITY_INSTANCES_STRICT: &str = "discover_activity_instances_strict";
pub const PUT_NOISE_EVENTS_IN_ONE_CLUSTER: &str = "put_noise_events_in_one_cluster";
pub const OCEL_ANNOTATION: &str = "ocel_annotation";

#[macro_export]
macro_rules! context_key {
  ($name:ident, $t:ty) => {
    paste::paste! {
      lazy_static! {
        pub static ref [<$name _KEY>]: $crate::utils::context_key::DefaultContextKey<$t> = $crate::utils::context_key::DefaultContextKey::new($name);
      }
    }
  };
}

context_key! { PETRI_NET, DefaultPetriNet }
context_key! { EVENT_LOG, XesEventLogImpl }
context_key! { ACTIVITIES, Vec<Rc<RefCell<ActivityNode>>> }
context_key! { REPEAT_SETS, Vec<SubArrayWithTraceIndex> }
context_key! { TRACE_ACTIVITIES, Vec<Vec<ActivityInTraceInfo>> }
context_key! { PATTERNS, Vec<Vec<SubArrayInTraceInfo>> }
context_key! { ACTIVITIES_TO_LOGS, HashMap<String, XesEventLogImpl> }
context_key! { ACTIVITY_NAME, String }
context_key! { HASHES_EVENT_LOG, Vec<Vec<u64>> }
context_key! { NAMES_EVENT_LOG, Vec<Vec<String>> }
context_key! { TANDEM_ARRAY_LENGTH, u32 }
context_key! { ACTIVITY_LEVEL, u32 }
context_key! { NARROW_ACTIVITIES, ActivityNarrowingKind }
context_key! { EVENT_NAME, String }
context_key! { REGEX, String }
context_key! { COLORS_EVENT_LOG, ColorsEventLog }
context_key! { COLORS_HOLDER, ColorsHolder }
context_key! { PATTERNS_DISCOVERY_STRATEGY, PatternsDiscoveryStrategy }
context_key! { OUTPUT_STRING, String }
context_key! { EVENT_LOG_INFO, OfflineEventLogInfo }
context_key! { UNDERLYING_EVENTS_COUNT, usize }
context_key! { EVENTS_COUNT, u32 }
context_key! { REGEXES, Vec<String> }
context_key! { ADJUSTING_MODE, AdjustingMode }
context_key! { EVENT_CLASS_REGEX, String }
context_key! { PATTERNS_KIND, PatternsKindDto }
context_key! { PIPELINE, Pipeline }
context_key! { MIN_ACTIVITY_LENGTH, u32 }
context_key! { UNDEF_ACTIVITY_HANDLING_STRATEGY, UndefActivityHandlingStrategyDto }
context_key! { ACTIVITY_IN_TRACE_FILTER_KIND, ActivityInTraceFilterKind }
context_key! { ACTIVITIES_LOGS_SOURCE, ActivitiesLogsSourceDto }
context_key! { PNML_USE_NAMES_AS_IDS, bool }
context_key! { GRAPH, DefaultGraph }
context_key! { GRAPHS, Vec<DefaultGraph> }
context_key! { DEPENDENCY_THRESHOLD, f64 }
context_key! { POSITIVE_OBSERVATIONS_THRESHOLD, u32 }
context_key! { RELATIVE_TO_BEST_THRESHOLD, f64 }
context_key! { AND_THRESHOLD, f64 }
context_key! { LOOP_LENGTH_TWO_THRESHOLD, f64 }
context_key! { UNARY_FREQUENCY_THRESHOLD, f64 }
context_key! { BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD, f64 }
context_key! { PRESERVE_THRESHOLD, f64 }
context_key! { RATIO_THRESHOLD, f64 }
context_key! { UTILITY_RATE, f64 }
context_key! { EDGE_CUTOFF_THRESHOLD, f64 }
context_key! { NODE_CUTOFF_THRESHOLD, f64 }
context_key! { PETRI_NET_COUNT_ANNOTATION, HashMap<u64, usize> }
context_key! { PETRI_NET_FREQUENCY_ANNOTATION, HashMap<u64, f64> }
context_key! { PETRI_NET_TRACE_FREQUENCY_ANNOTATION, HashMap<u64, f64> }
context_key! { TERMINATE_ON_UNREPLAYABLE_TRACES, bool }
context_key! { CLUSTERS_COUNT, u32 }
context_key! { LEARNING_ITERATIONS_COUNT, u32 }
context_key! { TOLERANCE, f64 }
context_key! { MIN_EVENTS_IN_CLUSTERS_COUNT, u32 }
context_key! { TRACES_ACTIVITIES_DATASET, FicusDataset }
context_key! { LABELED_TRACES_ACTIVITIES_DATASET, LabeledDataset }
context_key! { ACTIVITIES_REPR_SOURCE, ActivityRepresentationSource }
context_key! { DISTANCE, FicusDistance }
context_key! { EXECUTE_ONLY_ON_LAST_EXTRACTION, bool }
context_key! { EVENT_LOG_NAME, String }
context_key! { LOG_TRACES_DATASET, FicusDataset }
context_key! { LABELED_LOG_TRACES_DATASET, LabeledDataset }
context_key! { TRACES_REPR_SOURCE, TracesRepresentationSource }
context_key! { SYSTEM_METADATA, SystemMetadata }
context_key! { LOG_SERIALIZATION_FORMAT, LogSerializationFormat }
context_key! { BYTES, Vec<u8> }
context_key! { PATH, String }
context_key! { CASE_NAME, CaseName }
context_key! { PROCESS_NAME, String }
context_key! { PIPELINE_NAME, String }
context_key! { PIPELINE_ID, Uuid }
context_key! { SUBSCRIPTION_NAME, String }
context_key! { SUBSCRIPTION_ID, Uuid }
context_key! { UNSTRUCTURED_METADATA, Vec<(String, String)> }
context_key! { START_CASE_REGEX, String }
context_key! { END_CASE_REGEX, String }
context_key! { INLINE_INNER_CASES, bool }
context_key! { GRAPH_TIME_ANNOTATION, HashMap<u64, f64> }
context_key! { ATTRIBUTE, String }
context_key! { TIME_ANNOTATION_KIND, TimeAnnotationKind }
context_key! { ATTRIBUTES, Vec<String> }
context_key! { PATHS, Vec<String> }
context_key! { LOG_THREADS_DIAGRAM, LogTimelineDiagram }
context_key! { THREAD_ATTRIBUTE, String }
context_key! { TIME_ATTRIBUTE, String }
context_key! { TIME_DELTA, u32 }
context_key! { FEATURE_COUNT_KIND, FeatureCountKindDto }
context_key! { PERCENT_FROM_MAX_VALUE, f64 }
context_key! { TOLERANCES, Vec<f64> }
context_key! { MIN_POINTS_IN_CLUSTER_ARRAY, Vec<u64> }
context_key! { EXECUTION_ID, Uuid }
context_key! { ROOT_SEQUENCE_KIND, RootSequenceKind }
context_key! { MERGE_SEQUENCES_OF_EVENTS, bool }
context_key! { DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE, bool }
context_key! { SOFTWARE_DATA_EXTRACTION_CONFIG, SoftwareDataExtractionConfig }
context_key! { DISCOVER_ACTIVITY_INSTANCES_STRICT, bool }
context_key! { PUT_NOISE_EVENTS_IN_ONE_CLUSTER, bool }
context_key! { OCEL_ANNOTATION, OcelAnnotation }

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
    REGEXES => Some(REGEXES_KEY.deref() as &dyn ContextKey),
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
    GRAPHS => Some(GRAPHS_KEY.deref() as &dyn ContextKey),
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
    TRACES_REPR_SOURCE => Some(TRACES_REPR_SOURCE_KEY.deref() as &dyn ContextKey),
    SYSTEM_METADATA => Some(SYSTEM_METADATA_KEY.deref() as &dyn ContextKey),
    LOG_SERIALIZATION_FORMAT => Some(LOG_SERIALIZATION_FORMAT_KEY.deref() as &dyn ContextKey),
    START_CASE_REGEX => Some(START_CASE_REGEX_KEY.deref() as &dyn ContextKey),
    END_CASE_REGEX => Some(END_CASE_REGEX_KEY.deref() as &dyn ContextKey),
    INLINE_INNER_CASES => Some(INLINE_INNER_CASES_KEY.deref() as &dyn ContextKey),
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
    MERGE_SEQUENCES_OF_EVENTS => Some(MERGE_SEQUENCES_OF_EVENTS_KEY.deref() as &dyn ContextKey),
    DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE => Some(DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE_KEY.deref() as &dyn ContextKey),
    SOFTWARE_DATA_EXTRACTION_CONFIG => Some(SOFTWARE_DATA_EXTRACTION_CONFIG_KEY.deref() as &dyn ContextKey),
    DISCOVER_ACTIVITY_INSTANCES_STRICT => Some(DISCOVER_ACTIVITY_INSTANCES_STRICT_KEY.deref() as &dyn ContextKey),
    PUT_NOISE_EVENTS_IN_ONE_CLUSTER => Some(PUT_NOISE_EVENTS_IN_ONE_CLUSTER_KEY.deref() as &dyn ContextKey),
    OCEL_ANNOTATION => Some(OCEL_ANNOTATION_KEY.deref() as &dyn ContextKey),
    _ => None,
  }
}
