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
use std::convert::TryFrom;

pub trait Zero: Default {
    fn zero() -> Self {
        Self::default()
    }
}

macro_rules! impl_zero {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {}
        )*
    }
}
impl_zero!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($($t:ty),*) => {
        $(
            impl One for $t {
                fn one() -> Self { 1 as $t }
            }
        )*
    }
}
impl_one!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

pub trait Number:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Copy
    + Debug
    + One
    + Zero
{
}

macro_rules! impl_number {
    ($($t:ty),*) => {
        $(
            impl Number for $t {}
        )*
    }
}
impl_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

pub trait Integer:
    Number + Shl<Output = Self> + ShlAssign + Shr<Output = Self> + ShrAssign + TryFrom<usize>
{
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t {}
        )*
    }
}
impl_integer!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
