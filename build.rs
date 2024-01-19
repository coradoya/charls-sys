use std::env;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let target = env::var("TARGET").unwrap();

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

    if !target.contains("msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
}
