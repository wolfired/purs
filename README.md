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

# build exe
cl /Fo:.\out\purc.obj /DBUILD_DLL .\purc\main.c .\out\libpurc.lib /link /OUT:.\out\purc

```