from .core import execute_test_with_activities_dataset
from ..test_grpc_pipelines import _execute_test_with_names_log, ResultAssertanceKind
from ....ficus.grpc_pipelines.activities_parts import DiscoverActivitiesFromPatterns, \
  DiscoverActivitiesInstances
from ....ficus.grpc_pipelines.clustering import *
from ....ficus.grpc_pipelines.data_models import Distance, PatternsKind, PatternsDiscoveryStrategy, NarrowActivityKind, \
  ActivitiesRepresentationSource
from ....ficus.grpc_pipelines.entry_points.default_pipeline import Pipeline
from ....ficus.grpc_pipelines.util_parts import UseNamesEventLog


def test_levenshtein_in_activities_clustering():
  _execute_test_with_names_log(
    [],
    Pipeline(
      UseNamesEventLog(),
      DiscoverActivitiesFromPatterns(patterns_kind=PatternsKind.MaximalRepeats,
                                     strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
      DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown),
      ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                           tolerance=0.1,
                                           activities_repr_source=ActivitiesRepresentationSource.SubTracesUnderlyingEvents,
                                           distance=Distance.Levenshtein,
                                           activity_level=0,
                                           view_params=(30, 60),
                                           legend_cols=4,
                                           visualization_method=DatasetVisualizationMethod.TSNE,
                                           n_components=NComponents.Three),
    ),
    assertance_kind=ResultAssertanceKind.Error
  )


def test_activities_dataset_1():
  execute_test_with_activities_dataset(
    [
      ['A', 'B', 'C', 'x', 'A', 'B', 'C'],
      ['A', 'D', 'C', 'y', 'A', 'D', 'C'],

      ['X', 'Y', 'Z', 'x', 'X', 'Y', 'Z'],
      ['X', 'Q', 'Z', 'y', 'X', 'Q', 'Z'],
    ],
    ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                         tolerance=0.5,
                                         activities_repr_source=ActivitiesRepresentationSource.EventClasses,
                                         distance=Distance.Cosine,
                                         show_visualization=False),
    ['A', 'B', 'C', 'D', 'Q', 'X', 'Y', 'Z', 'cluster_labels'],
    ['(A)::(B)::(C)', '(A)::(D)::(C)', '(X)::(Q)::(Z)', '(X)::(Y)::(Z)'],
    [
      [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      [1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
      [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
    ],
    [0, 0, 1, 1]
  )


def test_activities_dataset_2():
  execute_test_with_activities_dataset(
    [
      ['A', 'A', 'B', 'C', 'x', 'A', 'A', 'B', 'C'],
      ['A', 'D', 'C', 'C', 'y', 'A', 'D', 'C', 'C'],

      ['X', 'X', 'Y', 'Z', 'x', 'X', 'X', 'Y', 'Z'],
      ['X', 'Q', 'Z', 'Z', 'y', 'X', 'Q', 'Z', 'Z'],
    ],
    ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                         tolerance=0.5,
                                         activities_repr_source=ActivitiesRepresentationSource.EventClasses,
                                         distance=Distance.Cosine,
                                         show_visualization=False),
    ['A', 'B', 'C', 'D', 'Q', 'X', 'Y', 'Z', 'cluster_labels'],
    ['(A)::(A)::(B)::(C)',
     '(A)::(D)::(C)::(C)',
     '(X)::(Q)::(Z)::(Z)',
     '(X)::(X)::(Y)::(Z)'],
    [
      [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      [1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
      [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
    ],
    [0, 0, 1, 1]
  )


def test_activities_dataset_3():
  execute_test_with_activities_dataset(
    [
      ['A', 'A', 'B', 'C', 'x', 'A', 'A', 'B', 'C'],
      ['A', 'D', 'C', 'C', 'y', 'A', 'D', 'C', 'C'],

      ['X', 'X', 'Y', 'Z', 'x', 'X', 'X', 'Y', 'Z'],
      ['X', 'Q', 'Z', 'Z', 'y', 'X', 'Q', 'Z', 'Z'],
    ],
    ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                         tolerance=0.6,
                                         activities_repr_source=ActivitiesRepresentationSource.SubTraces,
                                         distance=Distance.Cosine,
                                         show_visualization=False),
    ['A', 'B', 'C', 'D', 'Q', 'X', 'Y', 'Z', 'cluster_labels'],
    ['(A)::(A)::(B)::(C)',
     '(A)::(D)::(C)::(C)',
     '(X)::(Q)::(Z)::(Z)',
     '(X)::(X)::(Y)::(Z)'],
    [
      [1.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
      [0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0],
      [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5]
    ],
    [0, 0, 1, 1]
  )


def test_activities_dataset_4():
  execute_test_with_activities_dataset(
    [
      ['A', 'A', 'B', 'C', 'x', 'A', 'A', 'B', 'C'],
      ['A', 'D', 'C', 'C', 'y', 'A', 'D', 'C', 'C'],

      ['X', 'X', 'Y', 'Z', 'x', 'X', 'X', 'Y', 'Z'],
      ['X', 'Q', 'Z', 'Z', 'y', 'X', 'Q', 'Z', 'Z'],
    ],
    ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                         tolerance=0.6,
                                         activities_repr_source=ActivitiesRepresentationSource.SubTracesUnderlyingEvents,
                                         distance=Distance.Cosine,
                                         show_visualization=False),
    ['A', 'B', 'C', 'D', 'Q', 'X', 'Y', 'Z', 'cluster_labels'],
    ['(A)::(A)::(B)::(C)',
     '(A)::(D)::(C)::(C)',
     '(X)::(Q)::(Z)::(Z)',
     '(X)::(X)::(Y)::(Z)'],
    [
      [1.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
      [0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0],
      [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5]
    ],
    [0, 0, 1, 1]
  )
