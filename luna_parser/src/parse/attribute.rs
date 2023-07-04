use luna_ast::attribute::{Attribute, AttributeName};
use nom::{combinator::opt, sequence::delimited, Parser};

use crate::{
	combinator::{list, wschar},
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
	delimited(wschar(LESS), opt(name), wschar(GREATER))
		.map(Attribute)
		.parse(input)
}

pub fn attnamelist(input: In) -> IRes<Vec<AttributeName>> {
	dbg!(input);
	list(wschar(COMMA), attrib_name)(input)
}
