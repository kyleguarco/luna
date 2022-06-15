//! # Input Stream Interface
//! Since `luna` is written in lower-level rust, the programmer may be writing
//! for a system that doesn't have a standardized way of reading input. This
//! module provides a standard interface for input into the LVM.

use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;

pub type ZioResult = Result<usize, ()>;

pub enum SeekFrom {
    Start(usize),
    Current(isize),
}

pub trait Read {
    fn read<'a, T: ExactSizeIterator + AsRef<[u8]>>(&self, stream: &'a mut T) -> ZioResult;
}

pub trait Write {
    fn write<T: AsRef<[u8]>>(&mut self, stream: T) -> ZioResult;
}

pub trait Seek {
    fn seek(&mut self, offset: SeekFrom) -> ZioResult;
}

#[derive(Default)]
pub struct Zio {
    stream: Vec<u8>,
    pos: usize,
}

impl Read for Zio {
    fn read<'a, T: ExactSizeIterator + AsRef<[u8]>>(&self, stream: &'a mut T) -> ZioResult {
		//*stream = self.stream[self.pos..stream.len()];

		Ok(0)
    }
}
