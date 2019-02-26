#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
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
    pub type fileHandle_t = libc::c_int;
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
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type cplane_t = cplane_s;
    use super::{libc};
}
#[header_src =
      "ioq3/code/botlib/aasfile.h"]
pub mod aasfile_h {
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
    //NOTE:	int =	default signed
//				default long
    //presence types
    //travel types
    //temporary not possible
    //walking
    //crouching
    //jumping onto a barrier
    //jumping
    //climbing a ladder
    //walking of a ledge
    //swimming
    //jump out of the water
    //teleportation
    //travel by elevator
    //rocket jumping required for travel
    //bfg jumping required for travel
    //grappling hook required for travel
    //double jump
    //ramp jump
    //strafe jump
    //jump pad
    //func bob
    //additional travel flags
    //face flags
    //just solid at the other side
    //ladder
    //standing on ground when in this face
    //gap in the ground
    //face separating two areas with liquid
    //face separating liquid and air
    //can walk over this face if bridge is closed
    //area contents
    //number of model of the mover inside this area
    //area flags
    //bot can stand on the ground
    //area contains one or more ladder faces
    //area contains a liquid
    //area is disabled for routing when set
    //area ontop of a bridge
    //aas file header lumps
    //========== bounding box =========
    //bounding box
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_bbox_s {
        pub presencetype: libc::c_int,
        pub flags: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
    }
    pub type aas_bbox_t = aas_bbox_s;
    //============ settings ===========
    //reachability to another area
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reachability_s {
        pub areanum: libc::c_int,
        pub facenum: libc::c_int,
        pub edgenum: libc::c_int,
        pub start: vec3_t,
        pub end: vec3_t,
        pub traveltype: libc::c_int,
        pub traveltime: libc::c_ushort,
    }
    pub type aas_reachability_t = aas_reachability_s;
    //area settings
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_areasettings_s {
        pub contents: libc::c_int,
        pub areaflags: libc::c_int,
        pub presencetype: libc::c_int,
        pub cluster: libc::c_int,
        pub clusterareanum: libc::c_int,
        pub numreachableareas: libc::c_int,
        pub firstreachablearea: libc::c_int,
    }
    pub type aas_areasettings_t = aas_areasettings_s;
    //cluster portal
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_portal_s {
        pub areanum: libc::c_int,
        pub frontcluster: libc::c_int,
        pub backcluster: libc::c_int,
        pub clusterareanum: [libc::c_int; 2],
    }
    pub type aas_portal_t = aas_portal_s;
    //cluster portal index
    pub type aas_portalindex_t = libc::c_int;
    //cluster
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_cluster_s {
        pub numareas: libc::c_int,
        pub numreachabilityareas: libc::c_int,
        pub numportals: libc::c_int,
        pub firstportal: libc::c_int,
    }
    pub type aas_cluster_t = aas_cluster_s;
    //============ 3d definition ============
    pub type aas_vertex_t = vec3_t;
    //just a plane in the third dimension
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_plane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: libc::c_int,
    }
    pub type aas_plane_t = aas_plane_s;
    //edge
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_edge_s {
        pub v: [libc::c_int; 2],
    }
    pub type aas_edge_t = aas_edge_s;
    //edge index, negative if vertexes are reversed
    pub type aas_edgeindex_t = libc::c_int;
    //a face bounds an area, often it will also separate two areas
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_face_s {
        pub planenum: libc::c_int,
        pub faceflags: libc::c_int,
        pub numedges: libc::c_int,
        pub firstedge: libc::c_int,
        pub frontarea: libc::c_int,
        pub backarea: libc::c_int,
    }
    pub type aas_face_t = aas_face_s;
    //face index, stores a negative index if backside of face
    pub type aas_faceindex_t = libc::c_int;
    //area with a boundary of faces
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_area_s {
        pub areanum: libc::c_int,
        pub numfaces: libc::c_int,
        pub firstface: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub center: vec3_t,
    }
    pub type aas_area_t = aas_area_s;
    //nodes of the bsp tree
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_node_s {
        pub planenum: libc::c_int,
        pub children: [libc::c_int; 2],
    }
    //when a child is zero it's a solid leaf
    pub type aas_node_t = aas_node_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
}
#[header_src =
      "ioq3/code/botlib/be_aas.h"]
pub mod be_aas_h {
    /* Defined in botlib.h

//bsp_trace_t hit surface
typedef struct bsp_surface_s
{
	char name[16];
	int flags;
	int value;
} bsp_surface_t;

//a trace is returned when a box is swept through the BSP world
typedef struct bsp_trace_s
{
	qboolean		allsolid;	// if true, plane is not valid
	qboolean		startsolid;	// if true, the initial point was in a solid area
	float			fraction;	// time completed, 1.0 = didn't hit anything
	vec3_t			endpos;		// final position
	cplane_t		plane;		// surface normal at impact
	float			exp_dist;	// expanded plane distance
	int				sidenum;	// number of the brush side hit
	bsp_surface_t	surface;	// hit surface
	int				contents;	// contents on other side of surface hit
	int				ent;		// number of entity hit
} bsp_trace_t;
//
*/
    //entity info
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_entityinfo_s {
        pub valid: libc::c_int,
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub ltime: libc::c_float,
        pub update_time: libc::c_float,
        pub number: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub lastvisorigin: vec3_t,
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
    pub type aas_entityinfo_t = aas_entityinfo_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
}
#[header_src =
      "ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
    //bsp_trace_t hit surface
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    pub type bsp_surface_t = bsp_surface_s;
    //remove the bsp_trace_s structure definition l8r on
//a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_trace_s {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    pub type bsp_trace_t = bsp_trace_s;
    //bot AI library exported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_char, ...)
                              -> ()>,
        pub Trace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: libc::c_int, _: libc::c_int)
                              -> ()>,
        pub EntityTrace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
        pub PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                      -> libc::c_int>,
        pub inPVS: Option<unsafe extern "C" fn(_: *mut vec_t, _: *mut vec_t)
                              -> libc::c_int>,
        pub BSPEntityData: Option<unsafe extern "C" fn()
                                      -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t)
                                               -> ()>,
        pub BotClientCommand: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut fileHandle_t,
                                                      _: fsMode_t)
                                     -> libc::c_int>,
        pub FS_Read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: libc::c_int,
                                                 _: fileHandle_t)
                                -> libc::c_int>,
        pub FS_Write: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                                  _: libc::c_int,
                                                  _: fileHandle_t)
                                 -> libc::c_int>,
        pub FS_FCloseFile: Option<unsafe extern "C" fn(_: fileHandle_t)
                                      -> ()>,
        pub FS_Seek: Option<unsafe extern "C" fn(_: fileHandle_t,
                                                 _: libc::c_long,
                                                 _: libc::c_int)
                                -> libc::c_int>,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
        pub DebugLineShow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: libc::c_int) -> ()>,
        pub DebugPolygonCreate: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: *mut vec3_t)
                                           -> libc::c_int>,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    }
    pub type botlib_import_t = botlib_import_s;
    use super::{libc};
    use super::q_shared_h::{qboolean, vec3_t, cplane_t, vec_t, fileHandle_t,
                            fsMode_t};
}
#[header_src =
      "ioq3/code/botlib/be_aas_def.h"]
pub mod be_aas_def_h {
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
 * name:		be_aas_def.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_def.h $
 *
 *****************************************************************************/
    //debugging on
    //structure to link entities to areas and areas to entities
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_link_s {
        pub entnum: libc::c_int,
        pub areanum: libc::c_int,
        pub next_ent: *mut aas_link_s,
        pub prev_ent: *mut aas_link_s,
        pub next_area: *mut aas_link_s,
        pub prev_area: *mut aas_link_s,
    }
    pub type aas_link_t = aas_link_s;
    //structure to link entities to leaves and leaves to entities
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_link_s {
        pub entnum: libc::c_int,
        pub leafnum: libc::c_int,
        pub next_ent: *mut bsp_link_s,
        pub prev_ent: *mut bsp_link_s,
        pub next_leaf: *mut bsp_link_s,
        pub prev_leaf: *mut bsp_link_s,
    }
    pub type bsp_link_t = bsp_link_s;
    //entity
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_entity_s {
        pub i: aas_entityinfo_t,
        pub areas: *mut aas_link_t,
        pub leaves: *mut bsp_link_t,
    }
    pub type aas_entity_t = aas_entity_s;
    //routing cache
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_routingcache_s {
        pub type_0: byte,
        pub time: libc::c_float,
        pub size: libc::c_int,
        pub cluster: libc::c_int,
        pub areanum: libc::c_int,
        pub origin: vec3_t,
        pub starttraveltime: libc::c_float,
        pub travelflags: libc::c_int,
        pub prev: *mut aas_routingcache_s,
        pub next: *mut aas_routingcache_s,
        pub time_prev: *mut aas_routingcache_s,
        pub time_next: *mut aas_routingcache_s,
        pub reachabilities: *mut libc::c_uchar,
        pub traveltimes: [libc::c_ushort; 0],
    }
    pub type aas_routingcache_t = aas_routingcache_s;
    //fields for the routing algorithm
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_routingupdate_s {
        pub cluster: libc::c_int,
        pub areanum: libc::c_int,
        pub start: vec3_t,
        pub tmptraveltime: libc::c_ushort,
        pub areatraveltimes: *mut libc::c_ushort,
        pub inlist: qboolean,
        pub next: *mut aas_routingupdate_s,
        pub prev: *mut aas_routingupdate_s,
    }
    pub type aas_routingupdate_t = aas_routingupdate_s;
    //reversed reachability link
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reversedlink_s {
        pub linknum: libc::c_int,
        pub areanum: libc::c_int,
        pub next: *mut aas_reversedlink_s,
    }
    pub type aas_reversedlink_t = aas_reversedlink_s;
    //reversed area reachability
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reversedreachability_s {
        pub numlinks: libc::c_int,
        pub first: *mut aas_reversedlink_t,
    }
    pub type aas_reversedreachability_t = aas_reversedreachability_s;
    //areas a reachability goes through
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reachabilityareas_s {
        pub firstarea: libc::c_int,
        pub numareas: libc::c_int,
    }
    pub type aas_reachabilityareas_t = aas_reachabilityareas_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_s {
        pub loaded: libc::c_int,
        pub initialized: libc::c_int,
        pub savefile: libc::c_int,
        pub bspchecksum: libc::c_int,
        pub time: libc::c_float,
        pub numframes: libc::c_int,
        pub filename: [libc::c_char; 64],
        pub mapname: [libc::c_char; 64],
        pub numbboxes: libc::c_int,
        pub bboxes: *mut aas_bbox_t,
        pub numvertexes: libc::c_int,
        pub vertexes: *mut aas_vertex_t,
        pub numplanes: libc::c_int,
        pub planes: *mut aas_plane_t,
        pub numedges: libc::c_int,
        pub edges: *mut aas_edge_t,
        pub edgeindexsize: libc::c_int,
        pub edgeindex: *mut aas_edgeindex_t,
        pub numfaces: libc::c_int,
        pub faces: *mut aas_face_t,
        pub faceindexsize: libc::c_int,
        pub faceindex: *mut aas_faceindex_t,
        pub numareas: libc::c_int,
        pub areas: *mut aas_area_t,
        pub numareasettings: libc::c_int,
        pub areasettings: *mut aas_areasettings_t,
        pub reachabilitysize: libc::c_int,
        pub reachability: *mut aas_reachability_t,
        pub numnodes: libc::c_int,
        pub nodes: *mut aas_node_t,
        pub numportals: libc::c_int,
        pub portals: *mut aas_portal_t,
        pub portalindexsize: libc::c_int,
        pub portalindex: *mut aas_portalindex_t,
        pub numclusters: libc::c_int,
        pub clusters: *mut aas_cluster_t,
        pub numreachabilityareas: libc::c_int,
        pub reachabilitytime: libc::c_float,
        pub linkheap: *mut aas_link_t,
        pub linkheapsize: libc::c_int,
        pub freelinks: *mut aas_link_t,
        pub arealinkedentities: *mut *mut aas_link_t,
        pub maxentities: libc::c_int,
        pub maxclients: libc::c_int,
        pub entities: *mut aas_entity_t,
        pub travelflagfortype: [libc::c_int; 32],
        pub areacontentstravelflags: *mut libc::c_int,
        pub areaupdate: *mut aas_routingupdate_t,
        pub portalupdate: *mut aas_routingupdate_t,
        pub frameroutingupdates: libc::c_int,
        pub reversedreachability: *mut aas_reversedreachability_t,
        pub areatraveltimes: *mut *mut *mut libc::c_ushort,
        pub clusterareacache: *mut *mut *mut aas_routingcache_t,
        pub portalcache: *mut *mut aas_routingcache_t,
        pub oldestcache: *mut aas_routingcache_t,
        pub newestcache: *mut aas_routingcache_t,
        pub portalmaxtraveltimes: *mut libc::c_int,
        pub reachabilityareaindex: *mut libc::c_int,
        pub reachabilityareas: *mut aas_reachabilityareas_t,
    }
    pub type aas_t = aas_s;
    use super::{libc};
    use super::be_aas_h::{aas_entityinfo_t};
    use super::q_shared_h::{byte, vec3_t, qboolean};
    use super::aasfile_h::{aas_bbox_t, aas_vertex_t, aas_plane_t, aas_edge_t,
                           aas_edgeindex_t, aas_face_t, aas_faceindex_t,
                           aas_area_t, aas_areasettings_t, aas_reachability_t,
                           aas_node_t, aas_portal_t, aas_portalindex_t,
                           aas_cluster_t};
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
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::{libc};
    extern "C" {
        //write to the current opened log file
        #[no_mangle]
        pub fn Log_Write(fmt: *mut libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //gets the value of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetValue(var_name: *const libc::c_char) -> libc::c_float;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_reach.h"]
pub mod be_aas_reach_h {
    use super::{libc};
    extern "C" {
        //AASINTERN
        //returns true if the are has reachabilities to other areas
        #[no_mangle]
        pub fn AAS_AreaReachability(areanum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::be_aas_def_h::{aas_t};
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
 * name:		be_aas_main.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_main.h $
 *
 *****************************************************************************/
        #[no_mangle]
        pub static mut aasworld: aas_t;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_cluster.h"]
pub mod be_aas_cluster_h { }
#[header_src =
      "ioq3/code/botlib/be_aas_cluster.c"]
pub mod be_aas_cluster_c {
    use super::botlib_h::{botlib_import_t};
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
        /* ****************************************************************************
 * name:		be_aas_cluster.c
 *
 * desc:		area clustering
 *
 * $Archive: /MissionPack/code/botlib/be_aas_cluster.c $
 *
 *****************************************************************************/
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src =
      "ioq3/code/botlib/be_variadic.h"]
pub mod be_variadic_h {
    use super::{libc};
    extern "C" {
        //AAS error message
        #[no_mangle]
        pub fn AAS_Error(fmt: *mut libc::c_char, ...);
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t};
use self::aasfile_h::{aas_bbox_s, aas_bbox_t, aas_reachability_s,
                      aas_reachability_t, aas_areasettings_s,
                      aas_areasettings_t, aas_portal_s, aas_portal_t,
                      aas_portalindex_t, aas_cluster_s, aas_cluster_t,
                      aas_vertex_t, aas_plane_s, aas_plane_t, aas_edge_s,
                      aas_edge_t, aas_edgeindex_t, aas_face_s, aas_face_t,
                      aas_faceindex_t, aas_area_s, aas_area_t, aas_node_s,
                      aas_node_t};
use self::be_aas_h::{aas_entityinfo_s, aas_entityinfo_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_aas_def_h::{aas_link_s, aas_link_t, bsp_link_s, bsp_link_t,
                         aas_entity_s, aas_entity_t, aas_routingcache_s,
                         aas_routingcache_t, aas_routingupdate_s,
                         aas_routingupdate_t, aas_reversedlink_s,
                         aas_reversedlink_t, aas_reversedreachability_s,
                         aas_reversedreachability_t, aas_reachabilityareas_s,
                         aas_reachabilityareas_t, aas_s, aas_t};
use self::string_h::{memset};
use self::stdlib_h::{abs};
use self::l_memory_h::{GetClearedMemory, FreeMemory};
use self::l_log_h::{Log_Write};
use self::l_libvar_h::{LibVarGetValue};
use self::be_aas_reach_h::{AAS_AreaReachability};
use self::be_aas_main_h::{aasworld};
use self::be_aas_cluster_c::{botimport};
use self::be_variadic_h::{AAS_Error};
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
 * name:		be_aas_cluster.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_cluster.h $
 *
 *****************************************************************************/
//initialize the AAS clustering
#[no_mangle]
pub unsafe extern "C" fn AAS_InitClustering() {
    let mut i: libc::c_int = 0;
    let mut removedPortalAreas: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut numreachabilityareas: libc::c_int = 0;
    if 0 == aasworld.loaded { return }
    if aasworld.numclusters >= 1i32 {
        if 0 ==
               LibVarGetValue(b"forceclustering\x00" as *const u8 as
                                  *const libc::c_char) as libc::c_int &&
               0 ==
                   LibVarGetValue(b"forcereachability\x00" as *const u8 as
                                      *const libc::c_char) as libc::c_int {
            return
        }
    }
    AAS_SetViewPortalsAsClusterPortals();
    AAS_CountForcedClusterPortals();
    AAS_RemoveClusterAreas();
    AAS_FindPossiblePortals();
    AAS_CreateViewPortals();
    if !aasworld.portals.is_null() {
        FreeMemory(aasworld.portals as *mut libc::c_void);
    }
    aasworld.portals =
        GetClearedMemory((65536i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_portal_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_portal_t;
    if !aasworld.portalindex.is_null() {
        FreeMemory(aasworld.portalindex as *mut libc::c_void);
    }
    aasworld.portalindex =
        GetClearedMemory((65536i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_portalindex_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_portalindex_t;
    if !aasworld.clusters.is_null() {
        FreeMemory(aasworld.clusters as *mut libc::c_void);
    }
    aasworld.clusters =
        GetClearedMemory((65536i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_cluster_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_cluster_t;
    removedPortalAreas = 0i32;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"\r%6d removed portal areas\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        removedPortalAreas);
    loop  {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"\r%6d\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            removedPortalAreas);
        aasworld.numportals = 1i32;
        aasworld.portalindexsize = 0i32;
        aasworld.numclusters = 1i32;
        AAS_CreatePortals();
        removedPortalAreas += 1;
        //find the clusters
        if 0 == AAS_FindClusters() { continue ; }
        //test the portals
        if 0 == AAS_TestPortals() { continue ; }
        //
        break ;
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"\n\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
    aasworld.savefile = qtrue as libc::c_int;
    i = 1i32;
    while i < aasworld.numportals {
        Log_Write(b"portal %d: area %d\r\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char, i,
                  (*aasworld.portals.offset(i as isize)).areanum);
        i += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%6d portals created\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        aasworld.numportals);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%6d clusters created\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        aasworld.numclusters);
    i = 1i32;
    while i < aasworld.numclusters {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"cluster %d has %d reachability areas\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            i,
                                                            (*aasworld.clusters.offset(i
                                                                                           as
                                                                                           isize)).numreachabilityareas);
        i += 1
    }
    numreachabilityareas = 0i32;
    total = 0i32;
    i = 0i32;
    while i < aasworld.numclusters {
        n = (*aasworld.clusters.offset(i as isize)).numreachabilityareas;
        numreachabilityareas += n;
        total += n * n;
        i += 1
    }
    total += numreachabilityareas * aasworld.numportals;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%6i total reachability areas\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        numreachabilityareas);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%6i AAS memory/CPU usage (the lower the better)\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        total * 3i32);
}
//end for
//end of the function AAS_RemoveAllPortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_TestPortals() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    i = 1i32;
    while i < aasworld.numportals {
        portal =
            &mut *aasworld.portals.offset(i as isize) as *mut aas_portal_t;
        if 0 == (*portal).frontcluster {
            (*aasworld.areasettings.offset((*portal).areanum as
                                               isize)).contents &= !8i32;
            Log_Write(b"portal area %d has no front cluster\r\n\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, (*portal).areanum);
            return qfalse as libc::c_int
        }
        if 0 == (*portal).backcluster {
            (*aasworld.areasettings.offset((*portal).areanum as
                                               isize)).contents &= !8i32;
            Log_Write(b"portal area %d has no back cluster\r\n\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, (*portal).areanum);
            return qfalse as libc::c_int
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//end else
//end for
//end of the function AAS_NumberClusterAreas
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FindClusters() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    AAS_RemoveClusterAreas();
    let mut current_block_16: u64;
    i = 1i32;
    while i < aasworld.numareas {
        //if the area is already part of a cluster
        if !(0 != (*aasworld.areasettings.offset(i as isize)).cluster) {
            // if not flooding through faces only use areas that have reachabilities
            if 0 != nofaceflood {
                if 0 ==
                       (*aasworld.areasettings.offset(i as
                                                          isize)).numreachableareas
                   {
                    current_block_16 = 14155750587950065367;
                } else { current_block_16 = 6937071982253665452; }
            } else { current_block_16 = 6937071982253665452; }
            match current_block_16 {
                14155750587950065367 => { }
                _ => {
                    //end if
                    //if the area is a cluster portal
                    if !(0 !=
                             (*aasworld.areasettings.offset(i as
                                                                isize)).contents
                                 & 8i32) {
                        if aasworld.numclusters >= 65536i32 {
                            AAS_Error(b"AAS_MAX_CLUSTERS\n\x00" as *const u8
                                          as *const libc::c_char as
                                          *mut libc::c_char);
                            return qfalse as libc::c_int
                        }
                        cluster =
                            &mut *aasworld.clusters.offset(aasworld.numclusters
                                                               as isize) as
                                *mut aas_cluster_t;
                        (*cluster).numareas = 0i32;
                        (*cluster).numreachabilityareas = 0i32;
                        (*cluster).firstportal = aasworld.portalindexsize;
                        (*cluster).numportals = 0i32;
                        if 0 ==
                               AAS_FloodClusterAreas_r(i,
                                                       aasworld.numclusters) {
                            return qfalse as libc::c_int
                        }
                        if 0 ==
                               AAS_FloodClusterAreasUsingReachabilities(aasworld.numclusters)
                           {
                            return qfalse as libc::c_int
                        }
                        AAS_NumberClusterAreas(aasworld.numclusters);
                        aasworld.numclusters += 1
                    }
                }
            }
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//end else
//end for
//end of the function AAS_NumberClusterPortals
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_NumberClusterAreas(mut clusternum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut portalnum: libc::c_int = 0;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    (*aasworld.clusters.offset(clusternum as isize)).numareas = 0i32;
    (*aasworld.clusters.offset(clusternum as isize)).numreachabilityareas =
        0i32;
    i = 1i32;
    while i < aasworld.numareas {
        //
        if !((*aasworld.areasettings.offset(i as isize)).cluster !=
                 clusternum) {
            //
            if !(0 == AAS_AreaReachability(i)) {
                (*aasworld.areasettings.offset(i as isize)).clusterareanum =
                    (*aasworld.clusters.offset(clusternum as isize)).numareas;
                let ref mut fresh0 =
                    (*aasworld.clusters.offset(clusternum as isize)).numareas;
                *fresh0 += 1;
                let ref mut fresh1 =
                    (*aasworld.clusters.offset(clusternum as
                                                   isize)).numreachabilityareas;
                *fresh1 += 1
            }
        }
        i += 1
    }
    cluster =
        &mut *aasworld.clusters.offset(clusternum as isize) as
            *mut aas_cluster_t;
    i = 0i32;
    while i < (*cluster).numportals {
        portalnum =
            *aasworld.portalindex.offset(((*cluster).firstportal + i) as
                                             isize);
        portal =
            &mut *aasworld.portals.offset(portalnum as isize) as
                *mut aas_portal_t;
        if !(0 == AAS_AreaReachability((*portal).areanum)) {
            if (*portal).frontcluster == clusternum {
                let fresh2 = (*cluster).numareas;
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[0usize] = fresh2;
                let ref mut fresh3 =
                    (*aasworld.clusters.offset(clusternum as
                                                   isize)).numreachabilityareas;
                *fresh3 += 1
            } else {
                let fresh4 = (*cluster).numareas;
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[1usize] = fresh4;
                let ref mut fresh5 =
                    (*aasworld.clusters.offset(clusternum as
                                                   isize)).numreachabilityareas;
                *fresh5 += 1
            }
        }
        i += 1
    }
    i = 1i32;
    while i < aasworld.numareas {
        //
        if !((*aasworld.areasettings.offset(i as isize)).cluster !=
                 clusternum) {
            //
            if !(0 != AAS_AreaReachability(i)) {
                (*aasworld.areasettings.offset(i as isize)).clusterareanum =
                    (*aasworld.clusters.offset(clusternum as isize)).numareas;
                let ref mut fresh6 =
                    (*aasworld.clusters.offset(clusternum as isize)).numareas;
                *fresh6 += 1
            }
        }
        i += 1
    }
    cluster =
        &mut *aasworld.clusters.offset(clusternum as isize) as
            *mut aas_cluster_t;
    i = 0i32;
    while i < (*cluster).numportals {
        portalnum =
            *aasworld.portalindex.offset(((*cluster).firstportal + i) as
                                             isize);
        portal =
            &mut *aasworld.portals.offset(portalnum as isize) as
                *mut aas_portal_t;
        if !(0 != AAS_AreaReachability((*portal).areanum)) {
            if (*portal).frontcluster == clusternum {
                let fresh7 = (*cluster).numareas;
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[0usize] = fresh7
            } else {
                let fresh8 = (*cluster).numareas;
                (*cluster).numareas = (*cluster).numareas + 1;
                (*portal).clusterareanum[1usize] = fresh8
            }
        }
        i += 1
    };
}
//end of the function AAS_FloodClusterAreas_r
//===========================================================================
// try to flood from all areas without cluster into areas with a cluster set
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FloodClusterAreasUsingReachabilities(mut clusternum:
                                                                      libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    i = 1i32;
    while i < aasworld.numareas {
        //if this area already has a cluster set
        if !(0 != (*aasworld.areasettings.offset(i as isize)).cluster) {
            //if this area is a cluster portal
            if !(0 !=
                     (*aasworld.areasettings.offset(i as isize)).contents &
                         8i32) {
                j = 0i32;
                while j <
                          (*aasworld.areasettings.offset(i as
                                                             isize)).numreachableareas
                      {
                    areanum =
                        (*aasworld.reachability.offset(((*aasworld.areasettings.offset(i
                                                                                           as
                                                                                           isize)).firstreachablearea
                                                            + j) as
                                                           isize)).areanum;
                    //if this area is a cluster portal
                    if !(0 !=
                             (*aasworld.areasettings.offset(areanum as
                                                                isize)).contents
                                 & 8i32) {
                        //if this area has a cluster set
                        if 0 !=
                               (*aasworld.areasettings.offset(areanum as
                                                                  isize)).cluster
                           {
                            if 0 == AAS_FloodClusterAreas_r(i, clusternum) {
                                return qfalse as libc::c_int
                            }
                            i = 0i32;
                            break ;
                        }
                    }
                    j += 1
                }
            }
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//end of the function AAS_UpdatePortal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FloodClusterAreas_r(mut areanum: libc::c_int,
                                                 mut clusternum: libc::c_int)
 -> libc::c_int {
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut facenum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        AAS_Error(b"AAS_FloodClusterAreas_r: areanum out of range\n\x00" as
                      *const u8 as *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if (*aasworld.areasettings.offset(areanum as isize)).cluster > 0i32 {
        if (*aasworld.areasettings.offset(areanum as isize)).cluster ==
               clusternum {
            return qtrue as libc::c_int
        }
        AAS_Error(b"cluster %d touched cluster %d at area %d\n\x00" as
                      *const u8 as *const libc::c_char as *mut libc::c_char,
                  clusternum,
                  (*aasworld.areasettings.offset(areanum as isize)).cluster,
                  areanum);
        return qfalse as libc::c_int
    }
    if 0 != (*aasworld.areasettings.offset(areanum as isize)).contents & 8i32
       {
        return AAS_UpdatePortal(areanum, clusternum)
    }
    (*aasworld.areasettings.offset(areanum as isize)).cluster = clusternum;
    (*aasworld.areasettings.offset(areanum as isize)).clusterareanum =
        (*aasworld.clusters.offset(clusternum as isize)).numareas;
    let ref mut fresh9 =
        (*aasworld.clusters.offset(clusternum as isize)).numareas;
    *fresh9 += 1;
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    if 0 == nofaceflood {
        i = 0i32;
        while i < (*area).numfaces {
            facenum =
                abs(*aasworld.faceindex.offset(((*area).firstface + i) as
                                                   isize));
            face =
                &mut *aasworld.faces.offset(facenum as isize) as
                    *mut aas_face_t;
            if (*face).frontarea == areanum {
                if 0 != (*face).backarea {
                    if 0 ==
                           AAS_FloodClusterAreas_r((*face).backarea,
                                                   clusternum) {
                        return qfalse as libc::c_int
                    }
                }
            } else if 0 != (*face).frontarea {
                if 0 == AAS_FloodClusterAreas_r((*face).frontarea, clusternum)
                   {
                    return qfalse as libc::c_int
                }
            }
            i += 1
        }
    }
    i = 0i32;
    while i <
              (*aasworld.areasettings.offset(areanum as
                                                 isize)).numreachableareas {
        if !(0 ==
                 (*aasworld.reachability.offset(((*aasworld.areasettings.offset(areanum
                                                                                    as
                                                                                    isize)).firstreachablearea
                                                     + i) as isize)).areanum)
           {
            if 0 ==
                   AAS_FloodClusterAreas_r((*aasworld.reachability.offset(((*aasworld.areasettings.offset(areanum
                                                                                                              as
                                                                                                              isize)).firstreachablearea
                                                                               +
                                                                               i)
                                                                              as
                                                                              isize)).areanum,
                                           clusternum) {
                return qfalse as libc::c_int
            }
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//
// do not flood through area faces, only use reachabilities
#[no_mangle]
pub static mut nofaceflood: libc::c_int = qtrue as libc::c_int;
//end if
//end for
//end of the function AAS_RemovePortalsClusterReference
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_UpdatePortal(mut areanum: libc::c_int,
                                          mut clusternum: libc::c_int)
 -> libc::c_int {
    let mut portalnum: libc::c_int = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    portalnum = 1i32;
    while portalnum < aasworld.numportals {
        if (*aasworld.portals.offset(portalnum as isize)).areanum == areanum {
            break ;
        }
        portalnum += 1
    }
    if portalnum == aasworld.numportals {
        AAS_Error(b"no portal of area %d\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char, areanum);
        return qtrue as libc::c_int
    }
    portal =
        &mut *aasworld.portals.offset(portalnum as isize) as
            *mut aas_portal_t;
    if (*portal).frontcluster == clusternum { return qtrue as libc::c_int }
    if (*portal).backcluster == clusternum { return qtrue as libc::c_int }
    if 0 == (*portal).frontcluster {
        (*portal).frontcluster = clusternum
    } else if 0 == (*portal).backcluster {
        (*portal).backcluster = clusternum
    } else {
        (*aasworld.areasettings.offset(areanum as isize)).contents &= !8i32;
        Log_Write(b"portal area %d is separating more than two clusters\r\n\x00"
                      as *const u8 as *const libc::c_char as
                      *mut libc::c_char, areanum);
        return qfalse as libc::c_int
    }
    if aasworld.portalindexsize >= 65536i32 {
        AAS_Error(b"AAS_MAX_PORTALINDEXSIZE\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char);
        return qtrue as libc::c_int
    }
    (*aasworld.areasettings.offset(areanum as isize)).cluster = -portalnum;
    cluster =
        &mut *aasworld.clusters.offset(clusternum as isize) as
            *mut aas_cluster_t;
    *aasworld.portalindex.offset(((*cluster).firstportal +
                                      (*cluster).numportals) as isize) =
        portalnum;
    aasworld.portalindexsize += 1;
    (*cluster).numportals += 1;
    return qtrue as libc::c_int;
}
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_RemoveClusterAreas() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < aasworld.numareas {
        (*aasworld.areasettings.offset(i as isize)).cluster = 0i32;
        i += 1
    };
}
//end of the function AAS_FindClusters
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_CreatePortals() {
    let mut i: libc::c_int = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    i = 1i32;
    while i < aasworld.numareas {
        if 0 != (*aasworld.areasettings.offset(i as isize)).contents & 8i32 {
            if aasworld.numportals >= 65536i32 {
                AAS_Error(b"AAS_MAX_PORTALS\n\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
                return
            }
            portal =
                &mut *aasworld.portals.offset(aasworld.numportals as isize) as
                    *mut aas_portal_t;
            (*portal).areanum = i;
            (*portal).frontcluster = 0i32;
            (*portal).backcluster = 0i32;
            aasworld.numportals += 1
        }
        i += 1
    };
}
//end of the function AAS_CountForcedClusterPortals
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_CreateViewPortals() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < aasworld.numareas {
        if 0 != (*aasworld.areasettings.offset(i as isize)).contents & 8i32 {
            (*aasworld.areasettings.offset(i as isize)).contents |= 512i32
        }
        i += 1
    };
}
//end of the function AAS_CheckAreaForPossiblePortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FindPossiblePortals() {
    let mut i: libc::c_int = 0;
    let mut numpossibleportals: libc::c_int = 0;
    numpossibleportals = 0i32;
    i = 1i32;
    while i < aasworld.numareas {
        numpossibleportals += AAS_CheckAreaForPossiblePortals(i);
        i += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"\r%6d possible portal areas\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        numpossibleportals);
}
//end of the function AAS_GetAdjacentAreasWithLessPresenceTypes_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_CheckAreaForPossiblePortals(mut areanum:
                                                             libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut fen: libc::c_int = 0;
    let mut ben: libc::c_int = 0;
    let mut frontedgenum: libc::c_int = 0;
    let mut backedgenum: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut areanums: [libc::c_int; 1024] = [0; 1024];
    let mut numareas: libc::c_int = 0;
    let mut otherareanum: libc::c_int = 0;
    let mut numareafrontfaces: [libc::c_int; 1024] = [0; 1024];
    let mut numareabackfaces: [libc::c_int; 1024] = [0; 1024];
    let mut frontfacenums: [libc::c_int; 1024] = [0; 1024];
    let mut backfacenums: [libc::c_int; 1024] = [0; 1024];
    let mut numfrontfaces: libc::c_int = 0;
    let mut numbackfaces: libc::c_int = 0;
    let mut frontareanums: [libc::c_int; 1024] = [0; 1024];
    let mut backareanums: [libc::c_int; 1024] = [0; 1024];
    let mut numfrontareas: libc::c_int = 0;
    let mut numbackareas: libc::c_int = 0;
    let mut frontplanenum: libc::c_int = 0;
    let mut backplanenum: libc::c_int = 0;
    let mut faceplanenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut frontface: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut backface: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    if 0 != (*aasworld.areasettings.offset(areanum as isize)).contents & 8i32
       {
        return 0i32
    }
    if 0 == (*aasworld.areasettings.offset(areanum as isize)).areaflags & 1i32
       {
        return 0i32
    }
    memset(numareafrontfaces.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_int; 1024]>() as libc::c_ulong);
    memset(numareabackfaces.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_int; 1024]>() as libc::c_ulong);
    numbackfaces = 0i32;
    numfrontfaces = numbackfaces;
    numbackareas = 0i32;
    numfrontareas = numbackareas;
    backplanenum = -1i32;
    frontplanenum = backplanenum;
    numareas =
        AAS_GetAdjacentAreasWithLessPresenceTypes_r(areanums.as_mut_ptr(),
                                                    0i32, areanum);
    i = 0i32;
    while i < numareas {
        area =
            &mut *aasworld.areas.offset(*areanums.as_mut_ptr().offset(i as
                                                                          isize)
                                            as isize) as *mut aas_area_t;
        j = 0i32;
        while j < (*area).numfaces {
            facenum =
                abs(*aasworld.faceindex.offset(((*area).firstface + j) as
                                                   isize));
            face =
                &mut *aasworld.faces.offset(facenum as isize) as
                    *mut aas_face_t;
            //if the face is solid
            if !(0 != (*face).faceflags & 1i32) {
                k = 0i32;
                while k < numareas {
                    if !(k == i) {
                        if (*face).frontarea == areanums[k as usize] ||
                               (*face).backarea == areanums[k as usize] {
                            break ;
                        }
                    }
                    k += 1
                }
                //end for
                //if the face is shared
                if !(k != numareas) {
                    if (*face).frontarea == areanums[i as usize] {
                        otherareanum = (*face).backarea
                    } else { otherareanum = (*face).frontarea }
                    if 0 !=
                           (*aasworld.areasettings.offset(otherareanum as
                                                              isize)).contents
                               & 8i32 {
                        return 0i32
                    }
                    faceplanenum = (*face).planenum & !1i32;
                    if frontplanenum < 0i32 || faceplanenum == frontplanenum {
                        frontplanenum = faceplanenum;
                        let fresh10 = numfrontfaces;
                        numfrontfaces = numfrontfaces + 1;
                        frontfacenums[fresh10 as usize] = facenum;
                        k = 0i32;
                        while k < numfrontareas {
                            if frontareanums[k as usize] == otherareanum {
                                break ;
                            }
                            k += 1
                        }
                        if k == numfrontareas {
                            let fresh11 = numfrontareas;
                            numfrontareas = numfrontareas + 1;
                            frontareanums[fresh11 as usize] = otherareanum
                        }
                        numareafrontfaces[i as usize] += 1
                    } else if backplanenum < 0i32 ||
                                  faceplanenum == backplanenum {
                        backplanenum = faceplanenum;
                        let fresh12 = numbackfaces;
                        numbackfaces = numbackfaces + 1;
                        backfacenums[fresh12 as usize] = facenum;
                        k = 0i32;
                        while k < numbackareas {
                            if backareanums[k as usize] == otherareanum {
                                break ;
                            }
                            k += 1
                        }
                        if k == numbackareas {
                            let fresh13 = numbackareas;
                            numbackareas = numbackareas + 1;
                            backareanums[fresh13 as usize] = otherareanum
                        }
                        numareabackfaces[i as usize] += 1
                    } else { return 0i32 }
                }
            }
            j += 1
        }
        i += 1
    }
    i = 0i32;
    while i < numareas {
        if 0 == numareafrontfaces[i as usize] ||
               0 == numareabackfaces[i as usize] {
            return 0i32
        }
        i += 1
    }
    if 0 ==
           AAS_ConnectedAreas(frontareanums.as_mut_ptr(), numfrontareas) as
               u64 {
        return 0i32
    }
    if 0 == AAS_ConnectedAreas(backareanums.as_mut_ptr(), numbackareas) as u64
       {
        return 0i32
    }
    i = 0i32;
    while i < numfrontfaces {
        frontface =
            &mut *aasworld.faces.offset(*frontfacenums.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)
                                            as isize) as *mut aas_face_t;
        fen = 0i32;
        while fen < (*frontface).numedges {
            frontedgenum =
                abs(*aasworld.edgeindex.offset(((*frontface).firstedge + fen)
                                                   as isize));
            j = 0i32;
            while j < numbackfaces {
                backface =
                    &mut *aasworld.faces.offset(*backfacenums.as_mut_ptr().offset(j
                                                                                      as
                                                                                      isize)
                                                    as isize) as
                        *mut aas_face_t;
                ben = 0i32;
                while ben < (*backface).numedges {
                    backedgenum =
                        abs(*aasworld.edgeindex.offset(((*backface).firstedge
                                                            + ben) as isize));
                    if frontedgenum == backedgenum { break ; }
                    ben += 1
                }
                //end for
                if ben != (*backface).numedges { break ; }
                j += 1
            }
            //end for
            if j != numbackfaces { break ; }
            fen += 1
        }
        //end for
        if fen != (*frontface).numedges { break ; }
        i += 1
    }
    if i != numfrontfaces { return 0i32 }
    i = 0i32;
    while i < numareas {
        (*aasworld.areasettings.offset(areanums[i as usize] as
                                           isize)).contents |= 8i32;
        (*aasworld.areasettings.offset(areanums[i as usize] as
                                           isize)).contents |= 32i32;
        Log_Write(b"possible portal: %d\r\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  areanums[i as usize]);
        i += 1
    }
    return numareas;
}
//end for
//end of the function AAS_ConnectedAreas_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ConnectedAreas(mut areanums: *mut libc::c_int,
                                            mut numareas: libc::c_int)
 -> qboolean {
    let mut connectedareas: [libc::c_int; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    memset(connectedareas.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_int; 1024]>() as libc::c_ulong);
    if numareas < 1i32 { return qfalse }
    if numareas == 1i32 { return qtrue }
    AAS_ConnectedAreas_r(areanums, numareas, connectedareas.as_mut_ptr(),
                         0i32);
    i = 0i32;
    while i < numareas {
        if 0 == connectedareas[i as usize] { return qfalse }
        i += 1
    }
    return qtrue;
}
//end if
//end for
//end of the function AAS_CreatePortals
/*
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
int AAS_MapContainsTeleporters(void)
{
	bsp_entity_t *entities, *ent;
	char *classname;

	entities = AAS_ParseBSPEntities();

	for (ent = entities; ent; ent = ent->next)
	{
		classname = AAS_ValueForBSPEpairKey(ent, "classname");
		if (classname && !strcmp(classname, "misc_teleporter"))
		{
			AAS_FreeBSPEntities(entities);
			return qtrue;
		} //end if
	} //end for
	return qfalse;
} //end of the function AAS_MapContainsTeleporters
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
int AAS_NonConvexFaces(aas_face_t *face1, aas_face_t *face2, int side1, int side2)
{
	int i, j, edgenum;
	aas_plane_t *plane1, *plane2;
	aas_edge_t *edge;
	

	plane1 = &aasworld.planes[face1->planenum ^ side1];
	plane2 = &aasworld.planes[face2->planenum ^ side2];

	//check if one of the points of face1 is at the back of the plane of face2
	for (i = 0; i < face1->numedges; i++)
	{
		edgenum = abs(aasworld.edgeindex[face1->firstedge + i]);
		edge = &aasworld.edges[edgenum];
		for (j = 0; j < 2; j++)
		{
			if (DotProduct(plane2->normal, aasworld.vertexes[edge->v[j]]) -
							plane2->dist < -0.01) return qtrue;
		} //end for
	} //end for
	for (i = 0; i < face2->numedges; i++)
	{
		edgenum = abs(aasworld.edgeindex[face2->firstedge + i]);
		edge = &aasworld.edges[edgenum];
		for (j = 0; j < 2; j++)
		{
			if (DotProduct(plane1->normal, aasworld.vertexes[edge->v[j]]) -
							plane1->dist < -0.01) return qtrue;
		} //end for
	} //end for

	return qfalse;
} //end of the function AAS_NonConvexFaces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
qboolean AAS_CanMergeAreas(int *areanums, int numareas)
{
	int i, j, s, face1num, face2num, side1, side2, fn1, fn2;
	aas_face_t *face1, *face2;
	aas_area_t *area1, *area2;

	for (i = 0; i < numareas; i++)
	{
		area1 = &aasworld.areas[areanums[i]];
		for (fn1 = 0; fn1 < area1->numfaces; fn1++)
		{
			face1num = abs(aasworld.faceindex[area1->firstface + fn1]);
			face1 = &aasworld.faces[face1num];
			side1 = face1->frontarea != areanums[i];
			//check if the face isn't a shared one with one of the other areas
			for (s = 0; s < numareas; s++)
			{
				if (s == i) continue;
				if (face1->frontarea == s || face1->backarea == s) break;
			} //end for
			//if the face was a shared one
			if (s != numareas) continue;
			//
			for (j = 0; j < numareas; j++)
			{
				if (j == i) continue;
				area2 = &aasworld.areas[areanums[j]];
				for (fn2 = 0; fn2 < area2->numfaces; fn2++)
				{
					face2num = abs(aasworld.faceindex[area2->firstface + fn2]);
					face2 = &aasworld.faces[face2num];
					side2 = face2->frontarea != areanums[j];
					//check if the face isn't a shared one with one of the other areas
					for (s = 0; s < numareas; s++)
					{
						if (s == j) continue;
						if (face2->frontarea == s || face2->backarea == s) break;
					} //end for
					//if the face was a shared one
					if (s != numareas) continue;
					//
					if (AAS_NonConvexFaces(face1, face2, side1, side2)) return qfalse;
				} //end for
			} //end for
		} //end for
	} //end for
	return qtrue;
} //end of the function AAS_CanMergeAreas
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
qboolean AAS_NonConvexEdges(aas_edge_t *edge1, aas_edge_t *edge2, int side1, int side2, int planenum)
{
	int i;
	vec3_t edgevec1, edgevec2, normal1, normal2;
	float dist1, dist2;
	aas_plane_t *plane;

	plane = &aasworld.planes[planenum];
	VectorSubtract(aasworld.vertexes[edge1->v[1]], aasworld.vertexes[edge1->v[0]], edgevec1);
	VectorSubtract(aasworld.vertexes[edge2->v[1]], aasworld.vertexes[edge2->v[0]], edgevec2);
	if (side1) VectorInverse(edgevec1);
	if (side2) VectorInverse(edgevec2);
	//
	CrossProduct(edgevec1, plane->normal, normal1);
	dist1 = DotProduct(normal1, aasworld.vertexes[edge1->v[0]]);
	CrossProduct(edgevec2, plane->normal, normal2);
	dist2 = DotProduct(normal2, aasworld.vertexes[edge2->v[0]]);

	for (i = 0; i < 2; i++)
	{
		if (DotProduct(aasworld.vertexes[edge1->v[i]], normal2) - dist2 < -0.01) return qfalse;
	} //end for
	for (i = 0; i < 2; i++)
	{
		if (DotProduct(aasworld.vertexes[edge2->v[i]], normal1) - dist1 < -0.01) return qfalse;
	} //end for
	return qtrue;
} //end of the function AAS_NonConvexEdges
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
qboolean AAS_CanMergeFaces(int *facenums, int numfaces, int planenum)
{
	int i, j, s, edgenum1, edgenum2, side1, side2, en1, en2, ens;
	aas_face_t *face1, *face2, *otherface;
	aas_edge_t *edge1, *edge2;

	for (i = 0; i < numfaces; i++)
	{
		face1 = &aasworld.faces[facenums[i]];
		for (en1 = 0; en1 < face1->numedges; en1++)
		{
			edgenum1 = aasworld.edgeindex[face1->firstedge + en1];
			side1 = (edgenum1 < 0) ^ (face1->planenum != planenum);
			edgenum1 = abs(edgenum1);
			edge1 = &aasworld.edges[edgenum1];
			//check if the edge is shared with another face
			for (s = 0; s < numfaces; s++)
			{
				if (s == i) continue;
				otherface = &aasworld.faces[facenums[s]];
				for (ens = 0; ens < otherface->numedges; ens++)
				{
					if (edgenum1 == abs(aasworld.edgeindex[otherface->firstedge + ens])) break;
				} //end for
				if (ens != otherface->numedges) break;
			} //end for
			//if the edge was shared
			if (s != numfaces) continue;
			//
			for (j = 0; j < numfaces; j++)
			{
				if (j == i) continue;
				face2 = &aasworld.faces[facenums[j]];
				for (en2 = 0; en2 < face2->numedges; en2++)
				{
					edgenum2 = aasworld.edgeindex[face2->firstedge + en2];
					side2 = (edgenum2 < 0) ^ (face2->planenum != planenum);
					edgenum2 = abs(edgenum2);
					edge2 = &aasworld.edges[edgenum2];
					//check if the edge is shared with another face
					for (s = 0; s < numfaces; s++)
					{
						if (s == i) continue;
						otherface = &aasworld.faces[facenums[s]];
						for (ens = 0; ens < otherface->numedges; ens++)
						{
							if (edgenum2 == abs(aasworld.edgeindex[otherface->firstedge + ens])) break;
						} //end for
						if (ens != otherface->numedges) break;
					} //end for
					//if the edge was shared
					if (s != numfaces) continue;
					//
					if (AAS_NonConvexEdges(edge1, edge2, side1, side2, planenum)) return qfalse;
				} //end for
			} //end for
		} //end for
	} //end for
	return qtrue;
} //end of the function AAS_CanMergeFaces*/
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ConnectedAreas_r(mut areanums: *mut libc::c_int,
                                              mut numareas: libc::c_int,
                                              mut connectedareas:
                                                  *mut libc::c_int,
                                              mut curarea: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut otherareanum: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    *connectedareas.offset(curarea as isize) = qtrue as libc::c_int;
    area =
        &mut *aasworld.areas.offset(*areanums.offset(curarea as isize) as
                                        isize) as *mut aas_area_t;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            abs(*aasworld.faceindex.offset(((*area).firstface + i) as isize));
        face =
            &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
        //if the face is solid
        if !(0 != (*face).faceflags & 1i32) {
            if (*face).frontarea != *areanums.offset(curarea as isize) {
                otherareanum = (*face).frontarea
            } else { otherareanum = (*face).backarea }
            j = 0i32;
            while j < numareas {
                if *areanums.offset(j as isize) == otherareanum { break ; }
                j += 1
            }
            //end for
            //if the face isn't leading to one of the other areas
            if !(j == numareas) {
                //if the other area is already connected
                if !(0 != *connectedareas.offset(j as isize)) {
                    AAS_ConnectedAreas_r(areanums, numareas, connectedareas,
                                         j);
                }
            }
        }
        i += 1
    };
}
//end of the function AAS_ConnectedAreas
//===========================================================================
// gets adjacent areas with less presence types recursively
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_GetAdjacentAreasWithLessPresenceTypes_r(mut areanums:
                                                                         *mut libc::c_int,
                                                                     mut numareas:
                                                                         libc::c_int,
                                                                     mut curareanum:
                                                                         libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut presencetype: libc::c_int = 0;
    let mut otherpresencetype: libc::c_int = 0;
    let mut otherareanum: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let fresh14 = numareas;
    numareas = numareas + 1;
    *areanums.offset(fresh14 as isize) = curareanum;
    area =
        &mut *aasworld.areas.offset(curareanum as isize) as *mut aas_area_t;
    presencetype =
        (*aasworld.areasettings.offset(curareanum as isize)).presencetype;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            abs(*aasworld.faceindex.offset(((*area).firstface + i) as isize));
        face =
            &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
        //if the face is solid
        if !(0 != (*face).faceflags & 1i32) {
            if (*face).frontarea != curareanum {
                otherareanum = (*face).frontarea
            } else { otherareanum = (*face).backarea }
            otherpresencetype =
                (*aasworld.areasettings.offset(otherareanum as
                                                   isize)).presencetype;
            if 0 != presencetype & !otherpresencetype &&
                   0 == otherpresencetype & !presencetype {
                j = 0i32;
                while j < numareas {
                    if otherareanum == *areanums.offset(j as isize) {
                        break ;
                    }
                    j += 1
                }
                if j == numareas {
                    if numareas >= 1024i32 {
                        AAS_Error(b"MAX_PORTALAREAS\n\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                        return numareas
                    }
                    numareas =
                        AAS_GetAdjacentAreasWithLessPresenceTypes_r(areanums,
                                                                    numareas,
                                                                    otherareanum)
                }
            }
        }
        i += 1
    }
    return numareas;
}
//end of the function
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_CountForcedClusterPortals() {
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num = 0i32;
    i = 1i32;
    while i < aasworld.numareas {
        if 0 != (*aasworld.areasettings.offset(i as isize)).contents & 8i32 {
            Log_Write(b"area %d is a forced portal area\r\n\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char, i);
            num += 1
        }
        i += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%6d forced portal areas\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        num);
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_SetViewPortalsAsClusterPortals() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < aasworld.numareas {
        if 0 != (*aasworld.areasettings.offset(i as isize)).contents & 512i32
           {
            (*aasworld.areasettings.offset(i as isize)).contents |= 8i32
        }
        i += 1
    };
}
//end for
//end of the function AAS_RemoveClusterAreas
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ClearCluster(mut clusternum: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < aasworld.numareas {
        if (*aasworld.areasettings.offset(i as isize)).cluster == clusternum {
            (*aasworld.areasettings.offset(i as isize)).cluster = 0i32
        }
        i += 1
    };
}
//end if
//end for
//end of the function AAS_ClearCluster
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_RemovePortalsClusterReference(mut clusternum:
                                                               libc::c_int) {
    let mut portalnum: libc::c_int = 0;
    portalnum = 1i32;
    while portalnum < aasworld.numportals {
        if (*aasworld.portals.offset(portalnum as isize)).frontcluster ==
               clusternum {
            (*aasworld.portals.offset(portalnum as isize)).frontcluster = 0i32
        }
        if (*aasworld.portals.offset(portalnum as isize)).backcluster ==
               clusternum {
            (*aasworld.portals.offset(portalnum as isize)).backcluster = 0i32
        }
        portalnum += 1
    };
}
//end of the function AAS_FloodClusterAreasUsingReachabilities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_NumberClusterPortals(mut clusternum:
                                                      libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut portalnum: libc::c_int = 0;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    cluster =
        &mut *aasworld.clusters.offset(clusternum as isize) as
            *mut aas_cluster_t;
    i = 0i32;
    while i < (*cluster).numportals {
        portalnum =
            *aasworld.portalindex.offset(((*cluster).firstportal + i) as
                                             isize);
        portal =
            &mut *aasworld.portals.offset(portalnum as isize) as
                *mut aas_portal_t;
        if (*portal).frontcluster == clusternum {
            let fresh15 = (*cluster).numareas;
            (*cluster).numareas = (*cluster).numareas + 1;
            (*portal).clusterareanum[0usize] = fresh15
        } else {
            let fresh16 = (*cluster).numareas;
            (*cluster).numareas = (*cluster).numareas + 1;
            (*portal).clusterareanum[1usize] = fresh16
        }
        i += 1
    };
}
//end of the function AAS_FindPossiblePortals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_RemoveAllPortals() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < aasworld.numareas {
        (*aasworld.areasettings.offset(i as isize)).contents &= !8i32;
        i += 1
    };
}