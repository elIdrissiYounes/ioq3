// =============== BEGIN analysis_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TonalityAnalysisState {
    pub arch: i32,
    pub application: i32,
    pub Fs: crate::opus_types_h::opus_int32,
    pub angle: [f32; 240],
    pub d_angle: [f32; 240],
    pub d2_angle: [f32; 240],
    pub inmem: [crate::arch_h::opus_val32; 720],
    pub mem_fill: i32,
    pub prev_band_tonality: [f32; 18],
    pub prev_tonality: f32,
    pub prev_bandwidth: i32,
    pub E: [[f32; 18]; 8],
    pub logE: [[f32; 18]; 8],
    pub lowE: [f32; 18],
    pub highE: [f32; 18],
    pub meanE: [f32; 19],
    pub mem: [f32; 32],
    pub cmean: [f32; 8],
    pub std: [f32; 9],
    pub music_prob: f32,
    pub vad_prob: f32,
    pub Etracker: f32,
    pub lowECount: f32,
    pub E_count: i32,
    pub last_music: i32,
    pub count: i32,
    pub analysis_offset: i32,
    pub pspeech: [f32; 100],
    pub pmusic: [f32; 100],
    pub speech_confidence: f32,
    pub music_confidence: f32,
    pub speech_confidence_count: i32,
    pub music_confidence_count: i32,
    pub write_pos: i32,
    pub read_pos: i32,
    pub read_subframe: i32,
    pub hp_ener_accum: f32,
    pub downmix_state: [crate::arch_h::opus_val32; 3],
    pub info: [crate::celt_h::AnalysisInfo; 100],
}
use ::libc;

pub mod arch_h {

    /* Copyright (c) 2003-2008 Jean-Marc Valin
    Copyright (c) 2007-2008 CSIRO
    Copyright (c) 2007-2009 Xiph.Org Foundation
    Written by Jean-Marc Valin */
    /* *
       @file arch.h
       @brief Various architecture definitions for CELT
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
    /* *< Minimum 16-bit value.   */
    /* *< Maximum 16-bit value.   */
    /* *< Minimum 32-bit value.   */
    /* *< Maximum 32-bit value.   */
    /* *< Minimum int value.   */
    /* *< Maximum int value.   */
    /* Set this if opus_int64 is a native type of the CPU. */
    /* Assume that all LP64 architectures have fast 64-bit types; also x86_64
    (which can be ILP32 for x32) and Win64 (which is LLP64). */
    /* FIXED_POINT */
    /* This code should reliably detect NaN/inf even when -ffast-math is used.
    Assumes IEEE 754 format. */
    #[inline]

    pub unsafe extern "C" fn celt_isnan(mut x: f32) -> i32 {
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        return (in_0.i >> 23 & 0xff == 0xff && in_0.i & 0x7fffff != 0) as i32;
    }

    /* ARCH_H */
    /* !FIXED_POINT */
    /* This appears to be the same speed as C99's fabsf() but it's more portable. */
}

pub mod mathops_h {
    #[inline]

    pub unsafe extern "C" fn fast_atan2f(mut y: f32, mut x: f32) -> f32 {
        let mut x2: f32 = 0.;
        let mut y2: f32 = 0.;
        x2 = x * x;
        y2 = y * y;
        if x2 + y2 < 1e-18 {
            return 0f32;
        }
        if x2 < y2 {
            let mut den: f32 = (y2 + 0.67848403 * x2) * (y2 + 0.08595542 * x2);
            return -x * y * (y2 + 0.43157974f32 * x2) / den
                + (if y < 0f32 {
                    -(3.141592653f32 / 2f32)
                } else {
                    (3.141592653f32) / 2f32
                });
        } else {
            let mut den_0: f32 = (x2 + 0.67848403 * y2) * (x2 + 0.08595542 * y2);
            return x * y * (x2 + 0.43157974f32 * y2) / den_0
                + (if y < 0f32 {
                    -(3.141592653f32 / 2f32)
                } else {
                    (3.141592653f32) / 2f32
                })
                - (if x * y < 0f32 {
                    -(3.141592653f32 / 2f32)
                } else {
                    (3.141592653f32) / 2f32
                });
        };
    }
    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub mod cpu_support_h {
    /* Copyright (c) 2010 Xiph.Org Foundation
     * Copyright (c) 2013 Parrot */
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
    #[inline]

    pub unsafe extern "C" fn opus_select_arch() -> i32 {
        return 0;
    }
}

pub mod float_cast_h {
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: f32) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::arch_h::opus_val64;
pub use crate::celt_h::AnalysisInfo;
pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::opus_private_h::downmix_func;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::kiss_fft::opus_fft_c;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;
pub use crate::src::opus_1_2_1::src::analysis::arch_h::celt_isnan;

pub use crate::src::opus_1_2_1::src::analysis::cpu_support_h::opus_select_arch;
pub use crate::src::opus_1_2_1::src::analysis::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::src::analysis::mathops_h::fast_atan2f;
pub use crate::src::opus_1_2_1::src::mlp::mlp_process;
pub use crate::src::opus_1_2_1::src::mlp::MLP;
pub use crate::src::opus_1_2_1::src::mlp_data::net;
use crate::stdlib::fabs;
use crate::stdlib::floor;
use crate::stdlib::log;
use crate::stdlib::log10;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::pow;
use crate::stdlib::sqrt;
/* Copyright (c) 2011 Xiph.Org Foundation
Written by Jean-Marc Valin */
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
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

static mut dct_table: [f32; 128] = [
    0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.250000,
    0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.250000, 0.351851, 0.338330,
    0.311806, 0.273300, 0.224292, 0.166664, 0.102631, 0.034654, -0.034654, -0.102631, -0.166664,
    -0.224292, -0.273300, -0.311806, -0.338330, -0.351851, 0.346760, 0.293969, 0.196424, 0.068975,
    -0.068975, -0.196424, -0.293969, -0.346760, -0.346760, -0.293969, -0.196424, -0.068975,
    0.068975, 0.196424, 0.293969, 0.346760, 0.338330, 0.224292, 0.034654, -0.166664, -0.311806,
    -0.351851, -0.273300, -0.102631, 0.102631, 0.273300, 0.351851, 0.311806, 0.166664, -0.034654,
    -0.224292, -0.338330, 0.326641, 0.135299, -0.135299, -0.326641, -0.326641, -0.135299, 0.135299,
    0.326641, 0.326641, 0.135299, -0.135299, -0.326641, -0.326641, -0.135299, 0.135299, 0.326641,
    0.311806, 0.034654, -0.273300, -0.338330, -0.102631, 0.224292, 0.351851, 0.166664, -0.166664,
    -0.351851, -0.224292, 0.102631, 0.338330, 0.273300, -0.034654, -0.311806, 0.293969, -0.068975,
    -0.346760, -0.196424, 0.196424, 0.346760, 0.068975, -0.293969, -0.293969, 0.068975, 0.346760,
    0.196424, -0.196424, -0.346760, -0.068975, 0.293969, 0.273300, -0.166664, -0.338330, 0.034654,
    0.351851, 0.102631, -0.311806, -0.224292, 0.224292, 0.311806, -0.102631, -0.351851, -0.034654,
    0.338330, 0.166664, -0.273300,
];

static mut analysis_window: [f32; 240] = [
    0.000043, 0.000171, 0.000385, 0.000685, 0.001071, 0.001541, 0.002098, 0.002739, 0.003466,
    0.004278, 0.005174, 0.006156, 0.007222, 0.008373, 0.009607, 0.010926, 0.012329, 0.013815,
    0.015385, 0.017037, 0.018772, 0.020590, 0.022490, 0.024472, 0.026535, 0.028679, 0.030904,
    0.033210, 0.035595, 0.038060, 0.040604, 0.043227, 0.045928, 0.048707, 0.051564, 0.054497,
    0.057506, 0.060591, 0.063752, 0.066987, 0.070297, 0.073680, 0.077136, 0.080665, 0.084265,
    0.087937, 0.091679, 0.095492, 0.099373, 0.103323, 0.107342, 0.111427, 0.115579, 0.119797,
    0.124080, 0.128428, 0.132839, 0.137313, 0.141849, 0.146447, 0.151105, 0.155823, 0.160600,
    0.165435, 0.170327, 0.175276, 0.180280, 0.185340, 0.190453, 0.195619, 0.200838, 0.206107,
    0.211427, 0.216797, 0.222215, 0.227680, 0.233193, 0.238751, 0.244353, 0.250000, 0.255689,
    0.261421, 0.267193, 0.273005, 0.278856, 0.284744, 0.290670, 0.296632, 0.302628, 0.308658,
    0.314721, 0.320816, 0.326941, 0.333097, 0.339280, 0.345492, 0.351729, 0.357992, 0.364280,
    0.370590, 0.376923, 0.383277, 0.389651, 0.396044, 0.402455, 0.408882, 0.415325, 0.421783,
    0.428254, 0.434737, 0.441231, 0.447736, 0.454249, 0.460770, 0.467298, 0.473832, 0.480370,
    0.486912, 0.493455, 0.500000, 0.506545, 0.513088, 0.519630, 0.526168, 0.532702, 0.539230,
    0.545751, 0.552264, 0.558769, 0.565263, 0.571746, 0.578217, 0.584675, 0.591118, 0.597545,
    0.603956, 0.610349, 0.616723, 0.623077, 0.629410, 0.635720, 0.642008, 0.648271, 0.654508,
    0.660720, 0.666903, 0.673059, 0.679184, 0.685279, 0.691342, 0.697372, 0.703368, 0.709330,
    0.715256, 0.721144, 0.726995, 0.732807, 0.738579, 0.744311, 0.750000, 0.755647, 0.761249,
    0.766807, 0.772320, 0.777785, 0.783203, 0.788573, 0.793893, 0.799162, 0.804381, 0.809547,
    0.814660, 0.819720, 0.824724, 0.829673, 0.834565, 0.839400, 0.844177, 0.848895, 0.853553,
    0.858151, 0.862687, 0.867161, 0.871572, 0.875920, 0.880203, 0.884421, 0.888573, 0.892658,
    0.896677, 0.900627, 0.904508, 0.908321, 0.912063, 0.915735, 0.919335, 0.922864, 0.926320,
    0.929703, 0.933013, 0.936248, 0.939409, 0.942494, 0.945503, 0.948436, 0.951293, 0.954072,
    0.956773, 0.959396, 0.961940, 0.964405, 0.966790, 0.969096, 0.971321, 0.973465, 0.975528,
    0.977510, 0.979410, 0.981228, 0.982963, 0.984615, 0.986185, 0.987671, 0.989074, 0.990393,
    0.991627, 0.992778, 0.993844, 0.994826, 0.995722, 0.996534, 0.997261, 0.997902, 0.998459,
    0.998929, 0.999315, 0.999615, 0.999829, 0.999957, 1.000000,
];

static mut tbands: [i32; 19] = [
    4, 8, 12, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 136, 160, 192, 240,
];

unsafe extern "C" fn silk_resampler_down2_hp(
    mut S: *mut crate::arch_h::opus_val32,
    mut out: *mut crate::arch_h::opus_val32,
    mut in_0: *const crate::arch_h::opus_val32,
    mut inLen: i32,
) -> crate::arch_h::opus_val32
/* I    Number of input samples                                     */ {
    let mut k: i32 = 0;
    let mut len2: i32 = inLen / 2;
    let mut in32: crate::arch_h::opus_val32 = 0.;
    let mut out32: crate::arch_h::opus_val32 = 0.;
    let mut out32_hp: crate::arch_h::opus_val32 = 0.;
    let mut Y: crate::arch_h::opus_val32 = 0.;
    let mut X: crate::arch_h::opus_val32 = 0.;
    let mut hp_ener: crate::arch_h::opus_val64 = 0f32;
    /* Internal variables and state are in Q10 format */
    k = 0;
    while k < len2 {
        /* Convert to Q10 */
        in32 = *in_0.offset((2 * k) as isize);
        /* All-pass section for even input sample */
        Y = in32 - *S.offset(0);
        X = 0.6074371 * Y;
        out32 = *S.offset(0) + X;
        *S.offset(0) = in32 + X;
        out32_hp = out32;
        /* Convert to Q10 */
        in32 = *in_0.offset((2 * k + 1) as isize);
        /* All-pass section for odd input sample, and add to output of previous section */
        Y = in32 - *S.offset(1);
        X = 0.15063 * Y;
        out32 = out32 + *S.offset(1);
        out32 = out32 + X;
        *S.offset(1) = in32 + X;
        Y = -in32 - *S.offset(2);
        X = 0.15063 * Y;
        out32_hp = out32_hp + *S.offset(2);
        out32_hp = out32_hp + X;
        *S.offset(2) = -in32 + X;
        hp_ener += out32_hp * out32_hp;
        /* Add, convert back to int16 and store to output */
        *out.offset(k as isize) = 0.5 * out32;
        k += 1
    }
    return hp_ener;
}

unsafe extern "C" fn downmix_and_resample(
    mut downmix: crate::opus_private_h::downmix_func,
    mut _x: *const libc::c_void,
    mut y: *mut crate::arch_h::opus_val32,
    mut S: *mut crate::arch_h::opus_val32,
    mut subframe: i32,
    mut offset: i32,
    mut c1: i32,
    mut c2: i32,
    mut C: i32,
    mut Fs: i32,
) -> crate::arch_h::opus_val32 {
    let mut tmp: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    let mut scale: crate::arch_h::opus_val32 = 0.;
    let mut j: i32 = 0;
    let mut ret: crate::arch_h::opus_val32 = 0f32;
    if subframe == 0 {
        return 0f32;
    }
    if Fs == 48000 {
        subframe *= 2;
        offset *= 2
    } else if Fs == 16000 {
        subframe = subframe * 2 / 3;
        offset = offset * 2 / 3
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val32>()).wrapping_mul(subframe as usize),
    );
    tmp = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val32;
    downmix.expect("non-null function pointer")(_x, tmp, subframe, offset, c1, c2, C);
    scale = 1.0 / 32768f32;
    if c2 == -(2) {
        scale /= C as f32
    } else if c2 > -(1) {
        scale /= 2f32
    }
    j = 0;
    while j < subframe {
        let ref mut fresh1 = *tmp.offset(j as isize);
        *fresh1 *= scale;
        j += 1
    }
    if Fs == 48000 {
        ret = silk_resampler_down2_hp(S, y, tmp, subframe)
    } else if Fs == 24000 {
        crate::stdlib::memcpy(
            y as *mut libc::c_void,
            tmp as *const libc::c_void,
            (subframe as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>())
                .wrapping_add((0isize * y.wrapping_offset_from(tmp)) as usize),
        );
    } else if Fs == 16000 {
        let mut tmp3x: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::arch_h::opus_val32>())
                .wrapping_mul((3i32 * subframe) as usize),
        );
        tmp3x = fresh2.as_mut_ptr() as *mut crate::arch_h::opus_val32;
        /* Don't do this at home! This resampler is horrible and it's only (barely)
        usable for the purpose of the analysis because we don't care about all
        the aliasing between 8 kHz and 12 kHz. */
        j = 0;
        while j < subframe {
            *tmp3x.offset((3 * j) as isize) = *tmp.offset(j as isize);
            *tmp3x.offset((3 * j + 1) as isize) = *tmp.offset(j as isize);
            *tmp3x.offset((3 * j + 2) as isize) = *tmp.offset(j as isize);
            j += 1
        }
        silk_resampler_down2_hp(S, y, tmp3x, 3i32 * subframe);
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn tonality_analysis_init(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut Fs: crate::opus_types_h::opus_int32,
) {
    /* Initialize reusable fields. */
    (*tonal).arch = opus_select_arch();
    (*tonal).Fs = Fs;
    /* Clear remaining fields. */
    tonality_analysis_reset(tonal);
}
/* * Initialize a TonalityAnalysisState struct.
 *
 * This performs some possibly slow initialization steps which should
 * not be repeated every analysis step. No allocated memory is retained
 * by the state struct, so no cleanup call is required.
 */
/* * Reset a TonalityAnalysisState stuct.
 *
 * Call this when there's a discontinuity in the data.
 */
#[no_mangle]

pub unsafe extern "C" fn tonality_analysis_reset(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
) {
    /* Clear non-reusable fields. */
    let mut start: *mut i8 = &mut (*tonal).angle as *mut [f32; 240] as *mut i8;
    crate::stdlib::memset(
        start as *mut libc::c_void,
        0,
        (::std::mem::size_of::<crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState>())
            .wrapping_sub(start.wrapping_offset_from(tonal as *mut i8) as usize)
            .wrapping_mul(::std::mem::size_of::<i8>()),
    );
    (*tonal).music_confidence = 0.9f32;
    (*tonal).speech_confidence = 0.1f32;
}
#[no_mangle]

pub unsafe extern "C" fn tonality_get_info(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut info_out: *mut crate::celt_h::AnalysisInfo,
    mut len: i32,
) {
    let mut pos: i32 = 0;
    let mut curr_lookahead: i32 = 0;
    let mut psum: f32 = 0.;
    let mut tonality_max: f32 = 0.;
    let mut tonality_avg: f32 = 0.;
    let mut tonality_count: i32 = 0;
    let mut i: i32 = 0;
    pos = (*tonal).read_pos;
    curr_lookahead = (*tonal).write_pos - (*tonal).read_pos;
    if curr_lookahead < 0 {
        curr_lookahead += 100
    }
    /* On long frames, look at the second analysis window rather than the first. */
    if len > (*tonal).Fs / 50 && pos != (*tonal).write_pos {
        pos += 1;
        if pos == 100 {
            pos = 0
        }
    }
    if pos == (*tonal).write_pos {
        pos -= 1
    }
    if pos < 0 {
        pos = 100 - 1
    }
    crate::stdlib::memcpy(
        info_out as *mut libc::c_void,
        &mut *(*tonal).info.as_mut_ptr().offset(pos as isize) as *mut crate::celt_h::AnalysisInfo
            as *const libc::c_void,
        (1usize)
            .wrapping_mul(::std::mem::size_of::<crate::celt_h::AnalysisInfo>())
            .wrapping_add(
                (0 * info_out
                    .wrapping_offset_from(&mut *(*tonal).info.as_mut_ptr().offset(pos as isize)))
                    as usize,
            ),
    );
    tonality_avg = (*info_out).tonality;
    tonality_max = tonality_avg;
    tonality_count = 1;
    /* If possible, look ahead for a tone to compensate for the delay in the tone detector. */
    i = 0;
    while i < 3 {
        pos += 1;
        if pos == 100 {
            pos = 0
        }
        if pos == (*tonal).write_pos {
            break;
        }
        tonality_max = if tonality_max > (*tonal).info[pos as usize].tonality {
            tonality_max
        } else {
            (*tonal).info[pos as usize].tonality
        };
        tonality_avg += (*tonal).info[pos as usize].tonality;
        tonality_count += 1;
        i += 1
    }
    (*info_out).tonality = if tonality_avg / tonality_count as f32 > tonality_max - 0.2 {
        (tonality_avg) / tonality_count as f32
    } else {
        (tonality_max) - 0.2
    };
    (*tonal).read_subframe += len / ((*tonal).Fs / 400);
    while (*tonal).read_subframe >= 8 {
        (*tonal).read_subframe -= 8;
        (*tonal).read_pos += 1
    }
    if (*tonal).read_pos >= 100 {
        (*tonal).read_pos -= 100
    }
    /* The -1 is to compensate for the delay in the features themselves. */
    curr_lookahead = if curr_lookahead - 1 > 0 {
        (curr_lookahead) - 1
    } else {
        0
    };
    psum = 0f32;
    /* Summing the probability of transition patterns that involve music at
    time (DETECT_SIZE-curr_lookahead-1) */
    i = 0;
    while i < 100 - curr_lookahead {
        psum += (*tonal).pmusic[i as usize];
        i += 1
    }
    while i < 100 {
        psum += (*tonal).pspeech[i as usize];
        i += 1
    }
    psum = psum * (*tonal).music_confidence + (1f32 - psum) * (*tonal).speech_confidence;
    /*printf("%f %f %f %f %f\n", psum, info_out->music_prob, info_out->vad_prob, info_out->activity_probability, info_out->tonality);*/
    (*info_out).music_prob = psum;
}

static mut std_feature_bias: [f32; 9] = [
    5.684947, 3.475288, 1.770634, 1.599784, 3.773215, 2.163313, 1.260756, 1.116868, 1.918795,
];

unsafe extern "C" fn tonality_analysis(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut celt_mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut x: *const libc::c_void,
    mut len: i32,
    mut offset: i32,
    mut c1: i32,
    mut c2: i32,
    mut C: i32,
    mut lsb_depth: i32,
    mut downmix: crate::opus_private_h::downmix_func,
) {
    let mut i: i32 = 0;
    let mut b: i32 = 0;
    let mut kfft: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state =
        0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
    let mut in_0: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut out: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut N: i32 = 480;
    let mut N2: i32 = 240;
    let mut A: *mut f32 = (*tonal).angle.as_mut_ptr();
    let mut dA: *mut f32 = (*tonal).d_angle.as_mut_ptr();
    let mut d2A: *mut f32 = (*tonal).d2_angle.as_mut_ptr();
    let mut tonality: *mut f32 = 0 as *mut f32;
    let mut noisiness: *mut f32 = 0 as *mut f32;
    let mut band_tonality: [f32; 18] = [0.; 18];
    let mut logE: [f32; 18] = [0.; 18];
    let mut BFCC: [f32; 8] = [0.; 8];
    let mut features: [f32; 25] = [0.; 25];
    let mut frame_tonality: f32 = 0.;
    let mut max_frame_tonality: f32 = 0.;
    /*float tw_sum=0;*/
    let mut frame_noisiness: f32 = 0.;
    let pi4: f32 = (3.14159265358979323846f64
        * 3.14159265358979323846
        * 3.14159265358979323846
        * 3.14159265358979323846) as f32;
    let mut slope: f32 = 0f32;
    let mut frame_stationarity: f32 = 0.;
    let mut relativeE: f32 = 0.;
    let mut frame_probs: [f32; 2] = [0.; 2];
    let mut alpha: f32 = 0.;
    let mut alphaE: f32 = 0.;
    let mut alphaE2: f32 = 0.;
    let mut frame_loudness: f32 = 0.;
    let mut bandwidth_mask: f32 = 0.;
    let mut bandwidth: i32 = 0;
    let mut maxE: f32 = 0f32;
    let mut noise_floor: f32 = 0.;
    let mut remaining: i32 = 0;
    let mut info: *mut crate::celt_h::AnalysisInfo = 0 as *mut crate::celt_h::AnalysisInfo;
    let mut hp_ener: f32 = 0.;
    let mut tonality2: [f32; 240] = [0.; 240];
    let mut midE: [f32; 8] = [0.; 8];
    let mut spec_variability: f32 = 0f32;
    let mut band_log2: [f32; 19] = [0.; 19];
    let mut leakage_from: [f32; 19] = [0.; 19];
    let mut leakage_to: [f32; 19] = [0.; 19];
    alpha = 1.0
        / (if (10) < 1 + (*tonal).count {
            10i32
        } else {
            (1) + (*tonal).count
        }) as f32;
    alphaE = 1.0
        / (if (25) < 1 + (*tonal).count {
            25i32
        } else {
            (1) + (*tonal).count
        }) as f32;
    alphaE2 = 1.0
        / (if (500) < 1 + (*tonal).count {
            500i32
        } else {
            (1) + (*tonal).count
        }) as f32;
    if (*tonal).Fs == 48000 {
        /* len and offset are now at 24 kHz. */
        len /= 2;
        offset /= 2
    } else if (*tonal).Fs == 16000 {
        len = 3 * len / 2;
        offset = 3 * offset / 2
    }
    if (*tonal).count < 4 {
        if (*tonal).application == 2048 {
            (*tonal).music_prob = 0.1f32
        } else {
            (*tonal).music_prob = 0.625f32
        }
    }
    kfft = (*celt_mode).mdct.kfft[0];
    if (*tonal).count == 0 {
        (*tonal).mem_fill = 240
    }
    (*tonal).hp_ener_accum += downmix_and_resample(
        downmix,
        x,
        &mut *(*tonal)
            .inmem
            .as_mut_ptr()
            .offset((*tonal).mem_fill as isize),
        (*tonal).downmix_state.as_mut_ptr(),
        if len < 720 - (*tonal).mem_fill {
            len
        } else {
            (720) - (*tonal).mem_fill
        },
        offset,
        c1,
        c2,
        C,
        (*tonal).Fs,
    );
    if (*tonal).mem_fill + len < 720 {
        (*tonal).mem_fill += len;
        /* Don't have enough to update the analysis */
        return;
    }
    hp_ener = (*tonal).hp_ener_accum;
    let fresh3 = (*tonal).write_pos;
    (*tonal).write_pos = (*tonal).write_pos + 1;
    info = &mut *(*tonal).info.as_mut_ptr().offset(fresh3 as isize)
        as *mut crate::celt_h::AnalysisInfo;
    if (*tonal).write_pos >= 100 {
        (*tonal).write_pos -= 100
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx>())
            .wrapping_mul(480usize),
    );
    in_0 = fresh4.as_mut_ptr() as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx>())
            .wrapping_mul(480usize),
    );
    out = fresh5.as_mut_ptr() as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut fresh6 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<f32>()).wrapping_mul(240usize));
    tonality = fresh6.as_mut_ptr() as *mut f32;
    let mut fresh7 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<f32>()).wrapping_mul(240usize));
    noisiness = fresh7.as_mut_ptr() as *mut f32;
    i = 0;
    while i < N2 {
        let mut w: f32 = analysis_window[i as usize];
        (*in_0.offset(i as isize)).r = w * (*tonal).inmem[i as usize];
        (*in_0.offset(i as isize)).i = w * (*tonal).inmem[(N2 + i) as usize];
        (*in_0.offset((N - i - 1) as isize)).r = w * (*tonal).inmem[(N - i - 1) as usize];
        (*in_0.offset((N - i - 1) as isize)).i = w * (*tonal).inmem[(N + N2 - i - 1) as usize];
        i += 1
    }
    crate::stdlib::memmove(
        (*tonal).inmem.as_mut_ptr() as *mut libc::c_void,
        (*tonal).inmem.as_mut_ptr().offset(720).offset(-(240)) as *const libc::c_void,
        (240usize)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>())
            .wrapping_add(
                (0 * (*tonal)
                    .inmem
                    .as_mut_ptr()
                    .wrapping_offset_from((*tonal).inmem.as_mut_ptr().offset(720).offset(-(240))))
                    as usize,
            ),
    );
    remaining = len - (720 - (*tonal).mem_fill);
    (*tonal).hp_ener_accum = downmix_and_resample(
        downmix,
        x,
        &mut *(*tonal).inmem.as_mut_ptr().offset(240),
        (*tonal).downmix_state.as_mut_ptr(),
        remaining,
        offset + 720 - (*tonal).mem_fill,
        c1,
        c2,
        C,
        (*tonal).Fs,
    );
    (*tonal).mem_fill = 240 + remaining;
    crate::src::opus_1_2_1::celt::kiss_fft::opus_fft_c(kfft, in_0, out);
    /* If there's any NaN on the input, the entire output will be NaN, so we only need to check one value. */
    if celt_isnan((*out.offset(0)).r) != 0 {
        (*info).valid = 0;
        return;
    }
    i = 1;
    while i < N2 {
        let mut X1r: f32 = 0.;
        let mut X2r: f32 = 0.;
        let mut X1i: f32 = 0.;
        let mut X2i: f32 = 0.;
        let mut angle: f32 = 0.;
        let mut d_angle: f32 = 0.;
        let mut d2_angle: f32 = 0.;
        let mut angle2: f32 = 0.;
        let mut d_angle2: f32 = 0.;
        let mut d2_angle2: f32 = 0.;
        let mut mod1: f32 = 0.;
        let mut mod2: f32 = 0.;
        let mut avg_mod: f32 = 0.;
        X1r = (*out.offset(i as isize)).r + (*out.offset((N - i) as isize)).r;
        X1i = (*out.offset(i as isize)).i - (*out.offset((N - i) as isize)).i;
        X2r = (*out.offset(i as isize)).i + (*out.offset((N - i) as isize)).i;
        X2i = (*out.offset((N - i) as isize)).r - (*out.offset(i as isize)).r;
        angle = (0.5f64 / 3.14159265358979323846) as f32 * fast_atan2f(X1i, X1r);
        d_angle = angle - *A.offset(i as isize);
        d2_angle = d_angle - *dA.offset(i as isize);
        angle2 = (0.5f64 / 3.14159265358979323846) as f32 * fast_atan2f(X2i, X2r);
        d_angle2 = angle2 - angle;
        d2_angle2 = d_angle2 - d_angle;
        mod1 = d2_angle - float2int(d2_angle) as f32;
        *noisiness.offset(i as isize) = crate::stdlib::fabs(mod1 as f64) as f32;
        mod1 *= mod1;
        mod1 *= mod1;
        mod2 = d2_angle2 - float2int(d2_angle2) as f32;
        *noisiness.offset(i as isize) += crate::stdlib::fabs(mod2 as f64) as f32;
        mod2 *= mod2;
        mod2 *= mod2;
        avg_mod = 0.25 * (*d2A.offset(i as isize) + mod1 + 2f32 * mod2);
        /* This introduces an extra delay of 2 frames in the detection. */
        *tonality.offset(i as isize) = 1.0 / (1.0 + 40.0 * 16.0 * pi4 * avg_mod) - 0.015;
        /* No delay on this detection, but it's less reliable. */
        tonality2[i as usize] = 1.0 / (1.0 + 40.0 * 16.0 * pi4 * mod2) - 0.015;
        *A.offset(i as isize) = angle2;
        *dA.offset(i as isize) = d_angle2;
        *d2A.offset(i as isize) = mod2;
        i += 1
    }
    i = 2;
    while i < N2 - 1 {
        let mut tt: f32 = if tonality2[i as usize]
            < (if tonality2[(i - 1) as usize] > tonality2[(i + 1) as usize] {
                tonality2[(i - 1) as usize]
            } else {
                tonality2[(i + 1) as usize]
            }) {
            tonality2[i as usize]
        } else if tonality2[(i - 1) as usize] > tonality2[(i + 1) as usize] {
            tonality2[(i - 1) as usize]
        } else {
            tonality2[(i + 1) as usize]
        };
        *tonality.offset(i as isize) = 0.9
            * (if *tonality.offset(i as isize) > tt - 0.1 {
                *tonality.offset(i as isize)
            } else {
                (tt) - 0.1
            });
        i += 1
    }
    frame_tonality = 0f32;
    max_frame_tonality = 0f32;
    /*tw_sum = 0;*/
    (*info).activity = 0f32;
    frame_noisiness = 0f32;
    frame_stationarity = 0f32;
    if (*tonal).count == 0 {
        b = 0;
        while b < 18 {
            (*tonal).lowE[b as usize] = 10000000000f32;
            (*tonal).highE[b as usize] = -10000000000f32;
            b += 1
        }
    }
    relativeE = 0f32;
    frame_loudness = 0f32;
    /* The energy of the very first band is special because of DC. */
    let mut E: f32 = 0f32;
    let mut X1r_0: f32 = 0.;
    let mut X2r_0: f32 = 0.;
    X1r_0 = 2f32 * (*out.offset(0)).r;
    X2r_0 = 2f32 * (*out.offset(0)).i;
    E = X1r_0 * X1r_0 + X2r_0 * X2r_0;
    i = 1;
    while i < 4 {
        let mut binE: f32 = (*out.offset(i as isize)).r * (*out.offset(i as isize)).r
            + (*out.offset((N - i) as isize)).r * (*out.offset((N - i) as isize)).r
            + (*out.offset(i as isize)).i * (*out.offset(i as isize)).i
            + (*out.offset((N - i) as isize)).i * (*out.offset((N - i) as isize)).i;
        E += binE;
        i += 1
    }
    E = E;
    band_log2[0] = 0.5 * 1.442695 * crate::stdlib::log((E + 1e-10) as f64) as f32;
    b = 0;
    while b < 18 {
        let mut E_0: f32 = 0f32;
        let mut tE: f32 = 0f32;
        let mut nE: f32 = 0f32;
        let mut L1: f32 = 0.;
        let mut L2: f32 = 0.;
        let mut stationarity: f32 = 0.;
        i = tbands[b as usize];
        while i < tbands[(b + 1) as usize] {
            let mut binE_0: f32 = (*out.offset(i as isize)).r * (*out.offset(i as isize)).r
                + (*out.offset((N - i) as isize)).r * (*out.offset((N - i) as isize)).r
                + (*out.offset(i as isize)).i * (*out.offset(i as isize)).i
                + (*out.offset((N - i) as isize)).i * (*out.offset((N - i) as isize)).i;
            binE_0 = binE_0;
            E_0 += binE_0;
            tE += binE_0
                * (if 0f32 > *tonality.offset(i as isize) {
                    0f32
                } else {
                    *tonality.offset(i as isize)
                });
            nE += binE_0 * 2.0 * (0.5 - *noisiness.offset(i as isize));
            i += 1
        }
        /* Check for extreme band energies that could cause NaNs later. */
        if !(E_0 < 1e9) || celt_isnan(E_0) != 0 {
            (*info).valid = 0;
            return;
        }
        (*tonal).E[(*tonal).E_count as usize][b as usize] = E_0;
        frame_noisiness += nE / (1e-15 + E_0);
        frame_loudness += crate::stdlib::sqrt((E_0 + 1e-10) as f64) as f32;
        logE[b as usize] = crate::stdlib::log((E_0 + 1e-10) as f64) as f32;
        band_log2[(b + 1) as usize] =
            0.5 * 1.442695 * crate::stdlib::log((E_0 + 1e-10) as f64) as f32;
        (*tonal).logE[(*tonal).E_count as usize][b as usize] = logE[b as usize];
        if (*tonal).count == 0 {
            (*tonal).lowE[b as usize] = logE[b as usize];
            (*tonal).highE[b as usize] = (*tonal).lowE[b as usize]
        }
        if (*tonal).highE[b as usize] as f64 > (*tonal).lowE[b as usize] as f64 + 7.5 {
            if (*tonal).highE[b as usize] - logE[b as usize]
                > logE[b as usize] - (*tonal).lowE[b as usize]
            {
                (*tonal).highE[b as usize] -= 0.01f32
            } else {
                (*tonal).lowE[b as usize] += 0.01f32
            }
        }
        if logE[b as usize] > (*tonal).highE[b as usize] {
            (*tonal).highE[b as usize] = logE[b as usize];
            (*tonal).lowE[b as usize] =
                if (*tonal).highE[b as usize] - 15f32 > (*tonal).lowE[b as usize] {
                    ((*tonal).highE[b as usize]) - 15f32
                } else {
                    (*tonal).lowE[b as usize]
                }
        } else if logE[b as usize] < (*tonal).lowE[b as usize] {
            (*tonal).lowE[b as usize] = logE[b as usize];
            (*tonal).highE[b as usize] =
                if ((*tonal).lowE[b as usize] + 15f32) < (*tonal).highE[b as usize] {
                    ((*tonal).lowE[b as usize]) + 15f32
                } else {
                    (*tonal).highE[b as usize]
                }
        }
        relativeE += (logE[b as usize] - (*tonal).lowE[b as usize])
            / (1e-15 + ((*tonal).highE[b as usize] - (*tonal).lowE[b as usize]));
        L2 = 0f32;
        L1 = L2;
        i = 0;
        while i < 8 {
            L1 += crate::stdlib::sqrt((*tonal).E[i as usize][b as usize] as f64) as f32;
            L2 += (*tonal).E[i as usize][b as usize];
            i += 1
        }
        stationarity = if 0.99 < L1 / crate::stdlib::sqrt(1e-15 + (8f32 * L2) as f64) as f32 {
            0.99
        } else {
            (L1) / crate::stdlib::sqrt(1e-15 + (8f32 * L2) as f64) as f32
        };
        stationarity *= stationarity;
        stationarity *= stationarity;
        frame_stationarity += stationarity;
        /*band_tonality[b] = tE/(1e-15+E)*/
        band_tonality[b as usize] =
            if tE / (1e-15 + E_0) > stationarity * (*tonal).prev_band_tonality[b as usize] {
                (tE) / (1e-15 + E_0)
            } else {
                (stationarity) * (*tonal).prev_band_tonality[b as usize]
            };
        frame_tonality += band_tonality[b as usize];
        if b >= 18 - 9 {
            frame_tonality -= band_tonality[(b - 18 + 9) as usize]
        }
        max_frame_tonality = if max_frame_tonality > (1.0 + 0.03 * (b - 18) as f32) * frame_tonality
        {
            max_frame_tonality
        } else {
            (1.0 + 0.03 * (b - 18) as f32) * frame_tonality
        };
        slope += band_tonality[b as usize] * (b - 8) as f32;
        /*printf("%f %f ", band_tonality[b], stationarity);*/
        (*tonal).prev_band_tonality[b as usize] = band_tonality[b as usize];
        b += 1
    }
    leakage_from[0] = band_log2[0];
    leakage_to[0] = band_log2[0] - 2.5;
    b = 1;
    while b < 18 + 1 {
        let mut leak_slope: f32 =
            2.0 * (tbands[b as usize] - tbands[(b - 1) as usize]) as f32 / 4f32;
        leakage_from[b as usize] =
            if leakage_from[(b - 1) as usize] + leak_slope < band_log2[b as usize] {
                (leakage_from[(b - 1) as usize]) + leak_slope
            } else {
                band_log2[b as usize]
            };
        leakage_to[b as usize] =
            if leakage_to[(b - 1) as usize] - leak_slope > band_log2[b as usize] - 2.5 {
                (leakage_to[(b - 1) as usize]) - leak_slope
            } else {
                (band_log2[b as usize]) - 2.5
            };
        b += 1
    }
    b = 18 - 2;
    while b >= 0 {
        let mut leak_slope_0: f32 =
            2.0 * (tbands[(b + 1) as usize] - tbands[b as usize]) as f32 / 4f32;
        leakage_from[b as usize] =
            if leakage_from[(b + 1) as usize] + leak_slope_0 < leakage_from[b as usize] {
                (leakage_from[(b + 1) as usize]) + leak_slope_0
            } else {
                leakage_from[b as usize]
            };
        leakage_to[b as usize] =
            if leakage_to[(b + 1) as usize] - leak_slope_0 > leakage_to[b as usize] {
                (leakage_to[(b + 1) as usize]) - leak_slope_0
            } else {
                leakage_to[b as usize]
            };
        b -= 1
    }
    b = 0;
    while b < 18 + 1 {
        /* leak_boost[] is made up of two terms. The first, based on leakage_to[],
        represents the boost needed to overcome the amount of analysis leakage
        cause in a weaker band b by louder neighbouring bands.
        The second, based on leakage_from[], applies to a loud band b for
        which the quantization noise causes synthesis leakage to the weaker
        neighbouring bands. */
        let mut boost: f32 =
            (if 0f32 > leakage_to[b as usize] - band_log2[b as usize] {
                0f32
            } else {
                (leakage_to[b as usize]) - band_log2[b as usize]
            }) + (if 0f32 > band_log2[b as usize] - (leakage_from[b as usize] + 2.5) {
                0f32
            } else {
                (band_log2[b as usize]) - (leakage_from[b as usize] + 2.5)
            });
        (*info).leak_boost[b as usize] =
            if (255) < crate::stdlib::floor(0.5 + (64.0 * boost) as f64) as i32 {
                255
            } else {
                crate::stdlib::floor(0.5 + (64.0 * boost) as f64) as i32
            } as u8;
        b += 1
    }
    while b < 19 {
        (*info).leak_boost[b as usize] = 0;
        b += 1
    }
    i = 0;
    while i < 8 {
        let mut j: i32 = 0;
        let mut mindist: f32 = 1e15;
        j = 0;
        while j < 8 {
            let mut k: i32 = 0;
            let mut dist: f32 = 0f32;
            k = 0;
            while k < 18 {
                let mut tmp: f32 = 0.;
                tmp = (*tonal).logE[i as usize][k as usize] - (*tonal).logE[j as usize][k as usize];
                dist += tmp * tmp;
                k += 1
            }
            if j != i {
                mindist = if mindist < dist { mindist } else { dist }
            }
            j += 1
        }
        spec_variability += mindist;
        i += 1
    }
    spec_variability = crate::stdlib::sqrt((spec_variability / 8f32 / 18f32) as f64) as f32;
    bandwidth_mask = 0f32;
    bandwidth = 0;
    maxE = 0f32;
    noise_floor = 5.7e-4
        / ((1i32)
            << (if 0 > lsb_depth - 8 {
                0
            } else {
                (lsb_depth) - 8
            })) as f32;
    noise_floor *= noise_floor;
    b = 0;
    while b < 18 {
        let mut E_1: f32 = 0f32;
        let mut band_start: i32 = 0;
        let mut band_end: i32 = 0;
        /* Keep a margin of 300 Hz for aliasing */
        band_start = tbands[b as usize];
        band_end = tbands[(b + 1) as usize];
        i = band_start;
        while i < band_end {
            let mut binE_1: f32 = (*out.offset(i as isize)).r * (*out.offset(i as isize)).r
                + (*out.offset((N - i) as isize)).r * (*out.offset((N - i) as isize)).r
                + (*out.offset(i as isize)).i * (*out.offset(i as isize)).i
                + (*out.offset((N - i) as isize)).i * (*out.offset((N - i) as isize)).i;
            E_1 += binE_1;
            i += 1
        }
        E_1 = E_1;
        maxE = if maxE > E_1 { maxE } else { E_1 };
        (*tonal).meanE[b as usize] = if (1f32 - alphaE2) * (*tonal).meanE[b as usize] > E_1 {
            (1f32 - alphaE2) * (*tonal).meanE[b as usize]
        } else {
            E_1
        };
        E_1 = if E_1 > (*tonal).meanE[b as usize] {
            E_1
        } else {
            (*tonal).meanE[b as usize]
        };
        /* Use a simple follower with 13 dB/Bark slope for spreading function */
        bandwidth_mask = if 0.05 * bandwidth_mask > E_1 {
            (0.05) * bandwidth_mask
        } else {
            E_1
        };
        /* Consider the band "active" only if all these conditions are met:
           1) less than 10 dB below the simple follower
           2) less than 90 dB below the peak band (maximal masking possible considering
              both the ATH and the loudness-dependent slope of the spreading function)
           3) above the PCM quantization noise floor
           We use b+1 because the first CELT band isn't included in tbands[]
        */
        if E_1 as f64 > 0.1 * bandwidth_mask as f64
            && E_1 * 1e9 > maxE
            && E_1 > noise_floor * (band_end - band_start) as f32
        {
            bandwidth = b + 1
        }
        b += 1
    }
    /* Special case for the last two bands, for which we don't have spectrum but only
    the energy above 12 kHz. */
    if (*tonal).Fs == 48000 {
        let mut ratio: f32 = 0.;
        let mut E_2: f32 = hp_ener * (1.0 / (240i32 * 240) as f32);
        ratio = if (*tonal).prev_bandwidth == 20 {
            0.03
        } else {
            0.07
        };
        maxE = if maxE > E_2 { maxE } else { E_2 };
        (*tonal).meanE[b as usize] = if (1f32 - alphaE2) * (*tonal).meanE[b as usize] > E_2 {
            (1f32 - alphaE2) * (*tonal).meanE[b as usize]
        } else {
            E_2
        };
        E_2 = if E_2 > (*tonal).meanE[b as usize] {
            E_2
        } else {
            (*tonal).meanE[b as usize]
        };
        /* Use a simple follower with 13 dB/Bark slope for spreading function */
        bandwidth_mask = if 0.05 * bandwidth_mask > E_2 {
            (0.05) * bandwidth_mask
        } else {
            E_2
        };
        if E_2 > ratio * bandwidth_mask && E_2 * 1e9 > maxE && E_2 > noise_floor * 160f32 {
            bandwidth = 20
        }
        /* This detector is unreliable, so if the bandwidth is close to SWB, assume it's FB. */
        if bandwidth >= 17 {
            bandwidth = 20
        }
    }
    if (*tonal).count <= 2 {
        bandwidth = 20
    }
    frame_loudness = 20f32 * crate::stdlib::log10(frame_loudness as f64) as f32;
    (*tonal).Etracker = if (*tonal).Etracker - 0.003 > frame_loudness {
        ((*tonal).Etracker) - 0.003
    } else {
        frame_loudness
    };
    (*tonal).lowECount *= 1f32 - alphaE;
    if frame_loudness < (*tonal).Etracker - 30f32 {
        (*tonal).lowECount += alphaE
    }
    i = 0;
    while i < 8 {
        let mut sum: f32 = 0f32;
        b = 0;
        while b < 16 {
            sum += dct_table[(i * 16 + b) as usize] * logE[b as usize];
            b += 1
        }
        BFCC[i as usize] = sum;
        i += 1
    }
    i = 0;
    while i < 8 {
        let mut sum_0: f32 = 0f32;
        b = 0;
        while b < 16 {
            sum_0 += dct_table[(i * 16 + b) as usize]
                * 0.5
                * ((*tonal).highE[b as usize] + (*tonal).lowE[b as usize]);
            b += 1
        }
        midE[i as usize] = sum_0;
        i += 1
    }
    frame_stationarity /= 18f32;
    relativeE /= 18f32;
    if (*tonal).count < 10 {
        relativeE = 0.5
    }
    frame_noisiness /= 18f32;
    (*info).activity = frame_noisiness + (1f32 - frame_noisiness) * relativeE;
    frame_tonality = max_frame_tonality / (18i32 - 9) as f32;
    frame_tonality = if frame_tonality > (*tonal).prev_tonality * 0.8 {
        frame_tonality
    } else {
        ((*tonal).prev_tonality) * 0.8
    };
    (*tonal).prev_tonality = frame_tonality;
    slope /= (8i32 * 8) as f32;
    (*info).tonality_slope = slope;
    (*tonal).E_count = ((*tonal).E_count + 1) % 8;
    (*tonal).count = if ((*tonal).count + 1) < 10000 {
        ((*tonal).count) + 1
    } else {
        10000
    };
    (*info).tonality = frame_tonality;
    i = 0;
    while i < 4 {
        features[i as usize] = -0.12299 * (BFCC[i as usize] + (*tonal).mem[(i + 24) as usize])
            + 0.49195 * ((*tonal).mem[i as usize] + (*tonal).mem[(i + 16) as usize])
            + 0.69693 * (*tonal).mem[(i + 8) as usize]
            - 1.4349 * (*tonal).cmean[i as usize];
        i += 1
    }
    i = 0;
    while i < 4 {
        (*tonal).cmean[i as usize] =
            (1f32 - alpha) * (*tonal).cmean[i as usize] + alpha * BFCC[i as usize];
        i += 1
    }
    i = 0;
    while i < 4 {
        features[(4 + i) as usize] = 0.63246 * (BFCC[i as usize] - (*tonal).mem[(i + 24) as usize])
            + 0.31623 * ((*tonal).mem[i as usize] - (*tonal).mem[(i + 16) as usize]);
        i += 1
    }
    i = 0;
    while i < 3 {
        features[(8 + i) as usize] = 0.53452 * (BFCC[i as usize] + (*tonal).mem[(i + 24) as usize])
            - 0.26726 * ((*tonal).mem[i as usize] + (*tonal).mem[(i + 16) as usize])
            - 0.53452 * (*tonal).mem[(i + 8) as usize];
        i += 1
    }
    if (*tonal).count > 5 {
        i = 0;
        while i < 9 {
            (*tonal).std[i as usize] = (1f32 - alpha) * (*tonal).std[i as usize]
                + alpha * features[i as usize] * features[i as usize];
            i += 1
        }
    }
    i = 0;
    while i < 4 {
        features[i as usize] = BFCC[i as usize] - midE[i as usize];
        i += 1
    }
    i = 0;
    while i < 8 {
        (*tonal).mem[(i + 24) as usize] = (*tonal).mem[(i + 16) as usize];
        (*tonal).mem[(i + 16) as usize] = (*tonal).mem[(i + 8) as usize];
        (*tonal).mem[(i + 8) as usize] = (*tonal).mem[i as usize];
        (*tonal).mem[i as usize] = BFCC[i as usize];
        i += 1
    }
    i = 0;
    while i < 9 {
        features[(11 + i) as usize] = crate::stdlib::sqrt((*tonal).std[i as usize] as f64) as f32
            - std_feature_bias[i as usize];
        i += 1
    }
    features[18] = spec_variability - 0.78;
    features[20] = (*info).tonality - 0.154723;
    features[21] = (*info).activity - 0.724643;
    features[22] = frame_stationarity - 0.743717;
    features[23] = (*info).tonality_slope + 0.069216;
    features[24] = (*tonal).lowECount - 0.067930;
    crate::src::opus_1_2_1::src::mlp::mlp_process(
        &crate::src::opus_1_2_1::src::mlp_data::net,
        features.as_mut_ptr(),
        frame_probs.as_mut_ptr(),
    );
    frame_probs[0] = 0.5 * (frame_probs[0] + 1f32);
    /* Curve fitting between the MLP probability and the actual probability */
    /*frame_probs[0] = .01f + 1.21f*frame_probs[0]*frame_probs[0] - .23f*(float)pow(frame_probs[0], 10);*/
    /* Probability of active audio (as opposed to silence) */
    frame_probs[1] = 0.5 * frame_probs[1] + 0.5;
    frame_probs[1] *= frame_probs[1];
    /* Probability of speech or music vs noise */
    (*info).activity_probability = frame_probs[1];
    /*printf("%f %f\n", frame_probs[0], frame_probs[1]);*/
    /* Probability of state transition */
    let mut tau: f32 = 0.;
    let mut beta: f32 = 0.;
    let mut p0: f32 = 0.;
    let mut p1: f32 = 0.;
    let mut s0: f32 = 0.;
    let mut m0: f32 = 0.;
    let mut psum: f32 = 0.;
    let mut speech0: f32 = 0.;
    let mut music0: f32 = 0.;
    let mut p: f32 = 0.;
    let mut q: f32 = 0.;
    tau = 0.001 * (*tonal).music_prob + 0.01 * (1f32 - (*tonal).music_prob);
    p = if 0.05
        > (if 0.95 < frame_probs[1] {
            0.95
        } else {
            frame_probs[1]
        }) {
        0.05
    } else if 0.95 < frame_probs[1] {
        0.95
    } else {
        frame_probs[1]
    };
    q = if 0.05f32
        > (if 0.95f32 < (*tonal).vad_prob {
            0.95
        } else {
            (*tonal).vad_prob
        }) {
        0.05
    } else if 0.95f32 < (*tonal).vad_prob {
        0.95
    } else {
        (*tonal).vad_prob
    };
    beta = 0.02
        + 0.05 * crate::stdlib::fabs((p - q) as f64) as f32 / (p * (1f32 - q) + q * (1f32 - p));
    p0 = (1f32 - (*tonal).vad_prob) * (1f32 - tau) + (*tonal).vad_prob * tau;
    p1 = (*tonal).vad_prob * (1f32 - tau) + (1f32 - (*tonal).vad_prob) * tau;
    p0 *= crate::stdlib::pow((1f32 - frame_probs[1]) as f64, beta as f64) as f32;
    p1 *= crate::stdlib::pow(frame_probs[1] as f64, beta as f64) as f32;
    (*tonal).vad_prob = p1 / (p0 + p1);
    (*info).vad_prob = (*tonal).vad_prob;
    frame_probs[0] = (*tonal).vad_prob * frame_probs[0] + (1f32 - (*tonal).vad_prob) * 0.5;
    tau = 0.0001;
    p = if 0.05
        > (if 0.95 < frame_probs[0] {
            0.95
        } else {
            frame_probs[0]
        }) {
        0.05
    } else if 0.95 < frame_probs[0] {
        0.95
    } else {
        frame_probs[0]
    };
    q = if 0.05f32
        > (if 0.95f32 < (*tonal).music_prob {
            0.95
        } else {
            (*tonal).music_prob
        }) {
        0.05
    } else if 0.95f32 < (*tonal).music_prob {
        0.95
    } else {
        (*tonal).music_prob
    };
    beta = 0.02
        + 0.05 * crate::stdlib::fabs((p - q) as f64) as f32 / (p * (1f32 - q) + q * (1f32 - p));
    p0 = (1f32 - (*tonal).music_prob) * (1f32 - tau) + (*tonal).music_prob * tau;
    p1 = (*tonal).music_prob * (1f32 - tau) + (1f32 - (*tonal).music_prob) * tau;
    p0 *= crate::stdlib::pow((1f32 - frame_probs[0]) as f64, beta as f64) as f32;
    p1 *= crate::stdlib::pow(frame_probs[0] as f64, beta as f64) as f32;
    (*tonal).music_prob = p1 / (p0 + p1);
    (*info).music_prob = (*tonal).music_prob;
    psum = 1e-20;
    speech0 = crate::stdlib::pow((1f32 - frame_probs[0]) as f64, beta as f64) as f32;
    music0 = crate::stdlib::pow(frame_probs[0] as f64, beta as f64) as f32;
    if (*tonal).count == 1 {
        if (*tonal).application == 2048 {
            (*tonal).pmusic[0] = 0.1f32
        } else {
            (*tonal).pmusic[0] = 0.625f32
        }
        (*tonal).pspeech[0] = 1f32 - (*tonal).pmusic[0]
    }
    s0 = (*tonal).pspeech[0] + (*tonal).pspeech[1];
    m0 = (*tonal).pmusic[0] + (*tonal).pmusic[1];
    (*tonal).pspeech[0] = s0 * (1f32 - tau) * speech0;
    (*tonal).pmusic[0] = m0 * (1f32 - tau) * music0;
    i = 1;
    while i < 100 - 1 {
        (*tonal).pspeech[i as usize] = (*tonal).pspeech[(i + 1) as usize] * speech0;
        (*tonal).pmusic[i as usize] = (*tonal).pmusic[(i + 1) as usize] * music0;
        i += 1
    }
    (*tonal).pspeech[(100i32 - 1) as usize] = m0 * tau * speech0;
    (*tonal).pmusic[(100i32 - 1) as usize] = s0 * tau * music0;
    i = 0;
    while i < 100 {
        psum += (*tonal).pspeech[i as usize] + (*tonal).pmusic[i as usize];
        i += 1
    }
    psum = 1.0 / psum;
    i = 0;
    while i < 100 {
        (*tonal).pspeech[i as usize] *= psum;
        (*tonal).pmusic[i as usize] *= psum;
        i += 1
    }
    psum = (*tonal).pmusic[0];
    i = 1;
    while i < 100 {
        psum += (*tonal).pspeech[i as usize];
        i += 1
    }
    if frame_probs[1] as f64 > 0.75 {
        if (*tonal).music_prob as f64 > 0.9 {
            let mut adapt: f32 = 0.;
            (*tonal).music_confidence_count += 1;
            adapt = 1.0 / (*tonal).music_confidence_count as f32;
            (*tonal).music_confidence_count = if (*tonal).music_confidence_count < 500 {
                (*tonal).music_confidence_count
            } else {
                500
            };
            (*tonal).music_confidence += adapt
                * (if -0.2 > frame_probs[0] - (*tonal).music_confidence {
                    -0.2
                } else {
                    (frame_probs[0]) - (*tonal).music_confidence
                })
        }
        if ((*tonal).music_prob as f64) < 0.1 {
            let mut adapt_0: f32 = 0.;
            (*tonal).speech_confidence_count += 1;
            adapt_0 = 1.0 / (*tonal).speech_confidence_count as f32;
            (*tonal).speech_confidence_count = if (*tonal).speech_confidence_count < 500 {
                (*tonal).speech_confidence_count
            } else {
                500
            };
            (*tonal).speech_confidence += adapt_0
                * (if 0.2 < frame_probs[0] - (*tonal).speech_confidence {
                    0.2
                } else {
                    (frame_probs[0]) - (*tonal).speech_confidence
                })
        }
    }
    (*tonal).last_music = ((*tonal).music_prob > 0.5f32) as i32;
    (*info).bandwidth = bandwidth;
    (*tonal).prev_bandwidth = bandwidth;
    /* Represents independence of the MLP probabilities, where
    beta=1 means fully independent. */
    /* Denormalized probability of speech (p0) and music (p1) after update */
    /* Probabilities for "all speech" and "all music" */
    /* Probability sum for renormalisation */
    /* Instantaneous probability of speech and music, with beta pre-applied. */
    /* More silence transitions for speech than for music. */
    /* p0 and p1 are the probabilities of speech and music at this frame
    using only information from previous frame and applying the
    state transition model */
    /* We apply the current probability with exponent beta to work around
    the fact that the probability estimates aren't independent. */
    /* Normalise the probabilities to get the Marokv probability of music. */
    /* Consider that silence has a 50-50 probability of being speech or music. */
    /* One transition every 3 minutes of active audio */
    /* Adapt beta based on how "unexpected" the new prob is */
    /* p0 and p1 are the probabilities of speech and music at this frame
    using only information from previous frame and applying the
    state transition model */
    /* We apply the current probability with exponent beta to work around
    the fact that the probability estimates aren't independent. */
    /* Normalise the probabilities to get the Marokv probability of music. */
    /*printf("%f %f %f %f\n", frame_probs[0], frame_probs[1], tonal->music_prob, tonal->vad_prob);*/
    /* This chunk of code deals with delayed decision. */
    /* Instantaneous probability of speech and music, with beta pre-applied. */
    /* Updated probability of having only speech (s0) or only music (m0),
    before considering the new observation. */
    /* Updates s0 and m0 with instantaneous probability. */
    /* Propagate the transition probabilities */
    /* Probability that the latest frame is speech, when all the previous ones were music. */
    /* Probability that the latest frame is music, when all the previous ones were speech. */
    /* Renormalise probabilities to 1 */
    /* Estimate our confidence in the speech/music decisions */
    /*printf("%d %d\n", info->bandwidth, info->opus_bandwidth);*/
    (*info).noisiness = frame_noisiness;
    (*info).valid = 1;
}
#[no_mangle]

pub unsafe extern "C" fn run_analysis(
    mut analysis: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut celt_mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut analysis_pcm: *const libc::c_void,
    mut analysis_frame_size: i32,
    mut frame_size: i32,
    mut c1: i32,
    mut c2: i32,
    mut C: i32,
    mut Fs: crate::opus_types_h::opus_int32,
    mut lsb_depth: i32,
    mut downmix: crate::opus_private_h::downmix_func,
    mut analysis_info: *mut crate::celt_h::AnalysisInfo,
) {
    let mut offset: i32 = 0;
    let mut pcm_len: i32 = 0;
    analysis_frame_size -= analysis_frame_size & 1;
    if !analysis_pcm.is_null() {
        /* Avoid overflow/wrap-around of the analysis buffer */
        analysis_frame_size = if ((100 - 5) * Fs / 50) < analysis_frame_size {
            ((100 - 5) * Fs) / 50
        } else {
            analysis_frame_size
        };
        pcm_len = analysis_frame_size - (*analysis).analysis_offset;
        offset = (*analysis).analysis_offset;
        while pcm_len > 0 {
            tonality_analysis(
                analysis,
                celt_mode,
                analysis_pcm,
                if (Fs / 50) < pcm_len {
                    (Fs) / 50
                } else {
                    pcm_len
                },
                offset,
                c1,
                c2,
                C,
                lsb_depth,
                downmix,
            );
            offset += Fs / 50;
            pcm_len -= Fs / 50
        }
        (*analysis).analysis_offset = analysis_frame_size;
        (*analysis).analysis_offset -= frame_size
    }
    (*analysis_info).valid = 0;
    tonality_get_info(analysis, analysis_info, frame_size);
}
/* DISABLE_FLOAT_API */
