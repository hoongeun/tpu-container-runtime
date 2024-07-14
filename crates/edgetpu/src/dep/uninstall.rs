use crate::driver::util::{
    check_root_privileges_unix, check_root_privileges_windows, determine_linux_platform,
    determine_paths, get_script_dir,
};
use log::{error, info, warn};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

pub fn run_uninstall(runtime_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (libedgetpu_dir, _) = determine_paths(runtime_path)?;

    if env::consts::OS == "macos" {
        check_root_privileges_unix()?;
        uninstall_macos_dependencies()?;
    } else if env::consts::OS == "linux" {
        check_root_privileges_unix()?;
        let (cpu_dir, host_gnu_type) = determine_linux_platform()?;
        uninstall_linux_dependencies(&cpu_dir, &host_gnu_type)?;
    } else if env::consts::OS == "windows" {
        check_root_privileges_windows()?;
        uninstall_windows_dependencies(&libedgetpu_dir)?;
    } else {
        error!("Unsupported operating system.");
        return Err("Unsupported platform".into());
    }

    Ok(())
}

pub fn uninstall_windows_dependencies(
    libedgetpu_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = if libedgetpu_dir.exists() {
        libedgetpu_dir.clone()
    } else {
        libedgetpu_dir.join("..")
    };

    info!("Deleting edgetpu and libusb from System32");
    fs::remove_file("C:\\Windows\\System32\\edgetpu.dll")?;
    fs::remove_file("C:\\Windows\\System32\\libusb-1.0.dll")?;

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

pub fn uninstall_macos_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let libedgetpu_lib_dir = PathBuf::from("/usr/local/lib");

    if libedgetpu_lib_dir.join("libedgetpu.1.0.dylib").exists() {
        info!("Uninstalling Edge TPU runtime library...");
        fs::remove_file(libedgetpu_lib_dir.join("libedgetpu.1.0.dylib"))?;
        info!("Done");
    }

    if libedgetpu_lib_dir.join("libedgetpu.1.dylib").exists() {
        info!("Uninstalling Edge TPU runtime library symlink...");
        fs::remove_file(libedgetpu_lib_dir.join("libedgetpu.1.dylib"))?;
        info!("Done");
    }

    Ok(())
}

fn uninstall_linux_dependencies(
    cpu_dir: &str,
    host_gnu_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if Command::new("udevadm").output().is_ok() {
        let udev_rule_path = PathBuf::from("/etc/udev/rules.d/99-edgetpu-accelerator.rules");
        if udev_rule_path.exists() {
            info!(
                "Uninstalling device rule file [{}]...",
                udev_rule_path.display()
            );
            fs::remove_file(&udev_rule_path)?;
            Command::new("udevadm")
                .arg("control")
                .arg("--reload-rules")
                .status()?;
            Command::new("udevadm").arg("trigger").status()?;
            info!("Done.");
        }
    }

    let libedgetpu_dst = PathBuf::from(format!("/usr/lib/{}/libedgetpu.so.1.0", host_gnu_type));
    if libedgetpu_dst.exists() {
        info!(
            "Uninstalling Edge TPU runtime library [{}]...",
            libedgetpu_dst.display()
        );
        fs::remove_file(&libedgetpu_dst)?;
        Command::new("ldconfig").status()?;
        info!("Done.");
    }

    Ok(())
}
