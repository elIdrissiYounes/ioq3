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

    #[inline]

    pub unsafe extern "C" fn celt_isnan(mut x: libc::c_float) -> libc::c_int {
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        return (in_0.i >> 23 as libc::c_int & 0xff as libc::c_int as libc::c_uint
            == 0xff as libc::c_int as libc::c_uint
            && in_0.i & 0x7fffff as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint)
            as libc::c_int;
    }

    /* ARCH_H */
    /* !FIXED_POINT */
    /* This appears to be the same speed as C99's fabsf() but it's more portable. */
}

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    /*Returns the number of bits "used" by the encoded or decoded symbols so far.
    This same number can be computed in either the encoder or the decoder, and is
     suitable for making coding decisions.
    Return: The number of bits.
            This will always be slightly larger than the exact value (e.g., all
             rounding error is in the positive direction).*/
    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> libc::c_int {
        return (*_this).nbits_total
            - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod opus_private_h {

    /* Copyright (c) 2012 Xiph.Org Foundation
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
       A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
       OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
       EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
       PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
       PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
       LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
       NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
       SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
    */
    /* offsetof */

    /* Make sure everything is properly aligned. */
    #[inline]

    pub unsafe extern "C" fn align(mut i: libc::c_int) -> libc::c_int {
        let mut alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        /* Optimizing compilers should optimize div and multiply into and
        for all sensible alignment values. */
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }

    /* OPUS_PRIVATE_H */
}

pub mod mathops_h {

    #[inline]

    pub unsafe extern "C" fn celt_maxabs16(
        mut x: *const crate::arch_h::opus_val16,
        mut len: libc::c_int,
    ) -> crate::arch_h::opus_val32 {
        let mut i: libc::c_int = 0;
        let mut maxval: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
        let mut minval: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
        i = 0 as libc::c_int;
        while i < len {
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
            i += 1
        }
        return if maxval > -minval { maxval } else { -minval };
    }
    /* * Base-2 exponential approximation (2^x). */
    #[inline]

    pub unsafe extern "C" fn celt_exp2(mut x: libc::c_float) -> libc::c_float {
        let mut integer: libc::c_int = 0;
        let mut frac: libc::c_float = 0.;
        let mut res: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        integer = crate::stdlib::floor(x as libc::c_double) as libc::c_int;
        if integer < -(50 as libc::c_int) {
            return 0 as libc::c_int as libc::c_float;
        }
        frac = x - integer as libc::c_float;
        /* K0 = 1, K1 = log(2), K2 = 3-4*log(2), K3 = 3*log(2) - 2 */
        res.f =
            0.99992522f32 + frac * (0.69583354f32 + frac * (0.22606716f32 + 0.078024523f32 * frac));
        res.i = res
            .i
            .wrapping_add((integer << 23 as libc::c_int) as libc::c_uint)
            & 0x7fffffff as libc::c_int as libc::c_uint;
        return res.f;
    }

    use crate::arch_h::opus_val16;
    use crate::stdlib::floor;
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

    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}

pub mod float_cast_h {
    /* Copyright (C) 2001 Erik de Castro Lopo <erikd AT mega-nerd DOT com> */
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
    /* Version 1.1 */
    /*============================================================================
    **      On Intel Pentium processors (especially PIII and probably P4), converting
    **      from float to int is very slow. To meet the C specs, the code produced by
    **      most C compilers targeting Pentium needs to change the FPU rounding mode
    **      before the float to int conversion is performed.
    **
    **      Changing the FPU rounding mode causes the FPU pipeline to be flushed. It
    **      is this flushing of the pipeline which is so slow.
    **
    **      Fortunately the ISO C99 specifications define the functions lrint, lrintf,
    **      llrint and llrintf which fix this problem as a side effect.
    **
    **      On Unix-like systems, the configure process should have detected the
    **      presence of these functions. If they weren't found we have to replace them
    **      here with a standard C cast.
    */
    /*
    **      The C99 prototypes for lrint and lrintf are as follows:
    **
    **              long int lrintf (float x) ;
    **              long int lrint  (double x) ;
    */
    /*      The presence of the required functions are detected during the configure
    **      process and the values HAVE_LRINT and HAVE_LRINTF are set accordingly in
    **      the config.h file.
    */
    /* With GCC, when SSE is available, the fastest conversion is cvtss2si. */
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: libc::c_float) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    #[inline]

    pub unsafe extern "C" fn FLOAT2INT16(mut x: libc::c_float) -> crate::opus_types_h::opus_int16 {
        x = x * 32768.0f32;
        x = if x > -(32768 as libc::c_int) as libc::c_float {
            x
        } else {
            -(32768 as libc::c_int) as libc::c_float
        };
        x = if x < 32767 as libc::c_int as libc::c_float {
            x
        } else {
            32767 as libc::c_int as libc::c_float
        };
        return float2int(x) as crate::opus_types_h::opus_int16;
    }

    use crate::opus_types_h::opus_int16;
    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub mod pitch_h {
    /*We make sure a C version is always available for cases where the overhead of
    vectorization and passing around an arch flag aren't worth it.*/
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: libc::c_int,
    ) -> crate::arch_h::opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1
        }
        return xy;
    }
    use crate::arch_h::opus_val32;
}

pub mod os_support_h {
    /* Copyright (C) 2007 Jean-Marc Valin

       File: os_support.h
       This is the (tiny) OS abstraction layer. Aside from math.h, this is the
       only place where system headers are allowed.

       Redistribution and use in source and binary forms, with or without
       modification, are permitted provided that the following conditions are
       met:

       1. Redistributions of source code must retain the above copyright notice,
       this list of conditions and the following disclaimer.

       2. Redistributions in binary form must reproduce the above copyright
       notice, this list of conditions and the following disclaimer in the
       documentation and/or other materials provided with the distribution.

       THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
       IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
       OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
       DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT,
       INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
       (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
       SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
       HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
       STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
       ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
       POSSIBILITY OF SUCH DAMAGE.
    */
    /* * Opus wrapper for malloc(). To do your own dynamic allocation, all you need to do is replace this function and opus_free */
    /* * Same as celt_alloc(), except that the area is only needed inside a CELT call (might cause problem with wideband though) */
    /* Scratch space doesn't need to be cleared */
    /* * Opus wrapper for free(). To do your own dynamic allocation, all you need to do is replace this function and opus_alloc */
    #[inline]

    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        ::libc::free(ptr);
    }
    #[inline]

    pub unsafe extern "C" fn opus_alloc(mut size: crate::stddef_h::size_t) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }

    use crate::stdlib::malloc;
    use ::libc::free;
    /* OS_SUPPORT_H */
    /*#ifdef __GNUC__
    #pragma GCC poison printf sprintf
    #pragma GCC poison malloc free realloc calloc
    #endif*/
    /* * Set n elements of dst to zero */
    /* * Copy n elements from src to dst, allowing overlapping regions. The 0* term
    provides compile-time type checking */
    /* * Copy n elements from src to dst. The 0* term provides compile-time type checking  */
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
use crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl;
use crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::src::opus_encoder::arch_h::celt_isnan;
pub use crate::stddef_h::size_t;

pub use crate::celt_h::AnalysisInfo;
pub use crate::celt_h::SILKInfo;
pub use crate::control_h::silk_EncControlStruct;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_encode_with_ec;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_encoder_get_size;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_encoder_init;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::src::opus_encoder::entcode_h::ec_tell;

pub use crate::opus_private_h::downmix_func;
pub use crate::opus_private_h::foo;
pub use crate::opus_private_h::C2RustUnnamed_98;
pub use crate::opus_private_h::OpusRepacketizer;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_done;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_init;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_uint;
pub use crate::src::opus_1_2_1::src::analysis::run_analysis;
pub use crate::src::opus_1_2_1::src::analysis::tonality_analysis_init;
pub use crate::src::opus_1_2_1::src::analysis::tonality_analysis_reset;
pub use crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState;
pub use crate::src::opus_1_2_1::src::opus_encoder::cpu_support_h::opus_select_arch;
pub use crate::src::opus_1_2_1::src::opus_encoder::mathops_h::celt_exp2;
pub use crate::src::opus_1_2_1::src::opus_encoder::mathops_h::celt_maxabs16;
pub use crate::src::opus_1_2_1::src::opus_encoder::opus_private_h::align;
pub use crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_out_range_impl;
use crate::stdlib::fabs;
use crate::stdlib::floor;
use crate::stdlib::sqrt;
pub use crate::structs_FLP_h::silk_encoder;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::stereo_enc_state;
pub use crate::structs_h::SideInfoIndices;

use crate::src::opus_1_2_1::silk::enc_API::silk_Encode;
use crate::src::opus_1_2_1::silk::enc_API::silk_Get_Encoder_Size;
use crate::src::opus_1_2_1::silk::enc_API::silk_InitEncoder;
pub use crate::src::opus_1_2_1::src::opus_encoder::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::src::opus_encoder::float_cast_h::FLOAT2INT16;
pub use crate::src::opus_1_2_1::src::opus_encoder::pitch_h::celt_inner_prod_c;
use crate::src::opus_1_2_1::src::repacketizer::opus_packet_pad;
use crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_cat;
use crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_init;
use crate::stdlib::malloc;
use ::libc::free;

use crate::src::opus_1_2_1::silk::lin2log::silk_lin2log;
use crate::src::opus_1_2_1::silk::log2lin::silk_log2lin;
pub use crate::src::opus_1_2_1::src::opus_encoder::os_support_h::opus_alloc;
pub use crate::src::opus_1_2_1::src::opus_encoder::os_support_h::opus_free;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusEncoder {
    pub celt_enc_offset: libc::c_int,
    pub silk_enc_offset: libc::c_int,
    pub silk_mode: crate::control_h::silk_EncControlStruct,
    pub application: libc::c_int,
    pub channels: libc::c_int,
    pub delay_compensation: libc::c_int,
    pub force_channels: libc::c_int,
    pub signal_type: libc::c_int,
    pub user_bandwidth: libc::c_int,
    pub max_bandwidth: libc::c_int,
    pub user_forced_mode: libc::c_int,
    pub voice_ratio: libc::c_int,
    pub Fs: crate::opus_types_h::opus_int32,
    pub use_vbr: libc::c_int,
    pub vbr_constraint: libc::c_int,
    pub variable_duration: libc::c_int,
    pub bitrate_bps: crate::opus_types_h::opus_int32,
    pub user_bitrate_bps: crate::opus_types_h::opus_int32,
    pub lsb_depth: libc::c_int,
    pub encoder_buffer: libc::c_int,
    pub lfe: libc::c_int,
    pub arch: libc::c_int,
    pub use_dtx: libc::c_int,
    pub analysis: crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    pub stream_channels: libc::c_int,
    pub hybrid_stereo_width_Q14: crate::opus_types_h::opus_int16,
    pub variable_HP_smth2_Q15: crate::opus_types_h::opus_int32,
    pub prev_HB_gain: crate::arch_h::opus_val16,
    pub hp_mem: [crate::arch_h::opus_val32; 4],
    pub mode: libc::c_int,
    pub prev_mode: libc::c_int,
    pub prev_channels: libc::c_int,
    pub prev_framesize: libc::c_int,
    pub bandwidth: libc::c_int,
    pub auto_bandwidth: libc::c_int,
    pub silk_bw_switch: libc::c_int,
    pub first: libc::c_int,
    pub energy_masking: *mut crate::arch_h::opus_val16,
    pub width_mem: StereoWidthState,
    pub delay_buffer: [crate::arch_h::opus_val16; 960],
    pub detected_bandwidth: libc::c_int,
    pub nb_no_activity_frames: libc::c_int,
    pub peak_signal_energy: crate::arch_h::opus_val32,
    pub nonfinal_frame: libc::c_int,
    pub rangeFinal: crate::opus_types_h::opus_uint32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StereoWidthState {
    pub XX: crate::arch_h::opus_val32,
    pub XY: crate::arch_h::opus_val32,
    pub YY: crate::arch_h::opus_val32,
    pub smoothed_width: crate::arch_h::opus_val16,
    pub max_follower: crate::arch_h::opus_val16,
}
/* Transition tables for the voice and music. First column is the
middle (memoriless) threshold. The second column is the hysteresis
(difference with the middle) */

static mut mono_voice_bandwidth_thresholds: [crate::opus_types_h::opus_int32; 8] = [
    10000 as libc::c_int,
    1000 as libc::c_int,
    11000 as libc::c_int,
    1000 as libc::c_int,
    13500 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    2000 as libc::c_int,
];

static mut mono_music_bandwidth_thresholds: [crate::opus_types_h::opus_int32; 8] = [
    10000 as libc::c_int,
    1000 as libc::c_int,
    11000 as libc::c_int,
    1000 as libc::c_int,
    13500 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    2000 as libc::c_int,
];

static mut stereo_voice_bandwidth_thresholds: [crate::opus_types_h::opus_int32; 8] = [
    10000 as libc::c_int,
    1000 as libc::c_int,
    11000 as libc::c_int,
    1000 as libc::c_int,
    13500 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    2000 as libc::c_int,
];

static mut stereo_music_bandwidth_thresholds: [crate::opus_types_h::opus_int32; 8] = [
    10000 as libc::c_int,
    1000 as libc::c_int,
    11000 as libc::c_int,
    1000 as libc::c_int,
    13500 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    2000 as libc::c_int,
];
/* Threshold bit-rates for switching between mono and stereo */

static mut stereo_voice_threshold: crate::opus_types_h::opus_int32 = 24000 as libc::c_int;

static mut stereo_music_threshold: crate::opus_types_h::opus_int32 = 24000 as libc::c_int;
/* Threshold bit-rate for switching between SILK/hybrid and CELT-only */

static mut mode_thresholds: [[crate::opus_types_h::opus_int32; 2]; 2] = [
    [64000 as libc::c_int, 16000 as libc::c_int],
    [36000 as libc::c_int, 16000 as libc::c_int],
];

static mut fec_thresholds: [crate::opus_types_h::opus_int32; 10] = [
    12000 as libc::c_int,
    1000 as libc::c_int,
    14000 as libc::c_int,
    1000 as libc::c_int,
    16000 as libc::c_int,
    1000 as libc::c_int,
    20000 as libc::c_int,
    1000 as libc::c_int,
    22000 as libc::c_int,
    1000 as libc::c_int,
];
/* * Gets the size of an <code>OpusEncoder</code> structure.
 * @param[in] channels <tt>int</tt>: Number of channels.
 *                                   This must be 1 or 2.
 * @returns The size in bytes.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encoder_get_size(mut channels: libc::c_int) -> libc::c_int {
    let mut silkEncSizeBytes: libc::c_int = 0;
    let mut celtEncSizeBytes: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if channels < 1 as libc::c_int || channels > 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = crate::src::opus_1_2_1::silk::enc_API::silk_Get_Encoder_Size(&mut silkEncSizeBytes);
    if ret != 0 {
        return 0 as libc::c_int;
    }
    silkEncSizeBytes = align(silkEncSizeBytes);
    celtEncSizeBytes = crate::src::opus_1_2_1::celt::celt_encoder::celt_encoder_get_size(channels);
    return align(::std::mem::size_of::<OpusEncoder>() as libc::c_ulong as libc::c_int)
        + silkEncSizeBytes
        + celtEncSizeBytes;
}
/* * Initializes a previously allocated encoder state
 * The memory pointed to by st must be at least the size returned by opus_encoder_get_size().
 * This is intended for applications which use their own allocator instead of malloc.
 * @see opus_encoder_create(),opus_encoder_get_size()
 * To reset a previously initialized state, use the #OPUS_RESET_STATE CTL.
 * @param [in] st <tt>OpusEncoder*</tt>: Encoder state
 * @param [in] Fs <tt>opus_int32</tt>: Sampling rate of input signal (Hz)
 *                                      This must be one of 8000, 12000, 16000,
 *                                      24000, or 48000.
 * @param [in] channels <tt>int</tt>: Number of channels (1 or 2) in input signal
 * @param [in] application <tt>int</tt>: Coding mode (OPUS_APPLICATION_VOIP/OPUS_APPLICATION_AUDIO/OPUS_APPLICATION_RESTRICTED_LOWDELAY)
 * @retval #OPUS_OK Success or @ref opus_errorcodes
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encoder_init(
    mut st: *mut OpusEncoder,
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut application: libc::c_int,
) -> libc::c_int {
    let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_enc: *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder =
        0 as *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder;
    let mut err: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut silkEncSizeBytes: libc::c_int = 0;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
        || application != 2048 as libc::c_int
            && application != 2049 as libc::c_int
            && application != 2051 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    crate::stdlib::memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_encoder_get_size(channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    /* Create SILK encoder */
    ret = crate::src::opus_1_2_1::silk::enc_API::silk_Get_Encoder_Size(&mut silkEncSizeBytes);
    if ret != 0 {
        return -(1 as libc::c_int);
    }
    silkEncSizeBytes = align(silkEncSizeBytes);
    (*st).silk_enc_offset =
        align(::std::mem::size_of::<OpusEncoder>() as libc::c_ulong as libc::c_int);
    (*st).celt_enc_offset = (*st).silk_enc_offset + silkEncSizeBytes;
    silk_enc =
        (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize) as *mut libc::c_void;
    celt_enc = (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize)
        as *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).Fs = Fs;
    (*st).arch = opus_select_arch();
    ret = crate::src::opus_1_2_1::silk::enc_API::silk_InitEncoder(
        silk_enc,
        (*st).arch,
        &mut (*st).silk_mode as *mut _ as *mut crate::control_h::silk_EncControlStruct,
    );
    if ret != 0 {
        return -(3 as libc::c_int);
    }
    /* default SILK parameters */
    (*st).silk_mode.nChannelsAPI = channels;
    (*st).silk_mode.nChannelsInternal = channels;
    (*st).silk_mode.API_sampleRate = (*st).Fs;
    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
    (*st).silk_mode.minInternalSampleRate = 8000 as libc::c_int;
    (*st).silk_mode.desiredInternalSampleRate = 16000 as libc::c_int;
    (*st).silk_mode.payloadSize_ms = 20 as libc::c_int;
    (*st).silk_mode.bitRate = 25000 as libc::c_int;
    (*st).silk_mode.packetLossPercentage = 0 as libc::c_int;
    (*st).silk_mode.complexity = 9 as libc::c_int;
    (*st).silk_mode.useInBandFEC = 0 as libc::c_int;
    (*st).silk_mode.useDTX = 0 as libc::c_int;
    (*st).silk_mode.useCBR = 0 as libc::c_int;
    (*st).silk_mode.reducedDependency = 0 as libc::c_int;
    /* Create CELT encoder */
    /* Initialize CELT encoder */
    err = crate::src::opus_1_2_1::celt::celt_encoder::celt_encoder_init(
        celt_enc,
        Fs,
        channels,
        (*st).arch,
    );
    if err != 0 as libc::c_int {
        return -(3 as libc::c_int);
    }
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        10016 as libc::c_int,
        0 as libc::c_int,
    );
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        4010 as libc::c_int,
        (*st).silk_mode.complexity,
    );
    (*st).use_vbr = 1 as libc::c_int;
    /* Makes constrained VBR the default (safer for real-time use) */
    (*st).vbr_constraint = 1 as libc::c_int;
    (*st).user_bitrate_bps = -(1000 as libc::c_int);
    (*st).bitrate_bps = 3000 as libc::c_int + Fs * channels;
    (*st).application = application;
    (*st).signal_type = -(1000 as libc::c_int);
    (*st).user_bandwidth = -(1000 as libc::c_int);
    (*st).max_bandwidth = 1105 as libc::c_int;
    (*st).force_channels = -(1000 as libc::c_int);
    (*st).user_forced_mode = -(1000 as libc::c_int);
    (*st).voice_ratio = -(1 as libc::c_int);
    (*st).encoder_buffer = (*st).Fs / 100 as libc::c_int;
    (*st).lsb_depth = 24 as libc::c_int;
    (*st).variable_duration = 5000 as libc::c_int;
    /* Delay compensation of 4 ms (2.5 ms for SILK's extra look-ahead
    + 1.5 ms for SILK resamplers and stereo prediction) */
    (*st).delay_compensation = (*st).Fs / 250 as libc::c_int; /* Hybrid */
    (*st).hybrid_stereo_width_Q14 =
        ((1 as libc::c_int) << 14 as libc::c_int) as crate::opus_types_h::opus_int16;
    (*st).prev_HB_gain = 1.0f32;
    (*st).variable_HP_smth2_Q15 =
        ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
            as crate::opus_types_h::opus_uint32)
            << 8 as libc::c_int) as crate::opus_types_h::opus_int32;
    (*st).first = 1 as libc::c_int;
    (*st).mode = 1001 as libc::c_int;
    (*st).bandwidth = 1105 as libc::c_int;
    crate::src::opus_1_2_1::src::analysis::tonality_analysis_init(
        &mut (*st).analysis as *mut _
            as *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
        (*st).Fs,
    );
    (*st).analysis.application = (*st).application;
    return 0 as libc::c_int;
}

unsafe extern "C" fn gen_toc(
    mut mode: libc::c_int,
    mut framerate: libc::c_int,
    mut bandwidth: libc::c_int,
    mut channels: libc::c_int,
) -> libc::c_uchar {
    let mut period: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    period = 0 as libc::c_int;
    while framerate < 400 as libc::c_int {
        framerate <<= 1 as libc::c_int;
        period += 1
    }
    if mode == 1000 as libc::c_int {
        toc = ((bandwidth - 1101 as libc::c_int) << 5 as libc::c_int) as libc::c_uchar;
        toc =
            (toc as libc::c_int | (period - 2 as libc::c_int) << 3 as libc::c_int) as libc::c_uchar
    } else if mode == 1002 as libc::c_int {
        let mut tmp: libc::c_int = bandwidth - 1102 as libc::c_int;
        if tmp < 0 as libc::c_int {
            tmp = 0 as libc::c_int
        }
        toc = 0x80 as libc::c_int as libc::c_uchar;
        toc = (toc as libc::c_int | tmp << 5 as libc::c_int) as libc::c_uchar;
        toc = (toc as libc::c_int | period << 3 as libc::c_int) as libc::c_uchar
    } else {
        toc = 0x60 as libc::c_int as libc::c_uchar;
        toc = (toc as libc::c_int | (bandwidth - 1104 as libc::c_int) << 4 as libc::c_int)
            as libc::c_uchar;
        toc =
            (toc as libc::c_int | (period - 2 as libc::c_int) << 3 as libc::c_int) as libc::c_uchar
    }
    toc = (toc as libc::c_int | ((channels == 2 as libc::c_int) as libc::c_int) << 2 as libc::c_int)
        as libc::c_uchar;
    return toc;
}

unsafe extern "C" fn silk_biquad_float(
    mut in_0: *const crate::arch_h::opus_val16,
    mut B_Q28: *const crate::opus_types_h::opus_int32,
    mut A_Q28: *const crate::opus_types_h::opus_int32,
    mut S: *mut crate::arch_h::opus_val32,
    mut out: *mut crate::arch_h::opus_val16,
    len: crate::opus_types_h::opus_int32,
    mut stride: libc::c_int,
) {
    /* DIRECT FORM II TRANSPOSED (uses 2 element state vector) */
    let mut k: libc::c_int = 0;
    let mut vout: crate::arch_h::opus_val32 = 0.;
    let mut inval: crate::arch_h::opus_val32 = 0.;
    let mut A: [crate::arch_h::opus_val32; 2] = [0.; 2];
    let mut B: [crate::arch_h::opus_val32; 3] = [0.; 3];
    A[0 as libc::c_int as usize] = *A_Q28.offset(0 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    A[1 as libc::c_int as usize] = *A_Q28.offset(1 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    B[0 as libc::c_int as usize] = *B_Q28.offset(0 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    B[1 as libc::c_int as usize] = *B_Q28.offset(1 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    B[2 as libc::c_int as usize] = *B_Q28.offset(2 as libc::c_int as isize) as libc::c_float
        * (1.0f32 / ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_float);
    /* Negate A_Q28 values and split in two parts */
    k = 0 as libc::c_int;
    while k < len {
        /* S[ 0 ], S[ 1 ]: Q12 */
        inval = *in_0.offset((k * stride) as isize);
        vout = *S.offset(0 as libc::c_int as isize) + B[0 as libc::c_int as usize] * inval;
        *S.offset(0 as libc::c_int as isize) = *S.offset(1 as libc::c_int as isize)
            - vout * A[0 as libc::c_int as usize]
            + B[1 as libc::c_int as usize] * inval;
        *S.offset(1 as libc::c_int as isize) =
            -vout * A[1 as libc::c_int as usize] + B[2 as libc::c_int as usize] * inval + 1e-30f32;
        /* Scale back to Q0 and saturate */
        *out.offset((k * stride) as isize) = vout;
        k += 1
    }
}

unsafe extern "C" fn hp_cutoff(
    mut in_0: *const crate::arch_h::opus_val16,
    mut cutoff_Hz: crate::opus_types_h::opus_int32,
    mut out: *mut crate::arch_h::opus_val16,
    mut hp_mem: *mut crate::arch_h::opus_val32,
    mut len: libc::c_int,
    mut channels: libc::c_int,
    mut Fs: crate::opus_types_h::opus_int32,
    mut arch: libc::c_int,
) {
    let mut B_Q28: [crate::opus_types_h::opus_int32; 3] = [0; 3];
    let mut A_Q28: [crate::opus_types_h::opus_int32; 2] = [0; 2];
    let mut Fc_Q19: crate::opus_types_h::opus_int32 = 0;
    let mut r_Q28: crate::opus_types_h::opus_int32 = 0;
    let mut r_Q22: crate::opus_types_h::opus_int32 = 0;
    Fc_Q19 = (1.5f64 * 3.14159f64 / 1000 as libc::c_int as libc::c_double
        * ((1 as libc::c_int as libc::c_longlong) << 19 as libc::c_int) as libc::c_double
        + 0.5f64) as crate::opus_types_h::opus_int32 as crate::opus_types_h::opus_int16
        as crate::opus_types_h::opus_int32
        * cutoff_Hz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        / (Fs / 1000 as libc::c_int);
    r_Q28 = (1.0f64
        * ((1 as libc::c_int as libc::c_longlong) << 28 as libc::c_int) as libc::c_double
        + 0.5f64) as crate::opus_types_h::opus_int32
        - (0.92f64 * ((1 as libc::c_int as libc::c_longlong) << 9 as libc::c_int) as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
            * Fc_Q19;
    /* b = r * [ 1; -2; 1 ]; */
    /* a = [ 1; -2 * r * ( 1 - 0.5 * Fc^2 ); r^2 ]; */
    B_Q28[0 as libc::c_int as usize] = r_Q28;
    B_Q28[1 as libc::c_int as usize] = ((-r_Q28 as crate::opus_types_h::opus_uint32)
        << 1 as libc::c_int)
        as crate::opus_types_h::opus_int32;
    B_Q28[2 as libc::c_int as usize] = r_Q28;
    /* -r * ( 2 - Fc * Fc ); */
    r_Q22 = r_Q28 >> 6 as libc::c_int;
    A_Q28[0 as libc::c_int as usize] = (r_Q22 as libc::c_longlong
        * ((Fc_Q19 as libc::c_longlong * Fc_Q19 as libc::c_longlong >> 16 as libc::c_int)
            as crate::opus_types_h::opus_int32
            - (2.0f64
                * ((1 as libc::c_int as libc::c_longlong) << 22 as libc::c_int) as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32) as libc::c_longlong
        >> 16 as libc::c_int)
        as crate::opus_types_h::opus_int32;
    A_Q28[1 as libc::c_int as usize] = (r_Q22 as libc::c_longlong * r_Q22 as libc::c_longlong
        >> 16 as libc::c_int)
        as crate::opus_types_h::opus_int32;
    silk_biquad_float(
        in_0,
        B_Q28.as_mut_ptr(),
        A_Q28.as_mut_ptr(),
        hp_mem,
        out,
        len,
        channels,
    );
    if channels == 2 as libc::c_int {
        silk_biquad_float(
            in_0.offset(1 as libc::c_int as isize),
            B_Q28.as_mut_ptr(),
            A_Q28.as_mut_ptr(),
            hp_mem.offset(2 as libc::c_int as isize),
            out.offset(1 as libc::c_int as isize),
            len,
            channels,
        );
    };
}

unsafe extern "C" fn dc_reject(
    mut in_0: *const crate::arch_h::opus_val16,
    mut cutoff_Hz: crate::opus_types_h::opus_int32,
    mut out: *mut crate::arch_h::opus_val16,
    mut hp_mem: *mut crate::arch_h::opus_val32,
    mut len: libc::c_int,
    mut channels: libc::c_int,
    mut Fs: crate::opus_types_h::opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut coef: libc::c_float = 0.;
    let mut coef2: libc::c_float = 0.;
    coef = 4.0f32 * cutoff_Hz as libc::c_float / Fs as libc::c_float;
    coef2 = 1 as libc::c_int as libc::c_float - coef;
    if channels == 2 as libc::c_int {
        let mut m0: libc::c_float = 0.;
        let mut m1: libc::c_float = 0.;
        let mut m2: libc::c_float = 0.;
        let mut m3: libc::c_float = 0.;
        m0 = *hp_mem.offset(0 as libc::c_int as isize);
        m1 = *hp_mem.offset(1 as libc::c_int as isize);
        m2 = *hp_mem.offset(2 as libc::c_int as isize);
        m3 = *hp_mem.offset(3 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < len {
            let mut x0: crate::arch_h::opus_val32 = 0.;
            let mut x1: crate::arch_h::opus_val32 = 0.;
            let mut tmp0: crate::arch_h::opus_val32 = 0.;
            let mut tmp1: crate::arch_h::opus_val32 = 0.;
            let mut out0: crate::arch_h::opus_val32 = 0.;
            let mut out1: crate::arch_h::opus_val32 = 0.;
            x0 = *in_0.offset((2 as libc::c_int * i + 0 as libc::c_int) as isize);
            x1 = *in_0.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            /* First stage */
            tmp0 = x0 - m0;
            tmp1 = x1 - m2;
            m0 = coef * x0 + 1e-30f32 + coef2 * m0;
            m2 = coef * x1 + 1e-30f32 + coef2 * m2;
            /* Second stage */
            out0 = tmp0 - m1;
            out1 = tmp1 - m3;
            m1 = coef * tmp0 + 1e-30f32 + coef2 * m1;
            m3 = coef * tmp1 + 1e-30f32 + coef2 * m3;
            *out.offset((2 as libc::c_int * i + 0 as libc::c_int) as isize) = out0;
            *out.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) = out1;
            i += 1
        }
        *hp_mem.offset(0 as libc::c_int as isize) = m0;
        *hp_mem.offset(1 as libc::c_int as isize) = m1;
        *hp_mem.offset(2 as libc::c_int as isize) = m2;
        *hp_mem.offset(3 as libc::c_int as isize) = m3
    } else {
        let mut m0_0: libc::c_float = 0.;
        let mut m1_0: libc::c_float = 0.;
        m0_0 = *hp_mem.offset(0 as libc::c_int as isize);
        m1_0 = *hp_mem.offset(1 as libc::c_int as isize);
        i = 0 as libc::c_int;
        while i < len {
            let mut x: crate::arch_h::opus_val32 = 0.;
            let mut tmp: crate::arch_h::opus_val32 = 0.;
            let mut y: crate::arch_h::opus_val32 = 0.;
            x = *in_0.offset(i as isize);
            /* First stage */
            tmp = x - m0_0;
            m0_0 = coef * x + 1e-30f32 + coef2 * m0_0;
            /* Second stage */
            y = tmp - m1_0;
            m1_0 = coef * tmp + 1e-30f32 + coef2 * m1_0;
            *out.offset(i as isize) = y;
            i += 1
        }
        *hp_mem.offset(0 as libc::c_int as isize) = m0_0;
        *hp_mem.offset(1 as libc::c_int as isize) = m1_0
    };
}

unsafe extern "C" fn stereo_fade(
    mut in_0: *const crate::arch_h::opus_val16,
    mut out: *mut crate::arch_h::opus_val16,
    mut g1: crate::arch_h::opus_val16,
    mut g2: crate::arch_h::opus_val16,
    mut overlap48: libc::c_int,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut window: *const crate::arch_h::opus_val16,
    mut Fs: crate::opus_types_h::opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut inc: libc::c_int = 0;
    inc = 48000 as libc::c_int / Fs;
    overlap = overlap48 / inc;
    g1 = 1.0f32 - g1;
    g2 = 1.0f32 - g2;
    i = 0 as libc::c_int;
    while i < overlap {
        let mut diff: crate::arch_h::opus_val32 = 0.;
        let mut g: crate::arch_h::opus_val16 = 0.;
        let mut w: crate::arch_h::opus_val16 = 0.;
        w = *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
        g = w * g2 + (1.0f32 - w) * g1;
        diff = 0.5f32
            * (*in_0.offset((i * channels) as isize)
                - *in_0.offset((i * channels + 1 as libc::c_int) as isize));
        diff = g * diff;
        *out.offset((i * channels) as isize) = *out.offset((i * channels) as isize) - diff;
        *out.offset((i * channels + 1 as libc::c_int) as isize) =
            *out.offset((i * channels + 1 as libc::c_int) as isize) + diff;
        i += 1
    }
    while i < frame_size {
        let mut diff_0: crate::arch_h::opus_val32 = 0.;
        diff_0 = 0.5f32
            * (*in_0.offset((i * channels) as isize)
                - *in_0.offset((i * channels + 1 as libc::c_int) as isize));
        diff_0 = g2 * diff_0;
        *out.offset((i * channels) as isize) = *out.offset((i * channels) as isize) - diff_0;
        *out.offset((i * channels + 1 as libc::c_int) as isize) =
            *out.offset((i * channels + 1 as libc::c_int) as isize) + diff_0;
        i += 1
    }
}

unsafe extern "C" fn gain_fade(
    mut in_0: *const crate::arch_h::opus_val16,
    mut out: *mut crate::arch_h::opus_val16,
    mut g1: crate::arch_h::opus_val16,
    mut g2: crate::arch_h::opus_val16,
    mut overlap48: libc::c_int,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut window: *const crate::arch_h::opus_val16,
    mut Fs: crate::opus_types_h::opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut inc: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    inc = 48000 as libc::c_int / Fs;
    overlap = overlap48 / inc;
    if channels == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < overlap {
            let mut g: crate::arch_h::opus_val16 = 0.;
            let mut w: crate::arch_h::opus_val16 = 0.;
            w = *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            g = w * g2 + (1.0f32 - w) * g1;
            *out.offset(i as isize) = g * *in_0.offset(i as isize);
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < overlap {
            let mut g_0: crate::arch_h::opus_val16 = 0.;
            let mut w_0: crate::arch_h::opus_val16 = 0.;
            w_0 = *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            g_0 = w_0 * g2 + (1.0f32 - w_0) * g1;
            *out.offset((i * 2 as libc::c_int) as isize) =
                g_0 * *in_0.offset((i * 2 as libc::c_int) as isize);
            *out.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize) =
                g_0 * *in_0.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
            i += 1
        }
    }
    c = 0 as libc::c_int;
    loop {
        i = overlap;
        while i < frame_size {
            *out.offset((i * channels + c) as isize) =
                g2 * *in_0.offset((i * channels + c) as isize);
            i += 1
        }
        c += 1;
        if !(c < channels) {
            break;
        }
    }
}
/* *
 */
/* * Allocates and initializes an encoder state.
 * There are three coding modes:
 *
 * @ref OPUS_APPLICATION_VOIP gives best quality at a given bitrate for voice
 *    signals. It enhances the  input signal by high-pass filtering and
 *    emphasizing formants and harmonics. Optionally  it includes in-band
 *    forward error correction to protect against packet loss. Use this
 *    mode for typical VoIP applications. Because of the enhancement,
 *    even at high bitrates the output may sound different from the input.
 *
 * @ref OPUS_APPLICATION_AUDIO gives best quality at a given bitrate for most
 *    non-voice signals like music. Use this mode for music and mixed
 *    (music/voice) content, broadcast, and applications requiring less
 *    than 15 ms of coding delay.
 *
 * @ref OPUS_APPLICATION_RESTRICTED_LOWDELAY configures low-delay mode that
 *    disables the speech-optimized mode in exchange for slightly reduced delay.
 *    This mode can only be set on an newly initialized or freshly reset encoder
 *    because it changes the codec delay.
 *
 * This is useful when the caller knows that the speech-optimized modes will not be needed (use with caution).
 * @param [in] Fs <tt>opus_int32</tt>: Sampling rate of input signal (Hz)
 *                                     This must be one of 8000, 12000, 16000,
 *                                     24000, or 48000.
 * @param [in] channels <tt>int</tt>: Number of channels (1 or 2) in input signal
 * @param [in] application <tt>int</tt>: Coding mode (@ref OPUS_APPLICATION_VOIP/@ref OPUS_APPLICATION_AUDIO/@ref OPUS_APPLICATION_RESTRICTED_LOWDELAY)
 * @param [out] error <tt>int*</tt>: @ref opus_errorcodes
 * @note Regardless of the sampling rate and number channels selected, the Opus encoder
 * can switch to a lower audio bandwidth or number of channels if the bitrate
 * selected is too low. This also means that it is safe to always use 48 kHz stereo input
 * and let the encoder optimize the encoding.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encoder_create(
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut application: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusEncoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusEncoder = 0 as *mut OpusEncoder;
    if Fs != 48000 as libc::c_int
        && Fs != 24000 as libc::c_int
        && Fs != 16000 as libc::c_int
        && Fs != 12000 as libc::c_int
        && Fs != 8000 as libc::c_int
        || channels != 1 as libc::c_int && channels != 2 as libc::c_int
        || application != 2048 as libc::c_int
            && application != 2049 as libc::c_int
            && application != 2051 as libc::c_int
    {
        if !error.is_null() {
            *error = -(1 as libc::c_int)
        }
        return 0 as *mut OpusEncoder;
    }
    st = opus_alloc(opus_encoder_get_size(channels) as crate::stddef_h::size_t) as *mut OpusEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int)
        }
        return 0 as *mut OpusEncoder;
    }
    ret = opus_encoder_init(st, Fs, channels, application);
    if !error.is_null() {
        *error = ret
    }
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusEncoder
    }
    return st;
}

unsafe extern "C" fn user_bitrate_to_bitrate(
    mut st: *mut OpusEncoder,
    mut frame_size: libc::c_int,
    mut max_data_bytes: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    if frame_size == 0 {
        frame_size = (*st).Fs / 400 as libc::c_int
    }
    if (*st).user_bitrate_bps == -(1000 as libc::c_int) {
        return 60 as libc::c_int * (*st).Fs / frame_size + (*st).Fs * (*st).channels;
    } else if (*st).user_bitrate_bps == -(1 as libc::c_int) {
        return max_data_bytes * 8 as libc::c_int * (*st).Fs / frame_size;
    } else {
        return (*st).user_bitrate_bps;
    };
}
#[no_mangle]

pub unsafe extern "C" fn downmix_float(
    mut _x: *const libc::c_void,
    mut y: *mut crate::arch_h::opus_val32,
    mut subframe: libc::c_int,
    mut offset: libc::c_int,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
    mut C: libc::c_int,
) {
    let mut x: *const libc::c_float = 0 as *const libc::c_float;
    let mut j: libc::c_int = 0;
    x = _x as *const libc::c_float;
    j = 0 as libc::c_int;
    while j < subframe {
        *y.offset(j as isize) = *x.offset(((j + offset) * C + c1) as isize) * 32768.0f32;
        j += 1
    }
    if c2 > -(1 as libc::c_int) {
        j = 0 as libc::c_int;
        while j < subframe {
            let ref mut fresh0 = *y.offset(j as isize);
            *fresh0 += *x.offset(((j + offset) * C + c2) as isize) * 32768.0f32;
            j += 1
        }
    } else if c2 == -(2 as libc::c_int) {
        let mut c: libc::c_int = 0;
        c = 1 as libc::c_int;
        while c < C {
            j = 0 as libc::c_int;
            while j < subframe {
                let ref mut fresh1 = *y.offset(j as isize);
                *fresh1 += *x.offset(((j + offset) * C + c) as isize) * 32768.0f32;
                j += 1
            }
            c += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn downmix_int(
    mut _x: *const libc::c_void,
    mut y: *mut crate::arch_h::opus_val32,
    mut subframe: libc::c_int,
    mut offset: libc::c_int,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
    mut C: libc::c_int,
) {
    let mut x: *const crate::opus_types_h::opus_int16 = 0 as *const crate::opus_types_h::opus_int16;
    let mut j: libc::c_int = 0;
    x = _x as *const crate::opus_types_h::opus_int16;
    j = 0 as libc::c_int;
    while j < subframe {
        *y.offset(j as isize) =
            *x.offset(((j + offset) * C + c1) as isize) as crate::arch_h::opus_val32;
        j += 1
    }
    if c2 > -(1 as libc::c_int) {
        j = 0 as libc::c_int;
        while j < subframe {
            let ref mut fresh2 = *y.offset(j as isize);
            *fresh2 += *x.offset(((j + offset) * C + c2) as isize) as libc::c_int as libc::c_float;
            j += 1
        }
    } else if c2 == -(2 as libc::c_int) {
        let mut c: libc::c_int = 0;
        c = 1 as libc::c_int;
        while c < C {
            j = 0 as libc::c_int;
            while j < subframe {
                let ref mut fresh3 = *y.offset(j as isize);
                *fresh3 +=
                    *x.offset(((j + offset) * C + c) as isize) as libc::c_int as libc::c_float;
                j += 1
            }
            c += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn frame_size_select(
    mut frame_size: crate::opus_types_h::opus_int32,
    mut variable_duration: libc::c_int,
    mut Fs: crate::opus_types_h::opus_int32,
) -> crate::opus_types_h::opus_int32 {
    let mut new_size: libc::c_int = 0;
    if frame_size < Fs / 400 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if variable_duration == 5000 as libc::c_int {
        new_size = frame_size
    } else if variable_duration >= 5001 as libc::c_int && variable_duration <= 5009 as libc::c_int {
        if variable_duration <= 5005 as libc::c_int {
            new_size = (Fs / 400 as libc::c_int) << variable_duration - 5001 as libc::c_int
        } else {
            new_size = (variable_duration - 5001 as libc::c_int - 2 as libc::c_int) * Fs
                / 50 as libc::c_int
        }
    } else {
        return -(1 as libc::c_int);
    }
    if new_size > frame_size {
        return -(1 as libc::c_int);
    }
    if 400 as libc::c_int * new_size != Fs
        && 200 as libc::c_int * new_size != Fs
        && 100 as libc::c_int * new_size != Fs
        && 50 as libc::c_int * new_size != Fs
        && 25 as libc::c_int * new_size != Fs
        && 50 as libc::c_int * new_size != 3 as libc::c_int * Fs
        && 50 as libc::c_int * new_size != 4 as libc::c_int * Fs
        && 50 as libc::c_int * new_size != 5 as libc::c_int * Fs
        && 50 as libc::c_int * new_size != 6 as libc::c_int * Fs
    {
        return -(1 as libc::c_int);
    }
    return new_size;
}
#[no_mangle]

pub unsafe extern "C" fn compute_stereo_width(
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut Fs: crate::opus_types_h::opus_int32,
    mut mem: *mut StereoWidthState,
) -> crate::arch_h::opus_val16 {
    let mut xx: crate::arch_h::opus_val32 = 0.;
    let mut xy: crate::arch_h::opus_val32 = 0.;
    let mut yy: crate::arch_h::opus_val32 = 0.;
    let mut sqrt_xx: crate::arch_h::opus_val16 = 0.;
    let mut sqrt_yy: crate::arch_h::opus_val16 = 0.;
    let mut qrrt_xx: crate::arch_h::opus_val16 = 0.;
    let mut qrrt_yy: crate::arch_h::opus_val16 = 0.;
    let mut frame_rate: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut short_alpha: crate::arch_h::opus_val16 = 0.;
    frame_rate = Fs / frame_size;
    short_alpha = 1.0f32
        - 25 as libc::c_int as crate::arch_h::opus_val32 * 1.0f32
            / (if 50 as libc::c_int > frame_rate {
                50 as libc::c_int
            } else {
                frame_rate
            }) as libc::c_float;
    yy = 0 as libc::c_int as crate::arch_h::opus_val32;
    xy = yy;
    xx = xy;
    /* Unroll by 4. The frame size is always a multiple of 4 *except* for
    2.5 ms frames at 12 kHz. Since this setting is very rare (and very
    stupid), we just discard the last two samples. */
    i = 0 as libc::c_int;
    while i < frame_size - 3 as libc::c_int {
        let mut pxx: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        let mut pxy: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        let mut pyy: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        let mut x: crate::arch_h::opus_val16 = 0.;
        let mut y: crate::arch_h::opus_val16 = 0.;
        x = *pcm.offset((2 as libc::c_int * i) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
        pxx = x * x;
        pxy = x * y;
        pyy = y * y;
        x = *pcm.offset((2 as libc::c_int * i + 2 as libc::c_int) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 3 as libc::c_int) as isize);
        pxx += x * x;
        pxy += x * y;
        pyy += y * y;
        x = *pcm.offset((2 as libc::c_int * i + 4 as libc::c_int) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 5 as libc::c_int) as isize);
        pxx += x * x;
        pxy += x * y;
        pyy += y * y;
        x = *pcm.offset((2 as libc::c_int * i + 6 as libc::c_int) as isize);
        y = *pcm.offset((2 as libc::c_int * i + 7 as libc::c_int) as isize);
        pxx += x * x;
        pxy += x * y;
        pyy += y * y;
        xx += pxx;
        xy += pxy;
        yy += pyy;
        i += 4 as libc::c_int
    }
    (*mem).XX += short_alpha * (xx - (*mem).XX);
    (*mem).XY += short_alpha * (xy - (*mem).XY);
    (*mem).YY += short_alpha * (yy - (*mem).YY);
    (*mem).XX = if 0 as libc::c_int as libc::c_float > (*mem).XX {
        0 as libc::c_int as libc::c_float
    } else {
        (*mem).XX
    };
    (*mem).XY = if 0 as libc::c_int as libc::c_float > (*mem).XY {
        0 as libc::c_int as libc::c_float
    } else {
        (*mem).XY
    };
    (*mem).YY = if 0 as libc::c_int as libc::c_float > (*mem).YY {
        0 as libc::c_int as libc::c_float
    } else {
        (*mem).YY
    };
    if (if (*mem).XX > (*mem).YY {
        (*mem).XX
    } else {
        (*mem).YY
    }) > 8e-4f32
    {
        let mut corr: crate::arch_h::opus_val16 = 0.;
        let mut ldiff: crate::arch_h::opus_val16 = 0.;
        let mut width: crate::arch_h::opus_val16 = 0.;
        sqrt_xx = crate::stdlib::sqrt((*mem).XX as libc::c_double) as libc::c_float;
        sqrt_yy = crate::stdlib::sqrt((*mem).YY as libc::c_double) as libc::c_float;
        qrrt_xx = crate::stdlib::sqrt(sqrt_xx as libc::c_double) as libc::c_float;
        qrrt_yy = crate::stdlib::sqrt(sqrt_yy as libc::c_double) as libc::c_float;
        /* Inter-channel correlation */
        (*mem).XY = if (*mem).XY < sqrt_xx * sqrt_yy {
            (*mem).XY
        } else {
            (sqrt_xx) * sqrt_yy
        };
        corr = (*mem).XY / (1e-15f32 + sqrt_xx * sqrt_yy);
        /* Approximate loudness difference */
        ldiff = 1.0f32
            * crate::stdlib::fabs((qrrt_xx - qrrt_yy) as libc::c_double) as libc::c_float
            / (1e-15f32 + qrrt_xx + qrrt_yy);
        width =
            crate::stdlib::sqrt((1.0f32 - corr * corr) as libc::c_double) as libc::c_float * ldiff;
        /* Smoothing over one second */
        (*mem).smoothed_width += (width - (*mem).smoothed_width) / frame_rate as libc::c_float;
        /* Peak follower */
        (*mem).max_follower =
            if (*mem).max_follower - 0.02f32 / frame_rate as libc::c_float > (*mem).smoothed_width {
                ((*mem).max_follower) - 0.02f32 / frame_rate as libc::c_float
            } else {
                (*mem).smoothed_width
            }
    }
    /*printf("%f %f %f %f %f ", corr/(float)Q15ONE, ldiff/(float)Q15ONE, width/(float)Q15ONE, mem->smoothed_width/(float)Q15ONE, mem->max_follower/(float)Q15ONE);*/
    return if 1.0f32 < 20 as libc::c_int as crate::arch_h::opus_val32 * (*mem).max_follower {
        1.0f32
    } else {
        (20 as libc::c_int as crate::arch_h::opus_val32) * (*mem).max_follower
    };
}

unsafe extern "C" fn decide_fec(
    mut useInBandFEC: libc::c_int,
    mut PacketLoss_perc: libc::c_int,
    mut last_fec: libc::c_int,
    mut mode: libc::c_int,
    mut bandwidth: *mut libc::c_int,
    mut rate: crate::opus_types_h::opus_int32,
) -> libc::c_int {
    let mut orig_bandwidth: libc::c_int = 0;
    if useInBandFEC == 0 || PacketLoss_perc == 0 as libc::c_int || mode == 1002 as libc::c_int {
        return 0 as libc::c_int;
    }
    orig_bandwidth = *bandwidth;
    loop {
        let mut hysteresis: crate::opus_types_h::opus_int32 = 0;
        let mut LBRR_rate_thres_bps: crate::opus_types_h::opus_int32 = 0;
        /* Compute threshold for using FEC at the current bandwidth setting */
        LBRR_rate_thres_bps =
            fec_thresholds[(2 as libc::c_int * (*bandwidth - 1101 as libc::c_int)) as usize];
        hysteresis = fec_thresholds
            [(2 as libc::c_int * (*bandwidth - 1101 as libc::c_int) + 1 as libc::c_int) as usize];
        if last_fec == 1 as libc::c_int {
            LBRR_rate_thres_bps -= hysteresis
        }
        if last_fec == 0 as libc::c_int {
            LBRR_rate_thres_bps += hysteresis
        }
        LBRR_rate_thres_bps = ((LBRR_rate_thres_bps
            * (125 as libc::c_int
                - (if PacketLoss_perc < 25 as libc::c_int {
                    PacketLoss_perc
                } else {
                    25 as libc::c_int
                }))) as libc::c_longlong
            * (0.01f64
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32;
        /* If loss <= 5%, we look at whether we have enough rate to enable FEC.
        If loss > 5%, we decrease the bandwidth until we can enable FEC. */
        if rate > LBRR_rate_thres_bps {
            return 1 as libc::c_int;
        } else if PacketLoss_perc <= 5 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            if !(*bandwidth > 1101 as libc::c_int) {
                break;
            }
            *bandwidth -= 1
        }
    }
    /* Couldn't find any bandwidth to enable FEC, keep original bandwidth. */
    *bandwidth = orig_bandwidth;
    return 0 as libc::c_int;
}

unsafe extern "C" fn compute_silk_rate_for_hybrid(
    mut rate: libc::c_int,
    mut bandwidth: libc::c_int,
    mut frame20ms: libc::c_int,
    mut vbr: libc::c_int,
    mut fec: libc::c_int,
) -> libc::c_int {
    let mut entry: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut silk_rate: libc::c_int = 0;
    static mut rate_table: [[libc::c_int; 5]; 7] = [
        [
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ],
        [
            12000 as libc::c_int,
            10000 as libc::c_int,
            10000 as libc::c_int,
            11000 as libc::c_int,
            11000 as libc::c_int,
        ],
        [
            16000 as libc::c_int,
            13500 as libc::c_int,
            13500 as libc::c_int,
            15000 as libc::c_int,
            15000 as libc::c_int,
        ],
        [
            20000 as libc::c_int,
            16000 as libc::c_int,
            16000 as libc::c_int,
            18000 as libc::c_int,
            18000 as libc::c_int,
        ],
        [
            24000 as libc::c_int,
            18000 as libc::c_int,
            18000 as libc::c_int,
            21000 as libc::c_int,
            21000 as libc::c_int,
        ],
        [
            32000 as libc::c_int,
            22000 as libc::c_int,
            22000 as libc::c_int,
            28000 as libc::c_int,
            28000 as libc::c_int,
        ],
        [
            64000 as libc::c_int,
            38000 as libc::c_int,
            38000 as libc::c_int,
            50000 as libc::c_int,
            50000 as libc::c_int,
        ],
    ];
    entry = 1 as libc::c_int + frame20ms + 2 as libc::c_int * fec;
    N = (::std::mem::size_of::<[[libc::c_int; 5]; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
        as libc::c_int;
    i = 1 as libc::c_int;
    while i < N {
        if rate_table[i as usize][0 as libc::c_int as usize] > rate {
            break;
        }
        i += 1
    }
    if i == N {
        silk_rate = rate_table[(i - 1 as libc::c_int) as usize][entry as usize];
        /* For now, just give 50% of the extra bits to SILK. */
        silk_rate += (rate - rate_table[(i - 1 as libc::c_int) as usize][0 as libc::c_int as usize])
            / 2 as libc::c_int
    } else {
        let mut lo: crate::opus_types_h::opus_int32 = 0;
        let mut hi: crate::opus_types_h::opus_int32 = 0;
        let mut x0: crate::opus_types_h::opus_int32 = 0;
        let mut x1: crate::opus_types_h::opus_int32 = 0;
        lo = rate_table[(i - 1 as libc::c_int) as usize][entry as usize];
        hi = rate_table[i as usize][entry as usize];
        x0 = rate_table[(i - 1 as libc::c_int) as usize][0 as libc::c_int as usize];
        x1 = rate_table[i as usize][0 as libc::c_int as usize];
        silk_rate = (lo * (x1 - rate) + hi * (rate - x0)) / (x1 - x0)
    }
    if vbr == 0 {
        /* Tiny boost to SILK for CBR. We should probably tune this better. */
        silk_rate += 100 as libc::c_int
    }
    if bandwidth == 1104 as libc::c_int {
        silk_rate += 300 as libc::c_int
    }
    return silk_rate;
}
/* Returns the equivalent bitrate corresponding to 20 ms frames,
complexity 10 VBR operation. */

unsafe extern "C" fn compute_equiv_rate(
    mut bitrate: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut frame_rate: libc::c_int,
    mut vbr: libc::c_int,
    mut mode: libc::c_int,
    mut complexity: libc::c_int,
    mut loss: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    let mut equiv: crate::opus_types_h::opus_int32 = 0;
    equiv = bitrate;
    /* Take into account overhead from smaller frames. */
    equiv -= (40 as libc::c_int * channels + 20 as libc::c_int) * (frame_rate - 50 as libc::c_int);
    /* CBR is about a 8% penalty for both SILK and CELT. */
    if vbr == 0 {
        equiv -= equiv / 12 as libc::c_int
    }
    /* Complexity makes about 10% difference (from 0 to 10) in general. */
    equiv = equiv * (90 as libc::c_int + complexity) / 100 as libc::c_int;
    if mode == 1000 as libc::c_int || mode == 1001 as libc::c_int {
        /* SILK complexity 0-1 uses the non-delayed-decision NSQ, which
        costs about 20%. */
        if complexity < 2 as libc::c_int {
            equiv = equiv * 4 as libc::c_int / 5 as libc::c_int
        }
        equiv -= equiv * loss / (6 as libc::c_int * loss + 10 as libc::c_int)
    } else if mode == 1002 as libc::c_int {
        /* CELT complexity 0-4 doesn't have the pitch filter, which costs
        about 10%. */
        if complexity < 5 as libc::c_int {
            equiv = equiv * 9 as libc::c_int / 10 as libc::c_int
        }
    } else {
        /* Mode not known yet */
        /* Half the SILK loss*/
        equiv -= equiv * loss / (12 as libc::c_int * loss + 20 as libc::c_int)
    }
    return equiv;
}

unsafe extern "C" fn is_digital_silence(
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut lsb_depth: libc::c_int,
) -> libc::c_int {
    let mut silence: libc::c_int = 0 as libc::c_int;
    let mut sample_max: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
    sample_max = celt_maxabs16(pcm, frame_size * channels);
    silence = (sample_max
        <= 1 as libc::c_int as crate::arch_h::opus_val16
            / ((1 as libc::c_int) << lsb_depth) as libc::c_float) as libc::c_int;
    return silence;
}

unsafe extern "C" fn compute_frame_energy(
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut arch: libc::c_int,
) -> crate::arch_h::opus_val32 {
    let mut len: libc::c_int = frame_size * channels;
    return celt_inner_prod_c(pcm, pcm, len) / len as libc::c_float;
}
/* Decides if DTX should be turned on (=1) or off (=0) */

unsafe extern "C" fn decide_dtx_mode(
    mut activity_probability: libc::c_float,
    mut nb_no_activity_frames: *mut libc::c_int,
    mut peak_signal_energy: crate::arch_h::opus_val32,
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut channels: libc::c_int,
    mut is_silence: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut is_noise: libc::c_int = 0;
    let mut noise_energy: crate::arch_h::opus_val32 = 0.;
    let mut is_sufficiently_quiet: libc::c_int = 0;
    if is_silence == 0 {
        is_noise = (activity_probability < 0.1f32) as libc::c_int;
        if is_noise != 0 {
            noise_energy = compute_frame_energy(pcm, frame_size, channels, arch);
            is_sufficiently_quiet = (peak_signal_energy >= 316.23f32 * noise_energy) as libc::c_int
        }
    }
    if is_silence != 0 || is_noise != 0 && is_sufficiently_quiet != 0 {
        /* The number of consecutive DTX frames should be within the allowed bounds */
        *nb_no_activity_frames += 1;
        if *nb_no_activity_frames > 10 as libc::c_int {
            if *nb_no_activity_frames <= 10 as libc::c_int + 20 as libc::c_int {
                /* Valid frame for DTX! */
                return 1 as libc::c_int;
            } else {
                *nb_no_activity_frames = 10 as libc::c_int
            }
        }
    } else {
        *nb_no_activity_frames = 0 as libc::c_int
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn encode_multiframe_packet(
    mut st: *mut OpusEncoder,
    mut pcm: *const crate::arch_h::opus_val16,
    mut nb_frames: libc::c_int,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut out_data_bytes: crate::opus_types_h::opus_int32,
    mut to_celt: libc::c_int,
    mut lsb_depth: libc::c_int,
    mut float_api: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tmp_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bak_mode: libc::c_int = 0;
    let mut bak_bandwidth: libc::c_int = 0;
    let mut bak_channels: libc::c_int = 0;
    let mut bak_to_mono: libc::c_int = 0;
    let mut rp: *mut crate::opus_private_h::OpusRepacketizer =
        0 as *mut crate::opus_private_h::OpusRepacketizer;
    let mut max_header_bytes: libc::c_int = 0;
    let mut bytes_per_frame: crate::opus_types_h::opus_int32 = 0;
    let mut cbr_bytes: crate::opus_types_h::opus_int32 = 0;
    let mut repacketize_len: crate::opus_types_h::opus_int32 = 0;
    let mut tmp_len: libc::c_int = 0;
    /* Worst cases:
     * 2 frames: Code 2 with different compressed sizes
     * >2 frames: Code 3 VBR */
    max_header_bytes = if nb_frames == 2 as libc::c_int {
        3 as libc::c_int
    } else {
        (2 as libc::c_int) + (nb_frames - 1 as libc::c_int) * 2 as libc::c_int
    };
    if (*st).use_vbr != 0 || (*st).user_bitrate_bps == -(1 as libc::c_int) {
        repacketize_len = out_data_bytes
    } else {
        cbr_bytes = 3 as libc::c_int * (*st).bitrate_bps
            / (3 as libc::c_int * 8 as libc::c_int * (*st).Fs / (frame_size * nb_frames));
        repacketize_len = if cbr_bytes < out_data_bytes {
            cbr_bytes
        } else {
            out_data_bytes
        }
    }
    bytes_per_frame = if (1276 as libc::c_int)
        < 1 as libc::c_int + (repacketize_len - max_header_bytes) / nb_frames
    {
        1276 as libc::c_int
    } else {
        (1 as libc::c_int) + (repacketize_len - max_header_bytes) / nb_frames
    };
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul((nb_frames * bytes_per_frame) as libc::c_ulong) as usize,
    );
    tmp_data = fresh4.as_mut_ptr() as *mut libc::c_uchar;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_private_h::OpusRepacketizer>() as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong) as usize,
    );
    rp = fresh5.as_mut_ptr() as *mut crate::opus_private_h::OpusRepacketizer;

    crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_init(
        rp as *mut crate::opus_private_h::OpusRepacketizer,
    ) as *mut crate::opus_private_h::OpusRepacketizer;
    bak_mode = (*st).user_forced_mode;
    bak_bandwidth = (*st).user_bandwidth;
    bak_channels = (*st).force_channels;
    (*st).user_forced_mode = (*st).mode;
    (*st).user_bandwidth = (*st).bandwidth;
    (*st).force_channels = (*st).stream_channels;
    bak_to_mono = (*st).silk_mode.toMono;
    if bak_to_mono != 0 {
        (*st).force_channels = 1 as libc::c_int
    } else {
        (*st).prev_channels = (*st).stream_channels
    }
    i = 0 as libc::c_int;
    while i < nb_frames {
        (*st).silk_mode.toMono = 0 as libc::c_int;
        (*st).nonfinal_frame = (i < nb_frames - 1 as libc::c_int) as libc::c_int;
        /* When switching from SILK/Hybrid to CELT, only ask for a switch at the last frame */
        if to_celt != 0 && i == nb_frames - 1 as libc::c_int {
            (*st).user_forced_mode = 1002 as libc::c_int
        }
        tmp_len = opus_encode_native(
            st,
            pcm.offset((i * ((*st).channels * frame_size)) as isize),
            frame_size,
            tmp_data.offset((i * bytes_per_frame) as isize),
            bytes_per_frame,
            lsb_depth,
            0 as *const libc::c_void,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            None,
            float_api,
        );
        if tmp_len < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        ret = crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_cat(
            rp as *mut crate::opus_private_h::OpusRepacketizer,
            tmp_data.offset((i * bytes_per_frame) as isize),
            tmp_len,
        );
        if ret < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        i += 1
    }
    ret = crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_out_range_impl(
        rp as *mut crate::opus_private_h::OpusRepacketizer,
        0 as libc::c_int,
        nb_frames,
        data,
        repacketize_len,
        0 as libc::c_int,
        ((*st).use_vbr == 0) as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        return -(3 as libc::c_int);
    }
    /* Discard configs that were forced locally for the purpose of repacketization */
    (*st).user_forced_mode = bak_mode;
    (*st).user_bandwidth = bak_bandwidth;
    (*st).force_channels = bak_channels;
    (*st).silk_mode.toMono = bak_to_mono;
    return ret;
}

unsafe extern "C" fn compute_redundancy_bytes(
    mut max_data_bytes: crate::opus_types_h::opus_int32,
    mut bitrate_bps: crate::opus_types_h::opus_int32,
    mut frame_rate: libc::c_int,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut redundancy_bytes_cap: libc::c_int = 0;
    let mut redundancy_bytes: libc::c_int = 0;
    let mut redundancy_rate: crate::opus_types_h::opus_int32 = 0;
    let mut base_bits: libc::c_int = 0;
    let mut available_bits: crate::opus_types_h::opus_int32 = 0;
    base_bits = 40 as libc::c_int * channels + 20 as libc::c_int;
    /* Equivalent rate for 5 ms frames. */
    redundancy_rate = bitrate_bps + base_bits * (200 as libc::c_int - frame_rate);
    /* For VBR, further increase the bitrate if we can afford it. It's pretty short
    and we'll avoid artefacts. */
    redundancy_rate = 3 as libc::c_int * redundancy_rate / 2 as libc::c_int;
    redundancy_bytes = redundancy_rate / 1600 as libc::c_int;
    /* Compute the max rate we can use given CBR or VBR with cap. */
    available_bits = max_data_bytes * 8 as libc::c_int - 2 as libc::c_int * base_bits;
    redundancy_bytes_cap = (available_bits * 240 as libc::c_int
        / (240 as libc::c_int + 48000 as libc::c_int / frame_rate)
        + base_bits)
        / 8 as libc::c_int;
    redundancy_bytes = if redundancy_bytes < redundancy_bytes_cap {
        redundancy_bytes
    } else {
        redundancy_bytes_cap
    };
    /* It we can't get enough bits for redundancy to be worth it, rely on the decoder PLC. */
    if redundancy_bytes > 4 as libc::c_int + 8 as libc::c_int * channels {
        redundancy_bytes = if (257 as libc::c_int) < redundancy_bytes {
            257 as libc::c_int
        } else {
            redundancy_bytes
        }
    } else {
        redundancy_bytes = 0 as libc::c_int
    } /* Number of bytes to use for redundancy frame */
    return redundancy_bytes; /* Probability of voice in Q7 */
}
#[no_mangle]

pub unsafe extern "C" fn opus_encode_native(
    mut st: *mut OpusEncoder,
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut out_data_bytes: crate::opus_types_h::opus_int32,
    mut lsb_depth: libc::c_int,
    mut analysis_pcm: *const libc::c_void,
    mut analysis_size: crate::opus_types_h::opus_int32,
    mut c1: libc::c_int,
    mut c2: libc::c_int,
    mut analysis_channels: libc::c_int,
    mut downmix: crate::opus_private_h::downmix_func,
    mut float_api: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void; /* Max bitrate we're allowed to use */
    let mut celt_enc: *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder =
        0 as *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder; /* Max number of bytes we're allowed to use */
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nBytes: crate::opus_types_h::opus_int32 = 0;
    let mut enc: crate::src::opus_1_2_1::celt::entcode::ec_enc =
        crate::src::opus_1_2_1::celt::entcode::ec_enc {
            buf: 0 as *mut libc::c_uchar,
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
    let mut bytes_target: libc::c_int = 0;
    let mut prefill: libc::c_int = 0 as libc::c_int;
    let mut start_band: libc::c_int = 0 as libc::c_int;
    let mut redundancy: libc::c_int = 0 as libc::c_int;
    let mut redundancy_bytes: libc::c_int = 0 as libc::c_int;
    let mut celt_to_silk: libc::c_int = 0 as libc::c_int;
    let mut pcm_buf: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut nb_compr_bytes: libc::c_int = 0;
    let mut to_celt: libc::c_int = 0 as libc::c_int;
    let mut redundant_rng: crate::opus_types_h::opus_uint32 =
        0 as libc::c_int as crate::opus_types_h::opus_uint32;
    let mut cutoff_Hz: libc::c_int = 0;
    let mut hp_freq_smth1: libc::c_int = 0;
    let mut voice_est: libc::c_int = 0;
    let mut equiv_rate: crate::opus_types_h::opus_int32 = 0;
    let mut delay_compensation: libc::c_int = 0;
    let mut frame_rate: libc::c_int = 0;
    let mut max_rate: crate::opus_types_h::opus_int32 = 0;
    let mut curr_bandwidth: libc::c_int = 0;
    let mut HB_gain: crate::arch_h::opus_val16 = 0.;
    let mut max_data_bytes: crate::opus_types_h::opus_int32 = 0;
    let mut total_buffer: libc::c_int = 0;
    let mut stereo_width: crate::arch_h::opus_val16 = 0.;
    let mut celt_mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut analysis_info: crate::celt_h::AnalysisInfo = crate::celt_h::AnalysisInfo {
        valid: 0,
        tonality: 0.,
        tonality_slope: 0.,
        noisiness: 0.,
        activity: 0.,
        music_prob: 0.,
        vad_prob: 0.,
        bandwidth: 0,
        activity_probability: 0.,
        leak_boost: [0; 19],
    };
    let mut analysis_read_pos_bak: libc::c_int = -(1 as libc::c_int);
    let mut analysis_read_subframe_bak: libc::c_int = -(1 as libc::c_int);
    let mut is_silence: libc::c_int = 0 as libc::c_int;
    let mut tmp_prefill: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    max_data_bytes = if (1276 as libc::c_int) < out_data_bytes {
        1276 as libc::c_int
    } else {
        out_data_bytes
    };
    (*st).rangeFinal = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
    if frame_size <= 0 as libc::c_int || max_data_bytes <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /* Cannot encode 100 ms in 1 byte */
    if max_data_bytes == 1 as libc::c_int && (*st).Fs == frame_size * 10 as libc::c_int {
        return -(2 as libc::c_int);
    }
    silk_enc =
        (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize) as *mut libc::c_void;
    celt_enc = (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize)
        as *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder;
    if (*st).application == 2051 as libc::c_int {
        delay_compensation = 0 as libc::c_int
    } else {
        delay_compensation = (*st).delay_compensation
    }
    lsb_depth = if lsb_depth < (*st).lsb_depth {
        lsb_depth
    } else {
        (*st).lsb_depth
    };
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        10015 as libc::c_int,
        (&mut celt_mode as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode).offset(
            (&mut celt_mode as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode)
                .wrapping_offset_from(
                    &mut celt_mode
                        as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                ) as libc::c_long as isize,
        ),
    );
    analysis_info.valid = 0 as libc::c_int;
    if (*st).silk_mode.complexity >= 7 as libc::c_int && (*st).Fs >= 16000 as libc::c_int {
        if is_digital_silence(pcm, frame_size, (*st).channels, lsb_depth) != 0 {
            is_silence = 1 as libc::c_int
        } else {
            analysis_read_pos_bak = (*st).analysis.read_pos;
            analysis_read_subframe_bak = (*st).analysis.read_subframe;
            crate::src::opus_1_2_1::src::analysis::run_analysis(
                &mut (*st).analysis as *mut _
                    as *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
                celt_mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                analysis_pcm,
                analysis_size,
                frame_size,
                c1,
                c2,
                analysis_channels,
                (*st).Fs,
                lsb_depth,
                downmix,
                &mut analysis_info as *mut _ as *mut crate::celt_h::AnalysisInfo,
            );
        }
        /* Track the peak signal energy */
        if is_silence == 0 && analysis_info.activity_probability > 0.1f32 {
            (*st).peak_signal_energy = if 0.999f32 * (*st).peak_signal_energy
                > compute_frame_energy(pcm, frame_size, (*st).channels, (*st).arch)
            {
                (0.999f32) * (*st).peak_signal_energy
            } else {
                compute_frame_energy(pcm, frame_size, (*st).channels, (*st).arch)
            }
        }
    }
    /* Reset voice_ratio if this frame is not silent or if analysis is disabled.
     * Otherwise, preserve voice_ratio from the last non-silent frame */
    if is_silence == 0 {
        (*st).voice_ratio = -(1 as libc::c_int)
    }
    (*st).detected_bandwidth = 0 as libc::c_int;
    if analysis_info.valid != 0 {
        let mut analysis_bandwidth: libc::c_int = 0;
        if (*st).signal_type == -(1000 as libc::c_int) {
            (*st).voice_ratio = crate::stdlib::floor(
                0.5f64
                    + (100 as libc::c_int as libc::c_float
                        * (1 as libc::c_int as libc::c_float - analysis_info.music_prob))
                        as libc::c_double,
            ) as libc::c_int
        }
        analysis_bandwidth = analysis_info.bandwidth;
        if analysis_bandwidth <= 12 as libc::c_int {
            (*st).detected_bandwidth = 1101 as libc::c_int
        } else if analysis_bandwidth <= 14 as libc::c_int {
            (*st).detected_bandwidth = 1102 as libc::c_int
        } else if analysis_bandwidth <= 16 as libc::c_int {
            (*st).detected_bandwidth = 1103 as libc::c_int
        } else if analysis_bandwidth <= 18 as libc::c_int {
            (*st).detected_bandwidth = 1104 as libc::c_int
        } else {
            (*st).detected_bandwidth = 1105 as libc::c_int
        }
    }
    if (*st).channels == 2 as libc::c_int && (*st).force_channels != 1 as libc::c_int {
        stereo_width = compute_stereo_width(pcm, frame_size, (*st).Fs, &mut (*st).width_mem)
    } else {
        stereo_width = 0 as libc::c_int as crate::arch_h::opus_val16
    }
    total_buffer = delay_compensation;
    (*st).bitrate_bps = user_bitrate_to_bitrate(st, frame_size, max_data_bytes);
    frame_rate = (*st).Fs / frame_size;
    if (*st).use_vbr == 0 {
        let mut cbrBytes: libc::c_int = 0;
        /* Multiply by 12 to make sure the division is exact. */
        let mut frame_rate12: libc::c_int = 12 as libc::c_int * (*st).Fs / frame_size;
        /* We need to make sure that "int" values always fit in 16 bits. */
        cbrBytes = if (12 as libc::c_int * (*st).bitrate_bps / 8 as libc::c_int
            + frame_rate12 / 2 as libc::c_int)
            / frame_rate12
            < max_data_bytes
        {
            (12 as libc::c_int * (*st).bitrate_bps / 8 as libc::c_int
                + frame_rate12 / 2 as libc::c_int)
                / frame_rate12
        } else {
            max_data_bytes
        };
        (*st).bitrate_bps = cbrBytes * frame_rate12 * 8 as libc::c_int / 12 as libc::c_int;
        /* Make sure we provide at least one byte to avoid failing. */
        max_data_bytes = if 1 as libc::c_int > cbrBytes {
            1 as libc::c_int
        } else {
            cbrBytes
        }
    }
    if max_data_bytes < 3 as libc::c_int
        || (*st).bitrate_bps < 3 as libc::c_int * frame_rate * 8 as libc::c_int
        || frame_rate < 50 as libc::c_int
            && (max_data_bytes * frame_rate < 300 as libc::c_int
                || (*st).bitrate_bps < 2400 as libc::c_int)
    {
        /*If the space is too low to do something useful, emit 'PLC' frames.*/
        let mut tocmode: libc::c_int = (*st).mode;
        let mut bw: libc::c_int = if (*st).bandwidth == 0 as libc::c_int {
            1101 as libc::c_int
        } else {
            (*st).bandwidth
        };
        let mut packet_code: libc::c_int = 0 as libc::c_int;
        let mut num_multiframes: libc::c_int = 0 as libc::c_int;
        if tocmode == 0 as libc::c_int {
            tocmode = 1000 as libc::c_int
        }
        if frame_rate > 100 as libc::c_int {
            tocmode = 1002 as libc::c_int
        }
        /* 40 ms -> 2 x 20 ms if in CELT_ONLY or HYBRID mode */
        if frame_rate == 25 as libc::c_int && tocmode != 1000 as libc::c_int {
            frame_rate = 50 as libc::c_int;
            packet_code = 1 as libc::c_int
        }
        /* >= 60 ms frames */
        if frame_rate <= 16 as libc::c_int {
            /* 1 x 60 ms, 2 x 40 ms, 2 x 60 ms */
            if out_data_bytes == 1 as libc::c_int
                || tocmode == 1000 as libc::c_int && frame_rate != 10 as libc::c_int
            {
                tocmode = 1000 as libc::c_int;
                packet_code = (frame_rate <= 12 as libc::c_int) as libc::c_int;
                frame_rate = if frame_rate == 12 as libc::c_int {
                    25 as libc::c_int
                } else {
                    16 as libc::c_int
                }
            } else {
                num_multiframes = 50 as libc::c_int / frame_rate;
                frame_rate = 50 as libc::c_int;
                packet_code = 3 as libc::c_int
            }
        }
        if tocmode == 1000 as libc::c_int && bw > 1103 as libc::c_int {
            bw = 1103 as libc::c_int
        } else if tocmode == 1002 as libc::c_int && bw == 1102 as libc::c_int {
            bw = 1101 as libc::c_int
        } else if tocmode == 1001 as libc::c_int && bw <= 1104 as libc::c_int {
            bw = 1104 as libc::c_int
        }
        *data.offset(0 as libc::c_int as isize) =
            gen_toc(tocmode, frame_rate, bw, (*st).stream_channels);
        let ref mut fresh6 = *data.offset(0 as libc::c_int as isize);
        *fresh6 = (*fresh6 as libc::c_int | packet_code) as libc::c_uchar;
        ret = if packet_code <= 1 as libc::c_int {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        };
        max_data_bytes = if max_data_bytes > ret {
            max_data_bytes
        } else {
            ret
        };
        if packet_code == 3 as libc::c_int {
            *data.offset(1 as libc::c_int as isize) = num_multiframes as libc::c_uchar
        }
        if (*st).use_vbr == 0 {
            ret = crate::src::opus_1_2_1::src::repacketizer::opus_packet_pad(
                data,
                ret,
                max_data_bytes,
            );
            if ret == 0 as libc::c_int {
                ret = max_data_bytes
            } else {
                ret = -(3 as libc::c_int)
            }
        }
        return ret;
    }
    max_rate = frame_rate * max_data_bytes * 8 as libc::c_int;
    /* Equivalent 20-ms rate for mode/channel/bandwidth decisions */
    equiv_rate = compute_equiv_rate(
        (*st).bitrate_bps,
        (*st).channels,
        (*st).Fs / frame_size,
        (*st).use_vbr,
        0 as libc::c_int,
        (*st).silk_mode.complexity,
        (*st).silk_mode.packetLossPercentage,
    );
    if (*st).signal_type == 3001 as libc::c_int {
        voice_est = 127 as libc::c_int
    } else if (*st).signal_type == 3002 as libc::c_int {
        voice_est = 0 as libc::c_int
    } else if (*st).voice_ratio >= 0 as libc::c_int {
        voice_est = (*st).voice_ratio * 327 as libc::c_int >> 8 as libc::c_int;
        /* For AUDIO, never be more than 90% confident of having speech */
        if (*st).application == 2049 as libc::c_int {
            voice_est = if voice_est < 115 as libc::c_int {
                voice_est
            } else {
                115 as libc::c_int
            }
        }
    } else if (*st).application == 2048 as libc::c_int {
        voice_est = 115 as libc::c_int
    } else {
        voice_est = 48 as libc::c_int
    }
    if (*st).force_channels != -(1000 as libc::c_int) && (*st).channels == 2 as libc::c_int {
        (*st).stream_channels = (*st).force_channels
    } else if (*st).channels == 2 as libc::c_int {
        let mut stereo_threshold: crate::opus_types_h::opus_int32 = 0;
        stereo_threshold = stereo_music_threshold
            + (voice_est * voice_est * (stereo_voice_threshold - stereo_music_threshold)
                >> 14 as libc::c_int);
        if (*st).stream_channels == 2 as libc::c_int {
            stereo_threshold -= 1000 as libc::c_int
        } else {
            stereo_threshold += 1000 as libc::c_int
        }
        (*st).stream_channels = if equiv_rate > stereo_threshold {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else {
        (*st).stream_channels = (*st).channels
    }
    /* Rate-dependent mono-stereo decision */
    /* Update equivalent rate for channels decision. */
    equiv_rate = compute_equiv_rate(
        (*st).bitrate_bps,
        (*st).stream_channels,
        (*st).Fs / frame_size,
        (*st).use_vbr,
        0 as libc::c_int,
        (*st).silk_mode.complexity,
        (*st).silk_mode.packetLossPercentage,
    );
    /* Mode selection depending on application and signal type */
    if (*st).application == 2051 as libc::c_int {
        (*st).mode = 1002 as libc::c_int
    } else if (*st).user_forced_mode == -(1000 as libc::c_int) {
        let mut mode_voice: crate::opus_types_h::opus_int32 = 0;
        let mut mode_music: crate::opus_types_h::opus_int32 = 0;
        let mut threshold: crate::opus_types_h::opus_int32 = 0;
        /* Interpolate based on stereo width */
        mode_voice = ((1.0f32 - stereo_width)
            * mode_thresholds[0 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_float
            + stereo_width
                * mode_thresholds[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_float) as crate::opus_types_h::opus_int32;
        mode_music = ((1.0f32 - stereo_width)
            * mode_thresholds[1 as libc::c_int as usize][1 as libc::c_int as usize]
                as libc::c_float
            + stereo_width
                * mode_thresholds[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_float) as crate::opus_types_h::opus_int32;
        /* Interpolate based on speech/music probability */
        threshold =
            mode_music + (voice_est * voice_est * (mode_voice - mode_music) >> 14 as libc::c_int);
        /* Bias towards SILK for VoIP because of some useful features */
        if (*st).application == 2048 as libc::c_int {
            threshold += 8000 as libc::c_int
        }
        /*printf("%f %d\n", stereo_width/(float)Q15ONE, threshold);*/
        /* Hysteresis */
        if (*st).prev_mode == 1002 as libc::c_int {
            threshold -= 4000 as libc::c_int
        } else if (*st).prev_mode > 0 as libc::c_int {
            threshold += 4000 as libc::c_int
        }
        (*st).mode = if equiv_rate >= threshold {
            1002 as libc::c_int
        } else {
            1000 as libc::c_int
        };
        /* When FEC is enabled and there's enough packet loss, use SILK */
        if (*st).silk_mode.useInBandFEC != 0
            && (*st).silk_mode.packetLossPercentage
                > 128 as libc::c_int - voice_est >> 4 as libc::c_int
        {
            (*st).mode = 1000 as libc::c_int
        }
        /* When encoding voice and DTX is enabled but the generalized DTX cannot be used,
        because of complexity and sampling frequency settings, switch to SILK DTX and
        set the encoder to SILK mode */
        (*st).silk_mode.useDTX =
            ((*st).use_dtx != 0 && !(analysis_info.valid != 0 || is_silence != 0)) as libc::c_int;
        if (*st).silk_mode.useDTX != 0 && voice_est > 100 as libc::c_int {
            (*st).mode = 1000 as libc::c_int
        }
        /* If max_data_bytes represents less than 6 kb/s, switch to CELT-only mode */
        if max_data_bytes
            < (if frame_rate > 50 as libc::c_int {
                9000 as libc::c_int
            } else {
                6000 as libc::c_int
            }) * frame_size
                / ((*st).Fs * 8 as libc::c_int)
        {
            (*st).mode = 1002 as libc::c_int
        }
    } else {
        (*st).mode = (*st).user_forced_mode
    }
    /* Override the chosen mode to make sure we meet the requested frame size */
    if (*st).mode != 1002 as libc::c_int && frame_size < (*st).Fs / 100 as libc::c_int {
        (*st).mode = 1002 as libc::c_int
    }
    if (*st).lfe != 0 {
        (*st).mode = 1002 as libc::c_int
    }
    if (*st).prev_mode > 0 as libc::c_int
        && ((*st).mode != 1002 as libc::c_int && (*st).prev_mode == 1002 as libc::c_int
            || (*st).mode == 1002 as libc::c_int && (*st).prev_mode != 1002 as libc::c_int)
    {
        redundancy = 1 as libc::c_int;
        celt_to_silk = ((*st).mode != 1002 as libc::c_int) as libc::c_int;
        if celt_to_silk == 0 {
            /* Switch to SILK/hybrid if frame size is 10 ms or more*/
            if frame_size >= (*st).Fs / 100 as libc::c_int {
                (*st).mode = (*st).prev_mode;
                to_celt = 1 as libc::c_int
            } else {
                redundancy = 0 as libc::c_int
            }
        }
    }
    /* When encoding multiframes, we can ask for a switch to CELT only in the last frame. This switch
     * is processed above as the requested mode shouldn't interrupt stereo->mono transition. */
    if (*st).stream_channels == 1 as libc::c_int
        && (*st).prev_channels == 2 as libc::c_int
        && (*st).silk_mode.toMono == 0 as libc::c_int
        && (*st).mode != 1002 as libc::c_int
        && (*st).prev_mode != 1002 as libc::c_int
    {
        /* Delay stereo->mono transition by two frames so that SILK can do a smooth downmix */
        (*st).silk_mode.toMono = 1 as libc::c_int;
        (*st).stream_channels = 2 as libc::c_int
    } else {
        (*st).silk_mode.toMono = 0 as libc::c_int
    }
    /* Update equivalent rate with mode decision. */
    equiv_rate = compute_equiv_rate(
        (*st).bitrate_bps,
        (*st).stream_channels,
        (*st).Fs / frame_size,
        (*st).use_vbr,
        (*st).mode,
        (*st).silk_mode.complexity,
        (*st).silk_mode.packetLossPercentage,
    );
    if (*st).mode != 1002 as libc::c_int && (*st).prev_mode == 1002 as libc::c_int {
        let mut dummy: crate::control_h::silk_EncControlStruct =
            crate::control_h::silk_EncControlStruct {
                nChannelsAPI: 0,
                nChannelsInternal: 0,
                API_sampleRate: 0,
                maxInternalSampleRate: 0,
                minInternalSampleRate: 0,
                desiredInternalSampleRate: 0,
                payloadSize_ms: 0,
                bitRate: 0,
                packetLossPercentage: 0,
                complexity: 0,
                useInBandFEC: 0,
                LBRR_coded: 0,
                useDTX: 0,
                useCBR: 0,
                maxBits: 0,
                toMono: 0,
                opusCanSwitch: 0,
                reducedDependency: 0,
                internalSampleRate: 0,
                allowBandwidthSwitch: 0,
                inWBmodeWithoutVariableLP: 0,
                stereoWidth_Q14: 0,
                switchReady: 0,
                signalType: 0,
                offset: 0,
            };
        crate::src::opus_1_2_1::silk::enc_API::silk_InitEncoder(
            silk_enc,
            (*st).arch,
            &mut dummy as *mut _ as *mut crate::control_h::silk_EncControlStruct,
        );
        prefill = 1 as libc::c_int
    }
    /* Automatic (rate-dependent) bandwidth selection */
    if (*st).mode == 1002 as libc::c_int
        || (*st).first != 0
        || (*st).silk_mode.allowBandwidthSwitch != 0
    {
        let mut voice_bandwidth_thresholds: *const crate::opus_types_h::opus_int32 =
            0 as *const crate::opus_types_h::opus_int32;
        let mut music_bandwidth_thresholds: *const crate::opus_types_h::opus_int32 =
            0 as *const crate::opus_types_h::opus_int32;
        let mut bandwidth_thresholds: [crate::opus_types_h::opus_int32; 8] = [0; 8];
        let mut bandwidth: libc::c_int = 1105 as libc::c_int;
        if (*st).channels == 2 as libc::c_int && (*st).force_channels != 1 as libc::c_int {
            voice_bandwidth_thresholds = stereo_voice_bandwidth_thresholds.as_ptr();
            music_bandwidth_thresholds = stereo_music_bandwidth_thresholds.as_ptr()
        } else {
            voice_bandwidth_thresholds = mono_voice_bandwidth_thresholds.as_ptr();
            music_bandwidth_thresholds = mono_music_bandwidth_thresholds.as_ptr()
        }
        /* Interpolate bandwidth thresholds depending on voice estimation */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            bandwidth_thresholds[i as usize] = *music_bandwidth_thresholds.offset(i as isize)
                + (voice_est
                    * voice_est
                    * (*voice_bandwidth_thresholds.offset(i as isize)
                        - *music_bandwidth_thresholds.offset(i as isize))
                    >> 14 as libc::c_int);
            i += 1
        }
        loop {
            let mut threshold_0: libc::c_int = 0;
            let mut hysteresis: libc::c_int = 0;
            threshold_0 = bandwidth_thresholds
                [(2 as libc::c_int * (bandwidth - 1102 as libc::c_int)) as usize];
            hysteresis = bandwidth_thresholds[(2 as libc::c_int * (bandwidth - 1102 as libc::c_int)
                + 1 as libc::c_int) as usize];
            if (*st).first == 0 {
                if (*st).auto_bandwidth >= bandwidth {
                    threshold_0 -= hysteresis
                } else {
                    threshold_0 += hysteresis
                }
            }
            if equiv_rate >= threshold_0 {
                break;
            }
            bandwidth -= 1;
            if !(bandwidth > 1101 as libc::c_int) {
                break;
            }
        }
        (*st).auto_bandwidth = bandwidth;
        (*st).bandwidth = (*st).auto_bandwidth;
        /* Prevents any transition to SWB/FB until the SILK layer has fully
        switched to WB mode and turned the variable LP filter off */
        if (*st).first == 0
            && (*st).mode != 1002 as libc::c_int
            && (*st).silk_mode.inWBmodeWithoutVariableLP == 0
            && (*st).bandwidth > 1103 as libc::c_int
        {
            (*st).bandwidth = 1103 as libc::c_int
        }
    }
    if (*st).bandwidth > (*st).max_bandwidth {
        (*st).bandwidth = (*st).max_bandwidth
    }
    if (*st).user_bandwidth != -(1000 as libc::c_int) {
        (*st).bandwidth = (*st).user_bandwidth
    }
    /* This prevents us from using hybrid at unsafe CBR/max rates */
    if (*st).mode != 1002 as libc::c_int && max_rate < 15000 as libc::c_int {
        (*st).bandwidth = if (*st).bandwidth < 1103 as libc::c_int {
            (*st).bandwidth
        } else {
            1103 as libc::c_int
        }
    }
    /* Prevents Opus from wasting bits on frequencies that are above
    the Nyquist rate of the input signal */
    if (*st).Fs <= 24000 as libc::c_int && (*st).bandwidth > 1104 as libc::c_int {
        (*st).bandwidth = 1104 as libc::c_int
    }
    if (*st).Fs <= 16000 as libc::c_int && (*st).bandwidth > 1103 as libc::c_int {
        (*st).bandwidth = 1103 as libc::c_int
    }
    if (*st).Fs <= 12000 as libc::c_int && (*st).bandwidth > 1102 as libc::c_int {
        (*st).bandwidth = 1102 as libc::c_int
    }
    if (*st).Fs <= 8000 as libc::c_int && (*st).bandwidth > 1101 as libc::c_int {
        (*st).bandwidth = 1101 as libc::c_int
    }
    /* Use detected bandwidth to reduce the encoded bandwidth. */
    if (*st).detected_bandwidth != 0 && (*st).user_bandwidth == -(1000 as libc::c_int) {
        let mut min_detected_bandwidth: libc::c_int = 0;
        /* Makes bandwidth detection more conservative just in case the detector
        gets it wrong when we could have coded a high bandwidth transparently.
        When operating in SILK/hybrid mode, we don't go below wideband to avoid
        more complicated switches that require redundancy. */
        if equiv_rate <= 18000 as libc::c_int * (*st).stream_channels
            && (*st).mode == 1002 as libc::c_int
        {
            min_detected_bandwidth = 1101 as libc::c_int
        } else if equiv_rate <= 24000 as libc::c_int * (*st).stream_channels
            && (*st).mode == 1002 as libc::c_int
        {
            min_detected_bandwidth = 1102 as libc::c_int
        } else if equiv_rate <= 30000 as libc::c_int * (*st).stream_channels {
            min_detected_bandwidth = 1103 as libc::c_int
        } else if equiv_rate <= 44000 as libc::c_int * (*st).stream_channels {
            min_detected_bandwidth = 1104 as libc::c_int
        } else {
            min_detected_bandwidth = 1105 as libc::c_int
        }
        (*st).detected_bandwidth = if (*st).detected_bandwidth > min_detected_bandwidth {
            (*st).detected_bandwidth
        } else {
            min_detected_bandwidth
        };
        (*st).bandwidth = if (*st).bandwidth < (*st).detected_bandwidth {
            (*st).bandwidth
        } else {
            (*st).detected_bandwidth
        }
    }
    (*st).silk_mode.LBRR_coded = decide_fec(
        (*st).silk_mode.useInBandFEC,
        (*st).silk_mode.packetLossPercentage,
        (*st).silk_mode.LBRR_coded,
        (*st).mode,
        &mut (*st).bandwidth,
        equiv_rate,
    );
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        4036 as libc::c_int,
        lsb_depth,
    );
    /* CELT mode doesn't support mediumband, use wideband instead */
    if (*st).mode == 1002 as libc::c_int && (*st).bandwidth == 1102 as libc::c_int {
        (*st).bandwidth = 1103 as libc::c_int
    }
    if (*st).lfe != 0 {
        (*st).bandwidth = 1101 as libc::c_int
    }
    curr_bandwidth = (*st).bandwidth;
    /* Chooses the appropriate mode for speech
     *NEVER* switch to/from CELT-only mode here as this will invalidate some assumptions */
    if (*st).mode == 1000 as libc::c_int && curr_bandwidth > 1103 as libc::c_int {
        (*st).mode = 1001 as libc::c_int
    }
    if (*st).mode == 1001 as libc::c_int && curr_bandwidth <= 1103 as libc::c_int {
        (*st).mode = 1000 as libc::c_int
    }
    /* Can't support higher than >60 ms frames, and >20 ms when in Hybrid or CELT-only modes */
    if frame_size > (*st).Fs / 50 as libc::c_int && (*st).mode != 1000 as libc::c_int
        || frame_size > 3 as libc::c_int * (*st).Fs / 50 as libc::c_int
    {
        let mut enc_frame_size: libc::c_int = 0;
        let mut nb_frames: libc::c_int = 0;
        if (*st).mode == 1000 as libc::c_int {
            if frame_size == 2 as libc::c_int * (*st).Fs / 25 as libc::c_int {
                /* 80 ms -> 2x 40 ms */
                enc_frame_size = (*st).Fs / 25 as libc::c_int
            } else if frame_size == 3 as libc::c_int * (*st).Fs / 25 as libc::c_int {
                /* 120 ms -> 2x 60 ms */
                enc_frame_size = 3 as libc::c_int * (*st).Fs / 50 as libc::c_int
            } else {
                /* 100 ms -> 5x 20 ms */
                enc_frame_size = (*st).Fs / 50 as libc::c_int
            }
        } else {
            enc_frame_size = (*st).Fs / 50 as libc::c_int
        }
        nb_frames = frame_size / enc_frame_size;
        if analysis_read_pos_bak != -(1 as libc::c_int) {
            (*st).analysis.read_pos = analysis_read_pos_bak;
            (*st).analysis.read_subframe = analysis_read_subframe_bak
        }
        ret = encode_multiframe_packet(
            st,
            pcm,
            nb_frames,
            enc_frame_size,
            data,
            out_data_bytes,
            to_celt,
            lsb_depth,
            float_api,
        );
        return ret;
    }
    /* For the first frame at a new SILK bandwidth */
    if (*st).silk_bw_switch != 0 {
        redundancy = 1 as libc::c_int;
        celt_to_silk = 1 as libc::c_int;
        (*st).silk_bw_switch = 0 as libc::c_int;
        prefill = 1 as libc::c_int
    }
    /* If we decided to go with CELT, make sure redundancy is off, no matter what
    we decided earlier. */
    if (*st).mode == 1002 as libc::c_int {
        redundancy = 0 as libc::c_int
    }
    if redundancy != 0 {
        redundancy_bytes = compute_redundancy_bytes(
            max_data_bytes,
            (*st).bitrate_bps,
            frame_rate,
            (*st).stream_channels,
        );
        if redundancy_bytes == 0 as libc::c_int {
            redundancy = 0 as libc::c_int
        }
    }
    /* printf("%d %d %d %d\n", st->bitrate_bps, st->stream_channels, st->mode, curr_bandwidth); */
    bytes_target = (if max_data_bytes - redundancy_bytes
        < (*st).bitrate_bps * frame_size / ((*st).Fs * 8 as libc::c_int)
    {
        (max_data_bytes) - redundancy_bytes
    } else {
        ((*st).bitrate_bps * frame_size) / ((*st).Fs * 8 as libc::c_int)
    }) - 1 as libc::c_int;
    data = data.offset(1 as libc::c_int as isize);
    crate::src::opus_1_2_1::celt::entenc::ec_enc_init(
        &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        data,
        (max_data_bytes - 1 as libc::c_int) as crate::opus_types_h::opus_uint32,
    );
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(((total_buffer + frame_size) * (*st).channels) as libc::c_ulong)
            as usize,
    );
    pcm_buf = fresh7.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memcpy(
        pcm_buf as *mut libc::c_void,
        &mut *(*st)
            .delay_buffer
            .as_mut_ptr()
            .offset((((*st).encoder_buffer - total_buffer) * (*st).channels) as isize)
            as *mut crate::arch_h::opus_val16 as *const libc::c_void,
        ((total_buffer * (*st).channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * pcm_buf.wrapping_offset_from(
                        &mut *(*st).delay_buffer.as_mut_ptr().offset(
                            (((*st).encoder_buffer - total_buffer) * (*st).channels) as isize,
                        ),
                    ) as libc::c_long) as libc::c_ulong,
            ),
    );
    if (*st).mode == 1002 as libc::c_int {
        hp_freq_smth1 = ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
            as crate::opus_types_h::opus_uint32)
            << 8 as libc::c_int) as crate::opus_types_h::opus_int32
    } else {
        hp_freq_smth1 = (*(silk_enc as *mut crate::structs_FLP_h::silk_encoder)).state_Fxx
            [0 as libc::c_int as usize]
            .sCmn
            .variable_HP_smth1_Q15
    }
    (*st).variable_HP_smth2_Q15 = ((*st).variable_HP_smth2_Q15 as libc::c_longlong
        + ((hp_freq_smth1 - (*st).variable_HP_smth2_Q15) as libc::c_longlong
            * ((0.015f32
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int))
        as crate::opus_types_h::opus_int32;
    /* convert from log scale to Hertz */
    cutoff_Hz = crate::src::opus_1_2_1::silk::log2lin::silk_log2lin(
        (*st).variable_HP_smth2_Q15 >> 8 as libc::c_int,
    );
    if (*st).application == 2048 as libc::c_int {
        hp_cutoff(
            pcm,
            cutoff_Hz,
            &mut *pcm_buf.offset((total_buffer * (*st).channels) as isize),
            (*st).hp_mem.as_mut_ptr(),
            frame_size,
            (*st).channels,
            (*st).Fs,
            (*st).arch,
        );
    } else {
        dc_reject(
            pcm,
            3 as libc::c_int,
            &mut *pcm_buf.offset((total_buffer * (*st).channels) as isize),
            (*st).hp_mem.as_mut_ptr(),
            frame_size,
            (*st).channels,
            (*st).Fs,
        );
    }
    if float_api != 0 {
        let mut sum: crate::arch_h::opus_val32 = 0.;
        sum = celt_inner_prod_c(
            &mut *pcm_buf.offset((total_buffer * (*st).channels) as isize),
            &mut *pcm_buf.offset((total_buffer * (*st).channels) as isize),
            frame_size * (*st).channels,
        );
        /* This should filter out both NaNs and ridiculous signals that could
        cause NaNs further down. */
        if !(sum < 1e9f32) || celt_isnan(sum) != 0 {
            crate::stdlib::memset(
                &mut *pcm_buf.offset((total_buffer * (*st).channels) as isize)
                    as *mut crate::arch_h::opus_val16 as *mut libc::c_void,
                0 as libc::c_int,
                ((frame_size * (*st).channels) as libc::c_ulong).wrapping_mul(
                    ::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong,
                ),
            );
            (*st).hp_mem[3 as libc::c_int as usize] = 0 as libc::c_int as crate::arch_h::opus_val32;
            (*st).hp_mem[2 as libc::c_int as usize] = (*st).hp_mem[3 as libc::c_int as usize];
            (*st).hp_mem[1 as libc::c_int as usize] = (*st).hp_mem[2 as libc::c_int as usize];
            (*st).hp_mem[0 as libc::c_int as usize] = (*st).hp_mem[1 as libc::c_int as usize]
        }
    }
    /* SILK processing */
    HB_gain = 1.0f32;
    if (*st).mode != 1002 as libc::c_int {
        let mut total_bitRate: crate::opus_types_h::opus_int32 = 0;
        let mut celt_rate: crate::opus_types_h::opus_int32 = 0;
        let mut pcm_silk: *mut crate::opus_types_h::opus_int16 =
            0 as *mut crate::opus_types_h::opus_int16;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
                .wrapping_mul(((*st).channels * frame_size) as libc::c_ulong) as usize,
        );
        pcm_silk = fresh8.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
        /* Distribute bits between SILK and CELT */
        total_bitRate = 8 as libc::c_int * bytes_target * frame_rate;
        if (*st).mode == 1001 as libc::c_int {
            /* Base rate for SILK */
            (*st).silk_mode.bitRate = compute_silk_rate_for_hybrid(
                total_bitRate,
                curr_bandwidth,
                ((*st).Fs == 50 as libc::c_int * frame_size) as libc::c_int,
                (*st).use_vbr,
                (*st).silk_mode.LBRR_coded,
            );
            if (*st).energy_masking.is_null() {
                /* Increasingly attenuate high band when it gets allocated fewer bits */
                celt_rate = total_bitRate - (*st).silk_mode.bitRate;
                HB_gain = 1.0f32
                    - celt_exp2(
                        -celt_rate as libc::c_float
                            * (1.0f32 / 1024 as libc::c_int as libc::c_float),
                    )
            }
        } else {
            /* SILK gets all bits */
            (*st).silk_mode.bitRate = total_bitRate
        }
        /* Surround masking for SILK */
        if !(*st).energy_masking.is_null() && (*st).use_vbr != 0 && (*st).lfe == 0 {
            let mut mask_sum: crate::arch_h::opus_val32 =
                0 as libc::c_int as crate::arch_h::opus_val32;
            let mut masking_depth: crate::arch_h::opus_val16 = 0.;
            let mut rate_offset: crate::opus_types_h::opus_int32 = 0;
            let mut c: libc::c_int = 0;
            let mut end: libc::c_int = 17 as libc::c_int;
            let mut srate: crate::opus_types_h::opus_int16 =
                16000 as libc::c_int as crate::opus_types_h::opus_int16;
            if (*st).bandwidth == 1101 as libc::c_int {
                end = 13 as libc::c_int;
                srate = 8000 as libc::c_int as crate::opus_types_h::opus_int16
            } else if (*st).bandwidth == 1102 as libc::c_int {
                end = 15 as libc::c_int;
                srate = 12000 as libc::c_int as crate::opus_types_h::opus_int16
            }
            c = 0 as libc::c_int;
            while c < (*st).channels {
                i = 0 as libc::c_int;
                while i < end {
                    let mut mask: crate::arch_h::opus_val16 = 0.;
                    mask = if (if *(*st)
                        .energy_masking
                        .offset((21 as libc::c_int * c + i) as isize)
                        < 0.5f32
                    {
                        *(*st)
                            .energy_masking
                            .offset((21 as libc::c_int * c + i) as isize)
                    } else {
                        0.5f32
                    }) > -2.0f32
                    {
                        if *(*st)
                            .energy_masking
                            .offset((21 as libc::c_int * c + i) as isize)
                            < 0.5f32
                        {
                            *(*st)
                                .energy_masking
                                .offset((21 as libc::c_int * c + i) as isize)
                        } else {
                            0.5f32
                        }
                    } else {
                        -2.0f32
                    };
                    if mask > 0 as libc::c_int as libc::c_float {
                        mask = 0.5f32 * mask
                    }
                    mask_sum += mask;
                    i += 1
                }
                c += 1
            }
            /* Conservative rate reduction, we cut the masking in half */
            masking_depth = mask_sum / end as libc::c_float * (*st).channels as libc::c_float;
            masking_depth += 0.2f32;
            rate_offset = (srate as crate::arch_h::opus_val32 * masking_depth)
                as crate::opus_types_h::opus_int32;
            rate_offset =
                if rate_offset > -(2 as libc::c_int) * (*st).silk_mode.bitRate / 3 as libc::c_int {
                    rate_offset
                } else {
                    (-(2 as libc::c_int) * (*st).silk_mode.bitRate) / 3 as libc::c_int
                };
            /* Split the rate change between the SILK and CELT part for hybrid. */
            if (*st).bandwidth == 1104 as libc::c_int || (*st).bandwidth == 1105 as libc::c_int {
                (*st).silk_mode.bitRate += 3 as libc::c_int * rate_offset / 5 as libc::c_int
            } else {
                (*st).silk_mode.bitRate += rate_offset
            }
        }
        (*st).silk_mode.payloadSize_ms = 1000 as libc::c_int * frame_size / (*st).Fs;
        (*st).silk_mode.nChannelsAPI = (*st).channels;
        (*st).silk_mode.nChannelsInternal = (*st).stream_channels;
        if curr_bandwidth == 1101 as libc::c_int {
            (*st).silk_mode.desiredInternalSampleRate = 8000 as libc::c_int
        } else if curr_bandwidth == 1102 as libc::c_int {
            (*st).silk_mode.desiredInternalSampleRate = 12000 as libc::c_int
        } else {
            (*st).silk_mode.desiredInternalSampleRate = 16000 as libc::c_int
        }
        if (*st).mode == 1001 as libc::c_int {
            /* Don't allow bandwidth reduction at lowest bitrates in hybrid mode */
            (*st).silk_mode.minInternalSampleRate = 16000 as libc::c_int
        } else {
            (*st).silk_mode.minInternalSampleRate = 8000 as libc::c_int
        }
        (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int;
        if (*st).mode == 1000 as libc::c_int {
            let mut effective_max_rate: crate::opus_types_h::opus_int32 = max_rate;
            if frame_rate > 50 as libc::c_int {
                effective_max_rate = effective_max_rate * 2 as libc::c_int / 3 as libc::c_int
            }
            if effective_max_rate < 8000 as libc::c_int {
                (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int;
                (*st).silk_mode.desiredInternalSampleRate =
                    if (12000 as libc::c_int) < (*st).silk_mode.desiredInternalSampleRate {
                        12000 as libc::c_int
                    } else {
                        (*st).silk_mode.desiredInternalSampleRate
                    }
            }
            if effective_max_rate < 7000 as libc::c_int {
                (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int;
                (*st).silk_mode.desiredInternalSampleRate =
                    if (8000 as libc::c_int) < (*st).silk_mode.desiredInternalSampleRate {
                        8000 as libc::c_int
                    } else {
                        (*st).silk_mode.desiredInternalSampleRate
                    }
            }
        }
        (*st).silk_mode.useCBR = ((*st).use_vbr == 0) as libc::c_int;
        /* Call SILK encoder for the low band */
        /* Max bits for SILK, counting ToC, redundancy bytes, and optionally redundancy. */
        (*st).silk_mode.maxBits = (max_data_bytes - 1 as libc::c_int) * 8 as libc::c_int;
        if redundancy != 0 && redundancy_bytes >= 2 as libc::c_int {
            /* Counting 1 bit for redundancy position and 20 bits for flag+size (only for hybrid). */
            (*st).silk_mode.maxBits -= redundancy_bytes * 8 as libc::c_int + 1 as libc::c_int;
            if (*st).mode == 1001 as libc::c_int {
                (*st).silk_mode.maxBits -= 20 as libc::c_int
            }
        }
        if (*st).silk_mode.useCBR != 0 {
            if (*st).mode == 1001 as libc::c_int {
                (*st).silk_mode.maxBits =
                    if (*st).silk_mode.maxBits < (*st).silk_mode.bitRate * frame_size / (*st).Fs {
                        (*st).silk_mode.maxBits
                    } else {
                        ((*st).silk_mode.bitRate * frame_size) / (*st).Fs
                    }
            }
        } else if (*st).mode == 1001 as libc::c_int {
            /* Constrained VBR. */
            /* Compute SILK bitrate corresponding to the max total bits available */
            let mut maxBitRate: crate::opus_types_h::opus_int32 = compute_silk_rate_for_hybrid(
                (*st).silk_mode.maxBits * (*st).Fs / frame_size,
                curr_bandwidth,
                ((*st).Fs == 50 as libc::c_int * frame_size) as libc::c_int,
                (*st).use_vbr,
                (*st).silk_mode.LBRR_coded,
            );
            (*st).silk_mode.maxBits = maxBitRate * frame_size / (*st).Fs
        }
        if prefill != 0 {
            let mut zero: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
            let mut prefill_offset: libc::c_int = 0;
            /* Use a smooth onset for the SILK prefill to avoid the encoder trying to encode
            a discontinuity. The exact location is what we need to avoid leaving any "gap"
            in the audio when mixing with the redundant CELT frame. Here we can afford to
            overwrite st->delay_buffer because the only thing that uses it before it gets
            rewritten is tmp_prefill[] and even then only the part after the ramp really
            gets used (rather than sent to the encoder and discarded) */
            prefill_offset = (*st).channels
                * ((*st).encoder_buffer - (*st).delay_compensation - (*st).Fs / 400 as libc::c_int);
            gain_fade(
                (*st)
                    .delay_buffer
                    .as_mut_ptr()
                    .offset(prefill_offset as isize),
                (*st)
                    .delay_buffer
                    .as_mut_ptr()
                    .offset(prefill_offset as isize),
                0 as libc::c_int as crate::arch_h::opus_val16,
                1.0f32,
                (*celt_mode).overlap,
                (*st).Fs / 400 as libc::c_int,
                (*st).channels,
                (*celt_mode).window,
                (*st).Fs,
            );
            crate::stdlib::memset(
                (*st).delay_buffer.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (prefill_offset as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong
                    ),
            );
            i = 0 as libc::c_int;
            while i < (*st).encoder_buffer * (*st).channels {
                *pcm_silk.offset(i as isize) = FLOAT2INT16((*st).delay_buffer[i as usize]);
                i += 1
            }
            crate::src::opus_1_2_1::silk::enc_API::silk_Encode(
                silk_enc,
                &mut (*st).silk_mode as *mut _ as *mut crate::control_h::silk_EncControlStruct,
                pcm_silk,
                (*st).encoder_buffer,
                0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                    as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                &mut zero,
                1 as libc::c_int,
            );
        }
        i = 0 as libc::c_int;
        while i < frame_size * (*st).channels {
            *pcm_silk.offset(i as isize) =
                FLOAT2INT16(*pcm_buf.offset((total_buffer * (*st).channels + i) as isize));
            i += 1
        }
        ret = crate::src::opus_1_2_1::silk::enc_API::silk_Encode(
            silk_enc,
            &mut (*st).silk_mode as *mut _ as *mut crate::control_h::silk_EncControlStruct,
            pcm_silk,
            frame_size,
            &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            &mut nBytes,
            0 as libc::c_int,
        );
        if ret != 0 {
            /*fprintf (stderr, "SILK encode error: %d\n", ret);*/
            /* Handle error */
            return -(3 as libc::c_int);
        }
        /* Extract SILK internal bandwidth for signaling in first byte */
        if (*st).mode == 1000 as libc::c_int {
            if (*st).silk_mode.internalSampleRate == 8000 as libc::c_int {
                curr_bandwidth = 1101 as libc::c_int
            } else if (*st).silk_mode.internalSampleRate == 12000 as libc::c_int {
                curr_bandwidth = 1102 as libc::c_int
            } else if (*st).silk_mode.internalSampleRate == 16000 as libc::c_int {
                curr_bandwidth = 1103 as libc::c_int
            }
        }
        (*st).silk_mode.opusCanSwitch =
            ((*st).silk_mode.switchReady != 0 && (*st).nonfinal_frame == 0) as libc::c_int;
        if nBytes == 0 as libc::c_int {
            (*st).rangeFinal = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
            *data.offset(-(1 as libc::c_int) as isize) = gen_toc(
                (*st).mode,
                (*st).Fs / frame_size,
                curr_bandwidth,
                (*st).stream_channels,
            );
            return 1 as libc::c_int;
        }
        /* FIXME: How do we allocate the redundancy for CBR? */
        if (*st).silk_mode.opusCanSwitch != 0 {
            redundancy_bytes = compute_redundancy_bytes(
                max_data_bytes,
                (*st).bitrate_bps,
                frame_rate,
                (*st).stream_channels,
            );
            redundancy = (redundancy_bytes != 0 as libc::c_int) as libc::c_int;
            celt_to_silk = 0 as libc::c_int;
            (*st).silk_bw_switch = 1 as libc::c_int
        }
    }
    /* CELT processing */
    let mut endband: libc::c_int = 21 as libc::c_int;
    match curr_bandwidth {
        1101 => endband = 13 as libc::c_int,
        1102 | 1103 => endband = 17 as libc::c_int,
        1104 => endband = 19 as libc::c_int,
        1105 => endband = 21 as libc::c_int,
        _ => {}
    }
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        10012 as libc::c_int,
        endband,
    );
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        10008 as libc::c_int,
        (*st).stream_channels,
    );
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        4002 as libc::c_int,
        -(1 as libc::c_int),
    );
    if (*st).mode != 1000 as libc::c_int {
        let mut celt_pred: crate::arch_h::opus_val32 =
            2 as libc::c_int as crate::arch_h::opus_val32;
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4006 as libc::c_int,
            0 as libc::c_int,
        );
        /* We may still decide to disable prediction later */
        if (*st).silk_mode.reducedDependency != 0 {
            celt_pred = 0 as libc::c_int as crate::arch_h::opus_val32
        }
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10002 as libc::c_int,
            celt_pred as crate::opus_types_h::opus_int32,
        );
        if (*st).mode == 1001 as libc::c_int {
            if (*st).use_vbr != 0 {
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4002 as libc::c_int,
                    (*st).bitrate_bps - (*st).silk_mode.bitRate,
                );
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4020 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        } else if (*st).use_vbr != 0 {
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                4006 as libc::c_int,
                1 as libc::c_int,
            );
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                4020 as libc::c_int,
                (*st).vbr_constraint,
            );
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                4002 as libc::c_int,
                (*st).bitrate_bps,
            );
        }
    }
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(((*st).channels * (*st).Fs / 400 as libc::c_int) as libc::c_ulong)
            as usize,
    );
    tmp_prefill = fresh9.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if (*st).mode != 1000 as libc::c_int
        && (*st).mode != (*st).prev_mode
        && (*st).prev_mode > 0 as libc::c_int
    {
        crate::stdlib::memcpy(
            tmp_prefill as *mut libc::c_void,
            &mut *(*st).delay_buffer.as_mut_ptr().offset(
                (((*st).encoder_buffer - total_buffer - (*st).Fs / 400 as libc::c_int)
                    * (*st).channels) as isize,
            ) as *mut crate::arch_h::opus_val16 as *const libc::c_void,
            (((*st).channels * (*st).Fs / 400 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * tmp_prefill.wrapping_offset_from(
                            &mut *(*st).delay_buffer.as_mut_ptr().offset(
                                (((*st).encoder_buffer
                                    - total_buffer
                                    - (*st).Fs / 400 as libc::c_int)
                                    * (*st).channels) as isize,
                            ),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
    }
    if (*st).channels * ((*st).encoder_buffer - (frame_size + total_buffer)) > 0 as libc::c_int {
        crate::stdlib::memmove(
            (*st).delay_buffer.as_mut_ptr() as *mut libc::c_void,
            &mut *(*st)
                .delay_buffer
                .as_mut_ptr()
                .offset(((*st).channels * frame_size) as isize)
                as *mut crate::arch_h::opus_val16 as *const libc::c_void,
            (((*st).channels * ((*st).encoder_buffer - frame_size - total_buffer))
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (*st).delay_buffer.as_mut_ptr().wrapping_offset_from(
                            &mut *(*st)
                                .delay_buffer
                                .as_mut_ptr()
                                .offset(((*st).channels * frame_size) as isize),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
        crate::stdlib::memcpy(
            &mut *(*st).delay_buffer.as_mut_ptr().offset(
                ((*st).channels * ((*st).encoder_buffer - frame_size - total_buffer)) as isize,
            ) as *mut crate::arch_h::opus_val16 as *mut libc::c_void,
            &mut *pcm_buf.offset(0 as libc::c_int as isize) as *mut crate::arch_h::opus_val16
                as *const libc::c_void,
            (((frame_size + total_buffer) * (*st).channels) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (&mut *(*st).delay_buffer.as_mut_ptr().offset(
                            ((*st).channels * ((*st).encoder_buffer - frame_size - total_buffer))
                                as isize,
                        ) as *mut crate::arch_h::opus_val16)
                            .wrapping_offset_from(&mut *pcm_buf.offset(0 as libc::c_int as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
    } else {
        crate::stdlib::memcpy(
            (*st).delay_buffer.as_mut_ptr() as *mut libc::c_void,
            &mut *pcm_buf.offset(
                ((frame_size + total_buffer - (*st).encoder_buffer) * (*st).channels) as isize,
            ) as *mut crate::arch_h::opus_val16 as *const libc::c_void,
            (((*st).encoder_buffer * (*st).channels) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (*st).delay_buffer.as_mut_ptr().wrapping_offset_from(
                            &mut *pcm_buf.offset(
                                ((frame_size + total_buffer - (*st).encoder_buffer)
                                    * (*st).channels) as isize,
                            ),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
    }
    /* gain_fade() and stereo_fade() need to be after the buffer copying
    because we don't want any of this to affect the SILK part */
    if (*st).prev_HB_gain < 1.0f32 || HB_gain < 1.0f32 {
        gain_fade(
            pcm_buf,
            pcm_buf,
            (*st).prev_HB_gain,
            HB_gain,
            (*celt_mode).overlap,
            frame_size,
            (*st).channels,
            (*celt_mode).window,
            (*st).Fs,
        );
    }
    (*st).prev_HB_gain = HB_gain;
    if (*st).mode != 1001 as libc::c_int || (*st).stream_channels == 1 as libc::c_int {
        (*st).silk_mode.stereoWidth_Q14 = if ((1 as libc::c_int) << 14 as libc::c_int)
            < 2 as libc::c_int
                * (if 0 as libc::c_int > equiv_rate - 24000 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (equiv_rate) - 24000 as libc::c_int
                }) {
            (1 as libc::c_int) << 14 as libc::c_int
        } else {
            (2 as libc::c_int)
                * (if 0 as libc::c_int > equiv_rate - 24000 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (equiv_rate) - 24000 as libc::c_int
                })
        }
    }
    if (*st).energy_masking.is_null() && (*st).channels == 2 as libc::c_int {
        /* Apply stereo width reduction (at low bitrates) */
        if ((*st).hybrid_stereo_width_Q14 as libc::c_int) < (1 as libc::c_int) << 14 as libc::c_int
            || (*st).silk_mode.stereoWidth_Q14 < (1 as libc::c_int) << 14 as libc::c_int
        {
            let mut g1: crate::arch_h::opus_val16 = 0.;
            let mut g2: crate::arch_h::opus_val16 = 0.;
            g1 = (*st).hybrid_stereo_width_Q14 as crate::arch_h::opus_val16;
            g2 = (*st).silk_mode.stereoWidth_Q14 as crate::arch_h::opus_val16;
            g1 *= 1.0f32 / 16384 as libc::c_int as libc::c_float;
            g2 *= 1.0f32 / 16384 as libc::c_int as libc::c_float;
            stereo_fade(
                pcm_buf,
                pcm_buf,
                g1,
                g2,
                (*celt_mode).overlap,
                frame_size,
                (*st).channels,
                (*celt_mode).window,
                (*st).Fs,
            );
            (*st).hybrid_stereo_width_Q14 =
                (*st).silk_mode.stereoWidth_Q14 as crate::opus_types_h::opus_int16
        }
    }
    if (*st).mode != 1002 as libc::c_int
        && ec_tell(&mut enc)
            + 17 as libc::c_int
            + 20 as libc::c_int * ((*st).mode == 1001 as libc::c_int) as libc::c_int
            <= 8 as libc::c_int * (max_data_bytes - 1 as libc::c_int)
    {
        /* For SILK mode, the redundancy is inferred from the length */
        if (*st).mode == 1001 as libc::c_int {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                redundancy,
                12 as libc::c_int as libc::c_uint,
            );
        }
        if redundancy != 0 {
            let mut max_redundancy: libc::c_int = 0;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                celt_to_silk,
                1 as libc::c_int as libc::c_uint,
            );
            if (*st).mode == 1001 as libc::c_int {
                /* Reserve the 8 bits needed for the redundancy length,
                and at least a few bits for CELT if possible */
                max_redundancy = max_data_bytes
                    - 1 as libc::c_int
                    - (ec_tell(&mut enc) + 8 as libc::c_int + 3 as libc::c_int + 7 as libc::c_int
                        >> 3 as libc::c_int)
            } else {
                max_redundancy = max_data_bytes
                    - 1 as libc::c_int
                    - (ec_tell(&mut enc) + 7 as libc::c_int >> 3 as libc::c_int)
            }
            /* Target the same bit-rate for redundancy as for the rest,
            up to a max of 257 bytes */
            redundancy_bytes = if max_redundancy < redundancy_bytes {
                max_redundancy
            } else {
                redundancy_bytes
            };
            redundancy_bytes = if (257 as libc::c_int)
                < (if 2 as libc::c_int > redundancy_bytes {
                    2 as libc::c_int
                } else {
                    redundancy_bytes
                }) {
                257 as libc::c_int
            } else if 2 as libc::c_int > redundancy_bytes {
                2 as libc::c_int
            } else {
                redundancy_bytes
            };
            if (*st).mode == 1001 as libc::c_int {
                crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
                    &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (redundancy_bytes - 2 as libc::c_int) as crate::opus_types_h::opus_uint32,
                    256 as libc::c_int as crate::opus_types_h::opus_uint32,
                );
            }
        }
    } else {
        redundancy = 0 as libc::c_int
    }
    if redundancy == 0 {
        (*st).silk_bw_switch = 0 as libc::c_int;
        redundancy_bytes = 0 as libc::c_int
    }
    if (*st).mode != 1002 as libc::c_int {
        start_band = 17 as libc::c_int
    }
    if (*st).mode == 1000 as libc::c_int {
        ret = ec_tell(&mut enc) + 7 as libc::c_int >> 3 as libc::c_int;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_done(
            &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        );
        nb_compr_bytes = ret
    } else {
        nb_compr_bytes = max_data_bytes - 1 as libc::c_int - redundancy_bytes;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
            &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            nb_compr_bytes as crate::opus_types_h::opus_uint32,
        );
    }
    if redundancy != 0 || (*st).mode != 1000 as libc::c_int {
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10022 as libc::c_int,
            (&mut analysis_info as *mut crate::celt_h::AnalysisInfo).offset(
                (&mut analysis_info as *mut crate::celt_h::AnalysisInfo).wrapping_offset_from(
                    &mut analysis_info as *mut crate::celt_h::AnalysisInfo
                        as *const crate::celt_h::AnalysisInfo,
                ) as libc::c_long as isize,
            ),
        );
    }
    if (*st).mode == 1001 as libc::c_int {
        let mut info: crate::celt_h::SILKInfo = crate::celt_h::SILKInfo {
            signalType: 0,
            offset: 0,
        };
        info.signalType = (*st).silk_mode.signalType;
        info.offset = (*st).silk_mode.offset;
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10028 as libc::c_int,
            (&mut info as *mut crate::celt_h::SILKInfo).offset(
                (&mut info as *mut crate::celt_h::SILKInfo).wrapping_offset_from(
                    &mut info as *mut crate::celt_h::SILKInfo as *const crate::celt_h::SILKInfo,
                ) as libc::c_long as isize,
            ),
        );
    } else {
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10028 as libc::c_int,
            (0 as *mut libc::c_void as *mut crate::celt_h::SILKInfo).offset(
                (0 as *mut libc::c_void as *mut crate::celt_h::SILKInfo).wrapping_offset_from(
                    0 as *mut libc::c_void as *mut crate::celt_h::SILKInfo
                        as *const crate::celt_h::SILKInfo,
                ) as libc::c_long as isize,
            ),
        );
    }
    /* 5 ms redundant frame for CELT->SILK */
    if redundancy != 0 && celt_to_silk != 0 {
        let mut err: libc::c_int = 0;
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10010 as libc::c_int,
            0 as libc::c_int,
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4006 as libc::c_int,
            0 as libc::c_int,
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4002 as libc::c_int,
            -(1 as libc::c_int),
        );
        err = crate::src::opus_1_2_1::celt::celt_encoder::celt_encode_with_ec(
            celt_enc,
            pcm_buf,
            (*st).Fs / 200 as libc::c_int,
            data.offset(nb_compr_bytes as isize),
            redundancy_bytes,
            0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        );
        if err < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4031 as libc::c_int,
            (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32).offset(
                (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32).wrapping_offset_from(
                    &mut redundant_rng as *mut crate::opus_types_h::opus_uint32,
                ) as libc::c_long as isize,
            ),
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4028 as libc::c_int,
        );
    }
    crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
        celt_enc,
        10010 as libc::c_int,
        start_band,
    );
    if (*st).mode != 1000 as libc::c_int {
        if (*st).mode != (*st).prev_mode && (*st).prev_mode > 0 as libc::c_int {
            let mut dummy_0: [libc::c_uchar; 2] = [0; 2];
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                4028 as libc::c_int,
            );
            /* Prefilling */
            crate::src::opus_1_2_1::celt::celt_encoder::celt_encode_with_ec(
                celt_enc,
                tmp_prefill,
                (*st).Fs / 400 as libc::c_int,
                dummy_0.as_mut_ptr(),
                2 as libc::c_int,
                0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                    as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            );
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                10002 as libc::c_int,
                0 as libc::c_int,
            );
        }
        /* If false, we already busted the budget and we'll end up with a "PLC frame" */
        if ec_tell(&mut enc) <= 8 as libc::c_int * nb_compr_bytes {
            /* Set the bitrate again if it was overridden in the redundancy code above*/
            if redundancy != 0
                && celt_to_silk != 0
                && (*st).mode == 1001 as libc::c_int
                && (*st).use_vbr != 0
            {
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4002 as libc::c_int,
                    (*st).bitrate_bps - (*st).silk_mode.bitRate,
                );
            }
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                4006 as libc::c_int,
                (*st).use_vbr,
            );
            ret = crate::src::opus_1_2_1::celt::celt_encoder::celt_encode_with_ec(
                celt_enc,
                pcm_buf,
                frame_size,
                0 as *mut libc::c_uchar,
                nb_compr_bytes,
                &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            );
            if ret < 0 as libc::c_int {
                return -(3 as libc::c_int);
            }
            /* Put CELT->SILK redundancy data in the right place. */
            if redundancy != 0
                && celt_to_silk != 0
                && (*st).mode == 1001 as libc::c_int
                && (*st).use_vbr != 0
            {
                crate::stdlib::memmove(
                    data.offset(ret as isize) as *mut libc::c_void,
                    data.offset(nb_compr_bytes as isize) as *const libc::c_void,
                    (redundancy_bytes as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                        .wrapping_add(
                            (0 as libc::c_int as libc::c_long
                                * data
                                    .offset(ret as isize)
                                    .wrapping_offset_from(data.offset(nb_compr_bytes as isize))
                                    as libc::c_long) as libc::c_ulong,
                        ),
                );
                nb_compr_bytes = nb_compr_bytes + redundancy_bytes
            }
        }
    }
    /* 5 ms redundant frame for SILK->CELT */
    if redundancy != 0 && celt_to_silk == 0 {
        let mut err_0: libc::c_int = 0;
        let mut dummy_1: [libc::c_uchar; 2] = [0; 2];
        let mut N2: libc::c_int = 0;
        let mut N4: libc::c_int = 0;
        N2 = (*st).Fs / 200 as libc::c_int;
        N4 = (*st).Fs / 400 as libc::c_int;
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4028 as libc::c_int,
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10010 as libc::c_int,
            0 as libc::c_int,
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            10002 as libc::c_int,
            0 as libc::c_int,
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4006 as libc::c_int,
            0 as libc::c_int,
        );
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4002 as libc::c_int,
            -(1 as libc::c_int),
        );
        if (*st).mode == 1001 as libc::c_int {
            /* Shrink packet to what the encoder actually used. */
            nb_compr_bytes = ret;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
                &mut enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                nb_compr_bytes as crate::opus_types_h::opus_uint32,
            );
        }
        /* NOTE: We could speed this up slightly (at the expense of code size) by just adding a function that prefills the buffer */
        crate::src::opus_1_2_1::celt::celt_encoder::celt_encode_with_ec(
            celt_enc,
            pcm_buf.offset(((*st).channels * (frame_size - N2 - N4)) as isize),
            N4,
            dummy_1.as_mut_ptr(),
            2 as libc::c_int,
            0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        );
        err_0 = crate::src::opus_1_2_1::celt::celt_encoder::celt_encode_with_ec(
            celt_enc,
            pcm_buf.offset(((*st).channels * (frame_size - N2)) as isize),
            N2,
            data.offset(nb_compr_bytes as isize),
            redundancy_bytes,
            0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        );
        if err_0 < 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
            celt_enc,
            4031 as libc::c_int,
            (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32).offset(
                (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32).wrapping_offset_from(
                    &mut redundant_rng as *mut crate::opus_types_h::opus_uint32,
                ) as libc::c_long as isize,
            ),
        );
    }
    /* Signalling the mode in the first byte */
    data = data.offset(-1);
    *data.offset(0 as libc::c_int as isize) = gen_toc(
        (*st).mode,
        (*st).Fs / frame_size,
        curr_bandwidth,
        (*st).stream_channels,
    );
    (*st).rangeFinal = enc.rng ^ redundant_rng;
    if to_celt != 0 {
        (*st).prev_mode = 1002 as libc::c_int
    } else {
        (*st).prev_mode = (*st).mode
    }
    (*st).prev_channels = (*st).stream_channels;
    (*st).prev_framesize = frame_size;
    (*st).first = 0 as libc::c_int;
    /* DTX decision */
    if (*st).use_dtx != 0 && (analysis_info.valid != 0 || is_silence != 0) {
        if decide_dtx_mode(
            analysis_info.activity_probability,
            &mut (*st).nb_no_activity_frames,
            (*st).peak_signal_energy,
            pcm,
            frame_size,
            (*st).channels,
            is_silence,
            (*st).arch,
        ) != 0
        {
            (*st).rangeFinal = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
            *data.offset(0 as libc::c_int as isize) = gen_toc(
                (*st).mode,
                (*st).Fs / frame_size,
                curr_bandwidth,
                (*st).stream_channels,
            );
            return 1 as libc::c_int;
        }
    }
    /* In the unlikely case that the SILK encoder busted its target, tell
    the decoder to call the PLC */
    if ec_tell(&mut enc) > (max_data_bytes - 1 as libc::c_int) * 8 as libc::c_int {
        if max_data_bytes < 2 as libc::c_int {
            return -(2 as libc::c_int);
        }
        *data.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        ret = 1 as libc::c_int;
        (*st).rangeFinal = 0 as libc::c_int as crate::opus_types_h::opus_uint32
    } else if (*st).mode == 1000 as libc::c_int && redundancy == 0 {
        /*When in LPC only mode it's perfectly
        reasonable to strip off trailing zero bytes as
        the required range decoder behavior is to
        fill these in. This can't be done when the MDCT
        modes are used because the decoder needs to know
        the actual length for allocation purposes.*/
        while ret > 2 as libc::c_int
            && *data.offset(ret as isize) as libc::c_int == 0 as libc::c_int
        {
            ret -= 1
        }
    }
    /* Count ToC and redundancy */
    ret += 1 as libc::c_int + redundancy_bytes;
    if (*st).use_vbr == 0 {
        if crate::src::opus_1_2_1::src::repacketizer::opus_packet_pad(data, ret, max_data_bytes)
            != 0 as libc::c_int
        {
            return -(3 as libc::c_int);
        }
        ret = max_data_bytes
    }
    return ret;
}
/* * Encodes an Opus frame.
 * @param [in] st <tt>OpusEncoder*</tt>: Encoder state
 * @param [in] pcm <tt>opus_int16*</tt>: Input signal (interleaved if 2 channels). length is frame_size*channels*sizeof(opus_int16)
 * @param [in] frame_size <tt>int</tt>: Number of samples per channel in the
 *                                      input signal.
 *                                      This must be an Opus frame size for
 *                                      the encoder's sampling rate.
 *                                      For example, at 48 kHz the permitted
 *                                      values are 120, 240, 480, 960, 1920,
 *                                      and 2880.
 *                                      Passing in a duration of less than
 *                                      10 ms (480 samples at 48 kHz) will
 *                                      prevent the encoder from using the LPC
 *                                      or hybrid modes.
 * @param [out] data <tt>unsigned char*</tt>: Output payload.
 *                                            This must contain storage for at
 *                                            least \a max_data_bytes.
 * @param [in] max_data_bytes <tt>opus_int32</tt>: Size of the allocated
 *                                                 memory for the output
 *                                                 payload. This may be
 *                                                 used to impose an upper limit on
 *                                                 the instant bitrate, but should
 *                                                 not be used as the only bitrate
 *                                                 control. Use #OPUS_SET_BITRATE to
 *                                                 control the bitrate.
 * @returns The length of the encoded packet (in bytes) on success or a
 *          negative error code (see @ref opus_errorcodes) on failure.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encode(
    mut st: *mut OpusEncoder,
    mut pcm: *const crate::opus_types_h::opus_int16,
    mut analysis_frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: crate::opus_types_h::opus_int32,
) -> crate::opus_types_h::opus_int32 {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut frame_size: libc::c_int = 0;
    let mut in_0: *mut libc::c_float = 0 as *mut libc::c_float;
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, (*st).Fs);
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((frame_size * (*st).channels) as libc::c_ulong) as usize,
    );
    in_0 = fresh10.as_mut_ptr() as *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < frame_size * (*st).channels {
        *in_0.offset(i as isize) = 1.0f32 / 32768 as libc::c_int as libc::c_float
            * *pcm.offset(i as isize) as libc::c_int as libc::c_float;
        i += 1
    }
    ret = opus_encode_native(
        st,
        in_0,
        frame_size,
        data,
        max_data_bytes,
        16 as libc::c_int,
        pcm as *const libc::c_void,
        analysis_frame_size,
        0 as libc::c_int,
        -(2 as libc::c_int),
        (*st).channels,
        Some(
            downmix_int
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *mut crate::arch_h::opus_val32,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        0 as libc::c_int,
    );
    return ret;
}
/* * Encodes an Opus frame from floating point input.
 * @param [in] st <tt>OpusEncoder*</tt>: Encoder state
 * @param [in] pcm <tt>float*</tt>: Input in float format (interleaved if 2 channels), with a normal range of +/-1.0.
 *          Samples with a range beyond +/-1.0 are supported but will
 *          be clipped by decoders using the integer API and should
 *          only be used if it is known that the far end supports
 *          extended dynamic range.
 *          length is frame_size*channels*sizeof(float)
 * @param [in] frame_size <tt>int</tt>: Number of samples per channel in the
 *                                      input signal.
 *                                      This must be an Opus frame size for
 *                                      the encoder's sampling rate.
 *                                      For example, at 48 kHz the permitted
 *                                      values are 120, 240, 480, 960, 1920,
 *                                      and 2880.
 *                                      Passing in a duration of less than
 *                                      10 ms (480 samples at 48 kHz) will
 *                                      prevent the encoder from using the LPC
 *                                      or hybrid modes.
 * @param [out] data <tt>unsigned char*</tt>: Output payload.
 *                                            This must contain storage for at
 *                                            least \a max_data_bytes.
 * @param [in] max_data_bytes <tt>opus_int32</tt>: Size of the allocated
 *                                                 memory for the output
 *                                                 payload. This may be
 *                                                 used to impose an upper limit on
 *                                                 the instant bitrate, but should
 *                                                 not be used as the only bitrate
 *                                                 control. Use #OPUS_SET_BITRATE to
 *                                                 control the bitrate.
 * @returns The length of the encoded packet (in bytes) on success or a
 *          negative error code (see @ref opus_errorcodes) on failure.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encode_float(
    mut st: *mut OpusEncoder,
    mut pcm: *const libc::c_float,
    mut analysis_frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut out_data_bytes: crate::opus_types_h::opus_int32,
) -> crate::opus_types_h::opus_int32 {
    let mut frame_size: libc::c_int = 0;
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, (*st).Fs);
    return opus_encode_native(
        st,
        pcm,
        frame_size,
        data,
        out_data_bytes,
        24 as libc::c_int,
        pcm as *const libc::c_void,
        analysis_frame_size,
        0 as libc::c_int,
        -(2 as libc::c_int),
        (*st).channels,
        Some(
            downmix_float
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *mut crate::arch_h::opus_val32,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        1 as libc::c_int,
    );
}
/* * Perform a CTL function on an Opus encoder.
 *
 * Generally the request and subsequent arguments are generated
 * by a convenience macro.
 * @param st <tt>OpusEncoder*</tt>: Encoder state.
 * @param request This and all remaining parameters should be replaced by one
 *                of the convenience macros in @ref opus_genericctls or
 *                @ref opus_encoderctls.
 * @see opus_genericctls
 * @see opus_encoderctls
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encoder_ctl(
    mut st: *mut OpusEncoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut celt_enc: *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder =
        0 as *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder;
    let mut ap: ::std::ffi::VaListImpl;
    ret = 0 as libc::c_int;
    ap = args.clone();
    celt_enc = (st as *mut libc::c_char).offset((*st).celt_enc_offset as isize)
        as *mut crate::src::opus_1_2_1::celt::celt_encoder::OpusCustomEncoder;
    match request {
        4000 => {
            let mut value: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value != 2048 as libc::c_int
                && value != 2049 as libc::c_int
                && value != 2051 as libc::c_int
                || (*st).first == 0 && (*st).application != value
            {
                ret = -(1 as libc::c_int)
            } else {
                (*st).application = value;
                (*st).analysis.application = value
            }
            current_block = 12032176231992402880;
        }
        4001 => {
            let mut value_0: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_0.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_0 = (*st).application;
                current_block = 12032176231992402880;
            }
        }
        4002 => {
            let mut value_1: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_1 != -(1000 as libc::c_int) && value_1 != -(1 as libc::c_int) {
                if value_1 <= 0 as libc::c_int {
                    current_block = 18078460720374183796;
                } else {
                    if value_1 <= 500 as libc::c_int {
                        value_1 = 500 as libc::c_int
                    } else if value_1 > 300000 as libc::c_int * (*st).channels {
                        value_1 = 300000 as libc::c_int * (*st).channels
                    }
                    current_block = 11307063007268554308;
                }
            } else {
                current_block = 11307063007268554308;
            }
            match current_block {
                18078460720374183796 => {}
                _ => {
                    (*st).user_bitrate_bps = value_1;
                    current_block = 12032176231992402880;
                }
            }
        }
        4003 => {
            let mut value_2: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_2.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_2 = user_bitrate_to_bitrate(st, (*st).prev_framesize, 1276 as libc::c_int);
                current_block = 12032176231992402880;
            }
        }
        4022 => {
            let mut value_3: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if (value_3 < 1 as libc::c_int || value_3 > (*st).channels)
                && value_3 != -(1000 as libc::c_int)
            {
                current_block = 18078460720374183796;
            } else {
                (*st).force_channels = value_3;
                current_block = 12032176231992402880;
            }
        }
        4023 => {
            let mut value_4: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_4.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_4 = (*st).force_channels;
                current_block = 12032176231992402880;
            }
        }
        4004 => {
            let mut value_5: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_5 < 1101 as libc::c_int || value_5 > 1105 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).max_bandwidth = value_5;
                if (*st).max_bandwidth == 1101 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int
                } else if (*st).max_bandwidth == 1102 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int
                } else {
                    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int
                }
                current_block = 12032176231992402880;
            }
        }
        4005 => {
            let mut value_6: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_6.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_6 = (*st).max_bandwidth;
                current_block = 12032176231992402880;
            }
        }
        4008 => {
            let mut value_7: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if (value_7 < 1101 as libc::c_int || value_7 > 1105 as libc::c_int)
                && value_7 != -(1000 as libc::c_int)
            {
                current_block = 18078460720374183796;
            } else {
                (*st).user_bandwidth = value_7;
                if (*st).user_bandwidth == 1101 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 8000 as libc::c_int
                } else if (*st).user_bandwidth == 1102 as libc::c_int {
                    (*st).silk_mode.maxInternalSampleRate = 12000 as libc::c_int
                } else {
                    (*st).silk_mode.maxInternalSampleRate = 16000 as libc::c_int
                }
                current_block = 12032176231992402880;
            }
        }
        4009 => {
            let mut value_8: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_8.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_8 = (*st).bandwidth;
                current_block = 12032176231992402880;
            }
        }
        4016 => {
            let mut value_9: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_9 < 0 as libc::c_int || value_9 > 1 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).use_dtx = value_9;
                current_block = 12032176231992402880;
            }
        }
        4017 => {
            let mut value_10: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_10.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_10 = (*st).use_dtx;
                current_block = 12032176231992402880;
            }
        }
        4010 => {
            let mut value_11: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_11 < 0 as libc::c_int || value_11 > 10 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).silk_mode.complexity = value_11;
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4010 as libc::c_int,
                    value_11,
                );
                current_block = 12032176231992402880;
            }
        }
        4011 => {
            let mut value_12: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_12.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_12 = (*st).silk_mode.complexity;
                current_block = 12032176231992402880;
            }
        }
        4012 => {
            let mut value_13: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_13 < 0 as libc::c_int || value_13 > 1 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).silk_mode.useInBandFEC = value_13;
                current_block = 12032176231992402880;
            }
        }
        4013 => {
            let mut value_14: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_14.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_14 = (*st).silk_mode.useInBandFEC;
                current_block = 12032176231992402880;
            }
        }
        4014 => {
            let mut value_15: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_15 < 0 as libc::c_int || value_15 > 100 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).silk_mode.packetLossPercentage = value_15;
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4014 as libc::c_int,
                    value_15,
                );
                current_block = 12032176231992402880;
            }
        }
        4015 => {
            let mut value_16: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_16.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_16 = (*st).silk_mode.packetLossPercentage;
                current_block = 12032176231992402880;
            }
        }
        4006 => {
            let mut value_17: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_17 < 0 as libc::c_int || value_17 > 1 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).use_vbr = value_17;
                (*st).silk_mode.useCBR = 1 as libc::c_int - value_17;
                current_block = 12032176231992402880;
            }
        }
        4007 => {
            let mut value_18: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_18.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_18 = (*st).use_vbr;
                current_block = 12032176231992402880;
            }
        }
        11018 => {
            let mut value_19: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_19 < -(1 as libc::c_int) || value_19 > 100 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).voice_ratio = value_19;
                current_block = 12032176231992402880;
            }
        }
        11019 => {
            let mut value_20: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_20.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_20 = (*st).voice_ratio;
                current_block = 12032176231992402880;
            }
        }
        4020 => {
            let mut value_21: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_21 < 0 as libc::c_int || value_21 > 1 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).vbr_constraint = value_21;
                current_block = 12032176231992402880;
            }
        }
        4021 => {
            let mut value_22: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_22.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_22 = (*st).vbr_constraint;
                current_block = 12032176231992402880;
            }
        }
        4024 => {
            let mut value_23: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_23 != -(1000 as libc::c_int)
                && value_23 != 3001 as libc::c_int
                && value_23 != 3002 as libc::c_int
            {
                current_block = 18078460720374183796;
            } else {
                (*st).signal_type = value_23;
                current_block = 12032176231992402880;
            }
        }
        4025 => {
            let mut value_24: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_24.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_24 = (*st).signal_type;
                current_block = 12032176231992402880;
            }
        }
        4027 => {
            let mut value_25: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_25.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_25 = (*st).Fs / 400 as libc::c_int;
                if (*st).application != 2051 as libc::c_int {
                    *value_25 += (*st).delay_compensation
                }
                current_block = 12032176231992402880;
            }
        }
        4029 => {
            let mut value_26: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_26.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_26 = (*st).Fs;
                current_block = 12032176231992402880;
            }
        }
        4031 => {
            let mut value_27: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            if value_27.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_27 = (*st).rangeFinal;
                current_block = 12032176231992402880;
            }
        }
        4036 => {
            let mut value_28: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_28 < 8 as libc::c_int || value_28 > 24 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).lsb_depth = value_28;
                current_block = 12032176231992402880;
            }
        }
        4037 => {
            let mut value_29: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_29.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_29 = (*st).lsb_depth;
                current_block = 12032176231992402880;
            }
        }
        4040 => {
            let mut value_30: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_30 != 5000 as libc::c_int
                && value_30 != 5001 as libc::c_int
                && value_30 != 5002 as libc::c_int
                && value_30 != 5003 as libc::c_int
                && value_30 != 5004 as libc::c_int
                && value_30 != 5005 as libc::c_int
                && value_30 != 5006 as libc::c_int
                && value_30 != 5007 as libc::c_int
                && value_30 != 5008 as libc::c_int
                && value_30 != 5009 as libc::c_int
            {
                current_block = 18078460720374183796;
            } else {
                (*st).variable_duration = value_30;
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4040 as libc::c_int,
                    value_30,
                );
                current_block = 12032176231992402880;
            }
        }
        4041 => {
            let mut value_31: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_31.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_31 = (*st).variable_duration;
                current_block = 12032176231992402880;
            }
        }
        4042 => {
            let mut value_32: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_32 > 1 as libc::c_int || value_32 < 0 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                (*st).silk_mode.reducedDependency = value_32;
                current_block = 12032176231992402880;
            }
        }
        4043 => {
            let mut value_33: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_33.is_null() {
                current_block = 18078460720374183796;
            } else {
                *value_33 = (*st).silk_mode.reducedDependency;
                current_block = 12032176231992402880;
            }
        }
        4046 => {
            let mut value_34: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_34 < 0 as libc::c_int || value_34 > 1 as libc::c_int {
                current_block = 18078460720374183796;
            } else {
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4046 as libc::c_int,
                    value_34,
                );
                current_block = 12032176231992402880;
            }
        }
        4047 => {
            let mut value_35: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_35.is_null() {
                current_block = 18078460720374183796;
            } else {
                crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    4047 as libc::c_int,
                    value_35
                        .offset(value_35.wrapping_offset_from(value_35) as libc::c_long as isize),
                );
                current_block = 12032176231992402880;
            }
        }
        4028 => {
            let mut silk_enc: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut dummy: crate::control_h::silk_EncControlStruct =
                crate::control_h::silk_EncControlStruct {
                    nChannelsAPI: 0,
                    nChannelsInternal: 0,
                    API_sampleRate: 0,
                    maxInternalSampleRate: 0,
                    minInternalSampleRate: 0,
                    desiredInternalSampleRate: 0,
                    payloadSize_ms: 0,
                    bitRate: 0,
                    packetLossPercentage: 0,
                    complexity: 0,
                    useInBandFEC: 0,
                    LBRR_coded: 0,
                    useDTX: 0,
                    useCBR: 0,
                    maxBits: 0,
                    toMono: 0,
                    opusCanSwitch: 0,
                    reducedDependency: 0,
                    internalSampleRate: 0,
                    allowBandwidthSwitch: 0,
                    inWBmodeWithoutVariableLP: 0,
                    stereoWidth_Q14: 0,
                    switchReady: 0,
                    signalType: 0,
                    offset: 0,
                };
            let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
            silk_enc = (st as *mut libc::c_char).offset((*st).silk_enc_offset as isize)
                as *mut libc::c_void;
            crate::src::opus_1_2_1::src::analysis::tonality_analysis_reset(
                &mut (*st).analysis as *mut _
                    as *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
            );
            start = &mut (*st).stream_channels as *mut libc::c_int as *mut libc::c_char;
            crate::stdlib::memset(
                start as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<OpusEncoder>() as libc::c_ulong)
                    .wrapping_sub(start.wrapping_offset_from(st as *mut libc::c_char)
                        as libc::c_long as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                4028 as libc::c_int,
            );
            crate::src::opus_1_2_1::silk::enc_API::silk_InitEncoder(
                silk_enc,
                (*st).arch,
                &mut dummy as *mut _ as *mut crate::control_h::silk_EncControlStruct,
            );
            (*st).stream_channels = (*st).channels;
            (*st).hybrid_stereo_width_Q14 =
                ((1 as libc::c_int) << 14 as libc::c_int) as crate::opus_types_h::opus_int16;
            (*st).prev_HB_gain = 1.0f32;
            (*st).first = 1 as libc::c_int;
            (*st).mode = 1001 as libc::c_int;
            (*st).bandwidth = 1105 as libc::c_int;
            (*st).variable_HP_smth2_Q15 =
                ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
                    as crate::opus_types_h::opus_uint32)
                    << 8 as libc::c_int) as crate::opus_types_h::opus_int32;
            current_block = 12032176231992402880;
        }
        11002 => {
            let mut value_36: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if (value_36 < 1000 as libc::c_int || value_36 > 1002 as libc::c_int)
                && value_36 != -(1000 as libc::c_int)
            {
                current_block = 18078460720374183796;
            } else {
                (*st).user_forced_mode = value_36;
                current_block = 12032176231992402880;
            }
        }
        10024 => {
            let mut value_37: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).lfe = value_37;
            ret = crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                10024 as libc::c_int,
                value_37,
            );
            current_block = 12032176231992402880;
        }
        10026 => {
            let mut value_38: *mut crate::arch_h::opus_val16 =
                ap.as_va_list().arg::<*mut crate::arch_h::opus_val16>();
            (*st).energy_masking = value_38;
            ret = crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                celt_enc,
                10026 as libc::c_int,
                value_38.offset(value_38.wrapping_offset_from(value_38) as libc::c_long as isize),
            );
            current_block = 12032176231992402880;
        }
        10015 => {
            let mut value_39: *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode = ap
                .as_va_list()
                .arg::<*mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode>();
            if value_39.is_null() {
                current_block = 18078460720374183796;
            } else {
                ret = crate::src::opus_1_2_1::celt::celt_encoder::opus_custom_encoder_ctl(
                    celt_enc,
                    10015 as libc::c_int,
                    value_39
                        .offset(value_39.wrapping_offset_from(value_39) as libc::c_long as isize),
                );
                current_block = 12032176231992402880;
            }
        }
        _ => {
            /* fprintf(stderr, "unknown opus_encoder_ctl() request: %d", request);*/
            ret = -(5 as libc::c_int);
            current_block = 12032176231992402880;
        }
    }
    match current_block {
        12032176231992402880 => return ret,
        _ => return -(1 as libc::c_int),
    };
}
/* * Frees an <code>OpusEncoder</code> allocated by opus_encoder_create().
 * @param[in] st <tt>OpusEncoder*</tt>: State to be freed.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_encoder_destroy(mut st: *mut OpusEncoder) {
    opus_free(st as *mut libc::c_void);
}
