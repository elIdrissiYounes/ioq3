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
/* Cosine approximation table for LSF conversion */
/* Q12 values (even) */
#[no_mangle]

pub static mut silk_LSFCosTab_FIX_Q12: [crate::opus_types_h::opus_int16; 129] = [
    8192 as libc::c_int as crate::opus_types_h::opus_int16,
    8190 as libc::c_int as crate::opus_types_h::opus_int16,
    8182 as libc::c_int as crate::opus_types_h::opus_int16,
    8170 as libc::c_int as crate::opus_types_h::opus_int16,
    8152 as libc::c_int as crate::opus_types_h::opus_int16,
    8130 as libc::c_int as crate::opus_types_h::opus_int16,
    8104 as libc::c_int as crate::opus_types_h::opus_int16,
    8072 as libc::c_int as crate::opus_types_h::opus_int16,
    8034 as libc::c_int as crate::opus_types_h::opus_int16,
    7994 as libc::c_int as crate::opus_types_h::opus_int16,
    7946 as libc::c_int as crate::opus_types_h::opus_int16,
    7896 as libc::c_int as crate::opus_types_h::opus_int16,
    7840 as libc::c_int as crate::opus_types_h::opus_int16,
    7778 as libc::c_int as crate::opus_types_h::opus_int16,
    7714 as libc::c_int as crate::opus_types_h::opus_int16,
    7644 as libc::c_int as crate::opus_types_h::opus_int16,
    7568 as libc::c_int as crate::opus_types_h::opus_int16,
    7490 as libc::c_int as crate::opus_types_h::opus_int16,
    7406 as libc::c_int as crate::opus_types_h::opus_int16,
    7318 as libc::c_int as crate::opus_types_h::opus_int16,
    7226 as libc::c_int as crate::opus_types_h::opus_int16,
    7128 as libc::c_int as crate::opus_types_h::opus_int16,
    7026 as libc::c_int as crate::opus_types_h::opus_int16,
    6922 as libc::c_int as crate::opus_types_h::opus_int16,
    6812 as libc::c_int as crate::opus_types_h::opus_int16,
    6698 as libc::c_int as crate::opus_types_h::opus_int16,
    6580 as libc::c_int as crate::opus_types_h::opus_int16,
    6458 as libc::c_int as crate::opus_types_h::opus_int16,
    6332 as libc::c_int as crate::opus_types_h::opus_int16,
    6204 as libc::c_int as crate::opus_types_h::opus_int16,
    6070 as libc::c_int as crate::opus_types_h::opus_int16,
    5934 as libc::c_int as crate::opus_types_h::opus_int16,
    5792 as libc::c_int as crate::opus_types_h::opus_int16,
    5648 as libc::c_int as crate::opus_types_h::opus_int16,
    5502 as libc::c_int as crate::opus_types_h::opus_int16,
    5352 as libc::c_int as crate::opus_types_h::opus_int16,
    5198 as libc::c_int as crate::opus_types_h::opus_int16,
    5040 as libc::c_int as crate::opus_types_h::opus_int16,
    4880 as libc::c_int as crate::opus_types_h::opus_int16,
    4718 as libc::c_int as crate::opus_types_h::opus_int16,
    4552 as libc::c_int as crate::opus_types_h::opus_int16,
    4382 as libc::c_int as crate::opus_types_h::opus_int16,
    4212 as libc::c_int as crate::opus_types_h::opus_int16,
    4038 as libc::c_int as crate::opus_types_h::opus_int16,
    3862 as libc::c_int as crate::opus_types_h::opus_int16,
    3684 as libc::c_int as crate::opus_types_h::opus_int16,
    3502 as libc::c_int as crate::opus_types_h::opus_int16,
    3320 as libc::c_int as crate::opus_types_h::opus_int16,
    3136 as libc::c_int as crate::opus_types_h::opus_int16,
    2948 as libc::c_int as crate::opus_types_h::opus_int16,
    2760 as libc::c_int as crate::opus_types_h::opus_int16,
    2570 as libc::c_int as crate::opus_types_h::opus_int16,
    2378 as libc::c_int as crate::opus_types_h::opus_int16,
    2186 as libc::c_int as crate::opus_types_h::opus_int16,
    1990 as libc::c_int as crate::opus_types_h::opus_int16,
    1794 as libc::c_int as crate::opus_types_h::opus_int16,
    1598 as libc::c_int as crate::opus_types_h::opus_int16,
    1400 as libc::c_int as crate::opus_types_h::opus_int16,
    1202 as libc::c_int as crate::opus_types_h::opus_int16,
    1002 as libc::c_int as crate::opus_types_h::opus_int16,
    802 as libc::c_int as crate::opus_types_h::opus_int16,
    602 as libc::c_int as crate::opus_types_h::opus_int16,
    402 as libc::c_int as crate::opus_types_h::opus_int16,
    202 as libc::c_int as crate::opus_types_h::opus_int16,
    0 as libc::c_int as crate::opus_types_h::opus_int16,
    -(202 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(402 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(602 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(802 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(1002 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(1202 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(1400 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(1598 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(1794 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(1990 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(2186 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(2378 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(2570 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(2760 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(2948 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(3136 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(3320 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(3502 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(3684 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(3862 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4038 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4212 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4382 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4552 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4718 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(4880 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5040 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5198 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5352 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5502 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5648 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5792 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(5934 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6070 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6204 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6332 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6458 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6580 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6698 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6812 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(6922 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7026 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7128 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7226 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7318 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7406 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7490 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7568 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7644 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7714 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7778 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7840 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7896 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7946 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(7994 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8034 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8072 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8104 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8130 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8152 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8170 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8182 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8190 as libc::c_int) as crate::opus_types_h::opus_int16,
    -(8192 as libc::c_int) as crate::opus_types_h::opus_int16,
];
