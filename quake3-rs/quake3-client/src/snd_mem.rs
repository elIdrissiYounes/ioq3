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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/client/snd_local.h"]
pub mod snd_local_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct adpcm_state {
        pub sample: libc::c_short,
        pub index: libc::c_char,
    }
    pub type adpcm_state_t = adpcm_state;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sndBuffer_s {
        pub sndChunk: [libc::c_short; 1024],
        pub next: *mut sndBuffer_s,
        pub size: libc::c_int,
        pub adpcm: adpcm_state_t,
    }
    pub type sndBuffer = sndBuffer_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sfx_s {
        pub soundData: *mut sndBuffer,
        pub defaultSound: qboolean,
        pub inMemory: qboolean,
        pub soundCompressed: qboolean,
        pub soundCompressionMethod: libc::c_int,
        pub soundLength: libc::c_int,
        pub soundChannels: libc::c_int,
        pub soundName: [libc::c_char; 64],
        pub lastTimeUsed: libc::c_int,
        pub next: *mut sfx_s,
    }
    pub type sfx_t = sfx_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dma_t {
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub fullsamples: libc::c_int,
        pub submission_chunk: libc::c_int,
        pub samplebits: libc::c_int,
        pub isfloat: libc::c_int,
        pub speed: libc::c_int,
        pub buffer: *mut byte,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean, byte};
    extern "C" {
        #[no_mangle]
        pub static mut dma: dma_t;
        // wavelet function
        #[no_mangle]
        pub fn S_FreeOldestSound();
        #[no_mangle]
        pub fn S_AdpcmEncodeSound(sfx: *mut sfx_t,
                                  samples: *mut libc::c_short);
    }
}
#[header_src =
      "ioq3/code/client/snd_codec.h"]
pub mod snd_codec_h {
    pub type snd_info_t = snd_info_s;
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn S_CodecLoad(filename: *const libc::c_char,
                           info: *mut snd_info_t) -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::q_shared_h::{cvar_t};
    use super::{libc};
    extern "C" {
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // will be journaled properly
        #[no_mangle]
        pub fn Com_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/client/snd_public.h"]
pub mod snd_public_h { }
#[header_src =
      "ioq3/code/client/snd_mem.c"]
pub mod snd_mem_c { }
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, cvar_s, cvar_t,
                       Com_Printf};
use self::snd_local_h::{adpcm_state, adpcm_state_t, sndBuffer_s, sndBuffer,
                        sfx_s, sfx_t, dma_t, dma, S_FreeOldestSound,
                        S_AdpcmEncodeSound};
use self::snd_codec_h::{snd_info_t, snd_info_s, S_CodecLoad};
use self::stdlib_h::{malloc, free};
use self::qcommon_h::{Cvar_Get, Com_DPrintf, Com_Milliseconds,
                      Hunk_AllocateTempMemory, Hunk_FreeTempMemory};
#[no_mangle]
pub unsafe extern "C" fn S_DisplayFreeMemory() {
    Com_Printf(b"%d bytes free sound buffer memory, %d total used\n\x00" as
                   *const u8 as *const libc::c_char, inUse, totalInUse);
}
static mut totalInUse: libc::c_int = 0i32;
static mut inUse: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn S_LoadSound(mut sfx: *mut sfx_t) -> qboolean {
    let mut data: *mut byte = 0 as *mut byte;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut info: snd_info_t =
        snd_info_s{rate: 0,
                   width: 0,
                   channels: 0,
                   samples: 0,
                   size: 0,
                   dataofs: 0,};
    data = S_CodecLoad((*sfx).soundName.as_mut_ptr(), &mut info) as *mut byte;
    if data.is_null() { return qfalse }
    if info.width == 1i32 {
        Com_DPrintf(b"^3WARNING: %s is a 8 bit audio file\n\x00" as *const u8
                        as *const libc::c_char,
                    (*sfx).soundName.as_mut_ptr());
    }
    if info.rate != 22050i32 {
        Com_DPrintf(b"^3WARNING: %s is not a 22kHz audio file\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*sfx).soundName.as_mut_ptr());
    }
    samples =
        Hunk_AllocateTempMemory(((info.channels * info.samples) as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(2i32
                                                                                                     as
                                                                                                     libc::c_ulong)
                                    as libc::c_int) as *mut libc::c_short;
    (*sfx).lastTimeUsed = Com_Milliseconds() + 1i32;
    if info.channels == 1i32 &&
           (*sfx).soundCompressed as libc::c_uint ==
               qtrue as libc::c_int as libc::c_uint {
        (*sfx).soundCompressionMethod = 1i32;
        (*sfx).soundData = 0 as *mut sndBuffer;
        (*sfx).soundLength =
            ResampleSfxRaw(samples, info.channels, info.rate, info.width,
                           info.samples, data.offset(info.dataofs as isize));
        S_AdpcmEncodeSound(sfx, samples);
    } else {
        (*sfx).soundCompressionMethod = 0i32;
        (*sfx).soundData = 0 as *mut sndBuffer;
        (*sfx).soundLength =
            ResampleSfx(sfx, info.channels, info.rate, info.width,
                        info.samples, data.offset(info.dataofs as isize),
                        qfalse)
    }
    (*sfx).soundChannels = info.channels;
    Hunk_FreeTempMemory(samples as *mut libc::c_void);
    Hunk_FreeTempMemory(data as *mut libc::c_void);
    return qtrue;
}
/*
================
ResampleSfx

resample / decimate to the current source rate
================
*/
unsafe extern "C" fn ResampleSfx(mut sfx: *mut sfx_t,
                                 mut channels: libc::c_int,
                                 mut inrate: libc::c_int,
                                 mut inwidth: libc::c_int,
                                 mut samples: libc::c_int,
                                 mut data: *mut byte,
                                 mut compressed: qboolean) -> libc::c_int {
    let mut outcount: libc::c_int = 0;
    let mut srcsample: libc::c_int = 0;
    let mut stepscale: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sample: libc::c_int = 0;
    let mut samplefrac: libc::c_int = 0;
    let mut fracstep: libc::c_int = 0;
    let mut part: libc::c_int = 0;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    stepscale = inrate as libc::c_float / dma.speed as libc::c_float;
    outcount = (samples as libc::c_float / stepscale) as libc::c_int;
    srcsample = 0i32;
    samplefrac = 0i32;
    fracstep =
        (stepscale * 256i32 as libc::c_float * channels as libc::c_float) as
            libc::c_int;
    chunk = (*sfx).soundData;
    i = 0i32;
    while i < outcount {
        srcsample += samplefrac >> 8i32;
        samplefrac &= 255i32;
        samplefrac += fracstep;
        j = 0i32;
        while j < channels {
            if inwidth == 2i32 {
                sample =
                    *(data as
                          *mut libc::c_short).offset((srcsample + j) as isize)
                        as libc::c_int
            } else {
                sample =
                    (((*data.offset((srcsample + j) as isize) as libc::c_int -
                           128i32) as libc::c_uint) << 8i32) as libc::c_int
            }
            part = i * channels + j & 1024i32 - 1i32;
            if part == 0i32 {
                let mut newchunk: *mut sndBuffer = 0 as *mut sndBuffer;
                newchunk = SND_malloc();
                if chunk.is_null() {
                    (*sfx).soundData = newchunk
                } else { (*chunk).next = newchunk }
                chunk = newchunk
            }
            (*chunk).sndChunk[part as usize] = sample as libc::c_short;
            j += 1
        }
        i += 1
    }
    return outcount;
}
#[no_mangle]
pub unsafe extern "C" fn SND_malloc() -> *mut sndBuffer {
    let mut v: *mut sndBuffer = 0 as *mut sndBuffer;
    while freelist.is_null() { S_FreeOldestSound(); }
    inUse =
        (inUse as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<sndBuffer>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    totalInUse =
        (totalInUse as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<sndBuffer>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    v = freelist;
    freelist = *(freelist as *mut *mut sndBuffer);
    (*v).next = 0 as *mut sndBuffer_s;
    return v;
}
static mut freelist: *mut sndBuffer = 0 as *const sndBuffer as *mut sndBuffer;
/*
================
ResampleSfx

resample / decimate to the current source rate
================
*/
unsafe extern "C" fn ResampleSfxRaw(mut sfx: *mut libc::c_short,
                                    mut channels: libc::c_int,
                                    mut inrate: libc::c_int,
                                    mut inwidth: libc::c_int,
                                    mut samples: libc::c_int,
                                    mut data: *mut byte) -> libc::c_int {
    let mut outcount: libc::c_int = 0;
    let mut srcsample: libc::c_int = 0;
    let mut stepscale: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sample: libc::c_int = 0;
    let mut samplefrac: libc::c_int = 0;
    let mut fracstep: libc::c_int = 0;
    stepscale = inrate as libc::c_float / dma.speed as libc::c_float;
    outcount = (samples as libc::c_float / stepscale) as libc::c_int;
    srcsample = 0i32;
    samplefrac = 0i32;
    fracstep =
        (stepscale * 256i32 as libc::c_float * channels as libc::c_float) as
            libc::c_int;
    i = 0i32;
    while i < outcount {
        srcsample += samplefrac >> 8i32;
        samplefrac &= 255i32;
        samplefrac += fracstep;
        j = 0i32;
        while j < channels {
            if inwidth == 2i32 {
                sample =
                    *(data as
                          *mut libc::c_short).offset((srcsample + j) as isize)
                        as libc::c_int
            } else {
                sample =
                    *data.offset((srcsample + j) as isize) as libc::c_int -
                        128i32 << 8i32
            }
            *sfx.offset((i * channels + j) as isize) =
                sample as libc::c_short;
            j += 1
        }
        i += 1
    }
    return outcount;
}
#[no_mangle]
pub unsafe extern "C" fn SND_free(mut v: *mut sndBuffer) {
    let ref mut fresh0 = *(v as *mut *mut sndBuffer);
    *fresh0 = freelist;
    freelist = v;
    inUse =
        (inUse as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<sndBuffer>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SND_setup() {
    let mut p: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut q: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    let mut scs: libc::c_int = 0;
    cv =
        Cvar_Get(b"com_soundMegs\x00" as *const u8 as *const libc::c_char,
                 b"8\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    scs = (*cv).integer * 1536i32;
    buffer =
        malloc((scs as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<sndBuffer>()
                                                    as libc::c_ulong)) as
            *mut sndBuffer;
    sfxScratchBuffer =
        malloc((1024i32 as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(4i32
                                                                                    as
                                                                                    libc::c_ulong))
            as *mut libc::c_short;
    sfxScratchPointer = 0 as *mut sfx_t;
    inUse =
        (scs as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<sndBuffer>() as
                                             libc::c_ulong) as libc::c_int;
    p = buffer;
    q = p.offset(scs as isize);
    loop  {
        q = q.offset(-1isize);
        if !(q > p) { break ; }
        let ref mut fresh1 = *(q as *mut *mut sndBuffer);
        *fresh1 = q.offset(-1isize)
    }
    let ref mut fresh2 = *(q as *mut *mut sndBuffer);
    *fresh2 = 0 as *mut sndBuffer;
    freelist = p.offset(scs as isize).offset(-1isize);
    Com_Printf(b"Sound memory manager started\n\x00" as *const u8 as
                   *const libc::c_char);
}
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
/* ****************************************************************************
 * name:		snd_mem.c
 *
 * desc:		sound caching
 *
 * $Archive: /MissionPack/code/client/snd_mem.c $
 *
 *****************************************************************************/
/*
===============================================================================

memory management

===============================================================================
*/
static mut buffer: *mut sndBuffer = 0 as *const sndBuffer as *mut sndBuffer;
#[no_mangle]
pub static mut sfxScratchPointer: *mut sfx_t =
    0 as *const sfx_t as *mut sfx_t;
#[no_mangle]
pub static mut sfxScratchBuffer: *mut libc::c_short =
    0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub unsafe extern "C" fn SND_shutdown() {
    free(sfxScratchBuffer as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
#[no_mangle]
pub static mut sfxScratchIndex: libc::c_int = 0i32;