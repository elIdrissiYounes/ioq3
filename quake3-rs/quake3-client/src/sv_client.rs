use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
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
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        //=============================================
        //
// key / value info strings
//
        #[no_mangle]
        pub fn Info_ValueForKey(s: *const libc::c_char,
                                key: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Info_SetValueForKey(s: *mut libc::c_char,
                                   key: *const libc::c_char,
                                   value: *const libc::c_char);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
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
    use super::q_shared_h::{qboolean, byte, usercmd_t, entityState_s,
                            fileHandle_t, cvar_t};
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
        pub fn MSG_WriteByte(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteShort(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteLong(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteString(sb: *mut msg_t, s: *const libc::c_char);
        #[no_mangle]
        pub fn MSG_WriteBigString(sb: *mut msg_t, s: *const libc::c_char);
        #[no_mangle]
        pub fn MSG_HashKey(string: *const libc::c_char, maxlen: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadByte(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadShort(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadLong(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadString(sb: *mut msg_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn MSG_ReadData(sb: *mut msg_t, buffer: *mut libc::c_void,
                            size: libc::c_int);
        #[no_mangle]
        pub fn MSG_ReadDeltaUsercmdKey(msg: *mut msg_t, key: libc::c_int,
                                       from: *mut usercmd_t,
                                       to: *mut usercmd_t);
        #[no_mangle]
        pub fn MSG_WriteDeltaEntity(msg: *mut msg_t, from: *mut entityState_s,
                                    to: *mut entityState_s, force: qboolean);
        #[no_mangle]
        pub fn NET_OutOfBandPrint(net_socket: netsrc_t, adr: netadr_t,
                                  format: *const libc::c_char, ...);
        #[no_mangle]
        pub fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_CompareBaseAdrMask(a: netadr_t, b: netadr_t,
                                      netmask: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn NET_CompareBaseAdr(a: netadr_t, b: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_StringToAdr(s: *const libc::c_char, a: *mut netadr_t,
                               family: netadrtype_t) -> libc::c_int;
        #[no_mangle]
        pub fn Netchan_Setup(sock: netsrc_t, chan: *mut netchan_t,
                             adr: netadr_t, qport: libc::c_int,
                             challenge: libc::c_int, compat: qboolean);
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        #[no_mangle]
        pub fn VM_ExplicitArgPtr(vm: *mut vm_t, intValue: intptr_t)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_Args_Sanitize();
        // The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
        #[no_mangle]
        pub fn Cmd_TokenizeString(text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_TokenizeStringIgnoreQuotes(text_in: *const libc::c_char);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn FS_SV_FOpenFileRead(filename: *const libc::c_char,
                                   fp: *mut fileHandle_t) -> libc::c_long;
        // if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
        #[no_mangle]
        pub fn FS_FileIsInPAK(filename: *const libc::c_char,
                              pChecksum: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        // seek on a file
        #[no_mangle]
        pub fn FS_FilenameCompare(s1: *const libc::c_char,
                                  s2: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn FS_LoadedPakPureChecksums() -> *const libc::c_char;
        // Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
        #[no_mangle]
        pub fn FS_ReferencedPakNames() -> *const libc::c_char;
        #[no_mangle]
        pub fn FS_idPak(pak: *mut libc::c_char, base: *mut libc::c_char,
                        numPaks: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Com_IsVoipTarget(voipTargets: *mut uint8_t,
                                voipTargetsSize: libc::c_int,
                                clientNum: libc::c_int) -> qboolean;
        #[no_mangle]
        pub static mut com_dedicated: *mut cvar_t;
        #[no_mangle]
        pub static mut com_cl_running: *mut cvar_t;
        #[no_mangle]
        pub static mut com_standalone: *mut cvar_t;
        #[no_mangle]
        pub static mut com_gamename: *mut cvar_t;
        #[no_mangle]
        pub static mut com_protocol: *mut cvar_t;
        #[no_mangle]
        pub static mut com_legacyprotocol: *mut cvar_t;
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        //Does NOT parse port numbers, only base addresses.
        #[no_mangle]
        pub fn Sys_IsLANAddress(adr: netadr_t) -> qboolean;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_public.h"]
pub mod g_public_h {
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
    // the server looks at a sharedEntity, which is the start of the game's gentity_t structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sharedEntity_t {
        pub s: entityState_t,
        pub r: entityShared_t,
    }
    //
// functions exported by the game subsystem
//
    pub type unnamed_0 = libc::c_uint;
    // ConsoleCommand will be called when a command has been issued
	// that is not recognized as a builtin function.
	// The game can issue trap_argc() / trap_argv() commands to get the command
	// and parameters.  Return qfalse if the game doesn't recognize it as a command.
    // ( int time );
    pub const BOTAI_START_FRAME: unnamed_0 = 10;
    // ( void );
    pub const GAME_CONSOLE_COMMAND: unnamed_0 = 9;
    // ( int levelTime );
    pub const GAME_RUN_FRAME: unnamed_0 = 8;
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/bg_public.h"]
pub mod bg_public_h {
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
    // single player ffa
    pub const GT_SINGLE_PLAYER: unnamed_1 = 2;
    // one on one tournament
    pub const GT_TOURNAMENT: unnamed_1 = 1;
    // free for all
    pub const GT_FFA: unnamed_1 = 0;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/server.h"]
pub mod server_h {
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
    pub type voipServerPacket_t = voipServerPacket_s;
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
    pub type svEntity_t = svEntity_s;
    pub type serverState_t = libc::c_uint;
    // actively running
    pub const SS_GAME: serverState_t = 2;
    // spawning level entities
    pub const SS_LOADING: serverState_t = 1;
    // no map loaded
    pub const SS_DEAD: serverState_t = 0;
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
    pub struct netchan_buffer_s {
        pub msg: msg_t,
        pub msgBuffer: [byte; 16384],
        pub clientCommandString: [libc::c_char; 1024],
        pub next: *mut netchan_buffer_s,
    }
    pub type netchan_buffer_t = netchan_buffer_s;
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
    pub type client_t = client_s;
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
    // Structure for managing bans
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverBan_t {
        pub ip: netadr_t,
        pub subnet: libc::c_int,
        pub isexception: qboolean,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct leakyBucket_s {
        pub type_0: netadrtype_t,
        pub ipv: unnamed_2,
        pub lastTime: libc::c_int,
        pub burst: libc::c_schar,
        pub hash: libc::c_long,
        pub prev: *mut leakyBucket_t,
        pub next: *mut leakyBucket_t,
    }
    //===========================================================
    //
// sv_main.c
//
    pub type leakyBucket_t = leakyBucket_s;
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_2 {
        pub _4: [byte; 4],
        pub _6: [byte; 16],
    }
    use super::{libc};
    use super::q_shared_h::{byte, entityState_t, qboolean, playerState_t,
                            usercmd_t, fileHandle_t, cvar_t};
    use super::g_public_h::{sharedEntity_t};
    use super::qcommon_h::{msg_t, netchan_t, netadr_t, netadrtype_t, vm_t};
    extern "C" {
        pub type worldSector_s;
        //=============================================================================
        // persistant server info across maps
        #[no_mangle]
        pub static mut svs: serverStatic_t;
        // cleared each map
        #[no_mangle]
        pub static mut sv: server_t;
        // game virtual machine
        #[no_mangle]
        pub static mut gvm: *mut vm_t;
        #[no_mangle]
        pub static mut sv_fps: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_privatePassword: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_allowDownload: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_maxclients: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_privateClients: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_reconnectlimit: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_minPing: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_maxPing: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_pure: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_floodProtect: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_lanForceRate: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_strictAuth: *mut cvar_t;
        #[no_mangle]
        pub static mut serverBans: [serverBan_t; 1024];
        #[no_mangle]
        pub static mut serverBansCount: libc::c_int;
        #[no_mangle]
        pub static mut sv_voip: *mut cvar_t;
        #[no_mangle]
        pub static mut outboundLeakyBucket: leakyBucket_t;
        #[no_mangle]
        pub fn SVC_RateLimit(bucket: *mut leakyBucket_t, burst: libc::c_int,
                             period: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn SVC_RateLimitAddress(from: netadr_t, burst: libc::c_int,
                                    period: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn SV_RateMsec(client: *mut client_t) -> libc::c_int;
        #[no_mangle]
        pub fn SV_UpdateConfigstrings(client: *mut client_t);
        #[no_mangle]
        pub fn SV_SetUserinfo(index: libc::c_int, val: *const libc::c_char);
        //
// sv_ccmds.c
//
        #[no_mangle]
        pub fn SV_Heartbeat_f();
        #[no_mangle]
        pub fn SV_BotFreeClient(clientNum: libc::c_int);
        #[no_mangle]
        pub fn SV_Netchan_FreeQueue(client: *mut client_t);
        #[no_mangle]
        pub fn SV_GentityNum(num: libc::c_int) -> *mut sharedEntity_t;
        #[no_mangle]
        pub fn SV_SendMessageToClient(msg: *mut msg_t, client: *mut client_t);
        #[no_mangle]
        pub fn SV_UpdateServerCommandsToClient(client: *mut client_t,
                                               msg: *mut msg_t);
        #[no_mangle]
        pub fn SV_SendClientSnapshot(client: *mut client_t);
        // clip to a specific entity
        //
// sv_net_chan.c
//
        #[no_mangle]
        pub fn SV_Netchan_Transmit(client: *mut client_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn SV_Netchan_TransmitNextFragment(client: *mut client_t)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/sv_client.c"]
pub mod sv_client_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ucmd_t {
        pub name: *mut libc::c_char,
        pub func: Option<unsafe extern "C" fn(_: *mut client_t) -> ()>,
    }
    use super::{libc};
    use super::server_h::{client_t};
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
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
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
        pub fn rand() -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_platform.h"]
pub mod q_platform_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn ShortSwap(l: libc::c_short) -> libc::c_short;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/server/sv_variadic.h"]
pub mod sv_variadic_h {
    use super::server_h::{client_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn SV_SendServerCommand(cl: *mut client_t,
                                    fmt: *const libc::c_char, ...);
    }
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cvar_s, cvar_t,
                       playerState_s, playerState_t, usercmd_s, usercmd_t,
                       trType_t, TR_GRAVITY, TR_SINE, TR_LINEAR_STOP,
                       TR_LINEAR, TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, Com_sprintf, Q_stricmp,
                       Q_strncpyz, Info_ValueForKey, Info_SetValueForKey,
                       Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      svc_ops_e, svc_voipOpus, svc_voipSpeex, svc_EOF,
                      svc_snapshot, svc_download, svc_serverCommand,
                      svc_baseline, svc_configstring, svc_gamestate, svc_nop,
                      svc_bad, clc_ops_e, clc_voipOpus, clc_voipSpeex,
                      clc_EOF, clc_clientCommand, clc_moveNoDelta, clc_move,
                      clc_nop, clc_bad, vm_t, vm_s, MSG_Init, MSG_WriteData,
                      MSG_Bitstream, MSG_WriteByte, MSG_WriteShort,
                      MSG_WriteLong, MSG_WriteString, MSG_WriteBigString,
                      MSG_HashKey, MSG_ReadByte, MSG_ReadShort, MSG_ReadLong,
                      MSG_ReadString, MSG_ReadData, MSG_ReadDeltaUsercmdKey,
                      MSG_WriteDeltaEntity, NET_OutOfBandPrint,
                      NET_CompareAdr, NET_CompareBaseAdrMask,
                      NET_CompareBaseAdr, NET_IsLocalAddress, NET_AdrToString,
                      NET_StringToAdr, Netchan_Setup, VM_Call,
                      VM_ExplicitArgPtr, Cmd_Argc, Cmd_Argv,
                      Cmd_Args_Sanitize, Cmd_TokenizeString,
                      Cmd_TokenizeStringIgnoreQuotes, Cvar_VariableValue,
                      Cvar_VariableString, FS_SV_FOpenFileRead,
                      FS_FileIsInPAK, FS_Read, FS_FCloseFile,
                      FS_FilenameCompare, FS_LoadedPakPureChecksums,
                      FS_ReferencedPakNames, FS_idPak, Com_DPrintf,
                      Com_IsVoipTarget, com_dedicated, com_cl_running,
                      com_standalone, com_gamename, com_protocol,
                      com_legacyprotocol, Z_MallocDebug, Z_Free,
                      Sys_IsLANAddress};
use self::g_public_h::{entityShared_t, sharedEntity_t, unnamed_0,
                       BOTAI_START_FRAME, GAME_CONSOLE_COMMAND,
                       GAME_RUN_FRAME, GAME_CLIENT_THINK, GAME_CLIENT_COMMAND,
                       GAME_CLIENT_DISCONNECT, GAME_CLIENT_USERINFO_CHANGED,
                       GAME_CLIENT_BEGIN, GAME_CLIENT_CONNECT, GAME_SHUTDOWN,
                       GAME_INIT};
use self::bg_public_h::{unnamed_1, GT_MAX_GAME_TYPE, GT_HARVESTER, GT_OBELISK,
                        GT_1FCTF, GT_CTF, GT_TEAM, GT_SINGLE_PLAYER,
                        GT_TOURNAMENT, GT_FFA};
use self::server_h::{voipServerPacket_s, voipServerPacket_t, svEntity_s,
                     svEntity_t, serverState_t, SS_GAME, SS_LOADING, SS_DEAD,
                     server_t, clientSnapshot_t, clientState_t, CS_ACTIVE,
                     CS_PRIMED, CS_CONNECTED, CS_ZOMBIE, CS_FREE,
                     netchan_buffer_s, netchan_buffer_t, client_s, client_t,
                     challenge_t, serverStatic_t, serverBan_t, leakyBucket_s,
                     leakyBucket_t, unnamed_2, worldSector_s, svs, sv, gvm,
                     sv_fps, sv_privatePassword, sv_allowDownload,
                     sv_maxclients, sv_privateClients, sv_reconnectlimit,
                     sv_minPing, sv_maxPing, sv_pure, sv_floodProtect,
                     sv_lanForceRate, sv_strictAuth, serverBans,
                     serverBansCount, sv_voip, outboundLeakyBucket,
                     SVC_RateLimit, SVC_RateLimitAddress, SV_RateMsec,
                     SV_UpdateConfigstrings, SV_SetUserinfo, SV_Heartbeat_f,
                     SV_BotFreeClient, SV_Netchan_FreeQueue, SV_GentityNum,
                     SV_SendMessageToClient, SV_UpdateServerCommandsToClient,
                     SV_SendClientSnapshot, SV_Netchan_Transmit,
                     SV_Netchan_TransmitNextFragment};
use self::sv_client_c::{ucmd_t};
use self::string_h::{memcpy, memset, strcmp, strrchr, strstr, strlen};
use self::stdlib_h::{atoi, rand};
use self::q_platform_h::{ShortSwap};
use self::sv_variadic_h::{SV_SendServerCommand};
//
// sv_client.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_GetChallenge(mut from: netadr_t) {
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0;
    let mut oldestTime: libc::c_int = 0;
    let mut oldestClientTime: libc::c_int = 0;
    let mut clientChallenge: libc::c_int = 0;
    let mut challenge: *mut challenge_t = 0 as *mut challenge_t;
    let mut wasfound: qboolean = qfalse;
    let mut gameName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gameMismatch: qboolean = qfalse;
    if Cvar_VariableValue(b"g_gametype\x00" as *const u8 as
                              *const libc::c_char) ==
           GT_SINGLE_PLAYER as libc::c_int as libc::c_float ||
           0. !=
               Cvar_VariableValue(b"ui_singlePlayerActive\x00" as *const u8 as
                                      *const libc::c_char) {
        return
    }
    if 0 != SVC_RateLimitAddress(from, 10i32, 1000i32) as u64 {
        Com_DPrintf(b"SV_GetChallenge: rate limit from %s exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_AdrToString(from));
        return
    }
    if 0 != SVC_RateLimit(&mut outboundLeakyBucket, 10i32, 100i32) as u64 {
        Com_DPrintf(b"SV_GetChallenge: rate limit exceeded, dropping request\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    gameName = Cmd_Argv(2i32);
    if 0 != (*com_legacyprotocol).integer && 0 == *gameName {
        gameMismatch = qfalse
    } else {
        gameMismatch =
            (0 == *gameName ||
                 strcmp(gameName, (*com_gamename).string) != 0i32) as
                libc::c_int as qboolean
    }
    if 0 != gameMismatch as u64 {
        NET_OutOfBandPrint(NS_SERVER, from,
                           b"print\nGame mismatch: This is a %s server\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*com_gamename).string);
        return
    }
    oldest = 0i32;
    oldestTime = 0x7fffffffi32;
    oldestClientTime = oldestTime;
    challenge = &mut svs.challenges[0usize] as *mut challenge_t;
    clientChallenge = atoi(Cmd_Argv(1i32));
    i = 0i32;
    while i < 2048i32 {
        if 0 == (*challenge).connected as u64 &&
               0 != NET_CompareAdr(from, (*challenge).adr) as libc::c_uint {
            wasfound = qtrue;
            if (*challenge).time < oldestClientTime {
                oldestClientTime = (*challenge).time
            }
        }
        if 0 != wasfound as libc::c_uint && i >= 2048i32 / 2i32 {
            i = 2048i32;
            break ;
        } else {
            if (*challenge).time < oldestTime {
                oldestTime = (*challenge).time;
                oldest = i
            }
            i += 1;
            challenge = challenge.offset(1isize)
        }
    }
    if i == 2048i32 {
        challenge = &mut svs.challenges[oldest as usize] as *mut challenge_t;
        (*challenge).clientChallenge = clientChallenge;
        (*challenge).adr = from;
        (*challenge).firstTime = svs.time;
        (*challenge).connected = qfalse
    }
    (*challenge).challenge =
        ((rand() as libc::c_uint) << 16i32 ^ rand() as libc::c_uint ^
             svs.time as libc::c_uint) as libc::c_int;
    (*challenge).wasrefused = qfalse;
    (*challenge).time = svs.time;
    if (*challenge).adr.type_0 as libc::c_uint ==
           NA_IP as libc::c_int as libc::c_uint &&
           0 == (*com_standalone).integer &&
           0 == Sys_IsLANAddress(from) as u64 {
        if svs.authorizeAddress.type_0 as libc::c_uint ==
               NA_BAD as libc::c_int as libc::c_uint {
            Com_Printf(b"Resolving %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       b"authorize.quake3arena.com\x00" as *const u8 as
                           *const libc::c_char);
            if 0 !=
                   NET_StringToAdr(b"authorize.quake3arena.com\x00" as
                                       *const u8 as *const libc::c_char,
                                   &mut svs.authorizeAddress, NA_IP) {
                svs.authorizeAddress.port =
                    ShortSwap(27952i32 as libc::c_short) as libc::c_ushort;
                Com_Printf(b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8
                               as *const libc::c_char,
                           b"authorize.quake3arena.com\x00" as *const u8 as
                               *const libc::c_char,
                           svs.authorizeAddress.ip[0usize] as libc::c_int,
                           svs.authorizeAddress.ip[1usize] as libc::c_int,
                           svs.authorizeAddress.ip[2usize] as libc::c_int,
                           svs.authorizeAddress.ip[3usize] as libc::c_int,
                           ShortSwap(svs.authorizeAddress.port as
                                         libc::c_short) as libc::c_int);
            }
        }
        if svs.authorizeAddress.type_0 as libc::c_uint ==
               NA_BAD as libc::c_int as libc::c_uint {
            Com_Printf(b"Couldn\'t resolve auth server address\n\x00" as
                           *const u8 as *const libc::c_char);
        } else if svs.time - oldestClientTime > 5000i32 {
            Com_DPrintf(b"authorize server timed out\n\x00" as *const u8 as
                            *const libc::c_char);
        } else {
            let mut game: *const libc::c_char = 0 as *const libc::c_char;
            Com_DPrintf(b"sending getIpAuthorize for %s\n\x00" as *const u8 as
                            *const libc::c_char, NET_AdrToString(from));
            game =
                Cvar_VariableString(b"fs_game\x00" as *const u8 as
                                        *const libc::c_char);
            if *game.offset(0isize) as libc::c_int == 0i32 {
                game = b"baseq3\x00" as *const u8 as *const libc::c_char
            }
            NET_OutOfBandPrint(NS_SERVER, svs.authorizeAddress,
                               b"getIpAuthorize %i %i.%i.%i.%i %s 0 %s\x00" as
                                   *const u8 as *const libc::c_char,
                               (*challenge).challenge,
                               from.ip[0usize] as libc::c_int,
                               from.ip[1usize] as libc::c_int,
                               from.ip[2usize] as libc::c_int,
                               from.ip[3usize] as libc::c_int, game,
                               (*sv_strictAuth).string);
            return
        }
    }
    (*challenge).pingTime = svs.time;
    NET_OutOfBandPrint(NS_SERVER, (*challenge).adr,
                       b"challengeResponse %d %d %d\x00" as *const u8 as
                           *const libc::c_char, (*challenge).challenge,
                       clientChallenge, (*com_protocol).integer);
}
#[no_mangle]
pub unsafe extern "C" fn SV_DirectConnect(mut from: netadr_t) {
    let mut current_block: u64;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut newcl: *mut client_t = 0 as *mut client_t;
    let mut temp: client_t =
        client_s{state: CS_FREE,
                 userinfo: [0; 1024],
                 reliableCommands: [[0; 1024]; 64],
                 reliableSequence: 0,
                 reliableAcknowledge: 0,
                 reliableSent: 0,
                 messageAcknowledge: 0,
                 gamestateMessageNum: 0,
                 challenge: 0,
                 lastUsercmd:
                     usercmd_s{serverTime: 0,
                               angles: [0; 3],
                               buttons: 0,
                               weapon: 0,
                               forwardmove: 0,
                               rightmove: 0,
                               upmove: 0,},
                 lastMessageNum: 0,
                 lastClientCommand: 0,
                 lastClientCommandString: [0; 1024],
                 gentity: 0 as *mut sharedEntity_t,
                 name: [0; 32],
                 downloadName: [0; 64],
                 download: 0,
                 downloadSize: 0,
                 downloadCount: 0,
                 downloadClientBlock: 0,
                 downloadCurrentBlock: 0,
                 downloadXmitBlock: 0,
                 downloadBlocks: [0 as *mut libc::c_uchar; 48],
                 downloadBlockSize: [0; 48],
                 downloadEOF: qfalse,
                 downloadSendTime: 0,
                 deltaMessage: 0,
                 nextReliableTime: 0,
                 lastPacketTime: 0,
                 lastConnectTime: 0,
                 lastSnapshotTime: 0,
                 rateDelayed: qfalse,
                 timeoutCount: 0,
                 frames:
                     [clientSnapshot_t{areabytes: 0,
                                       areabits: [0; 32],
                                       ps:
                                           playerState_s{commandTime: 0,
                                                         pm_type: 0,
                                                         bobCycle: 0,
                                                         pm_flags: 0,
                                                         pm_time: 0,
                                                         origin: [0.; 3],
                                                         velocity: [0.; 3],
                                                         weaponTime: 0,
                                                         gravity: 0,
                                                         speed: 0,
                                                         delta_angles: [0; 3],
                                                         groundEntityNum: 0,
                                                         legsTimer: 0,
                                                         legsAnim: 0,
                                                         torsoTimer: 0,
                                                         torsoAnim: 0,
                                                         movementDir: 0,
                                                         grapplePoint:
                                                             [0.; 3],
                                                         eFlags: 0,
                                                         eventSequence: 0,
                                                         events: [0; 2],
                                                         eventParms: [0; 2],
                                                         externalEvent: 0,
                                                         externalEventParm: 0,
                                                         externalEventTime: 0,
                                                         clientNum: 0,
                                                         weapon: 0,
                                                         weaponstate: 0,
                                                         viewangles: [0.; 3],
                                                         viewheight: 0,
                                                         damageEvent: 0,
                                                         damageYaw: 0,
                                                         damagePitch: 0,
                                                         damageCount: 0,
                                                         stats: [0; 16],
                                                         persistant: [0; 16],
                                                         powerups: [0; 16],
                                                         ammo: [0; 16],
                                                         generic1: 0,
                                                         loopSound: 0,
                                                         jumppad_ent: 0,
                                                         ping: 0,
                                                         pmove_framecount: 0,
                                                         jumppad_frame: 0,
                                                         entityEventSequence:
                                                             0,},
                                       num_entities: 0,
                                       first_entity: 0,
                                       messageSent: 0,
                                       messageAcked: 0,
                                       messageSize: 0,}; 32],
                 ping: 0,
                 rate: 0,
                 snapshotMsec: 0,
                 pureAuthentic: 0,
                 gotCP: qfalse,
                 netchan:
                     netchan_t{sock: NS_CLIENT,
                               dropped: 0,
                               remoteAddress:
                                   netadr_t{type_0: NA_BAD,
                                            ip: [0; 4],
                                            ip6: [0; 16],
                                            port: 0,
                                            scope_id: 0,},
                               qport: 0,
                               incomingSequence: 0,
                               outgoingSequence: 0,
                               fragmentSequence: 0,
                               fragmentLength: 0,
                               fragmentBuffer: [0; 16384],
                               unsentFragments: qfalse,
                               unsentFragmentStart: 0,
                               unsentLength: 0,
                               unsentBuffer: [0; 16384],
                               challenge: 0,
                               lastSentTime: 0,
                               lastSentSize: 0,
                               compat: qfalse,},
                 netchan_start_queue: 0 as *mut netchan_buffer_t,
                 netchan_end_queue: 0 as *mut *mut netchan_buffer_t,
                 hasVoip: qfalse,
                 muteAllVoip: qfalse,
                 ignoreVoipFromClient: [qfalse; 64],
                 voipPacket: [0 as *mut voipServerPacket_t; 64],
                 queuedVoipPackets: 0,
                 queuedVoipIndex: 0,
                 oldServerTime: 0,
                 csUpdated: [qfalse; 1024],
                 compat: qfalse,};
    let mut ent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut clientNum: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut qport: libc::c_int = 0;
    let mut challenge: libc::c_int = 0;
    let mut password: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut startIndex: libc::c_int = 0;
    let mut denied: intptr_t = 0;
    let mut count: libc::c_int = 0;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut compat: qboolean = qfalse;
    Com_DPrintf(b"SVC_DirectConnect ()\n\x00" as *const u8 as
                    *const libc::c_char);
    if 0 != SV_IsBanned(&mut from, qfalse) as u64 {
        NET_OutOfBandPrint(NS_SERVER, from,
                           b"print\nYou are banned from this server.\n\x00" as
                               *const u8 as *const libc::c_char);
        return
    }
    Q_strncpyz(userinfo.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    version =
        atoi(Info_ValueForKey(userinfo.as_mut_ptr(),
                              b"protocol\x00" as *const u8 as
                                  *const libc::c_char));
    if version > 0i32 && (*com_legacyprotocol).integer == version {
        compat = qtrue
    } else if version != (*com_protocol).integer {
        NET_OutOfBandPrint(NS_SERVER, from,
                           b"print\nServer uses protocol version %i (yours is %i).\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*com_protocol).integer, version);
        Com_DPrintf(b"    rejected connect from version %i\n\x00" as *const u8
                        as *const libc::c_char, version);
        return
    }
    challenge =
        atoi(Info_ValueForKey(userinfo.as_mut_ptr(),
                              b"challenge\x00" as *const u8 as
                                  *const libc::c_char));
    qport =
        atoi(Info_ValueForKey(userinfo.as_mut_ptr(),
                              b"qport\x00" as *const u8 as
                                  *const libc::c_char));
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*cl).state as libc::c_uint ==
                 CS_FREE as libc::c_int as libc::c_uint) {
            if 0 !=
                   NET_CompareBaseAdr(from, (*cl).netchan.remoteAddress) as
                       libc::c_uint &&
                   ((*cl).netchan.qport == qport ||
                        from.port as libc::c_int ==
                            (*cl).netchan.remoteAddress.port as libc::c_int) {
                if svs.time - (*cl).lastConnectTime <
                       (*sv_reconnectlimit).integer * 1000i32 {
                    Com_DPrintf(b"%s:reconnect rejected : too soon\n\x00" as
                                    *const u8 as *const libc::c_char,
                                NET_AdrToString(from));
                    return
                }
                break ;
            }
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    if 0 != NET_IsLocalAddress(from) as u64 {
        ip =
            b"localhost\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else { ip = NET_AdrToString(from) as *mut libc::c_char }
    if strlen(ip).wrapping_add(strlen(userinfo.as_mut_ptr())).wrapping_add(4i32
                                                                               as
                                                                               libc::c_ulong)
           >= 1024i32 as libc::c_ulong {
        NET_OutOfBandPrint(NS_SERVER, from,
                           b"print\nUserinfo string length exceeded.  Try removing setu cvars from your config.\n\x00"
                               as *const u8 as *const libc::c_char);
        return
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(),
                        b"ip\x00" as *const u8 as *const libc::c_char, ip);
    if 0 == NET_IsLocalAddress(from) as u64 {
        let mut ping: libc::c_int = 0;
        let mut challengeptr: *mut challenge_t = 0 as *mut challenge_t;
        i = 0i32;
        while i < 2048i32 {
            if 0 !=
                   NET_CompareAdr(from, svs.challenges[i as usize].adr) as u64
               {
                if challenge == svs.challenges[i as usize].challenge {
                    break ;
                }
            }
            i += 1
        }
        if i == 2048i32 {
            NET_OutOfBandPrint(NS_SERVER, from,
                               b"print\nNo or bad challenge for your address.\n\x00"
                                   as *const u8 as *const libc::c_char);
            return
        }
        challengeptr = &mut svs.challenges[i as usize] as *mut challenge_t;
        if 0 != (*challengeptr).wasrefused as u64 { return }
        ping = svs.time - (*challengeptr).pingTime;
        if 0 == Sys_IsLANAddress(from) as u64 {
            if 0. != (*sv_minPing).value &&
                   (ping as libc::c_float) < (*sv_minPing).value {
                NET_OutOfBandPrint(NS_SERVER, from,
                                   b"print\nServer is for high pings only\n\x00"
                                       as *const u8 as *const libc::c_char);
                Com_DPrintf(b"Client %i rejected on a too low ping\n\x00" as
                                *const u8 as *const libc::c_char, i);
                (*challengeptr).wasrefused = qtrue;
                return
            }
            if 0. != (*sv_maxPing).value &&
                   ping as libc::c_float > (*sv_maxPing).value {
                NET_OutOfBandPrint(NS_SERVER, from,
                                   b"print\nServer is for low pings only\n\x00"
                                       as *const u8 as *const libc::c_char);
                Com_DPrintf(b"Client %i rejected on a too high ping\n\x00" as
                                *const u8 as *const libc::c_char, i);
                (*challengeptr).wasrefused = qtrue;
                return
            }
        }
        Com_Printf(b"Client %i connecting with %i challenge ping\n\x00" as
                       *const u8 as *const libc::c_char, i, ping);
        (*challengeptr).connected = qtrue
    }
    newcl = &mut temp;
    memset(newcl as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<client_t>() as libc::c_ulong);
    // if there is already a slot for this ip, reuse it
    i = 0i32;
    cl = svs.clients;
    loop  {
        if !(i < (*sv_maxclients).integer) {
            current_block = 479107131381816815;
            break ;
        }
        if !((*cl).state as libc::c_uint ==
                 CS_FREE as libc::c_int as libc::c_uint) {
            if 0 !=
                   NET_CompareBaseAdr(from, (*cl).netchan.remoteAddress) as
                       libc::c_uint &&
                   ((*cl).netchan.qport == qport ||
                        from.port as libc::c_int ==
                            (*cl).netchan.remoteAddress.port as libc::c_int) {
                Com_Printf(b"%s:reconnect\n\x00" as *const u8 as
                               *const libc::c_char, NET_AdrToString(from));
                newcl = cl;
                // this doesn't work because it nukes the players userinfo
                //			// disconnect the client from the game first so any flags the
//			// player might have are dropped
//			VM_Call( gvm, GAME_CLIENT_DISCONNECT, newcl - svs.clients );
			//
                current_block = 414540557241433404;
                break ;
            }
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    match current_block {
        479107131381816815 => {
            password =
                Info_ValueForKey(userinfo.as_mut_ptr(),
                                 b"password\x00" as *const u8 as
                                     *const libc::c_char);
            if 0 != *password as libc::c_int &&
                   0 == strcmp(password, (*sv_privatePassword).string) {
                startIndex = 0i32
            } else { startIndex = (*sv_privateClients).integer }
            newcl = 0 as *mut client_t;
            i = startIndex;
            while i < (*sv_maxclients).integer {
                cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
                if (*cl).state as libc::c_uint ==
                       CS_FREE as libc::c_int as libc::c_uint {
                    newcl = cl;
                    break ;
                } else { i += 1 }
            }
            if newcl.is_null() {
                if 0 != NET_IsLocalAddress(from) as u64 {
                    count = 0i32;
                    i = startIndex;
                    while i < (*sv_maxclients).integer {
                        cl =
                            &mut *svs.clients.offset(i as isize) as
                                *mut client_t;
                        if (*cl).netchan.remoteAddress.type_0 as libc::c_uint
                               == NA_BOT as libc::c_int as libc::c_uint {
                            count += 1
                        }
                        i += 1
                    }
                    if count >= (*sv_maxclients).integer - startIndex {
                        SV_DropClient(&mut *svs.clients.offset(((*sv_maxclients).integer
                                                                    - 1i32) as
                                                                   isize),
                                      b"only bots on server\x00" as *const u8
                                          as *const libc::c_char);
                        newcl =
                            &mut *svs.clients.offset(((*sv_maxclients).integer
                                                          - 1i32) as isize) as
                                *mut client_t
                    } else {
                        Com_Error(ERR_FATAL as libc::c_int,
                                  b"server is full on local connect\x00" as
                                      *const u8 as *const libc::c_char);
                    }
                } else {
                    NET_OutOfBandPrint(NS_SERVER, from,
                                       b"print\nServer is full.\n\x00" as
                                           *const u8 as *const libc::c_char);
                    Com_DPrintf(b"Rejected a connection.\n\x00" as *const u8
                                    as *const libc::c_char);
                    return
                }
            }
            (*cl).reliableAcknowledge = 0i32;
            (*cl).reliableSequence = 0i32
        }
        _ => { }
    }
    *newcl = temp;
    clientNum =
        newcl.wrapping_offset_from(svs.clients) as libc::c_long as
            libc::c_int;
    ent = SV_GentityNum(clientNum);
    (*newcl).gentity = ent;
    (*newcl).challenge = challenge;
    (*newcl).compat = compat;
    Netchan_Setup(NS_SERVER, &mut (*newcl).netchan, from, qport, challenge,
                  compat);
    (*newcl).netchan_end_queue = &mut (*newcl).netchan_start_queue;
    Q_strncpyz((*newcl).userinfo.as_mut_ptr(), userinfo.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    denied =
        VM_Call(gvm, GAME_CLIENT_CONNECT as libc::c_int, clientNum,
                qtrue as libc::c_int, qfalse as libc::c_int);
    if 0 != denied {
        let mut str: *mut libc::c_char =
            VM_ExplicitArgPtr(gvm, denied) as *mut libc::c_char;
        NET_OutOfBandPrint(NS_SERVER, from,
                           b"print\n%s\n\x00" as *const u8 as
                               *const libc::c_char, str);
        Com_DPrintf(b"Game rejected a connection: %s.\n\x00" as *const u8 as
                        *const libc::c_char, str);
        return
    }
    SV_UserinfoChanged(newcl);
    NET_OutOfBandPrint(NS_SERVER, from,
                       b"connectResponse %d\x00" as *const u8 as
                           *const libc::c_char, challenge);
    Com_DPrintf(b"Going from CS_FREE to CS_CONNECTED for %s\n\x00" as
                    *const u8 as *const libc::c_char,
                (*newcl).name.as_mut_ptr());
    (*newcl).state = CS_CONNECTED;
    (*newcl).lastSnapshotTime = 0i32;
    (*newcl).lastPacketTime = svs.time;
    (*newcl).lastConnectTime = svs.time;
    (*newcl).gamestateMessageNum = -1i32;
    count = 0i32;
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            count += 1
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    if count == 1i32 || count == (*sv_maxclients).integer {
        SV_Heartbeat_f();
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_UserinfoChanged(mut cl: *mut client_t) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    Q_strncpyz((*cl).name.as_mut_ptr(),
               Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                                b"name\x00" as *const u8 as
                                    *const libc::c_char),
               ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as
                   libc::c_int);
    if 0 != Sys_IsLANAddress((*cl).netchan.remoteAddress) as libc::c_uint &&
           (*com_dedicated).integer != 2i32 &&
           (*sv_lanForceRate).integer == 1i32 {
        (*cl).rate = 99999i32
    } else {
        val =
            Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                             b"rate\x00" as *const u8 as *const libc::c_char);
        if 0 != strlen(val) {
            i = atoi(val);
            (*cl).rate = i;
            if (*cl).rate < 1000i32 {
                (*cl).rate = 1000i32
            } else if (*cl).rate > 90000i32 { (*cl).rate = 90000i32 }
        } else { (*cl).rate = 3000i32 }
    }
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"handicap\x00" as *const u8 as *const libc::c_char);
    if 0 != strlen(val) {
        i = atoi(val);
        if i <= 0i32 || i > 100i32 || strlen(val) > 4i32 as libc::c_ulong {
            Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                                b"handicap\x00" as *const u8 as
                                    *const libc::c_char,
                                b"100\x00" as *const u8 as
                                    *const libc::c_char);
        }
    }
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"snaps\x00" as *const u8 as *const libc::c_char);
    if 0 != strlen(val) {
        i = atoi(val);
        if i < 1i32 {
            i = 1i32
        } else if i > (*sv_fps).integer { i = (*sv_fps).integer }
        i = 1000i32 / i
    } else { i = 50i32 }
    if i != (*cl).snapshotMsec {
        (*cl).lastSnapshotTime = 0i32;
        (*cl).snapshotMsec = i
    }
    if 0 != (*cl).compat as u64 {
        (*cl).hasVoip = qfalse
    } else {
        val =
            Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                             b"cl_voipProtocol\x00" as *const u8 as
                                 *const libc::c_char);
        (*cl).hasVoip =
            (0 ==
                 Q_stricmp(val,
                           b"opus\x00" as *const u8 as *const libc::c_char))
                as libc::c_int as qboolean
    }
    if 0 != NET_IsLocalAddress((*cl).netchan.remoteAddress) as u64 {
        ip =
            b"localhost\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else {
        ip = NET_AdrToString((*cl).netchan.remoteAddress) as *mut libc::c_char
    }
    val =
        Info_ValueForKey((*cl).userinfo.as_mut_ptr(),
                         b"ip\x00" as *const u8 as *const libc::c_char);
    if 0 != *val.offset(0isize) {
        len =
            strlen(ip).wrapping_sub(strlen(val)).wrapping_add(strlen((*cl).userinfo.as_mut_ptr()))
                as libc::c_int
    } else {
        len =
            strlen(ip).wrapping_add(4i32 as
                                        libc::c_ulong).wrapping_add(strlen((*cl).userinfo.as_mut_ptr()))
                as libc::c_int
    }
    if len >= 1024i32 {
        SV_DropClient(cl,
                      b"userinfo string length exceeded\x00" as *const u8 as
                          *const libc::c_char);
    } else {
        Info_SetValueForKey((*cl).userinfo.as_mut_ptr(),
                            b"ip\x00" as *const u8 as *const libc::c_char,
                            ip);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_DropClient(mut drop_0: *mut client_t,
                                       mut reason: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut challenge: *mut challenge_t = 0 as *mut challenge_t;
    let isBot: qboolean =
        ((*drop_0).netchan.remoteAddress.type_0 as libc::c_uint ==
             NA_BOT as libc::c_int as libc::c_uint) as libc::c_int as
            qboolean;
    if (*drop_0).state as libc::c_uint ==
           CS_ZOMBIE as libc::c_int as libc::c_uint {
        return
    }
    if 0 == isBot as u64 {
        challenge = &mut svs.challenges[0usize] as *mut challenge_t;
        i = 0i32;
        while i < 2048i32 {
            if 0 !=
                   NET_CompareAdr((*drop_0).netchan.remoteAddress,
                                  (*challenge).adr) as u64 {
                memset(challenge as *mut libc::c_void, 0i32,
                       ::std::mem::size_of::<challenge_t>() as libc::c_ulong);
                break ;
            } else { i += 1; challenge = challenge.offset(1isize) }
        }
    }
    SV_FreeClient(drop_0);
    SV_SendServerCommand(0 as *mut client_t,
                         b"print \"%s^7 %s\n\"\x00" as *const u8 as
                             *const libc::c_char, (*drop_0).name.as_mut_ptr(),
                         reason);
    VM_Call(gvm, GAME_CLIENT_DISCONNECT as libc::c_int,
            drop_0.wrapping_offset_from(svs.clients) as libc::c_long);
    SV_SendServerCommand(drop_0,
                         b"disconnect \"%s\"\x00" as *const u8 as
                             *const libc::c_char, reason);
    if 0 != isBot as u64 {
        SV_BotFreeClient(drop_0.wrapping_offset_from(svs.clients) as
                             libc::c_long as libc::c_int);
        (*drop_0).state = CS_FREE
    } else {
        Com_DPrintf(b"Going to CS_ZOMBIE for %s\n\x00" as *const u8 as
                        *const libc::c_char, (*drop_0).name.as_mut_ptr());
        (*drop_0).state = CS_ZOMBIE
    }
    SV_SetUserinfo(drop_0.wrapping_offset_from(svs.clients) as libc::c_long as
                       libc::c_int,
                   b"\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            break ;
        }
        i += 1
    }
    if i == (*sv_maxclients).integer { SV_Heartbeat_f(); };
}
#[no_mangle]
pub unsafe extern "C" fn SV_FreeClient(mut client: *mut client_t) {
    let mut index: libc::c_int = 0;
    index = (*client).queuedVoipIndex;
    while index < (*client).queuedVoipPackets {
        index =
            (index as
                 libc::c_ulong).wrapping_rem((::std::mem::size_of::<[*mut voipServerPacket_t; 64]>()
                                                  as
                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut voipServerPacket_t>()
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int;
        Z_Free((*client).voipPacket[index as usize] as *mut libc::c_void);
        index += 1
    }
    (*client).queuedVoipPackets = 0i32;
    SV_Netchan_FreeQueue(client);
    SV_CloseDownload(client);
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
// sv_client.c -- server code for dealing with clients
unsafe extern "C" fn SV_CloseDownload(mut cl: *mut client_t) {
    let mut i: libc::c_int = 0;
    if 0 != (*cl).download { FS_FCloseFile((*cl).download); }
    (*cl).download = 0i32;
    *(*cl).downloadName.as_mut_ptr() = 0i32 as libc::c_char;
    i = 0i32;
    while i < 48i32 {
        if !(*cl).downloadBlocks[i as usize].is_null() {
            Z_Free((*cl).downloadBlocks[i as usize] as *mut libc::c_void);
            (*cl).downloadBlocks[i as usize] = 0 as *mut libc::c_uchar
        }
        i += 1
    };
}
/*
==================
SV_IsBanned

Check whether a certain address is banned
==================
*/
unsafe extern "C" fn SV_IsBanned(mut from: *mut netadr_t,
                                 mut isexception: qboolean) -> qboolean {
    let mut index: libc::c_int = 0;
    let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
    if 0 == isexception as u64 {
        if 0 != SV_IsBanned(from, qtrue) as u64 { return qfalse }
    }
    index = 0i32;
    while index < serverBansCount {
        curban = &mut serverBans[index as usize] as *mut serverBan_t;
        if (*curban).isexception as libc::c_uint ==
               isexception as libc::c_uint {
            if 0 !=
                   NET_CompareBaseAdrMask((*curban).ip, *from,
                                          (*curban).subnet) as u64 {
                return qtrue
            }
        }
        index += 1
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn SV_AuthorizeIpPacket(mut from: netadr_t) {
    let mut challenge: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut challengeptr: *mut challenge_t = 0 as *mut challenge_t;
    if 0 == NET_CompareBaseAdr(from, svs.authorizeAddress) as u64 {
        Com_Printf(b"SV_AuthorizeIpPacket: not from authorize server\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    challenge = atoi(Cmd_Argv(1i32));
    i = 0i32;
    while i < 2048i32 {
        if svs.challenges[i as usize].challenge == challenge { break ; }
        i += 1
    }
    if i == 2048i32 {
        Com_Printf(b"SV_AuthorizeIpPacket: challenge not found\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    challengeptr = &mut svs.challenges[i as usize] as *mut challenge_t;
    (*challengeptr).pingTime = svs.time;
    s = Cmd_Argv(2i32);
    r = Cmd_Argv(3i32);
    if 0 == Q_stricmp(s, b"demo\x00" as *const u8 as *const libc::c_char) {
        NET_OutOfBandPrint(NS_SERVER, (*challengeptr).adr,
                           b"print\nServer is not a demo server\n\x00" as
                               *const u8 as *const libc::c_char);
        memset(challengeptr as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<challenge_t>() as libc::c_ulong);
        return
    }
    if 0 == Q_stricmp(s, b"accept\x00" as *const u8 as *const libc::c_char) {
        NET_OutOfBandPrint(NS_SERVER, (*challengeptr).adr,
                           b"challengeResponse %d %d %d\x00" as *const u8 as
                               *const libc::c_char, (*challengeptr).challenge,
                           (*challengeptr).clientChallenge,
                           (*com_protocol).integer);
        return
    }
    if 0 == Q_stricmp(s, b"unknown\x00" as *const u8 as *const libc::c_char) {
        if r.is_null() {
            NET_OutOfBandPrint(NS_SERVER, (*challengeptr).adr,
                               b"print\nAwaiting CD key authorization\n\x00"
                                   as *const u8 as *const libc::c_char);
        } else {
            NET_OutOfBandPrint(NS_SERVER, (*challengeptr).adr,
                               b"print\n%s\n\x00" as *const u8 as
                                   *const libc::c_char, r);
        }
        memset(challengeptr as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<challenge_t>() as libc::c_ulong);
        return
    }
    if r.is_null() {
        NET_OutOfBandPrint(NS_SERVER, (*challengeptr).adr,
                           b"print\nSomeone is using this CD Key\n\x00" as
                               *const u8 as *const libc::c_char);
    } else {
        NET_OutOfBandPrint(NS_SERVER, (*challengeptr).adr,
                           b"print\n%s\n\x00" as *const u8 as
                               *const libc::c_char, r);
    }
    memset(challengeptr as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<challenge_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ExecuteClientMessage(mut cl: *mut client_t,
                                                 mut msg: *mut msg_t) {
    let mut c: libc::c_int = 0;
    let mut serverId: libc::c_int = 0;
    MSG_Bitstream(msg);
    serverId = MSG_ReadLong(msg);
    (*cl).messageAcknowledge = MSG_ReadLong(msg);
    if (*cl).messageAcknowledge < 0i32 {
        SV_DropClient(cl,
                      b"DEBUG: illegible client message\x00" as *const u8 as
                          *const libc::c_char);
        return
    }
    (*cl).reliableAcknowledge = MSG_ReadLong(msg);
    if (*cl).reliableAcknowledge < (*cl).reliableSequence - 64i32 {
        SV_DropClient(cl,
                      b"DEBUG: illegible client message\x00" as *const u8 as
                          *const libc::c_char);
        (*cl).reliableAcknowledge = (*cl).reliableSequence;
        return
    }
    if serverId != sv.serverId && 0 == *(*cl).downloadName.as_mut_ptr() &&
           strstr((*cl).lastClientCommandString.as_mut_ptr(),
                  b"nextdl\x00" as *const u8 as *const libc::c_char).is_null()
       {
        if serverId >= sv.restartedServerId && serverId < sv.serverId {
            Com_DPrintf(b"%s : ignoring pre map_restart / outdated client message\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*cl).name.as_mut_ptr());
            return
        }
        if (*cl).state as libc::c_uint !=
               CS_ACTIVE as libc::c_int as libc::c_uint &&
               (*cl).messageAcknowledge > (*cl).gamestateMessageNum {
            Com_DPrintf(b"%s : dropped gamestate, resending\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*cl).name.as_mut_ptr());
            SV_SendClientGameState(cl);
        }
        return
    }
    if 0 != (*cl).oldServerTime && serverId == sv.serverId {
        Com_DPrintf(b"%s acknowledged gamestate\n\x00" as *const u8 as
                        *const libc::c_char, (*cl).name.as_mut_ptr());
        (*cl).oldServerTime = 0i32
    }
    loop  {
        c = MSG_ReadByte(msg);
        if c == clc_EOF as libc::c_int { break ; }
        if c != clc_clientCommand as libc::c_int { break ; }
        if 0 == SV_ClientCommand(cl, msg) as u64 { return }
        if (*cl).state as libc::c_uint ==
               CS_ZOMBIE as libc::c_int as libc::c_uint {
            return
        }
    }
    if c == clc_voipSpeex as libc::c_int {
        SV_UserVoip(cl, msg, qtrue);
        c = MSG_ReadByte(msg)
    }
    if c == clc_voipOpus as libc::c_int {
        SV_UserVoip(cl, msg, qfalse);
        c = MSG_ReadByte(msg)
    }
    if c == clc_move as libc::c_int {
        SV_UserMove(cl, msg, qtrue);
    } else if c == clc_moveNoDelta as libc::c_int {
        SV_UserMove(cl, msg, qfalse);
    } else if c != clc_EOF as libc::c_int {
        Com_Printf(b"WARNING: bad command byte for client %i\n\x00" as
                       *const u8 as *const libc::c_char,
                   cl.wrapping_offset_from(svs.clients) as libc::c_long as
                       libc::c_int);
    };
}
/*
==================
SV_UserMove

The message usually contains all the movement commands 
that were in the last three packets, so that the information
in dropped packets can be recovered.

On very fast clients, there may be multiple usercmd packed into
each of the backup packets.
==================
*/
unsafe extern "C" fn SV_UserMove(mut cl: *mut client_t, mut msg: *mut msg_t,
                                 mut delta: qboolean) {
    let mut i: libc::c_int = 0;
    let mut key: libc::c_int = 0;
    let mut cmdCount: libc::c_int = 0;
    let mut nullcmd: usercmd_t =
        usercmd_s{serverTime: 0,
                  angles: [0; 3],
                  buttons: 0,
                  weapon: 0,
                  forwardmove: 0,
                  rightmove: 0,
                  upmove: 0,};
    let mut cmds: [usercmd_t; 32] =
        [usercmd_s{serverTime: 0,
                   angles: [0; 3],
                   buttons: 0,
                   weapon: 0,
                   forwardmove: 0,
                   rightmove: 0,
                   upmove: 0,}; 32];
    let mut cmd: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut oldcmd: *mut usercmd_t = 0 as *mut usercmd_t;
    if 0 != delta as u64 {
        (*cl).deltaMessage = (*cl).messageAcknowledge
    } else { (*cl).deltaMessage = -1i32 }
    cmdCount = MSG_ReadByte(msg);
    if cmdCount < 1i32 {
        Com_Printf(b"cmdCount < 1\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    if cmdCount > 32i32 {
        Com_Printf(b"cmdCount > MAX_PACKET_USERCMDS\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    key = sv.checksumFeed;
    key ^= (*cl).messageAcknowledge;
    key ^=
        MSG_HashKey((*cl).reliableCommands[((*cl).reliableAcknowledge &
                                                64i32 - 1i32) as
                                               usize].as_mut_ptr(), 32i32);
    memset(&mut nullcmd as *mut usercmd_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    oldcmd = &mut nullcmd;
    i = 0i32;
    while i < cmdCount {
        cmd = &mut cmds[i as usize] as *mut usercmd_t;
        MSG_ReadDeltaUsercmdKey(msg, key, oldcmd, cmd);
        oldcmd = cmd;
        i += 1
    }
    (*cl).frames[((*cl).messageAcknowledge & 32i32 - 1i32) as
                     usize].messageAcked = svs.time;
    if (*sv_pure).integer != 0i32 && (*cl).pureAuthentic == 0i32 &&
           0 == (*cl).gotCP as u64 {
        if (*cl).state as libc::c_uint ==
               CS_ACTIVE as libc::c_int as libc::c_uint {
            Com_DPrintf(b"%s: didn\'t get cp command, resending gamestate\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*cl).name.as_mut_ptr());
            SV_SendClientGameState(cl);
        }
        return
    }
    if (*cl).state as libc::c_uint == CS_PRIMED as libc::c_int as libc::c_uint
       {
        SV_ClientEnterWorld(cl, &mut cmds[0usize]);
    }
    if (*sv_pure).integer != 0i32 && (*cl).pureAuthentic == 0i32 {
        SV_DropClient(cl,
                      b"Cannot validate pure client!\x00" as *const u8 as
                          *const libc::c_char);
        return
    }
    if (*cl).state as libc::c_uint != CS_ACTIVE as libc::c_int as libc::c_uint
       {
        (*cl).deltaMessage = -1i32;
        return
    }
    i = 0i32;
    while i < cmdCount {
        // if this is a cmd from before a map_restart ignore it
        if !(cmds[i as usize].serverTime >
                 cmds[(cmdCount - 1i32) as usize].serverTime) {
            // extremely lagged or cmd from before a map_restart
		//if ( cmds[i].serverTime > svs.time + 3000 ) {
		//	continue;
		//}
		// don't execute if this is an old cmd which is already executed
		// these old cmds are included when cl_packetdup > 0
            if !(cmds[i as usize].serverTime <= (*cl).lastUsercmd.serverTime)
               {
                SV_ClientThink(cl, &mut cmds[i as usize]);
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClientThink(mut cl: *mut client_t,
                                        mut cmd: *mut usercmd_t) {
    (*cl).lastUsercmd = *cmd;
    if (*cl).state as libc::c_uint != CS_ACTIVE as libc::c_int as libc::c_uint
       {
        return
    }
    VM_Call(gvm, GAME_CLIENT_THINK as libc::c_int,
            cl.wrapping_offset_from(svs.clients) as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClientEnterWorld(mut client: *mut client_t,
                                             mut cmd: *mut usercmd_t) {
    let mut clientNum: libc::c_int = 0;
    let mut ent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    Com_DPrintf(b"Going from CS_PRIMED to CS_ACTIVE for %s\n\x00" as *const u8
                    as *const libc::c_char, (*client).name.as_mut_ptr());
    (*client).state = CS_ACTIVE;
    SV_UpdateConfigstrings(client);
    clientNum =
        client.wrapping_offset_from(svs.clients) as libc::c_long as
            libc::c_int;
    ent = SV_GentityNum(clientNum);
    (*ent).s.number = clientNum;
    (*client).gentity = ent;
    (*client).deltaMessage = -1i32;
    (*client).lastSnapshotTime = 0i32;
    if !cmd.is_null() {
        memcpy(&mut (*client).lastUsercmd as *mut usercmd_t as
                   *mut libc::c_void, cmd as *const libc::c_void,
               ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    } else {
        memset(&mut (*client).lastUsercmd as *mut usercmd_t as
                   *mut libc::c_void, '\u{0}' as i32,
               ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
    }
    VM_Call(gvm, GAME_CLIENT_BEGIN as libc::c_int,
            client.wrapping_offset_from(svs.clients) as libc::c_long);
}
/*
================
SV_SendClientGameState

Sends the first message from the server to a connected client.
This will be sent on the initial connection and upon each new map load.

It will be resent if the client acknowledges a later message but has
the wrong gamestate.
================
*/
unsafe extern "C" fn SV_SendClientGameState(mut client: *mut client_t) {
    let mut start: libc::c_int = 0;
    let mut base: *mut entityState_t = 0 as *mut entityState_t;
    let mut nullstate: entityState_t =
        entityState_s{number: 0,
                      eType: 0,
                      eFlags: 0,
                      pos:
                          trajectory_t{trType: TR_STATIONARY,
                                       trTime: 0,
                                       trDuration: 0,
                                       trBase: [0.; 3],
                                       trDelta: [0.; 3],},
                      apos:
                          trajectory_t{trType: TR_STATIONARY,
                                       trTime: 0,
                                       trDuration: 0,
                                       trBase: [0.; 3],
                                       trDelta: [0.; 3],},
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
                      generic1: 0,};
    let mut msg: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut msgBuffer: [byte; 16384] = [0; 16384];
    Com_DPrintf(b"SV_SendClientGameState() for %s\n\x00" as *const u8 as
                    *const libc::c_char, (*client).name.as_mut_ptr());
    Com_DPrintf(b"Going from CS_CONNECTED to CS_PRIMED for %s\n\x00" as
                    *const u8 as *const libc::c_char,
                (*client).name.as_mut_ptr());
    (*client).state = CS_PRIMED;
    (*client).pureAuthentic = 0i32;
    (*client).gotCP = qfalse;
    (*client).gamestateMessageNum = (*client).netchan.outgoingSequence;
    MSG_Init(&mut msg, msgBuffer.as_mut_ptr(),
             ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                 libc::c_int);
    MSG_WriteLong(&mut msg, (*client).lastClientCommand);
    SV_UpdateServerCommandsToClient(client, &mut msg);
    MSG_WriteByte(&mut msg, svc_gamestate as libc::c_int);
    MSG_WriteLong(&mut msg, (*client).reliableSequence);
    start = 0i32;
    while start < 1024i32 {
        if 0 != *sv.configstrings[start as usize].offset(0isize) {
            MSG_WriteByte(&mut msg, svc_configstring as libc::c_int);
            MSG_WriteShort(&mut msg, start);
            MSG_WriteBigString(&mut msg, sv.configstrings[start as usize]);
        }
        start += 1
    }
    memset(&mut nullstate as *mut entityState_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<entityState_t>() as libc::c_ulong);
    start = 0i32;
    while start < 1i32 << 10i32 {
        base = &mut sv.svEntities[start as usize].baseline;
        if !(0 == (*base).number) {
            MSG_WriteByte(&mut msg, svc_baseline as libc::c_int);
            MSG_WriteDeltaEntity(&mut msg, &mut nullstate, base, qtrue);
        }
        start += 1
    }
    MSG_WriteByte(&mut msg, svc_EOF as libc::c_int);
    MSG_WriteLong(&mut msg,
                  client.wrapping_offset_from(svs.clients) as libc::c_long as
                      libc::c_int);
    MSG_WriteLong(&mut msg, sv.checksumFeed);
    SV_SendMessageToClient(&mut msg, client);
}
unsafe extern "C" fn SV_UserVoip(mut cl: *mut client_t, mut msg: *mut msg_t,
                                 mut ignoreData: qboolean) {
    let mut sender: libc::c_int = 0;
    let mut generation: libc::c_int = 0;
    let mut sequence: libc::c_int = 0;
    let mut frames: libc::c_int = 0;
    let mut packetsize: libc::c_int = 0;
    let mut recips: [uint8_t; 8] = [0; 8];
    let mut flags: libc::c_int = 0;
    let mut encoded: [byte; 4000] = [0; 4000];
    let mut client: *mut client_t = 0 as *mut client_t;
    let mut packet: *mut voipServerPacket_t = 0 as *mut voipServerPacket_t;
    let mut i: libc::c_int = 0;
    sender =
        cl.wrapping_offset_from(svs.clients) as libc::c_long as libc::c_int;
    generation = MSG_ReadByte(msg);
    sequence = MSG_ReadLong(msg);
    frames = MSG_ReadByte(msg);
    MSG_ReadData(msg, recips.as_mut_ptr() as *mut libc::c_void,
                 ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong as
                     libc::c_int);
    flags = MSG_ReadByte(msg);
    packetsize = MSG_ReadShort(msg);
    if (*msg).readcount > (*msg).cursize { return }
    if packetsize as libc::c_ulong >
           ::std::mem::size_of::<[byte; 4000]>() as libc::c_ulong {
        let mut bytesleft: libc::c_int = packetsize;
        while 0 != bytesleft {
            let mut br: libc::c_int = bytesleft;
            if br as libc::c_ulong >
                   ::std::mem::size_of::<[byte; 4000]>() as libc::c_ulong {
                br =
                    ::std::mem::size_of::<[byte; 4000]>() as libc::c_ulong as
                        libc::c_int
            }
            MSG_ReadData(msg, encoded.as_mut_ptr() as *mut libc::c_void, br);
            bytesleft -= br
        }
        return
    }
    MSG_ReadData(msg, encoded.as_mut_ptr() as *mut libc::c_void, packetsize);
    if 0 != ignoreData as libc::c_uint ||
           0 != SV_ShouldIgnoreVoipSender(cl) as libc::c_uint {
        return
    }
    i = 0i32;
    client = svs.clients;
    while i < (*sv_maxclients).integer {
        if !((*client).state as libc::c_uint !=
                 CS_ACTIVE as libc::c_int as libc::c_uint) {
            // not in the game yet, don't send to this guy.
            if !(i == sender) {
                // don't send voice packet back to original author.
                if !(0 == (*client).hasVoip as u64) {
                    // no VoIP support, or unsupported protocol
                    if !(0 != (*client).muteAllVoip as u64) {
                        // client is ignoring everyone.
                        if !(0 !=
                                 (*client).ignoreVoipFromClient[sender as
                                                                    usize] as
                                     u64) {
                            // client is ignoring this talker.
                            // !!! FIXME: possible to DoS?
                            if !(0 != *(*cl).downloadName.as_mut_ptr()) {
                                // no VoIP allowed if downloading, to save bandwidth.
                                if 0 !=
                                       Com_IsVoipTarget(recips.as_mut_ptr(),
                                                        ::std::mem::size_of::<[uint8_t; 8]>()
                                                            as libc::c_ulong
                                                            as libc::c_int, i)
                                           as u64 {
                                    flags |= 0x2i32
                                } else { flags &= !0x2i32 }
                                if !(0 == flags & (0x1i32 | 0x2i32)) {
                                    // not addressed to this player.
                                    // Transmit this packet to the client.
                                    if (*client).queuedVoipPackets as
                                           libc::c_ulong >=
                                           (::std::mem::size_of::<[*mut voipServerPacket_t; 64]>()
                                                as
                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut voipServerPacket_t>()
                                                                                as
                                                                                libc::c_ulong)
                                       {
                                        Com_Printf(b"Too many VoIP packets queued for client #%d\n\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   i);
                                    } else {
                                        // no room for another packet right now.
                                        packet =
                                            Z_MallocDebug(::std::mem::size_of::<voipServerPacket_t>()
                                                              as libc::c_ulong
                                                              as libc::c_int,
                                                          b"sizeof(*packet)\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char,
                                                          b"code/server/sv_client.c\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char,
                                                          1866i32) as
                                                *mut voipServerPacket_t;
                                        (*packet).sender = sender;
                                        (*packet).frames = frames;
                                        (*packet).len = packetsize;
                                        (*packet).generation = generation;
                                        (*packet).sequence = sequence;
                                        (*packet).flags = flags;
                                        memcpy((*packet).data.as_mut_ptr() as
                                                   *mut libc::c_void,
                                               encoded.as_mut_ptr() as
                                                   *const libc::c_void,
                                               packetsize as libc::c_ulong);
                                        (*client).voipPacket[(((*client).queuedVoipIndex
                                                                   +
                                                                   (*client).queuedVoipPackets)
                                                                  as
                                                                  libc::c_ulong).wrapping_rem((::std::mem::size_of::<[*mut voipServerPacket_t; 64]>()
                                                                                                   as
                                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut voipServerPacket_t>()
                                                                                                                                   as
                                                                                                                                   libc::c_ulong))
                                                                 as usize] =
                                            packet;
                                        (*client).queuedVoipPackets += 1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        client = client.offset(1isize)
    };
}
/*
==================
SV_ShouldIgnoreVoipSender

Blocking of voip packets based on source client
==================
*/
unsafe extern "C" fn SV_ShouldIgnoreVoipSender(mut cl: *const client_t)
 -> qboolean {
    if 0 == (*sv_voip).integer {
        return qtrue
    } else { if 0 == (*cl).hasVoip as u64 { return qtrue } }
    return qfalse;
}
/*
===============
SV_ClientCommand
===============
*/
unsafe extern "C" fn SV_ClientCommand(mut cl: *mut client_t,
                                      mut msg: *mut msg_t) -> qboolean {
    let mut seq: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut clientOk: qboolean = qtrue;
    seq = MSG_ReadLong(msg);
    s = MSG_ReadString(msg);
    if (*cl).lastClientCommand >= seq { return qtrue }
    Com_DPrintf(b"clientCommand: %s : %i : %s\n\x00" as *const u8 as
                    *const libc::c_char, (*cl).name.as_mut_ptr(), seq, s);
    if seq > (*cl).lastClientCommand + 1i32 {
        Com_Printf(b"Client %s lost %i clientCommands\n\x00" as *const u8 as
                       *const libc::c_char, (*cl).name.as_mut_ptr(),
                   seq - (*cl).lastClientCommand + 1i32);
        SV_DropClient(cl,
                      b"Lost reliable commands\x00" as *const u8 as
                          *const libc::c_char);
        return qfalse
    }
    if 0 == (*com_cl_running).integer &&
           (*cl).state as libc::c_uint >=
               CS_ACTIVE as libc::c_int as libc::c_uint &&
           0 != (*sv_floodProtect).integer &&
           svs.time < (*cl).nextReliableTime {
        clientOk = qfalse
    }
    (*cl).nextReliableTime = svs.time + 1000i32;
    SV_ExecuteClientCommand(cl, s, clientOk);
    (*cl).lastClientCommand = seq;
    Com_sprintf((*cl).lastClientCommandString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char, s);
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ExecuteClientCommand(mut cl: *mut client_t,
                                                 mut s: *const libc::c_char,
                                                 mut clientOK: qboolean) {
    let mut u: *mut ucmd_t = 0 as *mut ucmd_t;
    let mut bProcessed: qboolean = qfalse;
    Cmd_TokenizeString(s);
    u = ucmds.as_mut_ptr();
    while !(*u).name.is_null() {
        if 0 == strcmp(Cmd_Argv(0i32), (*u).name) {
            (*u).func.expect("non-null function pointer")(cl);
            bProcessed = qtrue;
            break ;
        } else { u = u.offset(1isize) }
    }
    if 0 != clientOK as u64 {
        if (*u).name.is_null() &&
               sv.state as libc::c_uint ==
                   SS_GAME as libc::c_int as libc::c_uint &&
               ((*cl).state as libc::c_uint ==
                    CS_ACTIVE as libc::c_int as libc::c_uint ||
                    (*cl).state as libc::c_uint ==
                        CS_PRIMED as libc::c_int as libc::c_uint) {
            Cmd_Args_Sanitize();
            VM_Call(gvm, GAME_CLIENT_COMMAND as libc::c_int,
                    cl.wrapping_offset_from(svs.clients) as libc::c_long);
        }
    } else if 0 == bProcessed as u64 {
        Com_DPrintf(b"client text ignored for %s: %s\n\x00" as *const u8 as
                        *const libc::c_char, (*cl).name.as_mut_ptr(),
                    Cmd_Argv(0i32));
    };
}
static mut ucmds: [ucmd_t; 10] =
    [ucmd_t{name:
                b"userinfo\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_UpdateUserinfo_f),},
     ucmd_t{name:
                b"disconnect\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_Disconnect_f),},
     ucmd_t{name:
                b"cp\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_VerifyPaks_f),},
     ucmd_t{name:
                b"vdr\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_ResetPureClient_f),},
     ucmd_t{name:
                b"download\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_BeginDownload_f),},
     ucmd_t{name:
                b"nextdl\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_NextDownload_f),},
     ucmd_t{name:
                b"stopdl\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_StopDownload_f),},
     ucmd_t{name:
                b"donedl\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_DoneDownload_f),},
     ucmd_t{name:
                b"voip\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char,
            func: Some(SV_Voip_f),},
     ucmd_t{name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,}];
/*
==================
SV_Voip_f
==================
*/
unsafe extern "C" fn SV_Voip_f(mut cl: *mut client_t) {
    let mut cmd: *const libc::c_char = Cmd_Argv(1i32);
    if strcmp(cmd, b"ignore\x00" as *const u8 as *const libc::c_char) == 0i32
       {
        SV_UpdateVoipIgnore(cl, Cmd_Argv(2i32), qtrue);
    } else if strcmp(cmd, b"unignore\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        SV_UpdateVoipIgnore(cl, Cmd_Argv(2i32), qfalse);
    } else if strcmp(cmd, b"muteall\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        (*cl).muteAllVoip = qtrue
    } else if strcmp(cmd,
                     b"unmuteall\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        (*cl).muteAllVoip = qfalse
    };
}
unsafe extern "C" fn SV_UpdateVoipIgnore(mut cl: *mut client_t,
                                         mut idstr: *const libc::c_char,
                                         mut ignore: qboolean) {
    if *idstr as libc::c_int >= '0' as i32 &&
           *idstr as libc::c_int <= '9' as i32 {
        let id: libc::c_int = atoi(idstr);
        if id >= 0i32 && id < 64i32 {
            (*cl).ignoreVoipFromClient[id as usize] = ignore
        }
    };
}
/*
==================
SV_DoneDownload_f

Downloads are finished
==================
*/
unsafe extern "C" fn SV_DoneDownload_f(mut cl: *mut client_t) {
    if (*cl).state as libc::c_uint == CS_ACTIVE as libc::c_int as libc::c_uint
       {
        return
    }
    Com_DPrintf(b"clientDownload: %s Done\n\x00" as *const u8 as
                    *const libc::c_char, (*cl).name.as_mut_ptr());
    SV_SendClientGameState(cl);
}
/*
==================
SV_StopDownload_f

Abort a download if in progress
==================
*/
unsafe extern "C" fn SV_StopDownload_f(mut cl: *mut client_t) {
    if 0 != *(*cl).downloadName.as_mut_ptr() {
        Com_DPrintf(b"clientDownload: %d : file \"%s\" aborted\n\x00" as
                        *const u8 as *const libc::c_char,
                    cl.wrapping_offset_from(svs.clients) as libc::c_long as
                        libc::c_int, (*cl).downloadName.as_mut_ptr());
    }
    SV_CloseDownload(cl);
}
/*
==================
SV_NextDownload_f

The argument will be the last acknowledged block from the client, it should be
the same as cl->downloadClientBlock
==================
*/
unsafe extern "C" fn SV_NextDownload_f(mut cl: *mut client_t) {
    let mut block: libc::c_int = atoi(Cmd_Argv(1i32));
    if block == (*cl).downloadClientBlock {
        Com_DPrintf(b"clientDownload: %d : client acknowledge of block %d\n\x00"
                        as *const u8 as *const libc::c_char,
                    cl.wrapping_offset_from(svs.clients) as libc::c_long as
                        libc::c_int, block);
        if (*cl).downloadBlockSize[((*cl).downloadClientBlock % 48i32) as
                                       usize] == 0i32 {
            Com_Printf(b"clientDownload: %d : file \"%s\" completed\n\x00" as
                           *const u8 as *const libc::c_char,
                       cl.wrapping_offset_from(svs.clients) as libc::c_long as
                           libc::c_int, (*cl).downloadName.as_mut_ptr());
            SV_CloseDownload(cl);
            return
        }
        (*cl).downloadSendTime = svs.time;
        (*cl).downloadClientBlock += 1;
        return
    }
    SV_DropClient(cl,
                  b"broken download\x00" as *const u8 as *const libc::c_char);
}
/*
==================
SV_BeginDownload_f
==================
*/
unsafe extern "C" fn SV_BeginDownload_f(mut cl: *mut client_t) {
    SV_CloseDownload(cl);
    Q_strncpyz((*cl).downloadName.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
}
/*
=================
SV_ResetPureClient_f
=================
*/
unsafe extern "C" fn SV_ResetPureClient_f(mut cl: *mut client_t) {
    (*cl).pureAuthentic = 0i32;
    (*cl).gotCP = qfalse;
}
/*
=================
SV_VerifyPaks_f

If we are pure, disconnect the client if they do no meet the following conditions:

1. the first two checksums match our view of cgame and ui
2. there are no any additional checksums that we do not have

This routine would be a bit simpler with a goto but i abstained

=================
*/
unsafe extern "C" fn SV_VerifyPaks_f(mut cl: *mut client_t) {
    let mut nChkSum1: libc::c_int = 0;
    let mut nChkSum2: libc::c_int = 0;
    let mut nClientPaks: libc::c_int = 0;
    let mut nServerPaks: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nCurArg: libc::c_int = 0;
    let mut nClientChkSum: [libc::c_int; 1024] = [0; 1024];
    let mut nServerChkSum: [libc::c_int; 1024] = [0; 1024];
    let mut pPaks: *const libc::c_char = 0 as *const libc::c_char;
    let mut pArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut bGood: qboolean = qtrue;
    if (*sv_pure).integer != 0i32 {
        nChkSum2 = 0i32;
        nChkSum1 = nChkSum2;
        bGood =
            (FS_FileIsInPAK(b"vm/cgame.qvm\x00" as *const u8 as
                                *const libc::c_char, &mut nChkSum1) == 1i32)
                as libc::c_int as qboolean;
        if 0 != bGood as u64 {
            bGood =
                (FS_FileIsInPAK(b"vm/ui.qvm\x00" as *const u8 as
                                    *const libc::c_char, &mut nChkSum2) ==
                     1i32) as libc::c_int as qboolean
        }
        nClientPaks = Cmd_Argc();
        nCurArg = 1i32;
        let fresh0 = nCurArg;
        nCurArg = nCurArg + 1;
        pArg = Cmd_Argv(fresh0);
        if pArg.is_null() {
            bGood = qfalse
        } else if atoi(pArg) < sv.checksumFeedServerId {
            Com_DPrintf(b"ignoring outdated cp command from client %s\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*cl).name.as_mut_ptr());
            return
        }
        if 0 != bGood as u64 {
            // must be at least 6: "cl_paks cgame ui @ firstref ... numChecksums"
			// numChecksums is encoded
            if nClientPaks < 6i32 {
                bGood = qfalse
            } else {
                let fresh1 = nCurArg;
                nCurArg = nCurArg + 1;
                pArg = Cmd_Argv(fresh1);
                if pArg.is_null() || *pArg as libc::c_int == '@' as i32 ||
                       atoi(pArg) != nChkSum1 {
                    bGood = qfalse
                } else {
                    let fresh2 = nCurArg;
                    nCurArg = nCurArg + 1;
                    pArg = Cmd_Argv(fresh2);
                    if pArg.is_null() || *pArg as libc::c_int == '@' as i32 ||
                           atoi(pArg) != nChkSum2 {
                        bGood = qfalse
                    } else {
                        let fresh3 = nCurArg;
                        nCurArg = nCurArg + 1;
                        pArg = Cmd_Argv(fresh3);
                        if *pArg as libc::c_int != '@' as i32 {
                            bGood = qfalse
                        } else {
                            i = 0i32;
                            while nCurArg < nClientPaks {
                                let fresh4 = nCurArg;
                                nCurArg = nCurArg + 1;
                                nClientChkSum[i as usize] =
                                    atoi(Cmd_Argv(fresh4));
                                i += 1
                            }
                            nClientPaks = i - 1i32;
                            i = 0i32;
                            while i < nClientPaks {
                                j = 0i32;
                                while j < nClientPaks {
                                    if !(i == j) {
                                        if nClientChkSum[i as usize] ==
                                               nClientChkSum[j as usize] {
                                            bGood = qfalse;
                                            break ;
                                        }
                                    }
                                    j += 1
                                }
                                if bGood as libc::c_uint ==
                                       qfalse as libc::c_int as libc::c_uint {
                                    break ;
                                }
                                i += 1
                            }
                            if !(bGood as libc::c_uint ==
                                     qfalse as libc::c_int as libc::c_uint) {
                                pPaks = FS_LoadedPakPureChecksums();
                                Cmd_TokenizeString(pPaks);
                                nServerPaks = Cmd_Argc();
                                if nServerPaks > 1024i32 {
                                    nServerPaks = 1024i32
                                }
                                i = 0i32;
                                while i < nServerPaks {
                                    nServerChkSum[i as usize] =
                                        atoi(Cmd_Argv(i));
                                    i += 1
                                }
                                i = 0i32;
                                while i < nClientPaks {
                                    j = 0i32;
                                    while j < nServerPaks {
                                        if nClientChkSum[i as usize] ==
                                               nServerChkSum[j as usize] {
                                            break ;
                                        }
                                        j += 1
                                    }
                                    if j >= nServerPaks {
                                        bGood = qfalse;
                                        break ;
                                    } else { i += 1 }
                                }
                                if !(bGood as libc::c_uint ==
                                         qfalse as libc::c_int as
                                             libc::c_uint) {
                                    nChkSum1 = sv.checksumFeed;
                                    i = 0i32;
                                    while i < nClientPaks {
                                        nChkSum1 ^= nClientChkSum[i as usize];
                                        i += 1
                                    }
                                    nChkSum1 ^= nClientPaks;
                                    if nChkSum1 !=
                                           nClientChkSum[nClientPaks as usize]
                                       {
                                        bGood = qfalse
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        (*cl).gotCP = qtrue;
        if 0 != bGood as u64 {
            (*cl).pureAuthentic = 1i32
        } else {
            (*cl).pureAuthentic = 0i32;
            (*cl).lastSnapshotTime = 0i32;
            (*cl).state = CS_ACTIVE;
            SV_SendClientSnapshot(cl);
            SV_DropClient(cl,
                          b"Unpure client detected. Invalid .PK3 files referenced!\x00"
                              as *const u8 as *const libc::c_char);
        }
    };
}
/*
=================
SV_Disconnect_f

The client is going to disconnect, so remove the connection immediately  FIXME: move to game?
=================
*/
unsafe extern "C" fn SV_Disconnect_f(mut cl: *mut client_t) {
    SV_DropClient(cl,
                  b"disconnected\x00" as *const u8 as *const libc::c_char);
}
/*
==================
SV_UpdateUserinfo_f
==================
*/
unsafe extern "C" fn SV_UpdateUserinfo_f(mut cl: *mut client_t) {
    Q_strncpyz((*cl).userinfo.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    SV_UserinfoChanged(cl);
    VM_Call(gvm, GAME_CLIENT_USERINFO_CHANGED as libc::c_int,
            cl.wrapping_offset_from(svs.clients) as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn SV_WriteDownloadToClient(mut cl: *mut client_t,
                                                  mut msg: *mut msg_t)
 -> libc::c_int {
    let mut curindex: libc::c_int = 0;
    let mut unreferenced: libc::c_int = 1i32;
    let mut errorMessage: [libc::c_char; 1024] = [0; 1024];
    let mut pakbuf: [libc::c_char; 64] = [0; 64];
    let mut pakptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numRefPaks: libc::c_int = 0;
    if 0 == *(*cl).downloadName.as_mut_ptr() { return 0i32 }
    if 0 == (*cl).download {
        let mut idPack: qboolean = qfalse;
        let mut missionPack: qboolean = qfalse;
        Com_sprintf(pakbuf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as
                        libc::c_ulong as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    (*cl).downloadName.as_mut_ptr());
        pakptr = strrchr(pakbuf.as_mut_ptr(), '.' as i32);
        if !pakptr.is_null() {
            *pakptr = '\u{0}' as i32 as libc::c_char;
            if 0 ==
                   Q_stricmp(pakptr.offset(1isize),
                             b"pk3\x00" as *const u8 as *const libc::c_char) {
                let mut referencedPaks: *const libc::c_char =
                    FS_ReferencedPakNames();
                Cmd_TokenizeStringIgnoreQuotes(referencedPaks);
                numRefPaks = Cmd_Argc();
                curindex = 0i32;
                while curindex < numRefPaks {
                    if 0 ==
                           FS_FilenameCompare(Cmd_Argv(curindex),
                                              pakbuf.as_mut_ptr()) as u64 {
                        unreferenced = 0i32;
                        missionPack =
                            FS_idPak(pakbuf.as_mut_ptr(),
                                     b"missionpack\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char, 4i32);
                        idPack = missionPack;
                        idPack =
                            (0 != idPack as libc::c_uint ||
                                 0 !=
                                     FS_idPak(pakbuf.as_mut_ptr(),
                                              b"baseq3\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char, 9i32) as
                                         libc::c_uint) as libc::c_int as
                                qboolean;
                        break ;
                    } else { curindex += 1 }
                }
            }
        }
        (*cl).download = 0i32;
        if 0 == (*sv_allowDownload).integer & 1i32 ||
               0 != (*sv_allowDownload).integer & 4i32 ||
               0 != idPack as libc::c_uint || 0 != unreferenced ||
               {
                   (*cl).downloadSize =
                       FS_SV_FOpenFileRead((*cl).downloadName.as_mut_ptr(),
                                           &mut (*cl).download) as
                           libc::c_int;
                   (*cl).downloadSize < 0i32
               } {
            if 0 != unreferenced {
                Com_Printf(b"clientDownload: %d : \"%s\" is not referenced and cannot be downloaded.\n\x00"
                               as *const u8 as *const libc::c_char,
                           cl.wrapping_offset_from(svs.clients) as
                               libc::c_long as libc::c_int,
                           (*cl).downloadName.as_mut_ptr());
                Com_sprintf(errorMessage.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"File \"%s\" is not referenced and cannot be downloaded.\x00"
                                as *const u8 as *const libc::c_char,
                            (*cl).downloadName.as_mut_ptr());
            } else if 0 != idPack as u64 {
                Com_Printf(b"clientDownload: %d : \"%s\" cannot download id pk3 files\n\x00"
                               as *const u8 as *const libc::c_char,
                           cl.wrapping_offset_from(svs.clients) as
                               libc::c_long as libc::c_int,
                           (*cl).downloadName.as_mut_ptr());
                if 0 != missionPack as u64 {
                    Com_sprintf(errorMessage.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as libc::c_int,
                                b"Cannot autodownload Team Arena file \"%s\"\nThe Team Arena mission pack can be found in your local game store.\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cl).downloadName.as_mut_ptr());
                } else {
                    Com_sprintf(errorMessage.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as libc::c_int,
                                b"Cannot autodownload id pk3 file \"%s\"\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cl).downloadName.as_mut_ptr());
                }
            } else if 0 == (*sv_allowDownload).integer & 1i32 ||
                          0 != (*sv_allowDownload).integer & 4i32 {
                Com_Printf(b"clientDownload: %d : \"%s\" download disabled\n\x00"
                               as *const u8 as *const libc::c_char,
                           cl.wrapping_offset_from(svs.clients) as
                               libc::c_long as libc::c_int,
                           (*cl).downloadName.as_mut_ptr());
                if 0 != (*sv_pure).integer {
                    Com_sprintf(errorMessage.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as libc::c_int,
                                b"Could not download \"%s\" because autodownloading is disabled on the server.\n\nYou will need to get this file elsewhere before you can connect to this pure server.\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cl).downloadName.as_mut_ptr());
                } else {
                    Com_sprintf(errorMessage.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as libc::c_int,
                                b"Could not download \"%s\" because autodownloading is disabled on the server.\n\nThe server you are connecting to is not a pure server, set autodownload to No in your settings and you might be able to join the game anyway.\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cl).downloadName.as_mut_ptr());
                }
            } else {
                Com_Printf(b"clientDownload: %d : \"%s\" file not found on server\n\x00"
                               as *const u8 as *const libc::c_char,
                           cl.wrapping_offset_from(svs.clients) as
                               libc::c_long as libc::c_int,
                           (*cl).downloadName.as_mut_ptr());
                Com_sprintf(errorMessage.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"File \"%s\" not found on server for autodownloading.\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*cl).downloadName.as_mut_ptr());
            }
            MSG_WriteByte(msg, svc_download as libc::c_int);
            MSG_WriteShort(msg, 0i32);
            MSG_WriteLong(msg, -1i32);
            MSG_WriteString(msg, errorMessage.as_mut_ptr());
            *(*cl).downloadName.as_mut_ptr() = 0i32 as libc::c_char;
            if 0 != (*cl).download { FS_FCloseFile((*cl).download); }
            return 1i32
        }
        Com_Printf(b"clientDownload: %d : beginning \"%s\"\n\x00" as *const u8
                       as *const libc::c_char,
                   cl.wrapping_offset_from(svs.clients) as libc::c_long as
                       libc::c_int, (*cl).downloadName.as_mut_ptr());
        (*cl).downloadXmitBlock = 0i32;
        (*cl).downloadClientBlock = (*cl).downloadXmitBlock;
        (*cl).downloadCurrentBlock = (*cl).downloadClientBlock;
        (*cl).downloadCount = 0i32;
        (*cl).downloadEOF = qfalse
    }
    while (*cl).downloadCurrentBlock - (*cl).downloadClientBlock < 48i32 &&
              (*cl).downloadSize != (*cl).downloadCount {
        curindex = (*cl).downloadCurrentBlock % 48i32;
        if (*cl).downloadBlocks[curindex as usize].is_null() {
            (*cl).downloadBlocks[curindex as usize] =
                Z_MallocDebug(1024i32,
                              b"MAX_DOWNLOAD_BLKSIZE\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              b"code/server/sv_client.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              1061i32) as *mut libc::c_uchar
        }
        (*cl).downloadBlockSize[curindex as usize] =
            FS_Read((*cl).downloadBlocks[curindex as usize] as
                        *mut libc::c_void, 1024i32, (*cl).download);
        if (*cl).downloadBlockSize[curindex as usize] < 0i32 {
            (*cl).downloadCount = (*cl).downloadSize;
            break ;
        } else {
            (*cl).downloadCount += (*cl).downloadBlockSize[curindex as usize];
            (*cl).downloadCurrentBlock += 1
        }
    }
    if (*cl).downloadCount == (*cl).downloadSize &&
           0 == (*cl).downloadEOF as u64 &&
           (*cl).downloadCurrentBlock - (*cl).downloadClientBlock < 48i32 {
        (*cl).downloadBlockSize[((*cl).downloadCurrentBlock % 48i32) as usize]
            = 0i32;
        (*cl).downloadCurrentBlock += 1;
        (*cl).downloadEOF = qtrue
    }
    if (*cl).downloadClientBlock == (*cl).downloadCurrentBlock { return 0i32 }
    if (*cl).downloadXmitBlock == (*cl).downloadCurrentBlock {
        if svs.time - (*cl).downloadSendTime > 1000i32 {
            (*cl).downloadXmitBlock = (*cl).downloadClientBlock
        } else { return 0i32 }
    }
    curindex = (*cl).downloadXmitBlock % 48i32;
    MSG_WriteByte(msg, svc_download as libc::c_int);
    MSG_WriteShort(msg, (*cl).downloadXmitBlock);
    if (*cl).downloadXmitBlock == 0i32 {
        MSG_WriteLong(msg, (*cl).downloadSize);
    }
    MSG_WriteShort(msg, (*cl).downloadBlockSize[curindex as usize]);
    if 0 != (*cl).downloadBlockSize[curindex as usize] {
        MSG_WriteData(msg,
                      (*cl).downloadBlocks[curindex as usize] as
                          *const libc::c_void,
                      (*cl).downloadBlockSize[curindex as usize]);
    }
    Com_DPrintf(b"clientDownload: %d : writing block %d\n\x00" as *const u8 as
                    *const libc::c_char,
                cl.wrapping_offset_from(svs.clients) as libc::c_long as
                    libc::c_int, (*cl).downloadXmitBlock);
    (*cl).downloadXmitBlock += 1;
    (*cl).downloadSendTime = svs.time;
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendDownloadMessages() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut numDLs: libc::c_int = 0i32;
    let mut retval: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut msg: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut msgBuffer: [byte; 16384] = [0; 16384];
    i = 0i32;
    while i < (*sv_maxclients).integer {
        cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if 0 != (*cl).state as libc::c_uint &&
               0 != *(*cl).downloadName.as_mut_ptr() as libc::c_int {
            MSG_Init(&mut msg, msgBuffer.as_mut_ptr(),
                     ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong
                         as libc::c_int);
            MSG_WriteLong(&mut msg, (*cl).lastClientCommand);
            retval = SV_WriteDownloadToClient(cl, &mut msg);
            if 0 != retval {
                MSG_WriteByte(&mut msg, svc_EOF as libc::c_int);
                SV_Netchan_Transmit(cl, &mut msg);
                numDLs += retval
            }
        }
        i += 1
    }
    return numDLs;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendQueuedMessages() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_int = -1i32;
    let mut nextFragT: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        cl = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if 0 != (*cl).state as u64 {
            nextFragT = SV_RateMsec(cl);
            if 0 == nextFragT {
                nextFragT = SV_Netchan_TransmitNextFragment(cl)
            }
            if nextFragT >= 0i32 && (retval == -1i32 || retval > nextFragT) {
                retval = nextFragT
            }
        }
        i += 1
    }
    return retval;
}