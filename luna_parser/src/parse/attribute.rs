use luna_ast::attribute::{Attribute, AttributeName};
use nom::{character::complete::char as tchar, combinator::opt, sequence::delimited, Parser};

use crate::{
	terminal::{
		name,
		string::{GREATER, LESS},
	},
	IRes, In,
};

pub fn attrib_name(input: In) -> IRes<AttributeName> {
	name
		.and(attrib)
		.map(|(name, attr)| AttributeName { name, attr })
		.parse(input)
}

pub fn attrib(input: In) -> IRes<Attribute> {
	delimited(tchar(LESS), opt(name), tchar(GREATER))
		.map(|oname| Attribute { oname })
		.parse(input)
}
