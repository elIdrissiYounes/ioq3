use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    pub type off_t = __off_t;
    use super::types_h::{__off_t};
}
#[header_src = "/usr/include/zconf.h"]
pub mod zconf_h {
    pub type Byte = libc::c_uchar;
    pub type uInt = libc::c_uint;
    pub type uLong = libc::c_ulong;
    pub type Bytef = Byte;
    pub type voidpf = *mut libc::c_void;
    pub type voidp = *mut libc::c_void;
    use super::{libc};
}
#[header_src = "/usr/include/zlib.h"]
pub mod zlib_h {
    pub type alloc_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
    pub type free_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct z_stream_s {
        pub next_in: *mut Bytef,
        pub avail_in: uInt,
        pub total_in: uLong,
        pub next_out: *mut Bytef,
        pub avail_out: uInt,
        pub total_out: uLong,
        pub msg: *mut libc::c_char,
        pub state: *mut internal_state,
        pub zalloc: alloc_func,
        pub zfree: free_func,
        pub opaque: voidpf,
        pub data_type: libc::c_int,
        pub adler: uLong,
        pub reserved: uLong,
    }
    pub type z_stream = z_stream_s;
    pub type z_streamp = *mut z_stream;
    use super::zconf_h::{voidpf, uInt, Bytef, uLong};
    use super::{libc};
    extern "C" {
        pub type internal_state;
        #[no_mangle]
        pub fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn inflateEnd(strm: z_streamp) -> libc::c_int;
        #[no_mangle]
        pub fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
        #[no_mangle]
        pub fn inflateInit2_(strm: z_streamp, windowBits: libc::c_int,
                             version: *const libc::c_char,
                             stream_size: libc::c_int) -> libc::c_int;
    }
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
    extern "C" {
        #[no_mangle]
        pub fn fill_fopen_filefunc(pzlib_filefunc_def:
                                       *mut zlib_filefunc_def);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/unzip.h"]
pub mod unzip_h {
    /* unzip.h -- IO for uncompress .zip files using zlib
   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant

   This unzip package allow extract file from .ZIP file, compatible with PKZip 2.04g
     WinZip, InfoZip tools and compatible.

   Multi volume ZipFile (span) are not supported.
   Encryption compatible with pkzip 2.04g only supported
   Old compressions used by old PKZip 1.x are not supported


   I WAIT FEEDBACK at mail info@winimage.com
   Visit also http://www.winimage.com/zLibDll/unzip.htm for evolution

   Condition of use and distribution are the same than zlib :

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.


*/
    /* for more info about .ZIP format, see
      http://www.info-zip.org/pub/infozip/doc/appnote-981119-iz.zip
      http://www.info-zip.org/pub/infozip/doc/
   PkWare has also a specification at :
      ftp://ftp.pkware.com/probdesc.zip
*/
    pub type unzFile = voidp;
    /* tm_unz contain date/time info */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct tm_unz_s {
        pub tm_sec: uInt,
        pub tm_min: uInt,
        pub tm_hour: uInt,
        pub tm_mday: uInt,
        pub tm_mon: uInt,
        pub tm_year: uInt,
    }
    pub type tm_unz = tm_unz_s;
    /* unz_global_info structure contain global data about the ZIPfile
   These data comes from the end of central dir */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_global_info_s {
        pub number_entry: uLong,
        pub size_comment: uLong,
    }
    pub type unz_global_info = unz_global_info_s;
    /* unz_file_info contain information about a file in the zipfile */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_file_info_s {
        pub version: uLong,
        pub version_needed: uLong,
        pub flag: uLong,
        pub compression_method: uLong,
        pub dosDate: uLong,
        pub crc: uLong,
        pub compressed_size: uLong,
        pub uncompressed_size: uLong,
        pub size_filename: uLong,
        pub size_file_extra: uLong,
        pub size_file_comment: uLong,
        pub disk_num_start: uLong,
        pub internal_fa: uLong,
        pub external_fa: uLong,
        pub tmu_date: tm_unz,
    }
    pub type unz_file_info = unz_file_info_s;
    /*
  Try locate the file szFileName in the zipfile.
  For the iCaseSensitivity signification, see unzStringFileNameCompare

  return value :
  UNZ_OK if the file is found. It becomes the current file.
  UNZ_END_OF_LIST_OF_FILE if the file is not found
*/
    /* ****************************************** */
/* Ryan supplied functions */
/* unz_file_info contain information about a file in the zipfile */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_file_pos_s {
        /* unz_s contain internal information about the zipfile
*/
        /* file_in_zip_read_info_s contain internal information about a file in zipfile,
    when reading and decompress it */
        /* unz_file_info_interntal contain internal info about a file in zipfile*/
        pub pos_in_zip_directory: uLong,
        pub num_of_file: uLong,
    }
    pub type unz_file_pos = unz_file_pos_s;
    use super::zconf_h::{voidp, uInt, uLong};
    use super::{libc};
    use super::ioapi_h::{zlib_filefunc_def};
    use super::stdio_h::{off_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/unzip.c"]
pub mod unzip_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_s {
        pub z_filefunc: zlib_filefunc_def,
        pub filestream: voidpf,
        pub gi: unz_global_info,
        pub byte_before_the_zipfile: uLong,
        pub num_file: uLong,
        pub pos_in_central_dir: uLong,
        pub current_file_ok: uLong,
        pub central_pos: uLong,
        pub size_central_dir: uLong,
        pub offset_central_dir: uLong,
        pub cur_file_info: unz_file_info,
        pub cur_file_info_internal: unz_file_info_internal,
        pub pfile_in_zip_read: *mut file_in_zip_read_info_s,
        pub encrypted: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct file_in_zip_read_info_s {
        pub read_buffer: *mut libc::c_char,
        pub stream: z_stream,
        pub pos_in_zipfile: uLong,
        pub stream_initialised: uLong,
        pub offset_local_extrafield: uLong,
        pub size_local_extrafield: uInt,
        pub pos_local_extrafield: uLong,
        pub crc32: uLong,
        pub crc32_wait: uLong,
        pub rest_read_compressed: uLong,
        pub rest_read_uncompressed: uLong,
        pub z_filefunc: zlib_filefunc_def,
        pub filestream: voidpf,
        pub compression_method: uLong,
        pub byte_before_the_zipfile: uLong,
        pub raw: libc::c_int,
    }
    pub type unz_file_info_internal = unz_file_info_internal_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_file_info_internal_s {
        pub offset_curfile: uLong,
    }
    use super::ioapi_h::{zlib_filefunc_def};
    use super::zconf_h::{voidpf, uLong, uInt};
    use super::unzip_h::{unz_global_info, unz_file_info};
    use super::{libc};
    use super::zlib_h::{z_stream};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
    }
}
use self::types_h::{__off_t};
use self::stdio_h::{off_t};
use self::zconf_h::{Byte, uInt, uLong, Bytef, voidpf, voidp};
use self::zlib_h::{alloc_func, free_func, z_stream_s, z_stream, z_streamp,
                   internal_state, inflate, inflateEnd, crc32, inflateInit2_};
use self::ioapi_h::{open_file_func, read_file_func, write_file_func,
                    tell_file_func, seek_file_func, close_file_func,
                    testerror_file_func, zlib_filefunc_def_s,
                    zlib_filefunc_def, fill_fopen_filefunc};
use self::unzip_h::{unzFile, tm_unz_s, tm_unz, unz_global_info_s,
                    unz_global_info, unz_file_info_s, unz_file_info,
                    unz_file_pos_s, unz_file_pos};
use self::unzip_c::{unz_s, file_in_zip_read_info_s, unz_file_info_internal,
                    unz_file_info_internal_s};
use self::string_h::{strcmp, strlen};
use self::qcommon_h::{Z_MallocDebug, Z_Free};
#[no_mangle]
pub unsafe extern "C" fn unzStringFileNameCompare(mut fileName1:
                                                      *const libc::c_char,
                                                  mut fileName2:
                                                      *const libc::c_char,
                                                  mut iCaseSensitivity:
                                                      libc::c_int)
 -> libc::c_int {
    if iCaseSensitivity == 0i32 { iCaseSensitivity = 1i32 }
    if iCaseSensitivity == 1i32 { return strcmp(fileName1, fileName2) }
    return strcmpcasenosensitive_internal(fileName1, fileName2);
}
unsafe extern "C" fn strcmpcasenosensitive_internal(mut fileName1:
                                                        *const libc::c_char,
                                                    mut fileName2:
                                                        *const libc::c_char)
 -> libc::c_int {
    loop  {
        let fresh0 = fileName1;
        fileName1 = fileName1.offset(1);
        let mut c1: libc::c_char = *fresh0;
        let fresh1 = fileName2;
        fileName2 = fileName2.offset(1);
        let mut c2: libc::c_char = *fresh1;
        if c1 as libc::c_int >= 'a' as i32 && c1 as libc::c_int <= 'z' as i32
           {
            c1 = (c1 as libc::c_int - 0x20i32) as libc::c_char
        }
        if c2 as libc::c_int >= 'a' as i32 && c2 as libc::c_int <= 'z' as i32
           {
            c2 = (c2 as libc::c_int - 0x20i32) as libc::c_char
        }
        if c1 as libc::c_int == '\u{0}' as i32 {
            return if c2 as libc::c_int == '\u{0}' as i32 {
                       0i32
                   } else { -1i32 }
        }
        if c2 as libc::c_int == '\u{0}' as i32 { return 1i32 }
        if (c1 as libc::c_int) < c2 as libc::c_int { return -1i32 }
        if c1 as libc::c_int > c2 as libc::c_int { return 1i32 }
    };
}
/*
   Compare two filename (fileName1,fileName2).
   If iCaseSenisivity = 1, comparison is case sensitivity (like strcmp)
   If iCaseSenisivity = 2, comparison is not case sensitivity (like strcmpi
                                or strcasecmp)
   If iCaseSenisivity = 0, case sensitivity is defaut of your operating system
    (like 1 on Unix, 2 on Windows)
*/
#[no_mangle]
pub unsafe extern "C" fn unzOpen(mut path: *const libc::c_char) -> unzFile {
    return unzOpen2(path, 0 as *mut zlib_filefunc_def);
}
/*
  Open a Zip file. path contain the full pathname (by example,
     on a Windows XP computer "c:\\zlib\\zlib113.zip" or on a Unix computer
     "zlib/zlib113.zip".
     If the zipfile cannot be opened (file don't exist or in not valid), the
       return value is NULL.
     Else, the return value is an unzFile Handle, usable with other function
       of this unzip package.
*/
#[no_mangle]
pub unsafe extern "C" fn unzOpen2(mut path: *const libc::c_char,
                                  mut pzlib_filefunc_def:
                                      *mut zlib_filefunc_def) -> unzFile {
    let mut us: unz_s =
        unz_s{z_filefunc:
                  zlib_filefunc_def_s{zopen_file: None,
                                      zread_file: None,
                                      zwrite_file: None,
                                      ztell_file: None,
                                      zseek_file: None,
                                      zclose_file: None,
                                      zerror_file: None,
                                      opaque: 0 as *mut libc::c_void,},
              filestream: 0 as *mut libc::c_void,
              gi: unz_global_info_s{number_entry: 0, size_comment: 0,},
              byte_before_the_zipfile: 0,
              num_file: 0,
              pos_in_central_dir: 0,
              current_file_ok: 0,
              central_pos: 0,
              size_central_dir: 0,
              offset_central_dir: 0,
              cur_file_info:
                  unz_file_info_s{version: 0,
                                  version_needed: 0,
                                  flag: 0,
                                  compression_method: 0,
                                  dosDate: 0,
                                  crc: 0,
                                  compressed_size: 0,
                                  uncompressed_size: 0,
                                  size_filename: 0,
                                  size_file_extra: 0,
                                  size_file_comment: 0,
                                  disk_num_start: 0,
                                  internal_fa: 0,
                                  external_fa: 0,
                                  tmu_date:
                                      tm_unz_s{tm_sec: 0,
                                               tm_min: 0,
                                               tm_hour: 0,
                                               tm_mday: 0,
                                               tm_mon: 0,
                                               tm_year: 0,},},
              cur_file_info_internal:
                  unz_file_info_internal_s{offset_curfile: 0,},
              pfile_in_zip_read: 0 as *mut file_in_zip_read_info_s,
              encrypted: 0,};
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut central_pos: uLong = 0;
    let mut uL: uLong = 0;
    /* number of the current dist, used for
                                   spaning ZIP, unsupported, always 0*/
    let mut number_disk: uLong = 0;
    /* number the the disk with central dir, used
                                   for spaning ZIP, unsupported, always 0*/
    let mut number_disk_with_CD: uLong = 0;
    /* total number of entries in
                                   the central dir
                                   (same than number_entry on nospan) */
    let mut number_entry_CD: uLong = 0;
    let mut err: libc::c_int = 0i32;
    if unz_copyright[0usize] as libc::c_int != ' ' as i32 {
        return 0 as *mut libc::c_void
    }
    if pzlib_filefunc_def.is_null() {
        fill_fopen_filefunc(&mut us.z_filefunc);
    } else { us.z_filefunc = *pzlib_filefunc_def }
    us.filestream =
        us.z_filefunc.zopen_file.expect("non-null function pointer")(us.z_filefunc.opaque,
                                                                     path,
                                                                     1i32 |
                                                                         4i32);
    if us.filestream.is_null() { return 0 as *mut libc::c_void }
    central_pos =
        unzlocal_SearchCentralDir(&mut us.z_filefunc, us.filestream);
    if central_pos == 0i32 as libc::c_ulong { err = -1i32 }
    if us.z_filefunc.zseek_file.expect("non-null function pointer")(us.z_filefunc.opaque,
                                                                    us.filestream,
                                                                    central_pos,
                                                                    0i32) !=
           0i32 as libc::c_long {
        err = -1i32
    }
    if unzlocal_getLong(&mut us.z_filefunc, us.filestream, &mut uL) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream, &mut number_disk)
           != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream,
                         &mut number_disk_with_CD) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream,
                         &mut us.gi.number_entry) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream,
                         &mut number_entry_CD) != 0i32 {
        err = -1i32
    }
    if number_entry_CD != us.gi.number_entry ||
           number_disk_with_CD != 0i32 as libc::c_ulong ||
           number_disk != 0i32 as libc::c_ulong {
        err = -103i32
    }
    if unzlocal_getLong(&mut us.z_filefunc, us.filestream,
                        &mut us.size_central_dir) != 0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut us.z_filefunc, us.filestream,
                        &mut us.offset_central_dir) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream,
                         &mut us.gi.size_comment) != 0i32 {
        err = -1i32
    }
    if central_pos < us.offset_central_dir.wrapping_add(us.size_central_dir)
           && err == 0i32 {
        err = -103i32
    }
    if err != 0i32 {
        us.z_filefunc.zclose_file.expect("non-null function pointer")(us.z_filefunc.opaque,
                                                                      us.filestream);
        return 0 as *mut libc::c_void
    }
    us.byte_before_the_zipfile =
        central_pos.wrapping_sub(us.offset_central_dir.wrapping_add(us.size_central_dir));
    us.central_pos = central_pos;
    us.pfile_in_zip_read = 0 as *mut file_in_zip_read_info_s;
    us.encrypted = 0i32;
    s =
        Z_MallocDebug(::std::mem::size_of::<unz_s>() as libc::c_ulong as
                          libc::c_int,
                      b"sizeof(unz_s)\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"code/qcommon/unzip.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 488i32) as
            *mut unz_s;
    *s = us;
    unzGoToFirstFile(s as unzFile);
    return s as unzFile;
}
/*
  Get the global comment string of the ZipFile, in the szComment buffer.
  uSizeBuf is the size of the szComment buffer.
  return the number of byte copied or an error code <0
*/
/* **************************************************************************/
/* Unzip package allow you browse the directory of the zipfile */
#[no_mangle]
pub unsafe extern "C" fn unzGoToFirstFile(mut file: unzFile) -> libc::c_int {
    let mut err: libc::c_int = 0i32;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    (*s).pos_in_central_dir = (*s).offset_central_dir;
    (*s).num_file = 0i32 as uLong;
    err =
        unzlocal_GetCurrentFileInfoInternal(file, &mut (*s).cur_file_info,
                                            &mut (*s).cur_file_info_internal,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong,
                                            0 as *mut libc::c_void,
                                            0i32 as uLong,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong);
    (*s).current_file_ok = (err == 0i32) as libc::c_int as uLong;
    return err;
}
/*
  Get Info about the current file in the zipfile, with internal only info
*/
unsafe extern "C" fn unzlocal_GetCurrentFileInfoInternal(mut file: unzFile,
                                                         mut pfile_info:
                                                             *mut unz_file_info,
                                                         mut pfile_info_internal:
                                                             *mut unz_file_info_internal,
                                                         mut szFileName:
                                                             *mut libc::c_char,
                                                         mut fileNameBufferSize:
                                                             uLong,
                                                         mut extraField:
                                                             *mut libc::c_void,
                                                         mut extraFieldBufferSize:
                                                             uLong,
                                                         mut szComment:
                                                             *mut libc::c_char,
                                                         mut commentBufferSize:
                                                             uLong)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut file_info: unz_file_info =
        unz_file_info_s{version: 0,
                        version_needed: 0,
                        flag: 0,
                        compression_method: 0,
                        dosDate: 0,
                        crc: 0,
                        compressed_size: 0,
                        uncompressed_size: 0,
                        size_filename: 0,
                        size_file_extra: 0,
                        size_file_comment: 0,
                        disk_num_start: 0,
                        internal_fa: 0,
                        external_fa: 0,
                        tmu_date:
                            tm_unz_s{tm_sec: 0,
                                     tm_min: 0,
                                     tm_hour: 0,
                                     tm_mday: 0,
                                     tm_mon: 0,
                                     tm_year: 0,},};
    let mut file_info_internal: unz_file_info_internal =
        unz_file_info_internal_s{offset_curfile: 0,};
    let mut err: libc::c_int = 0i32;
    let mut uMagic: uLong = 0;
    let mut lSeek: libc::c_long = 0i32 as libc::c_long;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    if (*s).z_filefunc.zseek_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                      (*s).filestream,
                                                                      (*s).pos_in_central_dir.wrapping_add((*s).byte_before_the_zipfile),
                                                                      0i32) !=
           0i32 as libc::c_long {
        err = -1i32
    }
    if err == 0i32 {
        if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                            &mut uMagic) != 0i32 {
            err = -1i32
        } else if uMagic != 0x2014b50i32 as libc::c_ulong { err = -103i32 }
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.version) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.version_needed) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.flag) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.compression_method) != 0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                        &mut file_info.dosDate) != 0i32 {
        err = -1i32
    }
    unzlocal_DosDateToTmuDate(file_info.dosDate, &mut file_info.tmu_date);
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                        &mut file_info.crc) != 0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                        &mut file_info.compressed_size) != 0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                        &mut file_info.uncompressed_size) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.size_filename) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.size_file_extra) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.size_file_comment) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.disk_num_start) != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut file_info.internal_fa) != 0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                        &mut file_info.external_fa) != 0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                        &mut file_info_internal.offset_curfile) != 0i32 {
        err = -1i32
    }
    lSeek =
        (lSeek as libc::c_ulong).wrapping_add(file_info.size_filename) as
            libc::c_long as libc::c_long;
    if err == 0i32 && !szFileName.is_null() {
        let mut uSizeRead: uLong = 0;
        if file_info.size_filename < fileNameBufferSize {
            *szFileName.offset(file_info.size_filename as isize) =
                '\u{0}' as i32 as libc::c_char;
            uSizeRead = file_info.size_filename
        } else { uSizeRead = fileNameBufferSize }
        if file_info.size_filename > 0i32 as libc::c_ulong &&
               fileNameBufferSize > 0i32 as libc::c_ulong {
            if (*s).z_filefunc.zread_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                              (*s).filestream,
                                                                              szFileName
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              uSizeRead)
                   != uSizeRead {
                err = -1i32
            }
        }
        lSeek =
            (lSeek as libc::c_ulong).wrapping_sub(uSizeRead) as libc::c_long
                as libc::c_long
    }
    if err == 0i32 && !extraField.is_null() {
        let mut uSizeRead_0: uLong = 0;
        if file_info.size_file_extra < extraFieldBufferSize {
            uSizeRead_0 = file_info.size_file_extra
        } else { uSizeRead_0 = extraFieldBufferSize }
        if lSeek != 0i32 as libc::c_long {
            if (*s).z_filefunc.zseek_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                              (*s).filestream,
                                                                              lSeek
                                                                                  as
                                                                                  uLong,
                                                                              1i32)
                   == 0i32 as libc::c_long {
                lSeek = 0i32 as libc::c_long
            } else { err = -1i32 }
        }
        if file_info.size_file_extra > 0i32 as libc::c_ulong &&
               extraFieldBufferSize > 0i32 as libc::c_ulong {
            if (*s).z_filefunc.zread_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                              (*s).filestream,
                                                                              extraField,
                                                                              uSizeRead_0)
                   != uSizeRead_0 {
                err = -1i32
            }
        }
        lSeek =
            (lSeek as
                 libc::c_ulong).wrapping_add(file_info.size_file_extra.wrapping_sub(uSizeRead_0))
                as libc::c_long as libc::c_long
    } else {
        lSeek =
            (lSeek as libc::c_ulong).wrapping_add(file_info.size_file_extra)
                as libc::c_long as libc::c_long
    }
    if err == 0i32 && !szComment.is_null() {
        let mut uSizeRead_1: uLong = 0;
        if file_info.size_file_comment < commentBufferSize {
            *szComment.offset(file_info.size_file_comment as isize) =
                '\u{0}' as i32 as libc::c_char;
            uSizeRead_1 = file_info.size_file_comment
        } else { uSizeRead_1 = commentBufferSize }
        if lSeek != 0i32 as libc::c_long {
            if (*s).z_filefunc.zseek_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                              (*s).filestream,
                                                                              lSeek
                                                                                  as
                                                                                  uLong,
                                                                              1i32)
                   != 0i32 as libc::c_long {
                err = -1i32
            }
        }
        if file_info.size_file_comment > 0i32 as libc::c_ulong &&
               commentBufferSize > 0i32 as libc::c_ulong {
            if (*s).z_filefunc.zread_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                              (*s).filestream,
                                                                              szComment
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              uSizeRead_1)
                   != uSizeRead_1 {
                err = -1i32
            }
        }
    }
    if err == 0i32 && !pfile_info.is_null() { *pfile_info = file_info }
    if err == 0i32 && !pfile_info_internal.is_null() {
        *pfile_info_internal = file_info_internal
    }
    return err;
}
unsafe extern "C" fn unzlocal_getLong(mut pzlib_filefunc_def:
                                          *const zlib_filefunc_def,
                                      mut filestream: voidpf,
                                      mut pX: *mut uLong) -> libc::c_int {
    let mut x: uLong = 0;
    let mut i: libc::c_int = 0i32;
    let mut err: libc::c_int = 0;
    err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i);
    x = i as uLong;
    if err == 0i32 {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x =
        (x as libc::c_ulong).wrapping_add((i as uLong) << 8i32) as uLong as
            uLong;
    if err == 0i32 {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x =
        (x as libc::c_ulong).wrapping_add((i as uLong) << 16i32) as uLong as
            uLong;
    if err == 0i32 {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x =
        (x as libc::c_ulong).wrapping_add((i as uLong) << 24i32) as uLong as
            uLong;
    if err == 0i32 { *pX = x } else { *pX = 0i32 as uLong }
    return err;
}
/* ===========================================================================
     Read a byte from a gz_stream; update next_in and avail_in. Return EOF
   for end of file.
   IN assertion: the stream s has been successfully opened for reading.
*/
unsafe extern "C" fn unzlocal_getByte(mut pzlib_filefunc_def:
                                          *const zlib_filefunc_def,
                                      mut filestream: voidpf,
                                      mut pi: *mut libc::c_int)
 -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    let mut err: libc::c_int =
        (*pzlib_filefunc_def).zread_file.expect("non-null function pointer")((*pzlib_filefunc_def).opaque,
                                                                             filestream,
                                                                             &mut c
                                                                                 as
                                                                                 *mut libc::c_uchar
                                                                                 as
                                                                                 *mut libc::c_void,
                                                                             1i32
                                                                                 as
                                                                                 uLong)
            as libc::c_int;
    if err == 1i32 {
        *pi = c as libc::c_int;
        return 0i32
    } else if 0 !=
                  (*pzlib_filefunc_def).zerror_file.expect("non-null function pointer")((*pzlib_filefunc_def).opaque,
                                                                                        filestream)
     {
        return -1i32
    } else { return 0i32 };
}
/* ===========================================================================
   Reads a long in LSB order from the given gz_stream. Sets
*/
unsafe extern "C" fn unzlocal_getShort(mut pzlib_filefunc_def:
                                           *const zlib_filefunc_def,
                                       mut filestream: voidpf,
                                       mut pX: *mut uLong) -> libc::c_int {
    let mut x: uLong = 0;
    let mut i: libc::c_int = 0i32;
    let mut err: libc::c_int = 0;
    err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i);
    x = i as uLong;
    if err == 0i32 {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x =
        (x as libc::c_ulong).wrapping_add((i as uLong) << 8i32) as uLong as
            uLong;
    if err == 0i32 { *pX = x } else { *pX = 0i32 as uLong }
    return err;
}
/*
   Translate date/time from Dos format to tm_unz (readable more easilty)
*/
unsafe extern "C" fn unzlocal_DosDateToTmuDate(mut ulDosDate: uLong,
                                               mut ptm: *mut tm_unz) {
    let mut uDate: uLong = 0;
    uDate = ulDosDate >> 16i32;
    (*ptm).tm_mday = (uDate & 0x1fi32 as libc::c_ulong) as uInt;
    (*ptm).tm_mon =
        (uDate &
             0x1e0i32 as
                 libc::c_ulong).wrapping_div(0x20i32 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)
            as uInt;
    (*ptm).tm_year =
        (uDate &
             0xfe00i32 as
                 libc::c_ulong).wrapping_div(0x200i32 as
                                                 libc::c_ulong).wrapping_add(1980i32
                                                                                 as
                                                                                 libc::c_ulong)
            as uInt;
    (*ptm).tm_hour =
        (ulDosDate &
             0xf800i32 as
                 libc::c_ulong).wrapping_div(0x800i32 as libc::c_ulong) as
            uInt;
    (*ptm).tm_min =
        (ulDosDate &
             0x7e0i32 as libc::c_ulong).wrapping_div(0x20i32 as libc::c_ulong)
            as uInt;
    (*ptm).tm_sec =
        (2i32 as
             libc::c_ulong).wrapping_mul(ulDosDate & 0x1fi32 as libc::c_ulong)
            as uInt;
}
/*
  Locate the Central directory of a zipfile (at the end, just before
    the global comment)
*/
unsafe extern "C" fn unzlocal_SearchCentralDir(mut pzlib_filefunc_def:
                                                   *const zlib_filefunc_def,
                                               mut filestream: voidpf)
 -> uLong {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut uSizeFile: uLong = 0;
    let mut uBackRead: uLong = 0;
    /* maximum size of global comment */
    let mut uMaxBack: uLong = 0xffffi32 as uLong;
    let mut uPosFound: uLong = 0i32 as uLong;
    if (*pzlib_filefunc_def).zseek_file.expect("non-null function pointer")((*pzlib_filefunc_def).opaque,
                                                                            filestream,
                                                                            0i32
                                                                                as
                                                                                uLong,
                                                                            2i32)
           != 0i32 as libc::c_long {
        return 0i32 as uLong
    }
    uSizeFile =
        (*pzlib_filefunc_def).ztell_file.expect("non-null function pointer")((*pzlib_filefunc_def).opaque,
                                                                             filestream)
            as uLong;
    if uMaxBack > uSizeFile { uMaxBack = uSizeFile }
    buf =
        Z_MallocDebug(0x400i32 + 4i32,
                      b"(0x400)+4\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/unzip.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 345i32) as
            *mut libc::c_uchar;
    if buf.is_null() { return 0i32 as uLong }
    uBackRead = 4i32 as uLong;
    while uBackRead < uMaxBack {
        let mut uReadSize: uLong = 0;
        let mut uReadPos: uLong = 0;
        let mut i: libc::c_int = 0;
        if uBackRead.wrapping_add(0x400i32 as libc::c_ulong) > uMaxBack {
            uBackRead = uMaxBack
        } else {
            uBackRead =
                (uBackRead as
                     libc::c_ulong).wrapping_add(0x400i32 as libc::c_ulong) as
                    uLong as uLong
        }
        uReadPos = uSizeFile.wrapping_sub(uBackRead);
        uReadSize =
            if ((0x400i32 + 4i32) as libc::c_ulong) <
                   uSizeFile.wrapping_sub(uReadPos) {
                (0x400i32 + 4i32) as libc::c_ulong
            } else { uSizeFile.wrapping_sub(uReadPos) };
        if (*pzlib_filefunc_def).zseek_file.expect("non-null function pointer")((*pzlib_filefunc_def).opaque,
                                                                                filestream,
                                                                                uReadPos,
                                                                                0i32)
               != 0i32 as libc::c_long {
            break ;
        }
        if (*pzlib_filefunc_def).zread_file.expect("non-null function pointer")((*pzlib_filefunc_def).opaque,
                                                                                filestream,
                                                                                buf
                                                                                    as
                                                                                    *mut libc::c_void,
                                                                                uReadSize)
               != uReadSize {
            break ;
        }
        i = uReadSize as libc::c_int - 3i32;
        loop  {
            let fresh2 = i;
            i = i - 1;
            if !(fresh2 > 0i32) { break ; }
            if !(*buf.offset(i as isize) as libc::c_int == 0x50i32 &&
                     *buf.offset(i as isize).offset(1isize) as libc::c_int ==
                         0x4bi32 &&
                     *buf.offset(i as isize).offset(2isize) as libc::c_int ==
                         0x5i32 &&
                     *buf.offset(i as isize).offset(3isize) as libc::c_int ==
                         0x6i32) {
                continue ;
            }
            uPosFound = uReadPos.wrapping_add(i as libc::c_ulong);
            break ;
        }
        if uPosFound != 0i32 as libc::c_ulong { break ; }
    }
    if !buf.is_null() { Z_Free(buf as *mut libc::c_void); }
    return uPosFound;
}
/* unzip.c -- IO for uncompress .zip files using zlib

   Modified for Quake III Arena to use the Z_Malloc() memory pool;
   this means a system copy of minizip is not a suitable replacement.

   Based on minizip:

   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant

   Read unzip.h for more info
*/
/* Decryption code comes from crypt.c by Info-ZIP but has been greatly reduced in terms of
compatibility with older software. The following is from the original crypt.c. Code
woven in by Terry Thorsen 1/2003.
*/
/*
  Copyright (c) 1990-2000 Info-ZIP.  All rights reserved.

  See the accompanying file LICENSE, version 2000-Apr-09 or later
  (the contents of which are also included in zip.h) for terms of use.
  If, for some reason, all these files are missing, the Info-ZIP license
  also may be found at:  ftp://ftp.info-zip.org/pub/infozip/license.html
*/
/*
  crypt.c (full version) by Info-ZIP.      Last revised:  [see crypt.h]

  The encryption/decryption parts of this source code (as opposed to the
  non-echoing password parts) were originally written in Europe.  The
  whole source package can be freely distributed, including from the USA.
  (Prior to January 2000, re-export from the US was a violation of US law.)
 */
/*
  This encryption code is a direct transcription of the algorithm from
  Roger Schlafly, described by Phil Katz in the file appnote.txt.  This
  file (appnote.txt) is distributed with the PKZIP program (even in the
  version without encryption capabilities).
 */
/* compile with -Dlocal if your debugger can't find static symbols */
#[no_mangle]
pub static mut unz_copyright: [libc::c_char; 81] =
    [32, 117, 110, 122, 105, 112, 32, 49, 46, 48, 49, 32, 67, 111, 112, 121,
     114, 105, 103, 104, 116, 32, 49, 57, 57, 56, 45, 50, 48, 48, 52, 32, 71,
     105, 108, 108, 101, 115, 32, 86, 111, 108, 108, 97, 110, 116, 32, 45, 32,
     104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 105, 110, 105,
     109, 97, 103, 101, 46, 99, 111, 109, 47, 122, 76, 105, 98, 68, 108, 108,
     0];
/*
   Open a Zip file, like unzOpen, but provide a set of file low level API
      for read/write the zip file (see ioapi.h)
*/
#[no_mangle]
pub unsafe extern "C" fn unzClose(mut file: unzFile) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    if !(*s).pfile_in_zip_read.is_null() { unzCloseCurrentFile(file); }
    (*s).z_filefunc.zclose_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                    (*s).filestream);
    if !s.is_null() { Z_Free(s as *mut libc::c_void); }
    return 0i32;
}
/*
  Same than unzOpenCurrentFile, but open for read raw the file (not uncompress)
    if raw==1
  *method will receive method of compression, *level will receive level of
     compression
  note : you can set level parameter as NULL (if you did not want known level,
         but you CANNOT set method parameter as NULL
*/
#[no_mangle]
pub unsafe extern "C" fn unzCloseCurrentFile(mut file: unzFile)
 -> libc::c_int {
    let mut err: libc::c_int = 0i32;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() { return -102i32 }
    if (*pfile_in_zip_read_info).rest_read_uncompressed ==
           0i32 as libc::c_ulong && 0 == (*pfile_in_zip_read_info).raw {
        if (*pfile_in_zip_read_info).crc32 !=
               (*pfile_in_zip_read_info).crc32_wait {
            err = -105i32
        }
    }
    if !(*pfile_in_zip_read_info).read_buffer.is_null() {
        Z_Free((*pfile_in_zip_read_info).read_buffer as *mut libc::c_void);
    }
    (*pfile_in_zip_read_info).read_buffer = 0 as *mut libc::c_char;
    if 0 != (*pfile_in_zip_read_info).stream_initialised {
        inflateEnd(&mut (*pfile_in_zip_read_info).stream);
    }
    (*pfile_in_zip_read_info).stream_initialised = 0i32 as uLong;
    if !pfile_in_zip_read_info.is_null() {
        Z_Free(pfile_in_zip_read_info as *mut libc::c_void);
    }
    (*s).pfile_in_zip_read = 0 as *mut file_in_zip_read_info_s;
    return err;
}
/*
  Close a ZipFile opened with unzipOpen.
  If there is files inside the .Zip opened with unzOpenCurrentFile (see later),
    these files MUST be closed with unzipCloseCurrentFile before call unzipClose.
  return UNZ_OK if there is no problem. */
#[no_mangle]
pub unsafe extern "C" fn unzGetGlobalInfo(mut file: unzFile,
                                          mut pglobal_info:
                                              *mut unz_global_info)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    *pglobal_info = (*s).gi;
    return 0i32;
}
/*
  Write info about the ZipFile in the *pglobal_info structure.
  No preparation of the structure is needed
  return UNZ_OK if there is no problem. */
#[no_mangle]
pub unsafe extern "C" fn unzGetGlobalComment(mut file: unzFile,
                                             mut szComment: *mut libc::c_char,
                                             mut uSizeBuf: uLong)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut uReadThis: uLong = 0;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    uReadThis = uSizeBuf;
    if uReadThis > (*s).gi.size_comment { uReadThis = (*s).gi.size_comment }
    if (*s).z_filefunc.zseek_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                      (*s).filestream,
                                                                      (*s).central_pos.wrapping_add(22i32
                                                                                                        as
                                                                                                        libc::c_ulong),
                                                                      0i32) !=
           0i32 as libc::c_long {
        return -1i32
    }
    if uReadThis > 0i32 as libc::c_ulong {
        *szComment = '\u{0}' as i32 as libc::c_char;
        if (*s).z_filefunc.zread_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                          (*s).filestream,
                                                                          szComment
                                                                              as
                                                                              *mut libc::c_void,
                                                                          uReadThis)
               != uReadThis {
            return -1i32
        }
    }
    if !szComment.is_null() && uSizeBuf > (*s).gi.size_comment {
        *szComment.offset((*s).gi.size_comment as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    return uReadThis as libc::c_int;
}
/*
  Set the current file of the zipfile to the first file.
  return UNZ_OK if there is no problem
*/
#[no_mangle]
pub unsafe extern "C" fn unzGoToNextFile(mut file: unzFile) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    if 0 == (*s).current_file_ok { return -100i32 }
    if (*s).gi.number_entry != 0xffffi32 as libc::c_ulong {
        if (*s).num_file.wrapping_add(1i32 as libc::c_ulong) ==
               (*s).gi.number_entry {
            return -100i32
        }
    }
    (*s).pos_in_central_dir =
        ((*s).pos_in_central_dir as
             libc::c_ulong).wrapping_add((0x2ei32 as
                                              libc::c_ulong).wrapping_add((*s).cur_file_info.size_filename).wrapping_add((*s).cur_file_info.size_file_extra).wrapping_add((*s).cur_file_info.size_file_comment))
            as uLong as uLong;
    (*s).num_file = (*s).num_file.wrapping_add(1);
    err =
        unzlocal_GetCurrentFileInfoInternal(file, &mut (*s).cur_file_info,
                                            &mut (*s).cur_file_info_internal,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong,
                                            0 as *mut libc::c_void,
                                            0i32 as uLong,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong);
    (*s).current_file_ok = (err == 0i32) as libc::c_int as uLong;
    return err;
}
/*
  Set the current file of the zipfile to the next file.
  return UNZ_OK if there is no problem
  return UNZ_END_OF_LIST_OF_FILE if the actual file was the latest.
*/
#[no_mangle]
pub unsafe extern "C" fn unzLocateFile(mut file: unzFile,
                                       mut szFileName: *const libc::c_char,
                                       mut iCaseSensitivity: libc::c_int)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    /* We remember the 'current' position in the file so that we can jump
     * back there if we fail.
     */
    let mut cur_file_infoSaved: unz_file_info =
        unz_file_info_s{version: 0,
                        version_needed: 0,
                        flag: 0,
                        compression_method: 0,
                        dosDate: 0,
                        crc: 0,
                        compressed_size: 0,
                        uncompressed_size: 0,
                        size_filename: 0,
                        size_file_extra: 0,
                        size_file_comment: 0,
                        disk_num_start: 0,
                        internal_fa: 0,
                        external_fa: 0,
                        tmu_date:
                            tm_unz_s{tm_sec: 0,
                                     tm_min: 0,
                                     tm_hour: 0,
                                     tm_mday: 0,
                                     tm_mon: 0,
                                     tm_year: 0,},};
    let mut cur_file_info_internalSaved: unz_file_info_internal =
        unz_file_info_internal_s{offset_curfile: 0,};
    let mut num_fileSaved: uLong = 0;
    let mut pos_in_central_dirSaved: uLong = 0;
    if file.is_null() { return -102i32 }
    if strlen(szFileName) >= 256i32 as libc::c_ulong { return -102i32 }
    s = file as *mut unz_s;
    if 0 == (*s).current_file_ok { return -100i32 }
    num_fileSaved = (*s).num_file;
    pos_in_central_dirSaved = (*s).pos_in_central_dir;
    cur_file_infoSaved = (*s).cur_file_info;
    cur_file_info_internalSaved = (*s).cur_file_info_internal;
    err = unzGoToFirstFile(file);
    while err == 0i32 {
        let mut szCurrentFileName: [libc::c_char; 257] = [0; 257];
        err =
            unzGetCurrentFileInfo(file, 0 as *mut unz_file_info,
                                  szCurrentFileName.as_mut_ptr(),
                                  (::std::mem::size_of::<[libc::c_char; 257]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1i32 as
                                                                       libc::c_ulong),
                                  0 as *mut libc::c_void, 0i32 as uLong,
                                  0 as *mut libc::c_char, 0i32 as uLong);
        if err == 0i32 {
            if unzStringFileNameCompare(szCurrentFileName.as_mut_ptr(),
                                        szFileName, iCaseSensitivity) == 0i32
               {
                return 0i32
            }
            err = unzGoToNextFile(file)
        }
    }
    (*s).num_file = num_fileSaved;
    (*s).pos_in_central_dir = pos_in_central_dirSaved;
    (*s).cur_file_info = cur_file_infoSaved;
    (*s).cur_file_info_internal = cur_file_info_internalSaved;
    return err;
}
/* ****************************************** */
#[no_mangle]
pub unsafe extern "C" fn unzGetCurrentFileInfo(mut file: unzFile,
                                               mut pfile_info:
                                                   *mut unz_file_info,
                                               mut szFileName:
                                                   *mut libc::c_char,
                                               mut fileNameBufferSize: uLong,
                                               mut extraField:
                                                   *mut libc::c_void,
                                               mut extraFieldBufferSize:
                                                   uLong,
                                               mut szComment:
                                                   *mut libc::c_char,
                                               mut commentBufferSize: uLong)
 -> libc::c_int {
    return unzlocal_GetCurrentFileInfoInternal(file, pfile_info,
                                               0 as
                                                   *mut unz_file_info_internal,
                                               szFileName, fileNameBufferSize,
                                               extraField,
                                               extraFieldBufferSize,
                                               szComment, commentBufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn unzGetFilePos(mut file: unzFile,
                                       mut file_pos: *mut unz_file_pos)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() || file_pos.is_null() { return -102i32 }
    s = file as *mut unz_s;
    if 0 == (*s).current_file_ok { return -100i32 }
    (*file_pos).pos_in_zip_directory = (*s).pos_in_central_dir;
    (*file_pos).num_of_file = (*s).num_file;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn unzGoToFilePos(mut file: unzFile,
                                        mut file_pos: *mut unz_file_pos)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    if file.is_null() || file_pos.is_null() { return -102i32 }
    s = file as *mut unz_s;
    (*s).pos_in_central_dir = (*file_pos).pos_in_zip_directory;
    (*s).num_file = (*file_pos).num_of_file;
    err =
        unzlocal_GetCurrentFileInfoInternal(file, &mut (*s).cur_file_info,
                                            &mut (*s).cur_file_info_internal,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong,
                                            0 as *mut libc::c_void,
                                            0i32 as uLong,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong);
    (*s).current_file_ok = (err == 0i32) as libc::c_int as uLong;
    return err;
}
/*
  Get Info about the current file
  if pfile_info!=NULL, the *pfile_info structure will contain somes info about
        the current file
  if szFileName!=NULL, the filemane string will be copied in szFileName
            (fileNameBufferSize is the size of the buffer)
  if extraField!=NULL, the extra field information will be copied in extraField
            (extraFieldBufferSize is the size of the buffer).
            This is the Central-header version of the extra field
  if szComment!=NULL, the comment string of the file will be copied in szComment
            (commentBufferSize is the size of the buffer)
*/
/* **************************************************************************/
/* for reading the content of the current zipfile, you can open it, read data
   from it, and close it (you can close it before reading all the file)
   */
#[no_mangle]
pub unsafe extern "C" fn unzOpenCurrentFile(mut file: unzFile)
 -> libc::c_int {
    return unzOpenCurrentFile3(file, 0 as *mut libc::c_int,
                               0 as *mut libc::c_int, 0i32,
                               0 as *const libc::c_char);
}
/*
  Same than unzOpenCurrentFile, but open for read raw the file (not uncompress)
    if raw==1
  *method will receive method of compression, *level will receive level of
     compression
  note : you can set level parameter as NULL (if you did not want known level,
         but you CANNOT set method parameter as NULL
*/
#[no_mangle]
pub unsafe extern "C" fn unzOpenCurrentFile3(mut file: unzFile,
                                             mut method: *mut libc::c_int,
                                             mut level: *mut libc::c_int,
                                             mut raw: libc::c_int,
                                             mut password:
                                                 *const libc::c_char)
 -> libc::c_int {
    let mut err: libc::c_int = 0i32;
    let mut iSizeVar: uInt = 0;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    /* offset of the local extra field */
    let mut offset_local_extrafield: uLong = 0;
    /* size of the local extra field */
    let mut size_local_extrafield: uInt = 0;
    if !password.is_null() { return -102i32 }
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    if 0 == (*s).current_file_ok { return -102i32 }
    if !(*s).pfile_in_zip_read.is_null() { unzCloseCurrentFile(file); }
    if unzlocal_CheckCurrentFileCoherencyHeader(s, &mut iSizeVar,
                                                &mut offset_local_extrafield,
                                                &mut size_local_extrafield) !=
           0i32 {
        return -103i32
    }
    pfile_in_zip_read_info =
        Z_MallocDebug(::std::mem::size_of::<file_in_zip_read_info_s>() as
                          libc::c_ulong as libc::c_int,
                      b"sizeof(file_in_zip_read_info_s)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/unzip.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 1093i32)
            as *mut file_in_zip_read_info_s;
    if pfile_in_zip_read_info.is_null() { return -104i32 }
    (*pfile_in_zip_read_info).read_buffer =
        Z_MallocDebug(16384i32,
                      b"(16384)\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/unzip.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 1097i32)
            as *mut libc::c_char;
    (*pfile_in_zip_read_info).offset_local_extrafield =
        offset_local_extrafield;
    (*pfile_in_zip_read_info).size_local_extrafield = size_local_extrafield;
    (*pfile_in_zip_read_info).pos_local_extrafield = 0i32 as uLong;
    (*pfile_in_zip_read_info).raw = raw;
    if (*pfile_in_zip_read_info).read_buffer.is_null() {
        if !pfile_in_zip_read_info.is_null() {
            Z_Free(pfile_in_zip_read_info as *mut libc::c_void);
        }
        return -104i32
    }
    (*pfile_in_zip_read_info).stream_initialised = 0i32 as uLong;
    if !method.is_null() {
        *method = (*s).cur_file_info.compression_method as libc::c_int
    }
    if !level.is_null() {
        *level = 6i32;
        match (*s).cur_file_info.flag & 0x6i32 as libc::c_ulong {
            6 => { *level = 1i32 }
            4 => { *level = 2i32 }
            2 => { *level = 9i32 }
            _ => { }
        }
    }
    if (*s).cur_file_info.compression_method != 0i32 as libc::c_ulong &&
           (*s).cur_file_info.compression_method != 8i32 as libc::c_ulong {
        err = -103i32
    }
    (*pfile_in_zip_read_info).crc32_wait = (*s).cur_file_info.crc;
    (*pfile_in_zip_read_info).crc32 = 0i32 as uLong;
    (*pfile_in_zip_read_info).compression_method =
        (*s).cur_file_info.compression_method;
    (*pfile_in_zip_read_info).filestream = (*s).filestream;
    (*pfile_in_zip_read_info).z_filefunc = (*s).z_filefunc;
    (*pfile_in_zip_read_info).byte_before_the_zipfile =
        (*s).byte_before_the_zipfile;
    (*pfile_in_zip_read_info).stream.total_out = 0i32 as uLong;
    if (*s).cur_file_info.compression_method == 8i32 as libc::c_ulong &&
           0 == raw {
        (*pfile_in_zip_read_info).stream.zalloc = None;
        (*pfile_in_zip_read_info).stream.zfree = None;
        (*pfile_in_zip_read_info).stream.opaque = 0 as voidpf;
        (*pfile_in_zip_read_info).stream.next_in = 0 as *mut Bytef;
        (*pfile_in_zip_read_info).stream.avail_in = 0i32 as uInt;
        if inflateInit2_(&mut (*pfile_in_zip_read_info).stream, -15i32,
                         b"1.2.11\x00" as *const u8 as *const libc::c_char,
                         ::std::mem::size_of::<z_stream>() as libc::c_ulong as
                             libc::c_int) == 0i32 {
            (*pfile_in_zip_read_info).stream_initialised = 1i32 as uLong
        } else {
            if !pfile_in_zip_read_info.is_null() {
                Z_Free(pfile_in_zip_read_info as *mut libc::c_void);
            }
            return -104i32
        }
    }
    (*pfile_in_zip_read_info).rest_read_compressed =
        (*s).cur_file_info.compressed_size;
    (*pfile_in_zip_read_info).rest_read_uncompressed =
        (*s).cur_file_info.uncompressed_size;
    (*pfile_in_zip_read_info).pos_in_zipfile =
        (*s).cur_file_info_internal.offset_curfile.wrapping_add(0x1ei32 as
                                                                    libc::c_ulong).wrapping_add(iSizeVar
                                                                                                    as
                                                                                                    libc::c_ulong);
    (*pfile_in_zip_read_info).stream.avail_in = 0i32 as uInt;
    (*s).pfile_in_zip_read = pfile_in_zip_read_info;
    return err;
}
/*
// Unzip Helper Functions - should be here?
///////////////////////////////////////////
*/
/*
  Read the local header of the current zipfile
  Check the coherency of the local header and info in the end of central
        directory about this file
  store in *piSizeVar the size of extra info in local header
        (filename and size of extra field data)
*/
unsafe extern "C" fn unzlocal_CheckCurrentFileCoherencyHeader(mut s:
                                                                  *mut unz_s,
                                                              mut piSizeVar:
                                                                  *mut uInt,
                                                              mut poffset_local_extrafield:
                                                                  *mut uLong,
                                                              mut psize_local_extrafield:
                                                                  *mut uInt)
 -> libc::c_int {
    let mut uMagic: uLong = 0;
    let mut uData: uLong = 0;
    let mut uFlags: uLong = 0;
    let mut size_filename: uLong = 0;
    let mut size_extra_field: uLong = 0;
    let mut err: libc::c_int = 0i32;
    *piSizeVar = 0i32 as uInt;
    *poffset_local_extrafield = 0i32 as uLong;
    *psize_local_extrafield = 0i32 as uInt;
    if (*s).z_filefunc.zseek_file.expect("non-null function pointer")((*s).z_filefunc.opaque,
                                                                      (*s).filestream,
                                                                      (*s).cur_file_info_internal.offset_curfile.wrapping_add((*s).byte_before_the_zipfile),
                                                                      0i32) !=
           0i32 as libc::c_long {
        return -1i32
    }
    if err == 0i32 {
        if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream,
                            &mut uMagic) != 0i32 {
            err = -1i32
        } else if uMagic != 0x4034b50i32 as libc::c_ulong { err = -103i32 }
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut uData) !=
           0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut uFlags)
           != 0i32 {
        err = -1i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut uData) !=
           0i32 {
        err = -1i32
    } else if err == 0i32 && uData != (*s).cur_file_info.compression_method {
        err = -103i32
    }
    if err == 0i32 &&
           (*s).cur_file_info.compression_method != 0i32 as libc::c_ulong &&
           (*s).cur_file_info.compression_method != 8i32 as libc::c_ulong {
        err = -103i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) !=
           0i32 {
        err = -1i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) !=
           0i32 {
        err = -1i32
    } else if err == 0i32 && uData != (*s).cur_file_info.crc &&
                  uFlags & 8i32 as libc::c_ulong == 0i32 as libc::c_ulong {
        err = -103i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) !=
           0i32 {
        err = -1i32
    } else if err == 0i32 && uData != (*s).cur_file_info.compressed_size &&
                  uFlags & 8i32 as libc::c_ulong == 0i32 as libc::c_ulong {
        err = -103i32
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) !=
           0i32 {
        err = -1i32
    } else if err == 0i32 && uData != (*s).cur_file_info.uncompressed_size &&
                  uFlags & 8i32 as libc::c_ulong == 0i32 as libc::c_ulong {
        err = -103i32
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut size_filename) != 0i32 {
        err = -1i32
    } else if err == 0i32 && size_filename != (*s).cur_file_info.size_filename
     {
        err = -103i32
    }
    *piSizeVar =
        (*piSizeVar as libc::c_uint).wrapping_add(size_filename as uInt) as
            uInt as uInt;
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream,
                         &mut size_extra_field) != 0i32 {
        err = -1i32
    }
    *poffset_local_extrafield =
        (*s).cur_file_info_internal.offset_curfile.wrapping_add(0x1ei32 as
                                                                    libc::c_ulong).wrapping_add(size_filename);
    *psize_local_extrafield = size_extra_field as uInt;
    *piSizeVar =
        (*piSizeVar as libc::c_uint).wrapping_add(size_extra_field as uInt) as
            uInt as uInt;
    return err;
}
/*
  Open for reading data the current file in the zipfile.
  If there is no error, the return value is UNZ_OK.
*/
#[no_mangle]
pub unsafe extern "C" fn unzOpenCurrentFilePassword(mut file: unzFile,
                                                    mut password:
                                                        *const libc::c_char)
 -> libc::c_int {
    return unzOpenCurrentFile3(file, 0 as *mut libc::c_int,
                               0 as *mut libc::c_int, 0i32, password);
}
/*
  Open for reading data the current file in the zipfile.
  password is a crypting password
  If there is no error, the return value is UNZ_OK.
*/
#[no_mangle]
pub unsafe extern "C" fn unzOpenCurrentFile2(mut file: unzFile,
                                             mut method: *mut libc::c_int,
                                             mut level: *mut libc::c_int,
                                             mut raw: libc::c_int)
 -> libc::c_int {
    return unzOpenCurrentFile3(file, method, level, raw,
                               0 as *const libc::c_char);
}
/*
  Close the file in zip opened with unzOpenCurrentFile
  Return UNZ_CRCERROR if all the file was read but the CRC is not good
*/
#[no_mangle]
pub unsafe extern "C" fn unzReadCurrentFile(mut file: unzFile, mut buf: voidp,
                                            mut len: libc::c_uint)
 -> libc::c_int {
    let mut err: libc::c_int = 0i32;
    let mut iRead: uInt = 0i32 as uInt;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() { return -102i32 }
    if (*pfile_in_zip_read_info).read_buffer.is_null() { return -100i32 }
    if len == 0i32 as libc::c_uint { return 0i32 }
    (*pfile_in_zip_read_info).stream.next_out = buf as *mut Bytef;
    (*pfile_in_zip_read_info).stream.avail_out = len;
    if len as libc::c_ulong > (*pfile_in_zip_read_info).rest_read_uncompressed
           && 0 == (*pfile_in_zip_read_info).raw {
        (*pfile_in_zip_read_info).stream.avail_out =
            (*pfile_in_zip_read_info).rest_read_uncompressed as uInt
    }
    if len as libc::c_ulong >
           (*pfile_in_zip_read_info).rest_read_compressed.wrapping_add((*pfile_in_zip_read_info).stream.avail_in
                                                                           as
                                                                           libc::c_ulong)
           && 0 != (*pfile_in_zip_read_info).raw {
        (*pfile_in_zip_read_info).stream.avail_out =
            ((*pfile_in_zip_read_info).rest_read_compressed as
                 uInt).wrapping_add((*pfile_in_zip_read_info).stream.avail_in)
    }
    while (*pfile_in_zip_read_info).stream.avail_out > 0i32 as libc::c_uint {
        if (*pfile_in_zip_read_info).stream.avail_in == 0i32 as libc::c_uint
               &&
               (*pfile_in_zip_read_info).rest_read_compressed >
                   0i32 as libc::c_ulong {
            let mut uReadThis: uInt = 16384i32 as uInt;
            if (*pfile_in_zip_read_info).rest_read_compressed <
                   uReadThis as libc::c_ulong {
                uReadThis =
                    (*pfile_in_zip_read_info).rest_read_compressed as uInt
            }
            if uReadThis == 0i32 as libc::c_uint { return 0i32 }
            if (*pfile_in_zip_read_info).z_filefunc.zseek_file.expect("non-null function pointer")((*pfile_in_zip_read_info).z_filefunc.opaque,
                                                                                                   (*pfile_in_zip_read_info).filestream,
                                                                                                   (*pfile_in_zip_read_info).pos_in_zipfile.wrapping_add((*pfile_in_zip_read_info).byte_before_the_zipfile),
                                                                                                   0i32)
                   != 0i32 as libc::c_long {
                return -1i32
            }
            if (*pfile_in_zip_read_info).z_filefunc.zread_file.expect("non-null function pointer")((*pfile_in_zip_read_info).z_filefunc.opaque,
                                                                                                   (*pfile_in_zip_read_info).filestream,
                                                                                                   (*pfile_in_zip_read_info).read_buffer
                                                                                                       as
                                                                                                       *mut libc::c_void,
                                                                                                   uReadThis
                                                                                                       as
                                                                                                       uLong)
                   != uReadThis as libc::c_ulong {
                return -1i32
            }
            (*pfile_in_zip_read_info).pos_in_zipfile =
                ((*pfile_in_zip_read_info).pos_in_zipfile as
                     libc::c_ulong).wrapping_add(uReadThis as libc::c_ulong)
                    as uLong as uLong;
            (*pfile_in_zip_read_info).rest_read_compressed =
                ((*pfile_in_zip_read_info).rest_read_compressed as
                     libc::c_ulong).wrapping_sub(uReadThis as libc::c_ulong)
                    as uLong as uLong;
            (*pfile_in_zip_read_info).stream.next_in =
                (*pfile_in_zip_read_info).read_buffer as *mut Bytef;
            (*pfile_in_zip_read_info).stream.avail_in = uReadThis
        }
        if (*pfile_in_zip_read_info).compression_method ==
               0i32 as libc::c_ulong || 0 != (*pfile_in_zip_read_info).raw {
            let mut uDoCopy: uInt = 0;
            let mut i: uInt = 0;
            if (*pfile_in_zip_read_info).stream.avail_in ==
                   0i32 as libc::c_uint &&
                   (*pfile_in_zip_read_info).rest_read_compressed ==
                       0i32 as libc::c_ulong {
                return (if iRead == 0i32 as libc::c_uint {
                            0i32 as libc::c_uint
                        } else { iRead }) as libc::c_int
            }
            if (*pfile_in_zip_read_info).stream.avail_out <
                   (*pfile_in_zip_read_info).stream.avail_in {
                uDoCopy = (*pfile_in_zip_read_info).stream.avail_out
            } else { uDoCopy = (*pfile_in_zip_read_info).stream.avail_in }
            i = 0i32 as uInt;
            while i < uDoCopy {
                *(*pfile_in_zip_read_info).stream.next_out.offset(i as isize)
                    =
                    *(*pfile_in_zip_read_info).stream.next_in.offset(i as
                                                                         isize);
                i = i.wrapping_add(1)
            }
            (*pfile_in_zip_read_info).crc32 =
                crc32((*pfile_in_zip_read_info).crc32,
                      (*pfile_in_zip_read_info).stream.next_out, uDoCopy);
            (*pfile_in_zip_read_info).rest_read_uncompressed =
                ((*pfile_in_zip_read_info).rest_read_uncompressed as
                     libc::c_ulong).wrapping_sub(uDoCopy as libc::c_ulong) as
                    uLong as uLong;
            (*pfile_in_zip_read_info).stream.avail_in =
                ((*pfile_in_zip_read_info).stream.avail_in as
                     libc::c_uint).wrapping_sub(uDoCopy) as uInt as uInt;
            (*pfile_in_zip_read_info).stream.avail_out =
                ((*pfile_in_zip_read_info).stream.avail_out as
                     libc::c_uint).wrapping_sub(uDoCopy) as uInt as uInt;
            (*pfile_in_zip_read_info).stream.next_out =
                (*pfile_in_zip_read_info).stream.next_out.offset(uDoCopy as
                                                                     isize);
            (*pfile_in_zip_read_info).stream.next_in =
                (*pfile_in_zip_read_info).stream.next_in.offset(uDoCopy as
                                                                    isize);
            (*pfile_in_zip_read_info).stream.total_out =
                ((*pfile_in_zip_read_info).stream.total_out as
                     libc::c_ulong).wrapping_add(uDoCopy as libc::c_ulong) as
                    uLong as uLong;
            iRead =
                (iRead as libc::c_uint).wrapping_add(uDoCopy) as uInt as uInt
        } else {
            let mut uTotalOutBefore: uLong = 0;
            let mut uTotalOutAfter: uLong = 0;
            let mut bufBefore: *const Bytef = 0 as *const Bytef;
            let mut uOutThis: uLong = 0;
            let mut flush: libc::c_int = 2i32;
            uTotalOutBefore = (*pfile_in_zip_read_info).stream.total_out;
            bufBefore = (*pfile_in_zip_read_info).stream.next_out;
            err = inflate(&mut (*pfile_in_zip_read_info).stream, flush);
            if err >= 0i32 && !(*pfile_in_zip_read_info).stream.msg.is_null()
               {
                err = -3i32
            }
            uTotalOutAfter = (*pfile_in_zip_read_info).stream.total_out;
            uOutThis = uTotalOutAfter.wrapping_sub(uTotalOutBefore);
            (*pfile_in_zip_read_info).crc32 =
                crc32((*pfile_in_zip_read_info).crc32, bufBefore,
                      uOutThis as uInt);
            (*pfile_in_zip_read_info).rest_read_uncompressed =
                ((*pfile_in_zip_read_info).rest_read_uncompressed as
                     libc::c_ulong).wrapping_sub(uOutThis) as uLong as uLong;
            iRead =
                (iRead as
                     libc::c_uint).wrapping_add(uTotalOutAfter.wrapping_sub(uTotalOutBefore)
                                                    as uInt) as uInt as uInt;
            if err == 1i32 {
                return (if iRead == 0i32 as libc::c_uint {
                            0i32 as libc::c_uint
                        } else { iRead }) as libc::c_int
            }
            if err != 0i32 { break ; }
        }
    }
    if err == 0i32 { return iRead as libc::c_int }
    return err;
}
/*
  Read bytes from the current file (opened by unzOpenCurrentFile)
  buf contain buffer where data must be copied
  len the size of buf.

  return the number of byte copied if somes bytes are copied
  return 0 if the end of file was reached
  return <0 with error code if there is an error
    (UNZ_ERRNO for IO error, or zLib error for uncompress error)
*/
#[no_mangle]
pub unsafe extern "C" fn unztell(mut file: unzFile) -> off_t {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() { return -102i32 as off_t }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() { return -102i32 as off_t }
    return (*pfile_in_zip_read_info).stream.total_out as off_t;
}
/*
  Give the current position in uncompressed data
*/
#[no_mangle]
pub unsafe extern "C" fn unzeof(mut file: unzFile) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() { return -102i32 }
    if (*pfile_in_zip_read_info).rest_read_uncompressed ==
           0i32 as libc::c_ulong {
        return 1i32
    } else { return 0i32 };
}
/*
  return 1 if the end of file was reached, 0 elsewhere
*/
#[no_mangle]
pub unsafe extern "C" fn unzGetLocalExtrafield(mut file: unzFile,
                                               mut buf: voidp,
                                               mut len: libc::c_uint)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    let mut read_now: uInt = 0;
    let mut size_to_read: uLong = 0;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() { return -102i32 }
    size_to_read =
        ((*pfile_in_zip_read_info).size_local_extrafield as
             libc::c_ulong).wrapping_sub((*pfile_in_zip_read_info).pos_local_extrafield);
    if buf.is_null() { return size_to_read as libc::c_int }
    if len as libc::c_ulong > size_to_read {
        read_now = size_to_read as uInt
    } else { read_now = len }
    if read_now == 0i32 as libc::c_uint { return 0i32 }
    if (*pfile_in_zip_read_info).z_filefunc.zseek_file.expect("non-null function pointer")((*pfile_in_zip_read_info).z_filefunc.opaque,
                                                                                           (*pfile_in_zip_read_info).filestream,
                                                                                           (*pfile_in_zip_read_info).offset_local_extrafield.wrapping_add((*pfile_in_zip_read_info).pos_local_extrafield),
                                                                                           0i32)
           != 0i32 as libc::c_long {
        return -1i32
    }
    if (*pfile_in_zip_read_info).z_filefunc.zread_file.expect("non-null function pointer")((*pfile_in_zip_read_info).z_filefunc.opaque,
                                                                                           (*pfile_in_zip_read_info).filestream,
                                                                                           buf,
                                                                                           read_now
                                                                                               as
                                                                                               uLong)
           != read_now as libc::c_ulong {
        return -1i32
    }
    return read_now as libc::c_int;
}
/*
  Read extra field from the current file (opened by unzOpenCurrentFile)
  This is the local-header version of the extra field (sometimes, there is
    more info in the local-header version than in the central-header)

  if buf==NULL, it return the size of the local extra field

  if buf!=NULL, len is the size of the buffer, the extra header is copied in
    buf.
  the return value is the number of bytes copied in buf, or (if <0)
    the error code
*/
/* **************************************************************************/
/* Get the current file offset */
#[no_mangle]
pub unsafe extern "C" fn unzGetOffset(mut file: unzFile) -> uLong {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() { return -102i32 as uLong }
    s = file as *mut unz_s;
    if 0 == (*s).current_file_ok { return 0i32 as uLong }
    if (*s).gi.number_entry != 0i32 as libc::c_ulong &&
           (*s).gi.number_entry != 0xffffi32 as libc::c_ulong {
        if (*s).num_file == (*s).gi.number_entry { return 0i32 as uLong }
    }
    return (*s).pos_in_central_dir;
}
/* Set the current file offset */
#[no_mangle]
pub unsafe extern "C" fn unzSetOffset(mut file: unzFile, mut pos: uLong)
 -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    if file.is_null() { return -102i32 }
    s = file as *mut unz_s;
    (*s).pos_in_central_dir = pos;
    (*s).num_file = (*s).gi.number_entry;
    err =
        unzlocal_GetCurrentFileInfoInternal(file, &mut (*s).cur_file_info,
                                            &mut (*s).cur_file_info_internal,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong,
                                            0 as *mut libc::c_void,
                                            0i32 as uLong,
                                            0 as *mut libc::c_char,
                                            0i32 as uLong);
    (*s).current_file_ok = (err == 0i32) as libc::c_int as uLong;
    return err;
}