use luna_ast::variable::Variable;
use nom::{branch::alt, Parser};

use crate::{
	combinator::{list, wschar},
	parse::affix::affix,
	terminal::{name, string::COMMA},
	IRes, In,
};

pub fn var(input: In) -> IRes<Variable> {
	dbg!(input);
	use Variable::*;

	alt((affix.map(Affixed), name.map(Name))).parse(input)
}

pub fn varlist(input: In) -> IRes<Vec<Variable>> {
	dbg!(input);
	list(wschar(COMMA), var)(input)
}
