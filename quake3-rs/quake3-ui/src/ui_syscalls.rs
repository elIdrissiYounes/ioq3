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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct orientation_t {
        pub origin: vec3_t,
        pub axis: [vec3_t; 3],
    }
    pub type connstate_t = libc::c_uint;
    // playing a cinematic or a static pic, not connected to a server
    pub const CA_CINEMATIC: connstate_t = 9;
    // game views should be displayed
    pub const CA_ACTIVE: connstate_t = 8;
    // got gamestate, waiting for first frame
    pub const CA_PRIMED: connstate_t = 7;
    // only during cgame initialization, never during main loop
    pub const CA_LOADING: connstate_t = 6;
    // netchan_t established, getting gamestate
    pub const CA_CONNECTED: connstate_t = 5;
    // sending challenge packets to the server
    pub const CA_CHALLENGING: connstate_t = 4;
    // sending request packets to the server
    pub const CA_CONNECTING: connstate_t = 3;
    // not used any more, was checking cd key 
    pub const CA_AUTHORIZING: connstate_t = 2;
    // not talking to a server
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CA_UNINITIALIZED: connstate_t = 0;
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_public.h"]
pub mod ui_public_h {
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct uiClientState_t {
        pub connState: connstate_t,
        pub connectPacketCount: libc::c_int,
        pub clientNum: libc::c_int,
        pub servername: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub messageString: [libc::c_char; 1024],
    }
    pub type unnamed = libc::c_uint;
    pub const UI_CEIL: unnamed = 108;
    pub const UI_FLOOR: unnamed = 107;
    pub const UI_SQRT: unnamed = 106;
    pub const UI_ATAN2: unnamed = 105;
    pub const UI_COS: unnamed = 104;
    pub const UI_SIN: unnamed = 103;
    pub const UI_STRNCPY: unnamed = 102;
    pub const UI_MEMCPY: unnamed = 101;
    pub const UI_MEMSET: unnamed = 100;
    pub const UI_SET_PBCLSTATUS: unnamed = 87;
    // 1.32
    pub const UI_FS_SEEK: unnamed = 86;
    pub const UI_LAN_COMPARESERVERS: unnamed = 85;
    pub const UI_LAN_SERVERISVISIBLE: unnamed = 84;
    pub const UI_LAN_GETSERVERPING: unnamed = 83;
    pub const UI_LAN_SERVERSTATUS: unnamed = 82;
    pub const UI_VERIFY_CDKEY: unnamed = 81;
    pub const UI_R_REMAP_SHADER: unnamed = 80;
    pub const UI_CIN_SETEXTENTS: unnamed = 79;
    pub const UI_CIN_DRAWCINEMATIC: unnamed = 78;
    pub const UI_CIN_RUNCINEMATIC: unnamed = 77;
    pub const UI_CIN_STOPCINEMATIC: unnamed = 76;
    pub const UI_CIN_PLAYCINEMATIC: unnamed = 75;
    pub const UI_LAN_REMOVESERVER: unnamed = 74;
    pub const UI_LAN_ADDSERVER: unnamed = 73;
    pub const UI_LAN_SAVECACHEDSERVERS: unnamed = 72;
    pub const UI_LAN_LOADCACHEDSERVERS: unnamed = 71;
    pub const UI_LAN_RESETPINGS: unnamed = 70;
    pub const UI_LAN_UPDATEVISIBLEPINGS: unnamed = 69;
    pub const UI_LAN_MARKSERVERVISIBLE: unnamed = 68;
    pub const UI_LAN_GETSERVERINFO: unnamed = 67;
    pub const UI_LAN_GETSERVERADDRESSSTRING: unnamed = 66;
    pub const UI_LAN_GETSERVERCOUNT: unnamed = 65;
    pub const UI_REAL_TIME: unnamed = 64;
    pub const UI_S_STARTBACKGROUNDTRACK: unnamed = 63;
    pub const UI_S_STOPBACKGROUNDTRACK: unnamed = 62;
    pub const UI_PC_SOURCE_FILE_AND_LINE: unnamed = 61;
    pub const UI_PC_READ_TOKEN: unnamed = 60;
    pub const UI_PC_FREE_SOURCE: unnamed = 59;
    pub const UI_PC_LOAD_SOURCE: unnamed = 58;
    pub const UI_PC_ADD_GLOBAL_DEFINE: unnamed = 57;
    pub const UI_R_MODELBOUNDS: unnamed = 56;
    pub const UI_R_REGISTERFONT: unnamed = 55;
    pub const UI_SET_CDKEY: unnamed = 54;
    pub const UI_GET_CDKEY: unnamed = 53;
    pub const UI_MEMORY_REMAINING: unnamed = 52;
    pub const UI_CVAR_UPDATE: unnamed = 51;
    pub const UI_CVAR_REGISTER: unnamed = 50;
    pub const UI_LAN_GETPINGINFO: unnamed = 49;
    pub const UI_LAN_GETPING: unnamed = 48;
    pub const UI_LAN_CLEARPING: unnamed = 47;
    pub const UI_LAN_GETPINGQUEUECOUNT: unnamed = 46;
    pub const UI_GETCONFIGSTRING: unnamed = 45;
    pub const UI_GETCLIENTSTATE: unnamed = 44;
    pub const UI_GETGLCONFIG: unnamed = 43;
    pub const UI_GETCLIPBOARDDATA: unnamed = 42;
    pub const UI_KEY_SETCATCHER: unnamed = 41;
    pub const UI_KEY_GETCATCHER: unnamed = 40;
    pub const UI_KEY_CLEARSTATES: unnamed = 39;
    pub const UI_KEY_SETOVERSTRIKEMODE: unnamed = 38;
    pub const UI_KEY_GETOVERSTRIKEMODE: unnamed = 37;
    pub const UI_KEY_ISDOWN: unnamed = 36;
    pub const UI_KEY_SETBINDING: unnamed = 35;
    pub const UI_KEY_GETBINDINGBUF: unnamed = 34;
    pub const UI_KEY_KEYNUMTOSTRINGBUF: unnamed = 33;
    pub const UI_S_STARTLOCALSOUND: unnamed = 32;
    pub const UI_S_REGISTERSOUND: unnamed = 31;
    pub const UI_CM_LOADMODEL: unnamed = 30;
    pub const UI_CM_LERPTAG: unnamed = 29;
    pub const UI_UPDATESCREEN: unnamed = 28;
    pub const UI_R_DRAWSTRETCHPIC: unnamed = 27;
    pub const UI_R_SETCOLOR: unnamed = 26;
    pub const UI_R_RENDERSCENE: unnamed = 25;
    pub const UI_R_ADDLIGHTTOSCENE: unnamed = 24;
    pub const UI_R_ADDPOLYTOSCENE: unnamed = 23;
    pub const UI_R_ADDREFENTITYTOSCENE: unnamed = 22;
    pub const UI_R_CLEARSCENE: unnamed = 21;
    pub const UI_R_REGISTERSHADERNOMIP: unnamed = 20;
    pub const UI_R_REGISTERSKIN: unnamed = 19;
    pub const UI_R_REGISTERMODEL: unnamed = 18;
    pub const UI_FS_GETFILELIST: unnamed = 17;
    pub const UI_FS_FCLOSEFILE: unnamed = 16;
    pub const UI_FS_WRITE: unnamed = 15;
    pub const UI_FS_READ: unnamed = 14;
    pub const UI_FS_FOPENFILE: unnamed = 13;
    pub const UI_CMD_EXECUTETEXT: unnamed = 12;
    pub const UI_ARGV: unnamed = 11;
    pub const UI_ARGC: unnamed = 10;
    pub const UI_CVAR_INFOSTRINGBUFFER: unnamed = 9;
    pub const UI_CVAR_CREATE: unnamed = 8;
    pub const UI_CVAR_RESET: unnamed = 7;
    pub const UI_CVAR_SETVALUE: unnamed = 6;
    pub const UI_CVAR_VARIABLESTRINGBUFFER: unnamed = 5;
    pub const UI_CVAR_VARIABLEVALUE: unnamed = 4;
    pub const UI_CVAR_SET: unnamed = 3;
    pub const UI_MILLISECONDS: unnamed = 2;
    pub const UI_PRINT: unnamed = 1;
    pub const UI_ERROR: unnamed = 0;
    use super::q_shared_h::{connstate_t};
    use super::{libc};
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_shared.h"]
pub mod ui_shared_h {
    use super::{libc};
    use super::q_shared_h::{pc_token_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_syscalls.c"]
pub mod ui_syscalls_c {
    use super::{libc};
    use super::stdint_h::{intptr_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_local.h"]
pub mod ui_local_h {
    use super::{libc};
    use super::q_shared_h::{vmCvar_t, fileHandle_t, fsMode_t, qhandle_t,
                            vec_t, clipHandle_t, orientation_t, sfxHandle_t,
                            qboolean, fontInfo_t, e_status, qtime_t};
    use super::tr_types_h::{refEntity_t, polyVert_t, refdef_t, glconfig_t};
    use super::ui_public_h::{uiClientState_t};
}
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t, qhandle_t,
                       sfxHandle_t, fileHandle_t, clipHandle_t, vec_t, vec3_t,
                       pc_token_s, pc_token_t, fsMode_t, FS_APPEND_SYNC,
                       FS_APPEND, FS_WRITE, FS_READ, cvarHandle_t, vmCvar_t,
                       orientation_t, connstate_t, CA_CINEMATIC, CA_ACTIVE,
                       CA_PRIMED, CA_LOADING, CA_CONNECTED, CA_CHALLENGING,
                       CA_CONNECTING, CA_AUTHORIZING, CA_DISCONNECTED,
                       CA_UNINITIALIZED, glyphInfo_t, fontInfo_t, qtime_s,
                       qtime_t, e_status, FMV_ID_WAIT, FMV_LOOPED,
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
use self::ui_public_h::{uiClientState_t, unnamed, UI_CEIL, UI_FLOOR, UI_SQRT,
                        UI_ATAN2, UI_COS, UI_SIN, UI_STRNCPY, UI_MEMCPY,
                        UI_MEMSET, UI_SET_PBCLSTATUS, UI_FS_SEEK,
                        UI_LAN_COMPARESERVERS, UI_LAN_SERVERISVISIBLE,
                        UI_LAN_GETSERVERPING, UI_LAN_SERVERSTATUS,
                        UI_VERIFY_CDKEY, UI_R_REMAP_SHADER, UI_CIN_SETEXTENTS,
                        UI_CIN_DRAWCINEMATIC, UI_CIN_RUNCINEMATIC,
                        UI_CIN_STOPCINEMATIC, UI_CIN_PLAYCINEMATIC,
                        UI_LAN_REMOVESERVER, UI_LAN_ADDSERVER,
                        UI_LAN_SAVECACHEDSERVERS, UI_LAN_LOADCACHEDSERVERS,
                        UI_LAN_RESETPINGS, UI_LAN_UPDATEVISIBLEPINGS,
                        UI_LAN_MARKSERVERVISIBLE, UI_LAN_GETSERVERINFO,
                        UI_LAN_GETSERVERADDRESSSTRING, UI_LAN_GETSERVERCOUNT,
                        UI_REAL_TIME, UI_S_STARTBACKGROUNDTRACK,
                        UI_S_STOPBACKGROUNDTRACK, UI_PC_SOURCE_FILE_AND_LINE,
                        UI_PC_READ_TOKEN, UI_PC_FREE_SOURCE,
                        UI_PC_LOAD_SOURCE, UI_PC_ADD_GLOBAL_DEFINE,
                        UI_R_MODELBOUNDS, UI_R_REGISTERFONT, UI_SET_CDKEY,
                        UI_GET_CDKEY, UI_MEMORY_REMAINING, UI_CVAR_UPDATE,
                        UI_CVAR_REGISTER, UI_LAN_GETPINGINFO, UI_LAN_GETPING,
                        UI_LAN_CLEARPING, UI_LAN_GETPINGQUEUECOUNT,
                        UI_GETCONFIGSTRING, UI_GETCLIENTSTATE, UI_GETGLCONFIG,
                        UI_GETCLIPBOARDDATA, UI_KEY_SETCATCHER,
                        UI_KEY_GETCATCHER, UI_KEY_CLEARSTATES,
                        UI_KEY_SETOVERSTRIKEMODE, UI_KEY_GETOVERSTRIKEMODE,
                        UI_KEY_ISDOWN, UI_KEY_SETBINDING,
                        UI_KEY_GETBINDINGBUF, UI_KEY_KEYNUMTOSTRINGBUF,
                        UI_S_STARTLOCALSOUND, UI_S_REGISTERSOUND,
                        UI_CM_LOADMODEL, UI_CM_LERPTAG, UI_UPDATESCREEN,
                        UI_R_DRAWSTRETCHPIC, UI_R_SETCOLOR, UI_R_RENDERSCENE,
                        UI_R_ADDLIGHTTOSCENE, UI_R_ADDPOLYTOSCENE,
                        UI_R_ADDREFENTITYTOSCENE, UI_R_CLEARSCENE,
                        UI_R_REGISTERSHADERNOMIP, UI_R_REGISTERSKIN,
                        UI_R_REGISTERMODEL, UI_FS_GETFILELIST,
                        UI_FS_FCLOSEFILE, UI_FS_WRITE, UI_FS_READ,
                        UI_FS_FOPENFILE, UI_CMD_EXECUTETEXT, UI_ARGV, UI_ARGC,
                        UI_CVAR_INFOSTRINGBUFFER, UI_CVAR_CREATE,
                        UI_CVAR_RESET, UI_CVAR_SETVALUE,
                        UI_CVAR_VARIABLESTRINGBUFFER, UI_CVAR_VARIABLEVALUE,
                        UI_CVAR_SET, UI_MILLISECONDS, UI_PRINT, UI_ERROR};
use self::stdlib_h::{exit};
#[no_mangle]
pub unsafe extern "C" fn trap_PC_AddGlobalDefine(mut define:
                                                     *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_PC_ADD_GLOBAL_DEFINE
                                                           as libc::c_int as
                                                           intptr_t, define)
               as libc::c_int;
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
// this file is only included when building a dll
// syscalls.asm is included instead when building a qvm
static mut syscall: Option<unsafe extern "C" fn(_: intptr_t, ...) -> intptr_t>
       =
    ::std::mem::transmute::<libc::intptr_t,
                            Option<unsafe extern "C" fn(_: intptr_t, ...)
                                       -> intptr_t>>(-1i32 as libc::intptr_t);
#[no_mangle]
pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_PC_LOAD_SOURCE as
                                                           libc::c_int as
                                                           intptr_t, filename)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_PC_FREE_SOURCE as
                                                           libc::c_int as
                                                           intptr_t, handle)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_ReadToken(mut handle: libc::c_int,
                                           mut pc_token: *mut pc_token_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_PC_READ_TOKEN as
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
    return syscall.expect("non-null function pointer")(UI_PC_SOURCE_FILE_AND_LINE
                                                           as libc::c_int as
                                                           intptr_t, handle,
                                                       filename, line) as
               libc::c_int;
}
//
// ui_syscalls.c
//
#[no_mangle]
pub unsafe extern "C" fn trap_Print(mut string: *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_PRINT as libc::c_int as
                                                    intptr_t, string);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Error(mut string: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(UI_ERROR as libc::c_int as
                                                    intptr_t, string);
    exit(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Milliseconds() -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_MILLISECONDS as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Register(mut cvar: *mut vmCvar_t,
                                            mut var_name: *const libc::c_char,
                                            mut value: *const libc::c_char,
                                            mut flags: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_CVAR_REGISTER as
                                                    libc::c_int as intptr_t,
                                                cvar, var_name, value, flags);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Update(mut cvar: *mut vmCvar_t) {
    syscall.expect("non-null function pointer")(UI_CVAR_UPDATE as libc::c_int
                                                    as intptr_t, cvar);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Set(mut var_name: *const libc::c_char,
                                       mut value: *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_CVAR_SET as libc::c_int as
                                                    intptr_t, var_name,
                                                value);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_VariableValue(mut var_name:
                                                     *const libc::c_char)
 -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i =
        syscall.expect("non-null function pointer")(UI_CVAR_VARIABLEVALUE as
                                                        libc::c_int as
                                                        intptr_t, var_name) as
            libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(mut var_name:
                                                            *const libc::c_char,
                                                        mut buffer:
                                                            *mut libc::c_char,
                                                        mut bufsize:
                                                            libc::c_int) {
    syscall.expect("non-null function pointer")(UI_CVAR_VARIABLESTRINGBUFFER
                                                    as libc::c_int as
                                                    intptr_t, var_name,
                                                buffer, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_SetValue(mut var_name: *const libc::c_char,
                                            mut value: libc::c_float) {
    syscall.expect("non-null function pointer")(UI_CVAR_SETVALUE as
                                                    libc::c_int as intptr_t,
                                                var_name, PASSFLOAT(value));
}
#[no_mangle]
pub unsafe extern "C" fn PASSFLOAT(mut x: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.f = x;
    return fi.i;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Reset(mut name: *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_CVAR_RESET as libc::c_int
                                                    as intptr_t, name);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Create(mut var_name: *const libc::c_char,
                                          mut var_value: *const libc::c_char,
                                          mut flags: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_CVAR_CREATE as libc::c_int
                                                    as intptr_t, var_name,
                                                var_value, flags);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_InfoStringBuffer(mut bit: libc::c_int,
                                                    mut buffer:
                                                        *mut libc::c_char,
                                                    mut bufsize:
                                                        libc::c_int) {
    syscall.expect("non-null function pointer")(UI_CVAR_INFOSTRINGBUFFER as
                                                    libc::c_int as intptr_t,
                                                bit, buffer, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Argc() -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_ARGC as libc::c_int
                                                           as intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Argv(mut n: libc::c_int,
                                   mut buffer: *mut libc::c_char,
                                   mut bufferLength: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_ARGV as libc::c_int as
                                                    intptr_t, n, buffer,
                                                bufferLength);
}
// don't use EXEC_NOW!
#[no_mangle]
pub unsafe extern "C" fn trap_Cmd_ExecuteText(mut exec_when: libc::c_int,
                                              mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_CMD_EXECUTETEXT as
                                                    libc::c_int as intptr_t,
                                                exec_when, text);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_FOpenFile(mut qpath: *const libc::c_char,
                                           mut f: *mut fileHandle_t,
                                           mut mode: fsMode_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_FS_FOPENFILE as
                                                           libc::c_int as
                                                           intptr_t, qpath, f,
                                                       mode as libc::c_uint)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Read(mut buffer: *mut libc::c_void,
                                      mut len: libc::c_int,
                                      mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(UI_FS_READ as libc::c_int as
                                                    intptr_t, buffer, len, f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Write(mut buffer: *const libc::c_void,
                                       mut len: libc::c_int,
                                       mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(UI_FS_WRITE as libc::c_int as
                                                    intptr_t, buffer, len, f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(UI_FS_FCLOSEFILE as
                                                    libc::c_int as intptr_t,
                                                f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_GetFileList(mut path: *const libc::c_char,
                                             mut extension:
                                                 *const libc::c_char,
                                             mut listbuf: *mut libc::c_char,
                                             mut bufsize: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_FS_GETFILELIST as
                                                           libc::c_int as
                                                           intptr_t, path,
                                                       extension, listbuf,
                                                       bufsize) as
               libc::c_int;
}
// fsOrigin_t
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Seek(mut f: fileHandle_t,
                                      mut offset: libc::c_long,
                                      mut origin: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_FS_SEEK as
                                                           libc::c_int as
                                                           intptr_t, f,
                                                       offset, origin) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterModel(mut name: *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(UI_R_REGISTERMODEL as
                                                           libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterSkin(mut name: *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(UI_R_REGISTERSKIN as
                                                           libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterShaderNoMip(mut name:
                                                        *const libc::c_char)
 -> qhandle_t {
    return syscall.expect("non-null function pointer")(UI_R_REGISTERSHADERNOMIP
                                                           as libc::c_int as
                                                           intptr_t, name) as
               qhandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_ClearScene() {
    syscall.expect("non-null function pointer")(UI_R_CLEARSCENE as libc::c_int
                                                    as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddRefEntityToScene(mut re:
                                                        *const refEntity_t) {
    syscall.expect("non-null function pointer")(UI_R_ADDREFENTITYTOSCENE as
                                                    libc::c_int as intptr_t,
                                                re);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddPolyToScene(mut hShader: qhandle_t,
                                               mut numVerts: libc::c_int,
                                               mut verts: *const polyVert_t) {
    syscall.expect("non-null function pointer")(UI_R_ADDPOLYTOSCENE as
                                                    libc::c_int as intptr_t,
                                                hShader, numVerts, verts);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_AddLightToScene(mut org: *const vec_t,
                                                mut intensity: libc::c_float,
                                                mut r: libc::c_float,
                                                mut g: libc::c_float,
                                                mut b: libc::c_float) {
    syscall.expect("non-null function pointer")(UI_R_ADDLIGHTTOSCENE as
                                                    libc::c_int as intptr_t,
                                                org, PASSFLOAT(intensity),
                                                PASSFLOAT(r), PASSFLOAT(g),
                                                PASSFLOAT(b));
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RenderScene(mut fd: *const refdef_t) {
    syscall.expect("non-null function pointer")(UI_R_RENDERSCENE as
                                                    libc::c_int as intptr_t,
                                                fd);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_SetColor(mut rgba: *const libc::c_float) {
    syscall.expect("non-null function pointer")(UI_R_SETCOLOR as libc::c_int
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
    syscall.expect("non-null function pointer")(UI_R_DRAWSTRETCHPIC as
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
    syscall.expect("non-null function pointer")(UI_R_MODELBOUNDS as
                                                    libc::c_int as intptr_t,
                                                model, mins, maxs);
}
#[no_mangle]
pub unsafe extern "C" fn trap_UpdateScreen() {
    syscall.expect("non-null function pointer")(UI_UPDATESCREEN as libc::c_int
                                                    as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CM_LerpTag(mut tag: *mut orientation_t,
                                         mut mod_0: clipHandle_t,
                                         mut startFrame: libc::c_int,
                                         mut endFrame: libc::c_int,
                                         mut frac: libc::c_float,
                                         mut tagName: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_CM_LERPTAG as
                                                           libc::c_int as
                                                           intptr_t, tag,
                                                       mod_0, startFrame,
                                                       endFrame,
                                                       PASSFLOAT(frac),
                                                       tagName) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_StartLocalSound(mut sfx: sfxHandle_t,
                                                mut channelNum: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_S_STARTLOCALSOUND as
                                                    libc::c_int as intptr_t,
                                                sfx, channelNum);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_RegisterSound(mut sample: *const libc::c_char,
                                              mut compressed: qboolean)
 -> sfxHandle_t {
    return syscall.expect("non-null function pointer")(UI_S_REGISTERSOUND as
                                                           libc::c_int as
                                                           intptr_t, sample,
                                                       compressed as
                                                           libc::c_uint) as
               sfxHandle_t;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_KeynumToStringBuf(mut keynum: libc::c_int,
                                                    mut buf:
                                                        *mut libc::c_char,
                                                    mut buflen: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_KEY_KEYNUMTOSTRINGBUF as
                                                    libc::c_int as intptr_t,
                                                keynum, buf, buflen);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_GetBindingBuf(mut keynum: libc::c_int,
                                                mut buf: *mut libc::c_char,
                                                mut buflen: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_KEY_GETBINDINGBUF as
                                                    libc::c_int as intptr_t,
                                                keynum, buf, buflen);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_SetBinding(mut keynum: libc::c_int,
                                             mut binding:
                                                 *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_KEY_SETBINDING as
                                                    libc::c_int as intptr_t,
                                                keynum, binding);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_IsDown(mut keynum: libc::c_int)
 -> qboolean {
    return syscall.expect("non-null function pointer")(UI_KEY_ISDOWN as
                                                           libc::c_int as
                                                           intptr_t, keynum)
               as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_GetOverstrikeMode() -> qboolean {
    return syscall.expect("non-null function pointer")(UI_KEY_GETOVERSTRIKEMODE
                                                           as libc::c_int as
                                                           intptr_t) as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_SetOverstrikeMode(mut state: qboolean) {
    syscall.expect("non-null function pointer")(UI_KEY_SETOVERSTRIKEMODE as
                                                    libc::c_int as intptr_t,
                                                state as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_ClearStates() {
    syscall.expect("non-null function pointer")(UI_KEY_CLEARSTATES as
                                                    libc::c_int as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_GetCatcher() -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_KEY_GETCATCHER as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Key_SetCatcher(mut catcher: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_KEY_SETCATCHER as
                                                    libc::c_int as intptr_t,
                                                catcher);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetClipboardData(mut buf: *mut libc::c_char,
                                               mut bufsize: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_GETCLIPBOARDDATA as
                                                    libc::c_int as intptr_t,
                                                buf, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetClientState(mut state:
                                                 *mut uiClientState_t) {
    syscall.expect("non-null function pointer")(UI_GETCLIENTSTATE as
                                                    libc::c_int as intptr_t,
                                                state);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetGlconfig(mut glconfig: *mut glconfig_t) {
    syscall.expect("non-null function pointer")(UI_GETGLCONFIG as libc::c_int
                                                    as intptr_t, glconfig);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetConfigString(mut index: libc::c_int,
                                              mut buff: *mut libc::c_char,
                                              mut buffsize: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_GETCONFIGSTRING as
                                                           libc::c_int as
                                                           intptr_t, index,
                                                       buff, buffsize) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetServerCount(mut source: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_GETSERVERCOUNT
                                                           as libc::c_int as
                                                           intptr_t, source)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetServerAddressString(mut source:
                                                             libc::c_int,
                                                         mut n: libc::c_int,
                                                         mut buf:
                                                             *mut libc::c_char,
                                                         mut buflen:
                                                             libc::c_int) {
    syscall.expect("non-null function pointer")(UI_LAN_GETSERVERADDRESSSTRING
                                                    as libc::c_int as
                                                    intptr_t, source, n, buf,
                                                buflen);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetServerInfo(mut source: libc::c_int,
                                                mut n: libc::c_int,
                                                mut buf: *mut libc::c_char,
                                                mut buflen: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_LAN_GETSERVERINFO as
                                                    libc::c_int as intptr_t,
                                                source, n, buf, buflen);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetServerPing(mut source: libc::c_int,
                                                mut n: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_GETSERVERPING as
                                                           libc::c_int as
                                                           intptr_t, source,
                                                       n) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetPingQueueCount() -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_GETPINGQUEUECOUNT
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_ClearPing(mut n: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_LAN_CLEARPING as
                                                    libc::c_int as intptr_t,
                                                n);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetPing(mut n: libc::c_int,
                                          mut buf: *mut libc::c_char,
                                          mut buflen: libc::c_int,
                                          mut pingtime: *mut libc::c_int) {
    syscall.expect("non-null function pointer")(UI_LAN_GETPING as libc::c_int
                                                    as intptr_t, n, buf,
                                                buflen, pingtime);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_GetPingInfo(mut n: libc::c_int,
                                              mut buf: *mut libc::c_char,
                                              mut buflen: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_LAN_GETPINGINFO as
                                                    libc::c_int as intptr_t,
                                                n, buf, buflen);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_LoadCachedServers() {
    syscall.expect("non-null function pointer")(UI_LAN_LOADCACHEDSERVERS as
                                                    libc::c_int as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_SaveCachedServers() {
    syscall.expect("non-null function pointer")(UI_LAN_SAVECACHEDSERVERS as
                                                    libc::c_int as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_MarkServerVisible(mut source: libc::c_int,
                                                    mut n: libc::c_int,
                                                    mut visible: qboolean) {
    syscall.expect("non-null function pointer")(UI_LAN_MARKSERVERVISIBLE as
                                                    libc::c_int as intptr_t,
                                                source, n,
                                                visible as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_ServerIsVisible(mut source: libc::c_int,
                                                  mut n: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_SERVERISVISIBLE
                                                           as libc::c_int as
                                                           intptr_t, source,
                                                       n) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_UpdateVisiblePings(mut source: libc::c_int)
 -> qboolean {
    return syscall.expect("non-null function pointer")(UI_LAN_UPDATEVISIBLEPINGS
                                                           as libc::c_int as
                                                           intptr_t, source)
               as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_AddServer(mut source: libc::c_int,
                                            mut name: *const libc::c_char,
                                            mut addr: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_ADDSERVER as
                                                           libc::c_int as
                                                           intptr_t, source,
                                                       name, addr) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_RemoveServer(mut source: libc::c_int,
                                               mut addr:
                                                   *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_LAN_REMOVESERVER as
                                                    libc::c_int as intptr_t,
                                                source, addr);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_ResetPings(mut n: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_LAN_RESETPINGS as
                                                    libc::c_int as intptr_t,
                                                n);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_ServerStatus(mut serverAddress:
                                                   *const libc::c_char,
                                               mut serverStatus:
                                                   *mut libc::c_char,
                                               mut maxLen: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_SERVERSTATUS as
                                                           libc::c_int as
                                                           intptr_t,
                                                       serverAddress,
                                                       serverStatus, maxLen)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LAN_CompareServers(mut source: libc::c_int,
                                                 mut sortKey: libc::c_int,
                                                 mut sortDir: libc::c_int,
                                                 mut s1: libc::c_int,
                                                 mut s2: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_LAN_COMPARESERVERS
                                                           as libc::c_int as
                                                           intptr_t, source,
                                                       sortKey, sortDir, s1,
                                                       s2) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_MemoryRemaining() -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_MEMORY_REMAINING as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetCDKey(mut buf: *mut libc::c_char,
                                       mut buflen: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_GET_CDKEY as libc::c_int as
                                                    intptr_t, buf, buflen);
}
#[no_mangle]
pub unsafe extern "C" fn trap_SetCDKey(mut buf: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(UI_SET_CDKEY as libc::c_int as
                                                    intptr_t, buf);
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RegisterFont(mut fontName:
                                                 *const libc::c_char,
                                             mut pointSize: libc::c_int,
                                             mut font: *mut fontInfo_t) {
    syscall.expect("non-null function pointer")(UI_R_REGISTERFONT as
                                                    libc::c_int as intptr_t,
                                                fontName, pointSize, font);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_StopBackgroundTrack() {
    syscall.expect("non-null function pointer")(UI_S_STOPBACKGROUNDTRACK as
                                                    libc::c_int as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_S_StartBackgroundTrack(mut intro:
                                                         *const libc::c_char,
                                                     mut loop_0:
                                                         *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_S_STARTBACKGROUNDTRACK as
                                                    libc::c_int as intptr_t,
                                                intro, loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_PlayCinematic(mut arg0: *const libc::c_char,
                                                mut xpos: libc::c_int,
                                                mut ypos: libc::c_int,
                                                mut width: libc::c_int,
                                                mut height: libc::c_int,
                                                mut bits: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_CIN_PLAYCINEMATIC as
                                                           libc::c_int as
                                                           intptr_t, arg0,
                                                       xpos, ypos, width,
                                                       height, bits) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_StopCinematic(mut handle: libc::c_int)
 -> e_status {
    return syscall.expect("non-null function pointer")(UI_CIN_STOPCINEMATIC as
                                                           libc::c_int as
                                                           intptr_t, handle)
               as e_status;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_RunCinematic(mut handle: libc::c_int)
 -> e_status {
    return syscall.expect("non-null function pointer")(UI_CIN_RUNCINEMATIC as
                                                           libc::c_int as
                                                           intptr_t, handle)
               as e_status;
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_DrawCinematic(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_CIN_DRAWCINEMATIC as
                                                    libc::c_int as intptr_t,
                                                handle);
}
#[no_mangle]
pub unsafe extern "C" fn trap_CIN_SetExtents(mut handle: libc::c_int,
                                             mut x: libc::c_int,
                                             mut y: libc::c_int,
                                             mut w: libc::c_int,
                                             mut h: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_CIN_SETEXTENTS as
                                                    libc::c_int as intptr_t,
                                                handle, x, y, w, h);
}
#[no_mangle]
pub unsafe extern "C" fn trap_RealTime(mut qtime: *mut qtime_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(UI_REAL_TIME as
                                                           libc::c_int as
                                                           intptr_t, qtime) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_R_RemapShader(mut oldShader:
                                                *const libc::c_char,
                                            mut newShader:
                                                *const libc::c_char,
                                            mut timeOffset:
                                                *const libc::c_char) {
    syscall.expect("non-null function pointer")(UI_R_REMAP_SHADER as
                                                    libc::c_int as intptr_t,
                                                oldShader, newShader,
                                                timeOffset);
}
#[no_mangle]
pub unsafe extern "C" fn trap_VerifyCDKey(mut key: *const libc::c_char,
                                          mut chksum: *const libc::c_char)
 -> qboolean {
    return syscall.expect("non-null function pointer")(UI_VERIFY_CDKEY as
                                                           libc::c_int as
                                                           intptr_t, key,
                                                       chksum) as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_SetPbClStatus(mut status: libc::c_int) {
    syscall.expect("non-null function pointer")(UI_SET_PBCLSTATUS as
                                                    libc::c_int as intptr_t,
                                                status);
}
#[no_mangle]
pub unsafe extern "C" fn dllEntry(mut syscallptr:
                                      Option<unsafe extern "C" fn(_:
                                                                      intptr_t, ...)
                                                 -> intptr_t>) {
    syscall = syscallptr;
}