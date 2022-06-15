pub(crate) enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Div,
    FloorDiv,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    ShiftLeft,
    ShiftRight,
    Concat,
    Equal,
    LessThan,
    LessEqual,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}

impl BinaryOperator {
    /// Can the operator be folded (is it arithmetic or bitwise)?
    pub fn can_fold(&self) -> bool {
        match self {
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Mod
            | Self::Pow
            | Self::Div
            | Self::FloorDiv
            | Self::BinaryAnd
            | Self::BinaryOr
            | Self::BinaryXor
            | Self::ShiftLeft
            | Self::ShiftRight => true,
            _ => false
        }
    }
}

pub(crate) enum UnaryOperator {
    Minus,
    BinaryNot,
    Not,
    Length,
}


