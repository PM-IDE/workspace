use std::rc::Rc;
use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::event_log::{core::event::event::EventPayloadValue, xes::constants::*};

use crate::event_log::core::event::lifecycle::xes_lifecycle::Lifecycle;
use quick_xml::{
    escape::unescape,
    events::{attributes::Attribute, BytesStart},
};

pub struct KeyValuePair<TKey, TValue> {
    pub key: Option<TKey>,
    pub value: Option<TValue>,
}

pub struct PayloadTagDescriptor {
    pub payload_type: String,
    pub key: String,
    pub value: String,
}

pub fn read_payload_like_tag(tag: &BytesStart) -> Option<PayloadTagDescriptor> {
    let kv = extract_key_value(&tag);
    if !kv.value.is_some() || !kv.key.is_some() {
        return None;
    }

    let key = unescape(kv.key.as_ref().unwrap()).ok().unwrap().to_string();
    let value = unescape(kv.value.as_ref().unwrap()).ok().unwrap().to_string();

    let payload_type = match String::from_utf8(tag.name().0.to_vec()) {
        Ok(string) => string,
        Err(_) => return None,
    };

    let descriptor = PayloadTagDescriptor { payload_type, key, value };

    Some(descriptor)
}

pub fn extract_key_value(start: &BytesStart) -> KeyValuePair<String, String> {
    let mut key: Option<String> = None;
    let mut value: Option<String> = None;

    for attr in start.attributes() {
        match attr {
            Err(_) => continue,
            Ok(real_attr) => match real_attr.key.0 {
                KEY_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                    Err(_) => continue,
                    Ok(string) => key = Some(string),
                },
                VALUE_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                    Err(_) => continue,
                    Ok(string) => value = Some(string),
                },
                _ => continue,
            },
        }
    }

    return KeyValuePair { key, value };
}

#[inline]
pub fn read_attr_value(real_attr: &Attribute, var: &mut Option<String>) -> bool {
    match String::from_utf8(real_attr.value.as_ref().to_vec()) {
        Ok(string) => {
            *var = Some(string);
            true
        }
        Err(_) => false,
    }
}

pub fn extract_payload_value(name: &[u8], key: &str, value: &str) -> Option<EventPayloadValue> {
    if key == LIFECYCLE_TRANSITION_STR && name == STRING_TAG_NAME {
        return Some(EventPayloadValue::Lifecycle(Lifecycle::from_str(value).ok().unwrap()));
    }

    match name {
        DATE_TAG_NAME => match DateTime::parse_from_rfc3339(value) {
            Err(_) => None,
            Ok(date) => Some(EventPayloadValue::Date(date.with_timezone(&Utc))),
        },
        INT_TAG_NAME => match value.parse::<i64>() {
            Err(_) => None,
            Ok(int_value) => Some(EventPayloadValue::Int64(int_value)),
        },
        FLOAT_TAG_NAME => match value.parse::<f64>() {
            Err(_) => None,
            Ok(float_value) => Some(EventPayloadValue::Float64(float_value)),
        },
        STRING_TAG_NAME => Some(EventPayloadValue::String(Rc::new(Box::new(value.to_owned())))),
        BOOLEAN_TAG_NAME => match value.parse::<bool>() {
            Err(_) => None,
            Ok(bool_value) => Some(EventPayloadValue::Boolean(bool_value)),
        },
        ID_TAG_NAME => match uuid::Uuid::parse_str(value) {
            Ok(guid) => Some(EventPayloadValue::Guid(guid)),
            Err(_) => None,
        },
        _ => None,
    }
}
