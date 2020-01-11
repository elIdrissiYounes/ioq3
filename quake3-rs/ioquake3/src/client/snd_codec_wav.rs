use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::src::client::snd_codec::snd_codec_s;
pub use crate::src::client::snd_codec::snd_codec_t;
pub use crate::src::client::snd_codec::snd_info_s;
pub use crate::src::client::snd_codec::snd_info_t;
pub use crate::src::client::snd_codec::snd_stream_s;
pub use crate::src::client::snd_codec::snd_stream_t;
pub use crate::src::client::snd_codec::S_CodecUtilClose;
pub use crate::src::client::snd_codec::S_CodecUtilOpen;
pub use crate::src::client::snd_codec::CODEC_CLOSE;
pub use crate::src::client::snd_codec::CODEC_LOAD;
pub use crate::src::client::snd_codec::CODEC_OPEN;
pub use crate::src::client::snd_codec::CODEC_READ;
pub use crate::src::qcommon::common::Com_Printf;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::Q_strncmp;
pub use crate::src::qcommon::q_shared::FS_SEEK_CUR;
pub use crate::src::qcommon::q_shared::FS_SEEK_END;
pub use crate::src::qcommon::q_shared::FS_SEEK_SET;
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

unsafe extern "C" fn FGetLittleLong(mut f: crate::src::qcommon::q_shared::fileHandle_t) -> i32 {
    let mut v: i32 = 0;
    crate::src::qcommon::files::FS_Read(
        &mut v as *mut i32 as *mut libc::c_void,
        ::std::mem::size_of::<i32>() as i32,
        f,
    );
    return v;
}
/*
=================
FGetLittleShort
=================
*/

unsafe extern "C" fn FGetLittleShort(mut f: crate::src::qcommon::q_shared::fileHandle_t) -> i16 {
    let mut v: i16 = 0;
    crate::src::qcommon::files::FS_Read(
        &mut v as *mut i16 as *mut libc::c_void,
        ::std::mem::size_of::<i16>() as i32,
        f,
    );
    return v;
}
/*
=================
S_ReadChunkInfo
=================
*/

unsafe extern "C" fn S_ReadChunkInfo(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
    mut name: *mut i8,
) -> i32 {
    let mut len: i32 = 0;
    let mut r: i32 = 0;
    *name.offset(4) = 0i8;
    r = crate::src::qcommon::files::FS_Read(name as *mut libc::c_void, 4, f);
    if r != 4 {
        return -(1i32);
    }
    len = FGetLittleLong(f);
    if len < 0 {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: Negative chunk length\n\x00" as *const u8 as *const i8,
        );
        return -(1i32);
    }
    return len;
}
/*
=================
S_FindRIFFChunk

Returns the length of the data in the chunk, or -1 if not found
=================
*/

unsafe extern "C" fn S_FindRIFFChunk(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
    mut chunk: *mut i8,
) -> i32 {
    let mut name: [i8; 5] = [0; 5];
    let mut len: i32 = 0;
    loop {
        len = S_ReadChunkInfo(f, name.as_mut_ptr());
        if !(len >= 0) {
            break;
        }
        // If this is the right chunk, return
        if crate::src::qcommon::q_shared::Q_strncmp(name.as_mut_ptr(), chunk, 4) == 0 {
            return len;
        }
        len = len + 2 - 1 & !(2 - 1);
        // Not the right chunk - skip it
        crate::src::qcommon::files::FS_Seek(
            f,
            len as isize,
            crate::src::qcommon::q_shared::FS_SEEK_CUR as i32,
        );
    }
    return -(1);
}
/*
=================
S_ByteSwapRawSamples
=================
*/

unsafe extern "C" fn S_ByteSwapRawSamples(
    mut samples: i32,
    mut width: i32,
    mut s_channels: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
) {
    let mut i: i32 = 0;
    if width != 2 {
        return;
    }
    if 256 == 256 {
        return;
    }
    if s_channels == 2 {
        samples <<= 1
    }
    i = 0;
    while i < samples {
        *(data as *mut i16).offset(i as isize) = *(data as *mut i16).offset(i as isize);
        i += 1
    }
}
/*
=================
S_ReadRIFFHeader
=================
*/

unsafe extern "C" fn S_ReadRIFFHeader(
    mut file: crate::src::qcommon::q_shared::fileHandle_t,
    mut info: *mut crate::src::client::snd_codec::snd_info_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut dump: [i8; 16] = [0; 16];
    let mut bits: i32 = 0;
    let mut fmtlen: i32 = 0;
    // skip the riff wav header
    crate::src::qcommon::files::FS_Read(dump.as_mut_ptr() as *mut libc::c_void, 12, file);
    // Scan for the format chunk
    fmtlen = S_FindRIFFChunk(file, b"fmt \x00" as *const u8 as *mut i8);
    if fmtlen < 0 {
        crate::src::qcommon::common::Com_Printf(
            b"^1ERROR: Couldn\'t find \"fmt\" chunk\n\x00" as *const u8 as *const i8,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    // Save the parameters
    FGetLittleShort(file); // wav_format
    (*info).channels = FGetLittleShort(file) as i32;
    (*info).rate = FGetLittleLong(file);
    FGetLittleLong(file);
    FGetLittleShort(file);
    bits = FGetLittleShort(file) as i32;
    if bits < 8 {
        crate::src::qcommon::common::Com_Printf(
            b"^1ERROR: Less than 8 bit sound is not supported\n\x00" as *const u8 as *const i8,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*info).width = bits / 8;
    (*info).dataofs = 0;
    // Skip the rest of the format chunk if required
    if fmtlen > 16 {
        fmtlen -= 16;
        crate::src::qcommon::files::FS_Seek(
            file,
            fmtlen as isize,
            crate::src::qcommon::q_shared::FS_SEEK_CUR as i32,
        );
    }
    // Scan for the data chunk
    (*info).size = S_FindRIFFChunk(file, b"data\x00" as *const u8 as *mut i8);
    if (*info).size < 0 {
        crate::src::qcommon::common::Com_Printf(
            b"^1ERROR: Couldn\'t find \"data\" chunk\n\x00" as *const u8 as *const i8,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*info).samples = (*info).size / (*info).width / (*info).channels;
    return crate::src::qcommon::q_shared::qtrue;
}
// WAV codec
#[no_mangle]

pub static mut wav_codec: crate::src::client::snd_codec::snd_codec_t = unsafe {
    {
        let mut init = crate::src::client::snd_codec::snd_codec_s {
            ext: b"wav\x00" as *const u8 as *mut i8,
            load: Some(
                S_WAV_CodecLoad
                    as unsafe extern "C" fn(
                        _: *const i8,
                        _: *mut crate::src::client::snd_codec::snd_info_t,
                    ) -> *mut libc::c_void,
            ),
            open: Some(
                S_WAV_CodecOpenStream
                    as unsafe extern "C" fn(
                        _: *const i8,
                    )
                        -> *mut crate::src::client::snd_codec::snd_stream_t,
            ),
            read: Some(
                S_WAV_CodecReadStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                        _: i32,
                        _: *mut libc::c_void,
                    ) -> i32,
            ),
            close: Some(
                S_WAV_CodecCloseStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                    ) -> (),
            ),
            next: 0 as *mut crate::src::client::snd_codec::snd_codec_t,
        };
        init
    }
};
// WAV Codec
/*
=================
S_WAV_CodecLoad
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_WAV_CodecLoad(
    mut filename: *const i8,
    mut info: *mut crate::src::client::snd_codec::snd_info_t,
) -> *mut libc::c_void {
    let mut file: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    // Try to open the file
    crate::src::qcommon::files::FS_FOpenFileRead(
        filename,
        &mut file,
        crate::src::qcommon::q_shared::qtrue,
    );
    if file == 0 {
        return 0 as *mut libc::c_void;
    }
    // Read the RIFF header
    if S_ReadRIFFHeader(file, info) as u64 == 0 {
        crate::src::qcommon::files::FS_FCloseFile(file);
        crate::src::qcommon::common::Com_Printf(
            b"^1ERROR: Incorrect/unsupported format in \"%s\"\n\x00" as *const u8 as *const i8,
            filename,
        );
        return 0 as *mut libc::c_void;
    }
    // Allocate some memory
    buffer = crate::src::qcommon::common::Hunk_AllocateTempMemory((*info).size);
    if buffer.is_null() {
        crate::src::qcommon::files::FS_FCloseFile(file);
        crate::src::qcommon::common::Com_Printf(
            b"^1ERROR: Out of memory reading \"%s\"\n\x00" as *const u8 as *const i8,
            filename,
        );
        return 0 as *mut libc::c_void;
    }
    // Read, byteswap
    crate::src::qcommon::files::FS_Read(buffer, (*info).size, file);
    S_ByteSwapRawSamples(
        (*info).samples,
        (*info).width,
        (*info).channels,
        buffer as *mut crate::src::qcommon::q_shared::byte,
    );
    // Close and return
    crate::src::qcommon::files::FS_FCloseFile(file);
    return buffer;
}
/*
=================
S_WAV_CodecOpenStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_WAV_CodecOpenStream(
    mut filename: *const i8,
) -> *mut crate::src::client::snd_codec::snd_stream_t {
    let mut rv: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // Open
    rv = crate::src::client::snd_codec::S_CodecUtilOpen(filename, &mut wav_codec);
    if rv.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // Read the RIFF header
    if S_ReadRIFFHeader((*rv).file, &mut (*rv).info) as u64 == 0 {
        crate::src::client::snd_codec::S_CodecUtilClose(&mut rv);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    return rv;
}
/*
=================
S_WAV_CodecCloseStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_WAV_CodecCloseStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
) {
    crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
}
/*
=================
S_WAV_CodecReadStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_WAV_CodecReadStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
    mut bytes: i32,
    mut buffer: *mut libc::c_void,
) -> i32 {
    let mut remaining: i32 = (*stream).info.size - (*stream).pos;
    let mut samples: i32 = 0;
    if remaining <= 0 {
        return 0i32;
    }
    if bytes > remaining {
        bytes = remaining
    }
    (*stream).pos += bytes;
    samples = bytes / (*stream).info.width / (*stream).info.channels;
    crate::src::qcommon::files::FS_Read(buffer, bytes, (*stream).file);
    S_ByteSwapRawSamples(
        samples,
        (*stream).info.width,
        (*stream).info.channels,
        buffer as *const crate::src::qcommon::q_shared::byte,
    );
    return bytes;
}
