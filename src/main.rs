#![crate_name = "purs"]
#![crate_type = "bin"]

use std::fs::{read, OpenOptions};
use std::io::Write;

use purs::netpbm::PNM;
use purs::fixed_point::FixedPoint;

fn main() {
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .truncate(true)
    //     .open("2048x2048.pbm")
    //     .unwrap();

    // for _ in 0..4 {
    //     file.write(&[0x31, 0x20][..]);
    // }

    // let pnm = PNM::from(read("./2x2.pbm").unwrap());
    // println!("{:?}", pnm);
    let mut fp0 = FixedPoint::<isize, 8>::from(25.5);
    let fp1 = FixedPoint::<isize, 8>::from(6.2);
    println!("{}", f64::from(fp0));
    println!("{}", f64::from(fp1));
    fp0 -= fp1;
    println!("{}", f64::from(fp0));
}
