#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           libc,
           ptr_wrapping_offset_from)]
extern crate libc;
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
    pub type clipHandle_t = libc::c_int;
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
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn RadiusFromBounds(mins: *const vec_t, maxs: *const vec_t)
         -> libc::c_float;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
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
    use super::sv_world_c::{worldSector_s};
    use super::q_shared_h::{entityState_t, qboolean, playerState_t,
                            clipHandle_t, vec_t, trace_t};
    use super::{libc};
    use super::g_public_h::{sharedEntity_t};
    extern "C" {
        // cleared each map
        #[no_mangle]
        pub static mut sv: server_t;
        #[no_mangle]
        pub fn SV_GentityNum(num: libc::c_int) -> *mut sharedEntity_t;
        #[no_mangle]
        pub fn SV_SvEntityForGentity(gEnt: *mut sharedEntity_t)
         -> *mut svEntity_t;
        #[no_mangle]
        pub fn SV_GEntityForSvEntity(svEnt: *mut svEntity_t)
         -> *mut sharedEntity_t;
    }
}
#[header_src =
      "ioq3/code/server/sv_world.c"]
pub mod sv_world_c {
    /*
===============================================================================

ENTITY CHECKING

To avoid linearly searching through lists of entities during environment testing,
the world is carved up with an evenly spaced, axially aligned bsp tree.  Entities
are kept in chains either at the final leafs, or at the first node that splits
them, which prevents having to deal with multiple fragments of a single entity.

===============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct worldSector_s {
        pub axis: libc::c_int,
        pub dist: libc::c_float,
        pub children: [*mut worldSector_s; 2],
        pub entities: *mut svEntity_t,
    }
    pub type worldSector_t = worldSector_s;
    /*
============================================================================

AREA QUERY

Fills in a list of all entities who's absmin / absmax intersects the given
bounds.  This does NOT mean that they actually touch in the case of bmodels.
============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct areaParms_t {
        pub mins: *const libc::c_float,
        pub maxs: *const libc::c_float,
        pub list: *mut libc::c_int,
        pub count: libc::c_int,
        pub maxcount: libc::c_int,
    }
    //===========================================================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct moveclip_t {
        pub boxmins: vec3_t,
        pub boxmaxs: vec3_t,
        pub mins: *const libc::c_float,
        pub maxs: *const libc::c_float,
        pub start: *const libc::c_float,
        pub end: vec3_t,
        pub trace: trace_t,
        pub passEntityNum: libc::c_int,
        pub contentmask: libc::c_int,
        pub capsule: libc::c_int,
    }
    use super::{libc};
    use super::server_h::{svEntity_t};
    use super::q_shared_h::{vec3_t, trace_t};
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
    use super::q_shared_h::{clipHandle_t, vec_t, trace_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CM_InlineModel(index: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_TempBoxModel(mins: *const vec_t, maxs: *const vec_t,
                               capsule: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_ModelBounds(model: clipHandle_t, mins: *mut vec_t,
                              maxs: *mut vec_t);
        // returns an ORed contents mask
        #[no_mangle]
        pub fn CM_PointContents(p: *const vec_t, model: clipHandle_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn CM_TransformedPointContents(p: *const vec_t,
                                           model: clipHandle_t,
                                           origin: *const vec_t,
                                           angles: *const vec_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn CM_BoxTrace(results: *mut trace_t, start: *const vec_t,
                           end: *const vec_t, mins: *mut vec_t,
                           maxs: *mut vec_t, model: clipHandle_t,
                           brushmask: libc::c_int, capsule: libc::c_int);
        #[no_mangle]
        pub fn CM_TransformedBoxTrace(results: *mut trace_t,
                                      start: *const vec_t, end: *const vec_t,
                                      mins: *mut vec_t, maxs: *mut vec_t,
                                      model: clipHandle_t,
                                      brushmask: libc::c_int,
                                      origin: *const vec_t,
                                      angles: *const vec_t,
                                      capsule: libc::c_int);
        // only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
        #[no_mangle]
        pub fn CM_BoxLeafnums(mins: *const vec_t, maxs: *const vec_t,
                              list: *mut libc::c_int, listsize: libc::c_int,
                              lastLeaf: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, clipHandle_t, vec_t,
                       vec3_t, cplane_s, cplane_t, trace_t, playerState_s,
                       playerState_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, vec3_origin, RadiusFromBounds,
                       Com_Printf};
use self::g_public_h::{entityShared_t, sharedEntity_t};
use self::server_h::{svEntity_s, svEntity_t, serverState_t, SS_GAME,
                     SS_LOADING, SS_DEAD, server_t, sv, SV_GentityNum,
                     SV_SvEntityForGentity, SV_GEntityForSvEntity};
use self::sv_world_c::{worldSector_s, worldSector_t, areaParms_t, moveclip_t};
use self::string_h::{memset};
use self::cm_public_h::{CM_InlineModel, CM_TempBoxModel, CM_ModelBounds,
                        CM_PointContents, CM_TransformedPointContents,
                        CM_BoxTrace, CM_TransformedBoxTrace, CM_BoxLeafnums,
                        CM_LeafCluster, CM_LeafArea};
use self::qcommon_h::{Com_DPrintf};
//============================================================
//
// high level object sorting to reduce interaction tests
//
#[no_mangle]
pub unsafe extern "C" fn SV_ClearWorld() {
    let mut h: clipHandle_t = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    memset(sv_worldSectors.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[worldSector_t; 64]>() as libc::c_ulong);
    sv_numworldSectors = 0i32;
    h = CM_InlineModel(0i32);
    CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    SV_CreateworldSector(0i32, mins.as_mut_ptr(), maxs.as_mut_ptr());
}
/*
===============
SV_CreateworldSector

Builds a uniformly subdivided tree for the given world size
===============
*/
unsafe extern "C" fn SV_CreateworldSector(mut depth: libc::c_int,
                                          mut mins: *mut vec_t,
                                          mut maxs: *mut vec_t)
 -> *mut worldSector_t {
    let mut anode: *mut worldSector_t = 0 as *mut worldSector_t;
    let mut size: vec3_t = [0.; 3];
    let mut mins1: vec3_t = [0.; 3];
    let mut maxs1: vec3_t = [0.; 3];
    let mut mins2: vec3_t = [0.; 3];
    let mut maxs2: vec3_t = [0.; 3];
    anode =
        &mut *sv_worldSectors.as_mut_ptr().offset(sv_numworldSectors as isize)
            as *mut worldSector_t;
    sv_numworldSectors += 1;
    if depth == 4i32 {
        (*anode).axis = -1i32;
        (*anode).children[1usize] = 0 as *mut worldSector_s;
        (*anode).children[0usize] = (*anode).children[1usize];
        return anode
    }
    size[0usize] = *maxs.offset(0isize) - *mins.offset(0isize);
    size[1usize] = *maxs.offset(1isize) - *mins.offset(1isize);
    size[2usize] = *maxs.offset(2isize) - *mins.offset(2isize);
    if size[0usize] > size[1usize] {
        (*anode).axis = 0i32
    } else { (*anode).axis = 1i32 }
    (*anode).dist =
        (0.5f64 *
             (*maxs.offset((*anode).axis as isize) +
                  *mins.offset((*anode).axis as isize)) as libc::c_double) as
            libc::c_float;
    mins1[0usize] = *mins.offset(0isize);
    mins1[1usize] = *mins.offset(1isize);
    mins1[2usize] = *mins.offset(2isize);
    mins2[0usize] = *mins.offset(0isize);
    mins2[1usize] = *mins.offset(1isize);
    mins2[2usize] = *mins.offset(2isize);
    maxs1[0usize] = *maxs.offset(0isize);
    maxs1[1usize] = *maxs.offset(1isize);
    maxs1[2usize] = *maxs.offset(2isize);
    maxs2[0usize] = *maxs.offset(0isize);
    maxs2[1usize] = *maxs.offset(1isize);
    maxs2[2usize] = *maxs.offset(2isize);
    mins2[(*anode).axis as usize] = (*anode).dist;
    maxs1[(*anode).axis as usize] = mins2[(*anode).axis as usize];
    (*anode).children[0usize] =
        SV_CreateworldSector(depth + 1i32, mins2.as_mut_ptr(),
                             maxs2.as_mut_ptr());
    (*anode).children[1usize] =
        SV_CreateworldSector(depth + 1i32, mins1.as_mut_ptr(),
                             maxs1.as_mut_ptr());
    return anode;
}
#[no_mangle]
pub static mut sv_numworldSectors: libc::c_int = 0;
#[no_mangle]
pub static mut sv_worldSectors: [worldSector_t; 64] =
    [worldSector_s{axis: 0,
                   dist: 0.,
                   children:
                       [0 as *const worldSector_s as *mut worldSector_s; 2],
                   entities: 0 as *const svEntity_t as *mut svEntity_t,}; 64];
// called after the world model has been loaded, before linking any entities
#[no_mangle]
pub unsafe extern "C" fn SV_UnlinkEntity(mut gEnt: *mut sharedEntity_t) {
    let mut ent: *mut svEntity_t = 0 as *mut svEntity_t;
    let mut scan: *mut svEntity_t = 0 as *mut svEntity_t;
    let mut ws: *mut worldSector_t = 0 as *mut worldSector_t;
    ent = SV_SvEntityForGentity(gEnt);
    (*gEnt).r.linked = qfalse;
    ws = (*ent).worldSector;
    if ws.is_null() { return }
    (*ent).worldSector = 0 as *mut worldSector_s;
    if (*ws).entities == ent {
        (*ws).entities = (*ent).nextEntityInWorldSector;
        return
    }
    scan = (*ws).entities;
    while !scan.is_null() {
        if (*scan).nextEntityInWorldSector == ent {
            (*scan).nextEntityInWorldSector = (*ent).nextEntityInWorldSector;
            return
        }
        scan = (*scan).nextEntityInWorldSector
    }
    Com_Printf(b"WARNING: SV_UnlinkEntity: not found in worldSector\n\x00" as
                   *const u8 as *const libc::c_char);
}
// call before removing an entity, and before trying to move one,
// so it doesn't clip against itself
#[no_mangle]
pub unsafe extern "C" fn SV_LinkEntity(mut gEnt: *mut sharedEntity_t) {
    let mut node: *mut worldSector_t = 0 as *mut worldSector_t;
    let mut leafs: [libc::c_int; 128] = [0; 128];
    let mut cluster: libc::c_int = 0;
    let mut num_leafs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut area: libc::c_int = 0;
    let mut lastLeaf: libc::c_int = 0;
    let mut origin: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut ent: *mut svEntity_t = 0 as *mut svEntity_t;
    ent = SV_SvEntityForGentity(gEnt);
    if !(*ent).worldSector.is_null() { SV_UnlinkEntity(gEnt); }
    if 0 != (*gEnt).r.bmodel as u64 {
        (*gEnt).s.solid = 0xffffffi32
    } else if 0 != (*gEnt).r.contents & (1i32 | 0x2000000i32) {
        i = (*gEnt).r.maxs[0usize] as libc::c_int;
        if i < 1i32 { i = 1i32 }
        if i > 255i32 { i = 255i32 }
        j = -(*gEnt).r.mins[2usize] as libc::c_int;
        if j < 1i32 { j = 1i32 }
        if j > 255i32 { j = 255i32 }
        k = ((*gEnt).r.maxs[2usize] + 32i32 as libc::c_float) as libc::c_int;
        if k < 1i32 { k = 1i32 }
        if k > 255i32 { k = 255i32 }
        (*gEnt).s.solid = k << 16i32 | j << 8i32 | i
    } else { (*gEnt).s.solid = 0i32 }
    origin = (*gEnt).r.currentOrigin.as_mut_ptr();
    angles = (*gEnt).r.currentAngles.as_mut_ptr();
    if 0 != (*gEnt).r.bmodel as libc::c_uint &&
           (0. != *angles.offset(0isize) || 0. != *angles.offset(1isize) ||
                0. != *angles.offset(2isize)) {
        let mut max: libc::c_float = 0.;
        max =
            RadiusFromBounds((*gEnt).r.mins.as_mut_ptr() as *const vec_t,
                             (*gEnt).r.maxs.as_mut_ptr() as *const vec_t);
        i = 0i32;
        while i < 3i32 {
            (*gEnt).r.absmin[i as usize] = *origin.offset(i as isize) - max;
            (*gEnt).r.absmax[i as usize] = *origin.offset(i as isize) + max;
            i += 1
        }
    } else {
        (*gEnt).r.absmin[0usize] =
            *origin.offset(0isize) + (*gEnt).r.mins[0usize];
        (*gEnt).r.absmin[1usize] =
            *origin.offset(1isize) + (*gEnt).r.mins[1usize];
        (*gEnt).r.absmin[2usize] =
            *origin.offset(2isize) + (*gEnt).r.mins[2usize];
        (*gEnt).r.absmax[0usize] =
            *origin.offset(0isize) + (*gEnt).r.maxs[0usize];
        (*gEnt).r.absmax[1usize] =
            *origin.offset(1isize) + (*gEnt).r.maxs[1usize];
        (*gEnt).r.absmax[2usize] =
            *origin.offset(2isize) + (*gEnt).r.maxs[2usize]
    }
    (*gEnt).r.absmin[0usize] -= 1i32 as libc::c_float;
    (*gEnt).r.absmin[1usize] -= 1i32 as libc::c_float;
    (*gEnt).r.absmin[2usize] -= 1i32 as libc::c_float;
    (*gEnt).r.absmax[0usize] += 1i32 as libc::c_float;
    (*gEnt).r.absmax[1usize] += 1i32 as libc::c_float;
    (*gEnt).r.absmax[2usize] += 1i32 as libc::c_float;
    (*ent).numClusters = 0i32;
    (*ent).lastCluster = 0i32;
    (*ent).areanum = -1i32;
    (*ent).areanum2 = -1i32;
    num_leafs =
        CM_BoxLeafnums((*gEnt).r.absmin.as_mut_ptr() as *const vec_t,
                       (*gEnt).r.absmax.as_mut_ptr() as *const vec_t,
                       leafs.as_mut_ptr(), 128i32, &mut lastLeaf);
    if 0 == num_leafs { return }
    i = 0i32;
    while i < num_leafs {
        area = CM_LeafArea(leafs[i as usize]);
        if area != -1i32 {
            if (*ent).areanum != -1i32 && (*ent).areanum != area {
                if (*ent).areanum2 != -1i32 && (*ent).areanum2 != area &&
                       sv.state as libc::c_uint ==
                           SS_LOADING as libc::c_int as libc::c_uint {
                    Com_DPrintf(b"Object %i touching 3 areas at %f %f %f\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*gEnt).s.number,
                                (*gEnt).r.absmin[0usize] as libc::c_double,
                                (*gEnt).r.absmin[1usize] as libc::c_double,
                                (*gEnt).r.absmin[2usize] as libc::c_double);
                }
                (*ent).areanum2 = area
            } else { (*ent).areanum = area }
        }
        i += 1
    }
    (*ent).numClusters = 0i32;
    i = 0i32;
    while i < num_leafs {
        cluster = CM_LeafCluster(leafs[i as usize]);
        if cluster != -1i32 {
            let fresh0 = (*ent).numClusters;
            (*ent).numClusters = (*ent).numClusters + 1;
            (*ent).clusternums[fresh0 as usize] = cluster;
            if (*ent).numClusters == 16i32 { break ; }
        }
        i += 1
    }
    if i != num_leafs { (*ent).lastCluster = CM_LeafCluster(lastLeaf) }
    (*gEnt).r.linkcount += 1;
    node = sv_worldSectors.as_mut_ptr();
    while !((*node).axis == -1i32) {
        if (*gEnt).r.absmin[(*node).axis as usize] > (*node).dist {
            node = (*node).children[0usize]
        } else if (*gEnt).r.absmax[(*node).axis as usize] < (*node).dist {
            node = (*node).children[1usize]
        } else {
            // crosses the node
            break ;
        }
    }
    (*ent).worldSector = node;
    (*ent).nextEntityInWorldSector = (*node).entities;
    (*node).entities = ent;
    (*gEnt).r.linked = qtrue;
}
// Needs to be called any time an entity changes origin, mins, maxs,
// or solid.  Automatically unlinks if needed.
// sets ent->r.absmin and ent->r.absmax
// sets ent->leafnums[] for pvs determination even if the entity
// is not solid
#[no_mangle]
pub unsafe extern "C" fn SV_ClipHandleForEntity(mut ent:
                                                    *const sharedEntity_t)
 -> clipHandle_t {
    if 0 != (*ent).r.bmodel as u64 {
        return CM_InlineModel((*ent).s.modelindex)
    }
    if 0 != (*ent).r.svFlags & 0x400i32 {
        return CM_TempBoxModel((*ent).r.mins.as_ptr(), (*ent).r.maxs.as_ptr(),
                               qtrue as libc::c_int)
    }
    return CM_TempBoxModel((*ent).r.mins.as_ptr(), (*ent).r.maxs.as_ptr(),
                           qfalse as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SV_SectorList_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut sec: *mut worldSector_t = 0 as *mut worldSector_t;
    let mut ent: *mut svEntity_t = 0 as *mut svEntity_t;
    i = 0i32;
    while i < 64i32 {
        sec =
            &mut *sv_worldSectors.as_mut_ptr().offset(i as isize) as
                *mut worldSector_t;
        c = 0i32;
        ent = (*sec).entities;
        while !ent.is_null() { c += 1; ent = (*ent).nextEntityInWorldSector }
        Com_Printf(b"sector %i: %i entities\n\x00" as *const u8 as
                       *const libc::c_char, i, c);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_AreaEntities(mut mins: *const vec_t,
                                         mut maxs: *const vec_t,
                                         mut entityList: *mut libc::c_int,
                                         mut maxcount: libc::c_int)
 -> libc::c_int {
    let mut ap: areaParms_t =
        areaParms_t{mins: 0 as *const libc::c_float,
                    maxs: 0 as *const libc::c_float,
                    list: 0 as *mut libc::c_int,
                    count: 0,
                    maxcount: 0,};
    ap.mins = mins;
    ap.maxs = maxs;
    ap.list = entityList;
    ap.count = 0i32;
    ap.maxcount = maxcount;
    SV_AreaEntities_r(sv_worldSectors.as_mut_ptr(), &mut ap);
    return ap.count;
}
/*
====================
SV_AreaEntities_r

====================
*/
unsafe extern "C" fn SV_AreaEntities_r(mut node: *mut worldSector_t,
                                       mut ap: *mut areaParms_t) {
    let mut check: *mut svEntity_t = 0 as *mut svEntity_t;
    let mut next: *mut svEntity_t = 0 as *mut svEntity_t;
    let mut gcheck: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    check = (*node).entities;
    while !check.is_null() {
        next = (*check).nextEntityInWorldSector;
        gcheck = SV_GEntityForSvEntity(check);
        if !((*gcheck).r.absmin[0usize] > *(*ap).maxs.offset(0isize) ||
                 (*gcheck).r.absmin[1usize] > *(*ap).maxs.offset(1isize) ||
                 (*gcheck).r.absmin[2usize] > *(*ap).maxs.offset(2isize) ||
                 (*gcheck).r.absmax[0usize] < *(*ap).mins.offset(0isize) ||
                 (*gcheck).r.absmax[1usize] < *(*ap).mins.offset(1isize) ||
                 (*gcheck).r.absmax[2usize] < *(*ap).mins.offset(2isize)) {
            if (*ap).count == (*ap).maxcount {
                Com_Printf(b"SV_AreaEntities: MAXCOUNT\n\x00" as *const u8 as
                               *const libc::c_char);
                return
            }
            *(*ap).list.offset((*ap).count as isize) =
                check.wrapping_offset_from(sv.svEntities.as_mut_ptr()) as
                    libc::c_long as libc::c_int;
            (*ap).count += 1
        }
        check = next
    }
    if (*node).axis == -1i32 { return }
    if *(*ap).maxs.offset((*node).axis as isize) > (*node).dist {
        SV_AreaEntities_r((*node).children[0usize], ap);
    }
    if *(*ap).mins.offset((*node).axis as isize) < (*node).dist {
        SV_AreaEntities_r((*node).children[1usize], ap);
    };
}
// fills in a table of entity numbers with entities that have bounding boxes
// that intersect the given area.  It is possible for a non-axial bmodel
// to be returned that doesn't actually intersect the area on an exact
// test.
// returns the number of pointers filled in
// The world entity is never returned in this list.
#[no_mangle]
pub unsafe extern "C" fn SV_PointContents(mut p: *const vec_t,
                                          mut passEntityNum: libc::c_int)
 -> libc::c_int {
    let mut touch: [libc::c_int; 1024] = [0; 1024];
    let mut hit: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut contents: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut clipHandle: clipHandle_t = 0;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    contents = CM_PointContents(p, 0i32);
    num = SV_AreaEntities(p, p, touch.as_mut_ptr(), 1i32 << 10i32);
    i = 0i32;
    while i < num {
        if !(touch[i as usize] == passEntityNum) {
            hit = SV_GentityNum(touch[i as usize]);
            clipHandle = SV_ClipHandleForEntity(hit);
            angles = (*hit).r.currentAngles.as_mut_ptr();
            if 0 == (*hit).r.bmodel as u64 {
                angles = vec3_origin.as_mut_ptr()
            }
            c2 =
                CM_TransformedPointContents(p, clipHandle,
                                            (*hit).r.currentOrigin.as_mut_ptr()
                                                as *const vec_t,
                                            angles as *const vec_t);
            contents |= c2
        }
        i += 1
    }
    return contents;
}
// returns the CONTENTS_* value from the world and all entities at the given point.
#[no_mangle]
pub unsafe extern "C" fn SV_Trace(mut results: *mut trace_t,
                                  mut start: *const vec_t,
                                  mut mins: *mut vec_t, mut maxs: *mut vec_t,
                                  mut end: *const vec_t,
                                  mut passEntityNum: libc::c_int,
                                  mut contentmask: libc::c_int,
                                  mut capsule: libc::c_int) {
    let mut clip: moveclip_t =
        moveclip_t{boxmins: [0.; 3],
                   boxmaxs: [0.; 3],
                   mins: 0 as *const libc::c_float,
                   maxs: 0 as *const libc::c_float,
                   start: 0 as *const libc::c_float,
                   end: [0.; 3],
                   trace:
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
                               entityNum: 0,},
                   passEntityNum: 0,
                   contentmask: 0,
                   capsule: 0,};
    let mut i: libc::c_int = 0;
    if mins.is_null() { mins = vec3_origin.as_mut_ptr() }
    if maxs.is_null() { maxs = vec3_origin.as_mut_ptr() }
    memset(&mut clip as *mut moveclip_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<moveclip_t>() as libc::c_ulong);
    CM_BoxTrace(&mut clip.trace, start, end, mins, maxs, 0i32, contentmask,
                capsule);
    clip.trace.entityNum =
        if clip.trace.fraction as libc::c_double != 1.0f64 {
            (1i32 << 10i32) - 2i32
        } else { (1i32 << 10i32) - 1i32 };
    if clip.trace.fraction == 0i32 as libc::c_float {
        *results = clip.trace;
        return
    }
    clip.contentmask = contentmask;
    clip.start = start;
    clip.end[0usize] = *end.offset(0isize);
    clip.end[1usize] = *end.offset(1isize);
    clip.end[2usize] = *end.offset(2isize);
    clip.mins = mins as *const libc::c_float;
    clip.maxs = maxs as *const libc::c_float;
    clip.passEntityNum = passEntityNum;
    clip.capsule = capsule;
    i = 0i32;
    while i < 3i32 {
        if *end.offset(i as isize) > *start.offset(i as isize) {
            clip.boxmins[i as usize] =
                *clip.start.offset(i as isize) + *clip.mins.offset(i as isize)
                    - 1i32 as libc::c_float;
            clip.boxmaxs[i as usize] =
                clip.end[i as usize] + *clip.maxs.offset(i as isize) +
                    1i32 as libc::c_float
        } else {
            clip.boxmins[i as usize] =
                clip.end[i as usize] + *clip.mins.offset(i as isize) -
                    1i32 as libc::c_float;
            clip.boxmaxs[i as usize] =
                *clip.start.offset(i as isize) + *clip.maxs.offset(i as isize)
                    + 1i32 as libc::c_float
        }
        i += 1
    }
    SV_ClipMoveToEntities(&mut clip);
    *results = clip.trace;
}
/*
====================
SV_ClipMoveToEntities

====================
*/
unsafe extern "C" fn SV_ClipMoveToEntities(mut clip: *mut moveclip_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touchlist: [libc::c_int; 1024] = [0; 1024];
    let mut touch: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut passOwnerNum: libc::c_int = 0;
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
    let mut clipHandle: clipHandle_t = 0;
    let mut origin: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    num =
        SV_AreaEntities((*clip).boxmins.as_mut_ptr() as *const vec_t,
                        (*clip).boxmaxs.as_mut_ptr() as *const vec_t,
                        touchlist.as_mut_ptr(), 1i32 << 10i32);
    if (*clip).passEntityNum != (1i32 << 10i32) - 1i32 {
        passOwnerNum = (*SV_GentityNum((*clip).passEntityNum)).r.ownerNum;
        if passOwnerNum == (1i32 << 10i32) - 1i32 { passOwnerNum = -1i32 }
    } else { passOwnerNum = -1i32 }
    let mut current_block_34: u64;
    i = 0i32;
    while i < num {
        if 0 != (*clip).trace.allsolid as u64 { return }
        touch = SV_GentityNum(touchlist[i as usize]);
        // see if we should ignore this entity
        if (*clip).passEntityNum != (1i32 << 10i32) - 1i32 {
            if touchlist[i as usize] == (*clip).passEntityNum {
                // don't clip against the pass entity
                current_block_34 = 13586036798005543211;
            } else if (*touch).r.ownerNum == (*clip).passEntityNum {
                // don't clip against own missiles
                current_block_34 = 13586036798005543211;
            } else if (*touch).r.ownerNum == passOwnerNum {
                // don't clip against other missiles from our owner
                current_block_34 = 13586036798005543211;
            } else { current_block_34 = 13472856163611868459; }
        } else { current_block_34 = 13472856163611868459; }
        match current_block_34 {
            13472856163611868459 => {
                // if it doesn't have any brushes of a type we
		// are looking for, ignore it
                if !(0 == (*clip).contentmask & (*touch).r.contents) {
                    clipHandle = SV_ClipHandleForEntity(touch);
                    origin = (*touch).r.currentOrigin.as_mut_ptr();
                    angles = (*touch).r.currentAngles.as_mut_ptr();
                    if 0 == (*touch).r.bmodel as u64 {
                        angles = vec3_origin.as_mut_ptr()
                    }
                    CM_TransformedBoxTrace(&mut trace,
                                           (*clip).start as *mut libc::c_float
                                               as *const vec_t,
                                           (*clip).end.as_mut_ptr() as
                                               *const vec_t,
                                           (*clip).mins as *mut libc::c_float,
                                           (*clip).maxs as *mut libc::c_float,
                                           clipHandle, (*clip).contentmask,
                                           origin as *const vec_t,
                                           angles as *const vec_t,
                                           (*clip).capsule);
                    if 0 != trace.allsolid as u64 {
                        (*clip).trace.allsolid = qtrue;
                        trace.entityNum = (*touch).s.number
                    } else if 0 != trace.startsolid as u64 {
                        (*clip).trace.startsolid = qtrue;
                        trace.entityNum = (*touch).s.number
                    }
                    if trace.fraction < (*clip).trace.fraction {
                        let mut oldStart: qboolean = qfalse;
                        oldStart = (*clip).trace.startsolid;
                        trace.entityNum = (*touch).s.number;
                        (*clip).trace = trace;
                        (*clip).trace.startsolid =
                            ::std::mem::transmute::<libc::c_uint,
                                                    qboolean>((*clip).trace.startsolid
                                                                  as
                                                                  libc::c_uint
                                                                  |
                                                                  oldStart as
                                                                      libc::c_uint)
                    }
                }
            }
            _ => { }
        }
        i += 1
    };
}
// mins and maxs are relative
// if the entire move stays in a solid volume, trace.allsolid will be set,
// trace.startsolid will be set, and trace.fraction will be 0
// if the starting point is in a solid, it will be allowed to move out
// to an open area
// passEntityNum is explicitly excluded from clipping checks (normally ENTITYNUM_NONE)
#[no_mangle]
pub unsafe extern "C" fn SV_ClipToEntity(mut trace: *mut trace_t,
                                         mut start: *const vec_t,
                                         mut mins: *const vec_t,
                                         mut maxs: *const vec_t,
                                         mut end: *const vec_t,
                                         mut entityNum: libc::c_int,
                                         mut contentmask: libc::c_int,
                                         mut capsule: libc::c_int) {
    let mut touch: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut clipHandle: clipHandle_t = 0;
    let mut origin: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    touch = SV_GentityNum(entityNum);
    memset(trace as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<trace_t>() as libc::c_ulong);
    if 0 == contentmask & (*touch).r.contents {
        (*trace).fraction = 1.0f64 as libc::c_float;
        return
    }
    clipHandle = SV_ClipHandleForEntity(touch);
    origin = (*touch).r.currentOrigin.as_mut_ptr();
    angles = (*touch).r.currentAngles.as_mut_ptr();
    if 0 == (*touch).r.bmodel as u64 { angles = vec3_origin.as_mut_ptr() }
    CM_TransformedBoxTrace(trace, start as *mut libc::c_float as *const vec_t,
                           end as *mut libc::c_float as *const vec_t,
                           mins as *mut libc::c_float,
                           maxs as *mut libc::c_float, clipHandle,
                           contentmask, origin as *const vec_t,
                           angles as *const vec_t, capsule);
    if (*trace).fraction < 1i32 as libc::c_float {
        (*trace).entityNum = (*touch).s.number
    };
}