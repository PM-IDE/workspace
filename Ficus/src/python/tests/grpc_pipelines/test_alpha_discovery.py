from ...ficus.grpc_pipelines.discovery_parts import DiscoverPetriNetAlpha

from .discovery_test_core import _execute_discovery_test


def test_discover_petri_net_alpha():
    _execute_discovery_test(
        'test_discover_petri_net_alpha',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'B', 'E', 'D']
        ],
        DiscoverPetriNetAlpha(),
    )


def test_discover_petri_net_alpha2():
    _execute_discovery_test(
        'test_discover_petri_net_alpha2',
        [
            ['A', 'C'],
            ['A', 'B', 'C'],
            ['A', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'B', 'C'],
        ],
        DiscoverPetriNetAlpha(),
    )


def test_discover_petri_net_alpha3():
    _execute_discovery_test(
        'test_discover_petri_net_alpha3',
        [
            ['A', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
        ],
        DiscoverPetriNetAlpha(),
    )


def test_discover_petri_net_alpha4():
    _execute_discovery_test(
        'test_discover_petri_net_alpha4',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'C', 'B', 'D'],
            ['E', 'F'],
        ],
        DiscoverPetriNetAlpha()
    )


def test_discover_petri_net_alpha5():
    _execute_discovery_test(
        'test_discover_petri_net_alpha5',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'C', 'B', 'D'],
            ['A', 'E', 'D'],
        ],
        DiscoverPetriNetAlpha()
    )


def test_discover_petri_net_alpha6():
    _execute_discovery_test(
        'test_discover_petri_net_alpha6',
        [
            ['A', 'B', 'E', 'C', 'C', 'D'],
            ['A', 'B', 'C', 'C', 'E', 'D'],
            ['A', 'E', 'C', 'C', 'D'],
        ],
        DiscoverPetriNetAlpha()
    )


def test_discover_petri_net_alpha7():
    _execute_discovery_test(
        'test_discover_petri_net_alpha7',
        [
            ['A', 'B', 'B', 'C', 'C', 'D'],
            ['A', 'C', 'C', 'B', 'B', 'D'],
            ['A', 'C', 'C', 'D'],
        ],
        DiscoverPetriNetAlpha()
    )


def test_discover_petri_net_alpha8():
    _execute_discovery_test(
        'test_discover_petri_net_alpha8',
        [
            ['A', 'B', 'B', 'D'],
            ['A', 'B', 'C', 'C', 'B', 'D'],
            ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
        ],
        DiscoverPetriNetAlpha()
    )
