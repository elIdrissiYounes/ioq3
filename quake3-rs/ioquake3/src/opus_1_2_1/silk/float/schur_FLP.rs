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
/* *******************************************************************/
/*                    SIGNAL PROCESSING FUNCTIONS                   */
/* *******************************************************************/
/* Chirp (bw expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    length of ar                                                */
/* I    chirp factor (typically in range (0..1) )                   */
/* compute inverse of LPC prediction gain, and                          */
/* test if LPC coefficients are stable (all poles within unit circle)   */
/* this code is based on silk_FLP_a2k()                                 */
/* O    return inverse prediction gain, energy domain               */
/* I    prediction coefficients [order]                             */
/* I    prediction order                                            */
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

pub unsafe extern "C" fn silk_schur_FLP(
    mut refl_coef: *mut libc::c_float,
    mut auto_corr: *const libc::c_float,
    mut order: libc::c_int,
) -> libc::c_float
/* I    order                                                       */ {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut C: [[libc::c_double; 2]; 25] = [[0.; 2]; 25];
    let mut Ctmp1: libc::c_double = 0.;
    let mut Ctmp2: libc::c_double = 0.;
    let mut rc_tmp: libc::c_double = 0.;
    /* Copy correlations */
    k = 0 as libc::c_int;
    loop {
        C[k as usize][1 as libc::c_int as usize] = *auto_corr.offset(k as isize) as libc::c_double;
        C[k as usize][0 as libc::c_int as usize] = C[k as usize][1 as libc::c_int as usize];
        k += 1;
        if !(k <= order) {
            break;
        }
    }
    k = 0 as libc::c_int;
    while k < order {
        /* Get reflection coefficient */
        rc_tmp = -C[(k + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
            / (if C[0 as libc::c_int as usize][1 as libc::c_int as usize]
                > 1e-9f32 as libc::c_double
            {
                C[0 as libc::c_int as usize][1 as libc::c_int as usize]
            } else {
                1e-9f32 as libc::c_double
            });
        /* Save the output */
        *refl_coef.offset(k as isize) = rc_tmp as libc::c_float;
        /* Update correlations */
        n = 0 as libc::c_int;
        while n < order - k {
            Ctmp1 = C[(n + k + 1 as libc::c_int) as usize][0 as libc::c_int as usize];
            Ctmp2 = C[n as usize][1 as libc::c_int as usize];
            C[(n + k + 1 as libc::c_int) as usize][0 as libc::c_int as usize] =
                Ctmp1 + Ctmp2 * rc_tmp;
            C[n as usize][1 as libc::c_int as usize] = Ctmp2 + Ctmp1 * rc_tmp;
            n += 1
        }
        k += 1
    }
    /* Return residual energy */
    return C[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_float;
}
