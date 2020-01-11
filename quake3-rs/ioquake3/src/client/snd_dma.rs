use ::libc;

pub mod q_shared_h {
    /*
    ===========================================================================
    Copyright (C) 1999-2005 Id Software, Inc.

    This file is part of Quake III Arena source code.

    Quake III Arena source code is free software; you can redistribute it
    and/or modify it under the terms of the GNU General Public License as
    published by the Free Software Foundation; either version 2 of the License,
    or (at your option) any later version.

    Quake III Arena source code is distributed in the hope that it will be
    useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Quake III Arena source code; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
    ===========================================================================
    */
    //
    // q_shared.h -- included first by ALL program modules.
    // A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
    // You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
     VM Considerations

     The VM can not use the standard system headers because we aren't really
     using the compiler they were meant for.  We use bg_lib.h which contains
     prototypes for the functions we define for our own use in bg_lib.c.

     When writing mods, please add needed headers HERE, do not start including
     stuff like <stdio.h> in the various .c files that make up each of the VMs
     since you will be including system headers files can will have issues.

     Remember, if you use a C library function that is not defined in bg_lib.c,
     you will have to add your own version for support in the VM.

    **********************************************************************/
    //=============================================================

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    //=====================================================================
    // in order from highest priority to lowest
    // if none of the catchers are active, bound key strings will be executed
    // sound channels
    // channel 0 never willingly overrides
    // other channels will allways override a playing sound on that channel

    // announcer voices, etc
    // chat messages, etc

    // menu sounds, etc

    // playing a cinematic or a static pic, not connected to a server
    // game views should be displayed

    // got gamestate, waiting for first frame

    // only during cgame initialization, never during main loop

    // netchan_t established, getting gamestate

    // sending challenge packets to the server

    // sending request packets to the server

    // not used any more, was checking cd key

    // not talking to a server

    #[inline]

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0) * *v.offset(0)
            + *v.offset(1) * *v.offset(1)
            + *v.offset(2) * *v.offset(2);
    }
    #[inline]

    pub unsafe extern "C" fn DistanceSquared(
        mut p1: *const crate::src::qcommon::q_shared::vec_t,
        mut p2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        v[0] = *p2.offset(0) - *p1.offset(0);
        v[1] = *p2.offset(1) - *p1.offset(1);
        v[2] = *p2.offset(2) - *p1.offset(2);
        return v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    }

    // __Q_SHARED_H
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::serverInfo_t;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::snd_local_h::adpcm_state;
pub use crate::snd_local_h::adpcm_state_t;
pub use crate::snd_local_h::channel_t;
pub use crate::snd_local_h::dma_t;
pub use crate::snd_local_h::loopSound_s;
pub use crate::snd_local_h::loopSound_t;
pub use crate::snd_local_h::portable_samplepair_t;
pub use crate::snd_local_h::sfx_s;
pub use crate::snd_local_h::sfx_t;
pub use crate::snd_local_h::sndBuffer;
pub use crate::snd_local_h::sndBuffer_s;
pub use crate::snd_local_h::soundInterface_t;
pub use crate::src::client::cl_avi::CL_VideoRecording;
pub use crate::src::client::cl_main::cl_aviFrameRate;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::snd_codec::snd_codec_s;
pub use crate::src::client::snd_codec::snd_codec_t;
pub use crate::src::client::snd_codec::snd_info_s;
pub use crate::src::client::snd_codec::snd_info_t;
pub use crate::src::client::snd_codec::snd_stream_s;
pub use crate::src::client::snd_codec::snd_stream_t;
pub use crate::src::client::snd_codec::S_CodecCloseStream;
pub use crate::src::client::snd_codec::S_CodecOpenStream;
pub use crate::src::client::snd_codec::S_CodecReadStream;
pub use crate::src::client::snd_codec::CODEC_CLOSE;
pub use crate::src::client::snd_codec::CODEC_LOAD;
pub use crate::src::client::snd_codec::CODEC_OPEN;
pub use crate::src::client::snd_codec::CODEC_READ;
pub use crate::src::client::snd_dma::q_shared_h::DistanceSquared;
pub use crate::src::client::snd_dma::q_shared_h::VectorLengthSquared;
pub use crate::src::client::snd_main::s_doppler;
pub use crate::src::client::snd_main::s_musicVolume;
pub use crate::src::client::snd_main::s_muted;
pub use crate::src::client::snd_main::s_volume;
pub use crate::src::client::snd_mem::SND_free;
pub use crate::src::client::snd_mem::SND_malloc;
pub use crate::src::client::snd_mem::SND_setup;
pub use crate::src::client::snd_mem::SND_shutdown;
pub use crate::src::client::snd_mem::S_LoadSound;
pub use crate::src::client::snd_mix::S_PaintChannels;
use crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
use crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommand;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Milliseconds;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorRotate;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::sdl::sdl_snd::SNDDMA_AvailableCaptureSamples;
pub use crate::src::sdl::sdl_snd::SNDDMA_BeginPainting;
pub use crate::src::sdl::sdl_snd::SNDDMA_Capture;
pub use crate::src::sdl::sdl_snd::SNDDMA_GetDMAPos;
pub use crate::src::sdl::sdl_snd::SNDDMA_Init;
pub use crate::src::sdl::sdl_snd::SNDDMA_MasterGain;
pub use crate::src::sdl::sdl_snd::SNDDMA_Shutdown;
pub use crate::src::sdl::sdl_snd::SNDDMA_StartCapture;
pub use crate::src::sdl::sdl_snd::SNDDMA_StopCapture;
pub use crate::src::sdl::sdl_snd::SNDDMA_Submit;

pub use crate::src::client::snd_dma::ctype_h::tolower;
use crate::src::client::snd_mem::S_DisplayFreeMemory;
pub use crate::stdlib::__ctype_tolower_loc;
use crate::stdlib::memset;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
#[no_mangle]

pub static mut s_backgroundStream: *mut crate::src::client::snd_codec::snd_stream_t =
    0 as *mut crate::src::client::snd_codec::snd_stream_t;

static mut s_backgroundLoop: [i8; 64] = [0; 64];
#[no_mangle]

pub static mut s_channels: [crate::snd_local_h::channel_t; 96] = [crate::snd_local_h::channel_t {
    allocTime: 0,
    startSample: 0,
    entnum: 0,
    entchannel: 0,
    leftvol: 0,
    rightvol: 0,
    master_vol: 0,
    dopplerScale: 0.,
    oldDopplerScale: 0.,
    origin: [0.; 3],
    fixed_origin: crate::src::qcommon::q_shared::qfalse,
    thesfx: 0 as *mut crate::snd_local_h::sfx_t,
    doppler: crate::src::qcommon::q_shared::qfalse,
    fullVolume: crate::src::qcommon::q_shared::qfalse,
}; 96];
#[no_mangle]

pub static mut loop_channels: [crate::snd_local_h::channel_t; 96] =
    [crate::snd_local_h::channel_t {
        allocTime: 0,
        startSample: 0,
        entnum: 0,
        entchannel: 0,
        leftvol: 0,
        rightvol: 0,
        master_vol: 0,
        dopplerScale: 0.,
        oldDopplerScale: 0.,
        origin: [0.; 3],
        fixed_origin: crate::src::qcommon::q_shared::qfalse,
        thesfx: 0 as *mut crate::snd_local_h::sfx_t,
        doppler: crate::src::qcommon::q_shared::qfalse,
        fullVolume: crate::src::qcommon::q_shared::qfalse,
    }; 96];
#[no_mangle]

pub static mut numLoopChannels: i32 = 0;

static mut s_soundStarted: i32 = 0;

static mut s_soundMuted: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut dma: crate::snd_local_h::dma_t = crate::snd_local_h::dma_t {
    channels: 0,
    samples: 0,
    fullsamples: 0,
    submission_chunk: 0,
    samplebits: 0,
    isfloat: 0,
    speed: 0,
    buffer: 0 as *mut crate::src::qcommon::q_shared::byte,
};

static mut listener_number: i32 = 0;

static mut listener_origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];

static mut listener_axis: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
#[no_mangle]

pub static mut s_soundtime: i32 = 0;
// sample PAIRS
#[no_mangle]

pub static mut s_paintedtime: i32 = 0;
#[no_mangle]

pub static mut s_knownSfx: [crate::snd_local_h::sfx_t; 4096] = [crate::snd_local_h::sfx_t {
    soundData: 0 as *mut crate::snd_local_h::sndBuffer,
    defaultSound: crate::src::qcommon::q_shared::qfalse,
    inMemory: crate::src::qcommon::q_shared::qfalse,
    soundCompressed: crate::src::qcommon::q_shared::qfalse,
    soundCompressionMethod: 0,
    soundLength: 0,
    soundChannels: 0,
    soundName: [0; 64],
    lastTimeUsed: 0,
    next: 0 as *mut crate::snd_local_h::sfx_s,
}; 4096];
#[no_mangle]

pub static mut s_numSfx: i32 = 0;

static mut sfxHash: [*mut crate::snd_local_h::sfx_t; 128] =
    [0 as *mut crate::snd_local_h::sfx_t; 128];
#[no_mangle]

pub static mut s_testsound: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_show: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_mixahead: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_mixPreStep: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;

static mut loopSounds: [crate::snd_local_h::loopSound_t; 1024] = [crate::snd_local_h::loopSound_t {
    origin: [0.; 3],
    velocity: [0.; 3],
    sfx: 0 as *mut crate::snd_local_h::sfx_t,
    mergeFrame: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    kill: crate::src::qcommon::q_shared::qfalse,
    doppler: crate::src::qcommon::q_shared::qfalse,
    dopplerScale: 0.,
    oldDopplerScale: 0.,
    framenum: 0,
}; 1024];

static mut freelist: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
#[no_mangle]

pub static mut s_rawend: [i32; 129] = [0; 129];
#[no_mangle]

pub static mut s_rawsamples: [[crate::snd_local_h::portable_samplepair_t; 16384]; 129] =
    [[crate::snd_local_h::portable_samplepair_t { left: 0, right: 0 }; 16384]; 129];
// ====================================================================
// User-setable variables
// ====================================================================
#[no_mangle]

pub unsafe extern "C" fn S_Base_SoundInfo() {
    crate::src::qcommon::common::Com_Printf(
        b"----- Sound Info -----\n\x00" as *const u8 as *const i8,
    );
    if s_soundStarted == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"sound system not started\n\x00" as *const u8 as *const i8,
        );
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"%5d channels\n\x00" as *const u8 as *const i8,
            dma.channels,
        );
        crate::src::qcommon::common::Com_Printf(
            b"%5d samples\n\x00" as *const u8 as *const i8,
            dma.samples,
        );
        crate::src::qcommon::common::Com_Printf(
            b"%5d samplebits (%s)\n\x00" as *const u8 as *const i8,
            dma.samplebits,
            if dma.isfloat != 0 {
                b"float\x00" as *const u8 as *const i8
            } else {
                b"int\x00" as *const u8 as *const i8
            },
        );
        crate::src::qcommon::common::Com_Printf(
            b"%5d submission_chunk\n\x00" as *const u8 as *const i8,
            dma.submission_chunk,
        );
        crate::src::qcommon::common::Com_Printf(
            b"%5d speed\n\x00" as *const u8 as *const i8,
            dma.speed,
        );
        crate::src::qcommon::common::Com_Printf(
            b"%p dma buffer\n\x00" as *const u8 as *const i8,
            dma.buffer,
        );
        if !s_backgroundStream.is_null() {
            crate::src::qcommon::common::Com_Printf(
                b"Background file: %s\n\x00" as *const u8 as *const i8,
                s_backgroundLoop.as_mut_ptr(),
            );
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"No background file.\n\x00" as *const u8 as *const i8,
            );
        }
    }
    crate::src::qcommon::common::Com_Printf(
        b"----------------------\n\x00" as *const u8 as *const i8,
    );
}

unsafe extern "C" fn S_Base_StartCapture() {
    crate::src::sdl::sdl_snd::SNDDMA_StartCapture();
}

unsafe extern "C" fn S_Base_AvailableCaptureSamples() -> i32 {
    return crate::src::sdl::sdl_snd::SNDDMA_AvailableCaptureSamples();
}

unsafe extern "C" fn S_Base_Capture(
    mut samples: i32,
    mut data: *mut crate::src::qcommon::q_shared::byte,
) {
    crate::src::sdl::sdl_snd::SNDDMA_Capture(samples, data);
}

unsafe extern "C" fn S_Base_StopCapture() {
    crate::src::sdl::sdl_snd::SNDDMA_StopCapture();
}

unsafe extern "C" fn S_Base_MasterGain(mut val: f32) {
    crate::src::sdl::sdl_snd::SNDDMA_MasterGain(val);
}
/*
=================
S_Base_SoundList
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_SoundList() {
    let mut i: i32 = 0;
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    let mut size: i32 = 0;
    let mut total: i32 = 0;
    let mut type_0: [[i8; 16]; 4] = [[0; 16]; 4];
    let mut mem: [[i8; 16]; 2] = [[0; 16]; 2];
    crate::stdlib::strcpy(
        type_0[0].as_mut_ptr(),
        b"16bit\x00" as *const u8 as *const i8,
    );
    crate::stdlib::strcpy(
        type_0[1].as_mut_ptr(),
        b"adpcm\x00" as *const u8 as *const i8,
    );
    crate::stdlib::strcpy(
        type_0[2].as_mut_ptr(),
        b"daub4\x00" as *const u8 as *const i8,
    );
    crate::stdlib::strcpy(
        type_0[3].as_mut_ptr(),
        b"mulaw\x00" as *const u8 as *const i8,
    );
    crate::stdlib::strcpy(
        mem[0].as_mut_ptr(),
        b"paged out\x00" as *const u8 as *const i8,
    );
    crate::stdlib::strcpy(
        mem[1].as_mut_ptr(),
        b"resident \x00" as *const u8 as *const i8,
    );
    total = 0;
    sfx = s_knownSfx.as_mut_ptr();
    i = 0;
    while i < s_numSfx {
        size = (*sfx).soundLength;
        total += size;
        crate::src::qcommon::common::Com_Printf(
            b"%6i[%s] : %s[%s]\n\x00" as *const u8 as *const i8,
            size,
            type_0[(*sfx).soundCompressionMethod as usize].as_mut_ptr(),
            (*sfx).soundName.as_mut_ptr(),
            mem[(*sfx).inMemory as usize].as_mut_ptr(),
        );
        i += 1;
        sfx = sfx.offset(1)
    }
    crate::src::qcommon::common::Com_Printf(
        b"Total resident: %i\n\x00" as *const u8 as *const i8,
        total,
    );
    crate::src::client::snd_mem::S_DisplayFreeMemory();
}
#[no_mangle]

pub unsafe extern "C" fn S_ChannelFree(mut v: *mut crate::snd_local_h::channel_t) {
    (*v).thesfx = 0 as *mut crate::snd_local_h::sfx_t;
    let ref mut fresh0 = *(v as *mut *mut crate::snd_local_h::channel_t);
    *fresh0 = freelist;
    freelist = v;
}
#[no_mangle]

pub unsafe extern "C" fn S_ChannelMalloc() -> *mut crate::snd_local_h::channel_t {
    let mut v: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    if freelist.is_null() {
        return 0 as *mut crate::snd_local_h::channel_t;
    }
    v = freelist;
    freelist = *(freelist as *mut *mut crate::snd_local_h::channel_t);
    (*v).allocTime = crate::src::qcommon::common::Com_Milliseconds();
    return v;
}
#[no_mangle]

pub unsafe extern "C" fn S_ChannelSetup() {
    let mut p: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut q: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    // clear all the sounds so they don't
    crate::stdlib::memset(
        s_channels.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[crate::snd_local_h::channel_t; 96]>(),
    );
    p = s_channels.as_mut_ptr();
    q = p.offset(96);
    loop {
        q = q.offset(-1);
        if !(q > p) {
            break;
        }
        let ref mut fresh1 = *(q as *mut *mut crate::snd_local_h::channel_t);
        *fresh1 = q.offset(-(1))
    }
    let ref mut fresh2 = *(q as *mut *mut crate::snd_local_h::channel_t);
    *fresh2 = 0 as *mut crate::snd_local_h::channel_t;
    freelist = p.offset(96).offset(-(1));
    crate::src::qcommon::common::Com_DPrintf(
        b"Channel memory manager started\n\x00" as *const u8 as *const i8,
    );
}
// =======================================================================
// Load a sound
// =======================================================================
/*
================
return a hash value for the sfx name
================
*/

unsafe extern "C" fn S_HashSFXName(mut name: *const i8) -> isize {
    let mut i: i32 = 0; // don't include extension
    let mut hash: isize = 0; // damn path names
    let mut letter: i8 = 0;
    hash = 0;
    i = 0;
    while *name.offset(i as isize) as i32 != '\u{0}' as i32 {
        letter = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *name.offset(i as isize) as i32;
                    __res = if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*name.offset(i as isize) as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc())
                    .offset(*name.offset(i as isize) as i32 as isize)
            }
            __res
        }) as i8;
        if letter as i32 == '.' as i32 {
            break;
        }
        if letter as i32 == '\\' as i32 {
            letter = '/' as i8
        }
        hash += letter as isize * (i + 119) as isize;
        i += 1
    }
    hash &= (128i32 - 1) as isize;
    return hash;
}
/*
==================
S_FindName

Will allocate a new sfx if it isn't found
==================
*/

unsafe extern "C" fn S_FindName(mut name: *const i8) -> *mut crate::snd_local_h::sfx_t {
    let mut i: i32 = 0;
    let mut hash: i32 = 0;
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    if name.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Sound name is NULL\x00" as *const u8 as *const i8,
        );
    }
    if *name.offset(0) == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: Sound name is empty\n\x00" as *const u8 as *const i8,
        );
        return 0 as *mut crate::snd_local_h::sfx_t;
    }
    if crate::stdlib::strlen(name) >= 64 {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: Sound name is too long: %s\n\x00" as *const u8 as *const i8,
            name,
        );
        return 0 as *mut crate::snd_local_h::sfx_t;
    }
    if *name.offset(0) as i32 == '*' as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: Tried to load player sound directly: %s\n\x00" as *const u8 as *const i8,
            name,
        );
        return 0 as *mut crate::snd_local_h::sfx_t;
    }
    hash = S_HashSFXName(name) as i32;
    sfx = sfxHash[hash as usize];
    // see if already loaded
    while !sfx.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp((*sfx).soundName.as_mut_ptr(), name) == 0 {
            return sfx;
        }
        sfx = (*sfx).next
    }
    // find a free sfx
    i = 0;
    while i < s_numSfx {
        if s_knownSfx[i as usize].soundName[0] == 0 {
            break;
        }
        i += 1
    }
    if i == s_numSfx {
        if s_numSfx == 4096 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                b"S_FindName: out of sfx_t\x00" as *const u8 as *const i8,
            );
        }
        s_numSfx += 1
    }
    sfx = &mut *s_knownSfx.as_mut_ptr().offset(i as isize) as *mut crate::snd_local_h::sfx_t;
    crate::stdlib::memset(
        sfx as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::snd_local_h::sfx_t>(),
    );
    crate::stdlib::strcpy((*sfx).soundName.as_mut_ptr(), name);
    (*sfx).next = sfxHash[hash as usize];
    sfxHash[hash as usize] = sfx;
    return sfx;
}
/*
=================
S_DefaultSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_DefaultSound(mut sfx: *mut crate::snd_local_h::sfx_t) {
    let mut i: i32 = 0;
    (*sfx).soundLength = 512;
    (*sfx).soundData = crate::src::client::snd_mem::SND_malloc();
    (*(*sfx).soundData).next = 0 as *mut crate::snd_local_h::sndBuffer_s;
    i = 0;
    while i < (*sfx).soundLength {
        (*(*sfx).soundData).sndChunk[i as usize] = i as i16;
        i += 1
    }
}
/*
===================
S_DisableSounds

Disables sounds until the next S_BeginRegistration.
This is called when the hunk is cleared and the sounds
are no longer valid.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_DisableSounds() {
    S_Base_StopAllSounds();
    s_soundMuted = crate::src::qcommon::q_shared::qtrue;
}
/*
==================
S_RegisterSound

Creates a default buzz sound if the file can't be loaded
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_RegisterSound(
    mut name: *const i8,
    mut compressed: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    compressed = crate::src::qcommon::q_shared::qfalse;
    if s_soundStarted == 0 {
        return 0i32;
    }
    sfx = S_FindName(name);
    if sfx.is_null() {
        return 0i32;
    }
    if !(*sfx).soundData.is_null() {
        if (*sfx).defaultSound as u64 != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"^3WARNING: could not find %s - using default\n\x00" as *const u8 as *const i8,
                (*sfx).soundName.as_mut_ptr(),
            );
            return 0i32;
        }
        return sfx.wrapping_offset_from(s_knownSfx.as_mut_ptr())
            as crate::src::qcommon::q_shared::sfxHandle_t;
    }
    (*sfx).inMemory = crate::src::qcommon::q_shared::qfalse;
    (*sfx).soundCompressed = compressed;
    S_memoryLoad(sfx);
    if (*sfx).defaultSound as u64 != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: could not find %s - using default\n\x00" as *const u8 as *const i8,
            (*sfx).soundName.as_mut_ptr(),
        );
        return 0i32;
    }
    return sfx.wrapping_offset_from(s_knownSfx.as_mut_ptr())
        as crate::src::qcommon::q_shared::sfxHandle_t;
}
/*
=====================
S_BeginRegistration

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_BeginRegistration() {
    s_soundMuted = crate::src::qcommon::q_shared::qfalse; // we can play again
    if s_numSfx == 0 {
        crate::src::client::snd_mem::SND_setup();
        crate::stdlib::memset(
            s_knownSfx.as_mut_ptr() as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<[crate::snd_local_h::sfx_t; 4096]>(),
        );
        crate::stdlib::memset(
            sfxHash.as_mut_ptr() as *mut libc::c_void,
            '\u{0}' as i32,
            (::std::mem::size_of::<*mut crate::snd_local_h::sfx_t>()).wrapping_mul(128usize),
        );
        S_Base_RegisterSound(
            b"sound/feedback/hit.wav\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::qfalse,
        );
        // changed to a sound in baseq3
    };
}
#[no_mangle]

pub unsafe extern "C" fn S_memoryLoad(mut sfx: *mut crate::snd_local_h::sfx_t) {
    // load the sound file
    if crate::src::client::snd_mem::S_LoadSound(sfx) as u64 == 0 {
        //		Com_Printf( S_COLOR_YELLOW "WARNING: couldn't load sound: %s\n", sfx->soundName );
        (*sfx).defaultSound = crate::src::qcommon::q_shared::qtrue
    }
    (*sfx).inMemory = crate::src::qcommon::q_shared::qtrue;
}
//=============================================================================
/*
=================
S_SpatializeOrigin

Used for spatializing s_channels
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_SpatializeOrigin(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut master_vol: i32,
    mut left_vol: *mut i32,
    mut right_vol: *mut i32,
) {
    let mut dot: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut dist: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut lscale: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut rscale: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut scale: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut source_vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let dist_mult: f32 = 0.0008;
    // calculate stereo separation and distance attenuation
    source_vec[0] = *origin.offset(0) - listener_origin[0]; // close enough to be at full volume
    source_vec[1] = *origin.offset(1) - listener_origin[1]; // different attenuation levels
    source_vec[2] = *origin.offset(2) - listener_origin[2];
    dist = crate::src::qcommon::q_math::VectorNormalize(source_vec.as_mut_ptr());
    dist -= 80f32;
    if dist < 0f32 {
        dist = 0f32
    }
    dist *= dist_mult;
    crate::src::qcommon::q_math::VectorRotate(
        source_vec.as_mut_ptr(),
        listener_axis.as_mut_ptr(),
        vec.as_mut_ptr(),
    );
    dot = -vec[1];
    if dma.channels == 1 {
        // no attenuation = no spatialization
        rscale = 1f32;
        lscale = 1f32
    } else {
        rscale = (0.5 * (1.0 + dot as f64)) as crate::src::qcommon::q_shared::vec_t;
        lscale = (0.5 * (1.0 - dot as f64)) as crate::src::qcommon::q_shared::vec_t;
        if rscale < 0f32 {
            rscale = 0f32
        }
        if lscale < 0f32 {
            lscale = 0f32
        }
    }
    // add in distance effect
    scale = ((1.0 - dist as f64) * rscale as f64) as crate::src::qcommon::q_shared::vec_t;
    *right_vol = (master_vol as f32 * scale) as i32;
    if *right_vol < 0 {
        *right_vol = 0
    }
    scale = ((1.0 - dist as f64) * lscale as f64) as crate::src::qcommon::q_shared::vec_t;
    *left_vol = (master_vol as f32 * scale) as i32;
    if *left_vol < 0 {
        *left_vol = 0
    };
}
// =======================================================================
// Start a sound effect
// =======================================================================
/*
=================
S_Base_HearingThroughEntity

Also see S_AL_HearingThroughEntity
=================
*/

unsafe extern "C" fn S_Base_HearingThroughEntity(
    mut entityNum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut distanceSq: f32 = 0.;
    let mut sorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if !origin.is_null() {
        sorigin[0] = *origin.offset(0);
        sorigin[1] = *origin.offset(1);
        sorigin[2] = *origin.offset(2)
    } else {
        sorigin[0] = loopSounds[entityNum as usize].origin[0];
        sorigin[1] = loopSounds[entityNum as usize].origin[1];
        sorigin[2] = loopSounds[entityNum as usize].origin[2]
    }
    if listener_number == entityNum {
        // This is an outrageous hack to detect
        // whether or not the player is rendering in third person or not. We can't
        // ask the renderer because the renderer has no notion of entities and we
        // can't ask cgame since that would involve changing the API and hence mod
        // compatibility. I don't think there is any way around this, but I'll leave
        // the FIXME just in case anyone has a bright idea.
        distanceSq = DistanceSquared(
            sorigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            listener_origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
        if distanceSq > 48.0 * 48.0 {
            //we're the player
            return crate::src::qcommon::q_shared::qfalse;
        } else {
            return crate::src::qcommon::q_shared::qtrue;
        }
    } else {
        return crate::src::qcommon::q_shared::qfalse;
    }; //we're the player, but third person
       //not the player
}
/*
====================
S_Base_StartSoundEx

Validates the parms and ques the sound up
if origin is NULL, the sound will be dynamically sourced from the entity
Entchannel 0 will never override a playing sound
====================
*/

unsafe extern "C" fn S_Base_StartSoundEx(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut entityNum: i32,
    mut entchannel: i32,
    mut sfxHandle: crate::src::qcommon::q_shared::sfxHandle_t,
    mut localSound: crate::src::qcommon::q_shared::qboolean,
) {
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    let mut i: i32 = 0;
    let mut oldest: i32 = 0;
    let mut chosen: i32 = 0;
    let mut time: i32 = 0;
    let mut inplay: i32 = 0;
    let mut allowed: i32 = 0;
    let mut fullVolume: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    if origin.is_null() && (entityNum < 0 || entityNum >= (1) << 10) {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"S_StartSound: bad entitynum %i\x00" as *const u8 as *const i8,
            entityNum,
        );
    }
    if sfxHandle < 0 || sfxHandle >= s_numSfx {
        crate::src::qcommon::common::Com_Printf(
            b"^3S_StartSound: handle %i out of range\n\x00" as *const u8 as *const i8,
            sfxHandle,
        );
        return;
    }
    sfx =
        &mut *s_knownSfx.as_mut_ptr().offset(sfxHandle as isize) as *mut crate::snd_local_h::sfx_t;
    if (*sfx).inMemory == crate::src::qcommon::q_shared::qfalse {
        S_memoryLoad(sfx);
    }
    if (*s_show).integer == 1 {
        crate::src::qcommon::common::Com_Printf(
            b"%i : %s\n\x00" as *const u8 as *const i8,
            s_paintedtime,
            (*sfx).soundName.as_mut_ptr(),
        );
    }
    time = crate::src::qcommon::common::Com_Milliseconds();
    //	Com_Printf("playing %s\n", sfx->soundName);
    // pick a channel to play on
    allowed = 4;
    if entityNum == listener_number {
        allowed = 8
    }
    fullVolume = crate::src::qcommon::q_shared::qfalse;
    if localSound != 0 || S_Base_HearingThroughEntity(entityNum, origin) != 0 {
        fullVolume = crate::src::qcommon::q_shared::qtrue
    }
    ch = s_channels.as_mut_ptr();
    inplay = 0;
    i = 0;
    while i < 96 {
        if (*ch).entnum == entityNum && (*ch).thesfx == sfx {
            if time - (*ch).allocTime < 50 {
                //				if (Cvar_VariableValue( "cg_showmiss" )) {
                //					Com_Printf("double sound start\n");
                //				}
                return;
            } // entityNum, entchannel);
            inplay += 1
        } // these will get calced at next spatialize
        i += 1; // unless the game isn't running
        ch = ch.offset(1)
    }
    if inplay > allowed {
        return;
    }
    (*sfx).lastTimeUsed = time;
    ch = S_ChannelMalloc();
    if ch.is_null() {
        ch = s_channels.as_mut_ptr();
        oldest = (*sfx).lastTimeUsed;
        chosen = -(1);
        i = 0;
        while i < 96 {
            if (*ch).entnum != listener_number
                && (*ch).entnum == entityNum
                && (*ch).allocTime < oldest
                && (*ch).entchannel != crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32
            {
                oldest = (*ch).allocTime;
                chosen = i
            }
            i += 1;
            ch = ch.offset(1)
        }
        if chosen == -(1) {
            ch = s_channels.as_mut_ptr();
            i = 0;
            while i < 96 {
                if (*ch).entnum != listener_number
                    && (*ch).allocTime < oldest
                    && (*ch).entchannel != crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32
                {
                    oldest = (*ch).allocTime;
                    chosen = i
                }
                i += 1;
                ch = ch.offset(1)
            }
            if chosen == -(1) {
                ch = s_channels.as_mut_ptr();
                if (*ch).entnum == listener_number {
                    i = 0;
                    while i < 96 {
                        if (*ch).allocTime < oldest {
                            oldest = (*ch).allocTime;
                            chosen = i
                        }
                        i += 1;
                        ch = ch.offset(1)
                    }
                }
                if chosen == -(1) {
                    crate::src::qcommon::common::Com_Printf(
                        b"dropping sound\n\x00" as *const u8 as *const i8,
                    );
                    return;
                }
            }
        }
        ch = &mut *s_channels.as_mut_ptr().offset(chosen as isize)
            as *mut crate::snd_local_h::channel_t;
        (*ch).allocTime = (*sfx).lastTimeUsed
    }
    if !origin.is_null() {
        (*ch).origin[0] = *origin.offset(0);
        (*ch).origin[1] = *origin.offset(1);
        (*ch).origin[2] = *origin.offset(2);
        (*ch).fixed_origin = crate::src::qcommon::q_shared::qtrue
    } else {
        (*ch).fixed_origin = crate::src::qcommon::q_shared::qfalse
    }
    (*ch).master_vol = 127;
    (*ch).entnum = entityNum;
    (*ch).thesfx = sfx;
    (*ch).startSample = 0x7fffffff;
    (*ch).entchannel = entchannel;
    (*ch).leftvol = (*ch).master_vol;
    (*ch).rightvol = (*ch).master_vol;
    (*ch).doppler = crate::src::qcommon::q_shared::qfalse;
    (*ch).fullVolume = fullVolume;
}
/*
====================
S_StartSound

if origin is NULL, the sound will be dynamically sourced from the entity
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_StartSound(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut entityNum: i32,
    mut entchannel: i32,
    mut sfxHandle: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    S_Base_StartSoundEx(
        origin,
        entityNum,
        entchannel,
        sfxHandle,
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
==================
S_StartLocalSound
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_StartLocalSound(
    mut sfxHandle: crate::src::qcommon::q_shared::sfxHandle_t,
    mut channelNum: i32,
) {
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    if sfxHandle < 0 || sfxHandle >= s_numSfx {
        crate::src::qcommon::common::Com_Printf(
            b"^3S_StartLocalSound: handle %i out of range\n\x00" as *const u8 as *const i8,
            sfxHandle,
        );
        return;
    }
    S_Base_StartSoundEx(
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        listener_number,
        channelNum,
        sfxHandle,
        crate::src::qcommon::q_shared::qtrue,
    );
}
/*
==================
S_ClearSoundBuffer

If we are about to perform file access, clear the buffer
so sound doesn't stutter.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_ClearSoundBuffer() {
    let mut clear: i32 = 0;
    if s_soundStarted == 0 {
        return;
    }
    // stop looping sounds
    crate::stdlib::memset(
        loopSounds.as_mut_ptr() as *mut libc::c_void,
        0,
        (((1i32) << 10) as usize)
            .wrapping_mul(::std::mem::size_of::<crate::snd_local_h::loopSound_t>()),
    );
    crate::stdlib::memset(
        loop_channels.as_mut_ptr() as *mut libc::c_void,
        0,
        (96usize).wrapping_mul(::std::mem::size_of::<crate::snd_local_h::channel_t>()),
    );
    numLoopChannels = 0;
    S_ChannelSetup();
    crate::stdlib::memset(
        s_rawend.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[i32; 129]>(),
    );
    if dma.samplebits == 8 {
        clear = 0x80
    } else {
        clear = 0
    }
    crate::src::sdl::sdl_snd::SNDDMA_BeginPainting();
    if !dma.buffer.is_null() {
        crate::stdlib::memset(
            dma.buffer as *mut libc::c_void,
            clear,
            (dma.samples * dma.samplebits / 8i32) as usize,
        );
    }
    crate::src::sdl::sdl_snd::SNDDMA_Submit();
}
/*
==================
S_StopAllSounds
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_StopAllSounds() {
    if s_soundStarted == 0 {
        return;
    }
    // stop the background music
    S_Base_StopBackgroundTrack();
    S_Base_ClearSoundBuffer();
}
/*
==============================================================

continuous looping sounds are added each frame

==============================================================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_StopLoopingSound(mut entityNum: i32) {
    loopSounds[entityNum as usize].active = crate::src::qcommon::q_shared::qfalse;
    //	loopSounds[entityNum].sfx = 0;
    loopSounds[entityNum as usize].kill = crate::src::qcommon::q_shared::qfalse;
}
/*
==================
S_ClearLoopingSounds

==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_ClearLoopingSounds(
    mut killall: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: i32 = 0;

    for i in 0..(1) << 10 {
        if killall != 0
            || loopSounds[i as usize].kill == crate::src::qcommon::q_shared::qtrue
            || !loopSounds[i as usize].sfx.is_null()
                && (*loopSounds[i as usize].sfx).soundLength == 0
        {
            S_Base_StopLoopingSound(i);
        }
    }
    numLoopChannels = 0;
}
/*
==================
S_AddLoopingSound

Called during entity generation for a frame
Include velocity in case I get around to doing doppler...
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_AddLoopingSound(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut velocity: *const crate::src::qcommon::q_shared::vec_t,
    mut sfxHandle: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    if sfxHandle < 0 || sfxHandle >= s_numSfx {
        crate::src::qcommon::common::Com_Printf(
            b"^3S_AddLoopingSound: handle %i out of range\n\x00" as *const u8 as *const i8,
            sfxHandle,
        );
        return;
    }
    sfx =
        &mut *s_knownSfx.as_mut_ptr().offset(sfxHandle as isize) as *mut crate::snd_local_h::sfx_t;
    if (*sfx).inMemory == crate::src::qcommon::q_shared::qfalse {
        S_memoryLoad(sfx);
    }
    if (*sfx).soundLength == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"%s has length 0\x00" as *const u8 as *const i8,
            (*sfx).soundName.as_mut_ptr(),
        );
    }
    loopSounds[entityNum as usize].origin[0] = *origin.offset(0);
    loopSounds[entityNum as usize].origin[1] = *origin.offset(1);
    loopSounds[entityNum as usize].origin[2] = *origin.offset(2);
    loopSounds[entityNum as usize].velocity[0] = *velocity.offset(0);
    loopSounds[entityNum as usize].velocity[1] = *velocity.offset(1);
    loopSounds[entityNum as usize].velocity[2] = *velocity.offset(2);
    loopSounds[entityNum as usize].active = crate::src::qcommon::q_shared::qtrue;
    loopSounds[entityNum as usize].kill = crate::src::qcommon::q_shared::qtrue;
    loopSounds[entityNum as usize].doppler = crate::src::qcommon::q_shared::qfalse;
    loopSounds[entityNum as usize].oldDopplerScale = 1f32;
    loopSounds[entityNum as usize].dopplerScale = 1f32;
    loopSounds[entityNum as usize].sfx = sfx;
    if (*crate::src::client::snd_main::s_doppler).integer != 0
        && VectorLengthSquared(velocity) as f64 > 0.0
    {
        let mut out: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut lena: f32 = 0.;
        let mut lenb: f32 = 0.;
        loopSounds[entityNum as usize].doppler = crate::src::qcommon::q_shared::qtrue;
        lena = DistanceSquared(
            loopSounds[listener_number as usize].origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            loopSounds[entityNum as usize].origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
        );
        out[0] =
            loopSounds[entityNum as usize].origin[0] + loopSounds[entityNum as usize].velocity[0];
        out[1] =
            loopSounds[entityNum as usize].origin[1] + loopSounds[entityNum as usize].velocity[1];
        out[2] =
            loopSounds[entityNum as usize].origin[2] + loopSounds[entityNum as usize].velocity[2];
        lenb = DistanceSquared(
            loopSounds[listener_number as usize].origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            out.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
        if loopSounds[entityNum as usize].framenum + 1
            != crate::src::client::cl_main::cls.framecount
        {
            loopSounds[entityNum as usize].oldDopplerScale = 1f32
        } else {
            loopSounds[entityNum as usize].oldDopplerScale =
                loopSounds[entityNum as usize].dopplerScale
        }
        loopSounds[entityNum as usize].dopplerScale = lenb / (lena * 100f32);
        if loopSounds[entityNum as usize].dopplerScale as f64 <= 1.0 {
            loopSounds[entityNum as usize].doppler = crate::src::qcommon::q_shared::qfalse
        // don't bother doing the math
        } else if loopSounds[entityNum as usize].dopplerScale > 50.0 {
            loopSounds[entityNum as usize].dopplerScale = 50.0
        }
    }
    loopSounds[entityNum as usize].framenum = crate::src::client::cl_main::cls.framecount;
}
/*
==================
S_AddLoopingSound

Called during entity generation for a frame
Include velocity in case I get around to doing doppler...
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_AddRealLoopingSound(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut velocity: *const crate::src::qcommon::q_shared::vec_t,
    mut sfxHandle: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    if sfxHandle < 0 || sfxHandle >= s_numSfx {
        crate::src::qcommon::common::Com_Printf(
            b"^3S_AddRealLoopingSound: handle %i out of range\n\x00" as *const u8 as *const i8,
            sfxHandle,
        );
        return;
    }
    sfx =
        &mut *s_knownSfx.as_mut_ptr().offset(sfxHandle as isize) as *mut crate::snd_local_h::sfx_t;
    if (*sfx).inMemory == crate::src::qcommon::q_shared::qfalse {
        S_memoryLoad(sfx);
    }
    if (*sfx).soundLength == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"%s has length 0\x00" as *const u8 as *const i8,
            (*sfx).soundName.as_mut_ptr(),
        );
    }
    loopSounds[entityNum as usize].origin[0] = *origin.offset(0);
    loopSounds[entityNum as usize].origin[1] = *origin.offset(1);
    loopSounds[entityNum as usize].origin[2] = *origin.offset(2);
    loopSounds[entityNum as usize].velocity[0] = *velocity.offset(0);
    loopSounds[entityNum as usize].velocity[1] = *velocity.offset(1);
    loopSounds[entityNum as usize].velocity[2] = *velocity.offset(2);
    loopSounds[entityNum as usize].sfx = sfx;
    loopSounds[entityNum as usize].active = crate::src::qcommon::q_shared::qtrue;
    loopSounds[entityNum as usize].kill = crate::src::qcommon::q_shared::qfalse;
    loopSounds[entityNum as usize].doppler = crate::src::qcommon::q_shared::qfalse;
}
/*
==================
S_AddLoopSounds

Spatialize all of the looping sounds.
All sounds are on the same cycle, so any duplicates can just
sum up the channel multipliers.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AddLoopSounds() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut time: i32 = 0;
    let mut left_total: i32 = 0;
    let mut right_total: i32 = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut loop_0: *mut crate::snd_local_h::loopSound_t =
        0 as *mut crate::snd_local_h::loopSound_t;
    let mut loop2: *mut crate::snd_local_h::loopSound_t = 0 as *mut crate::snd_local_h::loopSound_t;
    static mut loopFrame: i32 = 0;
    numLoopChannels = 0;
    time = crate::src::qcommon::common::Com_Milliseconds();
    loopFrame += 1;
    i = 0;
    while i < (1) << 10 {
        loop_0 = &mut *loopSounds.as_mut_ptr().offset(i as isize)
            as *mut crate::snd_local_h::loopSound_t;
        if !((*loop_0).active as u64 == 0 || (*loop_0).mergeFrame == loopFrame) {
            if (*loop_0).kill as u64 != 0 {
                S_SpatializeOrigin(
                    (*loop_0).origin.as_mut_ptr(),
                    127i32,
                    &mut left_total,
                    &mut right_total,
                );
            // 3d
            } else {
                S_SpatializeOrigin(
                    (*loop_0).origin.as_mut_ptr(),
                    90i32,
                    &mut left_total,
                    &mut right_total,
                );
                // sphere
            }
            (*(*loop_0).sfx).lastTimeUsed = time;

            for j in i + 1..(1) << 10 {
                loop2 = &mut *loopSounds.as_mut_ptr().offset(j as isize)
                    as *mut crate::snd_local_h::loopSound_t;

                if !((*loop2).active as u64 == 0
                    || (*loop2).doppler != 0
                    || (*loop2).sfx != (*loop_0).sfx)
                {
                    (*loop2).mergeFrame = loopFrame;
                    if (*loop2).kill as u64 != 0 {
                        S_SpatializeOrigin(
                            (*loop2).origin.as_mut_ptr(),
                            127i32,
                            &mut left,
                            &mut right,
                        );
                    // 3d
                    } else {
                        S_SpatializeOrigin(
                            (*loop2).origin.as_mut_ptr(),
                            90i32,
                            &mut left,
                            &mut right,
                        );
                        // sphere
                    }
                    (*(*loop2).sfx).lastTimeUsed = time;
                    left_total += left;
                    right_total += right
                }
            }
            if !(left_total == 0 && right_total == 0) {
                // allocate a channel
                ch = &mut *loop_channels.as_mut_ptr().offset(numLoopChannels as isize)
                    as *mut crate::snd_local_h::channel_t;
                if left_total > 255 {
                    left_total = 255
                }
                if right_total > 255 {
                    right_total = 255
                }
                (*ch).master_vol = 127;
                (*ch).leftvol = left_total;
                (*ch).rightvol = right_total;
                (*ch).thesfx = (*loop_0).sfx;
                (*ch).doppler = (*loop_0).doppler;
                (*ch).dopplerScale = (*loop_0).dopplerScale;
                (*ch).oldDopplerScale = (*loop_0).oldDopplerScale;
                (*ch).fullVolume = crate::src::qcommon::q_shared::qfalse;
                numLoopChannels += 1;
                if numLoopChannels == 96 {
                    return;
                }
            }
        }
        i += 1
        // not audible
    }
}
//=============================================================================
/*
=================
S_ByteSwapRawSamples

If raw data has been loaded in little endien binary form, this must be done.
If raw data was calculated, as with ADPCM, this should not be called.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_ByteSwapRawSamples(
    mut samples: i32,
    mut width: i32,
    mut s_channels_0: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
) {
    let mut i: i32 = 0;
    if width != 2 {
        return;
    }
    if 256 == 256 {
        return;
    }
    if s_channels_0 == 2 {
        samples <<= 1
    }
    i = 0;
    while i < samples {
        *(data as *mut i16).offset(i as isize) = *(data as *mut i16).offset(i as isize);
        i += 1
    }
}
/*
============
S_Base_RawSamples

Music streaming
============
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_RawSamples(
    mut stream: i32,
    mut samples: i32,
    mut rate: i32,
    mut width: i32,
    mut s_channels_0: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
    mut volume: f32,
    mut entityNum: i32,
) {
    let mut i: i32 = 0;
    let mut src: i32 = 0;
    let mut dst: i32 = 0;
    let mut scale: f32 = 0.;
    let mut intVolumeLeft: i32 = 0;
    let mut intVolumeRight: i32 = 0;
    let mut rawsamples: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    if stream < 0 || stream >= 64 * 2 + 1 {
        return;
    }
    rawsamples = s_rawsamples[stream as usize].as_mut_ptr();
    if (*crate::src::client::snd_main::s_muted).integer != 0 {
        intVolumeRight = 0;
        intVolumeLeft = intVolumeRight
    } else {
        let mut leftvol: i32 = 0;
        let mut rightvol: i32 = 0;
        if entityNum >= 0 && entityNum < (1) << 10 {
            // support spatialized raw streams, e.g. for VoIP
            S_SpatializeOrigin(
                loopSounds[entityNum as usize].origin.as_mut_ptr(),
                256i32,
                &mut leftvol,
                &mut rightvol,
            );
        } else {
            rightvol = 256;
            leftvol = rightvol
        }
        intVolumeLeft =
            (leftvol as f32 * volume * (*crate::src::client::snd_main::s_volume).value) as i32;
        intVolumeRight =
            (rightvol as f32 * volume * (*crate::src::client::snd_main::s_volume).value) as i32
    }
    if s_rawend[stream as usize] < s_soundtime {
        crate::src::qcommon::common::Com_DPrintf(
            b"S_Base_RawSamples: resetting minimum: %i < %i\n\x00" as *const u8 as *const i8,
            s_rawend[stream as usize],
            s_soundtime,
        );
        s_rawend[stream as usize] = s_soundtime
    }
    scale = rate as f32 / dma.speed as f32;
    //Com_Printf ("%i < %i < %i\n", s_soundtime, s_paintedtime, s_rawend[stream]);
    if s_channels_0 == 2 && width == 2 {
        if scale as f64 == 1.0 {
            // optimized case
            i = 0;
            while i < samples {
                dst = s_rawend[stream as usize] & 16384 - 1;
                s_rawend[stream as usize] += 1;
                (*rawsamples.offset(dst as isize)).left =
                    *(data as *mut i16).offset((i * 2) as isize) as i32 * intVolumeLeft;
                (*rawsamples.offset(dst as isize)).right =
                    *(data as *mut i16).offset((i * 2 + 1) as isize) as i32 * intVolumeRight;
                i += 1
            }
        } else {
            i = 0;
            loop {
                src = (i as f32 * scale) as i32;
                if src >= samples {
                    break;
                }
                dst = s_rawend[stream as usize] & 16384 - 1;
                s_rawend[stream as usize] += 1;
                (*rawsamples.offset(dst as isize)).left =
                    *(data as *mut i16).offset((src * 2) as isize) as i32 * intVolumeLeft;
                (*rawsamples.offset(dst as isize)).right =
                    *(data as *mut i16).offset((src * 2 + 1) as isize) as i32 * intVolumeRight;
                i += 1
            }
        }
    } else if s_channels_0 == 1 && width == 2 {
        i = 0;
        loop {
            src = (i as f32 * scale) as i32;
            if src >= samples {
                break;
            }
            dst = s_rawend[stream as usize] & 16384 - 1;
            s_rawend[stream as usize] += 1;
            (*rawsamples.offset(dst as isize)).left =
                *(data as *mut i16).offset(src as isize) as i32 * intVolumeLeft;
            (*rawsamples.offset(dst as isize)).right =
                *(data as *mut i16).offset(src as isize) as i32 * intVolumeRight;
            i += 1
        }
    } else if s_channels_0 == 2 && width == 1 {
        intVolumeLeft *= 256;
        intVolumeRight *= 256;
        i = 0;
        loop {
            src = (i as f32 * scale) as i32;
            if src >= samples {
                break;
            }
            dst = s_rawend[stream as usize] & 16384 - 1;
            s_rawend[stream as usize] += 1;
            (*rawsamples.offset(dst as isize)).left =
                *(data as *mut i8).offset((src * 2) as isize) as i32 * intVolumeLeft;
            (*rawsamples.offset(dst as isize)).right =
                *(data as *mut i8).offset((src * 2 + 1) as isize) as i32 * intVolumeRight;
            i += 1
        }
    } else if s_channels_0 == 1 && width == 1 {
        intVolumeLeft *= 256;
        intVolumeRight *= 256;
        i = 0;
        loop {
            src = (i as f32 * scale) as i32;
            if src >= samples {
                break;
            }
            dst = s_rawend[stream as usize] & 16384 - 1;
            s_rawend[stream as usize] += 1;
            (*rawsamples.offset(dst as isize)).left =
                (*(data as *mut crate::src::qcommon::q_shared::byte).offset(src as isize) as i32
                    - 128)
                    * intVolumeLeft;
            (*rawsamples.offset(dst as isize)).right =
                (*(data as *mut crate::src::qcommon::q_shared::byte).offset(src as isize) as i32
                    - 128)
                    * intVolumeRight;
            i += 1
        }
    }
    if s_rawend[stream as usize] > s_soundtime + 16384 {
        crate::src::qcommon::common::Com_DPrintf(
            b"S_Base_RawSamples: overflowed %i > %i\n\x00" as *const u8 as *const i8,
            s_rawend[stream as usize],
            s_soundtime,
        );
    };
}
//=============================================================================
/*
=====================
S_UpdateEntityPosition

let the sound system know where an entity currently is
======================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_UpdateEntityPosition(
    mut entityNum: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
) {
    if entityNum < 0 || entityNum >= (1) << 10 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"S_UpdateEntityPosition: bad entitynum %i\x00" as *const u8 as *const i8,
            entityNum,
        );
    }
    loopSounds[entityNum as usize].origin[0] = *origin.offset(0);
    loopSounds[entityNum as usize].origin[1] = *origin.offset(1);
    loopSounds[entityNum as usize].origin[2] = *origin.offset(2);
}
/*
============
S_Respatialize

Change the volumes of all the playing sounds for changes in their positions
============
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_Respatialize(
    mut entityNum: i32,
    mut head: *const crate::src::qcommon::q_shared::vec_t,
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
    mut inwater: i32,
) {
    let mut i: i32 = 0;
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    listener_number = entityNum;
    listener_origin[0] = *head.offset(0);
    listener_origin[1] = *head.offset(1);
    listener_origin[2] = *head.offset(2);
    listener_axis[0][0] = (*axis.offset(0))[0];
    listener_axis[0][1] = (*axis.offset(0))[1];
    listener_axis[0][2] = (*axis.offset(0))[2];
    listener_axis[1][0] = (*axis.offset(1))[0];
    listener_axis[1][1] = (*axis.offset(1))[1];
    listener_axis[1][2] = (*axis.offset(1))[2];
    listener_axis[2][0] = (*axis.offset(2))[0];
    listener_axis[2][1] = (*axis.offset(2))[1];
    listener_axis[2][2] = (*axis.offset(2))[2];
    // update spatialization for dynamic sounds
    ch = s_channels.as_mut_ptr();
    i = 0;
    while i < 96 {
        if !(*ch).thesfx.is_null() {
            // local and first person sounds will always be full volume
            if (*ch).fullVolume as u64 != 0 {
                (*ch).leftvol = (*ch).master_vol;
                (*ch).rightvol = (*ch).master_vol
            } else {
                if (*ch).fixed_origin as u64 != 0 {
                    origin[0] = (*ch).origin[0];
                    origin[1] = (*ch).origin[1];
                    origin[2] = (*ch).origin[2]
                } else {
                    origin[0] = loopSounds[(*ch).entnum as usize].origin[0];
                    origin[1] = loopSounds[(*ch).entnum as usize].origin[1];
                    origin[2] = loopSounds[(*ch).entnum as usize].origin[2]
                }
                S_SpatializeOrigin(
                    origin.as_mut_ptr(),
                    (*ch).master_vol,
                    &mut (*ch).leftvol,
                    &mut (*ch).rightvol,
                );
            }
        }
        i += 1;
        ch = ch.offset(1)
    }
    // add loopsounds
    S_AddLoopSounds();
}
/*
========================
S_ScanChannelStarts

Returns qtrue if any new sounds were started since the last mix
========================
*/
#[no_mangle]

pub unsafe extern "C" fn S_ScanChannelStarts() -> crate::src::qcommon::q_shared::qboolean {
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut i: i32 = 0;
    let mut newSamples: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    newSamples = crate::src::qcommon::q_shared::qfalse;
    ch = s_channels.as_mut_ptr();
    i = 0;
    while i < 96 {
        if !(*ch).thesfx.is_null() {
            // if this channel was just started this frame,
            // set the sample count to it begins mixing
            // into the very first sample
            if (*ch).startSample == 0x7fffffff {
                (*ch).startSample = s_paintedtime;
                newSamples = crate::src::qcommon::q_shared::qtrue
            } else if (*ch).startSample + (*(*ch).thesfx).soundLength <= s_paintedtime {
                S_ChannelFree(ch);
            }
        }
        i += 1;
        ch = ch.offset(1)
    }
    return newSamples;
}
// if it is completely finished by now, clear it
/*
============
S_Update

Called once each time through the main loop
============
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_Update() {
    let mut i: i32 = 0;
    let mut total: i32 = 0;
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    if s_soundStarted == 0 || s_soundMuted != 0 {
        //		Com_DPrintf ("not started or muted\n");
        return;
    }
    //
    // debugging output
    //
    if (*s_show).integer == 2 {
        total = 0;
        ch = s_channels.as_mut_ptr();
        i = 0;
        while i < 96 {
            if !(*ch).thesfx.is_null() && ((*ch).leftvol != 0 || (*ch).rightvol != 0) {
                crate::src::qcommon::common::Com_Printf(
                    b"%d %d %s\n\x00" as *const u8 as *const i8,
                    (*ch).leftvol,
                    (*ch).rightvol,
                    (*(*ch).thesfx).soundName.as_mut_ptr(),
                );
                total += 1
            }
            i += 1;
            ch = ch.offset(1)
        }
        crate::src::qcommon::common::Com_Printf(
            b"----(%i)---- painted: %i\n\x00" as *const u8 as *const i8,
            total,
            s_paintedtime,
        );
    }
    // add raw data from streamed samples
    S_UpdateBackgroundTrack();
    // mix some sound
    S_Update_();
}
#[no_mangle]

pub unsafe extern "C" fn S_GetSoundtime() {
    let mut samplepos: i32 = 0;
    static mut buffers: i32 = 0;
    static mut oldsamplepos: i32 = 0;
    if crate::src::client::cl_avi::CL_VideoRecording() as u64 != 0 {
        let mut fps: f32 = if (*crate::src::client::cl_main::cl_aviFrameRate).value < 1000.0 {
            (*crate::src::client::cl_main::cl_aviFrameRate).value
        } else {
            1000.0
        };
        let mut frameDuration: f32 = (if dma.speed as f32 / fps > 1.0 {
            (dma.speed as f32) / fps
        } else {
            1.0
        }) + crate::src::client::cl_main::clc.aviSoundFrameRemainder;
        let mut msec: i32 = frameDuration as i32;
        s_soundtime += msec;
        crate::src::client::cl_main::clc.aviSoundFrameRemainder = frameDuration - msec as f32;
        return;
    }
    // it is possible to miscount buffers if it has wrapped twice between
    // calls to S_Update.  Oh well.
    samplepos = crate::src::sdl::sdl_snd::SNDDMA_GetDMAPos(); // buffer wrapped
    if samplepos < oldsamplepos {
        buffers += 1;
        if s_paintedtime > 0x40000000 {
            // time to chop things off to avoid 32 bit limits
            buffers = 0;
            s_paintedtime = dma.fullsamples;
            S_Base_StopAllSounds();
        }
    }
    oldsamplepos = samplepos;
    s_soundtime = buffers * dma.fullsamples + samplepos / dma.channels;
    if dma.submission_chunk < 256 {
        s_paintedtime = (s_soundtime as f32 + (*s_mixPreStep).value * dma.speed as f32) as i32
    } else {
        s_paintedtime = s_soundtime + dma.submission_chunk
    };
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
/* ****************************************************************************
 * name:		snd_dma.c
 *
 * desc:		main control for any streaming sound output device
 *
 * $Archive: /MissionPack/code/client/snd_dma.c $
 *
 *****************************************************************************/
#[no_mangle]

pub unsafe extern "C" fn S_Update_() {
    let mut endtime: u32 = 0;
    static mut lastTime: f32 = 0.0;
    let mut ma: f32 = 0.;
    let mut op: f32 = 0.;
    let mut thisTime: f32 = 0.;
    let mut sane: f32 = 0.;
    static mut ot: i32 = -(1);
    if s_soundStarted == 0 || s_soundMuted != 0 {
        return;
    }
    thisTime = crate::src::qcommon::common::Com_Milliseconds() as f32;
    // Updates s_soundtime
    S_GetSoundtime();
    if s_soundtime == ot {
        return;
    }
    ot = s_soundtime;
    // clear any sound effects that end before the current time,
    // and start any new sounds
    S_ScanChannelStarts();
    sane = thisTime - lastTime;
    if sane < 11f32 {
        sane = 11f32
        // 85hz
    }
    ma = (*s_mixahead).value * dma.speed as f32;
    op = ((*s_mixPreStep).value as f64 + (sane * dma.speed as f32) as f64 * 0.01) as f32;
    if op < ma {
        ma = op
    }
    // mix ahead of current position
    endtime = (s_soundtime as f32 + ma) as u32;
    // mix to an even submission block size
    endtime = endtime
        .wrapping_add(dma.submission_chunk as u32)
        .wrapping_sub(1u32)
        & !(dma.submission_chunk - 1) as u32;
    // never mix more than the complete buffer
    if endtime.wrapping_sub(s_soundtime as u32) > dma.fullsamples as u32 {
        endtime = (s_soundtime + dma.fullsamples) as u32
    }
    crate::src::sdl::sdl_snd::SNDDMA_BeginPainting();
    crate::src::client::snd_mix::S_PaintChannels(endtime as i32);
    crate::src::sdl::sdl_snd::SNDDMA_Submit();
    lastTime = thisTime;
}
/*
===============================================================================

background music functions

===============================================================================
*/
/*
======================
S_StopBackgroundTrack
======================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_StopBackgroundTrack() {
    if s_backgroundStream.is_null() {
        return;
    }
    crate::src::client::snd_codec::S_CodecCloseStream(s_backgroundStream);
    s_backgroundStream = 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    s_rawend[0] = 0;
}
/*
======================
S_OpenBackgroundStream
======================
*/

unsafe extern "C" fn S_OpenBackgroundStream(mut filename: *const i8) {
    // close the background track, but DON'T reset s_rawend
    // if restarting the same back ground track
    if !s_backgroundStream.is_null() {
        crate::src::client::snd_codec::S_CodecCloseStream(s_backgroundStream);
        s_backgroundStream = 0 as *mut crate::src::client::snd_codec::snd_stream_t
    }
    // Open stream
    s_backgroundStream = crate::src::client::snd_codec::S_CodecOpenStream(filename);
    if s_backgroundStream.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: couldn\'t open music file %s\n\x00" as *const u8 as *const i8,
            filename,
        );
        return;
    }
    if (*s_backgroundStream).info.channels != 2 || (*s_backgroundStream).info.rate != 22050 {
        crate::src::qcommon::common::Com_Printf(
            b"^3WARNING: music file %s is not 22k stereo\n\x00" as *const u8 as *const i8,
            filename,
        );
    };
}
/*
======================
S_StartBackgroundTrack
======================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_StartBackgroundTrack(mut intro: *const i8, mut loop_0: *const i8) {
    if intro.is_null() {
        intro = b"\x00" as *const u8 as *const i8
    }
    if loop_0.is_null() || *loop_0.offset(0) == 0 {
        loop_0 = intro
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"S_StartBackgroundTrack( %s, %s )\n\x00" as *const u8 as *const i8,
        intro,
        loop_0,
    );
    if *intro == 0 {
        S_Base_StopBackgroundTrack();
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_backgroundLoop.as_mut_ptr(),
        loop_0,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    S_OpenBackgroundStream(intro);
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// if origin is NULL, the sound will be dynamically sourced from the entity
// if origin is NULL, the sound will be dynamically sourced from the entity
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
// stop all sounds and the background track
// stop all sounds and the background track
// all continuous looping sounds must be added before calling S_Update
// all continuous looping sounds must be added before calling S_Update
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
// let the sound system know where an entity currently is
// let the sound system know where an entity currently is
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
/*
======================
S_UpdateBackgroundTrack
======================
*/
#[no_mangle]

pub unsafe extern "C" fn S_UpdateBackgroundTrack() {
    let mut bufferSamples: i32 = 0; // just enough to fit in a mac stack frame
    let mut fileSamples: i32 = 0;
    let mut raw: [crate::src::qcommon::q_shared::byte; 30000] = [0; 30000];
    let mut fileBytes: i32 = 0;
    let mut r: i32 = 0;
    if s_backgroundStream.is_null() {
        return;
    }
    // don't bother playing anything if musicvolume is 0
    if (*crate::src::client::snd_main::s_musicVolume).value <= 0f32 {
        return;
    }
    // see how many samples should be copied into the raw buffer
    if s_rawend[0] < s_soundtime {
        s_rawend[0] = s_soundtime
    }
    while s_rawend[0] < s_soundtime + 16384 {
        bufferSamples = 16384 - (s_rawend[0] - s_soundtime);
        // decide how much data needs to be read from the file
        fileSamples = bufferSamples * (*s_backgroundStream).info.rate / dma.speed;
        if fileSamples == 0 {
            return;
        }
        // our max buffer size
        fileBytes =
            fileSamples * ((*s_backgroundStream).info.width * (*s_backgroundStream).info.channels);
        if fileBytes as usize
            > ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 30000]>()
        {
            fileBytes =
                ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 30000]>() as i32;
            fileSamples =
                fileBytes / ((*s_backgroundStream).info.width * (*s_backgroundStream).info.channels)
        }
        // Read
        r = crate::src::client::snd_codec::S_CodecReadStream(
            s_backgroundStream,
            fileBytes,
            raw.as_mut_ptr() as *mut libc::c_void,
        );
        if r < fileBytes {
            fileSamples =
                r / ((*s_backgroundStream).info.width * (*s_backgroundStream).info.channels)
        }
        if r > 0 {
            // add to raw buffer
            S_Base_RawSamples(
                0i32,
                fileSamples,
                (*s_backgroundStream).info.rate,
                (*s_backgroundStream).info.width,
                (*s_backgroundStream).info.channels,
                raw.as_mut_ptr(),
                (*crate::src::client::snd_main::s_musicVolume).value,
                -(1i32),
            );
        } else if s_backgroundLoop[0] != 0 {
            S_OpenBackgroundStream(s_backgroundLoop.as_mut_ptr());
            if s_backgroundStream.is_null() {
                return;
            }
        } else {
            S_Base_StopBackgroundTrack();
            return;
        }
    }
}
// loop
/*
======================
S_FreeOldestSound
======================
*/
#[no_mangle]

pub unsafe extern "C" fn S_FreeOldestSound() {
    let mut i: i32 = 0;
    let mut oldest: i32 = 0;
    let mut used: i32 = 0;
    let mut sfx: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    let mut buffer: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut nbuffer: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    oldest = crate::src::qcommon::common::Com_Milliseconds();
    used = 0;

    for i in 1..s_numSfx {
        sfx = &mut *s_knownSfx.as_mut_ptr().offset(i as isize) as *mut crate::snd_local_h::sfx_t;

        if (*sfx).inMemory != 0 && (*sfx).lastTimeUsed < oldest {
            used = i;
            oldest = (*sfx).lastTimeUsed
        }
    }
    sfx = &mut *s_knownSfx.as_mut_ptr().offset(used as isize) as *mut crate::snd_local_h::sfx_t;
    crate::src::qcommon::common::Com_DPrintf(
        b"S_FreeOldestSound: freeing sound %s\n\x00" as *const u8 as *const i8,
        (*sfx).soundName.as_mut_ptr(),
    );
    buffer = (*sfx).soundData;
    while !buffer.is_null() {
        nbuffer = (*buffer).next;
        crate::src::client::snd_mem::SND_free(buffer);
        buffer = nbuffer
    }
    (*sfx).inMemory = crate::src::qcommon::q_shared::qfalse;
    (*sfx).soundData = 0 as *mut crate::snd_local_h::sndBuffer;
}
// =======================================================================
// Shutdown sound engine
// =======================================================================
#[no_mangle]

pub unsafe extern "C" fn S_Base_Shutdown() {
    if s_soundStarted == 0 {
        return;
    }
    crate::src::sdl::sdl_snd::SNDDMA_Shutdown();
    crate::src::client::snd_mem::SND_shutdown();
    s_soundStarted = 0;
    s_numSfx = 0;
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"s_info\x00" as *const u8 as *const i8);
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// snd_local.h -- private sound definations
// this is in samples
// samples
// floats
// floats
// the final values will be clamped to +/- 0x00ffff00 and shifted down
/* Previous output value */
/* Index into stepsize table */
// couldn't be loaded, so use buzz
// not in Memory
// not in Memory
// mono samples in buffer
// samples with all channels in buffer (samples divided by channels)
// don't mix less than this #
//arbitrary
// START_SAMPLE_IMMEDIATE = set immediately on next mix
// to allow overriding a specific sound
// to allow overriding a specific sound
// 0-255 volume after spatialization
// 0-255 volume after spatialization
// 0-255 volume before spatialization
// only use if fixed_origin is set
// use origin instead of fetching entnum's origin
// sfx structure
// chunk starts this many bytes from file start
// Interface between Q3 sound "api" and the sound backend
/*
====================================================================

  SYSTEM SPECIFIC FUNCTIONS

====================================================================
*/
// initializes cycling through a DMA buffer and returns information on it
// gets the current DMA position
// shutdown the DMA xfer.
//====================================================================
// spatializes a channel
// adpcm functions
// wavelet function
/*
================
S_Init
================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Base_Init(
    mut si: *mut crate::snd_local_h::soundInterface_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut r: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if si.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    s_mixahead = crate::src::qcommon::cvar::Cvar_Get(
        b"s_mixahead\x00" as *const u8 as *const i8,
        b"0.2\x00" as *const u8 as *const i8,
        0x1,
    );
    s_mixPreStep = crate::src::qcommon::cvar::Cvar_Get(
        b"s_mixPreStep\x00" as *const u8 as *const i8,
        b"0.05\x00" as *const u8 as *const i8,
        0x1,
    );
    s_show = crate::src::qcommon::cvar::Cvar_Get(
        b"s_show\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    s_testsound = crate::src::qcommon::cvar::Cvar_Get(
        b"s_testsound\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    r = crate::src::sdl::sdl_snd::SNDDMA_Init();
    if r as u64 != 0 {
        s_soundStarted = 1;
        s_soundMuted = crate::src::qcommon::q_shared::qtrue;
        //		s_numSfx = 0;
        crate::stdlib::memset(
            sfxHash.as_mut_ptr() as *mut libc::c_void,
            0,
            (::std::mem::size_of::<*mut crate::snd_local_h::sfx_t>()).wrapping_mul(128usize),
        );
        s_soundtime = 0;
        s_paintedtime = 0;
        S_Base_StopAllSounds();
    } else {
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*si).Shutdown = Some(S_Base_Shutdown as unsafe extern "C" fn() -> ());
    (*si).StartSound = Some(
        S_Base_StartSound
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: crate::src::qcommon::q_shared::sfxHandle_t,
            ) -> (),
    );
    (*si).StartLocalSound = Some(
        S_Base_StartLocalSound
            as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::sfxHandle_t, _: i32) -> (),
    );
    (*si).StartBackgroundTrack =
        Some(S_Base_StartBackgroundTrack as unsafe extern "C" fn(_: *const i8, _: *const i8) -> ());
    (*si).StopBackgroundTrack = Some(S_Base_StopBackgroundTrack as unsafe extern "C" fn() -> ());
    (*si).RawSamples = Some(
        S_Base_RawSamples
            as unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: *const crate::src::qcommon::q_shared::byte,
                _: f32,
                _: i32,
            ) -> (),
    );
    (*si).StopAllSounds = Some(S_Base_StopAllSounds as unsafe extern "C" fn() -> ());
    (*si).ClearLoopingSounds = Some(
        S_Base_ClearLoopingSounds
            as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
    );
    (*si).AddLoopingSound = Some(
        S_Base_AddLoopingSound
            as unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: crate::src::qcommon::q_shared::sfxHandle_t,
            ) -> (),
    );
    (*si).AddRealLoopingSound = Some(
        S_Base_AddRealLoopingSound
            as unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: crate::src::qcommon::q_shared::sfxHandle_t,
            ) -> (),
    );
    (*si).StopLoopingSound = Some(S_Base_StopLoopingSound as unsafe extern "C" fn(_: i32) -> ());
    (*si).Respatialize = Some(
        S_Base_Respatialize
            as unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
                _: i32,
            ) -> (),
    );
    (*si).UpdateEntityPosition = Some(
        S_Base_UpdateEntityPosition
            as unsafe extern "C" fn(_: i32, _: *const crate::src::qcommon::q_shared::vec_t) -> (),
    );
    (*si).Update = Some(S_Base_Update as unsafe extern "C" fn() -> ());
    (*si).DisableSounds = Some(S_Base_DisableSounds as unsafe extern "C" fn() -> ());
    (*si).BeginRegistration = Some(S_Base_BeginRegistration as unsafe extern "C" fn() -> ());
    (*si).RegisterSound = Some(
        S_Base_RegisterSound
            as unsafe extern "C" fn(
                _: *const i8,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    (*si).ClearSoundBuffer = Some(S_Base_ClearSoundBuffer as unsafe extern "C" fn() -> ());
    (*si).SoundInfo = Some(S_Base_SoundInfo as unsafe extern "C" fn() -> ());
    (*si).SoundList = Some(S_Base_SoundList as unsafe extern "C" fn() -> ());
    (*si).StartCapture = Some(S_Base_StartCapture as unsafe extern "C" fn() -> ());
    (*si).AvailableCaptureSamples =
        Some(S_Base_AvailableCaptureSamples as unsafe extern "C" fn() -> i32);
    (*si).Capture = Some(
        S_Base_Capture
            as unsafe extern "C" fn(_: i32, _: *mut crate::src::qcommon::q_shared::byte) -> (),
    );
    (*si).StopCapture = Some(S_Base_StopCapture as unsafe extern "C" fn() -> ());
    (*si).MasterGain = Some(S_Base_MasterGain as unsafe extern "C" fn(_: f32) -> ());
    return crate::src::qcommon::q_shared::qtrue;
}
