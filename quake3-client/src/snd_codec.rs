use libc;
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type fileHandle_t = libc::c_int;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn COM_GetExtension(name: *const libc::c_char)
         -> *const libc::c_char;
        #[no_mangle]
        pub fn COM_StripExtension(in_0: *const libc::c_char,
                                  out: *mut libc::c_char,
                                  destsize: libc::c_int);
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_codec.h"]
pub mod snd_codec_h {
    /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_info_s {
        pub rate: libc::c_int,
        pub width: libc::c_int,
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub size: libc::c_int,
        pub dataofs: libc::c_int,
    }
    pub type snd_info_t = snd_info_s;
    // Codec data structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_codec_s {
        pub ext: *mut libc::c_char,
        pub load: CODEC_LOAD,
        pub open: CODEC_OPEN,
        pub read: CODEC_READ,
        pub close: CODEC_CLOSE,
        pub next: *mut snd_codec_t,
    }
    pub type snd_codec_t = snd_codec_s;
    pub type CODEC_CLOSE
        =
        Option<unsafe extern "C" fn(_: *mut snd_stream_t) -> ()>;
    pub type snd_stream_t = snd_stream_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_stream_s {
        pub codec: *mut snd_codec_t,
        pub file: fileHandle_t,
        pub info: snd_info_t,
        pub length: libc::c_int,
        pub pos: libc::c_int,
        pub ptr: *mut libc::c_void,
    }
    pub type CODEC_READ
        =
        Option<unsafe extern "C" fn(_: *mut snd_stream_t, _: libc::c_int,
                                    _: *mut libc::c_void) -> libc::c_int>;
    pub type CODEC_OPEN
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char)
                   -> *mut snd_stream_t>;
    // Codec functions
    pub type CODEC_LOAD
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char,
                                    _: *mut snd_info_t) -> *mut libc::c_void>;
    use super::{libc};
    use super::q_shared_h::{fileHandle_t};
    extern "C" {
        // WAV Codec
        #[no_mangle]
        pub static mut wav_codec: snd_codec_t;
        // Ogg Vorbis codec
        #[no_mangle]
        pub static mut ogg_codec: snd_codec_t;
        // USE_CODEC_VORBIS
        // Ogg Opus codec
        #[no_mangle]
        pub static mut opus_codec: snd_codec_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    use super::q_shared_h::{fileHandle_t, qboolean};
    extern "C" {
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_codec.c"]
pub mod snd_codec_c { }
use self::q_shared_h::{qboolean, qtrue, qfalse, fileHandle_t,
                       COM_GetExtension, COM_StripExtension, Com_sprintf,
                       Q_stricmp, Q_strncpyz, Com_Printf};
use self::snd_codec_h::{snd_info_s, snd_info_t, snd_codec_s, snd_codec_t,
                        CODEC_CLOSE, snd_stream_t, snd_stream_s, CODEC_READ,
                        CODEC_OPEN, CODEC_LOAD, wav_codec, ogg_codec,
                        opus_codec};
use self::qcommon_h::{FS_FOpenFileRead, FS_FCloseFile, Com_DPrintf,
                      Z_MallocDebug, Z_Free};
// Codec management
#[no_mangle]
pub unsafe extern "C" fn S_CodecInit() {
    codecs = 0 as *mut snd_codec_t;
    S_CodecRegister(&mut opus_codec);
    S_CodecRegister(&mut ogg_codec);
    S_CodecRegister(&mut wav_codec);
}
#[no_mangle]
pub unsafe extern "C" fn S_CodecRegister(mut codec: *mut snd_codec_t) {
    (*codec).next = codecs;
    codecs = codec;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
static mut codecs: *mut snd_codec_t =
    0 as *const snd_codec_t as *mut snd_codec_t;
#[no_mangle]
pub unsafe extern "C" fn S_CodecShutdown() { codecs = 0 as *mut snd_codec_t; }
#[no_mangle]
pub unsafe extern "C" fn S_CodecLoad(mut filename: *const libc::c_char,
                                     mut info: *mut snd_info_t)
 -> *mut libc::c_void {
    return S_CodecGetSound(filename, info);
}
/*
=================
S_CodecGetSound

Opens/loads a sound, tries codec based on the sound's file extension
then tries all supported codecs.
=================
*/
unsafe extern "C" fn S_CodecGetSound(mut filename: *const libc::c_char,
                                     mut info: *mut snd_info_t)
 -> *mut libc::c_void {
    let mut codec: *mut snd_codec_t = 0 as *mut snd_codec_t;
    let mut orgCodec: *mut snd_codec_t = 0 as *mut snd_codec_t;
    let mut orgNameFailed: qboolean = qfalse;
    let mut localName: [libc::c_char; 64] = [0; 64];
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut altName: [libc::c_char; 64] = [0; 64];
    let mut rtn: *mut libc::c_void = 0 as *mut libc::c_void;
    Q_strncpyz(localName.as_mut_ptr(), filename, 64i32);
    ext = COM_GetExtension(localName.as_mut_ptr());
    if 0 != *ext {
        codec = codecs;
        while !codec.is_null() {
            if 0 == Q_stricmp(ext, (*codec).ext) {
                if !info.is_null() {
                    rtn =
                        (*codec).load.expect("non-null function pointer")(localName.as_mut_ptr(),
                                                                          info)
                } else {
                    rtn =
                        (*codec).open.expect("non-null function pointer")(localName.as_mut_ptr())
                            as *mut libc::c_void
                }
                break ;
            } else { codec = (*codec).next }
        }
        if !codec.is_null() {
            if rtn.is_null() {
                orgNameFailed = qtrue;
                orgCodec = codec;
                COM_StripExtension(filename, localName.as_mut_ptr(), 64i32);
            } else { return rtn }
        }
    }
    codec = codecs;
    while !codec.is_null() {
        if !(codec == orgCodec) {
            Com_sprintf(altName.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as
                            libc::c_ulong as libc::c_int,
                        b"%s.%s\x00" as *const u8 as *const libc::c_char,
                        localName.as_mut_ptr(), (*codec).ext);
            if !info.is_null() {
                rtn =
                    (*codec).load.expect("non-null function pointer")(altName.as_mut_ptr(),
                                                                      info)
            } else {
                rtn =
                    (*codec).open.expect("non-null function pointer")(altName.as_mut_ptr())
                        as *mut libc::c_void
            }
            if !rtn.is_null() {
                if 0 != orgNameFailed as u64 {
                    Com_DPrintf(b"^3WARNING: %s not present, using %s instead\n\x00"
                                    as *const u8 as *const libc::c_char,
                                filename, altName.as_mut_ptr());
                }
                return rtn
            }
        }
        codec = (*codec).next
    }
    Com_Printf(b"^3WARNING: Failed to %s sound %s!\n\x00" as *const u8 as
                   *const libc::c_char,
               if !info.is_null() {
                   b"load\x00" as *const u8 as *const libc::c_char
               } else { b"open\x00" as *const u8 as *const libc::c_char },
               filename);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn S_CodecOpenStream(mut filename: *const libc::c_char)
 -> *mut snd_stream_t {
    return S_CodecGetSound(filename, 0 as *mut snd_info_t) as
               *mut snd_stream_t;
}
#[no_mangle]
pub unsafe extern "C" fn S_CodecCloseStream(mut stream: *mut snd_stream_t) {
    (*(*stream).codec).close.expect("non-null function pointer")(stream);
}
#[no_mangle]
pub unsafe extern "C" fn S_CodecReadStream(mut stream: *mut snd_stream_t,
                                           mut bytes: libc::c_int,
                                           mut buffer: *mut libc::c_void)
 -> libc::c_int {
    return (*(*stream).codec).read.expect("non-null function pointer")(stream,
                                                                       bytes,
                                                                       buffer);
}
// Util functions (used by codecs)
#[no_mangle]
pub unsafe extern "C" fn S_CodecUtilOpen(mut filename: *const libc::c_char,
                                         mut codec: *mut snd_codec_t)
 -> *mut snd_stream_t {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut hnd: fileHandle_t = 0;
    let mut length: libc::c_int = 0;
    length = FS_FOpenFileRead(filename, &mut hnd, qtrue) as libc::c_int;
    if 0 == hnd {
        Com_DPrintf(b"Can\'t read sound file %s\n\x00" as *const u8 as
                        *const libc::c_char, filename);
        return 0 as *mut snd_stream_t
    }
    stream =
        Z_MallocDebug(::std::mem::size_of::<snd_stream_t>() as libc::c_ulong
                          as libc::c_int,
                      b"sizeof(snd_stream_t)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/client/snd_codec.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 213i32) as
            *mut snd_stream_t;
    if stream.is_null() { FS_FCloseFile(hnd); return 0 as *mut snd_stream_t }
    (*stream).codec = codec;
    (*stream).file = hnd;
    (*stream).length = length;
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn S_CodecUtilClose(mut stream:
                                              *mut *mut snd_stream_t) {
    FS_FCloseFile((**stream).file);
    Z_Free(*stream as *mut libc::c_void);
    *stream = 0 as *mut snd_stream_t;
}