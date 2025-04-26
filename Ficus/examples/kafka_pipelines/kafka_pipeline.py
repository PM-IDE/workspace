from common import execute_pipeline
from ficus import *

execute_pipeline(
    'MySubscription',
    'Pipeline',
    [
        PrintEventLogInfo(),
        PrintEventLogInfo(),
        ShortenAllocationType(),
        PrepareSoftwareLog(),
        AddStartEndArtificialEvents(),
        DiscoverLoopsStrict(),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        DiscoverRootSequenceGraph(root_sequence_kind=RootSequenceKind.FindBest,
                                  merge_sequences_of_events=False),
        AnnotateGraphWithTime(TimeAnnotationKind.Mean),
    ]
)
