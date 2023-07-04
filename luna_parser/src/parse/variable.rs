use luna_ast::variable::Variable;
use nom::{branch::alt, sequence::pair, Parser};

use crate::{
	combinator::{list, wschar},
	parse::affix::{affix, index},
	terminal::{name, string::COMMA},
	IRes, In,
};

pub fn var(input: In) -> IRes<Variable> {
	dbg!(input);
	use Variable::*;

	alt((
		name.map(Name),
		// TODO! many0(suffix) consumes the last required `index`, causing an error
		pair(affix, index).map(|(affix, index)| Indexed { affix, index }),
	))
	.parse(input)
}

pub fn varlist(input: In) -> IRes<Vec<Variable>> {
	dbg!(input);
	list(wschar(COMMA), var)(input)
}
