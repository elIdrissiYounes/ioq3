#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, extern_types, libc)]
extern crate libc;
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
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
    pub type fileHandle_t = libc::c_int;
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
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
        //=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Info_SetValueForKey(s: *mut libc::c_char,
                                   key: *const libc::c_char,
                                   value: *const libc::c_char);
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
    pub type vm_t = vm_s;
    use super::q_shared_h::{qboolean, byte, cvar_t};
    use super::{libc};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn MSG_BeginReadingOOB(sb: *mut msg_t);
        #[no_mangle]
        pub fn MSG_ReadShort(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadLong(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadStringLine(sb: *mut msg_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn NET_OutOfBandPrint(net_socket: netsrc_t, adr: netadr_t,
                                  format: *const libc::c_char, ...);
        #[no_mangle]
        pub fn NET_CompareBaseAdr(a: netadr_t, b: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_AdrToStringwPort(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_StringToAdr(s: *const libc::c_char, a: *mut netadr_t,
                               family: netadrtype_t) -> libc::c_int;
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgsFrom(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_Cmd() -> *mut libc::c_char;
        // The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
        #[no_mangle]
        pub fn Cmd_TokenizeString(text: *const libc::c_char);
        // Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
        #[no_mangle]
        pub fn Cmd_ExecuteString(text: *const libc::c_char);
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        #[no_mangle]
        pub fn Cvar_VariableIntegerValue(var_name: *const libc::c_char)
         -> libc::c_int;
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_InfoString(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_InfoString_Big(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub static mut cvar_modifiedFlags: libc::c_int;
        #[no_mangle]
        pub fn Com_BeginRedirect(buffer: *mut libc::c_char,
                                 buffersize: libc::c_int,
                                 flush:
                                     Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_char)
                                                -> ()>);
        #[no_mangle]
        pub fn Com_EndRedirect();
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub static mut com_dedicated: *mut cvar_t;
        #[no_mangle]
        pub static mut com_speeds: *mut cvar_t;
        #[no_mangle]
        pub static mut com_timescale: *mut cvar_t;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        // both client and server must agree to pause
        #[no_mangle]
        pub static mut cl_paused: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_paused: *mut cvar_t;
        #[no_mangle]
        pub static mut com_gamename: *mut cvar_t;
        #[no_mangle]
        pub static mut com_protocol: *mut cvar_t;
        #[no_mangle]
        pub static mut com_legacyprotocol: *mut cvar_t;
        // com_speeds times
        #[no_mangle]
        pub static mut time_game: libc::c_int;
        #[no_mangle]
        pub fn SV_Shutdown(finalmsg: *mut libc::c_char);
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Huff_Decompress(buf: *mut msg_t, offset: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/server/server.h"]
pub mod server_h {
    // this structure will be cleared only when the game dll changes
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverStatic_t {
        pub initialized: qboolean,
        pub time: libc::c_int,
        pub snapFlagServerBit: libc::c_int,
        pub clients: *mut client_t,
        pub numSnapshotEntities: libc::c_int,
        pub nextSnapshotEntities: libc::c_int,
        pub snapshotEntities: *mut entityState_t,
        pub nextHeartbeatTime: libc::c_int,
        pub challenges: [challenge_t; 2048],
        pub redirectAddress: netadr_t,
        pub authorizeAddress: netadr_t,
        pub masterResolveTime: [libc::c_int; 5],
    }
    //=============================================================================
    // MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
    // Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct challenge_t {
        pub adr: netadr_t,
        pub challenge: libc::c_int,
        pub clientChallenge: libc::c_int,
        pub time: libc::c_int,
        pub pingTime: libc::c_int,
        pub firstTime: libc::c_int,
        pub wasrefused: qboolean,
        pub connected: qboolean,
    }
    pub type client_t = client_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct client_s {
        pub state: clientState_t,
        pub userinfo: [libc::c_char; 1024],
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableSent: libc::c_int,
        pub messageAcknowledge: libc::c_int,
        pub gamestateMessageNum: libc::c_int,
        pub challenge: libc::c_int,
        pub lastUsercmd: usercmd_t,
        pub lastMessageNum: libc::c_int,
        pub lastClientCommand: libc::c_int,
        pub lastClientCommandString: [libc::c_char; 1024],
        pub gentity: *mut sharedEntity_t,
        pub name: [libc::c_char; 32],
        pub downloadName: [libc::c_char; 64],
        pub download: fileHandle_t,
        pub downloadSize: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadClientBlock: libc::c_int,
        pub downloadCurrentBlock: libc::c_int,
        pub downloadXmitBlock: libc::c_int,
        pub downloadBlocks: [*mut libc::c_uchar; 48],
        pub downloadBlockSize: [libc::c_int; 48],
        pub downloadEOF: qboolean,
        pub downloadSendTime: libc::c_int,
        pub deltaMessage: libc::c_int,
        pub nextReliableTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub lastConnectTime: libc::c_int,
        pub lastSnapshotTime: libc::c_int,
        pub rateDelayed: qboolean,
        pub timeoutCount: libc::c_int,
        pub frames: [clientSnapshot_t; 32],
        pub ping: libc::c_int,
        pub rate: libc::c_int,
        pub snapshotMsec: libc::c_int,
        pub pureAuthentic: libc::c_int,
        pub gotCP: qboolean,
        pub netchan: netchan_t,
        pub netchan_start_queue: *mut netchan_buffer_t,
        pub netchan_end_queue: *mut *mut netchan_buffer_t,
        pub hasVoip: qboolean,
        pub muteAllVoip: qboolean,
        pub ignoreVoipFromClient: [qboolean; 64],
        pub voipPacket: [*mut voipServerPacket_t; 64],
        pub queuedVoipPackets: libc::c_int,
        pub queuedVoipIndex: libc::c_int,
        pub oldServerTime: libc::c_int,
        pub csUpdated: [qboolean; 1024],
        pub compat: qboolean,
    }
    pub type voipServerPacket_t = voipServerPacket_s;
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
// server.h
    //=============================================================================
    // !!! MUST NOT CHANGE, SERVER AND
    // GAME BOTH REFERENCE !!!
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct voipServerPacket_s {
        pub generation: libc::c_int,
        pub sequence: libc::c_int,
        pub frames: libc::c_int,
        pub len: libc::c_int,
        pub sender: libc::c_int,
        pub flags: libc::c_int,
        pub data: [byte; 4000],
    }
    pub type netchan_buffer_t = netchan_buffer_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_buffer_s {
        pub msg: msg_t,
        pub msgBuffer: [byte; 16384],
        pub clientCommandString: [libc::c_char; 1024],
        pub next: *mut netchan_buffer_s,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientSnapshot_t {
        pub areabytes: libc::c_int,
        pub areabits: [byte; 32],
        pub ps: playerState_t,
        pub num_entities: libc::c_int,
        pub first_entity: libc::c_int,
        pub messageSent: libc::c_int,
        pub messageAcked: libc::c_int,
        pub messageSize: libc::c_int,
    }
    pub type clientState_t = libc::c_uint;
    // client is fully in game
    pub const CS_ACTIVE: clientState_t = 4;
    // gamestate has been sent, but client hasn't sent a usercmd
    pub const CS_PRIMED: clientState_t = 3;
    // connection for a couple seconds
    // has been assigned to a client_t, but no gamestate yet
    pub const CS_CONNECTED: clientState_t = 2;
    // client has been disconnected, but don't reuse
    pub const CS_ZOMBIE: clientState_t = 1;
    // can be reused for a new connection
    pub const CS_FREE: clientState_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct server_t {
        pub state: serverState_t,
        pub restarting: qboolean,
        pub serverId: libc::c_int,
        pub restartedServerId: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub checksumFeedServerId: libc::c_int,
        pub snapshotCounter: libc::c_int,
        pub timeResidual: libc::c_int,
        pub nextFrameTime: libc::c_int,
        pub configstrings: [*mut libc::c_char; 1024],
        pub svEntities: [svEntity_t; 1024],
        pub entityParsePoint: *mut libc::c_char,
        pub gentities: *mut sharedEntity_t,
        pub gentitySize: libc::c_int,
        pub num_entities: libc::c_int,
        pub gameClients: *mut playerState_t,
        pub gameClientSize: libc::c_int,
        pub restartTime: libc::c_int,
        pub time: libc::c_int,
    }
    pub type svEntity_t = svEntity_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct svEntity_s {
        pub worldSector: *mut worldSector_s,
        pub nextEntityInWorldSector: *mut svEntity_s,
        pub baseline: entityState_t,
        pub numClusters: libc::c_int,
        pub clusternums: [libc::c_int; 16],
        pub lastCluster: libc::c_int,
        pub areanum: libc::c_int,
        pub areanum2: libc::c_int,
        pub snapshotCounter: libc::c_int,
    }
    pub type serverState_t = libc::c_uint;
    // actively running
    pub const SS_GAME: serverState_t = 2;
    // spawning level entities
    pub const SS_LOADING: serverState_t = 1;
    // no map loaded
    pub const SS_DEAD: serverState_t = 0;
    //===========================================================
    //
// sv_main.c
//
    pub type leakyBucket_t = leakyBucket_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct leakyBucket_s {
        pub type_0: netadrtype_t,
        pub ipv: unnamed,
        pub lastTime: libc::c_int,
        pub burst: libc::c_schar,
        pub hash: libc::c_long,
        pub prev: *mut leakyBucket_t,
        pub next: *mut leakyBucket_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed {
        pub _4: [byte; 4],
        pub _6: [byte; 16],
    }
    // Structure for managing bans
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverBan_t {
        //
// sv_bot.c
//
        //
// sv_init.c
//
        //
// sv_client.c
//
        // the server looks at a sharedEntity, which is the start of the game's gentity_t structure
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
        // g_public.h -- game module information visible to server
        // entity->svFlags
// the server does not know how to interpret most of the values
// in entityStates (level eType), so the game must explicitly flag
// special server behaviors
        // don't send entity to clients, even if it has effects
        // TTimo
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=551
        // set if the entity is a bot
        // send to all connected clients
        // merge a second pvs at origin2 into snapshots
        // entity->r.currentOrigin instead of entity->s.origin
        // for link position (missiles and movers)
        // only send to a single client (entityShared_t->singleClient)
        // don't send CS_SERVERINFO updates to this client
        // so that it can be updated for ping tools without
											// lagging clients
        // use capsule for collision detection instead of bbox
        // send entity to everyone but one client
        // (entityShared_t->singleClient)
        //===============================================================
        // ( int levelTime );
        //
// functions exported by the game subsystem
//
        // ConsoleCommand will be called when a command has been issued
	// that is not recognized as a builtin function.
	// The game can issue trap_argc() / trap_argv() commands to get the command
	// and parameters.  Return qfalse if the game doesn't recognize it as a command.
        // ( int time );
        pub ip: netadr_t,
        pub subnet: libc::c_int,
        pub isexception: qboolean,
    }
    use super::q_shared_h::{qboolean, entityState_t, usercmd_t, fileHandle_t,
                            byte, playerState_t, cvar_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, msg_t, netadrtype_t, vm_t};
    use super::g_public_h::{sharedEntity_t};
    extern "C" {
        pub type worldSector_s;
        #[no_mangle]
        pub fn SV_SendClientMessages();
        #[no_mangle]
        pub fn SV_DropClient(drop_0: *mut client_t,
                             reason: *const libc::c_char);
        #[no_mangle]
        pub fn SV_BotFrame(time: libc::c_int);
        #[no_mangle]
        pub fn SV_GameClientNum(num: libc::c_int) -> *mut playerState_t;
        #[no_mangle]
        pub fn SV_SetConfigstring(index: libc::c_int,
                                  val: *const libc::c_char);
        #[no_mangle]
        pub fn SV_ExecuteClientMessage(cl: *mut client_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn SV_Netchan_Process(client: *mut client_t, msg: *mut msg_t)
         -> qboolean;
        #[no_mangle]
        pub fn SV_AuthorizeIpPacket(from: netadr_t);
        #[no_mangle]
        pub fn SV_DirectConnect(from: netadr_t);
        #[no_mangle]
        pub fn SV_GetChallenge(from: netadr_t);
        #[no_mangle]
        pub fn SV_SendDownloadMessages() -> libc::c_int;
        #[no_mangle]
        pub fn SV_SendQueuedMessages() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/game/g_public.h"]
pub mod g_public_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sharedEntity_t {
        pub s: entityState_t,
        pub r: entityShared_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct entityShared_t {
        pub unused: entityState_t,
        pub linked: qboolean,
        pub linkcount: libc::c_int,
        pub svFlags: libc::c_int,
        pub singleClient: libc::c_int,
        pub bmodel: qboolean,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub contents: libc::c_int,
        pub absmin: vec3_t,
        pub absmax: vec3_t,
        pub currentOrigin: vec3_t,
        pub currentAngles: vec3_t,
        pub ownerNum: libc::c_int,
    }
    pub const GAME_RUN_FRAME: unnamed_0 = 8;
    pub type unnamed_0 = libc::c_uint;
    pub const BOTAI_START_FRAME: unnamed_0 = 10;
    // ( void );
    pub const GAME_CONSOLE_COMMAND: unnamed_0 = 9;
    // ( int clientNum );
    pub const GAME_CLIENT_THINK: unnamed_0 = 7;
    // ( int clientNum );
    pub const GAME_CLIENT_COMMAND: unnamed_0 = 6;
    // ( int clientNum );
    pub const GAME_CLIENT_DISCONNECT: unnamed_0 = 5;
    // ( int clientNum );
    pub const GAME_CLIENT_USERINFO_CHANGED: unnamed_0 = 4;
    // return NULL if the client is allowed to connect, otherwise return
	// a text string with the reason for denial
    // ( int clientNum );
    pub const GAME_CLIENT_BEGIN: unnamed_0 = 3;
    // ( int clientNum, qboolean firstTime, qboolean isBot );
    pub const GAME_CLIENT_CONNECT: unnamed_0 = 2;
    // init and shutdown will be called every single level
	// The game should call G_GET_ENTITY_TOKEN to parse through all the
	// entity configuration text and spawn gentities.
    // (void);
    pub const GAME_SHUTDOWN: unnamed_0 = 1;
    // ( int levelTime, int randomSeed, int restart );
    pub const GAME_INIT: unnamed_0 = 0;
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
    use super::{libc};
}
#[header_src =
      "ioq3/code/game/bg_public.h"]
pub mod bg_public_h {
    // single player ffa
    pub const GT_SINGLE_PLAYER: unnamed_1 = 2;
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
// bg_public.h -- definitions shared by both the server game and client game modules
    // because games can change separately from the main system version, we need a
// second version that must match between game and cgame
    // item sizes are needed for client side pickup detection
    // for the CS_SCORES[12] when only one player is present
    // 30 seconds before vote times out
    //
// config strings are a general means of communicating variable length strings
// from the server to all connected clients.
//
    // CS_SERVERINFO and CS_SYSTEMINFO are defined in q_shared.h
    // from the map worldspawn's message field
    // g_motd string for server message of the day
    // server time when the match will be restarted
    // so the timer only shows the current level
    // when 1, fraglimit/timelimit has been hit and intermission will start in a second or two
    // string indicating flag status in CTF
    // string of 0's and 1's that tell which items are present
    pub type unnamed_1 = libc::c_uint;
    pub const GT_MAX_GAME_TYPE: unnamed_1 = 8;
    pub const GT_HARVESTER: unnamed_1 = 7;
    pub const GT_OBELISK: unnamed_1 = 6;
    pub const GT_1FCTF: unnamed_1 = 5;
    // capture the flag
    pub const GT_CTF: unnamed_1 = 4;
    //-- team games go after this --
    // team deathmatch
    pub const GT_TEAM: unnamed_1 = 3;
    // one on one tournament
    pub const GT_TOURNAMENT: unnamed_1 = 1;
    // free for all
    pub const GT_FFA: unnamed_1 = 0;
    use super::{libc};
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
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/qcommon/q_platform.h"]
pub mod q_platform_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn ShortSwap(l: libc::c_short) -> libc::c_short;
    }
}
#[header_src =
      "ioq3/code/server/sv_main.c"]
pub mod sv_main_c {
    use super::{libc};
    use super::qcommon_h::{netadr_t};
}
use self::stddef_h::{size_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cvar_s, cvar_t, playerState_s, playerState_t,
                       usercmd_s, usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, Com_sprintf, Q_stricmp, Q_strncmp,
                       Q_strncpyz, Q_strcat, va, Info_SetValueForKey,
                       Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      vm_t, vm_s, MSG_BeginReadingOOB, MSG_ReadShort,
                      MSG_ReadLong, MSG_ReadStringLine, NET_OutOfBandPrint,
                      NET_CompareBaseAdr, NET_AdrToString,
                      NET_AdrToStringwPort, NET_StringToAdr, VM_Call,
                      Cbuf_AddText, Cmd_Argv, Cmd_ArgsFrom, Cmd_Cmd,
                      Cmd_TokenizeString, Cmd_ExecuteString, Cvar_Set,
                      Cvar_VariableValue, Cvar_VariableIntegerValue,
                      Cvar_VariableString, Cvar_InfoString,
                      Cvar_InfoString_Big, cvar_modifiedFlags,
                      Com_BeginRedirect, Com_EndRedirect, Com_DPrintf,
                      com_dedicated, com_speeds, com_timescale,
                      com_sv_running, cl_paused, sv_paused, com_gamename,
                      com_protocol, com_legacyprotocol, time_game,
                      SV_Shutdown, Sys_Milliseconds, Huff_Decompress};
use self::server_h::{serverStatic_t, challenge_t, client_t, client_s,
                     voipServerPacket_t, voipServerPacket_s, netchan_buffer_t,
                     netchan_buffer_s, clientSnapshot_t, clientState_t,
                     CS_ACTIVE, CS_PRIMED, CS_CONNECTED, CS_ZOMBIE, CS_FREE,
                     server_t, svEntity_t, svEntity_s, serverState_t, SS_GAME,
                     SS_LOADING, SS_DEAD, leakyBucket_t, leakyBucket_s,
                     unnamed, serverBan_t, worldSector_s,
                     SV_SendClientMessages, SV_DropClient, SV_BotFrame,
                     SV_GameClientNum, SV_SetConfigstring,
                     SV_ExecuteClientMessage, SV_Netchan_Process,
                     SV_AuthorizeIpPacket, SV_DirectConnect, SV_GetChallenge,
                     SV_SendDownloadMessages, SV_SendQueuedMessages};
use self::g_public_h::{sharedEntity_t, entityShared_t, GAME_RUN_FRAME,
                       unnamed_0, BOTAI_START_FRAME, GAME_CONSOLE_COMMAND,
                       GAME_CLIENT_THINK, GAME_CLIENT_COMMAND,
                       GAME_CLIENT_DISCONNECT, GAME_CLIENT_USERINFO_CHANGED,
                       GAME_CLIENT_BEGIN, GAME_CLIENT_CONNECT, GAME_SHUTDOWN,
                       GAME_INIT};
use self::bg_public_h::{GT_SINGLE_PLAYER, unnamed_1, GT_MAX_GAME_TYPE,
                        GT_HARVESTER, GT_OBELISK, GT_1FCTF, GT_CTF, GT_TEAM,
                        GT_TOURNAMENT, GT_FFA};
use self::string_h::{memcpy, memset, memcmp, strcpy, strcmp, strlen};
use self::q_platform_h::{ShortSwap};
#[no_mangle]
pub unsafe extern "C" fn SV_Frame(mut msec: libc::c_int) {
    let mut frameMsec: libc::c_int = 0;
    let mut startTime: libc::c_int = 0;
    if 0 != (*sv_killserver).integer {
        SV_Shutdown(b"Server was killed\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        Cvar_Set(b"sv_killserver\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
        return
    }
    if 0 == (*com_sv_running).integer { return }
    if 0 != SV_CheckPaused() as u64 { return }
    if (*sv_fps).integer < 1i32 {
        Cvar_Set(b"sv_fps\x00" as *const u8 as *const libc::c_char,
                 b"10\x00" as *const u8 as *const libc::c_char);
    }
    frameMsec =
        ((1000i32 / (*sv_fps).integer) as libc::c_float *
             (*com_timescale).value) as libc::c_int;
    if frameMsec < 1i32 {
        Cvar_Set(b"timescale\x00" as *const u8 as *const libc::c_char,
                 va(b"%f\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                    ((*sv_fps).integer as libc::c_float / 1000.0f32) as
                        libc::c_double));
        frameMsec = 1i32
    }
    sv.timeResidual += msec;
    if 0 == (*com_dedicated).integer {
        SV_BotFrame(sv.time + sv.timeResidual);
    }
    if svs.time > 0x70000000i32 {
        SV_Shutdown(b"Restarting server due to time wrapping\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char);
        Cbuf_AddText(va(b"map %s\n\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        Cvar_VariableString(b"mapname\x00" as *const u8 as
                                                *const libc::c_char)));
        return
    }
    if svs.nextSnapshotEntities >= 0x7ffffffei32 - svs.numSnapshotEntities {
        SV_Shutdown(b"Restarting server due to numSnapshotEntities wrapping\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char);
        Cbuf_AddText(va(b"map %s\n\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        Cvar_VariableString(b"mapname\x00" as *const u8 as
                                                *const libc::c_char)));
        return
    }
    if 0 != sv.restartTime && sv.time >= sv.restartTime {
        sv.restartTime = 0i32;
        Cbuf_AddText(b"map_restart 0\n\x00" as *const u8 as
                         *const libc::c_char);
        return
    }
    if 0 != cvar_modifiedFlags & 0x4i32 {
        SV_SetConfigstring(0i32, Cvar_InfoString(0x4i32));
        cvar_modifiedFlags &= !0x4i32
    }
    if 0 != cvar_modifiedFlags & 0x8i32 {
        SV_SetConfigstring(1i32, Cvar_InfoString_Big(0x8i32));
        cvar_modifiedFlags &= !0x8i32
    }
    if 0 != (*com_speeds).integer {
        startTime = Sys_Milliseconds()
    } else { startTime = 0i32 }
    SV_CalcPings();
    if 0 != (*com_dedicated).integer { SV_BotFrame(sv.time); }
    while sv.timeResidual >= frameMsec {
        sv.timeResidual -= frameMsec;
        svs.time += frameMsec;
        sv.time += frameMsec;
        VM_Call(gvm, GAME_RUN_FRAME as libc::c_int, sv.time);
    }
    if 0 != (*com_speeds).integer {
        time_game = Sys_Milliseconds() - startTime
    }
    SV_CheckTimeouts();
    SV_SendClientMessages();
    SV_MasterHeartbeat(b"DarkPlaces\x00" as *const u8 as *const libc::c_char);
}
/*
==============================================================================

MASTER SERVER FUNCTIONS

==============================================================================
*/
/*
================
SV_MasterHeartbeat

Send a message to the masters every few minutes to
let it know we are alive, and log information.
We will also have a heartbeat sent when a server
changes from empty to non-empty, and full to non-full,
but not on every player enter or exit.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_MasterHeartbeat(mut message:
                                                *const libc::c_char) {
    // [2] for v4 and v6 address for the same address string.
    static mut adr: [[netadr_t; 2]; 5] =
        [[netadr_t{type_0: NA_BAD,
                   ip: [0; 4],
                   ip6: [0; 16],
                   port: 0,
                   scope_id: 0,}; 2]; 5];
    let mut i: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut netenabled: libc::c_int = 0;
    netenabled =
        Cvar_VariableIntegerValue(b"net_enabled\x00" as *const u8 as
                                      *const libc::c_char);
    if com_dedicated.is_null() || (*com_dedicated).integer != 2i32 ||
           0 == netenabled & (0x1i32 | 0x2i32) {
        return
    }
    if svs.time < svs.nextHeartbeatTime { return }
    if 0 ==
           Q_stricmp((*com_gamename).string,
                     b"Quake3Arena\x00" as *const u8 as *const libc::c_char) {
        message = b"QuakeArena-1\x00" as *const u8 as *const libc::c_char
    }
    svs.nextHeartbeatTime = svs.time + 300i32 * 1000i32;
    i = 0i32;
    while i < 5i32 {
        if !(0 == *(*sv_master[i as usize]).string.offset(0isize)) {
            if 0 != (*sv_master[i as usize]).modified as libc::c_uint ||
                   svs.time > svs.masterResolveTime[i as usize] {
                (*sv_master[i as usize]).modified = qfalse;
                svs.masterResolveTime[i as usize] =
                    svs.time + 24i32 * 60i32 * 60i32 * 1000i32;
                if 0 != netenabled & 0x1i32 {
                    Com_Printf(b"Resolving %s (IPv4)\n\x00" as *const u8 as
                                   *const libc::c_char,
                               (*sv_master[i as usize]).string);
                    res =
                        NET_StringToAdr((*sv_master[i as usize]).string,
                                        &mut *(*adr.as_mut_ptr().offset(i as
                                                                            isize)).as_mut_ptr().offset(0isize),
                                        NA_IP);
                    if res == 2i32 {
                        adr[i as usize][0usize].port =
                            ShortSwap(27950i32 as libc::c_short) as
                                libc::c_ushort
                    }
                    if 0 != res {
                        Com_Printf(b"%s resolved to %s\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*sv_master[i as usize]).string,
                                   NET_AdrToStringwPort(adr[i as
                                                                usize][0usize]));
                    } else {
                        Com_Printf(b"%s has no IPv4 address.\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*sv_master[i as usize]).string);
                    }
                }
                if 0 != netenabled & 0x2i32 {
                    Com_Printf(b"Resolving %s (IPv6)\n\x00" as *const u8 as
                                   *const libc::c_char,
                               (*sv_master[i as usize]).string);
                    res =
                        NET_StringToAdr((*sv_master[i as usize]).string,
                                        &mut *(*adr.as_mut_ptr().offset(i as
                                                                            isize)).as_mut_ptr().offset(1isize),
                                        NA_IP6);
                    if res == 2i32 {
                        adr[i as usize][1usize].port =
                            ShortSwap(27950i32 as libc::c_short) as
                                libc::c_ushort
                    }
                    if 0 != res {
                        Com_Printf(b"%s resolved to %s\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*sv_master[i as usize]).string,
                                   NET_AdrToStringwPort(adr[i as
                                                                usize][1usize]));
                    } else {
                        Com_Printf(b"%s has no IPv6 address.\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*sv_master[i as usize]).string);
                    }
                }
            }
            if !(adr[i as usize][0usize].type_0 as libc::c_uint ==
                     NA_BAD as libc::c_int as libc::c_uint &&
                     adr[i as usize][1usize].type_0 as libc::c_uint ==
                         NA_BAD as libc::c_int as libc::c_uint) {
                Com_Printf(b"Sending heartbeat to %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           (*sv_master[i as usize]).string);
                if adr[i as usize][0usize].type_0 as libc::c_uint !=
                       NA_BAD as libc::c_int as libc::c_uint {
                    NET_OutOfBandPrint(NS_SERVER, adr[i as usize][0usize],
                                       b"heartbeat %s\n\x00" as *const u8 as
                                           *const libc::c_char, message);
                }
                if adr[i as usize][1usize].type_0 as libc::c_uint !=
                       NA_BAD as libc::c_int as libc::c_uint {
                    NET_OutOfBandPrint(NS_SERVER, adr[i as usize][1usize],
                                       b"heartbeat %s\n\x00" as *const u8 as
                                           *const libc::c_char, message);
                }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub static mut sv_master: [*mut cvar_t; 5] =
    [0 as *const cvar_t as *mut cvar_t; 5];
//=============================================================================
// persistant server info across maps
#[no_mangle]
pub static mut svs: serverStatic_t =
    serverStatic_t{initialized: qfalse,
                   time: 0,
                   snapFlagServerBit: 0,
                   clients: 0 as *const client_t as *mut client_t,
                   numSnapshotEntities: 0,
                   nextSnapshotEntities: 0,
                   snapshotEntities:
                       0 as *const entityState_t as *mut entityState_t,
                   nextHeartbeatTime: 0,
                   challenges:
                       [challenge_t{adr:
                                        netadr_t{type_0: NA_BAD,
                                                 ip: [0; 4],
                                                 ip6: [0; 16],
                                                 port: 0,
                                                 scope_id: 0,},
                                    challenge: 0,
                                    clientChallenge: 0,
                                    time: 0,
                                    pingTime: 0,
                                    firstTime: 0,
                                    wasrefused: qfalse,
                                    connected: qfalse,}; 2048],
                   redirectAddress:
                       netadr_t{type_0: NA_BAD,
                                ip: [0; 4],
                                ip6: [0; 16],
                                port: 0,
                                scope_id: 0,},
                   authorizeAddress:
                       netadr_t{type_0: NA_BAD,
                                ip: [0; 4],
                                ip6: [0; 16],
                                port: 0,
                                scope_id: 0,},
                   masterResolveTime: [0; 5],};
/*
==================
SV_CheckTimeouts

If a packet has not been received from a client for timeout->integer 
seconds, drop the conneciton.  Server time is used instead of
realtime to avoid dropping the local client while debugging.

When a client is normally dropped, the client_t goes into a zombie state
for a few seconds to make sure any final reliable message gets resent
if necessary
==================
*/
unsafe extern "C" fn SV_CheckTimeouts() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut droppoint: libc::c_int = 0;
    let mut zombiepoint: libc::c_int = 0;
    droppoint = svs.time - 1000i32 * (*sv_timeout).integer;
    zombiepoint = svs.time - 1000i32 * (*sv_zombietime).integer;
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if (*cl).lastPacketTime > svs.time { (*cl).lastPacketTime = svs.time }
        if (*cl).state as libc::c_uint ==
               CS_ZOMBIE as libc::c_int as libc::c_uint &&
               (*cl).lastPacketTime < zombiepoint {
            Com_DPrintf(b"Going from CS_ZOMBIE to CS_FREE for client %d\n\x00"
                            as *const u8 as *const libc::c_char, i);
            (*cl).state = CS_FREE
        } else if (*cl).state as libc::c_uint >=
                      CS_CONNECTED as libc::c_int as libc::c_uint &&
                      (*cl).lastPacketTime < droppoint {
            (*cl).timeoutCount += 1;
            if (*cl).timeoutCount > 5i32 {
                SV_DropClient(cl,
                              b"timed out\x00" as *const u8 as
                                  *const libc::c_char);
                (*cl).state = CS_FREE
            }
        } else { (*cl).timeoutCount = 0i32 }
        i += 1;
        cl = cl.offset(1isize)
    };
}
#[no_mangle]
pub static mut sv_maxclients: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_zombietime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_timeout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// cleared each map
#[no_mangle]
pub static mut sv: server_t =
    server_t{state: SS_DEAD,
             restarting: qfalse,
             serverId: 0,
             restartedServerId: 0,
             checksumFeed: 0,
             checksumFeedServerId: 0,
             snapshotCounter: 0,
             timeResidual: 0,
             nextFrameTime: 0,
             configstrings:
                 [0 as *const libc::c_char as *mut libc::c_char; 1024],
             svEntities:
                 [svEntity_s{worldSector:
                                 0 as *const worldSector_s as
                                     *mut worldSector_s,
                             nextEntityInWorldSector:
                                 0 as *const svEntity_s as *mut svEntity_s,
                             baseline:
                                 entityState_s{number: 0,
                                               eType: 0,
                                               eFlags: 0,
                                               pos:
                                                   trajectory_t{trType:
                                                                    TR_STATIONARY,
                                                                trTime: 0,
                                                                trDuration: 0,
                                                                trBase:
                                                                    [0.; 3],
                                                                trDelta:
                                                                    [0.; 3],},
                                               apos:
                                                   trajectory_t{trType:
                                                                    TR_STATIONARY,
                                                                trTime: 0,
                                                                trDuration: 0,
                                                                trBase:
                                                                    [0.; 3],
                                                                trDelta:
                                                                    [0.; 3],},
                                               time: 0,
                                               time2: 0,
                                               origin: [0.; 3],
                                               origin2: [0.; 3],
                                               angles: [0.; 3],
                                               angles2: [0.; 3],
                                               otherEntityNum: 0,
                                               otherEntityNum2: 0,
                                               groundEntityNum: 0,
                                               constantLight: 0,
                                               loopSound: 0,
                                               modelindex: 0,
                                               modelindex2: 0,
                                               clientNum: 0,
                                               frame: 0,
                                               solid: 0,
                                               event: 0,
                                               eventParm: 0,
                                               powerups: 0,
                                               weapon: 0,
                                               legsAnim: 0,
                                               torsoAnim: 0,
                                               generic1: 0,},
                             numClusters: 0,
                             clusternums: [0; 16],
                             lastCluster: 0,
                             areanum: 0,
                             areanum2: 0,
                             snapshotCounter: 0,}; 1024],
             entityParsePoint: 0 as *const libc::c_char as *mut libc::c_char,
             gentities: 0 as *const sharedEntity_t as *mut sharedEntity_t,
             gentitySize: 0,
             num_entities: 0,
             gameClients: 0 as *const playerState_t as *mut playerState_t,
             gameClientSize: 0,
             restartTime: 0,
             time: 0,};
// game virtual machine
#[no_mangle]
pub static mut gvm: *mut vm_t = 0 as *const vm_t as *mut vm_t;
/*
===================
SV_CalcPings

Updates the cl->ping variables
===================
*/
unsafe extern "C" fn SV_CalcPings() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut total: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if (*cl).state as libc::c_uint !=
               CS_ACTIVE as libc::c_int as libc::c_uint {
            (*cl).ping = 999i32
        } else if (*cl).gentity.is_null() {
            (*cl).ping = 999i32
        } else if 0 != (*(*cl).gentity).r.svFlags & 0x8i32 {
            (*cl).ping = 0i32
        } else {
            total = 0i32;
            count = 0i32;
            j = 0i32;
            while j < 32i32 {
                if !((*cl).frames[j as usize].messageAcked <= 0i32) {
                    delta =
                        (*cl).frames[j as usize].messageAcked -
                            (*cl).frames[j as usize].messageSent;
                    count += 1;
                    total += delta
                }
                j += 1
            }
            if 0 == count {
                (*cl).ping = 999i32
            } else {
                (*cl).ping = total / count;
                if (*cl).ping > 999i32 { (*cl).ping = 999i32 }
            }
            ps = SV_GameClientNum(i);
            (*ps).ping = (*cl).ping
        }
        i += 1
    };
}
#[no_mangle]
pub static mut sv_fps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
==================
SV_CheckPaused
==================
*/
unsafe extern "C" fn SV_CheckPaused() -> qboolean {
    let mut count: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    if 0 == (*cl_paused).integer { return qfalse }
    count = 0i32;
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if (*cl).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint &&
               (*cl).netchan.remoteAddress.type_0 as libc::c_uint !=
                   NA_BOT as libc::c_int as libc::c_uint {
            count += 1
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    if count > 1i32 {
        if 0 != (*sv_paused).integer {
            Cvar_Set(b"sv_paused\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char);
        }
        return qfalse
    }
    if 0 == (*sv_paused).integer {
        Cvar_Set(b"sv_paused\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char);
    }
    return qtrue;
}
#[no_mangle]
pub static mut sv_killserver: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn SV_PacketEvent(mut from: netadr_t,
                                        mut msg: *mut msg_t) {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut qport: libc::c_int = 0;
    if (*msg).cursize >= 4i32 && *((*msg).data as *mut libc::c_int) == -1i32 {
        SV_ConnectionlessPacket(from, msg);
        return
    }
    MSG_BeginReadingOOB(msg);
    MSG_ReadLong(msg);
    qport = MSG_ReadShort(msg) & 0xffffi32;
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as libc::c_uint ==
                 CS_FREE as libc::c_int as libc::c_uint) {
            if !(0 ==
                     NET_CompareBaseAdr(from, (*cl).netchan.remoteAddress) as
                         u64) {
                // it is possible to have multiple clients from a single IP
		// address, so they are differentiated by the qport variable
                if !((*cl).netchan.qport != qport) {
                    if (*cl).netchan.remoteAddress.port as libc::c_int !=
                           from.port as libc::c_int {
                        Com_Printf(b"SV_PacketEvent: fixing up a translated port\n\x00"
                                       as *const u8 as *const libc::c_char);
                        (*cl).netchan.remoteAddress.port = from.port
                    }
                    if 0 != SV_Netchan_Process(cl, msg) as u64 {
                        if (*cl).state as libc::c_uint !=
                               CS_ZOMBIE as libc::c_int as libc::c_uint {
                            (*cl).lastPacketTime = svs.time;
                            SV_ExecuteClientMessage(cl, msg);
                        }
                    }
                    return
                }
            }
        }
        i += 1;
        cl = cl.offset(1isize)
    };
}
/*
=================
SV_ConnectionlessPacket

A connectionless packet has four leading 0xff
characters to distinguish it from a game channel.
Clients that are in the game can still send
connectionless packets.
=================
*/
unsafe extern "C" fn SV_ConnectionlessPacket(mut from: netadr_t,
                                             mut msg: *mut msg_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    MSG_BeginReadingOOB(msg);
    MSG_ReadLong(msg);
    if 0 ==
           Q_strncmp(b"connect\x00" as *const u8 as *const libc::c_char,
                     &mut *(*msg).data.offset(4isize) as *mut byte as
                         *mut libc::c_char, 7i32) {
        Huff_Decompress(msg, 12i32);
    }
    s = MSG_ReadStringLine(msg);
    Cmd_TokenizeString(s);
    c = Cmd_Argv(0i32);
    Com_DPrintf(b"SV packet %s : %s\n\x00" as *const u8 as
                    *const libc::c_char, NET_AdrToString(from), c);
    if 0 == Q_stricmp(c, b"getstatus\x00" as *const u8 as *const libc::c_char)
       {
        SVC_Status(from);
    } else if 0 ==
                  Q_stricmp(c,
                            b"getinfo\x00" as *const u8 as
                                *const libc::c_char) {
        SVC_Info(from);
    } else if 0 ==
                  Q_stricmp(c,
                            b"getchallenge\x00" as *const u8 as
                                *const libc::c_char) {
        SV_GetChallenge(from);
    } else if 0 ==
                  Q_stricmp(c,
                            b"connect\x00" as *const u8 as
                                *const libc::c_char) {
        SV_DirectConnect(from);
    } else if 0 ==
                  Q_stricmp(c,
                            b"ipAuthorize\x00" as *const u8 as
                                *const libc::c_char) {
        SV_AuthorizeIpPacket(from);
    } else if 0 ==
                  Q_stricmp(c,
                            b"rcon\x00" as *const u8 as *const libc::c_char) {
        SVC_RemoteCommand(from, msg);
    } else if !(0 ==
                    Q_stricmp(c,
                              b"disconnect\x00" as *const u8 as
                                  *const libc::c_char)) {
        Com_DPrintf(b"bad connectionless packet from %s:\n%s\n\x00" as
                        *const u8 as *const libc::c_char,
                    NET_AdrToString(from), s);
    };
}
/*
===============
SVC_RemoteCommand

An rcon packet arrived from the network.
Shift down the remaining args
Redirect all printfs
===============
*/
unsafe extern "C" fn SVC_RemoteCommand(mut from: netadr_t,
                                       mut msg: *mut msg_t) {
    let mut valid: qboolean = qfalse;
    let mut remaining: [libc::c_char; 1024] = [0; 1024];
    // TTimo - scaled down to accumulate, but not overflow anything network wise, print wise etc.
	// (OOB messages are the bottleneck here)
    let mut sv_outputbuf: [libc::c_char; 1008] = [0; 1008];
    let mut cmd_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != SVC_RateLimitAddress(from, 10i32, 1000i32) as u64 {
        Com_DPrintf(b"SVC_RemoteCommand: rate limit from %s exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(from));
        return
    }
    if 0 == strlen((*sv_rconPassword).string) ||
           0 != strcmp(Cmd_Argv(1i32), (*sv_rconPassword).string) {
        static mut bucket: leakyBucket_t =
            leakyBucket_s{type_0: NA_BAD,
                          ipv: unnamed{_4: [0; 4],},
                          lastTime: 0,
                          burst: 0,
                          hash: 0,
                          prev:
                              0 as *const leakyBucket_t as *mut leakyBucket_t,
                          next:
                              0 as *const leakyBucket_t as
                                  *mut leakyBucket_t,};
        if 0 != SVC_RateLimit(&mut bucket, 10i32, 1000i32) as u64 {
            Com_DPrintf(b"SVC_RemoteCommand: rate limit exceeded, dropping request\n\x00"
                            as *const u8 as *const libc::c_char);
            return
        }
        valid = qfalse;
        Com_Printf(b"Bad rcon from %s: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_AdrToString(from),
                   Cmd_ArgsFrom(2i32));
    } else {
        valid = qtrue;
        Com_Printf(b"Rcon from %s: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_AdrToString(from),
                   Cmd_ArgsFrom(2i32));
    }
    svs.redirectAddress = from;
    Com_BeginRedirect(sv_outputbuf.as_mut_ptr(), 1024i32 - 16i32,
                      Some(SV_FlushRedirect));
    if 0 == strlen((*sv_rconPassword).string) {
        Com_Printf(b"No rconpassword set on the server.\n\x00" as *const u8 as
                       *const libc::c_char);
    } else if 0 == valid as u64 {
        Com_Printf(b"Bad rconpassword.\n\x00" as *const u8 as
                       *const libc::c_char);
    } else {
        remaining[0usize] = 0i32 as libc::c_char;
        cmd_aux = Cmd_Cmd();
        cmd_aux = cmd_aux.offset(4isize);
        while *cmd_aux.offset(0isize) as libc::c_int == ' ' as i32 {
            cmd_aux = cmd_aux.offset(1isize)
        }
        while 0 != *cmd_aux.offset(0isize) as libc::c_int &&
                  *cmd_aux.offset(0isize) as libc::c_int != ' ' as i32 {
            cmd_aux = cmd_aux.offset(1isize)
        }
        while *cmd_aux.offset(0isize) as libc::c_int == ' ' as i32 {
            cmd_aux = cmd_aux.offset(1isize)
        }
        Q_strcat(remaining.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int, cmd_aux);
        Cmd_ExecuteString(remaining.as_mut_ptr());
    }
    Com_EndRedirect();
}
#[no_mangle]
pub static mut sv_rconPassword: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
/*
================
SVC_FlushRedirect

================
*/
unsafe extern "C" fn SV_FlushRedirect(mut outputbuf: *mut libc::c_char) {
    NET_OutOfBandPrint(NS_SERVER, svs.redirectAddress,
                       b"print\n%s\x00" as *const u8 as *const libc::c_char,
                       outputbuf);
}
#[no_mangle]
pub unsafe extern "C" fn SVC_RateLimit(mut bucket: *mut leakyBucket_t,
                                       mut burst: libc::c_int,
                                       mut period: libc::c_int) -> qboolean {
    if !bucket.is_null() {
        let mut now: libc::c_int = Sys_Milliseconds();
        let mut interval: libc::c_int = now - (*bucket).lastTime;
        let mut expired: libc::c_int = interval / period;
        let mut expiredRemainder: libc::c_int = interval % period;
        if expired > (*bucket).burst as libc::c_int || interval < 0i32 {
            (*bucket).burst = 0i32 as libc::c_schar;
            (*bucket).lastTime = now
        } else {
            (*bucket).burst =
                ((*bucket).burst as libc::c_int - expired) as libc::c_schar;
            (*bucket).lastTime = now - expiredRemainder
        }
        if ((*bucket).burst as libc::c_int) < burst {
            (*bucket).burst += 1;
            return qfalse
        }
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn SVC_RateLimitAddress(mut from: netadr_t,
                                              mut burst: libc::c_int,
                                              mut period: libc::c_int)
 -> qboolean {
    let mut bucket: *mut leakyBucket_t =
        SVC_BucketForAddress(from, burst, period);
    return SVC_RateLimit(bucket, burst, period);
}
/*
================
SVC_BucketForAddress

Find or allocate a bucket for an address
================
*/
unsafe extern "C" fn SVC_BucketForAddress(mut address: netadr_t,
                                          mut burst: libc::c_int,
                                          mut period: libc::c_int)
 -> *mut leakyBucket_t {
    let mut bucket: *mut leakyBucket_t = 0 as *mut leakyBucket_t;
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_long = SVC_HashForAddress(address);
    let mut now: libc::c_int = Sys_Milliseconds();
    bucket = bucketHashes[hash as usize];
    while !bucket.is_null() {
        match (*bucket).type_0 as libc::c_uint {
            4 => {
                if memcmp((*bucket).ipv._4.as_mut_ptr() as
                              *const libc::c_void,
                          address.ip.as_mut_ptr() as *const libc::c_void,
                          4i32 as libc::c_ulong) == 0i32 {
                    return bucket
                }
            }
            5 => {
                if memcmp((*bucket).ipv._6.as_mut_ptr() as
                              *const libc::c_void,
                          address.ip6.as_mut_ptr() as *const libc::c_void,
                          16i32 as libc::c_ulong) == 0i32 {
                    return bucket
                }
            }
            _ => { }
        }
        bucket = (*bucket).next
    }
    i = 0i32;
    while i < 16384i32 {
        let mut interval: libc::c_int = 0;
        bucket =
            &mut *buckets.as_mut_ptr().offset(i as isize) as
                *mut leakyBucket_t;
        interval = now - (*bucket).lastTime;
        if (*bucket).lastTime > 0i32 &&
               (interval > burst * period || interval < 0i32) {
            if !(*bucket).prev.is_null() {
                (*(*bucket).prev).next = (*bucket).next
            } else { bucketHashes[(*bucket).hash as usize] = (*bucket).next }
            if !(*bucket).next.is_null() {
                (*(*bucket).next).prev = (*bucket).prev
            }
            memset(bucket as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<leakyBucket_t>() as libc::c_ulong);
        }
        if (*bucket).type_0 as libc::c_uint ==
               NA_BAD as libc::c_int as libc::c_uint {
            (*bucket).type_0 = address.type_0;
            match address.type_0 as libc::c_uint {
                4 => {
                    memcpy((*bucket).ipv._4.as_mut_ptr() as *mut libc::c_void,
                           address.ip.as_mut_ptr() as *const libc::c_void,
                           4i32 as libc::c_ulong);
                }
                5 => {
                    memcpy((*bucket).ipv._6.as_mut_ptr() as *mut libc::c_void,
                           address.ip6.as_mut_ptr() as *const libc::c_void,
                           16i32 as libc::c_ulong);
                }
                _ => { }
            }
            (*bucket).lastTime = now;
            (*bucket).burst = 0i32 as libc::c_schar;
            (*bucket).hash = hash;
            (*bucket).next = bucketHashes[hash as usize];
            if !bucketHashes[hash as usize].is_null() {
                (*bucketHashes[hash as usize]).prev = bucket
            }
            (*bucket).prev = 0 as *mut leakyBucket_t;
            bucketHashes[hash as usize] = bucket;
            return bucket
        }
        i += 1
    }
    return 0 as *mut leakyBucket_t;
}
/*
================
SVC_HashForAddress
================
*/
unsafe extern "C" fn SVC_HashForAddress(mut address: netadr_t)
 -> libc::c_long {
    let mut ip: *mut byte = 0 as *mut byte;
    let mut size: size_t = 0i32 as size_t;
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_long = 0i32 as libc::c_long;
    match address.type_0 as libc::c_uint {
        4 => { ip = address.ip.as_mut_ptr(); size = 4i32 as size_t }
        5 => { ip = address.ip6.as_mut_ptr(); size = 16i32 as size_t }
        _ => { }
    }
    i = 0i32;
    while (i as libc::c_ulong) < size {
        hash +=
            *ip.offset(i as isize) as libc::c_long *
                (i + 119i32) as libc::c_long;
        i += 1
    }
    hash = hash ^ hash >> 10i32 ^ hash >> 20i32;
    hash &= (1024i32 - 1i32) as libc::c_long;
    return hash;
}
static mut bucketHashes: [*mut leakyBucket_t; 1024] =
    [0 as *const leakyBucket_t as *mut leakyBucket_t; 1024];
// when the master tries to poll the server, it won't respond, so
	// it will be removed from the list
/*
==============================================================================

CONNECTIONLESS COMMANDS

==============================================================================
*/
// This is deliberately quite large to make it more of an effort to DoS
static mut buckets: [leakyBucket_t; 16384] =
    [leakyBucket_s{type_0: NA_BAD,
                   ipv: unnamed{_4: [0; 4],},
                   lastTime: 0,
                   burst: 0,
                   hash: 0,
                   prev: 0 as *const leakyBucket_t as *mut leakyBucket_t,
                   next: 0 as *const leakyBucket_t as *mut leakyBucket_t,};
        16384];
/*
================
SVC_Info

Responds with a short info message that should be enough to determine
if a user is interested in a server to do a full status
================
*/
#[no_mangle]
pub unsafe extern "C" fn SVC_Info(mut from: netadr_t) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut humans: libc::c_int = 0;
    let mut gamedir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut infostring: [libc::c_char; 1024] = [0; 1024];
    if Cvar_VariableValue(b"g_gametype\x00" as *const u8 as
                              *const libc::c_char) ==
           GT_SINGLE_PLAYER as libc::c_int as libc::c_float ||
           0. !=
               Cvar_VariableValue(b"ui_singlePlayerActive\x00" as *const u8 as
                                      *const libc::c_char) {
        return
    }
    if 0 != SVC_RateLimitAddress(from, 10i32, 1000i32) as u64 {
        Com_DPrintf(b"SVC_Info: rate limit from %s exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(from));
        return
    }
    if 0 != SVC_RateLimit(&mut outboundLeakyBucket, 10i32, 100i32) as u64 {
        Com_DPrintf(b"SVC_Info: rate limit exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    if strlen(Cmd_Argv(1i32)) > 128i32 as libc::c_ulong { return }
    humans = 0i32;
    count = humans;
    i = (*sv_privateClients).integer;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            count += 1;
            if (*svs.clients.offset(i as isize)).netchan.remoteAddress.type_0
                   as libc::c_uint != NA_BOT as libc::c_int as libc::c_uint {
                humans += 1
            }
        }
        i += 1
    }
    infostring[0usize] = 0i32 as libc::c_char;
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"challenge\x00" as *const u8 as *const libc::c_char,
                        Cmd_Argv(1i32));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"gamename\x00" as *const u8 as *const libc::c_char,
                        (*com_gamename).string);
    if (*com_legacyprotocol).integer > 0i32 {
        Info_SetValueForKey(infostring.as_mut_ptr(),
                            b"protocol\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*com_legacyprotocol).integer));
    } else {
        Info_SetValueForKey(infostring.as_mut_ptr(),
                            b"protocol\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*com_protocol).integer));
    }
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"hostname\x00" as *const u8 as *const libc::c_char,
                        (*sv_hostname).string);
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"mapname\x00" as *const u8 as *const libc::c_char,
                        (*sv_mapname).string);
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"clients\x00" as *const u8 as *const libc::c_char,
                        va(b"%i\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char, count));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"g_humanplayers\x00" as *const u8 as
                            *const libc::c_char,
                        va(b"%i\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char, humans));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"sv_maxclients\x00" as *const u8 as
                            *const libc::c_char,
                        va(b"%i\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                           (*sv_maxclients).integer -
                               (*sv_privateClients).integer));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"gametype\x00" as *const u8 as *const libc::c_char,
                        va(b"%i\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char, (*sv_gametype).integer));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"pure\x00" as *const u8 as *const libc::c_char,
                        va(b"%i\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char, (*sv_pure).integer));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"g_needpass\x00" as *const u8 as *const libc::c_char,
                        va(b"%d\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                           Cvar_VariableIntegerValue(b"g_needpass\x00" as
                                                         *const u8 as
                                                         *const libc::c_char)));
    if !(*sv_voipProtocol).string.is_null() &&
           0 != *(*sv_voipProtocol).string as libc::c_int {
        Info_SetValueForKey(infostring.as_mut_ptr(),
                            b"voip\x00" as *const u8 as *const libc::c_char,
                            (*sv_voipProtocol).string);
    }
    if 0 != (*sv_minPing).integer {
        Info_SetValueForKey(infostring.as_mut_ptr(),
                            b"minPing\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*sv_minPing).integer));
    }
    if 0 != (*sv_maxPing).integer {
        Info_SetValueForKey(infostring.as_mut_ptr(),
                            b"maxPing\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*sv_maxPing).integer));
    }
    gamedir =
        Cvar_VariableString(b"fs_game\x00" as *const u8 as
                                *const libc::c_char);
    if 0 != *gamedir {
        Info_SetValueForKey(infostring.as_mut_ptr(),
                            b"game\x00" as *const u8 as *const libc::c_char,
                            gamedir);
    }
    NET_OutOfBandPrint(NS_SERVER, from,
                       b"infoResponse\n%s\x00" as *const u8 as
                           *const libc::c_char, infostring.as_mut_ptr());
}
#[no_mangle]
pub static mut sv_maxPing: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_minPing: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_voipProtocol: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_pure: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_gametype: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_privateClients: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_mapname: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_hostname: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut outboundLeakyBucket: leakyBucket_t =
    leakyBucket_s{type_0: NA_BAD,
                  ipv: unnamed{_4: [0; 4],},
                  lastTime: 0,
                  burst: 0,
                  hash: 0,
                  prev: 0 as *const leakyBucket_t as *mut leakyBucket_t,
                  next: 0 as *const leakyBucket_t as *mut leakyBucket_t,};
/*
================
SVC_Status

Responds with all the info that qplug or qspy can see about the server
and all connected players.  Used for getting detailed information after
the simple info query.
================
*/
unsafe extern "C" fn SVC_Status(mut from: netadr_t) {
    let mut player: [libc::c_char; 1024] = [0; 1024];
    let mut status: [libc::c_char; 16384] = [0; 16384];
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    let mut statusLength: libc::c_int = 0;
    let mut playerLength: libc::c_int = 0;
    let mut infostring: [libc::c_char; 1024] = [0; 1024];
    if Cvar_VariableValue(b"g_gametype\x00" as *const u8 as
                              *const libc::c_char) ==
           GT_SINGLE_PLAYER as libc::c_int as libc::c_float ||
           0. !=
               Cvar_VariableValue(b"ui_singlePlayerActive\x00" as *const u8 as
                                      *const libc::c_char) {
        return
    }
    if 0 != SVC_RateLimitAddress(from, 10i32, 1000i32) as u64 {
        Com_DPrintf(b"SVC_Status: rate limit from %s exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(from));
        return
    }
    if 0 != SVC_RateLimit(&mut outboundLeakyBucket, 10i32, 100i32) as u64 {
        Com_DPrintf(b"SVC_Status: rate limit exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    if strlen(Cmd_Argv(1i32)) > 128i32 as libc::c_ulong { return }
    strcpy(infostring.as_mut_ptr(), Cvar_InfoString(0x4i32));
    Info_SetValueForKey(infostring.as_mut_ptr(),
                        b"challenge\x00" as *const u8 as *const libc::c_char,
                        Cmd_Argv(1i32));
    status[0usize] = 0i32 as libc::c_char;
    statusLength = 0i32;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if (*cl).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            ps = SV_GameClientNum(i);
            Com_sprintf(player.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong as libc::c_int,
                        b"%i %i \"%s\"\n\x00" as *const u8 as
                            *const libc::c_char, (*ps).persistant[0usize],
                        (*cl).ping, (*cl).name.as_mut_ptr());
            playerLength = strlen(player.as_mut_ptr()) as libc::c_int;
            if (statusLength + playerLength) as libc::c_ulong >=
                   ::std::mem::size_of::<[libc::c_char; 16384]>() as
                       libc::c_ulong {
                // can't hold any more
                break ;
            } else {
                strcpy(status.as_mut_ptr().offset(statusLength as isize),
                       player.as_mut_ptr());
                statusLength += playerLength
            }
        }
        i += 1
    }
    NET_OutOfBandPrint(NS_SERVER, from,
                       b"statusResponse\n%s\n%s\x00" as *const u8 as
                           *const libc::c_char, infostring.as_mut_ptr(),
                       status.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_FrameMsec() -> libc::c_int {
    if !sv_fps.is_null() {
        let mut frameMsec: libc::c_int = 0;
        frameMsec = (1000.0f32 / (*sv_fps).value) as libc::c_int;
        if frameMsec < sv.timeResidual {
            return 0i32
        } else { return frameMsec - sv.timeResidual }
    } else { return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendQueuedPackets() -> libc::c_int {
    let mut numBlocks: libc::c_int = 0;
    let mut dlStart: libc::c_int = 0;
    let mut deltaT: libc::c_int = 0;
    let mut delayT: libc::c_int = 0;
    static mut dlNextRound: libc::c_int = 0i32;
    let mut timeVal: libc::c_int = 2147483647i32;
    delayT = SV_SendQueuedMessages();
    if delayT >= 0i32 { timeVal = delayT }
    if 0 != (*sv_dlRate).integer {
        dlStart = Sys_Milliseconds();
        deltaT = dlNextRound - dlStart;
        if deltaT > 0i32 {
            if deltaT < timeVal { timeVal = deltaT + 1i32 }
        } else {
            numBlocks = SV_SendDownloadMessages();
            if 0 != numBlocks {
                deltaT = Sys_Milliseconds() - dlStart;
                delayT = 1000i32 * numBlocks * 1024i32;
                delayT /= (*sv_dlRate).integer * 1024i32;
                if delayT <= deltaT + 1i32 {
                    if timeVal > 2i32 { timeVal = 2i32 }
                    dlNextRound = dlStart + deltaT + 1i32
                } else {
                    dlNextRound = dlStart + delayT;
                    delayT -= deltaT;
                    if delayT < timeVal { timeVal = delayT }
                }
            }
        }
    } else if 0 != SV_SendDownloadMessages() { timeVal = 0i32 }
    return timeVal;
}
#[no_mangle]
pub static mut sv_dlRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_privatePassword: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_allowDownload: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_reconnectlimit: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_showloss: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_padPackets: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_mapChecksum: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_serverid: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_minRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_maxRate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_floodProtect: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_lanForceRate: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_strictAuth: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_banFile: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut serverBans: [serverBan_t; 1024] =
    [serverBan_t{ip:
                     netadr_t{type_0: NA_BAD,
                              ip: [0; 4],
                              ip6: [0; 16],
                              port: 0,
                              scope_id: 0,},
                 subnet: 0,
                 isexception: qfalse,}; 1024];
#[no_mangle]
pub static mut serverBansCount: libc::c_int = 0i32;
#[no_mangle]
pub static mut sv_voip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn SV_MasterShutdown() {
    svs.nextHeartbeatTime = -9999i32;
    SV_MasterHeartbeat(b"DarkPlaces\x00" as *const u8 as *const libc::c_char);
    svs.nextHeartbeatTime = -9999i32;
    SV_MasterHeartbeat(b"DarkPlaces\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SV_RateMsec(mut client: *mut client_t)
 -> libc::c_int {
    let mut rate: libc::c_int = 0;
    let mut rateMsec: libc::c_int = 0;
    let mut messageSize: libc::c_int = 0;
    messageSize = (*client).netchan.lastSentSize;
    rate = (*client).rate;
    if 0 != (*sv_maxRate).integer {
        if (*sv_maxRate).integer < 1000i32 {
            Cvar_Set(b"sv_MaxRate\x00" as *const u8 as *const libc::c_char,
                     b"1000\x00" as *const u8 as *const libc::c_char);
        }
        if (*sv_maxRate).integer < rate { rate = (*sv_maxRate).integer }
    }
    if 0 != (*sv_minRate).integer {
        if (*sv_minRate).integer < 1000i32 {
            Cvar_Set(b"sv_minRate\x00" as *const u8 as *const libc::c_char,
                     b"1000\x00" as *const u8 as *const libc::c_char);
        }
        if (*sv_minRate).integer > rate { rate = (*sv_minRate).integer }
    }
    if (*client).netchan.remoteAddress.type_0 as libc::c_uint ==
           NA_IP6 as libc::c_int as libc::c_uint {
        messageSize += 48i32
    } else { messageSize += 28i32 }
    rateMsec =
        messageSize * 1000i32 /
            (rate as libc::c_float * (*com_timescale).value) as libc::c_int;
    rate = Sys_Milliseconds() - (*client).netchan.lastSentTime;
    if rate > rateMsec { return 0i32 } else { return rateMsec - rate };
}
//
// sv_snapshot.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_AddServerCommand(mut client: *mut client_t,
                                             mut cmd: *const libc::c_char) {
    let mut index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if ((*client).state as libc::c_uint) <
           CS_PRIMED as libc::c_int as libc::c_uint {
        return
    }
    (*client).reliableSequence += 1;
    if (*client).reliableSequence - (*client).reliableAcknowledge ==
           64i32 + 1i32 {
        Com_Printf(b"===== pending server commands =====\n\x00" as *const u8
                       as *const libc::c_char);
        i = (*client).reliableAcknowledge + 1i32;
        while i <= (*client).reliableSequence {
            Com_Printf(b"cmd %5d: %s\n\x00" as *const u8 as
                           *const libc::c_char, i,
                       (*client).reliableCommands[(i & 64i32 - 1i32) as
                                                      usize].as_mut_ptr());
            i += 1
        }
        Com_Printf(b"cmd %5d: %s\n\x00" as *const u8 as *const libc::c_char,
                   i, cmd);
        SV_DropClient(client,
                      b"Server command overflow\x00" as *const u8 as
                          *const libc::c_char);
        return
    }
    index = (*client).reliableSequence & 64i32 - 1i32;
    Q_strncpyz((*client).reliableCommands[index as usize].as_mut_ptr(), cmd,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
}