use libc;
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
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
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
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
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_local.h"]
pub mod snd_local_h {
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
// snd_local.h -- private sound definations
    // this is in samples
    // samples
    // floats
    // floats
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct portable_samplepair_t {
        pub left: libc::c_int,
        pub right: libc::c_int,
    }
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct channel_t {
        pub allocTime: libc::c_int,
        pub startSample: libc::c_int,
        pub entnum: libc::c_int,
        pub entchannel: libc::c_int,
        pub leftvol: libc::c_int,
        pub rightvol: libc::c_int,
        pub master_vol: libc::c_int,
        pub dopplerScale: libc::c_float,
        pub oldDopplerScale: libc::c_float,
        pub origin: vec3_t,
        pub fixed_origin: qboolean,
        pub thesfx: *mut sfx_t,
        pub doppler: qboolean,
        pub fullVolume: qboolean,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean, byte, vec3_t, cvar_t};
    extern "C" {
        //====================================================================
        #[no_mangle]
        pub static mut s_channels: [channel_t; 96];
        #[no_mangle]
        pub static mut loop_channels: [channel_t; 96];
        #[no_mangle]
        pub static mut numLoopChannels: libc::c_int;
        #[no_mangle]
        pub static mut s_paintedtime: libc::c_int;
        #[no_mangle]
        pub static mut dma: dma_t;
        #[no_mangle]
        pub static mut s_rawsamples: [[portable_samplepair_t; 16384]; 129];
        #[no_mangle]
        pub static mut s_rawend: [libc::c_int; 129];
        #[no_mangle]
        pub static mut s_volume: *mut cvar_t;
        #[no_mangle]
        pub static mut s_muted: *mut cvar_t;
        #[no_mangle]
        pub static mut s_testsound: *mut cvar_t;
        #[no_mangle]
        pub static mut mulawToShort: [libc::c_short; 256];
        #[no_mangle]
        pub static mut sfxScratchIndex: libc::c_int;
        #[no_mangle]
        pub static mut sfxScratchBuffer: *mut libc::c_short;
        #[no_mangle]
        pub fn decodeWavelet(stream: *mut sndBuffer,
                             packets: *mut libc::c_short);
        #[no_mangle]
        pub static mut sfxScratchPointer: *mut sfx_t;
        #[no_mangle]
        pub fn S_AdpcmGetSamples(chunk: *mut sndBuffer,
                                 to: *mut libc::c_short);
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/client.h"]
pub mod client_h {
    use super::q_shared_h::{byte, qboolean};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CL_WriteAVIAudioFrame(pcmBuffer: *const byte,
                                     size: libc::c_int);
        #[no_mangle]
        pub fn CL_VideoRecording() -> qboolean;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_mix.c"]
pub mod snd_mix_c {
    use super::{libc};
    use super::snd_local_h::{channel_t, sfx_t};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, vec_t, vec3_t, cvar_s,
                       cvar_t};
use self::snd_local_h::{portable_samplepair_t, adpcm_state, adpcm_state_t,
                        sndBuffer_s, sndBuffer, sfx_s, sfx_t, dma_t,
                        channel_t, s_channels, loop_channels, numLoopChannels,
                        s_paintedtime, dma, s_rawsamples, s_rawend, s_volume,
                        s_muted, s_testsound, mulawToShort, sfxScratchIndex,
                        sfxScratchBuffer, decodeWavelet, sfxScratchPointer,
                        S_AdpcmGetSamples};
use self::mathcalls_h::{sin};
use self::string_h::{memset};
use self::client_h::{CL_WriteAVIAudioFrame, CL_VideoRecording};
#[no_mangle]
pub unsafe extern "C" fn S_PaintChannels(mut endtime: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut stream: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut sc: *mut sfx_t = 0 as *mut sfx_t;
    let mut ltime: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut sampleOffset: libc::c_int = 0;
    if 0 != (*s_muted).integer {
        snd_vol = 0i32
    } else {
        snd_vol = ((*s_volume).value * 255i32 as libc::c_float) as libc::c_int
    }
    while s_paintedtime < endtime {
        end = endtime;
        if endtime - s_paintedtime > 4096i32 { end = s_paintedtime + 4096i32 }
        memset(paintbuffer.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[portable_samplepair_t; 4096]>() as
                   libc::c_ulong);
        stream = 0i32;
        while stream < 64i32 * 2i32 + 1i32 {
            if s_rawend[stream as usize] >= s_paintedtime {
                let mut rawsamples: *const portable_samplepair_t =
                    s_rawsamples[stream as usize].as_mut_ptr();
                let stop: libc::c_int =
                    if end < s_rawend[stream as usize] {
                        end
                    } else { s_rawend[stream as usize] };
                i = s_paintedtime;
                while i < stop {
                    let s: libc::c_int = i & 16384i32 - 1i32;
                    paintbuffer[(i - s_paintedtime) as usize].left +=
                        (*rawsamples.offset(s as isize)).left;
                    paintbuffer[(i - s_paintedtime) as usize].right +=
                        (*rawsamples.offset(s as isize)).right;
                    i += 1
                }
            }
            stream += 1
        }
        ch = s_channels.as_mut_ptr();
        i = 0i32;
        while i < 96i32 {
            if !((*ch).thesfx.is_null() ||
                     ((*ch).leftvol as libc::c_double) < 0.25f64 &&
                         ((*ch).rightvol as libc::c_double) < 0.25f64) {
                ltime = s_paintedtime;
                sc = (*ch).thesfx;
                if !((*sc).soundData.is_null() || (*sc).soundLength == 0i32) {
                    sampleOffset = ltime - (*ch).startSample;
                    count = end - ltime;
                    if sampleOffset + count > (*sc).soundLength {
                        count = (*sc).soundLength - sampleOffset
                    }
                    if count > 0i32 {
                        if (*sc).soundCompressionMethod == 1i32 {
                            S_PaintChannelFromADPCM(ch, sc, count,
                                                    sampleOffset,
                                                    ltime - s_paintedtime);
                        } else if (*sc).soundCompressionMethod == 2i32 {
                            S_PaintChannelFromWavelet(ch, sc, count,
                                                      sampleOffset,
                                                      ltime - s_paintedtime);
                        } else if (*sc).soundCompressionMethod == 3i32 {
                            S_PaintChannelFromMuLaw(ch, sc, count,
                                                    sampleOffset,
                                                    ltime - s_paintedtime);
                        } else {
                            S_PaintChannelFrom16(ch, sc, count, sampleOffset,
                                                 ltime - s_paintedtime);
                        }
                    }
                }
            }
            i += 1;
            ch = ch.offset(1isize)
        }
        ch = loop_channels.as_mut_ptr();
        i = 0i32;
        while i < numLoopChannels {
            if !((*ch).thesfx.is_null() ||
                     0 == (*ch).leftvol && 0 == (*ch).rightvol) {
                ltime = s_paintedtime;
                sc = (*ch).thesfx;
                if !((*sc).soundData.is_null() || (*sc).soundLength == 0i32) {
                    loop  {
                        sampleOffset = ltime % (*sc).soundLength;
                        count = end - ltime;
                        if sampleOffset + count > (*sc).soundLength {
                            count = (*sc).soundLength - sampleOffset
                        }
                        if count > 0i32 {
                            if (*sc).soundCompressionMethod == 1i32 {
                                S_PaintChannelFromADPCM(ch, sc, count,
                                                        sampleOffset,
                                                        ltime -
                                                            s_paintedtime);
                            } else if (*sc).soundCompressionMethod == 2i32 {
                                S_PaintChannelFromWavelet(ch, sc, count,
                                                          sampleOffset,
                                                          ltime -
                                                              s_paintedtime);
                            } else if (*sc).soundCompressionMethod == 3i32 {
                                S_PaintChannelFromMuLaw(ch, sc, count,
                                                        sampleOffset,
                                                        ltime -
                                                            s_paintedtime);
                            } else {
                                S_PaintChannelFrom16(ch, sc, count,
                                                     sampleOffset,
                                                     ltime - s_paintedtime);
                            }
                            ltime += count
                        }
                        if !(ltime < end) { break ; }
                    }
                }
            }
            i += 1;
            ch = ch.offset(1isize)
        }
        S_TransferPaintBuffer(end);
        s_paintedtime = end
    };
}
/*
===================
S_TransferPaintBuffer

===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_TransferPaintBuffer(mut endtime: libc::c_int) {
    let mut out_idx: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut step: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pbuf: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    pbuf = dma.buffer as *mut libc::c_ulong;
    if 0 != (*s_testsound).integer {
        count = endtime - s_paintedtime;
        i = 0i32;
        while i < count {
            paintbuffer[i as usize].right =
                (sin((s_paintedtime + i) as libc::c_double * 0.1f64) *
                     20000i32 as libc::c_double * 256i32 as libc::c_double) as
                    libc::c_int;
            paintbuffer[i as usize].left = paintbuffer[i as usize].right;
            i += 1
        }
    }
    if dma.samplebits == 16i32 && dma.channels == 2i32 {
        S_TransferStereo16(pbuf, endtime);
    } else {
        p = paintbuffer.as_mut_ptr() as *mut libc::c_int;
        count = (endtime - s_paintedtime) * dma.channels;
        out_idx = s_paintedtime * dma.channels % dma.samples;
        step = 3i32 - if dma.channels < 2i32 { dma.channels } else { 2i32 };
        if 0 != dma.isfloat && dma.samplebits == 32i32 {
            let mut out: *mut libc::c_float = pbuf as *mut libc::c_float;
            i = 0i32;
            while i < count {
                if i % dma.channels >= 2i32 {
                    val = 0i32
                } else { val = *p >> 8i32; p = p.offset(step as isize) }
                if val > 0x7fffi32 {
                    val = 0x7fffi32
                } else if val < -32767i32 { val = -32767i32 }
                *out.offset(out_idx as isize) =
                    val as libc::c_float / 32767.0f32;
                out_idx = (out_idx + 1i32) % dma.samples;
                i += 1
            }
        } else if dma.samplebits == 16i32 {
            let mut out_0: *mut libc::c_short = pbuf as *mut libc::c_short;
            i = 0i32;
            while i < count {
                if i % dma.channels >= 2i32 {
                    val = 0i32
                } else { val = *p >> 8i32; p = p.offset(step as isize) }
                if val > 0x7fffi32 {
                    val = 0x7fffi32
                } else if val < -32768i32 { val = -32768i32 }
                *out_0.offset(out_idx as isize) = val as libc::c_short;
                out_idx = (out_idx + 1i32) % dma.samples;
                i += 1
            }
        } else if dma.samplebits == 8i32 {
            let mut out_1: *mut libc::c_uchar = pbuf as *mut libc::c_uchar;
            i = 0i32;
            while i < count {
                if i % dma.channels >= 2i32 {
                    val = 0i32
                } else { val = *p >> 8i32; p = p.offset(step as isize) }
                if val > 0x7fffi32 {
                    val = 0x7fffi32
                } else if val < -32768i32 { val = -32768i32 }
                *out_1.offset(out_idx as isize) =
                    ((val >> 8i32) + 128i32) as libc::c_uchar;
                out_idx = (out_idx + 1i32) % dma.samples;
                i += 1
            }
        }
    };
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
// snd_mix.c -- portable code to mix sounds for snd_dma.c
static mut paintbuffer: [portable_samplepair_t; 4096] =
    [portable_samplepair_t{left: 0, right: 0,}; 4096];
#[no_mangle]
pub unsafe extern "C" fn S_TransferStereo16(mut pbuf: *mut libc::c_ulong,
                                            mut endtime: libc::c_int) {
    let mut lpos: libc::c_int = 0;
    let mut ls_paintedtime: libc::c_int = 0;
    snd_p = paintbuffer.as_mut_ptr() as *mut libc::c_int;
    ls_paintedtime = s_paintedtime;
    while ls_paintedtime < endtime {
        lpos = ls_paintedtime % dma.fullsamples;
        snd_out =
            (pbuf as *mut libc::c_short).offset((lpos << 1i32) as isize);
        snd_linear_count = dma.fullsamples - lpos;
        if ls_paintedtime + snd_linear_count > endtime {
            snd_linear_count = endtime - ls_paintedtime
        }
        snd_linear_count <<= 1i32;
        S_WriteLinearBlastStereo16();
        snd_p = snd_p.offset(snd_linear_count as isize);
        ls_paintedtime += snd_linear_count >> 1i32;
        if 0 != CL_VideoRecording() as u64 {
            CL_WriteAVIAudioFrame(snd_out as *mut byte,
                                  snd_linear_count << 1i32);
        }
    };
}
#[no_mangle]
pub static mut snd_linear_count: libc::c_int = 0;
#[no_mangle]
pub static mut snd_out: *mut libc::c_short =
    0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut snd_p: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
// if configured not to use asm
#[no_mangle]
pub unsafe extern "C" fn S_WriteLinearBlastStereo16() {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    i = 0i32;
    while i < snd_linear_count {
        val = *snd_p.offset(i as isize) >> 8i32;
        if val > 0x7fffi32 {
            *snd_out.offset(i as isize) = 0x7fffi32 as libc::c_short
        } else if val < -32768i32 {
            *snd_out.offset(i as isize) = -32768i32 as libc::c_short
        } else { *snd_out.offset(i as isize) = val as libc::c_short }
        val = *snd_p.offset((i + 1i32) as isize) >> 8i32;
        if val > 0x7fffi32 {
            *snd_out.offset((i + 1i32) as isize) = 0x7fffi32 as libc::c_short
        } else if val < -32768i32 {
            *snd_out.offset((i + 1i32) as isize) = -32768i32 as libc::c_short
        } else { *snd_out.offset((i + 1i32) as isize) = val as libc::c_short }
        i += 2i32
    };
}
unsafe extern "C" fn S_PaintChannelFrom16(mut ch: *mut channel_t,
                                          mut sc: *const sfx_t,
                                          mut count: libc::c_int,
                                          mut sampleOffset: libc::c_int,
                                          mut bufferOffset: libc::c_int) {
    S_PaintChannelFrom16_scalar(ch, sc, count, sampleOffset, bufferOffset);
}
/*
===============================================================================

CHANNEL MIXING

===============================================================================
*/
unsafe extern "C" fn S_PaintChannelFrom16_scalar(mut ch: *mut channel_t,
                                                 mut sc: *const sfx_t,
                                                 mut count: libc::c_int,
                                                 mut sampleOffset:
                                                     libc::c_int,
                                                 mut bufferOffset:
                                                     libc::c_int) {
    let mut data: libc::c_int = 0;
    let mut aoff: libc::c_int = 0;
    let mut boff: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut samp: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut ooff: libc::c_float = 0.;
    let mut fdata: [libc::c_float; 2] = [0.; 2];
    let mut fdiv: libc::c_float = 0.;
    let mut fleftvol: libc::c_float = 0.;
    let mut frightvol: libc::c_float = 0.;
    if (*sc).soundChannels <= 0i32 { return }
    samp =
        &mut paintbuffer[bufferOffset as usize] as *mut portable_samplepair_t;
    if 0 != (*ch).doppler as u64 {
        sampleOffset =
            (sampleOffset as libc::c_float * (*ch).oldDopplerScale) as
                libc::c_int
    }
    if (*sc).soundChannels == 2i32 {
        sampleOffset *= (*sc).soundChannels;
        if 0 != sampleOffset & 1i32 { sampleOffset &= !1i32 }
    }
    chunk = (*sc).soundData;
    while sampleOffset >= 1024i32 {
        chunk = (*chunk).next;
        sampleOffset -= 1024i32;
        if chunk.is_null() { chunk = (*sc).soundData }
    }
    if 0 == (*ch).doppler as u64 || (*ch).dopplerScale == 1.0f32 {
        leftvol = (*ch).leftvol * snd_vol;
        rightvol = (*ch).rightvol * snd_vol;
        samples = (*chunk).sndChunk.as_mut_ptr();
        i = 0i32;
        while i < count {
            let fresh0 = sampleOffset;
            sampleOffset = sampleOffset + 1;
            data = *samples.offset(fresh0 as isize) as libc::c_int;
            (*samp.offset(i as isize)).left += data * leftvol >> 8i32;
            if (*sc).soundChannels == 2i32 {
                let fresh1 = sampleOffset;
                sampleOffset = sampleOffset + 1;
                data = *samples.offset(fresh1 as isize) as libc::c_int
            }
            (*samp.offset(i as isize)).right += data * rightvol >> 8i32;
            if sampleOffset == 1024i32 {
                chunk = (*chunk).next;
                samples = (*chunk).sndChunk.as_mut_ptr();
                sampleOffset = 0i32
            }
            i += 1
        }
    } else {
        fleftvol = ((*ch).leftvol * snd_vol) as libc::c_float;
        frightvol = ((*ch).rightvol * snd_vol) as libc::c_float;
        ooff = sampleOffset as libc::c_float;
        samples = (*chunk).sndChunk.as_mut_ptr();
        i = 0i32;
        while i < count {
            aoff = ooff as libc::c_int;
            ooff =
                ooff +
                    (*ch).dopplerScale * (*sc).soundChannels as libc::c_float;
            boff = ooff as libc::c_int;
            fdata[1usize] = 0i32 as libc::c_float;
            fdata[0usize] = fdata[1usize];
            j = aoff;
            while j < boff {
                if j == 1024i32 {
                    chunk = (*chunk).next;
                    if chunk.is_null() { chunk = (*sc).soundData }
                    samples = (*chunk).sndChunk.as_mut_ptr();
                    ooff -= 1024i32 as libc::c_float
                }
                if (*sc).soundChannels == 2i32 {
                    fdata[0usize] +=
                        *samples.offset((j & 1024i32 - 1i32) as isize) as
                            libc::c_int as libc::c_float;
                    fdata[1usize] +=
                        *samples.offset((j + 1i32 & 1024i32 - 1i32) as isize)
                            as libc::c_int as libc::c_float
                } else {
                    fdata[0usize] +=
                        *samples.offset((j & 1024i32 - 1i32) as isize) as
                            libc::c_int as libc::c_float;
                    fdata[1usize] +=
                        *samples.offset((j & 1024i32 - 1i32) as isize) as
                            libc::c_int as libc::c_float
                }
                j += (*sc).soundChannels
            }
            fdiv =
                (256i32 * (boff - aoff) / (*sc).soundChannels) as
                    libc::c_float;
            let ref mut fresh2 = (*samp.offset(i as isize)).left;
            *fresh2 =
                (*fresh2 as libc::c_float + fdata[0usize] * fleftvol / fdiv)
                    as libc::c_int;
            let ref mut fresh3 = (*samp.offset(i as isize)).right;
            *fresh3 =
                (*fresh3 as libc::c_float + fdata[1usize] * frightvol / fdiv)
                    as libc::c_int;
            i += 1
        }
    };
}
static mut snd_vol: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn S_PaintChannelFromMuLaw(mut ch: *mut channel_t,
                                                 mut sc: *mut sfx_t,
                                                 mut count: libc::c_int,
                                                 mut sampleOffset:
                                                     libc::c_int,
                                                 mut bufferOffset:
                                                     libc::c_int) {
    let mut data: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut samp: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut samples: *mut byte = 0 as *mut byte;
    let mut ooff: libc::c_float = 0.;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    samp =
        &mut paintbuffer[bufferOffset as usize] as *mut portable_samplepair_t;
    chunk = (*sc).soundData;
    while sampleOffset >= 1024i32 * 2i32 {
        chunk = (*chunk).next;
        sampleOffset -= 1024i32 * 2i32;
        if chunk.is_null() { chunk = (*sc).soundData }
    }
    if 0 == (*ch).doppler as u64 {
        samples =
            ((*chunk).sndChunk.as_mut_ptr() as
                 *mut byte).offset(sampleOffset as isize);
        i = 0i32;
        while i < count {
            data = mulawToShort[*samples as usize] as libc::c_int;
            (*samp.offset(i as isize)).left += data * leftvol >> 8i32;
            (*samp.offset(i as isize)).right += data * rightvol >> 8i32;
            samples = samples.offset(1isize);
            if !chunk.is_null() &&
                   samples ==
                       ((*chunk).sndChunk.as_mut_ptr() as
                            *mut byte).offset((1024i32 * 2i32) as isize) {
                chunk = (*chunk).next;
                samples = (*chunk).sndChunk.as_mut_ptr() as *mut byte
            }
            i += 1
        }
    } else {
        ooff = sampleOffset as libc::c_float;
        samples = (*chunk).sndChunk.as_mut_ptr() as *mut byte;
        i = 0i32;
        while i < count {
            data =
                mulawToShort[*samples.offset(ooff as libc::c_int as isize) as
                                 usize] as libc::c_int;
            ooff = ooff + (*ch).dopplerScale;
            (*samp.offset(i as isize)).left += data * leftvol >> 8i32;
            (*samp.offset(i as isize)).right += data * rightvol >> 8i32;
            if ooff >= (1024i32 * 2i32) as libc::c_float {
                chunk = (*chunk).next;
                if chunk.is_null() { chunk = (*sc).soundData }
                samples = (*chunk).sndChunk.as_mut_ptr() as *mut byte;
                ooff = 0.0f64 as libc::c_float
            }
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_PaintChannelFromWavelet(mut ch: *mut channel_t,
                                                   mut sc: *mut sfx_t,
                                                   mut count: libc::c_int,
                                                   mut sampleOffset:
                                                       libc::c_int,
                                                   mut bufferOffset:
                                                       libc::c_int) {
    let mut data: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut samp: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    i = 0i32;
    samp =
        &mut paintbuffer[bufferOffset as usize] as *mut portable_samplepair_t;
    chunk = (*sc).soundData;
    while sampleOffset >= 1024i32 / 2i32 * 4i32 {
        chunk = (*chunk).next;
        sampleOffset -= 1024i32 / 2i32 * 4i32;
        i += 1
    }
    if i != sfxScratchIndex || sfxScratchPointer != sc {
        S_AdpcmGetSamples(chunk, sfxScratchBuffer);
        sfxScratchIndex = i;
        sfxScratchPointer = sc
    }
    samples = sfxScratchBuffer;
    i = 0i32;
    while i < count {
        let fresh4 = sampleOffset;
        sampleOffset = sampleOffset + 1;
        data = *samples.offset(fresh4 as isize) as libc::c_int;
        (*samp.offset(i as isize)).left += data * leftvol >> 8i32;
        (*samp.offset(i as isize)).right += data * rightvol >> 8i32;
        if sampleOffset == 1024i32 * 2i32 {
            chunk = (*chunk).next;
            decodeWavelet(chunk, sfxScratchBuffer);
            sfxScratchIndex += 1;
            sampleOffset = 0i32
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_PaintChannelFromADPCM(mut ch: *mut channel_t,
                                                 mut sc: *mut sfx_t,
                                                 mut count: libc::c_int,
                                                 mut sampleOffset:
                                                     libc::c_int,
                                                 mut bufferOffset:
                                                     libc::c_int) {
    let mut data: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut samp: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    i = 0i32;
    samp =
        &mut paintbuffer[bufferOffset as usize] as *mut portable_samplepair_t;
    chunk = (*sc).soundData;
    if 0 != (*ch).doppler as u64 {
        sampleOffset =
            (sampleOffset as libc::c_float * (*ch).oldDopplerScale) as
                libc::c_int
    }
    while sampleOffset >= 1024i32 * 4i32 {
        chunk = (*chunk).next;
        sampleOffset -= 1024i32 * 4i32;
        i += 1
    }
    if i != sfxScratchIndex || sfxScratchPointer != sc {
        S_AdpcmGetSamples(chunk, sfxScratchBuffer);
        sfxScratchIndex = i;
        sfxScratchPointer = sc
    }
    samples = sfxScratchBuffer;
    i = 0i32;
    while i < count {
        let fresh5 = sampleOffset;
        sampleOffset = sampleOffset + 1;
        data = *samples.offset(fresh5 as isize) as libc::c_int;
        (*samp.offset(i as isize)).left += data * leftvol >> 8i32;
        (*samp.offset(i as isize)).right += data * rightvol >> 8i32;
        if sampleOffset == 1024i32 * 4i32 {
            chunk = (*chunk).next;
            S_AdpcmGetSamples(chunk, sfxScratchBuffer);
            sampleOffset = 0i32;
            sfxScratchIndex += 1
        }
        i += 1
    };
}