use luna_ast::expression::Expression;

use crate::parse::expression::exp;

#[test]
fn knil() {
	assert_eq!(exp("nil"), Ok(("", Expression::Nil)));
}

#[test]
fn kfalse() {
	assert_eq!(exp("false"), Ok(("", Expression::False)));
}

#[test]
fn ktrue() {
	assert_eq!(exp("true"), Ok(("", Expression::True)));
}

#[test]
fn operators() {
	let input = "1+2";
	println!("{:?}", exp(input));
}
