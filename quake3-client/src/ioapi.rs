use libc;
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/libio.h"]
pub mod libio_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub __pad1: *mut libc::c_void,
        pub __pad2: *mut libc::c_void,
        pub __pad3: *mut libc::c_void,
        pub __pad4: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    pub type _IO_lock_t = ();
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct _IO_marker {
        pub _next: *mut _IO_marker,
        pub _sbuf: *mut _IO_FILE,
        pub _pos: libc::c_int,
    }
    use super::{libc};
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::{size_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h"]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::libio_h::{_IO_FILE};
}
#[header_src = "/usr/include/zconf.h"]
pub mod zconf_h {
    pub type uLong = libc::c_ulong;
    pub type voidpf = *mut libc::c_void;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/ioapi.h"]
pub mod ioapi_h {
    /* ioapi.h -- IO base function header for compress/uncompress .zip
   files using zlib + zip or unzip API

   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant
*/
    pub type open_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: *const libc::c_char,
                                    _: libc::c_int) -> voidpf>;
    pub type read_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf,
                                    _: *mut libc::c_void, _: uLong) -> uLong>;
    pub type write_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf,
                                    _: *const libc::c_void, _: uLong)
                   -> uLong>;
    pub type tell_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> libc::c_long>;
    pub type seek_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf, _: uLong,
                                    _: libc::c_int) -> libc::c_long>;
    pub type close_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> libc::c_int>;
    pub type testerror_file_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> libc::c_int>;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct zlib_filefunc_def_s {
        pub zopen_file: open_file_func,
        pub zread_file: read_file_func,
        pub zwrite_file: write_file_func,
        pub ztell_file: tell_file_func,
        pub zseek_file: seek_file_func,
        pub zclose_file: close_file_func,
        pub zerror_file: testerror_file_func,
        pub opaque: voidpf,
    }
    pub type zlib_filefunc_def = zlib_filefunc_def_s;
    use super::zconf_h::{voidpf, uLong};
    use super::{libc};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::FILE_h::{FILE};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn fopen(__filename: *const libc::c_char,
                     __modes: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        pub fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
                     __stream: *mut FILE) -> size_t;
        #[no_mangle]
        pub fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
                      __s: *mut FILE) -> size_t;
        #[no_mangle]
        pub fn fseek(__stream: *mut FILE, __off: libc::c_long,
                     __whence: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn ftell(__stream: *mut FILE) -> libc::c_long;
        #[no_mangle]
        pub fn ferror(__stream: *mut FILE) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/ioapi.c"]
pub mod ioapi_c {
    use super::{libc};
    use super::zconf_h::{voidpf, uLong};
}
use self::stddef_h::{size_t};
use self::types_h::{__off_t, __off64_t};
use self::libio_h::{_IO_FILE, _IO_lock_t, _IO_marker};
use self::FILE_h::{FILE};
use self::zconf_h::{uLong, voidpf};
use self::ioapi_h::{open_file_func, read_file_func, write_file_func,
                    tell_file_func, seek_file_func, close_file_func,
                    testerror_file_func, zlib_filefunc_def_s,
                    zlib_filefunc_def};
use self::stdio_h::{fclose, fopen, fread, fwrite, fseek, ftell, ferror};
#[no_mangle]
pub unsafe extern "C" fn ferror_file_func(mut opaque: voidpf,
                                          mut stream: voidpf) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = ferror(stream as *mut FILE);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fclose_file_func(mut opaque: voidpf,
                                          mut stream: voidpf) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = fclose(stream as *mut FILE);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fseek_file_func(mut opaque: voidpf,
                                         mut stream: voidpf,
                                         mut offset: uLong,
                                         mut origin: libc::c_int)
 -> libc::c_long {
    let mut fseek_origin: libc::c_int = 0i32;
    let mut ret: libc::c_long = 0;
    match origin {
        1 => { fseek_origin = 1i32 }
        2 => { fseek_origin = 2i32 }
        0 => { fseek_origin = 0i32 }
        _ => { return -1i32 as libc::c_long }
    }
    ret = 0i32 as libc::c_long;
    fseek(stream as *mut FILE, offset as libc::c_long, fseek_origin);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ftell_file_func(mut opaque: voidpf,
                                         mut stream: voidpf) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    ret = ftell(stream as *mut FILE);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fwrite_file_func(mut opaque: voidpf,
                                          mut stream: voidpf,
                                          mut buf: *const libc::c_void,
                                          mut size: uLong) -> uLong {
    let mut ret: uLong = 0;
    ret = fwrite(buf, 1i32 as size_t, size, stream as *mut FILE);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fread_file_func(mut opaque: voidpf,
                                         mut stream: voidpf,
                                         mut buf: *mut libc::c_void,
                                         mut size: uLong) -> uLong {
    let mut ret: uLong = 0;
    ret = fread(buf, 1i32 as size_t, size, stream as *mut FILE);
    return ret;
}
/* ioapi.c -- IO base function header for compress/uncompress .zip
   files using zlib + zip or unzip API

   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant
*/
/* I've found an old Unix (a SunOS 4.1.3_U1) without all SEEK_* defined.... */
#[no_mangle]
pub unsafe extern "C" fn fopen_file_func(mut opaque: voidpf,
                                         mut filename: *const libc::c_char,
                                         mut mode: libc::c_int) -> voidpf {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut mode_fopen: *const libc::c_char = 0 as *const libc::c_char;
    if mode & 3i32 == 1i32 {
        mode_fopen = b"rb\x00" as *const u8 as *const libc::c_char
    } else if 0 != mode & 4i32 {
        mode_fopen = b"r+b\x00" as *const u8 as *const libc::c_char
    } else if 0 != mode & 8i32 {
        mode_fopen = b"wb\x00" as *const u8 as *const libc::c_char
    }
    if !filename.is_null() && !mode_fopen.is_null() {
        file = fopen(filename, mode_fopen)
    }
    return file as voidpf;
}