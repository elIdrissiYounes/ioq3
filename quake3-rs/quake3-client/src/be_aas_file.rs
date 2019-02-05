use libc;
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
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
    pub type unnamed = libc::c_uint;
    pub const FS_SEEK_SET: unnamed = 2;
    pub const FS_SEEK_END: unnamed = 1;
    pub const FS_SEEK_CUR: unnamed = 0;
    pub type cplane_t = cplane_s;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/aasfile.h"]
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
    //=========== aas file ===============
    //header lump
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_lump_t {
        pub fileofs: libc::c_int,
        pub filelen: libc::c_int,
    }
    //aas file header
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_header_s {
        pub ident: libc::c_int,
        pub version: libc::c_int,
        pub bspchecksum: libc::c_int,
        pub lumps: [aas_lump_t; 14],
    }
    pub type aas_header_t = aas_header_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_def.h"]
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
        pub traveltimes: [libc::c_ushort; 1],
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
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //gets the string of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_main.h"]
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
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_file.h"]
pub mod be_aas_file_h {
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_file.c"]
pub mod be_aas_file_c {
    use super::{libc};
    use super::q_shared_h::{fileHandle_t};
    use super::aasfile_h::{aas_header_t};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_variadic.h"]
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
                       FS_WRITE, FS_READ, unnamed, FS_SEEK_SET, FS_SEEK_END,
                       FS_SEEK_CUR, cplane_t};
use self::aasfile_h::{aas_bbox_s, aas_bbox_t, aas_reachability_s,
                      aas_reachability_t, aas_areasettings_s,
                      aas_areasettings_t, aas_portal_s, aas_portal_t,
                      aas_portalindex_t, aas_cluster_s, aas_cluster_t,
                      aas_vertex_t, aas_plane_s, aas_plane_t, aas_edge_s,
                      aas_edge_t, aas_edgeindex_t, aas_face_s, aas_face_t,
                      aas_faceindex_t, aas_area_s, aas_area_t, aas_node_s,
                      aas_node_t, aas_lump_t, aas_header_s, aas_header_t};
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
use self::stdlib_h::{atoi};
use self::l_memory_h::{GetClearedHunkMemory, FreeMemory};
use self::l_libvar_h::{LibVarGetString};
use self::be_interface_h::{botimport};
use self::be_aas_main_h::{aasworld};
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
 * name:		be_aas_file.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_file.h $
 *
 *****************************************************************************/
//loads the AAS file with the given name
#[no_mangle]
pub unsafe extern "C" fn AAS_LoadAASFile(mut filename: *mut libc::c_char)
 -> libc::c_int {
    let mut fp: fileHandle_t = 0;
    let mut header: aas_header_t =
        aas_header_s{ident: 0,
                     version: 0,
                     bspchecksum: 0,
                     lumps: [aas_lump_t{fileofs: 0, filelen: 0,}; 14],};
    let mut offset: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut lastoffset: libc::c_int = 0;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"trying to load %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename);
    AAS_DumpAASData();
    botimport.FS_FOpenFile.expect("non-null function pointer")(filename,
                                                               &mut fp,
                                                               FS_READ);
    if 0 == fp {
        AAS_Error(b"can\'t open %s\n\x00" as *const u8 as *const libc::c_char
                      as *mut libc::c_char, filename);
        return 4i32
    }
    botimport.FS_Read.expect("non-null function pointer")(&mut header as
                                                              *mut aas_header_t
                                                              as
                                                              *mut libc::c_void,
                                                          ::std::mem::size_of::<aas_header_t>()
                                                              as libc::c_ulong
                                                              as libc::c_int,
                                                          fp);
    lastoffset =
        ::std::mem::size_of::<aas_header_t>() as libc::c_ulong as libc::c_int;
    header.ident = header.ident;
    if header.ident !=
           (('S' as i32) << 24i32) + (('A' as i32) << 16i32) +
               (('A' as i32) << 8i32) + 'E' as i32 {
        AAS_Error(b"%s is not an AAS file\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char, filename);
        botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
        return 5i32
    }
    header.version = header.version;
    if header.version != 4i32 && header.version != 5i32 {
        AAS_Error(b"aas file %s is version %i, not %i\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char, filename,
                  header.version, 5i32);
        botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
        return 6i32
    }
    if header.version == 5i32 {
        AAS_DData((&mut header as *mut aas_header_t as
                       *mut libc::c_uchar).offset(8isize),
                  (::std::mem::size_of::<aas_header_t>() as
                       libc::c_ulong).wrapping_sub(8i32 as libc::c_ulong) as
                      libc::c_int);
    }
    aasworld.bspchecksum =
        atoi(LibVarGetString(b"sv_mapChecksum\x00" as *const u8 as
                                 *const libc::c_char));
    if header.bspchecksum != aasworld.bspchecksum {
        AAS_Error(b"aas file %s is out of date\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char, filename);
        botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
        return 6i32
    }
    offset = header.lumps[0usize].fileofs;
    length = header.lumps[0usize].filelen;
    aasworld.bboxes =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_bbox_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_bbox_t;
    aasworld.numbboxes =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_bbox_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numbboxes && aasworld.bboxes.is_null() { return 7i32 }
    offset = header.lumps[1usize].fileofs;
    length = header.lumps[1usize].filelen;
    aasworld.vertexes =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_vertex_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_vertex_t;
    aasworld.numvertexes =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_vertex_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numvertexes && aasworld.vertexes.is_null() {
        return 7i32
    }
    offset = header.lumps[2usize].fileofs;
    length = header.lumps[2usize].filelen;
    aasworld.planes =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_plane_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_plane_t;
    aasworld.numplanes =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_plane_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numplanes && aasworld.planes.is_null() { return 7i32 }
    offset = header.lumps[3usize].fileofs;
    length = header.lumps[3usize].filelen;
    aasworld.edges =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_edge_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_edge_t;
    aasworld.numedges =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_edge_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numedges && aasworld.edges.is_null() { return 7i32 }
    offset = header.lumps[4usize].fileofs;
    length = header.lumps[4usize].filelen;
    aasworld.edgeindex =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_edgeindex_t>() as
                            libc::c_ulong as libc::c_int) as
            *mut aas_edgeindex_t;
    aasworld.edgeindexsize =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_edgeindex_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.edgeindexsize && aasworld.edgeindex.is_null() {
        return 7i32
    }
    offset = header.lumps[5usize].fileofs;
    length = header.lumps[5usize].filelen;
    aasworld.faces =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_face_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_face_t;
    aasworld.numfaces =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_face_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numfaces && aasworld.faces.is_null() { return 7i32 }
    offset = header.lumps[6usize].fileofs;
    length = header.lumps[6usize].filelen;
    aasworld.faceindex =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_faceindex_t>() as
                            libc::c_ulong as libc::c_int) as
            *mut aas_faceindex_t;
    aasworld.faceindexsize =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_faceindex_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.faceindexsize && aasworld.faceindex.is_null() {
        return 7i32
    }
    offset = header.lumps[7usize].fileofs;
    length = header.lumps[7usize].filelen;
    aasworld.areas =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_area_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_area_t;
    aasworld.numareas =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_area_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numareas && aasworld.areas.is_null() { return 7i32 }
    offset = header.lumps[8usize].fileofs;
    length = header.lumps[8usize].filelen;
    aasworld.areasettings =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_areasettings_t>() as
                            libc::c_ulong as libc::c_int) as
            *mut aas_areasettings_t;
    aasworld.numareasettings =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_areasettings_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numareasettings && aasworld.areasettings.is_null() {
        return 7i32
    }
    offset = header.lumps[9usize].fileofs;
    length = header.lumps[9usize].filelen;
    aasworld.reachability =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_reachability_t>() as
                            libc::c_ulong as libc::c_int) as
            *mut aas_reachability_t;
    aasworld.reachabilitysize =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_reachability_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.reachabilitysize && aasworld.reachability.is_null() {
        return 7i32
    }
    offset = header.lumps[10usize].fileofs;
    length = header.lumps[10usize].filelen;
    aasworld.nodes =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_node_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_node_t;
    aasworld.numnodes =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_node_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numnodes && aasworld.nodes.is_null() { return 7i32 }
    offset = header.lumps[11usize].fileofs;
    length = header.lumps[11usize].filelen;
    aasworld.portals =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_portal_t>() as libc::c_ulong
                            as libc::c_int) as *mut aas_portal_t;
    aasworld.numportals =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_portal_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numportals && aasworld.portals.is_null() { return 7i32 }
    offset = header.lumps[12usize].fileofs;
    length = header.lumps[12usize].filelen;
    aasworld.portalindex =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_portalindex_t>() as
                            libc::c_ulong as libc::c_int) as
            *mut aas_portalindex_t;
    aasworld.portalindexsize =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_portalindex_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.portalindexsize && aasworld.portalindex.is_null() {
        return 7i32
    }
    offset = header.lumps[13usize].fileofs;
    length = header.lumps[13usize].filelen;
    aasworld.clusters =
        AAS_LoadAASLump(fp, offset, length, &mut lastoffset,
                        ::std::mem::size_of::<aas_cluster_t>() as
                            libc::c_ulong as libc::c_int) as
            *mut aas_cluster_t;
    aasworld.numclusters =
        (length as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<aas_cluster_t>()
                                             as libc::c_ulong) as libc::c_int;
    if 0 != aasworld.numclusters && aasworld.clusters.is_null() {
        return 7i32
    }
    AAS_SwapAASData();
    aasworld.loaded = qtrue as libc::c_int;
    botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
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
 * name:		be_aas_file.c
 *
 * desc:		AAS file loading/writing
 *
 * $Archive: /MissionPack/code/botlib/be_aas_file.c $
 *
 *****************************************************************************/
//#define AASFILEDEBUG
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_SwapAASData() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while i < aasworld.numbboxes {
        (*aasworld.bboxes.offset(i as isize)).presencetype =
            (*aasworld.bboxes.offset(i as isize)).presencetype;
        (*aasworld.bboxes.offset(i as isize)).flags =
            (*aasworld.bboxes.offset(i as isize)).flags;
        j = 0i32;
        while j < 3i32 {
            (*aasworld.bboxes.offset(i as isize)).mins[j as usize] =
                (*aasworld.bboxes.offset(i as isize)).mins[j as usize];
            (*aasworld.bboxes.offset(i as isize)).maxs[j as usize] =
                (*aasworld.bboxes.offset(i as isize)).maxs[j as usize];
            j += 1
        }
        i += 1
    }
    i = 0i32;
    while i < aasworld.numvertexes {
        j = 0i32;
        while j < 3i32 {
            (*aasworld.vertexes.offset(i as isize))[j as usize] =
                (*aasworld.vertexes.offset(i as isize))[j as usize];
            j += 1
        }
        i += 1
    }
    i = 0i32;
    while i < aasworld.numplanes {
        j = 0i32;
        while j < 3i32 {
            (*aasworld.planes.offset(i as isize)).normal[j as usize] =
                (*aasworld.planes.offset(i as isize)).normal[j as usize];
            j += 1
        }
        (*aasworld.planes.offset(i as isize)).dist =
            (*aasworld.planes.offset(i as isize)).dist;
        (*aasworld.planes.offset(i as isize)).type_0 =
            (*aasworld.planes.offset(i as isize)).type_0;
        i += 1
    }
    i = 0i32;
    while i < aasworld.numedges {
        (*aasworld.edges.offset(i as isize)).v[0usize] =
            (*aasworld.edges.offset(i as isize)).v[0usize];
        (*aasworld.edges.offset(i as isize)).v[1usize] =
            (*aasworld.edges.offset(i as isize)).v[1usize];
        i += 1
    }
    i = 0i32;
    while i < aasworld.edgeindexsize {
        *aasworld.edgeindex.offset(i as isize) =
            *aasworld.edgeindex.offset(i as isize);
        i += 1
    }
    i = 0i32;
    while i < aasworld.numfaces {
        (*aasworld.faces.offset(i as isize)).planenum =
            (*aasworld.faces.offset(i as isize)).planenum;
        (*aasworld.faces.offset(i as isize)).faceflags =
            (*aasworld.faces.offset(i as isize)).faceflags;
        (*aasworld.faces.offset(i as isize)).numedges =
            (*aasworld.faces.offset(i as isize)).numedges;
        (*aasworld.faces.offset(i as isize)).firstedge =
            (*aasworld.faces.offset(i as isize)).firstedge;
        (*aasworld.faces.offset(i as isize)).frontarea =
            (*aasworld.faces.offset(i as isize)).frontarea;
        (*aasworld.faces.offset(i as isize)).backarea =
            (*aasworld.faces.offset(i as isize)).backarea;
        i += 1
    }
    i = 0i32;
    while i < aasworld.faceindexsize {
        *aasworld.faceindex.offset(i as isize) =
            *aasworld.faceindex.offset(i as isize);
        i += 1
    }
    i = 0i32;
    while i < aasworld.numareas {
        (*aasworld.areas.offset(i as isize)).areanum =
            (*aasworld.areas.offset(i as isize)).areanum;
        (*aasworld.areas.offset(i as isize)).numfaces =
            (*aasworld.areas.offset(i as isize)).numfaces;
        (*aasworld.areas.offset(i as isize)).firstface =
            (*aasworld.areas.offset(i as isize)).firstface;
        j = 0i32;
        while j < 3i32 {
            (*aasworld.areas.offset(i as isize)).mins[j as usize] =
                (*aasworld.areas.offset(i as isize)).mins[j as usize];
            (*aasworld.areas.offset(i as isize)).maxs[j as usize] =
                (*aasworld.areas.offset(i as isize)).maxs[j as usize];
            (*aasworld.areas.offset(i as isize)).center[j as usize] =
                (*aasworld.areas.offset(i as isize)).center[j as usize];
            j += 1
        }
        i += 1
    }
    i = 0i32;
    while i < aasworld.numareasettings {
        (*aasworld.areasettings.offset(i as isize)).contents =
            (*aasworld.areasettings.offset(i as isize)).contents;
        (*aasworld.areasettings.offset(i as isize)).areaflags =
            (*aasworld.areasettings.offset(i as isize)).areaflags;
        (*aasworld.areasettings.offset(i as isize)).presencetype =
            (*aasworld.areasettings.offset(i as isize)).presencetype;
        (*aasworld.areasettings.offset(i as isize)).cluster =
            (*aasworld.areasettings.offset(i as isize)).cluster;
        (*aasworld.areasettings.offset(i as isize)).clusterareanum =
            (*aasworld.areasettings.offset(i as isize)).clusterareanum;
        (*aasworld.areasettings.offset(i as isize)).numreachableareas =
            (*aasworld.areasettings.offset(i as isize)).numreachableareas;
        (*aasworld.areasettings.offset(i as isize)).firstreachablearea =
            (*aasworld.areasettings.offset(i as isize)).firstreachablearea;
        i += 1
    }
    i = 0i32;
    while i < aasworld.reachabilitysize {
        (*aasworld.reachability.offset(i as isize)).areanum =
            (*aasworld.reachability.offset(i as isize)).areanum;
        (*aasworld.reachability.offset(i as isize)).facenum =
            (*aasworld.reachability.offset(i as isize)).facenum;
        (*aasworld.reachability.offset(i as isize)).edgenum =
            (*aasworld.reachability.offset(i as isize)).edgenum;
        j = 0i32;
        while j < 3i32 {
            (*aasworld.reachability.offset(i as isize)).start[j as usize] =
                (*aasworld.reachability.offset(i as isize)).start[j as usize];
            (*aasworld.reachability.offset(i as isize)).end[j as usize] =
                (*aasworld.reachability.offset(i as isize)).end[j as usize];
            j += 1
        }
        (*aasworld.reachability.offset(i as isize)).traveltype =
            (*aasworld.reachability.offset(i as isize)).traveltype;
        (*aasworld.reachability.offset(i as isize)).traveltime =
            (*aasworld.reachability.offset(i as isize)).traveltime;
        i += 1
    }
    i = 0i32;
    while i < aasworld.numnodes {
        (*aasworld.nodes.offset(i as isize)).planenum =
            (*aasworld.nodes.offset(i as isize)).planenum;
        (*aasworld.nodes.offset(i as isize)).children[0usize] =
            (*aasworld.nodes.offset(i as isize)).children[0usize];
        (*aasworld.nodes.offset(i as isize)).children[1usize] =
            (*aasworld.nodes.offset(i as isize)).children[1usize];
        i += 1
    }
    i = 0i32;
    while i < aasworld.numportals {
        (*aasworld.portals.offset(i as isize)).areanum =
            (*aasworld.portals.offset(i as isize)).areanum;
        (*aasworld.portals.offset(i as isize)).frontcluster =
            (*aasworld.portals.offset(i as isize)).frontcluster;
        (*aasworld.portals.offset(i as isize)).backcluster =
            (*aasworld.portals.offset(i as isize)).backcluster;
        (*aasworld.portals.offset(i as isize)).clusterareanum[0usize] =
            (*aasworld.portals.offset(i as isize)).clusterareanum[0usize];
        (*aasworld.portals.offset(i as isize)).clusterareanum[1usize] =
            (*aasworld.portals.offset(i as isize)).clusterareanum[1usize];
        i += 1
    }
    i = 0i32;
    while i < aasworld.portalindexsize {
        *aasworld.portalindex.offset(i as isize) =
            *aasworld.portalindex.offset(i as isize);
        i += 1
    }
    i = 0i32;
    while i < aasworld.numclusters {
        (*aasworld.clusters.offset(i as isize)).numareas =
            (*aasworld.clusters.offset(i as isize)).numareas;
        (*aasworld.clusters.offset(i as isize)).numreachabilityareas =
            (*aasworld.clusters.offset(i as isize)).numreachabilityareas;
        (*aasworld.clusters.offset(i as isize)).numportals =
            (*aasworld.clusters.offset(i as isize)).numportals;
        (*aasworld.clusters.offset(i as isize)).firstportal =
            (*aasworld.clusters.offset(i as isize)).firstportal;
        i += 1
    };
}
//end of the function AAS_DumpAASData
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
//AASFILEDEBUG
//===========================================================================
// allocate memory and read a lump of an AAS file
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_LoadAASLump(mut fp: fileHandle_t,
                                         mut offset: libc::c_int,
                                         mut length: libc::c_int,
                                         mut lastoffset: *mut libc::c_int,
                                         mut size: libc::c_int)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == length {
        return GetClearedHunkMemory((size + 1i32) as libc::c_ulong) as
                   *mut libc::c_char
    }
    if offset != *lastoffset {
        botimport.Print.expect("non-null function pointer")(2i32,
                                                            b"AAS file not sequentially read\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        if 0 !=
               botimport.FS_Seek.expect("non-null function pointer")(fp,
                                                                     offset as
                                                                         libc::c_long,
                                                                     FS_SEEK_SET
                                                                         as
                                                                         libc::c_int)
           {
            AAS_Error(b"can\'t seek to aas lump\n\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
            AAS_DumpAASData();
            botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
            return 0 as *mut libc::c_char
        }
    }
    buf =
        GetClearedHunkMemory((length + 1i32) as libc::c_ulong) as
            *mut libc::c_char;
    if 0 != length {
        botimport.FS_Read.expect("non-null function pointer")(buf as
                                                                  *mut libc::c_void,
                                                              length, fp);
        *lastoffset += length
    }
    return buf;
}
//dumps the loaded AAS data
#[no_mangle]
pub unsafe extern "C" fn AAS_DumpAASData() {
    aasworld.numbboxes = 0i32;
    if !aasworld.bboxes.is_null() {
        FreeMemory(aasworld.bboxes as *mut libc::c_void);
    }
    aasworld.bboxes = 0 as *mut aas_bbox_t;
    aasworld.numvertexes = 0i32;
    if !aasworld.vertexes.is_null() {
        FreeMemory(aasworld.vertexes as *mut libc::c_void);
    }
    aasworld.vertexes = 0 as *mut aas_vertex_t;
    aasworld.numplanes = 0i32;
    if !aasworld.planes.is_null() {
        FreeMemory(aasworld.planes as *mut libc::c_void);
    }
    aasworld.planes = 0 as *mut aas_plane_t;
    aasworld.numedges = 0i32;
    if !aasworld.edges.is_null() {
        FreeMemory(aasworld.edges as *mut libc::c_void);
    }
    aasworld.edges = 0 as *mut aas_edge_t;
    aasworld.edgeindexsize = 0i32;
    if !aasworld.edgeindex.is_null() {
        FreeMemory(aasworld.edgeindex as *mut libc::c_void);
    }
    aasworld.edgeindex = 0 as *mut aas_edgeindex_t;
    aasworld.numfaces = 0i32;
    if !aasworld.faces.is_null() {
        FreeMemory(aasworld.faces as *mut libc::c_void);
    }
    aasworld.faces = 0 as *mut aas_face_t;
    aasworld.faceindexsize = 0i32;
    if !aasworld.faceindex.is_null() {
        FreeMemory(aasworld.faceindex as *mut libc::c_void);
    }
    aasworld.faceindex = 0 as *mut aas_faceindex_t;
    aasworld.numareas = 0i32;
    if !aasworld.areas.is_null() {
        FreeMemory(aasworld.areas as *mut libc::c_void);
    }
    aasworld.areas = 0 as *mut aas_area_t;
    aasworld.numareasettings = 0i32;
    if !aasworld.areasettings.is_null() {
        FreeMemory(aasworld.areasettings as *mut libc::c_void);
    }
    aasworld.areasettings = 0 as *mut aas_areasettings_t;
    aasworld.reachabilitysize = 0i32;
    if !aasworld.reachability.is_null() {
        FreeMemory(aasworld.reachability as *mut libc::c_void);
    }
    aasworld.reachability = 0 as *mut aas_reachability_t;
    aasworld.numnodes = 0i32;
    if !aasworld.nodes.is_null() {
        FreeMemory(aasworld.nodes as *mut libc::c_void);
    }
    aasworld.nodes = 0 as *mut aas_node_t;
    aasworld.numportals = 0i32;
    if !aasworld.portals.is_null() {
        FreeMemory(aasworld.portals as *mut libc::c_void);
    }
    aasworld.portals = 0 as *mut aas_portal_t;
    aasworld.numportals = 0i32;
    if !aasworld.portalindex.is_null() {
        FreeMemory(aasworld.portalindex as *mut libc::c_void);
    }
    aasworld.portalindex = 0 as *mut aas_portalindex_t;
    aasworld.portalindexsize = 0i32;
    if !aasworld.clusters.is_null() {
        FreeMemory(aasworld.clusters as *mut libc::c_void);
    }
    aasworld.clusters = 0 as *mut aas_cluster_t;
    aasworld.numclusters = 0i32;
    aasworld.loaded = qfalse as libc::c_int;
    aasworld.initialized = qfalse as libc::c_int;
    aasworld.savefile = qfalse as libc::c_int;
}
//end of the function AAS_LoadAASLump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_DData(mut data: *mut libc::c_uchar,
                                   mut size: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < size {
        let ref mut fresh0 = *data.offset(i as isize);
        *fresh0 =
            (*fresh0 as libc::c_int ^
                 i as libc::c_uchar as libc::c_int * 119i32) as libc::c_uchar;
        i += 1
    };
}
//writes an AAS file with the given name
#[no_mangle]
pub unsafe extern "C" fn AAS_WriteAASFile(mut filename: *mut libc::c_char)
 -> qboolean {
    let mut header: aas_header_t =
        aas_header_s{ident: 0,
                     version: 0,
                     bspchecksum: 0,
                     lumps: [aas_lump_t{fileofs: 0, filelen: 0,}; 14],};
    let mut fp: fileHandle_t = 0;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"writing %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename);
    AAS_SwapAASData();
    memset(&mut header as *mut aas_header_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_header_t>() as libc::c_ulong);
    header.ident =
        (('S' as i32) << 24i32) + (('A' as i32) << 16i32) +
            (('A' as i32) << 8i32) + 'E' as i32;
    header.version = 5i32;
    header.bspchecksum = aasworld.bspchecksum;
    botimport.FS_FOpenFile.expect("non-null function pointer")(filename,
                                                               &mut fp,
                                                               FS_WRITE);
    if 0 == fp {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"error opening %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            filename);
        return qfalse
    }
    botimport.FS_Write.expect("non-null function pointer")(&mut header as
                                                               *mut aas_header_t
                                                               as
                                                               *const libc::c_void,
                                                           ::std::mem::size_of::<aas_header_t>()
                                                               as
                                                               libc::c_ulong
                                                               as libc::c_int,
                                                           fp);
    AAS_WriteAASLump_offset =
        ::std::mem::size_of::<aas_header_t>() as libc::c_ulong as libc::c_int;
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 0i32,
                            aasworld.bboxes as *mut libc::c_void,
                            (aasworld.numbboxes as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_bbox_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 1i32,
                            aasworld.vertexes as *mut libc::c_void,
                            (aasworld.numvertexes as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_vertex_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 2i32,
                            aasworld.planes as *mut libc::c_void,
                            (aasworld.numplanes as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_plane_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 3i32,
                            aasworld.edges as *mut libc::c_void,
                            (aasworld.numedges as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_edge_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 4i32,
                            aasworld.edgeindex as *mut libc::c_void,
                            (aasworld.edgeindexsize as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_edgeindex_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 5i32,
                            aasworld.faces as *mut libc::c_void,
                            (aasworld.numfaces as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_face_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 6i32,
                            aasworld.faceindex as *mut libc::c_void,
                            (aasworld.faceindexsize as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_faceindex_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 7i32,
                            aasworld.areas as *mut libc::c_void,
                            (aasworld.numareas as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_area_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 8i32,
                            aasworld.areasettings as *mut libc::c_void,
                            (aasworld.numareasettings as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_areasettings_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 9i32,
                            aasworld.reachability as *mut libc::c_void,
                            (aasworld.reachabilitysize as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_reachability_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 10i32,
                            aasworld.nodes as *mut libc::c_void,
                            (aasworld.numnodes as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_node_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 11i32,
                            aasworld.portals as *mut libc::c_void,
                            (aasworld.numportals as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_portal_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 12i32,
                            aasworld.portalindex as *mut libc::c_void,
                            (aasworld.portalindexsize as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_portalindex_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    if 0 ==
           AAS_WriteAASLump(fp, &mut header, 13i32,
                            aasworld.clusters as *mut libc::c_void,
                            (aasworld.numclusters as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_cluster_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int) {
        return qfalse
    }
    botimport.FS_Seek.expect("non-null function pointer")(fp,
                                                          0i32 as
                                                              libc::c_long,
                                                          FS_SEEK_SET as
                                                              libc::c_int);
    AAS_DData((&mut header as *mut aas_header_t as
                   *mut libc::c_uchar).offset(8isize),
              (::std::mem::size_of::<aas_header_t>() as
                   libc::c_ulong).wrapping_sub(8i32 as libc::c_ulong) as
                  libc::c_int);
    botimport.FS_Write.expect("non-null function pointer")(&mut header as
                                                               *mut aas_header_t
                                                               as
                                                               *const libc::c_void,
                                                           ::std::mem::size_of::<aas_header_t>()
                                                               as
                                                               libc::c_ulong
                                                               as libc::c_int,
                                                           fp);
    botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_WriteAASLump(mut fp: fileHandle_t,
                                          mut h: *mut aas_header_t,
                                          mut lumpnum: libc::c_int,
                                          mut data: *mut libc::c_void,
                                          mut length: libc::c_int)
 -> libc::c_int {
    let mut lump: *mut aas_lump_t = 0 as *mut aas_lump_t;
    lump = &mut (*h).lumps[lumpnum as usize] as *mut aas_lump_t;
    (*lump).fileofs = AAS_WriteAASLump_offset;
    (*lump).filelen = length;
    if length > 0i32 {
        botimport.FS_Write.expect("non-null function pointer")(data, length,
                                                               fp);
    }
    AAS_WriteAASLump_offset += length;
    return qtrue as libc::c_int;
}
//end of the function AAS_LoadAASFile
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
static mut AAS_WriteAASLump_offset: libc::c_int = 0;