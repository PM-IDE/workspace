from ...ficus.legacy.analysis.event_log_analysis import DatasetVisualizationMethod, NComponents
from ...ficus.grpc_pipelines.activities_parts import ClusterizeLogTracesDbscan, DiscoverActivitiesFromPatterns, \
    DiscoverActivitiesInstances, ClusterizeActivitiesFromTracesDbscan, DiscoverActivitiesForSeveralLevels
from ...ficus.grpc_pipelines.constants import const_labeled_log_traces_dataset, const_cluster_labels, \
    const_labeled_traces_activities_dataset
from ...ficus.grpc_pipelines.context_values import from_grpc_labeled_dataset
from ...ficus.grpc_pipelines.data_models import Distance, PatternsKind, PatternsDiscoveryStrategy, NarrowActivityKind, \
    ActivitiesRepresentationSource
from ...ficus.grpc_pipelines.grpc_pipelines import Pipeline, PipelinePartWithCallback, PipelinePart
from ...ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcContextValue
from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog
from .test_grpc_pipelines import _execute_test_with_names_log, ResultAssertanceKind


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
        [1, 1, 2, 2]
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
                                  tolerance=5,
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
        [1, 1, 2, 2]
    )


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
        [1, 1, 2, 2]
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
        [1, 1, 2, 2]
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
        [1, 1, 2, 2]
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
        [1, 1, 2, 2]
    )


class TestDatasetPipelinePart(PipelinePartWithCallback):
    def __init__(self,
                 original_part: PipelinePart,
                 expected_columns: list[str],
                 expected_rows: list[str],
                 expected_dataset: list[list[float]],
                 expected_clusters: list[int],
                 labeled_dataset_key: str):
        super().__init__()
        self.uuid = original_part.uuid
        self.original_part = original_part
        self.expected_dataset = expected_dataset
        self.labeled_dataset_key = labeled_dataset_key
        self.expected_clusters = expected_clusters
        self.expected_columns = expected_columns
        self.expected_rows = expected_rows

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return self.original_part.to_grpc_part()

    def execute_callback(self, values: dict[str, GrpcContextValue], labeled_log_traces_dataset=None):
        dataset = values[self.labeled_dataset_key].labeled_dataset
        df = from_grpc_labeled_dataset(dataset)

        assert df.columns.tolist() == self.expected_columns
        assert df.index.tolist() == self.expected_rows
        assert df.drop([const_cluster_labels], axis=1).values.tolist() == self.expected_dataset
        assert df[const_cluster_labels].to_numpy().tolist() == self.expected_clusters


def execute_test_with_activities_dataset(names_log,
                                         clusterization_pipeline,
                                         expected_columns,
                                         expected_rows,
                                         expected_raw_dataset,
                                         expected_clusters,
                                         assertance_kind=ResultAssertanceKind.Success):
    _execute_test_with_names_log(
        names_log,
        Pipeline(
            UseNamesEventLog(),
            DiscoverActivitiesForSeveralLevels(event_classes=['.*'],
                                               patterns_kind=PatternsKind.MaximalRepeats),
            TestDatasetPipelinePart(clusterization_pipeline,
                                    expected_columns,
                                    expected_rows,
                                    expected_raw_dataset,
                                    expected_clusters,
                                    const_labeled_traces_activities_dataset)
        ),
        assertance_kind
    )


def execute_test_with_traces_dataset(names_log,
                                     clusterization_pipeline,
                                     expected_columns,
                                     expected_rows,
                                     expected_raw_dataset,
                                     expected_clusters,
                                     assertance_kind=ResultAssertanceKind.Success):
    _execute_test_with_names_log(
        names_log,
        Pipeline(
            UseNamesEventLog(),
            TestDatasetPipelinePart(clusterization_pipeline,
                                    expected_columns,
                                    expected_rows,
                                    expected_raw_dataset,
                                    expected_clusters,
                                    const_labeled_log_traces_dataset),
        ),
        assertance_kind
    )
