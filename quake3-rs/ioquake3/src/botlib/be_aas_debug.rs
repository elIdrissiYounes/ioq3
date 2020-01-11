use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0) = *v1.offset(1) * *v2.offset(2) - *v1.offset(2) * *v2.offset(1);
        *cross.offset(1) = *v1.offset(2) * *v2.offset(0) - *v1.offset(0) * *v2.offset(2);
        *cross.offset(2) = *v1.offset(0) * *v2.offset(1) - *v1.offset(1) * *v2.offset(0);
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
pub use crate::src::botlib::be_aas_debug::q_shared_h::CrossProduct;
use crate::src::botlib::be_interface::botimport;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedMemory;
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
use crate::stdlib::abs;
use crate::stdlib::memcpy;

use crate::src::botlib::be_aas_main::aasworld;
use crate::src::botlib::be_aas_main::AAS_Time;
use crate::src::botlib::be_aas_move::aassettings;
use crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump;
use crate::src::botlib::be_aas_move::AAS_JumpReachRunStart;
use crate::src::botlib::be_aas_move::AAS_PredictClientMovement;
use crate::src::botlib::be_aas_move::AAS_RocketJumpZVelocity;
use crate::src::botlib::be_aas_sample::AAS_AreaCluster;
use crate::src::botlib::be_aas_sample::AAS_PointAreaNum;
#[no_mangle]

pub static mut debuglines: [i32; 1024] = [0; 1024];
#[no_mangle]

pub static mut debuglinevisible: [i32; 1024] = [0; 1024];
#[no_mangle]

pub static mut numdebuglines: i32 = 0;

static mut debugpolygons: [i32; 8192] = [0; 8192];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClearShownPolygons() {
    let mut i: i32 = 0;
    //*
    i = 0;
    while i < 8192 {
        if debugpolygons[i as usize] != 0 {
            crate::src::botlib::be_interface::botimport
                .DebugPolygonDelete
                .expect("non-null function pointer")(debugpolygons[i as usize]);
        }
        debugpolygons[i as usize] = 0;
        i += 1
    }
    //end for
    //*/
    /*
        for (i = 0; i < MAX_DEBUGPOLYGONS; i++)
        {
            botimport.DebugPolygonDelete(i);
            debugpolygons[i] = 0;
        } //end for
    */
}
//end of the function AAS_ClearShownPolygons
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowPolygon(
    mut color: i32,
    mut numpoints: i32,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 8192 {
        if debugpolygons[i as usize] == 0 {
            debugpolygons[i as usize] =
                crate::src::botlib::be_interface::botimport
                    .DebugPolygonCreate
                    .expect("non-null function pointer")(color, numpoints, points);
            break;
        } else {
            i += 1
        }
        //end if
    }
    //end for
}
//end of the function AAS_ShowPolygon
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClearShownDebugLines() {
    let mut i: i32 = 0;
    //make all lines invisible
    i = 0;
    while i < 1024 {
        if debuglines[i as usize] != 0 {
            //botimport.DebugLineShow(debuglines[i], NULL, NULL, LINECOLOR_NONE);
            crate::src::botlib::be_interface::botimport
                .DebugLineDelete
                .expect("non-null function pointer")(debuglines[i as usize]);
            debuglines[i as usize] = 0;
            debuglinevisible[i as usize] = crate::src::qcommon::q_shared::qfalse as i32
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function AAS_ClearShownDebugLines
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DebugLine(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: i32,
) {
    let mut line: i32 = 0; //end if
    line = 0;
    while line < 1024 {
        if debuglines[line as usize] == 0 {
            debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                .DebugLineCreate
                .expect("non-null function pointer")();
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qfalse as i32;
            numdebuglines += 1
        }
        if debuglinevisible[line as usize] == 0 {
            crate::src::botlib::be_interface::botimport
                .DebugLineShow
                .expect("non-null function pointer")(
                debuglines[line as usize], start, end, color
            );
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as i32;
            return;
        }
        line += 1
        //end else
    }
    //end for
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
 * name:		be_aas_debug.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_debug.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_debug.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_debug.h $
 *
 *****************************************************************************/
//clear the shown debug lines
//clear the shown debug lines
//
//
//show a debug line
//show a debug line
//show a permenent line
//show a permenent line
//end of the function AAS_DebugLine
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PermanentLine(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: i32,
) {
    let mut line: i32 = 0;
    line = crate::src::botlib::be_interface::botimport
        .DebugLineCreate
        .expect("non-null function pointer")();
    crate::src::botlib::be_interface::botimport
        .DebugLineShow
        .expect("non-null function pointer")(line, start, end, color);
}
//end of the function AAS_PermenentLine
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawPermanentCross(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut size: f32,
    mut color: i32,
) {
    let mut i: i32 = 0;
    let mut debugline: i32 = 0;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    i = 0;
    while i < 3 {
        start[0] = *origin.offset(0);
        start[1] = *origin.offset(1);
        start[2] = *origin.offset(2);
        start[i as usize] += size;
        end[0] = *origin.offset(0);
        end[1] = *origin.offset(1);
        end[2] = *origin.offset(2);
        end[i as usize] -= size;
        AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), color);
        debugline = crate::src::botlib::be_interface::botimport
            .DebugLineCreate
            .expect("non-null function pointer")();
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            debugline,
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            color,
        );
        i += 1
    }
    //end for
}
//end of the function AAS_DrawPermanentCross
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawPlaneCross(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: f32,
    mut type_0: i32,
    mut color: i32,
) {
    let mut n0: i32 = 0;
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut j: i32 = 0;
    let mut line: i32 = 0;
    let mut lines: [i32; 2] = [0; 2];
    let mut start1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //make a cross in the hit plane at the hit point
    start1[0] = *point.offset(0); //end for
    start1[1] = *point.offset(1); //end if
    start1[2] = *point.offset(2);
    end1[0] = *point.offset(0);
    end1[1] = *point.offset(1);
    end1[2] = *point.offset(2);
    start2[0] = *point.offset(0);
    start2[1] = *point.offset(1);
    start2[2] = *point.offset(2);
    end2[0] = *point.offset(0);
    end2[1] = *point.offset(1);
    end2[2] = *point.offset(2);
    n0 = type_0 % 3;
    n1 = (type_0 + 1) % 3;
    n2 = (type_0 + 2) % 3;
    start1[n1 as usize] -= 6f32;
    start1[n2 as usize] -= 6f32;
    end1[n1 as usize] += 6f32;
    end1[n2 as usize] += 6f32;
    start2[n1 as usize] += 6f32;
    start2[n2 as usize] -= 6f32;
    end2[n1 as usize] -= 6f32;
    end2[n2 as usize] += 6f32;
    start1[n0 as usize] = (dist
        - (start1[n1 as usize] * *normal.offset(n1 as isize)
            + start1[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    end1[n0 as usize] = (dist
        - (end1[n1 as usize] * *normal.offset(n1 as isize)
            + end1[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    start2[n0 as usize] = (dist
        - (start2[n1 as usize] * *normal.offset(n1 as isize)
            + start2[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    end2[n0 as usize] = (dist
        - (end2[n1 as usize] * *normal.offset(n1 as isize)
            + end2[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    j = 0;
    line = 0;
    while j < 2 && line < 1024 {
        if debuglines[line as usize] == 0 {
            debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                .DebugLineCreate
                .expect("non-null function pointer")();
            let fresh0 = j;
            j = j + 1;
            lines[fresh0 as usize] = debuglines[line as usize];
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as i32;
            numdebuglines += 1
        } else if debuglinevisible[line as usize] == 0 {
            let fresh1 = j;
            j = j + 1;
            lines[fresh1 as usize] = debuglines[line as usize];
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as i32
        }
        line += 1
        //end else
    }
    crate::src::botlib::be_interface::botimport
        .DebugLineShow
        .expect("non-null function pointer")(
        lines[0],
        start1.as_mut_ptr(),
        end1.as_mut_ptr(),
        color,
    );
    crate::src::botlib::be_interface::botimport
        .DebugLineShow
        .expect("non-null function pointer")(
        lines[1],
        start2.as_mut_ptr(),
        end2.as_mut_ptr(),
        color,
    );
}
//end of the function AAS_DrawPlaneCross
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowBoundingBox(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut bboxcorners: [crate::src::qcommon::q_shared::vec3_t; 8] = [[0.; 3]; 8];
    let mut lines: [i32; 3] = [0; 3];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut line: i32 = 0;
    //upper corners
    bboxcorners[0][0] = *origin.offset(0) + *maxs.offset(0);
    bboxcorners[0][1] = *origin.offset(1) + *maxs.offset(1);
    bboxcorners[0][2] = *origin.offset(2) + *maxs.offset(2);
    //
    bboxcorners[1][0] = *origin.offset(0) + *mins.offset(0);
    bboxcorners[1][1] = *origin.offset(1) + *maxs.offset(1);
    bboxcorners[1][2] = *origin.offset(2) + *maxs.offset(2);
    //
    bboxcorners[2][0] = *origin.offset(0) + *mins.offset(0);
    bboxcorners[2][1] = *origin.offset(1) + *mins.offset(1);
    bboxcorners[2][2] = *origin.offset(2) + *maxs.offset(2);
    //
    bboxcorners[3][0] = *origin.offset(0) + *maxs.offset(0);
    bboxcorners[3][1] = *origin.offset(1) + *mins.offset(1);
    bboxcorners[3][2] = *origin.offset(2) + *maxs.offset(2);
    //lower corners
    crate::stdlib::memcpy(
        bboxcorners[4].as_mut_ptr() as *mut libc::c_void,
        bboxcorners[0].as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<crate::src::qcommon::q_shared::vec3_t>()).wrapping_mul(4usize),
    );
    i = 0;
    while i < 4 {
        bboxcorners[(4 + i) as usize][2] = *origin.offset(2) + *mins.offset(2);
        i += 1
    }
    //draw bounding box
    i = 0; //end for
    while i < 4 {
        j = 0; //end if
        line = 0;
        while j < 3 && line < 1024 {
            if debuglines[line as usize] == 0 {
                debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                    .DebugLineCreate
                    .expect("non-null function pointer")(
                );
                let fresh2 = j;
                j = j + 1;
                lines[fresh2 as usize] = debuglines[line as usize];
                debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as i32;
                numdebuglines += 1
            } else if debuglinevisible[line as usize] == 0 {
                let fresh3 = j;
                j = j + 1;
                lines[fresh3 as usize] = debuglines[line as usize];
                debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as i32
            }
            line += 1
            //end else
        }
        //top plane
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            lines[0],
            bboxcorners[i as usize].as_mut_ptr(),
            bboxcorners[(i + 1 & 3) as usize].as_mut_ptr(),
            1,
        );
        //bottom plane
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            lines[1],
            bboxcorners[(4 + i) as usize].as_mut_ptr(),
            bboxcorners[(4 + (i + 1 & 3)) as usize].as_mut_ptr(),
            1,
        );
        //vertical lines
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            lines[2],
            bboxcorners[i as usize].as_mut_ptr(),
            bboxcorners[(4 + i) as usize].as_mut_ptr(),
            1,
        );
        i += 1
    }
    //end for
}
//end of the function AAS_ShowBoundingBox
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowFace(mut facenum: i32) {
    let mut i: i32 = 0;
    let mut color: i32 = 0;
    let mut edgenum: i32 = 0;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    color = 4;
    //check if face number is in range
    if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3i32,
            b"facenum %d out of range\n\x00" as *const u8 as *mut i8,
            facenum,
        ); //end if
    }
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;

    for i in 0..(*face).numedges {
        //edge number
        edgenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .edgeindex
                .offset(((*face).firstedge + i) as isize),
        );
        //check if edge number is in range
        if edgenum >= crate::src::botlib::be_aas_main::aasworld.numedges {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3i32,
                b"edgenum %d out of range\n\x00" as *const u8 as *mut i8,
                edgenum,
            ); //end if
        }

        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(edgenum as isize) as *mut crate::aasfile_h::aas_edge_t;

        if color == 1 {
            color = 2
        } else if color == 2 {
            color = 3
        } else if color == 3 {
            color = 4
        } else {
            color = 1
        }

        AAS_DebugLine(
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0] as isize))
            .as_mut_ptr(),
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1] as isize))
            .as_mut_ptr(),
            color,
        );
    }
    plane = &mut *crate::src::botlib::be_aas_main::aasworld
        .planes
        .offset((*face).planenum as isize) as *mut crate::aasfile_h::aas_plane_t;
    edgenum = crate::stdlib::abs(
        *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset((*face).firstedge as isize),
    );
    edge = &mut *crate::src::botlib::be_aas_main::aasworld
        .edges
        .offset(edgenum as isize) as *mut crate::aasfile_h::aas_edge_t;
    start[0] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0] as isize))[0];
    start[1] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0] as isize))[1];
    start[2] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0] as isize))[2];
    end[0] = start[0] + (*plane).normal[0] * 20f32;
    end[1] = start[1] + (*plane).normal[1] * 20f32;
    end[2] = start[2] + (*plane).normal[2] * 20f32;
    AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), 1);
}
//end of the function AAS_ShowFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowFacePolygon(mut facenum: i32, mut color: i32, mut flip: i32) {
    let mut i: i32 = 0;
    let mut edgenum: i32 = 0;
    let mut numpoints: i32 = 0;
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    //check if face number is in range
    if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3i32,
            b"facenum %d out of range\n\x00" as *const u8 as *mut i8,
            facenum,
        ); //end if
    }
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
    //walk through the edges of the face
    numpoints = 0; //end else
    if flip != 0 {
        i = (*face).numedges - 1;
        while i >= 0 {
            //edge number
            edgenum = *crate::src::botlib::be_aas_main::aasworld
                .edgeindex
                .offset(((*face).firstedge + i) as isize);
            edge = &mut *crate::src::botlib::be_aas_main::aasworld
                .edges
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: i32) -> i32)(edgenum) as isize,
                ) as *mut crate::aasfile_h::aas_edge_t;
            points[numpoints as usize][0] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[(edgenum < 0) as i32 as usize] as isize))[0];
            points[numpoints as usize][1] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[(edgenum < 0) as i32 as usize] as isize))[1];
            points[numpoints as usize][2] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[(edgenum < 0) as i32 as usize] as isize))[2];
            numpoints += 1;
            i -= 1
        }
    //end for
    } else {
        i = 0;
        while i < (*face).numedges {
            //edge number
            edgenum = *crate::src::botlib::be_aas_main::aasworld
                .edgeindex
                .offset(((*face).firstedge + i) as isize);
            edge = &mut *crate::src::botlib::be_aas_main::aasworld
                .edges
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: i32) -> i32)(edgenum) as isize,
                ) as *mut crate::aasfile_h::aas_edge_t;
            points[numpoints as usize][0] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[(edgenum < 0) as i32 as usize] as isize))[0];
            points[numpoints as usize][1] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[(edgenum < 0) as i32 as usize] as isize))[1];
            points[numpoints as usize][2] = (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[(edgenum < 0) as i32 as usize] as isize))[2];
            numpoints += 1;
            i += 1
        }
        //end for
    }
    AAS_ShowPolygon(color, numpoints, points.as_mut_ptr());
}
//end of the function AAS_ShowFacePolygon
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowArea(mut areanum: i32, mut groundfacesonly: i32) {
    let mut areaedges: [i32; 1024] = [0; 1024];
    let mut numareaedges: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut color: i32 = 0;
    let mut line: i32 = 0;
    let mut facenum: i32 = 0;
    let mut edgenum: i32 = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    //
    numareaedges = 0;
    //
    if areanum < 0 || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"area %d out of range [0, %d]\n\x00" as *const u8 as *mut i8,
            areanum,
            crate::src::botlib::be_aas_main::aasworld.numareas,
        ); //end if
        return;
    }
    //pointer to the convex area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    let mut current_block_23: u64;

    for i in 0..(*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        //end for
        //AAS_ShowFace(facenum);
        //check if face number is in range
        if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3i32,
                b"facenum %d out of range\n\x00" as *const u8 as *mut i8,
                facenum,
            ); //end if
        }

        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        //ground faces only
        if groundfacesonly != 0 {
            if (*face).faceflags & (4 | 2) == 0 {
                current_block_23 = 3276175668257526147; //end if
            } else {
                current_block_23 = 15652330335145281839;
            }
        } else {
            current_block_23 = 15652330335145281839;
        }

        match current_block_23 {
            15652330335145281839 => {
                //walk through the edges of the face
                j = 0;
                while j < (*face).numedges {
                    //edge number
                    edgenum = crate::stdlib::abs(
                        *crate::src::botlib::be_aas_main::aasworld
                            .edgeindex
                            .offset(((*face).firstedge + j) as isize),
                    );
                    //end if
                    if edgenum >= crate::src::botlib::be_aas_main::aasworld.numedges {
                        crate::src::botlib::be_interface::botimport
                            .Print
                            .expect("non-null function pointer")(
                            3i32,
                            b"edgenum %d out of range\n\x00" as *const u8 as *mut i8,
                            edgenum,
                        );
                    }
                    n = 0;
                    while n < numareaedges {
                        if areaedges[n as usize] == edgenum {
                            break;
                        }
                        n += 1
                    }
                    if n == numareaedges && numareaedges < 1024 {
                        let fresh4 = numareaedges;
                        numareaedges = numareaedges + 1;
                        areaedges[fresh4 as usize] = edgenum
                    }
                    j += 1
                }
            }
            _ => {}
        }
    }
    //check if edge number is in range
    //end if
    //check if the edge is stored already
    //end for
    //draw all the edges
    n = 0; //end for
    while n < numareaedges {
        line = 0; //end if
        while line < 1024 {
            if debuglines[line as usize] == 0 {
                debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                    .DebugLineCreate
                    .expect("non-null function pointer")(
                );
                debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qfalse as i32;
                numdebuglines += 1
            }
            if debuglinevisible[line as usize] == 0 {
                break;
            }
            line += 1
            //end else
        }
        if line >= 1024 {
            return;
        }
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(*areaedges.as_mut_ptr().offset(n as isize) as isize)
            as *mut crate::aasfile_h::aas_edge_t;
        if color == 1 {
            color = 3
        } else if color == 3 {
            color = 2
        } else if color == 2 {
            color = 4
        } else {
            color = 1
        }
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            debuglines[line as usize],
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0] as isize))
            .as_mut_ptr(),
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1] as isize))
            .as_mut_ptr(),
            color,
        );
        debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as i32;
        n += 1
    }
    //end for*/
}
//show a permanent cross
//show a permanent cross
//draw a cross in the plane
//draw a cross in the plane
//show a bounding box
//show a bounding box
//show a face
//show a face
//show an area
//show an area
//
//
//end of the function AAS_ShowArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowAreaPolygons(
    mut areanum: i32,
    mut color: i32,
    mut groundfacesonly: i32,
) {
    let mut i: i32 = 0;
    let mut facenum: i32 = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    //
    if areanum < 0 || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"area %d out of range [0, %d]\n\x00" as *const u8 as *mut i8,
            areanum,
            crate::src::botlib::be_aas_main::aasworld.numareas,
        ); //end if
        return;
    }
    //pointer to the convex area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    let mut current_block_11: u64;
    //walk through the faces of the area
    i = 0;
    while i < (*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        //check if face number is in range
        if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3i32,
                b"facenum %d out of range\n\x00" as *const u8 as *mut i8,
                facenum,
            ); //end if
        }
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        //ground faces only
        if groundfacesonly != 0 {
            if (*face).faceflags & (4 | 2) == 0 {
                current_block_11 = 2473556513754201174; //end if
            } else {
                current_block_11 = 8236137900636309791;
            }
        } else {
            current_block_11 = 8236137900636309791;
        }
        match current_block_11 {
            8236137900636309791 => {
                AAS_ShowFacePolygon(facenum, color, ((*face).frontarea != areanum) as i32);
            }
            _ => {}
        }
        i += 1
    }
    //end for
}
//draw a cros
//draw a cros
//end of the function AAS_ShowAreaPolygons
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawCross(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut size: f32,
    mut color: i32,
) {
    let mut i: i32 = 0;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    i = 0;
    while i < 3 {
        start[0] = *origin.offset(0);
        start[1] = *origin.offset(1);
        start[2] = *origin.offset(2);
        start[i as usize] += size;
        end[0] = *origin.offset(0);
        end[1] = *origin.offset(1);
        end[2] = *origin.offset(2);
        end[i as usize] -= size;
        AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), color);
        i += 1
    }
    //end for
}
//print the travel type
//print the travel type
//end of the function AAS_DrawCross
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PrintTravelType(mut traveltype: i32) {}
//draw an arrow
//draw an arrow
//end of the function AAS_PrintTravelType
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawArrow(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut linecolor: i32,
    mut arrowcolor: i32,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cross: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut dot: f32 = 0.;
    dir[0] = *end.offset(0) - *start.offset(0);
    dir[1] = *end.offset(1) - *start.offset(1);
    dir[2] = *end.offset(2) - *start.offset(2);
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    dot = dir[0] * up[0] + dir[1] * up[1] + dir[2] * up[2];
    if dot as f64 > 0.99 || (dot as f64) < -0.99 {
        cross[0] = 1f32;
        cross[1] = 0f32;
        cross[2] = 0f32
    } else {
        CrossProduct(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            cross.as_mut_ptr(),
        );
    }
    p1[0] = *end.offset(0) + dir[0] * -6f32;
    p1[1] = *end.offset(1) + dir[1] * -6f32;
    p1[2] = *end.offset(2) + dir[2] * -6f32;
    p2[0] = p1[0];
    p2[1] = p1[1];
    p2[2] = p1[2];
    p1[0] = p1[0] + cross[0] * 6f32;
    p1[1] = p1[1] + cross[1] * 6f32;
    p1[2] = p1[2] + cross[2] * 6f32;
    p2[0] = p2[0] + cross[0] * -6f32;
    p2[1] = p2[1] + cross[1] * -6f32;
    p2[2] = p2[2] + cross[2] * -6f32;
    AAS_DebugLine(start, end, linecolor);
    AAS_DebugLine(p1.as_mut_ptr(), end, arrowcolor);
    AAS_DebugLine(p2.as_mut_ptr(), end, arrowcolor);
}
//visualize the given reachability
//visualize the given reachability
//end of the function AAS_DrawArrow
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowReachability(
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut speed: f32 = 0.;
    let mut zvel: f32 = 0.;
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
    AAS_ShowAreaPolygons(
        (*reach).areanum,
        5,
        crate::src::qcommon::q_shared::qtrue as i32,
    );
    //AAS_ShowArea(reach->areanum, qtrue);
    AAS_DrawArrow((*reach).start.as_mut_ptr(), (*reach).end.as_mut_ptr(), 3, 4);
    //
    if (*reach).traveltype & 0xffffff == 5 || (*reach).traveltype & 0xffffff == 7 {
        crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
            crate::src::botlib::be_aas_move::aassettings.phys_jumpvel,
            (*reach).start.as_mut_ptr(),
            (*reach).end.as_mut_ptr(),
            &mut speed,
        );
        //end if
        dir[0] = (*reach).end[0] - (*reach).start[0];
        dir[1] = (*reach).end[1] - (*reach).start[1];
        dir[2] = (*reach).end[2] - (*reach).start[2];
        dir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        velocity[0] = dir[0] * speed;
        velocity[1] = dir[1] * speed;
        velocity[2] = dir[2] * speed;
        cmdmove[2] = 0f32;
        cmdmove[1] = cmdmove[2];
        cmdmove[0] = cmdmove[1];
        cmdmove[2] = crate::src::botlib::be_aas_move::aassettings.phys_jumpvel;
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            -(1),
            (*reach).start.as_mut_ptr(),
            2,
            crate::src::qcommon::q_shared::qtrue as i32,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            3,
            30,
            0.1,
            1 | 4 | 8 | 16 | 32,
            0,
            crate::src::qcommon::q_shared::qtrue as i32,
        );
        if (*reach).traveltype & 0xffffff == 5 {
            crate::src::botlib::be_aas_move::AAS_JumpReachRunStart(reach, dir.as_mut_ptr());
            AAS_DrawCross(dir.as_mut_ptr(), 4f32, 3i32);
        }
    } else if (*reach).traveltype & 0xffffff == 12 {
        //
        //set the velocity
        //set the command movement
        //
        //
        zvel =
            crate::src::botlib::be_aas_move::AAS_RocketJumpZVelocity((*reach).start.as_mut_ptr()); //end else if
        crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
            zvel,
            (*reach).start.as_mut_ptr(),
            (*reach).end.as_mut_ptr(),
            &mut speed,
        );
        //
        dir[0] = (*reach).end[0] - (*reach).start[0];
        dir[1] = (*reach).end[1] - (*reach).start[1];
        dir[2] = (*reach).end[2] - (*reach).start[2];
        dir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        //get command movement
        cmdmove[0] = dir[0] * speed;
        cmdmove[1] = dir[1] * speed;
        cmdmove[2] = dir[2] * speed;
        velocity[0] = 0f32;
        velocity[1] = 0f32;
        velocity[2] = zvel;
        //
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            -(1i32),
            (*reach).start.as_mut_ptr(),
            2i32,
            crate::src::qcommon::q_shared::qtrue as i32,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            30i32,
            30i32,
            0.1f32,
            4i32 | 8i32 | 16i32 | 32i32 | 128i32 | 1024i32,
            (*reach).areanum,
            crate::src::qcommon::q_shared::qtrue as i32,
        );
    } else if (*reach).traveltype & 0xffffff == 18 {
        cmdmove[0] = 0f32;
        cmdmove[1] = 0f32;
        cmdmove[2] = 0f32;
        //
        dir[0] = (*reach).end[0] - (*reach).start[0];
        dir[1] = (*reach).end[1] - (*reach).start[1];
        dir[2] = (*reach).end[2] - (*reach).start[2];
        dir[2] = 0f32;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        //set the velocity
        //NOTE: the edgenum is the horizontal velocity
        velocity[0] = dir[0] * (*reach).edgenum as f32;
        velocity[1] = dir[1] * (*reach).edgenum as f32;
        velocity[2] = dir[2] * (*reach).edgenum as f32;
        //NOTE: the facenum is the Z velocity
        velocity[2] = (*reach).facenum as crate::src::qcommon::q_shared::vec_t;
        //
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            -(1i32),
            (*reach).start.as_mut_ptr(),
            2i32,
            crate::src::qcommon::q_shared::qtrue as i32,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            30i32,
            30i32,
            0.1f32,
            4i32 | 8i32 | 16i32 | 32i32 | 128i32 | 1024i32,
            (*reach).areanum,
            crate::src::qcommon::q_shared::qtrue as i32,
        );
    };
    //end else if
}
//show the reachable areas from the given area
//show the reachable areas from the given area
//end of the function AAS_ShowReachability
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowReachableAreas(mut areanum: i32) {
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t; //end if
    static mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    static mut index: i32 = 0;
    static mut lastareanum: i32 = 0;
    static mut lasttime: f32 = 0.;
    if areanum != lastareanum {
        index = 0;
        lastareanum = areanum
    }
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_areasettings_t;
    //
    if (*settings).numreachableareas == 0 {
        return;
    }
    //
    if index >= (*settings).numreachableareas {
        index = 0
    }
    //
    if (crate::src::botlib::be_aas_main::AAS_Time() - lasttime) as f64 > 1.5 {
        crate::stdlib::memcpy(
            &mut reach as *mut crate::aasfile_h::aas_reachability_t as *mut libc::c_void,
            &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(((*settings).firstreachablearea + index) as isize)
                as *mut crate::aasfile_h::aas_reachability_t as *const libc::c_void,
            ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>(),
        ); //end if
        index += 1;
        lasttime = crate::src::botlib::be_aas_main::AAS_Time();
        AAS_PrintTravelType(reach.traveltype & 0xffffff);
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(1i32, b"\n\x00" as *const u8 as *mut i8);
    }
    AAS_ShowReachability(&mut reach);
}
//end of the function ShowReachableAreas
#[no_mangle]

pub unsafe extern "C" fn AAS_FloodAreas_r(mut areanum: i32, mut cluster: i32, mut done: *mut i32) {
    let mut nextareanum: i32 = 0;
    let mut i: i32 = 0;
    let mut facenum: i32 = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    AAS_ShowAreaPolygons(areanum, 1, crate::src::qcommon::q_shared::qtrue as i32);
    //pointer to the convex area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_areasettings_t;
    //walk through the faces of the area
    i = 0; //end for
    while i < (*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        if (*face).frontarea == areanum {
            nextareanum = (*face).backarea
        } else {
            nextareanum = (*face).frontarea
        }
        if !(nextareanum == 0) {
            if !(*done.offset(nextareanum as isize) != 0) {
                *done.offset(nextareanum as isize) = crate::src::qcommon::q_shared::qtrue as i32;
                if !((*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(nextareanum as isize))
                .contents
                    & 512
                    != 0)
                {
                    if !(crate::src::botlib::be_aas_sample::AAS_AreaCluster(nextareanum) != cluster)
                    {
                        AAS_FloodAreas_r(nextareanum, cluster, done);
                    }
                }
            }
        }
        i += 1
    }
    //
    i = 0;
    while i < (*settings).numreachableareas {
        reach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(((*settings).firstreachablearea + i) as isize)
            as *mut crate::aasfile_h::aas_reachability_t;
        nextareanum = (*reach).areanum;
        if !(nextareanum == 0) {
            if !(*done.offset(nextareanum as isize) != 0) {
                *done.offset(nextareanum as isize) = crate::src::qcommon::q_shared::qtrue as i32;
                if !((*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(nextareanum as isize))
                .contents
                    & 512
                    != 0)
                {
                    if !(crate::src::botlib::be_aas_sample::AAS_AreaCluster(nextareanum) != cluster)
                    {
                        /*
                        if ((reach->traveltype & TRAVELTYPE_MASK) == TRAVEL_WALKOFFLEDGE)
                        {
                            AAS_DebugLine(reach->start, reach->end, 1);
                        }
                        */
                        AAS_FloodAreas_r(nextareanum, cluster, done);
                    }
                }
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn AAS_FloodAreas(mut origin: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut areanum: i32 = 0;
    let mut cluster: i32 = 0;
    let mut done: *mut i32 = 0 as *mut i32;
    done = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as usize)
            .wrapping_mul(::std::mem::size_of::<i32>()),
    ) as *mut i32;
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(origin);
    cluster = crate::src::botlib::be_aas_sample::AAS_AreaCluster(areanum);
    AAS_FloodAreas_r(areanum, cluster, done);
    crate::src::botlib::l_memory::FreeMemory(done as *mut libc::c_void);
}
