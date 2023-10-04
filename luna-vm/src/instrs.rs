// TODO: Due to issue #20400 <https://github.com/rust-lang/rust/issues/20400>,
// implementations like the following will result in a compiler error:
//
// trait Instruction: Props {
//	/// Converts the fields from [Props::Format] into a 32-bit VM instruction.
//	///
//	/// Faillable. `serialize` will fail if any fields are bigger than the instruction set specify.
//	fn serialize(self) -> Option<InstructionSize>;
//	fn deserialize(inst: InstructionSize) -> Self::Format;
// }
//
// impl<P> Instruction for P where P: Prop<Format = ABC> {}
//
// // error! conflicting implementation
// impl<P> Instruction for P where P: Prop<Format = ASBX> {}

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
	kind::{OpCode, OpCodeSize},
	mask::Mask,
};

pub type InstructionSize = u32;

#[derive(Debug, Clone, Copy)]
pub struct Instruction(pub(crate) InstructionSize);

impl Instruction {
	pub fn to_opcode(&self) -> OpCode {
		match (self.0 & InstructionSize::mask1(7, 0)) as OpCodeSize {
			0 => OpCode::Move,
			1 => OpCode::LoadI,
			2 => OpCode::LoadF,
			3 => OpCode::LoadK,
			4 => OpCode::LoadKX,
			5 => OpCode::LoadFalse,
			6 => OpCode::LFalseSkip,
			7 => OpCode::LoadTrue,
			8 => OpCode::LoadNil,
			9 => OpCode::GetUpval,
			10 => OpCode::SetUpval,
			11 => OpCode::GetTabUp,
			12 => OpCode::GetTable,
			13 => OpCode::GetI,
			14 => OpCode::GetField,
			15 => OpCode::SetTabUp,
			16 => OpCode::SetTable,
			17 => OpCode::SetI,
			18 => OpCode::SetField,
			19 => OpCode::NewTable,
			20 => OpCode::ISelf,
			21 => OpCode::AddI,
			22 => OpCode::AddK,
			23 => OpCode::SubK,
			24 => OpCode::MulK,
			25 => OpCode::ModK,
			26 => OpCode::PowK,
			27 => OpCode::DivK,
			28 => OpCode::IDivK,
			29 => OpCode::BAndK,
			30 => OpCode::BOrK,
			31 => OpCode::BXorK,
			32 => OpCode::ShrI,
			33 => OpCode::ShlI,
			34 => OpCode::Add,
			35 => OpCode::Sub,
			36 => OpCode::Mul,
			37 => OpCode::Mod,
			38 => OpCode::Pow,
			39 => OpCode::Div,
			40 => OpCode::IDiv,
			41 => OpCode::BAnd,
			42 => OpCode::BOr,
			43 => OpCode::BXor,
			44 => OpCode::Shl,
			45 => OpCode::Shr,
			46 => OpCode::MMBin,
			47 => OpCode::MMBinI,
			48 => OpCode::MMBinK,
			49 => OpCode::UnM,
			50 => OpCode::BNot,
			51 => OpCode::Not,
			52 => OpCode::Len,
			53 => OpCode::Concat,
			54 => OpCode::Close,
			55 => OpCode::Tbc,
			56 => OpCode::Jmp,
			57 => OpCode::Eq,
			58 => OpCode::Lt,
			59 => OpCode::Le,
			60 => OpCode::EqK,
			61 => OpCode::EqI,
			62 => OpCode::LtI,
			63 => OpCode::LeI,
			64 => OpCode::GtI,
			65 => OpCode::GeI,
			66 => OpCode::Test,
			67 => OpCode::TestSet,
			68 => OpCode::Call,
			69 => OpCode::TailCall,
			70 => OpCode::Return,
			71 => OpCode::Return0,
			72 => OpCode::Return1,
			73 => OpCode::ForLoop,
			74 => OpCode::ForPrep,
			75 => OpCode::TForPrep,
			76 => OpCode::TForCall,
			77 => OpCode::TForLoop,
			78 => OpCode::SetList,
			79 => OpCode::Closure,
			80 => OpCode::VarArg,
			81 => OpCode::VarArgPrep,
			82 => OpCode::ExtraArg,
			_ => unreachable!("There should only be 82 opcode variants."),
		}
	}
}
