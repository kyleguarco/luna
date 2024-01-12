use crate::{
	mask::Mask,
	ops::{OpCode, OpCodeId},
};

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

pub struct Instruction(u32);

pub fn decode(raw: u32) -> Option<Instruction> {
	// This formula is also described in `lvm.c`
	// The opcode is basically the first seven bits.
	let code = raw & u32::mask1(7, 0);
	let code: OpCodeId = code
		.try_into()
		.expect("Couldn't decode instruction: u32 > u8!");
	let code = OpCode::from_opcodeid(code).expect("Bad OpCode.");

	println!("{code:?}");

	Some(Instruction(raw))
}

pub struct VM {
	reg: [usize; 64]
}

impl VM {
	pub fn new() -> Self {
		Self { reg: [0; 64] }
	}

	pub fn r#loop(&mut self, inst: Instruction) {

	}
}
