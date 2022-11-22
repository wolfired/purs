use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use super::header::TGAHeader;
use super::header::TGAHEADER_BYTES_COUNT;

pub struct TGAImage {
    header: TGAHeader,
    bytes: Vec<u8>,
}

impl TGAImage {
    pub fn new(image_width: u16, image_height: u16) -> Self {
        let image_bits_per_pixel = 32;
        Self {
            header: TGAHeader {
                image_type: 2,
                image_width,
                image_height,
                image_bits_per_pixel,
                image_descriptor: match image_bits_per_pixel {
                    16 => 0b00_00_0001,
                    24 => 0b00_00_0000,
                    32 => 0b00_00_1000,
                    _ => 0b00_00_1000,
                },
                ..TGAHeader::default()
            },
            bytes: vec![
                0;
                (image_width as u64 * image_height as u64 * image_bits_per_pixel as u64)
                    as usize
                    >> 3
            ],
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let header_bytes = &mut [0; TGAHEADER_BYTES_COUNT][..];

        file.read_exact(header_bytes)?;

        let header = TGAHeader::from_bytes(header_bytes)?;

        let mut bytes = vec![
            0;
            (header.image_width as u64
                * header.image_height as u64
                * header.image_bits_per_pixel as u64) as usize
                >> 3
        ];

        file.read_exact(bytes.as_mut_slice())?;

        Ok(Self { header, bytes })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        let header_bytes = &mut [0; TGAHEADER_BYTES_COUNT][..];

        self.header.to_bytes(header_bytes)?;

        file.write_all(header_bytes)?;
        file.write_all(self.bytes.as_slice())?;

        Ok(())
    }

    pub fn get_width(&self) -> u16 {
        self.header.image_width
    }
    pub fn get_height(&self) -> u16 {
        self.header.image_height
    }

    pub fn set_color(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8, a: u8) {
        let index = ((x as u64 + y as u64 * self.header.image_width as u64)
            * self.header.image_bits_per_pixel as u64) as usize
            >> 3;
        self.bytes[index + 0] = b;
        self.bytes[index + 1] = g;
        self.bytes[index + 2] = r;
        self.bytes[index + 3] = a;
    }
}
