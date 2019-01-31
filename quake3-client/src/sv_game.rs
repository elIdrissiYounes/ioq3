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
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn qsnapvectorsse(vec: *mut vec_t);
        // perpendicular vector could be replaced by this
        //int	PlaneTypeForNormal (vec3_t normal);
        #[no_mangle]
        pub fn MatrixMultiply(in1: *mut [libc::c_float; 3],
                              in2: *mut [libc::c_float; 3],
                              out: *mut [libc::c_float; 3]);
        #[no_mangle]
        pub fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                            right: *mut vec_t, up: *mut vec_t);
        #[no_mangle]
        pub fn PerpendicularVector(dst: *mut vec_t, src: *const vec_t);
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
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
    pub type vmInterpret_t = libc::c_uint;
    pub const VMI_COMPILED: vmInterpret_t = 2;
    pub const VMI_BYTECODE: vmInterpret_t = 1;
    pub const VMI_NATIVE: vmInterpret_t = 0;
    pub type unnamed_0 = libc::c_uint;
    pub const TRAP_TESTPRINTFLOAT: unnamed_0 = 113;
    pub const TRAP_TESTPRINTINT: unnamed_0 = 112;
    pub const TRAP_CEIL: unnamed_0 = 111;
    pub const TRAP_FLOOR: unnamed_0 = 110;
    pub const TRAP_PERPENDICULARVECTOR: unnamed_0 = 109;
    pub const TRAP_ANGLEVECTORS: unnamed_0 = 108;
    pub const TRAP_MATRIXMULTIPLY: unnamed_0 = 107;
    pub const TRAP_SQRT: unnamed_0 = 106;
    pub const TRAP_ATAN2: unnamed_0 = 105;
    pub const TRAP_COS: unnamed_0 = 104;
    pub const TRAP_SIN: unnamed_0 = 103;
    pub const TRAP_STRNCPY: unnamed_0 = 102;
    pub const TRAP_MEMCPY: unnamed_0 = 101;
    pub const TRAP_MEMSET: unnamed_0 = 100;
    use super::q_shared_h::{qboolean, byte, cvar_t, vmCvar_t, fileHandle_t,
                            fsMode_t, qtime_t};
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
        pub fn VM_Create(module: *const libc::c_char,
                         systemCalls:
                             Option<unsafe extern "C" fn(_: *mut intptr_t)
                                        -> intptr_t>,
                         interpret: vmInterpret_t) -> *mut vm_t;
        // module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
        #[no_mangle]
        pub fn VM_Free(vm: *mut vm_t);
        #[no_mangle]
        pub fn VM_Restart(vm: *mut vm_t, unpure: qboolean) -> *mut vm_t;
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        #[no_mangle]
        pub fn VM_ArgPtr(intValue: intptr_t) -> *mut libc::c_void;
        // Adds command text at the end of the buffer, does NOT add a final \n
        #[no_mangle]
        pub fn Cbuf_ExecuteText(exec_when: libc::c_int,
                                text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_ArgvBuffer(arg: libc::c_int, buffer: *mut libc::c_char,
                              bufferLength: libc::c_int);
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
        // same as Cvar_Set, but allows more control over setting of cvar
        #[no_mangle]
        pub fn Cvar_SetSafe(var_name: *const libc::c_char,
                            value: *const libc::c_char);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        #[no_mangle]
        pub fn Cvar_VariableIntegerValue(var_name: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Cvar_VariableStringBuffer(var_name: *const libc::c_char,
                                         buffer: *mut libc::c_char,
                                         bufsize: libc::c_int);
        #[no_mangle]
        pub fn Cvar_InfoString(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn FS_GetFileList(path: *const libc::c_char,
                              extension: *const libc::c_char,
                              listbuf: *mut libc::c_char,
                              bufsize: libc::c_int) -> libc::c_int;
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
        // will be journaled properly
        #[no_mangle]
        pub fn Com_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Com_RealTime(qtime: *mut qtime_t) -> libc::c_int;
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_public.h"]
pub mod g_public_h {
    // ( void );
    pub const GAME_CONSOLE_COMMAND: unnamed_2 = 9;
    // the server looks at a sharedEntity, which is the start of the game's gentity_t structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sharedEntity_t {
        pub s: entityState_t,
        pub r: entityShared_t,
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
    //===============================================================
    //
// system traps provided by the main engine
//
    pub type unnamed_1 = libc::c_uint;
    pub const BOTLIB_PC_SOURCE_FILE_AND_LINE: unnamed_1 = 581;
    pub const BOTLIB_PC_READ_TOKEN: unnamed_1 = 580;
    pub const BOTLIB_PC_FREE_SOURCE: unnamed_1 = 579;
    pub const BOTLIB_PC_LOAD_SOURCE: unnamed_1 = 578;
    pub const BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX: unnamed_1 = 577;
    pub const BOTLIB_AAS_PREDICT_ROUTE: unnamed_1 = 576;
    pub const BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL: unnamed_1 = 575;
    pub const BOTLIB_AI_ADD_AVOID_SPOT: unnamed_1 = 574;
    pub const BOTLIB_AI_SET_AVOID_GOAL_TIME: unnamed_1 = 573;
    pub const BOTLIB_AI_PREDICT_VISIBLE_POSITION: unnamed_1 = 572;
    pub const BOTLIB_AI_REMOVE_FROM_AVOID_GOALS: unnamed_1 = 571;
    pub const BOTLIB_AI_GET_CHAT_MESSAGE: unnamed_1 = 570;
    pub const BOTLIB_AI_NUM_INITIAL_CHATS: unnamed_1 = 569;
    pub const BOTLIB_AI_GET_MAP_LOCATION_GOAL: unnamed_1 = 568;
    pub const BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL: unnamed_1 = 567;
    pub const BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC: unnamed_1 = 566;
    pub const BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC: unnamed_1 = 565;
    pub const BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION: unnamed_1 = 564;
    pub const BOTLIB_AI_RESET_WEAPON_STATE: unnamed_1 = 563;
    pub const BOTLIB_AI_FREE_WEAPON_STATE: unnamed_1 = 562;
    pub const BOTLIB_AI_ALLOC_WEAPON_STATE: unnamed_1 = 561;
    pub const BOTLIB_AI_LOAD_WEAPON_WEIGHTS: unnamed_1 = 560;
    pub const BOTLIB_AI_GET_WEAPON_INFO: unnamed_1 = 559;
    pub const BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON: unnamed_1 = 558;
    pub const BOTLIB_AI_INIT_MOVE_STATE: unnamed_1 = 557;
    pub const BOTLIB_AI_FREE_MOVE_STATE: unnamed_1 = 556;
    pub const BOTLIB_AI_ALLOC_MOVE_STATE: unnamed_1 = 555;
    pub const BOTLIB_AI_MOVEMENT_VIEW_TARGET: unnamed_1 = 554;
    pub const BOTLIB_AI_REACHABILITY_AREA: unnamed_1 = 553;
    pub const BOTLIB_AI_RESET_LAST_AVOID_REACH: unnamed_1 = 552;
    pub const BOTLIB_AI_RESET_AVOID_REACH: unnamed_1 = 551;
    pub const BOTLIB_AI_MOVE_IN_DIRECTION: unnamed_1 = 550;
    pub const BOTLIB_AI_MOVE_TO_GOAL: unnamed_1 = 549;
    pub const BOTLIB_AI_RESET_MOVE_STATE: unnamed_1 = 548;
    pub const BOTLIB_AI_FREE_GOAL_STATE: unnamed_1 = 547;
    pub const BOTLIB_AI_ALLOC_GOAL_STATE: unnamed_1 = 546;
    pub const BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC: unnamed_1 = 545;
    pub const BOTLIB_AI_FREE_ITEM_WEIGHTS: unnamed_1 = 544;
    pub const BOTLIB_AI_LOAD_ITEM_WEIGHTS: unnamed_1 = 543;
    pub const BOTLIB_AI_UPDATE_ENTITY_ITEMS: unnamed_1 = 542;
    pub const BOTLIB_AI_INIT_LEVEL_ITEMS: unnamed_1 = 541;
    pub const BOTLIB_AI_AVOID_GOAL_TIME: unnamed_1 = 540;
    pub const BOTLIB_AI_GET_LEVEL_ITEM_GOAL: unnamed_1 = 539;
    pub const BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE: unnamed_1 = 538;
    pub const BOTLIB_AI_TOUCHING_GOAL: unnamed_1 = 537;
    pub const BOTLIB_AI_CHOOSE_NBG_ITEM: unnamed_1 = 536;
    pub const BOTLIB_AI_CHOOSE_LTG_ITEM: unnamed_1 = 535;
    pub const BOTLIB_AI_GET_SECOND_GOAL: unnamed_1 = 534;
    pub const BOTLIB_AI_GET_TOP_GOAL: unnamed_1 = 533;
    pub const BOTLIB_AI_GOAL_NAME: unnamed_1 = 532;
    pub const BOTLIB_AI_DUMP_GOAL_STACK: unnamed_1 = 531;
    pub const BOTLIB_AI_DUMP_AVOID_GOALS: unnamed_1 = 530;
    pub const BOTLIB_AI_EMPTY_GOAL_STACK: unnamed_1 = 529;
    pub const BOTLIB_AI_POP_GOAL: unnamed_1 = 528;
    pub const BOTLIB_AI_PUSH_GOAL: unnamed_1 = 527;
    pub const BOTLIB_AI_RESET_AVOID_GOALS: unnamed_1 = 526;
    pub const BOTLIB_AI_RESET_GOAL_STATE: unnamed_1 = 525;
    pub const BOTLIB_AI_SET_CHAT_NAME: unnamed_1 = 524;
    pub const BOTLIB_AI_SET_CHAT_GENDER: unnamed_1 = 523;
    pub const BOTLIB_AI_LOAD_CHAT_FILE: unnamed_1 = 522;
    pub const BOTLIB_AI_REPLACE_SYNONYMS: unnamed_1 = 521;
    pub const BOTLIB_AI_UNIFY_WHITE_SPACES: unnamed_1 = 520;
    pub const BOTLIB_AI_MATCH_VARIABLE: unnamed_1 = 519;
    pub const BOTLIB_AI_FIND_MATCH: unnamed_1 = 518;
    pub const BOTLIB_AI_STRING_CONTAINS: unnamed_1 = 517;
    pub const BOTLIB_AI_ENTER_CHAT: unnamed_1 = 516;
    pub const BOTLIB_AI_CHAT_LENGTH: unnamed_1 = 515;
    pub const BOTLIB_AI_REPLY_CHAT: unnamed_1 = 514;
    pub const BOTLIB_AI_INITIAL_CHAT: unnamed_1 = 513;
    pub const BOTLIB_AI_NUM_CONSOLE_MESSAGE: unnamed_1 = 512;
    pub const BOTLIB_AI_NEXT_CONSOLE_MESSAGE: unnamed_1 = 511;
    pub const BOTLIB_AI_REMOVE_CONSOLE_MESSAGE: unnamed_1 = 510;
    pub const BOTLIB_AI_QUEUE_CONSOLE_MESSAGE: unnamed_1 = 509;
    pub const BOTLIB_AI_FREE_CHAT_STATE: unnamed_1 = 508;
    pub const BOTLIB_AI_ALLOC_CHAT_STATE: unnamed_1 = 507;
    pub const BOTLIB_AI_CHARACTERISTIC_STRING: unnamed_1 = 506;
    pub const BOTLIB_AI_CHARACTERISTIC_BINTEGER: unnamed_1 = 505;
    pub const BOTLIB_AI_CHARACTERISTIC_INTEGER: unnamed_1 = 504;
    pub const BOTLIB_AI_CHARACTERISTIC_BFLOAT: unnamed_1 = 503;
    pub const BOTLIB_AI_CHARACTERISTIC_FLOAT: unnamed_1 = 502;
    pub const BOTLIB_AI_FREE_CHARACTER: unnamed_1 = 501;
    pub const BOTLIB_AI_LOAD_CHARACTER: unnamed_1 = 500;
    pub const BOTLIB_EA_RESET_INPUT: unnamed_1 = 423;
    pub const BOTLIB_EA_GET_INPUT: unnamed_1 = 422;
    pub const BOTLIB_EA_END_REGULAR: unnamed_1 = 421;
    pub const BOTLIB_EA_VIEW: unnamed_1 = 420;
    pub const BOTLIB_EA_MOVE: unnamed_1 = 419;
    pub const BOTLIB_EA_DELAYED_JUMP: unnamed_1 = 418;
    pub const BOTLIB_EA_JUMP: unnamed_1 = 417;
    pub const BOTLIB_EA_SELECT_WEAPON: unnamed_1 = 416;
    pub const BOTLIB_EA_MOVE_RIGHT: unnamed_1 = 415;
    pub const BOTLIB_EA_MOVE_LEFT: unnamed_1 = 414;
    pub const BOTLIB_EA_MOVE_BACK: unnamed_1 = 413;
    pub const BOTLIB_EA_MOVE_FORWARD: unnamed_1 = 412;
    pub const BOTLIB_EA_MOVE_DOWN: unnamed_1 = 411;
    pub const BOTLIB_EA_MOVE_UP: unnamed_1 = 410;
    pub const BOTLIB_EA_CROUCH: unnamed_1 = 409;
    pub const BOTLIB_EA_RESPAWN: unnamed_1 = 408;
    pub const BOTLIB_EA_USE: unnamed_1 = 407;
    pub const BOTLIB_EA_ATTACK: unnamed_1 = 406;
    pub const BOTLIB_EA_TALK: unnamed_1 = 405;
    pub const BOTLIB_EA_GESTURE: unnamed_1 = 404;
    pub const BOTLIB_EA_ACTION: unnamed_1 = 403;
    pub const BOTLIB_EA_COMMAND: unnamed_1 = 402;
    pub const BOTLIB_EA_SAY_TEAM: unnamed_1 = 401;
    pub const BOTLIB_EA_SAY: unnamed_1 = 400;
    pub const BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT: unnamed_1 = 318;
    pub const BOTLIB_AAS_SWIMMING: unnamed_1 = 317;
    pub const BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA: unnamed_1 = 316;
    pub const BOTLIB_AAS_AREA_REACHABILITY: unnamed_1 = 315;
    pub const BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY: unnamed_1 = 314;
    pub const BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY: unnamed_1 = 313;
    pub const BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY: unnamed_1 = 312;
    pub const BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY: unnamed_1 = 311;
    pub const BOTLIB_AAS_NEXT_BSP_ENTITY: unnamed_1 = 310;
    pub const BOTLIB_AAS_POINT_CONTENTS: unnamed_1 = 309;
    pub const BOTLIB_AAS_TRACE_AREAS: unnamed_1 = 308;
    pub const BOTLIB_AAS_POINT_AREA_NUM: unnamed_1 = 307;
    pub const BOTLIB_AAS_TIME: unnamed_1 = 306;
    pub const BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX: unnamed_1 = 305;
    pub const BOTLIB_AAS_INITIALIZED: unnamed_1 = 304;
    pub const BOTLIB_AAS_ENTITY_INFO: unnamed_1 = 303;
    pub const BOTLIB_AAS_AREA_INFO: unnamed_1 = 302;
    pub const BOTLIB_AAS_BBOX_AREAS: unnamed_1 = 301;
    pub const BOTLIB_AAS_ENABLE_ROUTING_AREA: unnamed_1 = 300;
    // ( int client, usercmd_t *ucmd );
    pub const BOTLIB_USER_COMMAND: unnamed_1 = 211;
    // ( int client, char *message, int size );
    pub const BOTLIB_GET_CONSOLE_MESSAGE: unnamed_1 = 210;
    // ( int client, int ent );
    pub const BOTLIB_GET_SNAPSHOT_ENTITY: unnamed_1 = 209;
    pub const BOTLIB_TEST: unnamed_1 = 208;
    pub const BOTLIB_UPDATENTITY: unnamed_1 = 207;
    pub const BOTLIB_LOAD_MAP: unnamed_1 = 206;
    pub const BOTLIB_START_FRAME: unnamed_1 = 205;
    pub const BOTLIB_PC_ADD_GLOBAL_DEFINE: unnamed_1 = 204;
    pub const BOTLIB_LIBVAR_GET: unnamed_1 = 203;
    pub const BOTLIB_LIBVAR_SET: unnamed_1 = 202;
    // ( void );
    pub const BOTLIB_SHUTDOWN: unnamed_1 = 201;
    // ( void );
    pub const BOTLIB_SETUP: unnamed_1 = 200;
    // 1.32
    pub const G_FS_SEEK: unnamed_1 = 45;
    // ( const vec3_t mins, const vec3_t maxs, const gentity_t *ent );
    pub const G_ENTITY_CONTACTCAPSULE: unnamed_1 = 44;
    // ( trace_t *results, const vec3_t start, const vec3_t mins, const vec3_t maxs, const vec3_t end, int passEntityNum, int contentmask );
    pub const G_TRACECAPSULE: unnamed_1 = 43;
    pub const G_SNAPVECTOR: unnamed_1 = 42;
    pub const G_REAL_TIME: unnamed_1 = 41;
    pub const G_DEBUG_POLYGON_DELETE: unnamed_1 = 40;
    pub const G_DEBUG_POLYGON_CREATE: unnamed_1 = 39;
    // Retrieves the next string token from the entity spawn text, returning
	// false when all tokens have been parsed.
	// This should only be done at GAME_INIT time.
    pub const G_FS_GETFILELIST: unnamed_1 = 38;
    // qboolean ( char *buffer, int bufferSize )
    pub const G_GET_ENTITY_TOKEN: unnamed_1 = 37;
    // ( int clientNum, usercmd_t *cmd )
    pub const G_GET_USERCMD: unnamed_1 = 36;
    // ( int clientNum );
    pub const G_BOT_FREE_CLIENT: unnamed_1 = 35;
    // perform an exact check against inline brush models of non-square shape
    // access for bots to get and free a server client (FIXME?)
    // ( void );
    pub const G_BOT_ALLOCATE_CLIENT: unnamed_1 = 34;
    // EntitiesInBox will return brush models based on their bounding box,
	// so exact determination must still be done with EntityContact
    // ( const vec3_t mins, const vec3_t maxs, const gentity_t *ent );
    pub const G_ENTITY_CONTACT: unnamed_1 = 33;
    // call before removing an interactive entity
    // ( const vec3_t mins, const vec3_t maxs, gentity_t **list, int maxcount );
    pub const G_ENTITIES_IN_BOX: unnamed_1 = 32;
    // an entity will never be sent to a client or used for collision
	// if it is not passed to linkentity.  If the size, position, or
	// solidity changes, it must be relinked.
    // ( gentity_t *ent );		
    pub const G_UNLINKENTITY: unnamed_1 = 31;
    // ( gentity_t *ent );
    pub const G_LINKENTITY: unnamed_1 = 30;
    // ( int area1, int area2 );
    pub const G_AREAS_CONNECTED: unnamed_1 = 29;
    // ( gentity_t *ent, qboolean open );
    pub const G_ADJUST_AREA_PORTAL_STATE: unnamed_1 = 28;
    // ( const vec3_t p1, const vec3_t p2 );
    pub const G_IN_PVS_IGNORE_PORTALS: unnamed_1 = 27;
    // point contents against all linked entities
    // ( const vec3_t p1, const vec3_t p2 );
    pub const G_IN_PVS: unnamed_1 = 26;
    // collision detection against all linked entities
    // ( const vec3_t point, int passEntityNum );
    pub const G_POINT_CONTENTS: unnamed_1 = 25;
    // sets mins and maxs based on the brushmodel name
    // ( trace_t *results, const vec3_t start, const vec3_t mins, const vec3_t maxs, const vec3_t end, int passEntityNum, int contentmask );
    pub const G_TRACE: unnamed_1 = 24;
    // the serverinfo info string has all the cvars visible to server browsers
    // ( gentity_t *ent, const char *name );
    pub const G_SET_BRUSH_MODEL: unnamed_1 = 23;
    // ( char *buffer, int bufferSize );
    pub const G_GET_SERVERINFO: unnamed_1 = 22;
    // userinfo strings are maintained by the server system, so they
	// are persistant across level loads, while all other game visible
	// data is completely reset
    // ( int num, const char *buffer );
    pub const G_SET_USERINFO: unnamed_1 = 21;
    // ( int num, char *buffer, int bufferSize );
    pub const G_GET_USERINFO: unnamed_1 = 20;
    // config strings hold all the index strings, and various other information
	// that is reliably communicated to all clients
	// All of the current configstrings are sent to clients when
	// they connect, and changes are sent to all connected clients.
	// All confgstrings are cleared at each level start.
    // ( int num, char *buffer, int bufferSize );
    pub const G_GET_CONFIGSTRING: unnamed_1 = 19;
    // reliably sends a command string to be interpreted by the given
	// client.  If clientNum is -1, it will be sent to all clients
    // ( int num, const char *string );
    pub const G_SET_CONFIGSTRING: unnamed_1 = 18;
    // kick a client off the server with a message
    // ( int clientNum, const char *fmt, ... );
    pub const G_SEND_SERVER_COMMAND: unnamed_1 = 17;
    //							playerState_t *clients, int sizeofGameClient );
	// the game needs to let the server system know where and how big the gentities
	// are, so it can look at them directly without going through an interface
    // ( int clientNum, const char *reason );
    pub const G_DROP_CLIENT: unnamed_1 = 16;
    // add commands to the console as if they were typed in
	// for map changing, etc
    //=========== server specific functionality =============
    // ( gentity_t *gEnts, int numGEntities, int sizeofGEntity_t,
    pub const G_LOCATE_GAME_DATA: unnamed_1 = 15;
    // ( const char *text );
    pub const G_SEND_CONSOLE_COMMAND: unnamed_1 = 14;
    // ( fileHandle_t f );
    pub const G_FS_FCLOSE_FILE: unnamed_1 = 13;
    // ( const void *buffer, int len, fileHandle_t f );
    pub const G_FS_WRITE: unnamed_1 = 12;
    // ( void *buffer, int len, fileHandle_t f );
    pub const G_FS_READ: unnamed_1 = 11;
    // ( const char *qpath, fileHandle_t *file, fsMode_t mode );
    pub const G_FS_FOPEN_FILE: unnamed_1 = 10;
    // ClientCommand and ServerCommand parameter access
    // ( int n, char *buffer, int bufferLength );
    pub const G_ARGV: unnamed_1 = 9;
    // ( void );
    pub const G_ARGC: unnamed_1 = 8;
    // ( const char *var_name, char *buffer, int bufsize );
    pub const G_CVAR_VARIABLE_STRING_BUFFER: unnamed_1 = 7;
    // ( const char *var_name );
    pub const G_CVAR_VARIABLE_INTEGER_VALUE: unnamed_1 = 6;
    // ( const char *var_name, const char *value );
    pub const G_CVAR_SET: unnamed_1 = 5;
    // ( vmCvar_t *vmCvar );
    pub const G_CVAR_UPDATE: unnamed_1 = 4;
    // get current time for profiling reasons
	// this should NOT be used for any game related tasks,
	// because it is not journaled
    // console variable interaction
    // ( vmCvar_t *vmCvar, const char *varName, const char *defaultValue, int flags );
    pub const G_CVAR_REGISTER: unnamed_1 = 3;
    // abort the game
    // ( void );
    pub const G_MILLISECONDS: unnamed_1 = 2;
    // print message on the local console
    // ( const char *string );
    pub const G_ERROR: unnamed_1 = 1;
    //============== general Quake services ==================
    // ( const char *string );
    pub const G_PRINT: unnamed_1 = 0;
    //
// functions exported by the game subsystem
//
    pub type unnamed_2 = libc::c_uint;
    // ConsoleCommand will be called when a command has been issued
	// that is not recognized as a builtin function.
	// The game can issue trap_argc() / trap_argv() commands to get the command
	// and parameters.  Return qfalse if the game doesn't recognize it as a command.
    // ( int time );
    pub const BOTAI_START_FRAME: unnamed_2 = 10;
    // ( int levelTime );
    pub const GAME_RUN_FRAME: unnamed_2 = 8;
    // ( int clientNum );
    pub const GAME_CLIENT_THINK: unnamed_2 = 7;
    // ( int clientNum );
    pub const GAME_CLIENT_COMMAND: unnamed_2 = 6;
    // ( int clientNum );
    pub const GAME_CLIENT_DISCONNECT: unnamed_2 = 5;
    // ( int clientNum );
    pub const GAME_CLIENT_USERINFO_CHANGED: unnamed_2 = 4;
    // return NULL if the client is allowed to connect, otherwise return
	// a text string with the reason for denial
    // ( int clientNum );
    pub const GAME_CLIENT_BEGIN: unnamed_2 = 3;
    // ( int clientNum, qboolean firstTime, qboolean isBot );
    pub const GAME_CLIENT_CONNECT: unnamed_2 = 2;
    // init and shutdown will be called every single level
	// The game should call G_GET_ENTITY_TOKEN to parse through all the
	// entity configuration text and spawn gentities.
    // (void);
    pub const GAME_SHUTDOWN: unnamed_2 = 1;
    // ( int levelTime, int randomSeed, int restart );
    pub const GAME_INIT: unnamed_2 = 0;
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/server.h"]
pub mod server_h {
    // actively running
    pub const SS_GAME: serverState_t = 2;
    pub type serverState_t = libc::c_uint;
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
    use super::q_shared_h::{qboolean, playerState_t, entityState_t, byte,
                            usercmd_t, fileHandle_t, cvar_t, vec3_t, vec_t,
                            trace_t, clipHandle_t};
    use super::g_public_h::{sharedEntity_t};
    use super::qcommon_h::{msg_t, netchan_t, netadr_t, vm_t};
    extern "C" {
        pub type worldSector_s;
        // game virtual machine
        #[no_mangle]
        pub static mut gvm: *mut vm_t;
        // cleared each map
        #[no_mangle]
        pub static mut sv: server_t;
        //=============================================================================
        // persistant server info across maps
        #[no_mangle]
        pub static mut svs: serverStatic_t;
        #[no_mangle]
        pub static mut sv_maxclients: *mut cvar_t;
        //
// sv_init.c
//
        #[no_mangle]
        pub fn SV_SetConfigstring(index: libc::c_int,
                                  val: *const libc::c_char);
        #[no_mangle]
        pub fn SV_GetConfigstring(index: libc::c_int,
                                  buffer: *mut libc::c_char,
                                  bufferSize: libc::c_int);
        #[no_mangle]
        pub fn SV_SetUserinfo(index: libc::c_int, val: *const libc::c_char);
        #[no_mangle]
        pub fn SV_GetUserinfo(index: libc::c_int, buffer: *mut libc::c_char,
                              bufferSize: libc::c_int);
        #[no_mangle]
        pub fn SV_DropClient(drop_0: *mut client_t,
                             reason: *const libc::c_char);
        #[no_mangle]
        pub fn SV_ClientThink(cl: *mut client_t, cmd: *mut usercmd_t);
        #[no_mangle]
        pub fn SV_BotGetConsoleMessage(client: libc::c_int,
                                       buf: *mut libc::c_char,
                                       size: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn SV_BotGetSnapshotEntity(client: libc::c_int, ent: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        pub fn SV_BotLibShutdown() -> libc::c_int;
        #[no_mangle]
        pub fn SV_BotLibSetup() -> libc::c_int;
        #[no_mangle]
        pub fn BotImport_DebugPolygonDelete(id: libc::c_int);
        #[no_mangle]
        pub fn BotImport_DebugPolygonCreate(color: libc::c_int,
                                            numPoints: libc::c_int,
                                            points: *mut vec3_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn SV_BotFreeClient(clientNum: libc::c_int);
        #[no_mangle]
        pub fn SV_BotAllocateClient() -> libc::c_int;
        // call before removing an entity, and before trying to move one,
// so it doesn't clip against itself
        #[no_mangle]
        pub fn SV_LinkEntity(ent: *mut sharedEntity_t);
        // fills in a table of entity numbers with entities that have bounding boxes
// that intersect the given area.  It is possible for a non-axial bmodel
// to be returned that doesn't actually intersect the area on an exact
// test.
// returns the number of pointers filled in
// The world entity is never returned in this list.
        #[no_mangle]
        pub fn SV_PointContents(p: *const vec_t, passEntityNum: libc::c_int)
         -> libc::c_int;
        // returns the CONTENTS_* value from the world and all entities at the given point.
        #[no_mangle]
        pub fn SV_Trace(results: *mut trace_t, start: *const vec_t,
                        mins: *mut vec_t, maxs: *mut vec_t, end: *const vec_t,
                        passEntityNum: libc::c_int, contentmask: libc::c_int,
                        capsule: libc::c_int);
        // Needs to be called any time an entity changes origin, mins, maxs,
// or solid.  Automatically unlinks if needed.
// sets ent->r.absmin and ent->r.absmax
// sets ent->leafnums[] for pvs determination even if the entity
// is not solid
        #[no_mangle]
        pub fn SV_ClipHandleForEntity(ent: *const sharedEntity_t)
         -> clipHandle_t;
        #[no_mangle]
        pub fn SV_AreaEntities(mins: *const vec_t, maxs: *const vec_t,
                               entityList: *mut libc::c_int,
                               maxcount: libc::c_int) -> libc::c_int;
        // called after the world model has been loaded, before linking any entities
        #[no_mangle]
        pub fn SV_UnlinkEntity(ent: *mut sharedEntity_t);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
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
    use super::q_shared_h::{vec_t, pc_token_t, vec3_t};
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
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::q_shared_h::{clipHandle_t, vec_t, trace_t, byte, qboolean};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CM_InlineModel(index: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_ModelBounds(model: clipHandle_t, mins: *mut vec_t,
                              maxs: *mut vec_t);
        #[no_mangle]
        pub fn CM_EntityString() -> *mut libc::c_char;
        #[no_mangle]
        pub fn CM_TransformedBoxTrace(results: *mut trace_t,
                                      start: *const vec_t, end: *const vec_t,
                                      mins: *mut vec_t, maxs: *mut vec_t,
                                      model: clipHandle_t,
                                      brushmask: libc::c_int,
                                      origin: *const vec_t,
                                      angles: *const vec_t,
                                      capsule: libc::c_int);
        #[no_mangle]
        pub fn CM_ClusterPVS(cluster: libc::c_int) -> *mut byte;
        #[no_mangle]
        pub fn CM_PointLeafnum(p: *const vec_t) -> libc::c_int;
        #[no_mangle]
        pub fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn CM_AdjustAreaPortalState(area1: libc::c_int,
                                        area2: libc::c_int, open: qboolean);
        #[no_mangle]
        pub fn CM_AreasConnected(area1: libc::c_int, area2: libc::c_int)
         -> qboolean;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/sv_game.c"]
pub mod sv_game_c {
    use super::stdint_h::{intptr_t};
    use super::botlib_h::{botlib_export_t};
    use super::{libc};
    use super::q_shared_h::{usercmd_t, qboolean, vec_t, playerState_t};
    use super::g_public_h::{sharedEntity_t};
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
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t,
                       fileHandle_t, clipHandle_t, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, vec_t, vec3_t, cplane_s, pc_token_s,
                       pc_token_t, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cvar_s, cvar_t, cvarHandle_t,
                       vmCvar_t, cplane_t, trace_t, playerState_s,
                       playerState_t, usercmd_s, usercmd_t, trType_t,
                       TR_GRAVITY, TR_SINE, TR_LINEAR_STOP, TR_LINEAR,
                       TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, qtime_s, qtime_t,
                       vec3_origin, qsnapvectorsse, MatrixMultiply,
                       AngleVectors, PerpendicularVector, COM_Parse,
                       Q_strncpyz, Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      vm_t, vmInterpret_t, VMI_COMPILED, VMI_BYTECODE,
                      VMI_NATIVE, unnamed_0, TRAP_TESTPRINTFLOAT,
                      TRAP_TESTPRINTINT, TRAP_CEIL, TRAP_FLOOR,
                      TRAP_PERPENDICULARVECTOR, TRAP_ANGLEVECTORS,
                      TRAP_MATRIXMULTIPLY, TRAP_SQRT, TRAP_ATAN2, TRAP_COS,
                      TRAP_SIN, TRAP_STRNCPY, TRAP_MEMCPY, TRAP_MEMSET, vm_s,
                      VM_Create, VM_Free, VM_Restart, VM_Call, VM_ArgPtr,
                      Cbuf_ExecuteText, Cmd_Argc, Cmd_ArgvBuffer, Cvar_Get,
                      Cvar_Register, Cvar_Update, Cvar_SetSafe,
                      Cvar_VariableValue, Cvar_VariableIntegerValue,
                      Cvar_VariableStringBuffer, Cvar_InfoString,
                      FS_GetFileList, FS_Write, FS_Read, FS_FCloseFile,
                      FS_FOpenFileByMode, FS_Seek, Com_Milliseconds,
                      Com_RealTime, Sys_Milliseconds};
use self::g_public_h::{GAME_CONSOLE_COMMAND, sharedEntity_t, entityShared_t,
                       unnamed_1, BOTLIB_PC_SOURCE_FILE_AND_LINE,
                       BOTLIB_PC_READ_TOKEN, BOTLIB_PC_FREE_SOURCE,
                       BOTLIB_PC_LOAD_SOURCE,
                       BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX,
                       BOTLIB_AAS_PREDICT_ROUTE,
                       BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL,
                       BOTLIB_AI_ADD_AVOID_SPOT,
                       BOTLIB_AI_SET_AVOID_GOAL_TIME,
                       BOTLIB_AI_PREDICT_VISIBLE_POSITION,
                       BOTLIB_AI_REMOVE_FROM_AVOID_GOALS,
                       BOTLIB_AI_GET_CHAT_MESSAGE,
                       BOTLIB_AI_NUM_INITIAL_CHATS,
                       BOTLIB_AI_GET_MAP_LOCATION_GOAL,
                       BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL,
                       BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC,
                       BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC,
                       BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION,
                       BOTLIB_AI_RESET_WEAPON_STATE,
                       BOTLIB_AI_FREE_WEAPON_STATE,
                       BOTLIB_AI_ALLOC_WEAPON_STATE,
                       BOTLIB_AI_LOAD_WEAPON_WEIGHTS,
                       BOTLIB_AI_GET_WEAPON_INFO,
                       BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON,
                       BOTLIB_AI_INIT_MOVE_STATE, BOTLIB_AI_FREE_MOVE_STATE,
                       BOTLIB_AI_ALLOC_MOVE_STATE,
                       BOTLIB_AI_MOVEMENT_VIEW_TARGET,
                       BOTLIB_AI_REACHABILITY_AREA,
                       BOTLIB_AI_RESET_LAST_AVOID_REACH,
                       BOTLIB_AI_RESET_AVOID_REACH,
                       BOTLIB_AI_MOVE_IN_DIRECTION, BOTLIB_AI_MOVE_TO_GOAL,
                       BOTLIB_AI_RESET_MOVE_STATE, BOTLIB_AI_FREE_GOAL_STATE,
                       BOTLIB_AI_ALLOC_GOAL_STATE,
                       BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC,
                       BOTLIB_AI_FREE_ITEM_WEIGHTS,
                       BOTLIB_AI_LOAD_ITEM_WEIGHTS,
                       BOTLIB_AI_UPDATE_ENTITY_ITEMS,
                       BOTLIB_AI_INIT_LEVEL_ITEMS, BOTLIB_AI_AVOID_GOAL_TIME,
                       BOTLIB_AI_GET_LEVEL_ITEM_GOAL,
                       BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE,
                       BOTLIB_AI_TOUCHING_GOAL, BOTLIB_AI_CHOOSE_NBG_ITEM,
                       BOTLIB_AI_CHOOSE_LTG_ITEM, BOTLIB_AI_GET_SECOND_GOAL,
                       BOTLIB_AI_GET_TOP_GOAL, BOTLIB_AI_GOAL_NAME,
                       BOTLIB_AI_DUMP_GOAL_STACK, BOTLIB_AI_DUMP_AVOID_GOALS,
                       BOTLIB_AI_EMPTY_GOAL_STACK, BOTLIB_AI_POP_GOAL,
                       BOTLIB_AI_PUSH_GOAL, BOTLIB_AI_RESET_AVOID_GOALS,
                       BOTLIB_AI_RESET_GOAL_STATE, BOTLIB_AI_SET_CHAT_NAME,
                       BOTLIB_AI_SET_CHAT_GENDER, BOTLIB_AI_LOAD_CHAT_FILE,
                       BOTLIB_AI_REPLACE_SYNONYMS,
                       BOTLIB_AI_UNIFY_WHITE_SPACES, BOTLIB_AI_MATCH_VARIABLE,
                       BOTLIB_AI_FIND_MATCH, BOTLIB_AI_STRING_CONTAINS,
                       BOTLIB_AI_ENTER_CHAT, BOTLIB_AI_CHAT_LENGTH,
                       BOTLIB_AI_REPLY_CHAT, BOTLIB_AI_INITIAL_CHAT,
                       BOTLIB_AI_NUM_CONSOLE_MESSAGE,
                       BOTLIB_AI_NEXT_CONSOLE_MESSAGE,
                       BOTLIB_AI_REMOVE_CONSOLE_MESSAGE,
                       BOTLIB_AI_QUEUE_CONSOLE_MESSAGE,
                       BOTLIB_AI_FREE_CHAT_STATE, BOTLIB_AI_ALLOC_CHAT_STATE,
                       BOTLIB_AI_CHARACTERISTIC_STRING,
                       BOTLIB_AI_CHARACTERISTIC_BINTEGER,
                       BOTLIB_AI_CHARACTERISTIC_INTEGER,
                       BOTLIB_AI_CHARACTERISTIC_BFLOAT,
                       BOTLIB_AI_CHARACTERISTIC_FLOAT,
                       BOTLIB_AI_FREE_CHARACTER, BOTLIB_AI_LOAD_CHARACTER,
                       BOTLIB_EA_RESET_INPUT, BOTLIB_EA_GET_INPUT,
                       BOTLIB_EA_END_REGULAR, BOTLIB_EA_VIEW, BOTLIB_EA_MOVE,
                       BOTLIB_EA_DELAYED_JUMP, BOTLIB_EA_JUMP,
                       BOTLIB_EA_SELECT_WEAPON, BOTLIB_EA_MOVE_RIGHT,
                       BOTLIB_EA_MOVE_LEFT, BOTLIB_EA_MOVE_BACK,
                       BOTLIB_EA_MOVE_FORWARD, BOTLIB_EA_MOVE_DOWN,
                       BOTLIB_EA_MOVE_UP, BOTLIB_EA_CROUCH, BOTLIB_EA_RESPAWN,
                       BOTLIB_EA_USE, BOTLIB_EA_ATTACK, BOTLIB_EA_TALK,
                       BOTLIB_EA_GESTURE, BOTLIB_EA_ACTION, BOTLIB_EA_COMMAND,
                       BOTLIB_EA_SAY_TEAM, BOTLIB_EA_SAY,
                       BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT,
                       BOTLIB_AAS_SWIMMING,
                       BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA,
                       BOTLIB_AAS_AREA_REACHABILITY,
                       BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_NEXT_BSP_ENTITY, BOTLIB_AAS_POINT_CONTENTS,
                       BOTLIB_AAS_TRACE_AREAS, BOTLIB_AAS_POINT_AREA_NUM,
                       BOTLIB_AAS_TIME, BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX,
                       BOTLIB_AAS_INITIALIZED, BOTLIB_AAS_ENTITY_INFO,
                       BOTLIB_AAS_AREA_INFO, BOTLIB_AAS_BBOX_AREAS,
                       BOTLIB_AAS_ENABLE_ROUTING_AREA, BOTLIB_USER_COMMAND,
                       BOTLIB_GET_CONSOLE_MESSAGE, BOTLIB_GET_SNAPSHOT_ENTITY,
                       BOTLIB_TEST, BOTLIB_UPDATENTITY, BOTLIB_LOAD_MAP,
                       BOTLIB_START_FRAME, BOTLIB_PC_ADD_GLOBAL_DEFINE,
                       BOTLIB_LIBVAR_GET, BOTLIB_LIBVAR_SET, BOTLIB_SHUTDOWN,
                       BOTLIB_SETUP, G_FS_SEEK, G_ENTITY_CONTACTCAPSULE,
                       G_TRACECAPSULE, G_SNAPVECTOR, G_REAL_TIME,
                       G_DEBUG_POLYGON_DELETE, G_DEBUG_POLYGON_CREATE,
                       G_FS_GETFILELIST, G_GET_ENTITY_TOKEN, G_GET_USERCMD,
                       G_BOT_FREE_CLIENT, G_BOT_ALLOCATE_CLIENT,
                       G_ENTITY_CONTACT, G_ENTITIES_IN_BOX, G_UNLINKENTITY,
                       G_LINKENTITY, G_AREAS_CONNECTED,
                       G_ADJUST_AREA_PORTAL_STATE, G_IN_PVS_IGNORE_PORTALS,
                       G_IN_PVS, G_POINT_CONTENTS, G_TRACE, G_SET_BRUSH_MODEL,
                       G_GET_SERVERINFO, G_SET_USERINFO, G_GET_USERINFO,
                       G_GET_CONFIGSTRING, G_SET_CONFIGSTRING,
                       G_SEND_SERVER_COMMAND, G_DROP_CLIENT,
                       G_LOCATE_GAME_DATA, G_SEND_CONSOLE_COMMAND,
                       G_FS_FCLOSE_FILE, G_FS_WRITE, G_FS_READ,
                       G_FS_FOPEN_FILE, G_ARGV, G_ARGC,
                       G_CVAR_VARIABLE_STRING_BUFFER,
                       G_CVAR_VARIABLE_INTEGER_VALUE, G_CVAR_SET,
                       G_CVAR_UPDATE, G_CVAR_REGISTER, G_MILLISECONDS,
                       G_ERROR, G_PRINT, unnamed_2, BOTAI_START_FRAME,
                       GAME_RUN_FRAME, GAME_CLIENT_THINK, GAME_CLIENT_COMMAND,
                       GAME_CLIENT_DISCONNECT, GAME_CLIENT_USERINFO_CHANGED,
                       GAME_CLIENT_BEGIN, GAME_CLIENT_CONNECT, GAME_SHUTDOWN,
                       GAME_INIT};
use self::server_h::{SS_GAME, serverState_t, SS_LOADING, SS_DEAD, server_t,
                     svEntity_t, svEntity_s, voipServerPacket_s,
                     voipServerPacket_t, clientSnapshot_t, clientState_t,
                     CS_ACTIVE, CS_PRIMED, CS_CONNECTED, CS_ZOMBIE, CS_FREE,
                     netchan_buffer_s, netchan_buffer_t, client_s, client_t,
                     challenge_t, serverStatic_t, worldSector_s, gvm, sv, svs,
                     sv_maxclients, SV_SetConfigstring, SV_GetConfigstring,
                     SV_SetUserinfo, SV_GetUserinfo, SV_DropClient,
                     SV_ClientThink, SV_BotGetConsoleMessage,
                     SV_BotGetSnapshotEntity, SV_BotLibShutdown,
                     SV_BotLibSetup, BotImport_DebugPolygonDelete,
                     BotImport_DebugPolygonCreate, SV_BotFreeClient,
                     SV_BotAllocateClient, SV_LinkEntity, SV_PointContents,
                     SV_Trace, SV_ClipHandleForEntity, SV_AreaEntities,
                     SV_UnlinkEntity};
use self::botlib_h::{ai_export_t, ai_export_s, botlib_export_t,
                     botlib_export_s, bot_entitystate_t, bot_entitystate_s,
                     ea_export_t, ea_export_s, bot_input_t, bot_input_s,
                     aas_export_t, aas_export_s, weaponinfo_s, bot_initmove_s,
                     bot_goal_s, bot_moveresult_s, bot_match_s,
                     bot_consolemessage_s, aas_clientmove_s,
                     aas_altroutegoal_s, aas_predictroute_s, aas_areainfo_s,
                     aas_entityinfo_s};
use self::mathcalls_h::{atan2, cos, sin, sqrt, ceil, floor};
use self::string_h::{memcpy, memset, strncpy};
use self::stdlib_h::{atoi};
use self::cm_public_h::{CM_InlineModel, CM_ModelBounds, CM_EntityString,
                        CM_TransformedBoxTrace, CM_ClusterPVS,
                        CM_PointLeafnum, CM_LeafCluster, CM_LeafArea,
                        CM_AdjustAreaPortalState, CM_AreasConnected};
use self::sv_variadic_h::{SV_SendServerCommand};
unsafe extern "C" fn _vmf(mut x: intptr_t) -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i = x as libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn SV_GameCommand() -> qboolean {
    if sv.state as libc::c_uint != SS_GAME as libc::c_int as libc::c_uint {
        return qfalse
    }
    return VM_Call(gvm, GAME_CONSOLE_COMMAND as libc::c_int) as qboolean;
}
//
// sv_game.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_NumForGentity(mut ent: *mut sharedEntity_t)
 -> libc::c_int {
    let mut num: libc::c_int = 0;
    num =
        ((ent as *mut byte).wrapping_offset_from(sv.gentities as *mut byte) as
             libc::c_long / sv.gentitySize as libc::c_long) as libc::c_int;
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn SV_GentityNum(mut num: libc::c_int)
 -> *mut sharedEntity_t {
    let mut ent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    ent =
        (sv.gentities as *mut byte).offset((sv.gentitySize * num) as isize) as
            *mut sharedEntity_t;
    return ent;
}
#[no_mangle]
pub unsafe extern "C" fn SV_GameClientNum(mut num: libc::c_int)
 -> *mut playerState_t {
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    ps =
        (sv.gameClients as
             *mut byte).offset((sv.gameClientSize * num) as isize) as
            *mut playerState_t;
    return ps;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SvEntityForGentity(mut gEnt: *mut sharedEntity_t)
 -> *mut svEntity_t {
    if gEnt.is_null() || (*gEnt).s.number < 0i32 ||
           (*gEnt).s.number >= 1i32 << 10i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_SvEntityForGentity: bad gEnt\x00" as *const u8 as
                      *const libc::c_char);
    }
    return &mut sv.svEntities[(*gEnt).s.number as usize] as *mut svEntity_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_GEntityForSvEntity(mut svEnt: *mut svEntity_t)
 -> *mut sharedEntity_t {
    let mut num: libc::c_int = 0;
    num =
        svEnt.wrapping_offset_from(sv.svEntities.as_mut_ptr()) as libc::c_long
            as libc::c_int;
    return SV_GentityNum(num);
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitGameProgs() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    //FIXME these are temp while I make bots run in vm
    extern "C" {
        #[no_mangle]
        pub static mut bot_enable: libc::c_int;
    }
    var =
        Cvar_Get(b"bot_enable\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x20i32);
    if !var.is_null() {
        bot_enable = (*var).integer
    } else { bot_enable = 0i32 }
    gvm =
        VM_Create(b"qagame\x00" as *const u8 as *const libc::c_char,
                  Some(SV_GameSystemCalls),
                  Cvar_VariableValue(b"vm_game\x00" as *const u8 as
                                         *const libc::c_char) as
                      vmInterpret_t);
    if gvm.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_Create on game failed\x00" as *const u8 as
                      *const libc::c_char);
    }
    SV_InitGameVM(qfalse);
}
/*
==================
SV_InitGameVM

Called for both a full init and a restart
==================
*/
unsafe extern "C" fn SV_InitGameVM(mut restart: qboolean) {
    let mut i: libc::c_int = 0;
    sv.entityParsePoint = CM_EntityString();
    i = 0i32;
    while i < (*sv_maxclients).integer {
        let ref mut fresh0 = (*svs.clients.offset(i as isize)).gentity;
        *fresh0 = 0 as *mut sharedEntity_t;
        i += 1
    }
    VM_Call(gvm, GAME_INIT as libc::c_int, sv.time, Com_Milliseconds(),
            restart as libc::c_uint);
}
/*
====================
SV_GameSystemCalls

The module is making a system call
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GameSystemCalls(mut args: *mut intptr_t)
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
            return Cvar_VariableIntegerValue(VM_ArgPtr(*args.offset(1isize))
                                                 as *const libc::c_char) as
                       intptr_t
        }
        7 => {
            Cvar_VariableStringBuffer(VM_ArgPtr(*args.offset(1isize)) as
                                          *const libc::c_char,
                                      VM_ArgPtr(*args.offset(2isize)) as
                                          *mut libc::c_char,
                                      *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        8 => { return Cmd_Argc() as intptr_t }
        9 => {
            Cmd_ArgvBuffer(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *mut libc::c_char,
                           *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        14 => {
            Cbuf_ExecuteText(*args.offset(1isize) as libc::c_int,
                             VM_ArgPtr(*args.offset(2isize)) as
                                 *const libc::c_char);
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
        38 => {
            return FS_GetFileList(VM_ArgPtr(*args.offset(1isize)) as
                                      *const libc::c_char,
                                  VM_ArgPtr(*args.offset(2isize)) as
                                      *const libc::c_char,
                                  VM_ArgPtr(*args.offset(3isize)) as
                                      *mut libc::c_char,
                                  *args.offset(4isize) as libc::c_int) as
                       intptr_t
        }
        45 => {
            return FS_Seek(*args.offset(1isize) as fileHandle_t,
                           *args.offset(2isize),
                           *args.offset(3isize) as libc::c_int) as intptr_t
        }
        15 => {
            SV_LocateGameData(VM_ArgPtr(*args.offset(1isize)) as
                                  *mut sharedEntity_t,
                              *args.offset(2isize) as libc::c_int,
                              *args.offset(3isize) as libc::c_int,
                              VM_ArgPtr(*args.offset(4isize)) as
                                  *mut playerState_t,
                              *args.offset(5isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        16 => {
            SV_GameDropClient(*args.offset(1isize) as libc::c_int,
                              VM_ArgPtr(*args.offset(2isize)) as
                                  *const libc::c_char);
            return 0i32 as intptr_t
        }
        17 => {
            SV_GameSendServerCommand(*args.offset(1isize) as libc::c_int,
                                     VM_ArgPtr(*args.offset(2isize)) as
                                         *const libc::c_char);
            return 0i32 as intptr_t
        }
        30 => {
            SV_LinkEntity(VM_ArgPtr(*args.offset(1isize)) as
                              *mut sharedEntity_t);
            return 0i32 as intptr_t
        }
        31 => {
            SV_UnlinkEntity(VM_ArgPtr(*args.offset(1isize)) as
                                *mut sharedEntity_t);
            return 0i32 as intptr_t
        }
        32 => {
            return SV_AreaEntities(VM_ArgPtr(*args.offset(1isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const vec_t,
                                   VM_ArgPtr(*args.offset(3isize)) as
                                       *mut libc::c_int,
                                   *args.offset(4isize) as libc::c_int) as
                       intptr_t
        }
        33 => {
            return SV_EntityContact(VM_ArgPtr(*args.offset(1isize)) as
                                        *mut vec_t,
                                    VM_ArgPtr(*args.offset(2isize)) as
                                        *mut vec_t,
                                    VM_ArgPtr(*args.offset(3isize)) as
                                        *const sharedEntity_t,
                                    qfalse as libc::c_int) as intptr_t
        }
        44 => {
            return SV_EntityContact(VM_ArgPtr(*args.offset(1isize)) as
                                        *mut vec_t,
                                    VM_ArgPtr(*args.offset(2isize)) as
                                        *mut vec_t,
                                    VM_ArgPtr(*args.offset(3isize)) as
                                        *const sharedEntity_t,
                                    qtrue as libc::c_int) as intptr_t
        }
        24 => {
            SV_Trace(VM_ArgPtr(*args.offset(1isize)) as *mut trace_t,
                     VM_ArgPtr(*args.offset(2isize)) as *const vec_t,
                     VM_ArgPtr(*args.offset(3isize)) as *mut vec_t,
                     VM_ArgPtr(*args.offset(4isize)) as *mut vec_t,
                     VM_ArgPtr(*args.offset(5isize)) as *const vec_t,
                     *args.offset(6isize) as libc::c_int,
                     *args.offset(7isize) as libc::c_int,
                     qfalse as libc::c_int);
            return 0i32 as intptr_t
        }
        43 => {
            SV_Trace(VM_ArgPtr(*args.offset(1isize)) as *mut trace_t,
                     VM_ArgPtr(*args.offset(2isize)) as *const vec_t,
                     VM_ArgPtr(*args.offset(3isize)) as *mut vec_t,
                     VM_ArgPtr(*args.offset(4isize)) as *mut vec_t,
                     VM_ArgPtr(*args.offset(5isize)) as *const vec_t,
                     *args.offset(6isize) as libc::c_int,
                     *args.offset(7isize) as libc::c_int,
                     qtrue as libc::c_int);
            return 0i32 as intptr_t
        }
        25 => {
            return SV_PointContents(VM_ArgPtr(*args.offset(1isize)) as
                                        *const vec_t,
                                    *args.offset(2isize) as libc::c_int) as
                       intptr_t
        }
        23 => {
            SV_SetBrushModel(VM_ArgPtr(*args.offset(1isize)) as
                                 *mut sharedEntity_t,
                             VM_ArgPtr(*args.offset(2isize)) as
                                 *const libc::c_char);
            return 0i32 as intptr_t
        }
        26 => {
            return SV_inPVS(VM_ArgPtr(*args.offset(1isize)) as *const vec_t,
                            VM_ArgPtr(*args.offset(2isize)) as *const vec_t)
                       as intptr_t
        }
        27 => {
            return SV_inPVSIgnorePortals(VM_ArgPtr(*args.offset(1isize)) as
                                             *const vec_t,
                                         VM_ArgPtr(*args.offset(2isize)) as
                                             *const vec_t) as intptr_t
        }
        18 => {
            SV_SetConfigstring(*args.offset(1isize) as libc::c_int,
                               VM_ArgPtr(*args.offset(2isize)) as
                                   *const libc::c_char);
            return 0i32 as intptr_t
        }
        19 => {
            SV_GetConfigstring(*args.offset(1isize) as libc::c_int,
                               VM_ArgPtr(*args.offset(2isize)) as
                                   *mut libc::c_char,
                               *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        21 => {
            SV_SetUserinfo(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *const libc::c_char);
            return 0i32 as intptr_t
        }
        20 => {
            SV_GetUserinfo(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *mut libc::c_char,
                           *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        22 => {
            SV_GetServerinfo(VM_ArgPtr(*args.offset(1isize)) as
                                 *mut libc::c_char,
                             *args.offset(2isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        28 => {
            SV_AdjustAreaPortalState(VM_ArgPtr(*args.offset(1isize)) as
                                         *mut sharedEntity_t,
                                     *args.offset(2isize) as qboolean);
            return 0i32 as intptr_t
        }
        29 => {
            return CM_AreasConnected(*args.offset(1isize) as libc::c_int,
                                     *args.offset(2isize) as libc::c_int) as
                       intptr_t
        }
        34 => { return SV_BotAllocateClient() as intptr_t }
        35 => {
            SV_BotFreeClient(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        36 => {
            SV_GetUsercmd(*args.offset(1isize) as libc::c_int,
                          VM_ArgPtr(*args.offset(2isize)) as *mut usercmd_t);
            return 0i32 as intptr_t
        }
        37 => {
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            s = COM_Parse(&mut sv.entityParsePoint);
            Q_strncpyz(VM_ArgPtr(*args.offset(1isize)) as *mut libc::c_char,
                       s, *args.offset(2isize) as libc::c_int);
            if sv.entityParsePoint.is_null() && 0 == *s.offset(0isize) {
                return qfalse as libc::c_int as intptr_t
            } else { return qtrue as libc::c_int as intptr_t }
        }
        39 => {
            return BotImport_DebugPolygonCreate(*args.offset(1isize) as
                                                    libc::c_int,
                                                *args.offset(2isize) as
                                                    libc::c_int,
                                                VM_ArgPtr(*args.offset(3isize))
                                                    as *mut vec3_t) as
                       intptr_t
        }
        40 => {
            BotImport_DebugPolygonDelete(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        41 => {
            return Com_RealTime(VM_ArgPtr(*args.offset(1isize)) as
                                    *mut qtime_t) as intptr_t
        }
        42 => {
            qsnapvectorsse(VM_ArgPtr(*args.offset(1isize)) as *mut vec_t);
            return 0i32 as intptr_t
        }
        200 => { return SV_BotLibSetup() as intptr_t }
        201 => { return SV_BotLibShutdown() as intptr_t }
        202 => {
            return (*botlib_export).BotLibVarSet.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     VM_ArgPtr(*args.offset(2isize))
                                                                                         as
                                                                                         *const libc::c_char)
                       as intptr_t
        }
        203 => {
            return (*botlib_export).BotLibVarGet.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     VM_ArgPtr(*args.offset(2isize))
                                                                                         as
                                                                                         *mut libc::c_char,
                                                                                     *args.offset(3isize)
                                                                                         as
                                                                                         libc::c_int)
                       as intptr_t
        }
        204 => {
            return (*botlib_export).PC_AddGlobalDefine.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                               as
                                                                                               *mut libc::c_char)
                       as intptr_t
        }
        578 => {
            return (*botlib_export).PC_LoadSourceHandle.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                as
                                                                                                *const libc::c_char)
                       as intptr_t
        }
        579 => {
            return (*botlib_export).PC_FreeSourceHandle.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int)
                       as intptr_t
        }
        580 => {
            return (*botlib_export).PC_ReadTokenHandle.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut pc_token_t)
                       as intptr_t
        }
        581 => {
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
        205 => {
            return (*botlib_export).BotLibStartFrame.expect("non-null function pointer")(_vmf(*args.offset(1isize)))
                       as intptr_t
        }
        206 => {
            return (*botlib_export).BotLibLoadMap.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                          as
                                                                                          *const libc::c_char)
                       as intptr_t
        }
        207 => {
            return (*botlib_export).BotLibUpdateEntity.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut bot_entitystate_t)
                       as intptr_t
        }
        208 => {
            return (*botlib_export).Test.expect("non-null function pointer")(*args.offset(1isize)
                                                                                 as
                                                                                 libc::c_int,
                                                                             VM_ArgPtr(*args.offset(2isize))
                                                                                 as
                                                                                 *mut libc::c_char,
                                                                             VM_ArgPtr(*args.offset(3isize))
                                                                                 as
                                                                                 *mut vec_t,
                                                                             VM_ArgPtr(*args.offset(4isize))
                                                                                 as
                                                                                 *mut vec_t)
                       as intptr_t
        }
        209 => {
            return SV_BotGetSnapshotEntity(*args.offset(1isize) as
                                               libc::c_int,
                                           *args.offset(2isize) as
                                               libc::c_int) as intptr_t
        }
        210 => {
            return SV_BotGetConsoleMessage(*args.offset(1isize) as
                                               libc::c_int,
                                           VM_ArgPtr(*args.offset(2isize)) as
                                               *mut libc::c_char,
                                           *args.offset(3isize) as
                                               libc::c_int) as intptr_t
        }
        211 => {
            let mut clientNum: libc::c_int =
                *args.offset(1isize) as libc::c_int;
            if clientNum >= 0i32 && clientNum < (*sv_maxclients).integer {
                SV_ClientThink(&mut *svs.clients.offset(clientNum as isize),
                               VM_ArgPtr(*args.offset(2isize)) as
                                   *mut usercmd_t);
            }
            return 0i32 as intptr_t
        }
        301 => {
            return (*botlib_export).aas.AAS_BBoxAreas.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                              as
                                                                                              *mut vec_t,
                                                                                          VM_ArgPtr(*args.offset(2isize))
                                                                                              as
                                                                                              *mut vec_t,
                                                                                          VM_ArgPtr(*args.offset(3isize))
                                                                                              as
                                                                                              *mut libc::c_int,
                                                                                          *args.offset(4isize)
                                                                                              as
                                                                                              libc::c_int)
                       as intptr_t
        }
        302 => {
            return (*botlib_export).aas.AAS_AreaInfo.expect("non-null function pointer")(*args.offset(1isize)
                                                                                             as
                                                                                             libc::c_int,
                                                                                         VM_ArgPtr(*args.offset(2isize))
                                                                                             as
                                                                                             *mut aas_areainfo_s)
                       as intptr_t
        }
        575 => {
            return (*botlib_export).aas.AAS_AlternativeRouteGoals.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                          as
                                                                                                          *mut vec_t,
                                                                                                      *args.offset(2isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      VM_ArgPtr(*args.offset(3isize))
                                                                                                          as
                                                                                                          *mut vec_t,
                                                                                                      *args.offset(4isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      *args.offset(5isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      VM_ArgPtr(*args.offset(6isize))
                                                                                                          as
                                                                                                          *mut aas_altroutegoal_s,
                                                                                                      *args.offset(7isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      *args.offset(8isize)
                                                                                                          as
                                                                                                          libc::c_int)
                       as intptr_t
        }
        303 => {
            (*botlib_export).aas.AAS_EntityInfo.expect("non-null function pointer")(*args.offset(1isize)
                                                                                        as
                                                                                        libc::c_int,
                                                                                    VM_ArgPtr(*args.offset(2isize))
                                                                                        as
                                                                                        *mut aas_entityinfo_s);
            return 0i32 as intptr_t
        }
        304 => {
            return (*botlib_export).aas.AAS_Initialized.expect("non-null function pointer")()
                       as intptr_t
        }
        305 => {
            (*botlib_export).aas.AAS_PresenceTypeBoundingBox.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                     as
                                                                                                     libc::c_int,
                                                                                                 VM_ArgPtr(*args.offset(2isize))
                                                                                                     as
                                                                                                     *mut vec_t,
                                                                                                 VM_ArgPtr(*args.offset(3isize))
                                                                                                     as
                                                                                                     *mut vec_t);
            return 0i32 as intptr_t
        }
        306 => {
            return FloatAsInt((*botlib_export).aas.AAS_Time.expect("non-null function pointer")())
                       as intptr_t
        }
        307 => {
            return (*botlib_export).aas.AAS_PointAreaNum.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                 as
                                                                                                 *mut vec_t)
                       as intptr_t
        }
        577 => {
            return (*botlib_export).aas.AAS_PointReachabilityAreaIndex.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                               as
                                                                                                               *mut vec_t)
                       as intptr_t
        }
        308 => {
            return (*botlib_export).aas.AAS_TraceAreas.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                               as
                                                                                               *mut vec_t,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut vec_t,
                                                                                           VM_ArgPtr(*args.offset(3isize))
                                                                                               as
                                                                                               *mut libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(4isize))
                                                                                               as
                                                                                               *mut vec3_t,
                                                                                           *args.offset(5isize)
                                                                                               as
                                                                                               libc::c_int)
                       as intptr_t
        }
        309 => {
            return (*botlib_export).aas.AAS_PointContents.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                  as
                                                                                                  *mut vec_t)
                       as intptr_t
        }
        310 => {
            return (*botlib_export).aas.AAS_NextBSPEntity.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                  as
                                                                                                  libc::c_int)
                       as intptr_t
        }
        311 => {
            return (*botlib_export).aas.AAS_ValueForBSPEpairKey.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    VM_ArgPtr(*args.offset(2isize))
                                                                                                        as
                                                                                                        *mut libc::c_char,
                                                                                                    VM_ArgPtr(*args.offset(3isize))
                                                                                                        as
                                                                                                        *mut libc::c_char,
                                                                                                    *args.offset(4isize)
                                                                                                        as
                                                                                                        libc::c_int)
                       as intptr_t
        }
        312 => {
            return (*botlib_export).aas.AAS_VectorForBSPEpairKey.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                         as
                                                                                                         libc::c_int,
                                                                                                     VM_ArgPtr(*args.offset(2isize))
                                                                                                         as
                                                                                                         *mut libc::c_char,
                                                                                                     VM_ArgPtr(*args.offset(3isize))
                                                                                                         as
                                                                                                         *mut vec_t)
                       as intptr_t
        }
        313 => {
            return (*botlib_export).aas.AAS_FloatForBSPEpairKey.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    VM_ArgPtr(*args.offset(2isize))
                                                                                                        as
                                                                                                        *mut libc::c_char,
                                                                                                    VM_ArgPtr(*args.offset(3isize))
                                                                                                        as
                                                                                                        *mut libc::c_float)
                       as intptr_t
        }
        314 => {
            return (*botlib_export).aas.AAS_IntForBSPEpairKey.expect("non-null function pointer")(*args.offset(1isize)
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
        315 => {
            return (*botlib_export).aas.AAS_AreaReachability.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                     as
                                                                                                     libc::c_int)
                       as intptr_t
        }
        316 => {
            return (*botlib_export).aas.AAS_AreaTravelTimeToGoalArea.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                             as
                                                                                                             libc::c_int,
                                                                                                         VM_ArgPtr(*args.offset(2isize))
                                                                                                             as
                                                                                                             *mut vec_t,
                                                                                                         *args.offset(3isize)
                                                                                                             as
                                                                                                             libc::c_int,
                                                                                                         *args.offset(4isize)
                                                                                                             as
                                                                                                             libc::c_int)
                       as intptr_t
        }
        300 => {
            return (*botlib_export).aas.AAS_EnableRoutingArea.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                      as
                                                                                                      libc::c_int,
                                                                                                  *args.offset(2isize)
                                                                                                      as
                                                                                                      libc::c_int)
                       as intptr_t
        }
        576 => {
            return (*botlib_export).aas.AAS_PredictRoute.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                 as
                                                                                                 *mut aas_predictroute_s,
                                                                                             *args.offset(2isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             VM_ArgPtr(*args.offset(3isize))
                                                                                                 as
                                                                                                 *mut vec_t,
                                                                                             *args.offset(4isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(5isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(6isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(7isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(8isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(9isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(10isize)
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             *args.offset(11isize)
                                                                                                 as
                                                                                                 libc::c_int)
                       as intptr_t
        }
        317 => {
            return (*botlib_export).aas.AAS_Swimming.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                             as
                                                                                             *mut vec_t)
                       as intptr_t
        }
        318 => {
            return (*botlib_export).aas.AAS_PredictClientMovement.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                          as
                                                                                                          *mut aas_clientmove_s,
                                                                                                      *args.offset(2isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      VM_ArgPtr(*args.offset(3isize))
                                                                                                          as
                                                                                                          *mut vec_t,
                                                                                                      *args.offset(4isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      *args.offset(5isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      VM_ArgPtr(*args.offset(6isize))
                                                                                                          as
                                                                                                          *mut vec_t,
                                                                                                      VM_ArgPtr(*args.offset(7isize))
                                                                                                          as
                                                                                                          *mut vec_t,
                                                                                                      *args.offset(8isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      *args.offset(9isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      _vmf(*args.offset(10isize)),
                                                                                                      *args.offset(11isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      *args.offset(12isize)
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      *args.offset(13isize)
                                                                                                          as
                                                                                                          libc::c_int)
                       as intptr_t
        }
        400 => {
            (*botlib_export).ea.EA_Say.expect("non-null function pointer")(*args.offset(1isize)
                                                                               as
                                                                               libc::c_int,
                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                               as
                                                                               *mut libc::c_char);
            return 0i32 as intptr_t
        }
        401 => {
            (*botlib_export).ea.EA_SayTeam.expect("non-null function pointer")(*args.offset(1isize)
                                                                                   as
                                                                                   libc::c_int,
                                                                               VM_ArgPtr(*args.offset(2isize))
                                                                                   as
                                                                                   *mut libc::c_char);
            return 0i32 as intptr_t
        }
        402 => {
            (*botlib_export).ea.EA_Command.expect("non-null function pointer")(*args.offset(1isize)
                                                                                   as
                                                                                   libc::c_int,
                                                                               VM_ArgPtr(*args.offset(2isize))
                                                                                   as
                                                                                   *mut libc::c_char);
            return 0i32 as intptr_t
        }
        403 => {
            (*botlib_export).ea.EA_Action.expect("non-null function pointer")(*args.offset(1isize)
                                                                                  as
                                                                                  libc::c_int,
                                                                              *args.offset(2isize)
                                                                                  as
                                                                                  libc::c_int);
            return 0i32 as intptr_t
        }
        404 => {
            (*botlib_export).ea.EA_Gesture.expect("non-null function pointer")(*args.offset(1isize)
                                                                                   as
                                                                                   libc::c_int);
            return 0i32 as intptr_t
        }
        405 => {
            (*botlib_export).ea.EA_Talk.expect("non-null function pointer")(*args.offset(1isize)
                                                                                as
                                                                                libc::c_int);
            return 0i32 as intptr_t
        }
        406 => {
            (*botlib_export).ea.EA_Attack.expect("non-null function pointer")(*args.offset(1isize)
                                                                                  as
                                                                                  libc::c_int);
            return 0i32 as intptr_t
        }
        407 => {
            (*botlib_export).ea.EA_Use.expect("non-null function pointer")(*args.offset(1isize)
                                                                               as
                                                                               libc::c_int);
            return 0i32 as intptr_t
        }
        408 => {
            (*botlib_export).ea.EA_Respawn.expect("non-null function pointer")(*args.offset(1isize)
                                                                                   as
                                                                                   libc::c_int);
            return 0i32 as intptr_t
        }
        409 => {
            (*botlib_export).ea.EA_Crouch.expect("non-null function pointer")(*args.offset(1isize)
                                                                                  as
                                                                                  libc::c_int);
            return 0i32 as intptr_t
        }
        410 => {
            (*botlib_export).ea.EA_MoveUp.expect("non-null function pointer")(*args.offset(1isize)
                                                                                  as
                                                                                  libc::c_int);
            return 0i32 as intptr_t
        }
        411 => {
            (*botlib_export).ea.EA_MoveDown.expect("non-null function pointer")(*args.offset(1isize)
                                                                                    as
                                                                                    libc::c_int);
            return 0i32 as intptr_t
        }
        412 => {
            (*botlib_export).ea.EA_MoveForward.expect("non-null function pointer")(*args.offset(1isize)
                                                                                       as
                                                                                       libc::c_int);
            return 0i32 as intptr_t
        }
        413 => {
            (*botlib_export).ea.EA_MoveBack.expect("non-null function pointer")(*args.offset(1isize)
                                                                                    as
                                                                                    libc::c_int);
            return 0i32 as intptr_t
        }
        414 => {
            (*botlib_export).ea.EA_MoveLeft.expect("non-null function pointer")(*args.offset(1isize)
                                                                                    as
                                                                                    libc::c_int);
            return 0i32 as intptr_t
        }
        415 => {
            (*botlib_export).ea.EA_MoveRight.expect("non-null function pointer")(*args.offset(1isize)
                                                                                     as
                                                                                     libc::c_int);
            return 0i32 as intptr_t
        }
        416 => {
            (*botlib_export).ea.EA_SelectWeapon.expect("non-null function pointer")(*args.offset(1isize)
                                                                                        as
                                                                                        libc::c_int,
                                                                                    *args.offset(2isize)
                                                                                        as
                                                                                        libc::c_int);
            return 0i32 as intptr_t
        }
        417 => {
            (*botlib_export).ea.EA_Jump.expect("non-null function pointer")(*args.offset(1isize)
                                                                                as
                                                                                libc::c_int);
            return 0i32 as intptr_t
        }
        418 => {
            (*botlib_export).ea.EA_DelayedJump.expect("non-null function pointer")(*args.offset(1isize)
                                                                                       as
                                                                                       libc::c_int);
            return 0i32 as intptr_t
        }
        419 => {
            (*botlib_export).ea.EA_Move.expect("non-null function pointer")(*args.offset(1isize)
                                                                                as
                                                                                libc::c_int,
                                                                            VM_ArgPtr(*args.offset(2isize))
                                                                                as
                                                                                *mut vec_t,
                                                                            _vmf(*args.offset(3isize)));
            return 0i32 as intptr_t
        }
        420 => {
            (*botlib_export).ea.EA_View.expect("non-null function pointer")(*args.offset(1isize)
                                                                                as
                                                                                libc::c_int,
                                                                            VM_ArgPtr(*args.offset(2isize))
                                                                                as
                                                                                *mut vec_t);
            return 0i32 as intptr_t
        }
        421 => {
            (*botlib_export).ea.EA_EndRegular.expect("non-null function pointer")(*args.offset(1isize)
                                                                                      as
                                                                                      libc::c_int,
                                                                                  _vmf(*args.offset(2isize)));
            return 0i32 as intptr_t
        }
        422 => {
            (*botlib_export).ea.EA_GetInput.expect("non-null function pointer")(*args.offset(1isize)
                                                                                    as
                                                                                    libc::c_int,
                                                                                _vmf(*args.offset(2isize)),
                                                                                VM_ArgPtr(*args.offset(3isize))
                                                                                    as
                                                                                    *mut bot_input_t);
            return 0i32 as intptr_t
        }
        423 => {
            (*botlib_export).ea.EA_ResetInput.expect("non-null function pointer")(*args.offset(1isize)
                                                                                      as
                                                                                      libc::c_int);
            return 0i32 as intptr_t
        }
        500 => {
            return (*botlib_export).ai.BotLoadCharacter.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                as
                                                                                                *mut libc::c_char,
                                                                                            _vmf(*args.offset(2isize)))
                       as intptr_t
        }
        501 => {
            (*botlib_export).ai.BotFreeCharacter.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        502 => {
            return FloatAsInt((*botlib_export).ai.Characteristic_Float.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                               as
                                                                                                               libc::c_int,
                                                                                                           *args.offset(2isize)
                                                                                                               as
                                                                                                               libc::c_int))
                       as intptr_t
        }
        503 => {
            return FloatAsInt((*botlib_export).ai.Characteristic_BFloat.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                                as
                                                                                                                libc::c_int,
                                                                                                            *args.offset(2isize)
                                                                                                                as
                                                                                                                libc::c_int,
                                                                                                            _vmf(*args.offset(3isize)),
                                                                                                            _vmf(*args.offset(4isize))))
                       as intptr_t
        }
        504 => {
            return (*botlib_export).ai.Characteristic_Integer.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                      as
                                                                                                      libc::c_int,
                                                                                                  *args.offset(2isize)
                                                                                                      as
                                                                                                      libc::c_int)
                       as intptr_t
        }
        505 => {
            return (*botlib_export).ai.Characteristic_BInteger.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                       as
                                                                                                       libc::c_int,
                                                                                                   *args.offset(2isize)
                                                                                                       as
                                                                                                       libc::c_int,
                                                                                                   *args.offset(3isize)
                                                                                                       as
                                                                                                       libc::c_int,
                                                                                                   *args.offset(4isize)
                                                                                                       as
                                                                                                       libc::c_int)
                       as intptr_t
        }
        506 => {
            (*botlib_export).ai.Characteristic_String.expect("non-null function pointer")(*args.offset(1isize)
                                                                                              as
                                                                                              libc::c_int,
                                                                                          *args.offset(2isize)
                                                                                              as
                                                                                              libc::c_int,
                                                                                          VM_ArgPtr(*args.offset(3isize))
                                                                                              as
                                                                                              *mut libc::c_char,
                                                                                          *args.offset(4isize)
                                                                                              as
                                                                                              libc::c_int);
            return 0i32 as intptr_t
        }
        507 => {
            return (*botlib_export).ai.BotAllocChatState.expect("non-null function pointer")()
                       as intptr_t
        }
        508 => {
            (*botlib_export).ai.BotFreeChatState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        509 => {
            (*botlib_export).ai.BotQueueConsoleMessage.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           *args.offset(2isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(3isize))
                                                                                               as
                                                                                               *mut libc::c_char);
            return 0i32 as intptr_t
        }
        510 => {
            (*botlib_export).ai.BotRemoveConsoleMessage.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            *args.offset(2isize)
                                                                                                as
                                                                                                libc::c_int);
            return 0i32 as intptr_t
        }
        511 => {
            return (*botlib_export).ai.BotNextConsoleMessage.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                     as
                                                                                                     libc::c_int,
                                                                                                 VM_ArgPtr(*args.offset(2isize))
                                                                                                     as
                                                                                                     *mut bot_consolemessage_s)
                       as intptr_t
        }
        512 => {
            return (*botlib_export).ai.BotNumConsoleMessages.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                     as
                                                                                                     libc::c_int)
                       as intptr_t
        }
        513 => {
            (*botlib_export).ai.BotInitialChat.expect("non-null function pointer")(*args.offset(1isize)
                                                                                       as
                                                                                       libc::c_int,
                                                                                   VM_ArgPtr(*args.offset(2isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   *args.offset(3isize)
                                                                                       as
                                                                                       libc::c_int,
                                                                                   VM_ArgPtr(*args.offset(4isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(5isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(6isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(7isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(8isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(9isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(10isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   VM_ArgPtr(*args.offset(11isize))
                                                                                       as
                                                                                       *mut libc::c_char);
            return 0i32 as intptr_t
        }
        569 => {
            return (*botlib_export).ai.BotNumInitialChats.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              VM_ArgPtr(*args.offset(2isize))
                                                                                                  as
                                                                                                  *mut libc::c_char)
                       as intptr_t
        }
        514 => {
            return (*botlib_export).ai.BotReplyChat.expect("non-null function pointer")(*args.offset(1isize)
                                                                                            as
                                                                                            libc::c_int,
                                                                                        VM_ArgPtr(*args.offset(2isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        *args.offset(3isize)
                                                                                            as
                                                                                            libc::c_int,
                                                                                        *args.offset(4isize)
                                                                                            as
                                                                                            libc::c_int,
                                                                                        VM_ArgPtr(*args.offset(5isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(6isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(7isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(8isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(9isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(10isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(11isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(12isize))
                                                                                            as
                                                                                            *mut libc::c_char)
                       as intptr_t
        }
        515 => {
            return (*botlib_export).ai.BotChatLength.expect("non-null function pointer")(*args.offset(1isize)
                                                                                             as
                                                                                             libc::c_int)
                       as intptr_t
        }
        516 => {
            (*botlib_export).ai.BotEnterChat.expect("non-null function pointer")(*args.offset(1isize)
                                                                                     as
                                                                                     libc::c_int,
                                                                                 *args.offset(2isize)
                                                                                     as
                                                                                     libc::c_int,
                                                                                 *args.offset(3isize)
                                                                                     as
                                                                                     libc::c_int);
            return 0i32 as intptr_t
        }
        570 => {
            (*botlib_export).ai.BotGetChatMessage.expect("non-null function pointer")(*args.offset(1isize)
                                                                                          as
                                                                                          libc::c_int,
                                                                                      VM_ArgPtr(*args.offset(2isize))
                                                                                          as
                                                                                          *mut libc::c_char,
                                                                                      *args.offset(3isize)
                                                                                          as
                                                                                          libc::c_int);
            return 0i32 as intptr_t
        }
        517 => {
            return (*botlib_export).ai.StringContains.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                              as
                                                                                              *mut libc::c_char,
                                                                                          VM_ArgPtr(*args.offset(2isize))
                                                                                              as
                                                                                              *mut libc::c_char,
                                                                                          *args.offset(3isize)
                                                                                              as
                                                                                              libc::c_int)
                       as intptr_t
        }
        518 => {
            return (*botlib_export).ai.BotFindMatch.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        VM_ArgPtr(*args.offset(2isize))
                                                                                            as
                                                                                            *mut bot_match_s,
                                                                                        *args.offset(3isize)
                                                                                            as
                                                                                            libc::c_ulong)
                       as intptr_t
        }
        519 => {
            (*botlib_export).ai.BotMatchVariable.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                         as
                                                                                         *mut bot_match_s,
                                                                                     *args.offset(2isize)
                                                                                         as
                                                                                         libc::c_int,
                                                                                     VM_ArgPtr(*args.offset(3isize))
                                                                                         as
                                                                                         *mut libc::c_char,
                                                                                     *args.offset(4isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        520 => {
            (*botlib_export).ai.UnifyWhiteSpaces.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                         as
                                                                                         *mut libc::c_char);
            return 0i32 as intptr_t
        }
        521 => {
            (*botlib_export).ai.BotReplaceSynonyms.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                           as
                                                                                           *mut libc::c_char,
                                                                                       *args.offset(2isize)
                                                                                           as
                                                                                           libc::c_ulong);
            return 0i32 as intptr_t
        }
        522 => {
            return (*botlib_export).ai.BotLoadChatFile.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut libc::c_char,
                                                                                           VM_ArgPtr(*args.offset(3isize))
                                                                                               as
                                                                                               *mut libc::c_char)
                       as intptr_t
        }
        523 => {
            (*botlib_export).ai.BotSetChatGender.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int,
                                                                                     *args.offset(2isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        524 => {
            (*botlib_export).ai.BotSetChatName.expect("non-null function pointer")(*args.offset(1isize)
                                                                                       as
                                                                                       libc::c_int,
                                                                                   VM_ArgPtr(*args.offset(2isize))
                                                                                       as
                                                                                       *mut libc::c_char,
                                                                                   *args.offset(3isize)
                                                                                       as
                                                                                       libc::c_int);
            return 0i32 as intptr_t
        }
        525 => {
            (*botlib_export).ai.BotResetGoalState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                          as
                                                                                          libc::c_int);
            return 0i32 as intptr_t
        }
        526 => {
            (*botlib_export).ai.BotResetAvoidGoals.expect("non-null function pointer")(*args.offset(1isize)
                                                                                           as
                                                                                           libc::c_int);
            return 0i32 as intptr_t
        }
        571 => {
            (*botlib_export).ai.BotRemoveFromAvoidGoals.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            *args.offset(2isize)
                                                                                                as
                                                                                                libc::c_int);
            return 0i32 as intptr_t
        }
        527 => {
            (*botlib_export).ai.BotPushGoal.expect("non-null function pointer")(*args.offset(1isize)
                                                                                    as
                                                                                    libc::c_int,
                                                                                VM_ArgPtr(*args.offset(2isize))
                                                                                    as
                                                                                    *mut bot_goal_s);
            return 0i32 as intptr_t
        }
        528 => {
            (*botlib_export).ai.BotPopGoal.expect("non-null function pointer")(*args.offset(1isize)
                                                                                   as
                                                                                   libc::c_int);
            return 0i32 as intptr_t
        }
        529 => {
            (*botlib_export).ai.BotEmptyGoalStack.expect("non-null function pointer")(*args.offset(1isize)
                                                                                          as
                                                                                          libc::c_int);
            return 0i32 as intptr_t
        }
        530 => {
            (*botlib_export).ai.BotDumpAvoidGoals.expect("non-null function pointer")(*args.offset(1isize)
                                                                                          as
                                                                                          libc::c_int);
            return 0i32 as intptr_t
        }
        531 => {
            (*botlib_export).ai.BotDumpGoalStack.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        532 => {
            (*botlib_export).ai.BotGoalName.expect("non-null function pointer")(*args.offset(1isize)
                                                                                    as
                                                                                    libc::c_int,
                                                                                VM_ArgPtr(*args.offset(2isize))
                                                                                    as
                                                                                    *mut libc::c_char,
                                                                                *args.offset(3isize)
                                                                                    as
                                                                                    libc::c_int);
            return 0i32 as intptr_t
        }
        533 => {
            return (*botlib_export).ai.BotGetTopGoal.expect("non-null function pointer")(*args.offset(1isize)
                                                                                             as
                                                                                             libc::c_int,
                                                                                         VM_ArgPtr(*args.offset(2isize))
                                                                                             as
                                                                                             *mut bot_goal_s)
                       as intptr_t
        }
        534 => {
            return (*botlib_export).ai.BotGetSecondGoal.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            VM_ArgPtr(*args.offset(2isize))
                                                                                                as
                                                                                                *mut bot_goal_s)
                       as intptr_t
        }
        535 => {
            return (*botlib_export).ai.BotChooseLTGItem.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            VM_ArgPtr(*args.offset(2isize))
                                                                                                as
                                                                                                *mut vec_t,
                                                                                            VM_ArgPtr(*args.offset(3isize))
                                                                                                as
                                                                                                *mut libc::c_int,
                                                                                            *args.offset(4isize)
                                                                                                as
                                                                                                libc::c_int)
                       as intptr_t
        }
        536 => {
            return (*botlib_export).ai.BotChooseNBGItem.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            VM_ArgPtr(*args.offset(2isize))
                                                                                                as
                                                                                                *mut vec_t,
                                                                                            VM_ArgPtr(*args.offset(3isize))
                                                                                                as
                                                                                                *mut libc::c_int,
                                                                                            *args.offset(4isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            VM_ArgPtr(*args.offset(5isize))
                                                                                                as
                                                                                                *mut bot_goal_s,
                                                                                            _vmf(*args.offset(6isize)))
                       as intptr_t
        }
        537 => {
            return (*botlib_export).ai.BotTouchingGoal.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                               as
                                                                                               *mut vec_t,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut bot_goal_s)
                       as intptr_t
        }
        538 => {
            return (*botlib_export).ai.BotItemGoalInVisButNotVisible.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                             as
                                                                                                             libc::c_int,
                                                                                                         VM_ArgPtr(*args.offset(2isize))
                                                                                                             as
                                                                                                             *mut vec_t,
                                                                                                         VM_ArgPtr(*args.offset(3isize))
                                                                                                             as
                                                                                                             *mut vec_t,
                                                                                                         VM_ArgPtr(*args.offset(4isize))
                                                                                                             as
                                                                                                             *mut bot_goal_s)
                       as intptr_t
        }
        539 => {
            return (*botlib_export).ai.BotGetLevelItemGoal.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                   as
                                                                                                   libc::c_int,
                                                                                               VM_ArgPtr(*args.offset(2isize))
                                                                                                   as
                                                                                                   *mut libc::c_char,
                                                                                               VM_ArgPtr(*args.offset(3isize))
                                                                                                   as
                                                                                                   *mut bot_goal_s)
                       as intptr_t
        }
        567 => {
            return (*botlib_export).ai.BotGetNextCampSpotGoal.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                      as
                                                                                                      libc::c_int,
                                                                                                  VM_ArgPtr(*args.offset(2isize))
                                                                                                      as
                                                                                                      *mut bot_goal_s)
                       as intptr_t
        }
        568 => {
            return (*botlib_export).ai.BotGetMapLocationGoal.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                     as
                                                                                                     *mut libc::c_char,
                                                                                                 VM_ArgPtr(*args.offset(2isize))
                                                                                                     as
                                                                                                     *mut bot_goal_s)
                       as intptr_t
        }
        540 => {
            return FloatAsInt((*botlib_export).ai.BotAvoidGoalTime.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                           as
                                                                                                           libc::c_int,
                                                                                                       *args.offset(2isize)
                                                                                                           as
                                                                                                           libc::c_int))
                       as intptr_t
        }
        573 => {
            (*botlib_export).ai.BotSetAvoidGoalTime.expect("non-null function pointer")(*args.offset(1isize)
                                                                                            as
                                                                                            libc::c_int,
                                                                                        *args.offset(2isize)
                                                                                            as
                                                                                            libc::c_int,
                                                                                        _vmf(*args.offset(3isize)));
            return 0i32 as intptr_t
        }
        541 => {
            (*botlib_export).ai.BotInitLevelItems.expect("non-null function pointer")();
            return 0i32 as intptr_t
        }
        542 => {
            (*botlib_export).ai.BotUpdateEntityItems.expect("non-null function pointer")();
            return 0i32 as intptr_t
        }
        543 => {
            return (*botlib_export).ai.BotLoadItemWeights.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              VM_ArgPtr(*args.offset(2isize))
                                                                                                  as
                                                                                                  *mut libc::c_char)
                       as intptr_t
        }
        544 => {
            (*botlib_export).ai.BotFreeItemWeights.expect("non-null function pointer")(*args.offset(1isize)
                                                                                           as
                                                                                           libc::c_int);
            return 0i32 as intptr_t
        }
        565 => {
            (*botlib_export).ai.BotInterbreedGoalFuzzyLogic.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                    as
                                                                                                    libc::c_int,
                                                                                                *args.offset(2isize)
                                                                                                    as
                                                                                                    libc::c_int,
                                                                                                *args.offset(3isize)
                                                                                                    as
                                                                                                    libc::c_int);
            return 0i32 as intptr_t
        }
        545 => {
            (*botlib_export).ai.BotSaveGoalFuzzyLogic.expect("non-null function pointer")(*args.offset(1isize)
                                                                                              as
                                                                                              libc::c_int,
                                                                                          VM_ArgPtr(*args.offset(2isize))
                                                                                              as
                                                                                              *mut libc::c_char);
            return 0i32 as intptr_t
        }
        566 => {
            (*botlib_export).ai.BotMutateGoalFuzzyLogic.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int,
                                                                                            _vmf(*args.offset(2isize)));
            return 0i32 as intptr_t
        }
        546 => {
            return (*botlib_export).ai.BotAllocGoalState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                 as
                                                                                                 libc::c_int)
                       as intptr_t
        }
        547 => {
            (*botlib_export).ai.BotFreeGoalState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        548 => {
            (*botlib_export).ai.BotResetMoveState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                          as
                                                                                          libc::c_int);
            return 0i32 as intptr_t
        }
        574 => {
            (*botlib_export).ai.BotAddAvoidSpot.expect("non-null function pointer")(*args.offset(1isize)
                                                                                        as
                                                                                        libc::c_int,
                                                                                    VM_ArgPtr(*args.offset(2isize))
                                                                                        as
                                                                                        *mut vec_t,
                                                                                    _vmf(*args.offset(3isize)),
                                                                                    *args.offset(4isize)
                                                                                        as
                                                                                        libc::c_int);
            return 0i32 as intptr_t
        }
        549 => {
            (*botlib_export).ai.BotMoveToGoal.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                      as
                                                                                      *mut bot_moveresult_s,
                                                                                  *args.offset(2isize)
                                                                                      as
                                                                                      libc::c_int,
                                                                                  VM_ArgPtr(*args.offset(3isize))
                                                                                      as
                                                                                      *mut bot_goal_s,
                                                                                  *args.offset(4isize)
                                                                                      as
                                                                                      libc::c_int);
            return 0i32 as intptr_t
        }
        550 => {
            return (*botlib_export).ai.BotMoveInDirection.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              VM_ArgPtr(*args.offset(2isize))
                                                                                                  as
                                                                                                  *mut vec_t,
                                                                                              _vmf(*args.offset(3isize)),
                                                                                              *args.offset(4isize)
                                                                                                  as
                                                                                                  libc::c_int)
                       as intptr_t
        }
        551 => {
            (*botlib_export).ai.BotResetAvoidReach.expect("non-null function pointer")(*args.offset(1isize)
                                                                                           as
                                                                                           libc::c_int);
            return 0i32 as intptr_t
        }
        552 => {
            (*botlib_export).ai.BotResetLastAvoidReach.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int);
            return 0i32 as intptr_t
        }
        553 => {
            return (*botlib_export).ai.BotReachabilityArea.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                   as
                                                                                                   *mut vec_t,
                                                                                               *args.offset(2isize)
                                                                                                   as
                                                                                                   libc::c_int)
                       as intptr_t
        }
        554 => {
            return (*botlib_export).ai.BotMovementViewTarget.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                     as
                                                                                                     libc::c_int,
                                                                                                 VM_ArgPtr(*args.offset(2isize))
                                                                                                     as
                                                                                                     *mut bot_goal_s,
                                                                                                 *args.offset(3isize)
                                                                                                     as
                                                                                                     libc::c_int,
                                                                                                 _vmf(*args.offset(4isize)),
                                                                                                 VM_ArgPtr(*args.offset(5isize))
                                                                                                     as
                                                                                                     *mut vec_t)
                       as intptr_t
        }
        572 => {
            return (*botlib_export).ai.BotPredictVisiblePosition.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                         as
                                                                                                         *mut vec_t,
                                                                                                     *args.offset(2isize)
                                                                                                         as
                                                                                                         libc::c_int,
                                                                                                     VM_ArgPtr(*args.offset(3isize))
                                                                                                         as
                                                                                                         *mut bot_goal_s,
                                                                                                     *args.offset(4isize)
                                                                                                         as
                                                                                                         libc::c_int,
                                                                                                     VM_ArgPtr(*args.offset(5isize))
                                                                                                         as
                                                                                                         *mut vec_t)
                       as intptr_t
        }
        555 => {
            return (*botlib_export).ai.BotAllocMoveState.expect("non-null function pointer")()
                       as intptr_t
        }
        556 => {
            (*botlib_export).ai.BotFreeMoveState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int);
            return 0i32 as intptr_t
        }
        557 => {
            (*botlib_export).ai.BotInitMoveState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int,
                                                                                     VM_ArgPtr(*args.offset(2isize))
                                                                                         as
                                                                                         *mut bot_initmove_s);
            return 0i32 as intptr_t
        }
        558 => {
            return (*botlib_export).ai.BotChooseBestFightWeapon.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    VM_ArgPtr(*args.offset(2isize))
                                                                                                        as
                                                                                                        *mut libc::c_int)
                       as intptr_t
        }
        559 => {
            (*botlib_export).ai.BotGetWeaponInfo.expect("non-null function pointer")(*args.offset(1isize)
                                                                                         as
                                                                                         libc::c_int,
                                                                                     *args.offset(2isize)
                                                                                         as
                                                                                         libc::c_int,
                                                                                     VM_ArgPtr(*args.offset(3isize))
                                                                                         as
                                                                                         *mut weaponinfo_s);
            return 0i32 as intptr_t
        }
        560 => {
            return (*botlib_export).ai.BotLoadWeaponWeights.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                    as
                                                                                                    libc::c_int,
                                                                                                VM_ArgPtr(*args.offset(2isize))
                                                                                                    as
                                                                                                    *mut libc::c_char)
                       as intptr_t
        }
        561 => {
            return (*botlib_export).ai.BotAllocWeaponState.expect("non-null function pointer")()
                       as intptr_t
        }
        562 => {
            (*botlib_export).ai.BotFreeWeaponState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                           as
                                                                                           libc::c_int);
            return 0i32 as intptr_t
        }
        563 => {
            (*botlib_export).ai.BotResetWeaponState.expect("non-null function pointer")(*args.offset(1isize)
                                                                                            as
                                                                                            libc::c_int);
            return 0i32 as intptr_t
        }
        564 => {
            return (*botlib_export).ai.GeneticParentsAndChildSelection.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                               as
                                                                                                               libc::c_int,
                                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                                               as
                                                                                                               *mut libc::c_float,
                                                                                                           VM_ArgPtr(*args.offset(3isize))
                                                                                                               as
                                                                                                               *mut libc::c_int,
                                                                                                           VM_ArgPtr(*args.offset(4isize))
                                                                                                               as
                                                                                                               *mut libc::c_int,
                                                                                                           VM_ArgPtr(*args.offset(5isize))
                                                                                                               as
                                                                                                               *mut libc::c_int)
                       as intptr_t
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
            MatrixMultiply(VM_ArgPtr(*args.offset(1isize)) as
                               *mut [libc::c_float; 3],
                           VM_ArgPtr(*args.offset(2isize)) as
                               *mut [libc::c_float; 3],
                           VM_ArgPtr(*args.offset(3isize)) as
                               *mut [libc::c_float; 3]);
            return 0i32 as intptr_t
        }
        108 => {
            AngleVectors(VM_ArgPtr(*args.offset(1isize)) as *const vec_t,
                         VM_ArgPtr(*args.offset(2isize)) as *mut vec_t,
                         VM_ArgPtr(*args.offset(3isize)) as *mut vec_t,
                         VM_ArgPtr(*args.offset(4isize)) as *mut vec_t);
            return 0i32 as intptr_t
        }
        109 => {
            PerpendicularVector(VM_ArgPtr(*args.offset(1isize)) as *mut vec_t,
                                VM_ArgPtr(*args.offset(2isize)) as
                                    *const vec_t);
            return 0i32 as intptr_t
        }
        110 => {
            return FloatAsInt(floor(_vmf(*args.offset(1isize)) as
                                        libc::c_double) as libc::c_float) as
                       intptr_t
        }
        111 => {
            return FloatAsInt(ceil(_vmf(*args.offset(1isize)) as
                                       libc::c_double) as libc::c_float) as
                       intptr_t
        }
        _ => {
            Com_Error(ERR_DROP as libc::c_int,
                      b"Bad game system trap: %ld\x00" as *const u8 as
                          *const libc::c_char, *args.offset(0isize));
        }
    };
}
//==============================================
unsafe extern "C" fn FloatAsInt(mut f: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.f = f;
    return fi.i;
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
// sv_game.c -- interface to the game dll
#[no_mangle]
pub static mut botlib_export: *mut botlib_export_t =
    0 as *const botlib_export_t as *mut botlib_export_t;
/*
===============
SV_GetUsercmd

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetUsercmd(mut clientNum: libc::c_int,
                                       mut cmd: *mut usercmd_t) {
    if clientNum < 0i32 || clientNum >= (*sv_maxclients).integer {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_GetUsercmd: bad clientNum:%i\x00" as *const u8 as
                      *const libc::c_char, clientNum);
    }
    *cmd = (*svs.clients.offset(clientNum as isize)).lastUsercmd;
}
/*
========================
SV_AdjustAreaPortalState
========================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AdjustAreaPortalState(mut ent:
                                                      *mut sharedEntity_t,
                                                  mut open: qboolean) {
    let mut svEnt: *mut svEntity_t = 0 as *mut svEntity_t;
    svEnt = SV_SvEntityForGentity(ent);
    if (*svEnt).areanum2 == -1i32 { return }
    CM_AdjustAreaPortalState((*svEnt).areanum, (*svEnt).areanum2, open);
}
/*
===============
SV_GetServerinfo

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetServerinfo(mut buffer: *mut libc::c_char,
                                          mut bufferSize: libc::c_int) {
    if bufferSize < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_GetServerinfo: bufferSize == %i\x00" as *const u8 as
                      *const libc::c_char, bufferSize);
    }
    Q_strncpyz(buffer, Cvar_InfoString(0x4i32), bufferSize);
}
/*
=================
SV_inPVSIgnorePortals

Does NOT check portalareas
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_inPVSIgnorePortals(mut p1: *const vec_t,
                                               mut p2: *const vec_t)
 -> qboolean {
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    leafnum = CM_PointLeafnum(p1);
    cluster = CM_LeafCluster(leafnum);
    mask = CM_ClusterPVS(cluster);
    leafnum = CM_PointLeafnum(p2);
    cluster = CM_LeafCluster(leafnum);
    if !mask.is_null() &&
           0 ==
               *mask.offset((cluster >> 3i32) as isize) as libc::c_int &
                   1i32 << (cluster & 7i32) {
        return qfalse
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn SV_inPVS(mut p1: *const vec_t, mut p2: *const vec_t)
 -> qboolean {
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut area1: libc::c_int = 0;
    let mut area2: libc::c_int = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    leafnum = CM_PointLeafnum(p1);
    cluster = CM_LeafCluster(leafnum);
    area1 = CM_LeafArea(leafnum);
    mask = CM_ClusterPVS(cluster);
    leafnum = CM_PointLeafnum(p2);
    cluster = CM_LeafCluster(leafnum);
    area2 = CM_LeafArea(leafnum);
    if !mask.is_null() &&
           0 ==
               *mask.offset((cluster >> 3i32) as isize) as libc::c_int &
                   1i32 << (cluster & 7i32) {
        return qfalse
    }
    if 0 == CM_AreasConnected(area1, area2) as u64 { return qfalse }
    return qtrue;
}
/*
=================
SV_SetBrushModel

sets mins and maxs for inline bmodels
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SetBrushModel(mut ent: *mut sharedEntity_t,
                                          mut name: *const libc::c_char) {
    let mut h: clipHandle_t = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    if name.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_SetBrushModel: NULL\x00" as *const u8 as
                      *const libc::c_char);
    }
    if *name.offset(0isize) as libc::c_int != '*' as i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_SetBrushModel: %s isn\'t a brush model\x00" as
                      *const u8 as *const libc::c_char, name);
    }
    (*ent).s.modelindex = atoi(name.offset(1isize));
    h = CM_InlineModel((*ent).s.modelindex);
    CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    (*ent).r.mins[0usize] = mins[0usize];
    (*ent).r.mins[1usize] = mins[1usize];
    (*ent).r.mins[2usize] = mins[2usize];
    (*ent).r.maxs[0usize] = maxs[0usize];
    (*ent).r.maxs[1usize] = maxs[1usize];
    (*ent).r.maxs[2usize] = maxs[2usize];
    (*ent).r.bmodel = qtrue;
    (*ent).r.contents = -1i32;
    SV_LinkEntity(ent);
}
/*
==================
SV_EntityContact
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EntityContact(mut mins: *mut vec_t,
                                          mut maxs: *mut vec_t,
                                          mut gEnt: *const sharedEntity_t,
                                          mut capsule: libc::c_int)
 -> qboolean {
    let mut origin: *const libc::c_float = 0 as *const libc::c_float;
    let mut angles: *const libc::c_float = 0 as *const libc::c_float;
    let mut ch: clipHandle_t = 0;
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
    origin = (*gEnt).r.currentOrigin.as_ptr();
    angles = (*gEnt).r.currentAngles.as_ptr();
    ch = SV_ClipHandleForEntity(gEnt);
    CM_TransformedBoxTrace(&mut trace,
                           vec3_origin.as_mut_ptr() as *const vec_t,
                           vec3_origin.as_mut_ptr() as *const vec_t, mins,
                           maxs, ch, -1i32, origin, angles, capsule);
    return trace.startsolid;
}
/*
===============
SV_GameSendServerCommand

Sends a command string to a client
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GameSendServerCommand(mut clientNum: libc::c_int,
                                                  mut text:
                                                      *const libc::c_char) {
    if clientNum == -1i32 {
        SV_SendServerCommand(0 as *mut client_t,
                             b"%s\x00" as *const u8 as *const libc::c_char,
                             text);
    } else {
        if clientNum < 0i32 || clientNum >= (*sv_maxclients).integer {
            return
        }
        SV_SendServerCommand(svs.clients.offset(clientNum as isize),
                             b"%s\x00" as *const u8 as *const libc::c_char,
                             text);
    };
}
/*
===============
SV_GameDropClient

Disconnects the client with a message
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GameDropClient(mut clientNum: libc::c_int,
                                           mut reason: *const libc::c_char) {
    if clientNum < 0i32 || clientNum >= (*sv_maxclients).integer { return }
    SV_DropClient(svs.clients.offset(clientNum as isize), reason);
}
/*
===============
SV_LocateGameData

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LocateGameData(mut gEnts: *mut sharedEntity_t,
                                           mut numGEntities: libc::c_int,
                                           mut sizeofGEntity_t: libc::c_int,
                                           mut clients: *mut playerState_t,
                                           mut sizeofGameClient:
                                               libc::c_int) {
    sv.gentities = gEnts;
    sv.gentitySize = sizeofGEntity_t;
    sv.num_entities = numGEntities;
    sv.gameClients = clients;
    sv.gameClientSize = sizeofGameClient;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ShutdownGameProgs() {
    if gvm.is_null() { return }
    VM_Call(gvm, GAME_SHUTDOWN as libc::c_int, qfalse as libc::c_int);
    VM_Free(gvm);
    gvm = 0 as *mut vm_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_RestartGameProgs() {
    if gvm.is_null() { return }
    VM_Call(gvm, GAME_SHUTDOWN as libc::c_int, qtrue as libc::c_int);
    gvm = VM_Restart(gvm, qtrue);
    if gvm.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_Restart on game failed\x00" as *const u8 as
                      *const libc::c_char);
    }
    SV_InitGameVM(qtrue);
}