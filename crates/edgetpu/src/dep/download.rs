use reqwest::blocking::Client;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use zip::ZipArchive;

pub fn download_edgetpu_runtime(out_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    create_dir_all(out_path)?;

    let url = "https://github.com/google-coral/libedgetpu/releases/download/release-grouper/edgetpu_runtime_20221024.zip";
    let zip_path = out_path.join("edgetpu_runtime.zip");
    let edgetpu_path = out_path.join("edgetpu_runtime");

    if !zip_path.exists() {
        download_file(url, &zip_path)?;

        if zip_path.metadata()?.len() == 0 {
            return Err("Download failed: ZIP file is empty".into());
        }
    }

    if !edgetpu_path.exists() {
        extract_zip(&zip_path, out_path)?;
    }

    Ok(edgetpu_path)
}

pub fn download_file(url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    if response.status().is_success() {
        let mut file = File::create(dest)?;
        let content = response.bytes()?;
        file.write_all(&content)?;
    } else {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }
    Ok(())
}

pub fn extract_zip(zip_path: &PathBuf, extract_to: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(extract_to)?;
    Ok(())
}
