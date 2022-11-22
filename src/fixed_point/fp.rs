//! simple fixed point
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 

use std::convert::TryFrom;
use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Add,
};

#[derive(Clone, Copy, Default)]
pub struct FixedPoint<T, const P: usize> {
    pub value: T,
}

macro_rules! impl_sqrt {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> FixedPoint<$t0, P> {
                pub fn sqrt(self) -> FixedPoint<$t0, P> {
                    let mut value = (self.value as $t1) << P;
            
                    let mut result = 0;
            
                    let mut halt_bits = (<$t1>::BITS - value.leading_zeros() + 1) >> 1;
            
                    while 0 < value && 0 < halt_bits {
                        halt_bits -= 1;
                        let middle = ((result << 1) + (1 << halt_bits)) << halt_bits;
                        if middle <= value {
                            result += 1 << halt_bits;
                            value -= middle;
                        }
                    }
            
                    Self { value: result as $t0 }
                }
            }
        )*
    }
}

impl_sqrt!(u8, u16);
impl_sqrt!(u16, u32);
impl_sqrt!(u32, u64);
impl_sqrt!(u64, u128);

impl_sqrt!(i8, i16);
impl_sqrt!(i16, i32);
impl_sqrt!(i32, i64);
impl_sqrt!(i64, i128);

macro_rules! impl_debug {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> Debug for FixedPoint<$t1, P> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "FixedPoint<{}, {}>{{ {:+0iwidth$.pwidth$}, {:+0fwidth$}, {:#0bits$b} }}",
                        stringify!($t1),
                        P,
                        Into::<$t0>::into(*self),
                        self.value,
                        self.value,
                        iwidth = (<$t1>::MAX / 4).to_string().len() + P + 2,
                        pwidth = P,
                        fwidth = <$t1>::MAX.to_string().len() + 1,
                        bits = { <$t1>::BITS as usize + 2 },
                    )
                }
            }
        )*
    }
}
impl_debug!(f64, u8, i8, u16, i16, u32, i32);

macro_rules! impl_display {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> Display for FixedPoint<$t1, P> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", Into::<$t0>::into(*self))
                }
            }
        )*
    }
}
impl_display!(f64, u8, i8, u16, i16, u32, i32);

macro_rules! impl_ux_tryfrom {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> TryFrom<$t1> for FixedPoint<$t0, P> {
                type Error = Box<dyn Error>;

                fn try_from(value: $t1) -> Result<Self, Self::Error> {
                    if 0 == value {
                        return Ok(Self { value: 0 });
                    }

                    let value: $t0 = value.try_into()?;

                    let zeros = value.leading_zeros();
                    let ones = value.leading_ones();

                    let shl_bits: u32 = P.try_into()?;

                    if if 0 == ones { zeros } else { 0 } < shl_bits {
                        return Err("impl_ux_tryfrom: not enough bits".into());
                    }

                    Ok(Self { value: value << shl_bits })
                }
            }
        )*
    }
}
impl_ux_tryfrom!(u8, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ux_tryfrom!(u16, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ux_tryfrom!(u32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ux_tryfrom!(u64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ux_tryfrom!(u128, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ux_tryfrom!(usize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! impl_ix_tryfrom {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> TryFrom<$t1> for FixedPoint<$t0, P> {
                type Error = Box<dyn Error>;

                fn try_from(value: $t1) -> Result<Self, Self::Error> {
                    let value: $t0 = value.try_into()?;

                    let zeros = value.leading_zeros();
                    let ones = value.leading_ones();

                    let shl_bits: u32 = P.try_into()?;

                    if if 0 == ones { zeros - 1 } else { ones - 1 } < shl_bits {
                        return Err("impl_ix_tryfrom: not enough bits".into());
                    }

                    Ok(Self { value: value << shl_bits })
                }
            }
        )*
    }
}
impl_ix_tryfrom!(i8, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ix_tryfrom!(i16, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ix_tryfrom!(i32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ix_tryfrom!(i64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ix_tryfrom!(i128, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_ix_tryfrom!(isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! impl_tryfrom_x {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> TryFrom<FixedPoint<$t0, P>> for $t1 {
                type Error = Box<dyn Error>;

                fn try_from(value: FixedPoint<$t0, P>) -> Result<Self, Self::Error> {
                    let shl_bits: u32 = P.try_into()?;

                    if <$t0>::BITS <= shl_bits {
                        return Ok(0);
                    }

                    let add_zero_point_five = if 0 == P { 0 } else { 1 << (P - 1) };
                    let sub_one = if 0 == P { 0 } else { if 0 == ((value.value >> 1) & (1 << (<$t0>::BITS - 1))) { 0 } else { 1 } };

                    if let Some(value) = value.value.checked_add(add_zero_point_five) {
                        Ok(((value - sub_one) >> P).try_into()?)
                    } else {
                        Err("overflow".into())
                    }
                }
            }
        )*
    }
}
impl_tryfrom_x!(u8, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(u16, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(u32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(u64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(u128, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(usize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(i8, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(i16, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(i32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(i64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(i128, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_tryfrom_x!(isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! impl_from_f {
    ($t0:ty,$($t1:ty),*) => {
        $(
            impl<const P: usize> TryFrom<$t0> for FixedPoint<$t1, P> {
                type Error = Box<dyn Error>;

                fn try_from(value: $t0) -> Result<Self, Self::Error> {
                    Ok(Self {
                        value: ((value * (1i64 << P) as $t0).round() as i64).try_into()?,
                    })
                }
            }

            impl<const P: usize> From<FixedPoint<$t1, P>> for $t0 {
                fn from(value: FixedPoint<$t1, P>) -> Self {
                    value.value as $t0 / (1i64 << P) as $t0
                }
            }
        )*
    }
}
impl_from_f!(f32, u8, i8, u16, i16);
impl_from_f!(f64, u8, i8, u16, i16, u32, i32);

impl<T: Add<Output = T>, const P: usize> Add for FixedPoint<T, P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

pub fn test() -> Result<(), Box<dyn Error>> {
    // for value in 0u8..=255 {
    //     let fp = FixedPoint::<u8, 8> { value: value as u8 };
    //     println!("{:?}", fp);
    // }
    let fp = TryInto::<FixedPoint<u8, 8>>::try_into(0)?;
    println!("{:?}", TryInto::<u8>::try_into(fp)?);

    // if let Some(v) = 1u8.checked_shr(8) {
    //     println!("{}", v);
    // } else {
    //     println!("err");
    // }

    Ok(())
}
