use clap::{arg, ArgMatches, Command};
use edgetpu::dep::download::download_edgetpu_runtime;
use edgetpu::dep::install::run_install;
use edgetpu::dep::util::init_logger;
use log::info;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    // Parse command-line arguments
    let args = get_arguments();

    // Define download path
    let download_path = get_download_path()?;

    // Execute main logic and handle cleanup
    let result = execute_installation(&download_path, &args);

    // Perform cleanup
    cleanup(&download_path);

    // Return the result of the main logic
    result
}

fn get_arguments() -> ArgMatches {
    Command::new("Edge TPU Installer")
        .version("1.0")
        .author("Hoongeun Cho <me@hoongeun.com>")
        .about("Installs the Edge TPU runtime")
        .arg(
            arg!(
                -f --freq <FREQ> "Sets the operating frequency of the Coral USB device"
            )
            .required(false)
            .value_parser(["normal", "max"])
            .default_value("normal"),
        )
        .get_matches()
}

fn get_download_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join("downloads"))
}

fn execute_installation(
    download_path: &PathBuf,
    args: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let runtime_dir = download_edgetpu_runtime(download_path)?;
    let freq = args.get_one::<String>("freq").unwrap();

    run_install(runtime_dir, freq == "max")
}

fn cleanup(download_path: &PathBuf) {
    if download_path.exists() {
        info!("Cleaning up downloaded files...");
        if let Err(e) = fs::remove_dir_all(download_path) {
            eprintln!("Failed to clean up downloaded files: {}", e);
        } else {
            info!("Downloaded files cleaned up successfully.");
        }
    }
}
