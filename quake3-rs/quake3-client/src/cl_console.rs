#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, extern_types, libc)]
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
    pub type qhandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    pub type vec4_t = [vec_t; 4];
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub static mut g_color_table: [vec4_t; 8];
        #[no_mangle]
        pub fn COM_CompareExtension(in_0: *const libc::c_char,
                                    ext: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn COM_DefaultExtension(path: *mut libc::c_char,
                                    maxSize: libc::c_int,
                                    extension: *const libc::c_char);
        #[no_mangle]
        pub fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                         n: libc::c_int) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
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
    pub type completionFunc_t
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                   -> ()>;
    /*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct field_t {
        pub cursor: libc::c_int,
        pub scroll: libc::c_int,
        pub widthInChars: libc::c_int,
        pub buffer: [libc::c_char; 256],
    }
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, cvar_t, fileHandle_t};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        // called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
        #[no_mangle]
        pub fn Cmd_RemoveCommand(cmd_name: *const libc::c_char);
        // callback with each valid string
        #[no_mangle]
        pub fn Cmd_SetCommandCompletionFunc(command: *const libc::c_char,
                                            complete: completionFunc_t);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
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
        pub fn FS_FOpenFileWrite(qpath: *const libc::c_char) -> fileHandle_t;
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn Field_Clear(edit: *mut field_t);
        #[no_mangle]
        pub fn Field_CompleteFilename(dir: *const libc::c_char,
                                      ext: *const libc::c_char,
                                      stripExt: qboolean,
                                      allowNonPureFilesOnDisk: qboolean);
        #[no_mangle]
        pub static mut com_cl_running: *mut cvar_t;
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
        #[no_mangle]
        pub fn CL_KeyEvent(key: libc::c_int, down: qboolean,
                           time: libc::c_uint);
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
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
        /*
=============================================================================

the clientActive_t structure is wiped completely at every
new gamestate_t, potentially several times during an established connection

=============================================================================
*/
        /*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
        //=============================================================================
        // interface to cgame dll or vm
        // interface to refresh .dll
        /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
        // this is for the GL_EXT_texture_compression_s3tc extension.
        // this is for the GL_S3_s3tc extension.
        // where you don't have src*dst
        // where you can't modulate alpha on alpha textures
        // the hardware type then there can NOT exist a secondary
							// display adapter
        // where you can't interpolate alpha
        // Voodoo Banshee or Voodoo3, relevant since if this is
        // where everything works the way it should
        // driver is a 3Dfx standalone driver
        // WARNING: there are tests that check for
								// > GLDRV_ICD for minidriverness, so this
								// should always be the lowest value in this
								// enum set
        // driver is a non-3Dfx standalone driver
        // driver is integrated with window system
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
        // doesn't draw anything, just info for portals
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
// console.c
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
        //
// these are the key numbers that should be passed to KeyEvent
//
        // normal keys should be passed as lowercased ascii
        // Pseudo-key that brings the console down
        // Gamepad controls
	// Ordered to match SDL2 game controller buttons and axes
	// Do not change this order without also changing IN_GamepadMove() in SDL_input.c
        /*
==================================================================

functions exported to the main executable

==================================================================
*/
        //	void	(*CG_MouseEvent)( int dx, int dy );
        //	void	(*CG_KeyEvent)( int key, qboolean down );
        //	int (*CG_LastAttacker)( void );
        //	int (*CG_CrosshairPlayer)( void );
        //	void (*CG_DrawActiveFrame)( int serverTime, stereoFrame_t stereoView, qboolean demoPlayback );
	// Generates and draws a game scene and status information at the given time.
	// If demoPlayback is set, local movement prediction will not be enabled
        //	qboolean (*CG_ConsoleCommand)( void );
	// a console command has been issued locally that is not recognized by the
	// main game system.
	// use Cmd_Argc() / Cmd_Argv() to read the command, return qfalse if the
	// command is not known to the game
        //	void (*CG_Shutdown)( void );
	// opportunity to flush and close any open files
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct outPacket_t {
        //	void CG_Init( int serverMessageNum, int serverCommandSequence, int clientNum )
	// called when the level loads or when the renderer is restarted
	// all media should be registered at this time
	// cgame will display loading status by calling SCR_Update, which
	// will call CG_DrawInformation during the loading process
	// reliableCommandSequence will be 0 on fresh loads, but higher for
	// demos, tourney restarts, or vid_restarts
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientConnection_t {
        /*
===================================================================================

PMOVE MODULE

The pmove code takes a player_state_t and a usercmd_t and generates a new player_state_t
and some other output data.  Used for local prediction on the client game and true
movement on the server game.
===================================================================================
*/
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
    use super::q_shared_h::{qboolean, qhandle_t, byte, playerState_t,
                            gameState_t, usercmd_t, vec3_t, entityState_t,
                            connstate_t, fileHandle_t, cvar_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t};
    use super::tr_types_h::{glconfig_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_public_h::{refexport_t};
    extern "C" {
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        #[no_mangle]
        pub static mut cl_noprint: *mut cvar_t;
        #[no_mangle]
        pub static mut cl: clientActive_t;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut cgvm: *mut vm_t;
        #[no_mangle]
        pub static mut re: refexport_t;
        #[no_mangle]
        pub static mut cl_conXOffset: *mut cvar_t;
        #[no_mangle]
        pub fn Key_GetCatcher() -> libc::c_int;
        #[no_mangle]
        pub fn Key_SetCatcher(catcher: libc::c_int);
        #[no_mangle]
        pub fn CL_LoadConsoleHistory();
        #[no_mangle]
        pub fn SCR_DrawBigString(x: libc::c_int, y: libc::c_int,
                                 s: *const libc::c_char, alpha: libc::c_float,
                                 noColorEscape: qboolean);
        #[no_mangle]
        pub fn SCR_DrawSmallChar(x: libc::c_int, y: libc::c_int,
                                 ch: libc::c_int);
        #[no_mangle]
        pub fn SCR_FillRect(x: libc::c_float, y: libc::c_float,
                            width: libc::c_float, height: libc::c_float,
                            color: *const libc::c_float);
        #[no_mangle]
        pub fn SCR_DrawPic(x: libc::c_float, y: libc::c_float,
                           width: libc::c_float, height: libc::c_float,
                           hShader: qhandle_t);
        #[no_mangle]
        pub fn SCR_AdjustFrom640(x: *mut libc::c_float, y: *mut libc::c_float,
                                 w: *mut libc::c_float,
                                 h: *mut libc::c_float);
    }
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
    pub type textureCompression_t = libc::c_uint;
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    pub const TC_S3TC: textureCompression_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    pub type glDriverType_t = libc::c_uint;
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    pub const GLDRV_ICD: glDriverType_t = 0;
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
    use super::{libc};
    use super::q_shared_h::{qboolean, vec3_t, byte, qhandle_t};
}
#[header_src =
      "ioq3/code/client/cl_console.c"]
pub mod cl_console_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct console_t {
        pub initialized: qboolean,
        pub text: [libc::c_short; 32768],
        pub current: libc::c_int,
        pub x: libc::c_int,
        pub display: libc::c_int,
        pub linewidth: libc::c_int,
        pub totallines: libc::c_int,
        pub xadjust: libc::c_float,
        pub displayFrac: libc::c_float,
        pub finalFrac: libc::c_float,
        pub vislines: libc::c_int,
        pub times: [libc::c_int; 4],
        pub color: vec4_t,
    }
    use super::q_shared_h::{qboolean, vec4_t, cvar_t};
    use super::{libc};
}
#[header_src =
      "ioq3/code/renderercommon/tr_public.h"]
pub mod tr_public_h {
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
#[header_src =
      "ioq3/code/client/keycodes.h"]
pub mod keycodes_h {
    pub type unnamed = libc::c_uint;
    pub const MAX_KEYS: unnamed = 366;
    pub const K_CONSOLE: unnamed = 365;
    pub const K_PAD0_RIGHTTRIGGER: unnamed = 364;
    pub const K_PAD0_LEFTTRIGGER: unnamed = 363;
    pub const K_PAD0_RIGHTSTICK_DOWN: unnamed = 362;
    pub const K_PAD0_RIGHTSTICK_UP: unnamed = 361;
    pub const K_PAD0_RIGHTSTICK_RIGHT: unnamed = 360;
    pub const K_PAD0_RIGHTSTICK_LEFT: unnamed = 359;
    pub const K_PAD0_LEFTSTICK_DOWN: unnamed = 358;
    pub const K_PAD0_LEFTSTICK_UP: unnamed = 357;
    pub const K_PAD0_LEFTSTICK_RIGHT: unnamed = 356;
    pub const K_PAD0_LEFTSTICK_LEFT: unnamed = 355;
    pub const K_PAD0_DPAD_RIGHT: unnamed = 354;
    pub const K_PAD0_DPAD_LEFT: unnamed = 353;
    pub const K_PAD0_DPAD_DOWN: unnamed = 352;
    pub const K_PAD0_DPAD_UP: unnamed = 351;
    pub const K_PAD0_RIGHTSHOULDER: unnamed = 350;
    pub const K_PAD0_LEFTSHOULDER: unnamed = 349;
    pub const K_PAD0_RIGHTSTICK_CLICK: unnamed = 348;
    pub const K_PAD0_LEFTSTICK_CLICK: unnamed = 347;
    pub const K_PAD0_START: unnamed = 346;
    pub const K_PAD0_GUIDE: unnamed = 345;
    pub const K_PAD0_BACK: unnamed = 344;
    pub const K_PAD0_Y: unnamed = 343;
    pub const K_PAD0_X: unnamed = 342;
    pub const K_PAD0_B: unnamed = 341;
    pub const K_PAD0_A: unnamed = 340;
    pub const K_UNDO: unnamed = 339;
    pub const K_EURO: unnamed = 338;
    pub const K_MENU: unnamed = 337;
    pub const K_BREAK: unnamed = 336;
    pub const K_SCROLLOCK: unnamed = 335;
    pub const K_SYSREQ: unnamed = 334;
    pub const K_PRINT: unnamed = 333;
    pub const K_HELP: unnamed = 332;
    pub const K_MODE: unnamed = 331;
    pub const K_COMPOSE: unnamed = 330;
    pub const K_SUPER: unnamed = 329;
    pub const K_WORLD_95: unnamed = 328;
    pub const K_WORLD_94: unnamed = 327;
    pub const K_WORLD_93: unnamed = 326;
    pub const K_WORLD_92: unnamed = 325;
    pub const K_WORLD_91: unnamed = 324;
    pub const K_WORLD_90: unnamed = 323;
    pub const K_WORLD_89: unnamed = 322;
    pub const K_WORLD_88: unnamed = 321;
    pub const K_WORLD_87: unnamed = 320;
    pub const K_WORLD_86: unnamed = 319;
    pub const K_WORLD_85: unnamed = 318;
    pub const K_WORLD_84: unnamed = 317;
    pub const K_WORLD_83: unnamed = 316;
    pub const K_WORLD_82: unnamed = 315;
    pub const K_WORLD_81: unnamed = 314;
    pub const K_WORLD_80: unnamed = 313;
    pub const K_WORLD_79: unnamed = 312;
    pub const K_WORLD_78: unnamed = 311;
    pub const K_WORLD_77: unnamed = 310;
    pub const K_WORLD_76: unnamed = 309;
    pub const K_WORLD_75: unnamed = 308;
    pub const K_WORLD_74: unnamed = 307;
    pub const K_WORLD_73: unnamed = 306;
    pub const K_WORLD_72: unnamed = 305;
    pub const K_WORLD_71: unnamed = 304;
    pub const K_WORLD_70: unnamed = 303;
    pub const K_WORLD_69: unnamed = 302;
    pub const K_WORLD_68: unnamed = 301;
    pub const K_WORLD_67: unnamed = 300;
    pub const K_WORLD_66: unnamed = 299;
    pub const K_WORLD_65: unnamed = 298;
    pub const K_WORLD_64: unnamed = 297;
    pub const K_WORLD_63: unnamed = 296;
    pub const K_WORLD_62: unnamed = 295;
    pub const K_WORLD_61: unnamed = 294;
    pub const K_WORLD_60: unnamed = 293;
    pub const K_WORLD_59: unnamed = 292;
    pub const K_WORLD_58: unnamed = 291;
    pub const K_WORLD_57: unnamed = 290;
    pub const K_WORLD_56: unnamed = 289;
    pub const K_WORLD_55: unnamed = 288;
    pub const K_WORLD_54: unnamed = 287;
    pub const K_WORLD_53: unnamed = 286;
    pub const K_WORLD_52: unnamed = 285;
    pub const K_WORLD_51: unnamed = 284;
    pub const K_WORLD_50: unnamed = 283;
    pub const K_WORLD_49: unnamed = 282;
    pub const K_WORLD_48: unnamed = 281;
    pub const K_WORLD_47: unnamed = 280;
    pub const K_WORLD_46: unnamed = 279;
    pub const K_WORLD_45: unnamed = 278;
    pub const K_WORLD_44: unnamed = 277;
    pub const K_WORLD_43: unnamed = 276;
    pub const K_WORLD_42: unnamed = 275;
    pub const K_WORLD_41: unnamed = 274;
    pub const K_WORLD_40: unnamed = 273;
    pub const K_WORLD_39: unnamed = 272;
    pub const K_WORLD_38: unnamed = 271;
    pub const K_WORLD_37: unnamed = 270;
    pub const K_WORLD_36: unnamed = 269;
    pub const K_WORLD_35: unnamed = 268;
    pub const K_WORLD_34: unnamed = 267;
    pub const K_WORLD_33: unnamed = 266;
    pub const K_WORLD_32: unnamed = 265;
    pub const K_WORLD_31: unnamed = 264;
    pub const K_WORLD_30: unnamed = 263;
    pub const K_WORLD_29: unnamed = 262;
    pub const K_WORLD_28: unnamed = 261;
    pub const K_WORLD_27: unnamed = 260;
    pub const K_WORLD_26: unnamed = 259;
    pub const K_WORLD_25: unnamed = 258;
    pub const K_WORLD_24: unnamed = 257;
    pub const K_WORLD_23: unnamed = 256;
    pub const K_WORLD_22: unnamed = 255;
    pub const K_WORLD_21: unnamed = 254;
    pub const K_WORLD_20: unnamed = 253;
    pub const K_WORLD_19: unnamed = 252;
    pub const K_WORLD_18: unnamed = 251;
    pub const K_WORLD_17: unnamed = 250;
    pub const K_WORLD_16: unnamed = 249;
    pub const K_WORLD_15: unnamed = 248;
    pub const K_WORLD_14: unnamed = 247;
    pub const K_WORLD_13: unnamed = 246;
    pub const K_WORLD_12: unnamed = 245;
    pub const K_WORLD_11: unnamed = 244;
    pub const K_WORLD_10: unnamed = 243;
    pub const K_WORLD_9: unnamed = 242;
    pub const K_WORLD_8: unnamed = 241;
    pub const K_WORLD_7: unnamed = 240;
    pub const K_WORLD_6: unnamed = 239;
    pub const K_WORLD_5: unnamed = 238;
    pub const K_WORLD_4: unnamed = 237;
    pub const K_WORLD_3: unnamed = 236;
    pub const K_WORLD_2: unnamed = 235;
    pub const K_WORLD_1: unnamed = 234;
    pub const K_WORLD_0: unnamed = 233;
    pub const K_AUX16: unnamed = 232;
    pub const K_AUX15: unnamed = 231;
    pub const K_AUX14: unnamed = 230;
    pub const K_AUX13: unnamed = 229;
    pub const K_AUX12: unnamed = 228;
    pub const K_AUX11: unnamed = 227;
    pub const K_AUX10: unnamed = 226;
    pub const K_AUX9: unnamed = 225;
    pub const K_AUX8: unnamed = 224;
    pub const K_AUX7: unnamed = 223;
    pub const K_AUX6: unnamed = 222;
    pub const K_AUX5: unnamed = 221;
    pub const K_AUX4: unnamed = 220;
    pub const K_AUX3: unnamed = 219;
    pub const K_AUX2: unnamed = 218;
    pub const K_AUX1: unnamed = 217;
    pub const K_JOY32: unnamed = 216;
    pub const K_JOY31: unnamed = 215;
    pub const K_JOY30: unnamed = 214;
    pub const K_JOY29: unnamed = 213;
    pub const K_JOY28: unnamed = 212;
    pub const K_JOY27: unnamed = 211;
    pub const K_JOY26: unnamed = 210;
    pub const K_JOY25: unnamed = 209;
    pub const K_JOY24: unnamed = 208;
    pub const K_JOY23: unnamed = 207;
    pub const K_JOY22: unnamed = 206;
    pub const K_JOY21: unnamed = 205;
    pub const K_JOY20: unnamed = 204;
    pub const K_JOY19: unnamed = 203;
    pub const K_JOY18: unnamed = 202;
    pub const K_JOY17: unnamed = 201;
    pub const K_JOY16: unnamed = 200;
    pub const K_JOY15: unnamed = 199;
    pub const K_JOY14: unnamed = 198;
    pub const K_JOY13: unnamed = 197;
    pub const K_JOY12: unnamed = 196;
    pub const K_JOY11: unnamed = 195;
    pub const K_JOY10: unnamed = 194;
    pub const K_JOY9: unnamed = 193;
    pub const K_JOY8: unnamed = 192;
    pub const K_JOY7: unnamed = 191;
    pub const K_JOY6: unnamed = 190;
    pub const K_JOY5: unnamed = 189;
    pub const K_JOY4: unnamed = 188;
    pub const K_JOY3: unnamed = 187;
    pub const K_JOY2: unnamed = 186;
    pub const K_JOY1: unnamed = 185;
    pub const K_MWHEELUP: unnamed = 184;
    pub const K_MWHEELDOWN: unnamed = 183;
    pub const K_MOUSE5: unnamed = 182;
    pub const K_MOUSE4: unnamed = 181;
    pub const K_MOUSE3: unnamed = 180;
    pub const K_MOUSE2: unnamed = 179;
    pub const K_MOUSE1: unnamed = 178;
    pub const K_KP_EQUALS: unnamed = 177;
    pub const K_KP_STAR: unnamed = 176;
    pub const K_KP_NUMLOCK: unnamed = 175;
    pub const K_KP_PLUS: unnamed = 174;
    pub const K_KP_MINUS: unnamed = 173;
    pub const K_KP_SLASH: unnamed = 172;
    pub const K_KP_DEL: unnamed = 171;
    pub const K_KP_INS: unnamed = 170;
    pub const K_KP_ENTER: unnamed = 169;
    pub const K_KP_PGDN: unnamed = 168;
    pub const K_KP_DOWNARROW: unnamed = 167;
    pub const K_KP_END: unnamed = 166;
    pub const K_KP_RIGHTARROW: unnamed = 165;
    pub const K_KP_5: unnamed = 164;
    pub const K_KP_LEFTARROW: unnamed = 163;
    pub const K_KP_PGUP: unnamed = 162;
    pub const K_KP_UPARROW: unnamed = 161;
    pub const K_KP_HOME: unnamed = 160;
    pub const K_F15: unnamed = 159;
    pub const K_F14: unnamed = 158;
    pub const K_F13: unnamed = 157;
    pub const K_F12: unnamed = 156;
    pub const K_F11: unnamed = 155;
    pub const K_F10: unnamed = 154;
    pub const K_F9: unnamed = 153;
    pub const K_F8: unnamed = 152;
    pub const K_F7: unnamed = 151;
    pub const K_F6: unnamed = 150;
    pub const K_F5: unnamed = 149;
    pub const K_F4: unnamed = 148;
    pub const K_F3: unnamed = 147;
    pub const K_F2: unnamed = 146;
    pub const K_F1: unnamed = 145;
    pub const K_END: unnamed = 144;
    pub const K_HOME: unnamed = 143;
    pub const K_PGUP: unnamed = 142;
    pub const K_PGDN: unnamed = 141;
    pub const K_DEL: unnamed = 140;
    pub const K_INS: unnamed = 139;
    pub const K_SHIFT: unnamed = 138;
    pub const K_CTRL: unnamed = 137;
    pub const K_ALT: unnamed = 136;
    pub const K_RIGHTARROW: unnamed = 135;
    pub const K_LEFTARROW: unnamed = 134;
    pub const K_DOWNARROW: unnamed = 133;
    pub const K_UPARROW: unnamed = 132;
    pub const K_PAUSE: unnamed = 131;
    pub const K_POWER: unnamed = 130;
    pub const K_CAPSLOCK: unnamed = 129;
    pub const K_COMMAND: unnamed = 128;
    pub const K_BACKSPACE: unnamed = 127;
    pub const K_SPACE: unnamed = 32;
    pub const K_ESCAPE: unnamed = 27;
    pub const K_ENTER: unnamed = 13;
    pub const K_TAB: unnamed = 9;
    use super::{libc};
}
#[header_src =
      "ioq3/code/cgame/cg_public.h"]
pub mod cg_public_h {
    pub type unnamed_0 = libc::c_uint;
    pub const CG_EVENT_HANDLING: unnamed_0 = 8;
    pub const CG_MOUSE_EVENT: unnamed_0 = 7;
    pub const CG_KEY_EVENT: unnamed_0 = 6;
    pub const CG_LAST_ATTACKER: unnamed_0 = 5;
    pub const CG_CROSSHAIR_PLAYER: unnamed_0 = 4;
    pub const CG_DRAW_ACTIVE_FRAME: unnamed_0 = 3;
    pub const CG_CONSOLE_COMMAND: unnamed_0 = 2;
    pub const CG_SHUTDOWN: unnamed_0 = 1;
    pub const CG_INIT: unnamed_0 = 0;
    use super::{libc};
}
#[header_src =
      "ioq3/code/game/bg_public.h"]
pub mod bg_public_h {
    pub type unnamed_1 = libc::c_uint;
    // no movement or status bar
    pub const PM_SPINTERMISSION: unnamed_1 = 6;
    // no movement or status bar
    pub const PM_INTERMISSION: unnamed_1 = 5;
    // stuck in place with no control
    pub const PM_FREEZE: unnamed_1 = 4;
    // no acceleration or turning, but free falling
    pub const PM_DEAD: unnamed_1 = 3;
    // still run into walls
    pub const PM_SPECTATOR: unnamed_1 = 2;
    // noclip movement
    pub const PM_NOCLIP: unnamed_1 = 1;
    // can accelerate and turn
    pub const PM_NORMAL: unnamed_1 = 0;
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
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/client/keys.h"]
pub mod keys_h {
    use super::qcommon_h::{field_t};
    use super::{libc};
    use super::q_shared_h::{qboolean};
    extern "C" {
        #[no_mangle]
        pub fn Field_Draw(edit: *mut field_t, x: libc::c_int, y: libc::c_int,
                          width: libc::c_int, showCursor: qboolean,
                          noColorEscape: qboolean);
        #[no_mangle]
        pub fn Field_BigDraw(edit: *mut field_t, x: libc::c_int,
                             y: libc::c_int, width: libc::c_int,
                             showCursor: qboolean, noColorEscape: qboolean);
        #[no_mangle]
        pub static mut historyEditLines: [field_t; 32];
        #[no_mangle]
        pub static mut g_consoleField: field_t;
        #[no_mangle]
        pub static mut chatField: field_t;
        #[no_mangle]
        pub static mut chat_team: qboolean;
        #[no_mangle]
        pub static mut chat_playerNum: libc::c_int;
    }
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       vec_t, vec3_t, vec4_t, cvar_s, cvar_t, markFragment_t,
                       orientation_t, gameState_t, playerState_s,
                       playerState_t, usercmd_s, usercmd_t, trType_t,
                       TR_GRAVITY, TR_SINE, TR_LINEAR_STOP, TR_LINEAR,
                       TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, connstate_t,
                       CA_CINEMATIC, CA_ACTIVE, CA_PRIMED, CA_LOADING,
                       CA_CONNECTED, CA_CHALLENGING, CA_CONNECTING,
                       CA_AUTHORIZING, CA_DISCONNECTED, CA_UNINITIALIZED,
                       glyphInfo_t, fontInfo_t, Q_IsColorString,
                       g_color_table, COM_CompareExtension,
                       COM_DefaultExtension, Q_strncmp, Q_strncpyz, Q_strcat,
                       Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t, vm_t,
                      xcommand_t, completionFunc_t, field_t, vm_s, VM_Call,
                      Cmd_AddCommand, Cmd_RemoveCommand,
                      Cmd_SetCommandCompletionFunc, Cmd_Argc, Cmd_Argv,
                      Cvar_Get, FS_FOpenFileWrite, FS_Write, FS_FCloseFile,
                      Field_Clear, Field_CompleteFilename, com_cl_running,
                      Hunk_AllocateTempMemory, Hunk_FreeTempMemory,
                      CL_KeyEvent, Sys_Milliseconds};
use self::client_h::{clientStatic_t, serverInfo_t, clSnapshot_t, outPacket_t,
                     clientActive_t, clientConnection_t, cls, cl_noprint, cl,
                     clc, cgvm, re, cl_conXOffset, Key_GetCatcher,
                     Key_SetCatcher, CL_LoadConsoleHistory, SCR_DrawBigString,
                     SCR_DrawSmallChar, SCR_FillRect, SCR_DrawPic,
                     SCR_AdjustFrom640};
use self::tr_types_h::{glconfig_t, textureCompression_t, TC_S3TC_ARB, TC_S3TC,
                       TC_NONE, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glDriverType_t, GLDRV_VOODOO,
                       GLDRV_STANDALONE, GLDRV_ICD, polyVert_t,
                       refEntityType_t, RT_MAX_REF_ENTITY_TYPE,
                       RT_PORTALSURFACE, RT_LIGHTNING, RT_RAIL_RINGS,
                       RT_RAIL_CORE, RT_BEAM, RT_SPRITE, RT_POLY, RT_MODEL,
                       refEntity_t, refdef_t, stereoFrame_t, STEREO_RIGHT,
                       STEREO_LEFT, STEREO_CENTER};
use self::cl_console_c::{console_t};
use self::tr_public_h::{refexport_t};
use self::keycodes_h::{unnamed, MAX_KEYS, K_CONSOLE, K_PAD0_RIGHTTRIGGER,
                       K_PAD0_LEFTTRIGGER, K_PAD0_RIGHTSTICK_DOWN,
                       K_PAD0_RIGHTSTICK_UP, K_PAD0_RIGHTSTICK_RIGHT,
                       K_PAD0_RIGHTSTICK_LEFT, K_PAD0_LEFTSTICK_DOWN,
                       K_PAD0_LEFTSTICK_UP, K_PAD0_LEFTSTICK_RIGHT,
                       K_PAD0_LEFTSTICK_LEFT, K_PAD0_DPAD_RIGHT,
                       K_PAD0_DPAD_LEFT, K_PAD0_DPAD_DOWN, K_PAD0_DPAD_UP,
                       K_PAD0_RIGHTSHOULDER, K_PAD0_LEFTSHOULDER,
                       K_PAD0_RIGHTSTICK_CLICK, K_PAD0_LEFTSTICK_CLICK,
                       K_PAD0_START, K_PAD0_GUIDE, K_PAD0_BACK, K_PAD0_Y,
                       K_PAD0_X, K_PAD0_B, K_PAD0_A, K_UNDO, K_EURO, K_MENU,
                       K_BREAK, K_SCROLLOCK, K_SYSREQ, K_PRINT, K_HELP,
                       K_MODE, K_COMPOSE, K_SUPER, K_WORLD_95, K_WORLD_94,
                       K_WORLD_93, K_WORLD_92, K_WORLD_91, K_WORLD_90,
                       K_WORLD_89, K_WORLD_88, K_WORLD_87, K_WORLD_86,
                       K_WORLD_85, K_WORLD_84, K_WORLD_83, K_WORLD_82,
                       K_WORLD_81, K_WORLD_80, K_WORLD_79, K_WORLD_78,
                       K_WORLD_77, K_WORLD_76, K_WORLD_75, K_WORLD_74,
                       K_WORLD_73, K_WORLD_72, K_WORLD_71, K_WORLD_70,
                       K_WORLD_69, K_WORLD_68, K_WORLD_67, K_WORLD_66,
                       K_WORLD_65, K_WORLD_64, K_WORLD_63, K_WORLD_62,
                       K_WORLD_61, K_WORLD_60, K_WORLD_59, K_WORLD_58,
                       K_WORLD_57, K_WORLD_56, K_WORLD_55, K_WORLD_54,
                       K_WORLD_53, K_WORLD_52, K_WORLD_51, K_WORLD_50,
                       K_WORLD_49, K_WORLD_48, K_WORLD_47, K_WORLD_46,
                       K_WORLD_45, K_WORLD_44, K_WORLD_43, K_WORLD_42,
                       K_WORLD_41, K_WORLD_40, K_WORLD_39, K_WORLD_38,
                       K_WORLD_37, K_WORLD_36, K_WORLD_35, K_WORLD_34,
                       K_WORLD_33, K_WORLD_32, K_WORLD_31, K_WORLD_30,
                       K_WORLD_29, K_WORLD_28, K_WORLD_27, K_WORLD_26,
                       K_WORLD_25, K_WORLD_24, K_WORLD_23, K_WORLD_22,
                       K_WORLD_21, K_WORLD_20, K_WORLD_19, K_WORLD_18,
                       K_WORLD_17, K_WORLD_16, K_WORLD_15, K_WORLD_14,
                       K_WORLD_13, K_WORLD_12, K_WORLD_11, K_WORLD_10,
                       K_WORLD_9, K_WORLD_8, K_WORLD_7, K_WORLD_6, K_WORLD_5,
                       K_WORLD_4, K_WORLD_3, K_WORLD_2, K_WORLD_1, K_WORLD_0,
                       K_AUX16, K_AUX15, K_AUX14, K_AUX13, K_AUX12, K_AUX11,
                       K_AUX10, K_AUX9, K_AUX8, K_AUX7, K_AUX6, K_AUX5,
                       K_AUX4, K_AUX3, K_AUX2, K_AUX1, K_JOY32, K_JOY31,
                       K_JOY30, K_JOY29, K_JOY28, K_JOY27, K_JOY26, K_JOY25,
                       K_JOY24, K_JOY23, K_JOY22, K_JOY21, K_JOY20, K_JOY19,
                       K_JOY18, K_JOY17, K_JOY16, K_JOY15, K_JOY14, K_JOY13,
                       K_JOY12, K_JOY11, K_JOY10, K_JOY9, K_JOY8, K_JOY7,
                       K_JOY6, K_JOY5, K_JOY4, K_JOY3, K_JOY2, K_JOY1,
                       K_MWHEELUP, K_MWHEELDOWN, K_MOUSE5, K_MOUSE4, K_MOUSE3,
                       K_MOUSE2, K_MOUSE1, K_KP_EQUALS, K_KP_STAR,
                       K_KP_NUMLOCK, K_KP_PLUS, K_KP_MINUS, K_KP_SLASH,
                       K_KP_DEL, K_KP_INS, K_KP_ENTER, K_KP_PGDN,
                       K_KP_DOWNARROW, K_KP_END, K_KP_RIGHTARROW, K_KP_5,
                       K_KP_LEFTARROW, K_KP_PGUP, K_KP_UPARROW, K_KP_HOME,
                       K_F15, K_F14, K_F13, K_F12, K_F11, K_F10, K_F9, K_F8,
                       K_F7, K_F6, K_F5, K_F4, K_F3, K_F2, K_F1, K_END,
                       K_HOME, K_PGUP, K_PGDN, K_DEL, K_INS, K_SHIFT, K_CTRL,
                       K_ALT, K_RIGHTARROW, K_LEFTARROW, K_DOWNARROW,
                       K_UPARROW, K_PAUSE, K_POWER, K_CAPSLOCK, K_COMMAND,
                       K_BACKSPACE, K_SPACE, K_ESCAPE, K_ENTER, K_TAB};
use self::cg_public_h::{unnamed_0, CG_EVENT_HANDLING, CG_MOUSE_EVENT,
                        CG_KEY_EVENT, CG_LAST_ATTACKER, CG_CROSSHAIR_PLAYER,
                        CG_DRAW_ACTIVE_FRAME, CG_CONSOLE_COMMAND, CG_SHUTDOWN,
                        CG_INIT};
use self::bg_public_h::{unnamed_1, PM_SPINTERMISSION, PM_INTERMISSION,
                        PM_FREEZE, PM_DEAD, PM_SPECTATOR, PM_NOCLIP,
                        PM_NORMAL};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::string_h::{memcpy, strlen};
use self::keys_h::{Field_Draw, Field_BigDraw, historyEditLines,
                   g_consoleField, chatField, chat_team, chat_playerNum};
#[no_mangle]
pub unsafe extern "C" fn CL_ConsolePrint(mut txt: *mut libc::c_char) {
    let mut y: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut color: libc::c_ushort = 0;
    // NERVE - SMF
    let mut skipnotify: qboolean = qfalse;
    // NERVE - SMF
    let mut prev: libc::c_int = 0;
    if 0 ==
           Q_strncmp(txt,
                     b"[skipnotify]\x00" as *const u8 as *const libc::c_char,
                     12i32) {
        skipnotify = qtrue;
        txt = txt.offset(12isize)
    }
    if !cl_noprint.is_null() && 0 != (*cl_noprint).integer { return }
    if 0 == con.initialized as u64 {
        con.color[3usize] = 1.0f32;
        con.color[2usize] = con.color[3usize];
        con.color[1usize] = con.color[2usize];
        con.color[0usize] = con.color[1usize];
        con.linewidth = -1i32;
        Con_CheckResize();
        con.initialized = qtrue
    }
    color = ('7' as i32 - '0' as i32 & 0x7i32) as libc::c_ushort;
    loop  {
        c = *(txt as *mut libc::c_uchar);
        if !(c as libc::c_int != 0i32) { break ; }
        if 0 != Q_IsColorString(txt) as u64 {
            color =
                (*txt.offset(1isize) as libc::c_int - '0' as i32 & 0x7i32) as
                    libc::c_ushort;
            txt = txt.offset(2isize)
        } else {
            l = 0i32;
            while l < con.linewidth {
                if *txt.offset(l as isize) as libc::c_int <= ' ' as i32 {
                    break ;
                }
                l += 1
            }
            if l != con.linewidth && con.x + l >= con.linewidth {
                Con_Linefeed(skipnotify);
            }
            txt = txt.offset(1isize);
            match c as libc::c_int {
                10 => { Con_Linefeed(skipnotify); }
                13 => { con.x = 0i32 }
                _ => {
                    y = con.current % con.totallines;
                    con.text[(y * con.linewidth + con.x) as usize] =
                        ((color as libc::c_int) << 8i32 | c as libc::c_int) as
                            libc::c_short;
                    con.x += 1;
                    if con.x >= con.linewidth { Con_Linefeed(skipnotify); }
                }
            }
        }
    }
    if con.current >= 0i32 {
        if 0 != skipnotify as u64 {
            prev = con.current % 4i32 - 1i32;
            if prev < 0i32 { prev = 4i32 - 1i32 }
            con.times[prev as usize] = 0i32
        } else { con.times[(con.current % 4i32) as usize] = cls.realtime }
    };
}
#[no_mangle]
pub static mut con: console_t =
    console_t{initialized: qfalse,
              text: [0; 32768],
              current: 0,
              x: 0,
              display: 0,
              linewidth: 0,
              totallines: 0,
              xadjust: 0.,
              displayFrac: 0.,
              finalFrac: 0.,
              vislines: 0,
              times: [0; 4],
              color: [0.; 4],};
/*
===============
Con_Linefeed
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Linefeed(mut skipnotify: qboolean) {
    let mut i: libc::c_int = 0;
    if con.current >= 0i32 {
        if 0 != skipnotify as u64 {
            con.times[(con.current % 4i32) as usize] = 0i32
        } else { con.times[(con.current % 4i32) as usize] = cls.realtime }
    }
    con.x = 0i32;
    if con.display == con.current { con.display += 1 }
    con.current += 1;
    i = 0i32;
    while i < con.linewidth {
        con.text[(con.current % con.totallines * con.linewidth + i) as usize]
            =
            (('7' as i32 - '0' as i32 & 0x7i32) << 8i32 | ' ' as i32) as
                libc::c_short;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Con_CheckResize() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut oldwidth: libc::c_int = 0;
    let mut oldtotallines: libc::c_int = 0;
    let mut numlines: libc::c_int = 0;
    let mut numchars: libc::c_int = 0;
    let mut tbuf: [libc::c_short; 32768] = [0; 32768];
    width = 640i32 / 8i32 - 2i32;
    if width == con.linewidth { return }
    if width < 1i32 {
        width = 78i32;
        con.linewidth = width;
        con.totallines = 32768i32 / con.linewidth;
        i = 0i32;
        while i < 32768i32 {
            con.text[i as usize] =
                (('7' as i32 - '0' as i32 & 0x7i32) << 8i32 | ' ' as i32) as
                    libc::c_short;
            i += 1
        }
    } else {
        oldwidth = con.linewidth;
        con.linewidth = width;
        oldtotallines = con.totallines;
        con.totallines = 32768i32 / con.linewidth;
        numlines = oldtotallines;
        if con.totallines < numlines { numlines = con.totallines }
        numchars = oldwidth;
        if con.linewidth < numchars { numchars = con.linewidth }
        memcpy(tbuf.as_mut_ptr() as *mut libc::c_void,
               con.text.as_mut_ptr() as *const libc::c_void,
               (32768i32 as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                    as libc::c_ulong));
        i = 0i32;
        while i < 32768i32 {
            con.text[i as usize] =
                (('7' as i32 - '0' as i32 & 0x7i32) << 8i32 | ' ' as i32) as
                    libc::c_short;
            i += 1
        }
        i = 0i32;
        while i < numlines {
            j = 0i32;
            while j < numchars {
                con.text[((con.totallines - 1i32 - i) * con.linewidth + j) as
                             usize] =
                    tbuf[((con.current - i + oldtotallines) % oldtotallines *
                              oldwidth + j) as usize];
                j += 1
            }
            i += 1
        }
        Con_ClearNotify();
    }
    con.current = con.totallines - 1i32;
    con.display = con.current;
}
#[no_mangle]
pub unsafe extern "C" fn Con_ClearNotify() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 4i32 { con.times[i as usize] = 0i32; i += 1 };
}
// the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original 
#[no_mangle]
pub static mut g_console_field_width: libc::c_int = 78i32;
#[no_mangle]
pub unsafe extern "C" fn Con_Init() {
    let mut i: libc::c_int = 0;
    con_notifytime =
        Cvar_Get(b"con_notifytime\x00" as *const u8 as *const libc::c_char,
                 b"3\x00" as *const u8 as *const libc::c_char, 0i32);
    con_conspeed =
        Cvar_Get(b"scr_conspeed\x00" as *const u8 as *const libc::c_char,
                 b"3\x00" as *const u8 as *const libc::c_char, 0i32);
    con_autoclear =
        Cvar_Get(b"con_autoclear\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Field_Clear(&mut g_consoleField);
    g_consoleField.widthInChars = g_console_field_width;
    i = 0i32;
    while i < 32i32 {
        Field_Clear(&mut *historyEditLines.as_mut_ptr().offset(i as isize));
        historyEditLines[i as usize].widthInChars = g_console_field_width;
        i += 1
    }
    CL_LoadConsoleHistory();
    Cmd_AddCommand(b"toggleconsole\x00" as *const u8 as *const libc::c_char,
                   Some(Con_ToggleConsole_f));
    Cmd_AddCommand(b"togglemenu\x00" as *const u8 as *const libc::c_char,
                   Some(Con_ToggleMenu_f));
    Cmd_AddCommand(b"messagemode\x00" as *const u8 as *const libc::c_char,
                   Some(Con_MessageMode_f));
    Cmd_AddCommand(b"messagemode2\x00" as *const u8 as *const libc::c_char,
                   Some(Con_MessageMode2_f));
    Cmd_AddCommand(b"messagemode3\x00" as *const u8 as *const libc::c_char,
                   Some(Con_MessageMode3_f));
    Cmd_AddCommand(b"messagemode4\x00" as *const u8 as *const libc::c_char,
                   Some(Con_MessageMode4_f));
    Cmd_AddCommand(b"clear\x00" as *const u8 as *const libc::c_char,
                   Some(Con_Clear_f));
    Cmd_AddCommand(b"condump\x00" as *const u8 as *const libc::c_char,
                   Some(Con_Dump_f));
    Cmd_SetCommandCompletionFunc(b"condump\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cmd_CompleteTxtName));
}
/*
==================
Cmd_CompleteTxtName
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_CompleteTxtName(mut args: *mut libc::c_char,
                                             mut argNum: libc::c_int) {
    if argNum == 2i32 {
        Field_CompleteFilename(b"\x00" as *const u8 as *const libc::c_char,
                               b"txt\x00" as *const u8 as *const libc::c_char,
                               qfalse, qtrue);
    };
}
/*
================
Con_Dump_f

Save the console contents out to a file
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Dump_f() {
    let mut l: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut line: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut f: fileHandle_t = 0;
    let mut bufferlen: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: [libc::c_char; 64] = [0; 64];
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"usage: condump <filename>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Q_strncpyz(filename.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    COM_DefaultExtension(filename.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 64]>() as
                             libc::c_ulong as libc::c_int,
                         b".txt\x00" as *const u8 as *const libc::c_char);
    if 0 ==
           COM_CompareExtension(filename.as_mut_ptr(),
                                b".txt\x00" as *const u8 as
                                    *const libc::c_char) as u64 {
        Com_Printf(b"Con_Dump_f: Only the \".txt\" extension is supported by this command!\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    f = FS_FOpenFileWrite(filename.as_mut_ptr());
    if 0 == f {
        Com_Printf(b"ERROR: couldn\'t open %s.\n\x00" as *const u8 as
                       *const libc::c_char, filename.as_mut_ptr());
        return
    }
    Com_Printf(b"Dumped console text to %s.\n\x00" as *const u8 as
                   *const libc::c_char, filename.as_mut_ptr());
    l = con.current - con.totallines + 1i32;
    while l <= con.current {
        line =
            con.text.as_mut_ptr().offset((l % con.totallines * con.linewidth)
                                             as isize);
        x = 0i32;
        while x < con.linewidth {
            if *line.offset(x as isize) as libc::c_int & 0xffi32 != ' ' as i32
               {
                break ;
            }
            x += 1
        }
        if x != con.linewidth { break ; }
        l += 1
    }
    bufferlen =
        (con.linewidth as
             libc::c_ulong).wrapping_add((2i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int;
    buffer = Hunk_AllocateTempMemory(bufferlen) as *mut libc::c_char;
    *buffer.offset((bufferlen - 1i32) as isize) = 0i32 as libc::c_char;
    while l <= con.current {
        line =
            con.text.as_mut_ptr().offset((l % con.totallines * con.linewidth)
                                             as isize);
        i = 0i32;
        while i < con.linewidth {
            *buffer.offset(i as isize) =
                (*line.offset(i as isize) as libc::c_int & 0xffi32) as
                    libc::c_char;
            i += 1
        }
        x = con.linewidth - 1i32;
        while x >= 0i32 {
            if !(*buffer.offset(x as isize) as libc::c_int == ' ' as i32) {
                break ;
            }
            *buffer.offset(x as isize) = 0i32 as libc::c_char;
            x -= 1
        }
        Q_strcat(buffer, bufferlen,
                 b"\n\x00" as *const u8 as *const libc::c_char);
        FS_Write(buffer as *const libc::c_void, strlen(buffer) as libc::c_int,
                 f);
        l += 1
    }
    Hunk_FreeTempMemory(buffer as *mut libc::c_void);
    FS_FCloseFile(f);
}
#[no_mangle]
pub unsafe extern "C" fn Con_Clear_f() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 32768i32 {
        con.text[i as usize] =
            (('7' as i32 - '0' as i32 & 0x7i32) << 8i32 | ' ' as i32) as
                libc::c_short;
        i += 1
    }
    Con_Bottom();
}
#[no_mangle]
pub unsafe extern "C" fn Con_Bottom() { con.display = con.current; }
/*
================
Con_MessageMode4_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode4_f() {
    chat_playerNum =
        VM_Call(cgvm, CG_LAST_ATTACKER as libc::c_int) as libc::c_int;
    if chat_playerNum < 0i32 || chat_playerNum >= 64i32 {
        chat_playerNum = -1i32;
        return
    }
    chat_team = qfalse;
    Field_Clear(&mut chatField);
    chatField.widthInChars = 30i32;
    Key_SetCatcher(Key_GetCatcher() ^ 0x4i32);
}
/*
================
Con_MessageMode3_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode3_f() {
    chat_playerNum =
        VM_Call(cgvm, CG_CROSSHAIR_PLAYER as libc::c_int) as libc::c_int;
    if chat_playerNum < 0i32 || chat_playerNum >= 64i32 {
        chat_playerNum = -1i32;
        return
    }
    chat_team = qfalse;
    Field_Clear(&mut chatField);
    chatField.widthInChars = 30i32;
    Key_SetCatcher(Key_GetCatcher() ^ 0x4i32);
}
/*
================
Con_MessageMode2_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode2_f() {
    chat_playerNum = -1i32;
    chat_team = qtrue;
    Field_Clear(&mut chatField);
    chatField.widthInChars = 25i32;
    Key_SetCatcher(Key_GetCatcher() ^ 0x4i32);
}
/*
================
Con_MessageMode_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode_f() {
    chat_playerNum = -1i32;
    chat_team = qfalse;
    Field_Clear(&mut chatField);
    chatField.widthInChars = 30i32;
    Key_SetCatcher(Key_GetCatcher() ^ 0x4i32);
}
/*
===================
Con_ToggleMenu_f
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_ToggleMenu_f() {
    CL_KeyEvent(K_ESCAPE as libc::c_int, qtrue,
                Sys_Milliseconds() as libc::c_uint);
    CL_KeyEvent(K_ESCAPE as libc::c_int, qfalse,
                Sys_Milliseconds() as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn Con_ToggleConsole_f() {
    if clc.state as libc::c_uint ==
           CA_DISCONNECTED as libc::c_int as libc::c_uint &&
           Key_GetCatcher() == 0x1i32 {
        return
    }
    if 0 != (*con_autoclear).integer { Field_Clear(&mut g_consoleField); }
    g_consoleField.widthInChars = g_console_field_width;
    Con_ClearNotify();
    Key_SetCatcher(Key_GetCatcher() ^ 0x1i32);
}
#[no_mangle]
pub static mut con_autoclear: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut con_conspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut con_notifytime: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn Con_Shutdown() {
    Cmd_RemoveCommand(b"toggleconsole\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"togglemenu\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"messagemode\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"messagemode2\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"messagemode3\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"messagemode4\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"clear\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"condump\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Con_DrawNotify() {
    let mut x: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut text: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut i: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut currentColor: libc::c_int = 0;
    currentColor = 7i32;
    re.SetColor.expect("non-null function pointer")(g_color_table[currentColor
                                                                      as
                                                                      usize].as_mut_ptr());
    v = 0i32;
    i = con.current - 4i32 + 1i32;
    while i <= con.current {
        if !(i < 0i32) {
            time = con.times[(i % 4i32) as usize];
            if !(time == 0i32) {
                time = cls.realtime - time;
                if !(time as libc::c_float >
                         (*con_notifytime).value * 1000i32 as libc::c_float) {
                    text =
                        con.text.as_mut_ptr().offset((i % con.totallines *
                                                          con.linewidth) as
                                                         isize);
                    if !(cl.snap.ps.pm_type != PM_INTERMISSION as libc::c_int
                             && 0 != Key_GetCatcher() & (0x2i32 | 0x8i32)) {
                        x = 0i32;
                        while x < con.linewidth {
                            if !(*text.offset(x as isize) as libc::c_int &
                                     0xffi32 == ' ' as i32) {
                                if *text.offset(x as isize) as libc::c_int >>
                                       8i32 & 0x7i32 != currentColor {
                                    currentColor =
                                        *text.offset(x as isize) as
                                            libc::c_int >> 8i32 & 0x7i32;
                                    re.SetColor.expect("non-null function pointer")(g_color_table[currentColor
                                                                                                      as
                                                                                                      usize].as_mut_ptr());
                                }
                                SCR_DrawSmallChar(((*cl_conXOffset).integer as
                                                       libc::c_float +
                                                       con.xadjust +
                                                       ((x + 1i32) * 8i32) as
                                                           libc::c_float) as
                                                      libc::c_int, v,
                                                  *text.offset(x as isize) as
                                                      libc::c_int & 0xffi32);
                            }
                            x += 1
                        }
                        v += 16i32
                    }
                }
            }
        }
        i += 1
    }
    re.SetColor.expect("non-null function pointer")(0 as
                                                        *const libc::c_float);
    if 0 != Key_GetCatcher() & (0x2i32 | 0x8i32) { return }
    if 0 != Key_GetCatcher() & 0x4i32 {
        if 0 != chat_team as u64 {
            SCR_DrawBigString(8i32, v,
                              b"say_team:\x00" as *const u8 as
                                  *const libc::c_char, 1.0f32, qfalse);
            skip = 10i32
        } else {
            SCR_DrawBigString(8i32, v,
                              b"say:\x00" as *const u8 as *const libc::c_char,
                              1.0f32, qfalse);
            skip = 5i32
        }
        Field_BigDraw(&mut chatField, skip * 16i32, v,
                      640i32 - (skip + 1i32) * 16i32, qtrue, qtrue);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Con_RunConsole() {
    if 0 != Key_GetCatcher() & 0x1i32 {
        con.finalFrac = 0.5f64 as libc::c_float
    } else { con.finalFrac = 0i32 as libc::c_float }
    if con.finalFrac < con.displayFrac {
        con.displayFrac =
            (con.displayFrac as libc::c_double -
                 ((*con_conspeed).value * cls.realFrametime as libc::c_float)
                     as libc::c_double * 0.001f64) as libc::c_float;
        if con.finalFrac > con.displayFrac { con.displayFrac = con.finalFrac }
    } else if con.finalFrac > con.displayFrac {
        con.displayFrac =
            (con.displayFrac as libc::c_double +
                 ((*con_conspeed).value * cls.realFrametime as libc::c_float)
                     as libc::c_double * 0.001f64) as libc::c_float;
        if con.finalFrac < con.displayFrac { con.displayFrac = con.finalFrac }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Con_DrawConsole() {
    Con_CheckResize();
    if clc.state as libc::c_uint ==
           CA_DISCONNECTED as libc::c_int as libc::c_uint {
        if 0 == Key_GetCatcher() & (0x2i32 | 0x8i32) {
            Con_DrawSolidConsole(1.0f64 as libc::c_float);
            return
        }
    }
    if 0. != con.displayFrac {
        Con_DrawSolidConsole(con.displayFrac);
    } else if clc.state as libc::c_uint ==
                  CA_ACTIVE as libc::c_int as libc::c_uint {
        Con_DrawNotify();
    };
}
/*
================
Con_DrawSolidConsole

Draws the console with the solid background
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawSolidConsole(mut frac: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut text: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut row: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    //	qhandle_t		conShader;
    let mut currentColor: libc::c_int = 0;
    let mut color: vec4_t = [0.; 4];
    lines = (cls.glconfig.vidHeight as libc::c_float * frac) as libc::c_int;
    if lines <= 0i32 { return }
    if lines > cls.glconfig.vidHeight { lines = cls.glconfig.vidHeight }
    con.xadjust = 0i32 as libc::c_float;
    SCR_AdjustFrom640(&mut con.xadjust, 0 as *mut libc::c_float,
                      0 as *mut libc::c_float, 0 as *mut libc::c_float);
    y = (frac * 480i32 as libc::c_float) as libc::c_int;
    if y < 1i32 {
        y = 0i32
    } else {
        SCR_DrawPic(0i32 as libc::c_float, 0i32 as libc::c_float,
                    640i32 as libc::c_float, y as libc::c_float,
                    cls.consoleShader);
    }
    color[0usize] = 1i32 as vec_t;
    color[1usize] = 0i32 as vec_t;
    color[2usize] = 0i32 as vec_t;
    color[3usize] = 1i32 as vec_t;
    SCR_FillRect(0i32 as libc::c_float, y as libc::c_float,
                 640i32 as libc::c_float, 2i32 as libc::c_float,
                 color.as_mut_ptr());
    re.SetColor.expect("non-null function pointer")(g_color_table[('1' as i32
                                                                       -
                                                                       '0' as
                                                                           i32
                                                                       &
                                                                       0x7i32)
                                                                      as
                                                                      usize].as_mut_ptr());
    i =
        strlen(b"ioq3 1.36_GIT_363a9303-2019-02-25\x00" as *const u8 as
                   *const libc::c_char) as libc::c_int;
    x = 0i32;
    while x < i {
        SCR_DrawSmallChar(cls.glconfig.vidWidth - (i - x + 1i32) * 8i32,
                          lines - 16i32,
                          (*::std::mem::transmute::<&[u8; 34],
                                                    &[libc::c_char; 34]>(b"ioq3 1.36_GIT_363a9303-2019-02-25\x00"))[x
                                                                                                                        as
                                                                                                                        usize]
                              as libc::c_int);
        x += 1
    }
    con.vislines = lines;
    rows = (lines - 16i32) / 16i32;
    y = lines - 16i32 * 3i32;
    if con.display != con.current {
        re.SetColor.expect("non-null function pointer")(g_color_table[('1' as
                                                                           i32
                                                                           -
                                                                           '0'
                                                                               as
                                                                               i32
                                                                           &
                                                                           0x7i32)
                                                                          as
                                                                          usize].as_mut_ptr());
        x = 0i32;
        while x < con.linewidth {
            SCR_DrawSmallChar((con.xadjust +
                                   ((x + 1i32) * 8i32) as libc::c_float) as
                                  libc::c_int, y, '^' as i32);
            x += 4i32
        }
        y -= 16i32;
        rows -= 1
    }
    row = con.display;
    if con.x == 0i32 { row -= 1 }
    currentColor = 7i32;
    re.SetColor.expect("non-null function pointer")(g_color_table[currentColor
                                                                      as
                                                                      usize].as_mut_ptr());
    i = 0i32;
    while i < rows {
        if row < 0i32 { break ; }
        if !(con.current - row >= con.totallines) {
            // past scrollback wrap point
            text =
                con.text.as_mut_ptr().offset((row % con.totallines *
                                                  con.linewidth) as isize);
            x = 0i32;
            while x < con.linewidth {
                if !(*text.offset(x as isize) as libc::c_int & 0xffi32 ==
                         ' ' as i32) {
                    if *text.offset(x as isize) as libc::c_int >> 8i32 &
                           0x7i32 != currentColor {
                        currentColor =
                            *text.offset(x as isize) as libc::c_int >> 8i32 &
                                0x7i32;
                        re.SetColor.expect("non-null function pointer")(g_color_table[currentColor
                                                                                          as
                                                                                          usize].as_mut_ptr());
                    }
                    SCR_DrawSmallChar((con.xadjust +
                                           ((x + 1i32) * 8i32) as
                                               libc::c_float) as libc::c_int,
                                      y,
                                      *text.offset(x as isize) as libc::c_int
                                          & 0xffi32);
                }
                x += 1
            }
        }
        i += 1;
        y -= 16i32;
        row -= 1
    }
    Con_DrawInput();
    re.SetColor.expect("non-null function pointer")(0 as
                                                        *const libc::c_float);
}
/*
==============================================================================

DRAWING

==============================================================================
*/
/*
================
Con_DrawInput

Draw the editline after a ] prompt
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawInput() {
    let mut y: libc::c_int = 0;
    if clc.state as libc::c_uint !=
           CA_DISCONNECTED as libc::c_int as libc::c_uint &&
           0 == Key_GetCatcher() & 0x1i32 {
        return
    }
    y = con.vislines - 16i32 * 2i32;
    re.SetColor.expect("non-null function pointer")(con.color.as_mut_ptr());
    SCR_DrawSmallChar((con.xadjust + (1i32 * 8i32) as libc::c_float) as
                          libc::c_int, y, ']' as i32);
    Field_Draw(&mut g_consoleField,
               (con.xadjust + (2i32 * 8i32) as libc::c_float) as libc::c_int,
               y, 640i32 - 3i32 * 8i32, qtrue, qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn Con_PageUp() {
    con.display -= 2i32;
    if con.current - con.display >= con.totallines {
        con.display = con.current - con.totallines + 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Con_PageDown() {
    con.display += 2i32;
    if con.display > con.current { con.display = con.current };
}
#[no_mangle]
pub unsafe extern "C" fn Con_Top() {
    con.display = con.totallines;
    if con.current - con.display >= con.totallines {
        con.display = con.current - con.totallines + 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Con_Close() {
    if 0 == (*com_cl_running).integer { return }
    Field_Clear(&mut g_consoleField);
    Con_ClearNotify();
    Key_SetCatcher(Key_GetCatcher() & !0x1i32);
    con.finalFrac = 0i32 as libc::c_float;
    con.displayFrac = 0i32 as libc::c_float;
}