pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
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
/* Piece-wise linear mapping from bitrate in kbps to coding quality in dB SNR */
#[no_mangle]

pub static mut silk_TargetRate_table_NB: [crate::opus_types_h::opus_int32; 8] =
    [0, 8000, 9400, 11500, 13500, 17500, 25000, 80000];
#[no_mangle]

pub static mut silk_TargetRate_table_MB: [crate::opus_types_h::opus_int32; 8] =
    [0, 9000, 12000, 14500, 18500, 24500, 35500, 80000];
#[no_mangle]

pub static mut silk_TargetRate_table_WB: [crate::opus_types_h::opus_int32; 8] =
    [0, 10500, 14000, 17000, 21500, 28500, 42000, 80000];
#[no_mangle]

pub static mut silk_SNR_table_Q1: [crate::opus_types_h::opus_int16; 8] =
    [18, 29, 38, 40, 46, 52, 62, 84];
/* Tables for stereo predictor coding */
#[no_mangle]

pub static mut silk_stereo_pred_quant_Q13: [crate::opus_types_h::opus_int16; 16] = [
    -13732, -10050, -8266, -7526, -6500, -5000, -2950, -820, 820, 2950, 5000, 6500, 7526, 8266,
    10050, 13732,
];
#[no_mangle]

pub static mut silk_stereo_pred_joint_iCDF: [u8; 25] = [
    249, 247, 246, 245, 244, 234, 210, 202, 201, 200, 197, 174, 82, 59, 56, 55, 54, 46, 22, 12, 11,
    10, 9, 7, 0,
];
#[no_mangle]

pub static mut silk_stereo_only_code_mid_iCDF: [u8; 2] = [64, 0];
/* Tables for LBRR flags */

static mut silk_LBRR_flags_2_iCDF: [u8; 3] = [203, 150, 0];

static mut silk_LBRR_flags_3_iCDF: [u8; 7] = [215, 195, 166, 125, 110, 82, 0];
#[no_mangle]

pub static mut silk_LBRR_flags_iCDF_ptr: [*const u8; 2] = unsafe {
    [
        silk_LBRR_flags_2_iCDF.as_ptr(),
        silk_LBRR_flags_3_iCDF.as_ptr(),
    ]
};
/* Table for LSB coding */
#[no_mangle]

pub static mut silk_lsb_iCDF: [u8; 2] = [120, 0];
/* Tables for LTPScale */
#[no_mangle]

pub static mut silk_LTPscale_iCDF: [u8; 3] = [128, 64, 0];
/* Tables for signal type and offset coding */
#[no_mangle]

pub static mut silk_type_offset_VAD_iCDF: [u8; 4] = [232, 158, 10, 0];
#[no_mangle]

pub static mut silk_type_offset_no_VAD_iCDF: [u8; 2] = [230, 0];
/* Tables for NLSF interpolation factor */
#[no_mangle]

pub static mut silk_NLSF_interpolation_factor_iCDF: [u8; 5] = [243, 221, 192, 181, 0];
/* Quantization offsets */
#[no_mangle]

pub static mut silk_Quantization_Offsets_Q10: [[crate::opus_types_h::opus_int16; 2]; 2] =
    [[100, 240], [32, 100]];
/* Table for LTPScale */
#[no_mangle]

pub static mut silk_LTPScales_table_Q14: [crate::opus_types_h::opus_int16; 3] =
    [15565, 12288, 8192];
/* Uniform entropy tables */
#[no_mangle]

pub static mut silk_uniform3_iCDF: [u8; 3] = [171, 85, 0];
#[no_mangle]

pub static mut silk_uniform4_iCDF: [u8; 4] = [192, 128, 64, 0];
#[no_mangle]

pub static mut silk_uniform5_iCDF: [u8; 5] = [205, 154, 102, 51, 0];
#[no_mangle]

pub static mut silk_uniform6_iCDF: [u8; 6] = [213, 171, 128, 85, 43, 0];
#[no_mangle]

pub static mut silk_uniform8_iCDF: [u8; 8] = [224, 192, 160, 128, 96, 64, 32, 0];
#[no_mangle]

pub static mut silk_NLSF_EXT_iCDF: [u8; 7] = [100, 40, 16, 7, 3, 1, 0];
/*  Elliptic/Cauer filters designed with 0.1 dB passband ripple,
80 dB minimum stopband attenuation, and
[0.95 : 0.15 : 0.35] normalized cut off frequencies. */
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
#[no_mangle]

pub static mut silk_Transition_LP_B_Q28: [[crate::opus_types_h::opus_int32; 3]; 5] = [
    [250767114, 501534038, 250767114],
    [209867381, 419732057, 209867381],
    [170987846, 341967853, 170987846],
    [131531482, 263046905, 131531482],
    [89306658, 178584282, 89306658],
];
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
#[no_mangle]

pub static mut silk_Transition_LP_A_Q28: [[crate::opus_types_h::opus_int32; 2]; 5] = [
    [506393414, 239854379],
    [411067935, 169683996],
    [306733530, 116694253],
    [185807084, 77959395],
    [35497197, 57401098],
];
