use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorCompare(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> i32 {
        if *v1.offset(0) != *v2.offset(0)
            || *v1.offset(1) != *v2.offset(1)
            || *v1.offset(2) != *v2.offset(2)
        {
            return 0i32;
        }
        return 1;
    }
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

    // __Q_SHARED_H
}

pub use crate::aasfile_h::aas_area_s;
pub use crate::aasfile_h::aas_area_t;
pub use crate::aasfile_h::aas_areasettings_s;
pub use crate::aasfile_h::aas_areasettings_t;
pub use crate::aasfile_h::aas_bbox_s;
pub use crate::aasfile_h::aas_bbox_t;
pub use crate::aasfile_h::aas_cluster_s;
pub use crate::aasfile_h::aas_cluster_t;
pub use crate::aasfile_h::aas_edge_s;
pub use crate::aasfile_h::aas_edge_t;
pub use crate::aasfile_h::aas_edgeindex_t;
pub use crate::aasfile_h::aas_face_s;
pub use crate::aasfile_h::aas_face_t;
pub use crate::aasfile_h::aas_faceindex_t;
pub use crate::aasfile_h::aas_node_s;
pub use crate::aasfile_h::aas_node_t;
pub use crate::aasfile_h::aas_plane_s;
pub use crate::aasfile_h::aas_plane_t;
pub use crate::aasfile_h::aas_portal_s;
pub use crate::aasfile_h::aas_portal_t;
pub use crate::aasfile_h::aas_portalindex_t;
pub use crate::aasfile_h::aas_reachability_s;
pub use crate::aasfile_h::aas_reachability_t;
pub use crate::aasfile_h::aas_vertex_t;
pub use crate::be_aas_def_h::aas_entity_s;
pub use crate::be_aas_def_h::aas_entity_t;
pub use crate::be_aas_def_h::aas_link_s;
pub use crate::be_aas_def_h::aas_link_t;
pub use crate::be_aas_def_h::aas_reachabilityareas_s;
pub use crate::be_aas_def_h::aas_reachabilityareas_t;
pub use crate::be_aas_def_h::aas_reversedlink_s;
pub use crate::be_aas_def_h::aas_reversedlink_t;
pub use crate::be_aas_def_h::aas_reversedreachability_s;
pub use crate::be_aas_def_h::aas_reversedreachability_t;
pub use crate::be_aas_def_h::aas_routingcache_s;
pub use crate::be_aas_def_h::aas_routingcache_t;
pub use crate::be_aas_def_h::aas_routingupdate_s;
pub use crate::be_aas_def_h::aas_routingupdate_t;
pub use crate::be_aas_def_h::aas_s;
pub use crate::be_aas_def_h::aas_settings_s;
pub use crate::be_aas_def_h::aas_settings_t;
pub use crate::be_aas_def_h::aas_t;
pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
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
pub use crate::src::botlib::be_aas_move::q_shared_h::VectorCompare;
pub use crate::src::botlib::be_aas_move::q_shared_h::VectorLength;

pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
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
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

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
    /* ****************************************************************************
     * name:		be_aas_move.c
     *
     * desc:		AAS
     *
     * $Archive: /MissionPack/code/botlib/be_aas_move.c $
     *
     *****************************************************************************/
    #[no_mangle]
    pub static mut botimport: crate::botlib_h::botlib_import_t;
}
#[no_mangle]

pub static mut aassettings: crate::be_aas_def_h::aas_settings_t =
    crate::be_aas_def_h::aas_settings_t {
        phys_gravitydirection: [0.; 3],
        phys_friction: 0.,
        phys_stopspeed: 0.,
        phys_gravity: 0.,
        phys_waterfriction: 0.,
        phys_watergravity: 0.,
        phys_maxvelocity: 0.,
        phys_maxwalkvelocity: 0.,
        phys_maxcrouchvelocity: 0.,
        phys_maxswimvelocity: 0.,
        phys_walkaccelerate: 0.,
        phys_airaccelerate: 0.,
        phys_swimaccelerate: 0.,
        phys_maxstep: 0.,
        phys_maxsteepness: 0.,
        phys_maxwaterjump: 0.,
        phys_maxbarrier: 0.,
        phys_jumpvel: 0.,
        phys_falldelta5: 0.,
        phys_falldelta10: 0.,
        rs_waterjump: 0.,
        rs_teleport: 0.,
        rs_barrierjump: 0.,
        rs_startcrouch: 0.,
        rs_startgrapple: 0.,
        rs_startwalkoffledge: 0.,
        rs_startjump: 0.,
        rs_rocketjump: 0.,
        rs_bfgjump: 0.,
        rs_jumppad: 0.,
        rs_aircontrolledjumppad: 0.,
        rs_funcbob: 0.,
        rs_startelevator: 0.,
        rs_falldamage5: 0.,
        rs_falldamage10: 0.,
        rs_maxfallheight: 0.,
        rs_maxjumpfallheight: 0.,
    };
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
 * name:		be_aas_move.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_move.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_move.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_move.h $
 *
 *****************************************************************************/
//AASINTERN
//AASINTERN
//movement prediction
//movement prediction
//predict movement until bounding box is hit
//predict movement until bounding box is hit
//returns true if on the ground at the given origin
//returns true if on the ground at the given origin
//returns true if swimming at the given origin
//returns true if swimming at the given origin
//returns the jump reachability run start point
//returns the jump reachability run start point
//returns true if against a ladder at the given origin
//returns true if against a ladder at the given origin
//rocket jump Z velocity when rocket-jumping at origin
//rocket jump Z velocity when rocket-jumping at origin
//bfg jump Z velocity when bfg-jumping at origin
//bfg jump Z velocity when bfg-jumping at origin
//calculates the horizontal velocity needed for a jump and returns true this velocity could be calculated
//calculates the horizontal velocity needed for a jump and returns true this velocity could be calculated
//
//
//
//
//#define AAS_MOVE_DEBUG
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DropToFloor(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    end[2] -= 100f32;
    trace = crate::src::botlib::be_aas_bspq3::AAS_Trace(origin, mins, maxs, end.as_mut_ptr(), 0, 1);
    if trace.startsolid as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    *origin.offset(0) = trace.endpos[0];
    *origin.offset(1) = trace.endpos[1];
    *origin.offset(2) = trace.endpos[2];
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//
//
//end of the function AAS_DropToFloor
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitSettings() {
    aassettings.phys_gravitydirection[0] = 0f32;
    aassettings.phys_gravitydirection[1] = 0f32;
    aassettings.phys_gravitydirection[2] = -1f32;
    aassettings.phys_friction = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_friction\x00" as *const u8 as *const i8,
        b"6\x00" as *const u8 as *const i8,
    );
    aassettings.phys_stopspeed = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_stopspeed\x00" as *const u8 as *const i8,
        b"100\x00" as *const u8 as *const i8,
    );
    aassettings.phys_gravity = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_gravity\x00" as *const u8 as *const i8,
        b"800\x00" as *const u8 as *const i8,
    );
    aassettings.phys_waterfriction = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_waterfriction\x00" as *const u8 as *const i8,
        b"1\x00" as *const u8 as *const i8,
    );
    aassettings.phys_watergravity = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_watergravity\x00" as *const u8 as *const i8,
        b"400\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxvelocity = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxvelocity\x00" as *const u8 as *const i8,
        b"320\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxwalkvelocity = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxwalkvelocity\x00" as *const u8 as *const i8,
        b"320\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxcrouchvelocity = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxcrouchvelocity\x00" as *const u8 as *const i8,
        b"100\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxswimvelocity = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxswimvelocity\x00" as *const u8 as *const i8,
        b"150\x00" as *const u8 as *const i8,
    );
    aassettings.phys_walkaccelerate = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_walkaccelerate\x00" as *const u8 as *const i8,
        b"10\x00" as *const u8 as *const i8,
    );
    aassettings.phys_airaccelerate = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_airaccelerate\x00" as *const u8 as *const i8,
        b"1\x00" as *const u8 as *const i8,
    );
    aassettings.phys_swimaccelerate = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_swimaccelerate\x00" as *const u8 as *const i8,
        b"4\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxstep = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxstep\x00" as *const u8 as *const i8,
        b"19\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxsteepness = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxsteepness\x00" as *const u8 as *const i8,
        b"0.7\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxwaterjump = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxwaterjump\x00" as *const u8 as *const i8,
        b"18\x00" as *const u8 as *const i8,
    );
    aassettings.phys_maxbarrier = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_maxbarrier\x00" as *const u8 as *const i8,
        b"33\x00" as *const u8 as *const i8,
    );
    aassettings.phys_jumpvel = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_jumpvel\x00" as *const u8 as *const i8,
        b"270\x00" as *const u8 as *const i8,
    );
    aassettings.phys_falldelta5 = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_falldelta5\x00" as *const u8 as *const i8,
        b"40\x00" as *const u8 as *const i8,
    );
    aassettings.phys_falldelta10 = crate::src::botlib::l_libvar::LibVarValue(
        b"phys_falldelta10\x00" as *const u8 as *const i8,
        b"60\x00" as *const u8 as *const i8,
    );
    aassettings.rs_waterjump = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_waterjump\x00" as *const u8 as *const i8,
        b"400\x00" as *const u8 as *const i8,
    );
    aassettings.rs_teleport = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_teleport\x00" as *const u8 as *const i8,
        b"50\x00" as *const u8 as *const i8,
    );
    aassettings.rs_barrierjump = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_barrierjump\x00" as *const u8 as *const i8,
        b"100\x00" as *const u8 as *const i8,
    );
    aassettings.rs_startcrouch = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_startcrouch\x00" as *const u8 as *const i8,
        b"300\x00" as *const u8 as *const i8,
    );
    aassettings.rs_startgrapple = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_startgrapple\x00" as *const u8 as *const i8,
        b"500\x00" as *const u8 as *const i8,
    );
    aassettings.rs_startwalkoffledge = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_startwalkoffledge\x00" as *const u8 as *const i8,
        b"70\x00" as *const u8 as *const i8,
    );
    aassettings.rs_startjump = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_startjump\x00" as *const u8 as *const i8,
        b"300\x00" as *const u8 as *const i8,
    );
    aassettings.rs_rocketjump = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_rocketjump\x00" as *const u8 as *const i8,
        b"500\x00" as *const u8 as *const i8,
    );
    aassettings.rs_bfgjump = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_bfgjump\x00" as *const u8 as *const i8,
        b"500\x00" as *const u8 as *const i8,
    );
    aassettings.rs_jumppad = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_jumppad\x00" as *const u8 as *const i8,
        b"250\x00" as *const u8 as *const i8,
    );
    aassettings.rs_aircontrolledjumppad = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_aircontrolledjumppad\x00" as *const u8 as *const i8,
        b"300\x00" as *const u8 as *const i8,
    );
    aassettings.rs_funcbob = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_funcbob\x00" as *const u8 as *const i8,
        b"300\x00" as *const u8 as *const i8,
    );
    aassettings.rs_startelevator = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_startelevator\x00" as *const u8 as *const i8,
        b"50\x00" as *const u8 as *const i8,
    );
    aassettings.rs_falldamage5 = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_falldamage5\x00" as *const u8 as *const i8,
        b"300\x00" as *const u8 as *const i8,
    );
    aassettings.rs_falldamage10 = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_falldamage10\x00" as *const u8 as *const i8,
        b"500\x00" as *const u8 as *const i8,
    );
    aassettings.rs_maxfallheight = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_maxfallheight\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
    );
    aassettings.rs_maxjumpfallheight = crate::src::botlib::l_libvar::LibVarValue(
        b"rs_maxjumpfallheight\x00" as *const u8 as *const i8,
        b"450\x00" as *const u8 as *const i8,
    );
}
//end of the function AAS_InitSettings
//===========================================================================
// returns qtrue if the bot is against a ladder
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AgainstLadder(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut areanum: i32 = 0; //end if
    let mut _i: i32 = 0;
    let mut facenum: i32 = 0;
    let mut side: i32 = 0;
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    org[0] = *origin.offset(0);
    org[1] = *origin.offset(1);
    org[2] = *origin.offset(2);
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
    if areanum == 0 {
        org[0] += 1f32;
        areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
        if areanum == 0 {
            org[1] += 1f32;
            areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
            if areanum == 0 {
                org[0] -= 2f32;
                areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
                if areanum == 0 {
                    org[1] -= 2f32;
                    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr())
                }
                //end if
                //end if
                //end if
            }
        }
    }
    //if in solid... wrrr shouldn't happen
    if areanum == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if not in a ladder area
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 2
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if a crouch only area
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .presencetype
        & 2
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t; //end for

    for i in 0..(*area).numfaces {
        facenum = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area).firstface + i) as isize);

        side = (facenum < 0) as i32;

        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset((crate::stdlib::abs as unsafe extern "C" fn(_: i32) -> i32)(facenum) as isize)
            as *mut crate::aasfile_h::aas_face_t;
        //end if
        //if the face isn't a ladder face
        if !((*face).faceflags & 2 == 0) {
            //get the plane the face is in
            plane = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset(((*face).planenum ^ side) as isize)
                as *mut crate::aasfile_h::aas_plane_t;
            //if the origin is pretty close to the plane
            if crate::stdlib::fabsf(
                (*plane).normal[0] * *origin.offset(0)
                    + (*plane).normal[1] * *origin.offset(1)
                    + (*plane).normal[2] * *origin.offset(2)
                    - (*plane).dist,
            ) < 3f32
            {
                if crate::src::botlib::be_aas_sample::AAS_PointInsideFace(
                    crate::stdlib::abs(facenum),
                    origin,
                    0.1,
                ) as u64
                    != 0
                {
                    return crate::src::qcommon::q_shared::qtrue as i32;
                }
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function AAS_AgainstLadder
//===========================================================================
// returns qtrue if the bot is on the ground
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_OnGround(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: i32,
    mut passent: i32,
) -> i32 {
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    end[2] -= 10f32;
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        origin,
        end.as_mut_ptr(),
        presencetype,
        passent,
    );
    //if in solid
    if trace.startsolid as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if nothing hit at all
    if trace.fraction as f64 >= 1.0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if too far from the hit plane
    if *origin.offset(2) - trace.endpos[2] > 10f32 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //check if the plane isn't too steep
    plane = crate::src::botlib::be_aas_sample::AAS_PlaneFromNum(trace.planenum);
    if (*plane).normal[0] * up[0] + (*plane).normal[1] * up[1] + (*plane).normal[2] * up[2]
        < aassettings.phys_maxsteepness
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //the bot is on the ground
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function AAS_OnGround
//===========================================================================
// returns qtrue if a bot at the given position is swimming
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Swimming(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut testorg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    testorg[0] = *origin.offset(0);
    testorg[1] = *origin.offset(1);
    testorg[2] = *origin.offset(2);
    testorg[2] -= 2f32;
    if crate::src::botlib::be_aas_bspq3::AAS_PointContents(testorg.as_mut_ptr()) & (8 | 16 | 32)
        != 0
    {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function AAS_Swimming
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================

static mut VEC_UP: crate::src::qcommon::q_shared::vec3_t = [0f32, -1f32, 0f32];

static mut MOVEDIR_UP: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];

static mut VEC_DOWN: crate::src::qcommon::q_shared::vec3_t = [0f32, -2f32, 0f32];

static mut MOVEDIR_DOWN: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, -1f32];
#[no_mangle]

pub unsafe extern "C" fn AAS_SetMovedir(
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut movedir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    if VectorCompare(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        VEC_UP.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) != 0
    {
        *movedir.offset(0) = MOVEDIR_UP[0]; //end if
        *movedir.offset(1) = MOVEDIR_UP[1]; //end else if
        *movedir.offset(2) = MOVEDIR_UP[2]
    } else if VectorCompare(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        VEC_DOWN.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) != 0
    {
        *movedir.offset(0) = MOVEDIR_DOWN[0];
        *movedir.offset(1) = MOVEDIR_DOWN[1];
        *movedir.offset(2) = MOVEDIR_DOWN[2]
    } else {
        crate::src::qcommon::q_math::AngleVectors(
            angles as *const crate::src::qcommon::q_shared::vec_t,
            movedir,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
    };
    //end else
}
//end of the function AAS_SetMovedir
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_JumpReachRunStart(
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
    mut runstart: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    //
    hordir[0] = (*reach).start[0] - (*reach).end[0];
    hordir[1] = (*reach).start[1] - (*reach).end[1];
    hordir[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
    //start point
    start[0] = (*reach).start[0];
    start[1] = (*reach).start[1];
    start[2] = (*reach).start[2];
    start[2] += 1f32;
    //get command movement
    cmdmove[0] = hordir[0] * 400f32;
    cmdmove[1] = hordir[1] * 400f32;
    cmdmove[2] = hordir[2] * 400f32;
    //
    AAS_PredictClientMovement(
        &mut move_0,
        -(1),
        start.as_mut_ptr(),
        2,
        crate::src::qcommon::q_shared::qtrue as i32,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr(),
        cmdmove.as_mut_ptr(),
        1,
        2,
        0.1,
        4 | 8 | 16 | 32 | 64,
        0,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    *runstart.offset(0) = move_0.endpos[0];
    *runstart.offset(1) = move_0.endpos[1];
    *runstart.offset(2) = move_0.endpos[2];
    //don't enter slime or lava and don't fall from too high
    if move_0.stopevent & (8 | 16 | 32) != 0 {
        *runstart.offset(0) = start[0];
        *runstart.offset(1) = start[1];
        *runstart.offset(2) = start[2]
    };
    //end if
}
//end of the function AAS_JumpReachRunStart
//===========================================================================
// returns the Z velocity when rocket jumping at the origin
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_WeaponJumpZVelocity(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radiusdamage: f32,
) -> f32 {
    let mut kvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut viewangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mass: f32 = 0.;
    let mut knockback: f32 = 0.;
    let mut points: f32 = 0.;
    let mut rocketoffset: crate::src::qcommon::q_shared::vec3_t = [8f32, 8f32, -8f32];
    let mut botmins: crate::src::qcommon::q_shared::vec3_t = [-16f32, -16f32, -24f32];
    let mut botmaxs: crate::src::qcommon::q_shared::vec3_t = [16f32, 16f32, 32f32];
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
    //look down (90 degrees)
    viewangles[0] = 90f32;
    viewangles[1] = 0f32;
    viewangles[2] = 0f32;
    //get the start point shooting from
    start[0] = *origin.offset(0); //view offset Z
    start[1] = *origin.offset(1);
    start[2] = *origin.offset(2);
    start[2] += 8f32;
    crate::src::qcommon::q_math::AngleVectors(
        viewangles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    start[0] += forward[0] * rocketoffset[0] + right[0] * rocketoffset[1];
    start[1] += forward[1] * rocketoffset[0] + right[1] * rocketoffset[1];
    start[2] += forward[2] * rocketoffset[0] + right[2] * rocketoffset[1] + rocketoffset[2];
    //end point of the trace
    end[0] = start[0] + forward[0] * 500f32;
    end[1] = start[1] + forward[1] * 500f32;
    end[2] = start[2] + forward[2] * 500f32;
    //trace a line to get the impact point
    bsptrace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
        start.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr(),
        1,
        1,
    );
    //calculate the damage the bot will get from the rocket impact
    v[0] = botmins[0] + botmaxs[0];
    v[1] = botmins[1] + botmaxs[1];
    v[2] = botmins[2] + botmaxs[2];
    v[0] = (*origin.offset(0) as f64 + v[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    v[1] = (*origin.offset(1) as f64 + v[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    v[2] = (*origin.offset(2) as f64 + v[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    v[0] = bsptrace.endpos[0] - v[0];
    v[1] = bsptrace.endpos[1] - v[1];
    v[2] = bsptrace.endpos[2] - v[2];
    //
    points = (radiusdamage as f64
        - 0.5 * VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) as f64)
        as f32;
    if points < 0f32 {
        points = 0f32
    }
    //the owner of the rocket gets half the damage
    points = (points as f64 * 0.5) as f32;
    //mass of the bot (p_client.c: PutClientInServer)
    mass = 200f32;
    //knockback is the same as the damage points
    knockback = points;
    //direction of the damage (from trace.endpos to bot origin)
    dir[0] = *origin.offset(0) - bsptrace.endpos[0];
    dir[1] = *origin.offset(1) - bsptrace.endpos[1];
    dir[2] = *origin.offset(2) - bsptrace.endpos[2];
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    //damage velocity
    kvel[0] = (dir[0] as f64 * (1600.0 * knockback as f64 / mass as f64))
        as crate::src::qcommon::q_shared::vec_t; //the rocket jump hack...
    kvel[1] = (dir[1] as f64 * (1600.0 * knockback as f64 / mass as f64))
        as crate::src::qcommon::q_shared::vec_t;
    kvel[2] = (dir[2] as f64 * (1600.0 * knockback as f64 / mass as f64))
        as crate::src::qcommon::q_shared::vec_t;
    //rocket impact velocity + jump velocity
    return kvel[2] + aassettings.phys_jumpvel;
}
//end of the function AAS_WeaponJumpZVelocity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_RocketJumpZVelocity(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    //rocket radius damage is 120 (p_weapon.c: Weapon_RocketLauncher_Fire)
    return AAS_WeaponJumpZVelocity(origin, 120f32);
}
//end of the function AAS_RocketJumpZVelocity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BFGJumpZVelocity(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    //bfg radius damage is 1000 (p_weapon.c: weapon_bfg_fire)
    return AAS_WeaponJumpZVelocity(origin, 120f32);
}
//end of the function AAS_BFGJumpZVelocity
//===========================================================================
// applies ground friction to the given velocity
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Accelerate(
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut frametime: f32,
    mut wishdir: *mut crate::src::qcommon::q_shared::vec_t,
    mut wishspeed: f32,
    mut accel: f32,
) {
    // q2 style
    let mut i: i32 = 0;
    let mut addspeed: f32 = 0.;
    let mut accelspeed: f32 = 0.;
    let mut currentspeed: f32 = 0.;
    currentspeed = *velocity.offset(0) * *wishdir.offset(0)
        + *velocity.offset(1) * *wishdir.offset(1)
        + *velocity.offset(2) * *wishdir.offset(2);
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0f32 {
        return;
    }
    accelspeed = accel * frametime * wishspeed;
    if accelspeed > addspeed {
        accelspeed = addspeed
    }
    i = 0;
    while i < 3 {
        let ref mut fresh0 = *velocity.offset(i as isize);
        *fresh0 += accelspeed * *wishdir.offset(i as isize);
        i += 1
    }
}
//end of the function AAS_Accelerate
//===========================================================================
// applies ground friction to the given velocity
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ApplyFriction(
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut friction: f32,
    mut stopspeed: f32,
    mut frametime: f32,
) {
    let mut speed: f32 = 0.;
    let mut control: f32 = 0.;
    let mut newspeed: f32 = 0.;
    //horizontal speed
    speed = crate::stdlib::sqrt(
        (*vel.offset(0) * *vel.offset(0) + *vel.offset(1) * *vel.offset(1)) as f64,
    ) as f32;
    if speed != 0. {
        control = if speed < stopspeed { stopspeed } else { speed };
        newspeed = speed - frametime * control * friction;
        if newspeed < 0f32 {
            newspeed = 0f32
        }
        newspeed /= speed;
        let ref mut fresh1 = *vel.offset(0);
        *fresh1 *= newspeed;
        let ref mut fresh2 = *vel.offset(1);
        *fresh2 *= newspeed
    };
    //end if
}
//end of the function AAS_ApplyFriction
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClipToBBox(
    mut trace: *mut crate::be_aas_h::aas_trace_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: i32,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut side: i32 = 0;
    let mut front: f32 = 0.;
    let mut back: f32 = 0.;
    let mut frac: f32 = 0.;
    let mut planedist: f32 = 0.;
    let mut bboxmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bboxmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox(
        presencetype,
        bboxmins.as_mut_ptr(),
        bboxmaxs.as_mut_ptr(),
    );
    absmins[0] = *mins.offset(0) - bboxmaxs[0];
    absmins[1] = *mins.offset(1) - bboxmaxs[1];
    absmins[2] = *mins.offset(2) - bboxmaxs[2];
    absmaxs[0] = *maxs.offset(0) - bboxmins[0];
    absmaxs[1] = *maxs.offset(1) - bboxmins[1];
    absmaxs[2] = *maxs.offset(2) - bboxmins[2];
    //
    (*trace).endpos[0] = *end.offset(0); //end for
    (*trace).endpos[1] = *end.offset(1);
    (*trace).endpos[2] = *end.offset(2);
    (*trace).fraction = 1f32;
    i = 0;
    while i < 3 {
        if *start.offset(i as isize) < absmins[i as usize]
            && *end.offset(i as isize) < absmins[i as usize]
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if *start.offset(i as isize) > absmaxs[i as usize]
            && *end.offset(i as isize) > absmaxs[i as usize]
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        i += 1
    }
    //check bounding box collision
    dir[0] = *end.offset(0) - *start.offset(0); //end for
    dir[1] = *end.offset(1) - *start.offset(1);
    dir[2] = *end.offset(2) - *start.offset(2);
    frac = 1f32;
    i = 0;
    while i < 3 {
        //get plane to test collision with for the current axis direction
        if dir[i as usize] > 0f32 {
            planedist = absmins[i as usize]
        } else {
            planedist = absmaxs[i as usize]
        }
        //end if
        //calculate collision fraction
        front = *start.offset(i as isize) - planedist;
        back = *end.offset(i as isize) - planedist;
        frac = front / (front - back);
        //check if between bounding planes of next axis
        side = i + 1;
        if side > 2 {
            side = 0
        }
        mid[side as usize] = *start.offset(side as isize) + dir[side as usize] * frac;
        if mid[side as usize] > absmins[side as usize]
            && mid[side as usize] < absmaxs[side as usize]
        {
            //check if between bounding planes of next axis
            side += 1;
            if side > 2 {
                side = 0
            }
            mid[side as usize] = *start.offset(side as isize) + dir[side as usize] * frac;
            if mid[side as usize] > absmins[side as usize]
                && mid[side as usize] < absmaxs[side as usize]
            {
                mid[i as usize] = planedist;
                break;
            }
            //end if
        }
        i += 1
    }
    //if there was a collision
    if i != 3 {
        (*trace).startsolid = crate::src::qcommon::q_shared::qfalse; //end if
        (*trace).fraction = frac;
        (*trace).ent = 0;
        (*trace).planenum = 0;
        (*trace).area = 0;
        (*trace).lastarea = 0;

        for j in 0..3 {
            (*trace).endpos[j as usize] = *start.offset(j as isize) + dir[j as usize] * frac;
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function AAS_ClipToBBox
//===========================================================================
// predicts the movement
// assumes regular bounding box sizes
// NOTE: out of water jumping is not included
// NOTE: grappling hook is not included
//
// Parameter:			origin			: origin to start with
//						presencetype	: presence type to start with
//						velocity		: velocity to start with
//						cmdmove			: client command movement
//						cmdframes		: number of frame cmdmove is valid
//						maxframes		: maximum number of predicted frames
//						frametime		: duration of one predicted frame
//						stopevent		: events that stop the prediction
//						stopareanum		: stop as soon as entered this area
// Returns:				aas_clientmove_t
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClientMovementPrediction(
    mut move_0: *mut crate::be_aas_h::aas_clientmove_s,
    mut entnum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: i32,
    mut onground: i32,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdmove: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdframes: i32,
    mut maxframes: i32,
    mut frametime: f32,
    mut stopevent: i32,
    mut stopareanum: i32,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut visualize: i32,
) -> i32 {
    let mut phys_friction: f32 = 0.;
    let mut phys_stopspeed: f32 = 0.;
    let mut phys_gravity: f32 = 0.;
    let mut phys_waterfriction: f32 = 0.;
    let mut phys_watergravity: f32 = 0.;
    let mut phys_walkaccelerate: f32 = 0.;
    let mut phys_airaccelerate: f32 = 0.;
    let mut phys_swimaccelerate: f32 = 0.;
    let mut phys_maxwalkvelocity: f32 = 0.;
    let mut phys_maxcrouchvelocity: f32 = 0.;
    let mut phys_maxswimvelocity: f32 = 0.;
    let mut phys_maxstep: f32 = 0.;
    let mut phys_maxsteepness: f32 = 0.;
    let mut phys_jumpvel: f32 = 0.;
    let mut friction: f32 = 0.;
    let mut gravity: f32 = 0.;
    let mut delta: f32 = 0.;
    let mut maxvel: f32 = 0.;
    let mut wishspeed: f32 = 0.;
    let mut accelerate: f32 = 0.;
    //float velchange, newvel;
    //int ax;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pc: i32 = 0;
    let mut step: i32 = 0;
    let mut swimming: i32 = 0;
    let mut crouch: i32 = 0;
    let mut event: i32 = 0;
    let mut jump_frame: i32 = 0;
    let mut areanum: i32 = 0;
    let mut areas: [i32; 20] = [0; 20];
    let mut numareas: i32 = 0;
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 20] = [[0.; 3]; 20];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut feet: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut stepend: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lastorg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wishdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut frame_test_vel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut old_frame_test_vel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut left_test_vel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut plane2: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    let mut steptrace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    if frametime <= 0f32 {
        frametime = 0.1f32
    }
    //
    phys_friction = aassettings.phys_friction; // * frametime;
    phys_stopspeed = aassettings.phys_stopspeed; // * frametime;
    phys_gravity = aassettings.phys_gravity; // * frametime;
    phys_waterfriction = aassettings.phys_waterfriction;
    phys_watergravity = aassettings.phys_watergravity;
    phys_maxwalkvelocity = aassettings.phys_maxwalkvelocity;
    phys_maxcrouchvelocity = aassettings.phys_maxcrouchvelocity;
    phys_maxswimvelocity = aassettings.phys_maxswimvelocity;
    phys_walkaccelerate = aassettings.phys_walkaccelerate;
    phys_airaccelerate = aassettings.phys_airaccelerate;
    phys_swimaccelerate = aassettings.phys_swimaccelerate;
    phys_maxstep = aassettings.phys_maxstep;
    phys_maxsteepness = aassettings.phys_maxsteepness;
    phys_jumpvel = aassettings.phys_jumpvel * frametime;
    //
    crate::stdlib::memset(
        move_0 as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::be_aas_h::aas_clientmove_t>(),
    );
    crate::stdlib::memset(
        &mut trace as *mut crate::be_aas_h::aas_trace_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::be_aas_h::aas_trace_t>(),
    );
    //start at the current origin
    org[0] = *origin.offset(0);
    org[1] = *origin.offset(1);
    org[2] = *origin.offset(2);
    org[2] = (org[2] as f64 + 0.25) as crate::src::qcommon::q_shared::vec_t;
    //velocity to test for the first frame
    frame_test_vel[0] = *velocity.offset(0) * frametime;
    frame_test_vel[1] = *velocity.offset(1) * frametime;
    frame_test_vel[2] = *velocity.offset(2) * frametime;
    //
    jump_frame = -(1);
    //predict a maximum of 'maxframes' ahead
    n = 0; //end for
    while n < maxframes {
        swimming = AAS_Swimming(org.as_mut_ptr());
        //end else if
        gravity = if swimming != 0 {
            phys_watergravity
        } else {
            phys_gravity
        };
        frame_test_vel[2] = (frame_test_vel[2] as f64 - gravity as f64 * 0.1 * frametime as f64)
            as crate::src::qcommon::q_shared::vec_t;
        if onground != 0 || swimming != 0 {
            friction = if swimming != 0 {
                phys_waterfriction
            } else {
                phys_friction
            };
            //get gravity depending on swimming or not
            //apply gravity at the START of the frame
            //if on the ground or swimming
            //end if
            //apply friction
            frame_test_vel[0] = frame_test_vel[0] * (1f32 / frametime);
            frame_test_vel[1] = frame_test_vel[1] * (1f32 / frametime);
            frame_test_vel[2] = frame_test_vel[2] * (1f32 / frametime);
            AAS_ApplyFriction(
                frame_test_vel.as_mut_ptr(),
                friction,
                phys_stopspeed,
                frametime,
            );
            frame_test_vel[0] = frame_test_vel[0] * frametime;
            frame_test_vel[1] = frame_test_vel[1] * frametime;
            frame_test_vel[2] = frame_test_vel[2] * frametime
        }
        crouch = crate::src::qcommon::q_shared::qfalse as i32;
        if n < cmdframes {
            //apply command movement
            //end if
            //ax = 0;
            maxvel = phys_maxwalkvelocity;
            accelerate = phys_airaccelerate;
            wishdir[0] = *cmdmove.offset(0);
            wishdir[1] = *cmdmove.offset(1);
            wishdir[2] = *cmdmove.offset(2);
            /*
            for (i = 0; i < ax; i++)
            {
                velchange = (cmdmove[i] * frametime) - frame_test_vel[i];
                if (velchange > phys_maxacceleration) velchange = phys_maxacceleration;
                else if (velchange < -phys_maxacceleration) velchange = -phys_maxacceleration;
                newvel = frame_test_vel[i] + velchange;
                //
                if (frame_test_vel[i] <= maxvel && newvel > maxvel) frame_test_vel[i] = maxvel;
                else if (frame_test_vel[i] >= -maxvel && newvel < -maxvel) frame_test_vel[i] = -maxvel;
                else frame_test_vel[i] = newvel;
            } //end for
            */
            if onground != 0 {
                //end if
                if *cmdmove.offset(2) < -300f32 {
                    crouch = crate::src::qcommon::q_shared::qtrue as i32; //end if
                    maxvel = phys_maxcrouchvelocity
                }
                //end else
                //ax = 2;
                if swimming == 0 && *cmdmove.offset(2) > 1f32 {
                    //if not swimming and upmove is positive then jump
                    frame_test_vel[2] =
                        (phys_jumpvel as f64 - gravity as f64 * 0.1 * frametime as f64 + 5f64)
                            as crate::src::qcommon::q_shared::vec_t; //end if
                    jump_frame = n;
                    //jump velocity minus the gravity for one frame + 5 for safety
                    //jumping so air accelerate
                    accelerate = phys_airaccelerate
                } else {
                    accelerate = phys_walkaccelerate
                }
            } //end else
            if swimming != 0 {
                maxvel = phys_maxswimvelocity;
                accelerate = phys_swimaccelerate
            //ax = 3;
            } else {
                wishdir[2] = 0f32
            }
            wishspeed = crate::src::qcommon::q_math::VectorNormalize(wishdir.as_mut_ptr());
            if wishspeed > maxvel {
                wishspeed = maxvel
            }
            frame_test_vel[0] = frame_test_vel[0] * (1f32 / frametime);
            frame_test_vel[1] = frame_test_vel[1] * (1f32 / frametime);
            frame_test_vel[2] = frame_test_vel[2] * (1f32 / frametime);
            AAS_Accelerate(
                frame_test_vel.as_mut_ptr(),
                frametime,
                wishdir.as_mut_ptr(),
                wishspeed,
                accelerate,
            );
            frame_test_vel[0] = frame_test_vel[0] * frametime;
            frame_test_vel[1] = frame_test_vel[1] * frametime;
            frame_test_vel[2] = frame_test_vel[2] * frametime
        }
        if crouch != 0 {
            //
            //end else
            presencetype = 4
        } else if presencetype == 4 {
            if crate::src::botlib::be_aas_sample::AAS_PointPresenceType(org.as_mut_ptr()) & 2 != 0 {
                presencetype = 2
            } //end if
              //end if
        }
        lastorg[0] = org[0];
        lastorg[1] = org[1];
        lastorg[2] = org[2];
        left_test_vel[0] = frame_test_vel[0];
        left_test_vel[1] = frame_test_vel[1];
        left_test_vel[2] = frame_test_vel[2];
        j = 0;
        loop {
            end[0] = org[0] + left_test_vel[0];
            end[1] = org[1] + left_test_vel[1];
            end[2] = org[2] + left_test_vel[2];
            //save the current origin
            //move linear during one frame
            //while there is a plane hit
            trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                org.as_mut_ptr(),
                end.as_mut_ptr(),
                presencetype,
                entnum,
            );
            if visualize != 0 {
                if trace.startsolid as u64 != 0 {
                    botimport.Print.expect("non-null function pointer")(
                        1i32,
                        b"PredictMovement: start solid\n\x00" as *const u8 as *mut i8,
                    );
                }
                crate::src::botlib::be_aas_debug::AAS_DebugLine(
                    org.as_mut_ptr(),
                    trace.endpos.as_mut_ptr(),
                    1i32,
                );
            }
            if stopevent & (512 | 128 | 256 | 4096) != 0 {
                numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                    org.as_mut_ptr(),
                    trace.endpos.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    points.as_mut_ptr(),
                    20,
                );
                i = 0;
                while i < numareas {
                    //trace a bounding box
                    //
                    //#ifdef AAS_MOVE_DEBUG
                    //end if
                    //#endif //AAS_MOVE_DEBUG
                    //
                    //end if
                    //end for
                    if stopevent & 512 != 0 {
                        if areas[i as usize] == stopareanum {
                            (*move_0).endpos[0] = points[i as usize][0]; //end if
                            (*move_0).endpos[1] = points[i as usize][1];
                            (*move_0).endpos[2] = points[i as usize][2];
                            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 512;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0;
                            (*move_0).time = n as f32 * frametime;
                            (*move_0).frames = n;
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        }
                        //end if
                    }
                    //end if
                    if stopevent & 128 != 0 && n != 0 {
                        if (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(areas[i as usize] as isize))
                        .contents
                            & 128
                            != 0
                        {
                            (*move_0).endpos[0] = points[i as usize][0];
                            (*move_0).endpos[1] = points[i as usize][1];
                            (*move_0).endpos[2] = points[i as usize][2];
                            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 128;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0;
                            (*move_0).time = n as f32 * frametime;
                            (*move_0).frames = n;
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        }
                        //NOTE: if not the first frame
                        //end if
                        //end if
                    } //end if
                    if stopevent & 256 != 0 {
                        if (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(areas[i as usize] as isize))
                        .contents
                            & 64
                            != 0
                        {
                            (*move_0).endpos[0] = points[i as usize][0];
                            (*move_0).endpos[1] = points[i as usize][1];
                            (*move_0).endpos[2] = points[i as usize][2];
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 256;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0;
                            (*move_0).time = n as f32 * frametime;
                            (*move_0).frames = n;
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        }
                        //end if
                    }
                    if stopevent & 4096 != 0 {
                        if (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(areas[i as usize] as isize))
                        .contents
                            & 8
                            != 0
                        {
                            (*move_0).endpos[0] = points[i as usize][0];
                            (*move_0).endpos[1] = points[i as usize][1];
                            (*move_0).endpos[2] = points[i as usize][2];
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 4096;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0;
                            (*move_0).time = n as f32 * frametime;
                            (*move_0).frames = n;
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        }
                        //end if
                    }
                    i += 1
                }
            }
            if stopevent & 2048 != 0 {
                if AAS_ClipToBBox(
                    &mut trace,
                    org.as_mut_ptr(),
                    trace.endpos.as_mut_ptr(),
                    presencetype,
                    mins,
                    maxs,
                ) != 0
                {
                    (*move_0).endpos[0] = trace.endpos[0];
                    (*move_0).endpos[1] = trace.endpos[1];
                    (*move_0).endpos[2] = trace.endpos[2];
                    (*move_0).endarea = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                        (*move_0).endpos.as_mut_ptr(),
                    );
                    (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                    (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                    (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                    (*move_0).trace = trace;
                    (*move_0).stopevent = 2048;
                    (*move_0).presencetype = presencetype;
                    (*move_0).endcontents = 0;
                    (*move_0).time = n as f32 * frametime;
                    (*move_0).frames = n;
                    return crate::src::qcommon::q_shared::qtrue as i32;
                }
                //
                //end if
                //end if
            }
            org[0] = trace.endpos[0];
            org[1] = trace.endpos[1];
            org[2] = trace.endpos[2];
            if (trace.fraction as f64) < 1.0 {
                //move the entity to the trace end point
                //if there was a collision
                //end if
                //get the plane the bounding box collided with
                plane = crate::src::botlib::be_aas_sample::AAS_PlaneFromNum(trace.planenum);
                //end if
                if stopevent & 1024 != 0 {
                    if (*plane).normal[0] * up[0]
                        + (*plane).normal[1] * up[1]
                        + (*plane).normal[2] * up[2]
                        > phys_maxsteepness
                    {
                        start[0] = org[0];
                        start[1] = org[1];
                        start[2] = org[2];
                        start[2] = (start[2] as f64 + 0.5) as crate::src::qcommon::q_shared::vec_t;
                        if crate::src::botlib::be_aas_sample::AAS_PointAreaNum(start.as_mut_ptr())
                            == stopareanum
                        {
                            (*move_0).endpos[0] = start[0];
                            (*move_0).endpos[1] = start[1];
                            (*move_0).endpos[2] = start[2];
                            (*move_0).endarea = stopareanum;
                            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 1024;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0;
                            (*move_0).time = n as f32 * frametime;
                            (*move_0).frames = n;
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        }
                        //
                        //end if
                        //end if
                    }
                    //end if
                }
                step = crate::src::qcommon::q_shared::qfalse as i32;
                if (*plane).normal[2] == 0f32 && (jump_frame < 0 || n - jump_frame > 2) {
                    //assume there's no step
                    //if it is a vertical plane and the bot didn't jump recently
                    //end if
                    //check for a step
                    start[0] = (org[0] as f64 + (*plane).normal[0] as f64 * -0.25)
                        as crate::src::qcommon::q_shared::vec_t;
                    start[1] = (org[1] as f64 + (*plane).normal[1] as f64 * -0.25)
                        as crate::src::qcommon::q_shared::vec_t;
                    start[2] = (org[2] as f64 + (*plane).normal[2] as f64 * -0.25)
                        as crate::src::qcommon::q_shared::vec_t;
                    stepend[0] = start[0];
                    stepend[1] = start[1];
                    stepend[2] = start[2];
                    start[2] += phys_maxstep;
                    steptrace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                        start.as_mut_ptr(),
                        stepend.as_mut_ptr(),
                        presencetype,
                        entnum,
                    );
                    //end if
                    if steptrace.startsolid as u64 == 0 {
                        plane2 =
                            crate::src::botlib::be_aas_sample::AAS_PlaneFromNum(steptrace.planenum);
                        if (*plane2).normal[0] * up[0]
                            + (*plane2).normal[1] * up[1]
                            + (*plane2).normal[2] * up[2]
                            > phys_maxsteepness
                        {
                            left_test_vel[0] = end[0] - steptrace.endpos[0];
                            left_test_vel[1] = end[1] - steptrace.endpos[1];
                            left_test_vel[2] = end[2] - steptrace.endpos[2];
                            left_test_vel[2] = 0f32;
                            frame_test_vel[2] = 0f32;
                            //
                            //end if
                            //#ifdef AAS_MOVE_DEBUG
                            if visualize != 0 {
                                if (steptrace.endpos[2] - org[2]) as f64 > 0.125 {
                                    start[0] = org[0]; //end if
                                    start[1] = org[1];
                                    start[2] = org[2];
                                    start[2] = steptrace.endpos[2];
                                    crate::src::botlib::be_aas_debug::AAS_DebugLine(
                                        org.as_mut_ptr(),
                                        start.as_mut_ptr(),
                                        3i32,
                                    );
                                }
                                //end if
                            }
                            //#endif //AAS_MOVE_DEBUG
                            org[2] = steptrace.endpos[2];
                            step = crate::src::qcommon::q_shared::qtrue as i32
                        }
                    }
                }
                if step == 0 {
                    //
                    //velocity left to test for this frame is the projection
                    //of the current test velocity into the hit plane
                    left_test_vel[0] = left_test_vel[0]
                        + (*plane).normal[0]
                            * -(left_test_vel[0] * (*plane).normal[0]
                                + left_test_vel[1] * (*plane).normal[1]
                                + left_test_vel[2] * (*plane).normal[2]);
                    left_test_vel[1] = left_test_vel[1]
                        + (*plane).normal[1]
                            * -(left_test_vel[0] * (*plane).normal[0]
                                + left_test_vel[1] * (*plane).normal[1]
                                + left_test_vel[2] * (*plane).normal[2]);
                    left_test_vel[2] = left_test_vel[2]
                        + (*plane).normal[2]
                            * -(left_test_vel[0] * (*plane).normal[0]
                                + left_test_vel[1] * (*plane).normal[1]
                                + left_test_vel[2] * (*plane).normal[2]);
                    //end if
                    old_frame_test_vel[0] = frame_test_vel[0];
                    old_frame_test_vel[1] = frame_test_vel[1];
                    old_frame_test_vel[2] = frame_test_vel[2];
                    frame_test_vel[0] = frame_test_vel[0]
                        + (*plane).normal[0]
                            * -(frame_test_vel[0] * (*plane).normal[0]
                                + frame_test_vel[1] * (*plane).normal[1]
                                + frame_test_vel[2] * (*plane).normal[2]);
                    frame_test_vel[1] = frame_test_vel[1]
                        + (*plane).normal[1]
                            * -(frame_test_vel[0] * (*plane).normal[0]
                                + frame_test_vel[1] * (*plane).normal[1]
                                + frame_test_vel[2] * (*plane).normal[2]);
                    frame_test_vel[2] = frame_test_vel[2]
                        + (*plane).normal[2]
                            * -(frame_test_vel[0] * (*plane).normal[0]
                                + frame_test_vel[1] * (*plane).normal[1]
                                + frame_test_vel[2] * (*plane).normal[2]);
                    if (*plane).normal[0] * up[0]
                        + (*plane).normal[1] * up[1]
                        + (*plane).normal[2] * up[2]
                        > phys_maxsteepness
                    {
                        onground = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    if stopevent & 32 != 0 {
                        delta = 0f32;
                        //store the old velocity for landing check
                        //test velocity for the next frame is the projection
                        //of the velocity of the current frame into the hit plane
                        //check for a landing on an almost horizontal floor
                        //end if
                        //end if
                        if old_frame_test_vel[2] < 0f32
                            && frame_test_vel[2] > old_frame_test_vel[2]
                            && onground == 0
                        {
                            //end else
                            delta = old_frame_test_vel[2]
                        } else if onground != 0 {
                            delta = frame_test_vel[2] - old_frame_test_vel[2]
                        } //end if
                        if delta != 0. {
                            delta = delta * 10f32;
                            delta = ((delta * delta) as f64 * 0.0001) as f32;
                            if swimming != 0 {
                                delta = 0f32
                            }
                            //end if
                            if delta > 40f32 {
                                (*move_0).endpos[0] = org[0];
                                (*move_0).endpos[1] = org[1];
                                (*move_0).endpos[2] = org[2];
                                (*move_0).endarea =
                                    crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                        org.as_mut_ptr(),
                                    );
                                (*move_0).velocity[0] = frame_test_vel[0];
                                (*move_0).velocity[1] = frame_test_vel[1];
                                (*move_0).velocity[2] = frame_test_vel[2];
                                (*move_0).trace = trace;
                                (*move_0).stopevent = 32;
                                (*move_0).presencetype = presencetype;
                                (*move_0).endcontents = 0;
                                (*move_0).time = n as f32 * frametime;
                                (*move_0).frames = n;
                                return crate::src::qcommon::q_shared::qtrue as i32;
                            }
                        }
                    }
                }
            }
            j += 1;
            if j > 20 {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            if !((trace.fraction as f64) < 1.0) {
                break;
            }
        }
        if frame_test_vel[2] <= 10f32 {
            // never take falling damage if completely underwater
            /*
            if (ent->waterlevel == 3) return;
            if (ent->waterlevel == 2) delta *= 0.25;
            if (ent->waterlevel == 1) delta *= 0.5;
            */
            //extra check to prevent endless loop
            //if going down
            //end if
            //check for a liquid at the feet of the bot
            feet[0] = org[0];
            feet[1] = org[1];
            feet[2] = org[2];
            feet[2] -= 22f32;
            pc = crate::src::botlib::be_aas_bspq3::AAS_PointContents(feet.as_mut_ptr());
            //end if
            event = 0;
            if pc & 8 != 0 {
                event |= 16
            }
            if pc & 16 != 0 {
                event |= 8
            }
            if pc & 32 != 0 {
                event |= 4
            }
            areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
            if (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(areanum as isize))
            .contents
                & 2
                != 0
            {
                event |= 16
            }
            if (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(areanum as isize))
            .contents
                & 4
                != 0
            {
                event |= 8
            }
            if (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(areanum as isize))
            .contents
                & 1
                != 0
            {
                event |= 4
            }
            if event & stopevent != 0 {
                (*move_0).endpos[0] = org[0];
                (*move_0).endpos[1] = org[1];
                (*move_0).endpos[2] = org[2];
                (*move_0).endarea = areanum;
                (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                (*move_0).stopevent = event & stopevent;
                (*move_0).presencetype = presencetype;
                (*move_0).endcontents = pc;
                (*move_0).time = n as f32 * frametime;
                (*move_0).frames = n;
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        onground = AAS_OnGround(org.as_mut_ptr(), presencetype, entnum);
        if onground != 0 {
            if stopevent & 1 != 0 {
                (*move_0).endpos[0] = org[0];
                (*move_0).endpos[1] = org[1];
                (*move_0).endpos[2] = org[2];
                (*move_0).endarea =
                    crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
                (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                (*move_0).trace = trace;
                (*move_0).stopevent = 1;
                (*move_0).presencetype = presencetype;
                (*move_0).endcontents = 0;
                (*move_0).time = n as f32 * frametime;
                (*move_0).frames = n;
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        //get event from pc
        //
        //if in lava or slime
        //
        //if onground and on the ground for at least one whole frame
        //end if
        } else if stopevent & 2 != 0 {
            (*move_0).endpos[0] = org[0]; //end else if
            (*move_0).endpos[1] = org[1];
            (*move_0).endpos[2] = org[2];
            (*move_0).endarea =
                crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
            (*move_0).trace = trace;
            (*move_0).stopevent = 2;
            (*move_0).presencetype = presencetype;
            (*move_0).endcontents = 0;
            (*move_0).time = n as f32 * frametime;
            (*move_0).frames = n;
            return crate::src::qcommon::q_shared::qtrue as i32;
        } else {
            if stopevent & 64 != 0 {
                let mut gaptrace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
                    startsolid: crate::src::qcommon::q_shared::qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,
                };
                start[0] = org[0];
                start[1] = org[1];
                start[2] = org[2];
                end[0] = start[0];
                end[1] = start[1];
                end[2] = start[2];
                end[2] -= 48f32 + aassettings.phys_maxbarrier;
                gaptrace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                    start.as_mut_ptr(),
                    end.as_mut_ptr(),
                    4,
                    -(1),
                );
                //end if
                if gaptrace.startsolid as u64 == 0 {
                    //if solid is found the bot cannot walk any further and will not fall into a gap
                    //if it is a gap (lower than one step height)
                    if gaptrace.endpos[2] < org[2] - aassettings.phys_maxstep - 1f32 {
                        if crate::src::botlib::be_aas_bspq3::AAS_PointContents(end.as_mut_ptr())
                            & 32
                            == 0
                        {
                            (*move_0).endpos[0] = lastorg[0];
                            (*move_0).endpos[1] = lastorg[1];
                            (*move_0).endpos[2] = lastorg[2];
                            (*move_0).endarea = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                lastorg.as_mut_ptr(),
                            );
                            (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
                            (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
                            (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 64;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0;
                            (*move_0).time = n as f32 * frametime;
                            (*move_0).frames = n;
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        }
                        //end if
                    }
                    //end if
                }
            }
        }
        n += 1
    }
    //
    (*move_0).endpos[0] = org[0];
    (*move_0).endpos[1] = org[1];
    (*move_0).endpos[2] = org[2];
    (*move_0).endarea = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(org.as_mut_ptr());
    (*move_0).velocity[0] = frame_test_vel[0] * (1f32 / frametime);
    (*move_0).velocity[1] = frame_test_vel[1] * (1f32 / frametime);
    (*move_0).velocity[2] = frame_test_vel[2] * (1f32 / frametime);
    (*move_0).stopevent = 0;
    (*move_0).presencetype = presencetype;
    (*move_0).endcontents = 0;
    (*move_0).time = n as f32 * frametime;
    (*move_0).frames = n;
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function AAS_ClientMovementPrediction
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PredictClientMovement(
    mut move_0: *mut crate::be_aas_h::aas_clientmove_s,
    mut entnum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: i32,
    mut onground: i32,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdmove: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdframes: i32,
    mut maxframes: i32,
    mut frametime: f32,
    mut stopevent: i32,
    mut stopareanum: i32,
    mut visualize: i32,
) -> i32 {
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    return AAS_ClientMovementPrediction(
        move_0,
        entnum,
        origin,
        presencetype,
        onground,
        velocity,
        cmdmove,
        cmdframes,
        maxframes,
        frametime,
        stopevent,
        stopareanum,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        visualize,
    );
}
//end of the function AAS_PredictClientMovement
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClientMovementHitBBox(
    mut move_0: *mut crate::be_aas_h::aas_clientmove_s,
    mut entnum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut presencetype: i32,
    mut onground: i32,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdmove: *mut crate::src::qcommon::q_shared::vec_t,
    mut cmdframes: i32,
    mut maxframes: i32,
    mut frametime: f32,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut visualize: i32,
) -> i32 {
    return AAS_ClientMovementPrediction(
        move_0,
        entnum,
        origin,
        presencetype,
        onground,
        velocity,
        cmdmove,
        cmdframes,
        maxframes,
        frametime,
        2048,
        0,
        mins,
        maxs,
        visualize,
    );
}
//end of the function AAS_ClientMovementHitBBox
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TestMovementPrediction(
    mut entnum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //SE_LEAVEGROUND);
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    velocity[2] = 0f32;
    velocity[1] = velocity[2];
    velocity[0] = velocity[1];
    if AAS_Swimming(origin) == 0 {
        *dir.offset(2) = 0f32
    }
    crate::src::qcommon::q_math::VectorNormalize(dir);
    cmdmove[0] = *dir.offset(0) * 400f32;
    cmdmove[1] = *dir.offset(1) * 400f32;
    cmdmove[2] = *dir.offset(2) * 400f32;
    cmdmove[2] = 224f32;
    crate::src::botlib::be_aas_debug::AAS_ClearShownDebugLines();
    AAS_PredictClientMovement(
        &mut move_0,
        entnum,
        origin,
        2,
        crate::src::qcommon::q_shared::qtrue as i32,
        velocity.as_mut_ptr(),
        cmdmove.as_mut_ptr(),
        13,
        13,
        0.1,
        1,
        0,
        crate::src::qcommon::q_shared::qtrue as i32,
    );
    if move_0.stopevent & 2 != 0 {
        botimport.Print.expect("non-null function pointer")(
            1i32,
            b"leave ground\n\x00" as *const u8 as *mut i8,
        );
    };
    //end if
}
//end of the function TestMovementPrediction
//===========================================================================
// calculates the horizontal velocity needed to perform a jump from start
// to end
//
// Parameter:			zvel	: z velocity for jump
//						start	: start position of jump
//						end		: end position of jump
//						*speed	: returned speed for jump
// Returns:				qfalse if too high or too far from start to end
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_HorizontalVelocityForJump(
    mut zvel: f32,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut velocity: *mut f32,
) -> i32 {
    let mut phys_gravity: f32 = 0.;
    let mut phys_maxvelocity: f32 = 0.;
    let mut maxjump: f32 = 0.;
    let mut height2fall: f32 = 0.;
    let mut t: f32 = 0.;
    let mut top: f32 = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    phys_gravity = aassettings.phys_gravity;
    phys_maxvelocity = aassettings.phys_maxvelocity;
    //maximum height a player can jump with the given initial z velocity
    maxjump =
        (0.5 * phys_gravity as f64 * (zvel / phys_gravity) as f64 * (zvel / phys_gravity) as f64)
            as f32;
    //top of the parabolic jump
    top = *start.offset(2) + maxjump;
    //height the bot will fall from the top
    height2fall = top - *end.offset(2);
    //if the goal is to high to jump to
    if height2fall < 0f32 {
        *velocity = phys_maxvelocity; //end if
        return 0i32;
    }
    //time a player takes to fall the height
    t = crate::stdlib::sqrt(height2fall as f64 / (0.5 * phys_gravity as f64)) as f32;
    //direction from start to end
    dir[0] = *end.offset(0) - *start.offset(0);
    dir[1] = *end.offset(1) - *start.offset(1);
    dir[2] = *end.offset(2) - *start.offset(2);
    //
    if t + zvel / phys_gravity == 0.0 {
        *velocity = phys_maxvelocity;
        return 0i32;
    }
    //calculate horizontal speed
    *velocity = (crate::stdlib::sqrt((dir[0] * dir[0] + dir[1] * dir[1]) as f64)
        / (t + zvel / phys_gravity) as f64) as f32;
    //the horizontal speed must be lower than the max speed
    if *velocity > phys_maxvelocity {
        *velocity = phys_maxvelocity; //end if
        return 0i32;
    }
    return 1;
}
//end of the function AAS_HorizontalVelocityForJump
