from ...ficus.grpc_pipelines.entry_points.sreaming_configuration import *

const_not_specified_field = 'notSpecified'
const_t1_configuration_field = 't1Configuration'
const_t2_configuration_field = 't2Configuration'

all_fields = [
  const_not_specified_field,
  const_t1_configuration_field,
  const_t2_configuration_field
]

def test_not_specified_configuration():
  configuration = create_not_specified_configuration()
  _assert_single_has_field(configuration, const_not_specified_field)

def _assert_single_has_field(configuration, field_name: str):
  for field in all_fields:
    has_field = configuration.HasField(field)
    assert (not (has_field ^ (field == field_name)))

def test_t1_configuration():
  timeout = 123
  configuration = create_time_caching_configuration(timeout)

  _assert_single_has_field(configuration, const_t1_configuration_field)
  assert configuration.t1Configuration.timeBasedConfiguration.tracesTimeoutMs == timeout

def test_t2_configuration():
  error, support = 1, 2
  configuration = create_lossy_count_configuration(error, support)

  _assert_single_has_field(configuration, const_t2_configuration_field)
  assert configuration.t2Configuration.lossyCountConfiguration.error == error
  assert configuration.t2Configuration.lossyCountConfiguration.support == support
