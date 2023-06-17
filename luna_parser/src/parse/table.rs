use luna_ast::table::{BracketField, Field, FieldList, NameField, TableConstructor};
use nom::{
	branch::alt,
	character::complete::char as tchar,
	combinator::{opt, recognize},
	sequence::terminated,
	Parser,
};

use crate::{
	combinator::{assign, braces, bracket, list},
	terminal::{
		name,
		string::{COMMA, SEMICOLON},
	},
	IRes, In,
};

use super::expression::exp;

pub fn tableconstructor(input: In) -> IRes<TableConstructor> {
	dbg!(input);
	opt(braces(fieldlist))
		.map(|oflist| TableConstructor { oflist })
		.parse(input)
}

fn bracket_field(input: In) -> IRes<BracketField> {
	dbg!(input);
	assign(bracket(exp), exp)
		.map(|(tabexp, val)| BracketField { tabexp, val })
		.parse(input)
}

fn name_field(input: In) -> IRes<NameField> {
	dbg!(input);
	assign(name, exp)
		.map(|(tabname, val)| NameField { tabname, val })
		.parse(input)
}

fn fieldlist(input: In) -> IRes<FieldList> {
	dbg!(input);
	terminated(list(fieldsep, field), opt(fieldsep))(input)
}

pub fn field(input: In) -> IRes<Field> {
	dbg!(input);
	alt((
		bracket_field.map(Field::from),
		name_field.map(Field::from),
		exp.map(Field::from),
	))
	.parse(input)
}

pub fn fieldsep(input: In) -> IRes<In> {
	dbg!(input);
	recognize(tchar(COMMA).or(tchar(SEMICOLON))).parse(input)
}
