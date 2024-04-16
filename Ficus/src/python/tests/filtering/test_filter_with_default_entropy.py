from ...ficus.legacy.filtering.event_log_filters import next_event_to_remove_default_entropy_direct, \
    next_removal_default_entropy_indirect
from ...tests import log_creators
from ...ficus.legacy.analysis.event_log_info import *
from ...tests.filtering.util import create_sequence_of_removals


def test_direct_filtering():
    log = log_creators.create_log_from_filter_out_chaotic_events()
    sequence_of_removed_events = create_sequence_of_removals(log, next_event_to_remove_default_entropy_direct)
    assert sequence_of_removed_events == ['x', 'a', 'b', 'c']


def _next_remove_default_entropy_indirect(log: MyEventLog):
    return next_removal_default_entropy_indirect(log, top_percent=1)


def test_indirect_filtering():
    log = log_creators.create_log_from_filter_out_chaotic_events()
    sequence_of_removed_events = create_sequence_of_removals(log, _next_remove_default_entropy_indirect)
    assert sequence_of_removed_events == ['x', 'a', 'b', 'c']


def test_indirect_filtering_noise():
    log = log_creators.create_log_from_filter_out_chaotic_events_with_noise()
    sequence_of_removed_events = create_sequence_of_removals(log, _next_remove_default_entropy_indirect)
    assert sequence_of_removed_events == ['d', 'x', 'v', 'a', 'b', 'c']


def test_direct_filtering_with_noise():
    log = log_creators.create_log_from_filter_out_chaotic_events_with_noise()
    sequence_of_removed_events = create_sequence_of_removals(log, next_event_to_remove_default_entropy_direct)
    assert sequence_of_removed_events == ['d', 'x', 'a', 'b', 'c', 'v']
