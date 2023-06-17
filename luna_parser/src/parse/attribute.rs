use luna_ast::attribute::{Attribute, AttributeName};
use nom::{character::complete::char as tchar, combinator::opt, sequence::delimited, Parser};

use crate::{
	combinator::list,
	terminal::{
		name,
		string::{COMMA, GREATER, LESS},
	},
	IRes, In,
};

fn attrib_name(input: In) -> IRes<AttributeName> {
	dbg!(input);
	name.and(attrib)
		.map(|(name, attr)| AttributeName { name, attr })
		.parse(input)
}

pub fn attrib(input: In) -> IRes<Attribute> {
	dbg!(input);
	delimited(tchar(LESS), opt(name), tchar(GREATER))
		.map(|oname| Attribute { oname })
		.parse(input)
}

pub fn attnamelist(input: In) -> IRes<Vec<AttributeName>> {
	dbg!(input);
	list(tchar(COMMA), attrib_name)(input)
}
