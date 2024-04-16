use std::io::{Read, Write};

use crate::binary_rw::{
    core::{ReadStream, SeekStream, WriteStream},
    file_stream::FileStream,
};

pub struct BufferedReadFileStream {
    stream: FileStream,
    buffer: Vec<u8>,
    occupied_size: usize,
    next_buffer_index: usize,
    file_length_bytes: usize,
    total_read_bytes: usize,
}

impl BufferedReadFileStream {
    pub fn new(stream: FileStream, buffer_size: usize) -> Self {
        let length = stream.len().ok().unwrap();
        Self {
            stream,
            buffer: vec![0; buffer_size],
            occupied_size: 0,
            next_buffer_index: 0,
            file_length_bytes: length,
            total_read_bytes: 0,
        }
    }

    pub fn total_read_bytes(&self) -> usize {
        self.total_read_bytes
    }
}

impl ReadStream for BufferedReadFileStream {}
impl Read for BufferedReadFileStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut out_buff_index = 0;

        loop {
            if out_buff_index >= buf.len() {
                break;
            }

            let to_read = buf.len() - out_buff_index;
            if self.next_buffer_index + to_read <= self.occupied_size {
                for i in 0..to_read {
                    buf[out_buff_index] = self.buffer[self.next_buffer_index + i];
                    out_buff_index += 1;
                }

                self.next_buffer_index += to_read;
                break;
            } else {
                let read_bytes = self.occupied_size - self.next_buffer_index;
                for i in 0..read_bytes {
                    buf[out_buff_index] = self.buffer[self.next_buffer_index + i];
                    out_buff_index += 1;
                }

                let current_pos = self.stream.tell().ok().unwrap();
                let remained_bytes_in_file = self.file_length_bytes - current_pos;
                self.next_buffer_index = 0;

                if remained_bytes_in_file < self.buffer.len() {
                    self.occupied_size = remained_bytes_in_file;
                    self.stream
                        .read_exact(&mut self.buffer[0..remained_bytes_in_file])?;
                } else {
                    self.occupied_size = self.buffer.len();
                    self.stream.read_exact(&mut self.buffer)?;
                }
            }
        }

        self.total_read_bytes += buf.len();
        Ok(buf.len())
    }
}

impl SeekStream for BufferedReadFileStream {
    fn seek(&mut self, to: usize) -> crate::binary_rw::core::Result<usize> {
        self.next_buffer_index = 0;
        self.occupied_size = 0;

        self.stream.seek(to)
    }

    fn tell(&mut self) -> crate::binary_rw::core::Result<usize> {
        //reduce the number of sys calls
        Ok(self.total_read_bytes)
    }

    fn len(&self) -> crate::binary_rw::core::Result<usize> {
        self.stream.len()
    }
}

pub struct BufferedWriteFileStream {
    stream: FileStream,
    buffer: Vec<u8>,
    written_bytes_count: usize,
}

impl BufferedWriteFileStream {
    pub fn new(stream: FileStream, buffer_size: usize) -> Self {
        Self {
            stream,
            buffer: vec![0; buffer_size],
            written_bytes_count: 0,
        }
    }
}

impl WriteStream for BufferedWriteFileStream {}

impl SeekStream for BufferedWriteFileStream {
    fn seek(&mut self, to: usize) -> crate::binary_rw::core::Result<usize> {
        self.flush()?;
        self.stream.seek(to)
    }

    fn tell(&mut self) -> crate::binary_rw::core::Result<usize> {
        self.stream.tell()
    }

    fn len(&self) -> crate::binary_rw::core::Result<usize> {
        self.stream.len()
    }
}

impl Write for BufferedWriteFileStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() > self.buffer.len() {
            self.flush()?;
            self.stream.write(buf)?;
        } else {
            let remained_space = self.buffer.len() - self.written_bytes_count;
            if buf.len() > remained_space {
                self.flush()?;
                self.written_bytes_count = buf.len();
                for i in 0..buf.len() {
                    self.buffer[i] = buf[i];
                }
            } else {
                for i in 0..buf.len() {
                    self.buffer[self.written_bytes_count + i] = buf[i];
                }

                self.written_bytes_count += buf.len();
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.written_bytes_count != 0 {
            self.written_bytes_count = 0;
            self.stream
                .write(&self.buffer[0..self.written_bytes_count])?;
        }

        self.stream.flush()
    }
}
