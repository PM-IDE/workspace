use ficus::pipelines::pipeline_parts::PipelineParts;
use ficus::vecs;

fn get_test_parts_names() -> Vec<String> {
  vecs![
        "ReadLogFromXes",
        "WriteLogToXes",
        "FindPrimitiveTandemArrays",
        "FindMaximalTandemArrays",
        "FindMaximalRepeats",
        "FindSuperMaximalRepeats",
        "FindNearSuperMaximalRepeats",
        "DiscoverActivities",
        "DiscoverActivitiesInstances",
        "CreateLogFromActivities",
        "FilterEventsByName",
        "FilterEventsByRegex",
        "RemainEventsByRegex",
        "FilterLogByVariants",
        "DrawPlacementOfEventByName",
        "DrawPlacementOfEventsByRegex",
        "DrawFullActivitiesDiagram",
        "DrawShortActivitiesDiagram",
        "GetEventLogInfo",
        "ClearActivities",
        "GetUnderlyingEventsCount",
        "FilterTracesByEventsCount",
        "TracesDiversityDiagram",
        "GetHashesEventLog",
        "GetNamesEventLog",
        "UseNamesEventLog",
        "DiscoverActivitiesForSeveralLevels",
        "DiscoverActivitiesInUnattachedSubTraces",
        "DiscoverActivitiesUntilNoMore",
        "ExecuteWithEachActivityLog",
        "SubstituteUnderlyingEvents",
        "ExecuteFrontendPipeline",
        "ApplyClassExtractor",
        "DiscoverPetriNetAlpha",
        "SerializePetriNet",
        "AddArtificialStartEndEvents",
        "AddArtificialStartEvents",
        "AddArtificialEndEvents",
        "DiscoverPetriNetAlphaPlus",
        "DiscoverPetriNetAlphaPlusPlus",
        "DiscoverPetriNetAlphaPlusPlusNfc",
        "DiscoverDirectlyFollowsGraph",
        "DiscoverPetriNetHeuristic",
        "DiscoverFuzzyGraph",
        "AnnotatePetriNetWithCount",
        "AnnotatePetriNetWithFrequency",
        "AnnotatePetriNetWithTraceFrequency",
        "EnsureInitialMarking",
        "ReadLogFromBxes",
        "ClusterizeActivitiesFromTracesKMeans",
        "ClusterizeActivitiesFromTracesKMeansGridSearch",
        "ClusterizeActivitiesFromTracesDbscan",
        "CreateTracesActivitiesDataset",
        "WriteLogToBxes",
        "ClusterizeLogTraces",
        "SerializeActivitiesLogs",
        "ReadXesLogFromBytes",
        "ReadBxesLogFromBytes",
        "WriteXesLogToBytes",
        "WriteBxesLogToBytes",
        "ReverseHierarchyIndices",
        "DiscoverCases",
        "AnnotateGraphWithTime",
        "TracesDiversityDiagramByAttribute",
        "DiscoverDirectlyFollowsGraphByAttribute",
        "AppendAttributesToName",
        "MergeXesLogsFromPaths",
        "DiscoverDirectlyFollowsGraphStream",
        "DiscoverPetriNetAlphaStream",
        "DiscoverLogTimelineDiagram",
        "CreateThreadsLog",
        "AbstractTimelineDiagram",
        "ClusterizeLogTracesKMeansGridSearch",
        "ClusterizeLogTracesDbscanGridSearch",
        "DiscoverRootSequenceGraph",
        "DiscoverLoopsStrict",
        "DiscoverTracesTimelineDiagram",
        "PrepareSoftwareEventLog",
        "ShortenAllocationType",
        "ShortenMethodNames",
        "SetMethodsDisplayName",
        "RemainOnlyMethodStartEvents",
        "RemainOnlyMethodEndEvents",
        "DiscoverMultithreadedDfg",
        "AbstractMultithreadedEventsGroups",
        "MergeGraphs",
        "AddGraphToGraphs",
        "ClearGraphs",
        "TerminateIfEmptyLog"
    ]
}

#[test]
fn test_pipeline_parts() {
  let parts = PipelineParts::new();
  let names = get_test_parts_names();

  for name in names {
    assert!(parts.find_part(name.as_str()).is_some());
  }
}

#[test]
fn test_pipeline_parts_count() {
  let parts = PipelineParts::new();
  let names = get_test_parts_names();

  assert_eq!(parts.len(), names.len());
}
