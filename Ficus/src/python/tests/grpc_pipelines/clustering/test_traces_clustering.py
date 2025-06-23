from .core import execute_test_with_traces_dataset
from ....ficus.grpc_pipelines.clustering import *
from ....ficus.grpc_pipelines.data_models import Distance
from ....ficus.grpc_pipelines.entry_points.default_pipeline import Pipeline


def test_traces_dataset_1():
  execute_test_with_traces_dataset(
    [
      ['A', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
    ],
    ClusterizeLogTracesDbscan(
      after_clusterization_pipeline=Pipeline(),
      min_events_count_in_cluster=2,
    ),
    ['A', 'B', 'C', 'D', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2'],
    [
      [1.0, 0.0, 1.0, 0.0],
      [1.0, 0.5, 1.0, 0.5],
      [1.0, 1.0, 1.0, 1.0]
    ],
    [0, 0, 0]
  )


def test_traces_dataset_2():
  execute_test_with_traces_dataset(
    [
      ['A', 'C'],
      ['A', 'B', 'C'],
      ['A', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'B', 'C'],
    ],
    ClusterizeLogTracesDbscan(
      after_clusterization_pipeline=Pipeline(),
      min_events_count_in_cluster=2,
    ),
    ['A', 'B', 'C', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3', 'Trace_4'],
    [
      [1.0, 0.0, 1.0],
      [1.0, 0.25, 1.0],
      [1.0, 0.5, 1.0],
      [1.0, 0.75, 1.0],
      [1.0, 1.0, 1.0]
    ],
    [0, 0, 0, 0, 0]
  )


def test_traces_dataset_3():
  execute_test_with_traces_dataset(
    [
      ['A', 'B', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
    ],
    ClusterizeLogTracesDbscan(
      after_clusterization_pipeline=Pipeline(),
      min_events_count_in_cluster=2,
    ),
    ['A', 'B', 'C', 'D', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2'],
    [
      [1.0, 1.0, 0.0, 1.0],
      [1.0, 1.0, 0.5, 1.0],
      [1.0, 1.0, 1.0, 1.0]
    ],
    [0, 0, 0]
  )


def test_traces_dataset_4():
  execute_test_with_traces_dataset(
    [
      ['A', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
    ],
    ClusterizeLogTracesDbscan(
      after_clusterization_pipeline=Pipeline(),
      min_events_count_in_cluster=2,
      distance=Distance.Levenshtein
    ),
    ['Symbol_0', 'Symbol_1', 'Symbol_2', 'Symbol_3', 'Symbol_4', 'Symbol_5', 'Symbol_6', 'Symbol_7', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2'],
    [
      [1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      [1.0, 2.0, 4.0, 2.0, 3.0, 0.0, 0.0, 0.0],
      [1.0, 2.0, 4.0, 2.0, 4.0, 2.0, 3.0, 0.0]
    ],
    [0, 0, 0],
  )


def test_traces_dataset_5():
  execute_test_with_traces_dataset(
    [
      ['A', 'C'],
      ['A', 'B', 'C'],
      ['A', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'B', 'C'],
    ],
    ClusterizeLogTracesDbscan(
      after_clusterization_pipeline=Pipeline(),
      min_events_count_in_cluster=2,
      distance=Distance.Levenshtein
    ),
    ['Symbol_0', 'Symbol_1', 'Symbol_2', 'Symbol_3', 'Symbol_4', 'Symbol_5', 'Symbol_6', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3', 'Trace_4'],
    [
      [1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      [1.0, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0],
      [1.0, 3.0, 3.0, 2.0, 0.0, 0.0, 0.0],
      [1.0, 3.0, 3.0, 3.0, 2.0, 0.0, 0.0],
      [1.0, 3.0, 3.0, 3.0, 3.0, 2.0, 0.0]
    ],
    [0, 0, 0, 0, 0]
  )


def test_traces_dataset_6():
  execute_test_with_traces_dataset(
    [
      ['A', 'B', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
    ],
    ClusterizeLogTracesDbscan(
      after_clusterization_pipeline=Pipeline(),
      min_events_count_in_cluster=2,
      distance=Distance.Levenshtein
    ),
    ['Symbol_0', 'Symbol_1', 'Symbol_2', 'Symbol_3', 'Symbol_4', 'Symbol_5', 'Symbol_6', 'Symbol_7', 'Symbol_8',
     'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2'],
    [
      [1.0, 2.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      [1.0, 2.0, 4.0, 4.0, 2.0, 3.0, 0.0, 0.0, 0.0],
      [1.0, 2.0, 4.0, 4.0, 4.0, 4.0, 2.0, 3.0, 0.0]
    ],
    [0, 0, 0]
  )


def test_traces_dataset_7():
  execute_test_with_traces_dataset(
    [
      ['A.X', 'A.B', 'B.C', 'C.C', 'x', 'A.Q', 'A.W', 'B.E', 'C.D'],
      ['A.D', 'D.D', 'C.S', 'C', 'y', 'A', 'D.D', 'C.f', 'C.A'],

      ['X.D', 'X.Q', 'Y.E', 'Z.R', 'a', 'X.P', 'X.E', 'Y.V', 'Z.D'],
      ['X.A', 'Q.D', 'Z.E', 'Z.R', 'b', 'X.F', 'Q.G', 'Z.W', 'Z.A'],
    ],
    ClusterizeLogTracesDbscan(after_clusterization_pipeline=Pipeline(),
                              min_events_count_in_cluster=2,
                              tolerance=0.7,
                              class_extractor=r'^(.*?)(?=\.)',
                              distance=Distance.Cosine,
                              show_visualization=False),
    ['A', 'B', 'C', 'D', 'Q', 'X', 'Y', 'Z', 'a', 'b', 'x', 'y', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3'],
    [
      [1.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
      [0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5, 1.0, 0.0, 0.0, 0.0],
      [0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]
    ],
    [0, 0, 1, 1]
  )


def test_traces_dataset_8():
  execute_test_with_traces_dataset(
    [
      ['A.X', 'A.B', 'B.C', 'C.C', 'x', 'A.Q', 'A.W', 'B.E', 'C.D'],
      ['A.D', 'D.D', 'C.S', 'C', 'y', 'A', 'D.D', 'C.f', 'C.A'],

      ['X.D', 'X.Q', 'Y.E', 'Z.R', 'a', 'X.P', 'X.E', 'Y.V', 'Z.D'],
      ['X.A', 'Q.D', 'Z.E', 'Z.R', 'b', 'X.F', 'Q.G', 'Z.W', 'Z.A'],
    ],
    ClusterizeLogTracesDbscan(after_clusterization_pipeline=Pipeline(),
                              min_events_count_in_cluster=2,
                              tolerance=6,
                              class_extractor=r'^(.*?)(?=\.)',
                              distance=Distance.Levenshtein,
                              show_visualization=False),
    ['Symbol_0', 'Symbol_1', 'Symbol_2', 'Symbol_3', 'Symbol_4', 'Symbol_5', 'Symbol_6',
     'Symbol_7', 'Symbol_8', 'Symbol_9', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3'],
    [
      [1.0, 1.0, 2.0, 3.0, 4.0, 1.0, 1.0, 2.0, 3.0, 0.0],
      [1.0, 5.0, 3.0, 3.0, 6.0, 1.0, 5.0, 3.0, 3.0, 0.0],
      [7.0, 7.0, 8.0, 9.0, 10.0, 7.0, 7.0, 8.0, 9.0, 0.0],
      [7.0, 11.0, 9.0, 9.0, 12.0, 7.0, 11.0, 9.0, 9.0, 0.0]
    ],
    [0, 0, 1, 1]
  )


def test_traces_dataset_10():
  execute_test_with_traces_dataset(
    [
      ['A'],
      ['B'],
      ['C'],
      ['D'],
      ['E'],
    ],
    ClusterizeLogTracesDbscan(after_clusterization_pipeline=Pipeline(),
                              min_events_count_in_cluster=2,
                              distance=Distance.LCS,
                              show_visualization=False),
    ['Symbol_0', 'Symbol_1', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3', 'Trace_4'],
    [
      [1.0, 0.0],
      [2.0, 0.0],
      [3.0, 0.0],
      [4.0, 0.0],
      [5.0, 0.0]
    ],
    [0, 0, 0, 0, 0]
  )


def test_traces_dataset_11():
  execute_test_with_traces_dataset(
    [
      ['A'],
      ['B'],
      ['C'],
      ['D'],
      ['E'],
    ],
    ClusterizeLogTracesDbscan(after_clusterization_pipeline=Pipeline(),
                              min_events_count_in_cluster=2,
                              distance=Distance.LCS,
                              show_visualization=False,
                              put_noise_events_in_one_cluster=False),
    ['Symbol_0', 'Symbol_1', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3', 'Trace_4'],
    [
      [1.0, 0.0],
      [2.0, 0.0],
      [3.0, 0.0],
      [4.0, 0.0],
      [5.0, 0.0]
    ],
    [1, 2, 3, 4, 5]
  )


def test_traces_dataset_12():
  execute_test_with_traces_dataset(
    [
      ['A', 'B', 'C', 'D', 'E'],
      ['A', 'B', 'B', 'C', 'D', 'E'],
      ['A', 'B', 'C', 'C', 'D', 'E'],

      ['X', 'D', 'C', 'B', 'Y'],
      ['X', 'D', 'D', 'C', 'B', 'Y'],
      ['X', 'D', 'C', 'B', 'B', 'Y'],
    ],
    ClusterizeLogTracesDbscan(after_clusterization_pipeline=Pipeline(),
                              min_events_count_in_cluster=2,
                              tolerance=0.2,
                              distance=Distance.LCS,
                              show_visualization=False,
                              put_noise_events_in_one_cluster=True),
    ['Symbol_0', 'Symbol_1', 'Symbol_2', 'Symbol_3', 'Symbol_4', 'Symbol_5', 'Symbol_6', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3', 'Trace_4', 'Trace_5'],
    [
      [1.0, 2.0, 3.0, 4.0, 5.0, 0.0, 0.0],
      [1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 0.0],
      [1.0, 2.0, 3.0, 3.0, 4.0, 5.0, 0.0],
      [6.0, 4.0, 3.0, 2.0, 7.0, 0.0, 0.0],
      [6.0, 4.0, 4.0, 3.0, 2.0, 7.0, 0.0],
      [6.0, 4.0, 3.0, 2.0, 2.0, 7.0, 0.0]
    ],
    [0, 0, 0, 1, 1, 1]
  )


def test_traces_dataset_13():
  execute_test_with_traces_dataset(
    [
      ['A', 'B', 'C', 'D', 'E'],
      ['A', 'B', 'B', 'C', 'D', 'E'],
      ['A', 'B', 'C', 'C', 'D', 'E'],

      ['X', 'D', 'C', 'B', 'Y'],
      ['X', 'D', 'D', 'C', 'B', 'Y'],
      ['X', 'D', 'C', 'B', 'B', 'Y'],
    ],
    ClusterizeLogTracesDbscanGridSearch(after_clusterization_pipeline=Pipeline(),
                                        tolerances=[0.1, 0.2, 0.3, 0.4, 0.5],
                                        min_points_in_cluster_vec=[2, 3, 4, 5],
                                        distance=Distance.LCS,
                                        show_visualization=False,
                                        put_noise_events_in_one_cluster=True),
    ['Symbol_0', 'Symbol_1', 'Symbol_2', 'Symbol_3', 'Symbol_4', 'Symbol_5', 'Symbol_6', 'cluster_labels'],
    ['Trace_0', 'Trace_1', 'Trace_2', 'Trace_3', 'Trace_4', 'Trace_5'],
    [
      [1.0, 2.0, 3.0, 4.0, 5.0, 0.0, 0.0],
      [1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 0.0],
      [1.0, 2.0, 3.0, 3.0, 4.0, 5.0, 0.0],
      [6.0, 4.0, 3.0, 2.0, 7.0, 0.0, 0.0],
      [6.0, 4.0, 4.0, 3.0, 2.0, 7.0, 0.0],
      [6.0, 4.0, 3.0, 2.0, 2.0, 7.0, 0.0]
    ],
    [0, 0, 0, 1, 1, 1]
  )