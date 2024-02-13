fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let dst = cmake::Config::new("charls")
        .define("BUILD_SHARED_LIBS", "0")
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

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}
