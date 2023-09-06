pub type Float = ordered_float::OrderedFloat<f64>;
pub type Integer = isize;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Number {
	Float(Float),
	Integer(Integer),
}

impl From<f64> for Number {
	fn from(value: f64) -> Self {
		Self::Float(Float::from(value))
	}
}

impl From<Integer> for Number {
	fn from(value: Integer) -> Self {
		Self::Integer(value)
	}
}

impl Default for Number {
	fn default() -> Self {
		Self::Integer(0)
	}
}

impl<T: Into<Number> + Default> crate::Value for T {
	fn kind(&self) -> crate::Kind {
		crate::Kind::Number
	}
}

macro_rules! impl_op_num {
	($op:tt, $mname:tt, $mnamesec:tt, $treg:tt, $tas:tt) => {
		impl std::ops::$treg for Number {
			type Output = Self;

			fn $mname(self, rhs: Self) -> Self::Output {
				match (self, rhs) {
					(Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs $op rhs),
					(Self::Float(lhs), Self::Integer(rhs)) => Self::Float(lhs $op Float::from(rhs as f64)),
					(Self::Integer(lhs), Self::Float(rhs)) => Self::Float(Float::from(lhs as f64) $op rhs),
					(Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs $op rhs),
				}
			}
		}

		impl std::ops::$tas for Number {
			fn $mnamesec(&mut self, rhs: Self) {
				match (&self, rhs) {
					(Self::Float(lhs), Self::Float(rhs)) => *self = Self::Float(lhs $op rhs),
					(Self::Float(lhs), Self::Integer(rhs)) => {
						*self = Self::Float(lhs $op Float::from(rhs as f64))
					}
					(Self::Integer(lhs), Self::Float(rhs)) => {
						*self = Self::Float(Float::from(*lhs as f64) $op rhs)
					}
					(Self::Integer(lhs), Self::Integer(rhs)) => *self = Self::Integer(lhs $op rhs),
				}
			}
		}
	};

	($(($op:tt, $mname:tt, $mnamesec:tt, $treg:tt, $tas:tt)),*) => {
		$(
			impl_op_num! { $op, $mname, $mnamesec, $treg, $tas }
		)*
	}
}

impl_op_num! {
	(+, add, add_assign, Add, AddAssign),
	(-, sub, sub_assign, Sub, SubAssign),
	(*, mul, mul_assign, Mul, MulAssign),
	(/, div, div_assign, Div, DivAssign)
}
