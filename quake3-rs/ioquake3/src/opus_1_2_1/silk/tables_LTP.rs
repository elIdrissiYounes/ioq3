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

pub static mut silk_LTP_per_index_iCDF: [libc::c_uchar; 3] = [
    179 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_iCDF_0: [libc::c_uchar; 8] = [
    71 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_iCDF_1: [libc::c_uchar; 16] = [
    199 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_iCDF_2: [libc::c_uchar; 32] = [
    241 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_BITS_Q5_0: [libc::c_uchar; 8] = [
    15 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_BITS_Q5_1: [libc::c_uchar; 16] = [
    69 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_BITS_Q5_2: [libc::c_uchar; 32] = [
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
];
#[no_mangle]

pub static mut silk_LTP_gain_iCDF_ptrs: [*const libc::c_uchar; 3] = unsafe {
    [
        silk_LTP_gain_iCDF_0.as_ptr(),
        silk_LTP_gain_iCDF_1.as_ptr(),
        silk_LTP_gain_iCDF_2.as_ptr(),
    ]
};
#[no_mangle]

pub static mut silk_LTP_gain_BITS_Q5_ptrs: [*const libc::c_uchar; 3] = unsafe {
    [
        silk_LTP_gain_BITS_Q5_0.as_ptr(),
        silk_LTP_gain_BITS_Q5_1.as_ptr(),
        silk_LTP_gain_BITS_Q5_2.as_ptr(),
    ]
};

static mut silk_LTP_gain_vq_0: [[libc::c_schar; 5]; 8] = [
    [
        4 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ],
    [
        12 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
    ],
    [
        -(9 as libc::c_int) as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        25 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
    ],
    [
        1 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        62 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
    ],
    [
        -(10 as libc::c_int) as libc::c_schar,
        37 as libc::c_int as libc::c_schar,
        65 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
    ],
    [
        -(6 as libc::c_int) as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        66 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        -(8 as libc::c_int) as libc::c_schar,
    ],
    [
        16 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
    ],
];

static mut silk_LTP_gain_vq_1: [[libc::c_schar; 5]; 16] = [
    [
        13 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
    ],
    [
        -(1 as libc::c_int) as libc::c_schar,
        36 as libc::c_int as libc::c_schar,
        64 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        -(6 as libc::c_int) as libc::c_schar,
    ],
    [
        -(7 as libc::c_int) as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        43 as libc::c_int as libc::c_schar,
        17 as libc::c_int as libc::c_schar,
    ],
    [
        1 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
    ],
    [
        6 as libc::c_int as libc::c_schar,
        -(11 as libc::c_int) as libc::c_schar,
        74 as libc::c_int as libc::c_schar,
        53 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
    ],
    [
        -(12 as libc::c_int) as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        76 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
    ],
    [
        -(3 as libc::c_int) as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        93 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
    ],
    [
        26 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        59 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        -(8 as libc::c_int) as libc::c_schar,
    ],
    [
        2 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        77 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
    ],
    [
        -(8 as libc::c_int) as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        -(6 as libc::c_int) as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
    ],
    [
        40 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
    ],
    [
        -(7 as libc::c_int) as libc::c_schar,
        20 as libc::c_int as libc::c_schar,
        101 as libc::c_int as libc::c_schar,
        -(7 as libc::c_int) as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
    ],
    [
        3 as libc::c_int as libc::c_schar,
        -(8 as libc::c_int) as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ],
    [
        -(15 as libc::c_int) as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
        68 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
    ],
    [
        -(2 as libc::c_int) as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
    ],
    [
        3 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        21 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
    ],
];

static mut silk_LTP_gain_vq_2: [[libc::c_schar; 5]; 32] = [
    [
        -(6 as libc::c_int) as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        61 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
    ],
    [
        -(11 as libc::c_int) as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        88 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
    ],
    [
        -(2 as libc::c_int) as libc::c_schar,
        60 as libc::c_int as libc::c_schar,
        65 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
    ],
    [
        -(1 as libc::c_int) as libc::c_schar,
        -(5 as libc::c_int) as libc::c_schar,
        73 as libc::c_int as libc::c_schar,
        56 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
    ],
    [
        -(9 as libc::c_int) as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
        94 as libc::c_int as libc::c_schar,
        29 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        99 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
    ],
    [
        8 as libc::c_int as libc::c_schar,
        -(19 as libc::c_int) as libc::c_schar,
        102 as libc::c_int as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        -(13 as libc::c_int) as libc::c_schar,
    ],
    [
        3 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
    ],
    [
        9 as libc::c_int as libc::c_schar,
        -(21 as libc::c_int) as libc::c_schar,
        84 as libc::c_int as libc::c_schar,
        72 as libc::c_int as libc::c_schar,
        -(18 as libc::c_int) as libc::c_schar,
    ],
    [
        -(11 as libc::c_int) as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        104 as libc::c_int as libc::c_schar,
        -(22 as libc::c_int) as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
    ],
    [
        18 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        48 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ],
    [
        -(16 as libc::c_int) as libc::c_schar,
        70 as libc::c_int as libc::c_schar,
        83 as libc::c_int as libc::c_schar,
        -(21 as libc::c_int) as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
    ],
    [
        5 as libc::c_int as libc::c_schar,
        -(11 as libc::c_int) as libc::c_schar,
        117 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        -(8 as libc::c_int) as libc::c_schar,
    ],
    [
        -(6 as libc::c_int) as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        117 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
    ],
    [
        3 as libc::c_int as libc::c_schar,
        -(8 as libc::c_int) as libc::c_schar,
        95 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
    ],
    [
        -(10 as libc::c_int) as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        77 as libc::c_int as libc::c_schar,
        60 as libc::c_int as libc::c_schar,
        -(15 as libc::c_int) as libc::c_schar,
    ],
    [
        -(1 as libc::c_int) as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        124 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
    ],
    [
        3 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        84 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        -(25 as libc::c_int) as libc::c_schar,
    ],
    [
        2 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
    ],
    [
        21 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
        56 as libc::c_int as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        -(1 as libc::c_int) as libc::c_schar,
        35 as libc::c_int as libc::c_schar,
        79 as libc::c_int as libc::c_schar,
        -(13 as libc::c_int) as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
    ],
    [
        -(7 as libc::c_int) as libc::c_schar,
        65 as libc::c_int as libc::c_schar,
        88 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        -(14 as libc::c_int) as libc::c_schar,
    ],
    [
        20 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        81 as libc::c_int as libc::c_schar,
        49 as libc::c_int as libc::c_schar,
        -(29 as libc::c_int) as libc::c_schar,
    ],
    [
        20 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        75 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        -(17 as libc::c_int) as libc::c_schar,
    ],
    [
        5 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        92 as libc::c_int as libc::c_schar,
        -(8 as libc::c_int) as libc::c_schar,
    ],
    [
        1 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        69 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
    ],
    [
        -(6 as libc::c_int) as libc::c_schar,
        95 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
    ],
    [
        39 as libc::c_int as libc::c_schar,
        67 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(6 as libc::c_int) as libc::c_schar,
        120 as libc::c_int as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        -(36 as libc::c_int) as libc::c_schar,
    ],
    [
        -(13 as libc::c_int) as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        122 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        -(24 as libc::c_int) as libc::c_schar,
    ],
    [
        81 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
    ],
    [
        2 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        88 as libc::c_int as libc::c_schar,
    ],
];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut silk_LTP_vq_ptrs_Q7: [*const libc::c_schar; 3] = [0 as *const libc::c_schar; 3];
/* Maximum frequency-dependent response of the pitch taps above,
computed as max(abs(freqz(taps))) */

static mut silk_LTP_gain_vq_0_gain: [libc::c_uchar; 8] = [
    46 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_vq_1_gain: [libc::c_uchar; 16] = [
    109 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
];

static mut silk_LTP_gain_vq_2_gain: [libc::c_uchar; 32] = [
    126 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    86 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut silk_LTP_vq_gain_ptrs_Q7: [*const libc::c_uchar; 3] = [0 as *const libc::c_uchar; 3];
#[no_mangle]

pub static mut silk_LTP_vq_sizes: [libc::c_schar; 3] = [
    8 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
];
unsafe extern "C" fn run_static_initializers() {
    silk_LTP_vq_ptrs_Q7 = [
        &*(*silk_LTP_gain_vq_0
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const libc::c_schar as *mut libc::c_schar
            as *const libc::c_schar,
        &*(*silk_LTP_gain_vq_1
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const libc::c_schar as *mut libc::c_schar
            as *const libc::c_schar,
        &*(*silk_LTP_gain_vq_2
            .as_ptr()
            .offset(0 as libc::c_int as isize))
        .as_ptr()
        .offset(0 as libc::c_int as isize) as *const libc::c_schar as *mut libc::c_schar
            as *const libc::c_schar,
    ];
    silk_LTP_vq_gain_ptrs_Q7 = [
        &*silk_LTP_gain_vq_0_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const libc::c_uchar,
        &*silk_LTP_gain_vq_1_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const libc::c_uchar,
        &*silk_LTP_gain_vq_2_gain
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const libc::c_uchar,
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
