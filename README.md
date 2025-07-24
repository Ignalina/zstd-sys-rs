# zstd-sys-rs

Low-level Rust FFI bindings to a statically linked Zstandard (`libzstd.a`) library.  
The Zstandard license is included in `zstd.license`.

This crate is used internally by the [Znippy](https://github.com/Ignalina/znippy) project.  
Version numbering corresponds to the bundled Zstandard version.

## 🔨 Build

To build the static Zstandard library (`libzstd.a`) yourself:

```bash
git clone https://github.com/facebook/zstd.git
cd zstd
make lib-mt  # Build with multithread support (ZSTD_MULTITHREAD=ON)
