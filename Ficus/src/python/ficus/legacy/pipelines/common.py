from typing import Union, Optional

from ..pipelines.serialization.pipeline_parts import SavePathCreator

from .contexts.part_results import PipelinePartResult
from ..pipelines.pipelines import InternalPipelinePart


class InternalDrawingPipelinePart(InternalPipelinePart):
  def __init__(self,
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Optional[Union[str, SavePathCreator]] = None):
    self.title = title
    self.plot_legend = plot_legend
    self.height_scale = height_scale
    self.save_path = save_path
    self.width_scale = width_scale

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    raise NotImplementedError()

  def _get_save_path(self, current_input: PipelinePartResult) -> Optional[str]:
    if self.save_path is None:
      return None

    if type(self.save_path) == str:
      return self.save_path

    return self.save_path(current_input)
