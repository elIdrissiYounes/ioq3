// =============== BEGIN mdct_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdct_lookup {
    pub n: i32,
    pub log2n: i32,
    pub trig: *mut f32,
    pub bitrev: *mut i32,
    pub scale: f32,
}
use ::libc;

use crate::stdlib::cos;
use crate::stdlib::free;
use crate::stdlib::log;
use crate::stdlib::malloc;
use crate::stdlib::memset;
use crate::stdlib::rint;
use crate::stdlib::sin;
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: normalized modified discrete cosine transform
          power of two length transform only [64 <= n ]

Original algorithm adapted long ago from _The use of multirate filter
banks for coding of high quality digital audio_, by T. Sporer,
K. Brandenburg and B. Edler, collection of the European Signal
Processing Conference (EUSIPCO), Amsterdam, June 1992, Vol.1, pp
211-214

The below code implements an algorithm that no longer looks much like
that presented in the paper, but the basic structure remains if you
dig deep enough to see it.

This module DOES NOT INCLUDE code to generate/apply the window
function.  Everybody has their own weird favorite including me... I
happen to like the properties of y=sin(.5PI*sin^2(x)), but others may
vehemently disagree.

********************************************************************/
/* this can also be run as an integer transform by uncommenting a
define in mdct.h; the integerization is a first pass and although
it's likely stable for Vorbis, the dynamic range is constrained and
roundoff isn't done (so it's noisy).  Consider it functional, but
only a starting point.  There's no point on a machine with an FPU */
/* build lookups for trig functions; also pre-figure scaling and
some window function algebra. */
#[no_mangle]

pub unsafe extern "C" fn mdct_init(
    mut lookup: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut n: i32,
) {
    let mut bitrev: *mut i32 =
        crate::stdlib::malloc((::std::mem::size_of::<i32>()).wrapping_mul((n / 4i32) as usize))
            as *mut i32;
    let mut T: *mut f32 =
        crate::stdlib::malloc((::std::mem::size_of::<f32>()).wrapping_mul((n + n / 4i32) as usize))
            as *mut f32;
    let mut i: i32 = 0;
    let mut n2: i32 = n >> 1;
    (*lookup).log2n =
        crate::stdlib::rint(crate::stdlib::log(n as f32 as f64) / crate::stdlib::log(2f64)) as i32;
    let mut log2n: i32 = (*lookup).log2n;
    (*lookup).n = n;
    (*lookup).trig = T;
    (*lookup).bitrev = bitrev;
    /* trig lookups... */
    i = 0;
    while i < n / 4 {
        *T.offset((i * 2) as isize) =
            crate::stdlib::cos(3.14159265358979323846 / n as f64 * (4 * i) as f64) as f32;
        *T.offset((i * 2 + 1) as isize) =
            -crate::stdlib::sin(3.14159265358979323846 / n as f64 * (4 * i) as f64) as f32;
        *T.offset((n2 + i * 2) as isize) =
            crate::stdlib::cos(3.14159265358979323846 / (2i32 * n) as f64 * (2 * i + 1) as f64)
                as f32;
        *T.offset((n2 + i * 2 + 1) as isize) =
            crate::stdlib::sin(3.14159265358979323846 / (2i32 * n) as f64 * (2 * i + 1) as f64)
                as f32;
        i += 1
    }
    i = 0;
    while i < n / 8 {
        *T.offset((n + i * 2) as isize) =
            (crate::stdlib::cos(3.14159265358979323846 / n as f64 * (4 * i + 2) as f64) * 0.5)
                as f32;
        *T.offset((n + i * 2 + 1) as isize) =
            (-crate::stdlib::sin(3.14159265358979323846 / n as f64 * (4 * i + 2) as f64) * 0.5)
                as f32;
        i += 1
    }
    /* bitreverse lookup... */
    let mut mask: i32 = ((1) << log2n - 1) - 1;
    let mut i_0: i32 = 0;
    let mut j: i32 = 0;
    let mut msb: i32 = (1) << log2n - 2;
    i_0 = 0;
    while i_0 < n / 8 {
        let mut acc: i32 = 0;
        j = 0;
        while msb >> j != 0 {
            if msb >> j & i_0 != 0 {
                acc |= (1) << j
            }
            j += 1
        }
        *bitrev.offset((i_0 * 2) as isize) = (!acc & mask) - 1;
        *bitrev.offset((i_0 * 2 + 1) as isize) = acc;
        i_0 += 1
    }
    (*lookup).scale = 4.0 / n as f32;
}
/* 8 point butterfly (in place, 4 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_8(mut x: *mut f32) {
    let mut r0: f32 = *x.offset(6) + *x.offset(2);
    let mut r1: f32 = *x.offset(6) - *x.offset(2);
    let mut r2: f32 = *x.offset(4) + *x.offset(0);
    let mut r3: f32 = *x.offset(4) - *x.offset(0);
    *x.offset(6) = r0 + r2;
    *x.offset(4) = r0 - r2;
    r0 = *x.offset(5) - *x.offset(1);
    r2 = *x.offset(7) - *x.offset(3);
    *x.offset(0) = r1 + r0;
    *x.offset(2) = r1 - r0;
    r0 = *x.offset(5) + *x.offset(1);
    r1 = *x.offset(7) + *x.offset(3);
    *x.offset(3) = r2 + r3;
    *x.offset(1) = r2 - r3;
    *x.offset(7) = r1 + r0;
    *x.offset(5) = r1 - r0;
}
/* 16 point butterfly (in place, 4 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_16(mut x: *mut f32) {
    let mut r0: f32 = *x.offset(1) - *x.offset(9);
    let mut r1: f32 = *x.offset(0) - *x.offset(8);
    *x.offset(8) += *x.offset(0);
    *x.offset(9) += *x.offset(1);
    *x.offset(0) = (r0 + r1) * 0.70710678118654752441;
    *x.offset(1) = (r0 - r1) * 0.70710678118654752441;
    r0 = *x.offset(3) - *x.offset(11);
    r1 = *x.offset(10) - *x.offset(2);
    *x.offset(10) += *x.offset(2);
    *x.offset(11) += *x.offset(3);
    *x.offset(2) = r0;
    *x.offset(3) = r1;
    r0 = *x.offset(12) - *x.offset(4);
    r1 = *x.offset(13) - *x.offset(5);
    *x.offset(12) += *x.offset(4);
    *x.offset(13) += *x.offset(5);
    *x.offset(4) = (r0 - r1) * 0.70710678118654752441;
    *x.offset(5) = (r0 + r1) * 0.70710678118654752441;
    r0 = *x.offset(14) - *x.offset(6);
    r1 = *x.offset(15) - *x.offset(7);
    *x.offset(14) += *x.offset(6);
    *x.offset(15) += *x.offset(7);
    *x.offset(6) = r0;
    *x.offset(7) = r1;
    mdct_butterfly_8(x);
    mdct_butterfly_8(x.offset(8));
}
/* 32 point butterfly (in place, 4 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_32(mut x: *mut f32) {
    let mut r0: f32 = *x.offset(30) - *x.offset(14);
    let mut r1: f32 = *x.offset(31) - *x.offset(15);
    *x.offset(30) += *x.offset(14);
    *x.offset(31) += *x.offset(15);
    *x.offset(14) = r0;
    *x.offset(15) = r1;
    r0 = *x.offset(28) - *x.offset(12);
    r1 = *x.offset(29) - *x.offset(13);
    *x.offset(28) += *x.offset(12);
    *x.offset(29) += *x.offset(13);
    *x.offset(12) = r0 * 0.92387953251128675613 - r1 * 0.38268343236508977175;
    *x.offset(13) = r0 * 0.38268343236508977175 + r1 * 0.92387953251128675613;
    r0 = *x.offset(26) - *x.offset(10);
    r1 = *x.offset(27) - *x.offset(11);
    *x.offset(26) += *x.offset(10);
    *x.offset(27) += *x.offset(11);
    *x.offset(10) = (r0 - r1) * 0.70710678118654752441;
    *x.offset(11) = (r0 + r1) * 0.70710678118654752441;
    r0 = *x.offset(24) - *x.offset(8);
    r1 = *x.offset(25) - *x.offset(9);
    *x.offset(24) += *x.offset(8);
    *x.offset(25) += *x.offset(9);
    *x.offset(8) = r0 * 0.38268343236508977175 - r1 * 0.92387953251128675613;
    *x.offset(9) = r1 * 0.38268343236508977175 + r0 * 0.92387953251128675613;
    r0 = *x.offset(22) - *x.offset(6);
    r1 = *x.offset(7) - *x.offset(23);
    *x.offset(22) += *x.offset(6);
    *x.offset(23) += *x.offset(7);
    *x.offset(6) = r1;
    *x.offset(7) = r0;
    r0 = *x.offset(4) - *x.offset(20);
    r1 = *x.offset(5) - *x.offset(21);
    *x.offset(20) += *x.offset(4);
    *x.offset(21) += *x.offset(5);
    *x.offset(4) = r1 * 0.92387953251128675613 + r0 * 0.38268343236508977175;
    *x.offset(5) = r1 * 0.38268343236508977175 - r0 * 0.92387953251128675613;
    r0 = *x.offset(2) - *x.offset(18);
    r1 = *x.offset(3) - *x.offset(19);
    *x.offset(18) += *x.offset(2);
    *x.offset(19) += *x.offset(3);
    *x.offset(2) = (r1 + r0) * 0.70710678118654752441;
    *x.offset(3) = (r1 - r0) * 0.70710678118654752441;
    r0 = *x.offset(0) - *x.offset(16);
    r1 = *x.offset(1) - *x.offset(17);
    *x.offset(16) += *x.offset(0);
    *x.offset(17) += *x.offset(1);
    *x.offset(0) = r1 * 0.38268343236508977175 + r0 * 0.92387953251128675613;
    *x.offset(1) = r1 * 0.92387953251128675613 - r0 * 0.38268343236508977175;
    mdct_butterfly_16(x);
    mdct_butterfly_16(x.offset(16));
}
/* N point first stage butterfly (in place, 2 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_first(mut T: *mut f32, mut x: *mut f32, mut points: i32) {
    let mut x1: *mut f32 = x.offset(points as isize).offset(-(8));
    let mut x2: *mut f32 = x.offset((points >> 1) as isize).offset(-(8));
    let mut r0: f32 = 0.;
    let mut r1: f32 = 0.;
    loop {
        r0 = *x1.offset(6) - *x2.offset(6);
        r1 = *x1.offset(7) - *x2.offset(7);
        *x1.offset(6) += *x2.offset(6);
        *x1.offset(7) += *x2.offset(7);
        *x2.offset(6) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *x2.offset(7) = r1 * *T.offset(0) - r0 * *T.offset(1);
        r0 = *x1.offset(4) - *x2.offset(4);
        r1 = *x1.offset(5) - *x2.offset(5);
        *x1.offset(4) += *x2.offset(4);
        *x1.offset(5) += *x2.offset(5);
        *x2.offset(4) = r1 * *T.offset(5) + r0 * *T.offset(4);
        *x2.offset(5) = r1 * *T.offset(4) - r0 * *T.offset(5);
        r0 = *x1.offset(2) - *x2.offset(2);
        r1 = *x1.offset(3) - *x2.offset(3);
        *x1.offset(2) += *x2.offset(2);
        *x1.offset(3) += *x2.offset(3);
        *x2.offset(2) = r1 * *T.offset(9) + r0 * *T.offset(8);
        *x2.offset(3) = r1 * *T.offset(8) - r0 * *T.offset(9);
        r0 = *x1.offset(0) - *x2.offset(0);
        r1 = *x1.offset(1) - *x2.offset(1);
        *x1.offset(0) += *x2.offset(0);
        *x1.offset(1) += *x2.offset(1);
        *x2.offset(0) = r1 * *T.offset(13) + r0 * *T.offset(12);
        *x2.offset(1) = r1 * *T.offset(12) - r0 * *T.offset(13);
        x1 = x1.offset(-(8));
        x2 = x2.offset(-(8));
        T = T.offset(16);
        if !(x2 >= x) {
            break;
        }
    }
}
/* N/stage point generic N stage butterfly (in place, 2 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_generic(
    mut T: *mut f32,
    mut x: *mut f32,
    mut points: i32,
    mut trigint: i32,
) {
    let mut x1: *mut f32 = x.offset(points as isize).offset(-(8));
    let mut x2: *mut f32 = x.offset((points >> 1) as isize).offset(-(8));
    let mut r0: f32 = 0.;
    let mut r1: f32 = 0.;
    loop {
        r0 = *x1.offset(6) - *x2.offset(6);
        r1 = *x1.offset(7) - *x2.offset(7);
        *x1.offset(6) += *x2.offset(6);
        *x1.offset(7) += *x2.offset(7);
        *x2.offset(6) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *x2.offset(7) = r1 * *T.offset(0) - r0 * *T.offset(1);
        T = T.offset(trigint as isize);
        r0 = *x1.offset(4) - *x2.offset(4);
        r1 = *x1.offset(5) - *x2.offset(5);
        *x1.offset(4) += *x2.offset(4);
        *x1.offset(5) += *x2.offset(5);
        *x2.offset(4) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *x2.offset(5) = r1 * *T.offset(0) - r0 * *T.offset(1);
        T = T.offset(trigint as isize);
        r0 = *x1.offset(2) - *x2.offset(2);
        r1 = *x1.offset(3) - *x2.offset(3);
        *x1.offset(2) += *x2.offset(2);
        *x1.offset(3) += *x2.offset(3);
        *x2.offset(2) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *x2.offset(3) = r1 * *T.offset(0) - r0 * *T.offset(1);
        T = T.offset(trigint as isize);
        r0 = *x1.offset(0) - *x2.offset(0);
        r1 = *x1.offset(1) - *x2.offset(1);
        *x1.offset(0) += *x2.offset(0);
        *x1.offset(1) += *x2.offset(1);
        *x2.offset(0) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *x2.offset(1) = r1 * *T.offset(0) - r0 * *T.offset(1);
        T = T.offset(trigint as isize);
        x1 = x1.offset(-(8));
        x2 = x2.offset(-(8));
        if !(x2 >= x) {
            break;
        }
    }
}
#[inline]

unsafe extern "C" fn mdct_butterflies(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut x: *mut f32,
    mut points: i32,
) {
    let mut T: *mut f32 = (*init).trig;
    let mut stages: i32 = (*init).log2n - 5;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    stages -= 1;
    if stages > 0 {
        mdct_butterfly_first(T, x, points);
    }
    i = 1;
    loop {
        stages -= 1;
        if !(stages > 0) {
            break;
        }
        j = 0;
        while j < (1) << i {
            mdct_butterfly_generic(
                T,
                x.offset(((points >> i) * j) as isize),
                points >> i,
                (4) << i,
            );
            j += 1
        }
        i += 1
    }
    j = 0;
    while j < points {
        mdct_butterfly_32(x.offset(j as isize));
        j += 32
    }
}
#[no_mangle]

pub unsafe extern "C" fn mdct_clear(
    mut l: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
) {
    if !l.is_null() {
        if !(*l).trig.is_null() {
            crate::stdlib::free((*l).trig as *mut libc::c_void);
        }
        if !(*l).bitrev.is_null() {
            crate::stdlib::free((*l).bitrev as *mut libc::c_void);
        }
        crate::stdlib::memset(
            l as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup>(),
        );
    };
}
#[inline]

unsafe extern "C" fn mdct_bitreverse(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut x: *mut f32,
) {
    let mut n: i32 = (*init).n;
    let mut bit: *mut i32 = (*init).bitrev;
    let mut w0: *mut f32 = x;
    x = w0.offset((n >> 1) as isize);
    let mut w1: *mut f32 = x;
    let mut T: *mut f32 = (*init).trig.offset(n as isize);
    loop {
        let mut x0: *mut f32 = x.offset(*bit.offset(0) as isize);
        let mut x1: *mut f32 = x.offset(*bit.offset(1) as isize);
        let mut r0: f32 = *x0.offset(1) - *x1.offset(1);
        let mut r1: f32 = *x0.offset(0) + *x1.offset(0);
        let mut r2: f32 = r1 * *T.offset(0) + r0 * *T.offset(1);
        let mut r3: f32 = r1 * *T.offset(1) - r0 * *T.offset(0);
        w1 = w1.offset(-(4));
        r0 = (*x0.offset(1) + *x1.offset(1)) * 0.5;
        r1 = (*x0.offset(0) - *x1.offset(0)) * 0.5;
        *w0.offset(0) = r0 + r2;
        *w1.offset(2) = r0 - r2;
        *w0.offset(1) = r1 + r3;
        *w1.offset(3) = r3 - r1;
        x0 = x.offset(*bit.offset(2) as isize);
        x1 = x.offset(*bit.offset(3) as isize);
        r0 = *x0.offset(1) - *x1.offset(1);
        r1 = *x0.offset(0) + *x1.offset(0);
        r2 = r1 * *T.offset(2) + r0 * *T.offset(3);
        r3 = r1 * *T.offset(3) - r0 * *T.offset(2);
        r0 = (*x0.offset(1) + *x1.offset(1)) * 0.5;
        r1 = (*x0.offset(0) - *x1.offset(0)) * 0.5;
        *w0.offset(2) = r0 + r2;
        *w1.offset(0) = r0 - r2;
        *w0.offset(3) = r1 + r3;
        *w1.offset(1) = r3 - r1;
        T = T.offset(4);
        bit = bit.offset(4);
        w0 = w0.offset(4);
        if !(w0 < w1) {
            break;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn mdct_backward(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut in_0: *mut f32,
    mut out: *mut f32,
) {
    let mut n: i32 = (*init).n;
    let mut n2: i32 = n >> 1;
    let mut n4: i32 = n >> 2;
    /* rotate */
    let mut iX: *mut f32 = in_0.offset(n2 as isize).offset(-(7));
    let mut oX: *mut f32 = out.offset(n2 as isize).offset(n4 as isize);
    let mut T: *mut f32 = (*init).trig.offset(n4 as isize);
    loop {
        oX = oX.offset(-(4));
        *oX.offset(0) = -*iX.offset(2) * *T.offset(3) - *iX.offset(0) * *T.offset(2);
        *oX.offset(1) = *iX.offset(0) * *T.offset(3) - *iX.offset(2) * *T.offset(2);
        *oX.offset(2) = -*iX.offset(6) * *T.offset(1) - *iX.offset(4) * *T.offset(0);
        *oX.offset(3) = *iX.offset(4) * *T.offset(1) - *iX.offset(6) * *T.offset(0);
        iX = iX.offset(-(8));
        T = T.offset(4);
        if !(iX >= in_0) {
            break;
        }
    }
    iX = in_0.offset(n2 as isize).offset(-(8));
    oX = out.offset(n2 as isize).offset(n4 as isize);
    T = (*init).trig.offset(n4 as isize);
    loop {
        T = T.offset(-(4));
        *oX.offset(0) = *iX.offset(4) * *T.offset(3) + *iX.offset(6) * *T.offset(2);
        *oX.offset(1) = *iX.offset(4) * *T.offset(2) - *iX.offset(6) * *T.offset(3);
        *oX.offset(2) = *iX.offset(0) * *T.offset(1) + *iX.offset(2) * *T.offset(0);
        *oX.offset(3) = *iX.offset(0) * *T.offset(0) - *iX.offset(2) * *T.offset(1);
        iX = iX.offset(-(8));
        oX = oX.offset(4);
        if !(iX >= in_0) {
            break;
        }
    }
    mdct_butterflies(init, out.offset(n2 as isize), n2);
    mdct_bitreverse(init, out);
    /* roatate + window */
    let mut oX1: *mut f32 = out.offset(n2 as isize).offset(n4 as isize); /* forward needs working space */
    let mut oX2: *mut f32 = out.offset(n2 as isize).offset(n4 as isize);
    let mut iX_0: *mut f32 = out;
    T = (*init).trig.offset(n2 as isize);
    loop {
        oX1 = oX1.offset(-(4));
        *oX1.offset(3) = *iX_0.offset(0) * *T.offset(1) - *iX_0.offset(1) * *T.offset(0);
        *oX2.offset(0) = -(*iX_0.offset(0) * *T.offset(0) + *iX_0.offset(1) * *T.offset(1));
        *oX1.offset(2) = *iX_0.offset(2) * *T.offset(3) - *iX_0.offset(3) * *T.offset(2);
        *oX2.offset(1) = -(*iX_0.offset(2) * *T.offset(2) + *iX_0.offset(3) * *T.offset(3));
        *oX1.offset(1) = *iX_0.offset(4) * *T.offset(5) - *iX_0.offset(5) * *T.offset(4);
        *oX2.offset(2) = -(*iX_0.offset(4) * *T.offset(4) + *iX_0.offset(5) * *T.offset(5));
        *oX1.offset(0) = *iX_0.offset(6) * *T.offset(7) - *iX_0.offset(7) * *T.offset(6);
        *oX2.offset(3) = -(*iX_0.offset(6) * *T.offset(6) + *iX_0.offset(7) * *T.offset(7));
        oX2 = oX2.offset(4);
        iX_0 = iX_0.offset(8);
        T = T.offset(8);
        if !(iX_0 < oX1) {
            break;
        }
    }
    iX_0 = out.offset(n2 as isize).offset(n4 as isize);
    oX1 = out.offset(n4 as isize);
    oX2 = oX1;
    loop {
        oX1 = oX1.offset(-(4));
        iX_0 = iX_0.offset(-(4));
        let ref mut fresh0 = *oX1.offset(3);
        *fresh0 = *iX_0.offset(3);
        *oX2.offset(0) = -*fresh0;
        let ref mut fresh1 = *oX1.offset(2);
        *fresh1 = *iX_0.offset(2);
        *oX2.offset(1) = -*fresh1;
        let ref mut fresh2 = *oX1.offset(1);
        *fresh2 = *iX_0.offset(1);
        *oX2.offset(2) = -*fresh2;
        let ref mut fresh3 = *oX1.offset(0);
        *fresh3 = *iX_0.offset(0);
        *oX2.offset(3) = -*fresh3;
        oX2 = oX2.offset(4);
        if !(oX2 < iX_0) {
            break;
        }
    }
    iX_0 = out.offset(n2 as isize).offset(n4 as isize);
    oX1 = out.offset(n2 as isize).offset(n4 as isize);
    oX2 = out.offset(n2 as isize);
    loop {
        oX1 = oX1.offset(-(4));
        *oX1.offset(0) = *iX_0.offset(3);
        *oX1.offset(1) = *iX_0.offset(2);
        *oX1.offset(2) = *iX_0.offset(1);
        *oX1.offset(3) = *iX_0.offset(0);
        iX_0 = iX_0.offset(4);
        if !(oX1 > oX2) {
            break;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn mdct_forward(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut in_0: *mut f32,
    mut out: *mut f32,
) {
    let mut n: i32 = (*init).n;
    let mut n2: i32 = n >> 1;
    let mut n4: i32 = n >> 2;
    let mut n8: i32 = n >> 3;
    let mut fresh4 =
        ::std::vec::from_elem(0, (n as usize).wrapping_mul(::std::mem::size_of::<f32>()));
    let mut w: *mut f32 = fresh4.as_mut_ptr() as *mut f32;
    let mut w2: *mut f32 = w.offset(n2 as isize);
    /* rotate */
    /* window + rotate + step 1 */
    let mut r0: f32 = 0.;
    let mut r1: f32 = 0.;
    let mut x0: *mut f32 = in_0.offset(n2 as isize).offset(n4 as isize);
    let mut x1: *mut f32 = x0.offset(1);
    let mut T: *mut f32 = (*init).trig.offset(n2 as isize);
    let mut i: i32 = 0;
    i = 0;
    while i < n8 {
        x0 = x0.offset(-(4));
        T = T.offset(-(2));
        r0 = *x0.offset(2) + *x1.offset(0);
        r1 = *x0.offset(0) + *x1.offset(2);
        *w2.offset(i as isize) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *w2.offset((i + 1) as isize) = r1 * *T.offset(0) - r0 * *T.offset(1);
        x1 = x1.offset(4);
        i += 2
    }
    x1 = in_0.offset(1);
    while i < n2 - n8 {
        T = T.offset(-(2));
        x0 = x0.offset(-(4));
        r0 = *x0.offset(2) - *x1.offset(0);
        r1 = *x0.offset(0) - *x1.offset(2);
        *w2.offset(i as isize) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *w2.offset((i + 1) as isize) = r1 * *T.offset(0) - r0 * *T.offset(1);
        x1 = x1.offset(4);
        i += 2
    }
    x0 = in_0.offset(n as isize);
    while i < n2 {
        T = T.offset(-(2));
        x0 = x0.offset(-(4));
        r0 = -*x0.offset(2) - *x1.offset(0);
        r1 = -*x0.offset(0) - *x1.offset(2);
        *w2.offset(i as isize) = r1 * *T.offset(1) + r0 * *T.offset(0);
        *w2.offset((i + 1) as isize) = r1 * *T.offset(0) - r0 * *T.offset(1);
        x1 = x1.offset(4);
        i += 2
    }
    mdct_butterflies(init, w.offset(n2 as isize), n2);
    mdct_bitreverse(init, w);
    /* roatate + window */
    T = (*init).trig.offset(n2 as isize);
    x0 = out.offset(n2 as isize);
    i = 0;
    while i < n4 {
        x0 = x0.offset(-1);
        *out.offset(i as isize) =
            (*w.offset(0) * *T.offset(0) + *w.offset(1) * *T.offset(1)) * (*init).scale;
        *x0.offset(0) = (*w.offset(0) * *T.offset(1) - *w.offset(1) * *T.offset(0)) * (*init).scale;
        w = w.offset(2);
        T = T.offset(2);
        i += 1
    }
}
