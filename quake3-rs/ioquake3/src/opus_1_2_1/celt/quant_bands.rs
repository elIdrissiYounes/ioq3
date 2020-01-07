use ::libc;

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    #[inline]

    pub unsafe extern "C" fn ec_range_bytes(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> crate::opus_types_h::opus_uint32 {
        return (*_this).offs;
    }
    #[inline]

    pub unsafe extern "C" fn ec_get_buffer(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> *mut u8 {
        return (*_this).buf;
    }
    /*Returns the number of bits "used" by the encoded or decoded symbols so far.
    This same number can be computed in either the encoder or the decoder, and is
     suitable for making coding decisions.
    Return: The number of bits.
            This will always be slightly larger than the exact value (e.g., all
             rounding error is in the positive direction).*/
    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> i32 {
        return (*_this).nbits_total
            - (::std::mem::size_of::<u32>() as i32 * 8 - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod mathops_h {

    #[inline]

    pub unsafe extern "C" fn celt_log2(mut x: f32) -> f32 {
        let mut integer: i32 = 0;
        let mut frac: f32 = 0.;
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        integer = (in_0.i >> 23).wrapping_sub(127u32) as i32;
        in_0.i = (in_0.i).wrapping_sub((integer << 23) as u32);
        frac = in_0.f - 1.5;
        frac = -0.41445418 + frac * (0.95909232 + frac * (-0.33951290 + frac * 0.16541097));
        return (1 + integer) as f32 + frac;
    }
    use crate::opus_types_h::opus_uint32;
    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_tell_frac;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::quant_bands::entcode_h::ec_get_buffer;
pub use crate::src::opus_1_2_1::celt::quant_bands::entcode_h::ec_range_bytes;
pub use crate::src::opus_1_2_1::celt::quant_bands::entcode_h::ec_tell;
pub use crate::src::opus_1_2_1::celt::quant_bands::mathops_h::celt_log2;
use crate::stdlib::floor;

use crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp;
use crate::src::opus_1_2_1::celt::entdec::ec_dec_bits;
use crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_bits;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf;
use crate::src::opus_1_2_1::celt::laplace::ec_laplace_decode;
use crate::src::opus_1_2_1::celt::laplace::ec_laplace_encode;
use crate::stdlib::abs;
use crate::stdlib::memcpy;
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
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
/* Mean energy in each band quantized in Q4 and converted back to float */
#[no_mangle]

pub static mut eMeans: [crate::arch_h::opus_val16; 25] = [
    6.437500, 6.250000, 5.750000, 5.312500, 5.062500, 4.812500, 4.500000, 4.375000, 4.875000,
    4.687500, 4.562500, 4.437500, 4.875000, 4.625000, 4.312500, 4.500000, 4.375000, 4.625000,
    4.750000, 4.437500, 3.750000, 3.750000, 3.750000, 3.750000, 3.750000,
];
/* prediction coefficients: 0.9, 0.8, 0.65, 0.5 */

static mut pred_coef: [crate::arch_h::opus_val16; 4] = [
    (29440f64 / 32768.0) as crate::arch_h::opus_val16,
    (26112f64 / 32768.0) as crate::arch_h::opus_val16,
    (21248f64 / 32768.0) as crate::arch_h::opus_val16,
    (16384f64 / 32768.0) as crate::arch_h::opus_val16,
];

static mut beta_coef: [crate::arch_h::opus_val16; 4] = [
    (30147f64 / 32768.0) as crate::arch_h::opus_val16,
    (22282f64 / 32768.0) as crate::arch_h::opus_val16,
    (12124f64 / 32768.0) as crate::arch_h::opus_val16,
    (6554f64 / 32768.0) as crate::arch_h::opus_val16,
];

static mut beta_intra: crate::arch_h::opus_val16 = (4915f64 / 32768.0) as crate::arch_h::opus_val16;
/*Parameters of the Laplace-like probability models used for the coarse energy.
There is one pair of parameters for each frame size, prediction type
 (inter/intra), and band number.
The first number of each pair is the probability of 0, and the second is the
 decay rate, both in Q8 precision.*/

static mut e_prob_model: [[[u8; 42]; 2]; 4] = [
    [
        [
            72, 127, 65, 129, 66, 128, 65, 128, 64, 128, 62, 128, 64, 128, 64, 128, 92, 78, 92, 79,
            92, 78, 90, 79, 116, 41, 115, 40, 114, 40, 132, 26, 132, 26, 145, 17, 161, 12, 176, 10,
            177, 11,
        ],
        [
            24, 179, 48, 138, 54, 135, 54, 132, 53, 134, 56, 133, 55, 132, 55, 132, 61, 114, 70,
            96, 74, 88, 75, 88, 87, 74, 89, 66, 91, 67, 100, 59, 108, 50, 120, 40, 122, 37, 97, 43,
            78, 50,
        ],
    ],
    [
        [
            83, 78, 84, 81, 88, 75, 86, 74, 87, 71, 90, 73, 93, 74, 93, 74, 109, 40, 114, 36, 117,
            34, 117, 34, 143, 17, 145, 18, 146, 19, 162, 12, 165, 10, 178, 7, 189, 6, 190, 8, 177,
            9,
        ],
        [
            23, 178, 54, 115, 63, 102, 66, 98, 69, 99, 74, 89, 71, 91, 73, 91, 78, 89, 86, 80, 92,
            66, 93, 64, 102, 59, 103, 60, 104, 60, 117, 52, 123, 44, 138, 35, 133, 31, 97, 38, 77,
            45,
        ],
    ],
    [
        [
            61, 90, 93, 60, 105, 42, 107, 41, 110, 45, 116, 38, 113, 38, 112, 38, 124, 26, 132, 27,
            136, 19, 140, 20, 155, 14, 159, 16, 158, 18, 170, 13, 177, 10, 187, 8, 192, 6, 175, 9,
            159, 10,
        ],
        [
            21, 178, 59, 110, 71, 86, 75, 85, 84, 83, 91, 66, 88, 73, 87, 72, 92, 75, 98, 72, 105,
            58, 107, 54, 115, 52, 114, 55, 112, 56, 129, 51, 132, 40, 150, 33, 140, 29, 98, 35, 77,
            42,
        ],
    ],
    [
        [
            42, 121, 96, 66, 108, 43, 111, 40, 117, 44, 123, 32, 120, 36, 119, 33, 127, 33, 134,
            34, 139, 21, 147, 23, 152, 20, 158, 25, 154, 26, 166, 21, 173, 16, 184, 13, 184, 10,
            150, 13, 139, 15,
        ],
        [
            22, 178, 63, 114, 74, 82, 84, 83, 92, 82, 103, 62, 96, 72, 96, 67, 101, 73, 107, 72,
            113, 55, 118, 52, 125, 52, 118, 52, 117, 55, 135, 49, 137, 39, 157, 32, 145, 29, 97,
            33, 77, 40,
        ],
    ],
];

static mut small_energy_icdf: [u8; 3] = [2, 1, 0];

unsafe extern "C" fn loss_distortion(
    mut eBands: *const crate::arch_h::opus_val16,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut start: i32,
    mut end: i32,
    mut len: i32,
    mut C: i32,
) -> crate::arch_h::opus_val32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut dist: crate::arch_h::opus_val32 = 0f32;
    c = 0;
    loop {
        i = start;
        while i < end {
            let mut d: crate::arch_h::opus_val16 =
                *eBands.offset((i + c * len) as isize) - *oldEBands.offset((i + c * len) as isize);
            dist = dist + d * d;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    return if (200f32) < dist { 200f32 } else { dist };
}

unsafe extern "C" fn quant_coarse_energy_impl(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut eBands: *const crate::arch_h::opus_val16,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut budget: crate::opus_types_h::opus_int32,
    mut tell: crate::opus_types_h::opus_int32,
    mut prob_model: *const u8,
    mut error: *mut crate::arch_h::opus_val16,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: i32,
    mut LM: i32,
    mut intra: i32,
    mut max_decay: crate::arch_h::opus_val16,
    mut lfe: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut badness: i32 = 0;
    let mut prev: [crate::arch_h::opus_val32; 2] = [0f32, 0f32];
    let mut coef: crate::arch_h::opus_val16 = 0.;
    let mut beta: crate::arch_h::opus_val16 = 0.;
    if tell + 3 <= budget {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, intra, 3u32);
    }
    if intra != 0 {
        coef = 0f32;
        beta = beta_intra
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize]
    }
    /* Encode at a fixed coarse resolution */
    i = start;
    while i < end {
        c = 0;
        loop {
            let mut bits_left: i32 = 0;
            let mut qi: i32 = 0;
            let mut qi0: i32 = 0;
            let mut q: crate::arch_h::opus_val32 = 0.;
            let mut x: crate::arch_h::opus_val16 = 0.;
            let mut f: crate::arch_h::opus_val32 = 0.;
            let mut tmp: crate::arch_h::opus_val32 = 0.;
            let mut oldE: crate::arch_h::opus_val16 = 0.;
            let mut decay_bound: crate::arch_h::opus_val16 = 0.;
            x = *eBands.offset((i + c * (*m).nbEBands) as isize);
            oldE = if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -9.0
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            };
            f = x - coef * oldE - prev[c as usize];
            /* Rounding to nearest integer here is really important! */
            qi = crate::stdlib::floor((0.5 + f) as f64) as i32;
            decay_bound = (if -28.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -28.0
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            }) - max_decay;
            /* Prevent the energy from going down too quickly (e.g. for bands
            that have just one bin) */
            if qi < 0 && x < decay_bound {
                qi += (decay_bound - x) as i32;
                if qi > 0 {
                    qi = 0
                }
            }
            qi0 = qi;
            /* If we don't have enough bits to encode all the energy, just assume
            something safe. */
            tell = ec_tell(enc);
            bits_left = budget - tell - 3 * C * (end - i);
            if i != start && bits_left < 30 {
                if bits_left < 24 {
                    qi = if (1) < qi { 1 } else { qi }
                }
                if bits_left < 16 {
                    qi = if -(1) > qi { -(1) } else { qi }
                }
            }
            if lfe != 0 && i >= 2 {
                qi = if qi < 0 { qi } else { 0 }
            }
            if budget - tell >= 15 {
                let mut pi: i32 = 0;
                pi = 2 * (if i < 20 { i } else { 20 });
                crate::src::opus_1_2_1::celt::laplace::ec_laplace_encode(
                    enc,
                    &mut qi,
                    ((*prob_model.offset(pi as isize) as i32) << 7i32) as u32,
                    (*prob_model.offset((pi + 1i32) as isize) as i32) << 6i32,
                );
            } else if budget - tell >= 2 {
                qi = if -(1) > (if qi < 1 { qi } else { 1 }) {
                    -(1)
                } else if qi < 1 {
                    qi
                } else {
                    1
                };
                crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                    enc,
                    2i32 * qi ^ -((qi < 0i32) as i32),
                    small_energy_icdf.as_ptr(),
                    2u32,
                );
            } else if budget - tell >= 1 {
                qi = if (0) < qi { 0 } else { qi };
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(enc, -qi, 1u32);
            } else {
                qi = -(1)
            }
            *error.offset((i + c * (*m).nbEBands) as isize) = f - qi as f32;
            badness += crate::stdlib::abs(qi0 - qi);
            q = qi as crate::arch_h::opus_val32;
            tmp = coef * oldE + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1
    }
    return if lfe != 0 { 0 } else { badness };
}
#[no_mangle]

pub unsafe extern "C" fn quant_coarse_energy(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut effEnd: i32,
    mut eBands: *const crate::arch_h::opus_val16,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut budget: crate::opus_types_h::opus_uint32,
    mut error: *mut crate::arch_h::opus_val16,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: i32,
    mut LM: i32,
    mut nbAvailableBytes: i32,
    mut force_intra: i32,
    mut delayedIntra: *mut crate::arch_h::opus_val32,
    mut two_pass: i32,
    mut loss_rate: i32,
    mut lfe: i32,
) {
    let mut intra: i32 = 0;
    let mut max_decay: crate::arch_h::opus_val16 = 0.;
    let mut oldEBands_intra: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut error_intra: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut enc_start_state: crate::src::opus_1_2_1::celt::entcode::ec_enc =
        crate::src::opus_1_2_1::celt::entcode::ec_enc {
            buf: 0 as *mut u8,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
    let mut tell: crate::opus_types_h::opus_uint32 = 0;
    let mut badness1: i32 = 0;
    let mut intra_bias: crate::opus_types_h::opus_int32 = 0;
    let mut new_distortion: crate::arch_h::opus_val32 = 0.;
    intra = (force_intra != 0
        || two_pass == 0
            && *delayedIntra > (2i32 * C * (end - start)) as f32
            && nbAvailableBytes > (end - start) * C) as i32;
    intra_bias = (budget as f32 * *delayedIntra * loss_rate as f32 / (C * 512i32) as f32)
        as crate::opus_types_h::opus_int32;
    new_distortion = loss_distortion(eBands, oldEBands, start, effEnd, (*m).nbEBands, C);
    tell = ec_tell(enc) as crate::opus_types_h::opus_uint32;
    if tell.wrapping_add(3u32) > budget {
        intra = 0;
        two_pass = intra
    }
    max_decay = 16.0;
    if end - start > 10 {
        max_decay = if max_decay < 0.125 * nbAvailableBytes as f32 {
            max_decay
        } else {
            (0.125) * nbAvailableBytes as f32
        }
    }
    if lfe != 0 {
        max_decay = 3.0
    }
    enc_start_state = *enc;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>())
            .wrapping_mul((C * (*m).nbEBands) as usize),
    );
    oldEBands_intra = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>())
            .wrapping_mul((C * (*m).nbEBands) as usize),
    );
    error_intra = fresh1.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memcpy(
        oldEBands_intra as *mut libc::c_void,
        oldEBands as *const libc::c_void,
        ((C * (*m).nbEBands) as usize)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
            .wrapping_add((0 * oldEBands_intra.wrapping_offset_from(oldEBands)) as usize),
    );
    if two_pass != 0 || intra != 0 {
        badness1 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands_intra,
            budget as crate::opus_types_h::opus_int32,
            tell as crate::opus_types_h::opus_int32,
            e_prob_model[LM as usize][1].as_ptr(),
            error_intra,
            enc,
            C,
            LM,
            1,
            max_decay,
            lfe,
        )
    }
    if intra == 0 {
        let mut intra_buf: *mut u8 = 0 as *mut u8;
        let mut enc_intra_state: crate::src::opus_1_2_1::celt::entcode::ec_enc =
            crate::src::opus_1_2_1::celt::entcode::ec_enc {
                buf: 0 as *mut u8,
                storage: 0,
                end_offs: 0,
                end_window: 0,
                nend_bits: 0,
                nbits_total: 0,
                offs: 0,
                rng: 0,
                val: 0,
                ext: 0,
                rem: 0,
                error: 0,
            };
        let mut tell_intra: crate::opus_types_h::opus_int32 = 0;
        let mut nstart_bytes: crate::opus_types_h::opus_uint32 = 0;
        let mut nintra_bytes: crate::opus_types_h::opus_uint32 = 0;
        let mut save_bytes: crate::opus_types_h::opus_uint32 = 0;
        let mut badness2: i32 = 0;
        let mut intra_bits: *mut u8 = 0 as *mut u8;
        tell_intra = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc)
            as crate::opus_types_h::opus_int32;
        enc_intra_state = *enc;
        nstart_bytes = ec_range_bytes(&mut enc_start_state);
        nintra_bytes = ec_range_bytes(&mut enc_intra_state);
        intra_buf = ec_get_buffer(&mut enc_intra_state).offset(nstart_bytes as isize);
        save_bytes = nintra_bytes.wrapping_sub(nstart_bytes);
        if save_bytes == 0 {
            save_bytes = 0
        }
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<u8>()).wrapping_mul(save_bytes as usize),
        );
        intra_bits = fresh2.as_mut_ptr() as *mut u8;
        /* Copy bits from intra bit-stream */
        crate::stdlib::memcpy(
            intra_bits as *mut libc::c_void,
            intra_buf as *const libc::c_void,
            (nintra_bytes.wrapping_sub(nstart_bytes) as usize)
                .wrapping_mul(::std::mem::size_of::<u8>())
                .wrapping_add((0 * intra_bits.wrapping_offset_from(intra_buf)) as usize),
        );
        *enc = enc_start_state;
        badness2 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands,
            budget as crate::opus_types_h::opus_int32,
            tell as crate::opus_types_h::opus_int32,
            e_prob_model[LM as usize][intra as usize].as_ptr(),
            error,
            enc,
            C,
            LM,
            0,
            max_decay,
            lfe,
        );
        if two_pass != 0
            && (badness1 < badness2
                || badness1 == badness2
                    && crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(enc)
                        as crate::opus_types_h::opus_int32
                        + intra_bias
                        > tell_intra)
        {
            *enc = enc_intra_state;
            /* Copy intra bits to bit-stream */
            crate::stdlib::memcpy(
                intra_buf as *mut libc::c_void,
                intra_bits as *const libc::c_void,
                (nintra_bytes.wrapping_sub(nstart_bytes) as usize)
                    .wrapping_mul(::std::mem::size_of::<u8>())
                    .wrapping_add((0 * intra_buf.wrapping_offset_from(intra_bits)) as usize),
            );
            crate::stdlib::memcpy(
                oldEBands as *mut libc::c_void,
                oldEBands_intra as *const libc::c_void,
                ((C * (*m).nbEBands) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                    .wrapping_add((0 * oldEBands.wrapping_offset_from(oldEBands_intra)) as usize),
            );
            crate::stdlib::memcpy(
                error as *mut libc::c_void,
                error_intra as *const libc::c_void,
                ((C * (*m).nbEBands) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                    .wrapping_add((0 * error.wrapping_offset_from(error_intra)) as usize),
            );
            intra = 1
        }
    } else {
        crate::stdlib::memcpy(
            oldEBands as *mut libc::c_void,
            oldEBands_intra as *const libc::c_void,
            ((C * (*m).nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0 * oldEBands.wrapping_offset_from(oldEBands_intra)) as usize),
        );
        crate::stdlib::memcpy(
            error as *mut libc::c_void,
            error_intra as *const libc::c_void,
            ((C * (*m).nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0isize * error.wrapping_offset_from(error_intra)) as usize),
        );
    }
    if intra != 0 {
        *delayedIntra = new_distortion
    } else {
        *delayedIntra =
            pred_coef[LM as usize] * pred_coef[LM as usize] * *delayedIntra + new_distortion
    };
}
#[no_mangle]

pub unsafe extern "C" fn quant_fine_energy(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut error: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut i32,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    /* Encode finer resolution */
    i = start;
    while i < end {
        let mut frac: crate::opus_types_h::opus_int16 =
            ((1i32) << *fine_quant.offset(i as isize)) as crate::opus_types_h::opus_int16;
        if !(*fine_quant.offset(i as isize) <= 0) {
            c = 0;
            loop {
                let mut q2: i32 = 0;
                let mut offset: crate::arch_h::opus_val16 = 0.;
                q2 = crate::stdlib::floor(
                    ((*error.offset((i + c * (*m).nbEBands) as isize) + 0.5) * frac as i32 as f32)
                        as f64,
                ) as i32;
                if q2 > frac as i32 - 1 {
                    q2 = frac as i32 - 1
                }
                if q2 < 0 {
                    q2 = 0
                }
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                    enc,
                    q2 as crate::opus_types_h::opus_uint32,
                    *fine_quant.offset(i as isize) as u32,
                );
                offset = (q2 as f32 + 0.5)
                    * ((1i32) << 14 - *fine_quant.offset(i as isize)) as f32
                    * (1.0 / 16384f32)
                    - 0.5;
                let ref mut fresh3 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh3 += offset;
                let ref mut fresh4 = *error.offset((i + c * (*m).nbEBands) as isize);
                *fresh4 -= offset;
                c += 1;
                if !(c < C) {
                    break;
                }
                /*printf ("%f ", error[i] - offset);*/
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn quant_energy_finalise(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut error: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut i32,
    mut fine_priority: *mut i32,
    mut bits_left: i32,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut prio: i32 = 0;
    let mut c: i32 = 0;
    /* Use up the remaining bits */
    prio = 0;
    while prio < 2 {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= 8 || *fine_priority.offset(i as isize) != prio) {
                c = 0;
                loop {
                    let mut q2: i32 = 0;
                    let mut offset: crate::arch_h::opus_val16 = 0.;
                    q2 = if *error.offset((i + c * (*m).nbEBands) as isize) < 0f32 {
                        0
                    } else {
                        1
                    };
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                        enc,
                        q2 as crate::opus_types_h::opus_uint32,
                        1,
                    );
                    offset = (q2 as f32 - 0.5)
                        * ((1i32) << 14 - *fine_quant.offset(i as isize) - 1) as f32
                        * (1.0 / 16384f32);
                    let ref mut fresh5 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh5 += offset;
                    let ref mut fresh6 = *error.offset((i + c * (*m).nbEBands) as isize);
                    *fresh6 -= offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1
        }
        prio += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn unquant_coarse_energy(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut intra: i32,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut C: i32,
    mut LM: i32,
) {
    let mut prob_model: *const u8 = e_prob_model[LM as usize][intra as usize].as_ptr();
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut prev: [crate::arch_h::opus_val32; 2] = [0f32, 0f32];
    let mut coef: crate::arch_h::opus_val16 = 0.;
    let mut beta: crate::arch_h::opus_val16 = 0.;
    let mut budget: crate::opus_types_h::opus_int32 = 0;
    let mut tell: crate::opus_types_h::opus_int32 = 0;
    if intra != 0 {
        coef = 0f32;
        beta = beta_intra
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize]
    }
    budget = (*dec).storage.wrapping_mul(8u32) as crate::opus_types_h::opus_int32;
    /* Decode at a fixed coarse resolution */
    i = start;
    while i < end {
        c = 0;
        loop {
            let mut qi: i32 = 0;
            let mut q: crate::arch_h::opus_val32 = 0.;
            let mut tmp: crate::arch_h::opus_val32 = 0.;
            /* It would be better to express this invariant as a
            test on C at function entry, but that isn't enough
            to make the static analyzer happy. */
            tell = ec_tell(dec);
            if budget - tell >= 15 {
                let mut pi: i32 = 0;
                pi = 2 * (if i < 20 { i } else { 20 });
                qi = crate::src::opus_1_2_1::celt::laplace::ec_laplace_decode(
                    dec,
                    ((*prob_model.offset(pi as isize) as i32) << 7) as u32,
                    (*prob_model.offset((pi + 1) as isize) as i32) << 6,
                )
            } else if budget - tell >= 2 {
                qi = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                    dec,
                    small_energy_icdf.as_ptr(),
                    2,
                );
                qi = qi >> 1 ^ -(qi & 1)
            } else if budget - tell >= 1 {
                qi = -crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, 1)
            } else {
                qi = -(1)
            }
            q = qi as crate::arch_h::opus_val32;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) =
                if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                    -9.0f32
                } else {
                    *oldEBands.offset((i + c * (*m).nbEBands) as isize)
                };
            tmp = coef * *oldEBands.offset((i + c * (*m).nbEBands) as isize) + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn unquant_fine_energy(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut i32,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    /* Decode finer resolution */
    i = start;
    while i < end {
        if !(*fine_quant.offset(i as isize) <= 0) {
            c = 0;
            loop {
                let mut q2: i32 = 0;
                let mut offset: crate::arch_h::opus_val16 = 0.;
                q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                    dec,
                    *fine_quant.offset(i as isize) as u32,
                ) as i32;
                offset = (q2 as f32 + 0.5)
                    * ((1i32) << 14 - *fine_quant.offset(i as isize)) as f32
                    * (1.0 / 16384f32)
                    - 0.5;
                let ref mut fresh7 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh7 += offset;
                c += 1;
                if !(c < C) {
                    break;
                }
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn unquant_energy_finalise(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut i32,
    mut fine_priority: *mut i32,
    mut bits_left: i32,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut prio: i32 = 0;
    let mut c: i32 = 0;
    /* Use up the remaining bits */
    prio = 0;
    while prio < 2 {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= 8 || *fine_priority.offset(i as isize) != prio) {
                c = 0;
                loop {
                    let mut q2: i32 = 0;
                    let mut offset: crate::arch_h::opus_val16 = 0.;
                    q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(dec, 1) as i32;
                    offset = (q2 as f32 - 0.5)
                        * ((1i32) << 14 - *fine_quant.offset(i as isize) - 1) as f32
                        * (1.0 / 16384f32);
                    let ref mut fresh8 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh8 += offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1
        }
        prio += 1
    }
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
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

pub unsafe extern "C" fn amp2Log2(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut effEnd: i32,
    mut end: i32,
    mut bandE: *mut crate::arch_h::celt_ener,
    mut bandLogE: *mut crate::arch_h::opus_val16,
    mut C: i32,
) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    c = 0;
    loop {
        i = 0;
        while i < effEnd {
            *bandLogE.offset((i + c * (*m).nbEBands) as isize) =
                celt_log2(*bandE.offset((i + c * (*m).nbEBands) as isize)) - eMeans[i as usize];
            i += 1
        }
        i = effEnd;
        while i < end {
            *bandLogE.offset((c * (*m).nbEBands + i) as isize) = -14.0f32;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
