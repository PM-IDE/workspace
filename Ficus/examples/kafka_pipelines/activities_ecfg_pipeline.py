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
      DiscoverLoopsStrict(),
      CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
      ClearActivitiesRelatedStuff(),
      DiscoverActivitiesForSeveralLevels(['.*'],
                                         PatternsKind.MaximalRepeats,
                                         activity_filter_kind=ActivityFilterKind.NoFilter),
      DrawFullActivitiesDiagramCanvas(plot_legend=False, height_scale=10, width_scale=0.01),
      ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                           tolerance=0.1,
                                           activities_repr_source=ActivitiesRepresentationSource.EventClasses,
                                           distance=Distance.Cosine,
                                           activity_level=0,
                                           view_params=(30, 60),
                                           legend_cols=4,
                                           visualization_method=DatasetVisualizationMethod.TSNE,
                                           n_components=NComponents.Three),
      DrawFullActivitiesDiagramCanvas(plot_legend=False, height_scale=10, width_scale=0.01),
      CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
      ClearActivitiesRelatedStuff(),
      TracesDiversityDiagramCanvas(),
      DiscoverECFG(root_sequence_kind=RootSequenceKind.FindBest,
                   merge_sequences_of_events=False),
      AnnotateGraphWithTime(TimeAnnotationKind.Mean),
    ])
  ]
)
