#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           libc,
           ptr_wrapping_offset_from)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int16_t = libc::c_short;
    pub type __int32_t = libc::c_int;
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/bits/types/struct_FILE.h"]
pub mod struct_FILE_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    pub type _IO_lock_t = ();
    use super::{libc};
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::{size_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
#[header_src = "/usr/include/bits/types/FILE.h"]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::{_IO_FILE};
}
#[header_src = "/usr/include/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
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
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        pub fn rand() -> libc::c_int;
        #[no_mangle]
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    pub type unnamed = libc::c_uint;
    pub const _ISalnum: unnamed = 8;
    pub const _ISpunct: unnamed = 4;
    pub const _IScntrl: unnamed = 2;
    pub const _ISblank: unnamed = 1;
    pub const _ISgraph: unnamed = 32768;
    pub const _ISprint: unnamed = 16384;
    pub const _ISspace: unnamed = 8192;
    pub const _ISxdigit: unnamed = 4096;
    pub const _ISdigit: unnamed = 2048;
    pub const _ISalpha: unnamed = 1024;
    pub const _ISlower: unnamed = 512;
    pub const _ISupper: unnamed = 256;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
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
    pub type unnamed_0 = libc::c_uint;
    // add to end of the command buffer (normal case)
    pub const EXEC_APPEND: unnamed_0 = 2;
    // because some commands might cause the VM to be unloaded...
    // insert at current position, but don't run yet
    pub const EXEC_INSERT: unnamed_0 = 1;
    // don't return until completed, a VM should NEVER use this,
    pub const EXEC_NOW: unnamed_0 = 0;
    // parameters to the main Error routine
    pub type unnamed_1 = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed_1 = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed_1 = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed_1 = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed_1 = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed_1 = 0;
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
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn qftolsse(f: libc::c_float) -> libc::c_long;
        #[no_mangle]
        pub fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                            right: *mut vec_t, up: *mut vec_t);
        //=============================================
        #[no_mangle]
        pub fn Com_Clamp(min: libc::c_float, max: libc::c_float,
                         value: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn COM_SkipPath(pathname: *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn COM_StripExtension(in_0: *const libc::c_char,
                                  out: *mut libc::c_char,
                                  destsize: libc::c_int);
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        #[no_mangle]
        pub fn Com_SkipTokens(s: *mut libc::c_char, numTokens: libc::c_int,
                              sep: *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_RandomBytes(string: *mut byte, len: libc::c_int);
        #[no_mangle]
        pub fn Q_isanumber(s: *const libc::c_char) -> qboolean;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                         n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Q_stricmpn(s1: *const libc::c_char, s2: *const libc::c_char,
                          n: libc::c_int) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
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
    pub type unnamed_2 = libc::c_uint;
    pub const TAG_STATIC: unnamed_2 = 5;
    pub const TAG_SMALL: unnamed_2 = 4;
    pub const TAG_RENDERER: unnamed_2 = 3;
    pub const TAG_BOTLIB: unnamed_2 = 2;
    pub const TAG_GENERAL: unnamed_2 = 1;
    pub const TAG_FREE: unnamed_2 = 0;
    use super::q_shared_h::{qboolean, byte, entityState_s, cvar_t,
                            fileHandle_t, qtime_t};
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
        pub fn MSG_Init(buf: *mut msg_t, data: *mut byte,
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
        pub fn MSG_WriteBigString(sb: *mut msg_t, s: *const libc::c_char);
        #[no_mangle]
        pub fn MSG_BeginReadingOOB(sb: *mut msg_t);
        #[no_mangle]
        pub fn MSG_ReadLong(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadString(sb: *mut msg_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn MSG_ReadStringLine(sb: *mut msg_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn MSG_WriteDeltaEntity(msg: *mut msg_t, from: *mut entityState_s,
                                    to: *mut entityState_s, force: qboolean);
        #[no_mangle]
        pub fn NET_SendPacket(sock: netsrc_t, length: libc::c_int,
                              data: *const libc::c_void, to: netadr_t);
        #[no_mangle]
        pub fn NET_OutOfBandPrint(net_socket: netsrc_t, adr: netadr_t,
                                  format: *const libc::c_char, ...);
        #[no_mangle]
        pub fn NET_OutOfBandData(sock: netsrc_t, adr: netadr_t,
                                 format: *mut byte, len: libc::c_int);
        #[no_mangle]
        pub fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
        #[no_mangle]
        pub fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_AdrToStringwPort(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn NET_StringToAdr(s: *const libc::c_char, a: *mut netadr_t,
                               family: netadrtype_t) -> libc::c_int;
        #[no_mangle]
        pub fn Netchan_Setup(sock: netsrc_t, chan: *mut netchan_t,
                             adr: netadr_t, qport: libc::c_int,
                             challenge: libc::c_int, compat: qboolean);
        /*
==============================================================

PROTOCOL

==============================================================
*/
        // 1.31 - 67
        // maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
        #[no_mangle]
        pub static mut demo_protocols: [libc::c_int; 0];
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        // Adds command text at the end of the buffer, does NOT add a final \n
        #[no_mangle]
        pub fn Cbuf_ExecuteText(exec_when: libc::c_int,
                                text: *const libc::c_char);
        // this can be used in place of either Cbuf_AddText or Cbuf_InsertText
        #[no_mangle]
        pub fn Cbuf_Execute();
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
        #[no_mangle]
        pub fn Cmd_Args() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgsFrom(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_Cmd() -> *mut libc::c_char;
        // The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
        #[no_mangle]
        pub fn Cmd_TokenizeString(text: *const libc::c_char);
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
        // will create the variable with no flags if it doesn't exist
        #[no_mangle]
        pub fn Cvar_Set2(var_name: *const libc::c_char,
                         value: *const libc::c_char, force: qboolean)
         -> *mut cvar_t;
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
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
        pub fn Cvar_VariableStringBuffer(var_name: *const libc::c_char,
                                         buffer: *mut libc::c_char,
                                         bufsize: libc::c_int);
        #[no_mangle]
        pub fn Cvar_ForceReset(var_name: *const libc::c_char);
        #[no_mangle]
        pub fn Cvar_InfoString(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_CheckRange(cv: *mut cvar_t, minVal: libc::c_float,
                               maxVal: libc::c_float,
                               shouldBeIntegral: qboolean);
        #[no_mangle]
        pub fn Cvar_SetDescription(var: *mut cvar_t,
                                   var_description: *const libc::c_char);
        #[no_mangle]
        pub static mut cvar_modifiedFlags: libc::c_int;
        #[no_mangle]
        pub fn FS_ConditionalRestart(checksumFeed: libc::c_int,
                                     disconnect: qboolean) -> qboolean;
        #[no_mangle]
        pub fn FS_Restart(checksumFeed: libc::c_int);
        #[no_mangle]
        pub fn FS_ListFiles(directory: *const libc::c_char,
                            extension: *const libc::c_char,
                            numfiles: *mut libc::c_int)
         -> *mut *mut libc::c_char;
        // directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
        #[no_mangle]
        pub fn FS_FreeFileList(list: *mut *mut libc::c_char);
        #[no_mangle]
        pub fn FS_FileExists(file: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn FS_BuildOSPath(base: *const libc::c_char,
                              game: *const libc::c_char,
                              qpath: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn FS_CompareZipChecksum(zipfile: *const libc::c_char)
         -> qboolean;
        #[no_mangle]
        pub fn FS_FOpenFileWrite(qpath: *const libc::c_char) -> fileHandle_t;
        // will properly create any needed paths and deal with seperater character issues
        #[no_mangle]
        pub fn FS_SV_FOpenFileWrite(filename: *const libc::c_char)
         -> fileHandle_t;
        #[no_mangle]
        pub fn FS_SV_FOpenFileRead(filename: *const libc::c_char,
                                   fp: *mut fileHandle_t) -> libc::c_long;
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        // if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
        #[no_mangle]
        pub fn FS_FileIsInPAK(filename: *const libc::c_char,
                              pChecksum: *mut libc::c_int) -> libc::c_int;
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
        // forces flush on files we're writing to.
        #[no_mangle]
        pub fn FS_FreeFile(buffer: *mut libc::c_void);
        // frees the memory returned by FS_ReadFile
        #[no_mangle]
        pub fn FS_WriteFile(qpath: *const libc::c_char,
                            buffer: *const libc::c_void, size: libc::c_int);
        #[no_mangle]
        pub fn FS_Printf(f: fileHandle_t, fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn FS_LoadedPakNames() -> *const libc::c_char;
        // Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
        #[no_mangle]
        pub fn FS_ReferencedPakNames() -> *const libc::c_char;
        #[no_mangle]
        pub fn FS_ReferencedPakPureChecksums() -> *const libc::c_char;
        // Returns a space separated string containing the checksums of all loaded 
// AND referenced pk3 files. Servers with sv_pure set will get this string 
// back from clients for pure validation 
        #[no_mangle]
        pub fn FS_ClearPakReferences(flags: libc::c_int);
        // clears referenced booleans on loaded pk3s
        #[no_mangle]
        pub fn FS_PureServerSetReferencedPaks(pakSums: *const libc::c_char,
                                              pakNames: *const libc::c_char);
        #[no_mangle]
        pub fn FS_PureServerSetLoadedPaks(pakSums: *const libc::c_char,
                                          pakNames: *const libc::c_char);
        #[no_mangle]
        pub fn FS_ComparePaks(neededpaks: *mut libc::c_char, len: libc::c_int,
                              dlstring: qboolean) -> qboolean;
        #[no_mangle]
        pub fn Field_CompleteFilename(dir: *const libc::c_char,
                                      ext: *const libc::c_char,
                                      stripExt: qboolean,
                                      allowNonPureFilesOnDisk: qboolean);
        #[no_mangle]
        pub fn Field_CompleteCommand(cmd: *mut libc::c_char,
                                     doCommands: qboolean, doCvars: qboolean);
        #[no_mangle]
        pub fn Field_CompletePlayerName(names: *mut *const libc::c_char,
                                        count: libc::c_int);
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
        pub fn Com_EventLoop() -> libc::c_int;
        #[no_mangle]
        pub fn Info_Print(s: *const libc::c_char);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // will be journaled properly
        #[no_mangle]
        pub fn Com_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Com_MD5File(filename: *const libc::c_char, length: libc::c_int,
                           prefix: *const libc::c_char,
                           prefix_len: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_RealTime(qtime: *mut qtime_t) -> libc::c_int;
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
        pub static mut com_timescale: *mut cvar_t;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        #[no_mangle]
        pub static mut com_cl_running: *mut cvar_t;
        #[no_mangle]
        pub static mut com_version: *mut cvar_t;
        #[no_mangle]
        pub static mut com_standalone: *mut cvar_t;
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
        #[no_mangle]
        pub static mut com_errorEntered: qboolean;
        #[no_mangle]
        pub static mut com_fullyInitialized: qboolean;
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
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Hunk_Clear();
        #[no_mangle]
        pub fn Hunk_ClearToMark();
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
        #[no_mangle]
        pub fn Sys_LowPhysicalMemory() -> qboolean;
        #[no_mangle]
        pub fn Sys_SetEnv(name: *const libc::c_char,
                          value: *const libc::c_char);
        #[no_mangle]
        pub fn IN_Restart();
        #[no_mangle]
        pub fn IN_Shutdown();
        //
// input interface
//
        #[no_mangle]
        pub fn IN_Init(windowData: *mut libc::c_void);
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Sys_ShowIP();
        // for writing the config files
        #[no_mangle]
        pub fn S_ClearSoundBuffer();
        #[no_mangle]
        pub fn SV_Frame(msec: libc::c_int);
        #[no_mangle]
        pub fn SV_Shutdown(finalmsg: *mut libc::c_char);
        //Does NOT parse port numbers, only base addresses.
        #[no_mangle]
        pub fn Sys_IsLANAddress(adr: netadr_t) -> qboolean;
        // call before filesystem access
        #[no_mangle]
        pub fn SCR_DebugGraph(value: libc::c_float);
    }
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
    //
// these are the functions imported by the refresh module
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refimport_t {
        pub Printf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: *const libc::c_char, ...)
                               -> ()>,
        pub Error: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *const libc::c_char, ...)
                              -> !>,
        pub Milliseconds: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub Hunk_AllocDebug: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: ha_pref,
                                                         _: *mut libc::c_char,
                                                         _: *mut libc::c_char,
                                                         _: libc::c_int)
                                        -> *mut libc::c_void>,
        pub Hunk_AllocateTempMemory: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int)
                                                -> *mut libc::c_void>,
        pub Hunk_FreeTempMemory: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_void)
                                            -> ()>,
        pub Malloc: Option<unsafe extern "C" fn(_: libc::c_int)
                               -> *mut libc::c_void>,
        pub Free: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub Cvar_Get: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int)
                                 -> *mut cvar_t>,
        pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const libc::c_char)
                                 -> ()>,
        pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: libc::c_float)
                                      -> ()>,
        pub Cvar_CheckRange: Option<unsafe extern "C" fn(_: *mut cvar_t,
                                                         _: libc::c_float,
                                                         _: libc::c_float,
                                                         _: qboolean) -> ()>,
        pub Cvar_SetDescription: Option<unsafe extern "C" fn(_: *mut cvar_t,
                                                             _:
                                                                 *const libc::c_char)
                                            -> ()>,
        pub Cvar_VariableIntegerValue: Option<unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> libc::c_int>,
        pub Cmd_AddCommand: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _:
                                                            Option<unsafe extern "C" fn()
                                                                       -> ()>)
                                       -> ()>,
        pub Cmd_RemoveCommand: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_char)
                                          -> ()>,
        pub Cmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub Cmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> *mut libc::c_char>,
        pub Cmd_ExecuteText: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _:
                                                             *const libc::c_char)
                                        -> ()>,
        pub CM_ClusterPVS: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> *mut byte>,
        pub CM_DrawDebugSurface: Option<unsafe extern "C" fn(_:
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 *mut libc::c_float)
                                                                            ->
                                                                                ()>)
                                            -> ()>,
        pub FS_FileIsInPAK: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int>,
        pub FS_ReadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _:
                                                         *mut *mut libc::c_void)
                                    -> libc::c_long>,
        pub FS_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
        pub FS_ListFiles: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *const libc::c_char,
                                                      _: *mut libc::c_int)
                                     -> *mut *mut libc::c_char>,
        pub FS_FreeFileList: Option<unsafe extern "C" fn(_:
                                                             *mut *mut libc::c_char)
                                        -> ()>,
        pub FS_WriteFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *const libc::c_void,
                                                      _: libc::c_int) -> ()>,
        pub FS_FileExists: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> qboolean>,
        pub CIN_UploadCinematic: Option<unsafe extern "C" fn(_: libc::c_int)
                                            -> ()>,
        pub CIN_PlayCinematic: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_char,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int)
                                          -> libc::c_int>,
        pub CIN_RunCinematic: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> e_status>,
        pub CL_WriteAVIVideoFrame: Option<unsafe extern "C" fn(_: *const byte,
                                                               _: libc::c_int)
                                              -> ()>,
        pub IN_Init: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub IN_Shutdown: Option<unsafe extern "C" fn() -> ()>,
        pub IN_Restart: Option<unsafe extern "C" fn() -> ()>,
        pub ftol: Option<unsafe extern "C" fn(_: libc::c_float)
                             -> libc::c_long>,
        pub Sys_SetEnv: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char)
                                   -> ()>,
        pub Sys_GLimpSafeInit: Option<unsafe extern "C" fn() -> ()>,
        pub Sys_GLimpInit: Option<unsafe extern "C" fn() -> ()>,
        pub Sys_LowPhysicalMemory: Option<unsafe extern "C" fn() -> qboolean>,
    }
    // this is the only function actually exported at the linker level
// If the module can't init to a valid rendering state, NULL will be
// returned.
    pub type GetRefAPI_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut refimport_t)
                   -> *mut refexport_t>;
    use super::q_shared_h::{qboolean, qhandle_t, byte, vec_t, vec3_t,
                            markFragment_t, orientation_t, fontInfo_t,
                            ha_pref, cvar_t, e_status};
    use super::tr_types_h::{glconfig_t, refEntity_t, polyVert_t, refdef_t,
                            stereoFrame_t};
    use super::{libc};
}
#[header_src =
      "ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
    pub type stereoFrame_t = libc::c_uint;
    pub const STEREO_RIGHT: stereoFrame_t = 2;
    pub const STEREO_LEFT: stereoFrame_t = 1;
    pub const STEREO_CENTER: stereoFrame_t = 0;
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
    use super::{libc};
    use super::q_shared_h::{vec3_t, byte, qhandle_t, qboolean};
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
    /*
==================================================================

the clientStatic_t structure is never wiped, and is used even when
no client connection is active at all
(except when CL_Shutdown is called)

==================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ping_t {
        pub adr: netadr_t,
        pub start: libc::c_int,
        pub time: libc::c_int,
        pub info: [libc::c_char; 1024],
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
    use super::{libc};
    use super::q_shared_h::{qboolean, gameState_t, usercmd_t, vec3_t,
                            entityState_t, byte, playerState_t, connstate_t,
                            fileHandle_t, qhandle_t, e_status, cvar_t};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t, msg_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    use super::tr_public_h::{refexport_t};
    extern "C" {
        //
// cl_scrn.c
//
        #[no_mangle]
        pub fn SCR_Init();
        #[no_mangle]
        pub fn CL_WriteAVIVideoFrame(imageBuffer: *const byte,
                                     size: libc::c_int);
        #[no_mangle]
        pub fn CIN_RunCinematic(handle: libc::c_int) -> e_status;
        #[no_mangle]
        pub fn CIN_PlayCinematic(arg0: *const libc::c_char, xpos: libc::c_int,
                                 ypos: libc::c_int, width: libc::c_int,
                                 height: libc::c_int, bits: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        pub fn CIN_UploadCinematic(handle: libc::c_int);
        #[no_mangle]
        pub fn CL_CloseAVI() -> qboolean;
        //
// cl_avi.c
//
        #[no_mangle]
        pub fn CL_OpenAVIForWriting(filename: *const libc::c_char)
         -> qboolean;
        #[no_mangle]
        pub fn Key_SetCatcher(catcher: libc::c_int);
        #[no_mangle]
        pub fn Con_Close();
        #[no_mangle]
        pub fn SCR_UpdateScreen();
        #[no_mangle]
        pub fn CL_VideoRecording() -> qboolean;
        //
// cl_parse.c
//
        #[no_mangle]
        pub static mut cl_connectedToPureServer: libc::c_int;
        #[no_mangle]
        pub fn CL_WritePacket();
        #[no_mangle]
        pub fn SCR_StopCinematic();
        // interface to ui dll or vm
        #[no_mangle]
        pub static mut uivm: *mut vm_t;
        //
// cl_cin.c
//
        #[no_mangle]
        pub fn CL_PlayCinematic_f();
        #[no_mangle]
        pub fn CL_ParseServerMessage(msg: *mut msg_t);
        //
// cl_cgame.c
//
        #[no_mangle]
        pub fn CL_InitCGame();
        //
// cl_ui.c
//
        #[no_mangle]
        pub fn CL_InitUI();
        // the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original 
        #[no_mangle]
        pub static mut g_console_field_width: libc::c_int;
        #[no_mangle]
        pub fn CL_ShutdownCGame();
        #[no_mangle]
        pub fn CL_ShutdownUI();
        #[no_mangle]
        pub static mut cl_run: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_anglespeedkey: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_pitchspeed: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_yawspeed: *mut cvar_t;
        #[no_mangle]
        pub fn CL_InitInput();
        #[no_mangle]
        pub fn Con_Init();
        #[no_mangle]
        pub fn Con_Shutdown();
        #[no_mangle]
        pub fn CL_ShutdownInput();
        #[no_mangle]
        pub fn Con_RunConsole();
        #[no_mangle]
        pub fn SCR_RunCinematic();
        #[no_mangle]
        pub fn CL_SetCGameTime();
        #[no_mangle]
        pub fn CL_SendCmd();
        #[no_mangle]
        pub static mut cl_timegraph: *mut cvar_t;
        #[no_mangle]
        pub fn CL_TakeVideoFrame();
        #[no_mangle]
        pub fn Key_GetCatcher() -> libc::c_int;
        #[no_mangle]
        pub fn CL_Netchan_Process(chan: *mut netchan_t, msg: *mut msg_t)
         -> qboolean;
    }
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
        /* * Frees an <code>OpusDecoder</code> allocated by opus_decoder_create().
  * @param[in] st <tt>OpusDecoder*</tt>: State to be freed.
  */
        #[no_mangle]
        pub fn opus_decoder_destroy(st: *mut OpusDecoder);
        /* * Frees an <code>OpusEncoder</code> allocated by opus_encoder_create().
  * @param[in] st <tt>OpusEncoder*</tt>: State to be freed.
  */
        #[no_mangle]
        pub fn opus_encoder_destroy(st: *mut OpusEncoder);
        /* * Perform a CTL function on an Opus encoder.
  *
  * Generally the request and subsequent arguments are generated
  * by a convenience macro.
  * @param st <tt>OpusEncoder*</tt>: Encoder state.
  * @param request This and all remaining parameters should be replaced by one
  *                of the convenience macros in @ref opus_genericctls or
  *                @ref opus_encoderctls.
  * @see opus_genericctls
  * @see opus_encoderctls
  */
        #[no_mangle]
        pub fn opus_encoder_ctl(st: *mut OpusEncoder,
                                request: libc::c_int, ...) -> libc::c_int;
        /* * Encodes an Opus frame.
  * @param [in] st <tt>OpusEncoder*</tt>: Encoder state
  * @param [in] pcm <tt>opus_int16*</tt>: Input signal (interleaved if 2 channels). length is frame_size*channels*sizeof(opus_int16)
  * @param [in] frame_size <tt>int</tt>: Number of samples per channel in the
  *                                      input signal.
  *                                      This must be an Opus frame size for
  *                                      the encoder's sampling rate.
  *                                      For example, at 48 kHz the permitted
  *                                      values are 120, 240, 480, 960, 1920,
  *                                      and 2880.
  *                                      Passing in a duration of less than
  *                                      10 ms (480 samples at 48 kHz) will
  *                                      prevent the encoder from using the LPC
  *                                      or hybrid modes.
  * @param [out] data <tt>unsigned char*</tt>: Output payload.
  *                                            This must contain storage for at
  *                                            least \a max_data_bytes.
  * @param [in] max_data_bytes <tt>opus_int32</tt>: Size of the allocated
  *                                                 memory for the output
  *                                                 payload. This may be
  *                                                 used to impose an upper limit on
  *                                                 the instant bitrate, but should
  *                                                 not be used as the only bitrate
  *                                                 control. Use #OPUS_SET_BITRATE to
  *                                                 control the bitrate.
  * @returns The length of the encoded packet (in bytes) on success or a
  *          negative error code (see @ref opus_errorcodes) on failure.
  */
        #[no_mangle]
        pub fn opus_encode(st: *mut OpusEncoder, pcm: *const opus_int16,
                           frame_size: libc::c_int, data: *mut libc::c_uchar,
                           max_data_bytes: opus_int32) -> opus_int32;
    }
}
#[header_src = "/usr/include/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src =
      "ioq3/code/client/cl_main.c"]
pub mod cl_main_c {
    pub type serverStatus_t = serverStatus_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverStatus_s {
        pub string: [libc::c_char; 8192],
        pub address: netadr_t,
        pub time: libc::c_int,
        pub startTime: libc::c_int,
        pub pending: qboolean,
        pub print: qboolean,
        pub retrieved: qboolean,
    }
    use super::{libc};
    use super::qcommon_h::{netadr_t, msg_t};
    use super::q_shared_h::{qboolean, cvar_t};
    use super::client_h::{ping_t, serverInfo_t};
}
#[header_src =
      "ioq3/code/ui/ui_public.h"]
pub mod ui_public_h {
    pub const UIMENU_NONE: unnamed_3 = 0;
    //	qboolean UI_IsFullscreen( void );
    pub const UI_SET_ACTIVE_MENU: unnamed_4 = 7;
    pub const UIMENU_MAIN: unnamed_3 = 1;
    pub const UIMENU_NEED_CD: unnamed_3 = 3;
    pub type unnamed_3 = libc::c_uint;
    pub const UIMENU_POSTGAME: unnamed_3 = 6;
    pub const UIMENU_TEAM: unnamed_3 = 5;
    pub const UIMENU_BAD_CD_KEY: unnamed_3 = 4;
    pub const UIMENU_INGAME: unnamed_3 = 2;
    pub type unnamed_4 = libc::c_uint;
    //	void	UI_DrawConnectScreen( qboolean overlay );
    pub const UI_HASUNIQUECDKEY: unnamed_4 = 10;
    //	qboolean UI_ConsoleCommand( int realTime );
    pub const UI_DRAW_CONNECT_SCREEN: unnamed_4 = 9;
    //	void	UI_SetActiveMenu( uiMenuCommand_t menu );
    pub const UI_CONSOLE_COMMAND: unnamed_4 = 8;
    //	void	UI_Refresh( int time );
    pub const UI_IS_FULLSCREEN: unnamed_4 = 6;
    //	void	UI_MouseEvent( int dx, int dy );
    pub const UI_REFRESH: unnamed_4 = 5;
    //	void	UI_KeyEvent( int key );
    pub const UI_MOUSE_EVENT: unnamed_4 = 4;
    //	void	UI_Shutdown( void );
    pub const UI_KEY_EVENT: unnamed_4 = 3;
    //	void	UI_Init( void );
    pub const UI_SHUTDOWN: unnamed_4 = 2;
    pub const UI_INIT: unnamed_4 = 1;
    // system reserved
    pub const UI_GETAPIVERSION: unnamed_4 = 0;
    use super::{libc};
}
#[header_src = "/usr/include/opus/opus_types.h"]
pub mod opus_types_h {
    pub type opus_int32 = int32_t;
    pub type opus_int16 = int16_t;
    use super::stdint_intn_h::{int32_t, int16_t};
}
#[header_src =
      "ioq3/code/cgame/cg_public.h"]
pub mod cg_public_h {
    //	void (*CG_DrawActiveFrame)( int serverTime, stereoFrame_t stereoView, qboolean demoPlayback );
	// Generates and draws a game scene and status information at the given time.
	// If demoPlayback is set, local movement prediction will not be enabled
    pub const CG_CROSSHAIR_PLAYER: unnamed_5 = 4;
    //	int (*CG_CrosshairPlayer)( void );
    pub const CG_LAST_ATTACKER: unnamed_5 = 5;
    /*
==================================================================

functions exported to the main executable

==================================================================
*/
    pub type unnamed_5 = libc::c_uint;
    //	void	(*CG_MouseEvent)( int dx, int dy );
    pub const CG_EVENT_HANDLING: unnamed_5 = 8;
    //	void	(*CG_KeyEvent)( int key, qboolean down );
    pub const CG_MOUSE_EVENT: unnamed_5 = 7;
    //	int (*CG_LastAttacker)( void );
    pub const CG_KEY_EVENT: unnamed_5 = 6;
    //	qboolean (*CG_ConsoleCommand)( void );
	// a console command has been issued locally that is not recognized by the
	// main game system.
	// use Cmd_Argc() / Cmd_Argv() to read the command, return qfalse if the
	// command is not known to the game
    pub const CG_DRAW_ACTIVE_FRAME: unnamed_5 = 3;
    //	void (*CG_Shutdown)( void );
	// opportunity to flush and close any open files
    pub const CG_CONSOLE_COMMAND: unnamed_5 = 2;
    //	void CG_Init( int serverMessageNum, int serverCommandSequence, int clientNum )
	// called when the level loads or when the renderer is restarted
	// all media should be registered at this time
	// cgame will display loading status by calling SCR_Update, which
	// will call CG_DrawInformation during the loading process
	// reliableCommandSequence will be 0 on fresh loads, but higher for
	// demos, tourney restarts, or vid_restarts
    pub const CG_SHUTDOWN: unnamed_5 = 1;
    pub const CG_INIT: unnamed_5 = 0;
    use super::{libc};
}
#[header_src =
      "ioq3/code/game/bg_public.h"]
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
    pub type unnamed_6 = libc::c_uint;
    pub const GT_MAX_GAME_TYPE: unnamed_6 = 8;
    pub const GT_HARVESTER: unnamed_6 = 7;
    pub const GT_OBELISK: unnamed_6 = 6;
    pub const GT_1FCTF: unnamed_6 = 5;
    // capture the flag
    pub const GT_CTF: unnamed_6 = 4;
    //-- team games go after this --
    // team deathmatch
    pub const GT_TEAM: unnamed_6 = 3;
    // single player ffa
    pub const GT_SINGLE_PLAYER: unnamed_6 = 2;
    // one on one tournament
    pub const GT_TOURNAMENT: unnamed_6 = 1;
    // free for all
    pub const GT_FFA: unnamed_6 = 0;
    use super::{libc};
}
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::FILE_h::{FILE};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...)
         -> libc::c_int;
        #[no_mangle]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...)
         -> libc::c_int;
        #[no_mangle]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...)
         -> libc::c_int;
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
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
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
      "ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::q_shared_h::{byte};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CM_ClearMap();
        #[no_mangle]
        pub fn CM_ClusterPVS(cluster: libc::c_int) -> *mut byte;
        // cm_patch.c
        #[no_mangle]
        pub fn CM_DrawDebugSurface(drawPoly:
                                       Option<unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut libc::c_float)
                                                  -> ()>);
    }
}
#[header_src =
      "ioq3/code/sys/sys_local.h"]
pub mod sys_local_h {
    extern "C" {
        #[no_mangle]
        pub fn Sys_GLimpInit();
        #[no_mangle]
        pub fn Sys_GLimpSafeInit();
    }
}
#[header_src =
      "ioq3/code/client/cl_variadic.h"]
pub mod cl_variadic_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CL_RefPrintf(print_level: libc::c_int,
                            fmt: *const libc::c_char, ...);
    }
}
#[header_src = "/usr/include/SDL2/SDL_error.h"]
pub mod SDL_error_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn SDL_GetError() -> *const libc::c_char;
    }
}
#[header_src = "/usr/include/SDL2/SDL_loadso.h"]
pub mod SDL_loadso_h {
    use super::{libc};
    extern "C" {
        /* *
 *  Given an object handle, this function looks up the address of the
 *  named function in the shared object and returns it.  This address
 *  is no longer valid after calling SDL_UnloadObject().
 */
        #[no_mangle]
        pub fn SDL_LoadFunction(handle: *mut libc::c_void,
                                name: *const libc::c_char)
         -> *mut libc::c_void;
        /* *
 *  Unload a shared object from memory.
 */
        #[no_mangle]
        pub fn SDL_UnloadObject(handle: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/sys/sys_loadlib.h"]
pub mod sys_loadlib_h {
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
        pub fn Sys_LoadDll(name: *const libc::c_char, useSystemLib: qboolean)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    use super::{libc};
    use super::q_shared_h::{byte};
    extern "C" {
        #[no_mangle]
        pub fn S_MasterGain(gain: libc::c_float);
        #[no_mangle]
        pub fn S_StopCapture();
        #[no_mangle]
        pub fn S_AvailableCaptureSamples() -> libc::c_int;
        #[no_mangle]
        pub fn S_Capture(samples: libc::c_int, data: *mut byte);
        #[no_mangle]
        pub fn S_StartCapture();
        #[no_mangle]
        pub fn S_BeginRegistration();
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
        pub fn S_Init();
        // stop all sounds and the background track
        #[no_mangle]
        pub fn S_StopAllSounds();
        #[no_mangle]
        pub fn S_Shutdown();
        #[no_mangle]
        pub fn S_DisableSounds();
        #[no_mangle]
        pub fn S_Update();
    }
}
#[header_src =
      "ioq3/code/client/libmumblelink.h"]
pub mod libmumblelink_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn mumble_unlink();
        #[no_mangle]
        pub fn mumble_islinked() -> libc::c_int;
        #[no_mangle]
        pub fn mumble_update_coordinates(fPosition: *mut libc::c_float,
                                         fFront: *mut libc::c_float,
                                         fTop: *mut libc::c_float);
    }
}
#[header_src =
      "ioq3/code/client/keys.h"]
pub mod keys_h {
    use super::qcommon_h::{field_t};
    extern "C" {
        #[no_mangle]
        pub static mut g_consoleField: field_t;
    }
}
#[header_src =
      "ioq3/code/client/cl_curl.h"]
pub mod cl_curl_h {
    use super::q_shared_h::{cvar_t, qboolean};
    use super::{libc};
    extern "C" {
        /*
===========================================================================
Copyright (C) 2006 Tony J. White (tjw@tjw.org)

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
        pub static mut cl_cURLLib: *mut cvar_t;
        #[no_mangle]
        pub fn CL_cURL_Shutdown();
        #[no_mangle]
        pub fn CL_cURL_PerformDownload();
        #[no_mangle]
        pub fn CL_cURL_Init() -> qboolean;
        #[no_mangle]
        pub fn CL_cURL_BeginDownload(localName: *const libc::c_char,
                                     remoteURL: *const libc::c_char);
    }
}
use self::types_h::{__uint8_t, __int16_t, __int32_t, __off_t, __off64_t};
use self::stddef_h::{size_t};
use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt,
                          _IO_marker};
use self::FILE_h::{FILE};
use self::stdint_intn_h::{int16_t, int32_t};
use self::stdlib_h::{__compar_fn_t, atof, atoi, strtol, rand, qsort};
use self::ctype_h::{unnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank, _ISgraph,
                    _ISprint, _ISspace, _ISxdigit, _ISdigit, _ISalpha,
                    _ISlower, _ISupper, __ctype_b_loc};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed_0, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
                       unnamed_1, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, ha_pref,
                       h_dontcare, h_low, h_high, vec_t, vec3_t, cvar_s,
                       cvar_t, markFragment_t, orientation_t, gameState_t,
                       playerState_s, playerState_t, usercmd_s, usercmd_t,
                       trType_t, TR_GRAVITY, TR_SINE, TR_LINEAR_STOP,
                       TR_LINEAR, TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, connstate_t,
                       CA_CINEMATIC, CA_ACTIVE, CA_PRIMED, CA_LOADING,
                       CA_CONNECTED, CA_CHALLENGING, CA_CONNECTING,
                       CA_AUTHORIZING, CA_DISCONNECTED, CA_UNINITIALIZED,
                       glyphInfo_t, fontInfo_t, qtime_s, qtime_t, e_status,
                       FMV_ID_WAIT, FMV_LOOPED, FMV_ID_IDLE, FMV_ID_BLT,
                       FMV_EOF, FMV_PLAY, FMV_IDLE, Hunk_AllocDebug, qftolsse,
                       AngleVectors, Com_Clamp, COM_SkipPath,
                       COM_StripExtension, Com_sprintf, Com_SkipTokens,
                       Com_RandomBytes, Q_isanumber, Q_stricmp, Q_strncmp,
                       Q_stricmpn, Q_strncpyz, Q_strcat, Q_CleanStr, va,
                       Info_ValueForKey, Info_SetValueForKey, Com_Error,
                       Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      svc_ops_e, svc_voipOpus, svc_voipSpeex, svc_EOF,
                      svc_snapshot, svc_download, svc_serverCommand,
                      svc_baseline, svc_configstring, svc_gamestate, svc_nop,
                      svc_bad, vm_t, xcommand_t, completionFunc_t, field_t,
                      unnamed_2, TAG_STATIC, TAG_SMALL, TAG_RENDERER,
                      TAG_BOTLIB, TAG_GENERAL, TAG_FREE, vm_s, MSG_Init,
                      MSG_Bitstream, MSG_WriteByte, MSG_WriteShort,
                      MSG_WriteLong, MSG_WriteBigString, MSG_BeginReadingOOB,
                      MSG_ReadLong, MSG_ReadString, MSG_ReadStringLine,
                      MSG_WriteDeltaEntity, NET_SendPacket,
                      NET_OutOfBandPrint, NET_OutOfBandData, NET_CompareAdr,
                      NET_IsLocalAddress, NET_AdrToString,
                      NET_AdrToStringwPort, NET_StringToAdr, Netchan_Setup,
                      demo_protocols, VM_Call, Cbuf_AddText, Cbuf_ExecuteText,
                      Cbuf_Execute, Cmd_AddCommand, Cmd_RemoveCommand,
                      Cmd_SetCommandCompletionFunc, Cmd_Argc, Cmd_Argv,
                      Cmd_Args, Cmd_ArgsFrom, Cmd_Cmd, Cmd_TokenizeString,
                      Cvar_Get, Cvar_Set, Cvar_Set2, Cvar_SetValue,
                      Cvar_VariableValue, Cvar_VariableIntegerValue,
                      Cvar_VariableString, Cvar_VariableStringBuffer,
                      Cvar_ForceReset, Cvar_InfoString, Cvar_CheckRange,
                      Cvar_SetDescription, cvar_modifiedFlags,
                      FS_ConditionalRestart, FS_Restart, FS_ListFiles,
                      FS_FreeFileList, FS_FileExists, FS_BuildOSPath,
                      FS_CompareZipChecksum, FS_FOpenFileWrite,
                      FS_SV_FOpenFileWrite, FS_SV_FOpenFileRead,
                      FS_FOpenFileRead, FS_FileIsInPAK, FS_Write, FS_Read,
                      FS_FCloseFile, FS_ReadFile, FS_FreeFile, FS_WriteFile,
                      FS_Printf, FS_LoadedPakNames, FS_ReferencedPakNames,
                      FS_ReferencedPakPureChecksums, FS_ClearPakReferences,
                      FS_PureServerSetReferencedPaks,
                      FS_PureServerSetLoadedPaks, FS_ComparePaks,
                      Field_CompleteFilename, Field_CompleteCommand,
                      Field_CompletePlayerName, cl_cdkey, Com_EventLoop,
                      Info_Print, Com_DPrintf, Com_Milliseconds, Com_MD5File,
                      Com_RealTime, Com_FieldStringToPlayerName,
                      Com_strCompare, com_dedicated, com_timescale,
                      com_sv_running, com_cl_running, com_version,
                      com_standalone, cl_paused, sv_paused, com_gamename,
                      com_protocol, com_legacyprotocol, com_errorEntered,
                      com_fullyInitialized, Z_TagMallocDebug, Z_Free,
                      Hunk_Clear, Hunk_ClearToMark, Hunk_AllocateTempMemory,
                      Hunk_FreeTempMemory, Sys_LowPhysicalMemory, Sys_SetEnv,
                      IN_Restart, IN_Shutdown, IN_Init, Sys_Milliseconds,
                      Sys_ShowIP, S_ClearSoundBuffer, SV_Frame, SV_Shutdown,
                      Sys_IsLANAddress, SCR_DebugGraph};
use self::tr_public_h::{refexport_t, refimport_t, GetRefAPI_t};
use self::tr_types_h::{stereoFrame_t, STEREO_RIGHT, STEREO_LEFT,
                       STEREO_CENTER, refdef_t, polyVert_t, refEntity_t,
                       refEntityType_t, RT_MAX_REF_ENTITY_TYPE,
                       RT_PORTALSURFACE, RT_LIGHTNING, RT_RAIL_RINGS,
                       RT_RAIL_CORE, RT_BEAM, RT_SPRITE, RT_POLY, RT_MODEL,
                       glconfig_t, textureCompression_t, TC_S3TC_ARB, TC_S3TC,
                       TC_NONE, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glDriverType_t, GLDRV_VOODOO,
                       GLDRV_STANDALONE, GLDRV_ICD};
use self::client_h::{clientActive_t, clSnapshot_t, outPacket_t,
                     clientConnection_t, ping_t, serverInfo_t, clientStatic_t,
                     SCR_Init, CL_WriteAVIVideoFrame, CIN_RunCinematic,
                     CIN_PlayCinematic, CIN_UploadCinematic, CL_CloseAVI,
                     CL_OpenAVIForWriting, Key_SetCatcher, Con_Close,
                     SCR_UpdateScreen, CL_VideoRecording,
                     cl_connectedToPureServer, CL_WritePacket,
                     SCR_StopCinematic, uivm, CL_PlayCinematic_f,
                     CL_ParseServerMessage, CL_InitCGame, CL_InitUI,
                     g_console_field_width, CL_ShutdownCGame, CL_ShutdownUI,
                     cl_run, cl_anglespeedkey, cl_pitchspeed, cl_yawspeed,
                     CL_InitInput, Con_Init, Con_Shutdown, CL_ShutdownInput,
                     Con_RunConsole, SCR_RunCinematic, CL_SetCGameTime,
                     CL_SendCmd, cl_timegraph, CL_TakeVideoFrame,
                     Key_GetCatcher, CL_Netchan_Process};
use self::opus_h::{OpusEncoder, OpusDecoder, opus_decoder_destroy,
                   opus_encoder_destroy, opus_encoder_ctl, opus_encode};
use self::multi_h::{CURLM};
use self::curl_h::{CURL};
use self::cl_main_c::{serverStatus_t, serverStatus_s};
use self::ui_public_h::{UIMENU_NONE, UI_SET_ACTIVE_MENU, UIMENU_MAIN,
                        UIMENU_NEED_CD, unnamed_3, UIMENU_POSTGAME,
                        UIMENU_TEAM, UIMENU_BAD_CD_KEY, UIMENU_INGAME,
                        unnamed_4, UI_HASUNIQUECDKEY, UI_DRAW_CONNECT_SCREEN,
                        UI_CONSOLE_COMMAND, UI_IS_FULLSCREEN, UI_REFRESH,
                        UI_MOUSE_EVENT, UI_KEY_EVENT, UI_SHUTDOWN, UI_INIT,
                        UI_GETAPIVERSION};
use self::opus_types_h::{opus_int32, opus_int16};
use self::cg_public_h::{CG_CROSSHAIR_PLAYER, CG_LAST_ATTACKER, unnamed_5,
                        CG_EVENT_HANDLING, CG_MOUSE_EVENT, CG_KEY_EVENT,
                        CG_DRAW_ACTIVE_FRAME, CG_CONSOLE_COMMAND, CG_SHUTDOWN,
                        CG_INIT};
use self::bg_public_h::{unnamed_6, GT_MAX_GAME_TYPE, GT_HARVESTER, GT_OBELISK,
                        GT_1FCTF, GT_CTF, GT_TEAM, GT_SINGLE_PLAYER,
                        GT_TOURNAMENT, GT_FFA};
use self::mathcalls_h::{sqrt, fabs};
use self::stdio_h::{stderr, fprintf, sprintf, sscanf};
use self::string_h::{memcpy, memmove, memset, strcmp, strchr, strrchr, strstr,
                     strlen};
use self::q_platform_h::{ShortSwap};
use self::cm_public_h::{CM_ClearMap, CM_ClusterPVS, CM_DrawDebugSurface};
use self::sys_local_h::{Sys_GLimpInit, Sys_GLimpSafeInit};
use self::cl_variadic_h::{CL_RefPrintf};
use self::SDL_error_h::{SDL_GetError};
use self::SDL_loadso_h::{SDL_LoadFunction, SDL_UnloadObject};
use self::sys_loadlib_h::{Sys_LoadDll};
use self::snd_public_h::{S_MasterGain, S_StopCapture,
                         S_AvailableCaptureSamples, S_Capture, S_StartCapture,
                         S_BeginRegistration, S_Init, S_StopAllSounds,
                         S_Shutdown, S_DisableSounds, S_Update};
use self::libmumblelink_h::{mumble_unlink, mumble_islinked,
                            mumble_update_coordinates};
use self::keys_h::{g_consoleField};
use self::cl_curl_h::{cl_cURLLib, CL_cURL_Shutdown, CL_cURL_PerformDownload,
                      CL_cURL_Init, CL_cURL_BeginDownload};
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
#[no_mangle]
pub unsafe extern "C" fn CL_Init() {
    Com_Printf(b"----- Client Initialization -----\n\x00" as *const u8 as
                   *const libc::c_char);
    Con_Init();
    if 0 == com_fullyInitialized as u64 {
        CL_ClearState();
        clc.state = CA_DISCONNECTED;
        cl_oldGameSet = qfalse
    }
    cls.realtime = 0i32;
    CL_InitInput();
    cl_noprint =
        Cvar_Get(b"cl_noprint\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_motd =
        Cvar_Get(b"cl_motd\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_timeout =
        Cvar_Get(b"cl_timeout\x00" as *const u8 as *const libc::c_char,
                 b"200\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_timeNudge =
        Cvar_Get(b"cl_timeNudge\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    cl_shownet =
        Cvar_Get(b"cl_shownet\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    cl_showSend =
        Cvar_Get(b"cl_showSend\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    cl_showTimeDelta =
        Cvar_Get(b"cl_showTimeDelta\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    cl_freezeDemo =
        Cvar_Get(b"cl_freezeDemo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    rcon_client_password =
        Cvar_Get(b"rconPassword\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x100i32);
    cl_activeAction =
        Cvar_Get(b"activeAction\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x100i32);
    cl_timedemo =
        Cvar_Get(b"timedemo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_timedemoLog =
        Cvar_Get(b"cl_timedemoLog\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_autoRecordDemo =
        Cvar_Get(b"cl_autoRecordDemo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_aviFrameRate =
        Cvar_Get(b"cl_aviFrameRate\x00" as *const u8 as *const libc::c_char,
                 b"25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_aviMotionJpeg =
        Cvar_Get(b"cl_aviMotionJpeg\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_forceavidemo =
        Cvar_Get(b"cl_forceavidemo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    rconAddress =
        Cvar_Get(b"rconAddress\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_yawspeed =
        Cvar_Get(b"cl_yawspeed\x00" as *const u8 as *const libc::c_char,
                 b"140\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_pitchspeed =
        Cvar_Get(b"cl_pitchspeed\x00" as *const u8 as *const libc::c_char,
                 b"140\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_anglespeedkey =
        Cvar_Get(b"cl_anglespeedkey\x00" as *const u8 as *const libc::c_char,
                 b"1.5\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_maxpackets =
        Cvar_Get(b"cl_maxpackets\x00" as *const u8 as *const libc::c_char,
                 b"30\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_packetdup =
        Cvar_Get(b"cl_packetdup\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_run =
        Cvar_Get(b"cl_run\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_sensitivity =
        Cvar_Get(b"sensitivity\x00" as *const u8 as *const libc::c_char,
                 b"5\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_mouseAccel =
        Cvar_Get(b"cl_mouseAccel\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_freelook =
        Cvar_Get(b"cl_freelook\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_mouseAccelStyle =
        Cvar_Get(b"cl_mouseAccelStyle\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_mouseAccelOffset =
        Cvar_Get(b"cl_mouseAccelOffset\x00" as *const u8 as
                     *const libc::c_char,
                 b"5\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_CheckRange(cl_mouseAccelOffset, 0.001f32, 50000.0f32, qfalse);
    cl_showMouseRate =
        Cvar_Get(b"cl_showmouserate\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_allowDownload =
        Cvar_Get(b"cl_allowDownload\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_cURLLib =
        Cvar_Get(b"cl_cURLLib\x00" as *const u8 as *const libc::c_char,
                 b"libcurl.so.4\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x2000i32);
    cl_conXOffset =
        Cvar_Get(b"cl_conXOffset\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_inGameVideo =
        Cvar_Get(b"r_inGameVideo\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_serverStatusResendTime =
        Cvar_Get(b"cl_serverStatusResendTime\x00" as *const u8 as
                     *const libc::c_char,
                 b"750\x00" as *const u8 as *const libc::c_char, 0i32);
    Cvar_Get(b"cg_autoswitch\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    m_pitch =
        Cvar_Get(b"m_pitch\x00" as *const u8 as *const libc::c_char,
                 b"0.022\x00" as *const u8 as *const libc::c_char, 0x1i32);
    m_yaw =
        Cvar_Get(b"m_yaw\x00" as *const u8 as *const libc::c_char,
                 b"0.022\x00" as *const u8 as *const libc::c_char, 0x1i32);
    m_forward =
        Cvar_Get(b"m_forward\x00" as *const u8 as *const libc::c_char,
                 b"0.25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    m_side =
        Cvar_Get(b"m_side\x00" as *const u8 as *const libc::c_char,
                 b"0.25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    m_filter =
        Cvar_Get(b"m_filter\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_pitch =
        Cvar_Get(b"j_pitch\x00" as *const u8 as *const libc::c_char,
                 b"0.022\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_yaw =
        Cvar_Get(b"j_yaw\x00" as *const u8 as *const libc::c_char,
                 b"-0.022\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_forward =
        Cvar_Get(b"j_forward\x00" as *const u8 as *const libc::c_char,
                 b"-0.25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_side =
        Cvar_Get(b"j_side\x00" as *const u8 as *const libc::c_char,
                 b"0.25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_up =
        Cvar_Get(b"j_up\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_pitch_axis =
        Cvar_Get(b"j_pitch_axis\x00" as *const u8 as *const libc::c_char,
                 b"3\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_yaw_axis =
        Cvar_Get(b"j_yaw_axis\x00" as *const u8 as *const libc::c_char,
                 b"2\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_forward_axis =
        Cvar_Get(b"j_forward_axis\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_side_axis =
        Cvar_Get(b"j_side_axis\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    j_up_axis =
        Cvar_Get(b"j_up_axis\x00" as *const u8 as *const libc::c_char,
                 b"4\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_CheckRange(j_pitch_axis, 0i32 as libc::c_float,
                    (16i32 - 1i32) as libc::c_float, qtrue);
    Cvar_CheckRange(j_yaw_axis, 0i32 as libc::c_float,
                    (16i32 - 1i32) as libc::c_float, qtrue);
    Cvar_CheckRange(j_forward_axis, 0i32 as libc::c_float,
                    (16i32 - 1i32) as libc::c_float, qtrue);
    Cvar_CheckRange(j_side_axis, 0i32 as libc::c_float,
                    (16i32 - 1i32) as libc::c_float, qtrue);
    Cvar_CheckRange(j_up_axis, 0i32 as libc::c_float,
                    (16i32 - 1i32) as libc::c_float, qtrue);
    cl_motdString =
        Cvar_Get(b"cl_motdString\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x40i32);
    Cvar_Get(b"cl_maxPing\x00" as *const u8 as *const libc::c_char,
             b"800\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_lanForcePackets =
        Cvar_Get(b"cl_lanForcePackets\x00" as *const u8 as
                     *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_guidServerUniq =
        Cvar_Get(b"cl_guidServerUniq\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_consoleKeys =
        Cvar_Get(b"cl_consoleKeys\x00" as *const u8 as *const libc::c_char,
                 b"~ ` 0x7e 0x60\x00" as *const u8 as *const libc::c_char,
                 0x1i32);
    Cvar_Get(b"name\x00" as *const u8 as *const libc::c_char,
             b"UnnamedPlayer\x00" as *const u8 as *const libc::c_char,
             0x2i32 | 0x1i32);
    cl_rate =
        Cvar_Get(b"rate\x00" as *const u8 as *const libc::c_char,
                 b"25000\x00" as *const u8 as *const libc::c_char,
                 0x2i32 | 0x1i32);
    Cvar_Get(b"snaps\x00" as *const u8 as *const libc::c_char,
             b"20\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x1i32);
    Cvar_Get(b"model\x00" as *const u8 as *const libc::c_char,
             b"sarge\x00" as *const u8 as *const libc::c_char,
             0x2i32 | 0x1i32);
    Cvar_Get(b"headmodel\x00" as *const u8 as *const libc::c_char,
             b"sarge\x00" as *const u8 as *const libc::c_char,
             0x2i32 | 0x1i32);
    Cvar_Get(b"team_model\x00" as *const u8 as *const libc::c_char,
             b"james\x00" as *const u8 as *const libc::c_char,
             0x2i32 | 0x1i32);
    Cvar_Get(b"team_headmodel\x00" as *const u8 as *const libc::c_char,
             b"*james\x00" as *const u8 as *const libc::c_char,
             0x2i32 | 0x1i32);
    Cvar_Get(b"g_redTeam\x00" as *const u8 as *const libc::c_char,
             b"Stroggs\x00" as *const u8 as *const libc::c_char,
             0x4i32 | 0x1i32);
    Cvar_Get(b"g_blueTeam\x00" as *const u8 as *const libc::c_char,
             b"Pagans\x00" as *const u8 as *const libc::c_char,
             0x4i32 | 0x1i32);
    Cvar_Get(b"color1\x00" as *const u8 as *const libc::c_char,
             b"4\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x1i32);
    Cvar_Get(b"color2\x00" as *const u8 as *const libc::c_char,
             b"5\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x1i32);
    Cvar_Get(b"handicap\x00" as *const u8 as *const libc::c_char,
             b"100\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x1i32);
    Cvar_Get(b"teamtask\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x2i32);
    Cvar_Get(b"sex\x00" as *const u8 as *const libc::c_char,
             b"male\x00" as *const u8 as *const libc::c_char,
             0x2i32 | 0x1i32);
    Cvar_Get(b"cl_anonymous\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x1i32);
    Cvar_Get(b"password\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x2i32);
    Cvar_Get(b"cg_predictItems\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x1i32);
    cl_useMumble =
        Cvar_Get(b"cl_useMumble\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x20i32);
    cl_mumbleScale =
        Cvar_Get(b"cl_mumbleScale\x00" as *const u8 as *const libc::c_char,
                 b"0.0254\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_voipSend =
        Cvar_Get(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_voipSendTarget =
        Cvar_Get(b"cl_voipSendTarget\x00" as *const u8 as *const libc::c_char,
                 b"spatial\x00" as *const u8 as *const libc::c_char, 0i32);
    cl_voipGainDuringCapture =
        Cvar_Get(b"cl_voipGainDuringCapture\x00" as *const u8 as
                     *const libc::c_char,
                 b"0.2\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_voipCaptureMult =
        Cvar_Get(b"cl_voipCaptureMult\x00" as *const u8 as
                     *const libc::c_char,
                 b"2.0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_voipUseVAD =
        Cvar_Get(b"cl_voipUseVAD\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_voipVADThreshold =
        Cvar_Get(b"cl_voipVADThreshold\x00" as *const u8 as
                     *const libc::c_char,
                 b"0.25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_voipShowMeter =
        Cvar_Get(b"cl_voipShowMeter\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cl_voip =
        Cvar_Get(b"cl_voip\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_CheckRange(cl_voip, 0i32 as libc::c_float, 1i32 as libc::c_float,
                    qtrue);
    cl_voipProtocol =
        Cvar_Get(b"cl_voipProtocol\x00" as *const u8 as *const libc::c_char,
                 if 0 != (*cl_voip).integer {
                     b"opus\x00" as *const u8 as *const libc::c_char
                 } else { b"\x00" as *const u8 as *const libc::c_char },
                 0x2i32 | 0x40i32);
    Cvar_Get(b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
             b"100\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_Get(b"cg_stereoSeparation\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    Cmd_AddCommand(b"cmd\x00" as *const u8 as *const libc::c_char,
                   Some(CL_ForwardToServer_f));
    Cmd_AddCommand(b"configstrings\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Configstrings_f));
    Cmd_AddCommand(b"clientinfo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Clientinfo_f));
    Cmd_AddCommand(b"snd_restart\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Snd_Restart_f));
    Cmd_AddCommand(b"vid_restart\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Vid_Restart_f));
    Cmd_AddCommand(b"disconnect\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Disconnect_f));
    Cmd_AddCommand(b"record\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Record_f));
    Cmd_AddCommand(b"demo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_PlayDemo_f));
    Cmd_SetCommandCompletionFunc(b"demo\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(CL_CompleteDemoName));
    Cmd_AddCommand(b"cinematic\x00" as *const u8 as *const libc::c_char,
                   Some(CL_PlayCinematic_f));
    Cmd_AddCommand(b"stoprecord\x00" as *const u8 as *const libc::c_char,
                   Some(CL_StopRecord_f));
    Cmd_AddCommand(b"connect\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Connect_f));
    Cmd_AddCommand(b"reconnect\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Reconnect_f));
    Cmd_AddCommand(b"localservers\x00" as *const u8 as *const libc::c_char,
                   Some(CL_LocalServers_f));
    Cmd_AddCommand(b"globalservers\x00" as *const u8 as *const libc::c_char,
                   Some(CL_GlobalServers_f));
    Cmd_AddCommand(b"rcon\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Rcon_f));
    Cmd_SetCommandCompletionFunc(b"rcon\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(CL_CompleteRcon));
    Cmd_AddCommand(b"ping\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Ping_f));
    Cmd_AddCommand(b"serverstatus\x00" as *const u8 as *const libc::c_char,
                   Some(CL_ServerStatus_f));
    Cmd_AddCommand(b"showip\x00" as *const u8 as *const libc::c_char,
                   Some(CL_ShowIP_f));
    Cmd_AddCommand(b"fs_openedList\x00" as *const u8 as *const libc::c_char,
                   Some(CL_OpenedPK3List_f));
    Cmd_AddCommand(b"fs_referencedList\x00" as *const u8 as
                       *const libc::c_char, Some(CL_ReferencedPK3List_f));
    Cmd_AddCommand(b"model\x00" as *const u8 as *const libc::c_char,
                   Some(CL_SetModel_f));
    Cmd_AddCommand(b"video\x00" as *const u8 as *const libc::c_char,
                   Some(CL_Video_f));
    Cmd_AddCommand(b"stopvideo\x00" as *const u8 as *const libc::c_char,
                   Some(CL_StopVideo_f));
    if 0 == (*com_dedicated).integer {
        Cmd_AddCommand(b"sayto\x00" as *const u8 as *const libc::c_char,
                       Some(CL_Sayto_f));
        Cmd_SetCommandCompletionFunc(b"sayto\x00" as *const u8 as
                                         *const libc::c_char,
                                     Some(CL_CompletePlayerName));
    }
    CL_InitRef();
    SCR_Init();
    Cvar_Set(b"cl_running\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
    CL_GenerateQKey();
    Cvar_Get(b"cl_guid\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x2i32 | 0x40i32);
    CL_UpdateGUID(0 as *const libc::c_char, 0i32);
    Com_Printf(b"----- Client Initialization Complete -----\n\x00" as
                   *const u8 as *const libc::c_char);
}
/*
====================
CL_UpdateGUID

update cl_guid using QKEY_FILE and optional prefix
====================
*/
unsafe extern "C" fn CL_UpdateGUID(mut prefix: *const libc::c_char,
                                   mut prefix_len: libc::c_int) {
    let mut f: fileHandle_t = 0;
    let mut len: libc::c_int = 0;
    len =
        FS_SV_FOpenFileRead(b"qkey\x00" as *const u8 as *const libc::c_char,
                            &mut f) as libc::c_int;
    FS_FCloseFile(f);
    if len != 2048i32 {
        Cvar_Set(b"cl_guid\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
    } else {
        Cvar_Set(b"cl_guid\x00" as *const u8 as *const libc::c_char,
                 Com_MD5File(b"qkey\x00" as *const u8 as *const libc::c_char,
                             2048i32, prefix, prefix_len));
    };
}
/*
===============
CL_GenerateQKey

test to see if a valid QKEY_FILE exists.  If one does not, try to generate
it by filling it with 2048 bytes of random data.
===============
*/
unsafe extern "C" fn CL_GenerateQKey() {
    let mut len: libc::c_int = 0i32;
    let mut buff: [libc::c_uchar; 2048] = [0; 2048];
    let mut f: fileHandle_t = 0;
    len =
        FS_SV_FOpenFileRead(b"qkey\x00" as *const u8 as *const libc::c_char,
                            &mut f) as libc::c_int;
    FS_FCloseFile(f);
    if len == 2048i32 {
        Com_Printf(b"QKEY found.\n\x00" as *const u8 as *const libc::c_char);
        return
    } else {
        if len > 0i32 {
            Com_Printf(b"QKEY file size != %d, regenerating\n\x00" as
                           *const u8 as *const libc::c_char, 2048i32);
        }
        Com_Printf(b"QKEY building random string\n\x00" as *const u8 as
                       *const libc::c_char);
        Com_RandomBytes(buff.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_uchar; 2048]>() as
                            libc::c_ulong as libc::c_int);
        f =
            FS_SV_FOpenFileWrite(b"qkey\x00" as *const u8 as
                                     *const libc::c_char);
        if 0 == f {
            Com_Printf(b"QKEY could not open %s for write\n\x00" as *const u8
                           as *const libc::c_char,
                       b"qkey\x00" as *const u8 as *const libc::c_char);
            return
        }
        FS_Write(buff.as_mut_ptr() as *const libc::c_void,
                 ::std::mem::size_of::<[libc::c_uchar; 2048]>() as
                     libc::c_ulong as libc::c_int, f);
        FS_FCloseFile(f);
        Com_Printf(b"QKEY generated\n\x00" as *const u8 as
                       *const libc::c_char);
    };
}
// shutdown client
#[no_mangle]
pub unsafe extern "C" fn CL_InitRef() {
    let mut ri: refimport_t =
        refimport_t{Printf: None,
                    Error: None,
                    Milliseconds: None,
                    Hunk_AllocDebug: None,
                    Hunk_AllocateTempMemory: None,
                    Hunk_FreeTempMemory: None,
                    Malloc: None,
                    Free: None,
                    Cvar_Get: None,
                    Cvar_Set: None,
                    Cvar_SetValue: None,
                    Cvar_CheckRange: None,
                    Cvar_SetDescription: None,
                    Cvar_VariableIntegerValue: None,
                    Cmd_AddCommand: None,
                    Cmd_RemoveCommand: None,
                    Cmd_Argc: None,
                    Cmd_Argv: None,
                    Cmd_ExecuteText: None,
                    CM_ClusterPVS: None,
                    CM_DrawDebugSurface: None,
                    FS_FileIsInPAK: None,
                    FS_ReadFile: None,
                    FS_FreeFile: None,
                    FS_ListFiles: None,
                    FS_FreeFileList: None,
                    FS_WriteFile: None,
                    FS_FileExists: None,
                    CIN_UploadCinematic: None,
                    CIN_PlayCinematic: None,
                    CIN_RunCinematic: None,
                    CL_WriteAVIVideoFrame: None,
                    IN_Init: None,
                    IN_Shutdown: None,
                    IN_Restart: None,
                    ftol: None,
                    Sys_SetEnv: None,
                    Sys_GLimpSafeInit: None,
                    Sys_GLimpInit: None,
                    Sys_LowPhysicalMemory: None,};
    let mut ret: *mut refexport_t = 0 as *mut refexport_t;
    let mut GetRefAPI: GetRefAPI_t = None;
    let mut dllName: [libc::c_char; 4096] = [0; 4096];
    Com_Printf(b"----- Initializing Renderer ----\n\x00" as *const u8 as
                   *const libc::c_char);
    cl_renderer =
        Cvar_Get(b"cl_renderer\x00" as *const u8 as *const libc::c_char,
                 b"opengl2\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x20i32);
    Com_sprintf(dllName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"renderer_%s_x86_64.so\x00" as *const u8 as
                    *const libc::c_char, (*cl_renderer).string);
    rendererLib = Sys_LoadDll(dllName.as_mut_ptr(), qfalse);
    if rendererLib.is_null() &&
           0 != strcmp((*cl_renderer).string, (*cl_renderer).resetString) {
        Com_Printf(b"failed:\n\"%s\"\n\x00" as *const u8 as
                       *const libc::c_char, SDL_GetError());
        Cvar_ForceReset(b"cl_renderer\x00" as *const u8 as
                            *const libc::c_char);
        Com_sprintf(dllName.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as
                        libc::c_ulong as libc::c_int,
                    b"renderer_opengl2_x86_64.so\x00" as *const u8 as
                        *const libc::c_char);
        rendererLib = Sys_LoadDll(dllName.as_mut_ptr(), qfalse)
    }
    if rendererLib.is_null() {
        Com_Printf(b"failed:\n\"%s\"\n\x00" as *const u8 as
                       *const libc::c_char, SDL_GetError());
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Failed to load renderer\x00" as *const u8 as
                      *const libc::c_char);
    }
    GetRefAPI =
        ::std::mem::transmute::<*mut libc::c_void,
                                GetRefAPI_t>(SDL_LoadFunction(rendererLib,
                                                              b"GetRefAPI\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if GetRefAPI.is_none() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Can\'t load symbol GetRefAPI: \'%s\'\x00" as *const u8 as
                      *const libc::c_char, SDL_GetError());
    }
    ri.Cmd_AddCommand = Some(Cmd_AddCommand);
    ri.Cmd_RemoveCommand = Some(Cmd_RemoveCommand);
    ri.Cmd_Argc = Some(Cmd_Argc);
    ri.Cmd_Argv = Some(Cmd_Argv);
    ri.Cmd_ExecuteText = Some(Cbuf_ExecuteText);
    ri.Printf = Some(CL_RefPrintf);
    ri.Error = Some(Com_Error);
    ri.Milliseconds = Some(CL_ScaledMilliseconds);
    ri.Malloc = Some(CL_RefMalloc);
    ri.Free = Some(Z_Free);
    ri.Hunk_AllocDebug = Some(Hunk_AllocDebug);
    ri.Hunk_AllocateTempMemory = Some(Hunk_AllocateTempMemory);
    ri.Hunk_FreeTempMemory = Some(Hunk_FreeTempMemory);
    ri.CM_ClusterPVS = Some(CM_ClusterPVS);
    ri.CM_DrawDebugSurface = Some(CM_DrawDebugSurface);
    ri.FS_ReadFile = Some(FS_ReadFile);
    ri.FS_FreeFile = Some(FS_FreeFile);
    ri.FS_WriteFile = Some(FS_WriteFile);
    ri.FS_FreeFileList = Some(FS_FreeFileList);
    ri.FS_ListFiles = Some(FS_ListFiles);
    ri.FS_FileIsInPAK = Some(FS_FileIsInPAK);
    ri.FS_FileExists = Some(FS_FileExists);
    ri.Cvar_Get = Some(Cvar_Get);
    ri.Cvar_Set = Some(Cvar_Set);
    ri.Cvar_SetValue = Some(Cvar_SetValue);
    ri.Cvar_CheckRange = Some(Cvar_CheckRange);
    ri.Cvar_SetDescription = Some(Cvar_SetDescription);
    ri.Cvar_VariableIntegerValue = Some(Cvar_VariableIntegerValue);
    ri.CIN_UploadCinematic = Some(CIN_UploadCinematic);
    ri.CIN_PlayCinematic = Some(CIN_PlayCinematic);
    ri.CIN_RunCinematic = Some(CIN_RunCinematic);
    ri.CL_WriteAVIVideoFrame = Some(CL_WriteAVIVideoFrame);
    ri.IN_Init = Some(IN_Init);
    ri.IN_Shutdown = Some(IN_Shutdown);
    ri.IN_Restart = Some(IN_Restart);
    ri.ftol = Some(qftolsse);
    ri.Sys_SetEnv = Some(Sys_SetEnv);
    ri.Sys_GLimpSafeInit = Some(Sys_GLimpSafeInit);
    ri.Sys_GLimpInit = Some(Sys_GLimpInit);
    ri.Sys_LowPhysicalMemory = Some(Sys_LowPhysicalMemory);
    ret = GetRefAPI.expect("non-null function pointer")(8i32, &mut ri);
    Com_Printf(b"-------------------------------\n\x00" as *const u8 as
                   *const libc::c_char);
    if ret.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Couldn\'t initialize refresh\x00" as *const u8 as
                      *const libc::c_char);
    }
    re = *ret;
    Cvar_Set(b"cl_paused\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
}
// interface to refresh .dll
#[no_mangle]
pub static mut re: refexport_t =
    refexport_t{Shutdown: None,
                BeginRegistration: None,
                RegisterModel: None,
                RegisterSkin: None,
                RegisterShader: None,
                RegisterShaderNoMip: None,
                LoadWorld: None,
                SetWorldVisData: None,
                EndRegistration: None,
                ClearScene: None,
                AddRefEntityToScene: None,
                AddPolyToScene: None,
                LightForPoint: None,
                AddLightToScene: None,
                AddAdditiveLightToScene: None,
                RenderScene: None,
                SetColor: None,
                DrawStretchPic: None,
                DrawStretchRaw: None,
                UploadCinematic: None,
                BeginFrame: None,
                EndFrame: None,
                MarkFragments: None,
                LerpTag: None,
                ModelBounds: None,
                RegisterFont: None,
                RemapShader: None,
                GetEntityToken: None,
                inPVS: None,
                TakeVideoFrame: None,};
/*
============
CL_RefMalloc
============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RefMalloc(mut size: libc::c_int)
 -> *mut libc::c_void {
    return Z_TagMallocDebug(size, TAG_RENDERER as libc::c_int,
                            b"size\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                            b"code/client/cl_main.c\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            3164i32);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ScaledMilliseconds() -> libc::c_int {
    return (Sys_Milliseconds() as libc::c_float * (*com_timescale).value) as
               libc::c_int;
}
// Structure containing functions exported from refresh DLL
static mut rendererLib: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut cl_renderer: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
==================
CL_CompletePlayerName
==================
*/
unsafe extern "C" fn CL_CompletePlayerName(mut args: *mut libc::c_char,
                                           mut argNum: libc::c_int) {
    if argNum == 2i32 {
        let mut names: [[libc::c_char; 32]; 64] = [[0; 32]; 64];
        let mut namesPtr: [*const libc::c_char; 64] =
            [0 as *const libc::c_char; 64];
        let mut i: libc::c_int = 0;
        let mut clientCount: libc::c_int = 0;
        let mut nameCount: libc::c_int = 0;
        let mut info: *const libc::c_char = 0 as *const libc::c_char;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        info =
            cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[0usize]
                                                            as isize);
        clientCount =
            atoi(Info_ValueForKey(info,
                                  b"sv_maxclients\x00" as *const u8 as
                                      *const libc::c_char));
        nameCount = 0i32;
        i = 0i32;
        while i < clientCount {
            if !(i == clc.clientNum) {
                info =
                    cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[(32i32
                                                                                                +
                                                                                                256i32
                                                                                                +
                                                                                                256i32
                                                                                                +
                                                                                                i)
                                                                                               as
                                                                                               usize]
                                                                    as isize);
                name =
                    Info_ValueForKey(info,
                                     b"n\x00" as *const u8 as
                                         *const libc::c_char);
                if !(*name.offset(0isize) as libc::c_int == '\u{0}' as i32) {
                    Q_strncpyz(names[nameCount as usize].as_mut_ptr(), name,
                               ::std::mem::size_of::<[libc::c_char; 32]>() as
                                   libc::c_ulong as libc::c_int);
                    Q_CleanStr(names[nameCount as usize].as_mut_ptr());
                    namesPtr[nameCount as usize] =
                        names[nameCount as usize].as_mut_ptr();
                    nameCount += 1
                }
            }
            i += 1
        }
        qsort(namesPtr.as_mut_ptr() as *mut libc::c_void, nameCount as size_t,
              ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
              Some(Com_strCompare));
        Field_CompletePlayerName(namesPtr.as_mut_ptr(), nameCount);
    };
}
#[no_mangle]
pub static mut cl: clientActive_t =
    clientActive_t{timeoutcount: 0,
                   snap:
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
                                                      entityEventSequence:
                                                          0,},
                                    numEntities: 0,
                                    parseEntitiesNum: 0,
                                    serverCommandNum: 0,},
                   serverTime: 0,
                   oldServerTime: 0,
                   oldFrameServerTime: 0,
                   serverTimeDelta: 0,
                   extrapolatedSnapshot: qfalse,
                   newSnapshots: qfalse,
                   gameState:
                       gameState_t{stringOffsets: [0; 1024],
                                   stringData: [0; 16000],
                                   dataCount: 0,},
                   mapname: [0; 64],
                   parseEntitiesNum: 0,
                   mouseDx: [0; 2],
                   mouseDy: [0; 2],
                   mouseIndex: 0,
                   joystickAxis: [0; 16],
                   cgameUserCmdValue: 0,
                   cgameSensitivity: 0.,
                   cmds:
                       [usercmd_s{serverTime: 0,
                                  angles: [0; 3],
                                  buttons: 0,
                                  weapon: 0,
                                  forwardmove: 0,
                                  rightmove: 0,
                                  upmove: 0,}; 64],
                   cmdNumber: 0,
                   outPackets:
                       [outPacket_t{p_cmdNumber: 0,
                                    p_serverTime: 0,
                                    p_realtime: 0,}; 32],
                   viewangles: [0.; 3],
                   serverId: 0,
                   snapshots:
                       [clSnapshot_t{valid: qfalse,
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
                                                       entityEventSequence:
                                                           0,},
                                     numEntities: 0,
                                     parseEntitiesNum: 0,
                                     serverCommandNum: 0,}; 32],
                   entityBaselines:
                       [entityState_s{number: 0,
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
                                      generic1: 0,}; 1024],
                   parseEntities:
                       [entityState_s{number: 0,
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
                                      generic1: 0,}; 8192],};
#[no_mangle]
pub static mut clc: clientConnection_t =
    clientConnection_t{state: CA_UNINITIALIZED,
                       clientNum: 0,
                       lastPacketSentTime: 0,
                       lastPacketTime: 0,
                       servername: [0; 4096],
                       serverAddress:
                           netadr_t{type_0: NA_BAD,
                                    ip: [0; 4],
                                    ip6: [0; 16],
                                    port: 0,
                                    scope_id: 0,},
                       connectTime: 0,
                       connectPacketCount: 0,
                       serverMessage: [0; 1024],
                       challenge: 0,
                       checksumFeed: 0,
                       reliableSequence: 0,
                       reliableAcknowledge: 0,
                       reliableCommands: [[0; 1024]; 64],
                       serverMessageSequence: 0,
                       serverCommandSequence: 0,
                       lastExecutedServerCommand: 0,
                       serverCommands: [[0; 1024]; 64],
                       download: 0,
                       downloadTempName: [0; 4096],
                       downloadName: [0; 4096],
                       cURLEnabled: qfalse,
                       cURLUsed: qfalse,
                       cURLDisconnected: qfalse,
                       downloadURL: [0; 4096],
                       downloadCURL:
                           0 as *const libc::c_void as *mut libc::c_void,
                       downloadCURLM:
                           0 as *const libc::c_void as *mut libc::c_void,
                       sv_allowDownload: 0,
                       sv_dlURL: [0; 256],
                       downloadNumber: 0,
                       downloadBlock: 0,
                       downloadCount: 0,
                       downloadSize: 0,
                       downloadList: [0; 1024],
                       downloadRestart: qfalse,
                       demoName: [0; 64],
                       spDemoRecording: qfalse,
                       demorecording: qfalse,
                       demoplaying: qfalse,
                       demowaiting: qfalse,
                       firstDemoFrameSkipped: qfalse,
                       demofile: 0,
                       timeDemoFrames: 0,
                       timeDemoStart: 0,
                       timeDemoBaseTime: 0,
                       timeDemoLastFrame: 0,
                       timeDemoMinDuration: 0,
                       timeDemoMaxDuration: 0,
                       timeDemoDurations: [0; 4096],
                       aviVideoFrameRemainder: 0.,
                       aviSoundFrameRemainder: 0.,
                       voipEnabled: qfalse,
                       voipCodecInitialized: qfalse,
                       opusDecoder:
                           [0 as *const OpusDecoder as *mut OpusDecoder; 64],
                       voipIncomingGeneration: [0; 64],
                       voipIncomingSequence: [0; 64],
                       voipGain: [0.; 64],
                       voipIgnore: [qfalse; 64],
                       voipMuteAll: qfalse,
                       voipTargets: [0; 8],
                       voipFlags: 0,
                       opusEncoder:
                           0 as *const OpusEncoder as *mut OpusEncoder,
                       voipOutgoingDataSize: 0,
                       voipOutgoingDataFrames: 0,
                       voipOutgoingSequence: 0,
                       voipOutgoingGeneration: 0,
                       voipOutgoingData: [0; 1024],
                       voipPower: 0.,
                       compat: qfalse,
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
                                     compat: qfalse,},};
#[no_mangle]
pub unsafe extern "C" fn CL_Sayto_f() {
    let mut rawname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut cleanName: [libc::c_char; 32] = [0; 32];
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut clientNum: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() < 3i32 {
        Com_Printf(b"sayto <player name> <text>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    rawname = Cmd_Argv(1i32);
    Com_FieldStringToPlayerName(name.as_mut_ptr(), 32i32, rawname);
    info =
        cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[0usize]
                                                        as isize);
    count =
        atoi(Info_ValueForKey(info,
                              b"sv_maxclients\x00" as *const u8 as
                                  *const libc::c_char));
    clientNum = -1i32;
    i = 0i32;
    while i < count {
        info =
            cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[(32i32
                                                                                        +
                                                                                        256i32
                                                                                        +
                                                                                        256i32
                                                                                        +
                                                                                        i)
                                                                                       as
                                                                                       usize]
                                                            as isize);
        Q_strncpyz(cleanName.as_mut_ptr(),
                   Info_ValueForKey(info,
                                    b"n\x00" as *const u8 as
                                        *const libc::c_char),
                   ::std::mem::size_of::<[libc::c_char; 32]>() as
                       libc::c_ulong as libc::c_int);
        Q_CleanStr(cleanName.as_mut_ptr());
        if 0 == Q_stricmp(cleanName.as_mut_ptr(), name.as_mut_ptr()) {
            clientNum = i;
            break ;
        } else { i += 1 }
    }
    if clientNum <= -1i32 {
        Com_Printf(b"No such player name: %s.\n\x00" as *const u8 as
                       *const libc::c_char, name.as_mut_ptr());
        return
    }
    p = Cmd_ArgsFrom(2i32);
    if *p as libc::c_int == '\"' as i32 {
        p = p.offset(1isize);
        *p.offset(strlen(p).wrapping_sub(1i32 as libc::c_ulong) as isize) =
            0i32 as libc::c_char
    }
    CL_AddReliableCommand(va(b"tell %i \"%s\"\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             clientNum, p), qfalse);
}
// 20ms at 48k
// 3 frame is 60ms of audio, the max opus will encode at once
//=================================================
//
// cl_main
//
#[no_mangle]
pub unsafe extern "C" fn CL_AddReliableCommand(mut cmd: *const libc::c_char,
                                               mut isDisconnectCmd:
                                                   qboolean) {
    let mut unacknowledged: libc::c_int =
        clc.reliableSequence - clc.reliableAcknowledge;
    if 0 != isDisconnectCmd as libc::c_uint && unacknowledged > 64i32 ||
           0 == isDisconnectCmd as u64 && unacknowledged >= 64i32 {
        if 0 != com_errorEntered as u64 {
            return
        } else {
            Com_Error(ERR_DROP as libc::c_int,
                      b"Client command overflow\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    clc.reliableSequence += 1;
    Q_strncpyz(clc.reliableCommands[(clc.reliableSequence & 64i32 - 1i32) as
                                        usize].as_mut_ptr(), cmd,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
}
/*
===============
CL_StopVideo_f
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_StopVideo_f() { CL_CloseAVI(); }
//===========================================================================================
/*
===============
CL_Video_f

video
video [filename]
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Video_f() {
    let mut filename: [libc::c_char; 4096] = [0; 4096];
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    if 0 == clc.demoplaying as u64 {
        Com_Printf(b"The video command can only be used when playing back demos\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    if Cmd_Argc() == 2i32 {
        Com_sprintf(filename.as_mut_ptr(), 4096i32,
                    b"videos/%s.avi\x00" as *const u8 as *const libc::c_char,
                    Cmd_Argv(1i32));
    } else {
        i = 0i32;
        while i <= 9999i32 {
            let mut a: libc::c_int = 0;
            let mut b: libc::c_int = 0;
            let mut c: libc::c_int = 0;
            let mut d: libc::c_int = 0;
            last = i;
            a = last / 1000i32;
            last -= a * 1000i32;
            b = last / 100i32;
            last -= b * 100i32;
            c = last / 10i32;
            last -= c * 10i32;
            d = last;
            Com_sprintf(filename.as_mut_ptr(), 4096i32,
                        b"videos/video%d%d%d%d.avi\x00" as *const u8 as
                            *const libc::c_char, a, b, c, d);
            if 0 == FS_FileExists(filename.as_mut_ptr()) as u64 {
                // file doesn't exist
                break ;
            } else { i += 1 }
        }
        if i > 9999i32 {
            Com_Printf(b"^1ERROR: no free file names to create video\n\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
    }
    CL_OpenAVIForWriting(filename.as_mut_ptr());
}
//===========================================================================================
#[no_mangle]
pub unsafe extern "C" fn CL_SetModel_f() {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 256] = [0; 256];
    arg = Cmd_Argv(1i32);
    if 0 != *arg.offset(0isize) {
        Cvar_Set(b"model\x00" as *const u8 as *const libc::c_char, arg);
        Cvar_Set(b"headmodel\x00" as *const u8 as *const libc::c_char, arg);
    } else {
        Cvar_VariableStringBuffer(b"model\x00" as *const u8 as
                                      *const libc::c_char, name.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 256]>()
                                      as libc::c_ulong as libc::c_int);
        Com_Printf(b"model is set to %s\n\x00" as *const u8 as
                       *const libc::c_char, name.as_mut_ptr());
    };
}
/*
==================
CL_PureList_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReferencedPK3List_f() {
    Com_Printf(b"Referenced PK3 Names: %s\n\x00" as *const u8 as
                   *const libc::c_char, FS_ReferencedPakNames());
}
/*
==================
CL_PK3List_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_OpenedPK3List_f() {
    Com_Printf(b"Opened PK3 Names: %s\n\x00" as *const u8 as
                   *const libc::c_char, FS_LoadedPakNames());
}
#[no_mangle]
pub unsafe extern "C" fn CL_ShowIP_f() { Sys_ShowIP(); }
#[no_mangle]
pub unsafe extern "C" fn CL_ServerStatus_f() {
    let mut to: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut toptr: *mut netadr_t = 0 as *mut netadr_t;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut serverStatus: *mut serverStatus_t = 0 as *mut serverStatus_t;
    let mut argc: libc::c_int = 0;
    let mut family: netadrtype_t = NA_UNSPEC;
    argc = Cmd_Argc();
    if argc != 2i32 && argc != 3i32 {
        if clc.state as libc::c_uint !=
               CA_ACTIVE as libc::c_int as libc::c_uint ||
               0 != clc.demoplaying as libc::c_uint {
            Com_Printf(b"Not connected to a server.\n\x00" as *const u8 as
                           *const libc::c_char);
            Com_Printf(b"usage: serverstatus [-4|-6] server\n\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
        toptr = &mut clc.serverAddress
    }
    if toptr.is_null() {
        memset(&mut to as *mut netadr_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
        if argc == 2i32 {
            server = Cmd_Argv(1i32)
        } else {
            if 0 ==
                   strcmp(Cmd_Argv(1i32),
                          b"-4\x00" as *const u8 as *const libc::c_char) {
                family = NA_IP
            } else if 0 ==
                          strcmp(Cmd_Argv(1i32),
                                 b"-6\x00" as *const u8 as
                                     *const libc::c_char) {
                family = NA_IP6
            } else {
                Com_Printf(b"warning: only -4 or -6 as address type understood.\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            server = Cmd_Argv(2i32)
        }
        toptr = &mut to;
        if 0 == NET_StringToAdr(server, toptr, family) { return }
    }
    NET_OutOfBandPrint(NS_CLIENT, *toptr,
                       b"getstatus\x00" as *const u8 as *const libc::c_char);
    serverStatus = CL_GetServerStatus(*toptr);
    (*serverStatus).address = *toptr;
    (*serverStatus).print = qtrue;
    (*serverStatus).pending = qtrue;
}
/*
===================
CL_GetServerStatus
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetServerStatus(mut from: netadr_t)
 -> *mut serverStatus_t {
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0;
    let mut oldestTime: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        if 0 !=
               NET_CompareAdr(from, cl_serverStatusList[i as usize].address)
                   as u64 {
            return &mut *cl_serverStatusList.as_mut_ptr().offset(i as isize)
                       as *mut serverStatus_t
        }
        i += 1
    }
    i = 0i32;
    while i < 16i32 {
        if 0 != cl_serverStatusList[i as usize].retrieved as u64 {
            return &mut *cl_serverStatusList.as_mut_ptr().offset(i as isize)
                       as *mut serverStatus_t
        }
        i += 1
    }
    oldest = -1i32;
    oldestTime = 0i32;
    i = 0i32;
    while i < 16i32 {
        if oldest == -1i32 ||
               cl_serverStatusList[i as usize].startTime < oldestTime {
            oldest = i;
            oldestTime = cl_serverStatusList[i as usize].startTime
        }
        i += 1
    }
    return &mut *cl_serverStatusList.as_mut_ptr().offset(oldest as isize) as
               *mut serverStatus_t;
}
#[no_mangle]
pub static mut cl_serverStatusList: [serverStatus_t; 16] =
    [serverStatus_s{string: [0; 8192],
                    address:
                        netadr_t{type_0: NA_BAD,
                                 ip: [0; 4],
                                 ip6: [0; 16],
                                 port: 0,
                                 scope_id: 0,},
                    time: 0,
                    startTime: 0,
                    pending: qfalse,
                    print: qfalse,
                    retrieved: qfalse,}; 16];
#[no_mangle]
pub unsafe extern "C" fn CL_Ping_f() {
    let mut to: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut pingptr: *mut ping_t = 0 as *mut ping_t;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut family: netadrtype_t = NA_UNSPEC;
    argc = Cmd_Argc();
    if argc != 2i32 && argc != 3i32 {
        Com_Printf(b"usage: ping [-4|-6] server\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if argc == 2i32 {
        server = Cmd_Argv(1i32)
    } else {
        if 0 ==
               strcmp(Cmd_Argv(1i32),
                      b"-4\x00" as *const u8 as *const libc::c_char) {
            family = NA_IP
        } else if 0 ==
                      strcmp(Cmd_Argv(1i32),
                             b"-6\x00" as *const u8 as *const libc::c_char) {
            family = NA_IP6
        } else {
            Com_Printf(b"warning: only -4 or -6 as address type understood.\n\x00"
                           as *const u8 as *const libc::c_char);
        }
        server = Cmd_Argv(2i32)
    }
    memset(&mut to as *mut netadr_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    if 0 == NET_StringToAdr(server, &mut to, family) { return }
    pingptr = CL_GetFreePing();
    memcpy(&mut (*pingptr).adr as *mut netadr_t as *mut libc::c_void,
           &mut to as *mut netadr_t as *const libc::c_void,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    (*pingptr).start = Sys_Milliseconds();
    (*pingptr).time = 0i32;
    CL_SetServerInfoByAddress((*pingptr).adr, 0 as *const libc::c_char, 0i32);
    NET_OutOfBandPrint(NS_CLIENT, to,
                       b"getinfo xxx\x00" as *const u8 as
                           *const libc::c_char);
}
unsafe extern "C" fn CL_SetServerInfoByAddress(mut from: netadr_t,
                                               mut info: *const libc::c_char,
                                               mut ping: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 128i32 {
        if 0 != NET_CompareAdr(from, cls.localServers[i as usize].adr) as u64
           {
            CL_SetServerInfo(&mut *cls.localServers.as_mut_ptr().offset(i as
                                                                            isize),
                             info, ping);
        }
        i += 1
    }
    i = 0i32;
    while i < 4096i32 {
        if 0 != NET_CompareAdr(from, cls.globalServers[i as usize].adr) as u64
           {
            CL_SetServerInfo(&mut *cls.globalServers.as_mut_ptr().offset(i as
                                                                             isize),
                             info, ping);
        }
        i += 1
    }
    i = 0i32;
    while i < 128i32 {
        if 0 !=
               NET_CompareAdr(from, cls.favoriteServers[i as usize].adr) as
                   u64 {
            CL_SetServerInfo(&mut *cls.favoriteServers.as_mut_ptr().offset(i
                                                                               as
                                                                               isize),
                             info, ping);
        }
        i += 1
    };
}
#[no_mangle]
pub static mut cls: clientStatic_t =
    clientStatic_t{cddialog: qfalse,
                   rendererStarted: qfalse,
                   soundStarted: qfalse,
                   soundRegistered: qfalse,
                   uiStarted: qfalse,
                   cgameStarted: qfalse,
                   framecount: 0,
                   frametime: 0,
                   realtime: 0,
                   realFrametime: 0,
                   numlocalservers: 0,
                   localServers:
                       [serverInfo_t{adr:
                                         netadr_t{type_0: NA_BAD,
                                                  ip: [0; 4],
                                                  ip6: [0; 16],
                                                  port: 0,
                                                  scope_id: 0,},
                                     hostName: [0; 32],
                                     mapName: [0; 32],
                                     game: [0; 32],
                                     netType: 0,
                                     gameType: 0,
                                     clients: 0,
                                     maxClients: 0,
                                     minPing: 0,
                                     maxPing: 0,
                                     ping: 0,
                                     visible: qfalse,
                                     punkbuster: 0,
                                     g_humanplayers: 0,
                                     g_needpass: 0,}; 128],
                   numglobalservers: 0,
                   globalServers:
                       [serverInfo_t{adr:
                                         netadr_t{type_0: NA_BAD,
                                                  ip: [0; 4],
                                                  ip6: [0; 16],
                                                  port: 0,
                                                  scope_id: 0,},
                                     hostName: [0; 32],
                                     mapName: [0; 32],
                                     game: [0; 32],
                                     netType: 0,
                                     gameType: 0,
                                     clients: 0,
                                     maxClients: 0,
                                     minPing: 0,
                                     maxPing: 0,
                                     ping: 0,
                                     visible: qfalse,
                                     punkbuster: 0,
                                     g_humanplayers: 0,
                                     g_needpass: 0,}; 4096],
                   numGlobalServerAddresses: 0,
                   globalServerAddresses:
                       [netadr_t{type_0: NA_BAD,
                                 ip: [0; 4],
                                 ip6: [0; 16],
                                 port: 0,
                                 scope_id: 0,}; 4096],
                   numfavoriteservers: 0,
                   favoriteServers:
                       [serverInfo_t{adr:
                                         netadr_t{type_0: NA_BAD,
                                                  ip: [0; 4],
                                                  ip6: [0; 16],
                                                  port: 0,
                                                  scope_id: 0,},
                                     hostName: [0; 32],
                                     mapName: [0; 32],
                                     game: [0; 32],
                                     netType: 0,
                                     gameType: 0,
                                     clients: 0,
                                     maxClients: 0,
                                     minPing: 0,
                                     maxPing: 0,
                                     ping: 0,
                                     visible: qfalse,
                                     punkbuster: 0,
                                     g_humanplayers: 0,
                                     g_needpass: 0,}; 128],
                   pingUpdateSource: 0,
                   updateServer:
                       netadr_t{type_0: NA_BAD,
                                ip: [0; 4],
                                ip6: [0; 16],
                                port: 0,
                                scope_id: 0,},
                   updateChallenge: [0; 1024],
                   updateInfoString: [0; 1024],
                   authorizeServer:
                       netadr_t{type_0: NA_BAD,
                                ip: [0; 4],
                                ip6: [0; 16],
                                port: 0,
                                scope_id: 0,},
                   rconAddress:
                       netadr_t{type_0: NA_BAD,
                                ip: [0; 4],
                                ip6: [0; 16],
                                port: 0,
                                scope_id: 0,},
                   glconfig:
                       glconfig_t{renderer_string: [0; 1024],
                                  vendor_string: [0; 1024],
                                  version_string: [0; 1024],
                                  extensions_string: [0; 8192],
                                  maxTextureSize: 0,
                                  numTextureUnits: 0,
                                  colorBits: 0,
                                  depthBits: 0,
                                  stencilBits: 0,
                                  driverType: GLDRV_ICD,
                                  hardwareType: GLHW_GENERIC,
                                  deviceSupportsGamma: qfalse,
                                  textureCompression: TC_NONE,
                                  textureEnvAddAvailable: qfalse,
                                  vidWidth: 0,
                                  vidHeight: 0,
                                  windowAspect: 0.,
                                  displayFrequency: 0,
                                  isFullscreen: qfalse,
                                  stereoEnabled: qfalse,
                                  smpActive: qfalse,},
                   charSetShader: 0,
                   whiteShader: 0,
                   consoleShader: 0,};
unsafe extern "C" fn CL_SetServerInfo(mut server: *mut serverInfo_t,
                                      mut info: *const libc::c_char,
                                      mut ping: libc::c_int) {
    if !server.is_null() {
        if !info.is_null() {
            (*server).clients =
                atoi(Info_ValueForKey(info,
                                      b"clients\x00" as *const u8 as
                                          *const libc::c_char));
            Q_strncpyz((*server).hostName.as_mut_ptr(),
                       Info_ValueForKey(info,
                                        b"hostname\x00" as *const u8 as
                                            *const libc::c_char), 32i32);
            Q_strncpyz((*server).mapName.as_mut_ptr(),
                       Info_ValueForKey(info,
                                        b"mapname\x00" as *const u8 as
                                            *const libc::c_char), 32i32);
            (*server).maxClients =
                atoi(Info_ValueForKey(info,
                                      b"sv_maxclients\x00" as *const u8 as
                                          *const libc::c_char));
            Q_strncpyz((*server).game.as_mut_ptr(),
                       Info_ValueForKey(info,
                                        b"game\x00" as *const u8 as
                                            *const libc::c_char), 32i32);
            (*server).gameType =
                atoi(Info_ValueForKey(info,
                                      b"gametype\x00" as *const u8 as
                                          *const libc::c_char));
            (*server).netType =
                atoi(Info_ValueForKey(info,
                                      b"nettype\x00" as *const u8 as
                                          *const libc::c_char));
            (*server).minPing =
                atoi(Info_ValueForKey(info,
                                      b"minping\x00" as *const u8 as
                                          *const libc::c_char));
            (*server).maxPing =
                atoi(Info_ValueForKey(info,
                                      b"maxping\x00" as *const u8 as
                                          *const libc::c_char));
            (*server).punkbuster =
                atoi(Info_ValueForKey(info,
                                      b"punkbuster\x00" as *const u8 as
                                          *const libc::c_char));
            (*server).g_humanplayers =
                atoi(Info_ValueForKey(info,
                                      b"g_humanplayers\x00" as *const u8 as
                                          *const libc::c_char));
            (*server).g_needpass =
                atoi(Info_ValueForKey(info,
                                      b"g_needpass\x00" as *const u8 as
                                          *const libc::c_char))
        }
        (*server).ping = ping
    };
}
/*
==================
CL_GetFreePing
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetFreePing() -> *mut ping_t {
    let mut pingptr: *mut ping_t = 0 as *mut ping_t;
    let mut best: *mut ping_t = 0 as *mut ping_t;
    let mut oldest: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    pingptr = cl_pinglist.as_mut_ptr();
    let mut current_block_3: u64;
    i = 0i32;
    while i < 32i32 {
        // find free ping slot
        if 0 != (*pingptr).adr.port {
            if 0 == (*pingptr).time {
                if Sys_Milliseconds() - (*pingptr).start < 500i32 {
                    // still waiting for response
                    current_block_3 = 735147466149431745;
                } else { current_block_3 = 1841672684692190573; }
            } else if (*pingptr).time < 500i32 {
                // results have not been queried
                current_block_3 = 735147466149431745;
            } else { current_block_3 = 1841672684692190573; }
            match current_block_3 {
                1841672684692190573 => { }
                _ => { i += 1; pingptr = pingptr.offset(1isize); continue ; }
            }
        }
        (*pingptr).adr.port = 0i32 as libc::c_ushort;
        return pingptr
    }
    pingptr = cl_pinglist.as_mut_ptr();
    best = cl_pinglist.as_mut_ptr();
    oldest = -2147483647i32 - 1i32;
    i = 0i32;
    while i < 32i32 {
        time = Sys_Milliseconds() - (*pingptr).start;
        if time > oldest { oldest = time; best = pingptr }
        i += 1;
        pingptr = pingptr.offset(1isize)
    }
    return best;
}
#[no_mangle]
pub static mut cl_pinglist: [ping_t; 32] =
    [ping_t{adr:
                netadr_t{type_0: NA_BAD,
                         ip: [0; 4],
                         ip6: [0; 16],
                         port: 0,
                         scope_id: 0,},
            start: 0,
            time: 0,
            info: [0; 1024],}; 32];
/*
==================
CL_CompleteRcon
==================
*/
unsafe extern "C" fn CL_CompleteRcon(mut args: *mut libc::c_char,
                                     mut argNum: libc::c_int) {
    if argNum == 2i32 {
        let mut p: *mut libc::c_char =
            Com_SkipTokens(args, 1i32,
                           b" \x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        if p > args { Field_CompleteCommand(p, qtrue, qtrue); }
    };
}
/*
=====================
CL_Rcon_f

  Send the rest of the command line over as
  an unconnected command.
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Rcon_f() {
    let mut message: [libc::c_char; 1024] = [0; 1024];
    let mut to: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    if 0 == *(*rcon_client_password).string.offset(0isize) {
        Com_Printf(b"You must set \'rconpassword\' before\nissuing an rcon command.\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    message[0usize] = -1i32 as libc::c_char;
    message[1usize] = -1i32 as libc::c_char;
    message[2usize] = -1i32 as libc::c_char;
    message[3usize] = -1i32 as libc::c_char;
    message[4usize] = 0i32 as libc::c_char;
    Q_strcat(message.as_mut_ptr(), 1024i32,
             b"rcon \x00" as *const u8 as *const libc::c_char);
    Q_strcat(message.as_mut_ptr(), 1024i32, (*rcon_client_password).string);
    Q_strcat(message.as_mut_ptr(), 1024i32,
             b" \x00" as *const u8 as *const libc::c_char);
    Q_strcat(message.as_mut_ptr(), 1024i32, Cmd_Cmd().offset(5isize));
    if clc.state as libc::c_uint >=
           CA_CONNECTED as libc::c_int as libc::c_uint {
        to = clc.netchan.remoteAddress
    } else {
        if 0 == strlen((*rconAddress).string) {
            Com_Printf(b"You must either be connected,\nor set the \'rconAddress\' cvar\nto issue rcon commands\n\x00"
                           as *const u8 as *const libc::c_char);
            return
        }
        NET_StringToAdr((*rconAddress).string, &mut to, NA_UNSPEC);
        if to.port as libc::c_int == 0i32 {
            to.port = ShortSwap(27960i32 as libc::c_short) as libc::c_ushort
        }
    }
    NET_SendPacket(NS_CLIENT,
                   strlen(message.as_mut_ptr()).wrapping_add(1i32 as
                                                                 libc::c_ulong)
                       as libc::c_int,
                   message.as_mut_ptr() as *const libc::c_void, to);
    cls.rconAddress = to;
}
#[no_mangle]
pub static mut rconAddress: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut rcon_client_password: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CL_GlobalServers_f() {
    let mut to: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut masterNum: libc::c_int = 0;
    let mut command: [libc::c_char; 1024] = [0; 1024];
    let mut masteraddress: *mut libc::c_char = 0 as *mut libc::c_char;
    count = Cmd_Argc();
    if count < 3i32 || { masterNum = atoi(Cmd_Argv(1i32)); masterNum < 0i32 }
           || masterNum > 5i32 {
        Com_Printf(b"usage: globalservers <master# 0-%d> <protocol> [keywords]\n\x00"
                       as *const u8 as *const libc::c_char, 5i32);
        return
    }
    if masterNum == 0i32 {
        let mut numAddress: libc::c_int = 0i32;
        i = 1i32;
        while i <= 5i32 {
            sprintf(command.as_mut_ptr(),
                    b"sv_master%d\x00" as *const u8 as *const libc::c_char,
                    i);
            masteraddress = Cvar_VariableString(command.as_mut_ptr());
            if !(0 == *masteraddress) {
                numAddress += 1;
                Com_sprintf(command.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"globalservers %d %s %s\n\x00" as *const u8 as
                                *const libc::c_char, i, Cmd_Argv(2i32),
                            Cmd_ArgsFrom(3i32));
                Cbuf_AddText(command.as_mut_ptr());
            }
            i += 1
        }
        if 0 == numAddress {
            Com_Printf(b"CL_GlobalServers_f: Error: No master server addresses.\n\x00"
                           as *const u8 as *const libc::c_char);
        }
        return
    }
    sprintf(command.as_mut_ptr(),
            b"sv_master%d\x00" as *const u8 as *const libc::c_char,
            masterNum);
    masteraddress = Cvar_VariableString(command.as_mut_ptr());
    if 0 == *masteraddress {
        Com_Printf(b"CL_GlobalServers_f: Error: No master server address given.\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    i = NET_StringToAdr(masteraddress, &mut to, NA_UNSPEC);
    if 0 == i {
        Com_Printf(b"CL_GlobalServers_f: Error: could not resolve address of master %s\n\x00"
                       as *const u8 as *const libc::c_char, masteraddress);
        return
    } else {
        if i == 2i32 {
            to.port = ShortSwap(27950i32 as libc::c_short) as libc::c_ushort
        }
    }
    Com_Printf(b"Requesting servers from %s (%s)...\n\x00" as *const u8 as
                   *const libc::c_char, masteraddress,
               NET_AdrToStringwPort(to));
    cls.numglobalservers = -1i32;
    cls.pingUpdateSource = 2i32;
    if to.type_0 as libc::c_uint == NA_IP6 as libc::c_int as libc::c_uint ||
           to.type_0 as libc::c_uint ==
               NA_MULTICAST6 as libc::c_int as libc::c_uint {
        let mut v4enabled: libc::c_int =
            Cvar_VariableIntegerValue(b"net_enabled\x00" as *const u8 as
                                          *const libc::c_char) & 0x1i32;
        if 0 != v4enabled {
            Com_sprintf(command.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong as libc::c_int,
                        b"getserversExt %s %s\x00" as *const u8 as
                            *const libc::c_char, (*com_gamename).string,
                        Cmd_Argv(2i32));
        } else {
            Com_sprintf(command.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong as libc::c_int,
                        b"getserversExt %s %s ipv6\x00" as *const u8 as
                            *const libc::c_char, (*com_gamename).string,
                        Cmd_Argv(2i32));
        }
    } else if 0 ==
                  Q_stricmp((*com_gamename).string,
                            b"Quake3Arena\x00" as *const u8 as
                                *const libc::c_char) {
        Com_sprintf(command.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong as libc::c_int,
                    b"getservers %s\x00" as *const u8 as *const libc::c_char,
                    Cmd_Argv(2i32));
    } else {
        Com_sprintf(command.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong as libc::c_int,
                    b"getservers %s %s\x00" as *const u8 as
                        *const libc::c_char, (*com_gamename).string,
                    Cmd_Argv(2i32));
    }
    i = 3i32;
    while i < count {
        Q_strcat(command.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int,
                 b" \x00" as *const u8 as *const libc::c_char);
        Q_strcat(command.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int, Cmd_Argv(i));
        i += 1
    }
    NET_OutOfBandPrint(NS_SERVER, to,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       command.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CL_LocalServers_f() {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut to: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    Com_Printf(b"Scanning for servers on the local network...\n\x00" as
                   *const u8 as *const libc::c_char);
    cls.numlocalservers = 0i32;
    cls.pingUpdateSource = 0i32;
    i = 0i32;
    while i < 128i32 {
        let mut b: qboolean = cls.localServers[i as usize].visible;
        memset(&mut *cls.localServers.as_mut_ptr().offset(i as isize) as
                   *mut serverInfo_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<serverInfo_t>() as libc::c_ulong);
        cls.localServers[i as usize].visible = b;
        i += 1
    }
    memset(&mut to as *mut netadr_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    message =
        b"\xff\xff\xff\xffgetinfo xxx\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    i = 0i32;
    while i < 2i32 {
        j = 0i32;
        while j < 4i32 {
            to.port =
                ShortSwap((27960i32 + j) as libc::c_short) as libc::c_ushort;
            to.type_0 = NA_BROADCAST;
            NET_SendPacket(NS_CLIENT, strlen(message) as libc::c_int,
                           message as *const libc::c_void, to);
            to.type_0 = NA_MULTICAST6;
            NET_SendPacket(NS_CLIENT, strlen(message) as libc::c_int,
                           message as *const libc::c_void, to);
            j += 1
        }
        i += 1
    };
}
/*
================
CL_Reconnect_f

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Reconnect_f() {
    if 0 == strlen(cl_reconnectArgs.as_mut_ptr()) { return }
    Cvar_Set(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Cbuf_AddText(va(b"connect %s\n\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, cl_reconnectArgs.as_mut_ptr()));
}
#[no_mangle]
pub static mut cl_reconnectArgs: [libc::c_char; 4096] = [0; 4096];
/*
================
CL_Connect_f

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Connect_f() {
    let mut server: [libc::c_char; 4096] = [0; 4096];
    let mut serverString: *const libc::c_char = 0 as *const libc::c_char;
    let mut argc: libc::c_int = Cmd_Argc();
    let mut family: netadrtype_t = NA_UNSPEC;
    if argc != 2i32 && argc != 3i32 {
        Com_Printf(b"usage: connect [-4|-6] server\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if argc == 2i32 {
        Q_strncpyz(server.as_mut_ptr(), Cmd_Argv(1i32),
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong as libc::c_int);
    } else {
        if 0 ==
               strcmp(Cmd_Argv(1i32),
                      b"-4\x00" as *const u8 as *const libc::c_char) {
            family = NA_IP
        } else if 0 ==
                      strcmp(Cmd_Argv(1i32),
                             b"-6\x00" as *const u8 as *const libc::c_char) {
            family = NA_IP6
        } else {
            Com_Printf(b"warning: only -4 or -6 as address type understood.\n\x00"
                           as *const u8 as *const libc::c_char);
        }
        Q_strncpyz(server.as_mut_ptr(), Cmd_Argv(2i32),
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong as libc::c_int);
    }
    Q_strncpyz(cl_reconnectArgs.as_mut_ptr(), Cmd_Args(),
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Cvar_Set(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    CL_RequestMotd();
    clc.serverMessage[0usize] = 0i32 as libc::c_char;
    if 0 != (*com_sv_running).integer &&
           0 ==
               strcmp(server.as_mut_ptr(),
                      b"localhost\x00" as *const u8 as *const libc::c_char) {
        SV_Shutdown(b"Server quit\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    }
    Cvar_Set(b"sv_killserver\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
    SV_Frame(0i32);
    noGameRestart = qtrue as libc::c_int;
    CL_Disconnect(qtrue);
    Con_Close();
    Q_strncpyz(clc.servername.as_mut_ptr(), server.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    if 0 ==
           NET_StringToAdr(clc.servername.as_mut_ptr(),
                           &mut clc.serverAddress, family) {
        Com_Printf(b"Bad server address\n\x00" as *const u8 as
                       *const libc::c_char);
        clc.state = CA_DISCONNECTED;
        return
    }
    if clc.serverAddress.port as libc::c_int == 0i32 {
        clc.serverAddress.port =
            ShortSwap(27960i32 as libc::c_short) as libc::c_ushort
    }
    serverString = NET_AdrToStringwPort(clc.serverAddress);
    Com_Printf(b"%s resolved to %s\n\x00" as *const u8 as *const libc::c_char,
               clc.servername.as_mut_ptr(), serverString);
    if 0 != (*cl_guidServerUniq).integer {
        CL_UpdateGUID(serverString, strlen(serverString) as libc::c_int);
    } else { CL_UpdateGUID(0 as *const libc::c_char, 0i32); }
    if 0 != NET_IsLocalAddress(clc.serverAddress) as u64 {
        clc.state = CA_CHALLENGING
    } else {
        clc.state = CA_CONNECTING;
        clc.challenge =
            ((rand() as libc::c_uint) << 16i32 ^ rand() as libc::c_uint ^
                 Com_Milliseconds() as libc::c_uint) as libc::c_int
    }
    Key_SetCatcher(0i32);
    clc.connectTime = -99999i32;
    clc.connectPacketCount = 0i32;
    Cvar_Set(b"cl_currentServerAddress\x00" as *const u8 as
                 *const libc::c_char, server.as_mut_ptr());
}
#[no_mangle]
pub static mut cl_guidServerUniq: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CL_Disconnect(mut showMainMenu: qboolean) {
    if com_cl_running.is_null() || 0 == (*com_cl_running).integer { return }
    Cvar_Set(b"r_uiFullScreen\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
    if 0 != clc.demorecording as u64 { CL_StopRecord_f(); }
    if 0 != clc.download { FS_FCloseFile(clc.download); clc.download = 0i32 }
    let ref mut fresh0 = *clc.downloadName.as_mut_ptr();
    *fresh0 = 0i32 as libc::c_char;
    *clc.downloadTempName.as_mut_ptr() = *fresh0;
    Cvar_Set(b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char);
    if 0 != (*cl_useMumble).integer && 0 != mumble_islinked() {
        Com_Printf(b"Mumble: Unlinking from Mumble application\n\x00" as
                       *const u8 as *const libc::c_char);
        mumble_unlink();
    }
    if 0 != (*cl_voipSend).integer {
        let mut tmp: libc::c_int = (*cl_voipUseVAD).integer;
        (*cl_voipUseVAD).integer = 0i32;
        clc.voipOutgoingDataSize = 0i32;
        Cvar_Set(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
        CL_CaptureVoip();
        (*cl_voipUseVAD).integer = tmp
    }
    if 0 != clc.voipCodecInitialized as u64 {
        let mut i: libc::c_int = 0;
        opus_encoder_destroy(clc.opusEncoder);
        i = 0i32;
        while i < 64i32 {
            opus_decoder_destroy(clc.opusDecoder[i as usize]);
            i += 1
        }
        clc.voipCodecInitialized = qfalse
    }
    Cmd_RemoveCommand(b"voip\x00" as *const u8 as *const libc::c_char);
    if 0 != clc.demofile { FS_FCloseFile(clc.demofile); clc.demofile = 0i32 }
    if !uivm.is_null() && 0 != showMainMenu as libc::c_uint {
        VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                UIMENU_NONE as libc::c_int);
    }
    SCR_StopCinematic();
    S_ClearSoundBuffer();
    if clc.state as libc::c_uint >=
           CA_CONNECTED as libc::c_int as libc::c_uint {
        CL_AddReliableCommand(b"disconnect\x00" as *const u8 as
                                  *const libc::c_char, qtrue);
        CL_WritePacket();
        CL_WritePacket();
        CL_WritePacket();
    }
    FS_PureServerSetLoadedPaks(b"\x00" as *const u8 as *const libc::c_char,
                               b"\x00" as *const u8 as *const libc::c_char);
    FS_PureServerSetReferencedPaks(b"\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"\x00" as *const u8 as
                                       *const libc::c_char);
    CL_ClearState();
    memset(&mut clc as *mut clientConnection_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<clientConnection_t>() as libc::c_ulong);
    clc.state = CA_DISCONNECTED;
    Cvar_Set(b"sv_cheats\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
    cl_connectedToPureServer = qfalse as libc::c_int;
    clc.voipEnabled = qfalse;
    if 0 != CL_VideoRecording() as u64 { SCR_UpdateScreen(); CL_CloseAVI(); }
    CL_UpdateGUID(0 as *const libc::c_char, 0i32);
    if 0 == noGameRestart {
        CL_OldGame();
    } else { noGameRestart = qfalse as libc::c_int };
}
static mut noGameRestart: libc::c_int = qfalse as libc::c_int;
unsafe extern "C" fn CL_OldGame() {
    if 0 != cl_oldGameSet as u64 {
        cl_oldGameSet = qfalse;
        Cvar_Set2(b"fs_game\x00" as *const u8 as *const libc::c_char,
                  cl_oldGame.as_mut_ptr(), qtrue);
        FS_ConditionalRestart(clc.checksumFeed, qfalse);
    };
}
#[no_mangle]
pub static mut cl_oldGame: [libc::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut cl_oldGameSet: qboolean = qfalse;
#[no_mangle]
pub unsafe extern "C" fn CL_ClearState() {
    memset(&mut cl as *mut clientActive_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<clientActive_t>() as libc::c_ulong);
}
// cl_voipSendTarget is a string: "all" to broadcast to everyone, "none" to
//  send to no one, or a comma-separated list of client numbers:
//  "0,7,2,23" ... an empty string is treated like "all".
#[no_mangle]
pub static mut cl_voipUseVAD: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
===============
CL_CaptureVoip

Record more audio from the hardware if required and encode it into Opus
 data for later transmission.
===============
*/
unsafe extern "C" fn CL_CaptureVoip() {
    let audioMult: libc::c_float = (*cl_voipCaptureMult).value;
    let useVad: qboolean =
        ((*cl_voipUseVAD).integer != 0i32) as libc::c_int as qboolean;
    let mut initialFrame: qboolean = qfalse;
    let mut finalFrame: qboolean = qfalse;
    if 0 != (*cl_useMumble).integer { return }
    if 0 != (*cl_voip).modified as libc::c_uint ||
           0 != (*cl_rate).modified as libc::c_uint {
        if 0 != (*cl_voip).integer && (*cl_rate).integer < 25000i32 {
            Com_Printf(b"^3Your network rate is too slow for VoIP.\n\x00" as
                           *const u8 as *const libc::c_char);
            Com_Printf(b"Set \'Data Rate\' to \'LAN/Cable/xDSL\' in \'Setup/System/Network\'.\n\x00"
                           as *const u8 as *const libc::c_char);
            Com_Printf(b"Until then, VoIP is disabled.\n\x00" as *const u8 as
                           *const libc::c_char);
            Cvar_Set(b"cl_voip\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char);
        }
        Cvar_Set(b"cl_voipProtocol\x00" as *const u8 as *const libc::c_char,
                 if 0 != (*cl_voip).integer {
                     b"opus\x00" as *const u8 as *const libc::c_char
                 } else { b"\x00" as *const u8 as *const libc::c_char });
        (*cl_voip).modified = qfalse;
        (*cl_rate).modified = qfalse
    }
    if 0 == clc.voipCodecInitialized as u64 { return }
    if clc.voipOutgoingDataSize > 0i32 { return }
    if 0 != (*cl_voipUseVAD).modified as u64 {
        Cvar_Set(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
                 if 0 != useVad as libc::c_uint {
                     b"1\x00" as *const u8 as *const libc::c_char
                 } else { b"0\x00" as *const u8 as *const libc::c_char });
        (*cl_voipUseVAD).modified = qfalse
    }
    if 0 != useVad as libc::c_uint && 0 == (*cl_voipSend).integer {
        Cvar_Set(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != (*cl_voipSend).modified as u64 {
        let mut dontCapture: qboolean = qfalse;
        if clc.state as libc::c_uint !=
               CA_ACTIVE as libc::c_int as libc::c_uint {
            dontCapture = qtrue
        } else if 0 == clc.voipEnabled as u64 {
            dontCapture = qtrue
        } else if 0 != clc.demoplaying as u64 {
            dontCapture = qtrue
        } else if (*cl_voip).integer == 0i32 {
            dontCapture = qtrue
        } else if audioMult == 0.0f32 { dontCapture = qtrue }
        (*cl_voipSend).modified = qfalse;
        if 0 != dontCapture as u64 {
            Cvar_Set(b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char);
            return
        }
        if 0 != (*cl_voipSend).integer {
            initialFrame = qtrue
        } else { finalFrame = qtrue }
    }
    if 0 != initialFrame as u64 {
        S_MasterGain(Com_Clamp(0.0f32, 1.0f32,
                               (*cl_voipGainDuringCapture).value));
        S_StartCapture();
        CL_VoipNewGeneration();
        CL_VoipParseTargets();
    }
    if 0 != (*cl_voipSend).integer || 0 != finalFrame as libc::c_uint {
        let mut samples: libc::c_int = S_AvailableCaptureSamples();
        let packetSamples: libc::c_int =
            if 0 != finalFrame as libc::c_uint {
                20i32 * 48i32
            } else { 20i32 * 48i32 * 3i32 };
        if samples >= packetSamples {
            static mut sampbuffer: [int16_t; 2880] = [0; 2880];
            let mut voipPower: libc::c_float = 0.0f32;
            let mut voipFrames: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut bytes: libc::c_int = 0;
            if samples > 20i32 * 48i32 * 3i32 {
                samples = 20i32 * 48i32 * 3i32
            }
            samples -= samples % (20i32 * 48i32);
            if samples != 120i32 && samples != 240i32 && samples != 480i32 &&
                   samples != 960i32 && samples != 1920i32 &&
                   samples != 2880i32 {
                Com_Printf(b"Voip: bad number of samples %d\n\x00" as
                               *const u8 as *const libc::c_char, samples);
                return
            }
            voipFrames = samples / (20i32 * 48i32);
            S_Capture(samples, sampbuffer.as_mut_ptr() as *mut byte);
            i = 0i32;
            while i < samples {
                let flsamp: libc::c_float =
                    sampbuffer[i as usize] as libc::c_float;
                let s: libc::c_float =
                    fabs(flsamp as libc::c_double) as libc::c_float;
                voipPower += s * s;
                sampbuffer[i as usize] = (flsamp * audioMult) as int16_t;
                i += 1
            }
            bytes =
                opus_encode(clc.opusEncoder, sampbuffer.as_mut_ptr(), samples,
                            clc.voipOutgoingData.as_mut_ptr(),
                            ::std::mem::size_of::<[byte; 1024]>() as
                                libc::c_ulong as opus_int32);
            if bytes <= 0i32 {
                Com_DPrintf(b"VoIP: Error encoding %d samples\n\x00" as
                                *const u8 as *const libc::c_char, samples);
                bytes = 0i32
            }
            clc.voipPower =
                voipPower /
                    (32768.0f32 * 32768.0f32 * samples as libc::c_float) *
                    100.0f32;
            if 0 != useVad as libc::c_uint &&
                   clc.voipPower < (*cl_voipVADThreshold).value {
                CL_VoipNewGeneration();
            } else {
                clc.voipOutgoingDataSize = bytes;
                clc.voipOutgoingDataFrames = voipFrames;
                Com_DPrintf(b"VoIP: Send %d frames, %d bytes, %f power\n\x00"
                                as *const u8 as *const libc::c_char,
                            voipFrames, bytes,
                            clc.voipPower as libc::c_double);
            }
        }
    }
    if 0 != finalFrame as u64 {
        S_StopCapture();
        S_MasterGain(1.0f32);
        clc.voipPower = 0.0f32
    };
}
unsafe extern "C" fn CL_VoipNewGeneration() {
    clc.voipOutgoingGeneration = clc.voipOutgoingGeneration.wrapping_add(1);
    if clc.voipOutgoingGeneration as libc::c_int <= 0i32 {
        clc.voipOutgoingGeneration = 1i32 as byte
    }
    clc.voipPower = 0.0f32;
    clc.voipOutgoingSequence = 0i32;
    opus_encoder_ctl(clc.opusEncoder, 4028i32);
}
#[no_mangle]
pub static mut cl_voipVADThreshold: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_voipCaptureMult: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_voipSend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
===============
CL_VoipParseTargets

sets clc.voipTargets according to cl_voipSendTarget
Generally we don't want who's listening to change during a transmission,
so this is only called when the key is first pressed
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_VoipParseTargets() {
    let mut target: *const libc::c_char = (*cl_voipSendTarget).string;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_int = 0;
    memset(clc.voipTargets.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong);
    clc.voipFlags = (clc.voipFlags as libc::c_int & !0x1i32) as uint8_t;
    while !target.is_null() {
        while *target as libc::c_int == ',' as i32 ||
                  *target as libc::c_int == ' ' as i32 {
            target = target.offset(1isize)
        }
        if 0 == *target { break ; }
        if 0 !=
               *(*__ctype_b_loc()).offset(*target as libc::c_int as isize) as
                   libc::c_int &
                   _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
            val = strtol(target, &mut end, 10i32) as libc::c_int;
            target = end
        } else {
            if 0 ==
                   Q_stricmpn(target,
                              b"all\x00" as *const u8 as *const libc::c_char,
                              3i32) {
                memset(clc.voipTargets.as_mut_ptr() as *mut libc::c_void,
                       !0i32,
                       ::std::mem::size_of::<[uint8_t; 8]>() as
                           libc::c_ulong);
                return
            }
            if 0 ==
                   Q_stricmpn(target,
                              b"spatial\x00" as *const u8 as
                                  *const libc::c_char, 7i32) {
                clc.voipFlags =
                    (clc.voipFlags as libc::c_int | 0x1i32) as uint8_t;
                target = target.offset(7isize);
                continue ;
            } else {
                if 0 ==
                       Q_stricmpn(target,
                                  b"attacker\x00" as *const u8 as
                                      *const libc::c_char, 8i32) {
                    val =
                        VM_Call(cgvm, CG_LAST_ATTACKER as libc::c_int) as
                            libc::c_int;
                    target = target.offset(8isize)
                } else if 0 ==
                              Q_stricmpn(target,
                                         b"crosshair\x00" as *const u8 as
                                             *const libc::c_char, 9i32) {
                    val =
                        VM_Call(cgvm, CG_CROSSHAIR_PLAYER as libc::c_int) as
                            libc::c_int;
                    target = target.offset(9isize)
                } else {
                    while 0 != *target as libc::c_int &&
                              *target as libc::c_int != ',' as i32 &&
                              *target as libc::c_int != ' ' as i32 {
                        target = target.offset(1isize)
                    }
                    continue ;
                }
                if val < 0i32 { continue ; }
            }
        }
        if val < 0i32 || val >= 64i32 {
            Com_Printf(b"^3WARNING: VoIP target %d is not a valid client number\n\x00"
                           as *const u8 as *const libc::c_char, val);
        } else {
            clc.voipTargets[(val / 8i32) as usize] =
                (clc.voipTargets[(val / 8i32) as usize] as libc::c_int |
                     1i32 << val % 8i32) as uint8_t
        }
    };
}
#[no_mangle]
pub static mut cl_voipSendTarget: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
//=============================================================================
// interface to cgame dll or vm
#[no_mangle]
pub static mut cgvm: *mut vm_t = 0 as *const vm_t as *mut vm_t;
#[no_mangle]
pub static mut cl_voipGainDuringCapture: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_voip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_rate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_useMumble: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CL_StopRecord_f() {
    let mut len: libc::c_int = 0;
    if 0 == clc.demorecording as u64 {
        Com_Printf(b"Not recording a demo.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    len = -1i32;
    FS_Write(&mut len as *mut libc::c_int as *const libc::c_void, 4i32,
             clc.demofile);
    FS_Write(&mut len as *mut libc::c_int as *const libc::c_void, 4i32,
             clc.demofile);
    FS_FCloseFile(clc.demofile);
    clc.demofile = 0i32;
    clc.demorecording = qfalse;
    clc.spDemoRecording = qfalse;
    Com_Printf(b"Stopped demo.\n\x00" as *const u8 as *const libc::c_char);
}
/*
===================
CL_RequestMotd

===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RequestMotd() {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    if 0 == (*cl_motd).integer { return }
    Com_Printf(b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
               b"update.quake3arena.com\x00" as *const u8 as
                   *const libc::c_char);
    if 0 ==
           NET_StringToAdr(b"update.quake3arena.com\x00" as *const u8 as
                               *const libc::c_char, &mut cls.updateServer,
                           NA_IP) {
        Com_Printf(b"Couldn\'t resolve address\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cls.updateServer.port =
        ShortSwap(27951i32 as libc::c_short) as libc::c_ushort;
    Com_Printf(b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as
                   *const libc::c_char,
               b"update.quake3arena.com\x00" as *const u8 as
                   *const libc::c_char,
               cls.updateServer.ip[0usize] as libc::c_int,
               cls.updateServer.ip[1usize] as libc::c_int,
               cls.updateServer.ip[2usize] as libc::c_int,
               cls.updateServer.ip[3usize] as libc::c_int,
               ShortSwap(cls.updateServer.port as libc::c_short) as
                   libc::c_int);
    info[0usize] = 0i32 as libc::c_char;
    Com_sprintf(cls.updateChallenge.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%i\x00" as *const u8 as *const libc::c_char,
                ((rand() as libc::c_uint) << 16i32 ^ rand() as libc::c_uint ^
                     Com_Milliseconds() as libc::c_uint) as libc::c_int);
    Info_SetValueForKey(info.as_mut_ptr(),
                        b"challenge\x00" as *const u8 as *const libc::c_char,
                        cls.updateChallenge.as_mut_ptr());
    Info_SetValueForKey(info.as_mut_ptr(),
                        b"renderer\x00" as *const u8 as *const libc::c_char,
                        cls.glconfig.renderer_string.as_mut_ptr());
    Info_SetValueForKey(info.as_mut_ptr(),
                        b"version\x00" as *const u8 as *const libc::c_char,
                        (*com_version).string);
    NET_OutOfBandPrint(NS_CLIENT, cls.updateServer,
                       b"getmotd \"%s\"\n\x00" as *const u8 as
                           *const libc::c_char, info.as_mut_ptr());
}
#[no_mangle]
pub static mut cl_motd: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
====================
CL_CompleteDemoName
====================
*/
unsafe extern "C" fn CL_CompleteDemoName(mut args: *mut libc::c_char,
                                         mut argNum: libc::c_int) {
    if argNum == 2i32 {
        let mut demoExt: [libc::c_char; 16] = [0; 16];
        Com_sprintf(demoExt.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 16]>() as
                        libc::c_ulong as libc::c_int,
                    b".%s%d\x00" as *const u8 as *const libc::c_char,
                    b"dm_\x00" as *const u8 as *const libc::c_char,
                    (*com_protocol).integer);
        Field_CompleteFilename(b"demos\x00" as *const u8 as
                                   *const libc::c_char, demoExt.as_mut_ptr(),
                               qtrue, qtrue);
    };
}
/*
====================
CL_PlayDemo_f

demo <demoname>

====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PlayDemo_f() {
    let mut name: [libc::c_char; 4096] = [0; 4096];
    let mut arg: [libc::c_char; 4096] = [0; 4096];
    let mut ext_test: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut protocol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut retry: [libc::c_char; 4096] = [0; 4096];
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"demo <demoname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Cvar_Set(b"sv_killserver\x00" as *const u8 as *const libc::c_char,
             b"2\x00" as *const u8 as *const libc::c_char);
    Q_strncpyz(arg.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    CL_Disconnect(qtrue);
    ext_test = strrchr(arg.as_mut_ptr(), '.' as i32);
    if !ext_test.is_null() &&
           0 ==
               Q_stricmpn(ext_test.offset(1isize),
                          b"dm_\x00" as *const u8 as *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                               as
                                                               libc::c_ulong).wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_ulong)
                              as libc::c_int) {
        protocol =
            atoi(ext_test.offset((::std::mem::size_of::<[libc::c_char; 4]>()
                                      as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                                      as
                                                                      libc::c_ulong)
                                     as isize));
        i = 0i32;
        while 0 != *demo_protocols.as_mut_ptr().offset(i as isize) {
            if *demo_protocols.as_mut_ptr().offset(i as isize) == protocol {
                break ;
            }
            i += 1
        }
        if 0 != *demo_protocols.as_mut_ptr().offset(i as isize) ||
               protocol == (*com_protocol).integer ||
               protocol == (*com_legacyprotocol).integer {
            Com_sprintf(name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as
                            libc::c_ulong as libc::c_int,
                        b"demos/%s\x00" as *const u8 as *const libc::c_char,
                        arg.as_mut_ptr());
            FS_FOpenFileRead(name.as_mut_ptr(), &mut clc.demofile, qtrue);
        } else {
            let mut len: libc::c_int = 0;
            Com_Printf(b"Protocol %d not supported for demos\n\x00" as
                           *const u8 as *const libc::c_char, protocol);
            len =
                ext_test.wrapping_offset_from(arg.as_mut_ptr()) as
                    libc::c_long as libc::c_int;
            if len as libc::c_ulong >=
                   (::std::mem::size_of::<[libc::c_char; 4096]>() as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                        as libc::c_ulong) {
                len =
                    (::std::mem::size_of::<[libc::c_char; 4096]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int
            }
            Q_strncpyz(retry.as_mut_ptr(), arg.as_mut_ptr(), len + 1i32);
            retry[len as usize] = '\u{0}' as i32 as libc::c_char;
            protocol =
                CL_WalkDemoExt(retry.as_mut_ptr(), name.as_mut_ptr(),
                               &mut clc.demofile)
        }
    } else {
        protocol =
            CL_WalkDemoExt(arg.as_mut_ptr(), name.as_mut_ptr(),
                           &mut clc.demofile)
    }
    if 0 == clc.demofile {
        Com_Error(ERR_DROP as libc::c_int,
                  b"couldn\'t open %s\x00" as *const u8 as
                      *const libc::c_char, name.as_mut_ptr());
    }
    Q_strncpyz(clc.demoName.as_mut_ptr(), arg.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    Con_Close();
    clc.state = CA_CONNECTED;
    clc.demoplaying = qtrue;
    Q_strncpyz(clc.servername.as_mut_ptr(), arg.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    if protocol <= (*com_legacyprotocol).integer {
        clc.compat = qtrue
    } else { clc.compat = qfalse }
    while clc.state as libc::c_uint >=
              CA_CONNECTED as libc::c_int as libc::c_uint &&
              (clc.state as libc::c_uint) <
                  CA_PRIMED as libc::c_int as libc::c_uint {
        CL_ReadDemoMessage();
    }
    clc.firstDemoFrameSkipped = qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn CL_ReadDemoMessage() {
    let mut r: libc::c_int = 0;
    let mut buf: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut bufData: [byte; 16384] = [0; 16384];
    let mut s: libc::c_int = 0;
    if 0 == clc.demofile { CL_DemoCompleted(); return }
    r =
        FS_Read(&mut s as *mut libc::c_int as *mut libc::c_void, 4i32,
                clc.demofile);
    if r != 4i32 { CL_DemoCompleted(); return }
    clc.serverMessageSequence = s;
    MSG_Init(&mut buf, bufData.as_mut_ptr(),
             ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                 libc::c_int);
    r =
        FS_Read(&mut buf.cursize as *mut libc::c_int as *mut libc::c_void,
                4i32, clc.demofile);
    if r != 4i32 { CL_DemoCompleted(); return }
    buf.cursize = buf.cursize;
    if buf.cursize == -1i32 { CL_DemoCompleted(); return }
    if buf.cursize > buf.maxsize {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_ReadDemoMessage: demoMsglen > MAX_MSGLEN\x00" as
                      *const u8 as *const libc::c_char);
    }
    r = FS_Read(buf.data as *mut libc::c_void, buf.cursize, clc.demofile);
    if r != buf.cursize {
        Com_Printf(b"Demo file was truncated.\n\x00" as *const u8 as
                       *const libc::c_char);
        CL_DemoCompleted();
        return
    }
    clc.lastPacketTime = cls.realtime;
    buf.readcount = 0i32;
    CL_ParseServerMessage(&mut buf);
}
/*
=================
CL_DemoCompleted
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoCompleted() {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    if !cl_timedemo.is_null() && 0 != (*cl_timedemo).integer {
        let mut time: libc::c_int = 0;
        time = Sys_Milliseconds() - clc.timeDemoStart;
        if time > 0i32 {
            Com_sprintf(buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong as libc::c_int,
                        b"%i frames %3.1f seconds %3.1f fps %d.0/%.1f/%d.0/%.1f ms\n\x00"
                            as *const u8 as *const libc::c_char,
                        clc.timeDemoFrames,
                        time as libc::c_double / 1000.0f64,
                        clc.timeDemoFrames as libc::c_double * 1000.0f64 /
                            time as libc::c_double, clc.timeDemoMinDuration,
                        (time as libc::c_float /
                             clc.timeDemoFrames as libc::c_float) as
                            libc::c_double, clc.timeDemoMaxDuration,
                        CL_DemoFrameDurationSDev() as libc::c_double);
            Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       buffer.as_mut_ptr());
            if !cl_timedemoLog.is_null() &&
                   strlen((*cl_timedemoLog).string) > 0i32 as libc::c_ulong {
                let mut i: libc::c_int = 0;
                let mut numFrames: libc::c_int = 0;
                let mut f: fileHandle_t = 0;
                if clc.timeDemoFrames - 1i32 > 4096i32 {
                    numFrames = 4096i32
                } else { numFrames = clc.timeDemoFrames - 1i32 }
                f = FS_FOpenFileWrite((*cl_timedemoLog).string);
                if 0 != f {
                    FS_Printf(f,
                              b"# %s\x00" as *const u8 as *const libc::c_char,
                              buffer.as_mut_ptr());
                    i = 0i32;
                    while i < numFrames {
                        FS_Printf(f,
                                  b"%d\n\x00" as *const u8 as
                                      *const libc::c_char,
                                  clc.timeDemoDurations[i as usize] as
                                      libc::c_int);
                        i += 1
                    }
                    FS_FCloseFile(f);
                    Com_Printf(b"%s written\n\x00" as *const u8 as
                                   *const libc::c_char,
                               (*cl_timedemoLog).string);
                } else {
                    Com_Printf(b"Couldn\'t open %s for writing\n\x00" as
                                   *const u8 as *const libc::c_char,
                               (*cl_timedemoLog).string);
                }
            }
        }
    }
    CL_Disconnect(qtrue);
    CL_NextDemo();
}
#[no_mangle]
pub unsafe extern "C" fn CL_NextDemo() {
    let mut v: [libc::c_char; 1024] = [0; 1024];
    Q_strncpyz(v.as_mut_ptr(),
               Cvar_VariableString(b"nextdemo\x00" as *const u8 as
                                       *const libc::c_char),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    v[(1024i32 - 1i32) as usize] = 0i32 as libc::c_char;
    Com_DPrintf(b"CL_NextDemo: %s\n\x00" as *const u8 as *const libc::c_char,
                v.as_mut_ptr());
    if 0 == v[0usize] { return }
    Cvar_Set(b"nextdemo\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char);
    Cbuf_AddText(v.as_mut_ptr());
    Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
    Cbuf_Execute();
}
#[no_mangle]
pub static mut cl_timedemoLog: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
// the rest of the demo file will be copied from net messages
/*
=======================================================================

CLIENT SIDE DEMO PLAYBACK

=======================================================================
*/
/*
=================
CL_DemoFrameDurationSDev
=================
*/
unsafe extern "C" fn CL_DemoFrameDurationSDev() -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut numFrames: libc::c_int = 0;
    let mut mean: libc::c_float = 0.0f32;
    let mut variance: libc::c_float = 0.0f32;
    if clc.timeDemoFrames - 1i32 > 4096i32 {
        numFrames = 4096i32
    } else { numFrames = clc.timeDemoFrames - 1i32 }
    i = 0i32;
    while i < numFrames {
        mean +=
            clc.timeDemoDurations[i as usize] as libc::c_int as libc::c_float;
        i += 1
    }
    mean /= numFrames as libc::c_float;
    i = 0i32;
    while i < numFrames {
        let mut x: libc::c_float =
            clc.timeDemoDurations[i as usize] as libc::c_float;
        variance += (x - mean) * (x - mean);
        i += 1
    }
    variance /= numFrames as libc::c_float;
    return sqrt(variance as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub static mut cl_timedemo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
====================
CL_WalkDemoExt
====================
*/
unsafe extern "C" fn CL_WalkDemoExt(mut arg: *mut libc::c_char,
                                    mut name: *mut libc::c_char,
                                    mut demofile: *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    *demofile = 0i32;
    if (*com_legacyprotocol).integer > 0i32 {
        Com_sprintf(name, 4096i32,
                    b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                    arg, b"dm_\x00" as *const u8 as *const libc::c_char,
                    (*com_legacyprotocol).integer);
        FS_FOpenFileRead(name, demofile, qtrue);
        if 0 != *demofile {
            Com_Printf(b"Demo file: %s\n\x00" as *const u8 as
                           *const libc::c_char, name);
            return (*com_legacyprotocol).integer
        }
    }
    if (*com_protocol).integer != (*com_legacyprotocol).integer {
        Com_sprintf(name, 4096i32,
                    b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                    arg, b"dm_\x00" as *const u8 as *const libc::c_char,
                    (*com_protocol).integer);
        FS_FOpenFileRead(name, demofile, qtrue);
        if 0 != *demofile {
            Com_Printf(b"Demo file: %s\n\x00" as *const u8 as
                           *const libc::c_char, name);
            return (*com_protocol).integer
        }
    }
    Com_Printf(b"Not found: %s\n\x00" as *const u8 as *const libc::c_char,
               name);
    while 0 != *demo_protocols.as_mut_ptr().offset(i as isize) {
        if *demo_protocols.as_mut_ptr().offset(i as isize) ==
               (*com_legacyprotocol).integer {
            continue ;
        }
        if *demo_protocols.as_mut_ptr().offset(i as isize) ==
               (*com_protocol).integer {
            continue ;
        }
        Com_sprintf(name, 4096i32,
                    b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                    arg, b"dm_\x00" as *const u8 as *const libc::c_char,
                    *demo_protocols.as_mut_ptr().offset(i as isize));
        FS_FOpenFileRead(name, demofile, qtrue);
        if 0 != *demofile {
            Com_Printf(b"Demo file: %s\n\x00" as *const u8 as
                           *const libc::c_char, name);
            return *demo_protocols.as_mut_ptr().offset(i as isize)
        } else {
            Com_Printf(b"Not found: %s\n\x00" as *const u8 as
                           *const libc::c_char, name);
        }
        i += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn CL_Record_f() {
    let mut name: [libc::c_char; 4096] = [0; 4096];
    let mut bufData: [byte; 16384] = [0; 16384];
    let mut buf: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ent: *mut entityState_t = 0 as *mut entityState_t;
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
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() > 2i32 {
        Com_Printf(b"record <demoname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if 0 != clc.demorecording as u64 {
        if 0 == clc.spDemoRecording as u64 {
            Com_Printf(b"Already recording.\n\x00" as *const u8 as
                           *const libc::c_char);
        }
        return
    }
    if clc.state as libc::c_uint != CA_ACTIVE as libc::c_int as libc::c_uint {
        Com_Printf(b"You must be in a level to record.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if 0 != NET_IsLocalAddress(clc.serverAddress) as libc::c_uint &&
           0. ==
               Cvar_VariableValue(b"g_synchronousClients\x00" as *const u8 as
                                      *const libc::c_char) {
        Com_Printf(b"^3WARNING: You should set \'g_synchronousClients 1\' for smoother demo recording\n\x00"
                       as *const u8 as *const libc::c_char);
    }
    if Cmd_Argc() == 2i32 {
        s = Cmd_Argv(1i32);
        Q_strncpyz(demoName.as_mut_ptr(), s,
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
        if 0 != clc.compat as u64 {
            Com_sprintf(name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as
                            libc::c_ulong as libc::c_int,
                        b"demos/%s.%s%d\x00" as *const u8 as
                            *const libc::c_char, demoName.as_mut_ptr(),
                        b"dm_\x00" as *const u8 as *const libc::c_char,
                        (*com_legacyprotocol).integer);
        } else {
            Com_sprintf(name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as
                            libc::c_ulong as libc::c_int,
                        b"demos/%s.%s%d\x00" as *const u8 as
                            *const libc::c_char, demoName.as_mut_ptr(),
                        b"dm_\x00" as *const u8 as *const libc::c_char,
                        (*com_protocol).integer);
        }
    } else {
        let mut number: libc::c_int = 0;
        number = 0i32;
        while number <= 9999i32 {
            CL_DemoFilename(number, demoName.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 64]>() as
                                libc::c_ulong as libc::c_int);
            if 0 != clc.compat as u64 {
                Com_sprintf(name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 4096]>() as
                                libc::c_ulong as libc::c_int,
                            b"demos/%s.%s%d\x00" as *const u8 as
                                *const libc::c_char, demoName.as_mut_ptr(),
                            b"dm_\x00" as *const u8 as *const libc::c_char,
                            (*com_legacyprotocol).integer);
            } else {
                Com_sprintf(name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 4096]>() as
                                libc::c_ulong as libc::c_int,
                            b"demos/%s.%s%d\x00" as *const u8 as
                                *const libc::c_char, demoName.as_mut_ptr(),
                            b"dm_\x00" as *const u8 as *const libc::c_char,
                            (*com_protocol).integer);
            }
            if 0 == FS_FileExists(name.as_mut_ptr()) as u64 {
                // file doesn't exist
                break ;
            } else { number += 1 }
        }
    }
    Com_Printf(b"recording to %s.\n\x00" as *const u8 as *const libc::c_char,
               name.as_mut_ptr());
    clc.demofile = FS_FOpenFileWrite(name.as_mut_ptr());
    if 0 == clc.demofile {
        Com_Printf(b"ERROR: couldn\'t open.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    clc.demorecording = qtrue;
    if 0. !=
           Cvar_VariableValue(b"ui_recordSPDemo\x00" as *const u8 as
                                  *const libc::c_char) {
        clc.spDemoRecording = qtrue
    } else { clc.spDemoRecording = qfalse }
    Q_strncpyz(clc.demoName.as_mut_ptr(), demoName.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    clc.demowaiting = qtrue;
    MSG_Init(&mut buf, bufData.as_mut_ptr(),
             ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                 libc::c_int);
    MSG_Bitstream(&mut buf);
    MSG_WriteLong(&mut buf, clc.reliableSequence);
    MSG_WriteByte(&mut buf, svc_gamestate as libc::c_int);
    MSG_WriteLong(&mut buf, clc.serverCommandSequence);
    i = 0i32;
    while i < 1024i32 {
        if !(0 == cl.gameState.stringOffsets[i as usize]) {
            s =
                cl.gameState.stringData.as_mut_ptr().offset(cl.gameState.stringOffsets[i
                                                                                           as
                                                                                           usize]
                                                                as isize);
            MSG_WriteByte(&mut buf, svc_configstring as libc::c_int);
            MSG_WriteShort(&mut buf, i);
            MSG_WriteBigString(&mut buf, s);
        }
        i += 1
    }
    memset(&mut nullstate as *mut entityState_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<entityState_t>() as libc::c_ulong);
    i = 0i32;
    while i < 1i32 << 10i32 {
        ent =
            &mut *cl.entityBaselines.as_mut_ptr().offset(i as isize) as
                *mut entityState_t;
        if !(0 == (*ent).number) {
            MSG_WriteByte(&mut buf, svc_baseline as libc::c_int);
            MSG_WriteDeltaEntity(&mut buf, &mut nullstate, ent, qtrue);
        }
        i += 1
    }
    MSG_WriteByte(&mut buf, svc_EOF as libc::c_int);
    MSG_WriteLong(&mut buf, clc.clientNum);
    MSG_WriteLong(&mut buf, clc.checksumFeed);
    MSG_WriteByte(&mut buf, svc_EOF as libc::c_int);
    len = clc.serverMessageSequence - 1i32;
    FS_Write(&mut len as *mut libc::c_int as *const libc::c_void, 4i32,
             clc.demofile);
    len = buf.cursize;
    FS_Write(&mut len as *mut libc::c_int as *const libc::c_void, 4i32,
             clc.demofile);
    FS_Write(buf.data as *const libc::c_void, buf.cursize, clc.demofile);
}
/*
====================
CL_Record_f

record <demoname>

Begins recording a demo from the current position
====================
*/
// compiler bug workaround
static mut demoName: [libc::c_char; 64] = [0; 64];
/* 
================== 
CL_DemoFilename
================== 
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoFilename(mut number: libc::c_int,
                                         mut fileName: *mut libc::c_char,
                                         mut fileNameSize: libc::c_int) {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    if number < 0i32 || number > 9999i32 { number = 9999i32 }
    a = number / 1000i32;
    number -= a * 1000i32;
    b = number / 100i32;
    number -= b * 100i32;
    c = number / 10i32;
    number -= c * 10i32;
    d = number;
    Com_sprintf(fileName, fileNameSize,
                b"demo%i%i%i%i\x00" as *const u8 as *const libc::c_char, a, b,
                c, d);
}
#[no_mangle]
pub unsafe extern "C" fn CL_Disconnect_f() {
    SCR_StopCinematic();
    Cvar_Set(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    if clc.state as libc::c_uint !=
           CA_DISCONNECTED as libc::c_int as libc::c_uint &&
           clc.state as libc::c_uint !=
               CA_CINEMATIC as libc::c_int as libc::c_uint {
        Com_Error(ERR_DISCONNECT as libc::c_int,
                  b"Disconnected from server\x00" as *const u8 as
                      *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_Vid_Restart_f() {
    if 0 != CL_VideoRecording() as u64 { CL_CloseAVI(); }
    if 0 != clc.demorecording as u64 { CL_StopRecord_f(); }
    S_StopAllSounds();
    if 0 == FS_ConditionalRestart(clc.checksumFeed, qtrue) as u64 {
        if 0 != (*com_sv_running).integer {
            Hunk_ClearToMark();
        } else { Hunk_Clear(); }
        CL_ShutdownUI();
        CL_ShutdownCGame();
        CL_ShutdownRef();
        CL_ResetPureClientAtServer();
        FS_ClearPakReferences(0x2i32 | 0x4i32);
        cls.rendererStarted = qfalse;
        cls.uiStarted = qfalse;
        cls.cgameStarted = qfalse;
        cls.soundRegistered = qfalse;
        Cvar_Set(b"cl_paused\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
        CL_InitRef();
        CL_StartHunkUsers(qfalse);
        if clc.state as libc::c_uint >
               CA_CONNECTED as libc::c_int as libc::c_uint &&
               clc.state as libc::c_uint !=
                   CA_CINEMATIC as libc::c_int as libc::c_uint {
            cls.cgameStarted = qtrue;
            CL_InitCGame();
            CL_SendPureChecksums();
        }
    };
}
/*
=================
CL_SendPureChecksums
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SendPureChecksums() {
    let mut cMsg: [libc::c_char; 1024] = [0; 1024];
    Com_sprintf(cMsg.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"cp %d %s\x00" as *const u8 as *const libc::c_char,
                cl.serverId, FS_ReferencedPakPureChecksums());
    CL_AddReliableCommand(cMsg.as_mut_ptr(), qfalse);
}
// initialize renderer interface
#[no_mangle]
pub unsafe extern "C" fn CL_StartHunkUsers(mut rendererOnly: qboolean) {
    if com_cl_running.is_null() { return }
    if 0 == (*com_cl_running).integer { return }
    if 0 == cls.rendererStarted as u64 {
        cls.rendererStarted = qtrue;
        CL_InitRenderer();
    }
    if 0 != rendererOnly as u64 { return }
    if 0 == cls.soundStarted as u64 { cls.soundStarted = qtrue; S_Init(); }
    if 0 == cls.soundRegistered as u64 {
        cls.soundRegistered = qtrue;
        S_BeginRegistration();
    }
    if 0 != (*com_dedicated).integer { return }
    if 0 == cls.uiStarted as u64 { cls.uiStarted = qtrue; CL_InitUI(); };
}
/*
============
CL_InitRenderer
============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitRenderer() {
    re.BeginRegistration.expect("non-null function pointer")(&mut cls.glconfig);
    cls.charSetShader =
        re.RegisterShader.expect("non-null function pointer")(b"gfx/2d/bigchars\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
    cls.whiteShader =
        re.RegisterShader.expect("non-null function pointer")(b"white\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char);
    cls.consoleShader =
        re.RegisterShader.expect("non-null function pointer")(b"console\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
    g_console_field_width = cls.glconfig.vidWidth / 8i32 - 2i32;
    g_consoleField.widthInChars = g_console_field_width;
}
/*
=================
CL_ResetPureClientAtServer
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ResetPureClientAtServer() {
    CL_AddReliableCommand(b"vdr\x00" as *const u8 as *const libc::c_char,
                          qfalse);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ShutdownRef() {
    if re.Shutdown.is_some() {
        re.Shutdown.expect("non-null function pointer")(qtrue);
    }
    memset(&mut re as *mut refexport_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<refexport_t>() as libc::c_ulong);
    if !rendererLib.is_null() {
        SDL_UnloadObject(rendererLib);
        rendererLib = 0 as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_Snd_Restart_f() {
    CL_Snd_Shutdown();
    CL_Vid_Restart_f();
}
// start all the client stuff using the hunk
#[no_mangle]
pub unsafe extern "C" fn CL_Snd_Shutdown() {
    S_Shutdown();
    cls.soundStarted = qfalse;
}
/*
==============
CL_Clientinfo_f
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Clientinfo_f() {
    Com_Printf(b"--------- Client Information ---------\n\x00" as *const u8 as
                   *const libc::c_char);
    Com_Printf(b"state: %i\n\x00" as *const u8 as *const libc::c_char,
               clc.state as libc::c_uint);
    Com_Printf(b"Server: %s\n\x00" as *const u8 as *const libc::c_char,
               clc.servername.as_mut_ptr());
    Com_Printf(b"User info settings:\n\x00" as *const u8 as
                   *const libc::c_char);
    Info_Print(Cvar_InfoString(0x2i32));
    Com_Printf(b"--------------------------------------\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
==================
CL_Configstrings_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Configstrings_f() {
    let mut i: libc::c_int = 0;
    let mut ofs: libc::c_int = 0;
    if clc.state as libc::c_uint != CA_ACTIVE as libc::c_int as libc::c_uint {
        Com_Printf(b"Not connected to a server.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = 0i32;
    while i < 1024i32 {
        ofs = cl.gameState.stringOffsets[i as usize];
        if !(0 == ofs) {
            Com_Printf(b"%4i: %s\n\x00" as *const u8 as *const libc::c_char,
                       i,
                       cl.gameState.stringData.as_mut_ptr().offset(ofs as
                                                                       isize));
        }
        i += 1
    };
}
/*
======================================================================

CONSOLE COMMANDS

======================================================================
*/
/*
==================
CL_ForwardToServer_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ForwardToServer_f() {
    if clc.state as libc::c_uint != CA_ACTIVE as libc::c_int as libc::c_uint
           || 0 != clc.demoplaying as libc::c_uint {
        Com_Printf(b"Not connected to a server.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() > 1i32 { CL_AddReliableCommand(Cmd_Args(), qfalse); };
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
// cl_main.c  -- client main loop
#[no_mangle]
pub static mut cl_voipProtocol: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_voipShowMeter: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_mumbleScale: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_consoleKeys: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_lanForcePackets: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_motdString: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_up_axis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_side_axis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_forward_axis: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_yaw_axis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_pitch_axis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_up: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_side: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_forward: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_yaw: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut j_pitch: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_filter: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_side: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_forward: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_yaw: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_pitch: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_serverStatusResendTime: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_inGameVideo: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_conXOffset: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_allowDownload: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_showMouseRate: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_mouseAccelOffset: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_mouseAccelStyle: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_freelook: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_mouseAccel: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_sensitivity: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_packetdup: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_maxpackets: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_forceavidemo: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_aviMotionJpeg: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_aviFrameRate: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_autoRecordDemo: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_activeAction: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_freezeDemo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_showTimeDelta: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_showSend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_shownet: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_timeNudge: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_timeout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_noprint: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CL_Shutdown(mut finalmsg: *mut libc::c_char,
                                     mut disconnect: qboolean,
                                     mut quit: qboolean) {
    static mut recursive: qboolean = qfalse;
    if !(!com_cl_running.is_null() && 0 != (*com_cl_running).integer) {
        return
    }
    Com_Printf(b"----- Client Shutdown (%s) -----\n\x00" as *const u8 as
                   *const libc::c_char, finalmsg);
    if 0 != recursive as u64 {
        Com_Printf(b"WARNING: Recursive shutdown\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    recursive = qtrue;
    noGameRestart = quit as libc::c_int;
    if 0 != disconnect as u64 { CL_Disconnect(qtrue); }
    CL_ClearMemory(qtrue);
    CL_Snd_Shutdown();
    Cmd_RemoveCommand(b"cmd\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"configstrings\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"clientinfo\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"snd_restart\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"vid_restart\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"disconnect\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"record\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"demo\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"cinematic\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"stoprecord\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"connect\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"reconnect\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"localservers\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"globalservers\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"rcon\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"ping\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"serverstatus\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"showip\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"fs_openedList\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"fs_referencedList\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"model\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"video\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"stopvideo\x00" as *const u8 as *const libc::c_char);
    CL_ShutdownInput();
    Con_Shutdown();
    Cvar_Set(b"cl_running\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    recursive = qfalse;
    memset(&mut cls as *mut clientStatic_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<clientStatic_t>() as libc::c_ulong);
    Key_SetCatcher(0i32);
    Com_Printf(b"-----------------------\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
=================
CL_ClearMemory

Called by Com_GameRestart
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearMemory(mut shutdownRef: qboolean) {
    CL_ShutdownAll(shutdownRef);
    if com_sv_running.is_null() || 0 == (*com_sv_running).integer {
        Hunk_Clear();
        CM_ClearMap();
    } else { Hunk_ClearToMark(); };
}
// dump all memory on an error
#[no_mangle]
pub unsafe extern "C" fn CL_ShutdownAll(mut shutdownRef: qboolean) {
    if 0 != CL_VideoRecording() as u64 { CL_CloseAVI(); }
    if 0 != clc.demorecording as u64 { CL_StopRecord_f(); }
    CL_cURL_Shutdown();
    S_DisableSounds();
    CL_ShutdownCGame();
    CL_ShutdownUI();
    if 0 != shutdownRef as u64 {
        CL_ShutdownRef();
    } else if re.Shutdown.is_some() {
        re.Shutdown.expect("non-null function pointer")(qfalse);
    }
    cls.uiStarted = qfalse;
    cls.cgameStarted = qfalse;
    cls.rendererStarted = qfalse;
    cls.soundRegistered = qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn CL_Frame(mut msec: libc::c_int) {
    if 0 == (*com_cl_running).integer { return }
    if !clc.downloadCURLM.is_null() {
        CL_cURL_PerformDownload();
        if 0 != clc.cURLDisconnected as u64 {
            cls.realFrametime = msec;
            cls.frametime = msec;
            cls.realtime += cls.frametime;
            SCR_UpdateScreen();
            S_Update();
            Con_RunConsole();
            cls.framecount += 1;
            return
        }
    }
    if 0 != cls.cddialog as u64 {
        cls.cddialog = qfalse;
        VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                UIMENU_NEED_CD as libc::c_int);
    } else if clc.state as libc::c_uint ==
                  CA_DISCONNECTED as libc::c_int as libc::c_uint &&
                  0 == Key_GetCatcher() & 0x2i32 &&
                  0 == (*com_sv_running).integer && !uivm.is_null() {
        S_StopAllSounds();
        VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                UIMENU_MAIN as libc::c_int);
    }
    if 0 != CL_VideoRecording() as libc::c_uint &&
           0 != (*cl_aviFrameRate).integer && 0 != msec {
        if clc.state as libc::c_uint ==
               CA_ACTIVE as libc::c_int as libc::c_uint ||
               0 != (*cl_forceavidemo).integer {
            let mut fps: libc::c_float =
                if (*cl_aviFrameRate).value * (*com_timescale).value <
                       1000.0f32 {
                    (*cl_aviFrameRate).value * (*com_timescale).value
                } else { 1000.0f32 };
            let mut frameDuration: libc::c_float =
                if 1000.0f32 / fps > 1.0f32 {
                    1000.0f32 / fps
                } else { 1.0f32 } + clc.aviVideoFrameRemainder;
            CL_TakeVideoFrame();
            msec = frameDuration as libc::c_int;
            clc.aviVideoFrameRemainder = frameDuration - msec as libc::c_float
        }
    }
    if 0 != (*cl_autoRecordDemo).integer {
        if clc.state as libc::c_uint ==
               CA_ACTIVE as libc::c_int as libc::c_uint &&
               0 == clc.demorecording as u64 && 0 == clc.demoplaying as u64 {
            let mut now: qtime_t =
                qtime_s{tm_sec: 0,
                        tm_min: 0,
                        tm_hour: 0,
                        tm_mday: 0,
                        tm_mon: 0,
                        tm_year: 0,
                        tm_wday: 0,
                        tm_yday: 0,
                        tm_isdst: 0,};
            let mut nowString: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut mapName: [libc::c_char; 64] = [0; 64];
            let mut serverName: [libc::c_char; 4096] = [0; 4096];
            Com_RealTime(&mut now);
            nowString =
                va(b"%04d%02d%02d%02d%02d%02d\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
                   1900i32 + now.tm_year, 1i32 + now.tm_mon, now.tm_mday,
                   now.tm_hour, now.tm_min, now.tm_sec);
            Q_strncpyz(serverName.as_mut_ptr(), clc.servername.as_mut_ptr(),
                       4096i32);
            p =
                strstr(serverName.as_mut_ptr(),
                       b":\x00" as *const u8 as *const libc::c_char);
            if !p.is_null() { *p = '.' as i32 as libc::c_char }
            Q_strncpyz(mapName.as_mut_ptr(),
                       COM_SkipPath(cl.mapname.as_mut_ptr()),
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong as libc::c_int);
            COM_StripExtension(mapName.as_mut_ptr(), mapName.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 64]>() as
                                   libc::c_ulong as libc::c_int);
            Cbuf_ExecuteText(EXEC_NOW as libc::c_int,
                             va(b"record %s-%s-%s\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                nowString, serverName.as_mut_ptr(),
                                mapName.as_mut_ptr()));
        } else if clc.state as libc::c_uint !=
                      CA_ACTIVE as libc::c_int as libc::c_uint &&
                      0 != clc.demorecording as libc::c_uint {
            CL_StopRecord_f();
        }
    }
    cls.realFrametime = msec;
    cls.frametime = msec;
    cls.realtime += cls.frametime;
    if 0 != (*cl_timegraph).integer {
        SCR_DebugGraph((cls.realFrametime as libc::c_double * 0.25f64) as
                           libc::c_float);
    }
    CL_CheckUserinfo();
    CL_CheckTimeout();
    CL_SendCmd();
    CL_CheckForResend();
    CL_SetCGameTime();
    SCR_UpdateScreen();
    S_Update();
    CL_CaptureVoip();
    CL_UpdateMumble();
    SCR_RunCinematic();
    Con_RunConsole();
    cls.framecount += 1;
}
unsafe extern "C" fn CL_UpdateMumble() {
    let mut pos: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut scale: libc::c_float = (*cl_mumbleScale).value;
    let mut tmp: libc::c_float = 0.;
    if 0 == (*cl_useMumble).integer { return }
    AngleVectors(cl.snap.ps.viewangles.as_mut_ptr() as *const vec_t,
                 forward.as_mut_ptr(), 0 as *mut vec_t, up.as_mut_ptr());
    pos[0usize] = cl.snap.ps.origin[0usize] * scale;
    pos[1usize] = cl.snap.ps.origin[2usize] * scale;
    pos[2usize] = cl.snap.ps.origin[1usize] * scale;
    tmp = forward[1usize];
    forward[1usize] = forward[2usize];
    forward[2usize] = tmp;
    tmp = up[1usize];
    up[1usize] = up[2usize];
    up[2usize] = tmp;
    if (*cl_useMumble).integer > 1i32 {
        fprintf(stderr,
                b"%f %f %f, %f %f %f, %f %f %f\n\x00" as *const u8 as
                    *const libc::c_char, pos[0usize] as libc::c_double,
                pos[1usize] as libc::c_double, pos[2usize] as libc::c_double,
                forward[0usize] as libc::c_double,
                forward[1usize] as libc::c_double,
                forward[2usize] as libc::c_double,
                up[0usize] as libc::c_double, up[1usize] as libc::c_double,
                up[2usize] as libc::c_double);
    }
    mumble_update_coordinates(pos.as_mut_ptr(), forward.as_mut_ptr(),
                              up.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CL_CheckForResend() {
    let mut port: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut data: [libc::c_char; 1034] = [0; 1034];
    if 0 != clc.demoplaying as u64 { return }
    if clc.state as libc::c_uint !=
           CA_CONNECTING as libc::c_int as libc::c_uint &&
           clc.state as libc::c_uint !=
               CA_CHALLENGING as libc::c_int as libc::c_uint {
        return
    }
    if cls.realtime - clc.connectTime < 3000i32 { return }
    clc.connectTime = cls.realtime;
    clc.connectPacketCount += 1;
    match clc.state as libc::c_uint {
        3 => {
            if 0 == (*com_standalone).integer &&
                   clc.serverAddress.type_0 as libc::c_uint ==
                       NA_IP as libc::c_int as libc::c_uint &&
                   0 == Sys_IsLANAddress(clc.serverAddress) as u64 {
                CL_RequestAuthorization();
            }
            Com_sprintf(data.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1034]>() as
                            libc::c_ulong as libc::c_int,
                        b"getchallenge %d %s\x00" as *const u8 as
                            *const libc::c_char, clc.challenge,
                        (*com_gamename).string);
            NET_OutOfBandPrint(NS_CLIENT, clc.serverAddress,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               data.as_mut_ptr());
        }
        4 => {
            port =
                Cvar_VariableValue(b"net_qport\x00" as *const u8 as
                                       *const libc::c_char) as libc::c_int;
            Q_strncpyz(info.as_mut_ptr(), Cvar_InfoString(0x2i32),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong as libc::c_int);
            if (*com_legacyprotocol).integer == (*com_protocol).integer {
                clc.compat = qtrue
            }
            if 0 != clc.compat as u64 {
                Info_SetValueForKey(info.as_mut_ptr(),
                                    b"protocol\x00" as *const u8 as
                                        *const libc::c_char,
                                    va(b"%i\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char,
                                       (*com_legacyprotocol).integer));
            } else {
                Info_SetValueForKey(info.as_mut_ptr(),
                                    b"protocol\x00" as *const u8 as
                                        *const libc::c_char,
                                    va(b"%i\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char,
                                       (*com_protocol).integer));
            }
            Info_SetValueForKey(info.as_mut_ptr(),
                                b"qport\x00" as *const u8 as
                                    *const libc::c_char,
                                va(b"%i\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, port));
            Info_SetValueForKey(info.as_mut_ptr(),
                                b"challenge\x00" as *const u8 as
                                    *const libc::c_char,
                                va(b"%i\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, clc.challenge));
            Com_sprintf(data.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1034]>() as
                            libc::c_ulong as libc::c_int,
                        b"connect \"%s\"\x00" as *const u8 as
                            *const libc::c_char, info.as_mut_ptr());
            NET_OutOfBandData(NS_CLIENT, clc.serverAddress,
                              data.as_mut_ptr() as *mut byte,
                              strlen(data.as_mut_ptr()) as libc::c_int);
            cvar_modifiedFlags &= !0x2i32
        }
        _ => {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"CL_CheckForResend: bad clc.state\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
/*
===================
CL_RequestAuthorization

Authorization server protocol
-----------------------------

All commands are text in Q3 out of band packets (leading 0xff 0xff 0xff 0xff).

Whenever the client tries to get a challenge from the server it wants to
connect to, it also blindly fires off a packet to the authorize server:

getKeyAuthorize <challenge> <cdkey>

cdkey may be "demo"


#OLD The authorize server returns a:
#OLD 
#OLD keyAthorize <challenge> <accept | deny>
#OLD 
#OLD A client will be accepted if the cdkey is valid and it has not been used by any other IP
#OLD address in the last 15 minutes.


The server sends a:

getIpAuthorize <challenge> <ip>

The authorize server returns a:

ipAuthorize <challenge> <accept | deny | demo | unknown >

A client will be accepted if a valid cdkey was sent by that ip (only) in the last 15 minutes.
If no response is received from the authorize server after two tries, the client will be let
in anyway.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RequestAuthorization() {
    let mut nums: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut fs: *mut cvar_t = 0 as *mut cvar_t;
    if 0 == cls.authorizeServer.port {
        Com_Printf(b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
                   b"authorize.quake3arena.com\x00" as *const u8 as
                       *const libc::c_char);
        if 0 ==
               NET_StringToAdr(b"authorize.quake3arena.com\x00" as *const u8
                                   as *const libc::c_char,
                               &mut cls.authorizeServer, NA_IP) {
            Com_Printf(b"Couldn\'t resolve address\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
        cls.authorizeServer.port =
            ShortSwap(27952i32 as libc::c_short) as libc::c_ushort;
        Com_Printf(b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"authorize.quake3arena.com\x00" as *const u8 as
                       *const libc::c_char,
                   cls.authorizeServer.ip[0usize] as libc::c_int,
                   cls.authorizeServer.ip[1usize] as libc::c_int,
                   cls.authorizeServer.ip[2usize] as libc::c_int,
                   cls.authorizeServer.ip[3usize] as libc::c_int,
                   ShortSwap(cls.authorizeServer.port as libc::c_short) as
                       libc::c_int);
    }
    if cls.authorizeServer.type_0 as libc::c_uint ==
           NA_BAD as libc::c_int as libc::c_uint {
        return
    }
    j = 0i32;
    l = strlen(cl_cdkey.as_mut_ptr()) as libc::c_int;
    if l > 32i32 { l = 32i32 }
    i = 0i32;
    while i < l {
        if cl_cdkey[i as usize] as libc::c_int >= '0' as i32 &&
               cl_cdkey[i as usize] as libc::c_int <= '9' as i32 ||
               cl_cdkey[i as usize] as libc::c_int >= 'a' as i32 &&
                   cl_cdkey[i as usize] as libc::c_int <= 'z' as i32 ||
               cl_cdkey[i as usize] as libc::c_int >= 'A' as i32 &&
                   cl_cdkey[i as usize] as libc::c_int <= 'Z' as i32 {
            nums[j as usize] = cl_cdkey[i as usize];
            j += 1
        }
        i += 1
    }
    nums[j as usize] = 0i32 as libc::c_char;
    fs =
        Cvar_Get(b"cl_anonymous\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x10i32 | 0x8i32);
    NET_OutOfBandPrint(NS_CLIENT, cls.authorizeServer,
                       b"getKeyAuthorize %i %s\x00" as *const u8 as
                           *const libc::c_char, (*fs).integer,
                       nums.as_mut_ptr());
}
/*
==================
CL_CheckTimeout

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CheckTimeout() {
    if (0 == CL_CheckPaused() as u64 || 0 == (*sv_paused).integer) &&
           clc.state as libc::c_uint >=
               CA_CONNECTED as libc::c_int as libc::c_uint &&
           clc.state as libc::c_uint !=
               CA_CINEMATIC as libc::c_int as libc::c_uint &&
           (cls.realtime - clc.lastPacketTime) as libc::c_float >
               (*cl_timeout).value * 1000i32 as libc::c_float {
        cl.timeoutcount += 1;
        if cl.timeoutcount > 5i32 {
            Com_Printf(b"\nServer connection timed out.\n\x00" as *const u8 as
                           *const libc::c_char);
            CL_Disconnect(qtrue);
            return
        }
    } else { cl.timeoutcount = 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn CL_CheckPaused() -> qboolean {
    if 0 != (*cl_paused).integer || 0 != (*cl_paused).modified as libc::c_uint
       {
        return qtrue
    }
    return qfalse;
}
//============================================================================
/*
==================
CL_CheckUserinfo

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CheckUserinfo() {
    if (clc.state as libc::c_uint) <
           CA_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    if 0 != CL_CheckPaused() as u64 { return }
    if 0 != cvar_modifiedFlags & 0x2i32 {
        cvar_modifiedFlags &= !0x2i32;
        CL_AddReliableCommand(va(b"userinfo \"%s\"\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 Cvar_InfoString(0x2i32)), qfalse);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_PacketEvent(mut from: netadr_t,
                                        mut msg: *mut msg_t) {
    let mut headerBytes: libc::c_int = 0;
    clc.lastPacketTime = cls.realtime;
    if (*msg).cursize >= 4i32 && *((*msg).data as *mut libc::c_int) == -1i32 {
        CL_ConnectionlessPacket(from, msg);
        return
    }
    if (clc.state as libc::c_uint) <
           CA_CONNECTED as libc::c_int as libc::c_uint {
        return
    }
    if (*msg).cursize < 4i32 {
        Com_Printf(b"%s: Runt packet\n\x00" as *const u8 as
                       *const libc::c_char, NET_AdrToStringwPort(from));
        return
    }
    if 0 == NET_CompareAdr(from, clc.netchan.remoteAddress) as u64 {
        Com_DPrintf(b"%s:sequenced packet without connection\n\x00" as
                        *const u8 as *const libc::c_char,
                    NET_AdrToStringwPort(from));
        return
    }
    if 0 == CL_Netchan_Process(&mut clc.netchan, msg) as u64 { return }
    headerBytes = (*msg).readcount;
    clc.serverMessageSequence = *((*msg).data as *mut libc::c_int);
    clc.lastPacketTime = cls.realtime;
    CL_ParseServerMessage(msg);
    if 0 != clc.demorecording as libc::c_uint && 0 == clc.demowaiting as u64 {
        CL_WriteDemoMessage(msg, headerBytes);
    };
}
//
// cl_main.c
//
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoMessage(mut msg: *mut msg_t,
                                             mut headerBytes: libc::c_int) {
    let mut len: libc::c_int = 0;
    let mut swlen: libc::c_int = 0;
    len = clc.serverMessageSequence;
    swlen = len;
    FS_Write(&mut swlen as *mut libc::c_int as *const libc::c_void, 4i32,
             clc.demofile);
    len = (*msg).cursize - headerBytes;
    swlen = len;
    FS_Write(&mut swlen as *mut libc::c_int as *const libc::c_void, 4i32,
             clc.demofile);
    FS_Write((*msg).data.offset(headerBytes as isize) as *const libc::c_void,
             len, clc.demofile);
}
/*
=================
CL_ConnectionlessPacket

Responses to broadcasts, etc
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ConnectionlessPacket(mut from: netadr_t,
                                                 mut msg: *mut msg_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut challenge: libc::c_int = 0i32;
    MSG_BeginReadingOOB(msg);
    MSG_ReadLong(msg);
    s = MSG_ReadStringLine(msg);
    Cmd_TokenizeString(s);
    c = Cmd_Argv(0i32);
    Com_DPrintf(b"CL packet %s: %s\n\x00" as *const u8 as *const libc::c_char,
                NET_AdrToStringwPort(from), c);
    if 0 ==
           Q_stricmp(c,
                     b"challengeResponse\x00" as *const u8 as
                         *const libc::c_char) {
        let mut strver: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ver: libc::c_int = 0;
        if clc.state as libc::c_uint !=
               CA_CONNECTING as libc::c_int as libc::c_uint {
            Com_DPrintf(b"Unwanted challenge response received. Ignored.\n\x00"
                            as *const u8 as *const libc::c_char);
            return
        }
        c = Cmd_Argv(2i32);
        if 0 != *c { challenge = atoi(c) }
        strver = Cmd_Argv(3i32);
        if 0 != *strver {
            ver = atoi(strver);
            if ver != (*com_protocol).integer {
                if (*com_legacyprotocol).integer > 0i32 {
                    clc.compat = qtrue;
                    Com_Printf(b"^3Warning: Server reports protocol version %d, we have %d. Trying legacy protocol %d.\n\x00"
                                   as *const u8 as *const libc::c_char, ver,
                               (*com_protocol).integer,
                               (*com_legacyprotocol).integer);
                } else {
                    Com_Printf(b"^3Warning: Server reports protocol version %d, we have %d. Trying anyways.\n\x00"
                                   as *const u8 as *const libc::c_char, ver,
                               (*com_protocol).integer);
                }
            }
        } else { clc.compat = qtrue }
        if 0 != clc.compat as u64 {
            if 0 == NET_CompareAdr(from, clc.serverAddress) as u64 {
                if 0 == *c || challenge != clc.challenge {
                    Com_DPrintf(b"Challenge response received from unexpected source. Ignored.\n\x00"
                                    as *const u8 as *const libc::c_char);
                    return
                }
            }
        } else if 0 == *c || challenge != clc.challenge {
            Com_Printf(b"Bad challenge for challengeResponse. Ignored.\n\x00"
                           as *const u8 as *const libc::c_char);
            return
        }
        clc.challenge = atoi(Cmd_Argv(1i32));
        clc.state = CA_CHALLENGING;
        clc.connectPacketCount = 0i32;
        clc.connectTime = -99999i32;
        clc.serverAddress = from;
        Com_DPrintf(b"challengeResponse: %d\n\x00" as *const u8 as
                        *const libc::c_char, clc.challenge);
        return
    }
    if 0 ==
           Q_stricmp(c,
                     b"connectResponse\x00" as *const u8 as
                         *const libc::c_char) {
        if clc.state as libc::c_uint >=
               CA_CONNECTED as libc::c_int as libc::c_uint {
            Com_Printf(b"Dup connect received. Ignored.\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
        if clc.state as libc::c_uint !=
               CA_CHALLENGING as libc::c_int as libc::c_uint {
            Com_Printf(b"connectResponse packet while not connecting. Ignored.\n\x00"
                           as *const u8 as *const libc::c_char);
            return
        }
        if 0 == NET_CompareAdr(from, clc.serverAddress) as u64 {
            Com_Printf(b"connectResponse from wrong address. Ignored.\n\x00"
                           as *const u8 as *const libc::c_char);
            return
        }
        if 0 == clc.compat as u64 {
            c = Cmd_Argv(1i32);
            if 0 != *c {
                challenge = atoi(c)
            } else {
                Com_Printf(b"Bad connectResponse received. Ignored.\n\x00" as
                               *const u8 as *const libc::c_char);
                return
            }
            if challenge != clc.challenge {
                Com_Printf(b"ConnectResponse with bad challenge received. Ignored.\n\x00"
                               as *const u8 as *const libc::c_char);
                return
            }
        }
        Netchan_Setup(NS_CLIENT, &mut clc.netchan, from,
                      Cvar_VariableValue(b"net_qport\x00" as *const u8 as
                                             *const libc::c_char) as
                          libc::c_int, clc.challenge, clc.compat);
        clc.state = CA_CONNECTED;
        clc.lastPacketSentTime = -9999i32;
        return
    }
    if 0 ==
           Q_stricmp(c,
                     b"infoResponse\x00" as *const u8 as *const libc::c_char)
       {
        CL_ServerInfoPacket(from, msg);
        return
    }
    if 0 ==
           Q_stricmp(c,
                     b"statusResponse\x00" as *const u8 as
                         *const libc::c_char) {
        CL_ServerStatusResponse(from, msg);
        return
    }
    if 0 == Q_stricmp(c, b"echo\x00" as *const u8 as *const libc::c_char) {
        if 0 != NET_CompareAdr(from, clc.serverAddress) as libc::c_uint ||
               0 != NET_CompareAdr(from, cls.rconAddress) as libc::c_uint {
            NET_OutOfBandPrint(NS_CLIENT, from,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               Cmd_Argv(1i32));
        }
        return
    }
    if 0 ==
           Q_stricmp(c,
                     b"keyAuthorize\x00" as *const u8 as *const libc::c_char)
       {
        return
    }
    if 0 == Q_stricmp(c, b"motd\x00" as *const u8 as *const libc::c_char) {
        CL_MotdPacket(from);
        return
    }
    if 0 == Q_stricmp(c, b"print\x00" as *const u8 as *const libc::c_char) {
        if 0 != NET_CompareAdr(from, clc.serverAddress) as libc::c_uint ||
               0 != NET_CompareAdr(from, cls.rconAddress) as libc::c_uint {
            s = MSG_ReadString(msg);
            Q_strncpyz(clc.serverMessage.as_mut_ptr(), s,
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong as libc::c_int);
            Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char, s);
        }
        return
    }
    if 0 ==
           Q_strncmp(c,
                     b"getserversResponse\x00" as *const u8 as
                         *const libc::c_char, 18i32) {
        CL_ServersResponsePacket(&mut from, msg, qfalse);
        return
    }
    if 0 ==
           Q_strncmp(c,
                     b"getserversExtResponse\x00" as *const u8 as
                         *const libc::c_char, 21i32) {
        CL_ServersResponsePacket(&mut from, msg, qtrue);
        return
    }
    Com_DPrintf(b"Unknown connectionless packet command.\n\x00" as *const u8
                    as *const libc::c_char);
}
/*
===================
CL_ServersResponsePacket
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ServersResponsePacket(mut from: *const netadr_t,
                                                  mut msg: *mut msg_t,
                                                  mut extended: qboolean) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut addresses: [netadr_t; 256] =
        [netadr_t{type_0: NA_BAD,
                  ip: [0; 4],
                  ip6: [0; 16],
                  port: 0,
                  scope_id: 0,}; 256];
    let mut numservers: libc::c_int = 0;
    let mut buffptr: *mut byte = 0 as *mut byte;
    let mut buffend: *mut byte = 0 as *mut byte;
    Com_Printf(b"CL_ServersResponsePacket from %s\n\x00" as *const u8 as
                   *const libc::c_char, NET_AdrToStringwPort(*from));
    if cls.numglobalservers == -1i32 {
        cls.numglobalservers = 0i32;
        cls.numGlobalServerAddresses = 0i32
    }
    numservers = 0i32;
    buffptr = (*msg).data;
    buffend = buffptr.offset((*msg).cursize as isize);
    while !(*buffptr as libc::c_int == '\\' as i32 ||
                0 != extended as libc::c_uint &&
                    *buffptr as libc::c_int == '/' as i32) {
        buffptr = buffptr.offset(1isize);
        if !(buffptr < buffend) { break ; }
    }
    while buffptr.offset(1isize) < buffend {
        // IPv4 address
        if *buffptr as libc::c_int == '\\' as i32 {
            buffptr = buffptr.offset(1isize);
            if (buffend.wrapping_offset_from(buffptr) as libc::c_long as
                    libc::c_ulong) <
                   (::std::mem::size_of::<[byte; 4]>() as
                        libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_ushort>()
                                                        as
                                                        libc::c_ulong).wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_ulong)
               {
                break ;
            }
            i = 0i32;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong {
                let fresh1 = buffptr;
                buffptr = buffptr.offset(1);
                addresses[numservers as usize].ip[i as usize] = *fresh1;
                i += 1
            }
            addresses[numservers as usize].type_0 = NA_IP
        } else if 0 != extended as libc::c_uint &&
                      *buffptr as libc::c_int == '/' as i32 {
            buffptr = buffptr.offset(1isize);
            if (buffend.wrapping_offset_from(buffptr) as libc::c_long as
                    libc::c_ulong) <
                   (::std::mem::size_of::<[byte; 16]>() as
                        libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_ushort>()
                                                        as
                                                        libc::c_ulong).wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_ulong)
               {
                break ;
            }
            i = 0i32;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong {
                let fresh2 = buffptr;
                buffptr = buffptr.offset(1);
                addresses[numservers as usize].ip6[i as usize] = *fresh2;
                i += 1
            }
            addresses[numservers as usize].type_0 = NA_IP6;
            addresses[numservers as usize].scope_id = (*from).scope_id
        } else {
            // syntax error!
            break ;
        }
        let fresh3 = buffptr;
        buffptr = buffptr.offset(1);
        addresses[numservers as usize].port =
            ((*fresh3 as libc::c_int) << 8i32) as libc::c_ushort;
        let fresh4 = buffptr;
        buffptr = buffptr.offset(1);
        addresses[numservers as usize].port =
            (addresses[numservers as usize].port as libc::c_int +
                 *fresh4 as libc::c_int) as libc::c_ushort;
        addresses[numservers as usize].port =
            ShortSwap(addresses[numservers as usize].port as libc::c_short) as
                libc::c_ushort;
        // syntax check
        if *buffptr as libc::c_int != '\\' as i32 &&
               *buffptr as libc::c_int != '/' as i32 {
            break ;
        }
        numservers += 1;
        if numservers >= 256i32 { break ; }
    }
    count = cls.numglobalservers;
    i = 0i32;
    while i < numservers && count < 4096i32 {
        // build net address
        let mut server: *mut serverInfo_t =
            &mut *cls.globalServers.as_mut_ptr().offset(count as isize) as
                *mut serverInfo_t;
        j = 0i32;
        while j < count {
            if 0 !=
                   NET_CompareAdr(cls.globalServers[j as usize].adr,
                                  addresses[i as usize]) as u64 {
                break ;
            }
            j += 1
        }
        if !(j < count) {
            CL_InitServerInfo(server,
                              &mut *addresses.as_mut_ptr().offset(i as
                                                                      isize));
            count += 1
        }
        i += 1
    }
    if count >= 4096i32 && cls.numGlobalServerAddresses < 4096i32 {
        while i < numservers && cls.numGlobalServerAddresses < 4096i32 {
            let fresh5 = cls.numGlobalServerAddresses;
            cls.numGlobalServerAddresses = cls.numGlobalServerAddresses + 1;
            cls.globalServerAddresses[fresh5 as usize] =
                addresses[i as usize];
            i += 1
        }
    }
    cls.numglobalservers = count;
    total = count + cls.numGlobalServerAddresses;
    Com_Printf(b"%d servers parsed (total %d)\n\x00" as *const u8 as
                   *const libc::c_char, numservers, total);
}
/*
===================
CL_InitServerInfo
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitServerInfo(mut server: *mut serverInfo_t,
                                           mut address: *mut netadr_t) {
    (*server).adr = *address;
    (*server).clients = 0i32;
    (*server).hostName[0usize] = '\u{0}' as i32 as libc::c_char;
    (*server).mapName[0usize] = '\u{0}' as i32 as libc::c_char;
    (*server).maxClients = 0i32;
    (*server).maxPing = 0i32;
    (*server).minPing = 0i32;
    (*server).ping = -1i32;
    (*server).game[0usize] = '\u{0}' as i32 as libc::c_char;
    (*server).gameType = 0i32;
    (*server).netType = 0i32;
    (*server).punkbuster = 0i32;
    (*server).g_humanplayers = 0i32;
    (*server).g_needpass = 0i32;
}
/*
===================
CL_MotdPacket

===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_MotdPacket(mut from: netadr_t) {
    let mut challenge: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == NET_CompareAdr(from, cls.updateServer) as u64 { return }
    info = Cmd_Argv(1i32);
    challenge =
        Info_ValueForKey(info,
                         b"challenge\x00" as *const u8 as
                             *const libc::c_char);
    if 0 != strcmp(challenge, cls.updateChallenge.as_mut_ptr()) { return }
    challenge =
        Info_ValueForKey(info,
                         b"motd\x00" as *const u8 as *const libc::c_char);
    Q_strncpyz(cls.updateInfoString.as_mut_ptr(), info,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    Cvar_Set(b"cl_motdString\x00" as *const u8 as *const libc::c_char,
             challenge);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ServerStatusResponse(mut from: netadr_t,
                                                 mut msg: *mut msg_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut ping: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut serverStatus: *mut serverStatus_t = 0 as *mut serverStatus_t;
    serverStatus = 0 as *mut serverStatus_t;
    i = 0i32;
    while i < 16i32 {
        if 0 !=
               NET_CompareAdr(from, cl_serverStatusList[i as usize].address)
                   as u64 {
            serverStatus =
                &mut *cl_serverStatusList.as_mut_ptr().offset(i as isize) as
                    *mut serverStatus_t;
            break ;
        } else { i += 1 }
    }
    if serverStatus.is_null() { return }
    s = MSG_ReadStringLine(msg);
    len = 0i32;
    Com_sprintf(&mut *(*serverStatus).string.as_mut_ptr().offset(len as isize)
                    as *mut libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong) as
                    libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char, s);
    if 0 != (*serverStatus).print as u64 {
        Com_Printf(b"Server settings:\n\x00" as *const u8 as
                       *const libc::c_char);
        while 0 != *s {
            i = 0i32;
            while i < 2i32 && 0 != *s as libc::c_int {
                if *s as libc::c_int == '\\' as i32 { s = s.offset(1isize) }
                l = 0i32;
                while 0 != *s {
                    let fresh6 = l;
                    l = l + 1;
                    info[fresh6 as usize] = *s;
                    if l >= 1024i32 - 1i32 { break ; }
                    s = s.offset(1isize);
                    if *s as libc::c_int == '\\' as i32 { break ; }
                }
                info[l as usize] = '\u{0}' as i32 as libc::c_char;
                if 0 != i {
                    Com_Printf(b"%s\n\x00" as *const u8 as
                                   *const libc::c_char, info.as_mut_ptr());
                } else {
                    Com_Printf(b"%-24s\x00" as *const u8 as
                                   *const libc::c_char, info.as_mut_ptr());
                }
                i += 1
            }
        }
    }
    len = strlen((*serverStatus).string.as_mut_ptr()) as libc::c_int;
    Com_sprintf(&mut *(*serverStatus).string.as_mut_ptr().offset(len as isize)
                    as *mut libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong) as
                    libc::c_int,
                b"\\\x00" as *const u8 as *const libc::c_char);
    if 0 != (*serverStatus).print as u64 {
        Com_Printf(b"\nPlayers:\n\x00" as *const u8 as *const libc::c_char);
        Com_Printf(b"num: score: ping: name:\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    i = 0i32;
    s = MSG_ReadStringLine(msg);
    while 0 != *s {
        len = strlen((*serverStatus).string.as_mut_ptr()) as libc::c_int;
        Com_sprintf(&mut *(*serverStatus).string.as_mut_ptr().offset(len as
                                                                         isize)
                        as *mut libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong).wrapping_sub(len as libc::c_ulong) as
                        libc::c_int,
                    b"\\%s\x00" as *const u8 as *const libc::c_char, s);
        if 0 != (*serverStatus).print as u64 {
            ping = 0i32;
            score = ping;
            sscanf(s, b"%d %d\x00" as *const u8 as *const libc::c_char,
                   &mut score as *mut libc::c_int,
                   &mut ping as *mut libc::c_int);
            s = strchr(s, ' ' as i32);
            if !s.is_null() { s = strchr(s.offset(1isize), ' ' as i32) }
            if !s.is_null() {
                s = s.offset(1isize)
            } else {
                s =
                    b"unknown\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            Com_Printf(b"%-2d   %-3d    %-3d   %s\n\x00" as *const u8 as
                           *const libc::c_char, i, score, ping, s);
        }
        s = MSG_ReadStringLine(msg);
        i += 1
    }
    len = strlen((*serverStatus).string.as_mut_ptr()) as libc::c_int;
    Com_sprintf(&mut *(*serverStatus).string.as_mut_ptr().offset(len as isize)
                    as *mut libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong) as
                    libc::c_int,
                b"\\\x00" as *const u8 as *const libc::c_char);
    (*serverStatus).time = Com_Milliseconds();
    (*serverStatus).address = from;
    (*serverStatus).pending = qfalse;
    if 0 != (*serverStatus).print as u64 {
        (*serverStatus).retrieved = qtrue
    };
}
//====================================================================
#[no_mangle]
pub unsafe extern "C" fn CL_ServerInfoPacket(mut from: netadr_t,
                                             mut msg: *mut msg_t) {
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut infoString: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prot: libc::c_int = 0;
    let mut gamename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gameMismatch: qboolean = qfalse;
    infoString = MSG_ReadString(msg);
    gamename =
        Info_ValueForKey(infoString,
                         b"gamename\x00" as *const u8 as *const libc::c_char);
    if 0 != (*com_legacyprotocol).integer && 0 == *gamename {
        gameMismatch = qfalse
    } else {
        gameMismatch =
            (0 == *gamename ||
                 strcmp(gamename, (*com_gamename).string) != 0i32) as
                libc::c_int as qboolean
    }
    if 0 != gameMismatch as u64 {
        Com_DPrintf(b"Game mismatch in info packet: %s\n\x00" as *const u8 as
                        *const libc::c_char, infoString);
        return
    }
    prot =
        atoi(Info_ValueForKey(infoString,
                              b"protocol\x00" as *const u8 as
                                  *const libc::c_char));
    if prot != (*com_protocol).integer &&
           prot != (*com_legacyprotocol).integer {
        Com_DPrintf(b"Different protocol info packet: %s\n\x00" as *const u8
                        as *const libc::c_char, infoString);
        return
    }
    i = 0i32;
    while i < 32i32 {
        if 0 != cl_pinglist[i as usize].adr.port as libc::c_int &&
               0 == cl_pinglist[i as usize].time &&
               0 !=
                   NET_CompareAdr(from, cl_pinglist[i as usize].adr) as
                       libc::c_uint {
            cl_pinglist[i as usize].time =
                Sys_Milliseconds() - cl_pinglist[i as usize].start;
            Com_DPrintf(b"ping time %dms from %s\n\x00" as *const u8 as
                            *const libc::c_char, cl_pinglist[i as usize].time,
                        NET_AdrToString(from));
            Q_strncpyz(cl_pinglist[i as usize].info.as_mut_ptr(), infoString,
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong as libc::c_int);
            match from.type_0 as libc::c_uint {
                3 | 4 => { type_0 = 1i32 }
                5 => { type_0 = 2i32 }
                _ => { type_0 = 0i32 }
            }
            Info_SetValueForKey(cl_pinglist[i as usize].info.as_mut_ptr(),
                                b"nettype\x00" as *const u8 as
                                    *const libc::c_char,
                                va(b"%d\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, type_0));
            CL_SetServerInfoByAddress(from, infoString,
                                      cl_pinglist[i as usize].time);
            return
        }
        i += 1
    }
    if cls.pingUpdateSource != 0i32 { return }
    i = 0i32;
    while i < 128i32 {
        // empty slot
        if cls.localServers[i as usize].adr.port as libc::c_int == 0i32 {
            break ;
        }
        if 0 != NET_CompareAdr(from, cls.localServers[i as usize].adr) as u64
           {
            return
        }
        i += 1
    }
    if i == 128i32 {
        Com_DPrintf(b"MAX_OTHER_SERVERS hit, dropping infoResponse\n\x00" as
                        *const u8 as *const libc::c_char);
        return
    }
    cls.numlocalservers = i + 1i32;
    CL_InitServerInfo(&mut *cls.localServers.as_mut_ptr().offset(i as isize),
                      &mut from);
    Q_strncpyz(info.as_mut_ptr(), MSG_ReadString(msg), 1024i32);
    if 0 != strlen(info.as_mut_ptr()) {
        if info[strlen(info.as_mut_ptr()).wrapping_sub(1i32 as libc::c_ulong)
                    as usize] as libc::c_int != '\n' as i32 {
            Q_strcat(info.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as libc::c_int,
                     b"\n\x00" as *const u8 as *const libc::c_char);
        }
        Com_Printf(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                   NET_AdrToStringwPort(from), info.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_MapLoading() {
    if 0 != (*com_dedicated).integer {
        clc.state = CA_DISCONNECTED;
        Key_SetCatcher(0x1i32);
        return
    }
    if 0 == (*com_cl_running).integer { return }
    Con_Close();
    Key_SetCatcher(0i32);
    if clc.state as libc::c_uint >=
           CA_CONNECTED as libc::c_int as libc::c_uint &&
           0 ==
               Q_stricmp(clc.servername.as_mut_ptr(),
                         b"localhost\x00" as *const u8 as *const libc::c_char)
       {
        clc.state = CA_CONNECTED;
        memset(cls.updateInfoString.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as
                   libc::c_ulong);
        memset(clc.serverMessage.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as
                   libc::c_ulong);
        memset(&mut cl.gameState as *mut gameState_t as *mut libc::c_void,
               0i32, ::std::mem::size_of::<gameState_t>() as libc::c_ulong);
        clc.lastPacketSentTime = -9999i32;
        SCR_UpdateScreen();
    } else {
        Cvar_Set(b"nextmap\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
        CL_Disconnect(qtrue);
        Q_strncpyz(clc.servername.as_mut_ptr(),
                   b"localhost\x00" as *const u8 as *const libc::c_char,
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong as libc::c_int);
        clc.state = CA_CHALLENGING;
        Key_SetCatcher(0i32);
        SCR_UpdateScreen();
        clc.connectTime = -3000i32;
        NET_StringToAdr(clc.servername.as_mut_ptr(), &mut clc.serverAddress,
                        NA_UNSPEC);
        CL_CheckForResend();
    };
}
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
#[no_mangle]
pub unsafe extern "C" fn CL_ForwardCommandToServer(mut string:
                                                       *const libc::c_char) {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd = Cmd_Argv(0i32);
    if *cmd.offset(0isize) as libc::c_int == '-' as i32 { return }
    if 0 != clc.demoplaying as libc::c_uint ||
           (clc.state as libc::c_uint) <
               CA_CONNECTED as libc::c_int as libc::c_uint ||
           *cmd.offset(0isize) as libc::c_int == '+' as i32 {
        Com_Printf(b"Unknown command \"%s^7\"\n\x00" as *const u8 as
                       *const libc::c_char, cmd);
        return
    }
    if Cmd_Argc() > 1i32 {
        CL_AddReliableCommand(string, qfalse);
    } else { CL_AddReliableCommand(cmd, qfalse); };
}
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
#[no_mangle]
pub unsafe extern "C" fn CL_CDDialog() { cls.cddialog = qtrue; }
// bring up the "need a cd to play" dialog
#[no_mangle]
pub unsafe extern "C" fn CL_FlushMemory() {
    CL_ClearMemory(qfalse);
    CL_StartHunkUsers(qfalse);
}
//
// cvars
//
#[no_mangle]
pub static mut cl_nodelta: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_debugMove: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CL_StartDemoLoop() {
    Cbuf_AddText(b"d1\n\x00" as *const u8 as *const libc::c_char);
    Key_SetCatcher(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn CL_InitDownloads() {
    let mut missingfiles: [libc::c_char; 1024] = [0; 1024];
    if 0 == (*cl_allowDownload).integer & 1i32 {
        if 0 !=
               FS_ComparePaks(missingfiles.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                  libc::c_ulong as libc::c_int, qfalse) as u64
           {
            Com_Printf(b"\nWARNING: You are missing some files referenced by the server:\n%sYou might not be able to join the game\nGo to the setting menu to turn on autodownload, or get the file elsewhere\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       missingfiles.as_mut_ptr());
        }
    } else if 0 !=
                  FS_ComparePaks(clc.downloadList.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 1024]>()
                                     as libc::c_ulong as libc::c_int, qtrue)
                      as u64 {
        Com_Printf(b"Need paks: %s\n\x00" as *const u8 as *const libc::c_char,
                   clc.downloadList.as_mut_ptr());
        if 0 != *clc.downloadList.as_mut_ptr() {
            clc.state = CA_CONNECTED;
            let ref mut fresh7 = *clc.downloadName.as_mut_ptr();
            *fresh7 = 0i32 as libc::c_char;
            *clc.downloadTempName.as_mut_ptr() = *fresh7;
            Cvar_Set(b"cl_downloadName\x00" as *const u8 as
                         *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char);
            CL_NextDownload();
            return
        }
    }
    CL_DownloadsComplete();
}
//====================================================================
/*
=================
CL_DownloadsComplete

Called when all downloading has been completed
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DownloadsComplete() {
    if 0 != clc.cURLUsed as u64 {
        clc.cURLUsed = qfalse;
        CL_cURL_Shutdown();
        if 0 != clc.cURLDisconnected as u64 {
            if 0 != clc.downloadRestart as u64 {
                FS_Restart(clc.checksumFeed);
                clc.downloadRestart = qfalse
            }
            clc.cURLDisconnected = qfalse;
            CL_Reconnect_f();
            return
        }
    }
    if 0 != clc.downloadRestart as u64 {
        clc.downloadRestart = qfalse;
        FS_Restart(clc.checksumFeed);
        CL_AddReliableCommand(b"donedl\x00" as *const u8 as
                                  *const libc::c_char, qfalse);
        return
    }
    clc.state = CA_LOADING;
    Com_EventLoop();
    if clc.state as libc::c_uint != CA_LOADING as libc::c_int as libc::c_uint
       {
        return
    }
    Cvar_Set(b"r_uiFullScreen\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    CL_FlushMemory();
    cls.cgameStarted = qtrue;
    CL_InitCGame();
    CL_SendPureChecksums();
    CL_WritePacket();
    CL_WritePacket();
    CL_WritePacket();
}
#[no_mangle]
pub unsafe extern "C" fn CL_NextDownload() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remoteName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut localName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut useCURL: qboolean = qfalse;
    if 0 != *clc.downloadName.as_mut_ptr() {
        let mut zippath: *mut libc::c_char =
            FS_BuildOSPath(Cvar_VariableString(b"fs_homepath\x00" as *const u8
                                                   as *const libc::c_char),
                           clc.downloadName.as_mut_ptr(),
                           b"\x00" as *const u8 as *const libc::c_char);
        *zippath.offset(strlen(zippath).wrapping_sub(1i32 as libc::c_ulong) as
                            isize) = '\u{0}' as i32 as libc::c_char;
        if 0 == FS_CompareZipChecksum(zippath) as u64 {
            Com_Error(ERR_DROP as libc::c_int,
                      b"Incorrect checksum for file: %s\x00" as *const u8 as
                          *const libc::c_char, clc.downloadName.as_mut_ptr());
        }
    }
    let ref mut fresh8 = *clc.downloadName.as_mut_ptr();
    *fresh8 = 0i32 as libc::c_char;
    *clc.downloadTempName.as_mut_ptr() = *fresh8;
    Cvar_Set(b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char);
    if 0 != *clc.downloadList.as_mut_ptr() {
        s = clc.downloadList.as_mut_ptr();
        if *s as libc::c_int == '@' as i32 { s = s.offset(1isize) }
        remoteName = s;
        s = strchr(s, '@' as i32);
        if s.is_null() { CL_DownloadsComplete(); return }
        let fresh9 = s;
        s = s.offset(1);
        *fresh9 = 0i32 as libc::c_char;
        localName = s;
        s = strchr(s, '@' as i32);
        if !s.is_null() {
            let fresh10 = s;
            s = s.offset(1);
            *fresh10 = 0i32 as libc::c_char
        } else { s = localName.offset(strlen(localName) as isize) }
        if 0 == (*cl_allowDownload).integer & 2i32 {
            if 0 != clc.sv_allowDownload & 2i32 {
                Com_Printf(b"WARNING: server does not allow download redirection (sv_allowDownload is %d)\n\x00"
                               as *const u8 as *const libc::c_char,
                           clc.sv_allowDownload);
            } else if 0 == *clc.sv_dlURL.as_mut_ptr() {
                Com_Printf(b"WARNING: server allows download redirection, but does not have sv_dlURL set\n\x00"
                               as *const u8 as *const libc::c_char);
            } else if 0 == CL_cURL_Init() as u64 {
                Com_Printf(b"WARNING: could not load cURL library\n\x00" as
                               *const u8 as *const libc::c_char);
            } else {
                CL_cURL_BeginDownload(localName,
                                      va(b"%s/%s\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         clc.sv_dlURL.as_mut_ptr(),
                                         remoteName));
                useCURL = qtrue
            }
        } else if 0 == clc.sv_allowDownload & 2i32 {
            Com_Printf(b"WARNING: server allows download redirection, but it disabled by client configuration (cl_allowDownload is %d)\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*cl_allowDownload).integer);
        }
        if 0 == useCURL as u64 {
            if 0 != (*cl_allowDownload).integer & 4i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"UDP Downloads are disabled on your client. (cl_allowDownload is %d)\x00"
                              as *const u8 as *const libc::c_char,
                          (*cl_allowDownload).integer);
            } else { CL_BeginDownload(localName, remoteName); }
        }
        clc.downloadRestart = qtrue;
        memmove(clc.downloadList.as_mut_ptr() as *mut libc::c_void,
                s as *const libc::c_void,
                strlen(s).wrapping_add(1i32 as libc::c_ulong));
        return
    }
    CL_DownloadsComplete();
}
/*
=================
CL_BeginDownload

Requests a file to download from the server.  Stores it in the current
game directory.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_BeginDownload(mut localName: *const libc::c_char,
                                          mut remoteName:
                                              *const libc::c_char) {
    Com_DPrintf(b"***** CL_BeginDownload *****\nLocalname: %s\nRemotename: %s\n****************************\n\x00"
                    as *const u8 as *const libc::c_char, localName,
                remoteName);
    Q_strncpyz(clc.downloadName.as_mut_ptr(), localName,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Com_sprintf(clc.downloadTempName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%s.tmp\x00" as *const u8 as *const libc::c_char, localName);
    Cvar_Set(b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
             remoteName);
    Cvar_Set(b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Cvar_Set(b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Cvar_SetValue(b"cl_downloadTime\x00" as *const u8 as *const libc::c_char,
                  cls.realtime as libc::c_float);
    clc.downloadBlock = 0i32;
    clc.downloadCount = 0i32;
    CL_AddReliableCommand(va(b"download %s\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             remoteName), qfalse);
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetPing(mut n: libc::c_int,
                                    mut buf: *mut libc::c_char,
                                    mut buflen: libc::c_int,
                                    mut pingtime: *mut libc::c_int) {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut time: libc::c_int = 0;
    let mut maxPing: libc::c_int = 0;
    if n < 0i32 || n >= 32i32 || 0 == cl_pinglist[n as usize].adr.port {
        *buf.offset(0isize) = '\u{0}' as i32 as libc::c_char;
        *pingtime = 0i32;
        return
    }
    str = NET_AdrToStringwPort(cl_pinglist[n as usize].adr);
    Q_strncpyz(buf, str, buflen);
    time = cl_pinglist[n as usize].time;
    if 0 == time {
        time = Sys_Milliseconds() - cl_pinglist[n as usize].start;
        maxPing =
            Cvar_VariableIntegerValue(b"cl_maxPing\x00" as *const u8 as
                                          *const libc::c_char);
        if maxPing < 100i32 { maxPing = 100i32 }
        if time < maxPing { time = 0i32 }
    }
    CL_SetServerInfoByAddress(cl_pinglist[n as usize].adr,
                              cl_pinglist[n as usize].info.as_mut_ptr(),
                              cl_pinglist[n as usize].time);
    *pingtime = time;
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetPingInfo(mut n: libc::c_int,
                                        mut buf: *mut libc::c_char,
                                        mut buflen: libc::c_int) {
    if n < 0i32 || n >= 32i32 || 0 == cl_pinglist[n as usize].adr.port {
        if 0 != buflen {
            *buf.offset(0isize) = '\u{0}' as i32 as libc::c_char
        }
        return
    }
    Q_strncpyz(buf, cl_pinglist[n as usize].info.as_mut_ptr(), buflen);
}
#[no_mangle]
pub unsafe extern "C" fn CL_ClearPing(mut n: libc::c_int) {
    if n < 0i32 || n >= 32i32 { return }
    cl_pinglist[n as usize].adr.port = 0i32 as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetPingQueueCount() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut pingptr: *mut ping_t = 0 as *mut ping_t;
    count = 0i32;
    pingptr = cl_pinglist.as_mut_ptr();
    i = 0i32;
    while i < 32i32 {
        if 0 != (*pingptr).adr.port { count += 1 }
        i += 1;
        pingptr = pingptr.offset(1isize)
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn CL_CDKeyValidate(mut key: *const libc::c_char,
                                          mut checksum: *const libc::c_char)
 -> qboolean {
    let mut ch: libc::c_char = 0;
    let mut sum: byte = 0;
    let mut chs: [libc::c_char; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = strlen(key) as libc::c_int;
    if len != 16i32 { return qfalse }
    if !checksum.is_null() && strlen(checksum) != 2i32 as libc::c_ulong {
        return qfalse
    }
    sum = 0i32 as byte;
    i = 0i32;
    while i < len {
        let fresh11 = key;
        key = key.offset(1);
        ch = *fresh11;
        if ch as libc::c_int >= 'a' as i32 && ch as libc::c_int <= 'z' as i32
           {
            ch = (ch as libc::c_int - 32i32) as libc::c_char
        }
        match ch as libc::c_int {
            50 | 51 | 55 | 65 | 66 | 67 | 68 | 71 | 72 | 74 | 76 | 80 | 82 |
            83 | 84 | 87 => {
                sum = (sum as libc::c_int + ch as libc::c_int) as byte;
                i += 1
            }
            _ => { return qfalse }
        }
    }
    sprintf(chs.as_mut_ptr(), b"%02x\x00" as *const u8 as *const libc::c_char,
            sum as libc::c_int);
    if !checksum.is_null() && 0 == Q_stricmp(chs.as_mut_ptr(), checksum) {
        return qtrue
    }
    if checksum.is_null() { return qtrue }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn CL_ServerStatus(mut serverAddress: *mut libc::c_char,
                                         mut serverStatusString:
                                             *mut libc::c_char,
                                         mut maxLen: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut to: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut serverStatus: *mut serverStatus_t = 0 as *mut serverStatus_t;
    if serverAddress.is_null() {
        i = 0i32;
        while i < 16i32 {
            cl_serverStatusList[i as usize].address.port =
                0i32 as libc::c_ushort;
            cl_serverStatusList[i as usize].retrieved = qtrue;
            i += 1
        }
        return qfalse as libc::c_int
    }
    if 0 == NET_StringToAdr(serverAddress, &mut to, NA_UNSPEC) {
        return qfalse as libc::c_int
    }
    serverStatus = CL_GetServerStatus(to);
    if serverStatusString.is_null() {
        (*serverStatus).retrieved = qtrue;
        return qfalse as libc::c_int
    }
    if 0 != NET_CompareAdr(to, (*serverStatus).address) as u64 {
        if 0 == (*serverStatus).pending as u64 {
            Q_strncpyz(serverStatusString,
                       (*serverStatus).string.as_mut_ptr(), maxLen);
            (*serverStatus).retrieved = qtrue;
            (*serverStatus).startTime = 0i32;
            return qtrue as libc::c_int
        } else {
            if (*serverStatus).startTime <
                   Com_Milliseconds() - (*cl_serverStatusResendTime).integer {
                (*serverStatus).print = qfalse;
                (*serverStatus).pending = qtrue;
                (*serverStatus).retrieved = qfalse;
                (*serverStatus).time = 0i32;
                (*serverStatus).startTime = Com_Milliseconds();
                NET_OutOfBandPrint(NS_CLIENT, to,
                                   b"getstatus\x00" as *const u8 as
                                       *const libc::c_char);
                return qfalse as libc::c_int
            }
        }
    } else if 0 != (*serverStatus).retrieved as u64 {
        (*serverStatus).address = to;
        (*serverStatus).print = qfalse;
        (*serverStatus).pending = qtrue;
        (*serverStatus).retrieved = qfalse;
        (*serverStatus).startTime = Com_Milliseconds();
        (*serverStatus).time = 0i32;
        NET_OutOfBandPrint(NS_CLIENT, to,
                           b"getstatus\x00" as *const u8 as
                               *const libc::c_char);
        return qfalse as libc::c_int
    }
    return qfalse as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_Voip_f() {
    let mut cmd: *const libc::c_char = Cmd_Argv(1i32);
    let mut reason: *const libc::c_char = 0 as *const libc::c_char;
    if clc.state as libc::c_uint != CA_ACTIVE as libc::c_int as libc::c_uint {
        reason =
            b"Not connected to a server\x00" as *const u8 as
                *const libc::c_char
    } else if 0 == clc.voipCodecInitialized as u64 {
        reason =
            b"Voip codec not initialized\x00" as *const u8 as
                *const libc::c_char
    } else if 0 == clc.voipEnabled as u64 {
        reason =
            b"Server doesn\'t support VoIP\x00" as *const u8 as
                *const libc::c_char
    } else if 0 == clc.demoplaying as u64 &&
                  (Cvar_VariableValue(b"g_gametype\x00" as *const u8 as
                                          *const libc::c_char) ==
                       GT_SINGLE_PLAYER as libc::c_int as libc::c_float ||
                       0. !=
                           Cvar_VariableValue(b"ui_singlePlayerActive\x00" as
                                                  *const u8 as
                                                  *const libc::c_char)) {
        reason =
            b"running in single-player mode\x00" as *const u8 as
                *const libc::c_char
    }
    if !reason.is_null() {
        Com_Printf(b"VoIP: command ignored: %s\n\x00" as *const u8 as
                       *const libc::c_char, reason);
        return
    }
    if strcmp(cmd, b"ignore\x00" as *const u8 as *const libc::c_char) == 0i32
       {
        CL_UpdateVoipIgnore(Cmd_Argv(2i32), qtrue);
    } else if strcmp(cmd, b"unignore\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        CL_UpdateVoipIgnore(Cmd_Argv(2i32), qfalse);
    } else if strcmp(cmd, b"gain\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        if Cmd_Argc() > 3i32 {
            CL_UpdateVoipGain(Cmd_Argv(2i32),
                              atof(Cmd_Argv(3i32)) as libc::c_float);
        } else if 0 != Q_isanumber(Cmd_Argv(2i32)) as u64 {
            let mut id: libc::c_int = atoi(Cmd_Argv(2i32));
            if id >= 0i32 && id < 64i32 {
                Com_Printf(b"VoIP: current gain for player #%d is %f\n\x00" as
                               *const u8 as *const libc::c_char, id,
                           clc.voipGain[id as usize] as libc::c_double);
            } else {
                Com_Printf(b"VoIP: invalid player ID#\n\x00" as *const u8 as
                               *const libc::c_char);
            }
        } else {
            Com_Printf(b"usage: voip gain <playerID#> [value]\n\x00" as
                           *const u8 as *const libc::c_char);
        }
    } else if strcmp(cmd, b"muteall\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        Com_Printf(b"VoIP: muting incoming voice\n\x00" as *const u8 as
                       *const libc::c_char);
        CL_AddReliableCommand(b"voip muteall\x00" as *const u8 as
                                  *const libc::c_char, qfalse);
        clc.voipMuteAll = qtrue
    } else if strcmp(cmd,
                     b"unmuteall\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        Com_Printf(b"VoIP: unmuting incoming voice\n\x00" as *const u8 as
                       *const libc::c_char);
        CL_AddReliableCommand(b"voip unmuteall\x00" as *const u8 as
                                  *const libc::c_char, qfalse);
        clc.voipMuteAll = qfalse
    } else {
        Com_Printf(b"usage: voip [un]ignore <playerID#>\n       voip [un]muteall\n       voip gain <playerID#> [value]\n\x00"
                       as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn CL_UpdateVoipGain(mut idstr: *const libc::c_char,
                                       mut gain: libc::c_float) {
    if *idstr as libc::c_int >= '0' as i32 &&
           *idstr as libc::c_int <= '9' as i32 {
        let id: libc::c_int = atoi(idstr);
        if gain < 0.0f32 { gain = 0.0f32 }
        if id >= 0i32 && id < 64i32 {
            clc.voipGain[id as usize] = gain;
            Com_Printf(b"VoIP: player #%d gain now set to %f\n\x00" as
                           *const u8 as *const libc::c_char, id,
                       gain as libc::c_double);
        }
    };
}
unsafe extern "C" fn CL_UpdateVoipIgnore(mut idstr: *const libc::c_char,
                                         mut ignore: qboolean) {
    if *idstr as libc::c_int >= '0' as i32 &&
           *idstr as libc::c_int <= '9' as i32 {
        let id: libc::c_int = atoi(idstr);
        if id >= 0i32 && id < 64i32 {
            clc.voipIgnore[id as usize] = ignore;
            CL_AddReliableCommand(va(b"voip %s %d\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char,
                                     if 0 != ignore as libc::c_uint {
                                         b"ignore\x00" as *const u8 as
                                             *const libc::c_char
                                     } else {
                                         b"unignore\x00" as *const u8 as
                                             *const libc::c_char
                                     }, id), qfalse);
            Com_Printf(b"VoIP: %s ignoring player #%d\n\x00" as *const u8 as
                           *const libc::c_char,
                       if 0 != ignore as libc::c_uint {
                           b"Now\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"No longer\x00" as *const u8 as
                               *const libc::c_char
                       }, id);
            return
        }
    }
    Com_Printf(b"VoIP: invalid player ID#\n\x00" as *const u8 as
                   *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateVisiblePings_f(mut source: libc::c_int)
 -> qboolean {
    let mut slots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut pingTime: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut status: qboolean = qfalse;
    if source < 0i32 || source > 3i32 { return qfalse }
    cls.pingUpdateSource = source;
    slots = CL_GetPingQueueCount();
    if slots < 32i32 {
        let mut server: *mut serverInfo_t = 0 as *mut serverInfo_t;
        match source {
            0 => {
                server =
                    &mut *cls.localServers.as_mut_ptr().offset(0isize) as
                        *mut serverInfo_t;
                max = cls.numlocalservers
            }
            2 => {
                server =
                    &mut *cls.globalServers.as_mut_ptr().offset(0isize) as
                        *mut serverInfo_t;
                max = cls.numglobalservers
            }
            3 => {
                server =
                    &mut *cls.favoriteServers.as_mut_ptr().offset(0isize) as
                        *mut serverInfo_t;
                max = cls.numfavoriteservers
            }
            _ => { return qfalse }
        }
        i = 0i32;
        while i < max {
            if 0 != (*server.offset(i as isize)).visible as u64 {
                if (*server.offset(i as isize)).ping == -1i32 {
                    let mut j: libc::c_int = 0;
                    if slots >= 32i32 { break ; }
                    j = 0i32;
                    while j < 32i32 {
                        if !(0 == cl_pinglist[j as usize].adr.port) {
                            if 0 !=
                                   NET_CompareAdr(cl_pinglist[j as usize].adr,
                                                  (*server.offset(i as
                                                                      isize)).adr)
                                       as u64 {
                                // already on the list
                                break ;
                            }
                        }
                        j += 1
                    }
                    if j >= 32i32 {
                        status = qtrue;
                        j = 0i32;
                        while j < 32i32 {
                            if 0 == cl_pinglist[j as usize].adr.port {
                                break ;
                            }
                            j += 1
                        }
                        memcpy(&mut (*cl_pinglist.as_mut_ptr().offset(j as
                                                                          isize)).adr
                                   as *mut netadr_t as *mut libc::c_void,
                               &mut (*server.offset(i as isize)).adr as
                                   *mut netadr_t as *const libc::c_void,
                               ::std::mem::size_of::<netadr_t>() as
                                   libc::c_ulong);
                        cl_pinglist[j as usize].start = Sys_Milliseconds();
                        cl_pinglist[j as usize].time = 0i32;
                        NET_OutOfBandPrint(NS_CLIENT,
                                           cl_pinglist[j as usize].adr,
                                           b"getinfo xxx\x00" as *const u8 as
                                               *const libc::c_char);
                        slots += 1
                    }
                } else if (*server.offset(i as isize)).ping == 0i32 {
                    if source == 2i32 {
                        if cls.numGlobalServerAddresses > 0i32 {
                            cls.numGlobalServerAddresses -= 1;
                            CL_InitServerInfo(&mut *server.offset(i as isize),
                                              &mut *cls.globalServerAddresses.as_mut_ptr().offset(cls.numGlobalServerAddresses
                                                                                                      as
                                                                                                      isize));
                        }
                    }
                }
            }
            i += 1
        }
    }
    if 0 != slots { status = qtrue }
    i = 0i32;
    while i < 32i32 {
        if !(0 == cl_pinglist[i as usize].adr.port) {
            CL_GetPing(i, buff.as_mut_ptr(), 1024i32, &mut pingTime);
            if pingTime != 0i32 { CL_ClearPing(i); status = qtrue }
        }
        i += 1
    }
    return status;
}