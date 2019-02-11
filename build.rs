
use std::path::PathBuf;
use std::env;
use std::fs;
use std::process::Command;


fn main() {
    let tcc_src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tinycc");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(tcc_src_dir.join("libtcc.h").to_str().unwrap())
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=tcc");

 
    // build libtcc
    let tcc_build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()); //.join("tcc_build");
    let tcc_config_command = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tinycc").join("configure");

    let dir_builder = fs::DirBuilder::new();
    let _ = dir_builder.create(&tcc_build_dir);
    // move to tcc_build_dir
    let _ = env::set_current_dir(tcc_build_dir).unwrap();
    let _ = Command::new(tcc_config_command.to_str().unwrap()).status();
    let _ = Command::new("make").status();

}
