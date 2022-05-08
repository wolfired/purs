#![crate_name = "prust"]
#![crate_type = "bin"]

use prust::com::wolfired;

fn main() {
    println!("hello max: {}", wolfired::max(3, 2));
    println!("hello max: {}", wolfired::max(1, 2));
    let sun = wolfired::Sun::new("吴思翰", 6);
    sun.hi();
    sun.hello((1, 2));
}
