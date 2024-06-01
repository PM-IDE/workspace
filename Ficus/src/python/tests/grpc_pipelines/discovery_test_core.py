import os
import tempfile

from ...ficus.grpc_pipelines.discovery_parts import SerializePetriNetToPNML, ViewPetriNet
from ...ficus.grpc_pipelines.grpc_pipelines import Pipeline

from ...ficus.grpc_pipelines.mutation_parts import AddStartEndArtificialEvents
from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog
from ..core.gold_based_test import execute_test_with_gold

from ..grpc_pipelines.test_grpc_pipelines import _execute_test_with_names_log
from ..test_data_provider import petri_net_test_gold_dir


def _execute_discovery_test(test_name: str, names_log: list[list[str]], discovery_part):
    temp_file = tempfile.NamedTemporaryFile()
    temp_file.close()

    gold_folder = petri_net_test_gold_dir(test_name)
    if not os.path.exists(gold_folder):
        os.makedirs(gold_folder, exist_ok=True)

    _execute_test_with_names_log(names_log, Pipeline(
        UseNamesEventLog(),
        AddStartEndArtificialEvents(),
        discovery_part,
        SerializePetriNetToPNML(save_path=temp_file.name, use_names_as_ids=True),
        ViewPetriNet(show_places_names=True, export_path=os.path.join(gold_folder, '.nets', 'petri_net.png'))
    ))

    with open(temp_file.name) as fin:
        execute_test_with_gold(os.path.join(gold_folder, 'petri_net.gold'), fin.read())

