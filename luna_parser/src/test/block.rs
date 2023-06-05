use luna_ast::{
	expression::Expression,
	statement::{Definition, Statement},
	terminal::{Name, Numeral},
	variable::Variable,
};

use crate::{chunk, test::test_chunk};

#[test]
fn empty() {
	assert_eq!(
		chunk(""),
		Err(crate::error::ParseError {
			input: "",
			kind: crate::error::ErrorKind::Eof
		})
	)
}

#[test]
fn one_block() {
	assert_eq!(
		chunk("y=1\n"),
		Ok(test_chunk(
			vec![Statement::Definition(Definition {
				vlist: vec![Variable::Name(Name("y".to_string()))],
				elist: vec![Expression::Numeral(Numeral::Integer(1))]
			}),],
			None,
		))
	)
}

#[test]
fn one_block_whitespace() {
	assert_eq!(
		chunk(" y=1 ;\n"),
		Ok(test_chunk(
			vec![
				Statement::Definition(Definition {
					vlist: vec![Variable::Name(Name("y".to_string()))],
					elist: vec![Expression::Numeral(Numeral::Integer(1))]
				}),
				Statement::End,
			],
			None
		))
	)
}
