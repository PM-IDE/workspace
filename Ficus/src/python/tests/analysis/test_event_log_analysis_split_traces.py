import os.path

import pytest

from ..test_data_provider import sources_dir
from ...ficus.legacy.analysis.event_log_split import split_log_by_traces
from ...ficus.legacy.log.functions import read_log_from_xes


@pytest.mark.skip(reason='Too big source')
def test():
  source = [
    ('consoleapp1', 10),
    ('dynamicassemblycreation', 10),
    ('lohallocations', 10),
    ('dynamicassemblyloading', 10),
    ('notexistingassemblyloading', 10),
    ('exceptiontrycatchfinally', 10),
    ('exceptiontrycatchfinallywhen', 10),
    ('systemarraypooling', 10),
    ('filewriteproject', 10),
    ('finalizableobject', 10),
    ('unsafefixed', 10),
    ('consoleapp1main', 69)
  ]

  for solution_name, expected_events_groups_counts in source:
    path = os.path.join(sources_dir(), 'test_split_traces', solution_name, 'UndefinedEvents.xes')
    log = read_log_from_xes(path)
    events_types = split_log_by_traces(log)
    assert len(events_types) == expected_events_groups_counts
