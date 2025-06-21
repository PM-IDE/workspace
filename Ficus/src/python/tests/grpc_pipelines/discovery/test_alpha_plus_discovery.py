from ....ficus.grpc_pipelines.discovery_parts import DiscoverPetriNetAlphaPlus

from .discovery_test_core import _execute_discovery_test


def test_discover_petri_net_alpha_plus():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus',
    [
      ['A', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'C'],
      ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
    ],
    DiscoverPetriNetAlphaPlus()
  )


def test_discover_petri_net_alpha_plus2():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus2',
    [
      ['A', 'C'],
      ['A', 'B', 'C'],
      ['A', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'C'],
      ['A', 'B', 'B', 'B', 'B', 'C'],
    ],
    DiscoverPetriNetAlphaPlus()
  )


def test_discover_petri_net_alpha_plus4():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus4',
    [
      ['A', 'B', 'C', 'D'],
      ['A', 'C', 'B', 'D'],
      ['E', 'F'],
    ],
    DiscoverPetriNetAlphaPlus()
  )


def test_discover_petri_net_alpha_plus5():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus5',
    [
      ['A', 'B', 'C', 'D'],
      ['A', 'C', 'B', 'D'],
      ['A', 'E', 'D'],
    ],
    DiscoverPetriNetAlphaPlus()
  )


def test_discover_petri_net_alpha_plus6():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus6',
    [
      ['A', 'B', 'E', 'C', 'C', 'D'],
      ['A', 'B', 'C', 'C', 'E', 'D'],
      ['A', 'E', 'C', 'C', 'D'],
    ],
    DiscoverPetriNetAlphaPlus()
  )


def test_discover_petri_net_alpha_plus7():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus7',
    [
      ['A', 'B', 'B', 'C', 'C', 'D'],
      ['A', 'C', 'C', 'B', 'B', 'D'],
      ['A', 'C', 'C', 'D'],
    ],
    DiscoverPetriNetAlphaPlus()
  )


def test_discover_petri_net_alpha_plus8():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus8',
    [
      ['A', 'B', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'B', 'D'],
      ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
    ],
    DiscoverPetriNetAlphaPlus()
  )
