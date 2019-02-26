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
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
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
    // route prediction stop events
    //no route to goal
    //stop as soon as on of the given travel types is used
    //stop when entering the given contents
    //stop when entering the given area
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_predictroute_s {
        pub endpos: vec3_t,
        pub endarea: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub endtravelflags: libc::c_int,
        pub numareas: libc::c_int,
        pub time: libc::c_int,
    }
    pub type aas_entityinfo_t = aas_entityinfo_s;
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
      "ioq3/code/botlib/be_aas_route.c"]
pub mod be_aas_route_c {
    pub type routecacheheader_t = routecacheheader_s;
    //end of the function AAS_CreateAllRoutingCache
    //===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
    //the route cache header
//this header is followed by numportalcache + numareacache aas_routingcache_t
//structures that store routing cache
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct routecacheheader_s {
        pub ident: libc::c_int,
        pub version: libc::c_int,
        pub numareas: libc::c_int,
        pub numclusters: libc::c_int,
        pub areacrc: libc::c_int,
        pub clustercrc: libc::c_int,
        pub numportalcache: libc::c_int,
        pub numareacache: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{vec_t, fileHandle_t};
    use super::be_aas_def_h::{aas_routingcache_t};
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
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
        pub fn rand() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
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
 * name:		l_memory.h
 *
 * desc:		memory management
 *
 * $Archive: /source/code/botlib/l_memory.h $
 *
 *****************************************************************************/
        //#define MEMDEBUG
        //allocate a memory block of the given size
        #[no_mangle]
        pub fn GetMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
        //returns the amount available memory
        #[no_mangle]
        pub fn AvailableMemory() -> libc::c_int;
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
      "ioq3/code/botlib/l_crc.h"]
pub mod l_crc_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CRC_ProcessString(data: *mut libc::c_uchar,
                                 length: libc::c_int) -> libc::c_ushort;
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
      "ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_def_h::{aas_t};
    extern "C" {
        //returns the current time
        #[no_mangle]
        pub fn AAS_Time() -> libc::c_float;
        //
        #[no_mangle]
        pub fn AAS_ProjectPointOntoVector(point: *mut vec_t,
                                          vStart: *mut vec_t,
                                          vEnd: *mut vec_t,
                                          vProj: *mut vec_t);
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
      "ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::be_aas_h::{aas_trace_t};
    use super::q_shared_h::{vec_t, vec3_t};
    use super::{libc};
    extern "C" {
        //returns the result of the trace of a client bbox
        #[no_mangle]
        pub fn AAS_TraceClientBBox(start: *mut vec_t, end: *mut vec_t,
                                   presencetype: libc::c_int,
                                   passent: libc::c_int) -> aas_trace_t;
        //stores the areas the trace went through and returns the number of passed areas
        #[no_mangle]
        pub fn AAS_TraceAreas(start: *mut vec_t, end: *mut vec_t,
                              areas: *mut libc::c_int, points: *mut vec3_t,
                              maxareas: libc::c_int) -> libc::c_int;
        //returns the area the point is in
        #[no_mangle]
        pub fn AAS_PointAreaNum(point: *mut vec_t) -> libc::c_int;
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
        //returns the total area of the ground faces of the given area
        #[no_mangle]
        pub fn AAS_AreaGroundFaceArea(areanum: libc::c_int) -> libc::c_float;
        //returns true if the area is crouch only
        #[no_mangle]
        pub fn AAS_AreaCrouch(areanum: libc::c_int) -> libc::c_int;
        //returns true if a player can swim in this area
        #[no_mangle]
        pub fn AAS_AreaSwim(areanum: libc::c_int) -> libc::c_int;
        //returns true if the area is donotenter
        #[no_mangle]
        pub fn AAS_AreaDoNotEnter(areanum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_route.h"]
pub mod be_aas_route_h {
    use super::{libc};
    use super::aasfile_h::{aas_reachability_s};
    use super::q_shared_h::{vec_t};
    use super::be_aas_h::{aas_predictroute_s};
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
                       FS_WRITE, FS_READ, cplane_t, Com_sprintf};
use self::aasfile_h::{aas_bbox_s, aas_bbox_t, aas_reachability_s,
                      aas_reachability_t, aas_areasettings_s,
                      aas_areasettings_t, aas_portal_s, aas_portal_t,
                      aas_portalindex_t, aas_cluster_s, aas_cluster_t,
                      aas_vertex_t, aas_plane_s, aas_plane_t, aas_edge_s,
                      aas_edge_t, aas_edgeindex_t, aas_face_s, aas_face_t,
                      aas_faceindex_t, aas_area_s, aas_area_t, aas_node_s,
                      aas_node_t};
use self::be_aas_h::{aas_trace_t, aas_trace_s, aas_entityinfo_s,
                     aas_predictroute_s, aas_entityinfo_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_aas_def_h::{aas_t, aas_s, aas_reachabilityareas_t,
                         aas_reachabilityareas_s, aas_routingcache_t,
                         aas_routingcache_s, aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s};
use self::be_aas_route_c::{routecacheheader_t, routecacheheader_s};
use self::mathcalls_h::{sqrt};
use self::string_h::{memcpy, memset};
use self::stdlib_h::{rand};
use self::l_memory_h::{GetMemory, GetClearedMemory, FreeMemory,
                       AvailableMemory};
use self::l_log_h::{Log_Write};
use self::l_crc_h::{CRC_ProcessString};
use self::l_libvar_h::{LibVarValue};
use self::be_aas_main_h::{AAS_Time, AAS_ProjectPointOntoVector, aasworld};
use self::be_aas_sample_h::{AAS_TraceClientBBox, AAS_TraceAreas,
                            AAS_PointAreaNum};
use self::be_aas_reach_h::{AAS_AreaReachability, AAS_AreaGroundFaceArea,
                           AAS_AreaCrouch, AAS_AreaSwim, AAS_AreaDoNotEnter};
use self::be_interface_h::{botimport, botDeveloper};
use self::be_variadic_h::{AAS_Error};
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
//returns the next reachability using the given model
#[no_mangle]
pub unsafe extern "C" fn AAS_NextModelReachability(mut num: libc::c_int,
                                                   mut modelnum: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if num <= 0i32 {
        num = 1i32
    } else if num >= aasworld.reachabilitysize {
        return 0i32
    } else { num += 1 }
    i = num;
    while i < aasworld.reachabilitysize {
        if (*aasworld.reachability.offset(i as isize)).traveltype &
               0xffffffi32 == 11i32 {
            if (*aasworld.reachability.offset(i as isize)).facenum == modelnum
               {
                return i
            }
        } else if (*aasworld.reachability.offset(i as isize)).traveltype &
                      0xffffffi32 == 19i32 {
            if (*aasworld.reachability.offset(i as isize)).facenum & 0xffffi32
                   == modelnum {
                return i
            }
        }
        i += 1
    }
    return 0i32;
}
//AASINTERN
//returns the travel flag for the given travel type
#[no_mangle]
pub unsafe extern "C" fn AAS_TravelFlagForType(mut traveltype: libc::c_int)
 -> libc::c_int {
    return AAS_TravelFlagForType_inline(traveltype);
}
//end of the function AAS_InitTravelFlagFromType
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
unsafe extern "C" fn AAS_TravelFlagForType_inline(mut traveltype: libc::c_int)
 -> libc::c_int {
    let mut tfl: libc::c_int = 0i32;
    if 0 != traveltype & 1i32 << 24i32 { tfl |= 0x8000000i32 }
    if 0 != traveltype & 2i32 << 24i32 { tfl |= 0x10000000i32 }
    traveltype &= 0xffffffi32;
    if traveltype < 0i32 || traveltype >= 32i32 { return 0x1i32 }
    tfl |= aasworld.travelflagfortype[traveltype as usize];
    return tfl;
}
//return the travel flag(s) for traveling through this area
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaContentsTravelFlags(mut areanum: libc::c_int)
 -> libc::c_int {
    return *aasworld.areacontentstravelflags.offset(areanum as isize);
}
//returns the index of the next reachability for the given area
#[no_mangle]
pub unsafe extern "C" fn AAS_NextAreaReachability(mut areanum: libc::c_int,
                                                  mut reachnum: libc::c_int)
 -> libc::c_int {
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    if 0 == aasworld.initialized { return 0i32 }
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_NextAreaReachability: areanum %d out of range\n\x00"
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
    if 0 == reachnum { return (*settings).firstreachablearea }
    if reachnum < (*settings).firstreachablearea {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_NextAreaReachability: reachnum < settings->firstreachableara\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0i32
    }
    reachnum += 1;
    if reachnum >=
           (*settings).firstreachablearea + (*settings).numreachableareas {
        return 0i32
    }
    return reachnum;
}
//returns the reachability with the given index
#[no_mangle]
pub unsafe extern "C" fn AAS_ReachabilityFromNum(mut num: libc::c_int,
                                                 mut reach:
                                                     *mut aas_reachability_s) {
    if 0 == aasworld.initialized {
        memset(reach as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<aas_reachability_t>() as libc::c_ulong);
        return
    }
    if num < 0i32 || num >= aasworld.reachabilitysize {
        memset(reach as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<aas_reachability_t>() as libc::c_ulong);
        return
    }
    memcpy(reach as *mut libc::c_void,
           &mut *aasworld.reachability.offset(num as isize) as
               *mut aas_reachability_t as *const libc::c_void,
           ::std::mem::size_of::<aas_reachability_t>() as libc::c_ulong);
}
//returns a random goal area and goal origin
#[no_mangle]
pub unsafe extern "C" fn AAS_RandomGoalArea(mut areanum: libc::c_int,
                                            mut travelflags: libc::c_int,
                                            mut goalareanum: *mut libc::c_int,
                                            mut goalorigin: *mut vec_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    if 0 == AAS_AreaReachability(areanum) { return qfalse as libc::c_int }
    n =
        (aasworld.numareas as libc::c_float *
             ((rand() & 0x7fffi32) as libc::c_float /
                  0x7fffi32 as libc::c_float)) as libc::c_int;
    i = 0i32;
    while i < aasworld.numareas {
        if n <= 0i32 { n = 1i32 }
        if n >= aasworld.numareas { n = 1i32 }
        if 0 != AAS_AreaReachability(n) {
            t =
                AAS_AreaTravelTimeToGoalArea(areanum,
                                             (*aasworld.areas.offset(areanum
                                                                         as
                                                                         isize)).center.as_mut_ptr(),
                                             n, travelflags);
            if t > 0i32 {
                if 0 != AAS_AreaSwim(n) {
                    *goalareanum = n;
                    *goalorigin.offset(0isize) =
                        (*aasworld.areas.offset(n as isize)).center[0usize];
                    *goalorigin.offset(1isize) =
                        (*aasworld.areas.offset(n as isize)).center[1usize];
                    *goalorigin.offset(2isize) =
                        (*aasworld.areas.offset(n as isize)).center[2usize];
                    return qtrue as libc::c_int
                }
                start[0usize] =
                    (*aasworld.areas.offset(n as isize)).center[0usize];
                start[1usize] =
                    (*aasworld.areas.offset(n as isize)).center[1usize];
                start[2usize] =
                    (*aasworld.areas.offset(n as isize)).center[2usize];
                if 0 == AAS_PointAreaNum(start.as_mut_ptr()) {
                    Log_Write(b"area %d center %f %f %f in solid?\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char, n,
                              start[0usize] as libc::c_double,
                              start[1usize] as libc::c_double,
                              start[2usize] as libc::c_double);
                }
                end[0usize] = start[0usize];
                end[1usize] = start[1usize];
                end[2usize] = start[2usize];
                end[2usize] -= 300i32 as libc::c_float;
                trace =
                    AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(),
                                        4i32, -1i32);
                if 0 == trace.startsolid as u64 &&
                       trace.fraction < 1i32 as libc::c_float &&
                       AAS_PointAreaNum(trace.endpos.as_mut_ptr()) == n {
                    if AAS_AreaGroundFaceArea(n) > 300i32 as libc::c_float {
                        *goalareanum = n;
                        *goalorigin.offset(0isize) = trace.endpos[0usize];
                        *goalorigin.offset(1isize) = trace.endpos[1usize];
                        *goalorigin.offset(2isize) = trace.endpos[2usize];
                        return qtrue as libc::c_int
                    }
                }
            }
        }
        n += 1;
        i += 1
    }
    return qfalse as libc::c_int;
}
//returns the travel time from the area to the goal area using the given travel flags
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaTravelTimeToGoalArea(mut areanum:
                                                          libc::c_int,
                                                      mut origin: *mut vec_t,
                                                      mut goalareanum:
                                                          libc::c_int,
                                                      mut travelflags:
                                                          libc::c_int)
 -> libc::c_int {
    let mut traveltime: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0i32;
    if 0 !=
           AAS_AreaRouteToGoalArea(areanum, origin, goalareanum, travelflags,
                                   &mut traveltime, &mut reachnum) {
        return traveltime
    }
    return 0i32;
}
//end of the function AAS_GetPortalRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaRouteToGoalArea(mut areanum: libc::c_int,
                                                 mut origin: *mut vec_t,
                                                 mut goalareanum: libc::c_int,
                                                 mut travelflags: libc::c_int,
                                                 mut traveltime:
                                                     *mut libc::c_int,
                                                 mut reachnum:
                                                     *mut libc::c_int)
 -> libc::c_int {
    let mut clusternum: libc::c_int = 0;
    let mut goalclusternum: libc::c_int = 0;
    let mut portalnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut bestreachnum: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut besttime: libc::c_ushort = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    let mut areacache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut portalcache: *mut aas_routingcache_t =
        0 as *mut aas_routingcache_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    if 0 == aasworld.initialized { return qfalse as libc::c_int }
    if areanum == goalareanum {
        *traveltime = 1i32;
        *reachnum = 0i32;
        return qtrue as libc::c_int
    }
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        if 0 != botDeveloper {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"AAS_AreaTravelTimeToGoalArea: areanum %d out of range\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                areanum);
        }
        return qfalse as libc::c_int
    }
    if goalareanum <= 0i32 || goalareanum >= aasworld.numareas {
        if 0 != botDeveloper {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"AAS_AreaTravelTimeToGoalArea: goalareanum %d out of range\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                goalareanum);
        }
        return qfalse as libc::c_int
    }
    if 0 ==
           (*aasworld.areasettings.offset(areanum as isize)).numreachableareas
           ||
           0 ==
               (*aasworld.areasettings.offset(goalareanum as
                                                  isize)).numreachableareas {
        return qfalse as libc::c_int
    }
    while AvailableMemory() < 1i32 * 1024i32 * 1024i32 {
        if 0 == AAS_FreeOldestCache() { break ; }
    }
    if 0 != AAS_AreaDoNotEnter(areanum) ||
           0 != AAS_AreaDoNotEnter(goalareanum) {
        travelflags |= 0x800000i32
    }
    clusternum = (*aasworld.areasettings.offset(areanum as isize)).cluster;
    goalclusternum =
        (*aasworld.areasettings.offset(goalareanum as isize)).cluster;
    if clusternum < 0i32 && goalclusternum > 0i32 {
        portal =
            &mut *aasworld.portals.offset(-clusternum as isize) as
                *mut aas_portal_t;
        if (*portal).frontcluster == goalclusternum ||
               (*portal).backcluster == goalclusternum {
            clusternum = goalclusternum
        }
    } else if clusternum > 0i32 && goalclusternum < 0i32 {
        portal =
            &mut *aasworld.portals.offset(-goalclusternum as isize) as
                *mut aas_portal_t;
        if (*portal).frontcluster == clusternum ||
               (*portal).backcluster == clusternum {
            goalclusternum = clusternum
        }
    }
    if clusternum > 0i32 && goalclusternum > 0i32 &&
           clusternum == goalclusternum {
        areacache =
            AAS_GetAreaRoutingCache(clusternum, goalareanum, travelflags);
        clusterareanum = AAS_ClusterAreaNum(clusternum, areanum);
        cluster =
            &mut *aasworld.clusters.offset(clusternum as isize) as
                *mut aas_cluster_t;
        if clusterareanum >= (*cluster).numreachabilityareas { return 0i32 }
        if *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum as
                                                             isize) as
               libc::c_int != 0i32 {
            *reachnum =
                (*aasworld.areasettings.offset(areanum as
                                                   isize)).firstreachablearea
                    +
                    *(*areacache).reachabilities.offset(clusterareanum as
                                                            isize) as
                        libc::c_int;
            if origin.is_null() {
                *traveltime =
                    *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                      as
                                                                      isize)
                        as libc::c_int;
                return qtrue as libc::c_int
            }
            reach =
                &mut *aasworld.reachability.offset(*reachnum as isize) as
                    *mut aas_reachability_t;
            *traveltime =
                *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                  as isize) as
                    libc::c_int +
                    AAS_AreaTravelTime(areanum, origin,
                                       (*reach).start.as_mut_ptr()) as
                        libc::c_int;
            return qtrue as libc::c_int
        }
    }
    clusternum = (*aasworld.areasettings.offset(areanum as isize)).cluster;
    goalclusternum =
        (*aasworld.areasettings.offset(goalareanum as isize)).cluster;
    if goalclusternum < 0i32 {
        portal =
            &mut *aasworld.portals.offset(-goalclusternum as isize) as
                *mut aas_portal_t;
        goalclusternum = (*portal).frontcluster
    }
    portalcache =
        AAS_GetPortalRoutingCache(goalclusternum, goalareanum, travelflags);
    if clusternum < 0i32 {
        *traveltime =
            *(*portalcache).traveltimes.as_mut_ptr().offset(-clusternum as
                                                                isize) as
                libc::c_int;
        *reachnum =
            (*aasworld.areasettings.offset(areanum as
                                               isize)).firstreachablearea +
                *(*portalcache).reachabilities.offset(-clusternum as isize) as
                    libc::c_int;
        return qtrue as libc::c_int
    }
    besttime = 0i32 as libc::c_ushort;
    bestreachnum = -1i32;
    cluster =
        &mut *aasworld.clusters.offset(clusternum as isize) as
            *mut aas_cluster_t;
    i = 0i32;
    while i < (*cluster).numportals {
        portalnum =
            *aasworld.portalindex.offset(((*cluster).firstportal + i) as
                                             isize);
        //if the goal area isn't reachable from the portal
        if !(0 ==
                 *(*portalcache).traveltimes.as_mut_ptr().offset(portalnum as
                                                                     isize)) {
            portal =
                &mut *aasworld.portals.offset(portalnum as isize) as
                    *mut aas_portal_t;
            areacache =
                AAS_GetAreaRoutingCache(clusternum, (*portal).areanum,
                                        travelflags);
            clusterareanum = AAS_ClusterAreaNum(clusternum, areanum);
            //if the area is NOT a reachability area
            if !(clusterareanum >= (*cluster).numreachabilityareas) {
                //if the portal is NOT reachable from this area
                if !(0 ==
                         *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                           as
                                                                           isize))
                   {
                    t =
                        (*(*portalcache).traveltimes.as_mut_ptr().offset(portalnum
                                                                             as
                                                                             isize)
                             as libc::c_int +
                             *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                               as
                                                                               isize)
                                 as libc::c_int) as libc::c_ushort;
                    t =
                        (t as libc::c_int +
                             *aasworld.portalmaxtraveltimes.offset(portalnum
                                                                       as
                                                                       isize))
                            as libc::c_ushort;
                    if !origin.is_null() {
                        *reachnum =
                            (*aasworld.areasettings.offset(areanum as
                                                               isize)).firstreachablearea
                                +
                                *(*areacache).reachabilities.offset(clusterareanum
                                                                        as
                                                                        isize)
                                    as libc::c_int;
                        reach =
                            aasworld.reachability.offset(*reachnum as isize);
                        t =
                            (t as libc::c_int +
                                 AAS_AreaTravelTime(areanum, origin,
                                                    (*reach).start.as_mut_ptr())
                                     as libc::c_int) as libc::c_ushort
                    }
                    if 0 == besttime ||
                           (t as libc::c_int) < besttime as libc::c_int {
                        bestreachnum = *reachnum;
                        besttime = t
                    }
                }
            }
        }
        i += 1
    }
    if bestreachnum < 0i32 { return qfalse as libc::c_int }
    *reachnum = bestreachnum;
    *traveltime = besttime as libc::c_int;
    return qtrue as libc::c_int;
}
//returns the travel time within the given area from start to end
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaTravelTime(mut areanum: libc::c_int,
                                            mut start: *mut vec_t,
                                            mut end: *mut vec_t)
 -> libc::c_ushort {
    let mut intdist: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    dir[0usize] = *start.offset(0isize) - *end.offset(0isize);
    dir[1usize] = *start.offset(1isize) - *end.offset(1isize);
    dir[2usize] = *start.offset(2isize) - *end.offset(2isize);
    dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
    if 0 != AAS_AreaCrouch(areanum) {
        dist *= 1.3f32
    } else if 0 != AAS_AreaSwim(areanum) {
        dist *= 1i32 as libc::c_float
    } else { dist *= 0.33f32 }
    intdist = dist as libc::c_int;
    if intdist <= 0i32 { intdist = 1i32 }
    return intdist as libc::c_ushort;
}
//end of the function AAS_RoutingInfo
//ROUTING_DEBUG
//===========================================================================
// returns the number of the area in the cluster
// assumes the given area is in the given cluster or a portal of the cluster
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
unsafe extern "C" fn AAS_ClusterAreaNum(mut cluster: libc::c_int,
                                        mut areanum: libc::c_int)
 -> libc::c_int {
    let mut side: libc::c_int = 0;
    let mut areacluster: libc::c_int = 0;
    areacluster = (*aasworld.areasettings.offset(areanum as isize)).cluster;
    if areacluster > 0i32 {
        return (*aasworld.areasettings.offset(areanum as
                                                  isize)).clusterareanum
    } else {
        side =
            ((*aasworld.portals.offset(-areacluster as isize)).frontcluster !=
                 cluster) as libc::c_int;
        return (*aasworld.portals.offset(-areacluster as
                                             isize)).clusterareanum[side as
                                                                        usize]
    };
}
//end if
//end if
//end for
//end while
//end of the function AAS_UpdateAreaRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_GetAreaRoutingCache(mut clusternum: libc::c_int,
                                                 mut areanum: libc::c_int,
                                                 mut travelflags: libc::c_int)
 -> *mut aas_routingcache_t {
    let mut clusterareanum: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut clustercache: *mut aas_routingcache_t =
        0 as *mut aas_routingcache_t;
    clusterareanum = AAS_ClusterAreaNum(clusternum, areanum);
    clustercache =
        *(*aasworld.clusterareacache.offset(clusternum as
                                                isize)).offset(clusterareanum
                                                                   as isize);
    cache = clustercache;
    while !cache.is_null() {
        //if there aren't used any undesired travel types for the cache
        if (*cache).travelflags == travelflags { break ; }
        cache = (*cache).next
    }
    if cache.is_null() {
        cache =
            AAS_AllocRoutingCache((*aasworld.clusters.offset(clusternum as
                                                                 isize)).numreachabilityareas);
        (*cache).cluster = clusternum;
        (*cache).areanum = areanum;
        (*cache).origin[0usize] =
            (*aasworld.areas.offset(areanum as isize)).center[0usize];
        (*cache).origin[1usize] =
            (*aasworld.areas.offset(areanum as isize)).center[1usize];
        (*cache).origin[2usize] =
            (*aasworld.areas.offset(areanum as isize)).center[2usize];
        (*cache).starttraveltime = 1i32 as libc::c_float;
        (*cache).travelflags = travelflags;
        (*cache).prev = 0 as *mut aas_routingcache_s;
        (*cache).next = clustercache;
        if !clustercache.is_null() { (*clustercache).prev = cache }
        let ref mut fresh0 =
            *(*aasworld.clusterareacache.offset(clusternum as
                                                    isize)).offset(clusterareanum
                                                                       as
                                                                       isize);
        *fresh0 = cache;
        AAS_UpdateAreaRoutingCache(cache);
    } else { AAS_UnlinkCache(cache); }
    (*cache).time = AAS_RoutingTime();
    (*cache).type_0 = 1i32 as byte;
    AAS_LinkCache(cache);
    return cache;
}
//end of the function AAS_UnlinkCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_LinkCache(mut cache: *mut aas_routingcache_t) {
    if !aasworld.newestcache.is_null() {
        (*aasworld.newestcache).time_next = cache;
        (*cache).time_prev = aasworld.newestcache
    } else {
        aasworld.oldestcache = cache;
        (*cache).time_prev = 0 as *mut aas_routingcache_s
    }
    (*cache).time_next = 0 as *mut aas_routingcache_s;
    aasworld.newestcache = cache;
}
//end of the function AAS_EnableRoutingArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
unsafe extern "C" fn AAS_RoutingTime() -> libc::c_float { return AAS_Time(); }
//end of the function AAS_TravelFlagForType_inline
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_UnlinkCache(mut cache: *mut aas_routingcache_t) {
    if !(*cache).time_next.is_null() {
        (*(*cache).time_next).time_prev = (*cache).time_prev
    } else { aasworld.newestcache = (*cache).time_prev }
    if !(*cache).time_prev.is_null() {
        (*(*cache).time_prev).time_next = (*cache).time_next
    } else { aasworld.oldestcache = (*cache).time_next }
    (*cache).time_next = 0 as *mut aas_routingcache_s;
    (*cache).time_prev = 0 as *mut aas_routingcache_s;
}
//end of the function AAS_FreeRoutingCaches
//===========================================================================
// update the given routing cache
//
// Parameter:			areacache		: routing cache to update
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_UpdateAreaRoutingCache(mut areacache:
                                                        *mut aas_routingcache_t) {
    let mut i: libc::c_int = 0;
    let mut nextareanum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut badtravelflags: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut linknum: libc::c_int = 0;
    let mut numreachabilityareas: libc::c_int = 0;
    //NOTE: not more than 128 reachabilities per area allowed
    let mut t: libc::c_ushort = 0;
    let mut startareatraveltimes: [libc::c_ushort; 128] = [0; 128];
    let mut updateliststart: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut updatelistend: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut curupdate: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut nextupdate: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    let mut revreach: *mut aas_reversedreachability_t =
        0 as *mut aas_reversedreachability_t;
    let mut revlink: *mut aas_reversedlink_t = 0 as *mut aas_reversedlink_t;
    numareacacheupdates += 1;
    numreachabilityareas =
        (*aasworld.clusters.offset((*areacache).cluster as
                                       isize)).numreachabilityareas;
    aasworld.frameroutingupdates += 1;
    badtravelflags = !(*areacache).travelflags;
    clusterareanum =
        AAS_ClusterAreaNum((*areacache).cluster, (*areacache).areanum);
    if clusterareanum >= numreachabilityareas { return }
    memset(startareatraveltimes.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_ushort; 128]>() as libc::c_ulong);
    curupdate =
        &mut *aasworld.areaupdate.offset(clusterareanum as isize) as
            *mut aas_routingupdate_t;
    (*curupdate).areanum = (*areacache).areanum;
    (*curupdate).areatraveltimes = startareatraveltimes.as_mut_ptr();
    (*curupdate).tmptraveltime =
        (*areacache).starttraveltime as libc::c_ushort;
    *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum as isize) =
        (*areacache).starttraveltime as libc::c_ushort;
    (*curupdate).next = 0 as *mut aas_routingupdate_s;
    (*curupdate).prev = 0 as *mut aas_routingupdate_s;
    updateliststart = curupdate;
    updatelistend = curupdate;
    while !updateliststart.is_null() {
        curupdate = updateliststart;
        if !(*curupdate).next.is_null() {
            (*(*curupdate).next).prev = 0 as *mut aas_routingupdate_s
        } else { updatelistend = 0 as *mut aas_routingupdate_t }
        updateliststart = (*curupdate).next;
        (*curupdate).inlist = qfalse;
        revreach =
            &mut *aasworld.reversedreachability.offset((*curupdate).areanum as
                                                           isize) as
                *mut aas_reversedreachability_t;
        i = 0i32;
        revlink = (*revreach).first;
        while !revlink.is_null() {
            linknum = (*revlink).linknum;
            reach =
                &mut *aasworld.reachability.offset(linknum as isize) as
                    *mut aas_reachability_t;
            //if there is used an undesired travel type
            if !(0 !=
                     AAS_TravelFlagForType_inline((*reach).traveltype) &
                         badtravelflags) {
                //if not allowed to enter the next area
                if !(0 !=
                         (*aasworld.areasettings.offset((*reach).areanum as
                                                            isize)).areaflags
                             & 8i32) {
                    //if the next area has a not allowed travel flag
                    if !(0 !=
                             AAS_AreaContentsTravelFlags_inline((*reach).areanum)
                                 & badtravelflags) {
                        nextareanum = (*revlink).areanum;
                        cluster =
                            (*aasworld.areasettings.offset(nextareanum as
                                                               isize)).cluster;
                        //don't leave the cluster
                        if !(cluster > 0i32 &&
                                 cluster != (*areacache).cluster) {
                            clusterareanum =
                                AAS_ClusterAreaNum((*areacache).cluster,
                                                   nextareanum);
                            if !(clusterareanum >= numreachabilityareas) {
                                t =
                                    ((*curupdate).tmptraveltime as libc::c_int
                                         +
                                         *(*curupdate).areatraveltimes.offset(i
                                                                                  as
                                                                                  isize)
                                             as libc::c_int +
                                         (*reach).traveltime as libc::c_int)
                                        as libc::c_ushort;
                                if 0 ==
                                       *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                                         as
                                                                                         isize)
                                       ||
                                       *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                                         as
                                                                                         isize)
                                           as libc::c_int > t as libc::c_int {
                                    *(*areacache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                                      as
                                                                                      isize)
                                        = t;
                                    *(*areacache).reachabilities.offset(clusterareanum
                                                                            as
                                                                            isize)
                                        =
                                        (linknum -
                                             (*aasworld.areasettings.offset(nextareanum
                                                                                as
                                                                                isize)).firstreachablearea)
                                            as libc::c_uchar;
                                    nextupdate =
                                        &mut *aasworld.areaupdate.offset(clusterareanum
                                                                             as
                                                                             isize)
                                            as *mut aas_routingupdate_t;
                                    (*nextupdate).areanum = nextareanum;
                                    (*nextupdate).tmptraveltime = t;
                                    (*nextupdate).areatraveltimes =
                                        *(*aasworld.areatraveltimes.offset(nextareanum
                                                                               as
                                                                               isize)).offset((linknum
                                                                                                   -
                                                                                                   (*aasworld.areasettings.offset(nextareanum
                                                                                                                                      as
                                                                                                                                      isize)).firstreachablearea)
                                                                                                  as
                                                                                                  isize);
                                    if 0 == (*nextupdate).inlist as u64 {
                                        (*nextupdate).next =
                                            0 as *mut aas_routingupdate_s;
                                        (*nextupdate).prev = updatelistend;
                                        if !updatelistend.is_null() {
                                            (*updatelistend).next = nextupdate
                                        } else {
                                            updateliststart = nextupdate
                                        }
                                        updatelistend = nextupdate;
                                        (*nextupdate).inlist = qtrue
                                    }
                                }
                            }
                        }
                    }
                }
            }
            revlink = (*revlink).next;
            i += 1
        }
    };
}
//end of the function AAS_GetAreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
unsafe extern "C" fn AAS_AreaContentsTravelFlags_inline(mut areanum:
                                                            libc::c_int)
 -> libc::c_int {
    return *aasworld.areacontentstravelflags.offset(areanum as isize);
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
 * name:		be_aas_route.c
 *
 * desc:		AAS
 *
 * $Archive: /MissionPack/code/botlib/be_aas_route.c $
 *
 *****************************************************************************/
//travel time in hundreths of a second = distance * 100 / speed
//crouch speed = 100
//should be 0.66, swim speed = 150
//walk speed = 300
//cache refresh time
//15 seconds refresh time
//maximum number of routing updates each frame
/*

  area routing cache:
  stores the distances within one cluster to a specific goal area
  this goal area is in this same cluster and could be a cluster portal
  for every cluster there's a list with routing cache for every area
  in that cluster (including the portals of that cluster)
  area cache stores aasworld.clusters[?].numreachabilityareas travel times

  portal routing cache:
  stores the distances of all portals to a specific goal area
  this goal area could be in any cluster and could also be a cluster portal
  for every area (aasworld.numareas) the portal cache stores
  aasworld.numportals travel times

*/
#[no_mangle]
pub static mut numareacacheupdates: libc::c_int = 0;
//end of the function AAS_FreeOldestCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AllocRoutingCache(mut numtraveltimes:
                                                   libc::c_int)
 -> *mut aas_routingcache_t {
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut size: libc::c_int = 0;
    size =
        (::std::mem::size_of::<aas_routingcache_t>() as
             libc::c_ulong).wrapping_add((numtraveltimes as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                              as
                                                                              libc::c_ulong)).wrapping_add((numtraveltimes
                                                                                                                as
                                                                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                                                                                as
                                                                                                                                                libc::c_ulong))
            as libc::c_int;
    routingcachesize += size;
    cache =
        GetClearedMemory(size as libc::c_ulong) as *mut aas_routingcache_t;
    (*cache).reachabilities =
        (cache as
             *mut libc::c_uchar).offset(::std::mem::size_of::<aas_routingcache_t>()
                                            as libc::c_ulong as
                                            isize).offset((numtraveltimes as
                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as isize);
    (*cache).size = size;
    return cache;
}
//ROUTING_DEBUG
#[no_mangle]
pub static mut routingcachesize: libc::c_int = 0;
//end if
//end if
//end for
//end while
//end of the function AAS_UpdatePortalRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_GetPortalRoutingCache(mut clusternum:
                                                       libc::c_int,
                                                   mut areanum: libc::c_int,
                                                   mut travelflags:
                                                       libc::c_int)
 -> *mut aas_routingcache_t {
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    cache = *aasworld.portalcache.offset(areanum as isize);
    while !cache.is_null() {
        if (*cache).travelflags == travelflags { break ; }
        cache = (*cache).next
    }
    if cache.is_null() {
        cache = AAS_AllocRoutingCache(aasworld.numportals);
        (*cache).cluster = clusternum;
        (*cache).areanum = areanum;
        (*cache).origin[0usize] =
            (*aasworld.areas.offset(areanum as isize)).center[0usize];
        (*cache).origin[1usize] =
            (*aasworld.areas.offset(areanum as isize)).center[1usize];
        (*cache).origin[2usize] =
            (*aasworld.areas.offset(areanum as isize)).center[2usize];
        (*cache).starttraveltime = 1i32 as libc::c_float;
        (*cache).travelflags = travelflags;
        (*cache).prev = 0 as *mut aas_routingcache_s;
        (*cache).next = *aasworld.portalcache.offset(areanum as isize);
        if !(*aasworld.portalcache.offset(areanum as isize)).is_null() {
            let ref mut fresh1 =
                (**aasworld.portalcache.offset(areanum as isize)).prev;
            *fresh1 = cache
        }
        let ref mut fresh2 = *aasworld.portalcache.offset(areanum as isize);
        *fresh2 = cache;
        AAS_UpdatePortalRoutingCache(cache);
    } else { AAS_UnlinkCache(cache); }
    (*cache).time = AAS_RoutingTime();
    (*cache).type_0 = 0i32 as byte;
    AAS_LinkCache(cache);
    return cache;
}
//end of the function AAS_GetAreaRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_UpdatePortalRoutingCache(mut portalcache:
                                                          *mut aas_routingcache_t) {
    let mut i: libc::c_int = 0;
    let mut portalnum: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut clusternum: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut updateliststart: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut updatelistend: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut curupdate: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut nextupdate: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    numportalcacheupdates += 1;
    curupdate =
        &mut *aasworld.portalupdate.offset(aasworld.numportals as isize) as
            *mut aas_routingupdate_t;
    (*curupdate).cluster = (*portalcache).cluster;
    (*curupdate).areanum = (*portalcache).areanum;
    (*curupdate).tmptraveltime =
        (*portalcache).starttraveltime as libc::c_ushort;
    clusternum =
        (*aasworld.areasettings.offset((*portalcache).areanum as
                                           isize)).cluster;
    if clusternum < 0i32 {
        *(*portalcache).traveltimes.as_mut_ptr().offset(-clusternum as isize)
            = (*portalcache).starttraveltime as libc::c_ushort
    }
    (*curupdate).next = 0 as *mut aas_routingupdate_s;
    (*curupdate).prev = 0 as *mut aas_routingupdate_s;
    updateliststart = curupdate;
    updatelistend = curupdate;
    while !updateliststart.is_null() {
        curupdate = updateliststart;
        if !(*curupdate).next.is_null() {
            (*(*curupdate).next).prev = 0 as *mut aas_routingupdate_s
        } else { updatelistend = 0 as *mut aas_routingupdate_t }
        updateliststart = (*curupdate).next;
        (*curupdate).inlist = qfalse;
        cluster =
            &mut *aasworld.clusters.offset((*curupdate).cluster as isize) as
                *mut aas_cluster_t;
        cache =
            AAS_GetAreaRoutingCache((*curupdate).cluster,
                                    (*curupdate).areanum,
                                    (*portalcache).travelflags);
        i = 0i32;
        while i < (*cluster).numportals {
            portalnum =
                *aasworld.portalindex.offset(((*cluster).firstportal + i) as
                                                 isize);
            portal =
                &mut *aasworld.portals.offset(portalnum as isize) as
                    *mut aas_portal_t;
            //if this is the portal of the current update continue
            if !((*portal).areanum == (*curupdate).areanum) {
                clusterareanum =
                    AAS_ClusterAreaNum((*curupdate).cluster,
                                       (*portal).areanum);
                if !(clusterareanum >= (*cluster).numreachabilityareas) {
                    t =
                        *(*cache).traveltimes.as_mut_ptr().offset(clusterareanum
                                                                      as
                                                                      isize);
                    if !(0 == t) {
                        t =
                            (t as libc::c_int +
                                 (*curupdate).tmptraveltime as libc::c_int) as
                                libc::c_ushort;
                        if 0 ==
                               *(*portalcache).traveltimes.as_mut_ptr().offset(portalnum
                                                                                   as
                                                                                   isize)
                               ||
                               *(*portalcache).traveltimes.as_mut_ptr().offset(portalnum
                                                                                   as
                                                                                   isize)
                                   as libc::c_int > t as libc::c_int {
                            *(*portalcache).traveltimes.as_mut_ptr().offset(portalnum
                                                                                as
                                                                                isize)
                                = t;
                            nextupdate =
                                &mut *aasworld.portalupdate.offset(portalnum
                                                                       as
                                                                       isize)
                                    as *mut aas_routingupdate_t;
                            if (*portal).frontcluster == (*curupdate).cluster
                               {
                                (*nextupdate).cluster = (*portal).backcluster
                            } else {
                                (*nextupdate).cluster = (*portal).frontcluster
                            }
                            (*nextupdate).areanum = (*portal).areanum;
                            (*nextupdate).tmptraveltime =
                                (t as libc::c_int +
                                     *aasworld.portalmaxtraveltimes.offset(portalnum
                                                                               as
                                                                               isize))
                                    as libc::c_ushort;
                            if 0 == (*nextupdate).inlist as u64 {
                                (*nextupdate).next =
                                    0 as *mut aas_routingupdate_s;
                                (*nextupdate).prev = updatelistend;
                                if !updatelistend.is_null() {
                                    (*updatelistend).next = nextupdate
                                } else { updateliststart = nextupdate }
                                updatelistend = nextupdate;
                                (*nextupdate).inlist = qtrue
                            }
                        }
                    }
                }
            }
            i += 1
        }
    };
}
#[no_mangle]
pub static mut numportalcacheupdates: libc::c_int = 0;
//botimport.Print(PRT_MESSAGE, "portal %d max tt = %d\n", i, aasworld.portalmaxtraveltimes[i]);
//end for
//end of the function AAS_InitPortalMaxTravelTimes
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
/*
int AAS_FreeOldestCache(void)
{
	int i, j, bestcluster, bestarea, freed;
	float besttime;
	aas_routingcache_t *cache, *bestcache;

	freed = qfalse;
	besttime = 999999999;
	bestcache = NULL;
	bestcluster = 0;
	bestarea = 0;
	//refresh cluster cache
	for (i = 0; i < aasworld.numclusters; i++)
	{
		for (j = 0; j < aasworld.clusters[i].numareas; j++)
		{
			for (cache = aasworld.clusterareacache[i][j]; cache; cache = cache->next)
			{
				//never remove cache leading towards a portal
				if (aasworld.areasettings[cache->areanum].cluster < 0) continue;
				//if this cache is older than the cache we found so far
				if (cache->time < besttime)
				{
					bestcache = cache;
					bestcluster = i;
					bestarea = j;
					besttime = cache->time;
				} //end if
			} //end for
		} //end for
	} //end for
	if (bestcache)
	{
		cache = bestcache;
		if (cache->prev) cache->prev->next = cache->next;
		else aasworld.clusterareacache[bestcluster][bestarea] = cache->next;
		if (cache->next) cache->next->prev = cache->prev;
		AAS_FreeRoutingCache(cache);
		freed = qtrue;
	} //end if
	besttime = 999999999;
	bestcache = NULL;
	bestarea = 0;
	for (i = 0; i < aasworld.numareas; i++)
	{
		//refresh portal cache
		for (cache = aasworld.portalcache[i]; cache; cache = cache->next)
		{
			if (cache->time < besttime)
			{
				bestcache = cache;
				bestarea = i;
				besttime = cache->time;
			} //end if
		} //end for
	} //end for
	if (bestcache)
	{
		cache = bestcache;
		if (cache->prev) cache->prev->next = cache->next;
		else aasworld.portalcache[bestarea] = cache->next;
		if (cache->next) cache->next->prev = cache->prev;
		AAS_FreeRoutingCache(cache);
		freed = qtrue;
	} //end if
	return freed;
} //end of the function AAS_FreeOldestCache
*/
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeOldestCache() -> libc::c_int {
    let mut clusterareanum: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    cache = aasworld.oldestcache;
    while !cache.is_null() {
        // never free area cache leading towards a portal
        if !((*cache).type_0 as libc::c_int == 1i32 &&
                 (*aasworld.areasettings.offset((*cache).areanum as
                                                    isize)).cluster < 0i32) {
            break ;
        }
        cache = (*cache).time_next
    }
    if !cache.is_null() {
        if (*cache).type_0 as libc::c_int == 1i32 {
            clusterareanum =
                AAS_ClusterAreaNum((*cache).cluster, (*cache).areanum);
            if !(*cache).prev.is_null() {
                (*(*cache).prev).next = (*cache).next
            } else {
                let ref mut fresh3 =
                    *(*aasworld.clusterareacache.offset((*cache).cluster as
                                                            isize)).offset(clusterareanum
                                                                               as
                                                                               isize);
                *fresh3 = (*cache).next
            }
            if !(*cache).next.is_null() {
                (*(*cache).next).prev = (*cache).prev
            }
        } else {
            if !(*cache).prev.is_null() {
                (*(*cache).prev).next = (*cache).next
            } else {
                let ref mut fresh4 =
                    *aasworld.portalcache.offset((*cache).areanum as isize);
                *fresh4 = (*cache).next
            }
            if !(*cache).next.is_null() {
                (*(*cache).next).prev = (*cache).prev
            }
        }
        AAS_FreeRoutingCache(cache);
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//end of the function AAS_LinkCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeRoutingCache(mut cache:
                                                  *mut aas_routingcache_t) {
    AAS_UnlinkCache(cache);
    routingcachesize -= (*cache).size;
    FreeMemory(cache as *mut libc::c_void);
}
//enable or disable an area for routing
#[no_mangle]
pub unsafe extern "C" fn AAS_EnableRoutingArea(mut areanum: libc::c_int,
                                               mut enable: libc::c_int)
 -> libc::c_int {
    let mut flags: libc::c_int = 0;
    if areanum <= 0i32 || areanum >= aasworld.numareas {
        if 0 != botDeveloper {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"AAS_EnableRoutingArea: areanum %d out of range\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                areanum);
        }
        return 0i32
    }
    flags =
        (*aasworld.areasettings.offset(areanum as isize)).areaflags & 8i32;
    if enable < 0i32 { return (0 == flags) as libc::c_int }
    if 0 != enable {
        (*aasworld.areasettings.offset(areanum as isize)).areaflags &= !8i32
    } else {
        (*aasworld.areasettings.offset(areanum as isize)).areaflags |= 8i32
    }
    if flags & 8i32 !=
           (*aasworld.areasettings.offset(areanum as isize)).areaflags & 8i32
       {
        AAS_RemoveRoutingCacheUsingArea(areanum);
    }
    return (0 == flags) as libc::c_int;
}
//end for
//end of the function AAS_RemoveRoutingCacheInCluster
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_RemoveRoutingCacheUsingArea(mut areanum:
                                                             libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut clusternum: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut nextcache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    clusternum = (*aasworld.areasettings.offset(areanum as isize)).cluster;
    if clusternum > 0i32 {
        AAS_RemoveRoutingCacheInCluster(clusternum);
    } else {
        AAS_RemoveRoutingCacheInCluster((*aasworld.portals.offset(-clusternum
                                                                      as
                                                                      isize)).frontcluster);
        AAS_RemoveRoutingCacheInCluster((*aasworld.portals.offset(-clusternum
                                                                      as
                                                                      isize)).backcluster);
    }
    i = 0i32;
    while i < aasworld.numareas {
        cache = *aasworld.portalcache.offset(i as isize);
        while !cache.is_null() {
            nextcache = (*cache).next;
            AAS_FreeRoutingCache(cache);
            cache = nextcache
        }
        let ref mut fresh5 = *aasworld.portalcache.offset(i as isize);
        *fresh5 = 0 as *mut aas_routingcache_t;
        i += 1
    };
}
//end of the function AAS_FreeRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_RemoveRoutingCacheInCluster(mut clusternum:
                                                             libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut nextcache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    if aasworld.clusterareacache.is_null() { return }
    cluster =
        &mut *aasworld.clusters.offset(clusternum as isize) as
            *mut aas_cluster_t;
    i = 0i32;
    while i < (*cluster).numareas {
        cache =
            *(*aasworld.clusterareacache.offset(clusternum as
                                                    isize)).offset(i as
                                                                       isize);
        while !cache.is_null() {
            nextcache = (*cache).next;
            AAS_FreeRoutingCache(cache);
            cache = nextcache
        }
        let ref mut fresh6 =
            *(*aasworld.clusterareacache.offset(clusternum as
                                                    isize)).offset(i as
                                                                       isize);
        *fresh6 = 0 as *mut aas_routingcache_t;
        i += 1
    };
}
//predict a route up to a stop event
#[no_mangle]
pub unsafe extern "C" fn AAS_PredictRoute(mut route: *mut aas_predictroute_s,
                                          mut areanum: libc::c_int,
                                          mut origin: *mut vec_t,
                                          mut goalareanum: libc::c_int,
                                          mut travelflags: libc::c_int,
                                          mut maxareas: libc::c_int,
                                          mut maxtime: libc::c_int,
                                          mut stopevent: libc::c_int,
                                          mut stopcontents: libc::c_int,
                                          mut stoptfl: libc::c_int,
                                          mut stopareanum: libc::c_int)
 -> libc::c_int {
    let mut curareanum: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut testareanum: libc::c_int = 0;
    let mut curorigin: vec3_t = [0.; 3];
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    let mut reachareas: *mut aas_reachabilityareas_t =
        0 as *mut aas_reachabilityareas_t;
    (*route).stopevent = 0i32;
    (*route).endarea = goalareanum;
    (*route).endcontents = 0i32;
    (*route).endtravelflags = 0i32;
    (*route).endpos[0usize] = *origin.offset(0isize);
    (*route).endpos[1usize] = *origin.offset(1isize);
    (*route).endpos[2usize] = *origin.offset(2isize);
    (*route).time = 0i32;
    curareanum = areanum;
    curorigin[0usize] = *origin.offset(0isize);
    curorigin[1usize] = *origin.offset(1isize);
    curorigin[2usize] = *origin.offset(2isize);
    i = 0i32;
    while curareanum != goalareanum && (0 == maxareas || i < maxareas) &&
              i < aasworld.numareas {
        reachnum =
            AAS_AreaReachabilityToGoalArea(curareanum, curorigin.as_mut_ptr(),
                                           goalareanum, travelflags);
        if 0 == reachnum {
            (*route).stopevent = 1i32;
            return qfalse as libc::c_int
        }
        reach =
            &mut *aasworld.reachability.offset(reachnum as isize) as
                *mut aas_reachability_t;
        if 0 != stopevent & 2i32 {
            if 0 !=
                   AAS_TravelFlagForType_inline((*reach).traveltype) & stoptfl
               {
                (*route).stopevent = 2i32;
                (*route).endarea = curareanum;
                (*route).endcontents =
                    (*aasworld.areasettings.offset(curareanum as
                                                       isize)).contents;
                (*route).endtravelflags =
                    AAS_TravelFlagForType_inline((*reach).traveltype);
                (*route).endpos[0usize] = (*reach).start[0usize];
                (*route).endpos[1usize] = (*reach).start[1usize];
                (*route).endpos[2usize] = (*reach).start[2usize];
                return qtrue as libc::c_int
            }
            if 0 !=
                   AAS_AreaContentsTravelFlags_inline((*reach).areanum) &
                       stoptfl {
                (*route).stopevent = 2i32;
                (*route).endarea = (*reach).areanum;
                (*route).endcontents =
                    (*aasworld.areasettings.offset((*reach).areanum as
                                                       isize)).contents;
                (*route).endtravelflags =
                    AAS_AreaContentsTravelFlags_inline((*reach).areanum);
                (*route).endpos[0usize] = (*reach).end[0usize];
                (*route).endpos[1usize] = (*reach).end[1usize];
                (*route).endpos[2usize] = (*reach).end[2usize];
                (*route).time +=
                    AAS_AreaTravelTime(areanum, origin,
                                       (*reach).start.as_mut_ptr()) as
                        libc::c_int;
                (*route).time += (*reach).traveltime as libc::c_int;
                return qtrue as libc::c_int
            }
        }
        reachareas =
            &mut *aasworld.reachabilityareas.offset(reachnum as isize) as
                *mut aas_reachabilityareas_t;
        j = 0i32;
        while j < (*reachareas).numareas + 1i32 {
            if j >= (*reachareas).numareas {
                testareanum = (*reach).areanum
            } else {
                testareanum =
                    *aasworld.reachabilityareaindex.offset(((*reachareas).firstarea
                                                                + j) as isize)
            }
            if 0 != stopevent & 4i32 {
                if 0 !=
                       (*aasworld.areasettings.offset(testareanum as
                                                          isize)).contents &
                           stopcontents {
                    (*route).stopevent = 4i32;
                    (*route).endarea = testareanum;
                    (*route).endcontents =
                        (*aasworld.areasettings.offset(testareanum as
                                                           isize)).contents;
                    (*route).endpos[0usize] = (*reach).end[0usize];
                    (*route).endpos[1usize] = (*reach).end[1usize];
                    (*route).endpos[2usize] = (*reach).end[2usize];
                    (*route).time +=
                        AAS_AreaTravelTime(areanum, origin,
                                           (*reach).start.as_mut_ptr()) as
                            libc::c_int;
                    (*route).time += (*reach).traveltime as libc::c_int;
                    return qtrue as libc::c_int
                }
            }
            if 0 != stopevent & 8i32 {
                if testareanum == stopareanum {
                    (*route).stopevent = 8i32;
                    (*route).endarea = testareanum;
                    (*route).endcontents =
                        (*aasworld.areasettings.offset(testareanum as
                                                           isize)).contents;
                    (*route).endpos[0usize] = (*reach).start[0usize];
                    (*route).endpos[1usize] = (*reach).start[1usize];
                    (*route).endpos[2usize] = (*reach).start[2usize];
                    return qtrue as libc::c_int
                }
            }
            j += 1
        }
        (*route).time +=
            AAS_AreaTravelTime(areanum, origin, (*reach).start.as_mut_ptr())
                as libc::c_int;
        (*route).time += (*reach).traveltime as libc::c_int;
        (*route).endarea = (*reach).areanum;
        (*route).endcontents =
            (*aasworld.areasettings.offset((*reach).areanum as
                                               isize)).contents;
        (*route).endtravelflags =
            AAS_TravelFlagForType_inline((*reach).traveltype);
        (*route).endpos[0usize] = (*reach).end[0usize];
        (*route).endpos[1usize] = (*reach).end[1usize];
        (*route).endpos[2usize] = (*reach).end[2usize];
        curareanum = (*reach).areanum;
        curorigin[0usize] = (*reach).end[0usize];
        curorigin[1usize] = (*reach).end[1usize];
        curorigin[2usize] = (*reach).end[2usize];
        //
        if 0 != maxtime && (*route).time > maxtime { break ; }
        i += 1
    }
    if curareanum != goalareanum { return qfalse as libc::c_int }
    return qtrue as libc::c_int;
}
//end of the function AAS_AreaTravelTimeToGoalArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaReachabilityToGoalArea(mut areanum:
                                                            libc::c_int,
                                                        mut origin:
                                                            *mut vec_t,
                                                        mut goalareanum:
                                                            libc::c_int,
                                                        mut travelflags:
                                                            libc::c_int)
 -> libc::c_int {
    let mut traveltime: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0i32;
    if 0 !=
           AAS_AreaRouteToGoalArea(areanum, origin, goalareanum, travelflags,
                                   &mut traveltime, &mut reachnum) {
        return reachnum
    }
    return 0i32;
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
 * name:		be_aas_route.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_route.h $
 *
 *****************************************************************************/
//initialize the AAS routing
#[no_mangle]
pub unsafe extern "C" fn AAS_InitRouting() {
    AAS_InitTravelFlagFromType();
    AAS_InitAreaContentsTravelFlags();
    AAS_InitRoutingUpdate();
    AAS_CreateReversedReachability();
    AAS_InitClusterAreaCache();
    AAS_InitPortalCache();
    AAS_CalculateAreaTravelTimes();
    AAS_InitPortalMaxTravelTimes();
    AAS_InitReachabilityAreas();
    numareacacheupdates = 0i32;
    numportalcacheupdates = 0i32;
    routingcachesize = 0i32;
    max_routingcachesize =
        1024i32 *
            LibVarValue(b"max_routingcache\x00" as *const u8 as
                            *const libc::c_char,
                        b"4096\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
    AAS_ReadRouteCache();
}
//end of the function AAS_ReadCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ReadRouteCache() -> libc::c_int {
    //, size;
    let mut i: libc::c_int = 0;
    let mut clusterareanum: libc::c_int = 0;
    let mut fp: fileHandle_t = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut routecacheheader: routecacheheader_t =
        routecacheheader_s{ident: 0,
                           version: 0,
                           numareas: 0,
                           numclusters: 0,
                           areacrc: 0,
                           clustercrc: 0,
                           numportalcache: 0,
                           numareacache: 0,};
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    Com_sprintf(filename.as_mut_ptr(), 64i32,
                b"maps/%s.rcd\x00" as *const u8 as *const libc::c_char,
                aasworld.mapname.as_mut_ptr());
    botimport.FS_FOpenFile.expect("non-null function pointer")(filename.as_mut_ptr(),
                                                               &mut fp,
                                                               FS_READ);
    if 0 == fp { return qfalse as libc::c_int }
    botimport.FS_Read.expect("non-null function pointer")(&mut routecacheheader
                                                              as
                                                              *mut routecacheheader_t
                                                              as
                                                              *mut libc::c_void,
                                                          ::std::mem::size_of::<routecacheheader_t>()
                                                              as libc::c_ulong
                                                              as libc::c_int,
                                                          fp);
    if routecacheheader.ident !=
           (('C' as i32) << 24i32) + (('R' as i32) << 16i32) +
               (('E' as i32) << 8i32) + 'M' as i32 {
        AAS_Error(b"%s is not a route cache dump\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  filename.as_mut_ptr());
        return qfalse as libc::c_int
    }
    if routecacheheader.version != 2i32 {
        AAS_Error(b"route cache dump has wrong version %d, should be %d\n\x00"
                      as *const u8 as *const libc::c_char as
                      *mut libc::c_char, routecacheheader.version, 2i32);
        return qfalse as libc::c_int
    }
    if routecacheheader.numareas != aasworld.numareas {
        return qfalse as libc::c_int
    }
    if routecacheheader.numclusters != aasworld.numclusters {
        return qfalse as libc::c_int
    }
    if routecacheheader.areacrc !=
           CRC_ProcessString(aasworld.areas as *mut libc::c_uchar,
                             (::std::mem::size_of::<aas_area_t>() as
                                  libc::c_ulong).wrapping_mul(aasworld.numareas
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::c_int) as libc::c_int {
        return qfalse as libc::c_int
    }
    if routecacheheader.clustercrc !=
           CRC_ProcessString(aasworld.clusters as *mut libc::c_uchar,
                             (::std::mem::size_of::<aas_cluster_t>() as
                                  libc::c_ulong).wrapping_mul(aasworld.numclusters
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::c_int) as libc::c_int {
        return qfalse as libc::c_int
    }
    i = 0i32;
    while i < routecacheheader.numportalcache {
        cache = AAS_ReadCache(fp);
        (*cache).next =
            *aasworld.portalcache.offset((*cache).areanum as isize);
        (*cache).prev = 0 as *mut aas_routingcache_s;
        if !(*aasworld.portalcache.offset((*cache).areanum as
                                              isize)).is_null() {
            let ref mut fresh7 =
                (**aasworld.portalcache.offset((*cache).areanum as
                                                   isize)).prev;
            *fresh7 = cache
        }
        let ref mut fresh8 =
            *aasworld.portalcache.offset((*cache).areanum as isize);
        *fresh8 = cache;
        i += 1
    }
    i = 0i32;
    while i < routecacheheader.numareacache {
        cache = AAS_ReadCache(fp);
        clusterareanum =
            AAS_ClusterAreaNum((*cache).cluster, (*cache).areanum);
        (*cache).next =
            *(*aasworld.clusterareacache.offset((*cache).cluster as
                                                    isize)).offset(clusterareanum
                                                                       as
                                                                       isize);
        (*cache).prev = 0 as *mut aas_routingcache_s;
        if !(*(*aasworld.clusterareacache.offset((*cache).cluster as
                                                     isize)).offset(clusterareanum
                                                                        as
                                                                        isize)).is_null()
           {
            let ref mut fresh9 =
                (**(*aasworld.clusterareacache.offset((*cache).cluster as
                                                          isize)).offset(clusterareanum
                                                                             as
                                                                             isize)).prev;
            *fresh9 = cache
        }
        let ref mut fresh10 =
            *(*aasworld.clusterareacache.offset((*cache).cluster as
                                                    isize)).offset(clusterareanum
                                                                       as
                                                                       isize);
        *fresh10 = cache;
        i += 1
    }
    botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
    return qtrue as libc::c_int;
}
//end of the function AAS_WriteRouteCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ReadCache(mut fp: fileHandle_t)
 -> *mut aas_routingcache_t {
    let mut size: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    botimport.FS_Read.expect("non-null function pointer")(&mut size as
                                                              *mut libc::c_int
                                                              as
                                                              *mut libc::c_void,
                                                          ::std::mem::size_of::<libc::c_int>()
                                                              as libc::c_ulong
                                                              as libc::c_int,
                                                          fp);
    cache = GetMemory(size as libc::c_ulong) as *mut aas_routingcache_t;
    (*cache).size = size;
    botimport.FS_Read.expect("non-null function pointer")((cache as
                                                               *mut libc::c_uchar).offset(::std::mem::size_of::<libc::c_int>()
                                                                                              as
                                                                                              libc::c_ulong
                                                                                              as
                                                                                              isize)
                                                              as
                                                              *mut libc::c_void,
                                                          (size as
                                                               libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_int>()
                                                                                               as
                                                                                               libc::c_ulong)
                                                              as libc::c_int,
                                                          fp);
    (*cache).reachabilities =
        (cache as
             *mut libc::c_uchar).offset(::std::mem::size_of::<aas_routingcache_t>()
                                            as libc::c_ulong as
                                            isize).offset(-(::std::mem::size_of::<libc::c_ushort>()
                                                                as
                                                                libc::c_ulong
                                                                as
                                                                isize)).offset((size
                                                                                    as
                                                                                    libc::c_ulong).wrapping_sub(::std::mem::size_of::<aas_routingcache_t>()
                                                                                                                    as
                                                                                                                    libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_ushort>()
                                                                                                                                                    as
                                                                                                                                                    libc::c_ulong).wrapping_div(3i32
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_ulong).wrapping_mul(2i32
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   isize);
    return cache;
}
#[no_mangle]
pub static mut max_routingcachesize: libc::c_int = 0;
//end of the function AAS_ReadRouteCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitReachabilityAreas() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 32] = [0; 32];
    let mut numreachareas: libc::c_int = 0;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    if !aasworld.reachabilityareas.is_null() {
        FreeMemory(aasworld.reachabilityareas as *mut libc::c_void);
    }
    if !aasworld.reachabilityareaindex.is_null() {
        FreeMemory(aasworld.reachabilityareaindex as *mut libc::c_void);
    }
    aasworld.reachabilityareas =
        GetClearedMemory((aasworld.reachabilitysize as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_reachabilityareas_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_reachabilityareas_t;
    aasworld.reachabilityareaindex =
        GetClearedMemory(((aasworld.reachabilitysize * 32i32) as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    numreachareas = 0i32;
    i = 0i32;
    while i < aasworld.reachabilitysize {
        reach =
            &mut *aasworld.reachability.offset(i as isize) as
                *mut aas_reachability_t;
        numareas = 0i32;
        match (*reach).traveltype & 0xffffffi32 {
            4 | 9 => {
                end[0usize] = (*reach).start[0usize];
                end[1usize] = (*reach).start[1usize];
                end[2usize] = (*reach).start[2usize];
                end[2usize] = (*reach).end[2usize];
                numareas =
                    AAS_TraceAreas((*reach).start.as_mut_ptr(),
                                   end.as_mut_ptr(), areas.as_mut_ptr(),
                                   0 as *mut vec3_t, 32i32)
            }
            7 => {
                start[0usize] = (*reach).end[0usize];
                start[1usize] = (*reach).end[1usize];
                start[2usize] = (*reach).end[2usize];
                start[2usize] = (*reach).start[2usize];
                numareas =
                    AAS_TraceAreas(start.as_mut_ptr(),
                                   (*reach).end.as_mut_ptr(),
                                   areas.as_mut_ptr(), 0 as *mut vec3_t,
                                   32i32)
            }
            14 => {
                numareas =
                    AAS_TraceAreas((*reach).start.as_mut_ptr(),
                                   (*reach).end.as_mut_ptr(),
                                   areas.as_mut_ptr(), 0 as *mut vec3_t,
                                   32i32)
            }
            5 | 12 | 13 | 18 | 11 | 19 | 2 | 3 | 6 | 8 | 10 | _ => { }
        }
        (*aasworld.reachabilityareas.offset(i as isize)).firstarea =
            numreachareas;
        (*aasworld.reachabilityareas.offset(i as isize)).numareas = numareas;
        j = 0i32;
        while j < numareas {
            let fresh11 = numreachareas;
            numreachareas = numreachareas + 1;
            *aasworld.reachabilityareaindex.offset(fresh11 as isize) =
                areas[j as usize];
            j += 1
        }
        i += 1
    };
}
//end of the function AAS_PortalMaxTravelTime
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitPortalMaxTravelTimes() {
    let mut i: libc::c_int = 0;
    if !aasworld.portalmaxtraveltimes.is_null() {
        FreeMemory(aasworld.portalmaxtraveltimes as *mut libc::c_void);
    }
    aasworld.portalmaxtraveltimes =
        GetClearedMemory((aasworld.numportals as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    i = 0i32;
    while i < aasworld.numportals {
        *aasworld.portalmaxtraveltimes.offset(i as isize) =
            AAS_PortalMaxTravelTime(i);
        i += 1
    };
}
//end for
//end for
//end for
//end of the function AAS_CalculateAreaTravelTimes
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_PortalMaxTravelTime(mut portalnum: libc::c_int)
 -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut maxt: libc::c_int = 0;
    let mut portal: *mut aas_portal_t = 0 as *mut aas_portal_t;
    let mut revreach: *mut aas_reversedreachability_t =
        0 as *mut aas_reversedreachability_t;
    let mut revlink: *mut aas_reversedlink_t = 0 as *mut aas_reversedlink_t;
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    portal =
        &mut *aasworld.portals.offset(portalnum as isize) as
            *mut aas_portal_t;
    revreach =
        &mut *aasworld.reversedreachability.offset((*portal).areanum as isize)
            as *mut aas_reversedreachability_t;
    settings =
        &mut *aasworld.areasettings.offset((*portal).areanum as isize) as
            *mut aas_areasettings_t;
    maxt = 0i32;
    l = 0i32;
    while l < (*settings).numreachableareas {
        n = 0i32;
        revlink = (*revreach).first;
        while !revlink.is_null() {
            t =
                *(*(*aasworld.areatraveltimes.offset((*portal).areanum as
                                                         isize)).offset(l as
                                                                            isize)).offset(n
                                                                                               as
                                                                                               isize)
                    as libc::c_int;
            if t > maxt { maxt = t }
            revlink = (*revlink).next;
            n += 1
        }
        l += 1
    }
    return maxt;
}
//end of the function AAS_AreaTravelTime
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_CalculateAreaTravelTimes() {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: vec3_t = [0.; 3];
    let mut revreach: *mut aas_reversedreachability_t =
        0 as *mut aas_reversedreachability_t;
    let mut revlink: *mut aas_reversedlink_t = 0 as *mut aas_reversedlink_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    if !aasworld.areatraveltimes.is_null() {
        FreeMemory(aasworld.areatraveltimes as *mut libc::c_void);
    }
    size =
        (aasworld.numareas as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_ushort>()
                                             as libc::c_ulong) as libc::c_int;
    i = 0i32;
    while i < aasworld.numareas {
        revreach =
            &mut *aasworld.reversedreachability.offset(i as isize) as
                *mut aas_reversedreachability_t;
        settings =
            &mut *aasworld.areasettings.offset(i as isize) as
                *mut aas_areasettings_t;
        size =
            (size as
                 libc::c_ulong).wrapping_add(((*settings).numreachableareas as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_ushort>()
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int;
        size =
            (size as
                 libc::c_ulong).wrapping_add(((*settings).numreachableareas as
                                                  libc::c_ulong).wrapping_mul(((*revreach).numlinks
                                                                                   as
                                                                                   libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_long>()
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)
                                                                                  &
                                                                                  !(::std::mem::size_of::<libc::c_long>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_sub(1i32
                                                                                                                        as
                                                                                                                        libc::c_ulong)).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                                                                                                         as
                                                                                                                                                         libc::c_ulong))
                as libc::c_int as libc::c_int;
        i += 1
    }
    ptr = GetClearedMemory(size as libc::c_ulong) as *mut libc::c_char;
    aasworld.areatraveltimes = ptr as *mut *mut *mut libc::c_ushort;
    ptr =
        ptr.offset((aasworld.numareas as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_ushort>()
                                                        as libc::c_ulong) as
                       isize);
    i = 0i32;
    while i < aasworld.numareas {
        revreach =
            &mut *aasworld.reversedreachability.offset(i as isize) as
                *mut aas_reversedreachability_t;
        settings =
            &mut *aasworld.areasettings.offset(i as isize) as
                *mut aas_areasettings_t;
        let ref mut fresh12 = *aasworld.areatraveltimes.offset(i as isize);
        *fresh12 = ptr as *mut *mut libc::c_ushort;
        ptr =
            ptr.offset(((*settings).numreachableareas as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_ushort>()
                                                            as libc::c_ulong)
                           as isize);
        l = 0i32;
        while l < (*settings).numreachableareas {
            let ref mut fresh13 =
                *(*aasworld.areatraveltimes.offset(i as
                                                       isize)).offset(l as
                                                                          isize);
            *fresh13 = ptr as *mut libc::c_ushort;
            ptr =
                ptr.offset((((*revreach).numlinks as
                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_long>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                                 as
                                                                                                 libc::c_ulong)
                                &
                                !(::std::mem::size_of::<libc::c_long>() as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                                                       as
                                                                                                       libc::c_ulong)
                               as isize);
            reach =
                &mut *aasworld.reachability.offset(((*settings).firstreachablearea
                                                        + l) as isize) as
                    *mut aas_reachability_t;
            n = 0i32;
            revlink = (*revreach).first;
            while !revlink.is_null() {
                end[0usize] =
                    (*aasworld.reachability.offset((*revlink).linknum as
                                                       isize)).end[0usize];
                end[1usize] =
                    (*aasworld.reachability.offset((*revlink).linknum as
                                                       isize)).end[1usize];
                end[2usize] =
                    (*aasworld.reachability.offset((*revlink).linknum as
                                                       isize)).end[2usize];
                *(*(*aasworld.areatraveltimes.offset(i as
                                                         isize)).offset(l as
                                                                            isize)).offset(n
                                                                                               as
                                                                                               isize)
                    =
                    AAS_AreaTravelTime(i, end.as_mut_ptr(),
                                       (*reach).start.as_mut_ptr());
                revlink = (*revlink).next;
                n += 1
            }
            l += 1
        }
        i += 1
    };
}
//end of the function AAS_FreeAllPortalCache
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitPortalCache() {
    aasworld.portalcache =
        GetClearedMemory((aasworld.numareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut aas_routingcache_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut *mut aas_routingcache_t;
}
//end of the function AAS_FreeAllClusterAreaCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitClusterAreaCache() {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    size = 0i32;
    i = 0i32;
    while i < aasworld.numclusters {
        size += (*aasworld.clusters.offset(i as isize)).numareas;
        i += 1
    }
    ptr =
        GetClearedMemory((aasworld.numclusters as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut *mut aas_routingcache_t>()
                                                              as
                                                              libc::c_ulong).wrapping_add((size
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut aas_routingcache_t>()
                                                                                                                               as
                                                                                                                               libc::c_ulong)))
            as *mut libc::c_char;
    aasworld.clusterareacache = ptr as *mut *mut *mut aas_routingcache_t;
    ptr =
        ptr.offset((aasworld.numclusters as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut *mut aas_routingcache_t>()
                                                        as libc::c_ulong) as
                       isize);
    i = 0i32;
    while i < aasworld.numclusters {
        let ref mut fresh14 = *aasworld.clusterareacache.offset(i as isize);
        *fresh14 = ptr as *mut *mut aas_routingcache_t;
        ptr =
            ptr.offset(((*aasworld.clusters.offset(i as isize)).numareas as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut aas_routingcache_t>()
                                                            as libc::c_ulong)
                           as isize);
        i += 1
    };
}
//end of the function AAS_InitAreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_CreateReversedReachability() {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut revlink: *mut aas_reversedlink_t = 0 as *mut aas_reversedlink_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !aasworld.reversedreachability.is_null() {
        FreeMemory(aasworld.reversedreachability as *mut libc::c_void);
    }
    ptr =
        GetClearedMemory((aasworld.numareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_reversedreachability_t>()
                                                              as
                                                              libc::c_ulong).wrapping_add((aasworld.reachabilitysize
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_reversedlink_t>()
                                                                                                                               as
                                                                                                                               libc::c_ulong)))
            as *mut libc::c_char;
    aasworld.reversedreachability = ptr as *mut aas_reversedreachability_t;
    ptr =
        ptr.offset((aasworld.numareas as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_reversedreachability_t>()
                                                        as libc::c_ulong) as
                       isize);
    i = 1i32;
    while i < aasworld.numareas {
        settings =
            &mut *aasworld.areasettings.offset(i as isize) as
                *mut aas_areasettings_t;
        if (*settings).numreachableareas >= 128i32 {
            botimport.Print.expect("non-null function pointer")(2i32,
                                                                b"area %d has more than 128 reachabilities\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                i);
        }
        n = 0i32;
        while n < (*settings).numreachableareas && n < 128i32 {
            reach =
                &mut *aasworld.reachability.offset(((*settings).firstreachablearea
                                                        + n) as isize) as
                    *mut aas_reachability_t;
            revlink = ptr as *mut aas_reversedlink_t;
            ptr =
                ptr.offset(::std::mem::size_of::<aas_reversedlink_t>() as
                               libc::c_ulong as isize);
            (*revlink).areanum = i;
            (*revlink).linknum = (*settings).firstreachablearea + n;
            (*revlink).next =
                (*aasworld.reversedreachability.offset((*reach).areanum as
                                                           isize)).first;
            let ref mut fresh15 =
                (*aasworld.reversedreachability.offset((*reach).areanum as
                                                           isize)).first;
            *fresh15 = revlink;
            let ref mut fresh16 =
                (*aasworld.reversedreachability.offset((*reach).areanum as
                                                           isize)).numlinks;
            *fresh16 += 1;
            n += 1
        }
        i += 1
    };
}
//end of the function AAS_InitPortalCache
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitRoutingUpdate() {
    let mut i: libc::c_int = 0;
    let mut maxreachabilityareas: libc::c_int = 0;
    if !aasworld.areaupdate.is_null() {
        FreeMemory(aasworld.areaupdate as *mut libc::c_void);
    }
    maxreachabilityareas = 0i32;
    i = 0i32;
    while i < aasworld.numclusters {
        if (*aasworld.clusters.offset(i as isize)).numreachabilityareas >
               maxreachabilityareas {
            maxreachabilityareas =
                (*aasworld.clusters.offset(i as isize)).numreachabilityareas
        }
        i += 1
    }
    aasworld.areaupdate =
        GetClearedMemory((maxreachabilityareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_routingupdate_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_routingupdate_t;
    if !aasworld.portalupdate.is_null() {
        FreeMemory(aasworld.portalupdate as *mut libc::c_void);
    }
    aasworld.portalupdate =
        GetClearedMemory(((aasworld.numportals + 1i32) as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_routingupdate_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_routingupdate_t;
}
//end of the function AAS_AreaContentsTravelFlags
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitAreaContentsTravelFlags() {
    let mut i: libc::c_int = 0;
    if !aasworld.areacontentstravelflags.is_null() {
        FreeMemory(aasworld.areacontentstravelflags as *mut libc::c_void);
    }
    aasworld.areacontentstravelflags =
        GetClearedMemory((aasworld.numareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    i = 0i32;
    while i < aasworld.numareas {
        *aasworld.areacontentstravelflags.offset(i as isize) =
            AAS_GetAreaContentsTravelFlags(i);
        i += 1
    };
}
//end of the function AAS_RoutingTime
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_GetAreaContentsTravelFlags(mut areanum:
                                                            libc::c_int)
 -> libc::c_int {
    let mut contents: libc::c_int = 0;
    let mut tfl: libc::c_int = 0;
    contents = (*aasworld.areasettings.offset(areanum as isize)).contents;
    tfl = 0i32;
    if 0 != contents & 1i32 {
        tfl |= 0x100000i32
    } else if 0 != contents & 4i32 {
        tfl |= 0x200000i32
    } else if 0 != contents & 2i32 {
        tfl |= 0x400000i32
    } else { tfl |= 0x80000i32 }
    if 0 != contents & 256i32 { tfl |= 0x800000i32 }
    if 0 != contents & 2048i32 { tfl |= 0x8000000i32 }
    if 0 != contents & 4096i32 { tfl |= 0x10000000i32 }
    if 0 !=
           (*aasworld.areasettings.offset(areanum as isize)).areaflags & 16i32
       {
        tfl |= 0x4000000i32
    }
    return tfl;
}
//end else
//end of the function AAS_ClusterAreaNum
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_InitTravelFlagFromType() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 32i32 {
        aasworld.travelflagfortype[i as usize] = 0x1i32;
        i += 1
    }
    aasworld.travelflagfortype[1usize] = 0x1i32;
    aasworld.travelflagfortype[2usize] = 0x2i32;
    aasworld.travelflagfortype[3usize] = 0x4i32;
    aasworld.travelflagfortype[4usize] = 0x8i32;
    aasworld.travelflagfortype[5usize] = 0x10i32;
    aasworld.travelflagfortype[6usize] = 0x20i32;
    aasworld.travelflagfortype[7usize] = 0x80i32;
    aasworld.travelflagfortype[8usize] = 0x100i32;
    aasworld.travelflagfortype[9usize] = 0x200i32;
    aasworld.travelflagfortype[10usize] = 0x400i32;
    aasworld.travelflagfortype[11usize] = 0x800i32;
    aasworld.travelflagfortype[12usize] = 0x1000i32;
    aasworld.travelflagfortype[13usize] = 0x2000i32;
    aasworld.travelflagfortype[14usize] = 0x4000i32;
    aasworld.travelflagfortype[15usize] = 0x8000i32;
    aasworld.travelflagfortype[16usize] = 0x10000i32;
    aasworld.travelflagfortype[17usize] = 0x20000i32;
    aasworld.travelflagfortype[18usize] = 0x40000i32;
    aasworld.travelflagfortype[19usize] = 0x1000000i32;
}
//free the AAS routing caches
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeRoutingCaches() {
    AAS_FreeAllClusterAreaCache();
    AAS_FreeAllPortalCache();
    if !aasworld.areatraveltimes.is_null() {
        FreeMemory(aasworld.areatraveltimes as *mut libc::c_void);
    }
    aasworld.areatraveltimes = 0 as *mut *mut *mut libc::c_ushort;
    if !aasworld.portalmaxtraveltimes.is_null() {
        FreeMemory(aasworld.portalmaxtraveltimes as *mut libc::c_void);
    }
    aasworld.portalmaxtraveltimes = 0 as *mut libc::c_int;
    if !aasworld.reversedreachability.is_null() {
        FreeMemory(aasworld.reversedreachability as *mut libc::c_void);
    }
    aasworld.reversedreachability = 0 as *mut aas_reversedreachability_t;
    if !aasworld.areaupdate.is_null() {
        FreeMemory(aasworld.areaupdate as *mut libc::c_void);
    }
    aasworld.areaupdate = 0 as *mut aas_routingupdate_t;
    if !aasworld.portalupdate.is_null() {
        FreeMemory(aasworld.portalupdate as *mut libc::c_void);
    }
    aasworld.portalupdate = 0 as *mut aas_routingupdate_t;
    if !aasworld.reachabilityareas.is_null() {
        FreeMemory(aasworld.reachabilityareas as *mut libc::c_void);
    }
    aasworld.reachabilityareas = 0 as *mut aas_reachabilityareas_t;
    if !aasworld.reachabilityareaindex.is_null() {
        FreeMemory(aasworld.reachabilityareaindex as *mut libc::c_void);
    }
    aasworld.reachabilityareaindex = 0 as *mut libc::c_int;
    if !aasworld.areacontentstravelflags.is_null() {
        FreeMemory(aasworld.areacontentstravelflags as *mut libc::c_void);
    }
    aasworld.areacontentstravelflags = 0 as *mut libc::c_int;
}
//end for
//end of the function AAS_InitClusterAreaCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeAllPortalCache() {
    let mut i: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut nextcache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    if aasworld.portalcache.is_null() { return }
    i = 0i32;
    while i < aasworld.numareas {
        cache = *aasworld.portalcache.offset(i as isize);
        while !cache.is_null() {
            nextcache = (*cache).next;
            AAS_FreeRoutingCache(cache);
            cache = nextcache
        }
        let ref mut fresh17 = *aasworld.portalcache.offset(i as isize);
        *fresh17 = 0 as *mut aas_routingcache_t;
        i += 1
    }
    FreeMemory(aasworld.portalcache as *mut libc::c_void);
    aasworld.portalcache = 0 as *mut *mut aas_routingcache_t;
}
//end of the function AAS_AllocRoutingCache
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeAllClusterAreaCache() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut nextcache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    if aasworld.clusterareacache.is_null() { return }
    i = 0i32;
    while i < aasworld.numclusters {
        cluster =
            &mut *aasworld.clusters.offset(i as isize) as *mut aas_cluster_t;
        j = 0i32;
        while j < (*cluster).numareas {
            cache =
                *(*aasworld.clusterareacache.offset(i as
                                                        isize)).offset(j as
                                                                           isize);
            while !cache.is_null() {
                nextcache = (*cache).next;
                AAS_FreeRoutingCache(cache);
                cache = nextcache
            }
            let ref mut fresh18 =
                *(*aasworld.clusterareacache.offset(i as
                                                        isize)).offset(j as
                                                                           isize);
            *fresh18 = 0 as *mut aas_routingcache_t;
            j += 1
        }
        i += 1
    }
    FreeMemory(aasworld.clusterareacache as *mut libc::c_void);
    aasworld.clusterareacache = 0 as *mut *mut *mut aas_routingcache_t;
}
//returns the travel time from start to end in the given area
//
#[no_mangle]
pub unsafe extern "C" fn AAS_CreateAllRoutingCache() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    aasworld.initialized = qtrue as libc::c_int;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"AAS_CreateAllRoutingCache\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
    i = 1i32;
    while i < aasworld.numareas {
        if !(0 == AAS_AreaReachability(i)) {
            j = 1i32;
            while j < aasworld.numareas {
                if !(i == j) {
                    if !(0 == AAS_AreaReachability(j)) {
                        AAS_AreaTravelTimeToGoalArea(i,
                                                     (*aasworld.areas.offset(i
                                                                                 as
                                                                                 isize)).center.as_mut_ptr(),
                                                     j,
                                                     0x2i32 | 0x4i32 | 0x8i32
                                                         | 0x10i32 | 0x20i32 |
                                                         0x80i32 | 0x100i32 |
                                                         0x200i32 | 0x400i32 |
                                                         0x800i32 | 0x80000i32
                                                         | 0x100000i32 |
                                                         0x40000i32 |
                                                         0x1000000i32);
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    aasworld.initialized = qfalse as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_WriteRouteCache() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numportalcache: libc::c_int = 0;
    let mut numareacache: libc::c_int = 0;
    let mut totalsize: libc::c_int = 0;
    let mut cache: *mut aas_routingcache_t = 0 as *mut aas_routingcache_t;
    let mut cluster: *mut aas_cluster_t = 0 as *mut aas_cluster_t;
    let mut fp: fileHandle_t = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut routecacheheader: routecacheheader_t =
        routecacheheader_s{ident: 0,
                           version: 0,
                           numareas: 0,
                           numclusters: 0,
                           areacrc: 0,
                           clustercrc: 0,
                           numportalcache: 0,
                           numareacache: 0,};
    numportalcache = 0i32;
    i = 0i32;
    while i < aasworld.numareas {
        cache = *aasworld.portalcache.offset(i as isize);
        while !cache.is_null() { numportalcache += 1; cache = (*cache).next }
        i += 1
    }
    numareacache = 0i32;
    i = 0i32;
    while i < aasworld.numclusters {
        cluster =
            &mut *aasworld.clusters.offset(i as isize) as *mut aas_cluster_t;
        j = 0i32;
        while j < (*cluster).numareas {
            cache =
                *(*aasworld.clusterareacache.offset(i as
                                                        isize)).offset(j as
                                                                           isize);
            while !cache.is_null() {
                numareacache += 1;
                cache = (*cache).next
            }
            j += 1
        }
        i += 1
    }
    Com_sprintf(filename.as_mut_ptr(), 64i32,
                b"maps/%s.rcd\x00" as *const u8 as *const libc::c_char,
                aasworld.mapname.as_mut_ptr());
    botimport.FS_FOpenFile.expect("non-null function pointer")(filename.as_mut_ptr(),
                                                               &mut fp,
                                                               FS_WRITE);
    if 0 == fp {
        AAS_Error(b"Unable to open file: %s\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  filename.as_mut_ptr());
        return
    }
    routecacheheader.ident =
        (('C' as i32) << 24i32) + (('R' as i32) << 16i32) +
            (('E' as i32) << 8i32) + 'M' as i32;
    routecacheheader.version = 2i32;
    routecacheheader.numareas = aasworld.numareas;
    routecacheheader.numclusters = aasworld.numclusters;
    routecacheheader.areacrc =
        CRC_ProcessString(aasworld.areas as *mut libc::c_uchar,
                          (::std::mem::size_of::<aas_area_t>() as
                               libc::c_ulong).wrapping_mul(aasworld.numareas
                                                               as
                                                               libc::c_ulong)
                              as libc::c_int) as libc::c_int;
    routecacheheader.clustercrc =
        CRC_ProcessString(aasworld.clusters as *mut libc::c_uchar,
                          (::std::mem::size_of::<aas_cluster_t>() as
                               libc::c_ulong).wrapping_mul(aasworld.numclusters
                                                               as
                                                               libc::c_ulong)
                              as libc::c_int) as libc::c_int;
    routecacheheader.numportalcache = numportalcache;
    routecacheheader.numareacache = numareacache;
    botimport.FS_Write.expect("non-null function pointer")(&mut routecacheheader
                                                               as
                                                               *mut routecacheheader_t
                                                               as
                                                               *const libc::c_void,
                                                           ::std::mem::size_of::<routecacheheader_t>()
                                                               as
                                                               libc::c_ulong
                                                               as libc::c_int,
                                                           fp);
    totalsize = 0i32;
    i = 0i32;
    while i < aasworld.numareas {
        cache = *aasworld.portalcache.offset(i as isize);
        while !cache.is_null() {
            botimport.FS_Write.expect("non-null function pointer")(cache as
                                                                       *const libc::c_void,
                                                                   (*cache).size,
                                                                   fp);
            totalsize += (*cache).size;
            cache = (*cache).next
        }
        i += 1
    }
    i = 0i32;
    while i < aasworld.numclusters {
        cluster =
            &mut *aasworld.clusters.offset(i as isize) as *mut aas_cluster_t;
        j = 0i32;
        while j < (*cluster).numareas {
            cache =
                *(*aasworld.clusterareacache.offset(i as
                                                        isize)).offset(j as
                                                                           isize);
            while !cache.is_null() {
                botimport.FS_Write.expect("non-null function pointer")(cache
                                                                           as
                                                                           *const libc::c_void,
                                                                       (*cache).size,
                                                                       fp);
                totalsize += (*cache).size;
                cache = (*cache).next
            }
            j += 1
        }
        i += 1
    }
    botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"\nroute cache written to %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename.as_mut_ptr());
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"written %d bytes of routing cache\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        totalsize);
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_RoutingInfo() {
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%d area cache updates\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        numareacacheupdates);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%d portal cache updates\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        numportalcacheupdates);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%d bytes routing cache\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        routingcachesize);
}
//end of the function AAS_PredictRoute
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_BridgeWalkable(mut areanum: libc::c_int)
 -> libc::c_int {
    return qfalse as libc::c_int;
}
//end of the function AAS_RandomGoalArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaVisible(mut srcarea: libc::c_int,
                                         mut destarea: libc::c_int)
 -> libc::c_int {
    return qfalse as libc::c_int;
}
//end of the function AAS_AreaVisible
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn DistancePointToLine(mut v1: *mut vec_t,
                                             mut v2: *mut vec_t,
                                             mut point: *mut vec_t)
 -> libc::c_float {
    let mut vec: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    AAS_ProjectPointOntoVector(point, v1, v2, p2.as_mut_ptr());
    vec[0usize] = *point.offset(0isize) - p2[0usize];
    vec[1usize] = *point.offset(1isize) - p2[1usize];
    vec[2usize] = *point.offset(2isize) - p2[2usize];
    return VectorLength(vec.as_mut_ptr() as *const vec_t);
}
//end of the function DistancePointToLine
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_NearestHideArea(mut srcnum: libc::c_int,
                                             mut origin: *mut vec_t,
                                             mut areanum: libc::c_int,
                                             mut enemynum: libc::c_int,
                                             mut enemyorigin: *mut vec_t,
                                             mut enemyareanum: libc::c_int,
                                             mut travelflags: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nextareanum: libc::c_int = 0;
    let mut badtravelflags: libc::c_int = 0;
    let mut numreach: libc::c_int = 0;
    let mut bestarea: libc::c_int = 0;
    let mut t: libc::c_ushort = 0;
    let mut besttraveltime: libc::c_ushort = 0;
    static mut hidetraveltimes: *mut libc::c_ushort =
        0 as *const libc::c_ushort as *mut libc::c_ushort;
    let mut updateliststart: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut updatelistend: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut curupdate: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut nextupdate: *mut aas_routingupdate_t =
        0 as *mut aas_routingupdate_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut p: vec3_t = [0.; 3];
    let mut startVisible: qboolean = qfalse;
    if hidetraveltimes.is_null() {
        hidetraveltimes =
            GetClearedMemory((aasworld.numareas as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                  as
                                                                  libc::c_ulong))
                as *mut libc::c_ushort
    } else {
        memset(hidetraveltimes as *mut libc::c_void, 0i32,
               (aasworld.numareas as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                    as libc::c_ulong));
    }
    besttraveltime = 0i32 as libc::c_ushort;
    bestarea = 0i32;
    startVisible = qtrue;
    badtravelflags = !travelflags;
    curupdate =
        &mut *aasworld.areaupdate.offset(areanum as isize) as
            *mut aas_routingupdate_t;
    (*curupdate).areanum = areanum;
    (*curupdate).start[0usize] = *origin.offset(0isize);
    (*curupdate).start[1usize] = *origin.offset(1isize);
    (*curupdate).start[2usize] = *origin.offset(2isize);
    (*curupdate).areatraveltimes =
        *(*aasworld.areatraveltimes.offset(areanum as isize)).offset(0isize);
    (*curupdate).tmptraveltime = 0i32 as libc::c_ushort;
    (*curupdate).next = 0 as *mut aas_routingupdate_s;
    (*curupdate).prev = 0 as *mut aas_routingupdate_s;
    updateliststart = curupdate;
    updatelistend = curupdate;
    while !updateliststart.is_null() {
        curupdate = updateliststart;
        if !(*curupdate).next.is_null() {
            (*(*curupdate).next).prev = 0 as *mut aas_routingupdate_s
        } else { updatelistend = 0 as *mut aas_routingupdate_t }
        updateliststart = (*curupdate).next;
        (*curupdate).inlist = qfalse;
        numreach =
            (*aasworld.areasettings.offset((*curupdate).areanum as
                                               isize)).numreachableareas;
        reach =
            &mut *aasworld.reachability.offset((*aasworld.areasettings.offset((*curupdate).areanum
                                                                                  as
                                                                                  isize)).firstreachablearea
                                                   as isize) as
                *mut aas_reachability_t;
        i = 0i32;
        while i < numreach {
            //if an undesired travel type is used
            if !(0 !=
                     AAS_TravelFlagForType_inline((*reach).traveltype) &
                         badtravelflags) {
                //
                if !(0 !=
                         AAS_AreaContentsTravelFlags_inline((*reach).areanum)
                             & badtravelflags) {
                    nextareanum = (*reach).areanum;
                    // if this moves us into the enemies area, skip it
                    if !(nextareanum == enemyareanum) {
                        t =
                            ((*curupdate).tmptraveltime as libc::c_int +
                                 AAS_AreaTravelTime((*curupdate).areanum,
                                                    (*curupdate).start.as_mut_ptr(),
                                                    (*reach).start.as_mut_ptr())
                                     as libc::c_int +
                                 (*reach).traveltime as libc::c_int) as
                                libc::c_ushort;
                        AAS_ProjectPointOntoVector(enemyorigin,
                                                   (*curupdate).start.as_mut_ptr(),
                                                   (*reach).end.as_mut_ptr(),
                                                   p.as_mut_ptr());
                        j = 0i32;
                        while j < 3i32 {
                            if p[j as usize] > (*curupdate).start[j as usize]
                                   && p[j as usize] > (*reach).end[j as usize]
                                   ||
                                   p[j as usize] <
                                       (*curupdate).start[j as usize] &&
                                       p[j as usize] <
                                           (*reach).end[j as usize] {
                                break ;
                            }
                            j += 1
                        }
                        if j < 3i32 {
                            v2[0usize] =
                                *enemyorigin.offset(0isize) -
                                    (*reach).end[0usize];
                            v2[1usize] =
                                *enemyorigin.offset(1isize) -
                                    (*reach).end[1usize];
                            v2[2usize] =
                                *enemyorigin.offset(2isize) -
                                    (*reach).end[2usize]
                        } else {
                            v2[0usize] =
                                *enemyorigin.offset(0isize) - p[0usize];
                            v2[1usize] =
                                *enemyorigin.offset(1isize) - p[1usize];
                            v2[2usize] =
                                *enemyorigin.offset(2isize) - p[2usize]
                        }
                        dist2 = VectorLength(v2.as_mut_ptr() as *const vec_t);
                        //never go through the enemy
                        if !(dist2 < 40i32 as libc::c_float) {
                            v1[0usize] =
                                *enemyorigin.offset(0isize) -
                                    (*curupdate).start[0usize];
                            v1[1usize] =
                                *enemyorigin.offset(1isize) -
                                    (*curupdate).start[1usize];
                            v1[2usize] =
                                *enemyorigin.offset(2isize) -
                                    (*curupdate).start[2usize];
                            dist1 =
                                VectorLength(v1.as_mut_ptr() as *const vec_t);
                            if dist2 < dist1 {
                                t =
                                    (t as libc::c_float +
                                         (dist1 - dist2) *
                                             10i32 as libc::c_float) as
                                        libc::c_ushort
                            }
                            // if we weren't visible when starting, make sure we don't move into their view
                            if !(0 == startVisible as u64 &&
                                     0 !=
                                         AAS_AreaVisible(enemyareanum,
                                                         nextareanum)) {
                                //
                                if !(0 != besttraveltime as libc::c_int &&
                                         t as libc::c_int >=
                                             besttraveltime as libc::c_int) {
                                    if 0 ==
                                           *hidetraveltimes.offset(nextareanum
                                                                       as
                                                                       isize)
                                           ||
                                           *hidetraveltimes.offset(nextareanum
                                                                       as
                                                                       isize)
                                               as libc::c_int >
                                               t as libc::c_int {
                                        if 0 ==
                                               AAS_AreaVisible(enemyareanum,
                                                               nextareanum) {
                                            besttraveltime = t;
                                            bestarea = nextareanum
                                        }
                                        *hidetraveltimes.offset(nextareanum as
                                                                    isize) =
                                            t;
                                        nextupdate =
                                            &mut *aasworld.areaupdate.offset(nextareanum
                                                                                 as
                                                                                 isize)
                                                as *mut aas_routingupdate_t;
                                        (*nextupdate).areanum = nextareanum;
                                        (*nextupdate).tmptraveltime = t;
                                        (*nextupdate).start[0usize] =
                                            (*reach).end[0usize];
                                        (*nextupdate).start[1usize] =
                                            (*reach).end[1usize];
                                        (*nextupdate).start[2usize] =
                                            (*reach).end[2usize];
                                        if 0 == (*nextupdate).inlist as u64 {
                                            (*nextupdate).next =
                                                0 as *mut aas_routingupdate_s;
                                            (*nextupdate).prev =
                                                updatelistend;
                                            if !updatelistend.is_null() {
                                                (*updatelistend).next =
                                                    nextupdate
                                            } else {
                                                updateliststart = nextupdate
                                            }
                                            updatelistend = nextupdate;
                                            (*nextupdate).inlist = qtrue
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
            reach = reach.offset(1isize)
        }
    }
    return bestarea;
}