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
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_local.h"]
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct wavinfo_t {
        pub format: libc::c_int,
        pub rate: libc::c_int,
        pub width: libc::c_int,
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub dataofs: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean, byte};
    extern "C" {
        #[no_mangle]
        pub static mut dma: dma_t;
        #[no_mangle]
        pub fn SND_malloc() -> *mut sndBuffer;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_adpcm.c"]
pub mod snd_adpcm_c {
    use super::{libc};
    use super::snd_local_h::{adpcm_state};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse};
use self::snd_local_h::{adpcm_state, adpcm_state_t, sndBuffer_s, sndBuffer,
                        sfx_s, sfx_t, dma_t, wavinfo_t, dma, SND_malloc};
// adpcm functions
#[no_mangle]
pub unsafe extern "C" fn S_AdpcmMemoryNeeded(mut info: *const wavinfo_t)
 -> libc::c_int {
    let mut scale: libc::c_float = 0.;
    let mut scaledSampleCount: libc::c_int = 0;
    let mut sampleMemory: libc::c_int = 0;
    let mut blockCount: libc::c_int = 0;
    let mut headerMemory: libc::c_int = 0;
    scale = (*info).rate as libc::c_float / dma.speed as libc::c_float;
    scaledSampleCount =
        ((*info).samples as libc::c_float / scale) as libc::c_int;
    sampleMemory = scaledSampleCount / 2i32;
    blockCount = scaledSampleCount / 4096i32;
    if 0 != scaledSampleCount % 4096i32 { blockCount += 1 }
    headerMemory =
        (blockCount as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<adpcm_state_t>()
                                             as libc::c_ulong) as libc::c_int;
    return sampleMemory + headerMemory;
}
#[no_mangle]
pub unsafe extern "C" fn S_AdpcmEncodeSound(mut sfx: *mut sfx_t,
                                            mut samples: *mut libc::c_short) {
    let mut state: adpcm_state_t = adpcm_state{sample: 0, index: 0,};
    let mut inOffset: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut newchunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut out: *mut byte = 0 as *mut byte;
    inOffset = 0i32;
    count = (*sfx).soundLength;
    state.index = 0i32 as libc::c_char;
    state.sample = *samples.offset(0isize);
    chunk = 0 as *mut sndBuffer;
    while 0 != count {
        n = count;
        if n > 1024i32 * 2i32 * 2i32 { n = 1024i32 * 2i32 * 2i32 }
        newchunk = SND_malloc();
        if (*sfx).soundData.is_null() {
            (*sfx).soundData = newchunk
        } else if !chunk.is_null() { (*chunk).next = newchunk }
        chunk = newchunk;
        (*chunk).adpcm.index = state.index;
        (*chunk).adpcm.sample = state.sample;
        out = (*chunk).sndChunk.as_mut_ptr() as *mut byte;
        S_AdpcmEncode(samples.offset(inOffset as isize),
                      out as *mut libc::c_char, n, &mut state);
        inOffset += n;
        count -= n
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_AdpcmEncode(mut indata: *mut libc::c_short,
                                       mut outdata: *mut libc::c_char,
                                       mut len: libc::c_int,
                                       mut state: *mut adpcm_state) {
    /* Input buffer pointer */
    let mut inp: *mut libc::c_short = 0 as *mut libc::c_short;
    /* output buffer pointer */
    let mut outp: *mut libc::c_schar = 0 as *mut libc::c_schar;
    /* Current input sample value */
    let mut val: libc::c_int = 0;
    /* Current adpcm sign bit */
    let mut sign: libc::c_int = 0;
    /* Current adpcm output value */
    let mut delta: libc::c_int = 0;
    /* Difference between val and sample */
    let mut diff: libc::c_int = 0;
    /* Stepsize */
    let mut step: libc::c_int = 0;
    /* Predicted output value */
    let mut valpred: libc::c_int = 0;
    /* Current change to valpred */
    let mut vpdiff: libc::c_int = 0;
    /* Current step change index */
    let mut index: libc::c_int = 0;
    /* place to keep previous 4-bit value */
    let mut outputbuffer: libc::c_int = 0;
    /* toggle between outputbuffer/output */
    let mut bufferstep: libc::c_int = 0;
    outp = outdata as *mut libc::c_schar;
    inp = indata;
    valpred = (*state).sample as libc::c_int;
    index = (*state).index as libc::c_int;
    step = stepsizeTable[index as usize];
    outputbuffer = 0i32;
    bufferstep = 1i32;
    while len > 0i32 {
        let fresh0 = inp;
        inp = inp.offset(1);
        val = *fresh0 as libc::c_int;
        diff = val - valpred;
        sign = if diff < 0i32 { 8i32 } else { 0i32 };
        if 0 != sign { diff = -diff }
        delta = 0i32;
        vpdiff = step >> 3i32;
        if diff >= step { delta = 4i32; diff -= step; vpdiff += step }
        step >>= 1i32;
        if diff >= step { delta |= 2i32; diff -= step; vpdiff += step }
        step >>= 1i32;
        if diff >= step { delta |= 1i32; vpdiff += step }
        if 0 != sign { valpred -= vpdiff } else { valpred += vpdiff }
        if valpred > 32767i32 {
            valpred = 32767i32
        } else if valpred < -32768i32 { valpred = -32768i32 }
        delta |= sign;
        index += indexTable[delta as usize];
        if index < 0i32 { index = 0i32 }
        if index > 88i32 { index = 88i32 }
        step = stepsizeTable[index as usize];
        if 0 != bufferstep {
            outputbuffer = delta << 4i32 & 0xf0i32
        } else {
            let fresh1 = outp;
            outp = outp.offset(1);
            *fresh1 = (delta & 0xfi32 | outputbuffer) as libc::c_schar
        }
        bufferstep = (0 == bufferstep) as libc::c_int;
        len -= 1
    }
    if 0 == bufferstep {
        let fresh2 = outp;
        outp = outp.offset(1);
        *fresh2 = outputbuffer as libc::c_schar
    }
    (*state).sample = valpred as libc::c_short;
    (*state).index = index as libc::c_char;
}
static mut stepsizeTable: [libc::c_int; 89] =
    [7i32, 8i32, 9i32, 10i32, 11i32, 12i32, 13i32, 14i32, 16i32, 17i32, 19i32,
     21i32, 23i32, 25i32, 28i32, 31i32, 34i32, 37i32, 41i32, 45i32, 50i32,
     55i32, 60i32, 66i32, 73i32, 80i32, 88i32, 97i32, 107i32, 118i32, 130i32,
     143i32, 157i32, 173i32, 190i32, 209i32, 230i32, 253i32, 279i32, 307i32,
     337i32, 371i32, 408i32, 449i32, 494i32, 544i32, 598i32, 658i32, 724i32,
     796i32, 876i32, 963i32, 1060i32, 1166i32, 1282i32, 1411i32, 1552i32,
     1707i32, 1878i32, 2066i32, 2272i32, 2499i32, 2749i32, 3024i32, 3327i32,
     3660i32, 4026i32, 4428i32, 4871i32, 5358i32, 5894i32, 6484i32, 7132i32,
     7845i32, 8630i32, 9493i32, 10442i32, 11487i32, 12635i32, 13899i32,
     15289i32, 16818i32, 18500i32, 20350i32, 22385i32, 24623i32, 27086i32,
     29794i32, 32767i32];
/* **********************************************************
Copyright 1992 by Stichting Mathematisch Centrum, Amsterdam, The
Netherlands.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its 
documentation for any purpose and without fee is hereby granted, 
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in 
supporting documentation, and that the names of Stichting Mathematisch
Centrum or CWI not be used in advertising or publicity pertaining to
distribution of the software without specific, written prior permission.

STICHTING MATHEMATISCH CENTRUM DISCLAIMS ALL WARRANTIES WITH REGARD TO
THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS, IN NO EVENT SHALL STICHTING MATHEMATISCH CENTRUM BE LIABLE
FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT
OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

******************************************************************/
/*
** Intel/DVI ADPCM coder/decoder.
**
** The algorithm for this coder was taken from the IMA Compatibility Project
** proceedings, Vol 2, Number 2; May 1992.
**
** Version 1.2, 18-Dec-92.
*/
/* Intel ADPCM step variation table */
static mut indexTable: [libc::c_int; 16] =
    [-1i32, -1i32, -1i32, -1i32, 2i32, 4i32, 6i32, 8i32, -1i32, -1i32, -1i32,
     -1i32, 2i32, 4i32, 6i32, 8i32];
#[no_mangle]
pub unsafe extern "C" fn S_AdpcmGetSamples(mut chunk: *mut sndBuffer,
                                           mut to: *mut libc::c_short) {
    let mut state: adpcm_state_t = adpcm_state{sample: 0, index: 0,};
    let mut out: *mut byte = 0 as *mut byte;
    state.index = (*chunk).adpcm.index;
    state.sample = (*chunk).adpcm.sample;
    out = (*chunk).sndChunk.as_mut_ptr() as *mut byte;
    S_AdpcmDecode(out as *mut libc::c_char as *const libc::c_char, to,
                  1024i32 * 2i32 * 2i32, &mut state);
}
#[no_mangle]
pub unsafe extern "C" fn S_AdpcmDecode(mut indata: *const libc::c_char,
                                       mut outdata: *mut libc::c_short,
                                       mut len: libc::c_int,
                                       mut state: *mut adpcm_state) {
    /* Input buffer pointer */
    let mut inp: *mut libc::c_schar = 0 as *mut libc::c_schar;
    /* output buffer pointer */
    let mut outp: libc::c_int = 0;
    /* Current adpcm sign bit */
    let mut sign: libc::c_int = 0;
    /* Current adpcm output value */
    let mut delta: libc::c_int = 0;
    /* Stepsize */
    let mut step: libc::c_int = 0;
    /* Predicted value */
    let mut valpred: libc::c_int = 0;
    /* Current change to valpred */
    let mut vpdiff: libc::c_int = 0;
    /* Current step change index */
    let mut index: libc::c_int = 0;
    /* place to keep next 4-bit value */
    let mut inputbuffer: libc::c_int = 0;
    /* toggle between inputbuffer/input */
    let mut bufferstep: libc::c_int = 0;
    outp = 0i32;
    inp = indata as *mut libc::c_schar;
    valpred = (*state).sample as libc::c_int;
    index = (*state).index as libc::c_int;
    step = stepsizeTable[index as usize];
    bufferstep = 0i32;
    inputbuffer = 0i32;
    while len > 0i32 {
        if 0 != bufferstep {
            delta = inputbuffer & 0xfi32
        } else {
            let fresh3 = inp;
            inp = inp.offset(1);
            inputbuffer = *fresh3 as libc::c_int;
            delta = inputbuffer >> 4i32 & 0xfi32
        }
        bufferstep = (0 == bufferstep) as libc::c_int;
        index += indexTable[delta as usize];
        if index < 0i32 { index = 0i32 }
        if index > 88i32 { index = 88i32 }
        sign = delta & 8i32;
        delta = delta & 7i32;
        vpdiff = step >> 3i32;
        if 0 != delta & 4i32 { vpdiff += step }
        if 0 != delta & 2i32 { vpdiff += step >> 1i32 }
        if 0 != delta & 1i32 { vpdiff += step >> 2i32 }
        if 0 != sign { valpred -= vpdiff } else { valpred += vpdiff }
        if valpred > 32767i32 {
            valpred = 32767i32
        } else if valpred < -32768i32 { valpred = -32768i32 }
        step = stepsizeTable[index as usize];
        *outdata.offset(outp as isize) = valpred as libc::c_short;
        outp += 1;
        len -= 1
    }
    (*state).sample = valpred as libc::c_short;
    (*state).index = index as libc::c_char;
}