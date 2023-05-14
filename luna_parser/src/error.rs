use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ParseError<I> {
	input: I,
	kind: ErrorKind,
}

impl<I: Clone + Debug> From<nom::error::Error<I>> for ParseError<I> {
	fn from(value: nom::error::Error<I>) -> Self {
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
