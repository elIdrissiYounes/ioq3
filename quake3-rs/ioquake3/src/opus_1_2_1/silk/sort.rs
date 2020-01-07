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
/* Insertion sort (fast for already almost sorted arrays):   */
/* Best case:  O(n)   for an already sorted array            */
/* Worst case: O(n^2) for an inversely sorted array          */
/*                                                           */
/* Shell short:    https://en.wikipedia.org/wiki/Shell_sort  */
#[no_mangle]

pub unsafe extern "C" fn silk_insertion_sort_increasing(
    mut a: *mut crate::opus_types_h::opus_int32,
    mut idx: *mut libc::c_int,
    L: libc::c_int,
    K: libc::c_int,
)
/* I     Number of correctly sorted positions   */
{
    let mut value: crate::opus_types_h::opus_int32 = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Safety checks */
    /* Write start indices in index vector */
    i = 0 as libc::c_int;
    while i < K {
        *idx.offset(i as isize) = i;
        i += 1
    }
    /* Sort vector elements by value, increasing order */
    i = 1 as libc::c_int;
    while i < K {
        value = *a.offset(i as isize);
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
            /* Write index */
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            *idx.offset((j + 1 as libc::c_int) as isize) = *idx.offset(j as isize);
            j -= 1 /* Shift value */
            /* Shift index */
        } /* Write value */
        *a.offset((j + 1 as libc::c_int) as isize) = value;
        *idx.offset((j + 1 as libc::c_int) as isize) = i;
        i += 1
    }
    /* If less than L values are asked for, check the remaining values, */
    /* but only spend CPU to ensure that the K first values are correct */
    i = K; /* Shift value */
    while i < L {
        value = *a.offset(i as isize);
        if value < *a.offset((K - 1 as libc::c_int) as isize) {
            j = K - 2 as libc::c_int;
            while j >= 0 as libc::c_int && value < *a.offset(j as isize) {
                *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
                *idx.offset((j + 1 as libc::c_int) as isize) = *idx.offset(j as isize);
                j -= 1
                /* Shift index */
            }
            /* Write index */
            *a.offset((j + 1 as libc::c_int) as isize) = value; /* Write value */
            *idx.offset((j + 1 as libc::c_int) as isize) = i
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn silk_insertion_sort_increasing_all_values_int16(
    mut a: *mut crate::opus_types_h::opus_int16,
    L: libc::c_int,
)
/* I     Vector length                                              */
{
    let mut value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* Safety checks */
    /* Sort vector elements by value, increasing order */
    i = 1 as libc::c_int;
    while i < L {
        value = *a.offset(i as isize) as libc::c_int;
        j = i - 1 as libc::c_int;
        while j >= 0 as libc::c_int && value < *a.offset(j as isize) as libc::c_int {
            *a.offset((j + 1 as libc::c_int) as isize) = *a.offset(j as isize);
            j -= 1
            /* Write value */
            /* Shift value */
        }
        *a.offset((j + 1 as libc::c_int) as isize) = value as crate::opus_types_h::opus_int16;
        i += 1
    }
}
