import uuid
from dataclasses import dataclass
from typing import Optional

from ..legacy.analysis.event_log_analysis import draw_colors_event_log
from ..legacy.analysis.event_log_analysis_canvas import draw_colors_event_log_canvas
from ..legacy.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ..grpc_pipelines.constants import *
from ..grpc_pipelines.context_values import ContextValue, from_grpc_colors_log, \
    StringContextValue, Uint32ContextValue, BoolContextValue, EnumContextValue, from_grpc_event_log_info, \
    StringsContextValue, FloatContextValue
from ..grpc_pipelines.data_models import PatternsDiscoveryStrategy, PatternsKind, NarrowActivityKind, \
    ActivityFilterKind, ActivitiesLogsSource
from ..grpc_pipelines.models.backend_service_pb2 import *
from ..grpc_pipelines.models.backend_service_pb2_grpc import *
from ..grpc_pipelines.models.pipelines_and_context_pb2 import *
from ..grpc_pipelines.models.util_pb2 import *
from ..legacy.pipelines.analysis.patterns.models import AdjustingMode
from ..legacy.util import performance_cookie

ficus_backend_addr_key = 'backend'

class Pipeline2:
    def __init__(self, *parts):
        self.parts: list['PipelinePart'] = list(parts)

    def execute(self, initial_context: dict[str, ContextValue]) -> GrpcPipelinePartExecutionResult:
        options = [('grpc.max_send_message_length', 512 * 1024 * 1024),
                   ('grpc.max_receive_message_length', 512 * 1024 * 1024)]

        addr = initial_context[ficus_backend_addr_key] if ficus_backend_addr_key in initial_context else 'localhost:8080'
        if ficus_backend_addr_key in initial_context:
            del initial_context[ficus_backend_addr_key]

        with grpc.insecure_channel(addr, options=options) as channel:
            stub = GrpcBackendServiceStub(channel)
            parts = list(self.parts)
            request = GrpcPipelineExecutionRequest(
                pipeline=self._create_grpc_pipeline(parts),
                initialContext=self._create_initial_context(initial_context)
            )

            callback_parts = []
            self.append_parts_with_callbacks(callback_parts)
            uuid_to_pipeline_with_callback = {}
            for part in callback_parts:
                uuid_to_pipeline_with_callback[part.uuid] = part

            last_result = None

            for part_result in stub.ExecutePipeline(request):
                last_result = part_result

                if last_result.HasField('finalResult'):
                    break

                if last_result.HasField('pipelinePartResult'):
                    issued_part_uuid = uuid.UUID(part_result.pipelinePartResult.uuid.uuid)
                    if issued_part_uuid in uuid_to_pipeline_with_callback:
                        map = dict()
                        for context_value_with_name in part_result.pipelinePartResult.contextValues:
                            map[context_value_with_name.key_name] = context_value_with_name.value

                        part = uuid_to_pipeline_with_callback[issued_part_uuid]

                        def action():
                            part.execute_callback(map)

                        performance_cookie(f'{type(part).__name__}Callback', action)

                if last_result.HasField('logMessage'):
                    print(part_result.logMessage.message)

            if last_result.finalResult.HasField('success'):
                guid = last_result.finalResult.success
                stub.DropExecutionResult(guid)

            return last_result

    def to_grpc_pipeline(self):
        return self._create_grpc_pipeline(self.parts)

    def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
        for part in list(self.parts):
            part.append_parts_with_callbacks(parts)

    @staticmethod
    def _create_grpc_pipeline(parts) -> GrpcPipeline:
        pipeline = GrpcPipeline()
        for part in parts:
            if not isinstance(part, PipelinePart):
                raise TypeError()

            pipeline.parts.append(part.to_grpc_part())

        return pipeline

    @staticmethod
    def _find_pipeline_parts_with_callbacks(parts) -> list["PipelinePartWithCallback"]:
        result = []
        for part in parts:
            if isinstance(part, PipelinePartWithCallback):
                result.append(part)

        return result

    @staticmethod
    def _create_initial_context(context: dict[str, ContextValue]) -> list[GrpcContextKeyValue]:
        result = []
        for key, value in context.items():
            result.append(GrpcContextKeyValue(
                key=GrpcContextKey(name=key),
                value=value.to_grpc_context_value()
            ))

        return result


class PipelinePart:
    def __init__(self):
        self.uuid = uuid.uuid4()

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        raise NotImplementedError()

    def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
        pass


class PipelinePartWithCallback(PipelinePart):
    def execute_callback(self, values: dict[str, GrpcContextValue]):
        raise NotImplementedError()

    def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
        super().append_parts_with_callbacks(parts)
        parts.append(self)


class PipelinePart2WithDrawColorsLogCallback(PipelinePartWithCallback):
    def __init__(self,
                 title: Optional[str] = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__()
        self.title = title
        self.save_path = save_path
        self.plot_legend = plot_legend
        self.height_scale = height_scale
        self.width_scale = width_scale

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        colors_log = from_grpc_colors_log(values[const_colors_event_log].colors_log)
        draw_colors_event_log(colors_log,
                              title=self.title,
                              save_path=self.save_path,
                              plot_legend=self.plot_legend,
                              height_scale=self.height_scale,
                              width_scale=self.width_scale)


class PipelinePart2WithCanvasCallback(PipelinePartWithCallback):
    def __init__(self,
                 save_path: Optional[str] = None,
                 title: Optional[str] = None,
                 plot_legend: bool = False,
                 height_scale: float = 1,
                 width_scale: float = 1):
        super().__init__()
        self.save_path = save_path
        self.width_scale = width_scale
        self.height_scale = height_scale
        self.title = title
        self.plot_legend = plot_legend

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        colors_log = from_grpc_colors_log(values['colors_event_log'].colors_log)
        draw_colors_event_log_canvas(colors_log,
                                     title=self.title,
                                     plot_legend=self.plot_legend,
                                     save_path=self.save_path,
                                     height_scale=self.height_scale,
                                     width_scale=self.width_scale)


class PrintEventLogInfo2(PipelinePartWithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(self.uuid, [const_event_log_info], const_get_event_log_info, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        log_info = from_grpc_event_log_info(values[const_event_log_info].event_log_info)
        print(log_info)


def _create_simple_get_context_value_part(frontend_part_uuid: uuid.UUID, key_name: str):
    return GrpcSimpleContextRequestPipelinePart(
        frontendPartUuid=_create_grpc_uuid(frontend_part_uuid),
        key=GrpcContextKey(name=key_name)
    )


def _create_grpc_uuid(uuid: uuid.UUID) -> GrpcUuid:
    return GrpcUuid(uuid=str(uuid))


def _create_complex_get_context_part(frontend_part_uuid: uuid.UUID,
                                     key_names: list[str],
                                     before_part_name: str,
                                     config: GrpcPipelinePartConfiguration):
    return GrpcComplexContextRequestPipelinePart(
        frontendPartUuid=_create_grpc_uuid(frontend_part_uuid),
        keys=list(map(lambda x: GrpcContextKey(name=x), key_names)),
        beforePipelinePart=GrpcPipelinePart(
            name=before_part_name,
            configuration=config
        ),
    )


def _create_default_pipeline_part(name: str, config=GrpcPipelinePartConfiguration()):
    return GrpcPipelinePart(configuration=config, name=name)


def append_string_value(config: GrpcPipelinePartConfiguration, key: str, value: str):
    _append_context_value(config, key, StringContextValue(value))


def append_float_value(config: GrpcPipelinePartConfiguration, key: str, value: float):
    _append_context_value(config, key, FloatContextValue(value))


def _append_context_value(config: GrpcPipelinePartConfiguration, key: str, value: ContextValue):
    config.configurationParameters.append(GrpcContextKeyValue(
        key=GrpcContextKey(name=key),
        value=value.to_grpc_context_value()
    ))


def append_uint32_value(config: GrpcPipelinePartConfiguration, key: str, value: int):
    _append_context_value(config, key, Uint32ContextValue(value))


def append_bool_value(config: GrpcPipelinePartConfiguration, key: str, value: bool):
    _append_context_value(config, key, BoolContextValue(value))


def append_enum_value(config: GrpcPipelinePartConfiguration, key: str, enum_name: str, value: str):
    _append_context_value(config, key, EnumContextValue(enum_name, value))


def append_patterns_discovery_strategy(config: GrpcPipelinePartConfiguration, key: str,
                                       value: PatternsDiscoveryStrategy):
    append_enum_value(config, key, const_pattern_discovery_strategy_enum_name, value.name)


def append_strings_context_value(config: GrpcPipelinePartConfiguration, key: str, value: list[str]):
    _append_context_value(config, key, StringsContextValue(value))


def append_patterns_kind(config: GrpcPipelinePartConfiguration, key: str, value: PatternsKind):
    append_enum_value(config, key, const_patterns_kind_enum_name, value.name)


def append_adjusting_mode(config: GrpcPipelinePartConfiguration, key: str, value: AdjustingMode):
    append_enum_value(config, key, const_adjusting_mode_enum_name, value.name)


def append_pipeline_value(config: GrpcPipelinePartConfiguration, key: str, value: Pipeline2):
    _append_context_value(config, key, PipelineContextValue(value))


def append_narrow_kind(config: GrpcPipelinePartConfiguration, key: str, value: NarrowActivityKind):
    append_enum_value(config, key, const_narrow_activities_kind_enum_name, value.name)


def append_undef_activity_handling_strat(config: GrpcPipelinePartConfiguration,
                                         key: str,
                                         strat: UndefinedActivityHandlingStrategy):
    append_enum_value(config, key, const_undef_activity_handling_strat_name, strat.name)


def append_activity_filter_kind(config: GrpcPipelinePartConfiguration, key: str, filter_kind: ActivityFilterKind):
    append_enum_value(config, key, const_activity_filter_kind_enum_name, filter_kind.name)


def append_activities_logs_source(config: GrpcPipelinePartConfiguration, key: str, source: ActivitiesLogsSource):
    append_enum_value(config, key, const_activities_logs_source_enum_name, source.name)


@dataclass
class PipelineContextValue(ContextValue):
    pipeline: Pipeline2

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(pipeline=self.pipeline.to_grpc_pipeline())
