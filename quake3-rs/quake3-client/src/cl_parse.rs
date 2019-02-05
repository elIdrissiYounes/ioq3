use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int16_t = libc::c_short;
    pub type __uint16_t = libc::c_ushort;
    pub type __int32_t = libc::c_int;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    use super::types_h::{__uint8_t, __uint16_t};
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
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
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
        //=============================================
        //
// key / value info strings
//
        #[no_mangle]
        pub fn Info_ValueForKey(s: *const libc::c_char,
                                key: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Info_NextPair(s: *mut *const libc::c_char,
                             key: *mut libc::c_char,
                             value: *mut libc::c_char);
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
    use super::q_shared_h::{qboolean, byte, entityState_t, playerState_s,
                            cvar_t, fileHandle_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn MSG_Bitstream(buf: *mut msg_t);
        #[no_mangle]
        pub fn MSG_ReadBits(msg: *mut msg_t, bits: libc::c_int)
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
        pub fn MSG_ReadBigString(sb: *mut msg_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn MSG_ReadData(sb: *mut msg_t, buffer: *mut libc::c_void,
                            size: libc::c_int);
        #[no_mangle]
        pub fn MSG_ReadDeltaEntity(msg: *mut msg_t, from: *mut entityState_t,
                                   to: *mut entityState_t,
                                   number: libc::c_int);
        #[no_mangle]
        pub fn MSG_ReadDeltaPlayerstate(msg: *mut msg_t,
                                        from: *mut playerState_s,
                                        to: *mut playerState_s);
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
        // same as Cvar_Set, but allows more control over setting of cvar
        #[no_mangle]
        pub fn Cvar_SetSafe(var_name: *const libc::c_char,
                            value: *const libc::c_char);
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_VariableStringBuffer(var_name: *const libc::c_char,
                                         buffer: *mut libc::c_char,
                                         bufsize: libc::c_int);
        // returns an empty string if not defined
        #[no_mangle]
        pub fn Cvar_Flags(var_name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn Cvar_SetCheatState();
        #[no_mangle]
        pub fn FS_ConditionalRestart(checksumFeed: libc::c_int,
                                     disconnect: qboolean) -> qboolean;
        // will properly create any needed paths and deal with seperater character issues
        #[no_mangle]
        pub fn FS_SV_FOpenFileWrite(filename: *const libc::c_char)
         -> fileHandle_t;
        #[no_mangle]
        pub fn FS_SV_Rename(from: *const libc::c_char,
                            to: *const libc::c_char, safe: qboolean);
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        // clears referenced booleans on loaded pk3s
        #[no_mangle]
        pub fn FS_PureServerSetReferencedPaks(pakSums: *const libc::c_char,
                                              pakNames: *const libc::c_char);
        #[no_mangle]
        pub fn FS_PureServerSetLoadedPaks(pakSums: *const libc::c_char,
                                          pakNames: *const libc::c_char);
        #[no_mangle]
        pub fn FS_InvalidGameDir(gamedir: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // both client and server must agree to pause
        #[no_mangle]
        pub static mut cl_paused: *mut cvar_t;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
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
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/x86_64-linux-gnu/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/opus/opus_types.h"]
pub mod opus_types_h {
    /* (C) COPYRIGHT 1994-2002 Xiph.Org Foundation */
/* Modified by Jean-Marc Valin */
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
/* opus_types.h based on ogg_types.h from libogg */
    /* *
   @file opus_types.h
   @brief Opus reference implementation types
*/
    /* Use the real stdint.h if it's there (taken from Paul Hsieh's pstdint.h) */
    pub type opus_int16 = int16_t;
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t};
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    use super::{libc};
    use super::opus_types_h::{opus_int32, opus_int16};
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
  * is 1 byte, then the packet does not need to be transmitted (DTX).
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
        /* * Decode an Opus packet.
  * @param [in] st <tt>OpusDecoder*</tt>: Decoder state
  * @param [in] data <tt>char*</tt>: Input payload. Use a NULL pointer to indicate packet loss
  * @param [in] len <tt>opus_int32</tt>: Number of bytes in payload*
  * @param [out] pcm <tt>opus_int16*</tt>: Output signal (interleaved if 2 channels). length
  *  is frame_size*channels*sizeof(opus_int16)
  * @param [in] frame_size Number of samples per channel of available space in \a pcm.
  *  If this is less than the maximum packet duration (120ms; 5760 for 48kHz), this function will
  *  not be capable of decoding some packets. In the case of PLC (data==NULL) or FEC (decode_fec=1),
  *  then frame_size needs to be exactly the duration of audio that is missing, otherwise the
  *  decoder will not be in the optimal state to decode the next incoming packet. For the PLC and
  *  FEC cases, frame_size <b>must</b> be a multiple of 2.5 ms.
  * @param [in] decode_fec <tt>int</tt>: Flag (0 or 1) to request that any in-band forward error correction data be
  *  decoded. If no such data is available, the frame is decoded as if it were lost.
  * @returns Number of decoded samples or @ref opus_errorcodes
  */
        #[no_mangle]
        pub fn opus_decode(st: *mut OpusDecoder, data: *const libc::c_uchar,
                           len: opus_int32, pcm: *mut opus_int16,
                           frame_size: libc::c_int, decode_fec: libc::c_int)
         -> libc::c_int;
        /* * Perform a CTL function on an Opus decoder.
  *
  * Generally the request and subsequent arguments are generated
  * by a convenience macro.
  * @param st <tt>OpusDecoder*</tt>: Decoder state.
  * @param request This and all remaining parameters should be replaced by one
  *                of the convenience macros in @ref opus_genericctls or
  *                @ref opus_decoderctls.
  * @see opus_genericctls
  * @see opus_decoderctls
  */
        #[no_mangle]
        pub fn opus_decoder_ctl(st: *mut OpusDecoder,
                                request: libc::c_int, ...) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/client.h"]
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
                            fileHandle_t, qhandle_t, cvar_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, msg_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    extern "C" {
        #[no_mangle]
        pub static mut cl: clientActive_t;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        #[no_mangle]
        pub static mut cl_oldGame: [libc::c_char; 64];
        #[no_mangle]
        pub static mut cl_oldGameSet: qboolean;
        #[no_mangle]
        pub static mut cl_shownet: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_autoRecordDemo: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_voip: *mut cvar_t;
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
        pub fn CL_StopRecord_f();
        #[no_mangle]
        pub fn CL_InitDownloads();
        #[no_mangle]
        pub fn CL_NextDownload();
        #[no_mangle]
        pub fn CL_ClearState();
        #[no_mangle]
        pub fn CL_WritePacket();
        #[no_mangle]
        pub fn Con_Close();
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
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    use super::{libc};
    use super::q_shared_h::{byte};
    extern "C" {
        // cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
        #[no_mangle]
        pub fn S_RawSamples(stream: libc::c_int, samples: libc::c_int,
                            rate: libc::c_int, width: libc::c_int,
                            channels: libc::c_int, data: *const byte,
                            volume: libc::c_float, entityNum: libc::c_int);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/cl_parse.c"]
pub mod cl_parse_c {
    use super::qcommon_h::{msg_t};
    use super::client_h::{clSnapshot_t};
    use super::{libc};
    use super::q_shared_h::{entityState_t, qboolean};
}
use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t};
use self::stdint_intn_h::{int16_t, int32_t};
use self::stdint_uintn_h::{uint8_t, uint16_t};
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
                       Q_stricmp, Q_strncpyz, va, Info_ValueForKey,
                       Info_NextPair, Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      svc_ops_e, svc_voipOpus, svc_voipSpeex, svc_EOF,
                      svc_snapshot, svc_download, svc_serverCommand,
                      svc_baseline, svc_configstring, svc_gamestate, svc_nop,
                      svc_bad, MSG_Bitstream, MSG_ReadBits, MSG_ReadByte,
                      MSG_ReadShort, MSG_ReadLong, MSG_ReadString,
                      MSG_ReadBigString, MSG_ReadData, MSG_ReadDeltaEntity,
                      MSG_ReadDeltaPlayerstate, Cvar_Get, Cvar_Set,
                      Cvar_SetSafe, Cvar_SetValue, Cvar_VariableValue,
                      Cvar_VariableString, Cvar_VariableStringBuffer,
                      Cvar_Flags, Cvar_SetCheatState, FS_ConditionalRestart,
                      FS_SV_FOpenFileWrite, FS_SV_Rename, FS_Write,
                      FS_FCloseFile, FS_PureServerSetReferencedPaks,
                      FS_PureServerSetLoadedPaks, FS_InvalidGameDir,
                      Com_DPrintf, cl_paused};
use self::tr_types_h::{textureCompression_t, TC_S3TC_ARB, TC_S3TC, TC_NONE,
                       glDriverType_t, GLDRV_VOODOO, GLDRV_STANDALONE,
                       GLDRV_ICD, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glconfig_t};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_types_h::{opus_int16, opus_int32};
use self::opus_h::{OpusEncoder, OpusDecoder, opus_decode, opus_decoder_ctl};
use self::client_h::{clSnapshot_t, outPacket_t, clientActive_t,
                     clientConnection_t, serverInfo_t, clientStatic_t, cl,
                     clc, cls, cl_oldGame, cl_oldGameSet, cl_shownet,
                     cl_autoRecordDemo, cl_voip, CL_AddReliableCommand,
                     CL_StopRecord_f, CL_InitDownloads, CL_NextDownload,
                     CL_ClearState, CL_WritePacket, Con_Close};
use self::assert_h::{__assert_fail};
use self::string_h::{memcpy, memset, strlen};
use self::stdlib_h::{atoi};
use self::snd_public_h::{S_RawSamples};
//
// cl_parse.c
//
#[no_mangle]
pub static mut cl_connectedToPureServer: libc::c_int = 0;
#[no_mangle]
pub static mut cl_connectedToCheatServer: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn CL_SystemInfoChanged() {
    let mut systemInfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [libc::c_char; 8192] = [0; 8192];
    let mut value: [libc::c_char; 8192] = [0; 8192];
    let mut gameSet: qboolean = qfalse;
    systemInfo =
        cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[1usize]
                                                        as isize);
    cl.serverId =
        atoi(Info_ValueForKey(systemInfo,
                              b"sv_serverid\x00" as *const u8 as
                                  *const libc::c_char));
    if 0 != clc.compat as u64 {
        clc.voipEnabled = qfalse
    } else {
        s =
            Info_ValueForKey(systemInfo,
                             b"sv_voipProtocol\x00" as *const u8 as
                                 *const libc::c_char);
        clc.voipEnabled =
            (0 ==
                 Q_stricmp(s,
                           b"opus\x00" as *const u8 as *const libc::c_char))
                as libc::c_int as qboolean
    }
    if 0 != clc.demoplaying as u64 { return }
    s =
        Info_ValueForKey(systemInfo,
                         b"sv_cheats\x00" as *const u8 as
                             *const libc::c_char);
    cl_connectedToCheatServer = atoi(s);
    if 0 == cl_connectedToCheatServer { Cvar_SetCheatState(); }
    s =
        Info_ValueForKey(systemInfo,
                         b"sv_paks\x00" as *const u8 as *const libc::c_char);
    t =
        Info_ValueForKey(systemInfo,
                         b"sv_pakNames\x00" as *const u8 as
                             *const libc::c_char);
    FS_PureServerSetLoadedPaks(s, t);
    s =
        Info_ValueForKey(systemInfo,
                         b"sv_referencedPaks\x00" as *const u8 as
                             *const libc::c_char);
    t =
        Info_ValueForKey(systemInfo,
                         b"sv_referencedPakNames\x00" as *const u8 as
                             *const libc::c_char);
    FS_PureServerSetReferencedPaks(s, t);
    gameSet = qfalse;
    s = systemInfo;
    while !s.is_null() {
        let mut cvar_flags: libc::c_int = 0;
        Info_NextPair(&mut s, key.as_mut_ptr(), value.as_mut_ptr());
        if 0 == key[0usize] { break ; }
        // ehw!
        if 0 ==
               Q_stricmp(key.as_mut_ptr(),
                         b"fs_game\x00" as *const u8 as *const libc::c_char) {
            if 0 != FS_InvalidGameDir(value.as_mut_ptr()) as u64 {
                Com_Printf(b"^3WARNING: Server sent invalid fs_game value %s\n\x00"
                               as *const u8 as *const libc::c_char,
                           value.as_mut_ptr());
                continue ;
            } else { gameSet = qtrue }
        }
        cvar_flags = Cvar_Flags(key.as_mut_ptr());
        if cvar_flags as libc::c_uint == 0x80000000u32 {
            Cvar_Get(key.as_mut_ptr(), value.as_mut_ptr(),
                     0x800i32 | 0x40i32);
        } else {
            // If this cvar may not be modified by a server discard the value.
            if 0 == cvar_flags & (0x8i32 | 0x800i32 | 0x80i32) {
                if 0 !=
                       Q_stricmp(key.as_mut_ptr(),
                                 b"g_synchronousClients\x00" as *const u8 as
                                     *const libc::c_char) &&
                       0 !=
                           Q_stricmp(key.as_mut_ptr(),
                                     b"pmove_fixed\x00" as *const u8 as
                                         *const libc::c_char) &&
                       0 !=
                           Q_stricmp(key.as_mut_ptr(),
                                     b"pmove_msec\x00" as *const u8 as
                                         *const libc::c_char) {
                    Com_Printf(b"^3WARNING: server is not allowed to set %s=%s\n\x00"
                                   as *const u8 as *const libc::c_char,
                               key.as_mut_ptr(), value.as_mut_ptr());
                    continue ;
                }
            }
            Cvar_SetSafe(key.as_mut_ptr(), value.as_mut_ptr());
        }
    }
    if 0 == gameSet as u64 &&
           0 !=
               *Cvar_VariableString(b"fs_game\x00" as *const u8 as
                                        *const libc::c_char) as libc::c_int {
        Cvar_Set(b"fs_game\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
    }
    cl_connectedToPureServer =
        Cvar_VariableValue(b"sv_pure\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_ParseServerMessage(mut msg: *mut msg_t) {
    let mut cmd: libc::c_int = 0;
    if (*cl_shownet).integer == 1i32 {
        Com_Printf(b"%i \x00" as *const u8 as *const libc::c_char,
                   (*msg).cursize);
    } else if (*cl_shownet).integer >= 2i32 {
        Com_Printf(b"------------------\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    MSG_Bitstream(msg);
    clc.reliableAcknowledge = MSG_ReadLong(msg);
    if clc.reliableAcknowledge < clc.reliableSequence - 64i32 {
        clc.reliableAcknowledge = clc.reliableSequence
    }
    loop  {
        if (*msg).readcount > (*msg).cursize {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CL_ParseServerMessage: read past end of server message\x00"
                          as *const u8 as *const libc::c_char);
        } else {
            cmd = MSG_ReadByte(msg);
            if cmd == svc_EOF as libc::c_int {
                SHOWNET(msg,
                        b"END OF MESSAGE\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
                break ;
            } else {
                if (*cl_shownet).integer >= 2i32 {
                    if cmd < 0i32 || svc_strings[cmd as usize].is_null() {
                        Com_Printf(b"%3i:BAD CMD %i\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*msg).readcount - 1i32, cmd);
                    } else { SHOWNET(msg, svc_strings[cmd as usize]); }
                }
                match cmd {
                    1 => { }
                    5 => { CL_ParseCommandString(msg); }
                    2 => { CL_ParseGamestate(msg); }
                    7 => { CL_ParseSnapshot(msg); }
                    6 => { CL_ParseDownload(msg); }
                    9 => { CL_ParseVoip(msg, qtrue); }
                    10 => {
                        CL_ParseVoip(msg,
                                     (0 == clc.voipEnabled as u64) as
                                         libc::c_int as qboolean);
                    }
                    _ => {
                        Com_Error(ERR_DROP as libc::c_int,
                                  b"CL_ParseServerMessage: Illegible server message\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                }
            }
        }
    };
}
/*
=====================
CL_ParseVoip

A VoIP message has been received from the server
=====================
*/
unsafe extern "C" fn CL_ParseVoip(mut msg: *mut msg_t,
                                  mut ignoreData: qboolean) {
    // !!! FIXME: don't hard code
    static mut decoded: [libc::c_short; 11520] = [0; 11520];
    let sender: libc::c_int = MSG_ReadShort(msg);
    let generation: libc::c_int = MSG_ReadByte(msg);
    let sequence: libc::c_int = MSG_ReadLong(msg);
    let frames: libc::c_int = MSG_ReadByte(msg);
    let packetsize: libc::c_int = MSG_ReadShort(msg);
    let flags: libc::c_int = MSG_ReadBits(msg, 2i32);
    let mut encoded: [libc::c_uchar; 4000] = [0; 4000];
    let mut numSamples: libc::c_int = 0;
    let mut seqdiff: libc::c_int = 0;
    let mut written: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    Com_DPrintf(b"VoIP: %d-byte packet from client %d\n\x00" as *const u8 as
                    *const libc::c_char, packetsize, sender);
    if sender < 0i32 {
        return
    } else {
        if generation < 0i32 {
            return
        } else {
            if sequence < 0i32 {
                return
            } else {
                if frames < 0i32 {
                    return
                } else { if packetsize < 0i32 { return } }
            }
        }
    }
    if packetsize as libc::c_ulong >
           ::std::mem::size_of::<[libc::c_uchar; 4000]>() as libc::c_ulong {
        let mut bytesleft: libc::c_int = packetsize;
        while 0 != bytesleft {
            let mut br: libc::c_int = bytesleft;
            if br as libc::c_ulong >
                   ::std::mem::size_of::<[libc::c_uchar; 4000]>() as
                       libc::c_ulong {
                br =
                    ::std::mem::size_of::<[libc::c_uchar; 4000]>() as
                        libc::c_ulong as libc::c_int
            }
            MSG_ReadData(msg, encoded.as_mut_ptr() as *mut libc::c_void, br);
            bytesleft -= br
        }
        return
    }
    MSG_ReadData(msg, encoded.as_mut_ptr() as *mut libc::c_void, packetsize);
    if 0 != ignoreData as u64 {
        return
    } else {
        if 0 == clc.voipCodecInitialized as u64 {
            return
        } else {
            if sender >= 64i32 {
                return
            } else {
                if 0 != CL_ShouldIgnoreVoipSender(sender) as u64 { return }
            }
        }
    }
    Com_DPrintf(b"VoIP: packet accepted!\n\x00" as *const u8 as
                    *const libc::c_char);
    seqdiff = sequence - clc.voipIncomingSequence[sender as usize];
    if generation !=
           clc.voipIncomingGeneration[sender as usize] as libc::c_int {
        Com_DPrintf(b"VoIP: new generation %d!\n\x00" as *const u8 as
                        *const libc::c_char, generation);
        opus_decoder_ctl(clc.opusDecoder[sender as usize], 4028i32);
        clc.voipIncomingGeneration[sender as usize] = generation as byte;
        seqdiff = 0i32
    } else if seqdiff < 0i32 {
        Com_DPrintf(b"VoIP: misordered sequence! %d < %d!\n\x00" as *const u8
                        as *const libc::c_char, sequence,
                    clc.voipIncomingSequence[sender as usize]);
        opus_decoder_ctl(clc.opusDecoder[sender as usize], 4028i32);
        seqdiff = 0i32
    } else if (seqdiff * (20i32 * 48i32 * 3i32) * 2i32) as libc::c_ulong >=
                  ::std::mem::size_of::<[libc::c_short; 11520]>() as
                      libc::c_ulong {
        Com_DPrintf(b"VoIP: Dropped way too many (%d) frames from client #%d\n\x00"
                        as *const u8 as *const libc::c_char, seqdiff, sender);
        opus_decoder_ctl(clc.opusDecoder[sender as usize], 4028i32);
        seqdiff = 0i32
    }
    if seqdiff != 0i32 {
        Com_DPrintf(b"VoIP: Dropped %d frames from client #%d\n\x00" as
                        *const u8 as *const libc::c_char, seqdiff, sender);
        i = 0i32;
        while i < seqdiff {
            if (((written + 20i32 * 48i32 * 3i32) * 2i32) as libc::c_ulong) <
                   ::std::mem::size_of::<[libc::c_short; 11520]>() as
                       libc::c_ulong {
            } else {
                __assert_fail(b"(written + VOIP_MAX_PACKET_SAMPLES) * 2 < sizeof (decoded)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"code/client/cl_parse.c\x00" as *const u8 as
                                  *const libc::c_char, 787i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 37],
                                                        &[libc::c_char; 37]>(b"void CL_ParseVoip(msg_t *, qboolean)\x00")).as_ptr());
            }
            numSamples =
                opus_decode(clc.opusDecoder[sender as usize],
                            0 as *const libc::c_uchar, 0i32,
                            decoded.as_mut_ptr().offset(written as isize),
                            20i32 * 48i32 * 3i32, 0i32);
            if numSamples <= 0i32 {
                Com_DPrintf(b"VoIP: Error decoding frame %d from client #%d\n\x00"
                                as *const u8 as *const libc::c_char, i,
                            sender);
            } else { written += numSamples }
            i += 1
        }
    }
    numSamples =
        opus_decode(clc.opusDecoder[sender as usize], encoded.as_mut_ptr(),
                    packetsize, decoded.as_mut_ptr().offset(written as isize),
                    (::std::mem::size_of::<[libc::c_short; 11520]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_short>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(written
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int, 0i32);
    if numSamples <= 0i32 {
        Com_DPrintf(b"VoIP: Error decoding voip data from client #%d\n\x00" as
                        *const u8 as *const libc::c_char, sender);
        numSamples = 0i32
    }
    written += numSamples;
    Com_DPrintf(b"VoIP: playback %d bytes, %d samples, %d frames\n\x00" as
                    *const u8 as *const libc::c_char, written * 2i32, written,
                frames);
    if written > 0i32 {
        CL_PlayVoip(sender, written, decoded.as_mut_ptr() as *const byte,
                    flags);
    }
    clc.voipIncomingSequence[sender as usize] = sequence + frames;
}
/*
=====================
CL_PlayVoip

Play raw data
=====================
*/
unsafe extern "C" fn CL_PlayVoip(mut sender: libc::c_int,
                                 mut samplecnt: libc::c_int,
                                 mut data: *const byte,
                                 mut flags: libc::c_int) {
    if 0 != flags & 0x2i32 {
        S_RawSamples(sender + 1i32, samplecnt, 48000i32, 2i32, 1i32, data,
                     clc.voipGain[sender as usize], -1i32);
    }
    if 0 != flags & 0x1i32 {
        S_RawSamples(sender + 64i32 + 1i32, samplecnt, 48000i32, 2i32, 1i32,
                     data, 1.0f32, sender);
    };
}
unsafe extern "C" fn CL_ShouldIgnoreVoipSender(mut sender: libc::c_int)
 -> qboolean {
    if 0 == (*cl_voip).integer {
        return qtrue
    } else {
        if sender == clc.clientNum && 0 == clc.demoplaying as u64 {
            return qtrue
        } else {
            if 0 != clc.voipMuteAll as u64 {
                return qtrue
            } else {
                if 0 != clc.voipIgnore[sender as usize] as u64 {
                    return qtrue
                } else {
                    if clc.voipGain[sender as usize] == 0.0f32 {
                        return qtrue
                    }
                }
            }
        }
    }
    return qfalse;
}
//=====================================================================
/*
=====================
CL_ParseDownload

A download message has been received from the server
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseDownload(mut msg: *mut msg_t) {
    let mut size: libc::c_int = 0;
    let mut data: [libc::c_uchar; 16384] = [0; 16384];
    let mut block: uint16_t = 0;
    if 0 == *clc.downloadTempName.as_mut_ptr() {
        Com_Printf(b"Server sending download, but no download was requested\n\x00"
                       as *const u8 as *const libc::c_char);
        CL_AddReliableCommand(b"stopdl\x00" as *const u8 as
                                  *const libc::c_char, qfalse);
        return
    }
    block = MSG_ReadShort(msg) as uint16_t;
    if 0 == block && 0 == clc.downloadBlock {
        clc.downloadSize = MSG_ReadLong(msg);
        Cvar_SetValue(b"cl_downloadSize\x00" as *const u8 as
                          *const libc::c_char,
                      clc.downloadSize as libc::c_float);
        if clc.downloadSize < 0i32 {
            Com_Error(ERR_DROP as libc::c_int,
                      b"%s\x00" as *const u8 as *const libc::c_char,
                      MSG_ReadString(msg));
        }
    }
    size = MSG_ReadShort(msg);
    if size < 0i32 ||
           size as libc::c_ulong >
               ::std::mem::size_of::<[libc::c_uchar; 16384]>() as
                   libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_ParseDownload: Invalid size %d for download chunk\x00"
                      as *const u8 as *const libc::c_char, size);
    }
    MSG_ReadData(msg, data.as_mut_ptr() as *mut libc::c_void, size);
    if clc.downloadBlock & 0xffffi32 != block as libc::c_int {
        Com_DPrintf(b"CL_ParseDownload: Expected block %d, got %d\n\x00" as
                        *const u8 as *const libc::c_char,
                    clc.downloadBlock & 0xffffi32, block as libc::c_int);
        return
    }
    if 0 == clc.download {
        clc.download =
            FS_SV_FOpenFileWrite(clc.downloadTempName.as_mut_ptr());
        if 0 == clc.download {
            Com_Printf(b"Could not create %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       clc.downloadTempName.as_mut_ptr());
            CL_AddReliableCommand(b"stopdl\x00" as *const u8 as
                                      *const libc::c_char, qfalse);
            CL_NextDownload();
            return
        }
    }
    if 0 != size {
        FS_Write(data.as_mut_ptr() as *const libc::c_void, size,
                 clc.download);
    }
    CL_AddReliableCommand(va(b"nextdl %d\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             clc.downloadBlock), qfalse);
    clc.downloadBlock += 1;
    clc.downloadCount += size;
    Cvar_SetValue(b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
                  clc.downloadCount as libc::c_float);
    if 0 == size {
        if 0 != clc.download {
            FS_FCloseFile(clc.download);
            clc.download = 0i32;
            FS_SV_Rename(clc.downloadTempName.as_mut_ptr(),
                         clc.downloadName.as_mut_ptr(), qfalse);
        }
        CL_WritePacket();
        CL_WritePacket();
        CL_NextDownload();
    };
}
/*
================
CL_ParseSnapshot

If the snapshot is parsed properly, it will be copied to
cl.snap and saved in cl.snapshots[].  If the snapshot is invalid
for any reason, no changes to the state will be made at all.
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseSnapshot(mut msg: *mut msg_t) {
    let mut len: libc::c_int = 0;
    let mut old: *mut clSnapshot_t = 0 as *mut clSnapshot_t;
    let mut newSnap: clSnapshot_t =
        clSnapshot_t{valid: qfalse,
                     snapFlags: 0,
                     serverTime: 0,
                     messageNum: 0,
                     deltaNum: 0,
                     ping: 0,
                     areamask: [0; 32],
                     cmdNum: 0,
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
                                       grapplePoint: [0.; 3],
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
                                       entityEventSequence: 0,},
                     numEntities: 0,
                     parseEntitiesNum: 0,
                     serverCommandNum: 0,};
    let mut deltaNum: libc::c_int = 0;
    let mut oldMessageNum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut packetNum: libc::c_int = 0;
    memset(&mut newSnap as *mut clSnapshot_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<clSnapshot_t>() as libc::c_ulong);
    newSnap.serverCommandNum = clc.serverCommandSequence;
    newSnap.serverTime = MSG_ReadLong(msg);
    (*cl_paused).modified = qfalse;
    newSnap.messageNum = clc.serverMessageSequence;
    deltaNum = MSG_ReadByte(msg);
    if 0 == deltaNum {
        newSnap.deltaNum = -1i32
    } else { newSnap.deltaNum = newSnap.messageNum - deltaNum }
    newSnap.snapFlags = MSG_ReadByte(msg);
    if newSnap.deltaNum <= 0i32 {
        newSnap.valid = qtrue;
        old = 0 as *mut clSnapshot_t;
        clc.demowaiting = qfalse
    } else {
        old =
            &mut cl.snapshots[(newSnap.deltaNum & 32i32 - 1i32) as usize] as
                *mut clSnapshot_t;
        if 0 == (*old).valid as u64 {
            Com_Printf(b"Delta from invalid frame (not supposed to happen!).\n\x00"
                           as *const u8 as *const libc::c_char);
        } else if (*old).messageNum != newSnap.deltaNum {
            Com_Printf(b"Delta frame too old.\n\x00" as *const u8 as
                           *const libc::c_char);
        } else if cl.parseEntitiesNum - (*old).parseEntitiesNum >
                      32i32 * 256i32 - 256i32 {
            Com_Printf(b"Delta parseEntitiesNum too old.\n\x00" as *const u8
                           as *const libc::c_char);
        } else { newSnap.valid = qtrue }
    }
    len = MSG_ReadByte(msg);
    if len as libc::c_ulong >
           ::std::mem::size_of::<[byte; 32]>() as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_ParseSnapshot: Invalid size %d for areamask\x00" as
                      *const u8 as *const libc::c_char, len);
    }
    MSG_ReadData(msg,
                 &mut newSnap.areamask as *mut [byte; 32] as
                     *mut libc::c_void, len);
    SHOWNET(msg,
            b"playerstate\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    if !old.is_null() {
        MSG_ReadDeltaPlayerstate(msg, &mut (*old).ps, &mut newSnap.ps);
    } else {
        MSG_ReadDeltaPlayerstate(msg, 0 as *mut playerState_s,
                                 &mut newSnap.ps);
    }
    SHOWNET(msg,
            b"packet entities\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char);
    CL_ParsePacketEntities(msg, old, &mut newSnap);
    if 0 == newSnap.valid as u64 { return }
    oldMessageNum = cl.snap.messageNum + 1i32;
    if newSnap.messageNum - oldMessageNum >= 32i32 {
        oldMessageNum = newSnap.messageNum - (32i32 - 1i32)
    }
    while oldMessageNum < newSnap.messageNum {
        cl.snapshots[(oldMessageNum & 32i32 - 1i32) as usize].valid = qfalse;
        oldMessageNum += 1
    }
    cl.snap = newSnap;
    cl.snap.ping = 999i32;
    i = 0i32;
    while i < 32i32 {
        packetNum = clc.netchan.outgoingSequence - 1i32 - i & 32i32 - 1i32;
        if cl.snap.ps.commandTime >=
               cl.outPackets[packetNum as usize].p_serverTime {
            cl.snap.ping =
                cls.realtime - cl.outPackets[packetNum as usize].p_realtime;
            break ;
        } else { i += 1 }
    }
    cl.snapshots[(cl.snap.messageNum & 32i32 - 1i32) as usize] = cl.snap;
    if (*cl_shownet).integer == 3i32 {
        Com_Printf(b"   snapshot:%i  delta:%i  ping:%i\n\x00" as *const u8 as
                       *const libc::c_char, cl.snap.messageNum,
                   cl.snap.deltaNum, cl.snap.ping);
    }
    cl.newSnapshots = qtrue;
}
/*
==================
CL_ParsePacketEntities

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParsePacketEntities(mut msg: *mut msg_t,
                                                mut oldframe:
                                                    *mut clSnapshot_t,
                                                mut newframe:
                                                    *mut clSnapshot_t) {
    let mut newnum: libc::c_int = 0;
    let mut oldstate: *mut entityState_t = 0 as *mut entityState_t;
    let mut oldindex: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    (*newframe).parseEntitiesNum = cl.parseEntitiesNum;
    (*newframe).numEntities = 0i32;
    oldindex = 0i32;
    oldstate = 0 as *mut entityState_t;
    if oldframe.is_null() {
        oldnum = 99999i32
    } else if oldindex >= (*oldframe).numEntities {
        oldnum = 99999i32
    } else {
        oldstate =
            &mut cl.parseEntities[((*oldframe).parseEntitiesNum + oldindex &
                                       32i32 * 256i32 - 1i32) as usize] as
                *mut entityState_t;
        oldnum = (*oldstate).number
    }
    loop  {
        newnum = MSG_ReadBits(msg, 10i32);
        if newnum == (1i32 << 10i32) - 1i32 { break ; }
        if (*msg).readcount > (*msg).cursize {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CL_ParsePacketEntities: end of message\x00" as
                          *const u8 as *const libc::c_char);
        }
        while oldnum < newnum {
            if (*cl_shownet).integer == 3i32 {
                Com_Printf(b"%3i:  unchanged: %i\n\x00" as *const u8 as
                               *const libc::c_char, (*msg).readcount, oldnum);
            }
            CL_DeltaEntity(msg, newframe, oldnum, oldstate, qtrue);
            oldindex += 1;
            if oldindex >= (*oldframe).numEntities {
                oldnum = 99999i32
            } else {
                oldstate =
                    &mut cl.parseEntities[((*oldframe).parseEntitiesNum +
                                               oldindex &
                                               32i32 * 256i32 - 1i32) as
                                              usize] as *mut entityState_t;
                oldnum = (*oldstate).number
            }
        }
        if oldnum == newnum {
            if (*cl_shownet).integer == 3i32 {
                Com_Printf(b"%3i:  delta: %i\n\x00" as *const u8 as
                               *const libc::c_char, (*msg).readcount, newnum);
            }
            CL_DeltaEntity(msg, newframe, newnum, oldstate, qfalse);
            oldindex += 1;
            if oldindex >= (*oldframe).numEntities {
                oldnum = 99999i32
            } else {
                oldstate =
                    &mut cl.parseEntities[((*oldframe).parseEntitiesNum +
                                               oldindex &
                                               32i32 * 256i32 - 1i32) as
                                              usize] as *mut entityState_t;
                oldnum = (*oldstate).number
            }
        } else {
            if !(oldnum > newnum) { continue ; }
            if (*cl_shownet).integer == 3i32 {
                Com_Printf(b"%3i:  baseline: %i\n\x00" as *const u8 as
                               *const libc::c_char, (*msg).readcount, newnum);
            }
            CL_DeltaEntity(msg, newframe, newnum,
                           &mut cl.entityBaselines[newnum as usize], qfalse);
        }
    }
    while oldnum != 99999i32 {
        if (*cl_shownet).integer == 3i32 {
            Com_Printf(b"%3i:  unchanged: %i\n\x00" as *const u8 as
                           *const libc::c_char, (*msg).readcount, oldnum);
        }
        CL_DeltaEntity(msg, newframe, oldnum, oldstate, qtrue);
        oldindex += 1;
        if oldindex >= (*oldframe).numEntities {
            oldnum = 99999i32
        } else {
            oldstate =
                &mut cl.parseEntities[((*oldframe).parseEntitiesNum + oldindex
                                           & 32i32 * 256i32 - 1i32) as usize]
                    as *mut entityState_t;
            oldnum = (*oldstate).number
        }
    };
}
/*
=========================================================================

MESSAGE PARSING

=========================================================================
*/
/*
==================
CL_DeltaEntity

Parses deltas from the given base and adds the resulting entity
to the current frame
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DeltaEntity(mut msg: *mut msg_t,
                                        mut frame: *mut clSnapshot_t,
                                        mut newnum: libc::c_int,
                                        mut old: *mut entityState_t,
                                        mut unchanged: qboolean) {
    let mut state: *mut entityState_t = 0 as *mut entityState_t;
    state =
        &mut cl.parseEntities[(cl.parseEntitiesNum & 32i32 * 256i32 - 1i32) as
                                  usize] as *mut entityState_t;
    if 0 != unchanged as u64 {
        *state = *old
    } else { MSG_ReadDeltaEntity(msg, old, state, newnum); }
    if (*state).number == (1i32 << 10i32) - 1i32 { return }
    cl.parseEntitiesNum += 1;
    (*frame).numEntities += 1;
}
#[no_mangle]
pub unsafe extern "C" fn SHOWNET(mut msg: *mut msg_t,
                                 mut s: *mut libc::c_char) {
    if (*cl_shownet).integer >= 2i32 {
        Com_Printf(b"%3i:%s\n\x00" as *const u8 as *const libc::c_char,
                   (*msg).readcount - 1i32, s);
    };
}
/*
==================
CL_ParseGamestate
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseGamestate(mut msg: *mut msg_t) {
    let mut i: libc::c_int = 0;
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut newnum: libc::c_int = 0;
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
    let mut cmd: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldGame: [libc::c_char; 64] = [0; 64];
    Con_Close();
    clc.connectPacketCount = 0i32;
    CL_ClearState();
    clc.serverCommandSequence = MSG_ReadLong(msg);
    cl.gameState.dataCount = 1i32;
    loop  {
        cmd = MSG_ReadByte(msg);
        if cmd == svc_EOF as libc::c_int { break ; }
        if cmd == svc_configstring as libc::c_int {
            let mut len: libc::c_int = 0;
            i = MSG_ReadShort(msg);
            if i < 0i32 || i >= 1024i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"configstring > MAX_CONFIGSTRINGS\x00" as *const u8
                              as *const libc::c_char);
            }
            s = MSG_ReadBigString(msg);
            len = strlen(s) as libc::c_int;
            if len + 1i32 + cl.gameState.dataCount > 16000i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"MAX_GAMESTATE_CHARS exceeded\x00" as *const u8 as
                              *const libc::c_char);
            }
            cl.gameState.stringOffsets[i as usize] = cl.gameState.dataCount;
            memcpy(cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.dataCount
                                                                   as isize)
                       as *mut libc::c_void, s as *const libc::c_void,
                   (len + 1i32) as libc::c_ulong);
            cl.gameState.dataCount += len + 1i32
        } else if cmd == svc_baseline as libc::c_int {
            newnum = MSG_ReadBits(msg, 10i32);
            if newnum < 0i32 || newnum >= 1i32 << 10i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"Baseline number out of range: %i\x00" as *const u8
                              as *const libc::c_char, newnum);
            }
            memset(&mut nullstate as *mut entityState_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<entityState_t>() as libc::c_ulong);
            es =
                &mut cl.entityBaselines[newnum as usize] as
                    *mut entityState_t;
            MSG_ReadDeltaEntity(msg, &mut nullstate, es, newnum);
        } else {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CL_ParseGamestate: bad command byte\x00" as *const u8
                          as *const libc::c_char);
        }
    }
    clc.clientNum = MSG_ReadLong(msg);
    clc.checksumFeed = MSG_ReadLong(msg);
    Cvar_VariableStringBuffer(b"fs_game\x00" as *const u8 as
                                  *const libc::c_char, oldGame.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong as libc::c_int);
    CL_ParseServerInfo();
    CL_SystemInfoChanged();
    if 0 != (*cl_autoRecordDemo).integer &&
           0 != clc.demorecording as libc::c_uint {
        CL_StopRecord_f();
    }
    if 0 == cl_oldGameSet as u64 &&
           0 !=
               Cvar_Flags(b"fs_game\x00" as *const u8 as *const libc::c_char)
                   & 0x40000000i32 {
        cl_oldGameSet = qtrue;
        Q_strncpyz(cl_oldGame.as_mut_ptr(), oldGame.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
    }
    FS_ConditionalRestart(clc.checksumFeed, qfalse);
    CL_InitDownloads();
    Cvar_Set(b"cl_paused\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
}
/*
==================
CL_ParseServerInfo
==================
*/
unsafe extern "C" fn CL_ParseServerInfo() {
    let mut serverInfo: *const libc::c_char = 0 as *const libc::c_char;
    serverInfo =
        cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[0usize]
                                                        as isize);
    clc.sv_allowDownload =
        atoi(Info_ValueForKey(serverInfo,
                              b"sv_allowDownload\x00" as *const u8 as
                                  *const libc::c_char));
    Q_strncpyz(clc.sv_dlURL.as_mut_ptr(),
               Info_ValueForKey(serverInfo,
                                b"sv_dlURL\x00" as *const u8 as
                                    *const libc::c_char),
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                   as libc::c_int);
}
/*
=====================
CL_ParseCommandString

Command strings are just saved off until cgame asks for them
when it transitions a snapshot
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseCommandString(mut msg: *mut msg_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seq: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    seq = MSG_ReadLong(msg);
    s = MSG_ReadString(msg);
    if clc.serverCommandSequence >= seq { return }
    clc.serverCommandSequence = seq;
    index = seq & 64i32 - 1i32;
    Q_strncpyz(clc.serverCommands[index as usize].as_mut_ptr(), s,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
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
// cl_parse.c  -- parse a message received from the server
#[no_mangle]
pub static mut svc_strings: [*mut libc::c_char; 256] =
    [b"svc_bad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"svc_nop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"svc_gamestate\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_configstring\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_baseline\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_serverCommand\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_download\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_snapshot\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_EOF\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"svc_voipSpeex\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"svc_voipOpus\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char, 0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char];