#![crate_name = "purs"]
#![crate_type = "bin"]

use purs::pseudorandom::middle_square_factory;

fn main() {
    let mut gen = middle_square_factory(1123);
    println!("{}", gen());
}
