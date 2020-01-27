// build.rs
use autotools;
use autotools::Config;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let proj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = format!("{}/libcoap", proj_dir);
    let out_lib_dir = format!("{}/libcoap", out_dir);
    //let out_lib_dir = &out_dir;
    let _e = std::fs::remove_dir_all(&out_lib_dir);

    // println!("out libdir: {}", out_lib_dir);
    // println!("src dir: {}", src_dir);
    let options = fs_extra::dir::CopyOptions::new(); //Initialize default values for CopyOptions
                                                     // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
    fs_extra::dir::copy(&src_dir, &out_dir, &options).unwrap();

    // just copy the damn library to build path...

    // let autogen = Command::new("sh")
    //     .current_dir(out_dir)
    //     .arg("-c")
    //     .arg("./autogen.sh")
    //     .output()
    //     .expect("failed to run autogen.sh");
    //
    let _autogen = Command::new("sh")
        .current_dir(&out_lib_dir)
        .arg("-c")
        .arg(format!(
            "autoreconf --force --install --verbose {}",
            &out_lib_dir
        ))
        // .arg("--force")
        // .arg("--install")
        // .arg("")
        // .arg(&lib_dir)
        .output()
        .expect("failed to run autoreconf, do you have the autotools installed?");

    // assert!(autogen.status.success(), "autogen.sh failed");

    // Build the project in the path `foo` and installs it in `$OUT_DIR`
    let dst = Config::new(&out_lib_dir)
        //.reconf("--force --install")
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
    let out_path = PathBuf::from(&out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
