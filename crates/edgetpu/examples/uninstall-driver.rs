use crate::driver::download::download_edgetpu_runtime;
use crate::driver::uninstall::run_uninstall;
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let download_path = PathBuf::from("downloads");
    let runtime_path = download_edgetpu_runtime(download_path)?;
    run_uninstall(runtime_path)?;
    Ok(())
}
