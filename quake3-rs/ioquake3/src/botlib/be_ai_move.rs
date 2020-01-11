// =============== BEGIN be_ai_move_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_moveresult_s {
    pub failure: i32,
    pub type_0: i32,
    pub blocked: i32,
    pub blockentity: i32,
    pub traveltype: i32,
    pub flags: i32,
    pub weapon: i32,
    pub movedir: crate::src::qcommon::q_shared::vec3_t,
    pub ideal_viewangles: crate::src::qcommon::q_shared::vec3_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_initmove_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub viewoffset: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: i32,
    pub client: i32,
    pub thinktime: f32,
    pub presencetype: i32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub or_moveflags: i32,
}

pub type bot_initmove_t = crate::src::botlib::be_ai_move::bot_initmove_s;

pub type bot_moveresult_t = crate::src::botlib::be_ai_move::bot_moveresult_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_avoidspot_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: f32,
    pub type_0: i32,
}

pub type bot_avoidspot_t = crate::src::botlib::be_ai_move::bot_avoidspot_s;
use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0) * *v.offset(0)
            + *v.offset(1) * *v.offset(1)
            + *v.offset(2) * *v.offset(2);
    }

    // __Q_SHARED_H
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::aasfile_h::aas_reachability_s;
pub use crate::aasfile_h::aas_reachability_t;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_clientmove_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_t;
pub use crate::src::botlib::be_ai_move::q_shared_h::VectorLength;
pub use crate::src::botlib::be_ai_move::q_shared_h::VectorLengthSquared;
pub use crate::src::botlib::l_libvar::libvar_s;
pub use crate::src::botlib::l_libvar::libvar_t;
pub use crate::src::botlib::l_libvar::LibVar;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

pub use crate::src::botlib::be_ai_move::stdlib_h::atoi;

pub use crate::stdlib::rand;
pub use crate::stdlib::strtol;
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
/* ****************************************************************************
 * name:		be_ai_move.c
 *
 * desc:		bot movement AI
 *
 * $Archive: /MissionPack/code/botlib/be_ai_move.c $
 *
 *****************************************************************************/
//#define DEBUG_AI_MOVE
//#define DEBUG_ELEVATOR
//#define DEBUG_GRAPPLE
//movement state
//NOTE: the moveflags MFL_ONGROUND, MFL_TELEPORTED, MFL_WATERJUMP and
//		MFL_GRAPPLEPULL must be set outside the movement code

pub type bot_movestate_t = bot_movestate_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_movestate_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub viewoffset: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: i32,
    pub client: i32,
    pub thinktime: f32,
    pub presencetype: i32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: i32,
    pub lastareanum: i32,
    pub lastgoalareanum: i32,
    pub lastreachnum: i32,
    pub lastorigin: crate::src::qcommon::q_shared::vec3_t,
    pub reachareanum: i32,
    pub moveflags: i32,
    pub jumpreach: i32,
    pub grapplevisible_time: f32,
    pub lastgrappledist: f32,
    pub reachability_time: f32,
    pub avoidreach: [i32; 1],
    pub avoidreachtimes: [f32; 1],
    pub avoidreachtries: [i32; 1],
    pub avoidspots: [crate::src::botlib::be_ai_move::bot_avoidspot_t; 32],
    pub numavoidspots: i32,
}
#[no_mangle]

pub static mut sv_maxstep: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut sv_maxbarrier: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut sv_gravity: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut weapindex_rocketlauncher: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut weapindex_bfg10k: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut weapindex_grapple: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut entitytypemissile: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut offhandgrapple: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut cmd_grappleoff: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
#[no_mangle]

pub static mut cmd_grappleon: *mut crate::src::botlib::l_libvar::libvar_t =
    0 as *mut crate::src::botlib::l_libvar::libvar_t;
//input vars (all set outside the movement code)
//origin of the bot
//velocity of the bot
//view offset
//entity number of the bot
//client number of the bot
//time the bot thinks
//presencetype of the bot
//view angles of the bot
//state vars
//area the bot is in
//last area the bot was in
//last goal area number
//last reachability number
//origin previous cycle
//area number of the reachabilty
//movement flags
//set when jumped
//last time the grapple was visible
//last distance to the grapple end
//time to use current reachability
//reachabilities to avoid
//times to avoid the reachabilities
//number of tries before avoiding
//
//spots to avoid
//type of model, func_plat or func_bobbing
#[no_mangle]

pub static mut modeltypes: [i32; 256] = [0; 256];
#[no_mangle]

pub static mut botmovestates: [*mut bot_movestate_t; 65] = [0 as *mut bot_movestate_t; 65];
//returns the handle of a newly allocated movestate
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAllocMoveState() -> i32 {
    let mut _i: i32 = 0; //end for

    for i in 1..=64 {
        if botmovestates[i as usize].is_null() {
            botmovestates[i as usize] = crate::src::botlib::l_memory::GetClearedMemory(
                ::std::mem::size_of::<bot_movestate_t>(),
            ) as *mut bot_movestate_t;
            return i;
        }
    }
    return 0;
}
//frees the movestate with the given handle
//end of the function BotAllocMoveState
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeMoveState(mut handle: i32) {
    if handle <= 0 || handle > 64 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"move state handle %d out of range\n\x00" as *const u8 as *mut i8,
            handle,
        ); //end if
        return;
    } //end if
    if botmovestates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"invalid move state %d\n\x00" as *const u8 as *mut i8,
            handle,
        );
        return;
    }
    crate::src::botlib::l_memory::FreeMemory(botmovestates[handle as usize] as *mut libc::c_void);
    botmovestates[handle as usize] = 0 as *mut bot_movestate_t;
}
//end of the function BotFreeMoveState
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotMoveStateFromHandle(mut handle: i32) -> *mut bot_movestate_t {
    if handle <= 0 || handle > 64 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"move state handle %d out of range\n\x00" as *const u8 as *mut i8,
            handle,
        ); //end if
        return 0 as *mut bot_movestate_t;
    } //end if
    if botmovestates[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"invalid move state %d\n\x00" as *const u8 as *mut i8,
            handle,
        );
        return 0 as *mut bot_movestate_t;
    }
    return botmovestates[handle as usize];
}
//initialize movement state before performing any movement
//end of the function BotMoveStateFromHandle
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInitMoveState(
    mut handle: i32,
    mut initmove: *mut crate::src::botlib::be_ai_move::bot_initmove_t,
) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(handle);
    if ms.is_null() {
        return;
    }
    (*ms).origin[0] = (*initmove).origin[0];
    (*ms).origin[1] = (*initmove).origin[1];
    (*ms).origin[2] = (*initmove).origin[2];
    (*ms).velocity[0] = (*initmove).velocity[0];
    (*ms).velocity[1] = (*initmove).velocity[1];
    (*ms).velocity[2] = (*initmove).velocity[2];
    (*ms).viewoffset[0] = (*initmove).viewoffset[0];
    (*ms).viewoffset[1] = (*initmove).viewoffset[1];
    (*ms).viewoffset[2] = (*initmove).viewoffset[2];
    (*ms).entitynum = (*initmove).entitynum;
    (*ms).client = (*initmove).client;
    (*ms).thinktime = (*initmove).thinktime;
    (*ms).presencetype = (*initmove).presencetype;
    (*ms).viewangles[0] = (*initmove).viewangles[0];
    (*ms).viewangles[1] = (*initmove).viewangles[1];
    (*ms).viewangles[2] = (*initmove).viewangles[2];
    //
    (*ms).moveflags &= !(2);
    if (*initmove).or_moveflags & 2 != 0 {
        (*ms).moveflags |= 2
    }
    (*ms).moveflags &= !(32);
    if (*initmove).or_moveflags & 32 != 0 {
        (*ms).moveflags |= 32
    }
    (*ms).moveflags &= !(16);
    if (*initmove).or_moveflags & 16 != 0 {
        (*ms).moveflags |= 16
    }
    (*ms).moveflags &= !(512);
    if (*initmove).or_moveflags & 512 != 0 {
        (*ms).moveflags |= 512
    }
    (*ms).moveflags &= !(64);
    if (*initmove).or_moveflags & 64 != 0 {
        (*ms).moveflags |= 64
    };
}
//end of the function BotInitMoveState
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn AngleDiff(mut ang1: f32, mut ang2: f32) -> f32 {
    let mut diff: f32 = 0.; //end else
    diff = ang1 - ang2; //end if
    if ang1 > ang2 {
        if diff as f64 > 180.0 {
            diff = (diff as f64 - 360.0) as f32
        }
    } else if (diff as f64) < -180.0 {
        diff = (diff as f64 + 360.0) as f32
    }
    return diff;
}
//end of the function AngleDiff
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFuzzyPointReachabilityArea(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut firstareanum: i32 = 0; //end if
    let mut j: i32 = 0; //end for
    let mut x: i32 = 0; //end for
    let mut y: i32 = 0; //end for
    let mut z: i32 = 0;
    let mut areas: [i32; 10] = [0; 10];
    let mut numareas: i32 = 0;
    let mut areanum: i32 = 0;
    let mut bestareanum: i32 = 0;
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 10] = [[0.; 3]; 10];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    firstareanum = 0;
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(origin);
    if areanum != 0 {
        firstareanum = areanum;
        if crate::src::botlib::be_aas_reach::AAS_AreaReachability(areanum) != 0 {
            return areanum;
        }
    }
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    end[2] += 4f32;
    numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
        origin,
        end.as_mut_ptr(),
        areas.as_mut_ptr(),
        points.as_mut_ptr(),
        10,
    );
    j = 0;
    while j < numareas {
        if crate::src::botlib::be_aas_reach::AAS_AreaReachability(areas[j as usize]) != 0 {
            return areas[j as usize];
        }
        j += 1
    }
    bestdist = 999999f32;
    bestareanum = 0;
    z = 1;
    while z >= -(1) {
        x = 1;
        while x >= -(1) {
            y = 1;
            while y >= -(1) {
                end[0] = *origin.offset(0);
                end[1] = *origin.offset(1);
                end[2] = *origin.offset(2);
                end[0] += (x * 8) as f32;
                end[1] += (y * 8) as f32;
                end[2] += (z * 12) as f32;
                numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                    origin,
                    end.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    points.as_mut_ptr(),
                    10,
                );
                j = 0;
                while j < numareas {
                    //end for
                    if crate::src::botlib::be_aas_reach::AAS_AreaReachability(areas[j as usize])
                        != 0
                    {
                        v[0] = points[j as usize][0] - *origin.offset(0); //end if
                        v[1] = points[j as usize][1] - *origin.offset(1);
                        v[2] = points[j as usize][2] - *origin.offset(2);
                        dist = VectorLength(
                            v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                        );
                        if dist < bestdist {
                            bestareanum = areas[j as usize];
                            bestdist = dist
                        }
                        //end if
                    }
                    if firstareanum == 0 {
                        firstareanum = areas[j as usize]
                    }
                    j += 1
                }
                y -= 1
            }
            x -= 1
            //end for
        }
        if bestareanum != 0 {
            return bestareanum;
        }
        z -= 1
    }
    return firstareanum;
}
//returns a reachability area if the origin is in one
//end of the function BotFuzzyPointReachabilityArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotReachabilityArea(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut client: i32,
) -> i32 {
    let mut modelnum: i32 = 0;
    let mut modeltype: i32 = 0;
    let mut reachnum: i32 = 0;
    let mut areanum: i32 = 0;
    let mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    //check if the bot is standing on something
    crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox(
        4,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    ); //end if
    end[0] = *origin.offset(0) + up[0] * -3f32;
    end[1] = *origin.offset(1) + up[1] * -3f32;
    end[2] = *origin.offset(2) + up[2] * -3f32;
    bsptrace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        origin,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        client,
        1 | 0x10000,
    );
    if bsptrace.startsolid as u64 == 0
        && bsptrace.fraction < 1f32
        && bsptrace.ent != ((1) << 10) - 1
    {
        //if standing on the world the bot should be in a valid area
        if bsptrace.ent == ((1) << 10) - 2 {
            return BotFuzzyPointReachabilityArea(origin);
        } //end if
        modelnum = crate::src::botlib::be_aas_entity::AAS_EntityModelindex(bsptrace.ent);
        modeltype = modeltypes[modelnum as usize];
        //if standing on a func_plat or func_bobbing then the bot is assumed to be
        //in the area the reachability points to
        if modeltype == 1 || modeltype == 2 {
            reachnum = crate::src::botlib::be_aas_route::AAS_NextModelReachability(0, modelnum);
            if reachnum != 0 {
                crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach); //end else if
                return reach.areanum;
            }
            //end if
        }
        //if the bot is swimming the bot should be in a valid area
        if crate::src::botlib::be_aas_move::AAS_Swimming(origin) != 0 {
            return BotFuzzyPointReachabilityArea(origin);
        } //end if
          //
        areanum = BotFuzzyPointReachabilityArea(origin);
        //if the bot is in an area with reachabilities
        if areanum != 0 && crate::src::botlib::be_aas_reach::AAS_AreaReachability(areanum) != 0 {
            return areanum;
        }
        //trace down till the ground is hit because the bot is standing on some other entity
        org[0] = *origin.offset(0); //end if
        org[1] = *origin.offset(1);
        org[2] = *origin.offset(2);
        end[0] = org[0];
        end[1] = org[1];
        end[2] = org[2];
        end[2] -= 800f32;
        trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
            org.as_mut_ptr(),
            end.as_mut_ptr(),
            4,
            -(1),
        );
        if trace.startsolid as u64 == 0 {
            org[0] = trace.endpos[0];
            org[1] = trace.endpos[1];
            org[2] = trace.endpos[2]
        }
        //
        return BotFuzzyPointReachabilityArea(org.as_mut_ptr());
    }
    //
    return BotFuzzyPointReachabilityArea(origin);
}
//end of the function BotReachabilityArea
//===========================================================================
// returns the reachability area the bot is in
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
/*
int BotReachabilityArea(vec3_t origin, int testground)
{
    int firstareanum, i, j, x, y, z;
    int areas[10], numareas, areanum, bestareanum;
    float dist, bestdist;
    vec3_t org, end, points[10], v;
    aas_trace_t trace;

    firstareanum = 0;
    for (i = 0; i < 2; i++)
    {
        VectorCopy(origin, org);
        //if test at the ground (used when bot is standing on an entity)
        if (i > 0)
        {
            VectorCopy(origin, end);
            end[2] -= 800;
            trace = AAS_TraceClientBBox(origin, end, PRESENCE_CROUCH, -1);
            if (!trace.startsolid)
            {
                VectorCopy(trace.endpos, org);
            } //end if
        } //end if

        firstareanum = 0;
        areanum = AAS_PointAreaNum(org);
        if (areanum)
        {
            firstareanum = areanum;
            if (AAS_AreaReachability(areanum)) return areanum;
        } //end if
        bestdist = 999999;
        bestareanum = 0;
        for (z = 1; z >= -1; z -= 1)
        {
            for (x = 1; x >= -1; x -= 1)
            {
                for (y = 1; y >= -1; y -= 1)
                {
                    VectorCopy(org, end);
                    end[0] += x * 8;
                    end[1] += y * 8;
                    end[2] += z * 12;
                    numareas = AAS_TraceAreas(org, end, areas, points, 10);
                    for (j = 0; j < numareas; j++)
                    {
                        if (AAS_AreaReachability(areas[j]))
                        {
                            VectorSubtract(points[j], org, v);
                            dist = VectorLength(v);
                            if (dist < bestdist)
                            {
                                bestareanum = areas[j];
                                bestdist = dist;
                            } //end if
                        } //end if
                    } //end for
                } //end for
            } //end for
            if (bestareanum) return bestareanum;
        } //end for
        if (!testground) break;
    } //end for
//#ifdef DEBUG
    //botimport.Print(PRT_MESSAGE, "no reachability area\n");
//#endif //DEBUG
    return firstareanum;
} //end of the function BotReachabilityArea*/
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotOnMover(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut entnum: i32,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> i32 {
    let mut _i: i32 = 0;
    let mut modelnum: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut modelorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 0f32];
    let mut boxmins: crate::src::qcommon::q_shared::vec3_t = [-16f32, -16f32, -8f32];
    let mut boxmaxs: crate::src::qcommon::q_shared::vec3_t = [16f32, 16f32, 8f32];
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    modelnum = (*reach).facenum & 0xffff;
    //get some bsp model info
    crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
        modelnum,
        angles.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    //
    if crate::src::botlib::be_aas_entity::AAS_OriginOfMoverWithModelNum(
        modelnum,
        modelorigin.as_mut_ptr(),
    ) == 0
    {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"no entity with model %d\n\x00" as *const u8 as *mut i8,
            modelnum,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    //end for
    for i in 0..2 {
        if *origin.offset(i as isize) > modelorigin[i as usize] + maxs[i as usize] + 16f32 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }

        if *origin.offset(i as isize) < modelorigin[i as usize] + mins[i as usize] - 16f32 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //
    org[0] = *origin.offset(0);
    org[1] = *origin.offset(1);
    org[2] = *origin.offset(2);
    org[2] += 24f32;
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    end[2] -= 48f32;
    //
    trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        org.as_mut_ptr(),
        boxmins.as_mut_ptr(),
        boxmaxs.as_mut_ptr(),
        end.as_mut_ptr(),
        entnum,
        1 | 0x10000,
    ); //end if
    if trace.startsolid as u64 == 0 && trace.allsolid as u64 == 0 {
        //NOTE: the reachability face number is the model number of the elevator
        if trace.ent != ((1) << 10) - 1
            && crate::src::botlib::be_aas_entity::AAS_EntityModelNum(trace.ent) == modelnum
        {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //end if
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function BotOnMover
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn MoverDown(mut reach: *mut crate::aasfile_h::aas_reachability_t) -> i32 {
    let mut modelnum: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 0f32];
    modelnum = (*reach).facenum & 0xffff;
    //get some bsp model info
    crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
        modelnum,
        angles.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        origin.as_mut_ptr(),
    );
    //
    if crate::src::botlib::be_aas_entity::AAS_OriginOfMoverWithModelNum(
        modelnum,
        origin.as_mut_ptr(),
    ) == 0
    {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"no entity with model %d\n\x00" as *const u8 as *mut i8,
            modelnum,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the top of the plat is below the reachability start point
    if origin[2] + maxs[2] < (*reach).start[2] {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//must be called every map change
//end of the function MoverDown
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetBrushModelTypes() {
    let mut ent: i32 = 0;
    let mut modelnum: i32 = 0;
    let mut classname: [i8; 128] = [0; 128];
    let mut model: [i8; 128] = [0; 128];
    crate::stdlib::memset(
        modeltypes.as_mut_ptr() as *mut libc::c_void,
        0,
        (256usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    //
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0); //end if
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *mut i8,
            classname.as_mut_ptr(),
            128,
        ) == 0)
        {
            if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                ent,
                b"model\x00" as *const u8 as *mut i8,
                model.as_mut_ptr(),
                128,
            ) == 0)
            {
                if model[0] != 0 {
                    modelnum = atoi(model.as_mut_ptr().offset(1))
                } else {
                    modelnum = 0
                }
                if modelnum < 0 || modelnum >= 256 {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        1i32,
                        b"entity %s model number out of range\n\x00" as *const u8 as *mut i8,
                        classname.as_mut_ptr(),
                    );
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    classname.as_mut_ptr(),
                    b"func_bobbing\x00" as *const u8 as *const i8,
                ) == 0
                {
                    modeltypes[modelnum as usize] = 2
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    classname.as_mut_ptr(),
                    b"func_plat\x00" as *const u8 as *const i8,
                ) == 0
                {
                    modeltypes[modelnum as usize] = 1
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    classname.as_mut_ptr(),
                    b"func_door\x00" as *const u8 as *const i8,
                ) == 0
                {
                    modeltypes[modelnum as usize] = 3
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    classname.as_mut_ptr(),
                    b"func_static\x00" as *const u8 as *const i8,
                ) == 0
                {
                    modeltypes[modelnum as usize] = 4
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
    }
    //end for
}
//end of the function BotSetBrushModelTypes
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotOnTopOfEntity(mut ms: *mut bot_movestate_t) -> i32 {
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //end if
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox(
        (*ms).presencetype,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    end[0] = (*ms).origin[0] + up[0] * -3f32;
    end[1] = (*ms).origin[1] + up[1] * -3f32;
    end[2] = (*ms).origin[2] + up[2] * -3f32;
    trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        (*ms).origin.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*ms).entitynum,
        1 | 0x10000,
    );
    if trace.startsolid as u64 == 0
        && (trace.ent != ((1) << 10) - 2 && trace.ent != ((1) << 10) - 1)
    {
        return trace.ent;
    }
    return -(1);
}
//end of the function BotOnTopOfEntity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotValidTravel(
    mut _origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
    mut travelflags: i32,
) -> i32 {
    //if the reachability uses an unwanted travel type
    if crate::src::botlib::be_aas_route::AAS_TravelFlagForType((*reach).traveltype) & !travelflags
        != 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //don't go into areas with bad travel types
    if crate::src::botlib::be_aas_route::AAS_AreaContentsTravelFlags((*reach).areanum)
        & !travelflags
        != 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function BotValidTravel
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAddToAvoidReach(
    mut ms: *mut bot_movestate_t,
    mut number: i32,
    mut avoidtime: f32,
) {
    let mut i: i32 = 0; //end for
    i = 0;
    while i < 1 {
        if (*ms).avoidreach[i as usize] == number {
            if (*ms).avoidreachtimes[i as usize] > crate::src::botlib::be_aas_main::AAS_Time() {
                (*ms).avoidreachtries[i as usize] += 1
            } else {
                (*ms).avoidreachtries[i as usize] = 1
            }
            (*ms).avoidreachtimes[i as usize] =
                crate::src::botlib::be_aas_main::AAS_Time() + avoidtime;
            return;
        }
        i += 1
        //end if
    }
    //add the reachability to the reachabilities to avoid for a while
    i = 0;
    while i < 1 {
        if (*ms).avoidreachtimes[i as usize] < crate::src::botlib::be_aas_main::AAS_Time() {
            (*ms).avoidreach[i as usize] = number;
            (*ms).avoidreachtimes[i as usize] =
                crate::src::botlib::be_aas_main::AAS_Time() + avoidtime;
            (*ms).avoidreachtries[i as usize] = 1;
            return;
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function BotAddToAvoidReach
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn DistanceFromLineSquared(
    mut p: *mut crate::src::qcommon::q_shared::vec_t,
    mut lp1: *mut crate::src::qcommon::q_shared::vec_t,
    mut lp2: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut proj: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut j: i32 = 0;
    crate::src::botlib::be_aas_main::AAS_ProjectPointOntoVector(p, lp1, lp2, proj.as_mut_ptr());
    j = 0;
    while j < 3 {
        if proj[j as usize] > *lp1.offset(j as isize) && proj[j as usize] > *lp2.offset(j as isize)
            || proj[j as usize] < *lp1.offset(j as isize)
                && proj[j as usize] < *lp2.offset(j as isize)
        {
            break;
        }
        j += 1
    }
    if j < 3 {
        if crate::stdlib::fabs((proj[j as usize] - *lp1.offset(j as isize)) as f64)
            < crate::stdlib::fabs((proj[j as usize] - *lp2.offset(j as isize)) as f64)
        {
            dir[0] = *p.offset(0) - *lp1.offset(0);
            dir[1] = *p.offset(1) - *lp1.offset(1);
            dir[2] = *p.offset(2) - *lp1.offset(2)
        } else {
            dir[0] = *p.offset(0) - *lp2.offset(0);
            dir[1] = *p.offset(1) - *lp2.offset(1);
            dir[2] = *p.offset(2) - *lp2.offset(2)
        }
        return VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    }
    dir[0] = *p.offset(0) - proj[0];
    dir[1] = *p.offset(1) - proj[1];
    dir[2] = *p.offset(2) - proj[2];
    return VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
//end of the function DistanceFromLineSquared
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn VectorDistanceSquared(
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    dir[0] = *p2.offset(0) - *p1.offset(0);
    dir[1] = *p2.offset(1) - *p1.offset(1);
    dir[2] = *p2.offset(2) - *p1.offset(2);
    return VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
//end of the function VectorDistanceSquared
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAvoidSpots(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
    mut avoidspots: *mut crate::src::botlib::be_ai_move::bot_avoidspot_t,
    mut numavoidspots: i32,
) -> i32 {
    let mut checkbetween: i32 = 0; //end switch
    let mut _i: i32 = 0; //end for
    let mut type_0: i32 = 0;
    let mut squareddist: f32 = 0.;
    let mut squaredradius: f32 = 0.;
    match (*reach).traveltype & 0xffffff {
        2 => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
        3 => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
        4 => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
        6 => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
        7 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        5 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        8 => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
        9 => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
        10 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        11 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        14 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        12 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        13 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        18 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        19 => checkbetween = crate::src::qcommon::q_shared::qfalse as i32,
        _ => checkbetween = crate::src::qcommon::q_shared::qtrue as i32,
    }
    type_0 = 0;

    for i in 0..numavoidspots {
        squaredradius =
            (*avoidspots.offset(i as isize)).radius * (*avoidspots.offset(i as isize)).radius;

        squareddist = DistanceFromLineSquared(
            (*avoidspots.offset(i as isize)).origin.as_mut_ptr(),
            origin,
            (*reach).start.as_mut_ptr(),
        );

        if squareddist < squaredradius
            && VectorDistanceSquared((*avoidspots.offset(i as isize)).origin.as_mut_ptr(), origin)
                > squareddist
        {
            //end else
            type_0 = (*avoidspots.offset(i as isize)).type_0
        } else if checkbetween != 0 {
            squareddist = DistanceFromLineSquared(
                (*avoidspots.offset(i as isize)).origin.as_mut_ptr(),
                (*reach).start.as_mut_ptr(),
                (*reach).end.as_mut_ptr(),
            ); //end if
               //end if
            if squareddist < squaredradius
                && VectorDistanceSquared(
                    (*avoidspots.offset(i as isize)).origin.as_mut_ptr(),
                    (*reach).start.as_mut_ptr(),
                ) > squareddist
            {
                type_0 = (*avoidspots.offset(i as isize)).type_0
            }
        } else {
            VectorDistanceSquared(
                (*avoidspots.offset(i as isize)).origin.as_mut_ptr(),
                (*reach).end.as_mut_ptr(),
            );
            // if moving towards the avoid spot
            //end if
            if squareddist < squaredradius
                && VectorDistanceSquared(
                    (*avoidspots.offset(i as isize)).origin.as_mut_ptr(),
                    (*reach).start.as_mut_ptr(),
                ) > squareddist
            {
                type_0 = (*avoidspots.offset(i as isize)).type_0
            }
        }

        if type_0 == 1 {
            return type_0;
        }
    }
    return type_0;
}
// if the reachability leads closer to the avoid spot
//add a spot to avoid (if type == AVOID_CLEAR all spots are removed)
//end of the function BotAvoidSpots
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAddAvoidSpot(
    mut movestate: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
    mut type_0: i32,
) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t; //end if
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return;
    }
    if type_0 == 0 {
        (*ms).numavoidspots = 0;
        return;
    }
    if (*ms).numavoidspots >= 32 {
        return;
    }
    (*ms).avoidspots[(*ms).numavoidspots as usize].origin[0] = *origin.offset(0);
    (*ms).avoidspots[(*ms).numavoidspots as usize].origin[1] = *origin.offset(1);
    (*ms).avoidspots[(*ms).numavoidspots as usize].origin[2] = *origin.offset(2);
    (*ms).avoidspots[(*ms).numavoidspots as usize].radius = radius;
    (*ms).avoidspots[(*ms).numavoidspots as usize].type_0 = type_0;
    (*ms).numavoidspots += 1;
}
//end of the function BotAddAvoidSpot
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGetReachabilityToGoal(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut areanum: i32,
    mut lastgoalareanum: i32,
    mut lastareanum: i32,
    mut avoidreach: *mut i32,
    mut avoidreachtimes: *mut f32,
    mut avoidreachtries: *mut i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
    mut travelflags: i32,
    mut avoidspots: *mut crate::src::botlib::be_ai_move::bot_avoidspot_s,
    mut numavoidspots: i32,
    mut flags: *mut i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut t: i32 = 0;
    let mut besttime: i32 = 0;
    let mut bestreachnum: i32 = 0;
    let mut reachnum: i32 = 0;
    let mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    //if not in a valid area
    if areanum == 0 {
        return 0i32;
    }
    //
    if crate::src::botlib::be_aas_reach::AAS_AreaDoNotEnter(areanum) != 0
        || crate::src::botlib::be_aas_reach::AAS_AreaDoNotEnter((*goal).areanum) != 0
    {
        travelflags |= 0x800000
    } //end if
      //use the routing to find the next area to go to
    besttime = 0;
    bestreachnum = 0;
    //
    reachnum = crate::src::botlib::be_aas_route::AAS_NextAreaReachability(areanum, 0); //end for
    while reachnum != 0 {
        //check if it isn't a reachability to avoid
        i = 0; //end for
        while i < 1 {
            if *avoidreach.offset(i as isize) == reachnum
                && *avoidreachtimes.offset(i as isize)
                    >= crate::src::botlib::be_aas_main::AAS_Time()
            {
                break;
            }
            i += 1
        }
        //end if
        if !(i != 1 && *avoidreachtries.offset(i as isize) > 4) {
            //end if
            //AVOIDREACH
            //get the reachability from the number
            crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach);
            //NOTE: do not go back to the previous area if the goal didn't change
            //NOTE: is this actually avoidance of local routing minima between two areas???
            if !(lastgoalareanum == (*goal).areanum && reach.areanum == lastareanum) {
                //if (AAS_AreaContentsTravelFlags(reach.areanum) & ~travelflags) continue;
                //if the travel isn't valid
                if !(BotValidTravel(origin, &mut reach, travelflags) == 0) {
                    //get the travel time
                    t = crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea(
                        reach.areanum,
                        reach.end.as_mut_ptr(),
                        (*goal).areanum,
                        travelflags,
                    );
                    //if the goal area isn't reachable from the reachable area
                    if !(t == 0) {
                        //if the bot should not use this reachability to avoid bad spots
                        if BotAvoidSpots(origin, &mut reach, avoidspots, numavoidspots) != 0 {
                            if !flags.is_null() {
                                *flags |= 256
                            }
                        } else {
                            //add the travel time towards the area
                            t += reach.traveltime as i32; // + AAS_AreaTravelTime(areanum, origin, reach.start);
                                                          //if the travel time is better than the ones already found
                            if besttime == 0 || t < besttime {
                                besttime = t;
                                bestreachnum = reachnum
                            }
                        }
                    }
                }
            }
        }
        reachnum = crate::src::botlib::be_aas_route::AAS_NextAreaReachability(areanum, reachnum)
    }
    //DEBUG
    //
    return bestreachnum;
}
//end of the function BotGetReachabilityToGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAddToTarget(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxdist: f32,
    mut dist: *mut f32,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //end if
    let mut curdist: f32 = 0.;
    dir[0] = *end.offset(0) - *start.offset(0);
    dir[1] = *end.offset(1) - *start.offset(1);
    dir[2] = *end.offset(2) - *start.offset(2);
    curdist = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    if *dist + curdist < maxdist {
        *target.offset(0) = *end.offset(0);
        *target.offset(1) = *end.offset(1);
        *target.offset(2) = *end.offset(2);
        *dist += curdist;
        return crate::src::qcommon::q_shared::qfalse as i32;
    } else {
        *target.offset(0) = *start.offset(0) + dir[0] * (maxdist - *dist);
        *target.offset(1) = *start.offset(1) + dir[1] * (maxdist - *dist);
        *target.offset(2) = *start.offset(2) + dir[2] * (maxdist - *dist);
        *dist = maxdist;
        return crate::src::qcommon::q_shared::qtrue as i32;
    };
    //end else
}
//view target based on movement
//end of the function BotAddToTarget
#[no_mangle]

pub unsafe extern "C" fn BotMovementViewTarget(
    mut movestate: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
    mut travelflags: i32,
    mut lookahead: f32,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    let mut reachnum: i32 = 0;
    let mut lastareanum: i32 = 0;
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the bot has no goal or no last reachability
    if (*ms).lastreachnum == 0 || goal.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    reachnum = (*ms).lastreachnum;
    end[0] = (*ms).origin[0];
    end[1] = (*ms).origin[1];
    end[2] = (*ms).origin[2];
    lastareanum = (*ms).lastareanum;
    dist = 0f32;
    //end if
    while reachnum != 0 && dist < lookahead {
        crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach); //end while
        if BotAddToTarget(
            end.as_mut_ptr(),
            reach.start.as_mut_ptr(),
            lookahead,
            &mut dist,
            target,
        ) != 0
        {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        if reach.traveltype & 0xffffff == 10 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        if reach.traveltype & 0xffffff == 12 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        if reach.traveltype & 0xffffff == 13 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        if reach.traveltype & 0xffffff != 18
            && reach.traveltype & 0xffffff != 11
            && reach.traveltype & 0xffffff != 19
        {
            if BotAddToTarget(
                reach.start.as_mut_ptr(),
                reach.end.as_mut_ptr(),
                lookahead,
                &mut dist,
                target,
            ) != 0
            {
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        reachnum = BotGetReachabilityToGoal(
            reach.end.as_mut_ptr(),
            reach.areanum,
            (*ms).lastgoalareanum,
            lastareanum,
            (*ms).avoidreach.as_mut_ptr(),
            (*ms).avoidreachtimes.as_mut_ptr(),
            (*ms).avoidreachtries.as_mut_ptr(),
            goal,
            travelflags,
            0 as *mut crate::src::botlib::be_ai_move::bot_avoidspot_s,
            0,
            0 as *mut i32,
        );
        end[0] = reach.end[0];
        end[1] = reach.end[1];
        end[2] = reach.end[2];
        lastareanum = reach.areanum;
        if lastareanum == (*goal).areanum {
            BotAddToTarget(
                reach.end.as_mut_ptr(),
                (*goal).origin.as_mut_ptr(),
                lookahead,
                &mut dist,
                target,
            );
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//never look beyond teleporters
//never look beyond the weapon jump point
//don't add jump pad distances
//end if
//
//end of the function BotMovementViewTarget
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotVisible(
    mut ent: i32,
    mut eye: *mut crate::src::qcommon::q_shared::vec_t,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        eye,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        target,
        ent,
        1 | 0x10000,
    );
    if trace.fraction >= 1f32 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//predict the position of a player based on movement towards a goal
//end of the function BotVisible
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotPredictVisiblePosition(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut areanum: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
    mut travelflags: i32,
    mut target: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    let mut reachnum: i32 = 0;
    let mut lastgoalareanum: i32 = 0;
    let mut lastareanum: i32 = 0;
    let mut i: i32 = 0;
    let mut avoidreach: [i32; 1] = [0; 1];
    let mut avoidreachtimes: [f32; 1] = [0.; 1];
    let mut avoidreachtries: [i32; 1] = [0; 1];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //if the bot has no goal or no last reachability
    if goal.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the areanum is not valid
    if areanum == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the goal areanum is not valid
    if (*goal).areanum == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    crate::stdlib::memset(
        avoidreach.as_mut_ptr() as *mut libc::c_void,
        0,
        (1usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    lastgoalareanum = (*goal).areanum;
    lastareanum = areanum;
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    //only do 20 hops
    i = 0; //end while
    while i < 20 && areanum != (*goal).areanum {
        //
        reachnum = BotGetReachabilityToGoal(
            end.as_mut_ptr(),
            areanum,
            lastgoalareanum,
            lastareanum,
            avoidreach.as_mut_ptr(),
            avoidreachtimes.as_mut_ptr(),
            avoidreachtries.as_mut_ptr(),
            goal,
            travelflags,
            0 as *mut crate::src::botlib::be_ai_move::bot_avoidspot_s,
            0,
            0 as *mut i32,
        );
        if reachnum == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach);
        //
        if BotVisible(
            (*goal).entitynum,
            (*goal).origin.as_mut_ptr(),
            reach.start.as_mut_ptr(),
        ) != 0
        {
            *target.offset(0) = reach.start[0];
            *target.offset(1) = reach.start[1];
            *target.offset(2) = reach.start[2];
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        if BotVisible(
            (*goal).entitynum,
            (*goal).origin.as_mut_ptr(),
            reach.end.as_mut_ptr(),
        ) != 0
        {
            *target.offset(0) = reach.end[0];
            *target.offset(1) = reach.end[1];
            *target.offset(2) = reach.end[2];
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        if reach.areanum == (*goal).areanum {
            *target.offset(0) = reach.end[0];
            *target.offset(1) = reach.end[1];
            *target.offset(2) = reach.end[2];
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        lastareanum = areanum;
        areanum = reach.areanum;
        end[0] = reach.end[0];
        end[1] = reach.end[1];
        end[2] = reach.end[2];
        i += 1
    }
    //
    //end if
    //
    //end if
    //
    //end if
    //
    //
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function BotPredictVisiblePosition
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn MoverBottomCenter(
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
    mut bottomcenter: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut modelnum: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mids: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 0f32];
    modelnum = (*reach).facenum & 0xffff;
    //get some bsp model info
    crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
        modelnum,
        angles.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        origin.as_mut_ptr(),
    );
    //
    if crate::src::botlib::be_aas_entity::AAS_OriginOfMoverWithModelNum(
        modelnum,
        origin.as_mut_ptr(),
    ) == 0
    {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1i32,
            b"no entity with model %d\n\x00" as *const u8 as *mut i8,
            modelnum,
        ); //end if
    }
    //get a point just above the plat in the bottom position
    mids[0] = mins[0] + maxs[0];
    mids[1] = mins[1] + maxs[1];
    mids[2] = mins[2] + maxs[2];
    *bottomcenter.offset(0) =
        (origin[0] as f64 + mids[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    *bottomcenter.offset(1) =
        (origin[1] as f64 + mids[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    *bottomcenter.offset(2) =
        (origin[2] as f64 + mids[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    *bottomcenter.offset(2) = (*reach).start[2];
}
//end of the function MoverBottomCenter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotGapDistance(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut hordir: *mut crate::src::qcommon::q_shared::vec_t,
    mut entnum: i32,
) -> f32 {
    let mut _dist: i32 = 0;
    let mut startz: f32 = 0.;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    //do gap checking
    //startz = origin[2];
    //this enables walking down stairs more fluidly
    start[0] = *origin.offset(0);
    start[1] = *origin.offset(1);
    start[2] = *origin.offset(2);
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    end[2] -= 60f32;
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        start.as_mut_ptr(),
        end.as_mut_ptr(),
        4,
        entnum,
    );
    if trace.fraction >= 1f32 {
        return 1f32;
    }
    startz = trace.endpos[2] + 1f32;
    //
    //end for
    for dist in (8..=100).step_by(8 as usize) {
        start[0] = *origin.offset(0) + *hordir.offset(0) * dist as f32;

        start[1] = *origin.offset(1) + *hordir.offset(1) * dist as f32;

        start[2] = *origin.offset(2) + *hordir.offset(2) * dist as f32;

        start[2] = startz + 24f32;

        end[0] = start[0];

        end[1] = start[1];

        end[2] = start[2];

        end[2] -= 48f32 + (*sv_maxbarrier).value;

        trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            4,
            entnum,
        );

        if trace.startsolid as u64 == 0 {
            //if it is a gap
            if trace.endpos[2] < startz - (*sv_maxstep).value - 8f32 {
                end[0] = trace.endpos[0]; //end if
                end[1] = trace.endpos[1];
                end[2] = trace.endpos[2];
                end[2] -= 20f32;
                if crate::src::botlib::be_aas_bspq3::AAS_PointContents(end.as_mut_ptr()) & 32 != 0 {
                    break;
                }
                //if a gap is found slow down
                //botimport.Print(PRT_MESSAGE, "gap at %i\n", dist);
                return dist as f32;
            } else {
                startz = trace.endpos[2]
            }
        }
    }
    return 0f32;
}
//end of the function BotGapDistance
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCheckBarrierJump(
    mut ms: *mut bot_movestate_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
) -> i32 {
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    end[0] = (*ms).origin[0];
    end[1] = (*ms).origin[1];
    end[2] = (*ms).origin[2];
    end[2] += (*sv_maxbarrier).value;
    //trace right up
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        (*ms).origin.as_mut_ptr(),
        end.as_mut_ptr(),
        2,
        (*ms).entitynum,
    );
    //this shouldn't happen... but we check anyway
    if trace.startsolid as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if very low ceiling it isn't possible to jump up to a barrier
    if trace.endpos[2] - (*ms).origin[2] < (*sv_maxstep).value {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    hordir[0] = *dir.offset(0);
    hordir[1] = *dir.offset(1);
    hordir[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    end[0] = ((*ms).origin[0] as f64 + hordir[0] as f64 * (((*ms).thinktime * speed) as f64 * 0.5))
        as crate::src::qcommon::q_shared::vec_t;
    end[1] = ((*ms).origin[1] as f64 + hordir[1] as f64 * (((*ms).thinktime * speed) as f64 * 0.5))
        as crate::src::qcommon::q_shared::vec_t;
    end[2] = ((*ms).origin[2] as f64 + hordir[2] as f64 * (((*ms).thinktime * speed) as f64 * 0.5))
        as crate::src::qcommon::q_shared::vec_t;
    start[0] = trace.endpos[0];
    start[1] = trace.endpos[1];
    start[2] = trace.endpos[2];
    end[2] = trace.endpos[2];
    //trace from previous trace end pos horizontally in the move direction
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        start.as_mut_ptr(),
        end.as_mut_ptr(),
        2,
        (*ms).entitynum,
    );
    //again this shouldn't happen
    if trace.startsolid as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    start[0] = trace.endpos[0];
    start[1] = trace.endpos[1];
    start[2] = trace.endpos[2];
    end[0] = trace.endpos[0];
    end[1] = trace.endpos[1];
    end[2] = trace.endpos[2];
    end[2] = (*ms).origin[2];
    //trace down from the previous trace end pos
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        start.as_mut_ptr(),
        end.as_mut_ptr(),
        2,
        (*ms).entitynum,
    );
    //if solid
    if trace.startsolid as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if no obstacle at all
    if trace.fraction as f64 >= 1.0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if less than the maximum step height
    if trace.endpos[2] - (*ms).origin[2] < (*sv_maxstep).value {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    crate::src::botlib::be_ea::EA_Jump((*ms).client);
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    (*ms).moveflags |= 1;
    //there is a barrier
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function BotCheckBarrierJump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSwimInDirection(
    mut ms: *mut bot_movestate_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
    mut _type_0: i32,
) -> i32 {
    let mut normdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    normdir[0] = *dir.offset(0);
    normdir[1] = *dir.offset(1);
    normdir[2] = *dir.offset(2);
    crate::src::qcommon::q_math::VectorNormalize(normdir.as_mut_ptr());
    crate::src::botlib::be_ea::EA_Move((*ms).client, normdir.as_mut_ptr(), speed);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function BotSwimInDirection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotWalkInDirection(
    mut ms: *mut bot_movestate_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
    mut type_0: i32,
) -> i32 {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tmpdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut presencetype: i32 = 0;
    let mut maxframes: i32 = 0;
    let mut cmdframes: i32 = 0;
    let mut stopevent: i32 = 0;
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    let mut dist: f32 = 0.;
    if crate::src::botlib::be_aas_move::AAS_OnGround(
        (*ms).origin.as_mut_ptr(),
        (*ms).presencetype,
        (*ms).entitynum,
    ) != 0
    {
        (*ms).moveflags |= 2
    }
    //if the bot is on the ground
    if (*ms).moveflags & 2 != 0 {
        if BotCheckBarrierJump(ms, dir, speed) != 0 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        } //end if
          //if there is a barrier the bot can jump on
          //remove barrier jump flag
        (*ms).moveflags &= !(1);
        //get the presence type for the movement
        if type_0 & 2 != 0 && type_0 & 4 == 0 {
            presencetype = 4
        } else {
            presencetype = 2
        }
        //horizontal direction
        hordir[0] = *dir.offset(0);
        hordir[1] = *dir.offset(1);
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //if the bot is not supposed to jump
        if type_0 & 4 == 0 {
            //end if
            //if there is a gap, try to jump over it
            if BotGapDistance(
                (*ms).origin.as_mut_ptr(),
                hordir.as_mut_ptr(),
                (*ms).entitynum,
            ) > 0f32
            {
                type_0 |= 4
            }
        }
        //get command movement
        cmdmove[0] = hordir[0] * speed;
        cmdmove[1] = hordir[1] * speed;
        cmdmove[2] = hordir[2] * speed;
        velocity[0] = (*ms).velocity[0];
        velocity[1] = (*ms).velocity[1];
        velocity[2] = (*ms).velocity[2];
        //
        if type_0 & 4 != 0 {
            //end else
            cmdmove[2] = 400f32; //end if
            maxframes = (3f64 / 0.1) as i32;
            cmdframes = 1;
            stopevent = 1 | 32 | 4 | 8 | 16
        } else {
            maxframes = 2;
            cmdframes = 2;
            stopevent = 32 | 4 | 8 | 16
        }
        //botimport.Print(PRT_MESSAGE, "trying jump\n");
        //AAS_ClearShownDebugLines();
        //
        origin[0] = (*ms).origin[0]; //qtrue);
        origin[1] = (*ms).origin[1];
        origin[2] = (*ms).origin[2];
        origin[2] = (origin[2] as f64 + 0.5) as crate::src::qcommon::q_shared::vec_t;
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            (*ms).entitynum,
            origin.as_mut_ptr(),
            presencetype,
            crate::src::qcommon::q_shared::qtrue as i32,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            cmdframes,
            maxframes,
            0.1,
            stopevent,
            0,
            crate::src::qcommon::q_shared::qfalse as i32,
        );
        //if prediction time wasn't enough to fully predict the movement
        if move_0.frames >= maxframes && type_0 & 4 != 0 {
            //end if
            //botimport.Print(PRT_MESSAGE, "client %d: max prediction frames\n", ms->client);
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //don't enter slime or lava and don't fall from too high
        if move_0.stopevent & (8 | 16 | 32) != 0 {
            //end if
            //botimport.Print(PRT_MESSAGE, "client %d: would be hurt ", ms->client);
            //if (move.stopevent & SE_ENTERSLIME) botimport.Print(PRT_MESSAGE, "slime\n");
            //if (move.stopevent & SE_ENTERLAVA) botimport.Print(PRT_MESSAGE, "lava\n");
            //if (move.stopevent & SE_HITGROUNDDAMAGE) botimport.Print(PRT_MESSAGE, "hitground\n");
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //if ground was hit
        if move_0.stopevent & 1 != 0 {
            //end if
            //check for nearby gap
            crate::src::qcommon::q_math::VectorNormalize2(
                move_0.velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                tmpdir.as_mut_ptr(),
            );
            dist = BotGapDistance(
                move_0.endpos.as_mut_ptr(),
                tmpdir.as_mut_ptr(),
                (*ms).entitynum,
            );
            if dist > 0f32 {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            //
            dist = BotGapDistance(
                move_0.endpos.as_mut_ptr(),
                hordir.as_mut_ptr(),
                (*ms).entitynum,
            );
            if dist > 0f32 {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        }
        //get horizontal movement
        tmpdir[0] = move_0.endpos[0] - (*ms).origin[0];
        tmpdir[1] = move_0.endpos[1] - (*ms).origin[1];
        tmpdir[2] = 0f32;
        //
        //AAS_DrawCross(move.endpos, 4, LINECOLOR_BLUE);
        //the bot is blocked by something
        if (VectorLength(tmpdir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) as f64)
            < (speed * (*ms).thinktime) as f64 * 0.5
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //perform the movement
        if type_0 & 4 != 0 {
            crate::src::botlib::be_ea::EA_Jump((*ms).client);
        }
        if type_0 & 2 != 0 {
            crate::src::botlib::be_ea::EA_Crouch((*ms).client);
        }
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
        //movement was succesfull
        return crate::src::qcommon::q_shared::qtrue as i32;
    } else {
        if (*ms).moveflags & 1 != 0 {
            //end if
            //if near the top or going down
            if (*ms).velocity[2] < 50f32 {
                crate::src::botlib::be_ea::EA_Move((*ms).client, dir, speed);
            }
            //end if
        }
        //FIXME: do air control to avoid hazards
        return crate::src::qcommon::q_shared::qtrue as i32;
    };
    //end else
}
//moves the bot in the specified direction using the specified type of movement
//end of the function BotWalkInDirection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotMoveInDirection(
    mut movestate: i32,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
    mut type_0: i32,
) -> i32 {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if swimming
    if crate::src::botlib::be_aas_move::AAS_Swimming((*ms).origin.as_mut_ptr()) != 0 {
        return BotSwimInDirection(ms, dir, speed, type_0);
    } else {
        return BotWalkInDirection(ms, dir, speed, type_0);
    }; //end if
       //end else
}
//end of the function BotMoveInDirection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Intersection(
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
    mut p3: *mut crate::src::qcommon::q_shared::vec_t,
    mut p4: *mut crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut x1: f32 = 0.; //end if
    let mut dx1: f32 = 0.;
    let mut dy1: f32 = 0.;
    let mut x2: f32 = 0.;
    let mut dx2: f32 = 0.;
    let mut dy2: f32 = 0.;
    let mut d: f32 = 0.;
    dx1 = *p2.offset(0) - *p1.offset(0);
    dy1 = *p2.offset(1) - *p1.offset(1);
    dx2 = *p4.offset(0) - *p3.offset(0);
    dy2 = *p4.offset(1) - *p3.offset(1);
    d = dy1 * dx2 - dx1 * dy2;
    if d != 0f32 {
        x1 = *p1.offset(1) * dx1 - *p1.offset(0) * dy1;
        x2 = *p3.offset(1) * dx2 - *p3.offset(0) * dy2;
        *out.offset(0) = ((dx1 * x2 - dx2 * x1) / d) as i32 as crate::src::qcommon::q_shared::vec_t;
        *out.offset(1) = ((dy1 * x2 - dy2 * x1) / d) as i32 as crate::src::qcommon::q_shared::vec_t;
        return crate::src::qcommon::q_shared::qtrue as i32;
    } else {
        return crate::src::qcommon::q_shared::qfalse as i32;
    };
    //end else
}
//end of the function Intersection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCheckBlocked(
    mut ms: *mut bot_movestate_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut checkbottom: i32,
    mut result: *mut crate::src::botlib::be_ai_move::bot_moveresult_t,
) {
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    //test for entities obstructing the bot's path
    crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox(
        (*ms).presencetype,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    //
    if crate::stdlib::fabs(
        (*dir.offset(0) * up[0] + *dir.offset(1) * up[1] + *dir.offset(2) * up[2]) as f64,
    ) < 0.7
    {
        //end if
        mins[2] += (*sv_maxstep).value;
        maxs[2] -= 10f32 //if the bot can step on
                         //a little lower to avoid low ceiling
    }
    end[0] = (*ms).origin[0] + *dir.offset(0) * 3f32;
    end[1] = (*ms).origin[1] + *dir.offset(1) * 3f32;
    end[2] = (*ms).origin[2] + *dir.offset(2) * 3f32;
    trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        (*ms).origin.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*ms).entitynum,
        1 | 0x10000 | 0x2000000,
    );
    //if not started in solid and not hitting the world entity
    if trace.startsolid as u64 == 0
        && (trace.ent != ((1) << 10) - 2 && trace.ent != ((1) << 10) - 1)
    {
        (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
        (*result).blockentity = trace.ent
    //DEBUG
    } else if checkbottom != 0
        && crate::src::botlib::be_aas_reach::AAS_AreaReachability((*ms).areanum) == 0
    {
        //if not in an area with reachability
        //check if the bot is standing on something
        crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox(
            (*ms).presencetype,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        end[0] = (*ms).origin[0] + up[0] * -3f32;
        end[1] = (*ms).origin[1] + up[1] * -3f32;
        end[2] = (*ms).origin[2] + up[2] * -3f32;
        trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
            (*ms).origin.as_mut_ptr(),
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
            end.as_mut_ptr(),
            (*ms).entitynum,
            1 | 0x10000,
        );
        if trace.startsolid as u64 == 0
            && (trace.ent != ((1) << 10) - 2 && trace.ent != ((1) << 10) - 1)
        {
            (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
            (*result).blockentity = trace.ent;
            (*result).flags |= 32
            //end if
            //DEBUG
        }
    };
    //end else
}
//end of the function BotCheckBlocked
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Walk(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //first walk straight to the reachability start
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //
    if dist < 10f32 {
        //end if
        //walk straight to the reachability end
        hordir[0] = (*reach).end[0] - (*ms).origin[0];
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr())
    }
    //if going towards a crouch area
    if crate::src::botlib::be_aas_sample::AAS_AreaPresenceType((*reach).areanum) & 2 == 0 {
        //end if
        //if pretty close to the reachable area
        if dist < 20f32 {
            crate::src::botlib::be_ea::EA_Crouch((*ms).client);
        }
    }
    //
    dist = BotGapDistance(
        (*ms).origin.as_mut_ptr(),
        hordir.as_mut_ptr(),
        (*ms).entitynum,
    );
    //
    if (*ms).moveflags & 512 != 0 {
        //end else
        if dist > 0f32 {
            speed = 200f32 - (180f32 - 1f32 * dist)
        } else {
            speed = 200f32
        } //end if
        crate::src::botlib::be_ea::EA_Walk((*ms).client);
    } else if dist > 0f32 {
        speed = 400f32 - (360f32 - 2f32 * dist)
    } else {
        speed = 400f32
    }
    //elemantary action move in direction
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_Walk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_Walk(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //if not on the ground and changed areas... don't walk back!!
    //(doesn't seem to help)
    /*
        ms->areanum = BotFuzzyPointReachabilityArea(ms->origin);
        if (ms->areanum == reach->areanum)
        {
    #ifdef DEBUG
            botimport.Print(PRT_MESSAGE, "BotFinishTravel_Walk: already in reach area\n");
    #endif //DEBUG
            return result;
        } //end if*/
    //go straight to the reachability end
    hordir[0] = (*reach).end[0] - (*ms).origin[0];
    hordir[1] = (*reach).end[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    if dist > 100f32 {
        dist = 100f32
    }
    speed = 400f32 - (400f32 - 3f32 * dist);
    //
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotFinishTravel_Walk
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Crouch(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut speed: f32 = 0.;
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //
    speed = 400f32;
    //walk straight to reachability end
    hordir[0] = (*reach).end[0] - (*ms).origin[0];
    hordir[1] = (*reach).end[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //elemantary actions
    crate::src::botlib::be_ea::EA_Crouch((*ms).client);
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    //
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_Crouch
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_BarrierJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //walk straight to reachability start
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //if pretty close to the barrier
    if dist < 9f32 {
        //end else
        crate::src::botlib::be_ea::EA_Jump((*ms).client); //end if
    } else {
        if dist > 60f32 {
            dist = 60f32
        }
        speed = 360f32 - (360f32 - 6f32 * dist);
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_BarrierJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_BarrierJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //if near the top or going down
    if (*ms).velocity[2] < 250f32 {
        hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        //
        BotCheckBlocked(
            ms,
            hordir.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue as i32,
            &mut result,
        );
        //
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 400f32);
        result.movedir[0] = hordir[0];
        result.movedir[1] = hordir[1];
        result.movedir[2] = hordir[2]
    }
    //
    return result;
}
//end of the function BotFinishTravel_BarrierJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Swim(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //swim straight to reachability end
    dir[0] = (*reach).start[0] - (*ms).origin[0];
    dir[1] = (*reach).start[1] - (*ms).origin[1];
    dir[2] = (*reach).start[2] - (*ms).origin[2];
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    //
    BotCheckBlocked(
        ms,
        dir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //elemantary actions
    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), 400f32);
    //
    result.movedir[0] = dir[0];
    result.movedir[1] = dir[1];
    result.movedir[2] = dir[2];
    crate::src::qcommon::q_math::vectoangles(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    result.flags |= 2;
    //
    return result;
}
//end of the function BotTravel_Swim
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_WaterJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //swim straight to reachability end
    dir[0] = (*reach).end[0] - (*ms).origin[0];
    dir[1] = (*reach).end[1] - (*ms).origin[1];
    dir[2] = (*reach).end[2] - (*ms).origin[2];
    hordir[0] = dir[0];
    hordir[1] = dir[1];
    hordir[2] = dir[2];
    hordir[2] = 0f32;
    dir[2] = (dir[2] as f64
        + (15f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 40f64))
        as crate::src::qcommon::q_shared::vec_t;
    //botimport.Print(PRT_MESSAGE, "BotTravel_WaterJump: dir[2] = %f\n", dir[2]);
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //elemantary actions
    //EA_Move(ms->client, dir, 400);
    crate::src::botlib::be_ea::EA_MoveForward((*ms).client);
    //move up if close to the actual out of water jump spot
    if dist < 40f32 {
        crate::src::botlib::be_ea::EA_MoveUp((*ms).client);
    }
    //set the ideal view angles
    crate::src::qcommon::q_math::vectoangles(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    result.flags |= 1;
    //
    result.movedir[0] = dir[0];
    result.movedir[1] = dir[1];
    result.movedir[2] = dir[2];
    //
    return result;
}
//end of the function BotTravel_WaterJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_WaterJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut pnt: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //botimport.Print(PRT_MESSAGE, "BotFinishTravel_WaterJump\n");
    //if waterjumping there's nothing to do
    if (*ms).moveflags & 16 != 0 {
        return result;
    }
    //if not touching any water anymore don't do anything
    //otherwise the bot sometimes keeps jumping?
    pnt[0] = (*ms).origin[0]; //extra for q2dm4 near red armor/mega health
    pnt[1] = (*ms).origin[1];
    pnt[2] = (*ms).origin[2];
    pnt[2] -= 32f32;
    if crate::src::botlib::be_aas_bspq3::AAS_PointContents(pnt.as_mut_ptr()) & (8 | 16 | 32) == 0 {
        return result;
    }
    //swim straight to reachability end
    dir[0] = (*reach).end[0] - (*ms).origin[0];
    dir[1] = (*reach).end[1] - (*ms).origin[1];
    dir[2] = (*reach).end[2] - (*ms).origin[2];
    dir[0] = (dir[0] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 10f64)
        as crate::src::qcommon::q_shared::vec_t;
    dir[1] = (dir[1] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 10f64)
        as crate::src::qcommon::q_shared::vec_t;
    dir[2] = (dir[2] as f64
        + (70f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 10f64))
        as crate::src::qcommon::q_shared::vec_t;
    //elemantary actions
    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), 400f32);
    //set the ideal view angles
    crate::src::qcommon::q_math::vectoangles(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    result.flags |= 1;
    //
    result.movedir[0] = dir[0];
    result.movedir[1] = dir[1];
    result.movedir[2] = dir[2];
    //
    return result;
}
//end of the function BotFinishTravel_WaterJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_WalkOffLedge(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut reachhordist: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //check if the bot is blocked by anything
    dir[0] = (*reach).start[0] - (*ms).origin[0];
    dir[1] = (*reach).start[1] - (*ms).origin[1];
    dir[2] = (*reach).start[2] - (*ms).origin[2];
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    BotCheckBlocked(
        ms,
        dir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //if the reachability start and end are practically above each other
    dir[0] = (*reach).end[0] - (*reach).start[0];
    dir[1] = (*reach).end[1] - (*reach).start[1];
    dir[2] = (*reach).end[2] - (*reach).start[2];
    dir[2] = 0f32;
    reachhordist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    //walk straight to the reachability start
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //if pretty close to the start focus on the reachability end
    if dist < 48f32 {
        hordir[0] = (*reach).end[0] - (*ms).origin[0];
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr()); //end else
                                                                           //end if
        if reachhordist < 20f32 {
            //
            speed = 100f32
        } else if crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
            0f32,
            (*reach).start.as_mut_ptr(),
            (*reach).end.as_mut_ptr(),
            &mut speed,
        ) == 0
        {
            speed = 400f32
        }
    } else if reachhordist < 20f32 {
        //end if
        if dist > 64f32 {
            dist = 64f32
        } //end if
        speed = 400f32 - (256f32 - 4f32 * dist)
    } else {
        speed = 400f32
    }
    //end else
    //
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //elemantary action
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_WalkOffLedge
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotAirControl(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut goal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: *mut f32,
) -> i32 {
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //end for
    let mut vel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut _i: i32 = 0;
    org[0] = *origin.offset(0);
    org[1] = *origin.offset(1);
    org[2] = *origin.offset(2);
    vel[0] = (*velocity.offset(0) as f64 * 0.1) as crate::src::qcommon::q_shared::vec_t;
    vel[1] = (*velocity.offset(1) as f64 * 0.1) as crate::src::qcommon::q_shared::vec_t;
    vel[2] = (*velocity.offset(2) as f64 * 0.1) as crate::src::qcommon::q_shared::vec_t;

    for _i in 0..50 {
        vel[2] = (vel[2] as f64 - (*sv_gravity).value as f64 * 0.01)
            as crate::src::qcommon::q_shared::vec_t;

        if vel[2] < 0f32 && org[2] + vel[2] < *goal.offset(2) {
            //if going down and next position would be below the goal
            vel[0] = vel[0] * ((*goal.offset(2) - org[2]) / vel[2]); //end if
            vel[1] = vel[1] * ((*goal.offset(2) - org[2]) / vel[2]);
            vel[2] = vel[2] * ((*goal.offset(2) - org[2]) / vel[2]);
            org[0] = org[0] + vel[0];
            org[1] = org[1] + vel[1];
            org[2] = org[2] + vel[2];
            *dir.offset(0) = *goal.offset(0) - org[0];
            *dir.offset(1) = *goal.offset(1) - org[1];
            *dir.offset(2) = *goal.offset(2) - org[2];
            dist = crate::src::qcommon::q_math::VectorNormalize(dir);
            if dist > 32f32 {
                dist = 32f32
            }
            *speed = 400f32 - (400f32 - 13f32 * dist);
            return crate::src::qcommon::q_shared::qtrue as i32;
        } else {
            org[0] = org[0] + vel[0];
            org[1] = org[1] + vel[1];
            org[2] = org[2] + vel[2]
        }
    }
    *dir.offset(0) = 0f32;
    *dir.offset(1) = 0f32;
    *dir.offset(2) = 0f32;
    *speed = 400f32;
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function BotAirControl
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_WalkOffLedge(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //
    dir[0] = (*reach).end[0] - (*ms).origin[0];
    dir[1] = (*reach).end[1] - (*ms).origin[1];
    dir[2] = (*reach).end[2] - (*ms).origin[2];
    BotCheckBlocked(
        ms,
        dir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //
    v[0] = (*reach).end[0] - (*ms).origin[0];
    v[1] = (*reach).end[1] - (*ms).origin[1];
    v[2] = (*reach).end[2] - (*ms).origin[2];
    v[2] = 0f32;
    dist = crate::src::qcommon::q_math::VectorNormalize(v.as_mut_ptr());
    if dist > 16f32 {
        end[0] = (*reach).end[0] + v[0] * 16f32;
        end[1] = (*reach).end[1] + v[1] * 16f32;
        end[2] = (*reach).end[2] + v[2] * 16f32
    } else {
        end[0] = (*reach).end[0];
        end[1] = (*reach).end[1];
        end[2] = (*reach).end[2]
    }
    //
    if BotAirControl(
        (*ms).origin.as_mut_ptr(),
        (*ms).velocity.as_mut_ptr(),
        end.as_mut_ptr(),
        hordir.as_mut_ptr(),
        &mut speed,
    ) == 0
    {
        //end if
        //go straight to the reachability end
        hordir[0] = dir[0];
        hordir[1] = dir[1];
        hordir[2] = dir[2];
        hordir[2] = 0f32;
        //
        speed = 400f32
    }
    //
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotFinishTravel_WalkOffLedge
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
/*
bot_moveresult_t BotTravel_Jump(bot_movestate_t *ms, aas_reachability_t *reach)
{
    vec3_t hordir;
    float dist, gapdist, speed, horspeed, sv_jumpvel;
    bot_moveresult_t_cleared( result );

    //
    sv_jumpvel = botlibglobals.sv_jumpvel->value;
    //walk straight to the reachability start
    hordir[0] = reach->start[0] - ms->origin[0];
    hordir[1] = reach->start[1] - ms->origin[1];
    hordir[2] = 0;
    dist = VectorNormalize(hordir);
    //
    speed = 350;
    //
    gapdist = BotGapDistance(ms, hordir, ms->entitynum);
    //if pretty close to the start focus on the reachability end
    if (dist < 50 || (gapdist && gapdist < 50))
    {
        //NOTE: using max speed (400) works best
        //if (AAS_HorizontalVelocityForJump(sv_jumpvel, ms->origin, reach->end, &horspeed))
        //{
        //	speed = horspeed * 400 / botlibglobals.sv_maxwalkvelocity->value;
        //} //end if
        hordir[0] = reach->end[0] - ms->origin[0];
        hordir[1] = reach->end[1] - ms->origin[1];
        VectorNormalize(hordir);
        //elemantary action jump
        EA_Jump(ms->client);
        //
        ms->jumpreach = ms->lastreachnum;
        speed = 600;
    } //end if
    else
    {
        if (AAS_HorizontalVelocityForJump(sv_jumpvel, reach->start, reach->end, &horspeed))
        {
            speed = horspeed * 400 / botlibglobals.sv_maxwalkvelocity->value;
        } //end if
    } //end else
    //elemantary action
    EA_Move(ms->client, hordir, speed);
    VectorCopy(hordir, result.movedir);
    //
    return result;
} //end of the function BotTravel_Jump*/
/*
bot_moveresult_t BotTravel_Jump(bot_movestate_t *ms, aas_reachability_t *reach)
{
    vec3_t hordir, dir1, dir2, mins, maxs, start, end;
    int gapdist;
    float dist1, dist2, speed;
    bot_moveresult_t_cleared( result );
    bsp_trace_t trace;

    //
    hordir[0] = reach->start[0] - reach->end[0];
    hordir[1] = reach->start[1] - reach->end[1];
    hordir[2] = 0;
    VectorNormalize(hordir);
    //
    VectorCopy(reach->start, start);
    start[2] += 1;
    //minus back the bouding box size plus 16
    VectorMA(reach->start, 80, hordir, end);
    //
    AAS_PresenceTypeBoundingBox(PRESENCE_NORMAL, mins, maxs);
    //check for solids
    trace = AAS_Trace(start, mins, maxs, end, ms->entitynum, MASK_PLAYERSOLID);
    if (trace.startsolid) VectorCopy(start, trace.endpos);
    //check for a gap
    for (gapdist = 0; gapdist < 80; gapdist += 10)
    {
        VectorMA(start, gapdist+10, hordir, end);
        end[2] += 1;
        if (AAS_PointAreaNum(end) != ms->reachareanum) break;
    } //end for
    if (gapdist < 80) VectorMA(reach->start, gapdist, hordir, trace.endpos);
//	dist1 = BotGapDistance(start, hordir, ms->entitynum);
//	if (dist1 && dist1 <= trace.fraction * 80) VectorMA(reach->start, dist1-20, hordir, trace.endpos);
    //
    VectorSubtract(ms->origin, reach->start, dir1);
    dir1[2] = 0;
    dist1 = VectorNormalize(dir1);
    VectorSubtract(ms->origin, trace.endpos, dir2);
    dir2[2] = 0;
    dist2 = VectorNormalize(dir2);
    //if just before the reachability start
    if (DotProduct(dir1, dir2) < -0.8 || dist2 < 5)
    {
        //botimport.Print(PRT_MESSAGE, "between jump start and run to point\n");
        hordir[0] = reach->end[0] - ms->origin[0];
        hordir[1] = reach->end[1] - ms->origin[1];
        hordir[2] = 0;
        VectorNormalize(hordir);
        //elemantary action jump
        if (dist1 < 24) EA_Jump(ms->client);
        else if (dist1 < 32) EA_DelayedJump(ms->client);
        EA_Move(ms->client, hordir, 600);
        //
        ms->jumpreach = ms->lastreachnum;
    } //end if
    else
    {
        //botimport.Print(PRT_MESSAGE, "going towards run to point\n");
        hordir[0] = trace.endpos[0] - ms->origin[0];
        hordir[1] = trace.endpos[1] - ms->origin[1];
        hordir[2] = 0;
        VectorNormalize(hordir);
        //
        if (dist2 > 80) dist2 = 80;
        speed = 400 - (400 - 5 * dist2);
        EA_Move(ms->client, hordir, speed);
    } //end else
    VectorCopy(hordir, result.movedir);
    //
    return result;
} //end of the function BotTravel_Jump*/
//*
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Jump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut runstart: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //	vec3_t runstart, dir1, dir2, hordir;
    let mut gapdist: i32 = 0;
    let mut dist1: f32 = 0.;
    let mut dist2: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //
    crate::src::botlib::be_aas_move::AAS_JumpReachRunStart(reach, runstart.as_mut_ptr());
    //*
    hordir[0] = runstart[0] - (*reach).start[0];
    hordir[1] = runstart[1] - (*reach).start[1];
    hordir[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    start[0] = (*reach).start[0];
    start[1] = (*reach).start[1];
    start[2] = (*reach).start[2];
    start[2] += 1f32;
    runstart[0] = (*reach).start[0] + hordir[0] * 80f32;
    runstart[1] = (*reach).start[1] + hordir[1] * 80f32;
    runstart[2] = (*reach).start[2] + hordir[2] * 80f32;
    //check for a gap
    gapdist = 0; //end for
    while gapdist < 80 {
        end[0] = start[0] + hordir[0] * (gapdist + 10) as f32;
        end[1] = start[1] + hordir[1] * (gapdist + 10) as f32;
        end[2] = start[2] + hordir[2] * (gapdist + 10) as f32;
        end[2] += 1f32;
        if crate::src::botlib::be_aas_sample::AAS_PointAreaNum(end.as_mut_ptr())
            != (*ms).reachareanum
        {
            break;
        }
        gapdist += 10
    }
    if gapdist < 80 {
        runstart[0] = (*reach).start[0] + hordir[0] * gapdist as f32;
        runstart[1] = (*reach).start[1] + hordir[1] * gapdist as f32;
        runstart[2] = (*reach).start[2] + hordir[2] * gapdist as f32
    }
    //
    dir1[0] = (*ms).origin[0] - (*reach).start[0];
    dir1[1] = (*ms).origin[1] - (*reach).start[1];
    dir1[2] = (*ms).origin[2] - (*reach).start[2];
    dir1[2] = 0f32;
    dist1 = crate::src::qcommon::q_math::VectorNormalize(dir1.as_mut_ptr());
    dir2[0] = (*ms).origin[0] - runstart[0];
    dir2[1] = (*ms).origin[1] - runstart[1];
    dir2[2] = (*ms).origin[2] - runstart[2];
    dir2[2] = 0f32;
    dist2 = crate::src::qcommon::q_math::VectorNormalize(dir2.as_mut_ptr());
    //if just before the reachability start
    if ((dir1[0] * dir2[0] + dir1[1] * dir2[1] + dir1[2] * dir2[2]) as f64) < -0.8 || dist2 < 5f32 {
        //end else
        hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //		botimport.Print(PRT_MESSAGE, "between jump start and run start point\n");
        //elemantary action jump
        if dist1 < 24f32 {
            crate::src::botlib::be_ea::EA_Jump((*ms).client);
        } else if dist1 < 32f32 {
            crate::src::botlib::be_ea::EA_DelayedJump((*ms).client);
        }
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 600f32);
        //
        (*ms).jumpreach = (*ms).lastreachnum
    } else {
        //		botimport.Print(PRT_MESSAGE, "going towards run start point\n");
        hordir[0] = runstart[0] - (*ms).origin[0];
        hordir[1] = runstart[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //
        if dist2 > 80f32 {
            dist2 = 80f32
        }
        speed = 400f32 - (400f32 - 5f32 * dist2);
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_Jump*/
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_Jump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut speed: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //if not jumped yet
    if (*ms).jumpreach == 0 {
        return result;
    }
    //go straight to the reachability end
    hordir[0] = (*reach).end[0] - (*ms).origin[0];
    hordir[1] = (*reach).end[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    hordir2[0] = (*reach).end[0] - (*reach).start[0];
    hordir2[1] = (*reach).end[1] - (*reach).start[1];
    hordir2[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(hordir2.as_mut_ptr());
    //
    if ((hordir[0] * hordir2[0] + hordir[1] * hordir2[1] + hordir[2] * hordir2[2]) as f64) < -0.5
        && dist < 24f32
    {
        return result;
    }
    //always use max speed when traveling through the air
    speed = 800f32;
    //
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotFinishTravel_Jump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Ladder(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    //float dist, speed;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //, hordir;
    let mut viewdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 0f32];
    //	vec3_t up = {0, 0, 1};
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //
    //	if ((ms->moveflags & MFL_AGAINSTLADDER))
    //NOTE: not a good idea for ladders starting in water
    // || !(ms->moveflags & MFL_ONGROUND))
    dir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
    dir[1] = (*reach).end[1] - (*ms).origin[1];
    dir[2] = (*reach).end[2] - (*ms).origin[2];
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    viewdir[0] = dir[0];
    viewdir[1] = dir[1];
    viewdir[2] = 3f32 * dir[2];
    crate::src::qcommon::q_math::vectoangles(
        viewdir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    crate::src::botlib::be_ea::EA_Move((*ms).client, origin.as_mut_ptr(), 0f32);
    crate::src::botlib::be_ea::EA_MoveForward((*ms).client);
    result.flags |= 1;
    //botimport.Print(PRT_MESSAGE, "against ladder or not on ground\n");
    //set the ideal view angles, facing the ladder up or down
    //elemantary action
    //set movement view flag so the AI can see the view is focussed
    /*	else
    {
        //botimport.Print(PRT_MESSAGE, "moving towards ladder\n");
        VectorSubtract(reach->end, ms->origin, dir);
        //make sure the horizontal movement is large enough
        VectorCopy(dir, hordir);
        hordir[2] = 0;
        dist = VectorNormalize(hordir);
        //
        dir[0] = hordir[0];
        dir[1] = hordir[1];
        if (dir[2] > 0) dir[2] = 1;
        else dir[2] = -1;
        if (dist > 50) dist = 50;
        speed = 400 - (200 - 4 * dist);
        EA_Move(ms->client, dir, speed);
    } //end else*/
    //save the movement direction
    result.movedir[0] = dir[0];
    result.movedir[1] = dir[1];
    result.movedir[2] = dir[2];
    return result;
}
//
//end of the function BotTravel_Ladder
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Teleport(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //if the bot is being teleported
    if (*ms).moveflags & 32 != 0 {
        return result;
    }
    //walk straight to center of the teleporter
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = (*reach).start[2] - (*ms).origin[2];
    if (*ms).moveflags & 4 == 0 {
        hordir[2] = 0f32
    }
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    if dist < 30f32 {
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 200f32);
    } else {
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 400f32);
    }
    if (*ms).moveflags & 4 != 0 {
        result.flags |= 2
    }
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    return result;
}
//end of the function BotTravel_Teleport
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Elevator(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottomcenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut dist1: f32 = 0.;
    let mut dist2: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //if standing on the plat
    if BotOnMover((*ms).origin.as_mut_ptr(), (*ms).entitynum, reach) != 0 {
        //end else
        //DEBUG_ELEVATOR
        //if vertically not too far from the end point
        if crate::stdlib::fabsf((*ms).origin[2] - (*reach).end[2]) < (*sv_maxbarrier).value {
            hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end else
            hordir[1] = (*reach).end[1] - (*ms).origin[1];
            hordir[2] = (*reach).end[2] - (*ms).origin[2];
            hordir[2] = 0f32;
            crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
            //DEBUG_ELEVATOR
            //move to the end point
            if BotCheckBarrierJump(ms, hordir.as_mut_ptr(), 100f32) == 0 {
                crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 400f32);
                //end if
            }
            result.movedir[0] = hordir[0];
            result.movedir[1] = hordir[1];
            result.movedir[2] = hordir[2]
        } else {
            //if not really close to the center of the elevator
            MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
            hordir[0] = bottomcenter[0] - (*ms).origin[0];
            hordir[1] = bottomcenter[1] - (*ms).origin[1];
            hordir[2] = bottomcenter[2] - (*ms).origin[2];
            hordir[2] = 0f32;
            dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
            //end if
            if dist > 10f32 {
                //
                //DEBUG_ELEVATOR
                //move to the center of the plat
                if dist > 100f32 {
                    dist = 100f32
                }
                speed = 400f32 - (400f32 - 4f32 * dist);
                //
                crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
                result.movedir[0] = hordir[0];
                result.movedir[1] = hordir[1];
                result.movedir[2] = hordir[2]
            }
        }
    //end else
    } else {
        //DEBUG_ELEVATOR
        //if very near the reachability end
        dir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        dir[1] = (*reach).end[1] - (*ms).origin[1];
        dir[2] = (*reach).end[2] - (*ms).origin[2];
        dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if dist < 64f32 {
            if dist > 60f32 {
                dist = 60f32
            }
            speed = 360f32 - (360f32 - 6f32 * dist);
            //
            if (*ms).moveflags & 4 != 0 || BotCheckBarrierJump(ms, dir.as_mut_ptr(), 50f32) == 0 {
                if speed > 5f32 {
                    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                    //end if
                }
            }
            result.movedir[0] = dir[0];
            result.movedir[1] = dir[1];
            result.movedir[2] = dir[2];
            //
            if (*ms).moveflags & 4 != 0 {
                result.flags |= 2
            }
            //stop using this reachability
            (*ms).reachability_time = 0f32;
            return result;
        }
        //get direction and distance to reachability start
        dir1[0] = (*reach).start[0] - (*ms).origin[0];
        dir1[1] = (*reach).start[1] - (*ms).origin[1];
        dir1[2] = (*reach).start[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            dir1[2] = 0f32
        }
        dist1 = crate::src::qcommon::q_math::VectorNormalize(dir1.as_mut_ptr());
        //if the elevator isn't down
        if MoverDown(reach) == 0 {
            //end if
            //DEBUG_ELEVATOR
            dist = dist1;
            dir[0] = dir1[0];
            dir[1] = dir1[1];
            dir[2] = dir1[2];
            //
            BotCheckBlocked(
                ms,
                dir.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse as i32,
                &mut result,
            );
            //
            if dist > 60f32 {
                dist = 60f32
            }
            speed = 360f32 - (360f32 - 6f32 * dist);
            //
            if (*ms).moveflags & 4 == 0 && BotCheckBarrierJump(ms, dir.as_mut_ptr(), 50f32) == 0 {
                if speed > 5f32 {
                    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                    //end if
                }
            }
            result.movedir[0] = dir[0];
            result.movedir[1] = dir[1];
            result.movedir[2] = dir[2];
            //
            if (*ms).moveflags & 4 != 0 {
                result.flags |= 2
            }
            //this isn't a failure... just wait till the elevator comes down
            result.type_0 = 1;
            result.flags |= 4;
            return result;
        }
        //get direction and distance to elevator bottom center
        MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
        dir2[0] = bottomcenter[0] - (*ms).origin[0];
        dir2[1] = bottomcenter[1] - (*ms).origin[1];
        dir2[2] = bottomcenter[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            dir2[2] = 0f32
        }
        dist2 = crate::src::qcommon::q_math::VectorNormalize(dir2.as_mut_ptr());
        //if very close to the reachability start or
        //closer to the elevator center or
        //between reachability start and elevator center
        if dist1 < 20f32
            || dist2 < dist1
            || dir1[0] * dir2[0] + dir1[1] * dir2[1] + dir1[2] * dir2[2] < 0f32
        {
            //end else
            dist = dist2; //end if
            dir[0] = dir2[0];
            dir[1] = dir2[1];
            dir[2] = dir2[2]
        } else {
            //DEBUG_ELEVATOR
            //closer to the reachability start
            //DEBUG_ELEVATOR
            dist = dist1;
            dir[0] = dir1[0];
            dir[1] = dir1[1];
            dir[2] = dir1[2]
        }
        //
        BotCheckBlocked(
            ms,
            dir.as_mut_ptr(),
            crate::src::qcommon::q_shared::qfalse as i32,
            &mut result,
        );
        //
        if dist > 60f32 {
            dist = 60f32
        }
        speed = 400f32 - (400f32 - 6f32 * dist);
        //
        if (*ms).moveflags & 4 == 0 && BotCheckBarrierJump(ms, dir.as_mut_ptr(), 50f32) == 0 {
            crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
            //end if
        }
        result.movedir[0] = dir[0];
        result.movedir[1] = dir[1];
        result.movedir[2] = dir[2];
        //
        if (*ms).moveflags & 4 != 0 {
            result.flags |= 2
        }
    }
    return result;
}
//end of the function BotTravel_Elevator
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_Elevator(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut bottomcenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottomdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut topdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //
    MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
    bottomdir[0] = bottomcenter[0] - (*ms).origin[0];
    bottomdir[1] = bottomcenter[1] - (*ms).origin[1];
    bottomdir[2] = bottomcenter[2] - (*ms).origin[2];
    //
    topdir[0] = (*reach).end[0] - (*ms).origin[0];
    topdir[1] = (*reach).end[1] - (*ms).origin[1];
    topdir[2] = (*reach).end[2] - (*ms).origin[2];
    //
    if crate::stdlib::fabs(bottomdir[2] as f64) < crate::stdlib::fabs(topdir[2] as f64) {
        //end else
        crate::src::qcommon::q_math::VectorNormalize(bottomdir.as_mut_ptr()); //end if
        crate::src::botlib::be_ea::EA_Move((*ms).client, bottomdir.as_mut_ptr(), 300f32);
    } else {
        crate::src::qcommon::q_math::VectorNormalize(topdir.as_mut_ptr());
        crate::src::botlib::be_ea::EA_Move((*ms).client, topdir.as_mut_ptr(), 300f32);
    }
    return result;
}
//end of the function BotFinishTravel_Elevator
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFuncBobStartEnd(
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut spawnflags: i32 = 0; //end if
    let mut modelnum: i32 = 0; //end if
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 0f32];
    let mut num0: i32 = 0;
    let mut num1: i32 = 0;
    modelnum = (*reach).facenum & 0xffff;
    if crate::src::botlib::be_aas_entity::AAS_OriginOfMoverWithModelNum(modelnum, origin) == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"BotFuncBobStartEnd: no entity with model %d\n\x00" as *const u8 as *mut i8,
            modelnum,
        );
        *start.offset(0) = 0f32;
        *start.offset(1) = 0f32;
        *start.offset(2) = 0f32;
        *end.offset(0) = 0f32;
        *end.offset(1) = 0f32;
        *end.offset(2) = 0f32;
        return;
    }
    crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
        modelnum,
        angles.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    mid[0] = mins[0] + maxs[0];
    mid[1] = mins[1] + maxs[1];
    mid[2] = mins[2] + maxs[2];
    mid[0] = (mid[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    mid[1] = (mid[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    mid[2] = (mid[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    *start.offset(0) = mid[0];
    *start.offset(1) = mid[1];
    *start.offset(2) = mid[2];
    *end.offset(0) = mid[0];
    *end.offset(1) = mid[1];
    *end.offset(2) = mid[2];
    spawnflags = (*reach).facenum >> 16;
    num0 = (*reach).edgenum >> 16;
    if num0 > 0x7fff {
        num0 = (num0 as u32 | 0xffff0000) as i32
    }
    num1 = (*reach).edgenum & 0xffff;
    if num1 > 0x7fff {
        num1 = (num1 as u32 | 0xffff0000) as i32
    }
    if spawnflags & 1 != 0 {
        *start.offset(0) = num0 as crate::src::qcommon::q_shared::vec_t;
        *end.offset(0) = num1 as crate::src::qcommon::q_shared::vec_t;
        //
        let ref mut fresh0 = *origin.offset(0); //end else if
        *fresh0 += mid[0];
        *origin.offset(1) = mid[1];
        *origin.offset(2) = mid[2]
    } else if spawnflags & 2 != 0 {
        *start.offset(1) = num0 as crate::src::qcommon::q_shared::vec_t;
        *end.offset(1) = num1 as crate::src::qcommon::q_shared::vec_t;
        //
        *origin.offset(0) = mid[0];
        let ref mut fresh1 = *origin.offset(1);
        *fresh1 += mid[1];
        *origin.offset(2) = mid[2]
    } else {
        *start.offset(2) = num0 as crate::src::qcommon::q_shared::vec_t;
        *end.offset(2) = num1 as crate::src::qcommon::q_shared::vec_t;
        //
        *origin.offset(0) = mid[0];
        *origin.offset(1) = mid[1];
        let ref mut fresh2 = *origin.offset(2);
        *fresh2 += mid[2]
    };
    //end else
}
//end of the function BotFuncBobStartEnd
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_FuncBobbing(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottomcenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bob_start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bob_end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bob_origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut dist1: f32 = 0.;
    let mut dist2: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //
    BotFuncBobStartEnd(
        reach,
        bob_start.as_mut_ptr(),
        bob_end.as_mut_ptr(),
        bob_origin.as_mut_ptr(),
    );
    //if standing ontop of the func_bobbing
    if BotOnMover((*ms).origin.as_mut_ptr(), (*ms).entitynum, reach) != 0 {
        //end else
        //if near end point of reachability
        dir[0] = bob_origin[0] - bob_end[0];
        dir[1] = bob_origin[1] - bob_end[1];
        dir[2] = bob_origin[2] - bob_end[2];
        if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < 24f32 {
            //end else
            hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end else
            hordir[1] = (*reach).end[1] - (*ms).origin[1];
            hordir[2] = (*reach).end[2] - (*ms).origin[2];
            hordir[2] = 0f32;
            crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
            //move to the end point
            if BotCheckBarrierJump(ms, hordir.as_mut_ptr(), 100f32) == 0 {
                crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 400f32);
                //end if
            }
            result.movedir[0] = hordir[0];
            result.movedir[1] = hordir[1];
            result.movedir[2] = hordir[2]
        } else {
            //if not really close to the center of the elevator
            MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
            hordir[0] = bottomcenter[0] - (*ms).origin[0];
            hordir[1] = bottomcenter[1] - (*ms).origin[1];
            hordir[2] = bottomcenter[2] - (*ms).origin[2];
            hordir[2] = 0f32;
            dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
            //end if
            if dist > 10f32 {
                //
                //move to the center of the plat
                if dist > 100f32 {
                    dist = 100f32
                }
                speed = 400f32 - (400f32 - 4f32 * dist);
                //
                crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
                result.movedir[0] = hordir[0];
                result.movedir[1] = hordir[1];
                result.movedir[2] = hordir[2]
            }
        }
    } else {
        //if very near the reachability end
        dir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        dir[1] = (*reach).end[1] - (*ms).origin[1];
        dir[2] = (*reach).end[2] - (*ms).origin[2];
        dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if dist < 64f32 {
            if dist > 60f32 {
                dist = 60f32
            }
            speed = 360f32 - (360f32 - 6f32 * dist);
            //if swimming or no barrier jump
            if (*ms).moveflags & 4 != 0 || BotCheckBarrierJump(ms, dir.as_mut_ptr(), 50f32) == 0 {
                if speed > 5f32 {
                    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                    //end if
                }
            }
            result.movedir[0] = dir[0];
            result.movedir[1] = dir[1];
            result.movedir[2] = dir[2];
            //
            if (*ms).moveflags & 4 != 0 {
                result.flags |= 2
            }
            //stop using this reachability
            (*ms).reachability_time = 0f32;
            return result;
        }
        //get direction and distance to reachability start
        dir1[0] = (*reach).start[0] - (*ms).origin[0];
        dir1[1] = (*reach).start[1] - (*ms).origin[1];
        dir1[2] = (*reach).start[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            dir1[2] = 0f32
        }
        dist1 = crate::src::qcommon::q_math::VectorNormalize(dir1.as_mut_ptr());
        //if func_bobbing is Not its start position
        dir[0] = bob_origin[0] - bob_start[0]; //end if
        dir[1] = bob_origin[1] - bob_start[1];
        dir[2] = bob_origin[2] - bob_start[2];
        if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) > 16f32 {
            dist = dist1;
            dir[0] = dir1[0];
            dir[1] = dir1[1];
            dir[2] = dir1[2];
            //
            BotCheckBlocked(
                ms,
                dir.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse as i32,
                &mut result,
            );
            //
            if dist > 60f32 {
                dist = 60f32
            }
            speed = 360f32 - (360f32 - 6f32 * dist);
            //
            if (*ms).moveflags & 4 == 0 && BotCheckBarrierJump(ms, dir.as_mut_ptr(), 50f32) == 0 {
                if speed > 5f32 {
                    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                    //end if
                }
            }
            result.movedir[0] = dir[0];
            result.movedir[1] = dir[1];
            result.movedir[2] = dir[2];
            //
            if (*ms).moveflags & 4 != 0 {
                result.flags |= 2
            }
            //this isn't a failure... just wait till the func_bobbing arrives
            result.type_0 = 2;
            result.flags |= 4;
            return result;
        }
        //get direction and distance to func_bob bottom center
        MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
        dir2[0] = bottomcenter[0] - (*ms).origin[0];
        dir2[1] = bottomcenter[1] - (*ms).origin[1];
        dir2[2] = bottomcenter[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            dir2[2] = 0f32
        }
        dist2 = crate::src::qcommon::q_math::VectorNormalize(dir2.as_mut_ptr());
        //if very close to the reachability start or
        //closer to the elevator center or
        //between reachability start and func_bobbing center
        if dist1 < 20f32
            || dist2 < dist1
            || dir1[0] * dir2[0] + dir1[1] * dir2[1] + dir1[2] * dir2[2] < 0f32
        {
            //end else
            dist = dist2; //end if
            dir[0] = dir2[0];
            dir[1] = dir2[1];
            dir[2] = dir2[2]
        } else {
            //closer to the reachability start
            dist = dist1;
            dir[0] = dir1[0];
            dir[1] = dir1[1];
            dir[2] = dir1[2]
        }
        //
        BotCheckBlocked(
            ms,
            dir.as_mut_ptr(),
            crate::src::qcommon::q_shared::qfalse as i32,
            &mut result,
        );
        //
        if dist > 60f32 {
            dist = 60f32
        }
        speed = 400f32 - (400f32 - 6f32 * dist);
        //
        if (*ms).moveflags & 4 == 0 && BotCheckBarrierJump(ms, dir.as_mut_ptr(), 50f32) == 0 {
            crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
            //end if
        }
        result.movedir[0] = dir[0];
        result.movedir[1] = dir[1];
        result.movedir[2] = dir[2];
        //
        if (*ms).moveflags & 4 != 0 {
            result.flags |= 2
        }
    }
    return result;
}
//end of the function BotTravel_FuncBobbing
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_FuncBobbing(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut bob_origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bob_start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bob_end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottomcenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    //
    BotFuncBobStartEnd(
        reach,
        bob_start.as_mut_ptr(),
        bob_end.as_mut_ptr(),
        bob_origin.as_mut_ptr(),
    );
    //
    dir[0] = bob_origin[0] - bob_end[0];
    dir[1] = bob_origin[1] - bob_end[1];
    dir[2] = bob_origin[2] - bob_end[2];
    dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    //if the func_bobbing is near the end
    if dist < 16f32 {
        //end else
        hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = (*reach).end[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            hordir[2] = 0f32
        }
        dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //
        if dist > 60f32 {
            dist = 60f32
        }
        speed = 360f32 - (360f32 - 6f32 * dist);
        //
        if speed > 5f32 {
            crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
        }
        result.movedir[0] = dir[0];
        result.movedir[1] = dir[1];
        result.movedir[2] = dir[2];
        //
        if (*ms).moveflags & 4 != 0 {
            result.flags |= 2
        }
    } else {
        MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
        hordir[0] = bottomcenter[0] - (*ms).origin[0];
        hordir[1] = bottomcenter[1] - (*ms).origin[1];
        hordir[2] = bottomcenter[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            hordir[2] = 0f32
        }
        dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //end if
        if dist > 5f32 {
            //
            //move to the center of the plat
            if dist > 100f32 {
                dist = 100f32
            }
            speed = 400f32 - (400f32 - 4f32 * dist);
            //
            crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
            result.movedir[0] = hordir[0];
            result.movedir[1] = hordir[1];
            result.movedir[2] = hordir[2]
        }
    }
    return result;
}
//end of the function BotFinishTravel_FuncBobbing
//===========================================================================
// 0  no valid grapple hook visible
// 1  the grapple hook is still flying
// 2  the grapple hooked into a wall
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GrappleState(
    mut ms: *mut bot_movestate_t,
    mut _reach: *mut crate::aasfile_h::aas_reachability_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    //if the grapple hook is pulling
    if (*ms).moveflags & 64 != 0 {
        return 2i32;
    }
    //check for a visible grapple missile entity
    //or visible grapple entity
    i = crate::src::botlib::be_aas_entity::AAS_NextEntity(0); //end for
    while i != 0 {
        if crate::src::botlib::be_aas_entity::AAS_EntityType(i) == (*entitytypemissile).value as i32
        {
            crate::src::botlib::be_aas_entity::AAS_EntityInfo(i, &mut entinfo);
            if entinfo.weapon == (*weapindex_grapple).value as i32 {
                return 1i32;
            }
            //end if
        }
        i = crate::src::botlib::be_aas_entity::AAS_NextEntity(i)
        //end if
    }
    //no valid grapple at all
    return 0;
}
//end of the function GrappleState
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetGrapple(mut ms: *mut bot_movestate_t) {
    let mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum((*ms).lastreachnum, &mut reach);
    //if not using the grapple hook reachability anymore
    if reach.traveltype & 0xffffff != 14 {
        if (*ms).moveflags & 128 != 0 || (*ms).grapplevisible_time != 0. {
            if (*offhandgrapple).value != 0. {
                crate::src::botlib::be_ea::EA_Command((*ms).client, (*cmd_grappleoff).string);
            }
            (*ms).moveflags &= !(128);
            (*ms).grapplevisible_time = 0f32
            //DEBUG_GRAPPLE
        }
        //end if
    };
    //end if
}
//end of the function BotResetGrapple
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_Grapple(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut viewdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut state: i32 = 0;
    let mut areanum: i32 = 0;
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    //DEBUG_GRAPPLE
    //
    if (*ms).moveflags & 256 != 0 {
        if (*offhandgrapple).value != 0. {
            crate::src::botlib::be_ea::EA_Command((*ms).client, (*cmd_grappleoff).string);
            //end if
        }
        (*ms).moveflags &= !(128);
        return result;
    }
    //
    if (*offhandgrapple).value as i32 == 0 {
        result.weapon = (*weapindex_grapple).value as i32; //end if
        result.flags |= 16
    }
    //
    if (*ms).moveflags & 128 != 0 {
        //end else
        state = GrappleState(ms, reach); //end if
                                         //DEBUG_GRAPPLE
                                         //
                                         //
        dir[0] = (*reach).end[0] - (*ms).origin[0];
        dir[1] = (*reach).end[1] - (*ms).origin[1];
        dir[2] = (*reach).end[2] - (*ms).origin[2];
        dir[2] = 0f32;
        dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        //if very close to the grapple end or the grappled is hooked and
        //the bot doesn't get any closer
        if state != 0 && dist < 48f32 {
            if (*ms).lastgrappledist - dist < 1f32 {
                //end else
                //DEBUG_GRAPPLE
                if (*offhandgrapple).value != 0. {
                    crate::src::botlib::be_ea::EA_Command((*ms).client, (*cmd_grappleoff).string);
                    //end the reachability
                }
                (*ms).moveflags &= !(128);
                (*ms).moveflags |= 256;
                (*ms).reachability_time = 0f32;
                return result;
            }
        //end if
        } else if state == 0 || state == 2 && dist > (*ms).lastgrappledist - 2f32 {
            if ((*ms).grapplevisible_time as f64)
                < crate::src::botlib::be_aas_main::AAS_Time() as f64 - 0.4
            {
                //if no valid grapple at all, or the grapple hooked and the bot
                //isn't moving anymore
                //DEBUG_GRAPPLE
                if (*offhandgrapple).value != 0. {
                    crate::src::botlib::be_ea::EA_Command((*ms).client, (*cmd_grappleoff).string);
                    //end the reachability
                }
                (*ms).moveflags &= !(128);
                (*ms).moveflags |= 256;
                (*ms).reachability_time = 0f32;
                return result;
            }
        //end if
        } else {
            (*ms).grapplevisible_time = crate::src::botlib::be_aas_main::AAS_Time()
        }
        //
        if (*offhandgrapple).value as i32 == 0 {
            crate::src::botlib::be_ea::EA_Attack((*ms).client); //end if
        }
        //remember the current grapple distance
        (*ms).lastgrappledist = dist
    } else {
        //DEBUG_GRAPPLE
        //
        (*ms).grapplevisible_time = crate::src::botlib::be_aas_main::AAS_Time();
        //
        dir[0] = (*reach).start[0] - (*ms).origin[0];
        dir[1] = (*reach).start[1] - (*ms).origin[1];
        dir[2] = (*reach).start[2] - (*ms).origin[2];
        if (*ms).moveflags & 4 == 0 {
            dir[2] = 0f32
        }
        org[0] = (*ms).origin[0] + (*ms).viewoffset[0];
        org[1] = (*ms).origin[1] + (*ms).viewoffset[1];
        org[2] = (*ms).origin[2] + (*ms).viewoffset[2];
        viewdir[0] = (*reach).end[0] - org[0];
        viewdir[1] = (*reach).end[1] - org[1];
        viewdir[2] = (*reach).end[2] - org[2];
        //
        dist = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        crate::src::qcommon::q_math::vectoangles(
            viewdir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            result.ideal_viewangles.as_mut_ptr(),
        );
        result.flags |= 1;
        //
        if dist < 5f32
            && crate::stdlib::fabs(AngleDiff(result.ideal_viewangles[0], (*ms).viewangles[0]) as f64)
                < 2f64
            && crate::stdlib::fabs(AngleDiff(result.ideal_viewangles[1], (*ms).viewangles[1]) as f64)
                < 2f64
        {
            //end else
            org[0] = (*ms).origin[0] + (*ms).viewoffset[0]; //end if
            org[1] = (*ms).origin[1] + (*ms).viewoffset[1];
            org[2] = (*ms).origin[2] + (*ms).viewoffset[2];
            trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
                org.as_mut_ptr(),
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                (*reach).end.as_mut_ptr(),
                (*ms).entitynum,
                1,
            );
            dir[0] = (*reach).end[0] - trace.endpos[0];
            dir[1] = (*reach).end[1] - trace.endpos[1];
            dir[2] = (*reach).end[2] - trace.endpos[2];
            //DEBUG_GRAPPLE
            //check if the grapple missile path is clear
            if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) > 16f32
            {
                result.failure = crate::src::qcommon::q_shared::qtrue as i32; //end if
                return result;
            }
            //activate the grapple
            if (*offhandgrapple).value != 0. {
                //end else
                crate::src::botlib::be_ea::EA_Command((*ms).client, (*cmd_grappleon).string);
            //end if
            } else {
                crate::src::botlib::be_ea::EA_Attack((*ms).client);
            }
            (*ms).moveflags |= 128;
            (*ms).lastgrappledist = 999999f32
        } else {
            if dist < 70f32 {
                speed = 300f32 - (300f32 - 4f32 * dist)
            } else {
                speed = 400f32
            }
            //
            BotCheckBlocked(
                ms,
                dir.as_mut_ptr(),
                crate::src::qcommon::q_shared::qtrue as i32,
                &mut result,
            );
            //elemantary action move in direction
            crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
            result.movedir[0] = dir[0];
            result.movedir[1] = dir[1];
            result.movedir[2] = dir[2]
        }
        //if in another area before actually grappling
        areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum((*ms).origin.as_mut_ptr());
        if areanum != 0 && areanum != (*ms).reachareanum {
            (*ms).reachability_time = 0f32
        }
    }
    return result;
}
//end of the function BotTravel_Grapple
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:			-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_RocketJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //botimport.Print(PRT_MESSAGE, "BotTravel_RocketJump: bah\n");
    //
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    //
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //look in the movement direction
    crate::src::qcommon::q_math::vectoangles(
        hordir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    //look straight down
    result.ideal_viewangles[0] = 90f32;
    //
    if dist < 5f32
        && crate::stdlib::fabs(AngleDiff(result.ideal_viewangles[0], (*ms).viewangles[0]) as f64)
            < 5f64
        && crate::stdlib::fabs(AngleDiff(result.ideal_viewangles[1], (*ms).viewangles[1]) as f64)
            < 5f64
    {
        //end else
        hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //botimport.Print(PRT_MESSAGE, "between jump start and run start point\n");
        //elemantary action jump
        crate::src::botlib::be_ea::EA_Jump((*ms).client);
        crate::src::botlib::be_ea::EA_Attack((*ms).client);
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 800f32);
        //
        (*ms).jumpreach = (*ms).lastreachnum
    } else {
        if dist > 80f32 {
            dist = 80f32
        }
        speed = 400f32 - (400f32 - 5f32 * dist);
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    //look in the movement direction
    crate::src::qcommon::q_math::vectoangles(
        hordir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    //look straight down
    result.ideal_viewangles[0] = 90f32;
    //set the view angles directly
    crate::src::botlib::be_ea::EA_View((*ms).client, result.ideal_viewangles.as_mut_ptr());
    //view is important for the movement
    result.flags |= 8;
    //select the rocket launcher
    crate::src::botlib::be_ea::EA_SelectWeapon(
        (*ms).client,
        (*weapindex_rocketlauncher).value as i32,
    );
    //weapon is used for movement
    result.weapon = (*weapindex_rocketlauncher).value as i32;
    result.flags |= 16;
    //
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_RocketJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_BFGJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //botimport.Print(PRT_MESSAGE, "BotTravel_BFGJump: bah\n");
    //
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    //
    dist = crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //
    if dist < 5f32
        && crate::stdlib::fabs(AngleDiff(result.ideal_viewangles[0], (*ms).viewangles[0]) as f64)
            < 5f64
        && crate::stdlib::fabs(AngleDiff(result.ideal_viewangles[1], (*ms).viewangles[1]) as f64)
            < 5f64
    {
        //end else
        hordir[0] = (*reach).end[0] - (*ms).origin[0]; //end if
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //botimport.Print(PRT_MESSAGE, "between jump start and run start point\n");
        //elemantary action jump
        crate::src::botlib::be_ea::EA_Jump((*ms).client);
        crate::src::botlib::be_ea::EA_Attack((*ms).client);
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 800f32);
        //
        (*ms).jumpreach = (*ms).lastreachnum
    } else {
        if dist > 80f32 {
            dist = 80f32
        }
        speed = 400f32 - (400f32 - 5f32 * dist);
        crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    //look in the movement direction
    crate::src::qcommon::q_math::vectoangles(
        hordir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        result.ideal_viewangles.as_mut_ptr(),
    );
    //look straight down
    result.ideal_viewangles[0] = 90f32;
    //set the view angles directly
    crate::src::botlib::be_ea::EA_View((*ms).client, result.ideal_viewangles.as_mut_ptr());
    //view is important for the movement
    result.flags |= 8;
    //select the rocket launcher
    crate::src::botlib::be_ea::EA_SelectWeapon((*ms).client, (*weapindex_bfg10k).value as i32);
    //weapon is used for movement
    result.weapon = (*weapindex_bfg10k).value as i32;
    result.flags |= 16;
    //
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_BFGJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_WeaponJump(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut speed: f32 = 0.;
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //if not jumped yet
    if (*ms).jumpreach == 0 {
        return result;
    }
    /*
    //go straight to the reachability end
    hordir[0] = reach->end[0] - ms->origin[0];
    hordir[1] = reach->end[1] - ms->origin[1];
    hordir[2] = 0;
    VectorNormalize(hordir);
    //always use max speed when traveling through the air
    EA_Move(ms->client, hordir, 800);
    VectorCopy(hordir, result.movedir);
    */
    //
    if BotAirControl(
        (*ms).origin.as_mut_ptr(),
        (*ms).velocity.as_mut_ptr(),
        (*reach).end.as_mut_ptr(),
        hordir.as_mut_ptr(),
        &mut speed,
    ) == 0
    {
        //end if
        //go straight to the reachability end
        hordir[0] = (*reach).end[0] - (*ms).origin[0];
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = (*reach).end[2] - (*ms).origin[2];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        speed = 400f32
    }
    //
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotFinishTravel_WeaponJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotTravel_JumpPad(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    //first walk straight to the reachability start
    hordir[0] = (*reach).start[0] - (*ms).origin[0];
    hordir[1] = (*reach).start[1] - (*ms).origin[1];
    hordir[2] = 0f32;
    //
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //elemantary action move in direction
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), 400f32);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotTravel_JumpPad
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFinishTravel_JumpPad(
    mut ms: *mut bot_movestate_t,
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut speed: f32 = 0.; //end if
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    if BotAirControl(
        (*ms).origin.as_mut_ptr(),
        (*ms).velocity.as_mut_ptr(),
        (*reach).end.as_mut_ptr(),
        hordir.as_mut_ptr(),
        &mut speed,
    ) == 0
    {
        hordir[0] = (*reach).end[0] - (*ms).origin[0];
        hordir[1] = (*reach).end[1] - (*ms).origin[1];
        hordir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        speed = 400f32
    }
    BotCheckBlocked(
        ms,
        hordir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //elemantary action move in direction
    crate::src::botlib::be_ea::EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0] = hordir[0];
    result.movedir[1] = hordir[1];
    result.movedir[2] = hordir[2];
    //
    return result;
}
//end of the function BotFinishTravel_JumpPad
//===========================================================================
// time before the reachability times out
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotReachabilityTime(
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) -> i32 {
    match (*reach).traveltype & 0xffffff {
        2 => {
            return 5i32;
            //end case
        }
        3 => return 5,
        4 => return 5,
        6 => return 6,
        7 => return 5,
        5 => return 5,
        8 => return 5,
        9 => return 5,
        10 => return 5,
        11 => return 10,
        14 => return 8,
        12 => return 6,
        13 => return 6,
        18 => return 10,
        19 => return 10,
        _ => {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3,
                b"travel type %d not implemented yet\n\x00" as *const u8 as *mut i8,
                (*reach).traveltype,
            );
            return 8i32;
        }
    };
    //end switch
}
//end of the function BotReachabilityTime
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotMoveInGoalArea(
    mut ms: *mut bot_movestate_t,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
) -> crate::src::botlib::be_ai_move::bot_moveresult_t {
    let mut result: crate::src::botlib::be_ai_move::bot_moveresult_t = {
        let mut init = crate::src::botlib::be_ai_move::bot_moveresult_s {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0f32, 0f32, 0f32],
            ideal_viewangles: [0f32, 0f32, 0f32],
        };
        init
    };
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut speed: f32 = 0.;
    //DEBUG
    //walk straight to the goal origin
    dir[0] = (*goal).origin[0] - (*ms).origin[0]; //endif
    dir[1] = (*goal).origin[1] - (*ms).origin[1]; //end if
    if (*ms).moveflags & 4 != 0 {
        dir[2] = (*goal).origin[2] - (*ms).origin[2];
        result.traveltype = 8
    } else {
        dir[2] = 0f32;
        result.traveltype = 2
    }
    //
    dist = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    if dist > 100f32 {
        dist = 100f32
    }
    speed = 400f32 - (400f32 - 4f32 * dist);
    if speed < 10f32 {
        speed = 0f32
    }
    //
    BotCheckBlocked(
        ms,
        dir.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue as i32,
        &mut result,
    );
    //elemantary action move in direction
    crate::src::botlib::be_ea::EA_Move((*ms).client, dir.as_mut_ptr(), speed);
    result.movedir[0] = dir[0];
    result.movedir[1] = dir[1];
    result.movedir[2] = dir[2];
    //
    if (*ms).moveflags & 4 != 0 {
        crate::src::qcommon::q_math::vectoangles(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            result.ideal_viewangles.as_mut_ptr(),
        ); //end if
        result.flags |= 2
    }
    //if (!debugline) debugline = botimport.DebugLineCreate();
    //botimport.DebugLineShow(debugline, ms->origin, goal->origin, LINECOLOR_BLUE);
    //
    (*ms).lastreachnum = 0;
    (*ms).lastareanum = 0;
    (*ms).lastgoalareanum = (*goal).areanum;
    (*ms).lastorigin[0] = (*ms).origin[0];
    (*ms).lastorigin[1] = (*ms).origin[1];
    (*ms).lastorigin[2] = (*ms).origin[2];
    //
    return result;
}
//moves the bot to the given goal
//end of the function BotMoveInGoalArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotMoveToGoal(
    mut result: *mut crate::src::botlib::be_ai_move::bot_moveresult_t,
    mut movestate: i32,
    mut goal: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
    mut travelflags: i32,
) {
    let mut reachnum: i32 = 0;
    let mut lastreachnum: i32 = 0;
    let mut foundjumppad: i32 = 0;
    let mut ent: i32 = 0;
    let mut resultflags: i32 = 0;
    let mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    let mut lastreach: crate::aasfile_h::aas_reachability_t =
        crate::aasfile_h::aas_reachability_t {
            areanum: 0,
            facenum: 0,
            edgenum: 0,
            start: [0.; 3],
            end: [0.; 3],
            traveltype: 0,
            traveltime: 0,
        };
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    //vec3_t mins, maxs, up = {0, 0, 1};
    //bsp_trace_t trace;
    //static int debugline;
    (*result).failure = crate::src::qcommon::q_shared::qfalse as i32;
    (*result).type_0 = 0;
    (*result).blocked = crate::src::qcommon::q_shared::qfalse as i32;
    (*result).blockentity = 0;
    (*result).traveltype = 0;
    (*result).flags = 0;
    //
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return;
    }
    //reset the grapple before testing if the bot has a valid goal
    //because the bot could lose all its goals when stuck to a wall
    BotResetGrapple(ms);
    //
    if goal.is_null() {
        //end if
        //DEBUG
        (*result).failure = crate::src::qcommon::q_shared::qtrue as i32;
        return;
    }
    //botimport.Print(PRT_MESSAGE, "numavoidreach = %d\n", ms->numavoidreach);
    //remove some of the move flags
    (*ms).moveflags &= !(4 | 8);
    //set some of the move flags
    //NOTE: the MFL_ONGROUND flag is also set in the higher AI
    if crate::src::botlib::be_aas_move::AAS_OnGround(
        (*ms).origin.as_mut_ptr(),
        (*ms).presencetype,
        (*ms).entitynum,
    ) != 0
    {
        (*ms).moveflags |= 2
    }
    //
    if (*ms).moveflags & 2 != 0 {
        let mut modeltype: i32 = 0; //end if
        let mut modelnum: i32 = 0;
        ent = BotOnTopOfEntity(ms);
        if ent != -(1) {
            modelnum = crate::src::botlib::be_aas_entity::AAS_EntityModelindex(ent);
            if modelnum >= 0 && modelnum < 256 {
                modeltype = modeltypes[modelnum as usize];
                if modeltype == 1 {
                    //end if
                    //end if
                    //end else
                    crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
                        (*ms).lastreachnum,
                        &mut reach,
                    ); //end if
                       //if the bot is Not using the elevator
                    if reach.traveltype & 0xffffff != 11 || reach.facenum & 0xffff != modelnum {
                        reachnum = crate::src::botlib::be_aas_route::AAS_NextModelReachability(
                            0, modelnum,
                        );
                        if reachnum != 0 {
                            //end if
                            //end else
                            crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
                                reachnum, &mut reach,
                            ); //end if
                            (*ms).lastreachnum = reachnum;
                            (*ms).reachability_time = crate::src::botlib::be_aas_main::AAS_Time()
                                + BotReachabilityTime(&mut reach) as f32
                        } else {
                            //botimport.Print(PRT_MESSAGE, "client %d: accidentally ended up on func_plat\n", ms->client);
                            if crate::src::botlib::be_interface::botDeveloper != 0 {
                                crate::src::botlib::be_interface::botimport
                                    .Print
                                    .expect("non-null function pointer")(
                                    1i32,
                                    b"client %d: on func_plat without reachability\n\x00"
                                        as *const u8 as *mut i8,
                                    (*ms).client,
                                ); //end if
                            } //end if
                            (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
                            (*result).blockentity = ent;
                            (*result).flags |= 32;
                            return;
                        }
                    }
                    (*result).flags |= 128
                } else if modeltype == 2 {
                    crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
                        (*ms).lastreachnum,
                        &mut reach,
                    );
                    //if the bot is Not using the func bobbing
                    if reach.traveltype & 0xffffff != 19 || reach.facenum & 0xffff != modelnum {
                        reachnum = crate::src::botlib::be_aas_route::AAS_NextModelReachability(
                            0, modelnum,
                        );
                        if reachnum != 0 {
                            //end if
                            //end else
                            crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
                                reachnum, &mut reach,
                            ); //end if
                            (*ms).lastreachnum = reachnum;
                            (*ms).reachability_time = crate::src::botlib::be_aas_main::AAS_Time()
                                + BotReachabilityTime(&mut reach) as f32
                        } else {
                            //botimport.Print(PRT_MESSAGE, "client %d: accidentally ended up on func_bobbing\n", ms->client);
                            if crate::src::botlib::be_interface::botDeveloper != 0 {
                                crate::src::botlib::be_interface::botimport
                                    .Print
                                    .expect("non-null function pointer")(
                                    1i32,
                                    b"client %d: on func_bobbing without reachability\n\x00"
                                        as *const u8 as *mut i8,
                                    (*ms).client,
                                ); //end if
                            }
                            (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
                            (*result).blockentity = ent;
                            (*result).flags |= 32;
                            return;
                        }
                    }
                    (*result).flags |= 64
                } else if modeltype == 4 || modeltype == 3 {
                    // check if ontop of a door bridge ?
                    (*ms).areanum = BotFuzzyPointReachabilityArea((*ms).origin.as_mut_ptr());
                    //end if
                    if crate::src::botlib::be_aas_reach::AAS_AreaReachability((*ms).areanum) == 0 {
                        (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
                        (*result).blockentity = ent;
                        (*result).flags |= 32;
                        return;
                    }
                } else {
                    (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
                    (*result).blockentity = ent;
                    (*result).flags |= 32;
                    return;
                }
            }
        }
    }
    // if not in a reachability area
    //if swimming
    if crate::src::botlib::be_aas_move::AAS_Swimming((*ms).origin.as_mut_ptr()) != 0 {
        (*ms).moveflags |= 4
    }
    //if against a ladder
    if crate::src::botlib::be_aas_move::AAS_AgainstLadder((*ms).origin.as_mut_ptr()) != 0 {
        (*ms).moveflags |= 8
    }
    //if the bot is on the ground, swimming or against a ladder
    if (*ms).moveflags & (2 | 4 | 8) != 0 {
        //end else
        //botimport.Print(PRT_MESSAGE, "%s: onground, swimming or against ladder\n", ClientName(ms->entitynum-1));
        //
        crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
            (*ms).lastreachnum,
            &mut lastreach,
        );
        //end else
        //DEBUG
        (*ms).areanum = BotFuzzyPointReachabilityArea((*ms).origin.as_mut_ptr());
        if (*ms).areanum == 0 {
            (*result).failure = crate::src::qcommon::q_shared::qtrue as i32;
            (*result).blocked = crate::src::qcommon::q_shared::qtrue as i32;
            (*result).blockentity = 0;
            (*result).type_0 = 8;
            return;
        }
        if (*ms).areanum == (*goal).areanum {
            *result = BotMoveInGoalArea(ms, goal);
            return;
        }
        reachnum = (*ms).lastreachnum;
        if reachnum != 0 {
            crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach);
            //reachability area the bot is in
            //ms->areanum = BotReachabilityArea(ms->origin, ((lastreach.traveltype & TRAVELTYPE_MASK) != TRAVEL_ELEVATOR));
            //
            //end if
            //if the bot is in the goal area
            //end if
            //assume we can use the reachability from the last frame
            //if there is a last reachability
            //end if
            //end else
            if crate::src::botlib::be_aas_route::AAS_TravelFlagForType(reach.traveltype)
                & travelflags
                == 0
            {
                //check if the reachability is still valid
                reachnum = 0
            } else if reach.traveltype & 0xffffff == 14 {
                if (*ms).reachability_time < crate::src::botlib::be_aas_main::AAS_Time()
                    || (*ms).moveflags & 256 != 0
                {
                    reachnum = 0
                } //end if
                  //special grapple hook case
                  //end if
            } else if reach.traveltype & 0xffffff == 11 || reach.traveltype & 0xffffff == 19 {
                //special elevator case
                if (*result).flags & 128 != 0 || (*result).flags & 64 != 0 {
                    (*ms).reachability_time = crate::src::botlib::be_aas_main::AAS_Time() + 5f32
                } //end if
                  //end if
                if (*ms).areanum == reach.areanum
                    || (*ms).reachability_time < crate::src::botlib::be_aas_main::AAS_Time()
                {
                    reachnum = 0
                }
            } else if (*ms).lastgoalareanum != (*goal).areanum
                || (*ms).reachability_time < crate::src::botlib::be_aas_main::AAS_Time()
                || (*ms).lastareanum != (*ms).areanum
            {
                reachnum = 0
                //if the bot was going for an elevator and reached the reachability area
                //DEBUG
                //if the goal area changed or the reachability timed out
                //or the area changed
                //botimport.Print(PRT_MESSAGE, "area change or timeout\n");
            }
        }
        resultflags = 0;
        if reachnum == 0 {
            //end else if
            //if the bot needs a new reachability
            //end else
            //if the area has no reachability links
            (crate::src::botlib::be_aas_reach::AAS_AreaReachability((*ms).areanum)) == 0; //end if
                                                                                          //end if
                                                                                          //DEBUG
            reachnum = BotGetReachabilityToGoal(
                (*ms).origin.as_mut_ptr(),
                (*ms).areanum,
                (*ms).lastgoalareanum,
                (*ms).lastareanum,
                (*ms).avoidreach.as_mut_ptr(),
                (*ms).avoidreachtimes.as_mut_ptr(),
                (*ms).avoidreachtries.as_mut_ptr(),
                goal,
                travelflags,
                (*ms).avoidspots.as_mut_ptr(),
                (*ms).numavoidspots,
                &mut resultflags,
            );
            (*ms).reachareanum = (*ms).areanum;
            (*ms).jumpreach = 0;
            (*ms).moveflags &= !(256);
            if reachnum != 0 {
                crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach);
                //get a new reachability leading towards the goal
                //the area number the reachability starts in
                //reset some state variables
                //for TRAVEL_JUMP
                //for TRAVEL_GRAPPLEHOOK
                //if there is a reachability to the goal
                //AVOIDREACH
                (*ms).reachability_time = crate::src::botlib::be_aas_main::AAS_Time()
                    + BotReachabilityTime(&mut reach) as f32;
                BotAddToAvoidReach(ms, reachnum, 6f32);
            }
        }
        (*ms).lastreachnum = reachnum;
        (*ms).lastgoalareanum = (*goal).areanum;
        (*ms).lastareanum = (*ms).areanum;
        if reachnum != 0 {
            //set a timeout for this reachability
            //
            //add the reachability to the reachabilities to avoid for a while
            //
            //if the bot has a reachability
            crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(reachnum, &mut reach); //end if
            (*result).traveltype = reach.traveltype;
            //get the reachability from the number
            //
            //DEBUG_AI_MOVE
            //
            //DEBUG
            match reach.traveltype & 0xffffff {
                2 => {
                    *result = BotTravel_Walk(ms, &mut reach) //end switch
                                                             //end case
                }
                3 => *result = BotTravel_Crouch(ms, &mut reach),
                4 => *result = BotTravel_BarrierJump(ms, &mut reach),
                6 => *result = BotTravel_Ladder(ms, &mut reach),
                7 => *result = BotTravel_WalkOffLedge(ms, &mut reach),
                5 => *result = BotTravel_Jump(ms, &mut reach),
                8 => *result = BotTravel_Swim(ms, &mut reach),
                9 => *result = BotTravel_WaterJump(ms, &mut reach),
                10 => *result = BotTravel_Teleport(ms, &mut reach),
                11 => *result = BotTravel_Elevator(ms, &mut reach),
                14 => *result = BotTravel_Grapple(ms, &mut reach),
                12 => *result = BotTravel_RocketJump(ms, &mut reach),
                13 => *result = BotTravel_BFGJump(ms, &mut reach),
                18 => *result = BotTravel_JumpPad(ms, &mut reach),
                19 => *result = BotTravel_FuncBobbing(ms, &mut reach),
                _ => {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        4i32,
                        b"travel type %d not implemented yet\n\x00" as *const u8 as *mut i8,
                        reach.traveltype & 0xffffffi32,
                    );
                }
            }
            (*result).traveltype = reach.traveltype;
            (*result).flags |= resultflags
        } else {
            (*result).failure = crate::src::qcommon::q_shared::qtrue as i32;
            (*result).flags |= resultflags;
            crate::stdlib::memset(
                &mut reach as *mut crate::aasfile_h::aas_reachability_t as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>(),
            );
        }
    } else {
        let mut i: i32 = 0;
        let mut numareas: i32 = 0;
        let mut areas: [i32; 16] = [0; 16];
        let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        //end if
        foundjumppad = crate::src::qcommon::q_shared::qfalse as i32;
        end[0] = (*ms).origin[0] + (*ms).velocity[0] * (-2f32 * (*ms).thinktime);
        end[1] = (*ms).origin[1] + (*ms).velocity[1] * (-2f32 * (*ms).thinktime);
        end[2] = (*ms).origin[2] + (*ms).velocity[2] * (-2f32 * (*ms).thinktime);
        numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
            (*ms).origin.as_mut_ptr(),
            end.as_mut_ptr(),
            areas.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec3_t,
            16,
        );
        i = numareas - 1;
        while i >= 0 {
            if crate::src::botlib::be_aas_reach::AAS_AreaJumpPad(areas[i as usize]) != 0 {
                //special handling of jump pads when the bot uses a jump pad without knowing it
                //end for
                //botimport.Print(PRT_MESSAGE, "client %d used a jumppad without knowing, area %d\n", ms->client, areas[i]);
                foundjumppad = crate::src::qcommon::q_shared::qtrue as i32;
                lastreachnum = BotGetReachabilityToGoal(
                    end.as_mut_ptr(),
                    areas[i as usize],
                    (*ms).lastgoalareanum,
                    (*ms).lastareanum,
                    (*ms).avoidreach.as_mut_ptr(),
                    (*ms).avoidreachtimes.as_mut_ptr(),
                    (*ms).avoidreachtries.as_mut_ptr(),
                    goal,
                    0x40000,
                    (*ms).avoidspots.as_mut_ptr(),
                    (*ms).numavoidspots,
                    0 as *mut i32,
                );
                if lastreachnum != 0 {
                    //end else
                    (*ms).lastreachnum = lastreachnum; //end if
                    (*ms).lastareanum = areas[i as usize]; //end for
                    break;
                } else {
                    lastreachnum = crate::src::botlib::be_aas_route::AAS_NextAreaReachability(
                        areas[i as usize],
                        0,
                    );
                    while lastreachnum != 0 {
                        //get the reachability from the number
                        crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
                            lastreachnum,
                            &mut reach,
                        );
                        if reach.traveltype & 0xffffff == 18 {
                            (*ms).lastreachnum = lastreachnum;
                            (*ms).lastareanum = areas[i as usize]
                            //end if
                            //botimport.Print(PRT_MESSAGE, "found jumppad reachability hard!!\n");
                        }
                        lastreachnum = crate::src::botlib::be_aas_route::AAS_NextAreaReachability(
                            areas[i as usize],
                            lastreachnum,
                        )
                    }
                    if lastreachnum != 0 {
                        break;
                    }
                }
            }
            i -= 1
            //end if
        } //end if
        if crate::src::botlib::be_interface::botDeveloper != 0 {
            //if a jumppad is found with the trace but no reachability is found
            if foundjumppad != 0 && (*ms).lastreachnum == 0 {
                crate::src::botlib::be_interface::botimport
                    .Print
                    .expect("non-null function pointer")(
                    1i32,
                    b"client %d didn\'t find jumppad reachability\n\x00" as *const u8 as *mut i8,
                    (*ms).client,
                );
            }
            //end if
        }
        if (*ms).lastreachnum != 0 {
            //
            //botimport.Print(PRT_MESSAGE, "%s: NOT onground, swimming or against ladder\n", ClientName(ms->entitynum-1));
            crate::src::botlib::be_aas_route::AAS_ReachabilityFromNum(
                (*ms).lastreachnum,
                &mut reach,
            );
            (*result).traveltype = reach.traveltype;
            //DEBUG
            match reach.traveltype & 0xffffff {
                2 => {
                    *result = BotTravel_Walk(ms, &mut reach)
                    //DEBUG
                    //
                    //end switch
                    //end case
                }
                3 => {}
                4 => *result = BotFinishTravel_BarrierJump(ms, &mut reach),
                6 => *result = BotTravel_Ladder(ms, &mut reach),
                7 => *result = BotFinishTravel_WalkOffLedge(ms, &mut reach),
                5 => *result = BotFinishTravel_Jump(ms, &mut reach),
                8 => *result = BotTravel_Swim(ms, &mut reach),
                9 => *result = BotFinishTravel_WaterJump(ms, &mut reach),
                10 => {}
                11 => *result = BotFinishTravel_Elevator(ms, &mut reach),
                14 => *result = BotTravel_Grapple(ms, &mut reach),
                12 | 13 => *result = BotFinishTravel_WeaponJump(ms, &mut reach),
                18 => *result = BotFinishTravel_JumpPad(ms, &mut reach),
                19 => *result = BotFinishTravel_FuncBobbing(ms, &mut reach),
                _ => {
                    crate::src::botlib::be_interface::botimport
                        .Print
                        .expect("non-null function pointer")(
                        4i32,
                        b"(last) travel type %d not implemented yet\n\x00" as *const u8 as *mut i8,
                        reach.traveltype & 0xffffffi32,
                    );
                }
            }
            (*result).traveltype = reach.traveltype
        }
    }
    //FIXME: is it right to do this here?
    if (*result).blocked != 0 {
        (*ms).reachability_time -= 10f32 * (*ms).thinktime
    }
    //copy the last origin
    (*ms).lastorigin[0] = (*ms).origin[0];
    (*ms).lastorigin[1] = (*ms).origin[1];
    (*ms).lastorigin[2] = (*ms).origin[2];
}
//reset avoid reachability
//end of the function BotMoveToGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetAvoidReach(mut movestate: i32) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return;
    }
    crate::stdlib::memset(
        (*ms).avoidreach.as_mut_ptr() as *mut libc::c_void,
        0,
        (1usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    crate::stdlib::memset(
        (*ms).avoidreachtimes.as_mut_ptr() as *mut libc::c_void,
        0,
        (1usize).wrapping_mul(::std::mem::size_of::<f32>()),
    );
    crate::stdlib::memset(
        (*ms).avoidreachtries.as_mut_ptr() as *mut libc::c_void,
        0,
        (1usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
}
//resets the last avoid reachability
//end of the function BotResetAvoidReach
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetLastAvoidReach(mut movestate: i32) {
    let mut _i: i32 = 0; //end for
    let mut latest: i32 = 0;
    let mut latesttime: f32 = 0.;
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return;
    }
    latesttime = 0f32;
    latest = 0;

    for i in 0..1 {
        if (*ms).avoidreachtimes[i as usize] > latesttime {
            latesttime = (*ms).avoidreachtimes[i as usize];
            latest = i
        }
    }
    if latesttime != 0. {
        (*ms).avoidreachtimes[latest as usize] = 0f32;
        if (*ms).avoidreachtries[latest as usize] > 0 {
            (*ms).avoidreachtries[latest as usize] -= 1
        }
    };
    //end if
}
//resets the whole move state
//end of the function BotResetLastAvoidReach
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotResetMoveState(mut movestate: i32) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() {
        return;
    }
    crate::stdlib::memset(
        ms as *mut libc::c_void,
        0,
        ::std::mem::size_of::<bot_movestate_t>(),
    );
}
//setup movement AI
//end of the function BotResetMoveState
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotSetupMoveAI() -> i32 {
    BotSetBrushModelTypes();
    sv_maxstep = crate::src::botlib::l_libvar::LibVar(
        b"sv_step\x00" as *const u8 as *const i8,
        b"18\x00" as *const u8 as *const i8,
    );
    sv_maxbarrier = crate::src::botlib::l_libvar::LibVar(
        b"sv_maxbarrier\x00" as *const u8 as *const i8,
        b"32\x00" as *const u8 as *const i8,
    );
    sv_gravity = crate::src::botlib::l_libvar::LibVar(
        b"sv_gravity\x00" as *const u8 as *const i8,
        b"800\x00" as *const u8 as *const i8,
    );
    weapindex_rocketlauncher = crate::src::botlib::l_libvar::LibVar(
        b"weapindex_rocketlauncher\x00" as *const u8 as *const i8,
        b"5\x00" as *const u8 as *const i8,
    );
    weapindex_bfg10k = crate::src::botlib::l_libvar::LibVar(
        b"weapindex_bfg10k\x00" as *const u8 as *const i8,
        b"9\x00" as *const u8 as *const i8,
    );
    weapindex_grapple = crate::src::botlib::l_libvar::LibVar(
        b"weapindex_grapple\x00" as *const u8 as *const i8,
        b"10\x00" as *const u8 as *const i8,
    );
    entitytypemissile = crate::src::botlib::l_libvar::LibVar(
        b"entitytypemissile\x00" as *const u8 as *const i8,
        b"3\x00" as *const u8 as *const i8,
    );
    offhandgrapple = crate::src::botlib::l_libvar::LibVar(
        b"offhandgrapple\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
    );
    cmd_grappleon = crate::src::botlib::l_libvar::LibVar(
        b"cmd_grappleon\x00" as *const u8 as *const i8,
        b"grappleon\x00" as *const u8 as *const i8,
    );
    cmd_grappleoff = crate::src::botlib::l_libvar::LibVar(
        b"cmd_grappleoff\x00" as *const u8 as *const i8,
        b"grappleoff\x00" as *const u8 as *const i8,
    );
    return 0;
}
//shutdown movement AI
//end of the function BotSetupMoveAI
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownMoveAI() {
    let mut i: i32 = 0;
    i = 1;
    while i <= 64 {
        if !botmovestates[i as usize].is_null() {
            crate::src::botlib::l_memory::FreeMemory(
                botmovestates[i as usize] as *mut libc::c_void,
            );
            botmovestates[i as usize] = 0 as *mut bot_movestate_t
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function BotShutdownMoveAI
