//! # Input Stream Interface
//! Since `luna` is written in lower-level rust, the programmer may be writing
//! for a system that doesn't have a standardized way of reading input. This
//! module provides a standard interface for input into the LVM.

use core::ops::Deref;

use alloc::vec::Vec;

pub type ZioResult = Result<usize, ()>;

pub enum SeekFrom {
    Start(usize),
    Current(isize),
}

pub trait Read {
    /// Reads from an internal stream and copies some bytes into the provided buffer.
    ///
    /// # Errors
    /// This function returns an error if `stream` is larger than the remaining
    /// amount of bytes in the internal stream.
    fn read(&self, stream: &mut [u8]) -> ZioResult;
}

pub trait Write<T: ExactSizeIterator + Deref<Target = [u8]>> {
    fn write(&mut self, stream: T) -> ZioResult;
}

pub trait Seek {
    fn seek(&mut self, offset: SeekFrom) -> ZioResult;
}

#[derive(Default)]
pub struct Zio {
    stream: Vec<u8>,
    pos: usize,
}

impl Zio {
    pub fn new<T: Into<Vec<u8>>>(stream: T) -> Self {
        Zio {
            stream: stream.into(),
            pos: 0,
        }
    }

    pub fn stream(&self) -> &[u8] {
        &self.stream
    }

    pub fn stream_mut(&mut self) -> &mut Vec<u8> {
        &mut self.stream
    }
}

impl Read for Zio {
    /// Reads from [Zio]'s internal stream and copies it into `stream`.
    fn read(&self, stream: &mut [u8]) -> ZioResult {
        let toread = self.pos + stream.len();

        if toread > self.stream.len() {
            return Err(());
        }

        stream.clone_from_slice(&self.stream[self.pos..toread]);
        Ok(toread)
    }
}

impl<T: ExactSizeIterator + Deref<Target = [u8]>> Write<T> for Zio {
    fn write(&mut self, stream: T) -> ZioResult {
        self.stream.extend_from_slice(&stream);
        Ok(stream.len())
    }
}

impl Seek for Zio {
    fn seek(&mut self, offset: SeekFrom) -> ZioResult {
        match offset {
            SeekFrom::Start(offset) => {
                self.pos = offset;
                Ok(offset)
            }
            SeekFrom::Current(offset) => {
                let isneg = offset.is_negative();
                let offset = offset as usize;

                match isneg {
                    true => self.pos.checked_add(offset).ok_or(()),
                    false => self.pos.checked_sub(offset).ok_or(()),
                }
            }
        }
    }
}
