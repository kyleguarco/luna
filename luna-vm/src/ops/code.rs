use std::fmt::Debug;

use crate::{
	cn,
	formats::{DecodableFormat, ABC, ABX, ASBX, AX, ISJ},
};

use super::mode::{FormatKind, OpMode};

#[derive(Debug, Clone)]
pub enum Format {
	ABC(ABC),
	ABX(ABX),
	ASBX(ASBX),
	AX(AX),
	ISJ(ISJ),
}

impl Format {
	pub fn as_abc(self) -> Option<ABC> {
		match self {
			Self::ABC(abc) => Some(abc),
			_ => None,
		}
	}

	pub fn as_abx(self) -> Option<ABX> {
		match self {
			Self::ABX(abx) => Some(abx),
			_ => None,
		}
	}

	pub fn as_asbx(self) -> Option<ASBX> {
		match self {
			Self::ASBX(asbx) => Some(asbx),
			_ => None,
		}
	}

	pub fn as_ax(self) -> Option<AX> {
		match self {
			Self::AX(ax) => Some(ax),
			_ => None,
		}
	}

	pub fn as_isj(self) -> Option<ISJ> {
		match self {
			Self::ISJ(isj) => Some(isj),
			_ => None,
		}
	}
}

impl DecodableFormat for Format {
	fn decode_format(inst: crate::inst::Instruction) -> Option<Self> {
		match inst.opmode().format() {
			Some(FormatKind::ABC) => Some(Self::ABC(ABC::decode_format(inst)?)),
			Some(FormatKind::ABX) => Some(Self::ABX(ABX::decode_format(inst)?)),
			Some(FormatKind::ASBX) => Some(Self::ASBX(ASBX::decode_format(inst)?)),
			Some(FormatKind::AX) => Some(Self::AX(AX::decode_format(inst)?)),
			Some(FormatKind::ISJ) => Some(Self::ISJ(ISJ::decode_format(inst)?)),
			None => None,
		}
	}
}

pub type OpCodeSize = u8;

/// All the possible instructions for the Lua VM.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	Move(ABC) = cn::OP_MOVE,
	LoadI(ASBX) = cn::OP_LOADI,
	LoadF(ASBX) = cn::OP_LOADF,
	LoadK(ABX) = cn::OP_LOADK,
	LoadKX(ABX) = cn::OP_LOADKX,
	LoadFalse(ABC) = cn::OP_LOADFALSE,
	LFalseSkip(ABC) = cn::OP_LFALSESKIP,
	LoadTrue(ABC) = cn::OP_LOADTRUE,
	LoadNil(ABC) = cn::OP_LOADNIL,
	GetUpval(ABC) = cn::OP_GETUPVAL,
	SetUpval(ABC) = cn::OP_SETUPVAL,
	GetTabUp(ABC) = cn::OP_GETTABUP,
	GetTable(ABC) = cn::OP_GETTABLE,
	GetI(ABC) = cn::OP_GETI,
	GetField(ABC) = cn::OP_GETFIELD,
	SetTabUp(ABC) = cn::OP_SETTABUP,
	SetTable(ABC) = cn::OP_SETTABLE,
	SetI(ABC) = cn::OP_SETI,
	SetField(ABC) = cn::OP_SETFIELD,
	NewTable(ABC) = cn::OP_NEWTABLE,
	ISelf(ABC) = cn::OP_ISELF,
	AddI(ABC) = cn::OP_ADDI,
	AddK(ABC) = cn::OP_ADDK,
	SubK(ABC) = cn::OP_SUBK,
	MulK(ABC) = cn::OP_MULK,
	ModK(ABC) = cn::OP_MODK,
	PowK(ABC) = cn::OP_POWK,
	DivK(ABC) = cn::OP_DIVK,
	IDivK(ABC) = cn::OP_IDIVK,
	BAndK(ABC) = cn::OP_BANDK,
	BOrK(ABC) = cn::OP_BORK,
	BXorK(ABC) = cn::OP_BXORK,
	ShrI(ABC) = cn::OP_SHRI,
	ShlI(ABC) = cn::OP_SHLI,
	Add(ABC) = cn::OP_ADD,
	Sub(ABC) = cn::OP_SUB,
	Mul(ABC) = cn::OP_MUL,
	Mod(ABC) = cn::OP_MOD,
	Pow(ABC) = cn::OP_POW,
	Div(ABC) = cn::OP_DIV,
	IDiv(ABC) = cn::OP_IDIV,
	BAnd(ABC) = cn::OP_BAND,
	BOr(ABC) = cn::OP_BOR,
	BXor(ABC) = cn::OP_BXOR,
	Shl(ABC) = cn::OP_SHL,
	Shr(ABC) = cn::OP_SHR,
	MMBin(ABC) = cn::OP_MMBIN,
	MMBinI(ABC) = cn::OP_MMBINI,
	MMBinK(ABC) = cn::OP_MMBINK,
	UnM(ABC) = cn::OP_UNM,
	BNot(ABC) = cn::OP_BNOT,
	Not(ABC) = cn::OP_NOT,
	Len(ABC) = cn::OP_LEN,
	Concat(ABC) = cn::OP_CONCAT,
	Close(ABC) = cn::OP_CLOSE,
	Tbc(ABC) = cn::OP_TBC,
	Jmp(ISJ) = cn::OP_JMP,
	Eq(ABC) = cn::OP_EQ,
	Lt(ABC) = cn::OP_LT,
	Le(ABC) = cn::OP_LE,
	EqK(ABC) = cn::OP_EQK,
	EqI(ABC) = cn::OP_EQI,
	LtI(ABC) = cn::OP_LTI,
	LeI(ABC) = cn::OP_LEI,
	GtI(ABC) = cn::OP_GTI,
	GeI(ABC) = cn::OP_GEI,
	Test(ABC) = cn::OP_TEST,
	TestSet(ABC) = cn::OP_TESTSET,
	Call(ABC) = cn::OP_CALL,
	TailCall(ABC) = cn::OP_TAILCALL,
	Return(ABC) = cn::OP_RETURN,
	Return0(ABC) = cn::OP_RETURN0,
	Return1(ABC) = cn::OP_RETURN1,
	ForLoop(ABX) = cn::OP_FORLOOP,
	ForPrep(ABX) = cn::OP_FORPREP,
	TForPrep(ABX) = cn::OP_TFORPREP,
	TForCall(ABC) = cn::OP_TFORCALL,
	TForLoop(ABX) = cn::OP_TFORLOOP,
	SetList(ABC) = cn::OP_SETLIST,
	Closure(ABX) = cn::OP_CLOSURE,
	VarArg(ABC) = cn::OP_VARARG,
	VarArgPrep(ABC) = cn::OP_VARARGPREP,
	ExtraArg(AX) = cn::OP_EXTRAARG,
}

impl OpCode {
	pub fn name(&self) -> &'static str {
		match self {
			Self::Move(_) => "MOVE",
			Self::LoadI(_) => "LOADI",
			Self::LoadF(_) => "LOADF",
			Self::LoadK(_) => "LOADK",
			Self::LoadKX(_) => "LOADKX",
			Self::LoadFalse(_) => "LOADFALSE",
			Self::LFalseSkip(_) => "LFALSESKIP",
			Self::LoadTrue(_) => "LOADTRUE",
			Self::LoadNil(_) => "LOADNIL",
			Self::GetUpval(_) => "GETUPVAL",
			Self::SetUpval(_) => "SETUPVAL",
			Self::GetTabUp(_) => "GETTABUP",
			Self::GetTable(_) => "GETTABLE",
			Self::GetI(_) => "GETI",
			Self::GetField(_) => "GETFIELD",
			Self::SetTabUp(_) => "SETTABUP",
			Self::SetTable(_) => "SETTABLE",
			Self::SetI(_) => "SETI",
			Self::SetField(_) => "SETFIELD",
			Self::NewTable(_) => "NEWTABLE",
			Self::ISelf(_) => "SELF",
			Self::AddI(_) => "ADDI",
			Self::AddK(_) => "ADDK",
			Self::SubK(_) => "SUBK",
			Self::MulK(_) => "MULK",
			Self::ModK(_) => "MODK",
			Self::PowK(_) => "POWK",
			Self::DivK(_) => "DIVK",
			Self::IDivK(_) => "IDIVK",
			Self::BAndK(_) => "BANDK",
			Self::BOrK(_) => "BORK",
			Self::BXorK(_) => "BXORK",
			Self::ShrI(_) => "SHRI",
			Self::ShlI(_) => "SHLI",
			Self::Add(_) => "ADD",
			Self::Sub(_) => "SUB",
			Self::Mul(_) => "MUL",
			Self::Mod(_) => "MOD",
			Self::Pow(_) => "POW",
			Self::Div(_) => "DIV",
			Self::IDiv(_) => "IDIV",
			Self::BAnd(_) => "BAND",
			Self::BOr(_) => "BOR",
			Self::BXor(_) => "BXOR",
			Self::Shl(_) => "SHL",
			Self::Shr(_) => "SHR",
			Self::MMBin(_) => "MMBIN",
			Self::MMBinI(_) => "MMBINI",
			Self::MMBinK(_) => "MMBINK",
			Self::UnM(_) => "UNM",
			Self::BNot(_) => "BNOT",
			Self::Not(_) => "NOT",
			Self::Len(_) => "LEN",
			Self::Concat(_) => "CONCAT",
			Self::Close(_) => "CLOSE",
			Self::Tbc(_) => "TBC",
			Self::Jmp(_) => "JMP",
			Self::Eq(_) => "EQ",
			Self::Lt(_) => "LT",
			Self::Le(_) => "LE",
			Self::EqK(_) => "EQK",
			Self::EqI(_) => "EQI",
			Self::LtI(_) => "LTI",
			Self::LeI(_) => "LEI",
			Self::GtI(_) => "GTI",
			Self::GeI(_) => "GEI",
			Self::Test(_) => "TEST",
			Self::TestSet(_) => "TESTSET",
			Self::Call(_) => "CALL",
			Self::TailCall(_) => "TAILCALL",
			Self::Return(_) => "RETURN",
			Self::Return0(_) => "RETURN0",
			Self::Return1(_) => "RETURN1",
			Self::ForLoop(_) => "FORLOOP",
			Self::ForPrep(_) => "FORPREP",
			Self::TForPrep(_) => "TFORPREP",
			Self::TForCall(_) => "TFORCALL",
			Self::TForLoop(_) => "TFORLOOP",
			Self::SetList(_) => "SETLIST",
			Self::Closure(_) => "CLOSURE",
			Self::VarArg(_) => "VARARG",
			Self::VarArgPrep(_) => "VARARGPREP",
			Self::ExtraArg(_) => "EXTRAARG",
		}
	}

	pub const fn opmode(&self) -> OpMode {
		match self {
			Self::Move(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LoadI(_) => OpMode::new(false, false, false, false, true, FormatKind::ASBX),
			Self::LoadF(_) => OpMode::new(false, false, false, false, true, FormatKind::ASBX),
			Self::LoadK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::LoadKX(_) => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::LoadFalse(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LFalseSkip(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LoadTrue(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LoadNil(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetUpval(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::SetUpval(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::GetTabUp(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetTable(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetI(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetField(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::SetTabUp(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::SetTable(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::SetI(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::SetField(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::NewTable(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ISelf(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::AddI(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::AddK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::SubK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::MulK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ModK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::PowK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::DivK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::IDivK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BAndK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BOrK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BXorK(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ShrI(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ShlI(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Add(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Sub(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Mul(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Mod(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Pow(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Div(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::IDiv(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BAnd(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BOr(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BXor(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Shl(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Shr(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::MMBin(_) => OpMode::new(true, false, false, false, false, FormatKind::ABC),
			Self::MMBinI(_) => OpMode::new(true, false, false, false, false, FormatKind::ABC),
			Self::MMBinK(_) => OpMode::new(true, false, false, false, false, FormatKind::ABC),
			Self::UnM(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BNot(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Not(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Len(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Concat(_) => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Close(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::Tbc(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::Jmp(_) => OpMode::new(false, false, false, false, false, FormatKind::ISJ),
			Self::Eq(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::Lt(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::Le(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::EqK(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::EqI(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::LtI(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::LeI(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::GtI(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::GeI(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::Test(_) => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::TestSet(_) => OpMode::new(false, false, false, true, true, FormatKind::ABC),
			Self::Call(_) => OpMode::new(false, true, true, false, true, FormatKind::ABC),
			Self::TailCall(_) => OpMode::new(false, true, true, false, true, FormatKind::ABC),
			Self::Return(_) => OpMode::new(false, false, true, false, false, FormatKind::ABC),
			Self::Return0(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::Return1(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::ForLoop(_) => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::ForPrep(_) => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::TForPrep(_) => OpMode::new(false, false, false, false, false, FormatKind::ABX),
			Self::TForCall(_) => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::TForLoop(_) => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::SetList(_) => OpMode::new(false, false, true, false, false, FormatKind::ABC),
			Self::Closure(_) => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::VarArg(_) => OpMode::new(false, true, false, false, true, FormatKind::ABC),
			Self::VarArgPrep(_) => OpMode::new(false, false, true, false, true, FormatKind::ABC),
			Self::ExtraArg(_) => OpMode::new(false, false, false, false, false, FormatKind::AX),
		}
	}
}
