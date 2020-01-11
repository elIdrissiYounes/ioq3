use ::libc;

pub mod SigProc_FIX_h {
    /* Allocate opus_int16 aligned to 4-byte memory address */
    /* Useful Macros that can be adjusted to other platforms */
    /* Fixed point macros */
    /* (a32 * b32) output have to be 32bit int */
    /* (a32 * b32) output have to be 32bit uint */
    /* a32 + (b32 * c32) output have to be 32bit int */
    /* a32 + (b32 * c32) output have to be 32bit uint */
    /* ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* a32 + ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* (a32 * b32) */
    /*(opus_int64)*/
    /* Adds two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Subtractss two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Multiply-accumulate macros that allow overflow in the addition (ie, no asserts in debug mode) */
    /* These macros enables checking for overflow in silk_API_Debug.h*/
    /* Saturation for positive input values */
    /* Add with saturation for positive input values */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* saturates before shifting */
    /* shift >= 0, allowed to overflow */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* Requires that shift > 0 */
    /* Number of rightshift required to fit the multiplication */
    /* Macro to convert floating-point constants to fixed-point */
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
        return if a < b { a } else { b };
    }
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
        return if a > b { a } else { b };
    }

    /* SILK_SIGPROC_FIX_H */
    /*    silk_SMMUL: Signed top word multiply.
    ARMv6        2 instruction cycles.
    ARMv3M+      3 instruction cycles. use SMULL and ignore LSB registers.(except xM)*/
    /*#define silk_SMMUL(a32, b32)                (opus_int32)silk_RSHIFT(silk_SMLAL(silk_SMULWB((a32), (b32)), (a32), silk_RSHIFT_ROUND((b32), 16)), 16)*/
    /* the following seems faster on x86 */
    /*  Add some multiplication functions that can be easily mapped to ARM. */
    /* PSEUDO-RANDOM GENERATOR                                                          */
    /* Make sure to store the result as the seed for the next call (also in between     */
    /* frames), otherwise result won't be random at all. When only using some of the    */
    /* bits, take the most significant bits by right-shifting.                          */
    /* Be careful, silk_abs returns wrong when input equals to silk_intXX_MIN */
}

pub mod float_cast_h {
    /* Copyright (C) 2001 Erik de Castro Lopo <erikd AT mega-nerd DOT com> */
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
    /* Version 1.1 */
    /*============================================================================
    **      On Intel Pentium processors (especially PIII and probably P4), converting
    **      from float to int is very slow. To meet the C specs, the code produced by
    **      most C compilers targeting Pentium needs to change the FPU rounding mode
    **      before the float to int conversion is performed.
    **
    **      Changing the FPU rounding mode causes the FPU pipeline to be flushed. It
    **      is this flushing of the pipeline which is so slow.
    **
    **      Fortunately the ISO C99 specifications define the functions lrint, lrintf,
    **      llrint and llrintf which fix this problem as a side effect.
    **
    **      On Unix-like systems, the configure process should have detected the
    **      presence of these functions. If they weren't found we have to replace them
    **      here with a standard C cast.
    */
    /*
    **      The C99 prototypes for lrint and lrintf are as follows:
    **
    **              long int lrintf (float x) ;
    **              long int lrint  (double x) ;
    */
    /*      The presence of the required functions are detected during the configure
    **      process and the values HAVE_LRINT and HAVE_LRINTF are set accordingly in
    **      the config.h file.
    */
    /* With GCC, when SSE is available, the fastest conversion is cvtss2si. */
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: libc::c_float) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub mod SigProc_FLP_h {
    /* floating-point to integer conversion (rounding) */
    #[inline]

    pub unsafe extern "C" fn silk_float2short_array(
        mut out: *mut crate::opus_types_h::opus_int16,
        mut in_0: *const libc::c_float,
        mut length: crate::opus_types_h::opus_int32,
    ) {
        let mut k: crate::opus_types_h::opus_int32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = if float2int(*in_0.offset(k as isize)) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if float2int(*in_0.offset(k as isize))
                < 0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
            {
                0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
            } else {
                float2int(*in_0.offset(k as isize))
            } as crate::opus_types_h::opus_int16;
            k -= 1
        }
    }
    /* integer to floating-point conversion */
    #[inline]

    pub unsafe extern "C" fn silk_short2float_array(
        mut out: *mut libc::c_float,
        mut in_0: *const crate::opus_types_h::opus_int16,
        mut length: crate::opus_types_h::opus_int32,
    ) {
        let mut k: crate::opus_types_h::opus_int32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = *in_0.offset(k as isize) as libc::c_float;
            k -= 1
        }
    }
    use crate::opus_types_h::opus_int16;
    use crate::opus_types_h::opus_int32;
    use crate::src::opus_1_2_1::silk::control_codec::float_cast_h::float2int;
    /* SILK_SIGPROC_FLP_H */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::control_h::silk_EncControlStruct;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;

pub use crate::src::opus_1_2_1::silk::control_codec::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::silk::control_codec::SigProc_FIX_h::silk_max_int;
pub use crate::src::opus_1_2_1::silk::control_codec::SigProc_FIX_h::silk_min_int;
pub use crate::src::opus_1_2_1::silk::resampler::silk_resampler;
pub use crate::src::opus_1_2_1::silk::resampler::silk_resampler_init;
use crate::stdlib::memset;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;

use crate::src::opus_1_2_1::silk::control_audio_bandwidth::silk_control_audio_bandwidth;
pub use crate::src::opus_1_2_1::silk::control_codec::SigProc_FLP_h::silk_float2short_array;
pub use crate::src::opus_1_2_1::silk::control_codec::SigProc_FLP_h::silk_short2float_array;
use crate::src::opus_1_2_1::silk::tables_NLSF_CB_NB_MB::silk_NLSF_CB_NB_MB;
use crate::src::opus_1_2_1::silk::tables_NLSF_CB_WB::silk_NLSF_CB_WB;
use crate::src::opus_1_2_1::silk::tables_other::silk_uniform4_iCDF;
use crate::src::opus_1_2_1::silk::tables_other::silk_uniform6_iCDF;
use crate::src::opus_1_2_1::silk::tables_other::silk_uniform8_iCDF;
use crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_10_ms_NB_iCDF;
use crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_10_ms_iCDF;
use crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_NB_iCDF;
use crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_iCDF;
/* Control the Silk encoder */
/* Control encoder */
#[no_mangle]

pub unsafe extern "C" fn silk_control_encoder(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut encControl: *mut crate::control_h::silk_EncControlStruct,
    allow_bw_switch: libc::c_int,
    channelNb: libc::c_int,
    force_fs_kHz: libc::c_int,
) -> libc::c_int {
    let mut fs_kHz: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    (*psEnc).sCmn.useDTX = (*encControl).useDTX;
    (*psEnc).sCmn.useCBR = (*encControl).useCBR;
    (*psEnc).sCmn.API_fs_Hz = (*encControl).API_sampleRate;
    (*psEnc).sCmn.maxInternal_fs_Hz = (*encControl).maxInternalSampleRate;
    (*psEnc).sCmn.minInternal_fs_Hz = (*encControl).minInternalSampleRate;
    (*psEnc).sCmn.desiredInternal_fs_Hz = (*encControl).desiredInternalSampleRate;
    (*psEnc).sCmn.useInBandFEC = (*encControl).useInBandFEC;
    (*psEnc).sCmn.nChannelsAPI = (*encControl).nChannelsAPI;
    (*psEnc).sCmn.nChannelsInternal = (*encControl).nChannelsInternal;
    (*psEnc).sCmn.allow_bandwidth_switch = allow_bw_switch;
    (*psEnc).sCmn.channelNb = channelNb;
    if (*psEnc).sCmn.controlled_since_last_payload != 0 as libc::c_int
        && (*psEnc).sCmn.prefillFlag == 0 as libc::c_int
    {
        if (*psEnc).sCmn.API_fs_Hz != (*psEnc).sCmn.prev_API_fs_Hz
            && (*psEnc).sCmn.fs_kHz > 0 as libc::c_int
        {
            /* Change in API sampling rate in the middle of encoding a packet */
            ret += silk_setup_resamplers(psEnc, (*psEnc).sCmn.fs_kHz)
        }
        return ret;
    }
    /* Beyond this point we know that there are no previously coded frames in the payload buffer */
    /* *******************************************/
    /* Determine internal sampling rate         */
    /* *******************************************/
    fs_kHz = crate::src::opus_1_2_1::silk::control_audio_bandwidth::silk_control_audio_bandwidth(
        &mut (*psEnc).sCmn as *mut _ as *mut crate::structs_h::silk_encoder_state,
        encControl as *mut crate::control_h::silk_EncControlStruct,
    );
    if force_fs_kHz != 0 {
        fs_kHz = force_fs_kHz
    }
    /* *******************************************/
    /* Prepare resampler and buffered data      */
    /* *******************************************/
    ret += silk_setup_resamplers(psEnc, fs_kHz);
    /* *******************************************/
    /* Set internal sampling frequency          */
    /* *******************************************/
    ret += silk_setup_fs(psEnc, fs_kHz, (*encControl).payloadSize_ms);
    /* *******************************************/
    /* Set encoding complexity                  */
    /* *******************************************/
    ret += silk_setup_complexity(&mut (*psEnc).sCmn, (*encControl).complexity);
    /* *******************************************/
    /* Set packet loss rate measured by farend  */
    /* *******************************************/
    (*psEnc).sCmn.PacketLoss_perc = (*encControl).packetLossPercentage;
    /* *******************************************/
    /* Set LBRR usage                           */
    /* *******************************************/
    ret += silk_setup_LBRR(&mut (*psEnc).sCmn, encControl);
    (*psEnc).sCmn.controlled_since_last_payload = 1 as libc::c_int;
    return ret;
}
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

unsafe extern "C" fn silk_setup_resamplers(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut fs_kHz: libc::c_int,
) -> libc::c_int
/* I                        */ {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*psEnc).sCmn.fs_kHz != fs_kHz || (*psEnc).sCmn.prev_API_fs_Hz != (*psEnc).sCmn.API_fs_Hz {
        if (*psEnc).sCmn.fs_kHz == 0 as libc::c_int {
            /* Initialize the resampler for enc_API.c preparing resampling from API_fs_Hz to fs_kHz */
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler_init(
                &mut (*psEnc).sCmn.resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                (*psEnc).sCmn.API_fs_Hz,
                fs_kHz * 1000 as libc::c_int,
                1 as libc::c_int,
            )
        } else {
            let mut x_buf_API_fs_Hz: *mut crate::opus_types_h::opus_int16 =
                0 as *mut crate::opus_types_h::opus_int16;
            let mut temp_resampler_state: *mut crate::resampler_structs_h::silk_resampler_state_struct =
                0 as *mut crate::resampler_structs_h::silk_resampler_state_struct;
            let mut x_bufFIX: *mut crate::opus_types_h::opus_int16 =
                0 as *mut crate::opus_types_h::opus_int16;
            let mut new_buf_samples: crate::opus_types_h::opus_int32 = 0;
            let mut api_buf_samples: crate::opus_types_h::opus_int32 = 0;
            let mut old_buf_samples: crate::opus_types_h::opus_int32 = 0;
            let mut buf_length_ms: crate::opus_types_h::opus_int32 = 0;
            buf_length_ms = ((((*psEnc).sCmn.nb_subfr * 5 as libc::c_int)
                as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                + 5 as libc::c_int;
            old_buf_samples = buf_length_ms * (*psEnc).sCmn.fs_kHz;
            new_buf_samples = buf_length_ms * fs_kHz;
            let mut fresh0 = ::std::vec::from_elem(
                0,
                (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
                    .wrapping_mul(
                        (if old_buf_samples > new_buf_samples {
                            old_buf_samples
                        } else {
                            new_buf_samples
                        }) as libc::c_ulong,
                    ) as usize,
            );
            x_bufFIX = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
            silk_float2short_array(x_bufFIX, (*psEnc).x_buf.as_mut_ptr(), old_buf_samples);
            /* Initialize resampler for temporary resampling of x_buf data to API_fs_Hz */
            let mut fresh1 = ::std::vec::from_elem(
                0,
                (::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>()
                    as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong) as usize,
            );
            temp_resampler_state =
                fresh1.as_mut_ptr() as *mut crate::resampler_structs_h::silk_resampler_state_struct;
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler_init(
                temp_resampler_state
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                (*psEnc).sCmn.fs_kHz as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * 1000 as libc::c_int as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32,
                (*psEnc).sCmn.API_fs_Hz,
                0 as libc::c_int,
            );
            /* Calculate number of samples to temporarily upsample */
            api_buf_samples = buf_length_ms * ((*psEnc).sCmn.API_fs_Hz / 1000 as libc::c_int);
            /* Temporary resampling of x_buf data to API_fs_Hz */
            let mut fresh2 = ::std::vec::from_elem(
                0,
                (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
                    .wrapping_mul(api_buf_samples as libc::c_ulong) as usize,
            );
            x_buf_API_fs_Hz = fresh2.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                temp_resampler_state
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                x_buf_API_fs_Hz,
                x_bufFIX as *const crate::opus_types_h::opus_int16,
                old_buf_samples,
            );
            /* Initialize the resampler for enc_API.c preparing resampling from API_fs_Hz to fs_kHz */
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler_init(
                &mut (*psEnc).sCmn.resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                (*psEnc).sCmn.API_fs_Hz,
                fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * 1000 as libc::c_int as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32,
                1 as libc::c_int,
            );
            /* Correct resampler state by resampling buffered data from API_fs_Hz to fs_kHz */
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*psEnc).sCmn.resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                x_bufFIX,
                x_buf_API_fs_Hz as *const crate::opus_types_h::opus_int16,
                api_buf_samples,
            );
            silk_short2float_array((*psEnc).x_buf.as_mut_ptr(), x_bufFIX, new_buf_samples);
        }
    }
    (*psEnc).sCmn.prev_API_fs_Hz = (*psEnc).sCmn.API_fs_Hz;
    return ret;
}

unsafe extern "C" fn silk_setup_fs(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut fs_kHz: libc::c_int,
    mut PacketSize_ms: libc::c_int,
) -> libc::c_int
/* I                        */ {
    let mut ret: libc::c_int = 0 as libc::c_int;
    /* Set packet size */
    if PacketSize_ms != (*psEnc).sCmn.PacketSize_ms {
        if PacketSize_ms != 10 as libc::c_int
            && PacketSize_ms != 20 as libc::c_int
            && PacketSize_ms != 40 as libc::c_int
            && PacketSize_ms != 60 as libc::c_int
        {
            ret = -(103 as libc::c_int)
        }
        if PacketSize_ms <= 10 as libc::c_int {
            (*psEnc).sCmn.nFramesPerPacket = 1 as libc::c_int;
            (*psEnc).sCmn.nb_subfr = if PacketSize_ms == 10 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            (*psEnc).sCmn.frame_length = PacketSize_ms as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
            (*psEnc).sCmn.pitch_LPC_win_length = (10 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int))
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
            if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
                (*psEnc).sCmn.pitch_contour_iCDF =
                    crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_10_ms_NB_iCDF
                        .as_ptr()
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF =
                    crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_10_ms_iCDF
                        .as_ptr()
            }
        } else {
            (*psEnc).sCmn.nFramesPerPacket = PacketSize_ms / (5 as libc::c_int * 4 as libc::c_int);
            (*psEnc).sCmn.nb_subfr = 4 as libc::c_int;
            (*psEnc).sCmn.frame_length = 20 as libc::c_int as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
            (*psEnc).sCmn.pitch_LPC_win_length = (20 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int))
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
            if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
                (*psEnc).sCmn.pitch_contour_iCDF =
                    crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_NB_iCDF
                        .as_ptr()
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF =
                    crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_iCDF.as_ptr()
            }
        }
        (*psEnc).sCmn.PacketSize_ms = PacketSize_ms;
        (*psEnc).sCmn.TargetRate_bps = 0 as libc::c_int
        /* trigger new SNR computation */
    }
    /* Set internal sampling frequency */
    if (*psEnc).sCmn.fs_kHz != fs_kHz {
        /* reset part of the state */
        crate::stdlib::memset(
            &mut (*psEnc).sShape as *mut crate::structs_FLP_h::silk_shape_state_FLP
                as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::structs_FLP_h::silk_shape_state_FLP>() as libc::c_ulong,
        ); /* trigger new SNR computation */
        crate::stdlib::memset(
            &mut (*psEnc).sCmn.sNSQ as *mut crate::structs_h::silk_nsq_state as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::structs_h::silk_nsq_state>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            (*psEnc).sCmn.prev_NLSFq_Q15.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 16]>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            &mut (*psEnc).sCmn.sLP.In_LP_State as *mut [crate::opus_types_h::opus_int32; 2]
                as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>() as libc::c_ulong,
        );
        (*psEnc).sCmn.inputBufIx = 0 as libc::c_int;
        (*psEnc).sCmn.nFramesEncoded = 0 as libc::c_int;
        (*psEnc).sCmn.TargetRate_bps = 0 as libc::c_int;
        /* Initialize non-zero parameters */
        (*psEnc).sCmn.prevLag = 100 as libc::c_int;
        (*psEnc).sCmn.first_frame_after_reset = 1 as libc::c_int;
        (*psEnc).sShape.LastGainIndex = 10 as libc::c_int as libc::c_schar;
        (*psEnc).sCmn.sNSQ.lagPrev = 100 as libc::c_int;
        (*psEnc).sCmn.sNSQ.prev_gain_Q16 = 65536 as libc::c_int;
        (*psEnc).sCmn.prevSignalType = 0 as libc::c_int as libc::c_schar;
        (*psEnc).sCmn.fs_kHz = fs_kHz;
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int {
            if (*psEnc).sCmn.nb_subfr == 4 as libc::c_int {
                (*psEnc).sCmn.pitch_contour_iCDF =
                    crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_NB_iCDF
                        .as_ptr()
            } else {
                (*psEnc).sCmn.pitch_contour_iCDF =
                    crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_10_ms_NB_iCDF
                        .as_ptr()
            }
        } else if (*psEnc).sCmn.nb_subfr == 4 as libc::c_int {
            (*psEnc).sCmn.pitch_contour_iCDF =
                crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_iCDF.as_ptr()
        } else {
            (*psEnc).sCmn.pitch_contour_iCDF =
                crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_contour_10_ms_iCDF
                    .as_ptr()
        }
        if (*psEnc).sCmn.fs_kHz == 8 as libc::c_int || (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
            (*psEnc).sCmn.predictLPCOrder = 10 as libc::c_int;
            (*psEnc).sCmn.psNLSF_CB =
                &crate::src::opus_1_2_1::silk::tables_NLSF_CB_NB_MB::silk_NLSF_CB_NB_MB
        } else {
            (*psEnc).sCmn.predictLPCOrder = 16 as libc::c_int;
            (*psEnc).sCmn.psNLSF_CB =
                &crate::src::opus_1_2_1::silk::tables_NLSF_CB_WB::silk_NLSF_CB_WB
        }
        (*psEnc).sCmn.subfr_length = 5 as libc::c_int * fs_kHz;
        (*psEnc).sCmn.frame_length = (*psEnc).sCmn.subfr_length as crate::opus_types_h::opus_int16
            as crate::opus_types_h::opus_int32
            * (*psEnc).sCmn.nb_subfr as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32;
        (*psEnc).sCmn.ltp_mem_length = 20 as libc::c_int as crate::opus_types_h::opus_int16
            as crate::opus_types_h::opus_int32
            * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
        (*psEnc).sCmn.la_pitch = 2 as libc::c_int as crate::opus_types_h::opus_int16
            as crate::opus_types_h::opus_int32
            * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
        (*psEnc).sCmn.max_pitch_lag = 18 as libc::c_int as crate::opus_types_h::opus_int16
            as crate::opus_types_h::opus_int32
            * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
        if (*psEnc).sCmn.nb_subfr == 4 as libc::c_int {
            (*psEnc).sCmn.pitch_LPC_win_length = (20 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int))
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        } else {
            (*psEnc).sCmn.pitch_LPC_win_length = (10 as libc::c_int
                + ((2 as libc::c_int) << 1 as libc::c_int))
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        }
        if (*psEnc).sCmn.fs_kHz == 16 as libc::c_int {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF =
                crate::src::opus_1_2_1::silk::tables_other::silk_uniform8_iCDF.as_ptr()
        } else if (*psEnc).sCmn.fs_kHz == 12 as libc::c_int {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF =
                crate::src::opus_1_2_1::silk::tables_other::silk_uniform6_iCDF.as_ptr()
        } else {
            (*psEnc).sCmn.pitch_lag_low_bits_iCDF =
                crate::src::opus_1_2_1::silk::tables_other::silk_uniform4_iCDF.as_ptr()
        }
    }
    /* Check that settings are valid */
    return ret;
}

unsafe extern "C" fn silk_setup_complexity(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut Complexity: libc::c_int,
) -> libc::c_int
/* I                        */ {
    let mut ret: libc::c_int = 0 as libc::c_int;
    /* Set encoding complexity */
    if Complexity < 1 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = 0 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.8f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 6 as libc::c_int;
        (*psEncC).shapingLPCOrder = 12 as libc::c_int;
        (*psEncC).la_shape = 3 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int
    } else if Complexity < 2 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = 1 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.76f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 8 as libc::c_int;
        (*psEncC).shapingLPCOrder = 14 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 1 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 3 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int
    } else if Complexity < 3 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = 0 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.8f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 6 as libc::c_int;
        (*psEncC).shapingLPCOrder = 12 as libc::c_int;
        (*psEncC).la_shape = 3 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 2 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int
    } else if Complexity < 4 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = 1 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.76f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 8 as libc::c_int;
        (*psEncC).shapingLPCOrder = 14 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 0 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 4 as libc::c_int;
        (*psEncC).warping_Q16 = 0 as libc::c_int
    } else if Complexity < 6 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = 1 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.74f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 10 as libc::c_int;
        (*psEncC).shapingLPCOrder = 16 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 2 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 6 as libc::c_int;
        (*psEncC).warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
    } else if Complexity < 8 as libc::c_int {
        (*psEncC).pitchEstimationComplexity = 1 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.72f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 12 as libc::c_int;
        (*psEncC).shapingLPCOrder = 20 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 3 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 8 as libc::c_int;
        (*psEncC).warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
    } else {
        (*psEncC).pitchEstimationComplexity = 2 as libc::c_int;
        (*psEncC).pitchEstimationThreshold_Q16 = (0.7f64
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64)
            as crate::opus_types_h::opus_int32;
        (*psEncC).pitchEstimationLPCOrder = 16 as libc::c_int;
        (*psEncC).shapingLPCOrder = 24 as libc::c_int;
        (*psEncC).la_shape = 5 as libc::c_int * (*psEncC).fs_kHz;
        (*psEncC).nStatesDelayedDecision = 4 as libc::c_int;
        (*psEncC).useInterpolatedNLSFs = 1 as libc::c_int;
        (*psEncC).NLSF_MSVQ_Survivors = 16 as libc::c_int;
        (*psEncC).warping_Q16 = (*psEncC).fs_kHz
            * ((0.015f32
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
    }
    /* Do not allow higher pitch estimation LPC order than predict LPC order */
    (*psEncC).pitchEstimationLPCOrder =
        silk_min_int((*psEncC).pitchEstimationLPCOrder, (*psEncC).predictLPCOrder);
    (*psEncC).shapeWinLength =
        5 as libc::c_int * (*psEncC).fs_kHz + 2 as libc::c_int * (*psEncC).la_shape;
    (*psEncC).Complexity = Complexity;
    return ret;
}
#[inline]

unsafe extern "C" fn silk_setup_LBRR(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut encControl: *const crate::control_h::silk_EncControlStruct,
) -> libc::c_int
/* I                        */ {
    let mut LBRR_in_previous_packet: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    LBRR_in_previous_packet = (*psEncC).LBRR_enabled;
    (*psEncC).LBRR_enabled = (*encControl).LBRR_coded;
    if (*psEncC).LBRR_enabled != 0 {
        /* Set gain increase for coding LBRR excitation */
        if LBRR_in_previous_packet == 0 as libc::c_int {
            /* Previous packet did not have LBRR, and was therefore coded at a higher bitrate */
            (*psEncC).LBRR_GainIncreases = 7 as libc::c_int
        } else {
            (*psEncC).LBRR_GainIncreases = silk_max_int(
                7 as libc::c_int
                    - ((*psEncC).PacketLoss_perc as libc::c_longlong
                        * (0.4f64
                            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int)
                                as libc::c_double
                            + 0.5f64) as crate::opus_types_h::opus_int32
                            as crate::opus_types_h::opus_int16
                            as libc::c_longlong
                        >> 16 as libc::c_int)
                        as crate::opus_types_h::opus_int32,
                2 as libc::c_int,
            )
        }
    }
    return ret;
}
