use luna_ast::table::{BracketField, Field, FieldList, NameField, TableConstructor};
use nom::{
	branch::alt,
	character::complete::char as tchar,
	combinator::{opt, recognize},
	multi::separated_list1,
	sequence::terminated,
	Parser,
};

use crate::{
	combinator::{assign, braces, bracket},
	terminal::{
		name,
		string::{COMMA, SEMICOLON},
	},
	IRes, In,
};

use super::expression::exp;

pub fn table_cons(input: In) -> IRes<TableConstructor> {
	opt(braces(fieldlist))
		.map(|oflist| TableConstructor { oflist })
		.parse(input)
}

pub fn bracket_field(input: In) -> IRes<BracketField> {
	assign(bracket(exp), exp)
		.map(|(tabexp, val)| BracketField { tabexp, val })
		.parse(input)
}

pub fn name_field(input: In) -> IRes<NameField> {
	assign(name, exp)
		.map(|(tabname, val)| NameField { tabname, val })
		.parse(input)
}

pub fn fieldlist(input: In) -> IRes<FieldList> {
	terminated(separated_list1(fieldsep, field), opt(fieldsep))(input)
}

pub fn field(input: In) -> IRes<Field> {
	alt((
		bracket_field.map(Field::from),
		name_field.map(Field::from),
		exp.map(Field::from),
	))
	.parse(input)
}

pub fn fieldsep(input: In) -> IRes<In> {
	recognize(tchar(COMMA).or(tchar(SEMICOLON))).parse(input)
}
