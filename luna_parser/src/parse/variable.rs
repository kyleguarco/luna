use luna_ast::variable::Variable;
use nom::{branch::alt, character::complete::char as tchar, multi::many0, sequence::tuple, Parser};

use crate::{
	combinator::list,
	parse::affix::{index, prefix, suffix},
	terminal::{name, string::COMMA},
	IRes, In,
};

pub fn var(input: In) -> IRes<Variable> {
	use Variable::*;

	alt((
		name.map(Name),
		tuple((prefix, many0(suffix), index)).map(|(pfix, slist, index)| Indexed {
			pfix,
			slist,
			index,
		}),
	))
	.parse(input)
}

pub fn varlist(input: In) -> IRes<Vec<Variable>> {
	dbg!(input);
	list(tchar(COMMA), var)(input)
}
