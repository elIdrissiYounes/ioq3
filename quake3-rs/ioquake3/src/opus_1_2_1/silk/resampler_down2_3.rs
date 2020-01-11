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
/* Downsample by a factor 2/3, low quality */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_down2_3(
    mut S: *mut crate::opus_types_h::opus_int32,
    mut out: *mut crate::opus_types_h::opus_int16,
    mut in_0: *const crate::opus_types_h::opus_int16,
    mut inLen: crate::opus_types_h::opus_int32,
)
/* I    Number of input samples                                     */
{
    let mut nSamplesIn: crate::opus_types_h::opus_int32 = 0;
    let mut counter: crate::opus_types_h::opus_int32 = 0;
    let mut res_Q6: crate::opus_types_h::opus_int32 = 0;
    let mut buf: *mut crate::opus_types_h::opus_int32 = 0 as *mut crate::opus_types_h::opus_int32;
    let mut buf_ptr: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>())
            .wrapping_mul((10i32 * 48 + 4) as usize),
    );
    buf = fresh0.as_mut_ptr();
    /* Copy buffered samples to start of buffer */
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        S as *const libc::c_void,
        (4usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
    loop
    /* Iterate over blocks of frameSizeIn input samples */
    {
        nSamplesIn = if inLen < 10 * 48 { inLen } else { (10) * 48 };
        /* Second-order AR filter (output in Q8) */
        crate::src::opus_1_2_1::silk::resampler_private_AR2::silk_resampler_private_AR2(
            &mut *S.offset(4),
            &mut *buf.offset(4),
            in_0,
            crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ.as_ptr(),
            nSamplesIn,
        );
        /* Interpolate filtered signal */
        buf_ptr = buf;
        counter = nSamplesIn;
        while counter > 2 {
            /* Inner product */
            res_Q6 = (*buf_ptr.offset(0) as i64
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[2]
                    as i64
                >> 16) as crate::opus_types_h::opus_int32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(1) as i64
                    * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[3]
                        as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(2) as i64
                    * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[5]
                        as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(3) as i64
                    * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[4]
                        as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            /* Scale down, saturate and store in output array */
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = if (if 6 == 1 {
                (res_Q6 >> 1) + (res_Q6 & 1)
            } else {
                ((res_Q6 >> 6 - 1) + 1) >> 1
            }) > 0x7fff
            {
                0x7fff
            } else if (if 6 == 1 {
                (res_Q6 >> 1) + (res_Q6 & 1)
            } else {
                ((res_Q6 >> 6 - 1) + 1) >> 1
            }) < 0x8000i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000i32 as crate::opus_types_h::opus_int16 as i32
            } else if 6 == 1 {
                (res_Q6 >> 1) + (res_Q6 & 1)
            } else {
                ((res_Q6 >> 6 - 1) + 1) >> 1
            } as crate::opus_types_h::opus_int16;
            res_Q6 = (*buf_ptr.offset(1) as i64
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[4]
                    as i64
                >> 16) as crate::opus_types_h::opus_int32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(2) as i64
                    * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[5]
                        as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(3) as i64
                    * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[3]
                        as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(4) as i64
                    * crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ[2]
                        as i64
                    >> 16)) as crate::opus_types_h::opus_int32;
            /* Scale down, saturate and store in output array */
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = if (if 6 == 1 {
                (res_Q6 >> 1) + (res_Q6 & 1)
            } else {
                ((res_Q6 >> 6 - 1) + 1) >> 1
            }) > 0x7fff
            {
                0x7fff
            } else if (if 6 == 1 {
                (res_Q6 >> 1) + (res_Q6 & 1)
            } else {
                ((res_Q6 >> 6 - 1) + 1) >> 1
            }) < 0x8000i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000i32 as crate::opus_types_h::opus_int16 as i32
            } else if 6 == 1 {
                (res_Q6 >> 1) + (res_Q6 & 1)
            } else {
                ((res_Q6 >> 6 - 1) + 1) >> 1
            } as crate::opus_types_h::opus_int16;
            buf_ptr = buf_ptr.offset(3);
            counter -= 3
        }
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0) {
            break;
        }
        /* More iterations to do; copy last part of filtered signal to beginning of buffer */
        crate::stdlib::memcpy(
            buf as *mut libc::c_void,
            &mut *buf.offset(nSamplesIn as isize) as *mut crate::opus_types_h::opus_int32
                as *const libc::c_void,
            (4usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
        );
    }
    /* Copy last part of filtered signal to the state for the next call */
    crate::stdlib::memcpy(
        S as *mut libc::c_void,
        &mut *buf.offset(nSamplesIn as isize) as *mut crate::opus_types_h::opus_int32
            as *const libc::c_void,
        (4usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
}
