use ::libc;

pub mod entcode_h {

    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> i32 {
        return (*_this).nbits_total
            - (::std::mem::size_of::<u32>() as i32 * 8 - (*_this).rng.leading_zeros() as i32);
    }
}

pub use crate::control_h::silk_EncControlStruct;
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
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf;
use crate::src::opus_1_2_1::celt::entenc::ec_enc_patch_initial_bits;
pub use crate::src::opus_1_2_1::silk::enc_API::entcode_h::ec_tell;
use crate::src::opus_1_2_1::silk::resampler::silk_resampler;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::structs_FLP_h::silk_encoder;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::stereo_enc_state;
pub use crate::structs_h::SideInfoIndices;

use crate::src::opus_1_2_1::silk::check_control_input::check_control_input;
use crate::src::opus_1_2_1::silk::control_SNR::silk_control_SNR;
use crate::src::opus_1_2_1::silk::control_codec::silk_control_encoder;
use crate::src::opus_1_2_1::silk::encode_indices::silk_encode_indices;
use crate::src::opus_1_2_1::silk::encode_pulses::silk_encode_pulses;
use crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_do_VAD_FLP;
use crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_frame_FLP;
use crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder;
use crate::src::opus_1_2_1::silk::stereo_LR_to_MS::silk_stereo_LR_to_MS;
use crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_mid_only;
use crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_pred;
use crate::src::opus_1_2_1::silk::tables_other::silk_LBRR_flags_iCDF_ptr;
use crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10;
use crate::src::opus_1_2_1::silk::HP_variable_cutoff::silk_HP_variable_cutoff;
/* ***************************************/
/* Encoder functions                    */
/* ***************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_Get_Encoder_Size(mut encSizeBytes: *mut i32) -> i32
/* O    Number of bytes in SILK encoder state           */ {
    let mut ret: i32 = 0;
    *encSizeBytes = ::std::mem::size_of::<crate::structs_FLP_h::silk_encoder>() as i32;
    return ret;
}
/* ************************/
/* Init or Reset encoder */
/* ************************/
#[no_mangle]

pub unsafe extern "C" fn silk_InitEncoder(
    mut encState: *mut libc::c_void,
    mut arch: i32,
    mut encStatus: *mut crate::control_h::silk_EncControlStruct,
) -> i32
/* O    Encoder Status                                  */ {
    let mut psEnc: *mut crate::structs_FLP_h::silk_encoder =
        0 as *mut crate::structs_FLP_h::silk_encoder;
    let mut n: i32 = 0;
    let mut ret: i32 = 0;
    psEnc = encState as *mut crate::structs_FLP_h::silk_encoder;
    /* Reset encoder */
    crate::stdlib::memset(
        psEnc as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::structs_FLP_h::silk_encoder>(),
    );
    n = 0;
    while n < 2 {
        ret += crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder(
            &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize),
            arch,
        );
        (ret) != 0;
        n += 1
    }
    (*psEnc).nChannelsAPI = 1;
    (*psEnc).nChannelsInternal = 1;
    /* Read control structure */
    ret += silk_QueryEncoder(encState, encStatus);
    (ret) != 0;
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
/* **************************************/
/* Read control structure from encoder */
/* **************************************/
/* **************************************/
/* Read control structure from encoder */
/* **************************************/

unsafe extern "C" fn silk_QueryEncoder(
    mut encState: *const libc::c_void,
    mut encStatus: *mut crate::control_h::silk_EncControlStruct,
) -> i32
/* O    Encoder Status                                  */ {
    let mut ret: i32 = 0;
    let mut state_Fxx: *mut crate::structs_FLP_h::silk_encoder_state_FLP =
        0 as *mut crate::structs_FLP_h::silk_encoder_state_FLP;
    let mut psEnc: *mut crate::structs_FLP_h::silk_encoder =
        encState as *mut crate::structs_FLP_h::silk_encoder;
    state_Fxx = (*psEnc).state_Fxx.as_mut_ptr();
    (*encStatus).nChannelsAPI = (*psEnc).nChannelsAPI;
    (*encStatus).nChannelsInternal = (*psEnc).nChannelsInternal;
    (*encStatus).API_sampleRate = (*state_Fxx.offset(0)).sCmn.API_fs_Hz;
    (*encStatus).maxInternalSampleRate = (*state_Fxx.offset(0)).sCmn.maxInternal_fs_Hz;
    (*encStatus).minInternalSampleRate = (*state_Fxx.offset(0)).sCmn.minInternal_fs_Hz;
    (*encStatus).desiredInternalSampleRate = (*state_Fxx.offset(0)).sCmn.desiredInternal_fs_Hz;
    (*encStatus).payloadSize_ms = (*state_Fxx.offset(0)).sCmn.PacketSize_ms;
    (*encStatus).bitRate = (*state_Fxx.offset(0)).sCmn.TargetRate_bps;
    (*encStatus).packetLossPercentage = (*state_Fxx.offset(0)).sCmn.PacketLoss_perc;
    (*encStatus).complexity = (*state_Fxx.offset(0)).sCmn.Complexity;
    (*encStatus).useInBandFEC = (*state_Fxx.offset(0)).sCmn.useInBandFEC;
    (*encStatus).useDTX = (*state_Fxx.offset(0)).sCmn.useDTX;
    (*encStatus).useCBR = (*state_Fxx.offset(0)).sCmn.useCBR;
    (*encStatus).internalSampleRate = (*state_Fxx.offset(0)).sCmn.fs_kHz
        as crate::opus_types_h::opus_int16
        as crate::opus_types_h::opus_int32
        * 1000;
    (*encStatus).allowBandwidthSwitch = (*state_Fxx.offset(0)).sCmn.allow_bandwidth_switch;
    (*encStatus).inWBmodeWithoutVariableLP = ((*state_Fxx.offset(0)).sCmn.fs_kHz == 16
        && (*state_Fxx.offset(0)).sCmn.sLP.mode == 0)
        as i32;
    return ret;
}
/* *************************/
/* Encode frame with Silk */
/* *************************/
/* Note: if prefillFlag is set, the input must contain 10 ms of audio, irrespective of what                     */
/* encControl->payloadSize_ms is set to                                                                         */
#[no_mangle]

pub unsafe extern "C" fn silk_Encode(
    mut encState: *mut libc::c_void,
    mut encControl: *mut crate::control_h::silk_EncControlStruct,
    mut samplesIn: *const crate::opus_types_h::opus_int16,
    mut nSamplesIn: i32,
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut nBytesOut: *mut crate::opus_types_h::opus_int32,
    prefillFlag: i32,
) -> i32
/* I    Flag to indicate prefilling buffers no coding   */ {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut nBits: i32 = 0;
    let mut flags: i32 = 0;
    let mut tmp_payloadSize_ms: i32 = 0;
    let mut tmp_complexity: i32 = 0;
    let mut ret: i32 = 0;
    let mut nSamplesToBuffer: i32 = 0;
    let mut nSamplesToBufferMax: i32 = 0;
    let mut nBlocksOf10ms: i32 = 0;
    let mut nSamplesFromInput: i32 = 0;
    let mut nSamplesFromInputMax: i32 = 0;
    let mut speech_act_thr_for_switch_Q8: i32 = 0;
    let mut TargetRate_bps: crate::opus_types_h::opus_int32 = 0;
    let mut MStargetRates_bps: [crate::opus_types_h::opus_int32; 2] = [0; 2];
    let mut channelRate_bps: crate::opus_types_h::opus_int32 = 0;
    let mut LBRR_symbol: crate::opus_types_h::opus_int32 = 0;
    let mut sum: crate::opus_types_h::opus_int32 = 0;
    let mut psEnc: *mut crate::structs_FLP_h::silk_encoder =
        encState as *mut crate::structs_FLP_h::silk_encoder;
    let mut buf: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut transition: i32 = 0;
    let mut curr_block: i32 = 0;
    let mut tot_blocks: i32 = 0;
    if (*encControl).reducedDependency != 0 {
        (*psEnc).state_Fxx[0].sCmn.first_frame_after_reset = 1;
        (*psEnc).state_Fxx[1].sCmn.first_frame_after_reset = 1
    }
    (*psEnc).state_Fxx[1].sCmn.nFramesEncoded = 0;
    (*psEnc).state_Fxx[0].sCmn.nFramesEncoded = (*psEnc).state_Fxx[1].sCmn.nFramesEncoded;
    /* Check values in encoder control structure */
    ret = crate::src::opus_1_2_1::silk::check_control_input::check_control_input(encControl);
    if ret != 0 {
        return ret;
    }
    (*encControl).switchReady = 0;
    if (*encControl).nChannelsInternal > (*psEnc).nChannelsInternal {
        /* Mono -> Stereo transition: init state of second channel and stereo state */
        ret += crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder(
            &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(1),
            (*psEnc).state_Fxx[0].sCmn.arch,
        );
        crate::stdlib::memset(
            (*psEnc).sStereo.pred_prev_Q13.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 2]>(),
        );
        crate::stdlib::memset(
            (*psEnc).sStereo.sSide.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 2]>(),
        );
        (*psEnc).sStereo.mid_side_amp_Q0[0] = 0;
        (*psEnc).sStereo.mid_side_amp_Q0[1] = 1;
        (*psEnc).sStereo.mid_side_amp_Q0[2] = 0;
        (*psEnc).sStereo.mid_side_amp_Q0[3] = 1;
        (*psEnc).sStereo.width_prev_Q14 = 0;
        (*psEnc).sStereo.smth_width_Q14 =
            ((1i64 * ((1) << 14)) as f64 + 0.5) as crate::opus_types_h::opus_int16;
        if (*psEnc).nChannelsAPI == 2 {
            crate::stdlib::memcpy(
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                    .sCmn
                    .resampler_state
                    as *mut crate::resampler_structs_h::silk_resampler_state_struct
                    as *mut libc::c_void,
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .resampler_state
                    as *mut crate::resampler_structs_h::silk_resampler_state_struct
                    as *const libc::c_void,
                ::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>(),
            );
            crate::stdlib::memcpy(
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1isize))
                    .sCmn
                    .In_HP_State as *mut [crate::opus_types_h::opus_int32; 2]
                    as *mut libc::c_void,
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(0isize))
                    .sCmn
                    .In_HP_State as *mut [crate::opus_types_h::opus_int32; 2]
                    as *const libc::c_void,
                ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>(),
            );
        }
    }
    transition = ((*encControl).payloadSize_ms != (*psEnc).state_Fxx[0].sCmn.PacketSize_ms
        || (*psEnc).nChannelsInternal != (*encControl).nChannelsInternal) as i32;
    (*psEnc).nChannelsAPI = (*encControl).nChannelsAPI;
    (*psEnc).nChannelsInternal = (*encControl).nChannelsInternal;
    nBlocksOf10ms = 100 * nSamplesIn / (*encControl).API_sampleRate;
    tot_blocks = if nBlocksOf10ms > 1 {
        (nBlocksOf10ms) >> 1
    } else {
        1
    };
    curr_block = 0;
    if prefillFlag != 0 {
        /* Only accept input length of 10 ms */
        if nBlocksOf10ms != 1 {
            return -(101i32);
        }
        /* Reset Encoder */
        n = 0;
        while n < (*encControl).nChannelsInternal {
            ret = crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder(
                &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize),
                (*psEnc).state_Fxx[n as usize].sCmn.arch,
            );
            n += 1
        }
        tmp_payloadSize_ms = (*encControl).payloadSize_ms;
        (*encControl).payloadSize_ms = 10;
        tmp_complexity = (*encControl).complexity;
        (*encControl).complexity = 0;
        n = 0;
        while n < (*encControl).nChannelsInternal {
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0;
            (*psEnc).state_Fxx[n as usize].sCmn.prefillFlag = 1;
            n += 1
        }
    } else {
        /* Only accept input lengths that are a multiple of 10 ms */
        if nBlocksOf10ms * (*encControl).API_sampleRate != 100 * nSamplesIn || nSamplesIn < 0 {
            return -(101i32);
        }
        /* Make sure no more than one packet can be produced */
        if 1000 * nSamplesIn > (*encControl).payloadSize_ms * (*encControl).API_sampleRate {
            return -(101i32);
        }
    }
    n = 0;
    while n < (*encControl).nChannelsInternal {
        /* Force the side channel to the same rate as the mid */
        let mut force_fs_kHz: i32 = if n == 1 {
            (*psEnc).state_Fxx[0].sCmn.fs_kHz
        } else {
            0
        };
        ret = crate::src::opus_1_2_1::silk::control_codec::silk_control_encoder(
            &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize),
            encControl,
            (*psEnc).allowBandwidthSwitch,
            n,
            force_fs_kHz,
        );
        if ret != 0 {
            return ret;
        }
        if (*psEnc).state_Fxx[n as usize].sCmn.first_frame_after_reset != 0 || transition != 0 {
            i = 0;
            while i < (*psEnc).state_Fxx[0].sCmn.nFramesPerPacket {
                (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize] = 0;
                i += 1
            }
        }
        (*psEnc).state_Fxx[n as usize].sCmn.inDTX = (*psEnc).state_Fxx[n as usize].sCmn.useDTX;
        n += 1
    }
    /* Input buffering/resampling and encoding */
    nSamplesToBufferMax = 10 * nBlocksOf10ms * (*psEnc).state_Fxx[0].sCmn.fs_kHz;
    nSamplesFromInputMax = nSamplesToBufferMax * (*psEnc).state_Fxx[0].sCmn.API_fs_Hz
        / ((*psEnc).state_Fxx[0].sCmn.fs_kHz * 1000);
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>())
            .wrapping_mul(nSamplesFromInputMax as usize),
    );
    buf = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    loop {
        nSamplesToBuffer =
            (*psEnc).state_Fxx[0].sCmn.frame_length - (*psEnc).state_Fxx[0].sCmn.inputBufIx;
        nSamplesToBuffer = if nSamplesToBuffer < nSamplesToBufferMax {
            nSamplesToBuffer
        } else {
            nSamplesToBufferMax
        };
        nSamplesFromInput = nSamplesToBuffer * (*psEnc).state_Fxx[0].sCmn.API_fs_Hz
            / ((*psEnc).state_Fxx[0].sCmn.fs_kHz * 1000);
        /* Resample and write to buffer */
        if (*encControl).nChannelsAPI == 2 && (*encControl).nChannelsInternal == 2 {
            let mut id: i32 = (*psEnc).state_Fxx[0].sCmn.nFramesEncoded;
            n = 0;
            while n < nSamplesFromInput {
                *buf.offset(n as isize) = *samplesIn.offset((2 * n) as isize);
                n += 1
            }
            /* Making sure to start both resamplers from the same state when switching from mono to stereo */
            if (*psEnc).nPrevChannelsInternal == 1 && id == 0 {
                crate::stdlib::memcpy(
                    &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1isize))
                        .sCmn
                        .resampler_state
                        as *mut crate::resampler_structs_h::silk_resampler_state_struct
                        as *mut libc::c_void,
                    &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(0isize))
                        .sCmn
                        .resampler_state
                        as *mut crate::resampler_structs_h::silk_resampler_state_struct
                        as *const libc::c_void,
                    ::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>(
                    ),
                );
            }
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .resampler_state,
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(
                        ((*(*psEnc).state_Fxx.as_mut_ptr().offset(0)).sCmn.inputBufIx + 2i32)
                            as isize,
                    ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[0].sCmn.inputBufIx += nSamplesToBuffer;
            nSamplesToBuffer =
                (*psEnc).state_Fxx[1].sCmn.frame_length - (*psEnc).state_Fxx[1].sCmn.inputBufIx;
            nSamplesToBuffer =
                if nSamplesToBuffer < 10 * nBlocksOf10ms * (*psEnc).state_Fxx[1].sCmn.fs_kHz {
                    nSamplesToBuffer
                } else {
                    (10 * nBlocksOf10ms) * (*psEnc).state_Fxx[1].sCmn.fs_kHz
                };
            n = 0;
            while n < nSamplesFromInput {
                *buf.offset(n as isize) = *samplesIn.offset((2 * n + 1) as isize);
                n += 1
            }
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                    .sCmn
                    .resampler_state,
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(
                        ((*(*psEnc).state_Fxx.as_mut_ptr().offset(1)).sCmn.inputBufIx + 2i32)
                            as isize,
                    ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[1].sCmn.inputBufIx += nSamplesToBuffer
        } else if (*encControl).nChannelsAPI == 2 && (*encControl).nChannelsInternal == 1 {
            /* Combine left and right channels before resampling */
            n = 0;
            while n < nSamplesFromInput {
                sum = *samplesIn.offset((2 * n) as isize) as i32
                    + *samplesIn.offset((2 * n + 1) as isize) as i32;
                *buf.offset(n as isize) = if 1 == 1 {
                    (sum >> 1) + (sum & 1)
                } else {
                    ((sum >> 1 - 1) + 1) >> 1
                } as crate::opus_types_h::opus_int16;
                n += 1
            }
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .resampler_state,
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(
                        ((*(*psEnc).state_Fxx.as_mut_ptr().offset(0)).sCmn.inputBufIx + 2i32)
                            as isize,
                    ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            /* On the first mono frame, average the results for the two resampler states  */
            if (*psEnc).nPrevChannelsInternal == 2 && (*psEnc).state_Fxx[0].sCmn.nFramesEncoded == 0
            {
                ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                    &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                        .sCmn
                        .resampler_state,
                    &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                        .sCmn
                        .inputBuf
                        .as_mut_ptr()
                        .offset(
                            ((*(*psEnc).state_Fxx.as_mut_ptr().offset(1)).sCmn.inputBufIx + 2i32)
                                as isize,
                        ),
                    buf as *const crate::opus_types_h::opus_int16,
                    nSamplesFromInput,
                );
                n = 0;
                while n < (*psEnc).state_Fxx[0].sCmn.frame_length {
                    (*psEnc).state_Fxx[0].sCmn.inputBuf
                        [((*psEnc).state_Fxx[0].sCmn.inputBufIx + n + 2) as usize] =
                        ((*psEnc).state_Fxx[0].sCmn.inputBuf
                            [((*psEnc).state_Fxx[0].sCmn.inputBufIx + n + 2) as usize]
                            as i32
                            + (*psEnc).state_Fxx[1].sCmn.inputBuf
                                [((*psEnc).state_Fxx[1].sCmn.inputBufIx + n + 2) as usize]
                                as i32
                            >> 1) as crate::opus_types_h::opus_int16;
                    n += 1
                }
            }
            (*psEnc).state_Fxx[0].sCmn.inputBufIx += nSamplesToBuffer
        } else {
            crate::stdlib::memcpy(
                buf as *mut libc::c_void,
                samplesIn as *const libc::c_void,
                (nSamplesFromInput as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()),
            );
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .resampler_state,
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(
                        ((*(*psEnc).state_Fxx.as_mut_ptr().offset(0)).sCmn.inputBufIx + 2i32)
                            as isize,
                    ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[0].sCmn.inputBufIx += nSamplesToBuffer
        }
        samplesIn = samplesIn.offset((nSamplesFromInput * (*encControl).nChannelsAPI) as isize);
        nSamplesIn -= nSamplesFromInput;
        /* Default */
        (*psEnc).allowBandwidthSwitch = 0;
        /* Silk encoder */
        if !((*psEnc).state_Fxx[0].sCmn.inputBufIx >= (*psEnc).state_Fxx[0].sCmn.frame_length) {
            break;
        }
        /* Enough data in input buffer, so encode */
        /* Deal with LBRR data */
        if (*psEnc).state_Fxx[0].sCmn.nFramesEncoded == 0 && prefillFlag == 0 {
            /* Create space at start of payload for VAD and FEC flags */
            let mut iCDF: [u8; 2] = [0, 0];
            iCDF[0] = (256i32
                - (256
                    >> ((*psEnc).state_Fxx[0].sCmn.nFramesPerPacket + 1)
                        * (*encControl).nChannelsInternal)) as u8;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(psRangeEnc, 0, iCDF.as_mut_ptr(), 8);
            /* Encode any LBRR data from previous packet */
            /* Encode LBRR flags */
            n = 0;
            while n < (*encControl).nChannelsInternal {
                LBRR_symbol = 0;
                i = 0;
                while i < (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket {
                    LBRR_symbol |= (((*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize]
                        as crate::opus_types_h::opus_uint32)
                        << i) as crate::opus_types_h::opus_int32;
                    i += 1
                }
                (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flag =
                    if LBRR_symbol > 0 { 1i32 } else { 0 } as i8;
                if LBRR_symbol != 0 && (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket > 1 {
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                        psRangeEnc,
                        LBRR_symbol - 1i32,
                        crate::src::opus_1_2_1::silk::tables_other::silk_LBRR_flags_iCDF_ptr
                            [((*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket - 2i32)
                                as usize],
                        8u32,
                    );
                }
                n += 1
            }
            /* Code LBRR indices and excitation signals */
            i = 0;
            while i < (*psEnc).state_Fxx[0].sCmn.nFramesPerPacket {
                n = 0;
                while n < (*encControl).nChannelsInternal {
                    if (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize] != 0 {
                        let mut condCoding: i32 = 0;
                        if (*encControl).nChannelsInternal == 2 && n == 0 {
                            crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_pred(psRangeEnc,
                                                    (*psEnc).sStereo.predIx[i
                                                                                as
                                                                                usize].as_mut_ptr());
                            /* For LBRR data there's no need to code the mid-only flag if the side-channel LBRR flag is set */
                            if (*psEnc).state_Fxx[1].sCmn.LBRR_flags[i as usize] == 0 {
                                crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_mid_only(psRangeEnc,
                                                            (*psEnc).sStereo.mid_only_flags[i
                                                                                                as
                                                                                                usize]);
                            }
                        }
                        /* Use conditional coding if previous frame available */
                        if i > 0
                            && (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[(i - 1) as usize] != 0
                        {
                            condCoding = 2
                        } else {
                            condCoding = 0
                        }
                        crate::src::opus_1_2_1::silk::encode_indices::silk_encode_indices(
                            &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize)).sCmn,
                            psRangeEnc,
                            i,
                            1,
                            condCoding,
                        );
                        crate::src::opus_1_2_1::silk::encode_pulses::silk_encode_pulses(
                            psRangeEnc,
                            (*psEnc).state_Fxx[n as usize].sCmn.indices_LBRR[i as usize].signalType
                                as i32,
                            (*psEnc).state_Fxx[n as usize].sCmn.indices_LBRR[i as usize]
                                .quantOffsetType as i32,
                            (*psEnc).state_Fxx[n as usize].sCmn.pulses_LBRR[i as usize]
                                .as_mut_ptr(),
                            (*psEnc).state_Fxx[n as usize].sCmn.frame_length,
                        );
                    }
                    n += 1
                }
                i += 1
            }
            /* Reset LBRR flags */
            n = 0;
            while n < (*encControl).nChannelsInternal {
                crate::stdlib::memset(
                    (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags.as_mut_ptr()
                        as *mut libc::c_void,
                    0,
                    ::std::mem::size_of::<[i32; 3]>(),
                );
                n += 1
            }
            (*psEnc).nBitsUsedLBRR = ec_tell(psRangeEnc)
        }
        crate::src::opus_1_2_1::silk::HP_variable_cutoff::silk_HP_variable_cutoff(
            (*psEnc).state_Fxx.as_mut_ptr(),
        );
        /* Total target bits for packet */
        nBits = (*encControl).bitRate * (*encControl).payloadSize_ms / 1000;
        /* Subtract bits used for LBRR */
        if prefillFlag == 0 {
            nBits -= (*psEnc).nBitsUsedLBRR
        }
        /* Divide by number of uncoded frames left in packet */
        nBits = nBits / (*psEnc).state_Fxx[0].sCmn.nFramesPerPacket;
        /* Convert to bits/second */
        if (*encControl).payloadSize_ms == 10 {
            TargetRate_bps =
                nBits as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32 * 100
        } else {
            TargetRate_bps =
                nBits as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32 * 50
        }
        /* Subtract fraction of bits in excess of target in previous frames and packets */
        TargetRate_bps -= (*psEnc).nBitsExceeded * 1000 / 500;
        if prefillFlag == 0 && (*psEnc).state_Fxx[0].sCmn.nFramesEncoded > 0 {
            /* Compare actual vs target bits so far in this packet */
            let mut bitsBalance: crate::opus_types_h::opus_int32 = ec_tell(psRangeEnc)
                - (*psEnc).nBitsUsedLBRR
                - nBits * (*psEnc).state_Fxx[0].sCmn.nFramesEncoded;
            TargetRate_bps -= bitsBalance * 1000 / 500
        }
        /* Never exceed input bitrate */
        TargetRate_bps = if (*encControl).bitRate > 5000 {
            if TargetRate_bps > (*encControl).bitRate {
                (*encControl).bitRate
            } else if TargetRate_bps < 5000 {
                5000
            } else {
                TargetRate_bps
            }
        } else if TargetRate_bps > 5000 {
            5000
        } else if TargetRate_bps < (*encControl).bitRate {
            (*encControl).bitRate
        } else {
            TargetRate_bps
        };
        /* Convert Left/Right to Mid/Side */
        if (*encControl).nChannelsInternal == 2 {
            crate::src::opus_1_2_1::silk::stereo_LR_to_MS::silk_stereo_LR_to_MS(
                &mut (*psEnc).sStereo,
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(2),
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(2),
                (*psEnc).sStereo.predIx[(*psEnc).state_Fxx[0].sCmn.nFramesEncoded as usize]
                    .as_mut_ptr(),
                &mut *(*psEnc).sStereo.mid_only_flags.as_mut_ptr().offset(
                    (*(*psEnc).state_Fxx.as_mut_ptr().offset(0))
                        .sCmn
                        .nFramesEncoded as isize,
                ),
                MStargetRates_bps.as_mut_ptr(),
                TargetRate_bps,
                (*psEnc).state_Fxx[0].sCmn.speech_activity_Q8,
                (*encControl).toMono,
                (*psEnc).state_Fxx[0].sCmn.fs_kHz,
                (*psEnc).state_Fxx[0].sCmn.frame_length,
            );
            if (*psEnc).sStereo.mid_only_flags[(*psEnc).state_Fxx[0].sCmn.nFramesEncoded as usize]
                as i32
                == 0
            {
                /* Reset side channel encoder memory for first frame with side coding */
                if (*psEnc).prev_decode_only_middle == 1 {
                    crate::stdlib::memset(
                        &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1)).sShape
                            as *mut crate::structs_FLP_h::silk_shape_state_FLP
                            as *mut libc::c_void,
                        0,
                        ::std::mem::size_of::<crate::structs_FLP_h::silk_shape_state_FLP>(),
                    );
                    crate::stdlib::memset(
                        &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1)).sCmn.sNSQ
                            as *mut crate::structs_h::silk_nsq_state
                            as *mut libc::c_void,
                        0,
                        ::std::mem::size_of::<crate::structs_h::silk_nsq_state>(),
                    );
                    crate::stdlib::memset(
                        (*psEnc).state_Fxx[1].sCmn.prev_NLSFq_Q15.as_mut_ptr() as *mut libc::c_void,
                        0,
                        ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 16]>(),
                    );
                    crate::stdlib::memset(
                        &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(1))
                            .sCmn
                            .sLP
                            .In_LP_State
                            as *mut [crate::opus_types_h::opus_int32; 2]
                            as *mut libc::c_void,
                        0,
                        ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>(),
                    );
                    (*psEnc).state_Fxx[1].sCmn.prevLag = 100;
                    (*psEnc).state_Fxx[1].sCmn.sNSQ.lagPrev = 100;
                    (*psEnc).state_Fxx[1].sShape.LastGainIndex = 10;
                    (*psEnc).state_Fxx[1].sCmn.prevSignalType = 0;
                    (*psEnc).state_Fxx[1].sCmn.sNSQ.prev_gain_Q16 = 65536;
                    (*psEnc).state_Fxx[1].sCmn.first_frame_after_reset = 1
                }
                crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_do_VAD_FLP(
                    &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(1isize),
                );
            } else {
                (*psEnc).state_Fxx[1].sCmn.VAD_flags
                    [(*psEnc).state_Fxx[0].sCmn.nFramesEncoded as usize] = 0
            }
            if prefillFlag == 0 {
                crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_pred(
                    psRangeEnc,
                    (*psEnc).sStereo.predIx[(*psEnc).state_Fxx[0].sCmn.nFramesEncoded as usize]
                        .as_mut_ptr(),
                );
                if (*psEnc).state_Fxx[1].sCmn.VAD_flags
                    [(*psEnc).state_Fxx[0].sCmn.nFramesEncoded as usize] as i32
                    == 0
                {
                    crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_mid_only(
                        psRangeEnc,
                        (*psEnc).sStereo.mid_only_flags
                            [(*psEnc).state_Fxx[0usize].sCmn.nFramesEncoded as usize],
                    );
                }
            }
        } else {
            /* Buffering */
            crate::stdlib::memcpy(
                (*psEnc).state_Fxx[0].sCmn.inputBuf.as_mut_ptr() as *mut libc::c_void,
                (*psEnc).sStereo.sMid.as_mut_ptr() as *const libc::c_void,
                (2usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()),
            );
            crate::stdlib::memcpy(
                (*psEnc).sStereo.sMid.as_mut_ptr() as *mut libc::c_void,
                &mut *(*(*psEnc).state_Fxx.as_mut_ptr().offset(0isize))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(
                        (*(*psEnc).state_Fxx.as_mut_ptr().offset(0isize))
                            .sCmn
                            .frame_length as isize,
                    ) as *mut crate::opus_types_h::opus_int16
                    as *const libc::c_void,
                (2usize).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()),
            );
        }
        crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_do_VAD_FLP(
            &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(0),
        );
        /* Encode */
        n = 0;
        while n < (*encControl).nChannelsInternal {
            let mut maxBits: i32 = 0;
            let mut useCBR: i32 = 0;
            /* Handling rate constraints */
            maxBits = (*encControl).maxBits;
            if tot_blocks == 2 && curr_block == 0 {
                maxBits = maxBits * 3 / 5
            } else if tot_blocks == 3 {
                if curr_block == 0 {
                    maxBits = maxBits * 2 / 5
                } else if curr_block == 1 {
                    maxBits = maxBits * 3 / 4
                }
            }
            useCBR = ((*encControl).useCBR != 0 && curr_block == tot_blocks - 1) as i32;
            if (*encControl).nChannelsInternal == 1 {
                channelRate_bps = TargetRate_bps
            } else {
                channelRate_bps = MStargetRates_bps[n as usize];
                if n == 0 && MStargetRates_bps[1] > 0 {
                    useCBR = 0;
                    /* Give mid up to 1/2 of the max bits for that frame */
                    maxBits -= (*encControl).maxBits / (tot_blocks * 2)
                }
            }
            if channelRate_bps > 0 {
                let mut condCoding_0: i32 = 0;
                crate::src::opus_1_2_1::silk::control_SNR::silk_control_SNR(
                    &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize)).sCmn,
                    channelRate_bps,
                );
                /* Use independent coding if no previous frame available */
                if (*psEnc).state_Fxx[0].sCmn.nFramesEncoded - n <= 0 {
                    condCoding_0 = 0
                } else if n > 0 && (*psEnc).prev_decode_only_middle != 0 {
                    /* If we skipped a side frame in this packet, we don't
                    need LTP scaling; the LTP state is well-defined. */
                    condCoding_0 = 1
                } else {
                    condCoding_0 = 2
                }
                ret = crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_frame_FLP(
                    &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize),
                    nBytesOut,
                    psRangeEnc,
                    condCoding_0,
                    maxBits,
                    useCBR,
                );
                (ret) != 0i32;
            }
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0;
            (*psEnc).state_Fxx[n as usize].sCmn.inputBufIx = 0;
            (*psEnc).state_Fxx[n as usize].sCmn.nFramesEncoded += 1;
            n += 1
        }
        (*psEnc).prev_decode_only_middle = (*psEnc).sStereo.mid_only_flags
            [((*psEnc).state_Fxx[0].sCmn.nFramesEncoded - 1) as usize]
            as i32;
        /* Insert VAD and FEC flags at beginning of bitstream */
        if *nBytesOut > 0
            && (*psEnc).state_Fxx[0].sCmn.nFramesEncoded
                == (*psEnc).state_Fxx[0].sCmn.nFramesPerPacket
        {
            flags = 0;
            n = 0;
            while n < (*encControl).nChannelsInternal {
                i = 0;
                while i < (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket {
                    flags = ((flags as crate::opus_types_h::opus_uint32) << 1)
                        as crate::opus_types_h::opus_int32;
                    flags |= (*psEnc).state_Fxx[n as usize].sCmn.VAD_flags[i as usize] as i32;
                    i += 1
                }
                flags = ((flags as crate::opus_types_h::opus_uint32) << 1)
                    as crate::opus_types_h::opus_int32;
                flags |= (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flag as i32;
                n += 1
            }
            if prefillFlag == 0 {
                crate::src::opus_1_2_1::celt::entenc::ec_enc_patch_initial_bits(
                    psRangeEnc,
                    flags as u32,
                    (((*psEnc).state_Fxx[0usize].sCmn.nFramesPerPacket + 1i32)
                        * (*encControl).nChannelsInternal) as u32,
                );
            }
            /* Return zero bytes if all channels DTXed */
            if (*psEnc).state_Fxx[0].sCmn.inDTX != 0
                && ((*encControl).nChannelsInternal == 1 || (*psEnc).state_Fxx[1].sCmn.inDTX != 0)
            {
                *nBytesOut = 0
            }
            (*psEnc).nBitsExceeded += *nBytesOut * 8;
            (*psEnc).nBitsExceeded -= (*encControl).bitRate * (*encControl).payloadSize_ms / 1000;
            (*psEnc).nBitsExceeded = if 0 > 10000 {
                if (*psEnc).nBitsExceeded > 0 {
                    0
                } else if (*psEnc).nBitsExceeded < 10000 {
                    10000
                } else {
                    (*psEnc).nBitsExceeded
                }
            } else if (*psEnc).nBitsExceeded > 10000 {
                10000
            } else if (*psEnc).nBitsExceeded < 0 {
                0
            } else {
                (*psEnc).nBitsExceeded
            };
            /* Update flag indicating if bandwidth switching is allowed */
            speech_act_thr_for_switch_Q8 = (((0.05 * ((1i64) << 8) as f32) as f64 + 0.5)
                as crate::opus_types_h::opus_int32
                as i64
                + ((((1f32 - 0.05) / 5000f32 * ((1i64) << 16 + 8) as f32) as f64 + 0.5)
                    as crate::opus_types_h::opus_int32 as i64
                    * (*psEnc).timeSinceSwitchAllowed_ms as crate::opus_types_h::opus_int16 as i64
                    >> 16))
                as crate::opus_types_h::opus_int32;
            if (*psEnc).state_Fxx[0].sCmn.speech_activity_Q8 < speech_act_thr_for_switch_Q8 {
                (*psEnc).allowBandwidthSwitch = 1;
                (*psEnc).timeSinceSwitchAllowed_ms = 0
            } else {
                (*psEnc).allowBandwidthSwitch = 0;
                (*psEnc).timeSinceSwitchAllowed_ms += (*encControl).payloadSize_ms
            }
        }
        if nSamplesIn == 0 {
            break;
        }
        curr_block += 1
    }
    (*psEnc).nPrevChannelsInternal = (*encControl).nChannelsInternal;
    (*encControl).allowBandwidthSwitch = (*psEnc).allowBandwidthSwitch;
    (*encControl).inWBmodeWithoutVariableLP = ((*psEnc).state_Fxx[0].sCmn.fs_kHz == 16
        && (*psEnc).state_Fxx[0].sCmn.sLP.mode == 0)
        as i32;
    (*encControl).internalSampleRate = (*psEnc).state_Fxx[0].sCmn.fs_kHz
        as crate::opus_types_h::opus_int16
        as crate::opus_types_h::opus_int32
        * 1000;
    (*encControl).stereoWidth_Q14 = if (*encControl).toMono != 0 {
        0
    } else {
        (*psEnc).sStereo.smth_width_Q14 as i32
    };
    if prefillFlag != 0 {
        (*encControl).payloadSize_ms = tmp_payloadSize_ms;
        (*encControl).complexity = tmp_complexity;
        n = 0;
        while n < (*encControl).nChannelsInternal {
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0;
            (*psEnc).state_Fxx[n as usize].sCmn.prefillFlag = 0;
            n += 1
        }
    }
    (*encControl).signalType = (*psEnc).state_Fxx[0].sCmn.indices.signalType as i32;
    (*encControl).offset = crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10
        [((*psEnc).state_Fxx[0].sCmn.indices.signalType as i32 >> 1) as usize]
        [(*psEnc).state_Fxx[0].sCmn.indices.quantOffsetType as usize]
        as i32;
    return ret;
}
