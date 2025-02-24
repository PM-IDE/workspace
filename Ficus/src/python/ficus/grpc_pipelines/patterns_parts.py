from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration


class FindTandemArrays(PipelinePart):
  def __init__(self,
               part_type: str,
               max_array_length: int,
               class_extractor: Optional[str]):
    super().__init__()
    self.max_array_length = max_array_length
    self.part_type = part_type
    self.class_extractor = class_extractor

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_uint32_value(config, const_tandem_array_length, self.max_array_length)
    if self.class_extractor is not None:
      append_string_value(config, const_event_class_regex, self.class_extractor)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(self.part_type, config))


class FindPrimitiveTandemArrays(FindTandemArrays):
  def __init__(self, max_array_length: int, class_extractor: Optional[str] = None):
    super().__init__(part_type=const_find_primitive_tandem_arrays,
                     max_array_length=max_array_length,
                     class_extractor=class_extractor)


class FindMaximalTandemArrays(FindTandemArrays):
  def __init__(self, max_array_length: int, class_extractor: Optional[str] = None):
    super().__init__(part_type=const_find_maximal_tandem_arrays,
                     max_array_length=max_array_length,
                     class_extractor=class_extractor)


class FindRepeats(PipelinePart):
  def __init__(self,
               part_name: str,
               strategy: PatternsDiscoveryStrategy,
               class_extractor: Optional[str] = None):
    super().__init__()
    self.strategy = strategy
    self.part_name = part_name
    self.class_extractor = class_extractor

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)
    if self.class_extractor is not None:
      append_string_value(config, const_event_class_regex, self.class_extractor)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(self.part_name, config))


class FindMaximalRepeats(FindRepeats):
  def __init__(self,
               strategy: PatternsDiscoveryStrategy,
               class_extractor: Optional[str] = None):
    super().__init__(part_name=const_find_maximal_repeats,
                     strategy=strategy,
                     class_extractor=class_extractor)


class FindSuperMaximalRepeats(FindRepeats):
  def __init__(self,
               strategy: PatternsDiscoveryStrategy,
               class_extractor: Optional[str] = None):
    super().__init__(part_name=const_find_super_maximal_repeats,
                     strategy=strategy,
                     class_extractor=class_extractor)


class FindNearSuperMaximalRepeats(FindRepeats):
  def __init__(self,
               strategy: PatternsDiscoveryStrategy,
               class_extractor: Optional[str] = None):
    super().__init__(part_name=const_find_near_super_maximal_repeats,
                     strategy=strategy,
                     class_extractor=class_extractor)
