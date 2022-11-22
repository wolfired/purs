use std::convert::TryInto;

use super::{is_comment, is_newline, is_space, Color};

#[derive(Debug)]
pub struct PBM {
    pub wid: i32,
    pub hei: i32,
    pub colors: Vec<Color>,
}

impl PBM {
    pub fn from_raw(raw: &[u8]) -> Result<Self, ()> {
        let mut wid = 0;
        let mut hei = 0;
        let mut colors = Vec::<Color>::with_capacity(0);

        let mut marker_magic_number = (true, false, 0);
        let mut marker_wid = (false, false, 0);
        let mut marker_hei = (false, false, 0);
        let mut marker_raster = (false, false, 0);
        let mut marker_comment = (false, false, 0);
        for (i, c) in raw.iter().enumerate() {
            if marker_comment.0 {
                if marker_comment.1 {
                    if is_newline(c) {
                        println!(
                            "{}",
                            String::from_utf8(raw[marker_comment.2..i].try_into().unwrap())
                                .unwrap()
                        );
                        marker_comment.0 = false;
                        marker_comment.1 = false;
                    }
                } else if !is_space(c) {
                    marker_comment.1 = true;
                    marker_comment.2 = i;
                }
            } else if is_comment(c) {
                marker_comment.0 = true;
            } else if marker_magic_number.0 {
                if marker_magic_number.1 {
                    if is_space(c) {
                        println!(
                            "{}",
                            String::from_utf8(raw[marker_magic_number.2..i].try_into().unwrap())
                                .unwrap()
                        );
                        marker_magic_number.0 = false;
                        marker_magic_number.1 = false;

                        marker_wid.0 = true
                    }
                } else if !is_space(c) {
                    marker_magic_number.1 = true;
                    marker_magic_number.2 = i;
                }
            } else if marker_wid.0 {
                if marker_wid.1 {
                    if is_space(c) {
                        wid = i32::from_str_radix(
                            &String::from_utf8(raw[marker_wid.2..i].try_into().unwrap()).unwrap(),
                            10,
                        )
                        .unwrap();

                        marker_wid.0 = false;
                        marker_wid.1 = false;

                        marker_hei.0 = true
                    }
                } else if !is_space(c) {
                    marker_wid.1 = true;
                    marker_wid.2 = i;
                }
            } else if marker_hei.0 {
                if marker_hei.1 {
                    if is_space(c) {
                        hei = i32::from_str_radix(
                            &String::from_utf8(raw[marker_hei.2..i].try_into().unwrap()).unwrap(),
                            10,
                        )
                        .unwrap();

                        colors = Vec::<Color>::with_capacity(wid as usize * hei as usize);

                        marker_hei.0 = false;
                        marker_hei.1 = false;

                        marker_raster.0 = true
                    }
                } else if !is_space(c) {
                    marker_hei.1 = true;
                    marker_hei.2 = i;
                }
            } else if marker_raster.0 {
                if marker_raster.1 {
                    if is_space(c) {
                        let is_black = 0
                            != i32::from_str_radix(
                                &String::from_utf8(raw[marker_raster.2..i].try_into().unwrap())
                                    .unwrap(),
                                10,
                            )
                            .unwrap();

                        if is_black {
                            colors.push(Color::black());
                        } else {
                            colors.push(Color::white());
                        }

                        marker_raster.1 = false;
                    }
                } else if !is_space(c) {
                    marker_raster.1 = true;
                    marker_raster.2 = i;
                }
            }
        }
        Ok(Self { wid, hei, colors })
    }
}
