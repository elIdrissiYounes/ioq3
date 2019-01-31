use libc;
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
    // font rendering values used by ui and cgame
    // default
    // default
    pub type ha_pref = libc::c_uint;
    pub const h_dontcare: ha_pref = 2;
    pub const h_low: ha_pref = 1;
    pub const h_high: ha_pref = 0;
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
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn RadiusFromBounds(mins: *const vec_t, maxs: *const vec_t)
         -> libc::c_float;
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
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
    pub type vm_t = vm_s;
    pub type unnamed_0 = libc::c_uint;
    pub const TAG_STATIC: unnamed_0 = 5;
    pub const TAG_SMALL: unnamed_0 = 4;
    pub const TAG_RENDERER: unnamed_0 = 3;
    pub const TAG_BOTLIB: unnamed_0 = 2;
    pub const TAG_GENERAL: unnamed_0 = 1;
    pub const TAG_FREE: unnamed_0 = 0;
    use super::q_shared_h::{qboolean, byte, cvar_t, fileHandle_t, fsMode_t};
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
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
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
        pub fn Cvar_VariableIntegerValue(var_name: *const libc::c_char)
         -> libc::c_int;
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
        pub static mut com_basegame: *mut cvar_t;
        /*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
        #[no_mangle]
        pub fn Z_TagMallocDebug(size: libc::c_int, tag: libc::c_int,
                                label: *mut libc::c_char,
                                file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Z_AvailableMemory() -> libc::c_int;
        #[no_mangle]
        pub fn Hunk_CheckMark() -> qboolean;
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
    pub type unnamed_1 = libc::c_uint;
    // ConsoleCommand will be called when a command has been issued
	// that is not recognized as a builtin function.
	// The game can issue trap_argc() / trap_argv() commands to get the command
	// and parameters.  Return qfalse if the game doesn't recognize it as a command.
    // ( int time );
    pub const BOTAI_START_FRAME: unnamed_1 = 10;
    // ( void );
    pub const GAME_CONSOLE_COMMAND: unnamed_1 = 9;
    // ( int levelTime );
    pub const GAME_RUN_FRAME: unnamed_1 = 8;
    // ( int clientNum );
    pub const GAME_CLIENT_THINK: unnamed_1 = 7;
    // ( int clientNum );
    pub const GAME_CLIENT_COMMAND: unnamed_1 = 6;
    // ( int clientNum );
    pub const GAME_CLIENT_DISCONNECT: unnamed_1 = 5;
    // ( int clientNum );
    pub const GAME_CLIENT_USERINFO_CHANGED: unnamed_1 = 4;
    // return NULL if the client is allowed to connect, otherwise return
	// a text string with the reason for denial
    // ( int clientNum );
    pub const GAME_CLIENT_BEGIN: unnamed_1 = 3;
    // ( int clientNum, qboolean firstTime, qboolean isBot );
    pub const GAME_CLIENT_CONNECT: unnamed_1 = 2;
    // init and shutdown will be called every single level
	// The game should call G_GET_ENTITY_TOKEN to parse through all the
	// entity configuration text and spawn gentities.
    // (void);
    pub const GAME_SHUTDOWN: unnamed_1 = 1;
    // ( int levelTime, int randomSeed, int restart );
    pub const GAME_INIT: unnamed_1 = 0;
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
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
    use super::{libc};
    use super::q_shared_h::{byte, playerState_t, usercmd_t, fileHandle_t,
                            qboolean, entityState_t, cvar_t, vec_t, vec3_t,
                            trace_t};
    use super::qcommon_h::{msg_t, netchan_t, netadr_t, vm_t};
    use super::g_public_h::{sharedEntity_t};
    extern "C" {
        //=============================================================================
        // persistant server info across maps
        #[no_mangle]
        pub static mut svs: serverStatic_t;
        // game virtual machine
        #[no_mangle]
        pub static mut gvm: *mut vm_t;
        #[no_mangle]
        pub static mut sv_maxclients: *mut cvar_t;
        #[no_mangle]
        pub fn SV_ExecuteClientCommand(cl: *mut client_t,
                                       s: *const libc::c_char,
                                       clientOK: qboolean);
        #[no_mangle]
        pub fn SV_GentityNum(num: libc::c_int) -> *mut sharedEntity_t;
        #[no_mangle]
        pub fn SV_inPVS(p1: *const vec_t, p2: *const vec_t) -> qboolean;
        // fills in a table of entity numbers with entities that have bounding boxes
// that intersect the given area.  It is possible for a non-axial bmodel
// to be returned that doesn't actually intersect the area on an exact
// test.
// returns the number of pointers filled in
// The world entity is never returned in this list.
        #[no_mangle]
        pub fn SV_PointContents(p: *const vec_t, passEntityNum: libc::c_int)
         -> libc::c_int;
        // mins and maxs are relative
        // if the entire move stays in a solid volume, trace.allsolid will be set,
// trace.startsolid will be set, and trace.fraction will be 0
        // if the starting point is in a solid, it will be allowed to move out
// to an open area
        // passEntityNum is explicitly excluded from clipping checks (normally ENTITYNUM_NONE)
        #[no_mangle]
        pub fn SV_ClipToEntity(trace: *mut trace_t, start: *const vec_t,
                               mins: *const vec_t, maxs: *const vec_t,
                               end: *const vec_t, entityNum: libc::c_int,
                               contentmask: libc::c_int,
                               capsule: libc::c_int);
        // returns the CONTENTS_* value from the world and all entities at the given point.
        #[no_mangle]
        pub fn SV_Trace(results: *mut trace_t, start: *const vec_t,
                        mins: *mut vec_t, maxs: *mut vec_t, end: *const vec_t,
                        passEntityNum: libc::c_int, contentmask: libc::c_int,
                        capsule: libc::c_int);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
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
    pub type botlib_import_t = botlib_import_s;
    //bot AI library exported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_char, ...)
                              -> ()>,
        pub Trace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: libc::c_int, _: libc::c_int)
                              -> ()>,
        pub EntityTrace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
        pub PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                      -> libc::c_int>,
        pub inPVS: Option<unsafe extern "C" fn(_: *mut vec_t, _: *mut vec_t)
                              -> libc::c_int>,
        pub BSPEntityData: Option<unsafe extern "C" fn()
                                      -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t)
                                               -> ()>,
        pub BotClientCommand: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut fileHandle_t,
                                                      _: fsMode_t)
                                     -> libc::c_int>,
        pub FS_Read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: libc::c_int,
                                                 _: fileHandle_t)
                                -> libc::c_int>,
        pub FS_Write: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                                  _: libc::c_int,
                                                  _: fileHandle_t)
                                 -> libc::c_int>,
        pub FS_FCloseFile: Option<unsafe extern "C" fn(_: fileHandle_t)
                                      -> ()>,
        pub FS_Seek: Option<unsafe extern "C" fn(_: fileHandle_t,
                                                 _: libc::c_long,
                                                 _: libc::c_int)
                                -> libc::c_int>,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
        pub DebugLineShow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: libc::c_int) -> ()>,
        pub DebugPolygonCreate: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: *mut vec3_t)
                                           -> libc::c_int>,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    }
    pub type bsp_trace_t = bsp_trace_s;
    //remove the bsp_trace_s structure definition l8r on
//a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_trace_s {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    pub type bsp_surface_t = bsp_surface_s;
    //bsp_trace_t hit surface
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{pc_token_t, vec_t, vec3_t, fileHandle_t, fsMode_t,
                            qboolean, cplane_t};
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
        //linking of bot library
        #[no_mangle]
        pub fn GetBotLibAPI(apiVersion: libc::c_int,
                            import: *mut botlib_import_t)
         -> *mut botlib_export_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/sv_bot.c"]
pub mod sv_bot_c {
    pub type bot_debugpoly_t = bot_debugpoly_s;
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
// sv_bot.c
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_debugpoly_s {
        pub inuse: libc::c_int,
        pub color: libc::c_int,
        pub numPoints: libc::c_int,
        pub points: [vec3_t; 128],
    }
    use super::{libc};
    use super::q_shared_h::{vec3_t};
    use super::botlib_h::{botlib_export_t};
    extern "C" {
        #[no_mangle]
        pub static mut botlib_export: *mut botlib_export_t;
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
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::q_shared_h::{clipHandle_t, vec_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CM_InlineModel(index: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_ModelBounds(model: clipHandle_t, mins: *mut vec_t,
                              maxs: *mut vec_t);
        #[no_mangle]
        pub fn CM_EntityString() -> *mut libc::c_char;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/server/sv_variadic.h"]
pub mod sv_variadic_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn BotImport_Print(type_0: libc::c_int,
                               fmt: *mut libc::c_char, ...);
    }
}
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t,
                       clipHandle_t, unnamed, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, ha_pref,
                       h_dontcare, h_low, h_high, vec_t, vec3_t, cplane_s,
                       pc_token_s, pc_token_t, fsMode_t, FS_APPEND_SYNC,
                       FS_APPEND, FS_WRITE, FS_READ, cvar_s, cvar_t, cplane_t,
                       trace_t, playerState_s, playerState_t, usercmd_s,
                       usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, Hunk_AllocDebug, RadiusFromBounds,
                       VectorNormalize, Q_strncpyz, Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      vm_t, unnamed_0, TAG_STATIC, TAG_SMALL, TAG_RENDERER,
                      TAG_BOTLIB, TAG_GENERAL, TAG_FREE, vm_s, VM_Call,
                      Cvar_Get, Cvar_VariableIntegerValue, FS_Write, FS_Read,
                      FS_FCloseFile, FS_FOpenFileByMode, FS_Seek,
                      com_basegame, Z_TagMallocDebug, Z_MallocDebug, Z_Free,
                      Z_AvailableMemory, Hunk_CheckMark};
use self::g_public_h::{entityShared_t, sharedEntity_t, unnamed_1,
                       BOTAI_START_FRAME, GAME_CONSOLE_COMMAND,
                       GAME_RUN_FRAME, GAME_CLIENT_THINK, GAME_CLIENT_COMMAND,
                       GAME_CLIENT_DISCONNECT, GAME_CLIENT_USERINFO_CHANGED,
                       GAME_CLIENT_BEGIN, GAME_CLIENT_CONNECT, GAME_SHUTDOWN,
                       GAME_INIT};
use self::server_h::{voipServerPacket_s, voipServerPacket_t, clientSnapshot_t,
                     clientState_t, CS_ACTIVE, CS_PRIMED, CS_CONNECTED,
                     CS_ZOMBIE, CS_FREE, netchan_buffer_s, netchan_buffer_t,
                     client_s, client_t, challenge_t, serverStatic_t, svs,
                     gvm, sv_maxclients, SV_ExecuteClientCommand,
                     SV_GentityNum, SV_inPVS, SV_PointContents,
                     SV_ClipToEntity, SV_Trace};
use self::botlib_h::{botlib_export_t, botlib_export_s, bot_entitystate_t,
                     bot_entitystate_s, ai_export_t, ai_export_s, ea_export_t,
                     ea_export_s, bot_input_t, bot_input_s, aas_export_t,
                     aas_export_s, botlib_import_t, botlib_import_s,
                     bsp_trace_t, bsp_trace_s, bsp_surface_t, bsp_surface_s,
                     weaponinfo_s, bot_initmove_s, bot_goal_s,
                     bot_moveresult_s, bot_match_s, bot_consolemessage_s,
                     aas_clientmove_s, aas_altroutegoal_s, aas_predictroute_s,
                     aas_areainfo_s, aas_entityinfo_s, GetBotLibAPI};
use self::sv_bot_c::{bot_debugpoly_t, bot_debugpoly_s, botlib_export};
use self::assert_h::{__assert_fail};
use self::string_h::{memcpy};
use self::cm_public_h::{CM_InlineModel, CM_ModelBounds, CM_EntityString};
use self::sv_variadic_h::{BotImport_Print};
unsafe extern "C" fn CrossProduct(mut v1: *const vec_t, mut v2: *const vec_t,
                                  mut cross: *mut vec_t) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) -
            *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) -
            *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) -
            *v1.offset(1isize) * *v2.offset(0isize);
}
//
// sv_bot.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_BotFrame(mut time: libc::c_int) {
    if 0 == bot_enable { return }
    if gvm.is_null() { return }
    VM_Call(gvm, BOTAI_START_FRAME as libc::c_int, time);
}
#[no_mangle]
pub static mut bot_enable: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn SV_BotAllocateClient() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if (*cl).state as libc::c_uint ==
               CS_FREE as libc::c_int as libc::c_uint {
            break ;
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    if i == (*sv_maxclients).integer { return -1i32 }
    (*cl).gentity = SV_GentityNum(i);
    (*(*cl).gentity).s.number = i;
    (*cl).state = CS_ACTIVE;
    (*cl).lastPacketTime = svs.time;
    (*cl).netchan.remoteAddress.type_0 = NA_BOT;
    (*cl).rate = 16384i32;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotFreeClient(mut clientNum: libc::c_int) {
    let mut cl: *mut client_t = 0 as *mut client_t;
    if clientNum < 0i32 || clientNum >= (*sv_maxclients).integer {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_BotFreeClient: bad clientNum: %i\x00" as *const u8 as
                      *const libc::c_char, clientNum);
    }
    cl = &mut *svs.clients.offset(clientNum as isize) as *mut client_t;
    (*cl).state = CS_FREE;
    (*cl).name[0usize] = 0i32 as libc::c_char;
    if !(*cl).gentity.is_null() { (*(*cl).gentity).r.svFlags &= !0x8i32 };
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotInitCvars() {
    Cvar_Get(b"bot_enable\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_developer\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_debug\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_maxdebugpolys\x00" as *const u8 as *const libc::c_char,
             b"2\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_groundonly\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_reachability\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_forceclustering\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_forcereachability\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_forcewrite\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_aasoptimize\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_thinktime\x00" as *const u8 as *const libc::c_char,
             b"100\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_testichat\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_testrchat\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_testsolid\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_testclusters\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_fastchat\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_nochat\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_pause\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_report\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_grapple\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_rocketjump\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_challenge\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_minplayers\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"bot_interbreedchar\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_interbreedbots\x00" as *const u8 as *const libc::c_char,
             b"10\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_interbreedcycle\x00" as *const u8 as *const libc::c_char,
             b"20\x00" as *const u8 as *const libc::c_char, 0x200i32);
    Cvar_Get(b"bot_interbreedwrite\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x200i32);
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotLibSetup() -> libc::c_int {
    if 0 == bot_enable { return 0i32 }
    if botlib_export.is_null() {
        Com_Printf(b"^1Error: SV_BotLibSetup without SV_BotInitBotLib\n\x00"
                       as *const u8 as *const libc::c_char);
        return -1i32
    }
    (*botlib_export).BotLibVarSet.expect("non-null function pointer")(b"basegame\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      (*com_basegame).string);
    return (*botlib_export).BotLibSetup.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotLibShutdown() -> libc::c_int {
    if botlib_export.is_null() { return -1i32 }
    return (*botlib_export).BotLibShutdown.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotGetSnapshotEntity(mut client: libc::c_int,
                                                 mut sequence: libc::c_int)
 -> libc::c_int {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut frame: *mut clientSnapshot_t = 0 as *mut clientSnapshot_t;
    cl = &mut *svs.clients.offset(client as isize) as *mut client_t;
    frame =
        &mut (*cl).frames[((*cl).netchan.outgoingSequence & 32i32 - 1i32) as
                              usize] as *mut clientSnapshot_t;
    if sequence < 0i32 || sequence >= (*frame).num_entities { return -1i32 }
    return (*svs.snapshotEntities.offset((((*frame).first_entity + sequence) %
                                              svs.numSnapshotEntities) as
                                             isize)).number;
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotGetConsoleMessage(mut client: libc::c_int,
                                                 mut buf: *mut libc::c_char,
                                                 mut size: libc::c_int)
 -> libc::c_int {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut index: libc::c_int = 0;
    cl = &mut *svs.clients.offset(client as isize) as *mut client_t;
    (*cl).lastPacketTime = svs.time;
    if (*cl).reliableAcknowledge == (*cl).reliableSequence {
        return qfalse as libc::c_int
    }
    (*cl).reliableAcknowledge += 1;
    index = (*cl).reliableAcknowledge & 64i32 - 1i32;
    if 0 == (*cl).reliableCommands[index as usize][0usize] {
        return qfalse as libc::c_int
    }
    Q_strncpyz(buf, (*cl).reliableCommands[index as usize].as_mut_ptr(),
               size);
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BotImport_DebugPolygonCreate(mut color: libc::c_int,
                                                      mut numPoints:
                                                          libc::c_int,
                                                      mut points: *mut vec3_t)
 -> libc::c_int {
    let mut poly: *mut bot_debugpoly_t = 0 as *mut bot_debugpoly_t;
    let mut i: libc::c_int = 0;
    if debugpolygons.is_null() { return 0i32 }
    i = 1i32;
    while i < bot_maxdebugpolys {
        if 0 == (*debugpolygons.offset(i as isize)).inuse { break ; }
        i += 1
    }
    if i >= bot_maxdebugpolys { return 0i32 }
    poly = &mut *debugpolygons.offset(i as isize) as *mut bot_debugpoly_t;
    (*poly).inuse = qtrue as libc::c_int;
    (*poly).color = color;
    (*poly).numPoints = numPoints;
    memcpy((*poly).points.as_mut_ptr() as *mut libc::c_void,
           points as *const libc::c_void,
           (numPoints as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec3_t>() as
                                                libc::c_ulong));
    return i;
}
static mut debugpolygons: *mut bot_debugpoly_t =
    0 as *const bot_debugpoly_t as *mut bot_debugpoly_t;
#[no_mangle]
pub static mut bot_maxdebugpolys: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn BotImport_DebugPolygonDelete(mut id: libc::c_int) {
    if debugpolygons.is_null() { return }
    (*debugpolygons.offset(id as isize)).inuse = qfalse as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_BotInitBotLib() {
    let mut botlib_import: botlib_import_t =
        botlib_import_s{Print: None,
                        Trace: None,
                        EntityTrace: None,
                        PointContents: None,
                        inPVS: None,
                        BSPEntityData: None,
                        BSPModelMinsMaxsOrigin: None,
                        BotClientCommand: None,
                        GetMemory: None,
                        FreeMemory: None,
                        AvailableMemory: None,
                        HunkAlloc: None,
                        FS_FOpenFile: None,
                        FS_Read: None,
                        FS_Write: None,
                        FS_FCloseFile: None,
                        FS_Seek: None,
                        DebugLineCreate: None,
                        DebugLineDelete: None,
                        DebugLineShow: None,
                        DebugPolygonCreate: None,
                        DebugPolygonDelete: None,};
    if !debugpolygons.is_null() {
        Z_Free(debugpolygons as *mut libc::c_void);
    }
    bot_maxdebugpolys =
        Cvar_VariableIntegerValue(b"bot_maxdebugpolys\x00" as *const u8 as
                                      *const libc::c_char);
    debugpolygons =
        Z_MallocDebug((::std::mem::size_of::<bot_debugpoly_t>() as
                           libc::c_ulong).wrapping_mul(bot_maxdebugpolys as
                                                           libc::c_ulong) as
                          libc::c_int,
                      b"sizeof(bot_debugpoly_t) * bot_maxdebugpolys\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/server/sv_bot.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 487i32) as
            *mut bot_debugpoly_t;
    botlib_import.Print = Some(BotImport_Print);
    botlib_import.Trace = Some(BotImport_Trace);
    botlib_import.EntityTrace = Some(BotImport_EntityTrace);
    botlib_import.PointContents = Some(BotImport_PointContents);
    botlib_import.inPVS = Some(BotImport_inPVS);
    botlib_import.BSPEntityData = Some(BotImport_BSPEntityData);
    botlib_import.BSPModelMinsMaxsOrigin =
        Some(BotImport_BSPModelMinsMaxsOrigin);
    botlib_import.BotClientCommand = Some(BotClientCommand);
    botlib_import.GetMemory = Some(BotImport_GetMemory);
    botlib_import.FreeMemory = Some(BotImport_FreeMemory);
    botlib_import.AvailableMemory = Some(Z_AvailableMemory);
    botlib_import.HunkAlloc = Some(BotImport_HunkAlloc);
    botlib_import.FS_FOpenFile = Some(FS_FOpenFileByMode);
    botlib_import.FS_Read = Some(FS_Read);
    botlib_import.FS_Write = Some(FS_Write);
    botlib_import.FS_FCloseFile = Some(FS_FCloseFile);
    botlib_import.FS_Seek = Some(FS_Seek);
    botlib_import.DebugLineCreate = Some(BotImport_DebugLineCreate);
    botlib_import.DebugLineDelete = Some(BotImport_DebugLineDelete);
    botlib_import.DebugLineShow = Some(BotImport_DebugLineShow);
    botlib_import.DebugPolygonCreate = Some(BotImport_DebugPolygonCreate);
    botlib_import.DebugPolygonDelete = Some(BotImport_DebugPolygonDelete);
    botlib_export = GetBotLibAPI(2i32, &mut botlib_import);
    if !botlib_export.is_null() {
    } else {
        __assert_fail(b"botlib_export\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/server/sv_bot.c\x00" as *const u8 as
                          *const libc::c_char, 521i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void SV_BotInitBotLib(void)\x00")).as_ptr());
    };
}
/*
==================
BotImport_DebugLineShow
==================
*/
unsafe extern "C" fn BotImport_DebugLineShow(mut line: libc::c_int,
                                             mut start: *mut vec_t,
                                             mut end: *mut vec_t,
                                             mut color: libc::c_int) {
    let mut points: [vec3_t; 4] = [[0.; 3]; 4];
    let mut dir: vec3_t = [0.; 3];
    let mut cross: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut dot: libc::c_float = 0.;
    points[0usize][0usize] = *start.offset(0isize);
    points[0usize][1usize] = *start.offset(1isize);
    points[0usize][2usize] = *start.offset(2isize);
    points[1usize][0usize] = *start.offset(0isize);
    points[1usize][1usize] = *start.offset(1isize);
    points[1usize][2usize] = *start.offset(2isize);
    points[2usize][0usize] = *end.offset(0isize);
    points[2usize][1usize] = *end.offset(1isize);
    points[2usize][2usize] = *end.offset(2isize);
    points[3usize][0usize] = *end.offset(0isize);
    points[3usize][1usize] = *end.offset(1isize);
    points[3usize][2usize] = *end.offset(2isize);
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    VectorNormalize(dir.as_mut_ptr());
    dot =
        dir[0usize] * up[0usize] + dir[1usize] * up[1usize] +
            dir[2usize] * up[2usize];
    if dot as libc::c_double > 0.99f64 || (dot as libc::c_double) < -0.99f64 {
        cross[0usize] = 1i32 as vec_t;
        cross[1usize] = 0i32 as vec_t;
        cross[2usize] = 0i32 as vec_t
    } else {
        CrossProduct(dir.as_mut_ptr() as *const vec_t,
                     up.as_mut_ptr() as *const vec_t, cross.as_mut_ptr());
    }
    VectorNormalize(cross.as_mut_ptr());
    points[0usize][0usize] =
        points[0usize][0usize] + cross[0usize] * 2i32 as libc::c_float;
    points[0usize][1usize] =
        points[0usize][1usize] + cross[1usize] * 2i32 as libc::c_float;
    points[0usize][2usize] =
        points[0usize][2usize] + cross[2usize] * 2i32 as libc::c_float;
    points[1usize][0usize] =
        points[1usize][0usize] + cross[0usize] * -2i32 as libc::c_float;
    points[1usize][1usize] =
        points[1usize][1usize] + cross[1usize] * -2i32 as libc::c_float;
    points[1usize][2usize] =
        points[1usize][2usize] + cross[2usize] * -2i32 as libc::c_float;
    points[2usize][0usize] =
        points[2usize][0usize] + cross[0usize] * -2i32 as libc::c_float;
    points[2usize][1usize] =
        points[2usize][1usize] + cross[1usize] * -2i32 as libc::c_float;
    points[2usize][2usize] =
        points[2usize][2usize] + cross[2usize] * -2i32 as libc::c_float;
    points[3usize][0usize] =
        points[3usize][0usize] + cross[0usize] * 2i32 as libc::c_float;
    points[3usize][1usize] =
        points[3usize][1usize] + cross[1usize] * 2i32 as libc::c_float;
    points[3usize][2usize] =
        points[3usize][2usize] + cross[2usize] * 2i32 as libc::c_float;
    BotImport_DebugPolygonShow(line, color, 4i32, points.as_mut_ptr());
}
/*
==================
BotImport_DebugPolygonShow
==================
*/
unsafe extern "C" fn BotImport_DebugPolygonShow(mut id: libc::c_int,
                                                mut color: libc::c_int,
                                                mut numPoints: libc::c_int,
                                                mut points: *mut vec3_t) {
    let mut poly: *mut bot_debugpoly_t = 0 as *mut bot_debugpoly_t;
    if debugpolygons.is_null() { return }
    poly = &mut *debugpolygons.offset(id as isize) as *mut bot_debugpoly_t;
    (*poly).inuse = qtrue as libc::c_int;
    (*poly).color = color;
    (*poly).numPoints = numPoints;
    memcpy((*poly).points.as_mut_ptr() as *mut libc::c_void,
           points as *const libc::c_void,
           (numPoints as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec3_t>() as
                                                libc::c_ulong));
}
/*
==================
BotImport_DebugLineDelete
==================
*/
unsafe extern "C" fn BotImport_DebugLineDelete(mut line: libc::c_int) {
    BotImport_DebugPolygonDelete(line);
}
/*
==================
BotImport_DebugLineCreate
==================
*/
unsafe extern "C" fn BotImport_DebugLineCreate() -> libc::c_int {
    let mut points: [vec3_t; 1] = [[0.; 3]; 1];
    return BotImport_DebugPolygonCreate(0i32, 0i32, points.as_mut_ptr());
}
/*
=================
BotImport_HunkAlloc
=================
*/
unsafe extern "C" fn BotImport_HunkAlloc(mut size: libc::c_int)
 -> *mut libc::c_void {
    if 0 != Hunk_CheckMark() as u64 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_Bot_HunkAlloc: Alloc with marks already set\x00" as
                      *const u8 as *const libc::c_char);
    }
    return Hunk_AllocDebug(size, h_high,
                           b"size\x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char,
                           b"code/server/sv_bot.c\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           271i32);
}
/*
==================
BotImport_FreeMemory
==================
*/
unsafe extern "C" fn BotImport_FreeMemory(mut ptr: *mut libc::c_void) {
    Z_Free(ptr);
}
/*
==================
BotImport_GetMemory
==================
*/
unsafe extern "C" fn BotImport_GetMemory(mut size: libc::c_int)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr =
        Z_TagMallocDebug(size, TAG_BOTLIB as libc::c_int,
                         b"size\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         b"code/server/sv_bot.c\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                         249i32);
    return ptr;
}
/*
==================
SV_BotClientCommand
==================
*/
unsafe extern "C" fn BotClientCommand(mut client: libc::c_int,
                                      mut command: *mut libc::c_char) {
    SV_ExecuteClientCommand(&mut *svs.clients.offset(client as isize),
                            command, qtrue);
}
/*
==================
BotImport_BSPModelMinsMaxsOrigin
==================
*/
unsafe extern "C" fn BotImport_BSPModelMinsMaxsOrigin(mut modelnum:
                                                          libc::c_int,
                                                      mut angles: *mut vec_t,
                                                      mut outmins: *mut vec_t,
                                                      mut outmaxs: *mut vec_t,
                                                      mut origin:
                                                          *mut vec_t) {
    let mut h: clipHandle_t = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut max: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    h = CM_InlineModel(modelnum);
    CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    if 0. != *angles.offset(0isize) || 0. != *angles.offset(1isize) ||
           0. != *angles.offset(2isize) {
        max =
            RadiusFromBounds(mins.as_mut_ptr() as *const vec_t,
                             maxs.as_mut_ptr() as *const vec_t);
        i = 0i32;
        while i < 3i32 {
            mins[i as usize] = -max;
            maxs[i as usize] = max;
            i += 1
        }
    }
    if !outmins.is_null() {
        *outmins.offset(0isize) = mins[0usize];
        *outmins.offset(1isize) = mins[1usize];
        *outmins.offset(2isize) = mins[2usize]
    }
    if !outmaxs.is_null() {
        *outmaxs.offset(0isize) = maxs[0usize];
        *outmaxs.offset(1isize) = maxs[1usize];
        *outmaxs.offset(2isize) = maxs[2usize]
    }
    if !origin.is_null() {
        let ref mut fresh1 = *origin.offset(1isize);
        let ref mut fresh0 = *origin.offset(2isize);
        *fresh0 = 0i32 as vec_t;
        *fresh1 = *fresh0;
        *origin.offset(0isize) = *fresh1
    };
}
/*
==================
BotImport_BSPEntityData
==================
*/
unsafe extern "C" fn BotImport_BSPEntityData() -> *mut libc::c_char {
    return CM_EntityString();
}
/*
==================
BotImport_inPVS
==================
*/
unsafe extern "C" fn BotImport_inPVS(mut p1: *mut vec_t, mut p2: *mut vec_t)
 -> libc::c_int {
    return SV_inPVS(p1 as *const vec_t, p2 as *const vec_t) as libc::c_int;
}
/*
==================
BotImport_PointContents
==================
*/
unsafe extern "C" fn BotImport_PointContents(mut point: *mut vec_t)
 -> libc::c_int {
    return SV_PointContents(point as *const vec_t, -1i32);
}
/*
==================
BotImport_EntityTrace
==================
*/
unsafe extern "C" fn BotImport_EntityTrace(mut bsptrace: *mut bsp_trace_t,
                                           mut start: *mut vec_t,
                                           mut mins: *mut vec_t,
                                           mut maxs: *mut vec_t,
                                           mut end: *mut vec_t,
                                           mut entnum: libc::c_int,
                                           mut contentmask: libc::c_int) {
    let mut trace: trace_t =
        trace_t{allsolid: qfalse,
                startsolid: qfalse,
                fraction: 0.,
                endpos: [0.; 3],
                plane:
                    cplane_s{normal: [0.; 3],
                             dist: 0.,
                             type_0: 0,
                             signbits: 0,
                             pad: [0; 2],},
                surfaceFlags: 0,
                contents: 0,
                entityNum: 0,};
    SV_ClipToEntity(&mut trace, start as *const vec_t, mins as *const vec_t,
                    maxs as *const vec_t, end as *const vec_t, entnum,
                    contentmask, qfalse as libc::c_int);
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0usize] = trace.endpos[0usize];
    (*bsptrace).endpos[1usize] = trace.endpos[1usize];
    (*bsptrace).endpos[2usize] = trace.endpos[2usize];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0usize] = trace.plane.normal[0usize];
    (*bsptrace).plane.normal[1usize] = trace.plane.normal[1usize];
    (*bsptrace).plane.normal[2usize] = trace.plane.normal[2usize];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0i32;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0i32 as libc::c_float;
    (*bsptrace).sidenum = 0i32;
    (*bsptrace).contents = 0i32;
}
//Com_Printf("poly %i, numpoints = %d\n", i, poly->numPoints);
/*
==================
BotImport_Trace
==================
*/
unsafe extern "C" fn BotImport_Trace(mut bsptrace: *mut bsp_trace_t,
                                     mut start: *mut vec_t,
                                     mut mins: *mut vec_t,
                                     mut maxs: *mut vec_t,
                                     mut end: *mut vec_t,
                                     mut passent: libc::c_int,
                                     mut contentmask: libc::c_int) {
    let mut trace: trace_t =
        trace_t{allsolid: qfalse,
                startsolid: qfalse,
                fraction: 0.,
                endpos: [0.; 3],
                plane:
                    cplane_s{normal: [0.; 3],
                             dist: 0.,
                             type_0: 0,
                             signbits: 0,
                             pad: [0; 2],},
                surfaceFlags: 0,
                contents: 0,
                entityNum: 0,};
    SV_Trace(&mut trace, start as *const vec_t, mins, maxs,
             end as *const vec_t, passent, contentmask,
             qfalse as libc::c_int);
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0usize] = trace.endpos[0usize];
    (*bsptrace).endpos[1usize] = trace.endpos[1usize];
    (*bsptrace).endpos[2usize] = trace.endpos[2usize];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0usize] = trace.plane.normal[0usize];
    (*bsptrace).plane.normal[1usize] = trace.plane.normal[1usize];
    (*bsptrace).plane.normal[2usize] = trace.plane.normal[2usize];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0i32;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0i32 as libc::c_float;
    (*bsptrace).sidenum = 0i32;
    (*bsptrace).contents = 0i32;
}
/*
==================
BotDrawDebugPolygons
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotDrawDebugPolygons(mut drawPoly:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  libc::c_int,
                                                                              _:
                                                                                  libc::c_int,
                                                                              _:
                                                                                  *mut libc::c_float)
                                                             -> ()>,
                                              mut value: libc::c_int) {
    static mut bot_debug: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    static mut bot_groundonly: *mut cvar_t =
        0 as *const cvar_t as *mut cvar_t;
    static mut bot_reachability: *mut cvar_t =
        0 as *const cvar_t as *mut cvar_t;
    static mut bot_highlightarea: *mut cvar_t =
        0 as *const cvar_t as *mut cvar_t;
    let mut poly: *mut bot_debugpoly_t = 0 as *mut bot_debugpoly_t;
    let mut i: libc::c_int = 0;
    let mut parm0: libc::c_int = 0;
    if debugpolygons.is_null() { return }
    if bot_debug.is_null() {
        bot_debug =
            Cvar_Get(b"bot_debug\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char, 0i32)
    }
    if 0 != bot_enable && 0 != (*bot_debug).integer {
        if bot_reachability.is_null() {
            bot_reachability =
                Cvar_Get(b"bot_reachability\x00" as *const u8 as
                             *const libc::c_char,
                         b"0\x00" as *const u8 as *const libc::c_char, 0i32)
        }
        if bot_groundonly.is_null() {
            bot_groundonly =
                Cvar_Get(b"bot_groundonly\x00" as *const u8 as
                             *const libc::c_char,
                         b"1\x00" as *const u8 as *const libc::c_char, 0i32)
        }
        if bot_highlightarea.is_null() {
            bot_highlightarea =
                Cvar_Get(b"bot_highlightarea\x00" as *const u8 as
                             *const libc::c_char,
                         b"0\x00" as *const u8 as *const libc::c_char, 0i32)
        }
        parm0 = 0i32;
        if 0 != (*svs.clients.offset(0isize)).lastUsercmd.buttons & 1i32 {
            parm0 |= 1i32
        }
        if 0 != (*bot_reachability).integer { parm0 |= 2i32 }
        if 0 != (*bot_groundonly).integer { parm0 |= 4i32 }
        (*botlib_export).BotLibVarSet.expect("non-null function pointer")(b"bot_highlightarea\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          (*bot_highlightarea).string);
        (*botlib_export).Test.expect("non-null function pointer")(parm0,
                                                                  0 as
                                                                      *mut libc::c_char,
                                                                  (*(*svs.clients.offset(0isize)).gentity).r.currentOrigin.as_mut_ptr(),
                                                                  (*(*svs.clients.offset(0isize)).gentity).r.currentAngles.as_mut_ptr());
    }
    i = 0i32;
    while i < bot_maxdebugpolys {
        poly = &mut *debugpolygons.offset(i as isize) as *mut bot_debugpoly_t;
        if !(0 == (*poly).inuse) {
            drawPoly.expect("non-null function pointer")((*poly).color,
                                                         (*poly).numPoints,
                                                         (*poly).points.as_mut_ptr()
                                                             as
                                                             *mut libc::c_float);
        }
        i += 1
    };
}