use bxes::models::system_models::SystemMetadata;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::clustering::activities::activities_params::ActivityRepresentationSource;
use crate::features::clustering::traces::traces_params::TracesRepresentationSource;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::utils::colors::ColorsEventLog;
use crate::utils::dataset::dataset::{FicusDataset, LabeledDataset};
use crate::utils::distance::distance::FicusDistance;
use crate::utils::graph::graph::DefaultGraph;
use crate::utils::log_serialization_format::LogSerializationFormat;
use crate::{
    event_log::xes::xes_event_log::XesEventLogImpl,
    features::analysis::{
        event_log_info::EventLogInfo,
        patterns::{
            activity_instances::{ActivityInTraceInfo, AdjustingMode},
            contexts::PatternsDiscoveryStrategy,
            repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
            tandem_arrays::SubArrayInTraceInfo,
        },
    },
    pipelines::{aliases::*, pipelines::Pipeline},
    utils::colors::ColorsHolder,
};

use super::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys_init::{ConcreteKeysStorage, ContextKeysStorage},
};

pub struct ContextKeys {
    pub(super) concrete_keys: ConcreteKeysStorage,
    pub(super) context_keys: ContextKeysStorage,
}

unsafe impl Sync for ContextKeys {}
unsafe impl Send for ContextKeys {}

impl ContextKeys {
    pub fn len(&self) -> usize {
        self.concrete_keys.len()
    }

    pub fn find_key(&self, name: &str) -> Option<&Box<dyn ContextKey>> {
        self.context_keys.get(name)
    }

    pub fn find_concrete_key<T: 'static>(&self, name: &str) -> Option<&DefaultContextKey<T>> {
        match self.concrete_keys.get(name) {
            Some(key) => Some(key.downcast_ref::<DefaultContextKey<T>>().unwrap()),
            None => None,
        }
    }

    pub fn path(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::PATH)
            .expect("PATH should be present in keys")
    }

    pub fn is_path(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.path())
    }

    fn are_keys_equal(first: &dyn ContextKey, second: &dyn ContextKey) -> bool {
        first.key().id() == second.key().id()
    }

    pub fn event_log(&self) -> &DefaultContextKey<XesEventLogImpl> {
        self.find_concrete_key::<XesEventLogImpl>(Self::EVENT_LOG)
            .expect("EVENT_LOG should be present in keys")
    }

    pub fn is_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.event_log())
    }

    pub fn activities(&self) -> &DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>> {
        self.find_concrete_key::<Activities>(Self::ACTIVITIES)
            .expect("ACTIVITIES should be present in keys")
    }

    pub fn is_activities(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activities())
    }

    pub fn repeat_sets(&self) -> &DefaultContextKey<Vec<SubArrayWithTraceIndex>> {
        self.find_concrete_key::<RepeatSets>(Self::REPEAT_SETS)
            .expect("REPEAT_SETS should be present in keys")
    }

    pub fn is_repeat_sets(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.repeat_sets())
    }

    pub fn trace_activities(&self) -> &DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>> {
        self.find_concrete_key::<TracesActivities>(Self::TRACE_ACTIVITIES)
            .expect("TRACE_ACTIVITIES should be present in keys")
    }

    pub fn is_trace_activities(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.trace_activities())
    }

    pub fn patterns(&self) -> &DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>> {
        self.find_concrete_key::<Patterns>(Self::PATTERNS)
            .expect("PATTERNS should be present in keys")
    }

    pub fn is_patterns(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.patterns())
    }

    pub fn petri_net(&self) -> &DefaultContextKey<DefaultPetriNet> {
        self.find_concrete_key::<DefaultPetriNet>(Self::PETRI_NET)
            .expect("PETRI_NET should be present in keys")
    }

    pub fn is_petri_net(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.petri_net())
    }

    pub fn activities_to_logs(&self) -> &DefaultContextKey<HashMap<String, XesEventLogImpl>> {
        self.find_concrete_key::<ActivitiesToLogs>(Self::ACTIVITIES_TO_LOGS)
            .expect("ACTIVITIES_TO_LOGS should be present in keys")
    }

    pub fn is_activities_to_logs(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activities_to_logs())
    }

    pub fn activity_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::ACTIVITY_NAME)
            .expect("ACTIVITY_NAME should be present in keys")
    }

    pub fn is_activity_name(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activity_name())
    }

    pub fn hashes_event_log(&self) -> &DefaultContextKey<Vec<Vec<u64>>> {
        self.find_concrete_key::<Vec<Vec<u64>>>(Self::HASHES_EVENT_LOG)
            .expect("HASHES_EVENT_LOG should be present in keys")
    }

    pub fn is_hashes_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.hashes_event_log())
    }

    pub fn names_event_log(&self) -> &DefaultContextKey<Vec<Vec<String>>> {
        self.find_concrete_key::<Vec<Vec<String>>>(Self::NAMES_EVENT_LOG)
            .expect("NAMES_EVENT_LOG should be present in keys")
    }

    pub fn is_names_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.names_event_log())
    }

    pub fn tandem_array_length(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::TANDEM_ARRAY_LENGTH)
            .expect("TANDEM_ARRAY_LENGTH should be present in keys")
    }

    pub fn is_tandem_array_length(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.tandem_array_length())
    }

    pub fn activity_level(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::ACTIVITY_LEVEL)
            .expect("ACTIVITY_LEVEL should be present in keys")
    }

    pub fn is_activity_level(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activity_level())
    }

    pub fn narrow_activities(&self) -> &DefaultContextKey<ActivityNarrowingKind> {
        self.find_concrete_key::<ActivityNarrowingKind>(Self::NARROW_ACTIVITIES)
            .expect("NARROW_ACTIVITIES should be present in keys")
    }

    pub fn is_narrow_activities(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.narrow_activities())
    }

    pub fn event_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::EVENT_NAME)
            .expect("EVENT_NAME should be present in keys")
    }

    pub fn is_event_name(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.event_name())
    }

    pub fn regex(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::REGEX)
            .expect("REGEX should be present in keys")
    }

    pub fn is_regex(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.regex())
    }

    pub fn colors_event_log(&self) -> &DefaultContextKey<ColorsEventLog> {
        self.find_concrete_key::<ColorsEventLog>(Self::COLORS_EVENT_LOG)
            .expect("COLORS_EVENT_LOG should be present in keys")
    }

    pub fn is_colors_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.colors_event_log())
    }

    pub fn colors_holder(&self) -> &DefaultContextKey<ColorsHolder> {
        self.find_concrete_key::<ColorsHolder>(Self::COLORS_HOLDER)
            .expect("COLORS_HOLDER should be present in keys")
    }

    pub fn is_colors_holder(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.colors_holder())
    }

    pub fn patterns_discovery_strategy(&self) -> &DefaultContextKey<PatternsDiscoveryStrategy> {
        self.find_concrete_key::<PatternsDiscoveryStrategy>(Self::PATTERNS_DISCOVERY_STRATEGY)
            .expect("PATTERNS_DISCOVERY_STRATEGY holder should be present in keys")
    }

    pub fn is_patterns_discovery_strategy(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.patterns_discovery_strategy())
    }

    pub fn output_string(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::OUTPUT_STRING)
            .expect("OUTPUT_STRING should be present in keys")
    }

    pub fn is_output_string(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.output_string(), key)
    }

    pub fn event_log_info(&self) -> &DefaultContextKey<EventLogInfo> {
        self.find_concrete_key::<EventLogInfo>(Self::EVENT_LOG_INFO)
            .expect("EVENT_LOG_INFO should be present in keys")
    }

    pub fn is_event_log_info(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_log_info(), key)
    }

    pub fn underlying_events_count(&self) -> &DefaultContextKey<usize> {
        self.find_concrete_key::<usize>(Self::UNDERLYING_EVENTS_COUNT)
            .expect("UNDERLYING_EVENTS_COUNT should be present in keys")
    }

    pub fn is_underlying_events_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.underlying_events_count(), key)
    }

    pub fn events_count(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::EVENTS_COUNT)
            .expect("EVENTS_COUNT should be present in keys")
    }

    pub fn is_events_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.events_count(), key)
    }

    pub fn event_classes_regexes(&self) -> &DefaultContextKey<Vec<String>> {
        self.find_concrete_key::<Vec<String>>(Self::EVENT_CLASSES_REGEXES)
            .expect("EVENT_CLASSES_REGEXES should be present in keys")
    }

    pub fn is_event_classes_regexes(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_classes_regexes(), key)
    }

    pub fn adjusting_mode(&self) -> &DefaultContextKey<AdjustingMode> {
        self.find_concrete_key::<AdjustingMode>(Self::ADJUSTING_MODE)
            .expect("ADJUSTING_MODE should be present in keys")
    }

    pub fn is_adjusting_mode(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.adjusting_mode(), key)
    }

    pub fn event_class_regex(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::EVENT_CLASS_REGEX)
            .expect("EVENT_CLASS_REGEX should be present in keys")
    }

    pub fn is_vent_class_regex(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_class_regex(), key)
    }

    pub fn patterns_kind(&self) -> &DefaultContextKey<PatternsKindDto> {
        self.find_concrete_key::<PatternsKindDto>(Self::PATTERNS_KIND)
            .expect("PATTERNS_KIND should be present in keys")
    }

    pub fn is_patterns_kind(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.patterns_kind(), key)
    }

    pub fn pipeline(&self) -> &DefaultContextKey<Pipeline> {
        self.find_concrete_key::<Pipeline>(Self::PIPELINE)
            .expect("PIPELINE should be present in keys")
    }

    pub fn is_pipeline(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.pipeline(), key)
    }

    pub fn min_activity_length(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::MIN_ACTIVITY_LENGTH)
            .expect("MIN_ACTIVITY_LENGTH kind should be present in keys")
    }

    pub fn is_min_activity_length(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.min_activity_length(), key)
    }

    pub fn undef_activity_handling_strategy(&self) -> &DefaultContextKey<UndefActivityHandlingStrategyDto> {
        self.find_concrete_key::<UndefActivityHandlingStrategyDto>(Self::UNDEF_ACTIVITY_HANDLING_STRATEGY)
            .expect("UNDEF_ACTIVITY_HANDLING_STRATEGY should be present in keys")
    }

    pub fn is_undef_activity_handling_strategy(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.undef_activity_handling_strategy(), key)
    }

    pub fn activity_filter_kind(&self) -> &DefaultContextKey<ActivityInTraceFilterKind> {
        self.find_concrete_key::<ActivityInTraceFilterKind>(Self::ACTIVITY_IN_TRACE_FILTER_KIND)
            .expect("ACTIVITY_IN_TRACE_FILTER_KIND should be present in keys")
    }

    pub fn is_activity_filter_kind(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.activity_filter_kind(), key)
    }

    pub fn activities_logs_source(&self) -> &DefaultContextKey<ActivitiesLogsSourceDto> {
        self.find_concrete_key::<ActivitiesLogsSourceDto>(Self::ACTIVITIES_LOGS_SOURCE)
            .expect("ACTIVITIES_LOGS_SOURCE should be present in keys")
    }

    pub fn is_activities_logs_source(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.activities_logs_source(), key)
    }

    pub fn pnml_use_names_as_ids(&self) -> &DefaultContextKey<bool> {
        self.find_concrete_key::<bool>(Self::PNML_USE_NAMES_AS_IDS)
            .expect("PNML_USE_NAMES_AS_IDS should be present in keys")
    }

    pub fn is_pnml_use_names_as_ids(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.pnml_use_names_as_ids(), key)
    }

    pub fn graph(&self) -> &DefaultContextKey<DefaultGraph> {
        self.find_concrete_key::<DefaultGraph>(Self::GRAPH)
            .expect("GRAPH should be present in keys")
    }

    pub fn is_graph(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.graph(), key)
    }

    pub fn dependency_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::DEPENDENCY_THRESHOLD)
            .expect("DEPENDENCY_THRESHOLD should be present in keys")
    }

    pub fn is_dependency_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.dependency_threshold(), key)
    }

    pub fn positive_observations_threshold(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::POSITIVE_OBSERVATIONS_THRESHOLD)
            .expect("POSITIVE_OBSERVATIONS_THRESHOLD should be present in keys")
    }

    pub fn is_positive_observations_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.positive_observations_threshold(), key)
    }

    pub fn relative_to_best_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::RELATIVE_TO_BEST_THRESHOLD)
            .expect("RELATIVE_TO_BEST_THRESHOLD should be present in keys")
    }

    pub fn is_relative_to_best_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.relative_to_best_threshold(), key)
    }

    pub fn and_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::AND_THRESHOLD)
            .expect("AND_THRESHOLD should be present in keys")
    }

    pub fn is_and_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.and_threshold(), key)
    }

    pub fn loop_length_two_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::LOOP_LENGTH_TWO_THRESHOLD)
            .expect("LOOP_LENGTH_TWO_THRESHOLD should be present in keys")
    }

    pub fn is_loop_length_two_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.loop_length_two_threshold(), key)
    }

    pub fn unary_frequency_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::UNARY_FREQUENCY_THRESHOLD)
            .expect("UNARY_FREQUENCY_THRESHOLD should be present in keys")
    }

    pub fn is_unary_frequency_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.unary_frequency_threshold(), key)
    }

    pub fn binary_significance_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD)
            .expect("BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD should be present in keys")
    }

    pub fn is_binary_significance_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.binary_significance_threshold(), key)
    }

    pub fn preserve_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::PRESERVE_THRESHOLD)
            .expect("PRESERVE_THRESHOLD should be present in keys")
    }

    pub fn is_preserve_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.preserve_threshold(), key)
    }

    pub fn ratio_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::RATIO_THRESHOLD)
            .expect("RATIO_THRESHOLD should be present in keys")
    }

    pub fn is_ratio_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.ratio_threshold(), key)
    }

    pub fn utility_rate(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::UTILITY_RATE)
            .expect("UTILITY_RATE should be present in keys")
    }

    pub fn is_utility_rate(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.utility_rate(), key)
    }

    pub fn edge_cutoff_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::EDGE_CUTOFF_THRESHOLD)
            .expect("EDGE_CUTOFF_THRESHOLD should be present in keys")
    }

    pub fn is_edge_cutoff_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.edge_cutoff_threshold(), key)
    }

    pub fn node_cutoff_threshold(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::NODE_CUTOFF_THRESHOLD)
            .expect("NODE_CUTOFF_THRESHOLD should be present in keys")
    }

    pub fn is_node_cutoff_threshold(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.node_cutoff_threshold(), key)
    }

    pub fn petri_net_count_annotation(&self) -> &DefaultContextKey<HashMap<u64, usize>> {
        self.find_concrete_key::<HashMap<u64, usize>>(Self::PETRI_NET_COUNT_ANNOTATION)
            .expect("PETRI_NET_COUNT_ANNOTATION should be present in keys")
    }

    pub fn is_petri_net_count_annotation(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.petri_net_count_annotation(), key)
    }

    pub fn petri_net_frequency_annotation(&self) -> &DefaultContextKey<HashMap<u64, f64>> {
        self.find_concrete_key::<HashMap<u64, f64>>(Self::PETRI_NET_FREQUENCY_ANNOTATION)
            .expect("PETRI_NET_FREQUENCY_ANNOTATION should be present in keys")
    }

    pub fn is_petri_net_frequency_annotation(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.petri_net_frequency_annotation(), key)
    }

    pub fn petri_net_trace_frequency_annotation(&self) -> &DefaultContextKey<HashMap<u64, f64>> {
        self.find_concrete_key::<HashMap<u64, f64>>(Self::PETRI_NET_TRACE_FREQUENCY_ANNOTATION)
            .expect("PETRI_NET_TRACE_FREQUENCY_ANNOTATION should be present in keys")
    }

    pub fn is_petri_net_trace_frequency_annotation(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.petri_net_trace_frequency_annotation(), key)
    }

    pub fn terminate_on_unreplayable_traces(&self) -> &DefaultContextKey<bool> {
        self.find_concrete_key::<bool>(Self::TERMINATE_ON_UNREPLAYABLE_TRACES)
            .expect("TERMINATE_ON_UNREPLAYABLE_TRACES should be present in keys")
    }

    pub fn is_terminate_on_unreplayable_traces(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.terminate_on_unreplayable_traces(), key)
    }

    pub fn clusters_count(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::CLUSTERS_COUNT)
            .expect("CLUSTERS_COUNT should be present in keys")
    }

    pub fn is_clusters_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.clusters_count(), key)
    }

    pub fn learning_iterations_count(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::LEARNING_ITERATIONS_COUNT)
            .expect("LEARNING_ITERATIONS_COUNT should be present in keys")
    }

    pub fn is_learning_iterations_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.learning_iterations_count(), key)
    }

    pub fn tolerance(&self) -> &DefaultContextKey<f64> {
        self.find_concrete_key::<f64>(Self::TOLERANCE)
            .expect("TOLERANCE should be present in keys")
    }

    pub fn is_tolerance(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.tolerance(), key)
    }

    pub fn min_events_in_clusters_count(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::MIN_EVENTS_IN_CLUSTERS_COUNT)
            .expect("MIN_EVENTS_IN_CLUSTERS_COUNT should be present in keys")
    }

    pub fn is_min_clusters_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.min_events_in_clusters_count(), key)
    }

    pub fn traces_activities_dataset(&self) -> &DefaultContextKey<FicusDataset> {
        self.find_concrete_key::<FicusDataset>(Self::TRACES_ACTIVITIES_DATASET)
            .expect("TRACES_ACTIVITIES_DATASET should be present in keys")
    }

    pub fn is_traces_activities_dataset(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.traces_activities_dataset(), key)
    }

    pub fn labeled_traces_activities_dataset(&self) -> &DefaultContextKey<LabeledDataset> {
        self.find_concrete_key::<LabeledDataset>(Self::LABELED_TRACES_ACTIVITIES_DATASET)
            .expect("LABELED_TRACES_ACTIVITIES_DATASET should be present in keys")
    }

    pub fn is_labeled_traces_activities_dataset(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.labeled_traces_activities_dataset(), key)
    }

    pub fn activities_repr_source(&self) -> &DefaultContextKey<ActivityRepresentationSource> {
        self.find_concrete_key::<ActivityRepresentationSource>(Self::ACTIVITIES_REPR_SOURCE)
            .expect("ACTIVITIES_REPR_SOURCE should be present in keys")
    }

    pub fn is_activities_repr_source(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.activities_repr_source(), key)
    }

    pub fn distance(&self) -> &DefaultContextKey<FicusDistance> {
        self.find_concrete_key::<FicusDistance>(Self::DISTANCE)
            .expect("DISTANCE should be present in keys")
    }

    pub fn is_distance(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.distance(), key)
    }

    pub fn execute_only_on_last_extraction(&self) -> &DefaultContextKey<bool> {
        self.find_concrete_key::<bool>(Self::EXECUTE_ONLY_ON_LAST_EXTRACTION)
            .expect("EXECUTE_ONLY_ON_LAST_EXTRACTION should be present in keys")
    }

    pub fn is_execute_only_on_last_extraction(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.execute_only_on_last_extraction(), key)
    }

    pub fn event_log_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::EVENT_LOG_NAME)
            .expect("EVENT_LOG_NAME should be present in keys")
    }

    pub fn is_event_log_name(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_log_name(), key)
    }

    pub fn log_traces_dataset(&self) -> &DefaultContextKey<FicusDataset> {
        self.find_concrete_key::<FicusDataset>(Self::LOG_TRACES_DATASET)
            .expect("LOG_TRACES_DATASET should be present in keys")
    }

    pub fn is_log_traces_dataset(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.log_traces_dataset(), key)
    }

    pub fn labeled_log_traces_dataset(&self) -> &DefaultContextKey<LabeledDataset> {
        self.find_concrete_key::<LabeledDataset>(Self::LABELED_LOG_TRACES_DATASET)
            .expect("LABELED_LOG_TRACES_DATASET should be present in keys")
    }

    pub fn is_labeled_log_traces_dataset(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.labeled_log_traces_dataset(), key)
    }

    pub fn traces_representation_source(&self) -> &DefaultContextKey<TracesRepresentationSource> {
        self.find_concrete_key::<TracesRepresentationSource>(Self::TRACES_REPR_SOURCE)
            .expect("TRACES_REPR_SOURCE should be present in keys")
    }

    pub fn is_traces_representation_source(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.traces_representation_source(), key)
    }

    pub fn system_metadata(&self) -> &DefaultContextKey<SystemMetadata> {
        self.find_concrete_key::<SystemMetadata>(Self::SYSTEM_METADATA)
            .expect("SYSTEM_METADATA should be present in keys")
    }

    pub fn is_system_metadata(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.system_metadata(), key)
    }

    pub fn log_serialization_format(&self) -> &DefaultContextKey<LogSerializationFormat> {
        self.find_concrete_key::<LogSerializationFormat>(Self::LOG_SERIALIZATION_FORMAT)
            .expect("SYSTEM_METADATA should be present in keys")
    }

    pub fn is_log_serialization_format(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.log_serialization_format(), key)
    }

    pub fn bytes(&self) -> &DefaultContextKey<Vec<u8>> {
        self.find_concrete_key::<Vec<u8>>(Self::BYTES)
            .expect("BYTES should be present in keys")
    }

    pub fn is_bytes(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.bytes(), key)
    }
}
