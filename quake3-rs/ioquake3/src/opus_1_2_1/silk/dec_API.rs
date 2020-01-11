use ::libc;

pub use crate::control_h::silk_DecControlStruct;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
use crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp;
use crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf;
use crate::src::opus_1_2_1::silk::decode_frame::silk_decode_frame;
use crate::src::opus_1_2_1::silk::decode_indices::silk_decode_indices;
use crate::src::opus_1_2_1::silk::decode_pulses::silk_decode_pulses;
use crate::src::opus_1_2_1::silk::decoder_set_fs::silk_decoder_set_fs;
use crate::src::opus_1_2_1::silk::init_decoder::silk_init_decoder;
use crate::src::opus_1_2_1::silk::resampler::silk_resampler;
use crate::src::opus_1_2_1::silk::stereo_MS_to_LR::silk_stereo_MS_to_LR;
use crate::src::opus_1_2_1::silk::stereo_decode_pred::silk_stereo_decode_mid_only;
use crate::src::opus_1_2_1::silk::stereo_decode_pred::silk_stereo_decode_pred;
use crate::src::opus_1_2_1::silk::tables_other::silk_LBRR_flags_iCDF_ptr;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::structs_h::silk_CNG_struct;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_PLC_struct;
pub use crate::structs_h::silk_decoder_state;
pub use crate::structs_h::stereo_dec_state;
pub use crate::structs_h::SideInfoIndices;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct silk_decoder {
    pub channel_state: [crate::structs_h::silk_decoder_state; 2],
    pub sStereo: crate::structs_h::stereo_dec_state,
    pub nChannelsAPI: libc::c_int,
    pub nChannelsInternal: libc::c_int,
    pub prev_decode_only_middle: libc::c_int,
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
/* Struct for TOC (Table of Contents) */
/* Voice activity for packet                            */
/* Voice activity for each frame in packet              */
/* Flag indicating if packet contains in-band FEC       */
/* ***************************************/
/* Encoder functions                    */
/* ***************************************/
/* **********************************************/
/* Get size in bytes of the Silk encoder state */
/* **********************************************/
/* O    Returns error code                              */
/* O    Number of bytes in SILK encoder state           */
/* ************************/
/* Init or reset encoder */
/* ************************/
/* O    Returns error code                              */
/* I/O  State                                           */
/* I    Run-time architecture                           */
/* O    Encoder Status                                  */
/* *************************/
/* Encode frame with Silk */
/* *************************/
/* Note: if prefillFlag is set, the input must contain 10 ms of audio, irrespective of what                     */
/* encControl->payloadSize_ms is set to                                                                         */
/* O    Returns error code                              */
/* I/O  State                                           */
/* I    Control status                                  */
/* I    Speech sample input vector                      */
/* I    Number of samples in input vector               */
/* I/O  Compressor data structure                       */
/* I/O  Number of bytes in payload (input: Max bytes)   */
/* I    Flag to indicate prefilling buffers no coding   */
/* ***************************************/
/* Decoder functions                    */
/* ***************************************/
/* **********************************************/
/* Get size in bytes of the Silk decoder state */
/* **********************************************/
/* ********************/
/* Decoder functions */
/* ********************/
#[no_mangle]

pub unsafe extern "C" fn silk_Get_Decoder_Size(mut decSizeBytes: *mut libc::c_int) -> libc::c_int
/* O    Number of bytes in SILK decoder state           */ {
    let mut ret: libc::c_int = 0 as libc::c_int;
    *decSizeBytes = ::std::mem::size_of::<silk_decoder>() as libc::c_ulong as libc::c_int;
    return ret;
}
/* ************************/
/* Init or Reset decoder */
/* ************************/
/* Reset decoder state */
#[no_mangle]

pub unsafe extern "C" fn silk_InitDecoder(mut decState: *mut libc::c_void) -> libc::c_int
/* I/O  State                                           */ {
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut channel_state: *mut crate::structs_h::silk_decoder_state = (*(decState
        as *mut silk_decoder))
        .channel_state
        .as_mut_ptr();
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ret = crate::src::opus_1_2_1::silk::init_decoder::silk_init_decoder(
            &mut *channel_state.offset(n as isize) as *mut _
                as *mut crate::structs_h::silk_decoder_state,
        );
        n += 1
    }
    crate::stdlib::memset(
        &mut (*(decState as *mut silk_decoder)).sStereo as *mut crate::structs_h::stereo_dec_state
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::structs_h::stereo_dec_state>() as libc::c_ulong,
    );
    /* Not strictly needed, but it's cleaner that way */
    (*(decState as *mut silk_decoder)).prev_decode_only_middle = 0 as libc::c_int;
    return ret;
}
/* *****************/
/* Decode a frame */
/* *****************/
/* Decode a frame */
#[no_mangle]

pub unsafe extern "C" fn silk_Decode(
    mut decState: *mut libc::c_void,
    mut decControl: *mut crate::control_h::silk_DecControlStruct,
    mut lostFlag: libc::c_int,
    mut newPacketFlag: libc::c_int,
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut samplesOut: *mut crate::opus_types_h::opus_int16,
    mut nSamplesOut: *mut crate::opus_types_h::opus_int32,
    mut arch: libc::c_int,
) -> libc::c_int
/* I    Run-time architecture                           */ {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut decode_only_middle: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nSamplesOutDec: crate::opus_types_h::opus_int32 = 0;
    let mut LBRR_symbol: crate::opus_types_h::opus_int32 = 0;
    let mut samplesOut1_tmp: [*mut crate::opus_types_h::opus_int16; 2] =
        [0 as *mut crate::opus_types_h::opus_int16; 2];
    let mut samplesOut1_tmp_storage1: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut samplesOut1_tmp_storage2: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut samplesOut2_tmp: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut MS_pred_Q13: [crate::opus_types_h::opus_int32; 2] = [0 as libc::c_int, 0];
    let mut resample_out_ptr: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut psDec: *mut silk_decoder = decState as *mut silk_decoder;
    let mut channel_state: *mut crate::structs_h::silk_decoder_state =
        (*psDec).channel_state.as_mut_ptr();
    let mut has_side: libc::c_int = 0;
    let mut stereo_to_mono: libc::c_int = 0;
    let mut delay_stack_alloc: libc::c_int = 0;
    /* *********************************/
    /* Test if first frame in payload */
    /* *********************************/
    if newPacketFlag != 0 {
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            (*channel_state.offset(n as isize)).nFramesDecoded = 0 as libc::c_int;
            n += 1
            /* Used to count frames in packet */
        }
    }
    /* If Mono -> Stereo transition in bitstream: init state of second channel */
    if (*decControl).nChannelsInternal > (*psDec).nChannelsInternal {
        ret += crate::src::opus_1_2_1::silk::init_decoder::silk_init_decoder(
            &mut *channel_state.offset(1 as libc::c_int as isize) as *mut _
                as *mut crate::structs_h::silk_decoder_state,
        )
    }
    stereo_to_mono = ((*decControl).nChannelsInternal == 1 as libc::c_int
        && (*psDec).nChannelsInternal == 2 as libc::c_int
        && (*decControl).internalSampleRate
            == 1000 as libc::c_int * (*channel_state.offset(0 as libc::c_int as isize)).fs_kHz)
        as libc::c_int;
    if (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded == 0 as libc::c_int {
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            let mut fs_kHz_dec: libc::c_int = 0;
            if (*decControl).payloadSize_ms == 0 as libc::c_int {
                /* Assuming packet loss, use 10 ms */
                (*channel_state.offset(n as isize)).nFramesPerPacket = 1 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 2 as libc::c_int
            } else if (*decControl).payloadSize_ms == 10 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 1 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 2 as libc::c_int
            } else if (*decControl).payloadSize_ms == 20 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 1 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 4 as libc::c_int
            } else if (*decControl).payloadSize_ms == 40 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 2 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 4 as libc::c_int
            } else if (*decControl).payloadSize_ms == 60 as libc::c_int {
                (*channel_state.offset(n as isize)).nFramesPerPacket = 3 as libc::c_int;
                (*channel_state.offset(n as isize)).nb_subfr = 4 as libc::c_int
            } else {
                return -(203 as libc::c_int);
            }
            fs_kHz_dec = ((*decControl).internalSampleRate >> 10 as libc::c_int) + 1 as libc::c_int;
            if fs_kHz_dec != 8 as libc::c_int
                && fs_kHz_dec != 12 as libc::c_int
                && fs_kHz_dec != 16 as libc::c_int
            {
                return -(200 as libc::c_int);
            }
            ret += crate::src::opus_1_2_1::silk::decoder_set_fs::silk_decoder_set_fs(
                &mut *channel_state.offset(n as isize) as *mut _
                    as *mut crate::structs_h::silk_decoder_state,
                fs_kHz_dec,
                (*decControl).API_sampleRate,
            );
            n += 1
        }
    }
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 2 as libc::c_int
        && ((*psDec).nChannelsAPI == 1 as libc::c_int
            || (*psDec).nChannelsInternal == 1 as libc::c_int)
    {
        crate::stdlib::memset(
            (*psDec).sStereo.pred_prev_Q13.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 2]>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            (*psDec).sStereo.sSide.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 2]>() as libc::c_ulong,
        );
        crate::stdlib::memcpy(
            &mut (*channel_state.offset(1 as libc::c_int as isize)).resampler_state
                as *mut crate::resampler_structs_h::silk_resampler_state_struct
                as *mut libc::c_void,
            &mut (*channel_state.offset(0 as libc::c_int as isize)).resampler_state
                as *mut crate::resampler_structs_h::silk_resampler_state_struct
                as *const libc::c_void,
            ::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>()
                as libc::c_ulong,
        );
    }
    (*psDec).nChannelsAPI = (*decControl).nChannelsAPI;
    (*psDec).nChannelsInternal = (*decControl).nChannelsInternal;
    if (*decControl).API_sampleRate > 48 as libc::c_int * 1000 as libc::c_int
        || (*decControl).API_sampleRate < 8000 as libc::c_int
    {
        ret = -(200 as libc::c_int);
        return ret;
    }
    if lostFlag != 1 as libc::c_int
        && (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded == 0 as libc::c_int
    {
        /* First decoder call for this payload */
        /* Decode VAD flags and LBRR flag */
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            i = 0 as libc::c_int;
            while i < (*channel_state.offset(n as isize)).nFramesPerPacket {
                (*channel_state.offset(n as isize)).VAD_flags[i as usize] =
                    crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                        psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        1 as libc::c_int as libc::c_uint,
                    );
                i += 1
            }
            (*channel_state.offset(n as isize)).LBRR_flag =
                crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                    psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    1 as libc::c_int as libc::c_uint,
                );
            n += 1
        }
        /* Decode LBRR flags */
        n = 0 as libc::c_int;
        while n < (*decControl).nChannelsInternal {
            crate::stdlib::memset(
                (*channel_state.offset(n as isize)).LBRR_flags.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong,
            );
            if (*channel_state.offset(n as isize)).LBRR_flag != 0 {
                if (*channel_state.offset(n as isize)).nFramesPerPacket == 1 as libc::c_int {
                    (*channel_state.offset(n as isize)).LBRR_flags[0 as libc::c_int as usize] =
                        1 as libc::c_int
                } else {
                    LBRR_symbol = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                        psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        crate::src::opus_1_2_1::silk::tables_other::silk_LBRR_flags_iCDF_ptr
                            [((*channel_state.offset(n as isize)).nFramesPerPacket
                                - 2 as libc::c_int) as usize],
                        8 as libc::c_int as libc::c_uint,
                    ) + 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < (*channel_state.offset(n as isize)).nFramesPerPacket {
                        (*channel_state.offset(n as isize)).LBRR_flags[i as usize] =
                            LBRR_symbol >> i & 1 as libc::c_int;
                        i += 1
                    }
                }
            }
            n += 1
        }
        if lostFlag == 0 as libc::c_int {
            /* Regular decoding: skip all LBRR data */
            i = 0 as libc::c_int;
            while i < (*channel_state.offset(0 as libc::c_int as isize)).nFramesPerPacket {
                n = 0 as libc::c_int;
                while n < (*decControl).nChannelsInternal {
                    if (*channel_state.offset(n as isize)).LBRR_flags[i as usize] != 0 {
                        let mut pulses: [crate::opus_types_h::opus_int16; 320] = [0; 320];
                        let mut condCoding: libc::c_int = 0;
                        if (*decControl).nChannelsInternal == 2 as libc::c_int
                            && n == 0 as libc::c_int
                        {
                            crate::src::opus_1_2_1::silk::stereo_decode_pred::silk_stereo_decode_pred(psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                                                    MS_pred_Q13.as_mut_ptr());
                            if (*channel_state.offset(1 as libc::c_int as isize)).LBRR_flags
                                [i as usize]
                                == 0 as libc::c_int
                            {
                                crate::src::opus_1_2_1::silk::stereo_decode_pred::silk_stereo_decode_mid_only(psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                                                            &mut decode_only_middle);
                            }
                        }
                        /* Use conditional coding if previous frame available */
                        if i > 0 as libc::c_int
                            && (*channel_state.offset(n as isize)).LBRR_flags
                                [(i - 1 as libc::c_int) as usize]
                                != 0
                        {
                            condCoding = 2 as libc::c_int
                        } else {
                            condCoding = 0 as libc::c_int
                        }
                        crate::src::opus_1_2_1::silk::decode_indices::silk_decode_indices(
                            &mut *channel_state.offset(n as isize) as *mut _
                                as *mut crate::structs_h::silk_decoder_state,
                            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            i,
                            1 as libc::c_int,
                            condCoding,
                        );
                        crate::src::opus_1_2_1::silk::decode_pulses::silk_decode_pulses(
                            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            pulses.as_mut_ptr(),
                            (*channel_state.offset(n as isize)).indices.signalType as libc::c_int,
                            (*channel_state.offset(n as isize)).indices.quantOffsetType
                                as libc::c_int,
                            (*channel_state.offset(n as isize)).frame_length,
                        );
                    }
                    n += 1
                }
                i += 1
            }
        }
    }
    /* Get MS predictor index */
    if (*decControl).nChannelsInternal == 2 as libc::c_int {
        if lostFlag == 0 as libc::c_int
            || lostFlag == 2 as libc::c_int
                && (*channel_state.offset(0 as libc::c_int as isize)).LBRR_flags
                    [(*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded as usize]
                    == 1 as libc::c_int
        {
            crate::src::opus_1_2_1::silk::stereo_decode_pred::silk_stereo_decode_pred(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                MS_pred_Q13.as_mut_ptr(),
            );
            /* For LBRR data, decode mid-only flag only if side-channel's LBRR flag is false */
            if lostFlag == 0 as libc::c_int
                && (*channel_state.offset(1 as libc::c_int as isize)).VAD_flags
                    [(*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded as usize]
                    == 0 as libc::c_int
                || lostFlag == 2 as libc::c_int
                    && (*channel_state.offset(1 as libc::c_int as isize)).LBRR_flags
                        [(*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded as usize]
                        == 0 as libc::c_int
            {
                crate::src::opus_1_2_1::silk::stereo_decode_pred::silk_stereo_decode_mid_only(
                    psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    &mut decode_only_middle,
                );
            } else {
                decode_only_middle = 0 as libc::c_int
            }
        } else {
            n = 0 as libc::c_int;
            while n < 2 as libc::c_int {
                MS_pred_Q13[n as usize] =
                    (*psDec).sStereo.pred_prev_Q13[n as usize] as crate::opus_types_h::opus_int32;
                n += 1
            }
        }
    }
    /* Reset side channel decoder prediction memory for first frame with side coding */
    if (*decControl).nChannelsInternal == 2 as libc::c_int
        && decode_only_middle == 0 as libc::c_int
        && (*psDec).prev_decode_only_middle == 1 as libc::c_int
    {
        crate::stdlib::memset(
            (*psDec).channel_state[1 as libc::c_int as usize]
                .outBuf
                .as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 480]>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            (*psDec).channel_state[1 as libc::c_int as usize]
                .sLPC_Q14_buf
                .as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 16]>() as libc::c_ulong,
        );
        (*psDec).channel_state[1 as libc::c_int as usize].lagPrev = 100 as libc::c_int;
        (*psDec).channel_state[1 as libc::c_int as usize].LastGainIndex =
            10 as libc::c_int as libc::c_schar;
        (*psDec).channel_state[1 as libc::c_int as usize].prevSignalType = 0 as libc::c_int;
        (*psDec).channel_state[1 as libc::c_int as usize].first_frame_after_reset = 1 as libc::c_int
    }
    /* Check if the temp buffer fits into the output PCM buffer. If it fits,
    we can delay allocating the temp buffer until after the SILK peak stack
    usage. We need to use a < and not a <= because of the two extra samples. */
    delay_stack_alloc = ((*decControl).internalSampleRate * (*decControl).nChannelsInternal
        < (*decControl).API_sampleRate * (*decControl).nChannelsAPI)
        as libc::c_int;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong).wrapping_mul(
            (if delay_stack_alloc != 0 {
                0 as libc::c_int
            } else {
                ((*decControl).nChannelsInternal)
                    * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                        + 2 as libc::c_int)
            }) as libc::c_ulong,
        ) as usize,
    );
    samplesOut1_tmp_storage1 = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    if delay_stack_alloc != 0 {
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut;
        samplesOut1_tmp[1 as libc::c_int as usize] = samplesOut
            .offset((*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize)
            .offset(2 as libc::c_int as isize)
    } else {
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut1_tmp_storage1;
        samplesOut1_tmp[1 as libc::c_int as usize] = samplesOut1_tmp_storage1
            .offset((*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize)
            .offset(2 as libc::c_int as isize)
    }
    if lostFlag == 0 as libc::c_int {
        has_side = (decode_only_middle == 0) as libc::c_int
    } else {
        has_side = ((*psDec).prev_decode_only_middle == 0
            || (*decControl).nChannelsInternal == 2 as libc::c_int
                && lostFlag == 2 as libc::c_int
                && (*channel_state.offset(1 as libc::c_int as isize)).LBRR_flags
                    [(*channel_state.offset(1 as libc::c_int as isize)).nFramesDecoded as usize]
                    == 1 as libc::c_int) as libc::c_int
    }
    /* Call decoder for one frame */
    n = 0 as libc::c_int;
    while n < (*decControl).nChannelsInternal {
        if n == 0 as libc::c_int || has_side != 0 {
            let mut FrameIndex: libc::c_int = 0;
            let mut condCoding_0: libc::c_int = 0;
            FrameIndex = (*channel_state.offset(0 as libc::c_int as isize)).nFramesDecoded - n;
            /* Use independent coding if no previous frame available */
            if FrameIndex <= 0 as libc::c_int {
                condCoding_0 = 0 as libc::c_int
            } else if lostFlag == 2 as libc::c_int {
                condCoding_0 = if (*channel_state.offset(n as isize)).LBRR_flags
                    [(FrameIndex - 1 as libc::c_int) as usize]
                    != 0
                {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                }
            } else if n > 0 as libc::c_int && (*psDec).prev_decode_only_middle != 0 {
                /* If we skipped a side frame in this packet, we don't
                need LTP scaling; the LTP state is well-defined. */
                condCoding_0 = 1 as libc::c_int
            } else {
                condCoding_0 = 2 as libc::c_int
            }
            ret += crate::src::opus_1_2_1::silk::decode_frame::silk_decode_frame(
                &mut *channel_state.offset(n as isize) as *mut _
                    as *mut crate::structs_h::silk_decoder_state,
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                    .offset(2 as libc::c_int as isize),
                &mut nSamplesOutDec,
                lostFlag,
                condCoding_0,
                arch,
            )
        } else {
            crate::stdlib::memset(
                &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::opus_types_h::opus_int16 as *mut libc::c_void,
                0 as libc::c_int,
                (nSamplesOutDec as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
        }
        let ref mut fresh1 = (*channel_state.offset(n as isize)).nFramesDecoded;
        *fresh1 += 1;
        n += 1
    }
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 2 as libc::c_int
    {
        /* Convert Mid/Side to Left/Right */
        crate::src::opus_1_2_1::silk::stereo_MS_to_LR::silk_stereo_MS_to_LR(
            &mut (*psDec).sStereo as *mut _ as *mut crate::structs_h::stereo_dec_state,
            samplesOut1_tmp[0 as libc::c_int as usize],
            samplesOut1_tmp[1 as libc::c_int as usize],
            MS_pred_Q13.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
            (*channel_state.offset(0 as libc::c_int as isize)).fs_kHz,
            nSamplesOutDec,
        );
    } else {
        /* Buffering */
        crate::stdlib::memcpy(
            samplesOut1_tmp[0 as libc::c_int as usize] as *mut libc::c_void,
            (*psDec).sStereo.sMid.as_mut_ptr() as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int16,
            >() as libc::c_ulong),
        );
        crate::stdlib::memcpy(
            (*psDec).sStereo.sMid.as_mut_ptr() as *mut libc::c_void,
            &mut *(*samplesOut1_tmp
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .offset(nSamplesOutDec as isize) as *mut crate::opus_types_h::opus_int16
                as *const libc::c_void,
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int16,
            >() as libc::c_ulong),
        );
    }
    /* Number of output samples */
    *nSamplesOut = nSamplesOutDec * (*decControl).API_sampleRate
        / ((*channel_state.offset(0 as libc::c_int as isize)).fs_kHz
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * 1000 as libc::c_int as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32);
    /* Set up pointers to temp buffers */
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong).wrapping_mul(
            (if (*decControl).nChannelsAPI == 2 as libc::c_int {
                *nSamplesOut
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        ) as usize,
    );
    samplesOut2_tmp = fresh2.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    if (*decControl).nChannelsAPI == 2 as libc::c_int {
        resample_out_ptr = samplesOut2_tmp
    } else {
        resample_out_ptr = samplesOut
    }
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong).wrapping_mul(
            (if delay_stack_alloc != 0 {
                ((*decControl).nChannelsInternal)
                    * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                        + 2 as libc::c_int)
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        ) as usize,
    );
    samplesOut1_tmp_storage2 = fresh3.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    if delay_stack_alloc != 0 {
        crate::stdlib::memcpy(
            samplesOut1_tmp_storage2 as *mut libc::c_void,
            samplesOut as *const libc::c_void,
            (((*decControl).nChannelsInternal
                * ((*channel_state.offset(0 as libc::c_int as isize)).frame_length
                    + 2 as libc::c_int)) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                )
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * samplesOut1_tmp_storage2.wrapping_offset_from(samplesOut) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        samplesOut1_tmp[0 as libc::c_int as usize] = samplesOut1_tmp_storage2;
        samplesOut1_tmp[1 as libc::c_int as usize] = samplesOut1_tmp_storage2
            .offset((*channel_state.offset(0 as libc::c_int as isize)).frame_length as isize)
            .offset(2 as libc::c_int as isize)
    }
    n = 0 as libc::c_int;
    while n
        < (if (*decControl).nChannelsAPI < (*decControl).nChannelsInternal {
            (*decControl).nChannelsAPI
        } else {
            (*decControl).nChannelsInternal
        })
    {
        /* Resample decoded signal to API_sampleRate */
        ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
            &mut (*channel_state.offset(n as isize)).resampler_state as *mut _
                as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
            resample_out_ptr,
            &mut *(*samplesOut1_tmp.as_mut_ptr().offset(n as isize))
                .offset(1 as libc::c_int as isize)
                as *mut crate::opus_types_h::opus_int16
                as *const crate::opus_types_h::opus_int16,
            nSamplesOutDec,
        );
        /* Interleave if stereo output and stereo stream */
        if (*decControl).nChannelsAPI == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut.offset((n + 2 as libc::c_int * i) as isize) =
                    *resample_out_ptr.offset(i as isize);
                i += 1
            }
        }
        n += 1
    }
    /* Create two channel output from mono stream */
    if (*decControl).nChannelsAPI == 2 as libc::c_int
        && (*decControl).nChannelsInternal == 1 as libc::c_int
    {
        if stereo_to_mono != 0 {
            /* Resample right channel for newly collapsed stereo just in case
            we weren't doing collapsing when switching to mono */
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*channel_state.offset(1 as libc::c_int as isize)).resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                resample_out_ptr,
                &mut *(*samplesOut1_tmp
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize)
                    as *mut crate::opus_types_h::opus_int16
                    as *const crate::opus_types_h::opus_int16,
                nSamplesOutDec,
            );
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut.offset((1 as libc::c_int + 2 as libc::c_int * i) as isize) =
                    *resample_out_ptr.offset(i as isize);
                i += 1
            }
        } else {
            i = 0 as libc::c_int;
            while i < *nSamplesOut {
                *samplesOut.offset((1 as libc::c_int + 2 as libc::c_int * i) as isize) =
                    *samplesOut.offset((0 as libc::c_int + 2 as libc::c_int * i) as isize);
                i += 1
            }
        }
    }
    /* Export pitch lag, measured at 48 kHz sampling rate */
    if (*channel_state.offset(0 as libc::c_int as isize)).prevSignalType == 2 as libc::c_int {
        let mut mult_tab: [libc::c_int; 3] = [6 as libc::c_int, 4 as libc::c_int, 3 as libc::c_int];
        (*decControl).prevPitchLag = (*channel_state.offset(0 as libc::c_int as isize)).lagPrev
            * mult_tab[((*channel_state.offset(0 as libc::c_int as isize)).fs_kHz
                - 8 as libc::c_int
                >> 2 as libc::c_int) as usize]
    } else {
        (*decControl).prevPitchLag = 0 as libc::c_int
    }
    if lostFlag == 1 as libc::c_int {
        /* On packet loss, remove the gain clamping to prevent having the energy "bounce back"
        if we lose packets when the energy is going down */
        i = 0 as libc::c_int;
        while i < (*psDec).nChannelsInternal {
            (*psDec).channel_state[i as usize].LastGainIndex = 10 as libc::c_int as libc::c_schar;
            i += 1
        }
    } else {
        (*psDec).prev_decode_only_middle = decode_only_middle
    }
    return ret;
}
