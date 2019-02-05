use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

extern crate cc;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_str());
    let lib_dir = cargo_dir.join("lib");

    let mut file_paths = Vec::new();

    file_paths.push(cargo_dir.join("../../code/qcommon/q_math.c"));
    file_paths.push(cargo_dir.join("../../code/qcommon/q_shared.c"));
    file_paths.push(cargo_dir.join("../../code/game/g_syscalls.c"));
    file_paths.push(cargo_dir.join("../../code/game/bg_lib.c"));
    file_paths.push(cargo_dir.join("../../code/game/bg_misc.c"));
    file_paths.push(cargo_dir.join("../../code/game/bg_pmove.c"));
    file_paths.push(cargo_dir.join("../../code/game/bg_slidemove.c"));
    file_paths.push(cargo_dir.join("../../code/game/ai_variadic.c"));
    file_paths.push(cargo_dir.join("../../code/game/g_variadic.c"));

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    if !lib_dir.is_dir() {
        create_dir(&lib_dir).unwrap();
    }

    let zlib = cargo_dir.join("../code/zlib");
    let jpeg = cargo_dir.join("../code/jpeg-8c");

    cc::Build::new()
        .files(file_paths)
        .flag("-DQAGAME")
        .flag("-fPIC")
        .flag("-pipe")
        .flag("-DUSE_ICON")
        .flag("-DARCH_STRING=\"x86_64\"")
        .flag("-w") // Hide warnings; cc will pass them to cargo annoyingly
        .include(zlib)
        .include(jpeg)
        .out_dir(lib_dir)
        .compile("game");
}
