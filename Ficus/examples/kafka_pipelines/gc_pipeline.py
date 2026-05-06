from common import execute_pipeline, pipeline_with_default_cfg
from ficus import *

execute_pipeline(
  'MySubscription',
  'GCPipeline',
  [
    pipeline_with_default_cfg([
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
      ShortenAllocationType(),
      ShortenMethodNames(),
      AddStartEndArtificialEvents(),
      DiscoverECFG(root_sequence_kind=RootSequenceKind.FindBest,
                   merge_sequences_of_events=True),
    ])
  ]
)
