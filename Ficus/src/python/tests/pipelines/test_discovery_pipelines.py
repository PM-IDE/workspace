from ..test_data_provider import get_example_log_path
from ...ficus.legacy.pipelines.contexts.keys import petri_net_key
from ...ficus.legacy.pipelines.discovery.discovery_parts import DiscoverPetriNetAlpha, DiscoverPetriNetAlphaPlus, \
  DiscoverPetriNetInductive, DiscoverPetriNetHeuristic
from ...ficus.legacy.pipelines.pipelines import Pipeline, PipelinePart
from ...ficus.legacy.pipelines.start.start_parts import ReadLogFromXes


def test_alpha_discovery():
  _do_test_with_petri_net_discovery(DiscoverPetriNetAlpha())


def _do_test_with_petri_net_discovery(discovery_part: PipelinePart):
  result = Pipeline(
    ReadLogFromXes(),
    discovery_part,
  )(get_example_log_path('exercise1.xes'))

  assert result.has_value(petri_net_key)


def test_alpha_plus_discovery():
  _do_test_with_petri_net_discovery(DiscoverPetriNetAlphaPlus())


def test_inductive_miner_discovery():
  _do_test_with_petri_net_discovery(DiscoverPetriNetInductive())


def test_heuristic_miner_discovery():
  _do_test_with_petri_net_discovery(DiscoverPetriNetHeuristic())
