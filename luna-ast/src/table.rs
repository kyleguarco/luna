use crate::{
	expression::{Expression, Value},
	function::Arguments,
	terminal::Name,
};

pub type FieldList = Vec<Field>;

#[derive(Clone, Debug, PartialEq)]
pub struct TableConstructor {
	pub oflist: Option<FieldList>,
}

impl From<TableConstructor> for Value {
	fn from(value: TableConstructor) -> Self {
		Self::TableConstructor(value)
	}
}

impl From<TableConstructor> for Arguments {
	fn from(value: TableConstructor) -> Self {
		Self::TableConstructor(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct BracketField {
	pub tabexp: Expression,
	pub val: Expression,
}

impl From<BracketField> for Field {
	fn from(value: BracketField) -> Self {
		Self::BracketField(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct NameField {
	pub tabname: Name,
	pub val: Expression,
}

impl From<NameField> for Field {
	fn from(value: NameField) -> Self {
		Self::NameField(value)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Field {
	BracketField(BracketField),
	NameField(NameField),
	Expression(Expression),
}
