//! # Function Structure Parsers

use luna_ast::function::{AsFunction, AsMethod, FunctionBody, FunctionIdentifier, FunctionCall};

use crate::{IRes, In};

pub fn func_ident(input: In) -> IRes<FunctionIdentifier> {
	todo!()
}

pub fn as_func(input: In) -> IRes<AsFunction> {
	todo!()
}

pub fn as_method(input: In) -> IRes<AsMethod> {
	todo!()
}

pub fn func_body(input: In) -> IRes<FunctionBody> {
	todo!()
}

pub fn func_call(input: In) -> IRes<FunctionCall> {
	todo!()
}
