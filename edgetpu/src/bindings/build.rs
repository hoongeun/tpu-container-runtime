extern crate bindgen;
extern crate curl;
extern crate zip;

use curl::easy::Easy;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use zip::ZipArchive;

fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    easy.url(url)?;
    let mut file = File::create(dest)?;
    easy.write_function(move |data| {
        file.write_all(data)?;
        Ok(data.len())
    })?;
    easy.perform()?;
    Ok(())
}

fn extract_zip(zip_path: &PathBuf, extract_to: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(extract_to)?;
    Ok(())
}

fn download_libedgetpu(out_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let url = "https://github.com/google-coral/libedgetpu/releases/download/release-grouper/edgetpu_runtime_20221024.zip";
    let zip_path = out_path.join("edgetpu_runtime.zip");
    let edgetpu_path = out_path.join("edgetpu_runtime");

    if !zip_path.exists() {
        download_file(url, &zip_path)?;
    }

    if !edgetpu_path.exists() {
        extract_zip(&zip_path, &out_path)?;
    }

    Ok(edgetpu_path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    let edgetpu_path = download_libedgetpu(&out_path)?;

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
