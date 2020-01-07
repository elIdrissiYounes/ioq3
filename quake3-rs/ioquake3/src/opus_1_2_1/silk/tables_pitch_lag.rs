use ::libc;
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
#[no_mangle]

pub static mut silk_pitch_lag_iCDF: [libc::c_uchar; 32] = [
    253 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_pitch_delta_iCDF: [libc::c_uchar; 21] = [
    210 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_pitch_contour_iCDF: [libc::c_uchar; 34] = [
    223 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    79 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_pitch_contour_NB_iCDF: [libc::c_uchar; 11] = [
    188 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_pitch_contour_10_ms_iCDF: [libc::c_uchar; 12] = [
    165 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_pitch_contour_10_ms_NB_iCDF: [libc::c_uchar; 3] = [
    113 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
