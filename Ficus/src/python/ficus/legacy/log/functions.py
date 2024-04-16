import os.path
import xml.etree.ElementTree as ET
from datetime import datetime
from typing import Union, Any

from dateutil import parser

from .event_log import MyEventLog, MyEvent, MyTrace
from ..util import concept_name

const_tag_log = 'log'
const_tag_trace = 'trace'
const_tag_extension = 'extension'
const_tag_classifier = 'classifier'

const_tag_date = 'date'
const_tag_string = 'string'
const_tag_bool = 'boolean'
const_tag_int = 'int'
const_tag_float = 'float'

const_tag_global = 'global'
const_tag_event = 'event'
const_attr_key = 'key'
const_attr_value = 'value'

const_scope = 'scope'

const_ext_name = 'name'
const_ext_prefix = 'prefix'
const_ext_uri = 'uri'

const_classifier_name = 'name'
const_classifier_keys = 'keys'

default_separator = ','
default_trace_separator = '\n'
default_events_separator = ','
default_logs_separator = '\n\n'


def read_log_from_xes(path: str) -> MyEventLog:
    if not os.path.exists(path):
        raise FileNotFoundError(path)

    tree = ET.parse(path)
    root = tree.getroot()
    log = MyEventLog()

    for log_attr_key, log_attr_value in root.attrib.items():
        log.attributes[log_attr_key] = log_attr_value

    traces: list[ET.Element] = []
    for child in root:
        tag_name = get_tag_wo_xmlns(child.tag)
        if tag_name == const_tag_trace:
            traces.append(child)
            continue

        if tag_name == const_tag_extension:
            ext_name = child.attrib[const_ext_name]
            ext_prefix = child.attrib[const_ext_prefix]
            ext_uri = child.attrib[const_ext_uri]
            log.extensions[ext_name] = {const_ext_prefix: ext_prefix, const_ext_uri: ext_uri}
            continue

        if tag_name == const_tag_classifier:
            classifier_name = child.attrib[const_classifier_name]
            classifier_keys = child.attrib[const_classifier_keys]
            log.classifiers[classifier_name] = classifier_keys.split(' ')
            continue

        if _is_value_tag(child):
            log.properties[child.attrib[const_attr_key]] = _parse_value_tag(child)
            continue

        if tag_name == const_tag_global:
            scope = child.attrib['scope']
            if scope not in log.global_values:
                log.global_values[scope] = {}

            for global_child in child:
                attrs = global_child.attrib
                log.global_values[scope][attrs[const_attr_key]] = attrs[const_attr_value]

            continue

    for trace in traces:
        events = []
        properties = {}
        if const_tag_trace in log.global_values:
            for key, value in log.global_values[const_tag_trace].items():
                properties[key] = value

        for child in trace:
            tag_name = get_tag_wo_xmlns(child.tag)
            if _is_value_tag(child):
                properties[child.attrib[const_attr_key]] = _parse_value_tag(child)
                continue

            if tag_name == const_tag_event:
                new_event = MyEvent()
                if const_tag_event in log.global_values:
                    for key, value in log.global_values[const_tag_event].items():
                        new_event[key] = value

                for event_child in child:
                    if _is_value_tag(event_child):
                        new_event[event_child.attrib[const_attr_key]] = _parse_value_tag(event_child)

                events.append(new_event)
                continue

        log.append(MyTrace(events, properties))

    return log


def get_tag_wo_xmlns(tag: str):
    return tag[tag.index('}')+1:] if '}' in tag else tag


def _parse_datetime(value: str):
    return parser.parse(value)


parse_functions = {
    const_tag_string: str,
    const_tag_date: _parse_datetime,
    const_tag_int: int,
    const_tag_float: float,
    const_tag_bool: bool
}


def _is_value_tag(element: ET.Element):
    return get_tag_wo_xmlns(element.tag) in parse_functions


def _parse_value_tag(element: ET.Element) -> Union[str, datetime, int, float, bool]:
    tag_name = get_tag_wo_xmlns(element.tag)
    return parse_functions[tag_name](element.attrib[const_attr_value])


types_to_names = {
    str: const_tag_string,
    datetime: const_tag_date,
    int: const_tag_int,
    float: const_tag_float,
    bool: const_tag_bool
}


def _create_value_tag(name: str, value: Union[str, datetime, int, float, bool]) -> ET.Element:
    element = ET.Element(types_to_names[type(value)])
    element.attrib[const_attr_key] = name
    element.attrib[const_attr_value] = str(value)
    return element


def parse_log_from_strings(strings: list[str], separator: str = default_separator) -> MyEventLog:
    log = MyEventLog()
    for string in strings:
        log.append(parse_trace_from_string(string, separator=separator))

    return log


def parse_log_from_string(string: str, separator: str = default_separator) -> MyEventLog:
    log = MyEventLog()
    log.append(parse_trace_from_string(string, separator=separator))
    return log


def parse_trace_from_string(string: str, separator: str = default_separator):
    trace = MyTrace()
    for char in string.split(separator):
        event = MyEvent({concept_name: char})
        trace.append(event)

    return trace


def serialize_log(log: MyEventLog,
                  traces_separator: str = default_trace_separator,
                  events_separator: str = default_events_separator) -> str:
    result = ''
    for trace_index, trace in enumerate(log):
        for event_index, event in enumerate(trace):
            if event_index > 0:
                result += events_separator

            result += event[concept_name]

        if trace_index < len(log) - 1:
            result += traces_separator

    return result


def serialize_logs(logs: list[MyEventLog],
                   logs_separator: str = default_logs_separator,
                   traces_separator: str = default_trace_separator,
                   events_separator: str = default_events_separator):
    result = ''
    for index, log in enumerate(logs):
        if index > 0:
            result += logs_separator

        result += serialize_log(log, traces_separator=traces_separator, events_separator=events_separator)

    return result


def serialize_log_to_xes(log: MyEventLog) -> str:
    log_element = ET.Element(const_tag_log)

    for key, value in log.attributes.items():
        log_element.attrib[key] = value

    for ext_name, ext_content in log.extensions.items():
        ext_element = ET.Element(const_tag_extension)
        ext_element.attrib[const_ext_name] = ext_name
        for key, value in ext_content.items():
            ext_element.attrib[key] = value

        log_element.append(ext_element)

    for classifier_name, classifier_content in log.classifiers.items():
        classifier_element = ET.Element(const_tag_classifier)
        classifier_element.attrib[const_classifier_name] = classifier_name
        classifier_element.attrib[const_classifier_keys] = ' '.join(classifier_content)
        log_element.append(classifier_element)

    for key, value in log.properties.items():
        property_element = ET.Element(const_tag_string)
        property_element.attrib[key] = value
        log_element.append(property_element)

    for scope_name, scope_default_values in log.global_values.items():
        global_scope_element = ET.Element(const_tag_global)
        global_scope_element.attrib[const_scope] = scope_name
        for name, value in scope_default_values.items():
            global_scope_element.append(_create_value_tag(name, value))

        log_element.append(global_scope_element)

    for trace in log:
        trace_element = ET.Element(const_tag_trace)
        for key, value in trace.attributes.items():
            if _check_if_default_value(log, const_tag_trace, key, value):
                continue

            if not _should_serialize_payload_key_value(key, value):
                continue

            trace_element.append(_create_value_tag(key, value))

        for event in trace:
            event_element = ET.Element(const_tag_event)
            for key, value in event.payload.items():
                if _check_if_default_value(log, const_tag_event, key, value):
                    continue

                if not _should_serialize_payload_key_value(key, value):
                    continue

                event_element.append(_create_value_tag(key, value))

            trace_element.append(event_element)

        log_element.append(trace_element)

    return ET.tostring(log_element, encoding="unicode")


def _should_serialize_payload_key_value(key: str, value: Any):
    return type(value) in types_to_names


def _check_if_default_value(log: MyEventLog,
                            entity_name: str,
                            key: str,
                            value: Union[str, int, float, datetime, bool]) -> bool:
    defaults = log.global_values
    if entity_name not in defaults:
        return False

    if key not in defaults[entity_name]:
        return False

    return defaults[entity_name][key] == value


def save_event_log_to_xes(log: MyEventLog, save_path: str):
    log_xes_string = serialize_log_to_xes(log)
    os.makedirs(os.path.dirname(save_path), exist_ok=True)
    with open(save_path, 'w') as fout:
        fout.write(log_xes_string)


def serialize_log_to_xes_beautiful(log: MyEventLog) -> str:
    return serialize_log_to_xes(log).replace('>', '>\n')
