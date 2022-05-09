# purs

a pure rustc project template

# vscode extensions

* CodeLLDB
* rust-analyzer

# steps

```bash

rustup toolchain install nightly && \
rustup default nightly && \
rustup component add llvm-tools-preview rust-src && \
cargo install rustfilt && \
./.vscode/prepare.sh

```
