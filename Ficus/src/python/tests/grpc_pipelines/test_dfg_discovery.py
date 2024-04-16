from ...ficus.grpc_pipelines.discovery_parts import ViewDirectlyFollowsGraph2
from ...ficus.grpc_pipelines.grpc_pipelines import Pipeline2
from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog2
from .test_grpc_pipelines import _execute_test_with_names_log


def test_dfg_discovery():
    _execute_test_with_names_log(
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'B', 'E', 'D']
        ],
        Pipeline2(
            UseNamesEventLog2(),
            ViewDirectlyFollowsGraph2()
        )
    )
