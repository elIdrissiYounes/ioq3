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
    pub type cplane_t = cplane_s;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
        #[no_mangle]
        pub fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                            right: *mut vec_t, up: *mut vec_t);
    }
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
    use super::{libc};
    use super::q_shared_h::{vec3_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas.h"]
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
        pub traveltimes: [libc::c_ushort; 1],
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_reach.c"]
pub mod be_aas_reach_c {
    pub type aas_lreachability_t = aas_lreachability_s;
    //linked reachability
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_lreachability_s {
        pub areanum: libc::c_int,
        pub facenum: libc::c_int,
        pub edgenum: libc::c_int,
        pub start: vec3_t,
        pub end: vec3_t,
        pub traveltype: libc::c_int,
        pub traveltime: libc::c_ushort,
        pub next: *mut aas_lreachability_s,
    }
    use super::{libc};
    use super::q_shared_h::{vec3_t, vec_t, qboolean};
    use super::botlib_h::{botlib_import_t};
    use super::aasfile_h::{aas_face_t, aas_plane_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
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
 * name:		be_aas_reach.c
 *
 * desc:		reachability calculations
 *
 * $Archive: /MissionPack/code/botlib/be_aas_reach.c $
 *
 *****************************************************************************/
        #[no_mangle]
        pub fn Sys_MilliSeconds() -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn tan(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabsf(_: libc::c_float) -> libc::c_float;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::{libc};
    extern "C" {
        //write to the current opened log file
        #[no_mangle]
        pub fn Log_Write(fmt: *mut libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //gets the value of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetValue(var_name: *const libc::c_char) -> libc::c_float;
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::{libc};
    use super::be_aas_h::{aas_trace_t};
    use super::q_shared_h::{vec_t, vec3_t, qboolean};
    use super::be_aas_def_h::{aas_link_t};
    extern "C" {
        //returns the presence type(s) of the area
        #[no_mangle]
        pub fn AAS_AreaPresenceType(areanum: libc::c_int) -> libc::c_int;
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
        #[no_mangle]
        pub fn AAS_UnlinkFromAreas(areas: *mut aas_link_t);
        #[no_mangle]
        pub fn AAS_LinkEntityClientBBox(absmins: *mut vec_t,
                                        absmaxs: *mut vec_t,
                                        entnum: libc::c_int,
                                        presencetype: libc::c_int)
         -> *mut aas_link_t;
        #[no_mangle]
        pub fn AAS_PointInsideFace(facenum: libc::c_int, point: *mut vec_t,
                                   epsilon: libc::c_float) -> qboolean;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_reach.h"]
pub mod be_aas_reach_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_def_h::{aas_link_t};
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
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_variadic.h"]
pub mod be_variadic_h {
    use super::{libc};
    extern "C" {
        //AAS error message
        #[no_mangle]
        pub fn AAS_Error(fmt: *mut libc::c_char, ...);
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    use super::{libc};
    use super::be_aas_h::{aas_clientmove_s};
    use super::q_shared_h::{vec_t};
    use super::be_aas_def_h::{aas_settings_t};
    extern "C" {
        //predict movement until bounding box is hit
        #[no_mangle]
        pub fn AAS_ClientMovementHitBBox(move_0: *mut aas_clientmove_s,
                                         entnum: libc::c_int,
                                         origin: *mut vec_t,
                                         presencetype: libc::c_int,
                                         onground: libc::c_int,
                                         velocity: *mut vec_t,
                                         cmdmove: *mut vec_t,
                                         cmdframes: libc::c_int,
                                         maxframes: libc::c_int,
                                         frametime: libc::c_float,
                                         mins: *mut vec_t, maxs: *mut vec_t,
                                         visualize: libc::c_int)
         -> libc::c_int;
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
        //rocket jump Z velocity when rocket-jumping at origin
        #[no_mangle]
        pub fn AAS_RocketJumpZVelocity(origin: *mut vec_t) -> libc::c_float;
        //bfg jump Z velocity when bfg-jumping at origin
        #[no_mangle]
        pub fn AAS_BFGJumpZVelocity(origin: *mut vec_t) -> libc::c_float;
        //calculates the horizontal velocity needed for a jump and returns true this velocity could be calculated
        #[no_mangle]
        pub fn AAS_HorizontalVelocityForJump(zvel: libc::c_float,
                                             start: *mut vec_t,
                                             end: *mut vec_t,
                                             velocity: *mut libc::c_float)
         -> libc::c_int;
        //
        #[no_mangle]
        pub fn AAS_DropToFloor(origin: *mut vec_t, mins: *mut vec_t,
                               maxs: *mut vec_t) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::botlib_h::{bsp_trace_t};
    extern "C" {
        //get a vector for the BSP epair key
        #[no_mangle]
        pub fn AAS_VectorForBSPEpairKey(ent: libc::c_int,
                                        key: *mut libc::c_char, v: *mut vec_t)
         -> libc::c_int;
        //return the value of the BSP epair key
        #[no_mangle]
        pub fn AAS_ValueForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_char,
                                       size: libc::c_int) -> libc::c_int;
        //handle to the next bsp entity
        #[no_mangle]
        pub fn AAS_NextBSPEntity(ent: libc::c_int) -> libc::c_int;
        //gets the mins, maxs and origin of a BSP model
        #[no_mangle]
        pub fn AAS_BSPModelMinsMaxsOrigin(modelnum: libc::c_int,
                                          angles: *mut vec_t,
                                          mins: *mut vec_t, maxs: *mut vec_t,
                                          origin: *mut vec_t);
        //get a float for the BSP epair key
        #[no_mangle]
        pub fn AAS_FloatForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_float)
         -> libc::c_int;
        //
        //AASINTERN
        //trace through the world
        #[no_mangle]
        pub fn AAS_Trace(start: *mut vec_t, mins: *mut vec_t,
                         maxs: *mut vec_t, end: *mut vec_t,
                         passent: libc::c_int, contentmask: libc::c_int)
         -> bsp_trace_t;
        //returns the contents at the given point
        #[no_mangle]
        pub fn AAS_PointContents(point: *mut vec_t) -> libc::c_int;
        //get an integer for the BSP epair key
        #[no_mangle]
        pub fn AAS_IntForBSPEpairKey(ent: libc::c_int, key: *mut libc::c_char,
                                     value: *mut libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_debug.h"]
pub mod be_aas_debug_h {
    use super::q_shared_h::{vec_t};
    use super::{libc};
    extern "C" {
        //show a permenent line
        #[no_mangle]
        pub fn AAS_PermanentLine(start: *mut vec_t, end: *mut vec_t,
                                 color: libc::c_int);
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, VectorNormalize,
                       AngleVectors};
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
use self::be_aas_reach_c::{aas_lreachability_t, aas_lreachability_s,
                           botimport, Sys_MilliSeconds};
use self::mathcalls_h::{tan, sqrt, fabs, fabsf};
use self::string_h::{memset, strcmp};
use self::stdlib_h::{atoi, abs};
use self::l_log_h::{Log_Write};
use self::l_memory_h::{GetClearedMemory, FreeMemory};
use self::l_libvar_h::{LibVarGetValue, LibVarValue};
use self::be_aas_sample_h::{AAS_AreaPresenceType, AAS_TraceClientBBox,
                            AAS_TraceAreas, AAS_PointAreaNum,
                            AAS_UnlinkFromAreas, AAS_LinkEntityClientBBox,
                            AAS_PointInsideFace};
use self::be_aas_main_h::{aasworld};
use self::be_variadic_h::{AAS_Error};
use self::be_aas_move_h::{AAS_ClientMovementHitBBox, aassettings,
                          AAS_PredictClientMovement, AAS_RocketJumpZVelocity,
                          AAS_BFGJumpZVelocity, AAS_HorizontalVelocityForJump,
                          AAS_DropToFloor};
use self::be_aas_bsp_h::{AAS_VectorForBSPEpairKey, AAS_ValueForBSPEpairKey,
                         AAS_NextBSPEntity, AAS_BSPModelMinsMaxsOrigin,
                         AAS_FloatForBSPEpairKey, AAS_Trace,
                         AAS_PointContents, AAS_IntForBSPEpairKey};
use self::be_aas_debug_h::{AAS_PermanentLine};
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
//returns true if the are has reachabilities to other areas
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaReachability(mut areanum: libc::c_int)
 -> libc::c_int {
    if areanum < 0i32 || areanum >= aasworld.numareas {
        AAS_Error(b"AAS_AreaReachability: areanum %d out of range\n\x00" as
                      *const u8 as *const libc::c_char as *mut libc::c_char,
                  areanum);
        return 0i32
    }
    return (*aasworld.areasettings.offset(areanum as
                                              isize)).numreachableareas;
}
//returns the best reachable area and goal origin for a bounding box at the given origin
#[no_mangle]
pub unsafe extern "C" fn AAS_BestReachableArea(mut origin: *mut vec_t,
                                               mut mins: *mut vec_t,
                                               mut maxs: *mut vec_t,
                                               mut goalorigin: *mut vec_t)
 -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut areas: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    //vec3_t bbmins, bbmaxs;
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
    if 0 == aasworld.loaded {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"AAS_BestReachableArea: aas not loaded\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0i32
    }
    start[0usize] = *origin.offset(0isize);
    start[1usize] = *origin.offset(1isize);
    start[2usize] = *origin.offset(2isize);
    areanum = AAS_PointAreaNum(start.as_mut_ptr());
    i = 0i32;
    while i < 5i32 && 0 == areanum {
        j = 0i32;
        while j < 5i32 && 0 == areanum {
            k = -1i32;
            while k <= 1i32 && 0 == areanum {
                l = -1i32;
                while l <= 1i32 && 0 == areanum {
                    start[0usize] = *origin.offset(0isize);
                    start[1usize] = *origin.offset(1isize);
                    start[2usize] = *origin.offset(2isize);
                    start[0usize] +=
                        j as libc::c_float * 4i32 as libc::c_float *
                            k as libc::c_float;
                    start[1usize] +=
                        j as libc::c_float * 4i32 as libc::c_float *
                            l as libc::c_float;
                    start[2usize] +=
                        i as libc::c_float * 4i32 as libc::c_float;
                    areanum = AAS_PointAreaNum(start.as_mut_ptr());
                    l += 1
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    if 0 != areanum {
        end[0usize] = start[0usize];
        end[1usize] = start[1usize];
        end[2usize] = start[2usize];
        start[2usize] = (start[2usize] as libc::c_double + 0.25f64) as vec_t;
        end[2usize] -= 50i32 as libc::c_float;
        trace =
            AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 4i32,
                                -1i32);
        if 0 == trace.startsolid as u64 {
            areanum = AAS_PointAreaNum(trace.endpos.as_mut_ptr());
            *goalorigin.offset(0isize) = trace.endpos[0usize];
            *goalorigin.offset(1isize) = trace.endpos[1usize];
            *goalorigin.offset(2isize) = trace.endpos[2usize];
            if 0 != areanum { return areanum }
        } else {
            *goalorigin.offset(0isize) = start[0usize];
            *goalorigin.offset(1isize) = start[1usize];
            *goalorigin.offset(2isize) = start[2usize];
            return areanum
        }
    }
    *goalorigin.offset(0isize) = *origin.offset(0isize);
    *goalorigin.offset(1isize) = *origin.offset(1isize);
    *goalorigin.offset(2isize) = *origin.offset(2isize);
    absmins[0usize] = *origin.offset(0isize) + *mins.offset(0isize);
    absmins[1usize] = *origin.offset(1isize) + *mins.offset(1isize);
    absmins[2usize] = *origin.offset(2isize) + *mins.offset(2isize);
    absmaxs[0usize] = *origin.offset(0isize) + *maxs.offset(0isize);
    absmaxs[1usize] = *origin.offset(1isize) + *maxs.offset(1isize);
    absmaxs[2usize] = *origin.offset(2isize) + *maxs.offset(2isize);
    areas =
        AAS_LinkEntityClientBBox(absmins.as_mut_ptr(), absmaxs.as_mut_ptr(),
                                 -1i32, 4i32);
    areanum = AAS_BestReachableLinkArea(areas);
    AAS_UnlinkFromAreas(areas);
    return areanum;
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_BestReachableLinkArea(mut areas: *mut aas_link_t)
 -> libc::c_int {
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    link = areas;
    while !link.is_null() {
        if 0 != AAS_AreaGrounded((*link).areanum) ||
               0 != AAS_AreaSwim((*link).areanum) {
            return (*link).areanum
        }
        link = (*link).next_area
    }
    link = areas;
    while !link.is_null() {
        if 0 != (*link).areanum { return (*link).areanum }
        if 0 != AAS_AreaReachability((*link).areanum) {
            return (*link).areanum
        }
        link = (*link).next_area
    }
    return 0i32;
}
//returns true if a player can swim in this area
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaSwim(mut areanum: libc::c_int)
 -> libc::c_int {
    if 0 != (*aasworld.areasettings.offset(areanum as isize)).areaflags & 4i32
       {
        return qtrue as libc::c_int
    } else { return qfalse as libc::c_int };
}
//returns true if the area has one or more ground faces
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaGrounded(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).areaflags & 1i32;
}
//returns the best jumppad area from which the bbox at origin is reachable
#[no_mangle]
pub unsafe extern "C" fn AAS_BestReachableFromJumpPadArea(mut origin:
                                                              *mut vec_t,
                                                          mut mins:
                                                              *mut vec_t,
                                                          mut maxs:
                                                              *mut vec_t)
 -> libc::c_int {
    let mut ent: libc::c_int = 0;
    let mut bot_visualizejumppads: libc::c_int = 0;
    let mut bestareanum: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut bestareavolume: libc::c_float = 0.;
    let mut areastart: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut bboxmins: vec3_t = [0.; 3];
    let mut bboxmaxs: vec3_t = [0.; 3];
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
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
    let mut areas: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut classname: [libc::c_char; 128] = [0; 128];
    bot_visualizejumppads =
        LibVarValue(b"bot_visualizejumppads\x00" as *const u8 as
                        *const libc::c_char,
                    b"0\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    bboxmins[0usize] = *origin.offset(0isize) + *mins.offset(0isize);
    bboxmins[1usize] = *origin.offset(1isize) + *mins.offset(1isize);
    bboxmins[2usize] = *origin.offset(2isize) + *mins.offset(2isize);
    bboxmaxs[0usize] = *origin.offset(0isize) + *maxs.offset(0isize);
    bboxmaxs[1usize] = *origin.offset(1isize) + *maxs.offset(1isize);
    bboxmaxs[2usize] = *origin.offset(2isize) + *maxs.offset(2isize);
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if !(0 !=
                     strcmp(classname.as_mut_ptr(),
                            b"trigger_push\x00" as *const u8 as
                                *const libc::c_char)) {
                //
                if !(0 ==
                         AAS_GetJumpPadInfo(ent, areastart.as_mut_ptr(),
                                            absmins.as_mut_ptr(),
                                            absmaxs.as_mut_ptr(),
                                            velocity.as_mut_ptr())) {
                    areas =
                        AAS_LinkEntityClientBBox(absmins.as_mut_ptr(),
                                                 absmaxs.as_mut_ptr(), -1i32,
                                                 4i32);
                    link = areas;
                    while !link.is_null() {
                        if 0 != AAS_AreaJumpPad((*link).areanum) { break ; }
                        link = (*link).next_area
                    }
                    //end for
                    if link.is_null() {
                        botimport.Print.expect("non-null function pointer")(1i32,
                                                                            b"trigger_push not in any jump pad area\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char);
                        AAS_UnlinkFromAreas(areas);
                    } else {
                        cmdmove[0usize] = 0i32 as vec_t;
                        cmdmove[1usize] = 0i32 as vec_t;
                        cmdmove[2usize] = 0i32 as vec_t;
                        memset(&mut move_0 as *mut aas_clientmove_t as
                                   *mut libc::c_void, 0i32,
                               ::std::mem::size_of::<aas_clientmove_t>() as
                                   libc::c_ulong);
                        AAS_ClientMovementHitBBox(&mut move_0, -1i32,
                                                  areastart.as_mut_ptr(),
                                                  2i32, qfalse as libc::c_int,
                                                  velocity.as_mut_ptr(),
                                                  cmdmove.as_mut_ptr(), 0i32,
                                                  30i32, 0.1f32,
                                                  bboxmins.as_mut_ptr(),
                                                  bboxmaxs.as_mut_ptr(),
                                                  bot_visualizejumppads);
                        if move_0.frames < 30i32 {
                            bestareanum = 0i32;
                            bestareavolume = 0i32 as libc::c_float;
                            link = areas;
                            while !link.is_null() {
                                if !(0 == AAS_AreaJumpPad((*link).areanum)) {
                                    volume = AAS_AreaVolume((*link).areanum);
                                    if volume >= bestareavolume {
                                        bestareanum = (*link).areanum;
                                        bestareavolume = volume
                                    }
                                }
                                link = (*link).next_area
                            }
                            AAS_UnlinkFromAreas(areas);
                            return bestareanum
                        }
                        AAS_UnlinkFromAreas(areas);
                    }
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    }
    return 0i32;
}
//end of the function AAS_FaceArea
//===========================================================================
// returns the volume of an area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaVolume(mut areanum: libc::c_int)
 -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut d: vec_t = 0.;
    let mut a: vec_t = 0.;
    let mut volume: vec_t = 0.;
    let mut corner: vec3_t = [0.; 3];
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    facenum = *aasworld.faceindex.offset((*area).firstface as isize);
    face =
        &mut *aasworld.faces.offset(abs(facenum) as isize) as *mut aas_face_t;
    edgenum = *aasworld.edgeindex.offset((*face).firstedge as isize);
    edge =
        &mut *aasworld.edges.offset(abs(edgenum) as isize) as *mut aas_edge_t;
    corner[0usize] =
        (*aasworld.vertexes.offset((*edge).v[0usize] as isize))[0usize];
    corner[1usize] =
        (*aasworld.vertexes.offset((*edge).v[0usize] as isize))[1usize];
    corner[2usize] =
        (*aasworld.vertexes.offset((*edge).v[0usize] as isize))[2usize];
    volume = 0i32 as vec_t;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            abs(*aasworld.faceindex.offset(((*area).firstface + i) as isize));
        face =
            &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
        side = ((*face).backarea != areanum) as libc::c_int;
        plane =
            &mut *aasworld.planes.offset(((*face).planenum ^ side) as isize)
                as *mut aas_plane_t;
        d =
            -(corner[0usize] * (*plane).normal[0usize] +
                  corner[1usize] * (*plane).normal[1usize] +
                  corner[2usize] * (*plane).normal[2usize] - (*plane).dist);
        a = AAS_FaceArea(face);
        volume += d * a;
        i += 1
    }
    volume /= 3i32 as libc::c_float;
    return volume;
}
//===========================================================================
// returns the surface area of the given face
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FaceArea(mut face: *mut aas_face_t)
 -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut total: libc::c_float = 0.;
    let mut v: *mut vec_t = 0 as *mut vec_t;
    let mut d1: vec3_t = [0.; 3];
    let mut d2: vec3_t = [0.; 3];
    let mut cross: vec3_t = [0.; 3];
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    edgenum = *aasworld.edgeindex.offset((*face).firstedge as isize);
    side = (edgenum < 0i32) as libc::c_int;
    edge =
        &mut *aasworld.edges.offset(abs(edgenum) as isize) as *mut aas_edge_t;
    v =
        (*aasworld.vertexes.offset((*edge).v[side as usize] as
                                       isize)).as_mut_ptr();
    total = 0i32 as libc::c_float;
    i = 1i32;
    while i < (*face).numedges - 1i32 {
        edgenum =
            *aasworld.edgeindex.offset(((*face).firstedge + i) as isize);
        side = (edgenum < 0i32) as libc::c_int;
        edge =
            &mut *aasworld.edges.offset(abs(edgenum) as isize) as
                *mut aas_edge_t;
        d1[0usize] =
            (*aasworld.vertexes.offset((*edge).v[side as usize] as
                                           isize))[0usize] -
                *v.offset(0isize);
        d1[1usize] =
            (*aasworld.vertexes.offset((*edge).v[side as usize] as
                                           isize))[1usize] -
                *v.offset(1isize);
        d1[2usize] =
            (*aasworld.vertexes.offset((*edge).v[side as usize] as
                                           isize))[2usize] -
                *v.offset(2isize);
        d2[0usize] =
            (*aasworld.vertexes.offset((*edge).v[(0 == side) as libc::c_int as
                                                     usize] as isize))[0usize]
                - *v.offset(0isize);
        d2[1usize] =
            (*aasworld.vertexes.offset((*edge).v[(0 == side) as libc::c_int as
                                                     usize] as isize))[1usize]
                - *v.offset(1isize);
        d2[2usize] =
            (*aasworld.vertexes.offset((*edge).v[(0 == side) as libc::c_int as
                                                     usize] as isize))[2usize]
                - *v.offset(2isize);
        CrossProduct(d1.as_mut_ptr() as *const vec_t,
                     d2.as_mut_ptr() as *const vec_t, cross.as_mut_ptr());
        total =
            (total as libc::c_double +
                 0.5f64 *
                     VectorLength(cross.as_mut_ptr() as *const vec_t) as
                         libc::c_double) as libc::c_float;
        i += 1
    }
    return total;
}
//returns true if the area is a jump pad
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaJumpPad(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).contents &
               128i32;
}
//end of the function AAS_BestReachableLinkArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_GetJumpPadInfo(mut ent: libc::c_int,
                                            mut areastart: *mut vec_t,
                                            mut absmins: *mut vec_t,
                                            mut absmaxs: *mut vec_t,
                                            mut velocity: *mut vec_t)
 -> libc::c_int {
    let mut modelnum: libc::c_int = 0;
    let mut ent2: libc::c_int = 0;
    let mut speed: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut forward: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut teststart: vec3_t = [0.; 3];
    let mut ent2origin: vec3_t = [0.; 3];
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut target: [libc::c_char; 128] = [0; 128];
    let mut targetname: [libc::c_char; 128] = [0; 128];
    AAS_FloatForBSPEpairKey(ent,
                            b"speed\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, &mut speed);
    if 0. == speed { speed = 1000i32 as libc::c_float }
    angles[2usize] = 0i32 as vec_t;
    angles[1usize] = angles[2usize];
    angles[0usize] = angles[1usize];
    AAS_ValueForBSPEpairKey(ent,
                            b"model\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, model.as_mut_ptr(),
                            128i32);
    if 0 != model[0usize] {
        modelnum = atoi(model.as_mut_ptr().offset(1isize))
    } else { modelnum = 0i32 }
    AAS_BSPModelMinsMaxsOrigin(modelnum, angles.as_mut_ptr(), absmins,
                               absmaxs, origin.as_mut_ptr());
    *absmins.offset(0isize) = origin[0usize] + *absmins.offset(0isize);
    *absmins.offset(1isize) = origin[1usize] + *absmins.offset(1isize);
    *absmins.offset(2isize) = origin[2usize] + *absmins.offset(2isize);
    *absmaxs.offset(0isize) = origin[0usize] + *absmaxs.offset(0isize);
    *absmaxs.offset(1isize) = origin[1usize] + *absmaxs.offset(1isize);
    *absmaxs.offset(2isize) = origin[2usize] + *absmaxs.offset(2isize);
    origin[0usize] = *absmins.offset(0isize) + *absmaxs.offset(0isize);
    origin[1usize] = *absmins.offset(1isize) + *absmaxs.offset(1isize);
    origin[2usize] = *absmins.offset(2isize) + *absmaxs.offset(2isize);
    origin[0usize] = (origin[0usize] as libc::c_double * 0.5f64) as vec_t;
    origin[1usize] = (origin[1usize] as libc::c_double * 0.5f64) as vec_t;
    origin[2usize] = (origin[2usize] as libc::c_double * 0.5f64) as vec_t;
    teststart[0usize] = origin[0usize];
    teststart[1usize] = origin[1usize];
    teststart[2usize] = origin[2usize];
    teststart[2usize] += 64i32 as libc::c_float;
    trace =
        AAS_TraceClientBBox(teststart.as_mut_ptr(), origin.as_mut_ptr(), 4i32,
                            -1i32);
    if 0 != trace.startsolid as u64 {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"trigger_push start solid\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        *areastart.offset(0isize) = origin[0usize];
        *areastart.offset(1isize) = origin[1usize];
        *areastart.offset(2isize) = origin[2usize]
    } else {
        *areastart.offset(0isize) = trace.endpos[0usize];
        *areastart.offset(1isize) = trace.endpos[1usize];
        *areastart.offset(2isize) = trace.endpos[2usize]
    }
    let ref mut fresh0 = *areastart.offset(2isize);
    *fresh0 = (*fresh0 as libc::c_double + 0.125f64) as vec_t;
    AAS_ValueForBSPEpairKey(ent,
                            b"target\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char, target.as_mut_ptr(),
                            128i32);
    ent2 = AAS_NextBSPEntity(0i32);
    while 0 != ent2 {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent2,
                                         b"targetname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         targetname.as_mut_ptr(), 128i32)) {
            if 0 == strcmp(targetname.as_mut_ptr(), target.as_mut_ptr()) {
                break ;
            }
        }
        ent2 = AAS_NextBSPEntity(ent2)
    }
    if 0 == ent2 {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"trigger_push without target entity %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            target.as_mut_ptr());
        return qfalse as libc::c_int
    }
    AAS_VectorForBSPEpairKey(ent2,
                             b"origin\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                             ent2origin.as_mut_ptr());
    height = ent2origin[2usize] - origin[2usize];
    gravity = aassettings.phys_gravity;
    time =
        sqrt(height as libc::c_double / (0.5f64 * gravity as libc::c_double))
            as libc::c_float;
    if 0. == time {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"trigger_push without time\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return qfalse as libc::c_int
    }
    *velocity.offset(0isize) = ent2origin[0usize] - origin[0usize];
    *velocity.offset(1isize) = ent2origin[1usize] - origin[1usize];
    *velocity.offset(2isize) = ent2origin[2usize] - origin[2usize];
    dist = VectorNormalize(velocity);
    forward = dist / time;
    forward *= 1.1f32;
    *velocity.offset(0isize) = *velocity.offset(0isize) * forward;
    *velocity.offset(1isize) = *velocity.offset(1isize) * forward;
    *velocity.offset(2isize) = *velocity.offset(2isize) * forward;
    *velocity.offset(2isize) = time * gravity;
    return qtrue as libc::c_int;
}
//returns the total area of the ground faces of the given area
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaGroundFaceArea(mut areanum: libc::c_int)
 -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_float = 0.;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    total = 0i32 as libc::c_float;
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    i = 0i32;
    while i < (*area).numfaces {
        face =
            &mut *aasworld.faces.offset(abs(*aasworld.faceindex.offset(((*area).firstface
                                                                            +
                                                                            i)
                                                                           as
                                                                           isize))
                                            as isize) as *mut aas_face_t;
        if !(0 == (*face).faceflags & 4i32) { total += AAS_FaceArea(face) }
        i += 1
    }
    return total;
}
//returns true if the area is crouch only
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaCrouch(mut areanum: libc::c_int)
 -> libc::c_int {
    if 0 ==
           (*aasworld.areasettings.offset(areanum as isize)).presencetype &
               2i32 {
        return qtrue as libc::c_int
    } else { return qfalse as libc::c_int };
}
//returns true if the area is filled with a liquid
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaLiquid(mut areanum: libc::c_int)
 -> libc::c_int {
    if 0 != (*aasworld.areasettings.offset(areanum as isize)).areaflags & 4i32
       {
        return qtrue as libc::c_int
    } else { return qfalse as libc::c_int };
}
//returns true if the area contains lava
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaLava(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).contents & 2i32;
}
//returns true if the area contains slime
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaSlime(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).contents & 4i32;
}
//returns true if the area has one or more ladder faces
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaLadder(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).areaflags & 2i32;
}
//returns true if the area is donotenter
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaDoNotEnter(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).contents &
               256i32;
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
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
//initialize calculating the reachabilities
#[no_mangle]
pub unsafe extern "C" fn AAS_InitReachability() {
    if 0 == aasworld.loaded { return }
    if 0 != aasworld.reachabilitysize {
        if 0 ==
               LibVarGetValue(b"forcereachability\x00" as *const u8 as
                                  *const libc::c_char) as libc::c_int {
            aasworld.numreachabilityareas = aasworld.numareas + 2i32;
            return
        }
    }
    calcgrapplereach =
        LibVarGetValue(b"grapplereach\x00" as *const u8 as
                           *const libc::c_char) as libc::c_int;
    aasworld.savefile = qtrue as libc::c_int;
    aasworld.numreachabilityareas = 1i32;
    AAS_SetupReachabilityHeap();
    areareachability =
        GetClearedMemory((aasworld.numareas as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut aas_lreachability_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut *mut aas_lreachability_t;
    AAS_SetWeaponJumpAreaFlags();
}
//end of the function AAS_Reachability_Grapple
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_SetWeaponJumpAreaFlags() {
    let mut ent: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mins: vec3_t =
        [-15i32 as vec_t, -15i32 as vec_t, -15i32 as vec_t];
    let mut maxs: vec3_t = [15i32 as vec_t, 15i32 as vec_t, 15i32 as vec_t];
    let mut origin: vec3_t = [0.; 3];
    let mut areanum: libc::c_int = 0;
    let mut weaponjumpareas: libc::c_int = 0;
    let mut spawnflags: libc::c_int = 0;
    let mut classname: [libc::c_char; 128] = [0; 128];
    weaponjumpareas = 0i32;
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if 0 ==
                   strcmp(classname.as_mut_ptr(),
                          b"item_armor_body\x00" as *const u8 as
                              *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"item_armor_combat\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"item_health_mega\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"weapon_grenadelauncher\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"weapon_rocketlauncher\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"weapon_lightning\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"weapon_plasmagun\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"weapon_railgun\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"weapon_bfg\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"item_quad\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"item_regen\x00" as *const u8 as
                                  *const libc::c_char) ||
                   0 ==
                       strcmp(classname.as_mut_ptr(),
                              b"item_invulnerability\x00" as *const u8 as
                                  *const libc::c_char) {
                if 0 !=
                       AAS_VectorForBSPEpairKey(ent,
                                                b"origin\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                origin.as_mut_ptr()) {
                    spawnflags = 0i32;
                    AAS_IntForBSPEpairKey(ent,
                                          b"spawnflags\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char,
                                          &mut spawnflags);
                    if 0 == spawnflags & 1i32 {
                        if 0 ==
                               AAS_DropToFloor(origin.as_mut_ptr(),
                                               mins.as_mut_ptr(),
                                               maxs.as_mut_ptr()) {
                            botimport.Print.expect("non-null function pointer")(1i32,
                                                                                b"%s in solid at (%1.1f %1.1f %1.1f)\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                                    as
                                                                                    *mut libc::c_char,
                                                                                classname.as_mut_ptr(),
                                                                                origin[0usize]
                                                                                    as
                                                                                    libc::c_double,
                                                                                origin[1usize]
                                                                                    as
                                                                                    libc::c_double,
                                                                                origin[2usize]
                                                                                    as
                                                                                    libc::c_double);
                        }
                    }
                    areanum =
                        AAS_BestReachableArea(origin.as_mut_ptr(),
                                              mins.as_mut_ptr(),
                                              maxs.as_mut_ptr(),
                                              origin.as_mut_ptr());
                    (*aasworld.areasettings.offset(areanum as
                                                       isize)).areaflags |=
                        8192i32;
                    weaponjumpareas += 1
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    }
    i = 1i32;
    while i < aasworld.numareas {
        if 0 != (*aasworld.areasettings.offset(i as isize)).contents & 128i32
           {
            (*aasworld.areasettings.offset(i as isize)).areaflags |= 8192i32;
            weaponjumpareas += 1
        }
        i += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"%d weapon jump areas\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        weaponjumpareas);
}
//reachability links for every area
#[no_mangle]
pub static mut areareachability: *mut *mut aas_lreachability_t =
    0 as *const *mut aas_lreachability_t as *mut *mut aas_lreachability_t;
//end of the function AAS_BestReachableArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_SetupReachabilityHeap() {
    let mut i: libc::c_int = 0;
    reachabilityheap =
        GetClearedMemory((65536i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_lreachability_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_lreachability_t;
    i = 0i32;
    while i < 65536i32 - 1i32 {
        let ref mut fresh1 = (*reachabilityheap.offset(i as isize)).next;
        *fresh1 =
            &mut *reachabilityheap.offset((i + 1i32) as isize) as
                *mut aas_lreachability_t;
        i += 1
    }
    let ref mut fresh2 =
        (*reachabilityheap.offset((65536i32 - 1i32) as isize)).next;
    *fresh2 = 0 as *mut aas_lreachability_s;
    nextreachability = reachabilityheap;
    numlreachabilities = 0i32;
}
#[no_mangle]
pub static mut numlreachabilities: libc::c_int = 0;
//temporary reachabilities
//heap with reachabilities
#[no_mangle]
pub static mut reachabilityheap: *mut aas_lreachability_t =
    0 as *const aas_lreachability_t as *mut aas_lreachability_t;
//next free reachability from the heap
#[no_mangle]
pub static mut nextreachability: *mut aas_lreachability_t =
    0 as *const aas_lreachability_t as *mut aas_lreachability_t;
//if true grapple reachabilities are skipped
#[no_mangle]
pub static mut calcgrapplereach: libc::c_int = 0;
//continue calculating the reachabilities
#[no_mangle]
pub unsafe extern "C" fn AAS_ContinueInitReachability(mut time: libc::c_float)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut todo: libc::c_int = 0;
    let mut start_time: libc::c_int = 0;
    static mut framereachability: libc::c_float = 0.;
    static mut reachability_delay: libc::c_float = 0.;
    static mut lastpercentage: libc::c_int = 0;
    if 0 == aasworld.loaded { return qfalse as libc::c_int }
    if aasworld.numreachabilityareas >= aasworld.numareas + 2i32 {
        return qfalse as libc::c_int
    }
    if aasworld.numreachabilityareas == 1i32 {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"calculating reachability...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        lastpercentage = 0i32;
        framereachability = 2000i32 as libc::c_float;
        reachability_delay = 1000i32 as libc::c_float
    }
    todo = aasworld.numreachabilityareas + framereachability as libc::c_int;
    start_time = Sys_MilliSeconds();
    i = aasworld.numreachabilityareas;
    while i < aasworld.numareas && i < todo {
        aasworld.numreachabilityareas += 1;
        //only create jumppad reachabilities from jumppad areas
        if !(0 !=
                 (*aasworld.areasettings.offset(i as isize)).contents &
                     128i32) {
            let mut current_block_14: u64;
            j = 1i32;
            while j < aasworld.numareas {
                if !(i == j) {
                    //never create reachabilities from teleporter or jumppad areas to regular areas
                    if 0 !=
                           (*aasworld.areasettings.offset(i as
                                                              isize)).contents
                               & (64i32 | 128i32) {
                        if 0 ==
                               (*aasworld.areasettings.offset(j as
                                                                  isize)).contents
                                   & (64i32 | 128i32) {
                            current_block_14 = 4956146061682418353;
                        } else { current_block_14 = 1109700713171191020; }
                    } else { current_block_14 = 1109700713171191020; }
                    match current_block_14 {
                        4956146061682418353 => { }
                        _ => {
                            //end if
                            //end if
                            //if there already is a reachability link from area i to j
                            if !(0 != AAS_ReachabilityExists(i, j) as u64) {
                                //check for a swim reachability
                                if !(0 != AAS_Reachability_Swim(i, j)) {
                                    //check for a simple walk on equal floor height reachability
                                    if !(0 !=
                                             AAS_Reachability_EqualFloorHeight(i,
                                                                               j))
                                       {
                                        //check for step, barrier, waterjump and walk off ledge reachabilities
                                        if !(0 !=
                                                 AAS_Reachability_Step_Barrier_WaterJump_WalkOffLedge(i,
                                                                                                      j))
                                           {
                                            //check for ladder reachabilities
                                            if !(0 !=
                                                     AAS_Reachability_Ladder(i,
                                                                             j))
                                               {
                                                //check for a jump reachability
                                                0 !=
                                                    AAS_Reachability_Jump(i,
                                                                          j);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                j += 1
            }
            //end for
            //never create these reachabilities from teleporter or jumppad areas
            if !(0 !=
                     (*aasworld.areasettings.offset(i as isize)).contents &
                         (64i32 | 128i32)) {
                j = 1i32;
                while j < aasworld.numareas {
                    if !(i == j) {
                        //
                        if !(0 != AAS_ReachabilityExists(i, j) as u64) {
                            if 0 != calcgrapplereach {
                                AAS_Reachability_Grapple(i, j);
                            }
                            AAS_Reachability_WeaponJump(i, j);
                        }
                    }
                    j += 1
                }
                //end for
                //if the calculation took more time than the max reachability delay
                if Sys_MilliSeconds() - start_time >
                       reachability_delay as libc::c_int {
                    break ;
                }
                //
                if aasworld.numreachabilityareas * 1000i32 / aasworld.numareas
                       > lastpercentage {
                    break ;
                }
            }
        }
        i += 1
    }
    if aasworld.numreachabilityareas == aasworld.numareas {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"\r%6.1f%%\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            100.0f64 as
                                                                libc::c_float
                                                                as
                                                                libc::c_double);
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"\nplease wait while storing reachability...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        aasworld.numreachabilityareas += 1
    } else if aasworld.numreachabilityareas == aasworld.numareas + 1i32 {
        i = 1i32;
        while i < aasworld.numareas {
            //only create jumppad reachabilities from jumppad areas
            if !(0 !=
                     (*aasworld.areasettings.offset(i as isize)).contents &
                         128i32) {
                AAS_Reachability_WalkOffLedge(i);
            }
            i += 1
        }
        AAS_Reachability_JumpPad();
        AAS_Reachability_Teleport();
        AAS_Reachability_Elevator();
        AAS_Reachability_FuncBobbing();
        AAS_StoreReachability();
        AAS_ShutDownReachabilityHeap();
        FreeMemory(areareachability as *mut libc::c_void);
        aasworld.numreachabilityareas += 1;
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"calculating clusters...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
    } else {
        lastpercentage =
            aasworld.numreachabilityareas * 1000i32 / aasworld.numareas;
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"\r%6.1f%%\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            (lastpercentage as
                                                                 libc::c_float
                                                                 /
                                                                 10i32 as
                                                                     libc::c_float)
                                                                as
                                                                libc::c_double);
    }
    return qtrue as libc::c_int;
}
//end of the function AAS_InitReachabilityHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ShutDownReachabilityHeap() {
    FreeMemory(reachabilityheap as *mut libc::c_void);
    numlreachabilities = 0i32;
}
//end if
//end for
//end for
//end for
//end for
//end of the function AAS_Reachability_WalkOffLedge
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_StoreReachability() {
    let mut i: libc::c_int = 0;
    let mut areasettings: *mut aas_areasettings_t =
        0 as *mut aas_areasettings_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut reach: *mut aas_reachability_t = 0 as *mut aas_reachability_t;
    if !aasworld.reachability.is_null() {
        FreeMemory(aasworld.reachability as *mut libc::c_void);
    }
    aasworld.reachability =
        GetClearedMemory(((numlreachabilities + 10i32) as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<aas_reachability_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut aas_reachability_t;
    aasworld.reachabilitysize = 1i32;
    i = 0i32;
    while i < aasworld.numareas {
        areasettings =
            &mut *aasworld.areasettings.offset(i as isize) as
                *mut aas_areasettings_t;
        (*areasettings).firstreachablearea = aasworld.reachabilitysize;
        (*areasettings).numreachableareas = 0i32;
        lreach = *areareachability.offset(i as isize);
        while !lreach.is_null() {
            reach =
                &mut *aasworld.reachability.offset(((*areasettings).firstreachablearea
                                                        +
                                                        (*areasettings).numreachableareas)
                                                       as isize) as
                    *mut aas_reachability_t;
            (*reach).areanum = (*lreach).areanum;
            (*reach).facenum = (*lreach).facenum;
            (*reach).edgenum = (*lreach).edgenum;
            (*reach).start[0usize] = (*lreach).start[0usize];
            (*reach).start[1usize] = (*lreach).start[1usize];
            (*reach).start[2usize] = (*lreach).start[2usize];
            (*reach).end[0usize] = (*lreach).end[0usize];
            (*reach).end[1usize] = (*lreach).end[1usize];
            (*reach).end[2usize] = (*lreach).end[2usize];
            (*reach).traveltype = (*lreach).traveltype;
            (*reach).traveltime = (*lreach).traveltime;
            (*areasettings).numreachableareas += 1;
            lreach = (*lreach).next
        }
        aasworld.reachabilitysize += (*areasettings).numreachableareas;
        i += 1
    };
}
//end of the function AAS_FindFaceReachabilities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_FuncBobbing() {
    let mut ent: libc::c_int = 0;
    let mut spawnflags: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut axis: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut origin: vec3_t = [0.; 3];
    let mut move_end: vec3_t = [0.; 3];
    let mut move_start: vec3_t = [0.; 3];
    let mut move_start_top: vec3_t = [0.; 3];
    let mut move_end_top: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut start_edgeverts: [vec3_t; 4] = [[0.; 3]; 4];
    let mut end_edgeverts: [vec3_t; 4] = [[0.; 3]; 4];
    let mut mid: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut points: [vec3_t; 10] = [[0.; 3]; 10];
    let mut height: libc::c_float = 0.;
    let mut start_plane: aas_plane_t =
        aas_plane_s{normal: [0.; 3], dist: 0., type_0: 0,};
    let mut end_plane: aas_plane_t =
        aas_plane_s{normal: [0.; 3], dist: 0., type_0: 0,};
    let mut startreach: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    let mut endreach: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    let mut nextstartreach: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    let mut nextendreach: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut firststartreach: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    let mut firstendreach: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if !(0 !=
                     strcmp(classname.as_mut_ptr(),
                            b"func_bobbing\x00" as *const u8 as
                                *const libc::c_char)) {
                AAS_FloatForBSPEpairKey(ent,
                                        b"height\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char, &mut height);
                if 0. == height { height = 32i32 as libc::c_float }
                //
                if 0 ==
                       AAS_ValueForBSPEpairKey(ent,
                                               b"model\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               model.as_mut_ptr(), 128i32) {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"func_bobbing without model\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                } else {
                    modelnum = atoi(model.as_mut_ptr().offset(1isize));
                    if modelnum <= 0i32 {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"func_bobbing with invalid model number\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char);
                    } else {
                        if 0 ==
                               AAS_VectorForBSPEpairKey(ent,
                                                        b"origin\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        origin.as_mut_ptr()) {
                            origin[0usize] = 0i32 as vec_t;
                            origin[1usize] = 0i32 as vec_t;
                            origin[2usize] = 0i32 as vec_t
                        }
                        AAS_BSPModelMinsMaxsOrigin(modelnum,
                                                   angles.as_mut_ptr(),
                                                   mins.as_mut_ptr(),
                                                   maxs.as_mut_ptr(),
                                                   0 as *mut vec_t);
                        mins[0usize] = mins[0usize] + origin[0usize];
                        mins[1usize] = mins[1usize] + origin[1usize];
                        mins[2usize] = mins[2usize] + origin[2usize];
                        maxs[0usize] = maxs[0usize] + origin[0usize];
                        maxs[1usize] = maxs[1usize] + origin[1usize];
                        maxs[2usize] = maxs[2usize] + origin[2usize];
                        mid[0usize] = mins[0usize] + maxs[0usize];
                        mid[1usize] = mins[1usize] + maxs[1usize];
                        mid[2usize] = mins[2usize] + maxs[2usize];
                        mid[0usize] =
                            (mid[0usize] as libc::c_double * 0.5f64) as vec_t;
                        mid[1usize] =
                            (mid[1usize] as libc::c_double * 0.5f64) as vec_t;
                        mid[2usize] =
                            (mid[2usize] as libc::c_double * 0.5f64) as vec_t;
                        origin[0usize] = mid[0usize];
                        origin[1usize] = mid[1usize];
                        origin[2usize] = mid[2usize];
                        move_end[0usize] = origin[0usize];
                        move_end[1usize] = origin[1usize];
                        move_end[2usize] = origin[2usize];
                        move_start[0usize] = origin[0usize];
                        move_start[1usize] = origin[1usize];
                        move_start[2usize] = origin[2usize];
                        AAS_IntForBSPEpairKey(ent,
                                              b"spawnflags\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char,
                                              &mut spawnflags);
                        if 0 != spawnflags & 1i32 {
                            axis = 0i32
                        } else if 0 != spawnflags & 2i32 {
                            axis = 1i32
                        } else { axis = 2i32 }
                        move_start[axis as usize] -= height;
                        move_end[axis as usize] += height;
                        Log_Write(b"funcbob model %d, start = {%1.1f, %1.1f, %1.1f} end = {%1.1f, %1.1f, %1.1f}\n\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char, modelnum,
                                  move_start[0usize] as libc::c_double,
                                  move_start[1usize] as libc::c_double,
                                  move_start[2usize] as libc::c_double,
                                  move_end[0usize] as libc::c_double,
                                  move_end[1usize] as libc::c_double,
                                  move_end[2usize] as libc::c_double);
                        i = 0i32;
                        while i < 4i32 {
                            start_edgeverts[i as usize][0usize] =
                                move_start[0usize];
                            start_edgeverts[i as usize][1usize] =
                                move_start[1usize];
                            start_edgeverts[i as usize][2usize] =
                                move_start[2usize];
                            start_edgeverts[i as usize][2usize] +=
                                maxs[2usize] - mid[2usize];
                            start_edgeverts[i as usize][2usize] +=
                                24i32 as libc::c_float;
                            i += 1
                        }
                        start_edgeverts[0usize][0usize] +=
                            maxs[0usize] - mid[0usize];
                        start_edgeverts[0usize][1usize] +=
                            maxs[1usize] - mid[1usize];
                        start_edgeverts[1usize][0usize] +=
                            maxs[0usize] - mid[0usize];
                        start_edgeverts[1usize][1usize] +=
                            mins[1usize] - mid[1usize];
                        start_edgeverts[2usize][0usize] +=
                            mins[0usize] - mid[0usize];
                        start_edgeverts[2usize][1usize] +=
                            mins[1usize] - mid[1usize];
                        start_edgeverts[3usize][0usize] +=
                            mins[0usize] - mid[0usize];
                        start_edgeverts[3usize][1usize] +=
                            maxs[1usize] - mid[1usize];
                        start_plane.dist = start_edgeverts[0usize][2usize];
                        start_plane.normal[0usize] = 0i32 as vec_t;
                        start_plane.normal[1usize] = 0i32 as vec_t;
                        start_plane.normal[2usize] = 1i32 as vec_t;
                        i = 0i32;
                        while i < 4i32 {
                            end_edgeverts[i as usize][0usize] =
                                move_end[0usize];
                            end_edgeverts[i as usize][1usize] =
                                move_end[1usize];
                            end_edgeverts[i as usize][2usize] =
                                move_end[2usize];
                            end_edgeverts[i as usize][2usize] +=
                                maxs[2usize] - mid[2usize];
                            end_edgeverts[i as usize][2usize] +=
                                24i32 as libc::c_float;
                            i += 1
                        }
                        end_edgeverts[0usize][0usize] +=
                            maxs[0usize] - mid[0usize];
                        end_edgeverts[0usize][1usize] +=
                            maxs[1usize] - mid[1usize];
                        end_edgeverts[1usize][0usize] +=
                            maxs[0usize] - mid[0usize];
                        end_edgeverts[1usize][1usize] +=
                            mins[1usize] - mid[1usize];
                        end_edgeverts[2usize][0usize] +=
                            mins[0usize] - mid[0usize];
                        end_edgeverts[2usize][1usize] +=
                            mins[1usize] - mid[1usize];
                        end_edgeverts[3usize][0usize] +=
                            mins[0usize] - mid[0usize];
                        end_edgeverts[3usize][1usize] +=
                            maxs[1usize] - mid[1usize];
                        end_plane.dist = end_edgeverts[0usize][2usize];
                        end_plane.normal[0usize] = 0i32 as vec_t;
                        end_plane.normal[1usize] = 0i32 as vec_t;
                        end_plane.normal[2usize] = 1i32 as vec_t;
                        move_start_top[0usize] = move_start[0usize];
                        move_start_top[1usize] = move_start[1usize];
                        move_start_top[2usize] = move_start[2usize];
                        move_start_top[2usize] +=
                            maxs[2usize] - mid[2usize] +
                                24i32 as libc::c_float;
                        move_end_top[0usize] = move_end[0usize];
                        move_end_top[1usize] = move_end[1usize];
                        move_end_top[2usize] = move_end[2usize];
                        move_end_top[2usize] +=
                            maxs[2usize] - mid[2usize] +
                                24i32 as libc::c_float;
                        //
                        if !(0 ==
                                 AAS_PointAreaNum(move_start_top.as_mut_ptr()))
                           {
                            if !(0 ==
                                     AAS_PointAreaNum(move_end_top.as_mut_ptr()))
                               {
                                i = 0i32;
                                while i < 2i32 {
                                    if i == 0i32 {
                                        firststartreach =
                                            AAS_FindFaceReachabilities(start_edgeverts.as_mut_ptr(),
                                                                       4i32,
                                                                       &mut start_plane,
                                                                       qtrue
                                                                           as
                                                                           libc::c_int);
                                        firstendreach =
                                            AAS_FindFaceReachabilities(end_edgeverts.as_mut_ptr(),
                                                                       4i32,
                                                                       &mut end_plane,
                                                                       qfalse
                                                                           as
                                                                           libc::c_int)
                                    } else {
                                        firststartreach =
                                            AAS_FindFaceReachabilities(end_edgeverts.as_mut_ptr(),
                                                                       4i32,
                                                                       &mut end_plane,
                                                                       qtrue
                                                                           as
                                                                           libc::c_int);
                                        firstendreach =
                                            AAS_FindFaceReachabilities(start_edgeverts.as_mut_ptr(),
                                                                       4i32,
                                                                       &mut start_plane,
                                                                       qfalse
                                                                           as
                                                                           libc::c_int)
                                    }
                                    startreach = firststartreach;
                                    while !startreach.is_null() {
                                        nextstartreach = (*startreach).next;
                                        endreach = firstendreach;
                                        while !endreach.is_null() {
                                            nextendreach = (*endreach).next;
                                            Log_Write(b"funcbob reach from area %d to %d\n\x00"
                                                          as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char,
                                                      (*startreach).areanum,
                                                      (*endreach).areanum);
                                            if i == 0i32 {
                                                org[0usize] =
                                                    move_start_top[0usize];
                                                org[1usize] =
                                                    move_start_top[1usize];
                                                org[2usize] =
                                                    move_start_top[2usize]
                                            } else {
                                                org[0usize] =
                                                    move_end_top[0usize];
                                                org[1usize] =
                                                    move_end_top[1usize];
                                                org[2usize] =
                                                    move_end_top[2usize]
                                            }
                                            dir[0usize] =
                                                (*startreach).start[0usize] -
                                                    org[0usize];
                                            dir[1usize] =
                                                (*startreach).start[1usize] -
                                                    org[1usize];
                                            dir[2usize] =
                                                (*startreach).start[2usize] -
                                                    org[2usize];
                                            dir[2usize] = 0i32 as vec_t;
                                            VectorNormalize(dir.as_mut_ptr());
                                            start[0usize] =
                                                (*startreach).start[0usize];
                                            start[1usize] =
                                                (*startreach).start[1usize];
                                            start[2usize] =
                                                (*startreach).start[2usize];
                                            start[0usize] =
                                                (*startreach).start[0usize] +
                                                    dir[0usize] *
                                                        1i32 as libc::c_float;
                                            start[1usize] =
                                                (*startreach).start[1usize] +
                                                    dir[1usize] *
                                                        1i32 as libc::c_float;
                                            start[2usize] =
                                                (*startreach).start[2usize] +
                                                    dir[2usize] *
                                                        1i32 as libc::c_float;
                                            start[2usize] +=
                                                1i32 as libc::c_float;
                                            end[0usize] =
                                                (*startreach).start[0usize] +
                                                    dir[0usize] *
                                                        16i32 as
                                                            libc::c_float;
                                            end[1usize] =
                                                (*startreach).start[1usize] +
                                                    dir[1usize] *
                                                        16i32 as
                                                            libc::c_float;
                                            end[2usize] =
                                                (*startreach).start[2usize] +
                                                    dir[2usize] *
                                                        16i32 as
                                                            libc::c_float;
                                            end[2usize] +=
                                                1i32 as libc::c_float;
                                            numareas =
                                                AAS_TraceAreas(start.as_mut_ptr(),
                                                               end.as_mut_ptr(),
                                                               areas.as_mut_ptr(),
                                                               points.as_mut_ptr(),
                                                               10i32);
                                            if !(numareas <= 0i32) {
                                                if numareas > 1i32 {
                                                    (*startreach).start[0usize]
                                                        =
                                                        points[1usize][0usize];
                                                    (*startreach).start[1usize]
                                                        =
                                                        points[1usize][1usize];
                                                    (*startreach).start[2usize]
                                                        =
                                                        points[1usize][2usize]
                                                } else {
                                                    (*startreach).start[0usize]
                                                        = end[0usize];
                                                    (*startreach).start[1usize]
                                                        = end[1usize];
                                                    (*startreach).start[2usize]
                                                        = end[2usize]
                                                }
                                                //
                                                if !(0 ==
                                                         AAS_PointAreaNum((*startreach).start.as_mut_ptr()))
                                                   {
                                                    if !(0 ==
                                                             AAS_PointAreaNum((*endreach).end.as_mut_ptr()))
                                                       {
                                                        lreach =
                                                            AAS_AllocReachability();
                                                        (*lreach).areanum =
                                                            (*endreach).areanum;
                                                        if i == 0i32 {
                                                            (*lreach).edgenum
                                                                =
                                                                (move_start[axis
                                                                                as
                                                                                usize]
                                                                     as
                                                                     libc::c_int)
                                                                    << 16i32 |
                                                                    move_end[axis
                                                                                 as
                                                                                 usize]
                                                                        as
                                                                        libc::c_int
                                                                        &
                                                                        0xffffi32
                                                        } else {
                                                            (*lreach).edgenum
                                                                =
                                                                (move_end[axis
                                                                              as
                                                                              usize]
                                                                     as
                                                                     libc::c_int)
                                                                    << 16i32 |
                                                                    move_start[axis
                                                                                   as
                                                                                   usize]
                                                                        as
                                                                        libc::c_int
                                                                        &
                                                                        0xffffi32
                                                        }
                                                        (*lreach).facenum =
                                                            spawnflags <<
                                                                16i32 |
                                                                modelnum;
                                                        (*lreach).start[0usize]
                                                            =
                                                            (*startreach).start[0usize];
                                                        (*lreach).start[1usize]
                                                            =
                                                            (*startreach).start[1usize];
                                                        (*lreach).start[2usize]
                                                            =
                                                            (*startreach).start[2usize];
                                                        (*lreach).end[0usize]
                                                            =
                                                            (*endreach).end[0usize];
                                                        (*lreach).end[1usize]
                                                            =
                                                            (*endreach).end[1usize];
                                                        (*lreach).end[2usize]
                                                            =
                                                            (*endreach).end[2usize];
                                                        (*lreach).traveltype =
                                                            19i32;
                                                        (*lreach).traveltype
                                                            |=
                                                            AAS_TravelFlagsForTeam(ent);
                                                        (*lreach).traveltime =
                                                            aassettings.rs_funcbob
                                                                as
                                                                libc::c_ushort;
                                                        reach_funcbob += 1;
                                                        (*lreach).next =
                                                            *areareachability.offset((*startreach).areanum
                                                                                         as
                                                                                         isize);
                                                        let ref mut fresh3 =
                                                            *areareachability.offset((*startreach).areanum
                                                                                         as
                                                                                         isize);
                                                        *fresh3 = lreach
                                                    }
                                                }
                                            }
                                            endreach = nextendreach
                                        }
                                        startreach = nextstartreach
                                    }
                                    startreach = firststartreach;
                                    while !startreach.is_null() {
                                        nextstartreach = (*startreach).next;
                                        AAS_FreeReachability(startreach);
                                        startreach = nextstartreach
                                    }
                                    endreach = firstendreach;
                                    while !endreach.is_null() {
                                        nextendreach = (*endreach).next;
                                        AAS_FreeReachability(endreach);
                                        endreach = nextendreach
                                    }
                                    //end for
                                    //only go up with func_bobbing entities that go up and down
                                    if 0 == spawnflags & 1i32 &&
                                           0 == spawnflags & 2i32 {
                                        break ;
                                    }
                                    i += 1
                                }
                            }
                        }
                    }
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    };
}
//end of the function AAS_AllocReachability
//===========================================================================
// frees a reachability link
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeReachability(mut lreach:
                                                  *mut aas_lreachability_t) {
    memset(lreach as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_lreachability_t>() as libc::c_ulong);
    (*lreach).next = nextreachability;
    nextreachability = lreach;
    numlreachabilities -= 1;
}
//use a func bob
#[no_mangle]
pub static mut reach_funcbob: libc::c_int = 0;
//end of the function AAS_Reachability_Ladder
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_TravelFlagsForTeam(mut ent: libc::c_int)
 -> libc::c_int {
    let mut notteam: libc::c_int = 0;
    if 0 ==
           AAS_IntForBSPEpairKey(ent,
                                 b"bot_notteam\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 &mut notteam) {
        return 0i32
    }
    if notteam == 1i32 { return 1i32 << 24i32 }
    if notteam == 2i32 { return 2i32 << 24i32 }
    return 0i32;
}
//end of the function AAS_ShutDownReachabilityHeap
//===========================================================================
// returns a reachability link
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AllocReachability() -> *mut aas_lreachability_t {
    let mut r: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    if nextreachability.is_null() { return 0 as *mut aas_lreachability_t }
    if (*nextreachability).next.is_null() {
        AAS_Error(b"AAS_MAX_REACHABILITYSIZE\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char);
    }
    r = nextreachability;
    nextreachability = (*nextreachability).next;
    numlreachabilities += 1;
    return r;
}
//end for
//end for
//end for
//end if
//end for
//end of the function AAS_Reachability_Elevator
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FindFaceReachabilities(mut facepoints:
                                                        *mut vec3_t,
                                                    mut numpoints:
                                                        libc::c_int,
                                                    mut plane:
                                                        *mut aas_plane_t,
                                                    mut towardsface:
                                                        libc::c_int)
 -> *mut aas_lreachability_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut bestfacenum: libc::c_int = 0;
    let mut v1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v3: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v4: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut bestdist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut hordist: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut beststart: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut beststart2: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut bestend: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut bestend2: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut tmp: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut testpoint: vec3_t = [0.; 3];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut lreachabilities: *mut aas_lreachability_t =
        0 as *mut aas_lreachability_t;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut faceplane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut bestfaceplane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    lreachabilities = 0 as *mut aas_lreachability_t;
    bestfacenum = 0i32;
    bestfaceplane = 0 as *mut aas_plane_t;
    let mut current_block_61: u64;
    i = 1i32;
    while i < aasworld.numareas {
        area = &mut *aasworld.areas.offset(i as isize) as *mut aas_area_t;
        bestdist = 999999i32 as libc::c_float;
        j = 0i32;
        while j < (*area).numfaces {
            facenum =
                *aasworld.faceindex.offset(((*area).firstface + j) as isize);
            face =
                &mut *aasworld.faces.offset(abs(facenum) as isize) as
                    *mut aas_face_t;
            //if not a ground face
            if !(0 == (*face).faceflags & 4i32) {
                faceplane =
                    &mut *aasworld.planes.offset((*face).planenum as isize) as
                        *mut aas_plane_t;
                k = 0i32;
                while k < (*face).numedges {
                    edgenum =
                        abs(*aasworld.edgeindex.offset(((*face).firstedge + k)
                                                           as isize));
                    edge =
                        &mut *aasworld.edges.offset(edgenum as isize) as
                            *mut aas_edge_t;
                    v1 =
                        (*aasworld.vertexes.offset((*edge).v[0usize] as
                                                       isize)).as_mut_ptr();
                    v2 =
                        (*aasworld.vertexes.offset((*edge).v[1usize] as
                                                       isize)).as_mut_ptr();
                    l = 0i32;
                    while l < numpoints {
                        v3 = (*facepoints.offset(l as isize)).as_mut_ptr();
                        v4 =
                            (*facepoints.offset(((l + 1i32) % numpoints) as
                                                    isize)).as_mut_ptr();
                        dist =
                            AAS_ClosestEdgePoints(v1, v2, v3, v4, faceplane,
                                                  plane,
                                                  beststart.as_mut_ptr(),
                                                  bestend.as_mut_ptr(),
                                                  beststart2.as_mut_ptr(),
                                                  bestend2.as_mut_ptr(),
                                                  bestdist);
                        if dist < bestdist {
                            bestfacenum = facenum;
                            bestfaceplane = faceplane;
                            bestdist = dist
                        }
                        l += 1
                    }
                    k += 1
                }
            }
            j += 1
        }
        //end if
        //end for
        //end for
        //end for
        //
        if !(bestdist > 192i32 as libc::c_float) {
            VectorMiddle(beststart.as_mut_ptr(), beststart2.as_mut_ptr(),
                         beststart.as_mut_ptr());
            VectorMiddle(bestend.as_mut_ptr(), bestend2.as_mut_ptr(),
                         bestend.as_mut_ptr());
            if 0 == towardsface {
                tmp[0usize] = beststart[0usize];
                tmp[1usize] = beststart[1usize];
                tmp[2usize] = beststart[2usize];
                beststart[0usize] = bestend[0usize];
                beststart[1usize] = bestend[1usize];
                beststart[2usize] = bestend[2usize];
                bestend[0usize] = tmp[0usize];
                bestend[1usize] = tmp[1usize];
                bestend[2usize] = tmp[2usize]
            }
            hordir[0usize] = bestend[0usize] - beststart[0usize];
            hordir[1usize] = bestend[1usize] - beststart[1usize];
            hordir[2usize] = bestend[2usize] - beststart[2usize];
            hordir[2usize] = 0i32 as vec_t;
            hordist = VectorLength(hordir.as_mut_ptr() as *const vec_t);
            //
            if !(hordist >
                     2i32 as libc::c_float *
                         AAS_MaxJumpDistance(aassettings.phys_jumpvel)) {
                //the end point should not be significantly higher than the start point
                if !(bestend[2usize] - 32i32 as libc::c_float >
                         beststart[2usize]) {
                    //don't fall down too far
                    if !(bestend[2usize] <
                             beststart[2usize] - 128i32 as libc::c_float) {
                        //the distance should not be too far
                        if hordist > 32i32 as libc::c_float {
                            //check for walk off ledge
                            if 0 ==
                                   AAS_HorizontalVelocityForJump(0i32 as
                                                                     libc::c_float,
                                                                 beststart.as_mut_ptr(),
                                                                 bestend.as_mut_ptr(),
                                                                 &mut speed) {
                                current_block_61 = 3276175668257526147;
                            } else {
                                current_block_61 = 11793792312832361944;
                            }
                        } else { current_block_61 = 11793792312832361944; }
                        match current_block_61 {
                            3276175668257526147 => { }
                            _ => {
                                beststart[2usize] += 1i32 as libc::c_float;
                                bestend[2usize] += 1i32 as libc::c_float;
                                if 0 != towardsface {
                                    testpoint[0usize] = bestend[0usize];
                                    testpoint[1usize] = bestend[1usize];
                                    testpoint[2usize] = bestend[2usize]
                                } else {
                                    testpoint[0usize] = beststart[0usize];
                                    testpoint[1usize] = beststart[1usize];
                                    testpoint[2usize] = beststart[2usize]
                                }
                                if !bestfaceplane.is_null() {
                                    testpoint[2usize] =
                                        ((*bestfaceplane).dist -
                                             ((*bestfaceplane).normal[0usize]
                                                  * testpoint[0usize] +
                                                  (*bestfaceplane).normal[1usize]
                                                      * testpoint[1usize] +
                                                  (*bestfaceplane).normal[2usize]
                                                      * testpoint[2usize])) /
                                            (*bestfaceplane).normal[2usize]
                                } else { testpoint[2usize] = 0i32 as vec_t }
                                //
                                if 0 ==
                                       AAS_PointInsideFace(bestfacenum,
                                                           testpoint.as_mut_ptr(),
                                                           0.1f32) as u64 {
                                    //if the faces are not overlapping then only go down
                                    if bestend[2usize] -
                                           16i32 as libc::c_float >
                                           beststart[2usize] {
                                        current_block_61 =
                                            3276175668257526147;
                                    } else {
                                        current_block_61 =
                                            6545907279487748450;
                                    }
                                } else {
                                    current_block_61 = 6545907279487748450;
                                }
                                match current_block_61 {
                                    3276175668257526147 => { }
                                    _ => {
                                        lreach = AAS_AllocReachability();
                                        if lreach.is_null() {
                                            return lreachabilities
                                        }
                                        (*lreach).areanum = i;
                                        (*lreach).facenum = 0i32;
                                        (*lreach).edgenum = 0i32;
                                        (*lreach).start[0usize] =
                                            beststart[0usize];
                                        (*lreach).start[1usize] =
                                            beststart[1usize];
                                        (*lreach).start[2usize] =
                                            beststart[2usize];
                                        (*lreach).end[0usize] =
                                            bestend[0usize];
                                        (*lreach).end[1usize] =
                                            bestend[1usize];
                                        (*lreach).end[2usize] =
                                            bestend[2usize];
                                        (*lreach).traveltype = 0i32;
                                        (*lreach).traveltime =
                                            0i32 as libc::c_ushort;
                                        (*lreach).next = lreachabilities;
                                        lreachabilities = lreach;
                                        if 0 != towardsface {
                                            AAS_PermanentLine((*lreach).start.as_mut_ptr(),
                                                              (*lreach).end.as_mut_ptr(),
                                                              1i32);
                                        } else {
                                            AAS_PermanentLine((*lreach).start.as_mut_ptr(),
                                                              (*lreach).end.as_mut_ptr(),
                                                              2i32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return lreachabilities;
}
//end of the function MaxJumpHeight
//===========================================================================
// returns true if a player can only crouch in the area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_MaxJumpDistance(mut phys_jumpvel: libc::c_float)
 -> libc::c_float {
    let mut phys_gravity: libc::c_float = 0.;
    let mut phys_maxvelocity: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    phys_gravity = aassettings.phys_gravity;
    phys_maxvelocity = aassettings.phys_maxvelocity;
    t =
        sqrt(aassettings.rs_maxjumpfallheight as libc::c_double /
                 (0.5f64 * phys_gravity as libc::c_double)) as libc::c_float;
    return phys_maxvelocity * (t + phys_jumpvel / phys_gravity);
}
//end of the function VectorBetweenVectors
//===========================================================================
// returns the mid point between the two vectors
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn VectorMiddle(mut v1: *mut vec_t, mut v2: *mut vec_t,
                                      mut middle: *mut vec_t) {
    *middle.offset(0isize) = *v1.offset(0isize) + *v2.offset(0isize);
    *middle.offset(1isize) = *v1.offset(1isize) + *v2.offset(1isize);
    *middle.offset(2isize) = *v1.offset(2isize) + *v2.offset(2isize);
    *middle.offset(0isize) =
        (*middle.offset(0isize) as libc::c_double * 0.5f64) as vec_t;
    *middle.offset(1isize) =
        (*middle.offset(1isize) as libc::c_double * 0.5f64) as vec_t;
    *middle.offset(2isize) =
        (*middle.offset(2isize) as libc::c_double * 0.5f64) as vec_t;
}
//end of the function VectorMiddle
//===========================================================================
// calculate a range of points closest to each other on both edges
//
// Parameter:			beststart1		start of the range of points on edge v1-v2
//						beststart2		end of the range of points  on edge v1-v2
//						bestend1		start of the range of points on edge v3-v4
//						bestend2		end of the range of points  on edge v3-v4
//						bestdist		best distance so far
// Returns:				-
// Changes Globals:		-
//===========================================================================
/*
float AAS_ClosestEdgePoints(vec3_t v1, vec3_t v2, vec3_t v3, vec3_t v4,
							aas_plane_t *plane1, aas_plane_t *plane2,
							vec3_t beststart, vec3_t bestend, float bestdist)
{
	vec3_t dir1, dir2, p1, p2, p3, p4;
	float a1, a2, b1, b2, dist;
	int founddist;

	//edge vectors
	VectorSubtract(v2, v1, dir1);
	VectorSubtract(v4, v3, dir2);
	//get the horizontal directions
	dir1[2] = 0;
	dir2[2] = 0;
	//
	// p1 = point on an edge vector of area2 closest to v1
	// p2 = point on an edge vector of area2 closest to v2
	// p3 = point on an edge vector of area1 closest to v3
	// p4 = point on an edge vector of area1 closest to v4
	//
	if (dir2[0])
	{
		a2 = dir2[1] / dir2[0];
		b2 = v3[1] - a2 * v3[0];
		//point on the edge vector of area2 closest to v1
		p1[0] = (DotProduct(v1, dir2) - (a2 * dir2[0] + b2 * dir2[1])) / dir2[0];
		p1[1] = a2 * p1[0] + b2;
		//point on the edge vector of area2 closest to v2
		p2[0] = (DotProduct(v2, dir2) - (a2 * dir2[0] + b2 * dir2[1])) / dir2[0];
		p2[1] = a2 * p2[0] + b2;
	} //end if
	else
	{
		//point on the edge vector of area2 closest to v1
		p1[0] = v3[0];
		p1[1] = v1[1];
		//point on the edge vector of area2 closest to v2
		p2[0] = v3[0];
		p2[1] = v2[1];
	} //end else
	//
	if (dir1[0])
	{
		//
		a1 = dir1[1] / dir1[0];
		b1 = v1[1] - a1 * v1[0];
		//point on the edge vector of area1 closest to v3
		p3[0] = (DotProduct(v3, dir1) - (a1 * dir1[0] + b1 * dir1[1])) / dir1[0];
		p3[1] = a1 * p3[0] + b1;
		//point on the edge vector of area1 closest to v4
		p4[0] = (DotProduct(v4, dir1) - (a1 * dir1[0] + b1 * dir1[1])) / dir1[0];
		p4[1] = a1 * p4[0] + b1;
	} //end if
	else
	{
		//point on the edge vector of area1 closest to v3
		p3[0] = v1[0];
		p3[1] = v3[1];
		//point on the edge vector of area1 closest to v4
		p4[0] = v1[0];
		p4[1] = v4[1];
	} //end else
	//start with zero z-coordinates
	p1[2] = 0;
	p2[2] = 0;
	p3[2] = 0;
	p4[2] = 0;
	//calculate the z-coordinates from the ground planes
	p1[2] = (plane2->dist - DotProduct(plane2->normal, p1)) / plane2->normal[2];
	p2[2] = (plane2->dist - DotProduct(plane2->normal, p2)) / plane2->normal[2];
	p3[2] = (plane1->dist - DotProduct(plane1->normal, p3)) / plane1->normal[2];
	p4[2] = (plane1->dist - DotProduct(plane1->normal, p4)) / plane1->normal[2];
	//
	founddist = qfalse;
	//
	if (VectorBetweenVectors(p1, v3, v4))
	{
		dist = VectorDistance(v1, p1);
		if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
		{
			VectorMiddle(beststart, v1, beststart);
			VectorMiddle(bestend, p1, bestend);
		} //end if
		else if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(v1, beststart);
			VectorCopy(p1, bestend);
		} //end if
		founddist = qtrue;
	} //end if
	if (VectorBetweenVectors(p2, v3, v4))
	{
		dist = VectorDistance(v2, p2);
		if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
		{
			VectorMiddle(beststart, v2, beststart);
			VectorMiddle(bestend, p2, bestend);
		} //end if
		else if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(v2, beststart);
			VectorCopy(p2, bestend);
		} //end if
		founddist = qtrue;
	} //end else if
	if (VectorBetweenVectors(p3, v1, v2))
	{
		dist = VectorDistance(v3, p3);
		if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
		{
			VectorMiddle(beststart, p3, beststart);
			VectorMiddle(bestend, v3, bestend);
		} //end if
		else if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(p3, beststart);
			VectorCopy(v3, bestend);
		} //end if
		founddist = qtrue;
	} //end else if
	if (VectorBetweenVectors(p4, v1, v2))
	{
		dist = VectorDistance(v4, p4);
		if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
		{
			VectorMiddle(beststart, p4, beststart);
			VectorMiddle(bestend, v4, bestend);
		} //end if
		else if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(p4, beststart);
			VectorCopy(v4, bestend);
		} //end if
		founddist = qtrue;
	} //end else if
	//if no shortest distance was found the shortest distance
	//is between one of the vertexes of edge1 and one of edge2
	if (!founddist)
	{
		dist = VectorDistance(v1, v3);
		if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(v1, beststart);
			VectorCopy(v3, bestend);
		} //end if
		dist = VectorDistance(v1, v4);
		if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(v1, beststart);
			VectorCopy(v4, bestend);
		} //end if
		dist = VectorDistance(v2, v3);
		if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(v2, beststart);
			VectorCopy(v3, bestend);
		} //end if
		dist = VectorDistance(v2, v4);
		if (dist < bestdist)
		{
			bestdist = dist;
			VectorCopy(v2, beststart);
			VectorCopy(v4, bestend);
		} //end if
	} //end if
	return bestdist;
} //end of the function AAS_ClosestEdgePoints*/
#[no_mangle]
pub unsafe extern "C" fn AAS_ClosestEdgePoints(mut v1: *mut vec_t,
                                               mut v2: *mut vec_t,
                                               mut v3: *mut vec_t,
                                               mut v4: *mut vec_t,
                                               mut plane1: *mut aas_plane_t,
                                               mut plane2: *mut aas_plane_t,
                                               mut beststart1: *mut vec_t,
                                               mut bestend1: *mut vec_t,
                                               mut beststart2: *mut vec_t,
                                               mut bestend2: *mut vec_t,
                                               mut bestdist: libc::c_float)
 -> libc::c_float {
    let mut dir1: vec3_t = [0.; 3];
    let mut dir2: vec3_t = [0.; 3];
    let mut p1: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    let mut p3: vec3_t = [0.; 3];
    let mut p4: vec3_t = [0.; 3];
    let mut a1: libc::c_float = 0.;
    let mut a2: libc::c_float = 0.;
    let mut b1: libc::c_float = 0.;
    let mut b2: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut founddist: libc::c_int = 0;
    dir1[0usize] = *v2.offset(0isize) - *v1.offset(0isize);
    dir1[1usize] = *v2.offset(1isize) - *v1.offset(1isize);
    dir1[2usize] = *v2.offset(2isize) - *v1.offset(2isize);
    dir2[0usize] = *v4.offset(0isize) - *v3.offset(0isize);
    dir2[1usize] = *v4.offset(1isize) - *v3.offset(1isize);
    dir2[2usize] = *v4.offset(2isize) - *v3.offset(2isize);
    dir1[2usize] = 0i32 as vec_t;
    dir2[2usize] = 0i32 as vec_t;
    if 0. != dir2[0usize] {
        a2 = dir2[1usize] / dir2[0usize];
        b2 = *v3.offset(1isize) - a2 * *v3.offset(0isize);
        p1[0usize] =
            (*v1.offset(0isize) * dir2[0usize] +
                 *v1.offset(1isize) * dir2[1usize] +
                 *v1.offset(2isize) * dir2[2usize] -
                 (a2 * dir2[0usize] + b2 * dir2[1usize])) / dir2[0usize];
        p1[1usize] = a2 * p1[0usize] + b2;
        p2[0usize] =
            (*v2.offset(0isize) * dir2[0usize] +
                 *v2.offset(1isize) * dir2[1usize] +
                 *v2.offset(2isize) * dir2[2usize] -
                 (a2 * dir2[0usize] + b2 * dir2[1usize])) / dir2[0usize];
        p2[1usize] = a2 * p2[0usize] + b2
    } else {
        p1[0usize] = *v3.offset(0isize);
        p1[1usize] = *v1.offset(1isize);
        p2[0usize] = *v3.offset(0isize);
        p2[1usize] = *v2.offset(1isize)
    }
    if 0. != dir1[0usize] {
        a1 = dir1[1usize] / dir1[0usize];
        b1 = *v1.offset(1isize) - a1 * *v1.offset(0isize);
        p3[0usize] =
            (*v3.offset(0isize) * dir1[0usize] +
                 *v3.offset(1isize) * dir1[1usize] +
                 *v3.offset(2isize) * dir1[2usize] -
                 (a1 * dir1[0usize] + b1 * dir1[1usize])) / dir1[0usize];
        p3[1usize] = a1 * p3[0usize] + b1;
        p4[0usize] =
            (*v4.offset(0isize) * dir1[0usize] +
                 *v4.offset(1isize) * dir1[1usize] +
                 *v4.offset(2isize) * dir1[2usize] -
                 (a1 * dir1[0usize] + b1 * dir1[1usize])) / dir1[0usize];
        p4[1usize] = a1 * p4[0usize] + b1
    } else {
        p3[0usize] = *v1.offset(0isize);
        p3[1usize] = *v3.offset(1isize);
        p4[0usize] = *v1.offset(0isize);
        p4[1usize] = *v4.offset(1isize)
    }
    p1[2usize] = 0i32 as vec_t;
    p2[2usize] = 0i32 as vec_t;
    p3[2usize] = 0i32 as vec_t;
    p4[2usize] = 0i32 as vec_t;
    p1[2usize] =
        ((*plane2).dist -
             ((*plane2).normal[0usize] * p1[0usize] +
                  (*plane2).normal[1usize] * p1[1usize] +
                  (*plane2).normal[2usize] * p1[2usize])) /
            (*plane2).normal[2usize];
    p2[2usize] =
        ((*plane2).dist -
             ((*plane2).normal[0usize] * p2[0usize] +
                  (*plane2).normal[1usize] * p2[1usize] +
                  (*plane2).normal[2usize] * p2[2usize])) /
            (*plane2).normal[2usize];
    p3[2usize] =
        ((*plane1).dist -
             ((*plane1).normal[0usize] * p3[0usize] +
                  (*plane1).normal[1usize] * p3[1usize] +
                  (*plane1).normal[2usize] * p3[2usize])) /
            (*plane1).normal[2usize];
    p4[2usize] =
        ((*plane1).dist -
             ((*plane1).normal[0usize] * p4[0usize] +
                  (*plane1).normal[1usize] * p4[1usize] +
                  (*plane1).normal[2usize] * p4[2usize])) /
            (*plane1).normal[2usize];
    founddist = qfalse as libc::c_int;
    if 0 != VectorBetweenVectors(p1.as_mut_ptr(), v3, v4) {
        dist = VectorDistance(v1, p1.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64 &&
               (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
           {
            dist1 = VectorDistance(beststart1, v1);
            dist2 = VectorDistance(beststart2, v1);
            if dist1 > dist2 {
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0isize) = *v1.offset(0isize);
                    *beststart2.offset(1isize) = *v1.offset(1isize);
                    *beststart2.offset(2isize) = *v1.offset(2isize)
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0isize) = *v1.offset(0isize);
                *beststart1.offset(1isize) = *v1.offset(1isize);
                *beststart1.offset(2isize) = *v1.offset(2isize)
            }
            dist1 = VectorDistance(bestend1, p1.as_mut_ptr());
            dist2 = VectorDistance(bestend2, p1.as_mut_ptr());
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0isize) = p1[0usize];
                    *bestend2.offset(1isize) = p1[1usize];
                    *bestend2.offset(2isize) = p1[2usize]
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0isize) = p1[0usize];
                *bestend1.offset(1isize) = p1[1usize];
                *bestend1.offset(2isize) = p1[2usize]
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = *v1.offset(0isize);
            *beststart1.offset(1isize) = *v1.offset(1isize);
            *beststart1.offset(2isize) = *v1.offset(2isize);
            *beststart2.offset(0isize) = *v1.offset(0isize);
            *beststart2.offset(1isize) = *v1.offset(1isize);
            *beststart2.offset(2isize) = *v1.offset(2isize);
            *bestend1.offset(0isize) = p1[0usize];
            *bestend1.offset(1isize) = p1[1usize];
            *bestend1.offset(2isize) = p1[2usize];
            *bestend2.offset(0isize) = p1[0usize];
            *bestend2.offset(1isize) = p1[1usize];
            *bestend2.offset(2isize) = p1[2usize]
        }
        founddist = qtrue as libc::c_int
    }
    if 0 != VectorBetweenVectors(p2.as_mut_ptr(), v3, v4) {
        dist = VectorDistance(v2, p2.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64 &&
               (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
           {
            dist1 = VectorDistance(beststart1, v2);
            dist2 = VectorDistance(beststart2, v2);
            if dist1 > dist2 {
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0isize) = *v2.offset(0isize);
                    *beststart2.offset(1isize) = *v2.offset(1isize);
                    *beststart2.offset(2isize) = *v2.offset(2isize)
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0isize) = *v2.offset(0isize);
                *beststart1.offset(1isize) = *v2.offset(1isize);
                *beststart1.offset(2isize) = *v2.offset(2isize)
            }
            dist1 = VectorDistance(bestend1, p2.as_mut_ptr());
            dist2 = VectorDistance(bestend2, p2.as_mut_ptr());
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0isize) = p2[0usize];
                    *bestend2.offset(1isize) = p2[1usize];
                    *bestend2.offset(2isize) = p2[2usize]
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0isize) = p2[0usize];
                *bestend1.offset(1isize) = p2[1usize];
                *bestend1.offset(2isize) = p2[2usize]
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = *v2.offset(0isize);
            *beststart1.offset(1isize) = *v2.offset(1isize);
            *beststart1.offset(2isize) = *v2.offset(2isize);
            *beststart2.offset(0isize) = *v2.offset(0isize);
            *beststart2.offset(1isize) = *v2.offset(1isize);
            *beststart2.offset(2isize) = *v2.offset(2isize);
            *bestend1.offset(0isize) = p2[0usize];
            *bestend1.offset(1isize) = p2[1usize];
            *bestend1.offset(2isize) = p2[2usize];
            *bestend2.offset(0isize) = p2[0usize];
            *bestend2.offset(1isize) = p2[1usize];
            *bestend2.offset(2isize) = p2[2usize]
        }
        founddist = qtrue as libc::c_int
    }
    if 0 != VectorBetweenVectors(p3.as_mut_ptr(), v1, v2) {
        dist = VectorDistance(v3, p3.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64 &&
               (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
           {
            dist1 = VectorDistance(beststart1, p3.as_mut_ptr());
            dist2 = VectorDistance(beststart2, p3.as_mut_ptr());
            if dist1 > dist2 {
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0isize) = p3[0usize];
                    *beststart2.offset(1isize) = p3[1usize];
                    *beststart2.offset(2isize) = p3[2usize]
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0isize) = p3[0usize];
                *beststart1.offset(1isize) = p3[1usize];
                *beststart1.offset(2isize) = p3[2usize]
            }
            dist1 = VectorDistance(bestend1, v3);
            dist2 = VectorDistance(bestend2, v3);
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0isize) = *v3.offset(0isize);
                    *bestend2.offset(1isize) = *v3.offset(1isize);
                    *bestend2.offset(2isize) = *v3.offset(2isize)
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0isize) = *v3.offset(0isize);
                *bestend1.offset(1isize) = *v3.offset(1isize);
                *bestend1.offset(2isize) = *v3.offset(2isize)
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = p3[0usize];
            *beststart1.offset(1isize) = p3[1usize];
            *beststart1.offset(2isize) = p3[2usize];
            *beststart2.offset(0isize) = p3[0usize];
            *beststart2.offset(1isize) = p3[1usize];
            *beststart2.offset(2isize) = p3[2usize];
            *bestend1.offset(0isize) = *v3.offset(0isize);
            *bestend1.offset(1isize) = *v3.offset(1isize);
            *bestend1.offset(2isize) = *v3.offset(2isize);
            *bestend2.offset(0isize) = *v3.offset(0isize);
            *bestend2.offset(1isize) = *v3.offset(1isize);
            *bestend2.offset(2isize) = *v3.offset(2isize)
        }
        founddist = qtrue as libc::c_int
    }
    if 0 != VectorBetweenVectors(p4.as_mut_ptr(), v1, v2) {
        dist = VectorDistance(v4, p4.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64 &&
               (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
           {
            dist1 = VectorDistance(beststart1, p4.as_mut_ptr());
            dist2 = VectorDistance(beststart2, p4.as_mut_ptr());
            if dist1 > dist2 {
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0isize) = p4[0usize];
                    *beststart2.offset(1isize) = p4[1usize];
                    *beststart2.offset(2isize) = p4[2usize]
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0isize) = p4[0usize];
                *beststart1.offset(1isize) = p4[1usize];
                *beststart1.offset(2isize) = p4[2usize]
            }
            dist1 = VectorDistance(bestend1, v4);
            dist2 = VectorDistance(bestend2, v4);
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0isize) = *v4.offset(0isize);
                    *bestend2.offset(1isize) = *v4.offset(1isize);
                    *bestend2.offset(2isize) = *v4.offset(2isize)
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0isize) = *v4.offset(0isize);
                *bestend1.offset(1isize) = *v4.offset(1isize);
                *bestend1.offset(2isize) = *v4.offset(2isize)
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = p4[0usize];
            *beststart1.offset(1isize) = p4[1usize];
            *beststart1.offset(2isize) = p4[2usize];
            *beststart2.offset(0isize) = p4[0usize];
            *beststart2.offset(1isize) = p4[1usize];
            *beststart2.offset(2isize) = p4[2usize];
            *bestend1.offset(0isize) = *v4.offset(0isize);
            *bestend1.offset(1isize) = *v4.offset(1isize);
            *bestend1.offset(2isize) = *v4.offset(2isize);
            *bestend2.offset(0isize) = *v4.offset(0isize);
            *bestend2.offset(1isize) = *v4.offset(1isize);
            *bestend2.offset(2isize) = *v4.offset(2isize)
        }
        founddist = qtrue as libc::c_int
    }
    if 0 == founddist {
        dist = VectorDistance(v1, v3);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = *v1.offset(0isize);
            *beststart1.offset(1isize) = *v1.offset(1isize);
            *beststart1.offset(2isize) = *v1.offset(2isize);
            *beststart2.offset(0isize) = *v1.offset(0isize);
            *beststart2.offset(1isize) = *v1.offset(1isize);
            *beststart2.offset(2isize) = *v1.offset(2isize);
            *bestend1.offset(0isize) = *v3.offset(0isize);
            *bestend1.offset(1isize) = *v3.offset(1isize);
            *bestend1.offset(2isize) = *v3.offset(2isize);
            *bestend2.offset(0isize) = *v3.offset(0isize);
            *bestend2.offset(1isize) = *v3.offset(1isize);
            *bestend2.offset(2isize) = *v3.offset(2isize)
        }
        dist = VectorDistance(v1, v4);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = *v1.offset(0isize);
            *beststart1.offset(1isize) = *v1.offset(1isize);
            *beststart1.offset(2isize) = *v1.offset(2isize);
            *beststart2.offset(0isize) = *v1.offset(0isize);
            *beststart2.offset(1isize) = *v1.offset(1isize);
            *beststart2.offset(2isize) = *v1.offset(2isize);
            *bestend1.offset(0isize) = *v4.offset(0isize);
            *bestend1.offset(1isize) = *v4.offset(1isize);
            *bestend1.offset(2isize) = *v4.offset(2isize);
            *bestend2.offset(0isize) = *v4.offset(0isize);
            *bestend2.offset(1isize) = *v4.offset(1isize);
            *bestend2.offset(2isize) = *v4.offset(2isize)
        }
        dist = VectorDistance(v2, v3);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = *v2.offset(0isize);
            *beststart1.offset(1isize) = *v2.offset(1isize);
            *beststart1.offset(2isize) = *v2.offset(2isize);
            *beststart2.offset(0isize) = *v2.offset(0isize);
            *beststart2.offset(1isize) = *v2.offset(1isize);
            *beststart2.offset(2isize) = *v2.offset(2isize);
            *bestend1.offset(0isize) = *v3.offset(0isize);
            *bestend1.offset(1isize) = *v3.offset(1isize);
            *bestend1.offset(2isize) = *v3.offset(2isize);
            *bestend2.offset(0isize) = *v3.offset(0isize);
            *bestend2.offset(1isize) = *v3.offset(1isize);
            *bestend2.offset(2isize) = *v3.offset(2isize)
        }
        dist = VectorDistance(v2, v4);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0isize) = *v2.offset(0isize);
            *beststart1.offset(1isize) = *v2.offset(1isize);
            *beststart1.offset(2isize) = *v2.offset(2isize);
            *beststart2.offset(0isize) = *v2.offset(0isize);
            *beststart2.offset(1isize) = *v2.offset(1isize);
            *beststart2.offset(2isize) = *v2.offset(2isize);
            *bestend1.offset(0isize) = *v4.offset(0isize);
            *bestend1.offset(1isize) = *v4.offset(1isize);
            *bestend1.offset(2isize) = *v4.offset(2isize);
            *bestend2.offset(0isize) = *v4.offset(0isize);
            *bestend2.offset(1isize) = *v4.offset(1isize);
            *bestend2.offset(2isize) = *v4.offset(2isize)
        }
    }
    return bestdist;
}
//end of the function AAS_Reachability_Step_Barrier_WaterJump_WalkOffLedge
//===========================================================================
// returns the distance between the two vectors
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn VectorDistance(mut v1: *mut vec_t,
                                        mut v2: *mut vec_t) -> libc::c_float {
    let mut dir: vec3_t = [0.; 3];
    dir[0usize] = *v2.offset(0isize) - *v1.offset(0isize);
    dir[1usize] = *v2.offset(1isize) - *v1.offset(1isize);
    dir[2usize] = *v2.offset(2isize) - *v1.offset(2isize);
    return VectorLength(dir.as_mut_ptr() as *const vec_t);
}
//end of the function VectorDistance
//===========================================================================
// returns true if the first vector is between the last two vectors
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn VectorBetweenVectors(mut v: *mut vec_t,
                                              mut v1: *mut vec_t,
                                              mut v2: *mut vec_t)
 -> libc::c_int {
    let mut dir1: vec3_t = [0.; 3];
    let mut dir2: vec3_t = [0.; 3];
    dir1[0usize] = *v.offset(0isize) - *v1.offset(0isize);
    dir1[1usize] = *v.offset(1isize) - *v1.offset(1isize);
    dir1[2usize] = *v.offset(2isize) - *v1.offset(2isize);
    dir2[0usize] = *v.offset(0isize) - *v2.offset(0isize);
    dir2[1usize] = *v.offset(1isize) - *v2.offset(1isize);
    dir2[2usize] = *v.offset(2isize) - *v2.offset(2isize);
    return (dir1[0usize] * dir2[0usize] + dir1[1usize] * dir2[1usize] +
                dir1[2usize] * dir2[2usize] <= 0i32 as libc::c_float) as
               libc::c_int;
}
//end for
//end of the function AAS_Reachability_Teleport
//===========================================================================
// create possible elevator (func_plat) reachabilities
// this is very game dependent.... :(
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Elevator() {
    let mut area1num: libc::c_int = 0;
    let mut area2num: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut lip: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut ent: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut pos1: vec3_t = [0.; 3];
    let mut pos2: vec3_t = [0.; 3];
    let mut mids: vec3_t = [0.; 3];
    let mut platbottom: vec3_t = [0.; 3];
    let mut plattop: vec3_t = [0.; 3];
    let mut bottomorg: vec3_t = [0.; 3];
    let mut toporg: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut xvals: [vec_t; 8] = [0.; 8];
    let mut yvals: [vec_t; 8] = [0.; 8];
    let mut xvals_top: [vec_t; 8] = [0.; 8];
    let mut yvals_top: [vec_t; 8] = [0.; 8];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if 0 ==
                   strcmp(classname.as_mut_ptr(),
                          b"func_plat\x00" as *const u8 as
                              *const libc::c_char) {
                //REACH_DEBUG
                if 0 ==
                       AAS_ValueForBSPEpairKey(ent,
                                               b"model\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               model.as_mut_ptr(), 128i32) {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"func_plat without model\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                } else {
                    modelnum = atoi(model.as_mut_ptr().offset(1isize));
                    if modelnum <= 0i32 {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"func_plat with invalid model number\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char);
                    } else {
                        AAS_BSPModelMinsMaxsOrigin(modelnum,
                                                   angles.as_mut_ptr(),
                                                   mins.as_mut_ptr(),
                                                   maxs.as_mut_ptr(),
                                                   origin.as_mut_ptr());
                        AAS_VectorForBSPEpairKey(ent,
                                                 b"origin\x00" as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char,
                                                 origin.as_mut_ptr());
                        pos1[0usize] = origin[0usize];
                        pos1[1usize] = origin[1usize];
                        pos1[2usize] = origin[2usize];
                        pos2[0usize] = origin[0usize];
                        pos2[1usize] = origin[1usize];
                        pos2[2usize] = origin[2usize];
                        AAS_FloatForBSPEpairKey(ent,
                                                b"lip\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                &mut lip);
                        if 0. == lip { lip = 8i32 as libc::c_float }
                        AAS_FloatForBSPEpairKey(ent,
                                                b"height\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                &mut height);
                        if 0. == height {
                            height = maxs[2usize] - mins[2usize] - lip
                        }
                        AAS_FloatForBSPEpairKey(ent,
                                                b"speed\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                &mut speed);
                        if 0. == speed { speed = 200i32 as libc::c_float }
                        pos2[2usize] -= height;
                        mids[0usize] = mins[0usize] + maxs[0usize];
                        mids[1usize] = mins[1usize] + maxs[1usize];
                        mids[2usize] = mins[2usize] + maxs[2usize];
                        platbottom[0usize] =
                            (pos2[0usize] as libc::c_double +
                                 mids[0usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        platbottom[1usize] =
                            (pos2[1usize] as libc::c_double +
                                 mids[1usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        platbottom[2usize] =
                            (pos2[2usize] as libc::c_double +
                                 mids[2usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        platbottom[2usize] =
                            maxs[2usize] - (pos1[2usize] - pos2[2usize]) +
                                2i32 as libc::c_float;
                        mids[0usize] = mins[0usize] + maxs[0usize];
                        mids[1usize] = mins[1usize] + maxs[1usize];
                        mids[2usize] = mins[2usize] + maxs[2usize];
                        plattop[0usize] =
                            (pos2[0usize] as libc::c_double +
                                 mids[0usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        plattop[1usize] =
                            (pos2[1usize] as libc::c_double +
                                 mids[1usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        plattop[2usize] =
                            (pos2[2usize] as libc::c_double +
                                 mids[2usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        plattop[2usize] =
                            maxs[2usize] + 2i32 as libc::c_float;
                        i = 0i32;
                        while i < 3i32 {
                            mins[i as usize] -= 1i32 as libc::c_float;
                            maxs[i as usize] += 1i32 as libc::c_float;
                            i += 1
                        }
                        mids[0usize] = mins[0usize] + maxs[0usize];
                        mids[1usize] = mins[1usize] + maxs[1usize];
                        mids[2usize] = mins[2usize] + maxs[2usize];
                        mids[0usize] =
                            (mids[0usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        mids[1usize] =
                            (mids[1usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        mids[2usize] =
                            (mids[2usize] as libc::c_double * 0.5f64) as
                                vec_t;
                        xvals[0usize] = mins[0usize];
                        xvals[1usize] = mids[0usize];
                        xvals[2usize] = maxs[0usize];
                        xvals[3usize] = mids[0usize];
                        yvals[0usize] = mids[1usize];
                        yvals[1usize] = maxs[1usize];
                        yvals[2usize] = mids[1usize];
                        yvals[3usize] = mins[1usize];
                        xvals[4usize] = mins[0usize];
                        xvals[5usize] = maxs[0usize];
                        xvals[6usize] = maxs[0usize];
                        xvals[7usize] = mins[0usize];
                        yvals[4usize] = maxs[1usize];
                        yvals[5usize] = maxs[1usize];
                        yvals[6usize] = mins[1usize];
                        yvals[7usize] = mins[1usize];
                        let mut current_block_119: u64;
                        i = 0i32;
                        while i < 9i32 {
                            //check at the sides of the plat
                            if i < 8i32 {
                                bottomorg[0usize] =
                                    origin[0usize] + xvals[i as usize];
                                bottomorg[1usize] =
                                    origin[1usize] + yvals[i as usize];
                                bottomorg[2usize] =
                                    platbottom[2usize] +
                                        16i32 as libc::c_float;
                                area1num =
                                    AAS_PointAreaNum(bottomorg.as_mut_ptr());
                                k = 0i32;
                                while k < 16i32 {
                                    if 0 != area1num {
                                        if 0 != AAS_AreaGrounded(area1num) ||
                                               0 != AAS_AreaSwim(area1num) {
                                            break ;
                                        }
                                    }
                                    bottomorg[2usize] +=
                                        4i32 as libc::c_float;
                                    area1num =
                                        AAS_PointAreaNum(bottomorg.as_mut_ptr());
                                    k += 1
                                }
                                //end if
                                //if in solid
                                if k >= 16i32 {
                                    current_block_119 = 5892776923941496671;
                                } else {
                                    current_block_119 = 7385833325316299293;
                                }
                            } else {
                                //end if
                                //end if
                                //at the middle of the plat
                                bottomorg[0usize] = plattop[0usize];
                                bottomorg[1usize] = plattop[1usize];
                                bottomorg[2usize] = plattop[2usize];
                                bottomorg[2usize] += 24i32 as libc::c_float;
                                area1num =
                                    AAS_PointAreaNum(bottomorg.as_mut_ptr());
                                if 0 == area1num {
                                    current_block_119 = 5892776923941496671;
                                } else {
                                    bottomorg[0usize] = platbottom[0usize];
                                    bottomorg[1usize] = platbottom[1usize];
                                    bottomorg[2usize] = platbottom[2usize];
                                    bottomorg[2usize] +=
                                        24i32 as libc::c_float;
                                    current_block_119 = 7385833325316299293;
                                }
                            }
                            match current_block_119 {
                                7385833325316299293 => {
                                    n = 0i32;
                                    while n < 3i32 {
                                        k = 0i32;
                                        while k < 3i32 {
                                            mins[k as usize] -=
                                                4i32 as libc::c_float;
                                            maxs[k as usize] +=
                                                4i32 as libc::c_float;
                                            k += 1
                                        }
                                        xvals_top[0usize] = mins[0usize];
                                        xvals_top[1usize] = mids[0usize];
                                        xvals_top[2usize] = maxs[0usize];
                                        xvals_top[3usize] = mids[0usize];
                                        yvals_top[0usize] = mids[1usize];
                                        yvals_top[1usize] = maxs[1usize];
                                        yvals_top[2usize] = mids[1usize];
                                        yvals_top[3usize] = mins[1usize];
                                        xvals_top[4usize] = mins[0usize];
                                        xvals_top[5usize] = maxs[0usize];
                                        xvals_top[6usize] = maxs[0usize];
                                        xvals_top[7usize] = mins[0usize];
                                        yvals_top[4usize] = maxs[1usize];
                                        yvals_top[5usize] = maxs[1usize];
                                        yvals_top[6usize] = mins[1usize];
                                        yvals_top[7usize] = mins[1usize];
                                        j = 0i32;
                                        while j < 8i32 {
                                            toporg[0usize] =
                                                origin[0usize] +
                                                    xvals_top[j as usize];
                                            toporg[1usize] =
                                                origin[1usize] +
                                                    yvals_top[j as usize];
                                            toporg[2usize] =
                                                plattop[2usize] +
                                                    16i32 as libc::c_float;
                                            area2num =
                                                AAS_PointAreaNum(toporg.as_mut_ptr());
                                            l = 0i32;
                                            while l < 16i32 {
                                                if 0 != area2num {
                                                    if 0 !=
                                                           AAS_AreaGrounded(area2num)
                                                           ||
                                                           0 !=
                                                               AAS_AreaSwim(area2num)
                                                       {
                                                        start[0usize] =
                                                            plattop[0usize];
                                                        start[1usize] =
                                                            plattop[1usize];
                                                        start[2usize] =
                                                            plattop[2usize];
                                                        start[2usize] +=
                                                            32i32 as
                                                                libc::c_float;
                                                        end[0usize] =
                                                            toporg[0usize];
                                                        end[1usize] =
                                                            toporg[1usize];
                                                        end[2usize] =
                                                            toporg[2usize];
                                                        end[2usize] +=
                                                            1i32 as
                                                                libc::c_float;
                                                        trace =
                                                            AAS_TraceClientBBox(start.as_mut_ptr(),
                                                                                end.as_mut_ptr(),
                                                                                4i32,
                                                                                -1i32);
                                                        if trace.fraction >=
                                                               1i32 as
                                                                   libc::c_float
                                                           {
                                                            break ;
                                                        }
                                                    }
                                                }
                                                toporg[2usize] +=
                                                    4i32 as libc::c_float;
                                                area2num =
                                                    AAS_PointAreaNum(toporg.as_mut_ptr());
                                                l += 1
                                            }
                                            //end if
                                            //if in solid
                                            if !(l >= 16i32) {
                                                //never create a reachability in the same area
                                                if !(area2num == area1num) {
                                                    //if the area isn't grounded
                                                    if !(0 ==
                                                             AAS_AreaGrounded(area2num))
                                                       {
                                                        //if there already exists reachability between the areas
                                                        if !(0 !=
                                                                 AAS_ReachabilityExists(area1num,
                                                                                        area2num)
                                                                     as u64) {
                                                            dir[0usize] =
                                                                bottomorg[0usize]
                                                                    -
                                                                    platbottom[0usize];
                                                            dir[1usize] =
                                                                bottomorg[1usize]
                                                                    -
                                                                    platbottom[1usize];
                                                            dir[2usize] =
                                                                bottomorg[2usize]
                                                                    -
                                                                    platbottom[2usize];
                                                            VectorNormalize(dir.as_mut_ptr());
                                                            dir[0usize] =
                                                                bottomorg[0usize]
                                                                    +
                                                                    24i32 as
                                                                        libc::c_float
                                                                        *
                                                                        dir[0usize];
                                                            dir[1usize] =
                                                                bottomorg[1usize]
                                                                    +
                                                                    24i32 as
                                                                        libc::c_float
                                                                        *
                                                                        dir[1usize];
                                                            dir[2usize] =
                                                                bottomorg[2usize];
                                                            p = 0i32;
                                                            while p < 3i32 {
                                                                if dir[p as
                                                                           usize]
                                                                       <
                                                                       origin[p
                                                                                  as
                                                                                  usize]
                                                                           +
                                                                           mins[p
                                                                                    as
                                                                                    usize]
                                                                       ||
                                                                       dir[p
                                                                               as
                                                                               usize]
                                                                           >
                                                                           origin[p
                                                                                      as
                                                                                      usize]
                                                                               +
                                                                               maxs[p
                                                                                        as
                                                                                        usize]
                                                                   {
                                                                    break ;
                                                                }
                                                                p += 1
                                                            }
                                                            if !(p >= 3i32) {
                                                                lreach =
                                                                    AAS_AllocReachability();
                                                                if !lreach.is_null()
                                                                   {
                                                                    (*lreach).areanum
                                                                        =
                                                                        area2num;
                                                                    (*lreach).facenum
                                                                        =
                                                                        modelnum;
                                                                    (*lreach).edgenum
                                                                        =
                                                                        height
                                                                            as
                                                                            libc::c_int;
                                                                    (*lreach).start[0usize]
                                                                        =
                                                                        dir[0usize];
                                                                    (*lreach).start[1usize]
                                                                        =
                                                                        dir[1usize];
                                                                    (*lreach).start[2usize]
                                                                        =
                                                                        dir[2usize];
                                                                    (*lreach).end[0usize]
                                                                        =
                                                                        toporg[0usize];
                                                                    (*lreach).end[1usize]
                                                                        =
                                                                        toporg[1usize];
                                                                    (*lreach).end[2usize]
                                                                        =
                                                                        toporg[2usize];
                                                                    (*lreach).traveltype
                                                                        =
                                                                        11i32;
                                                                    (*lreach).traveltype
                                                                        |=
                                                                        AAS_TravelFlagsForTeam(ent);
                                                                    (*lreach).traveltime
                                                                        =
                                                                        (aassettings.rs_startelevator
                                                                             +
                                                                             height
                                                                                 *
                                                                                 100i32
                                                                                     as
                                                                                     libc::c_float
                                                                                 /
                                                                                 speed)
                                                                            as
                                                                            libc::c_ushort;
                                                                    (*lreach).next
                                                                        =
                                                                        *areareachability.offset(area1num
                                                                                                     as
                                                                                                     isize);
                                                                    let ref mut fresh4 =
                                                                        *areareachability.offset(area1num
                                                                                                     as
                                                                                                     isize);
                                                                    *fresh4 =
                                                                        lreach;
                                                                    n =
                                                                        9999i32;
                                                                    reach_elevator
                                                                        += 1
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            j += 1
                                        }
                                        n += 1
                                    }
                                }
                                _ => { }
                            }
                            i += 1
                        }
                    }
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    };
}
//use an elevator
#[no_mangle]
pub static mut reach_elevator: libc::c_int = 0;
//end op the function AAS_BarrierJumpTravelTime
//===========================================================================
// returns true if there already exists a reachability from area1 to area2
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ReachabilityExists(mut area1num: libc::c_int,
                                                mut area2num: libc::c_int)
 -> qboolean {
    let mut r: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    r = *areareachability.offset(area1num as isize);
    while !r.is_null() {
        if (*r).areanum == area2num { return qtrue }
        r = (*r).next
    }
    return qfalse;
}
//end of the function AAS_TravelFlagsForTeam
//===========================================================================
// create possible teleporter reachabilities
// this is very game dependent.... :(
//
// classname = trigger_multiple or trigger_teleport
// target = "t1"
//
// classname = target_teleporter
// targetname = "t1"
// target = "t2"
//
// classname = misc_teleporter_dest
// targetname = "t2"
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Teleport() {
    let mut area1num: libc::c_int = 0;
    let mut area2num: libc::c_int = 0;
    let mut target: [libc::c_char; 128] = [0; 128];
    let mut targetname: [libc::c_char; 128] = [0; 128];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut ent: libc::c_int = 0;
    let mut dest: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut destorigin: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut mid: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
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
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    let mut areas: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut current_block_61: u64;
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if 0 ==
                   strcmp(classname.as_mut_ptr(),
                          b"trigger_multiple\x00" as *const u8 as
                              *const libc::c_char) {
                AAS_ValueForBSPEpairKey(ent,
                                        b"model\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        model.as_mut_ptr(), 128i32);
                botimport.Print.expect("non-null function pointer")(1i32,
                                                                    b"trigger_multiple model = \"%s\"\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    model.as_mut_ptr());
                angles[2usize] = 0i32 as vec_t;
                angles[1usize] = angles[2usize];
                angles[0usize] = angles[1usize];
                AAS_BSPModelMinsMaxsOrigin(atoi(model.as_mut_ptr().offset(1isize)),
                                           angles.as_mut_ptr(),
                                           mins.as_mut_ptr(),
                                           maxs.as_mut_ptr(),
                                           origin.as_mut_ptr());
                //
                if 0 ==
                       AAS_ValueForBSPEpairKey(ent,
                                               b"target\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               target.as_mut_ptr(), 128i32) {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"trigger_multiple at %1.0f %1.0f %1.0f without target\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        origin[0usize]
                                                                            as
                                                                            libc::c_double,
                                                                        origin[1usize]
                                                                            as
                                                                            libc::c_double,
                                                                        origin[2usize]
                                                                            as
                                                                            libc::c_double);
                    current_block_61 = 7351195479953500246;
                } else {
                    dest = AAS_NextBSPEntity(0i32);
                    while 0 != dest {
                        if !(0 ==
                                 AAS_ValueForBSPEpairKey(dest,
                                                         b"classname\x00" as
                                                             *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char,
                                                         classname.as_mut_ptr(),
                                                         128i32)) {
                            if 0 ==
                                   strcmp(classname.as_mut_ptr(),
                                          b"target_teleporter\x00" as
                                              *const u8 as
                                              *const libc::c_char) {
                                if !(0 ==
                                         AAS_ValueForBSPEpairKey(dest,
                                                                 b"targetname\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                                     as
                                                                     *mut libc::c_char,
                                                                 targetname.as_mut_ptr(),
                                                                 128i32)) {
                                    if 0 ==
                                           strcmp(targetname.as_mut_ptr(),
                                                  target.as_mut_ptr()) {
                                        break ;
                                    }
                                }
                            }
                        }
                        dest = AAS_NextBSPEntity(dest)
                    }
                    //end if
                    //end if
                    //end for
                    if 0 == dest {
                        current_block_61 = 7351195479953500246;
                    } else if 0 ==
                                  AAS_ValueForBSPEpairKey(dest,
                                                          b"target\x00" as
                                                              *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char,
                                                          target.as_mut_ptr(),
                                                          128i32) {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"target_teleporter without target\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char);
                        current_block_61 = 7351195479953500246;
                    } else { current_block_61 = 3123434771885419771; }
                }
            } else if 0 ==
                          strcmp(classname.as_mut_ptr(),
                                 b"trigger_teleport\x00" as *const u8 as
                                     *const libc::c_char) {
                AAS_ValueForBSPEpairKey(ent,
                                        b"model\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        model.as_mut_ptr(), 128i32);
                botimport.Print.expect("non-null function pointer")(1i32,
                                                                    b"trigger_teleport model = \"%s\"\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    model.as_mut_ptr());
                angles[2usize] = 0i32 as vec_t;
                angles[1usize] = angles[2usize];
                angles[0usize] = angles[1usize];
                AAS_BSPModelMinsMaxsOrigin(atoi(model.as_mut_ptr().offset(1isize)),
                                           angles.as_mut_ptr(),
                                           mins.as_mut_ptr(),
                                           maxs.as_mut_ptr(),
                                           origin.as_mut_ptr());
                //
                if 0 ==
                       AAS_ValueForBSPEpairKey(ent,
                                               b"target\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               target.as_mut_ptr(), 128i32) {
                    botimport.Print.expect("non-null function pointer")(3i32,
                                                                        b"trigger_teleport at %1.0f %1.0f %1.0f without target\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        origin[0usize]
                                                                            as
                                                                            libc::c_double,
                                                                        origin[1usize]
                                                                            as
                                                                            libc::c_double,
                                                                        origin[2usize]
                                                                            as
                                                                            libc::c_double);
                    current_block_61 = 7351195479953500246;
                } else { current_block_61 = 3123434771885419771; }
            } else {
                //end if
                //end if
                current_block_61 = 7351195479953500246;
            }
            match current_block_61 {
                7351195479953500246 => { }
                _ => {
                    dest = AAS_NextBSPEntity(0i32);
                    while 0 != dest {
                        //classname should be misc_teleporter_dest
			//but I've also seen target_position and actually any
			//entity could be used... burp
                        if 0 !=
                               AAS_ValueForBSPEpairKey(dest,
                                                       b"targetname\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       targetname.as_mut_ptr(),
                                                       128i32) {
                            if 0 ==
                                   strcmp(targetname.as_mut_ptr(),
                                          target.as_mut_ptr()) {
                                break ;
                            }
                        }
                        dest = AAS_NextBSPEntity(dest)
                    }
                    //end if
                    //end if
                    //end for
                    if 0 == dest {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"teleporter without misc_teleporter_dest (%s)\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            target.as_mut_ptr());
                    } else if 0 ==
                                  AAS_VectorForBSPEpairKey(dest,
                                                           b"origin\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           destorigin.as_mut_ptr())
                     {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"teleporter destination (%s) without origin\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            target.as_mut_ptr());
                    } else {
                        area2num = AAS_PointAreaNum(destorigin.as_mut_ptr());
                        //if not teleported into a teleporter or into a jumppad
                        if 0 == AAS_AreaTeleporter(area2num) &&
                               0 == AAS_AreaJumpPad(area2num) {
                            end[0usize] = destorigin[0usize];
                            end[1usize] = destorigin[1usize];
                            end[2usize] = destorigin[2usize];
                            end[2usize] -= 64i32 as libc::c_float;
                            trace =
                                AAS_TraceClientBBox(destorigin.as_mut_ptr(),
                                                    end.as_mut_ptr(), 4i32,
                                                    -1i32);
                            if 0 != trace.startsolid as u64 {
                                botimport.Print.expect("non-null function pointer")(3i32,
                                                                                    b"teleporter destination (%s) in solid\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char
                                                                                        as
                                                                                        *mut libc::c_char,
                                                                                    target.as_mut_ptr());
                                current_block_61 = 7351195479953500246;
                            } else {
                                AAS_FloatForBSPEpairKey(dest,
                                                        b"angle\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        &mut angle);
                                if 0. != angle {
                                    angles[0usize] = 0i32 as vec_t;
                                    angles[1usize] = angle;
                                    angles[2usize] = 0i32 as vec_t;
                                    AngleVectors(angles.as_mut_ptr() as
                                                     *const vec_t,
                                                 velocity.as_mut_ptr(),
                                                 0 as *mut vec_t,
                                                 0 as *mut vec_t);
                                    velocity[0usize] =
                                        velocity[0usize] *
                                            400i32 as libc::c_float;
                                    velocity[1usize] =
                                        velocity[1usize] *
                                            400i32 as libc::c_float;
                                    velocity[2usize] =
                                        velocity[2usize] *
                                            400i32 as libc::c_float
                                } else {
                                    velocity[2usize] = 0i32 as vec_t;
                                    velocity[1usize] = velocity[2usize];
                                    velocity[0usize] = velocity[1usize]
                                }
                                cmdmove[2usize] = 0i32 as vec_t;
                                cmdmove[1usize] = cmdmove[2usize];
                                cmdmove[0usize] = cmdmove[1usize];
                                AAS_PredictClientMovement(&mut move_0, -1i32,
                                                          destorigin.as_mut_ptr(),
                                                          2i32,
                                                          qfalse as
                                                              libc::c_int,
                                                          velocity.as_mut_ptr(),
                                                          cmdmove.as_mut_ptr(),
                                                          0i32, 30i32, 0.1f32,
                                                          1i32 | 4i32 | 8i32 |
                                                              16i32 | 32i32 |
                                                              128i32 | 256i32,
                                                          0i32,
                                                          qfalse as
                                                              libc::c_int);
                                area2num =
                                    AAS_PointAreaNum(move_0.endpos.as_mut_ptr());
                                if 0 != move_0.stopevent & (8i32 | 16i32) {
                                    botimport.Print.expect("non-null function pointer")(2i32,
                                                                                        b"teleported into slime or lava at dest %s\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char
                                                                                            as
                                                                                            *mut libc::c_char,
                                                                                        target.as_mut_ptr());
                                }
                                destorigin[0usize] = move_0.endpos[0usize];
                                destorigin[1usize] = move_0.endpos[1usize];
                                destorigin[2usize] = move_0.endpos[2usize];
                                current_block_61 = 7178192492338286402;
                            }
                        } else { current_block_61 = 7178192492338286402; }
                        match current_block_61 {
                            7351195479953500246 => { }
                            _ => {
                                mins[0usize] = origin[0usize] + mins[0usize];
                                mins[1usize] = origin[1usize] + mins[1usize];
                                mins[2usize] = origin[2usize] + mins[2usize];
                                maxs[0usize] = origin[0usize] + maxs[0usize];
                                maxs[1usize] = origin[1usize] + maxs[1usize];
                                maxs[2usize] = origin[2usize] + maxs[2usize];
                                mid[0usize] = mins[0usize] + maxs[0usize];
                                mid[1usize] = mins[1usize] + maxs[1usize];
                                mid[2usize] = mins[2usize] + maxs[2usize];
                                mid[0usize] =
                                    (mid[0usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                mid[1usize] =
                                    (mid[1usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                mid[2usize] =
                                    (mid[2usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                areas =
                                    AAS_LinkEntityClientBBox(mins.as_mut_ptr(),
                                                             maxs.as_mut_ptr(),
                                                             -1i32, 4i32);
                                if areas.is_null() {
                                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                                        b"trigger_multiple not in any area\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char
                                                                                            as
                                                                                            *mut libc::c_char);
                                }
                                link = areas;
                                while !link.is_null() {
                                    //if (!AAS_AreaGrounded(link->areanum)) continue;
                                    if !(0 ==
                                             AAS_AreaTeleporter((*link).areanum))
                                       {
                                        area1num = (*link).areanum;
                                        lreach = AAS_AllocReachability();
                                        if lreach.is_null() { break ; }
                                        (*lreach).areanum = area2num;
                                        (*lreach).facenum = 0i32;
                                        (*lreach).edgenum = 0i32;
                                        (*lreach).start[0usize] = mid[0usize];
                                        (*lreach).start[1usize] = mid[1usize];
                                        (*lreach).start[2usize] = mid[2usize];
                                        (*lreach).end[0usize] =
                                            destorigin[0usize];
                                        (*lreach).end[1usize] =
                                            destorigin[1usize];
                                        (*lreach).end[2usize] =
                                            destorigin[2usize];
                                        (*lreach).traveltype = 10i32;
                                        (*lreach).traveltype |=
                                            AAS_TravelFlagsForTeam(ent);
                                        (*lreach).traveltime =
                                            aassettings.rs_teleport as
                                                libc::c_ushort;
                                        (*lreach).next =
                                            *areareachability.offset(area1num
                                                                         as
                                                                         isize);
                                        let ref mut fresh5 =
                                            *areareachability.offset(area1num
                                                                         as
                                                                         isize);
                                        *fresh5 = lreach;
                                        reach_teleport += 1
                                    }
                                    link = (*link).next_area
                                }
                                AAS_UnlinkFromAreas(areas);
                            }
                        }
                    }
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    };
}
//teleport
#[no_mangle]
pub static mut reach_teleport: libc::c_int = 0;
//end of the function AAS_AreaJumpPad
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaTeleporter(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).contents & 64i32;
}
//end for
//end for
//end of the function AAS_Reachability_FuncBobbing
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_JumpPad() {
    let mut face2num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut area2num: libc::c_int = 0;
    let mut visualize: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut bot_visualizejumppads: libc::c_int = 0;
    //int modelnum, ent2;
	//float dist, time, height, gravity, forward;
    let mut speed: libc::c_float = 0.;
    let mut zvel: libc::c_float = 0.;
    //float hordist;
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut areastart: vec3_t = [0.; 3];
    let mut facecenter: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    //vec3_t origin, ent2origin, angles, teststart;
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
    //aas_trace_t trace;
    let mut areas: *mut aas_link_t = 0 as *mut aas_link_t;
    let mut link: *mut aas_link_t = 0 as *mut aas_link_t;
    //char target[MAX_EPAIRKEY], targetname[MAX_EPAIRKEY], model[MAX_EPAIRKEY];
    let mut classname: [libc::c_char; 128] = [0; 128];
    bot_visualizejumppads =
        LibVarValue(b"bot_visualizejumppads\x00" as *const u8 as
                        *const libc::c_char,
                    b"0\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if !(0 !=
                     strcmp(classname.as_mut_ptr(),
                            b"trigger_push\x00" as *const u8 as
                                *const libc::c_char)) {
                //
                if !(0 ==
                         AAS_GetJumpPadInfo(ent, areastart.as_mut_ptr(),
                                            absmins.as_mut_ptr(),
                                            absmaxs.as_mut_ptr(),
                                            velocity.as_mut_ptr())) {
                    areas =
                        AAS_LinkEntityClientBBox(absmins.as_mut_ptr(),
                                                 absmaxs.as_mut_ptr(), -1i32,
                                                 4i32);
                    link = areas;
                    while !link.is_null() {
                        if 0 != AAS_AreaJumpPad((*link).areanum) { break ; }
                        link = (*link).next_area
                    }
                    //end for
                    if link.is_null() {
                        botimport.Print.expect("non-null function pointer")(1i32,
                                                                            b"trigger_push not in any jump pad area\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char);
                        AAS_UnlinkFromAreas(areas);
                    } else {
                        botimport.Print.expect("non-null function pointer")(1i32,
                                                                            b"found a trigger_push with velocity %f %f %f\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            velocity[0usize]
                                                                                as
                                                                                libc::c_double,
                                                                            velocity[1usize]
                                                                                as
                                                                                libc::c_double,
                                                                            velocity[2usize]
                                                                                as
                                                                                libc::c_double);
                        if 0. != velocity[0usize] || 0. != velocity[1usize] {
                            cmdmove[0usize] = 0i32 as vec_t;
                            cmdmove[1usize] = 0i32 as vec_t;
                            cmdmove[2usize] = 0i32 as vec_t;
                            memset(&mut move_0 as *mut aas_clientmove_t as
                                       *mut libc::c_void, 0i32,
                                   ::std::mem::size_of::<aas_clientmove_t>()
                                       as libc::c_ulong);
                            area2num = 0i32;
                            i = 0i32;
                            while i < 20i32 {
                                AAS_PredictClientMovement(&mut move_0, -1i32,
                                                          areastart.as_mut_ptr(),
                                                          2i32,
                                                          qfalse as
                                                              libc::c_int,
                                                          velocity.as_mut_ptr(),
                                                          cmdmove.as_mut_ptr(),
                                                          0i32, 30i32, 0.1f32,
                                                          1i32 | 4i32 | 8i32 |
                                                              16i32 | 32i32 |
                                                              128i32 | 256i32,
                                                          0i32,
                                                          bot_visualizejumppads);
                                area2num = move_0.endarea;
                                link = areas;
                                while !link.is_null() {
                                    if !(0 ==
                                             AAS_AreaJumpPad((*link).areanum))
                                       {
                                        if (*link).areanum == area2num {
                                            break ;
                                        }
                                    }
                                    link = (*link).next_area
                                }
                                //end if
                                if link.is_null() { break ; }
                                areastart[0usize] = move_0.endpos[0usize];
                                areastart[1usize] = move_0.endpos[1usize];
                                areastart[2usize] = move_0.endpos[2usize];
                                velocity[0usize] = move_0.velocity[0usize];
                                velocity[1usize] = move_0.velocity[1usize];
                                velocity[2usize] = move_0.velocity[2usize];
                                i += 1
                            }
                            if 0 != area2num && i < 20i32 {
                                link = areas;
                                while !link.is_null() {
                                    if !(0 ==
                                             AAS_AreaJumpPad((*link).areanum))
                                       {
                                        if !(0 !=
                                                 AAS_ReachabilityExists((*link).areanum,
                                                                        area2num)
                                                     as u64) {
                                            lreach = AAS_AllocReachability();
                                            if lreach.is_null() {
                                                AAS_UnlinkFromAreas(areas);
                                                return
                                            }
                                            (*lreach).areanum = area2num;
                                            (*lreach).facenum =
                                                velocity[2usize] as
                                                    libc::c_int;
                                            (*lreach).edgenum =
                                                sqrt((velocity[0usize] *
                                                          velocity[0usize] +
                                                          velocity[1usize] *
                                                              velocity[1usize])
                                                         as libc::c_double) as
                                                    libc::c_int;
                                            (*lreach).start[0usize] =
                                                areastart[0usize];
                                            (*lreach).start[1usize] =
                                                areastart[1usize];
                                            (*lreach).start[2usize] =
                                                areastart[2usize];
                                            (*lreach).end[0usize] =
                                                move_0.endpos[0usize];
                                            (*lreach).end[1usize] =
                                                move_0.endpos[1usize];
                                            (*lreach).end[2usize] =
                                                move_0.endpos[2usize];
                                            (*lreach).traveltype = 18i32;
                                            (*lreach).traveltype |=
                                                AAS_TravelFlagsForTeam(ent);
                                            (*lreach).traveltime =
                                                aassettings.rs_jumppad as
                                                    libc::c_ushort;
                                            (*lreach).next =
                                                *areareachability.offset((*link).areanum
                                                                             as
                                                                             isize);
                                            let ref mut fresh6 =
                                                *areareachability.offset((*link).areanum
                                                                             as
                                                                             isize);
                                            *fresh6 = lreach;
                                            reach_jumppad += 1
                                        }
                                    }
                                    link = (*link).next_area
                                }
                            }
                        }
                        //end for
                        //end if
                        //end if
                        //
                        if !(fabs(velocity[0usize] as libc::c_double) >
                                 100i32 as libc::c_double ||
                                 fabs(velocity[1usize] as libc::c_double) >
                                     100i32 as libc::c_double) {
                            area2num = 1i32;
                            while area2num < aasworld.numareas {
                                visualize = qfalse as libc::c_int;
                                link = areas;
                                while !link.is_null() {
                                    if 0 !=
                                           AAS_ReachabilityExists((*link).areanum,
                                                                  area2num) as
                                               u64 {
                                        break ;
                                    }
                                    if 0 != AAS_AreaJumpPad((*link).areanum) {
                                        if (*link).areanum == area2num {
                                            break ;
                                        }
                                    }
                                    link = (*link).next_area
                                }
                                //end if
                                //end if
                                if link.is_null() {
                                    area2 =
                                        &mut *aasworld.areas.offset(area2num
                                                                        as
                                                                        isize)
                                            as *mut aas_area_t;
                                    i = 0i32;
                                    while i < (*area2).numfaces {
                                        face2num =
                                            *aasworld.faceindex.offset(((*area2).firstface
                                                                            +
                                                                            i)
                                                                           as
                                                                           isize);
                                        face2 =
                                            &mut *aasworld.faces.offset(abs(face2num)
                                                                            as
                                                                            isize)
                                                as *mut aas_face_t;
                                        //if it is not a ground face
                                        if !(0 == (*face2).faceflags & 4i32) {
                                            AAS_FaceCenter(face2num,
                                                           facecenter.as_mut_ptr());
                                            //only go higher up
                                            if !(facecenter[2usize] <
                                                     areastart[2usize]) {
                                                zvel = velocity[2usize];
                                                ret =
                                                    AAS_HorizontalVelocityForJump(zvel,
                                                                                  areastart.as_mut_ptr(),
                                                                                  facecenter.as_mut_ptr(),
                                                                                  &mut speed);
                                                if 0 != ret &&
                                                       speed <
                                                           150i32 as
                                                               libc::c_float {
                                                    dir[0usize] =
                                                        facecenter[0usize] -
                                                            areastart[0usize];
                                                    dir[1usize] =
                                                        facecenter[1usize] -
                                                            areastart[1usize];
                                                    dir[2usize] =
                                                        facecenter[2usize] -
                                                            areastart[2usize];
                                                    dir[2usize] =
                                                        0i32 as vec_t;
                                                    cmdmove[0usize] =
                                                        dir[0usize] * speed;
                                                    cmdmove[1usize] =
                                                        dir[1usize] * speed;
                                                    cmdmove[2usize] =
                                                        dir[2usize] * speed;
                                                    AAS_PredictClientMovement(&mut move_0,
                                                                              -1i32,
                                                                              areastart.as_mut_ptr(),
                                                                              2i32,
                                                                              qfalse
                                                                                  as
                                                                                  libc::c_int,
                                                                              velocity.as_mut_ptr(),
                                                                              cmdmove.as_mut_ptr(),
                                                                              30i32,
                                                                              30i32,
                                                                              0.1f32,
                                                                              4i32
                                                                                  |
                                                                                  8i32
                                                                                  |
                                                                                  16i32
                                                                                  |
                                                                                  32i32
                                                                                  |
                                                                                  128i32
                                                                                  |
                                                                                  256i32
                                                                                  |
                                                                                  1024i32,
                                                                              area2num,
                                                                              visualize);
                                                    if move_0.frames < 30i32
                                                           &&
                                                           0 ==
                                                               move_0.stopevent
                                                                   &
                                                                   (8i32 |
                                                                        16i32
                                                                        |
                                                                        32i32)
                                                           &&
                                                           0 !=
                                                               move_0.stopevent
                                                                   &
                                                                   (1024i32 |
                                                                        128i32
                                                                        |
                                                                        256i32)
                                                       {
                                                        link = areas;
                                                        while !link.is_null()
                                                              {
                                                            if (*link).areanum
                                                                   ==
                                                                   move_0.endarea
                                                               {
                                                                break ;
                                                            }
                                                            link =
                                                                (*link).next_area
                                                        }
                                                        if link.is_null() {
                                                            link = areas;
                                                            while !link.is_null()
                                                                  {
                                                                if !(0 ==
                                                                         AAS_AreaJumpPad((*link).areanum))
                                                                   {
                                                                    if !(0 !=
                                                                             AAS_ReachabilityExists((*link).areanum,
                                                                                                    area2num)
                                                                                 as
                                                                                 u64)
                                                                       {
                                                                        lreach
                                                                            =
                                                                            AAS_AllocReachability();
                                                                        if lreach.is_null()
                                                                           {
                                                                            AAS_UnlinkFromAreas(areas);
                                                                            return
                                                                        }
                                                                        (*lreach).areanum
                                                                            =
                                                                            move_0.endarea;
                                                                        (*lreach).facenum
                                                                            =
                                                                            velocity[2usize]
                                                                                as
                                                                                libc::c_int;
                                                                        (*lreach).edgenum
                                                                            =
                                                                            sqrt((cmdmove[0usize]
                                                                                      *
                                                                                      cmdmove[0usize]
                                                                                      +
                                                                                      cmdmove[1usize]
                                                                                          *
                                                                                          cmdmove[1usize])
                                                                                     as
                                                                                     libc::c_double)
                                                                                as
                                                                                libc::c_int;
                                                                        (*lreach).start[0usize]
                                                                            =
                                                                            areastart[0usize];
                                                                        (*lreach).start[1usize]
                                                                            =
                                                                            areastart[1usize];
                                                                        (*lreach).start[2usize]
                                                                            =
                                                                            areastart[2usize];
                                                                        (*lreach).end[0usize]
                                                                            =
                                                                            facecenter[0usize];
                                                                        (*lreach).end[1usize]
                                                                            =
                                                                            facecenter[1usize];
                                                                        (*lreach).end[2usize]
                                                                            =
                                                                            facecenter[2usize];
                                                                        (*lreach).traveltype
                                                                            =
                                                                            18i32;
                                                                        (*lreach).traveltype
                                                                            |=
                                                                            AAS_TravelFlagsForTeam(ent);
                                                                        (*lreach).traveltime
                                                                            =
                                                                            aassettings.rs_aircontrolledjumppad
                                                                                as
                                                                                libc::c_ushort;
                                                                        (*lreach).next
                                                                            =
                                                                            *areareachability.offset((*link).areanum
                                                                                                         as
                                                                                                         isize);
                                                                        let ref mut fresh7 =
                                                                            *areareachability.offset((*link).areanum
                                                                                                         as
                                                                                                         isize);
                                                                        *fresh7
                                                                            =
                                                                            lreach;
                                                                        reach_jumppad
                                                                            +=
                                                                            1
                                                                    }
                                                                }
                                                                link =
                                                                    (*link).next_area
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        i += 1
                                    }
                                }
                                area2num += 1
                            }
                            AAS_UnlinkFromAreas(areas);
                        }
                    }
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    };
}
//jump pads
#[no_mangle]
pub static mut reach_jumppad: libc::c_int = 0;
//end of the function AAS_AreaGroundFaceArea
//===========================================================================
// returns the center of a face
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FaceCenter(mut facenum: libc::c_int,
                                        mut center: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    face = &mut *aasworld.faces.offset(facenum as isize) as *mut aas_face_t;
    let ref mut fresh9 = *center.offset(1isize);
    let ref mut fresh8 = *center.offset(2isize);
    *fresh8 = 0i32 as vec_t;
    *fresh9 = *fresh8;
    *center.offset(0isize) = *fresh9;
    i = 0i32;
    while i < (*face).numedges {
        edge =
            &mut *aasworld.edges.offset(abs(*aasworld.edgeindex.offset(((*face).firstedge
                                                                            +
                                                                            i)
                                                                           as
                                                                           isize))
                                            as isize) as *mut aas_edge_t;
        *center.offset(0isize) =
            *center.offset(0isize) +
                (*aasworld.vertexes.offset((*edge).v[0usize] as
                                               isize))[0usize];
        *center.offset(1isize) =
            *center.offset(1isize) +
                (*aasworld.vertexes.offset((*edge).v[0usize] as
                                               isize))[1usize];
        *center.offset(2isize) =
            *center.offset(2isize) +
                (*aasworld.vertexes.offset((*edge).v[0usize] as
                                               isize))[2usize];
        *center.offset(0isize) =
            *center.offset(0isize) +
                (*aasworld.vertexes.offset((*edge).v[1usize] as
                                               isize))[0usize];
        *center.offset(1isize) =
            *center.offset(1isize) +
                (*aasworld.vertexes.offset((*edge).v[1usize] as
                                               isize))[1usize];
        *center.offset(2isize) =
            *center.offset(2isize) +
                (*aasworld.vertexes.offset((*edge).v[1usize] as
                                               isize))[2usize];
        i += 1
    }
    scale = (0.5f64 / (*face).numedges as libc::c_double) as libc::c_float;
    *center.offset(0isize) = *center.offset(0isize) * scale;
    *center.offset(1isize) = *center.offset(1isize) * scale;
    *center.offset(2isize) = *center.offset(2isize) * scale;
}
//end of the function AAS_Reachability_WeaponJump
//===========================================================================
// calculates additional walk off ledge reachabilities for the given area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_WalkOffLedge(mut areanum:
                                                           libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut face3num: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut edge3num: libc::c_int = 0;
    let mut otherareanum: libc::c_int = 0;
    let mut gap: libc::c_int = 0;
    let mut reachareanum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face3: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut v1: *mut vec_t = 0 as *mut vec_t;
    let mut v2: *mut vec_t = 0 as *mut vec_t;
    let mut sharededgevec: vec3_t = [0.; 3];
    let mut mid: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut testend: vec3_t = [0.; 3];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    if 0 == AAS_AreaGrounded(areanum) || 0 != AAS_AreaSwim(areanum) { return }
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    i = 0i32;
    while i < (*area).numfaces {
        face1num =
            *aasworld.faceindex.offset(((*area).firstface + i) as isize);
        face1 =
            &mut *aasworld.faces.offset(abs(face1num) as isize) as
                *mut aas_face_t;
        //face 1 must be a ground face
        if !(0 == (*face1).faceflags & 4i32) {
            k = 0i32;
            while k < (*face1).numedges {
                edge1num =
                    *aasworld.edgeindex.offset(((*face1).firstedge + k) as
                                                   isize);
                j = 0i32;
                while j < (*area).numfaces {
                    face2num =
                        *aasworld.faceindex.offset(((*area).firstface + j) as
                                                       isize);
                    face2 =
                        &mut *aasworld.faces.offset(abs(face2num) as isize) as
                            *mut aas_face_t;
                    //face 2 may not be a ground face
                    if !(0 != (*face2).faceflags & 4i32) {
                        l = 0i32;
                        while l < (*face2).numedges {
                            edge2num =
                                *aasworld.edgeindex.offset(((*face2).firstedge
                                                                + l) as
                                                               isize);
                            if abs(edge1num) == abs(edge2num) {
                                if (*face2).frontarea == areanum {
                                    otherareanum = (*face2).backarea
                                } else { otherareanum = (*face2).frontarea }
                                area2 =
                                    &mut *aasworld.areas.offset(otherareanum
                                                                    as isize)
                                        as *mut aas_area_t;
                                //if the other area is grounded!
                                if 0 !=
                                       (*aasworld.areasettings.offset(otherareanum
                                                                          as
                                                                          isize)).areaflags
                                           & 1i32 {
                                    gap = qfalse as libc::c_int;
                                    n = 0i32;
                                    while n < (*area2).numfaces {
                                        face3num =
                                            *aasworld.faceindex.offset(((*area2).firstface
                                                                            +
                                                                            n)
                                                                           as
                                                                           isize);
                                        //may not be the shared face of the two areas
                                        if !(abs(face3num) == abs(face2num)) {
                                            face3 =
                                                &mut *aasworld.faces.offset(abs(face3num)
                                                                                as
                                                                                isize)
                                                    as *mut aas_face_t;
                                            m = 0i32;
                                            while m < (*face3).numedges {
                                                edge3num =
                                                    *aasworld.edgeindex.offset(((*face3).firstedge
                                                                                    +
                                                                                    m)
                                                                                   as
                                                                                   isize);
                                                //but the edge should be shared by all three faces
                                                if abs(edge3num) ==
                                                       abs(edge1num) {
                                                    if 0 ==
                                                           (*face3).faceflags
                                                               & 1i32 {
                                                        gap =
                                                            qtrue as
                                                                libc::c_int;
                                                        break ;
                                                    } else if 0 !=
                                                                  (*face3).faceflags
                                                                      & 4i32 {
                                                        gap =
                                                            qfalse as
                                                                libc::c_int;
                                                        break ;
                                                    } else {
                                                        gap =
                                                            qtrue as
                                                                libc::c_int;
                                                        break ;
                                                    }
                                                } else { m += 1 }
                                            }
                                            //end if
                                            //end for
                                            if m < (*face3).numedges {
                                                break ;
                                            }
                                        }
                                        n += 1
                                    }
                                    //end for
                                    if 0 == gap { break ; }
                                }
                                edge =
                                    &mut *aasworld.edges.offset(abs(edge1num)
                                                                    as isize)
                                        as *mut aas_edge_t;
                                side = (edge1num < 0i32) as libc::c_int;
                                v1 =
                                    (*aasworld.vertexes.offset((*edge).v[side
                                                                             as
                                                                             usize]
                                                                   as
                                                                   isize)).as_mut_ptr();
                                v2 =
                                    (*aasworld.vertexes.offset((*edge).v[(0 ==
                                                                              side)
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                                                   as
                                                                   isize)).as_mut_ptr();
                                plane =
                                    &mut *aasworld.planes.offset((*face1).planenum
                                                                     as isize)
                                        as *mut aas_plane_t;
                                sharededgevec[0usize] =
                                    *v2.offset(0isize) - *v1.offset(0isize);
                                sharededgevec[1usize] =
                                    *v2.offset(1isize) - *v1.offset(1isize);
                                sharededgevec[2usize] =
                                    *v2.offset(2isize) - *v1.offset(2isize);
                                CrossProduct((*plane).normal.as_mut_ptr() as
                                                 *const vec_t,
                                             sharededgevec.as_mut_ptr() as
                                                 *const vec_t,
                                             dir.as_mut_ptr());
                                VectorNormalize(dir.as_mut_ptr());
                                mid[0usize] =
                                    *v1.offset(0isize) + *v2.offset(0isize);
                                mid[1usize] =
                                    *v1.offset(1isize) + *v2.offset(1isize);
                                mid[2usize] =
                                    *v1.offset(2isize) + *v2.offset(2isize);
                                mid[0usize] =
                                    (mid[0usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                mid[1usize] =
                                    (mid[1usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                mid[2usize] =
                                    (mid[2usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                mid[0usize] =
                                    mid[0usize] +
                                        dir[0usize] * 8i32 as libc::c_float;
                                mid[1usize] =
                                    mid[1usize] +
                                        dir[1usize] * 8i32 as libc::c_float;
                                mid[2usize] =
                                    mid[2usize] +
                                        dir[2usize] * 8i32 as libc::c_float;
                                testend[0usize] = mid[0usize];
                                testend[1usize] = mid[1usize];
                                testend[2usize] = mid[2usize];
                                testend[2usize] -= 1000i32 as libc::c_float;
                                trace =
                                    AAS_TraceClientBBox(mid.as_mut_ptr(),
                                                        testend.as_mut_ptr(),
                                                        4i32, -1i32);
                                //
                                if 0 != trace.startsolid as u64 {
                                    //Log_Write("area %d: trace.startsolid\r\n", areanum);
                                    break ;
                                } else {
                                    reachareanum =
                                        AAS_PointAreaNum(trace.endpos.as_mut_ptr());
                                    if reachareanum == areanum {
                                        //Log_Write("area %d: same area\r\n", areanum);
                                        break ;
                                    } else if 0 !=
                                                  AAS_ReachabilityExists(areanum,
                                                                         reachareanum)
                                                      as u64 {
                                        //Log_Write("area %d: reachability already exists\r\n", areanum);
                                        break ;
                                    } else if 0 ==
                                                  AAS_AreaGrounded(reachareanum)
                                                  &&
                                                  0 ==
                                                      AAS_AreaSwim(reachareanum)
                                     {
                                        //Log_Write("area %d, reach area %d: not grounded and not swim\r\n", areanum, reachareanum);
                                        break ;
                                    } else if 0 !=
                                                  (*aasworld.areasettings.offset(reachareanum
                                                                                     as
                                                                                     isize)).contents
                                                      & (4i32 | 2i32) {
                                        //Log_Write("area %d, reach area %d: lava or slime\r\n", areanum, reachareanum);
                                        break ;
                                    } else {
                                        numareas =
                                            AAS_TraceAreas(mid.as_mut_ptr(),
                                                           testend.as_mut_ptr(),
                                                           areas.as_mut_ptr(),
                                                           0 as *mut vec3_t,
                                                           (::std::mem::size_of::<[libc::c_int; 10]>()
                                                                as
                                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                                                                as
                                                                                                libc::c_ulong)
                                                               as
                                                               libc::c_int);
                                        p = 0i32;
                                        while p < numareas {
                                            if 0 !=
                                                   AAS_AreaClusterPortal(areas[p
                                                                                   as
                                                                                   usize])
                                               {
                                                break ;
                                            }
                                            p += 1
                                        }
                                        if p < numareas { break ; }
                                        // if a maximum fall height is set and the bot would fall down further
                                        if 0. != aassettings.rs_maxfallheight
                                               &&
                                               fabs((mid[2usize] -
                                                         trace.endpos[2usize])
                                                        as libc::c_double) >
                                                   aassettings.rs_maxfallheight
                                                       as libc::c_double {
                                            break ;
                                        }
                                        lreach = AAS_AllocReachability();
                                        if lreach.is_null() { break ; }
                                        (*lreach).areanum = reachareanum;
                                        (*lreach).facenum = 0i32;
                                        (*lreach).edgenum = edge1num;
                                        (*lreach).start[0usize] = mid[0usize];
                                        (*lreach).start[1usize] = mid[1usize];
                                        (*lreach).start[2usize] = mid[2usize];
                                        (*lreach).end[0usize] =
                                            trace.endpos[0usize];
                                        (*lreach).end[1usize] =
                                            trace.endpos[1usize];
                                        (*lreach).end[2usize] =
                                            trace.endpos[2usize];
                                        (*lreach).traveltype = 7i32;
                                        (*lreach).traveltime =
                                            (aassettings.rs_startwalkoffledge
                                                 as libc::c_double +
                                                 fabs((mid[2usize] -
                                                           trace.endpos[2usize])
                                                          as libc::c_double) *
                                                     50i32 as libc::c_double /
                                                     aassettings.phys_gravity
                                                         as libc::c_double) as
                                                libc::c_ushort;
                                        if 0 == AAS_AreaSwim(reachareanum) &&
                                               0 ==
                                                   AAS_AreaJumpPad(reachareanum)
                                           {
                                            if AAS_FallDelta(mid[2usize] -
                                                                 trace.endpos[2usize])
                                                   >
                                                   aassettings.phys_falldelta5
                                               {
                                                (*lreach).traveltime =
                                                    ((*lreach).traveltime as
                                                         libc::c_float +
                                                         aassettings.rs_falldamage5)
                                                        as libc::c_ushort
                                            } else if AAS_FallDelta(mid[2usize]
                                                                        -
                                                                        trace.endpos[2usize])
                                                          >
                                                          aassettings.phys_falldelta10
                                             {
                                                (*lreach).traveltime =
                                                    ((*lreach).traveltime as
                                                         libc::c_float +
                                                         aassettings.rs_falldamage10)
                                                        as libc::c_ushort
                                            }
                                        }
                                        (*lreach).next =
                                            *areareachability.offset(areanum
                                                                         as
                                                                         isize);
                                        let ref mut fresh10 =
                                            *areareachability.offset(areanum
                                                                         as
                                                                         isize);
                                        *fresh10 = lreach;
                                        reach_walkoffledge += 1
                                    }
                                }
                            }
                            l += 1
                        }
                    }
                    j += 1
                }
                k += 1
            }
        }
        i += 1
    };
}
//walk of a ledge
#[no_mangle]
pub static mut reach_walkoffledge: libc::c_int = 0;
//end of the function AAS_FallDamageDistance
//===========================================================================
// distance = 0.5 * gravity * t * t
// vel = t * gravity
// damage = vel * vel * 0.0001
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FallDelta(mut distance: libc::c_float)
 -> libc::c_float {
    let mut t: libc::c_float = 0.;
    let mut delta: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    gravity = aassettings.phys_gravity;
    t =
        sqrt(fabs(distance as libc::c_double) * 2i32 as libc::c_double /
                 gravity as libc::c_double) as libc::c_float;
    delta = t * gravity;
    return ((delta * delta) as libc::c_double * 0.0001f64) as libc::c_float;
}
//end of the function AAS_AreaTeleporter
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AreaClusterPortal(mut areanum: libc::c_int)
 -> libc::c_int {
    return (*aasworld.areasettings.offset(areanum as isize)).contents & 8i32;
}
//end of the function AAS_SetWeaponJumpAreaFlags
//===========================================================================
// create a possible weapon jump reachability from area1 to area2
//
// check if there's a cool item in the second area
// check if area1 is lower than area2
// check if the bot can rocketjump from area1 to area2
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_WeaponJump(mut area1num:
                                                         libc::c_int,
                                                     mut area2num:
                                                         libc::c_int)
 -> libc::c_int {
    let mut face2num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut visualize: libc::c_int = 0;
    let mut speed: libc::c_float = 0.;
    let mut zvel: libc::c_float = 0.;
    //float hordist;
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    // teststart;
    let mut areastart: vec3_t = [0.; 3];
    let mut facecenter: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
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
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    visualize = qfalse as libc::c_int;
    if 0 == AAS_AreaGrounded(area1num) || 0 != AAS_AreaSwim(area1num) {
        return qfalse as libc::c_int
    }
    if 0 == AAS_AreaGrounded(area2num) { return qfalse as libc::c_int }
    if 0 ==
           (*aasworld.areasettings.offset(area2num as isize)).areaflags &
               8192i32 {
        return qfalse as libc::c_int
    }
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    if (*area2).maxs[2usize] < (*area1).mins[2usize] {
        return qfalse as libc::c_int
    }
    start[0usize] =
        (*aasworld.areas.offset(area1num as isize)).center[0usize];
    start[1usize] =
        (*aasworld.areas.offset(area1num as isize)).center[1usize];
    start[2usize] =
        (*aasworld.areas.offset(area1num as isize)).center[2usize];
    if 0 == AAS_PointAreaNum(start.as_mut_ptr()) {
        Log_Write(b"area %d center %f %f %f in solid?\r\n\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char, area1num,
                  start[0usize] as libc::c_double,
                  start[1usize] as libc::c_double,
                  start[2usize] as libc::c_double);
    }
    end[0usize] = start[0usize];
    end[1usize] = start[1usize];
    end[2usize] = start[2usize];
    end[2usize] -= 1000i32 as libc::c_float;
    trace =
        AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 4i32,
                            -1i32);
    if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
    areastart[0usize] = trace.endpos[0usize];
    areastart[1usize] = trace.endpos[1usize];
    areastart[2usize] = trace.endpos[2usize];
    i = 0i32;
    while i < (*area2).numfaces {
        face2num =
            *aasworld.faceindex.offset(((*area2).firstface + i) as isize);
        face2 =
            &mut *aasworld.faces.offset(abs(face2num) as isize) as
                *mut aas_face_t;
        //if it is not a solid face
        if !(0 == (*face2).faceflags & 4i32) {
            AAS_FaceCenter(face2num, facecenter.as_mut_ptr());
            //only go higher up with weapon jumps
            if !(facecenter[2usize] <
                     areastart[2usize] + 64i32 as libc::c_float) {
                n = 0i32;
                while n < 1i32 {
                    if 0 != n {
                        zvel = AAS_BFGJumpZVelocity(areastart.as_mut_ptr())
                    } else {
                        zvel = AAS_RocketJumpZVelocity(areastart.as_mut_ptr())
                    }
                    ret =
                        AAS_HorizontalVelocityForJump(zvel,
                                                      areastart.as_mut_ptr(),
                                                      facecenter.as_mut_ptr(),
                                                      &mut speed);
                    if 0 != ret && speed < 300i32 as libc::c_float {
                        dir[0usize] = facecenter[0usize] - areastart[0usize];
                        dir[1usize] = facecenter[1usize] - areastart[1usize];
                        dir[2usize] = facecenter[2usize] - areastart[2usize];
                        dir[2usize] = 0i32 as vec_t;
                        cmdmove[0usize] = dir[0usize] * speed;
                        cmdmove[1usize] = dir[1usize] * speed;
                        cmdmove[2usize] = dir[2usize] * speed;
                        velocity[0usize] = 0i32 as vec_t;
                        velocity[1usize] = 0i32 as vec_t;
                        velocity[2usize] = zvel;
                        AAS_PredictClientMovement(&mut move_0, -1i32,
                                                  areastart.as_mut_ptr(),
                                                  2i32, qtrue as libc::c_int,
                                                  velocity.as_mut_ptr(),
                                                  cmdmove.as_mut_ptr(), 30i32,
                                                  30i32, 0.1f32,
                                                  4i32 | 8i32 | 16i32 | 32i32
                                                      | 128i32 | 1i32 |
                                                      1024i32, area2num,
                                                  visualize);
                        if move_0.frames < 30i32 &&
                               0 == move_0.stopevent & (8i32 | 16i32 | 32i32)
                               && 0 != move_0.stopevent & (1024i32 | 128i32) {
                            lreach = AAS_AllocReachability();
                            if lreach.is_null() {
                                return qfalse as libc::c_int
                            }
                            (*lreach).areanum = area2num;
                            (*lreach).facenum = 0i32;
                            (*lreach).edgenum = 0i32;
                            (*lreach).start[0usize] = areastart[0usize];
                            (*lreach).start[1usize] = areastart[1usize];
                            (*lreach).start[2usize] = areastart[2usize];
                            (*lreach).end[0usize] = facecenter[0usize];
                            (*lreach).end[1usize] = facecenter[1usize];
                            (*lreach).end[2usize] = facecenter[2usize];
                            if 0 != n {
                                (*lreach).traveltype = 13i32;
                                (*lreach).traveltime =
                                    aassettings.rs_bfgjump as libc::c_ushort
                            } else {
                                (*lreach).traveltype = 12i32;
                                (*lreach).traveltime =
                                    aassettings.rs_rocketjump as
                                        libc::c_ushort
                            }
                            (*lreach).next =
                                *areareachability.offset(area1num as isize);
                            let ref mut fresh11 =
                                *areareachability.offset(area1num as isize);
                            *fresh11 = lreach;
                            reach_rocketjump += 1;
                            return qtrue as libc::c_int
                        }
                    }
                    n += 1
                }
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
//rocket jump
#[no_mangle]
pub static mut reach_rocketjump: libc::c_int = 0;
//end for
//end of the function AAS_Reachability_JumpPad
//===========================================================================
// never point at ground faces
// always a higher and pretty far area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Grapple(mut area1num: libc::c_int,
                                                  mut area2num: libc::c_int)
 -> libc::c_int {
    let mut face2num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 20] = [0; 20];
    let mut mingrappleangle: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut hordist: libc::c_float = 0.;
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
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut areastart: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut facecenter: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut down: vec3_t = [0i32 as vec_t, 0i32 as vec_t, -1i32 as vec_t];
    let mut v: *mut vec_t = 0 as *mut vec_t;
    if 0 == AAS_AreaGrounded(area1num) && 0 == AAS_AreaSwim(area1num) {
        return qfalse as libc::c_int
    }
    if 0 == AAS_AreaPresenceType(area1num) & 2i32 {
        return qfalse as libc::c_int
    }
    if 0 != AAS_AreaSwim(area1num) { return qfalse as libc::c_int }
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    if (*area2).maxs[2usize] < (*area1).mins[2usize] {
        return qfalse as libc::c_int
    }
    start[0usize] =
        (*aasworld.areas.offset(area1num as isize)).center[0usize];
    start[1usize] =
        (*aasworld.areas.offset(area1num as isize)).center[1usize];
    start[2usize] =
        (*aasworld.areas.offset(area1num as isize)).center[2usize];
    if 0 == AAS_AreaSwim(area1num) {
        if 0 == AAS_PointAreaNum(start.as_mut_ptr()) {
            Log_Write(b"area %d center %f %f %f in solid?\r\n\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, area1num,
                      start[0usize] as libc::c_double,
                      start[1usize] as libc::c_double,
                      start[2usize] as libc::c_double);
        }
        end[0usize] = start[0usize];
        end[1usize] = start[1usize];
        end[2usize] = start[2usize];
        end[2usize] -= 1000i32 as libc::c_float;
        trace =
            AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 4i32,
                                -1i32);
        if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
        areastart[0usize] = trace.endpos[0usize];
        areastart[1usize] = trace.endpos[1usize];
        areastart[2usize] = trace.endpos[2usize]
    } else if 0 ==
                  AAS_PointContents(start.as_mut_ptr()) &
                      (8i32 | 16i32 | 32i32) {
        return qfalse as libc::c_int
    }
    i = 0i32;
    while i < (*area2).numfaces {
        face2num =
            *aasworld.faceindex.offset(((*area2).firstface + i) as isize);
        face2 =
            &mut *aasworld.faces.offset(abs(face2num) as isize) as
                *mut aas_face_t;
        //if it is not a solid face
        if !(0 == (*face2).faceflags & 1i32) {
            v =
                (*aasworld.vertexes.offset((*aasworld.edges.offset(abs(*aasworld.edgeindex.offset((*face2).firstedge
                                                                                                      as
                                                                                                      isize))
                                                                       as
                                                                       isize)).v[0usize]
                                               as isize)).as_mut_ptr();
            dir[0usize] = *v.offset(0isize) - areastart[0usize];
            dir[1usize] = *v.offset(1isize) - areastart[1usize];
            dir[2usize] = *v.offset(2isize) - areastart[2usize];
            //if the face plane is facing away
            if !((*aasworld.planes.offset((*face2).planenum as
                                              isize)).normal[0usize] *
                     dir[0usize] +
                     (*aasworld.planes.offset((*face2).planenum as
                                                  isize)).normal[1usize] *
                         dir[1usize] +
                     (*aasworld.planes.offset((*face2).planenum as
                                                  isize)).normal[2usize] *
                         dir[2usize] > 0i32 as libc::c_float) {
                AAS_FaceCenter(face2num, facecenter.as_mut_ptr());
                //only go higher up with the grapple
                if !(facecenter[2usize] <
                         areastart[2usize] + 64i32 as libc::c_float) {
                    //only use vertical faces or downward facing faces
                    if !((*aasworld.planes.offset((*face2).planenum as
                                                      isize)).normal[0usize] *
                             down[0usize] +
                             (*aasworld.planes.offset((*face2).planenum as
                                                          isize)).normal[1usize]
                                 * down[1usize] +
                             (*aasworld.planes.offset((*face2).planenum as
                                                          isize)).normal[2usize]
                                 * down[2usize] < 0i32 as libc::c_float) {
                        dir[0usize] = facecenter[0usize] - areastart[0usize];
                        dir[1usize] = facecenter[1usize] - areastart[1usize];
                        dir[2usize] = facecenter[2usize] - areastart[2usize];
                        z = dir[2usize];
                        dir[2usize] = 0i32 as vec_t;
                        hordist =
                            VectorLength(dir.as_mut_ptr() as *const vec_t);
                        if !(0. == hordist) {
                            //if too far
                            if !(hordist > 2000i32 as libc::c_float) {
                                mingrappleangle = 15i32 as libc::c_float;
                                if !(((z / hordist) as libc::c_double) <
                                         tan(2i32 as libc::c_double *
                                                 3.14159265358979323846f64 *
                                                 mingrappleangle as
                                                     libc::c_double /
                                                 360i32 as libc::c_double)) {
                                    start[0usize] = facecenter[0usize];
                                    start[1usize] = facecenter[1usize];
                                    start[2usize] = facecenter[2usize];
                                    end[0usize] =
                                        facecenter[0usize] +
                                            (*aasworld.planes.offset((*face2).planenum
                                                                         as
                                                                         isize)).normal[0usize]
                                                * -500i32 as libc::c_float;
                                    end[1usize] =
                                        facecenter[1usize] +
                                            (*aasworld.planes.offset((*face2).planenum
                                                                         as
                                                                         isize)).normal[1usize]
                                                * -500i32 as libc::c_float;
                                    end[2usize] =
                                        facecenter[2usize] +
                                            (*aasworld.planes.offset((*face2).planenum
                                                                         as
                                                                         isize)).normal[2usize]
                                                * -500i32 as libc::c_float;
                                    bsptrace =
                                        AAS_Trace(start.as_mut_ptr(),
                                                  0 as *mut vec_t,
                                                  0 as *mut vec_t,
                                                  end.as_mut_ptr(), 0i32,
                                                  1i32);
                                    //the grapple won't stick to the sky and the grapple point should be near the AAS wall
                                    if !(0 != bsptrace.surface.flags & 0x4i32
                                             ||
                                             bsptrace.fraction *
                                                 500i32 as libc::c_float >
                                                 32i32 as libc::c_float) {
                                        dir[0usize] =
                                            facecenter[0usize] -
                                                areastart[0usize];
                                        dir[1usize] =
                                            facecenter[1usize] -
                                                areastart[1usize];
                                        dir[2usize] =
                                            facecenter[2usize] -
                                                areastart[2usize];
                                        VectorNormalize(dir.as_mut_ptr());
                                        start[0usize] =
                                            areastart[0usize] +
                                                dir[0usize] *
                                                    4i32 as libc::c_float;
                                        start[1usize] =
                                            areastart[1usize] +
                                                dir[1usize] *
                                                    4i32 as libc::c_float;
                                        start[2usize] =
                                            areastart[2usize] +
                                                dir[2usize] *
                                                    4i32 as libc::c_float;
                                        end[0usize] = bsptrace.endpos[0usize];
                                        end[1usize] = bsptrace.endpos[1usize];
                                        end[2usize] = bsptrace.endpos[2usize];
                                        trace =
                                            AAS_TraceClientBBox(start.as_mut_ptr(),
                                                                end.as_mut_ptr(),
                                                                2i32, -1i32);
                                        dir[0usize] =
                                            trace.endpos[0usize] -
                                                facecenter[0usize];
                                        dir[1usize] =
                                            trace.endpos[1usize] -
                                                facecenter[1usize];
                                        dir[2usize] =
                                            trace.endpos[2usize] -
                                                facecenter[2usize];
                                        if !(VectorLength(dir.as_mut_ptr() as
                                                              *const vec_t) >
                                                 24i32 as libc::c_float) {
                                            start[0usize] =
                                                trace.endpos[0usize];
                                            start[1usize] =
                                                trace.endpos[1usize];
                                            start[2usize] =
                                                trace.endpos[2usize];
                                            end[0usize] =
                                                trace.endpos[0usize];
                                            end[1usize] =
                                                trace.endpos[1usize];
                                            end[2usize] =
                                                trace.endpos[2usize];
                                            end[2usize] -=
                                                AAS_FallDamageDistance() as
                                                    libc::c_float;
                                            trace =
                                                AAS_TraceClientBBox(start.as_mut_ptr(),
                                                                    end.as_mut_ptr(),
                                                                    2i32,
                                                                    -1i32);
                                            if !(trace.fraction >=
                                                     1i32 as libc::c_float) {
                                                areanum =
                                                    AAS_PointAreaNum(trace.endpos.as_mut_ptr());
                                                //if not in lava or slime
                                                if !(0 !=
                                                         (*aasworld.areasettings.offset(areanum
                                                                                            as
                                                                                            isize)).contents
                                                             & (4i32 | 2i32))
                                                   {
                                                    //end if
                                                    //do not go the the source area
                                                    if !(areanum == area1num)
                                                       {
                                                        //don't create reachabilities if they already exist
                                                        if !(0 !=
                                                                 AAS_ReachabilityExists(area1num,
                                                                                        areanum)
                                                                     as u64) {
                                                            //only end in areas we can stand
                                                            if !(0 ==
                                                                     AAS_AreaGrounded(areanum))
                                                               {
                                                                numareas =
                                                                    AAS_TraceAreas(areastart.as_mut_ptr(),
                                                                                   bsptrace.endpos.as_mut_ptr(),
                                                                                   areas.as_mut_ptr(),
                                                                                   0
                                                                                       as
                                                                                       *mut vec3_t,
                                                                                   20i32);
                                                                if !(numareas
                                                                         >=
                                                                         20i32)
                                                                   {
                                                                    j = 0i32;
                                                                    while j <
                                                                              numareas
                                                                          {
                                                                        if 0
                                                                               !=
                                                                               (*aasworld.areasettings.offset(areas[j
                                                                                                                        as
                                                                                                                        usize]
                                                                                                                  as
                                                                                                                  isize)).contents
                                                                                   &
                                                                                   8i32
                                                                           {
                                                                            break
                                                                                ;
                                                                        }
                                                                        j += 1
                                                                    }
                                                                    //end for
                                                                    if !(j <
                                                                             numareas)
                                                                       {
                                                                        lreach
                                                                            =
                                                                            AAS_AllocReachability();
                                                                        if lreach.is_null()
                                                                           {
                                                                            return qfalse
                                                                                       as
                                                                                       libc::c_int
                                                                        }
                                                                        (*lreach).areanum
                                                                            =
                                                                            areanum;
                                                                        (*lreach).facenum
                                                                            =
                                                                            face2num;
                                                                        (*lreach).edgenum
                                                                            =
                                                                            0i32;
                                                                        (*lreach).start[0usize]
                                                                            =
                                                                            areastart[0usize];
                                                                        (*lreach).start[1usize]
                                                                            =
                                                                            areastart[1usize];
                                                                        (*lreach).start[2usize]
                                                                            =
                                                                            areastart[2usize];
                                                                        (*lreach).end[0usize]
                                                                            =
                                                                            bsptrace.endpos[0usize];
                                                                        (*lreach).end[1usize]
                                                                            =
                                                                            bsptrace.endpos[1usize];
                                                                        (*lreach).end[2usize]
                                                                            =
                                                                            bsptrace.endpos[2usize];
                                                                        (*lreach).traveltype
                                                                            =
                                                                            14i32;
                                                                        dir[0usize]
                                                                            =
                                                                            (*lreach).end[0usize]
                                                                                -
                                                                                (*lreach).start[0usize];
                                                                        dir[1usize]
                                                                            =
                                                                            (*lreach).end[1usize]
                                                                                -
                                                                                (*lreach).start[1usize];
                                                                        dir[2usize]
                                                                            =
                                                                            (*lreach).end[2usize]
                                                                                -
                                                                                (*lreach).start[2usize];
                                                                        (*lreach).traveltime
                                                                            =
                                                                            (aassettings.rs_startgrapple
                                                                                 as
                                                                                 libc::c_double
                                                                                 +
                                                                                 VectorLength(dir.as_mut_ptr()
                                                                                                  as
                                                                                                  *const vec_t)
                                                                                     as
                                                                                     libc::c_double
                                                                                     *
                                                                                     0.25f64)
                                                                                as
                                                                                libc::c_ushort;
                                                                        (*lreach).next
                                                                            =
                                                                            *areareachability.offset(area1num
                                                                                                         as
                                                                                                         isize);
                                                                        let ref mut fresh12 =
                                                                            *areareachability.offset(area1num
                                                                                                         as
                                                                                                         isize);
                                                                        *fresh12
                                                                            =
                                                                            lreach;
                                                                        reach_grapple
                                                                            +=
                                                                            1
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
//grapple hook
#[no_mangle]
pub static mut reach_grapple: libc::c_int = 0;
//end of the function AAS_FaceCenter
//===========================================================================
// returns the maximum distance a player can fall before being damaged
// damage = deltavelocity*deltavelocity  * 0.0001
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FallDamageDistance() -> libc::c_int {
    let mut maxzvelocity: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    maxzvelocity =
        sqrt((30i32 * 10000i32) as libc::c_double) as libc::c_float;
    gravity = aassettings.phys_gravity;
    t = maxzvelocity / gravity;
    return (0.5f64 * gravity as libc::c_double * t as libc::c_double *
                t as libc::c_double) as libc::c_int;
}
//end of the function AAS_ClosestEdgePoints
//===========================================================================
// creates possible jump reachabilities between the areas
//
// The two closest points on the ground of the areas are calculated
// One of the points will be on an edge of a ground face of area1 and
// one on an edge of a ground face of area2.
// If there is a range of closest points the point in the middle of this range
// is selected.
// Between these two points there must be one or more gaps.
// If the gaps exist a potential jump is predicted.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Jump(mut area1num: libc::c_int,
                                               mut area2num: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut traveltype: libc::c_int = 0;
    let mut stopevent: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut phys_jumpvel: libc::c_float = 0.;
    let mut maxjumpdistance: libc::c_float = 0.;
    let mut maxjumpheight: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut v1: *mut vec_t = 0 as *mut vec_t;
    let mut v2: *mut vec_t = 0 as *mut vec_t;
    let mut v3: *mut vec_t = 0 as *mut vec_t;
    let mut v4: *mut vec_t = 0 as *mut vec_t;
    let mut beststart: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut beststart2: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut bestend: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut bestend2: vec3_t = [0i32 as vec_t, 0., 0.];
    let mut teststart: vec3_t = [0.; 3];
    let mut testend: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut sidewards: vec3_t = [0.; 3];
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge1: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut edge2: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut plane1: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut plane2: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
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
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    if 0 == AAS_AreaGrounded(area1num) || 0 == AAS_AreaGrounded(area2num) {
        return qfalse as libc::c_int
    }
    if 0 != AAS_AreaCrouch(area1num) || 0 != AAS_AreaCrouch(area2num) {
        return qfalse as libc::c_int
    }
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    phys_jumpvel = aassettings.phys_jumpvel;
    maxjumpdistance =
        2i32 as libc::c_float * AAS_MaxJumpDistance(phys_jumpvel);
    maxjumpheight = AAS_MaxJumpHeight(phys_jumpvel);
    i = 0i32;
    while i < 2i32 {
        if (*area1).mins[i as usize] >
               (*area2).maxs[i as usize] + maxjumpdistance {
            return qfalse as libc::c_int
        }
        if (*area1).maxs[i as usize] <
               (*area2).mins[i as usize] - maxjumpdistance {
            return qfalse as libc::c_int
        }
        i += 1
    }
    if (*area2).mins[2usize] > (*area1).maxs[2usize] + maxjumpheight {
        return qfalse as libc::c_int
    }
    bestdist = 999999i32 as libc::c_float;
    i = 0i32;
    while i < (*area1).numfaces {
        face1num =
            *aasworld.faceindex.offset(((*area1).firstface + i) as isize);
        face1 =
            &mut *aasworld.faces.offset(abs(face1num) as isize) as
                *mut aas_face_t;
        //if not a ground face
        if !(0 == (*face1).faceflags & 4i32) {
            j = 0i32;
            while j < (*area2).numfaces {
                face2num =
                    *aasworld.faceindex.offset(((*area2).firstface + j) as
                                                   isize);
                face2 =
                    &mut *aasworld.faces.offset(abs(face2num) as isize) as
                        *mut aas_face_t;
                //if not a ground face
                if !(0 == (*face2).faceflags & 4i32) {
                    k = 0i32;
                    while k < (*face1).numedges {
                        edge1num =
                            abs(*aasworld.edgeindex.offset(((*face1).firstedge
                                                                + k) as
                                                               isize));
                        edge1 =
                            &mut *aasworld.edges.offset(edge1num as isize) as
                                *mut aas_edge_t;
                        l = 0i32;
                        while l < (*face2).numedges {
                            edge2num =
                                abs(*aasworld.edgeindex.offset(((*face2).firstedge
                                                                    + l) as
                                                                   isize));
                            edge2 =
                                &mut *aasworld.edges.offset(edge2num as isize)
                                    as *mut aas_edge_t;
                            v1 =
                                (*aasworld.vertexes.offset((*edge1).v[0usize]
                                                               as
                                                               isize)).as_mut_ptr();
                            v2 =
                                (*aasworld.vertexes.offset((*edge1).v[1usize]
                                                               as
                                                               isize)).as_mut_ptr();
                            v3 =
                                (*aasworld.vertexes.offset((*edge2).v[0usize]
                                                               as
                                                               isize)).as_mut_ptr();
                            v4 =
                                (*aasworld.vertexes.offset((*edge2).v[1usize]
                                                               as
                                                               isize)).as_mut_ptr();
                            plane1 =
                                &mut *aasworld.planes.offset((*face1).planenum
                                                                 as isize) as
                                    *mut aas_plane_t;
                            plane2 =
                                &mut *aasworld.planes.offset((*face2).planenum
                                                                 as isize) as
                                    *mut aas_plane_t;
                            bestdist =
                                AAS_ClosestEdgePoints(v1, v2, v3, v4, plane1,
                                                      plane2,
                                                      beststart.as_mut_ptr(),
                                                      bestend.as_mut_ptr(),
                                                      beststart2.as_mut_ptr(),
                                                      bestend2.as_mut_ptr(),
                                                      bestdist);
                            l += 1
                        }
                        k += 1
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    VectorMiddle(beststart.as_mut_ptr(), beststart2.as_mut_ptr(),
                 beststart.as_mut_ptr());
    VectorMiddle(bestend.as_mut_ptr(), bestend2.as_mut_ptr(),
                 bestend.as_mut_ptr());
    if bestdist > 4i32 as libc::c_float && bestdist < maxjumpdistance {
        if bestdist <= 48i32 as libc::c_float &&
               fabs((beststart[2usize] - bestend[2usize]) as libc::c_double) <
                   8i32 as libc::c_double {
            speed = 400i32 as libc::c_float;
            traveltype = 7i32
        } else if 0 !=
                      AAS_HorizontalVelocityForJump(0i32 as libc::c_float,
                                                    beststart.as_mut_ptr(),
                                                    bestend.as_mut_ptr(),
                                                    &mut speed) {
            speed *= 1.2f32;
            traveltype = 7i32
        } else {
            if 0 ==
                   AAS_HorizontalVelocityForJump(phys_jumpvel,
                                                 beststart.as_mut_ptr(),
                                                 bestend.as_mut_ptr(),
                                                 &mut speed) {
                return qfalse as libc::c_int
            }
            speed *= 1.05f32;
            traveltype = 5i32;
            dir[0usize] = bestend[0usize] - beststart[0usize];
            dir[1usize] = bestend[1usize] - beststart[1usize];
            dir[2usize] = bestend[2usize] - beststart[2usize];
            dir[2usize] = 0i32 as vec_t;
            if VectorLength(dir.as_mut_ptr() as *const vec_t) <
                   10i32 as libc::c_float {
                return qfalse as libc::c_int
            }
        }
        dir[0usize] = bestend[0usize] - beststart[0usize];
        dir[1usize] = bestend[1usize] - beststart[1usize];
        dir[2usize] = bestend[2usize] - beststart[2usize];
        VectorNormalize(dir.as_mut_ptr());
        teststart[0usize] =
            beststart[0usize] + dir[0usize] * 1i32 as libc::c_float;
        teststart[1usize] =
            beststart[1usize] + dir[1usize] * 1i32 as libc::c_float;
        teststart[2usize] =
            beststart[2usize] + dir[2usize] * 1i32 as libc::c_float;
        testend[0usize] = teststart[0usize];
        testend[1usize] = teststart[1usize];
        testend[2usize] = teststart[2usize];
        testend[2usize] -= 100i32 as libc::c_float;
        trace =
            AAS_TraceClientBBox(teststart.as_mut_ptr(), testend.as_mut_ptr(),
                                2i32, -1i32);
        if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
        if trace.fraction < 1i32 as libc::c_float {
            plane =
                &mut *aasworld.planes.offset(trace.planenum as isize) as
                    *mut aas_plane_t;
            if ((*plane).normal[0usize] * up[0usize] +
                    (*plane).normal[1usize] * up[1usize] +
                    (*plane).normal[2usize] * up[2usize]) as libc::c_double >=
                   0.7f64 {
                if 0 ==
                       AAS_PointContents(trace.endpos.as_mut_ptr()) &
                           (8i32 | 16i32) {
                    if teststart[2usize] - trace.endpos[2usize] <=
                           aassettings.phys_maxbarrier {
                        return qfalse as libc::c_int
                    }
                }
            }
        }
        teststart[0usize] =
            bestend[0usize] + dir[0usize] * -1i32 as libc::c_float;
        teststart[1usize] =
            bestend[1usize] + dir[1usize] * -1i32 as libc::c_float;
        teststart[2usize] =
            bestend[2usize] + dir[2usize] * -1i32 as libc::c_float;
        testend[0usize] = teststart[0usize];
        testend[1usize] = teststart[1usize];
        testend[2usize] = teststart[2usize];
        testend[2usize] -= 100i32 as libc::c_float;
        trace =
            AAS_TraceClientBBox(teststart.as_mut_ptr(), testend.as_mut_ptr(),
                                2i32, -1i32);
        if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
        if trace.fraction < 1i32 as libc::c_float {
            plane =
                &mut *aasworld.planes.offset(trace.planenum as isize) as
                    *mut aas_plane_t;
            if ((*plane).normal[0usize] * up[0usize] +
                    (*plane).normal[1usize] * up[1usize] +
                    (*plane).normal[2usize] * up[2usize]) as libc::c_double >=
                   0.7f64 {
                if 0 ==
                       AAS_PointContents(trace.endpos.as_mut_ptr()) &
                           (8i32 | 16i32) {
                    if teststart[2usize] - trace.endpos[2usize] <=
                           aassettings.phys_maxbarrier {
                        return qfalse as libc::c_int
                    }
                }
            }
        }
        cmdmove[2usize] = 0i32 as vec_t;
        cmdmove[1usize] = cmdmove[2usize];
        cmdmove[0usize] = cmdmove[1usize];
        if traveltype & 0xffffffi32 == 5i32 {
            cmdmove[2usize] = aassettings.phys_jumpvel
        } else { cmdmove[2usize] = 0i32 as vec_t }
        dir[0usize] = bestend[0usize] - beststart[0usize];
        dir[1usize] = bestend[1usize] - beststart[1usize];
        dir[2usize] = bestend[2usize] - beststart[2usize];
        dir[2usize] = 0i32 as vec_t;
        VectorNormalize(dir.as_mut_ptr());
        CrossProduct(dir.as_mut_ptr() as *const vec_t,
                     up.as_mut_ptr() as *const vec_t, sidewards.as_mut_ptr());
        stopevent = 1i32 | 4i32 | 8i32 | 16i32 | 32i32;
        if 0 == AAS_AreaClusterPortal(area1num) &&
               0 == AAS_AreaClusterPortal(area2num) {
            stopevent |= 4096i32
        }
        i = 0i32;
        while i < 3i32 {
            if i == 1i32 {
                testend[0usize] = testend[0usize] + sidewards[0usize];
                testend[1usize] = testend[1usize] + sidewards[1usize];
                testend[2usize] = testend[2usize] + sidewards[2usize]
            } else if i == 2i32 {
                testend[0usize] = bestend[0usize] - sidewards[0usize];
                testend[1usize] = bestend[1usize] - sidewards[1usize];
                testend[2usize] = bestend[2usize] - sidewards[2usize]
            } else {
                testend[0usize] = bestend[0usize];
                testend[1usize] = bestend[1usize];
                testend[2usize] = bestend[2usize]
            }
            dir[0usize] = testend[0usize] - beststart[0usize];
            dir[1usize] = testend[1usize] - beststart[1usize];
            dir[2usize] = testend[2usize] - beststart[2usize];
            dir[2usize] = 0i32 as vec_t;
            VectorNormalize(dir.as_mut_ptr());
            velocity[0usize] = dir[0usize] * speed;
            velocity[1usize] = dir[1usize] * speed;
            velocity[2usize] = dir[2usize] * speed;
            AAS_PredictClientMovement(&mut move_0, -1i32,
                                      beststart.as_mut_ptr(), 2i32,
                                      qtrue as libc::c_int,
                                      velocity.as_mut_ptr(),
                                      cmdmove.as_mut_ptr(), 3i32, 30i32,
                                      0.1f32, stopevent, 0i32,
                                      qfalse as libc::c_int);
            if move_0.frames >= 30i32 { return qfalse as libc::c_int }
            if 0 != move_0.stopevent & (8i32 | 16i32) {
                return qfalse as libc::c_int
            }
            if 0 != move_0.stopevent & 4096i32 {
                return qfalse as libc::c_int
            }
            teststart[0usize] =
                move_0.endpos[0usize] + dir[0usize] * -64i32 as libc::c_float;
            teststart[1usize] =
                move_0.endpos[1usize] + dir[1usize] * -64i32 as libc::c_float;
            teststart[2usize] =
                move_0.endpos[2usize] + dir[2usize] * -64i32 as libc::c_float;
            teststart[2usize] += 1i32 as libc::c_float;
            numareas =
                AAS_TraceAreas(move_0.endpos.as_mut_ptr(),
                               teststart.as_mut_ptr(), areas.as_mut_ptr(),
                               0 as *mut vec3_t,
                               (::std::mem::size_of::<[libc::c_int; 10]>() as
                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                                    as
                                                                    libc::c_ulong)
                                   as libc::c_int);
            j = 0i32;
            while j < numareas {
                if areas[j as usize] == area2num { break ; }
                j += 1
            }
            //end for
            if j < numareas { break ; }
            i += 1
        }
        if i >= 3i32 { return qfalse as libc::c_int }
        lreach = AAS_AllocReachability();
        if lreach.is_null() { return qfalse as libc::c_int }
        (*lreach).areanum = area2num;
        (*lreach).facenum = 0i32;
        (*lreach).edgenum = 0i32;
        (*lreach).start[0usize] = beststart[0usize];
        (*lreach).start[1usize] = beststart[1usize];
        (*lreach).start[2usize] = beststart[2usize];
        (*lreach).end[0usize] = bestend[0usize];
        (*lreach).end[1usize] = bestend[1usize];
        (*lreach).end[2usize] = bestend[2usize];
        (*lreach).traveltype = traveltype;
        dir[0usize] = bestend[0usize] - beststart[0usize];
        dir[1usize] = bestend[1usize] - beststart[1usize];
        dir[2usize] = bestend[2usize] - beststart[2usize];
        height = dir[2usize];
        dir[2usize] = 0i32 as vec_t;
        if traveltype & 0xffffffi32 == 7i32 &&
               height > VectorLength(dir.as_mut_ptr() as *const vec_t) {
            (*lreach).traveltime =
                (aassettings.rs_startwalkoffledge +
                     height * 50i32 as libc::c_float /
                         aassettings.phys_gravity) as libc::c_ushort
        } else {
            (*lreach).traveltime =
                (aassettings.rs_startjump +
                     VectorDistance(bestend.as_mut_ptr(),
                                    beststart.as_mut_ptr()) *
                         240i32 as libc::c_float /
                         aassettings.phys_maxwalkvelocity) as libc::c_ushort
        }
        if 0 == AAS_AreaJumpPad(area2num) {
            if AAS_FallDelta(beststart[2usize] - bestend[2usize]) >
                   aassettings.phys_falldelta5 {
                (*lreach).traveltime =
                    ((*lreach).traveltime as libc::c_float +
                         aassettings.rs_falldamage5) as libc::c_ushort
            } else if AAS_FallDelta(beststart[2usize] - bestend[2usize]) >
                          aassettings.phys_falldelta10 {
                (*lreach).traveltime =
                    ((*lreach).traveltime as libc::c_float +
                         aassettings.rs_falldamage10) as libc::c_ushort
            }
        }
        (*lreach).next = *areareachability.offset(area1num as isize);
        let ref mut fresh13 = *areareachability.offset(area1num as isize);
        *fresh13 = lreach;
        if traveltype & 0xffffffi32 == 5i32 {
            reach_jump += 1
        } else { reach_walkoffledge += 1 }
    }
    return qfalse as libc::c_int;
}
//jump
#[no_mangle]
pub static mut reach_jump: libc::c_int = 0;
//end of the function AAS_FallDelta
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_MaxJumpHeight(mut phys_jumpvel: libc::c_float)
 -> libc::c_float {
    let mut phys_gravity: libc::c_float = 0.;
    phys_gravity = aassettings.phys_gravity;
    return (0.5f64 * phys_gravity as libc::c_double *
                (phys_jumpvel / phys_gravity) as libc::c_double *
                (phys_jumpvel / phys_gravity) as libc::c_double) as
               libc::c_float;
}
//end of the function AAS_Reachability_Jump
//===========================================================================
// create a possible ladder reachability from area1 to area2
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Ladder(mut area1num: libc::c_int,
                                                 mut area2num: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut sharededgenum: libc::c_int = 0i32;
    let mut lowestedgenum: libc::c_int = 0i32;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut ladderface1num: libc::c_int = 0i32;
    let mut ladderface2num: libc::c_int = 0i32;
    let mut ladderface1vertical: libc::c_int = 0;
    let mut ladderface2vertical: libc::c_int = 0;
    let mut firstv: libc::c_int = 0;
    let mut face1area: libc::c_float = 0.;
    let mut face2area: libc::c_float = 0.;
    let mut bestface1area: libc::c_float = -9999i32 as libc::c_float;
    let mut bestface2area: libc::c_float = -9999i32 as libc::c_float;
    let mut phys_jumpvel: libc::c_float = 0.;
    let mut maxjumpheight: libc::c_float = 0.;
    let mut area1point: vec3_t = [0.; 3];
    let mut area2point: vec3_t = [0.; 3];
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut mid: vec3_t = [0.; 3];
    let mut lowestpoint: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0.];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut sharededgevec: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut ladderface1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut ladderface2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut plane1: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut plane2: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut sharededge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut edge1: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    if 0 == AAS_AreaLadder(area1num) || 0 == AAS_AreaLadder(area2num) {
        return qfalse as libc::c_int
    }
    phys_jumpvel = aassettings.phys_jumpvel;
    maxjumpheight = AAS_MaxJumpHeight(phys_jumpvel);
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    i = 0i32;
    while i < (*area1).numfaces {
        face1num =
            *aasworld.faceindex.offset(((*area1).firstface + i) as isize);
        face1 =
            &mut *aasworld.faces.offset(abs(face1num) as isize) as
                *mut aas_face_t;
        //if not a ladder face
        if !(0 == (*face1).faceflags & 2i32) {
            j = 0i32;
            while j < (*area2).numfaces {
                face2num =
                    *aasworld.faceindex.offset(((*area2).firstface + j) as
                                                   isize);
                face2 =
                    &mut *aasworld.faces.offset(abs(face2num) as isize) as
                        *mut aas_face_t;
                //if not a ladder face
                if !(0 == (*face2).faceflags & 2i32) {
                    k = 0i32;
                    while k < (*face1).numedges {
                        edge1num =
                            *aasworld.edgeindex.offset(((*face1).firstedge +
                                                            k) as isize);
                        l = 0i32;
                        while l < (*face2).numedges {
                            edge2num =
                                *aasworld.edgeindex.offset(((*face2).firstedge
                                                                + l) as
                                                               isize);
                            if abs(edge1num) == abs(edge2num) {
                                face1area = AAS_FaceArea(face1);
                                face2area = AAS_FaceArea(face2);
                                if face1area > bestface1area &&
                                       face2area > bestface2area {
                                    bestface1area = face1area;
                                    bestface2area = face2area;
                                    ladderface1 = face1;
                                    ladderface2 = face2;
                                    ladderface1num = face1num;
                                    ladderface2num = face2num;
                                    sharededgenum = edge1num
                                }
                                //end if
                                break ;
                            } else { l += 1 }
                        }
                        //end if
                        //end for
                        if l != (*face2).numedges { break ; }
                        k += 1
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    if !ladderface1.is_null() && !ladderface2.is_null() {
        sharededge =
            &mut *aasworld.edges.offset(abs(sharededgenum) as isize) as
                *mut aas_edge_t;
        firstv = (sharededgenum < 0i32) as libc::c_int;
        v1[0usize] =
            (*aasworld.vertexes.offset((*sharededge).v[firstv as usize] as
                                           isize))[0usize];
        v1[1usize] =
            (*aasworld.vertexes.offset((*sharededge).v[firstv as usize] as
                                           isize))[1usize];
        v1[2usize] =
            (*aasworld.vertexes.offset((*sharededge).v[firstv as usize] as
                                           isize))[2usize];
        v2[0usize] =
            (*aasworld.vertexes.offset((*sharededge).v[(0 == firstv) as
                                                           libc::c_int as
                                                           usize] as
                                           isize))[0usize];
        v2[1usize] =
            (*aasworld.vertexes.offset((*sharededge).v[(0 == firstv) as
                                                           libc::c_int as
                                                           usize] as
                                           isize))[1usize];
        v2[2usize] =
            (*aasworld.vertexes.offset((*sharededge).v[(0 == firstv) as
                                                           libc::c_int as
                                                           usize] as
                                           isize))[2usize];
        area1point[0usize] = v1[0usize] + v2[0usize];
        area1point[1usize] = v1[1usize] + v2[1usize];
        area1point[2usize] = v1[2usize] + v2[2usize];
        area1point[0usize] =
            (area1point[0usize] as libc::c_double * 0.5f64) as vec_t;
        area1point[1usize] =
            (area1point[1usize] as libc::c_double * 0.5f64) as vec_t;
        area1point[2usize] =
            (area1point[2usize] as libc::c_double * 0.5f64) as vec_t;
        area2point[0usize] = area1point[0usize];
        area2point[1usize] = area1point[1usize];
        area2point[2usize] = area1point[2usize];
        plane1 =
            &mut *aasworld.planes.offset(((*ladderface1).planenum ^
                                              (ladderface1num < 0i32) as
                                                  libc::c_int) as isize) as
                *mut aas_plane_t;
        plane2 =
            &mut *aasworld.planes.offset(((*ladderface2).planenum ^
                                              (ladderface2num < 0i32) as
                                                  libc::c_int) as isize) as
                *mut aas_plane_t;
        sharededgevec[0usize] = v2[0usize] - v1[0usize];
        sharededgevec[1usize] = v2[1usize] - v1[1usize];
        sharededgevec[2usize] = v2[2usize] - v1[2usize];
        CrossProduct((*plane1).normal.as_mut_ptr() as *const vec_t,
                     sharededgevec.as_mut_ptr() as *const vec_t,
                     dir.as_mut_ptr());
        VectorNormalize(dir.as_mut_ptr());
        area1point[0usize] =
            area1point[0usize] + dir[0usize] * -32i32 as libc::c_float;
        area1point[1usize] =
            area1point[1usize] + dir[1usize] * -32i32 as libc::c_float;
        area1point[2usize] =
            area1point[2usize] + dir[2usize] * -32i32 as libc::c_float;
        area2point[0usize] =
            area2point[0usize] + dir[0usize] * 32i32 as libc::c_float;
        area2point[1usize] =
            area2point[1usize] + dir[1usize] * 32i32 as libc::c_float;
        area2point[2usize] =
            area2point[2usize] + dir[2usize] * 32i32 as libc::c_float;
        ladderface1vertical =
            ((fabsf((*plane1).normal[0usize] * up[0usize] +
                        (*plane1).normal[1usize] * up[1usize] +
                        (*plane1).normal[2usize] * up[2usize]) as
                  libc::c_double) < 0.1f64) as libc::c_int;
        ladderface2vertical =
            ((fabsf((*plane2).normal[0usize] * up[0usize] +
                        (*plane2).normal[1usize] * up[1usize] +
                        (*plane2).normal[2usize] * up[2usize]) as
                  libc::c_double) < 0.1f64) as libc::c_int;
        if 0 == ladderface1vertical && 0 == ladderface2vertical {
            return qfalse as libc::c_int
        }
        if 0 != ladderface1vertical && 0 != ladderface2vertical &&
               ((*plane1).normal[0usize] * (*plane2).normal[0usize] +
                    (*plane1).normal[1usize] * (*plane2).normal[1usize] +
                    (*plane1).normal[2usize] * (*plane2).normal[2usize]) as
                   libc::c_double > 0.7f64 &&
               (fabsf(sharededgevec[0usize] * up[0usize] +
                          sharededgevec[1usize] * up[1usize] +
                          sharededgevec[2usize] * up[2usize]) as
                    libc::c_double) < 0.7f64 {
            lreach = AAS_AllocReachability();
            if lreach.is_null() { return qfalse as libc::c_int }
            (*lreach).areanum = area2num;
            (*lreach).facenum = ladderface1num;
            (*lreach).edgenum = abs(sharededgenum);
            (*lreach).start[0usize] = area1point[0usize];
            (*lreach).start[1usize] = area1point[1usize];
            (*lreach).start[2usize] = area1point[2usize];
            (*lreach).end[0usize] =
                area2point[0usize] +
                    (*plane1).normal[0usize] * -3i32 as libc::c_float;
            (*lreach).end[1usize] =
                area2point[1usize] +
                    (*plane1).normal[1usize] * -3i32 as libc::c_float;
            (*lreach).end[2usize] =
                area2point[2usize] +
                    (*plane1).normal[2usize] * -3i32 as libc::c_float;
            (*lreach).traveltype = 6i32;
            (*lreach).traveltime = 10i32 as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area1num as isize);
            let ref mut fresh14 = *areareachability.offset(area1num as isize);
            *fresh14 = lreach;
            reach_ladder += 1;
            lreach = AAS_AllocReachability();
            if lreach.is_null() { return qfalse as libc::c_int }
            (*lreach).areanum = area1num;
            (*lreach).facenum = ladderface2num;
            (*lreach).edgenum = abs(sharededgenum);
            (*lreach).start[0usize] = area2point[0usize];
            (*lreach).start[1usize] = area2point[1usize];
            (*lreach).start[2usize] = area2point[2usize];
            (*lreach).end[0usize] =
                area1point[0usize] +
                    (*plane1).normal[0usize] * -3i32 as libc::c_float;
            (*lreach).end[1usize] =
                area1point[1usize] +
                    (*plane1).normal[1usize] * -3i32 as libc::c_float;
            (*lreach).end[2usize] =
                area1point[2usize] +
                    (*plane1).normal[2usize] * -3i32 as libc::c_float;
            (*lreach).traveltype = 6i32;
            (*lreach).traveltime = 10i32 as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area2num as isize);
            let ref mut fresh15 = *areareachability.offset(area2num as isize);
            *fresh15 = lreach;
            reach_ladder += 1;
            return qtrue as libc::c_int
        }
        if 0 != ladderface1vertical && 0 != (*ladderface2).faceflags & 4i32 {
            lreach = AAS_AllocReachability();
            if lreach.is_null() { return qfalse as libc::c_int }
            (*lreach).areanum = area2num;
            (*lreach).facenum = ladderface1num;
            (*lreach).edgenum = abs(sharededgenum);
            (*lreach).start[0usize] = area1point[0usize];
            (*lreach).start[1usize] = area1point[1usize];
            (*lreach).start[2usize] = area1point[2usize];
            (*lreach).end[0usize] = area2point[0usize];
            (*lreach).end[1usize] = area2point[1usize];
            (*lreach).end[2usize] = area2point[2usize];
            (*lreach).end[2usize] += 16i32 as libc::c_float;
            (*lreach).end[0usize] =
                (*lreach).end[0usize] +
                    (*plane1).normal[0usize] * -15i32 as libc::c_float;
            (*lreach).end[1usize] =
                (*lreach).end[1usize] +
                    (*plane1).normal[1usize] * -15i32 as libc::c_float;
            (*lreach).end[2usize] =
                (*lreach).end[2usize] +
                    (*plane1).normal[2usize] * -15i32 as libc::c_float;
            (*lreach).traveltype = 6i32;
            (*lreach).traveltime = 10i32 as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area1num as isize);
            let ref mut fresh16 = *areareachability.offset(area1num as isize);
            *fresh16 = lreach;
            reach_ladder += 1;
            lreach = AAS_AllocReachability();
            if lreach.is_null() { return qfalse as libc::c_int }
            (*lreach).areanum = area1num;
            (*lreach).facenum = ladderface2num;
            (*lreach).edgenum = abs(sharededgenum);
            (*lreach).start[0usize] = area2point[0usize];
            (*lreach).start[1usize] = area2point[1usize];
            (*lreach).start[2usize] = area2point[2usize];
            (*lreach).end[0usize] = area1point[0usize];
            (*lreach).end[1usize] = area1point[1usize];
            (*lreach).end[2usize] = area1point[2usize];
            (*lreach).traveltype = 7i32;
            (*lreach).traveltime = 10i32 as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area2num as isize);
            let ref mut fresh17 = *areareachability.offset(area2num as isize);
            *fresh17 = lreach;
            reach_walkoffledge += 1;
            return qtrue as libc::c_int
        }
        if 0 != ladderface1vertical {
            lowestpoint[2usize] = 99999i32 as vec_t;
            i = 0i32;
            while i < (*ladderface1).numedges {
                edge1num =
                    abs(*aasworld.edgeindex.offset(((*ladderface1).firstedge +
                                                        i) as isize));
                edge1 =
                    &mut *aasworld.edges.offset(edge1num as isize) as
                        *mut aas_edge_t;
                v1[0usize] =
                    (*aasworld.vertexes.offset((*edge1).v[0usize] as
                                                   isize))[0usize];
                v1[1usize] =
                    (*aasworld.vertexes.offset((*edge1).v[0usize] as
                                                   isize))[1usize];
                v1[2usize] =
                    (*aasworld.vertexes.offset((*edge1).v[0usize] as
                                                   isize))[2usize];
                v2[0usize] =
                    (*aasworld.vertexes.offset((*edge1).v[1usize] as
                                                   isize))[0usize];
                v2[1usize] =
                    (*aasworld.vertexes.offset((*edge1).v[1usize] as
                                                   isize))[1usize];
                v2[2usize] =
                    (*aasworld.vertexes.offset((*edge1).v[1usize] as
                                                   isize))[2usize];
                mid[0usize] = v1[0usize] + v2[0usize];
                mid[1usize] = v1[1usize] + v2[1usize];
                mid[2usize] = v1[2usize] + v2[2usize];
                mid[0usize] =
                    (mid[0usize] as libc::c_double * 0.5f64) as vec_t;
                mid[1usize] =
                    (mid[1usize] as libc::c_double * 0.5f64) as vec_t;
                mid[2usize] =
                    (mid[2usize] as libc::c_double * 0.5f64) as vec_t;
                if mid[2usize] < lowestpoint[2usize] {
                    lowestpoint[0usize] = mid[0usize];
                    lowestpoint[1usize] = mid[1usize];
                    lowestpoint[2usize] = mid[2usize];
                    lowestedgenum = edge1num
                }
                i += 1
            }
            plane1 =
                &mut *aasworld.planes.offset((*ladderface1).planenum as isize)
                    as *mut aas_plane_t;
            start[0usize] =
                lowestpoint[0usize] +
                    (*plane1).normal[0usize] * 5i32 as libc::c_float;
            start[1usize] =
                lowestpoint[1usize] +
                    (*plane1).normal[1usize] * 5i32 as libc::c_float;
            start[2usize] =
                lowestpoint[2usize] +
                    (*plane1).normal[2usize] * 5i32 as libc::c_float;
            end[0usize] = start[0usize];
            end[1usize] = start[1usize];
            end[2usize] = start[2usize];
            start[2usize] += 5i32 as libc::c_float;
            end[2usize] -= 100i32 as libc::c_float;
            trace =
                AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(),
                                    2i32, -1i32);
            trace.endpos[2usize] += 1i32 as libc::c_float;
            area2num = AAS_PointAreaNum(trace.endpos.as_mut_ptr());
            area2 =
                &mut *aasworld.areas.offset(area2num as isize) as
                    *mut aas_area_t;
            i = 0i32;
            while i < (*area2).numfaces {
                face2num =
                    *aasworld.faceindex.offset(((*area2).firstface + i) as
                                                   isize);
                face2 =
                    &mut *aasworld.faces.offset(abs(face2num) as isize) as
                        *mut aas_face_t;
                //
                if 0 != (*face2).faceflags & 2i32 {
                    plane2 =
                        &mut *aasworld.planes.offset((*face2).planenum as
                                                         isize) as
                            *mut aas_plane_t;
                    if (fabsf((*plane2).normal[0usize] * up[0usize] +
                                  (*plane2).normal[1usize] * up[1usize] +
                                  (*plane2).normal[2usize] * up[2usize]) as
                            libc::c_double) < 0.1f64 {
                        break ;
                    }
                }
                i += 1
            }
            if i >= (*area2).numfaces && area2num != area1num &&
                   0 == AAS_ReachabilityExists(area1num, area2num) as u64 &&
                   0 == AAS_ReachabilityExists(area2num, area1num) as u64 {
                if start[2usize] - trace.endpos[2usize] < maxjumpheight {
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() { return qfalse as libc::c_int }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = ladderface1num;
                    (*lreach).edgenum = lowestedgenum;
                    (*lreach).start[0usize] = lowestpoint[0usize];
                    (*lreach).start[1usize] = lowestpoint[1usize];
                    (*lreach).start[2usize] = lowestpoint[2usize];
                    (*lreach).end[0usize] = trace.endpos[0usize];
                    (*lreach).end[1usize] = trace.endpos[1usize];
                    (*lreach).end[2usize] = trace.endpos[2usize];
                    (*lreach).traveltype = 6i32;
                    (*lreach).traveltime = 10i32 as libc::c_ushort;
                    (*lreach).next =
                        *areareachability.offset(area1num as isize);
                    let ref mut fresh18 =
                        *areareachability.offset(area1num as isize);
                    *fresh18 = lreach;
                    reach_ladder += 1;
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() { return qfalse as libc::c_int }
                    (*lreach).areanum = area1num;
                    (*lreach).facenum = ladderface1num;
                    (*lreach).edgenum = lowestedgenum;
                    (*lreach).start[0usize] = trace.endpos[0usize];
                    (*lreach).start[1usize] = trace.endpos[1usize];
                    (*lreach).start[2usize] = trace.endpos[2usize];
                    (*lreach).end[0usize] =
                        lowestpoint[0usize] +
                            (*plane1).normal[0usize] * -5i32 as libc::c_float;
                    (*lreach).end[1usize] =
                        lowestpoint[1usize] +
                            (*plane1).normal[1usize] * -5i32 as libc::c_float;
                    (*lreach).end[2usize] =
                        lowestpoint[2usize] +
                            (*plane1).normal[2usize] * -5i32 as libc::c_float;
                    (*lreach).end[2usize] += 10i32 as libc::c_float;
                    (*lreach).traveltype = 5i32;
                    (*lreach).traveltime = 10i32 as libc::c_ushort;
                    (*lreach).next =
                        *areareachability.offset(area2num as isize);
                    let ref mut fresh19 =
                        *areareachability.offset(area2num as isize);
                    *fresh19 = lreach;
                    reach_jump += 1;
                    return qtrue as libc::c_int
                }
            }
        }
    }
    return qfalse as libc::c_int;
}
//climb or descent a ladder
#[no_mangle]
pub static mut reach_ladder: libc::c_int = 0;
//end of the function AAS_Reachability_EqualFloorHeight
//===========================================================================
// searches step, barrier, waterjump and walk off ledge reachabilities
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Step_Barrier_WaterJump_WalkOffLedge(mut area1num:
                                                                                  libc::c_int,
                                                                              mut area2num:
                                                                                  libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut ground_bestarea2groundedgenum: libc::c_int = 0;
    let mut ground_foundreach: libc::c_int = 0;
    let mut water_bestarea2groundedgenum: libc::c_int = 0;
    let mut water_foundreach: libc::c_int = 0;
    let mut side1: libc::c_int = 0;
    let mut area1swim: libc::c_int = 0;
    let mut faceside1: libc::c_int = 0;
    let mut groundface1num: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut diff: libc::c_float = 0.;
    let mut ortdot: libc::c_float = 0.;
    //float invgravitydot;
    let mut x1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut x3: libc::c_float = 0.;
    let mut x4: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut y3: libc::c_float = 0.;
    let mut y4: libc::c_float = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut ground_bestlength: libc::c_float = 0.;
    let mut water_bestlength: libc::c_float = 0.;
    let mut ground_bestdist: libc::c_float = 0.;
    let mut water_bestdist: libc::c_float = 0.;
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut v3: vec3_t = [0.; 3];
    let mut v4: vec3_t = [0.; 3];
    let mut tmpv: vec3_t = [0.; 3];
    let mut p1area1: vec3_t = [0.; 3];
    let mut p1area2: vec3_t = [0.; 3];
    let mut p2area1: vec3_t = [0.; 3];
    let mut p2area2: vec3_t = [0.; 3];
    let mut normal: vec3_t = [0.; 3];
    let mut ort: vec3_t = [0.; 3];
    let mut edgevec: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut ground_beststart: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut ground_bestend: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut ground_bestnormal: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut water_beststart: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut water_bestend: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut water_bestnormal: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut invgravity: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut testpoint: vec3_t = [0.; 3];
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut groundface1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut groundface2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge1: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut edge2: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    if 0 == AAS_AreaGrounded(area1num) && 0 == AAS_AreaSwim(area1num) {
        return qfalse as libc::c_int
    }
    if 0 == AAS_AreaGrounded(area2num) && 0 == AAS_AreaSwim(area2num) {
        return qfalse as libc::c_int
    }
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    area1swim = AAS_AreaSwim(area1num);
    i = 0i32;
    while i < 2i32 {
        if (*area1).mins[i as usize] >
               (*area2).maxs[i as usize] + 10i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        if (*area1).maxs[i as usize] <
               (*area2).mins[i as usize] - 10i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        i += 1
    }
    ground_foundreach = qfalse as libc::c_int;
    ground_bestdist = 99999i32 as libc::c_float;
    ground_bestlength = 0i32 as libc::c_float;
    ground_bestarea2groundedgenum = 0i32;
    water_foundreach = qfalse as libc::c_int;
    water_bestdist = 99999i32 as libc::c_float;
    water_bestlength = 0i32 as libc::c_float;
    water_bestarea2groundedgenum = 0i32;
    let mut current_block_168: u64;
    i = 0i32;
    while i < (*area1).numfaces {
        groundface1num =
            *aasworld.faceindex.offset(((*area1).firstface + i) as isize);
        faceside1 = (groundface1num < 0i32) as libc::c_int;
        groundface1 =
            &mut *aasworld.faces.offset(abs(groundface1num) as isize) as
                *mut aas_face_t;
        //if this isn't a ground face
        if 0 == (*groundface1).faceflags & 4i32 {
            //if we can swim in the first area
            if 0 != area1swim {
                plane =
                    &mut *aasworld.planes.offset(((*groundface1).planenum ^
                                                      (0 == faceside1) as
                                                          libc::c_int) as
                                                     isize) as
                        *mut aas_plane_t;
                if (((*plane).normal[0usize] * invgravity[0usize] +
                         (*plane).normal[1usize] * invgravity[1usize] +
                         (*plane).normal[2usize] * invgravity[2usize]) as
                        libc::c_double) < 0.7f64 {
                    current_block_168 = 8693738493027456495;
                } else { current_block_168 = 6450597802325118133; }
            } else {
                //end if
                //if we can't swim in the area it must be a ground face
                current_block_168 = 8693738493027456495;
            }
        } else { current_block_168 = 6450597802325118133; }
        match current_block_168 {
            6450597802325118133 => {
                k = 0i32;
                while k < (*groundface1).numedges {
                    edge1num =
                        *aasworld.edgeindex.offset(((*groundface1).firstedge +
                                                        k) as isize);
                    side1 = (edge1num < 0i32) as libc::c_int;
                    if 0 == (*groundface1).faceflags & 4i32 {
                        side1 = (side1 == faceside1) as libc::c_int
                    }
                    edge1num = abs(edge1num);
                    edge1 =
                        &mut *aasworld.edges.offset(edge1num as isize) as
                            *mut aas_edge_t;
                    v1[0usize] =
                        (*aasworld.vertexes.offset((*edge1).v[(0 == side1) as
                                                                  libc::c_int
                                                                  as usize] as
                                                       isize))[0usize];
                    v1[1usize] =
                        (*aasworld.vertexes.offset((*edge1).v[(0 == side1) as
                                                                  libc::c_int
                                                                  as usize] as
                                                       isize))[1usize];
                    v1[2usize] =
                        (*aasworld.vertexes.offset((*edge1).v[(0 == side1) as
                                                                  libc::c_int
                                                                  as usize] as
                                                       isize))[2usize];
                    v2[0usize] =
                        (*aasworld.vertexes.offset((*edge1).v[side1 as usize]
                                                       as isize))[0usize];
                    v2[1usize] =
                        (*aasworld.vertexes.offset((*edge1).v[side1 as usize]
                                                       as isize))[1usize];
                    v2[2usize] =
                        (*aasworld.vertexes.offset((*edge1).v[side1 as usize]
                                                       as isize))[2usize];
                    edgevec[0usize] = v2[0usize] - v1[0usize];
                    edgevec[1usize] = v2[1usize] - v1[1usize];
                    edgevec[2usize] = v2[2usize] - v1[2usize];
                    CrossProduct(edgevec.as_mut_ptr() as *const vec_t,
                                 invgravity.as_mut_ptr() as *const vec_t,
                                 normal.as_mut_ptr());
                    VectorNormalize(normal.as_mut_ptr());
                    dist =
                        normal[0usize] * v1[0usize] +
                            normal[1usize] * v1[1usize] +
                            normal[2usize] * v1[2usize];
                    j = 0i32;
                    while j < (*area2).numfaces {
                        groundface2 =
                            &mut *aasworld.faces.offset(abs(*aasworld.faceindex.offset(((*area2).firstface
                                                                                            +
                                                                                            j)
                                                                                           as
                                                                                           isize))
                                                            as isize) as
                                *mut aas_face_t;
                        //must be a ground face
                        if !(0 == (*groundface2).faceflags & 4i32) {
                            l = 0i32;
                            while l < (*groundface2).numedges {
                                edge2num =
                                    abs(*aasworld.edgeindex.offset(((*groundface2).firstedge
                                                                        + l)
                                                                       as
                                                                       isize));
                                edge2 =
                                    &mut *aasworld.edges.offset(edge2num as
                                                                    isize) as
                                        *mut aas_edge_t;
                                v3[0usize] =
                                    (*aasworld.vertexes.offset((*edge2).v[0usize]
                                                                   as
                                                                   isize))[0usize];
                                v3[1usize] =
                                    (*aasworld.vertexes.offset((*edge2).v[0usize]
                                                                   as
                                                                   isize))[1usize];
                                v3[2usize] =
                                    (*aasworld.vertexes.offset((*edge2).v[0usize]
                                                                   as
                                                                   isize))[2usize];
                                v4[0usize] =
                                    (*aasworld.vertexes.offset((*edge2).v[1usize]
                                                                   as
                                                                   isize))[0usize];
                                v4[1usize] =
                                    (*aasworld.vertexes.offset((*edge2).v[1usize]
                                                                   as
                                                                   isize))[1usize];
                                v4[2usize] =
                                    (*aasworld.vertexes.offset((*edge2).v[1usize]
                                                                   as
                                                                   isize))[2usize];
                                diff =
                                    normal[0usize] * v3[0usize] +
                                        normal[1usize] * v3[1usize] +
                                        normal[2usize] * v3[2usize] - dist;
                                if !((diff as libc::c_double) < -0.1f64 ||
                                         diff as libc::c_double > 0.1f64) {
                                    diff =
                                        normal[0usize] * v4[0usize] +
                                            normal[1usize] * v4[1usize] +
                                            normal[2usize] * v4[2usize] -
                                            dist;
                                    if !((diff as libc::c_double) < -0.1f64 ||
                                             diff as libc::c_double > 0.1f64)
                                       {
                                        CrossProduct(invgravity.as_mut_ptr()
                                                         as *const vec_t,
                                                     normal.as_mut_ptr() as
                                                         *const vec_t,
                                                     ort.as_mut_ptr());
                                        ortdot =
                                            ort[0usize] * ort[0usize] +
                                                ort[1usize] * ort[1usize] +
                                                ort[2usize] * ort[2usize];
                                        y1 = v1[2usize];
                                        y2 = v2[2usize];
                                        y3 = v3[2usize];
                                        y4 = v4[2usize];
                                        x1 =
                                            (v1[0usize] * ort[0usize] +
                                                 v1[1usize] * ort[1usize] +
                                                 v1[2usize] * ort[2usize]) /
                                                ortdot;
                                        x2 =
                                            (v2[0usize] * ort[0usize] +
                                                 v2[1usize] * ort[1usize] +
                                                 v2[2usize] * ort[2usize]) /
                                                ortdot;
                                        x3 =
                                            (v3[0usize] * ort[0usize] +
                                                 v3[1usize] * ort[1usize] +
                                                 v3[2usize] * ort[2usize]) /
                                                ortdot;
                                        x4 =
                                            (v4[0usize] * ort[0usize] +
                                                 v4[1usize] * ort[1usize] +
                                                 v4[2usize] * ort[2usize]) /
                                                ortdot;
                                        if x1 > x2 {
                                            tmp = x1;
                                            x1 = x2;
                                            x2 = tmp;
                                            tmp = y1;
                                            y1 = y2;
                                            y2 = tmp;
                                            tmpv[0usize] = v1[0usize];
                                            tmpv[1usize] = v1[1usize];
                                            tmpv[2usize] = v1[2usize];
                                            v1[0usize] = v2[0usize];
                                            v1[1usize] = v2[1usize];
                                            v1[2usize] = v2[2usize];
                                            v2[0usize] = tmpv[0usize];
                                            v2[1usize] = tmpv[1usize];
                                            v2[2usize] = tmpv[2usize]
                                        }
                                        if x3 > x4 {
                                            tmp = x3;
                                            x3 = x4;
                                            x4 = tmp;
                                            tmp = y3;
                                            y3 = y4;
                                            y4 = tmp;
                                            tmpv[0usize] = v3[0usize];
                                            tmpv[1usize] = v3[1usize];
                                            tmpv[2usize] = v3[2usize];
                                            v3[0usize] = v4[0usize];
                                            v3[1usize] = v4[1usize];
                                            v3[2usize] = v4[2usize];
                                            v4[0usize] = tmpv[0usize];
                                            v4[1usize] = tmpv[1usize];
                                            v4[2usize] = tmpv[2usize]
                                        }
                                        //end if
                                        //if the two projected edge lines have no overlap
                                        if !(x2 <= x3 || x4 <= x1) {
                                            //						Log_Write("lines no overlap: from area %d to %d\r\n", area1num, area2num);
                                            if x1 as libc::c_double - 0.5f64 <
                                                   x3 as libc::c_double &&
                                                   (x4 as libc::c_double) <
                                                       x2 as libc::c_double +
                                                           0.5f64 &&
                                                   (x3 as libc::c_double -
                                                        0.5f64 <
                                                        x1 as libc::c_double
                                                        &&
                                                        (x2 as libc::c_double)
                                                            <
                                                            x4 as
                                                                libc::c_double
                                                                + 0.5f64) {
                                                dist1 = y3 - y1;
                                                dist2 = y4 - y2;
                                                p1area1[0usize] = v1[0usize];
                                                p1area1[1usize] = v1[1usize];
                                                p1area1[2usize] = v1[2usize];
                                                p2area1[0usize] = v2[0usize];
                                                p2area1[1usize] = v2[1usize];
                                                p2area1[2usize] = v2[2usize];
                                                p1area2[0usize] = v3[0usize];
                                                p1area2[1usize] = v3[1usize];
                                                p1area2[2usize] = v3[2usize];
                                                p2area2[0usize] = v4[0usize];
                                                p2area2[1usize] = v4[1usize];
                                                p2area2[2usize] = v4[2usize]
                                            } else {
                                                if x1 as libc::c_double >
                                                       x3 as libc::c_double -
                                                           0.1f64 &&
                                                       (x1 as libc::c_double)
                                                           <
                                                           x3 as
                                                               libc::c_double
                                                               + 0.1f64 {
                                                    dist1 = y3 - y1;
                                                    p1area1[0usize] =
                                                        v1[0usize];
                                                    p1area1[1usize] =
                                                        v1[1usize];
                                                    p1area1[2usize] =
                                                        v1[2usize];
                                                    p1area2[0usize] =
                                                        v3[0usize];
                                                    p1area2[1usize] =
                                                        v3[1usize];
                                                    p1area2[2usize] =
                                                        v3[2usize]
                                                } else if x1 < x3 {
                                                    y =
                                                        y1 +
                                                            (x3 - x1) *
                                                                (y2 - y1) /
                                                                (x2 - x1);
                                                    dist1 = y3 - y;
                                                    p1area1[0usize] =
                                                        v3[0usize];
                                                    p1area1[1usize] =
                                                        v3[1usize];
                                                    p1area1[2usize] =
                                                        v3[2usize];
                                                    p1area1[2usize] = y;
                                                    p1area2[0usize] =
                                                        v3[0usize];
                                                    p1area2[1usize] =
                                                        v3[1usize];
                                                    p1area2[2usize] =
                                                        v3[2usize]
                                                } else {
                                                    y =
                                                        y3 +
                                                            (x1 - x3) *
                                                                (y4 - y3) /
                                                                (x4 - x3);
                                                    dist1 = y - y1;
                                                    p1area1[0usize] =
                                                        v1[0usize];
                                                    p1area1[1usize] =
                                                        v1[1usize];
                                                    p1area1[2usize] =
                                                        v1[2usize];
                                                    p1area2[0usize] =
                                                        v1[0usize];
                                                    p1area2[1usize] =
                                                        v1[1usize];
                                                    p1area2[2usize] =
                                                        v1[2usize];
                                                    p1area2[2usize] = y
                                                }
                                                if x2 as libc::c_double >
                                                       x4 as libc::c_double -
                                                           0.1f64 &&
                                                       (x2 as libc::c_double)
                                                           <
                                                           x4 as
                                                               libc::c_double
                                                               + 0.1f64 {
                                                    dist2 = y4 - y2;
                                                    p2area1[0usize] =
                                                        v2[0usize];
                                                    p2area1[1usize] =
                                                        v2[1usize];
                                                    p2area1[2usize] =
                                                        v2[2usize];
                                                    p2area2[0usize] =
                                                        v4[0usize];
                                                    p2area2[1usize] =
                                                        v4[1usize];
                                                    p2area2[2usize] =
                                                        v4[2usize]
                                                } else if x2 < x4 {
                                                    y =
                                                        y3 +
                                                            (x2 - x3) *
                                                                (y4 - y3) /
                                                                (x4 - x3);
                                                    dist2 = y - y2;
                                                    p2area1[0usize] =
                                                        v2[0usize];
                                                    p2area1[1usize] =
                                                        v2[1usize];
                                                    p2area1[2usize] =
                                                        v2[2usize];
                                                    p2area2[0usize] =
                                                        v2[0usize];
                                                    p2area2[1usize] =
                                                        v2[1usize];
                                                    p2area2[2usize] =
                                                        v2[2usize];
                                                    p2area2[2usize] = y
                                                } else {
                                                    y =
                                                        y1 +
                                                            (x4 - x1) *
                                                                (y2 - y1) /
                                                                (x2 - x1);
                                                    dist2 = y4 - y;
                                                    p2area1[0usize] =
                                                        v4[0usize];
                                                    p2area1[1usize] =
                                                        v4[1usize];
                                                    p2area1[2usize] =
                                                        v4[2usize];
                                                    p2area1[2usize] = y;
                                                    p2area2[0usize] =
                                                        v4[0usize];
                                                    p2area2[1usize] =
                                                        v4[1usize];
                                                    p2area2[2usize] =
                                                        v4[2usize]
                                                }
                                            }
                                            if dist1 >
                                                   dist2 -
                                                       1i32 as libc::c_float
                                                   &&
                                                   dist1 <
                                                       dist2 +
                                                           1i32 as
                                                               libc::c_float {
                                                dist = dist1;
                                                start[0usize] =
                                                    p1area1[0usize] +
                                                        p2area1[0usize];
                                                start[1usize] =
                                                    p1area1[1usize] +
                                                        p2area1[1usize];
                                                start[2usize] =
                                                    p1area1[2usize] +
                                                        p2area1[2usize];
                                                start[0usize] =
                                                    (start[0usize] as
                                                         libc::c_double *
                                                         0.5f64) as vec_t;
                                                start[1usize] =
                                                    (start[1usize] as
                                                         libc::c_double *
                                                         0.5f64) as vec_t;
                                                start[2usize] =
                                                    (start[2usize] as
                                                         libc::c_double *
                                                         0.5f64) as vec_t;
                                                end[0usize] =
                                                    p1area2[0usize] +
                                                        p2area2[0usize];
                                                end[1usize] =
                                                    p1area2[1usize] +
                                                        p2area2[1usize];
                                                end[2usize] =
                                                    p1area2[2usize] +
                                                        p2area2[2usize];
                                                end[0usize] =
                                                    (end[0usize] as
                                                         libc::c_double *
                                                         0.5f64) as vec_t;
                                                end[1usize] =
                                                    (end[1usize] as
                                                         libc::c_double *
                                                         0.5f64) as vec_t;
                                                end[2usize] =
                                                    (end[2usize] as
                                                         libc::c_double *
                                                         0.5f64) as vec_t
                                            } else if dist1 < dist2 {
                                                dist = dist1;
                                                start[0usize] =
                                                    p1area1[0usize];
                                                start[1usize] =
                                                    p1area1[1usize];
                                                start[2usize] =
                                                    p1area1[2usize];
                                                end[0usize] = p1area2[0usize];
                                                end[1usize] = p1area2[1usize];
                                                end[2usize] = p1area2[2usize]
                                            } else {
                                                dist = dist2;
                                                start[0usize] =
                                                    p2area1[0usize];
                                                start[1usize] =
                                                    p2area1[1usize];
                                                start[2usize] =
                                                    p2area1[2usize];
                                                end[0usize] = p2area2[0usize];
                                                end[1usize] = p2area2[1usize];
                                                end[2usize] = p2area2[2usize]
                                            }
                                            dir[0usize] =
                                                p2area2[0usize] -
                                                    p1area2[0usize];
                                            dir[1usize] =
                                                p2area2[1usize] -
                                                    p1area2[1usize];
                                            dir[2usize] =
                                                p2area2[2usize] -
                                                    p1area2[2usize];
                                            length =
                                                VectorLength(dir.as_mut_ptr()
                                                                 as
                                                                 *const vec_t);
                                            if 0 !=
                                                   (*groundface1).faceflags &
                                                       4i32 {
                                                if dist < ground_bestdist ||
                                                       dist <
                                                           ground_bestdist +
                                                               1i32 as
                                                                   libc::c_float
                                                           &&
                                                           length >
                                                               ground_bestlength
                                                   {
                                                    ground_bestdist = dist;
                                                    ground_bestlength =
                                                        length;
                                                    ground_foundreach =
                                                        qtrue as libc::c_int;
                                                    ground_bestarea2groundedgenum
                                                        = edge1num;
                                                    ground_beststart[0usize] =
                                                        start[0usize];
                                                    ground_beststart[1usize] =
                                                        start[1usize];
                                                    ground_beststart[2usize] =
                                                        start[2usize];
                                                    ground_bestnormal[0usize]
                                                        = normal[0usize];
                                                    ground_bestnormal[1usize]
                                                        = normal[1usize];
                                                    ground_bestnormal[2usize]
                                                        = normal[2usize];
                                                    ground_bestend[0usize] =
                                                        end[0usize];
                                                    ground_bestend[1usize] =
                                                        end[1usize];
                                                    ground_bestend[2usize] =
                                                        end[2usize]
                                                }
                                            } else if dist < water_bestdist ||
                                                          dist <
                                                              water_bestdist +
                                                                  1i32 as
                                                                      libc::c_float
                                                              &&
                                                              length >
                                                                  water_bestlength
                                             {
                                                water_bestdist = dist;
                                                water_bestlength = length;
                                                water_foundreach =
                                                    qtrue as libc::c_int;
                                                water_bestarea2groundedgenum =
                                                    edge1num;
                                                water_beststart[0usize] =
                                                    start[0usize];
                                                water_beststart[1usize] =
                                                    start[1usize];
                                                water_beststart[2usize] =
                                                    start[2usize];
                                                water_bestnormal[0usize] =
                                                    normal[0usize];
                                                water_bestnormal[1usize] =
                                                    normal[1usize];
                                                water_bestnormal[2usize] =
                                                    normal[2usize];
                                                water_bestend[0usize] =
                                                    end[0usize];
                                                water_bestend[1usize] =
                                                    end[1usize];
                                                water_bestend[2usize] =
                                                    end[2usize]
                                            }
                                        }
                                    }
                                }
                                l += 1
                            }
                        }
                        j += 1
                    }
                    k += 1
                }
            }
            _ => { }
        }
        i += 1
    }
    if 0 != ground_foundreach {
        if ground_bestdist >= 0i32 as libc::c_float &&
               ground_bestdist < aassettings.phys_maxstep {
            lreach = AAS_AllocReachability();
            if lreach.is_null() { return qfalse as libc::c_int }
            (*lreach).areanum = area2num;
            (*lreach).facenum = 0i32;
            (*lreach).edgenum = ground_bestarea2groundedgenum;
            (*lreach).start[0usize] =
                (ground_beststart[0usize] as libc::c_double +
                     ground_bestnormal[0usize] as libc::c_double * 0.1f64) as
                    vec_t;
            (*lreach).start[1usize] =
                (ground_beststart[1usize] as libc::c_double +
                     ground_bestnormal[1usize] as libc::c_double * 0.1f64) as
                    vec_t;
            (*lreach).start[2usize] =
                (ground_beststart[2usize] as libc::c_double +
                     ground_bestnormal[2usize] as libc::c_double * 0.1f64) as
                    vec_t;
            (*lreach).end[0usize] =
                ground_bestend[0usize] +
                    ground_bestnormal[0usize] * 5i32 as libc::c_float;
            (*lreach).end[1usize] =
                ground_bestend[1usize] +
                    ground_bestnormal[1usize] * 5i32 as libc::c_float;
            (*lreach).end[2usize] =
                ground_bestend[2usize] +
                    ground_bestnormal[2usize] * 5i32 as libc::c_float;
            (*lreach).traveltype = 2i32;
            (*lreach).traveltime = 0i32 as libc::c_ushort;
            if 0 == AAS_AreaCrouch(area1num) && 0 != AAS_AreaCrouch(area2num)
               {
                (*lreach).traveltime =
                    ((*lreach).traveltime as libc::c_float +
                         aassettings.rs_startcrouch) as libc::c_ushort
            }
            (*lreach).next = *areareachability.offset(area1num as isize);
            let ref mut fresh20 = *areareachability.offset(area1num as isize);
            *fresh20 = lreach;
            reach_step += 1;
            return qtrue as libc::c_int
        }
    }
    if 0 != water_foundreach {
        testpoint[0usize] =
            water_bestend[0usize] +
                water_bestnormal[0usize] * -2i32 as libc::c_float;
        testpoint[1usize] =
            water_bestend[1usize] +
                water_bestnormal[1usize] * -2i32 as libc::c_float;
        testpoint[2usize] =
            water_bestend[2usize] +
                water_bestnormal[2usize] * -2i32 as libc::c_float;
        testpoint[2usize] -= aassettings.phys_maxwaterjump;
        if 0 !=
               (*aasworld.areasettings.offset(AAS_PointAreaNum(testpoint.as_mut_ptr())
                                                  as isize)).areaflags & 4i32
           {
            if water_bestdist <
                   aassettings.phys_maxwaterjump + 24i32 as libc::c_float {
                if 0 !=
                       (*aasworld.areasettings.offset(area1num as
                                                          isize)).presencetype
                           & 2i32 &&
                       0 !=
                           (*aasworld.areasettings.offset(area2num as
                                                              isize)).presencetype
                               & 2i32 {
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() { return qfalse as libc::c_int }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = 0i32;
                    (*lreach).edgenum = water_bestarea2groundedgenum;
                    (*lreach).start[0usize] = water_beststart[0usize];
                    (*lreach).start[1usize] = water_beststart[1usize];
                    (*lreach).start[2usize] = water_beststart[2usize];
                    (*lreach).end[0usize] =
                        water_bestend[0usize] +
                            water_bestnormal[0usize] * 15i32 as libc::c_float;
                    (*lreach).end[1usize] =
                        water_bestend[1usize] +
                            water_bestnormal[1usize] * 15i32 as libc::c_float;
                    (*lreach).end[2usize] =
                        water_bestend[2usize] +
                            water_bestnormal[2usize] * 15i32 as libc::c_float;
                    (*lreach).traveltype = 9i32;
                    (*lreach).traveltime =
                        aassettings.rs_waterjump as libc::c_ushort;
                    (*lreach).next =
                        *areareachability.offset(area1num as isize);
                    let ref mut fresh21 =
                        *areareachability.offset(area1num as isize);
                    *fresh21 = lreach;
                    reach_waterjump += 1;
                    return qtrue as libc::c_int
                }
            }
        }
    }
    if 0 != ground_foundreach {
        if ground_bestdist > 0i32 as libc::c_float &&
               ground_bestdist < aassettings.phys_maxbarrier {
            if 0 == water_foundreach ||
                   ground_bestdist - water_bestdist < 16i32 as libc::c_float {
                if 0 == AAS_AreaCrouch(area1num) &&
                       0 == AAS_AreaCrouch(area2num) {
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() { return qfalse as libc::c_int }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = 0i32;
                    (*lreach).edgenum = ground_bestarea2groundedgenum;
                    (*lreach).start[0usize] =
                        (ground_beststart[0usize] as libc::c_double +
                             ground_bestnormal[0usize] as libc::c_double *
                                 0.1f64) as vec_t;
                    (*lreach).start[1usize] =
                        (ground_beststart[1usize] as libc::c_double +
                             ground_bestnormal[1usize] as libc::c_double *
                                 0.1f64) as vec_t;
                    (*lreach).start[2usize] =
                        (ground_beststart[2usize] as libc::c_double +
                             ground_bestnormal[2usize] as libc::c_double *
                                 0.1f64) as vec_t;
                    (*lreach).end[0usize] =
                        ground_bestend[0usize] +
                            ground_bestnormal[0usize] * 5i32 as libc::c_float;
                    (*lreach).end[1usize] =
                        ground_bestend[1usize] +
                            ground_bestnormal[1usize] * 5i32 as libc::c_float;
                    (*lreach).end[2usize] =
                        ground_bestend[2usize] +
                            ground_bestnormal[2usize] * 5i32 as libc::c_float;
                    (*lreach).traveltype = 4i32;
                    (*lreach).traveltime =
                        aassettings.rs_barrierjump as libc::c_ushort;
                    (*lreach).next =
                        *areareachability.offset(area1num as isize);
                    let ref mut fresh22 =
                        *areareachability.offset(area1num as isize);
                    *fresh22 = lreach;
                    reach_barrier += 1;
                    return qtrue as libc::c_int
                }
            }
        }
    }
    if 0 != ground_foundreach {
        if ground_bestdist < 0i32 as libc::c_float {
            if ground_bestdist > -aassettings.phys_maxstep {
                lreach = AAS_AllocReachability();
                if lreach.is_null() { return qfalse as libc::c_int }
                (*lreach).areanum = area2num;
                (*lreach).facenum = 0i32;
                (*lreach).edgenum = ground_bestarea2groundedgenum;
                (*lreach).start[0usize] =
                    (ground_beststart[0usize] as libc::c_double +
                         ground_bestnormal[0usize] as libc::c_double * 0.1f64)
                        as vec_t;
                (*lreach).start[1usize] =
                    (ground_beststart[1usize] as libc::c_double +
                         ground_bestnormal[1usize] as libc::c_double * 0.1f64)
                        as vec_t;
                (*lreach).start[2usize] =
                    (ground_beststart[2usize] as libc::c_double +
                         ground_bestnormal[2usize] as libc::c_double * 0.1f64)
                        as vec_t;
                (*lreach).end[0usize] =
                    ground_bestend[0usize] +
                        ground_bestnormal[0usize] * 5i32 as libc::c_float;
                (*lreach).end[1usize] =
                    ground_bestend[1usize] +
                        ground_bestnormal[1usize] * 5i32 as libc::c_float;
                (*lreach).end[2usize] =
                    ground_bestend[2usize] +
                        ground_bestnormal[2usize] * 5i32 as libc::c_float;
                (*lreach).traveltype = 2i32;
                (*lreach).traveltime = 1i32 as libc::c_ushort;
                (*lreach).next = *areareachability.offset(area1num as isize);
                let ref mut fresh23 =
                    *areareachability.offset(area1num as isize);
                *fresh23 = lreach;
                reach_walk += 1;
                return qtrue as libc::c_int
            }
            if 0. == aassettings.rs_maxfallheight ||
                   fabs(ground_bestdist as libc::c_double) <
                       aassettings.rs_maxfallheight as libc::c_double {
                ground_bestend[0usize] =
                    ground_bestend[0usize] +
                        ground_bestnormal[0usize] * 2i32 as libc::c_float;
                ground_bestend[1usize] =
                    ground_bestend[1usize] +
                        ground_bestnormal[1usize] * 2i32 as libc::c_float;
                ground_bestend[2usize] =
                    ground_bestend[2usize] +
                        ground_bestnormal[2usize] * 2i32 as libc::c_float;
                start[0usize] = ground_bestend[0usize];
                start[1usize] = ground_bestend[1usize];
                start[2usize] = ground_bestend[2usize];
                start[2usize] = ground_beststart[2usize];
                end[0usize] = ground_bestend[0usize];
                end[1usize] = ground_bestend[1usize];
                end[2usize] = ground_bestend[2usize];
                end[2usize] += 4i32 as libc::c_float;
                trace =
                    AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(),
                                        2i32, -1i32);
                if 0 == trace.startsolid as u64 &&
                       trace.fraction as libc::c_double >= 1.0f64 {
                    trace.endpos[2usize] += 1i32 as libc::c_float;
                    if AAS_PointAreaNum(trace.endpos.as_mut_ptr()) == area2num
                       {
                        numareas =
                            AAS_TraceAreas(start.as_mut_ptr(),
                                           end.as_mut_ptr(),
                                           areas.as_mut_ptr(),
                                           0 as *mut vec3_t,
                                           (::std::mem::size_of::<[libc::c_int; 10]>()
                                                as
                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                                                as
                                                                                libc::c_ulong)
                                               as libc::c_int);
                        i = 0i32;
                        while i < numareas {
                            if 0 != AAS_AreaClusterPortal(areas[i as usize]) {
                                break ;
                            }
                            i += 1
                        }
                        if i >= numareas {
                            lreach = AAS_AllocReachability();
                            if lreach.is_null() {
                                return qfalse as libc::c_int
                            }
                            (*lreach).areanum = area2num;
                            (*lreach).facenum = 0i32;
                            (*lreach).edgenum = ground_bestarea2groundedgenum;
                            (*lreach).start[0usize] =
                                ground_beststart[0usize];
                            (*lreach).start[1usize] =
                                ground_beststart[1usize];
                            (*lreach).start[2usize] =
                                ground_beststart[2usize];
                            (*lreach).end[0usize] = ground_bestend[0usize];
                            (*lreach).end[1usize] = ground_bestend[1usize];
                            (*lreach).end[2usize] = ground_bestend[2usize];
                            (*lreach).traveltype = 7i32;
                            (*lreach).traveltime =
                                (aassettings.rs_startwalkoffledge as
                                     libc::c_double +
                                     fabs(ground_bestdist as libc::c_double) *
                                         50i32 as libc::c_double /
                                         aassettings.phys_gravity as
                                             libc::c_double) as
                                    libc::c_ushort;
                            if 0 == AAS_AreaSwim(area2num) &&
                                   0 == AAS_AreaJumpPad(area2num) {
                                if AAS_FallDelta(ground_bestdist) >
                                       aassettings.phys_falldelta5 {
                                    (*lreach).traveltime =
                                        ((*lreach).traveltime as libc::c_float
                                             + aassettings.rs_falldamage5) as
                                            libc::c_ushort
                                }
                                if AAS_FallDelta(ground_bestdist) >
                                       aassettings.phys_falldelta10 {
                                    (*lreach).traveltime =
                                        ((*lreach).traveltime as libc::c_float
                                             + aassettings.rs_falldamage10) as
                                            libc::c_ushort
                                }
                            }
                            (*lreach).next =
                                *areareachability.offset(area1num as isize);
                            let ref mut fresh24 =
                                *areareachability.offset(area1num as isize);
                            *fresh24 = lreach;
                            reach_walkoffledge += 1;
                            return qtrue as libc::c_int
                        }
                    }
                }
            }
        }
    }
    return qfalse as libc::c_int;
}
//walk of step
#[no_mangle]
pub static mut reach_walk: libc::c_int = 0;
//jump up to a barrier
#[no_mangle]
pub static mut reach_barrier: libc::c_int = 0;
//jump out of water
#[no_mangle]
pub static mut reach_waterjump: libc::c_int = 0;
//step up
#[no_mangle]
pub static mut reach_step: libc::c_int = 0;
//end of the function AAS_Reachability_Swim
//===========================================================================
// searches for reachabilities between adjacent areas with equal floor
// heights
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_EqualFloorHeight(mut area1num:
                                                               libc::c_int,
                                                           mut area2num:
                                                               libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut edgenum1: libc::c_int = 0;
    let mut edgenum2: libc::c_int = 0;
    let mut foundreach: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut height: libc::c_float = 0.;
    let mut bestheight: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut bestlength: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut normal: vec3_t = [0.; 3];
    let mut invgravity: vec3_t = [0.; 3];
    let mut gravitydirection: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, -1i32 as vec_t];
    let mut edgevec: vec3_t = [0.; 3];
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut face2: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut edge: *mut aas_edge_t = 0 as *mut aas_edge_t;
    let mut plane2: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut lr: aas_lreachability_t =
        aas_lreachability_s{areanum: 0,
                            facenum: 0,
                            edgenum: 0,
                            start: [0.; 3],
                            end: [0.; 3],
                            traveltype: 0,
                            traveltime: 0,
                            next: 0 as *mut aas_lreachability_s,};
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    if 0 == AAS_AreaGrounded(area1num) || 0 == AAS_AreaGrounded(area2num) {
        return qfalse as libc::c_int
    }
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    i = 0i32;
    while i < 2i32 {
        if (*area1).mins[i as usize] >
               (*area2).maxs[i as usize] + 10i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        if (*area1).maxs[i as usize] <
               (*area2).mins[i as usize] - 10i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        i += 1
    }
    if (*area2).mins[2usize] > (*area1).maxs[2usize] {
        return qfalse as libc::c_int
    }
    invgravity[0usize] = gravitydirection[0usize];
    invgravity[1usize] = gravitydirection[1usize];
    invgravity[2usize] = gravitydirection[2usize];
    VectorInverse(invgravity.as_mut_ptr());
    bestheight = 99999i32 as libc::c_float;
    bestlength = 0i32 as libc::c_float;
    foundreach = qfalse as libc::c_int;
    memset(&mut lr as *mut aas_lreachability_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_lreachability_t>() as libc::c_ulong);
    i = 0i32;
    while i < (*area1).numfaces {
        face1 =
            &mut *aasworld.faces.offset(abs(*aasworld.faceindex.offset(((*area1).firstface
                                                                            +
                                                                            i)
                                                                           as
                                                                           isize))
                                            as isize) as *mut aas_face_t;
        if !(0 == (*face1).faceflags & 4i32) {
            j = 0i32;
            while j < (*area2).numfaces {
                face2 =
                    &mut *aasworld.faces.offset(abs(*aasworld.faceindex.offset(((*area2).firstface
                                                                                    +
                                                                                    j)
                                                                                   as
                                                                                   isize))
                                                    as isize) as
                        *mut aas_face_t;
                if !(0 == (*face2).faceflags & 4i32) {
                    edgenum1 = 0i32;
                    while edgenum1 < (*face1).numedges {
                        edgenum2 = 0i32;
                        while edgenum2 < (*face2).numedges {
                            if !(abs(*aasworld.edgeindex.offset(((*face1).firstedge
                                                                     +
                                                                     edgenum1)
                                                                    as isize))
                                     !=
                                     abs(*aasworld.edgeindex.offset(((*face2).firstedge
                                                                         +
                                                                         edgenum2)
                                                                        as
                                                                        isize)))
                               {
                                edgenum =
                                    *aasworld.edgeindex.offset(((*face1).firstedge
                                                                    +
                                                                    edgenum1)
                                                                   as isize);
                                side = (edgenum < 0i32) as libc::c_int;
                                edge =
                                    &mut *aasworld.edges.offset(abs(edgenum)
                                                                    as isize)
                                        as *mut aas_edge_t;
                                dir[0usize] =
                                    (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                   as
                                                                   isize))[0usize]
                                        -
                                        (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                       as
                                                                       isize))[0usize];
                                dir[1usize] =
                                    (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                   as
                                                                   isize))[1usize]
                                        -
                                        (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                       as
                                                                       isize))[1usize];
                                dir[2usize] =
                                    (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                   as
                                                                   isize))[2usize]
                                        -
                                        (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                       as
                                                                       isize))[2usize];
                                length =
                                    VectorLength(dir.as_mut_ptr() as
                                                     *const vec_t);
                                start[0usize] =
                                    (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                   as
                                                                   isize))[0usize]
                                        +
                                        (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                       as
                                                                       isize))[0usize];
                                start[1usize] =
                                    (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                   as
                                                                   isize))[1usize]
                                        +
                                        (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                       as
                                                                       isize))[1usize];
                                start[2usize] =
                                    (*aasworld.vertexes.offset((*edge).v[0usize]
                                                                   as
                                                                   isize))[2usize]
                                        +
                                        (*aasworld.vertexes.offset((*edge).v[1usize]
                                                                       as
                                                                       isize))[2usize];
                                start[0usize] =
                                    (start[0usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                start[1usize] =
                                    (start[1usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                start[2usize] =
                                    (start[2usize] as libc::c_double * 0.5f64)
                                        as vec_t;
                                end[0usize] = start[0usize];
                                end[1usize] = start[1usize];
                                end[2usize] = start[2usize];
                                edgevec[0usize] =
                                    (*aasworld.vertexes.offset((*edge).v[side
                                                                             as
                                                                             usize]
                                                                   as
                                                                   isize))[0usize]
                                        -
                                        (*aasworld.vertexes.offset((*edge).v[(0
                                                                                  ==
                                                                                  side)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                       as
                                                                       isize))[0usize];
                                edgevec[1usize] =
                                    (*aasworld.vertexes.offset((*edge).v[side
                                                                             as
                                                                             usize]
                                                                   as
                                                                   isize))[1usize]
                                        -
                                        (*aasworld.vertexes.offset((*edge).v[(0
                                                                                  ==
                                                                                  side)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                       as
                                                                       isize))[1usize];
                                edgevec[2usize] =
                                    (*aasworld.vertexes.offset((*edge).v[side
                                                                             as
                                                                             usize]
                                                                   as
                                                                   isize))[2usize]
                                        -
                                        (*aasworld.vertexes.offset((*edge).v[(0
                                                                                  ==
                                                                                  side)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                       as
                                                                       isize))[2usize];
                                plane2 =
                                    &mut *aasworld.planes.offset((*face2).planenum
                                                                     as isize)
                                        as *mut aas_plane_t;
                                CrossProduct(edgevec.as_mut_ptr() as
                                                 *const vec_t,
                                             (*plane2).normal.as_mut_ptr() as
                                                 *const vec_t,
                                             normal.as_mut_ptr());
                                VectorNormalize(normal.as_mut_ptr());
                                end[0usize] =
                                    end[0usize] +
                                        normal[0usize] *
                                            5i32 as libc::c_float;
                                end[1usize] =
                                    end[1usize] +
                                        normal[1usize] *
                                            5i32 as libc::c_float;
                                end[2usize] =
                                    end[2usize] +
                                        normal[2usize] *
                                            5i32 as libc::c_float;
                                start[0usize] =
                                    (start[0usize] as libc::c_double +
                                         normal[0usize] as libc::c_double *
                                             0.1f64) as vec_t;
                                start[1usize] =
                                    (start[1usize] as libc::c_double +
                                         normal[1usize] as libc::c_double *
                                             0.1f64) as vec_t;
                                start[2usize] =
                                    (start[2usize] as libc::c_double +
                                         normal[2usize] as libc::c_double *
                                             0.1f64) as vec_t;
                                end[2usize] =
                                    (end[2usize] as libc::c_double + 0.125f64)
                                        as vec_t;
                                height =
                                    invgravity[0usize] * start[0usize] +
                                        invgravity[1usize] * start[1usize] +
                                        invgravity[2usize] * start[2usize];
                                if height < bestheight ||
                                       height <
                                           bestheight + 1i32 as libc::c_float
                                           && length > bestlength {
                                    bestheight = height;
                                    bestlength = length;
                                    lr.areanum = area2num;
                                    lr.facenum = 0i32;
                                    lr.edgenum = edgenum;
                                    lr.start[0usize] = start[0usize];
                                    lr.start[1usize] = start[1usize];
                                    lr.start[2usize] = start[2usize];
                                    lr.end[0usize] = end[0usize];
                                    lr.end[1usize] = end[1usize];
                                    lr.end[2usize] = end[2usize];
                                    lr.traveltype = 2i32;
                                    lr.traveltime = 1i32 as libc::c_ushort;
                                    foundreach = qtrue as libc::c_int
                                }
                            }
                            edgenum2 += 1
                        }
                        edgenum1 += 1
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    if 0 != foundreach {
        lreach = AAS_AllocReachability();
        if lreach.is_null() { return qfalse as libc::c_int }
        (*lreach).areanum = lr.areanum;
        (*lreach).facenum = lr.facenum;
        (*lreach).edgenum = lr.edgenum;
        (*lreach).start[0usize] = lr.start[0usize];
        (*lreach).start[1usize] = lr.start[1usize];
        (*lreach).start[2usize] = lr.start[2usize];
        (*lreach).end[0usize] = lr.end[0usize];
        (*lreach).end[1usize] = lr.end[1usize];
        (*lreach).end[2usize] = lr.end[2usize];
        (*lreach).traveltype = lr.traveltype;
        (*lreach).traveltime = lr.traveltime;
        (*lreach).next = *areareachability.offset(area1num as isize);
        let ref mut fresh25 = *areareachability.offset(area1num as isize);
        *fresh25 = lreach;
        if 0 == AAS_AreaCrouch(area1num) && 0 != AAS_AreaCrouch(area2num) {
            (*lreach).traveltime =
                ((*lreach).traveltime as libc::c_float +
                     aassettings.rs_startcrouch) as libc::c_ushort
        }
        reach_equalfloor += 1;
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//walk on floors with equal height
#[no_mangle]
pub static mut reach_equalfloor: libc::c_int = 0;
//end of the function AAS_SolidGapTime
//===========================================================================
// searches for swim reachabilities between adjacent areas
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Reachability_Swim(mut area1num: libc::c_int,
                                               mut area2num: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut side1: libc::c_int = 0;
    let mut area1: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut area2: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut face1: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut start: vec3_t = [0.; 3];
    if 0 == AAS_AreaSwim(area1num) || 0 == AAS_AreaSwim(area2num) {
        return qfalse as libc::c_int
    }
    if 0 ==
           (*aasworld.areasettings.offset(area2num as isize)).presencetype &
               2i32 {
        return qfalse as libc::c_int
    }
    area1 = &mut *aasworld.areas.offset(area1num as isize) as *mut aas_area_t;
    area2 = &mut *aasworld.areas.offset(area2num as isize) as *mut aas_area_t;
    i = 0i32;
    while i < 3i32 {
        if (*area1).mins[i as usize] >
               (*area2).maxs[i as usize] + 10i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        if (*area1).maxs[i as usize] <
               (*area2).mins[i as usize] - 10i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        i += 1
    }
    i = 0i32;
    while i < (*area1).numfaces {
        face1num =
            *aasworld.faceindex.offset(((*area1).firstface + i) as isize);
        side1 = (face1num < 0i32) as libc::c_int;
        face1num = abs(face1num);
        j = 0i32;
        while j < (*area2).numfaces {
            face2num =
                abs(*aasworld.faceindex.offset(((*area2).firstface + j) as
                                                   isize));
            if face1num == face2num {
                AAS_FaceCenter(face1num, start.as_mut_ptr());
                if 0 !=
                       AAS_PointContents(start.as_mut_ptr()) &
                           (8i32 | 16i32 | 32i32) {
                    face1 =
                        &mut *aasworld.faces.offset(face1num as isize) as
                            *mut aas_face_t;
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() { return qfalse as libc::c_int }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = face1num;
                    (*lreach).edgenum = 0i32;
                    (*lreach).start[0usize] = start[0usize];
                    (*lreach).start[1usize] = start[1usize];
                    (*lreach).start[2usize] = start[2usize];
                    plane =
                        &mut *aasworld.planes.offset(((*face1).planenum ^
                                                          side1) as isize) as
                            *mut aas_plane_t;
                    (*lreach).end[0usize] =
                        (*lreach).start[0usize] +
                            (*plane).normal[0usize] * -2i32 as libc::c_float;
                    (*lreach).end[1usize] =
                        (*lreach).start[1usize] +
                            (*plane).normal[1usize] * -2i32 as libc::c_float;
                    (*lreach).end[2usize] =
                        (*lreach).start[2usize] +
                            (*plane).normal[2usize] * -2i32 as libc::c_float;
                    (*lreach).traveltype = 8i32;
                    (*lreach).traveltime = 1i32 as libc::c_ushort;
                    if AAS_AreaVolume(area2num) < 800i32 as libc::c_float {
                        (*lreach).traveltime =
                            ((*lreach).traveltime as libc::c_int + 200i32) as
                                libc::c_ushort
                    }
                    (*lreach).next =
                        *areareachability.offset(area1num as isize);
                    let ref mut fresh26 =
                        *areareachability.offset(area1num as isize);
                    *fresh26 = lreach;
                    reach_swim += 1;
                    return qtrue as libc::c_int
                }
            }
            j += 1
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
//#define REACH_DEBUG
//NOTE: all travel times are in hundreth of a second
//maximum number of reachability links
//number of areas reachability is calculated for each frame
//number of units reachability points are placed inside the areas
//area flag used for weapon jumping
//valid area to weapon jump to
//number of reachabilities of each type
//swim
#[no_mangle]
pub static mut reach_swim: libc::c_int = 0;
//double jump
#[no_mangle]
pub static mut reach_doublejump: libc::c_int = 0;
//ramp jump
#[no_mangle]
pub static mut reach_rampjump: libc::c_int = 0;
//strafe jump (just normal jump but further)
#[no_mangle]
pub static mut reach_strafejump: libc::c_int = 0;
//bfg jump
#[no_mangle]
pub static mut reach_bfgjump: libc::c_int = 0;
//end of the function AAS_AreaDoNotEnter
//===========================================================================
// returns the time it takes perform a barrier jump
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_BarrierJumpTravelTime() -> libc::c_ushort {
    return (aassettings.phys_jumpvel as libc::c_double /
                (aassettings.phys_gravity as libc::c_double * 0.1f64)) as
               libc::c_ushort;
}
//end of the function AAS_ReachabilityExists
//===========================================================================
// returns true if there is a solid just after the end point when going
// from start to end
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_NearbySolidOrGap(mut start: *mut vec_t,
                                              mut end: *mut vec_t)
 -> libc::c_int {
    let mut dir: vec3_t = [0.; 3];
    let mut testpoint: vec3_t = [0.; 3];
    let mut areanum: libc::c_int = 0;
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    dir[2usize] = 0i32 as vec_t;
    VectorNormalize(dir.as_mut_ptr());
    testpoint[0usize] =
        *end.offset(0isize) + dir[0usize] * 48i32 as libc::c_float;
    testpoint[1usize] =
        *end.offset(1isize) + dir[1usize] * 48i32 as libc::c_float;
    testpoint[2usize] =
        *end.offset(2isize) + dir[2usize] * 48i32 as libc::c_float;
    areanum = AAS_PointAreaNum(testpoint.as_mut_ptr());
    if 0 == areanum {
        testpoint[2usize] += 16i32 as libc::c_float;
        areanum = AAS_PointAreaNum(testpoint.as_mut_ptr());
        if 0 == areanum { return qtrue as libc::c_int }
    }
    testpoint[0usize] =
        *end.offset(0isize) + dir[0usize] * 64i32 as libc::c_float;
    testpoint[1usize] =
        *end.offset(1isize) + dir[1usize] * 64i32 as libc::c_float;
    testpoint[2usize] =
        *end.offset(2isize) + dir[2usize] * 64i32 as libc::c_float;
    areanum = AAS_PointAreaNum(testpoint.as_mut_ptr());
    if 0 != areanum {
        if 0 == AAS_AreaSwim(areanum) && 0 == AAS_AreaGrounded(areanum) {
            return qtrue as libc::c_int
        }
    }
    return qfalse as libc::c_int;
}