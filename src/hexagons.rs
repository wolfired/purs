use std::ops::{Add, AddAssign};

#[derive(Debug)]
pub struct CoordCube {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl CoordCube {}

impl Add for CoordCube {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl Add for &CoordCube {
    type Output = CoordCube;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl AddAssign for CoordCube {
    fn add_assign(&mut self, rhs: Self) {
        self.q += rhs.q;
        self.r += rhs.r;
        self.s += rhs.s;
    }
}

impl AddAssign<&Self> for CoordCube {
    fn add_assign(&mut self, rhs: &Self) {
        self.q += rhs.q;
        self.r += rhs.r;
        self.s += rhs.s;
    }
}
