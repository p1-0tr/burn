//#[cfg(all(feature = "dawn", not(target_os = "macos")))]
//compile_error!("The 'dawn' backend currently only builds on macos.");

fn main() {
    #[cfg(feature = "dawn")]
    link_and_bind_dawn();
}

#[cfg(feature = "dawn")]
fn link_and_bind_dawn() {
    use bindgen::builder;
    use std::path::PathBuf;
    use std::env;

    println!("cargo:rustc-link-lib=dylib=webgpudd");

    let bindings = builder()
        .header("webgpu.h")
        .clang_args([
            "-x",
            "c++",
            "--std=c++17",
        ])
        .allowlist_function(".*GetProcs.*")
        .allowlist_function(".*SetProcs.*")
        .allowlist_function("wgpu.*")
        .allowlist_file(".*webgpu.h")
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate Dawn bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dawn_native_bindings_gen.rs"))
        .expect("Couldn't write Dawn bindings!");
}
