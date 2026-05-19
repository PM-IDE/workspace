from common import execute_pipeline, pipeline_with_default_cfg
from ficus import *

execute_pipeline(
  'MySubscription',
  'Pipeline',
  [
    pipeline_with_default_cfg([
      RemainOnlyMethodStartEvents(),
      SetMethodsDisplayName(),
      ShortenAllocationType(),
      PrepareSoftwareLog(time_attribute='QpcStamp'),
      TerminateIfEmptyLog(),
      TracesDiversityDiagramCanvas(),
      AddStartEndArtificialEvents(),
      TracesDiversityDiagramCanvas(),
      DiscoverLoopsStrict(),
      CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
      TracesDiversityDiagramCanvas(),
      DiscoverECFG(root_sequence_kind=RootSequenceKind.FindBest,
                   merge_sequences_of_events=False),
      AnnotateGraphWithTime(TimeAnnotationKind.Mean),
    ])
  ]
)
