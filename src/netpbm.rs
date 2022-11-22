use std::convert::TryInto;

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

#[derive(Debug)]
pub struct PNM {
    pub magic_number: u16,
    pub width: i32,
    pub height: i32,
    pub colors: Vec<Color>,
}

impl From<Vec<u8>> for PNM {
    fn from(raw: Vec<u8>) -> Self {
        let (mut offset, magic_number, width, height, colors);
        (offset, magic_number) = u16::dump_from_be_bytes(raw.as_slice(), 0);
        (offset, width) = i32::dump_from_str(raw.as_slice(), offset);
        (offset, height) = i32::dump_from_str(raw.as_slice(), offset);
        (_, colors) = dump_raster(
            magic_number,
            (width * height) as usize,
            raw.as_slice(),
            offset,
        );
        Self {
            magic_number,
            width,
            height,
            colors,
        }
    }
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

fn skip_space(raw: &[u8], mut offset: usize) -> usize {
    for (i, c) in raw.iter().skip(offset).enumerate() {
        if !is_space(c) {
            offset += i;
            break;
        }
    }

    offset
}

fn skip_comment(raw: &[u8], mut offset: usize) -> usize {
    offset = skip_space(raw, offset);

    let mut comment_marker = false;
    for (i, c) in raw.iter().skip(offset).enumerate() {
        if comment_marker {
            if is_newline(c) {
                comment_marker = false;
            }
        } else if is_comment(c) {
            comment_marker = true;
        } else {
            offset += i;
            break;
        }
    }

    offset
}

trait DumpNumber {
    fn dump_from_be_bytes(raw: &[u8], offset: usize) -> (usize, Self);
}

macro_rules! impl_dump_integer {
    ($($t:ty),*) => {
        $(
            impl DumpNumber for $t {
                fn dump_from_be_bytes(raw: &[u8], mut offset: usize) -> (usize, Self) {
                    offset = skip_comment(raw, offset);

                    let mut magic_number = <$t>::MIN;
                    for (i, c) in raw.iter().skip(offset).enumerate() {
                        if is_space(c) {
                            magic_number = <$t>::from_be_bytes(raw[offset..(offset + i)].try_into().unwrap());
                            offset += i;
                            break;
                        } else if (offset + i) == (raw.len() - 1) {
                            magic_number = <$t>::from_be_bytes(raw[offset..=(offset + i)].try_into().unwrap());
                            offset += i;
                            break;
                        }
                    }

                    (offset, magic_number)
                }
            }
        )*
    }
}

impl_dump_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

trait DumpInterger {
    fn dump_from_str(raw: &[u8], offset: usize) -> (usize, Self);
}

macro_rules! impl_dump_integer {
    ($($t:ty),*) => {
        $(
            impl DumpInterger for $t {
                fn dump_from_str(raw: &[u8], mut offset: usize) -> (usize, Self) {
                    offset = skip_comment(raw, offset);

                    let mut value = <$t>::MIN;
                    for (i, c) in raw.iter().skip(offset).enumerate() {
                        if is_space(c) {
                            value = <$t>::from_str_radix(&String::from_utf8_lossy(&raw[offset..(offset + i)]), 10)
                                .unwrap();
                            offset += i;
                            break;
                        } else if (offset + i) == (raw.len() - 1) {
                            value = <$t>::from_str_radix(&String::from_utf8_lossy(&raw[offset..=(offset + i)]), 10)
                                .unwrap();
                            offset += i;
                            break;
                        }
                    }

                    (offset, value)
                }
            }
        )*
    }
}

impl_dump_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

fn dump_raster(
    _magic_number: u16,
    size: usize,
    raw: &[u8],
    mut offset: usize,
) -> (usize, Vec<Color>) {
    let raster = Vec::with_capacity(size);

    for _ in 0..size {
        (offset, _) = u8::dump_from_str(raw, offset);
    }

    (offset, raster)
}
