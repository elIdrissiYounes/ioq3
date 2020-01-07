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

static mut indexTable: [i32; 16] = [
    -(1),
    -(1),
    -(1),
    -(1),
    2,
    4,
    6,
    8,
    -(1),
    -(1),
    -(1),
    -(1),
    2,
    4,
    6,
    8,
];

static mut stepsizeTable: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmEncode(
    mut indata: *mut i16,
    mut outdata: *mut i8,
    mut len: i32,
    mut state: *mut crate::snd_local_h::adpcm_state,
) {
    let mut inp: *mut i16 = 0 as *mut i16; /* Input buffer pointer */
    let mut outp: *mut i8 = 0 as *mut i8; /* output buffer pointer */
    let mut val: i32 = 0; /* Current input sample value */
    let mut sign: i32 = 0; /* Current adpcm sign bit */
    let mut delta: i32 = 0; /* Current adpcm output value */
    let mut diff: i32 = 0; /* Difference between val and sample */
    let mut step: i32 = 0; /* Stepsize */
    let mut valpred: i32 = 0; /* Predicted output value */
    let mut vpdiff: i32 = 0; /* Current change to valpred */
    let mut index: i32 = 0; /* Current step change index */
    let mut outputbuffer: i32 = 0; /* place to keep previous 4-bit value */
    let mut bufferstep: i32 = 0; /* toggle between outputbuffer/output */
    outp = outdata; // quiet a compiler warning
    inp = indata;
    valpred = (*state).sample as i32;
    index = (*state).index as i32;
    step = stepsizeTable[index as usize];
    outputbuffer = 0;
    bufferstep = 1;
    while len > 0 {
        let fresh0 = inp;
        inp = inp.offset(1);
        val = *fresh0 as i32;
        /* Step 1 - compute difference with previous value */
        diff = val - valpred;
        sign = if diff < 0 { 8 } else { 0 };
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
        delta = 0;
        vpdiff = step >> 3;
        if diff >= step {
            delta = 4;
            diff -= step;
            vpdiff += step
        }
        step >>= 1;
        if diff >= step {
            delta |= 2;
            diff -= step;
            vpdiff += step
        }
        step >>= 1;
        if diff >= step {
            delta |= 1;
            vpdiff += step
        }
        /* Step 3 - Update previous value */
        if sign != 0 {
            valpred -= vpdiff
        } else {
            valpred += vpdiff
        }
        /* Step 4 - Clamp previous value to 16 bits */
        if valpred > 32767 {
            valpred = 32767
        } else if valpred < -(32768) {
            valpred = -(32768)
        }
        /* Step 5 - Assemble value, update index and step values */
        delta |= sign;
        index += indexTable[delta as usize];
        if index < 0 {
            index = 0
        }
        if index > 88 {
            index = 88
        }
        step = stepsizeTable[index as usize];
        /* Step 6 - Output value */
        if bufferstep != 0 {
            outputbuffer = delta << 4 & 0xf0
        } else {
            let fresh1 = outp;
            outp = outp.offset(1);
            *fresh1 = (delta & 0xf | outputbuffer) as i8
        }
        bufferstep = (bufferstep == 0) as i32;
        len -= 1
    }
    /* Output last step, if needed */
    if bufferstep == 0 {
        let fresh2 = outp;
        outp = outp.offset(1);
        *fresh2 = outputbuffer as i8
    }
    (*state).sample = valpred as i16;
    (*state).index = index as i8;
}
/* static */
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmDecode(
    mut indata: *const i8,
    mut outdata: *mut i16,
    mut len: i32,
    mut state: *mut crate::snd_local_h::adpcm_state,
) {
    let mut inp: *mut i8 = 0 as *mut i8; /* Input buffer pointer */
    let mut outp: i32 = 0; /* output buffer pointer */
    let mut sign: i32 = 0; /* Current adpcm sign bit */
    let mut delta: i32 = 0; /* Current adpcm output value */
    let mut step: i32 = 0; /* Stepsize */
    let mut valpred: i32 = 0; /* Predicted value */
    let mut vpdiff: i32 = 0; /* Current change to valpred */
    let mut index: i32 = 0; /* Current step change index */
    let mut inputbuffer: i32 = 0; /* place to keep next 4-bit value */
    let mut bufferstep: i32 = 0; /* toggle between inputbuffer/input */
    outp = 0; // quiet a compiler warning
    inp = indata as *mut i8;
    valpred = (*state).sample as i32;
    index = (*state).index as i32;
    step = stepsizeTable[index as usize];
    bufferstep = 0;
    inputbuffer = 0;
    while len > 0 {
        /* Step 1 - get the delta value */
        if bufferstep != 0 {
            delta = inputbuffer & 0xf
        } else {
            let fresh3 = inp;
            inp = inp.offset(1);
            inputbuffer = *fresh3 as i32;
            delta = inputbuffer >> 4 & 0xf
        }
        bufferstep = (bufferstep == 0) as i32;
        /* Step 2 - Find new index value (for later) */
        index += indexTable[delta as usize];
        if index < 0 {
            index = 0
        }
        if index > 88 {
            index = 88
        }
        /* Step 3 - Separate sign and magnitude */
        sign = delta & 8;
        delta = delta & 7;
        /* Step 4 - Compute difference and new predicted value */
        /*
        	** Computes 'vpdiff = (delta+0.5)*step/4', but see comment
        	** in adpcm_coder.
        	*/
        vpdiff = step >> 3;
        if delta & 4 != 0 {
            vpdiff += step
        }
        if delta & 2 != 0 {
            vpdiff += step >> 1
        }
        if delta & 1 != 0 {
            vpdiff += step >> 2
        }
        if sign != 0 {
            valpred -= vpdiff
        } else {
            valpred += vpdiff
        }
        /* Step 5 - clamp output value */
        if valpred > 32767 {
            valpred = 32767
        } else if valpred < -(32768) {
            valpred = -(32768)
        }
        /* Step 6 - Update step value */
        step = stepsizeTable[index as usize];
        /* Step 7 - Output value */
        *outdata.offset(outp as isize) = valpred as i16;
        outp += 1;
        len -= 1
    }
    (*state).sample = valpred as i16;
    (*state).index = index as i8;
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
) -> i32 {
    let mut scale: f32 = 0.;
    let mut scaledSampleCount: i32 = 0;
    let mut sampleMemory: i32 = 0;
    let mut blockCount: i32 = 0;
    let mut headerMemory: i32 = 0;
    // determine scale to convert from input sampling rate to desired sampling rate
    scale = (*info).rate as f32 / crate::src::client::snd_dma::dma.speed as f32;
    // calc number of samples at playback sampling rate
    scaledSampleCount = ((*info).samples as f32 / scale) as i32;
    // calc memory need to store those samples using ADPCM at 4 bits per sample
    sampleMemory = scaledSampleCount / 2;
    // calc number of sample blocks needed of PAINTBUFFER_SIZE
    blockCount = scaledSampleCount / 4096;
    if scaledSampleCount % 4096 != 0 {
        blockCount += 1
    }
    // calc memory needed to store the block headers
    headerMemory = (blockCount as usize)
        .wrapping_mul(::std::mem::size_of::<crate::snd_local_h::adpcm_state_t>())
        as i32;
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
    mut to: *mut i16,
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
    S_AdpcmDecode(out as *const i8, to, 1024 * 2 * 2, &mut state);
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
    mut samples: *mut i16,
) {
    let mut state: crate::snd_local_h::adpcm_state_t = crate::snd_local_h::adpcm_state_t {
        sample: 0,
        index: 0,
    };
    let mut inOffset: i32 = 0;
    let mut count: i32 = 0;
    let mut n: i32 = 0;
    let mut newchunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    inOffset = 0;
    count = (*sfx).soundLength;
    state.index = 0;
    state.sample = *samples.offset(0);
    chunk = 0 as *mut crate::snd_local_h::sndBuffer;
    while count != 0 {
        n = count;
        if n > 1024 * 2 * 2 {
            n = 1024 * 2 * 2
        }
        newchunk = crate::src::client::snd_mem::SND_malloc();
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
            out as *mut i8,
            n,
            &mut state,
        );
        inOffset += n;
        count -= n
    }
}
