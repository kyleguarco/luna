use nom::{
	bytes::streaming::tag, character::streaming::multispace0, error::ParseError,
	sequence::delimited, AsChar, Compare, IResult, InputLength, InputTake, InputTakeAtPosition,
};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// This function was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#whitespace
pub(crate) fn whitespace<F, I, O, E>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
	F: FnMut(I) -> IResult<I, O, E>,
	E: ParseError<I>,
	I: InputTakeAtPosition,
	<I as InputTakeAtPosition>::Item: Clone + AsChar,
{
	delimited(multispace0, inner, multispace0)
}

/// Similar to the `tag` combinator, except it ignores the whitespace around the input.
/// None of the terminals in lua are restricted by whitespace, so all of it can be ignored.
pub(crate) fn whitetag<T, I, E>(inner: T) -> impl FnMut(I) -> IResult<I, I, E>
where
	T: InputLength + Clone,
	E: ParseError<I>,
	I: InputTakeAtPosition + InputLength + InputTake + Compare<T>,
	<I as InputTakeAtPosition>::Item: Clone + AsChar,
{
	whitespace(tag(inner))
}
