use ::libc;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;

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
#[inline]

unsafe extern "C" fn silk_resampler_private_down_FIR_INTERPOL(
    mut out: *mut crate::opus_types_h::opus_int16,
    mut buf: *mut crate::opus_types_h::opus_int32,
    mut FIR_Coefs: *const crate::opus_types_h::opus_int16,
    mut FIR_Order: i32,
    mut FIR_Fracs: i32,
    mut max_index_Q16: crate::opus_types_h::opus_int32,
    mut index_increment_Q16: crate::opus_types_h::opus_int32,
) -> *mut crate::opus_types_h::opus_int16 {
    let mut index_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut res_Q6: crate::opus_types_h::opus_int32 = 0;
    let mut buf_ptr: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut interpol_ind: crate::opus_types_h::opus_int32 = 0;
    let mut interpol_ptr: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    match FIR_Order {
        18 => {
            index_Q16 = 0;
            while index_Q16 < max_index_Q16 {
                /* Integer part gives pointer to buffered input */
                buf_ptr = buf.offset((index_Q16 >> 16) as isize);
                /* Fractional part gives interpolation coefficients */
                interpol_ind = ((index_Q16 & 0xffff) as i64
                    * FIR_Fracs as crate::opus_types_h::opus_int16 as i64
                    >> 16) as crate::opus_types_h::opus_int32;
                /* Inner product */
                interpol_ptr = &*FIR_Coefs.offset((18 / 2 * interpol_ind) as isize)
                    as *const crate::opus_types_h::opus_int16;
                res_Q6 = (*buf_ptr.offset(0) as i64 * *interpol_ptr.offset(0) as i64 >> 16)
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(1) as i64 * *interpol_ptr.offset(1) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(2) as i64 * *interpol_ptr.offset(2) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(3) as i64 * *interpol_ptr.offset(3) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(4) as i64 * *interpol_ptr.offset(4) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(5) as i64 * *interpol_ptr.offset(5) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(6) as i64 * *interpol_ptr.offset(6) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(7) as i64 * *interpol_ptr.offset(7) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(8) as i64 * *interpol_ptr.offset(8) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                interpol_ptr = &*FIR_Coefs
                    .offset((18 / 2 * (FIR_Fracs - 1 - interpol_ind)) as isize)
                    as *const crate::opus_types_h::opus_int16;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(17) as i64 * *interpol_ptr.offset(0) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(16) as i64 * *interpol_ptr.offset(1) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(15) as i64 * *interpol_ptr.offset(2) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(14) as i64 * *interpol_ptr.offset(3) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(13) as i64 * *interpol_ptr.offset(4) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(12) as i64 * *interpol_ptr.offset(5) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(11) as i64 * *interpol_ptr.offset(6) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(10) as i64 * *interpol_ptr.offset(7) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(9) as i64 * *interpol_ptr.offset(8) as i64 >> 16))
                    as crate::opus_types_h::opus_int32;
                /* Scale down, saturate and store in output array */
                let fresh0 = out;
                out = out.offset(1);
                *fresh0 = if (if 6 == 1 {
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
                index_Q16 += index_increment_Q16
            }
        }
        24 => {
            index_Q16 = 0;
            while index_Q16 < max_index_Q16 {
                /* Integer part gives pointer to buffered input */
                buf_ptr = buf.offset((index_Q16 >> 16) as isize);
                /* Inner product */
                res_Q6 = ((*buf_ptr.offset(0) + *buf_ptr.offset(23)) as i64
                    * *FIR_Coefs.offset(0) as i64
                    >> 16) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(1) + *buf_ptr.offset(22)) as i64
                        * *FIR_Coefs.offset(1) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(2) + *buf_ptr.offset(21)) as i64
                        * *FIR_Coefs.offset(2) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(3) + *buf_ptr.offset(20)) as i64
                        * *FIR_Coefs.offset(3) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(4) + *buf_ptr.offset(19)) as i64
                        * *FIR_Coefs.offset(4) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(5) + *buf_ptr.offset(18)) as i64
                        * *FIR_Coefs.offset(5) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(6) + *buf_ptr.offset(17)) as i64
                        * *FIR_Coefs.offset(6) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(7) + *buf_ptr.offset(16)) as i64
                        * *FIR_Coefs.offset(7) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(8) + *buf_ptr.offset(15)) as i64
                        * *FIR_Coefs.offset(8) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(9) + *buf_ptr.offset(14)) as i64
                        * *FIR_Coefs.offset(9) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(10) + *buf_ptr.offset(13)) as i64
                        * *FIR_Coefs.offset(10) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(11) + *buf_ptr.offset(12)) as i64
                        * *FIR_Coefs.offset(11) as i64
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
                index_Q16 += index_increment_Q16
            }
        }
        36 => {
            index_Q16 = 0;
            while index_Q16 < max_index_Q16 {
                /* Integer part gives pointer to buffered input */
                buf_ptr = buf.offset((index_Q16 >> 16) as isize);
                /* Inner product */
                res_Q6 = ((*buf_ptr.offset(0) + *buf_ptr.offset(35)) as i64
                    * *FIR_Coefs.offset(0) as i64
                    >> 16) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(1) + *buf_ptr.offset(34)) as i64
                        * *FIR_Coefs.offset(1) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(2) + *buf_ptr.offset(33)) as i64
                        * *FIR_Coefs.offset(2) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(3) + *buf_ptr.offset(32)) as i64
                        * *FIR_Coefs.offset(3) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(4) + *buf_ptr.offset(31)) as i64
                        * *FIR_Coefs.offset(4) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(5) + *buf_ptr.offset(30)) as i64
                        * *FIR_Coefs.offset(5) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(6) + *buf_ptr.offset(29)) as i64
                        * *FIR_Coefs.offset(6) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(7) + *buf_ptr.offset(28)) as i64
                        * *FIR_Coefs.offset(7) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(8) + *buf_ptr.offset(27)) as i64
                        * *FIR_Coefs.offset(8) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(9) + *buf_ptr.offset(26)) as i64
                        * *FIR_Coefs.offset(9) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(10) + *buf_ptr.offset(25)) as i64
                        * *FIR_Coefs.offset(10) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(11) + *buf_ptr.offset(24)) as i64
                        * *FIR_Coefs.offset(11) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(12) + *buf_ptr.offset(23)) as i64
                        * *FIR_Coefs.offset(12) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(13) + *buf_ptr.offset(22)) as i64
                        * *FIR_Coefs.offset(13) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(14) + *buf_ptr.offset(21)) as i64
                        * *FIR_Coefs.offset(14) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(15) + *buf_ptr.offset(20)) as i64
                        * *FIR_Coefs.offset(15) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(16) + *buf_ptr.offset(19)) as i64
                        * *FIR_Coefs.offset(16) as i64
                        >> 16)) as crate::opus_types_h::opus_int32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(17) + *buf_ptr.offset(18)) as i64
                        * *FIR_Coefs.offset(17) as i64
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
                index_Q16 += index_increment_Q16
            }
        }
        _ => {}
    }
    return out;
}
/* Description: Hybrid IIR/FIR polyphase implementation of resampling */
/* Resample with a 2nd order AR filter followed by FIR interpolation */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_private_down_FIR(
    mut SS: *mut libc::c_void,
    mut out: *mut crate::opus_types_h::opus_int16,
    mut in_0: *const crate::opus_types_h::opus_int16,
    mut inLen: crate::opus_types_h::opus_int32,
)
/* I    Number of input samples     */
{
    let mut S: *mut crate::resampler_structs_h::silk_resampler_state_struct =
        SS as *mut crate::resampler_structs_h::silk_resampler_state_struct;
    let mut nSamplesIn: crate::opus_types_h::opus_int32 = 0;
    let mut max_index_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut index_increment_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut buf: *mut crate::opus_types_h::opus_int32 = 0 as *mut crate::opus_types_h::opus_int32;
    let mut FIR_Coefs: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>())
            .wrapping_mul(((*S).batchSize + (*S).FIR_Order) as usize),
    );
    buf = fresh3.as_mut_ptr();
    /* Copy buffered samples to start of buffer */
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        (*S).sFIR.i32_0.as_mut_ptr() as *const libc::c_void,
        ((*S).FIR_Order as usize)
            .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
    FIR_Coefs = &*(*S).Coefs.offset(2) as *const crate::opus_types_h::opus_int16;
    /* Iterate over blocks of frameSizeIn input samples */
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize {
            inLen
        } else {
            (*S).batchSize
        };
        /* Second-order AR filter (output in Q8) */
        crate::src::opus_1_2_1::silk::resampler_private_AR2::silk_resampler_private_AR2(
            (*S).sIIR.as_mut_ptr(),
            &mut *buf.offset((*S).FIR_Order as isize),
            in_0,
            (*S).Coefs,
            nSamplesIn,
        );
        max_index_Q16 = ((nSamplesIn as crate::opus_types_h::opus_uint32) << 16)
            as crate::opus_types_h::opus_int32;
        /* Interpolate filtered signal */
        out = silk_resampler_private_down_FIR_INTERPOL(
            out,
            buf,
            FIR_Coefs,
            (*S).FIR_Order,
            (*S).FIR_Fracs,
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 1) {
            break;
        }
        /* More iterations to do; copy last part of filtered signal to beginning of buffer */
        crate::stdlib::memcpy(
            buf as *mut libc::c_void,
            &mut *buf.offset(nSamplesIn as isize) as *mut crate::opus_types_h::opus_int32
                as *const libc::c_void,
            ((*S).FIR_Order as usize)
                .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
        );
    }
    /* Copy last part of filtered signal to the state for the next call */
    crate::stdlib::memcpy(
        (*S).sFIR.i32_0.as_mut_ptr() as *mut libc::c_void,
        &mut *buf.offset(nSamplesIn as isize) as *mut crate::opus_types_h::opus_int32
            as *const libc::c_void,
        ((*S).FIR_Order as usize)
            .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()),
    );
}
