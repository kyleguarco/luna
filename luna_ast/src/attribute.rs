use crate::terminal::Name;

/// Grammar: `Name attrib {',' Name attrib}`
pub type AttributeNameList = Vec<AttributeName>;

/// See [AttributeNameList].
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeName {
	pub name: Name,
	pub attr: Attribute,
}

/// A [variable](crate::variable::Variable) attribute. The attribute can
/// either be `const` or `close` as denoted by the Lua specification.
///
/// Grammar: `['<' Name '>']`
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
	pub oname: Option<Name>,
}
