use std::env;
use std::path::PathBuf;
use bindgen::EnumVariation;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // Compile C files
    cc::Build::new()
        .include("zstd/lib")
        .define("ZSTD_MULTITHREAD", None)
        .define("ZSTD_STATIC_LINKING_ONLY", None)
        .include("zstd/lib/common")
        .include("zstd/include")
        .files([
            "zstd/lib/common/debug.c",
            "zstd/lib/common/entropy_common.c",
            "zstd/lib/common/error_private.c",
            "zstd/lib/common/fse_decompress.c",
            "zstd/lib/common/pool.c",
            "zstd/lib/common/threading.c",
            "zstd/lib/common/xxhash.c",
            "zstd/lib/common/zstd_common.c",
            "zstd/lib/compress/zstd_compress.c",
            "zstd/lib/compress/zstd_compress_literals.c",
            "zstd/lib/compress/zstd_compress_sequences.c",
            "zstd/lib/compress/zstd_double_fast.c",
            "zstd/lib/compress/zstd_fast.c",
            "zstd/lib/compress/zstd_lazy.c",
            "zstd/lib/compress/zstd_ldm.c",
            "zstd/lib/compress/zstd_opt.c",
            "zstd/lib/compress/zstdmt_compress.c",
            "zstd/lib/decompress/huf_decompress.c",
            "zstd/lib/decompress/zstd_ddict.c",
            "zstd/lib/decompress/zstd_decompress.c",
            "zstd/lib/decompress/zstd_decompress_block.c",
        ])
        .flag_if_supported("-std=c99")
        .define("ZSTD_MULTITHREAD", None)
        .define("ZSTD_STATIC_LINKING_ONLY", None)
        .compile("zstd");

    println!("cargo:rustc-link-lib=static=zstd");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-DZSTD_MULTITHREAD")
        .clang_arg("-DZSTD_STATIC_LINKING_ONLY")
        .clang_arg("-Izstd/include")
        .clang_arg("-Izstd/lib")
        .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
        .allowlist_function("ZSTD_.*")
        .allowlist_type("ZSTD_.*")
        .allowlist_var("ZSTD_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
