use std::ops::{Range, RangeFrom};

use nom::{
	bytes::complete::tag,
	character::complete::{char as tchar, multispace0},
	multi::separated_list1,
	sequence::{delimited, separated_pair},
	AsChar, Compare, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Slice,
};

use crate::terminal::string::{EQUALS, LBRACE, LBRACKET, LPAREN, RBRACE, RBRACKET, RPAREN};

pub fn negate<I, F>(func: F) -> impl Fn(&I) -> bool
where
	I: ?Sized,
	F: Fn(&I) -> bool,
{
	move |input: &I| !func(input)
}

/// Abbreivated list combinator for the list grammar rules.
#[inline(always)]
pub fn list<F, G, I, O1, O2>(sep: F, parser: G) -> impl FnMut(I) -> IResult<I, Vec<O2>>
where
	F: FnMut(I) -> IResult<I, O1>,
	G: FnMut(I) -> IResult<I, O2>,
	I: Clone + InputLength + InputIter + InputTake + Slice<Range<usize>> + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
{
	separated_list1(sep, parser)
}

/// Strips the whitespace on both sides of `parser`.
///
/// There may or may not be whitespace.
#[inline(always)]
pub fn ws0<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputIter + InputTakeAtPosition,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(multispace0, parser, multispace0)
}

/// Strips the whitespace on both sides of an input character `ch`.
#[inline(always)]
pub fn wschar<I>(ch: char) -> impl FnMut(I) -> IResult<I, char>
where
	I: InputIter + InputTakeAtPosition + Slice<RangeFrom<usize>> + InputIter,
	<I as InputIter>::Item: AsChar,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
{
	ws0(tchar(ch))
}

/// Strips the whitespace on both sides of `pattern`
#[inline(always)]
pub fn wstag<T, I>(pattern: T) -> impl FnMut(I) -> IResult<I, I>
where
	T: InputLength + Clone,
	I: InputIter + InputTakeAtPosition + InputTake + Compare<T>,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
{
	ws0(tag(pattern))
}

/// Matches an object from `parser` encased in parenthesis.
#[inline(always)]
pub fn paren<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputIter + InputTakeAtPosition + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(wschar(LPAREN), parser, wschar(RPAREN))
}

/// Matches an object from `parser` encased in brackets.
#[inline(always)]
pub fn bracket<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputIter + InputTakeAtPosition + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(wschar(LBRACKET), parser, wschar(RBRACKET))
}

/// Matches an object from `parser` encased in braces.
#[inline(always)]
pub fn braces<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputIter + InputTakeAtPosition + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(wschar(LBRACE), parser, wschar(RBRACE))
}

/// Matches objects from the `first` and `second` parsers, which are separated by an assignment (`=`) operator.
#[inline(always)]
pub fn assign<F, G, I, O1, O2>(first: F, second: G) -> impl FnMut(I) -> IResult<I, (O1, O2)>
where
	I: InputIter + InputTakeAtPosition + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	<I as InputTakeAtPosition>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O1>,
	G: FnMut(I) -> IResult<I, O2>,
{
	separated_pair(first, wschar(EQUALS), second)
}
