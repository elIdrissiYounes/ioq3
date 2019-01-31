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
        pub fn VectorNormalize2(v: *const vec_t, out: *mut vec_t) -> vec_t;
        #[no_mangle]
        pub fn vectoangles(value1: *const vec_t, angles: *mut vec_t);
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
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
        //creates the library variable if not existing already and returns it
        #[no_mangle]
        pub fn LibVar(var_name: *const libc::c_char,
                      value: *const libc::c_char) -> *mut libvar_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/aasfile.h"]
pub mod aasfile_h {
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_goal.h"]
pub mod be_ai_goal_h {
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
/*****************************************************************************
 * name:		be_ai_goal.h
 *
 * desc:		goal AI
 *
 * $Archive: /source/code/botlib/be_ai_goal.h $
 *
 *****************************************************************************/
    //a bot goal
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_goal_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub entitynum: libc::c_int,
        pub number: libc::c_int,
        pub flags: libc::c_int,
        pub iteminfo: libc::c_int,
    }
    pub type bot_goal_t = bot_goal_s;
    use super::q_shared_h::{vec3_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_move.h"]
pub mod be_ai_move_h {
    //NOTE: the ideal_viewangles are only valid if MFL_MOVEMENTVIEW is set
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_moveresult_s {
        pub failure: libc::c_int,
        pub type_0: libc::c_int,
        pub blocked: libc::c_int,
        pub blockentity: libc::c_int,
        pub traveltype: libc::c_int,
        pub flags: libc::c_int,
        pub weapon: libc::c_int,
        pub movedir: vec3_t,
        pub ideal_viewangles: vec3_t,
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
 * name:		be_ai_move.h
 *
 * desc:		movement AI
 *
 * $Archive: /source/code/botlib/be_ai_move.h $
 *
 *****************************************************************************/
    //movement types
    //move flags
    //bot is performing a barrier jump
    //bot is in the ground
    //bot is swimming
    //bot is against a ladder
    //bot is waterjumping
    //bot is being teleported
    //bot is being pulled by the grapple
    //bot is using the grapple hook
    //bot has reset the grapple
    //bot should walk slowly
    // move result flags
    //bot uses view for movement
    //bot uses view for swimming
    //bot is waiting for something
    //bot has set the view in movement code
    //bot uses weapon for movement
    //bot is ontop of obstacle
    //bot is ontop of a func_bobbing
    //bot is ontop of an elevator (func_plat)
    //bot is blocked by an avoid spot
    //
    // avoid spot types
    //clear all avoid spots
    //avoid always
    //never totally block
    // restult types
    //elevator is up
    //waiting for func bobbing to arrive
    //grapple path is obstructed
    //stuck in solid area, this is bad
    //structure used to initialize the movement state
//the or_moveflags MFL_ONGROUND, MFL_TELEPORTED and MFL_WATERJUMP come from the playerstate
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_initmove_s {
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub viewoffset: vec3_t,
        pub entitynum: libc::c_int,
        pub client: libc::c_int,
        pub thinktime: libc::c_float,
        pub presencetype: libc::c_int,
        pub viewangles: vec3_t,
        pub or_moveflags: libc::c_int,
    }
    pub type bot_initmove_t = bot_initmove_s;
    pub type bot_moveresult_t = bot_moveresult_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_avoidspot_s {
        pub origin: vec3_t,
        pub radius: libc::c_float,
        pub type_0: libc::c_int,
    }
    pub type bot_avoidspot_t = bot_avoidspot_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t, vec_t};
    use super::be_ai_goal_h::{bot_goal_t};
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_move.c"]
pub mod be_ai_move_c {
    pub type bot_movestate_t = bot_movestate_s;
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
 * name:		be_ai_move.c
 *
 * desc:		bot movement AI
 *
 * $Archive: /MissionPack/code/botlib/be_ai_move.c $
 *
 *****************************************************************************/
    //#define DEBUG_AI_MOVE
//#define DEBUG_ELEVATOR
//#define DEBUG_GRAPPLE
    //movement state
//NOTE: the moveflags MFL_ONGROUND, MFL_TELEPORTED, MFL_WATERJUMP and
//		MFL_GRAPPLEPULL must be set outside the movement code
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_movestate_s {
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub viewoffset: vec3_t,
        pub entitynum: libc::c_int,
        pub client: libc::c_int,
        pub thinktime: libc::c_float,
        pub presencetype: libc::c_int,
        pub viewangles: vec3_t,
        pub areanum: libc::c_int,
        pub lastareanum: libc::c_int,
        pub lastgoalareanum: libc::c_int,
        pub lastreachnum: libc::c_int,
        pub lastorigin: vec3_t,
        pub reachareanum: libc::c_int,
        pub moveflags: libc::c_int,
        pub jumpreach: libc::c_int,
        pub grapplevisible_time: libc::c_float,
        pub lastgrappledist: libc::c_float,
        pub reachability_time: libc::c_float,
        pub avoidreach: [libc::c_int; 1],
        pub avoidreachtimes: [libc::c_float; 1],
        pub avoidreachtries: [libc::c_int; 1],
        pub avoidspots: [bot_avoidspot_t; 32],
        pub numavoidspots: libc::c_int,
    }
    use super::q_shared_h::{vec3_t, vec_t};
    use super::{libc};
    use super::be_ai_move_h::{bot_avoidspot_t, bot_moveresult_t,
                              bot_avoidspot_s};
    use super::aasfile_h::{aas_reachability_t};
    use super::l_libvar_h::{libvar_t};
    use super::be_ai_goal_h::{bot_goal_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
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
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn rand() -> libc::c_int;
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
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
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_entity.h"]
pub mod be_aas_entity_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_h::{aas_entityinfo_t};
    extern "C" {
        //returns the BSP model number of the entity
        #[no_mangle]
        pub fn AAS_EntityModelNum(entnum: libc::c_int) -> libc::c_int;
        //returns the origin of an entity with the given model number
        #[no_mangle]
        pub fn AAS_OriginOfMoverWithModelNum(modelnum: libc::c_int,
                                             origin: *mut vec_t)
         -> libc::c_int;
        //returns the info of the given entity
        #[no_mangle]
        pub fn AAS_EntityInfo(entnum: libc::c_int,
                              info: *mut aas_entityinfo_t);
        //returns the next entity
        #[no_mangle]
        pub fn AAS_NextEntity(entnum: libc::c_int) -> libc::c_int;
        //returns the entity type
        #[no_mangle]
        pub fn AAS_EntityType(entnum: libc::c_int) -> libc::c_int;
        //returns the model index of the entity
        #[no_mangle]
        pub fn AAS_EntityModelindex(entnum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::{libc};
    use super::q_shared_h::{vec_t, vec3_t};
    use super::be_aas_h::{aas_trace_t};
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
 * name:		be_aas_sample.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_sample.h $
 *
 *****************************************************************************/
        //AASINTERN
        //returns the mins and maxs of the bounding box for the given presence type
        #[no_mangle]
        pub fn AAS_PresenceTypeBoundingBox(presencetype: libc::c_int,
                                           mins: *mut vec_t,
                                           maxs: *mut vec_t);
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
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_reach.h"]
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
        //AASINTERN
        //returns true if the are has reachabilities to other areas
        #[no_mangle]
        pub fn AAS_AreaReachability(areanum: libc::c_int) -> libc::c_int;
        //returns the next reachability using the given model
        #[no_mangle]
        pub fn AAS_NextModelReachability(num: libc::c_int,
                                         modelnum: libc::c_int)
         -> libc::c_int;
        //returns true if the area is a jump pad
        #[no_mangle]
        pub fn AAS_AreaJumpPad(areanum: libc::c_int) -> libc::c_int;
        //returns true if the area is donotenter
        #[no_mangle]
        pub fn AAS_AreaDoNotEnter(areanum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_route.h"]
pub mod be_aas_route_h {
    use super::{libc};
    use super::aasfile_h::{aas_reachability_s};
    use super::q_shared_h::{vec_t};
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
 * name:		be_aas_route.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_route.h $
 *
 *****************************************************************************/
        //AASINTERN
        //returns the travel flag for the given travel type
        #[no_mangle]
        pub fn AAS_TravelFlagForType(traveltype: libc::c_int) -> libc::c_int;
        //return the travel flag(s) for traveling through this area
        #[no_mangle]
        pub fn AAS_AreaContentsTravelFlags(areanum: libc::c_int)
         -> libc::c_int;
        //returns the index of the next reachability for the given area
        #[no_mangle]
        pub fn AAS_NextAreaReachability(areanum: libc::c_int,
                                        reachnum: libc::c_int) -> libc::c_int;
        //returns the reachability with the given index
        #[no_mangle]
        pub fn AAS_ReachabilityFromNum(num: libc::c_int,
                                       reach: *mut aas_reachability_s);
        //returns the travel time from the area to the goal area using the given travel flags
        #[no_mangle]
        pub fn AAS_AreaTravelTimeToGoalArea(areanum: libc::c_int,
                                            origin: *mut vec_t,
                                            goalareanum: libc::c_int,
                                            travelflags: libc::c_int)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::botlib_h::{bsp_trace_t};
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
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
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
        //gets the mins, maxs and origin of a BSP model
        #[no_mangle]
        pub fn AAS_BSPModelMinsMaxsOrigin(modelnum: libc::c_int,
                                          angles: *mut vec_t,
                                          mins: *mut vec_t, maxs: *mut vec_t,
                                          origin: *mut vec_t);
        //handle to the next bsp entity
        #[no_mangle]
        pub fn AAS_NextBSPEntity(ent: libc::c_int) -> libc::c_int;
        //return the value of the BSP epair key
        #[no_mangle]
        pub fn AAS_ValueForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_char,
                                       size: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    use super::{libc};
    use super::be_aas_h::{aas_clientmove_s};
    use super::q_shared_h::{vec_t};
    use super::aasfile_h::{aas_reachability_s};
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
 * name:		be_aas_move.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_move.h $
 *
 *****************************************************************************/
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
        //returns true if on the ground at the given origin
        #[no_mangle]
        pub fn AAS_OnGround(origin: *mut vec_t, presencetype: libc::c_int,
                            passent: libc::c_int) -> libc::c_int;
        //returns true if swimming at the given origin
        #[no_mangle]
        pub fn AAS_Swimming(origin: *mut vec_t) -> libc::c_int;
        //returns the jump reachability run start point
        #[no_mangle]
        pub fn AAS_JumpReachRunStart(reach: *mut aas_reachability_s,
                                     runstart: *mut vec_t);
        //returns true if against a ladder at the given origin
        #[no_mangle]
        pub fn AAS_AgainstLadder(origin: *mut vec_t) -> libc::c_int;
        //calculates the horizontal velocity needed for a jump and returns true this velocity could be calculated
        #[no_mangle]
        pub fn AAS_HorizontalVelocityForJump(zvel: libc::c_float,
                                             start: *mut vec_t,
                                             end: *mut vec_t,
                                             velocity: *mut libc::c_float)
         -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ea.h"]
pub mod be_ea_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        #[no_mangle]
        pub fn EA_Command(client: libc::c_int, command: *mut libc::c_char);
        #[no_mangle]
        pub fn EA_Crouch(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Walk(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveUp(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveForward(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Attack(client: libc::c_int);
        //regular elementary actions
        #[no_mangle]
        pub fn EA_SelectWeapon(client: libc::c_int, weapon: libc::c_int);
        #[no_mangle]
        pub fn EA_Jump(client: libc::c_int);
        #[no_mangle]
        pub fn EA_DelayedJump(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Move(client: libc::c_int, dir: *mut vec_t,
                       speed: libc::c_float);
        #[no_mangle]
        pub fn EA_View(client: libc::c_int, viewangles: *mut vec_t);
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, VectorNormalize,
                       VectorNormalize2, vectoangles, Q_stricmp};
use self::l_libvar_h::{libvar_s, libvar_t, LibVar};
use self::aasfile_h::{aas_reachability_s, aas_reachability_t};
use self::be_aas_h::{aas_clientmove_s, aas_trace_t, aas_trace_s,
                     aas_entityinfo_s, aas_entityinfo_t, aas_clientmove_t};
use self::be_ai_goal_h::{bot_goal_s, bot_goal_t};
use self::be_ai_move_h::{bot_moveresult_s, bot_initmove_s, bot_initmove_t,
                         bot_moveresult_t, bot_avoidspot_s, bot_avoidspot_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_ai_move_c::{bot_movestate_t, bot_movestate_s};
use self::mathcalls_h::{sqrt, fabs, fabsf};
use self::string_h::{memset};
use self::stdlib_h::{atoi, rand};
use self::l_memory_h::{GetClearedMemory, FreeMemory};
use self::be_aas_main_h::{AAS_Time, AAS_ProjectPointOntoVector};
use self::be_aas_entity_h::{AAS_EntityModelNum, AAS_OriginOfMoverWithModelNum,
                            AAS_EntityInfo, AAS_NextEntity, AAS_EntityType,
                            AAS_EntityModelindex};
use self::be_aas_sample_h::{AAS_PresenceTypeBoundingBox, AAS_AreaPresenceType,
                            AAS_TraceClientBBox, AAS_TraceAreas,
                            AAS_PointAreaNum};
use self::be_aas_reach_h::{AAS_AreaReachability, AAS_NextModelReachability,
                           AAS_AreaJumpPad, AAS_AreaDoNotEnter};
use self::be_aas_route_h::{AAS_TravelFlagForType, AAS_AreaContentsTravelFlags,
                           AAS_NextAreaReachability, AAS_ReachabilityFromNum,
                           AAS_AreaTravelTimeToGoalArea};
use self::be_aas_bsp_h::{AAS_Trace, AAS_PointContents,
                         AAS_BSPModelMinsMaxsOrigin, AAS_NextBSPEntity,
                         AAS_ValueForBSPEpairKey};
use self::be_aas_move_h::{AAS_PredictClientMovement, AAS_OnGround,
                          AAS_Swimming, AAS_JumpReachRunStart,
                          AAS_AgainstLadder, AAS_HorizontalVelocityForJump};
use self::be_interface_h::{botimport, botDeveloper};
use self::be_ea_h::{EA_Command, EA_Crouch, EA_Walk, EA_MoveUp, EA_MoveForward,
                    EA_Attack, EA_SelectWeapon, EA_Jump, EA_DelayedJump,
                    EA_Move, EA_View};
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
unsafe extern "C" fn VectorLengthSquared(mut v: *const vec_t) -> vec_t {
    return *v.offset(0isize) * *v.offset(0isize) +
               *v.offset(1isize) * *v.offset(1isize) +
               *v.offset(2isize) * *v.offset(2isize);
}
//resets the whole move state
#[no_mangle]
pub unsafe extern "C" fn BotResetMoveState(mut movestate: libc::c_int) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return }
    memset(ms as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bot_movestate_t>() as libc::c_ulong);
}
//end of the function BotFreeMoveState
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotMoveStateFromHandle(mut handle: libc::c_int)
 -> *mut bot_movestate_t {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"move state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_movestate_t
    }
    if botmovestates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid move state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_movestate_t
    }
    return botmovestates[handle as usize];
}
#[no_mangle]
pub static mut botmovestates: [*mut bot_movestate_t; 65] =
    [0 as *const bot_movestate_t as *mut bot_movestate_t; 65];
//moves the bot to the given goal
#[no_mangle]
pub unsafe extern "C" fn BotMoveToGoal(mut result: *mut bot_moveresult_t,
                                       mut movestate: libc::c_int,
                                       mut goal: *mut bot_goal_t,
                                       mut travelflags: libc::c_int) {
    let mut reachnum: libc::c_int = 0;
    let mut lastreachnum: libc::c_int = 0;
    let mut foundjumppad: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut resultflags: libc::c_int = 0;
    let mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    let mut lastreach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    (*result).failure = qfalse as libc::c_int;
    (*result).type_0 = 0i32;
    (*result).blocked = qfalse as libc::c_int;
    (*result).blockentity = 0i32;
    (*result).traveltype = 0i32;
    (*result).flags = 0i32;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return }
    BotResetGrapple(ms);
    if goal.is_null() { (*result).failure = qtrue as libc::c_int; return }
    (*ms).moveflags &= !(4i32 | 8i32);
    if 0 !=
           AAS_OnGround((*ms).origin.as_mut_ptr(), (*ms).presencetype,
                        (*ms).entitynum) {
        (*ms).moveflags |= 2i32
    }
    if 0 != (*ms).moveflags & 2i32 {
        let mut modeltype: libc::c_int = 0;
        let mut modelnum: libc::c_int = 0;
        ent = BotOnTopOfEntity(ms);
        if ent != -1i32 {
            modelnum = AAS_EntityModelindex(ent);
            if modelnum >= 0i32 && modelnum < 256i32 {
                modeltype = modeltypes[modelnum as usize];
                if modeltype == 1i32 {
                    AAS_ReachabilityFromNum((*ms).lastreachnum, &mut reach);
                    if reach.traveltype & 0xffffffi32 != 11i32 ||
                           reach.facenum & 0xffffi32 != modelnum {
                        reachnum = AAS_NextModelReachability(0i32, modelnum);
                        if 0 != reachnum {
                            AAS_ReachabilityFromNum(reachnum, &mut reach);
                            (*ms).lastreachnum = reachnum;
                            (*ms).reachability_time =
                                AAS_Time() +
                                    BotReachabilityTime(&mut reach) as
                                        libc::c_float
                        } else {
                            if 0 != botDeveloper {
                                botimport.Print.expect("non-null function pointer")(1i32,
                                                                                    b"client %d: on func_plat without reachability\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char
                                                                                        as
                                                                                        *mut libc::c_char,
                                                                                    (*ms).client);
                            }
                            (*result).blocked = qtrue as libc::c_int;
                            (*result).blockentity = ent;
                            (*result).flags |= 32i32;
                            return
                        }
                    }
                    (*result).flags |= 128i32
                } else if modeltype == 2i32 {
                    AAS_ReachabilityFromNum((*ms).lastreachnum, &mut reach);
                    if reach.traveltype & 0xffffffi32 != 19i32 ||
                           reach.facenum & 0xffffi32 != modelnum {
                        reachnum = AAS_NextModelReachability(0i32, modelnum);
                        if 0 != reachnum {
                            AAS_ReachabilityFromNum(reachnum, &mut reach);
                            (*ms).lastreachnum = reachnum;
                            (*ms).reachability_time =
                                AAS_Time() +
                                    BotReachabilityTime(&mut reach) as
                                        libc::c_float
                        } else {
                            if 0 != botDeveloper {
                                botimport.Print.expect("non-null function pointer")(1i32,
                                                                                    b"client %d: on func_bobbing without reachability\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char
                                                                                        as
                                                                                        *mut libc::c_char,
                                                                                    (*ms).client);
                            }
                            (*result).blocked = qtrue as libc::c_int;
                            (*result).blockentity = ent;
                            (*result).flags |= 32i32;
                            return
                        }
                    }
                    (*result).flags |= 64i32
                } else if modeltype == 4i32 || modeltype == 3i32 {
                    (*ms).areanum =
                        BotFuzzyPointReachabilityArea((*ms).origin.as_mut_ptr());
                    if 0 == AAS_AreaReachability((*ms).areanum) {
                        (*result).blocked = qtrue as libc::c_int;
                        (*result).blockentity = ent;
                        (*result).flags |= 32i32;
                        return
                    }
                } else {
                    (*result).blocked = qtrue as libc::c_int;
                    (*result).blockentity = ent;
                    (*result).flags |= 32i32;
                    return
                }
            }
        }
    }
    if 0 != AAS_Swimming((*ms).origin.as_mut_ptr()) {
        (*ms).moveflags |= 4i32
    }
    if 0 != AAS_AgainstLadder((*ms).origin.as_mut_ptr()) {
        (*ms).moveflags |= 8i32
    }
    if 0 != (*ms).moveflags & (2i32 | 4i32 | 8i32) {
        AAS_ReachabilityFromNum((*ms).lastreachnum, &mut lastreach);
        (*ms).areanum =
            BotFuzzyPointReachabilityArea((*ms).origin.as_mut_ptr());
        if 0 == (*ms).areanum {
            (*result).failure = qtrue as libc::c_int;
            (*result).blocked = qtrue as libc::c_int;
            (*result).blockentity = 0i32;
            (*result).type_0 = 8i32;
            return
        }
        if (*ms).areanum == (*goal).areanum {
            *result = BotMoveInGoalArea(ms, goal);
            return
        }
        reachnum = (*ms).lastreachnum;
        if 0 != reachnum {
            AAS_ReachabilityFromNum(reachnum, &mut reach);
            if 0 == AAS_TravelFlagForType(reach.traveltype) & travelflags {
                reachnum = 0i32
            } else if reach.traveltype & 0xffffffi32 == 14i32 {
                if (*ms).reachability_time < AAS_Time() ||
                       0 != (*ms).moveflags & 256i32 {
                    reachnum = 0i32
                }
            } else if reach.traveltype & 0xffffffi32 == 11i32 ||
                          reach.traveltype & 0xffffffi32 == 19i32 {
                if 0 != (*result).flags & 128i32 ||
                       0 != (*result).flags & 64i32 {
                    (*ms).reachability_time =
                        AAS_Time() + 5i32 as libc::c_float
                }
                if (*ms).areanum == reach.areanum ||
                       (*ms).reachability_time < AAS_Time() {
                    reachnum = 0i32
                }
            } else if (*ms).lastgoalareanum != (*goal).areanum ||
                          (*ms).reachability_time < AAS_Time() ||
                          (*ms).lastareanum != (*ms).areanum {
                reachnum = 0i32
            }
        }
        resultflags = 0i32;
        if 0 == reachnum {
            0 == AAS_AreaReachability((*ms).areanum);
            reachnum =
                BotGetReachabilityToGoal((*ms).origin.as_mut_ptr(),
                                         (*ms).areanum, (*ms).lastgoalareanum,
                                         (*ms).lastareanum,
                                         (*ms).avoidreach.as_mut_ptr(),
                                         (*ms).avoidreachtimes.as_mut_ptr(),
                                         (*ms).avoidreachtries.as_mut_ptr(),
                                         goal, travelflags,
                                         (*ms).avoidspots.as_mut_ptr(),
                                         (*ms).numavoidspots,
                                         &mut resultflags);
            (*ms).reachareanum = (*ms).areanum;
            (*ms).jumpreach = 0i32;
            (*ms).moveflags &= !256i32;
            if 0 != reachnum {
                AAS_ReachabilityFromNum(reachnum, &mut reach);
                (*ms).reachability_time =
                    AAS_Time() +
                        BotReachabilityTime(&mut reach) as libc::c_float;
                BotAddToAvoidReach(ms, reachnum, 6i32 as libc::c_float);
            }
        }
        (*ms).lastreachnum = reachnum;
        (*ms).lastgoalareanum = (*goal).areanum;
        (*ms).lastareanum = (*ms).areanum;
        if 0 != reachnum {
            AAS_ReachabilityFromNum(reachnum, &mut reach);
            (*result).traveltype = reach.traveltype;
            match reach.traveltype & 0xffffffi32 {
                2 => { *result = BotTravel_Walk(ms, &mut reach) }
                3 => { *result = BotTravel_Crouch(ms, &mut reach) }
                4 => { *result = BotTravel_BarrierJump(ms, &mut reach) }
                6 => { *result = BotTravel_Ladder(ms, &mut reach) }
                7 => { *result = BotTravel_WalkOffLedge(ms, &mut reach) }
                5 => { *result = BotTravel_Jump(ms, &mut reach) }
                8 => { *result = BotTravel_Swim(ms, &mut reach) }
                9 => { *result = BotTravel_WaterJump(ms, &mut reach) }
                10 => { *result = BotTravel_Teleport(ms, &mut reach) }
                11 => { *result = BotTravel_Elevator(ms, &mut reach) }
                14 => { *result = BotTravel_Grapple(ms, &mut reach) }
                12 => { *result = BotTravel_RocketJump(ms, &mut reach) }
                13 => { *result = BotTravel_BFGJump(ms, &mut reach) }
                18 => { *result = BotTravel_JumpPad(ms, &mut reach) }
                19 => { *result = BotTravel_FuncBobbing(ms, &mut reach) }
                _ => {
                    botimport.Print.expect("non-null function pointer")(4i32,
                                                                        b"travel type %d not implemented yet\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        reach.traveltype
                                                                            &
                                                                            0xffffffi32);
                }
            }
            (*result).traveltype = reach.traveltype;
            (*result).flags |= resultflags
        } else {
            (*result).failure = qtrue as libc::c_int;
            (*result).flags |= resultflags;
            memset(&mut reach as *mut aas_reachability_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<aas_reachability_t>() as
                       libc::c_ulong);
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut numareas: libc::c_int = 0;
        let mut areas: [libc::c_int; 16] = [0; 16];
        let mut end: vec3_t = [0.; 3];
        foundjumppad = qfalse as libc::c_int;
        end[0usize] =
            (*ms).origin[0usize] +
                (*ms).velocity[0usize] *
                    (-2i32 as libc::c_float * (*ms).thinktime);
        end[1usize] =
            (*ms).origin[1usize] +
                (*ms).velocity[1usize] *
                    (-2i32 as libc::c_float * (*ms).thinktime);
        end[2usize] =
            (*ms).origin[2usize] +
                (*ms).velocity[2usize] *
                    (-2i32 as libc::c_float * (*ms).thinktime);
        numareas =
            AAS_TraceAreas((*ms).origin.as_mut_ptr(), end.as_mut_ptr(),
                           areas.as_mut_ptr(), 0 as *mut vec3_t, 16i32);
        i = numareas - 1i32;
        while i >= 0i32 {
            if 0 != AAS_AreaJumpPad(areas[i as usize]) {
                foundjumppad = qtrue as libc::c_int;
                lastreachnum =
                    BotGetReachabilityToGoal(end.as_mut_ptr(),
                                             areas[i as usize],
                                             (*ms).lastgoalareanum,
                                             (*ms).lastareanum,
                                             (*ms).avoidreach.as_mut_ptr(),
                                             (*ms).avoidreachtimes.as_mut_ptr(),
                                             (*ms).avoidreachtries.as_mut_ptr(),
                                             goal, 0x40000i32,
                                             (*ms).avoidspots.as_mut_ptr(),
                                             (*ms).numavoidspots,
                                             0 as *mut libc::c_int);
                if 0 != lastreachnum {
                    (*ms).lastreachnum = lastreachnum;
                    (*ms).lastareanum = areas[i as usize];
                    //botimport.Print(PRT_MESSAGE, "found jumppad reachability\n");
                    break ;
                } else {
                    //end if
                    lastreachnum =
                        AAS_NextAreaReachability(areas[i as usize], 0i32);
                    while 0 != lastreachnum {
                        AAS_ReachabilityFromNum(lastreachnum, &mut reach);
                        if reach.traveltype & 0xffffffi32 == 18i32 {
                            (*ms).lastreachnum = lastreachnum;
                            (*ms).lastareanum = areas[i as usize]
                        }
                        lastreachnum =
                            AAS_NextAreaReachability(areas[i as usize],
                                                     lastreachnum)
                    }
                    //botimport.Print(PRT_MESSAGE, "found jumppad reachability hard!!\n");
                    //end if
                    //end for
                    if 0 != lastreachnum { break ; }
                }
            }
            i -= 1
        }
        if 0 != botDeveloper {
            if 0 != foundjumppad && 0 == (*ms).lastreachnum {
                botimport.Print.expect("non-null function pointer")(1i32,
                                                                    b"client %d didn\'t find jumppad reachability\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    (*ms).client);
            }
        }
        if 0 != (*ms).lastreachnum {
            AAS_ReachabilityFromNum((*ms).lastreachnum, &mut reach);
            (*result).traveltype = reach.traveltype;
            match reach.traveltype & 0xffffffi32 {
                2 => { *result = BotTravel_Walk(ms, &mut reach) }
                3 => { }
                4 => {
                    //BotFinishTravel_Walk(ms, &reach); break;
                    /*do nothing*/
                    *result = BotFinishTravel_BarrierJump(ms, &mut reach)
                }
                6 => { *result = BotTravel_Ladder(ms, &mut reach) }
                7 => {
                    *result = BotFinishTravel_WalkOffLedge(ms, &mut reach)
                }
                5 => { *result = BotFinishTravel_Jump(ms, &mut reach) }
                8 => { *result = BotTravel_Swim(ms, &mut reach) }
                9 => { *result = BotFinishTravel_WaterJump(ms, &mut reach) }
                10 => { }
                11 => {
                    /*do nothing*/
                    *result = BotFinishTravel_Elevator(ms, &mut reach)
                }
                14 => { *result = BotTravel_Grapple(ms, &mut reach) }
                12 | 13 => {
                    *result = BotFinishTravel_WeaponJump(ms, &mut reach)
                }
                18 => { *result = BotFinishTravel_JumpPad(ms, &mut reach) }
                19 => {
                    *result = BotFinishTravel_FuncBobbing(ms, &mut reach)
                }
                _ => {
                    botimport.Print.expect("non-null function pointer")(4i32,
                                                                        b"(last) travel type %d not implemented yet\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        reach.traveltype
                                                                            &
                                                                            0xffffffi32);
                }
            }
            (*result).traveltype = reach.traveltype
        }
    }
    if 0 != (*result).blocked {
        (*ms).reachability_time -= 10i32 as libc::c_float * (*ms).thinktime
    }
    (*ms).lastorigin[0usize] = (*ms).origin[0usize];
    (*ms).lastorigin[1usize] = (*ms).origin[1usize];
    (*ms).lastorigin[2usize] = (*ms).origin[2usize];
}
//end of the function BotTravel_FuncBobbing
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_FuncBobbing(mut ms:
                                                         *mut bot_movestate_t,
                                                     mut reach:
                                                         *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut bob_origin: vec3_t = [0.; 3];
    let mut bob_start: vec3_t = [0.; 3];
    let mut bob_end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut bottomcenter: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    BotFuncBobStartEnd(reach, bob_start.as_mut_ptr(), bob_end.as_mut_ptr(),
                       bob_origin.as_mut_ptr());
    dir[0usize] = bob_origin[0usize] - bob_end[0usize];
    dir[1usize] = bob_origin[1usize] - bob_end[1usize];
    dir[2usize] = bob_origin[2usize] - bob_end[2usize];
    dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
    if dist < 16i32 as libc::c_float {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { hordir[2usize] = 0i32 as vec_t }
        dist = VectorNormalize(hordir.as_mut_ptr());
        if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
        speed =
            360i32 as libc::c_float -
                (360i32 as libc::c_float - 6i32 as libc::c_float * dist);
        if speed > 5i32 as libc::c_float {
            EA_Move((*ms).client, dir.as_mut_ptr(), speed);
        }
        result.movedir[0usize] = dir[0usize];
        result.movedir[1usize] = dir[1usize];
        result.movedir[2usize] = dir[2usize];
        if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
    } else {
        MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
        hordir[0usize] = bottomcenter[0usize] - (*ms).origin[0usize];
        hordir[1usize] = bottomcenter[1usize] - (*ms).origin[1usize];
        hordir[2usize] = bottomcenter[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { hordir[2usize] = 0i32 as vec_t }
        dist = VectorNormalize(hordir.as_mut_ptr());
        if dist > 5i32 as libc::c_float {
            if dist > 100i32 as libc::c_float {
                dist = 100i32 as libc::c_float
            }
            speed =
                400i32 as libc::c_float -
                    (400i32 as libc::c_float - 4i32 as libc::c_float * dist);
            EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
            result.movedir[0usize] = hordir[0usize];
            result.movedir[1usize] = hordir[1usize];
            result.movedir[2usize] = hordir[2usize]
        }
    }
    return result;
}
//end of the function BotPredictVisiblePosition
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn MoverBottomCenter(mut reach: *mut aas_reachability_t,
                                           mut bottomcenter: *mut vec_t) {
    let mut modelnum: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut mids: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    modelnum = (*reach).facenum & 0xffffi32;
    AAS_BSPModelMinsMaxsOrigin(modelnum, angles.as_mut_ptr(),
                               mins.as_mut_ptr(), maxs.as_mut_ptr(),
                               origin.as_mut_ptr());
    if 0 == AAS_OriginOfMoverWithModelNum(modelnum, origin.as_mut_ptr()) {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"no entity with model %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            modelnum);
    }
    mids[0usize] = mins[0usize] + maxs[0usize];
    mids[1usize] = mins[1usize] + maxs[1usize];
    mids[2usize] = mins[2usize] + maxs[2usize];
    *bottomcenter.offset(0isize) =
        (origin[0usize] as libc::c_double +
             mids[0usize] as libc::c_double * 0.5f64) as vec_t;
    *bottomcenter.offset(1isize) =
        (origin[1usize] as libc::c_double +
             mids[1usize] as libc::c_double * 0.5f64) as vec_t;
    *bottomcenter.offset(2isize) =
        (origin[2usize] as libc::c_double +
             mids[2usize] as libc::c_double * 0.5f64) as vec_t;
    *bottomcenter.offset(2isize) = (*reach).start[2usize];
}
//end of the function BotFinishTravel_Elevator
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFuncBobStartEnd(mut reach:
                                                *mut aas_reachability_t,
                                            mut start: *mut vec_t,
                                            mut end: *mut vec_t,
                                            mut origin: *mut vec_t) {
    let mut spawnflags: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut mid: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut num0: libc::c_int = 0;
    let mut num1: libc::c_int = 0;
    modelnum = (*reach).facenum & 0xffffi32;
    if 0 == AAS_OriginOfMoverWithModelNum(modelnum, origin) {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"BotFuncBobStartEnd: no entity with model %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            modelnum);
        *start.offset(0isize) = 0i32 as vec_t;
        *start.offset(1isize) = 0i32 as vec_t;
        *start.offset(2isize) = 0i32 as vec_t;
        *end.offset(0isize) = 0i32 as vec_t;
        *end.offset(1isize) = 0i32 as vec_t;
        *end.offset(2isize) = 0i32 as vec_t;
        return
    }
    AAS_BSPModelMinsMaxsOrigin(modelnum, angles.as_mut_ptr(),
                               mins.as_mut_ptr(), maxs.as_mut_ptr(),
                               0 as *mut vec_t);
    mid[0usize] = mins[0usize] + maxs[0usize];
    mid[1usize] = mins[1usize] + maxs[1usize];
    mid[2usize] = mins[2usize] + maxs[2usize];
    mid[0usize] = (mid[0usize] as libc::c_double * 0.5f64) as vec_t;
    mid[1usize] = (mid[1usize] as libc::c_double * 0.5f64) as vec_t;
    mid[2usize] = (mid[2usize] as libc::c_double * 0.5f64) as vec_t;
    *start.offset(0isize) = mid[0usize];
    *start.offset(1isize) = mid[1usize];
    *start.offset(2isize) = mid[2usize];
    *end.offset(0isize) = mid[0usize];
    *end.offset(1isize) = mid[1usize];
    *end.offset(2isize) = mid[2usize];
    spawnflags = (*reach).facenum >> 16i32;
    num0 = (*reach).edgenum >> 16i32;
    if num0 > 0x7fffi32 {
        num0 = (num0 as libc::c_uint | 0xffff0000u32) as libc::c_int
    }
    num1 = (*reach).edgenum & 0xffffi32;
    if num1 > 0x7fffi32 {
        num1 = (num1 as libc::c_uint | 0xffff0000u32) as libc::c_int
    }
    if 0 != spawnflags & 1i32 {
        *start.offset(0isize) = num0 as vec_t;
        *end.offset(0isize) = num1 as vec_t;
        let ref mut fresh0 = *origin.offset(0isize);
        *fresh0 += mid[0usize];
        *origin.offset(1isize) = mid[1usize];
        *origin.offset(2isize) = mid[2usize]
    } else if 0 != spawnflags & 2i32 {
        *start.offset(1isize) = num0 as vec_t;
        *end.offset(1isize) = num1 as vec_t;
        *origin.offset(0isize) = mid[0usize];
        let ref mut fresh1 = *origin.offset(1isize);
        *fresh1 += mid[1usize];
        *origin.offset(2isize) = mid[2usize]
    } else {
        *start.offset(2isize) = num0 as vec_t;
        *end.offset(2isize) = num1 as vec_t;
        *origin.offset(0isize) = mid[0usize];
        *origin.offset(1isize) = mid[1usize];
        let ref mut fresh2 = *origin.offset(2isize);
        *fresh2 += mid[2usize]
    };
}
//end of the function BotTravel_JumpPad
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_JumpPad(mut ms: *mut bot_movestate_t,
                                                 mut reach:
                                                     *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut speed: libc::c_float = 0.;
    let mut hordir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if 0 ==
           BotAirControl((*ms).origin.as_mut_ptr(),
                         (*ms).velocity.as_mut_ptr(),
                         (*reach).end.as_mut_ptr(), hordir.as_mut_ptr(),
                         &mut speed) {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        speed = 400i32 as libc::c_float
    }
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end else
//end of the function Intersection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCheckBlocked(mut ms: *mut bot_movestate_t,
                                         mut dir: *mut vec_t,
                                         mut checkbottom: libc::c_int,
                                         mut result: *mut bot_moveresult_t) {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
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
    AAS_PresenceTypeBoundingBox((*ms).presencetype, mins.as_mut_ptr(),
                                maxs.as_mut_ptr());
    if fabs((*dir.offset(0isize) * up[0usize] +
                 *dir.offset(1isize) * up[1usize] +
                 *dir.offset(2isize) * up[2usize]) as libc::c_double) < 0.7f64
       {
        mins[2usize] += (*sv_maxstep).value;
        maxs[2usize] -= 10i32 as libc::c_float
    }
    end[0usize] =
        (*ms).origin[0usize] + *dir.offset(0isize) * 3i32 as libc::c_float;
    end[1usize] =
        (*ms).origin[1usize] + *dir.offset(1isize) * 3i32 as libc::c_float;
    end[2usize] =
        (*ms).origin[2usize] + *dir.offset(2isize) * 3i32 as libc::c_float;
    trace =
        AAS_Trace((*ms).origin.as_mut_ptr(), mins.as_mut_ptr(),
                  maxs.as_mut_ptr(), end.as_mut_ptr(), (*ms).entitynum,
                  1i32 | 0x10000i32 | 0x2000000i32);
    if 0 == trace.startsolid as u64 &&
           (trace.ent != (1i32 << 10i32) - 2i32 &&
                trace.ent != (1i32 << 10i32) - 1i32) {
        (*result).blocked = qtrue as libc::c_int;
        (*result).blockentity = trace.ent
    } else if 0 != checkbottom && 0 == AAS_AreaReachability((*ms).areanum) {
        AAS_PresenceTypeBoundingBox((*ms).presencetype, mins.as_mut_ptr(),
                                    maxs.as_mut_ptr());
        end[0usize] =
            (*ms).origin[0usize] + up[0usize] * -3i32 as libc::c_float;
        end[1usize] =
            (*ms).origin[1usize] + up[1usize] * -3i32 as libc::c_float;
        end[2usize] =
            (*ms).origin[2usize] + up[2usize] * -3i32 as libc::c_float;
        trace =
            AAS_Trace((*ms).origin.as_mut_ptr(), mins.as_mut_ptr(),
                      maxs.as_mut_ptr(), end.as_mut_ptr(), (*ms).entitynum,
                      1i32 | 0x10000i32);
        if 0 == trace.startsolid as u64 &&
               (trace.ent != (1i32 << 10i32) - 2i32 &&
                    trace.ent != (1i32 << 10i32) - 1i32) {
            (*result).blocked = qtrue as libc::c_int;
            (*result).blockentity = trace.ent;
            (*result).flags |= 32i32
        }
    };
}
//used to avoid reachability links for some time after being used
//avoid links for 6 seconds after use
//prediction times
//in seconds
//in seconds
//weapon indexes for weapon jumping
#[no_mangle]
pub static mut sv_maxstep: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotTravel_WalkOffLedge
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotAirControl(mut origin: *mut vec_t,
                                       mut velocity: *mut vec_t,
                                       mut goal: *mut vec_t,
                                       mut dir: *mut vec_t,
                                       mut speed: *mut libc::c_float)
 -> libc::c_int {
    let mut org: vec3_t = [0.; 3];
    let mut vel: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    org[0usize] = *origin.offset(0isize);
    org[1usize] = *origin.offset(1isize);
    org[2usize] = *origin.offset(2isize);
    vel[0usize] =
        (*velocity.offset(0isize) as libc::c_double * 0.1f64) as vec_t;
    vel[1usize] =
        (*velocity.offset(1isize) as libc::c_double * 0.1f64) as vec_t;
    vel[2usize] =
        (*velocity.offset(2isize) as libc::c_double * 0.1f64) as vec_t;
    i = 0i32;
    while i < 50i32 {
        vel[2usize] =
            (vel[2usize] as libc::c_double -
                 (*sv_gravity).value as libc::c_double * 0.01f64) as vec_t;
        if vel[2usize] < 0i32 as libc::c_float &&
               org[2usize] + vel[2usize] < *goal.offset(2isize) {
            vel[0usize] =
                vel[0usize] *
                    ((*goal.offset(2isize) - org[2usize]) / vel[2usize]);
            vel[1usize] =
                vel[1usize] *
                    ((*goal.offset(2isize) - org[2usize]) / vel[2usize]);
            vel[2usize] =
                vel[2usize] *
                    ((*goal.offset(2isize) - org[2usize]) / vel[2usize]);
            org[0usize] = org[0usize] + vel[0usize];
            org[1usize] = org[1usize] + vel[1usize];
            org[2usize] = org[2usize] + vel[2usize];
            *dir.offset(0isize) = *goal.offset(0isize) - org[0usize];
            *dir.offset(1isize) = *goal.offset(1isize) - org[1usize];
            *dir.offset(2isize) = *goal.offset(2isize) - org[2usize];
            dist = VectorNormalize(dir);
            if dist > 32i32 as libc::c_float { dist = 32i32 as libc::c_float }
            *speed =
                400i32 as libc::c_float -
                    (400i32 as libc::c_float - 13i32 as libc::c_float * dist);
            return qtrue as libc::c_int
        } else {
            org[0usize] = org[0usize] + vel[0usize];
            org[1usize] = org[1usize] + vel[1usize];
            org[2usize] = org[2usize] + vel[2usize]
        }
        i += 1
    }
    *dir.offset(0isize) = 0i32 as vec_t;
    *dir.offset(1isize) = 0i32 as vec_t;
    *dir.offset(2isize) = 0i32 as vec_t;
    *speed = 400i32 as libc::c_float;
    return qfalse as libc::c_int;
}
#[no_mangle]
pub static mut sv_gravity: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotTravel_BFGJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_WeaponJump(mut ms:
                                                        *mut bot_movestate_t,
                                                    mut reach:
                                                        *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if 0 == (*ms).jumpreach { return result }
    if 0 ==
           BotAirControl((*ms).origin.as_mut_ptr(),
                         (*ms).velocity.as_mut_ptr(),
                         (*reach).end.as_mut_ptr(), hordir.as_mut_ptr(),
                         &mut speed) {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        speed = 400i32 as libc::c_float
    }
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//DEBUG_GRAPPLE
//end if
//end if
//end of the function BotResetGrapple
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Grapple(mut ms: *mut bot_movestate_t,
                                           mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut viewdir: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut state: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
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
    if 0 != (*ms).moveflags & 256i32 {
        if 0. != (*offhandgrapple).value {
            EA_Command((*ms).client, (*cmd_grappleoff).string);
        }
        (*ms).moveflags &= !128i32;
        return result
    }
    if 0 == (*offhandgrapple).value as libc::c_int {
        result.weapon = (*weapindex_grapple).value as libc::c_int;
        result.flags |= 16i32
    }
    if 0 != (*ms).moveflags & 128i32 {
        state = GrappleState(ms, reach);
        dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
        dir[2usize] = 0i32 as vec_t;
        dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
        if 0 != state && dist < 48i32 as libc::c_float {
            if (*ms).lastgrappledist - dist < 1i32 as libc::c_float {
                if 0. != (*offhandgrapple).value {
                    EA_Command((*ms).client, (*cmd_grappleoff).string);
                }
                (*ms).moveflags &= !128i32;
                (*ms).moveflags |= 256i32;
                (*ms).reachability_time = 0i32 as libc::c_float;
                return result
            }
        } else if 0 == state ||
                      state == 2i32 &&
                          dist > (*ms).lastgrappledist - 2i32 as libc::c_float
         {
            if ((*ms).grapplevisible_time as libc::c_double) <
                   AAS_Time() as libc::c_double - 0.4f64 {
                if 0. != (*offhandgrapple).value {
                    EA_Command((*ms).client, (*cmd_grappleoff).string);
                }
                (*ms).moveflags &= !128i32;
                (*ms).moveflags |= 256i32;
                (*ms).reachability_time = 0i32 as libc::c_float;
                return result
            }
        } else { (*ms).grapplevisible_time = AAS_Time() }
        if 0 == (*offhandgrapple).value as libc::c_int {
            EA_Attack((*ms).client);
        }
        (*ms).lastgrappledist = dist
    } else {
        (*ms).grapplevisible_time = AAS_Time();
        dir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
        dir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
        dir[2usize] = (*reach).start[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { dir[2usize] = 0i32 as vec_t }
        org[0usize] = (*ms).origin[0usize] + (*ms).viewoffset[0usize];
        org[1usize] = (*ms).origin[1usize] + (*ms).viewoffset[1usize];
        org[2usize] = (*ms).origin[2usize] + (*ms).viewoffset[2usize];
        viewdir[0usize] = (*reach).end[0usize] - org[0usize];
        viewdir[1usize] = (*reach).end[1usize] - org[1usize];
        viewdir[2usize] = (*reach).end[2usize] - org[2usize];
        dist = VectorNormalize(dir.as_mut_ptr());
        vectoangles(viewdir.as_mut_ptr() as *const vec_t,
                    result.ideal_viewangles.as_mut_ptr());
        result.flags |= 1i32;
        if dist < 5i32 as libc::c_float &&
               fabs(AngleDiff(result.ideal_viewangles[0usize],
                              (*ms).viewangles[0usize]) as libc::c_double) <
                   2i32 as libc::c_double &&
               fabs(AngleDiff(result.ideal_viewangles[1usize],
                              (*ms).viewangles[1usize]) as libc::c_double) <
                   2i32 as libc::c_double {
            org[0usize] = (*ms).origin[0usize] + (*ms).viewoffset[0usize];
            org[1usize] = (*ms).origin[1usize] + (*ms).viewoffset[1usize];
            org[2usize] = (*ms).origin[2usize] + (*ms).viewoffset[2usize];
            trace =
                AAS_Trace(org.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t,
                          (*reach).end.as_mut_ptr(), (*ms).entitynum, 1i32);
            dir[0usize] = (*reach).end[0usize] - trace.endpos[0usize];
            dir[1usize] = (*reach).end[1usize] - trace.endpos[1usize];
            dir[2usize] = (*reach).end[2usize] - trace.endpos[2usize];
            if VectorLength(dir.as_mut_ptr() as *const vec_t) >
                   16i32 as libc::c_float {
                result.failure = qtrue as libc::c_int;
                return result
            }
            if 0. != (*offhandgrapple).value {
                EA_Command((*ms).client, (*cmd_grappleon).string);
            } else { EA_Attack((*ms).client); }
            (*ms).moveflags |= 128i32;
            (*ms).lastgrappledist = 999999i32 as libc::c_float
        } else {
            if dist < 70i32 as libc::c_float {
                speed =
                    300i32 as libc::c_float -
                        (300i32 as libc::c_float -
                             4i32 as libc::c_float * dist)
            } else { speed = 400i32 as libc::c_float }
            BotCheckBlocked(ms, dir.as_mut_ptr(), qtrue as libc::c_int,
                            &mut result);
            EA_Move((*ms).client, dir.as_mut_ptr(), speed);
            result.movedir[0usize] = dir[0usize];
            result.movedir[1usize] = dir[1usize];
            result.movedir[2usize] = dir[2usize]
        }
        areanum = AAS_PointAreaNum((*ms).origin.as_mut_ptr());
        if 0 != areanum && areanum != (*ms).reachareanum {
            (*ms).reachability_time = 0i32 as libc::c_float
        }
    }
    return result;
}
#[no_mangle]
pub static mut cmd_grappleon: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
#[no_mangle]
pub static mut offhandgrapple: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotInitMoveState
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn AngleDiff(mut ang1: libc::c_float,
                                   mut ang2: libc::c_float) -> libc::c_float {
    let mut diff: libc::c_float = 0.;
    diff = ang1 - ang2;
    if ang1 > ang2 {
        if diff as libc::c_double > 180.0f64 {
            diff = (diff as libc::c_double - 360.0f64) as libc::c_float
        }
    } else if (diff as libc::c_double) < -180.0f64 {
        diff = (diff as libc::c_double + 360.0f64) as libc::c_float
    }
    return diff;
}
#[no_mangle]
pub static mut cmd_grappleoff: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotFinishTravel_FuncBobbing
//===========================================================================
// 0  no valid grapple hook visible
// 1  the grapple hook is still flying
// 2  the grapple hooked into a wall
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn GrappleState(mut ms: *mut bot_movestate_t,
                                      mut reach: *mut aas_reachability_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut entinfo: aas_entityinfo_t =
        aas_entityinfo_s{valid: 0,
                         type_0: 0,
                         flags: 0,
                         ltime: 0.,
                         update_time: 0.,
                         number: 0,
                         origin: [0.; 3],
                         angles: [0.; 3],
                         old_origin: [0.; 3],
                         lastvisorigin: [0.; 3],
                         mins: [0.; 3],
                         maxs: [0.; 3],
                         groundent: 0,
                         solid: 0,
                         modelindex: 0,
                         modelindex2: 0,
                         frame: 0,
                         event: 0,
                         eventParm: 0,
                         powerups: 0,
                         weapon: 0,
                         legsAnim: 0,
                         torsoAnim: 0,};
    if 0 != (*ms).moveflags & 64i32 { return 2i32 }
    i = AAS_NextEntity(0i32);
    while 0 != i {
        if AAS_EntityType(i) == (*entitytypemissile).value as libc::c_int {
            AAS_EntityInfo(i, &mut entinfo);
            if entinfo.weapon == (*weapindex_grapple).value as libc::c_int {
                return 1i32
            }
        }
        i = AAS_NextEntity(i)
    }
    return 0i32;
}
#[no_mangle]
pub static mut weapindex_grapple: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
#[no_mangle]
pub static mut entitytypemissile: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotTravel_Elevator
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_Elevator(mut ms:
                                                      *mut bot_movestate_t,
                                                  mut reach:
                                                      *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut bottomcenter: vec3_t = [0.; 3];
    let mut bottomdir: vec3_t = [0.; 3];
    let mut topdir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
    bottomdir[0usize] = bottomcenter[0usize] - (*ms).origin[0usize];
    bottomdir[1usize] = bottomcenter[1usize] - (*ms).origin[1usize];
    bottomdir[2usize] = bottomcenter[2usize] - (*ms).origin[2usize];
    topdir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    topdir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    topdir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
    if fabs(bottomdir[2usize] as libc::c_double) <
           fabs(topdir[2usize] as libc::c_double) {
        VectorNormalize(bottomdir.as_mut_ptr());
        EA_Move((*ms).client, bottomdir.as_mut_ptr(),
                300i32 as libc::c_float);
    } else {
        VectorNormalize(topdir.as_mut_ptr());
        EA_Move((*ms).client, topdir.as_mut_ptr(), 300i32 as libc::c_float);
    }
    return result;
}
//end of the function BotTravel_WaterJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_WaterJump(mut ms:
                                                       *mut bot_movestate_t,
                                                   mut reach:
                                                       *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dir: vec3_t = [0.; 3];
    let mut pnt: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if 0 != (*ms).moveflags & 16i32 { return result }
    pnt[0usize] = (*ms).origin[0usize];
    pnt[1usize] = (*ms).origin[1usize];
    pnt[2usize] = (*ms).origin[2usize];
    pnt[2usize] -= 32i32 as libc::c_float;
    if 0 == AAS_PointContents(pnt.as_mut_ptr()) & (8i32 | 16i32 | 32i32) {
        return result
    }
    dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
    dir[0usize] =
        (dir[0usize] as libc::c_double +
             2.0f64 *
                 (((rand() & 0x7fffi32) as libc::c_float /
                       0x7fffi32 as libc::c_float) as libc::c_double - 0.5f64)
                 * 10i32 as libc::c_double) as vec_t;
    dir[1usize] =
        (dir[1usize] as libc::c_double +
             2.0f64 *
                 (((rand() & 0x7fffi32) as libc::c_float /
                       0x7fffi32 as libc::c_float) as libc::c_double - 0.5f64)
                 * 10i32 as libc::c_double) as vec_t;
    dir[2usize] =
        (dir[2usize] as libc::c_double +
             (70i32 as libc::c_double +
                  2.0f64 *
                      (((rand() & 0x7fffi32) as libc::c_float /
                            0x7fffi32 as libc::c_float) as libc::c_double -
                           0.5f64) * 10i32 as libc::c_double)) as vec_t;
    EA_Move((*ms).client, dir.as_mut_ptr(), 400i32 as libc::c_float);
    vectoangles(dir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    result.flags |= 1i32;
    result.movedir[0usize] = dir[0usize];
    result.movedir[1usize] = dir[1usize];
    result.movedir[2usize] = dir[2usize];
    return result;
}
//end of the function BotFinishTravel_BarrierJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Swim(mut ms: *mut bot_movestate_t,
                                        mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    dir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    dir[2usize] = (*reach).start[2usize] - (*ms).origin[2usize];
    VectorNormalize(dir.as_mut_ptr());
    BotCheckBlocked(ms, dir.as_mut_ptr(), qtrue as libc::c_int, &mut result);
    EA_Move((*ms).client, dir.as_mut_ptr(), 400i32 as libc::c_float);
    result.movedir[0usize] = dir[0usize];
    result.movedir[1usize] = dir[1usize];
    result.movedir[2usize] = dir[2usize];
    vectoangles(dir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    result.flags |= 2i32;
    return result;
}
//end of the function BotTravel_Jump*/
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_Jump(mut ms: *mut bot_movestate_t,
                                              mut reach:
                                                  *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut hordir2: vec3_t = [0.; 3];
    let mut speed: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if 0 == (*ms).jumpreach { return result }
    hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    hordir2[0usize] = (*reach).end[0usize] - (*reach).start[0usize];
    hordir2[1usize] = (*reach).end[1usize] - (*reach).start[1usize];
    hordir2[2usize] = 0i32 as vec_t;
    VectorNormalize(hordir2.as_mut_ptr());
    if ((hordir[0usize] * hordir2[0usize] + hordir[1usize] * hordir2[1usize] +
             hordir[2usize] * hordir2[2usize]) as libc::c_double) < -0.5f64 &&
           dist < 24i32 as libc::c_float {
        return result
    }
    speed = 800i32 as libc::c_float;
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotAirControl
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_WalkOffLedge(mut ms:
                                                          *mut bot_movestate_t,
                                                      mut reach:
                                                          *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dir: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
    BotCheckBlocked(ms, dir.as_mut_ptr(), qtrue as libc::c_int, &mut result);
    v[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    v[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    v[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
    v[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(v.as_mut_ptr());
    if dist > 16i32 as libc::c_float {
        end[0usize] =
            (*reach).end[0usize] + v[0usize] * 16i32 as libc::c_float;
        end[1usize] =
            (*reach).end[1usize] + v[1usize] * 16i32 as libc::c_float;
        end[2usize] =
            (*reach).end[2usize] + v[2usize] * 16i32 as libc::c_float
    } else {
        end[0usize] = (*reach).end[0usize];
        end[1usize] = (*reach).end[1usize];
        end[2usize] = (*reach).end[2usize]
    }
    if 0 ==
           BotAirControl((*ms).origin.as_mut_ptr(),
                         (*ms).velocity.as_mut_ptr(), end.as_mut_ptr(),
                         hordir.as_mut_ptr(), &mut speed) {
        hordir[0usize] = dir[0usize];
        hordir[1usize] = dir[1usize];
        hordir[2usize] = dir[2usize];
        hordir[2usize] = 0i32 as vec_t;
        speed = 400i32 as libc::c_float
    }
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotFinishTravel_Jump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Ladder(mut ms: *mut bot_movestate_t,
                                          mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    //float dist, speed;
    //, hordir;
    let mut dir: vec3_t = [0.; 3];
    let mut viewdir: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    //	vec3_t up = {0, 0, 1};
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
    VectorNormalize(dir.as_mut_ptr());
    viewdir[0usize] = dir[0usize];
    viewdir[1usize] = dir[1usize];
    viewdir[2usize] = 3i32 as libc::c_float * dir[2usize];
    vectoangles(viewdir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    EA_Move((*ms).client, origin.as_mut_ptr(), 0i32 as libc::c_float);
    EA_MoveForward((*ms).client);
    result.flags |= 1i32;
    result.movedir[0usize] = dir[0usize];
    result.movedir[1usize] = dir[1usize];
    result.movedir[2usize] = dir[2usize];
    return result;
}
//end of the function BotTravel_BarrierJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_BarrierJump(mut ms:
                                                         *mut bot_movestate_t,
                                                     mut reach:
                                                         *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if (*ms).velocity[2usize] < 250i32 as libc::c_float {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                        &mut result);
        EA_Move((*ms).client, hordir.as_mut_ptr(), 400i32 as libc::c_float);
        result.movedir[0usize] = hordir[0usize];
        result.movedir[1usize] = hordir[1usize];
        result.movedir[2usize] = hordir[2usize]
    }
    return result;
}
//DEBUG
//end if
//end else
//end of the function BotCheckBlocked
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Walk(mut ms: *mut bot_movestate_t,
                                        mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut hordir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    if dist < 10i32 as libc::c_float {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        dist = VectorNormalize(hordir.as_mut_ptr())
    }
    if 0 == AAS_AreaPresenceType((*reach).areanum) & 2i32 {
        if dist < 20i32 as libc::c_float { EA_Crouch((*ms).client); }
    }
    dist =
        BotGapDistance((*ms).origin.as_mut_ptr(), hordir.as_mut_ptr(),
                       (*ms).entitynum);
    if 0 != (*ms).moveflags & 512i32 {
        if dist > 0i32 as libc::c_float {
            speed =
                200i32 as libc::c_float -
                    (180i32 as libc::c_float - 1i32 as libc::c_float * dist)
        } else { speed = 200i32 as libc::c_float }
        EA_Walk((*ms).client);
    } else if dist > 0i32 as libc::c_float {
        speed =
            400i32 as libc::c_float -
                (360i32 as libc::c_float - 2i32 as libc::c_float * dist)
    } else { speed = 400i32 as libc::c_float }
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function MoverBottomCenter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotGapDistance(mut origin: *mut vec_t,
                                        mut hordir: *mut vec_t,
                                        mut entnum: libc::c_int)
 -> libc::c_float {
    let mut dist: libc::c_int = 0;
    let mut startz: libc::c_float = 0.;
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
    start[0usize] = *origin.offset(0isize);
    start[1usize] = *origin.offset(1isize);
    start[2usize] = *origin.offset(2isize);
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    end[2usize] -= 60i32 as libc::c_float;
    trace =
        AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 4i32,
                            entnum);
    if trace.fraction >= 1i32 as libc::c_float {
        return 1i32 as libc::c_float
    }
    startz = trace.endpos[2usize] + 1i32 as libc::c_float;
    dist = 8i32;
    while dist <= 100i32 {
        start[0usize] =
            *origin.offset(0isize) +
                *hordir.offset(0isize) * dist as libc::c_float;
        start[1usize] =
            *origin.offset(1isize) +
                *hordir.offset(1isize) * dist as libc::c_float;
        start[2usize] =
            *origin.offset(2isize) +
                *hordir.offset(2isize) * dist as libc::c_float;
        start[2usize] = startz + 24i32 as libc::c_float;
        end[0usize] = start[0usize];
        end[1usize] = start[1usize];
        end[2usize] = start[2usize];
        end[2usize] -= 48i32 as libc::c_float + (*sv_maxbarrier).value;
        trace =
            AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 4i32,
                                entnum);
        //if solid is found the bot can't walk any further and fall into a gap
        if 0 == trace.startsolid as u64 {
            //if it is a gap
            if trace.endpos[2usize] <
                   startz - (*sv_maxstep).value - 8i32 as libc::c_float {
                end[0usize] = trace.endpos[0usize];
                end[1usize] = trace.endpos[1usize];
                end[2usize] = trace.endpos[2usize];
                end[2usize] -= 20i32 as libc::c_float;
                if 0 != AAS_PointContents(end.as_mut_ptr()) & 32i32 {
                    break ;
                }
                return dist as libc::c_float
            } else { startz = trace.endpos[2usize] }
        }
        dist += 8i32
    }
    return 0i32 as libc::c_float;
}
#[no_mangle]
pub static mut sv_maxbarrier: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotAddAvoidSpot
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotGetReachabilityToGoal(mut origin: *mut vec_t,
                                                  mut areanum: libc::c_int,
                                                  mut lastgoalareanum:
                                                      libc::c_int,
                                                  mut lastareanum:
                                                      libc::c_int,
                                                  mut avoidreach:
                                                      *mut libc::c_int,
                                                  mut avoidreachtimes:
                                                      *mut libc::c_float,
                                                  mut avoidreachtries:
                                                      *mut libc::c_int,
                                                  mut goal: *mut bot_goal_t,
                                                  mut travelflags:
                                                      libc::c_int,
                                                  mut avoidspots:
                                                      *mut bot_avoidspot_s,
                                                  mut numavoidspots:
                                                      libc::c_int,
                                                  mut flags: *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut besttime: libc::c_int = 0;
    let mut bestreachnum: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0;
    let mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    if 0 == areanum { return 0i32 }
    if 0 != AAS_AreaDoNotEnter(areanum) ||
           0 != AAS_AreaDoNotEnter((*goal).areanum) {
        travelflags |= 0x800000i32
    }
    besttime = 0i32;
    bestreachnum = 0i32;
    reachnum = AAS_NextAreaReachability(areanum, 0i32);
    while 0 != reachnum {
        i = 0i32;
        while i < 1i32 {
            if *avoidreach.offset(i as isize) == reachnum &&
                   *avoidreachtimes.offset(i as isize) >= AAS_Time() {
                break ;
            }
            i += 1
        }
        //end for
        if !(i != 1i32 && *avoidreachtries.offset(i as isize) > 4i32) {
            //DEBUG
            AAS_ReachabilityFromNum(reachnum, &mut reach);
            //NOTE: do not go back to the previous area if the goal didn't change
		//NOTE: is this actually avoidance of local routing minima between two areas???
            if !(lastgoalareanum == (*goal).areanum &&
                     reach.areanum == lastareanum) {
                //if (AAS_AreaContentsTravelFlags(reach.areanum) & ~travelflags) continue;
		//if the travel isn't valid
                if !(0 == BotValidTravel(origin, &mut reach, travelflags)) {
                    t =
                        AAS_AreaTravelTimeToGoalArea(reach.areanum,
                                                     reach.end.as_mut_ptr(),
                                                     (*goal).areanum,
                                                     travelflags);
                    //if the goal area isn't reachable from the reachable area
                    if !(0 == t) {
                        //if the bot should not use this reachability to avoid bad spots
                        if 0 !=
                               BotAvoidSpots(origin, &mut reach, avoidspots,
                                             numavoidspots) {
                            if !flags.is_null() { *flags |= 256i32 }
                        } else {
                            t += reach.traveltime as libc::c_int;
                            if 0 == besttime || t < besttime {
                                besttime = t;
                                bestreachnum = reachnum
                            }
                        }
                    }
                }
            }
        }
        reachnum = AAS_NextAreaReachability(areanum, reachnum)
    }
    return bestreachnum;
}
//end of the function VectorDistanceSquared
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotAvoidSpots(mut origin: *mut vec_t,
                                       mut reach: *mut aas_reachability_t,
                                       mut avoidspots: *mut bot_avoidspot_t,
                                       mut numavoidspots: libc::c_int)
 -> libc::c_int {
    let mut checkbetween: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut squareddist: libc::c_float = 0.;
    let mut squaredradius: libc::c_float = 0.;
    match (*reach).traveltype & 0xffffffi32 {
        2 => { checkbetween = qtrue as libc::c_int }
        3 => { checkbetween = qtrue as libc::c_int }
        4 => { checkbetween = qtrue as libc::c_int }
        6 => { checkbetween = qtrue as libc::c_int }
        7 => { checkbetween = qfalse as libc::c_int }
        5 => { checkbetween = qfalse as libc::c_int }
        8 => { checkbetween = qtrue as libc::c_int }
        9 => { checkbetween = qtrue as libc::c_int }
        10 => { checkbetween = qfalse as libc::c_int }
        11 => { checkbetween = qfalse as libc::c_int }
        14 => { checkbetween = qfalse as libc::c_int }
        12 => { checkbetween = qfalse as libc::c_int }
        13 => { checkbetween = qfalse as libc::c_int }
        18 => { checkbetween = qfalse as libc::c_int }
        19 => { checkbetween = qfalse as libc::c_int }
        _ => { checkbetween = qtrue as libc::c_int }
    }
    type_0 = 0i32;
    i = 0i32;
    while i < numavoidspots {
        squaredradius =
            (*avoidspots.offset(i as isize)).radius *
                (*avoidspots.offset(i as isize)).radius;
        squareddist =
            DistanceFromLineSquared((*avoidspots.offset(i as
                                                            isize)).origin.as_mut_ptr(),
                                    origin, (*reach).start.as_mut_ptr());
        if squareddist < squaredradius &&
               VectorDistanceSquared((*avoidspots.offset(i as
                                                             isize)).origin.as_mut_ptr(),
                                     origin) > squareddist {
            type_0 = (*avoidspots.offset(i as isize)).type_0
        } else if 0 != checkbetween {
            squareddist =
                DistanceFromLineSquared((*avoidspots.offset(i as
                                                                isize)).origin.as_mut_ptr(),
                                        (*reach).start.as_mut_ptr(),
                                        (*reach).end.as_mut_ptr());
            if squareddist < squaredradius &&
                   VectorDistanceSquared((*avoidspots.offset(i as
                                                                 isize)).origin.as_mut_ptr(),
                                         (*reach).start.as_mut_ptr()) >
                       squareddist {
                type_0 = (*avoidspots.offset(i as isize)).type_0
            }
        } else {
            VectorDistanceSquared((*avoidspots.offset(i as
                                                          isize)).origin.as_mut_ptr(),
                                  (*reach).end.as_mut_ptr());
            if squareddist < squaredradius &&
                   VectorDistanceSquared((*avoidspots.offset(i as
                                                                 isize)).origin.as_mut_ptr(),
                                         (*reach).start.as_mut_ptr()) >
                       squareddist {
                type_0 = (*avoidspots.offset(i as isize)).type_0
            }
        }
        if type_0 == 1i32 { return type_0 }
        i += 1
    }
    return type_0;
}
//end of the function DistanceFromLineSquared
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn VectorDistanceSquared(mut p1: *mut vec_t,
                                               mut p2: *mut vec_t)
 -> libc::c_float {
    let mut dir: vec3_t = [0.; 3];
    dir[0usize] = *p2.offset(0isize) - *p1.offset(0isize);
    dir[1usize] = *p2.offset(1isize) - *p1.offset(1isize);
    dir[2usize] = *p2.offset(2isize) - *p1.offset(2isize);
    return VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
}
//end if
//end for
//end of the function BotAddToAvoidReach
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn DistanceFromLineSquared(mut p: *mut vec_t,
                                                 mut lp1: *mut vec_t,
                                                 mut lp2: *mut vec_t)
 -> libc::c_float {
    let mut proj: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut j: libc::c_int = 0;
    AAS_ProjectPointOntoVector(p, lp1, lp2, proj.as_mut_ptr());
    j = 0i32;
    while j < 3i32 {
        if proj[j as usize] > *lp1.offset(j as isize) &&
               proj[j as usize] > *lp2.offset(j as isize) ||
               proj[j as usize] < *lp1.offset(j as isize) &&
                   proj[j as usize] < *lp2.offset(j as isize) {
            break ;
        }
        j += 1
    }
    if j < 3i32 {
        if fabs((proj[j as usize] - *lp1.offset(j as isize)) as
                    libc::c_double) <
               fabs((proj[j as usize] - *lp2.offset(j as isize)) as
                        libc::c_double) {
            dir[0usize] = *p.offset(0isize) - *lp1.offset(0isize);
            dir[1usize] = *p.offset(1isize) - *lp1.offset(1isize);
            dir[2usize] = *p.offset(2isize) - *lp1.offset(2isize)
        } else {
            dir[0usize] = *p.offset(0isize) - *lp2.offset(0isize);
            dir[1usize] = *p.offset(1isize) - *lp2.offset(1isize);
            dir[2usize] = *p.offset(2isize) - *lp2.offset(2isize)
        }
        return VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
    }
    dir[0usize] = *p.offset(0isize) - proj[0usize];
    dir[1usize] = *p.offset(1isize) - proj[1usize];
    dir[2usize] = *p.offset(2isize) - proj[2usize];
    return VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
}
//end of the function BotOnTopOfEntity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotValidTravel(mut origin: *mut vec_t,
                                        mut reach: *mut aas_reachability_t,
                                        mut travelflags: libc::c_int)
 -> libc::c_int {
    if 0 != AAS_TravelFlagForType((*reach).traveltype) & !travelflags {
        return qfalse as libc::c_int
    }
    if 0 != AAS_AreaContentsTravelFlags((*reach).areanum) & !travelflags {
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//end else
//end of the function BotFuncBobStartEnd
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_FuncBobbing(mut ms: *mut bot_movestate_t,
                                               mut reach:
                                                   *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dir: vec3_t = [0.; 3];
    let mut dir1: vec3_t = [0.; 3];
    let mut dir2: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut bottomcenter: vec3_t = [0.; 3];
    let mut bob_start: vec3_t = [0.; 3];
    let mut bob_end: vec3_t = [0.; 3];
    let mut bob_origin: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    BotFuncBobStartEnd(reach, bob_start.as_mut_ptr(), bob_end.as_mut_ptr(),
                       bob_origin.as_mut_ptr());
    if 0 != BotOnMover((*ms).origin.as_mut_ptr(), (*ms).entitynum, reach) {
        dir[0usize] = bob_origin[0usize] - bob_end[0usize];
        dir[1usize] = bob_origin[1usize] - bob_end[1usize];
        dir[2usize] = bob_origin[2usize] - bob_end[2usize];
        if VectorLength(dir.as_mut_ptr() as *const vec_t) <
               24i32 as libc::c_float {
            hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
            hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
            hordir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
            hordir[2usize] = 0i32 as vec_t;
            VectorNormalize(hordir.as_mut_ptr());
            if 0 ==
                   BotCheckBarrierJump(ms, hordir.as_mut_ptr(),
                                       100i32 as libc::c_float) {
                EA_Move((*ms).client, hordir.as_mut_ptr(),
                        400i32 as libc::c_float);
            }
            result.movedir[0usize] = hordir[0usize];
            result.movedir[1usize] = hordir[1usize];
            result.movedir[2usize] = hordir[2usize]
        } else {
            MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
            hordir[0usize] = bottomcenter[0usize] - (*ms).origin[0usize];
            hordir[1usize] = bottomcenter[1usize] - (*ms).origin[1usize];
            hordir[2usize] = bottomcenter[2usize] - (*ms).origin[2usize];
            hordir[2usize] = 0i32 as vec_t;
            dist = VectorNormalize(hordir.as_mut_ptr());
            if dist > 10i32 as libc::c_float {
                if dist > 100i32 as libc::c_float {
                    dist = 100i32 as libc::c_float
                }
                speed =
                    400i32 as libc::c_float -
                        (400i32 as libc::c_float -
                             4i32 as libc::c_float * dist);
                EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
                result.movedir[0usize] = hordir[0usize];
                result.movedir[1usize] = hordir[1usize];
                result.movedir[2usize] = hordir[2usize]
            }
        }
    } else {
        dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
        dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
        if dist < 64i32 as libc::c_float {
            if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
            speed =
                360i32 as libc::c_float -
                    (360i32 as libc::c_float - 6i32 as libc::c_float * dist);
            if 0 != (*ms).moveflags & 4i32 ||
                   0 ==
                       BotCheckBarrierJump(ms, dir.as_mut_ptr(),
                                           50i32 as libc::c_float) {
                if speed > 5i32 as libc::c_float {
                    EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                }
            }
            result.movedir[0usize] = dir[0usize];
            result.movedir[1usize] = dir[1usize];
            result.movedir[2usize] = dir[2usize];
            if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
            (*ms).reachability_time = 0i32 as libc::c_float;
            return result
        }
        dir1[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
        dir1[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
        dir1[2usize] = (*reach).start[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { dir1[2usize] = 0i32 as vec_t }
        dist1 = VectorNormalize(dir1.as_mut_ptr());
        dir[0usize] = bob_origin[0usize] - bob_start[0usize];
        dir[1usize] = bob_origin[1usize] - bob_start[1usize];
        dir[2usize] = bob_origin[2usize] - bob_start[2usize];
        if VectorLength(dir.as_mut_ptr() as *const vec_t) >
               16i32 as libc::c_float {
            dist = dist1;
            dir[0usize] = dir1[0usize];
            dir[1usize] = dir1[1usize];
            dir[2usize] = dir1[2usize];
            BotCheckBlocked(ms, dir.as_mut_ptr(), qfalse as libc::c_int,
                            &mut result);
            if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
            speed =
                360i32 as libc::c_float -
                    (360i32 as libc::c_float - 6i32 as libc::c_float * dist);
            if 0 == (*ms).moveflags & 4i32 &&
                   0 ==
                       BotCheckBarrierJump(ms, dir.as_mut_ptr(),
                                           50i32 as libc::c_float) {
                if speed > 5i32 as libc::c_float {
                    EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                }
            }
            result.movedir[0usize] = dir[0usize];
            result.movedir[1usize] = dir[1usize];
            result.movedir[2usize] = dir[2usize];
            if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
            result.type_0 = 2i32;
            result.flags |= 4i32;
            return result
        }
        MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
        dir2[0usize] = bottomcenter[0usize] - (*ms).origin[0usize];
        dir2[1usize] = bottomcenter[1usize] - (*ms).origin[1usize];
        dir2[2usize] = bottomcenter[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { dir2[2usize] = 0i32 as vec_t }
        dist2 = VectorNormalize(dir2.as_mut_ptr());
        if dist1 < 20i32 as libc::c_float || dist2 < dist1 ||
               dir1[0usize] * dir2[0usize] + dir1[1usize] * dir2[1usize] +
                   dir1[2usize] * dir2[2usize] < 0i32 as libc::c_float {
            dist = dist2;
            dir[0usize] = dir2[0usize];
            dir[1usize] = dir2[1usize];
            dir[2usize] = dir2[2usize]
        } else {
            dist = dist1;
            dir[0usize] = dir1[0usize];
            dir[1usize] = dir1[1usize];
            dir[2usize] = dir1[2usize]
        }
        BotCheckBlocked(ms, dir.as_mut_ptr(), qfalse as libc::c_int,
                        &mut result);
        if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
        speed =
            400i32 as libc::c_float -
                (400i32 as libc::c_float - 6i32 as libc::c_float * dist);
        if 0 == (*ms).moveflags & 4i32 &&
               0 ==
                   BotCheckBarrierJump(ms, dir.as_mut_ptr(),
                                       50i32 as libc::c_float) {
            EA_Move((*ms).client, dir.as_mut_ptr(), speed);
        }
        result.movedir[0usize] = dir[0usize];
        result.movedir[1usize] = dir[1usize];
        result.movedir[2usize] = dir[2usize];
        if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
    }
    return result;
}
//end of the function BotGapDistance
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCheckBarrierJump(mut ms: *mut bot_movestate_t,
                                             mut dir: *mut vec_t,
                                             mut speed: libc::c_float)
 -> libc::c_int {
    let mut start: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut trace: aas_trace_t =
        aas_trace_s{startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    ent: 0,
                    lastarea: 0,
                    area: 0,
                    planenum: 0,};
    end[0usize] = (*ms).origin[0usize];
    end[1usize] = (*ms).origin[1usize];
    end[2usize] = (*ms).origin[2usize];
    end[2usize] += (*sv_maxbarrier).value;
    trace =
        AAS_TraceClientBBox((*ms).origin.as_mut_ptr(), end.as_mut_ptr(), 2i32,
                            (*ms).entitynum);
    if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
    if trace.endpos[2usize] - (*ms).origin[2usize] < (*sv_maxstep).value {
        return qfalse as libc::c_int
    }
    hordir[0usize] = *dir.offset(0isize);
    hordir[1usize] = *dir.offset(1isize);
    hordir[2usize] = 0i32 as vec_t;
    VectorNormalize(hordir.as_mut_ptr());
    end[0usize] =
        ((*ms).origin[0usize] as libc::c_double +
             hordir[0usize] as libc::c_double *
                 (((*ms).thinktime * speed) as libc::c_double * 0.5f64)) as
            vec_t;
    end[1usize] =
        ((*ms).origin[1usize] as libc::c_double +
             hordir[1usize] as libc::c_double *
                 (((*ms).thinktime * speed) as libc::c_double * 0.5f64)) as
            vec_t;
    end[2usize] =
        ((*ms).origin[2usize] as libc::c_double +
             hordir[2usize] as libc::c_double *
                 (((*ms).thinktime * speed) as libc::c_double * 0.5f64)) as
            vec_t;
    start[0usize] = trace.endpos[0usize];
    start[1usize] = trace.endpos[1usize];
    start[2usize] = trace.endpos[2usize];
    end[2usize] = trace.endpos[2usize];
    trace =
        AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 2i32,
                            (*ms).entitynum);
    if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
    start[0usize] = trace.endpos[0usize];
    start[1usize] = trace.endpos[1usize];
    start[2usize] = trace.endpos[2usize];
    end[0usize] = trace.endpos[0usize];
    end[1usize] = trace.endpos[1usize];
    end[2usize] = trace.endpos[2usize];
    end[2usize] = (*ms).origin[2usize];
    trace =
        AAS_TraceClientBBox(start.as_mut_ptr(), end.as_mut_ptr(), 2i32,
                            (*ms).entitynum);
    if 0 != trace.startsolid as u64 { return qfalse as libc::c_int }
    if trace.fraction as libc::c_double >= 1.0f64 {
        return qfalse as libc::c_int
    }
    if trace.endpos[2usize] - (*ms).origin[2usize] < (*sv_maxstep).value {
        return qfalse as libc::c_int
    }
    EA_Jump((*ms).client);
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    (*ms).moveflags |= 1i32;
    return qtrue as libc::c_int;
}
//end of the function BotReachabilityArea
//===========================================================================
// returns the reachability area the bot is in
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
/*
int BotReachabilityArea(vec3_t origin, int testground)
{
	int firstareanum, i, j, x, y, z;
	int areas[10], numareas, areanum, bestareanum;
	float dist, bestdist;
	vec3_t org, end, points[10], v;
	aas_trace_t trace;

	firstareanum = 0;
	for (i = 0; i < 2; i++)
	{
		VectorCopy(origin, org);
		//if test at the ground (used when bot is standing on an entity)
		if (i > 0)
		{
			VectorCopy(origin, end);
			end[2] -= 800;
			trace = AAS_TraceClientBBox(origin, end, PRESENCE_CROUCH, -1);
			if (!trace.startsolid)
			{
				VectorCopy(trace.endpos, org);
			} //end if
		} //end if

		firstareanum = 0;
		areanum = AAS_PointAreaNum(org);
		if (areanum)
		{
			firstareanum = areanum;
			if (AAS_AreaReachability(areanum)) return areanum;
		} //end if
		bestdist = 999999;
		bestareanum = 0;
		for (z = 1; z >= -1; z -= 1)
		{
			for (x = 1; x >= -1; x -= 1)
			{
				for (y = 1; y >= -1; y -= 1)
				{
					VectorCopy(org, end);
					end[0] += x * 8;
					end[1] += y * 8;
					end[2] += z * 12;
					numareas = AAS_TraceAreas(org, end, areas, points, 10);
					for (j = 0; j < numareas; j++)
					{
						if (AAS_AreaReachability(areas[j]))
						{
							VectorSubtract(points[j], org, v);
							dist = VectorLength(v);
							if (dist < bestdist)
							{
								bestareanum = areas[j];
								bestdist = dist;
							} //end if
						} //end if
					} //end for
				} //end for
			} //end for
			if (bestareanum) return bestareanum;
		} //end for
		if (!testground) break;
	} //end for
//#ifdef DEBUG
	//botimport.Print(PRT_MESSAGE, "no reachability area\n");
//#endif //DEBUG
	return firstareanum;
} //end of the function BotReachabilityArea*/
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotOnMover(mut origin: *mut vec_t,
                                    mut entnum: libc::c_int,
                                    mut reach: *mut aas_reachability_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut modelorigin: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    let mut boxmins: vec3_t =
        [-16i32 as vec_t, -16i32 as vec_t, -8i32 as vec_t];
    let mut boxmaxs: vec3_t = [16i32 as vec_t, 16i32 as vec_t, 8i32 as vec_t];
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
    modelnum = (*reach).facenum & 0xffffi32;
    AAS_BSPModelMinsMaxsOrigin(modelnum, angles.as_mut_ptr(),
                               mins.as_mut_ptr(), maxs.as_mut_ptr(),
                               0 as *mut vec_t);
    if 0 == AAS_OriginOfMoverWithModelNum(modelnum, modelorigin.as_mut_ptr())
       {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"no entity with model %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            modelnum);
        return qfalse as libc::c_int
    }
    i = 0i32;
    while i < 2i32 {
        if *origin.offset(i as isize) >
               modelorigin[i as usize] + maxs[i as usize] +
                   16i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        if *origin.offset(i as isize) <
               modelorigin[i as usize] + mins[i as usize] -
                   16i32 as libc::c_float {
            return qfalse as libc::c_int
        }
        i += 1
    }
    org[0usize] = *origin.offset(0isize);
    org[1usize] = *origin.offset(1isize);
    org[2usize] = *origin.offset(2isize);
    org[2usize] += 24i32 as libc::c_float;
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    end[2usize] -= 48i32 as libc::c_float;
    trace =
        AAS_Trace(org.as_mut_ptr(), boxmins.as_mut_ptr(),
                  boxmaxs.as_mut_ptr(), end.as_mut_ptr(), entnum,
                  1i32 | 0x10000i32);
    if 0 == trace.startsolid as u64 && 0 == trace.allsolid as u64 {
        if trace.ent != (1i32 << 10i32) - 1i32 &&
               AAS_EntityModelNum(trace.ent) == modelnum {
            return qtrue as libc::c_int
        }
    }
    return qfalse as libc::c_int;
}
//end of the function BotFinishTravel_WeaponJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_JumpPad(mut ms: *mut bot_movestate_t,
                                           mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    EA_Move((*ms).client, hordir.as_mut_ptr(), 400i32 as libc::c_float);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotTravel_RocketJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_BFGJump(mut ms: *mut bot_movestate_t,
                                           mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    if dist < 5i32 as libc::c_float &&
           fabs(AngleDiff(result.ideal_viewangles[0usize],
                          (*ms).viewangles[0usize]) as libc::c_double) <
               5i32 as libc::c_double &&
           fabs(AngleDiff(result.ideal_viewangles[1usize],
                          (*ms).viewangles[1usize]) as libc::c_double) <
               5i32 as libc::c_double {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        EA_Jump((*ms).client);
        EA_Attack((*ms).client);
        EA_Move((*ms).client, hordir.as_mut_ptr(), 800i32 as libc::c_float);
        (*ms).jumpreach = (*ms).lastreachnum
    } else {
        if dist > 80i32 as libc::c_float { dist = 80i32 as libc::c_float }
        speed =
            400i32 as libc::c_float -
                (400i32 as libc::c_float - 5i32 as libc::c_float * dist);
        EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    vectoangles(hordir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    result.ideal_viewangles[0usize] = 90i32 as vec_t;
    EA_View((*ms).client, result.ideal_viewangles.as_mut_ptr());
    result.flags |= 8i32;
    EA_SelectWeapon((*ms).client, (*weapindex_bfg10k).value as libc::c_int);
    result.weapon = (*weapindex_bfg10k).value as libc::c_int;
    result.flags |= 16i32;
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
#[no_mangle]
pub static mut weapindex_bfg10k: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotTravel_Grapple
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:			-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_RocketJump(mut ms: *mut bot_movestate_t,
                                              mut reach:
                                                  *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    vectoangles(hordir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    result.ideal_viewangles[0usize] = 90i32 as vec_t;
    if dist < 5i32 as libc::c_float &&
           fabs(AngleDiff(result.ideal_viewangles[0usize],
                          (*ms).viewangles[0usize]) as libc::c_double) <
               5i32 as libc::c_double &&
           fabs(AngleDiff(result.ideal_viewangles[1usize],
                          (*ms).viewangles[1usize]) as libc::c_double) <
               5i32 as libc::c_double {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        EA_Jump((*ms).client);
        EA_Attack((*ms).client);
        EA_Move((*ms).client, hordir.as_mut_ptr(), 800i32 as libc::c_float);
        (*ms).jumpreach = (*ms).lastreachnum
    } else {
        if dist > 80i32 as libc::c_float { dist = 80i32 as libc::c_float }
        speed =
            400i32 as libc::c_float -
                (400i32 as libc::c_float - 5i32 as libc::c_float * dist);
        EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    vectoangles(hordir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    result.ideal_viewangles[0usize] = 90i32 as vec_t;
    EA_View((*ms).client, result.ideal_viewangles.as_mut_ptr());
    result.flags |= 8i32;
    EA_SelectWeapon((*ms).client,
                    (*weapindex_rocketlauncher).value as libc::c_int);
    result.weapon = (*weapindex_rocketlauncher).value as libc::c_int;
    result.flags |= 16i32;
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
#[no_mangle]
pub static mut weapindex_rocketlauncher: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function BotTravel_Teleport
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Elevator(mut ms: *mut bot_movestate_t,
                                            mut reach:
                                                *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dir: vec3_t = [0.; 3];
    let mut dir1: vec3_t = [0.; 3];
    let mut dir2: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut bottomcenter: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if 0 != BotOnMover((*ms).origin.as_mut_ptr(), (*ms).entitynum, reach) {
        if fabsf((*ms).origin[2usize] - (*reach).end[2usize]) <
               (*sv_maxbarrier).value {
            hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
            hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
            hordir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
            hordir[2usize] = 0i32 as vec_t;
            VectorNormalize(hordir.as_mut_ptr());
            if 0 ==
                   BotCheckBarrierJump(ms, hordir.as_mut_ptr(),
                                       100i32 as libc::c_float) {
                EA_Move((*ms).client, hordir.as_mut_ptr(),
                        400i32 as libc::c_float);
            }
            result.movedir[0usize] = hordir[0usize];
            result.movedir[1usize] = hordir[1usize];
            result.movedir[2usize] = hordir[2usize]
        } else {
            MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
            hordir[0usize] = bottomcenter[0usize] - (*ms).origin[0usize];
            hordir[1usize] = bottomcenter[1usize] - (*ms).origin[1usize];
            hordir[2usize] = bottomcenter[2usize] - (*ms).origin[2usize];
            hordir[2usize] = 0i32 as vec_t;
            dist = VectorNormalize(hordir.as_mut_ptr());
            if dist > 10i32 as libc::c_float {
                if dist > 100i32 as libc::c_float {
                    dist = 100i32 as libc::c_float
                }
                speed =
                    400i32 as libc::c_float -
                        (400i32 as libc::c_float -
                             4i32 as libc::c_float * dist);
                EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
                result.movedir[0usize] = hordir[0usize];
                result.movedir[1usize] = hordir[1usize];
                result.movedir[2usize] = hordir[2usize]
            }
        }
    } else {
        dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
        dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
        if dist < 64i32 as libc::c_float {
            if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
            speed =
                360i32 as libc::c_float -
                    (360i32 as libc::c_float - 6i32 as libc::c_float * dist);
            if 0 != (*ms).moveflags & 4i32 ||
                   0 ==
                       BotCheckBarrierJump(ms, dir.as_mut_ptr(),
                                           50i32 as libc::c_float) {
                if speed > 5i32 as libc::c_float {
                    EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                }
            }
            result.movedir[0usize] = dir[0usize];
            result.movedir[1usize] = dir[1usize];
            result.movedir[2usize] = dir[2usize];
            if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
            (*ms).reachability_time = 0i32 as libc::c_float;
            return result
        }
        dir1[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
        dir1[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
        dir1[2usize] = (*reach).start[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { dir1[2usize] = 0i32 as vec_t }
        dist1 = VectorNormalize(dir1.as_mut_ptr());
        if 0 == MoverDown(reach) {
            dist = dist1;
            dir[0usize] = dir1[0usize];
            dir[1usize] = dir1[1usize];
            dir[2usize] = dir1[2usize];
            BotCheckBlocked(ms, dir.as_mut_ptr(), qfalse as libc::c_int,
                            &mut result);
            if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
            speed =
                360i32 as libc::c_float -
                    (360i32 as libc::c_float - 6i32 as libc::c_float * dist);
            if 0 == (*ms).moveflags & 4i32 &&
                   0 ==
                       BotCheckBarrierJump(ms, dir.as_mut_ptr(),
                                           50i32 as libc::c_float) {
                if speed > 5i32 as libc::c_float {
                    EA_Move((*ms).client, dir.as_mut_ptr(), speed);
                }
            }
            result.movedir[0usize] = dir[0usize];
            result.movedir[1usize] = dir[1usize];
            result.movedir[2usize] = dir[2usize];
            if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
            result.type_0 = 1i32;
            result.flags |= 4i32;
            return result
        }
        MoverBottomCenter(reach, bottomcenter.as_mut_ptr());
        dir2[0usize] = bottomcenter[0usize] - (*ms).origin[0usize];
        dir2[1usize] = bottomcenter[1usize] - (*ms).origin[1usize];
        dir2[2usize] = bottomcenter[2usize] - (*ms).origin[2usize];
        if 0 == (*ms).moveflags & 4i32 { dir2[2usize] = 0i32 as vec_t }
        dist2 = VectorNormalize(dir2.as_mut_ptr());
        if dist1 < 20i32 as libc::c_float || dist2 < dist1 ||
               dir1[0usize] * dir2[0usize] + dir1[1usize] * dir2[1usize] +
                   dir1[2usize] * dir2[2usize] < 0i32 as libc::c_float {
            dist = dist2;
            dir[0usize] = dir2[0usize];
            dir[1usize] = dir2[1usize];
            dir[2usize] = dir2[2usize]
        } else {
            dist = dist1;
            dir[0usize] = dir1[0usize];
            dir[1usize] = dir1[1usize];
            dir[2usize] = dir1[2usize]
        }
        BotCheckBlocked(ms, dir.as_mut_ptr(), qfalse as libc::c_int,
                        &mut result);
        if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
        speed =
            400i32 as libc::c_float -
                (400i32 as libc::c_float - 6i32 as libc::c_float * dist);
        if 0 == (*ms).moveflags & 4i32 &&
               0 ==
                   BotCheckBarrierJump(ms, dir.as_mut_ptr(),
                                       50i32 as libc::c_float) {
            EA_Move((*ms).client, dir.as_mut_ptr(), speed);
        }
        result.movedir[0usize] = dir[0usize];
        result.movedir[1usize] = dir[1usize];
        result.movedir[2usize] = dir[2usize];
        if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
    }
    return result;
}
//end of the function BotOnMover
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn MoverDown(mut reach: *mut aas_reachability_t)
 -> libc::c_int {
    let mut modelnum: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    modelnum = (*reach).facenum & 0xffffi32;
    AAS_BSPModelMinsMaxsOrigin(modelnum, angles.as_mut_ptr(),
                               mins.as_mut_ptr(), maxs.as_mut_ptr(),
                               origin.as_mut_ptr());
    if 0 == AAS_OriginOfMoverWithModelNum(modelnum, origin.as_mut_ptr()) {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"no entity with model %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            modelnum);
        return qfalse as libc::c_int
    }
    if origin[2usize] + maxs[2usize] < (*reach).start[2usize] {
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//end of the function BotTravel_Ladder
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Teleport(mut ms: *mut bot_movestate_t,
                                            mut reach:
                                                *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    if 0 != (*ms).moveflags & 32i32 { return result }
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = (*reach).start[2usize] - (*ms).origin[2usize];
    if 0 == (*ms).moveflags & 4i32 { hordir[2usize] = 0i32 as vec_t }
    dist = VectorNormalize(hordir.as_mut_ptr());
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    if dist < 30i32 as libc::c_float {
        EA_Move((*ms).client, hordir.as_mut_ptr(), 200i32 as libc::c_float);
    } else {
        EA_Move((*ms).client, hordir.as_mut_ptr(), 400i32 as libc::c_float);
    }
    if 0 != (*ms).moveflags & 4i32 { result.flags |= 2i32 }
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotTravel_Swim
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_WaterJump(mut ms: *mut bot_movestate_t,
                                             mut reach:
                                                 *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dir: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    dir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    dir[2usize] = (*reach).end[2usize] - (*ms).origin[2usize];
    hordir[0usize] = dir[0usize];
    hordir[1usize] = dir[1usize];
    hordir[2usize] = dir[2usize];
    hordir[2usize] = 0i32 as vec_t;
    dir[2usize] =
        (dir[2usize] as libc::c_double +
             (15i32 as libc::c_double +
                  2.0f64 *
                      (((rand() & 0x7fffi32) as libc::c_float /
                            0x7fffi32 as libc::c_float) as libc::c_double -
                           0.5f64) * 40i32 as libc::c_double)) as vec_t;
    VectorNormalize(dir.as_mut_ptr());
    dist = VectorNormalize(hordir.as_mut_ptr());
    EA_MoveForward((*ms).client);
    if dist < 40i32 as libc::c_float { EA_MoveUp((*ms).client); }
    vectoangles(dir.as_mut_ptr() as *const vec_t,
                result.ideal_viewangles.as_mut_ptr());
    result.flags |= 1i32;
    result.movedir[0usize] = dir[0usize];
    result.movedir[1usize] = dir[1usize];
    result.movedir[2usize] = dir[2usize];
    return result;
}
//end of the function BotFinishTravel_WalkOffLedge
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
/*
bot_moveresult_t BotTravel_Jump(bot_movestate_t *ms, aas_reachability_t *reach)
{
	vec3_t hordir;
	float dist, gapdist, speed, horspeed, sv_jumpvel;
	bot_moveresult_t_cleared( result );

	//
	sv_jumpvel = botlibglobals.sv_jumpvel->value;
	//walk straight to the reachability start
	hordir[0] = reach->start[0] - ms->origin[0];
	hordir[1] = reach->start[1] - ms->origin[1];
	hordir[2] = 0;
	dist = VectorNormalize(hordir);
	//
	speed = 350;
	//
	gapdist = BotGapDistance(ms, hordir, ms->entitynum);
	//if pretty close to the start focus on the reachability end
	if (dist < 50 || (gapdist && gapdist < 50))
	{
		//NOTE: using max speed (400) works best
		//if (AAS_HorizontalVelocityForJump(sv_jumpvel, ms->origin, reach->end, &horspeed))
		//{
		//	speed = horspeed * 400 / botlibglobals.sv_maxwalkvelocity->value;
		//} //end if
		hordir[0] = reach->end[0] - ms->origin[0];
		hordir[1] = reach->end[1] - ms->origin[1];
		VectorNormalize(hordir);
		//elemantary action jump
		EA_Jump(ms->client);
		//
		ms->jumpreach = ms->lastreachnum;
		speed = 600;
	} //end if
	else
	{
		if (AAS_HorizontalVelocityForJump(sv_jumpvel, reach->start, reach->end, &horspeed))
		{
			speed = horspeed * 400 / botlibglobals.sv_maxwalkvelocity->value;
		} //end if
	} //end else
	//elemantary action
	EA_Move(ms->client, hordir, speed);
	VectorCopy(hordir, result.movedir);
	//
	return result;
} //end of the function BotTravel_Jump*/
/*
bot_moveresult_t BotTravel_Jump(bot_movestate_t *ms, aas_reachability_t *reach)
{
	vec3_t hordir, dir1, dir2, mins, maxs, start, end;
	int gapdist;
	float dist1, dist2, speed;
	bot_moveresult_t_cleared( result );
	bsp_trace_t trace;

	//
	hordir[0] = reach->start[0] - reach->end[0];
	hordir[1] = reach->start[1] - reach->end[1];
	hordir[2] = 0;
	VectorNormalize(hordir);
	//
	VectorCopy(reach->start, start);
	start[2] += 1;
	//minus back the bouding box size plus 16
	VectorMA(reach->start, 80, hordir, end);
	//
	AAS_PresenceTypeBoundingBox(PRESENCE_NORMAL, mins, maxs);
	//check for solids
	trace = AAS_Trace(start, mins, maxs, end, ms->entitynum, MASK_PLAYERSOLID);
	if (trace.startsolid) VectorCopy(start, trace.endpos);
	//check for a gap
	for (gapdist = 0; gapdist < 80; gapdist += 10)
	{
		VectorMA(start, gapdist+10, hordir, end);
		end[2] += 1;
		if (AAS_PointAreaNum(end) != ms->reachareanum) break;
	} //end for
	if (gapdist < 80) VectorMA(reach->start, gapdist, hordir, trace.endpos);
//	dist1 = BotGapDistance(start, hordir, ms->entitynum);
//	if (dist1 && dist1 <= trace.fraction * 80) VectorMA(reach->start, dist1-20, hordir, trace.endpos);
	//
	VectorSubtract(ms->origin, reach->start, dir1);
	dir1[2] = 0;
	dist1 = VectorNormalize(dir1);
	VectorSubtract(ms->origin, trace.endpos, dir2);
	dir2[2] = 0;
	dist2 = VectorNormalize(dir2);
	//if just before the reachability start
	if (DotProduct(dir1, dir2) < -0.8 || dist2 < 5)
	{
		//botimport.Print(PRT_MESSAGE, "between jump start and run to point\n");
		hordir[0] = reach->end[0] - ms->origin[0];
		hordir[1] = reach->end[1] - ms->origin[1];
		hordir[2] = 0;
		VectorNormalize(hordir);
		//elemantary action jump
		if (dist1 < 24) EA_Jump(ms->client);
		else if (dist1 < 32) EA_DelayedJump(ms->client);
		EA_Move(ms->client, hordir, 600);
		//
		ms->jumpreach = ms->lastreachnum;
	} //end if
	else
	{
		//botimport.Print(PRT_MESSAGE, "going towards run to point\n");
		hordir[0] = trace.endpos[0] - ms->origin[0];
		hordir[1] = trace.endpos[1] - ms->origin[1];
		hordir[2] = 0;
		VectorNormalize(hordir);
		//
		if (dist2 > 80) dist2 = 80;
		speed = 400 - (400 - 5 * dist2);
		EA_Move(ms->client, hordir, speed);
	} //end else
	VectorCopy(hordir, result.movedir);
	//
	return result;
} //end of the function BotTravel_Jump*/
//*
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Jump(mut ms: *mut bot_movestate_t,
                                        mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut dir1: vec3_t = [0.; 3];
    let mut dir2: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut runstart: vec3_t = [0.; 3];
    //	vec3_t runstart, dir1, dir2, hordir;
    let mut gapdist: libc::c_int = 0;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    AAS_JumpReachRunStart(reach, runstart.as_mut_ptr());
    hordir[0usize] = runstart[0usize] - (*reach).start[0usize];
    hordir[1usize] = runstart[1usize] - (*reach).start[1usize];
    hordir[2usize] = 0i32 as vec_t;
    VectorNormalize(hordir.as_mut_ptr());
    start[0usize] = (*reach).start[0usize];
    start[1usize] = (*reach).start[1usize];
    start[2usize] = (*reach).start[2usize];
    start[2usize] += 1i32 as libc::c_float;
    runstart[0usize] =
        (*reach).start[0usize] + hordir[0usize] * 80i32 as libc::c_float;
    runstart[1usize] =
        (*reach).start[1usize] + hordir[1usize] * 80i32 as libc::c_float;
    runstart[2usize] =
        (*reach).start[2usize] + hordir[2usize] * 80i32 as libc::c_float;
    gapdist = 0i32;
    while gapdist < 80i32 {
        end[0usize] =
            start[0usize] +
                hordir[0usize] * (gapdist + 10i32) as libc::c_float;
        end[1usize] =
            start[1usize] +
                hordir[1usize] * (gapdist + 10i32) as libc::c_float;
        end[2usize] =
            start[2usize] +
                hordir[2usize] * (gapdist + 10i32) as libc::c_float;
        end[2usize] += 1i32 as libc::c_float;
        if AAS_PointAreaNum(end.as_mut_ptr()) != (*ms).reachareanum {
            break ;
        }
        gapdist += 10i32
    }
    if gapdist < 80i32 {
        runstart[0usize] =
            (*reach).start[0usize] +
                hordir[0usize] * gapdist as libc::c_float;
        runstart[1usize] =
            (*reach).start[1usize] +
                hordir[1usize] * gapdist as libc::c_float;
        runstart[2usize] =
            (*reach).start[2usize] + hordir[2usize] * gapdist as libc::c_float
    }
    dir1[0usize] = (*ms).origin[0usize] - (*reach).start[0usize];
    dir1[1usize] = (*ms).origin[1usize] - (*reach).start[1usize];
    dir1[2usize] = (*ms).origin[2usize] - (*reach).start[2usize];
    dir1[2usize] = 0i32 as vec_t;
    dist1 = VectorNormalize(dir1.as_mut_ptr());
    dir2[0usize] = (*ms).origin[0usize] - runstart[0usize];
    dir2[1usize] = (*ms).origin[1usize] - runstart[1usize];
    dir2[2usize] = (*ms).origin[2usize] - runstart[2usize];
    dir2[2usize] = 0i32 as vec_t;
    dist2 = VectorNormalize(dir2.as_mut_ptr());
    if ((dir1[0usize] * dir2[0usize] + dir1[1usize] * dir2[1usize] +
             dir1[2usize] * dir2[2usize]) as libc::c_double) < -0.8f64 ||
           dist2 < 5i32 as libc::c_float {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        if dist1 < 24i32 as libc::c_float {
            EA_Jump((*ms).client);
        } else if dist1 < 32i32 as libc::c_float {
            EA_DelayedJump((*ms).client);
        }
        EA_Move((*ms).client, hordir.as_mut_ptr(), 600i32 as libc::c_float);
        (*ms).jumpreach = (*ms).lastreachnum
    } else {
        hordir[0usize] = runstart[0usize] - (*ms).origin[0usize];
        hordir[1usize] = runstart[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        if dist2 > 80i32 as libc::c_float { dist2 = 80i32 as libc::c_float }
        speed =
            400i32 as libc::c_float -
                (400i32 as libc::c_float - 5i32 as libc::c_float * dist2);
        EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotFinishTravel_WaterJump
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_WalkOffLedge(mut ms: *mut bot_movestate_t,
                                                mut reach:
                                                    *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut reachhordist: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    dir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    dir[2usize] = (*reach).start[2usize] - (*ms).origin[2usize];
    VectorNormalize(dir.as_mut_ptr());
    BotCheckBlocked(ms, dir.as_mut_ptr(), qtrue as libc::c_int, &mut result);
    dir[0usize] = (*reach).end[0usize] - (*reach).start[0usize];
    dir[1usize] = (*reach).end[1usize] - (*reach).start[1usize];
    dir[2usize] = (*reach).end[2usize] - (*reach).start[2usize];
    dir[2usize] = 0i32 as vec_t;
    reachhordist = VectorLength(dir.as_mut_ptr() as *const vec_t);
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    if dist < 48i32 as libc::c_float {
        hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
        hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        if reachhordist < 20i32 as libc::c_float {
            speed = 100i32 as libc::c_float
        } else if 0 ==
                      AAS_HorizontalVelocityForJump(0i32 as libc::c_float,
                                                    (*reach).start.as_mut_ptr(),
                                                    (*reach).end.as_mut_ptr(),
                                                    &mut speed) {
            speed = 400i32 as libc::c_float
        }
    } else if reachhordist < 20i32 as libc::c_float {
        if dist > 64i32 as libc::c_float { dist = 64i32 as libc::c_float }
        speed =
            400i32 as libc::c_float -
                (256i32 as libc::c_float - 4i32 as libc::c_float * dist)
    } else { speed = 400i32 as libc::c_float }
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotTravel_Crouch
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_BarrierJump(mut ms: *mut bot_movestate_t,
                                               mut reach:
                                                   *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut hordir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    hordir[0usize] = (*reach).start[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).start[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    if dist < 9i32 as libc::c_float {
        EA_Jump((*ms).client);
    } else {
        if dist > 60i32 as libc::c_float { dist = 60i32 as libc::c_float }
        speed =
            360i32 as libc::c_float -
                (360i32 as libc::c_float - 6i32 as libc::c_float * dist);
        EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    }
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotFinishTravel_Walk
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotTravel_Crouch(mut ms: *mut bot_movestate_t,
                                          mut reach: *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut speed: libc::c_float = 0.;
    let mut hordir: vec3_t = [0.; 3];
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    speed = 400i32 as libc::c_float;
    hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    VectorNormalize(hordir.as_mut_ptr());
    BotCheckBlocked(ms, hordir.as_mut_ptr(), qtrue as libc::c_int,
                    &mut result);
    EA_Crouch((*ms).client);
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}
//end of the function BotValidTravel
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotAddToAvoidReach(mut ms: *mut bot_movestate_t,
                                            mut number: libc::c_int,
                                            mut avoidtime: libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 1i32 {
        if (*ms).avoidreach[i as usize] == number {
            if (*ms).avoidreachtimes[i as usize] > AAS_Time() {
                (*ms).avoidreachtries[i as usize] += 1
            } else { (*ms).avoidreachtries[i as usize] = 1i32 }
            (*ms).avoidreachtimes[i as usize] = AAS_Time() + avoidtime;
            return
        }
        i += 1
    }
    i = 0i32;
    while i < 1i32 {
        if (*ms).avoidreachtimes[i as usize] < AAS_Time() {
            (*ms).avoidreach[i as usize] = number;
            (*ms).avoidreachtimes[i as usize] = AAS_Time() + avoidtime;
            (*ms).avoidreachtries[i as usize] = 1i32;
            return
        }
        i += 1
    };
}
//end of the function BotFinishTravel_JumpPad
//===========================================================================
// time before the reachability times out
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotReachabilityTime(mut reach:
                                                 *mut aas_reachability_t)
 -> libc::c_int {
    match (*reach).traveltype & 0xffffffi32 {
        2 => { return 5i32 }
        3 => { return 5i32 }
        4 => { return 5i32 }
        6 => { return 6i32 }
        7 => { return 5i32 }
        5 => { return 5i32 }
        8 => { return 5i32 }
        9 => { return 5i32 }
        10 => { return 5i32 }
        11 => { return 10i32 }
        14 => { return 8i32 }
        12 => { return 6i32 }
        13 => { return 6i32 }
        18 => { return 10i32 }
        19 => { return 10i32 }
        _ => {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"travel type %d not implemented yet\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*reach).traveltype);
            return 8i32
        }
    };
}
//end case
//end switch
//end of the function BotReachabilityTime
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotMoveInGoalArea(mut ms: *mut bot_movestate_t,
                                           mut goal: *mut bot_goal_t)
 -> bot_moveresult_t {
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    let mut dir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    dir[0usize] = (*goal).origin[0usize] - (*ms).origin[0usize];
    dir[1usize] = (*goal).origin[1usize] - (*ms).origin[1usize];
    if 0 != (*ms).moveflags & 4i32 {
        dir[2usize] = (*goal).origin[2usize] - (*ms).origin[2usize];
        result.traveltype = 8i32
    } else { dir[2usize] = 0i32 as vec_t; result.traveltype = 2i32 }
    dist = VectorNormalize(dir.as_mut_ptr());
    if dist > 100i32 as libc::c_float { dist = 100i32 as libc::c_float }
    speed =
        400i32 as libc::c_float -
            (400i32 as libc::c_float - 4i32 as libc::c_float * dist);
    if speed < 10i32 as libc::c_float { speed = 0i32 as libc::c_float }
    BotCheckBlocked(ms, dir.as_mut_ptr(), qtrue as libc::c_int, &mut result);
    EA_Move((*ms).client, dir.as_mut_ptr(), speed);
    result.movedir[0usize] = dir[0usize];
    result.movedir[1usize] = dir[1usize];
    result.movedir[2usize] = dir[2usize];
    if 0 != (*ms).moveflags & 4i32 {
        vectoangles(dir.as_mut_ptr() as *const vec_t,
                    result.ideal_viewangles.as_mut_ptr());
        result.flags |= 2i32
    }
    (*ms).lastreachnum = 0i32;
    (*ms).lastareanum = 0i32;
    (*ms).lastgoalareanum = (*goal).areanum;
    (*ms).lastorigin[0usize] = (*ms).origin[0usize];
    (*ms).lastorigin[1usize] = (*ms).origin[1usize];
    (*ms).lastorigin[2usize] = (*ms).origin[2usize];
    return result;
}
//end of the function AngleDiff
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFuzzyPointReachabilityArea(mut origin: *mut vec_t)
 -> libc::c_int {
    let mut firstareanum: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut bestareanum: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut points: [vec3_t; 10] = [[0.; 3]; 10];
    let mut v: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    firstareanum = 0i32;
    areanum = AAS_PointAreaNum(origin);
    if 0 != areanum {
        firstareanum = areanum;
        if 0 != AAS_AreaReachability(areanum) { return areanum }
    }
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    end[2usize] += 4i32 as libc::c_float;
    numareas =
        AAS_TraceAreas(origin, end.as_mut_ptr(), areas.as_mut_ptr(),
                       points.as_mut_ptr(), 10i32);
    j = 0i32;
    while j < numareas {
        if 0 != AAS_AreaReachability(areas[j as usize]) {
            return areas[j as usize]
        }
        j += 1
    }
    bestdist = 999999i32 as libc::c_float;
    bestareanum = 0i32;
    z = 1i32;
    while z >= -1i32 {
        x = 1i32;
        while x >= -1i32 {
            y = 1i32;
            while y >= -1i32 {
                end[0usize] = *origin.offset(0isize);
                end[1usize] = *origin.offset(1isize);
                end[2usize] = *origin.offset(2isize);
                end[0usize] += (x * 8i32) as libc::c_float;
                end[1usize] += (y * 8i32) as libc::c_float;
                end[2usize] += (z * 12i32) as libc::c_float;
                numareas =
                    AAS_TraceAreas(origin, end.as_mut_ptr(),
                                   areas.as_mut_ptr(), points.as_mut_ptr(),
                                   10i32);
                j = 0i32;
                while j < numareas {
                    if 0 != AAS_AreaReachability(areas[j as usize]) {
                        v[0usize] =
                            points[j as usize][0usize] -
                                *origin.offset(0isize);
                        v[1usize] =
                            points[j as usize][1usize] -
                                *origin.offset(1isize);
                        v[2usize] =
                            points[j as usize][2usize] -
                                *origin.offset(2isize);
                        dist = VectorLength(v.as_mut_ptr() as *const vec_t);
                        if dist < bestdist {
                            bestareanum = areas[j as usize];
                            bestdist = dist
                        }
                    }
                    if 0 == firstareanum { firstareanum = areas[j as usize] }
                    j += 1
                }
                y -= 1i32
            }
            x -= 1i32
        }
        if 0 != bestareanum { return bestareanum }
        z -= 1i32
    }
    return firstareanum;
}
//type of model, func_plat or func_bobbing
#[no_mangle]
pub static mut modeltypes: [libc::c_int; 256] = [0; 256];
//end for
//end of the function BotSetBrushModelTypes
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotOnTopOfEntity(mut ms: *mut bot_movestate_t)
 -> libc::c_int {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
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
    AAS_PresenceTypeBoundingBox((*ms).presencetype, mins.as_mut_ptr(),
                                maxs.as_mut_ptr());
    end[0usize] = (*ms).origin[0usize] + up[0usize] * -3i32 as libc::c_float;
    end[1usize] = (*ms).origin[1usize] + up[1usize] * -3i32 as libc::c_float;
    end[2usize] = (*ms).origin[2usize] + up[2usize] * -3i32 as libc::c_float;
    trace =
        AAS_Trace((*ms).origin.as_mut_ptr(), mins.as_mut_ptr(),
                  maxs.as_mut_ptr(), end.as_mut_ptr(), (*ms).entitynum,
                  1i32 | 0x10000i32);
    if 0 == trace.startsolid as u64 &&
           (trace.ent != (1i32 << 10i32) - 2i32 &&
                trace.ent != (1i32 << 10i32) - 1i32) {
        return trace.ent
    }
    return -1i32;
}
//end of the function GrappleState
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotResetGrapple(mut ms: *mut bot_movestate_t) {
    let mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    AAS_ReachabilityFromNum((*ms).lastreachnum, &mut reach);
    if reach.traveltype & 0xffffffi32 != 14i32 {
        if 0 != (*ms).moveflags & 128i32 || 0. != (*ms).grapplevisible_time {
            if 0. != (*offhandgrapple).value {
                EA_Command((*ms).client, (*cmd_grappleoff).string);
            }
            (*ms).moveflags &= !128i32;
            (*ms).grapplevisible_time = 0i32 as libc::c_float
        }
    };
}
//moves the bot in the specified direction using the specified type of movement
#[no_mangle]
pub unsafe extern "C" fn BotMoveInDirection(mut movestate: libc::c_int,
                                            mut dir: *mut vec_t,
                                            mut speed: libc::c_float,
                                            mut type_0: libc::c_int)
 -> libc::c_int {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return qfalse as libc::c_int }
    if 0 != AAS_Swimming((*ms).origin.as_mut_ptr()) {
        return BotSwimInDirection(ms, dir, speed, type_0)
    } else { return BotWalkInDirection(ms, dir, speed, type_0) };
}
//end of the function BotSwimInDirection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotWalkInDirection(mut ms: *mut bot_movestate_t,
                                            mut dir: *mut vec_t,
                                            mut speed: libc::c_float,
                                            mut type_0: libc::c_int)
 -> libc::c_int {
    let mut hordir: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    let mut tmpdir: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut presencetype: libc::c_int = 0;
    let mut maxframes: libc::c_int = 0;
    let mut cmdframes: libc::c_int = 0;
    let mut stopevent: libc::c_int = 0;
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
    let mut dist: libc::c_float = 0.;
    if 0 !=
           AAS_OnGround((*ms).origin.as_mut_ptr(), (*ms).presencetype,
                        (*ms).entitynum) {
        (*ms).moveflags |= 2i32
    }
    if 0 != (*ms).moveflags & 2i32 {
        if 0 != BotCheckBarrierJump(ms, dir, speed) {
            return qtrue as libc::c_int
        }
        (*ms).moveflags &= !1i32;
        if 0 != type_0 & 2i32 && 0 == type_0 & 4i32 {
            presencetype = 4i32
        } else { presencetype = 2i32 }
        hordir[0usize] = *dir.offset(0isize);
        hordir[1usize] = *dir.offset(1isize);
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        if 0 == type_0 & 4i32 {
            if BotGapDistance((*ms).origin.as_mut_ptr(), hordir.as_mut_ptr(),
                              (*ms).entitynum) > 0i32 as libc::c_float {
                type_0 |= 4i32
            }
        }
        cmdmove[0usize] = hordir[0usize] * speed;
        cmdmove[1usize] = hordir[1usize] * speed;
        cmdmove[2usize] = hordir[2usize] * speed;
        velocity[0usize] = (*ms).velocity[0usize];
        velocity[1usize] = (*ms).velocity[1usize];
        velocity[2usize] = (*ms).velocity[2usize];
        if 0 != type_0 & 4i32 {
            cmdmove[2usize] = 400i32 as vec_t;
            maxframes = (3i32 as libc::c_double / 0.1f64) as libc::c_int;
            cmdframes = 1i32;
            stopevent = 1i32 | 32i32 | 4i32 | 8i32 | 16i32
        } else {
            maxframes = 2i32;
            cmdframes = 2i32;
            stopevent = 32i32 | 4i32 | 8i32 | 16i32
        }
        origin[0usize] = (*ms).origin[0usize];
        origin[1usize] = (*ms).origin[1usize];
        origin[2usize] = (*ms).origin[2usize];
        origin[2usize] = (origin[2usize] as libc::c_double + 0.5f64) as vec_t;
        AAS_PredictClientMovement(&mut move_0, (*ms).entitynum,
                                  origin.as_mut_ptr(), presencetype,
                                  qtrue as libc::c_int, velocity.as_mut_ptr(),
                                  cmdmove.as_mut_ptr(), cmdframes, maxframes,
                                  0.1f32, stopevent, 0i32,
                                  qfalse as libc::c_int);
        if move_0.frames >= maxframes && 0 != type_0 & 4i32 {
            return qfalse as libc::c_int
        }
        if 0 != move_0.stopevent & (8i32 | 16i32 | 32i32) {
            return qfalse as libc::c_int
        }
        if 0 != move_0.stopevent & 1i32 {
            VectorNormalize2(move_0.velocity.as_mut_ptr() as *const vec_t,
                             tmpdir.as_mut_ptr());
            dist =
                BotGapDistance(move_0.endpos.as_mut_ptr(),
                               tmpdir.as_mut_ptr(), (*ms).entitynum);
            if dist > 0i32 as libc::c_float { return qfalse as libc::c_int }
            dist =
                BotGapDistance(move_0.endpos.as_mut_ptr(),
                               hordir.as_mut_ptr(), (*ms).entitynum);
            if dist > 0i32 as libc::c_float { return qfalse as libc::c_int }
        }
        tmpdir[0usize] = move_0.endpos[0usize] - (*ms).origin[0usize];
        tmpdir[1usize] = move_0.endpos[1usize] - (*ms).origin[1usize];
        tmpdir[2usize] = 0i32 as vec_t;
        if (VectorLength(tmpdir.as_mut_ptr() as *const vec_t) as
                libc::c_double) <
               (speed * (*ms).thinktime) as libc::c_double * 0.5f64 {
            return qfalse as libc::c_int
        }
        if 0 != type_0 & 4i32 { EA_Jump((*ms).client); }
        if 0 != type_0 & 2i32 { EA_Crouch((*ms).client); }
        EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
        return qtrue as libc::c_int
    } else {
        if 0 != (*ms).moveflags & 1i32 {
            if (*ms).velocity[2usize] < 50i32 as libc::c_float {
                EA_Move((*ms).client, dir, speed);
            }
        }
        return qtrue as libc::c_int
    };
}
//end of the function BotCheckBarrierJump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotSwimInDirection(mut ms: *mut bot_movestate_t,
                                            mut dir: *mut vec_t,
                                            mut speed: libc::c_float,
                                            mut type_0: libc::c_int)
 -> libc::c_int {
    let mut normdir: vec3_t = [0.; 3];
    normdir[0usize] = *dir.offset(0isize);
    normdir[1usize] = *dir.offset(1isize);
    normdir[2usize] = *dir.offset(2isize);
    VectorNormalize(normdir.as_mut_ptr());
    EA_Move((*ms).client, normdir.as_mut_ptr(), speed);
    return qtrue as libc::c_int;
}
//reset avoid reachability
#[no_mangle]
pub unsafe extern "C" fn BotResetAvoidReach(mut movestate: libc::c_int) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return }
    memset((*ms).avoidreach.as_mut_ptr() as *mut libc::c_void, 0i32,
           (1i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    memset((*ms).avoidreachtimes.as_mut_ptr() as *mut libc::c_void, 0i32,
           (1i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                as libc::c_ulong));
    memset((*ms).avoidreachtries.as_mut_ptr() as *mut libc::c_void, 0i32,
           (1i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
}
//resets the last avoid reachability
#[no_mangle]
pub unsafe extern "C" fn BotResetLastAvoidReach(mut movestate: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut latest: libc::c_int = 0;
    let mut latesttime: libc::c_float = 0.;
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return }
    latesttime = 0i32 as libc::c_float;
    latest = 0i32;
    i = 0i32;
    while i < 1i32 {
        if (*ms).avoidreachtimes[i as usize] > latesttime {
            latesttime = (*ms).avoidreachtimes[i as usize];
            latest = i
        }
        i += 1
    }
    if 0. != latesttime {
        (*ms).avoidreachtimes[latest as usize] = 0i32 as libc::c_float;
        if (*ms).avoidreachtries[latest as usize] > 0i32 {
            (*ms).avoidreachtries[latest as usize] -= 1
        }
    };
}
//returns a reachability area if the origin is in one
#[no_mangle]
pub unsafe extern "C" fn BotReachabilityArea(mut origin: *mut vec_t,
                                             mut client: libc::c_int)
 -> libc::c_int {
    let mut modelnum: libc::c_int = 0;
    let mut modeltype: libc::c_int = 0;
    let mut reachnum: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    let mut org: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
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
    AAS_PresenceTypeBoundingBox(4i32, mins.as_mut_ptr(), maxs.as_mut_ptr());
    end[0usize] =
        *origin.offset(0isize) + up[0usize] * -3i32 as libc::c_float;
    end[1usize] =
        *origin.offset(1isize) + up[1usize] * -3i32 as libc::c_float;
    end[2usize] =
        *origin.offset(2isize) + up[2usize] * -3i32 as libc::c_float;
    bsptrace =
        AAS_Trace(origin, mins.as_mut_ptr(), maxs.as_mut_ptr(),
                  end.as_mut_ptr(), client, 1i32 | 0x10000i32);
    if 0 == bsptrace.startsolid as u64 &&
           bsptrace.fraction < 1i32 as libc::c_float &&
           bsptrace.ent != (1i32 << 10i32) - 1i32 {
        if bsptrace.ent == (1i32 << 10i32) - 2i32 {
            return BotFuzzyPointReachabilityArea(origin)
        }
        modelnum = AAS_EntityModelindex(bsptrace.ent);
        modeltype = modeltypes[modelnum as usize];
        if modeltype == 1i32 || modeltype == 2i32 {
            reachnum = AAS_NextModelReachability(0i32, modelnum);
            if 0 != reachnum {
                AAS_ReachabilityFromNum(reachnum, &mut reach);
                return reach.areanum
            }
        }
        if 0 != AAS_Swimming(origin) {
            return BotFuzzyPointReachabilityArea(origin)
        }
        areanum = BotFuzzyPointReachabilityArea(origin);
        if 0 != areanum && 0 != AAS_AreaReachability(areanum) {
            return areanum
        }
        org[0usize] = *origin.offset(0isize);
        org[1usize] = *origin.offset(1isize);
        org[2usize] = *origin.offset(2isize);
        end[0usize] = org[0usize];
        end[1usize] = org[1usize];
        end[2usize] = org[2usize];
        end[2usize] -= 800i32 as libc::c_float;
        trace =
            AAS_TraceClientBBox(org.as_mut_ptr(), end.as_mut_ptr(), 4i32,
                                -1i32);
        if 0 == trace.startsolid as u64 {
            org[0usize] = trace.endpos[0usize];
            org[1usize] = trace.endpos[1usize];
            org[2usize] = trace.endpos[2usize]
        }
        return BotFuzzyPointReachabilityArea(org.as_mut_ptr())
    }
    return BotFuzzyPointReachabilityArea(origin);
}
//view target based on movement
#[no_mangle]
pub unsafe extern "C" fn BotMovementViewTarget(mut movestate: libc::c_int,
                                               mut goal: *mut bot_goal_t,
                                               mut travelflags: libc::c_int,
                                               mut lookahead: libc::c_float,
                                               mut target: *mut vec_t)
 -> libc::c_int {
    let mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    let mut reachnum: libc::c_int = 0;
    let mut lastareanum: libc::c_int = 0;
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    let mut end: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return qfalse as libc::c_int }
    if 0 == (*ms).lastreachnum || goal.is_null() {
        return qfalse as libc::c_int
    }
    reachnum = (*ms).lastreachnum;
    end[0usize] = (*ms).origin[0usize];
    end[1usize] = (*ms).origin[1usize];
    end[2usize] = (*ms).origin[2usize];
    lastareanum = (*ms).lastareanum;
    dist = 0i32 as libc::c_float;
    while 0 != reachnum && dist < lookahead {
        AAS_ReachabilityFromNum(reachnum, &mut reach);
        if 0 !=
               BotAddToTarget(end.as_mut_ptr(), reach.start.as_mut_ptr(),
                              lookahead, &mut dist, target) {
            return qtrue as libc::c_int
        }
        if reach.traveltype & 0xffffffi32 == 10i32 {
            return qtrue as libc::c_int
        }
        if reach.traveltype & 0xffffffi32 == 12i32 {
            return qtrue as libc::c_int
        }
        if reach.traveltype & 0xffffffi32 == 13i32 {
            return qtrue as libc::c_int
        }
        if reach.traveltype & 0xffffffi32 != 18i32 &&
               reach.traveltype & 0xffffffi32 != 11i32 &&
               reach.traveltype & 0xffffffi32 != 19i32 {
            if 0 !=
                   BotAddToTarget(reach.start.as_mut_ptr(),
                                  reach.end.as_mut_ptr(), lookahead,
                                  &mut dist, target) {
                return qtrue as libc::c_int
            }
        }
        reachnum =
            BotGetReachabilityToGoal(reach.end.as_mut_ptr(), reach.areanum,
                                     (*ms).lastgoalareanum, lastareanum,
                                     (*ms).avoidreach.as_mut_ptr(),
                                     (*ms).avoidreachtimes.as_mut_ptr(),
                                     (*ms).avoidreachtries.as_mut_ptr(), goal,
                                     travelflags, 0 as *mut bot_avoidspot_s,
                                     0i32, 0 as *mut libc::c_int);
        end[0usize] = reach.end[0usize];
        end[1usize] = reach.end[1usize];
        end[2usize] = reach.end[2usize];
        lastareanum = reach.areanum;
        if lastareanum == (*goal).areanum {
            BotAddToTarget(reach.end.as_mut_ptr(),
                           (*goal).origin.as_mut_ptr(), lookahead, &mut dist,
                           target);
            return qtrue as libc::c_int
        }
    }
    return qfalse as libc::c_int;
}
//end of the function BotGetReachabilityToGoal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotAddToTarget(mut start: *mut vec_t,
                                        mut end: *mut vec_t,
                                        mut maxdist: libc::c_float,
                                        mut dist: *mut libc::c_float,
                                        mut target: *mut vec_t)
 -> libc::c_int {
    let mut dir: vec3_t = [0.; 3];
    let mut curdist: libc::c_float = 0.;
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    curdist = VectorNormalize(dir.as_mut_ptr());
    if *dist + curdist < maxdist {
        *target.offset(0isize) = *end.offset(0isize);
        *target.offset(1isize) = *end.offset(1isize);
        *target.offset(2isize) = *end.offset(2isize);
        *dist += curdist;
        return qfalse as libc::c_int
    } else {
        *target.offset(0isize) =
            *start.offset(0isize) + dir[0usize] * (maxdist - *dist);
        *target.offset(1isize) =
            *start.offset(1isize) + dir[1usize] * (maxdist - *dist);
        *target.offset(2isize) =
            *start.offset(2isize) + dir[2usize] * (maxdist - *dist);
        *dist = maxdist;
        return qtrue as libc::c_int
    };
}
//predict the position of a player based on movement towards a goal
#[no_mangle]
pub unsafe extern "C" fn BotPredictVisiblePosition(mut origin: *mut vec_t,
                                                   mut areanum: libc::c_int,
                                                   mut goal: *mut bot_goal_t,
                                                   mut travelflags:
                                                       libc::c_int,
                                                   mut target: *mut vec_t)
 -> libc::c_int {
    let mut reach: aas_reachability_t =
        aas_reachability_s{areanum: 0,
                           facenum: 0,
                           edgenum: 0,
                           start: [0.; 3],
                           end: [0.; 3],
                           traveltype: 0,
                           traveltime: 0,};
    let mut reachnum: libc::c_int = 0;
    let mut lastgoalareanum: libc::c_int = 0;
    let mut lastareanum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut avoidreach: [libc::c_int; 1] = [0; 1];
    let mut avoidreachtimes: [libc::c_float; 1] = [0.; 1];
    let mut avoidreachtries: [libc::c_int; 1] = [0; 1];
    let mut end: vec3_t = [0.; 3];
    if goal.is_null() { return qfalse as libc::c_int }
    if 0 == areanum { return qfalse as libc::c_int }
    if 0 == (*goal).areanum { return qfalse as libc::c_int }
    memset(avoidreach.as_mut_ptr() as *mut libc::c_void, 0i32,
           (1i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    lastgoalareanum = (*goal).areanum;
    lastareanum = areanum;
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    i = 0i32;
    while i < 20i32 && areanum != (*goal).areanum {
        reachnum =
            BotGetReachabilityToGoal(end.as_mut_ptr(), areanum,
                                     lastgoalareanum, lastareanum,
                                     avoidreach.as_mut_ptr(),
                                     avoidreachtimes.as_mut_ptr(),
                                     avoidreachtries.as_mut_ptr(), goal,
                                     travelflags, 0 as *mut bot_avoidspot_s,
                                     0i32, 0 as *mut libc::c_int);
        if 0 == reachnum { return qfalse as libc::c_int }
        AAS_ReachabilityFromNum(reachnum, &mut reach);
        if 0 !=
               BotVisible((*goal).entitynum, (*goal).origin.as_mut_ptr(),
                          reach.start.as_mut_ptr()) {
            *target.offset(0isize) = reach.start[0usize];
            *target.offset(1isize) = reach.start[1usize];
            *target.offset(2isize) = reach.start[2usize];
            return qtrue as libc::c_int
        }
        if 0 !=
               BotVisible((*goal).entitynum, (*goal).origin.as_mut_ptr(),
                          reach.end.as_mut_ptr()) {
            *target.offset(0isize) = reach.end[0usize];
            *target.offset(1isize) = reach.end[1usize];
            *target.offset(2isize) = reach.end[2usize];
            return qtrue as libc::c_int
        }
        if reach.areanum == (*goal).areanum {
            *target.offset(0isize) = reach.end[0usize];
            *target.offset(1isize) = reach.end[1usize];
            *target.offset(2isize) = reach.end[2usize];
            return qtrue as libc::c_int
        }
        lastareanum = areanum;
        areanum = reach.areanum;
        end[0usize] = reach.end[0usize];
        end[1usize] = reach.end[1usize];
        end[2usize] = reach.end[2usize];
        i += 1
    }
    return qfalse as libc::c_int;
}
//end of the function BotMovementViewTarget
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotVisible(mut ent: libc::c_int, mut eye: *mut vec_t,
                                    mut target: *mut vec_t) -> libc::c_int {
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
    trace =
        AAS_Trace(eye, 0 as *mut vec_t, 0 as *mut vec_t, target, ent,
                  1i32 | 0x10000i32);
    if trace.fraction >= 1i32 as libc::c_float { return qtrue as libc::c_int }
    return qfalse as libc::c_int;
}
//returns the handle of a newly allocated movestate
#[no_mangle]
pub unsafe extern "C" fn BotAllocMoveState() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i <= 64i32 {
        if botmovestates[i as usize].is_null() {
            botmovestates[i as usize] =
                GetClearedMemory(::std::mem::size_of::<bot_movestate_t>() as
                                     libc::c_ulong) as *mut bot_movestate_t;
            return i
        }
        i += 1
    }
    return 0i32;
}
//frees the movestate with the given handle
#[no_mangle]
pub unsafe extern "C" fn BotFreeMoveState(mut handle: libc::c_int) {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"move state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    if botmovestates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid move state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    FreeMemory(botmovestates[handle as usize] as *mut libc::c_void);
    botmovestates[handle as usize] = 0 as *mut bot_movestate_t;
}
//initialize movement state before performing any movement
#[no_mangle]
pub unsafe extern "C" fn BotInitMoveState(mut handle: libc::c_int,
                                          mut initmove: *mut bot_initmove_t) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(handle);
    if ms.is_null() { return }
    (*ms).origin[0usize] = (*initmove).origin[0usize];
    (*ms).origin[1usize] = (*initmove).origin[1usize];
    (*ms).origin[2usize] = (*initmove).origin[2usize];
    (*ms).velocity[0usize] = (*initmove).velocity[0usize];
    (*ms).velocity[1usize] = (*initmove).velocity[1usize];
    (*ms).velocity[2usize] = (*initmove).velocity[2usize];
    (*ms).viewoffset[0usize] = (*initmove).viewoffset[0usize];
    (*ms).viewoffset[1usize] = (*initmove).viewoffset[1usize];
    (*ms).viewoffset[2usize] = (*initmove).viewoffset[2usize];
    (*ms).entitynum = (*initmove).entitynum;
    (*ms).client = (*initmove).client;
    (*ms).thinktime = (*initmove).thinktime;
    (*ms).presencetype = (*initmove).presencetype;
    (*ms).viewangles[0usize] = (*initmove).viewangles[0usize];
    (*ms).viewangles[1usize] = (*initmove).viewangles[1usize];
    (*ms).viewangles[2usize] = (*initmove).viewangles[2usize];
    (*ms).moveflags &= !2i32;
    if 0 != (*initmove).or_moveflags & 2i32 { (*ms).moveflags |= 2i32 }
    (*ms).moveflags &= !32i32;
    if 0 != (*initmove).or_moveflags & 32i32 { (*ms).moveflags |= 32i32 }
    (*ms).moveflags &= !16i32;
    if 0 != (*initmove).or_moveflags & 16i32 { (*ms).moveflags |= 16i32 }
    (*ms).moveflags &= !512i32;
    if 0 != (*initmove).or_moveflags & 512i32 { (*ms).moveflags |= 512i32 }
    (*ms).moveflags &= !64i32;
    if 0 != (*initmove).or_moveflags & 64i32 { (*ms).moveflags |= 64i32 };
}
//add a spot to avoid (if type == AVOID_CLEAR all spots are removed)
#[no_mangle]
pub unsafe extern "C" fn BotAddAvoidSpot(mut movestate: libc::c_int,
                                         mut origin: *mut vec_t,
                                         mut radius: libc::c_float,
                                         mut type_0: libc::c_int) {
    let mut ms: *mut bot_movestate_t = 0 as *mut bot_movestate_t;
    ms = BotMoveStateFromHandle(movestate);
    if ms.is_null() { return }
    if type_0 == 0i32 { (*ms).numavoidspots = 0i32; return }
    if (*ms).numavoidspots >= 32i32 { return }
    (*ms).avoidspots[(*ms).numavoidspots as usize].origin[0usize] =
        *origin.offset(0isize);
    (*ms).avoidspots[(*ms).numavoidspots as usize].origin[1usize] =
        *origin.offset(1isize);
    (*ms).avoidspots[(*ms).numavoidspots as usize].origin[2usize] =
        *origin.offset(2isize);
    (*ms).avoidspots[(*ms).numavoidspots as usize].radius = radius;
    (*ms).avoidspots[(*ms).numavoidspots as usize].type_0 = type_0;
    (*ms).numavoidspots += 1;
}
//must be called every map change
#[no_mangle]
pub unsafe extern "C" fn BotSetBrushModelTypes() {
    let mut ent: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut model: [libc::c_char; 128] = [0; 128];
    memset(modeltypes.as_mut_ptr() as *mut libc::c_void, 0i32,
           (256i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            if !(0 ==
                     AAS_ValueForBSPEpairKey(ent,
                                             b"model\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char,
                                             model.as_mut_ptr(), 128i32)) {
                if 0 != model[0usize] {
                    modelnum = atoi(model.as_mut_ptr().offset(1isize))
                } else { modelnum = 0i32 }
                if modelnum < 0i32 || modelnum >= 256i32 {
                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                        b"entity %s model number out of range\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        classname.as_mut_ptr());
                } else if 0 ==
                              Q_stricmp(classname.as_mut_ptr(),
                                        b"func_bobbing\x00" as *const u8 as
                                            *const libc::c_char) {
                    modeltypes[modelnum as usize] = 2i32
                } else if 0 ==
                              Q_stricmp(classname.as_mut_ptr(),
                                        b"func_plat\x00" as *const u8 as
                                            *const libc::c_char) {
                    modeltypes[modelnum as usize] = 1i32
                } else if 0 ==
                              Q_stricmp(classname.as_mut_ptr(),
                                        b"func_door\x00" as *const u8 as
                                            *const libc::c_char) {
                    modeltypes[modelnum as usize] = 3i32
                } else if 0 ==
                              Q_stricmp(classname.as_mut_ptr(),
                                        b"func_static\x00" as *const u8 as
                                            *const libc::c_char) {
                    modeltypes[modelnum as usize] = 4i32
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    };
}
//setup movement AI
#[no_mangle]
pub unsafe extern "C" fn BotSetupMoveAI() -> libc::c_int {
    BotSetBrushModelTypes();
    sv_maxstep =
        LibVar(b"sv_step\x00" as *const u8 as *const libc::c_char,
               b"18\x00" as *const u8 as *const libc::c_char);
    sv_maxbarrier =
        LibVar(b"sv_maxbarrier\x00" as *const u8 as *const libc::c_char,
               b"32\x00" as *const u8 as *const libc::c_char);
    sv_gravity =
        LibVar(b"sv_gravity\x00" as *const u8 as *const libc::c_char,
               b"800\x00" as *const u8 as *const libc::c_char);
    weapindex_rocketlauncher =
        LibVar(b"weapindex_rocketlauncher\x00" as *const u8 as
                   *const libc::c_char,
               b"5\x00" as *const u8 as *const libc::c_char);
    weapindex_bfg10k =
        LibVar(b"weapindex_bfg10k\x00" as *const u8 as *const libc::c_char,
               b"9\x00" as *const u8 as *const libc::c_char);
    weapindex_grapple =
        LibVar(b"weapindex_grapple\x00" as *const u8 as *const libc::c_char,
               b"10\x00" as *const u8 as *const libc::c_char);
    entitytypemissile =
        LibVar(b"entitytypemissile\x00" as *const u8 as *const libc::c_char,
               b"3\x00" as *const u8 as *const libc::c_char);
    offhandgrapple =
        LibVar(b"offhandgrapple\x00" as *const u8 as *const libc::c_char,
               b"0\x00" as *const u8 as *const libc::c_char);
    cmd_grappleon =
        LibVar(b"cmd_grappleon\x00" as *const u8 as *const libc::c_char,
               b"grappleon\x00" as *const u8 as *const libc::c_char);
    cmd_grappleoff =
        LibVar(b"cmd_grappleoff\x00" as *const u8 as *const libc::c_char,
               b"grappleoff\x00" as *const u8 as *const libc::c_char);
    return 0i32;
}
//shutdown movement AI
#[no_mangle]
pub unsafe extern "C" fn BotShutdownMoveAI() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i <= 64i32 {
        if !botmovestates[i as usize].is_null() {
            FreeMemory(botmovestates[i as usize] as *mut libc::c_void);
            botmovestates[i as usize] = 0 as *mut bot_movestate_t
        }
        i += 1
    };
}
//end else
//end of the function BotMoveInDirection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Intersection(mut p1: *mut vec_t, mut p2: *mut vec_t,
                                      mut p3: *mut vec_t, mut p4: *mut vec_t,
                                      mut out: *mut vec_t) -> libc::c_int {
    let mut x1: libc::c_float = 0.;
    let mut dx1: libc::c_float = 0.;
    let mut dy1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut dx2: libc::c_float = 0.;
    let mut dy2: libc::c_float = 0.;
    let mut d: libc::c_float = 0.;
    dx1 = *p2.offset(0isize) - *p1.offset(0isize);
    dy1 = *p2.offset(1isize) - *p1.offset(1isize);
    dx2 = *p4.offset(0isize) - *p3.offset(0isize);
    dy2 = *p4.offset(1isize) - *p3.offset(1isize);
    d = dy1 * dx2 - dx1 * dy2;
    if d != 0i32 as libc::c_float {
        x1 = *p1.offset(1isize) * dx1 - *p1.offset(0isize) * dy1;
        x2 = *p3.offset(1isize) * dx2 - *p3.offset(0isize) * dy2;
        *out.offset(0isize) =
            ((dx1 * x2 - dx2 * x1) / d) as libc::c_int as vec_t;
        *out.offset(1isize) =
            ((dy1 * x2 - dy2 * x1) / d) as libc::c_int as vec_t;
        return qtrue as libc::c_int
    } else { return qfalse as libc::c_int };
}
//end of the function BotTravel_Walk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFinishTravel_Walk(mut ms: *mut bot_movestate_t,
                                              mut reach:
                                                  *mut aas_reachability_t)
 -> bot_moveresult_t {
    let mut hordir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut result: bot_moveresult_t =
        bot_moveresult_s{failure: 0i32,
                         type_0: 0i32,
                         blocked: 0i32,
                         blockentity: 0i32,
                         traveltype: 0i32,
                         flags: 0i32,
                         weapon: 0i32,
                         movedir:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
                         ideal_viewangles:
                             [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],};
    hordir[0usize] = (*reach).end[0usize] - (*ms).origin[0usize];
    hordir[1usize] = (*reach).end[1usize] - (*ms).origin[1usize];
    hordir[2usize] = 0i32 as vec_t;
    dist = VectorNormalize(hordir.as_mut_ptr());
    if dist > 100i32 as libc::c_float { dist = 100i32 as libc::c_float }
    speed =
        400i32 as libc::c_float -
            (400i32 as libc::c_float - 3i32 as libc::c_float * dist);
    EA_Move((*ms).client, hordir.as_mut_ptr(), speed);
    result.movedir[0usize] = hordir[0usize];
    result.movedir[1usize] = hordir[1usize];
    result.movedir[2usize] = hordir[2usize];
    return result;
}