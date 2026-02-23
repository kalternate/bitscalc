use crate::Result;

use std::fmt::{Binary, Display, LowerHex};

use num_traits::{PrimInt, WrappingAdd, WrappingSub};
use serde::Serialize;

pub trait Value: PrimInt + Display + LowerHex + Binary {
    fn negative(self) -> Self;
    fn logical_not(self) -> Self;
    fn bitwise_not(self) -> Self;
    fn multiplication(self, other: Self) -> Result<Self>;
    fn division(self, other: Self) -> Result<Self>;
    fn remainder(self, other: Self) -> Result<Self>;
    fn addition(self, other: Self) -> Result<Self>;
    fn subtraction(self, other: Self) -> Result<Self>;
    fn left_bitshift(self, other: Self) -> Result<Self>;
    fn right_bitshift(self, other: Self) -> Result<Self>;
    fn less_than(self, other: Self) -> Result<Self>;
    fn greater_than(self, other: Self) -> Result<Self>;
    fn less_than_or_equal(self, other: Self) -> Result<Self>;
    fn greater_than_or_equal(self, other: Self) -> Result<Self>;
    fn equals(self, other: Self) -> Result<Self>;
    fn not_equals(self, other: Self) -> Result<Self>;
    fn bitwise_and(self, other: Self) -> Result<Self>;
    fn bitwise_xor(self, other: Self) -> Result<Self>;
    fn bitwise_or(self, other: Self) -> Result<Self>;
    fn logical_and(self, other: Self) -> Result<Self>;
    fn logical_xor(self, other: Self) -> Result<Self>;
    fn logical_or(self, other: Self) -> Result<Self>;
}

impl<V: PrimInt + WrappingAdd + WrappingSub + Display + LowerHex + Binary> Value for V {
    fn negative(self) -> Self {
        (!self).wrapping_add(&V::one())
    }

    fn logical_not(self) -> Self {
        if self.is_zero() {
            V::one()
        } else {
            V::zero()
        }
    }

    fn bitwise_not(self) -> Self {
        !self
    }

    fn multiplication(self, other: Self) -> Result<Self> {
        Ok(self * other)
    }

    fn division(self, other: Self) -> Result<Self> {
        if other.is_zero() {
            Err("Cannot divide by 0.".into())
        } else {
            Ok(self / other)
        }
    }

    fn remainder(self, other: Self) -> Result<Self> {
        if other.is_zero() {
            Err("Cannot divide by 0.".into())
        } else {
            Ok(self.rem(other))
        }
    }

    fn addition(self, other: Self) -> Result<Self> {
        Ok(self + other)
    }

    fn subtraction(self, other: Self) -> Result<Self> {
        Ok(self - other)
    }

    fn left_bitshift(self, other: Self) -> Result<Self> {
        let shift = other.to_usize().unwrap_or(0);
        Ok(self.shl(shift))
    }

    fn right_bitshift(self, other: Self) -> Result<Self> {
        let shift = other.to_usize().unwrap_or(0);
        Ok(self.shr(shift))
    }

    fn less_than(self, other: Self) -> Result<Self> {
        Ok(if self < other {
            V::one()
        } else {
            V::zero()
        })
    }

    fn greater_than(self, other: Self) -> Result<Self> {
        Ok(if self > other {
            V::one()
        } else {
            V::zero()
        })
    }

    fn less_than_or_equal(self, other: Self) -> Result<Self> {
        Ok(if self <= other {
            V::one()
        } else {
            V::zero()
        })
    }

    fn greater_than_or_equal(self, other: Self) -> Result<Self> {
        Ok(if self >= other {
            V::one()
        } else {
            V::zero()
        })
    }

    fn equals(self, other: Self) -> Result<Self> {
        Ok(if self == other {
            V::one()
        } else {
            V::zero()
        })
    }

    fn not_equals(self, other: Self) -> Result<Self> {
        Ok(if self != other {
            V::one()
        } else {
            V::zero()
        })
    }

    fn bitwise_and(self, other: Self) -> Result<Self> {
        Ok(self & other)
    }

    fn bitwise_xor(self, other: Self) -> Result<Self> {
        Ok(self ^ other)
    }

    fn bitwise_or(self, other: Self) -> Result<Self> {
        Ok(self | other)
    }

    fn logical_and(self, other: Self) -> Result<Self> {
        Ok(if self.is_zero() || other.is_zero() {
            V::zero()
        } else {
            V::one()
        })
    }

    fn logical_xor(self, other: Self) -> Result<Self> {
        Ok(if self.is_zero() && other.is_zero() {
            V::zero()
        } else if !self.is_zero() && !other.is_zero() {
            V::zero()
        } else {
            V::one()
        })
    }

    fn logical_or(self, other: Self) -> Result<Self> {
        Ok(if self.is_zero() && other.is_zero() {
            V::zero()
        } else {
            V::one()
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FormattedValue {
    pub dec: String,
    pub hex: String,
    pub bin: String,
}

impl FormattedValue {
    pub fn from_i64(value: i64) -> Self {
        FormattedValue {
            dec: format!("{}", value),
            hex: format!("{:#x}", value),
            bin: make_bin_format(value),
        }
    }

    pub fn from_value<V: Value>(value: V) -> Self {
        FormattedValue {
            dec: format!("{}", value),
            hex: format!("{:#x}", value),
            bin: make_bin_format(value),
        }
    }
}

fn make_bin_format<V: Value>(value: V) -> String {
    let unspaced: Vec<char> = format!("{:b}", value).chars().collect();
    let mut spaced = String::new();

    if unspaced.len() % 4 != 0 {
        for _ in 0..4 - (unspaced.len() % 4) {
            spaced.push('0')
        }
    }

    for i in 0..unspaced.len() {
        let l = unspaced.len() - i;
        if l % 4 == 0 {
            spaced.push(' ')
        }

        spaced.push(unspaced[i]);
    }

    spaced
}
