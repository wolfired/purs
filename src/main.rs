#![crate_name = "purs"]
#![crate_type = "bin"]

// use std::fs::{read, OpenOptions};
// use std::io::Write;

use purs::fixed_point::{FixedPoint};
// use purs::netpbm::PNM;

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
    let mut fp0 = FixedPoint::<i64, 23>::from(f32::EPSILON);
    let mut fp1 = FixedPoint::<i64, 23>::from(-1.0);
    // unsafe {
    //     *(&mut fp0.value as *const i64 as *mut u64) = 0b0_0000000_00000000_00000000_00000000_00000000_0_1111111_11111111_11111111;
    // }
    println!("{}", f32::from(fp0 + fp1));
    println!("{}", -1.0 + f32::EPSILON);
}
