use crate::driver::util::{
    check_root_privileges_unix, check_root_privileges_windows, determine_paths, determine_platform,
    get_script_dir,
};
use log::{error, info, warn};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};
use whoami;

pub fn run_install(
    runtime_path: PathBuf,
    max_freq: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (libedgetpu_dir, rules_file) = determine_paths(runtime_path)?;
    let (cpu, host_gnu_type) = determine_platform()?;
    let freq_dir = get_frequency_dir(max_freq);

    if env::consts::OS == "macos" {
        check_root_privileges_unix()?;
        install_macos_dependencies(&libedgetpu_dir, &freq_dir, &cpu)?;
    } else if env::consts::OS == "linux" {
        check_root_privileges_unix()?;
        install_linux_dependencies(
            &libedgetpu_dir,
            &rules_file,
            &freq_dir,
            &cpu,
            &host_gnu_type,
        )?;
    } else if env::consts::OS == "windows" {
        check_root_privileges_windows()?;
        install_windows_dependencies(&libedgetpu_dir, &freq_dir)?;
    } else {
        error!("Unsupported operating system.");
        return Err("Unsupported platform".into());
    }

    Ok(())
}

fn get_frequency_dir(max_freq: bool) -> &'static str {
    if max_freq {
        info!("Using the maximum operating frequency for Coral USB devices.");
        "direct"
    } else {
        info!("Using the reduced operating frequency for Coral USB devices.");
        "throttled"
    }
}

fn install_windows_dependencies(
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
    fs::copy(
        root_dir.join(format!("libedgetpu/{}/x64_windows/edgetpu.dll", freq_dir)),
        PathBuf::from("C:\\Windows\\System32\\edgetpu.dll"),
    )?;
    fs::copy(
        root_dir.join("third_party/libusb_win/libusb-1.0.dll"),
        PathBuf::from("C:\\Windows\\System32\\libusb-1.0.dll"),
    )?;

    info!("Install complete!");

    Ok(())
}

fn install_macos_dependencies(
    libedgetpu_dir: &PathBuf,
    freq_dir: &str,
    cpu: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let darwin_install_command = if Command::new("port").output().is_ok() {
        "port".to_string()
    } else if let Ok(brew_path) = Command::new("brew").output() {
        String::from_utf8_lossy(&brew_path.stdout)
            .trim()
            .to_string()
    } else {
        error!("You need to install either Homebrew or MacPorts first.");
        return Err("Homebrew or MacPorts not found".into());
    };

    let darwin_install_user = env::var("SUDO_USER").unwrap_or_else(|_| whoami::username());
    Command::new("sudo")
        .arg("-u")
        .arg(&darwin_install_user)
        .arg(&darwin_install_command)
        .arg("install")
        .arg("libusb")
        .status()?;

    let darwin_install_lib_dir = PathBuf::from(&darwin_install_command)
        .parent()
        .unwrap()
        .join("lib");
    let libedgetpu_lib_dir = PathBuf::from("/usr/local/lib");
    fs::create_dir_all(&libedgetpu_lib_dir)?;

    install_file(
        "Edge TPU runtime library",
        &libedgetpu_dir
            .join(freq_dir)
            .join(cpu)
            .join("libedgetpu.1.0.dylib"),
        &libedgetpu_lib_dir.join("libedgetpu.1.0.dylib"),
    )?;

    info!(
        "Generating symlink [{}]...",
        libedgetpu_lib_dir.join("libedgetpu.1.dylib").display()
    );
    std::os::unix::fs::symlink(
        "libedgetpu.1.0.dylib",
        libedgetpu_lib_dir.join("libedgetpu.1.dylib"),
    )?;

    Command::new("install_name_tool")
        .arg("-id")
        .arg(
            &libedgetpu_lib_dir
                .join("libedgetpu.1.dylib")
                .to_string_lossy()
                .to_string(),
        )
        .arg(
            &libedgetpu_lib_dir
                .join("libedgetpu.1.0.dylib")
                .to_string_lossy()
                .to_string(),
        )
        .status()?;

    let otool_output = Command::new("otool")
        .arg("-L")
        .arg(
            &libedgetpu_lib_dir
                .join("libedgetpu.1.0.dylib")
                .to_string_lossy()
                .to_string(),
        )
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
        .arg(
            &darwin_install_lib_dir
                .join("libusb-1.0.0.dylib")
                .to_string_lossy()
                .to_string(),
        )
        .arg(
            &libedgetpu_lib_dir
                .join("libedgetpu.1.0.dylib")
                .to_string_lossy()
                .to_string(),
        )
        .status()?;

    Ok(())
}

fn install_linux_dependencies(
    libedgetpu_dir: &PathBuf,
    rules_file: &PathBuf,
    freq_dir: &str,
    cpu: &str,
    host_gnu_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let packages = ["libc6", "libgcc1", "libstdc++6", "libusb-1.0-0"];
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

    if Command::new("udevadm").output().is_ok() {
        install_file(
            "device rule file",
            &rules_file,
            &PathBuf::from("/etc/udev/rules.d/99-edgetpu-accelerator.rules"),
        )?;
        Command::new("udevadm")
            .arg("control")
            .arg("--reload-rules")
            .status()?;
        Command::new("udevadm").arg("trigger").status()?;
        info!("Done.");
    }

    install_file(
        "Edge TPU runtime library",
        &libedgetpu_dir
            .join(freq_dir)
            .join(cpu)
            .join("libedgetpu.so.1.0"),
        &PathBuf::from(format!("/usr/lib/{}/libedgetpu.so.1.0", host_gnu_type)),
    )?;
    Command::new("ldconfig").status()?;
    info!("Done.");

    Ok(())
}

fn install_file(
    name: &str,
    src: &PathBuf,
    dst: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Installing {} [{}]...", name, dst.display());
    if dst.exists() {
        warn!("File already exists. Replacing it...");
        fs::remove_file(&dst)?;
    }
    fs::copy(&src, &dst)?;
    Ok(())
}
