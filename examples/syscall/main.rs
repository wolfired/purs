use std::{slice::from_raw_parts, vec, ffi::{CString}};

use purs::{syscall_open, syscall_fstat, syscall_read, syscall_write, syscall_mmap};

fn main() {
    if cfg!(windows) {
        return;
    }

    let fd = syscall_open(CString::new("/etc/hosts").unwrap().to_str().unwrap());

    let buf = &mut [0; 144][..];
    let _ret = syscall_fstat(fd, buf);
    let fsize = unsafe { *(buf.as_ptr().add(48) as *const usize) };

    let mut buf = vec![0; fsize];
    let size = syscall_read(fd, buf.as_mut_slice());
    let _size = syscall_write(1, &buf.as_slice()[..size]);

    let addr = syscall_mmap(fd, fsize);
    let buf2 = unsafe { from_raw_parts(addr as *const u8, fsize) };
    let size = syscall_write(1, buf2);

    println!("{}", size);
}
