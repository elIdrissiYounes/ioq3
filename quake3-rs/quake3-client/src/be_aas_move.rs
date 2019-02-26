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
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
        #[no_mangle]
        pub fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                            right: *mut vec_t, up: *mut vec_t);
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
    use super::q_shared_h::{vec3_t, byte, qboolean};
    use super::{libc};
    use super::aasfile_h::{aas_bbox_t, aas_vertex_t, aas_plane_t, aas_edge_t,
                           aas_edgeindex_t, aas_face_t, aas_faceindex_t,
                           aas_area_t, aas_areasettings_t, aas_reachability_t,
                           aas_node_t, aas_portal_t, aas_portalindex_t,
                           aas_cluster_t};
    use super::be_aas_h::{aas_entityinfo_t};
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
      "ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::{libc};
    use super::q_shared_h::{vec_t, vec3_t, qboolean};
    use super::be_aas_h::{aas_trace_t};
    use super::aasfile_h::{aas_plane_t};
    extern "C" {
        //AASINTERN
        //returns the mins and maxs of the bounding box for the given presence type
        #[no_mangle]
        pub fn AAS_PresenceTypeBoundingBox(presencetype: libc::c_int,
                                           mins: *mut vec_t,
                                           maxs: *mut vec_t);
        //returns the presence type(s) at the given point
        #[no_mangle]
        pub fn AAS_PointPresenceType(point: *mut vec_t) -> libc::c_int;
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
        pub fn AAS_PlaneFromNum(planenum: libc::c_int) -> *mut aas_plane_t;
        #[no_mangle]
        pub fn AAS_PointInsideFace(facenum: libc::c_int, point: *mut vec_t,
                                   epsilon: libc::c_float) -> qboolean;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_debug.h"]
pub mod be_aas_debug_h {
    use super::q_shared_h::{vec_t};
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
 * name:		be_aas_debug.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_debug.h $
 *
 *****************************************************************************/
        //clear the shown debug lines
        #[no_mangle]
        pub fn AAS_ClearShownDebugLines();
        //show a debug line
        #[no_mangle]
        pub fn AAS_DebugLine(start: *mut vec_t, end: *mut vec_t,
                             color: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::botlib_h::{bsp_trace_t};
    use super::q_shared_h::{vec_t};
    use super::{libc};
    extern "C" {
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
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    use super::{libc};
    use super::be_aas_h::{aas_clientmove_s};
    use super::q_shared_h::{vec_t};
    use super::be_aas_def_h::{aas_settings_t};
    use super::aasfile_h::{aas_reachability_s};
}
#[header_src =
      "ioq3/code/botlib/be_aas_move.c"]
pub mod be_aas_move_c {
    use super::{libc};
    use super::be_aas_h::{aas_clientmove_s, aas_trace_t};
    use super::q_shared_h::{vec_t};
    use super::botlib_h::{botlib_import_t};
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
 * name:		be_aas_move.c
 *
 * desc:		AAS
 *
 * $Archive: /MissionPack/code/botlib/be_aas_move.c $
 *
 *****************************************************************************/
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
                       FS_WRITE, FS_READ, cplane_t, vec3_origin,
                       VectorNormalize, AngleVectors};
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
use self::be_aas_def_h::{aas_settings_t, aas_settings_s, aas_t, aas_s,
                         aas_reachabilityareas_t, aas_reachabilityareas_s,
                         aas_routingcache_t, aas_routingcache_s,
                         aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s};
use self::mathcalls_h::{sqrt, fabsf};
use self::string_h::{memset};
use self::stdlib_h::{abs};
use self::l_libvar_h::{LibVarValue};
use self::be_aas_sample_h::{AAS_PresenceTypeBoundingBox,
                            AAS_PointPresenceType, AAS_TraceClientBBox,
                            AAS_TraceAreas, AAS_PointAreaNum,
                            AAS_PlaneFromNum, AAS_PointInsideFace};
use self::be_aas_debug_h::{AAS_ClearShownDebugLines, AAS_DebugLine};
use self::be_aas_bsp_h::{AAS_Trace, AAS_PointContents};
use self::be_aas_move_c::{botimport};
use self::be_aas_main_h::{aasworld};
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
//movement prediction
#[no_mangle]
pub unsafe extern "C" fn AAS_PredictClientMovement(mut move_0:
                                                       *mut aas_clientmove_s,
                                                   mut entnum: libc::c_int,
                                                   mut origin: *mut vec_t,
                                                   mut presencetype:
                                                       libc::c_int,
                                                   mut onground: libc::c_int,
                                                   mut velocity: *mut vec_t,
                                                   mut cmdmove: *mut vec_t,
                                                   mut cmdframes: libc::c_int,
                                                   mut maxframes: libc::c_int,
                                                   mut frametime:
                                                       libc::c_float,
                                                   mut stopevent: libc::c_int,
                                                   mut stopareanum:
                                                       libc::c_int,
                                                   mut visualize: libc::c_int)
 -> libc::c_int {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    return AAS_ClientMovementPrediction(move_0, entnum, origin, presencetype,
                                        onground, velocity, cmdmove,
                                        cmdframes, maxframes, frametime,
                                        stopevent, stopareanum,
                                        mins.as_mut_ptr(), maxs.as_mut_ptr(),
                                        visualize);
}
//end of the function AAS_ClipToBBox
//===========================================================================
// predicts the movement
// assumes regular bounding box sizes
// NOTE: out of water jumping is not included
// NOTE: grappling hook is not included
//
// Parameter:			origin			: origin to start with
//						presencetype	: presence type to start with
//						velocity		: velocity to start with
//						cmdmove			: client command movement
//						cmdframes		: number of frame cmdmove is valid
//						maxframes		: maximum number of predicted frames
//						frametime		: duration of one predicted frame
//						stopevent		: events that stop the prediction
//						stopareanum		: stop as soon as entered this area
// Returns:				aas_clientmove_t
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ClientMovementPrediction(mut move_0:
                                                          *mut aas_clientmove_s,
                                                      mut entnum: libc::c_int,
                                                      mut origin: *mut vec_t,
                                                      mut presencetype:
                                                          libc::c_int,
                                                      mut onground:
                                                          libc::c_int,
                                                      mut velocity:
                                                          *mut vec_t,
                                                      mut cmdmove: *mut vec_t,
                                                      mut cmdframes:
                                                          libc::c_int,
                                                      mut maxframes:
                                                          libc::c_int,
                                                      mut frametime:
                                                          libc::c_float,
                                                      mut stopevent:
                                                          libc::c_int,
                                                      mut stopareanum:
                                                          libc::c_int,
                                                      mut mins: *mut vec_t,
                                                      mut maxs: *mut vec_t,
                                                      mut visualize:
                                                          libc::c_int)
 -> libc::c_int {
    let mut phys_friction: libc::c_float = 0.;
    let mut phys_stopspeed: libc::c_float = 0.;
    let mut phys_gravity: libc::c_float = 0.;
    let mut phys_waterfriction: libc::c_float = 0.;
    let mut phys_watergravity: libc::c_float = 0.;
    let mut phys_walkaccelerate: libc::c_float = 0.;
    let mut phys_airaccelerate: libc::c_float = 0.;
    let mut phys_swimaccelerate: libc::c_float = 0.;
    let mut phys_maxwalkvelocity: libc::c_float = 0.;
    let mut phys_maxcrouchvelocity: libc::c_float = 0.;
    let mut phys_maxswimvelocity: libc::c_float = 0.;
    let mut phys_maxstep: libc::c_float = 0.;
    let mut phys_maxsteepness: libc::c_float = 0.;
    let mut phys_jumpvel: libc::c_float = 0.;
    let mut friction: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    let mut delta: libc::c_float = 0.;
    let mut maxvel: libc::c_float = 0.;
    let mut wishspeed: libc::c_float = 0.;
    let mut accelerate: libc::c_float = 0.;
    //float velchange, newvel;
	//int ax;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pc: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut swimming: libc::c_int = 0;
    let mut crouch: libc::c_int = 0;
    let mut event: libc::c_int = 0;
    let mut jump_frame: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut areas: [libc::c_int; 20] = [0; 20];
    let mut numareas: libc::c_int = 0;
    let mut points: [vec3_t; 20] = [[0.; 3]; 20];
    let mut org: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut feet: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut stepend: vec3_t = [0.; 3];
    let mut lastorg: vec3_t = [0.; 3];
    let mut wishdir: vec3_t = [0.; 3];
    let mut frame_test_vel: vec3_t = [0.; 3];
    let mut old_frame_test_vel: vec3_t = [0.; 3];
    let mut left_test_vel: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut plane2: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    let mut steptrace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    if frametime <= 0i32 as libc::c_float { frametime = 0.1f32 }
    phys_friction = aassettings.phys_friction;
    phys_stopspeed = aassettings.phys_stopspeed;
    phys_gravity = aassettings.phys_gravity;
    phys_waterfriction = aassettings.phys_waterfriction;
    phys_watergravity = aassettings.phys_watergravity;
    phys_maxwalkvelocity = aassettings.phys_maxwalkvelocity;
    phys_maxcrouchvelocity = aassettings.phys_maxcrouchvelocity;
    phys_maxswimvelocity = aassettings.phys_maxswimvelocity;
    phys_walkaccelerate = aassettings.phys_walkaccelerate;
    phys_airaccelerate = aassettings.phys_airaccelerate;
    phys_swimaccelerate = aassettings.phys_swimaccelerate;
    phys_maxstep = aassettings.phys_maxstep;
    phys_maxsteepness = aassettings.phys_maxsteepness;
    phys_jumpvel = aassettings.phys_jumpvel * frametime;
    memset(move_0 as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_clientmove_t>() as libc::c_ulong);
    memset(&mut trace as *mut aas_trace_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<aas_trace_t>() as libc::c_ulong);
    org[0usize] = *origin.offset(0isize);
    org[1usize] = *origin.offset(1isize);
    org[2usize] = *origin.offset(2isize);
    org[2usize] = (org[2usize] as libc::c_double + 0.25f64) as vec_t;
    frame_test_vel[0usize] = *velocity.offset(0isize) * frametime;
    frame_test_vel[1usize] = *velocity.offset(1isize) * frametime;
    frame_test_vel[2usize] = *velocity.offset(2isize) * frametime;
    jump_frame = -1i32;
    n = 0i32;
    while n < maxframes {
        swimming = AAS_Swimming(org.as_mut_ptr());
        gravity =
            if 0 != swimming { phys_watergravity } else { phys_gravity };
        frame_test_vel[2usize] =
            (frame_test_vel[2usize] as libc::c_double -
                 gravity as libc::c_double * 0.1f64 *
                     frametime as libc::c_double) as vec_t;
        if 0 != onground || 0 != swimming {
            friction =
                if 0 != swimming {
                    phys_waterfriction
                } else { phys_friction };
            frame_test_vel[0usize] =
                frame_test_vel[0usize] * (1i32 as libc::c_float / frametime);
            frame_test_vel[1usize] =
                frame_test_vel[1usize] * (1i32 as libc::c_float / frametime);
            frame_test_vel[2usize] =
                frame_test_vel[2usize] * (1i32 as libc::c_float / frametime);
            AAS_ApplyFriction(frame_test_vel.as_mut_ptr(), friction,
                              phys_stopspeed, frametime);
            frame_test_vel[0usize] = frame_test_vel[0usize] * frametime;
            frame_test_vel[1usize] = frame_test_vel[1usize] * frametime;
            frame_test_vel[2usize] = frame_test_vel[2usize] * frametime
        }
        crouch = qfalse as libc::c_int;
        if n < cmdframes {
            maxvel = phys_maxwalkvelocity;
            accelerate = phys_airaccelerate;
            wishdir[0usize] = *cmdmove.offset(0isize);
            wishdir[1usize] = *cmdmove.offset(1isize);
            wishdir[2usize] = *cmdmove.offset(2isize);
            if 0 != onground {
                if *cmdmove.offset(2isize) < -300i32 as libc::c_float {
                    crouch = qtrue as libc::c_int;
                    maxvel = phys_maxcrouchvelocity
                }
                if 0 == swimming &&
                       *cmdmove.offset(2isize) > 1i32 as libc::c_float {
                    frame_test_vel[2usize] =
                        (phys_jumpvel as libc::c_double -
                             gravity as libc::c_double * 0.1f64 *
                                 frametime as libc::c_double +
                             5i32 as libc::c_double) as vec_t;
                    jump_frame = n;
                    accelerate = phys_airaccelerate
                } else { accelerate = phys_walkaccelerate }
            }
            if 0 != swimming {
                maxvel = phys_maxswimvelocity;
                accelerate = phys_swimaccelerate
            } else { wishdir[2usize] = 0i32 as vec_t }
            wishspeed = VectorNormalize(wishdir.as_mut_ptr());
            if wishspeed > maxvel { wishspeed = maxvel }
            frame_test_vel[0usize] =
                frame_test_vel[0usize] * (1i32 as libc::c_float / frametime);
            frame_test_vel[1usize] =
                frame_test_vel[1usize] * (1i32 as libc::c_float / frametime);
            frame_test_vel[2usize] =
                frame_test_vel[2usize] * (1i32 as libc::c_float / frametime);
            AAS_Accelerate(frame_test_vel.as_mut_ptr(), frametime,
                           wishdir.as_mut_ptr(), wishspeed, accelerate);
            frame_test_vel[0usize] = frame_test_vel[0usize] * frametime;
            frame_test_vel[1usize] = frame_test_vel[1usize] * frametime;
            frame_test_vel[2usize] = frame_test_vel[2usize] * frametime
        }
        if 0 != crouch {
            presencetype = 4i32
        } else if presencetype == 4i32 {
            if 0 != AAS_PointPresenceType(org.as_mut_ptr()) & 2i32 {
                presencetype = 2i32
            }
        }
        lastorg[0usize] = org[0usize];
        lastorg[1usize] = org[1usize];
        lastorg[2usize] = org[2usize];
        left_test_vel[0usize] = frame_test_vel[0usize];
        left_test_vel[1usize] = frame_test_vel[1usize];
        left_test_vel[2usize] = frame_test_vel[2usize];
        j = 0i32;
        loop  {
            end[0usize] = org[0usize] + left_test_vel[0usize];
            end[1usize] = org[1usize] + left_test_vel[1usize];
            end[2usize] = org[2usize] + left_test_vel[2usize];
            trace =
                AAS_TraceClientBBox(org.as_mut_ptr(), end.as_mut_ptr(),
                                    presencetype, entnum);
            if 0 != visualize {
                if 0 != trace.startsolid as u64 {
                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                        b"PredictMovement: start solid\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                }
                AAS_DebugLine(org.as_mut_ptr(), trace.endpos.as_mut_ptr(),
                              1i32);
            }
            if 0 != stopevent & (512i32 | 128i32 | 256i32 | 4096i32) {
                numareas =
                    AAS_TraceAreas(org.as_mut_ptr(),
                                   trace.endpos.as_mut_ptr(),
                                   areas.as_mut_ptr(), points.as_mut_ptr(),
                                   20i32);
                i = 0i32;
                while i < numareas {
                    if 0 != stopevent & 512i32 {
                        if areas[i as usize] == stopareanum {
                            (*move_0).endpos[0usize] =
                                points[i as usize][0usize];
                            (*move_0).endpos[1usize] =
                                points[i as usize][1usize];
                            (*move_0).endpos[2usize] =
                                points[i as usize][2usize];
                            (*move_0).velocity[0usize] =
                                frame_test_vel[0usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[1usize] =
                                frame_test_vel[1usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[2usize] =
                                frame_test_vel[2usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 512i32;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0i32;
                            (*move_0).time = n as libc::c_float * frametime;
                            (*move_0).frames = n;
                            return qtrue as libc::c_int
                        }
                    }
                    if 0 != stopevent & 128i32 && 0 != n {
                        if 0 !=
                               (*aasworld.areasettings.offset(areas[i as
                                                                        usize]
                                                                  as
                                                                  isize)).contents
                                   & 128i32 {
                            (*move_0).endpos[0usize] =
                                points[i as usize][0usize];
                            (*move_0).endpos[1usize] =
                                points[i as usize][1usize];
                            (*move_0).endpos[2usize] =
                                points[i as usize][2usize];
                            (*move_0).velocity[0usize] =
                                frame_test_vel[0usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[1usize] =
                                frame_test_vel[1usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[2usize] =
                                frame_test_vel[2usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 128i32;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0i32;
                            (*move_0).time = n as libc::c_float * frametime;
                            (*move_0).frames = n;
                            return qtrue as libc::c_int
                        }
                    }
                    if 0 != stopevent & 256i32 {
                        if 0 !=
                               (*aasworld.areasettings.offset(areas[i as
                                                                        usize]
                                                                  as
                                                                  isize)).contents
                                   & 64i32 {
                            (*move_0).endpos[0usize] =
                                points[i as usize][0usize];
                            (*move_0).endpos[1usize] =
                                points[i as usize][1usize];
                            (*move_0).endpos[2usize] =
                                points[i as usize][2usize];
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).velocity[0usize] =
                                frame_test_vel[0usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[1usize] =
                                frame_test_vel[1usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[2usize] =
                                frame_test_vel[2usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 256i32;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0i32;
                            (*move_0).time = n as libc::c_float * frametime;
                            (*move_0).frames = n;
                            return qtrue as libc::c_int
                        }
                    }
                    if 0 != stopevent & 4096i32 {
                        if 0 !=
                               (*aasworld.areasettings.offset(areas[i as
                                                                        usize]
                                                                  as
                                                                  isize)).contents
                                   & 8i32 {
                            (*move_0).endpos[0usize] =
                                points[i as usize][0usize];
                            (*move_0).endpos[1usize] =
                                points[i as usize][1usize];
                            (*move_0).endpos[2usize] =
                                points[i as usize][2usize];
                            (*move_0).endarea = areas[i as usize];
                            (*move_0).velocity[0usize] =
                                frame_test_vel[0usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[1usize] =
                                frame_test_vel[1usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[2usize] =
                                frame_test_vel[2usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 4096i32;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0i32;
                            (*move_0).time = n as libc::c_float * frametime;
                            (*move_0).frames = n;
                            return qtrue as libc::c_int
                        }
                    }
                    i += 1
                }
            }
            if 0 != stopevent & 2048i32 {
                if 0 !=
                       AAS_ClipToBBox(&mut trace, org.as_mut_ptr(),
                                      trace.endpos.as_mut_ptr(), presencetype,
                                      mins, maxs) {
                    (*move_0).endpos[0usize] = trace.endpos[0usize];
                    (*move_0).endpos[1usize] = trace.endpos[1usize];
                    (*move_0).endpos[2usize] = trace.endpos[2usize];
                    (*move_0).endarea =
                        AAS_PointAreaNum((*move_0).endpos.as_mut_ptr());
                    (*move_0).velocity[0usize] =
                        frame_test_vel[0usize] *
                            (1i32 as libc::c_float / frametime);
                    (*move_0).velocity[1usize] =
                        frame_test_vel[1usize] *
                            (1i32 as libc::c_float / frametime);
                    (*move_0).velocity[2usize] =
                        frame_test_vel[2usize] *
                            (1i32 as libc::c_float / frametime);
                    (*move_0).trace = trace;
                    (*move_0).stopevent = 2048i32;
                    (*move_0).presencetype = presencetype;
                    (*move_0).endcontents = 0i32;
                    (*move_0).time = n as libc::c_float * frametime;
                    (*move_0).frames = n;
                    return qtrue as libc::c_int
                }
            }
            org[0usize] = trace.endpos[0usize];
            org[1usize] = trace.endpos[1usize];
            org[2usize] = trace.endpos[2usize];
            if (trace.fraction as libc::c_double) < 1.0f64 {
                plane = AAS_PlaneFromNum(trace.planenum);
                if 0 != stopevent & 1024i32 {
                    if (*plane).normal[0usize] * up[0usize] +
                           (*plane).normal[1usize] * up[1usize] +
                           (*plane).normal[2usize] * up[2usize] >
                           phys_maxsteepness {
                        start[0usize] = org[0usize];
                        start[1usize] = org[1usize];
                        start[2usize] = org[2usize];
                        start[2usize] =
                            (start[2usize] as libc::c_double + 0.5f64) as
                                vec_t;
                        if AAS_PointAreaNum(start.as_mut_ptr()) == stopareanum
                           {
                            (*move_0).endpos[0usize] = start[0usize];
                            (*move_0).endpos[1usize] = start[1usize];
                            (*move_0).endpos[2usize] = start[2usize];
                            (*move_0).endarea = stopareanum;
                            (*move_0).velocity[0usize] =
                                frame_test_vel[0usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[1usize] =
                                frame_test_vel[1usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[2usize] =
                                frame_test_vel[2usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 1024i32;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0i32;
                            (*move_0).time = n as libc::c_float * frametime;
                            (*move_0).frames = n;
                            return qtrue as libc::c_int
                        }
                    }
                }
                step = qfalse as libc::c_int;
                if (*plane).normal[2usize] == 0i32 as libc::c_float &&
                       (jump_frame < 0i32 || n - jump_frame > 2i32) {
                    start[0usize] =
                        (org[0usize] as libc::c_double +
                             (*plane).normal[0usize] as libc::c_double *
                                 -0.25f64) as vec_t;
                    start[1usize] =
                        (org[1usize] as libc::c_double +
                             (*plane).normal[1usize] as libc::c_double *
                                 -0.25f64) as vec_t;
                    start[2usize] =
                        (org[2usize] as libc::c_double +
                             (*plane).normal[2usize] as libc::c_double *
                                 -0.25f64) as vec_t;
                    stepend[0usize] = start[0usize];
                    stepend[1usize] = start[1usize];
                    stepend[2usize] = start[2usize];
                    start[2usize] += phys_maxstep;
                    steptrace =
                        AAS_TraceClientBBox(start.as_mut_ptr(),
                                            stepend.as_mut_ptr(),
                                            presencetype, entnum);
                    if 0 == steptrace.startsolid as u64 {
                        plane2 = AAS_PlaneFromNum(steptrace.planenum);
                        if (*plane2).normal[0usize] * up[0usize] +
                               (*plane2).normal[1usize] * up[1usize] +
                               (*plane2).normal[2usize] * up[2usize] >
                               phys_maxsteepness {
                            left_test_vel[0usize] =
                                end[0usize] - steptrace.endpos[0usize];
                            left_test_vel[1usize] =
                                end[1usize] - steptrace.endpos[1usize];
                            left_test_vel[2usize] =
                                end[2usize] - steptrace.endpos[2usize];
                            left_test_vel[2usize] = 0i32 as vec_t;
                            frame_test_vel[2usize] = 0i32 as vec_t;
                            if 0 != visualize {
                                if (steptrace.endpos[2usize] - org[2usize]) as
                                       libc::c_double > 0.125f64 {
                                    start[0usize] = org[0usize];
                                    start[1usize] = org[1usize];
                                    start[2usize] = org[2usize];
                                    start[2usize] = steptrace.endpos[2usize];
                                    AAS_DebugLine(org.as_mut_ptr(),
                                                  start.as_mut_ptr(), 3i32);
                                }
                            }
                            org[2usize] = steptrace.endpos[2usize];
                            step = qtrue as libc::c_int
                        }
                    }
                }
                if 0 == step {
                    left_test_vel[0usize] =
                        left_test_vel[0usize] +
                            (*plane).normal[0usize] *
                                -(left_test_vel[0usize] *
                                      (*plane).normal[0usize] +
                                      left_test_vel[1usize] *
                                          (*plane).normal[1usize] +
                                      left_test_vel[2usize] *
                                          (*plane).normal[2usize]);
                    left_test_vel[1usize] =
                        left_test_vel[1usize] +
                            (*plane).normal[1usize] *
                                -(left_test_vel[0usize] *
                                      (*plane).normal[0usize] +
                                      left_test_vel[1usize] *
                                          (*plane).normal[1usize] +
                                      left_test_vel[2usize] *
                                          (*plane).normal[2usize]);
                    left_test_vel[2usize] =
                        left_test_vel[2usize] +
                            (*plane).normal[2usize] *
                                -(left_test_vel[0usize] *
                                      (*plane).normal[0usize] +
                                      left_test_vel[1usize] *
                                          (*plane).normal[1usize] +
                                      left_test_vel[2usize] *
                                          (*plane).normal[2usize]);
                    old_frame_test_vel[0usize] = frame_test_vel[0usize];
                    old_frame_test_vel[1usize] = frame_test_vel[1usize];
                    old_frame_test_vel[2usize] = frame_test_vel[2usize];
                    frame_test_vel[0usize] =
                        frame_test_vel[0usize] +
                            (*plane).normal[0usize] *
                                -(frame_test_vel[0usize] *
                                      (*plane).normal[0usize] +
                                      frame_test_vel[1usize] *
                                          (*plane).normal[1usize] +
                                      frame_test_vel[2usize] *
                                          (*plane).normal[2usize]);
                    frame_test_vel[1usize] =
                        frame_test_vel[1usize] +
                            (*plane).normal[1usize] *
                                -(frame_test_vel[0usize] *
                                      (*plane).normal[0usize] +
                                      frame_test_vel[1usize] *
                                          (*plane).normal[1usize] +
                                      frame_test_vel[2usize] *
                                          (*plane).normal[2usize]);
                    frame_test_vel[2usize] =
                        frame_test_vel[2usize] +
                            (*plane).normal[2usize] *
                                -(frame_test_vel[0usize] *
                                      (*plane).normal[0usize] +
                                      frame_test_vel[1usize] *
                                          (*plane).normal[1usize] +
                                      frame_test_vel[2usize] *
                                          (*plane).normal[2usize]);
                    if (*plane).normal[0usize] * up[0usize] +
                           (*plane).normal[1usize] * up[1usize] +
                           (*plane).normal[2usize] * up[2usize] >
                           phys_maxsteepness {
                        onground = qtrue as libc::c_int
                    }
                    if 0 != stopevent & 32i32 {
                        delta = 0i32 as libc::c_float;
                        if old_frame_test_vel[2usize] < 0i32 as libc::c_float
                               &&
                               frame_test_vel[2usize] >
                                   old_frame_test_vel[2usize] && 0 == onground
                           {
                            delta = old_frame_test_vel[2usize]
                        } else if 0 != onground {
                            delta =
                                frame_test_vel[2usize] -
                                    old_frame_test_vel[2usize]
                        }
                        if 0. != delta {
                            delta = delta * 10i32 as libc::c_float;
                            delta =
                                ((delta * delta) as libc::c_double *
                                     0.0001f64) as libc::c_float;
                            if 0 != swimming { delta = 0i32 as libc::c_float }
                            if delta > 40i32 as libc::c_float {
                                (*move_0).endpos[0usize] = org[0usize];
                                (*move_0).endpos[1usize] = org[1usize];
                                (*move_0).endpos[2usize] = org[2usize];
                                (*move_0).endarea =
                                    AAS_PointAreaNum(org.as_mut_ptr());
                                (*move_0).velocity[0usize] =
                                    frame_test_vel[0usize];
                                (*move_0).velocity[1usize] =
                                    frame_test_vel[1usize];
                                (*move_0).velocity[2usize] =
                                    frame_test_vel[2usize];
                                (*move_0).trace = trace;
                                (*move_0).stopevent = 32i32;
                                (*move_0).presencetype = presencetype;
                                (*move_0).endcontents = 0i32;
                                (*move_0).time =
                                    n as libc::c_float * frametime;
                                (*move_0).frames = n;
                                return qtrue as libc::c_int
                            }
                        }
                    }
                }
            }
            j += 1;
            if j > 20i32 { return qfalse as libc::c_int }
            if !((trace.fraction as libc::c_double) < 1.0f64) { break ; }
        }
        if frame_test_vel[2usize] <= 10i32 as libc::c_float {
            feet[0usize] = org[0usize];
            feet[1usize] = org[1usize];
            feet[2usize] = org[2usize];
            feet[2usize] -= 22i32 as libc::c_float;
            pc = AAS_PointContents(feet.as_mut_ptr());
            event = 0i32;
            if 0 != pc & 8i32 { event |= 16i32 }
            if 0 != pc & 16i32 { event |= 8i32 }
            if 0 != pc & 32i32 { event |= 4i32 }
            areanum = AAS_PointAreaNum(org.as_mut_ptr());
            if 0 !=
                   (*aasworld.areasettings.offset(areanum as isize)).contents
                       & 2i32 {
                event |= 16i32
            }
            if 0 !=
                   (*aasworld.areasettings.offset(areanum as isize)).contents
                       & 4i32 {
                event |= 8i32
            }
            if 0 !=
                   (*aasworld.areasettings.offset(areanum as isize)).contents
                       & 1i32 {
                event |= 4i32
            }
            if 0 != event & stopevent {
                (*move_0).endpos[0usize] = org[0usize];
                (*move_0).endpos[1usize] = org[1usize];
                (*move_0).endpos[2usize] = org[2usize];
                (*move_0).endarea = areanum;
                (*move_0).velocity[0usize] =
                    frame_test_vel[0usize] *
                        (1i32 as libc::c_float / frametime);
                (*move_0).velocity[1usize] =
                    frame_test_vel[1usize] *
                        (1i32 as libc::c_float / frametime);
                (*move_0).velocity[2usize] =
                    frame_test_vel[2usize] *
                        (1i32 as libc::c_float / frametime);
                (*move_0).stopevent = event & stopevent;
                (*move_0).presencetype = presencetype;
                (*move_0).endcontents = pc;
                (*move_0).time = n as libc::c_float * frametime;
                (*move_0).frames = n;
                return qtrue as libc::c_int
            }
        }
        onground = AAS_OnGround(org.as_mut_ptr(), presencetype, entnum);
        if 0 != onground {
            if 0 != stopevent & 1i32 {
                (*move_0).endpos[0usize] = org[0usize];
                (*move_0).endpos[1usize] = org[1usize];
                (*move_0).endpos[2usize] = org[2usize];
                (*move_0).endarea = AAS_PointAreaNum(org.as_mut_ptr());
                (*move_0).velocity[0usize] =
                    frame_test_vel[0usize] *
                        (1i32 as libc::c_float / frametime);
                (*move_0).velocity[1usize] =
                    frame_test_vel[1usize] *
                        (1i32 as libc::c_float / frametime);
                (*move_0).velocity[2usize] =
                    frame_test_vel[2usize] *
                        (1i32 as libc::c_float / frametime);
                (*move_0).trace = trace;
                (*move_0).stopevent = 1i32;
                (*move_0).presencetype = presencetype;
                (*move_0).endcontents = 0i32;
                (*move_0).time = n as libc::c_float * frametime;
                (*move_0).frames = n;
                return qtrue as libc::c_int
            }
        } else if 0 != stopevent & 2i32 {
            (*move_0).endpos[0usize] = org[0usize];
            (*move_0).endpos[1usize] = org[1usize];
            (*move_0).endpos[2usize] = org[2usize];
            (*move_0).endarea = AAS_PointAreaNum(org.as_mut_ptr());
            (*move_0).velocity[0usize] =
                frame_test_vel[0usize] * (1i32 as libc::c_float / frametime);
            (*move_0).velocity[1usize] =
                frame_test_vel[1usize] * (1i32 as libc::c_float / frametime);
            (*move_0).velocity[2usize] =
                frame_test_vel[2usize] * (1i32 as libc::c_float / frametime);
            (*move_0).trace = trace;
            (*move_0).stopevent = 2i32;
            (*move_0).presencetype = presencetype;
            (*move_0).endcontents = 0i32;
            (*move_0).time = n as libc::c_float * frametime;
            (*move_0).frames = n;
            return qtrue as libc::c_int
        } else {
            if 0 != stopevent & 64i32 {
                let mut gaptrace: aas_trace_t =
                    aas_trace_s{startsolid: qfalse,
                                fraction: 0.,
                                endpos: [0.; 3],
                                ent: 0,
                                lastarea: 0,
                                area: 0,
                                planenum: 0,};
                start[0usize] = org[0usize];
                start[1usize] = org[1usize];
                start[2usize] = org[2usize];
                end[0usize] = start[0usize];
                end[1usize] = start[1usize];
                end[2usize] = start[2usize];
                end[2usize] -=
                    48i32 as libc::c_float + aassettings.phys_maxbarrier;
                gaptrace =
                    AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(),
                                        4i32, -1i32);
                if 0 == gaptrace.startsolid as u64 {
                    if gaptrace.endpos[2usize] <
                           org[2usize] - aassettings.phys_maxstep -
                               1i32 as libc::c_float {
                        if 0 == AAS_PointContents(end.as_mut_ptr()) & 32i32 {
                            (*move_0).endpos[0usize] = lastorg[0usize];
                            (*move_0).endpos[1usize] = lastorg[1usize];
                            (*move_0).endpos[2usize] = lastorg[2usize];
                            (*move_0).endarea =
                                AAS_PointAreaNum(lastorg.as_mut_ptr());
                            (*move_0).velocity[0usize] =
                                frame_test_vel[0usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[1usize] =
                                frame_test_vel[1usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).velocity[2usize] =
                                frame_test_vel[2usize] *
                                    (1i32 as libc::c_float / frametime);
                            (*move_0).trace = trace;
                            (*move_0).stopevent = 64i32;
                            (*move_0).presencetype = presencetype;
                            (*move_0).endcontents = 0i32;
                            (*move_0).time = n as libc::c_float * frametime;
                            (*move_0).frames = n;
                            return qtrue as libc::c_int
                        }
                    }
                }
            }
        }
        n += 1
    }
    (*move_0).endpos[0usize] = org[0usize];
    (*move_0).endpos[1usize] = org[1usize];
    (*move_0).endpos[2usize] = org[2usize];
    (*move_0).endarea = AAS_PointAreaNum(org.as_mut_ptr());
    (*move_0).velocity[0usize] =
        frame_test_vel[0usize] * (1i32 as libc::c_float / frametime);
    (*move_0).velocity[1usize] =
        frame_test_vel[1usize] * (1i32 as libc::c_float / frametime);
    (*move_0).velocity[2usize] =
        frame_test_vel[2usize] * (1i32 as libc::c_float / frametime);
    (*move_0).stopevent = 0i32;
    (*move_0).presencetype = presencetype;
    (*move_0).endcontents = 0i32;
    (*move_0).time = n as libc::c_float * frametime;
    (*move_0).frames = n;
    return qtrue as libc::c_int;
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
 * name:		be_aas_move.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_move.h $
 *
 *****************************************************************************/
#[no_mangle]
pub static mut aassettings: aas_settings_t =
    aas_settings_s{phys_gravitydirection: [0.; 3],
                   phys_friction: 0.,
                   phys_stopspeed: 0.,
                   phys_gravity: 0.,
                   phys_waterfriction: 0.,
                   phys_watergravity: 0.,
                   phys_maxvelocity: 0.,
                   phys_maxwalkvelocity: 0.,
                   phys_maxcrouchvelocity: 0.,
                   phys_maxswimvelocity: 0.,
                   phys_walkaccelerate: 0.,
                   phys_airaccelerate: 0.,
                   phys_swimaccelerate: 0.,
                   phys_maxstep: 0.,
                   phys_maxsteepness: 0.,
                   phys_maxwaterjump: 0.,
                   phys_maxbarrier: 0.,
                   phys_jumpvel: 0.,
                   phys_falldelta5: 0.,
                   phys_falldelta10: 0.,
                   rs_waterjump: 0.,
                   rs_teleport: 0.,
                   rs_barrierjump: 0.,
                   rs_startcrouch: 0.,
                   rs_startgrapple: 0.,
                   rs_startwalkoffledge: 0.,
                   rs_startjump: 0.,
                   rs_rocketjump: 0.,
                   rs_bfgjump: 0.,
                   rs_jumppad: 0.,
                   rs_aircontrolledjumppad: 0.,
                   rs_funcbob: 0.,
                   rs_startelevator: 0.,
                   rs_falldamage5: 0.,
                   rs_falldamage10: 0.,
                   rs_maxfallheight: 0.,
                   rs_maxjumpfallheight: 0.,};
//returns true if on the ground at the given origin
#[no_mangle]
pub unsafe extern "C" fn AAS_OnGround(mut origin: *mut vec_t,
                                      mut presencetype: libc::c_int,
                                      mut passent: libc::c_int)
 -> libc::c_int {
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    let mut end: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    end[2usize] -= 10i32 as libc::c_float;
    trace =
        AAS_TraceClientBBox(origin, end.as_mut_ptr(), presencetype, passent);
    if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
    if trace.fraction as libc::c_double >= 1.0f64 {
        return qfalse as libc::c_int
    }
    if *origin.offset(2isize) - trace.endpos[2usize] > 10i32 as libc::c_float
       {
        return qfalse as libc::c_int
    }
    plane = AAS_PlaneFromNum(trace.planenum);
    if (*plane).normal[0usize] * up[0usize] +
           (*plane).normal[1usize] * up[1usize] +
           (*plane).normal[2usize] * up[2usize] <
           aassettings.phys_maxsteepness {
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//end if
//end of the function AAS_ApplyFriction
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ClipToBBox(mut trace: *mut aas_trace_t,
                                        mut start: *mut vec_t,
                                        mut end: *mut vec_t,
                                        mut presencetype: libc::c_int,
                                        mut mins: *mut vec_t,
                                        mut maxs: *mut vec_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut planedist: libc::c_float = 0.;
    let mut bboxmins: vec3_t = [0.; 3];
    let mut bboxmaxs: vec3_t = [0.; 3];
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut mid: vec3_t = [0.; 3];
    AAS_PresenceTypeBoundingBox(presencetype, bboxmins.as_mut_ptr(),
                                bboxmaxs.as_mut_ptr());
    absmins[0usize] = *mins.offset(0isize) - bboxmaxs[0usize];
    absmins[1usize] = *mins.offset(1isize) - bboxmaxs[1usize];
    absmins[2usize] = *mins.offset(2isize) - bboxmaxs[2usize];
    absmaxs[0usize] = *maxs.offset(0isize) - bboxmins[0usize];
    absmaxs[1usize] = *maxs.offset(1isize) - bboxmins[1usize];
    absmaxs[2usize] = *maxs.offset(2isize) - bboxmins[2usize];
    (*trace).endpos[0usize] = *end.offset(0isize);
    (*trace).endpos[1usize] = *end.offset(1isize);
    (*trace).endpos[2usize] = *end.offset(2isize);
    (*trace).fraction = 1i32 as libc::c_float;
    i = 0i32;
    while i < 3i32 {
        if *start.offset(i as isize) < absmins[i as usize] &&
               *end.offset(i as isize) < absmins[i as usize] {
            return qfalse as libc::c_int
        }
        if *start.offset(i as isize) > absmaxs[i as usize] &&
               *end.offset(i as isize) > absmaxs[i as usize] {
            return qfalse as libc::c_int
        }
        i += 1
    }
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    frac = 1i32 as libc::c_float;
    i = 0i32;
    while i < 3i32 {
        if dir[i as usize] > 0i32 as libc::c_float {
            planedist = absmins[i as usize]
        } else { planedist = absmaxs[i as usize] }
        front = *start.offset(i as isize) - planedist;
        back = *end.offset(i as isize) - planedist;
        frac = front / (front - back);
        side = i + 1i32;
        if side > 2i32 { side = 0i32 }
        mid[side as usize] =
            *start.offset(side as isize) + dir[side as usize] * frac;
        if mid[side as usize] > absmins[side as usize] &&
               mid[side as usize] < absmaxs[side as usize] {
            side += 1;
            if side > 2i32 { side = 0i32 }
            mid[side as usize] =
                *start.offset(side as isize) + dir[side as usize] * frac;
            if mid[side as usize] > absmins[side as usize] &&
                   mid[side as usize] < absmaxs[side as usize] {
                mid[i as usize] = planedist;
                break ;
            }
        }
        i += 1
    }
    if i != 3i32 {
        (*trace).startsolid = qfalse;
        (*trace).fraction = frac;
        (*trace).ent = 0i32;
        (*trace).planenum = 0i32;
        (*trace).area = 0i32;
        (*trace).lastarea = 0i32;
        j = 0i32;
        while j < 3i32 {
            (*trace).endpos[j as usize] =
                *start.offset(j as isize) + dir[j as usize] * frac;
            j += 1
        }
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//end of the function AAS_BFGJumpZVelocity
//===========================================================================
// applies ground friction to the given velocity
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_Accelerate(mut velocity: *mut vec_t,
                                        mut frametime: libc::c_float,
                                        mut wishdir: *mut vec_t,
                                        mut wishspeed: libc::c_float,
                                        mut accel: libc::c_float) {
    // q2 style
    let mut i: libc::c_int = 0;
    let mut addspeed: libc::c_float = 0.;
    let mut accelspeed: libc::c_float = 0.;
    let mut currentspeed: libc::c_float = 0.;
    currentspeed =
        *velocity.offset(0isize) * *wishdir.offset(0isize) +
            *velocity.offset(1isize) * *wishdir.offset(1isize) +
            *velocity.offset(2isize) * *wishdir.offset(2isize);
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0i32 as libc::c_float { return }
    accelspeed = accel * frametime * wishspeed;
    if accelspeed > addspeed { accelspeed = addspeed }
    i = 0i32;
    while i < 3i32 {
        let ref mut fresh0 = *velocity.offset(i as isize);
        *fresh0 += accelspeed * *wishdir.offset(i as isize);
        i += 1
    };
}
//end of the function AAS_Accelerate
//===========================================================================
// applies ground friction to the given velocity
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ApplyFriction(mut vel: *mut vec_t,
                                           mut friction: libc::c_float,
                                           mut stopspeed: libc::c_float,
                                           mut frametime: libc::c_float) {
    let mut speed: libc::c_float = 0.;
    let mut control: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    speed =
        sqrt((*vel.offset(0isize) * *vel.offset(0isize) +
                  *vel.offset(1isize) * *vel.offset(1isize)) as
                 libc::c_double) as libc::c_float;
    if 0. != speed {
        control = if speed < stopspeed { stopspeed } else { speed };
        newspeed = speed - frametime * control * friction;
        if newspeed < 0i32 as libc::c_float {
            newspeed = 0i32 as libc::c_float
        }
        newspeed /= speed;
        let ref mut fresh1 = *vel.offset(0isize);
        *fresh1 *= newspeed;
        let ref mut fresh2 = *vel.offset(1isize);
        *fresh2 *= newspeed
    };
}
//returns true if swimming at the given origin
#[no_mangle]
pub unsafe extern "C" fn AAS_Swimming(mut origin: *mut vec_t) -> libc::c_int {
    let mut testorg: vec3_t = [0.; 3];
    testorg[0usize] = *origin.offset(0isize);
    testorg[1usize] = *origin.offset(1isize);
    testorg[2usize] = *origin.offset(2isize);
    testorg[2usize] -= 2i32 as libc::c_float;
    if 0 != AAS_PointContents(testorg.as_mut_ptr()) & (8i32 | 16i32 | 32i32) {
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//predict movement until bounding box is hit
#[no_mangle]
pub unsafe extern "C" fn AAS_ClientMovementHitBBox(mut move_0:
                                                       *mut aas_clientmove_s,
                                                   mut entnum: libc::c_int,
                                                   mut origin: *mut vec_t,
                                                   mut presencetype:
                                                       libc::c_int,
                                                   mut onground: libc::c_int,
                                                   mut velocity: *mut vec_t,
                                                   mut cmdmove: *mut vec_t,
                                                   mut cmdframes: libc::c_int,
                                                   mut maxframes: libc::c_int,
                                                   mut frametime:
                                                       libc::c_float,
                                                   mut mins: *mut vec_t,
                                                   mut maxs: *mut vec_t,
                                                   mut visualize: libc::c_int)
 -> libc::c_int {
    return AAS_ClientMovementPrediction(move_0, entnum, origin, presencetype,
                                        onground, velocity, cmdmove,
                                        cmdframes, maxframes, frametime,
                                        2048i32, 0i32, mins, maxs, visualize);
}
//returns the jump reachability run start point
#[no_mangle]
pub unsafe extern "C" fn AAS_JumpReachRunStart(mut reach:
                                                   *mut aas_reachability_t,
                                               mut runstart: *mut vec_t) {
    let mut hordir: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
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
    hordir[0usize] = (*reach).start[0usize] - (*reach).end[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*reach).end[1usize];
    hordir[2usize] = 0i32 as vec_t;
    VectorNormalize(hordir.as_mut_ptr());
    start[0usize] = (*reach).start[0usize];
    start[1usize] = (*reach).start[1usize];
    start[2usize] = (*reach).start[2usize];
    start[2usize] += 1i32 as libc::c_float;
    cmdmove[0usize] = hordir[0usize] * 400i32 as libc::c_float;
    cmdmove[1usize] = hordir[1usize] * 400i32 as libc::c_float;
    cmdmove[2usize] = hordir[2usize] * 400i32 as libc::c_float;
    AAS_PredictClientMovement(&mut move_0, -1i32, start.as_mut_ptr(), 2i32,
                              qtrue as libc::c_int, vec3_origin.as_mut_ptr(),
                              cmdmove.as_mut_ptr(), 1i32, 2i32, 0.1f32,
                              4i32 | 8i32 | 16i32 | 32i32 | 64i32, 0i32,
                              qfalse as libc::c_int);
    *runstart.offset(0isize) = move_0.endpos[0usize];
    *runstart.offset(1isize) = move_0.endpos[1usize];
    *runstart.offset(2isize) = move_0.endpos[2usize];
    if 0 != move_0.stopevent & (8i32 | 16i32 | 32i32) {
        *runstart.offset(0isize) = start[0usize];
        *runstart.offset(1isize) = start[1usize];
        *runstart.offset(2isize) = start[2usize]
    };
}
//returns true if against a ladder at the given origin
#[no_mangle]
pub unsafe extern "C" fn AAS_AgainstLadder(mut origin: *mut vec_t)
 -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut org: vec3_t = [0.; 3];
    let mut plane: *mut aas_plane_t = 0 as *mut aas_plane_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    org[0usize] = *origin.offset(0isize);
    org[1usize] = *origin.offset(1isize);
    org[2usize] = *origin.offset(2isize);
    areanum = AAS_PointAreaNum(org.as_mut_ptr());
    if 0 == areanum {
        org[0usize] += 1i32 as libc::c_float;
        areanum = AAS_PointAreaNum(org.as_mut_ptr());
        if 0 == areanum {
            org[1usize] += 1i32 as libc::c_float;
            areanum = AAS_PointAreaNum(org.as_mut_ptr());
            if 0 == areanum {
                org[0usize] -= 2i32 as libc::c_float;
                areanum = AAS_PointAreaNum(org.as_mut_ptr());
                if 0 == areanum {
                    org[1usize] -= 2i32 as libc::c_float;
                    areanum = AAS_PointAreaNum(org.as_mut_ptr())
                }
            }
        }
    }
    if 0 == areanum { return qfalse as libc::c_int }
    if 0 == (*aasworld.areasettings.offset(areanum as isize)).areaflags & 2i32
       {
        return qfalse as libc::c_int
    }
    if 0 ==
           (*aasworld.areasettings.offset(areanum as isize)).presencetype &
               2i32 {
        return qfalse as libc::c_int
    }
    area = &mut *aasworld.areas.offset(areanum as isize) as *mut aas_area_t;
    i = 0i32;
    while i < (*area).numfaces {
        facenum =
            *aasworld.faceindex.offset(((*area).firstface + i) as isize);
        side = (facenum < 0i32) as libc::c_int;
        face =
            &mut *aasworld.faces.offset(abs(facenum) as isize) as
                *mut aas_face_t;
        //if the face isn't a ladder face
        if !(0 == (*face).faceflags & 2i32) {
            plane =
                &mut *aasworld.planes.offset(((*face).planenum ^ side) as
                                                 isize) as *mut aas_plane_t;
            if fabsf((*plane).normal[0usize] * *origin.offset(0isize) +
                         (*plane).normal[1usize] * *origin.offset(1isize) +
                         (*plane).normal[2usize] * *origin.offset(2isize) -
                         (*plane).dist) < 3i32 as libc::c_float {
                if 0 !=
                       AAS_PointInsideFace(abs(facenum), origin, 0.1f32) as
                           u64 {
                    return qtrue as libc::c_int
                }
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
//rocket jump Z velocity when rocket-jumping at origin
#[no_mangle]
pub unsafe extern "C" fn AAS_RocketJumpZVelocity(mut origin: *mut vec_t)
 -> libc::c_float {
    return AAS_WeaponJumpZVelocity(origin, 120i32 as libc::c_float);
}
//end if
//end of the function AAS_JumpReachRunStart
//===========================================================================
// returns the Z velocity when rocket jumping at the origin
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_WeaponJumpZVelocity(mut origin: *mut vec_t,
                                                 mut radiusdamage:
                                                     libc::c_float)
 -> libc::c_float {
    let mut kvel: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut viewangles: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut mass: libc::c_float = 0.;
    let mut knockback: libc::c_float = 0.;
    let mut points: libc::c_float = 0.;
    let mut rocketoffset: vec3_t =
        [8i32 as vec_t, 8i32 as vec_t, -8i32 as vec_t];
    let mut botmins: vec3_t =
        [-16i32 as vec_t, -16i32 as vec_t, -24i32 as vec_t];
    let mut botmaxs: vec3_t =
        [16i32 as vec_t, 16i32 as vec_t, 32i32 as vec_t];
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
    viewangles[0usize] = 90i32 as vec_t;
    viewangles[1usize] = 0i32 as vec_t;
    viewangles[2usize] = 0i32 as vec_t;
    start[0usize] = *origin.offset(0isize);
    start[1usize] = *origin.offset(1isize);
    start[2usize] = *origin.offset(2isize);
    start[2usize] += 8i32 as libc::c_float;
    AngleVectors(viewangles.as_mut_ptr() as *const vec_t,
                 forward.as_mut_ptr(), right.as_mut_ptr(), 0 as *mut vec_t);
    start[0usize] +=
        forward[0usize] * rocketoffset[0usize] +
            right[0usize] * rocketoffset[1usize];
    start[1usize] +=
        forward[1usize] * rocketoffset[0usize] +
            right[1usize] * rocketoffset[1usize];
    start[2usize] +=
        forward[2usize] * rocketoffset[0usize] +
            right[2usize] * rocketoffset[1usize] + rocketoffset[2usize];
    end[0usize] = start[0usize] + forward[0usize] * 500i32 as libc::c_float;
    end[1usize] = start[1usize] + forward[1usize] * 500i32 as libc::c_float;
    end[2usize] = start[2usize] + forward[2usize] * 500i32 as libc::c_float;
    bsptrace =
        AAS_Trace(start.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t,
                  end.as_mut_ptr(), 1i32, 1i32);
    v[0usize] = botmins[0usize] + botmaxs[0usize];
    v[1usize] = botmins[1usize] + botmaxs[1usize];
    v[2usize] = botmins[2usize] + botmaxs[2usize];
    v[0usize] =
        (*origin.offset(0isize) as libc::c_double +
             v[0usize] as libc::c_double * 0.5f64) as vec_t;
    v[1usize] =
        (*origin.offset(1isize) as libc::c_double +
             v[1usize] as libc::c_double * 0.5f64) as vec_t;
    v[2usize] =
        (*origin.offset(2isize) as libc::c_double +
             v[2usize] as libc::c_double * 0.5f64) as vec_t;
    v[0usize] = bsptrace.endpos[0usize] - v[0usize];
    v[1usize] = bsptrace.endpos[1usize] - v[1usize];
    v[2usize] = bsptrace.endpos[2usize] - v[2usize];
    points =
        (radiusdamage as libc::c_double -
             0.5f64 *
                 VectorLength(v.as_mut_ptr() as *const vec_t) as
                     libc::c_double) as libc::c_float;
    if points < 0i32 as libc::c_float { points = 0i32 as libc::c_float }
    points = (points as libc::c_double * 0.5f64) as libc::c_float;
    mass = 200i32 as libc::c_float;
    knockback = points;
    dir[0usize] = *origin.offset(0isize) - bsptrace.endpos[0usize];
    dir[1usize] = *origin.offset(1isize) - bsptrace.endpos[1usize];
    dir[2usize] = *origin.offset(2isize) - bsptrace.endpos[2usize];
    VectorNormalize(dir.as_mut_ptr());
    kvel[0usize] =
        (dir[0usize] as libc::c_double *
             (1600.0f64 * knockback as libc::c_double /
                  mass as libc::c_double)) as vec_t;
    kvel[1usize] =
        (dir[1usize] as libc::c_double *
             (1600.0f64 * knockback as libc::c_double /
                  mass as libc::c_double)) as vec_t;
    kvel[2usize] =
        (dir[2usize] as libc::c_double *
             (1600.0f64 * knockback as libc::c_double /
                  mass as libc::c_double)) as vec_t;
    return kvel[2usize] + aassettings.phys_jumpvel;
}
//bfg jump Z velocity when bfg-jumping at origin
#[no_mangle]
pub unsafe extern "C" fn AAS_BFGJumpZVelocity(mut origin: *mut vec_t)
 -> libc::c_float {
    return AAS_WeaponJumpZVelocity(origin, 120i32 as libc::c_float);
}
//calculates the horizontal velocity needed for a jump and returns true this velocity could be calculated
#[no_mangle]
pub unsafe extern "C" fn AAS_HorizontalVelocityForJump(mut zvel:
                                                           libc::c_float,
                                                       mut start: *mut vec_t,
                                                       mut end: *mut vec_t,
                                                       mut velocity:
                                                           *mut libc::c_float)
 -> libc::c_int {
    let mut phys_gravity: libc::c_float = 0.;
    let mut phys_maxvelocity: libc::c_float = 0.;
    let mut maxjump: libc::c_float = 0.;
    let mut height2fall: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut top: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    phys_gravity = aassettings.phys_gravity;
    phys_maxvelocity = aassettings.phys_maxvelocity;
    maxjump =
        (0.5f64 * phys_gravity as libc::c_double *
             (zvel / phys_gravity) as libc::c_double *
             (zvel / phys_gravity) as libc::c_double) as libc::c_float;
    top = *start.offset(2isize) + maxjump;
    height2fall = top - *end.offset(2isize);
    if height2fall < 0i32 as libc::c_float {
        *velocity = phys_maxvelocity;
        return 0i32
    }
    t =
        sqrt(height2fall as libc::c_double /
                 (0.5f64 * phys_gravity as libc::c_double)) as libc::c_float;
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    if t + zvel / phys_gravity == 0.0f32 {
        *velocity = phys_maxvelocity;
        return 0i32
    }
    *velocity =
        (sqrt((dir[0usize] * dir[0usize] + dir[1usize] * dir[1usize]) as
                  libc::c_double) /
             (t + zvel / phys_gravity) as libc::c_double) as libc::c_float;
    if *velocity > phys_maxvelocity {
        *velocity = phys_maxvelocity;
        return 0i32
    }
    return 1i32;
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_SetMovedir(mut angles: *mut vec_t,
                                        mut movedir: *mut vec_t) {
    if 0 !=
           VectorCompare(angles as *const vec_t,
                         VEC_UP.as_mut_ptr() as *const vec_t) {
        *movedir.offset(0isize) = MOVEDIR_UP[0usize];
        *movedir.offset(1isize) = MOVEDIR_UP[1usize];
        *movedir.offset(2isize) = MOVEDIR_UP[2usize]
    } else if 0 !=
                  VectorCompare(angles as *const vec_t,
                                VEC_DOWN.as_mut_ptr() as *const vec_t) {
        *movedir.offset(0isize) = MOVEDIR_DOWN[0usize];
        *movedir.offset(1isize) = MOVEDIR_DOWN[1usize];
        *movedir.offset(2isize) = MOVEDIR_DOWN[2usize]
    } else {
        AngleVectors(angles as *const vec_t, movedir, 0 as *mut vec_t,
                     0 as *mut vec_t);
    };
}
static mut MOVEDIR_DOWN: vec3_t =
    [0i32 as vec_t, 0i32 as vec_t, -1i32 as vec_t];
static mut VEC_DOWN: vec3_t = [0i32 as vec_t, -2i32 as vec_t, 0i32 as vec_t];
static mut MOVEDIR_UP: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
//end of the function AAS_Swimming
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
static mut VEC_UP: vec3_t = [0i32 as vec_t, -1i32 as vec_t, 0i32 as vec_t];
//
#[no_mangle]
pub unsafe extern "C" fn AAS_DropToFloor(mut origin: *mut vec_t,
                                         mut mins: *mut vec_t,
                                         mut maxs: *mut vec_t)
 -> libc::c_int {
    let mut end: vec3_t = [0.; 3];
    let mut trace: bsp_trace_t =
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
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    end[2usize] -= 100i32 as libc::c_float;
    trace = AAS_Trace(origin, mins, maxs, end.as_mut_ptr(), 0i32, 1i32);
    if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
    *origin.offset(0isize) = trace.endpos[0usize];
    *origin.offset(1isize) = trace.endpos[1usize];
    *origin.offset(2isize) = trace.endpos[2usize];
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn AAS_InitSettings() {
    aassettings.phys_gravitydirection[0usize] = 0i32 as vec_t;
    aassettings.phys_gravitydirection[1usize] = 0i32 as vec_t;
    aassettings.phys_gravitydirection[2usize] = -1i32 as vec_t;
    aassettings.phys_friction =
        LibVarValue(b"phys_friction\x00" as *const u8 as *const libc::c_char,
                    b"6\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_stopspeed =
        LibVarValue(b"phys_stopspeed\x00" as *const u8 as *const libc::c_char,
                    b"100\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_gravity =
        LibVarValue(b"phys_gravity\x00" as *const u8 as *const libc::c_char,
                    b"800\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_waterfriction =
        LibVarValue(b"phys_waterfriction\x00" as *const u8 as
                        *const libc::c_char,
                    b"1\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_watergravity =
        LibVarValue(b"phys_watergravity\x00" as *const u8 as
                        *const libc::c_char,
                    b"400\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxvelocity =
        LibVarValue(b"phys_maxvelocity\x00" as *const u8 as
                        *const libc::c_char,
                    b"320\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxwalkvelocity =
        LibVarValue(b"phys_maxwalkvelocity\x00" as *const u8 as
                        *const libc::c_char,
                    b"320\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxcrouchvelocity =
        LibVarValue(b"phys_maxcrouchvelocity\x00" as *const u8 as
                        *const libc::c_char,
                    b"100\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxswimvelocity =
        LibVarValue(b"phys_maxswimvelocity\x00" as *const u8 as
                        *const libc::c_char,
                    b"150\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_walkaccelerate =
        LibVarValue(b"phys_walkaccelerate\x00" as *const u8 as
                        *const libc::c_char,
                    b"10\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_airaccelerate =
        LibVarValue(b"phys_airaccelerate\x00" as *const u8 as
                        *const libc::c_char,
                    b"1\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_swimaccelerate =
        LibVarValue(b"phys_swimaccelerate\x00" as *const u8 as
                        *const libc::c_char,
                    b"4\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxstep =
        LibVarValue(b"phys_maxstep\x00" as *const u8 as *const libc::c_char,
                    b"19\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxsteepness =
        LibVarValue(b"phys_maxsteepness\x00" as *const u8 as
                        *const libc::c_char,
                    b"0.7\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxwaterjump =
        LibVarValue(b"phys_maxwaterjump\x00" as *const u8 as
                        *const libc::c_char,
                    b"18\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_maxbarrier =
        LibVarValue(b"phys_maxbarrier\x00" as *const u8 as
                        *const libc::c_char,
                    b"33\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_jumpvel =
        LibVarValue(b"phys_jumpvel\x00" as *const u8 as *const libc::c_char,
                    b"270\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_falldelta5 =
        LibVarValue(b"phys_falldelta5\x00" as *const u8 as
                        *const libc::c_char,
                    b"40\x00" as *const u8 as *const libc::c_char);
    aassettings.phys_falldelta10 =
        LibVarValue(b"phys_falldelta10\x00" as *const u8 as
                        *const libc::c_char,
                    b"60\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_waterjump =
        LibVarValue(b"rs_waterjump\x00" as *const u8 as *const libc::c_char,
                    b"400\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_teleport =
        LibVarValue(b"rs_teleport\x00" as *const u8 as *const libc::c_char,
                    b"50\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_barrierjump =
        LibVarValue(b"rs_barrierjump\x00" as *const u8 as *const libc::c_char,
                    b"100\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_startcrouch =
        LibVarValue(b"rs_startcrouch\x00" as *const u8 as *const libc::c_char,
                    b"300\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_startgrapple =
        LibVarValue(b"rs_startgrapple\x00" as *const u8 as
                        *const libc::c_char,
                    b"500\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_startwalkoffledge =
        LibVarValue(b"rs_startwalkoffledge\x00" as *const u8 as
                        *const libc::c_char,
                    b"70\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_startjump =
        LibVarValue(b"rs_startjump\x00" as *const u8 as *const libc::c_char,
                    b"300\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_rocketjump =
        LibVarValue(b"rs_rocketjump\x00" as *const u8 as *const libc::c_char,
                    b"500\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_bfgjump =
        LibVarValue(b"rs_bfgjump\x00" as *const u8 as *const libc::c_char,
                    b"500\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_jumppad =
        LibVarValue(b"rs_jumppad\x00" as *const u8 as *const libc::c_char,
                    b"250\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_aircontrolledjumppad =
        LibVarValue(b"rs_aircontrolledjumppad\x00" as *const u8 as
                        *const libc::c_char,
                    b"300\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_funcbob =
        LibVarValue(b"rs_funcbob\x00" as *const u8 as *const libc::c_char,
                    b"300\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_startelevator =
        LibVarValue(b"rs_startelevator\x00" as *const u8 as
                        *const libc::c_char,
                    b"50\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_falldamage5 =
        LibVarValue(b"rs_falldamage5\x00" as *const u8 as *const libc::c_char,
                    b"300\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_falldamage10 =
        LibVarValue(b"rs_falldamage10\x00" as *const u8 as
                        *const libc::c_char,
                    b"500\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_maxfallheight =
        LibVarValue(b"rs_maxfallheight\x00" as *const u8 as
                        *const libc::c_char,
                    b"0\x00" as *const u8 as *const libc::c_char);
    aassettings.rs_maxjumpfallheight =
        LibVarValue(b"rs_maxjumpfallheight\x00" as *const u8 as
                        *const libc::c_char,
                    b"450\x00" as *const u8 as *const libc::c_char);
}
//end of the function AAS_ClientMovementHitBBox
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_TestMovementPrediction(mut entnum: libc::c_int,
                                                    mut origin: *mut vec_t,
                                                    mut dir: *mut vec_t) {
    let mut velocity: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
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
    velocity[2usize] = 0i32 as vec_t;
    velocity[1usize] = velocity[2usize];
    velocity[0usize] = velocity[1usize];
    if 0 == AAS_Swimming(origin) { *dir.offset(2isize) = 0i32 as vec_t }
    VectorNormalize(dir);
    cmdmove[0usize] = *dir.offset(0isize) * 400i32 as libc::c_float;
    cmdmove[1usize] = *dir.offset(1isize) * 400i32 as libc::c_float;
    cmdmove[2usize] = *dir.offset(2isize) * 400i32 as libc::c_float;
    cmdmove[2usize] = 224i32 as vec_t;
    AAS_ClearShownDebugLines();
    AAS_PredictClientMovement(&mut move_0, entnum, origin, 2i32,
                              qtrue as libc::c_int, velocity.as_mut_ptr(),
                              cmdmove.as_mut_ptr(), 13i32, 13i32, 0.1f32,
                              1i32, 0i32, qtrue as libc::c_int);
    if 0 != move_0.stopevent & 2i32 {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"leave ground\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
    };
}