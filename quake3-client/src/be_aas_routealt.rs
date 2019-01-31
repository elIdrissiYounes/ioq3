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
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
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
    // alternate route goals
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_altroutegoal_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub starttraveltime: libc::c_ushort,
        pub goaltraveltime: libc::c_ushort,
        pub extratraveltime: libc::c_ushort,
    }
    pub type aas_entityinfo_t = aas_entityinfo_s;
    pub type aas_altroutegoal_t = aas_altroutegoal_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_routealt.c"]
pub mod be_aas_routealt_c {
    pub type midrangearea_t = midrangearea_s;
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
 * name:		be_aas_routealt.c
 *
 * desc:		AAS
 *
 * $Archive: /MissionPack/code/botlib/be_aas_routealt.c $
 *
 *****************************************************************************/
    //#define ALTROUTE_DEBUG
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct midrangearea_s {
        pub valid: libc::c_int,
        pub starttime: libc::c_ushort,
        pub goaltime: libc::c_ushort,
    }
    use super::{libc};
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
    use super::{libc};
    use super::aasfile_h::{aas_bbox_t, aas_vertex_t, aas_plane_t, aas_edge_t,
                           aas_edgeindex_t, aas_face_t, aas_faceindex_t,
                           aas_area_t, aas_areasettings_t, aas_reachability_t,
                           aas_node_t, aas_portal_t, aas_portalindex_t,
                           aas_cluster_t};
    use super::q_shared_h::{byte, vec3_t, qboolean};
    use super::be_aas_h::{aas_entityinfo_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
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
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_reach.h"]
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
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_route.h"]
pub mod be_aas_route_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //returns the travel time from the area to the goal area using the given travel flags
        #[no_mangle]
        pub fn AAS_AreaTravelTimeToGoalArea(areanum: libc::c_int,
                                            origin: *mut vec_t,
                                            goalareanum: libc::c_int,
                                            travelflags: libc::c_int)
         -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_routealt.h"]
pub mod be_aas_routealt_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_h::{aas_altroutegoal_t};
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
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, vec_t, vec3_t};
use self::aasfile_h::{aas_bbox_s, aas_bbox_t, aas_reachability_s,
                      aas_reachability_t, aas_areasettings_s,
                      aas_areasettings_t, aas_portal_s, aas_portal_t,
                      aas_portalindex_t, aas_cluster_s, aas_cluster_t,
                      aas_vertex_t, aas_plane_s, aas_plane_t, aas_edge_s,
                      aas_edge_t, aas_edgeindex_t, aas_face_s, aas_face_t,
                      aas_faceindex_t, aas_area_s, aas_area_t, aas_node_s,
                      aas_node_t};
use self::be_aas_h::{aas_entityinfo_s, aas_altroutegoal_s, aas_entityinfo_t,
                     aas_altroutegoal_t};
use self::be_aas_routealt_c::{midrangearea_t, midrangearea_s};
use self::be_aas_def_h::{aas_t, aas_s, aas_reachabilityareas_t,
                         aas_reachabilityareas_s, aas_routingcache_t,
                         aas_routingcache_s, aas_reversedreachability_t,
                         aas_reversedreachability_s, aas_reversedlink_t,
                         aas_reversedlink_s, aas_routingupdate_t,
                         aas_routingupdate_s, aas_entity_t, aas_entity_s,
                         bsp_link_t, bsp_link_s, aas_link_t, aas_link_s};
use self::mathcalls_h::{sqrt};
use self::string_h::{memset};
use self::stdlib_h::{abs};
use self::l_memory_h::{GetMemory, FreeMemory};
use self::l_log_h::{Log_Write};
use self::be_aas_reach_h::{AAS_AreaReachability};
use self::be_aas_route_h::{AAS_AreaTravelTimeToGoalArea};
use self::be_aas_main_h::{aasworld};
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
//AASINTERN
#[no_mangle]
pub unsafe extern "C" fn AAS_AlternativeRouteGoals(mut start: *mut vec_t,
                                                   mut startareanum:
                                                       libc::c_int,
                                                   mut goal: *mut vec_t,
                                                   mut goalareanum:
                                                       libc::c_int,
                                                   mut travelflags:
                                                       libc::c_int,
                                                   mut altroutegoals:
                                                       *mut aas_altroutegoal_t,
                                                   mut maxaltroutegoals:
                                                       libc::c_int,
                                                   mut type_0: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bestareanum: libc::c_int = 0;
    let mut numaltroutegoals: libc::c_int = 0;
    let mut nummidrangeareas: libc::c_int = 0;
    let mut starttime: libc::c_int = 0;
    let mut goaltime: libc::c_int = 0;
    let mut goaltraveltime: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut mid: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    if 0 == startareanum || 0 == goalareanum { return 0i32 }
    goaltraveltime =
        AAS_AreaTravelTimeToGoalArea(startareanum, start, goalareanum,
                                     travelflags);
    memset(midrangeareas as *mut libc::c_void, 0i32,
           (aasworld.numareas as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<midrangearea_t>()
                                                as libc::c_ulong));
    numaltroutegoals = 0i32;
    nummidrangeareas = 0i32;
    let mut current_block_13: u64;
    i = 1i32;
    while i < aasworld.numareas {
        //
        if 0 == type_0 & 1i32 {
            if !(0 != type_0 & 2i32 &&
                     0 !=
                         (*aasworld.areasettings.offset(i as isize)).contents
                             & 8i32) {
                if !(0 != type_0 & 4i32 &&
                         0 !=
                             (*aasworld.areasettings.offset(i as
                                                                isize)).contents
                                 & 512i32) {
                    current_block_13 = 8515828400728868193;
                } else { current_block_13 = 10048703153582371463; }
            } else { current_block_13 = 10048703153582371463; }
        } else { current_block_13 = 10048703153582371463; }
        match current_block_13 {
            10048703153582371463 => {
                //end if
                //end if
                //end if
                //if the area has no reachabilities
                if !(0 == AAS_AreaReachability(i)) {
                    starttime =
                        AAS_AreaTravelTimeToGoalArea(startareanum, start, i,
                                                     travelflags);
                    if !(0 == starttime) {
                        //if the travel time from the start to the area is greater than the shortest goal travel time
                        if !(starttime as libc::c_float >
                                 1.1f64 as libc::c_float *
                                     goaltraveltime as libc::c_float) {
                            goaltime =
                                AAS_AreaTravelTimeToGoalArea(i,
                                                             0 as *mut vec_t,
                                                             goalareanum,
                                                             travelflags);
                            if !(0 == goaltime) {
                                //if the travel time from the area to the goal is greater than the shortest goal travel time
                                if !(goaltime as libc::c_float >
                                         0.8f64 as libc::c_float *
                                             goaltraveltime as libc::c_float)
                                   {
                                    (*midrangeareas.offset(i as isize)).valid
                                        = qtrue as libc::c_int;
                                    (*midrangeareas.offset(i as
                                                               isize)).starttime
                                        = starttime as libc::c_ushort;
                                    (*midrangeareas.offset(i as
                                                               isize)).goaltime
                                        = goaltime as libc::c_ushort;
                                    Log_Write(b"%d midrange area %d\x00" as
                                                  *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              nummidrangeareas, i);
                                    nummidrangeareas += 1
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        i += 1
    }
    i = 1i32;
    while i < aasworld.numareas {
        if !(0 == (*midrangeareas.offset(i as isize)).valid) {
            numclusterareas = 0i32;
            AAS_AltRoutingFloodCluster_r(i);
            mid[2usize] = 0i32 as vec_t;
            mid[1usize] = mid[2usize];
            mid[0usize] = mid[1usize];
            j = 0i32;
            while j < numclusterareas {
                mid[0usize] =
                    mid[0usize] +
                        (*aasworld.areas.offset(*clusterareas.offset(j as
                                                                         isize)
                                                    as isize)).center[0usize];
                mid[1usize] =
                    mid[1usize] +
                        (*aasworld.areas.offset(*clusterareas.offset(j as
                                                                         isize)
                                                    as isize)).center[1usize];
                mid[2usize] =
                    mid[2usize] +
                        (*aasworld.areas.offset(*clusterareas.offset(j as
                                                                         isize)
                                                    as isize)).center[2usize];
                j += 1
            }
            mid[0usize] =
                (mid[0usize] as libc::c_double *
                     (1.0f64 / numclusterareas as libc::c_double)) as vec_t;
            mid[1usize] =
                (mid[1usize] as libc::c_double *
                     (1.0f64 / numclusterareas as libc::c_double)) as vec_t;
            mid[2usize] =
                (mid[2usize] as libc::c_double *
                     (1.0f64 / numclusterareas as libc::c_double)) as vec_t;
            bestdist = 999999i32 as libc::c_float;
            bestareanum = 0i32;
            j = 0i32;
            while j < numclusterareas {
                dir[0usize] =
                    mid[0usize] -
                        (*aasworld.areas.offset(*clusterareas.offset(j as
                                                                         isize)
                                                    as isize)).center[0usize];
                dir[1usize] =
                    mid[1usize] -
                        (*aasworld.areas.offset(*clusterareas.offset(j as
                                                                         isize)
                                                    as isize)).center[1usize];
                dir[2usize] =
                    mid[2usize] -
                        (*aasworld.areas.offset(*clusterareas.offset(j as
                                                                         isize)
                                                    as isize)).center[2usize];
                dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
                if dist < bestdist {
                    bestdist = dist;
                    bestareanum = *clusterareas.offset(j as isize)
                }
                j += 1
            }
            (*altroutegoals.offset(numaltroutegoals as isize)).origin[0usize]
                =
                (*aasworld.areas.offset(bestareanum as isize)).center[0usize];
            (*altroutegoals.offset(numaltroutegoals as isize)).origin[1usize]
                =
                (*aasworld.areas.offset(bestareanum as isize)).center[1usize];
            (*altroutegoals.offset(numaltroutegoals as isize)).origin[2usize]
                =
                (*aasworld.areas.offset(bestareanum as isize)).center[2usize];
            (*altroutegoals.offset(numaltroutegoals as isize)).areanum =
                bestareanum;
            (*altroutegoals.offset(numaltroutegoals as isize)).starttraveltime
                = (*midrangeareas.offset(bestareanum as isize)).starttime;
            (*altroutegoals.offset(numaltroutegoals as isize)).goaltraveltime
                = (*midrangeareas.offset(bestareanum as isize)).goaltime;
            (*altroutegoals.offset(numaltroutegoals as isize)).extratraveltime
                =
                ((*midrangeareas.offset(bestareanum as isize)).starttime as
                     libc::c_int +
                     (*midrangeareas.offset(bestareanum as isize)).goaltime as
                         libc::c_int - goaltraveltime) as libc::c_ushort;
            numaltroutegoals += 1;
            //
            //don't return more than the maximum alternative route goals
            if numaltroutegoals >= maxaltroutegoals { break ; }
        }
        i += 1
    }
    return numaltroutegoals;
}
#[no_mangle]
pub static mut midrangeareas: *mut midrangearea_t =
    0 as *const midrangearea_t as *mut midrangearea_t;
#[no_mangle]
pub static mut clusterareas: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut numclusterareas: libc::c_int = 0;
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_AltRoutingFloodCluster_r(mut areanum:
                                                          libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut otherareanum: libc::c_int = 0;
    let mut area: *mut aas_area_t = 0 as *mut aas_area_t;
    let mut face: *mut aas_face_t = 0 as *mut aas_face_t;
    *clusterareas.offset(numclusterareas as isize) = areanum;
    numclusterareas += 1;
    (*midrangeareas.offset(areanum as isize)).valid = qfalse as libc::c_int;
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
        if (*face).frontarea == areanum {
            otherareanum = (*face).backarea
        } else { otherareanum = (*face).frontarea }
        //if there is an area at the other side of this face
        if !(0 == otherareanum) {
            //if the other area is not a midrange area
            if !(0 == (*midrangeareas.offset(otherareanum as isize)).valid) {
                AAS_AltRoutingFloodCluster_r(otherareanum);
            }
        }
        i += 1
    };
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
 * name:		be_aas_routealt.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_routealt.h $
 *
 *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn AAS_InitAlternativeRouting() {
    if !midrangeareas.is_null() {
        FreeMemory(midrangeareas as *mut libc::c_void);
    }
    midrangeareas =
        GetMemory((aasworld.numareas as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<midrangearea_t>()
                                                       as libc::c_ulong)) as
            *mut midrangearea_t;
    if !clusterareas.is_null() {
        FreeMemory(clusterareas as *mut libc::c_void);
    }
    clusterareas =
        GetMemory((aasworld.numareas as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                       as libc::c_ulong)) as
            *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AAS_ShutdownAlternativeRouting() {
    if !midrangeareas.is_null() {
        FreeMemory(midrangeareas as *mut libc::c_void);
    }
    midrangeareas = 0 as *mut midrangearea_t;
    if !clusterareas.is_null() {
        FreeMemory(clusterareas as *mut libc::c_void);
    }
    clusterareas = 0 as *mut libc::c_int;
    numclusterareas = 0i32;
}