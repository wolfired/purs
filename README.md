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