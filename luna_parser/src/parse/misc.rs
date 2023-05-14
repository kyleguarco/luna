use luna_ast::misc::{Label, ReturnStatement};
use nom::{
	bytes::complete::tag, character::complete::char as tchar, combinator::opt, sequence::delimited,
	Parser,
};

use crate::{
	combinator::list,
	terminal::{
		keyword::KRETURN,
		name,
		string::{COMMA, DOUBLECOLON, SEMICOLON},
	},
	IRes, In,
};

use super::expression::exp;

pub fn label(input: In) -> IRes<Label> {
	delimited(tag(DOUBLECOLON), name, tag(DOUBLECOLON))
		.map(|name| Label(name))
		.parse(input)
}

pub fn return_stat(input: In) -> IRes<ReturnStatement> {
	delimited(
		tag(KRETURN),
		opt(list(tchar(COMMA), exp)),
		opt(tchar(SEMICOLON)),
	)
	.map(|oelist| ReturnStatement { oelist })
	.parse(input)
}
