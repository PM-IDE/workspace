use std::io::{Cursor, Error, ErrorKind, Read, Seek, SeekFrom};
use crate::binary_rw::core::{ReadStream, SeekStream};
use crate::binary_rw::error::BinaryError;

pub struct CursorStream<'a> {
    cursor: Cursor<&'a [u8]>
}

impl<'a> CursorStream<'a> {
    pub fn new(cursor: Cursor<&'a [u8]>) -> Self {
        Self {
            cursor
        }
    }
}

impl<'a> Read for CursorStream<'a> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buffer)
    }
}

impl<'a> SeekStream for CursorStream<'a> {
    fn seek(&mut self, to: usize) -> crate::binary_rw::core::Result<usize> {
        match self.cursor.seek(SeekFrom::Start(to as u64)) {
            Ok(result) => Ok(result as usize),
            Err(_err) => Err(BinaryError::ReadPastEof)
        }
    }

    fn tell(&mut self) -> crate::binary_rw::core::Result<usize> {
        Ok(self.cursor.position() as usize)
    }

    fn len(&self) -> crate::binary_rw::core::Result<usize> {
        Ok(self.cursor.get_ref().len())
    }
}

impl<'a> ReadStream for CursorStream<'a> {
}