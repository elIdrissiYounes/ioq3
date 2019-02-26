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
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
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
 * name:		l_libvar.h
 *
 * desc:		botlib vars
 *
 * $Archive: /source/code/botlib/l_libvar.h $
 *
 *****************************************************************************/
    //library variable
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct libvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub value: libc::c_float,
        pub next: *mut libvar_s,
    }
    pub type libvar_t = libvar_s;
    use super::{libc};
    use super::q_shared_h::{qboolean};
    extern "C" {
        //gets the value of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetValue(var_name: *const libc::c_char) -> libc::c_float;
        //creates the library variable if not existing already and returns it
        #[no_mangle]
        pub fn LibVar(var_name: *const libc::c_char,
                      value: *const libc::c_char) -> *mut libvar_t;
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
        //sets the library variable
        #[no_mangle]
        pub fn LibVarSet(var_name: *const libc::c_char,
                         value: *const libc::c_char);
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
        //prints the total used memory size
        #[no_mangle]
        pub fn PrintUsedMemorySize();
        //print all memory blocks with label
        #[no_mangle]
        pub fn PrintMemoryLabels();
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::{libc};
    use super::be_aas_def_h::{aas_t};
    use super::q_shared_h::{vec_t};
}
#[header_src =
      "ioq3/code/botlib/be_aas_optimize.h"]
pub mod be_aas_optimize_h {
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
 * name:		be_aas_optimize.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_optimize.h $
 *
 *****************************************************************************/
        #[no_mangle]
        pub fn AAS_Optimize();
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    extern "C" {
        //
        #[no_mangle]
        pub fn AAS_InitSettings();
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
      "ioq3/code/botlib/be_aas_entity.h"]
pub mod be_aas_entity_h {
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
 * name:		be_aas_entity.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_entity.h $
 *
 *****************************************************************************/
        //invalidates all entity infos
        #[no_mangle]
        pub fn AAS_InvalidateEntities();
        //resets the entity AAS and BSP links (sets areas and leaves pointers to NULL)
        #[no_mangle]
        pub fn AAS_ResetEntityLinks();
        //unlink not updated entities
        #[no_mangle]
        pub fn AAS_UnlinkInvalidEntities();
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_main.c"]
pub mod be_aas_main_c {
    use super::l_libvar_h::{libvar_t};
    use super::{libc};
}
#[header_src =
      "ioq3/code/botlib/be_aas_file.h"]
pub mod be_aas_file_h {
    use super::{libc};
    use super::q_shared_h::{qboolean};
    extern "C" {
        //dumps the loaded AAS data
        #[no_mangle]
        pub fn AAS_DumpAASData();
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
 * name:		be_aas_file.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_file.h $
 *
 *****************************************************************************/
        //loads the AAS file with the given name
        #[no_mangle]
        pub fn AAS_LoadAASFile(filename: *mut libc::c_char) -> libc::c_int;
        //writes an AAS file with the given name
        #[no_mangle]
        pub fn AAS_WriteAASFile(filename: *mut libc::c_char) -> qboolean;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    extern "C" {
        #[no_mangle]
        pub fn AAS_FreeAASLinkedEntities();
        #[no_mangle]
        pub fn AAS_FreeAASLinkHeap();
        #[no_mangle]
        pub fn AAS_InitAASLinkedEntities();
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
        pub fn AAS_InitAASLinkHeap();
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_route.h"]
pub mod be_aas_route_h {
    extern "C" {
        //free the AAS routing caches
        #[no_mangle]
        pub fn AAS_FreeRoutingCaches();
        #[no_mangle]
        pub fn AAS_WriteRouteCache();
        //
        #[no_mangle]
        pub fn AAS_RoutingInfo();
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
        pub fn AAS_InitRouting();
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::{libc};
    extern "C" {
        //dump the loaded BSP data
        #[no_mangle]
        pub fn AAS_DumpBSPData();
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
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
        //loads the given BSP file
        #[no_mangle]
        pub fn AAS_LoadBSPFile() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_routealt.h"]
pub mod be_aas_routealt_h {
    extern "C" {
        #[no_mangle]
        pub fn AAS_ShutdownAlternativeRouting();
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
 * name:		be_aas_routealt.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_routealt.h $
 *
 *****************************************************************************/
        #[no_mangle]
        pub fn AAS_InitAlternativeRouting();
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_reach.h"]
pub mod be_aas_reach_h {
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
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
        //initialize calculating the reachabilities
        #[no_mangle]
        pub fn AAS_InitReachability();
        //continue calculating the reachabilities
        #[no_mangle]
        pub fn AAS_ContinueInitReachability(time: libc::c_float)
         -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_cluster.h"]
pub mod be_aas_cluster_h {
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
 * name:		be_aas_cluster.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_cluster.h $
 *
 *****************************************************************************/
        //initialize the AAS clustering
        #[no_mangle]
        pub fn AAS_InitClustering();
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, VectorNormalize,
                       Com_sprintf, Q_strncpyz};
use self::l_libvar_h::{libvar_s, libvar_t, LibVarGetValue, LibVar,
                       LibVarValue, LibVarSet};
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
use self::be_aas_def_h::{aas_t, aas_s, aas_reachabilityareas_t,
                         aas_reachabilityareas_s, aas_routingcache_t,
                         aas_routingcache_s, aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s};
use self::string_h::{memset};
use self::l_memory_h::{GetClearedHunkMemory, FreeMemory, PrintUsedMemorySize,
                       PrintMemoryLabels};
use self::be_aas_optimize_h::{AAS_Optimize};
use self::be_aas_move_h::{AAS_InitSettings};
use self::be_interface_h::{botimport, botDeveloper};
use self::be_aas_entity_h::{AAS_InvalidateEntities, AAS_ResetEntityLinks,
                            AAS_UnlinkInvalidEntities};
use self::be_aas_file_h::{AAS_DumpAASData, AAS_LoadAASFile, AAS_WriteAASFile};
use self::be_aas_sample_h::{AAS_FreeAASLinkedEntities, AAS_FreeAASLinkHeap,
                            AAS_InitAASLinkedEntities, AAS_InitAASLinkHeap};
use self::be_aas_route_h::{AAS_FreeRoutingCaches, AAS_WriteRouteCache,
                           AAS_RoutingInfo, AAS_InitRouting};
use self::be_aas_bsp_h::{AAS_DumpBSPData, AAS_LoadBSPFile};
use self::be_aas_routealt_h::{AAS_ShutdownAlternativeRouting,
                              AAS_InitAlternativeRouting};
use self::be_aas_reach_h::{AAS_InitReachability,
                           AAS_ContinueInitReachability};
use self::be_aas_cluster_h::{AAS_InitClustering};
//AASINTERN
//returns true if AAS is initialized
#[no_mangle]
pub unsafe extern "C" fn AAS_Initialized() -> libc::c_int {
    return aasworld.initialized;
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
 * name:		be_aas_main.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_main.h $
 *
 *****************************************************************************/
#[no_mangle]
pub static mut aasworld: aas_t =
    aas_s{loaded: 0,
          initialized: 0,
          savefile: 0,
          bspchecksum: 0,
          time: 0.,
          numframes: 0,
          filename: [0; 64],
          mapname: [0; 64],
          numbboxes: 0,
          bboxes: 0 as *const aas_bbox_t as *mut aas_bbox_t,
          numvertexes: 0,
          vertexes: 0 as *const aas_vertex_t as *mut aas_vertex_t,
          numplanes: 0,
          planes: 0 as *const aas_plane_t as *mut aas_plane_t,
          numedges: 0,
          edges: 0 as *const aas_edge_t as *mut aas_edge_t,
          edgeindexsize: 0,
          edgeindex: 0 as *const aas_edgeindex_t as *mut aas_edgeindex_t,
          numfaces: 0,
          faces: 0 as *const aas_face_t as *mut aas_face_t,
          faceindexsize: 0,
          faceindex: 0 as *const aas_faceindex_t as *mut aas_faceindex_t,
          numareas: 0,
          areas: 0 as *const aas_area_t as *mut aas_area_t,
          numareasettings: 0,
          areasettings:
              0 as *const aas_areasettings_t as *mut aas_areasettings_t,
          reachabilitysize: 0,
          reachability:
              0 as *const aas_reachability_t as *mut aas_reachability_t,
          numnodes: 0,
          nodes: 0 as *const aas_node_t as *mut aas_node_t,
          numportals: 0,
          portals: 0 as *const aas_portal_t as *mut aas_portal_t,
          portalindexsize: 0,
          portalindex:
              0 as *const aas_portalindex_t as *mut aas_portalindex_t,
          numclusters: 0,
          clusters: 0 as *const aas_cluster_t as *mut aas_cluster_t,
          numreachabilityareas: 0,
          reachabilitytime: 0.,
          linkheap: 0 as *const aas_link_t as *mut aas_link_t,
          linkheapsize: 0,
          freelinks: 0 as *const aas_link_t as *mut aas_link_t,
          arealinkedentities:
              0 as *const *mut aas_link_t as *mut *mut aas_link_t,
          maxentities: 0,
          maxclients: 0,
          entities: 0 as *const aas_entity_t as *mut aas_entity_t,
          travelflagfortype: [0; 32],
          areacontentstravelflags:
              0 as *const libc::c_int as *mut libc::c_int,
          areaupdate:
              0 as *const aas_routingupdate_t as *mut aas_routingupdate_t,
          portalupdate:
              0 as *const aas_routingupdate_t as *mut aas_routingupdate_t,
          frameroutingupdates: 0,
          reversedreachability:
              0 as *const aas_reversedreachability_t as
                  *mut aas_reversedreachability_t,
          areatraveltimes:
              0 as *const *mut *mut libc::c_ushort as
                  *mut *mut *mut libc::c_ushort,
          clusterareacache:
              0 as *const *mut *mut aas_routingcache_t as
                  *mut *mut *mut aas_routingcache_t,
          portalcache:
              0 as *const *mut aas_routingcache_t as
                  *mut *mut aas_routingcache_t,
          oldestcache:
              0 as *const aas_routingcache_t as *mut aas_routingcache_t,
          newestcache:
              0 as *const aas_routingcache_t as *mut aas_routingcache_t,
          portalmaxtraveltimes: 0 as *const libc::c_int as *mut libc::c_int,
          reachabilityareaindex: 0 as *const libc::c_int as *mut libc::c_int,
          reachabilityareas:
              0 as *const aas_reachabilityareas_t as
                  *mut aas_reachabilityareas_t,};
//returns true if the AAS file is loaded
#[no_mangle]
pub unsafe extern "C" fn AAS_Loaded() -> libc::c_int {
    return aasworld.loaded;
}
//returns the current time
#[no_mangle]
pub unsafe extern "C" fn AAS_Time() -> libc::c_float { return aasworld.time; }
//
#[no_mangle]
pub unsafe extern "C" fn AAS_ProjectPointOntoVector(mut point: *mut vec_t,
                                                    mut vStart: *mut vec_t,
                                                    mut vEnd: *mut vec_t,
                                                    mut vProj: *mut vec_t) {
    let mut pVec: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    pVec[0usize] = *point.offset(0isize) - *vStart.offset(0isize);
    pVec[1usize] = *point.offset(1isize) - *vStart.offset(1isize);
    pVec[2usize] = *point.offset(2isize) - *vStart.offset(2isize);
    vec[0usize] = *vEnd.offset(0isize) - *vStart.offset(0isize);
    vec[1usize] = *vEnd.offset(1isize) - *vStart.offset(1isize);
    vec[2usize] = *vEnd.offset(2isize) - *vStart.offset(2isize);
    VectorNormalize(vec.as_mut_ptr());
    *vProj.offset(0isize) =
        *vStart.offset(0isize) +
            vec[0usize] *
                (pVec[0usize] * vec[0usize] + pVec[1usize] * vec[1usize] +
                     pVec[2usize] * vec[2usize]);
    *vProj.offset(1isize) =
        *vStart.offset(1isize) +
            vec[1usize] *
                (pVec[0usize] * vec[0usize] + pVec[1usize] * vec[1usize] +
                     pVec[2usize] * vec[2usize]);
    *vProj.offset(2isize) =
        *vStart.offset(2isize) +
            vec[2usize] *
                (pVec[0usize] * vec[0usize] + pVec[1usize] * vec[1usize] +
                     pVec[2usize] * vec[2usize]);
}
//set AAS initialized
#[no_mangle]
pub unsafe extern "C" fn AAS_SetInitialized() {
    aasworld.initialized = qtrue as libc::c_int;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"AAS initialized.\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
}
//setup AAS with the given number of entities and clients
#[no_mangle]
pub unsafe extern "C" fn AAS_Setup() -> libc::c_int {
    aasworld.maxclients =
        LibVarValue(b"maxclients\x00" as *const u8 as *const libc::c_char,
                    b"128\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    aasworld.maxentities =
        LibVarValue(b"maxentities\x00" as *const u8 as *const libc::c_char,
                    b"1024\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    saveroutingcache =
        LibVar(b"saveroutingcache\x00" as *const u8 as *const libc::c_char,
               b"0\x00" as *const u8 as *const libc::c_char);
    if !aasworld.entities.is_null() {
        FreeMemory(aasworld.entities as *mut libc::c_void);
    }
    aasworld.entities =
        GetClearedHunkMemory((aasworld.maxentities as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_entity_t>()
                                                                  as
                                                                  libc::c_ulong))
            as *mut aas_entity_t;
    AAS_InvalidateEntities();
    aasworld.numframes = 0i32;
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
 * name:		be_aas_main.c
 *
 * desc:		AAS
 *
 * $Archive: /MissionPack/code/botlib/be_aas_main.c $
 *
 *****************************************************************************/
#[no_mangle]
pub static mut saveroutingcache: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//shutdown AAS
#[no_mangle]
pub unsafe extern "C" fn AAS_Shutdown() {
    AAS_ShutdownAlternativeRouting();
    AAS_DumpBSPData();
    AAS_FreeRoutingCaches();
    AAS_FreeAASLinkHeap();
    AAS_FreeAASLinkedEntities();
    AAS_DumpAASData();
    if !aasworld.entities.is_null() {
        FreeMemory(aasworld.entities as *mut libc::c_void);
    }
    memset(&mut aasworld as *mut aas_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_t>() as libc::c_ulong);
    aasworld.initialized = qfalse as libc::c_int;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"AAS shutdown.\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
}
//start a new map
#[no_mangle]
pub unsafe extern "C" fn AAS_LoadMap(mut mapname: *const libc::c_char)
 -> libc::c_int {
    let mut errnum: libc::c_int = 0;
    if mapname.is_null() { return 0i32 }
    aasworld.initialized = qfalse as libc::c_int;
    AAS_FreeRoutingCaches();
    errnum = AAS_LoadFiles(mapname);
    if errnum != 0i32 {
        aasworld.loaded = qfalse as libc::c_int;
        return errnum
    }
    AAS_InitSettings();
    AAS_InitAASLinkHeap();
    AAS_InitAASLinkedEntities();
    AAS_InitReachability();
    AAS_InitAlternativeRouting();
    return 0i32;
}
//end of the function AAS_ProjectPointOntoVector
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_LoadFiles(mut mapname: *const libc::c_char)
 -> libc::c_int {
    let mut errnum: libc::c_int = 0;
    let mut aasfile: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(aasworld.mapname.as_mut_ptr(), mapname,
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    AAS_ResetEntityLinks();
    AAS_LoadBSPFile();
    Com_sprintf(aasfile.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"maps/%s.aas\x00" as *const u8 as *const libc::c_char,
                mapname);
    errnum = AAS_LoadAASFile(aasfile.as_mut_ptr());
    if errnum != 0i32 { return errnum }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        aasfile.as_mut_ptr());
    Q_strncpyz(aasworld.filename.as_mut_ptr(), aasfile.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    return 0i32;
}
//start a new time frame
#[no_mangle]
pub unsafe extern "C" fn AAS_StartFrame(mut time: libc::c_float)
 -> libc::c_int {
    aasworld.time = time;
    AAS_UnlinkInvalidEntities();
    AAS_InvalidateEntities();
    AAS_ContinueInit(time);
    aasworld.frameroutingupdates = 0i32;
    if 0 != botDeveloper {
        if 0. !=
               LibVarGetValue(b"showcacheupdates\x00" as *const u8 as
                                  *const libc::c_char) {
            AAS_RoutingInfo();
            LibVarSet(b"showcacheupdates\x00" as *const u8 as
                          *const libc::c_char,
                      b"0\x00" as *const u8 as *const libc::c_char);
        }
        if 0. !=
               LibVarGetValue(b"showmemoryusage\x00" as *const u8 as
                                  *const libc::c_char) {
            PrintUsedMemorySize();
            LibVarSet(b"showmemoryusage\x00" as *const u8 as
                          *const libc::c_char,
                      b"0\x00" as *const u8 as *const libc::c_char);
        }
        if 0. !=
               LibVarGetValue(b"memorydump\x00" as *const u8 as
                                  *const libc::c_char) {
            PrintMemoryLabels();
            LibVarSet(b"memorydump\x00" as *const u8 as *const libc::c_char,
                      b"0\x00" as *const u8 as *const libc::c_char);
        }
    }
    if 0. != (*saveroutingcache).value {
        AAS_WriteRouteCache();
        LibVarSet(b"saveroutingcache\x00" as *const u8 as *const libc::c_char,
                  b"0\x00" as *const u8 as *const libc::c_char);
    }
    aasworld.numframes += 1;
    return 0i32;
}
//end of the function AAS_SetInitialized
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ContinueInit(mut time: libc::c_float) {
    if 0 == aasworld.loaded { return }
    if 0 != aasworld.initialized { return }
    if 0 != AAS_ContinueInitReachability(time) { return }
    AAS_InitClustering();
    if 0 != aasworld.savefile ||
           0 !=
               LibVarGetValue(b"forcewrite\x00" as *const u8 as
                                  *const libc::c_char) as libc::c_int {
        if 0 !=
               LibVarValue(b"aasoptimize\x00" as *const u8 as
                               *const libc::c_char,
                           b"0\x00" as *const u8 as *const libc::c_char) as
                   libc::c_int {
            AAS_Optimize();
        }
        if 0 != AAS_WriteAASFile(aasworld.filename.as_mut_ptr()) as u64 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"%s written successfully\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                aasworld.filename.as_mut_ptr());
        } else {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"couldn\'t write %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                aasworld.filename.as_mut_ptr());
        }
    }
    AAS_InitRouting();
    AAS_SetInitialized();
}