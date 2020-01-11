pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;
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
/* Entropy constrained matrix-weighted VQ, hard-coded to 5-element vectors, for a single input data vector */
#[no_mangle]

pub unsafe extern "C" fn silk_VQ_WMat_EC_c(
    mut ind: *mut i8,
    mut res_nrg_Q15: *mut crate::opus_types_h::opus_int32,
    mut rate_dist_Q8: *mut crate::opus_types_h::opus_int32,
    mut gain_Q7: *mut i32,
    mut XX_Q17: *const crate::opus_types_h::opus_int32,
    mut xX_Q17: *const crate::opus_types_h::opus_int32,
    mut cb_Q7: *const i8,
    mut cb_gain_Q7: *const u8,
    mut cl_Q5: *const u8,
    subfr_len: i32,
    max_gain_Q7: crate::opus_types_h::opus_int32,
    L: i32,
)
/* I    number of vectors in codebook               */
{
    let mut k: i32 = 0;
    let mut gain_tmp_Q7: i32 = 0;
    let mut cb_row_Q7: *const i8 = 0 as *const i8;
    let mut neg_xX_Q24: [crate::opus_types_h::opus_int32; 5] = [0; 5];
    let mut sum1_Q15: crate::opus_types_h::opus_int32 = 0;
    let mut sum2_Q24: crate::opus_types_h::opus_int32 = 0;
    let mut bits_res_Q8: crate::opus_types_h::opus_int32 = 0;
    let mut bits_tot_Q8: crate::opus_types_h::opus_int32 = 0;
    /* Negate and convert to new Q domain */
    neg_xX_Q24[0] = -(((*xX_Q17.offset(0) as crate::opus_types_h::opus_uint32) << 7)
        as crate::opus_types_h::opus_int32);
    neg_xX_Q24[1] = -(((*xX_Q17.offset(1) as crate::opus_types_h::opus_uint32) << 7)
        as crate::opus_types_h::opus_int32);
    neg_xX_Q24[2] = -(((*xX_Q17.offset(2) as crate::opus_types_h::opus_uint32) << 7)
        as crate::opus_types_h::opus_int32);
    neg_xX_Q24[3] = -(((*xX_Q17.offset(3) as crate::opus_types_h::opus_uint32) << 7)
        as crate::opus_types_h::opus_int32);
    neg_xX_Q24[4] = -(((*xX_Q17.offset(4) as crate::opus_types_h::opus_uint32) << 7)
        as crate::opus_types_h::opus_int32);
    /* Loop over codebook */
    *rate_dist_Q8 = 0x7fffffff;
    *res_nrg_Q15 = 0x7fffffff;
    cb_row_Q7 = cb_Q7;
    /* In things go really bad, at least *ind is set to something safe. */
    *ind = 0i8;
    k = 0;
    while k < L {
        let mut penalty: crate::opus_types_h::opus_int32 = 0;
        gain_tmp_Q7 = *cb_gain_Q7.offset(k as isize) as i32;
        /* Weighted rate */
        /* Quantization error: 1 - 2 * xX * cb + cb' * XX * cb */
        sum1_Q15 = (1.001 * ((1i64) << 15) as f64 + 0.5) as crate::opus_types_h::opus_int32;
        /* Penalty for too large gain */
        penalty = (((if gain_tmp_Q7 - max_gain_Q7 > 0 {
            (gain_tmp_Q7) - max_gain_Q7
        } else {
            0
        }) as crate::opus_types_h::opus_uint32)
            << 11) as crate::opus_types_h::opus_int32;
        /* first row of XX_Q17 */
        sum2_Q24 = neg_xX_Q24[0] + *XX_Q17.offset(1) * *cb_row_Q7.offset(1) as i32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(2) * *cb_row_Q7.offset(2) as i32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(3) * *cb_row_Q7.offset(3) as i32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(4) * *cb_row_Q7.offset(4) as i32;
        sum2_Q24 = ((sum2_Q24 as crate::opus_types_h::opus_uint32) << 1)
            as crate::opus_types_h::opus_int32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(0) * *cb_row_Q7.offset(0) as i32;
        sum1_Q15 = (sum1_Q15 as i64 + (sum2_Q24 as i64 * *cb_row_Q7.offset(0) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        /* second row of XX_Q17 */
        sum2_Q24 = neg_xX_Q24[1] + *XX_Q17.offset(7) * *cb_row_Q7.offset(2) as i32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(8) * *cb_row_Q7.offset(3) as i32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(9) * *cb_row_Q7.offset(4) as i32;
        sum2_Q24 = ((sum2_Q24 as crate::opus_types_h::opus_uint32) << 1)
            as crate::opus_types_h::opus_int32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(6) * *cb_row_Q7.offset(1) as i32;
        sum1_Q15 = (sum1_Q15 as i64 + (sum2_Q24 as i64 * *cb_row_Q7.offset(1) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        /* third row of XX_Q17 */
        sum2_Q24 = neg_xX_Q24[2] + *XX_Q17.offset(13) * *cb_row_Q7.offset(3) as i32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(14) * *cb_row_Q7.offset(4) as i32;
        sum2_Q24 = ((sum2_Q24 as crate::opus_types_h::opus_uint32) << 1)
            as crate::opus_types_h::opus_int32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(12) * *cb_row_Q7.offset(2) as i32;
        sum1_Q15 = (sum1_Q15 as i64 + (sum2_Q24 as i64 * *cb_row_Q7.offset(2) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        /* fourth row of XX_Q17 */
        sum2_Q24 = neg_xX_Q24[3] + *XX_Q17.offset(19) * *cb_row_Q7.offset(4) as i32;
        sum2_Q24 = ((sum2_Q24 as crate::opus_types_h::opus_uint32) << 1)
            as crate::opus_types_h::opus_int32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(18) * *cb_row_Q7.offset(3) as i32;
        sum1_Q15 = (sum1_Q15 as i64 + (sum2_Q24 as i64 * *cb_row_Q7.offset(3) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        /* last row of XX_Q17 */
        sum2_Q24 = ((neg_xX_Q24[4] as crate::opus_types_h::opus_uint32) << 1)
            as crate::opus_types_h::opus_int32;
        sum2_Q24 = sum2_Q24 + *XX_Q17.offset(24) * *cb_row_Q7.offset(4) as i32;
        sum1_Q15 = (sum1_Q15 as i64 + (sum2_Q24 as i64 * *cb_row_Q7.offset(4) as i64 >> 16))
            as crate::opus_types_h::opus_int32;
        /* find best */
        if sum1_Q15 >= 0 {
            /* Translate residual energy to bits using high-rate assumption (6 dB ==> 1 bit/sample) */
            bits_res_Q8 = subfr_len as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * (crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(sum1_Q15 + penalty)
                    - ((15) << 7)) as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32;
            /* In the following line we reduce the codelength component by half ("-1"); seems to slghtly improve quality */
            bits_tot_Q8 = bits_res_Q8
                + ((*cl_Q5.offset(k as isize) as crate::opus_types_h::opus_uint32) << 3 - 1)
                    as crate::opus_types_h::opus_int32;
            if bits_tot_Q8 <= *rate_dist_Q8 {
                *rate_dist_Q8 = bits_tot_Q8;
                *res_nrg_Q15 = sum1_Q15 + penalty;
                *ind = k as i8;
                *gain_Q7 = gain_tmp_Q7
            }
        }
        /* Go to next cbk vector */
        cb_row_Q7 = cb_row_Q7.offset(5);
        k += 1
    }
}
