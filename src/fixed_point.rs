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

use std::{
    fmt::Debug,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct FixedPoint<T, const P: usize> {
    pub value: T,
}

macro_rules! impl_from_block {
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
impl_from_block!(f32, u8, u16, u32, u64, i8, i16, i32, i64);
impl_from_block!(f64, u8, u16, u32, u64, i8, i16, i32, i64);

macro_rules! impl_block {
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

                pub fn mdlo(self, rhs:Self) -> FixedPoint<$t, P> {
                    Self {
                        value: self.value % rhs.value
                    }
                }
            }
        )*
    }
}
impl_block!(u8, u16, u32, u64, i8, i16, i32, i64);

macro_rules! impl_ops_block {
    ($($t:ty),*) => {
        $(
            impl<const P: usize> Add for FixedPoint<$t, P> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: self.value + rhs.value,
                    }
                }
            }

            impl<const P: usize> AddAssign for FixedPoint<$t, P> {
                fn add_assign(&mut self, other: Self) {
                    self.value += other.value;
                }
            }

            impl<const P: usize> Sub for FixedPoint<$t, P> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: self.value - rhs.value,
                    }
                }
            }

            impl<const P: usize> SubAssign for FixedPoint<$t, P> {
                fn sub_assign(&mut self, other: Self) {
                    self.value -= other.value;
                }
            }

            impl<const P: usize> Mul for FixedPoint<$t, P> {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: (self.value * rhs.value) >> P,
                    }
                }
            }

            impl<const P: usize> MulAssign for FixedPoint<$t, P> {
                fn mul_assign(&mut self, other: Self) {
                    self.value *= other.value;
                    self.value >>= P;
                }
            }

            impl<const P: usize> Div for FixedPoint<$t, P> {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: (self.value << P) / rhs.value,
                    }
                }
            }

            impl<const P: usize> DivAssign for FixedPoint<$t, P> {
                fn div_assign(&mut self, other: Self) {
                    self.value <<= P;
                    self.value /= other.value;
                }
            }

            impl<const P: usize> Shl for FixedPoint<$t, P> {
                type Output = Self;

                fn shl(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: self.value << (rhs.value >> P)
                    }
                }
            }

            impl<const P: usize> ShlAssign for FixedPoint<$t, P> {
                fn shl_assign(&mut self, other: Self) {
                    self.value <<= (other.value >> P);
                }
            }

            impl<const P: usize> Shr for FixedPoint<$t, P> {
                type Output = Self;

                fn shr(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: self.value >> (rhs.value >> P)
                    }
                }
            }

            impl<const P: usize> ShrAssign for FixedPoint<$t, P> {
                fn shr_assign(&mut self, other: Self) {
                    self.value >>= (other.value >> P);
                }
            }
        )*
    }
}
impl_ops_block!(u8, u16, u32, u64, i8, i16, i32, i64);
