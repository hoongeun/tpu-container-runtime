use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let version = edgetpu_get_runtime_version();
        println!(
            "Edge TPU runtime version: {}",
            CStr::from_ptr(version).to_str().unwrap()
        );
    }
}
