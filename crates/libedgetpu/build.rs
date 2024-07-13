use std::env;
use std::path::PathBuf;
use std::process::Command;
use vcpkg::find_package;

fn main() {
    println!("cargo:rerun-if-changed=src/headers/wrapper.h");
    println!("cargo:rerun-if-changed=src/headers/executable/executable.fbs");

    // Set the LLVM/Clang environment variables
    env::set_var("CC", "clang-17");
    env::set_var("CXX", "clang++-17");
    // env::set_var("CXXFLAGS", "-stdlib=libc++");

    // Find packages using vcpkg
    let abseil = find_package("abseil").expect("Failed to find Abseil with vcpkg");
    let flatbuffers = find_package("flatbuffers").expect("Failed to find FlatBuffers with vcpkg");
    let pthread = find_package("pthread").expect("Failed to find pthread with vcpkg");

    // Collect include paths from vcpkg packages
    let mut include_paths = Vec::new();
    include_paths.extend(abseil.include_paths.iter());
    include_paths.extend(flatbuffers.include_paths.iter());
    include_paths.extend(pthread.include_paths.iter());

    Command::new("flatc")
        .args(&["--cpp", "driver_options.fbs"])
        .current_dir("src/headers/api")
        .status()
        .expect("Failed to generate FlatBuffers headers");

    // Generate FlatBuffers header
    Command::new("flatc")
        .args(&["--cpp", "executable.fbs", "--gen-object-api", "--force-empty", "--gen-mutable"])
        .current_dir("src/headers/executable")
        .status()
        .expect("Failed to generate FlatBuffers headers");

    // Generate bindings using Clang
    let bindings = bindgen::Builder::default()
        .header("src/headers/wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-stdlib=libc++")
        .clang_args(include_paths.iter().map(|p| format!("-I{}", p.display())))
        .clang_arg("-Isrc/headers")
        // .clang_arg("-I/usr/include/c++/11")
        // .clang_arg("-I/usr/include/x86_64-linux-gnu/c++/11")
        // .clang_arg("-I/usr/lib/llvm-14/include/c++/v1")
        // .clang_arg("-I/usr/local/include")
        // .clang_arg("-I/usr/lib/llvm-14/lib/clang/14.0.0/include")
        // .clang_arg("-I/usr/include")
        // .clang_arg("-I/usr/include/x86_64-linux-gnu")
        // .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/11/include")
        // .clang_arg("-D__STDC_WANT_LIB_EXT2__=1")
        // .clang_arg("-D__STDC_LIMIT_MACROS")
        // .clang_arg("-D__STDC_CONSTANT_MACROS")
        // .clang_arg("-D_GNU_SOURCE")
        
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

    // Specify library paths and link libraries from vcpkg
    for lib_path in pthread.link_paths.iter().chain(abseil.link_paths.iter()).chain(flatbuffers.link_paths.iter()) {
        println!("cargo:rustc-link-search=native={}", lib_path.display());
    }
    
    // Link against required libraries
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=dylib=absl_base");
    println!("cargo:rustc-link-lib=dylib=absl_synchronization");
    println!("cargo:rustc-link-lib=dylib=flatbuffers");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=yourlibname");
}

