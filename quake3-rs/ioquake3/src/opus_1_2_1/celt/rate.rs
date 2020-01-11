pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    /* Tested exhaustively for all n and for 1<=d<=256 */
    #[inline]

    pub unsafe extern "C" fn celt_udiv(
        mut n: crate::opus_types_h::opus_uint32,
        mut d: crate::opus_types_h::opus_uint32,
    ) -> crate::opus_types_h::opus_uint32 {
        return n.wrapping_div(d);
    }
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::opus_val16;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

pub use crate::src::opus_1_2_1::celt::rate::entcode_h::celt_udiv;
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

static mut LOG2_FRAC_TABLE: [u8; 24] = [
    0, 8, 13, 16, 19, 21, 23, 24, 26, 27, 28, 29, 30, 31, 32, 32, 33, 34, 34, 35, 36, 36, 37, 37,
];
#[inline]

unsafe extern "C" fn interp_bits2pulses(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut skip_start: i32,
    mut bits1: *const i32,
    mut bits2: *const i32,
    mut thresh: *const i32,
    mut cap: *const i32,
    mut total: crate::opus_types_h::opus_int32,
    mut _balance: *mut crate::opus_types_h::opus_int32,
    mut skip_rsv: i32,
    mut intensity: *mut i32,
    mut intensity_rsv: i32,
    mut dual_stereo: *mut i32,
    mut dual_stereo_rsv: i32,
    mut bits: *mut i32,
    mut ebits: *mut i32,
    mut fine_priority: *mut i32,
    mut C: i32,
    mut LM: i32,
    mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    mut encode: i32,
    mut prev: i32,
    mut signalBandwidth: i32,
) -> i32 {
    let mut psum: crate::opus_types_h::opus_int32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut _i: i32 = 0;
    let mut j: i32 = 0;
    let mut logM: i32 = 0;
    let mut stereo: i32 = 0;
    let mut codedBands: i32 = -(1);
    let mut alloc_floor: i32 = 0;
    let mut left: crate::opus_types_h::opus_int32 = 0;
    let mut percoeff: crate::opus_types_h::opus_int32 = 0;
    let mut done: i32 = 0;
    let mut balance: crate::opus_types_h::opus_int32 = 0;
    alloc_floor = C << 3;
    stereo = (C > 1) as i32;
    logM = LM << 3;
    lo = 0;
    hi = (1) << 6;

    for _i in 0..6 {
        let mut mid: i32 = lo + hi >> 1;

        psum = 0;

        done = 0;

        j = end;

        loop {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 > start) {
                break;
            }
            let mut tmp: i32 = *bits1.offset(j as isize) + (mid * *bits2.offset(j as isize) >> 6);
            if tmp >= *thresh.offset(j as isize) || done != 0 {
                done = 1;
                /* Don't allocate more than we can actually use */
                psum += if tmp < *cap.offset(j as isize) {
                    tmp
                } else {
                    *cap.offset(j as isize)
                }
            } else if tmp >= alloc_floor {
                psum += alloc_floor
            }
        }

        if psum > total {
            hi = mid
        } else {
            lo = mid
        }
    }
    psum = 0;
    /*printf ("interp bisection gave %d\n", lo);*/
    done = 0;
    j = end;
    loop {
        let fresh1 = j;
        j = j - 1;
        if !(fresh1 > start) {
            break;
        }
        let mut tmp_0: i32 = *bits1.offset(j as isize) + (lo * *bits2.offset(j as isize) >> 6);
        if tmp_0 < *thresh.offset(j as isize) && done == 0 {
            if tmp_0 >= alloc_floor {
                tmp_0 = alloc_floor
            } else {
                tmp_0 = 0
            }
        } else {
            done = 1
        }
        /* Don't allocate more than we can actually use */
        tmp_0 = if tmp_0 < *cap.offset(j as isize) {
            tmp_0
        } else {
            *cap.offset(j as isize)
        };
        *bits.offset(j as isize) = tmp_0;
        psum += tmp_0
    }
    /* Decide which bands to skip, working backwards from the end. */
    codedBands = end;
    loop {
        let mut band_width: i32 = 0;
        let mut band_bits: i32 = 0;
        let mut rem: i32 = 0;
        j = codedBands - 1;
        /* Never skip the first band, nor a band that has been boosted by
         dynalloc.
        In the first case, we'd be coding a bit to signal we're going to waste
         all the other bits.
        In the second case, we'd be coding a bit to redistribute all the bits
         we just signaled should be cocentrated in this band. */
        if j <= skip_start {
            /* Give the bit we reserved to end skipping back. */
            total += skip_rsv;
            break;
        } else {
            /*Figure out how many left-over bits we would be adding to this band.
            This can include bits we've stolen back from higher, skipped bands.*/
            left = total - psum;
            percoeff = celt_udiv(
                left as crate::opus_types_h::opus_uint32,
                (*(*m).eBands.offset(codedBands as isize) as i32
                    - *(*m).eBands.offset(start as isize) as i32)
                    as crate::opus_types_h::opus_uint32,
            ) as crate::opus_types_h::opus_int32;
            left -= (*(*m).eBands.offset(codedBands as isize) as i32
                - *(*m).eBands.offset(start as isize) as i32)
                * percoeff;
            rem = if left
                - (*(*m).eBands.offset(j as isize) as i32
                    - *(*m).eBands.offset(start as isize) as i32)
                > 0
            {
                (left)
                    - (*(*m).eBands.offset(j as isize) as i32
                        - *(*m).eBands.offset(start as isize) as i32)
            } else {
                0
            };
            band_width = *(*m).eBands.offset(codedBands as isize) as i32
                - *(*m).eBands.offset(j as isize) as i32;
            band_bits = *bits.offset(j as isize) + percoeff * band_width + rem;
            /*Only code a skip decision if we're above the threshold for this band.
            Otherwise it is force-skipped.
            This ensures that we have enough bits to code the skip flag.*/
            if band_bits
                >= (if *thresh.offset(j as isize) > alloc_floor + ((1) << 3) {
                    *thresh.offset(j as isize)
                } else {
                    (alloc_floor) + ((1) << 3)
                })
            {
                if encode != 0 {
                    /*This if() block is the only part of the allocation function that
                    is not a mandatory part of the bitstream: any bands we choose to
                    skip here must be explicitly signaled.*/
                    let mut depth_threshold: i32 = 0;
                    /*We choose a threshold with some hysteresis to keep bands from
                    fluctuating in and out, but we try not to fold below a certain point. */
                    if codedBands > 17 {
                        depth_threshold = if j < prev { 7 } else { 9 }
                    } else {
                        depth_threshold = 0
                    }
                    if codedBands <= start + 2
                        || band_bits > depth_threshold * band_width << LM << 3 >> 4
                            && j <= signalBandwidth
                    {
                        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(ec, 1, 1);
                        break;
                    } else {
                        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(ec, 0i32, 1u32);
                    }
                } else if crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(ec, 1) != 0 {
                    break;
                }
                /*We used a bit to skip this band.*/
                psum += (1) << 3;
                band_bits -= (1) << 3
            }
            /*Reclaim the bits originally allocated to this band.*/
            psum -= *bits.offset(j as isize) + intensity_rsv;
            if intensity_rsv > 0 {
                intensity_rsv = LOG2_FRAC_TABLE[(j - start) as usize] as i32
            }
            psum += intensity_rsv;
            if band_bits >= alloc_floor {
                /*If we have enough for a fine energy bit per channel, use it.*/
                psum += alloc_floor;
                *bits.offset(j as isize) = alloc_floor
            } else {
                /*Otherwise this band gets nothing at all.*/
                *bits.offset(j as isize) = 0
            }
            codedBands -= 1
        }
    }
    /* Code the intensity and dual stereo parameters. */
    if intensity_rsv > 0 {
        if encode != 0 {
            *intensity = if *intensity < codedBands {
                *intensity
            } else {
                codedBands
            };
            crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
                ec,
                (*intensity - start) as crate::opus_types_h::opus_uint32,
                (codedBands + 1i32 - start) as crate::opus_types_h::opus_uint32,
            );
        } else {
            *intensity =
                (start as u32).wrapping_add(crate::src::opus_1_2_1::celt::entdec::ec_dec_uint(
                    ec,
                    (codedBands + 1 - start) as crate::opus_types_h::opus_uint32,
                )) as i32
        }
    } else {
        *intensity = 0
    }
    if *intensity <= start {
        total += dual_stereo_rsv;
        dual_stereo_rsv = 0
    }
    if dual_stereo_rsv > 0 {
        if encode != 0 {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(ec, *dual_stereo, 1u32);
        } else {
            *dual_stereo = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(ec, 1)
        }
    } else {
        *dual_stereo = 0
    }
    /* Allocate the remaining bits */
    left = total - psum;
    percoeff = celt_udiv(
        left as crate::opus_types_h::opus_uint32,
        (*(*m).eBands.offset(codedBands as isize) as i32
            - *(*m).eBands.offset(start as isize) as i32)
            as crate::opus_types_h::opus_uint32,
    ) as crate::opus_types_h::opus_int32;
    left -= (*(*m).eBands.offset(codedBands as isize) as i32
        - *(*m).eBands.offset(start as isize) as i32)
        * percoeff;
    j = start;
    while j < codedBands {
        *bits.offset(j as isize) += percoeff
            * (*(*m).eBands.offset((j + 1) as isize) as i32
                - *(*m).eBands.offset(j as isize) as i32);
        j += 1
    }
    j = start;
    while j < codedBands {
        let mut tmp_1: i32 = if left
            < *(*m).eBands.offset((j + 1) as isize) as i32 - *(*m).eBands.offset(j as isize) as i32
        {
            left
        } else {
            (*(*m).eBands.offset((j + 1) as isize) as i32) - *(*m).eBands.offset(j as isize) as i32
        };
        *bits.offset(j as isize) += tmp_1;
        left -= tmp_1;
        j += 1
    }
    /*for (j=0;j<end;j++)printf("%d ", bits[j]);printf("\n");*/
    balance = 0;
    j = start;
    while j < codedBands {
        let mut N0: i32 = 0;
        let mut N: i32 = 0;
        let mut den: i32 = 0;
        let mut offset: i32 = 0;
        let mut NClogN: i32 = 0;
        let mut excess: crate::opus_types_h::opus_int32 = 0;
        let mut bit: crate::opus_types_h::opus_int32 = 0;
        N0 = *(*m).eBands.offset((j + 1) as isize) as i32 - *(*m).eBands.offset(j as isize) as i32;
        N = N0 << LM;
        bit = *bits.offset(j as isize) + balance;
        if N > 1 {
            excess = if bit - *cap.offset(j as isize) > 0 {
                (bit) - *cap.offset(j as isize)
            } else {
                0
            };
            *bits.offset(j as isize) = bit - excess;
            /* Compensate for the extra DoF in stereo */
            den = C * N
                + (if C == 2 && N > 2 && *dual_stereo == 0 && j < *intensity {
                    1
                } else {
                    0
                });
            NClogN = den * (*(*m).logN.offset(j as isize) as i32 + logM);
            /* Offset for the number of fine bits by log2(N)/2 + FINE_OFFSET
            compared to their "fair share" of total/N */
            offset = (NClogN >> 1) - den * 21;
            /* N=2 is the only point that doesn't match the curve */
            if N == 2 {
                offset += den << 3 >> 2
            }
            /* Changing the offset for allocating the second and third
            fine energy bit */
            if *bits.offset(j as isize) + offset < (den * 2) << 3 {
                offset += NClogN >> 2
            } else if *bits.offset(j as isize) + offset < (den * 3) << 3 {
                offset += NClogN >> 3
            }
            /* Divide with rounding */
            *ebits.offset(j as isize) = if 0 > *bits.offset(j as isize) + offset + (den << 3 - 1) {
                0
            } else {
                (*bits.offset(j as isize) + offset) + (den << 3 - 1)
            };
            *ebits.offset(j as isize) = (celt_udiv(
                *ebits.offset(j as isize) as crate::opus_types_h::opus_uint32,
                den as crate::opus_types_h::opus_uint32,
            ) >> 3) as i32;
            /* Make sure not to bust */
            if C * *ebits.offset(j as isize) > *bits.offset(j as isize) >> 3 {
                *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> 3
            }
            /* More than that is useless because that's about as far as PVQ can go */
            *ebits.offset(j as isize) = if *ebits.offset(j as isize) < 8 {
                *ebits.offset(j as isize)
            } else {
                8
            };
            /* If we rounded down or capped this band, make it a candidate for the
            final fine energy pass */
            *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) * (den << 3)
                >= *bits.offset(j as isize) + offset)
                as i32;
            /* Remove the allocated fine bits; the rest are assigned to PVQ */
            *bits.offset(j as isize) -= C * *ebits.offset(j as isize) << 3
        } else {
            /* For N=1, all bits go to fine energy except for a single sign bit */
            excess = if 0 > bit - (C << 3) {
                0
            } else {
                (bit) - (C << 3)
            };
            *bits.offset(j as isize) = bit - excess;
            *ebits.offset(j as isize) = 0;
            *fine_priority.offset(j as isize) = 1
        }
        /* Fine energy can't take advantage of the re-balancing in
         quant_all_bands().
        Instead, do the re-balancing here.*/
        if excess > 0 {
            let mut extra_fine: i32 = 0;
            let mut extra_bits: i32 = 0;
            extra_fine = if (excess >> stereo + 3) < 8 - *ebits.offset(j as isize) {
                (excess) >> stereo + 3
            } else {
                (8) - *ebits.offset(j as isize)
            };
            *ebits.offset(j as isize) += extra_fine;
            extra_bits = extra_fine * C << 3;
            *fine_priority.offset(j as isize) = (extra_bits >= excess - balance) as i32;
            excess -= extra_bits
        }
        balance = excess;
        j += 1
    }
    /* Save any remaining bits over the cap for the rebalancing in
    quant_all_bands(). */
    *_balance = balance;
    /* The skipped bands use all their bits for fine energy. */
    while j < end {
        *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> 3;
        *bits.offset(j as isize) = 0;
        *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) < 1) as i32;
        j += 1
    }
    return codedBands;
}
/* * Compute the pulse allocation, i.e. how many pulses will go in each
  * band.
 @param m mode
 @param offsets Requested increase or decrease in the number of bits for
                each band
 @param total Number of bands
 @param pulses Number of pulses per band (returned)
 @return Total number of bits allocated
*/
#[no_mangle]

pub unsafe extern "C" fn compute_allocation(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut offsets: *const i32,
    mut cap: *const i32,
    mut alloc_trim: i32,
    mut intensity: *mut i32,
    mut dual_stereo: *mut i32,
    mut total: crate::opus_types_h::opus_int32,
    mut balance: *mut crate::opus_types_h::opus_int32,
    mut pulses: *mut i32,
    mut ebits: *mut i32,
    mut fine_priority: *mut i32,
    mut C: i32,
    mut LM: i32,
    mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    mut encode: i32,
    mut prev: i32,
    mut signalBandwidth: i32,
) -> i32 {
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut len: i32 = 0;
    let mut j: i32 = 0;
    let mut codedBands: i32 = 0;
    let mut skip_start: i32 = 0;
    let mut skip_rsv: i32 = 0;
    let mut intensity_rsv: i32 = 0;
    let mut dual_stereo_rsv: i32 = 0;
    let mut bits1: *mut i32 = 0 as *mut i32;
    let mut bits2: *mut i32 = 0 as *mut i32;
    let mut thresh: *mut i32 = 0 as *mut i32;
    let mut trim_offset: *mut i32 = 0 as *mut i32;
    total = if total > 0 { total } else { 0 };
    len = (*m).nbEBands;
    skip_start = start;
    /* Reserve a bit to signal the end of manually skipped bands. */
    skip_rsv = if total >= (1) << 3 { (1) << 3 } else { 0 };
    total -= skip_rsv;
    /* Reserve bits for the intensity and dual stereo parameters. */
    dual_stereo_rsv = 0;
    intensity_rsv = dual_stereo_rsv;
    if C == 2 {
        intensity_rsv = LOG2_FRAC_TABLE[(end - start) as usize] as i32;
        if intensity_rsv > total {
            intensity_rsv = 0
        } else {
            total -= intensity_rsv;
            dual_stereo_rsv = if total >= (1) << 3 { (1) << 3 } else { 0 };
            total -= dual_stereo_rsv
        }
    }
    let mut fresh2 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    bits1 = fresh2.as_mut_ptr();
    let mut fresh3 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    bits2 = fresh3.as_mut_ptr();
    let mut fresh4 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    thresh = fresh4.as_mut_ptr();
    let mut fresh5 =
        ::std::vec::from_elem(0, (::std::mem::size_of::<i32>()).wrapping_mul(len as usize));
    trim_offset = fresh5.as_mut_ptr();
    j = start;
    while j < end {
        /* Below this threshold, we're sure not to allocate any PVQ bits */
        *thresh.offset(j as isize) = if C << 3
            > (3 * (*(*m).eBands.offset((j + 1) as isize) as i32
                - *(*m).eBands.offset(j as isize) as i32))
                << LM
                << 3
                >> 4
        {
            (C) << 3
        } else {
            ((3 * (*(*m).eBands.offset((j + 1) as isize) as i32
                - *(*m).eBands.offset(j as isize) as i32))
                << LM
                << 3)
                >> 4
        };
        /* Tilt of the allocation curve */
        *trim_offset.offset(j as isize) = C
            * (*(*m).eBands.offset((j + 1) as isize) as i32
                - *(*m).eBands.offset(j as isize) as i32)
            * (alloc_trim - 5 - LM)
            * (end - j - 1)
            * ((1) << LM + 3)
            >> 6;
        /* Giving less resolution to single-coefficient bands because they get
        more benefit from having one coarse value per coefficient*/
        if (*(*m).eBands.offset((j + 1) as isize) as i32 - *(*m).eBands.offset(j as isize) as i32)
            << LM
            == 1
        {
            *trim_offset.offset(j as isize) -= C << 3
        }
        j += 1
    }
    lo = 1;
    hi = (*m).nbAllocVectors - 1;
    loop {
        let mut done: i32 = 0;
        let mut psum: i32 = 0;
        let mut mid: i32 = lo + hi >> 1;
        j = end;
        loop {
            let fresh6 = j;
            j = j - 1;
            if !(fresh6 > start) {
                break;
            }
            let mut bitsj: i32 = 0;
            let mut N: i32 = *(*m).eBands.offset((j + 1) as isize) as i32
                - *(*m).eBands.offset(j as isize) as i32;
            bitsj = (C * N * *(*m).allocVectors.offset((mid * len + j) as isize) as i32) << LM >> 2;
            if bitsj > 0 {
                bitsj = if 0 > bitsj + *trim_offset.offset(j as isize) {
                    0
                } else {
                    (bitsj) + *trim_offset.offset(j as isize)
                }
            }
            bitsj += *offsets.offset(j as isize);
            if bitsj >= *thresh.offset(j as isize) || done != 0 {
                done = 1;
                /*printf ("lo = %d, hi = %d\n", lo, hi);*/
                /* Don't allocate more than we can actually use */
                psum += if bitsj < *cap.offset(j as isize) {
                    bitsj
                } else {
                    *cap.offset(j as isize)
                }
            } else if bitsj >= C << 3 {
                psum += C << 3
            }
        }
        if psum > total {
            hi = mid - 1
        } else {
            lo = mid + 1
        }
        if !(lo <= hi) {
            break;
        }
    }
    let fresh7 = lo;
    lo = lo - 1;
    hi = fresh7;
    /*printf ("interp between %d and %d\n", lo, hi);*/
    j = start;
    while j < end {
        let mut bits1j: i32 = 0;
        let mut bits2j: i32 = 0;
        let mut N_0: i32 =
            *(*m).eBands.offset((j + 1) as isize) as i32 - *(*m).eBands.offset(j as isize) as i32;
        bits1j = (C * N_0 * *(*m).allocVectors.offset((lo * len + j) as isize) as i32) << LM >> 2;
        bits2j = if hi >= (*m).nbAllocVectors {
            *cap.offset(j as isize)
        } else {
            ((C * N_0 * *(*m).allocVectors.offset((hi * len + j) as isize) as i32) << LM) >> 2
        };
        if bits1j > 0 {
            bits1j = if 0 > bits1j + *trim_offset.offset(j as isize) {
                0
            } else {
                (bits1j) + *trim_offset.offset(j as isize)
            }
        }
        if bits2j > 0 {
            bits2j = if 0 > bits2j + *trim_offset.offset(j as isize) {
                0
            } else {
                (bits2j) + *trim_offset.offset(j as isize)
            }
        }
        if lo > 0 {
            bits1j += *offsets.offset(j as isize)
        }
        bits2j += *offsets.offset(j as isize);
        if *offsets.offset(j as isize) > 0 {
            skip_start = j
        }
        bits2j = if 0 > bits2j - bits1j {
            0
        } else {
            (bits2j) - bits1j
        };
        *bits1.offset(j as isize) = bits1j;
        *bits2.offset(j as isize) = bits2j;
        j += 1
    }
    codedBands = interp_bits2pulses(
        m,
        start,
        end,
        skip_start,
        bits1,
        bits2,
        thresh,
        cap,
        total,
        balance,
        skip_rsv,
        intensity,
        intensity_rsv,
        dual_stereo,
        dual_stereo_rsv,
        pulses,
        ebits,
        fine_priority,
        C,
        LM,
        ec,
        encode,
        prev,
        signalBandwidth,
    );
    return codedBands;
}
