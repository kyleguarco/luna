//! # Virtual Machine Constants
//!
//! These constants are defined here as they are in the Lua source code.
//! See `lopcodes.h` and `lopcodes.c` for more.

use crate::ops::OpCodeId;

/// Macro for creating a group of constants (constants of the same type).
/// Used to save poor developers some typing and copy-pasting.
macro_rules! group_const {
	($t:ty, { $($name:ident = $val:literal),*$(,)? }) => {
		$(pub const $name: $t = $val;)*
	};
}

// Instruction opcode constants
group_const! { OpCodeId, {
	OP_MOVE = 0, OP_LOADI = 1, OP_LOADF = 2, OP_LOADK = 3, OP_LOADKX = 4, OP_LOADFALSE = 5,
	OP_LFALSESKIP = 6, OP_LOADTRUE = 7, OP_LOADNIL = 8, OP_GETUPVAL = 9, OP_SETUPVAL = 10,
	OP_GETTABUP = 11, OP_GETTABLE = 12, OP_GETI = 13, OP_GETFIELD = 14, OP_SETTABUP = 15,
	OP_SETTABLE = 16, OP_SETI = 17, OP_SETFIELD = 18, OP_NEWTABLE = 19, OP_ISELF = 20,
	OP_ADDI = 21, OP_ADDK = 22, OP_SUBK = 23, OP_MULK = 24, OP_MODK = 25,
	OP_POWK = 26, OP_DIVK = 27, OP_IDIVK = 28, OP_BANDK = 29, OP_BORK = 30,
	OP_BXORK = 31, OP_SHRI = 32, OP_SHLI = 33, OP_ADD = 34, OP_SUB = 35,
	OP_MUL = 36, OP_MOD = 37, OP_POW = 38, OP_DIV = 39, OP_IDIV = 40,
	OP_BAND = 41, OP_BOR = 42, OP_BXOR = 43, OP_SHL = 44, OP_SHR = 45,
	OP_MMBIN = 46, OP_MMBINI = 47, OP_MMBINK = 48, OP_UNM = 49, OP_BNOT = 50,
	OP_NOT = 51, OP_LEN = 52, OP_CONCAT = 53, OP_CLOSE = 54, OP_TBC = 55,
	OP_JMP = 56, OP_EQ = 57, OP_LT = 58, OP_LE = 59, OP_EQK = 60,
	OP_EQI = 61, OP_LTI = 62, OP_LEI = 63, OP_GTI = 64, OP_GEI = 65,
	OP_TEST = 66, OP_TESTSET = 67, OP_CALL = 68, OP_TAILCALL = 69, OP_RETURN = 70,
	OP_RETURN0 = 71, OP_RETURN1 = 72, OP_FORLOOP = 73, OP_FORPREP = 74, OP_TFORPREP = 75,
	OP_TFORCALL = 76, OP_TFORLOOP = 77, OP_SETLIST = 78, OP_CLOSURE = 79, OP_VARARG = 80,
	OP_VARARGPREP = 81, OP_EXTRAARG = 82,
}}

// Instruction format identifiers
group_const! { u8, { FMT_ABC = 0, FMT_ABX = 1, FMT_ASBX = 2, FMT_AX = 3, FMT_ISJ = 4, }}

// Instruction format section sizes
group_const! { u8, { SIZE_A = 8, SIZE_B = 8, SIZE_C = 8, SIZE_OP = 7 }}
pub const SIZE_BX: u8 = SIZE_B + SIZE_C + 1;
pub const SIZE_AX: u8 = SIZE_A + SIZE_BX;
pub const SIZE_SJ: u8 = SIZE_A + SIZE_BX;

// Instruction section positions
pub const POS_OP: u8 = 0;
pub const POS_A: u8 = POS_OP + SIZE_OP;
pub const POS_K: u8 = POS_A + SIZE_A;
pub const POS_B: u8 = POS_K + 1;
pub const POS_C: u8 = POS_B + SIZE_B;
pub const POS_BX: u8 = POS_K;
pub const POS_AX: u8 = POS_A;
pub const POS_SJ: u8 = POS_A;
