use ::libc;

pub mod entcode_h {

    /* Copyright (c) 2001-2011 Timothy B. Terriberry
    Copyright (c) 2008-2009 Xiph.Org Foundation */
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
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/
    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/
    /*Buffered input/output.*/
    /*The size of the buffer.*/
    /*The offset at which the last byte containing raw bits was read/written.*/
    /*Bits that will be read from/written at the end.*/
    /*Number of valid bits in end_window.*/
    /*The total number of whole bits read/written.
    This does not include partial bits currently in the range coder.*/
    /*The offset at which the next range coder byte will be read/written.*/
    /*The number of values in the current range.*/
    /*In the decoder: the difference between the top of the current range and
     the input value, minus one.
    In the encoder: the low end of the current range.*/
    /*In the decoder: the saved normalization factor from ec_decode().
    In the encoder: the number of oustanding carry propagating symbols.*/
    /*A buffered input/output symbol, awaiting carry propagation.*/
    /*Nonzero if an error occurred.*/
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
    #[inline]

    pub unsafe extern "C" fn ec_get_error(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> i32 {
        return (*_this).error;
    }
}

pub mod cpu_support_h {
    /* Copyright (c) 2010 Xiph.Org Foundation
     * Copyright (c) 2013 Parrot */
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
    #[inline]

    pub unsafe extern "C" fn opus_select_arch() -> i32 {
        return 0;
    }
}

pub mod celt_h {

    pub static mut tapset_icdf: [u8; 3] = [2, 1, 0];

    pub static mut spread_icdf: [u8; 4] = [25, 23, 2, 0];

    pub static mut trim_icdf: [u8; 11] = [126, 124, 119, 109, 87, 41, 19, 9, 4, 2, 0];

    /* CELT_H */
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdarg_h::va_list;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::celt_norm;
pub use crate::arch_h::celt_sig;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::src::opus_1_2_1::celt::celt_decoder::cpu_support_h::opus_select_arch;
pub use crate::src::opus_1_2_1::celt::celt_decoder::entcode_h::ec_get_error;
pub use crate::src::opus_1_2_1::celt::celt_decoder::entcode_h::ec_tell;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_tell_frac;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::clt_mdct_backward_c;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;

pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::src::opus_1_2_1::celt::celt::comb_filter;
pub use crate::src::opus_1_2_1::celt::celt::init_caps;
pub use crate::src::opus_1_2_1::celt::celt::resampling_factor;
pub use crate::src::opus_1_2_1::celt::celt::tf_select_table;
pub use crate::src::opus_1_2_1::celt::celt_decoder::celt_h::spread_icdf;
pub use crate::src::opus_1_2_1::celt::celt_decoder::celt_h::tapset_icdf;
pub use crate::src::opus_1_2_1::celt::celt_decoder::celt_h::trim_icdf;

/* * Decoder state
@brief Decoder state
*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusCustomDecoder {
    pub mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    pub overlap: i32,
    pub channels: i32,
    pub stream_channels: i32,
    pub downsample: i32,
    pub start: i32,
    pub end: i32,
    pub signalling: i32,
    pub disable_inv: i32,
    pub arch: i32,
    pub rng: crate::opus_types_h::opus_uint32,
    pub error: i32,
    pub last_pitch_index: i32,
    pub loss_count: i32,
    pub skip_plc: i32,
    pub postfilter_period: i32,
    pub postfilter_period_old: i32,
    pub postfilter_gain: crate::arch_h::opus_val16,
    pub postfilter_gain_old: crate::arch_h::opus_val16,
    pub postfilter_tapset: i32,
    pub postfilter_tapset_old: i32,
    pub preemph_memD: [crate::arch_h::celt_sig; 2],
    pub _decode_mem: [crate::arch_h::celt_sig; 1],
}
/* Size = channels*(DECODE_BUFFER_SIZE+mode->overlap) */
/* opus_val16 lpc[],  Size = channels*LPC_ORDER */
/* opus_val16 oldEBands[], Size = 2*mode->nbEBands */
/* opus_val16 oldLogE[], Size = 2*mode->nbEBands */
/* opus_val16 oldLogE2[], Size = 2*mode->nbEBands */
/* opus_val16 backgroundLogE[], Size = 2*mode->nbEBands */
#[no_mangle]

pub unsafe extern "C" fn celt_decoder_get_size(mut channels: i32) -> i32 {
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        crate::src::opus_1_2_1::celt::modes::opus_custom_mode_create(48000, 960, 0 as *mut i32);
    return opus_custom_decoder_get_size(mode, channels);
}
#[inline]

unsafe extern "C" fn opus_custom_decoder_get_size(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut channels: i32,
) -> i32 {
    let mut size: i32 = (::std::mem::size_of::<OpusCustomDecoder>())
        .wrapping_add(
            ((channels * (2048i32 + (*mode).overlap) - 1) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>()),
        )
        .wrapping_add(
            ((channels * 24i32) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
        )
        .wrapping_add(
            ((4i32 * 2 * (*mode).nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>()),
        ) as i32;
    return size;
}
/* CUSTOM_MODES */
#[no_mangle]

pub unsafe extern "C" fn celt_decoder_init(
    mut st: *mut OpusCustomDecoder,
    mut sampling_rate: crate::opus_types_h::opus_int32,
    mut channels: i32,
) -> i32 {
    let mut ret: i32 = 0;
    ret = opus_custom_decoder_init(
        st,
        crate::src::opus_1_2_1::celt::modes::opus_custom_mode_create(48000, 960, 0 as *mut i32),
        channels,
    );
    if ret != 0 {
        return ret;
    }
    (*st).downsample = crate::src::opus_1_2_1::celt::celt::resampling_factor(sampling_rate);
    if (*st).downsample == 0 {
        return -(1i32);
    } else {
        return 0i32;
    };
}
/* Decoder */
/* * Gets the size of an OpusCustomDecoder structure.
 * @param [in] mode <tt>OpusCustomMode *</tt>: Mode configuration
 * @param [in] channels <tt>int</tt>: Number of channels
 * @returns size
 */
/* * Initializes a previously allocated decoder state
 * The memory pointed to by st must be the size returned by opus_custom_decoder_get_size.
 * This is intended for applications which use their own allocator instead of malloc.
 * @see opus_custom_decoder_create(),opus_custom_decoder_get_size()
 * To reset a previously initialized state use the OPUS_RESET_STATE CTL.
 * @param [in] st <tt>OpusCustomDecoder*</tt>: Decoder state
 * @param [in] mode <tt>OpusCustomMode *</tt>: Contains all the information about the characteristics of
 *  the stream (must be the same characteristics as used for the
 *  encoder)
 * @param [in] channels <tt>int</tt>: Number of channels
 * @return OPUS_OK Success or @ref opus_errorcodes
 */
#[inline]

unsafe extern "C" fn opus_custom_decoder_init(
    mut st: *mut OpusCustomDecoder,
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut channels: i32,
) -> i32 {
    if channels < 0 || channels > 2 {
        return -(1i32);
    }
    if st.is_null() {
        return -(7i32);
    }
    crate::stdlib::memset(
        st as *mut libc::c_void,
        0,
        (opus_custom_decoder_get_size(mode, channels) as usize)
            .wrapping_mul(::std::mem::size_of::<i8>()),
    );
    (*st).mode = mode;
    (*st).overlap = (*mode).overlap;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).downsample = 1;
    (*st).start = 0;
    (*st).end = (*(*st).mode).effEBands;
    (*st).signalling = 1;
    (*st).disable_inv = 0;
    (*st).arch = opus_select_arch();
    opus_custom_decoder_ctl(st, 4028);
    return 0;
}
/* CUSTOM_MODES */
/* Special case for stereo with no downsampling and no accumulation. This is
quite common and we can make it faster by processing both channels in the
same loop, reducing overhead due to the dependency loop in the IIR filter. */

unsafe extern "C" fn deemphasis_stereo_simple(
    mut in_0: *mut *mut crate::arch_h::celt_sig,
    mut pcm: *mut crate::arch_h::opus_val16,
    mut N: i32,
    coef0: crate::arch_h::opus_val16,
    mut mem: *mut crate::arch_h::celt_sig,
) {
    let mut x0: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut x1: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut m0: crate::arch_h::celt_sig = 0.;
    let mut m1: crate::arch_h::celt_sig = 0.;
    let mut _j: i32 = 0;
    x0 = *in_0.offset(0);
    x1 = *in_0.offset(1);
    m0 = *mem.offset(0);
    m1 = *mem.offset(1);

    for j in 0..N {
        let mut tmp0: crate::arch_h::celt_sig = 0.;

        let mut tmp1: crate::arch_h::celt_sig = 0.;

        tmp0 = *x0.offset(j as isize) + 1e-30 + m0;

        tmp1 = *x1.offset(j as isize) + 1e-30 + m1;

        m0 = coef0 * tmp0;

        m1 = coef0 * tmp1;

        *pcm.offset((2 * j) as isize) = tmp0 * (1f32 / 32768.0);

        *pcm.offset((2 * j + 1) as isize) = tmp1 * (1f32 / 32768.0);
    }
    *mem.offset(0) = m0;
    *mem.offset(1) = m1;
}

unsafe extern "C" fn deemphasis(
    mut in_0: *mut *mut crate::arch_h::celt_sig,
    mut pcm: *mut crate::arch_h::opus_val16,
    mut N: i32,
    mut C: i32,
    mut downsample: i32,
    mut coef: *const crate::arch_h::opus_val16,
    mut mem: *mut crate::arch_h::celt_sig,
    mut accum: i32,
) {
    let mut c: i32 = 0;
    let mut Nd: i32 = 0;
    let mut apply_downsampling: i32 = 0;
    let mut coef0: crate::arch_h::opus_val16 = 0.;
    let mut scratch: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    /* Short version for common case. */
    if downsample == 1 && C == 2 && accum == 0 {
        deemphasis_stereo_simple(in_0, pcm, N, *coef.offset(0), mem);
        return;
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>()).wrapping_mul(N as usize),
    );
    scratch = fresh0.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    coef0 = *coef.offset(0);
    Nd = N / downsample;
    c = 0;
    loop {
        let mut j: i32 = 0;
        let mut x: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
        let mut y: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
        let mut m: crate::arch_h::celt_sig = *mem.offset(c as isize);
        x = *in_0.offset(c as isize);
        y = pcm.offset(c as isize);
        if downsample > 1 {
            /* Shortcut for the standard (non-custom modes) case */
            j = 0;
            while j < N {
                let mut tmp: crate::arch_h::celt_sig = *x.offset(j as isize) + 1e-30 + m;
                m = coef0 * tmp;
                *scratch.offset(j as isize) = tmp;
                j += 1
            }
            apply_downsampling = 1
        } else {
            /* Shortcut for the standard (non-custom modes) case */
            j = 0;
            while j < N {
                let mut tmp_0: crate::arch_h::celt_sig = *x.offset(j as isize) + 1e-30 + m;
                m = coef0 * tmp_0;
                *y.offset((j * C) as isize) = tmp_0 * (1f32 / 32768.0);
                j += 1
            }
        }
        *mem.offset(c as isize) = m;
        if apply_downsampling != 0 {
            /* Perform down-sampling */
            j = 0; /* *< Interleaved signal MDCTs */
            while j < Nd {
                *y.offset((j * C) as isize) =
                    *scratch.offset((j * downsample) as isize) * (1f32 / 32768.0);
                j += 1
            }
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}

unsafe extern "C" fn celt_synthesis(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *mut crate::arch_h::celt_norm,
    mut out_syn: *mut *mut crate::arch_h::celt_sig,
    mut oldBandE: *mut crate::arch_h::opus_val16,
    mut start: i32,
    mut effEnd: i32,
    mut C: i32,
    mut CC: i32,
    mut isTransient: i32,
    mut LM: i32,
    mut downsample: i32,
    mut silence: i32,
    mut arch: i32,
) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut M: i32 = 0;
    let mut b: i32 = 0;
    let mut B: i32 = 0;
    let mut N: i32 = 0;
    let mut NB: i32 = 0;
    let mut shift: i32 = 0;
    let mut nbEBands: i32 = 0;
    let mut overlap: i32 = 0;
    let mut freq: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    overlap = (*mode).overlap;
    nbEBands = (*mode).nbEBands;
    N = (*mode).shortMdctSize << LM;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>()).wrapping_mul(N as usize),
    );
    freq = fresh1.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    M = (1) << LM;
    if isTransient != 0 {
        B = M;
        NB = (*mode).shortMdctSize;
        shift = (*mode).maxLM
    } else {
        B = 1;
        NB = (*mode).shortMdctSize << LM;
        shift = (*mode).maxLM - LM
    }
    if CC == 2 && C == 1 {
        /* Copying a mono streams to two channels */
        let mut freq2: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
        crate::src::opus_1_2_1::celt::bands::denormalise_bands(
            mode, X, freq, oldBandE, start, effEnd, M, downsample, silence,
        );
        /* Store a temporary copy in the output buffer because the IMDCT destroys its input. */
        freq2 = (*out_syn.offset(1)).offset((overlap / 2) as isize);
        crate::stdlib::memcpy(
            freq2 as *mut libc::c_void,
            freq as *const libc::c_void,
            (N as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                .wrapping_add((0 * freq2.wrapping_offset_from(freq)) as usize),
        );
        b = 0;
        while b < B {
            crate::src::opus_1_2_1::celt::mdct::clt_mdct_backward_c(
                &(*mode).mdct,
                &mut *freq2.offset(b as isize),
                (*out_syn.offset(0)).offset((NB * b) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1
        }
        b = 0;
        while b < B {
            crate::src::opus_1_2_1::celt::mdct::clt_mdct_backward_c(
                &(*mode).mdct,
                &mut *freq.offset(b as isize),
                (*out_syn.offset(1)).offset((NB * b) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1
        }
    } else if CC == 1 && C == 2 {
        /* Downmixing a stereo stream to mono */
        let mut freq2_0: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
        freq2_0 = (*out_syn.offset(0)).offset((overlap / 2) as isize);
        crate::src::opus_1_2_1::celt::bands::denormalise_bands(
            mode, X, freq, oldBandE, start, effEnd, M, downsample, silence,
        );
        /* Use the output buffer as temp array before downmixing. */
        crate::src::opus_1_2_1::celt::bands::denormalise_bands(
            mode,
            X.offset(N as isize),
            freq2_0,
            oldBandE.offset(nbEBands as isize),
            start,
            effEnd,
            M,
            downsample,
            silence,
        );
        i = 0;
        while i < N {
            *freq.offset(i as isize) =
                0.5f32 * *freq.offset(i as isize) + 0.5 * *freq2_0.offset(i as isize);
            i += 1
        }
        b = 0;
        while b < B {
            crate::src::opus_1_2_1::celt::mdct::clt_mdct_backward_c(
                &(*mode).mdct,
                &mut *freq.offset(b as isize),
                (*out_syn.offset(0)).offset((NB * b) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1
        }
    } else {
        /* Normal case (mono or stereo) */
        c = 0;
        loop {
            crate::src::opus_1_2_1::celt::bands::denormalise_bands(
                mode,
                X.offset((c * N) as isize),
                freq,
                oldBandE.offset((c * nbEBands) as isize),
                start,
                effEnd,
                M,
                downsample,
                silence,
            );
            b = 0;
            while b < B {
                crate::src::opus_1_2_1::celt::mdct::clt_mdct_backward_c(
                    &(*mode).mdct,
                    &mut *freq.offset(b as isize),
                    (*out_syn.offset(c as isize)).offset((NB * b) as isize),
                    (*mode).window,
                    overlap,
                    shift,
                    B,
                    arch,
                );
                b += 1
            }
            c += 1;
            if !(c < CC) {
                break;
            }
        }
    }
    /* Saturate IMDCT output so that we can't overflow in the pitch postfilter
    or in the */
    c = 0;
    loop {
        i = 0;
        while i < N {
            *(*out_syn.offset(c as isize)).offset(i as isize) =
                *(*out_syn.offset(c as isize)).offset(i as isize);
            i += 1
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
}

unsafe extern "C" fn tf_decode(
    mut start: i32,
    mut end: i32,
    mut isTransient: i32,
    mut tf_res: *mut i32,
    mut LM: i32,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
) {
    let mut i: i32 = 0;
    let mut curr: i32 = 0;
    let mut tf_select: i32 = 0;
    let mut tf_select_rsv: i32 = 0;
    let mut tf_changed: i32 = 0;
    let mut logp: i32 = 0;
    let mut budget: crate::opus_types_h::opus_uint32 = 0;
    let mut tell: crate::opus_types_h::opus_uint32 = 0;
    budget = (*dec).storage.wrapping_mul(8u32);
    tell = ec_tell(dec) as crate::opus_types_h::opus_uint32;
    logp = if isTransient != 0 { 2 } else { 4 };
    tf_select_rsv = (LM > 0 && tell.wrapping_add(logp as u32).wrapping_add(1u32) <= budget) as i32;
    budget = (budget).wrapping_sub(tf_select_rsv as u32);
    curr = 0;
    tf_changed = curr;
    i = start;
    while i < end {
        if tell.wrapping_add(logp as u32) <= budget {
            curr ^= crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, logp as u32);
            tell = ec_tell(dec) as crate::opus_types_h::opus_uint32;
            tf_changed |= curr
        }
        *tf_res.offset(i as isize) = curr;
        logp = if isTransient != 0 { 4 } else { 5 };
        i += 1
    }
    tf_select = 0;
    if tf_select_rsv != 0
        && crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
            [(4 * isTransient + 0 + tf_changed) as usize] as i32
            != crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                [(4 * isTransient + 2 + tf_changed) as usize] as i32
    {
        tf_select = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, 1)
    }
    i = start;
    while i < end {
        *tf_res.offset(i as isize) = crate::src::opus_1_2_1::celt::celt::tf_select_table
            [LM as usize][(4 * isTransient + 2 * tf_select + *tf_res.offset(i as isize)) as usize]
            as i32;
        i += 1
    }
}

unsafe extern "C" fn celt_plc_pitch_search(
    mut decode_mem: *mut *mut crate::arch_h::celt_sig,
    mut C: i32,
    mut arch: i32,
) -> i32 {
    let mut pitch_index: i32 = 0;
    let mut lp_pitch_buf: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>()).wrapping_mul((2048i32 >> 1) as usize),
    );
    lp_pitch_buf = fresh2.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::src::opus_1_2_1::celt::pitch::pitch_downsample(decode_mem, lp_pitch_buf, 2048, C, arch);
    crate::src::opus_1_2_1::celt::pitch::pitch_search(
        lp_pitch_buf.offset((720i32 >> 1) as isize),
        lp_pitch_buf,
        2048 - 720,
        720 - 100,
        &mut pitch_index,
        arch,
    );
    pitch_index = 720 - pitch_index;
    return pitch_index;
}

unsafe extern "C" fn celt_decode_lost(mut st: *mut OpusCustomDecoder, mut N: i32, mut LM: i32) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let C: i32 = (*st).channels;
    let mut decode_mem: [*mut crate::arch_h::celt_sig; 2] = [0 as *mut crate::arch_h::celt_sig; 2];
    let mut out_syn: [*mut crate::arch_h::celt_sig; 2] = [0 as *mut crate::arch_h::celt_sig; 2];
    let mut lpc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut backgroundLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut nbEBands: i32 = 0;
    let mut overlap: i32 = 0;
    let mut start: i32 = 0;
    let mut loss_count: i32 = 0;
    let mut noise_based: i32 = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    c = 0;
    loop {
        decode_mem[c as usize] = (*st)
            ._decode_mem
            .as_mut_ptr()
            .offset((c * (2048 + overlap)) as isize);
        out_syn[c as usize] = decode_mem[c as usize].offset(2048).offset(-(N as isize));
        c += 1;
        if !(c < C) {
            break;
        }
    }
    lpc = (*st)
        ._decode_mem
        .as_mut_ptr()
        .offset(((2048 + overlap) * C) as isize);
    oldBandE = lpc.offset((C * 24) as isize);
    oldLogE = oldBandE.offset((2 * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((2 * nbEBands) as isize);
    backgroundLogE = oldLogE2.offset((2 * nbEBands) as isize);
    loss_count = (*st).loss_count;
    start = (*st).start;
    noise_based = (loss_count >= 5 || start != 0 || (*st).skip_plc != 0) as i32;
    if noise_based != 0 {
        /* Noise-based PLC/CNG */
        let mut X: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm; /* *< Interleaved normalised MDCTs */
        let mut seed: crate::opus_types_h::opus_uint32 = 0;
        let mut end: i32 = 0;
        let mut effEnd: i32 = 0;
        let mut decay: crate::arch_h::opus_val16 = 0.;
        end = (*st).end;
        effEnd = if start
            > (if end < (*mode).effEBands {
                end
            } else {
                (*mode).effEBands
            }) {
            start
        } else if end < (*mode).effEBands {
            end
        } else {
            (*mode).effEBands
        };
        let mut fresh3 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::arch_h::celt_norm>()).wrapping_mul((C * N) as usize),
        );
        X = fresh3.as_mut_ptr() as *mut crate::arch_h::celt_norm;
        /* Energy decay */
        decay = if loss_count == 0 { 1.5 } else { 0.5 };
        c = 0;
        loop {
            i = start;
            while i < end {
                *oldBandE.offset((c * nbEBands + i) as isize) = if *backgroundLogE
                    .offset((c * nbEBands + i) as isize)
                    > *oldBandE.offset((c * nbEBands + i) as isize) - decay
                {
                    *backgroundLogE.offset((c * nbEBands + i) as isize)
                } else {
                    (*oldBandE.offset((c * nbEBands + i) as isize)) - decay
                };
                i += 1
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        seed = (*st).rng;
        c = 0;
        while c < C {
            i = start;
            while i < effEnd {
                let mut _j: i32 = 0;
                let mut boffs: i32 = 0;
                let mut blen: i32 = 0;
                boffs = N * c + ((*eBands.offset(i as isize) as i32) << LM);
                blen = (*eBands.offset((i + 1) as isize) as i32
                    - *eBands.offset(i as isize) as i32)
                    << LM;

                for j in 0..blen {
                    seed = crate::src::opus_1_2_1::celt::bands::celt_lcg_rand(seed);

                    *X.offset((boffs + j) as isize) =
                        (seed as crate::opus_types_h::opus_int32 >> 20) as crate::arch_h::celt_norm;
                }
                crate::src::opus_1_2_1::celt::vq::renormalise_vector(
                    X.offset(boffs as isize),
                    blen,
                    1.0,
                    (*st).arch,
                );
                i += 1
            }
            c += 1
        }
        (*st).rng = seed;
        c = 0;
        loop {
            crate::stdlib::memmove(
                decode_mem[c as usize] as *mut libc::c_void,
                decode_mem[c as usize].offset(N as isize) as *const libc::c_void,
                ((2048 - N + (overlap >> 1)) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                    .wrapping_add(
                        (0 * decode_mem[c as usize]
                            .wrapping_offset_from(decode_mem[c as usize].offset(N as isize)))
                            as usize,
                    ),
            );
            c += 1;
            if !(c < C) {
                break;
            }
        }
        celt_synthesis(
            mode,
            X,
            out_syn.as_mut_ptr(),
            oldBandE,
            start,
            effEnd,
            C,
            C,
            0i32,
            LM,
            (*st).downsample,
            0i32,
            (*st).arch,
        );
    } else {
        /* Pitch-based PLC */
        let mut window: *const crate::arch_h::opus_val16 = 0 as *const crate::arch_h::opus_val16;
        let mut exc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
        let mut fade: crate::arch_h::opus_val16 = 1.0;
        let mut pitch_index: i32 = 0;
        let mut etmp: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
        let mut _exc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
        if loss_count == 0 {
            pitch_index = celt_plc_pitch_search(decode_mem.as_mut_ptr(), C, (*st).arch);
            (*st).last_pitch_index = pitch_index
        } else {
            pitch_index = (*st).last_pitch_index;
            fade = 0.8
        }
        let mut fresh4 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::arch_h::opus_val32>()).wrapping_mul(overlap as usize),
        );
        etmp = fresh4.as_mut_ptr() as *mut crate::arch_h::opus_val32;
        let mut fresh5 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_mul((1024i32 + 24) as usize),
        );
        _exc = fresh5.as_mut_ptr() as *mut crate::arch_h::opus_val16;
        exc = _exc.offset(24);
        window = (*mode).window;
        c = 0;
        loop {
            let mut decay_0: crate::arch_h::opus_val16 = 0.;
            let mut attenuation: crate::arch_h::opus_val16 = 0.;
            let mut S1: crate::arch_h::opus_val32 = 0f32;
            let mut buf: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
            let mut extrapolation_offset: i32 = 0;
            let mut extrapolation_len: i32 = 0;
            let mut exc_length: i32 = 0;
            let mut j_0: i32 = 0;
            buf = decode_mem[c as usize];
            i = 0;
            while i < 1024 {
                *exc.offset(i as isize) = *buf.offset((2048 - 1024 + i) as isize);
                i += 1
            }
            if loss_count == 0 {
                let mut ac: [crate::arch_h::opus_val32; 25] = [0.; 25];
                /* Compute LPC coefficients for the last MAX_PERIOD samples before
                the first loss so we can work in the excitation-filter domain. */
                crate::src::opus_1_2_1::celt::celt_lpc::_celt_autocorr(
                    exc,
                    ac.as_mut_ptr(),
                    window,
                    overlap,
                    24,
                    1024,
                    (*st).arch,
                );
                /* Add a noise floor of -40 dB. */
                ac[0] *= 1.0001;
                /* Use lag windowing to stabilize the Levinson-Durbin recursion. */
                i = 1;
                while i <= 24 {
                    /*ac[i] *= exp(-.5*(2*M_PI*.002*i)*(2*M_PI*.002*i));*/
                    ac[i as usize] -= ac[i as usize] * (0.008 * 0.008) * i as f32 * i as f32;
                    i += 1
                }
                crate::src::opus_1_2_1::celt::celt_lpc::_celt_lpc(
                    lpc.offset((c * 24i32) as isize),
                    ac.as_mut_ptr(),
                    24i32,
                );
            }
            /* We want the excitation for 2 pitch periods in order to look for a
            decaying signal, but we can't get more than MAX_PERIOD. */
            exc_length = if 2 * pitch_index < 1024 {
                (2) * pitch_index
            } else {
                1024
            };
            /* Initialize the LPC history with the samples just before the start
            of the region for which we're computing the excitation. */
            i = 0;
            while i < 24 {
                *exc.offset((1024 - exc_length - 24 + i) as isize) =
                    *buf.offset((2048 - exc_length - 24 + i) as isize);
                i += 1
            }
            /* Compute the excitation for exc_length samples before the loss. */
            crate::src::opus_1_2_1::celt::celt_lpc::celt_fir_c(
                exc.offset(1024).offset(-(exc_length as isize)),
                lpc.offset((c * 24) as isize),
                exc.offset(1024).offset(-(exc_length as isize)),
                exc_length,
                24,
                (*st).arch,
            );
            /* Check if the waveform is decaying, and if so how fast.
            We do this to avoid adding energy when concealing in a segment
            with decaying energy. */
            let mut E1: crate::arch_h::opus_val32 = 1f32;
            let mut E2: crate::arch_h::opus_val32 = 1f32;
            let mut decay_length: i32 = 0;
            decay_length = exc_length >> 1;
            i = 0;
            while i < decay_length {
                let mut e: crate::arch_h::opus_val16 = 0.;
                e = *exc.offset((1024 - decay_length + i) as isize);
                E1 += e * e;
                e = *exc.offset((1024 - 2 * decay_length + i) as isize);
                E2 += e * e;
                i += 1
            }
            E1 = if E1 < E2 { E1 } else { E2 };
            decay_0 = crate::stdlib::sqrt((E1 / E2) as f64) as f32;
            /* Move the decoder memory one frame to the left to give us room to
            add the data for the new frame. We ignore the overlap that extends
            past the end of the buffer, because we aren't going to use it. */
            crate::stdlib::memmove(
                buf as *mut libc::c_void,
                buf.offset(N as isize) as *const libc::c_void,
                ((2048i32 - N) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                    .wrapping_add((0 * buf.wrapping_offset_from(buf.offset(N as isize))) as usize),
            );
            /* Extrapolate from the end of the excitation with a period of
            "pitch_index", scaling down each period by an additional factor of
            "decay". */
            extrapolation_offset = 1024 - pitch_index;
            /* We need to extrapolate enough samples to cover a complete MDCT
            window (including overlap/2 samples on both sides). */
            extrapolation_len = N + overlap;
            /* We also apply fading if this is not the first loss. */
            attenuation = fade * decay_0;
            j_0 = 0;
            i = j_0;
            while i < extrapolation_len {
                let mut tmp: crate::arch_h::opus_val16 = 0.;
                if j_0 >= pitch_index {
                    j_0 -= pitch_index;
                    attenuation = attenuation * decay_0
                }
                *buf.offset((2048 - N + i) as isize) =
                    attenuation * *exc.offset((extrapolation_offset + j_0) as isize);
                /* Compute the energy of the previously decoded signal whose
                excitation we're copying. */
                tmp = *buf.offset((2048 - 1024 - N + extrapolation_offset + j_0) as isize);
                S1 += tmp * tmp;
                i += 1;
                j_0 += 1
            }
            let mut lpc_mem: [crate::arch_h::opus_val16; 24] = [0.; 24];
            /* Copy the last decoded samples (prior to the overlap region) to
            synthesis filter memory so we can have a continuous signal. */
            i = 0;
            while i < 24 {
                lpc_mem[i as usize] = *buf.offset((2048 - N - 1 - i) as isize);
                i += 1
            }
            /* Apply the synthesis filter to convert the excitation back into
            the signal domain. */
            crate::src::opus_1_2_1::celt::celt_lpc::celt_iir(
                buf.offset(2048).offset(-(N as isize)),
                lpc.offset((c * 24) as isize),
                buf.offset(2048).offset(-(N as isize)),
                extrapolation_len,
                24,
                lpc_mem.as_mut_ptr(),
                (*st).arch,
            );
            /* Check if the synthesis energy is higher than expected, which can
            happen with the signal changes during our window. If so,
            attenuate. */
            let mut S2: crate::arch_h::opus_val32 = 0f32;
            i = 0;
            while i < extrapolation_len {
                let mut tmp_0: crate::arch_h::opus_val16 = *buf.offset((2048 - N + i) as isize);
                S2 += tmp_0 * tmp_0;
                i += 1
            }
            /* This checks for an "explosion" in the synthesis. */
            /* The float test is written this way to catch NaNs in the output
            of the IIR filter at the same time. */
            if !(S1 > 0.2 * S2) {
                i = 0;
                while i < extrapolation_len {
                    *buf.offset((2048 - N + i) as isize) = 0f32;
                    i += 1
                }
            } else if S1 < S2 {
                let mut ratio: crate::arch_h::opus_val16 =
                    crate::stdlib::sqrt(((S1 + 1f32) / (S2 + 1f32)) as f64) as f32;
                i = 0;
                while i < overlap {
                    let mut tmp_g: crate::arch_h::opus_val16 =
                        1.0 - *window.offset(i as isize) * (1.0 - ratio);
                    *buf.offset((2048 - N + i) as isize) =
                        tmp_g * *buf.offset((2048 - N + i) as isize);
                    i += 1
                }
                i = overlap;
                while i < extrapolation_len {
                    *buf.offset((2048 - N + i) as isize) =
                        ratio * *buf.offset((2048 - N + i) as isize);
                    i += 1
                }
            }
            /* Apply the pre-filter to the MDCT overlap for the next frame because
            the post-filter will be re-applied in the decoder after the MDCT
            overlap. */
            crate::src::opus_1_2_1::celt::celt::comb_filter(
                etmp,
                buf.offset(2048),
                (*st).postfilter_period,
                (*st).postfilter_period,
                overlap,
                -(*st).postfilter_gain,
                -(*st).postfilter_gain,
                (*st).postfilter_tapset,
                (*st).postfilter_tapset,
                0 as *const crate::arch_h::opus_val16,
                0,
                (*st).arch,
            );
            /* Simulate TDAC on the concealed audio so that it blends with the
            MDCT of the next frame. */
            i = 0;
            while i < overlap / 2 {
                *buf.offset((2048 + i) as isize) = *window.offset(i as isize)
                    * *etmp.offset((overlap - 1 - i) as isize)
                    + *window.offset((overlap - i - 1) as isize) * *etmp.offset(i as isize);
                i += 1
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
    }
    (*st).loss_count = loss_count + 1;
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Copyright (c) 2008 Gregory Maxwell
Written by Jean-Marc Valin and Gregory Maxwell */
/* *
 @file celt.h
 @brief Contains all the functions for encoding and decoding audio
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
/* Store as Q6 char to save space. */
/* Encoder/decoder Requests */
/* * Controls the use of interframe prediction.
   0=Independent frames
   1=Short term interframe prediction allowed
   2=Long term prediction allowed
*/
/* Internal */
/* * Get the CELTMode used by an encoder or decoder */
/* Encoder stuff */
/* Decoder stuff */
#[no_mangle]

pub unsafe extern "C" fn celt_decode_with_ec(
    mut st: *mut OpusCustomDecoder,
    mut data: *const u8,
    mut len: i32,
    mut pcm: *mut crate::arch_h::opus_val16,
    mut frame_size: i32,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut accum: i32,
) -> i32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut N: i32 = 0;
    let mut spread_decision: i32 = 0;
    let mut bits: crate::opus_types_h::opus_int32 = 0;
    let mut _dec: crate::src::opus_1_2_1::celt::entcode::ec_dec =
        crate::src::opus_1_2_1::celt::entcode::ec_dec {
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
    let mut X: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut fine_quant: *mut i32 = 0 as *mut i32;
    let mut pulses: *mut i32 = 0 as *mut i32;
    let mut cap: *mut i32 = 0 as *mut i32;
    let mut offsets: *mut i32 = 0 as *mut i32;
    let mut fine_priority: *mut i32 = 0 as *mut i32;
    let mut tf_res: *mut i32 = 0 as *mut i32;
    let mut collapse_masks: *mut u8 = 0 as *mut u8;
    let mut decode_mem: [*mut crate::arch_h::celt_sig; 2] = [0 as *mut crate::arch_h::celt_sig; 2];
    let mut out_syn: [*mut crate::arch_h::celt_sig; 2] = [0 as *mut crate::arch_h::celt_sig; 2];
    let mut lpc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut backgroundLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut shortBlocks: i32 = 0;
    let mut isTransient: i32 = 0;
    let mut intra_ener: i32 = 0;
    let CC: i32 = (*st).channels;
    let mut LM: i32 = 0;
    let mut M: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut effEnd: i32 = 0;
    let mut codedBands: i32 = 0;
    let mut alloc_trim: i32 = 0;
    let mut postfilter_pitch: i32 = 0;
    let mut postfilter_gain: crate::arch_h::opus_val16 = 0.;
    let mut intensity: i32 = 0;
    let mut dual_stereo: i32 = 0;
    let mut total_bits: crate::opus_types_h::opus_int32 = 0;
    let mut balance: crate::opus_types_h::opus_int32 = 0;
    let mut tell: crate::opus_types_h::opus_int32 = 0;
    let mut dynalloc_logp: i32 = 0;
    let mut postfilter_tapset: i32 = 0;
    let mut anti_collapse_rsv: i32 = 0;
    let mut anti_collapse_on: i32 = 0;
    let mut silence: i32 = 0;
    let mut C: i32 = (*st).stream_channels;
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut nbEBands: i32 = 0;
    let mut overlap: i32 = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    start = (*st).start;
    end = (*st).end;
    frame_size *= (*st).downsample;
    lpc = (*st)
        ._decode_mem
        .as_mut_ptr()
        .offset(((2048 + overlap) * CC) as isize);
    oldBandE = lpc.offset((CC * 24) as isize);
    oldLogE = oldBandE.offset((2 * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((2 * nbEBands) as isize);
    backgroundLogE = oldLogE2.offset((2 * nbEBands) as isize);
    LM = 0;
    while LM <= (*mode).maxLM {
        if (*mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1
    }
    if LM > (*mode).maxLM {
        return -(1i32);
    }
    M = (1) << LM;
    if len < 0 || len > 1275 || pcm.is_null() {
        return -(1i32);
    }
    N = M * (*mode).shortMdctSize;
    c = 0;
    loop {
        decode_mem[c as usize] = (*st)
            ._decode_mem
            .as_mut_ptr()
            .offset((c * (2048 + overlap)) as isize);
        out_syn[c as usize] = decode_mem[c as usize].offset(2048).offset(-(N as isize));
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    effEnd = end;
    if effEnd > (*mode).effEBands {
        effEnd = (*mode).effEBands
    }
    if data.is_null() || len <= 1 {
        celt_decode_lost(st, N, LM);
        deemphasis(
            out_syn.as_mut_ptr(),
            pcm,
            N,
            CC,
            (*st).downsample,
            (*mode).preemph.as_ptr(),
            (*st).preemph_memD.as_mut_ptr(),
            accum,
        );
        return frame_size / (*st).downsample;
    }
    /* Check if there are at least two packets received consecutively before
     * turning on the pitch-based PLC */
    (*st).skip_plc = ((*st).loss_count != 0) as i32;
    if dec.is_null() {
        crate::src::opus_1_2_1::celt::entdec::ec_dec_init(
            &mut _dec,
            data as *mut u8,
            len as crate::opus_types_h::opus_uint32,
        );
        dec = &mut _dec
    }
    if C == 1 {
        i = 0;
        while i < nbEBands {
            *oldBandE.offset(i as isize) =
                if *oldBandE.offset(i as isize) > *oldBandE.offset((nbEBands + i) as isize) {
                    *oldBandE.offset(i as isize)
                } else {
                    *oldBandE.offset((nbEBands + i) as isize)
                };
            i += 1
        }
    }
    total_bits = len * 8;
    tell = ec_tell(dec);
    if tell >= total_bits {
        silence = 1
    } else if tell == 1 {
        silence = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, 15)
    } else {
        silence = 0
    }
    if silence != 0 {
        /* Pretend we've read all the remaining bits */
        tell = len * 8;
        (*dec).nbits_total += tell - ec_tell(dec)
    }
    postfilter_gain = 0f32;
    postfilter_pitch = 0;
    postfilter_tapset = 0;
    if start == 0 && tell + 16 <= total_bits {
        if crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, 1) != 0 {
            let mut qg: i32 = 0;
            let mut octave: i32 = 0;
            octave = crate::src::opus_1_2_1::celt::entdec::ec_dec_uint(dec, 6) as i32;
            postfilter_pitch = (((16i32) << octave) as u32)
                .wrapping_add(crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                    dec,
                    (4 + octave) as u32,
                ))
                .wrapping_sub(1u32) as i32;
            qg = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(dec, 3) as i32;
            if ec_tell(dec) + 2 <= total_bits {
                postfilter_tapset =
                    crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(dec, tapset_icdf.as_ptr(), 2)
            }
            postfilter_gain = 0.09375 * (qg + 1) as f32
        }
        tell = ec_tell(dec)
    }
    if LM > 0 && tell + 3 <= total_bits {
        isTransient = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, 3);
        tell = ec_tell(dec)
    } else {
        isTransient = 0
    }
    if isTransient != 0 {
        shortBlocks = M
    } else {
        shortBlocks = 0
    }
    /* Decode the global flags (first symbols in the stream) */
    intra_ener = if tell + 3 <= total_bits {
        crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(dec, 3)
    } else {
        0
    };
    /* Get band energies */
    crate::src::opus_1_2_1::celt::quant_bands::unquant_coarse_energy(
        mode, start, end, oldBandE, intra_ener, dec, C, LM,
    );
    let mut fresh6 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    tf_res = fresh6.as_mut_ptr();
    tf_decode(start, end, isTransient, tf_res, LM, dec);
    tell = ec_tell(dec);
    spread_decision = 2;
    if tell + 4 <= total_bits {
        spread_decision =
            crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(dec, spread_icdf.as_ptr(), 5)
    }
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    cap = fresh7.as_mut_ptr();
    crate::src::opus_1_2_1::celt::celt::init_caps(mode, cap, LM, C);
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    offsets = fresh8.as_mut_ptr();
    dynalloc_logp = 6;
    total_bits <<= 3;
    tell =
        crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(dec) as crate::opus_types_h::opus_int32;
    i = start;
    while i < end {
        let mut width: i32 = 0;
        let mut quanta: i32 = 0;
        let mut dynalloc_loop_logp: i32 = 0;
        let mut boost: i32 = 0;
        width = (C * (*eBands.offset((i + 1) as isize) as i32 - *eBands.offset(i as isize) as i32))
            << LM;
        /* quanta is 6 bits, but no more than 1 bit/sample
        and no less than 1/8 bit/sample */
        quanta = if (width << 3) < (if (6) << 3 > width { (6) << 3 } else { width }) {
            (width) << 3
        } else if (6) << 3 > width {
            (6) << 3
        } else {
            width
        };
        dynalloc_loop_logp = dynalloc_logp;
        boost = 0;
        while (tell + (dynalloc_loop_logp << 3)) < total_bits && boost < *cap.offset(i as isize) {
            let mut flag: i32 = 0;
            flag = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                dec,
                dynalloc_loop_logp as u32,
            );
            tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(dec)
                as crate::opus_types_h::opus_int32;
            if flag == 0 {
                break;
            }
            boost += quanta;
            total_bits -= quanta;
            dynalloc_loop_logp = 1
        }
        *offsets.offset(i as isize) = boost;
        /* Making dynalloc more likely */
        if boost > 0 {
            dynalloc_logp = if 2 > dynalloc_logp - 1 {
                2
            } else {
                (dynalloc_logp) - 1
            }
        }
        i += 1
    }
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    fine_quant = fresh9.as_mut_ptr();
    alloc_trim = if tell + ((6) << 3) <= total_bits {
        crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(dec, trim_icdf.as_ptr(), 7)
    } else {
        5
    };
    bits = (((len * 8i32) << 3) as u32)
        .wrapping_sub(crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(dec))
        .wrapping_sub(1u32) as crate::opus_types_h::opus_int32;
    anti_collapse_rsv = if isTransient != 0 && LM >= 2 && bits >= (LM + 2) << 3 {
        (1) << 3
    } else {
        0
    };
    bits -= anti_collapse_rsv;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    pulses = fresh10.as_mut_ptr();
    let mut fresh11 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>()).wrapping_mul(nbEBands as usize),
    );
    fine_priority = fresh11.as_mut_ptr();
    codedBands = crate::src::opus_1_2_1::celt::rate::compute_allocation(
        mode,
        start,
        end,
        offsets,
        cap,
        alloc_trim,
        &mut intensity,
        &mut dual_stereo,
        bits,
        &mut balance,
        pulses,
        fine_quant,
        fine_priority,
        C,
        LM,
        dec,
        0,
        0,
        0,
    );
    crate::src::opus_1_2_1::celt::quant_bands::unquant_fine_energy(
        mode, start, end, oldBandE, fine_quant, dec, C,
    );
    c = 0;
    loop {
        crate::stdlib::memmove(
            decode_mem[c as usize] as *mut libc::c_void,
            decode_mem[c as usize].offset(N as isize) as *const libc::c_void,
            ((2048 - N + overlap / 2) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>())
                .wrapping_add(
                    (0 * decode_mem[c as usize]
                        .wrapping_offset_from(decode_mem[c as usize].offset(N as isize)))
                        as usize,
                ),
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    /* Decode fixed codebook */
    let mut fresh12 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<u8>()).wrapping_mul((C * nbEBands) as usize),
    ); /* *< Interleaved normalised MDCTs */
    collapse_masks = fresh12.as_mut_ptr() as *mut u8;
    let mut fresh13 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>()).wrapping_mul((C * N) as usize),
    );
    X = fresh13.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    crate::src::opus_1_2_1::celt::bands::quant_all_bands(
        0,
        mode,
        start,
        end,
        X,
        if C == 2 {
            X.offset(N as isize)
        } else {
            0 as *mut crate::arch_h::celt_norm
        },
        collapse_masks,
        0 as *const crate::arch_h::celt_ener,
        pulses,
        shortBlocks,
        spread_decision,
        dual_stereo,
        intensity,
        tf_res,
        len * ((8) << 3) - anti_collapse_rsv,
        balance,
        dec,
        LM,
        codedBands,
        &mut (*st).rng,
        0,
        (*st).arch,
        (*st).disable_inv,
    );
    if anti_collapse_rsv > 0 {
        anti_collapse_on = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(dec, 1) as i32
    }
    crate::src::opus_1_2_1::celt::quant_bands::unquant_energy_finalise(
        mode,
        start,
        end,
        oldBandE,
        fine_quant,
        fine_priority,
        len * 8 - ec_tell(dec),
        dec,
        C,
    );
    if anti_collapse_on != 0 {
        crate::src::opus_1_2_1::celt::bands::anti_collapse(
            mode,
            X,
            collapse_masks,
            LM,
            C,
            N,
            start,
            end,
            oldBandE,
            oldLogE,
            oldLogE2,
            pulses,
            (*st).rng,
            (*st).arch,
        );
    }
    if silence != 0 {
        i = 0;
        while i < C * nbEBands {
            *oldBandE.offset(i as isize) = -28.0f32;
            i += 1
        }
    }
    celt_synthesis(
        mode,
        X,
        out_syn.as_mut_ptr(),
        oldBandE,
        start,
        effEnd,
        C,
        CC,
        isTransient,
        LM,
        (*st).downsample,
        silence,
        (*st).arch,
    );
    c = 0;
    loop {
        (*st).postfilter_period = if (*st).postfilter_period > 15 {
            (*st).postfilter_period
        } else {
            15
        };
        (*st).postfilter_period_old = if (*st).postfilter_period_old > 15 {
            (*st).postfilter_period_old
        } else {
            15
        };
        crate::src::opus_1_2_1::celt::celt::comb_filter(
            out_syn[c as usize],
            out_syn[c as usize],
            (*st).postfilter_period_old,
            (*st).postfilter_period,
            (*mode).shortMdctSize,
            (*st).postfilter_gain_old,
            (*st).postfilter_gain,
            (*st).postfilter_tapset_old,
            (*st).postfilter_tapset,
            (*mode).window,
            overlap,
            (*st).arch,
        );
        if LM != 0 {
            crate::src::opus_1_2_1::celt::celt::comb_filter(
                out_syn[c as usize].offset((*mode).shortMdctSize as isize),
                out_syn[c as usize].offset((*mode).shortMdctSize as isize),
                (*st).postfilter_period,
                postfilter_pitch,
                N - (*mode).shortMdctSize,
                (*st).postfilter_gain,
                postfilter_gain,
                (*st).postfilter_tapset,
                postfilter_tapset,
                (*mode).window,
                overlap,
                (*st).arch,
            );
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    (*st).postfilter_period_old = (*st).postfilter_period;
    (*st).postfilter_gain_old = (*st).postfilter_gain;
    (*st).postfilter_tapset_old = (*st).postfilter_tapset;
    (*st).postfilter_period = postfilter_pitch;
    (*st).postfilter_gain = postfilter_gain;
    (*st).postfilter_tapset = postfilter_tapset;
    if LM != 0 {
        (*st).postfilter_period_old = (*st).postfilter_period;
        (*st).postfilter_gain_old = (*st).postfilter_gain;
        (*st).postfilter_tapset_old = (*st).postfilter_tapset
    }
    if C == 1 {
        crate::stdlib::memcpy(
            &mut *oldBandE.offset(nbEBands as isize) as *mut crate::arch_h::opus_val16
                as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            (nbEBands as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add(
                    (0isize
                        * (&mut *oldBandE.offset(nbEBands as isize)
                            as *mut crate::arch_h::opus_val16)
                            .wrapping_offset_from(oldBandE)) as usize,
                ),
        );
    }
    /* In case start or end were to change */
    if isTransient == 0 {
        let mut max_background_increase: crate::arch_h::opus_val16 = 0.;
        crate::stdlib::memcpy(
            oldLogE2 as *mut libc::c_void,
            oldLogE as *const libc::c_void,
            ((2 * nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0 * oldLogE2.wrapping_offset_from(oldLogE)) as usize),
        );
        crate::stdlib::memcpy(
            oldLogE as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            ((2 * nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>())
                .wrapping_add((0 * oldLogE.wrapping_offset_from(oldBandE)) as usize),
        );
        /* In normal circumstances, we only allow the noise floor to increase by
        up to 2.4 dB/second, but when we're in DTX, we allow up to 6 dB
        increase for each update.*/
        if (*st).loss_count < 10 {
            max_background_increase = M as f32 * 0.001
        } else {
            max_background_increase = 1.0
        }
        i = 0;
        while i < 2 * nbEBands {
            *backgroundLogE.offset(i as isize) = if *backgroundLogE.offset(i as isize)
                + max_background_increase
                < *oldBandE.offset(i as isize)
            {
                (*backgroundLogE.offset(i as isize)) + max_background_increase
            } else {
                *oldBandE.offset(i as isize)
            };
            i += 1
        }
    } else {
        i = 0;
        while i < 2 * nbEBands {
            *oldLogE.offset(i as isize) =
                if *oldLogE.offset(i as isize) < *oldBandE.offset(i as isize) {
                    *oldLogE.offset(i as isize)
                } else {
                    *oldBandE.offset(i as isize)
                };
            i += 1
        }
    }
    c = 0;
    loop {
        i = 0;
        while i < start {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0f32;
            let ref mut fresh14 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh14 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh14;
            i += 1
        }
        i = end;
        while i < nbEBands {
            *oldBandE.offset((c * nbEBands + i) as isize) = 0f32;
            let ref mut fresh15 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh15 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh15;
            i += 1
        }
        c += 1;
        if !(c < 2) {
            break;
        }
    }
    (*st).rng = (*dec).rng;
    deemphasis(
        out_syn.as_mut_ptr(),
        pcm,
        N,
        CC,
        (*st).downsample,
        (*mode).preemph.as_ptr(),
        (*st).preemph_memD.as_mut_ptr(),
        accum,
    );
    (*st).loss_count = 0;
    if ec_tell(dec) > 8 * len {
        return -(3i32);
    }
    if ec_get_error(dec) != 0 {
        (*st).error = 1
    }
    return frame_size / (*st).downsample;
}
/* * Perform a CTL function on an Opus custom decoder.
 *
 * Generally the request and subsequent arguments are generated
 * by a convenience macro.
 * @see opus_genericctls
 */
/* CUSTOM_MODES */
#[no_mangle]

pub unsafe extern "C" fn opus_custom_decoder_ctl(
    mut st: *mut OpusCustomDecoder,
    mut request: i32,
    mut args: ...
) -> i32 {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    match request {
        10010 => {
            let mut value: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value < 0 || value >= (*(*st).mode).nbEBands {
                current_block = 5597585068398118923;
            } else {
                (*st).start = value;
                current_block = 1623252117315916725;
            }
        }
        10012 => {
            let mut value_0: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_0 < 1 || value_0 > (*(*st).mode).nbEBands {
                current_block = 5597585068398118923;
            } else {
                (*st).end = value_0;
                current_block = 1623252117315916725;
            }
        }
        10008 => {
            let mut value_1: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_1 < 1 || value_1 > 2 {
                current_block = 5597585068398118923;
            } else {
                (*st).stream_channels = value_1;
                current_block = 1623252117315916725;
            }
        }
        10007 => {
            let mut value_2: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_2.is_null() {
                current_block = 5597585068398118923;
            } else {
                *value_2 = (*st).error;
                (*st).error = 0;
                current_block = 1623252117315916725;
            }
        }
        4027 => {
            let mut value_3: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_3.is_null() {
                current_block = 5597585068398118923;
            } else {
                *value_3 = (*st).overlap / (*st).downsample;
                current_block = 1623252117315916725;
            }
        }
        4028 => {
            let mut _i: i32 = 0;
            let mut lpc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            lpc = (*st)
                ._decode_mem
                .as_mut_ptr()
                .offset(((2048i32 + (*st).overlap) * (*st).channels) as isize);
            oldBandE = lpc.offset(((*st).channels * 24i32) as isize);
            oldLogE = oldBandE.offset((2i32 * (*(*st).mode).nbEBands) as isize);
            oldLogE2 = oldLogE.offset((2i32 * (*(*st).mode).nbEBands) as isize);
            crate::stdlib::memset(
                &mut (*st).rng as *mut crate::opus_types_h::opus_uint32 as *mut libc::c_void,
                0,
                ((opus_custom_decoder_get_size((*st).mode, (*st).channels) as isize
                    - (&mut (*st).rng as *mut crate::opus_types_h::opus_uint32 as *mut i8)
                        .wrapping_offset_from(st as *mut i8)) as usize)
                    .wrapping_mul(::std::mem::size_of::<i8>()),
            );

            for i in 0..2 * (*(*st).mode).nbEBands {
                let ref mut fresh16 = *oldLogE2.offset(i as isize);

                *fresh16 = -28.0f32;

                *oldLogE.offset(i as isize) = *fresh16;
            }
            (*st).skip_plc = 1;
            current_block = 1623252117315916725;
        }
        4033 => {
            let mut value_4: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_4.is_null() {
                current_block = 5597585068398118923;
            } else {
                *value_4 = (*st).postfilter_period;
                current_block = 1623252117315916725;
            }
        }
        10015 => {
            let mut value_5: *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode = ap
                .as_va_list()
                .arg::<*mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode>();
            if value_5.is_null() {
                current_block = 5597585068398118923;
            } else {
                *value_5 = (*st).mode;
                current_block = 1623252117315916725;
            }
        }
        10016 => {
            let mut value_6: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).signalling = value_6;
            current_block = 1623252117315916725;
        }
        4031 => {
            let mut value_7: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            if value_7.is_null() {
                current_block = 5597585068398118923;
            } else {
                *value_7 = (*st).rng;
                current_block = 1623252117315916725;
            }
        }
        4046 => {
            let mut value_8: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_8 < 0 || value_8 > 1 {
                current_block = 5597585068398118923;
            } else {
                (*st).disable_inv = value_8;
                current_block = 1623252117315916725;
            }
        }
        4047 => {
            let mut value_9: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_9.is_null() {
                current_block = 5597585068398118923;
            } else {
                *value_9 = (*st).disable_inv;
                current_block = 1623252117315916725;
            }
        }
        _ => return -(5),
    }
    match current_block {
        1623252117315916725 => return 0,
        _ => return -(1),
    };
}
