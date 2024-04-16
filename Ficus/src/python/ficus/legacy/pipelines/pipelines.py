import copy
from typing import Any, Callable

from ..util import random_unique_color_provider_instance
from .contexts.part_results import PipelinePartResult


class Pipeline:
    def __init__(self, *pipeline_parts):
        self.parts = _create_pipelines_list(list(pipeline_parts))

    def append(self, pipeline_part: 'PipelinePart'):
        self.parts.append(pipeline_part)

    def insert(self, index: int, pipeline_part: 'PipelinePart'):
        self.parts.insert(index, pipeline_part)

    def remove(self, index: int):
        del self.parts[index]

    def execute(self, input: Any):
        current_input = input
        random_unique_color_provider_instance.reset()
        for part in self.parts:
            current_input = part.execute(current_input)

        return current_input

    def __call__(self, *args, **kwargs):
        return self.execute(args[0] if len(args) != 0 else None)


class PipelinePart:
    def execute(self, current_input: Any) -> PipelinePartResult:
        raise NotImplementedError()

    def __call__(self, *args, **kwargs):
        return self.execute(args[0])


class FirstPipelinePart(PipelinePart):
    def execute(self, current_input: Any) -> PipelinePartResult:
        raise NotImplementedError()


class InternalPipelinePart(PipelinePart):
    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        raise NotImplementedError()


def _create_pipelines_list(pipeline_parts) -> list[PipelinePart]:
    parts: list[PipelinePart] = []
    for part in pipeline_parts:
        if part is None:
            continue

        if not isinstance(part, PipelinePart):
            raise TypeError()

        parts.append(part)

    return parts


class ParallelPipeline(InternalPipelinePart):
    def __init__(self, *parallel_pipelines):
        self.pipelines: list[Pipeline] = []
        for pipeline in parallel_pipelines:
            self.pipelines.append(pipeline)

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        last_output = None
        for pipeline in self.pipelines:
            last_output = pipeline.execute(copy.copy(current_input))

        return last_output


class WithTempInput(InternalPipelinePart):
    def __init__(self,
                 initial_input_transformer: Callable[[PipelinePartResult], PipelinePartResult],
                 output_merger: Callable[[PipelinePartResult, PipelinePartResult], PipelinePartResult],
                 pipeline: Pipeline):
        self.pipeline = pipeline
        self.initial_input_transformer = initial_input_transformer
        self.output_merger = output_merger

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        temp_input = self.initial_input_transformer(current_input)
        result = self.pipeline.execute(temp_input)
        return self.output_merger(current_input, result)


class WithInputCopy(InternalPipelinePart):
    def __init__(self, pipeline: Pipeline):
        self.pipeline = pipeline

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        current_input_copy = copy.copy(current_input)
        self.pipeline(current_input_copy)
        return current_input


Condition = Callable[[PipelinePartResult], bool]


class IfPipeline(InternalPipelinePart):
    def __init__(self, condition: Condition, pipeline: Pipeline):
        self.condition = condition
        self.pipeline = pipeline

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        if self.condition(current_input):
            return self.pipeline(current_input)

        return current_input
