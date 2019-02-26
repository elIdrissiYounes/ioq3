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
    /* ****************************************************************************
 * name:		be_aas.h
 *
 * desc:		Area Awareness System, stuff exported to the AI
 *
 * $Archive: /source/code/botlib/be_aas.h $
 *
 *****************************************************************************/
    //travel flags
    //traveling temporary not possible
    //walking
    //crouching
    //jumping onto a barrier
    //jumping
    //climbing a ladder
    //walking of a ledge
    //swimming
    //jumping out of the water
    //teleporting
    //elevator
    //rocket jumping
    //bfg jumping
    //grappling hook
    //double jump
    //ramp jump
    //strafe jump
    //jump pad
    //travel through air
    //travel through water
    //travel through slime
    //travel through lava
    //travel through donotenter area
    //func bobbing
    //flight
    //move over a bridge
    //
    //not team 1
    //not team 2
    //default travel flags
    pub type unnamed = libc::c_uint;
    // bsp clip, touch on edge
    pub const SOLID_BSP: unnamed = 3;
    // touch on edge
    pub const SOLID_BBOX: unnamed = 2;
    // only touch when inside, after moving
    pub const SOLID_TRIGGER: unnamed = 1;
    // no interaction with other objects
    pub const SOLID_NOT: unnamed = 0;
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
    // BSPTRACE
    //entity state
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_entitystate_s {
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
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
    pub type bot_entitystate_t = bot_entitystate_s;
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_entdata_s {
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub absmins: vec3_t,
        pub absmaxs: vec3_t,
        pub solid: libc::c_int,
        pub modelnum: libc::c_int,
    }
    pub type bsp_entdata_t = bsp_entdata_s;
    use super::be_aas_h::{aas_entityinfo_t};
    use super::{libc};
    use super::aasfile_h::{aas_bbox_t, aas_vertex_t, aas_plane_t, aas_edge_t,
                           aas_edgeindex_t, aas_face_t, aas_faceindex_t,
                           aas_area_t, aas_areasettings_t, aas_reachability_t,
                           aas_node_t, aas_portal_t, aas_portalindex_t,
                           aas_cluster_t};
    use super::q_shared_h::{byte, vec3_t, qboolean};
}
#[header_src =
      "ioq3/code/botlib/be_aas_entity.c"]
pub mod be_aas_entity_c {
    pub const ET_MOVER: unnamed_0 = 4;
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
 * name:		be_aas_entity.c
 *
 * desc:		AAS entities
 *
 * $Archive: /MissionPack/code/botlib/be_aas_entity.c $
 *
 *****************************************************************************/
    //FIXME: these might change
    pub type unnamed_0 = libc::c_uint;
    pub const ET_MISSILE: unnamed_0 = 3;
    pub const ET_ITEM: unnamed_0 = 2;
    pub const ET_PLAYER: unnamed_0 = 1;
    pub const ET_GENERAL: unnamed_0 = 0;
    use super::{libc};
    use super::q_shared_h::{vec_t};
}
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabsf(_: libc::c_float) -> libc::c_float;
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
      "ioq3/code/botlib/be_aas_entity.h"]
pub mod be_aas_entity_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_h::{aas_entityinfo_t};
    use super::botlib_h::{bot_entitystate_t};
    use super::be_aas_def_h::{bsp_entdata_t};
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
      "ioq3/code/botlib/be_aas_reach.h"]
pub mod be_aas_reach_h {
    use super::{libc};
    use super::be_aas_def_h::{aas_link_t};
    extern "C" {
        //
        #[no_mangle]
        pub fn AAS_BestReachableLinkArea(areas: *mut aas_link_t)
         -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_def_h::{bsp_link_t};
    extern "C" {
        //gets the mins, maxs and origin of a BSP model
        #[no_mangle]
        pub fn AAS_BSPModelMinsMaxsOrigin(modelnum: libc::c_int,
                                          angles: *mut vec_t,
                                          mins: *mut vec_t, maxs: *mut vec_t,
                                          origin: *mut vec_t);
        //unlink the given entity from the bsp tree leaves
        #[no_mangle]
        pub fn AAS_UnlinkFromBSPLeaves(leaves: *mut bsp_link_t);
        //link the given entity to the bsp tree leaves of the given model
        #[no_mangle]
        pub fn AAS_BSPLinkEntity(absmins: *mut vec_t, absmaxs: *mut vec_t,
                                 entnum: libc::c_int, modelnum: libc::c_int)
         -> *mut bsp_link_t;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::be_aas_def_h::{aas_link_t};
    use super::q_shared_h::{vec_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn AAS_UnlinkFromAreas(areas: *mut aas_link_t);
        #[no_mangle]
        pub fn AAS_LinkEntityClientBBox(absmins: *mut vec_t,
                                        absmaxs: *mut vec_t,
                                        entnum: libc::c_int,
                                        presencetype: libc::c_int)
         -> *mut aas_link_t;
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
use self::be_aas_h::{aas_entityinfo_s, unnamed, SOLID_BSP, SOLID_BBOX,
                     SOLID_TRIGGER, SOLID_NOT, aas_entityinfo_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     bot_entitystate_s, bot_entitystate_t, botlib_import_s,
                     botlib_import_t};
use self::be_aas_def_h::{aas_entity_t, aas_entity_s, bsp_link_t, bsp_link_s,
                         aas_link_t, aas_link_s, aas_t, aas_s,
                         aas_reachabilityareas_t, aas_reachabilityareas_s,
                         aas_routingcache_t, aas_routingcache_s,
                         aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, bsp_entdata_s, bsp_entdata_t};
use self::be_aas_entity_c::{ET_MOVER, unnamed_0, ET_MISSILE, ET_ITEM,
                            ET_PLAYER, ET_GENERAL};
use self::mathcalls_h::{sqrt, fabsf};
use self::string_h::{memcpy, memset};
use self::be_aas_main_h::{AAS_Time, aasworld};
use self::be_interface_h::{botimport};
use self::be_aas_reach_h::{AAS_BestReachableLinkArea};
use self::be_aas_bsp_h::{AAS_BSPModelMinsMaxsOrigin, AAS_UnlinkFromBSPLeaves,
                         AAS_BSPLinkEntity};
use self::be_aas_sample_h::{AAS_UnlinkFromAreas, AAS_LinkEntityClientBBox};
unsafe extern "C" fn VectorCompare(mut v1: *const vec_t, mut v2: *const vec_t)
 -> libc::c_int {
    if *v1.offset(0isize) != *v2.offset(0isize) ||
           *v1.offset(1isize) != *v2.offset(1isize) ||
           *v1.offset(2isize) != *v2.offset(2isize) {
        return 0i32
    }
    return 1i32;
}
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
//AASINTERN
//returns the size of the entity bounding box in mins and maxs
#[no_mangle]
pub unsafe extern "C" fn AAS_EntitySize(mut entnum: libc::c_int,
                                        mut mins: *mut vec_t,
                                        mut maxs: *mut vec_t) {
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    if 0 == aasworld.initialized { return }
    if entnum < 0i32 || entnum >= aasworld.maxentities {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntitySize: entnum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            entnum);
        return
    }
    ent =
        &mut *aasworld.entities.offset(entnum as isize) as *mut aas_entity_t;
    *mins.offset(0isize) = (*ent).i.mins[0usize];
    *mins.offset(1isize) = (*ent).i.mins[1usize];
    *mins.offset(2isize) = (*ent).i.mins[2usize];
    *maxs.offset(0isize) = (*ent).i.maxs[0usize];
    *maxs.offset(1isize) = (*ent).i.maxs[1usize];
    *maxs.offset(2isize) = (*ent).i.maxs[2usize];
}
//returns the BSP model number of the entity
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityModelNum(mut entnum: libc::c_int)
 -> libc::c_int {
    if 0 == aasworld.initialized { return 0i32 }
    if entnum < 0i32 || entnum >= aasworld.maxentities {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntityModelNum: entnum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            entnum);
        return 0i32
    }
    return (*aasworld.entities.offset(entnum as isize)).i.modelindex;
}
//returns the origin of an entity with the given model number
#[no_mangle]
pub unsafe extern "C" fn AAS_OriginOfMoverWithModelNum(mut modelnum:
                                                           libc::c_int,
                                                       mut origin: *mut vec_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    i = 0i32;
    while i < aasworld.maxentities {
        ent = &mut *aasworld.entities.offset(i as isize) as *mut aas_entity_t;
        if (*ent).i.type_0 == ET_MOVER as libc::c_int {
            if (*ent).i.modelindex == modelnum {
                *origin.offset(0isize) = (*ent).i.origin[0usize];
                *origin.offset(1isize) = (*ent).i.origin[1usize];
                *origin.offset(2isize) = (*ent).i.origin[2usize];
                return qtrue as libc::c_int
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
//returns the best reachable area the entity is situated in
#[no_mangle]
pub unsafe extern "C" fn AAS_BestReachableEntityArea(mut entnum: libc::c_int)
 -> libc::c_int {
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    ent =
        &mut *aasworld.entities.offset(entnum as isize) as *mut aas_entity_t;
    return AAS_BestReachableLinkArea((*ent).areas);
}
//returns the info of the given entity
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityInfo(mut entnum: libc::c_int,
                                        mut info: *mut aas_entityinfo_t) {
    if 0 == aasworld.initialized {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntityInfo: aasworld not initialized\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        memset(info as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<aas_entityinfo_t>() as libc::c_ulong);
        return
    }
    if entnum < 0i32 || entnum >= aasworld.maxentities {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntityInfo: entnum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            entnum);
        memset(info as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<aas_entityinfo_t>() as libc::c_ulong);
        return
    }
    memcpy(info as *mut libc::c_void,
           &mut (*aasworld.entities.offset(entnum as isize)).i as
               *mut aas_entityinfo_t as *const libc::c_void,
           ::std::mem::size_of::<aas_entityinfo_t>() as libc::c_ulong);
}
//returns the next entity
#[no_mangle]
pub unsafe extern "C" fn AAS_NextEntity(mut entnum: libc::c_int)
 -> libc::c_int {
    if 0 == aasworld.loaded { return 0i32 }
    if entnum < 0i32 { entnum = -1i32 }
    loop  {
        entnum += 1;
        if !(entnum < aasworld.maxentities) { break ; }
        if 0 != (*aasworld.entities.offset(entnum as isize)).i.valid {
            return entnum
        }
    }
    return 0i32;
}
//returns the origin of the entity
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityOrigin(mut entnum: libc::c_int,
                                          mut origin: *mut vec_t) {
    if entnum < 0i32 || entnum >= aasworld.maxentities {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntityOrigin: entnum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            entnum);
        let ref mut fresh1 = *origin.offset(1isize);
        let ref mut fresh0 = *origin.offset(2isize);
        *fresh0 = 0i32 as vec_t;
        *fresh1 = *fresh0;
        *origin.offset(0isize) = *fresh1;
        return
    }
    *origin.offset(0isize) =
        (*aasworld.entities.offset(entnum as isize)).i.origin[0usize];
    *origin.offset(1isize) =
        (*aasworld.entities.offset(entnum as isize)).i.origin[1usize];
    *origin.offset(2isize) =
        (*aasworld.entities.offset(entnum as isize)).i.origin[2usize];
}
//returns the entity type
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityType(mut entnum: libc::c_int)
 -> libc::c_int {
    if 0 == aasworld.initialized { return 0i32 }
    if entnum < 0i32 || entnum >= aasworld.maxentities {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntityType: entnum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            entnum);
        return 0i32
    }
    return (*aasworld.entities.offset(entnum as isize)).i.type_0;
}
//returns the model index of the entity
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityModelindex(mut entnum: libc::c_int)
 -> libc::c_int {
    if entnum < 0i32 || entnum >= aasworld.maxentities {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"AAS_EntityModelindex: entnum %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            entnum);
        return 0i32
    }
    return (*aasworld.entities.offset(entnum as isize)).i.modelindex;
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
 * name:		be_aas_entity.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_entity.h $
 *
 *****************************************************************************/
//invalidates all entity infos
#[no_mangle]
pub unsafe extern "C" fn AAS_InvalidateEntities() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < aasworld.maxentities {
        (*aasworld.entities.offset(i as isize)).i.valid =
            qfalse as libc::c_int;
        (*aasworld.entities.offset(i as isize)).i.number = i;
        i += 1
    };
}
//unlink not updated entities
#[no_mangle]
pub unsafe extern "C" fn AAS_UnlinkInvalidEntities() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    i = 0i32;
    while i < aasworld.maxentities {
        ent = &mut *aasworld.entities.offset(i as isize) as *mut aas_entity_t;
        if 0 == (*ent).i.valid {
            AAS_UnlinkFromAreas((*ent).areas);
            (*ent).areas = 0 as *mut aas_link_t;
            AAS_UnlinkFromBSPLeaves((*ent).leaves);
            (*ent).leaves = 0 as *mut bsp_link_t
        }
        i += 1
    };
}
//resets the entity AAS and BSP links (sets areas and leaves pointers to NULL)
#[no_mangle]
pub unsafe extern "C" fn AAS_ResetEntityLinks() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < aasworld.maxentities {
        let ref mut fresh2 = (*aasworld.entities.offset(i as isize)).areas;
        *fresh2 = 0 as *mut aas_link_t;
        let ref mut fresh3 = (*aasworld.entities.offset(i as isize)).leaves;
        *fresh3 = 0 as *mut bsp_link_t;
        i += 1
    };
}
//updates an entity
#[no_mangle]
pub unsafe extern "C" fn AAS_UpdateEntity(mut entnum: libc::c_int,
                                          mut state: *mut bot_entitystate_t)
 -> libc::c_int {
    let mut relink: libc::c_int = 0;
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    if 0 == aasworld.loaded {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"AAS_UpdateEntity: not loaded\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 3i32
    }
    ent =
        &mut *aasworld.entities.offset(entnum as isize) as *mut aas_entity_t;
    if state.is_null() {
        AAS_UnlinkFromAreas((*ent).areas);
        AAS_UnlinkFromBSPLeaves((*ent).leaves);
        (*ent).areas = 0 as *mut aas_link_t;
        (*ent).leaves = 0 as *mut bsp_link_t;
        return 0i32
    }
    (*ent).i.update_time = AAS_Time() - (*ent).i.ltime;
    (*ent).i.type_0 = (*state).type_0;
    (*ent).i.flags = (*state).flags;
    (*ent).i.ltime = AAS_Time();
    (*ent).i.lastvisorigin[0usize] = (*ent).i.origin[0usize];
    (*ent).i.lastvisorigin[1usize] = (*ent).i.origin[1usize];
    (*ent).i.lastvisorigin[2usize] = (*ent).i.origin[2usize];
    (*ent).i.old_origin[0usize] = (*state).old_origin[0usize];
    (*ent).i.old_origin[1usize] = (*state).old_origin[1usize];
    (*ent).i.old_origin[2usize] = (*state).old_origin[2usize];
    (*ent).i.solid = (*state).solid;
    (*ent).i.groundent = (*state).groundent;
    (*ent).i.modelindex = (*state).modelindex;
    (*ent).i.modelindex2 = (*state).modelindex2;
    (*ent).i.frame = (*state).frame;
    (*ent).i.event = (*state).event;
    (*ent).i.eventParm = (*state).eventParm;
    (*ent).i.powerups = (*state).powerups;
    (*ent).i.weapon = (*state).weapon;
    (*ent).i.legsAnim = (*state).legsAnim;
    (*ent).i.torsoAnim = (*state).torsoAnim;
    (*ent).i.number = entnum;
    (*ent).i.valid = qtrue as libc::c_int;
    if aasworld.numframes == 1i32 {
        relink = qtrue as libc::c_int
    } else { relink = qfalse as libc::c_int }
    if (*ent).i.solid == SOLID_BSP as libc::c_int {
        if 0 ==
               VectorCompare((*state).angles.as_mut_ptr() as *const vec_t,
                             (*ent).i.angles.as_mut_ptr() as *const vec_t) {
            (*ent).i.angles[0usize] = (*state).angles[0usize];
            (*ent).i.angles[1usize] = (*state).angles[1usize];
            (*ent).i.angles[2usize] = (*state).angles[2usize];
            relink = qtrue as libc::c_int
        }
        AAS_BSPModelMinsMaxsOrigin((*ent).i.modelindex,
                                   (*ent).i.angles.as_mut_ptr(),
                                   (*ent).i.mins.as_mut_ptr(),
                                   (*ent).i.maxs.as_mut_ptr(),
                                   0 as *mut vec_t);
    } else if (*ent).i.solid == SOLID_BBOX as libc::c_int {
        if 0 ==
               VectorCompare((*state).mins.as_mut_ptr() as *const vec_t,
                             (*ent).i.mins.as_mut_ptr() as *const vec_t) ||
               0 ==
                   VectorCompare((*state).maxs.as_mut_ptr() as *const vec_t,
                                 (*ent).i.maxs.as_mut_ptr() as *const vec_t) {
            (*ent).i.mins[0usize] = (*state).mins[0usize];
            (*ent).i.mins[1usize] = (*state).mins[1usize];
            (*ent).i.mins[2usize] = (*state).mins[2usize];
            (*ent).i.maxs[0usize] = (*state).maxs[0usize];
            (*ent).i.maxs[1usize] = (*state).maxs[1usize];
            (*ent).i.maxs[2usize] = (*state).maxs[2usize];
            relink = qtrue as libc::c_int
        }
        (*ent).i.angles[0usize] = (*state).angles[0usize];
        (*ent).i.angles[1usize] = (*state).angles[1usize];
        (*ent).i.angles[2usize] = (*state).angles[2usize]
    }
    if 0 ==
           VectorCompare((*state).origin.as_mut_ptr() as *const vec_t,
                         (*ent).i.origin.as_mut_ptr() as *const vec_t) {
        (*ent).i.origin[0usize] = (*state).origin[0usize];
        (*ent).i.origin[1usize] = (*state).origin[1usize];
        (*ent).i.origin[2usize] = (*state).origin[2usize];
        relink = qtrue as libc::c_int
    }
    if 0 != relink {
        if entnum != (1i32 << 10i32) - 2i32 {
            absmins[0usize] = (*ent).i.mins[0usize] + (*ent).i.origin[0usize];
            absmins[1usize] = (*ent).i.mins[1usize] + (*ent).i.origin[1usize];
            absmins[2usize] = (*ent).i.mins[2usize] + (*ent).i.origin[2usize];
            absmaxs[0usize] = (*ent).i.maxs[0usize] + (*ent).i.origin[0usize];
            absmaxs[1usize] = (*ent).i.maxs[1usize] + (*ent).i.origin[1usize];
            absmaxs[2usize] = (*ent).i.maxs[2usize] + (*ent).i.origin[2usize];
            AAS_UnlinkFromAreas((*ent).areas);
            (*ent).areas =
                AAS_LinkEntityClientBBox(absmins.as_mut_ptr(),
                                         absmaxs.as_mut_ptr(), entnum, 2i32);
            AAS_UnlinkFromBSPLeaves((*ent).leaves);
            (*ent).leaves =
                AAS_BSPLinkEntity(absmins.as_mut_ptr(), absmaxs.as_mut_ptr(),
                                  entnum, 0i32)
        }
    }
    return 0i32;
}
//gives the entity data used for collision detection
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityBSPData(mut entnum: libc::c_int,
                                           mut entdata: *mut bsp_entdata_t) {
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    ent =
        &mut *aasworld.entities.offset(entnum as isize) as *mut aas_entity_t;
    (*entdata).origin[0usize] = (*ent).i.origin[0usize];
    (*entdata).origin[1usize] = (*ent).i.origin[1usize];
    (*entdata).origin[2usize] = (*ent).i.origin[2usize];
    (*entdata).angles[0usize] = (*ent).i.angles[0usize];
    (*entdata).angles[1usize] = (*ent).i.angles[1usize];
    (*entdata).angles[2usize] = (*ent).i.angles[2usize];
    (*entdata).absmins[0usize] =
        (*ent).i.origin[0usize] + (*ent).i.mins[0usize];
    (*entdata).absmins[1usize] =
        (*ent).i.origin[1usize] + (*ent).i.mins[1usize];
    (*entdata).absmins[2usize] =
        (*ent).i.origin[2usize] + (*ent).i.mins[2usize];
    (*entdata).absmaxs[0usize] =
        (*ent).i.origin[0usize] + (*ent).i.maxs[0usize];
    (*entdata).absmaxs[1usize] =
        (*ent).i.origin[1usize] + (*ent).i.maxs[1usize];
    (*entdata).absmaxs[2usize] =
        (*ent).i.origin[2usize] + (*ent).i.maxs[2usize];
    (*entdata).solid = (*ent).i.solid;
    (*entdata).modelnum = (*ent).i.modelindex - 1i32;
}
//end for
//end for
//end of the function AAS_UnlinkInvalidEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_NearestEntity(mut origin: *mut vec_t,
                                           mut modelindex: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bestentnum: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut ent: *mut aas_entity_t = 0 as *mut aas_entity_t;
    let mut dir: vec3_t = [0.; 3];
    bestentnum = 0i32;
    bestdist = 99999i32 as libc::c_float;
    i = 0i32;
    while i < aasworld.maxentities {
        ent = &mut *aasworld.entities.offset(i as isize) as *mut aas_entity_t;
        if !((*ent).i.modelindex != modelindex) {
            dir[0usize] = (*ent).i.origin[0usize] - *origin.offset(0isize);
            dir[1usize] = (*ent).i.origin[1usize] - *origin.offset(1isize);
            dir[2usize] = (*ent).i.origin[2usize] - *origin.offset(2isize);
            if fabsf(dir[0usize]) < 40i32 as libc::c_float {
                if fabsf(dir[1usize]) < 40i32 as libc::c_float {
                    dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
                    if dist < bestdist { bestdist = dist; bestentnum = i }
                }
            }
        }
        i += 1
    }
    return bestentnum;
}