# purs

a pure rustc project template for vscode

# vscode extensions

* CodeLLDB
* rust-analyzer

# first use

```bash

code --install-extension vadimcn.vscode-lldb && \
code --install-extension rust-lang.rust-analyzer && \
rustup toolchain install nightly && \
rustup default nightly && \
rustup component add llvm-tools-preview rust-src && \
cargo install rustfilt && \
./.vscode/prepare.sh

```

```batch

# build dll
cl /Fo:.\target\libpurc.obj /LD /DBUILD_DLL .\purc\lib.c /link /OUT:.\target\purc.dll /IMPLIB:.\target\purc.lib

# build lib
cl /Fo:.\target\libpurc.obj /DBUILD_DLL /c .\purc\lib.c
lib /OUT:.\target\purc.lib .\target\libpurc.obj

# build bin
cl /Fo:.\target\purc.obj /DBUILD_DLL .\purc\main.c .\target\purc.lib /link /NOIMPLIB /OUT:.\target\purc

# build so
gcc -fPIC -shared -o ./target/libpurc.so ./purc/lib.c
export LD_LIBRARY_PATH=./target

# build a
gcc -c -o ./target/libpurc.o ./purc/lib.c
ar rcs ./target/libpurc.a ./target/libpurc.o

#build bin
gcc -o ./target/purc -L./target ./purc/main.c -lpurc

```

# code snippet

* system call

```rust
use std::{slice::from_raw_parts, vec, ffi::{CStr, CString}};

use purs::{syscall_open, syscall_fstat, syscall_read, syscall_write, syscall_mmap};

fn main() {
    let fd = syscall_open(CString::new("/etc/hosts").unwrap().to_str().unwrap());

    let buf = &mut [0; 144][..];
    let ret = syscall_fstat(fd, buf);
    let fsize = unsafe { *(buf.as_ptr().add(48) as *const usize) };

    let mut buf = vec![0; fsize];
    let size = syscall_read(fd, buf.as_mut_slice());
    let size = syscall_write(1, &buf.as_slice()[..size]);

    let addr = syscall_mmap(fd, fsize);
    let buf2 = unsafe { from_raw_parts(addr as *const u8, fsize) };
    let size = syscall_write(1, buf2);

    println!("{}", size);
}
```