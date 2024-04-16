from ...ficus.legacy.filtering.event_log_filters import next_event_remove_pos_entropy_direct, \
    next_event_remove_pos_entropy_indirect
from ...tests.log_creators import *
from ...tests.filtering.util import create_sequence_of_removals


def test_direct_filtering():
    log = create_log_from_filter_out_chaotic_events()
    sequence_of_removed_events = create_sequence_of_removals(log, next_event_remove_pos_entropy_direct)
    assert sequence_of_removed_events == ['x', 'a', 'b', 'c']


def test_indirect_filtering():
    log = create_log_from_filter_out_chaotic_events()
    sequence_of_removed_events = create_sequence_of_removals(log, next_event_remove_pos_entropy_indirect)
    assert sequence_of_removed_events == ['x', 'a', 'b', 'c']


def test_indirect_filtering_noise():
    log = create_log_from_filter_out_chaotic_events_with_noise()
    sequence_of_removed_events = create_sequence_of_removals(log, next_event_remove_pos_entropy_indirect)
    assert sequence_of_removed_events == ['x', 'v', 'd', 'a', 'b', 'c']


def test_direct_filtering_with_noise():
    log = create_log_from_filter_out_chaotic_events_with_noise()
    sequence_of_removed_events = create_sequence_of_removals(log, next_event_remove_pos_entropy_direct)
    assert sequence_of_removed_events == ['v', 'a', 'c', 'x', 'd', 'b']
