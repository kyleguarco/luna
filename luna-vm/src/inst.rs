// From `lopcodes.h`:
/*===========================================================================
  We assume that instructions are unsigned 32-bit integers.
  All instructions have an opcode in the first 7 bits.
  Instructions can have the following formats:

		3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0
		1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
iABC          C(8)     |      B(8)     |k|     A(8)      |   Op(7)     |
iABx                Bx(17)               |     A(8)      |   Op(7)     |
iAsBx              sBx (signed)(17)      |     A(8)      |   Op(7)     |
iAx                           Ax(25)                     |   Op(7)     |
isJ                           sJ (signed)(25)            |   Op(7)     |

  A signed argument is represented in excess K: the represented value is
  the written unsigned value minus K, where K is half the maximum for the
  corresponding unsigned argument.
===========================================================================*/

use crate::{
	cn,
	formats::{DecodableFormat, InvalidFormatError},
	mask::Mask,
	ops::{code::{Format, OpCode, OpCodeSize}, mode::OpMode},
};

#[derive(Debug, Clone)]
pub enum InstructionDecodeError {
	InvalidFormat(Format),
	InvalidOpcode(Instruction),
}

pub type InstructionSize = u32;

#[derive(Debug, Clone, Copy)]
pub struct Instruction(pub(crate) InstructionSize);

impl Instruction {
	pub fn opcode(self) -> Result<OpCode, InstructionDecodeError> {
		let format = Format::decode_format(self).unwrap();

		fn op_ok<F: DecodableFormat, T: TryInto<F>>(fmt: Result<F, InvalidFormatError>, f: impl FnOnce(T) -> OpCode) -> Result<OpCode, InstructionDecodeError> {
			Ok(f(fmt.try_into().map_err(|_| InstructionDecodeError::InvalidFormat(fmt))?))
		}

		match (self.0 & InstructionSize::mask1(cn::SIZE_OP, 0)) as OpCodeSize {
			cn::OP_MOVE => Ok(OpCode::Move(format.try_into().map_err(|_| InstructionDecodeError::InvalidFormat(format))?)),
			cn::OP_LOADI => OpCode::LoadI(ASBX::default()),
			cn::OP_LOADF => OpCode::LoadF(ASBX::default()),
			cn::OP_LOADK => OpCode::LoadK(ABX::default()),
			cn::OP_LOADKX => OpCode::LoadKX(ABX::default()),
			cn::OP_LOADFALSE => OpCode::LoadFalse(ABC::default()),
			cn::OP_LFALSESKIP => OpCode::LFalseSkip(ABC::default()),
			cn::OP_LOADTRUE => OpCode::LoadTrue(ABC::default()),
			cn::OP_LOADNIL => OpCode::LoadNil(ABC::default()),
			cn::OP_GETUPVAL => OpCode::GetUpval(ABC::default()),
			cn::OP_SETUPVAL => OpCode::SetUpval(ABC::default()),
			cn::OP_GETTABUP => OpCode::GetTabUp(ABC::default()),
			cn::OP_GETTABLE => OpCode::GetTable(ABC::default()),
			cn::OP_GETI => OpCode::GetI(ABC::default()),
			cn::OP_GETFIELD => OpCode::GetField(ABC::default()),
			cn::OP_SETTABUP => OpCode::SetTabUp(ABC::default()),
			cn::OP_SETTABLE => OpCode::SetTable(ABC::default()),
			cn::OP_SETI => OpCode::SetI(ABC::default()),
			cn::OP_SETFIELD => OpCode::SetField(ABC::default()),
			cn::OP_NEWTABLE => OpCode::NewTable(ABC::default()),
			cn::OP_ISELF => OpCode::ISelf(ABC::default()),
			cn::OP_ADDI => OpCode::AddI(ABC::default()),
			cn::OP_ADDK => OpCode::AddK(ABC::default()),
			cn::OP_SUBK => OpCode::SubK(ABC::default()),
			cn::OP_MULK => OpCode::MulK(ABC::default()),
			cn::OP_MODK => OpCode::ModK(ABC::default()),
			cn::OP_POWK => OpCode::PowK(ABC::default()),
			cn::OP_DIVK => OpCode::DivK(ABC::default()),
			cn::OP_IDIVK => OpCode::IDivK(ABC::default()),
			cn::OP_BANDK => OpCode::BAndK(ABC::default()),
			cn::OP_BORK => OpCode::BOrK(ABC::default()),
			cn::OP_BXORK => OpCode::BXorK(ABC::default()),
			cn::OP_SHRI => OpCode::ShrI(ABC::default()),
			cn::OP_SHLI => OpCode::ShlI(ABC::default()),
			cn::OP_ADD => OpCode::Add(ABC::default()),
			cn::OP_SUB => OpCode::Sub(ABC::default()),
			cn::OP_MUL => OpCode::Mul(ABC::default()),
			cn::OP_MOD => OpCode::Mod(ABC::default()),
			cn::OP_POW => OpCode::Pow(ABC::default()),
			cn::OP_DIV => OpCode::Div(ABC::default()),
			cn::OP_IDIV => OpCode::IDiv(ABC::default()),
			cn::OP_BAND => OpCode::BAnd(ABC::default()),
			cn::OP_BOR => OpCode::BOr(ABC::default()),
			cn::OP_BXOR => OpCode::BXor(ABC::default()),
			cn::OP_SHL => OpCode::Shl(ABC::default()),
			cn::OP_SHR => OpCode::Shr(ABC::default()),
			cn::OP_MMBIN => OpCode::MMBin(ABC::default()),
			cn::OP_MMBINI => OpCode::MMBinI(ABC::default()),
			cn::OP_MMBINK => OpCode::MMBinK(ABC::default()),
			cn::OP_UNM => OpCode::UnM(ABC::default()),
			cn::OP_BNOT => OpCode::BNot(ABC::default()),
			cn::OP_NOT => OpCode::Not(ABC::default()),
			cn::OP_LEN => OpCode::Len(ABC::default()),
			cn::OP_CONCAT => OpCode::Concat(ABC::default()),
			cn::OP_CLOSE => OpCode::Close(ABC::default()),
			cn::OP_TBC => OpCode::Tbc(ABC::default()),
			cn::OP_JMP => OpCode::Jmp(ISJ::default()),
			cn::OP_EQ => OpCode::Eq(ABC::default()),
			cn::OP_LT => OpCode::Lt(ABC::default()),
			cn::OP_LE => OpCode::Le(ABC::default()),
			cn::OP_EQK => OpCode::EqK(ABC::default()),
			cn::OP_EQI => OpCode::EqI(ABC::default()),
			cn::OP_LTI => OpCode::LtI(ABC::default()),
			cn::OP_LEI => OpCode::LeI(ABC::default()),
			cn::OP_GTI => OpCode::GtI(ABC::default()),
			cn::OP_GEI => OpCode::GeI(ABC::default()),
			cn::OP_TEST => OpCode::Test(ABC::default()),
			cn::OP_TESTSET => OpCode::TestSet(ABC::default()),
			cn::OP_CALL => OpCode::Call(ABC::default()),
			cn::OP_TAILCALL => OpCode::TailCall(ABC::default()),
			cn::OP_RETURN => OpCode::Return(ABC::default()),
			cn::OP_RETURN0 => OpCode::Return0(ABC::default()),
			cn::OP_RETURN1 => OpCode::Return1(ABC::default()),
			cn::OP_FORLOOP => OpCode::ForLoop(ABX::default()),
			cn::OP_FORPREP => OpCode::ForPrep(ABX::default()),
			cn::OP_TFORPREP => OpCode::TForPrep(ABX::default()),
			cn::OP_TFORCALL => OpCode::TForCall(ABC::default()),
			cn::OP_TFORLOOP => OpCode::TForLoop(ABX::default()),
			cn::OP_SETLIST => OpCode::SetList(ABC::default()),
			cn::OP_CLOSURE => OpCode::Closure(ABX::default()),
			cn::OP_VARARG => OpCode::VarArg(ABC::default()),
			cn::OP_VARARGPREP => OpCode::VarArgPrep(ABC::default()),
			cn::OP_EXTRAARG => OpCode::ExtraArg(AX::default()),
			_ => Err(InstructionDecodeError::InvalidOpcode),
		}
	}

	pub fn opmode(self) -> OpMode {
		self.into()
	}
}

// There should only be a 1-way conversion, since we can no longer
// guarantee the validity of this integer.
impl Into<u32> for Instruction {
	fn into(self) -> u32 {
		self.0
	}
}
