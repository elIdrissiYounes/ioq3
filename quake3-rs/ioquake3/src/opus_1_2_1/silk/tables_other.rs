use ::libc;

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

pub static mut silk_TargetRate_table_NB: [crate::opus_types_h::opus_int32; 8] = [
    0 as libc::c_int,
    8000 as libc::c_int,
    9400 as libc::c_int,
    11500 as libc::c_int,
    13500 as libc::c_int,
    17500 as libc::c_int,
    25000 as libc::c_int,
    80000 as libc::c_int,
];
#[no_mangle]

pub static mut silk_TargetRate_table_MB: [crate::opus_types_h::opus_int32; 8] = [
    0 as libc::c_int,
    9000 as libc::c_int,
    12000 as libc::c_int,
    14500 as libc::c_int,
    18500 as libc::c_int,
    24500 as libc::c_int,
    35500 as libc::c_int,
    80000 as libc::c_int,
];
#[no_mangle]

pub static mut silk_TargetRate_table_WB: [crate::opus_types_h::opus_int32; 8] = [
    0 as libc::c_int,
    10500 as libc::c_int,
    14000 as libc::c_int,
    17000 as libc::c_int,
    21500 as libc::c_int,
    28500 as libc::c_int,
    42000 as libc::c_int,
    80000 as libc::c_int,
];
#[no_mangle]

pub static mut silk_SNR_table_Q1: [crate::opus_types_h::opus_int16; 8] = [
    18 as libc::c_int as crate::opus_types_h::opus_int16,
    29 as libc::c_int as crate::opus_types_h::opus_int16,
    38 as libc::c_int as crate::opus_types_h::opus_int16,
    40 as libc::c_int as crate::opus_types_h::opus_int16,
    46 as libc::c_int as crate::opus_types_h::opus_int16,
    52 as libc::c_int as crate::opus_types_h::opus_int16,
    62 as libc::c_int as crate::opus_types_h::opus_int16,
    84 as libc::c_int as crate::opus_types_h::opus_int16,
];
/* Tables for stereo predictor coding */
#[no_mangle]

pub static mut silk_stereo_pred_quant_Q13: [crate::opus_types_h::opus_int16; 16] = [
    -(13732 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(10050 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8266 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7526 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6500 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5000 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(2950 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(820 as libc::c_int) as crate::opus_types_h::opus_int16,
    820 as libc::c_int as crate::opus_types_h::opus_int16,
    2950 as libc::c_int as crate::opus_types_h::opus_int16,
    5000 as libc::c_int as crate::opus_types_h::opus_int16,
    6500 as libc::c_int as crate::opus_types_h::opus_int16,
    7526 as libc::c_int as crate::opus_types_h::opus_int16,
    8266 as libc::c_int as crate::opus_types_h::opus_int16,
    10050 as libc::c_int as crate::opus_types_h::opus_int16,
    13732 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_stereo_pred_joint_iCDF: [libc::c_uchar; 25] = [
    249 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_stereo_only_code_mid_iCDF: [libc::c_uchar; 2] = [
    64 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
/* Tables for LBRR flags */

static mut silk_LBRR_flags_2_iCDF: [libc::c_uchar; 3] = [
    203 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

static mut silk_LBRR_flags_3_iCDF: [libc::c_uchar; 7] = [
    215 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_LBRR_flags_iCDF_ptr: [*const libc::c_uchar; 2] = unsafe {
    [
        silk_LBRR_flags_2_iCDF.as_ptr(),
        silk_LBRR_flags_3_iCDF.as_ptr(),
    ]
};
/* Table for LSB coding */
#[no_mangle]

pub static mut silk_lsb_iCDF: [libc::c_uchar; 2] = [
    120 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
/* Tables for LTPScale */
#[no_mangle]

pub static mut silk_LTPscale_iCDF: [libc::c_uchar; 3] = [
    128 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
/* Tables for signal type and offset coding */
#[no_mangle]

pub static mut silk_type_offset_VAD_iCDF: [libc::c_uchar; 4] = [
    232 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_type_offset_no_VAD_iCDF: [libc::c_uchar; 2] = [
    230 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
/* Tables for NLSF interpolation factor */
#[no_mangle]

pub static mut silk_NLSF_interpolation_factor_iCDF: [libc::c_uchar; 5] = [
    243 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
/* Quantization offsets */
#[no_mangle]

pub static mut silk_Quantization_Offsets_Q10: [[crate::opus_types_h::opus_int16; 2]; 2] = [
    [
        100 as libc::c_int as crate::opus_types_h::opus_int16,
        240 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        32 as libc::c_int as crate::opus_types_h::opus_int16,
        100 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
];
/* Table for LTPScale */
#[no_mangle]

pub static mut silk_LTPScales_table_Q14: [crate::opus_types_h::opus_int16; 3] = [
    15565 as libc::c_int as crate::opus_types_h::opus_int16,
    12288 as libc::c_int as crate::opus_types_h::opus_int16,
    8192 as libc::c_int as crate::opus_types_h::opus_int16,
];
/* Uniform entropy tables */
#[no_mangle]

pub static mut silk_uniform3_iCDF: [libc::c_uchar; 3] = [
    171 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_uniform4_iCDF: [libc::c_uchar; 4] = [
    192 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_uniform5_iCDF: [libc::c_uchar; 5] = [
    205 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_uniform6_iCDF: [libc::c_uchar; 6] = [
    213 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_uniform8_iCDF: [libc::c_uchar; 8] = [
    224 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_NLSF_EXT_iCDF: [libc::c_uchar; 7] = [
    100 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
/*  Elliptic/Cauer filters designed with 0.1 dB passband ripple,
80 dB minimum stopband attenuation, and
[0.95 : 0.15 : 0.35] normalized cut off frequencies. */
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
#[no_mangle]

pub static mut silk_Transition_LP_B_Q28: [[crate::opus_types_h::opus_int32; 3]; 5] = [
    [
        250767114 as libc::c_int,
        501534038 as libc::c_int,
        250767114 as libc::c_int,
    ],
    [
        209867381 as libc::c_int,
        419732057 as libc::c_int,
        209867381 as libc::c_int,
    ],
    [
        170987846 as libc::c_int,
        341967853 as libc::c_int,
        170987846 as libc::c_int,
    ],
    [
        131531482 as libc::c_int,
        263046905 as libc::c_int,
        131531482 as libc::c_int,
    ],
    [
        89306658 as libc::c_int,
        178584282 as libc::c_int,
        89306658 as libc::c_int,
    ],
];
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
#[no_mangle]

pub static mut silk_Transition_LP_A_Q28: [[crate::opus_types_h::opus_int32; 2]; 5] = [
    [506393414 as libc::c_int, 239854379 as libc::c_int],
    [411067935 as libc::c_int, 169683996 as libc::c_int],
    [306733530 as libc::c_int, 116694253 as libc::c_int],
    [185807084 as libc::c_int, 77959395 as libc::c_int],
    [35497197 as libc::c_int, 57401098 as libc::c_int],
];
