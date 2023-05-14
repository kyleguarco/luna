use crate::terminal::Name;

pub type AttributeNameList = Vec<AttributeName>;

#[derive(Clone, Debug)]
pub struct AttributeName {
	pub name: Name,
	pub attr: Attribute,
}

#[derive(Clone, Debug)]
pub struct Attribute {
	pub oname: Option<Name>,
}
