use std::{fmt::Display, rc::Rc};

use crate::binary_rw::error::BinaryError;
use crate::models::domain::bxes_value::BxesValue;

#[derive(Debug)]
pub enum BxesWriteError {
    FailedToOpenFileForWriting(String),
    WriteError(BinaryError),
    FailedToGetWriterPosition(String),
    FailedToSeek(String),
    FailedToFindKeyValueIndex((Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)),
    FailedToFindValueIndex(Rc<Box<BxesValue>>),
    FailedToCreateTempFile,
    FailedToCreateArchive,
    LebWriteError(String),
    Default(String),
}

impl ToString for BxesWriteError {
    fn to_string(&self) -> String {
        match self {
            BxesWriteError::FailedToOpenFileForWriting(err) => err.to_owned(),
            BxesWriteError::WriteError(err) => err.to_string(),
            BxesWriteError::FailedToGetWriterPosition(err) => err.to_owned(),
            BxesWriteError::FailedToSeek(err) => err.to_owned(),
            BxesWriteError::FailedToFindKeyValueIndex(value) => format!("{:?}", value),
            BxesWriteError::FailedToFindValueIndex(value) => format!("{:?}", value),
            BxesWriteError::FailedToCreateTempFile => "FailedToCreateTempFile".to_string(),
            BxesWriteError::FailedToCreateArchive => "FailedToCreateArchive".to_string(),
            BxesWriteError::LebWriteError(err) => err.to_string(),
            BxesWriteError::Default(err) => err.to_owned()
        }
    }
}
