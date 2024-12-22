use std::{env, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let create_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let open_vr_sdk_path = PathBuf::from("D:/Rs/reina_s_kawaii_move_assist/openvr"); //TODO : これの解決法が ... わからない `../` を用いたやつじゃどうにもならなかったや

    bindgen::builder()
        .header("../openvr/headers/openvr_driver.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .allowlist_type("vr::.*")
        .allowlist_function("vr::.*")
        .allowlist_recursively(true)
        .blocklist_type("std::.*")
        .blocklist_function("std::.*")
        .derive_default(true)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("openvr_driver_bindings.rs"))
        .unwrap();

    bindgen::builder()
        .header("cpp/bindings.h")
        .clang_arg("-xc++")
        .derive_default(true)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("cpp_bindings.rs"))
        .unwrap();

    let mut build_cpp = cc::Build::new();
    build_cpp
        .cpp(true)
        .file("./cpp/KMADriver.cpp") // TODO : 今は数が少ないからいいけど ... 適当に増やせるような仕組みをしたと合わせて何とかしたいところ。
        .file("./cpp/OVR-SC/Hooking.cpp")
        .file("./cpp/OVR-SC/InterfaceHookInjector.cpp")
        .flag_if_supported("-std=c++17")
        .flag("/std:c++17")
        .flag("/permissive-")
        .include(open_vr_sdk_path.join("headers"))
        .include("./cpp")
        .include("./cpp/OVR-SC")
        .include("./cpp/MinHook/include")
        .include("./cpp/ALVR")
        .debug(false);
    let out_cpp_driver_name = "kma_cpp_native_driver";
    build_cpp.compile(out_cpp_driver_name);

    println!(
        "cargo:rustc-link-search=native={}",
        create_path.join("cpp/MinHook/bin").to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=MinHook.x64");

    println!(
        "cargo:rustc-link-search=native={}",
        open_vr_sdk_path.join("lib/win64").to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=openvr_api");
}
