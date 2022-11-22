pub struct TGAColor<const N: usize> {
    raw: [u8; N],
}

impl<const N: usize> TGAColor<N> {
    pub fn new() -> Self {
        Self { raw: [0; N] }
    }
}

impl TGAColor<2> {
    pub fn b(&self) -> u8 {
        self.raw[0]
    }
    pub fn g(&self) -> u8 {
        self.raw[1]
    }
    pub fn r(&self) -> u8 {
        self.raw[2]
    }
    pub fn a(&self) -> u8 {
        self.raw[3]
    }
}

impl TGAColor<3> {
    pub fn b(&self) -> u8 {
        self.raw[0]
    }
    pub fn g(&self) -> u8 {
        self.raw[1]
    }
    pub fn r(&self) -> u8 {
        self.raw[2]
    }
}

impl TGAColor<4> {
    pub fn b(&self) -> u8 {
        self.raw[0]
    }
    pub fn g(&self) -> u8 {
        self.raw[1]
    }
    pub fn r(&self) -> u8 {
        self.raw[2]
    }
    pub fn a(&self) -> u8 {
        self.raw[3]
    }
}
