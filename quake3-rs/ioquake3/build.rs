#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");

    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");

    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
