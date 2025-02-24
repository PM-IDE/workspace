use std::io::{BufRead, Cursor, Read};
use std::{cell::RefCell, collections::HashMap, fs::File, io::BufReader, rc::Rc};

use quick_xml::{events::BytesStart, Reader};

use crate::event_log::xes::constants::*;
use crate::event_log::{
  core::event::event::EventPayloadValue,
  xes::{
    constants::{CLASSIFIER_TAG_NAME, EXTENSION_TAG_NAME},
    shared::{XesClassifier, XesEventLogExtension, XesGlobal, XesProperty},
    xes_event_log::XesEventLogImpl,
  },
};

use super::{utils, xes_log_trace_reader::TraceXesEventLogIterator};

pub enum XmlReader<'a> {
  FileReader(BufReader<File>),
  MemoryReader(BufReader<Cursor<&'a [u8]>>),
}

impl<'a> Read for XmlReader<'a> {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    match self {
      XmlReader::FileReader(reader) => reader.read(buf),
      XmlReader::MemoryReader(reader) => reader.read(buf),
    }
  }
}

impl<'a> BufRead for XmlReader<'a> {
  fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
    match self {
      XmlReader::FileReader(reader) => reader.fill_buf(),
      XmlReader::MemoryReader(reader) => reader.fill_buf(),
    }
  }

  fn consume(&mut self, amt: usize) {
    match self {
      XmlReader::FileReader(reader) => reader.consume(amt),
      XmlReader::MemoryReader(reader) => reader.consume(amt),
    }
  }
}

pub struct FromFileXesEventLogReader<'a> {
  storage: Rc<RefCell<Vec<u8>>>,
  reader: Rc<RefCell<Reader<XmlReader<'a>>>>,
  seen_globals: Rc<RefCell<HashMap<String, HashMap<String, EventPayloadValue>>>>,
}

pub enum XesEventLogItem<'a> {
  Trace(TraceXesEventLogIterator<'a>),
  Global(XesGlobal),
  Extension(XesEventLogExtension),
  Classifier(XesClassifier),
  Property(XesProperty),
}

pub fn read_event_log_from_bytes(bytes: &[u8]) -> Option<XesEventLogImpl> {
  let mut reader = FromFileXesEventLogReader::new_from_bytes(bytes)?;
  XesEventLogImpl::new(&mut reader)
}

pub fn read_event_log(file_path: &str) -> Option<XesEventLogImpl> {
  let mut reader = FromFileXesEventLogReader::new(file_path)?;
  XesEventLogImpl::new(&mut reader)
}

impl<'a> Iterator for FromFileXesEventLogReader<'a> {
  type Item = XesEventLogItem<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut storage = self.storage.borrow_mut();
    let mut reader = self.reader.borrow_mut();

    loop {
      match reader.read_event_into(&mut storage) {
        Ok(quick_xml::events::Event::Start(tag)) => match tag.name().as_ref() {
          TRACE_TAG_NAME => {
            let copy_reader = Rc::clone(&self.reader);
            let copy_globals = Rc::clone(&self.seen_globals);
            let iterator = TraceXesEventLogIterator::new(copy_reader, copy_globals);
            return Some(XesEventLogItem::Trace(iterator));
          }
          GLOBAL_TAG_NAME => match Self::try_read_scope_name(&tag) {
            Some(scope_name) => match Self::try_read_global(&mut reader, &mut storage) {
              Some(default_values) => {
                let mut globals = self.seen_globals.borrow_mut();
                if globals.contains_key(&scope_name) {
                  continue;
                }

                globals.insert(scope_name.to_owned(), default_values.to_owned());
                let global = XesGlobal {
                  scope: scope_name,
                  default_values,
                };
                return Some(XesEventLogItem::Global(global));
              }
              None => continue,
            },
            None => continue,
          },
          EXTENSION_TAG_NAME | CLASSIFIER_TAG_NAME => match Self::try_read_tag(&tag) {
            Some(item) => return Some(item),
            None => continue,
          },
          _ => match Self::try_read_property(&tag) {
            Some(item) => return Some(item),
            None => continue,
          },
        },
        Ok(quick_xml::events::Event::Empty(tag)) => match Self::try_read_tag(&tag) {
          Some(item) => return Some(item),
          None => continue,
        },
        Ok(quick_xml::events::Event::Eof) => return None,
        Err(_) => return None,
        _ => continue,
      }
    }
  }
}

impl<'a> FromFileXesEventLogReader<'a> {
  pub fn new_from_bytes(bytes: &[u8]) -> Option<FromFileXesEventLogReader> {
    let reader = XmlReader::MemoryReader(BufReader::new(Cursor::new(bytes)));
    Some(Self::create_quickxml_reader(reader))
  }

  fn create_quickxml_reader(reader: XmlReader) -> FromFileXesEventLogReader {
    FromFileXesEventLogReader {
      reader: Rc::new(RefCell::new(Reader::from_reader(reader))),
      storage: Rc::new(RefCell::new(Vec::new())),
      seen_globals: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  pub fn new(file_path: &str) -> Option<FromFileXesEventLogReader> {
    let file = match File::open(file_path) {
      Ok(file) => file,
      Err(_) => return None,
    };

    Some(Self::create_quickxml_reader(XmlReader::FileReader(BufReader::new(file))))
  }

  fn try_read_scope_name(tag: &BytesStart) -> Option<String> {
    let mut scope_name: Option<String> = None;

    for attr in tag.attributes() {
      match attr {
        Ok(real_attr) => match real_attr.key.0 {
          SCOPE_ATTR_NAME => {
            if !utils::read_attr_value(&real_attr, &mut scope_name) {
              continue;
            }
          }
          _ => continue,
        },
        Err(_) => continue,
      }
    }

    scope_name
  }

  fn try_read_tag(tag: &BytesStart) -> Option<XesEventLogItem<'a>> {
    let result = match tag.name().as_ref() {
      EXTENSION_TAG_NAME => match Self::try_read_extension(&tag) {
        Some(extension) => Some(XesEventLogItem::Extension(extension)),
        None => None,
      },
      CLASSIFIER_TAG_NAME => match Self::try_read_classifier(&tag) {
        Some(classifier) => Some(XesEventLogItem::Classifier(classifier)),
        None => None,
      },
      _ => None,
    };

    if result.is_some() {
      return result;
    }

    Self::try_read_property(tag)
  }

  fn try_read_property(tag: &BytesStart) -> Option<XesEventLogItem<'a>> {
    match utils::read_payload_like_tag(tag) {
      Some(descriptor) => {
        let payload_type = descriptor.payload_type.as_str().as_bytes();
        let key = descriptor.key;
        let value = descriptor.value.as_str();

        match utils::extract_payload_value(payload_type, key.as_str(), value) {
          Some(value) => Some(XesEventLogItem::Property(XesProperty { name: key, value })),
          None => None,
        }
      }
      None => None,
    }
  }

  fn try_read_global(reader: &mut Reader<XmlReader>, storage: &mut Vec<u8>) -> Option<HashMap<String, EventPayloadValue>> {
    let mut map: Option<HashMap<String, EventPayloadValue>> = None;

    loop {
      match reader.read_event_into(storage) {
        Err(_) => return None,
        Ok(quick_xml::events::Event::Empty(tag)) => {
          if let Some(descriptor) = utils::read_payload_like_tag(&tag) {
            if let None = map {
              map = Some(HashMap::new())
            }

            let payload_type = descriptor.payload_type.as_str().as_bytes();
            let value = utils::extract_payload_value(payload_type, descriptor.key.as_str(), &descriptor.value);
            if let Some(payload_value) = value {
              map.as_mut().unwrap().insert(descriptor.key, payload_value);
            }
          }
        }
        Ok(quick_xml::events::Event::End(tag)) => match tag.name().0 {
          GLOBAL_TAG_NAME => break,
          _ => continue,
        },
        _ => continue,
      }
    }

    map
  }

  fn try_read_classifier(tag: &BytesStart) -> Option<XesClassifier> {
    let mut name: Option<String> = None;
    let mut keys: Option<Vec<String>> = None;

    for attr in tag.attributes() {
      match attr {
        Ok(real_attr) => match real_attr.key.0 {
          NAME_ATTR_NAME => {
            if !utils::read_attr_value(&real_attr, &mut name) {
              return None;
            }
          }
          KEYS_ATTR_NAME => match String::from_utf8(real_attr.value.into_owned()) {
            Ok(keys_string) => keys = Some(keys_string.split(" ").map(|s| s.to_owned()).collect()),
            Err(_) => return None,
          },
          _ => continue,
        },
        Err(_) => continue,
      }
    }

    if name.is_none() || keys.is_none() {
      return None;
    }

    Some(XesClassifier {
      name: name.unwrap(),
      keys: keys.unwrap(),
    })
  }

  fn try_read_extension(tag: &BytesStart) -> Option<XesEventLogExtension> {
    let mut name: Option<String> = None;
    let mut prefix: Option<String> = None;
    let mut uri: Option<String> = None;

    for attr in tag.attributes() {
      match attr {
        Ok(real_attr) => match real_attr.key.0 {
          PREFIX_ATTR_NAME => {
            if !utils::read_attr_value(&real_attr, &mut prefix) {
              return None;
            }
          }
          NAME_ATTR_NAME => {
            if !utils::read_attr_value(&real_attr, &mut name) {
              return None;
            }
          }
          URI_ATTR_NAME => {
            if !utils::read_attr_value(&real_attr, &mut uri) {
              return None;
            }
          }
          _ => continue,
        },
        Err(_) => return None,
      }
    }

    if !name.is_some() || !prefix.is_some() || !uri.is_some() {
      return None;
    }

    Some(XesEventLogExtension {
      name: name.unwrap(),
      prefix: prefix.unwrap(),
      uri: uri.unwrap(),
    })
  }
}
