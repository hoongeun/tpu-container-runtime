use std::env;
use std::path::PathBuf;
use std::process::Command;
use vcpkg::find_package;

fn main() {
    println!("cargo:rerun-if-changed=src/api/wrapper.h");
    println!("cargo:rerun-if-changed=src/api/executable/executable.fbs");

    // Set the LLVM/Clang environment variables
    env::set_var("CC", "clang-14");
    env::set_var("CXX", "clang++-14");

    // Find packages using vcpkg
    let abseil = find_package("abseil").expect("Failed to find Abseil with vcpkg");
    let flatbuffers = find_package("flatbuffers").expect("Failed to find FlatBuffers with vcpkg");
    let pthread = find_package("pthread").expect("Failed to find pthread with vcpkg");

    // Collect include paths from vcpkg packages
    let mut include_paths = Vec::new();
    include_paths.extend(abseil.include_paths.iter());
    include_paths.extend(flatbuffers.include_paths.iter());
    include_paths.extend(pthread.include_paths.iter());

    // Collect library paths from vcpkg packages
    let mut library_paths = Vec::new();
    library_paths.extend(abseil.link_paths.iter());
    library_paths.extend(flatbuffers.link_paths.iter());
    library_paths.extend(pthread.link_paths.iter());

    // Generate FlatBuffers header
    Command::new("flatc")
        .args(&["--cpp", "executable.fbs"])
        .current_dir("src/api/executable")
        .status()
        .expect("Failed to generate FlatBuffers headers");

    // Generate bindings using Clang
    let bindings = bindgen::Builder::default()
        .header("src/api/wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")  // Ensure C++14 is used
        .clang_args(include_paths.iter().map(|p| format!("-I{}", p.display())))
        .clang_arg("-Isrc/api") // Include path for the api directory
        .clang_arg("-Isrc/api/port") // Include path for the port directory
        .clang_arg("-nostdinc++") // Do not use standard GCC include paths
        .clang_arg("-I/usr/lib/llvm-14/include/c++/v1") // Use Clang's libc++
        .clang_arg("--sysroot=/usr") // Specify sysroot
        .clang_arg("-D_GNU_SOURCE") // Define _GNU_SOURCE for nanosleep and other GNU extensions
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link the shared library
    let lib_dir = if cfg!(feature = "max-freq") {
        "src/lib/direct/k8"
    } else {
        "src/lib/throttled/k8"
    };

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=yourlibname");

    // Specify library paths and link libraries from vcpkg
    for lib_path in library_paths {
        println!("cargo:rustc-link-search=native={}", lib_path.display());
    }
    
    println!("cargo:rustc-link-lib=dylib=absl_base");
    println!("cargo:rustc-link-lib=dylib=absl_synchronization");
    println!("cargo:rustc-link-lib=dylib=flatbuffers");
    println!("cargo:rustc-link-lib=dylib=pthread"); // Use vcpkg pthread
}
