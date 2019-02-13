use libc;
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
}
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
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub type qhandle_t = libc::c_int;
    pub type sfxHandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    pub type clipHandle_t = libc::c_int;
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
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct pc_token_s {
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub intvalue: libc::c_int,
        pub floatvalue: libc::c_float,
        pub string: [libc::c_char; 1024],
    }
    pub type pc_token_t = pc_token_s;
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type cvarHandle_t = libc::c_int;
    // the modules that run in the virtual machine can't access the cvar_t directly,
// so they must ask for structured updates
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    pub type cplane_t = cplane_s;
    // a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct trace_t {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub entityNum: libc::c_int,
    }
    // trace->entityNum can also be 0 to (MAX_GENTITIES-1)
// or ENTITYNUM_NONE, ENTITYNUM_WORLD
    // markfragments are returned by R_MarkFragments()
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct markFragment_t {
        pub firstPoint: libc::c_int,
        pub numPoints: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct orientation_t {
        pub origin: vec3_t,
        pub axis: [vec3_t; 3],
    }
    /*
========================================================================

  ELEMENTS COMMUNICATED ACROSS THE NET

========================================================================
*/
    // snapshot used during connection and for zombies
    // toggled every map_restart so transitions can be detected
    //
// per-level limits
//
    // absolute limit
    // don't need to send any more
    // entitynums are communicated with GENTITY_BITS, so any reserved
// values that are going to be communcated over the net need to
// also be in this range
    // these are sent over the net as 8 bits
    // so they cannot be blindly increased
    // these are the only configstrings that the system reserves, all the
// other ones are strictly for servergame to clientgame communication
    // an info string with all the serverinfo cvars
    // an info string for server system to client system configuration (timescale, etc)
    // game can't modify below this, only the system can
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct gameState_t {
        pub stringOffsets: [libc::c_int; 1024],
        pub stringData: [libc::c_char; 16000],
        pub dataCount: libc::c_int,
    }
    //=========================================================
    // bit field limits
    // playerState_t is the information needed by both the client and server
// to predict player motion and actions
// nothing outside of pmove should modify these, or some degree of prediction error
// will occur
    // you can't add anything to this without modifying the code in msg.c
    // playerState_t is a full superset of entityState_t as it is used by players,
// so if a playerState_t is transmitted, the entityState_t can be fully derived
// from it.
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct playerState_s {
        pub commandTime: libc::c_int,
        pub pm_type: libc::c_int,
        pub bobCycle: libc::c_int,
        pub pm_flags: libc::c_int,
        pub pm_time: libc::c_int,
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub weaponTime: libc::c_int,
        pub gravity: libc::c_int,
        pub speed: libc::c_int,
        pub delta_angles: [libc::c_int; 3],
        pub groundEntityNum: libc::c_int,
        pub legsTimer: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoTimer: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub movementDir: libc::c_int,
        pub grapplePoint: vec3_t,
        pub eFlags: libc::c_int,
        pub eventSequence: libc::c_int,
        pub events: [libc::c_int; 2],
        pub eventParms: [libc::c_int; 2],
        pub externalEvent: libc::c_int,
        pub externalEventParm: libc::c_int,
        pub externalEventTime: libc::c_int,
        pub clientNum: libc::c_int,
        pub weapon: libc::c_int,
        pub weaponstate: libc::c_int,
        pub viewangles: vec3_t,
        pub viewheight: libc::c_int,
        pub damageEvent: libc::c_int,
        pub damageYaw: libc::c_int,
        pub damagePitch: libc::c_int,
        pub damageCount: libc::c_int,
        pub stats: [libc::c_int; 16],
        pub persistant: [libc::c_int; 16],
        pub powerups: [libc::c_int; 16],
        pub ammo: [libc::c_int; 16],
        pub generic1: libc::c_int,
        pub loopSound: libc::c_int,
        pub jumppad_ent: libc::c_int,
        pub ping: libc::c_int,
        pub pmove_framecount: libc::c_int,
        pub jumppad_frame: libc::c_int,
        pub entityEventSequence: libc::c_int,
    }
    pub type playerState_t = playerState_s;
    //====================================================================
    //
// usercmd_t->button bits, many of which are generated by the client system,
// so they aren't game/cgame only definitions
//
    // displays talk balloon and disables actions
    // walking can't just be inferred from MOVE_RUN
    // because a key pressed late in the frame will
										// only generate a small move value for that frame
										// walking will use different animations and
										// won't generate footsteps
    // any key whatsoever
    // if forwardmove or rightmove are >= MOVE_RUN,
    // then BUTTON_WALKING should be set
    // usercmd_t is sent to the server each client frame
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct usercmd_s {
        pub serverTime: libc::c_int,
        pub angles: [libc::c_int; 3],
        pub buttons: libc::c_int,
        pub weapon: byte,
        pub forwardmove: libc::c_schar,
        pub rightmove: libc::c_schar,
        pub upmove: libc::c_schar,
    }
    pub type usercmd_t = usercmd_s;
    //===================================================================
    // if entityState->solid == SOLID_BMODEL, modelindex is an inline model number
    pub type trType_t = libc::c_uint;
    pub const TR_GRAVITY: trType_t = 5;
    // value = base + sin( time / duration ) * delta
    pub const TR_SINE: trType_t = 4;
    pub const TR_LINEAR_STOP: trType_t = 3;
    pub const TR_LINEAR: trType_t = 2;
    // non-parametric, but interpolate between snapshots
    pub const TR_INTERPOLATE: trType_t = 1;
    pub const TR_STATIONARY: trType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct trajectory_t {
        pub trType: trType_t,
        pub trTime: libc::c_int,
        pub trDuration: libc::c_int,
        pub trBase: vec3_t,
        pub trDelta: vec3_t,
    }
    // entityState_t is the information conveyed from the server
// in an update message about entities that the client will
// need to render in some way
// Different eTypes may use the information in different ways
// The messages are delta compressed, so it doesn't really matter if
// the structure size is fairly large
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct entityState_s {
        pub number: libc::c_int,
        pub eType: libc::c_int,
        pub eFlags: libc::c_int,
        pub pos: trajectory_t,
        pub apos: trajectory_t,
        pub time: libc::c_int,
        pub time2: libc::c_int,
        pub origin: vec3_t,
        pub origin2: vec3_t,
        pub angles: vec3_t,
        pub angles2: vec3_t,
        pub otherEntityNum: libc::c_int,
        pub otherEntityNum2: libc::c_int,
        pub groundEntityNum: libc::c_int,
        pub constantLight: libc::c_int,
        pub loopSound: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub clientNum: libc::c_int,
        pub frame: libc::c_int,
        pub solid: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub generic1: libc::c_int,
    }
    pub type entityState_t = entityState_s;
    // font support 
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glyphInfo_t {
        pub height: libc::c_int,
        pub top: libc::c_int,
        pub bottom: libc::c_int,
        pub pitch: libc::c_int,
        pub xSkip: libc::c_int,
        pub imageWidth: libc::c_int,
        pub imageHeight: libc::c_int,
        pub s: libc::c_float,
        pub t: libc::c_float,
        pub s2: libc::c_float,
        pub t2: libc::c_float,
        pub glyph: qhandle_t,
        pub shaderName: [libc::c_char; 32],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fontInfo_t {
        pub glyphs: [glyphInfo_t; 256],
        pub glyphScale: libc::c_float,
        pub name: [libc::c_char; 64],
    }
    // real time
//=============================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qtime_s {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
    }
    pub type qtime_t = qtime_s;
    // server browser sources
// TTimo: AS_MPLAYER is no longer used
    // cinematic states
    pub type e_status = libc::c_uint;
    pub const FMV_ID_WAIT: e_status = 6;
    pub const FMV_LOOPED: e_status = 5;
    pub const FMV_ID_IDLE: e_status = 4;
    pub const FMV_ID_BLT: e_status = 3;
    // all other conditions, i.e. stop/EOF/abort
    pub const FMV_EOF: e_status = 2;
    // play
    pub const FMV_PLAY: e_status = 1;
    pub const FMV_IDLE: e_status = 0;
    use super::{libc};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
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
    // can't be increased, because bit flags are used on surfaces
    // can't be increased without changing drawsurf bit packing
    // the last N-bit number (2^REFENTITYNUM_BITS - 1) is reserved for the special world refentity,
//  and this is reflected by the value of MAX_REFENTITIES (which therefore is not a power-of-2)
    // renderfx flags
    // allways have some light (viewmodel, some items)
    // don't draw through eyes, only mirrors (player bodies, chat sprites)
    // only draw through eyes (view weapon, damage blood blob)
    // for view weapon Z crunching
    // This item is a cross hair and will draw over everything similar to
    // DEPTHHACK in stereo rendering mode, with the difference that the
						// projection matrix won't be hacked to reduce the stereo separation as
						// is done for the gun.
    // don't add stencil shadows
    // use refEntity->lightingOrigin instead of refEntity->origin
    // for lighting.  This allows entities to sink into the floor
						// with their origin going solid, and allows all parts of a
						// player to get the same lighting
    // use refEntity->shadowPlane
    // mod the model frames by the maxframes to allow continuous
    // animation without needing to know the frame count
    // refdef flags
    // used for player configuration screen
    // teleportation effect
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct polyVert_t {
        pub xyz: vec3_t,
        pub st: [libc::c_float; 2],
        pub modulate: [byte; 4],
    }
    pub type refEntityType_t = libc::c_uint;
    pub const RT_MAX_REF_ENTITY_TYPE: refEntityType_t = 8;
    // doesn't draw anything, just info for portals
    pub const RT_PORTALSURFACE: refEntityType_t = 7;
    pub const RT_LIGHTNING: refEntityType_t = 6;
    pub const RT_RAIL_RINGS: refEntityType_t = 5;
    pub const RT_RAIL_CORE: refEntityType_t = 4;
    pub const RT_BEAM: refEntityType_t = 3;
    pub const RT_SPRITE: refEntityType_t = 2;
    pub const RT_POLY: refEntityType_t = 1;
    pub const RT_MODEL: refEntityType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refEntity_t {
        pub reType: refEntityType_t,
        pub renderfx: libc::c_int,
        pub hModel: qhandle_t,
        pub lightingOrigin: vec3_t,
        pub shadowPlane: libc::c_float,
        pub axis: [vec3_t; 3],
        pub nonNormalizedAxes: qboolean,
        pub origin: [libc::c_float; 3],
        pub frame: libc::c_int,
        pub oldorigin: [libc::c_float; 3],
        pub oldframe: libc::c_int,
        pub backlerp: libc::c_float,
        pub skinNum: libc::c_int,
        pub customSkin: qhandle_t,
        pub customShader: qhandle_t,
        pub shaderRGBA: [byte; 4],
        pub shaderTexCoord: [libc::c_float; 2],
        pub shaderTime: libc::c_float,
        pub radius: libc::c_float,
        pub rotation: libc::c_float,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refdef_t {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub fov_x: libc::c_float,
        pub fov_y: libc::c_float,
        pub vieworg: vec3_t,
        pub viewaxis: [vec3_t; 3],
        pub time: libc::c_int,
        pub rdflags: libc::c_int,
        pub areamask: [byte; 32],
        pub text: [[libc::c_char; 32]; 8],
    }
    /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
    pub type textureCompression_t = libc::c_uint;
    // this is for the GL_EXT_texture_compression_s3tc extension.
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    // this is for the GL_S3_s3tc extension.
    pub const TC_S3TC: textureCompression_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glDriverType_t = libc::c_uint;
    // driver is a 3Dfx standalone driver
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    // WARNING: there are tests that check for
								// > GLDRV_ICD for minidriverness, so this
								// should always be the lowest value in this
								// enum set
    // driver is a non-3Dfx standalone driver
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    // driver is integrated with window system
    pub const GLDRV_ICD: glDriverType_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    // where you don't have src*dst
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    // where you can't modulate alpha on alpha textures
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    // the hardware type then there can NOT exist a secondary
							// display adapter
    // where you can't interpolate alpha
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    // Voodoo Banshee or Voodoo3, relevant since if this is
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    // where everything works the way it should
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    use super::q_shared_h::{vec3_t, byte, qhandle_t, qboolean};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/cgame/cg_public.h"]
pub mod cg_public_h {
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
    // allow a lot of command backups for very fast systems
// multiple commands may be combined into a single packet, so this
// needs to be larger than PACKET_BACKUP
    // snapshots are a view of the server at a given time
    // Snapshots are generated at regular time intervals by the server,
// but they may not be sent if a client's rate level is exceeded, or
// they may be dropped by the network.
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snapshot_t {
        pub snapFlags: libc::c_int,
        pub ping: libc::c_int,
        pub serverTime: libc::c_int,
        pub areamask: [byte; 32],
        pub ps: playerState_t,
        pub numEntities: libc::c_int,
        pub entities: [entityState_t; 256],
        pub numServerCommands: libc::c_int,
        pub serverCommandSequence: libc::c_int,
    }
    /*
==================================================================

functions imported from the main executable

==================================================================
*/
    pub type unnamed = libc::c_uint;
    pub const CG_ACOS: unnamed = 111;
    pub const CG_TESTPRINTFLOAT: unnamed = 110;
    pub const CG_TESTPRINTINT: unnamed = 109;
    pub const CG_CEIL: unnamed = 108;
    pub const CG_FLOOR: unnamed = 107;
    pub const CG_SQRT: unnamed = 106;
    pub const CG_ATAN2: unnamed = 105;
    pub const CG_COS: unnamed = 104;
    pub const CG_SIN: unnamed = 103;
    pub const CG_STRNCPY: unnamed = 102;
    pub const CG_MEMCPY: unnamed = 101;
    /*
	CG_LOADCAMERA,
	CG_STARTCAMERA,
	CG_GETCAMERAINFO,
*/
    pub const CG_MEMSET: unnamed = 100;
    // 1.32
    pub const CG_FS_SEEK: unnamed = 89;
    pub const CG_R_INPVS: unnamed = 88;
    pub const CG_R_ADDPOLYSTOSCENE: unnamed = 87;
    pub const CG_GET_ENTITY_TOKEN: unnamed = 86;
    pub const CG_R_ADDADDITIVELIGHTTOSCENE: unnamed = 85;
    pub const CG_CM_TRANSFORMEDCAPSULETRACE: unnamed = 84;
    pub const CG_CM_CAPSULETRACE: unnamed = 83;
    pub const CG_CM_TEMPCAPSULEMODEL: unnamed = 82;
    pub const CG_S_STOPLOOPINGSOUND: unnamed = 81;
    pub const CG_S_ADDREALLOOPINGSOUND: unnamed = 80;
    pub const CG_R_REMAP_SHADER: unnamed = 79;
    pub const CG_CIN_SETEXTENTS: unnamed = 78;
    pub const CG_CIN_DRAWCINEMATIC: unnamed = 77;
    pub const CG_CIN_RUNCINEMATIC: unnamed = 76;
    pub const CG_CIN_STOPCINEMATIC: unnamed = 75;
    pub const CG_CIN_PLAYCINEMATIC: unnamed = 74;
    pub const CG_R_LIGHTFORPOINT: unnamed = 73;
    pub const CG_REMOVECOMMAND: unnamed = 72;
    pub const CG_SNAPVECTOR: unnamed = 71;
    pub const CG_REAL_TIME: unnamed = 70;
    pub const CG_S_STOPBACKGROUNDTRACK: unnamed = 69;
    pub const CG_PC_SOURCE_FILE_AND_LINE: unnamed = 68;
    pub const CG_PC_READ_TOKEN: unnamed = 67;
    pub const CG_PC_FREE_SOURCE: unnamed = 66;
    pub const CG_PC_LOAD_SOURCE: unnamed = 65;
    pub const CG_PC_ADD_GLOBAL_DEFINE: unnamed = 64;
    pub const CG_KEY_GETKEY: unnamed = 63;
    pub const CG_KEY_SETCATCHER: unnamed = 62;
    pub const CG_KEY_GETCATCHER: unnamed = 61;
    pub const CG_KEY_ISDOWN: unnamed = 60;
    pub const CG_R_REGISTERFONT: unnamed = 59;
    pub const CG_MEMORY_REMAINING: unnamed = 58;
    pub const CG_R_REGISTERSHADERNOMIP: unnamed = 57;
    pub const CG_SETUSERCMDVALUE: unnamed = 56;
    pub const CG_GETUSERCMD: unnamed = 55;
    pub const CG_GETCURRENTCMDNUMBER: unnamed = 54;
    pub const CG_GETSERVERCOMMAND: unnamed = 53;
    pub const CG_GETSNAPSHOT: unnamed = 52;
    pub const CG_GETCURRENTSNAPSHOTNUMBER: unnamed = 51;
    pub const CG_GETGAMESTATE: unnamed = 50;
    pub const CG_GETGLCONFIG: unnamed = 49;
    pub const CG_R_LERPTAG: unnamed = 48;
    pub const CG_R_MODELBOUNDS: unnamed = 47;
    pub const CG_R_DRAWSTRETCHPIC: unnamed = 46;
    pub const CG_R_SETCOLOR: unnamed = 45;
    pub const CG_R_RENDERSCENE: unnamed = 44;
    pub const CG_R_ADDLIGHTTOSCENE: unnamed = 43;
    pub const CG_R_ADDPOLYTOSCENE: unnamed = 42;
    pub const CG_R_ADDREFENTITYTOSCENE: unnamed = 41;
    pub const CG_R_CLEARSCENE: unnamed = 40;
    pub const CG_R_REGISTERSHADER: unnamed = 39;
    pub const CG_R_REGISTERSKIN: unnamed = 38;
    pub const CG_R_REGISTERMODEL: unnamed = 37;
    pub const CG_R_LOADWORLDMAP: unnamed = 36;
    pub const CG_S_STARTBACKGROUNDTRACK: unnamed = 35;
    pub const CG_S_REGISTERSOUND: unnamed = 34;
    pub const CG_S_RESPATIALIZE: unnamed = 33;
    pub const CG_S_UPDATEENTITYPOSITION: unnamed = 32;
    pub const CG_S_ADDLOOPINGSOUND: unnamed = 31;
    pub const CG_S_CLEARLOOPINGSOUNDS: unnamed = 30;
    pub const CG_S_STARTLOCALSOUND: unnamed = 29;
    pub const CG_S_STARTSOUND: unnamed = 28;
    pub const CG_CM_MARKFRAGMENTS: unnamed = 27;
    pub const CG_CM_TRANSFORMEDBOXTRACE: unnamed = 26;
    pub const CG_CM_BOXTRACE: unnamed = 25;
    pub const CG_CM_TRANSFORMEDPOINTCONTENTS: unnamed = 24;
    pub const CG_CM_POINTCONTENTS: unnamed = 23;
    pub const CG_CM_TEMPBOXMODEL: unnamed = 22;
    pub const CG_CM_LOADMODEL: unnamed = 21;
    pub const CG_CM_INLINEMODEL: unnamed = 20;
    pub const CG_CM_NUMINLINEMODELS: unnamed = 19;
    pub const CG_CM_LOADMAP: unnamed = 18;
    pub const CG_UPDATESCREEN: unnamed = 17;
    pub const CG_SENDCLIENTCOMMAND: unnamed = 16;
    pub const CG_ADDCOMMAND: unnamed = 15;
    pub const CG_SENDCONSOLECOMMAND: unnamed = 14;
    pub const CG_FS_FCLOSEFILE: unnamed = 13;
    pub const CG_FS_WRITE: unnamed = 12;
    pub const CG_FS_READ: unnamed = 11;
    pub const CG_FS_FOPENFILE: unnamed = 10;
    pub const CG_ARGS: unnamed = 9;
    pub const CG_ARGV: unnamed = 8;
    pub const CG_ARGC: unnamed = 7;
    pub const CG_CVAR_VARIABLESTRINGBUFFER: unnamed = 6;
    pub const CG_CVAR_SET: unnamed = 5;
    pub const CG_CVAR_UPDATE: unnamed = 4;
    pub const CG_CVAR_REGISTER: unnamed = 3;
    pub const CG_MILLISECONDS: unnamed = 2;
    pub const CG_ERROR: unnamed = 1;
    pub const CG_PRINT: unnamed = 0;
    use super::{libc};
    use super::q_shared_h::{byte, playerState_t, entityState_t};
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/cgame/cg_local.h"]
pub mod cg_local_h {
    use super::{libc};
    use super::q_shared_h::{vmCvar_t, fileHandle_t, fsMode_t, clipHandle_t,
                            vec_t, trace_t, vec3_t, markFragment_t,
                            sfxHandle_t, qboolean, qhandle_t, orientation_t,
                            gameState_t, usercmd_t, fontInfo_t, e_status,
                            qtime_t};
    use super::tr_types_h::{refEntity_t, polyVert_t, refdef_t, glconfig_t};
    use super::cg_public_h::{snapshot_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/cgame/cg_syscalls.c"]
pub mod cg_syscalls_c {
    use super::{libc};
    use super::stdint_h::{intptr_t};
    use super::q_shared_h::{clipHandle_t, vec_t, pc_token_t};
}
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t, qhandle_t,
                       sfxHandle_t, fileHandle_t, clipHandle_t, vec_t, vec3_t,
                       cplane_s, pc_token_s, pc_token_t, fsMode_t,
                       FS_APPEND_SYNC, FS_APPEND, FS_WRITE, FS_READ,
                       cvarHandle_t, vmCvar_t, cplane_t, trace_t,
                       markFragment_t, orientation_t, gameState_t,
                       playerState_s, playerState_t, usercmd_s, usercmd_t,
                       trType_t, TR_GRAVITY, TR_SINE, TR_LINEAR_STOP,
                       TR_LINEAR, TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, glyphInfo_t, fontInfo_t,
                       qtime_s, qtime_t, e_status, FMV_ID_WAIT, FMV_LOOPED,
                       FMV_ID_IDLE, FMV_ID_BLT, FMV_EOF, FMV_PLAY, FMV_IDLE};
use self::tr_types_h::{polyVert_t, refEntityType_t, RT_MAX_REF_ENTITY_TYPE,
                       RT_PORTALSURFACE, RT_LIGHTNING, RT_RAIL_RINGS,
                       RT_RAIL_CORE, RT_BEAM, RT_SPRITE, RT_POLY, RT_MODEL,
                       refEntity_t, refdef_t, textureCompression_t,
                       TC_S3TC_ARB, TC_S3TC, TC_NONE, glDriverType_t,
                       GLDRV_VOODOO, GLDRV_STANDALONE, GLDRV_ICD,
                       glHardwareType_t, GLHW_PERMEDIA2, GLHW_RAGEPRO,
                       GLHW_RIVA128, GLHW_3DFX_2D3D, GLHW_GENERIC,
                       glconfig_t};
use self::cg_public_h::{snapshot_t, unnamed, CG_ACOS, CG_TESTPRINTFLOAT,
                        CG_TESTPRINTINT, CG_CEIL, CG_FLOOR, CG_SQRT, CG_ATAN2,
                        CG_COS, CG_SIN, CG_STRNCPY, CG_MEMCPY, CG_MEMSET,
                        CG_FS_SEEK, CG_R_INPVS, CG_R_ADDPOLYSTOSCENE,
                        CG_GET_ENTITY_TOKEN, CG_R_ADDADDITIVELIGHTTOSCENE,
                        CG_CM_TRANSFORMEDCAPSULETRACE, CG_CM_CAPSULETRACE,
                        CG_CM_TEMPCAPSULEMODEL, CG_S_STOPLOOPINGSOUND,
                        CG_S_ADDREALLOOPINGSOUND, CG_R_REMAP_SHADER,
                        CG_CIN_SETEXTENTS, CG_CIN_DRAWCINEMATIC,
                        CG_CIN_RUNCINEMATIC, CG_CIN_STOPCINEMATIC,
                        CG_CIN_PLAYCINEMATIC, CG_R_LIGHTFORPOINT,
                        CG_REMOVECOMMAND, CG_SNAPVECTOR, CG_REAL_TIME,
                        CG_S_STOPBACKGROUNDTRACK, CG_PC_SOURCE_FILE_AND_LINE,
                        CG_PC_READ_TOKEN, CG_PC_FREE_SOURCE,
                        CG_PC_LOAD_SOURCE, CG_PC_ADD_GLOBAL_DEFINE,
                        CG_KEY_GETKEY, CG_KEY_SETCATCHER, CG_KEY_GETCATCHER,
                        CG_KEY_ISDOWN, CG_R_REGISTERFONT, CG_MEMORY_REMAINING,
                        CG_R_REGISTERSHADERNOMIP, CG_SETUSERCMDVALUE,
                        CG_GETUSERCMD, CG_GETCURRENTCMDNUMBER,
                        CG_GETSERVERCOMMAND, CG_GETSNAPSHOT,
                        CG_GETCURRENTSNAPSHOTNUMBER, CG_GETGAMESTATE,
                        CG_GETGLCONFIG, CG_R_LERPTAG, CG_R_MODELBOUNDS,
                        CG_R_DRAWSTRETCHPIC, CG_R_SETCOLOR, CG_R_RENDERSCENE,
                        CG_R_ADDLIGHTTOSCENE, CG_R_ADDPOLYTOSCENE,
                        CG_R_ADDREFENTITYTOSCENE, CG_R_CLEARSCENE,
                        CG_R_REGISTERSHADER, CG_R_REGISTERSKIN,
                        CG_R_REGISTERMODEL, CG_R_LOADWORLDMAP,
                        CG_S_STARTBACKGROUNDTRACK, CG_S_REGISTERSOUND,
                        CG_S_RESPATIALIZE, CG_S_UPDATEENTITYPOSITION,
                        CG_S_ADDLOOPINGSOUND, CG_S_CLEARLOOPINGSOUNDS,
                        CG_S_STARTLOCALSOUND, CG_S_STARTSOUND,
                        CG_CM_MARKFRAGMENTS, CG_CM_TRANSFORMEDBOXTRACE,
                        CG_CM_BOXTRACE, CG_CM_TRANSFORMEDPOINTCONTENTS,
                        CG_CM_POINTCONTENTS, CG_CM_TEMPBOXMODEL,
                        CG_CM_LOADMODEL, CG_CM_INLINEMODEL,
                        CG_CM_NUMINLINEMODELS, CG_CM_LOADMAP, CG_UPDATESCREEN,
                        CG_SENDCLIENTCOMMAND, CG_ADDCOMMAND,
                        CG_SENDCONSOLECOMMAND, CG_FS_FCLOSEFILE, CG_FS_WRITE,
                        CG_FS_READ, CG_FS_FOPENFILE, CG_ARGS, CG_ARGV,
                        CG_ARGC, CG_CVAR_VARIABLESTRINGBUFFER, CG_CVAR_SET,
                        CG_CVAR_UPDATE, CG_CVAR_REGISTER, CG_MILLISECONDS,
                        CG_ERROR, CG_PRINT};
use self::stdlib_h::{exit};
//===============================================
//
// system traps
// These functions are how the cgame communicates with the main game system
//
// print message on the local console
#[no_mangle]
pub unsafe extern "C" fn trap_Print(mut fmt: *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_PRINT as libc::c_int as
                                                    intptr_t, fmt);
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
// cg_syscalls.c -- this file is only included when building a dll
// cg_syscalls.asm is included instead when building a qvm
static mut syscall: Option<unsafe extern "C" fn(_: intptr_t, ...) -> intptr_t>
       =
    ::std::mem::transmute::<libc::intptr_t,
                            Option<unsafe extern "C" fn(_: intptr_t, ...)
                                       -> intptr_t>>(-1i32 as libc::intptr_t);
// abort the game
#[no_mangle]
pub unsafe extern "C" fn trap_Error(mut fmt: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(CG_ERROR as libc::c_int as
                                                    intptr_t, fmt);
    exit(1i32);
}
// milliseconds should only be used for performance tuning, never
// for anything game related.  Get time from the CG_DrawActiveFrame parameter
#[no_mangle]
pub unsafe extern "C" fn trap_Milliseconds() -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_MILLISECONDS as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
// console variable interaction
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Register(mut vmCvar: *mut vmCvar_t,
                                            mut varName: *const libc::c_char,
                                            mut defaultValue:
                                                *const libc::c_char,
                                            mut flags: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_CVAR_REGISTER as
                                                    libc::c_int as intptr_t,
                                                vmCvar, varName, defaultValue,
                                                flags);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Update(mut vmCvar: *mut vmCvar_t) {
    syscall.expect("non-null function pointer")(CG_CVAR_UPDATE as libc::c_int
                                                    as intptr_t, vmCvar);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Set(mut var_name: *const libc::c_char,
                                       mut value: *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_CVAR_SET as libc::c_int as
                                                    intptr_t, var_name,
                                                value);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(mut var_name:
                                                            *const libc::c_char,
                                                        mut buffer:
                                                            *mut libc::c_char,
                                                        mut bufsize:
                                                            libc::c_int) {
    syscall.expect("non-null function pointer")(CG_CVAR_VARIABLESTRINGBUFFER
                                                    as libc::c_int as
                                                    intptr_t, var_name,
                                                buffer, bufsize);
}
// ServerCommand and ConsoleCommand parameter access
#[no_mangle]
pub unsafe extern "C" fn trap_Argc() -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_ARGC as libc::c_int
                                                           as intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Argv(mut n: libc::c_int,
                                   mut buffer: *mut libc::c_char,
                                   mut bufferLength: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_ARGV as libc::c_int as
                                                    intptr_t, n, buffer,
                                                bufferLength);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Args(mut buffer: *mut libc::c_char,
                                   mut bufferLength: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_ARGS as libc::c_int as
                                                    intptr_t, buffer,
                                                bufferLength);
}
// filesystem access
// returns length of file
#[no_mangle]
pub unsafe extern "C" fn trap_FS_FOpenFile(mut qpath: *const libc::c_char,
                                           mut f: *mut fileHandle_t,
                                           mut mode: fsMode_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_FS_FOPENFILE as
                                                           libc::c_int as
                                                           intptr_t, qpath, f,
                                                       mode as libc::c_uint)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Read(mut buffer: *mut libc::c_void,
                                      mut len: libc::c_int,
                                      mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(CG_FS_READ as libc::c_int as
                                                    intptr_t, buffer, len, f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Write(mut buffer: *const libc::c_void,
                                       mut len: libc::c_int,
                                       mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(CG_FS_WRITE as libc::c_int as
                                                    intptr_t, buffer, len, f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(CG_FS_FCLOSEFILE as
                                                    libc::c_int as intptr_t,
                                                f);
}
// fsOrigin_t
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Seek(mut f: fileHandle_t,
                                      mut offset: libc::c_long,
                                      mut origin: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_FS_SEEK as
                                                           libc::c_int as
                                                           intptr_t, f,
                                                       offset, origin) as
               libc::c_int;
}
// add commands to the local console as if they were typed in
// for map changing, etc.  The command is not executed immediately,
// but will be executed in order the next time console commands
// are processed
#[no_mangle]
pub unsafe extern "C" fn trap_SendConsoleCommand(mut text:
                                                     *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_SENDCONSOLECOMMAND as
                                                    libc::c_int as intptr_t,
                                                text);
}
// register a command name so the console can perform command completion.
// FIXME: replace this with a normal console command "defineCommand"?
#[no_mangle]
pub unsafe extern "C" fn trap_AddCommand(mut cmdName: *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_ADDCOMMAND as libc::c_int
                                                    as intptr_t, cmdName);
}
#[no_mangle]
pub unsafe extern "C" fn trap_RemoveCommand(mut cmdName:
                                                *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_REMOVECOMMAND as
                                                    libc::c_int as intptr_t,
                                                cmdName);
}
// send a string to the server over the network
#[no_mangle]
pub unsafe extern "C" fn trap_SendClientCommand(mut s: *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_SENDCLIENTCOMMAND as
                                                    libc::c_int as intptr_t,
                                                s);
}
// force a screen update, only used during gamestate load
#[no_mangle]
pub unsafe extern "C" fn trap_UpdateScreen() {
    syscall.expect("non-null function pointer")(CG_UPDATESCREEN as libc::c_int
                                                    as intptr_t);
}
// model collision
#[no_mangle]
pub unsafe extern "C" fn trap_CM_LoadMap(mut mapname: *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_CM_LOADMAP as libc::c_int
                                                    as intptr_t, mapname);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_NumInlineModels() -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_CM_NUMINLINEMODELS
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
// 0 = world, 1+ = bmodels
#[no_mangle]
pub unsafe extern "C" fn trap_CM_InlineModel(mut index: libc::c_int)
 -> clipHandle_t {
    return syscall.expect("non-null function pointer")(CG_CM_INLINEMODEL as
                                                           libc::c_int as
                                                           intptr_t, index) as
               clipHandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_TempBoxModel(mut mins: *const vec_t,
                                              mut maxs: *const vec_t)
 -> clipHandle_t {
    return syscall.expect("non-null function pointer")(CG_CM_TEMPBOXMODEL as
                                                           libc::c_int as
                                                           intptr_t, mins,
                                                       maxs) as clipHandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_PointContents(mut p: *const vec_t,
                                               mut model: clipHandle_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_CM_POINTCONTENTS as
                                                           libc::c_int as
                                                           intptr_t, p, model)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_TransformedPointContents(mut p: *const vec_t,
                                                          mut model:
                                                              clipHandle_t,
                                                          mut origin:
                                                              *const vec_t,
                                                          mut angles:
                                                              *const vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_CM_TRANSFORMEDPOINTCONTENTS
                                                           as libc::c_int as
                                                           intptr_t, p, model,
                                                       origin, angles) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_BoxTrace(mut results: *mut trace_t,
                                          mut start: *const vec_t,
                                          mut end: *const vec_t,
                                          mut mins: *const vec_t,
                                          mut maxs: *const vec_t,
                                          mut model: clipHandle_t,
                                          mut brushmask: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_CM_BOXTRACE as libc::c_int
                                                    as intptr_t, results,
                                                start, end, mins, maxs, model,
                                                brushmask);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_CapsuleTrace(mut results: *mut trace_t,
                                              mut start: *const vec_t,
                                              mut end: *const vec_t,
                                              mut mins: *const vec_t,
                                              mut maxs: *const vec_t,
                                              mut model: clipHandle_t,
                                              mut brushmask: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_CM_CAPSULETRACE as
                                                    libc::c_int as intptr_t,
                                                results, start, end, mins,
                                                maxs, model, brushmask);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_TransformedBoxTrace(mut results:
                                                         *mut trace_t,
                                                     mut start: *const vec_t,
                                                     mut end: *const vec_t,
                                                     mut mins: *const vec_t,
                                                     mut maxs: *const vec_t,
                                                     mut model: clipHandle_t,
                                                     mut brushmask:
                                                         libc::c_int,
                                                     mut origin: *const vec_t,
                                                     mut angles:
                                                         *const vec_t) {
    syscall.expect("non-null function pointer")(CG_CM_TRANSFORMEDBOXTRACE as
                                                    libc::c_int as intptr_t,
                                                results, start, end, mins,
                                                maxs, model, brushmask,
                                                origin, angles);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_TransformedCapsuleTrace(mut results:
                                                             *mut trace_t,
                                                         mut start:
                                                             *const vec_t,
                                                         mut end:
                                                             *const vec_t,
                                                         mut mins:
                                                             *const vec_t,
                                                         mut maxs:
                                                             *const vec_t,
                                                         mut model:
                                                             clipHandle_t,
                                                         mut brushmask:
                                                             libc::c_int,
                                                         mut origin:
                                                             *const vec_t,
                                                         mut angles:
                                                             *const vec_t) {
    syscall.expect("non-null function pointer")(CG_CM_TRANSFORMEDCAPSULETRACE
                                                    as libc::c_int as
                                                    intptr_t, results, start,
                                                end, mins, maxs, model,
                                                brushmask, origin, angles);
}
// Returns the projection of a polygon onto the solid brushes in the world
#[no_mangle]
pub unsafe extern "C" fn trap_CM_MarkFragments(mut numPoints: libc::c_int,
                                               mut points: *const vec3_t,
                                               mut projection: *const vec_t,
                                               mut maxPoints: libc::c_int,
                                               mut pointBuffer: *mut vec_t,
                                               mut maxFragments: libc::c_int,
                                               mut fragmentBuffer:
                                                   *mut markFragment_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_CM_MARKFRAGMENTS as
                                                           libc::c_int as
                                                           intptr_t,
                                                       numPoints, points,
                                                       projection, maxPoints,
                                                       pointBuffer,
                                                       maxFragments,
                                                       fragmentBuffer) as
               libc::c_int;
}
// normal sounds will have their volume dynamically changed as their entity
// moves and the listener moves
#[no_mangle]
pub unsafe extern "C" fn trap_S_StartSound(mut origin: *mut vec_t,
                                           mut entityNum: libc::c_int,
                                           mut entchannel: libc::c_int,
                                           mut sfx: sfxHandle_t) {
    syscall.expect("non-null function pointer")(CG_S_STARTSOUND as libc::c_int
                                                    as intptr_t, origin,
                                                entityNum, entchannel, sfx);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_StopLoopingSound(mut entityNum: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_S_STOPLOOPINGSOUND as
                                                    libc::c_int as intptr_t,
                                                entityNum);
}
// a local sound is always played full volume
#[no_mangle]
pub unsafe extern "C" fn trap_S_StartLocalSound(mut sfx: sfxHandle_t,
                                                mut channelNum: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_S_STARTLOCALSOUND as
                                                    libc::c_int as intptr_t,
                                                sfx, channelNum);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_ClearLoopingSounds(mut killall: qboolean) {
    syscall.expect("non-null function pointer")(CG_S_CLEARLOOPINGSOUNDS as
                                                    libc::c_int as intptr_t,
                                                killall as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_AddLoopingSound(mut entityNum: libc::c_int,
                                                mut origin: *const vec_t,
                                                mut velocity: *const vec_t,
                                                mut sfx: sfxHandle_t) {
    syscall.expect("non-null function pointer")(CG_S_ADDLOOPINGSOUND as
                                                    libc::c_int as intptr_t,
                                                entityNum, origin, velocity,
                                                sfx);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_AddRealLoopingSound(mut entityNum:
                                                        libc::c_int,
                                                    mut origin: *const vec_t,
                                                    mut velocity:
                                                        *const vec_t,
                                                    mut sfx: sfxHandle_t) {
    syscall.expect("non-null function pointer")(CG_S_ADDREALLOOPINGSOUND as
                                                    libc::c_int as intptr_t,
                                                entityNum, origin, velocity,
                                                sfx);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_UpdateEntityPosition(mut entityNum:
                                                         libc::c_int,
                                                     mut origin:
                                                         *const vec_t) {
    syscall.expect("non-null function pointer")(CG_S_UPDATEENTITYPOSITION as
                                                    libc::c_int as intptr_t,
                                                entityNum, origin);
}
// respatialize recalculates the volumes of sound as they should be heard by the
// given entityNum and position
#[no_mangle]
pub unsafe extern "C" fn trap_S_Respatialize(mut entityNum: libc::c_int,
                                             mut origin: *const vec_t,
                                             mut axis: *mut vec3_t,
                                             mut inwater: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_S_RESPATIALIZE as
                                                    libc::c_int as intptr_t,
                                                entityNum, origin, axis,
                                                inwater);
}
// returns buzz if not found
#[no_mangle]
pub unsafe extern "C" fn trap_S_RegisterSound(mut sample: *const libc::c_char,
                                              mut compressed: qboolean)
 -> sfxHandle_t {
    return syscall.expect("non-null function pointer")(CG_S_REGISTERSOUND as
                                                           libc::c_int as
                                                           intptr_t, sample,
                                                       compressed as
                                                           libc::c_uint) as
               sfxHandle_t;
}
// empty name stops music
#[no_mangle]
pub unsafe extern "C" fn trap_S_StartBackgroundTrack(mut intro:
                                                         *const libc::c_char,
                                                     mut loop_0:
                                                         *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_S_STARTBACKGROUNDTRACK as
                                                    libc::c_int as intptr_t,
                                                intro, loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_StopBackgroundTrack() {
    syscall.expect("non-null function pointer")(CG_S_STOPBACKGROUNDTRACK as
                                                    libc::c_int as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_LoadWorldMap(mut mapname:
                                                 *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_R_LOADWORLDMAP as
                                                    libc::c_int as intptr_t,
                                                mapname);
}
// all media should be registered during level startup to prevent
// hitches during gameplay
// returns rgb axis if not found
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterModel(mut name: *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(CG_R_REGISTERMODEL as
                                                           libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
// returns all white if not found
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterSkin(mut name: *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(CG_R_REGISTERSKIN as
                                                           libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
// returns all white if not found
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterShader(mut name: *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(CG_R_REGISTERSHADER as
                                                           libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
// returns all white if not found
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterShaderNoMip(mut name:
                                                        *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(CG_R_REGISTERSHADERNOMIP
                                                           as libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
// a scene is built up by calls to R_ClearScene and the various R_Add functions.
// Nothing is drawn until R_RenderScene is called.
#[no_mangle]
pub unsafe extern "C" fn trap_R_ClearScene() {
    syscall.expect("non-null function pointer")(CG_R_CLEARSCENE as libc::c_int
                                                    as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddRefEntityToScene(mut re:
                                                        *const refEntity_t) {
    syscall.expect("non-null function pointer")(CG_R_ADDREFENTITYTOSCENE as
                                                    libc::c_int as intptr_t,
                                                re);
}
// polys are intended for simple wall marks, not really for doing
// significant construction
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddPolyToScene(mut hShader: qhandle_t,
                                               mut numVerts: libc::c_int,
                                               mut verts: *const polyVert_t) {
    syscall.expect("non-null function pointer")(CG_R_ADDPOLYTOSCENE as
                                                    libc::c_int as intptr_t,
                                                hShader, numVerts, verts);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddPolysToScene(mut hShader: qhandle_t,
                                                mut numVerts: libc::c_int,
                                                mut verts: *const polyVert_t,
                                                mut num: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_R_ADDPOLYSTOSCENE as
                                                    libc::c_int as intptr_t,
                                                hShader, numVerts, verts,
                                                num);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddLightToScene(mut org: *const vec_t,
                                                mut intensity: libc::c_float,
                                                mut r: libc::c_float,
                                                mut g: libc::c_float,
                                                mut b: libc::c_float) {
    syscall.expect("non-null function pointer")(CG_R_ADDLIGHTTOSCENE as
                                                    libc::c_int as intptr_t,
                                                org, PASSFLOAT(intensity),
                                                PASSFLOAT(r), PASSFLOAT(g),
                                                PASSFLOAT(b));
}
#[no_mangle]
pub unsafe extern "C" fn PASSFLOAT(mut x: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.f = x;
    return fi.i;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddAdditiveLightToScene(mut org: *const vec_t,
                                                        mut intensity:
                                                            libc::c_float,
                                                        mut r: libc::c_float,
                                                        mut g: libc::c_float,
                                                        mut b:
                                                            libc::c_float) {
    syscall.expect("non-null function pointer")(CG_R_ADDADDITIVELIGHTTOSCENE
                                                    as libc::c_int as
                                                    intptr_t, org,
                                                PASSFLOAT(intensity),
                                                PASSFLOAT(r), PASSFLOAT(g),
                                                PASSFLOAT(b));
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_LightForPoint(mut point: *mut vec_t,
                                              mut ambientLight: *mut vec_t,
                                              mut directedLight: *mut vec_t,
                                              mut lightDir: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_R_LIGHTFORPOINT as
                                                           libc::c_int as
                                                           intptr_t, point,
                                                       ambientLight,
                                                       directedLight,
                                                       lightDir) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RenderScene(mut fd: *const refdef_t) {
    syscall.expect("non-null function pointer")(CG_R_RENDERSCENE as
                                                    libc::c_int as intptr_t,
                                                fd);
}
// NULL = 1,1,1,1
#[no_mangle]
pub unsafe extern "C" fn trap_R_SetColor(mut rgba: *const libc::c_float) {
    syscall.expect("non-null function pointer")(CG_R_SETCOLOR as libc::c_int
                                                    as intptr_t, rgba);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_DrawStretchPic(mut x: libc::c_float,
                                               mut y: libc::c_float,
                                               mut w: libc::c_float,
                                               mut h: libc::c_float,
                                               mut s1: libc::c_float,
                                               mut t1: libc::c_float,
                                               mut s2: libc::c_float,
                                               mut t2: libc::c_float,
                                               mut hShader: qhandle_t) {
    syscall.expect("non-null function pointer")(CG_R_DRAWSTRETCHPIC as
                                                    libc::c_int as intptr_t,
                                                PASSFLOAT(x), PASSFLOAT(y),
                                                PASSFLOAT(w), PASSFLOAT(h),
                                                PASSFLOAT(s1), PASSFLOAT(t1),
                                                PASSFLOAT(s2), PASSFLOAT(t2),
                                                hShader);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_ModelBounds(mut model: clipHandle_t,
                                            mut mins: *mut vec_t,
                                            mut maxs: *mut vec_t) {
    syscall.expect("non-null function pointer")(CG_R_MODELBOUNDS as
                                                    libc::c_int as intptr_t,
                                                model, mins, maxs);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_LerpTag(mut tag: *mut orientation_t,
                                        mut mod_0: clipHandle_t,
                                        mut startFrame: libc::c_int,
                                        mut endFrame: libc::c_int,
                                        mut frac: libc::c_float,
                                        mut tagName: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_R_LERPTAG as
                                                           libc::c_int as
                                                           intptr_t, tag,
                                                       mod_0, startFrame,
                                                       endFrame,
                                                       PASSFLOAT(frac),
                                                       tagName) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RemapShader(mut oldShader:
                                                *const libc::c_char,
                                            mut newShader:
                                                *const libc::c_char,
                                            mut timeOffset:
                                                *const libc::c_char) {
    syscall.expect("non-null function pointer")(CG_R_REMAP_SHADER as
                                                    libc::c_int as intptr_t,
                                                oldShader, newShader,
                                                timeOffset);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_inPVS(mut p1: *const vec_t,
                                      mut p2: *const vec_t) -> qboolean {
    return syscall.expect("non-null function pointer")(CG_R_INPVS as
                                                           libc::c_int as
                                                           intptr_t, p1, p2)
               as qboolean;
}
// The glconfig_t will not change during the life of a cgame.
// If it needs to change, the entire cgame will be restarted, because
// all the qhandle_t are then invalid.
#[no_mangle]
pub unsafe extern "C" fn trap_GetGlconfig(mut glconfig: *mut glconfig_t) {
    syscall.expect("non-null function pointer")(CG_GETGLCONFIG as libc::c_int
                                                    as intptr_t, glconfig);
}
// the gamestate should be grabbed at startup, and whenever a
// configstring changes
#[no_mangle]
pub unsafe extern "C" fn trap_GetGameState(mut gamestate: *mut gameState_t) {
    syscall.expect("non-null function pointer")(CG_GETGAMESTATE as libc::c_int
                                                    as intptr_t, gamestate);
}
// cgame will poll each frame to see if a newer snapshot has arrived
// that it is interested in.  The time is returned separately so that
// snapshot latency can be calculated.
#[no_mangle]
pub unsafe extern "C" fn trap_GetCurrentSnapshotNumber(mut snapshotNumber:
                                                           *mut libc::c_int,
                                                       mut serverTime:
                                                           *mut libc::c_int) {
    syscall.expect("non-null function pointer")(CG_GETCURRENTSNAPSHOTNUMBER as
                                                    libc::c_int as intptr_t,
                                                snapshotNumber, serverTime);
}
// a snapshot get can fail if the snapshot (or the entties it holds) is so
// old that it has fallen out of the client system queue
#[no_mangle]
pub unsafe extern "C" fn trap_GetSnapshot(mut snapshotNumber: libc::c_int,
                                          mut snapshot: *mut snapshot_t)
 -> qboolean {
    return syscall.expect("non-null function pointer")(CG_GETSNAPSHOT as
                                                           libc::c_int as
                                                           intptr_t,
                                                       snapshotNumber,
                                                       snapshot) as qboolean;
}
// retrieve a text command from the server stream
// the current snapshot will hold the number of the most recent command
// qfalse can be returned if the client system handled the command
// argc() / argv() can be used to examine the parameters of the command
#[no_mangle]
pub unsafe extern "C" fn trap_GetServerCommand(mut serverCommandNumber:
                                                   libc::c_int) -> qboolean {
    return syscall.expect("non-null function pointer")(CG_GETSERVERCOMMAND as
                                                           libc::c_int as
                                                           intptr_t,
                                                       serverCommandNumber) as
               qboolean;
}
// returns the most recent command number that can be passed to GetUserCmd
// this will always be at least one higher than the number in the current
// snapshot, and it may be quite a few higher if it is a fast computer on
// a lagged connection
#[no_mangle]
pub unsafe extern "C" fn trap_GetCurrentCmdNumber() -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_GETCURRENTCMDNUMBER
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetUserCmd(mut cmdNumber: libc::c_int,
                                         mut ucmd: *mut usercmd_t)
 -> qboolean {
    return syscall.expect("non-null function pointer")(CG_GETUSERCMD as
                                                           libc::c_int as
                                                           intptr_t,
                                                       cmdNumber, ucmd) as
               qboolean;
}
// used for the weapon select and zoom
#[no_mangle]
pub unsafe extern "C" fn trap_SetUserCmdValue(mut stateValue: libc::c_int,
                                              mut sensitivityScale:
                                                  libc::c_float) {
    syscall.expect("non-null function pointer")(CG_SETUSERCMDVALUE as
                                                    libc::c_int as intptr_t,
                                                stateValue,
                                                PASSFLOAT(sensitivityScale));
}
// aids for VM testing
#[no_mangle]
pub unsafe extern "C" fn testPrintInt(mut string: *mut libc::c_char,
                                      mut i: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_TESTPRINTINT as libc::c_int
                                                    as intptr_t, string, i);
}
#[no_mangle]
pub unsafe extern "C" fn testPrintFloat(mut string: *mut libc::c_char,
                                        mut f: libc::c_float) {
    syscall.expect("non-null function pointer")(CG_TESTPRINTFLOAT as
                                                    libc::c_int as intptr_t,
                                                string, PASSFLOAT(f));
}
#[no_mangle]
pub unsafe extern "C" fn trap_MemoryRemaining() -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_MEMORY_REMAINING as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterFont(mut fontName:
                                                 *const libc::c_char,
                                             mut pointSize: libc::c_int,
                                             mut font: *mut fontInfo_t) {
    syscall.expect("non-null function pointer")(CG_R_REGISTERFONT as
                                                    libc::c_int as intptr_t,
                                                fontName, pointSize, font);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_IsDown(mut keynum: libc::c_int)
 -> qboolean {
    return syscall.expect("non-null function pointer")(CG_KEY_ISDOWN as
                                                           libc::c_int as
                                                           intptr_t, keynum)
               as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_GetCatcher() -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_KEY_GETCATCHER as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_SetCatcher(mut catcher: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_KEY_SETCATCHER as
                                                    libc::c_int as intptr_t,
                                                catcher);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_GetKey(mut binding: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_KEY_GETKEY as
                                                           libc::c_int as
                                                           intptr_t, binding)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_PlayCinematic(mut arg0: *const libc::c_char,
                                                mut xpos: libc::c_int,
                                                mut ypos: libc::c_int,
                                                mut width: libc::c_int,
                                                mut height: libc::c_int,
                                                mut bits: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_CIN_PLAYCINEMATIC as
                                                           libc::c_int as
                                                           intptr_t, arg0,
                                                       xpos, ypos, width,
                                                       height, bits) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_StopCinematic(mut handle: libc::c_int)
 -> e_status {
    return syscall.expect("non-null function pointer")(CG_CIN_STOPCINEMATIC as
                                                           libc::c_int as
                                                           intptr_t, handle)
               as e_status;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_RunCinematic(mut handle: libc::c_int)
 -> e_status {
    return syscall.expect("non-null function pointer")(CG_CIN_RUNCINEMATIC as
                                                           libc::c_int as
                                                           intptr_t, handle)
               as e_status;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_DrawCinematic(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_CIN_DRAWCINEMATIC as
                                                    libc::c_int as intptr_t,
                                                handle);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_SetExtents(mut handle: libc::c_int,
                                             mut x: libc::c_int,
                                             mut y: libc::c_int,
                                             mut w: libc::c_int,
                                             mut h: libc::c_int) {
    syscall.expect("non-null function pointer")(CG_CIN_SETEXTENTS as
                                                    libc::c_int as intptr_t,
                                                handle, x, y, w, h);
}
#[no_mangle]
pub unsafe extern "C" fn trap_RealTime(mut qtime: *mut qtime_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_REAL_TIME as
                                                           libc::c_int as
                                                           intptr_t, qtime) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_SnapVector(mut v: *mut libc::c_float) {
    syscall.expect("non-null function pointer")(CG_SNAPVECTOR as libc::c_int
                                                    as intptr_t, v);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetEntityToken(mut buffer: *mut libc::c_char,
                                             mut bufferSize: libc::c_int)
 -> qboolean {
    return syscall.expect("non-null function pointer")(CG_GET_ENTITY_TOKEN as
                                                           libc::c_int as
                                                           intptr_t, buffer,
                                                       bufferSize) as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn dllEntry(mut syscallptr:
                                      Option<unsafe extern "C" fn(_:
                                                                      intptr_t, ...)
                                                 -> intptr_t>) {
    syscall = syscallptr;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_TempCapsuleModel(mut mins: *const vec_t,
                                                  mut maxs: *const vec_t)
 -> clipHandle_t {
    return syscall.expect("non-null function pointer")(CG_CM_TEMPCAPSULEMODEL
                                                           as libc::c_int as
                                                           intptr_t, mins,
                                                       maxs) as clipHandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_AddGlobalDefine(mut define:
                                                     *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_PC_ADD_GLOBAL_DEFINE
                                                           as libc::c_int as
                                                           intptr_t, define)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_PC_LOAD_SOURCE as
                                                           libc::c_int as
                                                           intptr_t, filename)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_PC_FREE_SOURCE as
                                                           libc::c_int as
                                                           intptr_t, handle)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_ReadToken(mut handle: libc::c_int,
                                           mut pc_token: *mut pc_token_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_PC_READ_TOKEN as
                                                           libc::c_int as
                                                           intptr_t, handle,
                                                       pc_token) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_SourceFileAndLine(mut handle: libc::c_int,
                                                   mut filename:
                                                       *mut libc::c_char,
                                                   mut line: *mut libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(CG_PC_SOURCE_FILE_AND_LINE
                                                           as libc::c_int as
                                                           intptr_t, handle,
                                                       filename, line) as
               libc::c_int;
}