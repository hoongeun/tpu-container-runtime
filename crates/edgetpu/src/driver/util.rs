fn check_root_privileges() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        check_root_privileges_windows()
    } else {
        check_root_privileges_unix()
    }
}

fn check_root_privileges_windows() -> Result<(), Box<dyn std::error::Error>> {
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

fn check_root_privileges_unix() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("EUID").unwrap_or_else(|_| "0".to_string()) != "0" {
        error!("Please use sudo to run as root.");
        return Err("Insufficient privileges".into());
    }
    Ok(())
}

fn determine_platform() -> Result<(&'static str, &'static str), Box<dyn std::error::Error>> {
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
