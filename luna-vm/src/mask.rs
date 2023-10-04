//! Implements bitmask macros found in `lopcodes.h`

/// Trait that assists with creating bitmasks.
pub trait Mask: Sized + std::ops::Not<Output = Self> {
	/// Creates a mask with `amount` 1 bits at `position`.
	fn mask1(amount: u8, position: u8) -> Self;

	/// The inverse of [Mask::mask1]
	///
	/// Creates a mask with `amount` 0 bits at `position`.
	fn mask0(amount: u8, position: u8) -> Self {
		!Self::mask1(amount, position)
	}
}

macro_rules! impl_mask {
	($t:ty) => {
		impl Mask for $t {
			fn mask1(amount: u8, position: u8) -> Self {
				// Basically copied from `lopcodes.h`
				(!((!(0 as $t)) << (amount))) << (position)
			}
		}
	};

	($($t:ty),+) => {
		$(
			impl_mask! { $t }
		)*
	};
}

impl_mask! { u64, i64, u32, i32, u16, i16, u8, i8 }
