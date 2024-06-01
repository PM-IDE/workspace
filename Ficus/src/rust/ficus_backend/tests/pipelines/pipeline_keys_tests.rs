use bxes::models::system_models::SystemMetadata;
use std::collections::HashMap;
use std::{collections::HashSet, sync::Arc};

use ficus_backend::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use ficus_backend::features::clustering::activities::activities_params::ActivityRepresentationSource;
use ficus_backend::features::clustering::traces::traces_params::TracesRepresentationSource;
use ficus_backend::features::discovery::petri_net::petri_net::DefaultPetriNet;
use ficus_backend::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use ficus_backend::pipelines::patterns_parts::PatternsKindDto;
use ficus_backend::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use ficus_backend::utils::distance::distance::FicusDistance;
use ficus_backend::utils::graph::graph::DefaultGraph;
use ficus_backend::{
    event_log::{core::event_log::EventLog, xes::xes_event_log::XesEventLogImpl},
    features::analysis::{
        event_log_info::EventLogInfo,
        patterns::{activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy},
    },
    pipelines::{
        aliases::{Activities, ActivitiesToLogs, ColorsEventLog, Patterns, RepeatSets, TracesActivities},
        context::PipelineContext,
        keys::context_keys::ContextKeys,
        pipelines::Pipeline,
    },
    utils::{
        colors::ColorsHolder,
        user_data::{keys::Key, user_data::UserData},
    },
    vecs,
};

#[test]
fn test_event_log_key() {
    execute_test(|keys, context| {
        let log_key = keys.event_log();
        let log = XesEventLogImpl::empty();

        assert!(context.concrete(log_key.key()).is_none());

        context.put_concrete(log_key.key(), log);

        assert!(context.concrete(log_key.key()).is_some())
    })
}

fn execute_test(test: impl Fn(&ContextKeys, &mut PipelineContext) -> ()) {
    let keys = Arc::new(Box::new(ContextKeys::new()));
    let mut context = PipelineContext::empty();

    test(&keys, &mut context);
}

#[test]
#[rustfmt::skip]
fn test_event_log_all_concrete_keys() {
    execute_test(|keys, _| {
        let mut used = HashSet::new();

        assert_existence::<String>(keys, ContextKeys::PATH, &mut used);
        assert_existence::<u32>(keys, ContextKeys::TANDEM_ARRAY_LENGTH, &mut used);
        assert_existence::<u32>(keys, ContextKeys::ACTIVITY_LEVEL, &mut used);
        assert_existence::<ActivityNarrowingKind>(keys, ContextKeys::NARROW_ACTIVITIES, &mut used);
        assert_existence::<String>(keys, ContextKeys::EVENT_NAME, &mut used);
        assert_existence::<String>(keys, ContextKeys::REGEX, &mut used);
        assert_existence::<PatternsDiscoveryStrategy>(keys, ContextKeys::PATTERNS_DISCOVERY_STRATEGY, &mut used);
        assert_existence::<String>(keys, ContextKeys::OUTPUT_STRING, &mut used);
        assert_existence::<EventLogInfo>(keys, ContextKeys::EVENT_LOG_INFO, &mut used);
        assert_existence::<usize>(keys, ContextKeys::UNDERLYING_EVENTS_COUNT, &mut used);
        assert_existence::<u32>(keys, ContextKeys::EVENTS_COUNT, &mut used);
        assert_existence::<Vec<String>>(keys, ContextKeys::EVENT_CLASSES_REGEXES, &mut used);
        assert_existence::<AdjustingMode>(keys, ContextKeys::ADJUSTING_MODE, &mut used);
        assert_existence::<String>(keys, ContextKeys::EVENT_CLASS_REGEX, &mut used);
        assert_existence::<PatternsKindDto>(keys, ContextKeys::PATTERNS_KIND, &mut used);
        assert_existence::<Pipeline>(keys, ContextKeys::PIPELINE, &mut used);
        assert_existence::<u32>(keys, ContextKeys::MIN_ACTIVITY_LENGTH, &mut used);
        assert_existence::<UndefActivityHandlingStrategyDto>(keys, ContextKeys::UNDEF_ACTIVITY_HANDLING_STRATEGY, &mut used);
        assert_existence::<ActivityInTraceFilterKind>(keys, ContextKeys::ACTIVITY_IN_TRACE_FILTER_KIND, &mut used);
        assert_existence::<ActivitiesLogsSourceDto>(keys, ContextKeys::ACTIVITIES_LOGS_SOURCE, &mut used);
        assert_existence::<bool>(keys, ContextKeys::PNML_USE_NAMES_AS_IDS, &mut used);
        assert_existence::<f64>(keys, ContextKeys::DEPENDENCY_THRESHOLD, &mut used);
        assert_existence::<u32>(keys, ContextKeys::POSITIVE_OBSERVATIONS_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::RELATIVE_TO_BEST_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::AND_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::LOOP_LENGTH_TWO_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::UNARY_FREQUENCY_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::PRESERVE_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::RATIO_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::UTILITY_RATE, &mut used);
        assert_existence::<f64>(keys, ContextKeys::EDGE_CUTOFF_THRESHOLD, &mut used);
        assert_existence::<f64>(keys, ContextKeys::NODE_CUTOFF_THRESHOLD, &mut used);

        assert_existence::<XesEventLogImpl>(keys, ContextKeys::EVENT_LOG, &mut used);
        assert_existence::<Activities>(keys, ContextKeys::ACTIVITIES, &mut used);
        assert_existence::<ActivitiesToLogs>(keys, ContextKeys::ACTIVITIES_TO_LOGS, &mut used);
        assert_existence::<String>(keys, ContextKeys::ACTIVITY_NAME, &mut used);
        assert_existence::<Patterns>(keys, ContextKeys::PATTERNS, &mut used);
        assert_existence::<Vec<Vec<u64>>>(keys, ContextKeys::HASHES_EVENT_LOG, &mut used);
        assert_existence::<Vec<Vec<String>>>(keys, ContextKeys::NAMES_EVENT_LOG, &mut used);
        assert_existence::<DefaultPetriNet>(keys, ContextKeys::PETRI_NET, &mut used);
        assert_existence::<RepeatSets>(keys, ContextKeys::REPEAT_SETS, &mut used);
        assert_existence::<TracesActivities>(keys, ContextKeys::TRACE_ACTIVITIES, &mut used);
        assert_existence::<ColorsEventLog>(keys, ContextKeys::COLORS_EVENT_LOG, &mut used);
        assert_existence::<ColorsHolder>(keys, ContextKeys::COLORS_HOLDER, &mut used);
        assert_existence::<DefaultGraph>(keys, ContextKeys::GRAPH, &mut used);

        assert_existence::<HashMap<u64, usize>>(keys, ContextKeys::PETRI_NET_COUNT_ANNOTATION, &mut used);
        assert_existence::<HashMap<u64, f64>>(keys, ContextKeys::PETRI_NET_FREQUENCY_ANNOTATION, &mut used);
        assert_existence::<HashMap<u64, f64>>(keys, ContextKeys::PETRI_NET_TRACE_FREQUENCY_ANNOTATION, &mut used);
        assert_existence::<bool>(keys, ContextKeys::TERMINATE_ON_UNREPLAYABLE_TRACES, &mut used);
        assert_existence::<u32>(keys, ContextKeys::CLUSTERS_COUNT, &mut used);
        assert_existence::<u32>(keys, ContextKeys::LEARNING_ITERATIONS_COUNT, &mut used);
        assert_existence::<f64>(keys, ContextKeys::TOLERANCE, &mut used);
        assert_existence::<u32>(keys, ContextKeys::MIN_EVENTS_IN_CLUSTERS_COUNT, &mut used);
        assert_existence::<FicusDataset>(keys, ContextKeys::TRACES_ACTIVITIES_DATASET, &mut used);
        assert_existence::<LabeledDataset>(keys, ContextKeys::LABELED_TRACES_ACTIVITIES_DATASET, &mut used);
        assert_existence::<ActivityRepresentationSource>(keys, ContextKeys::ACTIVITIES_REPR_SOURCE, &mut used);
        assert_existence::<FicusDistance>(keys, ContextKeys::DISTANCE, &mut used);
        assert_existence::<bool>(keys, ContextKeys::EXECUTE_ONLY_ON_LAST_EXTRACTION, &mut used);
        assert_existence::<String>(keys, ContextKeys::EVENT_LOG_NAME, &mut used);
        assert_existence::<FicusDataset>(keys, ContextKeys::LOG_TRACES_DATASET, &mut used);
        assert_existence::<LabeledDataset>(keys, ContextKeys::LABELED_LOG_TRACES_DATASET, &mut used);
        assert_existence::<TracesRepresentationSource>(keys, ContextKeys::TRACES_REPR_SOURCE, &mut used);
        assert_existence::<SystemMetadata>(keys, ContextKeys::SYSTEM_METADATA, &mut used);

        assert_eq!(used.len(), get_all_keys_names().len())
    })
}

fn assert_existence<T: 'static>(keys: &ContextKeys, name: &str, used: &mut HashSet<String>) {
    if used.contains(name) {
        assert!(false)
    }

    used.insert(name.to_owned());
    assert!(keys.find_concrete_key::<T>(name).is_some());
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
        "system_metadata"
    ]
}

#[test]
fn test_event_log_all_keys() {
    execute_test(|keys, _| {
        for key_name in get_all_keys_names() {
            assert!(keys.find_key(&key_name).is_some());
        }
    })
}

#[test]
fn test_keys_count() {
    execute_test(|keys, _| assert_eq!(keys.len(), get_all_keys_names().len()))
}

#[test]
#[rustfmt::skip]
fn test_equivalence_of_keys() {
    execute_test(|keys, _| {
        let mut used = HashSet::new();

        assert_keys_equivalence::<String>(keys, ContextKeys::PATH, &mut used);        
        assert_keys_equivalence::<u32>(keys, ContextKeys::TANDEM_ARRAY_LENGTH, &mut used);        
        assert_keys_equivalence::<u32>(keys, ContextKeys::ACTIVITY_LEVEL, &mut used);        
        assert_keys_equivalence::<ActivityNarrowingKind>(keys, ContextKeys::NARROW_ACTIVITIES, &mut used);
        assert_keys_equivalence::<String>(keys, ContextKeys::EVENT_NAME, &mut used);        
        assert_keys_equivalence::<String>(keys, ContextKeys::REGEX, &mut used);        
        assert_keys_equivalence::<ColorsEventLog>(keys, ContextKeys::COLORS_EVENT_LOG, &mut used);        
        assert_keys_equivalence::<ColorsHolder>(keys, ContextKeys::COLORS_HOLDER, &mut used);        
        assert_keys_equivalence::<PatternsDiscoveryStrategy>(keys, ContextKeys::PATTERNS_DISCOVERY_STRATEGY, &mut used);       
        assert_keys_equivalence::<String>(keys, ContextKeys::OUTPUT_STRING, &mut used);         
        assert_keys_equivalence::<EventLogInfo>(keys, ContextKeys::EVENT_LOG_INFO, &mut used);
        assert_keys_equivalence::<usize>(keys, ContextKeys::UNDERLYING_EVENTS_COUNT, &mut used);        
        assert_keys_equivalence::<u32>(keys, ContextKeys::EVENTS_COUNT, &mut used);        
        assert_keys_equivalence::<Vec<String>>(keys, ContextKeys::EVENT_CLASSES_REGEXES, &mut used);        
        assert_keys_equivalence::<AdjustingMode>(keys, ContextKeys::ADJUSTING_MODE, &mut used);        
        assert_keys_equivalence::<String>(keys, ContextKeys::EVENT_CLASS_REGEX, &mut used);        
        assert_keys_equivalence::<PatternsKindDto>(keys, ContextKeys::PATTERNS_KIND, &mut used);
        assert_keys_equivalence::<Pipeline>(keys, ContextKeys::PIPELINE, &mut used);
        assert_keys_equivalence::<u32>(keys, ContextKeys::MIN_ACTIVITY_LENGTH, &mut used);
        assert_keys_equivalence::<UndefActivityHandlingStrategyDto>(keys, ContextKeys::UNDEF_ACTIVITY_HANDLING_STRATEGY, &mut used);
        assert_keys_equivalence::<ActivityInTraceFilterKind>(keys, ContextKeys::ACTIVITY_IN_TRACE_FILTER_KIND, &mut used);
        assert_keys_equivalence::<ActivitiesLogsSourceDto>(keys, ContextKeys::ACTIVITIES_LOGS_SOURCE, &mut used);
        assert_keys_equivalence::<bool>(keys, ContextKeys::PNML_USE_NAMES_AS_IDS, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::DEPENDENCY_THRESHOLD, &mut used);
        assert_keys_equivalence::<u32>(keys, ContextKeys::POSITIVE_OBSERVATIONS_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::RELATIVE_TO_BEST_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::AND_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::LOOP_LENGTH_TWO_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::UNARY_FREQUENCY_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::PRESERVE_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::RATIO_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::UTILITY_RATE, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::EDGE_CUTOFF_THRESHOLD, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::NODE_CUTOFF_THRESHOLD, &mut used);

        assert_keys_equivalence::<XesEventLogImpl>(keys, ContextKeys::EVENT_LOG, &mut used);
        assert_keys_equivalence::<Activities>(keys, ContextKeys::ACTIVITIES, &mut used);
        assert_keys_equivalence::<ActivitiesToLogs>(keys, ContextKeys::ACTIVITIES_TO_LOGS, &mut used);        
        assert_keys_equivalence::<String>(keys, ContextKeys::ACTIVITY_NAME, &mut used);        
        assert_keys_equivalence::<Patterns>(keys, ContextKeys::PATTERNS, &mut used);        
        assert_keys_equivalence::<Vec<Vec<u64>>>(keys, ContextKeys::HASHES_EVENT_LOG, &mut used);        
        assert_keys_equivalence::<Vec<Vec<String>>>(keys, ContextKeys::NAMES_EVENT_LOG, &mut used);        
        assert_keys_equivalence::<DefaultPetriNet>(keys, ContextKeys::PETRI_NET, &mut used);
        assert_keys_equivalence::<RepeatSets>(keys, ContextKeys::REPEAT_SETS, &mut used);        
        assert_keys_equivalence::<TracesActivities>(keys, ContextKeys::TRACE_ACTIVITIES, &mut used);        
        assert_keys_equivalence::<DefaultGraph>(keys, ContextKeys::GRAPH, &mut used);

        assert_keys_equivalence::<HashMap<u64, usize>>(keys, ContextKeys::PETRI_NET_COUNT_ANNOTATION, &mut used);
        assert_keys_equivalence::<HashMap<u64, f64>>(keys, ContextKeys::PETRI_NET_FREQUENCY_ANNOTATION, &mut used);
        assert_keys_equivalence::<HashMap<u64, f64>>(keys, ContextKeys::PETRI_NET_TRACE_FREQUENCY_ANNOTATION, &mut used);
        assert_keys_equivalence::<bool>(keys, ContextKeys::TERMINATE_ON_UNREPLAYABLE_TRACES, &mut used);
        assert_keys_equivalence::<u32>(keys, ContextKeys::CLUSTERS_COUNT, &mut used);
        assert_keys_equivalence::<u32>(keys, ContextKeys::LEARNING_ITERATIONS_COUNT, &mut used);
        assert_keys_equivalence::<f64>(keys, ContextKeys::TOLERANCE, &mut used);
        assert_keys_equivalence::<u32>(keys, ContextKeys::MIN_EVENTS_IN_CLUSTERS_COUNT, &mut used);
        assert_keys_equivalence::<FicusDataset>(keys, ContextKeys::TRACES_ACTIVITIES_DATASET, &mut used);
        assert_keys_equivalence::<LabeledDataset>(keys, ContextKeys::LABELED_TRACES_ACTIVITIES_DATASET, &mut used);
        assert_keys_equivalence::<ActivityRepresentationSource>(keys, ContextKeys::ACTIVITIES_REPR_SOURCE, &mut used);
        assert_keys_equivalence::<FicusDistance>(keys, ContextKeys::DISTANCE, &mut used);
        assert_keys_equivalence::<bool>(keys, ContextKeys::EXECUTE_ONLY_ON_LAST_EXTRACTION, &mut used);
        assert_keys_equivalence::<String>(keys, ContextKeys::EVENT_LOG_NAME, &mut used);
        assert_keys_equivalence::<FicusDataset>(keys, ContextKeys::LOG_TRACES_DATASET, &mut used);
        assert_keys_equivalence::<LabeledDataset>(keys, ContextKeys::LABELED_LOG_TRACES_DATASET, &mut used);
        assert_keys_equivalence::<TracesRepresentationSource>(keys, ContextKeys::TRACES_REPR_SOURCE, &mut used);
        assert_keys_equivalence::<SystemMetadata>(keys, ContextKeys::SYSTEM_METADATA, &mut used);

        assert_eq!(used.len(), get_all_keys_names().len())
    })
}

fn assert_keys_equivalence<T: 'static>(keys: &ContextKeys, name: &str, used: &mut HashSet<String>) {
    if used.contains(name) {
        assert!(false)
    }

    used.insert(name.to_owned());
    assert_eq!(
        keys.find_key(name).unwrap().key().id(),
        keys.find_concrete_key::<T>(name).unwrap().key().id()
    );
}
