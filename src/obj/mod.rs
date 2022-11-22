//! http://paulbourke.net/dataformats/obj/

#![allow(dead_code)]

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct Obj {
    pub vs: Vec<(f32, f32, f32)>,
    vts: Vec<(f32, f32)>,
    vns: Vec<(f32, f32, f32)>,
    pub fs: Vec<(
        (usize, usize, usize),
        (usize, usize, usize),
        (usize, usize, usize),
    )>,
}

impl Obj {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let mut vs = Vec::new();
        let mut vts = Vec::new();
        let mut vns = Vec::new();
        let mut fs = Vec::new();

        for line in content.lines() {
            if line.starts_with("v ") {
                let ds: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| w.parse().unwrap())
                    .collect();
                vs.push((ds[0], ds[1], ds[2]));
            } else if line.starts_with("vt ") {
                let ds: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .take(2)
                    .map(|w| w.parse().unwrap())
                    .collect();
                vts.push((ds[0], ds[1]));
            } else if line.starts_with("vn ") {
                let ds: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| w.parse().unwrap())
                    .collect();
                vns.push((ds[0], ds[1], ds[2]));
            } else if line.starts_with("f ") {
                let ds: Vec<(usize, usize, usize)> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| {
                        let ds: Vec<usize> = w.split("/").map(|w| w.parse().unwrap()).collect();
                        (ds[0], ds[1], ds[2])
                    })
                    .collect();
                fs.push((ds[0], ds[1], ds[2]));
            }
        }

        Ok(Self { vs, vts, vns, fs })
    }
}
