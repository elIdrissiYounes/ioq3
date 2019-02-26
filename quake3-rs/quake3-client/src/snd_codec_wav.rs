#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
extern crate libc;
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

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
//
    // q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
  VM Considerations

  The VM can not use the standard system headers because we aren't really
  using the compiler they were meant for.  We use bg_lib.h which contains
  prototypes for the functions we define for our own use in bg_lib.c.

  When writing mods, please add needed headers HERE, do not start including
  stuff like <stdio.h> in the various .c files that make up each of the VMs
  since you will be including system headers files can will have issues.

  Remember, if you use a C library function that is not defined in bg_lib.c,
  you will have to add your own version for support in the VM.

 **********************************************************************/
    //=============================================================
    pub type byte = libc::c_uchar;
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type fileHandle_t = libc::c_int;
    pub type unnamed = libc::c_uint;
    pub const FS_SEEK_SET: unnamed = 2;
    pub const FS_SEEK_END: unnamed = 1;
    pub const FS_SEEK_CUR: unnamed = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                         n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/client/snd_codec.h"]
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
        // Util functions (used by codecs)
        #[no_mangle]
        pub fn S_CodecUtilOpen(filename: *const libc::c_char,
                               codec: *mut snd_codec_t) -> *mut snd_stream_t;
        #[no_mangle]
        pub fn S_CodecUtilClose(stream: *mut *mut snd_stream_t);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    use super::q_shared_h::{fileHandle_t, qboolean};
    extern "C" {
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        // opens a file for reading, writing, or appending depending on the value of mode
        #[no_mangle]
        pub fn FS_Seek(f: fileHandle_t, offset: libc::c_long,
                       origin: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/client/snd_codec_wav.c"]
pub mod snd_codec_wav_c { }
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       FS_SEEK_SET, FS_SEEK_END, FS_SEEK_CUR, Q_strncmp,
                       Com_Printf};
use self::snd_codec_h::{snd_info_s, snd_info_t, snd_codec_s, snd_codec_t,
                        CODEC_CLOSE, snd_stream_t, snd_stream_s, CODEC_READ,
                        CODEC_OPEN, CODEC_LOAD, S_CodecUtilOpen,
                        S_CodecUtilClose};
use self::qcommon_h::{FS_FOpenFileRead, FS_Read, FS_FCloseFile, FS_Seek,
                      Hunk_AllocateTempMemory};
// WAV Codec
#[no_mangle]
pub static mut wav_codec: snd_codec_t =
    snd_codec_s{ext:
                    b"wav\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                load: Some(S_WAV_CodecLoad),
                open: Some(S_WAV_CodecOpenStream),
                read: Some(S_WAV_CodecReadStream),
                close: Some(S_WAV_CodecCloseStream),
                next: 0 as *const snd_codec_t as *mut snd_codec_t,};
#[no_mangle]
pub unsafe extern "C" fn S_WAV_CodecCloseStream(mut stream:
                                                    *mut snd_stream_t) {
    S_CodecUtilClose(&mut stream);
}
#[no_mangle]
pub unsafe extern "C" fn S_WAV_CodecReadStream(mut stream: *mut snd_stream_t,
                                               mut bytes: libc::c_int,
                                               mut buffer: *mut libc::c_void)
 -> libc::c_int {
    let mut remaining: libc::c_int = (*stream).info.size - (*stream).pos;
    let mut samples: libc::c_int = 0;
    if remaining <= 0i32 { return 0i32 }
    if bytes > remaining { bytes = remaining }
    (*stream).pos += bytes;
    samples = bytes / (*stream).info.width / (*stream).info.channels;
    FS_Read(buffer, bytes, (*stream).file);
    S_ByteSwapRawSamples(samples, (*stream).info.width,
                         (*stream).info.channels, buffer as *const byte);
    return bytes;
}
/*
=================
S_ByteSwapRawSamples
=================
*/
unsafe extern "C" fn S_ByteSwapRawSamples(mut samples: libc::c_int,
                                          mut width: libc::c_int,
                                          mut s_channels: libc::c_int,
                                          mut data: *const byte) {
    let mut i: libc::c_int = 0;
    if width != 2i32 { return }
    if 256i32 == 256i32 { return }
    if s_channels == 2i32 { samples <<= 1i32 }
    i = 0i32;
    while i < samples {
        *(data as *mut libc::c_short).offset(i as isize) =
            *(data as *mut libc::c_short).offset(i as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_WAV_CodecOpenStream(mut filename:
                                                   *const libc::c_char)
 -> *mut snd_stream_t {
    let mut rv: *mut snd_stream_t = 0 as *mut snd_stream_t;
    rv = S_CodecUtilOpen(filename, &mut wav_codec);
    if rv.is_null() { return 0 as *mut snd_stream_t }
    if 0 == S_ReadRIFFHeader((*rv).file, &mut (*rv).info) as u64 {
        S_CodecUtilClose(&mut rv);
        return 0 as *mut snd_stream_t
    }
    return rv;
}
/*
=================
S_ReadRIFFHeader
=================
*/
unsafe extern "C" fn S_ReadRIFFHeader(mut file: fileHandle_t,
                                      mut info: *mut snd_info_t) -> qboolean {
    let mut dump: [libc::c_char; 16] = [0; 16];
    let mut bits: libc::c_int = 0;
    let mut fmtlen: libc::c_int = 0i32;
    FS_Read(dump.as_mut_ptr() as *mut libc::c_void, 12i32, file);
    fmtlen =
        S_FindRIFFChunk(file,
                        b"fmt \x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char);
    if fmtlen < 0i32 {
        Com_Printf(b"^1ERROR: Couldn\'t find \"fmt\" chunk\n\x00" as *const u8
                       as *const libc::c_char);
        return qfalse
    }
    FGetLittleShort(file);
    (*info).channels = FGetLittleShort(file) as libc::c_int;
    (*info).rate = FGetLittleLong(file);
    FGetLittleLong(file);
    FGetLittleShort(file);
    bits = FGetLittleShort(file) as libc::c_int;
    if bits < 8i32 {
        Com_Printf(b"^1ERROR: Less than 8 bit sound is not supported\n\x00" as
                       *const u8 as *const libc::c_char);
        return qfalse
    }
    (*info).width = bits / 8i32;
    (*info).dataofs = 0i32;
    if fmtlen > 16i32 {
        fmtlen -= 16i32;
        FS_Seek(file, fmtlen as libc::c_long, FS_SEEK_CUR as libc::c_int);
    }
    (*info).size =
        S_FindRIFFChunk(file,
                        b"data\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char);
    if (*info).size < 0i32 {
        Com_Printf(b"^1ERROR: Couldn\'t find \"data\" chunk\n\x00" as
                       *const u8 as *const libc::c_char);
        return qfalse
    }
    (*info).samples = (*info).size / (*info).width / (*info).channels;
    return qtrue;
}
/*
=================
S_FindRIFFChunk

Returns the length of the data in the chunk, or -1 if not found
=================
*/
unsafe extern "C" fn S_FindRIFFChunk(mut f: fileHandle_t,
                                     mut chunk: *mut libc::c_char)
 -> libc::c_int {
    let mut name: [libc::c_char; 5] = [0; 5];
    let mut len: libc::c_int = 0;
    loop  {
        len = S_ReadChunkInfo(f, name.as_mut_ptr());
        if !(len >= 0i32) { break ; }
        if 0 == Q_strncmp(name.as_mut_ptr(), chunk, 4i32) { return len }
        len = len + 2i32 - 1i32 & !(2i32 - 1i32);
        FS_Seek(f, len as libc::c_long, FS_SEEK_CUR as libc::c_int);
    }
    return -1i32;
}
/*
=================
S_ReadChunkInfo
=================
*/
unsafe extern "C" fn S_ReadChunkInfo(mut f: fileHandle_t,
                                     mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    *name.offset(4isize) = 0i32 as libc::c_char;
    r = FS_Read(name as *mut libc::c_void, 4i32, f);
    if r != 4i32 { return -1i32 }
    len = FGetLittleLong(f);
    if len < 0i32 {
        Com_Printf(b"^3WARNING: Negative chunk length\n\x00" as *const u8 as
                       *const libc::c_char);
        return -1i32
    }
    return len;
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
/*
=================
FGetLittleLong
=================
*/
unsafe extern "C" fn FGetLittleLong(mut f: fileHandle_t) -> libc::c_int {
    let mut v: libc::c_int = 0;
    FS_Read(&mut v as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                libc::c_int, f);
    return v;
}
/*
=================
FGetLittleShort
=================
*/
unsafe extern "C" fn FGetLittleShort(mut f: fileHandle_t) -> libc::c_short {
    let mut v: libc::c_short = 0;
    FS_Read(&mut v as *mut libc::c_short as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as
                libc::c_int, f);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn S_WAV_CodecLoad(mut filename: *const libc::c_char,
                                         mut info: *mut snd_info_t)
 -> *mut libc::c_void {
    let mut file: fileHandle_t = 0;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    FS_FOpenFileRead(filename, &mut file, qtrue);
    if 0 == file { return 0 as *mut libc::c_void }
    if 0 == S_ReadRIFFHeader(file, info) as u64 {
        FS_FCloseFile(file);
        Com_Printf(b"^1ERROR: Incorrect/unsupported format in \"%s\"\n\x00" as
                       *const u8 as *const libc::c_char, filename);
        return 0 as *mut libc::c_void
    }
    buffer = Hunk_AllocateTempMemory((*info).size);
    if buffer.is_null() {
        FS_FCloseFile(file);
        Com_Printf(b"^1ERROR: Out of memory reading \"%s\"\n\x00" as *const u8
                       as *const libc::c_char, filename);
        return 0 as *mut libc::c_void
    }
    FS_Read(buffer, (*info).size, file);
    S_ByteSwapRawSamples((*info).samples, (*info).width, (*info).channels,
                         buffer as *mut byte);
    FS_FCloseFile(file);
    return buffer;
}