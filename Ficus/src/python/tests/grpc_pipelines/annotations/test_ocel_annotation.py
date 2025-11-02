import json

from ...core.gold_based_test import execute_test_with_gold
from ....ficus import *

from ...grpc_pipelines.test_grpc_pipelines import _execute_pipeline, assert_success_pipeline_final_result
from ...test_data_provider import *


class AssertCorrectOcelAnnotation(PipelinePartWithCallback):
  def __init__(self, test_name: str):
    super().__init__()
    self.test_name = test_name

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_simple_get_context_value_part(
      self.uuid,
      self.__class__.__name__,
      const_ocel_annotation,
    )

    return GrpcPipelinePartBase(simpleContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    assert const_ocel_annotation in values

    annotations = list(values[const_ocel_annotation].ocel_annotation.annotations)
    annotations = sorted(annotations, key=lambda a: a.element_id)
    final_states = list(map(lambda a: a.final_state, annotations))

    test_results = []

    for state in final_states:
      entity_states = []
      type_states = sorted(state.type_states, key=lambda t: t.type)

      for type_state in type_states:
        ids = [id for id in type_state.object_ids]
        ids.sort()

        entity_states.append({
          'type': type_state.type,
          'object_ids': ids
        })

      test_results.append(entity_states)

    gold_path = get_ocel_gold_path(self.test_name)

    execute_test_with_gold(gold_path, json.dumps(test_results, indent=2))


def test_ocel_annotation_1():
  _execute_ocel_annotation_test(
    'test_ocel_annotation_1',
    'ocel.xes',
    [
      RemainEventsByRegex('(^Procfiler|^Ocel)'),
      FilterEventsByRegex('NextId'),
      RemainOnlyMethodStartEvents(),
      PrepareSoftwareLog(time_attribute='time:timestamp'),
    ]
  )


def _execute_ocel_annotation_test(test_name: str, log_name: str, filter_parts: list[PipelinePart]):
  with open(get_ocel_logs_software_data_extraction_config(), "r") as f:
    software_data_config = f.read()

  pipeline = Pipeline()

  pipeline.parts.append(ReadLogFromXes(use_bytes=True))

  pipeline.parts.extend(filter_parts)

  pipeline.parts.extend([
    AddStartArtificialEvents(),
    DiscoverRootSequenceGraph(root_sequence_kind=RootSequenceKind.FindBest,
                              merge_sequences_of_events=False),
    AnnotateGraphWithOCEL(),
    AssertCorrectOcelAnnotation(test_name)
  ])

  result = _execute_pipeline(pipeline, {
    'bytes': BytesContextValue(read_file_bytes(get_ocel_log_path(log_name))),
    'software_data_extraction_config': JsonContextValue(software_data_config)
  })

  assert_success_pipeline_final_result(result)
