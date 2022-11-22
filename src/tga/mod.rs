//! TGAHeader.color_map_type:
//!     0=indicates that no color-map data is included with this image.
//!     1=indicates that a color-map is included with this image.
//!
//! TGAHeader.image_type:
//!     0=no image data included.
//!     1=uncompressed, color-mapped image.
//!     2=uncompressed, RGB image.
//!     3=uncompressed, black and white image.
//!     9=runlength encoded color-mapped image.
//!    10=runlength encoded RGB image.
//!    11=compressed, black and white image.
//!    32=compressed color-mapped data, using Huffman, Delta, and runlength encoding.
//!    33=compressed color-mapped data, using Huffman, Delta, and runlength encoding. 4-pass quadtree-type process.
//!

use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

pub mod color;
pub mod header;
pub mod image;

pub enum TGAError {
    FailNew = 1,
    FailLoad = 2,
    Header = 3,
}

impl Error for TGAError {}

impl Debug for TGAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FailNew => "新建失败",
                Self::FailLoad => "加载失败",
                Self::Header => "文件头",
            }
        )
    }
}

impl Display for TGAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FailNew => "新建失败",
                Self::FailLoad => "加载失败",
                Self::Header => "文件头",
            }
        )
    }
}
