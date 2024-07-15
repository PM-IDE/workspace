use std::{cell::RefCell, fs, io::Cursor};

use quick_xml::Writer;

use crate::event_log::{
    core::{
        event::event::{Event, EventPayloadValue},
        event_log::EventLog,
        trace::trace::Trace,
    },
    xes::{constants::*, xes_event_log::XesEventLogImpl},
};
use crate::utils::xml_utils::{write_empty, StartEndElementCookie, XmlWriteError};

pub fn write_xes_log_to_bytes(log: &XesEventLogImpl) -> Result<Vec<u8>, XmlWriteError> {
    match serialize_event_log(log) {
        Ok(content) => Ok(content.as_bytes().to_vec()),
        Err(error) => Err(error),
    }
}

pub fn write_xes_log(log: &XesEventLogImpl, save_path: &str) -> Result<(), XmlWriteError> {
    match serialize_event_log(log) {
        Ok(content) => match fs::write(save_path, content) {
            Ok(_) => Ok(()),
            Err(error) => Err(XmlWriteError::IOError(error)),
        },
        Err(error) => Err(error),
    }
}

pub fn serialize_event_log(log: &XesEventLogImpl) -> Result<String, XmlWriteError> {
    let writer = RefCell::new(Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2));

    {
        let _log_cookie = StartEndElementCookie::new(&writer, LOG_TAG_NAME_STR);

        for ext in log.extensions() {
            let attrs = vec![
                (NAME_ATTR_NAME_STR, ext.name.as_str()),
                (URI_ATTR_NAME_STR, ext.uri.as_str()),
                (PREFIX_ATTR_NAME_STR, ext.prefix.as_str()),
            ];

            write_empty(&mut writer.borrow_mut(), EXTENSION_TAG_NAME_STR, &attrs)?;
        }

        for classifier in log.classifiers() {
            let keys = classifier.keys.join(" ");
            let attrs = vec![(NAME_ATTR_NAME_STR, classifier.name.as_str()), (KEYS_ATTR_NAME_STR, keys.as_str())];

            write_empty(&mut writer.borrow_mut(), CLASSIFIER_TAG_NAME_STR, &attrs)?;
        }

        for (name, value) in log.ordered_properties() {
            write_payload_tag(&writer, name, value)?;
        }

        for (scope, defaults) in log.ordered_globals() {
            let mut attrs = vec![(SCOPE_ATTR_NAME_STR, scope.as_str())];

            let _global_cookie = StartEndElementCookie::new_with_attrs(&writer, GLOBAL_TAG_NAME_STR, &attrs);

            for (key, value) in defaults {
                attrs.clear();
                write_payload_tag(&writer, key, value)?;
            }
        }

        for trace in log.traces() {
            let trace = trace.borrow();
            let events = trace.events();
            if events.len() == 0 {
                continue;
            }

            let _trace_cookie = StartEndElementCookie::new(&writer, TRACE_TAG_NAME_STR);

            for event in events {
                let _event_cookie = StartEndElementCookie::new(&writer, EVENT_TAG_NAME_STR);
                let event = event.borrow();

                let attrs = vec![(KEY_ATTR_NAME_STR, CONCEPT_NAME_STR), (VALUE_ATTR_NAME_STR, event.name())];

                write_empty(&mut writer.borrow_mut(), STRING_TAG_NAME_STR, &attrs)?;

                let date_string = event.timestamp().to_rfc3339();
                let attrs = vec![(KEY_ATTR_NAME_STR, TIME_TIMESTAMP_STR), (VALUE_ATTR_NAME_STR, date_string.as_str())];

                write_empty(&mut writer.borrow_mut(), DATE_TAG_NAME_STR, &attrs)?;

                for (key, value) in event.ordered_payload() {
                    write_payload_tag(&writer, key, value)?;
                }
            }
        }
    }

    let content = writer.borrow().get_ref().get_ref().clone();
    match String::from_utf8(content) {
        Ok(string) => Ok(string),
        Err(error) => Err(XmlWriteError::FromUt8Error(error)),
    }
}

fn write_payload_tag(writer: &RefCell<Writer<Cursor<Vec<u8>>>>, key: &str, value: &EventPayloadValue) -> Result<(), XmlWriteError> {
    let tag_name = match value {
        EventPayloadValue::Null => return Ok(()),
        EventPayloadValue::Date(_) => DATE_TAG_NAME_STR,
        EventPayloadValue::String(_) => STRING_TAG_NAME_STR,
        EventPayloadValue::Boolean(_) => BOOLEAN_TAG_NAME_STR,
        EventPayloadValue::Int32(_) => INT_TAG_NAME_STR,
        EventPayloadValue::Int64(_) => INT_TAG_NAME_STR,
        EventPayloadValue::Float32(_) => FLOAT_TAG_NAME_STR,
        EventPayloadValue::Float64(_) => FLOAT_TAG_NAME_STR,
        EventPayloadValue::Uint32(_) => INT_TAG_NAME_STR,
        EventPayloadValue::Uint64(_) => INT_TAG_NAME_STR,
        EventPayloadValue::Guid(_) => ID_TAG_NAME_STR,
        EventPayloadValue::Timestamp(_) => DATE_TAG_NAME_STR,
        EventPayloadValue::Lifecycle(_) => STRING_TAG_NAME_STR,
        EventPayloadValue::Artifact(_) => todo!(),
        EventPayloadValue::Drivers(_) => todo!(),
        EventPayloadValue::SoftwareEvent(_) => todo!(),
    };

    let string_value = value.to_string();
    let attrs = vec![(KEY_ATTR_NAME_STR, key), (VALUE_ATTR_NAME_STR, string_value.as_str())];

    write_empty(&mut writer.borrow_mut(), tag_name, &attrs)
}
