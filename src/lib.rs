#![crate_name = "purs"]
#![crate_type = "lib"]

mod purc;
pub use purc::purc_max;
pub use purc::purc_call_rs;

pub mod pseudorandom;
