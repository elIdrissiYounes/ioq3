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

pub static mut silk_gain_iCDF: [[libc::c_uchar; 8]; 3] = [
    [
        224 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        44 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        254 as libc::c_int as libc::c_uchar,
        237 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
        132 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        255 as libc::c_int as libc::c_uchar,
        252 as libc::c_int as libc::c_uchar,
        226 as libc::c_int as libc::c_uchar,
        155 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
];
#[no_mangle]

pub static mut silk_delta_gain_iCDF: [libc::c_uchar; 41] = [
    250 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
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
