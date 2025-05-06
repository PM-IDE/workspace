from common import execute_pipeline
from ficus import *

execute_pipeline(
    'MySubscription',
    'Pipeline',
    [
        PrintEventLogInfo(),
        TracesDiversityDiagramCanvas(),
        SetMethodsDisplayName(),
        ShortenAllocationType(),
        ShortenMethodNames(),
        PrepareSoftwareLog(time_attribute = 'QpcStamp'),
        AddStartEndArtificialEvents(),
        TracesDiversityDiagramCanvas(),
        DiscoverLoopsStrict(),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        TracesDiversityDiagramCanvas(),
        DiscoverRootSequenceGraph(root_sequence_kind=RootSequenceKind.FindBest,
                                  merge_sequences_of_events=False),
        AnnotateGraphWithTime(TimeAnnotationKind.Mean),
    ]
)
