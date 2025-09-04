import dataclasses

import docker
from docker import DockerClient
from docker.errors import DockerException
from docker.models.containers import Container

from .util import *
from ...grpc_pipelines.constants import *
from ...grpc_pipelines.context_values import *
from ...grpc_pipelines.data_models import PatternsDiscoveryStrategy, PatternsKind, NarrowActivityKind, \
  ActivityFilterKind, \
  ActivitiesLogsSource, RootSequenceKind
from ...grpc_pipelines.models.backend_service_pb2 import *
from ...grpc_pipelines.models.pipelines_and_context_pb2 import *
from ...legacy.analysis.event_log_analysis import draw_colors_event_log
from ...legacy.analysis.event_log_analysis_canvas import draw_colors_event_log_canvas
from ...legacy.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ...legacy.pipelines.analysis.patterns.models import AdjustingMode


@dataclasses.dataclass
class ContainerCreationResult:
  id: str
  port: int


class Pipeline:
  def __init__(self, *parts):
    self.parts: list['PipelinePart'] = list(parts)

  def execute_docker(self, initial_context: dict[str, ContextValue]) -> Optional[GrpcPipelinePartExecutionResult]:
    try:
      client = docker.from_env()
    except DockerException as err:
      print(f"Failed to create docker client, please ensure that docker is installed and up and running, {err}")
      return None

    res: Optional[ContainerCreationResult] = None
    try:
      try:
        res = _create_and_run_container(client)
        self.execute(f'localhost:{res.port}', initial_context)
      except Exception as err:
        print("Failed to start a ficus backend container", err)
        return None
    finally:
      if res is not None:
        try:
          _terminate_container(res.id, client)
        except Exception as err:
          print(f'Failed to terminate container {err}')

  def execute(self, ficus_backend: str, initial_context: dict[str, ContextValue]) -> Optional[GrpcPipelinePartExecutionResult]:
    with create_ficus_grpc_channel(ficus_backend) as channel:
      def action(ids):
        stub = GrpcBackendServiceStub(channel)
        parts = list(self.parts)
        request = GrpcProxyPipelineExecutionRequest(
          pipeline=create_grpc_pipeline(parts),
          contextValuesIds=ids
        )

        callback_parts = []
        append_parts_with_callbacks(list(self.parts), callback_parts)
        last_result = process_pipeline_output_stream(callback_parts, stub.ExecutePipeline(request))

        if last_result.finalResult.HasField('success'):
          guid = last_result.finalResult.success
          stub.DropExecutionResult(guid)
        elif last_result.finalResult.HasField('error'):
          print(f'ERROR: {last_result.finalResult.error}')

        return last_result

      return execute_with_context_values(channel, initial_context, action)

  def to_grpc_pipeline(self):
    return create_grpc_pipeline(self.parts)

  @staticmethod
  def _find_pipeline_parts_with_callbacks(parts) -> list["PipelinePartWithCallback"]:
    result = []
    for part in parts:
      if isinstance(part, PipelinePartWithCallback):
        result.append(part)

    return result

def _create_and_run_container(client: DockerClient) -> Optional[ContainerCreationResult]:
  image_name = 'aerooneqq/ficus'
  image_version = '1.0.3'

  client.images.pull('aerooneqq/ficus', image_version)

  container_internal_port = '8080'
  container = client.containers.run(f'{image_name}:{image_version}',
                                    detach=True,
                                    ports={container_internal_port: 0})

  print(f"Created container for ficus backend: {container.id}, {container.name}")

  container_port_key = f'{container_internal_port}/tcp'
  while not _contains_port(container, container_port_key):
    container.reload()

  return ContainerCreationResult(container.id, container.ports[container_port_key][0]['HostPort'])

def _contains_port(container: Container, container_port_key: str):
  return not (len(container.ports) == 0 or
              container_port_key not in container.ports or
              len(container.ports[container_port_key]) == 0)

def _terminate_container(container_id: str, client: DockerClient):
  container = client.containers.get(container_id)
  if container is None:
    print(f'The container with {container_id} does not exist, can not terminate or remove it')
    return

  try:
    container.stop()
    print(f'Terminated container for ficus backend: {container_id}')
  except Exception as err:
    print(f'Failed to stop container {container_id}, {err}')

  try:
    container.remove(force=True, v=True)
    print(f'Removed container {container_id}')
  except Exception as err:
    print(f'Failed to remove container {container_id}, {err}')


class PipelinePart:
  def __init__(self):
    self.uuid = uuid.uuid4()

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    raise NotImplementedError()

  def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
    pass


def create_grpc_pipeline(parts) -> GrpcPipeline:
  pipeline = GrpcPipeline()
  for part in parts:
    if isinstance(part, list):
      for list_part in part:
        pipeline.parts.append(list_part.to_grpc_part())

      continue

    if not isinstance(part, PipelinePart):
      raise TypeError()

    pipeline.parts.append(part.to_grpc_part())

  return pipeline


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
               height_scale: float = 1,
               width_scale: float = 1):
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
    draw_colors_event_log_canvas(values['colors_event_log'].colors_log,
                                 title=self.title,
                                 plot_legend=self.plot_legend,
                                 save_path=self.save_path,
                                 height_scale=self.height_scale,
                                 width_scale=self.width_scale)


class PrintEventLogInfo(PipelinePartWithCallback):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_event_log_info],
                                           const_get_event_log_info,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    log_info = from_grpc_event_log_info(values[const_event_log_info].event_log_info)
    print(log_info)


@dataclass
class PipelineContextValue(ContextValue):
  pipeline: Pipeline

  def to_grpc_context_value(self) -> GrpcContextValue:
    return GrpcContextValue(pipeline=self.pipeline.to_grpc_pipeline())


def create_simple_get_context_value_part(frontend_part_uuid: uuid.UUID, frontend_pipeline_part_name: str,
                                         key_name: str):
  return GrpcSimpleContextRequestPipelinePart(
    frontendPipelinePartName=frontend_pipeline_part_name,
    frontendPartUuid=create_grpc_guid(frontend_part_uuid),
    key=GrpcContextKey(name=key_name)
  )


def create_grpc_guid(uuid: uuid.UUID):
  return GrpcGuid(guid=str(uuid))


def create_complex_get_context_part(frontend_part_uuid: uuid.UUID,
                                    frontend_pipeline_part_name: str,
                                    key_names: list[str],
                                    before_part_name: str,
                                    config: GrpcPipelinePartConfiguration):
  return GrpcComplexContextRequestPipelinePart(
    frontendPipelinePartName=frontend_pipeline_part_name,
    frontendPartUuid=create_grpc_guid(frontend_part_uuid),
    keys=list(map(lambda x: GrpcContextKey(name=x), key_names)),
    beforePipelinePart=GrpcPipelinePart(
      name=before_part_name,
      configuration=config
    ),
  )


def create_default_pipeline_part(name: str, config=GrpcPipelinePartConfiguration()):
  return GrpcPipelinePart(configuration=config, name=name)


def append_string_value(config: GrpcPipelinePartConfiguration, key: str, value: str):
  append_context_value(config, key, StringContextValue(value))


def append_float_value(config: GrpcPipelinePartConfiguration, key: str, value: float):
  append_context_value(config, key, FloatContextValue(value))


def append_float_array_value(config: GrpcPipelinePartConfiguration, key: str, value: list[float]):
  append_context_value(config, key, FloatArrayContextValue(value))


def append_int_array_value(config: GrpcPipelinePartConfiguration, key: str, value: list[int]):
  append_context_value(config, key, IntArrayContextValue(value))


def append_uint_array_value(config: GrpcPipelinePartConfiguration, key: str, value: list[int]):
  append_context_value(config, key, UintArrayContextValue(value))


def append_context_value(config: GrpcPipelinePartConfiguration, key: str, value: ContextValue):
  config.configurationParameters.append(GrpcContextKeyValue(
    key=GrpcContextKey(name=key),
    value=value.to_grpc_context_value()
  ))


def append_uint32_value(config: GrpcPipelinePartConfiguration, key: str, value: int):
  append_context_value(config, key, Uint32ContextValue(value))


def append_bool_value(config: GrpcPipelinePartConfiguration, key: str, value: bool):
  append_context_value(config, key, BoolContextValue(value))


def append_enum_value(config: GrpcPipelinePartConfiguration, key: str, enum_name: str, value: str):
  append_context_value(config, key, EnumContextValue(enum_name, value))


def append_root_sequence_kind(config: GrpcPipelinePartConfiguration, key: str, value: RootSequenceKind):
  append_enum_value(config, key, const_root_sequence_kind_enum_name, value.name)


def append_patterns_discovery_strategy(config: GrpcPipelinePartConfiguration, key: str,
                                       value: PatternsDiscoveryStrategy):
  append_enum_value(config, key, const_pattern_discovery_strategy_enum_name, value.name)


def append_strings_context_value(config: GrpcPipelinePartConfiguration, key: str, value: list[str]):
  append_context_value(config, key, StringsContextValue(value))


def append_patterns_kind(config: GrpcPipelinePartConfiguration, key: str, value: PatternsKind):
  append_enum_value(config, key, const_patterns_kind_enum_name, value.name)


def append_adjusting_mode(config: GrpcPipelinePartConfiguration, key: str, value: AdjustingMode):
  append_enum_value(config, key, const_adjusting_mode_enum_name, value.name)


def append_pipeline_value(config: GrpcPipelinePartConfiguration, key: str, value: Pipeline):
  append_context_value(config, key, PipelineContextValue(value))


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


def append_json_value(config: GrpcPipelinePartConfiguration, key: str, json_string: str):
  config.configurationParameters.append(GrpcContextKeyValue(
    key=GrpcContextKey(name=key),
    value=GrpcContextValue(json=json_string)
  ))
