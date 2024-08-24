use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::io;
use std::string::FromUtf8Error;

use quick_xml::events::{BytesEnd, BytesStart};
use quick_xml::Writer;

pub enum XmlWriteError {
    FromUt8Error(FromUtf8Error),
    IOError(io::Error),
    WriterError(quick_xml::Error),
}

impl Display for XmlWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromUt8Error(err) => Display::fmt(&err, f),
            Self::IOError(err) => Display::fmt(&err, f),
            Self::WriterError(err) => Display::fmt(&err, f),
        }
    }
}

impl Debug for XmlWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromUt8Error(arg0) => f.debug_tuple("FromUt8Error").field(arg0).finish(),
            Self::IOError(arg0) => f.debug_tuple("IOError").field(arg0).finish(),
            Self::WriterError(arg0) => f.debug_tuple("WriterError").field(arg0).finish(),
        }
    }
}

impl Error for XmlWriteError {}

pub fn write_empty<T>(writer: &mut Writer<T>, tag_name: &str, attrs: &Vec<(&str, &str)>) -> Result<(), XmlWriteError>
where
    T: std::io::Write,
{
    let mut empty_tag = BytesStart::new(tag_name);
    for (name, value) in attrs {
        empty_tag.push_attribute((*name, *value));
    }

    let empty = quick_xml::events::Event::Empty(empty_tag);

    match writer.write_event(empty) {
        Ok(_) => Ok(()),
        Err(error) => Err(XmlWriteError::WriterError(error)),
    }
}

pub struct StartEndElementCookie<'a, T>
where
    T: io::Write,
{
    tag_name: &'a str,
    writer: &'a RefCell<Writer<T>>,
}

impl<'a, T> Drop for StartEndElementCookie<'a, T>
where
    T: io::Write,
{
    fn drop(&mut self) {
        let end = quick_xml::events::Event::End(BytesEnd::new(self.tag_name));
        assert!(self.writer.borrow_mut().write_event(end).is_ok());
    }
}

impl<'a, T> StartEndElementCookie<'a, T>
where
    T: io::Write,
{
    pub fn new(writer: &'a RefCell<Writer<T>>, tag_name: &'a str) -> Result<StartEndElementCookie<'a, T>, XmlWriteError> {
        let start = quick_xml::events::Event::Start(BytesStart::new(tag_name));

        match writer.borrow_mut().write_event(start) {
            Err(error) => Err(XmlWriteError::WriterError(error)),
            Ok(_) => Ok(StartEndElementCookie { tag_name, writer }),
        }
    }

    pub fn new_with_attrs(
        writer: &'a RefCell<Writer<T>>,
        tag_name: &'a str,
        attrs: &Vec<(&str, &str)>,
    ) -> Result<StartEndElementCookie<'a, T>, XmlWriteError> {
        let mut start_tag = BytesStart::new(tag_name);
        for (name, value) in attrs {
            start_tag.push_attribute((*name, *value));
        }

        let start_event = quick_xml::events::Event::Start(start_tag);
        match writer.borrow_mut().write_event(start_event) {
            Err(error) => Err(XmlWriteError::WriterError(error)),
            Ok(_) => Ok(StartEndElementCookie { tag_name, writer }),
        }
    }
}
