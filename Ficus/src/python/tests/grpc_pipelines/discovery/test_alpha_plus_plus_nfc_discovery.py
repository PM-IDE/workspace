from ....ficus.grpc_pipelines.discovery_parts import DiscoverPetriNetAlphaPlusPlusNfc

from .discovery_test_core import _execute_discovery_test


def test_discover_petri_net_alpha_plus_plus_nfc():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc',
    [
      ['A', 'B', 'C'],
      ['A', 'B', 'D', 'E', 'C'],
      ['A', 'D', 'B', 'E', 'C'],
      ['A', 'D', 'E', 'B', 'C'],
      ['A', 'B', 'D', 'E', 'D', 'E', 'C']
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_2():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_2',
    [
      ['A', 'B', 'C', 'E'],
      ['A', 'C', 'B', 'E'],
      ['A', 'D', 'E']
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_3():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_3',
    [
      ['A', 'C', 'F', 'B', 'G', 'E'],
      ['A', 'F', 'C', 'B', 'G', 'E'],
      ['A', 'F', 'B', 'C', 'G', 'E'],
      ['A', 'F', 'B', 'C', 'G', 'E'],
      ['A', 'F', 'B', 'G', 'C', 'E'],
      ['A', 'F', 'D', 'G', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_4():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_4',
    [
      ['A', 'C', 'D'],
      ['B', 'C', 'E'],
      ['A', 'F', 'C', 'E'],
      ['A', 'C', 'F', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_5():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_5',
    [
      ['A', 'B', 'C'],
      ['A', 'B', 'D', 'E'],
      ['A', 'D', 'B', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_6():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_6',
    [
      ['A', 'C', 'D'],
      ['A', 'C', 'E'],
      ['B', 'C', 'D'],
      ['B', 'C', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_7():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_7',
    [
      ['A', 'C', 'D'],
      ['B', 'C', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_8():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_8',
    [
      ['A', 'C', 'F', 'D'],
      ['A', 'F', 'C', 'D'],
      ['B', 'C', 'G', 'E'],
      ['B', 'G', 'C', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_9():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_9',
    [
      ['A', 'C', 'D'],
      ['B', 'C', 'F', 'E'],
      ['B', 'F', 'C', 'E'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_10():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_10',
    [
      ['A', 'C', 'E', 'B', 'C', 'D'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_11():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_11',
    [
      ['A', 'C', 'D', 'E', 'G', 'H'],
      ['A', 'C', 'D', 'G', 'E', 'H'],
      ['A', 'C', 'G', 'D', 'E', 'H'],
      ['B', 'C', 'D', 'F', 'H'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )


def test_discover_petri_net_alpha_plus_plus_nfc_13():
  _execute_discovery_test(
    'test_discover_petri_net_alpha_plus_plus_nfc_13',
    [
      ['F', 'B', 'G'],
      ['A', 'B', 'C'],
      ['F', 'D', 'B', 'E', 'G'],
      ['F', 'B', 'D', 'E', 'G'],
      ['F', 'D', 'E', 'B', 'G'],
      ['A', 'D', 'E', 'D', 'E', 'B', 'G'],
      ['A', 'B', 'D', 'E', 'C'],
    ],
    DiscoverPetriNetAlphaPlusPlusNfc()
  )
