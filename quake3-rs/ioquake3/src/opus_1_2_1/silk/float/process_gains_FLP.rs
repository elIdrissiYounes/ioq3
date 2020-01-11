use ::libc;

pub mod SigProc_FLP_h {
    /* *******************************************************************/
    /*                                MACROS                            */
    /* *******************************************************************/
    /* sigmoid function */
    #[inline]

    pub unsafe extern "C" fn silk_sigmoid(mut x: f32) -> f32 {
        return (1.0 / (1.0 + crate::stdlib::exp(-x as f64))) as f32;
    }

    /* SILK_SIGPROC_FLP_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::src::opus_1_2_1::silk::float::process_gains_FLP::SigProc_FLP_h::silk_sigmoid;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;

pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::structs_FLP_h::silk_encoder_control_FLP;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;
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
/* ********************/
/* Encoder Functions */
/* ********************/
/* High-pass filter with cutoff frequency adaptation based on pitch lag statistics */
/* I/O  Encoder states                              */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* O    Number of payload bytes;                    */
/* I/O  compressor data structure                   */
/* I    The type of conditional coding to use       */
/* I    If > 0: maximum number of output bits       */
/* I    Flag to force constant-bitrate operation    */
/* Initializes the Silk encoder state */
/* I/O  Encoder state FLP                           */
/* I    Run-tim architecture                        */
/* Control the Silk encoder */
/* I/O  Pointer to Silk encoder state FLP           */
/* I    Control structure                           */
/* I    Flag to allow switching audio bandwidth     */
/* I    Channel number                              */
/* *************************/
/* Noise shaping analysis */
/* *************************/
/* Compute noise shaping coefficients and initial gain values */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    LPC residual from pitch analysis            */
/* I    Input signal [frame_length + la_shape]      */
/* Autocorrelations for a warped frequency axis */
/* O    Result [order + 1]                          */
/* I    Input data to correlate                     */
/* I    Warping coefficient                         */
/* I    Length of input                             */
/* I    Correlation order (even)                    */
/* Calculation of LTP state scaling */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    The type of conditional coding to use       */
/* *********************************************/
/* Prediction Analysis                        */
/* *********************************************/
/* Find pitch lags */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* O    Residual                                    */
/* I    Speech signal                               */
/* I    Run-time architecture                       */
/* Find LPC and LTP coefficients */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    Residual from pitch analysis                */
/* I    Speech signal                               */
/* I    The type of conditional coding to use       */
/* LPC analysis */
/* I/O  Encoder state                               */
/* O    NLSFs                                       */
/* I    Input signal                                */
/* I    Prediction gain from LTP (dB)               */
/* LTP analysis */
/* O    Weight for LTP quantization         */
/* O    Weight for LTP quantization                 */
/* I    LPC residual                                */
/* I    LTP lags                                    */
/* I    Subframe length                             */
/* I    number of subframes                         */
/* O    LTP res MAX_NB_SUBFR*(pre_lgth+subfr_lngth) */
/* I    Input signal, with preceding samples        */
/* I    LTP coefficients for each subframe          */
/* I    Pitch lags                                  */
/* I    Inverse quantization gains                  */
/* I    Length of each subframe                     */
/* I    number of subframes                         */
/* I    Preceding samples for each subframe         */
/* Calculates residual energies of input subframes where all subframes have LPC_order   */
/* of preceding samples                                                                 */
/* O    Residual energy per subframe                */
/* I    Input signal                                */
/* I    AR coefs for each frame half                */
/* I    Quantization gains                          */
/* I    Subframe length                             */
/* I    number of subframes                         */
/* I    LPC order                                   */
/* 16th order LPC analysis filter */
/* O    LPC residual signal                         */
/* I    LPC coefficients                            */
/* I    Input signal                                */
/* I    Length of input signal                      */
/* I    LPC order                                   */
/* LTP tap quantizer */
/* O    Quantized LTP gains                         */
/* O    Codebook index                              */
/* O    Periodicity index                           */
/* I/O  Cumulative max prediction gain  */
/* O    LTP prediction gain                         */
/* I    Correlation matrix                  */
/* I    Correlation vector                          */
/* I    Number of samples per subframe              */
/* I    Number of subframes                         */
/* I    Run-time architecture                       */
/* Residual energy: nrg = wxx - 2 * wXx * c + c' * wXX * c */
/* O    Weighted residual energy                    */
/* I    Filter coefficients                         */
/* I/O  Weighted correlation matrix, reg. out       */
/* I    Weighted correlation vector                 */
/* I    Weighted correlation value                  */
/* I    Dimension                                   */
/* Processing of gains */
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
/* Processing of gains */
#[no_mangle]

pub unsafe extern "C" fn silk_process_gains_FLP(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut psEncCtrl: *mut crate::structs_FLP_h::silk_encoder_control_FLP,
    mut condCoding: i32,
)
/* I    The type of conditional coding to use       */
{
    let mut psShapeSt: *mut crate::structs_FLP_h::silk_shape_state_FLP = &mut (*psEnc).sShape;
    let mut k: i32 = 0;
    let mut pGains_Q16: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut s: f32 = 0.;
    let mut InvMaxSqrVal: f32 = 0.;
    let mut gain: f32 = 0.;
    let mut quant_offset: f32 = 0.;
    /* Gain reduction when LTP coding gain is high */
    if (*psEnc).sCmn.indices.signalType as i32 == 2 {
        s = 1.0 - 0.5 * silk_sigmoid(0.25 * ((*psEncCtrl).LTPredCodGain - 12.0));
        k = 0;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl).Gains[k as usize] *= s;
            k += 1
        }
    }
    /* Limit the quantized signal */
    InvMaxSqrVal = (crate::stdlib::pow(
        2f64,
        (0.33 * (21.0 - (*psEnc).sCmn.SNR_dB_Q7 as f32 * (1f32 / 128.0))) as f64,
    ) / (*psEnc).sCmn.subfr_length as f64) as f32;
    k = 0;
    while k < (*psEnc).sCmn.nb_subfr {
        /* Soft limit on ratio residual energy and squared gains */
        gain = (*psEncCtrl).Gains[k as usize];
        gain = crate::stdlib::sqrt(
            (gain * gain + (*psEncCtrl).ResNrg[k as usize] * InvMaxSqrVal) as f64,
        ) as f32;
        (*psEncCtrl).Gains[k as usize] = if gain < 32767.0 { gain } else { 32767.0 };
        k += 1
    }
    /* Prepare gains for noise shaping quantization */
    k = 0;
    while k < (*psEnc).sCmn.nb_subfr {
        pGains_Q16[k as usize] =
            ((*psEncCtrl).Gains[k as usize] * 65536.0f32) as crate::opus_types_h::opus_int32;
        k += 1
    }
    /* Save unquantized gains and gain Index */
    crate::stdlib::memcpy(
        (*psEncCtrl).GainsUnq_Q16.as_mut_ptr() as *mut libc::c_void,
        pGains_Q16.as_mut_ptr() as *const libc::c_void,
        ((*psEnc).sCmn.nb_subfr as usize)
            .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
    (*psEncCtrl).lastGainIndexPrev = (*psShapeSt).LastGainIndex;
    /* Quantize gains */
    crate::src::opus_1_2_1::silk::gain_quant::silk_gains_quant(
        (*psEnc).sCmn.indices.GainsIndices.as_mut_ptr(),
        pGains_Q16.as_mut_ptr(),
        &mut (*psShapeSt).LastGainIndex,
        (condCoding == 2) as i32,
        (*psEnc).sCmn.nb_subfr,
    );
    /* Overwrite unquantized gains with quantized gains and convert back to Q0 from Q16 */
    k = 0;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psEncCtrl).Gains[k as usize] = pGains_Q16[k as usize] as f32 / 65536.0;
        k += 1
    }
    /* Set quantizer offset for voiced signals. Larger offset when LTP coding gain is low or tilt is high (ie low-pass) */
    if (*psEnc).sCmn.indices.signalType as i32 == 2 {
        if (*psEncCtrl).LTPredCodGain + (*psEnc).sCmn.input_tilt_Q15 as f32 * (1.0 / 32768.0) > 1.0
        {
            (*psEnc).sCmn.indices.quantOffsetType = 0i8
        } else {
            (*psEnc).sCmn.indices.quantOffsetType = 1i8
        }
    }
    /* Quantizer boundary adjustment */
    quant_offset = crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10
        [((*psEnc).sCmn.indices.signalType as i32 >> 1) as usize]
        [(*psEnc).sCmn.indices.quantOffsetType as usize] as i32 as f32
        / 1024.0;
    (*psEncCtrl).Lambda = 1.2
        + -0.05 * (*psEnc).sCmn.nStatesDelayedDecision as f32
        + -0.2 * (*psEnc).sCmn.speech_activity_Q8 as f32 * (1.0 / 256.0)
        + -0.1 * (*psEncCtrl).input_quality
        + -0.2 * (*psEncCtrl).coding_quality
        + 0.8 * quant_offset;
}
