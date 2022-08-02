#![crate_name = "purs"]
#![crate_type = "bin"]

use purs::{pseudorandom::middle_square_factory, purc_max, trace};

fn main() {
    let mut gen = middle_square_factory(1123);
    println!("{}", gen());

    trace(format!("max(1, 2) = {}", unsafe { purc_max(1, 2) }).as_str());
}
