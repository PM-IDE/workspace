from common import execute_pipeline
from ficus import *

execute_pipeline(
    'MySubscription',
    'GCPipeline',
    [
        RemainEventsByRegex('GC/'),
        FilterEventsByRegex('GC/RestartEEStart'),
        FilterEventsByRegex('GC/RestartEEStop'),
        FilterEventsByRegex('GC/SuspendEEStart'),
        FilterEventsByRegex('GC/SuspendEEStop'),
        FilterEventsByRegex('GC/Finaliz'),
        FilterEventsByRegex('GC/SampledObject'),
        FilterEventsByRegex('GC/SetGCHandle'),
        FilterEventsByRegex('GC/BGCAllocWait'),
        FilterEventsByRegex('GC/Pin'),
        FilterEventsByRegex('GC/CreateSegment'),
        FilterEventsByRegex('GC/Triggered'),
        FilterEventsByRegex('GC/Join'),
        FilterEventsByRegex('Heap'),
        FilterEventsByRegex('Bulk'),
        FilterEventsByRegex('Mark'),
        AddStartEndArtificialEvents(),
        DiscoverRootSequenceGraph(root_sequence_kind=RootSequenceKind.FindBest,
                                  merge_sequences_of_events=True),
    ]
)
