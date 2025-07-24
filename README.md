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

```

✨ Heder

  🧙‍♂Med Allfaderns visdom, kompression och korruptionsskydd.  
  ⚡ Med hans blick över varje bit.  

## License

- Zstandard is licensed under the BSD 3-Clause License — se filen `zstd.license` för fullständig information.  
- Denna crate är licensierad under MIT eller Apache-2.0 (ditt val).

Vänligen respektera dessa licenser vid användning och distribution av denna mjukvara.
