from ..test_grpc_pipelines import _execute_test_with_names_log, ResultAssertanceKind
from ....ficus.grpc_pipelines.activities_parts import DiscoverActivitiesForSeveralLevels
from ....ficus.grpc_pipelines.constants import const_labeled_log_traces_dataset, const_cluster_labels, \
  const_labeled_traces_activities_dataset
from ....ficus.grpc_pipelines.context_values import from_grpc_labeled_dataset
from ....ficus.grpc_pipelines.data_models import PatternsKind
from ....ficus.grpc_pipelines.entry_points.default_pipeline import Pipeline, PipelinePartWithCallback, PipelinePart
from ....ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcContextValue
from ....ficus.grpc_pipelines.util_parts import UseNamesEventLog


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
