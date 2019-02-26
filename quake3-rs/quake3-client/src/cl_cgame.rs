#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           label_break_value,
           libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int32_t = libc::c_int;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int32_t = __int32_t;
    use super::types_h::{__int32_t};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
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
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub type qhandle_t = libc::c_int;
    pub type sfxHandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    pub type clipHandle_t = libc::c_int;
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
    /*
==============================================================

VoIP

==============================================================
*/
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
// change this.
    /*
==============================================================

COLLISION DETECTION

==============================================================
*/
    // plane types are used to speed some tests
// 0-2 are axial planes
    /*
=================
PlaneTypeForNormal
=================
*/
    // plane_t structure
// !!! if this is changed, it must be changed in asm code too !!!
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct pc_token_s {
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub intvalue: libc::c_int,
        pub floatvalue: libc::c_float,
        pub string: [libc::c_char; 1024],
    }
    pub type pc_token_t = pc_token_s;
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
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
    pub type cvarHandle_t = libc::c_int;
    // the modules that run in the virtual machine can't access the cvar_t directly,
// so they must ask for structured updates
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    pub type cplane_t = cplane_s;
    // a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct trace_t {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub entityNum: libc::c_int,
    }
    // trace->entityNum can also be 0 to (MAX_GENTITIES-1)
// or ENTITYNUM_NONE, ENTITYNUM_WORLD
    // markfragments are returned by R_MarkFragments()
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct markFragment_t {
        pub firstPoint: libc::c_int,
        pub numPoints: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct orientation_t {
        pub origin: vec3_t,
        pub axis: [vec3_t; 3],
    }
    /*
========================================================================

  ELEMENTS COMMUNICATED ACROSS THE NET

========================================================================
*/
    // snapshot used during connection and for zombies
    // toggled every map_restart so transitions can be detected
    //
// per-level limits
//
    // absolute limit
    // don't need to send any more
    // entitynums are communicated with GENTITY_BITS, so any reserved
// values that are going to be communcated over the net need to
// also be in this range
    // these are sent over the net as 8 bits
    // so they cannot be blindly increased
    // these are the only configstrings that the system reserves, all the
// other ones are strictly for servergame to clientgame communication
    // an info string with all the serverinfo cvars
    // an info string for server system to client system configuration (timescale, etc)
    // game can't modify below this, only the system can
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct gameState_t {
        pub stringOffsets: [libc::c_int; 1024],
        pub stringData: [libc::c_char; 16000],
        pub dataCount: libc::c_int,
    }
    //=========================================================
    // bit field limits
    // playerState_t is the information needed by both the client and server
// to predict player motion and actions
// nothing outside of pmove should modify these, or some degree of prediction error
// will occur
    // you can't add anything to this without modifying the code in msg.c
    // playerState_t is a full superset of entityState_t as it is used by players,
// so if a playerState_t is transmitted, the entityState_t can be fully derived
// from it.
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct playerState_s {
        pub commandTime: libc::c_int,
        pub pm_type: libc::c_int,
        pub bobCycle: libc::c_int,
        pub pm_flags: libc::c_int,
        pub pm_time: libc::c_int,
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub weaponTime: libc::c_int,
        pub gravity: libc::c_int,
        pub speed: libc::c_int,
        pub delta_angles: [libc::c_int; 3],
        pub groundEntityNum: libc::c_int,
        pub legsTimer: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoTimer: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub movementDir: libc::c_int,
        pub grapplePoint: vec3_t,
        pub eFlags: libc::c_int,
        pub eventSequence: libc::c_int,
        pub events: [libc::c_int; 2],
        pub eventParms: [libc::c_int; 2],
        pub externalEvent: libc::c_int,
        pub externalEventParm: libc::c_int,
        pub externalEventTime: libc::c_int,
        pub clientNum: libc::c_int,
        pub weapon: libc::c_int,
        pub weaponstate: libc::c_int,
        pub viewangles: vec3_t,
        pub viewheight: libc::c_int,
        pub damageEvent: libc::c_int,
        pub damageYaw: libc::c_int,
        pub damagePitch: libc::c_int,
        pub damageCount: libc::c_int,
        pub stats: [libc::c_int; 16],
        pub persistant: [libc::c_int; 16],
        pub powerups: [libc::c_int; 16],
        pub ammo: [libc::c_int; 16],
        pub generic1: libc::c_int,
        pub loopSound: libc::c_int,
        pub jumppad_ent: libc::c_int,
        pub ping: libc::c_int,
        pub pmove_framecount: libc::c_int,
        pub jumppad_frame: libc::c_int,
        pub entityEventSequence: libc::c_int,
    }
    pub type playerState_t = playerState_s;
    //====================================================================
    //
// usercmd_t->button bits, many of which are generated by the client system,
// so they aren't game/cgame only definitions
//
    // displays talk balloon and disables actions
    // walking can't just be inferred from MOVE_RUN
    // because a key pressed late in the frame will
										// only generate a small move value for that frame
										// walking will use different animations and
										// won't generate footsteps
    // any key whatsoever
    // if forwardmove or rightmove are >= MOVE_RUN,
    // then BUTTON_WALKING should be set
    // usercmd_t is sent to the server each client frame
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct usercmd_s {
        pub serverTime: libc::c_int,
        pub angles: [libc::c_int; 3],
        pub buttons: libc::c_int,
        pub weapon: byte,
        pub forwardmove: libc::c_schar,
        pub rightmove: libc::c_schar,
        pub upmove: libc::c_schar,
    }
    pub type usercmd_t = usercmd_s;
    //===================================================================
    // if entityState->solid == SOLID_BMODEL, modelindex is an inline model number
    pub type trType_t = libc::c_uint;
    pub const TR_GRAVITY: trType_t = 5;
    // value = base + sin( time / duration ) * delta
    pub const TR_SINE: trType_t = 4;
    pub const TR_LINEAR_STOP: trType_t = 3;
    pub const TR_LINEAR: trType_t = 2;
    // non-parametric, but interpolate between snapshots
    pub const TR_INTERPOLATE: trType_t = 1;
    pub const TR_STATIONARY: trType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct trajectory_t {
        pub trType: trType_t,
        pub trTime: libc::c_int,
        pub trDuration: libc::c_int,
        pub trBase: vec3_t,
        pub trDelta: vec3_t,
    }
    // entityState_t is the information conveyed from the server
// in an update message about entities that the client will
// need to render in some way
// Different eTypes may use the information in different ways
// The messages are delta compressed, so it doesn't really matter if
// the structure size is fairly large
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct entityState_s {
        pub number: libc::c_int,
        pub eType: libc::c_int,
        pub eFlags: libc::c_int,
        pub pos: trajectory_t,
        pub apos: trajectory_t,
        pub time: libc::c_int,
        pub time2: libc::c_int,
        pub origin: vec3_t,
        pub origin2: vec3_t,
        pub angles: vec3_t,
        pub angles2: vec3_t,
        pub otherEntityNum: libc::c_int,
        pub otherEntityNum2: libc::c_int,
        pub groundEntityNum: libc::c_int,
        pub constantLight: libc::c_int,
        pub loopSound: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub clientNum: libc::c_int,
        pub frame: libc::c_int,
        pub solid: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub generic1: libc::c_int,
    }
    pub type entityState_t = entityState_s;
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
    // font support 
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glyphInfo_t {
        pub height: libc::c_int,
        pub top: libc::c_int,
        pub bottom: libc::c_int,
        pub pitch: libc::c_int,
        pub xSkip: libc::c_int,
        pub imageWidth: libc::c_int,
        pub imageHeight: libc::c_int,
        pub s: libc::c_float,
        pub t: libc::c_float,
        pub s2: libc::c_float,
        pub t2: libc::c_float,
        pub glyph: qhandle_t,
        pub shaderName: [libc::c_char; 32],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fontInfo_t {
        pub glyphs: [glyphInfo_t; 256],
        pub glyphScale: libc::c_float,
        pub name: [libc::c_char; 64],
    }
    // real time
//=============================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qtime_s {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
    }
    pub type qtime_t = qtime_s;
    // server browser sources
// TTimo: AS_MPLAYER is no longer used
    // cinematic states
    pub type e_status = libc::c_uint;
    pub const FMV_ID_WAIT: e_status = 6;
    pub const FMV_LOOPED: e_status = 5;
    pub const FMV_ID_IDLE: e_status = 4;
    pub const FMV_ID_BLT: e_status = 3;
    // all other conditions, i.e. stop/EOF/abort
    pub const FMV_EOF: e_status = 2;
    // play
    pub const FMV_PLAY: e_status = 1;
    pub const FMV_IDLE: e_status = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn qsnapvectorsse(vec: *mut vec_t);
        #[no_mangle]
        pub fn Q_acos(c: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        //=============================================
        //
// key / value info strings
//
        #[no_mangle]
        pub fn Info_ValueForKey(s: *const libc::c_char,
                                key: *const libc::c_char)
         -> *mut libc::c_char;
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
    pub type vm_t = vm_s;
    pub type vmInterpret_t = libc::c_uint;
    pub const VMI_COMPILED: vmInterpret_t = 2;
    pub const VMI_BYTECODE: vmInterpret_t = 1;
    pub const VMI_NATIVE: vmInterpret_t = 0;
    // Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
    //===========================================================================
    /*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
    pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, vmCvar_t, fileHandle_t, fsMode_t,
                            qtime_t, cvar_t};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn VM_Create(module: *const libc::c_char,
                         systemCalls:
                             Option<unsafe extern "C" fn(_: *mut intptr_t)
                                        -> intptr_t>,
                         interpret: vmInterpret_t) -> *mut vm_t;
        // module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
        #[no_mangle]
        pub fn VM_Free(vm: *mut vm_t);
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        #[no_mangle]
        pub fn VM_Debug(level: libc::c_int);
        #[no_mangle]
        pub fn VM_ArgPtr(intValue: intptr_t) -> *mut libc::c_void;
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        // don't allow VMs to remove system commands
        #[no_mangle]
        pub fn Cmd_RemoveCommandSafe(cmd_name: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgvBuffer(arg: libc::c_int, buffer: *mut libc::c_char,
                              bufferLength: libc::c_int);
        #[no_mangle]
        pub fn Cmd_ArgsFrom(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgsBuffer(buffer: *mut libc::c_char,
                              bufferLength: libc::c_int);
        // The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
        #[no_mangle]
        pub fn Cmd_TokenizeString(text: *const libc::c_char);
        // creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
        #[no_mangle]
        pub fn Cvar_Register(vmCvar: *mut vmCvar_t,
                             varName: *const libc::c_char,
                             defaultValue: *const libc::c_char,
                             flags: libc::c_int);
        // basically a slightly modified Cvar_Get for the interpreted modules
        #[no_mangle]
        pub fn Cvar_Update(vmCvar: *mut vmCvar_t);
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // same as Cvar_Set, but allows more control over setting of cvar
        #[no_mangle]
        pub fn Cvar_SetSafe(var_name: *const libc::c_char,
                            value: *const libc::c_char);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        #[no_mangle]
        pub fn Cvar_VariableStringBuffer(var_name: *const libc::c_char,
                                         buffer: *mut libc::c_char,
                                         bufsize: libc::c_int);
        #[no_mangle]
        pub fn Cvar_SetCheatState();
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        // like fprintf
        #[no_mangle]
        pub fn FS_FOpenFileByMode(qpath: *const libc::c_char,
                                  f: *mut fileHandle_t, mode: fsMode_t)
         -> libc::c_int;
        // opens a file for reading, writing, or appending depending on the value of mode
        #[no_mangle]
        pub fn FS_Seek(f: fileHandle_t, offset: libc::c_long,
                       origin: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Com_RealTime(qtime: *mut qtime_t) -> libc::c_int;
        #[no_mangle]
        pub static mut com_timescale: *mut cvar_t;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_paused: *mut cvar_t;
        #[no_mangle]
        pub fn Hunk_MemoryRemaining() -> libc::c_int;
        #[no_mangle]
        pub fn Com_TouchMemory();
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Sys_LowPhysicalMemory() -> qboolean;
    }
}
#[header_src =
      "ioq3/code/cgame/cg_public.h"]
pub mod cg_public_h {
    //	void (*CG_Shutdown)( void );
	// opportunity to flush and close any open files
    pub const CG_CONSOLE_COMMAND: unnamed_1 = 2;
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
    // allow a lot of command backups for very fast systems
// multiple commands may be combined into a single packet, so this
// needs to be larger than PACKET_BACKUP
    // snapshots are a view of the server at a given time
    // Snapshots are generated at regular time intervals by the server,
// but they may not be sent if a client's rate level is exceeded, or
// they may be dropped by the network.
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snapshot_t {
        pub snapFlags: libc::c_int,
        pub ping: libc::c_int,
        pub serverTime: libc::c_int,
        pub areamask: [byte; 32],
        pub ps: playerState_t,
        pub numEntities: libc::c_int,
        pub entities: [entityState_t; 256],
        pub numServerCommands: libc::c_int,
        pub serverCommandSequence: libc::c_int,
    }
    /*
==================================================================

functions imported from the main executable

==================================================================
*/
    pub type unnamed_0 = libc::c_uint;
    pub const CG_ACOS: unnamed_0 = 111;
    pub const CG_TESTPRINTFLOAT: unnamed_0 = 110;
    pub const CG_TESTPRINTINT: unnamed_0 = 109;
    pub const CG_CEIL: unnamed_0 = 108;
    pub const CG_FLOOR: unnamed_0 = 107;
    pub const CG_SQRT: unnamed_0 = 106;
    pub const CG_ATAN2: unnamed_0 = 105;
    pub const CG_COS: unnamed_0 = 104;
    pub const CG_SIN: unnamed_0 = 103;
    pub const CG_STRNCPY: unnamed_0 = 102;
    pub const CG_MEMCPY: unnamed_0 = 101;
    /*
	CG_LOADCAMERA,
	CG_STARTCAMERA,
	CG_GETCAMERAINFO,
*/
    pub const CG_MEMSET: unnamed_0 = 100;
    // 1.32
    pub const CG_FS_SEEK: unnamed_0 = 89;
    pub const CG_R_INPVS: unnamed_0 = 88;
    pub const CG_R_ADDPOLYSTOSCENE: unnamed_0 = 87;
    pub const CG_GET_ENTITY_TOKEN: unnamed_0 = 86;
    pub const CG_R_ADDADDITIVELIGHTTOSCENE: unnamed_0 = 85;
    pub const CG_CM_TRANSFORMEDCAPSULETRACE: unnamed_0 = 84;
    pub const CG_CM_CAPSULETRACE: unnamed_0 = 83;
    pub const CG_CM_TEMPCAPSULEMODEL: unnamed_0 = 82;
    pub const CG_S_STOPLOOPINGSOUND: unnamed_0 = 81;
    pub const CG_S_ADDREALLOOPINGSOUND: unnamed_0 = 80;
    pub const CG_R_REMAP_SHADER: unnamed_0 = 79;
    pub const CG_CIN_SETEXTENTS: unnamed_0 = 78;
    pub const CG_CIN_DRAWCINEMATIC: unnamed_0 = 77;
    pub const CG_CIN_RUNCINEMATIC: unnamed_0 = 76;
    pub const CG_CIN_STOPCINEMATIC: unnamed_0 = 75;
    pub const CG_CIN_PLAYCINEMATIC: unnamed_0 = 74;
    pub const CG_R_LIGHTFORPOINT: unnamed_0 = 73;
    pub const CG_REMOVECOMMAND: unnamed_0 = 72;
    pub const CG_SNAPVECTOR: unnamed_0 = 71;
    pub const CG_REAL_TIME: unnamed_0 = 70;
    pub const CG_S_STOPBACKGROUNDTRACK: unnamed_0 = 69;
    pub const CG_PC_SOURCE_FILE_AND_LINE: unnamed_0 = 68;
    pub const CG_PC_READ_TOKEN: unnamed_0 = 67;
    pub const CG_PC_FREE_SOURCE: unnamed_0 = 66;
    pub const CG_PC_LOAD_SOURCE: unnamed_0 = 65;
    pub const CG_PC_ADD_GLOBAL_DEFINE: unnamed_0 = 64;
    pub const CG_KEY_GETKEY: unnamed_0 = 63;
    pub const CG_KEY_SETCATCHER: unnamed_0 = 62;
    pub const CG_KEY_GETCATCHER: unnamed_0 = 61;
    pub const CG_KEY_ISDOWN: unnamed_0 = 60;
    pub const CG_R_REGISTERFONT: unnamed_0 = 59;
    pub const CG_MEMORY_REMAINING: unnamed_0 = 58;
    pub const CG_R_REGISTERSHADERNOMIP: unnamed_0 = 57;
    pub const CG_SETUSERCMDVALUE: unnamed_0 = 56;
    pub const CG_GETUSERCMD: unnamed_0 = 55;
    pub const CG_GETCURRENTCMDNUMBER: unnamed_0 = 54;
    pub const CG_GETSERVERCOMMAND: unnamed_0 = 53;
    pub const CG_GETSNAPSHOT: unnamed_0 = 52;
    pub const CG_GETCURRENTSNAPSHOTNUMBER: unnamed_0 = 51;
    pub const CG_GETGAMESTATE: unnamed_0 = 50;
    pub const CG_GETGLCONFIG: unnamed_0 = 49;
    pub const CG_R_LERPTAG: unnamed_0 = 48;
    pub const CG_R_MODELBOUNDS: unnamed_0 = 47;
    pub const CG_R_DRAWSTRETCHPIC: unnamed_0 = 46;
    pub const CG_R_SETCOLOR: unnamed_0 = 45;
    pub const CG_R_RENDERSCENE: unnamed_0 = 44;
    pub const CG_R_ADDLIGHTTOSCENE: unnamed_0 = 43;
    pub const CG_R_ADDPOLYTOSCENE: unnamed_0 = 42;
    pub const CG_R_ADDREFENTITYTOSCENE: unnamed_0 = 41;
    pub const CG_R_CLEARSCENE: unnamed_0 = 40;
    pub const CG_R_REGISTERSHADER: unnamed_0 = 39;
    pub const CG_R_REGISTERSKIN: unnamed_0 = 38;
    pub const CG_R_REGISTERMODEL: unnamed_0 = 37;
    pub const CG_R_LOADWORLDMAP: unnamed_0 = 36;
    pub const CG_S_STARTBACKGROUNDTRACK: unnamed_0 = 35;
    pub const CG_S_REGISTERSOUND: unnamed_0 = 34;
    pub const CG_S_RESPATIALIZE: unnamed_0 = 33;
    pub const CG_S_UPDATEENTITYPOSITION: unnamed_0 = 32;
    pub const CG_S_ADDLOOPINGSOUND: unnamed_0 = 31;
    pub const CG_S_CLEARLOOPINGSOUNDS: unnamed_0 = 30;
    pub const CG_S_STARTLOCALSOUND: unnamed_0 = 29;
    pub const CG_S_STARTSOUND: unnamed_0 = 28;
    pub const CG_CM_MARKFRAGMENTS: unnamed_0 = 27;
    pub const CG_CM_TRANSFORMEDBOXTRACE: unnamed_0 = 26;
    pub const CG_CM_BOXTRACE: unnamed_0 = 25;
    pub const CG_CM_TRANSFORMEDPOINTCONTENTS: unnamed_0 = 24;
    pub const CG_CM_POINTCONTENTS: unnamed_0 = 23;
    pub const CG_CM_TEMPBOXMODEL: unnamed_0 = 22;
    pub const CG_CM_LOADMODEL: unnamed_0 = 21;
    pub const CG_CM_INLINEMODEL: unnamed_0 = 20;
    pub const CG_CM_NUMINLINEMODELS: unnamed_0 = 19;
    pub const CG_CM_LOADMAP: unnamed_0 = 18;
    pub const CG_UPDATESCREEN: unnamed_0 = 17;
    pub const CG_SENDCLIENTCOMMAND: unnamed_0 = 16;
    pub const CG_ADDCOMMAND: unnamed_0 = 15;
    pub const CG_SENDCONSOLECOMMAND: unnamed_0 = 14;
    pub const CG_FS_FCLOSEFILE: unnamed_0 = 13;
    pub const CG_FS_WRITE: unnamed_0 = 12;
    pub const CG_FS_READ: unnamed_0 = 11;
    pub const CG_FS_FOPENFILE: unnamed_0 = 10;
    pub const CG_ARGS: unnamed_0 = 9;
    pub const CG_ARGV: unnamed_0 = 8;
    pub const CG_ARGC: unnamed_0 = 7;
    pub const CG_CVAR_VARIABLESTRINGBUFFER: unnamed_0 = 6;
    pub const CG_CVAR_SET: unnamed_0 = 5;
    pub const CG_CVAR_UPDATE: unnamed_0 = 4;
    pub const CG_CVAR_REGISTER: unnamed_0 = 3;
    pub const CG_MILLISECONDS: unnamed_0 = 2;
    pub const CG_ERROR: unnamed_0 = 1;
    pub const CG_PRINT: unnamed_0 = 0;
    /*
==================================================================

functions exported to the main executable

==================================================================
*/
    pub type unnamed_1 = libc::c_uint;
    //	void	(*CG_MouseEvent)( int dx, int dy );
    pub const CG_EVENT_HANDLING: unnamed_1 = 8;
    //	void	(*CG_KeyEvent)( int key, qboolean down );
    pub const CG_MOUSE_EVENT: unnamed_1 = 7;
    //	int (*CG_LastAttacker)( void );
    pub const CG_KEY_EVENT: unnamed_1 = 6;
    //	int (*CG_CrosshairPlayer)( void );
    pub const CG_LAST_ATTACKER: unnamed_1 = 5;
    //	void (*CG_DrawActiveFrame)( int serverTime, stereoFrame_t stereoView, qboolean demoPlayback );
	// Generates and draws a game scene and status information at the given time.
	// If demoPlayback is set, local movement prediction will not be enabled
    pub const CG_CROSSHAIR_PLAYER: unnamed_1 = 4;
    //	qboolean (*CG_ConsoleCommand)( void );
	// a console command has been issued locally that is not recognized by the
	// main game system.
	// use Cmd_Argc() / Cmd_Argv() to read the command, return qfalse if the
	// command is not known to the game
    pub const CG_DRAW_ACTIVE_FRAME: unnamed_1 = 3;
    //	void CG_Init( int serverMessageNum, int serverCommandSequence, int clientNum )
	// called when the level loads or when the renderer is restarted
	// all media should be registered at this time
	// cgame will display loading status by calling SCR_Update, which
	// will call CG_DrawInformation during the loading process
	// reliableCommandSequence will be 0 on fresh loads, but higher for
	// demos, tourney restarts, or vid_restarts
    pub const CG_SHUTDOWN: unnamed_1 = 1;
    pub const CG_INIT: unnamed_1 = 0;
    use super::{libc};
    use super::q_shared_h::{byte, playerState_t, entityState_t};
}
#[header_src =
      "ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
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
    // can't be increased, because bit flags are used on surfaces
    // can't be increased without changing drawsurf bit packing
    // the last N-bit number (2^REFENTITYNUM_BITS - 1) is reserved for the special world refentity,
//  and this is reflected by the value of MAX_REFENTITIES (which therefore is not a power-of-2)
    // renderfx flags
    // allways have some light (viewmodel, some items)
    // don't draw through eyes, only mirrors (player bodies, chat sprites)
    // only draw through eyes (view weapon, damage blood blob)
    // for view weapon Z crunching
    // This item is a cross hair and will draw over everything similar to
    // DEPTHHACK in stereo rendering mode, with the difference that the
						// projection matrix won't be hacked to reduce the stereo separation as
						// is done for the gun.
    // don't add stencil shadows
    // use refEntity->lightingOrigin instead of refEntity->origin
    // for lighting.  This allows entities to sink into the floor
						// with their origin going solid, and allows all parts of a
						// player to get the same lighting
    // use refEntity->shadowPlane
    // mod the model frames by the maxframes to allow continuous
    // animation without needing to know the frame count
    // refdef flags
    // used for player configuration screen
    // teleportation effect
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct polyVert_t {
        pub xyz: vec3_t,
        pub st: [libc::c_float; 2],
        pub modulate: [byte; 4],
    }
    pub type refEntityType_t = libc::c_uint;
    pub const RT_MAX_REF_ENTITY_TYPE: refEntityType_t = 8;
    // doesn't draw anything, just info for portals
    pub const RT_PORTALSURFACE: refEntityType_t = 7;
    pub const RT_LIGHTNING: refEntityType_t = 6;
    pub const RT_RAIL_RINGS: refEntityType_t = 5;
    pub const RT_RAIL_CORE: refEntityType_t = 4;
    pub const RT_BEAM: refEntityType_t = 3;
    pub const RT_SPRITE: refEntityType_t = 2;
    pub const RT_POLY: refEntityType_t = 1;
    pub const RT_MODEL: refEntityType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refEntity_t {
        pub reType: refEntityType_t,
        pub renderfx: libc::c_int,
        pub hModel: qhandle_t,
        pub lightingOrigin: vec3_t,
        pub shadowPlane: libc::c_float,
        pub axis: [vec3_t; 3],
        pub nonNormalizedAxes: qboolean,
        pub origin: [libc::c_float; 3],
        pub frame: libc::c_int,
        pub oldorigin: [libc::c_float; 3],
        pub oldframe: libc::c_int,
        pub backlerp: libc::c_float,
        pub skinNum: libc::c_int,
        pub customSkin: qhandle_t,
        pub customShader: qhandle_t,
        pub shaderRGBA: [byte; 4],
        pub shaderTexCoord: [libc::c_float; 2],
        pub shaderTime: libc::c_float,
        pub radius: libc::c_float,
        pub rotation: libc::c_float,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refdef_t {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub fov_x: libc::c_float,
        pub fov_y: libc::c_float,
        pub vieworg: vec3_t,
        pub viewaxis: [vec3_t; 3],
        pub time: libc::c_int,
        pub rdflags: libc::c_int,
        pub areamask: [byte; 32],
        pub text: [[libc::c_char; 32]; 8],
    }
    pub type stereoFrame_t = libc::c_uint;
    pub const STEREO_RIGHT: stereoFrame_t = 2;
    pub const STEREO_LEFT: stereoFrame_t = 1;
    pub const STEREO_CENTER: stereoFrame_t = 0;
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
    use super::q_shared_h::{vec3_t, byte, qhandle_t, qboolean};
    use super::{libc};
}
#[header_src =
      "ioq3/code/renderercommon/tr_public.h"]
pub mod tr_public_h {
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
// these are the functions exported by the refresh module
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refexport_t {
        pub Shutdown: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
        pub BeginRegistration: Option<unsafe extern "C" fn(_: *mut glconfig_t)
                                          -> ()>,
        pub RegisterModel: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> qhandle_t>,
        pub RegisterSkin: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> qhandle_t>,
        pub RegisterShader: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char)
                                       -> qhandle_t>,
        pub RegisterShaderNoMip: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> qhandle_t>,
        pub LoadWorld: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> ()>,
        pub SetWorldVisData: Option<unsafe extern "C" fn(_: *const byte)
                                        -> ()>,
        pub EndRegistration: Option<unsafe extern "C" fn() -> ()>,
        pub ClearScene: Option<unsafe extern "C" fn() -> ()>,
        pub AddRefEntityToScene: Option<unsafe extern "C" fn(_:
                                                                 *const refEntity_t)
                                            -> ()>,
        pub AddPolyToScene: Option<unsafe extern "C" fn(_: qhandle_t,
                                                        _: libc::c_int,
                                                        _: *const polyVert_t,
                                                        _: libc::c_int)
                                       -> ()>,
        pub LightForPoint: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t)
                                      -> libc::c_int>,
        pub AddLightToScene: Option<unsafe extern "C" fn(_: *const vec_t,
                                                         _: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_float)
                                        -> ()>,
        pub AddAdditiveLightToScene: Option<unsafe extern "C" fn(_:
                                                                     *const vec_t,
                                                                 _:
                                                                     libc::c_float,
                                                                 _:
                                                                     libc::c_float,
                                                                 _:
                                                                     libc::c_float,
                                                                 _:
                                                                     libc::c_float)
                                                -> ()>,
        pub RenderScene: Option<unsafe extern "C" fn(_: *const refdef_t)
                                    -> ()>,
        pub SetColor: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> ()>,
        pub DrawStretchPic: Option<unsafe extern "C" fn(_: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: qhandle_t) -> ()>,
        pub DrawStretchRaw: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte,
                                                        _: libc::c_int,
                                                        _: qboolean) -> ()>,
        pub UploadCinematic: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *const byte,
                                                         _: libc::c_int,
                                                         _: qboolean) -> ()>,
        pub BeginFrame: Option<unsafe extern "C" fn(_: stereoFrame_t) -> ()>,
        pub EndFrame: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                  _: *mut libc::c_int) -> ()>,
        pub MarkFragments: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *const vec3_t,
                                                       _: *const vec_t,
                                                       _: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: libc::c_int,
                                                       _: *mut markFragment_t)
                                      -> libc::c_int>,
        pub LerpTag: Option<unsafe extern "C" fn(_: *mut orientation_t,
                                                 _: qhandle_t, _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_float,
                                                 _: *const libc::c_char)
                                -> libc::c_int>,
        pub ModelBounds: Option<unsafe extern "C" fn(_: qhandle_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t) -> ()>,
        pub RegisterFont: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: libc::c_int,
                                                      _: *mut fontInfo_t)
                                     -> ()>,
        pub RemapShader: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *const libc::c_char,
                                                     _: *const libc::c_char)
                                    -> ()>,
        pub GetEntityToken: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> qboolean>,
        pub inPVS: Option<unsafe extern "C" fn(_: *const vec_t,
                                               _: *const vec_t) -> qboolean>,
        pub TakeVideoFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *mut byte,
                                                        _: *mut byte,
                                                        _: qboolean) -> ()>,
    }
    use super::q_shared_h::{qboolean, qhandle_t, byte, vec_t, vec3_t,
                            markFragment_t, orientation_t, fontInfo_t};
    use super::tr_types_h::{glconfig_t, refEntity_t, polyVert_t, refdef_t,
                            stereoFrame_t};
    use super::{libc};
}
#[header_src = "/usr/include/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src = "/usr/include/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/opus/opus_types.h"]
pub mod opus_types_h {
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int32_t};
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    use super::opus_types_h::{opus_int32};
    use super::{libc};
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
        /* *
 */
        /* * Allocates and initializes an encoder state.
 * There are three coding modes:
 *
 * @ref OPUS_APPLICATION_VOIP gives best quality at a given bitrate for voice
 *    signals. It enhances the  input signal by high-pass filtering and
 *    emphasizing formants and harmonics. Optionally  it includes in-band
 *    forward error correction to protect against packet loss. Use this
 *    mode for typical VoIP applications. Because of the enhancement,
 *    even at high bitrates the output may sound different from the input.
 *
 * @ref OPUS_APPLICATION_AUDIO gives best quality at a given bitrate for most
 *    non-voice signals like music. Use this mode for music and mixed
 *    (music/voice) content, broadcast, and applications requiring less
 *    than 15 ms of coding delay.
 *
 * @ref OPUS_APPLICATION_RESTRICTED_LOWDELAY configures low-delay mode that
 *    disables the speech-optimized mode in exchange for slightly reduced delay.
 *    This mode can only be set on an newly initialized or freshly reset encoder
 *    because it changes the codec delay.
 *
 * This is useful when the caller knows that the speech-optimized modes will not be needed (use with caution).
 * @param [in] Fs <tt>opus_int32</tt>: Sampling rate of input signal (Hz)
 *                                     This must be one of 8000, 12000, 16000,
 *                                     24000, or 48000.
 * @param [in] channels <tt>int</tt>: Number of channels (1 or 2) in input signal
 * @param [in] application <tt>int</tt>: Coding mode (@ref OPUS_APPLICATION_VOIP/@ref OPUS_APPLICATION_AUDIO/@ref OPUS_APPLICATION_RESTRICTED_LOWDELAY)
 * @param [out] error <tt>int*</tt>: @ref opus_errorcodes
 * @note Regardless of the sampling rate and number channels selected, the Opus encoder
 * can switch to a lower audio bandwidth or number of channels if the bitrate
 * selected is too low. This also means that it is safe to always use 48 kHz stereo input
 * and let the encoder optimize the encoding.
 */
        #[no_mangle]
        pub fn opus_encoder_create(Fs: opus_int32, channels: libc::c_int,
                                   application: libc::c_int,
                                   error: *mut libc::c_int)
         -> *mut OpusEncoder;
        /* * Allocates and initializes a decoder state.
  * @param [in] Fs <tt>opus_int32</tt>: Sample rate to decode at (Hz).
  *                                     This must be one of 8000, 12000, 16000,
  *                                     24000, or 48000.
  * @param [in] channels <tt>int</tt>: Number of channels (1 or 2) to decode
  * @param [out] error <tt>int*</tt>: #OPUS_OK Success or @ref opus_errorcodes
  *
  * Internally Opus stores data at 48000 Hz, so that should be the default
  * value for Fs. However, the decoder can efficiently decode to buffers
  * at 8, 12, 16, and 24 kHz so if for some reason the caller cannot use
  * data at the full sample rate, or knows the compressed data doesn't
  * use the full frequency range, it can request decoding at a reduced
  * rate. Likewise, the decoder is capable of filling in either mono or
  * interleaved stereo pcm buffers, at the caller's request.
  */
        #[no_mangle]
        pub fn opus_decoder_create(Fs: opus_int32, channels: libc::c_int,
                                   error: *mut libc::c_int)
         -> *mut OpusDecoder;
    }
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
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
// client.h -- primary header for client
    /* USE_CURL */
    // file full of random crap that gets used to create cl_guid
    // time between connection packet retransmits
    // snapshots are a view of the server at a given time
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clSnapshot_t {
        pub valid: qboolean,
        pub snapFlags: libc::c_int,
        pub serverTime: libc::c_int,
        pub messageNum: libc::c_int,
        pub deltaNum: libc::c_int,
        pub ping: libc::c_int,
        pub areamask: [byte; 32],
        pub cmdNum: libc::c_int,
        pub ps: playerState_t,
        pub numEntities: libc::c_int,
        pub parseEntitiesNum: libc::c_int,
        pub serverCommandNum: libc::c_int,
    }
    /*
=============================================================================

the clientActive_t structure is wiped completely at every
new gamestate_t, potentially several times during an established connection

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct outPacket_t {
        pub p_cmdNumber: libc::c_int,
        pub p_serverTime: libc::c_int,
        pub p_realtime: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientActive_t {
        pub timeoutcount: libc::c_int,
        pub snap: clSnapshot_t,
        pub serverTime: libc::c_int,
        pub oldServerTime: libc::c_int,
        pub oldFrameServerTime: libc::c_int,
        pub serverTimeDelta: libc::c_int,
        pub extrapolatedSnapshot: qboolean,
        pub newSnapshots: qboolean,
        pub gameState: gameState_t,
        pub mapname: [libc::c_char; 64],
        pub parseEntitiesNum: libc::c_int,
        pub mouseDx: [libc::c_int; 2],
        pub mouseDy: [libc::c_int; 2],
        pub mouseIndex: libc::c_int,
        pub joystickAxis: [libc::c_int; 16],
        pub cgameUserCmdValue: libc::c_int,
        pub cgameSensitivity: libc::c_float,
        pub cmds: [usercmd_t; 64],
        pub cmdNumber: libc::c_int,
        pub outPackets: [outPacket_t; 32],
        pub viewangles: vec3_t,
        pub serverId: libc::c_int,
        pub snapshots: [clSnapshot_t; 32],
        pub entityBaselines: [entityState_t; 1024],
        pub parseEntities: [entityState_t; 8192],
    }
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
    use super::q_shared_h::{qboolean, byte, playerState_t, gameState_t,
                            usercmd_t, vec3_t, entityState_t, connstate_t,
                            fileHandle_t, qhandle_t, cvar_t, e_status};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t, stereoFrame_t};
    use super::tr_public_h::{refexport_t};
    extern "C" {
        //=============================================================================
        // interface to cgame dll or vm
        #[no_mangle]
        pub static mut cgvm: *mut vm_t;
        #[no_mangle]
        pub static mut cl: clientActive_t;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        // interface to refresh .dll
        #[no_mangle]
        pub static mut re: refexport_t;
        #[no_mangle]
        pub static mut cl_timeNudge: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_showTimeDelta: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_freezeDemo: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_timedemo: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_activeAction: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_useMumble: *mut cvar_t;
        // 20ms at 48k
        // 3 frame is 60ms of audio, the max opus will encode at once
        //=================================================
        //
// cl_main
//
        #[no_mangle]
        pub fn CL_AddReliableCommand(cmd: *const libc::c_char,
                                     isDisconnectCmd: qboolean);
        #[no_mangle]
        pub fn CL_ReadDemoMessage();
        #[no_mangle]
        pub fn CL_CheckPaused() -> qboolean;
        //
// cl_parse.c
//
        #[no_mangle]
        pub static mut cl_connectedToPureServer: libc::c_int;
        #[no_mangle]
        pub static mut cl_connectedToCheatServer: libc::c_int;
        #[no_mangle]
        pub fn CL_Voip_f();
        #[no_mangle]
        pub fn CL_SystemInfoChanged();
        #[no_mangle]
        pub fn Con_ClearNotify();
        #[no_mangle]
        pub fn Con_Close();
        #[no_mangle]
        pub fn SCR_UpdateScreen();
        #[no_mangle]
        pub fn CIN_PlayCinematic(arg0: *const libc::c_char, xpos: libc::c_int,
                                 ypos: libc::c_int, width: libc::c_int,
                                 height: libc::c_int, bits: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        pub fn CIN_StopCinematic(handle: libc::c_int) -> e_status;
        #[no_mangle]
        pub fn CIN_RunCinematic(handle: libc::c_int) -> e_status;
        #[no_mangle]
        pub fn CIN_DrawCinematic(handle: libc::c_int);
        #[no_mangle]
        pub fn CIN_SetExtents(handle: libc::c_int, x: libc::c_int,
                              y: libc::c_int, w: libc::c_int, h: libc::c_int);
        #[no_mangle]
        pub fn Key_GetCatcher() -> libc::c_int;
        #[no_mangle]
        pub fn Key_SetCatcher(catcher: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
    pub type botlib_export_t = botlib_export_s;
    //bot AI library imported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_export_s {
        pub aas: aas_export_t,
        pub ea: ea_export_t,
        pub ai: ai_export_t,
        pub BotLibSetup: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotLibShutdown: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotLibVarSet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *const libc::c_char)
                                     -> libc::c_int>,
        pub BotLibVarGet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub PC_AddGlobalDefine: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
        pub PC_LoadSourceHandle: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_int>,
        pub PC_FreeSourceHandle: Option<unsafe extern "C" fn(_: libc::c_int)
                                            -> libc::c_int>,
        pub PC_ReadTokenHandle: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut pc_token_t)
                                           -> libc::c_int>,
        pub PC_SourceFileAndLine: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut libc::c_int)
                                             -> libc::c_int>,
        pub BotLibStartFrame: Option<unsafe extern "C" fn(_: libc::c_float)
                                         -> libc::c_int>,
        pub BotLibLoadMap: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> libc::c_int>,
        pub BotLibUpdateEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut bot_entitystate_t)
                                           -> libc::c_int>,
        pub Test: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *mut libc::c_char,
                                              _: *mut vec_t, _: *mut vec_t)
                             -> libc::c_int>,
    }
    pub type bot_entitystate_t = bot_entitystate_s;
    // BSPTRACE
    //entity state
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_entitystate_s {
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    pub type ai_export_t = ai_export_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ai_export_s {
        pub BotLoadCharacter: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_char,
                                                          _: libc::c_float)
                                         -> libc::c_int>,
        pub BotFreeCharacter: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub Characteristic_Float: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> libc::c_float>,
        pub Characteristic_BFloat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int,
                                                               _:
                                                                   libc::c_float,
                                                               _:
                                                                   libc::c_float)
                                              -> libc::c_float>,
        pub Characteristic_Integer: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int>,
        pub Characteristic_BInteger: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> libc::c_int>,
        pub Characteristic_String: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _: libc::c_int)
                                              -> ()>,
        pub BotAllocChatState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeChatState: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotQueueConsoleMessage: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut libc::c_char)
                                               -> ()>,
        pub BotRemoveConsoleMessage: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> ()>,
        pub BotNextConsoleMessage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut bot_consolemessage_s)
                                              -> libc::c_int>,
        pub BotNumConsoleMessages: Option<unsafe extern "C" fn(_: libc::c_int)
                                              -> libc::c_int>,
        pub BotInitialChat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char)
                                       -> ()>,
        pub BotNumInitialChats: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
        pub BotReplyChat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut libc::c_char,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char)
                                     -> libc::c_int>,
        pub BotChatLength: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> libc::c_int>,
        pub BotEnterChat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
        pub BotGetChatMessage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                           _:
                                                               *mut libc::c_char,
                                                           _: libc::c_int)
                                          -> ()>,
        pub StringContains: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
        pub BotFindMatch: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                      _: *mut bot_match_s,
                                                      _: libc::c_ulong)
                                     -> libc::c_int>,
        pub BotMatchVariable: Option<unsafe extern "C" fn(_: *mut bot_match_s,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut libc::c_char,
                                                          _: libc::c_int)
                                         -> ()>,
        pub UnifyWhiteSpaces: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub BotReplaceSynonyms: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char,
                                                            _: libc::c_ulong)
                                           -> ()>,
        pub BotLoadChatFile: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: *mut libc::c_char,
                                                         _: *mut libc::c_char)
                                        -> libc::c_int>,
        pub BotSetChatGender: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
        pub BotSetChatName: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> ()>,
        pub BotResetGoalState: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotResetAvoidGoals: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotRemoveFromAvoidGoals: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> ()>,
        pub BotPushGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut bot_goal_s)
                                    -> ()>,
        pub BotPopGoal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotEmptyGoalStack: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotDumpAvoidGoals: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotDumpGoalStack: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotGoalName: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_char,
                                                     _: libc::c_int) -> ()>,
        pub BotGetTopGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut bot_goal_s)
                                      -> libc::c_int>,
        pub BotGetSecondGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: *mut bot_goal_s)
                                         -> libc::c_int>,
        pub BotChooseLTGItem: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: *mut vec_t,
                                                          _: *mut libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
        pub BotChooseNBGItem: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: *mut vec_t,
                                                          _: *mut libc::c_int,
                                                          _: libc::c_int,
                                                          _: *mut bot_goal_s,
                                                          _: libc::c_float)
                                         -> libc::c_int>,
        pub BotTouchingGoal: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                         _: *mut bot_goal_s)
                                        -> libc::c_int>,
        pub BotItemGoalInVisButNotVisible: Option<unsafe extern "C" fn(_:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *mut bot_goal_s)
                                                      -> libc::c_int>,
        pub BotGetLevelItemGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _:
                                                                 *mut libc::c_char,
                                                             _:
                                                                 *mut bot_goal_s)
                                            -> libc::c_int>,
        pub BotGetNextCampSpotGoal: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut bot_goal_s)
                                               -> libc::c_int>,
        pub BotGetMapLocationGoal: Option<unsafe extern "C" fn(_:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   *mut bot_goal_s)
                                              -> libc::c_int>,
        pub BotAvoidGoalTime: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_float>,
        pub BotSetAvoidGoalTime: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: libc::c_int,
                                                             _: libc::c_float)
                                            -> ()>,
        pub BotInitLevelItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotUpdateEntityItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotLoadItemWeights: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
        pub BotFreeItemWeights: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotInterbreedGoalFuzzyLogic: Option<unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_int)
                                                    -> ()>,
        pub BotSaveGoalFuzzyLogic: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut libc::c_char)
                                              -> ()>,
        pub BotMutateGoalFuzzyLogic: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_float)
                                                -> ()>,
        pub BotAllocGoalState: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
        pub BotFreeGoalState: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotResetMoveState: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotMoveToGoal: Option<unsafe extern "C" fn(_:
                                                           *mut bot_moveresult_s,
                                                       _: libc::c_int,
                                                       _: *mut bot_goal_s,
                                                       _: libc::c_int) -> ()>,
        pub BotMoveInDirection: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: *mut vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int)
                                           -> libc::c_int>,
        pub BotResetAvoidReach: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotResetLastAvoidReach: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()>,
        pub BotReachabilityArea: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                             _: libc::c_int)
                                            -> libc::c_int>,
        pub BotMovementViewTarget: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut bot_goal_s,
                                                               _: libc::c_int,
                                                               _:
                                                                   libc::c_float,
                                                               _: *mut vec_t)
                                              -> libc::c_int>,
        pub BotPredictVisiblePosition: Option<unsafe extern "C" fn(_:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut bot_goal_s,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t)
                                                  -> libc::c_int>,
        pub BotAllocMoveState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeMoveState: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotInitMoveState: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut bot_initmove_s)
                                         -> ()>,
        pub BotAddAvoidSpot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: *mut vec_t,
                                                         _: libc::c_float,
                                                         _: libc::c_int)
                                        -> ()>,
        pub BotChooseBestFightWeapon: Option<unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> libc::c_int>,
        pub BotGetWeaponInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut weaponinfo_s)
                                         -> ()>,
        pub BotLoadWeaponWeights: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_char)
                                             -> libc::c_int>,
        pub BotAllocWeaponState: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
        pub BotFreeWeaponState: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotResetWeaponState: Option<unsafe extern "C" fn(_: libc::c_int)
                                            -> ()>,
        pub GeneticParentsAndChildSelection: Option<unsafe extern "C" fn(_:
                                                                             libc::c_int,
                                                                         _:
                                                                             *mut libc::c_float,
                                                                         _:
                                                                             *mut libc::c_int,
                                                                         _:
                                                                             *mut libc::c_int,
                                                                         _:
                                                                             *mut libc::c_int)
                                                        -> libc::c_int>,
    }
    pub type ea_export_t = ea_export_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ea_export_s {
        pub EA_Command: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_char)
                                   -> ()>,
        pub EA_Say: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: *mut libc::c_char) -> ()>,
        pub EA_SayTeam: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_char)
                                   -> ()>,
        pub EA_Action: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int) -> ()>,
        pub EA_Gesture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Talk: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Attack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Use: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Respawn: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveUp: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveDown: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveForward: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
        pub EA_MoveBack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveLeft: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveRight: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Crouch: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_SelectWeapon: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
        pub EA_Jump: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_DelayedJump: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
        pub EA_Move: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *mut vec_t,
                                                 _: libc::c_float) -> ()>,
        pub EA_View: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *mut vec_t) -> ()>,
        pub EA_EndRegular: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_float)
                                      -> ()>,
        pub EA_GetInput: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_float,
                                                     _: *mut bot_input_t)
                                    -> ()>,
        pub EA_ResetInput: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    }
    pub type bot_input_t = bot_input_s;
    //debug line colors
    //0xf2f2f0f0L
    //0xd0d1d2d3L
    //0xf3f3f1f1L
    //0xdcdddedfL
    //0xe0e1e2e3L
    //Print types
    //console message types
    //botlib error codes
    //no error
    //library not setup
    //invalid entity number
    //no AAS file available
    //cannot open AAS file
    //incorrect AAS file id
    //incorrect AAS file version
    //cannot read AAS file lump
    //cannot load initial chats
    //cannot load item weights
    //cannot load item config
    //cannot load weapon weights
    //cannot load weapon config
    //action flags
    //the bot input, will be converted to a usercmd_t
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_input_s {
        pub thinktime: libc::c_float,
        pub dir: vec3_t,
        pub speed: libc::c_float,
        pub viewangles: vec3_t,
        pub actionflags: libc::c_int,
        pub weapon: libc::c_int,
    }
    pub type aas_export_t = aas_export_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_export_s {
        pub AAS_EntityInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *mut aas_entityinfo_s)
                                       -> ()>,
        pub AAS_Initialized: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub AAS_PresenceTypeBoundingBox: Option<unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut vec_t,
                                                                     _:
                                                                         *mut vec_t)
                                                    -> ()>,
        pub AAS_Time: Option<unsafe extern "C" fn() -> libc::c_float>,
        pub AAS_PointAreaNum: Option<unsafe extern "C" fn(_: *mut vec_t)
                                         -> libc::c_int>,
        pub AAS_PointReachabilityAreaIndex: Option<unsafe extern "C" fn(_:
                                                                            *mut vec_t)
                                                       -> libc::c_int>,
        pub AAS_TraceAreas: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                        _: *mut vec_t,
                                                        _: *mut libc::c_int,
                                                        _: *mut vec3_t,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
        pub AAS_BBoxAreas: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: *mut libc::c_int,
                                                       _: libc::c_int)
                                      -> libc::c_int>,
        pub AAS_AreaInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut aas_areainfo_s)
                                     -> libc::c_int>,
        pub AAS_PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                          -> libc::c_int>,
        pub AAS_NextBSPEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
        pub AAS_ValueForBSPEpairKey: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     libc::c_int)
                                                -> libc::c_int>,
        pub AAS_VectorForBSPEpairKey: Option<unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_char,
                                                                  _:
                                                                      *mut vec_t)
                                                 -> libc::c_int>,
        pub AAS_FloatForBSPEpairKey: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     *mut libc::c_float)
                                                -> libc::c_int>,
        pub AAS_IntForBSPEpairKey: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   *mut libc::c_int)
                                              -> libc::c_int>,
        pub AAS_AreaReachability: Option<unsafe extern "C" fn(_: libc::c_int)
                                             -> libc::c_int>,
        pub AAS_AreaTravelTimeToGoalArea: Option<unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          *mut vec_t,
                                                                      _:
                                                                          libc::c_int,
                                                                      _:
                                                                          libc::c_int)
                                                     -> libc::c_int>,
        pub AAS_EnableRoutingArea: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int)
                                              -> libc::c_int>,
        pub AAS_PredictRoute: Option<unsafe extern "C" fn(_:
                                                              *mut aas_predictroute_s,
                                                          _: libc::c_int,
                                                          _: *mut vec_t,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
        pub AAS_AlternativeRouteGoals: Option<unsafe extern "C" fn(_:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut aas_altroutegoal_s,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int>,
        pub AAS_Swimming: Option<unsafe extern "C" fn(_: *mut vec_t)
                                     -> libc::c_int>,
        pub AAS_PredictClientMovement: Option<unsafe extern "C" fn(_:
                                                                       *mut aas_clientmove_s,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int>,
    }
    use super::{libc};
    use super::q_shared_h::{pc_token_t, vec_t, vec3_t};
    extern "C" {
        pub type weaponinfo_s;
        pub type bot_initmove_s;
        pub type bot_goal_s;
        pub type bot_moveresult_s;
        pub type bot_match_s;
        pub type bot_consolemessage_s;
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
/*****************************************************************************
 * name:		botlib.h
 *
 * desc:		bot AI library
 *
 * $Archive: /source/code/game/botai.h $
 *
 *****************************************************************************/
        pub type aas_clientmove_s;
        pub type aas_altroutegoal_s;
        pub type aas_predictroute_s;
        pub type aas_areainfo_s;
        pub type aas_entityinfo_s;
    }
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn cos(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn ceil(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::{libc};
    use super::q_shared_h::{qboolean, clipHandle_t, vec_t, trace_t};
    extern "C" {
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
        #[no_mangle]
        pub fn CM_LoadMap(name: *const libc::c_char, clientload: qboolean,
                          checksum: *mut libc::c_int);
        #[no_mangle]
        pub fn CM_InlineModel(index: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_TempBoxModel(mins: *const vec_t, maxs: *const vec_t,
                               capsule: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_NumInlineModels() -> libc::c_int;
        // returns an ORed contents mask
        #[no_mangle]
        pub fn CM_PointContents(p: *const vec_t, model: clipHandle_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn CM_TransformedPointContents(p: *const vec_t,
                                           model: clipHandle_t,
                                           origin: *const vec_t,
                                           angles: *const vec_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn CM_BoxTrace(results: *mut trace_t, start: *const vec_t,
                           end: *const vec_t, mins: *mut vec_t,
                           maxs: *mut vec_t, model: clipHandle_t,
                           brushmask: libc::c_int, capsule: libc::c_int);
        #[no_mangle]
        pub fn CM_TransformedBoxTrace(results: *mut trace_t,
                                      start: *const vec_t, end: *const vec_t,
                                      mins: *mut vec_t, maxs: *mut vec_t,
                                      model: clipHandle_t,
                                      brushmask: libc::c_int,
                                      origin: *const vec_t,
                                      angles: *const vec_t,
                                      capsule: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/client/keys.h"]
pub mod keys_h {
    use super::q_shared_h::{qboolean};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Key_IsDown(keynum: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn Key_GetKey(binding: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    use super::q_shared_h::{vec_t, sfxHandle_t, qboolean, vec3_t};
    use super::{libc};
    extern "C" {
        // if origin is NULL, the sound will be dynamically sourced from the entity
        #[no_mangle]
        pub fn S_StartSound(origin: *mut vec_t, entnum: libc::c_int,
                            entchannel: libc::c_int, sfx: sfxHandle_t);
        #[no_mangle]
        pub fn S_StartLocalSound(sfx: sfxHandle_t, channelNum: libc::c_int);
        #[no_mangle]
        pub fn S_StartBackgroundTrack(intro: *const libc::c_char,
                                      loop_0: *const libc::c_char);
        #[no_mangle]
        pub fn S_StopBackgroundTrack();
        // all continuous looping sounds must be added before calling S_Update
        #[no_mangle]
        pub fn S_ClearLoopingSounds(killall: qboolean);
        #[no_mangle]
        pub fn S_AddLoopingSound(entityNum: libc::c_int, origin: *const vec_t,
                                 velocity: *const vec_t, sfx: sfxHandle_t);
        #[no_mangle]
        pub fn S_AddRealLoopingSound(entityNum: libc::c_int,
                                     origin: *const vec_t,
                                     velocity: *const vec_t,
                                     sfx: sfxHandle_t);
        #[no_mangle]
        pub fn S_StopLoopingSound(entityNum: libc::c_int);
        // recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
        #[no_mangle]
        pub fn S_Respatialize(entityNum: libc::c_int, origin: *const vec_t,
                              axis: *mut vec3_t, inwater: libc::c_int);
        // let the sound system know where an entity currently is
        #[no_mangle]
        pub fn S_UpdateEntityPosition(entityNum: libc::c_int,
                                      origin: *const vec_t);
        // RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
        #[no_mangle]
        pub fn S_RegisterSound(sample: *const libc::c_char,
                               compressed: qboolean) -> sfxHandle_t;
    }
}
#[header_src =
      "ioq3/code/client/cl_cgame.c"]
pub mod cl_cgame_c {
    use super::stdint_h::{intptr_t};
    use super::botlib_h::{botlib_export_t};
    use super::{libc};
    use super::q_shared_h::{qboolean, usercmd_t, gameState_t, entityState_t};
    use super::cg_public_h::{snapshot_t};
    use super::tr_types_h::{glconfig_t};
    extern "C" {
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
// cl_cgame.c  -- client system interaction with client game
        #[no_mangle]
        pub static mut botlib_export: *mut botlib_export_t;
    }
}
#[header_src =
      "ioq3/code/client/libmumblelink.h"]
pub mod libmumblelink_h {
    use super::{libc};
    extern "C" {
        /* libmumblelink.h -- mumble link interface

  Copyright (C) 2008 Ludwig Nussel <ludwig.nussel@suse.de>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

*/
        #[no_mangle]
        pub fn mumble_link(name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn mumble_islinked() -> libc::c_int;
    }
}
use self::types_h::{__uint8_t, __int32_t};
use self::stdint_intn_h::{int32_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t, qhandle_t,
                       sfxHandle_t, fileHandle_t, clipHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cplane_s,
                       pc_token_s, pc_token_t, fsMode_t, FS_APPEND_SYNC,
                       FS_APPEND, FS_WRITE, FS_READ, cvar_s, cvar_t,
                       cvarHandle_t, vmCvar_t, cplane_t, trace_t,
                       markFragment_t, orientation_t, gameState_t,
                       playerState_s, playerState_t, usercmd_s, usercmd_t,
                       trType_t, TR_GRAVITY, TR_SINE, TR_LINEAR_STOP,
                       TR_LINEAR, TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, connstate_t,
                       CA_CINEMATIC, CA_ACTIVE, CA_PRIMED, CA_LOADING,
                       CA_CONNECTED, CA_CHALLENGING, CA_CONNECTING,
                       CA_AUTHORIZING, CA_DISCONNECTED, CA_UNINITIALIZED,
                       glyphInfo_t, fontInfo_t, qtime_s, qtime_t, e_status,
                       FMV_ID_WAIT, FMV_LOOPED, FMV_ID_IDLE, FMV_ID_BLT,
                       FMV_EOF, FMV_PLAY, FMV_IDLE, qsnapvectorsse, Q_acos,
                       Com_sprintf, Info_ValueForKey, Com_Error, Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t, vm_t,
                      vmInterpret_t, VMI_COMPILED, VMI_BYTECODE, VMI_NATIVE,
                      xcommand_t, vm_s, VM_Create, VM_Free, VM_Call, VM_Debug,
                      VM_ArgPtr, Cbuf_AddText, Cmd_AddCommand,
                      Cmd_RemoveCommandSafe, Cmd_Argc, Cmd_Argv,
                      Cmd_ArgvBuffer, Cmd_ArgsFrom, Cmd_ArgsBuffer,
                      Cmd_TokenizeString, Cvar_Register, Cvar_Update,
                      Cvar_Set, Cvar_SetSafe, Cvar_VariableValue,
                      Cvar_VariableStringBuffer, Cvar_SetCheatState, FS_Write,
                      FS_Read, FS_FCloseFile, FS_FOpenFileByMode, FS_Seek,
                      Com_DPrintf, Com_RealTime, com_timescale,
                      com_sv_running, sv_paused, Hunk_MemoryRemaining,
                      Com_TouchMemory, Sys_Milliseconds,
                      Sys_LowPhysicalMemory};
use self::cg_public_h::{CG_CONSOLE_COMMAND, snapshot_t, unnamed_0, CG_ACOS,
                        CG_TESTPRINTFLOAT, CG_TESTPRINTINT, CG_CEIL, CG_FLOOR,
                        CG_SQRT, CG_ATAN2, CG_COS, CG_SIN, CG_STRNCPY,
                        CG_MEMCPY, CG_MEMSET, CG_FS_SEEK, CG_R_INPVS,
                        CG_R_ADDPOLYSTOSCENE, CG_GET_ENTITY_TOKEN,
                        CG_R_ADDADDITIVELIGHTTOSCENE,
                        CG_CM_TRANSFORMEDCAPSULETRACE, CG_CM_CAPSULETRACE,
                        CG_CM_TEMPCAPSULEMODEL, CG_S_STOPLOOPINGSOUND,
                        CG_S_ADDREALLOOPINGSOUND, CG_R_REMAP_SHADER,
                        CG_CIN_SETEXTENTS, CG_CIN_DRAWCINEMATIC,
                        CG_CIN_RUNCINEMATIC, CG_CIN_STOPCINEMATIC,
                        CG_CIN_PLAYCINEMATIC, CG_R_LIGHTFORPOINT,
                        CG_REMOVECOMMAND, CG_SNAPVECTOR, CG_REAL_TIME,
                        CG_S_STOPBACKGROUNDTRACK, CG_PC_SOURCE_FILE_AND_LINE,
                        CG_PC_READ_TOKEN, CG_PC_FREE_SOURCE,
                        CG_PC_LOAD_SOURCE, CG_PC_ADD_GLOBAL_DEFINE,
                        CG_KEY_GETKEY, CG_KEY_SETCATCHER, CG_KEY_GETCATCHER,
                        CG_KEY_ISDOWN, CG_R_REGISTERFONT, CG_MEMORY_REMAINING,
                        CG_R_REGISTERSHADERNOMIP, CG_SETUSERCMDVALUE,
                        CG_GETUSERCMD, CG_GETCURRENTCMDNUMBER,
                        CG_GETSERVERCOMMAND, CG_GETSNAPSHOT,
                        CG_GETCURRENTSNAPSHOTNUMBER, CG_GETGAMESTATE,
                        CG_GETGLCONFIG, CG_R_LERPTAG, CG_R_MODELBOUNDS,
                        CG_R_DRAWSTRETCHPIC, CG_R_SETCOLOR, CG_R_RENDERSCENE,
                        CG_R_ADDLIGHTTOSCENE, CG_R_ADDPOLYTOSCENE,
                        CG_R_ADDREFENTITYTOSCENE, CG_R_CLEARSCENE,
                        CG_R_REGISTERSHADER, CG_R_REGISTERSKIN,
                        CG_R_REGISTERMODEL, CG_R_LOADWORLDMAP,
                        CG_S_STARTBACKGROUNDTRACK, CG_S_REGISTERSOUND,
                        CG_S_RESPATIALIZE, CG_S_UPDATEENTITYPOSITION,
                        CG_S_ADDLOOPINGSOUND, CG_S_CLEARLOOPINGSOUNDS,
                        CG_S_STARTLOCALSOUND, CG_S_STARTSOUND,
                        CG_CM_MARKFRAGMENTS, CG_CM_TRANSFORMEDBOXTRACE,
                        CG_CM_BOXTRACE, CG_CM_TRANSFORMEDPOINTCONTENTS,
                        CG_CM_POINTCONTENTS, CG_CM_TEMPBOXMODEL,
                        CG_CM_LOADMODEL, CG_CM_INLINEMODEL,
                        CG_CM_NUMINLINEMODELS, CG_CM_LOADMAP, CG_UPDATESCREEN,
                        CG_SENDCLIENTCOMMAND, CG_ADDCOMMAND,
                        CG_SENDCONSOLECOMMAND, CG_FS_FCLOSEFILE, CG_FS_WRITE,
                        CG_FS_READ, CG_FS_FOPENFILE, CG_ARGS, CG_ARGV,
                        CG_ARGC, CG_CVAR_VARIABLESTRINGBUFFER, CG_CVAR_SET,
                        CG_CVAR_UPDATE, CG_CVAR_REGISTER, CG_MILLISECONDS,
                        CG_ERROR, CG_PRINT, unnamed_1, CG_EVENT_HANDLING,
                        CG_MOUSE_EVENT, CG_KEY_EVENT, CG_LAST_ATTACKER,
                        CG_CROSSHAIR_PLAYER, CG_DRAW_ACTIVE_FRAME,
                        CG_SHUTDOWN, CG_INIT};
use self::tr_types_h::{polyVert_t, refEntityType_t, RT_MAX_REF_ENTITY_TYPE,
                       RT_PORTALSURFACE, RT_LIGHTNING, RT_RAIL_RINGS,
                       RT_RAIL_CORE, RT_BEAM, RT_SPRITE, RT_POLY, RT_MODEL,
                       refEntity_t, refdef_t, stereoFrame_t, STEREO_RIGHT,
                       STEREO_LEFT, STEREO_CENTER, textureCompression_t,
                       TC_S3TC_ARB, TC_S3TC, TC_NONE, glDriverType_t,
                       GLDRV_VOODOO, GLDRV_STANDALONE, GLDRV_ICD,
                       glHardwareType_t, GLHW_PERMEDIA2, GLHW_RAGEPRO,
                       GLHW_RIVA128, GLHW_3DFX_2D3D, GLHW_GENERIC,
                       glconfig_t};
use self::tr_public_h::{refexport_t};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_types_h::{opus_int32};
use self::opus_h::{OpusEncoder, OpusDecoder, opus_encoder_create,
                   opus_decoder_create};
use self::client_h::{clSnapshot_t, outPacket_t, clientActive_t,
                     clientConnection_t, serverInfo_t, clientStatic_t, cgvm,
                     cl, clc, cls, re, cl_timeNudge, cl_showTimeDelta,
                     cl_freezeDemo, cl_timedemo, cl_activeAction,
                     cl_useMumble, CL_AddReliableCommand, CL_ReadDemoMessage,
                     CL_CheckPaused, cl_connectedToPureServer,
                     cl_connectedToCheatServer, CL_Voip_f,
                     CL_SystemInfoChanged, Con_ClearNotify, Con_Close,
                     SCR_UpdateScreen, CIN_PlayCinematic, CIN_StopCinematic,
                     CIN_RunCinematic, CIN_DrawCinematic, CIN_SetExtents,
                     Key_GetCatcher, Key_SetCatcher};
use self::botlib_h::{botlib_export_t, botlib_export_s, bot_entitystate_t,
                     bot_entitystate_s, ai_export_t, ai_export_s, ea_export_t,
                     ea_export_s, bot_input_t, bot_input_s, aas_export_t,
                     aas_export_s, weaponinfo_s, bot_initmove_s, bot_goal_s,
                     bot_moveresult_s, bot_match_s, bot_consolemessage_s,
                     aas_clientmove_s, aas_altroutegoal_s, aas_predictroute_s,
                     aas_areainfo_s, aas_entityinfo_s};
use self::assert_h::{__assert_fail};
use self::mathcalls_h::{atan2, cos, sin, sqrt, ceil, floor};
use self::string_h::{memcpy, memset, strncpy, strcat, strcmp, strlen};
use self::stdlib_h::{atoi, abs};
use self::cm_public_h::{CM_LoadMap, CM_InlineModel, CM_TempBoxModel,
                        CM_NumInlineModels, CM_PointContents,
                        CM_TransformedPointContents, CM_BoxTrace,
                        CM_TransformedBoxTrace};
use self::keys_h::{Key_IsDown, Key_GetKey};
use self::snd_public_h::{S_StartSound, S_StartLocalSound,
                         S_StartBackgroundTrack, S_StopBackgroundTrack,
                         S_ClearLoopingSounds, S_AddLoopingSound,
                         S_AddRealLoopingSound, S_StopLoopingSound,
                         S_Respatialize, S_UpdateEntityPosition,
                         S_RegisterSound};
use self::cl_cgame_c::{botlib_export};
use self::libmumblelink_h::{mumble_link, mumble_islinked};
unsafe extern "C" fn _vmf(mut x: intptr_t) -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i = x as libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn CL_GameCommand() -> qboolean {
    if cgvm.is_null() { return qfalse }
    return VM_Call(cgvm, CG_CONSOLE_COMMAND as libc::c_int) as qboolean;
}
//
// cl_cgame.c
//
#[no_mangle]
pub unsafe extern "C" fn CL_InitCGame() {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapname: *const libc::c_char = 0 as *const libc::c_char;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut interpret: vmInterpret_t = VMI_NATIVE;
    t1 = Sys_Milliseconds();
    Con_Close();
    info =
        cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[0usize]
                                                        as isize);
    mapname =
        Info_ValueForKey(info,
                         b"mapname\x00" as *const u8 as *const libc::c_char);
    Com_sprintf(cl.mapname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
                mapname);
    interpret =
        Cvar_VariableValue(b"vm_cgame\x00" as *const u8 as
                               *const libc::c_char) as vmInterpret_t;
    if 0 != cl_connectedToPureServer {
        if interpret as libc::c_uint !=
               VMI_COMPILED as libc::c_int as libc::c_uint &&
               interpret as libc::c_uint !=
                   VMI_BYTECODE as libc::c_int as libc::c_uint {
            interpret = VMI_COMPILED
        }
    }
    cgvm =
        VM_Create(b"cgame\x00" as *const u8 as *const libc::c_char,
                  Some(CL_CgameSystemCalls), interpret);
    if cgvm.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"VM_Create on cgame failed\x00" as *const u8 as
                      *const libc::c_char);
    }
    clc.state = CA_LOADING;
    VM_Call(cgvm, CG_INIT as libc::c_int, clc.serverMessageSequence,
            clc.lastExecutedServerCommand, clc.clientNum);
    if 0 == clc.demoplaying as u64 && 0 == cl_connectedToCheatServer {
        Cvar_SetCheatState();
    }
    clc.state = CA_PRIMED;
    t2 = Sys_Milliseconds();
    Com_Printf(b"CL_InitCGame: %5.2f seconds\n\x00" as *const u8 as
                   *const libc::c_char,
               (t2 - t1) as libc::c_double / 1000.0f64);
    re.EndRegistration.expect("non-null function pointer")();
    if 0 == Sys_LowPhysicalMemory() as u64 { Com_TouchMemory(); }
    Con_ClearNotify();
}
/*
====================
CL_CgameSystemCalls

The cgame module is making a system call
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CgameSystemCalls(mut args: *mut intptr_t)
 -> intptr_t {
    match *args.offset(0isize) {
        0 => {
            Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       VM_ArgPtr(*args.offset(1isize)) as
                           *const libc::c_char);
            return 0i32 as intptr_t
        }
        1 => {
            Com_Error(ERR_DROP as libc::c_int,
                      b"%s\x00" as *const u8 as *const libc::c_char,
                      VM_ArgPtr(*args.offset(1isize)) as *const libc::c_char);
        }
        2 => { return Sys_Milliseconds() as intptr_t }
        3 => {
            Cvar_Register(VM_ArgPtr(*args.offset(1isize)) as *mut vmCvar_t,
                          VM_ArgPtr(*args.offset(2isize)) as
                              *const libc::c_char,
                          VM_ArgPtr(*args.offset(3isize)) as
                              *const libc::c_char,
                          *args.offset(4isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        4 => {
            Cvar_Update(VM_ArgPtr(*args.offset(1isize)) as *mut vmCvar_t);
            return 0i32 as intptr_t
        }
        5 => {
            Cvar_SetSafe(VM_ArgPtr(*args.offset(1isize)) as
                             *const libc::c_char,
                         VM_ArgPtr(*args.offset(2isize)) as
                             *const libc::c_char);
            return 0i32 as intptr_t
        }
        6 => {
            Cvar_VariableStringBuffer(VM_ArgPtr(*args.offset(1isize)) as
                                          *const libc::c_char,
                                      VM_ArgPtr(*args.offset(2isize)) as
                                          *mut libc::c_char,
                                      *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        7 => { return Cmd_Argc() as intptr_t }
        8 => {
            Cmd_ArgvBuffer(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *mut libc::c_char,
                           *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        9 => {
            Cmd_ArgsBuffer(VM_ArgPtr(*args.offset(1isize)) as
                               *mut libc::c_char,
                           *args.offset(2isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        10 => {
            return FS_FOpenFileByMode(VM_ArgPtr(*args.offset(1isize)) as
                                          *const libc::c_char,
                                      VM_ArgPtr(*args.offset(2isize)) as
                                          *mut fileHandle_t,
                                      *args.offset(3isize) as fsMode_t) as
                       intptr_t
        }
        11 => {
            FS_Read(VM_ArgPtr(*args.offset(1isize)),
                    *args.offset(2isize) as libc::c_int,
                    *args.offset(3isize) as fileHandle_t);
            return 0i32 as intptr_t
        }
        12 => {
            FS_Write(VM_ArgPtr(*args.offset(1isize)),
                     *args.offset(2isize) as libc::c_int,
                     *args.offset(3isize) as fileHandle_t);
            return 0i32 as intptr_t
        }
        13 => {
            FS_FCloseFile(*args.offset(1isize) as fileHandle_t);
            return 0i32 as intptr_t
        }
        89 => {
            return FS_Seek(*args.offset(1isize) as fileHandle_t,
                           *args.offset(2isize),
                           *args.offset(3isize) as libc::c_int) as intptr_t
        }
        14 => {
            Cbuf_AddText(VM_ArgPtr(*args.offset(1isize)) as
                             *const libc::c_char);
            return 0i32 as intptr_t
        }
        15 => {
            CL_AddCgameCommand(VM_ArgPtr(*args.offset(1isize)) as
                                   *const libc::c_char);
            return 0i32 as intptr_t
        }
        72 => {
            Cmd_RemoveCommandSafe(VM_ArgPtr(*args.offset(1isize)) as
                                      *const libc::c_char);
            return 0i32 as intptr_t
        }
        16 => {
            CL_AddReliableCommand(VM_ArgPtr(*args.offset(1isize)) as
                                      *const libc::c_char, qfalse);
            return 0i32 as intptr_t
        }
        17 => { SCR_UpdateScreen(); return 0i32 as intptr_t }
        18 => {
            CL_CM_LoadMap(VM_ArgPtr(*args.offset(1isize)) as
                              *const libc::c_char);
            return 0i32 as intptr_t
        }
        19 => { return CM_NumInlineModels() as intptr_t }
        20 => {
            return CM_InlineModel(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        22 => {
            return CM_TempBoxModel(VM_ArgPtr(*args.offset(1isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const vec_t, qfalse as libc::c_int) as
                       intptr_t
        }
        82 => {
            return CM_TempBoxModel(VM_ArgPtr(*args.offset(1isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const vec_t, qtrue as libc::c_int) as
                       intptr_t
        }
        23 => {
            return CM_PointContents(VM_ArgPtr(*args.offset(1isize)) as
                                        *const vec_t,
                                    *args.offset(2isize) as clipHandle_t) as
                       intptr_t
        }
        24 => {
            return CM_TransformedPointContents(VM_ArgPtr(*args.offset(1isize))
                                                   as *const vec_t,
                                               *args.offset(2isize) as
                                                   clipHandle_t,
                                               VM_ArgPtr(*args.offset(3isize))
                                                   as *const vec_t,
                                               VM_ArgPtr(*args.offset(4isize))
                                                   as *const vec_t) as
                       intptr_t
        }
        25 => {
            CM_BoxTrace(VM_ArgPtr(*args.offset(1isize)) as *mut trace_t,
                        VM_ArgPtr(*args.offset(2isize)) as *const vec_t,
                        VM_ArgPtr(*args.offset(3isize)) as *const vec_t,
                        VM_ArgPtr(*args.offset(4isize)) as *mut vec_t,
                        VM_ArgPtr(*args.offset(5isize)) as *mut vec_t,
                        *args.offset(6isize) as clipHandle_t,
                        *args.offset(7isize) as libc::c_int,
                        qfalse as libc::c_int);
            return 0i32 as intptr_t
        }
        83 => {
            CM_BoxTrace(VM_ArgPtr(*args.offset(1isize)) as *mut trace_t,
                        VM_ArgPtr(*args.offset(2isize)) as *const vec_t,
                        VM_ArgPtr(*args.offset(3isize)) as *const vec_t,
                        VM_ArgPtr(*args.offset(4isize)) as *mut vec_t,
                        VM_ArgPtr(*args.offset(5isize)) as *mut vec_t,
                        *args.offset(6isize) as clipHandle_t,
                        *args.offset(7isize) as libc::c_int,
                        qtrue as libc::c_int);
            return 0i32 as intptr_t
        }
        26 => {
            CM_TransformedBoxTrace(VM_ArgPtr(*args.offset(1isize)) as
                                       *mut trace_t,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(3isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(4isize)) as
                                       *mut vec_t,
                                   VM_ArgPtr(*args.offset(5isize)) as
                                       *mut vec_t,
                                   *args.offset(6isize) as clipHandle_t,
                                   *args.offset(7isize) as libc::c_int,
                                   VM_ArgPtr(*args.offset(8isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(9isize)) as
                                       *const vec_t, qfalse as libc::c_int);
            return 0i32 as intptr_t
        }
        84 => {
            CM_TransformedBoxTrace(VM_ArgPtr(*args.offset(1isize)) as
                                       *mut trace_t,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(3isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(4isize)) as
                                       *mut vec_t,
                                   VM_ArgPtr(*args.offset(5isize)) as
                                       *mut vec_t,
                                   *args.offset(6isize) as clipHandle_t,
                                   *args.offset(7isize) as libc::c_int,
                                   VM_ArgPtr(*args.offset(8isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(9isize)) as
                                       *const vec_t, qtrue as libc::c_int);
            return 0i32 as intptr_t
        }
        27 => {
            return re.MarkFragments.expect("non-null function pointer")(*args.offset(1isize)
                                                                            as
                                                                            libc::c_int,
                                                                        VM_ArgPtr(*args.offset(2isize))
                                                                            as
                                                                            *const vec3_t,
                                                                        VM_ArgPtr(*args.offset(3isize))
                                                                            as
                                                                            *const vec_t,
                                                                        *args.offset(4isize)
                                                                            as
                                                                            libc::c_int,
                                                                        VM_ArgPtr(*args.offset(5isize))
                                                                            as
                                                                            *mut vec_t,
                                                                        *args.offset(6isize)
                                                                            as
                                                                            libc::c_int,
                                                                        VM_ArgPtr(*args.offset(7isize))
                                                                            as
                                                                            *mut markFragment_t)
                       as intptr_t
        }
        28 => {
            S_StartSound(VM_ArgPtr(*args.offset(1isize)) as *mut vec_t,
                         *args.offset(2isize) as libc::c_int,
                         *args.offset(3isize) as libc::c_int,
                         *args.offset(4isize) as sfxHandle_t);
            return 0i32 as intptr_t
        }
        29 => {
            S_StartLocalSound(*args.offset(1isize) as sfxHandle_t,
                              *args.offset(2isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        30 => {
            S_ClearLoopingSounds(*args.offset(1isize) as qboolean);
            return 0i32 as intptr_t
        }
        31 => {
            S_AddLoopingSound(*args.offset(1isize) as libc::c_int,
                              VM_ArgPtr(*args.offset(2isize)) as *const vec_t,
                              VM_ArgPtr(*args.offset(3isize)) as *const vec_t,
                              *args.offset(4isize) as sfxHandle_t);
            return 0i32 as intptr_t
        }
        80 => {
            S_AddRealLoopingSound(*args.offset(1isize) as libc::c_int,
                                  VM_ArgPtr(*args.offset(2isize)) as
                                      *const vec_t,
                                  VM_ArgPtr(*args.offset(3isize)) as
                                      *const vec_t,
                                  *args.offset(4isize) as sfxHandle_t);
            return 0i32 as intptr_t
        }
        81 => {
            S_StopLoopingSound(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        32 => {
            S_UpdateEntityPosition(*args.offset(1isize) as libc::c_int,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const vec_t);
            return 0i32 as intptr_t
        }
        33 => {
            S_Respatialize(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as *const vec_t,
                           VM_ArgPtr(*args.offset(3isize)) as *mut vec3_t,
                           *args.offset(4isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        34 => {
            return S_RegisterSound(VM_ArgPtr(*args.offset(1isize)) as
                                       *const libc::c_char,
                                   *args.offset(2isize) as qboolean) as
                       intptr_t
        }
        35 => {
            S_StartBackgroundTrack(VM_ArgPtr(*args.offset(1isize)) as
                                       *const libc::c_char,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const libc::c_char);
            return 0i32 as intptr_t
        }
        36 => {
            re.LoadWorld.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                 as
                                                                 *const libc::c_char);
            return 0i32 as intptr_t
        }
        37 => {
            return re.RegisterModel.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                            as
                                                                            *const libc::c_char)
                       as intptr_t
        }
        38 => {
            return re.RegisterSkin.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                           as
                                                                           *const libc::c_char)
                       as intptr_t
        }
        39 => {
            return re.RegisterShader.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                             as
                                                                             *const libc::c_char)
                       as intptr_t
        }
        57 => {
            return re.RegisterShaderNoMip.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                  as
                                                                                  *const libc::c_char)
                       as intptr_t
        }
        59 => {
            re.RegisterFont.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                    as
                                                                    *const libc::c_char,
                                                                *args.offset(2isize)
                                                                    as
                                                                    libc::c_int,
                                                                VM_ArgPtr(*args.offset(3isize))
                                                                    as
                                                                    *mut fontInfo_t);
            return 0i32 as intptr_t
        }
        40 => {
            re.ClearScene.expect("non-null function pointer")();
            return 0i32 as intptr_t
        }
        41 => {
            re.AddRefEntityToScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                           as
                                                                           *const refEntity_t);
            return 0i32 as intptr_t
        }
        42 => {
            re.AddPolyToScene.expect("non-null function pointer")(*args.offset(1isize)
                                                                      as
                                                                      qhandle_t,
                                                                  *args.offset(2isize)
                                                                      as
                                                                      libc::c_int,
                                                                  VM_ArgPtr(*args.offset(3isize))
                                                                      as
                                                                      *const polyVert_t,
                                                                  1i32);
            return 0i32 as intptr_t
        }
        87 => {
            re.AddPolyToScene.expect("non-null function pointer")(*args.offset(1isize)
                                                                      as
                                                                      qhandle_t,
                                                                  *args.offset(2isize)
                                                                      as
                                                                      libc::c_int,
                                                                  VM_ArgPtr(*args.offset(3isize))
                                                                      as
                                                                      *const polyVert_t,
                                                                  *args.offset(4isize)
                                                                      as
                                                                      libc::c_int);
            return 0i32 as intptr_t
        }
        73 => {
            return re.LightForPoint.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                            as
                                                                            *mut vec_t,
                                                                        VM_ArgPtr(*args.offset(2isize))
                                                                            as
                                                                            *mut vec_t,
                                                                        VM_ArgPtr(*args.offset(3isize))
                                                                            as
                                                                            *mut vec_t,
                                                                        VM_ArgPtr(*args.offset(4isize))
                                                                            as
                                                                            *mut vec_t)
                       as intptr_t
        }
        43 => {
            re.AddLightToScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                       as
                                                                       *const vec_t,
                                                                   _vmf(*args.offset(2isize)),
                                                                   _vmf(*args.offset(3isize)),
                                                                   _vmf(*args.offset(4isize)),
                                                                   _vmf(*args.offset(5isize)));
            return 0i32 as intptr_t
        }
        85 => {
            re.AddAdditiveLightToScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                               as
                                                                               *const vec_t,
                                                                           _vmf(*args.offset(2isize)),
                                                                           _vmf(*args.offset(3isize)),
                                                                           _vmf(*args.offset(4isize)),
                                                                           _vmf(*args.offset(5isize)));
            return 0i32 as intptr_t
        }
        44 => {
            re.RenderScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                   as
                                                                   *const refdef_t);
            return 0i32 as intptr_t
        }
        45 => {
            re.SetColor.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                as
                                                                *const libc::c_float);
            return 0i32 as intptr_t
        }
        46 => {
            re.DrawStretchPic.expect("non-null function pointer")(_vmf(*args.offset(1isize)),
                                                                  _vmf(*args.offset(2isize)),
                                                                  _vmf(*args.offset(3isize)),
                                                                  _vmf(*args.offset(4isize)),
                                                                  _vmf(*args.offset(5isize)),
                                                                  _vmf(*args.offset(6isize)),
                                                                  _vmf(*args.offset(7isize)),
                                                                  _vmf(*args.offset(8isize)),
                                                                  *args.offset(9isize)
                                                                      as
                                                                      qhandle_t);
            return 0i32 as intptr_t
        }
        47 => {
            re.ModelBounds.expect("non-null function pointer")(*args.offset(1isize)
                                                                   as
                                                                   qhandle_t,
                                                               VM_ArgPtr(*args.offset(2isize))
                                                                   as
                                                                   *mut vec_t,
                                                               VM_ArgPtr(*args.offset(3isize))
                                                                   as
                                                                   *mut vec_t);
            return 0i32 as intptr_t
        }
        48 => {
            return re.LerpTag.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                      as
                                                                      *mut orientation_t,
                                                                  *args.offset(2isize)
                                                                      as
                                                                      qhandle_t,
                                                                  *args.offset(3isize)
                                                                      as
                                                                      libc::c_int,
                                                                  *args.offset(4isize)
                                                                      as
                                                                      libc::c_int,
                                                                  _vmf(*args.offset(5isize)),
                                                                  VM_ArgPtr(*args.offset(6isize))
                                                                      as
                                                                      *const libc::c_char)
                       as intptr_t
        }
        49 => {
            CL_GetGlconfig(VM_ArgPtr(*args.offset(1isize)) as
                               *mut glconfig_t);
            return 0i32 as intptr_t
        }
        50 => {
            CL_GetGameState(VM_ArgPtr(*args.offset(1isize)) as
                                *mut gameState_t);
            return 0i32 as intptr_t
        }
        51 => {
            CL_GetCurrentSnapshotNumber(VM_ArgPtr(*args.offset(1isize)) as
                                            *mut libc::c_int,
                                        VM_ArgPtr(*args.offset(2isize)) as
                                            *mut libc::c_int);
            return 0i32 as intptr_t
        }
        52 => {
            return CL_GetSnapshot(*args.offset(1isize) as libc::c_int,
                                  VM_ArgPtr(*args.offset(2isize)) as
                                      *mut snapshot_t) as intptr_t
        }
        53 => {
            return CL_GetServerCommand(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        54 => { return CL_GetCurrentCmdNumber() as intptr_t }
        55 => {
            return CL_GetUserCmd(*args.offset(1isize) as libc::c_int,
                                 VM_ArgPtr(*args.offset(2isize)) as
                                     *mut usercmd_t) as intptr_t
        }
        56 => {
            CL_SetUserCmdValue(*args.offset(1isize) as libc::c_int,
                               _vmf(*args.offset(2isize)));
            return 0i32 as intptr_t
        }
        58 => { return Hunk_MemoryRemaining() as intptr_t }
        60 => {
            return Key_IsDown(*args.offset(1isize) as libc::c_int) as intptr_t
        }
        61 => { return Key_GetCatcher() as intptr_t }
        62 => {
            Key_SetCatcher((*args.offset(1isize) |
                                (Key_GetCatcher() & 0x1i32) as libc::c_long)
                               as libc::c_int);
            return 0i32 as intptr_t
        }
        63 => {
            return Key_GetKey(VM_ArgPtr(*args.offset(1isize)) as
                                  *const libc::c_char) as intptr_t
        }
        100 => {
            memset(VM_ArgPtr(*args.offset(1isize)),
                   *args.offset(2isize) as libc::c_int,
                   *args.offset(3isize) as libc::c_ulong);
            return 0i32 as intptr_t
        }
        101 => {
            memcpy(VM_ArgPtr(*args.offset(1isize)),
                   VM_ArgPtr(*args.offset(2isize)),
                   *args.offset(3isize) as libc::c_ulong);
            return 0i32 as intptr_t
        }
        102 => {
            strncpy(VM_ArgPtr(*args.offset(1isize)) as *mut libc::c_char,
                    VM_ArgPtr(*args.offset(2isize)) as *const libc::c_char,
                    *args.offset(3isize) as libc::c_ulong);
            return *args.offset(1isize)
        }
        103 => {
            return FloatAsInt(sin(_vmf(*args.offset(1isize)) as
                                      libc::c_double) as libc::c_float) as
                       intptr_t
        }
        104 => {
            return FloatAsInt(cos(_vmf(*args.offset(1isize)) as
                                      libc::c_double) as libc::c_float) as
                       intptr_t
        }
        105 => {
            return FloatAsInt(atan2(_vmf(*args.offset(1isize)) as
                                        libc::c_double,
                                    _vmf(*args.offset(2isize)) as
                                        libc::c_double) as libc::c_float) as
                       intptr_t
        }
        106 => {
            return FloatAsInt(sqrt(_vmf(*args.offset(1isize)) as
                                       libc::c_double) as libc::c_float) as
                       intptr_t
        }
        107 => {
            return FloatAsInt(floor(_vmf(*args.offset(1isize)) as
                                        libc::c_double) as libc::c_float) as
                       intptr_t
        }
        108 => {
            return FloatAsInt(ceil(_vmf(*args.offset(1isize)) as
                                       libc::c_double) as libc::c_float) as
                       intptr_t
        }
        111 => {
            return FloatAsInt(Q_acos(_vmf(*args.offset(1isize)))) as intptr_t
        }
        64 => {
            return (*botlib_export).PC_AddGlobalDefine.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                               as
                                                                                               *mut libc::c_char)
                       as intptr_t
        }
        65 => {
            return (*botlib_export).PC_LoadSourceHandle.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                as
                                                                                                *const libc::c_char)
                       as intptr_t
        }
        66 => {
            return (*botlib_export).PC_FreeSourceHandle.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int)
                       as intptr_t
        }
        67 => {
            return (*botlib_export).PC_ReadTokenHandle.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut pc_token_t)
                       as intptr_t
        }
        68 => {
            return (*botlib_export).PC_SourceFileAndLine.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             VM_ArgPtr(*args.offset(2isize))
                                                                                                 as
                                                                                                 *mut libc::c_char,
                                                                                             VM_ArgPtr(*args.offset(3isize))
                                                                                                 as
                                                                                                 *mut libc::c_int)
                       as intptr_t
        }
        69 => { S_StopBackgroundTrack(); return 0i32 as intptr_t }
        70 => {
            return Com_RealTime(VM_ArgPtr(*args.offset(1isize)) as
                                    *mut qtime_t) as intptr_t
        }
        71 => {
            qsnapvectorsse(VM_ArgPtr(*args.offset(1isize)) as *mut vec_t);
            return 0i32 as intptr_t
        }
        74 => {
            return CIN_PlayCinematic(VM_ArgPtr(*args.offset(1isize)) as
                                         *const libc::c_char,
                                     *args.offset(2isize) as libc::c_int,
                                     *args.offset(3isize) as libc::c_int,
                                     *args.offset(4isize) as libc::c_int,
                                     *args.offset(5isize) as libc::c_int,
                                     *args.offset(6isize) as libc::c_int) as
                       intptr_t
        }
        75 => {
            return CIN_StopCinematic(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        76 => {
            return CIN_RunCinematic(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        77 => {
            CIN_DrawCinematic(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        78 => {
            CIN_SetExtents(*args.offset(1isize) as libc::c_int,
                           *args.offset(2isize) as libc::c_int,
                           *args.offset(3isize) as libc::c_int,
                           *args.offset(4isize) as libc::c_int,
                           *args.offset(5isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        79 => {
            re.RemapShader.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                   as
                                                                   *const libc::c_char,
                                                               VM_ArgPtr(*args.offset(2isize))
                                                                   as
                                                                   *const libc::c_char,
                                                               VM_ArgPtr(*args.offset(3isize))
                                                                   as
                                                                   *const libc::c_char);
            return 0i32 as intptr_t
        }
        86 => {
            return re.GetEntityToken.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                             as
                                                                             *mut libc::c_char,
                                                                         *args.offset(2isize)
                                                                             as
                                                                             libc::c_int)
                       as intptr_t
        }
        88 => {
            return re.inPVS.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                    as
                                                                    *const vec_t,
                                                                VM_ArgPtr(*args.offset(2isize))
                                                                    as
                                                                    *const vec_t)
                       as intptr_t
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"code/client/cl_cgame.c\x00" as *const u8 as
                              *const libc::c_char, 697i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 41],
                                                    &[libc::c_char; 41]>(b"intptr_t CL_CgameSystemCalls(intptr_t *)\x00")).as_ptr());
            Com_Error(ERR_DROP as libc::c_int,
                      b"Bad cgame system trap: %ld\x00" as *const u8 as
                          *const libc::c_char, *args.offset(0isize));
        }
    };
}
unsafe extern "C" fn FloatAsInt(mut f: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.f = f;
    return fi.i;
}
/*
=====================
CL_SetUserCmdValue
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetUserCmdValue(mut userCmdValue: libc::c_int,
                                            mut sensitivityScale:
                                                libc::c_float) {
    cl.cgameUserCmdValue = userCmdValue;
    cl.cgameSensitivity = sensitivityScale;
}
/*
====================
CL_GetUserCmd
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetUserCmd(mut cmdNumber: libc::c_int,
                                       mut ucmd: *mut usercmd_t) -> qboolean {
    if cmdNumber > cl.cmdNumber {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_GetUserCmd: %i >= %i\x00" as *const u8 as
                      *const libc::c_char, cmdNumber, cl.cmdNumber);
    }
    if cmdNumber <= cl.cmdNumber - 64i32 { return qfalse }
    *ucmd = cl.cmds[(cmdNumber & 64i32 - 1i32) as usize];
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetCurrentCmdNumber() -> libc::c_int {
    return cl.cmdNumber;
}
/*
===================
CL_GetServerCommand

Set up argc/argv for the given command
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetServerCommand(mut serverCommandNumber:
                                                 libc::c_int) -> qboolean {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut bigConfigString: [libc::c_char; 8192] = [0; 8192];
    let mut argc: libc::c_int = 0;
    if serverCommandNumber <= clc.serverCommandSequence - 64i32 {
        if 0 != clc.demoplaying as u64 { return qfalse }
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_GetServerCommand: a reliable command was cycled out\x00"
                      as *const u8 as *const libc::c_char);
    }
    if serverCommandNumber > clc.serverCommandSequence {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_GetServerCommand: requested a command not received\x00"
                      as *const u8 as *const libc::c_char);
    }
    s =
        clc.serverCommands[(serverCommandNumber & 64i32 - 1i32) as
                               usize].as_mut_ptr();
    clc.lastExecutedServerCommand = serverCommandNumber;
    Com_DPrintf(b"serverCommand: %i : %s\n\x00" as *const u8 as
                    *const libc::c_char, serverCommandNumber, s);
    loop  {
        Cmd_TokenizeString(s);
        cmd = Cmd_Argv(0i32);
        argc = Cmd_Argc();
        if 0 ==
               strcmp(cmd,
                      b"disconnect\x00" as *const u8 as *const libc::c_char) {
            if argc >= 2i32 {
                Com_Error(ERR_SERVERDISCONNECT as libc::c_int,
                          b"Server disconnected - %s\x00" as *const u8 as
                              *const libc::c_char, Cmd_Argv(1i32));
            } else {
                Com_Error(ERR_SERVERDISCONNECT as libc::c_int,
                          b"Server disconnected\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        if 0 == strcmp(cmd, b"bcs0\x00" as *const u8 as *const libc::c_char) {
            Com_sprintf(bigConfigString.as_mut_ptr(), 8192i32,
                        b"cs %s \"%s\x00" as *const u8 as *const libc::c_char,
                        Cmd_Argv(1i32), Cmd_Argv(2i32));
            return qfalse
        }
        if 0 == strcmp(cmd, b"bcs1\x00" as *const u8 as *const libc::c_char) {
            s = Cmd_Argv(2i32);
            if strlen(bigConfigString.as_mut_ptr()).wrapping_add(strlen(s)) >=
                   8192i32 as libc::c_ulong {
                Com_Error(ERR_DROP as libc::c_int,
                          b"bcs exceeded BIG_INFO_STRING\x00" as *const u8 as
                              *const libc::c_char);
            }
            strcat(bigConfigString.as_mut_ptr(), s);
            return qfalse
        }
        if !(0 ==
                 strcmp(cmd, b"bcs2\x00" as *const u8 as *const libc::c_char))
           {
            break ;
        }
        s = Cmd_Argv(2i32);
        if strlen(bigConfigString.as_mut_ptr()).wrapping_add(strlen(s)).wrapping_add(1i32
                                                                                         as
                                                                                         libc::c_ulong)
               >= 8192i32 as libc::c_ulong {
            Com_Error(ERR_DROP as libc::c_int,
                      b"bcs exceeded BIG_INFO_STRING\x00" as *const u8 as
                          *const libc::c_char);
        }
        strcat(bigConfigString.as_mut_ptr(), s);
        strcat(bigConfigString.as_mut_ptr(),
               b"\"\x00" as *const u8 as *const libc::c_char);
        s = bigConfigString.as_mut_ptr()
    }
    if 0 == strcmp(cmd, b"cs\x00" as *const u8 as *const libc::c_char) {
        CL_ConfigstringModified();
        Cmd_TokenizeString(s);
        return qtrue
    }
    if 0 ==
           strcmp(cmd, b"map_restart\x00" as *const u8 as *const libc::c_char)
       {
        Con_ClearNotify();
        Cmd_TokenizeString(s);
        memset(cl.cmds.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[usercmd_t; 64]>() as libc::c_ulong);
        return qtrue
    }
    if 0 ==
           strcmp(cmd,
                  b"clientLevelShot\x00" as *const u8 as *const libc::c_char)
       {
        if 0 == (*com_sv_running).integer { return qfalse }
        Con_Close();
        Cbuf_AddText(b"wait ; wait ; wait ; wait ; screenshot levelshot\n\x00"
                         as *const u8 as *const libc::c_char);
        return qtrue
    }
    return qtrue;
}
/*
=====================
CL_ConfigstringModified
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ConfigstringModified() {
    let mut old: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut dup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldGs: gameState_t =
        gameState_t{stringOffsets: [0; 1024],
                    stringData: [0; 16000],
                    dataCount: 0,};
    let mut len: libc::c_int = 0;
    index = atoi(Cmd_Argv(1i32));
    if index < 0i32 || index >= 1024i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_ConfigstringModified: bad index %i\x00" as *const u8 as
                      *const libc::c_char, index);
    }
    s = Cmd_ArgsFrom(2i32);
    old =
        cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[index
                                                                                   as
                                                                                   usize]
                                                        as isize);
    if 0 == strcmp(old, s) { return }
    oldGs = cl.gameState;
    memset(&mut cl.gameState as *mut gameState_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<gameState_t>() as libc::c_ulong);
    cl.gameState.dataCount = 1i32;
    i = 0i32;
    while i < 1024i32 {
        if i == index {
            dup = s
        } else {
            dup =
                oldGs.stringData.as_mut_ptr().offset(oldGs.stringOffsets[i as
                                                                             usize]
                                                         as isize)
        }
        if !(0 == *dup.offset(0isize)) {
            // leave with the default empty string
            len = strlen(dup) as libc::c_int;
            if len + 1i32 + cl.gameState.dataCount > 16000i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"MAX_GAMESTATE_CHARS exceeded\x00" as *const u8 as
                              *const libc::c_char);
            }
            cl.gameState.stringOffsets[i as usize] = cl.gameState.dataCount;
            memcpy(cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.dataCount
                                                                   as isize)
                       as *mut libc::c_void, dup as *const libc::c_void,
                   (len + 1i32) as libc::c_ulong);
            cl.gameState.dataCount += len + 1i32
        }
        i += 1
    }
    if index == 1i32 { CL_SystemInfoChanged(); };
}
/*
====================
CL_GetSnapshot
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetSnapshot(mut snapshotNumber: libc::c_int,
                                        mut snapshot: *mut snapshot_t)
 -> qboolean {
    let mut clSnap: *mut clSnapshot_t = 0 as *mut clSnapshot_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if snapshotNumber > cl.snap.messageNum {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_GetSnapshot: snapshotNumber > cl.snapshot.messageNum\x00"
                      as *const u8 as *const libc::c_char);
    }
    if cl.snap.messageNum - snapshotNumber >= 32i32 { return qfalse }
    clSnap =
        &mut *cl.snapshots.as_mut_ptr().offset((snapshotNumber & 32i32 - 1i32)
                                                   as isize) as
            *mut clSnapshot_t;
    if 0 == (*clSnap).valid as u64 { return qfalse }
    if cl.parseEntitiesNum - (*clSnap).parseEntitiesNum >= 32i32 * 256i32 {
        return qfalse
    }
    (*snapshot).snapFlags = (*clSnap).snapFlags;
    (*snapshot).serverCommandSequence = (*clSnap).serverCommandNum;
    (*snapshot).ping = (*clSnap).ping;
    (*snapshot).serverTime = (*clSnap).serverTime;
    memcpy((*snapshot).areamask.as_mut_ptr() as *mut libc::c_void,
           (*clSnap).areamask.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[byte; 32]>() as libc::c_ulong);
    (*snapshot).ps = (*clSnap).ps;
    count = (*clSnap).numEntities;
    if count > 256i32 {
        Com_DPrintf(b"CL_GetSnapshot: truncated %i entities to %i\n\x00" as
                        *const u8 as *const libc::c_char, count, 256i32);
        count = 256i32
    }
    (*snapshot).numEntities = count;
    i = 0i32;
    while i < count {
        (*snapshot).entities[i as usize] =
            cl.parseEntities[((*clSnap).parseEntitiesNum + i &
                                  32i32 * 256i32 - 1i32) as usize];
        i += 1
    }
    return qtrue;
}
/*
====================
CL_GetCurrentSnapshotNumber
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetCurrentSnapshotNumber(mut snapshotNumber:
                                                         *mut libc::c_int,
                                                     mut serverTime:
                                                         *mut libc::c_int) {
    *snapshotNumber = cl.snap.messageNum;
    *serverTime = cl.snap.serverTime;
}
/*
====================
CL_GetGameState
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetGameState(mut gs: *mut gameState_t) {
    *gs = cl.gameState;
}
/*
====================
CL_GetGlconfig
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetGlconfig(mut glconfig: *mut glconfig_t) {
    *glconfig = cls.glconfig;
}
/*
====================
CL_CM_LoadMap

Just adds default parameters that cgame doesn't need to know about
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CM_LoadMap(mut mapname: *const libc::c_char) {
    let mut checksum: libc::c_int = 0;
    CM_LoadMap(mapname, qtrue, &mut checksum);
}
/*
=====================
CL_AddCgameCommand
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddCgameCommand(mut cmdName:
                                                *const libc::c_char) {
    Cmd_AddCommand(cmdName, None);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ShutdownCGame() {
    Key_SetCatcher(Key_GetCatcher() & !0x8i32);
    cls.cgameStarted = qfalse;
    if cgvm.is_null() { return }
    VM_Call(cgvm, CG_SHUTDOWN as libc::c_int);
    VM_Free(cgvm);
    cgvm = 0 as *mut vm_t;
}
#[no_mangle]
pub unsafe extern "C" fn CL_CGameRendering(mut stereo: stereoFrame_t) {
    VM_Call(cgvm, CG_DRAW_ACTIVE_FRAME as libc::c_int, cl.serverTime,
            stereo as libc::c_uint, clc.demoplaying as libc::c_uint);
    VM_Debug(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn CL_SetCGameTime() {
    if clc.state as libc::c_uint != CA_ACTIVE as libc::c_int as libc::c_uint {
        if clc.state as libc::c_uint !=
               CA_PRIMED as libc::c_int as libc::c_uint {
            return
        }
        if 0 != clc.demoplaying as u64 {
            if 0 == clc.firstDemoFrameSkipped as u64 {
                clc.firstDemoFrameSkipped = qtrue;
                return
            }
            CL_ReadDemoMessage();
        }
        if 0 != cl.newSnapshots as u64 {
            cl.newSnapshots = qfalse;
            CL_FirstSnapshot();
        }
        if clc.state as libc::c_uint !=
               CA_ACTIVE as libc::c_int as libc::c_uint {
            return
        }
    }
    if 0 == cl.snap.valid as u64 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_SetCGameTime: !cl.snap.valid\x00" as *const u8 as
                      *const libc::c_char);
    }
    if 0 != (*sv_paused).integer && 0 != CL_CheckPaused() as libc::c_uint &&
           0 != (*com_sv_running).integer {
        return
    }
    if cl.snap.serverTime < cl.oldFrameServerTime {
        Com_Error(ERR_DROP as libc::c_int,
                  b"cl.snap.serverTime < cl.oldFrameServerTime\x00" as
                      *const u8 as *const libc::c_char);
    }
    cl.oldFrameServerTime = cl.snap.serverTime;
    if !(0 != clc.demoplaying as libc::c_uint &&
             0 != (*cl_freezeDemo).integer) {
        let mut tn: libc::c_int = 0;
        tn = (*cl_timeNudge).integer;
        if tn < -30i32 { tn = -30i32 } else if tn > 30i32 { tn = 30i32 }
        cl.serverTime = cls.realtime + cl.serverTimeDelta - tn;
        if cl.serverTime < cl.oldServerTime {
            cl.serverTime = cl.oldServerTime
        }
        cl.oldServerTime = cl.serverTime;
        if cls.realtime + cl.serverTimeDelta >= cl.snap.serverTime - 5i32 {
            cl.extrapolatedSnapshot = qtrue
        }
    }
    if 0 != cl.newSnapshots as u64 { CL_AdjustTimeDelta(); }
    if 0 == clc.demoplaying as u64 { return }
    if 0 != (*cl_timedemo).integer {
        let mut now: libc::c_int = Sys_Milliseconds();
        let mut frameDuration: libc::c_int = 0;
        if 0 == clc.timeDemoStart {
            clc.timeDemoLastFrame = now;
            clc.timeDemoStart = clc.timeDemoLastFrame;
            clc.timeDemoMinDuration = 2147483647i32;
            clc.timeDemoMaxDuration = 0i32
        }
        frameDuration = now - clc.timeDemoLastFrame;
        clc.timeDemoLastFrame = now;
        if clc.timeDemoFrames > 0i32 {
            if frameDuration > clc.timeDemoMaxDuration {
                clc.timeDemoMaxDuration = frameDuration
            }
            if frameDuration < clc.timeDemoMinDuration {
                clc.timeDemoMinDuration = frameDuration
            }
            if frameDuration > 127i32 * 2i32 + 1i32 {
                frameDuration = 127i32 * 2i32 + 1i32
            }
            clc.timeDemoDurations[((clc.timeDemoFrames - 1i32) % 4096i32) as
                                      usize] = frameDuration as libc::c_uchar
        }
        clc.timeDemoFrames += 1;
        cl.serverTime = clc.timeDemoBaseTime + clc.timeDemoFrames * 50i32
    }
    while cl.serverTime >= cl.snap.serverTime {
        CL_ReadDemoMessage();
        if clc.state as libc::c_uint !=
               CA_ACTIVE as libc::c_int as libc::c_uint {
            return
        }
    };
}
/*
=================
CL_AdjustTimeDelta

Adjust the clients view of server time.

We attempt to have cl.serverTime exactly equal the server's view
of time plus the timeNudge, but with variable latencies over
the internet it will often need to drift a bit to match conditions.

Our ideal time would be to have the adjusted time approach, but not pass,
the very latest snapshot.

Adjustments are only made when a new snapshot arrives with a rational
latency, which keeps the adjustment process framerate independent and
prevents massive overadjustment during times of significant packet loss
or bursted delayed packets.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AdjustTimeDelta() {
    let mut newDelta: libc::c_int = 0;
    let mut deltaDelta: libc::c_int = 0;
    cl.newSnapshots = qfalse;
    if 0 != clc.demoplaying as u64 { return }
    newDelta = cl.snap.serverTime - cls.realtime;
    deltaDelta = abs(newDelta - cl.serverTimeDelta);
    if deltaDelta > 500i32 {
        cl.serverTimeDelta = newDelta;
        cl.oldServerTime = cl.snap.serverTime;
        cl.serverTime = cl.snap.serverTime;
        if 0 != (*cl_showTimeDelta).integer {
            Com_Printf(b"<RESET> \x00" as *const u8 as *const libc::c_char);
        }
    } else if deltaDelta > 100i32 {
        if 0 != (*cl_showTimeDelta).integer {
            Com_Printf(b"<FAST> \x00" as *const u8 as *const libc::c_char);
        }
        cl.serverTimeDelta = cl.serverTimeDelta + newDelta >> 1i32
    } else if (*com_timescale).value == 0i32 as libc::c_float ||
                  (*com_timescale).value == 1i32 as libc::c_float {
        if 0 != cl.extrapolatedSnapshot as u64 {
            cl.extrapolatedSnapshot = qfalse;
            cl.serverTimeDelta -= 2i32
        } else { cl.serverTimeDelta += 1 }
    }
    if 0 != (*cl_showTimeDelta).integer {
        Com_Printf(b"%i \x00" as *const u8 as *const libc::c_char,
                   cl.serverTimeDelta);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_FirstSnapshot() {
    if 0 != cl.snap.snapFlags & 2i32 { return }
    clc.state = CA_ACTIVE;
    cl.serverTimeDelta = cl.snap.serverTime - cls.realtime;
    cl.oldServerTime = cl.snap.serverTime;
    clc.timeDemoBaseTime = cl.snap.serverTime;
    if 0 != *(*cl_activeAction).string.offset(0isize) {
        Cbuf_AddText((*cl_activeAction).string);
        Cvar_Set(b"activeAction\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*cl_useMumble).integer && 0 == mumble_islinked() {
        let mut ret: libc::c_int =
            mumble_link(b"ioquake3\x00" as *const u8 as *const libc::c_char);
        Com_Printf(b"Mumble: Linking to Mumble application %s\n\x00" as
                       *const u8 as *const libc::c_char,
                   if ret == 0i32 {
                       b"ok\x00" as *const u8 as *const libc::c_char
                   } else {
                       b"failed\x00" as *const u8 as *const libc::c_char
                   });
    }
    if 0 == clc.voipCodecInitialized as u64 {
        let mut i: libc::c_int = 0;
        let mut error: libc::c_int = 0;
        clc.opusEncoder =
            opus_encoder_create(48000i32, 1i32, 2048i32, &mut error);
        if 0 != error {
            Com_DPrintf(b"VoIP: Error opus_encoder_create %d\n\x00" as
                            *const u8 as *const libc::c_char, error);
            return
        }
        i = 0i32;
        while i < 64i32 {
            clc.opusDecoder[i as usize] =
                opus_decoder_create(48000i32, 1i32, &mut error);
            if 0 != error {
                Com_DPrintf(b"VoIP: Error opus_decoder_create(%d) %d\n\x00" as
                                *const u8 as *const libc::c_char, i, error);
                return
            }
            clc.voipIgnore[i as usize] = qfalse;
            clc.voipGain[i as usize] = 1.0f32;
            i += 1
        }
        clc.voipCodecInitialized = qtrue;
        clc.voipMuteAll = qfalse;
        Cmd_AddCommand(b"voip\x00" as *const u8 as *const libc::c_char,
                       Some(CL_Voip_f));
        Cvar_Set(b"cl_voipSendTarget\x00" as *const u8 as *const libc::c_char,
                 b"spatial\x00" as *const u8 as *const libc::c_char);
        memset(clc.voipTargets.as_mut_ptr() as *mut libc::c_void, !0i32,
               ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong);
    };
}
/*
====================
CL_GetParseEntityState
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetParseEntityState(mut parseEntityNumber:
                                                    libc::c_int,
                                                mut state: *mut entityState_t)
 -> qboolean {
    if parseEntityNumber >= cl.parseEntitiesNum {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_GetParseEntityState: %i >= %i\x00" as *const u8 as
                      *const libc::c_char, parseEntityNumber,
                  cl.parseEntitiesNum);
    }
    if parseEntityNumber <= cl.parseEntitiesNum - 32i32 * 256i32 {
        return qfalse
    }
    *state =
        cl.parseEntities[(parseEntityNumber & 32i32 * 256i32 - 1i32) as
                             usize];
    return qtrue;
}