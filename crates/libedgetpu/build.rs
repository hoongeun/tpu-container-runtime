use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashSet;
use vcpkg::find_package;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/headers/wrapper.h");
    println!("cargo:rerun-if-changed=src/headers/api/driver_options.fbs");
    println!("cargo:rerun-if-changed=src/headers/executable/executable.fbs");

    env::set_var("CC", "clang");
    env::set_var("CXX", "clang++");

    let abseil = find_package("abseil").expect("Failed to find Abseil with vcpkg");
    let flatbuffers = find_package("flatbuffers").expect("Failed to find FlatBuffers with vcpkg");
    let pthread = find_package("pthread").expect("Failed to find pthread with vcpkg");

    let mut include_paths = Vec::new();
    include_paths.extend(abseil.include_paths.iter());
    include_paths.extend(flatbuffers.include_paths.iter());
    include_paths.extend(pthread.include_paths.iter());

    Command::new("flatc")
        .args(&["--cpp", "driver_options.fbs"])
        .current_dir("src/headers/api")
        .status()
        .expect("Failed to generate FlatBuffers headers");

    Command::new("flatc")
        .args(&[
            "--cpp",
            "executable.fbs",
            "--gen-object-api",
            "--force-empty",
            "--gen-mutable",
        ])
        .current_dir("src/headers/executable")
        .status()
        .expect("Failed to generate FlatBuffers headers");

    let ignored_macros = IgnoreMacros(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into(),
                "FP_INT_UPWARD".into(),
                "FP_INT_DOWNWARD".into(),
                "FP_INT_TONEAREST".into(),
                "FP_INT_TOWARDZERO".into(),
                "FP_INT_TONEARESTFROMZERO".into(),
            ]
            .into_iter()
            .collect(),
        );

    let bindings = bindgen::Builder::default()
        .header("src/headers/wrapper.h")
        .parse_callbacks(Box::new(ignored_macros))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-stdlib=libc++")
        .clang_args(include_paths.iter().map(|p| format!("-I{}", p.display())))
        .clang_arg("-Isrc/headers")
        .blocklist_type("rep")
        .blocklist_type("char_type")
        .allowlist_type("_Tp")  // Add this to blocklist template types
        .allowlist_type("_Pred")
        .allowlist_type("_Type")
        .allowlist_type("_ValueType")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=dylib=absl_base");
    println!("cargo:rustc-link-lib=dylib=absl_synchronization");
    println!("cargo:rustc-link-lib=dylib=flatbuffers");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=edgetpu");
}
