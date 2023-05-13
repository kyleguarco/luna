use crate::{expression::Expression, terminal::Identifier};

pub type FieldList = Vec<Field>;

#[derive(Clone, Debug)]
pub struct TableConstructor {
	pub oflist: Option<FieldList>,
}

#[derive(Clone, Debug)]
pub struct BracketField {
	pub tabexp: Expression,
	pub val: Expression,
}

#[derive(Clone, Debug)]
pub struct IdentifierField {
	pub tabident: Identifier,
	pub val: Expression,
}

#[derive(Clone, Debug)]
pub enum Field {
	BracketField(BracketField),
	IdentifierField(IdentifierField),
	Expression(Expression),
}
