//https://github.com/mathias234/binary_rw/blob/master/src/stream/file.rs
use std::fs::{File, Metadata, OpenOptions};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read, SeekFrom, Write};
use std::path::Path;

use super::core::{ReadStream, SeekStream, WriteStream};
use super::error::BinaryError;

/// Stream that wraps a file.
pub struct FileStream {
    file: File,
    metadata: std::result::Result<Metadata, std::io::Error>,
}

impl FileStream {
    /// Create a file stream.
    pub fn new(file: File) -> Self {
        Self {
            metadata: file.metadata(),
            file,
        }
    }

    /// Create a file stream in write-only mode.
    ///
    /// If the file exists it is truncated, if it does not
    /// exist it will be created.
    pub fn create<P: AsRef<Path>>(path: P) -> crate::binary_rw::core::Result<Self> {
        Ok(FileStream::new(File::create(path.as_ref())?))
    }

    /// Attempts to open a file stream in read-only mode.
    pub fn open<P: AsRef<Path>>(path: P) -> crate::binary_rw::core::Result<Self> {
        Ok(FileStream::new(File::open(path.as_ref())?))
    }

    /// Attempts to open a file stream with read and write modes enabled.
    pub fn write<P: AsRef<Path>>(path: P) -> crate::binary_rw::core::Result<Self> {
        Ok(FileStream::new(OpenOptions::new().read(true).write(true).open(path)?))
    }

    /// Attempts to get the metadata for file
    pub fn metadata(&self) -> std::io::Result<&Metadata> {
        match self.metadata.as_ref() {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(e.kind(), "Unable to get metadata")),
        }
    }
}

impl SeekStream for FileStream {
    fn seek(&mut self, to: usize) -> crate::binary_rw::core::Result<usize> {
        Ok(self.file.seek(SeekFrom::Start(to as u64))? as usize)
    }

    fn tell(&mut self) -> crate::binary_rw::core::Result<usize> {
        Ok(self.file.seek(SeekFrom::Current(0))? as usize)
    }

    fn len(&self) -> crate::binary_rw::core::Result<usize> {
        Ok(self.metadata()?.len().try_into()?)
    }
}

impl Read for FileStream {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.tell().unwrap() + buffer.len() > self.metadata()?.len() as usize {
            return Err(Error::new(ErrorKind::UnexpectedEof, BinaryError::ReadPastEof));
        }
        Ok(self.file.read(buffer)?)
    }
}

impl Write for FileStream {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.file.write(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl ReadStream for FileStream {}
impl WriteStream for FileStream {}
