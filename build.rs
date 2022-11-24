use std::env;
use std::path::PathBuf;

fn out_dir() -> PathBuf {
    PathBuf::from(
        env::var("OUT_DIR")
            .expect("OUT_DIR environment variable should be defined")
    )
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut config = cmake::Config::new("charls");
    let dst = config.build();

    #[cfg(feature="static")]
    {
        println!("cargo:rustc-link-lib=static=charls");
    }

    #[cfg(not(feature="static"))]
    {
        println!("cargo:rustc-link-lib=charls");
    }

    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include/", out_dir().display()))
        .clang_arg(format!("-I{}", dst.display()))
        .clang_arg("--language=c++")
        .clang_arg("-std=c++17")
        .header("wrapper.h")
        .allowlist_function("charls_jpegls_decoder_create")
        .allowlist_function("charls_jpegls_decoder_decode_to_buffer")
        .allowlist_function("charls_jpegls_decoder_destroy")
        .allowlist_function("charls_jpegls_decoder_get_destination_size")
        .allowlist_function("charls_jpegls_decoder_get_frame_info")
        .allowlist_function("charls_jpegls_decoder_read_header")
        .allowlist_function("charls_jpegls_decoder_read_spiff_header")
        .allowlist_function("charls_jpegls_decoder_set_source_buffer")
        .allowlist_function("charls_jpegls_encoder_create")
        .allowlist_function("charls_jpegls_encoder_destroy")
        .allowlist_function("charls_jpegls_encoder_encode_from_buffer")
        .allowlist_function("charls_jpegls_encoder_get_bytes_written")
        .allowlist_function("charls_jpegls_encoder_get_estimated_destination_size")
        .allowlist_function("charls_jpegls_encoder_set_destination_buffer")
        .allowlist_function("charls_jpegls_encoder_set_encoding_options")
        .allowlist_function("charls_jpegls_encoder_set_frame_info")
        .allowlist_function("charls_jpegls_encoder_set_interleave_mode")
        .allowlist_function("charls_jpegls_encoder_set_near_lossless")
        .allowlist_function("charls_jpegls_encoder_set_preset_coding_parameters")
        .blocklist_type("charls_encoding_options")
        .blocklist_type("charls_interleave_mode")
        .blocklist_type("charls_jpegls_errc")
        .blocklist_type("charls_spiff_color_space")
        .blocklist_type("charls_spiff_compression_type")
        .blocklist_type("charls_spiff_profile_id")
        .blocklist_type("charls_spiff_resolution_units")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
