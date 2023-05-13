//! # Function Structure Parsers

use luna_ast::function::{CallFunction, CallObjectFunction, FunctionBody, FunctionIdentifier};

use crate::{IRes, In};

pub fn func_ident(input: In) -> IRes<FunctionIdentifier> {
	todo!()
}

pub fn call_func(input: In) -> IRes<CallFunction> {
	todo!()
}

pub fn call_obj_func(input: In) -> IRes<CallObjectFunction> {
	todo!()
}

pub fn func_body(input: In) -> IRes<FunctionBody> {
	todo!()
}
