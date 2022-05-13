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
cl /Fo:.\out\purc.obj /LD /DBUILD_DLL .\purc\lib.c /link /OUT:.\out\purc.dll /IMPLIB:.\out\purc.lib

# build lib
cl /Fo:.\out\purc.obj /DBUILD_DLL /c .\purc\lib.c
lib /OUT:.\out\purc.lib .\out\purc.obj

```