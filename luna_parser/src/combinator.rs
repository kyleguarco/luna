use nom::{
	character::streaming::{multispace0, multispace1},
	error::ParseError,
	sequence::delimited,
	AsChar, IResult, InputTakeAtPosition,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// This combinator was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#whitespace
pub(crate) fn ws0<F, I, O, E>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
	F: FnMut(I) -> IResult<I, O, E>,
	E: ParseError<I>,
	I: InputTakeAtPosition,
	<I as InputTakeAtPosition>::Item: Clone + AsChar,
{
	delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// This combinator requires the presence of at least one space around each side of `inner`.
///
/// This combinator was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#whitespace
pub(crate) fn ws1<F, I, O, E>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
	F: FnMut(I) -> IResult<I, O, E>,
	E: ParseError<I>,
	I: InputTakeAtPosition,
	<I as InputTakeAtPosition>::Item: Clone + AsChar,
{
	delimited(multispace1, inner, multispace1)
}
