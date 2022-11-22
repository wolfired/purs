use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::number::Number;

#[derive(Debug)]
pub struct Matrix<T: Number, const R: usize, const C: usize> {
    pub(super) raw: [[T; C]; R],
}

impl<T: Number, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new_zero() -> Self {
        Self {
            raw: [[T::zero(); C]; R],
        }
    }

    pub fn new_with(raw: [[T; C]; R]) -> Self {
        Self { raw }
    }

    pub fn set(&mut self, r: usize, c: usize, val: T) {
        self.raw[r][c] = val;
    }

    pub fn get(&self, r: usize, c: usize) -> T {
        self.raw[r][c]
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut raw = [[T::default(); R]; C];

        for r in 0..R {
            for c in 0..C {
                raw[c][r] = self.raw[r][c];
            }
        }

        Matrix::<T, C, R> { raw }
    }

    pub fn mul<const O: usize>(&self, right: &Matrix<T, C, O>) -> Matrix<T, R, O> {
        let mut raw = [[T::zero(); O]; R];

        for r in 0..R {
            for c in 0..C {
                for o in 0..O {
                    raw[r][o] += self.raw[r][c] * right.raw[c][o];
                }
            }
        }

        Matrix::<T, R, O> { raw }
    }
}

impl<T: Number, const N: usize> Matrix<T, N, N> {
    pub fn new_identity() -> Self {
        let mut raw = [[T::zero(); N]; N];

        for i in 0..N {
            raw[i][i] = T::one();
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::default(); C]; R];

        for (r, cols) in raw.iter_mut().enumerate() {
            for (c, val) in cols.iter_mut().enumerate() {
                *val = self.raw[r][c] + rhs.raw[r][c];
            }
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> AddAssign for Matrix<T, R, C> {
    fn add_assign(&mut self, rhs: Self) {
        for (r, cols) in self.raw.iter_mut().enumerate() {
            for (c, val) in cols.iter_mut().enumerate() {
                *val += rhs.raw[r][c];
            }
        }
    }
}

impl<T: Number, const R: usize, const C: usize> Sub for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::default(); C]; R];

        for (r, cols) in raw.iter_mut().enumerate() {
            for (c, val) in cols.iter_mut().enumerate() {
                *val = self.raw[r][c] - rhs.raw[r][c];
            }
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> SubAssign for Matrix<T, R, C> {
    fn sub_assign(&mut self, rhs: Self) {
        for (r, cols) in self.raw.iter_mut().enumerate() {
            for (c, val) in cols.iter_mut().enumerate() {
                *val -= rhs.raw[r][c];
            }
        }
    }
}
