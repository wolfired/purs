pub mod pbm;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[rustfmt::skip]
    pub fn black() -> Self { Color { r: 0, g: 0, b: 0, a: 255 } }

    #[rustfmt::skip]
    pub fn white() -> Self { Color { r: 255, g: 255, b: 255, a: 255 } }
}

fn is_newline(c: &u8) -> bool {
    return b'\r' == *c || b'\n' == *c;
}

fn is_space(c: &u8) -> bool {
    return b'\t' == *c || b'\n' == *c || 0x0b == *c || 0x0c == *c || b'\r' == *c || b' ' == *c;
}

fn is_comment(c: &u8) -> bool {
    return b'#' == *c;
}
