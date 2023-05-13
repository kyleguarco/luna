use luna_ast::attribute::{Attribute, AttributeName};
use nom::{character::complete::char as tchar, combinator::opt, sequence::delimited, Parser};

use crate::{
	terminal::{
		identifier,
		string::{GREATER, LESS},
	},
	IRes, In,
};

pub fn attrib_name(input: In) -> IRes<AttributeName> {
	identifier
		.and(attrib)
		.map(|(ident, attr)| AttributeName { ident, attr })
		.parse(input)
}

pub fn attrib(input: In) -> IRes<Attribute> {
	delimited(tchar(LESS), opt(identifier), tchar(GREATER))
		.map(|oident| Attribute { oident })
		.parse(input)
}
