use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

extern crate cc;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_str());
    let lib_dir = cargo_dir.join("lib");

    let mut file_paths = Vec::new();

    file_paths.push(cargo_dir.join("../code/qcommon/q_math.c"));
    file_paths.push(cargo_dir.join("../code/qcommon/q_shared.c"));
    file_paths.push(cargo_dir.join("../code/ui/ui_syscalls.c"));
    file_paths.push(cargo_dir.join("../code/game/bg_lib.c"));
    file_paths.push(cargo_dir.join("../code/game/bg_misc.c"));
    file_paths.push(cargo_dir.join("../code/q3_ui/ui_variadic.c"));

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    if !lib_dir.is_dir() {
        create_dir(&lib_dir).unwrap();
    }

    for file_path in &file_paths {
        compile_files(file_path.clone(), &lib_dir, &cargo_dir)
    }
}

fn compile_files(file_path: PathBuf, lib_dir: &PathBuf, cargo_dir: &Path) {
    let file = file_path.clone();
    let file_name = file_path.file_name().expect("File name should be here!");

    /*
    cc  -DUI -fPIC -fvisibility=hidden -Wall -fno-strict-aliasing -Wimplicit -Wstrict-prototypes -pipe -DUSE_ICON -DARCH_STRING=\"x86_64\" -DNO_GZIP -Icode/zlib -DUSE_INTERNAL_JPEG -Icode/jpeg-8c -DUSE_LOCAL_HEADERS -DPRODUCT_VERSION=\"1.36_GIT_4f258568-
    2019-01-25\" -Wformat=2 -Wno-format-zero-length -Wformat-security -Wno-format-nonliteral -Wstrict-aliasing=2 -Wmissing-format-attribute -Wdisabled-optimization -Werror-implicit-function-declaration -MMD -ggdb -O0 -o build/debug-linux-x86_64/baseq3/ui
    /ui_variadic.o -c code/q3_ui/ui_variadic.c
    */
    cc::Build::new()
        .flag("-c")
        .file(file)
        .flag("-DUI")
        .flag("-fPIC")
        .flag("-pipe")
        .flag("-DUSE_ICON")
        .flag("-DARCH_STRING=\"x86_64\"")
        .flag("-w") // Hide warnings; cc will pass them to cargo annoyingly
        .out_dir(lib_dir)
        .compile(file_name.to_str().expect("String"));
}
