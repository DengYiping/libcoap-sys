// build.rs
use autotools;
use autotools::Config;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen;

fn main() {
    let autogen = Command::new("sh")
        .current_dir("libcoap")
        .arg("-c")
        .arg("./autogen.sh")
        .output()
        .expect("failed to run autogen.sh");

    assert!(autogen.status.success(), "autogen.sh failed");

    // Build the project in the path `foo` and installs it in `$OUT_DIR`
    let dst = Config::new("libcoap")
        .enable("manpages", Some("no"))
        .enable("doxygen", Some("no"))
        .enable("examples", Some("no"))
        .enable("dtls", None)
        //.with("tinydtls", None)
        .with("openssl", None)
        .build();

    // // Simply link the library without using pkg-config
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=coap-2-openssl");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/include/coap2/coap.h", dst.display()))
        .clang_arg(format!("-I{}/include/coap2", dst.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
