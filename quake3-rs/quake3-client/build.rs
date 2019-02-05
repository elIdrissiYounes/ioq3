use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

extern crate cc;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_str());
    let lib_dir = cargo_dir.join("lib");

    let mut file_paths = Vec::new();

    // Files that need to be linked, due to untranslatable code
    // E.g.: Variadic functions, inline, assembly, etc..
    file_paths.push(cargo_dir.join("../../code/asm/ftola.c"));
    file_paths.push(cargo_dir.join("../../code/asm/snapvector.c"));
    file_paths.push(cargo_dir.join("../../code/qcommon/common.c"));
    file_paths.push(cargo_dir.join("../../code/qcommon/q_shared.c"));
    file_paths.push(cargo_dir.join("../../code/qcommon/vm_variadic.c"));
    file_paths.push(cargo_dir.join("../../code/qcommon/net_ip.c"));
    file_paths.push(cargo_dir.join("../../code/qcommon/ioapi.c"));
    file_paths.push(cargo_dir.join("../../code/client/cl_variadic.c"));
    file_paths.push(cargo_dir.join("../../code/botlib/l_variadic.c"));
    file_paths.push(cargo_dir.join("../../code/botlib/l_log.c"));
    file_paths.push(cargo_dir.join("../../code/botlib/be_variadic.c"));
    file_paths.push(cargo_dir.join("../../code/server/sv_variadic.c"));
    file_paths.push(cargo_dir.join("../../code/sys/sys_variadic.c"));

    // -lSDL2 -lrt -lopusfile -lopus -lvorbisfile -lvorbis -logg
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=opusfile");
    println!("cargo:rustc-link-lib=opus");
    println!("cargo:rustc-link-lib=vorbisfile");
    println!("cargo:rustc-link-lib=vorbis");
    println!("cargo:rustc-link-lib=ogg");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    if !lib_dir.is_dir() {
        create_dir(&lib_dir).unwrap();
    }

    cc::Build::new()
        .files(file_paths)
        .flag("-pipe")
        .flag("-I/usr/include/SDL2")
        .flag("-DUSE_ICON")
        .flag("-DARCH_STRING=\"x86_64\"")
        .flag("-D_REENTRANT")
        .flag("-DUSE_OPENAL")
        .flag("-DUSE_OPENAL_DLOPEN")
        .flag("-DUSE_CURL")
        .flag("-DUSE_CURL_DLOPEN")
        .flag("-DUSE_CODEC_OPUS")
        .flag("-DUSE_CODEC_VORBIS")
        .flag("-DUSE_RENDERER_DLOPEN")
        .flag("-DUSE_MUMBLE")
        .flag("-w") // Hide warnings; cc will pass them to cargo annoyingly
        .out_dir(lib_dir)
        .compile("client");
}
