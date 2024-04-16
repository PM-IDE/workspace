from ...log.event_log import MyEventLog
from ...log.functions import read_log_from_xes
from ...pipelines.pipelines import *


class ReadLogFromXes(FirstPipelinePart):
    def execute(self, current_input: Any) -> PipelinePartResult:
        if not isinstance(current_input, str):
            raise TypeError()

        return PipelinePartResult().with_log(read_log_from_xes(current_input))


class UseExistingLog(FirstPipelinePart):
    def execute(self, current_input: Any) -> PipelinePartResult:
        if not isinstance(current_input, MyEventLog):
            raise ValueError()

        return PipelinePartResult().with_log(current_input)


class EmptyStartPart(FirstPipelinePart):
    def execute(self, current_input: Any) -> PipelinePartResult:
        return PipelinePartResult()
