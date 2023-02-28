use luna_ast::types::Expression;
use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::value,
	sequence::{pair, tuple},
	IResult,
};

use crate::terminal::{
	keyword::{keyword, Keyword},
	literal_string, numeral,
	string::TRIPLEDOT,
};

use super::{binop, functiondef, prefixexp, tableconstructor, unop};

type IResultExp<'a> = IResult<&'a str, Expression>;

fn exp_num(input: &str) -> IResultExp {
	let (input, num) = numeral(input)?;
	Ok((input, Expression::Numeral(num)))
}

fn exp_lit(input: &str) -> IResultExp {
	let (input, lit) = literal_string(input)?;
	Ok((input, Expression::LiteralString(lit)))
}

fn exp_funcdef(input: &str) -> IResultExp {
	let (input, fdef) = functiondef(input)?;
	Ok((input, Expression::AnonFunctionDefinition(fdef)))
}

fn exp_prefexp(input: &str) -> IResultExp {
	let (input, pexp) = prefixexp(input)?;
	Ok((input, Expression::PrefixExpression(pexp)))
}

fn exp_tabcon(input: &str) -> IResultExp {
	let (input, tabcon) = tableconstructor(input)?;
	Ok((input, Expression::TableConstructor(tabcon)))
}

fn exp_binop(input: &str) -> IResultExp {
	let (input, (expa, itype, expb)) = tuple((exp, binop, exp))(input)?;
	Ok((
		input,
		Expression::InfixOperation(Box::new(expa), itype, Box::new(expb)),
	))
}

fn exp_unop(input: &str) -> IResultExp {
	let (input, (utype, exp)) = pair(unop, exp)(input)?;
	Ok((input, Expression::PrefixOperation(utype, Box::new(exp))))
}

pub fn exp(input: &str) -> IResultExp {
	alt((
		value(Expression::Nil, keyword(Keyword::Nil)),
		value(Expression::False, keyword(Keyword::False)),
		value(Expression::True, keyword(Keyword::True)),
		exp_num,
		exp_lit,
		value(Expression::VarArgs, tag(TRIPLEDOT)),
		exp_funcdef,
		exp_prefexp,
		exp_tabcon,
		exp_binop,
		exp_unop,
	))(input)
}
