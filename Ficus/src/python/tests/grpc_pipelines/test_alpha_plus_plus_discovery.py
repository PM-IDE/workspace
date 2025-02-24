from ...ficus.grpc_pipelines.discovery_parts import DiscoverPetriNetAlphaPlusPlus

from .discovery_test_core import _execute_discovery_test


def test_discover_petri_net_alpha_plus_plus():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus',
    [
      ['A', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )


def test_discover_petri_net_alpha_plus_plus2():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus2',
    [
      ['A', 'C'],
      ['A', 'B', 'C'],
      ['A', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'B', 'C'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )


def test_discover_petri_net_alpha_plus_plus4():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus4',
    [
      ['A', 'B', 'C', 'D'],
      ['A', 'C', 'B', 'D'],
      ['E', 'F'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )


def test_discover_petri_net_alpha_plus_plus5():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus5',
    [
      ['A', 'B', 'C', 'D'],
      ['A', 'C', 'B', 'D'],
      ['A', 'E', 'D'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )


def test_discover_petri_net_alpha_plus_plus6():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus6',
    [
      ['A', 'B', 'E', 'C', 'C', 'D'],
      ['A', 'B', 'C', 'C', 'E', 'D'],
      ['A', 'E', 'C', 'C', 'D'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )


def test_discover_petri_net_alpha_plus_plus7():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus7',
    [
      ['A', 'B', 'B', 'C', 'C', 'D'],
      ['A', 'C', 'C', 'B', 'B', 'D'],
      ['A', 'C', 'C', 'D'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )


def test_discover_petri_net_alpha_plus_plus8():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus8',
    [
      ['A', 'B', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
    ],
    DiscoverPetriNetAlphaPlusPlus()
  )
