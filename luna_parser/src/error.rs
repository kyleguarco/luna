use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ParseError<I> {
	pub input: I,
	pub kind: ErrorKind,
}

impl<I: Clone + Debug> From<nom::error::Error<I>> for ParseError<I> {
	fn from(value: nom::error::Error<I>) -> Self {
		dbg!(value.code);
		Self {
			input: value.input,
			kind: ErrorKind::NotEnoughData,
		}
	}
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
	NotEnoughData,
}
