use std::fmt::Debug;

use nom::error::{self, Error};

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError<I> {
	pub input: I,
	pub kind: ErrorKind,
}

impl<I: Clone + Debug> From<Error<I>> for ParseError<I> {
	fn from(value: Error<I>) -> Self {
		dbg!(value.code);

		let kind = match value.code {
			error::ErrorKind::Eof => ErrorKind::Eof,
			_ => ErrorKind::NotEnoughData,
		};

		Self { input: value.input, kind }
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
	Eof,
	NotEnoughData,
}
