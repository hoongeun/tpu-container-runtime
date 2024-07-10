use curl::easy::Easy;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use zip::ZipArchive;

pub fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn extract_zip(
    zip_path: &PathBuf,
    extract_to: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(extract_to)?;
    Ok(())
}

pub fn download_libedgetpu(out_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
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
