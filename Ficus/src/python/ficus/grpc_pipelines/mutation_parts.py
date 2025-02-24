from typing import Optional

from .constants import *
from .entry_points.default_pipeline import PipelinePart, create_default_pipeline_part, append_strings_context_value
from .models.pipelines_and_context_pb2 import *


class AddArtificialEventsBase(PipelinePart):
  def __init__(self, pipeline_part_name: str, attributes_to_copy: Optional[list[str]]):
    super().__init__()
    self.attributes_to_copy = attributes_to_copy
    self.pipeline_part_name = pipeline_part_name

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    if self.attributes_to_copy is not None:
      append_strings_context_value(config, const_attributes, self.attributes_to_copy)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_add_start_end_artificial_events, config))


class AddStartEndArtificialEvents(AddArtificialEventsBase):
  def __init__(self, attributes_to_copy: Optional[list[str]] = None):
    super().__init__(const_add_start_end_artificial_events, attributes_to_copy)


class AddStartArtificialEvents(AddArtificialEventsBase):
  def __init__(self, attributes_to_copy: Optional[list[str]] = None):
    super().__init__(const_add_start_artificial_events, attributes_to_copy)


class AddEndArtificialEvents(AddArtificialEventsBase):
  def __init__(self, attributes_to_copy: Optional[list[str]] = None):
    super().__init__(const_add_end_artificial_events, attributes_to_copy)


class AppendAttributesToName(PipelinePart):
  def __init__(self, attributes: list[str]):
    super().__init__()
    self.attributes = attributes

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_strings_context_value(config, const_attributes, self.attributes)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_append_attributes_to_name, config))
