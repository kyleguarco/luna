use nom::{
	character::complete::multispace0, error::ParseError, sequence::delimited, AsChar, IResult,
	InputTakeAtPosition,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// This function was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#whitespace
pub(crate) fn whitespace<'a, F, Input, Output, Error>(
	inner: F,
) -> impl FnMut(Input) -> IResult<Input, Output, Error>
where
	F: FnMut(Input) -> IResult<Input, Output, Error> + 'a,
	Error: ParseError<Input>,
	Input: InputTakeAtPosition + 'a,
	<Input as InputTakeAtPosition>::Item: Clone + AsChar,
{
	delimited(multispace0, inner, multispace0)
}
