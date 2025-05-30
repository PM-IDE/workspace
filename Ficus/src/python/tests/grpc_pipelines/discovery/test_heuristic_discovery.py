from ....ficus.grpc_pipelines.discovery_parts import DiscoverPetriNetHeuristic
from .discovery_test_core import _execute_discovery_test


def test_discover_petri_net_heuristic():
  _execute_discovery_test(
    'test_discover_petri_net_heuristic',
    [
      ['A', 'B', 'C', 'D'],
      ['A', 'C', 'B', 'D'],
    ],
    DiscoverPetriNetHeuristic()
  )
