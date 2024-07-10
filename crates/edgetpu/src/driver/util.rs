use log::error;
use std::env;
use std::path::PathBuf;
use std::process::{self, Command};

pub fn check_root_privileges_windows() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("fsutil")
        .arg("dirty")
        .arg("query")
        .arg("%systemdrive%")
        .output()?;
    if !output.status.success() {
        Command::new("powershell")
            .arg("Start-Process")
            .arg("-FilePath")
            .arg(env::current_exe()?)
            .arg("-ArgumentList")
            .arg("elevated")
            .arg("-Verb")
            .arg("runas")
            .status()?;
        process::exit(0);
    }
    Ok(())
}

pub fn check_root_privileges_unix() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("EUID").unwrap_or_else(|_| "0".to_string()) != "0" {
        error!("Please use sudo to run as root.");
        return Err("Insufficient privileges".into());
    }
    Ok(())
}

pub fn determine_platform() -> Result<(&'static str, &'static str), Box<dyn std::error::Error>> {
    let os = env::consts::OS;
    let machine = env::consts::ARCH;
    match (os, machine) {
        ("linux", "x86_64") => Ok(("k8", "x86_64-linux-gnu")),
        ("linux", "arm") => Ok(("armv7a", "arm-linux-gnueabihf")),
        ("linux", "aarch64") => Ok(("aarch64", "aarch64-linux-gnu")),
        ("macos", "x86_64") => Ok(("darwin_x86_64", "")),
        ("macos", "aarch64") => Ok(("darwin_arm64", "")),
        ("windows", "x86_64") => Ok(("x64_windows", "")),
        _ => {
            error!("Your platform is not supported.");
            Err("Unsupported platform".into())
        }
    }
}

pub fn get_script_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(env::current_exe()?.parent().unwrap().to_path_buf())
}

pub fn determine_linux_platform() -> Result<(String, String), Box<dyn std::error::Error>> {
    let os = env::consts::OS;
    let machine = env::consts::ARCH;
    match (os, machine) {
        ("linux", "x86_64") => Ok(("k8".to_string(), "x86_64-linux-gnu".to_string())),
        ("linux", "arm") => Ok(("armv7a".to_string(), "arm-linux-gnueabihf".to_string())),
        ("linux", "aarch64") => Ok(("aarch64".to_string(), "aarch64-linux-gnu".to_string())),
        _ => {
            error!("Your Linux platform is not supported. There's nothing to uninstall.");
            Err("Unsupported platform".into())
        }
    }
}

pub fn determine_paths(
    script_dir: &PathBuf,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    if script_dir.join("libedgetpu").exists() {
        Ok((
            script_dir.join("libedgetpu"),
            script_dir.join("libedgetpu/edgetpu-accelerator.rules"),
        ))
    } else {
        Ok((
            PathBuf::from(
                env::var("LIBEDGETPU_BIN")
                    .unwrap_or_else(|_| script_dir.join("../out").to_string_lossy().to_string()),
            ),
            script_dir.join("../debian/edgetpu-accelerator.rules"),
        ))
    }
}
