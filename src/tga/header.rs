use std::error::Error;
use std::fmt::Display;
use std::io::Write;

use super::TGAError;

pub(super) const TGAHEADER_BYTES_COUNT: usize = 18;

#[derive(Default)]
pub(super) struct TGAHeader {
    pub(super) id_length: u8,
    pub(super) color_map_type: u8,
    pub(super) image_type: u8,
    pub(super) color_map_origin: u16,
    pub(super) color_map_entries_count: u16,
    pub(super) color_map_bits_per_entry: u8,
    pub(super) image_origin_x: u16,
    pub(super) image_origin_y: u16,
    pub(super) image_width: u16,
    pub(super) image_height: u16,
    pub(super) image_bits_per_pixel: u8,
    pub(super) image_descriptor: u8,
}

impl TGAHeader {
    pub(super) fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if TGAHEADER_BYTES_COUNT > bytes.len() {
            return Err(TGAError::Header.into());
        }
        
        Ok(Self {
            id_length: bytes[0],
            color_map_type: bytes[1],
            image_type: bytes[2],
            color_map_origin: u16::from_le_bytes(bytes[3..5].try_into()?),
            color_map_entries_count: u16::from_le_bytes(bytes[5..7].try_into()?),
            color_map_bits_per_entry: bytes[7],
            image_origin_x: u16::from_le_bytes(bytes[8..10].try_into()?),
            image_origin_y: u16::from_le_bytes(bytes[10..12].try_into()?),
            image_width: u16::from_le_bytes(bytes[12..14].try_into()?),
            image_height: u16::from_le_bytes(bytes[14..16].try_into()?),
            image_bits_per_pixel: bytes[16],
            image_descriptor: bytes[17],
        })
    }

    pub(super) fn to_bytes(&self, mut bytes: &mut [u8]) -> Result<(), Box<dyn Error>> {
        if TGAHEADER_BYTES_COUNT > bytes.len() {
            return Err(TGAError::Header.into());
        }

        bytes.write_all(&self.id_length.to_le_bytes())?;
        bytes.write_all(&self.color_map_type.to_le_bytes())?;
        bytes.write_all(&self.image_type.to_le_bytes())?;
        bytes.write_all(&self.color_map_origin.to_le_bytes())?;
        bytes.write_all(&self.color_map_entries_count.to_le_bytes())?;
        bytes.write_all(&self.color_map_bits_per_entry.to_le_bytes())?;
        bytes.write_all(&self.image_origin_x.to_le_bytes())?;
        bytes.write_all(&self.image_origin_y.to_le_bytes())?;
        bytes.write_all(&self.image_width.to_le_bytes())?;
        bytes.write_all(&self.image_height.to_le_bytes())?;
        bytes.write_all(&self.image_bits_per_pixel.to_le_bytes())?;
        bytes.write_all(&self.image_descriptor.to_le_bytes())?;

        Ok(())
    }
}

impl Display for TGAHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"
ID Length: {},
Color Map Type: {},
Image Type: {},
Color Map Specification:
    Color Map Origin: {},
    Color Map Entries Count: {},
    Color Map Bits Per Entry: {},
Image Specification:
    X Origin of Image: {},
    Y Origin of Image: {},
    Width of Image: {},
    Height of Image: {},
    Image Bits Per Pixel: {},
    Image Descriptor: {:08b}
",
            self.id_length,
            self.color_map_type,
            self.image_type,
            self.color_map_origin,
            self.color_map_entries_count,
            self.color_map_bits_per_entry,
            self.image_origin_x,
            self.image_origin_y,
            self.image_width,
            self.image_height,
            self.image_bits_per_pixel,
            self.image_descriptor
        )
    }
}
