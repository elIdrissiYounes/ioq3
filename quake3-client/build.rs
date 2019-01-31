use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

extern crate cc;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_str());
    let lib_dir = cargo_dir.join("lib");

    let mut file_paths = Vec::new();

    file_paths.push(cargo_dir.join("../code/asm/ftola.c"));
    file_paths.push(cargo_dir.join("../code/asm/snapvector.c"));

    file_paths.push(cargo_dir.join("../code/qcommon/common.c"));
    file_paths.push(cargo_dir.join("../code/qcommon/q_shared.c"));
    file_paths.push(cargo_dir.join("../code/qcommon/files.c"));
    file_paths.push(cargo_dir.join("../code/qcommon/vm_variadic.c"));
    file_paths.push(cargo_dir.join("../code/qcommon/net_ip.c"));
    file_paths.push(cargo_dir.join("../code/qcommon/ioapi.c"));

    file_paths.push(cargo_dir.join("../code/client/cl_variadic.c"));

    file_paths.push(cargo_dir.join("../code/botlib/l_variadic.c"));
    file_paths.push(cargo_dir.join("../code/botlib/l_log.c"));
    file_paths.push(cargo_dir.join("../code/botlib/be_variadic.c"));

    // These 4 cause a bad AI
    file_paths.push(cargo_dir.join("../code/server/sv_client.c"));
    file_paths.push(cargo_dir.join("../code/server/sv_game.c"));
    file_paths.push(cargo_dir.join("../code/server/sv_bot.c"));
    file_paths.push(cargo_dir.join("../code/server/sv_init.c"));

    file_paths.push(cargo_dir.join("../code/server/sv_variadic.c"));

    file_paths.push(cargo_dir.join("../code/sys/sys_variadic.c"));

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

    for file_path in &file_paths {
        compile_files(file_path.clone(), &lib_dir, &cargo_dir)
    }
}

fn compile_files(file_path: PathBuf, lib_dir: &PathBuf, cargo_dir: &Path) {
    let file = file_path.clone();
    let file_name = file_path.file_name().expect("File name should be here!");

    /*
     cc  -Wall -fno-strict-aliasing -Wimplicit -Wstrict-prototypes -pipe -DUSE_ICON -DARCH_STRING=\"x86_64\"   -DPRODUCT_VERSION=\"1.36_GIT_3db749c8-2019-01-29\" -Wformat=2 -Wno-format-zero-length -Wformat-security -Wno-format-nonliteral -Wstrict-aliasing=2 -Wmissing-format-attribute -Wdisabled-optimization -Werror-implicit-function-declaration -MMD -D_REENTRANT -I/usr/include/SDL2 -I/usr/include/x86_64-linux-gnu -DUSE_OPENAL -DUSE_OPENAL_DLOPEN -DUSE_CURL -DUSE_CURL_DLOPEN -DUSE_VOIP -DUSE_CODEC_OPUS -I/usr/include/opus -DUSE_CODEC_VORBIS   -DUSE_RENDERER_DLOPEN -DUSE_MUMBLE -ggdb -O0 -o build/debug-linux-x86_64/client/cl_cin.o -c code/client/cl_cin.c

    */
    cc::Build::new()
        .flag("-c")
        .file(file)
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
        .compile(file_name.to_str().expect("String"));
}
