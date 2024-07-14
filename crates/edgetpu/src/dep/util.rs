use std::path::PathBuf;
use env_logger::Env;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use nix::unistd::Uid;

#[cfg(target_os = "windows")]
extern crate winapi;

pub fn init_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn check_privileges() -> bool {
    Uid::effective().is_root()
}

#[cfg(target_os = "windows")]
pub fn check_privileges() -> bool {
    use std::ptr;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::securitybaseapi;
    use winapi::um::winbase::GetCurrentProcess;
    use winapi::um::winnt::{TokenElevation, TokenElevationType, TOKEN_QUERY};

    unsafe {
        let mut handle = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) != 0 {
            let mut elevation = TokenElevation { TokenIsElevated: 0 };
            let mut size = std::mem::size_of::<TokenElevation>() as u32;
            if securitybaseapi::GetTokenInformation(
                handle,
                TokenElevationType,
                &mut elevation as *mut _ as *mut _,
                size,
                &mut size,
            ) != 0
            {
                CloseHandle(handle);
                return elevation.TokenIsElevated != 0;
            }
            CloseHandle(handle);
        }
    }
    false
}

#[cfg(target_os = "windows")]
pub fn install_path_of() -> PathBuf {
    PathBuf::from("C:\\Windows\\System32")
}

#[cfg(target_os = "macos")]
pub fn install_path_of() -> PathBuf {
    PathBuf::from("/usr/local/lib")
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub fn install_path_of() -> PathBuf {
    PathBuf::from("/usr/lib").join("x86_64-linux-gnu")
}

#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub fn install_path_of() -> PathBuf {
    PathBuf::from("/usr/lib").join("arm-linux-gnueabihf")
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub fn install_path_of() -> PathBuf {
    PathBuf::from("/usr/lib").join("aarch64-linux-gnu")
}
