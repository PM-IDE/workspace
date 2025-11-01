from ....ficus import *

from ...grpc_pipelines.test_grpc_pipelines import _execute_pipeline, assert_success_pipeline_final_result
from ...test_data_provider import *

class AssertCorrectOcelAnnotation(PipelinePartWithCallback):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_simple_get_context_value_part(
      self.uuid,
      self.__class__.__name__,
      const_ocel_annotation,
    )

    return GrpcPipelinePartBase(simpleContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    print(values)


def test_ocel_annotation_1():
  _execute_ocel_annotation_test('ocel.bxes', Pipeline(
    ReadLogFromBxes(use_bytes=True),
    RemainEventsByRegex('(^Procfiler|^Ocel)'),
    RemainOnlyMethodStartEvents(),
    AddStartArtificialEvents(),
    DiscoverRootSequenceGraph(root_sequence_kind=RootSequenceKind.FindBest,
                              merge_sequences_of_events=False),
    AnnotateGraphWithOCEL(),
    AssertCorrectOcelAnnotation()
  ))


def _execute_ocel_annotation_test(log_name: str, pipeline: Pipeline):
  with open(get_ocel_logs_software_data_extraction_config(), "r") as f:
    software_data_config = f.read()

  result = _execute_pipeline(pipeline, {
    'bytes': BytesContextValue(read_file_bytes(get_ocel_log_path(log_name))),
    'software_data_extraction_config': JsonContextValue(software_data_config)
  })

  assert_success_pipeline_final_result(result)