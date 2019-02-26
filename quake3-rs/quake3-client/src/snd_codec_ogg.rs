#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __int64_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int64_t = __int64_t;
    use super::types_h::{__int64_t};
}
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
    pub type fileHandle_t = libc::c_int;
    pub type unnamed = libc::c_uint;
    pub const FS_SEEK_SET: unnamed = 2;
    pub const FS_SEEK_END: unnamed = 1;
    pub const FS_SEEK_CUR: unnamed = 0;
    use super::{libc};
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
#[header_src = "/usr/include/vorbis/vorbisfile.h"]
pub mod vorbisfile_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct OggVorbis_File {
        pub datasource: *mut libc::c_void,
        pub seekable: libc::c_int,
        pub offset: ogg_int64_t,
        pub end: ogg_int64_t,
        pub oy: ogg_sync_state,
        pub links: libc::c_int,
        pub offsets: *mut ogg_int64_t,
        pub dataoffsets: *mut ogg_int64_t,
        pub serialnos: *mut libc::c_long,
        pub pcmlengths: *mut ogg_int64_t,
        pub vi: *mut vorbis_info,
        pub vc: *mut vorbis_comment,
        pub pcm_offset: ogg_int64_t,
        pub ready_state: libc::c_int,
        pub current_serialno: libc::c_long,
        pub current_link: libc::c_int,
        pub bittrack: libc::c_double,
        pub samptrack: libc::c_double,
        pub os: ogg_stream_state,
        pub vd: vorbis_dsp_state,
        pub vb: vorbis_block,
        pub callbacks: ov_callbacks,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ov_callbacks {
        pub read_func: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: size_t, _: size_t,
                                                   _: *mut libc::c_void)
                                  -> size_t>,
        pub seek_func: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: ogg_int64_t,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
        pub close_func: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> libc::c_int>,
        pub tell_func: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> libc::c_long>,
    }
    use super::{libc};
    use super::config_types_h::{ogg_int64_t};
    use super::ogg_h::{ogg_sync_state, ogg_stream_state};
    use super::codec_h::{vorbis_info, vorbis_comment, vorbis_dsp_state,
                         vorbis_block};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn ov_clear(vf: *mut OggVorbis_File) -> libc::c_int;
        #[no_mangle]
        pub fn ov_read(vf: *mut OggVorbis_File, buffer: *mut libc::c_char,
                       length: libc::c_int, bigendianp: libc::c_int,
                       word: libc::c_int, sgned: libc::c_int,
                       bitstream: *mut libc::c_int) -> libc::c_long;
        #[no_mangle]
        pub fn ov_pcm_total(vf: *mut OggVorbis_File, i: libc::c_int)
         -> ogg_int64_t;
        #[no_mangle]
        pub fn ov_info(vf: *mut OggVorbis_File, link: libc::c_int)
         -> *mut vorbis_info;
        #[no_mangle]
        pub fn ov_streams(vf: *mut OggVorbis_File) -> libc::c_long;
        #[no_mangle]
        pub fn ov_seekable(vf: *mut OggVorbis_File) -> libc::c_long;
        #[no_mangle]
        pub fn ov_open_callbacks(datasource: *mut libc::c_void,
                                 vf: *mut OggVorbis_File,
                                 initial: *const libc::c_char,
                                 ibytes: libc::c_long,
                                 callbacks: ov_callbacks) -> libc::c_int;
    }
}
#[header_src = "/usr/include/ogg/config_types.h"]
pub mod config_types_h {
    pub type ogg_int64_t = int64_t;
    use super::stdint_intn_h::{int64_t};
}
#[header_src = "/usr/include/vorbis/codec.h"]
pub mod codec_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vorbis_block {
        pub pcm: *mut *mut libc::c_float,
        pub opb: oggpack_buffer,
        pub lW: libc::c_long,
        pub W: libc::c_long,
        pub nW: libc::c_long,
        pub pcmend: libc::c_int,
        pub mode: libc::c_int,
        pub eofflag: libc::c_int,
        pub granulepos: ogg_int64_t,
        pub sequence: ogg_int64_t,
        pub vd: *mut vorbis_dsp_state,
        pub localstore: *mut libc::c_void,
        pub localtop: libc::c_long,
        pub localalloc: libc::c_long,
        pub totaluse: libc::c_long,
        pub reap: *mut alloc_chain,
        pub glue_bits: libc::c_long,
        pub time_bits: libc::c_long,
        pub floor_bits: libc::c_long,
        pub res_bits: libc::c_long,
        pub internal: *mut libc::c_void,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct alloc_chain {
        pub ptr: *mut libc::c_void,
        pub next: *mut alloc_chain,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vorbis_dsp_state {
        pub analysisp: libc::c_int,
        pub vi: *mut vorbis_info,
        pub pcm: *mut *mut libc::c_float,
        pub pcmret: *mut *mut libc::c_float,
        pub pcm_storage: libc::c_int,
        pub pcm_current: libc::c_int,
        pub pcm_returned: libc::c_int,
        pub preextrapolate: libc::c_int,
        pub eofflag: libc::c_int,
        pub lW: libc::c_long,
        pub W: libc::c_long,
        pub nW: libc::c_long,
        pub centerW: libc::c_long,
        pub granulepos: ogg_int64_t,
        pub sequence: ogg_int64_t,
        pub glue_bits: ogg_int64_t,
        pub time_bits: ogg_int64_t,
        pub floor_bits: ogg_int64_t,
        pub res_bits: ogg_int64_t,
        pub backend_state: *mut libc::c_void,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vorbis_info {
        pub version: libc::c_int,
        pub channels: libc::c_int,
        pub rate: libc::c_long,
        pub bitrate_upper: libc::c_long,
        pub bitrate_nominal: libc::c_long,
        pub bitrate_lower: libc::c_long,
        pub bitrate_window: libc::c_long,
        pub codec_setup: *mut libc::c_void,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vorbis_comment {
        pub user_comments: *mut *mut libc::c_char,
        pub comment_lengths: *mut libc::c_int,
        pub comments: libc::c_int,
        pub vendor: *mut libc::c_char,
    }
    use super::{libc};
    use super::ogg_h::{oggpack_buffer};
    use super::config_types_h::{ogg_int64_t};
}
#[header_src = "/usr/include/ogg/ogg.h"]
pub mod ogg_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct oggpack_buffer {
        pub endbyte: libc::c_long,
        pub endbit: libc::c_int,
        pub buffer: *mut libc::c_uchar,
        pub ptr: *mut libc::c_uchar,
        pub storage: libc::c_long,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ogg_stream_state {
        pub body_data: *mut libc::c_uchar,
        pub body_storage: libc::c_long,
        pub body_fill: libc::c_long,
        pub body_returned: libc::c_long,
        pub lacing_vals: *mut libc::c_int,
        pub granule_vals: *mut ogg_int64_t,
        pub lacing_storage: libc::c_long,
        pub lacing_fill: libc::c_long,
        pub lacing_packet: libc::c_long,
        pub lacing_returned: libc::c_long,
        pub header: [libc::c_uchar; 282],
        pub header_fill: libc::c_int,
        pub e_o_s: libc::c_int,
        pub b_o_s: libc::c_int,
        pub serialno: libc::c_long,
        pub pageno: libc::c_long,
        pub packetno: ogg_int64_t,
        pub granulepos: ogg_int64_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ogg_sync_state {
        pub data: *mut libc::c_uchar,
        pub storage: libc::c_int,
        pub fill: libc::c_int,
        pub returned: libc::c_int,
        pub unsynced: libc::c_int,
        pub headerbytes: libc::c_int,
        pub bodybytes: libc::c_int,
    }
    use super::{libc};
    use super::config_types_h::{ogg_int64_t};
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    use super::q_shared_h::{fileHandle_t};
    extern "C" {
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // doesn't work for files that are opened from a pack file
        #[no_mangle]
        pub fn FS_FTell(f: fileHandle_t) -> libc::c_int;
        // opens a file for reading, writing, or appending depending on the value of mode
        #[no_mangle]
        pub fn FS_Seek(f: fileHandle_t, offset: libc::c_long,
                       origin: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/client/snd_codec_ogg.c"]
pub mod snd_codec_ogg_c {
    use super::vorbisfile_h::{ov_callbacks};
    use super::{libc};
    use super::config_types_h::{ogg_int64_t};
    use super::stddef_h::{size_t};
}
#[header_src = "/usr/include/errno.h"]
pub mod errno_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
use self::types_h::{__int64_t};
use self::stddef_h::{size_t};
use self::stdint_intn_h::{int64_t};
use self::q_shared_h::{byte, fileHandle_t, unnamed, FS_SEEK_SET, FS_SEEK_END,
                       FS_SEEK_CUR};
use self::snd_codec_h::{snd_info_s, snd_info_t, snd_codec_s, snd_codec_t,
                        CODEC_CLOSE, snd_stream_t, snd_stream_s, CODEC_READ,
                        CODEC_OPEN, CODEC_LOAD, S_CodecUtilOpen,
                        S_CodecUtilClose};
use self::vorbisfile_h::{OggVorbis_File, ov_callbacks, ov_clear, ov_read,
                         ov_pcm_total, ov_info, ov_streams, ov_seekable,
                         ov_open_callbacks};
use self::config_types_h::{ogg_int64_t};
use self::codec_h::{vorbis_block, alloc_chain, vorbis_dsp_state, vorbis_info,
                    vorbis_comment};
use self::ogg_h::{oggpack_buffer, ogg_stream_state, ogg_sync_state};
use self::qcommon_h::{FS_Read, FS_FTell, FS_Seek, Z_MallocDebug, Z_Free,
                      Hunk_AllocateTempMemory, Hunk_FreeTempMemory};
use self::errno_h::{__errno_location};
// Ogg Vorbis codec
#[no_mangle]
pub static mut ogg_codec: snd_codec_t =
    snd_codec_s{ext:
                    b"ogg\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                load: Some(S_OGG_CodecLoad),
                open: Some(S_OGG_CodecOpenStream),
                read: Some(S_OGG_CodecReadStream),
                close: Some(S_OGG_CodecCloseStream),
                next: 0 as *const snd_codec_t as *mut snd_codec_t,};
#[no_mangle]
pub unsafe extern "C" fn S_OGG_CodecCloseStream(mut stream:
                                                    *mut snd_stream_t) {
    if stream.is_null() { return }
    ov_clear((*stream).ptr as *mut OggVorbis_File);
    Z_Free((*stream).ptr);
    S_CodecUtilClose(&mut stream);
}
#[no_mangle]
pub unsafe extern "C" fn S_OGG_CodecReadStream(mut stream: *mut snd_stream_t,
                                               mut bytes: libc::c_int,
                                               mut buffer: *mut libc::c_void)
 -> libc::c_int {
    // buffer handling
    let mut bytesRead: libc::c_int = 0;
    let mut bytesLeft: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut bufPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    // Bitstream for the decoder
    let mut BS: libc::c_int = 0i32;
    // big endian machines want their samples in big endian order
    let mut IsBigEndian: libc::c_int = 0i32;
    if !(!stream.is_null() && !buffer.is_null()) { return 0i32 }
    if bytes <= 0i32 { return 0i32 }
    bytesRead = 0i32;
    bytesLeft = bytes;
    bufPtr = buffer as *mut libc::c_char;
    while 0 != -1i32 {
        c =
            ov_read((*stream).ptr as *mut OggVorbis_File, bufPtr, bytesLeft,
                    IsBigEndian, 2i32, 1i32, &mut BS) as libc::c_int;
        // no more bytes are left
        if c <= 0i32 { break ; }
        bytesRead += c;
        bytesLeft -= c;
        bufPtr = bufPtr.offset(c as isize);
        // we have enough bytes
        if bytesLeft <= 0i32 { break ; }
    }
    return bytesRead;
}
#[no_mangle]
pub unsafe extern "C" fn S_OGG_CodecOpenStream(mut filename:
                                                   *const libc::c_char)
 -> *mut snd_stream_t {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    // OGG codec control structure
    let mut vf: *mut OggVorbis_File = 0 as *mut OggVorbis_File;
    // some variables used to get informations about the OGG 
    let mut OGGInfo: *mut vorbis_info = 0 as *mut vorbis_info;
    let mut numSamples: ogg_int64_t = 0;
    if filename.is_null() { return 0 as *mut snd_stream_t }
    stream = S_CodecUtilOpen(filename, &mut ogg_codec);
    if stream.is_null() { return 0 as *mut snd_stream_t }
    vf =
        Z_MallocDebug(::std::mem::size_of::<OggVorbis_File>() as libc::c_ulong
                          as libc::c_int,
                      b"sizeof(OggVorbis_File)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/client/snd_codec_ogg.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 253i32) as
            *mut OggVorbis_File;
    if vf.is_null() {
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    if ov_open_callbacks(stream as *mut libc::c_void, vf,
                         0 as *const libc::c_char, 0i32 as libc::c_long,
                         S_OGG_Callbacks) != 0i32 {
        Z_Free(vf as *mut libc::c_void);
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    if 0 == ov_seekable(vf) {
        ov_clear(vf);
        Z_Free(vf as *mut libc::c_void);
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    if ov_streams(vf) != 1i32 as libc::c_long {
        ov_clear(vf);
        Z_Free(vf as *mut libc::c_void);
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    OGGInfo = ov_info(vf, 0i32);
    if OGGInfo.is_null() {
        ov_clear(vf);
        Z_Free(vf as *mut libc::c_void);
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    numSamples = ov_pcm_total(vf, 0i32);
    (*stream).info.rate = (*OGGInfo).rate as libc::c_int;
    (*stream).info.width = 2i32;
    (*stream).info.channels = (*OGGInfo).channels;
    (*stream).info.samples = numSamples as libc::c_int;
    (*stream).info.size =
        (*stream).info.samples * (*stream).info.channels *
            (*stream).info.width;
    (*stream).info.dataofs = 0i32;
    (*stream).pos = 0i32;
    (*stream).ptr = vf as *mut libc::c_void;
    return stream;
}
// the callback structure
#[no_mangle]
pub static mut S_OGG_Callbacks: ov_callbacks =
    ov_callbacks{read_func: Some(S_OGG_Callback_read),
                 seek_func: Some(S_OGG_Callback_seek),
                 close_func: Some(S_OGG_Callback_close),
                 tell_func: Some(S_OGG_Callback_tell),};
// ftell() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OGG_Callback_tell(mut datasource:
                                                 *mut libc::c_void)
 -> libc::c_long {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    if datasource.is_null() {
        *__errno_location() = 9i32;
        return -1i32 as libc::c_long
    }
    stream = datasource as *mut snd_stream_t;
    return FS_FTell((*stream).file) as libc::c_long;
}
// fclose() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OGG_Callback_close(mut datasource:
                                                  *mut libc::c_void)
 -> libc::c_int {
    return 0i32;
}
// fseek() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OGG_Callback_seek(mut datasource:
                                                 *mut libc::c_void,
                                             mut offset: ogg_int64_t,
                                             mut whence: libc::c_int)
 -> libc::c_int {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut retVal: libc::c_int = 0i32;
    if datasource.is_null() { *__errno_location() = 9i32; return -1i32 }
    stream = datasource as *mut snd_stream_t;
    match whence {
        0 => {
            retVal =
                FS_Seek((*stream).file, offset, FS_SEEK_SET as libc::c_int);
            if retVal < 0i32 { return retVal }
            (*stream).pos = offset as libc::c_int
        }
        1 => {
            retVal =
                FS_Seek((*stream).file, offset, FS_SEEK_CUR as libc::c_int);
            if retVal < 0i32 { return retVal }
            (*stream).pos += offset as libc::c_int
        }
        2 => {
            retVal =
                FS_Seek((*stream).file, offset, FS_SEEK_END as libc::c_int);
            if retVal < 0i32 { return retVal }
            (*stream).pos = (*stream).length + offset as libc::c_int
        }
        _ => { *__errno_location() = 22i32; return -1i32 }
    }
    (*stream).pos = if (*stream).pos < 0i32 { 0i32 } else { (*stream).pos };
    (*stream).pos =
        if (*stream).pos > (*stream).length {
            (*stream).length
        } else { (*stream).pos };
    return 0i32;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)
Copyright (C) 2005-2006 Joerg Dietrich <dietrich_joerg@gmx.de>

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
// OGG support is enabled by this define
// includes for the Q3 sound system
// includes for the OGG codec
// The OGG codec can return the samples in a number of different formats,
// we use the standard signed short format.
// Q3 OGG codec
// callbacks for vobisfile
// fread() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OGG_Callback_read(mut ptr: *mut libc::c_void,
                                             mut size: size_t,
                                             mut nmemb: size_t,
                                             mut datasource:
                                                 *mut libc::c_void)
 -> size_t {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut byteSize: libc::c_int = 0i32;
    let mut bytesRead: libc::c_int = 0i32;
    let mut nMembRead: size_t = 0i32 as size_t;
    if ptr.is_null() { *__errno_location() = 14i32; return 0i32 as size_t }
    if !(0 != size && 0 != nmemb) {
        *__errno_location() = 0i32;
        return 0i32 as size_t
    }
    if datasource.is_null() {
        *__errno_location() = 9i32;
        return 0i32 as size_t
    }
    stream = datasource as *mut snd_stream_t;
    byteSize = nmemb.wrapping_mul(size) as libc::c_int;
    bytesRead = FS_Read(ptr, byteSize, (*stream).file);
    (*stream).pos += bytesRead;
    nMembRead = (bytesRead as libc::c_ulong).wrapping_div(size);
    if 0 != (bytesRead as libc::c_ulong).wrapping_rem(size) {
        nMembRead = nMembRead.wrapping_add(1)
    }
    return nMembRead;
}
#[no_mangle]
pub unsafe extern "C" fn S_OGG_CodecLoad(mut filename: *const libc::c_char,
                                         mut info: *mut snd_info_t)
 -> *mut libc::c_void {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut bytesRead: libc::c_int = 0;
    if !(!filename.is_null() && !info.is_null()) {
        return 0 as *mut libc::c_void
    }
    stream = S_OGG_CodecOpenStream(filename);
    if stream.is_null() { return 0 as *mut libc::c_void }
    (*info).rate = (*stream).info.rate;
    (*info).width = (*stream).info.width;
    (*info).channels = (*stream).info.channels;
    (*info).samples = (*stream).info.samples;
    (*info).size = (*stream).info.size;
    (*info).dataofs = (*stream).info.dataofs;
    buffer = Hunk_AllocateTempMemory((*info).size) as *mut byte;
    if buffer.is_null() {
        S_OGG_CodecCloseStream(stream);
        return 0 as *mut libc::c_void
    }
    bytesRead =
        S_OGG_CodecReadStream(stream, (*info).size,
                              buffer as *mut libc::c_void);
    if bytesRead <= 0i32 {
        Hunk_FreeTempMemory(buffer as *mut libc::c_void);
        S_OGG_CodecCloseStream(stream);
        return 0 as *mut libc::c_void
    }
    S_OGG_CodecCloseStream(stream);
    return buffer as *mut libc::c_void;
}