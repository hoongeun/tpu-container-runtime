extern crate bindgen;

use curl::easy::Easy;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use zip::ZipArchive;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    let edgetpu_path = download_libedgetpu(&out_path)?;

    run_install_script(&edgetpu_path)?;

    println!("cargo:rustc-link-lib=static=edgetpu");
    let var_name = println!(
        "cargo:rustc-link-search=native={}",
        edgetpu_path.join("libedgetpu").display()
    );

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(edgetpu_path.join("libedgetpu/edgetpu.h").to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()?;

    // Write the bindings to the $OUT_DIR/bindings.rs file
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
