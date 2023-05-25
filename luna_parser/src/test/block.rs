use luna_ast::{
	expression::Expression,
	statement::{Definition, Statement},
	terminal::{Name, Numeral},
	variable::Variable,
};
use nom::{combinator::opt, multi::many0, sequence::pair};

use crate::{combinator::ws0, parse::statement::stat, return_stat};

#[test]
fn one_block() {
	assert_eq!(
		pair(ws0(stat), opt(return_stat))("y=1\n"),
		Ok((
			"",
			(
				Statement::Definition(Definition {
					vlist: vec![Variable::Name(Name("y".to_string()))],
					elist: vec![Expression::Numeral(Numeral::Integer(1))]
				}),
				None
			)
		))
	)
}

#[test]
fn one_block_whitespace() {
	assert_eq!(
		pair(many0(ws0(stat)), opt(return_stat))(" y=1 ;\n"),
		Ok((
			"",
			(
				vec![
					Statement::Definition(Definition {
						vlist: vec![Variable::Name(Name("y".to_string()))],
						elist: vec![Expression::Numeral(Numeral::Integer(1))]
					}),
					Statement::End,
				],
				None
			)
		))
	)
}
