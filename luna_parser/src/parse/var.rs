use luna_ast::types::Variable;
use nom::{
	branch::alt,
	sequence::{delimited, separated_pair},
	IResult, bytes::complete::tag,
};

use crate::terminal::{
	identifier,
	string::{LEFTBRACKET, RIGHTBRACKET, DOT},
};

use super::{exp, prefixexp};

type IResultVar<'a> = IResult<&'a str, Variable<'a>>;

fn var_ident(input: &str) -> IResultVar {
	let (input, ident) = identifier(input)?;
	Ok((input, Variable::Identifier(ident)))
}

fn var_pexp(input: &str) -> IResultVar {
	let (input, pref) = prefixexp(input)?;
	let (input, pexp) = delimited(tag(LEFTBRACKET), exp, tag(RIGHTBRACKET))(input)?;
	Ok((input, Variable::PrefixExpressionIndex(pref, pexp)))
}

fn var_pident(input: &str) -> IResultVar {
	let (input, pair) = separated_pair(prefixexp, tag(DOT), identifier)(input)?;
	Ok((input, Variable::PrefixExpressionIdentifier(pair.0, pair.1)))
}

pub fn var(input: &str) -> IResult<&str, Variable> {
	alt((var_ident, var_pexp, var_pident))(input)
}
