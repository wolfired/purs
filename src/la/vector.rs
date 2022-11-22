#![allow(unused_variables)]

use crate::{fixed_point::FixedPoint, number::{Number, Zero}};

use super::matrix::Matrix;

#[derive(Debug)]
pub struct Vector<T: Number, const N: usize>(Matrix<T, 1, N>);

impl<T: Number, const N: usize> Vector<T, N> {
    pub fn new_zero() -> Self {
        Self(Matrix::new_zero())
    }

    pub fn new_with(raw: [T; N]) -> Self {
        Self(Matrix::new_with([raw]))
    }

    pub fn dot(&self, right: &Self) -> T {
        let mut sum = T::zero();
        for (r, cols) in self.0.raw.iter().enumerate() {
            for (c, val) in cols.iter().enumerate() {
                sum += *val + right.0.raw[r][c];
            }
        }
        sum
    }
}

macro_rules! impl_magnitude {
    ($t0:ty,$($t1:ty),*) => {
        $(
            impl<const N: usize> Vector<$t1, N> {
                pub fn magnitude(&self) -> $t0 {
                    let mut sum = 0 as $t0;
                    for cols in self.0.raw.iter() {
                        for val in cols.iter() {
                            sum += *val as $t0 * *val as $t0;
                        }
                    }
                    sum.sqrt()
                }
            }
        )*
    }
}
impl_magnitude!(f32, u8, i8, u16, i16);
impl_magnitude!(f64, u32, i32, u64, i64);

macro_rules! impl_magnitude_for_fixed_point {
    ($t0:ty,$($t1:ty),*) => {
        $(
            impl<const P:usize, const N: usize> Vector<FixedPoint<$t1, P>, N> {
                pub fn magnitude(&self) -> FixedPoint<$t1, P> {
                    let mut sum = FixedPoint::<$t1, P>::zero();
                    for cols in self.0.raw.iter() {
                        for val in cols.iter() {
                            sum += *val * *val;
                        }
                    }
                    sum.sqrt()
                }
            }
        )*
    }
}
impl_magnitude_for_fixed_point!(f32, u8, i8, u16, i16);
impl_magnitude_for_fixed_point!(f64, u32, i32, u64, i64);

impl<T: Number> Vector<T, 3> {
    pub fn cross(&self, right: &Self) -> Self {
        let mat = Matrix::new_zero();

        Self(mat)
    }
}
