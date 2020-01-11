use ::libc;

pub mod celt_h {

    pub static mut tapset_icdf: [u8; 3] = [2, 1, 0];

    pub static mut trim_icdf: [u8; 11] = [126, 124, 119, 109, 87, 41, 19, 9, 4, 2, 0];

    pub static mut spread_icdf: [u8; 4] = [25, 23, 2, 0];

    /* CELT_H */
}

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    #[inline]

    pub unsafe extern "C" fn ec_get_error(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> i32 {
        return (*_this).error;
    }
    /*Returns the number of bits "used" by the encoded or decoded symbols so far.
    This same number can be computed in either the encoder or the decoder, and is
     suitable for making coding decisions.
    Return: The number of bits.
            This will always be slightly larger than the exact value (e.g., all
             rounding error is in the positive direction).*/
    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> i32 {
        return (*_this).nbits_total
            - (::std::mem::size_of::<u32>() as i32 * 8 - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod mathops_h {

    /* Copyright (c) 2002-2008 Jean-Marc Valin
    Copyright (c) 2007-2008 CSIRO
    Copyright (c) 2007-2009 Xiph.Org Foundation
    Written by Jean-Marc Valin */
    /* *
       @file mathops.h
       @brief Various math functions
    */
    /*
       Redistribution and use in source and binary forms, with or without
       modification, are permitted provided that the following conditions
       are met:

       - Redistributions of source code must retain the above copyright
       notice, this list of conditions and the following disclaimer.

       - Redistributions in binary form must reproduce the above copyright
       notice, this list of conditions and the following disclaimer in the
       documentation and/or other materials provided with the distribution.

       THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
       ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
       LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
       A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
       OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
       EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
       PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
       PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
       LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
       NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
       SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
    */
    /* Multiplies two 16-bit fractional values. Bit-exactness of this macro is important */
    /* CELT doesn't need it for fixed-point, by analysis.c does. */
    /* For very small values, we don't care about the answer, so
    we can just return 0. */
    #[inline]

    pub unsafe extern "C" fn celt_maxabs16(
        mut x: *const crate::arch_h::opus_val16,
        mut len: i32,
    ) -> crate::arch_h::opus_val32 {
        let mut _i: i32 = 0;
        let mut maxval: crate::arch_h::opus_val16 = 0f32;
        let mut minval: crate::arch_h::opus_val16 = 0f32;

        for i in 0..len {
            maxval = if maxval > *x.offset(i as isize) {
                maxval
            } else {
                *x.offset(i as isize)
            };

            minval = if minval < *x.offset(i as isize) {
                minval
            } else {
                *x.offset(i as isize)
            };
        }
        return if maxval > -minval { maxval } else { -minval };
    }
    /* Note: This assumes radix-2 floating point with the exponent at bits 23..30 and an offset of 127
    denorm, +/- inf and NaN are *not* handled */
    /* * Base-2 log approximation (log2(x)). */
    #[inline]

    pub unsafe extern "C" fn celt_log2(mut x: f32) -> f32 {
        let mut integer: i32 = 0;
        let mut frac: f32 = 0.;
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        integer = (in_0.i >> 23).wrapping_sub(127u32) as i32;
        in_0.i = (in_0.i).wrapping_sub((integer << 23) as u32);
        frac = in_0.f - 1.5;
        frac = -0.41445418 + frac * (0.95909232 + frac * (-0.33951290 + frac * 0.16541097));
        return (1 + integer) as f32 + frac;
    }

    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub mod pitch_h {
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: i32,
    ) -> crate::arch_h::opus_val32 {
        let mut _i: i32 = 0;
        let mut xy: crate::arch_h::opus_val32 = 0f32;

        for i in 0..N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
        }
        return xy;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdarg_h::va_list;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::celt_norm;
pub use crate::arch_h::celt_sig;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::celt_h::AnalysisInfo;
pub use crate::celt_h::SILKInfo;
pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::celt::celt::comb_filter;
pub use crate::src::opus_1_2_1::celt::celt::init_caps;
pub use crate::src::opus_1_2_1::celt::celt::resampling_factor;
pub use crate::src::opus_1_2_1::celt::celt::tf_select_table;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_h::spread_icdf;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_h::tapset_icdf;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_h::trim_icdf;
pub use crate::src::opus_1_2_1::celt::celt_encoder::entcode_h::ec_get_error;
pub use crate::src::opus_1_2_1::celt::celt_encoder::entcode_h::ec_tell;
pub use crate::src::opus_1_2_1::celt::celt_encoder::mathops_h::celt_log2;
pub use crate::src::opus_1_2_1::celt::celt_encoder::mathops_h::celt_maxabs16;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_tell_frac;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::clt_mdct_forward_c;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;

pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::src::opus_1_2_1::celt::celt_encoder::pitch_h::celt_inner_prod_c;

pub use crate::src::opus_1_2_1::celt::pitch::pitch_downsample;
pub use crate::src::opus_1_2_1::celt::pitch::pitch_search;
pub use crate::src::opus_1_2_1::celt::pitch::remove_doubling;

/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2010 Xiph.Org Foundation
Copyright (c) 2008 Gregory Maxwell
Written by Jean-Marc Valin and Gregory Maxwell */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* * Encoder state
@brief Encoder state
*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusCustomEncoder {
    pub mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    pub channels: i32,
    pub stream_channels: i32,
    pub force_intra: i32,
    pub clip: i32,
    pub disable_pf: i32,
    pub complexity: i32,
    pub upsample: i32,
    pub start: i32,
    pub end: i32,
    pub bitrate: crate::opus_types_h::opus_int32,
    pub vbr: i32,
    pub signalling: i32,
    pub constrained_vbr: i32,
    pub loss_rate: i32,
    pub lsb_depth: i32,
    pub lfe: i32,
    pub disable_inv: i32,
    pub arch: i32,
    pub rng: crate::opus_types_h::opus_uint32,
    pub spread_decision: i32,
    pub delayedIntra: crate::arch_h::opus_val32,
    pub tonal_average: i32,
    pub lastCodedBands: i32,
    pub hf_average: i32,
    pub tapset_decision: i32,
    pub prefilter_period: i32,
    pub prefilter_gain: crate::arch_h::opus_val16,
    pub prefilter_tapset: i32,
    pub consec_transient: i32,
    pub analysis: crate::celt_h::AnalysisInfo,
    pub silk_info: crate::celt_h::SILKInfo,
    pub preemph_memE: [crate::arch_h::opus_val32; 2],
    pub preemph_memD: [crate::arch_h::opus_val32; 2],
    pub vbr_reservoir: crate::opus_types_h::opus_int32,
    pub vbr_drift: crate::opus_types_h::opus_int32,
    pub vbr_offset: crate::opus_types_h::opus_int32,
    pub vbr_count: crate::opus_types_h::opus_int32,
    pub overlap_max: crate::arch_h::opus_val32,
    pub stereo_saving: crate::arch_h::opus_val16,
    pub intensity: i32,
    pub energy_mask: *mut crate::arch_h::opus_val16,
    pub spec_avg: crate::arch_h::opus_val16,
    pub in_mem: [crate::arch_h::celt_sig; 1],
}
/* Size = channels*mode->overlap */
/* celt_sig prefilter_mem[],  Size = channels*COMBFILTER_MAXPERIOD */
/* opus_val16 oldBandE[],     Size = channels*mode->nbEBands */
/* opus_val16 oldLogE[],      Size = channels*mode->nbEBands */
/* opus_val16 oldLogE2[],     Size = channels*mode->nbEBands */
/* opus_val16 energyError[],  Size = channels*mode->nbEBands */
#[no_mangle]

pub unsafe extern "C" fn celt_encoder_get_size(mut channels: i32) -> i32 {
    let mut mode: *mut crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        crate::src::opus_1_2_1::celt::modes::opus_custom_mode_create(48000, 960, 0 as *mut i32); /* opus_val16 oldBandE[channels*mode->nbEBands]; */
    /* opus_val16 oldLogE[channels*mode->nbEBands]; */
    /* opus_val16 oldLogE2[channels*mode->nbEBands]; */
    /* opus_val16 energyError[channels*mode->nbEBands]; */
    return opus_custom_encoder_get_size(mode, channels);
}
#[inline]

unsafe extern "C" fn opus_custom_encoder_get_size(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut channels: i32,
) -> i32 {
    let mut size: i32 = (::std::mem::size_of::<OpusCustomEncoder>())
        .wrapping_add(
            ((channels * (*mode).overlap - 1i32) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>()),
        )
        .wrapping_add(
            ((channels * 1024i32) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>()),
        )
        .wrapping_add(
            ((4i32 * channels * (*mode).nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
        ) as i32;
    return size;
}
/* CUSTOM_MODES */

unsafe extern "C" fn opus_custom_encoder_init_arch(
    mut st: *mut OpusCustomEncoder,
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut channels: i32,
    mut arch: i32,
) -> i32 {
    if channels < 0 || channels > 2 {
        return -(1i32);
    }
    if st.is_null() || mode.is_null() {
        return -(7i32);
    }
    crate::stdlib::memset(
        st as *mut libc::c_void,
        0,
        (opus_custom_encoder_get_size(mode, channels) as usize)
            .wrapping_mul(::std::mem::size_of::<i8>()),
    );
    (*st).mode = mode;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).upsample = 1;
    (*st).start = 0;
    (*st).end = (*(*st).mode).effEBands;
    (*st).signalling = 1;
    (*st).arch = arch;
    (*st).constrained_vbr = 1;
    (*st).clip = 1;
    (*st).bitrate = -(1);
    (*st).vbr = 0;
    (*st).force_intra = 0;
    (*st).complexity = 5;
    (*st).lsb_depth = 24;
    opus_custom_encoder_ctl(st, 4028);
    return 0;
}
/* Encoder stuff */
#[no_mangle]

pub unsafe extern "C" fn celt_encoder_init(
    mut st: *mut OpusCustomEncoder,
    mut sampling_rate: crate::opus_types_h::opus_int32,
    mut channels: i32,
    mut arch: i32,
) -> i32 {
    let mut ret: i32 = 0;
    ret = opus_custom_encoder_init_arch(
        st,
        crate::src::opus_1_2_1::celt::modes::opus_custom_mode_create(48000, 960, 0 as *mut i32),
        channels,
        arch,
    );
    if ret != 0 {
        return ret;
    }
    (*st).upsample = crate::src::opus_1_2_1::celt::celt::resampling_factor(sampling_rate);
    return 0;
}
/* CUSTOM_MODES */

unsafe extern "C" fn transient_analysis(
    mut in_0: *const crate::arch_h::opus_val32,
    mut len: i32,
    mut C: i32,
    mut tf_estimate: *mut crate::arch_h::opus_val16,
    mut tf_chan: *mut i32,
    mut allow_weak_transients: i32,
    mut weak_transient: *mut i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut tmp: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut mem0: crate::arch_h::opus_val32 = 0.;
    let mut mem1: crate::arch_h::opus_val32 = 0.;
    let mut is_transient: i32 = 0;
    let mut mask_metric: crate::opus_types_h::opus_int32 = 0;
    let mut _c: i32 = 0;
    let mut tf_max: crate::arch_h::opus_val16 = 0.;
    let mut len2: i32 = 0;
    /* Forward masking: 6.7 dB/ms. */
    let mut forward_decay: crate::arch_h::opus_val16 = 0.0625;
    /* Table of 6*64/x, trained on real data to minimize the average error */
    static mut inv_table: [u8; 128] = [
        255, 255, 156, 110, 86, 70, 59, 51, 45, 40, 37, 33, 31, 28, 26, 25, 23, 22, 21, 20, 19, 18,
        17, 16, 16, 15, 15, 14, 13, 13, 12, 12, 12, 12, 11, 11, 11, 10, 10, 10, 9, 9, 9, 9, 9, 9,
        8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2,
    ];
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul(len as usize),
    );
    tmp = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    *weak_transient = 0;
    /* For lower bitrates, let's be more conservative and have a forward masking
    decay of 3.3 dB/ms. This avoids having to code transients at very low
    bitrate (mostly for hybrid), which can result in unstable energy and/or
    partial collapse. */
    if allow_weak_transients != 0 {
        forward_decay = 0.03125
    }
    len2 = len / 2;

    for c in 0..C {
        let mut mean: crate::arch_h::opus_val32 = 0.;

        let mut unmask: crate::opus_types_h::opus_int32 = 0;

        let mut norm: crate::arch_h::opus_val32 = 0.;

        let mut maxE: crate::arch_h::opus_val16 = 0.;

        mem0 = 0f32;

        mem1 = 0f32;

        i = 0;

        while i < len {
            let mut x: crate::arch_h::opus_val32 = 0.;
            let mut y: crate::arch_h::opus_val32 = 0.;
            x = *in_0.offset((i + c * len) as isize);
            y = mem0 + x;
            mem0 = mem1 + y - 2f32 * x;
            mem1 = x - 0.5 * y;
            *tmp.offset(i as isize) = y;
            i += 1
            /*printf("%f ", tmp[i]);*/
        }

        crate::stdlib::memset(
            tmp as *mut libc::c_void,
            0,
            (12usize).wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
        );

        mean = 0f32;

        mem0 = 0f32;

        i = 0;

        while i < len2 {
            let mut x2: crate::arch_h::opus_val16 = *tmp.offset((2 * i) as isize)
                * *tmp.offset((2 * i) as isize)
                + *tmp.offset((2 * i + 1) as isize) * *tmp.offset((2 * i + 1) as isize);
            mean += x2;
            *tmp.offset(i as isize) = mem0 + forward_decay * (x2 - mem0);
            mem0 = *tmp.offset(i as isize);
            i += 1
        }

        mem0 = 0f32;

        maxE = 0f32;

        i = len2 - 1;

        while i >= 0 {
            /* Backward masking: 13.9 dB/ms. */
            *tmp.offset(i as isize) = mem0 + 0.125 * (*tmp.offset(i as isize) - mem0);
            mem0 = *tmp.offset(i as isize);
            maxE = if maxE > mem0 { maxE } else { mem0 };
            i -= 1
        }

        mean = crate::stdlib::sqrt((mean * maxE) as f64 * 0.5 * len2 as f64) as f32;

        norm = len2 as f32 / (1e-15 + mean);

        unmask = 0;

        i = 12;

        while i < len2 - 5 {
            let mut id: i32 = 0;
            id = if 0f64
                > (if (127f64)
                    < crate::stdlib::floor(
                        (64f32 * norm * (*tmp.offset(i as isize) + 1e-15)) as f64,
                    )
                {
                    127f64
                } else {
                    crate::stdlib::floor((64f32 * norm * (*tmp.offset(i as isize) + 1e-15)) as f64)
                }) {
                0f64
            } else if (127f64)
                < crate::stdlib::floor((64f32 * norm * (*tmp.offset(i as isize) + 1e-15)) as f64)
            {
                127f64
            } else {
                crate::stdlib::floor((64f32 * norm * (*tmp.offset(i as isize) + 1e-15)) as f64)
            } as i32;
            unmask += inv_table[id as usize] as i32;
            i += 4
        }

        unmask = 64 * unmask * 4 / (6 * (len2 - 17));

        if unmask > mask_metric {
            *tf_chan = c;
            mask_metric = unmask
        }
    }
    is_transient = (mask_metric > 200) as i32;
    /* For low bitrates, define "weak transients" that need to be
    handled differently to avoid partial collapse. */
    if allow_weak_transients != 0 && is_transient != 0 && mask_metric < 600 {
        is_transient = 0;
        *weak_transient = 1
    }
    /* Arbitrary metric for VBR boost */
    tf_max = if 0f32 > crate::stdlib::sqrt((27 * mask_metric) as f64) as f32 - 42f32 {
        0f32
    } else {
        (crate::stdlib::sqrt((27 * mask_metric) as f64) as f32) - 42f32
    };
    /* *tf_estimate = 1 + MIN16(1, sqrt(MAX16(0, tf_max-30))/20); */
    *tf_estimate = crate::stdlib::sqrt(
        if 0f64 > (0.0069 * (if (163f32) < tf_max { 163f32 } else { tf_max })) as f64 - 0.139 {
            0f64
        } else {
            ((0.0069 * (if (163f32) < tf_max { 163f32 } else { tf_max })) as f64) - 0.139
        },
    ) as f32;
    /*printf("%d %f\n", tf_max, mask_metric);*/
    /*printf("%d %f %d\n", is_transient, (float)*tf_estimate, tf_max);*/
    return is_transient;
}
/* Looks for sudden increases of energy to decide whether we need to patch
the transient decision */

unsafe extern "C" fn patch_transient_decision(
    mut newE: *mut crate::arch_h::opus_val16,
    mut oldE: *mut crate::arch_h::opus_val16,
    mut nbEBands: i32,
    mut start: i32,
    mut end: i32,
    mut C: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut mean_diff: crate::arch_h::opus_val32 = 0f32;
    let mut spread_old: [crate::arch_h::opus_val16; 26] = [0.; 26];
    /* Apply an aggressive (-6 dB/Bark) spreading function to the old frame to
    avoid false detection caused by irrelevant bands */
    if C == 1 {
        spread_old[start as usize] = *oldE.offset(start as isize);
        i = start + 1;
        while i < end {
            spread_old[i as usize] =
                if spread_old[(i - 1) as usize] - 1.0 > *oldE.offset(i as isize) {
                    (spread_old[(i - 1) as usize]) - 1.0
                } else {
                    *oldE.offset(i as isize)
                };
            i += 1
        }
    } else {
        spread_old[start as usize] =
            if *oldE.offset(start as isize) > *oldE.offset((start + nbEBands) as isize) {
                *oldE.offset(start as isize)
            } else {
                *oldE.offset((start + nbEBands) as isize)
            };
        i = start + 1;
        while i < end {
            spread_old[i as usize] = if spread_old[(i - 1) as usize] - 1.0
                > (if *oldE.offset(i as isize) > *oldE.offset((i + nbEBands) as isize) {
                    *oldE.offset(i as isize)
                } else {
                    *oldE.offset((i + nbEBands) as isize)
                }) {
                (spread_old[(i - 1) as usize]) - 1.0
            } else if *oldE.offset(i as isize) > *oldE.offset((i + nbEBands) as isize) {
                *oldE.offset(i as isize)
            } else {
                *oldE.offset((i + nbEBands) as isize)
            };
            i += 1
        }
    }
    i = end - 2;
    while i >= start {
        spread_old[i as usize] = if spread_old[i as usize] > spread_old[(i + 1) as usize] - 1.0 {
            spread_old[i as usize]
        } else {
            (spread_old[(i + 1) as usize]) - 1.0
        };
        i -= 1
    }
    /* Compute mean increase */
    c = 0;
    loop {
        i = if 2 > start { 2 } else { start };
        while i < end - 1 {
            let mut x1: crate::arch_h::opus_val16 = 0.;
            let mut x2: crate::arch_h::opus_val16 = 0.;
            x1 = if 0f32 > *newE.offset((i + c * nbEBands) as isize) {
                0f32
            } else {
                *newE.offset((i + c * nbEBands) as isize)
            };
            x2 = if 0f32 > spread_old[i as usize] {
                0f32
            } else {
                spread_old[i as usize]
            };
            mean_diff = mean_diff + (if 0f32 > x1 - x2 { 0f32 } else { (x1) - x2 });
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    mean_diff = mean_diff
        / (C * (end - 1i32 - (if 2 > start { 2 } else { start }))) as crate::arch_h::opus_val32;
    /*printf("%f %f %d\n", mean_diff, max_diff, count);*/
    return (mean_diff > 1.0) as i32;
}
/* * Apply window and compute the MDCT for all sub-frames and
all channels in a frame */

unsafe extern "C" fn compute_mdcts(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut shortBlocks: i32,
    mut in_0: *mut crate::arch_h::celt_sig,
    mut out: *mut crate::arch_h::celt_sig,
    mut C: i32,
    mut CC: i32,
    mut LM: i32,
    mut upsample: i32,
    mut arch: i32,
) {
    let overlap: i32 = (*mode).overlap;
    let mut N: i32 = 0;
    let mut B: i32 = 0;
    let mut shift: i32 = 0;
    let mut i: i32 = 0;
    let mut _b: i32 = 0;
    let mut c: i32 = 0;
    if shortBlocks != 0 {
        B = shortBlocks;
        N = (*mode).shortMdctSize;
        shift = (*mode).maxLM
    } else {
        B = 1;
        N = (*mode).shortMdctSize << LM;
        shift = (*mode).maxLM - LM
    }
    c = 0;
    loop {
        for b in 0..B {
            crate::src::opus_1_2_1::celt::mdct::clt_mdct_forward_c(
                &(*mode).mdct,
                in_0.offset((c * (B * N + overlap)) as isize)
                    .offset((b * N) as isize),
                &mut *out.offset((b + c * N * B) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if CC == 2 && C == 1 {
        i = 0;
        while i < B * N {
            *out.offset(i as isize) =
                0.5f32 * *out.offset(i as isize) + 0.5 * *out.offset((B * N + i) as isize);
            i += 1
        }
    }
    if upsample != 1 {
        c = 0;
        loop {
            let mut bound: i32 = B * N / upsample;
            i = 0;
            while i < bound {
                let ref mut fresh1 = *out.offset((c * B * N + i) as isize);
                *fresh1 *= upsample as f32;
                i += 1
            }
            crate::stdlib::memset(
                &mut *out.offset((c * B * N + bound) as isize) as *mut crate::arch_h::celt_sig
                    as *mut libc::c_void,
                0,
                ((B * N - bound) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>()),
            );
            c += 1;
            if !(c < C) {
                break;
            }
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn celt_preemphasis(
    mut pcmp: *const crate::arch_h::opus_val16,
    mut inp: *mut crate::arch_h::celt_sig,
    mut N: i32,
    mut CC: i32,
    mut upsample: i32,
    mut coef: *const crate::arch_h::opus_val16,
    mut mem: *mut crate::arch_h::celt_sig,
    mut clip: i32,
) {
    let mut i: i32 = 0;
    let mut coef0: crate::arch_h::opus_val16 = 0.;
    let mut m: crate::arch_h::celt_sig = 0.;
    let mut Nu: i32 = 0;
    coef0 = *coef.offset(0);
    m = *mem;
    /* Fast path for the normal 48kHz case and no clipping */
    if *coef.offset(1) == 0f32 && upsample == 1 && clip == 0 {
        i = 0;
        while i < N {
            let mut x: crate::arch_h::opus_val16 = 0.;
            x = *pcmp.offset((CC * i) as isize) * 32768.0;
            /* Apply pre-emphasis */
            *inp.offset(i as isize) = x - m;
            m = coef0 * x;
            i += 1
        }
        *mem = m;
        return;
    }
    Nu = N / upsample;
    if upsample != 1 {
        crate::stdlib::memset(
            inp as *mut libc::c_void,
            0i32,
            (N as usize).wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>()),
        );
    }
    i = 0;
    while i < Nu {
        *inp.offset((i * upsample) as isize) = *pcmp.offset((CC * i) as isize) * 32768.0f32;
        i += 1
    }
    if clip != 0 {
        /* Clip input to avoid encoding non-portable files */
        i = 0;
        while i < Nu {
            *inp.offset((i * upsample) as isize) = if -65536.0f32
                > (if 65536.0f32 < *inp.offset((i * upsample) as isize) {
                    65536.0
                } else {
                    *inp.offset((i * upsample) as isize)
                }) {
                -65536.0f32
            } else if 65536.0f32 < *inp.offset((i * upsample) as isize) {
                65536.0
            } else {
                *inp.offset((i * upsample) as isize)
            };
            i += 1
        }
    }
    i = 0;
    while i < N {
        let mut x_0: crate::arch_h::opus_val16 = 0.;
        x_0 = *inp.offset(i as isize);
        /* Apply pre-emphasis */
        *inp.offset(i as isize) = x_0 - m;
        m = coef0 * x_0;
        i += 1
    }
    *mem = m;
}

unsafe extern "C" fn l1_metric(
    mut tmp: *const crate::arch_h::celt_norm,
    mut N: i32,
    mut LM: i32,
    mut bias: crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val32 {
    let mut _i: i32 = 0;
    let mut L1: crate::arch_h::opus_val32 = 0.;
    L1 = 0f32;

    for i in 0..N {
        L1 += crate::stdlib::fabs(*tmp.offset(i as isize) as f64) as f32;
    }
    /* When in doubt, prefer good freq resolution */
    L1 = L1 + LM as f32 * bias * L1;
    return L1;
}

unsafe extern "C" fn tf_analysis(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut len: i32,
    mut isTransient: i32,
    mut tf_res: *mut i32,
    mut lambda: i32,
    mut X: *mut crate::arch_h::celt_norm,
    mut N0: i32,
    mut LM: i32,
    mut tf_estimate: crate::arch_h::opus_val16,
    mut tf_chan: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut metric: *mut i32 = 0 as *mut i32;
    let mut cost0: i32 = 0;
    let mut cost1: i32 = 0;
    let mut path0: *mut i32 = 0 as *mut i32;
    let mut path1: *mut i32 = 0 as *mut i32;
    let mut tmp: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut tmp_1: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut _sel: i32 = 0;
    let mut selcost: [i32; 2] = [0; 2];
    let mut tf_select: i32 = 0;
    let mut bias: crate::arch_h::opus_val16 = 0.;
    bias = 0.04
        * (if -0.25f32 > 0.5 - tf_estimate {
            -0.25
        } else {
            (0.5) - tf_estimate
        });
    /*printf("%f ", bias);*/
    let mut fresh2 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    metric = fresh2.as_mut_ptr();
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>()).wrapping_mul(
            ((*(*m).eBands.offset(len as isize) as i32
                - *(*m).eBands.offset((len - 1i32) as isize) as i32)
                << LM) as usize,
        ),
    );
    tmp = fresh3.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>()).wrapping_mul(
            ((*(*m).eBands.offset(len as isize) as i32
                - *(*m).eBands.offset((len - 1i32) as isize) as i32)
                << LM) as usize,
        ),
    );
    tmp_1 = fresh4.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh5 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    path0 = fresh5.as_mut_ptr();
    let mut fresh6 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    path1 = fresh6.as_mut_ptr();
    i = 0;
    while i < len {
        let mut _k: i32 = 0;
        let mut N: i32 = 0;
        let mut narrow: i32 = 0;
        let mut L1: crate::arch_h::opus_val32 = 0.;
        let mut best_L1: crate::arch_h::opus_val32 = 0.;
        let mut best_level: i32 = 0;
        N = (*(*m).eBands.offset((i + 1) as isize) as i32 - *(*m).eBands.offset(i as isize) as i32)
            << LM;
        /*printf("%d ", metric[i]);*/
        narrow = (*(*m).eBands.offset((i + 1) as isize) as i32
            - *(*m).eBands.offset(i as isize) as i32
            == 1) as i32;
        crate::stdlib::memcpy(
            tmp as *mut libc::c_void,
            &mut *X
                .offset((tf_chan * N0 + ((*(*m).eBands.offset(i as isize) as i32) << LM)) as isize)
                as *mut crate::arch_h::celt_norm as *const libc::c_void,
            (N as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>())
                .wrapping_add(
                    (0 * tmp.wrapping_offset_from(&mut *X.offset(
                        (tf_chan * N0 + ((*(*m).eBands.offset(i as isize) as i32) << LM)) as isize,
                    ))) as usize,
                ),
        );
        L1 = l1_metric(tmp, N, if isTransient != 0 { LM } else { 0 }, bias);
        best_L1 = L1;
        if isTransient != 0 && narrow == 0 {
            crate::stdlib::memcpy(
                tmp_1 as *mut libc::c_void,
                tmp as *const libc::c_void,
                (N as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>())
                    .wrapping_add((0 * tmp_1.wrapping_offset_from(tmp)) as usize),
            );
            crate::src::opus_1_2_1::celt::bands::haar1(tmp_1, N >> LM, (1) << LM);
            L1 = l1_metric(tmp_1, N, LM + 1, bias);
            if L1 < best_L1 {
                best_L1 = L1;
                best_level = -(1)
            }
        }

        for k in 0..LM + !(isTransient != 0 || narrow != 0) as i32 {
            let mut B: i32 = 0;

            if isTransient != 0 {
                B = LM - k - 1
            } else {
                B = k + 1
            }

            crate::src::opus_1_2_1::celt::bands::haar1(tmp, N >> k, (1) << k);

            L1 = l1_metric(tmp, N, B, bias);

            if L1 < best_L1 {
                best_L1 = L1;
                best_level = k + 1
            }
        }
        if isTransient != 0 {
            *metric.offset(i as isize) = 2 * best_level
        } else {
            *metric.offset(i as isize) = -(2) * best_level
        }
        if narrow != 0
            && (*metric.offset(i as isize) == 0 || *metric.offset(i as isize) == -(2) * LM)
        {
            *metric.offset(i as isize) -= 1
        }
        i += 1
    }
    /* band is too narrow to be split down to LM=-1 */
    /* Just add the right channel if we're in stereo */
    /*if (C==2)
    for (j=0;j<N;j++)
       tmp[j] = ADD16(SHR16(tmp[j], 1),SHR16(X[N0+j+(m->eBands[i]<<LM)], 1));*/
    /* Check the -1 case for transients */
    /*printf ("%f ", L1);*/
    /*printf ("%d ", isTransient ? LM-best_level : best_level);*/
    /* metric is in Q1 to be able to select the mid-point (-0.5) for narrower bands */
    /* For bands that can't be split to -1, set the metric to the half-way point to avoid
    biasing the decision */
    /*printf("\n");*/
    /* Search for the optimal tf resolution, including tf_select */
    tf_select = 0;

    for sel in 0..2 {
        cost0 = 0;

        cost1 = if isTransient != 0 { 0 } else { lambda };

        i = 1;

        while i < len {
            let mut curr0: i32 = 0;
            let mut curr1: i32 = 0;
            curr0 = if cost0 < cost1 + lambda {
                cost0
            } else {
                (cost1) + lambda
            };
            curr1 = if cost0 + lambda < cost1 {
                (cost0) + lambda
            } else {
                cost1
            };
            cost0 = curr0
                + crate::stdlib::abs(
                    *metric.offset(i as isize)
                        - 2 * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                            [(4 * isTransient + 2 * sel + 0) as usize]
                            as i32,
                );
            cost1 = curr1
                + crate::stdlib::abs(
                    *metric.offset(i as isize)
                        - 2 * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                            [(4 * isTransient + 2 * sel + 1) as usize]
                            as i32,
                );
            i += 1
        }

        cost0 = if cost0 < cost1 { cost0 } else { cost1 };

        selcost[sel as usize] = cost0;
    }
    /* For now, we're conservative and only allow tf_select=1 for transients.
     * If tests confirm it's useful for non-transients, we could allow it. */
    if selcost[1] < selcost[0] && isTransient != 0 {
        tf_select = 1
    }
    cost0 = 0;
    cost1 = if isTransient != 0 { 0 } else { lambda };
    /* Viterbi forward pass */
    i = 1;
    while i < len {
        let mut curr0_0: i32 = 0;
        let mut curr1_0: i32 = 0;
        let mut from0: i32 = 0;
        let mut from1: i32 = 0;
        from0 = cost0;
        from1 = cost1 + lambda;
        if from0 < from1 {
            curr0_0 = from0;
            *path0.offset(i as isize) = 0
        } else {
            curr0_0 = from1;
            *path0.offset(i as isize) = 1
        }
        from0 = cost0 + lambda;
        from1 = cost1;
        if from0 < from1 {
            curr1_0 = from0;
            *path1.offset(i as isize) = 0
        } else {
            curr1_0 = from1;
            *path1.offset(i as isize) = 1
        }
        cost0 = curr0_0
            + crate::stdlib::abs(
                *metric.offset(i as isize)
                    - 2 * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                        [(4 * isTransient + 2 * tf_select + 0) as usize]
                        as i32,
            );
        cost1 = curr1_0
            + crate::stdlib::abs(
                *metric.offset(i as isize)
                    - 2 * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                        [(4 * isTransient + 2 * tf_select + 1) as usize]
                        as i32,
            );
        i += 1
    }
    *tf_res.offset((len - 1i32) as isize) = if cost0 < cost1 { 0 } else { 1 };
    /* Viterbi backward pass to check the decisions */
    i = len - 2;
    while i >= 0 {
        if *tf_res.offset((i + 1) as isize) == 1 {
            *tf_res.offset(i as isize) = *path1.offset((i + 1) as isize)
        } else {
            *tf_res.offset(i as isize) = *path0.offset((i + 1) as isize)
        }
        i -= 1
    }
    /*printf("%d %f\n", *tf_sum, tf_estimate);*/
    return tf_select;
}

unsafe extern "C" fn tf_encode(
    mut start: i32,
    mut end: i32,
    mut isTransient: i32,
    mut tf_res: *mut i32,
    mut LM: i32,
    mut tf_select: i32,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) {
    let mut curr: i32 = 0;
    let mut i: i32 = 0;
    let mut tf_select_rsv: i32 = 0;
    let mut tf_changed: i32 = 0;
    let mut logp: i32 = 0;
    let mut budget: crate::opus_types_h::opus_uint32 = 0;
    let mut tell: crate::opus_types_h::opus_uint32 = 0;
    budget = (*enc).storage.wrapping_mul(8u32);
    tell = ec_tell(enc) as crate::opus_types_h::opus_uint32;
    logp = if isTransient != 0 { 2 } else { 4 };
    /* Reserve space to code the tf_select decision. */
    tf_select_rsv = (LM > 0 && tell.wrapping_add(logp as u32).wrapping_add(1u32) <= budget) as i32;
    budget = (budget).wrapping_sub(tf_select_rsv as u32);
    tf_changed = 0;
    curr = tf_changed;
    i = start;
    while i < end {
        if tell.wrapping_add(logp as u32) <= budget {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                enc,
                *tf_res.offset(i as isize) ^ curr,
                logp as u32,
            );
            tell = ec_tell(enc) as crate::opus_types_h::opus_uint32;
            curr = *tf_res.offset(i as isize);
            tf_changed |= curr
        } else {
            *tf_res.offset(i as isize) = curr
        }
        logp = if isTransient != 0 { 4 } else { 5 };
        i += 1
    }
    /* Only code tf_select if it would actually make a difference. */
    if tf_select_rsv != 0
        && crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
            [(4 * isTransient + 0 + tf_changed) as usize] as i32
            != crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                [(4 * isTransient + 2 + tf_changed) as usize] as i32
    {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, tf_select, 1u32);
    } else {
        tf_select = 0
    }
    i = start;
    while i < end {
        *tf_res.offset(i as isize) = crate::src::opus_1_2_1::celt::celt::tf_select_table
            [LM as usize]
            [(4i32 * isTransient + 2 * tf_select + *tf_res.offset(i as isize)) as usize]
            as i32;
        i += 1
    }
    /*for(i=0;i<end;i++)printf("%d ", isTransient ? tf_res[i] : LM+tf_res[i]);printf("\n");*/
}

unsafe extern "C" fn alloc_trim_analysis(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_norm,
    mut bandLogE: *const crate::arch_h::opus_val16,
    mut end: i32,
    mut LM: i32,
    mut C: i32,
    mut N0: i32,
    mut analysis: *mut crate::celt_h::AnalysisInfo,
    mut stereo_saving: *mut crate::arch_h::opus_val16,
    mut tf_estimate: crate::arch_h::opus_val16,
    mut intensity: i32,
    mut surround_trim: crate::arch_h::opus_val16,
    mut equiv_rate: crate::opus_types_h::opus_int32,
    mut _arch: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut diff: crate::arch_h::opus_val32 = 0f32;
    let mut c: i32 = 0;
    let mut trim_index: i32 = 0;
    let mut trim: crate::arch_h::opus_val16 = 5.0;
    let mut logXC: crate::arch_h::opus_val16 = 0.;
    let mut logXC2: crate::arch_h::opus_val16 = 0.;
    /* At low bitrate, reducing the trim seems to help. At higher bitrates, it's less
    clear what's best, so we're keeping it as it was before, at least for now. */
    if equiv_rate < 64000 {
        trim = 4.0
    } else if equiv_rate < 80000 {
        let mut frac: crate::opus_types_h::opus_int32 = equiv_rate - 64000 >> 10; /* Q10 */
        trim = 4.0 + 1.0 / 16.0 * frac as f32
    } /* Q10 */
    if C == 2 {
        let mut sum: crate::arch_h::opus_val16 = 0f32;
        let mut minXC: crate::arch_h::opus_val16 = 0.;
        /* Compute inter-channel correlation for low frequencies */
        i = 0;
        while i < 8 {
            let mut partial: crate::arch_h::opus_val32 = 0.;
            partial = celt_inner_prod_c(
                &*X.offset(((*(*m).eBands.offset(i as isize) as i32) << LM) as isize),
                &*X.offset((N0 + ((*(*m).eBands.offset(i as isize) as i32) << LM)) as isize),
                (*(*m).eBands.offset((i + 1) as isize) as i32
                    - *(*m).eBands.offset(i as isize) as i32)
                    << LM,
            );
            sum = sum + partial;
            i += 1
        }
        sum = 1.0 / 8f32 * sum;
        sum = if 1.0 < crate::stdlib::fabs(sum as f64) as f32 {
            1.0
        } else {
            crate::stdlib::fabs(sum as f64) as f32
        };
        minXC = sum;
        i = 8;
        while i < intensity {
            let mut partial_0: crate::arch_h::opus_val32 = 0.;
            partial_0 = celt_inner_prod_c(
                &*X.offset(((*(*m).eBands.offset(i as isize) as i32) << LM) as isize),
                &*X.offset((N0 + ((*(*m).eBands.offset(i as isize) as i32) << LM)) as isize),
                (*(*m).eBands.offset((i + 1) as isize) as i32
                    - *(*m).eBands.offset(i as isize) as i32)
                    << LM,
            );
            minXC = if minXC < crate::stdlib::fabs(partial_0 as f64) as f32 {
                minXC
            } else {
                crate::stdlib::fabs(partial_0 as f64) as f32
            };
            i += 1
        }
        minXC = if 1.0 < crate::stdlib::fabs(minXC as f64) as f32 {
            1.0
        } else {
            crate::stdlib::fabs(minXC as f64) as f32
        };
        /*printf ("%f\n", sum);*/
        /* mid-side savings estimations based on the LF average*/
        logXC = celt_log2(1.001 - sum * sum);
        /* mid-side savings estimations based on min correlation */
        logXC2 = if 0.5 * logXC > celt_log2(1.001 - minXC * minXC) {
            (0.5) * logXC
        } else {
            celt_log2(1.001 - minXC * minXC)
        };
        trim += if -4.0 > 0.75 * logXC {
            -4.0
        } else {
            (0.75) * logXC
        };
        *stereo_saving = if *stereo_saving + 0.25 < -(0.5 * logXC2) {
            (*stereo_saving) + 0.25
        } else {
            -(0.5 * logXC2)
        }
    }
    /* Estimate spectral tilt */
    c = 0;
    loop {
        i = 0;
        while i < end - 1 {
            diff += *bandLogE.offset((i + c * (*m).nbEBands) as isize) * (2 + 2 * i - end) as f32;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    diff /= (C * (end - 1i32)) as f32;
    /*printf("%f\n", diff);*/
    trim -= if -2.0
        > (if 2.0 < (diff + 1.0) / 6f32 {
            2.0
        } else {
            (diff + 1.0) / 6f32
        }) {
        -2.0
    } else if 2.0 < (diff + 1.0) / 6f32 {
        2.0
    } else {
        (diff + 1.0) / 6f32
    };
    trim -= surround_trim;
    trim -= 2f32 * tf_estimate;
    if (*analysis).valid != 0 {
        trim -= if -2.0f32
            > (if 2.0f32 < 2.0 * ((*analysis).tonality_slope + 0.05) {
                2.0
            } else {
                (2.0) * ((*analysis).tonality_slope + 0.05)
            }) {
            -2.0
        } else if 2.0f32 < 2.0 * ((*analysis).tonality_slope + 0.05) {
            2.0
        } else {
            (2.0) * ((*analysis).tonality_slope + 0.05)
        }
    }
    trim_index = crate::stdlib::floor((0.5 + trim) as f64) as i32;
    trim_index = if 0 > (if (10) < trim_index { 10 } else { trim_index }) {
        0
    } else if (10) < trim_index {
        10
    } else {
        trim_index
    };
    /*printf("%d\n", trim_index);*/
    return trim_index;
}

unsafe extern "C" fn stereo_analysis(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_norm,
    mut LM: i32,
    mut N0: i32,
) -> i32 {
    let mut _i: i32 = 0;
    let mut thetas: i32 = 0;
    let mut sumLR: crate::arch_h::opus_val32 = 1e-15;
    let mut sumMS: crate::arch_h::opus_val32 = 1e-15;
    /* Use the L1 norm to model the entropy of the L/R signal vs the M/S signal */

    for i in 0..13 {
        let mut _j: i32 = 0;
        for j in (*(*m).eBands.offset(i as isize) as i32) << LM
            ..(*(*m).eBands.offset((i + 1) as isize) as i32) << LM
        {
            let mut L: crate::arch_h::opus_val32 = 0.;

            let mut R: crate::arch_h::opus_val32 = 0.;

            let mut M: crate::arch_h::opus_val32 = 0.;

            let mut S: crate::arch_h::opus_val32 = 0.;

            L = *X.offset(j as isize);

            R = *X.offset((N0 + j) as isize);

            M = L + R;

            S = L - R;

            sumLR = sumLR
                + (crate::stdlib::fabs(L as f64) as f32 + crate::stdlib::fabs(R as f64) as f32);

            sumMS = sumMS
                + (crate::stdlib::fabs(M as f64) as f32 + crate::stdlib::fabs(S as f64) as f32);
        }
    }
    sumMS = 0.707107 * sumMS;
    thetas = 13;
    /* We don't need thetas for lower bands with LM<=1 */
    if LM <= 1 {
        thetas -= 8
    }
    return ((((*(*m).eBands.offset(13) as i32) << LM + 1) + thetas) as f32 * sumMS
        > ((*(*m).eBands.offset(13) as i32) << LM + 1) as f32 * sumLR) as i32;
}

unsafe extern "C" fn median_of_5(
    mut x: *const crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val16 {
    let mut t0: crate::arch_h::opus_val16 = 0.;
    let mut t1: crate::arch_h::opus_val16 = 0.;
    let mut t2: crate::arch_h::opus_val16 = 0.;
    let mut t3: crate::arch_h::opus_val16 = 0.;
    let mut t4: crate::arch_h::opus_val16 = 0.;
    t2 = *x.offset(2);
    if *x.offset(0) > *x.offset(1) {
        t0 = *x.offset(1);
        t1 = *x.offset(0)
    } else {
        t0 = *x.offset(0);
        t1 = *x.offset(1)
    }
    if *x.offset(3) > *x.offset(4) {
        t3 = *x.offset(4);
        t4 = *x.offset(3)
    } else {
        t3 = *x.offset(3);
        t4 = *x.offset(4)
    }
    if t0 > t3 {
        let mut tmp: crate::arch_h::opus_val16 = t0;
        t0 = t3;
        t3 = tmp;
        let mut tmp_0: crate::arch_h::opus_val16 = t1;
        t1 = t4;
        t4 = tmp_0
    }
    if t2 > t1 {
        if t1 < t3 {
            return if t2 < t3 { t2 } else { t3 };
        } else {
            return if t4 < t1 { t4 } else { t1 };
        }
    } else if t2 < t3 {
        return if t1 < t3 { t1 } else { t3 };
    } else {
        return if t2 < t4 { t2 } else { t4 };
    };
}

unsafe extern "C" fn median_of_3(
    mut x: *const crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val16 {
    let mut t0: crate::arch_h::opus_val16 = 0.;
    let mut t1: crate::arch_h::opus_val16 = 0.;
    let mut t2: crate::arch_h::opus_val16 = 0.;
    if *x.offset(0) > *x.offset(1) {
        t0 = *x.offset(1);
        t1 = *x.offset(0)
    } else {
        t0 = *x.offset(0);
        t1 = *x.offset(1)
    }
    t2 = *x.offset(2);
    if t1 < t2 {
        return t1;
    } else if t0 < t2 {
        return t2;
    } else {
        return t0;
    };
}

unsafe extern "C" fn dynalloc_analysis(
    mut bandLogE: *const crate::arch_h::opus_val16,
    mut bandLogE2: *const crate::arch_h::opus_val16,
    mut nbEBands: i32,
    mut start: i32,
    mut end: i32,
    mut C: i32,
    mut offsets: *mut i32,
    mut lsb_depth: i32,
    mut logN: *const crate::opus_types_h::opus_int16,
    mut isTransient: i32,
    mut vbr: i32,
    mut constrained_vbr: i32,
    mut eBands: *const crate::opus_types_h::opus_int16,
    mut LM: i32,
    mut effectiveBytes: i32,
    mut tot_boost_: *mut crate::opus_types_h::opus_int32,
    mut lfe: i32,
    mut surround_dynalloc: *mut crate::arch_h::opus_val16,
    mut analysis: *mut crate::celt_h::AnalysisInfo,
) -> crate::arch_h::opus_val16 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut tot_boost: crate::opus_types_h::opus_int32 = 0;
    let mut maxDepth: crate::arch_h::opus_val16 = 0.;
    let mut follower: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut noise_floor: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((C * nbEBands) as usize),
    );
    follower = fresh7.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((C * nbEBands) as usize),
    );
    noise_floor = fresh8.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memset(
        offsets as *mut libc::c_void,
        0,
        (nbEBands as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    /* Dynamic allocation code */
    maxDepth = -31.9;
    i = 0;
    while i < end {
        /* Noise floor must take into account eMeans, the depth, the width of the bands
        and the preemphasis filter (approx. square of bark band ID) */
        *noise_floor.offset(i as isize) = 0.0625
            * *logN.offset(i as isize) as crate::arch_h::opus_val32
            + 0.5
            + (9i32 - lsb_depth) as f32
            - crate::src::opus_1_2_1::celt::quant_bands::eMeans[i as usize]
            + 0.0062 * ((i + 5) * (i + 5)) as crate::arch_h::opus_val32;
        i += 1
    }
    c = 0;
    loop {
        i = 0;
        while i < end {
            maxDepth = if maxDepth
                > *bandLogE.offset((c * nbEBands + i) as isize) - *noise_floor.offset(i as isize)
            {
                maxDepth
            } else {
                (*bandLogE.offset((c * nbEBands + i) as isize)) - *noise_floor.offset(i as isize)
            };
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    /* Make sure that dynamic allocation can't make us bust the budget */
    if effectiveBytes > 50 && LM >= 1 && lfe == 0 {
        let mut last: i32 = 0;
        c = 0;
        loop {
            let mut offset: crate::arch_h::opus_val16 = 0.;
            let mut tmp: crate::arch_h::opus_val16 = 0.;
            let mut f: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            f = &mut *follower.offset((c * nbEBands) as isize) as *mut crate::arch_h::opus_val16;
            *f.offset(0) = *bandLogE2.offset((c * nbEBands) as isize);
            i = 1;
            while i < end {
                /* The last band to be at least 3 dB higher than the previous one
                is the last we'll consider. Otherwise, we run into problems on
                bandlimited signals. */
                if *bandLogE2.offset((c * nbEBands + i) as isize)
                    > *bandLogE2.offset((c * nbEBands + i - 1) as isize) + 0.5f32
                {
                    last = i
                }
                *f.offset(i as isize) = if *f.offset((i - 1) as isize) + 1.5f32
                    < *bandLogE2.offset((c * nbEBands + i) as isize)
                {
                    (*f.offset((i - 1) as isize)) + 1.5f32
                } else {
                    *bandLogE2.offset((c * nbEBands + i) as isize)
                };
                i += 1
            }
            i = last - 1;
            while i >= 0 {
                *f.offset(i as isize) = if *f.offset(i as isize)
                    < (if *f.offset((i + 1) as isize) + 2.0f32
                        < *bandLogE2.offset((c * nbEBands + i) as isize)
                    {
                        (*f.offset((i + 1) as isize)) + 2.0f32
                    } else {
                        *bandLogE2.offset((c * nbEBands + i) as isize)
                    }) {
                    *f.offset(i as isize)
                } else if *f.offset((i + 1) as isize) + 2.0f32
                    < *bandLogE2.offset((c * nbEBands + i) as isize)
                {
                    (*f.offset((i + 1) as isize)) + 2.0f32
                } else {
                    *bandLogE2.offset((c * nbEBands + i) as isize)
                };
                i -= 1
            }
            /* Combine with a median filter to avoid dynalloc triggering unnecessarily.
            The "offset" value controls how conservative we are -- a higher offset
            reduces the impact of the median filter and makes dynalloc use more bits. */
            offset = 1.0;
            i = 2;
            while i < end - 2 {
                *f.offset(i as isize) = if *f.offset(i as isize)
                    > median_of_5(&*bandLogE2.offset((c * nbEBands + i - 2) as isize)) - offset
                {
                    *f.offset(i as isize)
                } else {
                    (median_of_5(&*bandLogE2.offset((c * nbEBands + i - 2) as isize))) - offset
                };
                i += 1
            }
            tmp = median_of_3(&*bandLogE2.offset((c * nbEBands) as isize)) - offset;
            *f.offset(0) = if *f.offset(0) > tmp {
                *f.offset(0)
            } else {
                tmp
            };
            *f.offset(1) = if *f.offset(1) > tmp {
                *f.offset(1)
            } else {
                tmp
            };
            tmp = median_of_3(&*bandLogE2.offset((c * nbEBands + end - 3) as isize)) - offset;
            *f.offset((end - 2i32) as isize) = if *f.offset((end - 2i32) as isize) > tmp {
                *f.offset((end - 2i32) as isize)
            } else {
                tmp
            };
            *f.offset((end - 1i32) as isize) = if *f.offset((end - 1i32) as isize) > tmp {
                *f.offset((end - 1i32) as isize)
            } else {
                tmp
            };
            i = 0;
            while i < end {
                *f.offset(i as isize) = if *f.offset(i as isize) > *noise_floor.offset(i as isize) {
                    *f.offset(i as isize)
                } else {
                    *noise_floor.offset(i as isize)
                };
                i += 1
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        if C == 2 {
            i = start;
            while i < end {
                /* Consider 24 dB "cross-talk" */
                *follower.offset((nbEBands + i) as isize) = if *follower
                    .offset((nbEBands + i) as isize)
                    > *follower.offset(i as isize) - 4.0f32
                {
                    *follower.offset((nbEBands + i) as isize)
                } else {
                    (*follower.offset(i as isize)) - 4.0f32
                };
                *follower.offset(i as isize) = if *follower.offset(i as isize)
                    > *follower.offset((nbEBands + i) as isize) - 4.0f32
                {
                    *follower.offset(i as isize)
                } else {
                    (*follower.offset((nbEBands + i) as isize)) - 4.0f32
                };
                *follower.offset(i as isize) = 0.5f32
                    * ((if 0f32 > *bandLogE.offset(i as isize) - *follower.offset(i as isize) {
                        0f32
                    } else {
                        (*bandLogE.offset(i as isize)) - *follower.offset(i as isize)
                    }) + (if 0f32
                        > *bandLogE.offset((nbEBands + i) as isize)
                            - *follower.offset((nbEBands + i) as isize)
                    {
                        0f32
                    } else {
                        (*bandLogE.offset((nbEBands + i) as isize))
                            - *follower.offset((nbEBands + i) as isize)
                    }));
                i += 1
            }
        } else {
            i = start;
            while i < end {
                *follower.offset(i as isize) =
                    if 0f32 > *bandLogE.offset(i as isize) - *follower.offset(i as isize) {
                        0f32
                    } else {
                        (*bandLogE.offset(i as isize)) - *follower.offset(i as isize)
                    };
                i += 1
            }
        }
        i = start;
        while i < end {
            *follower.offset(i as isize) =
                if *follower.offset(i as isize) > *surround_dynalloc.offset(i as isize) {
                    *follower.offset(i as isize)
                } else {
                    *surround_dynalloc.offset(i as isize)
                };
            i += 1
        }
        /* For non-transient CBR/CVBR frames, halve the dynalloc contribution */
        if (vbr == 0 || constrained_vbr != 0) && isTransient == 0 {
            i = start;
            while i < end {
                *follower.offset(i as isize) = 0.5f32 * *follower.offset(i as isize);
                i += 1
            }
        }
        i = start;
        while i < end {
            if i < 8 {
                let ref mut fresh9 = *follower.offset(i as isize);
                *fresh9 *= 2f32
            }
            if i >= 12 {
                *follower.offset(i as isize) = 0.5f32 * *follower.offset(i as isize)
            }
            i += 1
        }
        if (*analysis).valid != 0 {
            i = start;
            while i < (if (19) < end { 19 } else { end }) {
                *follower.offset(i as isize) = *follower.offset(i as isize)
                    + 1.0 / 64.0 * (*analysis).leak_boost[i as usize] as i32 as f32;
                i += 1
            }
        }
        i = start;
        while i < end {
            let mut width: i32 = 0;
            let mut boost: i32 = 0;
            let mut boost_bits: i32 = 0;
            *follower.offset(i as isize) = if *follower.offset(i as isize) < 4f32 {
                *follower.offset(i as isize)
            } else {
                4f32
            };
            width = (C
                * (*eBands.offset((i + 1) as isize) as i32 - *eBands.offset(i as isize) as i32))
                << LM;
            if width < 6 {
                boost = *follower.offset(i as isize) as i32;
                boost_bits = boost * width << 3
            } else if width > 48 {
                boost = (*follower.offset(i as isize) * 8f32) as i32;
                boost_bits = (boost * width << 3) / 8
            } else {
                boost = (*follower.offset(i as isize) * width as f32 / 6f32) as i32;
                boost_bits = (boost * 6) << 3
            }
            /* For CBR and non-transient CVBR frames, limit dynalloc to 2/3 of the bits */
            if (vbr == 0 || constrained_vbr != 0 && isTransient == 0)
                && tot_boost + boost_bits >> 3 >> 3 > 2 * effectiveBytes / 3
            {
                let mut cap: crate::opus_types_h::opus_int32 = ((2 * effectiveBytes / 3) << 3) << 3;
                *offsets.offset(i as isize) = cap - tot_boost;
                tot_boost = cap;
                break;
            } else {
                *offsets.offset(i as isize) = boost;
                tot_boost += boost_bits;
                i += 1
            }
        }
    }
    *tot_boost_ = tot_boost;
    return maxDepth;
}

unsafe extern "C" fn run_prefilter(
    mut st: *mut OpusCustomEncoder,
    mut in_0: *mut crate::arch_h::celt_sig,
    mut prefilter_mem: *mut crate::arch_h::celt_sig,
    mut CC: i32,
    mut N: i32,
    mut prefilter_tapset: i32,
    mut pitch: *mut i32,
    mut gain: *mut crate::arch_h::opus_val16,
    mut qgain: *mut i32,
    mut enabled: i32,
    mut nbAvailableBytes: i32,
) -> i32 {
    let mut c: i32 = 0;
    let mut _pre: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut pre: [*mut crate::arch_h::celt_sig; 2] = [0 as *mut crate::arch_h::celt_sig; 2];
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut pitch_index: i32 = 0;
    let mut gain1: crate::arch_h::opus_val16 = 0.;
    let mut pf_threshold: crate::arch_h::opus_val16 = 0.;
    let mut pf_on: i32 = 0;
    let mut qg: i32 = 0;
    let mut overlap: i32 = 0;
    mode = (*st).mode;
    overlap = (*mode).overlap;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>())
            .wrapping_mul((CC * (N + 1024i32)) as usize),
    );
    _pre = fresh10.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    pre[0] = _pre;
    pre[1] = _pre.offset((N + 1024i32) as isize);
    c = 0;
    loop {
        crate::stdlib::memcpy(
            pre[c as usize] as *mut libc::c_void,
            prefilter_mem.offset((c * 1024) as isize) as *const libc::c_void,
            (1024usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                .wrapping_add(
                    (0 * pre[c as usize]
                        .wrapping_offset_from(prefilter_mem.offset((c * 1024) as isize)))
                        as usize,
                ),
        );
        crate::stdlib::memcpy(
            pre[c as usize].offset(1024) as *mut libc::c_void,
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize) as *const libc::c_void,
            (N as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                .wrapping_add(
                    (0 * pre[c as usize].offset(1024).wrapping_offset_from(
                        in_0.offset((c * (N + overlap)) as isize)
                            .offset(overlap as isize),
                    )) as usize,
                ),
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if enabled != 0 {
        let mut pitch_buf: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
        let mut fresh11 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_mul((1024i32 + N >> 1) as usize),
        );
        pitch_buf = fresh11.as_mut_ptr() as *mut crate::arch_h::opus_val16;
        crate::src::opus_1_2_1::celt::pitch::pitch_downsample(
            pre.as_mut_ptr(),
            pitch_buf,
            1024 + N,
            CC,
            (*st).arch,
        );
        /* Don't search for the fir last 1.5 octave of the range because
        there's too many false-positives due to short-term correlation */
        crate::src::opus_1_2_1::celt::pitch::pitch_search(
            pitch_buf.offset((1024i32 >> 1) as isize),
            pitch_buf,
            N,
            1024 - 3 * 15,
            &mut pitch_index,
            (*st).arch,
        );
        pitch_index = 1024 - pitch_index;
        gain1 = crate::src::opus_1_2_1::celt::pitch::remove_doubling(
            pitch_buf,
            1024,
            15,
            N,
            &mut pitch_index,
            (*st).prefilter_period,
            (*st).prefilter_gain,
            (*st).arch,
        );
        if pitch_index > 1024 - 2 {
            pitch_index = 1024 - 2
        }
        gain1 = 0.7 * gain1;
        /*printf("%d %d %f %f\n", pitch_change, pitch_index, gain1, st->analysis.tonality);*/
        if (*st).loss_rate > 2 {
            gain1 = 0.5 * gain1
        }
        if (*st).loss_rate > 4 {
            gain1 = 0.5 * gain1
        }
        if (*st).loss_rate > 8 {
            gain1 = 0f32
        }
    } else {
        gain1 = 0f32;
        pitch_index = 15
    }
    /* Gain threshold for enabling the prefilter/postfilter */
    pf_threshold = 0.2;
    /* Adjusting the threshold based on rate and continuity */
    if crate::stdlib::abs(pitch_index - (*st).prefilter_period) * 10 > pitch_index {
        pf_threshold += 0.2
    }
    if nbAvailableBytes < 25 {
        pf_threshold += 0.1
    }
    if nbAvailableBytes < 35 {
        pf_threshold += 0.1
    }
    if (*st).prefilter_gain > 0.4f32 {
        pf_threshold -= 0.1
    }
    if (*st).prefilter_gain > 0.55f32 {
        pf_threshold -= 0.1
    }
    /* Hard threshold at 0.2 */
    pf_threshold = if pf_threshold > 0.2 {
        pf_threshold
    } else {
        0.2
    };
    if gain1 < pf_threshold {
        gain1 = 0f32;
        pf_on = 0;
        qg = 0
    } else {
        /*This block is not gated by a total bits check only because
        of the nbAvailableBytes check above.*/
        if (crate::stdlib::fabs((gain1 - (*st).prefilter_gain) as f64) as f32) < 0.1 {
            gain1 = (*st).prefilter_gain
        }
        qg = crate::stdlib::floor((0.5 + gain1 * 32f32 / 3f32) as f64) as i32 - 1;
        qg = if 0 > (if (7) < qg { 7 } else { qg }) {
            0
        } else if (7) < qg {
            7
        } else {
            qg
        };
        gain1 = 0.09375 * (qg + 1) as f32;
        pf_on = 1
    }
    /*printf("%d %f\n", pitch_index, gain1);*/
    c = 0;
    loop {
        let mut offset: i32 = (*mode).shortMdctSize - overlap;
        (*st).prefilter_period = if (*st).prefilter_period > 15 {
            (*st).prefilter_period
        } else {
            15
        };
        crate::stdlib::memcpy(
            in_0.offset((c * (N + overlap)) as isize) as *mut libc::c_void,
            (*st).in_mem.as_mut_ptr().offset((c * overlap) as isize) as *const libc::c_void,
            (overlap as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                .wrapping_add(
                    (0 * in_0
                        .offset((c * (N + overlap)) as isize)
                        .wrapping_offset_from(
                            (*st).in_mem.as_mut_ptr().offset((c * overlap) as isize),
                        )) as usize,
                ),
        );
        if offset != 0 {
            crate::src::opus_1_2_1::celt::celt::comb_filter(
                in_0.offset((c * (N + overlap)) as isize)
                    .offset(overlap as isize),
                pre[c as usize].offset(1024isize),
                (*st).prefilter_period,
                (*st).prefilter_period,
                offset,
                -(*st).prefilter_gain,
                -(*st).prefilter_gain,
                (*st).prefilter_tapset,
                (*st).prefilter_tapset,
                0 as *const crate::arch_h::opus_val16,
                0i32,
                (*st).arch,
            );
        }
        crate::src::opus_1_2_1::celt::celt::comb_filter(
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize)
                .offset(offset as isize),
            pre[c as usize].offset(1024).offset(offset as isize),
            (*st).prefilter_period,
            pitch_index,
            N - offset,
            -(*st).prefilter_gain,
            -gain1,
            (*st).prefilter_tapset,
            prefilter_tapset,
            (*mode).window,
            overlap,
            (*st).arch,
        );
        crate::stdlib::memcpy(
            (*st).in_mem.as_mut_ptr().offset((c * overlap) as isize) as *mut libc::c_void,
            in_0.offset((c * (N + overlap)) as isize).offset(N as isize) as *const libc::c_void,
            (overlap as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                .wrapping_add(
                    (0 * (*st)
                        .in_mem
                        .as_mut_ptr()
                        .offset((c * overlap) as isize)
                        .wrapping_offset_from(
                            in_0.offset((c * (N + overlap)) as isize).offset(N as isize),
                        )) as usize,
                ),
        );
        if N > 1024 {
            crate::stdlib::memcpy(
                prefilter_mem.offset((c * 1024i32) as isize) as *mut libc::c_void,
                pre[c as usize].offset(N as isize) as *const libc::c_void,
                (1024usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                    .wrapping_add(
                        (0isize
                            * prefilter_mem
                                .offset((c * 1024i32) as isize)
                                .wrapping_offset_from(pre[c as usize].offset(N as isize)))
                            as usize,
                    ),
            );
        } else {
            crate::stdlib::memmove(
                prefilter_mem.offset((c * 1024) as isize) as *mut libc::c_void,
                prefilter_mem.offset((c * 1024) as isize).offset(N as isize) as *const libc::c_void,
                ((1024i32 - N) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                    .wrapping_add(
                        (0 * prefilter_mem
                            .offset((c * 1024) as isize)
                            .wrapping_offset_from(
                                prefilter_mem.offset((c * 1024) as isize).offset(N as isize),
                            )) as usize,
                    ),
            );
            crate::stdlib::memcpy(
                prefilter_mem
                    .offset((c * 1024i32) as isize)
                    .offset(1024isize)
                    .offset(-(N as isize)) as *mut libc::c_void,
                pre[c as usize].offset(1024isize) as *const libc::c_void,
                (N as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                    .wrapping_add(
                        (0isize
                            * prefilter_mem
                                .offset((c * 1024i32) as isize)
                                .offset(1024isize)
                                .offset(-(N as isize))
                                .wrapping_offset_from(pre[c as usize].offset(1024isize)))
                            as usize,
                    ),
            );
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    *gain = gain1;
    *pitch = pitch_index;
    *qgain = qg;
    return pf_on;
}

unsafe extern "C" fn compute_vbr(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut analysis: *mut crate::celt_h::AnalysisInfo,
    mut base_target: crate::opus_types_h::opus_int32,
    mut LM: i32,
    mut bitrate: crate::opus_types_h::opus_int32,
    mut lastCodedBands: i32,
    mut C: i32,
    mut intensity: i32,
    mut constrained_vbr: i32,
    mut stereo_saving: crate::arch_h::opus_val16,
    mut tot_boost: i32,
    mut tf_estimate: crate::arch_h::opus_val16,
    mut pitch_change: i32,
    mut maxDepth: crate::arch_h::opus_val16,
    mut lfe: i32,
    mut has_surround_mask: i32,
    mut surround_masking: crate::arch_h::opus_val16,
    mut temporal_vbr: crate::arch_h::opus_val16,
) -> i32 {
    /* The target rate in 8th bits per frame */
    let mut target: crate::opus_types_h::opus_int32 = 0;
    let mut coded_bins: i32 = 0;
    let mut coded_bands: i32 = 0;
    let mut tf_calibration: crate::arch_h::opus_val16 = 0.;
    let mut nbEBands: i32 = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    nbEBands = (*mode).nbEBands;
    eBands = (*mode).eBands;
    coded_bands = if lastCodedBands != 0 {
        lastCodedBands
    } else {
        nbEBands
    };
    coded_bins = (*eBands.offset(coded_bands as isize) as i32) << LM;
    if C == 2 {
        coded_bins += (*eBands.offset(
            (if intensity < coded_bands {
                intensity
            } else {
                coded_bands
            }) as isize,
        ) as i32)
            << LM
    }
    target = base_target;
    /*printf("%f %f %f %f %d %d ", st->analysis.activity, st->analysis.tonality, tf_estimate, st->stereo_saving, tot_boost, coded_bands);*/
    if (*analysis).valid != 0 && ((*analysis).activity as f64) < 0.4 {
        target -= ((coded_bins << 3) as f32 * (0.4 - (*analysis).activity))
            as crate::opus_types_h::opus_int32
    }
    /* Stereo savings */
    if C == 2 {
        let mut coded_stereo_bands: i32 = 0;
        let mut coded_stereo_dof: i32 = 0;
        let mut max_frac: crate::arch_h::opus_val16 = 0.;
        coded_stereo_bands = if intensity < coded_bands {
            intensity
        } else {
            coded_bands
        };
        coded_stereo_dof =
            ((*eBands.offset(coded_stereo_bands as isize) as i32) << LM) - coded_stereo_bands;
        /* Maximum fraction of the bits we can save if the signal is mono. */
        max_frac = 0.8 * coded_stereo_dof as crate::arch_h::opus_val32
            / coded_bins as crate::arch_h::opus_val16;
        stereo_saving = if stereo_saving < 1.0f32 {
            stereo_saving
        } else {
            1.0f32
        };
        /*printf("%d %d %d ", coded_stereo_dof, coded_bins, tot_boost);*/
        target -= if (max_frac * target as f32)
            < (stereo_saving - 0.1) * (coded_stereo_dof << 3) as crate::arch_h::opus_val32
        {
            (max_frac) * target as f32
        } else {
            (stereo_saving - 0.1) * (coded_stereo_dof << 3) as crate::arch_h::opus_val32
        } as crate::opus_types_h::opus_int32
    }
    /* Boost the rate according to dynalloc (minus the dynalloc average for calibration). */
    target += tot_boost - ((19) << LM);
    /* Apply transient boost, compensating for average boost. */
    tf_calibration = 0.044;
    target += ((tf_estimate - tf_calibration) * target as f32) as crate::opus_types_h::opus_int32;
    /* Apply tonality boost */
    if (*analysis).valid != 0 && lfe == 0 {
        let mut tonal_target: crate::opus_types_h::opus_int32 = 0;
        let mut tonal: f32 = 0.;
        /* Tonality boost (compensating for the average). */
        tonal = (if 0.0f32 > (*analysis).tonality - 0.15 {
            0.0
        } else {
            ((*analysis).tonality) - 0.15
        }) - 0.12;
        tonal_target =
            target + ((coded_bins << 3) as f32 * 1.2 * tonal) as crate::opus_types_h::opus_int32;
        if pitch_change != 0 {
            tonal_target += ((coded_bins << 3) as f32 * 0.8) as crate::opus_types_h::opus_int32
        }
        /*printf("%f %f ", analysis->tonality, tonal);*/
        target = tonal_target
    }
    if has_surround_mask != 0 && lfe == 0 {
        let mut surround_target: crate::opus_types_h::opus_int32 = target
            + (surround_masking * (coded_bins << 3) as crate::arch_h::opus_val32)
                as crate::opus_types_h::opus_int32;
        /*printf("%f %d %d %d %d %d %d ", surround_masking, coded_bins, st->end, st->intensity, surround_target, target, st->bitrate);*/
        target = if target / 4 > surround_target {
            (target) / 4
        } else {
            surround_target
        }
    }
    let mut floor_depth: crate::opus_types_h::opus_int32 = 0;
    let mut bins: i32 = 0;
    bins = (*eBands.offset((nbEBands - 2) as isize) as i32) << LM;
    /*printf("%f %d\n", maxDepth, floor_depth);*/
    floor_depth = ((C * bins << 3) as crate::arch_h::opus_val32 * maxDepth)
        as crate::opus_types_h::opus_int32;
    floor_depth = if floor_depth > target >> 2 {
        floor_depth
    } else {
        (target) >> 2
    };
    target = if target < floor_depth {
        target
    } else {
        floor_depth
    };
    /*floor_depth = SHR32(MULT16_16((C*bins<<BITRES),celt_log2(SHL32(MAX16(1,sample_max),13))), DB_SHIFT);*/
    /* Make VBR less aggressive for constrained VBR because we can't keep a higher bitrate
    for long. Needs tuning. */
    if (has_surround_mask == 0 || lfe != 0) && constrained_vbr != 0 {
        target =
            base_target + (0.67 * (target - base_target) as f32) as crate::opus_types_h::opus_int32
    }
    if has_surround_mask == 0 && tf_estimate < 0.2f32 {
        let mut amount: crate::arch_h::opus_val16 = 0.;
        let mut tvbr_factor: crate::arch_h::opus_val16 = 0.;
        amount = 0.0000031
            * (if 0
                > (if (32000) < 96000 - bitrate {
                    32000
                } else {
                    (96000) - bitrate
                })
            {
                0i32
            } else {
                (if (32000) < 96000 - bitrate {
                    32000
                } else {
                    (96000) - bitrate
                })
            }) as f32;
        tvbr_factor = temporal_vbr * amount;
        target += (tvbr_factor * target as f32) as crate::opus_types_h::opus_int32
    }
    /* Don't allow more than doubling the rate */
    target = if 2 * base_target < target {
        (2) * base_target
    } else {
        target
    };
    return target;
}
#[no_mangle]

pub unsafe extern "C" fn celt_encode_with_ec(
    mut st: *mut OpusCustomEncoder,
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: i32,
    mut compressed: *mut u8,
    mut nbCompressedBytes: i32,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut N: i32 = 0;
    let mut bits: crate::opus_types_h::opus_int32 = 0;
    let mut _enc: crate::src::opus_1_2_1::celt::entcode::ec_enc =
        crate::src::opus_1_2_1::celt::entcode::ec_enc {
            buf: 0 as *mut u8,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
    let mut in_0: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut freq: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut X: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut bandE: *mut crate::arch_h::celt_ener = 0 as *mut crate::arch_h::celt_ener;
    let mut bandLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut bandLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fine_quant: *mut i32 = 0 as *mut i32;
    let mut error: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut pulses: *mut i32 = 0 as *mut i32;
    let mut cap: *mut i32 = 0 as *mut i32;
    let mut offsets: *mut i32 = 0 as *mut i32;
    let mut fine_priority: *mut i32 = 0 as *mut i32;
    let mut tf_res: *mut i32 = 0 as *mut i32;
    let mut collapse_masks: *mut u8 = 0 as *mut u8;
    let mut prefilter_mem: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut energyError: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut shortBlocks: i32 = 0;
    let mut isTransient: i32 = 0;
    let CC: i32 = (*st).channels;
    let C: i32 = (*st).stream_channels;
    let mut LM: i32 = 0;
    let mut M: i32 = 0;
    let mut tf_select: i32 = 0;
    let mut nbFilledBytes: i32 = 0;
    let mut nbAvailableBytes: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut effEnd: i32 = 0;
    let mut codedBands: i32 = 0;
    let mut alloc_trim: i32 = 0;
    let mut pitch_index: i32 = 15;
    let mut gain1: crate::arch_h::opus_val16 = 0f32;
    let mut dual_stereo: i32 = 0;
    let mut effectiveBytes: i32 = 0;
    let mut dynalloc_logp: i32 = 0;
    let mut vbr_rate: crate::opus_types_h::opus_int32 = 0;
    let mut total_bits: crate::opus_types_h::opus_int32 = 0;
    let mut total_boost: crate::opus_types_h::opus_int32 = 0;
    let mut balance: crate::opus_types_h::opus_int32 = 0;
    let mut tell: crate::opus_types_h::opus_int32 = 0;
    let mut tell0_frac: crate::opus_types_h::opus_int32 = 0;
    let mut prefilter_tapset: i32 = 0;
    let mut pf_on: i32 = 0;
    let mut anti_collapse_rsv: i32 = 0;
    let mut anti_collapse_on: i32 = 0;
    let mut silence: i32 = 0;
    let mut tf_chan: i32 = 0;
    let mut tf_estimate: crate::arch_h::opus_val16 = 0.;
    let mut pitch_change: i32 = 0;
    let mut tot_boost: crate::opus_types_h::opus_int32 = 0;
    let mut sample_max: crate::arch_h::opus_val32 = 0.;
    let mut maxDepth: crate::arch_h::opus_val16 = 0.;
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut nbEBands: i32 = 0;
    let mut overlap: i32 = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut secondMdct: i32 = 0;
    let mut signalBandwidth: i32 = 0;
    let mut transient_got_disabled: i32 = 0;
    let mut surround_masking: crate::arch_h::opus_val16 = 0f32;
    let mut temporal_vbr: crate::arch_h::opus_val16 = 0f32;
    let mut surround_trim: crate::arch_h::opus_val16 = 0f32;
    let mut equiv_rate: crate::opus_types_h::opus_int32 = 0;
    let mut hybrid: i32 = 0;
    let mut weak_transient: i32 = 0;
    let mut surround_dynalloc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    start = (*st).start;
    end = (*st).end;
    hybrid = (start != 0) as i32;
    tf_estimate = 0f32;
    if nbCompressedBytes < 2 || pcm.is_null() {
        return -(1i32);
    }
    frame_size *= (*st).upsample;
    LM = 0;
    while LM <= (*mode).maxLM {
        if (*mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1
    }
    if LM > (*mode).maxLM {
        return -(1i32);
    }
    M = (1) << LM;
    N = M * (*mode).shortMdctSize;
    prefilter_mem = (*st).in_mem.as_mut_ptr().offset((CC * overlap) as isize);
    oldBandE = (*st)
        .in_mem
        .as_mut_ptr()
        .offset((CC * (overlap + 1024)) as isize);
    oldLogE = oldBandE.offset((CC * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((CC * nbEBands) as isize);
    energyError = oldLogE2.offset((CC * nbEBands) as isize);
    if enc.is_null() {
        tell = 1;
        tell0_frac = tell;
        nbFilledBytes = 0
    } else {
        tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc)
            as crate::opus_types_h::opus_int32;
        tell0_frac = tell;
        tell = ec_tell(enc);
        nbFilledBytes = tell + 4 >> 3
    }
    /* Can't produce more than 1275 output bytes */
    nbCompressedBytes = if nbCompressedBytes < 1275 {
        nbCompressedBytes
    } else {
        1275
    };
    nbAvailableBytes = nbCompressedBytes - nbFilledBytes;
    if (*st).vbr != 0 && (*st).bitrate != -(1) {
        let mut den: crate::opus_types_h::opus_int32 = (*mode).Fs >> 3;
        vbr_rate = ((*st).bitrate * frame_size + (den >> 1)) / den;
        effectiveBytes = vbr_rate >> 3 + 3
    } else {
        let mut tmp: crate::opus_types_h::opus_int32 = 0;
        vbr_rate = 0;
        tmp = (*st).bitrate * frame_size;
        if tell > 1 {
            tmp += tell
        }
        if (*st).bitrate != -(1) {
            nbCompressedBytes = if 2
                > (if nbCompressedBytes
                    < (tmp + 4 * (*mode).Fs) / (8 * (*mode).Fs) - ((*st).signalling != 0) as i32
                {
                    nbCompressedBytes
                } else {
                    ((tmp + 4 * (*mode).Fs) / (8 * (*mode).Fs)) - ((*st).signalling != 0) as i32
                }) {
                2
            } else if nbCompressedBytes
                < (tmp + 4 * (*mode).Fs) / (8 * (*mode).Fs) - ((*st).signalling != 0) as i32
            {
                nbCompressedBytes
            } else {
                ((tmp + 4 * (*mode).Fs) / (8 * (*mode).Fs)) - ((*st).signalling != 0) as i32
            }
        }
        effectiveBytes = nbCompressedBytes - nbFilledBytes
    }
    equiv_rate = (nbCompressedBytes * 8 * 50 >> 3 - LM) - (40 * C + 20) * ((400 >> LM) - 50);
    if (*st).bitrate != -(1) {
        equiv_rate = if equiv_rate < (*st).bitrate - (40 * C + 20) * ((400 >> LM) - 50) {
            equiv_rate
        } else {
            ((*st).bitrate) - (40 * C + 20) * ((400 >> LM) - 50)
        }
    }
    if enc.is_null() {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_init(
            &mut _enc,
            compressed,
            nbCompressedBytes as crate::opus_types_h::opus_uint32,
        );
        enc = &mut _enc
    }
    if vbr_rate > 0 {
        /* Computes the max bit-rate allowed in VBR mode to avoid violating the
         target rate and buffering.
        We must do this up front so that bust-prevention logic triggers
         correctly if we don't have enough bits. */
        if (*st).constrained_vbr != 0 {
            let mut vbr_bound: crate::opus_types_h::opus_int32 = 0;
            let mut max_allowed: crate::opus_types_h::opus_int32 = 0;
            /* We could use any multiple of vbr_rate as bound (depending on the
             delay).
            This is clamped to ensure we use at least two bytes if the encoder
             was entirely empty, but to allow 0 in hybrid mode. */
            vbr_bound = vbr_rate;
            max_allowed = if (if (if tell == 1 { 2 } else { 0 })
                > vbr_rate + vbr_bound - (*st).vbr_reservoir >> 3 + 3
            {
                (if tell == 1 { 2 } else { 0 })
            } else {
                (vbr_rate + vbr_bound - (*st).vbr_reservoir) >> 3 + 3
            }) < nbAvailableBytes
            {
                if (if tell == 1 { 2 } else { 0 })
                    > vbr_rate + vbr_bound - (*st).vbr_reservoir >> 3 + 3
                {
                    if tell == 1 {
                        2
                    } else {
                        0
                    }
                } else {
                    (vbr_rate + vbr_bound - (*st).vbr_reservoir) >> 3 + 3
                }
            } else {
                nbAvailableBytes
            };
            if max_allowed < nbAvailableBytes {
                nbCompressedBytes = nbFilledBytes + max_allowed;
                nbAvailableBytes = max_allowed;
                crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
                    enc,
                    nbCompressedBytes as crate::opus_types_h::opus_uint32,
                );
            }
        }
    }
    total_bits = nbCompressedBytes * 8;
    effEnd = end;
    if effEnd > (*mode).effEBands {
        effEnd = (*mode).effEBands
    }
    let mut fresh12 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>())
            .wrapping_mul((CC * (N + overlap)) as usize),
    );
    in_0 = fresh12.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    sample_max = if (*st).overlap_max > celt_maxabs16(pcm, C * (N - overlap) / (*st).upsample) {
        (*st).overlap_max
    } else {
        celt_maxabs16(pcm, C * (N - overlap) / (*st).upsample)
    };
    (*st).overlap_max = celt_maxabs16(
        pcm.offset((C * (N - overlap) / (*st).upsample) as isize),
        C * overlap / (*st).upsample,
    );
    sample_max = if sample_max > (*st).overlap_max {
        sample_max
    } else {
        (*st).overlap_max
    };
    silence = (sample_max <= 1f32 / ((1i32) << (*st).lsb_depth) as f32) as i32;
    if tell == 1 {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, silence, 15u32);
    } else {
        silence = 0
    }
    if silence != 0 {
        /*In VBR mode there is no need to send more than the minimum. */
        if vbr_rate > 0 {
            nbCompressedBytes = if nbCompressedBytes < nbFilledBytes + 2 {
                nbCompressedBytes
            } else {
                (nbFilledBytes) + 2
            };
            effectiveBytes = nbCompressedBytes;
            total_bits = nbCompressedBytes * 8;
            nbAvailableBytes = 2;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
                enc,
                nbCompressedBytes as crate::opus_types_h::opus_uint32,
            );
        }
        /* Pretend we've filled all the remaining bits with zeros
        (that's what the initialiser did anyway) */
        tell = nbCompressedBytes * 8;
        (*enc).nbits_total += tell - ec_tell(enc)
    }
    c = 0;
    loop {
        let mut need_clip: i32 = 0;
        need_clip = ((*st).clip != 0 && sample_max > 65536.0) as i32;
        celt_preemphasis(
            pcm.offset(c as isize),
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize),
            N,
            CC,
            (*st).upsample,
            (*mode).preemph.as_ptr(),
            (*st).preemph_memE.as_mut_ptr().offset(c as isize),
            need_clip,
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    /* Find pitch period and gain */
    let mut enabled: i32 = 0;
    let mut qg: i32 = 0;
    enabled = (((*st).lfe != 0 && nbAvailableBytes > 3 || nbAvailableBytes > 12 * C)
        && hybrid == 0
        && silence == 0
        && (*st).disable_pf == 0
        && (*st).complexity >= 5) as i32;
    prefilter_tapset = (*st).tapset_decision;
    pf_on = run_prefilter(
        st,
        in_0,
        prefilter_mem,
        CC,
        N,
        prefilter_tapset,
        &mut pitch_index,
        &mut gain1,
        &mut qg,
        enabled,
        nbAvailableBytes,
    );
    if (gain1 > 0.4 || (*st).prefilter_gain > 0.4f32)
        && ((*st).analysis.valid == 0 || (*st).analysis.tonality as f64 > 0.3)
        && (pitch_index as f64 > 1.26 * (*st).prefilter_period as f64
            || (pitch_index as f64) < 0.79 * (*st).prefilter_period as f64)
    {
        pitch_change = 1
    }
    if pf_on == 0 {
        if hybrid == 0 && tell + 16 <= total_bits {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, 0i32, 1u32);
        }
    } else {
        /*This block is not gated by a total bits check only because
        of the nbAvailableBytes check above.*/
        let mut octave: i32 = 0;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, 1, 1);
        pitch_index += 1;
        octave = ::std::mem::size_of::<u32>() as i32 * 8
            - (pitch_index as u32).leading_zeros() as i32
            - 5;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
            enc,
            octave as crate::opus_types_h::opus_uint32,
            6,
        );
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
            enc,
            (pitch_index - ((16) << octave)) as crate::opus_types_h::opus_uint32,
            (4 + octave) as u32,
        );
        pitch_index -= 1;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
            enc,
            qg as crate::opus_types_h::opus_uint32,
            3,
        );
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
            enc,
            prefilter_tapset,
            tapset_icdf.as_ptr(),
            2u32,
        );
    }
    isTransient = 0;
    shortBlocks = 0;
    if (*st).complexity >= 1 && (*st).lfe == 0 {
        /* Reduces the likelihood of energy instability on fricatives at low bitrate
        in hybrid mode. It seems like we still want to have real transients on vowels
        though (small SILK quantization offset value). */
        let mut allow_weak_transients: i32 =
            (hybrid != 0 && effectiveBytes < 15 && (*st).silk_info.offset >= 100) as i32; /* *< Interleaved signal MDCTs */
        isTransient = transient_analysis(
            in_0,
            N + overlap,
            CC,
            &mut tf_estimate,
            &mut tf_chan,
            allow_weak_transients,
            &mut weak_transient,
        )
    }
    if LM > 0 && ec_tell(enc) + 3 <= total_bits {
        if isTransient != 0 {
            shortBlocks = M
        }
    } else {
        isTransient = 0;
        transient_got_disabled = 1
    }
    let mut fresh13 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>()).wrapping_mul((CC * N) as usize),
    );
    freq = fresh13.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    let mut fresh14 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_ener>()).wrapping_mul((nbEBands * CC) as usize),
    );
    bandE = fresh14.as_mut_ptr() as *mut crate::arch_h::celt_ener;
    let mut fresh15 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((nbEBands * CC) as usize),
    );
    bandLogE = fresh15.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    secondMdct = (shortBlocks != 0 && (*st).complexity >= 8) as i32;
    let mut fresh16 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((C * nbEBands) as usize),
    );
    bandLogE2 = fresh16.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if secondMdct != 0 {
        compute_mdcts(mode, 0, in_0, freq, C, CC, LM, (*st).upsample, (*st).arch);
        crate::src::opus_1_2_1::celt::bands::compute_band_energies(
            mode,
            freq,
            bandE,
            effEnd,
            C,
            LM,
            (*st).arch,
        );
        crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(mode, effEnd, end, bandE, bandLogE2, C);
        i = 0;
        while i < C * nbEBands {
            let ref mut fresh17 = *bandLogE2.offset(i as isize);
            *fresh17 += 0.5 * LM as f32;
            i += 1
        }
    }
    compute_mdcts(
        mode,
        shortBlocks,
        in_0,
        freq,
        C,
        CC,
        LM,
        (*st).upsample,
        (*st).arch,
    );
    if CC == 2 && C == 1 {
        tf_chan = 0
    }
    crate::src::opus_1_2_1::celt::bands::compute_band_energies(
        mode,
        freq,
        bandE,
        effEnd,
        C,
        LM,
        (*st).arch,
    );
    if (*st).lfe != 0 {
        i = 2;
        while i < end {
            *bandE.offset(i as isize) = if *bandE.offset(i as isize) < 1e-4f32 * *bandE.offset(0) {
                *bandE.offset(i as isize)
            } else {
                (1e-4f32) * *bandE.offset(0)
            };
            *bandE.offset(i as isize) = if *bandE.offset(i as isize) > 1e-15f32 {
                *bandE.offset(i as isize)
            } else {
                1e-15f32
            };
            i += 1
        }
    }
    crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(mode, effEnd, end, bandE, bandLogE, C);
    let mut fresh18 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((C * nbEBands) as usize),
    );
    surround_dynalloc = fresh18.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memset(
        surround_dynalloc as *mut libc::c_void,
        0,
        (end as usize).wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
    );
    /* This computes how much masking takes place between surround channels */
    if hybrid == 0 && !(*st).energy_mask.is_null() && (*st).lfe == 0 {
        let mut mask_end: i32 = 0;
        let mut midband: i32 = 0;
        let mut count_dynalloc: i32 = 0;
        let mut mask_avg: crate::arch_h::opus_val32 = 0f32;
        let mut diff: crate::arch_h::opus_val32 = 0f32;
        let mut count: i32 = 0;
        mask_end = if 2 > (*st).lastCodedBands {
            2
        } else {
            (*st).lastCodedBands
        };
        c = 0;
        while c < C {
            i = 0;
            while i < mask_end {
                let mut mask: crate::arch_h::opus_val16 = 0.;
                mask = if (if *(*st).energy_mask.offset((nbEBands * c + i) as isize) < 0.25f32 {
                    *(*st).energy_mask.offset((nbEBands * c + i) as isize)
                } else {
                    0.25f32
                }) > -2.0
                {
                    if *(*st).energy_mask.offset((nbEBands * c + i) as isize) < 0.25f32 {
                        *(*st).energy_mask.offset((nbEBands * c + i) as isize)
                    } else {
                        0.25
                    }
                } else {
                    -2.0
                };
                if mask > 0f32 {
                    mask = 0.5 * mask
                }
                mask_avg += mask
                    * (*eBands.offset((i + 1) as isize) as i32 - *eBands.offset(i as isize) as i32)
                        as crate::arch_h::opus_val32;
                count +=
                    *eBands.offset((i + 1) as isize) as i32 - *eBands.offset(i as isize) as i32;
                diff += mask * (1 + 2 * i - mask_end) as crate::arch_h::opus_val32;
                i += 1
            }
            c += 1
        }
        mask_avg = mask_avg / count as crate::arch_h::opus_val16;
        mask_avg += 0.2;
        diff = diff * 6f32 / (C * (mask_end - 1) * (mask_end + 1) * mask_end) as f32;
        /* Again, being conservative */
        diff = 0.5 * diff;
        diff = if (if diff < 0.031 { diff } else { 0.031 }) > -0.031 {
            if diff < 0.031 {
                diff
            } else {
                0.031
            }
        } else {
            -0.031
        };
        /* Find the band that's in the middle of the coded spectrum */
        midband = 0;
        while (*eBands.offset((midband + 1) as isize) as i32)
            < *eBands.offset(mask_end as isize) as i32 / 2
        {
            midband += 1
        }
        count_dynalloc = 0;
        i = 0;
        while i < mask_end {
            let mut lin: crate::arch_h::opus_val32 = 0.;
            let mut unmask: crate::arch_h::opus_val16 = 0.;
            lin = mask_avg + diff * (i - midband) as f32;
            if C == 2 {
                unmask = if *(*st).energy_mask.offset(i as isize)
                    > *(*st).energy_mask.offset((nbEBands + i) as isize)
                {
                    *(*st).energy_mask.offset(i as isize)
                } else {
                    *(*st).energy_mask.offset((nbEBands + i) as isize)
                }
            } else {
                unmask = *(*st).energy_mask.offset(i as isize)
            }
            unmask = if unmask < 0.0 { unmask } else { 0.0 };
            unmask -= lin;
            if unmask > 0.25 {
                *surround_dynalloc.offset(i as isize) = unmask - 0.25;
                count_dynalloc += 1
            }
            i += 1
        }
        if count_dynalloc >= 3 {
            /* If we need dynalloc in many bands, it's probably because our
            initial masking rate was too low. */
            mask_avg += 0.25;
            if mask_avg > 0f32 {
                /* Something went really wrong in the original calculations,
                disabling masking. */
                mask_avg = 0f32;
                diff = 0f32;
                crate::stdlib::memset(
                    surround_dynalloc as *mut libc::c_void,
                    0i32,
                    (mask_end as usize)
                        .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
                );
            } else {
                i = 0;
                while i < mask_end {
                    *surround_dynalloc.offset(i as isize) =
                        if 0f32 > *surround_dynalloc.offset(i as isize) - 0.25 {
                            0f32
                        } else {
                            (*surround_dynalloc.offset(i as isize)) - 0.25
                        };
                    i += 1
                }
            }
        }
        mask_avg += 0.2;
        /* Convert to 1/64th units used for the trim */
        surround_trim = 64f32 * diff;
        /*printf("%d %d ", mask_avg, surround_trim);*/
        surround_masking = mask_avg
    }
    /* Temporal VBR (but not for LFE) */
    if (*st).lfe == 0 {
        let mut follow: crate::arch_h::opus_val16 = -10.0;
        let mut frame_avg: crate::arch_h::opus_val32 = 0f32;
        let mut offset: crate::arch_h::opus_val16 = if shortBlocks != 0 {
            (0.5) * LM as f32
        } else {
            0f32
        };
        i = start;
        while i < end {
            follow = if follow - 1.0 > *bandLogE.offset(i as isize) - offset {
                (follow) - 1.0
            } else {
                (*bandLogE.offset(i as isize)) - offset
            };
            if C == 2 {
                follow = if follow > *bandLogE.offset((i + nbEBands) as isize) - offset {
                    follow
                } else {
                    (*bandLogE.offset((i + nbEBands) as isize)) - offset
                }
            }
            frame_avg += follow;
            i += 1
        }
        frame_avg /= (end - start) as f32;
        temporal_vbr = frame_avg - (*st).spec_avg;
        temporal_vbr = if 3.0
            < (if -1.5 > temporal_vbr {
                -1.5
            } else {
                temporal_vbr
            }) {
            3.0
        } else if -1.5 > temporal_vbr {
            -1.5
        } else {
            temporal_vbr
        };
        (*st).spec_avg += 0.02 * temporal_vbr
    }
    /*for (i=0;i<21;i++)
       printf("%f ", bandLogE[i]);
    printf("\n");*/
    if secondMdct == 0 {
        crate::stdlib::memcpy(
            bandLogE2 as *mut libc::c_void,
            bandLogE as *const libc::c_void,
            ((C * nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0isize * bandLogE2.wrapping_offset_from(bandLogE)) as usize),
        );
    }
    /* Last chance to catch any transient we might have missed in the
    time-domain analysis */
    if LM > 0
        && ec_tell(enc) + 3 <= total_bits
        && isTransient == 0
        && (*st).complexity >= 5
        && (*st).lfe == 0
        && hybrid == 0
    {
        if patch_transient_decision(bandLogE, oldBandE, nbEBands, start, end, C) != 0 {
            isTransient = 1;
            shortBlocks = M;
            compute_mdcts(
                mode,
                shortBlocks,
                in_0,
                freq,
                C,
                CC,
                LM,
                (*st).upsample,
                (*st).arch,
            );
            crate::src::opus_1_2_1::celt::bands::compute_band_energies(
                mode,
                freq,
                bandE,
                effEnd,
                C,
                LM,
                (*st).arch,
            );
            crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(
                mode, effEnd, end, bandE, bandLogE, C,
            );
            /* Compensate for the scaling of short vs long mdcts */
            i = 0; /* *< Interleaved normalised MDCTs */
            while i < C * nbEBands {
                let ref mut fresh19 = *bandLogE2.offset(i as isize);
                *fresh19 += 0.5 * LM as f32;
                i += 1
            }
            tf_estimate = 0.2
        }
    }
    if LM > 0 && ec_tell(enc) + 3 <= total_bits {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, isTransient, 3u32);
    }
    let mut fresh20 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>()).wrapping_mul((C * N) as usize),
    );
    X = fresh20.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    /* Band normalisation */
    crate::src::opus_1_2_1::celt::bands::normalise_bands(mode, freq, X, bandE, effEnd, C, M);
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    tf_res = fresh21.as_mut_ptr();
    /* Disable variable tf resolution for hybrid and at very low bitrate */
    if effectiveBytes >= 15 * C && hybrid == 0 && (*st).complexity >= 2 && (*st).lfe == 0 {
        let mut lambda: i32 = 0;
        lambda = if 5 > 1280 / effectiveBytes + 2 {
            5
        } else {
            (1280 / effectiveBytes) + 2
        };
        tf_select = tf_analysis(
            mode,
            effEnd,
            isTransient,
            tf_res,
            lambda,
            X,
            N,
            LM,
            tf_estimate,
            tf_chan,
        );
        i = effEnd;
        while i < end {
            *tf_res.offset(i as isize) = *tf_res.offset((effEnd - 1) as isize);
            i += 1
        }
    } else if hybrid != 0 && weak_transient != 0 {
        /* For weak transients, we rely on the fact that improving time resolution using
        TF on a long window is imperfect and will not result in an energy collapse at
        low bitrate. */
        i = 0;
        while i < end {
            *tf_res.offset(i as isize) = 1;
            i += 1
        }
        tf_select = 0
    } else if hybrid != 0 && effectiveBytes < 15 {
        /* For low bitrate hybrid, we force temporal resolution to 5 ms rather than 2.5 ms. */
        i = 0;
        while i < end {
            *tf_res.offset(i as isize) = 0;
            i += 1
        }
        tf_select = isTransient
    } else {
        i = 0;
        while i < end {
            *tf_res.offset(i as isize) = isTransient;
            i += 1
        }
        tf_select = 0
    }
    let mut fresh22 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((C * nbEBands) as usize),
    );
    error = fresh22.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    c = 0;
    loop {
        i = start;
        while i < end {
            /* When the energy is stable, slightly bias energy quantization towards
            the previous error to make the gain more stable (a constant offset is
            better than fluctuations). */
            if (crate::stdlib::fabs(
                (*bandLogE.offset((i + c * nbEBands) as isize)
                    - *oldBandE.offset((i + c * nbEBands) as isize)) as f64,
            ) as f32)
                < 2.0
            {
                let ref mut fresh23 = *bandLogE.offset((i + c * nbEBands) as isize);
                *fresh23 -= *energyError.offset((i + c * nbEBands) as isize) * 0.25f32
            }
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    crate::src::opus_1_2_1::celt::quant_bands::quant_coarse_energy(
        mode,
        start,
        end,
        effEnd,
        bandLogE,
        oldBandE,
        total_bits as crate::opus_types_h::opus_uint32,
        error,
        enc,
        C,
        LM,
        nbAvailableBytes,
        (*st).force_intra,
        &mut (*st).delayedIntra,
        ((*st).complexity >= 4) as i32,
        (*st).loss_rate,
        (*st).lfe,
    );
    tf_encode(start, end, isTransient, tf_res, LM, tf_select, enc);
    if ec_tell(enc) + 4 <= total_bits {
        if (*st).lfe != 0 {
            (*st).tapset_decision = 0;
            (*st).spread_decision = 2
        } else if hybrid != 0 {
            if (*st).complexity == 0 {
                (*st).spread_decision = 0
            } else if isTransient != 0 {
                (*st).spread_decision = 2
            } else {
                (*st).spread_decision = 3
            }
        } else if shortBlocks != 0 || (*st).complexity < 3 || nbAvailableBytes < 10 * C {
            if (*st).complexity == 0 {
                (*st).spread_decision = 0
            } else {
                (*st).spread_decision = 2
            }
        } else {
            /* Disable new spreading+tapset estimator until we can show it works
            better than the old one. So far it seems like spreading_decision()
            works best. */
            (*st).spread_decision = crate::src::opus_1_2_1::celt::bands::spreading_decision(
                mode,
                X,
                &mut (*st).tonal_average,
                (*st).spread_decision,
                &mut (*st).hf_average,
                &mut (*st).tapset_decision,
                (pf_on != 0 && shortBlocks == 0) as i32,
                effEnd,
                C,
                M,
            )
            /*printf("%d %d\n", st->tapset_decision, st->spread_decision);*/
            /*printf("%f %d %f %d\n\n", st->analysis.tonality, st->spread_decision, st->analysis.tonality_slope, st->tapset_decision);*/
        }
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
            enc,
            (*st).spread_decision,
            spread_icdf.as_ptr(),
            5u32,
        );
    }
    let mut fresh24 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    offsets = fresh24.as_mut_ptr();
    maxDepth = dynalloc_analysis(
        bandLogE,
        bandLogE2,
        nbEBands,
        start,
        end,
        C,
        offsets,
        (*st).lsb_depth,
        (*mode).logN,
        isTransient,
        (*st).vbr,
        (*st).constrained_vbr,
        eBands,
        LM,
        effectiveBytes,
        &mut tot_boost,
        (*st).lfe,
        surround_dynalloc,
        &mut (*st).analysis,
    );
    /* For LFE, everything interesting is in the first band */
    if (*st).lfe != 0 {
        *offsets.offset(0) = if (8) < effectiveBytes / 3 {
            8
        } else {
            (effectiveBytes) / 3
        }
    }
    let mut fresh25 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    cap = fresh25.as_mut_ptr();
    crate::src::opus_1_2_1::celt::celt::init_caps(mode, cap, LM, C);
    dynalloc_logp = 6;
    total_bits <<= 3;
    total_boost = 0;
    tell =
        crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc) as crate::opus_types_h::opus_int32;
    i = start;
    while i < end {
        let mut width: i32 = 0;
        let mut quanta: i32 = 0;
        let mut dynalloc_loop_logp: i32 = 0;
        let mut boost: i32 = 0;
        let mut j: i32 = 0;
        width = (C * (*eBands.offset((i + 1) as isize) as i32 - *eBands.offset(i as isize) as i32))
            << LM;
        /* quanta is 6 bits, but no more than 1 bit/sample
        and no less than 1/8 bit/sample */
        quanta = if (width << 3) < (if (6) << 3 > width { (6) << 3 } else { width }) {
            (width) << 3
        } else if (6) << 3 > width {
            (6) << 3
        } else {
            width
        };
        dynalloc_loop_logp = dynalloc_logp;
        boost = 0;
        j = 0;
        while (tell + (dynalloc_loop_logp << 3)) < total_bits - total_boost
            && boost < *cap.offset(i as isize)
        {
            let mut flag: i32 = 0;
            flag = (j < *offsets.offset(i as isize)) as i32;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                enc,
                flag,
                dynalloc_loop_logp as u32,
            );
            tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc)
                as crate::opus_types_h::opus_int32;
            if flag == 0 {
                break;
            }
            boost += quanta;
            total_boost += quanta;
            dynalloc_loop_logp = 1;
            j += 1
        }
        /* Making dynalloc more likely */
        if j != 0 {
            dynalloc_logp = if 2 > dynalloc_logp - 1 {
                2
            } else {
                (dynalloc_logp) - 1
            }
        }
        *offsets.offset(i as isize) = boost;
        i += 1
    }
    if C == 2 {
        static mut intensity_thresholds: [crate::arch_h::opus_val16; 21] = [
            1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 16f32, 24f32, 36f32, 44f32, 50f32,
            56f32, 62f32, 67f32, 72f32, 79f32, 88f32, 106f32, 134f32,
        ];
        static mut intensity_histeresis: [crate::arch_h::opus_val16; 21] = [
            1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 2f32, 2f32, 2f32, 2f32, 2f32, 2f32, 2f32,
            3f32, 3f32, 4f32, 5f32, 6f32, 8f32, 8f32,
        ];
        /* Always use MS for 2.5 ms frames until we can do a better analysis */
        if LM != 0 {
            dual_stereo = stereo_analysis(mode, X, LM, N)
        }
        (*st).intensity = crate::src::opus_1_2_1::celt::bands::hysteresis_decision(
            (equiv_rate / 1000) as crate::arch_h::opus_val16,
            intensity_thresholds.as_ptr(),
            intensity_histeresis.as_ptr(),
            21,
            (*st).intensity,
        );
        (*st).intensity = if end
            < (if start > (*st).intensity {
                start
            } else {
                (*st).intensity
            }) {
            end
        } else if start > (*st).intensity {
            start
        } else {
            (*st).intensity
        }
    }
    alloc_trim = 5;
    if tell + ((6) << 3) <= total_bits - total_boost {
        if start > 0 || (*st).lfe != 0 {
            (*st).stereo_saving = 0f32;
            alloc_trim = 5
        } else {
            alloc_trim = alloc_trim_analysis(
                mode,
                X,
                bandLogE,
                end,
                LM,
                C,
                N,
                &mut (*st).analysis,
                &mut (*st).stereo_saving,
                tf_estimate,
                (*st).intensity,
                surround_trim,
                equiv_rate,
                (*st).arch,
            )
        }
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(enc, alloc_trim, trim_icdf.as_ptr(), 7);
        tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc)
            as crate::opus_types_h::opus_int32
    }
    /* Variable bitrate */
    if vbr_rate > 0 {
        let mut alpha: crate::arch_h::opus_val16 = 0.;
        let mut delta: crate::opus_types_h::opus_int32 = 0;
        /* The target rate in 8th bits per frame */
        let mut target: crate::opus_types_h::opus_int32 = 0;
        let mut base_target: crate::opus_types_h::opus_int32 = 0;
        let mut min_allowed: crate::opus_types_h::opus_int32 = 0;
        let mut lm_diff: i32 = (*mode).maxLM - LM;
        /* Don't attempt to use more than 510 kb/s, even for frames smaller than 20 ms.
        The CELT allocator will just not be able to use more than that anyway. */
        nbCompressedBytes = if nbCompressedBytes < 1275 >> 3 - LM {
            nbCompressedBytes
        } else {
            (1275) >> 3 - LM
        };
        if hybrid == 0 {
            base_target = vbr_rate - ((40 * C + 20) << 3)
        } else {
            base_target = if 0 > vbr_rate - ((9 * C + 4) << 3) {
                0
            } else {
                (vbr_rate) - ((9 * C + 4) << 3)
            }
        }
        if (*st).constrained_vbr != 0 {
            base_target += (*st).vbr_offset >> lm_diff
        }
        if hybrid == 0 {
            target = compute_vbr(
                mode,
                &mut (*st).analysis,
                base_target,
                LM,
                equiv_rate,
                (*st).lastCodedBands,
                C,
                (*st).intensity,
                (*st).constrained_vbr,
                (*st).stereo_saving,
                tot_boost,
                tf_estimate,
                pitch_change,
                maxDepth,
                (*st).lfe,
                ((*st).energy_mask != 0 as *mut crate::arch_h::opus_val16) as i32,
                surround_masking,
                temporal_vbr,
            )
        } else {
            target = base_target;
            /* Tonal frames (offset<100) need more bits than noisy (offset>100) ones. */
            if (*st).silk_info.offset < 100 {
                target += (12) << 3 >> 3 - LM
            }
            if (*st).silk_info.offset > 100 {
                target -= (18) << 3 >> 3 - LM
            }
            /* Boosting bitrate on transients and vowels with significant temporal
            spikes. */
            target +=
                ((tf_estimate - 0.25) * ((50i32) << 3) as f32) as crate::opus_types_h::opus_int32;
            /* If we have a strong transient, let's make sure it has enough bits to code
            the first two bands, so that it can use folding rather than noise. */
            if tf_estimate > 0.7 {
                target = if target > (50) << 3 {
                    target
                } else {
                    (50) << 3
                }
            }
        }
        /* The current offset is removed from the target and the space used
        so far is added*/
        target = target + tell;
        /* In VBR mode the frame size must not be reduced so much that it would
         result in the encoder running out of bits.
        The margin of 2 bytes ensures that none of the bust-prevention logic
         in the decoder will have triggered so far. */
        min_allowed = (tell + total_boost + ((1) << 3 + 3) - 1 >> 3 + 3) + 2;
        /* Take into account the 37 bits we need to have left in the packet to
        signal a redundant frame in hybrid mode. Creating a shorter packet would
        create an entropy coder desync. */
        if hybrid != 0 {
            min_allowed = if min_allowed
                > tell0_frac + ((37) << 3) + total_boost + ((1) << 3 + 3) - 1 >> 3 + 3
            {
                min_allowed
            } else {
                (tell0_frac + ((37) << 3) + total_boost + ((1) << 3 + 3) - 1) >> 3 + 3
            }
        }
        nbAvailableBytes = target + ((1) << 3 + 2) >> 3 + 3;
        nbAvailableBytes = if min_allowed > nbAvailableBytes {
            min_allowed
        } else {
            nbAvailableBytes
        };
        nbAvailableBytes = if nbCompressedBytes < nbAvailableBytes {
            nbCompressedBytes
        } else {
            nbAvailableBytes
        };
        /* By how much did we "miss" the target on that frame */
        delta = target - vbr_rate;
        target = nbAvailableBytes << 3 + 3;
        /*If the frame is silent we don't adjust our drift, otherwise
        the encoder will shoot to very high rates after hitting a
        span of silence, but we do allow the bitres to refill.
        This means that we'll undershoot our target in CVBR/VBR modes
        on files with lots of silence. */
        if silence != 0 {
            nbAvailableBytes = 2;
            target = (2 * 8) << 3;
            delta = 0
        }
        if (*st).vbr_count < 970 {
            (*st).vbr_count += 1;
            alpha = 1.0 / ((*st).vbr_count + 20i32) as f32
        } else {
            alpha = 0.001
        }
        /* How many bits have we used in excess of what we're allowed */
        if (*st).constrained_vbr != 0 {
            (*st).vbr_reservoir += target - vbr_rate
        }
        /*printf ("%d\n", st->vbr_reservoir);*/
        /* Compute the offset we need to apply in order to reach the target */
        if (*st).constrained_vbr != 0 {
            (*st).vbr_drift += (alpha
                * (delta * ((1) << lm_diff) - (*st).vbr_offset - (*st).vbr_drift) as f32)
                as crate::opus_types_h::opus_int32;
            (*st).vbr_offset = -(*st).vbr_drift
        }
        /*printf ("%d\n", st->vbr_drift);*/
        if (*st).constrained_vbr != 0 && (*st).vbr_reservoir < 0 {
            /* We're under the min value -- increase rate */
            let mut adjust: i32 = -(*st).vbr_reservoir / ((8) << 3);
            /* Unless we're just coding silence */
            nbAvailableBytes += if silence != 0 { 0 } else { adjust };
            (*st).vbr_reservoir = 0
        }
        nbCompressedBytes = if nbCompressedBytes < nbAvailableBytes {
            nbCompressedBytes
        } else {
            nbAvailableBytes
        };
        /*printf("%d\n", nbCompressedBytes*50*8);*/
        /* This moves the raw bits to take into account the new compressed size */
        crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
            enc,
            nbCompressedBytes as crate::opus_types_h::opus_uint32,
        );
    }
    /* Bit allocation */
    let mut fresh26 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    fine_quant = fresh26.as_mut_ptr();
    let mut fresh27 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    pulses = fresh27.as_mut_ptr();
    let mut fresh28 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    fine_priority = fresh28.as_mut_ptr();
    /* bits =           packet size                    - where we are - safety*/
    bits = (((nbCompressedBytes * 8i32) << 3) as u32)
        .wrapping_sub(crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc))
        .wrapping_sub(1u32) as crate::opus_types_h::opus_int32;
    anti_collapse_rsv = if isTransient != 0 && LM >= 2 && bits >= (LM + 2) << 3 {
        (1) << 3
    } else {
        0
    };
    bits -= anti_collapse_rsv;
    signalBandwidth = end - 1;
    if (*st).analysis.valid != 0 {
        let mut min_bandwidth: i32 = 0;
        if equiv_rate < 32000 * C {
            min_bandwidth = 13
        } else if equiv_rate < 48000 * C {
            min_bandwidth = 16
        } else if equiv_rate < 60000 * C {
            min_bandwidth = 18
        } else if equiv_rate < 80000 * C {
            min_bandwidth = 19
        } else {
            min_bandwidth = 20
        }
        signalBandwidth = if (*st).analysis.bandwidth > min_bandwidth {
            (*st).analysis.bandwidth
        } else {
            min_bandwidth
        }
    }
    if (*st).lfe != 0 {
        signalBandwidth = 1
    }
    codedBands = crate::src::opus_1_2_1::celt::rate::compute_allocation(
        mode,
        start,
        end,
        offsets,
        cap,
        alloc_trim,
        &mut (*st).intensity,
        &mut dual_stereo,
        bits,
        &mut balance,
        pulses,
        fine_quant,
        fine_priority,
        C,
        LM,
        enc,
        1,
        (*st).lastCodedBands,
        signalBandwidth,
    );
    if (*st).lastCodedBands != 0 {
        (*st).lastCodedBands = if ((*st).lastCodedBands + 1)
            < (if (*st).lastCodedBands - 1 > codedBands {
                ((*st).lastCodedBands) - 1
            } else {
                codedBands
            }) {
            ((*st).lastCodedBands) + 1
        } else if (*st).lastCodedBands - 1 > codedBands {
            ((*st).lastCodedBands) - 1
        } else {
            codedBands
        }
    } else {
        (*st).lastCodedBands = codedBands
    }
    crate::src::opus_1_2_1::celt::quant_bands::quant_fine_energy(
        mode, start, end, oldBandE, error, fine_quant, enc, C,
    );
    /* Residual quantisation */
    let mut fresh29 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<u8>()).wrapping_mul((C * nbEBands) as usize),
    );
    collapse_masks = fresh29.as_mut_ptr() as *mut u8;
    crate::src::opus_1_2_1::celt::bands::quant_all_bands(
        1,
        mode,
        start,
        end,
        X,
        if C == 2 {
            X.offset(N as isize)
        } else {
            0 as *mut crate::arch_h::celt_norm
        },
        collapse_masks,
        bandE,
        pulses,
        shortBlocks,
        (*st).spread_decision,
        dual_stereo,
        (*st).intensity,
        tf_res,
        nbCompressedBytes * ((8) << 3) - anti_collapse_rsv,
        balance,
        enc,
        LM,
        codedBands,
        &mut (*st).rng,
        (*st).complexity,
        (*st).arch,
        (*st).disable_inv,
    );
    if anti_collapse_rsv > 0 {
        anti_collapse_on = ((*st).consec_transient < 2) as i32;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
            enc,
            anti_collapse_on as crate::opus_types_h::opus_uint32,
            1u32,
        );
    }
    crate::src::opus_1_2_1::celt::quant_bands::quant_energy_finalise(
        mode,
        start,
        end,
        oldBandE,
        error,
        fine_quant,
        fine_priority,
        nbCompressedBytes * 8 - ec_tell(enc),
        enc,
        C,
    );
    crate::stdlib::memset(
        energyError as *mut libc::c_void,
        0,
        ((nbEBands * CC) as usize).wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
    );
    c = 0;
    loop {
        i = start;
        while i < end {
            *energyError.offset((i + c * nbEBands) as isize) = if -0.5f32
                > (if 0.5f32 < *error.offset((i + c * nbEBands) as isize) {
                    0.5
                } else {
                    *error.offset((i + c * nbEBands) as isize)
                }) {
                -0.5f32
            } else if 0.5f32 < *error.offset((i + c * nbEBands) as isize) {
                0.5
            } else {
                *error.offset((i + c * nbEBands) as isize)
            };
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    if silence != 0 {
        i = 0;
        while i < C * nbEBands {
            *oldBandE.offset(i as isize) = -28.0f32;
            i += 1
        }
    }
    (*st).prefilter_period = pitch_index;
    (*st).prefilter_gain = gain1;
    (*st).prefilter_tapset = prefilter_tapset;
    if CC == 2 && C == 1 {
        crate::stdlib::memcpy(
            &mut *oldBandE.offset(nbEBands as isize) as *mut crate::arch_h::opus_val16
                as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            (nbEBands as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add(
                    (0isize
                        * (&mut *oldBandE.offset(nbEBands as isize)
                            as *mut crate::arch_h::opus_val16)
                            .wrapping_offset_from(oldBandE)) as usize,
                ),
        );
    }
    if isTransient == 0 {
        crate::stdlib::memcpy(
            oldLogE2 as *mut libc::c_void,
            oldLogE as *const libc::c_void,
            ((CC * nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0 * oldLogE2.wrapping_offset_from(oldLogE)) as usize),
        );
        crate::stdlib::memcpy(
            oldLogE as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            ((CC * nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0isize * oldLogE.wrapping_offset_from(oldBandE)) as usize),
        );
    } else {
        i = 0;
        while i < CC * nbEBands {
            *oldLogE.offset(i as isize) =
                if *oldLogE.offset(i as isize) < *oldBandE.offset(i as isize) {
                    *oldLogE.offset(i as isize)
                } else {
                    *oldBandE.offset(i as isize)
                };
            i += 1
        }
    }
    /* In case start or end were to change */
    c = 0;
    loop {
        i = 0;
        while i < start {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0f32;
            let ref mut fresh30 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh30 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh30;
            i += 1
        }
        i = end;
        while i < nbEBands {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0f32;
            let ref mut fresh31 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh31 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh31;
            i += 1
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if isTransient != 0 || transient_got_disabled != 0 {
        (*st).consec_transient += 1
    } else {
        (*st).consec_transient = 0
    }
    (*st).rng = (*enc).rng;
    /* If there's any room left (can only happen for very high rates),
    it's already filled with zeros */
    crate::src::opus_1_2_1::celt::entenc::ec_enc_done(enc);
    if ec_get_error(enc) != 0 {
        return -(3i32);
    } else {
        return nbCompressedBytes;
    };
}
/* CUSTOM_MODES */
#[no_mangle]

pub unsafe extern "C" fn opus_custom_encoder_ctl(
    mut st: *mut OpusCustomEncoder,
    mut request: i32,
    mut args: ...
) -> i32 {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    match request {
        4010 => {
            let mut value: i32 = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value < 0 || value > 10 {
                current_block = 796174441944384681;
            } else {
                (*st).complexity = value;
                current_block = 4488496028633655612;
            }
        }
        10010 => {
            let mut value_0: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_0 < 0 || value_0 >= (*(*st).mode).nbEBands {
                current_block = 796174441944384681;
            } else {
                (*st).start = value_0;
                current_block = 4488496028633655612;
            }
        }
        10012 => {
            let mut value_1: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_1 < 1 || value_1 > (*(*st).mode).nbEBands {
                current_block = 796174441944384681;
            } else {
                (*st).end = value_1;
                current_block = 4488496028633655612;
            }
        }
        10002 => {
            let mut value_2: i32 = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_2 < 0 || value_2 > 2 {
                current_block = 796174441944384681;
            } else {
                (*st).disable_pf = (value_2 <= 1) as i32;
                (*st).force_intra = (value_2 == 0) as i32;
                current_block = 4488496028633655612;
            }
        }
        4014 => {
            let mut value_3: i32 = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_3 < 0 || value_3 > 100 {
                current_block = 796174441944384681;
            } else {
                (*st).loss_rate = value_3;
                current_block = 4488496028633655612;
            }
        }
        4020 => {
            let mut value_4: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).constrained_vbr = value_4;
            current_block = 4488496028633655612;
        }
        4006 => {
            let mut value_5: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).vbr = value_5;
            current_block = 4488496028633655612;
        }
        4002 => {
            let mut value_6: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_6 <= 500 && value_6 != -(1) {
                current_block = 796174441944384681;
            } else {
                value_6 = if value_6 < 260000 * (*st).channels {
                    value_6
                } else {
                    (260000) * (*st).channels
                };
                (*st).bitrate = value_6;
                current_block = 4488496028633655612;
            }
        }
        10008 => {
            let mut value_7: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_7 < 1 || value_7 > 2 {
                current_block = 796174441944384681;
            } else {
                (*st).stream_channels = value_7;
                current_block = 4488496028633655612;
            }
        }
        4036 => {
            let mut value_8: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_8 < 8 || value_8 > 24 {
                current_block = 796174441944384681;
            } else {
                (*st).lsb_depth = value_8;
                current_block = 4488496028633655612;
            }
        }
        4037 => {
            let mut value_9: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            *value_9 = (*st).lsb_depth;
            current_block = 4488496028633655612;
        }
        4046 => {
            let mut value_10: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_10 < 0 || value_10 > 1 {
                current_block = 796174441944384681;
            } else {
                (*st).disable_inv = value_10;
                current_block = 4488496028633655612;
            }
        }
        4047 => {
            let mut value_11: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_11.is_null() {
                current_block = 796174441944384681;
            } else {
                *value_11 = (*st).disable_inv;
                current_block = 4488496028633655612;
            }
        }
        4028 => {
            let mut _i: i32 = 0;
            let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            oldBandE = (*st)
                .in_mem
                .as_mut_ptr()
                .offset(((*st).channels * ((*(*st).mode).overlap + 1024i32)) as isize);
            oldLogE = oldBandE.offset(((*st).channels * (*(*st).mode).nbEBands) as isize);
            oldLogE2 = oldLogE.offset(((*st).channels * (*(*st).mode).nbEBands) as isize);
            crate::stdlib::memset(
                &mut (*st).rng as *mut crate::opus_types_h::opus_uint32 as *mut libc::c_void,
                0,
                ((opus_custom_encoder_get_size((*st).mode, (*st).channels) as isize
                    - (&mut (*st).rng as *mut crate::opus_types_h::opus_uint32 as *mut i8)
                        .wrapping_offset_from(st as *mut i8)) as usize)
                    .wrapping_mul(::std::mem::size_of::<i8>()),
            );

            for i in 0..(*st).channels * (*(*st).mode).nbEBands {
                let ref mut fresh32 = *oldLogE2.offset(i as isize);

                *fresh32 = -28.0f32;

                *oldLogE.offset(i as isize) = *fresh32;
            }
            (*st).vbr_offset = 0;
            (*st).delayedIntra = 1f32;
            (*st).spread_decision = 2;
            (*st).tonal_average = 256;
            (*st).hf_average = 0;
            (*st).tapset_decision = 0;
            current_block = 4488496028633655612;
        }
        10016 => {
            let mut value_12: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).signalling = value_12;
            current_block = 4488496028633655612;
        }
        10022 => {
            let mut info: *mut crate::celt_h::AnalysisInfo =
                ap.as_va_list().arg::<*mut crate::celt_h::AnalysisInfo>();
            if !info.is_null() {
                crate::stdlib::memcpy(
                    &mut (*st).analysis as *mut crate::celt_h::AnalysisInfo as *mut libc::c_void,
                    info as *const libc::c_void,
                    (1usize)
                        .wrapping_mul(::std::mem::size_of::<crate::celt_h::AnalysisInfo>())
                        .wrapping_add(
                            (0isize
                                * (&mut (*st).analysis as *mut crate::celt_h::AnalysisInfo)
                                    .wrapping_offset_from(info))
                                as usize,
                        ),
                );
            }
            current_block = 4488496028633655612;
        }
        10028 => {
            let mut info_0: *mut crate::celt_h::SILKInfo =
                ap.as_va_list().arg::<*mut crate::celt_h::SILKInfo>();
            if !info_0.is_null() {
                crate::stdlib::memcpy(
                    &mut (*st).silk_info as *mut crate::celt_h::SILKInfo as *mut libc::c_void,
                    info_0 as *const libc::c_void,
                    (1usize)
                        .wrapping_mul(::std::mem::size_of::<crate::celt_h::SILKInfo>())
                        .wrapping_add(
                            (0isize
                                * (&mut (*st).silk_info as *mut crate::celt_h::SILKInfo)
                                    .wrapping_offset_from(info_0))
                                as usize,
                        ),
                );
            }
            current_block = 4488496028633655612;
        }
        10015 => {
            let mut value_13: *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode = ap
                .as_va_list()
                .arg::<*mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode>();
            if value_13.is_null() {
                current_block = 796174441944384681;
            } else {
                *value_13 = (*st).mode;
                current_block = 4488496028633655612;
            }
        }
        4031 => {
            let mut value_14: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            if value_14.is_null() {
                current_block = 796174441944384681;
            } else {
                *value_14 = (*st).rng;
                current_block = 4488496028633655612;
            }
        }
        10024 => {
            let mut value_15: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).lfe = value_15;
            current_block = 4488496028633655612;
        }
        10026 => {
            let mut value_16: *mut crate::arch_h::opus_val16 =
                ap.as_va_list().arg::<*mut crate::arch_h::opus_val16>();
            (*st).energy_mask = value_16;
            current_block = 4488496028633655612;
        }
        _ => return -(5),
    }
    match current_block {
        4488496028633655612 => return 0,
        _ => return -(1),
    };
}
