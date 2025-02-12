use std::collections::HashMap;
use std::collections::HashSet;

use bxes::models::system_models::SystemMetadata;

use ficus::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use ficus::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use ficus::features::clustering::activities::activities_params::ActivityRepresentationSource;
use ficus::features::clustering::traces::traces_params::TracesRepresentationSource;
use ficus::features::discovery::petri_net::annotations::TimeAnnotationKind;
use ficus::features::discovery::petri_net::petri_net::DefaultPetriNet;
use ficus::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use ficus::pipelines::keys::context_keys::*;
use ficus::pipelines::patterns_parts::PatternsKindDto;
use ficus::utils::colors::ColorsEventLog;
use ficus::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use ficus::utils::distance::distance::FicusDistance;
use ficus::utils::graph::graph::DefaultGraph;
use ficus::utils::log_serialization_format::LogSerializationFormat;
use ficus::{
    event_log::{core::event_log::EventLog, xes::xes_event_log::XesEventLogImpl},
    features::analysis::patterns::{activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy},
    pipelines::{
        aliases::{Activities, ActivitiesToLogs, Patterns, RepeatSets, TracesActivities},
        pipelines::Pipeline,
    },
    utils::{
        colors::ColorsHolder,
        user_data::{keys::Key, user_data::UserData},
    },
    vecs,
};
use ficus::features::analysis::threads_diagram::discovery::LogThreadsDiagram;

#[test]
#[rustfmt::skip]
fn test_event_log_all_concrete_keys() {
    let mut used = HashSet::new();

    assert_existence::<String>(&PATH, &mut used);
    assert_existence::<u32>(&TANDEM_ARRAY_LENGTH, &mut used);
    assert_existence::<u32>(&ACTIVITY_LEVEL, &mut used);
    assert_existence::<ActivityNarrowingKind>(&NARROW_ACTIVITIES, &mut used);
    assert_existence::<String>(&EVENT_NAME, &mut used);
    assert_existence::<String>(&REGEX, &mut used);
    assert_existence::<PatternsDiscoveryStrategy>(&PATTERNS_DISCOVERY_STRATEGY, &mut used);
    assert_existence::<String>(&OUTPUT_STRING, &mut used);
    assert_existence::<OfflineEventLogInfo>(&EVENT_LOG_INFO, &mut used);
    assert_existence::<usize>(&UNDERLYING_EVENTS_COUNT, &mut used);
    assert_existence::<u32>(&EVENTS_COUNT, &mut used);
    assert_existence::<Vec<String>>(&EVENT_CLASSES_REGEXES, &mut used);
    assert_existence::<AdjustingMode>(&ADJUSTING_MODE, &mut used);
    assert_existence::<String>(&EVENT_CLASS_REGEX, &mut used);
    assert_existence::<PatternsKindDto>(&PATTERNS_KIND, &mut used);
    assert_existence::<Pipeline>(&PIPELINE, &mut used);
    assert_existence::<u32>(&MIN_ACTIVITY_LENGTH, &mut used);
    assert_existence::<UndefActivityHandlingStrategyDto>(&UNDEF_ACTIVITY_HANDLING_STRATEGY, &mut used);
    assert_existence::<ActivityInTraceFilterKind>(&ACTIVITY_IN_TRACE_FILTER_KIND, &mut used);
    assert_existence::<ActivitiesLogsSourceDto>(&ACTIVITIES_LOGS_SOURCE, &mut used);
    assert_existence::<bool>(&PNML_USE_NAMES_AS_IDS, &mut used);
    assert_existence::<f64>(&DEPENDENCY_THRESHOLD, &mut used);
    assert_existence::<u32>(&POSITIVE_OBSERVATIONS_THRESHOLD, &mut used);
    assert_existence::<f64>(&RELATIVE_TO_BEST_THRESHOLD, &mut used);
    assert_existence::<f64>(&AND_THRESHOLD, &mut used);
    assert_existence::<f64>(&LOOP_LENGTH_TWO_THRESHOLD, &mut used);
    assert_existence::<f64>(&UNARY_FREQUENCY_THRESHOLD, &mut used);
    assert_existence::<f64>(&BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD, &mut used);
    assert_existence::<f64>(&PRESERVE_THRESHOLD, &mut used);
    assert_existence::<f64>(&RATIO_THRESHOLD, &mut used);
    assert_existence::<f64>(&UTILITY_RATE, &mut used);
    assert_existence::<f64>(&EDGE_CUTOFF_THRESHOLD, &mut used);
    assert_existence::<f64>(&NODE_CUTOFF_THRESHOLD, &mut used);
    assert_existence::<String>(&START_CASE_REGEX_STR, &mut used);
    assert_existence::<String>(&END_CASE_REGEX_STR, &mut used);
    assert_existence::<bool>(&INLINE_INNER_CASES_STR, &mut used);

    assert_existence::<XesEventLogImpl>(&EVENT_LOG, &mut used);
    assert_existence::<Activities>(&ACTIVITIES, &mut used);
    assert_existence::<ActivitiesToLogs>(&ACTIVITIES_TO_LOGS, &mut used);
    assert_existence::<String>(&ACTIVITY_NAME, &mut used);
    assert_existence::<Patterns>(&PATTERNS, &mut used);
    assert_existence::<Vec<Vec<u64>>>(&HASHES_EVENT_LOG, &mut used);
    assert_existence::<Vec<Vec<String>>>(&NAMES_EVENT_LOG, &mut used);
    assert_existence::<DefaultPetriNet>(&PETRI_NET, &mut used);
    assert_existence::<RepeatSets>(&REPEAT_SETS, &mut used);
    assert_existence::<TracesActivities>(&TRACE_ACTIVITIES, &mut used);
    assert_existence::<ColorsEventLog>(&COLORS_EVENT_LOG, &mut used);
    assert_existence::<ColorsHolder>(&COLORS_HOLDER, &mut used);
    assert_existence::<DefaultGraph>(&GRAPH, &mut used);

    assert_existence::<HashMap<u64, usize>>(&PETRI_NET_COUNT_ANNOTATION, &mut used);
    assert_existence::<HashMap<u64, f64>>(&PETRI_NET_FREQUENCY_ANNOTATION, &mut used);
    assert_existence::<HashMap<u64, f64>>(&PETRI_NET_TRACE_FREQUENCY_ANNOTATION, &mut used);
    assert_existence::<bool>(&TERMINATE_ON_UNREPLAYABLE_TRACES, &mut used);
    assert_existence::<u32>(&CLUSTERS_COUNT, &mut used);
    assert_existence::<u32>(&LEARNING_ITERATIONS_COUNT, &mut used);
    assert_existence::<f64>(&TOLERANCE, &mut used);
    assert_existence::<u32>(&MIN_EVENTS_IN_CLUSTERS_COUNT, &mut used);
    assert_existence::<FicusDataset>(&TRACES_ACTIVITIES_DATASET, &mut used);
    assert_existence::<LabeledDataset>(&LABELED_TRACES_ACTIVITIES_DATASET, &mut used);
    assert_existence::<ActivityRepresentationSource>(&ACTIVITIES_REPR_SOURCE, &mut used);
    assert_existence::<FicusDistance>(&DISTANCE, &mut used);
    assert_existence::<bool>(&EXECUTE_ONLY_ON_LAST_EXTRACTION, &mut used);
    assert_existence::<String>(&EVENT_LOG_NAME, &mut used);
    assert_existence::<FicusDataset>(&LOG_TRACES_DATASET, &mut used);
    assert_existence::<LabeledDataset>(&LABELED_LOG_TRACES_DATASET, &mut used);
    assert_existence::<TracesRepresentationSource>(&TRACES_REPR_SOURCE, &mut used);
    assert_existence::<SystemMetadata>(&SYSTEM_METADATA, &mut used);
    assert_existence::<LogSerializationFormat>(&LOG_SERIALIZATION_FORMAT, &mut used);
    assert_existence::<Vec<u8>>(&BYTES, &mut used);
    assert_existence::<HashMap<u64, f64>>(&GRAPH_TIME_ANNOTATION, &mut used);
    assert_existence::<String>(&ATTRIBUTE, &mut used);
    assert_existence::<TimeAnnotationKind>(&TIME_ANNOTATION_KIND, &mut used);
    assert_existence::<Vec<String>>(&ATTRIBUTES, &mut used);
    assert_existence::<Vec<String>>(&PATHS, &mut used);
    assert_existence::<LogThreadsDiagram>(&LOG_THREADS_DIAGRAM, &mut used);

    assert_eq!(used.len(), get_all_keys_names().len())
}

fn assert_existence<T: 'static>(name: &str, used: &mut HashSet<String>) {
    if used.contains(name) {
        assert!(false)
    }

    used.insert(name.to_owned());
    assert!(find_context_key(name).is_some());
}

#[rustfmt::skip]
fn get_all_keys_names() -> Vec<String> {
    vecs![
        "path",
        "tandem_array_length",
        "activity_level",
        "narrow_activities",
        "event_name",
        "regex",
        "patterns_discovery_strategy",
        "output_string",
        "event_log_info",
        "underlying_events_count",
        "events_count",
        "event_classes_regexes",
        "adjusting_mode",
        "event_class_regex",
        "patterns_kind",
        "pipeline",
        "min_activity_length",
        "undef_activity_handling_strategy",
        "activity_in_trace_filter_kind",
        "activities_logs_source",
        "pnml_use_names_as_ids",
        "dependency_threshold",
        "positive_observations_threshold",
        "relative_to_best_threshold",
        "and_threshold",
        "loop_length_two_threshold",
        "unary_frequency_threshold",
        "binary_frequency_significance_threshold",
        "preserve_threshold",
        "ratio_threshold",
        "utility_rate",
        "edge_cutoff_threshold",
        "node_cutoff_threshold",
        "start_case_regex",
        "end_case_regex",
        "inline_inner_cases",

        "event_log",
        "activities",
        "repeat_sets",
        "trace_activities",
        "patterns",
        "petri_net",
        "activities_to_logs",
        "activity_name",
        "hashes_event_log",
        "names_event_log",
        "colors_event_log",
        "colors_holder",
        "graph",
        "petri_net_count_annotation",
        "petri_net_frequency_annotation",
        "petri_net_trace_frequency_annotation",
        "terminate_on_unreplayable_traces",
        "clusters_count",
        "learning_iterations_count",
        "tolerance",
        "min_events_in_cluster_count",
        "traces_activities_dataset",
        "labeled_traces_activities_dataset",
        "activities_repr_source",
        "distance",
        "execute_only_on_last_extraction",
        "event_log_name",
        "log_traces_dataset",
        "labeled_log_traces_dataset",
        "traces_repr_source",
        "system_metadata",
        "log_serialization_format",
        "bytes",
        "graph_time_annotation",
        "attribute",
        "time_annotation_kind",
        "attributes",
        "paths",
        "log_threads_diagram"
    ]
}

#[test]
fn test_event_log_alls() {
    for key_name in get_all_keys_names() {
        assert!(find_context_key(&key_name).is_some());
    }
}

#[test]
#[rustfmt::skip]
fn test_equivalence_of_keys() {
    let mut used = HashSet::new();

    assert_keys_equivalence::<String>(&PATH, &mut used);
    assert_keys_equivalence::<u32>(&TANDEM_ARRAY_LENGTH, &mut used);
    assert_keys_equivalence::<u32>(&ACTIVITY_LEVEL, &mut used);
    assert_keys_equivalence::<ActivityNarrowingKind>(&NARROW_ACTIVITIES, &mut used);
    assert_keys_equivalence::<String>(&EVENT_NAME, &mut used);
    assert_keys_equivalence::<String>(&REGEX, &mut used);
    assert_keys_equivalence::<ColorsEventLog>(&COLORS_EVENT_LOG, &mut used);
    assert_keys_equivalence::<ColorsHolder>(&COLORS_HOLDER, &mut used);
    assert_keys_equivalence::<PatternsDiscoveryStrategy>(&PATTERNS_DISCOVERY_STRATEGY, &mut used);
    assert_keys_equivalence::<String>(&OUTPUT_STRING, &mut used);
    assert_keys_equivalence::<OfflineEventLogInfo>(&EVENT_LOG_INFO, &mut used);
    assert_keys_equivalence::<usize>(&UNDERLYING_EVENTS_COUNT, &mut used);
    assert_keys_equivalence::<u32>(&EVENTS_COUNT, &mut used);
    assert_keys_equivalence::<Vec<String>>(&EVENT_CLASSES_REGEXES, &mut used);
    assert_keys_equivalence::<AdjustingMode>(&ADJUSTING_MODE, &mut used);
    assert_keys_equivalence::<String>(&EVENT_CLASS_REGEX, &mut used);
    assert_keys_equivalence::<PatternsKindDto>(&PATTERNS_KIND, &mut used);
    assert_keys_equivalence::<Pipeline>(&PIPELINE, &mut used);
    assert_keys_equivalence::<u32>(&MIN_ACTIVITY_LENGTH, &mut used);
    assert_keys_equivalence::<UndefActivityHandlingStrategyDto>(&UNDEF_ACTIVITY_HANDLING_STRATEGY, &mut used);
    assert_keys_equivalence::<ActivityInTraceFilterKind>(&ACTIVITY_IN_TRACE_FILTER_KIND, &mut used);
    assert_keys_equivalence::<ActivitiesLogsSourceDto>(&ACTIVITIES_LOGS_SOURCE, &mut used);
    assert_keys_equivalence::<bool>(&PNML_USE_NAMES_AS_IDS, &mut used);
    assert_keys_equivalence::<f64>(&DEPENDENCY_THRESHOLD, &mut used);
    assert_keys_equivalence::<u32>(&POSITIVE_OBSERVATIONS_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&RELATIVE_TO_BEST_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&AND_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&LOOP_LENGTH_TWO_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&UNARY_FREQUENCY_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&PRESERVE_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&RATIO_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&UTILITY_RATE, &mut used);
    assert_keys_equivalence::<f64>(&EDGE_CUTOFF_THRESHOLD, &mut used);
    assert_keys_equivalence::<f64>(&NODE_CUTOFF_THRESHOLD, &mut used);
    assert_keys_equivalence::<String>(&START_CASE_REGEX_STR, &mut used);
    assert_keys_equivalence::<String>(&END_CASE_REGEX_STR, &mut used);
    assert_keys_equivalence::<bool>(&INLINE_INNER_CASES_STR, &mut used);

    assert_keys_equivalence::<XesEventLogImpl>(&EVENT_LOG, &mut used);
    assert_keys_equivalence::<Activities>(&ACTIVITIES, &mut used);
    assert_keys_equivalence::<ActivitiesToLogs>(&ACTIVITIES_TO_LOGS, &mut used);
    assert_keys_equivalence::<String>(&ACTIVITY_NAME, &mut used);
    assert_keys_equivalence::<Patterns>(&PATTERNS, &mut used);
    assert_keys_equivalence::<Vec<Vec<u64>>>(&HASHES_EVENT_LOG, &mut used);
    assert_keys_equivalence::<Vec<Vec<String>>>(&NAMES_EVENT_LOG, &mut used);
    assert_keys_equivalence::<DefaultPetriNet>(&PETRI_NET, &mut used);
    assert_keys_equivalence::<RepeatSets>(&REPEAT_SETS, &mut used);
    assert_keys_equivalence::<TracesActivities>(&TRACE_ACTIVITIES, &mut used);
    assert_keys_equivalence::<DefaultGraph>(&GRAPH, &mut used);

    assert_keys_equivalence::<HashMap<u64, usize>>(&PETRI_NET_COUNT_ANNOTATION, &mut used);
    assert_keys_equivalence::<HashMap<u64, f64>>(&PETRI_NET_FREQUENCY_ANNOTATION, &mut used);
    assert_keys_equivalence::<HashMap<u64, f64>>(&PETRI_NET_TRACE_FREQUENCY_ANNOTATION, &mut used);
    assert_keys_equivalence::<bool>(&TERMINATE_ON_UNREPLAYABLE_TRACES, &mut used);
    assert_keys_equivalence::<u32>(&CLUSTERS_COUNT, &mut used);
    assert_keys_equivalence::<u32>(&LEARNING_ITERATIONS_COUNT, &mut used);
    assert_keys_equivalence::<f64>(&TOLERANCE, &mut used);
    assert_keys_equivalence::<u32>(&MIN_EVENTS_IN_CLUSTERS_COUNT, &mut used);
    assert_keys_equivalence::<FicusDataset>(&TRACES_ACTIVITIES_DATASET, &mut used);
    assert_keys_equivalence::<LabeledDataset>(&LABELED_TRACES_ACTIVITIES_DATASET, &mut used);
    assert_keys_equivalence::<ActivityRepresentationSource>(&ACTIVITIES_REPR_SOURCE, &mut used);
    assert_keys_equivalence::<FicusDistance>(&DISTANCE, &mut used);
    assert_keys_equivalence::<bool>(&EXECUTE_ONLY_ON_LAST_EXTRACTION, &mut used);
    assert_keys_equivalence::<String>(&EVENT_LOG_NAME, &mut used);
    assert_keys_equivalence::<FicusDataset>(&LOG_TRACES_DATASET, &mut used);
    assert_keys_equivalence::<LabeledDataset>(&LABELED_LOG_TRACES_DATASET, &mut used);
    assert_keys_equivalence::<TracesRepresentationSource>(&TRACES_REPR_SOURCE, &mut used);
    assert_keys_equivalence::<SystemMetadata>(&SYSTEM_METADATA, &mut used);
    assert_keys_equivalence::<LogSerializationFormat>(&LOG_SERIALIZATION_FORMAT, &mut used);
    assert_keys_equivalence::<Vec<u8>>(&BYTES, &mut used);
    assert_keys_equivalence::<HashMap<u64, f64>>(&GRAPH_TIME_ANNOTATION, &mut used);
    assert_keys_equivalence::<String>(&ATTRIBUTE, &mut used);
    assert_keys_equivalence::<TimeAnnotationKind>(&TIME_ANNOTATION_KIND, &mut used);
    assert_keys_equivalence::<Vec<String>>(&ATTRIBUTES, &mut used);
    assert_keys_equivalence::<Vec<String>>(&PATHS, &mut used);
    assert_keys_equivalence::<LogThreadsDiagram>(&LOG_THREADS_DIAGRAM, &mut used);

    assert_eq!(used.len(), get_all_keys_names().len())
}

fn assert_keys_equivalence<T: 'static>(name: &str, used: &mut HashSet<String>) {
    if used.contains(name) {
        assert!(false)
    }

    used.insert(name.to_owned());
    assert_eq!(
        find_context_key(name).unwrap().key().id(),
        find_context_key(name).unwrap().key().id()
    );
}
