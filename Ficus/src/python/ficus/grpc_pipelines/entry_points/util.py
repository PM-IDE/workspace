import uuid
from typing import Callable, Any

from grpc import Channel

from ..models.context_values_service_pb2 import *
from ..models.context_values_service_pb2_grpc import *
from ..models.util_pb2 import *
from ...grpc_pipelines.context_values import ContextValue
from ...grpc_pipelines.models.backend_service_pb2_grpc import *
from ...grpc_pipelines.models.pipelines_and_context_pb2 import *
from ...legacy.util import performance_cookie


def create_ficus_grpc_channel(ficus_backend_addr: str) -> grpc.Channel:
  options = [('grpc.max_send_message_length', 512 * 1024 * 1024),
             ('grpc.max_receive_message_length', 512 * 1024 * 1024)]

  return grpc.insecure_channel(ficus_backend_addr, options=options)


def create_initial_context(context: dict[str, ContextValue]) -> list[GrpcContextKeyValue]:
  result = []
  for key, value in context.items():
    result.append(GrpcContextKeyValue(
      key=GrpcContextKey(name=key),
      value=value.to_grpc_context_value()
    ))

  return result


def process_single_pipeline_output_stream(uuid_to_pipeline_with_callback, stream):
  last_result = None

  for part_result in stream:
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

  return last_result


def create_pipeline_callbacks_map(callback_parts):
  uuid_to_pipeline_with_callback = {}
  for part in callback_parts:
    uuid_to_pipeline_with_callback[part.uuid] = part

  return uuid_to_pipeline_with_callback


def process_multiple_pipelines_output_stream(callback_parts, stream):
  uuid_to_pipeline_with_callback = create_pipeline_callbacks_map(callback_parts)
  while True:
    print(process_single_pipeline_output_stream(uuid_to_pipeline_with_callback, stream))


def process_pipeline_output_stream(callback_parts, stream):
  uuid_to_pipeline_with_callback = create_pipeline_callbacks_map(callback_parts)
  return process_single_pipeline_output_stream(uuid_to_pipeline_with_callback, stream)


def append_parts_with_callbacks(original_parts, callback_parts: list['PipelinePartWithCallback']):
  for part in list(original_parts):
    if isinstance(part, list):
      for pipeline_part in part:
        pipeline_part.append_parts_with_callbacks(callback_parts)

      continue

    part.append_parts_with_callbacks(callback_parts)


def execute_with_context_values(channel: Channel,
                                initial_context: dict[str, ContextValue],
                                action: Callable[[list[GrpcGuid]], Any]):
  cv_service = GrpcContextValuesServiceStub(channel)
  ids = set_initial_context(cv_service, initial_context)

  try:
    return action(ids)
  finally:
    cv_service.DropContextValues(GrpcDropContextValuesRequest(ids=ids))


def set_initial_context(cv_service: GrpcContextValuesServiceStub, context: dict[str, ContextValue]) -> list[GrpcGuid]:
  ids = []
  for key, value in context.items():
    ids.append(set_context_value(cv_service, key, value))

  return ids


def set_context_value(cv_service: GrpcContextValuesServiceStub, key: str, value: ContextValue):
  message_bytes = bytes(value.to_grpc_context_value().SerializeToString())
  chunk_length = 1024 * 16
  index = 0

  cv_parts = []

  while index + chunk_length < len(message_bytes):
    current_bytes = message_bytes[index:(index + chunk_length)]
    cv_parts.append(GrpcContextValuePart(
      bytes=current_bytes,
      key=key
    ))

    index += chunk_length

  if index < len(message_bytes):
    cv_parts.append(GrpcContextValuePart(
      bytes=message_bytes[index:],
      key=key
    ))

  return cv_service.SetContextValue(iter(cv_parts))
