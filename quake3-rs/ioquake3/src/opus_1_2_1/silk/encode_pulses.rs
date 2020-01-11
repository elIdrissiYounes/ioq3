use ::libc;

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
use crate::stdlib::memset;

use crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf;
use crate::src::opus_1_2_1::silk::code_signs::silk_encode_signs;
use crate::src::opus_1_2_1::silk::shell_coder::silk_shell_encoder;
use crate::src::opus_1_2_1::silk::tables_other::silk_lsb_iCDF;
use crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_max_pulses_table;
use crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_pulses_per_block_BITS_Q5;
use crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_pulses_per_block_iCDF;
use crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_rate_levels_BITS_Q5;
use crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_rate_levels_iCDF;
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
/* ********************************************/
/* Encode quantization indices of excitation */
/* ********************************************/
#[inline]

unsafe extern "C" fn combine_and_check(
    mut pulses_comb: *mut libc::c_int,
    mut pulses_in: *const libc::c_int,
    mut max_pulses: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int
/* I    number of output values        */ {
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        sum = *pulses_in.offset((2 as libc::c_int * k) as isize)
            + *pulses_in.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize);
        if sum > max_pulses {
            return 1 as libc::c_int;
        }
        *pulses_comb.offset(k as isize) = sum;
        k += 1
    }
    return 0 as libc::c_int;
}
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
/* Encode quantization indices of excitation */
#[no_mangle]

pub unsafe extern "C" fn silk_encode_pulses(
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    signalType: libc::c_int,
    quantOffsetType: libc::c_int,
    mut pulses: *mut libc::c_schar,
    frame_length: libc::c_int,
)
/* I    Frame length                                */
{
    let mut i: libc::c_int = 0; /* Fixing Valgrind reported problem*/
    let mut k: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut nLS: libc::c_int = 0;
    let mut scale_down: libc::c_int = 0;
    let mut RateLevelIndex: libc::c_int = 0 as libc::c_int;
    let mut abs_q: crate::opus_types_h::opus_int32 = 0;
    let mut minSumBits_Q5: crate::opus_types_h::opus_int32 = 0;
    let mut sumBits_Q5: crate::opus_types_h::opus_int32 = 0;
    let mut abs_pulses: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sum_pulses: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nRshifts: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pulses_comb: [libc::c_int; 8] = [0; 8];
    let mut abs_pulses_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pulses_ptr: *const libc::c_schar = 0 as *const libc::c_schar;
    let mut cdf_ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut nBits_ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    crate::stdlib::memset(
        pulses_comb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    /* ***************************/
    /* Prepare for shell coding */
    /* ***************************/
    /* Calculate number of shell blocks */
    iter = frame_length >> 4 as libc::c_int;
    if (iter * 16 as libc::c_int) < frame_length {
        /* Make sure only happens for 10 ms @ 12 kHz */
        iter += 1;
        crate::stdlib::memset(
            &mut *pulses.offset(frame_length as isize) as *mut libc::c_schar as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_schar>() as libc::c_ulong),
        );
    }
    /* Take the absolute value of the pulses */
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((iter * 16 as libc::c_int) as libc::c_ulong) as usize,
    );
    abs_pulses = fresh0.as_mut_ptr() as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < iter * 16 as libc::c_int {
        *abs_pulses.offset((i + 0 as libc::c_int) as isize) =
            if *pulses.offset((i + 0 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 0 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 0 as libc::c_int) as isize) as libc::c_int)
            };
        *abs_pulses.offset((i + 1 as libc::c_int) as isize) =
            if *pulses.offset((i + 1 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 1 as libc::c_int) as isize) as libc::c_int)
            };
        *abs_pulses.offset((i + 2 as libc::c_int) as isize) =
            if *pulses.offset((i + 2 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 2 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 2 as libc::c_int) as isize) as libc::c_int)
            };
        *abs_pulses.offset((i + 3 as libc::c_int) as isize) =
            if *pulses.offset((i + 3 as libc::c_int) as isize) as libc::c_int > 0 as libc::c_int {
                *pulses.offset((i + 3 as libc::c_int) as isize) as libc::c_int
            } else {
                -(*pulses.offset((i + 3 as libc::c_int) as isize) as libc::c_int)
            };
        i += 4 as libc::c_int
    }
    /* Calc sum pulses per shell code frame */
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(iter as libc::c_ulong)
            as usize,
    );
    sum_pulses = fresh1.as_mut_ptr() as *mut libc::c_int;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(iter as libc::c_ulong)
            as usize,
    );
    nRshifts = fresh2.as_mut_ptr() as *mut libc::c_int;
    abs_pulses_ptr = abs_pulses;
    i = 0 as libc::c_int;
    while i < iter {
        *nRshifts.offset(i as isize) = 0 as libc::c_int;
        loop {
            /* 1+1 -> 2 */
            scale_down = combine_and_check(
                pulses_comb.as_mut_ptr(),
                abs_pulses_ptr,
                crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_max_pulses_table
                    [0 as libc::c_int as usize] as libc::c_int,
                8 as libc::c_int,
            );
            /* 2+2 -> 4 */
            scale_down += combine_and_check(
                pulses_comb.as_mut_ptr(),
                pulses_comb.as_mut_ptr(),
                crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_max_pulses_table
                    [1 as libc::c_int as usize] as libc::c_int,
                4 as libc::c_int,
            );
            /* 4+4 -> 8 */
            scale_down += combine_and_check(
                pulses_comb.as_mut_ptr(),
                pulses_comb.as_mut_ptr(),
                crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_max_pulses_table
                    [2 as libc::c_int as usize] as libc::c_int,
                2 as libc::c_int,
            );
            /* 8+8 -> 16 */
            scale_down += combine_and_check(
                &mut *sum_pulses.offset(i as isize),
                pulses_comb.as_mut_ptr(),
                crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_max_pulses_table
                    [3 as libc::c_int as usize] as libc::c_int,
                1 as libc::c_int,
            );
            if !(scale_down != 0) {
                break;
            }
            /* We need to downscale the quantization signal */
            let ref mut fresh3 = *nRshifts.offset(i as isize);
            *fresh3 += 1;
            k = 0 as libc::c_int;
            while k < 16 as libc::c_int {
                *abs_pulses_ptr.offset(k as isize) =
                    *abs_pulses_ptr.offset(k as isize) >> 1 as libc::c_int;
                k += 1
            }
        }
        abs_pulses_ptr = abs_pulses_ptr.offset(16 as libc::c_int as isize);
        i += 1
    }
    /* *************/
    /* Rate level */
    /* *************/
    /* find rate level that leads to fewest bits for coding of pulses per block info */
    minSumBits_Q5 = 0x7fffffff as libc::c_int;
    k = 0 as libc::c_int;
    while k < 10 as libc::c_int - 1 as libc::c_int {
        nBits_ptr =
            crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_pulses_per_block_BITS_Q5
                [k as usize]
                .as_ptr();
        sumBits_Q5 = crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_rate_levels_BITS_Q5
            [(signalType >> 1 as libc::c_int) as usize][k as usize]
            as crate::opus_types_h::opus_int32;
        i = 0 as libc::c_int;
        while i < iter {
            if *nRshifts.offset(i as isize) > 0 as libc::c_int {
                sumBits_Q5 += *nBits_ptr.offset((16 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_int
            } else {
                sumBits_Q5 +=
                    *nBits_ptr.offset(*sum_pulses.offset(i as isize) as isize) as libc::c_int
            }
            i += 1
        }
        if sumBits_Q5 < minSumBits_Q5 {
            minSumBits_Q5 = sumBits_Q5;
            RateLevelIndex = k
        }
        k += 1
    }
    crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        RateLevelIndex,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_rate_levels_iCDF
            [(signalType >> 1 as libc::c_int) as usize]
            .as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    /* **************************************************/
    /* Sum-Weighted-Pulses Encoding                    */
    /* **************************************************/
    cdf_ptr = crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_pulses_per_block_iCDF
        [RateLevelIndex as usize]
        .as_ptr();
    i = 0 as libc::c_int;
    while i < iter {
        if *nRshifts.offset(i as isize) == 0 as libc::c_int {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                *sum_pulses.offset(i as isize),
                cdf_ptr,
                8 as libc::c_int as libc::c_uint,
            );
        } else {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                16 as libc::c_int + 1 as libc::c_int,
                cdf_ptr,
                8 as libc::c_int as libc::c_uint,
            );
            k = 0 as libc::c_int;
            while k < *nRshifts.offset(i as isize) - 1 as libc::c_int {
                crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx, 16 as libc::c_int + 1 as libc::c_int,
                            crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_pulses_per_block_iCDF[(10 as libc::c_int -
                                                            1 as libc::c_int)
                                                           as usize].as_ptr(),
                            8 as libc::c_int as libc::c_uint);
                k += 1
            }
            crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                *sum_pulses.offset(i as isize),
                crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_pulses_per_block_iCDF
                    [(10 as libc::c_int - 1 as libc::c_int) as usize]
                    .as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        }
        i += 1
    }
    /* *****************/
    /* Shell Encoding */
    /* *****************/
    i = 0 as libc::c_int;
    while i < iter {
        if *sum_pulses.offset(i as isize) > 0 as libc::c_int {
            crate::src::opus_1_2_1::silk::shell_coder::silk_shell_encoder(
                psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                &mut *abs_pulses.offset((i * 16 as libc::c_int) as isize),
            );
        }
        i += 1
    }
    /* ***************/
    /* LSB Encoding */
    /* ***************/
    i = 0 as libc::c_int;
    while i < iter {
        if *nRshifts.offset(i as isize) > 0 as libc::c_int {
            pulses_ptr =
                &mut *pulses.offset((i * 16 as libc::c_int) as isize) as *mut libc::c_schar;
            nLS = *nRshifts.offset(i as isize) - 1 as libc::c_int;
            k = 0 as libc::c_int;
            while k < 16 as libc::c_int {
                abs_q = if *pulses_ptr.offset(k as isize) as libc::c_int > 0 as libc::c_int {
                    *pulses_ptr.offset(k as isize) as libc::c_int
                } else {
                    -(*pulses_ptr.offset(k as isize) as libc::c_int)
                } as libc::c_schar as crate::opus_types_h::opus_int32;
                j = nLS;
                while j > 0 as libc::c_int {
                    bit = abs_q >> j & 1 as libc::c_int;
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        bit,
                        crate::src::opus_1_2_1::silk::tables_other::silk_lsb_iCDF.as_ptr(),
                        8 as libc::c_int as libc::c_uint,
                    );
                    j -= 1
                }
                bit = abs_q & 1 as libc::c_int;
                crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                    psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    bit,
                    crate::src::opus_1_2_1::silk::tables_other::silk_lsb_iCDF.as_ptr(),
                    8 as libc::c_int as libc::c_uint,
                );
                k += 1
            }
        }
        i += 1
    }
    /* ***************/
    /* Encode signs */
    /* ***************/
    crate::src::opus_1_2_1::silk::code_signs::silk_encode_signs(
        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        pulses as *const libc::c_schar,
        frame_length,
        signalType,
        quantOffsetType,
        sum_pulses as *const libc::c_int,
    );
}
