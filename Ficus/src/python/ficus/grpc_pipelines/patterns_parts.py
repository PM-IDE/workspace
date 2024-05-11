from typing import Optional

from .constants import const_tandem_array_length, const_event_class_regex, \
    const_find_primitive_tandem_arrays, const_find_maximal_tandem_arrays, const_patterns_discovery_strategy, \
    const_find_maximal_repeats, const_find_super_maximal_repeats, const_find_near_super_maximal_repeats
from .data_models import PatternsDiscoveryStrategy
from .grpc_pipelines import PipelinePart, _create_default_pipeline_part, append_uint32_value, \
    append_string_value, append_patterns_discovery_strategy
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

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(self.part_type, config))


class FindPrimitiveTandemArrays2(FindTandemArrays):
    def __init__(self, max_array_length: int, class_extractor: Optional[str] = None):
        super().__init__(part_type=const_find_primitive_tandem_arrays,
                         max_array_length=max_array_length,
                         class_extractor=class_extractor)


class FindMaximalTandemArrays2(FindTandemArrays):
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

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(self.part_name, config))


class FindMaximalRepeats2(FindRepeats):
    def __init__(self,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        super().__init__(part_name=const_find_maximal_repeats,
                         strategy=strategy,
                         class_extractor=class_extractor)


class FindSuperMaximalRepeats2(FindRepeats):
    def __init__(self,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        super().__init__(part_name=const_find_super_maximal_repeats,
                         strategy=strategy,
                         class_extractor=class_extractor)


class FindNearSuperMaximalRepeats2(FindRepeats):
    def __init__(self,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        super().__init__(part_name=const_find_near_super_maximal_repeats,
                         strategy=strategy,
                         class_extractor=class_extractor)
