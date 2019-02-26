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
      "ioq3/code/botlib/be_aas_optimize.c"]
pub mod be_aas_optimize_c {
    pub type optimized_t = optimized_s;
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
 * name:		be_aas_optimize.c
 *
 * desc:		decreases the .aas file size after the reachabilities have
 *				been calculated, just dumps all the faces, edges and vertexes
 *
 * $Archive: /MissionPack/code/botlib/be_aas_optimize.c $
 *
 *****************************************************************************/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct optimized_s {
        pub numvertexes: libc::c_int,
        pub vertexes: *mut aas_vertex_t,
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
        pub vertexoptimizeindex: *mut libc::c_int,
        pub edgeoptimizeindex: *mut libc::c_int,
        pub faceoptimizeindex: *mut libc::c_int,
    }
    use super::{libc};
    use super::aasfile_h::{aas_vertex_t, aas_edge_t, aas_edgeindex_t,
                           aas_face_t, aas_faceindex_t, aas_area_t};
}
#[header_src =
      "ioq3/code/botlib/be_aas_def.h"]
pub mod be_aas_def_h {
    pub type aas_t = aas_s;
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
    pub type aas_reachabilityareas_t = aas_reachabilityareas_s;
    //areas a reachability goes through
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reachabilityareas_s {
        pub firstarea: libc::c_int,
        pub numareas: libc::c_int,
    }
    pub type aas_routingcache_t = aas_routingcache_s;
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
    pub type aas_reversedreachability_t = aas_reversedreachability_s;
    //reversed area reachability
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reversedreachability_s {
        pub numlinks: libc::c_int,
        pub first: *mut aas_reversedlink_t,
    }
    pub type aas_reversedlink_t = aas_reversedlink_s;
    //reversed reachability link
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_reversedlink_s {
        pub linknum: libc::c_int,
        pub areanum: libc::c_int,
        pub next: *mut aas_reversedlink_s,
    }
    pub type aas_routingupdate_t = aas_routingupdate_s;
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
    pub type aas_entity_t = aas_entity_s;
    //entity
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_entity_s {
        pub i: aas_entityinfo_t,
        pub areas: *mut aas_link_t,
        pub leaves: *mut bsp_link_t,
    }
    pub type bsp_link_t = bsp_link_s;
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
    pub type aas_link_t = aas_link_s;
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
    use super::{libc};
    use super::aasfile_h::{aas_bbox_t, aas_vertex_t, aas_plane_t, aas_edge_t,
                           aas_edgeindex_t, aas_face_t, aas_faceindex_t,
                           aas_area_t, aas_areasettings_t, aas_reachability_t,
                           aas_node_t, aas_portal_t, aas_portalindex_t,
                           aas_cluster_t};
    use super::q_shared_h::{byte, vec3_t, qboolean};
    use super::be_aas_h::{aas_entityinfo_t};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
      "ioq3/code/botlib/be_aas_optimize.h"]
pub mod be_aas_optimize_h { }
#[header_src =
      "ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
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
use self::be_aas_optimize_c::{optimized_t, optimized_s};
use self::be_aas_def_h::{aas_t, aas_s, aas_reachabilityareas_t,
                         aas_reachabilityareas_s, aas_routingcache_t,
                         aas_routingcache_s, aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s};
use self::string_h::{memcpy};
use self::stdlib_h::{abs};
use self::l_memory_h::{GetClearedMemory, FreeMemory};
use self::be_interface_h::{botimport};
use self::be_aas_main_h::{aasworld};
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
 * name:		be_aas_optimize.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_optimize.h $
 *
 *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn AAS_Optimize() {
    let mut i: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut optimized: optimized_t =
        optimized_s{numvertexes: 0,
                    vertexes: 0 as *mut aas_vertex_t,
                    numedges: 0,
                    edges: 0 as *mut aas_edge_t,
                    edgeindexsize: 0,
                    edgeindex: 0 as *mut aas_edgeindex_t,
                    numfaces: 0,
                    faces: 0 as *mut aas_face_t,
                    faceindexsize: 0,
                    faceindex: 0 as *mut aas_faceindex_t,
                    numareas: 0,
                    areas: 0 as *mut aas_area_t,
                    vertexoptimizeindex: 0 as *mut libc::c_int,
                    edgeoptimizeindex: 0 as *mut libc::c_int,
                    faceoptimizeindex: 0 as *mut libc::c_int,};
    AAS_OptimizeAlloc(&mut optimized);
    i = 1i32;
    while i < aasworld.numareas {
        AAS_OptimizeArea(&mut optimized, i);
        i += 1
    }
    i = 0i32;
    while i < aasworld.reachabilitysize {
        //NOTE: for TRAVEL_ELEVATOR the facenum is the model number of
		//		the elevator
        if !((*aasworld.reachability.offset(i as isize)).traveltype &
                 0xffffffi32 == 11i32) {
            //NOTE: for TRAVEL_JUMPPAD the facenum is the Z velocity and the edgenum is the hor velocity
            if !((*aasworld.reachability.offset(i as isize)).traveltype &
                     0xffffffi32 == 18i32) {
                //NOTE: for TRAVEL_FUNCBOB the facenum and edgenum contain other coded information
                if !((*aasworld.reachability.offset(i as isize)).traveltype &
                         0xffffffi32 == 19i32) {
                    sign =
                        (*aasworld.reachability.offset(i as isize)).facenum;
                    (*aasworld.reachability.offset(i as isize)).facenum =
                        *optimized.faceoptimizeindex.offset(abs((*aasworld.reachability.offset(i
                                                                                                   as
                                                                                                   isize)).facenum)
                                                                as isize);
                    if sign < 0i32 {
                        (*aasworld.reachability.offset(i as isize)).facenum =
                            -(*aasworld.reachability.offset(i as
                                                                isize)).facenum
                    }
                    sign =
                        (*aasworld.reachability.offset(i as isize)).edgenum;
                    (*aasworld.reachability.offset(i as isize)).edgenum =
                        *optimized.edgeoptimizeindex.offset(abs((*aasworld.reachability.offset(i
                                                                                                   as
                                                                                                   isize)).edgenum)
                                                                as isize);
                    if sign < 0i32 {
                        (*aasworld.reachability.offset(i as isize)).edgenum =
                            -(*aasworld.reachability.offset(i as
                                                                isize)).edgenum
                    }
                }
            }
        }
        i += 1
    }
    AAS_OptimizeStore(&mut optimized);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"AAS data optimized.\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
}
//end of the function AAS_OptimizeAlloc
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_OptimizeStore(mut optimized: *mut optimized_t) {
    if !aasworld.vertexes.is_null() {
        FreeMemory(aasworld.vertexes as *mut libc::c_void);
    }
    aasworld.vertexes = (*optimized).vertexes;
    aasworld.numvertexes = (*optimized).numvertexes;
    if !aasworld.edges.is_null() {
        FreeMemory(aasworld.edges as *mut libc::c_void);
    }
    aasworld.edges = (*optimized).edges;
    aasworld.numedges = (*optimized).numedges;
    if !aasworld.edgeindex.is_null() {
        FreeMemory(aasworld.edgeindex as *mut libc::c_void);
    }
    aasworld.edgeindex = (*optimized).edgeindex;
    aasworld.edgeindexsize = (*optimized).edgeindexsize;
    if !aasworld.faces.is_null() {
        FreeMemory(aasworld.faces as *mut libc::c_void);
    }
    aasworld.faces = (*optimized).faces;
    aasworld.numfaces = (*optimized).numfaces;
    if !aasworld.faceindex.is_null() {
        FreeMemory(aasworld.faceindex as *mut libc::c_void);
    }
    aasworld.faceindex = (*optimized).faceindex;
    aasworld.faceindexsize = (*optimized).faceindexsize;
    if !aasworld.areas.is_null() {
        FreeMemory(aasworld.areas as *mut libc::c_void);
    }
    aasworld.areas = (*optimized).areas;
    aasworld.numareas = (*optimized).numareas;
    FreeMemory((*optimized).vertexoptimizeindex as *mut libc::c_void);
    FreeMemory((*optimized).edgeoptimizeindex as *mut libc::c_void);
    FreeMemory((*optimized).faceoptimizeindex as *mut libc::c_void);
}
//end of the function AAS_OptimizeFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_OptimizeArea(mut optimized: *mut optimized_t,
                                          mut areanum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut optfacenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut optarea: *mut aas_area_t = 0 as *mut aas_area_t;
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    optarea =
        &mut *(*optimized).areas.offset(areanum as isize) as *mut aas_area_t;
    memcpy(optarea as *mut libc::c_void, area as *const libc::c_void,
           ::std::mem::size_of::<aas_area_t>() as libc::c_ulong);
    (*optarea).numfaces = 0i32;
    (*optarea).firstface = (*optimized).faceindexsize;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            *aasworld.faceindex.offset(((*area).firstface + i) as isize);
        optfacenum = AAS_OptimizeFace(optimized, facenum);
        if 0 != optfacenum {
            *(*optimized).faceindex.offset(((*optarea).firstface +
                                                (*optarea).numfaces) as isize)
                = optfacenum;
            (*optarea).numfaces += 1;
            (*optimized).faceindexsize += 1
        }
        i += 1
    };
}
//end of the function AAS_KeepFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_OptimizeFace(mut optimized: *mut optimized_t,
                                          mut facenum: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut optedgenum: libc::c_int = 0;
    let mut optfacenum: libc::c_int = 0;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut optface: *mut aas_face_t = 0 as *mut aas_face_t;
    face =
        &mut *aasworld.faces.offset(abs(facenum) as isize) as *mut aas_face_t;
    if 0 == AAS_KeepFace(face) { return 0i32 }
    optfacenum =
        *(*optimized).faceoptimizeindex.offset(abs(facenum) as isize);
    if 0 != optfacenum {
        if facenum > 0i32 { return optfacenum } else { return -optfacenum }
    }
    optface =
        &mut *(*optimized).faces.offset((*optimized).numfaces as isize) as
            *mut aas_face_t;
    memcpy(optface as *mut libc::c_void, face as *const libc::c_void,
           ::std::mem::size_of::<aas_face_t>() as libc::c_ulong);
    (*optface).numedges = 0i32;
    (*optface).firstedge = (*optimized).edgeindexsize;
    i = 0i32;
    while i < (*face).numedges {
        edgenum =
            *aasworld.edgeindex.offset(((*face).firstedge + i) as isize);
        optedgenum = AAS_OptimizeEdge(optimized, edgenum);
        if 0 != optedgenum {
            *(*optimized).edgeindex.offset(((*optface).firstedge +
                                                (*optface).numedges) as isize)
                = optedgenum;
            (*optface).numedges += 1;
            (*optimized).edgeindexsize += 1
        }
        i += 1
    }
    *(*optimized).faceoptimizeindex.offset(abs(facenum) as isize) =
        (*optimized).numfaces;
    optfacenum = (*optimized).numfaces;
    (*optimized).numfaces += 1;
    if facenum > 0i32 { return optfacenum } else { return -optfacenum };
}
//end of the function AAS_KeepFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_OptimizeEdge(mut optimized: *mut optimized_t,
                                          mut edgenum: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut optedgenum: libc::c_int = 0;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut optedge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    edge =
        &mut *aasworld.edges.offset(abs(edgenum) as isize) as *mut aas_edge_t;
    if 0 == AAS_KeepEdge(edge) { return 0i32 }
    optedgenum =
        *(*optimized).edgeoptimizeindex.offset(abs(edgenum) as isize);
    if 0 != optedgenum {
        if edgenum > 0i32 { return optedgenum } else { return -optedgenum }
    }
    optedge =
        &mut *(*optimized).edges.offset((*optimized).numedges as isize) as
            *mut aas_edge_t;
    i = 0i32;
    while i < 2i32 {
        if 0 !=
               *(*optimized).vertexoptimizeindex.offset((*edge).v[i as usize]
                                                            as isize) {
            (*optedge).v[i as usize] =
                *(*optimized).vertexoptimizeindex.offset((*edge).v[i as usize]
                                                             as isize)
        } else {
            (*(*optimized).vertexes.offset((*optimized).numvertexes as
                                               isize))[0usize] =
                (*aasworld.vertexes.offset((*edge).v[i as usize] as
                                               isize))[0usize];
            (*(*optimized).vertexes.offset((*optimized).numvertexes as
                                               isize))[1usize] =
                (*aasworld.vertexes.offset((*edge).v[i as usize] as
                                               isize))[1usize];
            (*(*optimized).vertexes.offset((*optimized).numvertexes as
                                               isize))[2usize] =
                (*aasworld.vertexes.offset((*edge).v[i as usize] as
                                               isize))[2usize];
            (*optedge).v[i as usize] = (*optimized).numvertexes;
            *(*optimized).vertexoptimizeindex.offset((*edge).v[i as usize] as
                                                         isize) =
                (*optimized).numvertexes;
            (*optimized).numvertexes += 1
        }
        i += 1
    }
    *(*optimized).edgeoptimizeindex.offset(abs(edgenum) as isize) =
        (*optimized).numedges;
    optedgenum = (*optimized).numedges;
    (*optimized).numedges += 1;
    if edgenum > 0i32 { return optedgenum } else { return -optedgenum };
}
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_KeepEdge(mut edge: *mut aas_edge_t)
 -> libc::c_int {
    return 1i32;
}
//end of the function AAS_OptimizeEdge
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_KeepFace(mut face: *mut aas_face_t)
 -> libc::c_int {
    if 0 == (*face).faceflags & 2i32 { return 0i32 } else { return 1i32 };
}
//end if
//end for
//end of the function AAS_OptimizeArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_OptimizeAlloc(mut optimized: *mut optimized_t) {
    (*optimized).vertexes =
        GetClearedMemory((aasworld.numvertexes as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_vertex_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_vertex_t;
    (*optimized).numvertexes = 0i32;
    (*optimized).edges =
        GetClearedMemory((aasworld.numedges as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_edge_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_edge_t;
    (*optimized).numedges = 1i32;
    (*optimized).edgeindex =
        GetClearedMemory((aasworld.edgeindexsize as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_edgeindex_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_edgeindex_t;
    (*optimized).edgeindexsize = 0i32;
    (*optimized).faces =
        GetClearedMemory((aasworld.numfaces as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_face_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_face_t;
    (*optimized).numfaces = 1i32;
    (*optimized).faceindex =
        GetClearedMemory((aasworld.faceindexsize as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_faceindex_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_faceindex_t;
    (*optimized).faceindexsize = 0i32;
    (*optimized).areas =
        GetClearedMemory((aasworld.numareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_area_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_area_t;
    (*optimized).numareas = aasworld.numareas;
    (*optimized).vertexoptimizeindex =
        GetClearedMemory((aasworld.numvertexes as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    (*optimized).edgeoptimizeindex =
        GetClearedMemory((aasworld.numedges as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    (*optimized).faceoptimizeindex =
        GetClearedMemory((aasworld.numfaces as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
}