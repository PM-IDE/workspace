from common import execute_pipeline, pipeline_with_default_cfg, PipelinePartInfo
from ficus import *
import os

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
    ]),
    PipelinePartInfo(
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
        TerminateIfEmptyLog(),
        DiscoverCases(start_regex='GC/Start', end_regex='GC/Stop', inline_inner_cases=True, pipeline=Pipeline(
          DiscoverMultithreadedDfg(thread_attribute='NativeThreadId'),
          ViewGraph(export_path=os.path.join(os.path.abspath(os.curdir), 'multithreaded.png')),
          DiscoverDirectlyFollowsGraph(),
          ViewGraph(export_path=os.path.join(os.path.abspath(os.curdir), 'default.png')),
          AbstractMultithreadedEventsGroups(
            thread_attribute='NativeThreadId',
            time_attribute='QpcStamp',
            n_components=NComponents.Two,
            distance=Distance.Cosine,
            tolerance=0.1,
            view_params=(-50, 20),
            put_noise_events_in_one_cluster=False,
            min_events_count_in_cluster=2,
            feature_count_kind=FeatureCountKind.Count,
            visualization_method=DatasetVisualizationMethod.TSNE,
            after_clusterization_pipeline=Pipeline(
              AddStartEndArtificialEvents(),
              TracesDiversityDiagramCanvas(),
              DiscoverLoopsStrict(),
              CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
              DiscoverECFG(root_sequence_kind=RootSequenceKind.FindBest,
                           merge_sequences_of_events=False),
              AnnotateGraphWithTime(TimeAnnotationKind.Mean),
            )
          ),
        ))
      ],
      os.path.join(os.path.abspath(os.curdir), 'gc_config.json'),
      'GC pipeline'
    )
  ]
)
