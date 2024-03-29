//! ## `OpCode` Implementation

use crate::{cn, formats::FormatKind};

use super::{OpCode, OpCodeId, OpMode};

impl OpCode {
	pub fn from_opcodeid(code: OpCodeId) -> Option<Self> {
		match code {
			cn::OP_MOVE => Some(Self::Move),
			cn::OP_LOADI => Some(Self::LoadI),
			cn::OP_LOADF => Some(Self::LoadF),
			cn::OP_LOADK => Some(Self::LoadK),
			cn::OP_LOADKX => Some(Self::LoadKX),
			cn::OP_LOADFALSE => Some(Self::LoadFalse),
			cn::OP_LFALSESKIP => Some(Self::LFalseSkip),
			cn::OP_LOADTRUE => Some(Self::LoadTrue),
			cn::OP_LOADNIL => Some(Self::LoadNil),
			cn::OP_GETUPVAL => Some(Self::GetUpval),
			cn::OP_SETUPVAL => Some(Self::SetUpval),
			cn::OP_GETTABUP => Some(Self::GetTabUp),
			cn::OP_GETTABLE => Some(Self::GetTable),
			cn::OP_GETI => Some(Self::GetI),
			cn::OP_GETFIELD => Some(Self::GetField),
			cn::OP_SETTABUP => Some(Self::SetTabUp),
			cn::OP_SETTABLE => Some(Self::SetTable),
			cn::OP_SETI => Some(Self::SetI),
			cn::OP_SETFIELD => Some(Self::SetField),
			cn::OP_NEWTABLE => Some(Self::NewTable),
			cn::OP_ISELF => Some(Self::ISelf),
			cn::OP_ADDI => Some(Self::AddI),
			cn::OP_ADDK => Some(Self::AddK),
			cn::OP_SUBK => Some(Self::SubK),
			cn::OP_MULK => Some(Self::MulK),
			cn::OP_MODK => Some(Self::ModK),
			cn::OP_POWK => Some(Self::PowK),
			cn::OP_DIVK => Some(Self::DivK),
			cn::OP_IDIVK => Some(Self::IDivK),
			cn::OP_BANDK => Some(Self::BAndK),
			cn::OP_BORK => Some(Self::BOrK),
			cn::OP_BXORK => Some(Self::BXorK),
			cn::OP_SHRI => Some(Self::ShrI),
			cn::OP_SHLI => Some(Self::ShlI),
			cn::OP_ADD => Some(Self::Add),
			cn::OP_SUB => Some(Self::Sub),
			cn::OP_MUL => Some(Self::Mul),
			cn::OP_MOD => Some(Self::Mod),
			cn::OP_POW => Some(Self::Pow),
			cn::OP_DIV => Some(Self::Div),
			cn::OP_IDIV => Some(Self::IDiv),
			cn::OP_BAND => Some(Self::BAnd),
			cn::OP_BOR => Some(Self::BOr),
			cn::OP_BXOR => Some(Self::BXor),
			cn::OP_SHL => Some(Self::Shl),
			cn::OP_SHR => Some(Self::Shr),
			cn::OP_MMBIN => Some(Self::MMBin),
			cn::OP_MMBINI => Some(Self::MMBinI),
			cn::OP_MMBINK => Some(Self::MMBinK),
			cn::OP_UNM => Some(Self::UnM),
			cn::OP_BNOT => Some(Self::BNot),
			cn::OP_NOT => Some(Self::Not),
			cn::OP_LEN => Some(Self::Len),
			cn::OP_CONCAT => Some(Self::Concat),
			cn::OP_CLOSE => Some(Self::Close),
			cn::OP_TBC => Some(Self::Tbc),
			cn::OP_JMP => Some(Self::Jmp),
			cn::OP_EQ => Some(Self::Eq),
			cn::OP_LT => Some(Self::Lt),
			cn::OP_LE => Some(Self::Le),
			cn::OP_EQK => Some(Self::EqK),
			cn::OP_EQI => Some(Self::EqI),
			cn::OP_LTI => Some(Self::LtI),
			cn::OP_LEI => Some(Self::LeI),
			cn::OP_GTI => Some(Self::GtI),
			cn::OP_GEI => Some(Self::GeI),
			cn::OP_TEST => Some(Self::Test),
			cn::OP_TESTSET => Some(Self::TestSet),
			cn::OP_CALL => Some(Self::Call),
			cn::OP_TAILCALL => Some(Self::TailCall),
			cn::OP_RETURN => Some(Self::Return),
			cn::OP_RETURN0 => Some(Self::Return0),
			cn::OP_RETURN1 => Some(Self::Return1),
			cn::OP_FORLOOP => Some(Self::ForLoop),
			cn::OP_FORPREP => Some(Self::ForPrep),
			cn::OP_TFORPREP => Some(Self::TForPrep),
			cn::OP_TFORCALL => Some(Self::TForCall),
			cn::OP_TFORLOOP => Some(Self::TForLoop),
			cn::OP_SETLIST => Some(Self::SetList),
			cn::OP_CLOSURE => Some(Self::Closure),
			cn::OP_VARARG => Some(Self::VarArg),
			cn::OP_VARARGPREP => Some(Self::VarArgPrep),
			cn::OP_EXTRAARG => Some(Self::ExtraArg),
			_ => None,
		}
	}

	pub fn name(self) -> &'static str {
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

	pub const fn mode(self) -> OpMode {
		match self {
			Self::Move => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LoadI => OpMode::new(false, false, false, false, true, FormatKind::ASBX),
			Self::LoadF => OpMode::new(false, false, false, false, true, FormatKind::ASBX),
			Self::LoadK => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::LoadKX => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::LoadFalse => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LFalseSkip => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LoadTrue => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::LoadNil => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetUpval => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::SetUpval => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::GetTabUp => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetTable => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetI => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::GetField => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::SetTabUp => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::SetTable => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::SetI => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::SetField => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::NewTable => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ISelf => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::AddI => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::AddK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::SubK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::MulK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ModK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::PowK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::DivK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::IDivK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BAndK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BOrK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BXorK => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ShrI => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::ShlI => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Add => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Sub => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Mul => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Mod => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Pow => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Div => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::IDiv => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BAnd => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BOr => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BXor => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Shl => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Shr => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::MMBin => OpMode::new(true, false, false, false, false, FormatKind::ABC),
			Self::MMBinI => OpMode::new(true, false, false, false, false, FormatKind::ABC),
			Self::MMBinK => OpMode::new(true, false, false, false, false, FormatKind::ABC),
			Self::UnM => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::BNot => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Not => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Len => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Concat => OpMode::new(false, false, false, false, true, FormatKind::ABC),
			Self::Close => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::Tbc => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::Jmp => OpMode::new(false, false, false, false, false, FormatKind::ISJ),
			Self::Eq => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::Lt => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::Le => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::EqK => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::EqI => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::LtI => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::LeI => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::GtI => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::GeI => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::Test => OpMode::new(false, false, false, true, false, FormatKind::ABC),
			Self::TestSet => OpMode::new(false, false, false, true, true, FormatKind::ABC),
			Self::Call => OpMode::new(false, true, true, false, true, FormatKind::ABC),
			Self::TailCall => OpMode::new(false, true, true, false, true, FormatKind::ABC),
			Self::Return => OpMode::new(false, false, true, false, false, FormatKind::ABC),
			Self::Return0 => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::Return1 => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::ForLoop => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::ForPrep => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::TForPrep => OpMode::new(false, false, false, false, false, FormatKind::ABX),
			Self::TForCall => OpMode::new(false, false, false, false, false, FormatKind::ABC),
			Self::TForLoop => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::SetList => OpMode::new(false, false, true, false, false, FormatKind::ABC),
			Self::Closure => OpMode::new(false, false, false, false, true, FormatKind::ABX),
			Self::VarArg => OpMode::new(false, true, false, false, true, FormatKind::ABC),
			Self::VarArgPrep => OpMode::new(false, false, true, false, true, FormatKind::ABC),
			Self::ExtraArg => OpMode::new(false, false, false, false, false, FormatKind::AX),
		}
	}
}
