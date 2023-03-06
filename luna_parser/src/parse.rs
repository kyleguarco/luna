use crate::terminal::{
	identifier,
	keyword::{keyword, Keyword},
	literal_string,
	string::{
		AMPHERSAND, CARET, COLON, COMMA, DOT, DOUBLECOLON, DOUBLEDOT, DOUBLESLASH, EQUALS, GREATER,
		GREATEREQUAL, ISEQUAL, LEFTBRACE, LEFTBRACKET, LEFTPAREN, LESS, LESSEQUAL, MINUS, NOTEQUAL,
		OCTOTHORPE, PERCENT, PIPE, PLUS, RIGHTBRACE, RIGHTBRACKET, RIGHTPAREN, SEMICOLON,
		SHIFTLEFT, SHIFTRIGHT, SLASH, STAR, TILDE, TRIPLEDOT,
	},
};
use luna_ast::types::{
	AnonFunctionDefinition, Arguments, Attribute, AttributeName, AttributeNameList, Block,
	ExpressionList, Field, FieldList, FunctionBody, FunctionCall, FunctionIdentifier,
	IdentifierList, InfixOperation, Label, ParameterList, PrefixExpression, PrefixOperation,
	ReturnStatement, TableConstructor, VariableList,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, opt, recognize, value},
	multi::{many0, many1, separated_list1},
	sequence::{delimited, pair, preceded, separated_pair, terminated},
	IResult,
};

pub use exp::exp;
pub use stat::stat;
pub use var::var;

mod exp;
mod stat;
mod var;

pub fn block(input: &str) -> IResult<&str, Block> {
	let (input, slist) = many0(stat)(input)?;
	let (input, ret) = opt(retstat)(input)?;
	Ok((input, Block(slist, ret)))
}

pub fn attnamelist(input: &str) -> IResult<&str, AttributeNameList> {
	fn inner(input: &str) -> IResult<&str, AttributeName> {
		let (input, ident) = identifier(input)?;
		let (input, attr) = attrib(input)?;
		Ok((input, AttributeName(ident, attr)))
	}

	let (input, alist) = many1(terminated(inner, tag(COMMA)))(input)?;
	Ok((input, AttributeNameList(alist)))
}

pub fn attrib(input: &str) -> IResult<&str, Attribute> {
	let (input, attr) = opt(delimited(tag(LESS), identifier, tag(LESS)))(input)?;
	Ok((input, Attribute(attr)))
}

pub fn retstat(input: &str) -> IResult<&str, ReturnStatement> {
	let (input, _) = keyword(Keyword::Return)(input)?;
	let (input, elist) = opt(explist)(input)?;
	let (input, _) = opt(tag(SEMICOLON))(input)?;
	Ok((input, ReturnStatement(elist)))
}

pub fn label(input: &str) -> IResult<&str, Label> {
	let (input, _) = tag(DOUBLECOLON)(input)?;
	let (input, ident) = identifier(input)?;
	let (input, _) = tag(DOUBLECOLON)(input)?;

	Ok((input, Label(ident)))
}

pub fn funcname(input: &str) -> IResult<&str, FunctionIdentifier> {
	let (input, ilist) = separated_list1(tag(DOT), identifier)(input)?;
	let (input, objident) = opt(preceded(tag(COLON), identifier))(input)?;
	Ok((input, FunctionIdentifier { ilist, objident }))
}

pub fn varlist(input: &str) -> IResult<&str, VariableList> {
	// let (input, vlist) = many0(preceded(tag(COMMA), var))(input)?;
	let (input, vlist) = separated_list1(tag(COMMA), var)(input)?;
	Ok((input, VariableList(vlist)))
}

pub fn namelist(input: &str) -> IResult<&str, IdentifierList> {
	let (input, ilist) = many1(terminated(identifier, tag(COMMA)))(input)?;
	Ok((input, IdentifierList(ilist)))
}

pub fn explist(input: &str) -> IResult<&str, ExpressionList> {
	let (input, elist) = many1(terminated(exp, tag(COMMA)))(input)?;
	Ok((input, ExpressionList(elist)))
}

pub fn prefixexp(input: &str) -> IResult<&str, PrefixExpression> {
	alt((
		map(var, |v| PrefixExpression::Variable(v)),
		map(functioncall, |fcall| {
			PrefixExpression::FunctionCall(Box::new(fcall))
		}),
		map(delimited(tag(LEFTPAREN), exp, tag(RIGHTPAREN)), |ex| {
			PrefixExpression::ClosedExpression(ex)
		}),
	))(input)
}

pub fn functioncall(input: &str) -> IResult<&str, FunctionCall> {
	alt((
		map(pair(prefixexp, args), |(pexp, argu)| {
			FunctionCall::CallFunction(pexp, argu)
		}),
		map(
			pair(separated_pair(prefixexp, tag(COLON), identifier), args),
			|((pexp, ident), argu)| FunctionCall::CallObjectFunction(pexp, ident, argu),
		),
	))(input)
}

pub fn args(input: &str) -> IResult<&str, Arguments> {
	alt((
		map(
			delimited(tag(LEFTPAREN), opt(explist), tag(RIGHTPAREN)),
			|elist| Arguments::ClosedExpressionList(elist),
		),
		map(tableconstructor, |tbc| Arguments::TableConstructor(tbc)),
		map(literal_string, |ls| Arguments::LiteralString(ls)),
	))(input)
}

pub fn functiondef(input: &str) -> IResult<&str, AnonFunctionDefinition> {
	map(preceded(keyword(Keyword::Function), funcbody), |fbody| {
		AnonFunctionDefinition(fbody)
	})(input)
}

pub fn funcbody(input: &str) -> IResult<&str, FunctionBody> {
	let (input, plist) = delimited(tag(LEFTPAREN), opt(parlist), tag(RIGHTPAREN))(input)?;
	let (input, bl) = block(input)?;
	let (input, _) = keyword(Keyword::End)(input)?;
	Ok((input, FunctionBody(plist, bl)))
}

pub fn parlist(input: &str) -> IResult<&str, ParameterList> {
	alt((
		map(
			pair(namelist, opt(recognize(pair(tag(COMMA), tag(TRIPLEDOT))))),
			|(nlist, vargs)| match vargs {
				Some(_) => ParameterList::IdentifierListWithVarArgs(nlist),
				None => ParameterList::IdentifierList(nlist),
			},
		),
		value(ParameterList::VarArgs, recognize(tag(TRIPLEDOT))),
	))(input)
}

pub fn tableconstructor(input: &str) -> IResult<&str, TableConstructor> {
	map(
		delimited(tag(LEFTBRACE), opt(fieldlist), tag(RIGHTBRACE)),
		|flist| TableConstructor(flist),
	)(input)
}

pub fn fieldlist(input: &str) -> IResult<&str, FieldList> {
	map(
		terminated(separated_list1(fieldsep, field), opt(fieldsep)),
		|flist| FieldList(flist),
	)(input)
}

pub fn field(input: &str) -> IResult<&str, Field> {
	alt((
		map(
			separated_pair(
				delimited(tag(LEFTBRACKET), exp, tag(RIGHTBRACKET)),
				tag(EQUALS),
				exp,
			),
			|(ex1, ex2)| Field::BracketField(ex1, ex2),
		),
		map(
			separated_pair(identifier, tag(EQUALS), exp),
			|(ident, ex)| Field::IdentifierField(ident, ex),
		),
	))(input)
}

pub fn fieldsep(input: &str) -> IResult<&str, &str> {
	alt((tag(COMMA), tag(SEMICOLON)))(input)
}

pub fn binop(input: &str) -> IResult<&str, InfixOperation> {
	alt((
		value(InfixOperation::Add, tag(PLUS)),
		value(InfixOperation::Subtract, tag(MINUS)),
		value(InfixOperation::Multiply, tag(STAR)),
		value(InfixOperation::Divide, tag(SLASH)),
		value(InfixOperation::FloorDivide, tag(DOUBLESLASH)),
		value(InfixOperation::Power, tag(CARET)),
		value(InfixOperation::Modulo, tag(PERCENT)),
		value(InfixOperation::BitwiseAnd, tag(AMPHERSAND)),
		value(InfixOperation::BitwiseXor, tag(TILDE)),
		value(InfixOperation::BitwiseOr, tag(PIPE)),
		value(InfixOperation::BitwiseRightShift, tag(SHIFTRIGHT)),
		value(InfixOperation::BitwiseLeftShift, tag(SHIFTLEFT)),
		value(InfixOperation::Concat, tag(DOUBLEDOT)),
		value(InfixOperation::LessThan, tag(LESS)),
		value(InfixOperation::LessEqual, tag(LESSEQUAL)),
		value(InfixOperation::GreaterThan, tag(GREATER)),
		value(InfixOperation::GreaterEqual, tag(GREATEREQUAL)),
		value(InfixOperation::IsEqual, tag(ISEQUAL)),
		value(InfixOperation::IsNotEqual, tag(NOTEQUAL)),
		value(InfixOperation::And, keyword(Keyword::And)),
		value(InfixOperation::Or, keyword(Keyword::Or)),
	))(input)
}

pub fn unop(input: &str) -> IResult<&str, PrefixOperation> {
	alt((
		value(PrefixOperation::Not, tag(MINUS)),
		value(PrefixOperation::Negate, keyword(Keyword::Not)),
		value(PrefixOperation::Length, tag(OCTOTHORPE)),
		value(PrefixOperation::BitwiseNot, tag(TILDE)),
	))(input)
}
