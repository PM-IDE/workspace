from ..core.utils import maximal_repeats_to_substrings
from ...ficus.legacy.analysis.patterns.event_log_patterns import *
from ...ficus.legacy.analysis.patterns.util import create_activity_name_from_log
from ...ficus.legacy.log.functions import parse_log_from_strings
from ...tests import log_creators


def test_tandem_arrays_from_paper():
    event_log = log_creators.create_tandem_array_log_taxonomy_of_patterns()
    arrays = find_maximal_tandem_arrays(event_log)
    tuples = []
    for trace_array in arrays:
        for array in trace_array:
            tuples.append((array.first_pos, array.length, array.repeat_count))

    assert tuples == [(2, 3, 4), (3, 3, 4), (4, 3, 3), (2, 6, 2), (3, 6, 2)]


def test_primitive_tandem_arrays_from_paper():
    raw_log = ['g,d,a,b,c,a,b,c,a,b,c,a,b,c,a,f,i,c,a']
    event_log = parse_log_from_strings(raw_log)
    arrays = find_primitive_tandem_arrays(event_log)
    tuples = []
    for trace_array in arrays:
        for array in trace_array:
            tuples.append((array.first_pos, array.length, array.repeat_count))

    assert tuples == [(2, 3, 4), (3, 3, 4), (4, 3, 3)]


def test_tandem_arrays_from_paper_2():
    log_str = log_creators.create_tandem_array_raw_string_taxonomy_of_patterns()
    event_log = log_creators.create_tandem_array_log_taxonomy_of_patterns()
    arrays = find_maximal_tandem_arrays(event_log)

    primitive_tandem_arrays = []
    for trace_array in arrays:
        for array in trace_array:
            primitive_tandem_arrays.append(log_str[array.first_pos:(array.first_pos + array.length)])

    assert primitive_tandem_arrays == ['abc', 'bca', 'cab', 'abcabc', 'bcabca']


def test_primitive_tandem_arrays_from_paper_2():
    log_str = log_creators.create_tandem_array_raw_string_taxonomy_of_patterns()
    event_log = log_creators.create_tandem_array_log_taxonomy_of_patterns()
    arrays = find_primitive_tandem_arrays(event_log)

    primitive_tandem_arrays = []
    for trace_array in arrays:
        for array in trace_array:
            primitive_tandem_arrays.append(log_str[array.first_pos:(array.first_pos + array.length)])

    assert primitive_tandem_arrays == ['abc', 'bca', 'cab']


def test_no_tandem_arrays():
    raw_event_log = 'a,b,c,d'
    event_log = parse_log_from_strings([raw_event_log])
    arrays = find_maximal_tandem_arrays(event_log)
    assert len(arrays[0]) == 0


def test_one_tandem_array():
    raw_events = 'ababcd'
    raw_event_log = log_creators.insert_separator(raw_events)
    event_log = parse_log_from_strings([raw_event_log])
    arrays = find_maximal_tandem_arrays(event_log)
    assert len(arrays[0]) == 1
    assert raw_events[arrays[0][0].first_pos:(arrays[0][0].first_pos + arrays[0][0].length)] == 'ab'


def test_maximal_repeat_set():
    raw_events = 'abcdxabcyabcz'
    raw_event_log = parse_log_from_strings([log_creators.insert_separator(raw_events)])
    maximal_repeat_set_by_traces = find_maximal_repeats(raw_event_log)

    substrings = []
    for trace_repeats in maximal_repeat_set_by_traces:
        for repeat in trace_repeats:
            substrings.append(raw_events[repeat.first_pos:(repeat.first_pos + repeat.length)])

    assert substrings == ['abc']


def test_maximal_repeat_from_taxonomy_of_patterns():
    raw_events = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_logs = list(map(log_creators.insert_separator, raw_events))
    event_log = parse_log_from_strings(raw_logs)
    maximal_repeat_set_by_traces = find_maximal_repeats(event_log)
    substrings = maximal_repeats_to_substrings(raw_events, maximal_repeat_set_by_traces)

    assert substrings == [['a', 'b', 'bcd'],
                          ['dabc', 'b'],
                          ['b', 'bb', 'bbbc', 'c', 'a'],
                          ['a', 'aa', 'b', 'c', 'cc'],
                          ['a', 'aa', 'c', 'cdc', 'cb', 'd', 'dc', 'db', 'b', 'bd', 'e']]


def test_super_maximal_repeat_from_taxonomy_of_patterns():
    raw_events = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_logs = list(map(log_creators.insert_separator, raw_events))
    event_log = parse_log_from_strings(raw_logs)
    super_maximal_repeats = find_super_maximal_repeats(event_log)
    substrings = maximal_repeats_to_substrings(raw_events, super_maximal_repeats)

    assert substrings == [['a', 'bcd'],
                          ['dabc'],
                          ['bbbc', 'a'],
                          ['aa', 'b', 'cc'],
                          ['aa', 'cdc', 'cb', 'db', 'bd', 'e']]


def test_near_super_maximal_repeat_from_taxonomy_of_patterns():
    raw_events = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_logs = list(map(log_creators.insert_separator, raw_events))
    event_log = parse_log_from_strings(raw_logs)
    near_super_maximal_repeats = find_near_super_maximal_repeats(event_log)
    substrings = maximal_repeats_to_substrings(raw_events, near_super_maximal_repeats)

    assert substrings == [['a', 'b', 'bcd'],
                          ['dabc', 'b'],
                          ['bbbc', 'c', 'a'],
                          ['a', 'aa', 'b', 'cc'],
                          ['a', 'aa', 'c', 'cdc', 'cb', 'dc', 'db', 'bd', 'e']]


def _create_substrings(event_log: MyEventLog, items: list[SubArrayWithTraceIndex]) -> list[list[str]]:
    substrings = []
    for item in items:
        trace = event_log[item.trace_index]
        substrings.append(list(map(lambda x: x[concept_name], trace[item.first_pos:(item.first_pos + item.length)])))

    return substrings


def test_creating_repeat_set_from_tandem_array():
    raw_logs = [log_creators.create_tandem_array_raw_string_taxonomy_of_patterns()]
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    _do_test_with_repeat_set(raw_events, [['a', 'b', 'c']], find_primitive_tandem_arrays)


def _do_test_with_repeat_set(raw_events: list[str],
                             expected_sets: list[list[str]],
                             func: Callable[[MyEventLog], list[list[SubArrayInEventLog]]]):
    event_log = parse_log_from_strings(raw_events)
    tandem_arrays = func(event_log)
    repeat_sets = create_repeat_sets(event_log, tandem_arrays)
    substrings = _create_substrings(event_log, repeat_sets)

    assert substrings == expected_sets


def test_creating_repeat_set_from_tandem_array2():
    raw_logs = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = [['d', 'a', 'b', 'c'], ['c', 'd']]
    _do_test_with_repeat_set(raw_events, expected, find_primitive_tandem_arrays)


def test_creating_repeat_set_from_super_maximal_repeats():
    raw_logs = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = [['a'], ['b', 'c', 'd'], ['d', 'a', 'b', 'c'],
                ['b', 'b', 'b', 'c'], ['b'], ['c', 'c'], ['c', 'd', 'c'],
                ['d', 'b'], ['e']]

    _do_test_with_repeat_set(raw_events, expected, find_super_maximal_repeats)


def test_creating_repeat_set_from_maximal_repeats():
    raw_logs = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = [['a'], ['b'], ['b', 'c', 'd'], ['d', 'a', 'b', 'c'],
                ['b', 'b', 'b', 'c'], ['c'], ['c', 'd', 'c'], ['d'], ['d', 'b'], ['e']]

    _do_test_with_repeat_set(raw_events, expected, find_maximal_repeats)


def _do_test_with_abstractions(raw_events: list[str],
                               expected_sets: list[str],
                               func: Callable[[MyEventLog], list[list[SubArrayInEventLog]]]):
    event_log = parse_log_from_strings(raw_events)
    tandem_arrays = func(event_log)
    repeat_sets = create_repeat_sets(event_log, tandem_arrays)

    def create_activity_name(sub_array: SubArrayWithTraceIndex):
        return create_activity_name_from_log(event_log, sub_array, default_class_extractor)

    abstractions = build_repeat_set_tree(event_log, repeat_sets, create_activity_name, activity_level=0)
    substrings = list(sorted(map(lambda x: x.name, abstractions)))

    assert substrings == expected_sets


def test_creating_abstractions_from_tandem_arrays():
    raw_logs = [log_creators.create_tandem_array_raw_string_taxonomy_of_patterns()]
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = ['a::b::c']
    _do_test_with_abstractions(raw_events, expected, find_primitive_tandem_arrays)


def test_creating_abstractions_from_tandem_arrays2():
    raw_logs = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = ['a::b::c::d']
    _do_test_with_abstractions(raw_events, expected, find_primitive_tandem_arrays)


def test_creating_abstractions_from_super_maximal_repeats():
    raw_logs = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = ['a::b::c::d', 'e']
    _do_test_with_abstractions(raw_events, expected, find_super_maximal_repeats)


def test_creating_abstractions_from_maximal_repeats():
    raw_logs = log_creators.create_list_of_raw_events_for_maximal_repeat()
    raw_events = list(map(log_creators.insert_separator, raw_logs))
    expected = ['a::b::c::d', 'e']
    _do_test_with_abstractions(raw_events, expected, find_maximal_repeats)


def test_maximal_repeats_from_single_merged_trace():
    current_log = log_creators.create_single_merged_trace_maximal_repeats_log()
    raw_events = log_creators.create_single_merged_trace_maximal_repeat_traces()

    near_super_maximal_repeats = find_maximal_repeats(current_log)
    substrings = maximal_repeats_to_substrings(raw_events, near_super_maximal_repeats)
    assert substrings == [['a', 'aa', 'aaa', 'ab', 'abc', 'abcd', 'ad', 'b', 'bc',
                           'bcd', 'bcdbb', 'bcda', 'bcc', 'bb', 'bbc', 'bbcd', 'bbcc',
                           'bbbc', 'bd', 'c', 'cd', 'cdc', 'cb', 'cc', 'd', 'db', 'da',
                           'dab', 'dabc', 'dc', 'e']]


def test_super_maximal_repeats_from_single_merged_trace():
    current_log = log_creators.create_single_merged_trace_maximal_repeats_log()
    raw_events = log_creators.create_single_merged_trace_maximal_repeat_traces()

    near_super_maximal_repeats = find_super_maximal_repeats(current_log)
    substrings = maximal_repeats_to_substrings(raw_events, near_super_maximal_repeats)
    assert substrings == [['aaa', 'abcd', 'ad', 'bcdbb', 'bcda', 'bbcd',
                           'bbcc', 'bbbc', 'bd', 'cdc', 'cb', 'dabc', 'e']]


def test_near_super_maximal_repeats_from_single_merged_trace():
    current_log = log_creators.create_single_merged_trace_maximal_repeats_log()
    raw_events = log_creators.create_single_merged_trace_maximal_repeat_traces()

    near_super_maximal_repeats = find_near_super_maximal_repeats(current_log)
    substrings = maximal_repeats_to_substrings(raw_events, near_super_maximal_repeats)
    assert substrings == [['aa', 'aaa', 'abcd', 'ad', 'bcdbb', 'bcda', 'bcc', 'bb',
                           'bbcd', 'bbcc', 'bbbc', 'bd', 'cdc', 'cb', 'cc', 'db', 'dab',
                           'dabc', 'dc', 'e']]
