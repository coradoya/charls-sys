fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let dst = cmake::Config::new("charls")
        .define("BUILD_SHARED_LIBS", "0")
        .define("CMAKE_LINK_DEPENDS_USE_LINKER", "0")
        .always_configure(true)
        .build();

    #[cfg(feature = "static")]
    {
        println!("cargo:rustc-link-lib=static=charls");
    }

    #[cfg(not(feature = "static"))]
    {
        println!("cargo:rustc-link-lib=charls");
    }

    if let Ok(inner) = std::env::var("CARGO_CFG_TARGET_OS") {
        match inner.as_str() {
            "linux" => {
                println!("cargo:rustc-link-lib=stdc++");
            }
            "macos" => {
                println!("cargo:rustc-link-lib=c++");
            }
            _ => {}
        }
    }
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}
