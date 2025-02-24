from ...ficus.grpc_pipelines.context_values import *


def test_string_context_value():
  value = "asdasdasdsdasda"
  context_value = StringContextValue(value).to_grpc_context_value()
  assert value == context_value.string


def test_uint32_context_value():
  value = 123123
  context_value = Uint32ContextValue(value).to_grpc_context_value()
  assert value == context_value.uint32


def test_bool_context_value():
  value = True
  context_value = BoolContextValue(value).to_grpc_context_value()
  assert value == context_value.bool


def test_names_log_context_value():
  raw_log = [["asdasdasdads", "asdasdasd"]]
  context_value = NamesLogContextValue(raw_log).to_grpc_context_value()
  assert raw_log[0][0] == context_value.names_log.log.traces[0].events[0]
  assert raw_log[0][1] == context_value.names_log.log.traces[0].events[1]


def test_hashes_log_context_value():
  raw_log = [[12312312, 323123123]]
  context_value = HashesLogContextValue(raw_log).to_grpc_context_value()
  assert raw_log[0][0] == context_value.hashes_log.log.traces[0].events[0]
  assert raw_log[0][1] == context_value.hashes_log.log.traces[0].events[1]


def test_enum_context_value():
  enum_name = 'ASDASDSDSD'
  enum_value = 'ASDasdasdasdadasd'
  context_value = EnumContextValue(enum_name=enum_name, value=enum_value).to_grpc_context_value()
  assert enum_name == context_value.enum.enumType
  assert enum_value == context_value.enum.value


def test_strings_context_value():
  raw_strings = ['12312312', '323123123']
  context_value = StringsContextValue(raw_strings).to_grpc_context_value()
  assert raw_strings[0] == context_value.strings.strings[0]
  assert raw_strings[1] == context_value.strings.strings[1]
