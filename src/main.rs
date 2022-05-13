#![crate_name = "purs"]
#![crate_type = "bin"]

use purs::purc_max;

fn main() {
    unsafe {
        println!("{}", purc_max(1, 3));
    }
}
