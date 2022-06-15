// The order of declaration is preserved here (lopcode.h)
pub enum OpCode {
    Move,
    LoadI,
    LoadF,
    LoadK,
    LoadKX,
    LoadFalse,
    LoadFalseSkip,
    LoadTrue,
    LoadNil,
    GetUpVal,
    SetUpVal,
    GetTabUp,
    GetTable,
    GetI,
    GetField,
    SetTabUp,
    SetTable,
    SetI,
    SetField,
    NewTable,
    LSelf,
    AddI,
    AddK,
    SubK,
    MulK,
    ModK,
    PowK,
    DivK,
    IDivK,
    BinaryAndK,
    BinaryOrK,
    BinaryXorK,
    ShiftRightK,
    ShiftLeftK,
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Div,
    IDiv,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    ShiftLeft,
    ShiftRight,
    MetaMethodBin,
    MetaMethodBinI,
    MetaMethodBinK,
    UseNegative,
    BinaryNot,
    Not,
    Len,
    Concat,
    Close,
    ToBeClosed,
    Jump,
    Equal,
    LessThan,
    LessThanEqual,
    EqualK,
    EqualI,
    LessThanI,
    LessEqualI,
    GreaterThanI,
    GreaterEqualI,
    Test,
    TestSet,
    Call,
    TailCall,
    Return,
    ReturnNone,
    ReturnValue,
    ForLoop,
    ForPrep,
    TForPrep,
    TForCall,
    TForLoop,
    SetList,
    Closure,
    VarArg,
    VarArgPrep,
    ExtraArg,
}
