use crate::{expression::Expression, terminal::Identifier};

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
pub struct IdentifierField {
	pub tabident: Identifier,
	pub val: Expression,
}

impl From<IdentifierField> for Field {
	fn from(value: IdentifierField) -> Self {
		Self::IdentifierField(value)
	}
}

#[derive(Clone, Debug)]
pub enum Field {
	BracketField(BracketField),
	IdentifierField(IdentifierField),
	Expression(Expression),
}
