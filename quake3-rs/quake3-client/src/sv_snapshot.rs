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
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
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
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
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
    use super::q_shared_h::{qboolean, byte, entityState_s, playerState_s,
                            cvar_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn MSG_Init(buf: *mut msg_t, data: *mut byte,
                        length: libc::c_int);
        #[no_mangle]
        pub fn MSG_Clear(buf: *mut msg_t);
        #[no_mangle]
        pub fn MSG_WriteData(buf: *mut msg_t, data: *const libc::c_void,
                             length: libc::c_int);
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
        pub fn MSG_WriteDeltaEntity(msg: *mut msg_t, from: *mut entityState_s,
                                    to: *mut entityState_s, force: qboolean);
        #[no_mangle]
        pub fn MSG_WriteDeltaPlayerstate(msg: *mut msg_t,
                                         from: *mut playerState_s,
                                         to: *mut playerState_s);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub static mut com_timescale: *mut cvar_t;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        //Does NOT parse port numbers, only base addresses.
        #[no_mangle]
        pub fn Sys_IsLANAddress(adr: netadr_t) -> qboolean;
    }
}
#[header_src =
      "ioq3/code/game/g_public.h"]
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
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
    use super::{libc};
}
#[header_src =
      "ioq3/code/server/server.h"]
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
    use super::{libc};
    use super::q_shared_h::{byte, entityState_t, qboolean, playerState_t,
                            usercmd_t, fileHandle_t, cvar_t};
    use super::g_public_h::{sharedEntity_t};
    use super::qcommon_h::{msg_t, netchan_t, netadr_t};
    extern "C" {
        pub type worldSector_s;
        //=============================================================================
        // persistant server info across maps
        #[no_mangle]
        pub static mut svs: serverStatic_t;
        // cleared each map
        #[no_mangle]
        pub static mut sv: server_t;
        #[no_mangle]
        pub static mut sv_maxclients: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_padPackets: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_lanForceRate: *mut cvar_t;
        #[no_mangle]
        pub fn SV_RateMsec(client: *mut client_t) -> libc::c_int;
        // clip to a specific entity
        //
// sv_net_chan.c
//
        #[no_mangle]
        pub fn SV_Netchan_Transmit(client: *mut client_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn SV_GentityNum(num: libc::c_int) -> *mut sharedEntity_t;
        #[no_mangle]
        pub fn SV_SvEntityForGentity(gEnt: *mut sharedEntity_t)
         -> *mut svEntity_t;
        #[no_mangle]
        pub fn SV_GameClientNum(num: libc::c_int) -> *mut playerState_t;
    }
}
#[header_src =
      "ioq3/code/server/sv_snapshot.c"]
pub mod sv_snapshot_c {
    /*
=============================================================================

Build a client snapshot structure

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snapshotEntityNumbers_t {
        pub numSnapshotEntities: libc::c_int,
        pub snapshotEntities: [libc::c_int; 256],
    }
    use super::{libc};
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
#[header_src =
      "ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::q_shared_h::{byte, vec_t, qboolean};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CM_ClusterPVS(cluster: libc::c_int) -> *mut byte;
        #[no_mangle]
        pub fn CM_PointLeafnum(p: *const vec_t) -> libc::c_int;
        #[no_mangle]
        pub fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn CM_AreasConnected(area1: libc::c_int, area2: libc::c_int)
         -> qboolean;
        #[no_mangle]
        pub fn CM_WriteAreaBits(buffer: *mut byte, area: libc::c_int)
         -> libc::c_int;
    }
}
use self::stddef_h::{size_t};
use self::stdlib_h::{__compar_fn_t, qsort};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cvar_s, cvar_t,
                       playerState_s, playerState_t, usercmd_s, usercmd_t,
                       trType_t, TR_GRAVITY, TR_SINE, TR_LINEAR_STOP,
                       TR_LINEAR, TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      svc_ops_e, svc_voipOpus, svc_voipSpeex, svc_EOF,
                      svc_snapshot, svc_download, svc_serverCommand,
                      svc_baseline, svc_configstring, svc_gamestate, svc_nop,
                      svc_bad, MSG_Init, MSG_Clear, MSG_WriteData,
                      MSG_WriteBits, MSG_WriteByte, MSG_WriteShort,
                      MSG_WriteLong, MSG_WriteString, MSG_WriteDeltaEntity,
                      MSG_WriteDeltaPlayerstate, Com_DPrintf, com_timescale,
                      Z_Free, Sys_IsLANAddress};
use self::g_public_h::{entityShared_t, sharedEntity_t};
use self::server_h::{voipServerPacket_s, voipServerPacket_t, svEntity_s,
                     svEntity_t, serverState_t, SS_GAME, SS_LOADING, SS_DEAD,
                     server_t, clientSnapshot_t, clientState_t, CS_ACTIVE,
                     CS_PRIMED, CS_CONNECTED, CS_ZOMBIE, CS_FREE,
                     netchan_buffer_s, netchan_buffer_t, client_s, client_t,
                     challenge_t, serverStatic_t, worldSector_s, svs, sv,
                     sv_maxclients, sv_padPackets, sv_lanForceRate,
                     SV_RateMsec, SV_Netchan_Transmit, SV_GentityNum,
                     SV_SvEntityForGentity, SV_GameClientNum};
use self::sv_snapshot_c::{snapshotEntityNumbers_t};
use self::string_h::{memset};
use self::cm_public_h::{CM_ClusterPVS, CM_PointLeafnum, CM_LeafCluster,
                        CM_LeafArea, CM_AreasConnected, CM_WriteAreaBits};
unsafe extern "C" fn VectorLengthSquared(mut v: *const vec_t) -> vec_t {
    return *v.offset(0isize) * *v.offset(0isize) +
               *v.offset(1isize) * *v.offset(1isize) +
               *v.offset(2isize) * *v.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateServerCommandsToClient(mut client:
                                                             *mut client_t,
                                                         mut msg:
                                                             *mut msg_t) {
    let mut i: libc::c_int = 0;
    i = (*client).reliableAcknowledge + 1i32;
    while i <= (*client).reliableSequence {
        MSG_WriteByte(msg, svc_serverCommand as libc::c_int);
        MSG_WriteLong(msg, i);
        MSG_WriteString(msg,
                        (*client).reliableCommands[(i & 64i32 - 1i32) as
                                                       usize].as_mut_ptr());
        i += 1
    }
    (*client).reliableSent = (*client).reliableSequence;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendMessageToClient(mut msg: *mut msg_t,
                                                mut client: *mut client_t) {
    (*client).frames[((*client).netchan.outgoingSequence & 32i32 - 1i32) as
                         usize].messageSize = (*msg).cursize;
    (*client).frames[((*client).netchan.outgoingSequence & 32i32 - 1i32) as
                         usize].messageSent = svs.time;
    (*client).frames[((*client).netchan.outgoingSequence & 32i32 - 1i32) as
                         usize].messageAcked = -1i32;
    SV_Netchan_Transmit(client, msg);
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendClientMessages() {
    let mut i: libc::c_int = 0;
    let mut c: *mut client_t = 0 as *mut client_t;
    let mut current_block_6: u64;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        c = &mut *svs.clients.offset(i as isize) as *mut client_t;
        if !(0 == (*c).state as u64) {
            // not connected
            if !(((svs.time - (*c).lastSnapshotTime) as libc::c_float) <
                     (*c).snapshotMsec as libc::c_float *
                         (*com_timescale).value) {
                // It's not time yet
                if !(0 != *(*c).downloadName.as_mut_ptr()) {
                    // Client is downloading, don't send snapshots
                    if 0 != (*c).netchan.unsentFragments as libc::c_uint ||
                           !(*c).netchan_start_queue.is_null() {
                        (*c).rateDelayed = qtrue
                    } else {
                        // Drop this snapshot if the packet queue is still full or delta compression will break
                        if !((*c).netchan.remoteAddress.type_0 as libc::c_uint
                                 == NA_LOOPBACK as libc::c_int as libc::c_uint
                                 ||
                                 0 != (*sv_lanForceRate).integer &&
                                     0 !=
                                         Sys_IsLANAddress((*c).netchan.remoteAddress)
                                             as libc::c_uint) {
                            // rate control for clients not on LAN 
                            if SV_RateMsec(c) > 0i32 {
                                (*c).rateDelayed = qtrue;
                                current_block_6 = 16559507199688588974;
                            } else { current_block_6 = 12599329904712511516; }
                        } else { current_block_6 = 12599329904712511516; }
                        match current_block_6 {
                            16559507199688588974 => { }
                            _ => {
                                SV_SendClientSnapshot(c);
                                (*c).lastSnapshotTime = svs.time;
                                (*c).rateDelayed = qfalse
                            }
                        }
                    }
                }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendClientSnapshot(mut client: *mut client_t) {
    let mut msg_buf: [byte; 16384] = [0; 16384];
    let mut msg: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    SV_BuildClientSnapshot(client);
    if !(*client).gentity.is_null() &&
           0 != (*(*client).gentity).r.svFlags & 0x8i32 {
        return
    }
    MSG_Init(&mut msg, msg_buf.as_mut_ptr(),
             ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                 libc::c_int);
    msg.allowoverflow = qtrue;
    MSG_WriteLong(&mut msg, (*client).lastClientCommand);
    SV_UpdateServerCommandsToClient(client, &mut msg);
    SV_WriteSnapshotToClient(client, &mut msg);
    SV_WriteVoipToClient(client, &mut msg);
    if 0 != msg.overflowed as u64 {
        Com_Printf(b"WARNING: msg overflowed for %s\n\x00" as *const u8 as
                       *const libc::c_char, (*client).name.as_mut_ptr());
        MSG_Clear(&mut msg);
    }
    SV_SendMessageToClient(&mut msg, client);
}
/*
==================
SV_WriteVoipToClient

Check to see if there is any VoIP queued for a client, and send if there is.
==================
*/
unsafe extern "C" fn SV_WriteVoipToClient(mut cl: *mut client_t,
                                          mut msg: *mut msg_t) {
    let mut totalbytes: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut packet: *mut voipServerPacket_t = 0 as *mut voipServerPacket_t;
    if 0 != (*cl).queuedVoipPackets {
        i = 0i32;
        while i < (*cl).queuedVoipPackets {
            packet =
                (*cl).voipPacket[((i + (*cl).queuedVoipIndex) as
                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<[*mut voipServerPacket_t; 64]>()
                                                                       as
                                                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut voipServerPacket_t>()
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize];
            if 0 == *(*cl).downloadName.as_mut_ptr() {
                totalbytes += (*packet).len;
                if totalbytes > ((*msg).maxsize - (*msg).cursize) / 2i32 {
                    break ;
                }
                MSG_WriteByte(msg, svc_voipOpus as libc::c_int);
                MSG_WriteShort(msg, (*packet).sender);
                MSG_WriteByte(msg,
                              (*packet).generation as byte as libc::c_int);
                MSG_WriteLong(msg, (*packet).sequence);
                MSG_WriteByte(msg, (*packet).frames);
                MSG_WriteShort(msg, (*packet).len);
                MSG_WriteBits(msg, (*packet).flags, 2i32);
                MSG_WriteData(msg,
                              (*packet).data.as_mut_ptr() as
                                  *const libc::c_void, (*packet).len);
            }
            Z_Free(packet as *mut libc::c_void);
            i += 1
        }
        (*cl).queuedVoipPackets -= i;
        (*cl).queuedVoipIndex += i;
        (*cl).queuedVoipIndex =
            ((*cl).queuedVoipIndex as
                 libc::c_ulong).wrapping_rem((::std::mem::size_of::<[*mut voipServerPacket_t; 64]>()
                                                  as
                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut voipServerPacket_t>()
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int
    };
}
/*
==================
SV_WriteSnapshotToClient
==================
*/
unsafe extern "C" fn SV_WriteSnapshotToClient(mut client: *mut client_t,
                                              mut msg: *mut msg_t) {
    let mut frame: *mut clientSnapshot_t = 0 as *mut clientSnapshot_t;
    let mut oldframe: *mut clientSnapshot_t = 0 as *mut clientSnapshot_t;
    let mut lastframe: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut snapFlags: libc::c_int = 0;
    frame =
        &mut *(*client).frames.as_mut_ptr().offset(((*client).netchan.outgoingSequence
                                                        & 32i32 - 1i32) as
                                                       isize) as
            *mut clientSnapshot_t;
    if (*client).deltaMessage <= 0i32 ||
           (*client).state as libc::c_uint !=
               CS_ACTIVE as libc::c_int as libc::c_uint {
        oldframe = 0 as *mut clientSnapshot_t;
        lastframe = 0i32
    } else if (*client).netchan.outgoingSequence - (*client).deltaMessage >=
                  32i32 - 3i32 {
        Com_DPrintf(b"%s: Delta request from out of date packet.\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*client).name.as_mut_ptr());
        oldframe = 0 as *mut clientSnapshot_t;
        lastframe = 0i32
    } else {
        oldframe =
            &mut *(*client).frames.as_mut_ptr().offset(((*client).deltaMessage
                                                            & 32i32 - 1i32) as
                                                           isize) as
                *mut clientSnapshot_t;
        lastframe =
            (*client).netchan.outgoingSequence - (*client).deltaMessage;
        if (*oldframe).first_entity <=
               svs.nextSnapshotEntities - svs.numSnapshotEntities {
            Com_DPrintf(b"%s: Delta request from out of date entities.\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*client).name.as_mut_ptr());
            oldframe = 0 as *mut clientSnapshot_t;
            lastframe = 0i32
        }
    }
    MSG_WriteByte(msg, svc_snapshot as libc::c_int);
    if 0 != (*client).oldServerTime {
        MSG_WriteLong(msg, sv.time + (*client).oldServerTime);
    } else { MSG_WriteLong(msg, sv.time); }
    MSG_WriteByte(msg, lastframe);
    snapFlags = svs.snapFlagServerBit;
    if 0 != (*client).rateDelayed as u64 { snapFlags |= 1i32 }
    if (*client).state as libc::c_uint !=
           CS_ACTIVE as libc::c_int as libc::c_uint {
        snapFlags |= 2i32
    }
    MSG_WriteByte(msg, snapFlags);
    MSG_WriteByte(msg, (*frame).areabytes);
    MSG_WriteData(msg, (*frame).areabits.as_mut_ptr() as *const libc::c_void,
                  (*frame).areabytes);
    if !oldframe.is_null() {
        MSG_WriteDeltaPlayerstate(msg, &mut (*oldframe).ps, &mut (*frame).ps);
    } else {
        MSG_WriteDeltaPlayerstate(msg, 0 as *mut playerState_s,
                                  &mut (*frame).ps);
    }
    SV_EmitPacketEntities(oldframe, frame, msg);
    if 0 != (*sv_padPackets).integer {
        i = 0i32;
        while i < (*sv_padPackets).integer {
            MSG_WriteByte(msg, svc_nop as libc::c_int);
            i += 1
        }
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
/*
=============================================================================

Delta encode a client frame onto the network channel

A normal server packet will look like:

4	sequence number (high bit set if an oversize fragment)
<optional reliable commands>
1	svc_snapshot
4	last client reliable command
4	serverTime
1	lastframe for delta compression
1	snapFlags
1	areaBytes
<areabytes>
<playerstate>
<packetentities>

=============================================================================
*/
/*
=============
SV_EmitPacketEntities

Writes a delta update of an entityState_t list to the message.
=============
*/
unsafe extern "C" fn SV_EmitPacketEntities(mut from: *mut clientSnapshot_t,
                                           mut to: *mut clientSnapshot_t,
                                           mut msg: *mut msg_t) {
    let mut oldent: *mut entityState_t = 0 as *mut entityState_t;
    let mut newent: *mut entityState_t = 0 as *mut entityState_t;
    let mut oldindex: libc::c_int = 0;
    let mut newindex: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut from_num_entities: libc::c_int = 0;
    if from.is_null() {
        from_num_entities = 0i32
    } else { from_num_entities = (*from).num_entities }
    newent = 0 as *mut entityState_t;
    oldent = 0 as *mut entityState_t;
    newindex = 0i32;
    oldindex = 0i32;
    while newindex < (*to).num_entities || oldindex < from_num_entities {
        if newindex >= (*to).num_entities {
            newnum = 9999i32
        } else {
            newent =
                &mut *svs.snapshotEntities.offset((((*to).first_entity +
                                                        newindex) %
                                                       svs.numSnapshotEntities)
                                                      as isize) as
                    *mut entityState_t;
            newnum = (*newent).number
        }
        if oldindex >= from_num_entities {
            oldnum = 9999i32
        } else {
            oldent =
                &mut *svs.snapshotEntities.offset((((*from).first_entity +
                                                        oldindex) %
                                                       svs.numSnapshotEntities)
                                                      as isize) as
                    *mut entityState_t;
            oldnum = (*oldent).number
        }
        if newnum == oldnum {
            MSG_WriteDeltaEntity(msg, oldent, newent, qfalse);
            oldindex += 1;
            newindex += 1
        } else if newnum < oldnum {
            MSG_WriteDeltaEntity(msg,
                                 &mut (*sv.svEntities.as_mut_ptr().offset(newnum
                                                                              as
                                                                              isize)).baseline,
                                 newent, qtrue);
            newindex += 1
        } else {
            if !(newnum > oldnum) { continue ; }
            MSG_WriteDeltaEntity(msg, oldent, 0 as *mut entityState_s, qtrue);
            oldindex += 1
        }
    }
    MSG_WriteBits(msg, (1i32 << 10i32) - 1i32, 10i32);
}
/*
=============
SV_BuildClientSnapshot

Decides which entities are going to be visible to the client, and
copies off the playerstate and areabits.

This properly handles multiple recursive portals, but the render
currently doesn't.

For viewing through other player's eyes, clent can be something other than client->gentity
=============
*/
unsafe extern "C" fn SV_BuildClientSnapshot(mut client: *mut client_t) {
    let mut org: vec3_t = [0.; 3];
    let mut frame: *mut clientSnapshot_t = 0 as *mut clientSnapshot_t;
    let mut entityNumbers: snapshotEntityNumbers_t =
        snapshotEntityNumbers_t{numSnapshotEntities: 0,
                                snapshotEntities: [0; 256],};
    let mut i: libc::c_int = 0;
    let mut ent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut state: *mut entityState_t = 0 as *mut entityState_t;
    let mut svEnt: *mut svEntity_t = 0 as *mut svEntity_t;
    let mut clent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut clientNum: libc::c_int = 0;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    sv.snapshotCounter += 1;
    frame =
        &mut *(*client).frames.as_mut_ptr().offset(((*client).netchan.outgoingSequence
                                                        & 32i32 - 1i32) as
                                                       isize) as
            *mut clientSnapshot_t;
    entityNumbers.numSnapshotEntities = 0i32;
    memset((*frame).areabits.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[byte; 32]>() as libc::c_ulong);
    (*frame).num_entities = 0i32;
    clent = (*client).gentity;
    if clent.is_null() ||
           (*client).state as libc::c_uint ==
               CS_ZOMBIE as libc::c_int as libc::c_uint {
        return
    }
    ps =
        SV_GameClientNum(client.wrapping_offset_from(svs.clients) as
                             libc::c_long as libc::c_int);
    (*frame).ps = *ps;
    clientNum = (*frame).ps.clientNum;
    if clientNum < 0i32 || clientNum >= 1i32 << 10i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_SvEntityForGentity: bad gEnt\x00" as *const u8 as
                      *const libc::c_char);
    }
    svEnt =
        &mut *sv.svEntities.as_mut_ptr().offset(clientNum as isize) as
            *mut svEntity_t;
    (*svEnt).snapshotCounter = sv.snapshotCounter;
    org[0usize] = (*ps).origin[0usize];
    org[1usize] = (*ps).origin[1usize];
    org[2usize] = (*ps).origin[2usize];
    org[2usize] += (*ps).viewheight as libc::c_float;
    SV_AddEntitiesVisibleFromPoint(org.as_mut_ptr(), frame,
                                   &mut entityNumbers, qfalse);
    qsort(entityNumbers.snapshotEntities.as_mut_ptr() as *mut libc::c_void,
          entityNumbers.numSnapshotEntities as size_t,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
          Some(SV_QsortEntityNumbers));
    i = 0i32;
    while i < 32i32 / 4i32 {
        *((*frame).areabits.as_mut_ptr() as
              *mut libc::c_int).offset(i as isize) =
            *((*frame).areabits.as_mut_ptr() as
                  *mut libc::c_int).offset(i as isize) ^ -1i32;
        i += 1
    }
    (*frame).num_entities = 0i32;
    (*frame).first_entity = svs.nextSnapshotEntities;
    i = 0i32;
    while i < entityNumbers.numSnapshotEntities {
        ent = SV_GentityNum(entityNumbers.snapshotEntities[i as usize]);
        state =
            &mut *svs.snapshotEntities.offset((svs.nextSnapshotEntities %
                                                   svs.numSnapshotEntities) as
                                                  isize) as
                *mut entityState_t;
        *state = (*ent).s;
        svs.nextSnapshotEntities += 1;
        if svs.nextSnapshotEntities >= 0x7ffffffei32 {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"svs.nextSnapshotEntities wrapped\x00" as *const u8 as
                          *const libc::c_char);
        }
        (*frame).num_entities += 1;
        i += 1
    };
}
/*
=======================
SV_QsortEntityNumbers
=======================
*/
unsafe extern "C" fn SV_QsortEntityNumbers(mut a: *const libc::c_void,
                                           mut b: *const libc::c_void)
 -> libc::c_int {
    let mut ea: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut eb: *mut libc::c_int = 0 as *mut libc::c_int;
    ea = a as *mut libc::c_int;
    eb = b as *mut libc::c_int;
    if *ea == *eb {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_QsortEntityStates: duplicated entity\x00" as *const u8
                      as *const libc::c_char);
    }
    if *ea < *eb { return -1i32 }
    return 1i32;
}
/*
===============
SV_AddEntitiesVisibleFromPoint
===============
*/
unsafe extern "C" fn SV_AddEntitiesVisibleFromPoint(mut origin: *mut vec_t,
                                                    mut frame:
                                                        *mut clientSnapshot_t,
                                                    mut eNums:
                                                        *mut snapshotEntityNumbers_t,
                                                    mut portal: qboolean) {
    let mut e: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut svEnt: *mut svEntity_t = 0 as *mut svEntity_t;
    let mut l: libc::c_int = 0;
    let mut clientarea: libc::c_int = 0;
    let mut clientcluster: libc::c_int = 0;
    let mut leafnum: libc::c_int = 0;
    let mut clientpvs: *mut byte = 0 as *mut byte;
    let mut bitvector: *mut byte = 0 as *mut byte;
    if 0 == sv.state as u64 { return }
    leafnum = CM_PointLeafnum(origin as *const vec_t);
    clientarea = CM_LeafArea(leafnum);
    clientcluster = CM_LeafCluster(leafnum);
    (*frame).areabytes =
        CM_WriteAreaBits((*frame).areabits.as_mut_ptr(), clientarea);
    clientpvs = CM_ClusterPVS(clientcluster);
    let mut current_block_26: u64;
    e = 0i32;
    while e < sv.num_entities {
        ent = SV_GentityNum(e);
        // never send entities that aren't linked in
        if !(0 == (*ent).r.linked as u64) {
            if (*ent).s.number != e {
                Com_DPrintf(b"FIXING ENT->S.NUMBER!!!\n\x00" as *const u8 as
                                *const libc::c_char);
                (*ent).s.number = e
            }
            // entities can be flagged to explicitly not be sent to the client
            if !(0 != (*ent).r.svFlags & 0x1i32) {
                // entities can be flagged to be sent to only one client
                if 0 != (*ent).r.svFlags & 0x100i32 {
                    if (*ent).r.singleClient != (*frame).ps.clientNum {
                        current_block_26 = 7651349459974463963;
                    } else { current_block_26 = 13472856163611868459; }
                } else { current_block_26 = 13472856163611868459; }
                match current_block_26 {
                    7651349459974463963 => { }
                    _ => {
                        // entities can be flagged to be sent to everyone but one client
                        if 0 != (*ent).r.svFlags & 0x800i32 {
                            if (*ent).r.singleClient == (*frame).ps.clientNum
                               {
                                current_block_26 = 7651349459974463963;
                            } else {
                                current_block_26 = 13550086250199790493;
                            }
                        } else { current_block_26 = 13550086250199790493; }
                        match current_block_26 {
                            7651349459974463963 => { }
                            _ => {
                                // entities can be flagged to be sent to a given mask of clients
                                if 0 != (*ent).r.svFlags & 0x2i32 {
                                    if (*frame).ps.clientNum >= 32i32 {
                                        Com_Error(ERR_DROP as libc::c_int,
                                                  b"SVF_CLIENTMASK: clientNum >= 32\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    }
                                    if 0 !=
                                           !(*ent).r.singleClient &
                                               1i32 << (*frame).ps.clientNum {
                                        current_block_26 =
                                            7651349459974463963;
                                    } else {
                                        current_block_26 = 652864300344834934;
                                    }
                                } else {
                                    current_block_26 = 652864300344834934;
                                }
                                match current_block_26 {
                                    7651349459974463963 => { }
                                    _ => {
                                        svEnt = SV_SvEntityForGentity(ent);
                                        // don't double add an entity through portals
                                        if !((*svEnt).snapshotCounter ==
                                                 sv.snapshotCounter) {
                                            // broadcast entities are always sent
                                            if 0 != (*ent).r.svFlags & 0x20i32
                                               {
                                                SV_AddEntToSnapshot(svEnt,
                                                                    ent,
                                                                    eNums);
                                            } else {
                                                // ignore if not touching a PV leaf
		// check area
                                                if 0 ==
                                                       CM_AreasConnected(clientarea,
                                                                         (*svEnt).areanum)
                                                           as u64 {
                                                    // doors can legally straddle two areas, so
			// we may need to check another one
                                                    if 0 ==
                                                           CM_AreasConnected(clientarea,
                                                                             (*svEnt).areanum2)
                                                               as u64 {
                                                        // blocked by a door
                                                        current_block_26 =
                                                            7651349459974463963;
                                                    } else {
                                                        current_block_26 =
                                                            7427571413727699167;
                                                    }
                                                } else {
                                                    current_block_26 =
                                                        7427571413727699167;
                                                }
                                                match current_block_26 {
                                                    7651349459974463963 => { }
                                                    _ => {
                                                        bitvector = clientpvs;
                                                        // check individual leafs
                                                        if !(0 ==
                                                                 (*svEnt).numClusters)
                                                           {
                                                            l = 0i32;
                                                            i = 0i32;
                                                            while i <
                                                                      (*svEnt).numClusters
                                                                  {
                                                                l =
                                                                    (*svEnt).clusternums[i
                                                                                             as
                                                                                             usize];
                                                                if 0 !=
                                                                       *bitvector.offset((l
                                                                                              >>
                                                                                              3i32)
                                                                                             as
                                                                                             isize)
                                                                           as
                                                                           libc::c_int
                                                                           &
                                                                           1i32
                                                                               <<
                                                                               (l
                                                                                    &
                                                                                    7i32)
                                                                   {
                                                                    break ;
                                                                }
                                                                i += 1
                                                            }
                                                            // if we haven't found it to be visible,
		// check overflow clusters that coudln't be stored
                                                            if i ==
                                                                   (*svEnt).numClusters
                                                               {
                                                                if 0 !=
                                                                       (*svEnt).lastCluster
                                                                   {
                                                                    while l <=
                                                                              (*svEnt).lastCluster
                                                                          {
                                                                        if 0
                                                                               !=
                                                                               *bitvector.offset((l
                                                                                                      >>
                                                                                                      3i32)
                                                                                                     as
                                                                                                     isize)
                                                                                   as
                                                                                   libc::c_int
                                                                                   &
                                                                                   1i32
                                                                                       <<
                                                                                       (l
                                                                                            &
                                                                                            7i32)
                                                                           {
                                                                            break
                                                                                ;
                                                                        }
                                                                        l += 1
                                                                    }
                                                                    if l ==
                                                                           (*svEnt).lastCluster
                                                                       {
                                                                        // not visible
                                                                        current_block_26
                                                                            =
                                                                            7651349459974463963;
                                                                    } else {
                                                                        current_block_26
                                                                            =
                                                                            9512719473022792396;
                                                                    }
                                                                } else {
                                                                    current_block_26
                                                                        =
                                                                        7651349459974463963;
                                                                }
                                                            } else {
                                                                current_block_26
                                                                    =
                                                                    9512719473022792396;
                                                            }
                                                            match current_block_26
                                                                {
                                                                7651349459974463963
                                                                => {
                                                                }
                                                                _ => {
                                                                    SV_AddEntToSnapshot(svEnt,
                                                                                        ent,
                                                                                        eNums);
                                                                    // if it's a portal entity, add everything visible from its camera position
                                                                    if 0 !=
                                                                           (*ent).r.svFlags
                                                                               &
                                                                               0x40i32
                                                                       {
                                                                        if 0
                                                                               !=
                                                                               (*ent).s.generic1
                                                                           {
                                                                            let mut dir:
                                                                                    vec3_t =
                                                                                [0.;
                                                                                    3];
                                                                            dir[0usize]
                                                                                =
                                                                                (*ent).s.origin[0usize]
                                                                                    -
                                                                                    *origin.offset(0isize);
                                                                            dir[1usize]
                                                                                =
                                                                                (*ent).s.origin[1usize]
                                                                                    -
                                                                                    *origin.offset(1isize);
                                                                            dir[2usize]
                                                                                =
                                                                                (*ent).s.origin[2usize]
                                                                                    -
                                                                                    *origin.offset(2isize);
                                                                            if VectorLengthSquared(dir.as_mut_ptr()
                                                                                                       as
                                                                                                       *const vec_t)
                                                                                   >
                                                                                   (*ent).s.generic1
                                                                                       as
                                                                                       libc::c_float
                                                                                       *
                                                                                       (*ent).s.generic1
                                                                                           as
                                                                                           libc::c_float
                                                                               {
                                                                                current_block_26
                                                                                    =
                                                                                    7651349459974463963;
                                                                            } else {
                                                                                current_block_26
                                                                                    =
                                                                                    8869332144787829186;
                                                                            }
                                                                        } else {
                                                                            current_block_26
                                                                                =
                                                                                8869332144787829186;
                                                                        }
                                                                        match current_block_26
                                                                            {
                                                                            7651349459974463963
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                SV_AddEntitiesVisibleFromPoint((*ent).s.origin2.as_mut_ptr(),
                                                                                                               frame,
                                                                                                               eNums,
                                                                                                               qtrue);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        e += 1
    };
}
/*
===============
SV_AddEntToSnapshot
===============
*/
unsafe extern "C" fn SV_AddEntToSnapshot(mut svEnt: *mut svEntity_t,
                                         mut gEnt: *mut sharedEntity_t,
                                         mut eNums:
                                             *mut snapshotEntityNumbers_t) {
    if (*svEnt).snapshotCounter == sv.snapshotCounter { return }
    (*svEnt).snapshotCounter = sv.snapshotCounter;
    if (*eNums).numSnapshotEntities == 256i32 { return }
    (*eNums).snapshotEntities[(*eNums).numSnapshotEntities as usize] =
        (*gEnt).s.number;
    (*eNums).numSnapshotEntities += 1;
}