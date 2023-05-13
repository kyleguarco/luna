use std::ops::{Range, RangeFrom};

use nom::{
	character::complete::{char as tchar, multispace0, multispace1},
	multi::separated_list1,
	sequence::{delimited, separated_pair},
	AsChar, IResult, InputIter, InputLength, InputTake, Slice, UnspecializedInput,
};

use crate::terminal::string::{COMMA, EQUALS, LBRACE, LBRACKET, LPAREN, RBRACE, RBRACKET, RPAREN};

/// Strips the whitespace on both sides of `parser`.
///
/// There may or may not be whitespace.
#[inline(always)]
pub fn ws0<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputLength + InputTake + InputIter + Clone + UnspecializedInput,
	<I as InputIter>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(multispace0, parser, multispace0)
}

/// Strips the whitespace on both sides of `parser`.
///
/// There must be whitespace.
#[inline(always)]
pub fn ws1<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputLength + InputTake + InputIter + Clone + UnspecializedInput,
	<I as InputIter>::Item: AsChar + Clone,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(multispace1, parser, multispace1)
}

/// Matches an object from `parser` encased in parenthesis.
#[inline(always)]
pub fn paren<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputLength + InputTake + InputIter + Slice<Range<usize>> + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(tchar(LPAREN), parser, tchar(RPAREN))
}

/// Matches an object from `parser` encased in brackets.
#[inline(always)]
pub fn bracket<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputLength + InputTake + InputIter + Slice<Range<usize>> + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(tchar(LBRACKET), parser, tchar(RBRACKET))
}

/// Matches an object from `parser` encased in braces.
#[inline(always)]
pub fn braces<F, I, O>(parser: F) -> impl FnMut(I) -> IResult<I, O>
where
	I: InputLength + InputTake + InputIter + Slice<Range<usize>> + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	F: FnMut(I) -> IResult<I, O>,
{
	delimited(tchar(LBRACE), parser, tchar(RBRACE))
}

/// Matches objects from the `first` and `second` parsers, which are separated by an assignment (`=`) operator.
#[inline(always)]
pub fn assign<F, G, I, O1, O2>(first: F, second: G) -> impl FnMut(I) -> IResult<I, (O1, O2)>
where
	I: InputLength + InputTake + InputIter + Slice<Range<usize>> + Slice<RangeFrom<usize>>,
	<I as InputIter>::Item: AsChar,
	F: FnMut(I) -> IResult<I, O1>,
	G: FnMut(I) -> IResult<I, O2>,
{
	separated_pair(first, tchar(EQUALS), second)
}
