use ::libc;

pub use crate::opus_types_h::opus_int16;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::int16_t;
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
/* Filter coefficients for IIR/FIR polyphase resampling     *
 * Total size: 179 Words (358 Bytes)                        */
/* Matlab code for the notch filter coefficients: */
/* B = [1, 0.147, 1];  A = [1, 0.107, 0.89]; G = 0.93; freqz(G * B, A, 2^14, 16e3); axis([0, 8000, -10, 1]) */
/* fprintf('\t%6d, %6d, %6d, %6d\n', round(B(2)*2^16), round(-A(2)*2^16), round((1-A(3))*2^16), round(G*2^15)) */
/* const opus_int16 silk_resampler_up2_hq_notch[ 4 ] = { 9634,  -7012,   7209,  30474 }; */
/* Tables with IIR and FIR coefficients for fractional downsamplers (123 Words) */
#[no_mangle]

pub static mut silk_Resampler_3_4_COEFS: [crate::opus_types_h::opus_int16; 29] = [
    -(20694 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(13867 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(49 as libc::c_int) as crate::opus_types_h::opus_int16,
    64 as libc::c_int as crate::opus_types_h::opus_int16,
    17 as libc::c_int as crate::opus_types_h::opus_int16,
    -(157 as libc::c_int) as crate::opus_types_h::opus_int16,
    353 as libc::c_int as crate::opus_types_h::opus_int16,
    -(496 as libc::c_int) as crate::opus_types_h::opus_int16,
    163 as libc::c_int as crate::opus_types_h::opus_int16,
    11047 as libc::c_int as crate::opus_types_h::opus_int16,
    22205 as libc::c_int as crate::opus_types_h::opus_int16,
    -(39 as libc::c_int) as crate::opus_types_h::opus_int16,
    6 as libc::c_int as crate::opus_types_h::opus_int16,
    91 as libc::c_int as crate::opus_types_h::opus_int16,
    -(170 as libc::c_int) as crate::opus_types_h::opus_int16,
    186 as libc::c_int as crate::opus_types_h::opus_int16,
    23 as libc::c_int as crate::opus_types_h::opus_int16,
    -(896 as libc::c_int) as crate::opus_types_h::opus_int16,
    6336 as libc::c_int as crate::opus_types_h::opus_int16,
    19928 as libc::c_int as crate::opus_types_h::opus_int16,
    -(19 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(36 as libc::c_int) as crate::opus_types_h::opus_int16,
    102 as libc::c_int as crate::opus_types_h::opus_int16,
    -(89 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(24 as libc::c_int) as crate::opus_types_h::opus_int16,
    328 as libc::c_int as crate::opus_types_h::opus_int16,
    -(951 as libc::c_int) as crate::opus_types_h::opus_int16,
    2568 as libc::c_int as crate::opus_types_h::opus_int16,
    15909 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_2_3_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    -(14457 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(14019 as libc::c_int) as crate::opus_types_h::opus_int16,
    64 as libc::c_int as crate::opus_types_h::opus_int16,
    128 as libc::c_int as crate::opus_types_h::opus_int16,
    -(122 as libc::c_int) as crate::opus_types_h::opus_int16,
    36 as libc::c_int as crate::opus_types_h::opus_int16,
    310 as libc::c_int as crate::opus_types_h::opus_int16,
    -(768 as libc::c_int) as crate::opus_types_h::opus_int16,
    584 as libc::c_int as crate::opus_types_h::opus_int16,
    9267 as libc::c_int as crate::opus_types_h::opus_int16,
    17733 as libc::c_int as crate::opus_types_h::opus_int16,
    12 as libc::c_int as crate::opus_types_h::opus_int16,
    128 as libc::c_int as crate::opus_types_h::opus_int16,
    18 as libc::c_int as crate::opus_types_h::opus_int16,
    -(142 as libc::c_int) as crate::opus_types_h::opus_int16,
    288 as libc::c_int as crate::opus_types_h::opus_int16,
    -(117 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(865 as libc::c_int) as crate::opus_types_h::opus_int16,
    4123 as libc::c_int as crate::opus_types_h::opus_int16,
    14459 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_2_COEFS: [crate::opus_types_h::opus_int16; 14] = [
    616 as libc::c_int as crate::opus_types_h::opus_int16,
    -(14323 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(10 as libc::c_int) as crate::opus_types_h::opus_int16,
    39 as libc::c_int as crate::opus_types_h::opus_int16,
    58 as libc::c_int as crate::opus_types_h::opus_int16,
    -(46 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(84 as libc::c_int) as crate::opus_types_h::opus_int16,
    120 as libc::c_int as crate::opus_types_h::opus_int16,
    184 as libc::c_int as crate::opus_types_h::opus_int16,
    -(315 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(541 as libc::c_int) as crate::opus_types_h::opus_int16,
    1284 as libc::c_int as crate::opus_types_h::opus_int16,
    5380 as libc::c_int as crate::opus_types_h::opus_int16,
    9024 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_3_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    16102 as libc::c_int as crate::opus_types_h::opus_int16,
    -(15162 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(13 as libc::c_int) as crate::opus_types_h::opus_int16,
    0 as libc::c_int as crate::opus_types_h::opus_int16,
    20 as libc::c_int as crate::opus_types_h::opus_int16,
    26 as libc::c_int as crate::opus_types_h::opus_int16,
    5 as libc::c_int as crate::opus_types_h::opus_int16,
    -(31 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(43 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4 as libc::c_int) as crate::opus_types_h::opus_int16,
    65 as libc::c_int as crate::opus_types_h::opus_int16,
    90 as libc::c_int as crate::opus_types_h::opus_int16,
    7 as libc::c_int as crate::opus_types_h::opus_int16,
    -(157 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(248 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(44 as libc::c_int) as crate::opus_types_h::opus_int16,
    593 as libc::c_int as crate::opus_types_h::opus_int16,
    1583 as libc::c_int as crate::opus_types_h::opus_int16,
    2612 as libc::c_int as crate::opus_types_h::opus_int16,
    3271 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_4_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    22500 as libc::c_int as crate::opus_types_h::opus_int16,
    -(15099 as libc::c_int) as crate::opus_types_h::opus_int16,
    3 as libc::c_int as crate::opus_types_h::opus_int16,
    -(14 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(20 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(15 as libc::c_int) as crate::opus_types_h::opus_int16,
    2 as libc::c_int as crate::opus_types_h::opus_int16,
    25 as libc::c_int as crate::opus_types_h::opus_int16,
    37 as libc::c_int as crate::opus_types_h::opus_int16,
    25 as libc::c_int as crate::opus_types_h::opus_int16,
    -(16 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(71 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(107 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(79 as libc::c_int) as crate::opus_types_h::opus_int16,
    50 as libc::c_int as crate::opus_types_h::opus_int16,
    292 as libc::c_int as crate::opus_types_h::opus_int16,
    623 as libc::c_int as crate::opus_types_h::opus_int16,
    982 as libc::c_int as crate::opus_types_h::opus_int16,
    1288 as libc::c_int as crate::opus_types_h::opus_int16,
    1464 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_6_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    27540 as libc::c_int as crate::opus_types_h::opus_int16,
    -(15257 as libc::c_int) as crate::opus_types_h::opus_int16,
    17 as libc::c_int as crate::opus_types_h::opus_int16,
    12 as libc::c_int as crate::opus_types_h::opus_int16,
    8 as libc::c_int as crate::opus_types_h::opus_int16,
    1 as libc::c_int as crate::opus_types_h::opus_int16,
    -(10 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(22 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(30 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(32 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(22 as libc::c_int) as crate::opus_types_h::opus_int16,
    3 as libc::c_int as crate::opus_types_h::opus_int16,
    44 as libc::c_int as crate::opus_types_h::opus_int16,
    100 as libc::c_int as crate::opus_types_h::opus_int16,
    168 as libc::c_int as crate::opus_types_h::opus_int16,
    243 as libc::c_int as crate::opus_types_h::opus_int16,
    317 as libc::c_int as crate::opus_types_h::opus_int16,
    381 as libc::c_int as crate::opus_types_h::opus_int16,
    429 as libc::c_int as crate::opus_types_h::opus_int16,
    455 as libc::c_int as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_2_3_COEFS_LQ: [crate::opus_types_h::opus_int16; 6] = [
    -(2797 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6507 as libc::c_int) as crate::opus_types_h::opus_int16,
    4697 as libc::c_int as crate::opus_types_h::opus_int16,
    10739 as libc::c_int as crate::opus_types_h::opus_int16,
    1567 as libc::c_int as crate::opus_types_h::opus_int16,
    8276 as libc::c_int as crate::opus_types_h::opus_int16,
];
/* Table with interplation fractions of 1/24, 3/24, 5/24, ... , 23/24 : 23/24 (46 Words) */
#[no_mangle]

pub static mut silk_resampler_frac_FIR_12: [[crate::opus_types_h::opus_int16; 4]; 12] = [
    [
        189 as libc::c_int as crate::opus_types_h::opus_int16,
        -(600 as libc::c_int) as crate::opus_types_h::opus_int16,
        617 as libc::c_int as crate::opus_types_h::opus_int16,
        30567 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        117 as libc::c_int as crate::opus_types_h::opus_int16,
        -(159 as libc::c_int) as crate::opus_types_h::opus_int16,
        -(1070 as libc::c_int) as crate::opus_types_h::opus_int16,
        29704 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        52 as libc::c_int as crate::opus_types_h::opus_int16,
        221 as libc::c_int as crate::opus_types_h::opus_int16,
        -(2392 as libc::c_int) as crate::opus_types_h::opus_int16,
        28276 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(4 as libc::c_int) as crate::opus_types_h::opus_int16,
        529 as libc::c_int as crate::opus_types_h::opus_int16,
        -(3350 as libc::c_int) as crate::opus_types_h::opus_int16,
        26341 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(48 as libc::c_int) as crate::opus_types_h::opus_int16,
        758 as libc::c_int as crate::opus_types_h::opus_int16,
        -(3956 as libc::c_int) as crate::opus_types_h::opus_int16,
        23973 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(80 as libc::c_int) as crate::opus_types_h::opus_int16,
        905 as libc::c_int as crate::opus_types_h::opus_int16,
        -(4235 as libc::c_int) as crate::opus_types_h::opus_int16,
        21254 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(99 as libc::c_int) as crate::opus_types_h::opus_int16,
        972 as libc::c_int as crate::opus_types_h::opus_int16,
        -(4222 as libc::c_int) as crate::opus_types_h::opus_int16,
        18278 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(107 as libc::c_int) as crate::opus_types_h::opus_int16,
        967 as libc::c_int as crate::opus_types_h::opus_int16,
        -(3957 as libc::c_int) as crate::opus_types_h::opus_int16,
        15143 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(103 as libc::c_int) as crate::opus_types_h::opus_int16,
        896 as libc::c_int as crate::opus_types_h::opus_int16,
        -(3487 as libc::c_int) as crate::opus_types_h::opus_int16,
        11950 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(91 as libc::c_int) as crate::opus_types_h::opus_int16,
        773 as libc::c_int as crate::opus_types_h::opus_int16,
        -(2865 as libc::c_int) as crate::opus_types_h::opus_int16,
        8798 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(71 as libc::c_int) as crate::opus_types_h::opus_int16,
        611 as libc::c_int as crate::opus_types_h::opus_int16,
        -(2143 as libc::c_int) as crate::opus_types_h::opus_int16,
        5784 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
    [
        -(46 as libc::c_int) as crate::opus_types_h::opus_int16,
        425 as libc::c_int as crate::opus_types_h::opus_int16,
        -(1375 as libc::c_int) as crate::opus_types_h::opus_int16,
        2996 as libc::c_int as crate::opus_types_h::opus_int16,
    ],
];
