use crate::types::Identifier;

#[derive(Clone, Debug)]
pub struct AttributeName {
	pub ident: Identifier,
	pub attr: Attribute
}

#[derive(Clone, Debug)]
pub struct Attribute(pub Option<Identifier>);
