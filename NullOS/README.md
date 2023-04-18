
#  Screenshots

![screenshot](https://i.imgur.com/uFgH0J1.png)

#  Compilation Manual

A linux environment is needed for compiling, dosfstools is needed to generate a disk img.

###  Install Rust

Install rustup from https://www.rustup.rs

###  Configure Rust

Set the default keychain to nightly: `rustup override add nightly`

###  Install Xargo

Install Xargo, a wrapper for cargo that eases cross compilation.

`cargo install xargo`

Add the rust source code component for cross compiling (needed by xargo).

`rustup component add rust-src`

Updates the nightly version of Rust toolchain forcefully, ignoring any warnings or errors.

`rustup update nightly --force`

Installs Bootimage, a tool that simplifies building bootable disk images for Rust kernels.

`cargo install bootimage`

Adds the LLVM Tools Preview component to the Rust toolchain, providing additional tools for working with LLVM-based code generation in Rust.

`rustup component add llvm-tools-preview`

### Build a Bootable Image

Build using `cargo bootimage`

###  Run

Run using `cargo run`