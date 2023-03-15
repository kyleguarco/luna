#[derive(Clone, Debug)]
pub struct AttributeName(pub Identifier, pub Attribute);

#[derive(Clone, Debug)]
pub struct Attribute(pub Option<Identifier>);
