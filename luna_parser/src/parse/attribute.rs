use luna_ast::attribute::{Attribute, AttributeName};
use nom::{character::complete::char as tchar, combinator::opt, sequence::delimited, Parser};

use crate::{
	terminal::{
		name,
		string::{GREATER, LESS, COMMA},
	},
	IRes, In, combinator::list,
};

pub fn attrib_name(input: In) -> IRes<AttributeName> {
	name.and(attrib)
		.map(|(name, attr)| AttributeName { name, attr })
		.parse(input)
}

pub fn attrib(input: In) -> IRes<Attribute> {
	delimited(tchar(LESS), opt(name), tchar(GREATER))
		.map(|oname| Attribute { oname })
		.parse(input)
}

pub fn att_name_list(input: In) -> IRes<Vec<AttributeName>> {
	list(tchar(COMMA), attrib_name)(input)
}
