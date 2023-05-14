use luna_ast::misc::ReturnStatement;
use nom::{
	bytes::complete::tag, character::complete::char as tchar, combinator::opt,
	multi::separated_list1, sequence::delimited, Parser,
};

use crate::{
	terminal::{
		keyword::KRETURN,
		string::{COMMA, SEMICOLON},
	},
	IRes, In,
};

use super::expression::exp;

pub fn return_stat(input: In) -> IRes<ReturnStatement> {
	delimited(
		tag(KRETURN),
		opt(separated_list1(tchar(COMMA), exp)),
		opt(tchar(SEMICOLON)),
	)
	.map(|oelist| ReturnStatement { oelist })
	.parse(input)
}
