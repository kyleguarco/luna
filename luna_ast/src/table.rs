use crate::{expression::Expression, terminal::Name};

pub type FieldList = Vec<Field>;

#[derive(Clone, Debug)]
pub struct TableConstructor {
	pub oflist: Option<FieldList>,
}

impl From<TableConstructor> for Expression {
	fn from(value: TableConstructor) -> Self {
		Self::TableConstructor(value)
	}
}

#[derive(Clone, Debug)]
pub struct BracketField {
	pub tabexp: Expression,
	pub val: Expression,
}

impl From<BracketField> for Field {
	fn from(value: BracketField) -> Self {
		Self::BracketField(value)
	}
}

#[derive(Clone, Debug)]
pub struct NameField {
	pub tabname: Name,
	pub val: Expression,
}

impl From<NameField> for Field {
	fn from(value: NameField) -> Self {
		Self::NameField(value)
	}
}

#[derive(Clone, Debug)]
pub enum Field {
	BracketField(BracketField),
	NameField(NameField),
	Expression(Expression),
}
