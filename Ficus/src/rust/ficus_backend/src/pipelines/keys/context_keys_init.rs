use std::{any::Any, borrow::Cow, collections::HashMap};

use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::clustering::activities::activities_params::ActivityRepresentationSource;
use crate::features::clustering::traces::traces_params::TracesRepresentationSource;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use crate::utils::distance::distance::FicusDistance;
use crate::utils::graph::graph::DefaultGraph;
use crate::{
    event_log::xes::xes_event_log::XesEventLogImpl,
    features::analysis::{
        event_log_info::EventLogInfo,
        patterns::{activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy},
    },
    pipelines::{aliases::*, pipelines::Pipeline},
    utils::colors::ColorsHolder,
};

use super::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys::ContextKeys,
};

pub(super) type ConcreteKeysStorage = HashMap<Cow<'static, str>, Box<dyn Any>>;
pub(super) type ContextKeysStorage = HashMap<Cow<'static, str>, Box<dyn ContextKey>>;

struct ContextKeysInitContext {
    concrete_keys: ConcreteKeysStorage,
    context_keys: ContextKeysStorage,
}

impl ContextKeysInitContext {
    fn empty() -> Self {
        Self {
            concrete_keys: ConcreteKeysStorage::new(),
            context_keys: ContextKeysStorage::new(),
        }
    }

    fn insert<T>(&mut self, name: &'static str, key: &Box<DefaultContextKey<T>>) {
        self.insert_concrete(name, key.clone());
        self.insert_context(name, key.clone());
    }

    fn insert_concrete<T>(&mut self, name: &'static str, key: Box<DefaultContextKey<T>>) {
        let prev = self.context_keys.insert(Cow::Borrowed(name), key.clone());
        assert!(prev.is_none());
    }

    fn insert_context<T>(&mut self, name: &'static str, key: Box<DefaultContextKey<T>>) {
        let prev = self.concrete_keys.insert(Cow::Borrowed(name), key.clone());
        assert!(prev.is_none());
    }

    fn deconstruct(self) -> (ConcreteKeysStorage, ContextKeysStorage) {
        (self.concrete_keys, self.context_keys)
    }
}

impl ContextKeys {
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

    pub fn new() -> Self {
        let mut context = ContextKeysInitContext::empty();

        Self::insert_path(&mut context);
        Self::insert_tandem_arrays_length(&mut context);
        Self::insert_activity_level(&mut context);
        Self::insert_narrow_activities(&mut context);
        Self::insert_event_name(&mut context);
        Self::insert_regex(&mut context);
        Self::insert_patterns_discovery_strategy(&mut context);
        Self::insert_output_string(&mut context);
        Self::insert_event_log_info(&mut context);
        Self::insert_underlying_events_count(&mut context);
        Self::insert_events_count(&mut context);
        Self::insert_event_classes_regexes(&mut context);
        Self::insert_adjusting_mode(&mut context);
        Self::insert_event_class_regex(&mut context);
        Self::insert_patterns_kind(&mut context);
        Self::insert_pipeline(&mut context);
        Self::insert_min_activity_length(&mut context);
        Self::insert_undef_activity_handling_strategy(&mut context);
        Self::insert_activity_in_filter_kind(&mut context);
        Self::insert_activities_logs_source(&mut context);
        Self::insert_pnml_use_names_as_ids(&mut context);
        Self::insert_dependency_threshold(&mut context);
        Self::insert_positive_observations_threshold(&mut context);
        Self::insert_relative_to_best_threshold(&mut context);
        Self::insert_and_threshold(&mut context);
        Self::insert_loop_length_two_threshold(&mut context);
        Self::insert_unary_frequency_threshold(&mut context);
        Self::insert_binary_significance_threshold(&mut context);
        Self::insert_preserve_threshold(&mut context);
        Self::insert_ratio_threshold(&mut context);
        Self::insert_utility_rate(&mut context);
        Self::insert_edge_cutoff_threshold(&mut context);
        Self::insert_node_cutoff_threshold(&mut context);

        Self::insert_event_log(&mut context);
        Self::insert_activities(&mut context);
        Self::insert_repeat_sets(&mut context);
        Self::insert_trace_activities(&mut context);
        Self::insert_patterns(&mut context);
        Self::insert_petri_net(&mut context);
        Self::insert_activities_to_logs(&mut context);
        Self::insert_activity_name(&mut context);

        Self::insert_hashes_event_log(&mut context);
        Self::insert_names_event_log(&mut context);
        Self::insert_colors_event_log(&mut context);
        Self::insert_colors_holder(&mut context);
        Self::insert_graph(&mut context);

        Self::insert_petri_net_frequency_annotation(&mut context);
        Self::insert_petri_net_count_annotation(&mut context);
        Self::insert_petri_net_trace_frequency_annotation(&mut context);
        Self::insert_terminate_on_unreplayable_traces(&mut context);
        Self::insert_clusters_count(&mut context);
        Self::insert_learning_iterations_count(&mut context);
        Self::insert_tolerance(&mut context);
        Self::insert_min_clusters_count(&mut context);
        Self::insert_traces_activities_dataset(&mut context);
        Self::insert_labeled_traces_activities(&mut context);
        Self::insert_obtain_activities_repr_from_traces(&mut context);
        Self::insert_distance(&mut context);
        Self::insert_execute_only_on_last_extraction(&mut context);
        Self::insert_event_log_name(&mut context);
        Self::insert_log_traces_dataset(&mut context);
        Self::insert_labeled_log_traces_dataset(&mut context);
        Self::insert_traces_repr_source(&mut context);

        let (concrete_keys, context_keys) = context.deconstruct();

        Self {
            concrete_keys,
            context_keys,
        }
    }

    fn insert_path(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::PATH);
    }

    fn insert_key<T: 'static>(context: &mut ContextKeysInitContext, name: &'static str) {
        let key = Box::new(DefaultContextKey::<T>::new(name));
        Self::insert_key_to_map(context, key, name);
    }

    fn insert_key_to_map<T: 'static>(context: &mut ContextKeysInitContext, key: Box<DefaultContextKey<T>>, name: &'static str) {
        context.insert(name, &key);
    }

    fn insert_tandem_arrays_length(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::TANDEM_ARRAY_LENGTH);
    }

    fn insert_activity_level(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::ACTIVITY_LEVEL);
    }

    fn insert_narrow_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ActivityNarrowingKind>(context, Self::NARROW_ACTIVITIES);
    }

    fn insert_event_name(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::EVENT_NAME);
    }

    fn insert_regex(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::REGEX);
    }

    fn insert_patterns_discovery_strategy(context: &mut ContextKeysInitContext) {
        Self::insert_key::<PatternsDiscoveryStrategy>(context, Self::PATTERNS_DISCOVERY_STRATEGY);
    }

    fn insert_output_string(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::OUTPUT_STRING);
    }

    fn insert_event_log_info(context: &mut ContextKeysInitContext) {
        Self::insert_key::<EventLogInfo>(context, Self::EVENT_LOG_INFO);
    }

    fn insert_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<XesEventLogImpl>(context, Self::EVENT_LOG);
    }

    fn insert_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Activities>(context, Self::ACTIVITIES);
    }

    fn insert_repeat_sets(context: &mut ContextKeysInitContext) {
        Self::insert_key::<RepeatSets>(context, Self::REPEAT_SETS);
    }

    fn insert_trace_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<TracesActivities>(context, Self::TRACE_ACTIVITIES);
    }

    fn insert_patterns(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Patterns>(context, Self::PATTERNS);
    }

    fn insert_petri_net(context: &mut ContextKeysInitContext) {
        Self::insert_key::<DefaultPetriNet>(context, Self::PETRI_NET);
    }

    fn insert_activities_to_logs(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ActivitiesToLogs>(context, Self::ACTIVITIES_TO_LOGS);
    }

    fn insert_activity_name(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::ACTIVITY_NAME);
    }

    fn insert_hashes_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Vec<Vec<u64>>>(context, Self::HASHES_EVENT_LOG);
    }

    fn insert_names_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Vec<Vec<String>>>(context, Self::NAMES_EVENT_LOG);
    }

    fn insert_colors_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ColorsEventLog>(context, Self::COLORS_EVENT_LOG);
    }

    fn insert_colors_holder(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ColorsHolder>(context, Self::COLORS_HOLDER);
    }

    fn insert_underlying_events_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<usize>(context, Self::UNDERLYING_EVENTS_COUNT);
    }

    fn insert_events_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::EVENTS_COUNT);
    }

    fn insert_event_classes_regexes(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Vec<String>>(context, Self::EVENT_CLASSES_REGEXES);
    }

    fn insert_adjusting_mode(context: &mut ContextKeysInitContext) {
        Self::insert_key::<AdjustingMode>(context, Self::ADJUSTING_MODE)
    }

    fn insert_event_class_regex(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::EVENT_CLASS_REGEX)
    }

    fn insert_patterns_kind(context: &mut ContextKeysInitContext) {
        Self::insert_key::<PatternsKindDto>(context, Self::PATTERNS_KIND)
    }

    fn insert_pipeline(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Pipeline>(context, Self::PIPELINE)
    }

    fn insert_min_activity_length(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::MIN_ACTIVITY_LENGTH)
    }

    fn insert_undef_activity_handling_strategy(context: &mut ContextKeysInitContext) {
        Self::insert_key::<UndefActivityHandlingStrategyDto>(context, Self::UNDEF_ACTIVITY_HANDLING_STRATEGY)
    }

    fn insert_activity_in_filter_kind(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ActivityInTraceFilterKind>(context, Self::ACTIVITY_IN_TRACE_FILTER_KIND)
    }

    fn insert_activities_logs_source(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ActivitiesLogsSourceDto>(context, Self::ACTIVITIES_LOGS_SOURCE);
    }

    fn insert_pnml_use_names_as_ids(context: &mut ContextKeysInitContext) {
        Self::insert_key::<bool>(context, Self::PNML_USE_NAMES_AS_IDS);
    }

    fn insert_graph(context: &mut ContextKeysInitContext) {
        Self::insert_key::<DefaultGraph>(context, Self::GRAPH);
    }

    fn insert_dependency_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::DEPENDENCY_THRESHOLD)
    }

    fn insert_positive_observations_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::POSITIVE_OBSERVATIONS_THRESHOLD)
    }

    fn insert_relative_to_best_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::RELATIVE_TO_BEST_THRESHOLD)
    }

    fn insert_and_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::AND_THRESHOLD)
    }

    fn insert_loop_length_two_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::LOOP_LENGTH_TWO_THRESHOLD)
    }

    fn insert_unary_frequency_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::UNARY_FREQUENCY_THRESHOLD);
    }

    fn insert_binary_significance_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD);
    }

    fn insert_preserve_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::PRESERVE_THRESHOLD);
    }

    fn insert_ratio_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::RATIO_THRESHOLD);
    }

    fn insert_utility_rate(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::UTILITY_RATE)
    }

    fn insert_edge_cutoff_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::EDGE_CUTOFF_THRESHOLD)
    }

    fn insert_node_cutoff_threshold(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::NODE_CUTOFF_THRESHOLD)
    }

    fn insert_petri_net_count_annotation(context: &mut ContextKeysInitContext) {
        Self::insert_key::<HashMap<u64, usize>>(context, Self::PETRI_NET_COUNT_ANNOTATION)
    }

    fn insert_petri_net_frequency_annotation(context: &mut ContextKeysInitContext) {
        Self::insert_key::<HashMap<u64, f64>>(context, Self::PETRI_NET_FREQUENCY_ANNOTATION)
    }

    fn insert_petri_net_trace_frequency_annotation(context: &mut ContextKeysInitContext) {
        Self::insert_key::<HashMap<u64, f64>>(context, Self::PETRI_NET_TRACE_FREQUENCY_ANNOTATION)
    }

    fn insert_terminate_on_unreplayable_traces(context: &mut ContextKeysInitContext) {
        Self::insert_key::<bool>(context, Self::TERMINATE_ON_UNREPLAYABLE_TRACES)
    }

    fn insert_clusters_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::CLUSTERS_COUNT)
    }

    fn insert_learning_iterations_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::LEARNING_ITERATIONS_COUNT)
    }

    fn insert_tolerance(context: &mut ContextKeysInitContext) {
        Self::insert_key::<f64>(context, Self::TOLERANCE)
    }

    fn insert_min_clusters_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::MIN_EVENTS_IN_CLUSTERS_COUNT)
    }

    fn insert_traces_activities_dataset(context: &mut ContextKeysInitContext) {
        Self::insert_key::<FicusDataset>(context, Self::TRACES_ACTIVITIES_DATASET)
    }

    fn insert_labeled_traces_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<LabeledDataset>(context, Self::LABELED_TRACES_ACTIVITIES_DATASET)
    }

    fn insert_obtain_activities_repr_from_traces(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ActivityRepresentationSource>(context, Self::ACTIVITIES_REPR_SOURCE)
    }

    fn insert_distance(context: &mut ContextKeysInitContext) {
        Self::insert_key::<FicusDistance>(context, Self::DISTANCE)
    }

    fn insert_execute_only_on_last_extraction(context: &mut ContextKeysInitContext) {
        Self::insert_key::<bool>(context, Self::EXECUTE_ONLY_ON_LAST_EXTRACTION)
    }

    fn insert_event_log_name(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::EVENT_LOG_NAME)
    }

    fn insert_log_traces_dataset(context: &mut ContextKeysInitContext) {
        Self::insert_key::<FicusDataset>(context, Self::LOG_TRACES_DATASET)
    }

    fn insert_labeled_log_traces_dataset(context: &mut ContextKeysInitContext) {
        Self::insert_key::<LabeledDataset>(context, Self::LABELED_LOG_TRACES_DATASET)
    }

    fn insert_traces_repr_source(context: &mut ContextKeysInitContext) {
        Self::insert_key::<TracesRepresentationSource>(context, Self::TRACES_REPR_SOURCE)
    }
}
