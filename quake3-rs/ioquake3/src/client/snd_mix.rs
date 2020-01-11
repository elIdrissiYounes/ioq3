use ::libc;

pub use crate::snd_local_h::adpcm_state;
pub use crate::snd_local_h::adpcm_state_t;
pub use crate::snd_local_h::channel_t;
pub use crate::snd_local_h::dma_t;
pub use crate::snd_local_h::portable_samplepair_t;
pub use crate::snd_local_h::sfx_s;
pub use crate::snd_local_h::sfx_t;
pub use crate::snd_local_h::sndBuffer;
pub use crate::snd_local_h::sndBuffer_s;

pub use crate::src::client::snd_adpcm::S_AdpcmGetSamples;
pub use crate::src::client::snd_dma::dma;
pub use crate::src::client::snd_dma::loop_channels;
pub use crate::src::client::snd_dma::numLoopChannels;
pub use crate::src::client::snd_dma::s_channels;
pub use crate::src::client::snd_dma::s_paintedtime;
pub use crate::src::client::snd_dma::s_rawend;
pub use crate::src::client::snd_dma::s_rawsamples;
pub use crate::src::client::snd_dma::s_testsound;
pub use crate::src::client::snd_main::s_muted;
pub use crate::src::client::snd_main::s_volume;
pub use crate::src::client::snd_mem::sfxScratchBuffer;
pub use crate::src::client::snd_mem::sfxScratchIndex;
pub use crate::src::client::snd_mem::sfxScratchPointer;
pub use crate::src::client::snd_wavelet::decodeWavelet;
pub use crate::src::client::snd_wavelet::mulawToShort;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;

/* Previous output value */
/* Index into stepsize table */
// couldn't be loaded, so use buzz
// not in Memory
// not in Memory
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

static mut paintbuffer: [crate::snd_local_h::portable_samplepair_t; 4096] =
    [crate::snd_local_h::portable_samplepair_t { left: 0, right: 0 }; 4096];

static mut snd_vol: i32 = 0;
#[no_mangle]

pub static mut snd_p: *mut i32 = 0 as *mut i32;
#[no_mangle]

pub static mut snd_linear_count: i32 = 0;
#[no_mangle]

pub static mut snd_out: *mut i16 = 0 as *mut i16;
// if configured not to use asm
#[no_mangle]

pub unsafe extern "C" fn S_WriteLinearBlastStereo16() {
    let mut i: i32 = 0;
    let mut val: i32 = 0;
    i = 0;
    while i < snd_linear_count {
        val = *snd_p.offset(i as isize) >> 8;
        if val > 0x7fff {
            *snd_out.offset(i as isize) = 0x7fffi16
        } else if val < -(32768) {
            *snd_out.offset(i as isize) = -(32768i32) as i16
        } else {
            *snd_out.offset(i as isize) = val as i16
        }
        val = *snd_p.offset((i + 1) as isize) >> 8;
        if val > 0x7fff {
            *snd_out.offset((i + 1) as isize) = 0x7fffi16
        } else if val < -(32768) {
            *snd_out.offset((i + 1) as isize) = -(32768i32) as i16
        } else {
            *snd_out.offset((i + 1) as isize) = val as i16
        }
        i += 2
    }
}
#[no_mangle]

pub unsafe extern "C" fn S_TransferStereo16(mut pbuf: *mut usize, mut endtime: i32) {
    let mut lpos: i32 = 0;
    let mut ls_paintedtime: i32 = 0;
    snd_p = paintbuffer.as_mut_ptr() as *mut i32;
    ls_paintedtime = crate::src::client::snd_dma::s_paintedtime;
    while ls_paintedtime < endtime {
        // handle recirculating buffer issues
        lpos = ls_paintedtime % crate::src::client::snd_dma::dma.fullsamples;
        // snd_linear_count * (dma.samplebits/8)
        snd_out = (pbuf as *mut i16).offset((lpos << 1) as isize); // lpos * dma.channels
        snd_linear_count = crate::src::client::snd_dma::dma.fullsamples - lpos; // snd_linear_count *= dma.channels
        if ls_paintedtime + snd_linear_count > endtime {
            snd_linear_count = endtime - ls_paintedtime
        }
        snd_linear_count <<= 1;
        S_WriteLinearBlastStereo16();
        snd_p = snd_p.offset(snd_linear_count as isize);
        ls_paintedtime += snd_linear_count >> 1;
        if crate::src::client::cl_avi::CL_VideoRecording() as u64 != 0 {
            crate::src::client::cl_avi::CL_WriteAVIAudioFrame(
                snd_out as *mut crate::src::qcommon::q_shared::byte,
                snd_linear_count << 1i32,
            );
        }
    }
}
// write a linear blast of samples
// snd_linear_count / dma.channels
/*
===================
S_TransferPaintBuffer

===================
*/
#[no_mangle]

pub unsafe extern "C" fn S_TransferPaintBuffer(mut endtime: i32) {
    let mut out_idx: i32 = 0;
    let mut count: i32 = 0;
    let mut p: *mut i32 = 0 as *mut i32;
    let mut step: i32 = 0;
    let mut val: i32 = 0;
    let mut i: i32 = 0;
    let mut pbuf: *mut usize = 0 as *mut usize;
    pbuf = crate::src::client::snd_dma::dma.buffer as *mut usize;
    if (*crate::src::client::snd_dma::s_testsound).integer != 0 {
        // write a fixed sine wave
        count = endtime - crate::src::client::snd_dma::s_paintedtime;
        i = 0;
        while i < count {
            paintbuffer[i as usize].right =
                (crate::stdlib::sin((crate::src::client::snd_dma::s_paintedtime + i) as f64 * 0.1)
                    * 20000f64
                    * 256f64) as i32;
            paintbuffer[i as usize].left = paintbuffer[i as usize].right;
            i += 1
        }
    }
    if crate::src::client::snd_dma::dma.samplebits == 16
        && crate::src::client::snd_dma::dma.channels == 2
    {
        // optimized case
        S_TransferStereo16(pbuf, endtime);
    } else {
        // general case
        p = paintbuffer.as_mut_ptr() as *mut i32;
        count = (endtime - crate::src::client::snd_dma::s_paintedtime)
            * crate::src::client::snd_dma::dma.channels;
        out_idx = crate::src::client::snd_dma::s_paintedtime
            * crate::src::client::snd_dma::dma.channels
            % crate::src::client::snd_dma::dma.samples;
        step = 3
            - (if crate::src::client::snd_dma::dma.channels < 2 {
                crate::src::client::snd_dma::dma.channels
            } else {
                2
            });
        if crate::src::client::snd_dma::dma.isfloat != 0
            && crate::src::client::snd_dma::dma.samplebits == 32
        {
            let mut out: *mut f32 = pbuf as *mut f32;
            i = 0;
            while i < count {
                if i % crate::src::client::snd_dma::dma.channels >= 2 {
                    val = 0
                } else {
                    val = *p >> 8;
                    p = p.offset(step as isize)
                }
                if val > 0x7fff {
                    val = 0x7fff
                } else if val < -(32767) {
                    /* clamp to one less than max to make division max out at -1.0f. */
                    val = -(32767)
                }
                *out.offset(out_idx as isize) = val as f32 / 32767.0;
                out_idx = (out_idx + 1) % crate::src::client::snd_dma::dma.samples;
                i += 1
            }
        } else if crate::src::client::snd_dma::dma.samplebits == 16 {
            let mut out_0: *mut i16 = pbuf as *mut i16;
            i = 0;
            while i < count {
                if i % crate::src::client::snd_dma::dma.channels >= 2 {
                    val = 0
                } else {
                    val = *p >> 8;
                    p = p.offset(step as isize)
                }
                if val > 0x7fff {
                    val = 0x7fff
                } else if val < -(32768) {
                    val = -(32768)
                }
                *out_0.offset(out_idx as isize) = val as i16;
                out_idx = (out_idx + 1) % crate::src::client::snd_dma::dma.samples;
                i += 1
            }
        } else if crate::src::client::snd_dma::dma.samplebits == 8 {
            let mut out_1: *mut u8 = pbuf as *mut u8;
            i = 0;
            while i < count {
                if i % crate::src::client::snd_dma::dma.channels >= 2 {
                    val = 0
                } else {
                    val = *p >> 8;
                    p = p.offset(step as isize)
                }
                if val > 0x7fff {
                    val = 0x7fff
                } else if val < -(32768) {
                    val = -(32768)
                }
                *out_1.offset(out_idx as isize) = ((val >> 8) + 128) as u8;
                out_idx = (out_idx + 1) % crate::src::client::snd_dma::dma.samples;
                i += 1
            }
        }
    };
}
/*
===============================================================================

CHANNEL MIXING

===============================================================================
*/

unsafe extern "C" fn S_PaintChannelFrom16_scalar(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *const crate::snd_local_h::sfx_t,
    mut count: i32,
    mut sampleOffset: i32,
    mut bufferOffset: i32,
) {
    let mut data: i32 = 0;
    let mut aoff: i32 = 0;
    let mut boff: i32 = 0;
    let mut leftvol: i32 = 0;
    let mut rightvol: i32 = 0;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut i16 = 0 as *mut i16;
    let mut ooff: f32 = 0.;
    let mut fdata: [f32; 2] = [0.; 2];
    let mut fdiv: f32 = 0.;
    let mut fleftvol: f32 = 0.;
    let mut frightvol: f32 = 0.;
    if (*sc).soundChannels <= 0 {
        return;
    }
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    if (*ch).doppler as u64 != 0 {
        sampleOffset = (sampleOffset as f32 * (*ch).oldDopplerScale) as i32
    }
    if (*sc).soundChannels == 2 {
        sampleOffset *= (*sc).soundChannels;
        if sampleOffset & 1 != 0 {
            sampleOffset &= !(1)
        }
    }
    chunk = (*sc).soundData;
    while sampleOffset >= 1024 {
        chunk = (*chunk).next;
        sampleOffset -= 1024;
        if chunk.is_null() {
            chunk = (*sc).soundData
        }
    }
    if (*ch).doppler as u64 == 0 || (*ch).dopplerScale == 1.0f32 {
        leftvol = (*ch).leftvol * snd_vol;
        rightvol = (*ch).rightvol * snd_vol;
        samples = (*chunk).sndChunk.as_mut_ptr();
        i = 0;
        while i < count {
            let fresh0 = sampleOffset;
            sampleOffset = sampleOffset + 1;
            data = *samples.offset(fresh0 as isize) as i32;
            (*samp.offset(i as isize)).left += data * leftvol >> 8;
            if (*sc).soundChannels == 2 {
                let fresh1 = sampleOffset;
                sampleOffset = sampleOffset + 1;
                data = *samples.offset(fresh1 as isize) as i32
            }
            (*samp.offset(i as isize)).right += data * rightvol >> 8;
            if sampleOffset == 1024 {
                chunk = (*chunk).next;
                samples = (*chunk).sndChunk.as_mut_ptr();
                sampleOffset = 0
            }
            i += 1
        }
    } else {
        fleftvol = ((*ch).leftvol * snd_vol) as f32;
        frightvol = ((*ch).rightvol * snd_vol) as f32;
        ooff = sampleOffset as f32;
        samples = (*chunk).sndChunk.as_mut_ptr();
        i = 0;
        while i < count {
            aoff = ooff as i32;
            ooff = ooff + (*ch).dopplerScale * (*sc).soundChannels as f32;
            boff = ooff as i32;
            fdata[1] = 0f32;
            fdata[0] = fdata[1];

            for j in (aoff..boff).step_by((*sc).soundChannels as usize) {
                if j == 1024 {
                    chunk = (*chunk).next;
                    if chunk.is_null() {
                        chunk = (*sc).soundData
                    }
                    samples = (*chunk).sndChunk.as_mut_ptr();
                    ooff -= 1024f32
                }

                if (*sc).soundChannels == 2 {
                    fdata[0] += *samples.offset((j & 1024 - 1) as isize) as i32 as f32;
                    fdata[1] += *samples.offset((j + 1 & 1024 - 1) as isize) as i32 as f32
                } else {
                    fdata[0] += *samples.offset((j & 1024 - 1) as isize) as i32 as f32;
                    fdata[1] += *samples.offset((j & 1024 - 1) as isize) as i32 as f32
                }
            }
            fdiv = (256 * (boff - aoff) / (*sc).soundChannels) as f32;
            let ref mut fresh2 = (*samp.offset(i as isize)).left;
            *fresh2 = (*fresh2 as f32 + fdata[0] * fleftvol / fdiv) as i32;
            let ref mut fresh3 = (*samp.offset(i as isize)).right;
            *fresh3 = (*fresh3 as f32 + fdata[1] * frightvol / fdiv) as i32;
            i += 1
        }
    };
}

unsafe extern "C" fn S_PaintChannelFrom16(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *const crate::snd_local_h::sfx_t,
    mut count: i32,
    mut sampleOffset: i32,
    mut bufferOffset: i32,
) {
    S_PaintChannelFrom16_scalar(ch, sc, count, sampleOffset, bufferOffset);
}
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannelFromWavelet(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *mut crate::snd_local_h::sfx_t,
    mut count: i32,
    mut sampleOffset: i32,
    mut bufferOffset: i32,
) {
    let mut data: i32 = 0;
    let mut leftvol: i32 = 0;
    let mut rightvol: i32 = 0;
    let mut i: i32 = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut i16 = 0 as *mut i16;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    i = 0;
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    chunk = (*sc).soundData;
    while sampleOffset >= 1024 / 2 * 4 {
        chunk = (*chunk).next;
        sampleOffset -= 1024 / 2 * 4;
        i += 1
    }
    if i != crate::src::client::snd_mem::sfxScratchIndex
        || crate::src::client::snd_mem::sfxScratchPointer != sc
    {
        crate::src::client::snd_adpcm::S_AdpcmGetSamples(
            chunk,
            crate::src::client::snd_mem::sfxScratchBuffer,
        );
        crate::src::client::snd_mem::sfxScratchIndex = i;
        crate::src::client::snd_mem::sfxScratchPointer = sc
    }
    samples = crate::src::client::snd_mem::sfxScratchBuffer;
    i = 0;
    while i < count {
        let fresh4 = sampleOffset;
        sampleOffset = sampleOffset + 1;
        data = *samples.offset(fresh4 as isize) as i32;
        (*samp.offset(i as isize)).left += data * leftvol >> 8;
        (*samp.offset(i as isize)).right += data * rightvol >> 8;
        if sampleOffset == 1024 * 2 {
            chunk = (*chunk).next;
            crate::src::client::snd_wavelet::decodeWavelet(
                chunk,
                crate::src::client::snd_mem::sfxScratchBuffer,
            );
            crate::src::client::snd_mem::sfxScratchIndex += 1;
            sampleOffset = 0
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannelFromADPCM(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *mut crate::snd_local_h::sfx_t,
    mut count: i32,
    mut sampleOffset: i32,
    mut bufferOffset: i32,
) {
    let mut data: i32 = 0;
    let mut leftvol: i32 = 0;
    let mut rightvol: i32 = 0;
    let mut i: i32 = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut i16 = 0 as *mut i16;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    i = 0;
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    chunk = (*sc).soundData;
    if (*ch).doppler as u64 != 0 {
        sampleOffset = (sampleOffset as f32 * (*ch).oldDopplerScale) as i32
    }
    while sampleOffset >= 1024 * 4 {
        chunk = (*chunk).next;
        sampleOffset -= 1024 * 4;
        i += 1
    }
    if i != crate::src::client::snd_mem::sfxScratchIndex
        || crate::src::client::snd_mem::sfxScratchPointer != sc
    {
        crate::src::client::snd_adpcm::S_AdpcmGetSamples(
            chunk,
            crate::src::client::snd_mem::sfxScratchBuffer,
        );
        crate::src::client::snd_mem::sfxScratchIndex = i;
        crate::src::client::snd_mem::sfxScratchPointer = sc
    }
    samples = crate::src::client::snd_mem::sfxScratchBuffer;
    i = 0;
    while i < count {
        let fresh5 = sampleOffset;
        sampleOffset = sampleOffset + 1;
        data = *samples.offset(fresh5 as isize) as i32;
        (*samp.offset(i as isize)).left += data * leftvol >> 8;
        (*samp.offset(i as isize)).right += data * rightvol >> 8;
        if sampleOffset == 1024 * 4 {
            chunk = (*chunk).next;
            crate::src::client::snd_adpcm::S_AdpcmGetSamples(
                chunk,
                crate::src::client::snd_mem::sfxScratchBuffer,
            );
            sampleOffset = 0;
            crate::src::client::snd_mem::sfxScratchIndex += 1
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannelFromMuLaw(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *mut crate::snd_local_h::sfx_t,
    mut count: i32,
    mut sampleOffset: i32,
    mut bufferOffset: i32,
) {
    let mut data: i32 = 0;
    let mut leftvol: i32 = 0;
    let mut rightvol: i32 = 0;
    let mut i: i32 = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut ooff: f32 = 0.;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    chunk = (*sc).soundData;
    while sampleOffset >= 1024 * 2 {
        chunk = (*chunk).next;
        sampleOffset -= 1024 * 2;
        if chunk.is_null() {
            chunk = (*sc).soundData
        }
    }
    if (*ch).doppler as u64 == 0 {
        samples = ((*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
            .offset(sampleOffset as isize);
        i = 0;
        while i < count {
            data = crate::src::client::snd_wavelet::mulawToShort[*samples as usize] as i32;
            (*samp.offset(i as isize)).left += data * leftvol >> 8;
            (*samp.offset(i as isize)).right += data * rightvol >> 8;
            samples = samples.offset(1);
            if !chunk.is_null()
                && samples
                    == ((*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                        .offset((1024i32 * 2) as isize)
            {
                chunk = (*chunk).next;
                samples = (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte
            }
            i += 1
        }
    } else {
        ooff = sampleOffset as f32;
        samples = (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
        i = 0;
        while i < count {
            data = crate::src::client::snd_wavelet::mulawToShort
                [*samples.offset(ooff as i32 as isize) as usize] as i32;
            ooff = ooff + (*ch).dopplerScale;
            (*samp.offset(i as isize)).left += data * leftvol >> 8;
            (*samp.offset(i as isize)).right += data * rightvol >> 8;
            if ooff >= (1024i32 * 2) as f32 {
                chunk = (*chunk).next;
                if chunk.is_null() {
                    chunk = (*sc).soundData
                }
                samples =
                    (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
                ooff = 0f32
            }
            i += 1
        }
    };
}
/*
===================
S_PaintChannels
===================
*/
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannels(mut endtime: i32) {
    let mut i: i32 = 0;
    let mut end: i32 = 0;
    let mut _stream: i32 = 0;
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut sc: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    let mut ltime: i32 = 0;
    let mut count: i32 = 0;
    let mut sampleOffset: i32 = 0;
    if (*crate::src::client::snd_main::s_muted).integer != 0 {
        snd_vol = 0
    } else {
        snd_vol = ((*crate::src::client::snd_main::s_volume).value * 255f32) as i32
    }
    //Com_Printf ("%i to %i\n", s_paintedtime, endtime);
    while crate::src::client::snd_dma::s_paintedtime < endtime {
        // if paintbuffer is smaller than DMA buffer
        // we may need to fill it multiple times
        end = endtime;
        if endtime - crate::src::client::snd_dma::s_paintedtime > 4096 {
            end = crate::src::client::snd_dma::s_paintedtime + 4096
        }
        // clear the paint buffer and mix any raw samples...
        crate::stdlib::memset(
            paintbuffer.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[crate::snd_local_h::portable_samplepair_t; 4096]>(),
        );

        for stream in 0..64 * 2 + 1 {
            if crate::src::client::snd_dma::s_rawend[stream as usize]
                >= crate::src::client::snd_dma::s_paintedtime
            {
                // copy from the streaming sound source
                let mut rawsamples: *const crate::snd_local_h::portable_samplepair_t =
                    crate::src::client::snd_dma::s_rawsamples[stream as usize].as_mut_ptr();
                let stop: i32 = if end < crate::src::client::snd_dma::s_rawend[stream as usize] {
                    end
                } else {
                    crate::src::client::snd_dma::s_rawend[stream as usize]
                };
                i = crate::src::client::snd_dma::s_paintedtime;
                while i < stop {
                    let s: i32 = i & 16384 - 1;
                    paintbuffer[(i - crate::src::client::snd_dma::s_paintedtime) as usize].left +=
                        (*rawsamples.offset(s as isize)).left;
                    paintbuffer[(i - crate::src::client::snd_dma::s_paintedtime) as usize].right +=
                        (*rawsamples.offset(s as isize)).right;
                    i += 1
                }
            }
        }
        // paint in the channels.
        ch = crate::src::client::snd_dma::s_channels.as_mut_ptr();
        i = 0;
        while i < 96 {
            if !((*ch).thesfx.is_null()
                || ((*ch).leftvol as f64) < 0.25 && ((*ch).rightvol as f64) < 0.25)
            {
                ltime = crate::src::client::snd_dma::s_paintedtime;
                sc = (*ch).thesfx;
                if !((*sc).soundData.is_null() || (*sc).soundLength == 0) {
                    sampleOffset = ltime - (*ch).startSample;
                    count = end - ltime;
                    if sampleOffset + count > (*sc).soundLength {
                        count = (*sc).soundLength - sampleOffset
                    }
                    if count > 0 {
                        if (*sc).soundCompressionMethod == 1 {
                            S_PaintChannelFromADPCM(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        } else if (*sc).soundCompressionMethod == 2 {
                            S_PaintChannelFromWavelet(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        } else if (*sc).soundCompressionMethod == 3 {
                            S_PaintChannelFromMuLaw(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        } else {
                            S_PaintChannelFrom16(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        }
                    }
                }
            }
            i += 1;
            ch = ch.offset(1)
        }
        // paint in the looped channels.
        ch = crate::src::client::snd_dma::loop_channels.as_mut_ptr();
        i = 0;
        while i < crate::src::client::snd_dma::numLoopChannels {
            if !((*ch).thesfx.is_null() || (*ch).leftvol == 0 && (*ch).rightvol == 0) {
                ltime = crate::src::client::snd_dma::s_paintedtime;
                sc = (*ch).thesfx;
                if !((*sc).soundData.is_null() || (*sc).soundLength == 0) {
                    loop
                    // we might have to make two passes if it
                    // is a looping sound effect and the end of
                    // the sample is hit
                    {
                        sampleOffset = ltime % (*sc).soundLength;
                        count = end - ltime;
                        if sampleOffset + count > (*sc).soundLength {
                            count = (*sc).soundLength - sampleOffset
                        }
                        if count > 0 {
                            if (*sc).soundCompressionMethod == 1 {
                                S_PaintChannelFromADPCM(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            } else if (*sc).soundCompressionMethod == 2 {
                                S_PaintChannelFromWavelet(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            } else if (*sc).soundCompressionMethod == 3 {
                                S_PaintChannelFromMuLaw(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            } else {
                                S_PaintChannelFrom16(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            }
                            ltime += count
                        }
                        if !(ltime < end) {
                            break;
                        }
                    }
                }
            }
            i += 1;
            ch = ch.offset(1)
        }
        // transfer out according to DMA format
        S_TransferPaintBuffer(end);
        crate::src::client::snd_dma::s_paintedtime = end
    }
}
