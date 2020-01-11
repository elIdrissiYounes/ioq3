use ::libc;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::stdlib::uint32_t;
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
/*#define silk_MACRO_COUNT */
/* Used to enable WMOPS counting */
/* max order of the LPC analysis in schur() and k2a() */
/* for memset(), memcpy(), memmove() */
/* *******************************************************************/
/*                    SIGNAL PROCESSING FUNCTIONS                   */
/* *******************************************************************/
/* !
 * Initialize/reset the resampler state for a given pair of input/output sampling rates
*/
/* I/O  Resampler state                                             */
/* I    Input sampling rate (Hz)                                    */
/* I    Output sampling rate (Hz)                                   */
/* I    If 1: encoder; if 0: decoder                                */
/* !
 * Resampler: convert from one sampling rate to another
 */
/* I/O  Resampler state                                             */
/* O    Output signal                                               */
/* I    Input signal                                                */
/* I    Number of input samples                                     */
/* !
* Downsample 2x, mediocre quality
*/
/* I/O  State vector [ 2 ]                                          */
/* O    Output signal [ len ]                                       */
/* I    Input signal [ floor(len/2) ]                               */
/* I    Number of input samples                                     */
/* !
 * Downsample by a factor 2/3, low quality
*/
/* I/O  State vector [ 6 ]                                          */
/* O    Output signal [ floor(2*inLen/3) ]                          */
/* I    Input signal [ inLen ]                                      */
/* I    Number of input samples                                     */
/* !
 * second order ARMA filter;
 * slower than biquad() but uses more precise coefficients
 * can handle (slowly) varying coefficients
 */
/* I     input signal                                               */
/* I     MA coefficients [3]                                        */
/* I     AR coefficients [2]                                        */
/* I/O   State vector [2]                                           */
/* O     output signal                                              */
/* I     signal length (must be even)                               */
/* I     input signal                                               */
/* I     MA coefficients [3]                                        */
/* I     AR coefficients [2]                                        */
/* I/O   State vector [4]                                           */
/* O     output signal                                              */
/* I     signal length (must be even)                               */
/* Variable order MA prediction error filter. */
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
/* ******************************************/
/* LPC analysis filter                     */
/* NB! State is kept internally and the    */
/* filter always starts with zero state    */
/* first d output samples are set to zero  */
/* ******************************************/
/* OPT: Using celt_fir() for this function should be faster, but it may cause
integer overflows in intermediate values (not final results), which the
current implementation silences by casting to unsigned. Enabling
this should be safe in pretty much all cases, even though it is not technically
C89-compliant. */
#[no_mangle]

pub unsafe extern "C" fn silk_LPC_analysis_filter(
    mut out: *mut crate::opus_types_h::opus_int16,
    mut in_0: *const crate::opus_types_h::opus_int16,
    mut B: *const crate::opus_types_h::opus_int16,
    len: crate::opus_types_h::opus_int32,
    d: crate::opus_types_h::opus_int32,
    mut _arch: i32,
)
/* I    Run-time architecture                                       */
{
    let mut _j: i32 = 0;
    let mut _ix: i32 = 0;
    let mut out32_Q12: crate::opus_types_h::opus_int32 = 0;
    let mut out32: crate::opus_types_h::opus_int32 = 0;
    let mut in_ptr: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;

    for ix in d..len {
        in_ptr = &*in_0.offset((ix - 1) as isize) as *const crate::opus_types_h::opus_int16;

        out32_Q12 = *in_ptr.offset(0) as crate::opus_types_h::opus_int32
            * *B.offset(0) as crate::opus_types_h::opus_int32;

        out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
            (*in_ptr.offset(-1) as crate::opus_types_h::opus_int32
                * *B.offset(1) as crate::opus_types_h::opus_int32)
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32;

        out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
            (*in_ptr.offset(-2) as crate::opus_types_h::opus_int32
                * *B.offset(2) as crate::opus_types_h::opus_int32)
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32;

        out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
            (*in_ptr.offset(-3) as crate::opus_types_h::opus_int32
                * *B.offset(3) as crate::opus_types_h::opus_int32)
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32;

        out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
            (*in_ptr.offset(-4) as crate::opus_types_h::opus_int32
                * *B.offset(4) as crate::opus_types_h::opus_int32)
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32;

        out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
            (*in_ptr.offset(-5) as crate::opus_types_h::opus_int32
                * *B.offset(5) as crate::opus_types_h::opus_int32)
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32;
        for j in (6..d).step_by(2 as usize) {
            out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
                (*in_ptr.offset(-j as isize) as crate::opus_types_h::opus_int32
                    * *B.offset(j as isize) as crate::opus_types_h::opus_int32)
                    as crate::opus_types_h::opus_uint32,
            ) as crate::opus_types_h::opus_int32;

            out32_Q12 = (out32_Q12 as crate::opus_types_h::opus_uint32).wrapping_add(
                (*in_ptr.offset((-j - 1) as isize) as crate::opus_types_h::opus_int32
                    * *B.offset((j + 1) as isize) as crate::opus_types_h::opus_int32)
                    as crate::opus_types_h::opus_uint32,
            ) as crate::opus_types_h::opus_int32;
        }

        out32_Q12 = ((*in_ptr.offset(1) as crate::opus_types_h::opus_uint32) << 12)
            .wrapping_sub(out32_Q12 as crate::opus_types_h::opus_uint32)
            as crate::opus_types_h::opus_int32;

        out32 = if 12 == 1 {
            (out32_Q12 >> 1) + (out32_Q12 & 1)
        } else {
            ((out32_Q12 >> 12 - 1) + 1) >> 1
        };

        *out.offset(ix as isize) = if out32 > 0x7fff {
            0x7fff
        } else if out32 < 0x8000i32 as crate::opus_types_h::opus_int16 as i32 {
            0x8000i32 as crate::opus_types_h::opus_int16 as i32
        } else {
            out32
        } as crate::opus_types_h::opus_int16;
    }
    /* Set first d output samples to zero */
    crate::stdlib::memset(
        out as *mut libc::c_void,
        0,
        (d as usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()),
    );
}
