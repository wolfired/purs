use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct FixedPoint<T, const P: usize> {
    pub value: T,
}

macro_rules! impl_block {
    ($($t:ty),*) => {
        $(
            impl<const P: usize> From<f32> for FixedPoint<$t, P> {
                fn from(ori: f32) -> Self {
                    Self {
                        value: (ori * (1 << P) as f32).round() as $t,
                    }
                }
            }

            impl<const P: usize> From<FixedPoint<$t, P>> for f32 {
                fn from(ori: FixedPoint<$t, P>) -> Self {
                    ori.value as f32 / (1 << P) as f32
                }
            }

            impl<const P: usize> From<f64> for FixedPoint<$t, P> {
                fn from(ori: f64) -> Self {
                    Self {
                        value: (ori * (1 << P) as f64).round() as $t,
                    }
                }
            }

            impl<const P: usize> From<FixedPoint<$t, P>> for f64 {
                fn from(ori: FixedPoint<$t, P>) -> Self {
                    ori.value as f64 / (1 << P) as f64
                }
            }

            impl<const P: usize> Add for FixedPoint<$t, P> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: <$t as Add>::add(self.value, rhs.value)
                    }
                }
            }

            impl<const P: usize> AddAssign for FixedPoint<$t, P> {
                fn add_assign(&mut self, other: Self) {
                    <$t as AddAssign>::add_assign(&mut self.value, other.value)
                }
            }

            impl<const P: usize> Sub for FixedPoint<$t, P> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self::Output {
                        value: <$t as Sub>::sub(self.value, rhs.value)
                    }
                }
            }

            impl<const P: usize> SubAssign for FixedPoint<$t, P> {
                fn sub_assign(&mut self, other: Self) {
                    <$t as SubAssign>::sub_assign(&mut self.value, other.value)
                }
            }
        )*
    }
}
impl_block!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
