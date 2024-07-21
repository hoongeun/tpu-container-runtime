use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use cpp_build;

fn main() {
    // Build the Docker image
    if !Command::new("docker")
        .args(["build", "-t", "libedgetpu-builder", "./build"])
        .status()
        .expect("Failed to start Docker build")
        .success()
    {
        panic!("Failed to build Docker image for TensorFlow Lite and libedgetpu");
    }

    // Get the output directory from the environment variable set by Cargo
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let output_path = PathBuf::from(&out_dir);

    // Create a temporary container to extract the build artifacts
    let container_id = Command::new("docker")
        .args(["create", "libedgetpu-builder"])
        .output()
        .expect("Failed to create Docker container")
        .stdout;
    let container_id = String::from_utf8_lossy(&container_id).trim().to_string();

    fs::create_dir_all(output_path.join("tensorflow").to_str().unwrap())
        .expect("Failed to create TensorFlow Lite include path");
    fs::create_dir_all(output_path.join("libedgetpu").to_str().unwrap())
        .expect("Failed to create TensorFlow Lite include path");

    let tensorflow_include_path = output_path.join("tensorflow/include");
    let tensorflow_lib_path = output_path.join("tensorflow/lib");
    let libedgetpu_include_path = output_path.join("libedgetpu/include");
    let libedgetpu_lib_path = output_path.join("libedgetpu/lib");

    Command::new("docker")
        .args([
            "cp",
            &format!("{}:/tensorflow", container_id),
            output_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to copy TensorFlow Lite lib");

    Command::new("docker")
        .args([
            "cp",
            &format!("{}:/libedgetpu", container_id),
            output_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to copy libedgetpu lib");

    // Remove the temporary container
    Command::new("docker")
        .args(["rm", &container_id])
        .status()
        .expect("Failed to remove Docker container");

    // Define include and lib paths
    let tflite_include_path = tensorflow_include_path.to_str().unwrap();
    let tflite_lib_path = tensorflow_lib_path.to_str().unwrap();
    let edgetpu_include_path = libedgetpu_include_path.to_str().unwrap();
    let edgetpu_lib_path = libedgetpu_lib_path.to_str().unwrap();

    cpp_build::Config::new()
        .include(tflite_include_path)
        .include(edgetpu_include_path)
        .flag_if_supported(format!("-L{}", tflite_lib_path).as_str())
        .flag_if_supported(format!("-L{}", edgetpu_lib_path).as_str())
        .flag_if_supported("-ltensorflowlite_c")
        .flag_if_supported("-ledgetpu")
        .flag_if_supported("-lpthread")
        .flag_if_supported("-lm")
        .flag_if_supported("-ldl")
        .build("src/driver/driver.rs");

    // Setup linking for TensorFlow Lite
    println!("cargo:rustc-link-search=native={}", tflite_lib_path);
    // Setup linking for libedgetpu
    println!("cargo:rustc-link-search=native={}", edgetpu_lib_path);

    println!("cargo:rustc-link-lib=tensorflowlite_c");
    println!("cargo:rustc-link-lib=dylib=edgetpu");
    println!("cargo:rustc-link-lib=pthread");
}
