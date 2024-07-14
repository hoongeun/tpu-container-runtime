use crate::dep::util::{check_privileges, install_path_of};
use log::info;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn run_uninstall() -> Result<(), Box<dyn std::error::Error>> {
    if !check_privileges() {
        return Err("Root privileges are required to uninstall the Edge TPU driver.".into());
    }

    uninstall_dependencies()
}

#[cfg(target_os = "windows")]
pub fn uninstall_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let libedgetpu_dir = install_path_of();
    let root_dir = if libedgetpu_dir.exists() {
        libedgetpu_dir.clone()
    } else {
        libedgetpu_dir.join("..")
    };

    info!("Deleting edgetpu and libusb from System32");
    fs::remove_file(libedgetpu_dir.join("edgetpu.dll"))?;
    fs::remove_file(libedgetpu_dir.join("libusb-1.0.dll"))?;

    info!("Uninstalling WinUSB drivers");
    let output = Command::new("pnputil")
        .arg("/enum-devices")
        .arg("/class")
        .arg("{88bae032-5a81-49f0-bc3d-a4ff138216d6}")
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let infs: Vec<&str> = output_str
        .lines()
        .filter_map(|line| line.strip_prefix("Driver Name:"))
        .map(|line| line.trim())
        .collect();
    for inf in infs {
        info!("Uninstalling driver: {}", inf);
        Command::new("pnputil")
            .arg("/delete-driver")
            .arg(inf)
            .arg("/uninstall")
            .status()?;
    }

    info!("Uninstalling UsbDk");
    Command::new("msiexec")
        .arg("/x")
        .arg(root_dir.join("third_party/usbdk/UsbDk_1.0.22_x64.msi"))
        .arg("/quiet")
        .arg("/qb!")
        .arg("/norestart")
        .status()?;

    info!("Uninstall complete!");

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn uninstall_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let libedgetpu_lib_dir = install_path_of();

    if libedgetpu_lib_dir.join("libedgetpu.1.0.dylib").exists() {
        info!("Uninstalling Edge TPU runtime library...");
        fs::remove_file(libedgetpu_lib_dir.join("libedgetpu.1.0.dylib"))?;
        info!("Done");
    }

    if libedgetpu_lib_dir.join("libedgetpu.1.dylib").exists() {
        info!("Uninstalling Edge TPU runtime library symlink...");
        fs::remove_file(libedgetpu_lib_dir.join("libedgetpu.1.dylib"))?;
    }

    info!("Uninstall complete!");

    Ok(())
}

#[cfg(target_os = "linux")]
fn uninstall_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    if !Command::new("udevadm").output().is_ok() {
        return Err("udevadm not found.".into());
    }

    let rules_file = PathBuf::from("/etc/udev/rules.d/99-edgetpu-accelerator.rules");
    if rules_file.exists() {
        info!("Registering edgetpu device driver...");
        fs::remove_file(&rules_file)?;
        Command::new("udevadm")
            .arg("control")
            .arg("--reload-rules")
            .status()?;
        Command::new("udevadm").arg("trigger").status()?;
    }

    let libedgetpu_dst = install_path_of().join("libedgetpu.so.1.0");
    if libedgetpu_dst.exists() {
        info!("Uninstalling Edge TPU runtime library ...");
        fs::remove_file(&libedgetpu_dst)?;
        Command::new("ldconfig").status()?;
    }

    info!("Uninstall complete!");

    Ok(())
}
