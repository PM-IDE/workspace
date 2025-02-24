from ...ficus.legacy.pipelines.contexts.part_results import PipelinePartResult
from ...ficus.legacy.pipelines.pipelines import InternalPipelinePart, Pipeline, ParallelPipeline, IfPipeline, \
  WithInputCopy, \
  WithTempInput
from ...ficus.legacy.pipelines.start.start_parts import EmptyStartPart

custom_pipeline_part_data_key = 'custom_pipeline_part_data_key'


class CustomPipelinePart(InternalPipelinePart):
  def __init__(self, index: int):
    self.index = index

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    if current_input.has_value(custom_pipeline_part_data_key):
      current_value = current_input.get_value_or_throw(custom_pipeline_part_data_key)
      return current_input.with_custom_data(custom_pipeline_part_data_key, current_value + f' {self.index}')

    return current_input.with_custom_data(custom_pipeline_part_data_key, f'{self.index}')


def test_pipelines_infra():
  result = Pipeline(
    EmptyStartPart(),
    CustomPipelinePart(1),
    CustomPipelinePart(2),
    CustomPipelinePart(3),
    CustomPipelinePart(4),
    CustomPipelinePart(5),
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '1 2 3 4 5'


def test_parallel_pipeline():
  result = Pipeline(
    EmptyStartPart(),
    ParallelPipeline(
      Pipeline(
        CustomPipelinePart(1),
        CustomPipelinePart(2),
      ),
      Pipeline(
        CustomPipelinePart(3),
        CustomPipelinePart(4),
        CustomPipelinePart(5),
      )
    )
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '3 4 5'


def test_parallel_pipeline_2():
  result = Pipeline(
    EmptyStartPart(),
    ParallelPipeline(
      Pipeline(
        CustomPipelinePart(1),
        CustomPipelinePart(2),
        CustomPipelinePart(3),
      )
    )
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '1 2 3'


def test_parallel_pipeline_3():
  result = Pipeline(
    EmptyStartPart(),
    ParallelPipeline(
      Pipeline(
        CustomPipelinePart(1),
      ),
      Pipeline(
        CustomPipelinePart(2),
      ),
      Pipeline(
        CustomPipelinePart(3),
      ),
      Pipeline(
        CustomPipelinePart(4),
      ),
      Pipeline(
        CustomPipelinePart(5),
      ),
    )
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '5'


def test_if_pipeline():
  def decision_func(current_input: PipelinePartResult) -> bool:
    return len(current_input.get_value_or_throw(custom_pipeline_part_data_key)) > 2

  result = Pipeline(
    EmptyStartPart(),
    CustomPipelinePart(1),
    IfPipeline(decision_func, Pipeline(
      CustomPipelinePart(2),
      CustomPipelinePart(3),
      CustomPipelinePart(4),
      CustomPipelinePart(5),
    ))
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '1'


def test_if_pipeline_2():
  def decision_func(current_input: PipelinePartResult) -> bool:
    return len(current_input.get_value_or_throw(custom_pipeline_part_data_key)) >= 1

  result = Pipeline(
    EmptyStartPart(),
    CustomPipelinePart(1),
    IfPipeline(decision_func, Pipeline(
      CustomPipelinePart(2),
      CustomPipelinePart(3),
      CustomPipelinePart(4),
      CustomPipelinePart(5),
    ))
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '1 2 3 4 5'


def test_with_input_copy_pipeline():
  result = Pipeline(
    EmptyStartPart(),
    CustomPipelinePart(1),
    CustomPipelinePart(2),
    WithInputCopy(Pipeline(
      CustomPipelinePart(3),
      CustomPipelinePart(4),
      CustomPipelinePart(5),
    ))
  )()

  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '1 2'


def test_with_input_copy_2():
  result = Pipeline(
    EmptyStartPart(),
    WithInputCopy(Pipeline(
      CustomPipelinePart(1),
      CustomPipelinePart(2),
      CustomPipelinePart(3),
      CustomPipelinePart(4),
      CustomPipelinePart(5),
    ))
  )()

  assert result.has_value(custom_pipeline_part_data_key) == False


def test_with_temp_input():
  order = []

  def temp_input_creator(initial_input: PipelinePartResult) -> PipelinePartResult:
    order.append(1)
    return initial_input.with_custom_data(custom_pipeline_part_data_key, '0')

  def output_merger(initial_input: PipelinePartResult, temp_input: PipelinePartResult) -> PipelinePartResult:
    order.append(2)
    data = temp_input.get_value_or_throw(custom_pipeline_part_data_key)
    return initial_input.with_custom_data(custom_pipeline_part_data_key, data)

  result = Pipeline(
    EmptyStartPart(),
    WithTempInput(temp_input_creator, output_merger, Pipeline(
      CustomPipelinePart(1),
      CustomPipelinePart(2),
      CustomPipelinePart(3),
      CustomPipelinePart(4),
      CustomPipelinePart(5),
    ))
  )()

  assert order == [1, 2]
  assert result.get_value_or_throw(custom_pipeline_part_data_key) == '0 1 2 3 4 5'
