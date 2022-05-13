#![crate_name = "purs"]
#![crate_type = "lib"]

pub mod pseudorandom;

pub trait ISun {
    fn hi(&self);
}

pub struct Sun;

impl Sun {
    pub fn hello(&self) {
        println!("hello")
    }
}
