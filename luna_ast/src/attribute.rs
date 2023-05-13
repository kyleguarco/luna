use crate::terminal::Identifier;

pub type AttributeNameList = Vec<AttributeName>;

#[derive(Clone, Debug)]
pub struct AttributeName {
	pub ident: Identifier,
	pub attr: Attribute,
}

#[derive(Clone, Debug)]
pub struct Attribute {
	pub oident: Option<Identifier>,
}
