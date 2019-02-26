#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           libc,
           ptr_wrapping_offset_from)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
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
    pub type byte = libc::c_uchar;
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type qhandle_t = libc::c_int;
    pub type sfxHandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    // parameters to the main Error routine
    pub type unnamed = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed = 0;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
    //=====================================================================
    // in order from highest priority to lowest
// if none of the catchers are active, bound key strings will be executed
    // sound channels
// channel 0 never willingly overrides
// other channels will allways override a playing sound on that channel
    pub type unnamed_0 = libc::c_uint;
    // announcer voices, etc
    pub const CHAN_ANNOUNCER: unnamed_0 = 7;
    // chat messages, etc
    pub const CHAN_LOCAL_SOUND: unnamed_0 = 6;
    pub const CHAN_BODY: unnamed_0 = 5;
    pub const CHAN_ITEM: unnamed_0 = 4;
    pub const CHAN_VOICE: unnamed_0 = 3;
    pub const CHAN_WEAPON: unnamed_0 = 2;
    // menu sounds, etc
    pub const CHAN_LOCAL: unnamed_0 = 1;
    pub const CHAN_AUTO: unnamed_0 = 0;
    pub type connstate_t = libc::c_uint;
    // playing a cinematic or a static pic, not connected to a server
    pub const CA_CINEMATIC: connstate_t = 9;
    // game views should be displayed
    pub const CA_ACTIVE: connstate_t = 8;
    // got gamestate, waiting for first frame
    pub const CA_PRIMED: connstate_t = 7;
    // only during cgame initialization, never during main loop
    pub const CA_LOADING: connstate_t = 6;
    // netchan_t established, getting gamestate
    pub const CA_CONNECTED: connstate_t = 5;
    // sending challenge packets to the server
    pub const CA_CHALLENGING: connstate_t = 4;
    // sending request packets to the server
    pub const CA_CONNECTING: connstate_t = 3;
    // not used any more, was checking cd key 
    pub const CA_AUTHORIZING: connstate_t = 2;
    // not talking to a server
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CA_UNINITIALIZED: connstate_t = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
        #[no_mangle]
        pub fn VectorRotate(in_0: *mut vec_t, matrix: *mut vec3_t,
                            out: *mut vec_t);
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    //============================================================================
    /*
==============================================================

NET

==============================================================
*/
    // if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
    // disables ipv6 multicast support if set.
    // number of old messages that must be kept on client and
    // server for delta comrpession and ping estimation
    // max number of usercmd_t in a packet
    // max string commands buffered for restransmit
    pub type netadrtype_t = libc::c_uint;
    pub const NA_UNSPEC: netadrtype_t = 7;
    pub const NA_MULTICAST6: netadrtype_t = 6;
    pub const NA_IP6: netadrtype_t = 5;
    pub const NA_IP: netadrtype_t = 4;
    pub const NA_BROADCAST: netadrtype_t = 3;
    pub const NA_LOOPBACK: netadrtype_t = 2;
    pub const NA_BOT: netadrtype_t = 1;
    // an address lookup failed
    pub const NA_BAD: netadrtype_t = 0;
    pub type netsrc_t = libc::c_uint;
    pub const NS_SERVER: netsrc_t = 1;
    pub const NS_CLIENT: netsrc_t = 0;
    // maximum length of an IPv6 address string including trailing '\0'
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netadr_t {
        pub type_0: netadrtype_t,
        pub ip: [byte; 4],
        pub ip6: [byte; 16],
        pub port: libc::c_ushort,
        pub scope_id: libc::c_ulong,
    }
    // max length of a message, which may
    // be fragmented into multiple packets
    // ACK window of 48 download chunks. Cannot set this higher, or clients
    // will overflow the reliable commands buffer
    // 896 byte block chunks
    /*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_t {
        pub sock: netsrc_t,
        pub dropped: libc::c_int,
        pub remoteAddress: netadr_t,
        pub qport: libc::c_int,
        pub incomingSequence: libc::c_int,
        pub outgoingSequence: libc::c_int,
        pub fragmentSequence: libc::c_int,
        pub fragmentLength: libc::c_int,
        pub fragmentBuffer: [byte; 16384],
        pub unsentFragments: qboolean,
        pub unsentFragmentStart: libc::c_int,
        pub unsentLength: libc::c_int,
        pub unsentBuffer: [byte; 16384],
        pub challenge: libc::c_int,
        pub lastSentTime: libc::c_int,
        pub lastSentSize: libc::c_int,
        pub compat: qboolean,
    }
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, cvar_t};
    extern "C" {
        // called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
        #[no_mangle]
        pub fn Cmd_RemoveCommand(cmd_name: *const libc::c_char);
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // will be journaled properly
        #[no_mangle]
        pub fn Com_Milliseconds() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/snd_codec.h"]
pub mod snd_codec_h {
    pub type snd_stream_t = snd_stream_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_stream_s {
        pub codec: *mut snd_codec_t,
        pub file: fileHandle_t,
        pub info: snd_info_t,
        pub length: libc::c_int,
        pub pos: libc::c_int,
        pub ptr: *mut libc::c_void,
    }
    pub type snd_info_t = snd_info_s;
    /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)

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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_info_s {
        pub rate: libc::c_int,
        pub width: libc::c_int,
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub size: libc::c_int,
        pub dataofs: libc::c_int,
    }
    pub type snd_codec_t = snd_codec_s;
    // Codec data structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_codec_s {
        pub ext: *mut libc::c_char,
        pub load: CODEC_LOAD,
        pub open: CODEC_OPEN,
        pub read: CODEC_READ,
        pub close: CODEC_CLOSE,
        pub next: *mut snd_codec_t,
    }
    pub type CODEC_CLOSE
        =
        Option<unsafe extern "C" fn(_: *mut snd_stream_t) -> ()>;
    pub type CODEC_READ
        =
        Option<unsafe extern "C" fn(_: *mut snd_stream_t, _: libc::c_int,
                                    _: *mut libc::c_void) -> libc::c_int>;
    pub type CODEC_OPEN
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char)
                   -> *mut snd_stream_t>;
    // Codec functions
    pub type CODEC_LOAD
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char,
                                    _: *mut snd_info_t) -> *mut libc::c_void>;
    use super::q_shared_h::{fileHandle_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn S_CodecCloseStream(stream: *mut snd_stream_t);
        #[no_mangle]
        pub fn S_CodecOpenStream(filename: *const libc::c_char)
         -> *mut snd_stream_t;
        #[no_mangle]
        pub fn S_CodecReadStream(stream: *mut snd_stream_t,
                                 bytes: libc::c_int,
                                 buffer: *mut libc::c_void) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/snd_local.h"]
pub mod snd_local_h {
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct portable_samplepair_t {
        pub left: libc::c_int,
        pub right: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dma_t {
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub fullsamples: libc::c_int,
        pub submission_chunk: libc::c_int,
        pub samplebits: libc::c_int,
        pub isfloat: libc::c_int,
        pub speed: libc::c_int,
        pub buffer: *mut byte,
    }
    pub type loopSound_t = loopSound_s;
    //arbitrary
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct loopSound_s {
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub sfx: *mut sfx_t,
        pub mergeFrame: libc::c_int,
        pub active: qboolean,
        pub kill: qboolean,
        pub doppler: qboolean,
        pub dopplerScale: libc::c_float,
        pub oldDopplerScale: libc::c_float,
        pub framenum: libc::c_int,
    }
    pub type sfx_t = sfx_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sfx_s {
        pub soundData: *mut sndBuffer,
        pub defaultSound: qboolean,
        pub inMemory: qboolean,
        pub soundCompressed: qboolean,
        pub soundCompressionMethod: libc::c_int,
        pub soundLength: libc::c_int,
        pub soundChannels: libc::c_int,
        pub soundName: [libc::c_char; 64],
        pub lastTimeUsed: libc::c_int,
        pub next: *mut sfx_s,
    }
    pub type sndBuffer = sndBuffer_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sndBuffer_s {
        pub sndChunk: [libc::c_short; 1024],
        pub next: *mut sndBuffer_s,
        pub size: libc::c_int,
        pub adpcm: adpcm_state_t,
    }
    pub type adpcm_state_t = adpcm_state;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct adpcm_state {
        pub sample: libc::c_short,
        pub index: libc::c_char,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct channel_t {
        pub allocTime: libc::c_int,
        pub startSample: libc::c_int,
        pub entnum: libc::c_int,
        pub entchannel: libc::c_int,
        pub leftvol: libc::c_int,
        pub rightvol: libc::c_int,
        pub master_vol: libc::c_int,
        pub dopplerScale: libc::c_float,
        pub oldDopplerScale: libc::c_float,
        pub origin: vec3_t,
        pub fixed_origin: qboolean,
        pub thesfx: *mut sfx_t,
        pub doppler: qboolean,
        pub fullVolume: qboolean,
    }
    // Interface between Q3 sound "api" and the sound backend
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct soundInterface_t {
        pub Shutdown: Option<unsafe extern "C" fn() -> ()>,
        pub StartSound: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: sfxHandle_t) -> ()>,
        pub StartLocalSound: Option<unsafe extern "C" fn(_: sfxHandle_t,
                                                         _: libc::c_int)
                                        -> ()>,
        pub StartBackgroundTrack: Option<unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _:
                                                                  *const libc::c_char)
                                             -> ()>,
        pub StopBackgroundTrack: Option<unsafe extern "C" fn() -> ()>,
        pub RawSamples: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *const byte,
                                                    _: libc::c_float,
                                                    _: libc::c_int) -> ()>,
        pub StopAllSounds: Option<unsafe extern "C" fn() -> ()>,
        pub ClearLoopingSounds: Option<unsafe extern "C" fn(_: qboolean)
                                           -> ()>,
        pub AddLoopingSound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: *const vec_t,
                                                         _: *const vec_t,
                                                         _: sfxHandle_t)
                                        -> ()>,
        pub AddRealLoopingSound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: *const vec_t,
                                                             _: *const vec_t,
                                                             _: sfxHandle_t)
                                            -> ()>,
        pub StopLoopingSound: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub Respatialize: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const vec_t,
                                                      _: *mut vec3_t,
                                                      _: libc::c_int) -> ()>,
        pub UpdateEntityPosition: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _: *const vec_t)
                                             -> ()>,
        pub Update: Option<unsafe extern "C" fn() -> ()>,
        pub DisableSounds: Option<unsafe extern "C" fn() -> ()>,
        pub BeginRegistration: Option<unsafe extern "C" fn() -> ()>,
        pub RegisterSound: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: qboolean)
                                      -> sfxHandle_t>,
        pub ClearSoundBuffer: Option<unsafe extern "C" fn() -> ()>,
        pub SoundInfo: Option<unsafe extern "C" fn() -> ()>,
        pub SoundList: Option<unsafe extern "C" fn() -> ()>,
        pub StartCapture: Option<unsafe extern "C" fn() -> ()>,
        pub AvailableCaptureSamples: Option<unsafe extern "C" fn()
                                                -> libc::c_int>,
        pub Capture: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut byte)
                                -> ()>,
        pub StopCapture: Option<unsafe extern "C" fn() -> ()>,
        pub MasterGain: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    }
    use super::{libc};
    use super::q_shared_h::{byte, vec3_t, qboolean, vec_t, sfxHandle_t,
                            cvar_t};
    extern "C" {
        #[no_mangle]
        pub static mut s_musicVolume: *mut cvar_t;
        #[no_mangle]
        pub static mut s_volume: *mut cvar_t;
        #[no_mangle]
        pub static mut s_muted: *mut cvar_t;
        /*
====================================================================

  SYSTEM SPECIFIC FUNCTIONS

====================================================================
*/
        // initializes cycling through a DMA buffer and returns information on it
        #[no_mangle]
        pub fn SNDDMA_Init() -> qboolean;
        // gets the current DMA position
        #[no_mangle]
        pub fn SNDDMA_GetDMAPos() -> libc::c_int;
        // shutdown the DMA xfer.
        #[no_mangle]
        pub fn SNDDMA_Shutdown();
        #[no_mangle]
        pub fn SNDDMA_BeginPainting();
        #[no_mangle]
        pub fn SNDDMA_Submit();
        #[no_mangle]
        pub fn SNDDMA_StartCapture();
        #[no_mangle]
        pub fn SNDDMA_AvailableCaptureSamples() -> libc::c_int;
        #[no_mangle]
        pub fn SNDDMA_Capture(samples: libc::c_int, data: *mut byte);
        #[no_mangle]
        pub fn SNDDMA_StopCapture();
        #[no_mangle]
        pub fn SNDDMA_MasterGain(val: libc::c_float);
        #[no_mangle]
        pub static mut s_doppler: *mut cvar_t;
        #[no_mangle]
        pub fn S_LoadSound(sfx: *mut sfx_t) -> qboolean;
        #[no_mangle]
        pub fn SND_free(v: *mut sndBuffer);
        #[no_mangle]
        pub fn SND_malloc() -> *mut sndBuffer;
        #[no_mangle]
        pub fn SND_setup();
        #[no_mangle]
        pub fn SND_shutdown();
        #[no_mangle]
        pub fn S_PaintChannels(endtime: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
    /*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientConnection_t {
        pub state: connstate_t,
        pub clientNum: libc::c_int,
        pub lastPacketSentTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub servername: [libc::c_char; 4096],
        pub serverAddress: netadr_t,
        pub connectTime: libc::c_int,
        pub connectPacketCount: libc::c_int,
        pub serverMessage: [libc::c_char; 1024],
        pub challenge: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub serverMessageSequence: libc::c_int,
        pub serverCommandSequence: libc::c_int,
        pub lastExecutedServerCommand: libc::c_int,
        pub serverCommands: [[libc::c_char; 1024]; 64],
        pub download: fileHandle_t,
        pub downloadTempName: [libc::c_char; 4096],
        pub downloadName: [libc::c_char; 4096],
        pub cURLEnabled: qboolean,
        pub cURLUsed: qboolean,
        pub cURLDisconnected: qboolean,
        pub downloadURL: [libc::c_char; 4096],
        pub downloadCURL: *mut libc::c_void,
        pub downloadCURLM: *mut libc::c_void,
        pub sv_allowDownload: libc::c_int,
        pub sv_dlURL: [libc::c_char; 256],
        pub downloadNumber: libc::c_int,
        pub downloadBlock: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadSize: libc::c_int,
        pub downloadList: [libc::c_char; 1024],
        pub downloadRestart: qboolean,
        pub demoName: [libc::c_char; 64],
        pub spDemoRecording: qboolean,
        pub demorecording: qboolean,
        pub demoplaying: qboolean,
        pub demowaiting: qboolean,
        pub firstDemoFrameSkipped: qboolean,
        pub demofile: fileHandle_t,
        pub timeDemoFrames: libc::c_int,
        pub timeDemoStart: libc::c_int,
        pub timeDemoBaseTime: libc::c_int,
        pub timeDemoLastFrame: libc::c_int,
        pub timeDemoMinDuration: libc::c_int,
        pub timeDemoMaxDuration: libc::c_int,
        pub timeDemoDurations: [libc::c_uchar; 4096],
        pub aviVideoFrameRemainder: libc::c_float,
        pub aviSoundFrameRemainder: libc::c_float,
        pub voipEnabled: qboolean,
        pub voipCodecInitialized: qboolean,
        pub opusDecoder: [*mut OpusDecoder; 64],
        pub voipIncomingGeneration: [byte; 64],
        pub voipIncomingSequence: [libc::c_int; 64],
        pub voipGain: [libc::c_float; 64],
        pub voipIgnore: [qboolean; 64],
        pub voipMuteAll: qboolean,
        pub voipTargets: [uint8_t; 8],
        pub voipFlags: uint8_t,
        pub opusEncoder: *mut OpusEncoder,
        pub voipOutgoingDataSize: libc::c_int,
        pub voipOutgoingDataFrames: libc::c_int,
        pub voipOutgoingSequence: libc::c_int,
        pub voipOutgoingGeneration: byte,
        pub voipOutgoingData: [byte; 1024],
        pub voipPower: libc::c_float,
        pub compat: qboolean,
        pub netchan: netchan_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientStatic_t {
        pub cddialog: qboolean,
        pub rendererStarted: qboolean,
        pub soundStarted: qboolean,
        pub soundRegistered: qboolean,
        pub uiStarted: qboolean,
        pub cgameStarted: qboolean,
        pub framecount: libc::c_int,
        pub frametime: libc::c_int,
        pub realtime: libc::c_int,
        pub realFrametime: libc::c_int,
        pub numlocalservers: libc::c_int,
        pub localServers: [serverInfo_t; 128],
        pub numglobalservers: libc::c_int,
        pub globalServers: [serverInfo_t; 4096],
        pub numGlobalServerAddresses: libc::c_int,
        pub globalServerAddresses: [netadr_t; 4096],
        pub numfavoriteservers: libc::c_int,
        pub favoriteServers: [serverInfo_t; 128],
        pub pingUpdateSource: libc::c_int,
        pub updateServer: netadr_t,
        pub updateChallenge: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub authorizeServer: netadr_t,
        pub rconAddress: netadr_t,
        pub glconfig: glconfig_t,
        pub charSetShader: qhandle_t,
        pub whiteShader: qhandle_t,
        pub consoleShader: qhandle_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverInfo_t {
        pub adr: netadr_t,
        pub hostName: [libc::c_char; 32],
        pub mapName: [libc::c_char; 32],
        pub game: [libc::c_char; 32],
        pub netType: libc::c_int,
        pub gameType: libc::c_int,
        pub clients: libc::c_int,
        pub maxClients: libc::c_int,
        pub minPing: libc::c_int,
        pub maxPing: libc::c_int,
        pub ping: libc::c_int,
        pub visible: qboolean,
        pub punkbuster: libc::c_int,
        pub g_humanplayers: libc::c_int,
        pub g_needpass: libc::c_int,
    }
    use super::q_shared_h::{connstate_t, fileHandle_t, qboolean, byte,
                            qhandle_t, cvar_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    extern "C" {
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut cl_aviFrameRate: *mut cvar_t;
        #[no_mangle]
        pub fn CL_VideoRecording() -> qboolean;
        #[no_mangle]
        pub static mut cls: clientStatic_t;
    }
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    extern "C" {
        /* Copyright (c) 2010-2011 Xiph.Org Foundation, Skype Limited
   Written by Jean-Marc Valin and Koen Vos */
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
        /* *
 * @file opus.h
 * @brief Opus reference implementation API
 */
        /* *
 * @mainpage Opus
 *
 * The Opus codec is designed for interactive speech and audio transmission over the Internet.
 * It is designed by the IETF Codec Working Group and incorporates technology from
 * Skype's SILK codec and Xiph.Org's CELT codec.
 *
 * The Opus codec is designed to handle a wide range of interactive audio applications,
 * including Voice over IP, videoconferencing, in-game chat, and even remote live music
 * performances. It can scale from low bit-rate narrowband speech to very high quality
 * stereo music. Its main features are:

 * @li Sampling rates from 8 to 48 kHz
 * @li Bit-rates from 6 kb/s to 510 kb/s
 * @li Support for both constant bit-rate (CBR) and variable bit-rate (VBR)
 * @li Audio bandwidth from narrowband to full-band
 * @li Support for speech and music
 * @li Support for mono and stereo
 * @li Support for multichannel (up to 255 channels)
 * @li Frame sizes from 2.5 ms to 60 ms
 * @li Good loss robustness and packet loss concealment (PLC)
 * @li Floating point and fixed-point implementation
 *
 * Documentation sections:
 * @li @ref opus_encoder
 * @li @ref opus_decoder
 * @li @ref opus_repacketizer
 * @li @ref opus_multistream
 * @li @ref opus_libinfo
 * @li @ref opus_custom
 */
        /* * @defgroup opus_encoder Opus Encoder
  * @{
  *
  * @brief This page describes the process and functions used to encode Opus.
  *
  * Since Opus is a stateful codec, the encoding process starts with creating an encoder
  * state. This can be done with:
  *
  * @code
  * int          error;
  * OpusEncoder *enc;
  * enc = opus_encoder_create(Fs, channels, application, &error);
  * @endcode
  *
  * From this point, @c enc can be used for encoding an audio stream. An encoder state
  * @b must @b not be used for more than one stream at the same time. Similarly, the encoder
  * state @b must @b not be re-initialized for each frame.
  *
  * While opus_encoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  *
  * @code
  * int          size;
  * int          error;
  * OpusEncoder *enc;
  * size = opus_encoder_get_size(channels);
  * enc = malloc(size);
  * error = opus_encoder_init(enc, Fs, channels, application);
  * @endcode
  *
  * where opus_encoder_get_size() returns the required size for the encoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The encoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * It is possible to change some of the encoder's settings using the opus_encoder_ctl()
  * interface. All these settings already default to the recommended value, so they should
  * only be changed when necessary. The most common settings one may want to change are:
  *
  * @code
  * opus_encoder_ctl(enc, OPUS_SET_BITRATE(bitrate));
  * opus_encoder_ctl(enc, OPUS_SET_COMPLEXITY(complexity));
  * opus_encoder_ctl(enc, OPUS_SET_SIGNAL(signal_type));
  * @endcode
  *
  * where
  *
  * @arg bitrate is in bits per second (b/s)
  * @arg complexity is a value from 1 to 10, where 1 is the lowest complexity and 10 is the highest
  * @arg signal_type is either OPUS_AUTO (default), OPUS_SIGNAL_VOICE, or OPUS_SIGNAL_MUSIC
  *
  * See @ref opus_encoderctls and @ref opus_genericctls for a complete list of parameters that can be set or queried. Most parameters can be set or changed at any time during a stream.
  *
  * To encode a frame, opus_encode() or opus_encode_float() must be called with exactly one frame (2.5, 5, 10, 20, 40 or 60 ms) of audio data:
  * @code
  * len = opus_encode(enc, audio_frame, frame_size, packet, max_packet);
  * @endcode
  *
  * where
  * <ul>
  * <li>audio_frame is the audio data in opus_int16 (or float for opus_encode_float())</li>
  * <li>frame_size is the duration of the frame in samples (per channel)</li>
  * <li>packet is the byte array to which the compressed data is written</li>
  * <li>max_packet is the maximum number of bytes that can be written in the packet (4000 bytes is recommended).
  *     Do not use max_packet to control VBR target bitrate, instead use the #OPUS_SET_BITRATE CTL.</li>
  * </ul>
  *
  * opus_encode() and opus_encode_float() return the number of bytes actually written to the packet.
  * The return value <b>can be negative</b>, which indicates that an error has occurred. If the return value
  * is 2 bytes or less, then the packet does not need to be transmitted (DTX).
  *
  * Once the encoder state if no longer needed, it can be destroyed with
  *
  * @code
  * opus_encoder_destroy(enc);
  * @endcode
  *
  * If the encoder was created with opus_encoder_init() rather than opus_encoder_create(),
  * then no action is required aside from potentially freeing the memory that was manually
  * allocated for it (calling free(enc) for the example above)
  *
  */
        /* * Opus encoder state.
  * This contains the complete state of an Opus encoder.
  * It is position independent and can be freely copied.
  * @see opus_encoder_create,opus_encoder_init
  */
        pub type OpusEncoder;
        /* *@}*/
        /* * @defgroup opus_decoder Opus Decoder
  * @{
  *
  * @brief This page describes the process and functions used to decode Opus.
  *
  * The decoding process also starts with creating a decoder
  * state. This can be done with:
  * @code
  * int          error;
  * OpusDecoder *dec;
  * dec = opus_decoder_create(Fs, channels, &error);
  * @endcode
  * where
  * @li Fs is the sampling rate and must be 8000, 12000, 16000, 24000, or 48000
  * @li channels is the number of channels (1 or 2)
  * @li error will hold the error code in case of failure (or #OPUS_OK on success)
  * @li the return value is a newly created decoder state to be used for decoding
  *
  * While opus_decoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  * @code
  * int          size;
  * int          error;
  * OpusDecoder *dec;
  * size = opus_decoder_get_size(channels);
  * dec = malloc(size);
  * error = opus_decoder_init(dec, Fs, channels);
  * @endcode
  * where opus_decoder_get_size() returns the required size for the decoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The decoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * To decode a frame, opus_decode() or opus_decode_float() must be called with a packet of compressed audio data:
  * @code
  * frame_size = opus_decode(dec, packet, len, decoded, max_size, 0);
  * @endcode
  * where
  *
  * @li packet is the byte array containing the compressed data
  * @li len is the exact number of bytes contained in the packet
  * @li decoded is the decoded audio data in opus_int16 (or float for opus_decode_float())
  * @li max_size is the max duration of the frame in samples (per channel) that can fit into the decoded_frame array
  *
  * opus_decode() and opus_decode_float() return the number of samples (per channel) decoded from the packet.
  * If that value is negative, then an error has occurred. This can occur if the packet is corrupted or if the audio
  * buffer is too small to hold the decoded audio.
  *
  * Opus is a stateful codec with overlapping blocks and as a result Opus
  * packets are not coded independently of each other. Packets must be
  * passed into the decoder serially and in the correct order for a correct
  * decode. Lost packets can be replaced with loss concealment by calling
  * the decoder with a null pointer and zero length for the missing packet.
  *
  * A single codec state may only be accessed from a single thread at
  * a time and any required locking must be performed by the caller. Separate
  * streams must be decoded with separate decoder states and can be decoded
  * in parallel unless the library was compiled with NONTHREADSAFE_PSEUDOSTACK
  * defined.
  *
  */
        /* * Opus decoder state.
  * This contains the complete state of an Opus decoder.
  * It is position independent and can be freely copied.
  * @see opus_decoder_create,opus_decoder_init
  */
        pub type OpusDecoder;
    }
}
#[header_src = "/usr/include/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src =
      "ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
    pub type textureCompression_t = libc::c_uint;
    // this is for the GL_EXT_texture_compression_s3tc extension.
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    // this is for the GL_S3_s3tc extension.
    pub const TC_S3TC: textureCompression_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    // where you don't have src*dst
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    // where you can't modulate alpha on alpha textures
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    // the hardware type then there can NOT exist a secondary
							// display adapter
    // where you can't interpolate alpha
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    // Voodoo Banshee or Voodoo3, relevant since if this is
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    // where everything works the way it should
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    pub type glDriverType_t = libc::c_uint;
    // driver is a 3Dfx standalone driver
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    // WARNING: there are tests that check for
								// > GLDRV_ICD for minidriverness, so this
								// should always be the lowest value in this
								// enum set
    // driver is a non-3Dfx standalone driver
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    // driver is integrated with window system
    pub const GLDRV_ICD: glDriverType_t = 0;
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    extern "C" {
        #[no_mangle]
        pub fn S_DisplayFreeMemory();
    }
}
#[header_src =
      "ioq3/code/client/snd_dma.c"]
pub mod snd_dma_c {
    use super::snd_codec_h::{snd_stream_t};
    use super::{libc};
    use super::q_shared_h::{byte, vec_t, sfxHandle_t, qboolean, cvar_t,
                            vec3_t};
    use super::snd_local_h::{sfx_t, channel_t};
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, sfxHandle_t,
                       fileHandle_t, unnamed, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, vec_t,
                       vec3_t, cvar_s, cvar_t, unnamed_0, CHAN_ANNOUNCER,
                       CHAN_LOCAL_SOUND, CHAN_BODY, CHAN_ITEM, CHAN_VOICE,
                       CHAN_WEAPON, CHAN_LOCAL, CHAN_AUTO, connstate_t,
                       CA_CINEMATIC, CA_ACTIVE, CA_PRIMED, CA_LOADING,
                       CA_CONNECTED, CA_CHALLENGING, CA_CONNECTING,
                       CA_AUTHORIZING, CA_DISCONNECTED, CA_UNINITIALIZED,
                       VectorNormalize, VectorRotate, Q_stricmp, Q_strncpyz,
                       Com_Error, Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      Cmd_RemoveCommand, Cvar_Get, Com_DPrintf,
                      Com_Milliseconds};
use self::snd_codec_h::{snd_stream_t, snd_stream_s, snd_info_t, snd_info_s,
                        snd_codec_t, snd_codec_s, CODEC_CLOSE, CODEC_READ,
                        CODEC_OPEN, CODEC_LOAD, S_CodecCloseStream,
                        S_CodecOpenStream, S_CodecReadStream};
use self::snd_local_h::{portable_samplepair_t, dma_t, loopSound_t,
                        loopSound_s, sfx_t, sfx_s, sndBuffer, sndBuffer_s,
                        adpcm_state_t, adpcm_state, channel_t,
                        soundInterface_t, s_musicVolume, s_volume, s_muted,
                        SNDDMA_Init, SNDDMA_GetDMAPos, SNDDMA_Shutdown,
                        SNDDMA_BeginPainting, SNDDMA_Submit,
                        SNDDMA_StartCapture, SNDDMA_AvailableCaptureSamples,
                        SNDDMA_Capture, SNDDMA_StopCapture, SNDDMA_MasterGain,
                        s_doppler, S_LoadSound, SND_free, SND_malloc,
                        SND_setup, SND_shutdown, S_PaintChannels};
use self::client_h::{clientConnection_t, clientStatic_t, serverInfo_t, clc,
                     cl_aviFrameRate, CL_VideoRecording, cls};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::multi_h::{CURLM};
use self::curl_h::{CURL};
use self::tr_types_h::{glconfig_t, textureCompression_t, TC_S3TC_ARB, TC_S3TC,
                       TC_NONE, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glDriverType_t, GLDRV_VOODOO,
                       GLDRV_STANDALONE, GLDRV_ICD};
use self::string_h::{memset, strcpy, strlen};
use self::ctype_h::{tolower};
use self::snd_public_h::{S_DisplayFreeMemory};
unsafe extern "C" fn VectorLengthSquared(mut v: *const vec_t) -> vec_t {
    return *v.offset(0isize) * *v.offset(0isize) +
               *v.offset(1isize) * *v.offset(1isize) +
               *v.offset(2isize) * *v.offset(2isize);
}
unsafe extern "C" fn DistanceSquared(mut p1: *const vec_t,
                                     mut p2: *const vec_t) -> vec_t {
    let mut v: vec3_t = [0.; 3];
    v[0usize] = *p2.offset(0isize) - *p1.offset(0isize);
    v[1usize] = *p2.offset(1isize) - *p1.offset(1isize);
    v[2usize] = *p2.offset(2isize) - *p1.offset(2isize);
    return v[0usize] * v[0usize] + v[1usize] * v[1usize] +
               v[2usize] * v[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn S_UpdateBackgroundTrack() {
    let mut bufferSamples: libc::c_int = 0;
    let mut fileSamples: libc::c_int = 0;
    // just enough to fit in a mac stack frame
    let mut raw: [byte; 30000] = [0; 30000];
    let mut fileBytes: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if s_backgroundStream.is_null() { return }
    if (*s_musicVolume).value <= 0i32 as libc::c_float { return }
    if s_rawend[0usize] < s_soundtime { s_rawend[0usize] = s_soundtime }
    while s_rawend[0usize] < s_soundtime + 16384i32 {
        bufferSamples = 16384i32 - (s_rawend[0usize] - s_soundtime);
        fileSamples =
            bufferSamples * (*s_backgroundStream).info.rate / dma.speed;
        if 0 == fileSamples { return }
        fileBytes =
            fileSamples *
                ((*s_backgroundStream).info.width *
                     (*s_backgroundStream).info.channels);
        if fileBytes as libc::c_ulong >
               ::std::mem::size_of::<[byte; 30000]>() as libc::c_ulong {
            fileBytes =
                ::std::mem::size_of::<[byte; 30000]>() as libc::c_ulong as
                    libc::c_int;
            fileSamples =
                fileBytes /
                    ((*s_backgroundStream).info.width *
                         (*s_backgroundStream).info.channels)
        }
        r =
            S_CodecReadStream(s_backgroundStream, fileBytes,
                              raw.as_mut_ptr() as *mut libc::c_void);
        if r < fileBytes {
            fileSamples =
                r /
                    ((*s_backgroundStream).info.width *
                         (*s_backgroundStream).info.channels)
        }
        if r > 0i32 {
            S_Base_RawSamples(0i32, fileSamples,
                              (*s_backgroundStream).info.rate,
                              (*s_backgroundStream).info.width,
                              (*s_backgroundStream).info.channels,
                              raw.as_mut_ptr(), (*s_musicVolume).value,
                              -1i32);
        } else if 0 != s_backgroundLoop[0usize] {
            S_OpenBackgroundStream(s_backgroundLoop.as_mut_ptr());
            if s_backgroundStream.is_null() { return }
        } else { S_Base_StopBackgroundTrack(); return }
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_Base_StopBackgroundTrack() {
    if s_backgroundStream.is_null() { return }
    S_CodecCloseStream(s_backgroundStream);
    s_backgroundStream = 0 as *mut snd_stream_t;
    s_rawend[0usize] = 0i32;
}
#[no_mangle]
pub static mut s_rawend: [libc::c_int; 129] = [0; 129];
#[no_mangle]
pub static mut s_backgroundStream: *mut snd_stream_t =
    0 as *const snd_stream_t as *mut snd_stream_t;
static mut s_backgroundLoop: [libc::c_char; 64] = [0; 64];
/*
======================
S_OpenBackgroundStream
======================
*/
unsafe extern "C" fn S_OpenBackgroundStream(mut filename:
                                                *const libc::c_char) {
    if !s_backgroundStream.is_null() {
        S_CodecCloseStream(s_backgroundStream);
        s_backgroundStream = 0 as *mut snd_stream_t
    }
    s_backgroundStream = S_CodecOpenStream(filename);
    if s_backgroundStream.is_null() {
        Com_Printf(b"^3WARNING: couldn\'t open music file %s\n\x00" as
                       *const u8 as *const libc::c_char, filename);
        return
    }
    if (*s_backgroundStream).info.channels != 2i32 ||
           (*s_backgroundStream).info.rate != 22050i32 {
        Com_Printf(b"^3WARNING: music file %s is not 22k stereo\n\x00" as
                       *const u8 as *const libc::c_char, filename);
    };
}
/*
============
S_Base_RawSamples

Music streaming
============
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_RawSamples(mut stream: libc::c_int,
                                           mut samples: libc::c_int,
                                           mut rate: libc::c_int,
                                           mut width: libc::c_int,
                                           mut s_channels_0: libc::c_int,
                                           mut data: *const byte,
                                           mut volume: libc::c_float,
                                           mut entityNum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut src: libc::c_int = 0;
    let mut dst: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut intVolumeLeft: libc::c_int = 0;
    let mut intVolumeRight: libc::c_int = 0;
    let mut rawsamples: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    if stream < 0i32 || stream >= 64i32 * 2i32 + 1i32 { return }
    rawsamples = s_rawsamples[stream as usize].as_mut_ptr();
    if 0 != (*s_muted).integer {
        intVolumeRight = 0i32;
        intVolumeLeft = intVolumeRight
    } else {
        let mut leftvol: libc::c_int = 0;
        let mut rightvol: libc::c_int = 0;
        if entityNum >= 0i32 && entityNum < 1i32 << 10i32 {
            S_SpatializeOrigin(loopSounds[entityNum as
                                              usize].origin.as_mut_ptr(),
                               256i32, &mut leftvol, &mut rightvol);
        } else { rightvol = 256i32; leftvol = rightvol }
        intVolumeLeft =
            (leftvol as libc::c_float * volume * (*s_volume).value) as
                libc::c_int;
        intVolumeRight =
            (rightvol as libc::c_float * volume * (*s_volume).value) as
                libc::c_int
    }
    if s_rawend[stream as usize] < s_soundtime {
        Com_DPrintf(b"S_Base_RawSamples: resetting minimum: %i < %i\n\x00" as
                        *const u8 as *const libc::c_char,
                    s_rawend[stream as usize], s_soundtime);
        s_rawend[stream as usize] = s_soundtime
    }
    scale = rate as libc::c_float / dma.speed as libc::c_float;
    if s_channels_0 == 2i32 && width == 2i32 {
        if scale as libc::c_double == 1.0f64 {
            i = 0i32;
            while i < samples {
                dst = s_rawend[stream as usize] & 16384i32 - 1i32;
                s_rawend[stream as usize] += 1;
                (*rawsamples.offset(dst as isize)).left =
                    *(data as *mut libc::c_short).offset((i * 2i32) as isize)
                        as libc::c_int * intVolumeLeft;
                (*rawsamples.offset(dst as isize)).right =
                    *(data as
                          *mut libc::c_short).offset((i * 2i32 + 1i32) as
                                                         isize) as libc::c_int
                        * intVolumeRight;
                i += 1
            }
        } else {
            i = 0i32;
            loop  {
                src = (i as libc::c_float * scale) as libc::c_int;
                if src >= samples { break ; }
                dst = s_rawend[stream as usize] & 16384i32 - 1i32;
                s_rawend[stream as usize] += 1;
                (*rawsamples.offset(dst as isize)).left =
                    *(data as
                          *mut libc::c_short).offset((src * 2i32) as isize) as
                        libc::c_int * intVolumeLeft;
                (*rawsamples.offset(dst as isize)).right =
                    *(data as
                          *mut libc::c_short).offset((src * 2i32 + 1i32) as
                                                         isize) as libc::c_int
                        * intVolumeRight;
                i += 1
            }
        }
    } else if s_channels_0 == 1i32 && width == 2i32 {
        i = 0i32;
        loop  {
            src = (i as libc::c_float * scale) as libc::c_int;
            if src >= samples { break ; }
            dst = s_rawend[stream as usize] & 16384i32 - 1i32;
            s_rawend[stream as usize] += 1;
            (*rawsamples.offset(dst as isize)).left =
                *(data as *mut libc::c_short).offset(src as isize) as
                    libc::c_int * intVolumeLeft;
            (*rawsamples.offset(dst as isize)).right =
                *(data as *mut libc::c_short).offset(src as isize) as
                    libc::c_int * intVolumeRight;
            i += 1
        }
    } else if s_channels_0 == 2i32 && width == 1i32 {
        intVolumeLeft *= 256i32;
        intVolumeRight *= 256i32;
        i = 0i32;
        loop  {
            src = (i as libc::c_float * scale) as libc::c_int;
            if src >= samples { break ; }
            dst = s_rawend[stream as usize] & 16384i32 - 1i32;
            s_rawend[stream as usize] += 1;
            (*rawsamples.offset(dst as isize)).left =
                *(data as *mut libc::c_char).offset((src * 2i32) as isize) as
                    libc::c_int * intVolumeLeft;
            (*rawsamples.offset(dst as isize)).right =
                *(data as
                      *mut libc::c_char).offset((src * 2i32 + 1i32) as isize)
                    as libc::c_int * intVolumeRight;
            i += 1
        }
    } else if s_channels_0 == 1i32 && width == 1i32 {
        intVolumeLeft *= 256i32;
        intVolumeRight *= 256i32;
        i = 0i32;
        loop  {
            src = (i as libc::c_float * scale) as libc::c_int;
            if src >= samples { break ; }
            dst = s_rawend[stream as usize] & 16384i32 - 1i32;
            s_rawend[stream as usize] += 1;
            (*rawsamples.offset(dst as isize)).left =
                (*(data as *mut byte).offset(src as isize) as libc::c_int -
                     128i32) * intVolumeLeft;
            (*rawsamples.offset(dst as isize)).right =
                (*(data as *mut byte).offset(src as isize) as libc::c_int -
                     128i32) * intVolumeRight;
            i += 1
        }
    }
    if s_rawend[stream as usize] > s_soundtime + 16384i32 {
        Com_DPrintf(b"S_Base_RawSamples: overflowed %i > %i\n\x00" as
                        *const u8 as *const libc::c_char,
                    s_rawend[stream as usize], s_soundtime);
    };
}
// sample PAIRS
#[no_mangle]
pub static mut s_soundtime: libc::c_int = 0;
#[no_mangle]
pub static mut dma: dma_t =
    dma_t{channels: 0,
          samples: 0,
          fullsamples: 0,
          submission_chunk: 0,
          samplebits: 0,
          isfloat: 0,
          speed: 0,
          buffer: 0 as *const byte as *mut byte,};
static mut loopSounds: [loopSound_t; 1024] =
    [loopSound_s{origin: [0.; 3],
                 velocity: [0.; 3],
                 sfx: 0 as *const sfx_t as *mut sfx_t,
                 mergeFrame: 0,
                 active: qfalse,
                 kill: qfalse,
                 doppler: qfalse,
                 dopplerScale: 0.,
                 oldDopplerScale: 0.,
                 framenum: 0,}; 1024];
//=============================================================================
/*
=================
S_SpatializeOrigin

Used for spatializing s_channels
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SpatializeOrigin(mut origin: *mut vec_t,
                                            mut master_vol: libc::c_int,
                                            mut left_vol: *mut libc::c_int,
                                            mut right_vol: *mut libc::c_int) {
    let mut dot: vec_t = 0.;
    let mut dist: vec_t = 0.;
    let mut lscale: vec_t = 0.;
    let mut rscale: vec_t = 0.;
    let mut scale: vec_t = 0.;
    let mut source_vec: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let dist_mult: libc::c_float = 0.0008f32;
    source_vec[0usize] = *origin.offset(0isize) - listener_origin[0usize];
    source_vec[1usize] = *origin.offset(1isize) - listener_origin[1usize];
    source_vec[2usize] = *origin.offset(2isize) - listener_origin[2usize];
    dist = VectorNormalize(source_vec.as_mut_ptr());
    dist -= 80i32 as libc::c_float;
    if dist < 0i32 as libc::c_float { dist = 0i32 as vec_t }
    dist *= dist_mult;
    VectorRotate(source_vec.as_mut_ptr(), listener_axis.as_mut_ptr(),
                 vec.as_mut_ptr());
    dot = -vec[1usize];
    if dma.channels == 1i32 {
        rscale = 1.0f64 as vec_t;
        lscale = 1.0f64 as vec_t
    } else {
        rscale = (0.5f64 * (1.0f64 + dot as libc::c_double)) as vec_t;
        lscale = (0.5f64 * (1.0f64 - dot as libc::c_double)) as vec_t;
        if rscale < 0i32 as libc::c_float { rscale = 0i32 as vec_t }
        if lscale < 0i32 as libc::c_float { lscale = 0i32 as vec_t }
    }
    scale =
        ((1.0f64 - dist as libc::c_double) * rscale as libc::c_double) as
            vec_t;
    *right_vol = (master_vol as libc::c_float * scale) as libc::c_int;
    if *right_vol < 0i32 { *right_vol = 0i32 }
    scale =
        ((1.0f64 - dist as libc::c_double) * lscale as libc::c_double) as
            vec_t;
    *left_vol = (master_vol as libc::c_float * scale) as libc::c_int;
    if *left_vol < 0i32 { *left_vol = 0i32 };
}
static mut listener_axis: [vec3_t; 3] = [[0.; 3]; 3];
static mut listener_origin: vec3_t = [0.; 3];
#[no_mangle]
pub static mut s_rawsamples: [[portable_samplepair_t; 16384]; 129] =
    [[portable_samplepair_t{left: 0, right: 0,}; 16384]; 129];
static mut s_soundMuted: qboolean = qfalse;
//static char		s_backgroundMusic[MAX_QPATH]; //TTimo: unused
// =======================================================================
// Internal sound data & structures
// =======================================================================
// only begin attenuating sound volumes when outside the FULLVOLUME range
static mut s_soundStarted: libc::c_int = 0;
//====================================================================
#[no_mangle]
pub static mut s_channels: [channel_t; 96] =
    [channel_t{allocTime: 0,
               startSample: 0,
               entnum: 0,
               entchannel: 0,
               leftvol: 0,
               rightvol: 0,
               master_vol: 0,
               dopplerScale: 0.,
               oldDopplerScale: 0.,
               origin: [0.; 3],
               fixed_origin: qfalse,
               thesfx: 0 as *const sfx_t as *mut sfx_t,
               doppler: qfalse,
               fullVolume: qfalse,}; 96];
#[no_mangle]
pub static mut loop_channels: [channel_t; 96] =
    [channel_t{allocTime: 0,
               startSample: 0,
               entnum: 0,
               entchannel: 0,
               leftvol: 0,
               rightvol: 0,
               master_vol: 0,
               dopplerScale: 0.,
               oldDopplerScale: 0.,
               origin: [0.; 3],
               fixed_origin: qfalse,
               thesfx: 0 as *const sfx_t as *mut sfx_t,
               doppler: qfalse,
               fullVolume: qfalse,}; 96];
#[no_mangle]
pub static mut numLoopChannels: libc::c_int = 0;
#[no_mangle]
pub static mut s_paintedtime: libc::c_int = 0;
#[no_mangle]
pub static mut s_testsound: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn S_memoryLoad(mut sfx: *mut sfx_t) {
    if 0 == S_LoadSound(sfx) as u64 { (*sfx).defaultSound = qtrue }
    (*sfx).inMemory = qtrue;
}
// wavelet function
#[no_mangle]
pub unsafe extern "C" fn S_FreeOldestSound() {
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut buffer: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut nbuffer: *mut sndBuffer = 0 as *mut sndBuffer;
    oldest = Com_Milliseconds();
    used = 0i32;
    i = 1i32;
    while i < s_numSfx {
        sfx = &mut *s_knownSfx.as_mut_ptr().offset(i as isize) as *mut sfx_t;
        if 0 != (*sfx).inMemory as libc::c_uint &&
               (*sfx).lastTimeUsed < oldest {
            used = i;
            oldest = (*sfx).lastTimeUsed
        }
        i += 1
    }
    sfx = &mut *s_knownSfx.as_mut_ptr().offset(used as isize) as *mut sfx_t;
    Com_DPrintf(b"S_FreeOldestSound: freeing sound %s\n\x00" as *const u8 as
                    *const libc::c_char, (*sfx).soundName.as_mut_ptr());
    buffer = (*sfx).soundData;
    while !buffer.is_null() {
        nbuffer = (*buffer).next;
        SND_free(buffer);
        buffer = nbuffer
    }
    (*sfx).inMemory = qfalse;
    (*sfx).soundData = 0 as *mut sndBuffer;
}
// sample PAIRS
// MAX_SFX may be larger than MAX_SOUNDS because
// of custom player sounds
#[no_mangle]
pub static mut s_knownSfx: [sfx_t; 4096] =
    [sfx_s{soundData: 0 as *const sndBuffer as *mut sndBuffer,
           defaultSound: qfalse,
           inMemory: qfalse,
           soundCompressed: qfalse,
           soundCompressionMethod: 0,
           soundLength: 0,
           soundChannels: 0,
           soundName: [0; 64],
           lastTimeUsed: 0,
           next: 0 as *const sfx_s as *mut sfx_s,}; 4096];
#[no_mangle]
pub static mut s_numSfx: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn S_Base_Init(mut si: *mut soundInterface_t)
 -> qboolean {
    let mut r: qboolean = qfalse;
    if si.is_null() { return qfalse }
    s_mixahead =
        Cvar_Get(b"s_mixahead\x00" as *const u8 as *const libc::c_char,
                 b"0.2\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s_mixPreStep =
        Cvar_Get(b"s_mixPreStep\x00" as *const u8 as *const libc::c_char,
                 b"0.05\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s_show =
        Cvar_Get(b"s_show\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    s_testsound =
        Cvar_Get(b"s_testsound\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    r = SNDDMA_Init();
    if 0 != r as u64 {
        s_soundStarted = 1i32;
        s_soundMuted = qtrue;
        memset(sfxHash.as_mut_ptr() as *mut libc::c_void, 0i32,
               (::std::mem::size_of::<*mut sfx_t>() as
                    libc::c_ulong).wrapping_mul(128i32 as libc::c_ulong));
        s_soundtime = 0i32;
        s_paintedtime = 0i32;
        S_Base_StopAllSounds();
    } else { return qfalse }
    (*si).Shutdown = Some(S_Base_Shutdown);
    (*si).StartSound = Some(S_Base_StartSound);
    (*si).StartLocalSound = Some(S_Base_StartLocalSound);
    (*si).StartBackgroundTrack = Some(S_Base_StartBackgroundTrack);
    (*si).StopBackgroundTrack = Some(S_Base_StopBackgroundTrack);
    (*si).RawSamples = Some(S_Base_RawSamples);
    (*si).StopAllSounds = Some(S_Base_StopAllSounds);
    (*si).ClearLoopingSounds = Some(S_Base_ClearLoopingSounds);
    (*si).AddLoopingSound = Some(S_Base_AddLoopingSound);
    (*si).AddRealLoopingSound = Some(S_Base_AddRealLoopingSound);
    (*si).StopLoopingSound = Some(S_Base_StopLoopingSound);
    (*si).Respatialize = Some(S_Base_Respatialize);
    (*si).UpdateEntityPosition = Some(S_Base_UpdateEntityPosition);
    (*si).Update = Some(S_Base_Update);
    (*si).DisableSounds = Some(S_Base_DisableSounds);
    (*si).BeginRegistration = Some(S_Base_BeginRegistration);
    (*si).RegisterSound = Some(S_Base_RegisterSound);
    (*si).ClearSoundBuffer = Some(S_Base_ClearSoundBuffer);
    (*si).SoundInfo = Some(S_Base_SoundInfo);
    (*si).SoundList = Some(S_Base_SoundList);
    (*si).StartCapture = Some(S_Base_StartCapture);
    (*si).AvailableCaptureSamples = Some(S_Base_AvailableCaptureSamples);
    (*si).Capture = Some(S_Base_Capture);
    (*si).StopCapture = Some(S_Base_StopCapture);
    (*si).MasterGain = Some(S_Base_MasterGain);
    return qtrue;
}
unsafe extern "C" fn S_Base_MasterGain(mut val: libc::c_float) {
    SNDDMA_MasterGain(val);
}
unsafe extern "C" fn S_Base_StopCapture() { SNDDMA_StopCapture(); }
unsafe extern "C" fn S_Base_Capture(mut samples: libc::c_int,
                                    mut data: *mut byte) {
    SNDDMA_Capture(samples, data);
}
unsafe extern "C" fn S_Base_AvailableCaptureSamples() -> libc::c_int {
    return SNDDMA_AvailableCaptureSamples();
}
unsafe extern "C" fn S_Base_StartCapture() { SNDDMA_StartCapture(); }
/*
=================
S_Base_SoundList
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_SoundList() {
    let mut i: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut size: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut type_0: [[libc::c_char; 16]; 4] = [[0; 16]; 4];
    let mut mem: [[libc::c_char; 16]; 2] = [[0; 16]; 2];
    strcpy(type_0[0usize].as_mut_ptr(),
           b"16bit\x00" as *const u8 as *const libc::c_char);
    strcpy(type_0[1usize].as_mut_ptr(),
           b"adpcm\x00" as *const u8 as *const libc::c_char);
    strcpy(type_0[2usize].as_mut_ptr(),
           b"daub4\x00" as *const u8 as *const libc::c_char);
    strcpy(type_0[3usize].as_mut_ptr(),
           b"mulaw\x00" as *const u8 as *const libc::c_char);
    strcpy(mem[0usize].as_mut_ptr(),
           b"paged out\x00" as *const u8 as *const libc::c_char);
    strcpy(mem[1usize].as_mut_ptr(),
           b"resident \x00" as *const u8 as *const libc::c_char);
    total = 0i32;
    sfx = s_knownSfx.as_mut_ptr();
    i = 0i32;
    while i < s_numSfx {
        size = (*sfx).soundLength;
        total += size;
        Com_Printf(b"%6i[%s] : %s[%s]\n\x00" as *const u8 as
                       *const libc::c_char, size,
                   type_0[(*sfx).soundCompressionMethod as
                              usize].as_mut_ptr(),
                   (*sfx).soundName.as_mut_ptr(),
                   mem[(*sfx).inMemory as usize].as_mut_ptr());
        i += 1;
        sfx = sfx.offset(1isize)
    }
    Com_Printf(b"Total resident: %i\n\x00" as *const u8 as
                   *const libc::c_char, total);
    S_DisplayFreeMemory();
}
// ====================================================================
// User-setable variables
// ====================================================================
#[no_mangle]
pub unsafe extern "C" fn S_Base_SoundInfo() {
    Com_Printf(b"----- Sound Info -----\n\x00" as *const u8 as
                   *const libc::c_char);
    if 0 == s_soundStarted {
        Com_Printf(b"sound system not started\n\x00" as *const u8 as
                       *const libc::c_char);
    } else {
        Com_Printf(b"%5d channels\n\x00" as *const u8 as *const libc::c_char,
                   dma.channels);
        Com_Printf(b"%5d samples\n\x00" as *const u8 as *const libc::c_char,
                   dma.samples);
        Com_Printf(b"%5d samplebits (%s)\n\x00" as *const u8 as
                       *const libc::c_char, dma.samplebits,
                   if 0 != dma.isfloat {
                       b"float\x00" as *const u8 as *const libc::c_char
                   } else { b"int\x00" as *const u8 as *const libc::c_char });
        Com_Printf(b"%5d submission_chunk\n\x00" as *const u8 as
                       *const libc::c_char, dma.submission_chunk);
        Com_Printf(b"%5d speed\n\x00" as *const u8 as *const libc::c_char,
                   dma.speed);
        Com_Printf(b"%p dma buffer\n\x00" as *const u8 as *const libc::c_char,
                   dma.buffer);
        if !s_backgroundStream.is_null() {
            Com_Printf(b"Background file: %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       s_backgroundLoop.as_mut_ptr());
        } else {
            Com_Printf(b"No background file.\n\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    Com_Printf(b"----------------------\n\x00" as *const u8 as
                   *const libc::c_char);
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
    let mut clear: libc::c_int = 0;
    if 0 == s_soundStarted { return }
    memset(loopSounds.as_mut_ptr() as *mut libc::c_void, 0i32,
           ((1i32 << 10i32) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<loopSound_t>()
                                                as libc::c_ulong));
    memset(loop_channels.as_mut_ptr() as *mut libc::c_void, 0i32,
           (96i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<channel_t>()
                                                as libc::c_ulong));
    numLoopChannels = 0i32;
    S_ChannelSetup();
    memset(s_rawend.as_mut_ptr() as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<[libc::c_int; 129]>() as libc::c_ulong);
    if dma.samplebits == 8i32 { clear = 0x80i32 } else { clear = 0i32 }
    SNDDMA_BeginPainting();
    if !dma.buffer.is_null() {
        memset(dma.buffer as *mut libc::c_void, clear,
               (dma.samples * dma.samplebits / 8i32) as libc::c_ulong);
    }
    SNDDMA_Submit();
}
#[no_mangle]
pub unsafe extern "C" fn S_ChannelSetup() {
    let mut p: *mut channel_t = 0 as *mut channel_t;
    let mut q: *mut channel_t = 0 as *mut channel_t;
    memset(s_channels.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[channel_t; 96]>() as libc::c_ulong);
    p = s_channels.as_mut_ptr();
    q = p.offset(96isize);
    loop  {
        q = q.offset(-1isize);
        if !(q > p) { break ; }
        let ref mut fresh0 = *(q as *mut *mut channel_t);
        *fresh0 = q.offset(-1isize)
    }
    let ref mut fresh1 = *(q as *mut *mut channel_t);
    *fresh1 = 0 as *mut channel_t;
    freelist = p.offset(96isize).offset(-1isize);
    Com_DPrintf(b"Channel memory manager started\n\x00" as *const u8 as
                    *const libc::c_char);
}
static mut freelist: *mut channel_t = 0 as *const channel_t as *mut channel_t;
/*
==================
S_RegisterSound

Creates a default buzz sound if the file can't be loaded
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_RegisterSound(mut name: *const libc::c_char,
                                              mut compressed: qboolean)
 -> sfxHandle_t {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    compressed = qfalse;
    if 0 == s_soundStarted { return 0i32 }
    sfx = S_FindName(name);
    if sfx.is_null() { return 0i32 }
    if !(*sfx).soundData.is_null() {
        if 0 != (*sfx).defaultSound as u64 {
            Com_Printf(b"^3WARNING: could not find %s - using default\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*sfx).soundName.as_mut_ptr());
            return 0i32
        }
        return sfx.wrapping_offset_from(s_knownSfx.as_mut_ptr()) as
                   libc::c_long as sfxHandle_t
    }
    (*sfx).inMemory = qfalse;
    (*sfx).soundCompressed = compressed;
    S_memoryLoad(sfx);
    if 0 != (*sfx).defaultSound as u64 {
        Com_Printf(b"^3WARNING: could not find %s - using default\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*sfx).soundName.as_mut_ptr());
        return 0i32
    }
    return sfx.wrapping_offset_from(s_knownSfx.as_mut_ptr()) as libc::c_long
               as sfxHandle_t;
}
/*
==================
S_FindName

Will allocate a new sfx if it isn't found
==================
*/
unsafe extern "C" fn S_FindName(mut name: *const libc::c_char) -> *mut sfx_t {
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if name.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Sound name is NULL\x00" as *const u8 as
                      *const libc::c_char);
    }
    if 0 == *name.offset(0isize) {
        Com_Printf(b"^3WARNING: Sound name is empty\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as *mut sfx_t
    }
    if strlen(name) >= 64i32 as libc::c_ulong {
        Com_Printf(b"^3WARNING: Sound name is too long: %s\n\x00" as *const u8
                       as *const libc::c_char, name);
        return 0 as *mut sfx_t
    }
    if *name.offset(0isize) as libc::c_int == '*' as i32 {
        Com_Printf(b"^3WARNING: Tried to load player sound directly: %s\n\x00"
                       as *const u8 as *const libc::c_char, name);
        return 0 as *mut sfx_t
    }
    hash = S_HashSFXName(name) as libc::c_int;
    sfx = sfxHash[hash as usize];
    while !sfx.is_null() {
        if 0 == Q_stricmp((*sfx).soundName.as_mut_ptr(), name) { return sfx }
        sfx = (*sfx).next
    }
    i = 0i32;
    while i < s_numSfx {
        if 0 == s_knownSfx[i as usize].soundName[0usize] { break ; }
        i += 1
    }
    if i == s_numSfx {
        if s_numSfx == 4096i32 {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"S_FindName: out of sfx_t\x00" as *const u8 as
                          *const libc::c_char);
        }
        s_numSfx += 1
    }
    sfx = &mut *s_knownSfx.as_mut_ptr().offset(i as isize) as *mut sfx_t;
    memset(sfx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sfx_t>() as libc::c_ulong);
    strcpy((*sfx).soundName.as_mut_ptr(), name);
    (*sfx).next = sfxHash[hash as usize];
    sfxHash[hash as usize] = sfx;
    return sfx;
}
static mut sfxHash: [*mut sfx_t; 128] =
    [0 as *const sfx_t as *mut sfx_t; 128];
// =======================================================================
// Load a sound
// =======================================================================
/*
================
return a hash value for the sfx name
================
*/
unsafe extern "C" fn S_HashSFXName(mut name: *const libc::c_char)
 -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_long = 0;
    let mut letter: libc::c_char = 0;
    hash = 0i32 as libc::c_long;
    i = 0i32;
    while *name.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        letter =
            tolower(*name.offset(i as isize) as libc::c_int) as libc::c_char;
        if letter as libc::c_int == '.' as i32 {
            // don't include extension
            break ;
        } else {
            if letter as libc::c_int == '\\' as i32 {
                letter = '/' as i32 as libc::c_char
            }
            hash += letter as libc::c_long * (i + 119i32) as libc::c_long;
            i += 1
        }
    }
    hash &= (128i32 - 1i32) as libc::c_long;
    return hash;
}
/*
=====================
S_BeginRegistration

=====================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_BeginRegistration() {
    s_soundMuted = qfalse;
    if s_numSfx == 0i32 {
        SND_setup();
        memset(s_knownSfx.as_mut_ptr() as *mut libc::c_void, '\u{0}' as i32,
               ::std::mem::size_of::<[sfx_t; 4096]>() as libc::c_ulong);
        memset(sfxHash.as_mut_ptr() as *mut libc::c_void, '\u{0}' as i32,
               (::std::mem::size_of::<*mut sfx_t>() as
                    libc::c_ulong).wrapping_mul(128i32 as libc::c_ulong));
        S_Base_RegisterSound(b"sound/feedback/hit.wav\x00" as *const u8 as
                                 *const libc::c_char, qfalse);
    };
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
    s_soundMuted = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn S_Base_StopAllSounds() {
    if 0 == s_soundStarted { return }
    S_Base_StopBackgroundTrack();
    S_Base_ClearSoundBuffer();
}
/*
============
S_Update

Called once each time through the main loop
============
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_Update() {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    if (*s_show).integer == 2i32 {
        total = 0i32;
        ch = s_channels.as_mut_ptr();
        i = 0i32;
        while i < 96i32 {
            if !(*ch).thesfx.is_null() &&
                   (0 != (*ch).leftvol || 0 != (*ch).rightvol) {
                Com_Printf(b"%d %d %s\n\x00" as *const u8 as
                               *const libc::c_char, (*ch).leftvol,
                           (*ch).rightvol,
                           (*(*ch).thesfx).soundName.as_mut_ptr());
                total += 1
            }
            i += 1;
            ch = ch.offset(1isize)
        }
        Com_Printf(b"----(%i)---- painted: %i\n\x00" as *const u8 as
                       *const libc::c_char, total, s_paintedtime);
    }
    S_UpdateBackgroundTrack();
    S_Update_();
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
    let mut endtime: libc::c_uint = 0;
    static mut lastTime: libc::c_float = 0.0f32;
    let mut ma: libc::c_float = 0.;
    let mut op: libc::c_float = 0.;
    let mut thisTime: libc::c_float = 0.;
    let mut sane: libc::c_float = 0.;
    static mut ot: libc::c_int = -1i32;
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    thisTime = Com_Milliseconds() as libc::c_float;
    S_GetSoundtime();
    if s_soundtime == ot { return }
    ot = s_soundtime;
    S_ScanChannelStarts();
    sane = thisTime - lastTime;
    if sane < 11i32 as libc::c_float { sane = 11i32 as libc::c_float }
    ma = (*s_mixahead).value * dma.speed as libc::c_float;
    op =
        ((*s_mixPreStep).value as libc::c_double +
             (sane * dma.speed as libc::c_float) as libc::c_double * 0.01f64)
            as libc::c_float;
    if op < ma { ma = op }
    endtime = (s_soundtime as libc::c_float + ma) as libc::c_uint;
    endtime =
        endtime.wrapping_add(dma.submission_chunk as
                                 libc::c_uint).wrapping_sub(1i32 as
                                                                libc::c_uint)
            & !(dma.submission_chunk - 1i32) as libc::c_uint;
    if endtime.wrapping_sub(s_soundtime as libc::c_uint) >
           dma.fullsamples as libc::c_uint {
        endtime = (s_soundtime + dma.fullsamples) as libc::c_uint
    }
    SNDDMA_BeginPainting();
    S_PaintChannels(endtime as libc::c_int);
    SNDDMA_Submit();
    lastTime = thisTime;
}
#[no_mangle]
pub static mut s_mixPreStep: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_mixahead: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
========================
S_ScanChannelStarts

Returns qtrue if any new sounds were started since the last mix
========================
*/
#[no_mangle]
pub unsafe extern "C" fn S_ScanChannelStarts() -> qboolean {
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut i: libc::c_int = 0;
    let mut newSamples: qboolean = qfalse;
    newSamples = qfalse;
    ch = s_channels.as_mut_ptr();
    i = 0i32;
    while i < 96i32 {
        if !(*ch).thesfx.is_null() {
            // if this channel was just started this frame,
		// set the sample count to it begins mixing
		// into the very first sample
            if (*ch).startSample == 0x7fffffffi32 {
                (*ch).startSample = s_paintedtime;
                newSamples = qtrue
            } else if (*ch).startSample + (*(*ch).thesfx).soundLength <=
                          s_paintedtime {
                S_ChannelFree(ch);
            }
        }
        i += 1;
        ch = ch.offset(1isize)
    }
    return newSamples;
}
#[no_mangle]
pub unsafe extern "C" fn S_ChannelFree(mut v: *mut channel_t) {
    (*v).thesfx = 0 as *mut sfx_t;
    let ref mut fresh2 = *(v as *mut *mut channel_t);
    *fresh2 = freelist;
    freelist = v;
}
#[no_mangle]
pub unsafe extern "C" fn S_GetSoundtime() {
    let mut samplepos: libc::c_int = 0;
    static mut buffers: libc::c_int = 0;
    static mut oldsamplepos: libc::c_int = 0;
    if 0 != CL_VideoRecording() as u64 {
        let mut fps: libc::c_float =
            if (*cl_aviFrameRate).value < 1000.0f32 {
                (*cl_aviFrameRate).value
            } else { 1000.0f32 };
        let mut frameDuration: libc::c_float =
            if dma.speed as libc::c_float / fps > 1.0f32 {
                dma.speed as libc::c_float / fps
            } else { 1.0f32 } + clc.aviSoundFrameRemainder;
        let mut msec: libc::c_int = frameDuration as libc::c_int;
        s_soundtime += msec;
        clc.aviSoundFrameRemainder = frameDuration - msec as libc::c_float;
        return
    }
    samplepos = SNDDMA_GetDMAPos();
    if samplepos < oldsamplepos {
        buffers += 1;
        if s_paintedtime > 0x40000000i32 {
            buffers = 0i32;
            s_paintedtime = dma.fullsamples;
            S_Base_StopAllSounds();
        }
    }
    oldsamplepos = samplepos;
    s_soundtime = buffers * dma.fullsamples + samplepos / dma.channels;
    if dma.submission_chunk < 256i32 {
        s_paintedtime =
            (s_soundtime as libc::c_float +
                 (*s_mixPreStep).value * dma.speed as libc::c_float) as
                libc::c_int
    } else { s_paintedtime = s_soundtime + dma.submission_chunk };
}
#[no_mangle]
pub static mut s_show: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
//=============================================================================
/*
=====================
S_UpdateEntityPosition

let the sound system know where an entity currently is
======================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_UpdateEntityPosition(mut entityNum:
                                                         libc::c_int,
                                                     mut origin:
                                                         *const vec_t) {
    if entityNum < 0i32 || entityNum >= 1i32 << 10i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"S_UpdateEntityPosition: bad entitynum %i\x00" as *const u8
                      as *const libc::c_char, entityNum);
    }
    loopSounds[entityNum as usize].origin[0usize] = *origin.offset(0isize);
    loopSounds[entityNum as usize].origin[1usize] = *origin.offset(1isize);
    loopSounds[entityNum as usize].origin[2usize] = *origin.offset(2isize);
}
/*
============
S_Respatialize

Change the volumes of all the playing sounds for changes in their positions
============
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_Respatialize(mut entityNum: libc::c_int,
                                             mut head: *const vec_t,
                                             mut axis: *mut vec3_t,
                                             mut inwater: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut origin: vec3_t = [0.; 3];
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    listener_number = entityNum;
    listener_origin[0usize] = *head.offset(0isize);
    listener_origin[1usize] = *head.offset(1isize);
    listener_origin[2usize] = *head.offset(2isize);
    listener_axis[0usize][0usize] = (*axis.offset(0isize))[0usize];
    listener_axis[0usize][1usize] = (*axis.offset(0isize))[1usize];
    listener_axis[0usize][2usize] = (*axis.offset(0isize))[2usize];
    listener_axis[1usize][0usize] = (*axis.offset(1isize))[0usize];
    listener_axis[1usize][1usize] = (*axis.offset(1isize))[1usize];
    listener_axis[1usize][2usize] = (*axis.offset(1isize))[2usize];
    listener_axis[2usize][0usize] = (*axis.offset(2isize))[0usize];
    listener_axis[2usize][1usize] = (*axis.offset(2isize))[1usize];
    listener_axis[2usize][2usize] = (*axis.offset(2isize))[2usize];
    ch = s_channels.as_mut_ptr();
    i = 0i32;
    while i < 96i32 {
        if !(*ch).thesfx.is_null() {
            if 0 != (*ch).fullVolume as u64 {
                (*ch).leftvol = (*ch).master_vol;
                (*ch).rightvol = (*ch).master_vol
            } else {
                if 0 != (*ch).fixed_origin as u64 {
                    origin[0usize] = (*ch).origin[0usize];
                    origin[1usize] = (*ch).origin[1usize];
                    origin[2usize] = (*ch).origin[2usize]
                } else {
                    origin[0usize] =
                        loopSounds[(*ch).entnum as usize].origin[0usize];
                    origin[1usize] =
                        loopSounds[(*ch).entnum as usize].origin[1usize];
                    origin[2usize] =
                        loopSounds[(*ch).entnum as usize].origin[2usize]
                }
                S_SpatializeOrigin(origin.as_mut_ptr(), (*ch).master_vol,
                                   &mut (*ch).leftvol, &mut (*ch).rightvol);
            }
        }
        i += 1;
        ch = ch.offset(1isize)
    }
    S_AddLoopSounds();
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut left_total: libc::c_int = 0;
    let mut right_total: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut loop_0: *mut loopSound_t = 0 as *mut loopSound_t;
    let mut loop2: *mut loopSound_t = 0 as *mut loopSound_t;
    static mut loopFrame: libc::c_int = 0;
    numLoopChannels = 0i32;
    time = Com_Milliseconds();
    loopFrame += 1;
    i = 0i32;
    while i < 1i32 << 10i32 {
        loop_0 =
            &mut *loopSounds.as_mut_ptr().offset(i as isize) as
                *mut loopSound_t;
        if !(0 == (*loop_0).active as u64 ||
                 (*loop_0).mergeFrame == loopFrame) {
            // already merged into an earlier sound
            if 0 != (*loop_0).kill as u64 {
                S_SpatializeOrigin((*loop_0).origin.as_mut_ptr(), 127i32,
                                   &mut left_total, &mut right_total);
            } else {
                S_SpatializeOrigin((*loop_0).origin.as_mut_ptr(), 90i32,
                                   &mut left_total, &mut right_total);
            }
            (*(*loop_0).sfx).lastTimeUsed = time;
            j = i + 1i32;
            while j < 1i32 << 10i32 {
                loop2 =
                    &mut *loopSounds.as_mut_ptr().offset(j as isize) as
                        *mut loopSound_t;
                if !(0 == (*loop2).active as u64 ||
                         0 != (*loop2).doppler as libc::c_uint ||
                         (*loop2).sfx != (*loop_0).sfx) {
                    (*loop2).mergeFrame = loopFrame;
                    if 0 != (*loop2).kill as u64 {
                        S_SpatializeOrigin((*loop2).origin.as_mut_ptr(),
                                           127i32, &mut left, &mut right);
                    } else {
                        S_SpatializeOrigin((*loop2).origin.as_mut_ptr(),
                                           90i32, &mut left, &mut right);
                    }
                    (*(*loop2).sfx).lastTimeUsed = time;
                    left_total += left;
                    right_total += right
                }
                j += 1
            }
            if !(left_total == 0i32 && right_total == 0i32) {
                // not audible
                ch =
                    &mut *loop_channels.as_mut_ptr().offset(numLoopChannels as
                                                                isize) as
                        *mut channel_t;
                if left_total > 255i32 { left_total = 255i32 }
                if right_total > 255i32 { right_total = 255i32 }
                (*ch).master_vol = 127i32;
                (*ch).leftvol = left_total;
                (*ch).rightvol = right_total;
                (*ch).thesfx = (*loop_0).sfx;
                (*ch).doppler = (*loop_0).doppler;
                (*ch).dopplerScale = (*loop_0).dopplerScale;
                (*ch).oldDopplerScale = (*loop_0).oldDopplerScale;
                (*ch).fullVolume = qfalse;
                numLoopChannels += 1;
                if numLoopChannels == 96i32 { return }
            }
        }
        i += 1
    };
}
static mut listener_number: libc::c_int = 0;
/*
==============================================================

continuous looping sounds are added each frame

==============================================================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_StopLoopingSound(mut entityNum: libc::c_int) {
    loopSounds[entityNum as usize].active = qfalse;
    loopSounds[entityNum as usize].kill = qfalse;
}
/*
==================
S_AddLoopingSound

Called during entity generation for a frame
Include velocity in case I get around to doing doppler...
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_AddRealLoopingSound(mut entityNum:
                                                        libc::c_int,
                                                    mut origin: *const vec_t,
                                                    mut velocity:
                                                        *const vec_t,
                                                    mut sfxHandle:
                                                        sfxHandle_t) {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    if sfxHandle < 0i32 || sfxHandle >= s_numSfx {
        Com_Printf(b"^3S_AddRealLoopingSound: handle %i out of range\n\x00" as
                       *const u8 as *const libc::c_char, sfxHandle);
        return
    }
    sfx =
        &mut *s_knownSfx.as_mut_ptr().offset(sfxHandle as isize) as
            *mut sfx_t;
    if (*sfx).inMemory as libc::c_uint ==
           qfalse as libc::c_int as libc::c_uint {
        S_memoryLoad(sfx);
    }
    if 0 == (*sfx).soundLength {
        Com_Error(ERR_DROP as libc::c_int,
                  b"%s has length 0\x00" as *const u8 as *const libc::c_char,
                  (*sfx).soundName.as_mut_ptr());
    }
    loopSounds[entityNum as usize].origin[0usize] = *origin.offset(0isize);
    loopSounds[entityNum as usize].origin[1usize] = *origin.offset(1isize);
    loopSounds[entityNum as usize].origin[2usize] = *origin.offset(2isize);
    loopSounds[entityNum as usize].velocity[0usize] =
        *velocity.offset(0isize);
    loopSounds[entityNum as usize].velocity[1usize] =
        *velocity.offset(1isize);
    loopSounds[entityNum as usize].velocity[2usize] =
        *velocity.offset(2isize);
    loopSounds[entityNum as usize].sfx = sfx;
    loopSounds[entityNum as usize].active = qtrue;
    loopSounds[entityNum as usize].kill = qfalse;
    loopSounds[entityNum as usize].doppler = qfalse;
}
/*
==================
S_AddLoopingSound

Called during entity generation for a frame
Include velocity in case I get around to doing doppler...
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_AddLoopingSound(mut entityNum: libc::c_int,
                                                mut origin: *const vec_t,
                                                mut velocity: *const vec_t,
                                                mut sfxHandle: sfxHandle_t) {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    if sfxHandle < 0i32 || sfxHandle >= s_numSfx {
        Com_Printf(b"^3S_AddLoopingSound: handle %i out of range\n\x00" as
                       *const u8 as *const libc::c_char, sfxHandle);
        return
    }
    sfx =
        &mut *s_knownSfx.as_mut_ptr().offset(sfxHandle as isize) as
            *mut sfx_t;
    if (*sfx).inMemory as libc::c_uint ==
           qfalse as libc::c_int as libc::c_uint {
        S_memoryLoad(sfx);
    }
    if 0 == (*sfx).soundLength {
        Com_Error(ERR_DROP as libc::c_int,
                  b"%s has length 0\x00" as *const u8 as *const libc::c_char,
                  (*sfx).soundName.as_mut_ptr());
    }
    loopSounds[entityNum as usize].origin[0usize] = *origin.offset(0isize);
    loopSounds[entityNum as usize].origin[1usize] = *origin.offset(1isize);
    loopSounds[entityNum as usize].origin[2usize] = *origin.offset(2isize);
    loopSounds[entityNum as usize].velocity[0usize] =
        *velocity.offset(0isize);
    loopSounds[entityNum as usize].velocity[1usize] =
        *velocity.offset(1isize);
    loopSounds[entityNum as usize].velocity[2usize] =
        *velocity.offset(2isize);
    loopSounds[entityNum as usize].active = qtrue;
    loopSounds[entityNum as usize].kill = qtrue;
    loopSounds[entityNum as usize].doppler = qfalse;
    loopSounds[entityNum as usize].oldDopplerScale = 1.0f64 as libc::c_float;
    loopSounds[entityNum as usize].dopplerScale = 1.0f64 as libc::c_float;
    loopSounds[entityNum as usize].sfx = sfx;
    if 0 != (*s_doppler).integer &&
           VectorLengthSquared(velocity) as libc::c_double > 0.0f64 {
        let mut out: vec3_t = [0.; 3];
        let mut lena: libc::c_float = 0.;
        let mut lenb: libc::c_float = 0.;
        loopSounds[entityNum as usize].doppler = qtrue;
        lena =
            DistanceSquared(loopSounds[listener_number as
                                           usize].origin.as_mut_ptr() as
                                *const vec_t,
                            loopSounds[entityNum as usize].origin.as_mut_ptr()
                                as *const vec_t);
        out[0usize] =
            loopSounds[entityNum as usize].origin[0usize] +
                loopSounds[entityNum as usize].velocity[0usize];
        out[1usize] =
            loopSounds[entityNum as usize].origin[1usize] +
                loopSounds[entityNum as usize].velocity[1usize];
        out[2usize] =
            loopSounds[entityNum as usize].origin[2usize] +
                loopSounds[entityNum as usize].velocity[2usize];
        lenb =
            DistanceSquared(loopSounds[listener_number as
                                           usize].origin.as_mut_ptr() as
                                *const vec_t,
                            out.as_mut_ptr() as *const vec_t);
        if loopSounds[entityNum as usize].framenum + 1i32 != cls.framecount {
            loopSounds[entityNum as usize].oldDopplerScale =
                1.0f64 as libc::c_float
        } else {
            loopSounds[entityNum as usize].oldDopplerScale =
                loopSounds[entityNum as usize].dopplerScale
        }
        loopSounds[entityNum as usize].dopplerScale =
            lenb / (lena * 100i32 as libc::c_float);
        if loopSounds[entityNum as usize].dopplerScale as libc::c_double <=
               1.0f64 {
            loopSounds[entityNum as usize].doppler = qfalse
        } else if loopSounds[entityNum as usize].dopplerScale > 50.0f32 {
            loopSounds[entityNum as usize].dopplerScale = 50.0f32
        }
    }
    loopSounds[entityNum as usize].framenum = cls.framecount;
}
/*
==================
S_ClearLoopingSounds

==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_ClearLoopingSounds(mut killall: qboolean) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 1i32 << 10i32 {
        if 0 != killall as libc::c_uint ||
               loopSounds[i as usize].kill as libc::c_uint ==
                   qtrue as libc::c_int as libc::c_uint ||
               !loopSounds[i as usize].sfx.is_null() &&
                   (*loopSounds[i as usize].sfx).soundLength == 0i32 {
            S_Base_StopLoopingSound(i);
        }
        i += 1
    }
    numLoopChannels = 0i32;
}
/*
======================
S_StartBackgroundTrack
======================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_StartBackgroundTrack(mut intro:
                                                         *const libc::c_char,
                                                     mut loop_0:
                                                         *const libc::c_char) {
    if intro.is_null() { intro = b"\x00" as *const u8 as *const libc::c_char }
    if loop_0.is_null() || 0 == *loop_0.offset(0isize) { loop_0 = intro }
    Com_DPrintf(b"S_StartBackgroundTrack( %s, %s )\n\x00" as *const u8 as
                    *const libc::c_char, intro, loop_0);
    if 0 == *intro { S_Base_StopBackgroundTrack(); return }
    Q_strncpyz(s_backgroundLoop.as_mut_ptr(), loop_0,
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    S_OpenBackgroundStream(intro);
}
/*
==================
S_StartLocalSound
==================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_StartLocalSound(mut sfxHandle: sfxHandle_t,
                                                mut channelNum: libc::c_int) {
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    if sfxHandle < 0i32 || sfxHandle >= s_numSfx {
        Com_Printf(b"^3S_StartLocalSound: handle %i out of range\n\x00" as
                       *const u8 as *const libc::c_char, sfxHandle);
        return
    }
    S_Base_StartSoundEx(0 as *mut vec_t, listener_number, channelNum,
                        sfxHandle, qtrue);
}
/*
====================
S_Base_StartSoundEx

Validates the parms and ques the sound up
if origin is NULL, the sound will be dynamically sourced from the entity
Entchannel 0 will never override a playing sound
====================
*/
unsafe extern "C" fn S_Base_StartSoundEx(mut origin: *mut vec_t,
                                         mut entityNum: libc::c_int,
                                         mut entchannel: libc::c_int,
                                         mut sfxHandle: sfxHandle_t,
                                         mut localSound: qboolean) {
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0;
    let mut chosen: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut inplay: libc::c_int = 0;
    let mut allowed: libc::c_int = 0;
    let mut fullVolume: qboolean = qfalse;
    if 0 == s_soundStarted || 0 != s_soundMuted as libc::c_uint { return }
    if origin.is_null() && (entityNum < 0i32 || entityNum >= 1i32 << 10i32) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"S_StartSound: bad entitynum %i\x00" as *const u8 as
                      *const libc::c_char, entityNum);
    }
    if sfxHandle < 0i32 || sfxHandle >= s_numSfx {
        Com_Printf(b"^3S_StartSound: handle %i out of range\n\x00" as
                       *const u8 as *const libc::c_char, sfxHandle);
        return
    }
    sfx =
        &mut *s_knownSfx.as_mut_ptr().offset(sfxHandle as isize) as
            *mut sfx_t;
    if (*sfx).inMemory as libc::c_uint ==
           qfalse as libc::c_int as libc::c_uint {
        S_memoryLoad(sfx);
    }
    if (*s_show).integer == 1i32 {
        Com_Printf(b"%i : %s\n\x00" as *const u8 as *const libc::c_char,
                   s_paintedtime, (*sfx).soundName.as_mut_ptr());
    }
    time = Com_Milliseconds();
    allowed = 4i32;
    if entityNum == listener_number { allowed = 8i32 }
    fullVolume = qfalse;
    if 0 != localSound as libc::c_uint ||
           0 != S_Base_HearingThroughEntity(entityNum, origin) as libc::c_uint
       {
        fullVolume = qtrue
    }
    ch = s_channels.as_mut_ptr();
    inplay = 0i32;
    i = 0i32;
    while i < 96i32 {
        if (*ch).entnum == entityNum && (*ch).thesfx == sfx {
            if time - (*ch).allocTime < 50i32 { return }
            inplay += 1
        }
        i += 1;
        ch = ch.offset(1isize)
    }
    if inplay > allowed { return }
    (*sfx).lastTimeUsed = time;
    ch = S_ChannelMalloc();
    if ch.is_null() {
        ch = s_channels.as_mut_ptr();
        oldest = (*sfx).lastTimeUsed;
        chosen = -1i32;
        i = 0i32;
        while i < 96i32 {
            if (*ch).entnum != listener_number && (*ch).entnum == entityNum &&
                   (*ch).allocTime < oldest &&
                   (*ch).entchannel != CHAN_ANNOUNCER as libc::c_int {
                oldest = (*ch).allocTime;
                chosen = i
            }
            i += 1;
            ch = ch.offset(1isize)
        }
        if chosen == -1i32 {
            ch = s_channels.as_mut_ptr();
            i = 0i32;
            while i < 96i32 {
                if (*ch).entnum != listener_number && (*ch).allocTime < oldest
                       && (*ch).entchannel != CHAN_ANNOUNCER as libc::c_int {
                    oldest = (*ch).allocTime;
                    chosen = i
                }
                i += 1;
                ch = ch.offset(1isize)
            }
            if chosen == -1i32 {
                ch = s_channels.as_mut_ptr();
                if (*ch).entnum == listener_number {
                    i = 0i32;
                    while i < 96i32 {
                        if (*ch).allocTime < oldest {
                            oldest = (*ch).allocTime;
                            chosen = i
                        }
                        i += 1;
                        ch = ch.offset(1isize)
                    }
                }
                if chosen == -1i32 {
                    Com_Printf(b"dropping sound\n\x00" as *const u8 as
                                   *const libc::c_char);
                    return
                }
            }
        }
        ch =
            &mut *s_channels.as_mut_ptr().offset(chosen as isize) as
                *mut channel_t;
        (*ch).allocTime = (*sfx).lastTimeUsed
    }
    if !origin.is_null() {
        (*ch).origin[0usize] = *origin.offset(0isize);
        (*ch).origin[1usize] = *origin.offset(1isize);
        (*ch).origin[2usize] = *origin.offset(2isize);
        (*ch).fixed_origin = qtrue
    } else { (*ch).fixed_origin = qfalse }
    (*ch).master_vol = 127i32;
    (*ch).entnum = entityNum;
    (*ch).thesfx = sfx;
    (*ch).startSample = 0x7fffffffi32;
    (*ch).entchannel = entchannel;
    (*ch).leftvol = (*ch).master_vol;
    (*ch).rightvol = (*ch).master_vol;
    (*ch).doppler = qfalse;
    (*ch).fullVolume = fullVolume;
}
#[no_mangle]
pub unsafe extern "C" fn S_ChannelMalloc() -> *mut channel_t {
    let mut v: *mut channel_t = 0 as *mut channel_t;
    if freelist.is_null() { return 0 as *mut channel_t }
    v = freelist;
    freelist = *(freelist as *mut *mut channel_t);
    (*v).allocTime = Com_Milliseconds();
    return v;
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
unsafe extern "C" fn S_Base_HearingThroughEntity(mut entityNum: libc::c_int,
                                                 mut origin: *mut vec_t)
 -> qboolean {
    let mut distanceSq: libc::c_float = 0.;
    let mut sorigin: vec3_t = [0.; 3];
    if !origin.is_null() {
        sorigin[0usize] = *origin.offset(0isize);
        sorigin[1usize] = *origin.offset(1isize);
        sorigin[2usize] = *origin.offset(2isize)
    } else {
        sorigin[0usize] = loopSounds[entityNum as usize].origin[0usize];
        sorigin[1usize] = loopSounds[entityNum as usize].origin[1usize];
        sorigin[2usize] = loopSounds[entityNum as usize].origin[2usize]
    }
    if listener_number == entityNum {
        distanceSq =
            DistanceSquared(sorigin.as_mut_ptr() as *const vec_t,
                            listener_origin.as_mut_ptr() as *const vec_t);
        if distanceSq > 48.0f32 * 48.0f32 {
            return qfalse
        } else { return qtrue }
    } else { return qfalse };
}
/*
====================
S_StartSound

if origin is NULL, the sound will be dynamically sourced from the entity
====================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Base_StartSound(mut origin: *mut vec_t,
                                           mut entityNum: libc::c_int,
                                           mut entchannel: libc::c_int,
                                           mut sfxHandle: sfxHandle_t) {
    S_Base_StartSoundEx(origin, entityNum, entchannel, sfxHandle, qfalse);
}
// =======================================================================
// Shutdown sound engine
// =======================================================================
#[no_mangle]
pub unsafe extern "C" fn S_Base_Shutdown() {
    if 0 == s_soundStarted { return }
    SNDDMA_Shutdown();
    SND_shutdown();
    s_soundStarted = 0i32;
    s_numSfx = 0i32;
    Cmd_RemoveCommand(b"s_info\x00" as *const u8 as *const libc::c_char);
}
/*
=================
S_DefaultSound
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_DefaultSound(mut sfx: *mut sfx_t) {
    let mut i: libc::c_int = 0;
    (*sfx).soundLength = 512i32;
    (*sfx).soundData = SND_malloc();
    (*(*sfx).soundData).next = 0 as *mut sndBuffer_s;
    i = 0i32;
    while i < (*sfx).soundLength {
        (*(*sfx).soundData).sndChunk[i as usize] = i as libc::c_short;
        i += 1
    };
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
pub unsafe extern "C" fn S_ByteSwapRawSamples(mut samples: libc::c_int,
                                              mut width: libc::c_int,
                                              mut s_channels_0: libc::c_int,
                                              mut data: *const byte) {
    let mut i: libc::c_int = 0;
    if width != 2i32 { return }
    if 256i32 == 256i32 { return }
    if s_channels_0 == 2i32 { samples <<= 1i32 }
    i = 0i32;
    while i < samples {
        *(data as *mut libc::c_short).offset(i as isize) =
            *(data as *mut libc::c_short).offset(i as isize);
        i += 1
    };
}