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
    // expand constants before stringifying them
    // angle indexes
    // up / down
    // left / right
    // fall over
    // the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
    // max length of a string passed to Cmd_TokenizeString
    // max tokens resulting from Cmd_TokenizeString
    // max length of an individual token
    // used for system info key only
    // max length of a quake game pathname
    // max length of a client name
    // parameters for command buffer stuffing
    pub type unnamed = libc::c_uint;
    // add to end of the command buffer (normal case)
    pub const EXEC_APPEND: unnamed = 2;
    // because some commands might cause the VM to be unloaded...
    // insert at current position, but don't run yet
    pub const EXEC_INSERT: unnamed = 1;
    // don't return until completed, a VM should NEVER use this,
    pub const EXEC_NOW: unnamed = 0;
    // parameters to the main Error routine
    pub type unnamed_0 = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed_0 = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed_0 = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed_0 = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed_0 = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed_0 = 0;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
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
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, vmCvar_t, fileHandle_t, fsMode_t,
                            qtime_t};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_AdrToStringwPort(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_StringToAdr(s: *const libc::c_char, a: *mut netadr_t,
                               family: netadrtype_t) -> libc::c_int;
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
        #[no_mangle]
        pub fn Cvar_SetValueSafe(var_name: *const libc::c_char,
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
        // callback with each valid string
        #[no_mangle]
        pub fn Cvar_Reset(var_name: *const libc::c_char);
        // returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
        #[no_mangle]
        pub fn Cvar_InfoStringBuffer(bit: libc::c_int,
                                     buff: *mut libc::c_char,
                                     buffsize: libc::c_int);
        #[no_mangle]
        pub static mut cvar_modifiedFlags: libc::c_int;
        #[no_mangle]
        pub fn FS_GetFileList(path: *const libc::c_char,
                              extension: *const libc::c_char,
                              listbuf: *mut libc::c_char,
                              bufsize: libc::c_int) -> libc::c_int;
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
        // like fprintf
        #[no_mangle]
        pub fn FS_FOpenFileByMode(qpath: *const libc::c_char,
                                  f: *mut fileHandle_t, mode: fsMode_t)
         -> libc::c_int;
        // opens a file for reading, writing, or appending depending on the value of mode
        #[no_mangle]
        pub fn FS_Seek(f: fileHandle_t, offset: libc::c_long,
                       origin: libc::c_int) -> libc::c_int;
        /*
==============================================================

MISC

==============================================================
*/
        // centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
        #[no_mangle]
        pub static mut cl_cdkey: [libc::c_char; 34];
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Com_RealTime(qtime: *mut qtime_t) -> libc::c_int;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Hunk_MemoryRemaining() -> libc::c_int;
        // note that this isn't journaled...
        #[no_mangle]
        pub fn Sys_GetClipboardData() -> *mut libc::c_char;
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/client.h"]
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
        // interface to refresh .dll
        //
// cl_parse.c
//
        /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
        // this is for the GL_EXT_texture_compression_s3tc extension.
        // this is for the GL_S3_s3tc extension.
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
        // where you don't have src*dst
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
        // where you can't modulate alpha on alpha textures
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
                            connstate_t, fileHandle_t, e_status};
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
        pub static mut cl: clientActive_t;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut re: refexport_t;
        #[no_mangle]
        pub fn CL_GetPing(n: libc::c_int, buf: *mut libc::c_char,
                          buflen: libc::c_int, pingtime: *mut libc::c_int);
        #[no_mangle]
        pub fn CL_GetPingInfo(n: libc::c_int, buf: *mut libc::c_char,
                              buflen: libc::c_int);
        #[no_mangle]
        pub fn CL_ClearPing(n: libc::c_int);
        #[no_mangle]
        pub fn CL_GetPingQueueCount() -> libc::c_int;
        #[no_mangle]
        pub fn CL_CDKeyValidate(key: *const libc::c_char,
                                checksum: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn CL_ServerStatus(serverAddress: *mut libc::c_char,
                               serverStatusString: *mut libc::c_char,
                               maxLen: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Key_KeynumToString(keynum: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub static mut cl_connectedToPureServer: libc::c_int;
        #[no_mangle]
        pub fn CL_UpdateVisiblePings_f(source: libc::c_int) -> qboolean;
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
      "/home/miguelsaldivar/workspace/ioq3/code/renderercommon/tr_types.h"]
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
    use super::{libc};
    use super::q_shared_h::{qboolean, vec3_t, byte, qhandle_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_public.h"]
pub mod ui_public_h {
    //	void	UI_SetActiveMenu( uiMenuCommand_t menu );
    pub const UI_CONSOLE_COMMAND: unnamed_2 = 8;
    //	void	UI_DrawConnectScreen( qboolean overlay );
    pub const UI_HASUNIQUECDKEY: unnamed_2 = 10;
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct uiClientState_t {
        pub connState: connstate_t,
        pub connectPacketCount: libc::c_int,
        pub clientNum: libc::c_int,
        pub servername: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub messageString: [libc::c_char; 1024],
    }
    pub type unnamed_1 = libc::c_uint;
    pub const UI_CEIL: unnamed_1 = 108;
    pub const UI_FLOOR: unnamed_1 = 107;
    pub const UI_SQRT: unnamed_1 = 106;
    pub const UI_ATAN2: unnamed_1 = 105;
    pub const UI_COS: unnamed_1 = 104;
    pub const UI_SIN: unnamed_1 = 103;
    pub const UI_STRNCPY: unnamed_1 = 102;
    pub const UI_MEMCPY: unnamed_1 = 101;
    pub const UI_MEMSET: unnamed_1 = 100;
    pub const UI_SET_PBCLSTATUS: unnamed_1 = 87;
    // 1.32
    pub const UI_FS_SEEK: unnamed_1 = 86;
    pub const UI_LAN_COMPARESERVERS: unnamed_1 = 85;
    pub const UI_LAN_SERVERISVISIBLE: unnamed_1 = 84;
    pub const UI_LAN_GETSERVERPING: unnamed_1 = 83;
    pub const UI_LAN_SERVERSTATUS: unnamed_1 = 82;
    pub const UI_VERIFY_CDKEY: unnamed_1 = 81;
    pub const UI_R_REMAP_SHADER: unnamed_1 = 80;
    pub const UI_CIN_SETEXTENTS: unnamed_1 = 79;
    pub const UI_CIN_DRAWCINEMATIC: unnamed_1 = 78;
    pub const UI_CIN_RUNCINEMATIC: unnamed_1 = 77;
    pub const UI_CIN_STOPCINEMATIC: unnamed_1 = 76;
    pub const UI_CIN_PLAYCINEMATIC: unnamed_1 = 75;
    pub const UI_LAN_REMOVESERVER: unnamed_1 = 74;
    pub const UI_LAN_ADDSERVER: unnamed_1 = 73;
    pub const UI_LAN_SAVECACHEDSERVERS: unnamed_1 = 72;
    pub const UI_LAN_LOADCACHEDSERVERS: unnamed_1 = 71;
    pub const UI_LAN_RESETPINGS: unnamed_1 = 70;
    pub const UI_LAN_UPDATEVISIBLEPINGS: unnamed_1 = 69;
    pub const UI_LAN_MARKSERVERVISIBLE: unnamed_1 = 68;
    pub const UI_LAN_GETSERVERINFO: unnamed_1 = 67;
    pub const UI_LAN_GETSERVERADDRESSSTRING: unnamed_1 = 66;
    pub const UI_LAN_GETSERVERCOUNT: unnamed_1 = 65;
    pub const UI_REAL_TIME: unnamed_1 = 64;
    pub const UI_S_STARTBACKGROUNDTRACK: unnamed_1 = 63;
    pub const UI_S_STOPBACKGROUNDTRACK: unnamed_1 = 62;
    pub const UI_PC_SOURCE_FILE_AND_LINE: unnamed_1 = 61;
    pub const UI_PC_READ_TOKEN: unnamed_1 = 60;
    pub const UI_PC_FREE_SOURCE: unnamed_1 = 59;
    pub const UI_PC_LOAD_SOURCE: unnamed_1 = 58;
    pub const UI_PC_ADD_GLOBAL_DEFINE: unnamed_1 = 57;
    pub const UI_R_MODELBOUNDS: unnamed_1 = 56;
    pub const UI_R_REGISTERFONT: unnamed_1 = 55;
    pub const UI_SET_CDKEY: unnamed_1 = 54;
    pub const UI_GET_CDKEY: unnamed_1 = 53;
    pub const UI_MEMORY_REMAINING: unnamed_1 = 52;
    pub const UI_CVAR_UPDATE: unnamed_1 = 51;
    pub const UI_CVAR_REGISTER: unnamed_1 = 50;
    pub const UI_LAN_GETPINGINFO: unnamed_1 = 49;
    pub const UI_LAN_GETPING: unnamed_1 = 48;
    pub const UI_LAN_CLEARPING: unnamed_1 = 47;
    pub const UI_LAN_GETPINGQUEUECOUNT: unnamed_1 = 46;
    pub const UI_GETCONFIGSTRING: unnamed_1 = 45;
    pub const UI_GETCLIENTSTATE: unnamed_1 = 44;
    pub const UI_GETGLCONFIG: unnamed_1 = 43;
    pub const UI_GETCLIPBOARDDATA: unnamed_1 = 42;
    pub const UI_KEY_SETCATCHER: unnamed_1 = 41;
    pub const UI_KEY_GETCATCHER: unnamed_1 = 40;
    pub const UI_KEY_CLEARSTATES: unnamed_1 = 39;
    pub const UI_KEY_SETOVERSTRIKEMODE: unnamed_1 = 38;
    pub const UI_KEY_GETOVERSTRIKEMODE: unnamed_1 = 37;
    pub const UI_KEY_ISDOWN: unnamed_1 = 36;
    pub const UI_KEY_SETBINDING: unnamed_1 = 35;
    pub const UI_KEY_GETBINDINGBUF: unnamed_1 = 34;
    pub const UI_KEY_KEYNUMTOSTRINGBUF: unnamed_1 = 33;
    pub const UI_S_STARTLOCALSOUND: unnamed_1 = 32;
    pub const UI_S_REGISTERSOUND: unnamed_1 = 31;
    pub const UI_CM_LOADMODEL: unnamed_1 = 30;
    pub const UI_CM_LERPTAG: unnamed_1 = 29;
    pub const UI_UPDATESCREEN: unnamed_1 = 28;
    pub const UI_R_DRAWSTRETCHPIC: unnamed_1 = 27;
    pub const UI_R_SETCOLOR: unnamed_1 = 26;
    pub const UI_R_RENDERSCENE: unnamed_1 = 25;
    pub const UI_R_ADDLIGHTTOSCENE: unnamed_1 = 24;
    pub const UI_R_ADDPOLYTOSCENE: unnamed_1 = 23;
    pub const UI_R_ADDREFENTITYTOSCENE: unnamed_1 = 22;
    pub const UI_R_CLEARSCENE: unnamed_1 = 21;
    pub const UI_R_REGISTERSHADERNOMIP: unnamed_1 = 20;
    pub const UI_R_REGISTERSKIN: unnamed_1 = 19;
    pub const UI_R_REGISTERMODEL: unnamed_1 = 18;
    pub const UI_FS_GETFILELIST: unnamed_1 = 17;
    pub const UI_FS_FCLOSEFILE: unnamed_1 = 16;
    pub const UI_FS_WRITE: unnamed_1 = 15;
    pub const UI_FS_READ: unnamed_1 = 14;
    pub const UI_FS_FOPENFILE: unnamed_1 = 13;
    pub const UI_CMD_EXECUTETEXT: unnamed_1 = 12;
    pub const UI_ARGV: unnamed_1 = 11;
    pub const UI_ARGC: unnamed_1 = 10;
    pub const UI_CVAR_INFOSTRINGBUFFER: unnamed_1 = 9;
    pub const UI_CVAR_CREATE: unnamed_1 = 8;
    pub const UI_CVAR_RESET: unnamed_1 = 7;
    pub const UI_CVAR_SETVALUE: unnamed_1 = 6;
    pub const UI_CVAR_VARIABLESTRINGBUFFER: unnamed_1 = 5;
    pub const UI_CVAR_VARIABLEVALUE: unnamed_1 = 4;
    pub const UI_CVAR_SET: unnamed_1 = 3;
    pub const UI_MILLISECONDS: unnamed_1 = 2;
    pub const UI_PRINT: unnamed_1 = 1;
    pub const UI_ERROR: unnamed_1 = 0;
    pub type unnamed_2 = libc::c_uint;
    //	qboolean UI_ConsoleCommand( int realTime );
    pub const UI_DRAW_CONNECT_SCREEN: unnamed_2 = 9;
    //	qboolean UI_IsFullscreen( void );
    pub const UI_SET_ACTIVE_MENU: unnamed_2 = 7;
    //	void	UI_Refresh( int time );
    pub const UI_IS_FULLSCREEN: unnamed_2 = 6;
    //	void	UI_MouseEvent( int dx, int dy );
    pub const UI_REFRESH: unnamed_2 = 5;
    //	void	UI_KeyEvent( int key );
    pub const UI_MOUSE_EVENT: unnamed_2 = 4;
    //	void	UI_Shutdown( void );
    pub const UI_KEY_EVENT: unnamed_2 = 3;
    //	void	UI_Init( void );
    pub const UI_SHUTDOWN: unnamed_2 = 2;
    pub const UI_INIT: unnamed_2 = 1;
    // system reserved
    pub const UI_GETAPIVERSION: unnamed_2 = 0;
    use super::q_shared_h::{connstate_t};
    use super::{libc};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/renderercommon/tr_public.h"]
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
        #[no_mangle]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/keys.h"]
pub mod keys_h {
    use super::{libc};
    use super::q_shared_h::{qboolean};
    extern "C" {
        #[no_mangle]
        pub fn Key_SetBinding(keynum: libc::c_int,
                              binding: *const libc::c_char);
        #[no_mangle]
        pub fn Key_GetBinding(keynum: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Key_IsDown(keynum: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn Key_GetOverstrikeMode() -> qboolean;
        #[no_mangle]
        pub fn Key_SetOverstrikeMode(state: qboolean);
        #[no_mangle]
        pub fn Key_ClearStates();
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    use super::q_shared_h::{sfxHandle_t, qboolean};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn S_StartLocalSound(sfx: sfxHandle_t, channelNum: libc::c_int);
        #[no_mangle]
        pub fn S_StartBackgroundTrack(intro: *const libc::c_char,
                                      loop_0: *const libc::c_char);
        #[no_mangle]
        pub fn S_StopBackgroundTrack();
        // RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
        #[no_mangle]
        pub fn S_RegisterSound(sample: *const libc::c_char,
                               compressed: qboolean) -> sfxHandle_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/cl_ui.c"]
pub mod cl_ui_c {
    use super::stdint_h::{intptr_t};
    use super::botlib_h::{botlib_export_t};
    use super::{libc};
    use super::q_shared_h::{qboolean};
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
        pub static mut botlib_export: *mut botlib_export_t;
    }
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t, qhandle_t,
                       sfxHandle_t, fileHandle_t, unnamed, EXEC_APPEND,
                       EXEC_INSERT, EXEC_NOW, unnamed_0, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, vec_t, vec3_t, pc_token_s, pc_token_t,
                       fsMode_t, FS_APPEND_SYNC, FS_APPEND, FS_WRITE, FS_READ,
                       cvarHandle_t, vmCvar_t, markFragment_t, orientation_t,
                       gameState_t, playerState_s, playerState_t, usercmd_s,
                       usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, connstate_t, CA_CINEMATIC, CA_ACTIVE,
                       CA_PRIMED, CA_LOADING, CA_CONNECTED, CA_CHALLENGING,
                       CA_CONNECTING, CA_AUTHORIZING, CA_DISCONNECTED,
                       CA_UNINITIALIZED, glyphInfo_t, fontInfo_t, qtime_s,
                       qtime_t, e_status, FMV_ID_WAIT, FMV_LOOPED,
                       FMV_ID_IDLE, FMV_ID_BLT, FMV_EOF, FMV_PLAY, FMV_IDLE,
                       Q_stricmp, Q_strncpyz, va, Info_SetValueForKey,
                       Com_Error, Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t, vm_t,
                      vmInterpret_t, VMI_COMPILED, VMI_BYTECODE, VMI_NATIVE,
                      vm_s, NET_CompareAdr, NET_AdrToStringwPort,
                      NET_StringToAdr, VM_Create, VM_Free, VM_Call, VM_ArgPtr,
                      Cbuf_ExecuteText, Cmd_Argc, Cmd_ArgvBuffer,
                      Cvar_Register, Cvar_Update, Cvar_SetSafe,
                      Cvar_SetValueSafe, Cvar_VariableValue,
                      Cvar_VariableString, Cvar_VariableStringBuffer,
                      Cvar_Reset, Cvar_InfoStringBuffer, cvar_modifiedFlags,
                      FS_GetFileList, FS_SV_FOpenFileWrite,
                      FS_SV_FOpenFileRead, FS_Write, FS_Read, FS_FCloseFile,
                      FS_FOpenFileByMode, FS_Seek, cl_cdkey, Com_DPrintf,
                      Com_RealTime, Z_Free, Hunk_MemoryRemaining,
                      Sys_GetClipboardData, Sys_Milliseconds};
use self::client_h::{clientStatic_t, serverInfo_t, clSnapshot_t, outPacket_t,
                     clientActive_t, clientConnection_t, cls, cl, clc, re,
                     CL_GetPing, CL_GetPingInfo, CL_ClearPing,
                     CL_GetPingQueueCount, CL_CDKeyValidate, CL_ServerStatus,
                     Key_KeynumToString, cl_connectedToPureServer,
                     CL_UpdateVisiblePings_f, SCR_UpdateScreen,
                     CIN_PlayCinematic, CIN_StopCinematic, CIN_RunCinematic,
                     CIN_DrawCinematic, CIN_SetExtents, Key_GetCatcher,
                     Key_SetCatcher};
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
use self::ui_public_h::{UI_CONSOLE_COMMAND, UI_HASUNIQUECDKEY,
                        uiClientState_t, unnamed_1, UI_CEIL, UI_FLOOR,
                        UI_SQRT, UI_ATAN2, UI_COS, UI_SIN, UI_STRNCPY,
                        UI_MEMCPY, UI_MEMSET, UI_SET_PBCLSTATUS, UI_FS_SEEK,
                        UI_LAN_COMPARESERVERS, UI_LAN_SERVERISVISIBLE,
                        UI_LAN_GETSERVERPING, UI_LAN_SERVERSTATUS,
                        UI_VERIFY_CDKEY, UI_R_REMAP_SHADER, UI_CIN_SETEXTENTS,
                        UI_CIN_DRAWCINEMATIC, UI_CIN_RUNCINEMATIC,
                        UI_CIN_STOPCINEMATIC, UI_CIN_PLAYCINEMATIC,
                        UI_LAN_REMOVESERVER, UI_LAN_ADDSERVER,
                        UI_LAN_SAVECACHEDSERVERS, UI_LAN_LOADCACHEDSERVERS,
                        UI_LAN_RESETPINGS, UI_LAN_UPDATEVISIBLEPINGS,
                        UI_LAN_MARKSERVERVISIBLE, UI_LAN_GETSERVERINFO,
                        UI_LAN_GETSERVERADDRESSSTRING, UI_LAN_GETSERVERCOUNT,
                        UI_REAL_TIME, UI_S_STARTBACKGROUNDTRACK,
                        UI_S_STOPBACKGROUNDTRACK, UI_PC_SOURCE_FILE_AND_LINE,
                        UI_PC_READ_TOKEN, UI_PC_FREE_SOURCE,
                        UI_PC_LOAD_SOURCE, UI_PC_ADD_GLOBAL_DEFINE,
                        UI_R_MODELBOUNDS, UI_R_REGISTERFONT, UI_SET_CDKEY,
                        UI_GET_CDKEY, UI_MEMORY_REMAINING, UI_CVAR_UPDATE,
                        UI_CVAR_REGISTER, UI_LAN_GETPINGINFO, UI_LAN_GETPING,
                        UI_LAN_CLEARPING, UI_LAN_GETPINGQUEUECOUNT,
                        UI_GETCONFIGSTRING, UI_GETCLIENTSTATE, UI_GETGLCONFIG,
                        UI_GETCLIPBOARDDATA, UI_KEY_SETCATCHER,
                        UI_KEY_GETCATCHER, UI_KEY_CLEARSTATES,
                        UI_KEY_SETOVERSTRIKEMODE, UI_KEY_GETOVERSTRIKEMODE,
                        UI_KEY_ISDOWN, UI_KEY_SETBINDING,
                        UI_KEY_GETBINDINGBUF, UI_KEY_KEYNUMTOSTRINGBUF,
                        UI_S_STARTLOCALSOUND, UI_S_REGISTERSOUND,
                        UI_CM_LOADMODEL, UI_CM_LERPTAG, UI_UPDATESCREEN,
                        UI_R_DRAWSTRETCHPIC, UI_R_SETCOLOR, UI_R_RENDERSCENE,
                        UI_R_ADDLIGHTTOSCENE, UI_R_ADDPOLYTOSCENE,
                        UI_R_ADDREFENTITYTOSCENE, UI_R_CLEARSCENE,
                        UI_R_REGISTERSHADERNOMIP, UI_R_REGISTERSKIN,
                        UI_R_REGISTERMODEL, UI_FS_GETFILELIST,
                        UI_FS_FCLOSEFILE, UI_FS_WRITE, UI_FS_READ,
                        UI_FS_FOPENFILE, UI_CMD_EXECUTETEXT, UI_ARGV, UI_ARGC,
                        UI_CVAR_INFOSTRINGBUFFER, UI_CVAR_CREATE,
                        UI_CVAR_RESET, UI_CVAR_SETVALUE,
                        UI_CVAR_VARIABLESTRINGBUFFER, UI_CVAR_VARIABLEVALUE,
                        UI_CVAR_SET, UI_MILLISECONDS, UI_PRINT, UI_ERROR,
                        unnamed_2, UI_DRAW_CONNECT_SCREEN, UI_SET_ACTIVE_MENU,
                        UI_IS_FULLSCREEN, UI_REFRESH, UI_MOUSE_EVENT,
                        UI_KEY_EVENT, UI_SHUTDOWN, UI_INIT, UI_GETAPIVERSION};
use self::tr_public_h::{refexport_t};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::botlib_h::{botlib_export_t, botlib_export_s, bot_entitystate_t,
                     bot_entitystate_s, ai_export_t, ai_export_s, ea_export_t,
                     ea_export_s, bot_input_t, bot_input_s, aas_export_t,
                     aas_export_s, weaponinfo_s, bot_initmove_s, bot_goal_s,
                     bot_moveresult_s, bot_match_s, bot_consolemessage_s,
                     aas_clientmove_s, aas_altroutegoal_s, aas_predictroute_s,
                     aas_areainfo_s, aas_entityinfo_s};
use self::mathcalls_h::{atan2, cos, sin, sqrt, ceil, floor};
use self::string_h::{memcpy, memset, strncpy, strncmp};
use self::keys_h::{Key_SetBinding, Key_GetBinding, Key_IsDown,
                   Key_GetOverstrikeMode, Key_SetOverstrikeMode,
                   Key_ClearStates};
use self::snd_public_h::{S_StartLocalSound, S_StartBackgroundTrack,
                         S_StopBackgroundTrack, S_RegisterSound};
use self::cl_ui_c::{botlib_export};
unsafe extern "C" fn _vmf(mut x: intptr_t) -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i = x as libc::c_int;
    return fi.f;
}
//
// UI interface
//
#[no_mangle]
pub unsafe extern "C" fn UI_GameCommand() -> qboolean {
    if uivm.is_null() { return qfalse }
    return VM_Call(uivm, UI_CONSOLE_COMMAND as libc::c_int, cls.realtime) as
               qboolean;
}
// interface to ui dll or vm
#[no_mangle]
pub static mut uivm: *mut vm_t = 0 as *const vm_t as *mut vm_t;
#[no_mangle]
pub unsafe extern "C" fn UI_usesUniqueCDKey() -> qboolean {
    if !uivm.is_null() {
        return (VM_Call(uivm, UI_HASUNIQUECDKEY as libc::c_int) ==
                    qtrue as libc::c_int as libc::c_long) as libc::c_int as
                   qboolean
    } else { return qfalse };
}
//
// cl_ui.c
//
#[no_mangle]
pub unsafe extern "C" fn CL_InitUI() {
    let mut v: libc::c_int = 0;
    let mut interpret: vmInterpret_t = VMI_NATIVE;
    interpret =
        Cvar_VariableValue(b"vm_ui\x00" as *const u8 as *const libc::c_char)
            as vmInterpret_t;
    if 0 != cl_connectedToPureServer {
        if interpret as libc::c_uint !=
               VMI_COMPILED as libc::c_int as libc::c_uint &&
               interpret as libc::c_uint !=
                   VMI_BYTECODE as libc::c_int as libc::c_uint {
            interpret = VMI_COMPILED
        }
    }
    uivm =
        VM_Create(b"ui\x00" as *const u8 as *const libc::c_char,
                  Some(CL_UISystemCalls), interpret);
    if uivm.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_Create on UI failed\x00" as *const u8 as
                      *const libc::c_char);
    }
    v = VM_Call(uivm, UI_GETAPIVERSION as libc::c_int) as libc::c_int;
    if v == 4i32 {
        VM_Call(uivm, UI_INIT as libc::c_int,
                (clc.state as libc::c_uint >=
                     CA_AUTHORIZING as libc::c_int as libc::c_uint &&
                     (clc.state as libc::c_uint) <
                         CA_ACTIVE as libc::c_int as libc::c_uint) as
                    libc::c_int);
    } else if v != 6i32 {
        VM_Free(uivm);
        uivm = 0 as *mut vm_t;
        Com_Error(ERR_DROP as libc::c_int,
                  b"User Interface is version %d, expected %d\x00" as
                      *const u8 as *const libc::c_char, v, 6i32);
    } else {
        VM_Call(uivm, UI_INIT as libc::c_int,
                (clc.state as libc::c_uint >=
                     CA_AUTHORIZING as libc::c_int as libc::c_uint &&
                     (clc.state as libc::c_uint) <
                         CA_ACTIVE as libc::c_int as libc::c_uint) as
                    libc::c_int);
    };
}
/*
====================
CL_UISystemCalls

The ui module is making a system call
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UISystemCalls(mut args: *mut intptr_t)
 -> intptr_t {
    match *args.offset(0isize) {
        0 => {
            Com_Error(ERR_DROP as libc::c_int,
                      b"%s\x00" as *const u8 as *const libc::c_char,
                      VM_ArgPtr(*args.offset(1isize)) as *const libc::c_char);
        }
        1 => {
            Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       VM_ArgPtr(*args.offset(1isize)) as
                           *const libc::c_char);
            return 0i32 as intptr_t
        }
        2 => { return Sys_Milliseconds() as intptr_t }
        50 => {
            Cvar_Register(VM_ArgPtr(*args.offset(1isize)) as *mut vmCvar_t,
                          VM_ArgPtr(*args.offset(2isize)) as
                              *const libc::c_char,
                          VM_ArgPtr(*args.offset(3isize)) as
                              *const libc::c_char,
                          *args.offset(4isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        51 => {
            Cvar_Update(VM_ArgPtr(*args.offset(1isize)) as *mut vmCvar_t);
            return 0i32 as intptr_t
        }
        3 => {
            Cvar_SetSafe(VM_ArgPtr(*args.offset(1isize)) as
                             *const libc::c_char,
                         VM_ArgPtr(*args.offset(2isize)) as
                             *const libc::c_char);
            return 0i32 as intptr_t
        }
        4 => {
            return FloatAsInt(Cvar_VariableValue(VM_ArgPtr(*args.offset(1isize))
                                                     as *const libc::c_char))
                       as intptr_t
        }
        5 => {
            Cvar_VariableStringBuffer(VM_ArgPtr(*args.offset(1isize)) as
                                          *const libc::c_char,
                                      VM_ArgPtr(*args.offset(2isize)) as
                                          *mut libc::c_char,
                                      *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        6 => {
            Cvar_SetValueSafe(VM_ArgPtr(*args.offset(1isize)) as
                                  *const libc::c_char,
                              _vmf(*args.offset(2isize)));
            return 0i32 as intptr_t
        }
        7 => {
            Cvar_Reset(VM_ArgPtr(*args.offset(1isize)) as
                           *const libc::c_char);
            return 0i32 as intptr_t
        }
        8 => {
            Cvar_Register(0 as *mut vmCvar_t,
                          VM_ArgPtr(*args.offset(1isize)) as
                              *const libc::c_char,
                          VM_ArgPtr(*args.offset(2isize)) as
                              *const libc::c_char,
                          *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        9 => {
            Cvar_InfoStringBuffer(*args.offset(1isize) as libc::c_int,
                                  VM_ArgPtr(*args.offset(2isize)) as
                                      *mut libc::c_char,
                                  *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        10 => { return Cmd_Argc() as intptr_t }
        11 => {
            Cmd_ArgvBuffer(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *mut libc::c_char,
                           *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        12 => {
            if *args.offset(1isize) == EXEC_NOW as libc::c_int as libc::c_long
                   &&
                   (0 ==
                        strncmp(VM_ArgPtr(*args.offset(2isize)) as
                                    *const libc::c_char,
                                b"snd_restart\x00" as *const u8 as
                                    *const libc::c_char,
                                11i32 as libc::c_ulong) ||
                        0 ==
                            strncmp(VM_ArgPtr(*args.offset(2isize)) as
                                        *const libc::c_char,
                                    b"vid_restart\x00" as *const u8 as
                                        *const libc::c_char,
                                    11i32 as libc::c_ulong) ||
                        0 ==
                            strncmp(VM_ArgPtr(*args.offset(2isize)) as
                                        *const libc::c_char,
                                    b"quit\x00" as *const u8 as
                                        *const libc::c_char,
                                    5i32 as libc::c_ulong)) {
                Com_Printf(b"^3turning EXEC_NOW \'%.11s\' into EXEC_INSERT\n\x00"
                               as *const u8 as *const libc::c_char,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *const libc::c_char);
                *args.offset(1isize) = EXEC_INSERT as libc::c_int as intptr_t
            }
            Cbuf_ExecuteText(*args.offset(1isize) as libc::c_int,
                             VM_ArgPtr(*args.offset(2isize)) as
                                 *const libc::c_char);
            return 0i32 as intptr_t
        }
        13 => {
            return FS_FOpenFileByMode(VM_ArgPtr(*args.offset(1isize)) as
                                          *const libc::c_char,
                                      VM_ArgPtr(*args.offset(2isize)) as
                                          *mut fileHandle_t,
                                      *args.offset(3isize) as fsMode_t) as
                       intptr_t
        }
        14 => {
            FS_Read(VM_ArgPtr(*args.offset(1isize)),
                    *args.offset(2isize) as libc::c_int,
                    *args.offset(3isize) as fileHandle_t);
            return 0i32 as intptr_t
        }
        15 => {
            FS_Write(VM_ArgPtr(*args.offset(1isize)),
                     *args.offset(2isize) as libc::c_int,
                     *args.offset(3isize) as fileHandle_t);
            return 0i32 as intptr_t
        }
        16 => {
            FS_FCloseFile(*args.offset(1isize) as fileHandle_t);
            return 0i32 as intptr_t
        }
        17 => {
            return FS_GetFileList(VM_ArgPtr(*args.offset(1isize)) as
                                      *const libc::c_char,
                                  VM_ArgPtr(*args.offset(2isize)) as
                                      *const libc::c_char,
                                  VM_ArgPtr(*args.offset(3isize)) as
                                      *mut libc::c_char,
                                  *args.offset(4isize) as libc::c_int) as
                       intptr_t
        }
        86 => {
            return FS_Seek(*args.offset(1isize) as fileHandle_t,
                           *args.offset(2isize),
                           *args.offset(3isize) as libc::c_int) as intptr_t
        }
        18 => {
            return re.RegisterModel.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                            as
                                                                            *const libc::c_char)
                       as intptr_t
        }
        19 => {
            return re.RegisterSkin.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                           as
                                                                           *const libc::c_char)
                       as intptr_t
        }
        20 => {
            return re.RegisterShaderNoMip.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                  as
                                                                                  *const libc::c_char)
                       as intptr_t
        }
        21 => {
            re.ClearScene.expect("non-null function pointer")();
            return 0i32 as intptr_t
        }
        22 => {
            re.AddRefEntityToScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                           as
                                                                           *const refEntity_t);
            return 0i32 as intptr_t
        }
        23 => {
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
        24 => {
            re.AddLightToScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                       as
                                                                       *const vec_t,
                                                                   _vmf(*args.offset(2isize)),
                                                                   _vmf(*args.offset(3isize)),
                                                                   _vmf(*args.offset(4isize)),
                                                                   _vmf(*args.offset(5isize)));
            return 0i32 as intptr_t
        }
        25 => {
            re.RenderScene.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                   as
                                                                   *const refdef_t);
            return 0i32 as intptr_t
        }
        26 => {
            re.SetColor.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                as
                                                                *const libc::c_float);
            return 0i32 as intptr_t
        }
        27 => {
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
        56 => {
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
        28 => { SCR_UpdateScreen(); return 0i32 as intptr_t }
        29 => {
            re.LerpTag.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                               as
                                                               *mut orientation_t,
                                                           *args.offset(2isize)
                                                               as qhandle_t,
                                                           *args.offset(3isize)
                                                               as libc::c_int,
                                                           *args.offset(4isize)
                                                               as libc::c_int,
                                                           _vmf(*args.offset(5isize)),
                                                           VM_ArgPtr(*args.offset(6isize))
                                                               as
                                                               *const libc::c_char);
            return 0i32 as intptr_t
        }
        31 => {
            return S_RegisterSound(VM_ArgPtr(*args.offset(1isize)) as
                                       *const libc::c_char,
                                   *args.offset(2isize) as qboolean) as
                       intptr_t
        }
        32 => {
            S_StartLocalSound(*args.offset(1isize) as sfxHandle_t,
                              *args.offset(2isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        33 => {
            Key_KeynumToStringBuf(*args.offset(1isize) as libc::c_int,
                                  VM_ArgPtr(*args.offset(2isize)) as
                                      *mut libc::c_char,
                                  *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        34 => {
            Key_GetBindingBuf(*args.offset(1isize) as libc::c_int,
                              VM_ArgPtr(*args.offset(2isize)) as
                                  *mut libc::c_char,
                              *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        35 => {
            Key_SetBinding(*args.offset(1isize) as libc::c_int,
                           VM_ArgPtr(*args.offset(2isize)) as
                               *const libc::c_char);
            return 0i32 as intptr_t
        }
        36 => {
            return Key_IsDown(*args.offset(1isize) as libc::c_int) as intptr_t
        }
        37 => { return Key_GetOverstrikeMode() as intptr_t }
        38 => {
            Key_SetOverstrikeMode(*args.offset(1isize) as qboolean);
            return 0i32 as intptr_t
        }
        39 => { Key_ClearStates(); return 0i32 as intptr_t }
        40 => { return Key_GetCatcher() as intptr_t }
        41 => {
            Key_SetCatcher((*args.offset(1isize) |
                                (Key_GetCatcher() & 0x1i32) as libc::c_long)
                               as libc::c_int);
            return 0i32 as intptr_t
        }
        42 => {
            CL_GetClipboardData(VM_ArgPtr(*args.offset(1isize)) as
                                    *mut libc::c_char,
                                *args.offset(2isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        44 => {
            GetClientState(VM_ArgPtr(*args.offset(1isize)) as
                               *mut uiClientState_t);
            return 0i32 as intptr_t
        }
        43 => {
            CL_GetGlconfig(VM_ArgPtr(*args.offset(1isize)) as
                               *mut glconfig_t);
            return 0i32 as intptr_t
        }
        45 => {
            return GetConfigString(*args.offset(1isize) as libc::c_int,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *mut libc::c_char,
                                   *args.offset(3isize) as libc::c_int) as
                       intptr_t
        }
        71 => { LAN_LoadCachedServers(); return 0i32 as intptr_t }
        72 => { LAN_SaveServersToCache(); return 0i32 as intptr_t }
        73 => {
            return LAN_AddServer(*args.offset(1isize) as libc::c_int,
                                 VM_ArgPtr(*args.offset(2isize)) as
                                     *const libc::c_char,
                                 VM_ArgPtr(*args.offset(3isize)) as
                                     *const libc::c_char) as intptr_t
        }
        74 => {
            LAN_RemoveServer(*args.offset(1isize) as libc::c_int,
                             VM_ArgPtr(*args.offset(2isize)) as
                                 *const libc::c_char);
            return 0i32 as intptr_t
        }
        46 => { return LAN_GetPingQueueCount() as intptr_t }
        47 => {
            LAN_ClearPing(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        48 => {
            LAN_GetPing(*args.offset(1isize) as libc::c_int,
                        VM_ArgPtr(*args.offset(2isize)) as *mut libc::c_char,
                        *args.offset(3isize) as libc::c_int,
                        VM_ArgPtr(*args.offset(4isize)) as *mut libc::c_int);
            return 0i32 as intptr_t
        }
        49 => {
            LAN_GetPingInfo(*args.offset(1isize) as libc::c_int,
                            VM_ArgPtr(*args.offset(2isize)) as
                                *mut libc::c_char,
                            *args.offset(3isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        65 => {
            return LAN_GetServerCount(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        66 => {
            LAN_GetServerAddressString(*args.offset(1isize) as libc::c_int,
                                       *args.offset(2isize) as libc::c_int,
                                       VM_ArgPtr(*args.offset(3isize)) as
                                           *mut libc::c_char,
                                       *args.offset(4isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        67 => {
            LAN_GetServerInfo(*args.offset(1isize) as libc::c_int,
                              *args.offset(2isize) as libc::c_int,
                              VM_ArgPtr(*args.offset(3isize)) as
                                  *mut libc::c_char,
                              *args.offset(4isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        83 => {
            return LAN_GetServerPing(*args.offset(1isize) as libc::c_int,
                                     *args.offset(2isize) as libc::c_int) as
                       intptr_t
        }
        68 => {
            LAN_MarkServerVisible(*args.offset(1isize) as libc::c_int,
                                  *args.offset(2isize) as libc::c_int,
                                  *args.offset(3isize) as qboolean);
            return 0i32 as intptr_t
        }
        84 => {
            return LAN_ServerIsVisible(*args.offset(1isize) as libc::c_int,
                                       *args.offset(2isize) as libc::c_int) as
                       intptr_t
        }
        69 => {
            return LAN_UpdateVisiblePings(*args.offset(1isize) as libc::c_int)
                       as intptr_t
        }
        70 => {
            LAN_ResetPings(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        82 => {
            return LAN_GetServerStatus(VM_ArgPtr(*args.offset(1isize)) as
                                           *mut libc::c_char,
                                       VM_ArgPtr(*args.offset(2isize)) as
                                           *mut libc::c_char,
                                       *args.offset(3isize) as libc::c_int) as
                       intptr_t
        }
        85 => {
            return LAN_CompareServers(*args.offset(1isize) as libc::c_int,
                                      *args.offset(2isize) as libc::c_int,
                                      *args.offset(3isize) as libc::c_int,
                                      *args.offset(4isize) as libc::c_int,
                                      *args.offset(5isize) as libc::c_int) as
                       intptr_t
        }
        52 => { return Hunk_MemoryRemaining() as intptr_t }
        53 => {
            CLUI_GetCDKey(VM_ArgPtr(*args.offset(1isize)) as
                              *mut libc::c_char,
                          *args.offset(2isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        54 => {
            CLUI_SetCDKey(VM_ArgPtr(*args.offset(1isize)) as
                              *mut libc::c_char);
            return 0i32 as intptr_t
        }
        87 => { return 0i32 as intptr_t }
        55 => {
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
        57 => {
            return (*botlib_export).PC_AddGlobalDefine.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                               as
                                                                                               *mut libc::c_char)
                       as intptr_t
        }
        58 => {
            return (*botlib_export).PC_LoadSourceHandle.expect("non-null function pointer")(VM_ArgPtr(*args.offset(1isize))
                                                                                                as
                                                                                                *const libc::c_char)
                       as intptr_t
        }
        59 => {
            return (*botlib_export).PC_FreeSourceHandle.expect("non-null function pointer")(*args.offset(1isize)
                                                                                                as
                                                                                                libc::c_int)
                       as intptr_t
        }
        60 => {
            return (*botlib_export).PC_ReadTokenHandle.expect("non-null function pointer")(*args.offset(1isize)
                                                                                               as
                                                                                               libc::c_int,
                                                                                           VM_ArgPtr(*args.offset(2isize))
                                                                                               as
                                                                                               *mut pc_token_t)
                       as intptr_t
        }
        61 => {
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
        62 => { S_StopBackgroundTrack(); return 0i32 as intptr_t }
        63 => {
            S_StartBackgroundTrack(VM_ArgPtr(*args.offset(1isize)) as
                                       *const libc::c_char,
                                   VM_ArgPtr(*args.offset(2isize)) as
                                       *const libc::c_char);
            return 0i32 as intptr_t
        }
        64 => {
            return Com_RealTime(VM_ArgPtr(*args.offset(1isize)) as
                                    *mut qtime_t) as intptr_t
        }
        75 => {
            Com_DPrintf(b"UI_CIN_PlayCinematic\n\x00" as *const u8 as
                            *const libc::c_char);
            return CIN_PlayCinematic(VM_ArgPtr(*args.offset(1isize)) as
                                         *const libc::c_char,
                                     *args.offset(2isize) as libc::c_int,
                                     *args.offset(3isize) as libc::c_int,
                                     *args.offset(4isize) as libc::c_int,
                                     *args.offset(5isize) as libc::c_int,
                                     *args.offset(6isize) as libc::c_int) as
                       intptr_t
        }
        76 => {
            return CIN_StopCinematic(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        77 => {
            return CIN_RunCinematic(*args.offset(1isize) as libc::c_int) as
                       intptr_t
        }
        78 => {
            CIN_DrawCinematic(*args.offset(1isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        79 => {
            CIN_SetExtents(*args.offset(1isize) as libc::c_int,
                           *args.offset(2isize) as libc::c_int,
                           *args.offset(3isize) as libc::c_int,
                           *args.offset(4isize) as libc::c_int,
                           *args.offset(5isize) as libc::c_int);
            return 0i32 as intptr_t
        }
        80 => {
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
        81 => {
            return CL_CDKeyValidate(VM_ArgPtr(*args.offset(1isize)) as
                                        *const libc::c_char,
                                    VM_ArgPtr(*args.offset(2isize)) as
                                        *const libc::c_char) as intptr_t
        }
        _ => {
            Com_Error(ERR_DROP as libc::c_int,
                      b"Bad UI system trap: %ld\x00" as *const u8 as
                          *const libc::c_char, *args.offset(0isize));
        }
    };
}
/*
====================
FloatAsInt
====================
*/
unsafe extern "C" fn FloatAsInt(mut f: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.f = f;
    return fi.i;
}
/*
====================
CLUI_SetCDKey
====================
*/
unsafe extern "C" fn CLUI_SetCDKey(mut buf: *mut libc::c_char) {
    let mut gamedir: *const libc::c_char = 0 as *const libc::c_char;
    gamedir =
        Cvar_VariableString(b"fs_game\x00" as *const u8 as
                                *const libc::c_char);
    if 0 != UI_usesUniqueCDKey() as libc::c_uint &&
           *gamedir.offset(0isize) as libc::c_int != 0i32 {
        memcpy(&mut cl_cdkey[16usize] as *mut libc::c_char as
                   *mut libc::c_void, buf as *const libc::c_void,
               16i32 as libc::c_ulong);
        cl_cdkey[32usize] = 0i32 as libc::c_char;
        cvar_modifiedFlags |= 0x1i32
    } else {
        memcpy(cl_cdkey.as_mut_ptr() as *mut libc::c_void,
               buf as *const libc::c_void, 16i32 as libc::c_ulong);
        cvar_modifiedFlags |= 0x1i32
    };
}
/*
====================
CLUI_GetCDKey
====================
*/
unsafe extern "C" fn CLUI_GetCDKey(mut buf: *mut libc::c_char,
                                   mut buflen: libc::c_int) {
    let mut gamedir: *const libc::c_char = 0 as *const libc::c_char;
    gamedir =
        Cvar_VariableString(b"fs_game\x00" as *const u8 as
                                *const libc::c_char);
    if 0 != UI_usesUniqueCDKey() as libc::c_uint &&
           *gamedir.offset(0isize) as libc::c_int != 0i32 {
        memcpy(buf as *mut libc::c_void,
               &mut cl_cdkey[16usize] as *mut libc::c_char as
                   *const libc::c_void, 16i32 as libc::c_ulong);
        *buf.offset(16isize) = 0i32 as libc::c_char
    } else {
        memcpy(buf as *mut libc::c_void,
               cl_cdkey.as_mut_ptr() as *const libc::c_void,
               16i32 as libc::c_ulong);
        *buf.offset(16isize) = 0i32 as libc::c_char
    };
}
/*
====================
LAN_CompareServers
====================
*/
unsafe extern "C" fn LAN_CompareServers(mut source: libc::c_int,
                                        mut sortKey: libc::c_int,
                                        mut sortDir: libc::c_int,
                                        mut s1: libc::c_int,
                                        mut s2: libc::c_int) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut server1: *mut serverInfo_t = 0 as *mut serverInfo_t;
    let mut server2: *mut serverInfo_t = 0 as *mut serverInfo_t;
    let mut clients1: libc::c_int = 0;
    let mut clients2: libc::c_int = 0;
    server1 = LAN_GetServerPtr(source, s1);
    server2 = LAN_GetServerPtr(source, s2);
    if server1.is_null() || server2.is_null() { return 0i32 }
    res = 0i32;
    match sortKey {
        0 => {
            res =
                Q_stricmp((*server1).hostName.as_mut_ptr(),
                          (*server2).hostName.as_mut_ptr())
        }
        1 => {
            res =
                Q_stricmp((*server1).mapName.as_mut_ptr(),
                          (*server2).mapName.as_mut_ptr())
        }
        2 => {
            if (*server1).clients == (*server2).clients {
                clients1 = (*server1).maxClients;
                clients2 = (*server2).maxClients
            } else {
                clients1 = (*server1).clients;
                clients2 = (*server2).clients
            }
            if clients1 < clients2 {
                res = -1i32
            } else if clients1 > clients2 { res = 1i32 } else { res = 0i32 }
        }
        3 => {
            if (*server1).gameType < (*server2).gameType {
                res = -1i32
            } else if (*server1).gameType > (*server2).gameType {
                res = 1i32
            } else { res = 0i32 }
        }
        4 => {
            if (*server1).ping < (*server2).ping {
                res = -1i32
            } else if (*server1).ping > (*server2).ping {
                res = 1i32
            } else { res = 0i32 }
        }
        _ => { }
    }
    if 0 != sortDir {
        if res < 0i32 { return 1i32 }
        if res > 0i32 { return -1i32 }
        return 0i32
    }
    return res;
}
/*
====================
LAN_GetServerPtr
====================
*/
unsafe extern "C" fn LAN_GetServerPtr(mut source: libc::c_int,
                                      mut n: libc::c_int)
 -> *mut serverInfo_t {
    match source {
        0 => {
            if n >= 0i32 && n < 128i32 {
                return &mut cls.localServers[n as usize] as *mut serverInfo_t
            }
        }
        1 | 2 => {
            if n >= 0i32 && n < 4096i32 {
                return &mut cls.globalServers[n as usize] as *mut serverInfo_t
            }
        }
        3 => {
            if n >= 0i32 && n < 128i32 {
                return &mut cls.favoriteServers[n as usize] as
                           *mut serverInfo_t
            }
        }
        _ => { }
    }
    return 0 as *mut serverInfo_t;
}
/*
====================
LAN_GetServerStatus
====================
*/
#[no_mangle]
pub unsafe extern "C" fn LAN_GetServerStatus(mut serverAddress:
                                                 *mut libc::c_char,
                                             mut serverStatus:
                                                 *mut libc::c_char,
                                             mut maxLen: libc::c_int)
 -> libc::c_int {
    return CL_ServerStatus(serverAddress, serverStatus, maxLen);
}
/*
====================
LAN_ResetPings
====================
*/
unsafe extern "C" fn LAN_ResetPings(mut source: libc::c_int) {
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut servers: *mut serverInfo_t = 0 as *mut serverInfo_t;
    count = 0i32;
    match source {
        0 => {
            servers = &mut cls.localServers[0usize] as *mut serverInfo_t;
            count = 128i32
        }
        1 | 2 => {
            servers = &mut cls.globalServers[0usize] as *mut serverInfo_t;
            count = 4096i32
        }
        3 => {
            servers = &mut cls.favoriteServers[0usize] as *mut serverInfo_t;
            count = 128i32
        }
        _ => { }
    }
    if !servers.is_null() {
        i = 0i32;
        while i < count { (*servers.offset(i as isize)).ping = -1i32; i += 1 }
    };
}
/*
=======================
LAN_UpdateVisiblePings
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn LAN_UpdateVisiblePings(mut source: libc::c_int)
 -> qboolean {
    return CL_UpdateVisiblePings_f(source);
}
/*
=======================
LAN_ServerIsVisible
=======================
*/
unsafe extern "C" fn LAN_ServerIsVisible(mut source: libc::c_int,
                                         mut n: libc::c_int) -> libc::c_int {
    match source {
        0 => {
            if n >= 0i32 && n < 128i32 {
                return cls.localServers[n as usize].visible as libc::c_int
            }
        }
        1 | 2 => {
            if n >= 0i32 && n < 4096i32 {
                return cls.globalServers[n as usize].visible as libc::c_int
            }
        }
        3 => {
            if n >= 0i32 && n < 128i32 {
                return cls.favoriteServers[n as usize].visible as libc::c_int
            }
        }
        _ => { }
    }
    return qfalse as libc::c_int;
}
/*
====================
LAN_MarkServerVisible
====================
*/
unsafe extern "C" fn LAN_MarkServerVisible(mut source: libc::c_int,
                                           mut n: libc::c_int,
                                           mut visible: qboolean) {
    if n == -1i32 {
        let mut count: libc::c_int = 128i32;
        let mut server: *mut serverInfo_t = 0 as *mut serverInfo_t;
        match source {
            0 => {
                server = &mut cls.localServers[0usize] as *mut serverInfo_t
            }
            1 | 2 => {
                server = &mut cls.globalServers[0usize] as *mut serverInfo_t;
                count = 4096i32
            }
            3 => {
                server = &mut cls.favoriteServers[0usize] as *mut serverInfo_t
            }
            _ => { }
        }
        if !server.is_null() {
            n = 0i32;
            while n < count {
                (*server.offset(n as isize)).visible = visible;
                n += 1
            }
        }
    } else {
        match source {
            0 => {
                if n >= 0i32 && n < 128i32 {
                    cls.localServers[n as usize].visible = visible
                }
            }
            1 | 2 => {
                if n >= 0i32 && n < 4096i32 {
                    cls.globalServers[n as usize].visible = visible
                }
            }
            3 => {
                if n >= 0i32 && n < 128i32 {
                    cls.favoriteServers[n as usize].visible = visible
                }
            }
            _ => { }
        }
    };
}
/*
====================
LAN_GetServerPing
====================
*/
unsafe extern "C" fn LAN_GetServerPing(mut source: libc::c_int,
                                       mut n: libc::c_int) -> libc::c_int {
    let mut server: *mut serverInfo_t = 0 as *mut serverInfo_t;
    match source {
        0 => {
            if n >= 0i32 && n < 128i32 {
                server =
                    &mut cls.localServers[n as usize] as *mut serverInfo_t
            }
        }
        1 | 2 => {
            if n >= 0i32 && n < 4096i32 {
                server =
                    &mut cls.globalServers[n as usize] as *mut serverInfo_t
            }
        }
        3 => {
            if n >= 0i32 && n < 128i32 {
                server =
                    &mut cls.favoriteServers[n as usize] as *mut serverInfo_t
            }
        }
        _ => { }
    }
    if !server.is_null() { return (*server).ping }
    return -1i32;
}
/*
====================
LAN_GetServerInfo
====================
*/
unsafe extern "C" fn LAN_GetServerInfo(mut source: libc::c_int,
                                       mut n: libc::c_int,
                                       mut buf: *mut libc::c_char,
                                       mut buflen: libc::c_int) {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut server: *mut serverInfo_t = 0 as *mut serverInfo_t;
    info[0usize] = '\u{0}' as i32 as libc::c_char;
    match source {
        0 => {
            if n >= 0i32 && n < 128i32 {
                server =
                    &mut cls.localServers[n as usize] as *mut serverInfo_t
            }
        }
        1 | 2 => {
            if n >= 0i32 && n < 4096i32 {
                server =
                    &mut cls.globalServers[n as usize] as *mut serverInfo_t
            }
        }
        3 => {
            if n >= 0i32 && n < 128i32 {
                server =
                    &mut cls.favoriteServers[n as usize] as *mut serverInfo_t
            }
        }
        _ => { }
    }
    if !server.is_null() && !buf.is_null() {
        *buf.offset(0isize) = '\u{0}' as i32 as libc::c_char;
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"hostname\x00" as *const u8 as
                                *const libc::c_char,
                            (*server).hostName.as_mut_ptr());
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"mapname\x00" as *const u8 as
                                *const libc::c_char,
                            (*server).mapName.as_mut_ptr());
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"clients\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char, (*server).clients));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"sv_maxclients\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*server).maxClients));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"ping\x00" as *const u8 as *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char, (*server).ping));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"minping\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char, (*server).minPing));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"maxping\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char, (*server).maxPing));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"game\x00" as *const u8 as *const libc::c_char,
                            (*server).game.as_mut_ptr());
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"gametype\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char, (*server).gameType));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"nettype\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char, (*server).netType));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"addr\x00" as *const u8 as *const libc::c_char,
                            NET_AdrToStringwPort((*server).adr));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"punkbuster\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*server).punkbuster));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"g_needpass\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*server).g_needpass));
        Info_SetValueForKey(info.as_mut_ptr(),
                            b"g_humanplayers\x00" as *const u8 as
                                *const libc::c_char,
                            va(b"%i\x00" as *const u8 as *const libc::c_char
                                   as *mut libc::c_char,
                               (*server).g_humanplayers));
        Q_strncpyz(buf, info.as_mut_ptr(), buflen);
    } else if !buf.is_null() {
        *buf.offset(0isize) = '\u{0}' as i32 as libc::c_char
    };
}
/*
====================
LAN_GetLocalServerAddressString
====================
*/
unsafe extern "C" fn LAN_GetServerAddressString(mut source: libc::c_int,
                                                mut n: libc::c_int,
                                                mut buf: *mut libc::c_char,
                                                mut buflen: libc::c_int) {
    match source {
        0 => {
            if n >= 0i32 && n < 128i32 {
                Q_strncpyz(buf,
                           NET_AdrToStringwPort(cls.localServers[n as
                                                                     usize].adr),
                           buflen);
                return
            }
        }
        1 | 2 => {
            if n >= 0i32 && n < 4096i32 {
                Q_strncpyz(buf,
                           NET_AdrToStringwPort(cls.globalServers[n as
                                                                      usize].adr),
                           buflen);
                return
            }
        }
        3 => {
            if n >= 0i32 && n < 128i32 {
                Q_strncpyz(buf,
                           NET_AdrToStringwPort(cls.favoriteServers[n as
                                                                        usize].adr),
                           buflen);
                return
            }
        }
        _ => { }
    }
    *buf.offset(0isize) = '\u{0}' as i32 as libc::c_char;
}
/*
====================
LAN_GetServerCount
====================
*/
unsafe extern "C" fn LAN_GetServerCount(mut source: libc::c_int)
 -> libc::c_int {
    match source {
        0 => { return cls.numlocalservers }
        1 | 2 => { return cls.numglobalservers }
        3 => { return cls.numfavoriteservers }
        _ => { }
    }
    return 0i32;
}
/*
====================
LAN_GetPingInfo
====================
*/
unsafe extern "C" fn LAN_GetPingInfo(mut n: libc::c_int,
                                     mut buf: *mut libc::c_char,
                                     mut buflen: libc::c_int) {
    CL_GetPingInfo(n, buf, buflen);
}
/*
====================
LAN_GetPing
====================
*/
unsafe extern "C" fn LAN_GetPing(mut n: libc::c_int,
                                 mut buf: *mut libc::c_char,
                                 mut buflen: libc::c_int,
                                 mut pingtime: *mut libc::c_int) {
    CL_GetPing(n, buf, buflen, pingtime);
}
/*
====================
LAN_ClearPing
====================
*/
unsafe extern "C" fn LAN_ClearPing(mut n: libc::c_int) { CL_ClearPing(n); }
/*
====================
LAN_GetPingQueueCount
====================
*/
unsafe extern "C" fn LAN_GetPingQueueCount() -> libc::c_int {
    return CL_GetPingQueueCount();
}
/*
====================
LAN_RemoveServer
====================
*/
unsafe extern "C" fn LAN_RemoveServer(mut source: libc::c_int,
                                      mut addr: *const libc::c_char) {
    let mut count: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut servers: *mut serverInfo_t = 0 as *mut serverInfo_t;
    count = 0 as *mut libc::c_int;
    match source {
        0 => {
            count = &mut cls.numlocalservers;
            servers = &mut cls.localServers[0usize] as *mut serverInfo_t
        }
        1 | 2 => {
            count = &mut cls.numglobalservers;
            servers = &mut cls.globalServers[0usize] as *mut serverInfo_t
        }
        3 => {
            count = &mut cls.numfavoriteservers;
            servers = &mut cls.favoriteServers[0usize] as *mut serverInfo_t
        }
        _ => { }
    }
    if !servers.is_null() {
        let mut comp: netadr_t =
            netadr_t{type_0: NA_BAD,
                     ip: [0; 4],
                     ip6: [0; 16],
                     port: 0,
                     scope_id: 0,};
        NET_StringToAdr(addr, &mut comp, NA_UNSPEC);
        i = 0i32;
        while i < *count {
            if 0 !=
                   NET_CompareAdr(comp, (*servers.offset(i as isize)).adr) as
                       u64 {
                let mut j: libc::c_int = i;
                while j < *count - 1i32 {
                    memcpy(&mut *servers.offset(j as isize) as
                               *mut serverInfo_t as *mut libc::c_void,
                           &mut *servers.offset((j + 1i32) as isize) as
                               *mut serverInfo_t as *const libc::c_void,
                           ::std::mem::size_of::<serverInfo_t>() as
                               libc::c_ulong);
                    j += 1
                }
                *count -= 1;
                break ;
            } else { i += 1 }
        }
    };
}
/*
====================
LAN_AddServer
====================
*/
unsafe extern "C" fn LAN_AddServer(mut source: libc::c_int,
                                   mut name: *const libc::c_char,
                                   mut address: *const libc::c_char)
 -> libc::c_int {
    let mut max: libc::c_int = 0;
    let mut count: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut adr: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut servers: *mut serverInfo_t = 0 as *mut serverInfo_t;
    max = 128i32;
    count = 0 as *mut libc::c_int;
    match source {
        0 => {
            count = &mut cls.numlocalservers;
            servers = &mut cls.localServers[0usize] as *mut serverInfo_t
        }
        1 | 2 => {
            max = 4096i32;
            count = &mut cls.numglobalservers;
            servers = &mut cls.globalServers[0usize] as *mut serverInfo_t
        }
        3 => {
            count = &mut cls.numfavoriteservers;
            servers = &mut cls.favoriteServers[0usize] as *mut serverInfo_t
        }
        _ => { }
    }
    if !servers.is_null() && *count < max {
        NET_StringToAdr(address, &mut adr, NA_UNSPEC);
        i = 0i32;
        while i < *count {
            if 0 !=
                   NET_CompareAdr((*servers.offset(i as isize)).adr, adr) as
                       u64 {
                break ;
            }
            i += 1
        }
        if i >= *count {
            (*servers.offset(*count as isize)).adr = adr;
            Q_strncpyz((*servers.offset(*count as
                                            isize)).hostName.as_mut_ptr(),
                       name,
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong as libc::c_int);
            (*servers.offset(*count as isize)).visible = qtrue;
            *count += 1;
            return 1i32
        }
        return 0i32
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn LAN_SaveServersToCache() {
    let mut size: libc::c_int = 0;
    let mut fileOut: fileHandle_t =
        FS_SV_FOpenFileWrite(b"servercache.dat\x00" as *const u8 as
                                 *const libc::c_char);
    FS_Write(&mut cls.numglobalservers as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                 libc::c_int, fileOut);
    FS_Write(&mut cls.numfavoriteservers as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                 libc::c_int, fileOut);
    size =
        (::std::mem::size_of::<[serverInfo_t; 4096]>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<[serverInfo_t; 128]>()
                                             as libc::c_ulong) as libc::c_int;
    FS_Write(&mut size as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                 libc::c_int, fileOut);
    FS_Write(&mut cls.globalServers as *mut [serverInfo_t; 4096] as
                 *const libc::c_void,
             ::std::mem::size_of::<[serverInfo_t; 4096]>() as libc::c_ulong as
                 libc::c_int, fileOut);
    FS_Write(&mut cls.favoriteServers as *mut [serverInfo_t; 128] as
                 *const libc::c_void,
             ::std::mem::size_of::<[serverInfo_t; 128]>() as libc::c_ulong as
                 libc::c_int, fileOut);
    FS_FCloseFile(fileOut);
}
#[no_mangle]
pub unsafe extern "C" fn LAN_LoadCachedServers() {
    let mut size: libc::c_int = 0;
    let mut fileIn: fileHandle_t = 0;
    cls.numfavoriteservers = 0i32;
    cls.numglobalservers = cls.numfavoriteservers;
    cls.numGlobalServerAddresses = 0i32;
    if 0 !=
           FS_SV_FOpenFileRead(b"servercache.dat\x00" as *const u8 as
                                   *const libc::c_char, &mut fileIn) {
        FS_Read(&mut cls.numglobalservers as *mut libc::c_int as
                    *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                    libc::c_int, fileIn);
        FS_Read(&mut cls.numfavoriteservers as *mut libc::c_int as
                    *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                    libc::c_int, fileIn);
        FS_Read(&mut size as *mut libc::c_int as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                    libc::c_int, fileIn);
        if size as libc::c_ulong ==
               (::std::mem::size_of::<[serverInfo_t; 4096]>() as
                    libc::c_ulong).wrapping_add(::std::mem::size_of::<[serverInfo_t; 128]>()
                                                    as libc::c_ulong) {
            FS_Read(&mut cls.globalServers as *mut [serverInfo_t; 4096] as
                        *mut libc::c_void,
                    ::std::mem::size_of::<[serverInfo_t; 4096]>() as
                        libc::c_ulong as libc::c_int, fileIn);
            FS_Read(&mut cls.favoriteServers as *mut [serverInfo_t; 128] as
                        *mut libc::c_void,
                    ::std::mem::size_of::<[serverInfo_t; 128]>() as
                        libc::c_ulong as libc::c_int, fileIn);
        } else {
            cls.numfavoriteservers = 0i32;
            cls.numglobalservers = cls.numfavoriteservers;
            cls.numGlobalServerAddresses = 0i32
        }
        FS_FCloseFile(fileIn);
    };
}
/*
====================
GetConfigString
====================
*/
unsafe extern "C" fn GetConfigString(mut index: libc::c_int,
                                     mut buf: *mut libc::c_char,
                                     mut size: libc::c_int) -> libc::c_int {
    let mut offset: libc::c_int = 0;
    if index < 0i32 || index >= 1024i32 { return qfalse as libc::c_int }
    offset = cl.gameState.stringOffsets[index as usize];
    if 0 == offset {
        if 0 != size { *buf.offset(0isize) = 0i32 as libc::c_char }
        return qfalse as libc::c_int
    }
    Q_strncpyz(buf,
               cl.gameState.stringData.as_mut_ptr().offset(offset as isize),
               size);
    return qtrue as libc::c_int;
}
/*
====================
CL_GetGlConfig
====================
*/
unsafe extern "C" fn CL_GetGlconfig(mut config: *mut glconfig_t) {
    *config = cls.glconfig;
}
/*
====================
GetClientState
====================
*/
unsafe extern "C" fn GetClientState(mut state: *mut uiClientState_t) {
    (*state).connectPacketCount = clc.connectPacketCount;
    (*state).connState = clc.state;
    Q_strncpyz((*state).servername.as_mut_ptr(), clc.servername.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz((*state).updateInfoString.as_mut_ptr(),
               cls.updateInfoString.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz((*state).messageString.as_mut_ptr(),
               clc.serverMessage.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    (*state).clientNum = cl.snap.ps.clientNum;
}
/*
====================
CL_GetClipboardData
====================
*/
unsafe extern "C" fn CL_GetClipboardData(mut buf: *mut libc::c_char,
                                         mut buflen: libc::c_int) {
    let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
    cbd = Sys_GetClipboardData();
    if cbd.is_null() { *buf = 0i32 as libc::c_char; return }
    Q_strncpyz(buf, cbd, buflen);
    Z_Free(cbd as *mut libc::c_void);
}
/*
====================
Key_GetBindingBuf
====================
*/
unsafe extern "C" fn Key_GetBindingBuf(mut keynum: libc::c_int,
                                       mut buf: *mut libc::c_char,
                                       mut buflen: libc::c_int) {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = Key_GetBinding(keynum);
    if !value.is_null() {
        Q_strncpyz(buf, value, buflen);
    } else { *buf = 0i32 as libc::c_char };
}
/*
====================
Key_KeynumToStringBuf
====================
*/
unsafe extern "C" fn Key_KeynumToStringBuf(mut keynum: libc::c_int,
                                           mut buf: *mut libc::c_char,
                                           mut buflen: libc::c_int) {
    Q_strncpyz(buf, Key_KeynumToString(keynum), buflen);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ShutdownUI() {
    Key_SetCatcher(Key_GetCatcher() & !0x2i32);
    cls.uiStarted = qfalse;
    if uivm.is_null() { return }
    VM_Call(uivm, UI_SHUTDOWN as libc::c_int);
    VM_Free(uivm);
    uivm = 0 as *mut vm_t;
}