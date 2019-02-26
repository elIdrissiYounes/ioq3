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
    extern "C" {
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
    }
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
    pub type aas_trace_t = aas_trace_s;
    //a trace is returned when a box is swept through the AAS world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_trace_s {
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub ent: libc::c_int,
        pub lastarea: libc::c_int,
        pub area: libc::c_int,
        pub planenum: libc::c_int,
    }
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
    // area info
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_areainfo_s {
        pub contents: libc::c_int,
        pub flags: libc::c_int,
        pub presencetype: libc::c_int,
        pub cluster: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub center: vec3_t,
    }
    pub type aas_entityinfo_t = aas_entityinfo_s;
    pub type aas_areainfo_t = aas_areainfo_s;
    use super::q_shared_h::{qboolean, vec3_t};
    use super::{libc};
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
#[header_src =
      "ioq3/code/botlib/be_aas_sample.c"]
pub mod be_aas_sample_c {
    pub type aas_tracestack_t = aas_tracestack_s;
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
 * name:		be_aas_sample.c
 *
 * desc:		AAS environment sampling
 *
 * $Archive: /MissionPack/code/botlib/be_aas_sample.c $
 *
 *****************************************************************************/
    //#define AAS_SAMPLE_DEBUG
    //0.0005
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_tracestack_s {
        pub start: vec3_t,
        pub end: vec3_t,
        pub planenum: libc::c_int,
        pub nodenum: libc::c_int,
    }
    //end for
    //end of the function AAS_UnlinkFromAreas
    //===========================================================================
// link the entity to the areas the bounding box is totally or partly
// situated in. This is done with recursion down the tree using the
// bounding box to test for plane sides
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_linkstack_t {
        pub nodenum: libc::c_int,
    }
    use super::q_shared_h::{vec3_t, qboolean, vec_t};
    use super::{libc};
    use super::be_aas_h::{aas_trace_t};
    use super::be_aas_def_h::{aas_link_t};
    use super::aasfile_h::{aas_plane_t};
}
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
    }
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
        //
        //allocate a memory block of the given size
        #[no_mangle]
        pub fn GetHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
    }
}
#[header_src =
      "ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
        //true if developer is on
        #[no_mangle]
        pub static mut botDeveloper: libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::{libc};
    use super::q_shared_h::{vec_t, vec3_t, qboolean};
    use super::be_aas_h::{aas_trace_t, aas_areainfo_t};
    use super::be_aas_def_h::{aas_link_t};
    use super::aasfile_h::{aas_face_t, aas_plane_t};
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
      "ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::q_shared_h::{qboolean, vec_t};
    use super::{libc};
    use super::botlib_h::{bsp_trace_t};
    extern "C" {
        //calculates collision with given entity
        #[no_mangle]
        pub fn AAS_EntityCollision(entnum: libc::c_int, start: *mut vec_t,
                                   boxmins: *mut vec_t, boxmaxs: *mut vec_t,
                                   end: *mut vec_t, contentmask: libc::c_int,
                                   trace: *mut bsp_trace_t) -> qboolean;
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
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, VectorNormalize};
use self::aasfile_h::{aas_bbox_s, aas_bbox_t, aas_reachability_s,
                      aas_reachability_t, aas_areasettings_s,
                      aas_areasettings_t, aas_portal_s, aas_portal_t,
                      aas_portalindex_t, aas_cluster_s, aas_cluster_t,
                      aas_vertex_t, aas_plane_s, aas_plane_t, aas_edge_s,
                      aas_edge_t, aas_edgeindex_t, aas_face_s, aas_face_t,
                      aas_faceindex_t, aas_area_s, aas_area_t, aas_node_s,
                      aas_node_t};
use self::be_aas_h::{aas_trace_t, aas_trace_s, aas_entityinfo_s,
                     aas_areainfo_s, aas_entityinfo_t, aas_areainfo_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_aas_def_h::{aas_t, aas_s, aas_reachabilityareas_t,
                         aas_reachabilityareas_s, aas_routingcache_t,
                         aas_routingcache_s, aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s};
use self::be_aas_sample_c::{aas_tracestack_t, aas_tracestack_s,
                            aas_linkstack_t};
use self::mathcalls_h::{sqrt};
use self::string_h::{memset};
use self::stdlib_h::{abs};
use self::l_memory_h::{GetHunkMemory, GetClearedHunkMemory, FreeMemory};
use self::l_libvar_h::{LibVarValue};
use self::be_interface_h::{botimport, botDeveloper};
use self::be_aas_main_h::{aasworld};
use self::be_aas_bsp_h::{AAS_EntityCollision};
use self::be_aas_reach_h::{AAS_AreaReachability};
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
unsafe extern "C" fn VectorInverse(mut v: *mut vec_t) {
    *v.offset(0isize) = -*v.offset(0isize);
    *v.offset(1isize) = -*v.offset(1isize);
    *v.offset(2isize) = -*v.offset(2isize);
}
unsafe extern "C" fn CrossProduct(mut v1: *const vec_t, mut v2: *const vec_t,
                                  mut cross: *mut vec_t) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) -
            *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) -
            *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) -
            *v1.offset(1isize) * *v2.offset(0isize);
}
//AASINTERN
//returns the mins and maxs of the bounding box for the given presence type
#[no_mangle]
pub unsafe extern "C" fn AAS_PresenceTypeBoundingBox(mut presencetype:
                                                         libc::c_int,
                                                     mut mins: *mut vec_t,
                                                     mut maxs: *mut vec_t) {
    let mut index: libc::c_int = 0;
    //bounding box size for each presence type
    let mut boxmins: [vec3_t; 3] =
        [[0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
         [-15i32 as vec_t, -15i32 as vec_t, -24i32 as vec_t],
         [-15i32 as vec_t, -15i32 as vec_t, -24i32 as vec_t]];
    let mut boxmaxs: [vec3_t; 3] =
        [[0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
         [15i32 as vec_t, 15i32 as vec_t, 32i32 as vec_t],
         [15i32 as vec_t, 15i32 as vec_t, 8i32 as vec_t]];
    if presencetype == 2i32 {
        index = 1i32
    } else if presencetype == 4i32 {
        index = 2i32
    } else {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_PresenceTypeBoundingBox: unknown presence type\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        index = 2i32
    }
    *mins.offset(0isize) = boxmins[index as usize][0usize];
    *mins.offset(1isize) = boxmins[index as usize][1usize];
    *mins.offset(2isize) = boxmins[index as usize][2usize];
    *maxs.offset(0isize) = boxmaxs[index as usize][0usize];
    *maxs.offset(1isize) = boxmaxs[index as usize][1usize];
    *maxs.offset(2isize) = boxmaxs[index as usize][2usize];
}
//returns the cluster the area is in (negative portal number if the area is a portal)
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaCluster(mut areanum: libc::c_int)
 -> libc::c_int {
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_AreaCluster: invalid area number\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0i32
    }
    return (*aasworld.areasettings.offset(areanum as isize)).cluster;
}
//returns the presence type(s) of the area
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaPresenceType(mut areanum: libc::c_int)
 -> libc::c_int {
    if 0 == aasworld.loaded { return 0i32 }
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_AreaPresenceType: invalid area number\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0i32
    }
    return (*aasworld.areasettings.offset(areanum as isize)).presencetype;
}
//returns the presence type(s) at the given point
#[no_mangle]
pub unsafe extern "C" fn AAS_PointPresenceType(mut point: *mut vec_t)
 -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    if 0 == aasworld.loaded { return 0i32 }
    areanum = AAS_PointAreaNum(point);
    if 0 == areanum { return 1i32 }
    return (*aasworld.areasettings.offset(areanum as isize)).presencetype;
}
//returns the area the point is in
#[no_mangle]
pub unsafe extern "C" fn AAS_PointAreaNum(mut point: *mut vec_t)
 -> libc::c_int {
    let mut nodenum: libc::c_int = 0;
    let mut dist: vec_t = 0.;
    let mut node: *mut aas_node_t = 0 as *mut aas_node_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    if 0 == aasworld.loaded {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_PointAreaNum: aas not loaded\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0i32
    }
    nodenum = 1i32;
    while nodenum > 0i32 {
        node =
            &mut *aasworld.nodes.offset(nodenum as isize) as *mut aas_node_t;
        plane =
            &mut *aasworld.planes.offset((*node).planenum as isize) as
                *mut aas_plane_t;
        dist =
            *point.offset(0isize) * (*plane).normal[0usize] +
                *point.offset(1isize) * (*plane).normal[1usize] +
                *point.offset(2isize) * (*plane).normal[2usize] -
                (*plane).dist;
        if dist > 0i32 as libc::c_float {
            nodenum = (*node).children[0usize]
        } else { nodenum = (*node).children[1usize] }
    }
    if 0 == nodenum { return 0i32 }
    return -nodenum;
}
//returns the result of the trace of a client bbox
#[no_mangle]
pub unsafe extern "C" fn AAS_TraceClientBBox(mut start: *mut vec_t,
                                             mut end: *mut vec_t,
                                             mut presencetype: libc::c_int,
                                             mut passent: libc::c_int)
 -> aas_trace_t {
    let mut side: libc::c_int = 0;
    let mut nodenum: libc::c_int = 0;
    let mut tmpplanenum: libc::c_int = 0;
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut cur_start: vec3_t = [0.; 3];
    let mut cur_end: vec3_t = [0.; 3];
    let mut cur_mid: vec3_t = [0.; 3];
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut tracestack: [aas_tracestack_t; 127] =
        [aas_tracestack_s{start: [0.; 3],
                          end: [0.; 3],
                          planenum: 0,
                          nodenum: 0,}; 127];
    let mut tstack_p: *mut aas_tracestack_t = 0 as *mut aas_tracestack_t;
    let mut aasnode: *mut aas_node_t = 0 as *mut aas_node_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    memset(&mut trace as *mut aas_trace_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_trace_t>() as libc::c_ulong);
    if 0 == aasworld.loaded { return trace }
    tstack_p = tracestack.as_mut_ptr();
    (*tstack_p).start[0usize] = *start.offset(0isize);
    (*tstack_p).start[1usize] = *start.offset(1isize);
    (*tstack_p).start[2usize] = *start.offset(2isize);
    (*tstack_p).end[0usize] = *end.offset(0isize);
    (*tstack_p).end[1usize] = *end.offset(1isize);
    (*tstack_p).end[2usize] = *end.offset(2isize);
    (*tstack_p).planenum = 0i32;
    (*tstack_p).nodenum = 1i32;
    tstack_p = tstack_p.offset(1isize);
    loop  {
        tstack_p = tstack_p.offset(-1isize);
        if tstack_p < tracestack.as_mut_ptr() {
            tstack_p = tstack_p.offset(1isize);
            trace.startsolid = qfalse;
            trace.fraction = 1.0f64 as libc::c_float;
            trace.endpos[0usize] = *end.offset(0isize);
            trace.endpos[1usize] = *end.offset(1isize);
            trace.endpos[2usize] = *end.offset(2isize);
            trace.ent = 0i32;
            trace.area = 0i32;
            trace.planenum = 0i32;
            return trace
        }
        nodenum = (*tstack_p).nodenum;
        //if it is an area
        if nodenum < 0i32 {
            if 0 ==
                   (*aasworld.areasettings.offset(-nodenum as
                                                      isize)).presencetype &
                       presencetype {
                if (*tstack_p).start[0usize] == *start.offset(0isize) &&
                       (*tstack_p).start[1usize] == *start.offset(1isize) &&
                       (*tstack_p).start[2usize] == *start.offset(2isize) {
                    trace.startsolid = qtrue;
                    trace.fraction = 0.0f64 as libc::c_float;
                    v1[2usize] = 0i32 as vec_t;
                    v1[1usize] = v1[2usize];
                    v1[0usize] = v1[1usize]
                } else {
                    trace.startsolid = qfalse;
                    v1[0usize] = *end.offset(0isize) - *start.offset(0isize);
                    v1[1usize] = *end.offset(1isize) - *start.offset(1isize);
                    v1[2usize] = *end.offset(2isize) - *start.offset(2isize);
                    v2[0usize] =
                        (*tstack_p).start[0usize] - *start.offset(0isize);
                    v2[1usize] =
                        (*tstack_p).start[1usize] - *start.offset(1isize);
                    v2[2usize] =
                        (*tstack_p).start[2usize] - *start.offset(2isize);
                    trace.fraction =
                        VectorLength(v2.as_mut_ptr() as *const vec_t) /
                            VectorNormalize(v1.as_mut_ptr());
                    (*tstack_p).start[0usize] =
                        ((*tstack_p).start[0usize] as libc::c_double +
                             v1[0usize] as libc::c_double * -0.125f64) as
                            vec_t;
                    (*tstack_p).start[1usize] =
                        ((*tstack_p).start[1usize] as libc::c_double +
                             v1[1usize] as libc::c_double * -0.125f64) as
                            vec_t;
                    (*tstack_p).start[2usize] =
                        ((*tstack_p).start[2usize] as libc::c_double +
                             v1[2usize] as libc::c_double * -0.125f64) as
                            vec_t
                }
                trace.endpos[0usize] = (*tstack_p).start[0usize];
                trace.endpos[1usize] = (*tstack_p).start[1usize];
                trace.endpos[2usize] = (*tstack_p).start[2usize];
                trace.ent = 0i32;
                trace.area = -nodenum;
                trace.planenum = (*tstack_p).planenum;
                plane =
                    &mut *aasworld.planes.offset(trace.planenum as isize) as
                        *mut aas_plane_t;
                if v1[0usize] * (*plane).normal[0usize] +
                       v1[1usize] * (*plane).normal[1usize] +
                       v1[2usize] * (*plane).normal[2usize] >
                       0i32 as libc::c_float {
                    trace.planenum ^= 1i32
                }
                return trace
            } else {
                if passent >= 0i32 {
                    if 0 !=
                           AAS_AreaEntityCollision(-nodenum,
                                                   (*tstack_p).start.as_mut_ptr(),
                                                   (*tstack_p).end.as_mut_ptr(),
                                                   presencetype, passent,
                                                   &mut trace) as u64 {
                        if 0 == trace.startsolid as u64 {
                            v1[0usize] =
                                *end.offset(0isize) - *start.offset(0isize);
                            v1[1usize] =
                                *end.offset(1isize) - *start.offset(1isize);
                            v1[2usize] =
                                *end.offset(2isize) - *start.offset(2isize);
                            v2[0usize] =
                                trace.endpos[0usize] - *start.offset(0isize);
                            v2[1usize] =
                                trace.endpos[1usize] - *start.offset(1isize);
                            v2[2usize] =
                                trace.endpos[2usize] - *start.offset(2isize);
                            trace.fraction =
                                VectorLength(v2.as_mut_ptr() as *const vec_t)
                                    /
                                    VectorLength(v1.as_mut_ptr() as
                                                     *const vec_t)
                        }
                        return trace
                    }
                }
            }
            trace.lastarea = -nodenum
        } else {
            if 0 == nodenum {
                if (*tstack_p).start[0usize] == *start.offset(0isize) &&
                       (*tstack_p).start[1usize] == *start.offset(1isize) &&
                       (*tstack_p).start[2usize] == *start.offset(2isize) {
                    trace.startsolid = qtrue;
                    trace.fraction = 0.0f64 as libc::c_float;
                    v1[2usize] = 0i32 as vec_t;
                    v1[1usize] = v1[2usize];
                    v1[0usize] = v1[1usize]
                } else {
                    trace.startsolid = qfalse;
                    v1[0usize] = *end.offset(0isize) - *start.offset(0isize);
                    v1[1usize] = *end.offset(1isize) - *start.offset(1isize);
                    v1[2usize] = *end.offset(2isize) - *start.offset(2isize);
                    v2[0usize] =
                        (*tstack_p).start[0usize] - *start.offset(0isize);
                    v2[1usize] =
                        (*tstack_p).start[1usize] - *start.offset(1isize);
                    v2[2usize] =
                        (*tstack_p).start[2usize] - *start.offset(2isize);
                    trace.fraction =
                        VectorLength(v2.as_mut_ptr() as *const vec_t) /
                            VectorNormalize(v1.as_mut_ptr());
                    (*tstack_p).start[0usize] =
                        ((*tstack_p).start[0usize] as libc::c_double +
                             v1[0usize] as libc::c_double * -0.125f64) as
                            vec_t;
                    (*tstack_p).start[1usize] =
                        ((*tstack_p).start[1usize] as libc::c_double +
                             v1[1usize] as libc::c_double * -0.125f64) as
                            vec_t;
                    (*tstack_p).start[2usize] =
                        ((*tstack_p).start[2usize] as libc::c_double +
                             v1[2usize] as libc::c_double * -0.125f64) as
                            vec_t
                }
                trace.endpos[0usize] = (*tstack_p).start[0usize];
                trace.endpos[1usize] = (*tstack_p).start[1usize];
                trace.endpos[2usize] = (*tstack_p).start[2usize];
                trace.ent = 0i32;
                trace.area = 0i32;
                trace.planenum = (*tstack_p).planenum;
                plane =
                    &mut *aasworld.planes.offset(trace.planenum as isize) as
                        *mut aas_plane_t;
                if v1[0usize] * (*plane).normal[0usize] +
                       v1[1usize] * (*plane).normal[1usize] +
                       v1[2usize] * (*plane).normal[2usize] >
                       0i32 as libc::c_float {
                    trace.planenum ^= 1i32
                }
                return trace
            }
            aasnode =
                &mut *aasworld.nodes.offset(nodenum as isize) as
                    *mut aas_node_t;
            cur_start[0usize] = (*tstack_p).start[0usize];
            cur_start[1usize] = (*tstack_p).start[1usize];
            cur_start[2usize] = (*tstack_p).start[2usize];
            cur_end[0usize] = (*tstack_p).end[0usize];
            cur_end[1usize] = (*tstack_p).end[1usize];
            cur_end[2usize] = (*tstack_p).end[2usize];
            plane =
                &mut *aasworld.planes.offset((*aasnode).planenum as isize) as
                    *mut aas_plane_t;
            match (*plane).type_0 { _ => { } }
            front =
                cur_start[0usize] * (*plane).normal[0usize] +
                    cur_start[1usize] * (*plane).normal[1usize] +
                    cur_start[2usize] * (*plane).normal[2usize] -
                    (*plane).dist;
            back =
                cur_end[0usize] * (*plane).normal[0usize] +
                    cur_end[1usize] * (*plane).normal[1usize] +
                    cur_end[2usize] * (*plane).normal[2usize] - (*plane).dist;
            if front >= -0i32 as libc::c_float &&
                   back >= -0i32 as libc::c_float {
                (*tstack_p).nodenum = (*aasnode).children[0usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceBoundingBox: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return trace
                }
            } else if front < 0i32 as libc::c_float &&
                          back < 0i32 as libc::c_float {
                (*tstack_p).nodenum = (*aasnode).children[1usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceBoundingBox: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return trace
                }
            } else {
                tmpplanenum = (*tstack_p).planenum;
                if front == back { front -= 0.001f32 }
                if front < 0i32 as libc::c_float {
                    frac =
                        ((front as libc::c_double + 0.125f64) /
                             (front - back) as libc::c_double) as
                            libc::c_float
                } else {
                    frac =
                        ((front as libc::c_double - 0.125f64) /
                             (front - back) as libc::c_double) as
                            libc::c_float
                }
                if frac < 0i32 as libc::c_float {
                    frac = 0.001f32
                } else if frac > 1i32 as libc::c_float { frac = 0.999f32 }
                cur_mid[0usize] =
                    cur_start[0usize] +
                        (cur_end[0usize] - cur_start[0usize]) * frac;
                cur_mid[1usize] =
                    cur_start[1usize] +
                        (cur_end[1usize] - cur_start[1usize]) * frac;
                cur_mid[2usize] =
                    cur_start[2usize] +
                        (cur_end[2usize] - cur_start[2usize]) * frac;
                side = (front < 0i32 as libc::c_float) as libc::c_int;
                (*tstack_p).start[0usize] = cur_mid[0usize];
                (*tstack_p).start[1usize] = cur_mid[1usize];
                (*tstack_p).start[2usize] = cur_mid[2usize];
                (*tstack_p).planenum = (*aasnode).planenum;
                (*tstack_p).nodenum =
                    (*aasnode).children[(0 == side) as libc::c_int as usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceBoundingBox: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return trace
                }
                (*tstack_p).start[0usize] = cur_start[0usize];
                (*tstack_p).start[1usize] = cur_start[1usize];
                (*tstack_p).start[2usize] = cur_start[2usize];
                (*tstack_p).end[0usize] = cur_mid[0usize];
                (*tstack_p).end[1usize] = cur_mid[1usize];
                (*tstack_p).end[2usize] = cur_mid[2usize];
                (*tstack_p).planenum = tmpplanenum;
                (*tstack_p).nodenum = (*aasnode).children[side as usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceBoundingBox: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return trace
                }
            }
        }
    };
}
//end of the function AAS_BoxOriginDistanceFromPlane
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaEntityCollision(mut areanum: libc::c_int,
                                                 mut start: *mut vec_t,
                                                 mut end: *mut vec_t,
                                                 mut presencetype:
                                                     libc::c_int,
                                                 mut passent: libc::c_int,
                                                 mut trace: *mut aas_trace_t)
 -> qboolean {
    let mut collision: libc::c_int = 0;
    let mut boxmins: vec3_t = [0.; 3];
    let mut boxmaxs: vec3_t = [0.; 3];
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut bsptrace: bsp_trace_t =
        bsp_trace_s{allsolid: qfalse,
                    startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    plane:
                        cplane_s{normal: [0.; 3],
                                 dist: 0.,
                                 type_0: 0,
                                 signbits: 0,
                                 pad: [0; 2],},
                    exp_dist: 0.,
                    sidenum: 0,
                    surface:
                        bsp_surface_s{name: [0; 16], flags: 0, value: 0,},
                    contents: 0,
                    ent: 0,};
    AAS_PresenceTypeBoundingBox(presencetype, boxmins.as_mut_ptr(),
                                boxmaxs.as_mut_ptr());
    memset(&mut bsptrace as *mut bsp_trace_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bsp_trace_t>() as libc::c_ulong);
    bsptrace.fraction = 1i32 as libc::c_float;
    collision = qfalse as libc::c_int;
    link = *aasworld.arealinkedentities.offset(areanum as isize);
    while !link.is_null() {
        //ignore the pass entity
        if !((*link).entnum == passent) {
            if 0 !=
                   AAS_EntityCollision((*link).entnum, start,
                                       boxmins.as_mut_ptr(),
                                       boxmaxs.as_mut_ptr(), end,
                                       1i32 | 0x10000i32, &mut bsptrace) as
                       u64 {
                collision = qtrue as libc::c_int
            }
        }
        link = (*link).next_ent
    }
    if 0 != collision {
        (*trace).startsolid = bsptrace.startsolid;
        (*trace).ent = bsptrace.ent;
        (*trace).endpos[0usize] = bsptrace.endpos[0usize];
        (*trace).endpos[1usize] = bsptrace.endpos[1usize];
        (*trace).endpos[2usize] = bsptrace.endpos[2usize];
        (*trace).area = 0i32;
        (*trace).planenum = 0i32;
        return qtrue
    }
    return qfalse;
}
//stores the areas the trace went through and returns the number of passed areas
#[no_mangle]
pub unsafe extern "C" fn AAS_TraceAreas(mut start: *mut vec_t,
                                        mut end: *mut vec_t,
                                        mut areas: *mut libc::c_int,
                                        mut points: *mut vec3_t,
                                        mut maxareas: libc::c_int)
 -> libc::c_int {
    let mut side: libc::c_int = 0;
    let mut nodenum: libc::c_int = 0;
    let mut tmpplanenum: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut cur_start: vec3_t = [0.; 3];
    let mut cur_end: vec3_t = [0.; 3];
    let mut cur_mid: vec3_t = [0.; 3];
    let mut tracestack: [aas_tracestack_t; 127] =
        [aas_tracestack_s{start: [0.; 3],
                          end: [0.; 3],
                          planenum: 0,
                          nodenum: 0,}; 127];
    let mut tstack_p: *mut aas_tracestack_t = 0 as *mut aas_tracestack_t;
    let mut aasnode: *mut aas_node_t = 0 as *mut aas_node_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    numareas = 0i32;
    *areas.offset(0isize) = 0i32;
    if 0 == aasworld.loaded { return numareas }
    tstack_p = tracestack.as_mut_ptr();
    (*tstack_p).start[0usize] = *start.offset(0isize);
    (*tstack_p).start[1usize] = *start.offset(1isize);
    (*tstack_p).start[2usize] = *start.offset(2isize);
    (*tstack_p).end[0usize] = *end.offset(0isize);
    (*tstack_p).end[1usize] = *end.offset(1isize);
    (*tstack_p).end[2usize] = *end.offset(2isize);
    (*tstack_p).planenum = 0i32;
    (*tstack_p).nodenum = 1i32;
    tstack_p = tstack_p.offset(1isize);
    loop  {
        tstack_p = tstack_p.offset(-1isize);
        if tstack_p < tracestack.as_mut_ptr() { return numareas }
        nodenum = (*tstack_p).nodenum;
        //if it is an area
        if nodenum < 0i32 {
            *areas.offset(numareas as isize) = -nodenum;
            if !points.is_null() {
                (*points.offset(numareas as isize))[0usize] =
                    (*tstack_p).start[0usize];
                (*points.offset(numareas as isize))[1usize] =
                    (*tstack_p).start[1usize];
                (*points.offset(numareas as isize))[2usize] =
                    (*tstack_p).start[2usize]
            }
            numareas += 1;
            if numareas >= maxareas { return numareas }
        } else {
            //end if
            //if it is a solid leaf
            if 0 == nodenum { continue ; }
            aasnode =
                &mut *aasworld.nodes.offset(nodenum as isize) as
                    *mut aas_node_t;
            cur_start[0usize] = (*tstack_p).start[0usize];
            cur_start[1usize] = (*tstack_p).start[1usize];
            cur_start[2usize] = (*tstack_p).start[2usize];
            cur_end[0usize] = (*tstack_p).end[0usize];
            cur_end[1usize] = (*tstack_p).end[1usize];
            cur_end[2usize] = (*tstack_p).end[2usize];
            plane =
                &mut *aasworld.planes.offset((*aasnode).planenum as isize) as
                    *mut aas_plane_t;
            match (*plane).type_0 { _ => { } }
            front =
                cur_start[0usize] * (*plane).normal[0usize] +
                    cur_start[1usize] * (*plane).normal[1usize] +
                    cur_start[2usize] * (*plane).normal[2usize] -
                    (*plane).dist;
            back =
                cur_end[0usize] * (*plane).normal[0usize] +
                    cur_end[1usize] * (*plane).normal[1usize] +
                    cur_end[2usize] * (*plane).normal[2usize] - (*plane).dist;
            if front > 0i32 as libc::c_float && back > 0i32 as libc::c_float {
                (*tstack_p).nodenum = (*aasnode).children[0usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceAreas: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return numareas
                }
            } else if front <= 0i32 as libc::c_float &&
                          back <= 0i32 as libc::c_float {
                (*tstack_p).nodenum = (*aasnode).children[1usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceAreas: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return numareas
                }
            } else {
                tmpplanenum = (*tstack_p).planenum;
                if front < 0i32 as libc::c_float {
                    frac = front / (front - back)
                } else { frac = front / (front - back) }
                if frac < 0i32 as libc::c_float {
                    frac = 0i32 as libc::c_float
                } else if frac > 1i32 as libc::c_float {
                    frac = 1i32 as libc::c_float
                }
                cur_mid[0usize] =
                    cur_start[0usize] +
                        (cur_end[0usize] - cur_start[0usize]) * frac;
                cur_mid[1usize] =
                    cur_start[1usize] +
                        (cur_end[1usize] - cur_start[1usize]) * frac;
                cur_mid[2usize] =
                    cur_start[2usize] +
                        (cur_end[2usize] - cur_start[2usize]) * frac;
                side = (front < 0i32 as libc::c_float) as libc::c_int;
                (*tstack_p).start[0usize] = cur_mid[0usize];
                (*tstack_p).start[1usize] = cur_mid[1usize];
                (*tstack_p).start[2usize] = cur_mid[2usize];
                (*tstack_p).planenum = (*aasnode).planenum;
                (*tstack_p).nodenum =
                    (*aasnode).children[(0 == side) as libc::c_int as usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceAreas: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return numareas
                }
                (*tstack_p).start[0usize] = cur_start[0usize];
                (*tstack_p).start[1usize] = cur_start[1usize];
                (*tstack_p).start[2usize] = cur_start[2usize];
                (*tstack_p).end[0usize] = cur_mid[0usize];
                (*tstack_p).end[1usize] = cur_mid[1usize];
                (*tstack_p).end[2usize] = cur_mid[2usize];
                (*tstack_p).planenum = tmpplanenum;
                (*tstack_p).nodenum = (*aasnode).children[side as usize];
                tstack_p = tstack_p.offset(1isize);
                if tstack_p >=
                       &mut *tracestack.as_mut_ptr().offset(127isize) as
                           *mut aas_tracestack_t {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"AAS_TraceAreas: stack overflow\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                    return numareas
                }
            }
        }
    };
}
//returns the areas the bounding box is in
#[no_mangle]
pub unsafe extern "C" fn AAS_BBoxAreas(mut absmins: *mut vec_t,
                                       mut absmaxs: *mut vec_t,
                                       mut areas: *mut libc::c_int,
                                       mut maxareas: libc::c_int)
 -> libc::c_int {
    let mut linkedareas: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut num: libc::c_int = 0;
    linkedareas = AAS_AASLinkEntity(absmins, absmaxs, -1i32);
    num = 0i32;
    link = linkedareas;
    while !link.is_null() {
        *areas.offset(num as isize) = (*link).areanum;
        num += 1;
        if num >= maxareas { break ; }
        link = (*link).next_area
    }
    AAS_UnlinkFromAreas(linkedareas);
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_UnlinkFromAreas(mut areas: *mut aas_link_t) {
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut nextlink: *mut aas_link_t = 0 as *mut aas_link_t;
    link = areas;
    while !link.is_null() {
        nextlink = (*link).next_area;
        if !(*link).prev_ent.is_null() {
            (*(*link).prev_ent).next_ent = (*link).next_ent
        } else {
            let ref mut fresh0 =
                *aasworld.arealinkedentities.offset((*link).areanum as isize);
            *fresh0 = (*link).next_ent
        }
        if !(*link).next_ent.is_null() {
            (*(*link).next_ent).prev_ent = (*link).prev_ent
        }
        AAS_DeAllocAASLink(link);
        link = nextlink
    };
}
//end of the function AAS_AllocAASLink
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_DeAllocAASLink(mut link: *mut aas_link_t) {
    if !aasworld.freelinks.is_null() { (*aasworld.freelinks).prev_ent = link }
    (*link).prev_ent = 0 as *mut aas_link_s;
    (*link).next_ent = aasworld.freelinks;
    (*link).prev_area = 0 as *mut aas_link_s;
    (*link).next_area = 0 as *mut aas_link_s;
    aasworld.freelinks = link;
    numaaslinks += 1;
}
#[no_mangle]
pub static mut numaaslinks: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn AAS_AASLinkEntity(mut absmins: *mut vec_t,
                                           mut absmaxs: *mut vec_t,
                                           mut entnum: libc::c_int)
 -> *mut aas_link_t {
    let mut side: libc::c_int = 0;
    let mut nodenum: libc::c_int = 0;
    let mut linkstack: [aas_linkstack_t; 128] =
        [aas_linkstack_t{nodenum: 0,}; 128];
    let mut lstack_p: *mut aas_linkstack_t = 0 as *mut aas_linkstack_t;
    let mut aasnode: *mut aas_node_t = 0 as *mut aas_node_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut areas: *mut aas_link_t = 0 as *mut aas_link_t;
    if 0 == aasworld.loaded {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_LinkEntity: aas not loaded\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0 as *mut aas_link_t
    }
    areas = 0 as *mut aas_link_t;
    lstack_p = linkstack.as_mut_ptr();
    (*lstack_p).nodenum = 1i32;
    lstack_p = lstack_p.offset(1isize);
    loop  {
        lstack_p = lstack_p.offset(-1isize);
        //if the trace stack is empty (ended up with a piece of the
		//line to be traced in an area)
        if lstack_p < linkstack.as_mut_ptr() { break ; }
        nodenum = (*lstack_p).nodenum;
        //if it is an area
        if nodenum < 0i32 {
            link = *aasworld.arealinkedentities.offset(-nodenum as isize);
            while !link.is_null() {
                if (*link).entnum == entnum { break ; }
                link = (*link).next_ent
            }
            //end for
            if !link.is_null() { continue ; }
            link = AAS_AllocAASLink();
            if link.is_null() { return areas }
            (*link).entnum = entnum;
            (*link).areanum = -nodenum;
            (*link).prev_area = 0 as *mut aas_link_s;
            (*link).next_area = areas;
            if !areas.is_null() { (*areas).prev_area = link }
            areas = link;
            (*link).prev_ent = 0 as *mut aas_link_s;
            (*link).next_ent =
                *aasworld.arealinkedentities.offset(-nodenum as isize);
            if !(*aasworld.arealinkedentities.offset(-nodenum as
                                                         isize)).is_null() {
                let ref mut fresh1 =
                    (**aasworld.arealinkedentities.offset(-nodenum as
                                                              isize)).prev_ent;
                *fresh1 = link
            }
            let ref mut fresh2 =
                *aasworld.arealinkedentities.offset(-nodenum as isize);
            *fresh2 = link
        } else {
            //
            //end if
            //if solid leaf
            if 0 == nodenum { continue ; }
            aasnode =
                &mut *aasworld.nodes.offset(nodenum as isize) as
                    *mut aas_node_t;
            plane =
                &mut *aasworld.planes.offset((*aasnode).planenum as isize) as
                    *mut aas_plane_t;
            side = AAS_BoxOnPlaneSide2(absmins, absmaxs, plane);
            if 0 != side & 1i32 {
                (*lstack_p).nodenum = (*aasnode).children[0usize];
                lstack_p = lstack_p.offset(1isize)
            }
            //end if
            if lstack_p >=
                   &mut *linkstack.as_mut_ptr().offset(127isize) as
                       *mut aas_linkstack_t {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"AAS_LinkEntity: stack overflow\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char);
                break ;
            } else {
                if 0 != side & 2i32 {
                    (*lstack_p).nodenum = (*aasnode).children[1usize];
                    lstack_p = lstack_p.offset(1isize)
                }
                //end if
                if !(lstack_p >=
                         &mut *linkstack.as_mut_ptr().offset(127isize) as
                             *mut aas_linkstack_t) {
                    continue ;
                }
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"AAS_LinkEntity: stack overflow\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char);
                break ;
            }
        }
    }
    return areas;
}
//end of the function AAS_TraceEndFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_BoxOnPlaneSide2(mut absmins: *mut vec_t,
                                             mut absmaxs: *mut vec_t,
                                             mut p: *mut aas_plane_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sides: libc::c_int = 0;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut corners: [vec3_t; 2] = [[0.; 3]; 2];
    i = 0i32;
    while i < 3i32 {
        if (*p).normal[i as usize] < 0i32 as libc::c_float {
            corners[0usize][i as usize] = *absmins.offset(i as isize);
            corners[1usize][i as usize] = *absmaxs.offset(i as isize)
        } else {
            corners[1usize][i as usize] = *absmins.offset(i as isize);
            corners[0usize][i as usize] = *absmaxs.offset(i as isize)
        }
        i += 1
    }
    dist1 =
        (*p).normal[0usize] * corners[0usize][0usize] +
            (*p).normal[1usize] * corners[0usize][1usize] +
            (*p).normal[2usize] * corners[0usize][2usize] - (*p).dist;
    dist2 =
        (*p).normal[0usize] * corners[1usize][0usize] +
            (*p).normal[1usize] * corners[1usize][1usize] +
            (*p).normal[2usize] * corners[1usize][2usize] - (*p).dist;
    sides = 0i32;
    if dist1 >= 0i32 as libc::c_float { sides = 1i32 }
    if dist2 < 0i32 as libc::c_float { sides |= 2i32 }
    return sides;
}
//end of the function AAS_FreeAASLinkHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AllocAASLink() -> *mut aas_link_t {
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    link = aasworld.freelinks;
    if link.is_null() {
        if 0 != botDeveloper {
            botimport.Print.expect("non-null function pointer")(4i32,
                                                                b"empty aas link heap\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        }
        return 0 as *mut aas_link_t
    }
    if !aasworld.freelinks.is_null() {
        aasworld.freelinks = (*aasworld.freelinks).next_ent
    }
    if !aasworld.freelinks.is_null() {
        (*aasworld.freelinks).prev_ent = 0 as *mut aas_link_s
    }
    numaaslinks -= 1;
    return link;
}
//return area information
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaInfo(mut areanum: libc::c_int,
                                      mut info: *mut aas_areainfo_t)
 -> libc::c_int {
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    if info.is_null() { return 0i32 }
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_AreaInfo: areanum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            areanum);
        return 0i32
    }
    settings =
        &mut *aasworld.areasettings.offset(areanum as isize) as
            *mut aas_areasettings_t;
    (*info).cluster = (*settings).cluster;
    (*info).contents = (*settings).contents;
    (*info).flags = (*settings).areaflags;
    (*info).presencetype = (*settings).presencetype;
    (*info).mins[0usize] =
        (*aasworld.areas.offset(areanum as isize)).mins[0usize];
    (*info).mins[1usize] =
        (*aasworld.areas.offset(areanum as isize)).mins[1usize];
    (*info).mins[2usize] =
        (*aasworld.areas.offset(areanum as isize)).mins[2usize];
    (*info).maxs[0usize] =
        (*aasworld.areas.offset(areanum as isize)).maxs[0usize];
    (*info).maxs[1usize] =
        (*aasworld.areas.offset(areanum as isize)).maxs[1usize];
    (*info).maxs[2usize] =
        (*aasworld.areas.offset(areanum as isize)).maxs[2usize];
    (*info).center[0usize] =
        (*aasworld.areas.offset(areanum as isize)).center[0usize];
    (*info).center[1usize] =
        (*aasworld.areas.offset(areanum as isize)).center[1usize];
    (*info).center[2usize] =
        (*aasworld.areas.offset(areanum as isize)).center[2usize];
    return ::std::mem::size_of::<aas_areainfo_t>() as libc::c_ulong as
               libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_PointReachabilityAreaIndex(mut origin:
                                                            *mut vec_t)
 -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    if 0 == aasworld.initialized { return 0i32 }
    if origin.is_null() {
        index = 0i32;
        i = 0i32;
        while i < aasworld.numclusters {
            index +=
                (*aasworld.clusters.offset(i as isize)).numreachabilityareas;
            i += 1
        }
        return index
    }
    areanum = AAS_PointAreaNum(origin);
    if 0 == areanum || 0 == AAS_AreaReachability(areanum) { return 0i32 }
    cluster = (*aasworld.areasettings.offset(areanum as isize)).cluster;
    areanum =
        (*aasworld.areasettings.offset(areanum as isize)).clusterareanum;
    if cluster < 0i32 {
        cluster = (*aasworld.portals.offset(-cluster as isize)).frontcluster;
        areanum =
            (*aasworld.portals.offset(-cluster as
                                          isize)).clusterareanum[0usize]
    }
    index = 0i32;
    i = 0i32;
    while i < cluster {
        index += (*aasworld.clusters.offset(i as isize)).numreachabilityareas;
        i += 1
    }
    index += areanum;
    return index;
}
//returns the plane the given face is in
#[no_mangle]
pub unsafe extern "C" fn AAS_FacePlane(mut facenum: libc::c_int,
                                       mut normal: *mut vec_t,
                                       mut dist: *mut libc::c_float) {
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    plane =
        &mut *aasworld.planes.offset((*aasworld.faces.offset(facenum as
                                                                 isize)).planenum
                                         as isize) as *mut aas_plane_t;
    *normal.offset(0isize) = (*plane).normal[0usize];
    *normal.offset(1isize) = (*plane).normal[1usize];
    *normal.offset(2isize) = (*plane).normal[2usize];
    *dist = (*plane).dist;
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
/* ****************************************************************************
 * name:		be_aas_sample.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_sample.h $
 *
 *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn AAS_InitAASLinkHeap() {
    let mut i: libc::c_int = 0;
    let mut max_aaslinks: libc::c_int = 0;
    max_aaslinks = aasworld.linkheapsize;
    if aasworld.linkheap.is_null() {
        max_aaslinks =
            LibVarValue(b"max_aaslinks\x00" as *const u8 as
                            *const libc::c_char,
                        b"6144\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
        if max_aaslinks < 0i32 { max_aaslinks = 0i32 }
        aasworld.linkheapsize = max_aaslinks;
        aasworld.linkheap =
            GetHunkMemory((max_aaslinks as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_link_t>()
                                                               as
                                                               libc::c_ulong))
                as *mut aas_link_t
    }
    let ref mut fresh3 = (*aasworld.linkheap.offset(0isize)).prev_ent;
    *fresh3 = 0 as *mut aas_link_s;
    let ref mut fresh4 = (*aasworld.linkheap.offset(0isize)).next_ent;
    *fresh4 = &mut *aasworld.linkheap.offset(1isize) as *mut aas_link_t;
    i = 1i32;
    while i < max_aaslinks - 1i32 {
        let ref mut fresh5 = (*aasworld.linkheap.offset(i as isize)).prev_ent;
        *fresh5 =
            &mut *aasworld.linkheap.offset((i - 1i32) as isize) as
                *mut aas_link_t;
        let ref mut fresh6 = (*aasworld.linkheap.offset(i as isize)).next_ent;
        *fresh6 =
            &mut *aasworld.linkheap.offset((i + 1i32) as isize) as
                *mut aas_link_t;
        i += 1
    }
    let ref mut fresh7 =
        (*aasworld.linkheap.offset((max_aaslinks - 1i32) as isize)).prev_ent;
    *fresh7 =
        &mut *aasworld.linkheap.offset((max_aaslinks - 2i32) as isize) as
            *mut aas_link_t;
    let ref mut fresh8 =
        (*aasworld.linkheap.offset((max_aaslinks - 1i32) as isize)).next_ent;
    *fresh8 = 0 as *mut aas_link_s;
    aasworld.freelinks =
        &mut *aasworld.linkheap.offset(0isize) as *mut aas_link_t;
    numaaslinks = max_aaslinks;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_InitAASLinkedEntities() {
    if 0 == aasworld.loaded { return }
    if !aasworld.arealinkedentities.is_null() {
        FreeMemory(aasworld.arealinkedentities as *mut libc::c_void);
    }
    aasworld.arealinkedentities =
        GetClearedHunkMemory((aasworld.numareas as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut aas_link_t>()
                                                                  as
                                                                  libc::c_ulong))
            as *mut *mut aas_link_t;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeAASLinkHeap() {
    if !aasworld.linkheap.is_null() {
        FreeMemory(aasworld.linkheap as *mut libc::c_void);
    }
    aasworld.linkheap = 0 as *mut aas_link_t;
    aasworld.linkheapsize = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeAASLinkedEntities() {
    if !aasworld.arealinkedentities.is_null() {
        FreeMemory(aasworld.arealinkedentities as *mut libc::c_void);
    }
    aasworld.arealinkedentities = 0 as *mut *mut aas_link_t;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaGroundFace(mut areanum: libc::c_int,
                                            mut point: *mut vec_t)
 -> *mut aas_face_t {
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut normal: vec3_t = [0.; 3];
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    if 0 == aasworld.loaded { return 0 as *mut aas_face_t }
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            *aasworld.faceindex.offset(((*area).firstface + i) as isize);
        face =
            &mut *aasworld.faces.offset(abs(facenum) as isize) as
                *mut aas_face_t;
        if 0 != (*face).faceflags & 4i32 {
            if (*aasworld.planes.offset((*face).planenum as
                                            isize)).normal[2usize] <
                   0i32 as libc::c_float {
                normal[0usize] = -up[0usize];
                normal[1usize] = -up[1usize];
                normal[2usize] = -up[2usize]
            } else {
                normal[0usize] = up[0usize];
                normal[1usize] = up[1usize];
                normal[2usize] = up[2usize]
            }
            if 0 !=
                   AAS_InsideFace(face, normal.as_mut_ptr(), point, 0.01f32)
                       as u64 {
                return face
            }
        }
        i += 1
    }
    return 0 as *mut aas_face_t;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_InsideFace(mut face: *mut aas_face_t,
                                        mut pnormal: *mut vec_t,
                                        mut point: *mut vec_t,
                                        mut epsilon: libc::c_float)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut firstvertex: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut v0: vec3_t = [0.; 3];
    let mut edgevec: vec3_t = [0.; 3];
    let mut pointvec: vec3_t = [0.; 3];
    let mut sepnormal: vec3_t = [0.; 3];
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    if 0 == aasworld.loaded { return qfalse }
    i = 0i32;
    while i < (*face).numedges {
        edgenum =
            *aasworld.edgeindex.offset(((*face).firstedge + i) as isize);
        edge =
            &mut *aasworld.edges.offset(abs(edgenum) as isize) as
                *mut aas_edge_t;
        firstvertex = (edgenum < 0i32) as libc::c_int;
        v0[0usize] =
            (*aasworld.vertexes.offset((*edge).v[firstvertex as usize] as
                                           isize))[0usize];
        v0[1usize] =
            (*aasworld.vertexes.offset((*edge).v[firstvertex as usize] as
                                           isize))[1usize];
        v0[2usize] =
            (*aasworld.vertexes.offset((*edge).v[firstvertex as usize] as
                                           isize))[2usize];
        edgevec[0usize] =
            (*aasworld.vertexes.offset((*edge).v[(0 == firstvertex) as
                                                     libc::c_int as usize] as
                                           isize))[0usize] - v0[0usize];
        edgevec[1usize] =
            (*aasworld.vertexes.offset((*edge).v[(0 == firstvertex) as
                                                     libc::c_int as usize] as
                                           isize))[1usize] - v0[1usize];
        edgevec[2usize] =
            (*aasworld.vertexes.offset((*edge).v[(0 == firstvertex) as
                                                     libc::c_int as usize] as
                                           isize))[2usize] - v0[2usize];
        pointvec[0usize] = *point.offset(0isize) - v0[0usize];
        pointvec[1usize] = *point.offset(1isize) - v0[1usize];
        pointvec[2usize] = *point.offset(2isize) - v0[2usize];
        sepnormal[0usize] =
            edgevec[1usize] * *pnormal.offset(2isize) -
                edgevec[2usize] * *pnormal.offset(1isize);
        sepnormal[1usize] =
            edgevec[2usize] * *pnormal.offset(0isize) -
                edgevec[0usize] * *pnormal.offset(2isize);
        sepnormal[2usize] =
            edgevec[0usize] * *pnormal.offset(1isize) -
                edgevec[1usize] * *pnormal.offset(0isize);
        if pointvec[0usize] * sepnormal[0usize] +
               pointvec[1usize] * sepnormal[1usize] +
               pointvec[2usize] * sepnormal[2usize] < -epsilon {
            return qfalse
        }
        i += 1
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_TraceEndFace(mut trace: *mut aas_trace_t)
 -> *mut aas_face_t {
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut firstface: *mut aas_face_t = 0 as *mut aas_face_t;
    if 0 == aasworld.loaded { return 0 as *mut aas_face_t }
    if 0 != (*trace).startsolid as u64 { return 0 as *mut aas_face_t }
    area =
        &mut *aasworld.areas.offset((*trace).lastarea as isize) as
            *mut aas_area_t;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            *aasworld.faceindex.offset(((*area).firstface + i) as isize);
        face =
            &mut *aasworld.faces.offset(abs(facenum) as isize) as
                *mut aas_face_t;
        if (*face).planenum & !1i32 == (*trace).planenum & !1i32 {
            if 0 !=
                   AAS_InsideFace(face,
                                  (*aasworld.planes.offset((*face).planenum as
                                                               isize)).normal.as_mut_ptr(),
                                  (*trace).endpos.as_mut_ptr(), 0.01f32) as
                       u64 {
                return face
            }
        }
        i += 1
    }
    return firstface;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_PlaneFromNum(mut planenum: libc::c_int)
 -> *mut aas_plane_t {
    if 0 == aasworld.loaded { return 0 as *mut aas_plane_t }
    return &mut *aasworld.planes.offset(planenum as isize) as
               *mut aas_plane_t;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_LinkEntityClientBBox(mut absmins: *mut vec_t,
                                                  mut absmaxs: *mut vec_t,
                                                  mut entnum: libc::c_int,
                                                  mut presencetype:
                                                      libc::c_int)
 -> *mut aas_link_t {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut newabsmins: vec3_t = [0.; 3];
    let mut newabsmaxs: vec3_t = [0.; 3];
    AAS_PresenceTypeBoundingBox(presencetype, mins.as_mut_ptr(),
                                maxs.as_mut_ptr());
    newabsmins[0usize] = *absmins.offset(0isize) - maxs[0usize];
    newabsmins[1usize] = *absmins.offset(1isize) - maxs[1usize];
    newabsmins[2usize] = *absmins.offset(2isize) - maxs[2usize];
    newabsmaxs[0usize] = *absmaxs.offset(0isize) - mins[0usize];
    newabsmaxs[1usize] = *absmaxs.offset(1isize) - mins[1usize];
    newabsmaxs[2usize] = *absmaxs.offset(2isize) - mins[2usize];
    return AAS_AASLinkEntity(newabsmins.as_mut_ptr(), newabsmaxs.as_mut_ptr(),
                             entnum);
}
#[no_mangle]
pub unsafe extern "C" fn AAS_PointInsideFace(mut facenum: libc::c_int,
                                             mut point: *mut vec_t,
                                             mut epsilon: libc::c_float)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut firstvertex: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut v1: *mut vec_t = 0 as *mut vec_t;
    let mut v2: *mut vec_t = 0 as *mut vec_t;
    let mut edgevec: vec3_t = [0.; 3];
    let mut pointvec: vec3_t = [0.; 3];
    let mut sepnormal: vec3_t = [0.; 3];
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    if 0 == aasworld.loaded { return qfalse }
    face = &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
    plane =
        &mut *aasworld.planes.offset((*face).planenum as isize) as
            *mut aas_plane_t;
    i = 0i32;
    while i < (*face).numedges {
        edgenum =
            *aasworld.edgeindex.offset(((*face).firstedge + i) as isize);
        edge =
            &mut *aasworld.edges.offset(abs(edgenum) as isize) as
                *mut aas_edge_t;
        firstvertex = (edgenum < 0i32) as libc::c_int;
        v1 =
            (*aasworld.vertexes.offset((*edge).v[firstvertex as usize] as
                                           isize)).as_mut_ptr();
        v2 =
            (*aasworld.vertexes.offset((*edge).v[(0 == firstvertex) as
                                                     libc::c_int as usize] as
                                           isize)).as_mut_ptr();
        edgevec[0usize] = *v2.offset(0isize) - *v1.offset(0isize);
        edgevec[1usize] = *v2.offset(1isize) - *v1.offset(1isize);
        edgevec[2usize] = *v2.offset(2isize) - *v1.offset(2isize);
        pointvec[0usize] = *point.offset(0isize) - *v1.offset(0isize);
        pointvec[1usize] = *point.offset(1isize) - *v1.offset(1isize);
        pointvec[2usize] = *point.offset(2isize) - *v1.offset(2isize);
        CrossProduct(edgevec.as_mut_ptr() as *const vec_t,
                     (*plane).normal.as_mut_ptr() as *const vec_t,
                     sepnormal.as_mut_ptr());
        if pointvec[0usize] * sepnormal[0usize] +
               pointvec[1usize] * sepnormal[1usize] +
               pointvec[2usize] * sepnormal[2usize] < -epsilon {
            return qfalse
        }
        i += 1
    }
    return qtrue;
}
//end of the function AAS_PointPresenceType
//===========================================================================
// calculates the minimum distance between the origin of the box and the
// given plane when both will collide on the given side of the plane
//
// normal	=	normal vector of plane to calculate distance from
// mins		=	minimums of box relative to origin
// maxs		=	maximums of box relative to origin
// side		=	side of the plane we want to calculate the distance from
//					0 normal vector side
//					1 not normal vector side
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_BoxOriginDistanceFromPlane(mut normal:
                                                            *mut vec_t,
                                                        mut mins: *mut vec_t,
                                                        mut maxs: *mut vec_t,
                                                        mut side: libc::c_int)
 -> vec_t {
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    if 0 != side {
        i = 0i32;
        while i < 3i32 {
            if *normal.offset(i as isize) as libc::c_double > 0.001f64 {
                v1[i as usize] = *maxs.offset(i as isize)
            } else if (*normal.offset(i as isize) as libc::c_double) <
                          -0.001f64 {
                v1[i as usize] = *mins.offset(i as isize)
            } else { v1[i as usize] = 0i32 as vec_t }
            i += 1
        }
    } else {
        i = 0i32;
        while i < 3i32 {
            if *normal.offset(i as isize) as libc::c_double > 0.001f64 {
                v1[i as usize] = *mins.offset(i as isize)
            } else if (*normal.offset(i as isize) as libc::c_double) <
                          -0.001f64 {
                v1[i as usize] = *maxs.offset(i as isize)
            } else { v1[i as usize] = 0i32 as vec_t }
            i += 1
        }
    }
    v2[0usize] = *normal.offset(0isize);
    v2[1usize] = *normal.offset(1isize);
    v2[2usize] = *normal.offset(2isize);
    VectorInverse(v2.as_mut_ptr());
    return v1[0usize] * v2[0usize] + v1[1usize] * v2[1usize] +
               v1[2usize] * v2[2usize];
}