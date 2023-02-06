use luna_ast::types::{
	AnonFunctionDefinition, Arguments, Attribute, AttributeNameList, Block, Expression,
	ExpressionList, Field, FieldList, FunctionBody, FunctionCall, FunctionIdentifier, Identifier,
	IdentifierList, InfixOperation, Label, ParameterList, PrefixExpression, PrefixOperation,
	ReturnStatement, TableConstructor, Variable, VariableList,
};
use nom::{
	character::complete::multispace0, error::ParseError, sequence::delimited, AsChar, IResult,
	InputTakeAtPosition,
};
pub use stat::stat;

mod stat;

pub fn block(input: &str) -> IResult<&str, Block> {
	todo!()
}

pub fn attnamelist(input: &str) -> IResult<&str, AttributeNameList> {
	todo!()
}

pub fn attrib(input: &str) -> IResult<&str, Attribute> {
	todo!()
}

pub fn retstat(input: &str) -> IResult<&str, ReturnStatement> {
	todo!()
}

pub fn label(input: &str) -> IResult<&str, Label> {
	Ok((input, Label(Identifier(""))))
}

pub fn funcname(input: &str) -> IResult<&str, FunctionIdentifier> {
	todo!()
}

pub fn varlist(input: &str) -> IResult<&str, VariableList> {
	todo!()
}

pub fn var(input: &str) -> IResult<&str, Variable> {
	todo!()
}

pub fn namelist(input: &str) -> IResult<&str, IdentifierList> {
	todo!()
}

pub fn explist(input: &str) -> IResult<&str, ExpressionList> {
	todo!()
}

pub fn exp(input: &str) -> IResult<&str, Expression> {
	todo!()
}

pub fn prefixexp(input: &str) -> IResult<&str, PrefixExpression> {
	todo!()
}

pub fn functioncall(input: &str) -> IResult<&str, FunctionCall> {
	todo!()
}

pub fn args(input: &str) -> IResult<&str, Arguments> {
	todo!()
}

pub fn functiondef(input: &str) -> IResult<&str, AnonFunctionDefinition> {
	todo!()
}

pub fn funcbody(input: &str) -> IResult<&str, FunctionBody> {
	todo!()
}

pub fn parlist(input: &str) -> IResult<&str, ParameterList> {
	todo!()
}

pub fn tableconstructor(input: &str) -> IResult<&str, TableConstructor> {
	todo!()
}

pub fn fieldlist(input: &str) -> IResult<&str, FieldList> {
	todo!()
}

pub fn field(input: &str) -> IResult<&str, Field> {
	todo!()
}

pub fn fieldsep(input: &str) -> IResult<&str, &str> {
	todo!()
}

pub fn binop(input: &str) -> IResult<&str, InfixOperation> {
	todo!()
}

pub fn unop(input: &str) -> IResult<&str, PrefixOperation> {
	todo!()
}

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
