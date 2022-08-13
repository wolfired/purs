#![crate_name = "purs"]
#![crate_type = "lib"]

mod purc;
pub use purc::{purc_call_rs, purc_max};

pub mod pseudorandom;

mod syscall;
pub use syscall::{syscall_fstat, syscall_mmap, syscall_open, syscall_read, syscall_write};

pub mod hexagons;
pub mod instant_buffer;
