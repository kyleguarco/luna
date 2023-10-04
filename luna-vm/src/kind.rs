use std::fmt::Debug;

use crate::{
	formats::FormatKind,
	instrs::{Instruction, InstructionSize},
	mask::Mask,
};

pub type OpModeSize = u8;

/// Operating mode of an instruction.
#[derive(Clone, Copy)]
pub struct OpMode(OpModeSize);

impl OpMode {
	pub fn mode(&self) -> FormatKind {
		match self.0 & 7 {
			0 => FormatKind::ABC,
			1 => FormatKind::ABX,
			2 => FormatKind::ASBX,
			3 => FormatKind::AX,
			4 => FormatKind::ISJ,
			_ => unreachable!("Format identifier should be 0-4"),
		}
	}

	pub fn sets_reg_a(&self) -> bool {
		(self.0 & (1 << 3)) != 0
	}

	pub fn is_test(&self) -> bool {
		(self.0 & (1 << 4)) != 0
	}

	pub fn uses_top(&self) -> bool {
		(self.0 & (1 << 5)) != 0
	}

	pub fn sets_top(&self) -> bool {
		(self.0 & (1 << 6)) != 0
	}

	pub fn calls_metamethod(&self) -> bool {
		(self.0 & (1 << 7)) != 0
	}
}

impl Debug for OpMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("OpMode")
			.field("mode",&self.mode())
			.field("a", &self.sets_reg_a())
			.field("t", &self.is_test())
			.field("it", &self.uses_top())
			.field("ot", &self.sets_top())
			.field("mm", &self.calls_metamethod())
			.finish()
	}
}

// There should only be a 1-way conversion, since we can no longer
// guarantee the validity of this byte.
impl Into<u8> for OpMode {
	fn into(self) -> u8 {
		self.0
	}
}

/// Specifies flags in a raw operation mode byte.
///
/// # Arguments
///
/// * `mm` - Calls a metamethod.
/// * `ot` - Sets the top of the stack for the next instruction (out top).
/// * `it` - Uses the top of the stack (in top).
/// * `t` - Next instruction must be a jump (this is a test).
/// * `a` - Sets the `A` register.
const fn opmode(mm: bool, ot: bool, it: bool, t: bool, a: bool, mode: FormatKind) -> OpMode {
	OpMode(
		((mm as u8) << 7)
			| ((ot as u8) << 6)
			| ((it as u8) << 5)
			| ((t as u8) << 4)
			| ((a as u8) << 3)
			| (mode as u8),
	)
}

pub type OpCodeSize = u8;

/// All the possible instructions for the Lua VM.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	Move = 0,
	LoadI = 1,
	LoadF = 2,
	LoadK = 3,
	LoadKX = 4,
	LoadFalse = 5,
	LFalseSkip = 6,
	LoadTrue = 7,
	LoadNil = 8,
	GetUpval = 9,
	SetUpval = 10,
	GetTabUp = 11,
	GetTable = 12,
	GetI = 13,
	GetField = 14,
	SetTabUp = 15,
	SetTable = 16,
	SetI = 17,
	SetField = 18,
	NewTable = 19,
	ISelf = 20,
	AddI = 21,
	AddK = 22,
	SubK = 23,
	MulK = 24,
	ModK = 25,
	PowK = 26,
	DivK = 27,
	IDivK = 28,
	BAndK = 29,
	BOrK = 30,
	BXorK = 31,
	ShrI = 32,
	ShlI = 33,
	Add = 34,
	Sub = 35,
	Mul = 36,
	Mod = 37,
	Pow = 38,
	Div = 39,
	IDiv = 40,
	BAnd = 41,
	BOr = 42,
	BXor = 43,
	Shl = 44,
	Shr = 45,
	MMBin = 46,
	MMBinI = 47,
	MMBinK = 48,
	UnM = 49,
	BNot = 50,
	Not = 51,
	Len = 52,
	Concat = 53,
	Close = 54,
	Tbc = 55,
	Jmp = 56,
	Eq = 57,
	Lt = 58,
	Le = 59,
	EqK = 60,
	EqI = 61,
	LtI = 62,
	LeI = 63,
	GtI = 64,
	GeI = 65,
	Test = 66,
	TestSet = 67,
	Call = 68,
	TailCall = 69,
	Return = 70,
	Return0 = 71,
	Return1 = 72,
	ForLoop = 73,
	ForPrep = 74,
	TForPrep = 75,
	TForCall = 76,
	TForLoop = 77,
	SetList = 78,
	Closure = 79,
	VarArg = 80,
	VarArgPrep = 81,
	ExtraArg = 82,
}

impl OpCode {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Move => "MOVE",
			Self::LoadI => "LOADI",
			Self::LoadF => "LOADF",
			Self::LoadK => "LOADK",
			Self::LoadKX => "LOADKX",
			Self::LoadFalse => "LOADFALSE",
			Self::LFalseSkip => "LFALSESKIP",
			Self::LoadTrue => "LOADTRUE",
			Self::LoadNil => "LOADNIL",
			Self::GetUpval => "GETUPVAL",
			Self::SetUpval => "SETUPVAL",
			Self::GetTabUp => "GETTABUP",
			Self::GetTable => "GETTABLE",
			Self::GetI => "GETI",
			Self::GetField => "GETFIELD",
			Self::SetTabUp => "SETTABUP",
			Self::SetTable => "SETTABLE",
			Self::SetI => "SETI",
			Self::SetField => "SETFIELD",
			Self::NewTable => "NEWTABLE",
			Self::ISelf => "SELF",
			Self::AddI => "ADDI",
			Self::AddK => "ADDK",
			Self::SubK => "SUBK",
			Self::MulK => "MULK",
			Self::ModK => "MODK",
			Self::PowK => "POWK",
			Self::DivK => "DIVK",
			Self::IDivK => "IDIVK",
			Self::BAndK => "BANDK",
			Self::BOrK => "BORK",
			Self::BXorK => "BXORK",
			Self::ShrI => "SHRI",
			Self::ShlI => "SHLI",
			Self::Add => "ADD",
			Self::Sub => "SUB",
			Self::Mul => "MUL",
			Self::Mod => "MOD",
			Self::Pow => "POW",
			Self::Div => "DIV",
			Self::IDiv => "IDIV",
			Self::BAnd => "BAND",
			Self::BOr => "BOR",
			Self::BXor => "BXOR",
			Self::Shl => "SHL",
			Self::Shr => "SHR",
			Self::MMBin => "MMBIN",
			Self::MMBinI => "MMBINI",
			Self::MMBinK => "MMBINK",
			Self::UnM => "UNM",
			Self::BNot => "BNOT",
			Self::Not => "NOT",
			Self::Len => "LEN",
			Self::Concat => "CONCAT",
			Self::Close => "CLOSE",
			Self::Tbc => "TBC",
			Self::Jmp => "JMP",
			Self::Eq => "EQ",
			Self::Lt => "LT",
			Self::Le => "LE",
			Self::EqK => "EQK",
			Self::EqI => "EQI",
			Self::LtI => "LTI",
			Self::LeI => "LEI",
			Self::GtI => "GTI",
			Self::GeI => "GEI",
			Self::Test => "TEST",
			Self::TestSet => "TESTSET",
			Self::Call => "CALL",
			Self::TailCall => "TAILCALL",
			Self::Return => "RETURN",
			Self::Return0 => "RETURN0",
			Self::Return1 => "RETURN1",
			Self::ForLoop => "FORLOOP",
			Self::ForPrep => "FORPREP",
			Self::TForPrep => "TFORPREP",
			Self::TForCall => "TFORCALL",
			Self::TForLoop => "TFORLOOP",
			Self::SetList => "SETLIST",
			Self::Closure => "CLOSURE",
			Self::VarArg => "VARARG",
			Self::VarArgPrep => "VARARGPREP",
			Self::ExtraArg => "EXTRAARG",
		}
	}

	pub const fn to_opmode(&self) -> OpMode {
		match self {
			Self::Move => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::LoadI => opmode(false, false, false, false, true, FormatKind::ASBX),
			Self::LoadF => opmode(false, false, false, false, true, FormatKind::ASBX),
			Self::LoadK => opmode(false, false, false, false, true, FormatKind::ABX),
			Self::LoadKX => opmode(false, false, false, false, true, FormatKind::ABX),
			Self::LoadFalse => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::LFalseSkip => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::LoadTrue => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::LoadNil => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::GetUpval => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::SetUpval => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::GetTabUp => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::GetTable => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::GetI => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::GetField => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::SetTabUp => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::SetTable => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::SetI => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::SetField => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::NewTable => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::ISelf => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::AddI => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::AddK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::SubK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::MulK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::ModK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::PowK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::DivK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::IDivK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BAndK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BOrK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BXorK => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::ShrI => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::ShlI => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Add => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Sub => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Mul => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Mod => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Pow => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Div => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::IDiv => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BAnd => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BOr => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BXor => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Shl => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Shr => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::MMBin => opmode(true, false, false, false, false, FormatKind::ABC),
			Self::MMBinI => opmode(true, false, false, false, false, FormatKind::ABC),
			Self::MMBinK => opmode(true, false, false, false, false, FormatKind::ABC),
			Self::UnM => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::BNot => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Not => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Len => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Concat => opmode(false, false, false, false, true, FormatKind::ABC),
			Self::Close => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::Tbc => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::Jmp => opmode(false, false, false, false, false, FormatKind::ISJ),
			Self::Eq => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::Lt => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::Le => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::EqK => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::EqI => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::LtI => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::LeI => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::GtI => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::GeI => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::Test => opmode(false, false, false, true, false, FormatKind::ABC),
			Self::TestSet => opmode(false, false, false, true, true, FormatKind::ABC),
			Self::Call => opmode(false, true, true, false, true, FormatKind::ABC),
			Self::TailCall => opmode(false, true, true, false, true, FormatKind::ABC),
			Self::Return => opmode(false, false, true, false, false, FormatKind::ABC),
			Self::Return0 => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::Return1 => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::ForLoop => opmode(false, false, false, false, true, FormatKind::ABX),
			Self::ForPrep => opmode(false, false, false, false, true, FormatKind::ABX),
			Self::TForPrep => opmode(false, false, false, false, false, FormatKind::ABX),
			Self::TForCall => opmode(false, false, false, false, false, FormatKind::ABC),
			Self::TForLoop => opmode(false, false, false, false, true, FormatKind::ABX),
			Self::SetList => opmode(false, false, true, false, false, FormatKind::ABC),
			Self::Closure => opmode(false, false, false, false, true, FormatKind::ABX),
			Self::VarArg => opmode(false, true, false, false, true, FormatKind::ABC),
			Self::VarArgPrep => opmode(false, false, true, false, true, FormatKind::ABC),
			Self::ExtraArg => opmode(false, false, false, false, false, FormatKind::AX),
		}
	}
}
