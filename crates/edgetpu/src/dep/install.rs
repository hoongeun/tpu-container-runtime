use crate::dep::util::{check_privileges, install_path_of};
use log::{info, warn};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[cfg(any(target_os = "macos"))]
use nix::unistd::Uid;

#[cfg(any(target_os = "macos"))]
use nix::unistd::User;

#[cfg(any(target_os = "macos"))]
use nix::sys::utsname::uname;

pub fn run_install(runtime_dir: PathBuf, max_freq: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !check_privileges() {
        return Err("Root privileges are required to install the Edge TPU driver.".into());
    }

    let freq_dir = if max_freq {
        info!("Using the maximum operating frequency(500Mhz) for Coral USB devices.");
        "direct"
    } else {
        info!("Using the reduced operating frequency(200Mhz) for Coral USB devices.");
        "throttled"
    };

    install_dependencies(&runtime_dir, freq_dir)
}

#[cfg(target_os = "windows")]
fn install_dependencies(
    libedgetpu_dir: &PathBuf,
    freq_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = if libedgetpu_dir.exists() {
        libedgetpu_dir.clone()
    } else {
        libedgetpu_dir.join("..")
    };

    info!("Installing UsbDk");
    Command::new("msiexec")
        .arg("/i")
        .arg(root_dir.join("third_party/usbdk/UsbDk_1.0.22_x64.msi"))
        .arg("/qb!")
        .arg("/norestart")
        .status()?;

    info!("Installing Windows drivers");
    Command::new("pnputil")
        .arg("/add-driver")
        .arg(root_dir.join("third_party/coral_accelerator_windows/*.inf"))
        .arg("/install")
        .status()?;

    info!("Installing performance counters");
    Command::new("lodctr")
        .arg("/M:")
        .arg(root_dir.join("third_party/coral_accelerator_windows/coral.man"))
        .status()?;

    info!("Copying edgetpu and libusb to System32");
    overwrite(
        &edgetpu_path_of(libedgetpu_dir, freq_dir).join("x64_windows").join("edgetpu.dll"),
        &install_path_of().join("edgetpu.dll"),
    )?;
    overwrite(
        &edgetpu_path_of(libedgetpu_dir, freq_dir).join("libusb_win").join("libusb-1.0.dll"),
        &install_path_of().join("libusb-1.0.dll"),
    )?;

    info!("Install complete!");

    Ok(())
}

#[cfg(target_os = "macos")]
fn install_dependencies(
    libedgetpu_dir: &PathBuf,
    freq_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let install_cmd = get_install_command();

    let user = env::var("SUDO_USER").unwrap_or_else(|_| {
        User::from_uid(Uid::effective())
            .unwrap()
            .unwrap()
            .name
    });

    Command::new("sudo")
        .arg("-u")
        .arg(&user)
        .arg(&install_cmd)
        .arg("install")
        .arg("libusb")
        .status()?;

    let darwin_install_lib_dir = PathBuf::from(&install_cmd).parent().unwrap().join("lib");
    fs::create_dir_all(&install_path_of())?;

    let uname_info = uname();
    let arch = uname_info.machine();

    overwrite(
        &edgetpu_path_of(libedgetpu_dir, freq_dir).join(arch).join("libedgetpu.1.0.dylib"),
        install_path_of().join("libedgetpu.1.0.dylib"),
    )?;

    info!(
        "Generating symlink [{}]...",
        install_path_of().join("libedgetpu.1.dylib").display()
    );
    std::os::unix::fs::symlink(
        "libedgetpu.1.0.dylib",
        install_path_of().join("libedgetpu.1.dylib"),
    )?;

    Command::new("install_name_tool")
        .arg("-id")
        .arg(&install_path_of().join("libedgetpu.1.dylib").to_string())
        .arg(&install_path_of().join("libedgetpu.1.0.dylib").to_string())
        .status()?;

    let otool_output = Command::new("otool")
        .arg("-L")
        .arg(&install_path_of().join("libedgetpu.1.0.dylib").to_string())
        .output()?
        .stdout;

    let otool_output_str = String::from_utf8_lossy(&otool_output);
    let dependency = otool_output_str
        .lines()
        .find(|line| line.contains("libusb"))
        .and_then(|line| line.split_whitespace().next())
        .ok_or("libusb dependency not found")?;

    Command::new("install_name_tool")
        .arg("-change")
        .arg(dependency)
        .arg(&install_path_of().join("libusb-1.0.0.dylib").to_string())
        .arg(&install_path_of().join("libedgetpu.1.0.dylib").to_string())
        .status()?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn get_install_command() -> Result<String, Box<dyn std::error::Error>> {
    if Command::new("port").output().is_ok() {
        return Ok("port".to_string());
    }

    if Command::new("brew").output().is_ok() {
        return Ok("brew".to_string());
    }

    error!("You need to install either Homebrew or MacPorts first.");
    Err("Homebrew or MacPorts not found".into())
}

#[cfg(target_os = "linux")]
fn install_dependencies(
    runtime_dir: &PathBuf,
    freq_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let packages = ["libc6", "libgcc1", "libstdc++6", "libusb-1.0-0"];
    
    info!("Checking library dependencies...");
    let missing_packages: Vec<&str> = packages
        .iter()
        .filter(|&&pkg| Command::new("dpkg").arg("-l").arg(pkg).output().is_err())
        .copied()
        .collect();

    if !missing_packages.is_empty() {
        info!("Installing library dependencies: {:?}...", missing_packages);
        Command::new("apt-get").arg("update").status()?;
        Command::new("apt-get")
            .arg("install")
            .arg("-y")
            .args(&missing_packages)
            .status()?;
        info!("Done.");
    }

    if !Command::new("udevadm").output().is_ok() {
        return Err("udevadm not found".into());
    }

    info!("Registering edgetpu device driver...");
    let rules_file = runtime_dir.join("libedgetpu").join("edgetpu-accelerator.rules");
    overwrite(
        &rules_file,
        &PathBuf::from("/etc/udev/rules.d/99-edgetpu-accelerator.rules"),
    )?;
    Command::new("udevadm")
        .arg("control")
        .arg("--reload-rules")
        .status()?;
    Command::new("udevadm").arg("trigger").status()?;

    info!("Installing Edge TPU runtime library...");
    overwrite(
        &edgetpu_path_of(&runtime_dir.join("libedgetpu"), freq_dir).join("libedgetpu.so.1.0"),
        &install_path_of().join("libedgetpu.so.1.0"),
    )?;
    Command::new("ldconfig").status()?;
    
    info!("Install complete!");
    Ok(())
}

fn overwrite(src: &PathBuf, dst: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if dst.exists() {
        warn!("File already exists. Replacing it...");
        fs::remove_file(&dst)?;
    }
    fs::copy(&src, &dst)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn edgetpu_path_of(lib_root_path: &PathBuf, freq_dir: &str) -> PathBuf {
    lib_root_path.join(freq_dir).join("x64_windows").join("edgetpu.dll")
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn edgetpu_path_of(lib_root_path: &PathBuf, freq_dir: &str) -> PathBuf {
    lib_root_path
        .join(freq_dir)
        .join("x86_64-linux-gnu")
        .join("libedgetpu.so.1.0")
}

#[cfg(all(target_os = "macos", target_arch = "arm"))]
fn edgetpu_path_of(lib_root_path: &PathBuf, freq_dir: &str) -> PathBuf {
    lib_root_path
        .join(freq_dir)
        .join("x86_64-linux-gnu")
        .join("libedgetpu.so.1.0")
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn edgetpu_path_of(lib_root_path: &PathBuf, freq_dir: &str) -> PathBuf {
    lib_root_path.join(freq_dir).join("k8")
}

#[cfg(all(target_os = "linux", target_arch = "armv7a"))]
fn edgetpu_path_of(lib_root_path: &PathBuf, freq_dir: &str) -> PathBuf {
    lib_root_path.join(freq_dir).join("arm7a")
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn edgetpu_path_of(lib_root_path: &PathBuf, freq_dir: &str) -> PathBuf {
    lib_root_path.join(freq_dir).join("aarch64")
}
