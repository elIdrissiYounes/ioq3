use libc;
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    use super::{libc};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
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
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Q_stricmpn(s1: *const libc::c_char, s2: *const libc::c_char,
                          n: libc::c_int) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        // removes color sequences from string
        #[no_mangle]
        pub fn Q_CleanStr(string: *mut libc::c_char) -> *mut libc::c_char;
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
    use super::q_shared_h::{qboolean, byte, cvar_t, fileHandle_t};
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
        pub fn NET_OutOfBandPrint(net_socket: netsrc_t, adr: netadr_t,
                                  format: *const libc::c_char, ...);
        #[no_mangle]
        pub fn NET_CompareBaseAdrMask(a: netadr_t, b: netadr_t,
                                      netmask: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_StringToAdr(s: *const libc::c_char, a: *mut netadr_t,
                               family: netadrtype_t) -> libc::c_int;
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        #[no_mangle]
        pub fn VM_ExplicitArgPtr(vm: *mut vm_t, intValue: intptr_t)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        // callback with each valid string
        #[no_mangle]
        pub fn Cmd_SetCommandCompletionFunc(command: *const libc::c_char,
                                            complete: completionFunc_t);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_Args() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgsFrom(arg: libc::c_int) -> *mut libc::c_char;
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
        // sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
        #[no_mangle]
        pub fn Cvar_SetLatched(var_name: *const libc::c_char,
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
        pub fn Cvar_InfoString(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_InfoString_Big(bit: libc::c_int) -> *mut libc::c_char;
        // will properly create any needed paths and deal with seperater character issues
        #[no_mangle]
        pub fn FS_SV_FOpenFileWrite(filename: *const libc::c_char)
         -> fileHandle_t;
        #[no_mangle]
        pub fn FS_SV_FOpenFileRead(filename: *const libc::c_char,
                                   fp: *mut fileHandle_t) -> libc::c_long;
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
        #[no_mangle]
        pub fn FS_ReadFile(qpath: *const libc::c_char,
                           buffer: *mut *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        pub fn FS_GetCurrentGameDir() -> *const libc::c_char;
        #[no_mangle]
        pub fn Field_CompleteFilename(dir: *const libc::c_char,
                                      ext: *const libc::c_char,
                                      stripExt: qboolean,
                                      allowNonPureFilesOnDisk: qboolean);
        #[no_mangle]
        pub fn Field_CompletePlayerName(names: *mut *const libc::c_char,
                                        count: libc::c_int);
        #[no_mangle]
        pub fn Info_Print(s: *const libc::c_char);
        #[no_mangle]
        pub fn Com_FieldStringToPlayerName(name: *mut libc::c_char,
                                           length: libc::c_int,
                                           rawname: *const libc::c_char)
         -> qboolean;
        #[no_mangle]
        pub fn Com_strCompare(a: *const libc::c_void, b: *const libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        pub static mut com_dedicated: *mut cvar_t;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        #[no_mangle]
        pub static mut com_standalone: *mut cvar_t;
        #[no_mangle]
        pub static mut com_frameTime: libc::c_int;
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn SV_Shutdown(finalmsg: *mut libc::c_char);
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
    pub type unnamed = libc::c_uint;
    // ConsoleCommand will be called when a command has been issued
	// that is not recognized as a builtin function.
	// The game can issue trap_argc() / trap_argv() commands to get the command
	// and parameters.  Return qfalse if the game doesn't recognize it as a command.
    // ( int time );
    pub const BOTAI_START_FRAME: unnamed = 10;
    // ( void );
    pub const GAME_CONSOLE_COMMAND: unnamed = 9;
    // ( int levelTime );
    pub const GAME_RUN_FRAME: unnamed = 8;
    // ( int clientNum );
    pub const GAME_CLIENT_THINK: unnamed = 7;
    // ( int clientNum );
    pub const GAME_CLIENT_COMMAND: unnamed = 6;
    // ( int clientNum );
    pub const GAME_CLIENT_DISCONNECT: unnamed = 5;
    // ( int clientNum );
    pub const GAME_CLIENT_USERINFO_CHANGED: unnamed = 4;
    // return NULL if the client is allowed to connect, otherwise return
	// a text string with the reason for denial
    // ( int clientNum );
    pub const GAME_CLIENT_BEGIN: unnamed = 3;
    // ( int clientNum, qboolean firstTime, qboolean isBot );
    pub const GAME_CLIENT_CONNECT: unnamed = 2;
    // init and shutdown will be called every single level
	// The game should call G_GET_ENTITY_TOKEN to parse through all the
	// entity configuration text and spawn gentities.
    // (void);
    pub const GAME_SHUTDOWN: unnamed = 1;
    // ( int levelTime, int randomSeed, int restart );
    pub const GAME_INIT: unnamed = 0;
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
    pub type unnamed_0 = libc::c_uint;
    pub const GT_MAX_GAME_TYPE: unnamed_0 = 8;
    pub const GT_HARVESTER: unnamed_0 = 7;
    pub const GT_OBELISK: unnamed_0 = 6;
    pub const GT_1FCTF: unnamed_0 = 5;
    // capture the flag
    pub const GT_CTF: unnamed_0 = 4;
    //-- team games go after this --
    // team deathmatch
    pub const GT_TEAM: unnamed_0 = 3;
    // single player ffa
    pub const GT_SINGLE_PLAYER: unnamed_0 = 2;
    // one on one tournament
    pub const GT_TOURNAMENT: unnamed_0 = 1;
    // free for all
    pub const GT_FFA: unnamed_0 = 0;
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
    use super::{libc};
    use super::q_shared_h::{byte, entityState_t, qboolean, playerState_t,
                            usercmd_t, fileHandle_t, cvar_t};
    use super::g_public_h::{sharedEntity_t};
    use super::qcommon_h::{msg_t, netchan_t, netadr_t, vm_t};
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
        pub static mut sv_maxclients: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_mapname: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_gametype: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_banFile: *mut cvar_t;
        #[no_mangle]
        pub static mut serverBans: [serverBan_t; 1024];
        #[no_mangle]
        pub static mut serverBansCount: libc::c_int;
        #[no_mangle]
        pub fn SV_SendServerCommand(cl: *mut client_t,
                                    fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn SV_SpawnServer(server: *mut libc::c_char, killBots: qboolean);
        #[no_mangle]
        pub fn SV_SectorList_f();
        #[no_mangle]
        pub fn SV_ClientEnterWorld(client: *mut client_t,
                                   cmd: *mut usercmd_t);
        #[no_mangle]
        pub fn SV_DropClient(drop_0: *mut client_t,
                             reason: *const libc::c_char);
        //
// sv_snapshot.c
//
        #[no_mangle]
        pub fn SV_AddServerCommand(client: *mut client_t,
                                   cmd: *const libc::c_char);
        #[no_mangle]
        pub fn SV_RestartGameProgs();
        //
// sv_init.c
//
        #[no_mangle]
        pub fn SV_SetConfigstring(index: libc::c_int,
                                  val: *const libc::c_char);
        #[no_mangle]
        pub fn SV_GameClientNum(num: libc::c_int) -> *mut playerState_t;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/sv_ccmds.c"]
pub mod sv_ccmds_c { }
use self::stddef_h::{size_t};
use self::stdlib_h::{__compar_fn_t, atoi, qsort};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cvar_s, cvar_t, playerState_s, playerState_t,
                       usercmd_s, usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, Q_IsColorString, Com_sprintf, Q_stricmp,
                       Q_stricmpn, Q_strncpyz, Q_CleanStr, va, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      vm_t, xcommand_t, completionFunc_t, vm_s,
                      NET_OutOfBandPrint, NET_CompareBaseAdrMask,
                      NET_AdrToString, NET_StringToAdr, VM_Call,
                      VM_ExplicitArgPtr, Cmd_AddCommand,
                      Cmd_SetCommandCompletionFunc, Cmd_Argc, Cmd_Argv,
                      Cmd_Args, Cmd_ArgsFrom, Cvar_Get, Cvar_Set,
                      Cvar_SetLatched, Cvar_SetValue, Cvar_VariableValue,
                      Cvar_VariableString, Cvar_InfoString,
                      Cvar_InfoString_Big, FS_SV_FOpenFileWrite,
                      FS_SV_FOpenFileRead, FS_Write, FS_Read, FS_FCloseFile,
                      FS_ReadFile, FS_GetCurrentGameDir,
                      Field_CompleteFilename, Field_CompletePlayerName,
                      Info_Print, Com_FieldStringToPlayerName, Com_strCompare,
                      com_dedicated, com_sv_running, com_standalone,
                      com_frameTime, Z_MallocDebug, Z_Free, SV_Shutdown};
use self::g_public_h::{entityShared_t, sharedEntity_t, unnamed,
                       BOTAI_START_FRAME, GAME_CONSOLE_COMMAND,
                       GAME_RUN_FRAME, GAME_CLIENT_THINK, GAME_CLIENT_COMMAND,
                       GAME_CLIENT_DISCONNECT, GAME_CLIENT_USERINFO_CHANGED,
                       GAME_CLIENT_BEGIN, GAME_CLIENT_CONNECT, GAME_SHUTDOWN,
                       GAME_INIT};
use self::bg_public_h::{unnamed_0, GT_MAX_GAME_TYPE, GT_HARVESTER, GT_OBELISK,
                        GT_1FCTF, GT_CTF, GT_TEAM, GT_SINGLE_PLAYER,
                        GT_TOURNAMENT, GT_FFA};
use self::server_h::{voipServerPacket_s, voipServerPacket_t, svEntity_s,
                     svEntity_t, serverState_t, SS_GAME, SS_LOADING, SS_DEAD,
                     server_t, clientSnapshot_t, clientState_t, CS_ACTIVE,
                     CS_PRIMED, CS_CONNECTED, CS_ZOMBIE, CS_FREE,
                     netchan_buffer_s, netchan_buffer_t, client_s, client_t,
                     challenge_t, serverStatic_t, serverBan_t, worldSector_s,
                     svs, sv, gvm, sv_maxclients, sv_mapname, sv_gametype,
                     sv_banFile, serverBans, serverBansCount,
                     SV_SendServerCommand, SV_SpawnServer, SV_SectorList_f,
                     SV_ClientEnterWorld, SV_DropClient, SV_AddServerCommand,
                     SV_RestartGameProgs, SV_SetConfigstring,
                     SV_GameClientNum};
use self::string_h::{memmove, strcpy, strcat, strchr, strlen};
use self::q_platform_h::{ShortSwap};
#[no_mangle]
pub unsafe extern "C" fn SV_AddOperatorCommands() {
    static mut initialized: qboolean = qfalse;
    if 0 != initialized as u64 { return }
    initialized = qtrue;
    Cmd_AddCommand(b"heartbeat\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Heartbeat_f));
    Cmd_AddCommand(b"kick\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Kick_f));
    if 0 == (*com_standalone).integer {
        Cmd_AddCommand(b"banUser\x00" as *const u8 as *const libc::c_char,
                       Some(SV_Ban_f));
        Cmd_AddCommand(b"banClient\x00" as *const u8 as *const libc::c_char,
                       Some(SV_BanNum_f));
    }
    Cmd_AddCommand(b"kickbots\x00" as *const u8 as *const libc::c_char,
                   Some(SV_KickBots_f));
    Cmd_AddCommand(b"kickall\x00" as *const u8 as *const libc::c_char,
                   Some(SV_KickAll_f));
    Cmd_AddCommand(b"kicknum\x00" as *const u8 as *const libc::c_char,
                   Some(SV_KickNum_f));
    Cmd_AddCommand(b"clientkick\x00" as *const u8 as *const libc::c_char,
                   Some(SV_KickNum_f));
    Cmd_AddCommand(b"status\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Status_f));
    Cmd_AddCommand(b"serverinfo\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Serverinfo_f));
    Cmd_AddCommand(b"systeminfo\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Systeminfo_f));
    Cmd_AddCommand(b"dumpuser\x00" as *const u8 as *const libc::c_char,
                   Some(SV_DumpUser_f));
    Cmd_AddCommand(b"map_restart\x00" as *const u8 as *const libc::c_char,
                   Some(SV_MapRestart_f));
    Cmd_AddCommand(b"sectorlist\x00" as *const u8 as *const libc::c_char,
                   Some(SV_SectorList_f));
    Cmd_AddCommand(b"map\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Map_f));
    Cmd_SetCommandCompletionFunc(b"map\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_CompleteMapName));
    Cmd_AddCommand(b"devmap\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Map_f));
    Cmd_SetCommandCompletionFunc(b"devmap\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_CompleteMapName));
    Cmd_AddCommand(b"spmap\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Map_f));
    Cmd_SetCommandCompletionFunc(b"spmap\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_CompleteMapName));
    Cmd_AddCommand(b"spdevmap\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Map_f));
    Cmd_SetCommandCompletionFunc(b"spdevmap\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_CompleteMapName));
    Cmd_AddCommand(b"killserver\x00" as *const u8 as *const libc::c_char,
                   Some(SV_KillServer_f));
    if 0 != (*com_dedicated).integer {
        Cmd_AddCommand(b"say\x00" as *const u8 as *const libc::c_char,
                       Some(SV_ConSay_f));
        Cmd_AddCommand(b"tell\x00" as *const u8 as *const libc::c_char,
                       Some(SV_ConTell_f));
        Cmd_AddCommand(b"sayto\x00" as *const u8 as *const libc::c_char,
                       Some(SV_ConSayto_f));
        Cmd_SetCommandCompletionFunc(b"sayto\x00" as *const u8 as
                                         *const libc::c_char,
                                     Some(SV_CompletePlayerName));
    }
    Cmd_AddCommand(b"rehashbans\x00" as *const u8 as *const libc::c_char,
                   Some(SV_RehashBans_f));
    Cmd_AddCommand(b"listbans\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ListBans_f));
    Cmd_AddCommand(b"banaddr\x00" as *const u8 as *const libc::c_char,
                   Some(SV_BanAddr_f));
    Cmd_AddCommand(b"exceptaddr\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ExceptAddr_f));
    Cmd_AddCommand(b"bandel\x00" as *const u8 as *const libc::c_char,
                   Some(SV_BanDel_f));
    Cmd_AddCommand(b"exceptdel\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ExceptDel_f));
    Cmd_AddCommand(b"flushbans\x00" as *const u8 as *const libc::c_char,
                   Some(SV_FlushBans_f));
}
/*
==================
SV_FlushBans_f

Delete all bans and exceptions.
==================
*/
unsafe extern "C" fn SV_FlushBans_f() {
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    serverBansCount = 0i32;
    SV_WriteBans();
    Com_Printf(b"All bans and exceptions have been deleted.\n\x00" as
                   *const u8 as *const libc::c_char);
}
/*
==================
SV_WriteBans

Save bans to file.
==================
*/
unsafe extern "C" fn SV_WriteBans() {
    let mut index: libc::c_int = 0;
    let mut writeto: fileHandle_t = 0;
    let mut filepath: [libc::c_char; 64] = [0; 64];
    if (*sv_banFile).string.is_null() || 0 == *(*sv_banFile).string { return }
    Com_sprintf(filepath.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/%s\x00" as *const u8 as *const libc::c_char,
                FS_GetCurrentGameDir(), (*sv_banFile).string);
    writeto = FS_SV_FOpenFileWrite(filepath.as_mut_ptr());
    if 0 != writeto {
        let mut writebuf: [libc::c_char; 128] = [0; 128];
        let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
        index = 0i32;
        while index < serverBansCount {
            curban = &mut serverBans[index as usize] as *mut serverBan_t;
            Com_sprintf(writebuf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as
                            libc::c_ulong as libc::c_int,
                        b"%d %s %d\n\x00" as *const u8 as *const libc::c_char,
                        (*curban).isexception as libc::c_uint,
                        NET_AdrToString((*curban).ip), (*curban).subnet);
            FS_Write(writebuf.as_mut_ptr() as *const libc::c_void,
                     strlen(writebuf.as_mut_ptr()) as libc::c_int, writeto);
            index += 1
        }
        FS_FCloseFile(writeto);
    };
}
unsafe extern "C" fn SV_ExceptDel_f() { SV_DelBanFromList(qtrue); }
/*
==================
SV_DelBanFromList

Remove a ban or an exception from the list.
==================
*/
unsafe extern "C" fn SV_DelBanFromList(mut isexception: qboolean) {
    let mut index: libc::c_int = 0;
    let mut count: libc::c_int = 0i32;
    let mut todel: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut ip: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut banstring: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: %s (ip[/subnet] | num)\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(0i32));
        return
    }
    banstring = Cmd_Argv(1i32);
    if !strchr(banstring, '.' as i32).is_null() ||
           !strchr(banstring, ':' as i32).is_null() {
        let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
        if 0 != SV_ParseCIDRNotation(&mut ip, &mut mask, banstring) as u64 {
            Com_Printf(b"Error: Invalid address %s\n\x00" as *const u8 as
                           *const libc::c_char, banstring);
            return
        }
        index = 0i32;
        while index < serverBansCount {
            curban = &mut serverBans[index as usize] as *mut serverBan_t;
            if (*curban).isexception as libc::c_uint ==
                   isexception as libc::c_uint && (*curban).subnet >= mask &&
                   0 !=
                       NET_CompareBaseAdrMask((*curban).ip, ip, mask) as
                           libc::c_uint {
                Com_Printf(b"Deleting %s %s/%d\n\x00" as *const u8 as
                               *const libc::c_char,
                           if 0 != isexception as libc::c_uint {
                               b"exception\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"ban\x00" as *const u8 as *const libc::c_char
                           }, NET_AdrToString((*curban).ip),
                           (*curban).subnet);
                SV_DelBanEntryFromList(index);
            } else { index += 1 }
        }
    } else {
        todel = atoi(Cmd_Argv(1i32));
        if todel < 1i32 || todel > serverBansCount {
            Com_Printf(b"Error: Invalid ban number given\n\x00" as *const u8
                           as *const libc::c_char);
            return
        }
        index = 0i32;
        while index < serverBansCount {
            if serverBans[index as usize].isexception as libc::c_uint ==
                   isexception as libc::c_uint {
                count += 1;
                if count == todel {
                    Com_Printf(b"Deleting %s %s/%d\n\x00" as *const u8 as
                                   *const libc::c_char,
                               if 0 != isexception as libc::c_uint {
                                   b"exception\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"ban\x00" as *const u8 as
                                       *const libc::c_char
                               },
                               NET_AdrToString(serverBans[index as usize].ip),
                               serverBans[index as usize].subnet);
                    SV_DelBanEntryFromList(index);
                    break ;
                }
            }
            index += 1
        }
    }
    SV_WriteBans();
}
/*
==================
SV_DelBanEntryFromList

Remove a ban or an exception from the list.
==================
*/
unsafe extern "C" fn SV_DelBanEntryFromList(mut index: libc::c_int)
 -> qboolean {
    if index == serverBansCount - 1i32 {
        serverBansCount -= 1
    } else if (index as libc::c_ulong) <
                  (::std::mem::size_of::<[serverBan_t; 1024]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<serverBan_t>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong)
     {
        memmove(serverBans.as_mut_ptr().offset(index as isize) as
                    *mut libc::c_void,
                serverBans.as_mut_ptr().offset(index as isize).offset(1isize)
                    as *const libc::c_void,
                ((serverBansCount - index - 1i32) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<serverBan_t>()
                                                     as libc::c_ulong));
        serverBansCount -= 1
    } else { return qtrue }
    return qfalse;
}
/*
==================
SV_ParseCIDRNotation

Parse a CIDR notation type string and return a netadr_t and suffix by reference
==================
*/
unsafe extern "C" fn SV_ParseCIDRNotation(mut dest: *mut netadr_t,
                                          mut mask: *mut libc::c_int,
                                          mut adrstr: *mut libc::c_char)
 -> qboolean {
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    suffix = strchr(adrstr, '/' as i32);
    if !suffix.is_null() {
        *suffix = '\u{0}' as i32 as libc::c_char;
        suffix = suffix.offset(1isize)
    }
    if 0 == NET_StringToAdr(adrstr, dest, NA_UNSPEC) { return qtrue }
    if !suffix.is_null() {
        *mask = atoi(suffix);
        if (*dest).type_0 as libc::c_uint ==
               NA_IP as libc::c_int as libc::c_uint {
            if *mask < 1i32 || *mask > 32i32 { *mask = 32i32 }
        } else if *mask < 1i32 || *mask > 128i32 { *mask = 128i32 }
    } else if (*dest).type_0 as libc::c_uint ==
                  NA_IP as libc::c_int as libc::c_uint {
        *mask = 32i32
    } else { *mask = 128i32 }
    return qfalse;
}
unsafe extern "C" fn SV_BanDel_f() { SV_DelBanFromList(qfalse); }
unsafe extern "C" fn SV_ExceptAddr_f() { SV_AddBanToList(qtrue); }
/*
==================
SV_AddBanToList

Ban a user from being able to play on this server based on his ip address.
==================
*/
unsafe extern "C" fn SV_AddBanToList(mut isexception: qboolean) {
    let mut banstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addy2: [libc::c_char; 48] = [0; 48];
    let mut ip: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut index: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut curban: *mut serverBan_t = 0 as *mut serverBan_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    argc = Cmd_Argc();
    if argc < 2i32 || argc > 3i32 {
        Com_Printf(b"Usage: %s (ip[/subnet] | clientnum [subnet])\n\x00" as
                       *const u8 as *const libc::c_char, Cmd_Argv(0i32));
        return
    }
    if serverBansCount as libc::c_ulong >=
           (::std::mem::size_of::<[serverBan_t; 1024]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<serverBan_t>()
                                                as libc::c_ulong) {
        Com_Printf(b"Error: Maximum number of bans/exceptions exceeded.\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    banstring = Cmd_Argv(1i32);
    if !strchr(banstring, '.' as i32).is_null() ||
           !strchr(banstring, ':' as i32).is_null() {
        if 0 != SV_ParseCIDRNotation(&mut ip, &mut mask, banstring) as u64 {
            Com_Printf(b"Error: Invalid address %s\n\x00" as *const u8 as
                           *const libc::c_char, banstring);
            return
        }
    } else {
        let mut cl: *mut client_t = 0 as *mut client_t;
        cl = SV_GetPlayerByNum();
        if cl.is_null() {
            Com_Printf(b"Error: Playernum %s does not exist.\n\x00" as
                           *const u8 as *const libc::c_char, Cmd_Argv(1i32));
            return
        }
        ip = (*cl).netchan.remoteAddress;
        if argc == 3i32 {
            mask = atoi(Cmd_Argv(2i32));
            if ip.type_0 as libc::c_uint ==
                   NA_IP as libc::c_int as libc::c_uint {
                if mask < 1i32 || mask > 32i32 { mask = 32i32 }
            } else if mask < 1i32 || mask > 128i32 { mask = 128i32 }
        } else {
            mask =
                if ip.type_0 as libc::c_uint ==
                       NA_IP6 as libc::c_int as libc::c_uint {
                    128i32
                } else { 32i32 }
        }
    }
    if ip.type_0 as libc::c_uint != NA_IP as libc::c_int as libc::c_uint &&
           ip.type_0 as libc::c_uint != NA_IP6 as libc::c_int as libc::c_uint
       {
        Com_Printf(b"Error: Can ban players connected via the internet only.\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    index = 0i32;
    while index < serverBansCount {
        curban = &mut serverBans[index as usize] as *mut serverBan_t;
        if (*curban).subnet <= mask {
            if (0 != (*curban).isexception as libc::c_uint ||
                    0 == isexception as u64) &&
                   0 !=
                       NET_CompareBaseAdrMask((*curban).ip, ip,
                                              (*curban).subnet) as
                           libc::c_uint {
                Q_strncpyz(addy2.as_mut_ptr(), NET_AdrToString(ip),
                           ::std::mem::size_of::<[libc::c_char; 48]>() as
                               libc::c_ulong as libc::c_int);
                Com_Printf(b"Error: %s %s/%d supersedes %s %s/%d\n\x00" as
                               *const u8 as *const libc::c_char,
                           if 0 != (*curban).isexception as libc::c_uint {
                               b"Exception\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"Ban\x00" as *const u8 as *const libc::c_char
                           }, NET_AdrToString((*curban).ip), (*curban).subnet,
                           if 0 != isexception as libc::c_uint {
                               b"exception\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"ban\x00" as *const u8 as *const libc::c_char
                           }, addy2.as_mut_ptr(), mask);
                return
            }
        }
        if (*curban).subnet >= mask {
            if 0 == (*curban).isexception as u64 &&
                   0 != isexception as libc::c_uint &&
                   0 !=
                       NET_CompareBaseAdrMask((*curban).ip, ip, mask) as
                           libc::c_uint {
                Q_strncpyz(addy2.as_mut_ptr(), NET_AdrToString((*curban).ip),
                           ::std::mem::size_of::<[libc::c_char; 48]>() as
                               libc::c_ulong as libc::c_int);
                Com_Printf(b"Error: %s %s/%d supersedes already existing %s %s/%d\n\x00"
                               as *const u8 as *const libc::c_char,
                           if 0 != isexception as libc::c_uint {
                               b"Exception\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"Ban\x00" as *const u8 as *const libc::c_char
                           }, NET_AdrToString(ip), mask,
                           if 0 != (*curban).isexception as libc::c_uint {
                               b"exception\x00" as *const u8 as
                                   *const libc::c_char
                           } else {
                               b"ban\x00" as *const u8 as *const libc::c_char
                           }, addy2.as_mut_ptr(), (*curban).subnet);
                return
            }
        }
        index += 1
    }
    index = 0i32;
    while index < serverBansCount {
        curban = &mut serverBans[index as usize] as *mut serverBan_t;
        if (*curban).subnet > mask &&
               (0 == (*curban).isexception as u64 ||
                    0 != isexception as libc::c_uint) &&
               0 !=
                   NET_CompareBaseAdrMask((*curban).ip, ip, mask) as
                       libc::c_uint {
            SV_DelBanEntryFromList(index);
        } else { index += 1 }
    }
    serverBans[serverBansCount as usize].ip = ip;
    serverBans[serverBansCount as usize].subnet = mask;
    serverBans[serverBansCount as usize].isexception = isexception;
    serverBansCount += 1;
    SV_WriteBans();
    Com_Printf(b"Added %s: %s/%d\n\x00" as *const u8 as *const libc::c_char,
               if 0 != isexception as libc::c_uint {
                   b"ban exception\x00" as *const u8 as *const libc::c_char
               } else { b"ban\x00" as *const u8 as *const libc::c_char },
               NET_AdrToString(ip), mask);
}
/*
==================
SV_GetPlayerByNum

Returns the player with idnum from Cmd_Argv(1)
==================
*/
unsafe extern "C" fn SV_GetPlayerByNum() -> *mut client_t {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    let mut idnum: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == (*com_sv_running).integer { return 0 as *mut client_t }
    if Cmd_Argc() < 2i32 {
        Com_Printf(b"No player specified.\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as *mut client_t
    }
    s = Cmd_Argv(1i32);
    i = 0i32;
    while 0 != *s.offset(i as isize) {
        if (*s.offset(i as isize) as libc::c_int) < '0' as i32 ||
               *s.offset(i as isize) as libc::c_int > '9' as i32 {
            Com_Printf(b"Bad slot number: %s\n\x00" as *const u8 as
                           *const libc::c_char, s);
            return 0 as *mut client_t
        }
        i += 1
    }
    idnum = atoi(s);
    if idnum < 0i32 || idnum >= (*sv_maxclients).integer {
        Com_Printf(b"Bad client slot: %i\n\x00" as *const u8 as
                       *const libc::c_char, idnum);
        return 0 as *mut client_t
    }
    cl = &mut *svs.clients.offset(idnum as isize) as *mut client_t;
    if 0 == (*cl).state as u64 {
        Com_Printf(b"Client %i is not active\n\x00" as *const u8 as
                       *const libc::c_char, idnum);
        return 0 as *mut client_t
    }
    return cl;
}
unsafe extern "C" fn SV_BanAddr_f() { SV_AddBanToList(qfalse); }
/*
==================
SV_ListBans_f

List all bans and exceptions on console
==================
*/
unsafe extern "C" fn SV_ListBans_f() {
    let mut index: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut ban: *mut serverBan_t = 0 as *mut serverBan_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    count = 0i32;
    index = count;
    while index < serverBansCount {
        ban = &mut serverBans[index as usize] as *mut serverBan_t;
        if 0 == (*ban).isexception as u64 {
            count += 1;
            Com_Printf(b"Ban #%d: %s/%d\n\x00" as *const u8 as
                           *const libc::c_char, count,
                       NET_AdrToString((*ban).ip), (*ban).subnet);
        }
        index += 1
    }
    count = 0i32;
    index = count;
    while index < serverBansCount {
        ban = &mut serverBans[index as usize] as *mut serverBan_t;
        if 0 != (*ban).isexception as u64 {
            count += 1;
            Com_Printf(b"Except #%d: %s/%d\n\x00" as *const u8 as
                           *const libc::c_char, count,
                       NET_AdrToString((*ban).ip), (*ban).subnet);
        }
        index += 1
    };
}
/*
==================
SV_RehashBans_f

Load saved bans from file.
==================
*/
unsafe extern "C" fn SV_RehashBans_f() {
    let mut index: libc::c_int = 0;
    let mut filelen: libc::c_int = 0;
    let mut readfrom: fileHandle_t = 0;
    let mut textbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut curpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maskpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newlinepos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filepath: [libc::c_char; 64] = [0; 64];
    if 0 == (*com_sv_running).integer { return }
    serverBansCount = 0i32;
    if (*sv_banFile).string.is_null() || 0 == *(*sv_banFile).string { return }
    Com_sprintf(filepath.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/%s\x00" as *const u8 as *const libc::c_char,
                FS_GetCurrentGameDir(), (*sv_banFile).string);
    filelen =
        FS_SV_FOpenFileRead(filepath.as_mut_ptr(), &mut readfrom) as
            libc::c_int;
    if filelen >= 0i32 {
        if filelen < 2i32 { FS_FCloseFile(readfrom); return }
        textbuf =
            Z_MallocDebug(filelen,
                          b"filelen\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                          b"code/server/sv_ccmds.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          663i32) as *mut libc::c_char;
        curpos = textbuf;
        filelen = FS_Read(textbuf as *mut libc::c_void, filelen, readfrom);
        FS_FCloseFile(readfrom);
        endpos = textbuf.offset(filelen as isize);
        index = 0i32;
        while index < 1024i32 && curpos.offset(2isize) < endpos {
            maskpos = curpos.offset(2isize);
            while maskpos < endpos && *maskpos as libc::c_int != ' ' as i32 {
                maskpos = maskpos.offset(1isize)
            }
            if maskpos.offset(1isize) >= endpos { break ; }
            *maskpos = '\u{0}' as i32 as libc::c_char;
            maskpos = maskpos.offset(1isize);
            newlinepos = maskpos;
            while newlinepos < endpos &&
                      *newlinepos as libc::c_int != '\n' as i32 {
                newlinepos = newlinepos.offset(1isize)
            }
            if newlinepos >= endpos { break ; }
            *newlinepos = '\u{0}' as i32 as libc::c_char;
            if 0 !=
                   NET_StringToAdr(curpos.offset(2isize),
                                   &mut serverBans[index as usize].ip,
                                   NA_UNSPEC) {
                serverBans[index as usize].isexception =
                    (*curpos.offset(0isize) as libc::c_int != '0' as i32) as
                        libc::c_int as qboolean;
                serverBans[index as usize].subnet = atoi(maskpos);
                if serverBans[index as usize].ip.type_0 as libc::c_uint ==
                       NA_IP as libc::c_int as libc::c_uint &&
                       (serverBans[index as usize].subnet < 1i32 ||
                            serverBans[index as usize].subnet > 32i32) {
                    serverBans[index as usize].subnet = 32i32
                } else if serverBans[index as usize].ip.type_0 as libc::c_uint
                              == NA_IP6 as libc::c_int as libc::c_uint &&
                              (serverBans[index as usize].subnet < 1i32 ||
                                   serverBans[index as usize].subnet > 128i32)
                 {
                    serverBans[index as usize].subnet = 128i32
                }
            }
            curpos = newlinepos.offset(1isize);
            index += 1
        }
        serverBansCount = index;
        Z_Free(textbuf as *mut libc::c_void);
    };
}
/*
==================
SV_CompletePlayerName
==================
*/
unsafe extern "C" fn SV_CompletePlayerName(mut args: *mut libc::c_char,
                                           mut argNum: libc::c_int) {
    if argNum == 2i32 {
        let mut names: [[libc::c_char; 32]; 64] = [[0; 32]; 64];
        let mut namesPtr: [*const libc::c_char; 64] =
            [0 as *const libc::c_char; 64];
        let mut cl: *mut client_t = 0 as *mut client_t;
        let mut i: libc::c_int = 0;
        let mut nameCount: libc::c_int = 0;
        let mut clientCount: libc::c_int = 0;
        nameCount = 0i32;
        clientCount = (*sv_maxclients).integer;
        i = 0i32;
        cl = svs.clients;
        while i < clientCount {
            if !(0 == (*cl).state as u64) {
                if i >= 64i32 { break ; }
                Q_strncpyz(names[nameCount as usize].as_mut_ptr(),
                           (*cl).name.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 32]>() as
                               libc::c_ulong as libc::c_int);
                Q_CleanStr(names[nameCount as usize].as_mut_ptr());
                namesPtr[nameCount as usize] =
                    names[nameCount as usize].as_mut_ptr();
                nameCount += 1
            }
            i += 1;
            cl = cl.offset(1isize)
        }
        qsort(namesPtr.as_mut_ptr() as *mut libc::c_void, nameCount as size_t,
              ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
              Some(Com_strCompare));
        Field_CompletePlayerName(namesPtr.as_mut_ptr(), nameCount);
    };
}
/*
==================
SV_ConSayto_f
==================
*/
unsafe extern "C" fn SV_ConSayto_f() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut rawname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut cleanName: [libc::c_char; 32] = [0; 32];
    let mut saytocl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() < 3i32 {
        Com_Printf(b"Usage: sayto <player name> <text>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    rawname = Cmd_Argv(1i32);
    Com_FieldStringToPlayerName(name.as_mut_ptr(), 32i32, rawname);
    saytocl = 0 as *mut client_t;
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !(0 == (*cl).state as u64) {
            Q_strncpyz(cleanName.as_mut_ptr(), (*cl).name.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong as libc::c_int);
            Q_CleanStr(cleanName.as_mut_ptr());
            if 0 == Q_stricmp(cleanName.as_mut_ptr(), name.as_mut_ptr()) {
                saytocl = cl;
                break ;
            }
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    if saytocl.is_null() {
        Com_Printf(b"No such player name: %s.\n\x00" as *const u8 as
                       *const libc::c_char, name.as_mut_ptr());
        return
    }
    strcpy(text.as_mut_ptr(),
           b"console_sayto: \x00" as *const u8 as *const libc::c_char);
    p = Cmd_ArgsFrom(2i32);
    if *p as libc::c_int == '\"' as i32 {
        p = p.offset(1isize);
        *p.offset(strlen(p).wrapping_sub(1i32 as libc::c_ulong) as isize) =
            0i32 as libc::c_char
    }
    strcat(text.as_mut_ptr(), p);
    Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
               text.as_mut_ptr());
    SV_SendServerCommand(saytocl,
                         b"chat \"%s\"\x00" as *const u8 as
                             *const libc::c_char, text.as_mut_ptr());
}
/*
==================
SV_ConTell_f
==================
*/
unsafe extern "C" fn SV_ConTell_f() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut cl: *mut client_t = 0 as *mut client_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() < 3i32 {
        Com_Printf(b"Usage: tell <client number> <text>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cl = SV_GetPlayerByNum();
    if cl.is_null() { return }
    strcpy(text.as_mut_ptr(),
           b"console_tell: \x00" as *const u8 as *const libc::c_char);
    p = Cmd_ArgsFrom(2i32);
    if *p as libc::c_int == '\"' as i32 {
        p = p.offset(1isize);
        *p.offset(strlen(p).wrapping_sub(1i32 as libc::c_ulong) as isize) =
            0i32 as libc::c_char
    }
    strcat(text.as_mut_ptr(), p);
    Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
               text.as_mut_ptr());
    SV_SendServerCommand(cl,
                         b"chat \"%s\"\x00" as *const u8 as
                             *const libc::c_char, text.as_mut_ptr());
}
/*
==================
SV_ConSay_f
==================
*/
unsafe extern "C" fn SV_ConSay_f() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() < 2i32 { return }
    strcpy(text.as_mut_ptr(),
           b"console: \x00" as *const u8 as *const libc::c_char);
    p = Cmd_Args();
    if *p as libc::c_int == '\"' as i32 {
        p = p.offset(1isize);
        *p.offset(strlen(p).wrapping_sub(1i32 as libc::c_ulong) as isize) =
            0i32 as libc::c_char
    }
    strcat(text.as_mut_ptr(), p);
    Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
               text.as_mut_ptr());
    SV_SendServerCommand(0 as *mut client_t,
                         b"chat \"%s\"\x00" as *const u8 as
                             *const libc::c_char, text.as_mut_ptr());
}
/*
=================
SV_KillServer
=================
*/
unsafe extern "C" fn SV_KillServer_f() {
    SV_Shutdown(b"killserver\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
}
//===========================================================
/*
==================
SV_CompleteMapName
==================
*/
unsafe extern "C" fn SV_CompleteMapName(mut args: *mut libc::c_char,
                                        mut argNum: libc::c_int) {
    if argNum == 2i32 {
        Field_CompleteFilename(b"maps\x00" as *const u8 as
                                   *const libc::c_char,
                               b"bsp\x00" as *const u8 as *const libc::c_char,
                               qtrue, qfalse);
    };
}
//=========================================================
/*
==================
SV_Map_f

Restart the server on a different map
==================
*/
unsafe extern "C" fn SV_Map_f() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut killBots: qboolean = qfalse;
    let mut cheat: qboolean = qfalse;
    let mut expanded: [libc::c_char; 64] = [0; 64];
    let mut mapname: [libc::c_char; 64] = [0; 64];
    map = Cmd_Argv(1i32);
    if map.is_null() { return }
    Com_sprintf(expanded.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char, map);
    if FS_ReadFile(expanded.as_mut_ptr(), 0 as *mut *mut libc::c_void) ==
           -1i32 as libc::c_long {
        Com_Printf(b"Can\'t find map %s\n\x00" as *const u8 as
                       *const libc::c_char, expanded.as_mut_ptr());
        return
    }
    Cvar_Get(b"g_gametype\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char,
             0x4i32 | 0x2i32 | 0x20i32);
    cmd = Cmd_Argv(0i32);
    if Q_stricmpn(cmd, b"sp\x00" as *const u8 as *const libc::c_char, 2i32) ==
           0i32 {
        Cvar_SetValue(b"g_gametype\x00" as *const u8 as *const libc::c_char,
                      GT_SINGLE_PLAYER as libc::c_int as libc::c_float);
        Cvar_SetValue(b"g_doWarmup\x00" as *const u8 as *const libc::c_char,
                      0i32 as libc::c_float);
        Cvar_SetLatched(b"sv_maxclients\x00" as *const u8 as
                            *const libc::c_char,
                        b"8\x00" as *const u8 as *const libc::c_char);
        cmd = cmd.offset(2isize);
        if 0 ==
               Q_stricmp(cmd,
                         b"devmap\x00" as *const u8 as *const libc::c_char) {
            cheat = qtrue
        } else { cheat = qfalse }
        killBots = qtrue
    } else {
        if 0 ==
               Q_stricmp(cmd,
                         b"devmap\x00" as *const u8 as *const libc::c_char) {
            cheat = qtrue;
            killBots = qtrue
        } else { cheat = qfalse; killBots = qfalse }
        if (*sv_gametype).integer == GT_SINGLE_PLAYER as libc::c_int {
            Cvar_SetValue(b"g_gametype\x00" as *const u8 as
                              *const libc::c_char,
                          GT_FFA as libc::c_int as libc::c_float);
        }
    }
    Q_strncpyz(mapname.as_mut_ptr(), map,
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    SV_SpawnServer(mapname.as_mut_ptr(), killBots);
    if 0 != cheat as u64 {
        Cvar_Set(b"sv_cheats\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char);
    } else {
        Cvar_Set(b"sv_cheats\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
    };
}
/*
================
SV_MapRestart_f

Completely restarts a level, but doesn't send a new gamestate to the clients.
This allows fair starts with variable load times.
================
*/
unsafe extern "C" fn SV_MapRestart_f() {
    let mut i: libc::c_int = 0;
    let mut client: *mut client_t = 0 as *mut client_t;
    let mut denied: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut isBot: qboolean = qfalse;
    let mut delay: libc::c_int = 0;
    if com_frameTime == sv.serverId { return }
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if 0 != sv.restartTime { return }
    if Cmd_Argc() > 1i32 {
        delay = atoi(Cmd_Argv(1i32))
    } else { delay = 5i32 }
    if 0 != delay &&
           0. ==
               Cvar_VariableValue(b"g_doWarmup\x00" as *const u8 as
                                      *const libc::c_char) {
        sv.restartTime = sv.time + delay * 1000i32;
        SV_SetConfigstring(5i32,
                           va(b"%i\x00" as *const u8 as *const libc::c_char as
                                  *mut libc::c_char, sv.restartTime));
        return
    }
    if 0 != (*sv_maxclients).modified as libc::c_uint ||
           0 != (*sv_gametype).modified as libc::c_uint {
        let mut mapname: [libc::c_char; 64] = [0; 64];
        Com_Printf(b"variable change -- restarting.\n\x00" as *const u8 as
                       *const libc::c_char);
        Q_strncpyz(mapname.as_mut_ptr(),
                   Cvar_VariableString(b"mapname\x00" as *const u8 as
                                           *const libc::c_char),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
        SV_SpawnServer(mapname.as_mut_ptr(), qfalse);
        return
    }
    svs.snapFlagServerBit ^= 4i32;
    sv.serverId = com_frameTime;
    Cvar_Set(b"sv_serverid\x00" as *const u8 as *const libc::c_char,
             va(b"%i\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char, sv.serverId));
    i = 0i32;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint ==
               CS_PRIMED as libc::c_int as libc::c_uint {
            (*svs.clients.offset(i as isize)).oldServerTime = sv.restartTime
        }
        i += 1
    }
    sv.state = SS_LOADING;
    sv.restarting = qtrue;
    SV_RestartGameProgs();
    i = 0i32;
    while i < 3i32 {
        VM_Call(gvm, GAME_RUN_FRAME as libc::c_int, sv.time);
        sv.time += 100i32;
        svs.time += 100i32;
        i += 1
    }
    sv.state = SS_GAME;
    sv.restarting = qfalse;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        client = &mut *svs.clients.offset(i as isize) as *mut client_t;
        // send the new gamestate to all connected clients
        if !(((*client).state as libc::c_uint) <
                 CS_CONNECTED as libc::c_int as libc::c_uint) {
            if (*client).netchan.remoteAddress.type_0 as libc::c_uint ==
                   NA_BOT as libc::c_int as libc::c_uint {
                isBot = qtrue
            } else { isBot = qfalse }
            SV_AddServerCommand(client,
                                b"map_restart\n\x00" as *const u8 as
                                    *const libc::c_char);
            denied =
                VM_ExplicitArgPtr(gvm,
                                  VM_Call(gvm,
                                          GAME_CLIENT_CONNECT as libc::c_int,
                                          i, qfalse as libc::c_int,
                                          isBot as libc::c_uint)) as
                    *mut libc::c_char;
            if !denied.is_null() {
                SV_DropClient(client, denied);
                Com_Printf(b"SV_MapRestart_f(%d): dropped client %i - denied!\n\x00"
                               as *const u8 as *const libc::c_char, delay, i);
            } else if (*client).state as libc::c_uint ==
                          CS_ACTIVE as libc::c_int as libc::c_uint {
                SV_ClientEnterWorld(client, &mut (*client).lastUsercmd);
            } else { SV_ClientEnterWorld(client, 0 as *mut usercmd_t); }
        }
        i += 1
    }
    VM_Call(gvm, GAME_RUN_FRAME as libc::c_int, sv.time);
    sv.time += 100i32;
    svs.time += 100i32;
}
/*
===========
SV_DumpUser_f

Examine all a users info strings
===========
*/
unsafe extern "C" fn SV_DumpUser_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: dumpuser <userid>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cl = SV_GetPlayerByHandle();
    if cl.is_null() { return }
    Com_Printf(b"userinfo\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"--------\n\x00" as *const u8 as *const libc::c_char);
    Info_Print((*cl).userinfo.as_mut_ptr());
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
===============================================================================

OPERATOR CONSOLE ONLY COMMANDS

These commands can only be entered from stdin or by a remote operator datagram
===============================================================================
*/
/*
==================
SV_GetPlayerByHandle

Returns the player with player id or name from Cmd_Argv(1)
==================
*/
unsafe extern "C" fn SV_GetPlayerByHandle() -> *mut client_t {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cleanName: [libc::c_char; 64] = [0; 64];
    if 0 == (*com_sv_running).integer { return 0 as *mut client_t }
    if Cmd_Argc() < 2i32 {
        Com_Printf(b"No player specified.\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as *mut client_t
    }
    s = Cmd_Argv(1i32);
    i = 0i32;
    while *s.offset(i as isize) as libc::c_int >= '0' as i32 &&
              *s.offset(i as isize) as libc::c_int <= '9' as i32 {
        i += 1
    }
    if 0 == *s.offset(i as isize) {
        let mut plid: libc::c_int = atoi(s);
        if plid >= 0i32 && plid < (*sv_maxclients).integer {
            cl = &mut *svs.clients.offset(plid as isize) as *mut client_t;
            if 0 != (*cl).state as u64 { return cl }
        }
    }
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !(0 == (*cl).state as u64) {
            if 0 == Q_stricmp((*cl).name.as_mut_ptr(), s) { return cl }
            Q_strncpyz(cleanName.as_mut_ptr(), (*cl).name.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong as libc::c_int);
            Q_CleanStr(cleanName.as_mut_ptr());
            if 0 == Q_stricmp(cleanName.as_mut_ptr(), s) { return cl }
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    Com_Printf(b"Player %s is not on the server\n\x00" as *const u8 as
                   *const libc::c_char, s);
    return 0 as *mut client_t;
}
/*
===========
SV_Systeminfo_f

Examine the systeminfo string
===========
*/
unsafe extern "C" fn SV_Systeminfo_f() {
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Com_Printf(b"System info settings:\n\x00" as *const u8 as
                   *const libc::c_char);
    Info_Print(Cvar_InfoString_Big(0x8i32));
}
/*
===========
SV_Serverinfo_f

Examine the serverinfo string
===========
*/
unsafe extern "C" fn SV_Serverinfo_f() {
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Com_Printf(b"Server info settings:\n\x00" as *const u8 as
                   *const libc::c_char);
    Info_Print(Cvar_InfoString(0x4i32));
}
/*
================
SV_Status_f
================
*/
unsafe extern "C" fn SV_Status_f() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ping: libc::c_int = 0;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Com_Printf(b"map: %s\n\x00" as *const u8 as *const libc::c_char,
               (*sv_mapname).string);
    Com_Printf(b"cl score ping name            address                                 rate \n\x00"
                   as *const u8 as *const libc::c_char);
    Com_Printf(b"-- ----- ---- --------------- --------------------------------------- -----\n\x00"
                   as *const u8 as *const libc::c_char);
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !(0 == (*cl).state as u64) {
            Com_Printf(b"%2i \x00" as *const u8 as *const libc::c_char, i);
            ps = SV_GameClientNum(i);
            Com_Printf(b"%5i \x00" as *const u8 as *const libc::c_char,
                       (*ps).persistant[0usize]);
            if (*cl).state as libc::c_uint ==
                   CS_CONNECTED as libc::c_int as libc::c_uint {
                Com_Printf(b"CON \x00" as *const u8 as *const libc::c_char);
            } else if (*cl).state as libc::c_uint ==
                          CS_ZOMBIE as libc::c_int as libc::c_uint {
                Com_Printf(b"ZMB \x00" as *const u8 as *const libc::c_char);
            } else {
                ping =
                    if (*cl).ping < 9999i32 { (*cl).ping } else { 9999i32 };
                Com_Printf(b"%4i \x00" as *const u8 as *const libc::c_char,
                           ping);
            }
            Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       (*cl).name.as_mut_ptr());
            l = 16i32 - SV_Strlen((*cl).name.as_mut_ptr());
            j = 0i32;
            loop  {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
                j += 1;
                if !(j < l) { break ; }
            }
            s = NET_AdrToString((*cl).netchan.remoteAddress);
            Com_Printf(b"^7%s\x00" as *const u8 as *const libc::c_char, s);
            l =
                (39i32 as libc::c_ulong).wrapping_sub(strlen(s)) as
                    libc::c_int;
            j = 0i32;
            loop  {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
                j += 1;
                if !(j < l) { break ; }
            }
            Com_Printf(b" %5i\x00" as *const u8 as *const libc::c_char,
                       (*cl).rate);
            Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        i += 1;
        cl = cl.offset(1isize)
    }
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
** SV_Strlen -- skips color escape codes
*/
unsafe extern "C" fn SV_Strlen(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = str;
    let mut count: libc::c_int = 0i32;
    while 0 != *s {
        if 0 != Q_IsColorString(s) as u64 {
            s = s.offset(2isize)
        } else { count += 1; s = s.offset(1isize) }
    }
    return count;
}
/*
==================
SV_KickNum_f

Kick a user off of the server
==================
*/
unsafe extern "C" fn SV_KickNum_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: %s <client number>\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(0i32));
        return
    }
    cl = SV_GetPlayerByNum();
    if cl.is_null() { return }
    if (*cl).netchan.remoteAddress.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        Com_Printf(b"Cannot kick host player\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    SV_DropClient(cl, b"was kicked\x00" as *const u8 as *const libc::c_char);
    (*cl).lastPacketTime = svs.time;
}
/*
==================
SV_KickAll_f

Kick all users off of the server
==================
*/
unsafe extern "C" fn SV_KickAll_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !(0 == (*cl).state as u64) {
            if !((*cl).netchan.remoteAddress.type_0 as libc::c_uint ==
                     NA_LOOPBACK as libc::c_int as libc::c_uint) {
                SV_DropClient(cl,
                              b"was kicked\x00" as *const u8 as
                                  *const libc::c_char);
                (*cl).lastPacketTime = svs.time
            }
        }
        i += 1;
        cl = cl.offset(1isize)
    };
}
/*
==================
SV_KickBots_f

Kick all bots off of the server
==================
*/
unsafe extern "C" fn SV_KickBots_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = 0i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if !(0 == (*cl).state as u64) {
            if !((*cl).netchan.remoteAddress.type_0 as libc::c_uint !=
                     NA_BOT as libc::c_int as libc::c_uint) {
                SV_DropClient(cl,
                              b"was kicked\x00" as *const u8 as
                                  *const libc::c_char);
                (*cl).lastPacketTime = svs.time
            }
        }
        i += 1;
        cl = cl.offset(1isize)
    };
}
/*
==================
SV_BanNum_f

Ban a user from being able to play on this server through the auth
server
==================
*/
unsafe extern "C" fn SV_BanNum_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: banClient <client number>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cl = SV_GetPlayerByNum();
    if cl.is_null() { return }
    if (*cl).netchan.remoteAddress.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        Com_Printf(b"Cannot kick host player\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if 0 == svs.authorizeAddress.ip[0usize] &&
           svs.authorizeAddress.type_0 as libc::c_uint !=
               NA_BAD as libc::c_int as libc::c_uint {
        Com_Printf(b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
                   b"authorize.quake3arena.com\x00" as *const u8 as
                       *const libc::c_char);
        if 0 ==
               NET_StringToAdr(b"authorize.quake3arena.com\x00" as *const u8
                                   as *const libc::c_char,
                               &mut svs.authorizeAddress, NA_IP) {
            Com_Printf(b"Couldn\'t resolve address\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
        svs.authorizeAddress.port =
            ShortSwap(27952i32 as libc::c_short) as libc::c_ushort;
        Com_Printf(b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"authorize.quake3arena.com\x00" as *const u8 as
                       *const libc::c_char,
                   svs.authorizeAddress.ip[0usize] as libc::c_int,
                   svs.authorizeAddress.ip[1usize] as libc::c_int,
                   svs.authorizeAddress.ip[2usize] as libc::c_int,
                   svs.authorizeAddress.ip[3usize] as libc::c_int,
                   ShortSwap(svs.authorizeAddress.port as libc::c_short) as
                       libc::c_int);
    }
    if svs.authorizeAddress.type_0 as libc::c_uint !=
           NA_BAD as libc::c_int as libc::c_uint {
        NET_OutOfBandPrint(NS_SERVER, svs.authorizeAddress,
                           b"banUser %i.%i.%i.%i\x00" as *const u8 as
                               *const libc::c_char,
                           (*cl).netchan.remoteAddress.ip[0usize] as
                               libc::c_int,
                           (*cl).netchan.remoteAddress.ip[1usize] as
                               libc::c_int,
                           (*cl).netchan.remoteAddress.ip[2usize] as
                               libc::c_int,
                           (*cl).netchan.remoteAddress.ip[3usize] as
                               libc::c_int);
        Com_Printf(b"%s was banned from coming back\n\x00" as *const u8 as
                       *const libc::c_char, (*cl).name.as_mut_ptr());
    };
}
// these functions require the auth server which of course is not available anymore for stand-alone games.
/*
==================
SV_Ban_f

Ban a user from being able to play on this server through the auth
server
==================
*/
unsafe extern "C" fn SV_Ban_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: banUser <player name>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cl = SV_GetPlayerByHandle();
    if cl.is_null() { return }
    if (*cl).netchan.remoteAddress.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        Com_Printf(b"Cannot kick host player\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if 0 == svs.authorizeAddress.ip[0usize] &&
           svs.authorizeAddress.type_0 as libc::c_uint !=
               NA_BAD as libc::c_int as libc::c_uint {
        Com_Printf(b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
                   b"authorize.quake3arena.com\x00" as *const u8 as
                       *const libc::c_char);
        if 0 ==
               NET_StringToAdr(b"authorize.quake3arena.com\x00" as *const u8
                                   as *const libc::c_char,
                               &mut svs.authorizeAddress, NA_IP) {
            Com_Printf(b"Couldn\'t resolve address\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
        svs.authorizeAddress.port =
            ShortSwap(27952i32 as libc::c_short) as libc::c_ushort;
        Com_Printf(b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"authorize.quake3arena.com\x00" as *const u8 as
                       *const libc::c_char,
                   svs.authorizeAddress.ip[0usize] as libc::c_int,
                   svs.authorizeAddress.ip[1usize] as libc::c_int,
                   svs.authorizeAddress.ip[2usize] as libc::c_int,
                   svs.authorizeAddress.ip[3usize] as libc::c_int,
                   ShortSwap(svs.authorizeAddress.port as libc::c_short) as
                       libc::c_int);
    }
    if svs.authorizeAddress.type_0 as libc::c_uint !=
           NA_BAD as libc::c_int as libc::c_uint {
        NET_OutOfBandPrint(NS_SERVER, svs.authorizeAddress,
                           b"banUser %i.%i.%i.%i\x00" as *const u8 as
                               *const libc::c_char,
                           (*cl).netchan.remoteAddress.ip[0usize] as
                               libc::c_int,
                           (*cl).netchan.remoteAddress.ip[1usize] as
                               libc::c_int,
                           (*cl).netchan.remoteAddress.ip[2usize] as
                               libc::c_int,
                           (*cl).netchan.remoteAddress.ip[3usize] as
                               libc::c_int);
        Com_Printf(b"%s was banned from coming back\n\x00" as *const u8 as
                       *const libc::c_char, (*cl).name.as_mut_ptr());
    };
}
//===============================================================
/*
==================
SV_Kick_f

Kick a user off of the server
==================
*/
unsafe extern "C" fn SV_Kick_f() {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    if 0 == (*com_sv_running).integer {
        Com_Printf(b"Server is not running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: kick <player name>\nkick all = kick everyone\nkick allbots = kick all bots\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    cl = SV_GetPlayerByHandle();
    if cl.is_null() {
        if 0 ==
               Q_stricmp(Cmd_Argv(1i32),
                         b"all\x00" as *const u8 as *const libc::c_char) {
            i = 0i32;
            cl = svs.clients;
            while i < (*sv_maxclients).integer {
                if !(0 == (*cl).state as u64) {
                    if !((*cl).netchan.remoteAddress.type_0 as libc::c_uint ==
                             NA_LOOPBACK as libc::c_int as libc::c_uint) {
                        SV_DropClient(cl,
                                      b"was kicked\x00" as *const u8 as
                                          *const libc::c_char);
                        (*cl).lastPacketTime = svs.time
                    }
                }
                i += 1;
                cl = cl.offset(1isize)
            }
        } else if 0 ==
                      Q_stricmp(Cmd_Argv(1i32),
                                b"allbots\x00" as *const u8 as
                                    *const libc::c_char) {
            i = 0i32;
            cl = svs.clients;
            while i < (*sv_maxclients).integer {
                if !(0 == (*cl).state as u64) {
                    if !((*cl).netchan.remoteAddress.type_0 as libc::c_uint !=
                             NA_BOT as libc::c_int as libc::c_uint) {
                        SV_DropClient(cl,
                                      b"was kicked\x00" as *const u8 as
                                          *const libc::c_char);
                        (*cl).lastPacketTime = svs.time
                    }
                }
                i += 1;
                cl = cl.offset(1isize)
            }
        }
        return
    }
    if (*cl).netchan.remoteAddress.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        Com_Printf(b"Cannot kick host player\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    SV_DropClient(cl, b"was kicked\x00" as *const u8 as *const libc::c_char);
    (*cl).lastPacketTime = svs.time;
}
//
// sv_ccmds.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_Heartbeat_f() {
    svs.nextHeartbeatTime = -9999999i32;
}
#[no_mangle]
pub unsafe extern "C" fn SV_RemoveOperatorCommands() { }