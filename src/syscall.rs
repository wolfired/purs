use std::arch::asm;

pub fn syscall_open(path: &str) -> usize {
    let ret: usize;

    unsafe {
        let path = *(&path as *const _ as *const usize);
        asm!(
            "syscall",
            inlateout("rax") 2usize => ret,
            in("rdi") path,
            in("rsi") 0usize,
        );
    }

    return ret;
}

pub fn syscall_read(fd: usize, buf: &mut [u8]) -> usize {
    let ret: usize;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 0usize => ret,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
        );
    }

    return ret;
}

pub fn syscall_write(fd: usize, buf: &[u8]) -> usize {
    let ret: usize;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 1usize => ret,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
        );
    }

    return ret;
}

pub fn syscall_fstat(fd: usize, buf: &mut [u8]) -> usize {
    let ret: usize;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 5usize => ret,
            in("rdi") fd,
            in("rsi") buf.as_ptr(),
        );
    }

    return ret;
}

pub fn syscall_mmap(fd: usize, size: usize) -> usize {
    let ret: usize;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 9usize => ret,
            in("rdi") 0usize,
            in("rsi") size,
            in("rdx") 0x1usize,
            in("r10") 0x2usize,
            in("r8") fd,
            in("r9") 0usize,
        );
    }

    return ret;
}
