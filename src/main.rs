#![crate_name = "purs"]
#![crate_type = "bin"]

use purs::purc_max;
use purs::purc_call_rs;

fn main() {
    unsafe {
        println!("{}", purc_max(1, 3));
        purc_call_rs(Some(|| println!("cb")));
    }
}
