use nom::{IResult, error::ParseError, InputTakeAtPosition, AsChar, character::complete::multispace0, sequence::delimited};

mod stat;

pub fn chunk(input: &str) -> IResult<&str, &str> {
	block(input)
}

pub fn block(input: &str) -> IResult<&str, &str> {
	todo!()
}

pub fn stat(input: &str) -> IResult<&str, ()> {
	use stat;

	todo!()
}

pub fn attnamelist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn attrib(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn retstat(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn label(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn funcname(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn varlist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn var(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn namelist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn explist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn exp(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn prefixexp(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn functioncall(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn args(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn functiondef(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn funcbody(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn parlist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn tableconstructor(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn fieldlist(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn field(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn fieldsep(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn binop(input: &str) -> IResult<&str, ()> {
	todo!()
}

pub fn unop(input: &str) -> IResult<&str, ()> {
	todo!()
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
///
/// This function was taken from [the nom::recipes documentation][1]
///
/// [1]: https://docs.rs/nom/latest/nom/recipes/index.html#whitespace
pub fn whitespace<'a, T, F, O, E>(inner: F) -> impl FnMut(T) -> IResult<T, O, E>
where
	F: Fn(T) -> IResult<T, O, E> + 'a,
	E: ParseError<T>,
	T: InputTakeAtPosition + 'a,
	<T as InputTakeAtPosition>::Item: Clone + AsChar,
{
	delimited(multispace0, inner, multispace0)
}
