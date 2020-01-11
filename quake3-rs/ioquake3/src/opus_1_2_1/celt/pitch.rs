pub mod entcode_h {
    /* Tested exhaustively for all n and for 1<=d<=256 */
    #[inline]

    pub unsafe extern "C" fn celt_udiv(
        mut n: crate::opus_types_h::opus_uint32,
        mut d: crate::opus_types_h::opus_uint32,
    ) -> crate::opus_types_h::opus_uint32 {
        return n.wrapping_div(d);
    }
}

pub mod pitch_h {
    #[inline]

    pub unsafe extern "C" fn dual_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y01: *const crate::arch_h::opus_val16,
        mut y02: *const crate::arch_h::opus_val16,
        mut N: i32,
        mut xy1: *mut crate::arch_h::opus_val32,
        mut xy2: *mut crate::arch_h::opus_val32,
    ) {
        let mut _i: i32 = 0;
        let mut xy01: crate::arch_h::opus_val32 = 0f32;
        let mut xy02: crate::arch_h::opus_val32 = 0f32;

        for i in 0..N {
            xy01 = xy01 + *x.offset(i as isize) * *y01.offset(i as isize);

            xy02 = xy02 + *x.offset(i as isize) * *y02.offset(i as isize);
        }
        *xy1 = xy01;
        *xy2 = xy02;
    }
    #[inline]

    pub unsafe extern "C" fn xcorr_kernel_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut sum: *mut crate::arch_h::opus_val32,
        mut len: i32,
    ) {
        let mut j: i32 = 0;
        let mut y_0: crate::arch_h::opus_val16 = 0.;
        let mut y_1: crate::arch_h::opus_val16 = 0.;
        let mut y_2: crate::arch_h::opus_val16 = 0.;
        let mut y_3: crate::arch_h::opus_val16 = 0.;
        y_3 = 0f32;
        let fresh0 = y;
        y = y.offset(1);
        y_0 = *fresh0;
        let fresh1 = y;
        y = y.offset(1);
        y_1 = *fresh1;
        let fresh2 = y;
        y = y.offset(1);
        y_2 = *fresh2;
        j = 0;
        while j < len - 3 {
            let mut tmp: crate::arch_h::opus_val16 = 0.;
            let fresh3 = x;
            x = x.offset(1);
            tmp = *fresh3;
            let fresh4 = y;
            y = y.offset(1);
            y_3 = *fresh4;
            *sum.offset(0) = *sum.offset(0) + tmp * y_0;
            *sum.offset(1) = *sum.offset(1) + tmp * y_1;
            *sum.offset(2) = *sum.offset(2) + tmp * y_2;
            *sum.offset(3) = *sum.offset(3) + tmp * y_3;
            let fresh5 = x;
            x = x.offset(1);
            tmp = *fresh5;
            let fresh6 = y;
            y = y.offset(1);
            y_0 = *fresh6;
            *sum.offset(0) = *sum.offset(0) + tmp * y_1;
            *sum.offset(1) = *sum.offset(1) + tmp * y_2;
            *sum.offset(2) = *sum.offset(2) + tmp * y_3;
            *sum.offset(3) = *sum.offset(3) + tmp * y_0;
            let fresh7 = x;
            x = x.offset(1);
            tmp = *fresh7;
            let fresh8 = y;
            y = y.offset(1);
            y_1 = *fresh8;
            *sum.offset(0) = *sum.offset(0) + tmp * y_2;
            *sum.offset(1) = *sum.offset(1) + tmp * y_3;
            *sum.offset(2) = *sum.offset(2) + tmp * y_0;
            *sum.offset(3) = *sum.offset(3) + tmp * y_1;
            let fresh9 = x;
            x = x.offset(1);
            tmp = *fresh9;
            let fresh10 = y;
            y = y.offset(1);
            y_2 = *fresh10;
            *sum.offset(0) = *sum.offset(0) + tmp * y_3;
            *sum.offset(1) = *sum.offset(1) + tmp * y_0;
            *sum.offset(2) = *sum.offset(2) + tmp * y_1;
            *sum.offset(3) = *sum.offset(3) + tmp * y_2;
            j += 4
        }
        let fresh11 = j;
        j = j + 1;
        if fresh11 < len {
            let fresh12 = x;
            x = x.offset(1);
            let mut tmp_0: crate::arch_h::opus_val16 = *fresh12;
            let fresh13 = y;
            y = y.offset(1);
            y_3 = *fresh13;
            *sum.offset(0) = *sum.offset(0) + tmp_0 * y_0;
            *sum.offset(1) = *sum.offset(1) + tmp_0 * y_1;
            *sum.offset(2) = *sum.offset(2) + tmp_0 * y_2;
            *sum.offset(3) = *sum.offset(3) + tmp_0 * y_3
        }
        let fresh14 = j;
        j = j + 1;
        if fresh14 < len {
            let fresh15 = x;
            x = x.offset(1);
            let mut tmp_1: crate::arch_h::opus_val16 = *fresh15;
            let fresh16 = y;
            y = y.offset(1);
            y_0 = *fresh16;
            *sum.offset(0) = *sum.offset(0) + tmp_1 * y_1;
            *sum.offset(1) = *sum.offset(1) + tmp_1 * y_2;
            *sum.offset(2) = *sum.offset(2) + tmp_1 * y_3;
            *sum.offset(3) = *sum.offset(3) + tmp_1 * y_0
        }
        if j < len {
            let fresh17 = x;
            x = x.offset(1);
            let mut tmp_2: crate::arch_h::opus_val16 = *fresh17;
            let fresh18 = y;
            y = y.offset(1);
            y_1 = *fresh18;
            *sum.offset(0) = *sum.offset(0) + tmp_2 * y_2;
            *sum.offset(1) = *sum.offset(1) + tmp_2 * y_3;
            *sum.offset(2) = *sum.offset(2) + tmp_2 * y_0;
            *sum.offset(3) = *sum.offset(3) + tmp_2 * y_1
        };
    }
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: i32,
    ) -> crate::arch_h::opus_val32 {
        let mut _i: i32 = 0;
        let mut xy: crate::arch_h::opus_val32 = 0f32;

        for i in 0..N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
        }
        return xy;
    }
}

pub use crate::arch_h::celt_sig;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::pitch::entcode_h::celt_udiv;
pub use crate::stdlib::__uint32_t;

pub use crate::stdlib::uint32_t;

pub use crate::src::opus_1_2_1::celt::pitch::pitch_h::celt_inner_prod_c;
pub use crate::src::opus_1_2_1::celt::pitch::pitch_h::dual_inner_prod_c;
pub use crate::src::opus_1_2_1::celt::pitch::pitch_h::xcorr_kernel_c;
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/* *
  @file pitch.c
  @brief Pitch analysis
*/
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

unsafe extern "C" fn find_best_pitch(
    mut xcorr: *mut crate::arch_h::opus_val32,
    mut y: *mut crate::arch_h::opus_val16,
    mut len: i32,
    mut max_pitch: i32,
    mut best_pitch: *mut i32,
) {
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut Syy: crate::arch_h::opus_val32 = 1f32;
    let mut best_num: [crate::arch_h::opus_val16; 2] = [0.; 2];
    let mut best_den: [crate::arch_h::opus_val32; 2] = [0.; 2];
    best_num[0] = -1f32;
    best_num[1] = -1f32;
    best_den[0] = 0f32;
    best_den[1] = 0f32;
    *best_pitch.offset(0) = 0;
    *best_pitch.offset(1) = 1;

    for j in 0..len {
        Syy = Syy + *y.offset(j as isize) * *y.offset(j as isize);
    }
    i = 0;
    while i < max_pitch {
        if *xcorr.offset(i as isize) > 0f32 {
            let mut num: crate::arch_h::opus_val16 = 0.;
            let mut xcorr16: crate::arch_h::opus_val32 = 0.;
            xcorr16 = *xcorr.offset(i as isize);
            /* Considering the range of xcorr16, this should avoid both underflows
            and overflows (inf) when squaring xcorr16 */
            xcorr16 *= 1e-12;
            num = xcorr16 * xcorr16;
            if num * best_den[1] > best_num[1] * Syy {
                if num * best_den[0] > best_num[0] * Syy {
                    best_num[1] = best_num[0];
                    best_den[1] = best_den[0];
                    *best_pitch.offset(1) = *best_pitch.offset(0);
                    best_num[0] = num;
                    best_den[0] = Syy;
                    *best_pitch.offset(0) = i
                } else {
                    best_num[1] = num;
                    best_den[1] = Syy;
                    *best_pitch.offset(1) = i
                }
            }
        }
        Syy += *y.offset((i + len) as isize) * *y.offset((i + len) as isize)
            - *y.offset(i as isize) * *y.offset(i as isize);
        Syy = if 1f32 > Syy { 1f32 } else { Syy };
        i += 1
    }
}

unsafe extern "C" fn celt_fir5(
    mut x: *const crate::arch_h::opus_val16,
    mut num: *const crate::arch_h::opus_val16,
    mut y: *mut crate::arch_h::opus_val16,
    mut N: i32,
    mut mem: *mut crate::arch_h::opus_val16,
) {
    let mut _i: i32 = 0;
    let mut num0: crate::arch_h::opus_val16 = 0.;
    let mut num1: crate::arch_h::opus_val16 = 0.;
    let mut num2: crate::arch_h::opus_val16 = 0.;
    let mut num3: crate::arch_h::opus_val16 = 0.;
    let mut num4: crate::arch_h::opus_val16 = 0.;
    let mut mem0: crate::arch_h::opus_val32 = 0.;
    let mut mem1: crate::arch_h::opus_val32 = 0.;
    let mut mem2: crate::arch_h::opus_val32 = 0.;
    let mut mem3: crate::arch_h::opus_val32 = 0.;
    let mut mem4: crate::arch_h::opus_val32 = 0.;
    num0 = *num.offset(0);
    num1 = *num.offset(1);
    num2 = *num.offset(2);
    num3 = *num.offset(3);
    num4 = *num.offset(4);
    mem0 = *mem.offset(0);
    mem1 = *mem.offset(1);
    mem2 = *mem.offset(2);
    mem3 = *mem.offset(3);
    mem4 = *mem.offset(4);

    for i in 0..N {
        let mut sum: crate::arch_h::opus_val32 = *x.offset(i as isize);

        sum = sum + num0 * mem0;

        sum = sum + num1 * mem1;

        sum = sum + num2 * mem2;

        sum = sum + num3 * mem3;

        sum = sum + num4 * mem4;

        mem4 = mem3;

        mem3 = mem2;

        mem2 = mem1;

        mem1 = mem0;

        mem0 = *x.offset(i as isize);

        *y.offset(i as isize) = sum;
    }
    *mem.offset(0) = mem0;
    *mem.offset(1) = mem1;
    *mem.offset(2) = mem2;
    *mem.offset(3) = mem3;
    *mem.offset(4) = mem4;
}
#[no_mangle]

pub unsafe extern "C" fn pitch_downsample(
    mut x: *mut *mut crate::arch_h::celt_sig,
    mut x_lp: *mut crate::arch_h::opus_val16,
    mut len: i32,
    mut C: i32,
    mut arch: i32,
) {
    let mut i: i32 = 0;
    let mut ac: [crate::arch_h::opus_val32; 5] = [0.; 5];
    let mut tmp: crate::arch_h::opus_val16 = 1.0;
    let mut lpc: [crate::arch_h::opus_val16; 4] = [0.; 4];
    let mut mem: [crate::arch_h::opus_val16; 5] = [0f32, 0f32, 0f32, 0f32, 0f32];
    let mut lpc2: [crate::arch_h::opus_val16; 5] = [0.; 5];
    let mut c1: crate::arch_h::opus_val16 = 0.8;
    i = 1;
    while i < len >> 1 {
        *x_lp.offset(i as isize) = 0.5f32
            * (0.5
                * (*(*x.offset(0)).offset((2 * i - 1) as isize)
                    + *(*x.offset(0)).offset((2 * i + 1) as isize))
                + *(*x.offset(0)).offset((2 * i) as isize));
        i += 1
    }
    *x_lp.offset(0) = 0.5f32 * (0.5 * *(*x.offset(0)).offset(1) + *(*x.offset(0)).offset(0));
    if C == 2 {
        i = 1;
        while i < len >> 1 {
            let ref mut fresh19 = *x_lp.offset(i as isize);
            *fresh19 += 0.5f32
                * (0.5
                    * (*(*x.offset(1)).offset((2 * i - 1) as isize)
                        + *(*x.offset(1)).offset((2 * i + 1) as isize))
                    + *(*x.offset(1)).offset((2 * i) as isize));
            i += 1
        }
        let ref mut fresh20 = *x_lp.offset(0);
        *fresh20 += 0.5f32 * (0.5 * *(*x.offset(1)).offset(1) + *(*x.offset(1)).offset(0))
    }
    crate::src::opus_1_2_1::celt::celt_lpc::_celt_autocorr(
        x_lp,
        ac.as_mut_ptr(),
        0 as *const crate::arch_h::opus_val16,
        0,
        4,
        len >> 1,
        arch,
    );
    /* Noise floor -40 dB */
    ac[0] *= 1.0001;
    /* Lag windowing */
    i = 1;
    while i <= 4 {
        /*ac[i] *= exp(-.5*(2*M_PI*.002*i)*(2*M_PI*.002*i));*/
        ac[i as usize] -= ac[i as usize] * (0.008 * i as f32) * (0.008 * i as f32);
        i += 1
    }
    crate::src::opus_1_2_1::celt::celt_lpc::_celt_lpc(lpc.as_mut_ptr(), ac.as_mut_ptr(), 4);
    i = 0;
    while i < 4 {
        tmp = 0.9 * tmp;
        lpc[i as usize] = lpc[i as usize] * tmp;
        i += 1
    }
    /* Add a zero */
    lpc2[0] = lpc[0] + 0.8;
    lpc2[1] = lpc[1] + c1 * lpc[0];
    lpc2[2] = lpc[2] + c1 * lpc[1];
    lpc2[3] = lpc[3] + c1 * lpc[2];
    lpc2[4] = c1 * lpc[3];
    celt_fir5(x_lp, lpc2.as_mut_ptr(), x_lp, len >> 1, mem.as_mut_ptr());
}
/* OPT: This is the kernel you really want to optimize. It gets used a lot
by the prefilter and by the PLC. */
/* gcc doesn't realize that y_3 can't be used uninitialized */
/* OVERRIDE_XCORR_KERNEL */
/*We make sure a C version is always available for cases where the overhead of
vectorization and passing around an arch flag aren't worth it.*/
/* Pure C implementation. */
#[no_mangle]

pub unsafe extern "C" fn celt_pitch_xcorr_c(
    mut _x: *const crate::arch_h::opus_val16,
    mut _y: *const crate::arch_h::opus_val16,
    mut xcorr: *mut crate::arch_h::opus_val32,
    mut len: i32,
    mut max_pitch: i32,
    mut _arch: i32,
) {
    /* This is a simple version of the pitch correlation that should work
    well on DSPs like Blackfin and TI C5x/C6x */
    /* Unrolled version of the pitch correlation -- runs faster on x86 and ARM */
    let mut i: i32 = 0;
    /*The EDSP version requires that max_pitch is at least 1, and that _x is
     32-bit aligned.
    Since it's hard to put asserts in assembly, put them here.*/
    i = 0;
    while i < max_pitch - 3 {
        let mut sum: [crate::arch_h::opus_val32; 4] = [0f32, 0f32, 0f32, 0f32];
        xcorr_kernel_c(_x, _y.offset(i as isize), sum.as_mut_ptr(), len);
        *xcorr.offset(i as isize) = sum[0];
        *xcorr.offset((i + 1) as isize) = sum[1];
        *xcorr.offset((i + 2) as isize) = sum[2];
        *xcorr.offset((i + 3) as isize) = sum[3];
        i += 4
    }
    /* In case max_pitch isn't a multiple of 4, do non-unrolled version. */
    while i < max_pitch {
        let mut sum_0: crate::arch_h::opus_val32 = 0.;
        sum_0 = celt_inner_prod_c(_x, _y.offset(i as isize), len);
        *xcorr.offset(i as isize) = sum_0;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn pitch_search(
    mut x_lp: *const crate::arch_h::opus_val16,
    mut y: *mut crate::arch_h::opus_val16,
    mut len: i32,
    mut max_pitch: i32,
    mut pitch: *mut i32,
    mut arch: i32,
) {
    let mut _i: i32 = 0;
    let mut j: i32 = 0;
    let mut lag: i32 = 0;
    let mut best_pitch: [i32; 2] = [0, 0];
    let mut x_lp4: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut y_lp4: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut xcorr: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    let mut offset: i32 = 0;
    lag = len + max_pitch;
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((len >> 2) as usize),
    );
    x_lp4 = fresh21.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh22 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((lag >> 2) as usize),
    );
    y_lp4 = fresh22.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh23 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val32>())
            .wrapping_mul((max_pitch >> 1) as usize),
    );
    xcorr = fresh23.as_mut_ptr() as *mut crate::arch_h::opus_val32;
    /* Downsample by 2 again */
    j = 0;
    while j < len >> 2 {
        *x_lp4.offset(j as isize) = *x_lp.offset((2 * j) as isize);
        j += 1
    }
    j = 0;
    while j < lag >> 2 {
        *y_lp4.offset(j as isize) = *y.offset((2 * j) as isize);
        j += 1
    }
    /* Coarse search with 4x decimation */
    celt_pitch_xcorr_c(x_lp4, y_lp4, xcorr, len >> 2, max_pitch >> 2, arch);
    find_best_pitch(
        xcorr,
        y_lp4,
        len >> 2,
        max_pitch >> 2,
        best_pitch.as_mut_ptr(),
    );
    /* Finer search with 2x decimation */

    for i in 0..max_pitch >> 1 {
        let mut sum: crate::arch_h::opus_val32 = 0.;

        *xcorr.offset(i as isize) = 0f32;

        if !(crate::stdlib::abs(i - 2 * best_pitch[0]) > 2
            && crate::stdlib::abs(i - 2 * best_pitch[1]) > 2)
        {
            sum = celt_inner_prod_c(x_lp, y.offset(i as isize), len >> 1);
            *xcorr.offset(i as isize) = if -1f32 > sum { -1f32 } else { sum }
        }
    }
    find_best_pitch(xcorr, y, len >> 1, max_pitch >> 1, best_pitch.as_mut_ptr());
    /* Refine by pseudo-interpolation */
    if best_pitch[0] > 0 && best_pitch[0] < (max_pitch >> 1) - 1 {
        let mut a: crate::arch_h::opus_val32 = 0.;
        let mut b: crate::arch_h::opus_val32 = 0.;
        let mut c: crate::arch_h::opus_val32 = 0.;
        a = *xcorr.offset((best_pitch[0] - 1) as isize);
        b = *xcorr.offset(best_pitch[0] as isize);
        c = *xcorr.offset((best_pitch[0] + 1) as isize);
        if c - a > 0.7 * (b - a) {
            offset = 1
        } else if a - c > 0.7 * (b - c) {
            offset = -(1)
        } else {
            offset = 0
        }
    } else {
        offset = 0
    }
    *pitch = 2 * best_pitch[0] - offset;
}

unsafe extern "C" fn compute_pitch_gain(
    mut xy: crate::arch_h::opus_val32,
    mut xx: crate::arch_h::opus_val32,
    mut yy: crate::arch_h::opus_val32,
) -> crate::arch_h::opus_val16 {
    return xy / crate::stdlib::sqrt((1f32 + xx * yy) as f64) as f32;
}

static mut second_check: [i32; 16] = [0, 0, 3, 2, 3, 2, 5, 2, 3, 2, 3, 2, 5, 2, 3, 2];
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/* *
  @file pitch.h
  @brief Pitch analysis
*/
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
#[no_mangle]

pub unsafe extern "C" fn remove_doubling(
    mut x: *mut crate::arch_h::opus_val16,
    mut maxperiod: i32,
    mut minperiod: i32,
    mut N: i32,
    mut T0_: *mut i32,
    mut prev_period: i32,
    mut prev_gain: crate::arch_h::opus_val16,
    mut _arch: i32,
) -> crate::arch_h::opus_val16 {
    let mut k: i32 = 0;
    let mut _i: i32 = 0;
    let mut T: i32 = 0;
    let mut T0: i32 = 0;
    let mut g: crate::arch_h::opus_val16 = 0.;
    let mut g0: crate::arch_h::opus_val16 = 0.;
    let mut pg: crate::arch_h::opus_val16 = 0.;
    let mut xy: crate::arch_h::opus_val32 = 0.;
    let mut xx: crate::arch_h::opus_val32 = 0.;
    let mut yy: crate::arch_h::opus_val32 = 0.;
    let mut xy2: crate::arch_h::opus_val32 = 0.;
    let mut xcorr: [crate::arch_h::opus_val32; 3] = [0.; 3];
    let mut best_xy: crate::arch_h::opus_val32 = 0.;
    let mut best_yy: crate::arch_h::opus_val32 = 0.;
    let mut offset: i32 = 0;
    let mut minperiod0: i32 = 0;
    let mut yy_lookup: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    minperiod0 = minperiod;
    maxperiod /= 2;
    minperiod /= 2;
    *T0_ /= 2;
    prev_period /= 2;
    N /= 2;
    x = x.offset(maxperiod as isize);
    if *T0_ >= maxperiod {
        *T0_ = maxperiod - 1
    }
    T0 = *T0_;
    T = T0;
    let mut fresh24 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val32>())
            .wrapping_mul((maxperiod + 1i32) as usize),
    );
    yy_lookup = fresh24.as_mut_ptr() as *mut crate::arch_h::opus_val32;
    dual_inner_prod_c(x, x, x.offset(-(T0 as isize)), N, &mut xx, &mut xy);
    *yy_lookup.offset(0) = xx;
    yy = xx;

    for i in 1..=maxperiod {
        yy = yy + *x.offset(-i as isize) * *x.offset(-i as isize)
            - *x.offset((N - i) as isize) * *x.offset((N - i) as isize);

        *yy_lookup.offset(i as isize) = if 0f32 > yy { 0f32 } else { yy };
    }
    yy = *yy_lookup.offset(T0 as isize);
    best_xy = xy;
    best_yy = yy;
    g0 = compute_pitch_gain(xy, xx, yy);
    g = g0;
    /* Look for any pitch at T/k */
    k = 2;
    while k <= 15 {
        let mut T1: i32 = 0;
        let mut T1b: i32 = 0;
        let mut g1: crate::arch_h::opus_val16 = 0.;
        let mut cont: crate::arch_h::opus_val16 = 0f32;
        let mut thresh: crate::arch_h::opus_val16 = 0.;
        T1 = celt_udiv(
            (2 * T0 + k) as crate::opus_types_h::opus_uint32,
            (2 * k) as crate::opus_types_h::opus_uint32,
        ) as i32;
        if T1 < minperiod {
            break;
        }
        /* Look for another strong correlation at T1b */
        if k == 2 {
            if T1 + T0 > maxperiod {
                T1b = T0
            } else {
                T1b = T0 + T1
            }
        } else {
            T1b = celt_udiv(
                (2 * second_check[k as usize] * T0 + k) as crate::opus_types_h::opus_uint32,
                (2 * k) as crate::opus_types_h::opus_uint32,
            ) as i32
        }
        dual_inner_prod_c(
            x,
            &mut *x.offset(-T1 as isize),
            &mut *x.offset(-T1b as isize),
            N,
            &mut xy,
            &mut xy2,
        );
        xy = 0.5 * (xy + xy2);
        yy = 0.5 * (*yy_lookup.offset(T1 as isize) + *yy_lookup.offset(T1b as isize));
        g1 = compute_pitch_gain(xy, xx, yy);
        if crate::stdlib::abs(T1 - prev_period) <= 1 {
            cont = prev_gain
        } else if crate::stdlib::abs(T1 - prev_period) <= 2 && 5 * k * k < T0 {
            cont = 0.5 * prev_gain
        } else {
            cont = 0f32
        }
        thresh = if 0.3 > 0.7 * g0 - cont {
            0.3
        } else {
            (0.7 * g0) - cont
        };
        /* Bias against very high pitch (very short period) to avoid false-positives
        due to short-term correlation */
        if T1 < 3 * minperiod {
            thresh = if 0.4 > 0.85 * g0 - cont {
                0.4
            } else {
                (0.85 * g0) - cont
            }
        } else if T1 < 2 * minperiod {
            thresh = if 0.5 > 0.9 * g0 - cont {
                0.5
            } else {
                (0.9 * g0) - cont
            }
        }
        if g1 > thresh {
            best_xy = xy;
            best_yy = yy;
            T = T1;
            g = g1
        }
        k += 1
    }
    best_xy = if 0f32 > best_xy { 0f32 } else { best_xy };
    if best_yy <= best_xy {
        pg = 1.0
    } else {
        pg = best_xy / (best_yy + 1f32)
    }
    k = 0;
    while k < 3 {
        xcorr[k as usize] = celt_inner_prod_c(x, x.offset(-((T + k - 1) as isize)), N);
        k += 1
    }
    if xcorr[2] - xcorr[0] > 0.7 * (xcorr[1] - xcorr[0]) {
        offset = 1
    } else if xcorr[0] - xcorr[2] > 0.7 * (xcorr[1] - xcorr[2]) {
        offset = -(1)
    } else {
        offset = 0
    }
    if pg > g {
        pg = g
    }
    *T0_ = 2 * T + offset;
    if *T0_ < minperiod0 {
        *T0_ = minperiod0
    }
    return pg;
}
