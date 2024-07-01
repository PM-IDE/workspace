use crate::binary_rw::core::SeekStream;
use std::io::{ErrorKind, Read, Seek, SeekFrom};

pub struct MemoryStream {
    index: usize,
    buffer: Vec<u8>,
}

impl MemoryStream {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { index: 0, buffer }
    }
}

impl Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.index >= self.buffer.len() {
            Ok(0)
        } else {
            let to_read = buf.len().min(self.buffer.len() - self.index);
            let mut current_index = self.index;
            for i in 0..to_read {
                buf[i] = self.buffer[current_index + i];
            }

            self.index += to_read;
            Ok(to_read)
        }
    }
}

impl Seek for MemoryStream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_index = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::End(offset) => self.buffer.len() as i64 - offset,
            SeekFrom::Current(offset) => self.index as i64 + offset,
        };

        if new_index >= 0 && new_index < self.buffer.len() as i64 {
            self.index = new_index as usize;
            Ok(self.index as u64)
        } else {
            Err(std::io::Error::new(ErrorKind::Other, "Bad Seek position"))
        }
    }
}
