use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let home_dir = env::var("USERPROFILE").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let debug_dir = PathBuf::from(&out_dir).join("..").join("..").join("..");
    
    // Update paths to match your Scoop installation
    let sdl2_dir = PathBuf::from(&home_dir)
        .join("scoop")
        .join("apps")
        .join("sdl2")
        .join("current");
    
    let sdl2_image_dir = PathBuf::from(&home_dir)
        .join("scoop")
        .join("apps")
        .join("sdl2-image")
        .join("current");


    // Copy DLLs to output directory
    let _ = fs::copy(
        sdl2_dir.join("lib").join("SDL2.dll"),
        debug_dir.join("SDL2.dll")
    );
    let _ = fs::copy(
        sdl2_image_dir.join("lib").join("SDL2_image.dll"),
        debug_dir.join("SDL2_image.dll")
    );

    // Link directories
    println!("cargo:rustc-link-search={}", sdl2_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", sdl2_image_dir.join("lib").display());

    // Link libraries
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
} 