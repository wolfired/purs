use std::{array::from_mut, cell::RefCell, fmt::Display, ops::AddAssign, slice::from_raw_parts};
use std::convert::TryInto;

pub trait ICodec<const N: usize> {
    fn encode(&self, rb: &InstantBuffer<N>);
    fn decode(&mut self, rb: &InstantBuffer<N>);
}

#[derive(Debug)]
pub struct InstantBuffer<const N: usize> {
    r: RefCell<isize>,
    w: RefCell<isize>,
    raw: [u8; N],
}

impl<const N: usize> Display for InstantBuffer<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InstantBuffer {{ r: {:08}, w: {:08}, raw: [u8; {}] }}",
            self.get_r(),
            self.get_w(),
            N
        )
    }
}

impl<const N: usize> InstantBuffer<N> {
    pub fn new() -> Self {
        Self {
            r: RefCell::new(0),
            w: RefCell::new(0),
            raw: [0; N],
        }
    }

    pub fn write_object<T: ICodec<N>>(&self, obj: &T) {
        obj.encode(self);
    }

    pub fn read_object<T: Default + ICodec<N>>(&self) -> Option<T> {
        if *self.r.borrow() < *self.w.borrow() {
            let mut obj = T::default();
            obj.decode(self);
            Some(obj)
        } else {
            None
        }
    }

    fn write_bytes(&self, bytes: &[u8]) {
        unsafe {
            (self.raw.as_ptr() as *mut u8)
                .add(self.get_w())
                .copy_from_nonoverlapping(bytes.as_ptr(), bytes.len());
            self.inc_w(bytes.len() as isize);
        }
    }

    fn read_bytes(&self, bytes: &mut [u8]) {
        unsafe {
            self.raw
                .as_ptr()
                .add(self.get_r())
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), bytes.len());
            self.inc_r(bytes.len() as isize);
            if self.get_r() == self.get_w() {
                *self.r.borrow_mut() = 0;
                *self.w.borrow_mut() = 0;
            }
        }
    }

    fn read_array<const M: usize>(&self) -> [u8; M] {
        unsafe {
            let r = from_raw_parts(self.raw.as_ptr().add(self.get_r()), M)
                .try_into()
                .unwrap();
            self.inc_r(M as isize);
            if self.get_r() == self.get_w() {
                *self.r.borrow_mut() = 0;
                *self.w.borrow_mut() = 0;
            }
            r
        }
    }

    fn inc_r(&self, d: isize) {
        self.r.borrow_mut().add_assign(d);
    }

    fn get_r(&self) -> usize {
        *self.r.borrow() as usize
    }

    fn inc_w(&self, d: isize) {
        self.w.borrow_mut().add_assign(d);
    }

    fn get_w(&self) -> usize {
        *self.w.borrow() as usize
    }
}

macro_rules! impl_num {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> ICodec<N> for $t {
                fn encode(&self, rb: &InstantBuffer<N>) {
                    rb.write_bytes(self.to_be_bytes().as_slice());
                }

                fn decode(&mut self, rb: &InstantBuffer<N>) {
                    *self = <$t>::from_be_bytes(rb.read_array());
                }
            }
        )*
    }
}

impl_num!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl<const N: usize> ICodec<N> for bool {
    fn encode(&self, rb: &InstantBuffer<N>) {
        rb.write_bytes(&[*self as u8][..]);
    }

    fn decode(&mut self, rb: &InstantBuffer<N>) {
        let mut b = 0;
        rb.read_bytes(from_mut(&mut b));
        *self = 0 != b;
    }
}

impl<const N: usize> ICodec<N> for String {
    fn encode(&self, rb: &InstantBuffer<N>) {
        self.len().encode(rb);
        rb.write_bytes(self.as_bytes());
    }

    fn decode(&mut self, rb: &InstantBuffer<N>) {
        let mut l = 0;
        l.decode(rb);
        let mut v = vec![0; l];
        rb.read_bytes(v.as_mut_slice());
        *self = String::from_utf8(v).unwrap()
    }
}
