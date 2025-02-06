use crate::pipelines::pipeline_parts::PipelineParts;

impl PipelineParts {
    pub const READ_LOG_FROM_XES: &'static str = "ReadLogFromXes";
    pub const WRITE_LOG_TO_XES: &'static str = "WriteLogToXes";
    pub const FIND_PRIMITIVE_TANDEM_ARRAYS: &'static str = "FindPrimitiveTandemArrays";
    pub const FIND_MAXIMAL_TANDEM_ARRAYS: &'static str = "FindMaximalTandemArrays";
    pub const FIND_MAXIMAL_REPEATS: &'static str = "FindMaximalRepeats";
    pub const FIND_SUPER_MAXIMAL_REPEATS: &'static str = "FindSuperMaximalRepeats";
    pub const FIND_NEAR_SUPER_MAXIMAL_REPEATS: &'static str = "FindNearSuperMaximalRepeats";
    pub const DISCOVER_ACTIVITIES: &'static str = "DiscoverActivities";
    pub const DISCOVER_ACTIVITIES_INSTANCES: &'static str = "DiscoverActivitiesInstances";
    pub const CREATE_LOG_FROM_ACTIVITIES: &'static str = "CreateLogFromActivities";
    pub const FILTER_EVENTS_BY_NAME: &'static str = "FilterEventsByName";
    pub const FILTER_EVENTS_BY_REGEX: &'static str = "FilterEventsByRegex";
    pub const REMAIN_EVENTS_BY_REGEX: &'static str = "RemainEventsByRegex";
    pub const FILTER_LOG_BY_VARIANTS: &'static str = "FilterLogByVariants";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_NAME: &'static str = "DrawPlacementOfEventByName";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_REGEX: &'static str = "DrawPlacementOfEventsByRegex";
    pub const DRAW_FULL_ACTIVITIES_DIAGRAM: &'static str = "DrawFullActivitiesDiagram";
    pub const DRAW_SHORT_ACTIVITIES_DIAGRAM: &'static str = "DrawShortActivitiesDiagram";
    pub const GET_EVENT_LOG_INFO: &'static str = "GetEventLogInfo";
    pub const CLEAR_ACTIVITIES: &'static str = "ClearActivities";
    pub const GET_UNDERLYING_EVENTS_COUNT: &'static str = "GetUnderlyingEventsCount";
    pub const FILTER_TRACES_BY_EVENTS_COUNT: &'static str = "FilterTracesByEventsCount";
    pub const TRACES_DIVERSITY_DIAGRAM: &'static str = "TracesDiversityDiagram";
    pub const GET_NAMES_EVENT_LOG: &'static str = "GetNamesEventLog";
    pub const GET_HASHES_EVENT_LOG: &'static str = "GetHashesEventLog";
    pub const USE_NAMES_EVENT_LOG: &'static str = "UseNamesEventLog";
    pub const DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL: &'static str = "DiscoverActivitiesForSeveralLevels";
    pub const DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES: &'static str = "DiscoverActivitiesInUnattachedSubTraces";
    pub const DISCOVER_ACTIVITIES_UNTIL_NO_MORE: &'static str = "DiscoverActivitiesUntilNoMore";
    pub const EXECUTE_WITH_EACH_ACTIVITY_LOG: &'static str = "ExecuteWithEachActivityLog";
    pub const SUBSTITUTE_UNDERLYING_EVENTS: &'static str = "SubstituteUnderlyingEvents";
    pub const EXECUTE_FRONTEND_PIPELINE: &'static str = "ExecuteFrontendPipeline";
    pub const APPLY_CLASS_EXTRACTOR: &'static str = "ApplyClassExtractor";
    pub const SERIALIZE_PETRI_NET: &'static str = "SerializePetriNet";
    pub const DISCOVER_PETRI_NET_ALPHA: &'static str = "DiscoverPetriNetAlpha";
    pub const ADD_ARTIFICIAL_START_END_EVENTS: &'static str = "AddArtificialStartEndEvents";
    pub const ADD_ARTIFICIAL_START_EVENTS: &'static str = "AddArtificialStartEvents";
    pub const ADD_ARTIFICIAL_END_EVENTS: &'static str = "AddArtificialEndEvents";
    pub const DISCOVER_PETRI_NET_ALPHA_PLUS: &'static str = "DiscoverPetriNetAlphaPlus";
    pub const DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS: &'static str = "DiscoverPetriNetAlphaPlusPlus";
    pub const DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS_NFC: &'static str = "DiscoverPetriNetAlphaPlusPlusNfc";
    pub const DISCOVER_DFG: &'static str = "DiscoverDirectlyFollowsGraph";
    pub const DISCOVER_PETRI_NET_HEURISTIC: &'static str = "DiscoverPetriNetHeuristic";
    pub const DISCOVER_FUZZY_GRAPH: &'static str = "DiscoverFuzzyGraph";
    pub const READ_LOG_FROM_BXES: &'static str = "ReadLogFromBxes";
    pub const WRITE_LOG_TO_BXES: &'static str = "WriteLogToBxes";
    pub const DISCOVER_CASES: &'static str = "DiscoverCases";
    pub const TRACES_DIVERSITY_DIAGRAM_BY_ATTRIBUTE: &'static str = "TracesDiversityDiagramByAttribute";
    pub const DISCOVER_DFG_BY_ATTRIBUTE: &'static str = "DiscoverDirectlyFollowsGraphByAttribute";
    pub const APPEND_ATTRIBUTES_TO_NAME: &'static str = "AppendAttributesToName";
    pub const DISCOVER_DFG_STREAM: &'static str = "DiscoverDirectlyFollowsGraphStream";

    pub const ANNOTATE_PETRI_NET_COUNT: &'static str = "AnnotatePetriNetWithCount";
    pub const ANNOTATE_PETRI_NET_FREQUENCY: &'static str = "AnnotatePetriNetWithFrequency";
    pub const ANNOTATE_PETRI_NET_TRACE_FREQUENCY: &'static str = "AnnotatePetriNetWithTraceFrequency";

    pub const ANNOTATE_GRAPH_WITH_TIME: &'static str = "AnnotateGraphWithTime";

    pub const ENSURE_INITIAL_MARKING: &'static str = "EnsureInitialMarking";

    pub const CLUSTERIZE_ACTIVITIES_FROM_TRACES_KMEANS: &'static str = "ClusterizeActivitiesFromTracesKMeans";
    pub const CLUSTERIZE_ACTIVITIES_FROM_TRACES_KMEANS_GRID_SEARCH: &'static str = "ClusterizeActivitiesFromTracesKMeansGridSearch";
    pub const CLUSTERIZE_ACTIVITIES_FROM_TRACES_DBSCAN: &'static str = "ClusterizeActivitiesFromTracesDbscan";
    pub const CREATE_TRACES_ACTIVITIES_DATASET: &'static str = "CreateTracesActivitiesDataset";
    pub const CLUSTERIZE_LOG_TRACES: &'static str = "ClusterizeLogTraces";
    pub const SERIALIZE_ACTIVITIES_LOGS: &'static str = "SerializeActivitiesLogs";
    pub const REVERSE_HIERARCHY_INDICES: &'static str = "ReverseHierarchyIndices";

    pub const READ_XES_LOG_FROM_BYTES: &'static str = "ReadXesLogFromBytes";
    pub const READ_BXES_LOG_FROM_BYTES: &'static str = "ReadBxesLogFromBytes";
    pub const WRITE_XES_LOG_TO_BYTES: &'static str = "WriteXesLogToBytes";
    pub const WRITE_BXES_LOG_TO_BYTES: &'static str = "WriteBxesLogToBytes";

    pub const MERGE_XES_LOGS_FROM_PATHS: &'static str = "MergeXesLogsFromPaths";
}
