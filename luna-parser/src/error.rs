use std::fmt::Debug;

use nom::error;

#[derive(Clone, Debug, PartialEq)]
pub struct Error<I> {
	pub input: I,
	pub kind: ErrorKind,
}

impl<I> error::ParseError<I> for Error<I> {
	fn from_error_kind(input: I, kind: error::ErrorKind) -> Self {
		let kind = match kind {
			error::ErrorKind::Eof => ErrorKind::Eof,
			_ => ErrorKind::NotEnoughData,
		};

		Self { input, kind }
	}

	fn append(input: I, kind: error::ErrorKind, _other: Self) -> Self {
		// TODO!: more errors?
		Self::from_error_kind(input, kind)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
	Eof,
	NotEnoughData,
}
