extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../include/vulkan/vulkan.h");
    println!("cargo:rerun-if-changed=../../include/vulkan/vk_icd.h");

    let out_path = PathBuf::from(::std::env::var("OUT_DIR").unwrap());

    let bindings = bindgen::builder()
        .header("../../include/vulkan/vk_icd.h")
        .clang_arg("-I../../include/vulkan")
        .clang_arg("-DVK_NO_PROTOTYPES")
        .generate().unwrap();

    bindings.write_to_file(out_path.join("vk_icd.rs")).unwrap();
}
