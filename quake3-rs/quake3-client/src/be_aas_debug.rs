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
    // client movement prediction stop events, stop as soon as:
    // the ground is hit
    // there's no ground
    // water is entered
    // slime is entered
    // lava is entered
    // the ground is hit with damage
    // there's a gap
    // touching a jump pad area
    // touching teleporter
    // the given stoparea is entered
    // a ground face in the area is hit
    // hit the specified bounding box
    // touching a cluster portal
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_clientmove_s {
        pub endpos: vec3_t,
        pub endarea: libc::c_int,
        pub velocity: vec3_t,
        pub trace: aas_trace_t,
        pub presencetype: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub time: libc::c_float,
        pub frames: libc::c_int,
    }
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
    pub type aas_entityinfo_t = aas_entityinfo_s;
    pub type aas_clientmove_t = aas_clientmove_s;
    use super::q_shared_h::{vec3_t, qboolean};
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
    pub type aas_settings_t = aas_settings_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_settings_s {
        pub phys_gravitydirection: vec3_t,
        pub phys_friction: libc::c_float,
        pub phys_stopspeed: libc::c_float,
        pub phys_gravity: libc::c_float,
        pub phys_waterfriction: libc::c_float,
        pub phys_watergravity: libc::c_float,
        pub phys_maxvelocity: libc::c_float,
        pub phys_maxwalkvelocity: libc::c_float,
        pub phys_maxcrouchvelocity: libc::c_float,
        pub phys_maxswimvelocity: libc::c_float,
        pub phys_walkaccelerate: libc::c_float,
        pub phys_airaccelerate: libc::c_float,
        pub phys_swimaccelerate: libc::c_float,
        pub phys_maxstep: libc::c_float,
        pub phys_maxsteepness: libc::c_float,
        pub phys_maxwaterjump: libc::c_float,
        pub phys_maxbarrier: libc::c_float,
        pub phys_jumpvel: libc::c_float,
        pub phys_falldelta5: libc::c_float,
        pub phys_falldelta10: libc::c_float,
        pub rs_waterjump: libc::c_float,
        pub rs_teleport: libc::c_float,
        pub rs_barrierjump: libc::c_float,
        pub rs_startcrouch: libc::c_float,
        pub rs_startgrapple: libc::c_float,
        pub rs_startwalkoffledge: libc::c_float,
        pub rs_startjump: libc::c_float,
        pub rs_rocketjump: libc::c_float,
        pub rs_bfgjump: libc::c_float,
        pub rs_jumppad: libc::c_float,
        pub rs_aircontrolledjumppad: libc::c_float,
        pub rs_funcbob: libc::c_float,
        pub rs_startelevator: libc::c_float,
        pub rs_falldamage5: libc::c_float,
        pub rs_falldamage10: libc::c_float,
        pub rs_maxfallheight: libc::c_float,
        pub rs_maxjumpfallheight: libc::c_float,
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
    use super::{libc};
    use super::be_aas_def_h::{aas_t};
    extern "C" {
        //returns the current time
        #[no_mangle]
        pub fn AAS_Time() -> libc::c_float;
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
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //returns the cluster the area is in (negative portal number if the area is a portal)
        #[no_mangle]
        pub fn AAS_AreaCluster(areanum: libc::c_int) -> libc::c_int;
        //returns the area the point is in
        #[no_mangle]
        pub fn AAS_PointAreaNum(point: *mut vec_t) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_debug.h"]
pub mod be_aas_debug_h {
    use super::q_shared_h::{vec_t};
    use super::{libc};
    use super::aasfile_h::{aas_reachability_s};
}
#[header_src =
      "ioq3/code/botlib/be_aas_debug.c"]
pub mod be_aas_debug_c {
    use super::{libc};
    use super::q_shared_h::{vec3_t, vec_t};
}
#[header_src =
      "ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    use super::{libc};
    use super::be_aas_h::{aas_clientmove_s};
    use super::q_shared_h::{vec_t};
    use super::aasfile_h::{aas_reachability_s};
    use super::be_aas_def_h::{aas_settings_t};
    extern "C" {
        //AASINTERN
        //movement prediction
        #[no_mangle]
        pub fn AAS_PredictClientMovement(move_0: *mut aas_clientmove_s,
                                         entnum: libc::c_int,
                                         origin: *mut vec_t,
                                         presencetype: libc::c_int,
                                         onground: libc::c_int,
                                         velocity: *mut vec_t,
                                         cmdmove: *mut vec_t,
                                         cmdframes: libc::c_int,
                                         maxframes: libc::c_int,
                                         frametime: libc::c_float,
                                         stopevent: libc::c_int,
                                         stopareanum: libc::c_int,
                                         visualize: libc::c_int)
         -> libc::c_int;
        //calculates the horizontal velocity needed for a jump and returns true this velocity could be calculated
        #[no_mangle]
        pub fn AAS_HorizontalVelocityForJump(zvel: libc::c_float,
                                             start: *mut vec_t,
                                             end: *mut vec_t,
                                             velocity: *mut libc::c_float)
         -> libc::c_int;
        //rocket jump Z velocity when rocket-jumping at origin
        #[no_mangle]
        pub fn AAS_RocketJumpZVelocity(origin: *mut vec_t) -> libc::c_float;
        //returns the jump reachability run start point
        #[no_mangle]
        pub fn AAS_JumpReachRunStart(reach: *mut aas_reachability_s,
                                     runstart: *mut vec_t);
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
        #[no_mangle]
        pub static mut aassettings: aas_settings_t;
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
use self::be_aas_h::{aas_clientmove_s, aas_trace_t, aas_trace_s,
                     aas_entityinfo_s, aas_entityinfo_t, aas_clientmove_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_aas_def_h::{aas_t, aas_s, aas_reachabilityareas_t,
                         aas_reachabilityareas_s, aas_routingcache_t,
                         aas_routingcache_s, aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s,
                         aas_settings_t, aas_settings_s};
use self::string_h::{memcpy};
use self::stdlib_h::{abs};
use self::l_memory_h::{GetClearedMemory, FreeMemory};
use self::be_interface_h::{botimport};
use self::be_aas_main_h::{AAS_Time, aasworld};
use self::be_aas_sample_h::{AAS_AreaCluster, AAS_PointAreaNum};
use self::be_aas_move_h::{AAS_PredictClientMovement,
                          AAS_HorizontalVelocityForJump,
                          AAS_RocketJumpZVelocity, AAS_JumpReachRunStart,
                          aassettings};
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
//clear the shown debug lines
#[no_mangle]
pub unsafe extern "C" fn AAS_ClearShownDebugLines() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 1024i32 {
        if 0 != debuglines[i as usize] {
            botimport.DebugLineDelete.expect("non-null function pointer")(debuglines[i
                                                                                         as
                                                                                         usize]);
            debuglines[i as usize] = 0i32;
            debuglinevisible[i as usize] = qfalse as libc::c_int
        }
        i += 1
    };
}
#[no_mangle]
pub static mut debuglinevisible: [libc::c_int; 1024] = [0; 1024];
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
 * name:		be_aas_debug.c
 *
 * desc:		AAS debug code
 *
 * $Archive: /MissionPack/code/botlib/be_aas_debug.c $
 *
 *****************************************************************************/
#[no_mangle]
pub static mut debuglines: [libc::c_int; 1024] = [0; 1024];
//
#[no_mangle]
pub unsafe extern "C" fn AAS_ClearShownPolygons() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8192i32 {
        if 0 != debugpolygons[i as usize] {
            botimport.DebugPolygonDelete.expect("non-null function pointer")(debugpolygons[i
                                                                                               as
                                                                                               usize]);
        }
        debugpolygons[i as usize] = 0i32;
        i += 1
    };
}
static mut debugpolygons: [libc::c_int; 8192] = [0; 8192];
//show a debug line
#[no_mangle]
pub unsafe extern "C" fn AAS_DebugLine(mut start: *mut vec_t,
                                       mut end: *mut vec_t,
                                       mut color: libc::c_int) {
    let mut line: libc::c_int = 0;
    line = 0i32;
    while line < 1024i32 {
        if 0 == debuglines[line as usize] {
            debuglines[line as usize] =
                botimport.DebugLineCreate.expect("non-null function pointer")();
            debuglinevisible[line as usize] = qfalse as libc::c_int;
            numdebuglines += 1
        }
        if 0 == debuglinevisible[line as usize] {
            botimport.DebugLineShow.expect("non-null function pointer")(debuglines[line
                                                                                       as
                                                                                       usize],
                                                                        start,
                                                                        end,
                                                                        color);
            debuglinevisible[line as usize] = qtrue as libc::c_int;
            return
        }
        line += 1
    };
}
#[no_mangle]
pub static mut numdebuglines: libc::c_int = 0;
//show a permenent line
#[no_mangle]
pub unsafe extern "C" fn AAS_PermanentLine(mut start: *mut vec_t,
                                           mut end: *mut vec_t,
                                           mut color: libc::c_int) {
    let mut line: libc::c_int = 0;
    line = botimport.DebugLineCreate.expect("non-null function pointer")();
    botimport.DebugLineShow.expect("non-null function pointer")(line, start,
                                                                end, color);
}
//show a permanent cross
#[no_mangle]
pub unsafe extern "C" fn AAS_DrawPermanentCross(mut origin: *mut vec_t,
                                                mut size: libc::c_float,
                                                mut color: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut debugline: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    i = 0i32;
    while i < 3i32 {
        start[0usize] = *origin.offset(0isize);
        start[1usize] = *origin.offset(1isize);
        start[2usize] = *origin.offset(2isize);
        start[i as usize] += size;
        end[0usize] = *origin.offset(0isize);
        end[1usize] = *origin.offset(1isize);
        end[2usize] = *origin.offset(2isize);
        end[i as usize] -= size;
        AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), color);
        debugline =
            botimport.DebugLineCreate.expect("non-null function pointer")();
        botimport.DebugLineShow.expect("non-null function pointer")(debugline,
                                                                    start.as_mut_ptr(),
                                                                    end.as_mut_ptr(),
                                                                    color);
        i += 1
    };
}
//draw a cross in the plane
#[no_mangle]
pub unsafe extern "C" fn AAS_DrawPlaneCross(mut point: *mut vec_t,
                                            mut normal: *mut vec_t,
                                            mut dist: libc::c_float,
                                            mut type_0: libc::c_int,
                                            mut color: libc::c_int) {
    let mut n0: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut lines: [libc::c_int; 2] = [0; 2];
    let mut start1: vec3_t = [0.; 3];
    let mut end1: vec3_t = [0.; 3];
    let mut start2: vec3_t = [0.; 3];
    let mut end2: vec3_t = [0.; 3];
    start1[0usize] = *point.offset(0isize);
    start1[1usize] = *point.offset(1isize);
    start1[2usize] = *point.offset(2isize);
    end1[0usize] = *point.offset(0isize);
    end1[1usize] = *point.offset(1isize);
    end1[2usize] = *point.offset(2isize);
    start2[0usize] = *point.offset(0isize);
    start2[1usize] = *point.offset(1isize);
    start2[2usize] = *point.offset(2isize);
    end2[0usize] = *point.offset(0isize);
    end2[1usize] = *point.offset(1isize);
    end2[2usize] = *point.offset(2isize);
    n0 = type_0 % 3i32;
    n1 = (type_0 + 1i32) % 3i32;
    n2 = (type_0 + 2i32) % 3i32;
    start1[n1 as usize] -= 6i32 as libc::c_float;
    start1[n2 as usize] -= 6i32 as libc::c_float;
    end1[n1 as usize] += 6i32 as libc::c_float;
    end1[n2 as usize] += 6i32 as libc::c_float;
    start2[n1 as usize] += 6i32 as libc::c_float;
    start2[n2 as usize] -= 6i32 as libc::c_float;
    end2[n1 as usize] -= 6i32 as libc::c_float;
    end2[n2 as usize] += 6i32 as libc::c_float;
    start1[n0 as usize] =
        (dist -
             (start1[n1 as usize] * *normal.offset(n1 as isize) +
                  start1[n2 as usize] * *normal.offset(n2 as isize))) /
            *normal.offset(n0 as isize);
    end1[n0 as usize] =
        (dist -
             (end1[n1 as usize] * *normal.offset(n1 as isize) +
                  end1[n2 as usize] * *normal.offset(n2 as isize))) /
            *normal.offset(n0 as isize);
    start2[n0 as usize] =
        (dist -
             (start2[n1 as usize] * *normal.offset(n1 as isize) +
                  start2[n2 as usize] * *normal.offset(n2 as isize))) /
            *normal.offset(n0 as isize);
    end2[n0 as usize] =
        (dist -
             (end2[n1 as usize] * *normal.offset(n1 as isize) +
                  end2[n2 as usize] * *normal.offset(n2 as isize))) /
            *normal.offset(n0 as isize);
    j = 0i32;
    line = 0i32;
    while j < 2i32 && line < 1024i32 {
        if 0 == debuglines[line as usize] {
            debuglines[line as usize] =
                botimport.DebugLineCreate.expect("non-null function pointer")();
            let fresh0 = j;
            j = j + 1;
            lines[fresh0 as usize] = debuglines[line as usize];
            debuglinevisible[line as usize] = qtrue as libc::c_int;
            numdebuglines += 1
        } else if 0 == debuglinevisible[line as usize] {
            let fresh1 = j;
            j = j + 1;
            lines[fresh1 as usize] = debuglines[line as usize];
            debuglinevisible[line as usize] = qtrue as libc::c_int
        }
        line += 1
    }
    botimport.DebugLineShow.expect("non-null function pointer")(lines[0usize],
                                                                start1.as_mut_ptr(),
                                                                end1.as_mut_ptr(),
                                                                color);
    botimport.DebugLineShow.expect("non-null function pointer")(lines[1usize],
                                                                start2.as_mut_ptr(),
                                                                end2.as_mut_ptr(),
                                                                color);
}
//show a bounding box
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowBoundingBox(mut origin: *mut vec_t,
                                             mut mins: *mut vec_t,
                                             mut maxs: *mut vec_t) {
    let mut bboxcorners: [vec3_t; 8] = [[0.; 3]; 8];
    let mut lines: [libc::c_int; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    bboxcorners[0usize][0usize] =
        *origin.offset(0isize) + *maxs.offset(0isize);
    bboxcorners[0usize][1usize] =
        *origin.offset(1isize) + *maxs.offset(1isize);
    bboxcorners[0usize][2usize] =
        *origin.offset(2isize) + *maxs.offset(2isize);
    bboxcorners[1usize][0usize] =
        *origin.offset(0isize) + *mins.offset(0isize);
    bboxcorners[1usize][1usize] =
        *origin.offset(1isize) + *maxs.offset(1isize);
    bboxcorners[1usize][2usize] =
        *origin.offset(2isize) + *maxs.offset(2isize);
    bboxcorners[2usize][0usize] =
        *origin.offset(0isize) + *mins.offset(0isize);
    bboxcorners[2usize][1usize] =
        *origin.offset(1isize) + *mins.offset(1isize);
    bboxcorners[2usize][2usize] =
        *origin.offset(2isize) + *maxs.offset(2isize);
    bboxcorners[3usize][0usize] =
        *origin.offset(0isize) + *maxs.offset(0isize);
    bboxcorners[3usize][1usize] =
        *origin.offset(1isize) + *mins.offset(1isize);
    bboxcorners[3usize][2usize] =
        *origin.offset(2isize) + *maxs.offset(2isize);
    memcpy(bboxcorners[4usize].as_mut_ptr() as *mut libc::c_void,
           bboxcorners[0usize].as_mut_ptr() as *const libc::c_void,
           (::std::mem::size_of::<vec3_t>() as
                libc::c_ulong).wrapping_mul(4i32 as libc::c_ulong));
    i = 0i32;
    while i < 4i32 {
        bboxcorners[(4i32 + i) as usize][2usize] =
            *origin.offset(2isize) + *mins.offset(2isize);
        i += 1
    }
    i = 0i32;
    while i < 4i32 {
        j = 0i32;
        line = 0i32;
        while j < 3i32 && line < 1024i32 {
            if 0 == debuglines[line as usize] {
                debuglines[line as usize] =
                    botimport.DebugLineCreate.expect("non-null function pointer")();
                let fresh2 = j;
                j = j + 1;
                lines[fresh2 as usize] = debuglines[line as usize];
                debuglinevisible[line as usize] = qtrue as libc::c_int;
                numdebuglines += 1
            } else if 0 == debuglinevisible[line as usize] {
                let fresh3 = j;
                j = j + 1;
                lines[fresh3 as usize] = debuglines[line as usize];
                debuglinevisible[line as usize] = qtrue as libc::c_int
            }
            line += 1
        }
        botimport.DebugLineShow.expect("non-null function pointer")(lines[0usize],
                                                                    bboxcorners[i
                                                                                    as
                                                                                    usize].as_mut_ptr(),
                                                                    bboxcorners[(i
                                                                                     +
                                                                                     1i32
                                                                                     &
                                                                                     3i32)
                                                                                    as
                                                                                    usize].as_mut_ptr(),
                                                                    1i32);
        botimport.DebugLineShow.expect("non-null function pointer")(lines[1usize],
                                                                    bboxcorners[(4i32
                                                                                     +
                                                                                     i)
                                                                                    as
                                                                                    usize].as_mut_ptr(),
                                                                    bboxcorners[(4i32
                                                                                     +
                                                                                     (i
                                                                                          +
                                                                                          1i32
                                                                                          &
                                                                                          3i32))
                                                                                    as
                                                                                    usize].as_mut_ptr(),
                                                                    1i32);
        botimport.DebugLineShow.expect("non-null function pointer")(lines[2usize],
                                                                    bboxcorners[i
                                                                                    as
                                                                                    usize].as_mut_ptr(),
                                                                    bboxcorners[(4i32
                                                                                     +
                                                                                     i)
                                                                                    as
                                                                                    usize].as_mut_ptr(),
                                                                    1i32);
        i += 1
    };
}
//show a face
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowFace(mut facenum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    color = 4i32;
    if facenum >= aasworld.numfaces {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"facenum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            facenum);
    }
    face = &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
    i = 0i32;
    while i < (*face).numedges {
        edgenum =
            abs(*aasworld.edgeindex.offset(((*face).firstedge + i) as isize));
        if edgenum >= aasworld.numedges {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"edgenum %d out of range\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                edgenum);
        }
        edge =
            &mut *aasworld.edges.offset(edgenum as isize) as *mut aas_edge_t;
        if color == 1i32 {
            color = 2i32
        } else if color == 2i32 {
            color = 3i32
        } else if color == 3i32 { color = 4i32 } else { color = 1i32 }
        AAS_DebugLine((*aasworld.vertexes.offset((*edge).v[0usize] as
                                                     isize)).as_mut_ptr(),
                      (*aasworld.vertexes.offset((*edge).v[1usize] as
                                                     isize)).as_mut_ptr(),
                      color);
        i += 1
    }
    plane =
        &mut *aasworld.planes.offset((*face).planenum as isize) as
            *mut aas_plane_t;
    edgenum = abs(*aasworld.edgeindex.offset((*face).firstedge as isize));
    edge = &mut *aasworld.edges.offset(edgenum as isize) as *mut aas_edge_t;
    start[0usize] =
        (*aasworld.vertexes.offset((*edge).v[0usize] as isize))[0usize];
    start[1usize] =
        (*aasworld.vertexes.offset((*edge).v[0usize] as isize))[1usize];
    start[2usize] =
        (*aasworld.vertexes.offset((*edge).v[0usize] as isize))[2usize];
    end[0usize] =
        start[0usize] + (*plane).normal[0usize] * 20i32 as libc::c_float;
    end[1usize] =
        start[1usize] + (*plane).normal[1usize] * 20i32 as libc::c_float;
    end[2usize] =
        start[2usize] + (*plane).normal[2usize] * 20i32 as libc::c_float;
    AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), 1i32);
}
//show an area
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowArea(mut areanum: libc::c_int,
                                      mut groundfacesonly: libc::c_int) {
    let mut areaedges: [libc::c_int; 1024] = [0; 1024];
    let mut numareaedges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut color: libc::c_int = 0i32;
    let mut line: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    numareaedges = 0i32;
    if areanum < 0i32 || areanum >= aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"area %d out of range [0, %d]\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            areanum,
                                                            aasworld.numareas);
        return
    }
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    let mut current_block_23: u64;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            abs(*aasworld.faceindex.offset(((*area).firstface + i) as isize));
        if facenum >= aasworld.numfaces {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"facenum %d out of range\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                facenum);
        }
        face =
            &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
        //ground faces only
        if 0 != groundfacesonly {
            if 0 == (*face).faceflags & (4i32 | 2i32) {
                current_block_23 = 3276175668257526147;
            } else { current_block_23 = 15652330335145281839; }
        } else { current_block_23 = 15652330335145281839; }
        match current_block_23 {
            15652330335145281839 => {
                j = 0i32;
                while j < (*face).numedges {
                    edgenum =
                        abs(*aasworld.edgeindex.offset(((*face).firstedge + j)
                                                           as isize));
                    if edgenum >= aasworld.numedges {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"edgenum %d out of range\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            edgenum);
                    }
                    n = 0i32;
                    while n < numareaedges {
                        if areaedges[n as usize] == edgenum { break ; }
                        n += 1
                    }
                    if n == numareaedges && numareaedges < 1024i32 {
                        let fresh4 = numareaedges;
                        numareaedges = numareaedges + 1;
                        areaedges[fresh4 as usize] = edgenum
                    }
                    j += 1
                }
            }
            _ => { }
        }
        i += 1
    }
    n = 0i32;
    while n < numareaedges {
        line = 0i32;
        while line < 1024i32 {
            if 0 == debuglines[line as usize] {
                debuglines[line as usize] =
                    botimport.DebugLineCreate.expect("non-null function pointer")();
                debuglinevisible[line as usize] = qfalse as libc::c_int;
                numdebuglines += 1
            }
            //end if
            if 0 == debuglinevisible[line as usize] { break ; }
            line += 1
        }
        if line >= 1024i32 { return }
        edge =
            &mut *aasworld.edges.offset(*areaedges.as_mut_ptr().offset(n as
                                                                           isize)
                                            as isize) as *mut aas_edge_t;
        if color == 1i32 {
            color = 3i32
        } else if color == 3i32 {
            color = 2i32
        } else if color == 2i32 { color = 4i32 } else { color = 1i32 }
        botimport.DebugLineShow.expect("non-null function pointer")(debuglines[line
                                                                                   as
                                                                                   usize],
                                                                    (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                                                   as
                                                                                                   isize)).as_mut_ptr(),
                                                                    (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                                                   as
                                                                                                   isize)).as_mut_ptr(),
                                                                    color);
        debuglinevisible[line as usize] = qtrue as libc::c_int;
        n += 1
    };
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowAreaPolygons(mut areanum: libc::c_int,
                                              mut color: libc::c_int,
                                              mut groundfacesonly:
                                                  libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    if areanum < 0i32 || areanum >= aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"area %d out of range [0, %d]\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            areanum,
                                                            aasworld.numareas);
        return
    }
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    let mut current_block_11: u64;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            abs(*aasworld.faceindex.offset(((*area).firstface + i) as isize));
        if facenum >= aasworld.numfaces {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"facenum %d out of range\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                facenum);
        }
        face =
            &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
        //ground faces only
        if 0 != groundfacesonly {
            if 0 == (*face).faceflags & (4i32 | 2i32) {
                current_block_11 = 2473556513754201174;
            } else { current_block_11 = 8236137900636309791; }
        } else { current_block_11 = 8236137900636309791; }
        match current_block_11 {
            8236137900636309791 => {
                AAS_ShowFacePolygon(facenum, color,
                                    ((*face).frontarea != areanum) as
                                        libc::c_int);
            }
            _ => { }
        }
        i += 1
    };
}
//end of the function AAS_ShowFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowFacePolygon(mut facenum: libc::c_int,
                                             mut color: libc::c_int,
                                             mut flip: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut numpoints: libc::c_int = 0;
    let mut points: [vec3_t; 128] = [[0.; 3]; 128];
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    if facenum >= aasworld.numfaces {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"facenum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            facenum);
    }
    face = &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
    numpoints = 0i32;
    if 0 != flip {
        i = (*face).numedges - 1i32;
        while i >= 0i32 {
            edgenum =
                *aasworld.edgeindex.offset(((*face).firstedge + i) as isize);
            edge =
                &mut *aasworld.edges.offset(abs(edgenum) as isize) as
                    *mut aas_edge_t;
            points[numpoints as usize][0usize] =
                (*aasworld.vertexes.offset((*edge).v[(edgenum < 0i32) as
                                                         libc::c_int as usize]
                                               as isize))[0usize];
            points[numpoints as usize][1usize] =
                (*aasworld.vertexes.offset((*edge).v[(edgenum < 0i32) as
                                                         libc::c_int as usize]
                                               as isize))[1usize];
            points[numpoints as usize][2usize] =
                (*aasworld.vertexes.offset((*edge).v[(edgenum < 0i32) as
                                                         libc::c_int as usize]
                                               as isize))[2usize];
            numpoints += 1;
            i -= 1
        }
    } else {
        i = 0i32;
        while i < (*face).numedges {
            edgenum =
                *aasworld.edgeindex.offset(((*face).firstedge + i) as isize);
            edge =
                &mut *aasworld.edges.offset(abs(edgenum) as isize) as
                    *mut aas_edge_t;
            points[numpoints as usize][0usize] =
                (*aasworld.vertexes.offset((*edge).v[(edgenum < 0i32) as
                                                         libc::c_int as usize]
                                               as isize))[0usize];
            points[numpoints as usize][1usize] =
                (*aasworld.vertexes.offset((*edge).v[(edgenum < 0i32) as
                                                         libc::c_int as usize]
                                               as isize))[1usize];
            points[numpoints as usize][2usize] =
                (*aasworld.vertexes.offset((*edge).v[(edgenum < 0i32) as
                                                         libc::c_int as usize]
                                               as isize))[2usize];
            numpoints += 1;
            i += 1
        }
    }
    AAS_ShowPolygon(color, numpoints, points.as_mut_ptr());
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
//end of the function AAS_ClearShownPolygons
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowPolygon(mut color: libc::c_int,
                                         mut numpoints: libc::c_int,
                                         mut points: *mut vec3_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8192i32 {
        if 0 == debugpolygons[i as usize] {
            debugpolygons[i as usize] =
                botimport.DebugPolygonCreate.expect("non-null function pointer")(color,
                                                                                 numpoints,
                                                                                 points);
            break ;
        } else { i += 1 }
    };
}
//draw a cros
#[no_mangle]
pub unsafe extern "C" fn AAS_DrawCross(mut origin: *mut vec_t,
                                       mut size: libc::c_float,
                                       mut color: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    i = 0i32;
    while i < 3i32 {
        start[0usize] = *origin.offset(0isize);
        start[1usize] = *origin.offset(1isize);
        start[2usize] = *origin.offset(2isize);
        start[i as usize] += size;
        end[0usize] = *origin.offset(0isize);
        end[1usize] = *origin.offset(1isize);
        end[2usize] = *origin.offset(2isize);
        end[i as usize] -= size;
        AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), color);
        i += 1
    };
}
//print the travel type
#[no_mangle]
pub unsafe extern "C" fn AAS_PrintTravelType(mut traveltype: libc::c_int) { }
//draw an arrow
#[no_mangle]
pub unsafe extern "C" fn AAS_DrawArrow(mut start: *mut vec_t,
                                       mut end: *mut vec_t,
                                       mut linecolor: libc::c_int,
                                       mut arrowcolor: libc::c_int) {
    let mut dir: vec3_t = [0.; 3];
    let mut cross: vec3_t = [0.; 3];
    let mut p1: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut dot: libc::c_float = 0.;
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    VectorNormalize(dir.as_mut_ptr());
    dot =
        dir[0usize] * up[0usize] + dir[1usize] * up[1usize] +
            dir[2usize] * up[2usize];
    if dot as libc::c_double > 0.99f64 || (dot as libc::c_double) < -0.99f64 {
        cross[0usize] = 1i32 as vec_t;
        cross[1usize] = 0i32 as vec_t;
        cross[2usize] = 0i32 as vec_t
    } else {
        CrossProduct(dir.as_mut_ptr() as *const vec_t,
                     up.as_mut_ptr() as *const vec_t, cross.as_mut_ptr());
    }
    p1[0usize] = *end.offset(0isize) + dir[0usize] * -6i32 as libc::c_float;
    p1[1usize] = *end.offset(1isize) + dir[1usize] * -6i32 as libc::c_float;
    p1[2usize] = *end.offset(2isize) + dir[2usize] * -6i32 as libc::c_float;
    p2[0usize] = p1[0usize];
    p2[1usize] = p1[1usize];
    p2[2usize] = p1[2usize];
    p1[0usize] = p1[0usize] + cross[0usize] * 6i32 as libc::c_float;
    p1[1usize] = p1[1usize] + cross[1usize] * 6i32 as libc::c_float;
    p1[2usize] = p1[2usize] + cross[2usize] * 6i32 as libc::c_float;
    p2[0usize] = p2[0usize] + cross[0usize] * -6i32 as libc::c_float;
    p2[1usize] = p2[1usize] + cross[1usize] * -6i32 as libc::c_float;
    p2[2usize] = p2[2usize] + cross[2usize] * -6i32 as libc::c_float;
    AAS_DebugLine(start, end, linecolor);
    AAS_DebugLine(p1.as_mut_ptr(), end, arrowcolor);
    AAS_DebugLine(p2.as_mut_ptr(), end, arrowcolor);
}
//visualize the given reachability
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowReachability(mut reach:
                                                  *mut aas_reachability_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    let mut speed: libc::c_float = 0.;
    let mut zvel: libc::c_float = 0.;
    let mut move_0: aas_clientmove_t =
        aas_clientmove_s{endpos: [0.; 3],
                         endarea: 0,
                         velocity: [0.; 3],
                         trace:
                             aas_trace_s{startsolid: qfalse,
                                         fraction: 0.,
                                         endpos: [0.; 3],
                                         ent: 0,
                                         lastarea: 0,
                                         area: 0,
                                         planenum: 0,},
                         presencetype: 0,
                         stopevent: 0,
                         endcontents: 0,
                         time: 0.,
                         frames: 0,};
    AAS_ShowAreaPolygons((*reach).areanum, 5i32, qtrue as libc::c_int);
    AAS_DrawArrow((*reach).start.as_mut_ptr(), (*reach).end.as_mut_ptr(),
                  3i32, 4i32);
    if (*reach).traveltype & 0xffffffi32 == 5i32 ||
           (*reach).traveltype & 0xffffffi32 == 7i32 {
        AAS_HorizontalVelocityForJump(aassettings.phys_jumpvel,
                                      (*reach).start.as_mut_ptr(),
                                      (*reach).end.as_mut_ptr(), &mut speed);
        dir[0usize] = (*reach).end[0usize] - (*reach).start[0usize];
        dir[1usize] = (*reach).end[1usize] - (*reach).start[1usize];
        dir[2usize] = (*reach).end[2usize] - (*reach).start[2usize];
        dir[2usize] = 0i32 as vec_t;
        VectorNormalize(dir.as_mut_ptr());
        velocity[0usize] = dir[0usize] * speed;
        velocity[1usize] = dir[1usize] * speed;
        velocity[2usize] = dir[2usize] * speed;
        cmdmove[2usize] = 0i32 as vec_t;
        cmdmove[1usize] = cmdmove[2usize];
        cmdmove[0usize] = cmdmove[1usize];
        cmdmove[2usize] = aassettings.phys_jumpvel;
        AAS_PredictClientMovement(&mut move_0, -1i32,
                                  (*reach).start.as_mut_ptr(), 2i32,
                                  qtrue as libc::c_int, velocity.as_mut_ptr(),
                                  cmdmove.as_mut_ptr(), 3i32, 30i32, 0.1f32,
                                  1i32 | 4i32 | 8i32 | 16i32 | 32i32, 0i32,
                                  qtrue as libc::c_int);
        if (*reach).traveltype & 0xffffffi32 == 5i32 {
            AAS_JumpReachRunStart(reach, dir.as_mut_ptr());
            AAS_DrawCross(dir.as_mut_ptr(), 4i32 as libc::c_float, 3i32);
        }
    } else if (*reach).traveltype & 0xffffffi32 == 12i32 {
        zvel = AAS_RocketJumpZVelocity((*reach).start.as_mut_ptr());
        AAS_HorizontalVelocityForJump(zvel, (*reach).start.as_mut_ptr(),
                                      (*reach).end.as_mut_ptr(), &mut speed);
        dir[0usize] = (*reach).end[0usize] - (*reach).start[0usize];
        dir[1usize] = (*reach).end[1usize] - (*reach).start[1usize];
        dir[2usize] = (*reach).end[2usize] - (*reach).start[2usize];
        dir[2usize] = 0i32 as vec_t;
        VectorNormalize(dir.as_mut_ptr());
        cmdmove[0usize] = dir[0usize] * speed;
        cmdmove[1usize] = dir[1usize] * speed;
        cmdmove[2usize] = dir[2usize] * speed;
        velocity[0usize] = 0i32 as vec_t;
        velocity[1usize] = 0i32 as vec_t;
        velocity[2usize] = zvel;
        AAS_PredictClientMovement(&mut move_0, -1i32,
                                  (*reach).start.as_mut_ptr(), 2i32,
                                  qtrue as libc::c_int, velocity.as_mut_ptr(),
                                  cmdmove.as_mut_ptr(), 30i32, 30i32, 0.1f32,
                                  4i32 | 8i32 | 16i32 | 32i32 | 128i32 |
                                      1024i32, (*reach).areanum,
                                  qtrue as libc::c_int);
    } else if (*reach).traveltype & 0xffffffi32 == 18i32 {
        cmdmove[0usize] = 0i32 as vec_t;
        cmdmove[1usize] = 0i32 as vec_t;
        cmdmove[2usize] = 0i32 as vec_t;
        dir[0usize] = (*reach).end[0usize] - (*reach).start[0usize];
        dir[1usize] = (*reach).end[1usize] - (*reach).start[1usize];
        dir[2usize] = (*reach).end[2usize] - (*reach).start[2usize];
        dir[2usize] = 0i32 as vec_t;
        VectorNormalize(dir.as_mut_ptr());
        velocity[0usize] = dir[0usize] * (*reach).edgenum as libc::c_float;
        velocity[1usize] = dir[1usize] * (*reach).edgenum as libc::c_float;
        velocity[2usize] = dir[2usize] * (*reach).edgenum as libc::c_float;
        velocity[2usize] = (*reach).facenum as vec_t;
        AAS_PredictClientMovement(&mut move_0, -1i32,
                                  (*reach).start.as_mut_ptr(), 2i32,
                                  qtrue as libc::c_int, velocity.as_mut_ptr(),
                                  cmdmove.as_mut_ptr(), 30i32, 30i32, 0.1f32,
                                  4i32 | 8i32 | 16i32 | 32i32 | 128i32 |
                                      1024i32, (*reach).areanum,
                                  qtrue as libc::c_int);
    };
}
//show the reachable areas from the given area
#[no_mangle]
pub unsafe extern "C" fn AAS_ShowReachableAreas(mut areanum: libc::c_int) {
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    static mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    static mut index: libc::c_int = 0;
    static mut lastareanum: libc::c_int = 0;
    static mut lasttime: libc::c_float = 0.;
    if areanum != lastareanum { index = 0i32; lastareanum = areanum }
    settings =
        &mut *aasworld.areasettings.offset(areanum as isize) as
            *mut aas_areasettings_t;
    if 0 == (*settings).numreachableareas { return }
    if index >= (*settings).numreachableareas { index = 0i32 }
    if (AAS_Time() - lasttime) as libc::c_double > 1.5f64 {
        memcpy(&mut reach as *mut aas_reachability_t as *mut libc::c_void,
               &mut *aasworld.reachability.offset(((*settings).firstreachablearea
                                                       + index) as isize) as
                   *mut aas_reachability_t as *const libc::c_void,
               ::std::mem::size_of::<aas_reachability_t>() as libc::c_ulong);
        index += 1;
        lasttime = AAS_Time();
        AAS_PrintTravelType(reach.traveltype & 0xffffffi32);
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"\n\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
    }
    AAS_ShowReachability(&mut reach);
}
//end of the function ShowReachableAreas
#[no_mangle]
pub unsafe extern "C" fn AAS_FloodAreas_r(mut areanum: libc::c_int,
                                          mut cluster: libc::c_int,
                                          mut done: *mut libc::c_int) {
    let mut nextareanum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut settings: *mut aas_areasettings_t = 0 as *mut aas_areasettings_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    AAS_ShowAreaPolygons(areanum, 1i32, qtrue as libc::c_int);
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    settings =
        &mut *aasworld.areasettings.offset(areanum as isize) as
            *mut aas_areasettings_t;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            abs(*aasworld.faceindex.offset(((*area).firstface + i) as isize));
        face =
            &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
        if (*face).frontarea == areanum {
            nextareanum = (*face).backarea
        } else { nextareanum = (*face).frontarea }
        if !(0 == nextareanum) {
            if !(0 != *done.offset(nextareanum as isize)) {
                *done.offset(nextareanum as isize) = qtrue as libc::c_int;
                if !(0 !=
                         (*aasworld.areasettings.offset(nextareanum as
                                                            isize)).contents &
                             512i32) {
                    if !(AAS_AreaCluster(nextareanum) != cluster) {
                        AAS_FloodAreas_r(nextareanum, cluster, done);
                    }
                }
            }
        }
        i += 1
    }
    i = 0i32;
    while i < (*settings).numreachableareas {
        reach =
            &mut *aasworld.reachability.offset(((*settings).firstreachablearea
                                                    + i) as isize) as
                *mut aas_reachability_t;
        nextareanum = (*reach).areanum;
        if !(0 == nextareanum) {
            if !(0 != *done.offset(nextareanum as isize)) {
                *done.offset(nextareanum as isize) = qtrue as libc::c_int;
                if !(0 !=
                         (*aasworld.areasettings.offset(nextareanum as
                                                            isize)).contents &
                             512i32) {
                    if !(AAS_AreaCluster(nextareanum) != cluster) {
                        AAS_FloodAreas_r(nextareanum, cluster, done);
                    }
                }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AAS_FloodAreas(mut origin: *mut vec_t) {
    let mut areanum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut done: *mut libc::c_int = 0 as *mut libc::c_int;
    done =
        GetClearedMemory((aasworld.numareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                              as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    areanum = AAS_PointAreaNum(origin);
    cluster = AAS_AreaCluster(areanum);
    AAS_FloodAreas_r(areanum, cluster, done);
    FreeMemory(done as *mut libc::c_void);
}