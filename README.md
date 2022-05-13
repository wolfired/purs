# purs

a pure rustc project template for vscode

# vscode extensions

* CodeLLDB
* rust-analyzer

# first use

```bash

code --install-extension vadimcn.vscode-lldb && \
code --install-extension matklad.rust-analyzer && \
rustup toolchain install nightly && \
rustup default nightly && \
rustup component add llvm-tools-preview rust-src && \
cargo install rustfilt && \
./.vscode/prepare.sh

```

```batch

# build dll
cl /Fo:.\out\libpurc.obj /LD /DBUILD_DLL .\purc\lib.c /link /OUT:.\out\libpurc.dll /IMPLIB:.\out\libpurc.lib

# build lib
cl /Fo:.\out\libpurc.obj /DBUILD_DLL /c .\purc\lib.c
lib /OUT:.\out\libpurc.lib .\out\libpurc.obj

# build bin
cl /Fo:.\out\purc.obj /DBUILD_DLL .\purc\main.c .\out\libpurc.lib /link /OUT:.\out\purc

# build so
gcc -fPIC -shared -o ./out/libpurc.so ./purc/lib.c
export LD_LIBRARY_PATH=./out

# build a
gcc -c -o ./out/libpurc.o ./purc/lib.c
ar rcs ./out/libpurc.a ./out/libpurc.o

#build bin
gcc -o ./out/purc -L./out ./purc/main.c -lpurc

```