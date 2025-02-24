from ...ficus.grpc_pipelines.discovery_parts import ViewDirectlyFollowsGraph
from ...ficus.grpc_pipelines.entry_points.default_pipeline import Pipeline
from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog
from .test_grpc_pipelines import _execute_test_with_names_log


def test_dfg_discovery():
  _execute_test_with_names_log(
    [
      ['A', 'B', 'C', 'D'],
      ['A', 'B', 'E', 'D']
    ],
    Pipeline(
      UseNamesEventLog(),
      ViewDirectlyFollowsGraph()
    )
  )
