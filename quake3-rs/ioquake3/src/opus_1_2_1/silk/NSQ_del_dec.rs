use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32) - (::std::mem::size_of::<u32>() as i32 * 8 - (in32 as u32).leading_zeros() as i32)
        } else {
            32
        };
    }

    /* SILK_MACROS_H */
    /* Column based */
    /* Row based */
}

pub mod SigProc_FIX_h {
    /* Allocate opus_int16 aligned to 4-byte memory address */
    /* Useful Macros that can be adjusted to other platforms */
    /* Fixed point macros */
    /* (a32 * b32) output have to be 32bit int */
    /* (a32 * b32) output have to be 32bit uint */
    /* a32 + (b32 * c32) output have to be 32bit int */
    /* a32 + (b32 * c32) output have to be 32bit uint */
    /* ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* a32 + ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* (a32 * b32) */
    /*(opus_int64)*/
    /* Adds two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Subtractss two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Multiply-accumulate macros that allow overflow in the addition (ie, no asserts in debug mode) */
    /* These macros enables checking for overflow in silk_API_Debug.h*/
    /* Saturation for positive input values */
    /* Add with saturation for positive input values */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* saturates before shifting */
    /* shift >= 0, allowed to overflow */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* Requires that shift > 0 */
    /* Number of rightshift required to fit the multiplication */
    /* Macro to convert floating-point constants to fixed-point */
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_min_int(mut a: i32, mut b: i32) -> i32 {
        return if a < b { a } else { b };
    }

    /* SILK_SIGPROC_FIX_H */
    /*    silk_SMMUL: Signed top word multiply.
    ARMv6        2 instruction cycles.
    ARMv3M+      3 instruction cycles. use SMULL and ignore LSB registers.(except xM)*/
    /*#define silk_SMMUL(a32, b32)                (opus_int32)silk_RSHIFT(silk_SMLAL(silk_SMULWB((a32), (b32)), (a32), silk_RSHIFT_ROUND((b32), 16)), 16)*/
    /* the following seems faster on x86 */
    /*  Add some multiplication functions that can be easily mapped to ARM. */
    /* PSEUDO-RANDOM GENERATOR                                                          */
    /* Make sure to store the result as the seed for the next call (also in between     */
    /* frames), otherwise result won't be random at all. When only using some of the    */
    /* bits, take the most significant bits by right-shifting.                          */
    /* Be careful, silk_abs returns wrong when input equals to silk_intXX_MIN */
}

pub mod Inlines_h {
    /* Divide two int32 values and return result as int32 in a given Q-domain */
    #[inline]

    pub unsafe extern "C" fn silk_DIV32_varQ(
        a32: crate::opus_types_h::opus_int32,
        b32: crate::opus_types_h::opus_int32,
        Qres: i32,
    ) -> crate::opus_types_h::opus_int32
/* I    Q-domain of result (>= 0)       */ {
        let mut a_headrm: i32 = 0;
        let mut b_headrm: i32 = 0;
        let mut lshift: i32 = 0;
        let mut b32_inv: crate::opus_types_h::opus_int32 = 0;
        let mut a32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut b32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut result: crate::opus_types_h::opus_int32 = 0;
        /* Compute number of bits head room and normalize inputs */
        a_headrm = silk_CLZ32((if a32 > 0 { a32 } else { -a32 })) - 1; /* Q: a_headrm                  */
        a32_nrm = ((a32 as crate::opus_types_h::opus_uint32) << a_headrm)
            as crate::opus_types_h::opus_int32; /* Q: b_headrm                  */
        b_headrm = silk_CLZ32((if b32 > 0 { b32 } else { -b32 })) - 1;
        b32_nrm = ((b32 as crate::opus_types_h::opus_uint32) << b_headrm)
            as crate::opus_types_h::opus_int32;
        /* Inverse of b32, with 14 bits of precision */
        b32_inv = (0x7fffffff >> 2) / (b32_nrm >> 16); /* Q: 29 + 16 - b_headrm        */
        /* First approximation */
        result = (a32_nrm as i64 * b32_inv as crate::opus_types_h::opus_int16 as i64 >> 16)
            as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Compute residual by subtracting product of denominator and first approximation */
        /* It's OK to overflow because the final value of a32_nrm should always be small */
        a32_nrm = (a32_nrm as crate::opus_types_h::opus_uint32).wrapping_sub(
            ((b32_nrm as i64 * result as i64 >> 32) as crate::opus_types_h::opus_uint32) << 3,
        ) as crate::opus_types_h::opus_int32; /* Q: a_headrm   */
        /* Refinement */
        result = (result as i64
            + (a32_nrm as i64 * b32_inv as crate::opus_types_h::opus_int16 as i64 >> 16))
            as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Convert to Qres domain */
        lshift = 29 + a_headrm - b_headrm - Qres;
        if lshift < 0 {
            return (((if 0x80000000u32 as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffffi32 >> -lshift
            {
                (if result > 0x80000000u32 as crate::opus_types_h::opus_int32 >> -lshift {
                    (0x80000000u32 as crate::opus_types_h::opus_int32) >> -lshift
                } else {
                    (if result < 0x7fffffffi32 >> -lshift {
                        (0x7fffffffi32) >> -lshift
                    } else {
                        result
                    })
                })
            } else {
                (if result > 0x7fffffffi32 >> -lshift {
                    (0x7fffffffi32) >> -lshift
                } else {
                    (if result < 0x80000000u32 as crate::opus_types_h::opus_int32 >> -lshift {
                        (0x80000000u32 as crate::opus_types_h::opus_int32) >> -lshift
                    } else {
                        result
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << -lshift) as crate::opus_types_h::opus_int32;
        } else if lshift < 32 {
            return result >> lshift;
        } else {
            /* Avoid undefined result */
            return 0i32;
        };
    }
    /* Invert int32 value and return result as int32 in a given Q-domain */
    #[inline]

    pub unsafe extern "C" fn silk_INVERSE32_varQ(
        b32: crate::opus_types_h::opus_int32,
        Qres: i32,
    ) -> crate::opus_types_h::opus_int32
/* I    Q-domain of result (> 0)        */ {
        let mut b_headrm: i32 = 0;
        let mut lshift: i32 = 0;
        let mut b32_inv: crate::opus_types_h::opus_int32 = 0;
        let mut b32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut err_Q32: crate::opus_types_h::opus_int32 = 0;
        let mut result: crate::opus_types_h::opus_int32 = 0;
        /* Compute number of bits head room and normalize input */
        b_headrm = silk_CLZ32((if b32 > 0 { b32 } else { -b32 })) - 1; /* Q: b_headrm                */
        b32_nrm = ((b32 as crate::opus_types_h::opus_uint32) << b_headrm)
            as crate::opus_types_h::opus_int32;
        /* Inverse of b32, with 14 bits of precision */
        b32_inv = (0x7fffffff >> 2) / (b32_nrm >> 16); /* Q: 29 + 16 - b_headrm    */
        /* First approximation */
        result = ((b32_inv as crate::opus_types_h::opus_uint32) << 16)
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Compute residual by subtracting product of denominator and first approximation from one */
        err_Q32 = (((((1) << 29)
            - (b32_nrm as i64 * b32_inv as crate::opus_types_h::opus_int16 as i64 >> 16)
                as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 3) as crate::opus_types_h::opus_int32; /* Q32                        */
        /* Refinement */
        result = (result as i64 + (err_Q32 as i64 * b32_inv as i64 >> 16))
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Convert to Qres domain */
        lshift = 61 - b_headrm - Qres;
        if lshift <= 0 {
            return (((if 0x80000000u32 as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffffi32 >> -lshift
            {
                (if result > 0x80000000u32 as crate::opus_types_h::opus_int32 >> -lshift {
                    (0x80000000u32 as crate::opus_types_h::opus_int32) >> -lshift
                } else {
                    (if result < 0x7fffffffi32 >> -lshift {
                        (0x7fffffffi32) >> -lshift
                    } else {
                        result
                    })
                })
            } else {
                (if result > 0x7fffffffi32 >> -lshift {
                    (0x7fffffffi32) >> -lshift
                } else {
                    (if result < 0x80000000u32 as crate::opus_types_h::opus_int32 >> -lshift {
                        (0x80000000u32 as crate::opus_types_h::opus_int32) >> -lshift
                    } else {
                        result
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << -lshift) as crate::opus_types_h::opus_int32;
        } else if lshift < 32 {
            return result >> lshift;
        } else {
            /* Avoid undefined result */
            return 0i32;
        };
    }
    use crate::opus_types_h::opus_int16;
    use crate::opus_types_h::opus_int32;
    use crate::opus_types_h::opus_uint32;
    use crate::src::opus_1_2_1::silk::NSQ_del_dec::macros_h::silk_CLZ32;
    /* SILK_FIX_INLINES_H */
}

pub mod NSQ_h {
    /* **********************************************************************
    Copyright (c) 2014 Vidyo.
    Copyright (c) 2006-2011, Skype Limited. All rights reserved.
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions
    are met:
    - Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.
    - Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in the
    documentation and/or other materials provided with the distribution.
    - Neither the name of Internet Society, IETF or IETF Trust, nor the
    names of specific contributors, may be used to endorse or promote
    products derived from this software without specific prior written
    permission.
    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
    ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
    LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
    CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
    SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
    INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
    CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
    ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
    POSSIBILITY OF SUCH DAMAGE.
    ***********************************************************************/
    #[inline]

    pub unsafe extern "C" fn silk_noise_shape_quantizer_short_prediction_c(
        mut buf32: *const crate::opus_types_h::opus_int32,
        mut coef16: *const crate::opus_types_h::opus_int16,
        mut order: i32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut out: crate::opus_types_h::opus_int32 = 0;
        /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
        out = order >> 1;
        out = (out as i64 + (*buf32.offset(0) as i64 * *coef16.offset(0) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-1) as i64 * *coef16.offset(1) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-2) as i64 * *coef16.offset(2) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-3) as i64 * *coef16.offset(3) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-4) as i64 * *coef16.offset(4) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-5) as i64 * *coef16.offset(5) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-6) as i64 * *coef16.offset(6) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-7) as i64 * *coef16.offset(7) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-8) as i64 * *coef16.offset(8) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        out = (out as i64 + (*buf32.offset(-9) as i64 * *coef16.offset(9) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        if order == 16 {
            out = (out as i64 + (*buf32.offset(-10) as i64 * *coef16.offset(10) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            out = (out as i64 + (*buf32.offset(-11) as i64 * *coef16.offset(11) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            out = (out as i64 + (*buf32.offset(-12) as i64 * *coef16.offset(12) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            out = (out as i64 + (*buf32.offset(-13) as i64 * *coef16.offset(13) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            out = (out as i64 + (*buf32.offset(-14) as i64 * *coef16.offset(14) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            out = (out as i64 + (*buf32.offset(-15) as i64 * *coef16.offset(15) as i64 >> 16))
                as crate::opus_types_h::opus_int32
        }
        return out;
    }
    use crate::opus_types_h::opus_int32;
    /* SILK_NSQ_H */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
use crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10;
pub use crate::src::opus_1_2_1::silk::NSQ_del_dec::macros_h::silk_CLZ32;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;

pub use crate::src::opus_1_2_1::silk::LPC_analysis_filter::silk_LPC_analysis_filter;
pub use crate::src::opus_1_2_1::silk::NSQ_del_dec::Inlines_h::silk_DIV32_varQ;
pub use crate::src::opus_1_2_1::silk::NSQ_del_dec::Inlines_h::silk_INVERSE32_varQ;
pub use crate::src::opus_1_2_1::silk::NSQ_del_dec::NSQ_h::silk_noise_shape_quantizer_short_prediction_c;
pub use crate::src::opus_1_2_1::silk::NSQ_del_dec::SigProc_FIX_h::silk_min_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSQ_del_dec_struct {
    pub sLPC_Q14: [crate::opus_types_h::opus_int32; 96],
    pub RandState: [crate::opus_types_h::opus_int32; 40],
    pub Q_Q10: [crate::opus_types_h::opus_int32; 40],
    pub Xq_Q14: [crate::opus_types_h::opus_int32; 40],
    pub Pred_Q15: [crate::opus_types_h::opus_int32; 40],
    pub Shape_Q14: [crate::opus_types_h::opus_int32; 40],
    pub sAR2_Q14: [crate::opus_types_h::opus_int32; 24],
    pub LF_AR_Q14: crate::opus_types_h::opus_int32,
    pub Diff_Q14: crate::opus_types_h::opus_int32,
    pub Seed: crate::opus_types_h::opus_int32,
    pub SeedInit: crate::opus_types_h::opus_int32,
    pub RD_Q10: crate::opus_types_h::opus_int32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSQ_sample_struct {
    pub Q_Q10: crate::opus_types_h::opus_int32,
    pub RD_Q10: crate::opus_types_h::opus_int32,
    pub xq_Q14: crate::opus_types_h::opus_int32,
    pub LF_AR_Q14: crate::opus_types_h::opus_int32,
    pub Diff_Q14: crate::opus_types_h::opus_int32,
    pub sLTP_shp_Q14: crate::opus_types_h::opus_int32,
    pub LPC_exc_Q14: crate::opus_types_h::opus_int32,
}

pub type NSQ_sample_pair = [NSQ_sample_struct; 2];
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* Convert Left/Right stereo signal to adaptive Mid/Side representation */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* O    Quantization indices                        */
/* O    Flag: only mid signal coded                 */
/* O    Bitrates for mid and side signals           */
/* I    Total bitrate                               */
/* I    Speech activity level in previous frame     */
/* I    Last frame before a stereo->mono transition */
/* I    Sample rate (kHz)                           */
/* I    Number of samples                           */
/* Convert adaptive Mid/Side representation to Left/Right stereo signal */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* I    Predictors                                  */
/* I    Samples rate (kHz)                          */
/* I    Number of samples                           */
/* Find least-squares prediction gain for one signal based on another and quantize it */
/* O    Returns predictor in Q13                    */
/* O    Ratio of residual and mid energies          */
/* I    Basis signal                                */
/* I    Target signal                               */
/* I/O  Smoothed mid, residual norms                */
/* I    Number of samples                           */
/* I    Smoothing coefficient                       */
/* Quantize mid/side predictors */
/* I/O  Predictors (out: quantized)                 */
/* O    Quantization indices                        */
/* Entropy code the mid/side quantization indices */
/* I/O  Compressor data structure                   */
/* I    Quantization indices                        */
/* Entropy code the mid-only flag */
/* I/O  Compressor data structure                   */
/* Decode mid/side predictors */
/* I/O  Compressor data structure                   */
/* O    Predictors                                  */
/* Decode mid-only flag */
/* I/O  Compressor data structure                   */
/* O    Flag that only mid channel has been coded   */
/* Encodes signs of excitation */
/* I/O  Compressor data structure               */
/* I    pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Decodes signs of excitation */
/* I/O  Compressor data structure               */
/* I/O  pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Check encoder control struct */
/* I    Control structure                           */
/* Control internal sampling rate */
/* I/O  Pointer to Silk encoder state               */
/* I    Control structure                           */
/* Control SNR of redidual quantizer */
/* I/O  Pointer to Silk encoder state               */
/* I    Target max bitrate (bps)                    */
/* **************/
/* Shell coder */
/* **************/
/* Encode quantization indices of excitation */
/* I/O  compressor data structure                   */
/* I    Signal type                                 */
/* I    quantOffsetType                             */
/* I    quantization indices                        */
/* I    Frame length                                */
/* Shell encoder, operates on one shell code frame of 16 pulses */
/* I/O  compressor data structure                   */
/* I    data: nonnegative pulse amplitudes          */
/* Shell decoder, operates on one shell code frame of 16 pulses */
/* O    data: nonnegative pulse amplitudes          */
/* I/O  Compressor data structure                   */
/* I    number of pulses per pulse-subframe         */
/* Gain scalar quantization with hysteresis, uniform on log scale */
/* O    gain indices                                */
/* I/O  gains (quantized out)                       */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                         */
/* Gains scalar dequantization, uniform on log scale */
/* O    quantized gains                             */
/* I    gain indices                                */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                          */
/* Compute unique identifier of gain indices vector */
/* O    returns unique identifier of gains          */
/* I    gain indices                                */
/* I    number of subframes                         */
/* Interpolate two vectors */
/* O    interpolated vector                         */
/* I    first vector                                */
/* I    second vector                               */
/* I    interp. factor, weight on 2nd vector        */
/* I    number of parameters                        */
/* LTP tap quantizer */
/* O    Quantized LTP gains             */
/* O    Codebook Index                  */
/* O    Periodicity Index               */
/* I/O  Cumulative max prediction gain  */
/* O    LTP prediction gain             */
/* I    Correlation matrix in Q18       */
/* I    Correlation vector in Q18       */
/* I    Number of samples per subframe  */
/* I    Number of subframes             */
/* I    Run-time architecture           */
/* Entropy constrained matrix-weighted VQ, for a single input data vector */
/* O    index of best codebook vector               */
/* O    best residual energy                        */
/* O    best total bitrate                          */
/* O    sum of absolute LTP coefficients            */
/* I    correlation matrix                          */
/* I    correlation vector                          */
/* I    codebook                                    */
/* I    codebook effective gain                     */
/* I    code length for each codebook vector        */
/* I    number of samples per subframe              */
/* I    maximum sum of absolute LTP coefficients    */
/* I    number of vectors in codebook               */
/* ***********************************/
/* Noise shaping quantization (NSQ) */
/* ***********************************/
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* Noise shaping using delayed decision */
#[no_mangle]

pub unsafe extern "C" fn silk_NSQ_del_dec_c(
    mut psEncC: *const crate::structs_h::silk_encoder_state,
    mut NSQ: *mut crate::structs_h::silk_nsq_state,
    mut psIndices: *mut crate::structs_h::SideInfoIndices,
    mut x16: *const crate::opus_types_h::opus_int16,
    mut pulses: *mut i8,
    mut PredCoef_Q12: *const crate::opus_types_h::opus_int16,
    mut LTPCoef_Q14: *const crate::opus_types_h::opus_int16,
    mut AR_Q13: *const crate::opus_types_h::opus_int16,
    mut HarmShapeGain_Q14: *const i32,
    mut Tilt_Q14: *const i32,
    mut LF_shp_Q14: *const crate::opus_types_h::opus_int32,
    mut Gains_Q16: *const crate::opus_types_h::opus_int32,
    mut pitchL: *const i32,
    Lambda_Q10: i32,
    LTP_scale_Q14: i32,
)
/* I    LTP state scaling               */
{
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut lag: i32 = 0;
    let mut start_idx: i32 = 0;
    let mut LSF_interpolation_flag: i32 = 0;
    let mut Winner_ind: i32 = 0;
    let mut subfr: i32 = 0;
    let mut last_smple_idx: i32 = 0;
    let mut smpl_buf_idx: i32 = 0;
    let mut decisionDelay: i32 = 0;
    let mut A_Q12: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut B_Q14: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut AR_shp_Q13: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut pxq: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut sLTP_Q15: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut sLTP: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut HarmShapeFIRPacked_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut offset_Q10: i32 = 0;
    let mut RDmin_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut Gain_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut x_sc_Q10: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut delayedGain_Q10: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut psDelDec: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    /* Set unvoiced lag to the previous one, overwrite later for voiced */
    lag = (*NSQ).lagPrev;
    /* Initialize delayed decision states */
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<NSQ_del_dec_struct>())
            .wrapping_mul((*psEncC).nStatesDelayedDecision as usize),
    ); /* index of oldest samples */
    psDelDec = fresh0.as_mut_ptr() as *mut NSQ_del_dec_struct;
    crate::stdlib::memset(
        psDelDec as *mut libc::c_void,
        0,
        ((*psEncC).nStatesDelayedDecision as usize)
            .wrapping_mul(::std::mem::size_of::<NSQ_del_dec_struct>()),
    );
    k = 0;
    while k < (*psEncC).nStatesDelayedDecision {
        psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
        (*psDD).Seed = k + (*psIndices).Seed as i32 & 3;
        (*psDD).SeedInit = (*psDD).Seed;
        (*psDD).RD_Q10 = 0;
        (*psDD).LF_AR_Q14 = (*NSQ).sLF_AR_shp_Q14;
        (*psDD).Diff_Q14 = (*NSQ).sDiff_shp_Q14;
        (*psDD).Shape_Q14[0] = (*NSQ).sLTP_shp_Q14[((*psEncC).ltp_mem_length - 1i32) as usize];
        crate::stdlib::memcpy(
            (*psDD).sLPC_Q14.as_mut_ptr() as *mut libc::c_void,
            (*NSQ).sLPC_Q14.as_mut_ptr() as *const libc::c_void,
            (16usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
        );
        crate::stdlib::memcpy(
            (*psDD).sAR2_Q14.as_mut_ptr() as *mut libc::c_void,
            (*NSQ).sAR2_Q14.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 24]>(),
        );
        k += 1
    }
    offset_Q10 = crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10
        [((*psIndices).signalType as i32 >> 1) as usize][(*psIndices).quantOffsetType as usize]
        as i32;
    smpl_buf_idx = 0;
    decisionDelay = silk_min_int(40, (*psEncC).subfr_length);
    /* For voiced frames limit the decision delay to lower than the pitch lag */
    if (*psIndices).signalType as i32 == 2 {
        k = 0;
        while k < (*psEncC).nb_subfr {
            decisionDelay = silk_min_int(decisionDelay, *pitchL.offset(k as isize) - 5 / 2 - 1);
            k += 1
        }
    } else if lag > 0 {
        decisionDelay = silk_min_int(decisionDelay, lag - 5 / 2 - 1)
    }
    if (*psIndices).NLSFInterpCoef_Q2 as i32 == 4 {
        LSF_interpolation_flag = 0
    } else {
        LSF_interpolation_flag = 1
    }
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>())
            .wrapping_mul(((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize),
    );
    sLTP_Q15 = fresh1.as_mut_ptr();
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>())
            .wrapping_mul(((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize),
    );
    sLTP = fresh2.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>())
            .wrapping_mul((*psEncC).subfr_length as usize),
    );
    x_sc_Q10 = fresh3.as_mut_ptr();
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>()).wrapping_mul(40usize),
    );
    delayedGain_Q10 = fresh4.as_mut_ptr();
    /* Set up pointers to start of sub frame */
    pxq = &mut *(*NSQ)
        .xq
        .as_mut_ptr()
        .offset((*psEncC).ltp_mem_length as isize)
        as *mut crate::opus_types_h::opus_int16;
    (*NSQ).sLTP_shp_buf_idx = (*psEncC).ltp_mem_length;
    (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
    subfr = 0;
    k = 0;
    while k < (*psEncC).nb_subfr {
        A_Q12 = &*PredCoef_Q12.offset(((k >> 1 | 1 - LSF_interpolation_flag) * 16) as isize)
            as *const crate::opus_types_h::opus_int16;
        B_Q14 = &*LTPCoef_Q14.offset((k * 5) as isize) as *const crate::opus_types_h::opus_int16;
        AR_shp_Q13 = &*AR_Q13.offset((k * 24) as isize) as *const crate::opus_types_h::opus_int16;
        /* Noise shape parameters */
        HarmShapeFIRPacked_Q14 = *HarmShapeGain_Q14.offset(k as isize) >> 2;
        HarmShapeFIRPacked_Q14 |= (((*HarmShapeGain_Q14.offset(k as isize) >> 1)
            as crate::opus_types_h::opus_uint32)
            << 16) as crate::opus_types_h::opus_int32;
        (*NSQ).rewhite_flag = 0;
        if (*psIndices).signalType as i32 == 2 {
            /* Voiced */
            lag = *pitchL.offset(k as isize);
            /* Re-whitening */
            if k & 3
                - ((LSF_interpolation_flag as crate::opus_types_h::opus_uint32) << 1)
                    as crate::opus_types_h::opus_int32
                == 0
            {
                if k == 2 {
                    /* RESET DELAYED DECISIONS */
                    /* Find winner */
                    RDmin_Q10 = (*psDelDec.offset(0)).RD_Q10;
                    Winner_ind = 0;
                    i = 1;
                    while i < (*psEncC).nStatesDelayedDecision {
                        if (*psDelDec.offset(i as isize)).RD_Q10 < RDmin_Q10 {
                            RDmin_Q10 = (*psDelDec.offset(i as isize)).RD_Q10;
                            Winner_ind = i
                        }
                        i += 1
                    }
                    i = 0;
                    while i < (*psEncC).nStatesDelayedDecision {
                        if i != Winner_ind {
                            let ref mut fresh5 = (*psDelDec.offset(i as isize)).RD_Q10;
                            *fresh5 += 0x7fffffff >> 4
                        }
                        i += 1
                    }
                    /* Copy final part of signals from winner state to output and long-term filter states */
                    psDD = &mut *psDelDec.offset(Winner_ind as isize) as *mut NSQ_del_dec_struct;
                    last_smple_idx = smpl_buf_idx + decisionDelay;
                    i = 0;
                    while i < decisionDelay {
                        last_smple_idx = (last_smple_idx - 1) % 40;
                        if last_smple_idx < 0 {
                            last_smple_idx += 40
                        }
                        *pulses.offset((i - decisionDelay) as isize) = if 10 == 1 {
                            ((*psDD).Q_Q10[last_smple_idx as usize] >> 1)
                                + ((*psDD).Q_Q10[last_smple_idx as usize] & 1)
                        } else {
                            (((*psDD).Q_Q10[last_smple_idx as usize] >> 10 - 1) + 1) >> 1
                        }
                            as i8;
                        *pxq.offset((i - decisionDelay) as isize) = if (if 14 == 1 {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                * *Gains_Q16.offset(1) as i64
                                >> 16)
                                as crate::opus_types_h::opus_int32
                                >> 1)
                                + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                    * *Gains_Q16.offset(1) as i64
                                    >> 16)
                                    as crate::opus_types_h::opus_int32
                                    & 1)
                        } else {
                            ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                * *Gains_Q16.offset(1) as i64
                                >> 16)
                                as crate::opus_types_h::opus_int32
                                >> 14 - 1)
                                + 1)
                                >> 1
                        }) > 0x7fff
                        {
                            0x7fff
                        } else if (if 14 == 1 {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                * *Gains_Q16.offset(1) as i64
                                >> 16)
                                as crate::opus_types_h::opus_int32
                                >> 1)
                                + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                    * *Gains_Q16.offset(1) as i64
                                    >> 16)
                                    as crate::opus_types_h::opus_int32
                                    & 1)
                        } else {
                            ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                * *Gains_Q16.offset(1) as i64
                                >> 16)
                                as crate::opus_types_h::opus_int32
                                >> 14 - 1)
                                + 1)
                                >> 1
                        }) < 0x8000i32 as crate::opus_types_h::opus_int16 as i32
                        {
                            0x8000i32 as crate::opus_types_h::opus_int16 as i32
                        } else if 14 == 1 {
                            (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                * *Gains_Q16.offset(1) as i64
                                >> 16)
                                as crate::opus_types_h::opus_int32
                                >> 1)
                                + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                    * *Gains_Q16.offset(1) as i64
                                    >> 16)
                                    as crate::opus_types_h::opus_int32
                                    & 1)
                        } else {
                            ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                                * *Gains_Q16.offset(1) as i64
                                >> 16)
                                as crate::opus_types_h::opus_int32
                                >> 14 - 1)
                                + 1)
                                >> 1
                        }
                            as crate::opus_types_h::opus_int16;
                        (*NSQ).sLTP_shp_Q14
                            [((*NSQ).sLTP_shp_buf_idx - decisionDelay + i) as usize] =
                            (*psDD).Shape_Q14[last_smple_idx as usize];
                        i += 1
                    }
                    subfr = 0
                }
                /* Rewhiten with new A coefs */
                start_idx = (*psEncC).ltp_mem_length - lag - (*psEncC).predictLPCOrder - 5 / 2;
                crate::src::opus_1_2_1::silk::LPC_analysis_filter::silk_LPC_analysis_filter(
                    &mut *sLTP.offset(start_idx as isize),
                    &mut *(*NSQ)
                        .xq
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psEncC).subfr_length) as isize),
                    A_Q12,
                    (*psEncC).ltp_mem_length - start_idx,
                    (*psEncC).predictLPCOrder,
                    (*psEncC).arch,
                );
                (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
                (*NSQ).rewhite_flag = 1
            }
        }
        silk_nsq_del_dec_scale_states(
            psEncC,
            NSQ,
            psDelDec,
            x16,
            x_sc_Q10,
            sLTP as *const crate::opus_types_h::opus_int16,
            sLTP_Q15,
            k,
            (*psEncC).nStatesDelayedDecision,
            LTP_scale_Q14,
            Gains_Q16,
            pitchL,
            (*psIndices).signalType as i32,
            decisionDelay,
        );
        let fresh6 = subfr;
        subfr = subfr + 1;
        silk_noise_shape_quantizer_del_dec(
            NSQ,
            psDelDec,
            (*psIndices).signalType as i32,
            x_sc_Q10 as *const crate::opus_types_h::opus_int32,
            pulses,
            pxq,
            sLTP_Q15,
            delayedGain_Q10,
            A_Q12,
            B_Q14,
            AR_shp_Q13,
            lag,
            HarmShapeFIRPacked_Q14,
            *Tilt_Q14.offset(k as isize),
            *LF_shp_Q14.offset(k as isize),
            *Gains_Q16.offset(k as isize),
            Lambda_Q10,
            offset_Q10,
            (*psEncC).subfr_length,
            fresh6,
            (*psEncC).shapingLPCOrder,
            (*psEncC).predictLPCOrder,
            (*psEncC).warping_Q16,
            (*psEncC).nStatesDelayedDecision,
            &mut smpl_buf_idx,
            decisionDelay,
            (*psEncC).arch,
        );
        x16 = x16.offset((*psEncC).subfr_length as isize);
        pulses = pulses.offset((*psEncC).subfr_length as isize);
        pxq = pxq.offset((*psEncC).subfr_length as isize);
        k += 1
    }
    /* Find winner */
    RDmin_Q10 = (*psDelDec.offset(0)).RD_Q10;
    Winner_ind = 0;
    k = 1;
    while k < (*psEncC).nStatesDelayedDecision {
        if (*psDelDec.offset(k as isize)).RD_Q10 < RDmin_Q10 {
            RDmin_Q10 = (*psDelDec.offset(k as isize)).RD_Q10;
            Winner_ind = k
        }
        k += 1
    }
    /* Copy final part of signals from winner state to output and long-term filter states */
    psDD = &mut *psDelDec.offset(Winner_ind as isize) as *mut NSQ_del_dec_struct;
    (*psIndices).Seed = (*psDD).SeedInit as i8;
    last_smple_idx = smpl_buf_idx + decisionDelay;
    Gain_Q10 = *Gains_Q16.offset(((*psEncC).nb_subfr - 1i32) as isize) >> 6;
    i = 0;
    while i < decisionDelay {
        last_smple_idx = (last_smple_idx - 1) % 40;
        if last_smple_idx < 0 {
            last_smple_idx += 40
        }
        *pulses.offset((i - decisionDelay) as isize) = if 10 == 1 {
            ((*psDD).Q_Q10[last_smple_idx as usize] >> 1)
                + ((*psDD).Q_Q10[last_smple_idx as usize] & 1)
        } else {
            (((*psDD).Q_Q10[last_smple_idx as usize] >> 10 - 1) + 1) >> 1
        } as i8;
        *pxq.offset((i - decisionDelay) as isize) = if (if 8 == 1 {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                as crate::opus_types_h::opus_int32
                >> 1)
                + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                    as crate::opus_types_h::opus_int32
                    & 1)
        } else {
            ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                as crate::opus_types_h::opus_int32
                >> 8 - 1)
                + 1)
                >> 1
        }) > 0x7fff
        {
            0x7fff
        } else if (if 8 == 1 {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                as crate::opus_types_h::opus_int32
                >> 1)
                + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                    as crate::opus_types_h::opus_int32
                    & 1)
        } else {
            ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                as crate::opus_types_h::opus_int32
                >> 8 - 1)
                + 1)
                >> 1
        }) < 0x8000i32 as crate::opus_types_h::opus_int16 as i32
        {
            0x8000i32 as crate::opus_types_h::opus_int16 as i32
        } else if 8 == 1 {
            (((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                as crate::opus_types_h::opus_int32
                >> 1)
                + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                    as crate::opus_types_h::opus_int32
                    & 1)
        } else {
            ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64 * Gain_Q10 as i64 >> 16)
                as crate::opus_types_h::opus_int32
                >> 8 - 1)
                + 1)
                >> 1
        } as crate::opus_types_h::opus_int16;
        (*NSQ).sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - decisionDelay + i) as usize] =
            (*psDD).Shape_Q14[last_smple_idx as usize];
        i += 1
    }
    crate::stdlib::memcpy(
        (*NSQ).sLPC_Q14.as_mut_ptr() as *mut libc::c_void,
        &mut *(*psDD)
            .sLPC_Q14
            .as_mut_ptr()
            .offset((*psEncC).subfr_length as isize) as *mut crate::opus_types_h::opus_int32
            as *const libc::c_void,
        (16usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
    crate::stdlib::memcpy(
        (*NSQ).sAR2_Q14.as_mut_ptr() as *mut libc::c_void,
        (*psDD).sAR2_Q14.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 24]>(),
    );
    /* Update states */
    (*NSQ).sLF_AR_shp_Q14 = (*psDD).LF_AR_Q14;
    (*NSQ).sDiff_shp_Q14 = (*psDD).Diff_Q14;
    (*NSQ).lagPrev = *pitchL.offset(((*psEncC).nb_subfr - 1i32) as isize);
    /* Save quantized speech signal */
    crate::stdlib::memmove(
        (*NSQ).xq.as_mut_ptr() as *mut libc::c_void,
        &mut *(*NSQ)
            .xq
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut crate::opus_types_h::opus_int16
            as *const libc::c_void,
        ((*psEncC).ltp_mem_length as usize)
            .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()),
    );
    crate::stdlib::memmove(
        (*NSQ).sLTP_shp_Q14.as_mut_ptr() as *mut libc::c_void,
        &mut *(*NSQ)
            .sLTP_shp_Q14
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut crate::opus_types_h::opus_int32
            as *const libc::c_void,
        ((*psEncC).ltp_mem_length as usize)
            .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
}
/* *****************************************/
/* Noise shape quantizer for one subframe */
/* *****************************************/
/* *****************************************/
/* Noise shape quantizer for one subframe */
/* *****************************************/
#[inline]

unsafe extern "C" fn silk_noise_shape_quantizer_del_dec(
    mut NSQ: *mut crate::structs_h::silk_nsq_state,
    mut psDelDec: *mut NSQ_del_dec_struct,
    mut signalType: i32,
    mut x_Q10: *const crate::opus_types_h::opus_int32,
    mut pulses: *mut i8,
    mut xq: *mut crate::opus_types_h::opus_int16,
    mut sLTP_Q15: *mut crate::opus_types_h::opus_int32,
    mut delayedGain_Q10: *mut crate::opus_types_h::opus_int32,
    mut a_Q12: *const crate::opus_types_h::opus_int16,
    mut b_Q14: *const crate::opus_types_h::opus_int16,
    mut AR_shp_Q13: *const crate::opus_types_h::opus_int16,
    mut lag: i32,
    mut HarmShapeFIRPacked_Q14: crate::opus_types_h::opus_int32,
    mut Tilt_Q14: i32,
    mut LF_shp_Q14: crate::opus_types_h::opus_int32,
    mut Gain_Q16: crate::opus_types_h::opus_int32,
    mut Lambda_Q10: i32,
    mut offset_Q10: i32,
    mut length: i32,
    mut subfr: i32,
    mut shapingLPCOrder: i32,
    mut predictLPCOrder: i32,
    mut warping_Q16: i32,
    mut nStatesDelayedDecision: i32,
    mut smpl_buf_idx: *mut i32,
    mut decisionDelay: i32,
    mut arch: i32,
)
/* I                                        */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut Winner_ind: i32 = 0;
    let mut RDmin_ind: i32 = 0;
    let mut RDmax_ind: i32 = 0;
    let mut last_smple_idx: i32 = 0;
    let mut Winner_rand_state: crate::opus_types_h::opus_int32 = 0;
    let mut LTP_pred_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut LPC_pred_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut n_AR_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut n_LTP_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut n_LF_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut r_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut rr_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut rd1_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut rd2_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut RDmin_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut RDmax_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut q1_Q0: crate::opus_types_h::opus_int32 = 0;
    let mut q1_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut q2_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut exc_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut LPC_exc_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut xq_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut Gain_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut tmp1: crate::opus_types_h::opus_int32 = 0;
    let mut tmp2: crate::opus_types_h::opus_int32 = 0;
    let mut sLF_AR_shp_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut pred_lag_ptr: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut shp_lag_ptr: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut psLPC_Q14: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut psSampleState: *mut NSQ_sample_pair = 0 as *mut NSQ_sample_pair;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    let mut psSS: *mut NSQ_sample_struct = 0 as *mut NSQ_sample_struct;
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<NSQ_sample_pair>()).wrapping_mul(nStatesDelayedDecision as usize),
    );
    psSampleState = fresh7.as_mut_ptr() as *mut NSQ_sample_pair;
    shp_lag_ptr = &mut *(*NSQ)
        .sLTP_shp_Q14
        .as_mut_ptr()
        .offset(((*NSQ).sLTP_shp_buf_idx - lag + 3i32 / 2) as isize)
        as *mut crate::opus_types_h::opus_int32;
    pred_lag_ptr = &mut *sLTP_Q15.offset(((*NSQ).sLTP_buf_idx - lag + 5i32 / 2) as isize)
        as *mut crate::opus_types_h::opus_int32;
    Gain_Q10 = Gain_Q16 >> 6;

    for i in 0..length {
        if signalType == 2 {
            /* Unrolled loop */
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            LTP_pred_Q14 = 2; /* Q13 -> Q14 */
            LTP_pred_Q14 = (LTP_pred_Q14 as i64
                + (*pred_lag_ptr.offset(0) as i64 * *b_Q14.offset(0) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as i64
                + (*pred_lag_ptr.offset(-1) as i64 * *b_Q14.offset(1) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as i64
                + (*pred_lag_ptr.offset(-2) as i64 * *b_Q14.offset(2) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as i64
                + (*pred_lag_ptr.offset(-3) as i64 * *b_Q14.offset(3) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            LTP_pred_Q14 = (LTP_pred_Q14 as i64
                + (*pred_lag_ptr.offset(-4) as i64 * *b_Q14.offset(4) as i64 >> 16))
                as crate::opus_types_h::opus_int32;
            LTP_pred_Q14 = ((LTP_pred_Q14 as crate::opus_types_h::opus_uint32) << 1)
                as crate::opus_types_h::opus_int32;
            pred_lag_ptr = pred_lag_ptr.offset(1)
        } else {
            LTP_pred_Q14 = 0
        }

        if lag > 0 {
            /* Symmetric, packed FIR coefficients */
            n_LTP_Q14 = ((*shp_lag_ptr.offset(0) + *shp_lag_ptr.offset(-2)) as i64
                * HarmShapeFIRPacked_Q14 as crate::opus_types_h::opus_int16 as i64
                >> 16) as crate::opus_types_h::opus_int32; /* Q12 -> Q14 */
            n_LTP_Q14 = (n_LTP_Q14 as i64
                + (*shp_lag_ptr.offset(-1) as i64 * (HarmShapeFIRPacked_Q14 as i64 >> 16) >> 16))
                as crate::opus_types_h::opus_int32;
            n_LTP_Q14 = LTP_pred_Q14
                - ((n_LTP_Q14 as crate::opus_types_h::opus_uint32) << 2)
                    as crate::opus_types_h::opus_int32;
            shp_lag_ptr = shp_lag_ptr.offset(1)
        } else {
            n_LTP_Q14 = 0
        }

        k = 0;

        while k < nStatesDelayedDecision {
            /* Delayed decision state */
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            /* Sample state */
            psSS = (*psSampleState.offset(k as isize)).as_mut_ptr();
            /* Generate dither */
            (*psDD).Seed = (907633515u32).wrapping_add(
                ((*psDD).Seed as crate::opus_types_h::opus_uint32).wrapping_mul(196314165u32),
            ) as crate::opus_types_h::opus_int32;
            /* Pointer used in short term prediction and shaping */
            psLPC_Q14 = &mut *(*psDD).sLPC_Q14.as_mut_ptr().offset((16 - 1 + i) as isize)
                as *mut crate::opus_types_h::opus_int32;
            /* Short-term prediction */
            LPC_pred_Q14 =
                silk_noise_shape_quantizer_short_prediction_c(psLPC_Q14, a_Q12, predictLPCOrder); /* Q10 -> Q14 */
            LPC_pred_Q14 = ((LPC_pred_Q14 as crate::opus_types_h::opus_uint32) << 4)
                as crate::opus_types_h::opus_int32;
            /* Noise shape feedback */
            /* check that order is even */
            /* Output of lowpass section */
            tmp2 = ((*psDD).Diff_Q14 as i64
                + ((*psDD).sAR2_Q14[0] as i64
                    * warping_Q16 as crate::opus_types_h::opus_int16 as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            tmp1 = ((*psDD).sAR2_Q14[0] as i64
                + (((*psDD).sAR2_Q14[1] - tmp2) as i64
                    * warping_Q16 as crate::opus_types_h::opus_int16 as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            (*psDD).sAR2_Q14[0] = tmp2;
            n_AR_Q14 = shapingLPCOrder >> 1;
            n_AR_Q14 = (n_AR_Q14 as i64 + (tmp2 as i64 * *AR_shp_Q13.offset(0) as i64 >> 16))
                as crate::opus_types_h::opus_int32;

            for j in (2..shapingLPCOrder).step_by(2 as usize) {
                tmp2 = ((*psDD).sAR2_Q14[(j - 1) as usize] as i64
                    + (((*psDD).sAR2_Q14[(j + 0) as usize] - tmp1) as i64
                        * warping_Q16 as crate::opus_types_h::opus_int16 as i64
                        >> 16)) as crate::opus_types_h::opus_int32;

                (*psDD).sAR2_Q14[(j - 1) as usize] = tmp1;

                n_AR_Q14 = (n_AR_Q14 as i64
                    + (tmp1 as i64 * *AR_shp_Q13.offset((j - 1) as isize) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;

                tmp1 = ((*psDD).sAR2_Q14[(j + 0) as usize] as i64
                    + (((*psDD).sAR2_Q14[(j + 1) as usize] - tmp2) as i64
                        * warping_Q16 as crate::opus_types_h::opus_int16 as i64
                        >> 16)) as crate::opus_types_h::opus_int32;

                (*psDD).sAR2_Q14[(j + 0) as usize] = tmp2;

                n_AR_Q14 = (n_AR_Q14 as i64
                    + (tmp2 as i64 * *AR_shp_Q13.offset(j as isize) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
            } /* Q12 */
            (*psDD).sAR2_Q14[(shapingLPCOrder - 1i32) as usize] = tmp1; /* Q12 */
            n_AR_Q14 = (n_AR_Q14 as i64
                + (tmp1 as i64 * *AR_shp_Q13.offset((shapingLPCOrder - 1i32) as isize) as i64
                    >> 16)) as crate::opus_types_h::opus_int32; /* Q12 -> Q14 */
            n_AR_Q14 = ((n_AR_Q14 as crate::opus_types_h::opus_uint32) << 1)
                as crate::opus_types_h::opus_int32;
            n_AR_Q14 = (n_AR_Q14 as i64
                + ((*psDD).LF_AR_Q14 as i64 * Tilt_Q14 as crate::opus_types_h::opus_int16 as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            n_AR_Q14 = ((n_AR_Q14 as crate::opus_types_h::opus_uint32) << 2)
                as crate::opus_types_h::opus_int32;
            n_LF_Q14 = ((*psDD).Shape_Q14[*smpl_buf_idx as usize] as i64
                * LF_shp_Q14 as crate::opus_types_h::opus_int16 as i64
                >> 16) as crate::opus_types_h::opus_int32;
            n_LF_Q14 = (n_LF_Q14 as i64
                + ((*psDD).LF_AR_Q14 as i64 * (LF_shp_Q14 as i64 >> 16) >> 16))
                as crate::opus_types_h::opus_int32;
            n_LF_Q14 = ((n_LF_Q14 as crate::opus_types_h::opus_uint32) << 2)
                as crate::opus_types_h::opus_int32;
            tmp1 = n_AR_Q14 + n_LF_Q14;
            tmp2 = n_LTP_Q14 + LPC_pred_Q14;
            tmp1 = tmp2 - tmp1;
            tmp1 = if 4 == 1 {
                (tmp1 >> 1) + (tmp1 & 1)
            } else {
                ((tmp1 >> 4 - 1) + 1) >> 1
            };
            r_Q10 = *x_Q10.offset(i as isize) - tmp1;
            if (*psDD).Seed < 0 {
                r_Q10 = -r_Q10
            }
            r_Q10 = if -((31) << 10) > (30) << 10 {
                if r_Q10 > -((31) << 10) {
                    -((31) << 10)
                } else if r_Q10 < (30) << 10 {
                    (30) << 10
                } else {
                    r_Q10
                }
            } else if r_Q10 > (30) << 10 {
                (30) << 10
            } else if r_Q10 < -((31) << 10) {
                -((31) << 10)
            } else {
                r_Q10
            };
            q1_Q10 = r_Q10 - offset_Q10;
            q1_Q0 = q1_Q10 >> 10;
            if Lambda_Q10 > 2048 {
                /* Input minus prediction plus noise feedback                       */
                /* r = x[ i ] - LTP_pred - LPC_pred + n_AR + n_Tilt + n_LF + n_LTP  */
                /* Q14 */
                /* Q13 */
                /* Q13 */
                /* Q10 */
                /* residual error Q10 */
                /* Flip sign depending on dither */
                /* Find two quantization level candidates and measure their rate-distortion */
                /* For aggressive RDO, the bias becomes more than one pulse. */
                let mut rdo_offset: i32 = Lambda_Q10 / 2 - 512; /* q1_Q0 < -1 */
                if q1_Q10 > rdo_offset {
                    q1_Q0 = q1_Q10 - rdo_offset >> 10
                } else if q1_Q10 < -rdo_offset {
                    q1_Q0 = q1_Q10 + rdo_offset >> 10
                } else if q1_Q10 < 0 {
                    q1_Q0 = -(1)
                } else {
                    q1_Q0 = 0
                }
            }
            if q1_Q0 > 0 {
                q1_Q10 = ((q1_Q0 as crate::opus_types_h::opus_uint32) << 10)
                    as crate::opus_types_h::opus_int32
                    - 80;
                q1_Q10 = q1_Q10 + offset_Q10;
                q2_Q10 = q1_Q10 + 1024;
                rd1_Q10 = q1_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32;
                rd2_Q10 = q2_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
            } else if q1_Q0 == 0 {
                q1_Q10 = offset_Q10;
                q2_Q10 = q1_Q10 + (1024 - 80);
                rd1_Q10 = q1_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32;
                rd2_Q10 = q2_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
            } else if q1_Q0 == -(1) {
                q2_Q10 = offset_Q10;
                q1_Q10 = q2_Q10 - (1024 - 80);
                rd1_Q10 = -q1_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32;
                rd2_Q10 = q2_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
            } else {
                q1_Q10 = ((q1_Q0 as crate::opus_types_h::opus_uint32) << 10)
                    as crate::opus_types_h::opus_int32
                    + 80;
                q1_Q10 = q1_Q10 + offset_Q10;
                q2_Q10 = q1_Q10 + 1024;
                rd1_Q10 = -q1_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32;
                rd2_Q10 = -q2_Q10 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * Lambda_Q10 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
            }
            rr_Q10 = r_Q10 - q1_Q10;
            rd1_Q10 = rd1_Q10
                + rr_Q10 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * rr_Q10 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                >> 10;
            rr_Q10 = r_Q10 - q2_Q10;
            rd2_Q10 = rd2_Q10
                + rr_Q10 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * rr_Q10 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                >> 10;
            if rd1_Q10 < rd2_Q10 {
                (*psSS.offset(0)).RD_Q10 = (*psDD).RD_Q10 + rd1_Q10;
                (*psSS.offset(1)).RD_Q10 = (*psDD).RD_Q10 + rd2_Q10;
                (*psSS.offset(0)).Q_Q10 = q1_Q10;
                (*psSS.offset(1)).Q_Q10 = q2_Q10
            } else {
                (*psSS.offset(0)).RD_Q10 = (*psDD).RD_Q10 + rd2_Q10;
                (*psSS.offset(1)).RD_Q10 = (*psDD).RD_Q10 + rd1_Q10;
                (*psSS.offset(0)).Q_Q10 = q2_Q10;
                (*psSS.offset(1)).Q_Q10 = q1_Q10
            }
            exc_Q14 = (((*psSS.offset(0)).Q_Q10 as crate::opus_types_h::opus_uint32) << 4)
                as crate::opus_types_h::opus_int32;
            if (*psDD).Seed < 0 {
                exc_Q14 = -exc_Q14
            }
            LPC_exc_Q14 = exc_Q14 + LTP_pred_Q14;
            xq_Q14 = LPC_exc_Q14 + LPC_pred_Q14;
            (*psSS.offset(0)).Diff_Q14 = xq_Q14
                - ((*x_Q10.offset(i as isize) as crate::opus_types_h::opus_uint32) << 4)
                    as crate::opus_types_h::opus_int32;
            sLF_AR_shp_Q14 = (*psSS.offset(0)).Diff_Q14 - n_AR_Q14;
            (*psSS.offset(0)).sLTP_shp_Q14 = sLF_AR_shp_Q14 - n_LF_Q14;
            (*psSS.offset(0)).LF_AR_Q14 = sLF_AR_shp_Q14;
            (*psSS.offset(0)).LPC_exc_Q14 = LPC_exc_Q14;
            (*psSS.offset(0)).xq_Q14 = xq_Q14;
            exc_Q14 = (((*psSS.offset(1)).Q_Q10 as crate::opus_types_h::opus_uint32) << 4)
                as crate::opus_types_h::opus_int32;
            if (*psDD).Seed < 0 {
                exc_Q14 = -exc_Q14
            }
            LPC_exc_Q14 = exc_Q14 + LTP_pred_Q14;
            xq_Q14 = LPC_exc_Q14 + LPC_pred_Q14;
            (*psSS.offset(1)).Diff_Q14 = xq_Q14
                - ((*x_Q10.offset(i as isize) as crate::opus_types_h::opus_uint32) << 4)
                    as crate::opus_types_h::opus_int32;
            sLF_AR_shp_Q14 = (*psSS.offset(1)).Diff_Q14 - n_AR_Q14;
            (*psSS.offset(1)).sLTP_shp_Q14 = sLF_AR_shp_Q14 - n_LF_Q14;
            (*psSS.offset(1)).LF_AR_Q14 = sLF_AR_shp_Q14;
            (*psSS.offset(1)).LPC_exc_Q14 = LPC_exc_Q14;
            (*psSS.offset(1)).xq_Q14 = xq_Q14;
            k += 1
        }

        *smpl_buf_idx = (*smpl_buf_idx - 1) % 40;

        if *smpl_buf_idx < 0 {
            *smpl_buf_idx += 40
        }

        last_smple_idx = (*smpl_buf_idx + decisionDelay) % 40;

        RDmin_Q10 = (*psSampleState.offset(0))[0].RD_Q10;

        Winner_ind = 0;

        k = 1;

        while k < nStatesDelayedDecision {
            if (*psSampleState.offset(k as isize))[0].RD_Q10 < RDmin_Q10 {
                RDmin_Q10 = (*psSampleState.offset(k as isize))[0].RD_Q10;
                Winner_ind = k
            }
            k += 1
        }

        Winner_rand_state =
            (*psDelDec.offset(Winner_ind as isize)).RandState[last_smple_idx as usize];

        k = 0;

        while k < nStatesDelayedDecision {
            if (*psDelDec.offset(k as isize)).RandState[last_smple_idx as usize]
                != Winner_rand_state
            {
                (*psSampleState.offset(k as isize))[0].RD_Q10 =
                    (*psSampleState.offset(k as isize))[0].RD_Q10 + (0x7fffffff >> 4);
                (*psSampleState.offset(k as isize))[1].RD_Q10 =
                    (*psSampleState.offset(k as isize))[1].RD_Q10 + (0x7fffffff >> 4)
            }
            k += 1
        }

        RDmax_Q10 = (*psSampleState.offset(0))[0].RD_Q10;

        RDmin_Q10 = (*psSampleState.offset(0))[1].RD_Q10;

        RDmax_ind = 0;

        RDmin_ind = 0;

        k = 1;

        while k < nStatesDelayedDecision {
            /* find worst in first set */
            if (*psSampleState.offset(k as isize))[0].RD_Q10 > RDmax_Q10 {
                RDmax_Q10 = (*psSampleState.offset(k as isize))[0].RD_Q10;
                RDmax_ind = k
            }
            /* find best in second set */
            if (*psSampleState.offset(k as isize))[1].RD_Q10 < RDmin_Q10 {
                RDmin_Q10 = (*psSampleState.offset(k as isize))[1].RD_Q10;
                RDmin_ind = k
            }
            k += 1
        }

        if RDmin_Q10 < RDmax_Q10 {
            crate::stdlib::memcpy(
                (&mut *psDelDec.offset(RDmax_ind as isize) as *mut NSQ_del_dec_struct
                    as *mut crate::opus_types_h::opus_int32)
                    .offset(i as isize) as *mut libc::c_void,
                (&mut *psDelDec.offset(RDmin_ind as isize) as *mut NSQ_del_dec_struct
                    as *mut crate::opus_types_h::opus_int32)
                    .offset(i as isize) as *const libc::c_void,
                (::std::mem::size_of::<NSQ_del_dec_struct>()).wrapping_sub(
                    (i as usize)
                        .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
                ),
            );
            crate::stdlib::memcpy(
                &mut *(*psSampleState.offset(RDmax_ind as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut NSQ_sample_struct as *mut libc::c_void,
                &mut *(*psSampleState.offset(RDmin_ind as isize))
                    .as_mut_ptr()
                    .offset(1isize) as *mut NSQ_sample_struct
                    as *const libc::c_void,
                ::std::mem::size_of::<NSQ_sample_struct>(),
            );
        }

        psDD = &mut *psDelDec.offset(Winner_ind as isize) as *mut NSQ_del_dec_struct;

        if subfr > 0 || i >= decisionDelay {
            *pulses.offset((i - decisionDelay) as isize) = if 10 == 1 {
                ((*psDD).Q_Q10[last_smple_idx as usize] >> 1)
                    + ((*psDD).Q_Q10[last_smple_idx as usize] & 1)
            } else {
                (((*psDD).Q_Q10[last_smple_idx as usize] >> 10 - 1) + 1) >> 1
            } as i8;
            *xq.offset((i - decisionDelay) as isize) = if (if 8 == 1 {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                    >> 16) as crate::opus_types_h::opus_int32
                    >> 1)
                    + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                        * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                        >> 16) as crate::opus_types_h::opus_int32
                        & 1)
            } else {
                ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                    >> 16) as crate::opus_types_h::opus_int32
                    >> 8 - 1)
                    + 1)
                    >> 1
            }) > 0x7fff
            {
                0x7fff
            } else if (if 8 == 1 {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                    >> 16) as crate::opus_types_h::opus_int32
                    >> 1)
                    + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                        * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                        >> 16) as crate::opus_types_h::opus_int32
                        & 1)
            } else {
                ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                    >> 16) as crate::opus_types_h::opus_int32
                    >> 8 - 1)
                    + 1)
                    >> 1
            }) < 0x8000i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000i32 as crate::opus_types_h::opus_int16 as i32
            } else if 8 == 1 {
                (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                    >> 16) as crate::opus_types_h::opus_int32
                    >> 1)
                    + (((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                        * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                        >> 16) as crate::opus_types_h::opus_int32
                        & 1)
            } else {
                ((((*psDD).Xq_Q14[last_smple_idx as usize] as i64
                    * *delayedGain_Q10.offset(last_smple_idx as isize) as i64
                    >> 16) as crate::opus_types_h::opus_int32
                    >> 8 - 1)
                    + 1)
                    >> 1
            }
                as crate::opus_types_h::opus_int16;
            (*NSQ).sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - decisionDelay) as usize] =
                (*psDD).Shape_Q14[last_smple_idx as usize];
            *sLTP_Q15.offset(((*NSQ).sLTP_buf_idx - decisionDelay) as isize) =
                (*psDD).Pred_Q15[last_smple_idx as usize]
        }

        (*NSQ).sLTP_shp_buf_idx += 1;

        (*NSQ).sLTP_buf_idx += 1;

        k = 0;

        while k < nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
            psSS = &mut *(*psSampleState.offset(k as isize)).as_mut_ptr().offset(0)
                as *mut NSQ_sample_struct;
            (*psDD).LF_AR_Q14 = (*psSS).LF_AR_Q14;
            (*psDD).Diff_Q14 = (*psSS).Diff_Q14;
            (*psDD).sLPC_Q14[(16 + i) as usize] = (*psSS).xq_Q14;
            (*psDD).Xq_Q14[*smpl_buf_idx as usize] = (*psSS).xq_Q14;
            (*psDD).Q_Q10[*smpl_buf_idx as usize] = (*psSS).Q_Q10;
            (*psDD).Pred_Q15[*smpl_buf_idx as usize] =
                (((*psSS).LPC_exc_Q14 as crate::opus_types_h::opus_uint32) << 1)
                    as crate::opus_types_h::opus_int32;
            (*psDD).Shape_Q14[*smpl_buf_idx as usize] = (*psSS).sLTP_shp_Q14;
            (*psDD).Seed = ((*psDD).Seed as crate::opus_types_h::opus_uint32).wrapping_add(
                (if 10 == 1 {
                    ((*psSS).Q_Q10 >> 1) + ((*psSS).Q_Q10 & 1)
                } else {
                    (((*psSS).Q_Q10 >> 10 - 1) + 1) >> 1
                }) as crate::opus_types_h::opus_uint32,
            ) as crate::opus_types_h::opus_int32;
            (*psDD).RandState[*smpl_buf_idx as usize] = (*psDD).Seed;
            (*psDD).RD_Q10 = (*psSS).RD_Q10;
            k += 1
        }

        *delayedGain_Q10.offset(*smpl_buf_idx as isize) = Gain_Q10;
    }
    /* Update LPC states */
    k = 0;
    while k < nStatesDelayedDecision {
        psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;
        crate::stdlib::memcpy(
            (*psDD).sLPC_Q14.as_mut_ptr() as *mut libc::c_void,
            &mut *(*psDD).sLPC_Q14.as_mut_ptr().offset(length as isize)
                as *mut crate::opus_types_h::opus_int32 as *const libc::c_void,
            (16usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
        );
        k += 1
    }
}
/* OVERRIDE_silk_noise_shape_quantizer_del_dec */
#[inline]

unsafe extern "C" fn silk_nsq_del_dec_scale_states(
    mut psEncC: *const crate::structs_h::silk_encoder_state,
    mut NSQ: *mut crate::structs_h::silk_nsq_state,
    mut psDelDec: *mut NSQ_del_dec_struct,
    mut x16: *const crate::opus_types_h::opus_int16,
    mut x_sc_Q10: *mut crate::opus_types_h::opus_int32,
    mut sLTP: *const crate::opus_types_h::opus_int16,
    mut sLTP_Q15: *mut crate::opus_types_h::opus_int32,
    mut subfr: i32,
    mut nStatesDelayedDecision: i32,
    LTP_scale_Q14: i32,
    mut Gains_Q16: *const crate::opus_types_h::opus_int32,
    mut pitchL: *const i32,
    signal_type: i32,
    decisionDelay: i32,
)
/* I    Decision delay                      */
{
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut lag: i32 = 0;
    let mut gain_adj_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut inv_gain_Q31: crate::opus_types_h::opus_int32 = 0;
    let mut inv_gain_Q26: crate::opus_types_h::opus_int32 = 0;
    let mut psDD: *mut NSQ_del_dec_struct = 0 as *mut NSQ_del_dec_struct;
    lag = *pitchL.offset(subfr as isize);
    inv_gain_Q31 = silk_INVERSE32_varQ(
        if *Gains_Q16.offset(subfr as isize) > 1 {
            *Gains_Q16.offset(subfr as isize)
        } else {
            1
        },
        47,
    );
    /* Scale input */
    inv_gain_Q26 = if 5 == 1 {
        (inv_gain_Q31 >> 1) + (inv_gain_Q31 & 1)
    } else {
        ((inv_gain_Q31 >> 5 - 1) + 1) >> 1
    };
    i = 0;
    while i < (*psEncC).subfr_length {
        *x_sc_Q10.offset(i as isize) = (*x16.offset(i as isize) as i64 * inv_gain_Q26 as i64 >> 16)
            as crate::opus_types_h::opus_int32;
        i += 1
    }
    /* After rewhitening the LTP state is un-scaled, so scale with inv_gain_Q16 */
    if (*NSQ).rewhite_flag != 0 {
        if subfr == 0 {
            /* Do LTP downscaling */
            inv_gain_Q31 = (((inv_gain_Q31 as i64
                * LTP_scale_Q14 as crate::opus_types_h::opus_int16 as i64
                >> 16) as crate::opus_types_h::opus_uint32)
                << 2) as crate::opus_types_h::opus_int32
        }
        i = (*NSQ).sLTP_buf_idx - lag - 5 / 2;
        while i < (*NSQ).sLTP_buf_idx {
            *sLTP_Q15.offset(i as isize) = (inv_gain_Q31 as i64 * *sLTP.offset(i as isize) as i64
                >> 16)
                as crate::opus_types_h::opus_int32;
            i += 1
        }
    }
    /* Adjust for changing gain */
    if *Gains_Q16.offset(subfr as isize) != (*NSQ).prev_gain_Q16 {
        gain_adj_Q16 = silk_DIV32_varQ((*NSQ).prev_gain_Q16, *Gains_Q16.offset(subfr as isize), 16);
        /* Scale long-term shaping state */
        i = (*NSQ).sLTP_shp_buf_idx - (*psEncC).ltp_mem_length;
        while i < (*NSQ).sLTP_shp_buf_idx {
            (*NSQ).sLTP_shp_Q14[i as usize] =
                (gain_adj_Q16 as i64 * (*NSQ).sLTP_shp_Q14[i as usize] as i64 >> 16)
                    as crate::opus_types_h::opus_int32;
            i += 1
        }
        /* Scale long-term prediction state */
        if signal_type == 2 && (*NSQ).rewhite_flag == 0 {
            i = (*NSQ).sLTP_buf_idx - lag - 5 / 2;
            while i < (*NSQ).sLTP_buf_idx - decisionDelay {
                *sLTP_Q15.offset(i as isize) =
                    (gain_adj_Q16 as i64 * *sLTP_Q15.offset(i as isize) as i64 >> 16)
                        as crate::opus_types_h::opus_int32;
                i += 1
            }
        }

        for k in 0..nStatesDelayedDecision {
            psDD = &mut *psDelDec.offset(k as isize) as *mut NSQ_del_dec_struct;

            (*psDD).LF_AR_Q14 = (gain_adj_Q16 as i64 * (*psDD).LF_AR_Q14 as i64 >> 16)
                as crate::opus_types_h::opus_int32;

            (*psDD).Diff_Q14 = (gain_adj_Q16 as i64 * (*psDD).Diff_Q14 as i64 >> 16)
                as crate::opus_types_h::opus_int32;

            i = 0;

            while i < 16 {
                (*psDD).sLPC_Q14[i as usize] =
                    (gain_adj_Q16 as i64 * (*psDD).sLPC_Q14[i as usize] as i64 >> 16)
                        as crate::opus_types_h::opus_int32;
                i += 1
            }

            i = 0;

            while i < 24 {
                (*psDD).sAR2_Q14[i as usize] =
                    (gain_adj_Q16 as i64 * (*psDD).sAR2_Q14[i as usize] as i64 >> 16)
                        as crate::opus_types_h::opus_int32;
                i += 1
            }

            i = 0;

            while i < 40 {
                (*psDD).Pred_Q15[i as usize] =
                    (gain_adj_Q16 as i64 * (*psDD).Pred_Q15[i as usize] as i64 >> 16)
                        as crate::opus_types_h::opus_int32;
                (*psDD).Shape_Q14[i as usize] =
                    (gain_adj_Q16 as i64 * (*psDD).Shape_Q14[i as usize] as i64 >> 16)
                        as crate::opus_types_h::opus_int32;
                i += 1
            }
        }
        /* Save inverse gain */
        (*NSQ).prev_gain_Q16 = *Gains_Q16.offset(subfr as isize)
    };
}
