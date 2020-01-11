use ::libc;

pub use crate::snd_local_h::adpcm_state;
pub use crate::snd_local_h::adpcm_state_t;
pub use crate::snd_local_h::dma_t;
pub use crate::snd_local_h::sfx_s;
pub use crate::snd_local_h::sfx_t;
pub use crate::snd_local_h::sndBuffer;
pub use crate::snd_local_h::sndBuffer_s;
pub use crate::snd_local_h::wavinfo_t;
pub use crate::src::client::snd_dma::dma;
pub use crate::src::client::snd_mem::SND_malloc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
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

static mut indexTable: [libc::c_int; 16] = [
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    2 as libc::c_int,
    4 as libc::c_int,
    6 as libc::c_int,
    8 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    2 as libc::c_int,
    4 as libc::c_int,
    6 as libc::c_int,
    8 as libc::c_int,
];

static mut stepsizeTable: [libc::c_int; 89] = [
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    19 as libc::c_int,
    21 as libc::c_int,
    23 as libc::c_int,
    25 as libc::c_int,
    28 as libc::c_int,
    31 as libc::c_int,
    34 as libc::c_int,
    37 as libc::c_int,
    41 as libc::c_int,
    45 as libc::c_int,
    50 as libc::c_int,
    55 as libc::c_int,
    60 as libc::c_int,
    66 as libc::c_int,
    73 as libc::c_int,
    80 as libc::c_int,
    88 as libc::c_int,
    97 as libc::c_int,
    107 as libc::c_int,
    118 as libc::c_int,
    130 as libc::c_int,
    143 as libc::c_int,
    157 as libc::c_int,
    173 as libc::c_int,
    190 as libc::c_int,
    209 as libc::c_int,
    230 as libc::c_int,
    253 as libc::c_int,
    279 as libc::c_int,
    307 as libc::c_int,
    337 as libc::c_int,
    371 as libc::c_int,
    408 as libc::c_int,
    449 as libc::c_int,
    494 as libc::c_int,
    544 as libc::c_int,
    598 as libc::c_int,
    658 as libc::c_int,
    724 as libc::c_int,
    796 as libc::c_int,
    876 as libc::c_int,
    963 as libc::c_int,
    1060 as libc::c_int,
    1166 as libc::c_int,
    1282 as libc::c_int,
    1411 as libc::c_int,
    1552 as libc::c_int,
    1707 as libc::c_int,
    1878 as libc::c_int,
    2066 as libc::c_int,
    2272 as libc::c_int,
    2499 as libc::c_int,
    2749 as libc::c_int,
    3024 as libc::c_int,
    3327 as libc::c_int,
    3660 as libc::c_int,
    4026 as libc::c_int,
    4428 as libc::c_int,
    4871 as libc::c_int,
    5358 as libc::c_int,
    5894 as libc::c_int,
    6484 as libc::c_int,
    7132 as libc::c_int,
    7845 as libc::c_int,
    8630 as libc::c_int,
    9493 as libc::c_int,
    10442 as libc::c_int,
    11487 as libc::c_int,
    12635 as libc::c_int,
    13899 as libc::c_int,
    15289 as libc::c_int,
    16818 as libc::c_int,
    18500 as libc::c_int,
    20350 as libc::c_int,
    22385 as libc::c_int,
    24623 as libc::c_int,
    27086 as libc::c_int,
    29794 as libc::c_int,
    32767 as libc::c_int,
];
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmEncode(
    mut indata: *mut libc::c_short,
    mut outdata: *mut libc::c_char,
    mut len: libc::c_int,
    mut state: *mut crate::snd_local_h::adpcm_state,
) {
    let mut inp: *mut libc::c_short = 0 as *mut libc::c_short; /* Input buffer pointer */
    let mut outp: *mut libc::c_schar = 0 as *mut libc::c_schar; /* output buffer pointer */
    let mut val: libc::c_int = 0; /* Current input sample value */
    let mut sign: libc::c_int = 0; /* Current adpcm sign bit */
    let mut delta: libc::c_int = 0; /* Current adpcm output value */
    let mut diff: libc::c_int = 0; /* Difference between val and sample */
    let mut step: libc::c_int = 0; /* Stepsize */
    let mut valpred: libc::c_int = 0; /* Predicted output value */
    let mut vpdiff: libc::c_int = 0; /* Current change to valpred */
    let mut index: libc::c_int = 0; /* Current step change index */
    let mut outputbuffer: libc::c_int = 0; /* place to keep previous 4-bit value */
    let mut bufferstep: libc::c_int = 0; /* toggle between outputbuffer/output */
    outp = outdata as *mut libc::c_schar; // quiet a compiler warning
    inp = indata;
    valpred = (*state).sample as libc::c_int;
    index = (*state).index as libc::c_int;
    step = stepsizeTable[index as usize];
    outputbuffer = 0 as libc::c_int;
    bufferstep = 1 as libc::c_int;
    while len > 0 as libc::c_int {
        let fresh0 = inp;
        inp = inp.offset(1);
        val = *fresh0 as libc::c_int;
        /* Step 1 - compute difference with previous value */
        diff = val - valpred;
        sign = if diff < 0 as libc::c_int {
            8 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if sign != 0 {
            diff = -diff
        }
        /* Step 2 - Divide and clamp */
        /* Note:
        	** This code *approximately* computes:
        	**    delta = diff*4/step;
        	**    vpdiff = (delta+0.5)*step/4;
        	** but in shift step bits are dropped. The net result of this is
        	** that even if you have fast mul/div hardware you cannot put it to
        	** good use since the fixup would be too expensive.
        	*/
        delta = 0 as libc::c_int;
        vpdiff = step >> 3 as libc::c_int;
        if diff >= step {
            delta = 4 as libc::c_int;
            diff -= step;
            vpdiff += step
        }
        step >>= 1 as libc::c_int;
        if diff >= step {
            delta |= 2 as libc::c_int;
            diff -= step;
            vpdiff += step
        }
        step >>= 1 as libc::c_int;
        if diff >= step {
            delta |= 1 as libc::c_int;
            vpdiff += step
        }
        /* Step 3 - Update previous value */
        if sign != 0 {
            valpred -= vpdiff
        } else {
            valpred += vpdiff
        }
        /* Step 4 - Clamp previous value to 16 bits */
        if valpred > 32767 as libc::c_int {
            valpred = 32767 as libc::c_int
        } else if valpred < -(32768 as libc::c_int) {
            valpred = -(32768 as libc::c_int)
        }
        /* Step 5 - Assemble value, update index and step values */
        delta |= sign;
        index += indexTable[delta as usize];
        if index < 0 as libc::c_int {
            index = 0 as libc::c_int
        }
        if index > 88 as libc::c_int {
            index = 88 as libc::c_int
        }
        step = stepsizeTable[index as usize];
        /* Step 6 - Output value */
        if bufferstep != 0 {
            outputbuffer = delta << 4 as libc::c_int & 0xf0 as libc::c_int
        } else {
            let fresh1 = outp;
            outp = outp.offset(1);
            *fresh1 = (delta & 0xf as libc::c_int | outputbuffer) as libc::c_schar
        }
        bufferstep = (bufferstep == 0) as libc::c_int;
        len -= 1
    }
    /* Output last step, if needed */
    if bufferstep == 0 {
        let fresh2 = outp;
        outp = outp.offset(1);
        *fresh2 = outputbuffer as libc::c_schar
    }
    (*state).sample = valpred as libc::c_short;
    (*state).index = index as libc::c_char;
}
/* static */
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmDecode(
    mut indata: *const libc::c_char,
    mut outdata: *mut libc::c_short,
    mut len: libc::c_int,
    mut state: *mut crate::snd_local_h::adpcm_state,
) {
    let mut inp: *mut libc::c_schar = 0 as *mut libc::c_schar; /* Input buffer pointer */
    let mut outp: libc::c_int = 0; /* output buffer pointer */
    let mut sign: libc::c_int = 0; /* Current adpcm sign bit */
    let mut delta: libc::c_int = 0; /* Current adpcm output value */
    let mut step: libc::c_int = 0; /* Stepsize */
    let mut valpred: libc::c_int = 0; /* Predicted value */
    let mut vpdiff: libc::c_int = 0; /* Current change to valpred */
    let mut index: libc::c_int = 0; /* Current step change index */
    let mut inputbuffer: libc::c_int = 0; /* place to keep next 4-bit value */
    let mut bufferstep: libc::c_int = 0; /* toggle between inputbuffer/input */
    outp = 0 as libc::c_int; // quiet a compiler warning
    inp = indata as *mut libc::c_schar;
    valpred = (*state).sample as libc::c_int;
    index = (*state).index as libc::c_int;
    step = stepsizeTable[index as usize];
    bufferstep = 0 as libc::c_int;
    inputbuffer = 0 as libc::c_int;
    while len > 0 as libc::c_int {
        /* Step 1 - get the delta value */
        if bufferstep != 0 {
            delta = inputbuffer & 0xf as libc::c_int
        } else {
            let fresh3 = inp;
            inp = inp.offset(1);
            inputbuffer = *fresh3 as libc::c_int;
            delta = inputbuffer >> 4 as libc::c_int & 0xf as libc::c_int
        }
        bufferstep = (bufferstep == 0) as libc::c_int;
        /* Step 2 - Find new index value (for later) */
        index += indexTable[delta as usize];
        if index < 0 as libc::c_int {
            index = 0 as libc::c_int
        }
        if index > 88 as libc::c_int {
            index = 88 as libc::c_int
        }
        /* Step 3 - Separate sign and magnitude */
        sign = delta & 8 as libc::c_int;
        delta = delta & 7 as libc::c_int;
        /* Step 4 - Compute difference and new predicted value */
        /*
        	** Computes 'vpdiff = (delta+0.5)*step/4', but see comment
        	** in adpcm_coder.
        	*/
        vpdiff = step >> 3 as libc::c_int;
        if delta & 4 as libc::c_int != 0 {
            vpdiff += step
        }
        if delta & 2 as libc::c_int != 0 {
            vpdiff += step >> 1 as libc::c_int
        }
        if delta & 1 as libc::c_int != 0 {
            vpdiff += step >> 2 as libc::c_int
        }
        if sign != 0 {
            valpred -= vpdiff
        } else {
            valpred += vpdiff
        }
        /* Step 5 - clamp output value */
        if valpred > 32767 as libc::c_int {
            valpred = 32767 as libc::c_int
        } else if valpred < -(32768 as libc::c_int) {
            valpred = -(32768 as libc::c_int)
        }
        /* Step 6 - Update step value */
        step = stepsizeTable[index as usize];
        /* Step 7 - Output value */
        *outdata.offset(outp as isize) = valpred as libc::c_short;
        outp += 1;
        len -= 1
    }
    (*state).sample = valpred as libc::c_short;
    (*state).index = index as libc::c_char;
}
/*
====================
S_AdpcmMemoryNeeded

Returns the amount of memory (in bytes) needed to store the samples in out internal adpcm format
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmMemoryNeeded(
    mut info: *const crate::snd_local_h::wavinfo_t,
) -> libc::c_int {
    let mut scale: libc::c_float = 0.;
    let mut scaledSampleCount: libc::c_int = 0;
    let mut sampleMemory: libc::c_int = 0;
    let mut blockCount: libc::c_int = 0;
    let mut headerMemory: libc::c_int = 0;
    // determine scale to convert from input sampling rate to desired sampling rate
    scale = (*info).rate as libc::c_float / crate::src::client::snd_dma::dma.speed as libc::c_float;
    // calc number of samples at playback sampling rate
    scaledSampleCount = ((*info).samples as libc::c_float / scale) as libc::c_int;
    // calc memory need to store those samples using ADPCM at 4 bits per sample
    sampleMemory = scaledSampleCount / 2 as libc::c_int;
    // calc number of sample blocks needed of PAINTBUFFER_SIZE
    blockCount = scaledSampleCount / 4096 as libc::c_int;
    if scaledSampleCount % 4096 as libc::c_int != 0 {
        blockCount += 1
    }
    // calc memory needed to store the block headers
    headerMemory = (blockCount as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<crate::snd_local_h::adpcm_state_t>() as libc::c_ulong)
        as libc::c_int;
    return sampleMemory + headerMemory;
}
/*
====================
S_AdpcmGetSamples
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmGetSamples(
    mut chunk: *mut crate::snd_local_h::sndBuffer,
    mut to: *mut libc::c_short,
) {
    let mut state: crate::snd_local_h::adpcm_state_t = crate::snd_local_h::adpcm_state_t {
        sample: 0,
        index: 0,
    };
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    // get the starting state from the block header
    state.index = (*chunk).adpcm.index;
    state.sample = (*chunk).adpcm.sample;
    out = (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
    // get samples
    S_AdpcmDecode(
        out as *mut libc::c_char as *const libc::c_char,
        to,
        1024 as libc::c_int * 2 as libc::c_int * 2 as libc::c_int,
        &mut state,
    );
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
// snd_local.h -- private sound definations
// this is in samples
// samples
// floats
// floats
// the final values will be clamped to +/- 0x00ffff00 and shifted down
/* Previous output value */
/* Index into stepsize table */
// couldn't be loaded, so use buzz
// not in Memory
// not in Memory
// mono samples in buffer
// samples with all channels in buffer (samples divided by channels)
// don't mix less than this #
//arbitrary
// START_SAMPLE_IMMEDIATE = set immediately on next mix
// to allow overriding a specific sound
// to allow overriding a specific sound
// 0-255 volume after spatialization
// 0-255 volume after spatialization
// 0-255 volume before spatialization
// only use if fixed_origin is set
// use origin instead of fetching entnum's origin
// sfx structure
// chunk starts this many bytes from file start
// Interface between Q3 sound "api" and the sound backend
/*
====================================================================

  SYSTEM SPECIFIC FUNCTIONS

====================================================================
*/
// initializes cycling through a DMA buffer and returns information on it
// gets the current DMA position
// shutdown the DMA xfer.
//====================================================================
// spatializes a channel
// adpcm functions
/*
====================
S_AdpcmEncodeSound
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmEncodeSound(
    mut sfx: *mut crate::snd_local_h::sfx_t,
    mut samples: *mut libc::c_short,
) {
    let mut state: crate::snd_local_h::adpcm_state_t = crate::snd_local_h::adpcm_state_t {
        sample: 0,
        index: 0,
    };
    let mut inOffset: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut newchunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    inOffset = 0 as libc::c_int;
    count = (*sfx).soundLength;
    state.index = 0 as libc::c_int as libc::c_char;
    state.sample = *samples.offset(0 as libc::c_int as isize);
    chunk = 0 as *mut crate::snd_local_h::sndBuffer;
    while count != 0 {
        n = count;
        if n > 1024 as libc::c_int * 2 as libc::c_int * 2 as libc::c_int {
            n = 1024 as libc::c_int * 2 as libc::c_int * 2 as libc::c_int
        }
        newchunk =
            crate::src::client::snd_mem::SND_malloc() as *mut crate::snd_local_h::sndBuffer_s;
        if (*sfx).soundData.is_null() {
            (*sfx).soundData = newchunk
        } else if !chunk.is_null() {
            (*chunk).next = newchunk
        }
        chunk = newchunk;
        // output the header
        (*chunk).adpcm.index = state.index;
        (*chunk).adpcm.sample = state.sample;
        out = (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
        // encode the samples
        S_AdpcmEncode(
            samples.offset(inOffset as isize),
            out as *mut libc::c_char,
            n,
            &mut state,
        );
        inOffset += n;
        count -= n
    }
}
