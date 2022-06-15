use alloc::vec::Vec;

use crate::ltypes::{
    object::{Proto, TString, TValue},
    LuaByte, LuaInteger, LuaNumber,
};

pub enum ExpressionKind {
    Void,
    Nil,
    True,
    False,
    KConstant,
    KFloat,
    KInteger,
    KString,
    NonReloc,
    Local,
    UpVal,
    Static,
    Index,
    UpIndex,
    IntegerIndex,
    StringIndex,
    Jump,
    Reloc,
    Call,
    VarArg,
}

impl ExpressionKind {
    fn is_indexed(&self) -> bool {
        match self {
            Self::Index | Self::UpIndex | Self::IntegerIndex | Self::StringIndex => true,
            _ => false,
        }
    }

    fn is_variable(&self) -> bool {
        match self {
            Self::Local | Self::UpVal | Self::Static => self.is_indexed(),
            _ => false,
        }
    }
}
