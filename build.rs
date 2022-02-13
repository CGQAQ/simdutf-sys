fn main() {
    let mut config = cmake::Config::new("libsimdutf");

    println!("cargo:rerun-if-changed=libsimdutf/src");
    println!("cargo:rerun-if-changed=libsimdutf/include");

    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        // config.define("CMAKE_CXX_COMPILER", "cl.exe");
        config.very_verbose(true);
        config.generator("Visual Studio 17 2022");
        config.build_arg("/property:Configuration=Release");
        let dst = config.build();
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
    } else {
        config.generator("Ninja");
        config.build_arg("--config Release");
        let dst = config.build();
    }


    println!("cargo:rustc-link-lib=static=simdutf");

    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("libsimdutf/include/simdutf.h")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");
    //
    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}