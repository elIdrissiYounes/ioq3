#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           label_break_value,
           libc)]
extern crate libc;
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
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
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
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
    /* Maximum symbol */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct huff_t {
        pub blocNode: libc::c_int,
        pub blocPtrs: libc::c_int,
        pub tree: *mut node_t,
        pub lhead: *mut node_t,
        pub ltail: *mut node_t,
        pub loc: [*mut node_t; 257],
        pub freelist: *mut *mut node_t,
        pub nodeList: [node_t; 768],
        pub nodePtrs: [*mut node_t; 768],
    }
    pub type node_t = nodetype;
    /* This is based on the Adaptive Huffman algorithm described in Sayood's Data
 * Compression book.  The ranks are not actually stored, but implicitly defined
 * by the location of a node within a doubly-linked list */
    /* NYT = Not Yet Transmitted */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct nodetype {
        pub left: *mut nodetype,
        pub right: *mut nodetype,
        pub parent: *mut nodetype,
        pub next: *mut nodetype,
        pub prev: *mut nodetype,
        pub head: *mut *mut nodetype,
        pub weight: libc::c_int,
        pub symbol: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct huffman_t {
        pub compressor: huff_t,
        pub decompressor: huff_t,
    }
    use super::q_shared_h::{qboolean, byte, usercmd_t, entityState_s,
                            entityState_t, playerState_s};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Huff_addRef(huff: *mut huff_t, ch: byte);
        #[no_mangle]
        pub fn Huff_Init(huff: *mut huffman_t);
        #[no_mangle]
        pub fn Huff_offsetTransmit(huff: *mut huff_t, ch: libc::c_int,
                                   fout: *mut byte, offset: *mut libc::c_int,
                                   maxoffset: libc::c_int);
        #[no_mangle]
        pub fn Huff_putBit(bit: libc::c_int, fout: *mut byte,
                           offset: *mut libc::c_int);
        #[no_mangle]
        pub fn Huff_offsetReceive(node: *mut node_t, ch: *mut libc::c_int,
                                  fin: *mut byte, offset: *mut libc::c_int,
                                  maxoffset: libc::c_int);
        #[no_mangle]
        pub fn Huff_getBit(fout: *mut byte, offset: *mut libc::c_int)
         -> libc::c_int;
        // don't use if you don't know what you're doing.
        #[no_mangle]
        pub fn Huff_getBloc() -> libc::c_int;
        #[no_mangle]
        pub fn Huff_setBloc(_bloc: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/qcommon/msg.c"]
pub mod msg_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netField_t {
        pub name: *mut libc::c_char,
        pub offset: libc::c_int,
        pub bits: libc::c_int,
    }
    use super::{libc};
    use super::qcommon_h::{msg_t};
    use super::q_shared_h::{cvar_t};
    extern "C" {
        #[no_mangle]
        pub static mut cl_shownet: *mut cvar_t;
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
use self::stddef_h::{size_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cvar_s, cvar_t,
                       playerState_s, playerState_t, usercmd_s, usercmd_t,
                       trType_t, TR_GRAVITY, TR_SINE, TR_LINEAR_STOP,
                       TR_LINEAR, TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, Q_strncpyz, Com_Error,
                       Com_Printf};
use self::qcommon_h::{msg_t, huff_t, node_t, nodetype, huffman_t, Huff_addRef,
                      Huff_Init, Huff_offsetTransmit, Huff_putBit,
                      Huff_offsetReceive, Huff_getBit, Huff_getBloc,
                      Huff_setBloc};
use self::msg_c::{netField_t, cl_shownet};
use self::assert_h::{__assert_fail};
use self::string_h::{memcpy, memset, strlen};
#[no_mangle]
pub unsafe extern "C" fn MSG_Init(mut buf: *mut msg_t, mut data: *mut byte,
                                  mut length: libc::c_int) {
    if 0 == msgInit as u64 { MSG_initHuffman(); }
    memset(buf as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<msg_t>() as libc::c_ulong);
    (*buf).data = data;
    (*buf).maxsize = length;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_initHuffman() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    msgInit = qtrue;
    Huff_Init(&mut msgHuff);
    i = 0i32;
    while i < 256i32 {
        j = 0i32;
        while j < msg_hData[i as usize] {
            Huff_addRef(&mut msgHuff.compressor, i as byte);
            Huff_addRef(&mut msgHuff.decompressor, i as byte);
            j += 1
        }
        i += 1
    };
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
static mut msgHuff: huffman_t =
    huffman_t{compressor:
                  huff_t{blocNode: 0,
                         blocPtrs: 0,
                         tree: 0 as *const node_t as *mut node_t,
                         lhead: 0 as *const node_t as *mut node_t,
                         ltail: 0 as *const node_t as *mut node_t,
                         loc: [0 as *const node_t as *mut node_t; 257],
                         freelist:
                             0 as *const *mut node_t as *mut *mut node_t,
                         nodeList:
                             [nodetype{left:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       right:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       parent:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       next:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       prev:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       head:
                                           0 as *const *mut nodetype as
                                               *mut *mut nodetype,
                                       weight: 0,
                                       symbol: 0,}; 768],
                         nodePtrs: [0 as *const node_t as *mut node_t; 768],},
              decompressor:
                  huff_t{blocNode: 0,
                         blocPtrs: 0,
                         tree: 0 as *const node_t as *mut node_t,
                         lhead: 0 as *const node_t as *mut node_t,
                         ltail: 0 as *const node_t as *mut node_t,
                         loc: [0 as *const node_t as *mut node_t; 257],
                         freelist:
                             0 as *const *mut node_t as *mut *mut node_t,
                         nodeList:
                             [nodetype{left:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       right:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       parent:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       next:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       prev:
                                           0 as *const nodetype as
                                               *mut nodetype,
                                       head:
                                           0 as *const *mut nodetype as
                                               *mut *mut nodetype,
                                       weight: 0,
                                       symbol: 0,}; 768],
                         nodePtrs:
                             [0 as *const node_t as *mut node_t; 768],},};
#[no_mangle]
pub static mut msg_hData: [libc::c_int; 256] =
    [250315i32, 41193i32, 6292i32, 7106i32, 3730i32, 3750i32, 6110i32,
     23283i32, 33317i32, 6950i32, 7838i32, 9714i32, 9257i32, 17259i32,
     3949i32, 1778i32, 8288i32, 1604i32, 1590i32, 1663i32, 1100i32, 1213i32,
     1238i32, 1134i32, 1749i32, 1059i32, 1246i32, 1149i32, 1273i32, 4486i32,
     2805i32, 3472i32, 21819i32, 1159i32, 1670i32, 1066i32, 1043i32, 1012i32,
     1053i32, 1070i32, 1726i32, 888i32, 1180i32, 850i32, 960i32, 780i32,
     1752i32, 3296i32, 10630i32, 4514i32, 5881i32, 2685i32, 4650i32, 3837i32,
     2093i32, 1867i32, 2584i32, 1949i32, 1972i32, 940i32, 1134i32, 1788i32,
     1670i32, 1206i32, 5719i32, 6128i32, 7222i32, 6654i32, 3710i32, 3795i32,
     1492i32, 1524i32, 2215i32, 1140i32, 1355i32, 971i32, 2180i32, 1248i32,
     1328i32, 1195i32, 1770i32, 1078i32, 1264i32, 1266i32, 1168i32, 965i32,
     1155i32, 1186i32, 1347i32, 1228i32, 1529i32, 1600i32, 2617i32, 2048i32,
     2546i32, 3275i32, 2410i32, 3585i32, 2504i32, 2800i32, 2675i32, 6146i32,
     3663i32, 2840i32, 14253i32, 3164i32, 2221i32, 1687i32, 3208i32, 2739i32,
     3512i32, 4796i32, 4091i32, 3515i32, 5288i32, 4016i32, 7937i32, 6031i32,
     5360i32, 3924i32, 4892i32, 3743i32, 4566i32, 4807i32, 5852i32, 6400i32,
     6225i32, 8291i32, 23243i32, 7838i32, 7073i32, 8935i32, 5437i32, 4483i32,
     3641i32, 5256i32, 5312i32, 5328i32, 5370i32, 3492i32, 2458i32, 1694i32,
     1821i32, 2121i32, 1916i32, 1149i32, 1516i32, 1367i32, 1236i32, 1029i32,
     1258i32, 1104i32, 1245i32, 1006i32, 1149i32, 1025i32, 1241i32, 952i32,
     1287i32, 997i32, 1713i32, 1009i32, 1187i32, 879i32, 1099i32, 929i32,
     1078i32, 951i32, 1656i32, 930i32, 1153i32, 1030i32, 1262i32, 1062i32,
     1214i32, 1060i32, 1621i32, 930i32, 1106i32, 912i32, 1034i32, 892i32,
     1158i32, 990i32, 1175i32, 850i32, 1121i32, 903i32, 1087i32, 920i32,
     1144i32, 1056i32, 3462i32, 2240i32, 4397i32, 12136i32, 7758i32, 1345i32,
     1307i32, 3278i32, 1950i32, 886i32, 1023i32, 1112i32, 1077i32, 1042i32,
     1061i32, 1071i32, 1484i32, 1001i32, 1096i32, 915i32, 1052i32, 995i32,
     1070i32, 876i32, 1111i32, 851i32, 1059i32, 805i32, 1112i32, 923i32,
     1103i32, 817i32, 1899i32, 1872i32, 976i32, 841i32, 1127i32, 956i32,
     1159i32, 950i32, 7791i32, 954i32, 1289i32, 933i32, 1127i32, 3207i32,
     1020i32, 927i32, 1355i32, 768i32, 1040i32, 745i32, 952i32, 805i32,
     1073i32, 740i32, 1013i32, 805i32, 1008i32, 796i32, 996i32, 1057i32,
     11457i32, 13504i32];
static mut msgInit: qboolean = qfalse;
#[no_mangle]
pub unsafe extern "C" fn MSG_InitOOB(mut buf: *mut msg_t, mut data: *mut byte,
                                     mut length: libc::c_int) {
    if 0 == msgInit as u64 { MSG_initHuffman(); }
    memset(buf as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<msg_t>() as libc::c_ulong);
    (*buf).data = data;
    (*buf).maxsize = length;
    (*buf).oob = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_Clear(mut buf: *mut msg_t) {
    (*buf).cursize = 0i32;
    (*buf).overflowed = qfalse;
    (*buf).bit = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteData(mut buf: *mut msg_t,
                                       mut data: *const libc::c_void,
                                       mut length: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < length {
        MSG_WriteByte(buf,
                      *(data as *mut byte).offset(i as isize) as libc::c_int);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteByte(mut sb: *mut msg_t,
                                       mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 8i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBits(mut msg: *mut msg_t,
                                       mut value: libc::c_int,
                                       mut bits: libc::c_int) {
    let mut i: libc::c_int = 0;
    oldsize += bits;
    if 0 != (*msg).overflowed as u64 { return }
    if bits == 0i32 || bits < -31i32 || bits > 32i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MSG_WriteBits: bad bits %i\x00" as *const u8 as
                      *const libc::c_char, bits);
    }
    if bits < 0i32 { bits = -bits }
    if 0 != (*msg).oob as u64 {
        if (*msg).cursize + (bits >> 3i32) > (*msg).maxsize {
            (*msg).overflowed = qtrue;
            return
        }
        if bits == 8i32 {
            *(*msg).data.offset((*msg).cursize as isize) = value as byte;
            (*msg).cursize += 1i32;
            (*msg).bit += 8i32
        } else if bits == 16i32 {
            let mut temp: libc::c_short = value as libc::c_short;
            memcpy(&mut *(*msg).data.offset((*msg).cursize as isize) as
                       *mut byte as *mut libc::c_void,
                   &mut temp as *mut libc::c_short as *const libc::c_void,
                   2i32 as libc::c_ulong);
            (*msg).cursize += 2i32;
            (*msg).bit += 16i32
        } else if bits == 32i32 {
            memcpy(&mut *(*msg).data.offset((*msg).cursize as isize) as
                       *mut byte as *mut libc::c_void,
                   &mut value as *mut libc::c_int as *const libc::c_void,
                   4i32 as libc::c_ulong);
            (*msg).cursize += 4i32;
            (*msg).bit += 32i32
        } else {
            Com_Error(ERR_DROP as libc::c_int,
                      b"can\'t write %d bits\x00" as *const u8 as
                          *const libc::c_char, bits);
        }
    } else {
        value =
            (value as libc::c_uint & 0xffffffffu32 >> 32i32 - bits) as
                libc::c_int;
        if 0 != bits & 7i32 {
            let mut nbits: libc::c_int = 0;
            nbits = bits & 7i32;
            if (*msg).bit + nbits > (*msg).maxsize << 3i32 {
                (*msg).overflowed = qtrue;
                return
            }
            i = 0i32;
            while i < nbits {
                Huff_putBit(value & 1i32, (*msg).data, &mut (*msg).bit);
                value = value >> 1i32;
                i += 1
            }
            bits = bits - nbits
        }
        if 0 != bits {
            i = 0i32;
            while i < bits {
                Huff_offsetTransmit(&mut msgHuff.compressor, value & 0xffi32,
                                    (*msg).data, &mut (*msg).bit,
                                    (*msg).maxsize << 3i32);
                value = value >> 8i32;
                if (*msg).bit > (*msg).maxsize << 3i32 {
                    (*msg).overflowed = qtrue;
                    return
                }
                i += 8i32
            }
        }
        (*msg).cursize = ((*msg).bit >> 3i32) + 1i32
    };
}
/*
==============================================================================

			MESSAGE IO FUNCTIONS

Handles byte ordering and avoids alignment errors
==============================================================================
*/
#[no_mangle]
pub static mut oldsize: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn MSG_Bitstream(mut buf: *mut msg_t) {
    (*buf).oob = qfalse;
}
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
#[no_mangle]
pub unsafe extern "C" fn MSG_Copy(mut buf: *mut msg_t, mut data: *mut byte,
                                  mut length: libc::c_int,
                                  mut src: *mut msg_t) {
    if length < (*src).cursize {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MSG_Copy: can\'t copy into a smaller msg_t buffer\x00" as
                      *const u8 as *const libc::c_char);
    }
    memcpy(buf as *mut libc::c_void, src as *const libc::c_void,
           ::std::mem::size_of::<msg_t>() as libc::c_ulong);
    (*buf).data = data;
    memcpy((*buf).data as *mut libc::c_void,
           (*src).data as *const libc::c_void,
           (*src).cursize as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteChar(mut sb: *mut msg_t,
                                       mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 8i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteShort(mut sb: *mut msg_t,
                                        mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 16i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteLong(mut sb: *mut msg_t,
                                       mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 32i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteFloat(mut sb: *mut msg_t,
                                        mut f: libc::c_float) {
    let mut dat: floatint_t = floatint_t{f: 0.,};
    dat.f = f;
    MSG_WriteBits(sb, dat.i, 32i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteString(mut sb: *mut msg_t,
                                         mut s: *const libc::c_char) {
    if s.is_null() {
        MSG_WriteData(sb,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *const libc::c_void, 1i32);
    } else {
        let mut l: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut string: [libc::c_char; 1024] = [0; 1024];
        l = strlen(s) as libc::c_int;
        if l >= 1024i32 {
            Com_Printf(b"MSG_WriteString: MAX_STRING_CHARS\x00" as *const u8
                           as *const libc::c_char);
            MSG_WriteData(sb,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *const libc::c_void, 1i32);
            return
        }
        Q_strncpyz(string.as_mut_ptr(), s,
                   ::std::mem::size_of::<[libc::c_char; 1024]>() as
                       libc::c_ulong as libc::c_int);
        i = 0i32;
        while i < l {
            if *(string.as_mut_ptr() as *mut byte).offset(i as isize) as
                   libc::c_int > 127i32 ||
                   string[i as usize] as libc::c_int == '%' as i32 {
                string[i as usize] = '.' as i32 as libc::c_char
            }
            i += 1
        }
        MSG_WriteData(sb, string.as_mut_ptr() as *const libc::c_void,
                      l + 1i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteBigString(mut sb: *mut msg_t,
                                            mut s: *const libc::c_char) {
    if s.is_null() {
        MSG_WriteData(sb,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *const libc::c_void, 1i32);
    } else {
        let mut l: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut string: [libc::c_char; 8192] = [0; 8192];
        l = strlen(s) as libc::c_int;
        if l >= 8192i32 {
            Com_Printf(b"MSG_WriteString: BIG_INFO_STRING\x00" as *const u8 as
                           *const libc::c_char);
            MSG_WriteData(sb,
                          b"\x00" as *const u8 as *const libc::c_char as
                              *const libc::c_void, 1i32);
            return
        }
        Q_strncpyz(string.as_mut_ptr(), s,
                   ::std::mem::size_of::<[libc::c_char; 8192]>() as
                       libc::c_ulong as libc::c_int);
        i = 0i32;
        while i < l {
            if *(string.as_mut_ptr() as *mut byte).offset(i as isize) as
                   libc::c_int > 127i32 ||
                   string[i as usize] as libc::c_int == '%' as i32 {
                string[i as usize] = '.' as i32 as libc::c_char
            }
            i += 1
        }
        MSG_WriteData(sb, string.as_mut_ptr() as *const libc::c_void,
                      l + 1i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteAngle16(mut sb: *mut msg_t,
                                          mut f: libc::c_float) {
    MSG_WriteShort(sb,
                   (f * 65536i32 as libc::c_float / 360i32 as libc::c_float)
                       as libc::c_int & 65535i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_HashKey(mut string: *const libc::c_char,
                                     mut maxlen: libc::c_int) -> libc::c_int {
    let mut hash: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    hash = 0i32;
    i = 0i32;
    while i < maxlen &&
              *string.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        if 0 != *string.offset(i as isize) as libc::c_int & 0x80i32 ||
               *string.offset(i as isize) as libc::c_int == '%' as i32 {
            hash += '.' as i32 * (119i32 + i)
        } else {
            hash += *string.offset(i as isize) as libc::c_int * (119i32 + i)
        }
        i += 1
    }
    hash = hash ^ hash >> 10i32 ^ hash >> 20i32;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_BeginReading(mut msg: *mut msg_t) {
    (*msg).readcount = 0i32;
    (*msg).bit = 0i32;
    (*msg).oob = qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_BeginReadingOOB(mut msg: *mut msg_t) {
    (*msg).readcount = 0i32;
    (*msg).bit = 0i32;
    (*msg).oob = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBits(mut msg: *mut msg_t,
                                      mut bits: libc::c_int) -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut get: libc::c_int = 0;
    let mut sgn: qboolean = qfalse;
    let mut i: libc::c_int = 0;
    let mut nbits: libc::c_int = 0;
    if (*msg).readcount > (*msg).cursize { return 0i32 }
    value = 0i32;
    if bits < 0i32 { bits = -bits; sgn = qtrue } else { sgn = qfalse }
    if 0 != (*msg).oob as u64 {
        if (*msg).readcount + (bits >> 3i32) > (*msg).cursize {
            (*msg).readcount = (*msg).cursize + 1i32;
            return 0i32
        }
        if bits == 8i32 {
            value =
                *(*msg).data.offset((*msg).readcount as isize) as libc::c_int;
            (*msg).readcount += 1i32;
            (*msg).bit += 8i32
        } else if bits == 16i32 {
            let mut temp: libc::c_short = 0;
            memcpy(&mut temp as *mut libc::c_short as *mut libc::c_void,
                   &mut *(*msg).data.offset((*msg).readcount as isize) as
                       *mut byte as *const libc::c_void,
                   2i32 as libc::c_ulong);
            value = temp as libc::c_int;
            (*msg).readcount += 2i32;
            (*msg).bit += 16i32
        } else if bits == 32i32 {
            memcpy(&mut value as *mut libc::c_int as *mut libc::c_void,
                   &mut *(*msg).data.offset((*msg).readcount as isize) as
                       *mut byte as *const libc::c_void,
                   4i32 as libc::c_ulong);
            (*msg).readcount += 4i32;
            (*msg).bit += 32i32
        } else {
            Com_Error(ERR_DROP as libc::c_int,
                      b"can\'t read %d bits\x00" as *const u8 as
                          *const libc::c_char, bits);
        }
    } else {
        nbits = 0i32;
        if 0 != bits & 7i32 {
            nbits = bits & 7i32;
            if (*msg).bit + nbits > (*msg).cursize << 3i32 {
                (*msg).readcount = (*msg).cursize + 1i32;
                return 0i32
            }
            i = 0i32;
            while i < nbits {
                value |= Huff_getBit((*msg).data, &mut (*msg).bit) << i;
                i += 1
            }
            bits = bits - nbits
        }
        if 0 != bits {
            i = 0i32;
            while i < bits {
                Huff_offsetReceive(msgHuff.decompressor.tree, &mut get,
                                   (*msg).data, &mut (*msg).bit,
                                   (*msg).cursize << 3i32);
                value =
                    (value as libc::c_uint |
                         (get as libc::c_uint) << i + nbits) as libc::c_int;
                if (*msg).bit > (*msg).cursize << 3i32 {
                    (*msg).readcount = (*msg).cursize + 1i32;
                    return 0i32
                }
                i += 8i32
            }
        }
        (*msg).readcount = ((*msg).bit >> 3i32) + 1i32
    }
    if 0 != sgn as libc::c_uint && bits > 0i32 && bits < 32i32 {
        if 0 != value & 1i32 << bits - 1i32 {
            value |= -1i32 ^ (1i32 << bits) - 1i32
        }
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadChar(mut msg: *mut msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 8i32) as libc::c_schar as libc::c_int;
    if (*msg).readcount > (*msg).cursize { c = -1i32 }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadByte(mut msg: *mut msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 8i32) as libc::c_uchar as libc::c_int;
    if (*msg).readcount > (*msg).cursize { c = -1i32 }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadShort(mut msg: *mut msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 16i32) as libc::c_short as libc::c_int;
    if (*msg).readcount > (*msg).cursize { c = -1i32 }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadLong(mut msg: *mut msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 32i32);
    if (*msg).readcount > (*msg).cursize { c = -1i32 }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadFloat(mut msg: *mut msg_t) -> libc::c_float {
    let mut dat: floatint_t = floatint_t{f: 0.,};
    dat.i = MSG_ReadBits(msg, 32i32);
    if (*msg).readcount > (*msg).cursize { dat.f = -1i32 as libc::c_float }
    return dat.f;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadString(mut msg: *mut msg_t)
 -> *mut libc::c_char {
    static mut string: [libc::c_char; 1024] = [0; 1024];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0i32;
    loop  {
        c = MSG_ReadByte(msg);
        if c == -1i32 || c == 0i32 { break ; }
        if c == '%' as i32 { c = '.' as i32 }
        if c > 127i32 { c = '.' as i32 }
        // break only after reading all expected data from bitstream
        if l as libc::c_ulong >=
               (::std::mem::size_of::<[libc::c_char; 1024]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
            break ;
        }
        let fresh0 = l;
        l = l + 1;
        string[fresh0 as usize] = c as libc::c_char
    }
    string[l as usize] = '\u{0}' as i32 as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadBigString(mut msg: *mut msg_t)
 -> *mut libc::c_char {
    static mut string: [libc::c_char; 8192] = [0; 8192];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0i32;
    loop  {
        c = MSG_ReadByte(msg);
        if c == -1i32 || c == 0i32 { break ; }
        if c == '%' as i32 { c = '.' as i32 }
        if c > 127i32 { c = '.' as i32 }
        // break only after reading all expected data from bitstream
        if l as libc::c_ulong >=
               (::std::mem::size_of::<[libc::c_char; 8192]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
            break ;
        }
        let fresh1 = l;
        l = l + 1;
        string[fresh1 as usize] = c as libc::c_char
    }
    string[l as usize] = '\u{0}' as i32 as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadStringLine(mut msg: *mut msg_t)
 -> *mut libc::c_char {
    static mut string: [libc::c_char; 1024] = [0; 1024];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0i32;
    loop  {
        c = MSG_ReadByte(msg);
        if c == -1i32 || c == 0i32 || c == '\n' as i32 { break ; }
        if c == '%' as i32 { c = '.' as i32 }
        if c > 127i32 { c = '.' as i32 }
        // break only after reading all expected data from bitstream
        if l as libc::c_ulong >=
               (::std::mem::size_of::<[libc::c_char; 1024]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
            break ;
        }
        let fresh2 = l;
        l = l + 1;
        string[fresh2 as usize] = c as libc::c_char
    }
    string[l as usize] = '\u{0}' as i32 as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadAngle16(mut msg: *mut msg_t)
 -> libc::c_float {
    return (MSG_ReadShort(msg) as libc::c_double *
                (360.0f64 / 65536i32 as libc::c_double)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadData(mut msg: *mut msg_t,
                                      mut data: *mut libc::c_void,
                                      mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < len {
        *(data as *mut byte).offset(i as isize) = MSG_ReadByte(msg) as byte;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_LookaheadByte(mut msg: *mut msg_t)
 -> libc::c_int {
    let bloc: libc::c_int = Huff_getBloc();
    let readcount: libc::c_int = (*msg).readcount;
    let bit: libc::c_int = (*msg).bit;
    let mut c: libc::c_int = MSG_ReadByte(msg);
    Huff_setBloc(bloc);
    (*msg).readcount = readcount;
    (*msg).bit = bit;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaUsercmdKey(mut msg: *mut msg_t,
                                                  mut key: libc::c_int,
                                                  mut from: *mut usercmd_t,
                                                  mut to: *mut usercmd_t) {
    if (*to).serverTime - (*from).serverTime < 256i32 {
        MSG_WriteBits(msg, 1i32, 1i32);
        MSG_WriteBits(msg, (*to).serverTime - (*from).serverTime, 8i32);
    } else {
        MSG_WriteBits(msg, 0i32, 1i32);
        MSG_WriteBits(msg, (*to).serverTime, 32i32);
    }
    if (*from).angles[0usize] == (*to).angles[0usize] &&
           (*from).angles[1usize] == (*to).angles[1usize] &&
           (*from).angles[2usize] == (*to).angles[2usize] &&
           (*from).forwardmove as libc::c_int ==
               (*to).forwardmove as libc::c_int &&
           (*from).rightmove as libc::c_int == (*to).rightmove as libc::c_int
           && (*from).upmove as libc::c_int == (*to).upmove as libc::c_int &&
           (*from).buttons == (*to).buttons &&
           (*from).weapon as libc::c_int == (*to).weapon as libc::c_int {
        MSG_WriteBits(msg, 0i32, 1i32);
        oldsize += 7i32;
        return
    }
    key ^= (*to).serverTime;
    MSG_WriteBits(msg, 1i32, 1i32);
    MSG_WriteDeltaKey(msg, key, (*from).angles[0usize], (*to).angles[0usize],
                      16i32);
    MSG_WriteDeltaKey(msg, key, (*from).angles[1usize], (*to).angles[1usize],
                      16i32);
    MSG_WriteDeltaKey(msg, key, (*from).angles[2usize], (*to).angles[2usize],
                      16i32);
    MSG_WriteDeltaKey(msg, key, (*from).forwardmove as libc::c_int,
                      (*to).forwardmove as libc::c_int, 8i32);
    MSG_WriteDeltaKey(msg, key, (*from).rightmove as libc::c_int,
                      (*to).rightmove as libc::c_int, 8i32);
    MSG_WriteDeltaKey(msg, key, (*from).upmove as libc::c_int,
                      (*to).upmove as libc::c_int, 8i32);
    MSG_WriteDeltaKey(msg, key, (*from).buttons, (*to).buttons, 16i32);
    MSG_WriteDeltaKey(msg, key, (*from).weapon as libc::c_int,
                      (*to).weapon as libc::c_int, 8i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaKey(mut msg: *mut msg_t,
                                           mut key: libc::c_int,
                                           mut oldV: libc::c_int,
                                           mut newV: libc::c_int,
                                           mut bits: libc::c_int) {
    if oldV == newV { MSG_WriteBits(msg, 0i32, 1i32); return }
    MSG_WriteBits(msg, 1i32, 1i32);
    MSG_WriteBits(msg, newV ^ key, bits);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaUsercmdKey(mut msg: *mut msg_t,
                                                 mut key: libc::c_int,
                                                 mut from: *mut usercmd_t,
                                                 mut to: *mut usercmd_t) {
    if 0 != MSG_ReadBits(msg, 1i32) {
        (*to).serverTime = (*from).serverTime + MSG_ReadBits(msg, 8i32)
    } else { (*to).serverTime = MSG_ReadBits(msg, 32i32) }
    if 0 != MSG_ReadBits(msg, 1i32) {
        key ^= (*to).serverTime;
        (*to).angles[0usize] =
            MSG_ReadDeltaKey(msg, key, (*from).angles[0usize], 16i32);
        (*to).angles[1usize] =
            MSG_ReadDeltaKey(msg, key, (*from).angles[1usize], 16i32);
        (*to).angles[2usize] =
            MSG_ReadDeltaKey(msg, key, (*from).angles[2usize], 16i32);
        (*to).forwardmove =
            MSG_ReadDeltaKey(msg, key, (*from).forwardmove as libc::c_int,
                             8i32) as libc::c_schar;
        if (*to).forwardmove as libc::c_int == -128i32 {
            (*to).forwardmove = -127i32 as libc::c_schar
        }
        (*to).rightmove =
            MSG_ReadDeltaKey(msg, key, (*from).rightmove as libc::c_int, 8i32)
                as libc::c_schar;
        if (*to).rightmove as libc::c_int == -128i32 {
            (*to).rightmove = -127i32 as libc::c_schar
        }
        (*to).upmove =
            MSG_ReadDeltaKey(msg, key, (*from).upmove as libc::c_int, 8i32) as
                libc::c_schar;
        if (*to).upmove as libc::c_int == -128i32 {
            (*to).upmove = -127i32 as libc::c_schar
        }
        (*to).buttons = MSG_ReadDeltaKey(msg, key, (*from).buttons, 16i32);
        (*to).weapon =
            MSG_ReadDeltaKey(msg, key, (*from).weapon as libc::c_int, 8i32) as
                byte
    } else {
        (*to).angles[0usize] = (*from).angles[0usize];
        (*to).angles[1usize] = (*from).angles[1usize];
        (*to).angles[2usize] = (*from).angles[2usize];
        (*to).forwardmove = (*from).forwardmove;
        (*to).rightmove = (*from).rightmove;
        (*to).upmove = (*from).upmove;
        (*to).buttons = (*from).buttons;
        (*to).weapon = (*from).weapon
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaKey(mut msg: *mut msg_t,
                                          mut key: libc::c_int,
                                          mut oldV: libc::c_int,
                                          mut bits: libc::c_int)
 -> libc::c_int {
    if 0 != MSG_ReadBits(msg, 1i32) {
        return MSG_ReadBits(msg, bits) ^
                   key & kbitmask[(bits - 1i32) as usize]
    }
    return oldV;
}
/*
=============================================================================

delta functions with keys
  
=============================================================================
*/
#[no_mangle]
pub static mut kbitmask: [libc::c_int; 32] =
    [0x1i32, 0x3i32, 0x7i32, 0xfi32, 0x1fi32, 0x3fi32, 0x7fi32, 0xffi32,
     0x1ffi32, 0x3ffi32, 0x7ffi32, 0xfffi32, 0x1fffi32, 0x3fffi32, 0x7fffi32,
     0xffffi32, 0x1ffffi32, 0x3ffffi32, 0x7ffffi32, 0xfffffi32, 0x1fffffi32,
     0x3fffffi32, 0x7fffffi32, 0xffffffi32, 0x1ffffffi32, 0x3ffffffi32,
     0x7ffffffi32, 0xfffffffi32, 0x1fffffffi32, 0x3fffffffi32, 0x7fffffffi32,
     0xffffffffu32 as libc::c_int];
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaEntity(mut msg: *mut msg_t,
                                              mut from: *mut entityState_s,
                                              mut to: *mut entityState_s,
                                              mut force: qboolean) {
    let mut i: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut numFields: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut trunc: libc::c_int = 0;
    let mut fullFloat: libc::c_float = 0.;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    numFields =
        (::std::mem::size_of::<[netField_t; 51]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<netField_t>()
                                             as libc::c_ulong) as libc::c_int;
    if (numFields + 1i32) as libc::c_ulong ==
           (::std::mem::size_of::<entityState_s>() as
                libc::c_ulong).wrapping_div(4i32 as libc::c_ulong) {
    } else {
        __assert_fail(b"numFields + 1 == sizeof( *from )/4\x00" as *const u8
                          as *const libc::c_char,
                      b"code/qcommon/msg.c\x00" as *const u8 as
                          *const libc::c_char, 831i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 93],
                                                &[libc::c_char; 93]>(b"void MSG_WriteDeltaEntity(msg_t *, struct entityState_s *, struct entityState_s *, qboolean)\x00")).as_ptr());
    }
    if to.is_null() {
        if from.is_null() { return }
        MSG_WriteBits(msg, (*from).number, 10i32);
        MSG_WriteBits(msg, 1i32, 1i32);
        return
    }
    if (*to).number < 0i32 || (*to).number >= 1i32 << 10i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"MSG_WriteDeltaEntity: Bad entity number: %i\x00" as
                      *const u8 as *const libc::c_char, (*to).number);
    }
    lc = 0i32;
    i = 0i32;
    field = entityStateFields.as_mut_ptr();
    while i < numFields {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        if *fromF != *toF { lc = i + 1i32 }
        i += 1;
        field = field.offset(1isize)
    }
    if lc == 0i32 {
        if 0 == force as u64 { return }
        MSG_WriteBits(msg, (*to).number, 10i32);
        MSG_WriteBits(msg, 0i32, 1i32);
        MSG_WriteBits(msg, 0i32, 1i32);
        return
    }
    MSG_WriteBits(msg, (*to).number, 10i32);
    MSG_WriteBits(msg, 0i32, 1i32);
    MSG_WriteBits(msg, 1i32, 1i32);
    MSG_WriteByte(msg, lc);
    oldsize += numFields;
    i = 0i32;
    field = entityStateFields.as_mut_ptr();
    while i < lc {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        if *fromF == *toF {
            MSG_WriteBits(msg, 0i32, 1i32);
        } else {
            MSG_WriteBits(msg, 1i32, 1i32);
            if (*field).bits == 0i32 {
                fullFloat = *(toF as *mut libc::c_float);
                trunc = fullFloat as libc::c_int;
                if fullFloat == 0.0f32 {
                    MSG_WriteBits(msg, 0i32, 1i32);
                    oldsize += 13i32
                } else {
                    MSG_WriteBits(msg, 1i32, 1i32);
                    if trunc as libc::c_float == fullFloat &&
                           trunc + (1i32 << 13i32 - 1i32) >= 0i32 &&
                           trunc + (1i32 << 13i32 - 1i32) < 1i32 << 13i32 {
                        MSG_WriteBits(msg, 0i32, 1i32);
                        MSG_WriteBits(msg, trunc + (1i32 << 13i32 - 1i32),
                                      13i32);
                    } else {
                        MSG_WriteBits(msg, 1i32, 1i32);
                        MSG_WriteBits(msg, *toF, 32i32);
                    }
                }
            } else if *toF == 0i32 {
                MSG_WriteBits(msg, 0i32, 1i32);
            } else {
                MSG_WriteBits(msg, 1i32, 1i32);
                MSG_WriteBits(msg, *toF, (*field).bits);
            }
        }
        i += 1;
        field = field.offset(1isize)
    };
}
// Initialized in run_static_initializers
#[no_mangle]
pub static mut entityStateFields: [netField_t; 51] =
    [netField_t{name: 0 as *const libc::c_char as *mut libc::c_char,
                offset: 0,
                bits: 0,}; 51];
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaEntity(mut msg: *mut msg_t,
                                             mut from: *mut entityState_t,
                                             mut to: *mut entityState_t,
                                             mut number: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut numFields: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut print: libc::c_int = 0;
    let mut trunc: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut endBit: libc::c_int = 0;
    if number < 0i32 || number >= 1i32 << 10i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Bad delta entity number: %i\x00" as *const u8 as
                      *const libc::c_char, number);
    }
    if (*msg).bit == 0i32 {
        startBit = (*msg).readcount * 8i32 - 10i32
    } else {
        startBit = ((*msg).readcount - 1i32) * 8i32 + (*msg).bit - 10i32
    }
    if MSG_ReadBits(msg, 1i32) == 1i32 {
        memset(to as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<entityState_t>() as libc::c_ulong);
        (*to).number = (1i32 << 10i32) - 1i32;
        if !cl_shownet.is_null() &&
               ((*cl_shownet).integer >= 2i32 ||
                    (*cl_shownet).integer == -1i32) {
            Com_Printf(b"%3i: #%-3i remove\n\x00" as *const u8 as
                           *const libc::c_char, (*msg).readcount, number);
        }
        return
    }
    if MSG_ReadBits(msg, 1i32) == 0i32 {
        *to = *from;
        (*to).number = number;
        return
    }
    numFields =
        (::std::mem::size_of::<[netField_t; 51]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<netField_t>()
                                             as libc::c_ulong) as libc::c_int;
    lc = MSG_ReadByte(msg);
    if lc > numFields || lc < 0i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"invalid entityState field count\x00" as *const u8 as
                      *const libc::c_char);
    }
    if !cl_shownet.is_null() &&
           ((*cl_shownet).integer >= 2i32 || (*cl_shownet).integer == -1i32) {
        print = 1i32;
        Com_Printf(b"%3i: #%-3i \x00" as *const u8 as *const libc::c_char,
                   (*msg).readcount, (*to).number);
    } else { print = 0i32 }
    (*to).number = number;
    i = 0i32;
    field = entityStateFields.as_mut_ptr();
    while i < lc {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        if 0 == MSG_ReadBits(msg, 1i32) {
            *toF = *fromF
        } else if (*field).bits == 0i32 {
            if MSG_ReadBits(msg, 1i32) == 0i32 {
                *(toF as *mut libc::c_float) = 0.0f32
            } else if MSG_ReadBits(msg, 1i32) == 0i32 {
                trunc = MSG_ReadBits(msg, 13i32);
                trunc -= 1i32 << 13i32 - 1i32;
                *(toF as *mut libc::c_float) = trunc as libc::c_float;
                if 0 != print {
                    Com_Printf(b"%s:%i \x00" as *const u8 as
                                   *const libc::c_char, (*field).name, trunc);
                }
            } else {
                *toF = MSG_ReadBits(msg, 32i32);
                if 0 != print {
                    Com_Printf(b"%s:%f \x00" as *const u8 as
                                   *const libc::c_char, (*field).name,
                               *(toF as *mut libc::c_float) as
                                   libc::c_double);
                }
            }
        } else if MSG_ReadBits(msg, 1i32) == 0i32 {
            *toF = 0i32
        } else {
            *toF = MSG_ReadBits(msg, (*field).bits);
            if 0 != print {
                Com_Printf(b"%s:%i \x00" as *const u8 as *const libc::c_char,
                           (*field).name, *toF);
            }
        }
        i += 1;
        field = field.offset(1isize)
    }
    i = lc;
    field =
        &mut *entityStateFields.as_mut_ptr().offset(lc as isize) as
            *mut netField_t;
    while i < numFields {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        *toF = *fromF;
        i += 1;
        field = field.offset(1isize)
    }
    if 0 != print {
        if (*msg).bit == 0i32 {
            endBit = (*msg).readcount * 8i32 - 10i32
        } else {
            endBit = ((*msg).readcount - 1i32) * 8i32 + (*msg).bit - 10i32
        }
        Com_Printf(b" (%i bits)\n\x00" as *const u8 as *const libc::c_char,
                   endBit - startBit);
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaPlayerstate(mut msg: *mut msg_t,
                                                   mut from:
                                                       *mut playerState_s,
                                                   mut to:
                                                       *mut playerState_s) {
    let mut i: libc::c_int = 0;
    let mut dummy: playerState_t =
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
                      entityEventSequence: 0,};
    let mut statsbits: libc::c_int = 0;
    let mut persistantbits: libc::c_int = 0;
    let mut ammobits: libc::c_int = 0;
    let mut powerupbits: libc::c_int = 0;
    let mut numFields: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fullFloat: libc::c_float = 0.;
    let mut trunc: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    if from.is_null() {
        from = &mut dummy;
        memset(&mut dummy as *mut playerState_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<playerState_t>() as libc::c_ulong);
    }
    numFields =
        (::std::mem::size_of::<[netField_t; 48]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<netField_t>()
                                             as libc::c_ulong) as libc::c_int;
    lc = 0i32;
    i = 0i32;
    field = playerStateFields.as_mut_ptr();
    while i < numFields {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        if *fromF != *toF { lc = i + 1i32 }
        i += 1;
        field = field.offset(1isize)
    }
    MSG_WriteByte(msg, lc);
    oldsize += numFields - lc;
    i = 0i32;
    field = playerStateFields.as_mut_ptr();
    while i < lc {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        if *fromF == *toF {
            MSG_WriteBits(msg, 0i32, 1i32);
        } else {
            MSG_WriteBits(msg, 1i32, 1i32);
            if (*field).bits == 0i32 {
                fullFloat = *(toF as *mut libc::c_float);
                trunc = fullFloat as libc::c_int;
                if trunc as libc::c_float == fullFloat &&
                       trunc + (1i32 << 13i32 - 1i32) >= 0i32 &&
                       trunc + (1i32 << 13i32 - 1i32) < 1i32 << 13i32 {
                    MSG_WriteBits(msg, 0i32, 1i32);
                    MSG_WriteBits(msg, trunc + (1i32 << 13i32 - 1i32), 13i32);
                } else {
                    MSG_WriteBits(msg, 1i32, 1i32);
                    MSG_WriteBits(msg, *toF, 32i32);
                }
            } else { MSG_WriteBits(msg, *toF, (*field).bits); }
        }
        i += 1;
        field = field.offset(1isize)
    }
    statsbits = 0i32;
    i = 0i32;
    while i < 16i32 {
        if (*to).stats[i as usize] != (*from).stats[i as usize] {
            statsbits |= 1i32 << i
        }
        i += 1
    }
    persistantbits = 0i32;
    i = 0i32;
    while i < 16i32 {
        if (*to).persistant[i as usize] != (*from).persistant[i as usize] {
            persistantbits |= 1i32 << i
        }
        i += 1
    }
    ammobits = 0i32;
    i = 0i32;
    while i < 16i32 {
        if (*to).ammo[i as usize] != (*from).ammo[i as usize] {
            ammobits |= 1i32 << i
        }
        i += 1
    }
    powerupbits = 0i32;
    i = 0i32;
    while i < 16i32 {
        if (*to).powerups[i as usize] != (*from).powerups[i as usize] {
            powerupbits |= 1i32 << i
        }
        i += 1
    }
    if 0 == statsbits && 0 == persistantbits && 0 == ammobits &&
           0 == powerupbits {
        MSG_WriteBits(msg, 0i32, 1i32);
        oldsize += 4i32;
        return
    }
    MSG_WriteBits(msg, 1i32, 1i32);
    if 0 != statsbits {
        MSG_WriteBits(msg, 1i32, 1i32);
        MSG_WriteBits(msg, statsbits, 16i32);
        i = 0i32;
        while i < 16i32 {
            if 0 != statsbits & 1i32 << i {
                MSG_WriteShort(msg, (*to).stats[i as usize]);
            }
            i += 1
        }
    } else { MSG_WriteBits(msg, 0i32, 1i32); }
    if 0 != persistantbits {
        MSG_WriteBits(msg, 1i32, 1i32);
        MSG_WriteBits(msg, persistantbits, 16i32);
        i = 0i32;
        while i < 16i32 {
            if 0 != persistantbits & 1i32 << i {
                MSG_WriteShort(msg, (*to).persistant[i as usize]);
            }
            i += 1
        }
    } else { MSG_WriteBits(msg, 0i32, 1i32); }
    if 0 != ammobits {
        MSG_WriteBits(msg, 1i32, 1i32);
        MSG_WriteBits(msg, ammobits, 16i32);
        i = 0i32;
        while i < 16i32 {
            if 0 != ammobits & 1i32 << i {
                MSG_WriteShort(msg, (*to).ammo[i as usize]);
            }
            i += 1
        }
    } else { MSG_WriteBits(msg, 0i32, 1i32); }
    if 0 != powerupbits {
        MSG_WriteBits(msg, 1i32, 1i32);
        MSG_WriteBits(msg, powerupbits, 16i32);
        i = 0i32;
        while i < 16i32 {
            if 0 != powerupbits & 1i32 << i {
                MSG_WriteLong(msg, (*to).powerups[i as usize]);
            }
            i += 1
        }
    } else { MSG_WriteBits(msg, 0i32, 1i32); };
}
// Initialized in run_static_initializers
#[no_mangle]
pub static mut playerStateFields: [netField_t; 48] =
    [netField_t{name: 0 as *const libc::c_char as *mut libc::c_char,
                offset: 0,
                bits: 0,}; 48];
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaPlayerstate(mut msg: *mut msg_t,
                                                  mut from:
                                                      *mut playerState_t,
                                                  mut to:
                                                      *mut playerState_t) {
    let mut i: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut numFields: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut endBit: libc::c_int = 0;
    let mut print: libc::c_int = 0;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut trunc: libc::c_int = 0;
    let mut dummy: playerState_t =
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
                      entityEventSequence: 0,};
    if from.is_null() {
        from = &mut dummy;
        memset(&mut dummy as *mut playerState_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<playerState_t>() as libc::c_ulong);
    }
    *to = *from;
    if (*msg).bit == 0i32 {
        startBit = (*msg).readcount * 8i32 - 10i32
    } else {
        startBit = ((*msg).readcount - 1i32) * 8i32 + (*msg).bit - 10i32
    }
    if !cl_shownet.is_null() &&
           ((*cl_shownet).integer >= 2i32 || (*cl_shownet).integer == -2i32) {
        print = 1i32;
        Com_Printf(b"%3i: playerstate \x00" as *const u8 as
                       *const libc::c_char, (*msg).readcount);
    } else { print = 0i32 }
    numFields =
        (::std::mem::size_of::<[netField_t; 48]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<netField_t>()
                                             as libc::c_ulong) as libc::c_int;
    lc = MSG_ReadByte(msg);
    if lc > numFields || lc < 0i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"invalid playerState field count\x00" as *const u8 as
                      *const libc::c_char);
    }
    i = 0i32;
    field = playerStateFields.as_mut_ptr();
    while i < lc {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        if 0 == MSG_ReadBits(msg, 1i32) {
            *toF = *fromF
        } else if (*field).bits == 0i32 {
            if MSG_ReadBits(msg, 1i32) == 0i32 {
                trunc = MSG_ReadBits(msg, 13i32);
                trunc -= 1i32 << 13i32 - 1i32;
                *(toF as *mut libc::c_float) = trunc as libc::c_float;
                if 0 != print {
                    Com_Printf(b"%s:%i \x00" as *const u8 as
                                   *const libc::c_char, (*field).name, trunc);
                }
            } else {
                *toF = MSG_ReadBits(msg, 32i32);
                if 0 != print {
                    Com_Printf(b"%s:%f \x00" as *const u8 as
                                   *const libc::c_char, (*field).name,
                               *(toF as *mut libc::c_float) as
                                   libc::c_double);
                }
            }
        } else {
            *toF = MSG_ReadBits(msg, (*field).bits);
            if 0 != print {
                Com_Printf(b"%s:%i \x00" as *const u8 as *const libc::c_char,
                           (*field).name, *toF);
            }
        }
        i += 1;
        field = field.offset(1isize)
    }
    i = lc;
    field =
        &mut *playerStateFields.as_mut_ptr().offset(lc as isize) as
            *mut netField_t;
    while i < numFields {
        fromF =
            (from as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        toF =
            (to as *mut byte).offset((*field).offset as isize) as
                *mut libc::c_int;
        *toF = *fromF;
        i += 1;
        field = field.offset(1isize)
    }
    if 0 != MSG_ReadBits(msg, 1i32) {
        if 0 != MSG_ReadBits(msg, 1i32) {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4i32 {
                Com_Printf(b"%s \x00" as *const u8 as *const libc::c_char,
                           b"PS_STATS\x00" as *const u8 as
                               *const libc::c_char);
            }
            bits = MSG_ReadBits(msg, 16i32);
            i = 0i32;
            while i < 16i32 {
                if 0 != bits & 1i32 << i {
                    (*to).stats[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        if 0 != MSG_ReadBits(msg, 1i32) {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4i32 {
                Com_Printf(b"%s \x00" as *const u8 as *const libc::c_char,
                           b"PS_PERSISTANT\x00" as *const u8 as
                               *const libc::c_char);
            }
            bits = MSG_ReadBits(msg, 16i32);
            i = 0i32;
            while i < 16i32 {
                if 0 != bits & 1i32 << i {
                    (*to).persistant[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        if 0 != MSG_ReadBits(msg, 1i32) {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4i32 {
                Com_Printf(b"%s \x00" as *const u8 as *const libc::c_char,
                           b"PS_AMMO\x00" as *const u8 as
                               *const libc::c_char);
            }
            bits = MSG_ReadBits(msg, 16i32);
            i = 0i32;
            while i < 16i32 {
                if 0 != bits & 1i32 << i {
                    (*to).ammo[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        if 0 != MSG_ReadBits(msg, 1i32) {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4i32 {
                Com_Printf(b"%s \x00" as *const u8 as *const libc::c_char,
                           b"PS_POWERUPS\x00" as *const u8 as
                               *const libc::c_char);
            }
            bits = MSG_ReadBits(msg, 16i32);
            i = 0i32;
            while i < 16i32 {
                if 0 != bits & 1i32 << i {
                    (*to).powerups[i as usize] = MSG_ReadLong(msg)
                }
                i += 1
            }
        }
    }
    if 0 != print {
        if (*msg).bit == 0i32 {
            endBit = (*msg).readcount * 8i32 - 10i32
        } else {
            endBit = ((*msg).readcount - 1i32) * 8i32 + (*msg).bit - 10i32
        }
        Com_Printf(b" (%i bits)\n\x00" as *const u8 as *const libc::c_char,
                   endBit - startBit);
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReportChangeVectors_f() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 256i32 {
        if 0 != pcount[i as usize] {
            Com_Printf(b"%d used %d\n\x00" as *const u8 as
                           *const libc::c_char, i, pcount[i as usize]);
        }
        i += 1
    };
}
#[no_mangle]
pub static mut pcount: [libc::c_int; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteAngle(mut sb: *mut msg_t,
                                        mut f: libc::c_float) {
    MSG_WriteByte(sb,
                  (f * 256i32 as libc::c_float / 360i32 as libc::c_float) as
                      libc::c_int & 255i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaKeyFloat(mut msg: *mut msg_t,
                                                mut key: libc::c_int,
                                                mut oldV: libc::c_float,
                                                mut newV: libc::c_float) {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    if oldV == newV { MSG_WriteBits(msg, 0i32, 1i32); return }
    fi.f = newV;
    MSG_WriteBits(msg, 1i32, 1i32);
    MSG_WriteBits(msg, fi.i ^ key, 32i32);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaKeyFloat(mut msg: *mut msg_t,
                                               mut key: libc::c_int,
                                               mut oldV: libc::c_float)
 -> libc::c_float {
    if 0 != MSG_ReadBits(msg, 1i32) {
        let mut fi: floatint_t = floatint_t{f: 0.,};
        fi.i = MSG_ReadBits(msg, 32i32) ^ key;
        return fi.f
    }
    return oldV;
}
unsafe extern "C" fn run_static_initializers() {
    entityStateFields =
        [netField_t{name:
                        b"pos.trTime\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).pos.trTime as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"pos.trBase[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).pos.trBase.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"pos.trBase[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).pos.trBase.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"pos.trDelta[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).pos.trDelta.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"pos.trDelta[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).pos.trDelta.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"pos.trBase[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).pos.trBase.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"apos.trBase[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).apos.trBase.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"pos.trDelta[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).pos.trDelta.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"apos.trBase[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).apos.trBase.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"event\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).event as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"angles2[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).angles2.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"eType\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).eType as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"torsoAnim\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).torsoAnim as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"eventParm\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).eventParm as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"legsAnim\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).legsAnim as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"groundEntityNum\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).groundEntityNum as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"pos.trType\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).pos.trType as
                            *mut trType_t as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"eFlags\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).eFlags as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 19i32,},
         netField_t{name:
                        b"otherEntityNum\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).otherEntityNum as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"weapon\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).weapon as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"clientNum\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).clientNum as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"angles[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).angles.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"pos.trDuration\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).pos.trDuration as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"apos.trType\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).apos.trType as
                            *mut trType_t as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"origin[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).origin.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"origin[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).origin.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"origin[2]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).origin.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"solid\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).solid as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 24i32,},
         netField_t{name:
                        b"powerups\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).powerups as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"modelindex\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).modelindex as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"otherEntityNum2\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).otherEntityNum2 as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"loopSound\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).loopSound as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"generic1\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).generic1 as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"origin2[2]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).origin2.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"origin2[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).origin2.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"origin2[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).origin2.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"modelindex2\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).modelindex2 as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"angles[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).angles.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"time\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).time as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"apos.trTime\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).apos.trTime as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"apos.trDuration\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).apos.trDuration as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"apos.trBase[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).apos.trBase.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"apos.trDelta[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).apos.trDelta.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"apos.trDelta[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).apos.trDelta.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"apos.trDelta[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).apos.trDelta.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"time2\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).time2 as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"angles[2]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).angles.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"angles2[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).angles2.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"angles2[2]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut entityState_t)).angles2.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"constantLight\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).constantLight as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"frame\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut entityState_t)).frame as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,}];
    playerStateFields =
        [netField_t{name:
                        b"commandTime\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).commandTime as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 32i32,},
         netField_t{name:
                        b"origin[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).origin.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"origin[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).origin.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"bobCycle\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).bobCycle as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"velocity[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).velocity.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"velocity[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).velocity.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"viewangles[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).viewangles.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"viewangles[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).viewangles.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"weaponTime\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).weaponTime as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: -16i32,},
         netField_t{name:
                        b"origin[2]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).origin.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"velocity[2]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).velocity.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"legsTimer\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).legsTimer as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"pm_time\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).pm_time as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: -16i32,},
         netField_t{name:
                        b"eventSequence\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).eventSequence as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"torsoAnim\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).torsoAnim as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"movementDir\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).movementDir as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 4i32,},
         netField_t{name:
                        b"events[0]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).events.as_mut_ptr().offset(0isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"legsAnim\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).legsAnim as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"events[1]\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).events.as_mut_ptr().offset(1isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"pm_flags\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).pm_flags as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"groundEntityNum\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).groundEntityNum as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"weaponstate\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).weaponstate as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 4i32,},
         netField_t{name:
                        b"eFlags\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).eFlags as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"externalEvent\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).externalEvent as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"gravity\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).gravity as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"speed\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).speed as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"delta_angles[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).delta_angles.as_mut_ptr().offset(1isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"externalEventParm\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).externalEventParm as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"viewheight\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).viewheight as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: -8i32,},
         netField_t{name:
                        b"damageEvent\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).damageEvent as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"damageYaw\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).damageYaw as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"damagePitch\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).damagePitch as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"damageCount\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).damageCount as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"generic1\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).generic1 as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"pm_type\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).pm_type as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"delta_angles[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).delta_angles.as_mut_ptr().offset(0isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"delta_angles[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).delta_angles.as_mut_ptr().offset(2isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,},
         netField_t{name:
                        b"torsoTimer\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).torsoTimer as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 12i32,},
         netField_t{name:
                        b"eventParms[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).eventParms.as_mut_ptr().offset(0isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"eventParms[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).eventParms.as_mut_ptr().offset(1isize)
                            as *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"clientNum\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).clientNum as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 8i32,},
         netField_t{name:
                        b"weapon\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).weapon as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 5i32,},
         netField_t{name:
                        b"viewangles[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).viewangles.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"grapplePoint[0]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).grapplePoint.as_mut_ptr().offset(0isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"grapplePoint[1]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).grapplePoint.as_mut_ptr().offset(1isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"grapplePoint[2]\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &mut *(*(0 as
                                     *mut playerState_t)).grapplePoint.as_mut_ptr().offset(2isize)
                            as *mut vec_t as size_t as libc::c_int,
                    bits: 0i32,},
         netField_t{name:
                        b"jumppad_ent\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).jumppad_ent as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 10i32,},
         netField_t{name:
                        b"loopSound\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut playerState_t)).loopSound as
                            *mut libc::c_int as size_t as libc::c_int,
                    bits: 16i32,}]
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];