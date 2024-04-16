from typing import Callable

from ..contexts.accessors import log
from ..contexts.part_results import PipelinePartResult
from ...log.functions import save_event_log_to_xes
from ...pipelines.pipelines import InternalPipelinePart

SavePathCreator = Callable[[PipelinePartResult], str]


class SaveEventLogToXes(InternalPipelinePart):
    def __init__(self, save_path_creator: SavePathCreator):
        self.save_path_creator = save_path_creator

    def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
        save_event_log_to_xes(log(current_input), self.save_path_creator(current_input))
        return current_input
