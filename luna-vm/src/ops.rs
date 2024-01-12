mod code;
mod mode;

/// Operating mode of an instruction.
#[derive(Clone, Copy)]
pub struct OpMode(u8);

impl std::fmt::Debug for OpMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("OpMode")
			.field("format", &self.format())
			.field("a", &self.sets_reg_a())
			.field("t", &self.is_test())
			.field("it", &self.uses_top())
			.field("ot", &self.sets_top())
			.field("mm", &self.calls_metamethod())
			.finish()
	}
}

/// An alias for the unsigned byte `u8` that represents a value for one of
/// the 83 possible Lua opcodes.
pub type OpCodeId = u8;

/// Possible operation codes for instructions on the Lua Virtual Machine.
#[derive(Clone, Copy)]
pub enum OpCode {
	Move,
	LoadI,
	LoadF,
	LoadK,
	LoadKX,
	LoadFalse,
	LFalseSkip,
	LoadTrue,
	LoadNil,
	GetUpval,
	SetUpval,
	GetTabUp,
	GetTable,
	GetI,
	GetField,
	SetTabUp,
	SetTable,
	SetI,
	SetField,
	NewTable,
	ISelf,
	AddI,
	AddK,
	SubK,
	MulK,
	ModK,
	PowK,
	DivK,
	IDivK,
	BAndK,
	BOrK,
	BXorK,
	ShrI,
	ShlI,
	Add,
	Sub,
	Mul,
	Mod,
	Pow,
	Div,
	IDiv,
	BAnd,
	BOr,
	BXor,
	Shl,
	Shr,
	MMBin,
	MMBinI,
	MMBinK,
	UnM,
	BNot,
	Not,
	Len,
	Concat,
	Close,
	Tbc,
	Jmp,
	Eq,
	Lt,
	Le,
	EqK,
	EqI,
	LtI,
	LeI,
	GtI,
	GeI,
	Test,
	TestSet,
	Call,
	TailCall,
	Return,
	Return0,
	Return1,
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

pub struct InvalidOpCodeId(pub u8);

impl TryFrom<OpCodeId> for OpCode {
	type Error = InvalidOpCodeId;

	fn try_from(value: OpCodeId) -> Result<Self, Self::Error> {
		Self::from_opcodeid(value).ok_or(InvalidOpCodeId(value))
	}
}

impl std::fmt::Debug for OpCode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("OpCode")
			.field(&self.name())
			.field(&self.mode())
			.finish()
	}
}
