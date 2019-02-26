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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn ClampChar(i: libc::c_int) -> libc::c_schar;
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
    //Ignore __attribute__ on non-gcc platforms
    //#define	PRE_RELEASE_DEMO
    //============================================================================
    //
// msg.c
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct msg_t {
        pub allowoverflow: qboolean,
        pub overflowed: qboolean,
        pub oob: qboolean,
        pub data: *mut byte,
        pub maxsize: libc::c_int,
        pub cursize: libc::c_int,
        pub readcount: libc::c_int,
        pub bit: libc::c_int,
    }
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
    // override on command line, config files etc.
    // broadcast scan this many ports after
    // PORT_SERVER so a single machine can
									// run multiple servers
    // the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
    pub type svc_ops_e = libc::c_uint;
    //
    pub const svc_voipOpus: svc_ops_e = 10;
    // new commands, supported only by ioquake3 protocol but not legacy
    // not wrapped in USE_VOIP, so this value is reserved.
    pub const svc_voipSpeex: svc_ops_e = 9;
    pub const svc_EOF: svc_ops_e = 8;
    pub const svc_snapshot: svc_ops_e = 7;
    // [short] size [size bytes]
    pub const svc_download: svc_ops_e = 6;
    // [string] to be executed by client game module
    pub const svc_serverCommand: svc_ops_e = 5;
    // only in gamestate messages
    pub const svc_baseline: svc_ops_e = 4;
    // [short] [string] only in gamestate messages
    pub const svc_configstring: svc_ops_e = 3;
    pub const svc_gamestate: svc_ops_e = 2;
    pub const svc_nop: svc_ops_e = 1;
    pub const svc_bad: svc_ops_e = 0;
    //
// client to server
//
    pub type clc_ops_e = libc::c_uint;
    //
    pub const clc_voipOpus: clc_ops_e = 7;
    // new commands, supported only by ioquake3 protocol but not legacy
    // not wrapped in USE_VOIP, so this value is reserved.
    pub const clc_voipSpeex: clc_ops_e = 6;
    pub const clc_EOF: clc_ops_e = 5;
    // [string] message
    pub const clc_clientCommand: clc_ops_e = 4;
    // [[usercmd_t]
    pub const clc_moveNoDelta: clc_ops_e = 3;
    // [[usercmd_t]
    pub const clc_move: clc_ops_e = 2;
    pub const clc_nop: clc_ops_e = 1;
    pub const clc_bad: clc_ops_e = 0;
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
    use super::q_shared_h::{qboolean, byte, usercmd_t, cvar_t};
    use super::{libc};
    use super::stdint_h::{intptr_t};
    use super::stdint_uintn_h::{uint8_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn MSG_Init(buf: *mut msg_t, data: *mut byte,
                        length: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteData(buf: *mut msg_t, data: *const libc::c_void,
                             length: libc::c_int);
        #[no_mangle]
        pub fn MSG_Bitstream(buf: *mut msg_t);
        #[no_mangle]
        pub fn MSG_WriteBits(msg: *mut msg_t, value: libc::c_int,
                             bits: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteByte(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteShort(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteLong(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteString(sb: *mut msg_t, s: *const libc::c_char);
        #[no_mangle]
        pub fn MSG_HashKey(string: *const libc::c_char, maxlen: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        pub fn MSG_WriteDeltaUsercmdKey(msg: *mut msg_t, key: libc::c_int,
                                        from: *mut usercmd_t,
                                        to: *mut usercmd_t);
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
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        #[no_mangle]
        pub fn Com_IsVoipTarget(voipTargets: *mut uint8_t,
                                voipTargetsSize: libc::c_int,
                                clientNum: libc::c_int) -> qboolean;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        // both client and server must agree to pause
        #[no_mangle]
        pub static mut cl_paused: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_paused: *mut cvar_t;
        #[no_mangle]
        pub static mut com_frameTime: libc::c_int;
        // call before filesystem access
        #[no_mangle]
        pub fn SCR_DebugGraph(value: libc::c_float);
        //Does NOT parse port numbers, only base addresses.
        #[no_mangle]
        pub fn Sys_IsLANAddress(adr: netadr_t) -> qboolean;
    }
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
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
        //
// cl_input
//
        //=============================================================================
        // interface to cgame dll or vm
        // interface to ui dll or vm
        //
// cvars
//
        //
// cl_net_chan.c
//
        //
// cl_main.c
//
        //	void	(*CG_KeyEvent)( int key, qboolean down );
        /*
==================================================================

functions exported to the main executable

==================================================================
*/
        //	void	(*CG_MouseEvent)( int dx, int dy );
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
        //	void CG_Init( int serverMessageNum, int serverCommandSequence, int clientNum )
	// called when the level loads or when the renderer is restarted
	// all media should be registered at this time
	// cgame will display loading status by calling SCR_Update, which
	// will call CG_DrawInformation during the loading process
	// reliableCommandSequence will be 0 on fresh loads, but higher for
	// demos, tourney restarts, or vid_restarts
        //	void	UI_KeyEvent( int key );
        //	void	UI_DrawConnectScreen( qboolean overlay );
        //	qboolean UI_ConsoleCommand( int realTime );
        //	void	UI_SetActiveMenu( uiMenuCommand_t menu );
        //	qboolean UI_IsFullscreen( void );
        //	void	UI_Refresh( int time );
        //	void	UI_MouseEvent( int dx, int dy );
        //	void	UI_Shutdown( void );
        //	void	UI_Init( void );
        // system reserved
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct kbutton_t {
        /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
        pub down: [libc::c_int; 2],
        pub downtime: libc::c_uint,
        pub msec: libc::c_uint,
        pub active: qboolean,
        pub wasPressed: qboolean,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean, gameState_t, usercmd_t, vec3_t,
                            entityState_t, byte, playerState_t, connstate_t,
                            fileHandle_t, qhandle_t, cvar_t};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t, msg_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    extern "C" {
        #[no_mangle]
        pub static mut cl: clientActive_t;
        #[no_mangle]
        pub static mut cgvm: *mut vm_t;
        #[no_mangle]
        pub fn Key_GetCatcher() -> libc::c_int;
        #[no_mangle]
        pub static mut uivm: *mut vm_t;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        #[no_mangle]
        pub static mut cl_nodelta: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_debugMove: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_maxpackets: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_packetdup: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_showSend: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_sensitivity: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_freelook: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_mouseAccel: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_mouseAccelOffset: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_mouseAccelStyle: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_showMouseRate: *mut cvar_t;
        #[no_mangle]
        pub static mut m_pitch: *mut cvar_t;
        #[no_mangle]
        pub static mut m_yaw: *mut cvar_t;
        #[no_mangle]
        pub static mut m_forward: *mut cvar_t;
        #[no_mangle]
        pub static mut m_side: *mut cvar_t;
        #[no_mangle]
        pub static mut m_filter: *mut cvar_t;
        #[no_mangle]
        pub static mut j_pitch: *mut cvar_t;
        #[no_mangle]
        pub static mut j_yaw: *mut cvar_t;
        #[no_mangle]
        pub static mut j_forward: *mut cvar_t;
        #[no_mangle]
        pub static mut j_side: *mut cvar_t;
        #[no_mangle]
        pub static mut j_up: *mut cvar_t;
        #[no_mangle]
        pub static mut j_pitch_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_yaw_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_forward_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_side_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut j_up_axis: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_lanForcePackets: *mut cvar_t;
        #[no_mangle]
        pub fn CL_Netchan_Transmit(chan: *mut netchan_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn CL_WriteDemoMessage(msg: *mut msg_t, headerBytes: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/cgame/cg_public.h"]
pub mod cg_public_h {
    pub const CG_MOUSE_EVENT: unnamed_1 = 7;
    pub type unnamed_1 = libc::c_uint;
    pub const CG_EVENT_HANDLING: unnamed_1 = 8;
    pub const CG_KEY_EVENT: unnamed_1 = 6;
    pub const CG_LAST_ATTACKER: unnamed_1 = 5;
    pub const CG_CROSSHAIR_PLAYER: unnamed_1 = 4;
    pub const CG_DRAW_ACTIVE_FRAME: unnamed_1 = 3;
    pub const CG_CONSOLE_COMMAND: unnamed_1 = 2;
    pub const CG_SHUTDOWN: unnamed_1 = 1;
    pub const CG_INIT: unnamed_1 = 0;
    use super::{libc};
}
#[header_src =
      "ioq3/code/ui/ui_public.h"]
pub mod ui_public_h {
    pub const UI_MOUSE_EVENT: unnamed_0 = 4;
    pub type unnamed_0 = libc::c_uint;
    pub const UI_HASUNIQUECDKEY: unnamed_0 = 10;
    pub const UI_DRAW_CONNECT_SCREEN: unnamed_0 = 9;
    pub const UI_CONSOLE_COMMAND: unnamed_0 = 8;
    pub const UI_SET_ACTIVE_MENU: unnamed_0 = 7;
    pub const UI_IS_FULLSCREEN: unnamed_0 = 6;
    pub const UI_REFRESH: unnamed_0 = 5;
    pub const UI_KEY_EVENT: unnamed_0 = 3;
    pub const UI_SHUTDOWN: unnamed_0 = 2;
    pub const UI_INIT: unnamed_0 = 1;
    pub const UI_GETAPIVERSION: unnamed_0 = 0;
    use super::{libc};
}
#[header_src =
      "ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
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
    use super::{libc};
    use super::q_shared_h::{qboolean};
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
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/keys.h"]
pub mod keys_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut anykeydown: libc::c_int;
    }
}
#[header_src =
      "ioq3/code/client/cl_input.c"]
pub mod cl_input_c {
    use super::client_h::{kbutton_t};
    use super::{libc};
    use super::q_shared_h::{qboolean, usercmd_t};
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, vec_t,
                       vec3_t, cvar_s, cvar_t, gameState_t, playerState_s,
                       playerState_t, usercmd_s, usercmd_t, trType_t,
                       TR_GRAVITY, TR_SINE, TR_LINEAR_STOP, TR_LINEAR,
                       TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, connstate_t,
                       CA_CINEMATIC, CA_ACTIVE, CA_PRIMED, CA_LOADING,
                       CA_CONNECTED, CA_CHALLENGING, CA_CONNECTING,
                       CA_AUTHORIZING, CA_DISCONNECTED, CA_UNINITIALIZED,
                       ClampChar, Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      svc_ops_e, svc_voipOpus, svc_voipSpeex, svc_EOF,
                      svc_snapshot, svc_download, svc_serverCommand,
                      svc_baseline, svc_configstring, svc_gamestate, svc_nop,
                      svc_bad, clc_ops_e, clc_voipOpus, clc_voipSpeex,
                      clc_EOF, clc_clientCommand, clc_moveNoDelta, clc_move,
                      clc_nop, clc_bad, vm_t, xcommand_t, vm_s, MSG_Init,
                      MSG_WriteData, MSG_Bitstream, MSG_WriteBits,
                      MSG_WriteByte, MSG_WriteShort, MSG_WriteLong,
                      MSG_WriteString, MSG_HashKey, MSG_WriteDeltaUsercmdKey,
                      VM_Call, Cmd_AddCommand, Cmd_RemoveCommand, Cmd_Argv,
                      Cvar_Get, Cvar_Set, Com_IsVoipTarget, com_sv_running,
                      cl_paused, sv_paused, com_frameTime, SCR_DebugGraph,
                      Sys_IsLANAddress};
use self::client_h::{clientActive_t, clSnapshot_t, outPacket_t,
                     clientConnection_t, serverInfo_t, clientStatic_t,
                     kbutton_t, cl, cgvm, Key_GetCatcher, uivm, clc, cls,
                     cl_nodelta, cl_debugMove, cl_maxpackets, cl_packetdup,
                     cl_showSend, cl_sensitivity, cl_freelook, cl_mouseAccel,
                     cl_mouseAccelOffset, cl_mouseAccelStyle,
                     cl_showMouseRate, m_pitch, m_yaw, m_forward, m_side,
                     m_filter, j_pitch, j_yaw, j_forward, j_side, j_up,
                     j_pitch_axis, j_yaw_axis, j_forward_axis, j_side_axis,
                     j_up_axis, cl_lanForcePackets, CL_Netchan_Transmit,
                     CL_WriteDemoMessage};
use self::cg_public_h::{CG_MOUSE_EVENT, unnamed_1, CG_EVENT_HANDLING,
                        CG_KEY_EVENT, CG_LAST_ATTACKER, CG_CROSSHAIR_PLAYER,
                        CG_DRAW_ACTIVE_FRAME, CG_CONSOLE_COMMAND, CG_SHUTDOWN,
                        CG_INIT};
use self::ui_public_h::{UI_MOUSE_EVENT, unnamed_0, UI_HASUNIQUECDKEY,
                        UI_DRAW_CONNECT_SCREEN, UI_CONSOLE_COMMAND,
                        UI_SET_ACTIVE_MENU, UI_IS_FULLSCREEN, UI_REFRESH,
                        UI_KEY_EVENT, UI_SHUTDOWN, UI_INIT, UI_GETAPIVERSION};
use self::tr_types_h::{textureCompression_t, TC_S3TC_ARB, TC_S3TC, TC_NONE,
                       glDriverType_t, GLDRV_VOODOO, GLDRV_STANDALONE,
                       GLDRV_ICD, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glconfig_t};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::mathcalls_h::{sqrt, fabs, powf};
use self::string_h::{memset};
use self::stdlib_h::{atoi};
use self::keys_h::{anykeydown};
// char events are for field typing, not game control
#[no_mangle]
pub unsafe extern "C" fn CL_MouseEvent(mut dx: libc::c_int,
                                       mut dy: libc::c_int,
                                       mut time: libc::c_int) {
    if 0 != Key_GetCatcher() & 0x2i32 {
        VM_Call(uivm, UI_MOUSE_EVENT as libc::c_int, dx, dy);
    } else if 0 != Key_GetCatcher() & 0x8i32 {
        VM_Call(cgvm, CG_MOUSE_EVENT as libc::c_int, dx, dy);
    } else {
        cl.mouseDx[cl.mouseIndex as usize] += dx;
        cl.mouseDy[cl.mouseIndex as usize] += dy
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_JoystickEvent(mut axis: libc::c_int,
                                          mut value: libc::c_int,
                                          mut time: libc::c_int) {
    if axis < 0i32 || axis >= 16i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_JoystickEvent: bad axis %i\x00" as *const u8 as
                      *const libc::c_char, axis);
    }
    cl.joystickAxis[axis as usize] = value;
}
#[no_mangle]
pub static mut cl_yawspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_pitchspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_run: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_anglespeedkey: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CL_InitInput() {
    Cmd_AddCommand(b"centerview\x00" as *const u8 as *const libc::c_char,
                   Some(IN_CenterView));
    Cmd_AddCommand(b"+moveup\x00" as *const u8 as *const libc::c_char,
                   Some(IN_UpDown));
    Cmd_AddCommand(b"-moveup\x00" as *const u8 as *const libc::c_char,
                   Some(IN_UpUp));
    Cmd_AddCommand(b"+movedown\x00" as *const u8 as *const libc::c_char,
                   Some(IN_DownDown));
    Cmd_AddCommand(b"-movedown\x00" as *const u8 as *const libc::c_char,
                   Some(IN_DownUp));
    Cmd_AddCommand(b"+left\x00" as *const u8 as *const libc::c_char,
                   Some(IN_LeftDown));
    Cmd_AddCommand(b"-left\x00" as *const u8 as *const libc::c_char,
                   Some(IN_LeftUp));
    Cmd_AddCommand(b"+right\x00" as *const u8 as *const libc::c_char,
                   Some(IN_RightDown));
    Cmd_AddCommand(b"-right\x00" as *const u8 as *const libc::c_char,
                   Some(IN_RightUp));
    Cmd_AddCommand(b"+forward\x00" as *const u8 as *const libc::c_char,
                   Some(IN_ForwardDown));
    Cmd_AddCommand(b"-forward\x00" as *const u8 as *const libc::c_char,
                   Some(IN_ForwardUp));
    Cmd_AddCommand(b"+back\x00" as *const u8 as *const libc::c_char,
                   Some(IN_BackDown));
    Cmd_AddCommand(b"-back\x00" as *const u8 as *const libc::c_char,
                   Some(IN_BackUp));
    Cmd_AddCommand(b"+lookup\x00" as *const u8 as *const libc::c_char,
                   Some(IN_LookupDown));
    Cmd_AddCommand(b"-lookup\x00" as *const u8 as *const libc::c_char,
                   Some(IN_LookupUp));
    Cmd_AddCommand(b"+lookdown\x00" as *const u8 as *const libc::c_char,
                   Some(IN_LookdownDown));
    Cmd_AddCommand(b"-lookdown\x00" as *const u8 as *const libc::c_char,
                   Some(IN_LookdownUp));
    Cmd_AddCommand(b"+strafe\x00" as *const u8 as *const libc::c_char,
                   Some(IN_StrafeDown));
    Cmd_AddCommand(b"-strafe\x00" as *const u8 as *const libc::c_char,
                   Some(IN_StrafeUp));
    Cmd_AddCommand(b"+moveleft\x00" as *const u8 as *const libc::c_char,
                   Some(IN_MoveleftDown));
    Cmd_AddCommand(b"-moveleft\x00" as *const u8 as *const libc::c_char,
                   Some(IN_MoveleftUp));
    Cmd_AddCommand(b"+moveright\x00" as *const u8 as *const libc::c_char,
                   Some(IN_MoverightDown));
    Cmd_AddCommand(b"-moveright\x00" as *const u8 as *const libc::c_char,
                   Some(IN_MoverightUp));
    Cmd_AddCommand(b"+speed\x00" as *const u8 as *const libc::c_char,
                   Some(IN_SpeedDown));
    Cmd_AddCommand(b"-speed\x00" as *const u8 as *const libc::c_char,
                   Some(IN_SpeedUp));
    Cmd_AddCommand(b"+attack\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button0Down));
    Cmd_AddCommand(b"-attack\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button0Up));
    Cmd_AddCommand(b"+button0\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button0Down));
    Cmd_AddCommand(b"-button0\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button0Up));
    Cmd_AddCommand(b"+button1\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button1Down));
    Cmd_AddCommand(b"-button1\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button1Up));
    Cmd_AddCommand(b"+button2\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button2Down));
    Cmd_AddCommand(b"-button2\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button2Up));
    Cmd_AddCommand(b"+button3\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button3Down));
    Cmd_AddCommand(b"-button3\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button3Up));
    Cmd_AddCommand(b"+button4\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button4Down));
    Cmd_AddCommand(b"-button4\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button4Up));
    Cmd_AddCommand(b"+button5\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button5Down));
    Cmd_AddCommand(b"-button5\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button5Up));
    Cmd_AddCommand(b"+button6\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button6Down));
    Cmd_AddCommand(b"-button6\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button6Up));
    Cmd_AddCommand(b"+button7\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button7Down));
    Cmd_AddCommand(b"-button7\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button7Up));
    Cmd_AddCommand(b"+button8\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button8Down));
    Cmd_AddCommand(b"-button8\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button8Up));
    Cmd_AddCommand(b"+button9\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button9Down));
    Cmd_AddCommand(b"-button9\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button9Up));
    Cmd_AddCommand(b"+button10\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button10Down));
    Cmd_AddCommand(b"-button10\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button10Up));
    Cmd_AddCommand(b"+button11\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button11Down));
    Cmd_AddCommand(b"-button11\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button11Up));
    Cmd_AddCommand(b"+button12\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button12Down));
    Cmd_AddCommand(b"-button12\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button12Up));
    Cmd_AddCommand(b"+button13\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button13Down));
    Cmd_AddCommand(b"-button13\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button13Up));
    Cmd_AddCommand(b"+button14\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button14Down));
    Cmd_AddCommand(b"-button14\x00" as *const u8 as *const libc::c_char,
                   Some(IN_Button14Up));
    Cmd_AddCommand(b"+mlook\x00" as *const u8 as *const libc::c_char,
                   Some(IN_MLookDown));
    Cmd_AddCommand(b"-mlook\x00" as *const u8 as *const libc::c_char,
                   Some(IN_MLookUp));
    Cmd_AddCommand(b"+voiprecord\x00" as *const u8 as *const libc::c_char,
                   Some(IN_VoipRecordDown));
    Cmd_AddCommand(b"-voiprecord\x00" as *const u8 as *const libc::c_char,
                   Some(IN_VoipRecordUp));
    cl_nodelta =
        Cvar_Get(b"cl_nodelta\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_debugMove =
        Cvar_Get(b"cl_debugMove\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn IN_VoipRecordUp() {
    IN_KeyUp(&mut in_voiprecord);
    Cvar_Set(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub static mut in_voiprecord: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_KeyUp(mut b: *mut kbutton_t) {
    let mut k: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uptime: libc::c_uint = 0;
    c = Cmd_Argv(1i32);
    if 0 != *c.offset(0isize) {
        k = atoi(c)
    } else {
        (*b).down[1usize] = 0i32;
        (*b).down[0usize] = (*b).down[1usize];
        (*b).active = qfalse;
        return
    }
    if (*b).down[0usize] == k {
        (*b).down[0usize] = 0i32
    } else if (*b).down[1usize] == k {
        (*b).down[1usize] = 0i32
    } else { return }
    if 0 != (*b).down[0usize] || 0 != (*b).down[1usize] { return }
    (*b).active = qfalse;
    c = Cmd_Argv(2i32);
    uptime = atoi(c) as libc::c_uint;
    if 0 != uptime {
        (*b).msec = (*b).msec.wrapping_add(uptime.wrapping_sub((*b).downtime))
    } else {
        (*b).msec =
            (*b).msec.wrapping_add(frame_msec.wrapping_div(2i32 as
                                                               libc::c_uint))
    }
    (*b).active = qfalse;
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
// cl.input.c  -- builds an intended movement command to send to the server
#[no_mangle]
pub static mut frame_msec: libc::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn IN_VoipRecordDown() {
    IN_KeyDown(&mut in_voiprecord);
    Cvar_Set(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn IN_KeyDown(mut b: *mut kbutton_t) {
    let mut k: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    c = Cmd_Argv(1i32);
    if 0 != *c.offset(0isize) { k = atoi(c) } else { k = -1i32 }
    if k == (*b).down[0usize] || k == (*b).down[1usize] { return }
    if 0 == (*b).down[0usize] {
        (*b).down[0usize] = k
    } else if 0 == (*b).down[1usize] {
        (*b).down[1usize] = k
    } else {
        Com_Printf(b"Three keys down for a button!\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if 0 != (*b).active as u64 { return }
    c = Cmd_Argv(2i32);
    (*b).downtime = atoi(c) as libc::c_uint;
    (*b).active = qtrue;
    (*b).wasPressed = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn IN_MLookUp() {
    in_mlooking = qfalse;
    if 0 == (*cl_freelook).integer { IN_CenterView(); };
}
#[no_mangle]
pub unsafe extern "C" fn IN_CenterView() {
    cl.viewangles[0usize] =
        -(cl.snap.ps.delta_angles[0usize] as libc::c_double *
              (360.0f64 / 65536i32 as libc::c_double)) as vec_t;
}
#[no_mangle]
pub static mut in_mlooking: qboolean = qfalse;
#[no_mangle]
pub unsafe extern "C" fn IN_MLookDown() { in_mlooking = qtrue; }
#[no_mangle]
pub unsafe extern "C" fn IN_Button14Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(14isize));
}
#[no_mangle]
pub static mut in_buttons: [kbutton_t; 16] =
    [kbutton_t{down: [0; 2],
               downtime: 0,
               msec: 0,
               active: qfalse,
               wasPressed: qfalse,}; 16];
#[no_mangle]
pub unsafe extern "C" fn IN_Button14Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(14isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button13Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(13isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button13Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(13isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button12Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(12isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button12Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(12isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button11Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(11isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button11Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(11isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button10Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(10isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button10Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(10isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button9Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(9isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button9Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(9isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button8Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(8isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button8Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(8isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button7Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(7isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button7Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(7isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button6Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(6isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button6Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(6isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button5Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(5isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button5Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(5isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button4Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(4isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button4Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(4isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button3Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(3isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button3Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(3isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button2Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(2isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button2Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(2isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button1Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(1isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button1Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(1isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button0Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(0isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button0Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(0isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_SpeedUp() { IN_KeyUp(&mut in_speed); }
#[no_mangle]
pub static mut in_speed: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_SpeedDown() { IN_KeyDown(&mut in_speed); }
#[no_mangle]
pub unsafe extern "C" fn IN_MoverightUp() { IN_KeyUp(&mut in_moveright); }
#[no_mangle]
pub static mut in_moveright: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_MoverightDown() { IN_KeyDown(&mut in_moveright); }
#[no_mangle]
pub unsafe extern "C" fn IN_MoveleftUp() { IN_KeyUp(&mut in_moveleft); }
#[no_mangle]
pub static mut in_moveleft: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_MoveleftDown() { IN_KeyDown(&mut in_moveleft); }
#[no_mangle]
pub unsafe extern "C" fn IN_StrafeUp() { IN_KeyUp(&mut in_strafe); }
#[no_mangle]
pub static mut in_strafe: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_StrafeDown() { IN_KeyDown(&mut in_strafe); }
#[no_mangle]
pub unsafe extern "C" fn IN_LookdownUp() { IN_KeyUp(&mut in_lookdown); }
#[no_mangle]
pub static mut in_lookdown: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_LookdownDown() { IN_KeyDown(&mut in_lookdown); }
#[no_mangle]
pub unsafe extern "C" fn IN_LookupUp() { IN_KeyUp(&mut in_lookup); }
#[no_mangle]
pub static mut in_lookup: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_LookupDown() { IN_KeyDown(&mut in_lookup); }
#[no_mangle]
pub unsafe extern "C" fn IN_BackUp() { IN_KeyUp(&mut in_back); }
#[no_mangle]
pub static mut in_back: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_BackDown() { IN_KeyDown(&mut in_back); }
#[no_mangle]
pub unsafe extern "C" fn IN_ForwardUp() { IN_KeyUp(&mut in_forward); }
#[no_mangle]
pub static mut in_forward: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_ForwardDown() { IN_KeyDown(&mut in_forward); }
#[no_mangle]
pub unsafe extern "C" fn IN_RightUp() { IN_KeyUp(&mut in_right); }
#[no_mangle]
pub static mut in_right: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_RightDown() { IN_KeyDown(&mut in_right); }
#[no_mangle]
pub unsafe extern "C" fn IN_LeftUp() { IN_KeyUp(&mut in_left); }
/*
===============================================================================

KEY BUTTONS

Continuous button event tracking is complicated by the fact that two different
input sources (say, mouse button 1 and the control key) can both press the
same button, but the button should only be released when both of the
pressing key have been released.

When a key event issues a button command (+forward, +attack, etc), it appends
its key number as argv(1) so it can be matched up with the release.

argv(2) will be set to the time the event happened, which allows exact
control even at low framerates when the down and up events may both get qued
at the same time.

===============================================================================
*/
#[no_mangle]
pub static mut in_left: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_LeftDown() { IN_KeyDown(&mut in_left); }
#[no_mangle]
pub unsafe extern "C" fn IN_DownUp() { IN_KeyUp(&mut in_down); }
#[no_mangle]
pub static mut in_down: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_DownDown() { IN_KeyDown(&mut in_down); }
#[no_mangle]
pub unsafe extern "C" fn IN_UpUp() { IN_KeyUp(&mut in_up); }
#[no_mangle]
pub static mut in_up: kbutton_t =
    kbutton_t{down: [0; 2],
              downtime: 0,
              msec: 0,
              active: qfalse,
              wasPressed: qfalse,};
#[no_mangle]
pub unsafe extern "C" fn IN_UpDown() { IN_KeyDown(&mut in_up); }
#[no_mangle]
pub unsafe extern "C" fn CL_ShutdownInput() {
    Cmd_RemoveCommand(b"centerview\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+moveup\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-moveup\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+movedown\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-movedown\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+left\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-left\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+right\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-right\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+forward\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-forward\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+back\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-back\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+lookup\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-lookup\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+lookdown\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-lookdown\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+strafe\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-strafe\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+moveleft\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-moveleft\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+moveright\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-moveright\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+speed\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-speed\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+attack\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-attack\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button0\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button0\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button1\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button1\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button2\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button2\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button3\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button3\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button4\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button4\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button5\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button5\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button6\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button6\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button7\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button7\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button8\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button8\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button9\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button9\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button10\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button10\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button11\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button11\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button12\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button12\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button13\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button13\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+button14\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-button14\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+mlook\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-mlook\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"+voiprecord\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"-voiprecord\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn CL_SendCmd() {
    if (clc.state as libc::c_uint) <
           CA_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    if 0 != (*com_sv_running).integer && 0 != (*sv_paused).integer &&
           0 != (*cl_paused).integer {
        return
    }
    CL_CreateNewCommands();
    if 0 == CL_ReadyToSendPacket() as u64 {
        if 0 != (*cl_showSend).integer {
            Com_Printf(b". \x00" as *const u8 as *const libc::c_char);
        }
        return
    }
    CL_WritePacket();
}
#[no_mangle]
pub unsafe extern "C" fn CL_WritePacket() {
    let mut buf: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut data: [byte; 16384] = [0; 16384];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cmd: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut oldcmd: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut nullcmd: usercmd_t =
        usercmd_s{serverTime: 0,
                  angles: [0; 3],
                  buttons: 0,
                  weapon: 0,
                  forwardmove: 0,
                  rightmove: 0,
                  upmove: 0,};
    let mut packetNum: libc::c_int = 0;
    let mut oldPacketNum: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut key: libc::c_int = 0;
    if 0 != clc.demoplaying as libc::c_uint ||
           clc.state as libc::c_uint ==
               CA_CINEMATIC as libc::c_int as libc::c_uint {
        return
    }
    memset(&mut nullcmd as *mut usercmd_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    oldcmd = &mut nullcmd;
    MSG_Init(&mut buf, data.as_mut_ptr(),
             ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                 libc::c_int);
    MSG_Bitstream(&mut buf);
    MSG_WriteLong(&mut buf, cl.serverId);
    MSG_WriteLong(&mut buf, clc.serverMessageSequence);
    MSG_WriteLong(&mut buf, clc.serverCommandSequence);
    i = clc.reliableAcknowledge + 1i32;
    while i <= clc.reliableSequence {
        MSG_WriteByte(&mut buf, clc_clientCommand as libc::c_int);
        MSG_WriteLong(&mut buf, i);
        MSG_WriteString(&mut buf,
                        clc.reliableCommands[(i & 64i32 - 1i32) as
                                                 usize].as_mut_ptr());
        i += 1
    }
    if (*cl_packetdup).integer < 0i32 {
        Cvar_Set(b"cl_packetdup\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
    } else if (*cl_packetdup).integer > 5i32 {
        Cvar_Set(b"cl_packetdup\x00" as *const u8 as *const libc::c_char,
                 b"5\x00" as *const u8 as *const libc::c_char);
    }
    oldPacketNum =
        clc.netchan.outgoingSequence - 1i32 - (*cl_packetdup).integer &
            32i32 - 1i32;
    count = cl.cmdNumber - cl.outPackets[oldPacketNum as usize].p_cmdNumber;
    if count > 32i32 {
        count = 32i32;
        Com_Printf(b"MAX_PACKET_USERCMDS\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    if clc.voipOutgoingDataSize > 0i32 {
        if 0 != clc.voipFlags as libc::c_int & 0x1i32 ||
               0 !=
                   Com_IsVoipTarget(clc.voipTargets.as_mut_ptr(),
                                    ::std::mem::size_of::<[uint8_t; 8]>() as
                                        libc::c_ulong as libc::c_int, -1i32)
                       as libc::c_uint {
            MSG_WriteByte(&mut buf, clc_voipOpus as libc::c_int);
            MSG_WriteByte(&mut buf,
                          clc.voipOutgoingGeneration as libc::c_int);
            MSG_WriteLong(&mut buf, clc.voipOutgoingSequence);
            MSG_WriteByte(&mut buf, clc.voipOutgoingDataFrames);
            MSG_WriteData(&mut buf,
                          clc.voipTargets.as_mut_ptr() as *const libc::c_void,
                          ::std::mem::size_of::<[uint8_t; 8]>() as
                              libc::c_ulong as libc::c_int);
            MSG_WriteByte(&mut buf, clc.voipFlags as libc::c_int);
            MSG_WriteShort(&mut buf, clc.voipOutgoingDataSize);
            MSG_WriteData(&mut buf,
                          clc.voipOutgoingData.as_mut_ptr() as
                              *const libc::c_void, clc.voipOutgoingDataSize);
            if 0 != clc.demorecording as libc::c_uint &&
                   0 == clc.demowaiting as u64 {
                let voipSize: libc::c_int = clc.voipOutgoingDataSize;
                let mut fakemsg: msg_t =
                    msg_t{allowoverflow: qfalse,
                          overflowed: qfalse,
                          oob: qfalse,
                          data: 0 as *mut byte,
                          maxsize: 0,
                          cursize: 0,
                          readcount: 0,
                          bit: 0,};
                let mut fakedata: [byte; 16384] = [0; 16384];
                MSG_Init(&mut fakemsg, fakedata.as_mut_ptr(),
                         ::std::mem::size_of::<[byte; 16384]>() as
                             libc::c_ulong as libc::c_int);
                MSG_Bitstream(&mut fakemsg);
                MSG_WriteLong(&mut fakemsg, clc.reliableAcknowledge);
                MSG_WriteByte(&mut fakemsg, svc_voipOpus as libc::c_int);
                MSG_WriteShort(&mut fakemsg, clc.clientNum);
                MSG_WriteByte(&mut fakemsg,
                              clc.voipOutgoingGeneration as libc::c_int);
                MSG_WriteLong(&mut fakemsg, clc.voipOutgoingSequence);
                MSG_WriteByte(&mut fakemsg, clc.voipOutgoingDataFrames);
                MSG_WriteShort(&mut fakemsg, clc.voipOutgoingDataSize);
                MSG_WriteBits(&mut fakemsg, clc.voipFlags as libc::c_int,
                              2i32);
                MSG_WriteData(&mut fakemsg,
                              clc.voipOutgoingData.as_mut_ptr() as
                                  *const libc::c_void, voipSize);
                MSG_WriteByte(&mut fakemsg, svc_EOF as libc::c_int);
                CL_WriteDemoMessage(&mut fakemsg, 0i32);
            }
            clc.voipOutgoingSequence += clc.voipOutgoingDataFrames;
            clc.voipOutgoingDataSize = 0i32;
            clc.voipOutgoingDataFrames = 0i32
        } else {
            clc.voipOutgoingDataSize = 0i32;
            clc.voipOutgoingDataFrames = 0i32
        }
    }
    if count >= 1i32 {
        if 0 != (*cl_showSend).integer {
            Com_Printf(b"(%i)\x00" as *const u8 as *const libc::c_char,
                       count);
        }
        if 0 != (*cl_nodelta).integer || 0 == cl.snap.valid as u64 ||
               0 != clc.demowaiting as libc::c_uint ||
               clc.serverMessageSequence != cl.snap.messageNum {
            MSG_WriteByte(&mut buf, clc_moveNoDelta as libc::c_int);
        } else { MSG_WriteByte(&mut buf, clc_move as libc::c_int); }
        MSG_WriteByte(&mut buf, count);
        key = clc.checksumFeed;
        key ^= clc.serverMessageSequence;
        key ^=
            MSG_HashKey(clc.serverCommands[(clc.serverCommandSequence &
                                                64i32 - 1i32) as
                                               usize].as_mut_ptr(), 32i32);
        i = 0i32;
        while i < count {
            j = cl.cmdNumber - count + i + 1i32 & 64i32 - 1i32;
            cmd =
                &mut *cl.cmds.as_mut_ptr().offset(j as isize) as
                    *mut usercmd_t;
            MSG_WriteDeltaUsercmdKey(&mut buf, key, oldcmd, cmd);
            oldcmd = cmd;
            i += 1
        }
    }
    packetNum = clc.netchan.outgoingSequence & 32i32 - 1i32;
    cl.outPackets[packetNum as usize].p_realtime = cls.realtime;
    cl.outPackets[packetNum as usize].p_serverTime = (*oldcmd).serverTime;
    cl.outPackets[packetNum as usize].p_cmdNumber = cl.cmdNumber;
    clc.lastPacketSentTime = cls.realtime;
    if 0 != (*cl_showSend).integer {
        Com_Printf(b"%i \x00" as *const u8 as *const libc::c_char,
                   buf.cursize);
    }
    CL_Netchan_Transmit(&mut clc.netchan, &mut buf);
}
/*
=================
CL_ReadyToSendPacket

Returns qfalse if we are over the maxpackets limit
and should choke back the bandwidth a bit by not sending
a packet this frame.  All the commands will still get
delivered in the next packet, but saving a header and
getting more delta compression will reduce total bandwidth.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadyToSendPacket() -> qboolean {
    let mut oldPacketNum: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    if 0 != clc.demoplaying as libc::c_uint ||
           clc.state as libc::c_uint ==
               CA_CINEMATIC as libc::c_int as libc::c_uint {
        return qfalse
    }
    if 0 != *clc.downloadTempName.as_mut_ptr() as libc::c_int &&
           cls.realtime - clc.lastPacketSentTime < 50i32 {
        return qfalse
    }
    if clc.state as libc::c_uint != CA_ACTIVE as libc::c_int as libc::c_uint
           &&
           clc.state as libc::c_uint !=
               CA_PRIMED as libc::c_int as libc::c_uint &&
           0 == *clc.downloadTempName.as_mut_ptr() &&
           cls.realtime - clc.lastPacketSentTime < 1000i32 {
        return qfalse
    }
    if clc.netchan.remoteAddress.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        return qtrue
    }
    if 0 != (*cl_lanForcePackets).integer &&
           0 != Sys_IsLANAddress(clc.netchan.remoteAddress) as libc::c_uint {
        return qtrue
    }
    if (*cl_maxpackets).integer < 15i32 {
        Cvar_Set(b"cl_maxpackets\x00" as *const u8 as *const libc::c_char,
                 b"15\x00" as *const u8 as *const libc::c_char);
    } else if (*cl_maxpackets).integer > 125i32 {
        Cvar_Set(b"cl_maxpackets\x00" as *const u8 as *const libc::c_char,
                 b"125\x00" as *const u8 as *const libc::c_char);
    }
    oldPacketNum = clc.netchan.outgoingSequence - 1i32 & 32i32 - 1i32;
    delta = cls.realtime - cl.outPackets[oldPacketNum as usize].p_realtime;
    if delta < 1000i32 / (*cl_maxpackets).integer { return qfalse }
    return qtrue;
}
/*
=================
CL_CreateNewCommands

Create a new usercmd_t structure for this frame
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CreateNewCommands() {
    let mut cmdNum: libc::c_int = 0;
    if (clc.state as libc::c_uint) < CA_PRIMED as libc::c_int as libc::c_uint
       {
        return
    }
    frame_msec = (com_frameTime - old_com_frameTime) as libc::c_uint;
    if frame_msec < 1i32 as libc::c_uint { frame_msec = 1i32 as libc::c_uint }
    if frame_msec > 200i32 as libc::c_uint {
        frame_msec = 200i32 as libc::c_uint
    }
    old_com_frameTime = com_frameTime;
    cl.cmdNumber += 1;
    cmdNum = cl.cmdNumber & 64i32 - 1i32;
    cl.cmds[cmdNum as usize] = CL_CreateCmd();
}
/*
=================
CL_CreateCmd
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CreateCmd() -> usercmd_t {
    let mut cmd: usercmd_t =
        usercmd_s{serverTime: 0,
                  angles: [0; 3],
                  buttons: 0,
                  weapon: 0,
                  forwardmove: 0,
                  rightmove: 0,
                  upmove: 0,};
    let mut oldAngles: vec3_t = [0.; 3];
    oldAngles[0usize] = cl.viewangles[0usize];
    oldAngles[1usize] = cl.viewangles[1usize];
    oldAngles[2usize] = cl.viewangles[2usize];
    CL_AdjustAngles();
    memset(&mut cmd as *mut usercmd_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    CL_CmdButtons(&mut cmd);
    CL_KeyMove(&mut cmd);
    CL_MouseMove(&mut cmd);
    CL_JoystickMove(&mut cmd);
    if cl.viewangles[0usize] - oldAngles[0usize] > 90i32 as libc::c_float {
        cl.viewangles[0usize] = oldAngles[0usize] + 90i32 as libc::c_float
    } else if oldAngles[0usize] - cl.viewangles[0usize] >
                  90i32 as libc::c_float {
        cl.viewangles[0usize] = oldAngles[0usize] - 90i32 as libc::c_float
    }
    CL_FinishMove(&mut cmd);
    if 0 != (*cl_debugMove).integer {
        if (*cl_debugMove).integer == 1i32 {
            SCR_DebugGraph(fabs((cl.viewangles[1usize] - oldAngles[1usize]) as
                                    libc::c_double) as libc::c_float);
        }
        if (*cl_debugMove).integer == 2i32 {
            SCR_DebugGraph(fabs((cl.viewangles[0usize] - oldAngles[0usize]) as
                                    libc::c_double) as libc::c_float);
        }
    }
    return cmd;
}
/*
==============
CL_FinishMove
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FinishMove(mut cmd: *mut usercmd_t) {
    let mut i: libc::c_int = 0;
    (*cmd).weapon = cl.cgameUserCmdValue as byte;
    (*cmd).serverTime = cl.serverTime;
    i = 0i32;
    while i < 3i32 {
        (*cmd).angles[i as usize] =
            (cl.viewangles[i as usize] * 65536i32 as libc::c_float /
                 360i32 as libc::c_float) as libc::c_int & 65535i32;
        i += 1
    };
}
/*
=================
CL_JoystickMove
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_JoystickMove(mut cmd: *mut usercmd_t) {
    let mut anglespeed: libc::c_float = 0.;
    let mut yaw: libc::c_float =
        (*j_yaw).value *
            cl.joystickAxis[(*j_yaw_axis).integer as usize] as libc::c_float;
    let mut right: libc::c_float =
        (*j_side).value *
            cl.joystickAxis[(*j_side_axis).integer as usize] as libc::c_float;
    let mut forward: libc::c_float =
        (*j_forward).value *
            cl.joystickAxis[(*j_forward_axis).integer as usize] as
                libc::c_float;
    let mut pitch: libc::c_float =
        (*j_pitch).value *
            cl.joystickAxis[(*j_pitch_axis).integer as usize] as
                libc::c_float;
    let mut up: libc::c_float =
        (*j_up).value *
            cl.joystickAxis[(*j_up_axis).integer as usize] as libc::c_float;
    if 0 ==
           in_speed.active as libc::c_uint ^ (*cl_run).integer as libc::c_uint
       {
        (*cmd).buttons |= 16i32
    }
    if 0 != in_speed.active as u64 {
        anglespeed =
            (0.001f64 * cls.frametime as libc::c_double *
                 (*cl_anglespeedkey).value as libc::c_double) as libc::c_float
    } else {
        anglespeed =
            (0.001f64 * cls.frametime as libc::c_double) as libc::c_float
    }
    if 0 == in_strafe.active as u64 {
        cl.viewangles[1usize] += anglespeed * yaw;
        (*cmd).rightmove =
            ClampChar((*cmd).rightmove as libc::c_int + right as libc::c_int)
    } else {
        cl.viewangles[1usize] += anglespeed * right;
        (*cmd).rightmove =
            ClampChar((*cmd).rightmove as libc::c_int + yaw as libc::c_int)
    }
    if 0 != in_mlooking as u64 {
        cl.viewangles[0usize] += anglespeed * forward;
        (*cmd).forwardmove =
            ClampChar((*cmd).forwardmove as libc::c_int +
                          pitch as libc::c_int)
    } else {
        cl.viewangles[0usize] += anglespeed * pitch;
        (*cmd).forwardmove =
            ClampChar((*cmd).forwardmove as libc::c_int +
                          forward as libc::c_int)
    }
    (*cmd).upmove =
        ClampChar((*cmd).upmove as libc::c_int + up as libc::c_int);
}
/*
=================
CL_MouseMove
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_MouseMove(mut cmd: *mut usercmd_t) {
    let mut mx: libc::c_float = 0.;
    let mut my: libc::c_float = 0.;
    if 0 != (*m_filter).integer {
        mx =
            (cl.mouseDx[0usize] + cl.mouseDx[1usize]) as libc::c_float *
                0.5f32;
        my =
            (cl.mouseDy[0usize] + cl.mouseDy[1usize]) as libc::c_float *
                0.5f32
    } else {
        mx = cl.mouseDx[cl.mouseIndex as usize] as libc::c_float;
        my = cl.mouseDy[cl.mouseIndex as usize] as libc::c_float
    }
    cl.mouseIndex ^= 1i32;
    cl.mouseDx[cl.mouseIndex as usize] = 0i32;
    cl.mouseDy[cl.mouseIndex as usize] = 0i32;
    if mx == 0.0f32 && my == 0.0f32 { return }
    if (*cl_mouseAccel).value != 0.0f32 {
        if (*cl_mouseAccelStyle).integer == 0i32 {
            let mut accelSensitivity: libc::c_float = 0.;
            let mut rate: libc::c_float = 0.;
            rate =
                (sqrt((mx * mx + my * my) as libc::c_double) /
                     frame_msec as libc::c_float as libc::c_double) as
                    libc::c_float;
            accelSensitivity =
                (*cl_sensitivity).value + rate * (*cl_mouseAccel).value;
            mx *= accelSensitivity;
            my *= accelSensitivity;
            if 0 != (*cl_showMouseRate).integer {
                Com_Printf(b"rate: %f, accelSensitivity: %f\n\x00" as
                               *const u8 as *const libc::c_char,
                           rate as libc::c_double,
                           accelSensitivity as libc::c_double);
            }
        } else {
            let mut rate_0: [libc::c_float; 2] = [0.; 2];
            let mut power: [libc::c_float; 2] = [0.; 2];
            rate_0[0usize] =
                (fabs(mx as libc::c_double) /
                     frame_msec as libc::c_float as libc::c_double) as
                    libc::c_float;
            rate_0[1usize] =
                (fabs(my as libc::c_double) /
                     frame_msec as libc::c_float as libc::c_double) as
                    libc::c_float;
            power[0usize] =
                powf(rate_0[0usize] / (*cl_mouseAccelOffset).value,
                     (*cl_mouseAccel).value);
            power[1usize] =
                powf(rate_0[1usize] / (*cl_mouseAccelOffset).value,
                     (*cl_mouseAccel).value);
            mx =
                (*cl_sensitivity).value *
                    (mx +
                         if mx < 0i32 as libc::c_float {
                             -power[0usize]
                         } else { power[0usize] } *
                             (*cl_mouseAccelOffset).value);
            my =
                (*cl_sensitivity).value *
                    (my +
                         if my < 0i32 as libc::c_float {
                             -power[1usize]
                         } else { power[1usize] } *
                             (*cl_mouseAccelOffset).value);
            if 0 != (*cl_showMouseRate).integer {
                Com_Printf(b"ratex: %f, ratey: %f, powx: %f, powy: %f\n\x00"
                               as *const u8 as *const libc::c_char,
                           rate_0[0usize] as libc::c_double,
                           rate_0[1usize] as libc::c_double,
                           power[0usize] as libc::c_double,
                           power[1usize] as libc::c_double);
            }
        }
    } else { mx *= (*cl_sensitivity).value; my *= (*cl_sensitivity).value }
    mx *= cl.cgameSensitivity;
    my *= cl.cgameSensitivity;
    if 0 != in_strafe.active as u64 {
        (*cmd).rightmove =
            ClampChar(((*cmd).rightmove as libc::c_int as libc::c_float +
                           (*m_side).value * mx) as libc::c_int)
    } else { cl.viewangles[1usize] -= (*m_yaw).value * mx }
    if (0 != in_mlooking as libc::c_uint || 0 != (*cl_freelook).integer) &&
           0 == in_strafe.active as u64 {
        cl.viewangles[0usize] += (*m_pitch).value * my
    } else {
        (*cmd).forwardmove =
            ClampChar(((*cmd).forwardmove as libc::c_int as libc::c_float -
                           (*m_forward).value * my) as libc::c_int)
    };
}
/*
================
CL_KeyMove

Sets the usercmd_t based on key states
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_KeyMove(mut cmd: *mut usercmd_t) {
    let mut movespeed: libc::c_int = 0;
    let mut forward: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    if 0 !=
           in_speed.active as libc::c_uint ^ (*cl_run).integer as libc::c_uint
       {
        movespeed = 127i32;
        (*cmd).buttons &= !16i32
    } else { (*cmd).buttons |= 16i32; movespeed = 64i32 }
    forward = 0i32;
    side = 0i32;
    up = 0i32;
    if 0 != in_strafe.active as u64 {
        side =
            (side as libc::c_float +
                 movespeed as libc::c_float * CL_KeyState(&mut in_right)) as
                libc::c_int;
        side =
            (side as libc::c_float -
                 movespeed as libc::c_float * CL_KeyState(&mut in_left)) as
                libc::c_int
    }
    side =
        (side as libc::c_float +
             movespeed as libc::c_float * CL_KeyState(&mut in_moveright)) as
            libc::c_int;
    side =
        (side as libc::c_float -
             movespeed as libc::c_float * CL_KeyState(&mut in_moveleft)) as
            libc::c_int;
    up =
        (up as libc::c_float +
             movespeed as libc::c_float * CL_KeyState(&mut in_up)) as
            libc::c_int;
    up =
        (up as libc::c_float -
             movespeed as libc::c_float * CL_KeyState(&mut in_down)) as
            libc::c_int;
    forward =
        (forward as libc::c_float +
             movespeed as libc::c_float * CL_KeyState(&mut in_forward)) as
            libc::c_int;
    forward =
        (forward as libc::c_float -
             movespeed as libc::c_float * CL_KeyState(&mut in_back)) as
            libc::c_int;
    (*cmd).forwardmove = ClampChar(forward);
    (*cmd).rightmove = ClampChar(side);
    (*cmd).upmove = ClampChar(up);
}
#[no_mangle]
pub unsafe extern "C" fn CL_KeyState(mut key: *mut kbutton_t)
 -> libc::c_float {
    let mut val: libc::c_float = 0.;
    let mut msec: libc::c_int = 0;
    msec = (*key).msec as libc::c_int;
    (*key).msec = 0i32 as libc::c_uint;
    if 0 != (*key).active as u64 {
        if 0 == (*key).downtime {
            msec = com_frameTime
        } else {
            msec =
                (msec as
                     libc::c_uint).wrapping_add((com_frameTime as
                                                     libc::c_uint).wrapping_sub((*key).downtime))
                    as libc::c_int as libc::c_int
        }
        (*key).downtime = com_frameTime as libc::c_uint
    }
    val = msec as libc::c_float / frame_msec as libc::c_float;
    if val < 0i32 as libc::c_float { val = 0i32 as libc::c_float }
    if val > 1i32 as libc::c_float { val = 1i32 as libc::c_float }
    return val;
}
/*
==============
CL_CmdButtons
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CmdButtons(mut cmd: *mut usercmd_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 15i32 {
        if 0 != in_buttons[i as usize].active as libc::c_uint ||
               0 != in_buttons[i as usize].wasPressed as libc::c_uint {
            (*cmd).buttons |= 1i32 << i
        }
        in_buttons[i as usize].wasPressed = qfalse;
        i += 1
    }
    if 0 != Key_GetCatcher() { (*cmd).buttons |= 2i32 }
    if 0 != anykeydown && Key_GetCatcher() == 0i32 {
        (*cmd).buttons |= 2048i32
    };
}
//==========================================================================
/*
================
CL_AdjustAngles

Moves the local angle positions
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AdjustAngles() {
    let mut speed: libc::c_float = 0.;
    if 0 != in_speed.active as u64 {
        speed =
            (0.001f64 * cls.frametime as libc::c_double *
                 (*cl_anglespeedkey).value as libc::c_double) as libc::c_float
    } else {
        speed = (0.001f64 * cls.frametime as libc::c_double) as libc::c_float
    }
    if 0 == in_strafe.active as u64 {
        cl.viewangles[1usize] -=
            speed * (*cl_yawspeed).value * CL_KeyState(&mut in_right);
        cl.viewangles[1usize] +=
            speed * (*cl_yawspeed).value * CL_KeyState(&mut in_left)
    }
    cl.viewangles[0usize] -=
        speed * (*cl_pitchspeed).value * CL_KeyState(&mut in_lookup);
    cl.viewangles[0usize] +=
        speed * (*cl_pitchspeed).value * CL_KeyState(&mut in_lookdown);
}
#[no_mangle]
pub static mut old_com_frameTime: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn IN_Button15Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(15isize));
}
#[no_mangle]
pub unsafe extern "C" fn IN_Button15Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(15isize));
}