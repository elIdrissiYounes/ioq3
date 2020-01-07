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
/* Autocorrelations for a warped frequency axis */
#[no_mangle]

pub unsafe extern "C" fn silk_warped_autocorrelation_FLP(
    mut corr: *mut libc::c_float,
    mut input: *const libc::c_float,
    warping: libc::c_float,
    length: libc::c_int,
    order: libc::c_int,
)
/* I    Correlation order (even)                    */
{
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut state: [libc::c_double; 25] = [
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut C: [libc::c_double; 25] = [
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    /* Order must be even */
    /* Loop over samples */
    n = 0 as libc::c_int;
    while n < length {
        tmp1 = *input.offset(n as isize) as libc::c_double;
        /* Loop over allpass sections */
        i = 0 as libc::c_int;
        while i < order {
            /* Output of allpass section */
            tmp2 = state[i as usize]
                + warping as libc::c_double * (state[(i + 1 as libc::c_int) as usize] - tmp1);
            state[i as usize] = tmp1;
            C[i as usize] += state[0 as libc::c_int as usize] * tmp1;
            /* Output of allpass section */
            tmp1 = state[(i + 1 as libc::c_int) as usize]
                + warping as libc::c_double * (state[(i + 2 as libc::c_int) as usize] - tmp2);
            state[(i + 1 as libc::c_int) as usize] = tmp2;
            C[(i + 1 as libc::c_int) as usize] += state[0 as libc::c_int as usize] * tmp2;
            i += 2 as libc::c_int
        }
        state[order as usize] = tmp1;
        C[order as usize] += state[0 as libc::c_int as usize] * tmp1;
        n += 1
    }
    /* Copy correlations in silk_float output format */
    i = 0 as libc::c_int;
    while i < order + 1 as libc::c_int {
        *corr.offset(i as isize) = C[i as usize] as libc::c_float;
        i += 1
    }
}
