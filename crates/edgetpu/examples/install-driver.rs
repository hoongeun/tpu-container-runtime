use crate::driver::download::download_edgetpu_driver;
use crate::driver::install::run_install;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_edgetpu_runtime()?;
    run_install()?;
    Ok(())
}
