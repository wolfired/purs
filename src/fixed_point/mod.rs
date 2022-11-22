//! simple fixed point
//!
//! integer bits: m     
//! fractional bits: n     
//! floating point: a, b    
//! fixed point: A, B    
//!
//! $$ A = a \cdot 2^n $$
//! $$ B = b \cdot 2^n $$
//! $$ (a + b) \cdot 2^n = a \cdot 2^n + b \cdot 2^n = A + B $$
//! $$ (a - b) \cdot 2^n = a \cdot 2^n - b \cdot 2^n = A - B $$
//! $$ (a \cdot b) \cdot 2^n = \frac {a \cdot 2^n \cdot b \cdot 2^n} {2^n} = \frac {A \cdot B} {2^n} $$
//! $$ \frac {a} {b} \cdot 2^n = \frac {a \cdot 2^n} {b \cdot 2^n} \cdot 2^n = \frac {A} {B} \cdot 2^n $$
//! $$ (a \verb|<<| b) \cdot 2^n = a \cdot 2^b \cdot 2^n = a \cdot 2^n \cdot 2^{\frac {b \cdot 2^n} {2^n}} = A \cdot 2^{\frac B {2^n}} = A \verb|<<| (B \verb|>>| n) $$
//! $$ (a \verb|>>| b) \cdot 2^n = \frac a {2^b} \cdot 2^n = \frac {a \cdot 2^n} {2^{\frac {b \cdot 2^n} {2^n}}} = \frac A {2^{\frac B {2^n}}} = A \verb|>>| (B \verb|>>| n) $$
//!

pub mod fp;

use crate::number::Integer;
use crate::number::Number;
use crate::number::One;
use crate::number::Zero;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Clone, Copy, Debug, Default)]
pub struct FixedPoint<T: Integer, const P: usize> {
    pub value: T,
}

macro_rules! impl_fixedpoint {
    ($($t:ty),*) => {
        $(
            impl<const P: usize> FixedPoint<$t, P> {
                pub const BITS:u32 = <$t>::BITS;
                pub const FRAC:u32 = P as u32;

                pub fn sqrt(self) -> FixedPoint<$t, P> {
                    let mut s = self.value << P;
                    let mut n = ((<$t>::BITS - s.leading_zeros() + 1) >> 1) as $t;

                    let mut r = 0;

                    while 0 < s && 0 < n {
                        n -= 1;
                        let r2 = ((r << 1) + (1 << n)) << n;
                        if r2 <= s {
                            r += 1 << n;
                            s -= r2;
                        }
                    }

                    Self {
                        value: r
                    }
                }
            }
        )*
    }
}
impl_fixedpoint!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

macro_rules! impl_from_float {
    ($t0:ty,$($t1:ty),*) => {
        $(
            impl<const P: usize> From<$t0> for FixedPoint<$t1, P> {
                fn from(ori: $t0) -> Self {
                    Self {
                        value: (ori * (1 << P) as $t0).round() as $t1,
                    }
                }
            }

            impl<const P: usize> From<FixedPoint<$t1, P>> for $t0 {
                fn from(ori: FixedPoint<$t1, P>) -> Self {
                    ori.value as $t0 / (1 << P) as $t0
                }
            }
        )*
    }
}
impl_from_float!(f32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
impl_from_float!(f64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

impl<T: Integer, const P: usize> From<T> for FixedPoint<T, P> {
    fn from(value: T) -> Self {
        Self {
            value: value << T::try_from(P).ok().expect("msg"),
        }
    }
}

impl<T: Integer, const P: usize> Add for FixedPoint<T, P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<T: Integer, const P: usize> AddAssign for FixedPoint<T, P> {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl<T: Integer, const P: usize> Sub for FixedPoint<T, P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}

impl<T: Integer, const P: usize> SubAssign for FixedPoint<T, P> {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl<T: Integer, const P: usize> Mul for FixedPoint<T, P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value * rhs.value) >> T::try_from(P).ok().expect("msg"),
        }
    }
}

impl<T: Integer, const P: usize> MulAssign for FixedPoint<T, P> {
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
        self.value >>= T::try_from(P).ok().expect("msg");
    }
}

impl<T: Integer, const P: usize> Div for FixedPoint<T, P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value << T::try_from(P).ok().expect("msg")) / rhs.value,
        }
    }
}

impl<T: Integer, const P: usize> DivAssign for FixedPoint<T, P> {
    fn div_assign(&mut self, other: Self) {
        self.value <<= T::try_from(P).ok().expect("msg");
        self.value /= other.value;
    }
}

impl<T: Integer, const P: usize> Shl for FixedPoint<T, P> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value << (rhs.value >> T::try_from(P).ok().expect("msg")),
        }
    }
}

impl<T: Integer, const P: usize> ShlAssign for FixedPoint<T, P> {
    fn shl_assign(&mut self, other: Self) {
        self.value <<= other.value >> T::try_from(P).ok().expect("msg");
    }
}

impl<T: Integer, const P: usize> Shr for FixedPoint<T, P> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value >> (rhs.value >> T::try_from(P).ok().expect("msg")),
        }
    }
}

impl<T: Integer, const P: usize> ShrAssign for FixedPoint<T, P> {
    fn shr_assign(&mut self, other: Self) {
        self.value >>= other.value >> T::try_from(P).ok().expect("msg");
    }
}

impl<T: Integer, const P: usize> Rem for FixedPoint<T, P> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value % rhs.value,
        }
    }
}

impl<T: Integer, const P: usize> RemAssign for FixedPoint<T, P> {
    fn rem_assign(&mut self, other: Self) {
        self.value %= other.value;
    }
}

impl<T: Integer, const P: usize> Zero for FixedPoint<T, P> {}

impl<T: Integer, const P: usize> One for FixedPoint<T, P> {
    fn one() -> Self {
        Self { value: T::one() }
    }
}

impl<T: Integer, const P: usize> Number for FixedPoint<T, P> {}

impl<T: Integer, const P: usize> Integer for FixedPoint<T, P> where FixedPoint<T, P>: From<usize> {}
