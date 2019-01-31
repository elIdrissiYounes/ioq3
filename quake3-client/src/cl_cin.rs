use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
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
    pub type qhandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    // expand constants before stringifying them
    // angle indexes
    // up / down
    // left / right
    // fall over
    // the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
    // max length of a string passed to Cmd_TokenizeString
    // max tokens resulting from Cmd_TokenizeString
    // max length of an individual token
    // used for system info key only
    // max length of a quake game pathname
    // max length of a client name
    // parameters for command buffer stuffing
    pub type unnamed = libc::c_uint;
    // add to end of the command buffer (normal case)
    pub const EXEC_APPEND: unnamed = 2;
    // because some commands might cause the VM to be unloaded...
    // insert at current position, but don't run yet
    pub const EXEC_INSERT: unnamed = 1;
    // don't return until completed, a VM should NEVER use this,
    pub const EXEC_NOW: unnamed = 0;
    // parameters to the main Error routine
    pub type unnamed_0 = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed_0 = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed_0 = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed_0 = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed_0 = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed_0 = 0;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
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
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        //=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    //============================================================================
    /*
==============================================================

NET

==============================================================
*/
    // if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
    // disables ipv6 multicast support if set.
    // number of old messages that must be kept on client and
    // server for delta comrpession and ping estimation
    // max number of usercmd_t in a packet
    // max string commands buffered for restransmit
    pub type netadrtype_t = libc::c_uint;
    pub const NA_UNSPEC: netadrtype_t = 7;
    pub const NA_MULTICAST6: netadrtype_t = 6;
    pub const NA_IP6: netadrtype_t = 5;
    pub const NA_IP: netadrtype_t = 4;
    pub const NA_BROADCAST: netadrtype_t = 3;
    pub const NA_LOOPBACK: netadrtype_t = 2;
    pub const NA_BOT: netadrtype_t = 1;
    // an address lookup failed
    pub const NA_BAD: netadrtype_t = 0;
    pub type netsrc_t = libc::c_uint;
    pub const NS_SERVER: netsrc_t = 1;
    pub const NS_CLIENT: netsrc_t = 0;
    // maximum length of an IPv6 address string including trailing '\0'
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netadr_t {
        pub type_0: netadrtype_t,
        pub ip: [byte; 4],
        pub ip6: [byte; 16],
        pub port: libc::c_ushort,
        pub scope_id: libc::c_ulong,
    }
    // max length of a message, which may
    // be fragmented into multiple packets
    // ACK window of 48 download chunks. Cannot set this higher, or clients
    // will overflow the reliable commands buffer
    // 896 byte block chunks
    /*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_t {
        pub sock: netsrc_t,
        pub dropped: libc::c_int,
        pub remoteAddress: netadr_t,
        pub qport: libc::c_int,
        pub incomingSequence: libc::c_int,
        pub outgoingSequence: libc::c_int,
        pub fragmentSequence: libc::c_int,
        pub fragmentLength: libc::c_int,
        pub fragmentBuffer: [byte; 16384],
        pub unsentFragments: qboolean,
        pub unsentFragmentStart: libc::c_int,
        pub unsentLength: libc::c_int,
        pub unsentBuffer: [byte; 16384],
        pub challenge: libc::c_int,
        pub lastSentTime: libc::c_int,
        pub lastSentSize: libc::c_int,
        pub compat: qboolean,
    }
    pub type vm_t = vm_s;
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, fileHandle_t};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        // Adds command text at the end of the buffer, does NOT add a final \n
        #[no_mangle]
        pub fn Cbuf_ExecuteText(exec_when: libc::c_int,
                                text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
    }
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
    pub type stereoFrame_t = libc::c_uint;
    pub const STEREO_RIGHT: stereoFrame_t = 2;
    pub const STEREO_LEFT: stereoFrame_t = 1;
    pub const STEREO_CENTER: stereoFrame_t = 0;
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/renderercommon/tr_public.h"]
pub mod tr_public_h {
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
// these are the functions exported by the refresh module
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct refexport_t {
        pub Shutdown: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
        pub BeginRegistration: Option<unsafe extern "C" fn(_: *mut glconfig_t)
                                          -> ()>,
        pub RegisterModel: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> qhandle_t>,
        pub RegisterSkin: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> qhandle_t>,
        pub RegisterShader: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char)
                                       -> qhandle_t>,
        pub RegisterShaderNoMip: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> qhandle_t>,
        pub LoadWorld: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> ()>,
        pub SetWorldVisData: Option<unsafe extern "C" fn(_: *const byte)
                                        -> ()>,
        pub EndRegistration: Option<unsafe extern "C" fn() -> ()>,
        pub ClearScene: Option<unsafe extern "C" fn() -> ()>,
        pub AddRefEntityToScene: Option<unsafe extern "C" fn(_:
                                                                 *const refEntity_t)
                                            -> ()>,
        pub AddPolyToScene: Option<unsafe extern "C" fn(_: qhandle_t,
                                                        _: libc::c_int,
                                                        _: *const polyVert_t,
                                                        _: libc::c_int)
                                       -> ()>,
        pub LightForPoint: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t)
                                      -> libc::c_int>,
        pub AddLightToScene: Option<unsafe extern "C" fn(_: *const vec_t,
                                                         _: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_float)
                                        -> ()>,
        pub AddAdditiveLightToScene: Option<unsafe extern "C" fn(_:
                                                                     *const vec_t,
                                                                 _:
                                                                     libc::c_float,
                                                                 _:
                                                                     libc::c_float,
                                                                 _:
                                                                     libc::c_float,
                                                                 _:
                                                                     libc::c_float)
                                                -> ()>,
        pub RenderScene: Option<unsafe extern "C" fn(_: *const refdef_t)
                                    -> ()>,
        pub SetColor: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> ()>,
        pub DrawStretchPic: Option<unsafe extern "C" fn(_: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: qhandle_t) -> ()>,
        pub DrawStretchRaw: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte,
                                                        _: libc::c_int,
                                                        _: qboolean) -> ()>,
        pub UploadCinematic: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *const byte,
                                                         _: libc::c_int,
                                                         _: qboolean) -> ()>,
        pub BeginFrame: Option<unsafe extern "C" fn(_: stereoFrame_t) -> ()>,
        pub EndFrame: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                  _: *mut libc::c_int) -> ()>,
        pub MarkFragments: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *const vec3_t,
                                                       _: *const vec_t,
                                                       _: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: libc::c_int,
                                                       _: *mut markFragment_t)
                                      -> libc::c_int>,
        pub LerpTag: Option<unsafe extern "C" fn(_: *mut orientation_t,
                                                 _: qhandle_t, _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_float,
                                                 _: *const libc::c_char)
                                -> libc::c_int>,
        pub ModelBounds: Option<unsafe extern "C" fn(_: qhandle_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t) -> ()>,
        pub RegisterFont: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: libc::c_int,
                                                      _: *mut fontInfo_t)
                                     -> ()>,
        pub RemapShader: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *const libc::c_char,
                                                     _: *const libc::c_char)
                                    -> ()>,
        pub GetEntityToken: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> qboolean>,
        pub inPVS: Option<unsafe extern "C" fn(_: *const vec_t,
                                               _: *const vec_t) -> qboolean>,
        pub TakeVideoFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *mut byte,
                                                        _: *mut byte,
                                                        _: qboolean) -> ()>,
    }
    use super::q_shared_h::{qboolean, qhandle_t, byte, vec_t, vec3_t,
                            markFragment_t, orientation_t, fontInfo_t};
    use super::tr_types_h::{glconfig_t, refEntity_t, polyVert_t, refdef_t,
                            stereoFrame_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_public.h"]
pub mod ui_public_h {
    pub type unnamed_1 = libc::c_uint;
    pub const UIMENU_POSTGAME: unnamed_1 = 6;
    pub const UIMENU_TEAM: unnamed_1 = 5;
    pub const UIMENU_BAD_CD_KEY: unnamed_1 = 4;
    pub const UIMENU_NEED_CD: unnamed_1 = 3;
    pub const UIMENU_INGAME: unnamed_1 = 2;
    pub const UIMENU_MAIN: unnamed_1 = 1;
    pub const UIMENU_NONE: unnamed_1 = 0;
    pub type unnamed_2 = libc::c_uint;
    //	void	UI_DrawConnectScreen( qboolean overlay );
    pub const UI_HASUNIQUECDKEY: unnamed_2 = 10;
    //	qboolean UI_ConsoleCommand( int realTime );
    pub const UI_DRAW_CONNECT_SCREEN: unnamed_2 = 9;
    //	void	UI_SetActiveMenu( uiMenuCommand_t menu );
    pub const UI_CONSOLE_COMMAND: unnamed_2 = 8;
    //	qboolean UI_IsFullscreen( void );
    pub const UI_SET_ACTIVE_MENU: unnamed_2 = 7;
    //	void	UI_Refresh( int time );
    pub const UI_IS_FULLSCREEN: unnamed_2 = 6;
    //	void	UI_MouseEvent( int dx, int dy );
    pub const UI_REFRESH: unnamed_2 = 5;
    //	void	UI_KeyEvent( int key );
    pub const UI_MOUSE_EVENT: unnamed_2 = 4;
    //	void	UI_Shutdown( void );
    pub const UI_KEY_EVENT: unnamed_2 = 3;
    //	void	UI_Init( void );
    pub const UI_SHUTDOWN: unnamed_2 = 2;
    pub const UI_INIT: unnamed_2 = 1;
    // system reserved
    pub const UI_GETAPIVERSION: unnamed_2 = 0;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    extern "C" {
        /* Copyright (c) 2010-2011 Xiph.Org Foundation, Skype Limited
   Written by Jean-Marc Valin and Koen Vos */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
        /* *
 * @file opus.h
 * @brief Opus reference implementation API
 */
        /* *
 * @mainpage Opus
 *
 * The Opus codec is designed for interactive speech and audio transmission over the Internet.
 * It is designed by the IETF Codec Working Group and incorporates technology from
 * Skype's SILK codec and Xiph.Org's CELT codec.
 *
 * The Opus codec is designed to handle a wide range of interactive audio applications,
 * including Voice over IP, videoconferencing, in-game chat, and even remote live music
 * performances. It can scale from low bit-rate narrowband speech to very high quality
 * stereo music. Its main features are:

 * @li Sampling rates from 8 to 48 kHz
 * @li Bit-rates from 6 kb/s to 510 kb/s
 * @li Support for both constant bit-rate (CBR) and variable bit-rate (VBR)
 * @li Audio bandwidth from narrowband to full-band
 * @li Support for speech and music
 * @li Support for mono and stereo
 * @li Support for multichannel (up to 255 channels)
 * @li Frame sizes from 2.5 ms to 60 ms
 * @li Good loss robustness and packet loss concealment (PLC)
 * @li Floating point and fixed-point implementation
 *
 * Documentation sections:
 * @li @ref opus_encoder
 * @li @ref opus_decoder
 * @li @ref opus_repacketizer
 * @li @ref opus_multistream
 * @li @ref opus_libinfo
 * @li @ref opus_custom
 */
        /* * @defgroup opus_encoder Opus Encoder
  * @{
  *
  * @brief This page describes the process and functions used to encode Opus.
  *
  * Since Opus is a stateful codec, the encoding process starts with creating an encoder
  * state. This can be done with:
  *
  * @code
  * int          error;
  * OpusEncoder *enc;
  * enc = opus_encoder_create(Fs, channels, application, &error);
  * @endcode
  *
  * From this point, @c enc can be used for encoding an audio stream. An encoder state
  * @b must @b not be used for more than one stream at the same time. Similarly, the encoder
  * state @b must @b not be re-initialized for each frame.
  *
  * While opus_encoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  *
  * @code
  * int          size;
  * int          error;
  * OpusEncoder *enc;
  * size = opus_encoder_get_size(channels);
  * enc = malloc(size);
  * error = opus_encoder_init(enc, Fs, channels, application);
  * @endcode
  *
  * where opus_encoder_get_size() returns the required size for the encoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The encoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * It is possible to change some of the encoder's settings using the opus_encoder_ctl()
  * interface. All these settings already default to the recommended value, so they should
  * only be changed when necessary. The most common settings one may want to change are:
  *
  * @code
  * opus_encoder_ctl(enc, OPUS_SET_BITRATE(bitrate));
  * opus_encoder_ctl(enc, OPUS_SET_COMPLEXITY(complexity));
  * opus_encoder_ctl(enc, OPUS_SET_SIGNAL(signal_type));
  * @endcode
  *
  * where
  *
  * @arg bitrate is in bits per second (b/s)
  * @arg complexity is a value from 1 to 10, where 1 is the lowest complexity and 10 is the highest
  * @arg signal_type is either OPUS_AUTO (default), OPUS_SIGNAL_VOICE, or OPUS_SIGNAL_MUSIC
  *
  * See @ref opus_encoderctls and @ref opus_genericctls for a complete list of parameters that can be set or queried. Most parameters can be set or changed at any time during a stream.
  *
  * To encode a frame, opus_encode() or opus_encode_float() must be called with exactly one frame (2.5, 5, 10, 20, 40 or 60 ms) of audio data:
  * @code
  * len = opus_encode(enc, audio_frame, frame_size, packet, max_packet);
  * @endcode
  *
  * where
  * <ul>
  * <li>audio_frame is the audio data in opus_int16 (or float for opus_encode_float())</li>
  * <li>frame_size is the duration of the frame in samples (per channel)</li>
  * <li>packet is the byte array to which the compressed data is written</li>
  * <li>max_packet is the maximum number of bytes that can be written in the packet (4000 bytes is recommended).
  *     Do not use max_packet to control VBR target bitrate, instead use the #OPUS_SET_BITRATE CTL.</li>
  * </ul>
  *
  * opus_encode() and opus_encode_float() return the number of bytes actually written to the packet.
  * The return value <b>can be negative</b>, which indicates that an error has occurred. If the return value
  * is 1 byte, then the packet does not need to be transmitted (DTX).
  *
  * Once the encoder state if no longer needed, it can be destroyed with
  *
  * @code
  * opus_encoder_destroy(enc);
  * @endcode
  *
  * If the encoder was created with opus_encoder_init() rather than opus_encoder_create(),
  * then no action is required aside from potentially freeing the memory that was manually
  * allocated for it (calling free(enc) for the example above)
  *
  */
        /* * Opus encoder state.
  * This contains the complete state of an Opus encoder.
  * It is position independent and can be freely copied.
  * @see opus_encoder_create,opus_encoder_init
  */
        pub type OpusEncoder;
        /* *@}*/
        /* * @defgroup opus_decoder Opus Decoder
  * @{
  *
  * @brief This page describes the process and functions used to decode Opus.
  *
  * The decoding process also starts with creating a decoder
  * state. This can be done with:
  * @code
  * int          error;
  * OpusDecoder *dec;
  * dec = opus_decoder_create(Fs, channels, &error);
  * @endcode
  * where
  * @li Fs is the sampling rate and must be 8000, 12000, 16000, 24000, or 48000
  * @li channels is the number of channels (1 or 2)
  * @li error will hold the error code in case of failure (or #OPUS_OK on success)
  * @li the return value is a newly created decoder state to be used for decoding
  *
  * While opus_decoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  * @code
  * int          size;
  * int          error;
  * OpusDecoder *dec;
  * size = opus_decoder_get_size(channels);
  * dec = malloc(size);
  * error = opus_decoder_init(dec, Fs, channels);
  * @endcode
  * where opus_decoder_get_size() returns the required size for the decoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The decoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * To decode a frame, opus_decode() or opus_decode_float() must be called with a packet of compressed audio data:
  * @code
  * frame_size = opus_decode(dec, packet, len, decoded, max_size, 0);
  * @endcode
  * where
  *
  * @li packet is the byte array containing the compressed data
  * @li len is the exact number of bytes contained in the packet
  * @li decoded is the decoded audio data in opus_int16 (or float for opus_decode_float())
  * @li max_size is the max duration of the frame in samples (per channel) that can fit into the decoded_frame array
  *
  * opus_decode() and opus_decode_float() return the number of samples (per channel) decoded from the packet.
  * If that value is negative, then an error has occurred. This can occur if the packet is corrupted or if the audio
  * buffer is too small to hold the decoded audio.
  *
  * Opus is a stateful codec with overlapping blocks and as a result Opus
  * packets are not coded independently of each other. Packets must be
  * passed into the decoder serially and in the correct order for a correct
  * decode. Lost packets can be replaced with loss concealment by calling
  * the decoder with a null pointer and zero length for the missing packet.
  *
  * A single codec state may only be accessed from a single thread at
  * a time and any required locking must be performed by the caller. Separate
  * streams must be decoded with separate decoder states and can be decoded
  * in parallel unless the library was compiled with NONTHREADSAFE_PSEUDOSTACK
  * defined.
  *
  */
        /* * Opus decoder state.
  * This contains the complete state of an Opus decoder.
  * It is position independent and can be freely copied.
  * @see opus_decoder_create,opus_decoder_init
  */
        pub type OpusDecoder;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/client.h"]
pub mod client_h {
    /*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientConnection_t {
        pub state: connstate_t,
        pub clientNum: libc::c_int,
        pub lastPacketSentTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub servername: [libc::c_char; 4096],
        pub serverAddress: netadr_t,
        pub connectTime: libc::c_int,
        pub connectPacketCount: libc::c_int,
        pub serverMessage: [libc::c_char; 1024],
        pub challenge: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub serverMessageSequence: libc::c_int,
        pub serverCommandSequence: libc::c_int,
        pub lastExecutedServerCommand: libc::c_int,
        pub serverCommands: [[libc::c_char; 1024]; 64],
        pub download: fileHandle_t,
        pub downloadTempName: [libc::c_char; 4096],
        pub downloadName: [libc::c_char; 4096],
        pub cURLEnabled: qboolean,
        pub cURLUsed: qboolean,
        pub cURLDisconnected: qboolean,
        pub downloadURL: [libc::c_char; 4096],
        pub downloadCURL: *mut libc::c_void,
        pub downloadCURLM: *mut libc::c_void,
        pub sv_allowDownload: libc::c_int,
        pub sv_dlURL: [libc::c_char; 256],
        pub downloadNumber: libc::c_int,
        pub downloadBlock: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadSize: libc::c_int,
        pub downloadList: [libc::c_char; 1024],
        pub downloadRestart: qboolean,
        pub demoName: [libc::c_char; 64],
        pub spDemoRecording: qboolean,
        pub demorecording: qboolean,
        pub demoplaying: qboolean,
        pub demowaiting: qboolean,
        pub firstDemoFrameSkipped: qboolean,
        pub demofile: fileHandle_t,
        pub timeDemoFrames: libc::c_int,
        pub timeDemoStart: libc::c_int,
        pub timeDemoBaseTime: libc::c_int,
        pub timeDemoLastFrame: libc::c_int,
        pub timeDemoMinDuration: libc::c_int,
        pub timeDemoMaxDuration: libc::c_int,
        pub timeDemoDurations: [libc::c_uchar; 4096],
        pub aviVideoFrameRemainder: libc::c_float,
        pub aviSoundFrameRemainder: libc::c_float,
        pub voipEnabled: qboolean,
        pub voipCodecInitialized: qboolean,
        pub opusDecoder: [*mut OpusDecoder; 64],
        pub voipIncomingGeneration: [byte; 64],
        pub voipIncomingSequence: [libc::c_int; 64],
        pub voipGain: [libc::c_float; 64],
        pub voipIgnore: [qboolean; 64],
        pub voipMuteAll: qboolean,
        pub voipTargets: [uint8_t; 8],
        pub voipFlags: uint8_t,
        pub opusEncoder: *mut OpusEncoder,
        pub voipOutgoingDataSize: libc::c_int,
        pub voipOutgoingDataFrames: libc::c_int,
        pub voipOutgoingSequence: libc::c_int,
        pub voipOutgoingGeneration: byte,
        pub voipOutgoingData: [byte; 1024],
        pub voipPower: libc::c_float,
        pub compat: qboolean,
        pub netchan: netchan_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverInfo_t {
        pub adr: netadr_t,
        pub hostName: [libc::c_char; 32],
        pub mapName: [libc::c_char; 32],
        pub game: [libc::c_char; 32],
        pub netType: libc::c_int,
        pub gameType: libc::c_int,
        pub clients: libc::c_int,
        pub maxClients: libc::c_int,
        pub minPing: libc::c_int,
        pub maxPing: libc::c_int,
        pub ping: libc::c_int,
        pub visible: qboolean,
        pub punkbuster: libc::c_int,
        pub g_humanplayers: libc::c_int,
        pub g_needpass: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientStatic_t {
        pub cddialog: qboolean,
        pub rendererStarted: qboolean,
        pub soundStarted: qboolean,
        pub soundRegistered: qboolean,
        pub uiStarted: qboolean,
        pub cgameStarted: qboolean,
        pub framecount: libc::c_int,
        pub frametime: libc::c_int,
        pub realtime: libc::c_int,
        pub realFrametime: libc::c_int,
        pub numlocalservers: libc::c_int,
        pub localServers: [serverInfo_t; 128],
        pub numglobalservers: libc::c_int,
        pub globalServers: [serverInfo_t; 4096],
        pub numGlobalServerAddresses: libc::c_int,
        pub globalServerAddresses: [netadr_t; 4096],
        pub numfavoriteservers: libc::c_int,
        pub favoriteServers: [serverInfo_t; 128],
        pub pingUpdateSource: libc::c_int,
        pub updateServer: netadr_t,
        pub updateChallenge: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub authorizeServer: netadr_t,
        pub rconAddress: netadr_t,
        pub glconfig: glconfig_t,
        pub charSetShader: qhandle_t,
        pub whiteShader: qhandle_t,
        pub consoleShader: qhandle_t,
    }
    use super::q_shared_h::{connstate_t, fileHandle_t, qboolean, byte,
                            qhandle_t, cvar_t, e_status};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    use super::tr_public_h::{refexport_t};
    extern "C" {
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        // interface to ui dll or vm
        #[no_mangle]
        pub static mut uivm: *mut vm_t;
        // interface to refresh .dll
        #[no_mangle]
        pub static mut re: refexport_t;
        #[no_mangle]
        pub static mut cl_inGameVideo: *mut cvar_t;
        #[no_mangle]
        pub fn Con_Close();
        #[no_mangle]
        pub fn SCR_AdjustFrom640(x: *mut libc::c_float, y: *mut libc::c_float,
                                 w: *mut libc::c_float,
                                 h: *mut libc::c_float);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/cl_cin.c"]
pub mod cl_cin_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cin_cache {
        pub fileName: [libc::c_char; 4096],
        pub CIN_WIDTH: libc::c_int,
        pub CIN_HEIGHT: libc::c_int,
        pub xpos: libc::c_int,
        pub ypos: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub looping: qboolean,
        pub holdAtEnd: qboolean,
        pub dirty: qboolean,
        pub alterGameState: qboolean,
        pub silent: qboolean,
        pub shader: qboolean,
        pub iFile: fileHandle_t,
        pub status: e_status,
        pub startTime: libc::c_int,
        pub lastTime: libc::c_int,
        pub tfps: libc::c_long,
        pub RoQPlayed: libc::c_long,
        pub ROQSize: libc::c_long,
        pub RoQFrameSize: libc::c_uint,
        pub onQuad: libc::c_long,
        pub numQuads: libc::c_long,
        pub samplesPerLine: libc::c_long,
        pub roq_id: libc::c_uint,
        pub screenDelta: libc::c_long,
        pub VQ0: Option<unsafe extern "C" fn(_: *mut byte,
                                             _: *mut libc::c_void) -> ()>,
        pub VQ1: Option<unsafe extern "C" fn(_: *mut byte,
                                             _: *mut libc::c_void) -> ()>,
        pub VQNormal: Option<unsafe extern "C" fn(_: *mut byte,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub VQBuffer: Option<unsafe extern "C" fn(_: *mut byte,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub samplesPerPixel: libc::c_long,
        pub gray: *mut byte,
        pub xsize: libc::c_uint,
        pub ysize: libc::c_uint,
        pub maxsize: libc::c_uint,
        pub minsize: libc::c_uint,
        pub half: qboolean,
        pub smootheddouble: qboolean,
        pub inMemory: qboolean,
        pub normalBuffer0: libc::c_long,
        pub roq_flags: libc::c_long,
        pub roqF0: libc::c_long,
        pub roqF1: libc::c_long,
        pub t: [libc::c_long; 2],
        pub roqFPS: libc::c_long,
        pub playonwalls: libc::c_int,
        pub buf: *mut byte,
        pub drawX: libc::c_long,
        pub drawY: libc::c_long,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cinematics_t {
        pub linbuf: [byte; 2097152],
        pub file: [byte; 65536],
        pub sqrTable: [libc::c_short; 256],
        pub mcomp: [libc::c_int; 256],
        pub qStatus: [[*mut byte; 32768]; 2],
        pub oldXOff: libc::c_long,
        pub oldYOff: libc::c_long,
        pub oldysize: libc::c_long,
        pub oldxsize: libc::c_long,
        pub currentHandle: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_3 {
        pub i: *mut libc::c_uint,
        pub s: *mut libc::c_ushort,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean, fileHandle_t, e_status, byte};
    extern "C" {
        #[no_mangle]
        pub fn CL_ScaledMilliseconds() -> libc::c_int;
        // sample PAIRS
        #[no_mangle]
        pub static mut s_soundtime: libc::c_int;
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
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    use super::{libc};
    use super::q_shared_h::{byte};
    extern "C" {
        // cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
        #[no_mangle]
        pub fn S_RawSamples(stream: libc::c_int, samples: libc::c_int,
                            rate: libc::c_int, width: libc::c_int,
                            channels: libc::c_int, data: *const byte,
                            volume: libc::c_float, entityNum: libc::c_int);
        // stop all sounds and the background track
        #[no_mangle]
        pub fn S_StopAllSounds();
        #[no_mangle]
        pub fn S_Update();
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_local.h"]
pub mod snd_local_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut s_rawend: [libc::c_int; 129];
    }
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed, EXEC_APPEND, EXEC_INSERT, EXEC_NOW, unnamed_0,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cvar_s, cvar_t,
                       markFragment_t, orientation_t, connstate_t,
                       CA_CINEMATIC, CA_ACTIVE, CA_PRIMED, CA_LOADING,
                       CA_CONNECTED, CA_CHALLENGING, CA_CONNECTING,
                       CA_AUTHORIZING, CA_DISCONNECTED, CA_UNINITIALIZED,
                       glyphInfo_t, fontInfo_t, e_status, FMV_ID_WAIT,
                       FMV_LOOPED, FMV_ID_IDLE, FMV_ID_BLT, FMV_EOF, FMV_PLAY,
                       FMV_IDLE, Com_sprintf, Q_stricmp, va, Com_Error,
                       Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t, vm_t, vm_s,
                      VM_Call, Cbuf_ExecuteText, Cmd_Argv, Cvar_Set,
                      Cvar_VariableString, FS_FOpenFileRead, FS_Read,
                      FS_FCloseFile, Com_DPrintf, Hunk_AllocateTempMemory,
                      Hunk_FreeTempMemory};
use self::tr_types_h::{polyVert_t, refEntityType_t, RT_MAX_REF_ENTITY_TYPE,
                       RT_PORTALSURFACE, RT_LIGHTNING, RT_RAIL_RINGS,
                       RT_RAIL_CORE, RT_BEAM, RT_SPRITE, RT_POLY, RT_MODEL,
                       refEntity_t, refdef_t, stereoFrame_t, STEREO_RIGHT,
                       STEREO_LEFT, STEREO_CENTER, textureCompression_t,
                       TC_S3TC_ARB, TC_S3TC, TC_NONE, glDriverType_t,
                       GLDRV_VOODOO, GLDRV_STANDALONE, GLDRV_ICD,
                       glHardwareType_t, GLHW_PERMEDIA2, GLHW_RAGEPRO,
                       GLHW_RIVA128, GLHW_3DFX_2D3D, GLHW_GENERIC,
                       glconfig_t};
use self::tr_public_h::{refexport_t};
use self::ui_public_h::{unnamed_1, UIMENU_POSTGAME, UIMENU_TEAM,
                        UIMENU_BAD_CD_KEY, UIMENU_NEED_CD, UIMENU_INGAME,
                        UIMENU_MAIN, UIMENU_NONE, unnamed_2,
                        UI_HASUNIQUECDKEY, UI_DRAW_CONNECT_SCREEN,
                        UI_CONSOLE_COMMAND, UI_SET_ACTIVE_MENU,
                        UI_IS_FULLSCREEN, UI_REFRESH, UI_MOUSE_EVENT,
                        UI_KEY_EVENT, UI_SHUTDOWN, UI_INIT, UI_GETAPIVERSION};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::client_h::{clientConnection_t, serverInfo_t, clientStatic_t, clc,
                     cls, uivm, re, cl_inGameVideo, Con_Close,
                     SCR_AdjustFrom640};
use self::cl_cin_c::{cin_cache, cinematics_t, unnamed_3,
                     CL_ScaledMilliseconds, s_soundtime};
use self::string_h::{memcpy, memmove, memset, strcpy, strcmp, strstr};
use self::stdlib_h::{abs};
use self::snd_public_h::{S_RawSamples, S_StopAllSounds, S_Update};
use self::snd_local_h::{s_rawend};
//
// cl_cin.c
//
#[no_mangle]
pub unsafe extern "C" fn CL_PlayCinematic_f() {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bits: libc::c_int = 1i32;
    Com_DPrintf(b"CL_PlayCinematic_f\n\x00" as *const u8 as
                    *const libc::c_char);
    if clc.state as libc::c_uint ==
           CA_CINEMATIC as libc::c_int as libc::c_uint {
        SCR_StopCinematic();
    }
    arg = Cmd_Argv(1i32);
    s = Cmd_Argv(2i32);
    if !s.is_null() && *s.offset(0isize) as libc::c_int == '1' as i32 ||
           Q_stricmp(arg,
                     b"demoend.roq\x00" as *const u8 as *const libc::c_char)
               == 0i32 ||
           Q_stricmp(arg, b"end.roq\x00" as *const u8 as *const libc::c_char)
               == 0i32 {
        bits |= 4i32
    }
    if !s.is_null() && *s.offset(0isize) as libc::c_int == '2' as i32 {
        bits |= 2i32
    }
    S_StopAllSounds();
    CL_handle = CIN_PlayCinematic(arg, 0i32, 0i32, 640i32, 480i32, bits);
    if CL_handle >= 0i32 {
        loop  {
            SCR_RunCinematic();
            if !(cinTable[currentHandle as usize].buf.is_null() &&
                     cinTable[currentHandle as usize].status as libc::c_uint
                         == FMV_PLAY as libc::c_int as libc::c_uint) {
                break ;
            }
        }
    };
}
static mut currentHandle: libc::c_int = -1i32;
static mut cinTable: [cin_cache; 16] =
    [cin_cache{fileName: [0; 4096],
               CIN_WIDTH: 0,
               CIN_HEIGHT: 0,
               xpos: 0,
               ypos: 0,
               width: 0,
               height: 0,
               looping: qfalse,
               holdAtEnd: qfalse,
               dirty: qfalse,
               alterGameState: qfalse,
               silent: qfalse,
               shader: qfalse,
               iFile: 0,
               status: FMV_IDLE,
               startTime: 0,
               lastTime: 0,
               tfps: 0,
               RoQPlayed: 0,
               ROQSize: 0,
               RoQFrameSize: 0,
               onQuad: 0,
               numQuads: 0,
               samplesPerLine: 0,
               roq_id: 0,
               screenDelta: 0,
               VQ0: None,
               VQ1: None,
               VQNormal: None,
               VQBuffer: None,
               samplesPerPixel: 0,
               gray: 0 as *const byte as *mut byte,
               xsize: 0,
               ysize: 0,
               maxsize: 0,
               minsize: 0,
               half: qfalse,
               smootheddouble: qfalse,
               inMemory: qfalse,
               normalBuffer0: 0,
               roq_flags: 0,
               roqF0: 0,
               roqF1: 0,
               t: [0; 2],
               roqFPS: 0,
               playonwalls: 0,
               buf: 0 as *const byte as *mut byte,
               drawX: 0,
               drawY: 0,}; 16];
#[no_mangle]
pub unsafe extern "C" fn SCR_RunCinematic() {
    if CL_handle >= 0i32 && CL_handle < 16i32 {
        CIN_RunCinematic(CL_handle);
    };
}
static mut CL_handle: libc::c_int = -1i32;
#[no_mangle]
pub unsafe extern "C" fn CIN_RunCinematic(mut handle: libc::c_int)
 -> e_status {
    let mut start: libc::c_int = 0i32;
    let mut thisTime: libc::c_int = 0i32;
    if handle < 0i32 || handle >= 16i32 ||
           cinTable[handle as usize].status as libc::c_uint ==
               FMV_EOF as libc::c_int as libc::c_uint {
        return FMV_EOF
    }
    if cin.currentHandle != handle {
        currentHandle = handle;
        cin.currentHandle = currentHandle;
        cinTable[currentHandle as usize].status = FMV_EOF;
        RoQReset();
    }
    if cinTable[handle as usize].playonwalls < -1i32 {
        return cinTable[handle as usize].status
    }
    currentHandle = handle;
    if 0 != cinTable[currentHandle as usize].alterGameState as u64 {
        if clc.state as libc::c_uint !=
               CA_CINEMATIC as libc::c_int as libc::c_uint {
            return cinTable[currentHandle as usize].status
        }
    }
    if cinTable[currentHandle as usize].status as libc::c_uint ==
           FMV_IDLE as libc::c_int as libc::c_uint {
        return cinTable[currentHandle as usize].status
    }
    thisTime = CL_ScaledMilliseconds();
    if 0 != cinTable[currentHandle as usize].shader as libc::c_uint &&
           abs(thisTime - cinTable[currentHandle as usize].lastTime) > 100i32
       {
        cinTable[currentHandle as usize].startTime +=
            thisTime - cinTable[currentHandle as usize].lastTime
    }
    cinTable[currentHandle as usize].tfps =
        ((CL_ScaledMilliseconds() -
              cinTable[currentHandle as usize].startTime) * 3i32 / 100i32) as
            libc::c_long;
    start = cinTable[currentHandle as usize].startTime;
    while cinTable[currentHandle as usize].tfps !=
              cinTable[currentHandle as usize].numQuads &&
              cinTable[currentHandle as usize].status as libc::c_uint ==
                  FMV_PLAY as libc::c_int as libc::c_uint {
        RoQInterrupt();
        if start != cinTable[currentHandle as usize].startTime {
            cinTable[currentHandle as usize].tfps =
                ((CL_ScaledMilliseconds() -
                      cinTable[currentHandle as usize].startTime) * 3i32 /
                     100i32) as libc::c_long;
            start = cinTable[currentHandle as usize].startTime
        }
    }
    cinTable[currentHandle as usize].lastTime = thisTime;
    if cinTable[currentHandle as usize].status as libc::c_uint ==
           FMV_LOOPED as libc::c_int as libc::c_uint {
        cinTable[currentHandle as usize].status = FMV_PLAY
    }
    if cinTable[currentHandle as usize].status as libc::c_uint ==
           FMV_EOF as libc::c_int as libc::c_uint {
        if 0 != cinTable[currentHandle as usize].looping as u64 {
            RoQReset();
        } else { RoQShutdown(); return FMV_EOF }
    }
    return cinTable[currentHandle as usize].status;
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn RoQShutdown() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if cinTable[currentHandle as usize].buf.is_null() { return }
    if cinTable[currentHandle as usize].status as libc::c_uint ==
           FMV_IDLE as libc::c_int as libc::c_uint {
        return
    }
    Com_DPrintf(b"finished cinematic\n\x00" as *const u8 as
                    *const libc::c_char);
    cinTable[currentHandle as usize].status = FMV_IDLE;
    if 0 != cinTable[currentHandle as usize].iFile {
        FS_FCloseFile(cinTable[currentHandle as usize].iFile);
        cinTable[currentHandle as usize].iFile = 0i32
    }
    if 0 != cinTable[currentHandle as usize].alterGameState as u64 {
        clc.state = CA_DISCONNECTED;
        s =
            Cvar_VariableString(b"nextmap\x00" as *const u8 as
                                    *const libc::c_char);
        if 0 != *s.offset(0isize) {
            Cbuf_ExecuteText(EXEC_APPEND as libc::c_int,
                             va(b"%s\n\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                s));
            Cvar_Set(b"nextmap\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char);
        }
        CL_handle = -1i32
    }
    cinTable[currentHandle as usize].fileName[0usize] = 0i32 as libc::c_char;
    currentHandle = -1i32;
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
/*
static byte* RoQFetchInterlaced( byte *source ) {
	int x, *src, *dst;

	if (currentHandle < 0) return NULL;

	src = (int *)source;
	dst = (int *)cinTable[currentHandle].buf2;

	for(x=0;x<256*256;x++) {
		*dst = *src;
		dst++; src += 2;
	}
	return cinTable[currentHandle].buf2;
}
*/
unsafe extern "C" fn RoQReset() {
    if currentHandle < 0i32 { return }
    FS_FCloseFile(cinTable[currentHandle as usize].iFile);
    FS_FOpenFileRead(cinTable[currentHandle as usize].fileName.as_mut_ptr(),
                     &mut cinTable[currentHandle as usize].iFile, qtrue);
    FS_Read(cin.file.as_mut_ptr() as *mut libc::c_void, 16i32,
            cinTable[currentHandle as usize].iFile);
    RoQ_init();
    cinTable[currentHandle as usize].status = FMV_LOOPED;
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
 * name:		cl_cin.c
 *
 * desc:		video and cinematic playback
 *
 * $Archive: /MissionPack/code/client/cl_cin.c $
 *
 * cl_glconfig.hwtype trtypes 3dfx/ragepro need 256x256
 *
 *****************************************************************************/
unsafe extern "C" fn RoQ_init() {
    cinTable[currentHandle as usize].lastTime = CL_ScaledMilliseconds();
    cinTable[currentHandle as usize].startTime =
        cinTable[currentHandle as usize].lastTime;
    cinTable[currentHandle as usize].RoQPlayed = 24i32 as libc::c_long;
    cinTable[currentHandle as usize].roqFPS =
        (cin.file[6usize] as libc::c_int +
             cin.file[7usize] as libc::c_int * 256i32) as libc::c_long;
    if 0 == cinTable[currentHandle as usize].roqFPS {
        cinTable[currentHandle as usize].roqFPS = 30i32 as libc::c_long
    }
    cinTable[currentHandle as usize].numQuads = -1i32 as libc::c_long;
    cinTable[currentHandle as usize].roq_id =
        (cin.file[8usize] as libc::c_int +
             cin.file[9usize] as libc::c_int * 256i32) as libc::c_uint;
    cinTable[currentHandle as usize].RoQFrameSize =
        (cin.file[10usize] as libc::c_int +
             cin.file[11usize] as libc::c_int * 256i32 +
             cin.file[12usize] as libc::c_int * 65536i32) as libc::c_uint;
    cinTable[currentHandle as usize].roq_flags =
        (cin.file[14usize] as libc::c_int +
             cin.file[15usize] as libc::c_int * 256i32) as libc::c_long;
    if cinTable[currentHandle as usize].RoQFrameSize >
           65536i32 as libc::c_uint ||
           0 == cinTable[currentHandle as usize].RoQFrameSize {
        return
    };
}
static mut cin: cinematics_t =
    cinematics_t{linbuf: [0; 2097152],
                 file: [0; 65536],
                 sqrTable: [0; 256],
                 mcomp: [0; 256],
                 qStatus: [[0 as *const byte as *mut byte; 32768]; 2],
                 oldXOff: 0,
                 oldYOff: 0,
                 oldysize: 0,
                 oldxsize: 0,
                 currentHandle: 0,};
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn RoQInterrupt() {
    let mut framedata: *mut byte = 0 as *mut byte;
    let mut sbuf: [libc::c_short; 32768] = [0; 32768];
    let mut ssize: libc::c_int = 0;
    if currentHandle < 0i32 { return }
    FS_Read(cin.file.as_mut_ptr() as *mut libc::c_void,
            cinTable[currentHandle as
                         usize].RoQFrameSize.wrapping_add(8i32 as
                                                              libc::c_uint) as
                libc::c_int, cinTable[currentHandle as usize].iFile);
    if cinTable[currentHandle as usize].RoQPlayed >=
           cinTable[currentHandle as usize].ROQSize {
        if cinTable[currentHandle as usize].holdAtEnd as libc::c_uint ==
               qfalse as libc::c_int as libc::c_uint {
            if 0 != cinTable[currentHandle as usize].looping as u64 {
                RoQReset();
            } else { cinTable[currentHandle as usize].status = FMV_EOF }
        } else { cinTable[currentHandle as usize].status = FMV_IDLE }
        return
    }
    framedata = cin.file.as_mut_ptr();
    //
// new frame is ready
//
    loop  {
        match cinTable[currentHandle as usize].roq_id {
            4113 => {
                if 0 !=
                       cinTable[currentHandle as usize].numQuads &
                           1i32 as libc::c_long {
                    cinTable[currentHandle as usize].normalBuffer0 =
                        cinTable[currentHandle as usize].t[1usize];
                    RoQPrepMcomp(cinTable[currentHandle as usize].roqF0,
                                 cinTable[currentHandle as usize].roqF1);
                    cinTable[currentHandle as
                                 usize].VQ1.expect("non-null function pointer")(cin.qStatus[1usize].as_mut_ptr()
                                                                                    as
                                                                                    *mut byte,
                                                                                framedata
                                                                                    as
                                                                                    *mut libc::c_void);
                    cinTable[currentHandle as usize].buf =
                        cin.linbuf.as_mut_ptr().offset(cinTable[currentHandle
                                                                    as
                                                                    usize].screenDelta
                                                           as isize)
                } else {
                    cinTable[currentHandle as usize].normalBuffer0 =
                        cinTable[currentHandle as usize].t[0usize];
                    RoQPrepMcomp(cinTable[currentHandle as usize].roqF0,
                                 cinTable[currentHandle as usize].roqF1);
                    cinTable[currentHandle as
                                 usize].VQ0.expect("non-null function pointer")(cin.qStatus[0usize].as_mut_ptr()
                                                                                    as
                                                                                    *mut byte,
                                                                                framedata
                                                                                    as
                                                                                    *mut libc::c_void);
                    cinTable[currentHandle as usize].buf =
                        cin.linbuf.as_mut_ptr()
                }
                if cinTable[currentHandle as usize].numQuads ==
                       0i32 as libc::c_long {
                    memcpy(cin.linbuf.as_mut_ptr().offset(cinTable[currentHandle
                                                                       as
                                                                       usize].screenDelta
                                                              as isize) as
                               *mut libc::c_void,
                           cin.linbuf.as_mut_ptr() as *const libc::c_void,
                           (cinTable[currentHandle as usize].samplesPerLine *
                                cinTable[currentHandle as usize].ysize as
                                    libc::c_long) as libc::c_ulong);
                }
                cinTable[currentHandle as usize].numQuads += 1;
                cinTable[currentHandle as usize].dirty = qtrue
            }
            4098 => {
                decodeCodeBook(framedata,
                               cinTable[currentHandle as usize].roq_flags as
                                   libc::c_ushort);
            }
            4128 => {
                if 0 == cinTable[currentHandle as usize].silent as u64 {
                    ssize =
                        RllDecodeMonoToStereo(framedata, sbuf.as_mut_ptr(),
                                              cinTable[currentHandle as
                                                           usize].RoQFrameSize,
                                              0i32 as libc::c_char,
                                              cinTable[currentHandle as
                                                           usize].roq_flags as
                                                  libc::c_ushort) as
                            libc::c_int;
                    S_RawSamples(0i32, ssize, 22050i32, 2i32, 1i32,
                                 sbuf.as_mut_ptr() as *mut byte, 1.0f32,
                                 -1i32);
                }
            }
            4129 => {
                if 0 == cinTable[currentHandle as usize].silent as u64 {
                    if cinTable[currentHandle as usize].numQuads ==
                           -1i32 as libc::c_long {
                        S_Update();
                        s_rawend[0usize] = s_soundtime
                    }
                    ssize =
                        RllDecodeStereoToStereo(framedata, sbuf.as_mut_ptr(),
                                                cinTable[currentHandle as
                                                             usize].RoQFrameSize,
                                                0i32 as libc::c_char,
                                                cinTable[currentHandle as
                                                             usize].roq_flags
                                                    as libc::c_ushort) as
                            libc::c_int;
                    S_RawSamples(0i32, ssize, 22050i32, 2i32, 2i32,
                                 sbuf.as_mut_ptr() as *mut byte, 1.0f32,
                                 -1i32);
                }
            }
            4097 => {
                if cinTable[currentHandle as usize].numQuads ==
                       -1i32 as libc::c_long {
                    readQuadInfo(framedata);
                    setupQuad(0i32 as libc::c_long, 0i32 as libc::c_long);
                    cinTable[currentHandle as usize].lastTime =
                        CL_ScaledMilliseconds();
                    cinTable[currentHandle as usize].startTime =
                        cinTable[currentHandle as usize].lastTime
                }
                if cinTable[currentHandle as usize].numQuads !=
                       1i32 as libc::c_long {
                    cinTable[currentHandle as usize].numQuads =
                        0i32 as libc::c_long
                }
            }
            4144 => {
                cinTable[currentHandle as usize].inMemory =
                    cinTable[currentHandle as usize].roq_flags as qboolean;
                cinTable[currentHandle as usize].RoQFrameSize =
                    0i32 as libc::c_uint
            }
            4115 => {
                cinTable[currentHandle as usize].RoQFrameSize =
                    0i32 as libc::c_uint
            }
            4114 => { }
            _ => { cinTable[currentHandle as usize].status = FMV_EOF }
        }
        if cinTable[currentHandle as usize].RoQPlayed >=
               cinTable[currentHandle as usize].ROQSize {
            if cinTable[currentHandle as usize].holdAtEnd as libc::c_uint ==
                   qfalse as libc::c_int as libc::c_uint {
                if 0 != cinTable[currentHandle as usize].looping as u64 {
                    RoQReset();
                } else { cinTable[currentHandle as usize].status = FMV_EOF }
            } else { cinTable[currentHandle as usize].status = FMV_IDLE }
            return
        }
        framedata =
            framedata.offset(cinTable[currentHandle as usize].RoQFrameSize as
                                 isize);
        cinTable[currentHandle as usize].roq_id =
            (*framedata.offset(0isize) as libc::c_int +
                 *framedata.offset(1isize) as libc::c_int * 256i32) as
                libc::c_uint;
        cinTable[currentHandle as usize].RoQFrameSize =
            (*framedata.offset(2isize) as libc::c_int +
                 *framedata.offset(3isize) as libc::c_int * 256i32 +
                 *framedata.offset(4isize) as libc::c_int * 65536i32) as
                libc::c_uint;
        cinTable[currentHandle as usize].roq_flags =
            (*framedata.offset(6isize) as libc::c_int +
                 *framedata.offset(7isize) as libc::c_int * 256i32) as
                libc::c_long;
        cinTable[currentHandle as usize].roqF0 =
            *framedata.offset(7isize) as libc::c_schar as libc::c_long;
        cinTable[currentHandle as usize].roqF1 =
            *framedata.offset(6isize) as libc::c_schar as libc::c_long;
        if cinTable[currentHandle as usize].RoQFrameSize >
               65536i32 as libc::c_uint ||
               cinTable[currentHandle as usize].roq_id ==
                   0x1084i32 as libc::c_uint {
            Com_DPrintf(b"roq_size>65536||roq_id==0x1084\n\x00" as *const u8
                            as *const libc::c_char);
            cinTable[currentHandle as usize].status = FMV_EOF;
            if 0 != cinTable[currentHandle as usize].looping as u64 {
                RoQReset();
            }
            return
        }
        if !(0 != cinTable[currentHandle as usize].inMemory as libc::c_uint &&
                 cinTable[currentHandle as usize].status as libc::c_uint !=
                     FMV_EOF as libc::c_int as libc::c_uint) {
            break ;
        }
        cinTable[currentHandle as usize].inMemory -= 1;
        framedata = framedata.offset(8isize)
    }
    cinTable[currentHandle as usize].RoQPlayed +=
        cinTable[currentHandle as
                     usize].RoQFrameSize.wrapping_add(8i32 as libc::c_uint) as
            libc::c_long;
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn setupQuad(mut xOff: libc::c_long,
                               mut yOff: libc::c_long) {
    let mut numQuadCels: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut x: libc::c_long = 0;
    let mut y: libc::c_long = 0;
    let mut temp: *mut byte = 0 as *mut byte;
    if xOff == cin.oldXOff && yOff == cin.oldYOff &&
           cinTable[currentHandle as usize].ysize as libc::c_long ==
               cin.oldysize &&
           cinTable[currentHandle as usize].xsize as libc::c_long ==
               cin.oldxsize {
        return
    }
    cin.oldXOff = xOff;
    cin.oldYOff = yOff;
    cin.oldysize = cinTable[currentHandle as usize].ysize as libc::c_long;
    cin.oldxsize = cinTable[currentHandle as usize].xsize as libc::c_long;
    numQuadCels =
        cinTable[currentHandle as
                     usize].xsize.wrapping_mul(cinTable[currentHandle as
                                                            usize].ysize).wrapping_div(16i32
                                                                                           as
                                                                                           libc::c_uint)
            as libc::c_long;
    numQuadCels += numQuadCels / 4i32 as libc::c_long;
    numQuadCels += 64i32 as libc::c_long;
    cinTable[currentHandle as usize].onQuad = 0i32 as libc::c_long;
    y = 0i32 as libc::c_long;
    while y < cinTable[currentHandle as usize].ysize as libc::c_long {
        x = 0i32 as libc::c_long;
        while x < cinTable[currentHandle as usize].xsize as libc::c_long {
            recurseQuad(x, y, 16i32 as libc::c_long, xOff, yOff);
            x += 16i32 as libc::c_long
        }
        y += 16i32 as libc::c_long
    }
    temp = 0 as *mut byte;
    i = numQuadCels - 64i32 as libc::c_long;
    while i < numQuadCels {
        cin.qStatus[0usize][i as usize] = temp;
        cin.qStatus[1usize][i as usize] = temp;
        i += 1
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn recurseQuad(mut startX: libc::c_long,
                                 mut startY: libc::c_long,
                                 mut quadSize: libc::c_long,
                                 mut xOff: libc::c_long,
                                 mut yOff: libc::c_long) {
    let mut scroff: *mut byte = 0 as *mut byte;
    let mut bigx: libc::c_long = 0;
    let mut bigy: libc::c_long = 0;
    let mut lowx: libc::c_long = 0;
    let mut lowy: libc::c_long = 0;
    let mut useY: libc::c_long = 0;
    let mut offset: libc::c_long = 0;
    offset = cinTable[currentHandle as usize].screenDelta;
    lowy = 0i32 as libc::c_long;
    lowx = lowy;
    bigx = cinTable[currentHandle as usize].xsize as libc::c_long;
    bigy = cinTable[currentHandle as usize].ysize as libc::c_long;
    if bigx > cinTable[currentHandle as usize].CIN_WIDTH as libc::c_long {
        bigx = cinTable[currentHandle as usize].CIN_WIDTH as libc::c_long
    }
    if bigy > cinTable[currentHandle as usize].CIN_HEIGHT as libc::c_long {
        bigy = cinTable[currentHandle as usize].CIN_HEIGHT as libc::c_long
    }
    if startX >= lowx && startX + quadSize <= bigx &&
           startY + quadSize <= bigy && startY >= lowy &&
           quadSize <= 8i32 as libc::c_long {
        useY = startY;
        scroff =
            cin.linbuf.as_mut_ptr().offset(((useY +
                                                 (cinTable[currentHandle as
                                                               usize].CIN_HEIGHT
                                                      as libc::c_long - bigy
                                                      >> 1i32) + yOff) *
                                                cinTable[currentHandle as
                                                             usize].samplesPerLine)
                                               as
                                               isize).offset(((startX + xOff)
                                                                  *
                                                                  cinTable[currentHandle
                                                                               as
                                                                               usize].samplesPerPixel)
                                                                 as isize);
        cin.qStatus[0usize][cinTable[currentHandle as usize].onQuad as usize]
            = scroff;
        let fresh0 = cinTable[currentHandle as usize].onQuad;
        cinTable[currentHandle as usize].onQuad =
            cinTable[currentHandle as usize].onQuad + 1;
        cin.qStatus[1usize][fresh0 as usize] = scroff.offset(offset as isize)
    }
    if quadSize != 4i32 as libc::c_long {
        quadSize >>= 1i32;
        recurseQuad(startX, startY, quadSize, xOff, yOff);
        recurseQuad(startX + quadSize, startY, quadSize, xOff, yOff);
        recurseQuad(startX, startY + quadSize, quadSize, xOff, yOff);
        recurseQuad(startX + quadSize, startY + quadSize, quadSize, xOff,
                    yOff);
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn readQuadInfo(mut qData: *mut byte) {
    if currentHandle < 0i32 { return }
    cinTable[currentHandle as usize].xsize =
        (*qData.offset(0isize) as libc::c_int +
             *qData.offset(1isize) as libc::c_int * 256i32) as libc::c_uint;
    cinTable[currentHandle as usize].ysize =
        (*qData.offset(2isize) as libc::c_int +
             *qData.offset(3isize) as libc::c_int * 256i32) as libc::c_uint;
    cinTable[currentHandle as usize].maxsize =
        (*qData.offset(4isize) as libc::c_int +
             *qData.offset(5isize) as libc::c_int * 256i32) as libc::c_uint;
    cinTable[currentHandle as usize].minsize =
        (*qData.offset(6isize) as libc::c_int +
             *qData.offset(7isize) as libc::c_int * 256i32) as libc::c_uint;
    cinTable[currentHandle as usize].CIN_HEIGHT =
        cinTable[currentHandle as usize].ysize as libc::c_int;
    cinTable[currentHandle as usize].CIN_WIDTH =
        cinTable[currentHandle as usize].xsize as libc::c_int;
    cinTable[currentHandle as usize].samplesPerLine =
        cinTable[currentHandle as usize].CIN_WIDTH as libc::c_long *
            cinTable[currentHandle as usize].samplesPerPixel;
    cinTable[currentHandle as usize].screenDelta =
        cinTable[currentHandle as usize].CIN_HEIGHT as libc::c_long *
            cinTable[currentHandle as usize].samplesPerLine;
    cinTable[currentHandle as usize].half = qfalse;
    cinTable[currentHandle as usize].smootheddouble = qfalse;
    cinTable[currentHandle as usize].VQ0 =
        cinTable[currentHandle as usize].VQNormal;
    cinTable[currentHandle as usize].VQ1 =
        cinTable[currentHandle as usize].VQBuffer;
    cinTable[currentHandle as usize].t[0usize] =
        cinTable[currentHandle as usize].screenDelta;
    cinTable[currentHandle as usize].t[1usize] =
        -cinTable[currentHandle as usize].screenDelta;
    cinTable[currentHandle as usize].drawX =
        cinTable[currentHandle as usize].CIN_WIDTH as libc::c_long;
    cinTable[currentHandle as usize].drawY =
        cinTable[currentHandle as usize].CIN_HEIGHT as libc::c_long;
    if cls.glconfig.hardwareType as libc::c_uint ==
           GLHW_RAGEPRO as libc::c_int as libc::c_uint ||
           cls.glconfig.maxTextureSize <= 256i32 {
        if cinTable[currentHandle as usize].drawX > 256i32 as libc::c_long {
            cinTable[currentHandle as usize].drawX = 256i32 as libc::c_long
        }
        if cinTable[currentHandle as usize].drawY > 256i32 as libc::c_long {
            cinTable[currentHandle as usize].drawY = 256i32 as libc::c_long
        }
        if cinTable[currentHandle as usize].CIN_WIDTH != 256i32 ||
               cinTable[currentHandle as usize].CIN_HEIGHT != 256i32 {
            Com_Printf(b"HACK: approxmimating cinematic for Rage Pro or Voodoo\n\x00"
                           as *const u8 as *const libc::c_char);
        }
    };
}
//-----------------------------------------------------------------------------
// RllDecodeStereoToStereo
//
// Decode stereo source data into a stereo buffer.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= 1/2 # of bytes of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn RllDecodeStereoToStereo(mut from: *mut libc::c_uchar,
                                                 mut to: *mut libc::c_short,
                                                 mut size: libc::c_uint,
                                                 mut signedOutput:
                                                     libc::c_char,
                                                 mut flag: libc::c_ushort)
 -> libc::c_long {
    let mut z: libc::c_uint = 0;
    let mut zz: *mut libc::c_uchar = from;
    let mut prevL: libc::c_int = 0;
    let mut prevR: libc::c_int = 0;
    if 0 != signedOutput {
        prevL = (flag as libc::c_int & 0xff00i32) - 0x8000i32;
        prevR = ((flag as libc::c_int & 0xffi32) << 8i32) - 0x8000i32
    } else {
        prevL = flag as libc::c_int & 0xff00i32;
        prevR = (flag as libc::c_int & 0xffi32) << 8i32
    }
    z = 0i32 as libc::c_uint;
    while z < size {
        let fresh1 = zz;
        zz = zz.offset(1);
        prevL =
            (prevL + cin.sqrTable[*fresh1 as usize] as libc::c_int) as
                libc::c_short as libc::c_int;
        let fresh2 = zz;
        zz = zz.offset(1);
        prevR =
            (prevR + cin.sqrTable[*fresh2 as usize] as libc::c_int) as
                libc::c_short as libc::c_int;
        *to.offset(z.wrapping_add(0i32 as libc::c_uint) as isize) =
            prevL as libc::c_short;
        *to.offset(z.wrapping_add(1i32 as libc::c_uint) as isize) =
            prevR as libc::c_short;
        z = z.wrapping_add(2i32 as libc::c_uint)
    }
    return (size >> 1i32) as libc::c_long;
}
//-----------------------------------------------------------------------------
// RllDecodeMonoToStereo
//
// Decode mono source data into a stereo buffer. Output is 4 times the number
// of bytes in the input.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= 1/4 # of bytes of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn RllDecodeMonoToStereo(mut from: *mut libc::c_uchar,
                                               mut to: *mut libc::c_short,
                                               mut size: libc::c_uint,
                                               mut signedOutput: libc::c_char,
                                               mut flag: libc::c_ushort)
 -> libc::c_long {
    let mut z: libc::c_uint = 0;
    let mut prev: libc::c_int = 0;
    if 0 != signedOutput {
        prev = flag as libc::c_int - 0x8000i32
    } else { prev = flag as libc::c_int }
    z = 0i32 as libc::c_uint;
    while z < size {
        prev =
            (prev +
                 cin.sqrTable[*from.offset(z as isize) as usize] as
                     libc::c_int) as libc::c_short as libc::c_int;
        let ref mut fresh3 =
            *to.offset(z.wrapping_mul(2i32 as
                                          libc::c_uint).wrapping_add(1i32 as
                                                                         libc::c_uint)
                           as isize);
        *fresh3 = prev as libc::c_short;
        *to.offset(z.wrapping_mul(2i32 as
                                      libc::c_uint).wrapping_add(0i32 as
                                                                     libc::c_uint)
                       as isize) = *fresh3;
        z = z.wrapping_add(1)
    }
    return size as libc::c_long;
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn decodeCodeBook(mut input: *mut byte,
                                    mut roq_flags: libc::c_ushort) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut two: libc::c_long = 0;
    let mut four: libc::c_long = 0;
    let mut aptr: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut bptr: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut cptr: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut dptr: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut y0: libc::c_long = 0;
    let mut y1: libc::c_long = 0;
    let mut y2: libc::c_long = 0;
    let mut y3: libc::c_long = 0;
    let mut cr: libc::c_long = 0;
    let mut cb: libc::c_long = 0;
    let mut bbptr: *mut byte = 0 as *mut byte;
    let mut baptr: *mut byte = 0 as *mut byte;
    let mut bcptr: *mut byte = 0 as *mut byte;
    let mut bdptr: *mut byte = 0 as *mut byte;
    let mut iaptr: unnamed_3 = unnamed_3{i: 0 as *mut libc::c_uint,};
    let mut ibptr: unnamed_3 = unnamed_3{i: 0 as *mut libc::c_uint,};
    let mut icptr: unnamed_3 = unnamed_3{i: 0 as *mut libc::c_uint,};
    let mut idptr: unnamed_3 = unnamed_3{i: 0 as *mut libc::c_uint,};
    if 0 == roq_flags {
        four = 256i32 as libc::c_long;
        two = four
    } else {
        two = (roq_flags as libc::c_int >> 8i32) as libc::c_long;
        if 0 == two { two = 256i32 as libc::c_long }
        four = (roq_flags as libc::c_int & 0xffi32) as libc::c_long
    }
    four *= 2i32 as libc::c_long;
    bptr = vq2.as_mut_ptr();
    if 0 == cinTable[currentHandle as usize].half as u64 {
        if 0 == cinTable[currentHandle as usize].smootheddouble as u64 {
            if cinTable[currentHandle as usize].samplesPerPixel ==
                   2i32 as libc::c_long {
                i = 0i32 as libc::c_long;
                while i < two {
                    let fresh4 = input;
                    input = input.offset(1);
                    y0 = *fresh4 as libc::c_long;
                    let fresh5 = input;
                    input = input.offset(1);
                    y1 = *fresh5 as libc::c_long;
                    let fresh6 = input;
                    input = input.offset(1);
                    y2 = *fresh6 as libc::c_long;
                    let fresh7 = input;
                    input = input.offset(1);
                    y3 = *fresh7 as libc::c_long;
                    let fresh8 = input;
                    input = input.offset(1);
                    cr = *fresh8 as libc::c_long;
                    let fresh9 = input;
                    input = input.offset(1);
                    cb = *fresh9 as libc::c_long;
                    let fresh10 = bptr;
                    bptr = bptr.offset(1);
                    *fresh10 = yuv_to_rgb(y0, cr, cb);
                    let fresh11 = bptr;
                    bptr = bptr.offset(1);
                    *fresh11 = yuv_to_rgb(y1, cr, cb);
                    let fresh12 = bptr;
                    bptr = bptr.offset(1);
                    *fresh12 = yuv_to_rgb(y2, cr, cb);
                    let fresh13 = bptr;
                    bptr = bptr.offset(1);
                    *fresh13 = yuv_to_rgb(y3, cr, cb);
                    i += 1
                }
                cptr = vq4.as_mut_ptr();
                dptr = vq8.as_mut_ptr();
                i = 0i32 as libc::c_long;
                while i < four {
                    let fresh14 = input;
                    input = input.offset(1);
                    aptr =
                        vq2.as_mut_ptr().offset((*fresh14 as libc::c_int *
                                                     4i32) as isize);
                    let fresh15 = input;
                    input = input.offset(1);
                    bptr =
                        vq2.as_mut_ptr().offset((*fresh15 as libc::c_int *
                                                     4i32) as isize);
                    j = 0i32 as libc::c_long;
                    while j < 2i32 as libc::c_long {
                        let fresh16 = cptr;
                        cptr = cptr.offset(1);
                        *fresh16 = *aptr.offset(0isize);
                        let fresh17 = dptr;
                        dptr = dptr.offset(1);
                        *fresh17 = *aptr.offset(0isize);
                        let fresh18 = dptr;
                        dptr = dptr.offset(1);
                        *fresh18 = *aptr.offset(0isize);
                        let fresh19 = cptr;
                        cptr = cptr.offset(1);
                        *fresh19 = *aptr.offset(1isize);
                        let fresh20 = dptr;
                        dptr = dptr.offset(1);
                        *fresh20 = *aptr.offset(1isize);
                        let fresh21 = dptr;
                        dptr = dptr.offset(1);
                        *fresh21 = *aptr.offset(1isize);
                        let fresh22 = cptr;
                        cptr = cptr.offset(1);
                        *fresh22 = *bptr.offset(0isize);
                        let fresh23 = dptr;
                        dptr = dptr.offset(1);
                        *fresh23 = *bptr.offset(0isize);
                        let fresh24 = dptr;
                        dptr = dptr.offset(1);
                        *fresh24 = *bptr.offset(0isize);
                        let fresh25 = cptr;
                        cptr = cptr.offset(1);
                        *fresh25 = *bptr.offset(1isize);
                        let fresh26 = dptr;
                        dptr = dptr.offset(1);
                        *fresh26 = *bptr.offset(1isize);
                        let fresh27 = dptr;
                        dptr = dptr.offset(1);
                        *fresh27 = *bptr.offset(1isize);
                        let fresh28 = dptr;
                        dptr = dptr.offset(1);
                        *fresh28 = *aptr.offset(0isize);
                        let fresh29 = dptr;
                        dptr = dptr.offset(1);
                        *fresh29 = *aptr.offset(0isize);
                        let fresh30 = dptr;
                        dptr = dptr.offset(1);
                        *fresh30 = *aptr.offset(1isize);
                        let fresh31 = dptr;
                        dptr = dptr.offset(1);
                        *fresh31 = *aptr.offset(1isize);
                        let fresh32 = dptr;
                        dptr = dptr.offset(1);
                        *fresh32 = *bptr.offset(0isize);
                        let fresh33 = dptr;
                        dptr = dptr.offset(1);
                        *fresh33 = *bptr.offset(0isize);
                        let fresh34 = dptr;
                        dptr = dptr.offset(1);
                        *fresh34 = *bptr.offset(1isize);
                        let fresh35 = dptr;
                        dptr = dptr.offset(1);
                        *fresh35 = *bptr.offset(1isize);
                        aptr = aptr.offset(2isize);
                        bptr = bptr.offset(2isize);
                        j += 1
                    }
                    i += 1
                }
            } else if cinTable[currentHandle as usize].samplesPerPixel ==
                          4i32 as libc::c_long {
                ibptr.s = bptr;
                i = 0i32 as libc::c_long;
                while i < two {
                    let fresh36 = input;
                    input = input.offset(1);
                    y0 = *fresh36 as libc::c_long;
                    let fresh37 = input;
                    input = input.offset(1);
                    y1 = *fresh37 as libc::c_long;
                    let fresh38 = input;
                    input = input.offset(1);
                    y2 = *fresh38 as libc::c_long;
                    let fresh39 = input;
                    input = input.offset(1);
                    y3 = *fresh39 as libc::c_long;
                    let fresh40 = input;
                    input = input.offset(1);
                    cr = *fresh40 as libc::c_long;
                    let fresh41 = input;
                    input = input.offset(1);
                    cb = *fresh41 as libc::c_long;
                    let fresh42 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh42 = yuv_to_rgb24(y0, cr, cb);
                    let fresh43 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh43 = yuv_to_rgb24(y1, cr, cb);
                    let fresh44 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh44 = yuv_to_rgb24(y2, cr, cb);
                    let fresh45 = ibptr.i;
                    ibptr.i = ibptr.i.offset(1);
                    *fresh45 = yuv_to_rgb24(y3, cr, cb);
                    i += 1
                }
                icptr.s = vq4.as_mut_ptr();
                idptr.s = vq8.as_mut_ptr();
                i = 0i32 as libc::c_long;
                while i < four {
                    iaptr.s = vq2.as_mut_ptr();
                    let fresh46 = input;
                    input = input.offset(1);
                    iaptr.i =
                        iaptr.i.offset((*fresh46 as libc::c_int * 4i32) as
                                           isize);
                    ibptr.s = vq2.as_mut_ptr();
                    let fresh47 = input;
                    input = input.offset(1);
                    ibptr.i =
                        ibptr.i.offset((*fresh47 as libc::c_int * 4i32) as
                                           isize);
                    j = 0i32 as libc::c_long;
                    while j < 2i32 as libc::c_long {
                        let fresh48 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh48 = *iaptr.i.offset(0isize);
                        let fresh49 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh49 = *iaptr.i.offset(0isize);
                        let fresh50 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh50 = *iaptr.i.offset(0isize);
                        let fresh51 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh51 = *iaptr.i.offset(1isize);
                        let fresh52 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh52 = *iaptr.i.offset(1isize);
                        let fresh53 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh53 = *iaptr.i.offset(1isize);
                        let fresh54 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh54 = *ibptr.i.offset(0isize);
                        let fresh55 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh55 = *ibptr.i.offset(0isize);
                        let fresh56 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh56 = *ibptr.i.offset(0isize);
                        let fresh57 = icptr.i;
                        icptr.i = icptr.i.offset(1);
                        *fresh57 = *ibptr.i.offset(1isize);
                        let fresh58 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh58 = *ibptr.i.offset(1isize);
                        let fresh59 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh59 = *ibptr.i.offset(1isize);
                        let fresh60 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh60 = *iaptr.i.offset(0isize);
                        let fresh61 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh61 = *iaptr.i.offset(0isize);
                        let fresh62 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh62 = *iaptr.i.offset(1isize);
                        let fresh63 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh63 = *iaptr.i.offset(1isize);
                        let fresh64 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh64 = *ibptr.i.offset(0isize);
                        let fresh65 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh65 = *ibptr.i.offset(0isize);
                        let fresh66 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh66 = *ibptr.i.offset(1isize);
                        let fresh67 = idptr.i;
                        idptr.i = idptr.i.offset(1);
                        *fresh67 = *ibptr.i.offset(1isize);
                        iaptr.i = iaptr.i.offset(2isize);
                        ibptr.i = ibptr.i.offset(2isize);
                        j += 1
                    }
                    i += 1
                }
            } else if cinTable[currentHandle as usize].samplesPerPixel ==
                          1i32 as libc::c_long {
                bbptr = bptr as *mut byte;
                i = 0i32 as libc::c_long;
                while i < two {
                    let fresh69 = bbptr;
                    bbptr = bbptr.offset(1);
                    let fresh68 = input;
                    input = input.offset(1);
                    *fresh69 =
                        *cinTable[currentHandle as
                                      usize].gray.offset(*fresh68 as isize);
                    let fresh71 = bbptr;
                    bbptr = bbptr.offset(1);
                    let fresh70 = input;
                    input = input.offset(1);
                    *fresh71 =
                        *cinTable[currentHandle as
                                      usize].gray.offset(*fresh70 as isize);
                    let fresh73 = bbptr;
                    bbptr = bbptr.offset(1);
                    let fresh72 = input;
                    input = input.offset(1);
                    *fresh73 =
                        *cinTable[currentHandle as
                                      usize].gray.offset(*fresh72 as isize);
                    let fresh74 = bbptr;
                    bbptr = bbptr.offset(1);
                    *fresh74 =
                        *cinTable[currentHandle as
                                      usize].gray.offset(*input as isize);
                    input = input.offset(3isize);
                    i += 1
                }
                bcptr = vq4.as_mut_ptr() as *mut byte;
                bdptr = vq8.as_mut_ptr() as *mut byte;
                i = 0i32 as libc::c_long;
                while i < four {
                    let fresh75 = input;
                    input = input.offset(1);
                    baptr =
                        (vq2.as_mut_ptr() as
                             *mut byte).offset((*fresh75 as libc::c_int *
                                                    4i32) as isize);
                    let fresh76 = input;
                    input = input.offset(1);
                    bbptr =
                        (vq2.as_mut_ptr() as
                             *mut byte).offset((*fresh76 as libc::c_int *
                                                    4i32) as isize);
                    j = 0i32 as libc::c_long;
                    while j < 2i32 as libc::c_long {
                        let fresh77 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh77 = *baptr.offset(0isize);
                        let fresh78 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh78 = *baptr.offset(0isize);
                        let fresh79 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh79 = *baptr.offset(0isize);
                        let fresh80 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh80 = *baptr.offset(1isize);
                        let fresh81 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh81 = *baptr.offset(1isize);
                        let fresh82 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh82 = *baptr.offset(1isize);
                        let fresh83 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh83 = *bbptr.offset(0isize);
                        let fresh84 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh84 = *bbptr.offset(0isize);
                        let fresh85 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh85 = *bbptr.offset(0isize);
                        let fresh86 = bcptr;
                        bcptr = bcptr.offset(1);
                        *fresh86 = *bbptr.offset(1isize);
                        let fresh87 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh87 = *bbptr.offset(1isize);
                        let fresh88 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh88 = *bbptr.offset(1isize);
                        let fresh89 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh89 = *baptr.offset(0isize);
                        let fresh90 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh90 = *baptr.offset(0isize);
                        let fresh91 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh91 = *baptr.offset(1isize);
                        let fresh92 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh92 = *baptr.offset(1isize);
                        let fresh93 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh93 = *bbptr.offset(0isize);
                        let fresh94 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh94 = *bbptr.offset(0isize);
                        let fresh95 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh95 = *bbptr.offset(1isize);
                        let fresh96 = bdptr;
                        bdptr = bdptr.offset(1);
                        *fresh96 = *bbptr.offset(1isize);
                        baptr = baptr.offset(2isize);
                        bbptr = bbptr.offset(2isize);
                        j += 1
                    }
                    i += 1
                }
            }
        } else if cinTable[currentHandle as usize].samplesPerPixel ==
                      2i32 as libc::c_long {
            i = 0i32 as libc::c_long;
            while i < two {
                let fresh97 = input;
                input = input.offset(1);
                y0 = *fresh97 as libc::c_long;
                let fresh98 = input;
                input = input.offset(1);
                y1 = *fresh98 as libc::c_long;
                let fresh99 = input;
                input = input.offset(1);
                y2 = *fresh99 as libc::c_long;
                let fresh100 = input;
                input = input.offset(1);
                y3 = *fresh100 as libc::c_long;
                let fresh101 = input;
                input = input.offset(1);
                cr = *fresh101 as libc::c_long;
                let fresh102 = input;
                input = input.offset(1);
                cb = *fresh102 as libc::c_long;
                let fresh103 = bptr;
                bptr = bptr.offset(1);
                *fresh103 = yuv_to_rgb(y0, cr, cb);
                let fresh104 = bptr;
                bptr = bptr.offset(1);
                *fresh104 = yuv_to_rgb(y1, cr, cb);
                let fresh105 = bptr;
                bptr = bptr.offset(1);
                *fresh105 =
                    yuv_to_rgb((y0 * 3i32 as libc::c_long + y2) /
                                   4i32 as libc::c_long, cr, cb);
                let fresh106 = bptr;
                bptr = bptr.offset(1);
                *fresh106 =
                    yuv_to_rgb((y1 * 3i32 as libc::c_long + y3) /
                                   4i32 as libc::c_long, cr, cb);
                let fresh107 = bptr;
                bptr = bptr.offset(1);
                *fresh107 =
                    yuv_to_rgb((y0 + y2 * 3i32 as libc::c_long) /
                                   4i32 as libc::c_long, cr, cb);
                let fresh108 = bptr;
                bptr = bptr.offset(1);
                *fresh108 =
                    yuv_to_rgb((y1 + y3 * 3i32 as libc::c_long) /
                                   4i32 as libc::c_long, cr, cb);
                let fresh109 = bptr;
                bptr = bptr.offset(1);
                *fresh109 = yuv_to_rgb(y2, cr, cb);
                let fresh110 = bptr;
                bptr = bptr.offset(1);
                *fresh110 = yuv_to_rgb(y3, cr, cb);
                i += 1
            }
            cptr = vq4.as_mut_ptr();
            dptr = vq8.as_mut_ptr();
            i = 0i32 as libc::c_long;
            while i < four {
                let fresh111 = input;
                input = input.offset(1);
                aptr =
                    vq2.as_mut_ptr().offset((*fresh111 as libc::c_int * 8i32)
                                                as isize);
                let fresh112 = input;
                input = input.offset(1);
                bptr =
                    vq2.as_mut_ptr().offset((*fresh112 as libc::c_int * 8i32)
                                                as isize);
                j = 0i32 as libc::c_long;
                while j < 2i32 as libc::c_long {
                    let fresh113 = cptr;
                    cptr = cptr.offset(1);
                    *fresh113 = *aptr.offset(0isize);
                    let fresh114 = dptr;
                    dptr = dptr.offset(1);
                    *fresh114 = *aptr.offset(0isize);
                    let fresh115 = dptr;
                    dptr = dptr.offset(1);
                    *fresh115 = *aptr.offset(0isize);
                    let fresh116 = cptr;
                    cptr = cptr.offset(1);
                    *fresh116 = *aptr.offset(1isize);
                    let fresh117 = dptr;
                    dptr = dptr.offset(1);
                    *fresh117 = *aptr.offset(1isize);
                    let fresh118 = dptr;
                    dptr = dptr.offset(1);
                    *fresh118 = *aptr.offset(1isize);
                    let fresh119 = cptr;
                    cptr = cptr.offset(1);
                    *fresh119 = *bptr.offset(0isize);
                    let fresh120 = dptr;
                    dptr = dptr.offset(1);
                    *fresh120 = *bptr.offset(0isize);
                    let fresh121 = dptr;
                    dptr = dptr.offset(1);
                    *fresh121 = *bptr.offset(0isize);
                    let fresh122 = cptr;
                    cptr = cptr.offset(1);
                    *fresh122 = *bptr.offset(1isize);
                    let fresh123 = dptr;
                    dptr = dptr.offset(1);
                    *fresh123 = *bptr.offset(1isize);
                    let fresh124 = dptr;
                    dptr = dptr.offset(1);
                    *fresh124 = *bptr.offset(1isize);
                    let fresh125 = dptr;
                    dptr = dptr.offset(1);
                    *fresh125 = *aptr.offset(0isize);
                    let fresh126 = dptr;
                    dptr = dptr.offset(1);
                    *fresh126 = *aptr.offset(0isize);
                    let fresh127 = dptr;
                    dptr = dptr.offset(1);
                    *fresh127 = *aptr.offset(1isize);
                    let fresh128 = dptr;
                    dptr = dptr.offset(1);
                    *fresh128 = *aptr.offset(1isize);
                    let fresh129 = dptr;
                    dptr = dptr.offset(1);
                    *fresh129 = *bptr.offset(0isize);
                    let fresh130 = dptr;
                    dptr = dptr.offset(1);
                    *fresh130 = *bptr.offset(0isize);
                    let fresh131 = dptr;
                    dptr = dptr.offset(1);
                    *fresh131 = *bptr.offset(1isize);
                    let fresh132 = dptr;
                    dptr = dptr.offset(1);
                    *fresh132 = *bptr.offset(1isize);
                    aptr = aptr.offset(2isize);
                    bptr = bptr.offset(2isize);
                    let fresh133 = cptr;
                    cptr = cptr.offset(1);
                    *fresh133 = *aptr.offset(0isize);
                    let fresh134 = dptr;
                    dptr = dptr.offset(1);
                    *fresh134 = *aptr.offset(0isize);
                    let fresh135 = dptr;
                    dptr = dptr.offset(1);
                    *fresh135 = *aptr.offset(0isize);
                    let fresh136 = cptr;
                    cptr = cptr.offset(1);
                    *fresh136 = *aptr.offset(1isize);
                    let fresh137 = dptr;
                    dptr = dptr.offset(1);
                    *fresh137 = *aptr.offset(1isize);
                    let fresh138 = dptr;
                    dptr = dptr.offset(1);
                    *fresh138 = *aptr.offset(1isize);
                    let fresh139 = cptr;
                    cptr = cptr.offset(1);
                    *fresh139 = *bptr.offset(0isize);
                    let fresh140 = dptr;
                    dptr = dptr.offset(1);
                    *fresh140 = *bptr.offset(0isize);
                    let fresh141 = dptr;
                    dptr = dptr.offset(1);
                    *fresh141 = *bptr.offset(0isize);
                    let fresh142 = cptr;
                    cptr = cptr.offset(1);
                    *fresh142 = *bptr.offset(1isize);
                    let fresh143 = dptr;
                    dptr = dptr.offset(1);
                    *fresh143 = *bptr.offset(1isize);
                    let fresh144 = dptr;
                    dptr = dptr.offset(1);
                    *fresh144 = *bptr.offset(1isize);
                    let fresh145 = dptr;
                    dptr = dptr.offset(1);
                    *fresh145 = *aptr.offset(0isize);
                    let fresh146 = dptr;
                    dptr = dptr.offset(1);
                    *fresh146 = *aptr.offset(0isize);
                    let fresh147 = dptr;
                    dptr = dptr.offset(1);
                    *fresh147 = *aptr.offset(1isize);
                    let fresh148 = dptr;
                    dptr = dptr.offset(1);
                    *fresh148 = *aptr.offset(1isize);
                    let fresh149 = dptr;
                    dptr = dptr.offset(1);
                    *fresh149 = *bptr.offset(0isize);
                    let fresh150 = dptr;
                    dptr = dptr.offset(1);
                    *fresh150 = *bptr.offset(0isize);
                    let fresh151 = dptr;
                    dptr = dptr.offset(1);
                    *fresh151 = *bptr.offset(1isize);
                    let fresh152 = dptr;
                    dptr = dptr.offset(1);
                    *fresh152 = *bptr.offset(1isize);
                    aptr = aptr.offset(2isize);
                    bptr = bptr.offset(2isize);
                    j += 1
                }
                i += 1
            }
        } else if cinTable[currentHandle as usize].samplesPerPixel ==
                      4i32 as libc::c_long {
            ibptr.s = bptr;
            i = 0i32 as libc::c_long;
            while i < two {
                let fresh153 = input;
                input = input.offset(1);
                y0 = *fresh153 as libc::c_long;
                let fresh154 = input;
                input = input.offset(1);
                y1 = *fresh154 as libc::c_long;
                let fresh155 = input;
                input = input.offset(1);
                y2 = *fresh155 as libc::c_long;
                let fresh156 = input;
                input = input.offset(1);
                y3 = *fresh156 as libc::c_long;
                let fresh157 = input;
                input = input.offset(1);
                cr = *fresh157 as libc::c_long;
                let fresh158 = input;
                input = input.offset(1);
                cb = *fresh158 as libc::c_long;
                let fresh159 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh159 = yuv_to_rgb24(y0, cr, cb);
                let fresh160 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh160 = yuv_to_rgb24(y1, cr, cb);
                let fresh161 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh161 =
                    yuv_to_rgb24((y0 * 3i32 as libc::c_long + y2) /
                                     4i32 as libc::c_long, cr, cb);
                let fresh162 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh162 =
                    yuv_to_rgb24((y1 * 3i32 as libc::c_long + y3) /
                                     4i32 as libc::c_long, cr, cb);
                let fresh163 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh163 =
                    yuv_to_rgb24((y0 + y2 * 3i32 as libc::c_long) /
                                     4i32 as libc::c_long, cr, cb);
                let fresh164 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh164 =
                    yuv_to_rgb24((y1 + y3 * 3i32 as libc::c_long) /
                                     4i32 as libc::c_long, cr, cb);
                let fresh165 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh165 = yuv_to_rgb24(y2, cr, cb);
                let fresh166 = ibptr.i;
                ibptr.i = ibptr.i.offset(1);
                *fresh166 = yuv_to_rgb24(y3, cr, cb);
                i += 1
            }
            icptr.s = vq4.as_mut_ptr();
            idptr.s = vq8.as_mut_ptr();
            i = 0i32 as libc::c_long;
            while i < four {
                iaptr.s = vq2.as_mut_ptr();
                let fresh167 = input;
                input = input.offset(1);
                iaptr.i =
                    iaptr.i.offset((*fresh167 as libc::c_int * 8i32) as
                                       isize);
                ibptr.s = vq2.as_mut_ptr();
                let fresh168 = input;
                input = input.offset(1);
                ibptr.i =
                    ibptr.i.offset((*fresh168 as libc::c_int * 8i32) as
                                       isize);
                j = 0i32 as libc::c_long;
                while j < 2i32 as libc::c_long {
                    let fresh169 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh169 = *iaptr.i.offset(0isize);
                    let fresh170 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh170 = *iaptr.i.offset(0isize);
                    let fresh171 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh171 = *iaptr.i.offset(0isize);
                    let fresh172 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh172 = *iaptr.i.offset(1isize);
                    let fresh173 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh173 = *iaptr.i.offset(1isize);
                    let fresh174 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh174 = *iaptr.i.offset(1isize);
                    let fresh175 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh175 = *ibptr.i.offset(0isize);
                    let fresh176 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh176 = *ibptr.i.offset(0isize);
                    let fresh177 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh177 = *ibptr.i.offset(0isize);
                    let fresh178 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh178 = *ibptr.i.offset(1isize);
                    let fresh179 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh179 = *ibptr.i.offset(1isize);
                    let fresh180 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh180 = *ibptr.i.offset(1isize);
                    let fresh181 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh181 = *iaptr.i.offset(0isize);
                    let fresh182 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh182 = *iaptr.i.offset(0isize);
                    let fresh183 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh183 = *iaptr.i.offset(1isize);
                    let fresh184 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh184 = *iaptr.i.offset(1isize);
                    let fresh185 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh185 = *ibptr.i.offset(0isize);
                    let fresh186 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh186 = *ibptr.i.offset(0isize);
                    let fresh187 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh187 = *ibptr.i.offset(1isize);
                    let fresh188 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh188 = *ibptr.i.offset(1isize);
                    iaptr.i = iaptr.i.offset(2isize);
                    ibptr.i = ibptr.i.offset(2isize);
                    let fresh189 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh189 = *iaptr.i.offset(0isize);
                    let fresh190 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh190 = *iaptr.i.offset(0isize);
                    let fresh191 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh191 = *iaptr.i.offset(0isize);
                    let fresh192 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh192 = *iaptr.i.offset(1isize);
                    let fresh193 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh193 = *iaptr.i.offset(1isize);
                    let fresh194 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh194 = *iaptr.i.offset(1isize);
                    let fresh195 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh195 = *ibptr.i.offset(0isize);
                    let fresh196 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh196 = *ibptr.i.offset(0isize);
                    let fresh197 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh197 = *ibptr.i.offset(0isize);
                    let fresh198 = icptr.i;
                    icptr.i = icptr.i.offset(1);
                    *fresh198 = *ibptr.i.offset(1isize);
                    let fresh199 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh199 = *ibptr.i.offset(1isize);
                    let fresh200 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh200 = *ibptr.i.offset(1isize);
                    let fresh201 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh201 = *iaptr.i.offset(0isize);
                    let fresh202 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh202 = *iaptr.i.offset(0isize);
                    let fresh203 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh203 = *iaptr.i.offset(1isize);
                    let fresh204 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh204 = *iaptr.i.offset(1isize);
                    let fresh205 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh205 = *ibptr.i.offset(0isize);
                    let fresh206 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh206 = *ibptr.i.offset(0isize);
                    let fresh207 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh207 = *ibptr.i.offset(1isize);
                    let fresh208 = idptr.i;
                    idptr.i = idptr.i.offset(1);
                    *fresh208 = *ibptr.i.offset(1isize);
                    iaptr.i = iaptr.i.offset(2isize);
                    ibptr.i = ibptr.i.offset(2isize);
                    j += 1
                }
                i += 1
            }
        } else if cinTable[currentHandle as usize].samplesPerPixel ==
                      1i32 as libc::c_long {
            bbptr = bptr as *mut byte;
            i = 0i32 as libc::c_long;
            while i < two {
                let fresh209 = input;
                input = input.offset(1);
                y0 = *fresh209 as libc::c_long;
                let fresh210 = input;
                input = input.offset(1);
                y1 = *fresh210 as libc::c_long;
                let fresh211 = input;
                input = input.offset(1);
                y2 = *fresh211 as libc::c_long;
                y3 = *input as libc::c_long;
                input = input.offset(3isize);
                let fresh212 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh212 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(y0 as isize);
                let fresh213 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh213 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(y1 as isize);
                let fresh214 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh214 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(((y0 *
                                                           3i32 as
                                                               libc::c_long +
                                                           y2) /
                                                          4i32 as
                                                              libc::c_long) as
                                                         isize);
                let fresh215 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh215 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(((y1 *
                                                           3i32 as
                                                               libc::c_long +
                                                           y3) /
                                                          4i32 as
                                                              libc::c_long) as
                                                         isize);
                let fresh216 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh216 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(((y0 +
                                                           y2 *
                                                               3i32 as
                                                                   libc::c_long)
                                                          /
                                                          4i32 as
                                                              libc::c_long) as
                                                         isize);
                let fresh217 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh217 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(((y1 +
                                                           y3 *
                                                               3i32 as
                                                                   libc::c_long)
                                                          /
                                                          4i32 as
                                                              libc::c_long) as
                                                         isize);
                let fresh218 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh218 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(y2 as isize);
                let fresh219 = bbptr;
                bbptr = bbptr.offset(1);
                *fresh219 =
                    *cinTable[currentHandle as
                                  usize].gray.offset(y3 as isize);
                i += 1
            }
            bcptr = vq4.as_mut_ptr() as *mut byte;
            bdptr = vq8.as_mut_ptr() as *mut byte;
            i = 0i32 as libc::c_long;
            while i < four {
                let fresh220 = input;
                input = input.offset(1);
                baptr =
                    (vq2.as_mut_ptr() as
                         *mut byte).offset((*fresh220 as libc::c_int * 8i32)
                                               as isize);
                let fresh221 = input;
                input = input.offset(1);
                bbptr =
                    (vq2.as_mut_ptr() as
                         *mut byte).offset((*fresh221 as libc::c_int * 8i32)
                                               as isize);
                j = 0i32 as libc::c_long;
                while j < 2i32 as libc::c_long {
                    let fresh222 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh222 = *baptr.offset(0isize);
                    let fresh223 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh223 = *baptr.offset(0isize);
                    let fresh224 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh224 = *baptr.offset(0isize);
                    let fresh225 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh225 = *baptr.offset(1isize);
                    let fresh226 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh226 = *baptr.offset(1isize);
                    let fresh227 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh227 = *baptr.offset(1isize);
                    let fresh228 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh228 = *bbptr.offset(0isize);
                    let fresh229 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh229 = *bbptr.offset(0isize);
                    let fresh230 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh230 = *bbptr.offset(0isize);
                    let fresh231 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh231 = *bbptr.offset(1isize);
                    let fresh232 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh232 = *bbptr.offset(1isize);
                    let fresh233 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh233 = *bbptr.offset(1isize);
                    let fresh234 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh234 = *baptr.offset(0isize);
                    let fresh235 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh235 = *baptr.offset(0isize);
                    let fresh236 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh236 = *baptr.offset(1isize);
                    let fresh237 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh237 = *baptr.offset(1isize);
                    let fresh238 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh238 = *bbptr.offset(0isize);
                    let fresh239 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh239 = *bbptr.offset(0isize);
                    let fresh240 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh240 = *bbptr.offset(1isize);
                    let fresh241 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh241 = *bbptr.offset(1isize);
                    baptr = baptr.offset(2isize);
                    bbptr = bbptr.offset(2isize);
                    let fresh242 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh242 = *baptr.offset(0isize);
                    let fresh243 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh243 = *baptr.offset(0isize);
                    let fresh244 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh244 = *baptr.offset(0isize);
                    let fresh245 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh245 = *baptr.offset(1isize);
                    let fresh246 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh246 = *baptr.offset(1isize);
                    let fresh247 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh247 = *baptr.offset(1isize);
                    let fresh248 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh248 = *bbptr.offset(0isize);
                    let fresh249 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh249 = *bbptr.offset(0isize);
                    let fresh250 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh250 = *bbptr.offset(0isize);
                    let fresh251 = bcptr;
                    bcptr = bcptr.offset(1);
                    *fresh251 = *bbptr.offset(1isize);
                    let fresh252 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh252 = *bbptr.offset(1isize);
                    let fresh253 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh253 = *bbptr.offset(1isize);
                    let fresh254 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh254 = *baptr.offset(0isize);
                    let fresh255 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh255 = *baptr.offset(0isize);
                    let fresh256 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh256 = *baptr.offset(1isize);
                    let fresh257 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh257 = *baptr.offset(1isize);
                    let fresh258 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh258 = *bbptr.offset(0isize);
                    let fresh259 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh259 = *bbptr.offset(0isize);
                    let fresh260 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh260 = *bbptr.offset(1isize);
                    let fresh261 = bdptr;
                    bdptr = bdptr.offset(1);
                    *fresh261 = *bbptr.offset(1isize);
                    baptr = baptr.offset(2isize);
                    bbptr = bbptr.offset(2isize);
                    j += 1
                }
                i += 1
            }
        }
    } else if cinTable[currentHandle as usize].samplesPerPixel ==
                  2i32 as libc::c_long {
        i = 0i32 as libc::c_long;
        while i < two {
            y0 = *input as libc::c_long;
            input = input.offset(2isize);
            y2 = *input as libc::c_long;
            input = input.offset(2isize);
            let fresh262 = input;
            input = input.offset(1);
            cr = *fresh262 as libc::c_long;
            let fresh263 = input;
            input = input.offset(1);
            cb = *fresh263 as libc::c_long;
            let fresh264 = bptr;
            bptr = bptr.offset(1);
            *fresh264 = yuv_to_rgb(y0, cr, cb);
            let fresh265 = bptr;
            bptr = bptr.offset(1);
            *fresh265 = yuv_to_rgb(y2, cr, cb);
            i += 1
        }
        cptr = vq4.as_mut_ptr();
        dptr = vq8.as_mut_ptr();
        i = 0i32 as libc::c_long;
        while i < four {
            let fresh266 = input;
            input = input.offset(1);
            aptr =
                vq2.as_mut_ptr().offset((*fresh266 as libc::c_int * 2i32) as
                                            isize);
            let fresh267 = input;
            input = input.offset(1);
            bptr =
                vq2.as_mut_ptr().offset((*fresh267 as libc::c_int * 2i32) as
                                            isize);
            j = 0i32 as libc::c_long;
            while j < 2i32 as libc::c_long {
                let fresh268 = cptr;
                cptr = cptr.offset(1);
                *fresh268 = *aptr;
                let fresh269 = dptr;
                dptr = dptr.offset(1);
                *fresh269 = *aptr;
                let fresh270 = dptr;
                dptr = dptr.offset(1);
                *fresh270 = *aptr;
                let fresh271 = cptr;
                cptr = cptr.offset(1);
                *fresh271 = *bptr;
                let fresh272 = dptr;
                dptr = dptr.offset(1);
                *fresh272 = *bptr;
                let fresh273 = dptr;
                dptr = dptr.offset(1);
                *fresh273 = *bptr;
                let fresh274 = dptr;
                dptr = dptr.offset(1);
                *fresh274 = *aptr;
                let fresh275 = dptr;
                dptr = dptr.offset(1);
                *fresh275 = *aptr;
                let fresh276 = dptr;
                dptr = dptr.offset(1);
                *fresh276 = *bptr;
                let fresh277 = dptr;
                dptr = dptr.offset(1);
                *fresh277 = *bptr;
                aptr = aptr.offset(1isize);
                bptr = bptr.offset(1isize);
                j += 1
            }
            i += 1
        }
    } else if cinTable[currentHandle as usize].samplesPerPixel ==
                  1i32 as libc::c_long {
        bbptr = bptr as *mut byte;
        i = 0i32 as libc::c_long;
        while i < two {
            let fresh278 = bbptr;
            bbptr = bbptr.offset(1);
            *fresh278 =
                *cinTable[currentHandle as
                              usize].gray.offset(*input as isize);
            input = input.offset(2isize);
            let fresh279 = bbptr;
            bbptr = bbptr.offset(1);
            *fresh279 =
                *cinTable[currentHandle as
                              usize].gray.offset(*input as isize);
            input = input.offset(4isize);
            i += 1
        }
        bcptr = vq4.as_mut_ptr() as *mut byte;
        bdptr = vq8.as_mut_ptr() as *mut byte;
        i = 0i32 as libc::c_long;
        while i < four {
            let fresh280 = input;
            input = input.offset(1);
            baptr =
                (vq2.as_mut_ptr() as
                     *mut byte).offset((*fresh280 as libc::c_int * 2i32) as
                                           isize);
            let fresh281 = input;
            input = input.offset(1);
            bbptr =
                (vq2.as_mut_ptr() as
                     *mut byte).offset((*fresh281 as libc::c_int * 2i32) as
                                           isize);
            j = 0i32 as libc::c_long;
            while j < 2i32 as libc::c_long {
                let fresh282 = bcptr;
                bcptr = bcptr.offset(1);
                *fresh282 = *baptr;
                let fresh283 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh283 = *baptr;
                let fresh284 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh284 = *baptr;
                let fresh285 = bcptr;
                bcptr = bcptr.offset(1);
                *fresh285 = *bbptr;
                let fresh286 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh286 = *bbptr;
                let fresh287 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh287 = *bbptr;
                let fresh288 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh288 = *baptr;
                let fresh289 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh289 = *baptr;
                let fresh290 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh290 = *bbptr;
                let fresh291 = bdptr;
                bdptr = bdptr.offset(1);
                *fresh291 = *bbptr;
                baptr = baptr.offset(1isize);
                bbptr = bbptr.offset(1isize);
                j += 1
            }
            i += 1
        }
    } else if cinTable[currentHandle as usize].samplesPerPixel ==
                  4i32 as libc::c_long {
        ibptr.s = bptr;
        i = 0i32 as libc::c_long;
        while i < two {
            y0 = *input as libc::c_long;
            input = input.offset(2isize);
            y2 = *input as libc::c_long;
            input = input.offset(2isize);
            let fresh292 = input;
            input = input.offset(1);
            cr = *fresh292 as libc::c_long;
            let fresh293 = input;
            input = input.offset(1);
            cb = *fresh293 as libc::c_long;
            let fresh294 = ibptr.i;
            ibptr.i = ibptr.i.offset(1);
            *fresh294 = yuv_to_rgb24(y0, cr, cb);
            let fresh295 = ibptr.i;
            ibptr.i = ibptr.i.offset(1);
            *fresh295 = yuv_to_rgb24(y2, cr, cb);
            i += 1
        }
        icptr.s = vq4.as_mut_ptr();
        idptr.s = vq8.as_mut_ptr();
        i = 0i32 as libc::c_long;
        while i < four {
            iaptr.s = vq2.as_mut_ptr();
            let fresh296 = input;
            input = input.offset(1);
            iaptr.i =
                iaptr.i.offset((*fresh296 as libc::c_int * 2i32) as isize);
            let fresh297 = input;
            input = input.offset(1);
            ibptr.s =
                vq2.as_mut_ptr().offset((*fresh297 as libc::c_int * 2i32) as
                                            isize);
            let fresh298 = input;
            input = input.offset(1);
            ibptr.i =
                ibptr.i.offset((*fresh298 as libc::c_int * 2i32) as isize);
            j = 0i32 as libc::c_long;
            while j < 2i32 as libc::c_long {
                let fresh299 = icptr.i;
                icptr.i = icptr.i.offset(1);
                *fresh299 = *iaptr.i;
                let fresh300 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh300 = *iaptr.i;
                let fresh301 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh301 = *iaptr.i;
                let fresh302 = icptr.i;
                icptr.i = icptr.i.offset(1);
                *fresh302 = *ibptr.i;
                let fresh303 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh303 = *ibptr.i;
                let fresh304 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh304 = *ibptr.i;
                let fresh305 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh305 = *iaptr.i;
                let fresh306 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh306 = *iaptr.i;
                let fresh307 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh307 = *ibptr.i;
                let fresh308 = idptr.i;
                idptr.i = idptr.i.offset(1);
                *fresh308 = *ibptr.i;
                iaptr.i = iaptr.i.offset(1isize);
                ibptr.i = ibptr.i.offset(1isize);
                j += 1
            }
            i += 1
        }
    };
}
static mut vq2: [libc::c_ushort; 16384] = [0; 16384];
static mut vq8: [libc::c_ushort; 262144] = [0; 262144];
static mut vq4: [libc::c_ushort; 65536] = [0; 65536];
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn yuv_to_rgb24(mut y: libc::c_long, mut u: libc::c_long,
                                  mut v: libc::c_long) -> libc::c_uint {
    let mut r: libc::c_long = 0;
    let mut g: libc::c_long = 0;
    let mut b: libc::c_long = 0;
    let mut YY: libc::c_long = ROQ_YY_tab[y as usize];
    r = YY + ROQ_VR_tab[v as usize] >> 6i32;
    g = YY + ROQ_UG_tab[u as usize] + ROQ_VG_tab[v as usize] >> 6i32;
    b = YY + ROQ_UB_tab[u as usize] >> 6i32;
    if r < 0i32 as libc::c_long { r = 0i32 as libc::c_long }
    if g < 0i32 as libc::c_long { g = 0i32 as libc::c_long }
    if b < 0i32 as libc::c_long { b = 0i32 as libc::c_long }
    if r > 255i32 as libc::c_long { r = 255i32 as libc::c_long }
    if g > 255i32 as libc::c_long { g = 255i32 as libc::c_long }
    if b > 255i32 as libc::c_long { b = 255i32 as libc::c_long }
    return ((r | g << 8i32 | b << 16i32) as libc::c_ulong | 255u64 << 24i32)
               as libc::c_uint;
}
static mut ROQ_UB_tab: [libc::c_long; 256] = [0; 256];
/* *****************************************************************************
*
* Class:		trFMV
*
* Description:	RoQ/RnR manipulation routines
*				not entirely complete for first run
*
******************************************************************************/
static mut ROQ_YY_tab: [libc::c_long; 256] = [0; 256];
static mut ROQ_VG_tab: [libc::c_long; 256] = [0; 256];
static mut ROQ_UG_tab: [libc::c_long; 256] = [0; 256];
static mut ROQ_VR_tab: [libc::c_long; 256] = [0; 256];
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn yuv_to_rgb(mut y: libc::c_long, mut u: libc::c_long,
                                mut v: libc::c_long) -> libc::c_ushort {
    let mut r: libc::c_long = 0;
    let mut g: libc::c_long = 0;
    let mut b: libc::c_long = 0;
    let mut YY: libc::c_long = ROQ_YY_tab[y as usize];
    r = YY + ROQ_VR_tab[v as usize] >> 9i32;
    g = YY + ROQ_UG_tab[u as usize] + ROQ_VG_tab[v as usize] >> 8i32;
    b = YY + ROQ_UB_tab[u as usize] >> 9i32;
    if r < 0i32 as libc::c_long { r = 0i32 as libc::c_long }
    if g < 0i32 as libc::c_long { g = 0i32 as libc::c_long }
    if b < 0i32 as libc::c_long { b = 0i32 as libc::c_long }
    if r > 31i32 as libc::c_long { r = 31i32 as libc::c_long }
    if g > 63i32 as libc::c_long { g = 63i32 as libc::c_long }
    if b > 31i32 as libc::c_long { b = 31i32 as libc::c_long }
    return ((r << 11i32) + (g << 5i32) + b) as libc::c_ushort;
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn RoQPrepMcomp(mut xoff: libc::c_long,
                                  mut yoff: libc::c_long) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut x: libc::c_long = 0;
    let mut y: libc::c_long = 0;
    let mut temp: libc::c_long = 0;
    let mut temp2: libc::c_long = 0;
    i = cinTable[currentHandle as usize].samplesPerLine;
    j = cinTable[currentHandle as usize].samplesPerPixel;
    if cinTable[currentHandle as usize].xsize ==
           cinTable[currentHandle as
                        usize].ysize.wrapping_mul(4i32 as libc::c_uint) &&
           0 == cinTable[currentHandle as usize].half as u64 {
        j = j + j;
        i = i + i
    }
    y = 0i32 as libc::c_long;
    while y < 16i32 as libc::c_long {
        temp2 = (y + yoff - 8i32 as libc::c_long) * i;
        x = 0i32 as libc::c_long;
        while x < 16i32 as libc::c_long {
            temp = (x + xoff - 8i32 as libc::c_long) * j;
            cin.mcomp[(x * 16i32 as libc::c_long + y) as usize] =
                (cinTable[currentHandle as usize].normalBuffer0 -
                     (temp2 + temp)) as libc::c_int;
            x += 1
        }
        y += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CIN_PlayCinematic(mut arg: *const libc::c_char,
                                           mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut w: libc::c_int,
                                           mut h: libc::c_int,
                                           mut systemBits: libc::c_int)
 -> libc::c_int {
    let mut RoQID: libc::c_ushort = 0;
    let mut name: [libc::c_char; 4096] = [0; 4096];
    let mut i: libc::c_int = 0;
    if strstr(arg, b"/\x00" as *const u8 as *const libc::c_char).is_null() &&
           strstr(arg,
                  b"\\\x00" as *const u8 as *const libc::c_char).is_null() {
        Com_sprintf(name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as
                        libc::c_ulong as libc::c_int,
                    b"video/%s\x00" as *const u8 as *const libc::c_char, arg);
    } else {
        Com_sprintf(name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as
                        libc::c_ulong as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char, arg);
    }
    if 0 == systemBits & 1i32 {
        i = 0i32;
        while i < 16i32 {
            if 0 ==
                   strcmp(cinTable[i as usize].fileName.as_mut_ptr(),
                          name.as_mut_ptr()) {
                return i
            }
            i += 1
        }
    }
    Com_DPrintf(b"CIN_PlayCinematic( %s )\n\x00" as *const u8 as
                    *const libc::c_char, arg);
    memset(&mut cin as *mut cinematics_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<cinematics_t>() as libc::c_ulong);
    currentHandle = CIN_HandleForVideo();
    cin.currentHandle = currentHandle;
    strcpy(cinTable[currentHandle as usize].fileName.as_mut_ptr(),
           name.as_mut_ptr());
    cinTable[currentHandle as usize].ROQSize = 0i32 as libc::c_long;
    cinTable[currentHandle as usize].ROQSize =
        FS_FOpenFileRead(cinTable[currentHandle as
                                      usize].fileName.as_mut_ptr(),
                         &mut cinTable[currentHandle as usize].iFile, qtrue);
    if cinTable[currentHandle as usize].ROQSize <= 0i32 as libc::c_long {
        Com_DPrintf(b"play(%s), ROQSize<=0\n\x00" as *const u8 as
                        *const libc::c_char, arg);
        cinTable[currentHandle as usize].fileName[0usize] =
            0i32 as libc::c_char;
        return -1i32
    }
    CIN_SetExtents(currentHandle, x, y, w, h);
    CIN_SetLooping(currentHandle,
                   (systemBits & 2i32 != 0i32) as libc::c_int as qboolean);
    cinTable[currentHandle as usize].CIN_HEIGHT = 512i32;
    cinTable[currentHandle as usize].CIN_WIDTH = 512i32;
    cinTable[currentHandle as usize].holdAtEnd =
        (systemBits & 4i32 != 0i32) as libc::c_int as qboolean;
    cinTable[currentHandle as usize].alterGameState =
        (systemBits & 1i32 != 0i32) as libc::c_int as qboolean;
    cinTable[currentHandle as usize].playonwalls = 1i32;
    cinTable[currentHandle as usize].silent =
        (systemBits & 8i32 != 0i32) as libc::c_int as qboolean;
    cinTable[currentHandle as usize].shader =
        (systemBits & 16i32 != 0i32) as libc::c_int as qboolean;
    if 0 != cinTable[currentHandle as usize].alterGameState as u64 {
        if !uivm.is_null() {
            VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                    UIMENU_NONE as libc::c_int);
        }
    } else {
        cinTable[currentHandle as usize].playonwalls =
            (*cl_inGameVideo).integer
    }
    initRoQ();
    FS_Read(cin.file.as_mut_ptr() as *mut libc::c_void, 16i32,
            cinTable[currentHandle as usize].iFile);
    RoQID =
        (cin.file[0usize] as libc::c_ushort as libc::c_int +
             cin.file[1usize] as libc::c_ushort as libc::c_int * 256i32) as
            libc::c_ushort;
    if RoQID as libc::c_int == 0x1084i32 {
        RoQ_init();
        cinTable[currentHandle as usize].status = FMV_PLAY;
        Com_DPrintf(b"trFMV::play(), playing %s\n\x00" as *const u8 as
                        *const libc::c_char, arg);
        if 0 != cinTable[currentHandle as usize].alterGameState as u64 {
            clc.state = CA_CINEMATIC
        }
        Con_Close();
        if 0 == cinTable[currentHandle as usize].silent as u64 {
            s_rawend[0usize] = s_soundtime
        }
        return currentHandle
    }
    Com_DPrintf(b"trFMV::play(), invalid RoQ ID\n\x00" as *const u8 as
                    *const libc::c_char);
    RoQShutdown();
    return -1i32;
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn initRoQ() {
    if currentHandle < 0i32 { return }
    cinTable[currentHandle as usize].VQNormal =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *mut *mut byte,
                                                            _:
                                                                *mut libc::c_uchar)
                                           -> ()>,
                                Option<unsafe extern "C" fn(_: *mut byte,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>>(Some(blitVQQuad32fs));
    cinTable[currentHandle as usize].VQBuffer =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *mut *mut byte,
                                                            _:
                                                                *mut libc::c_uchar)
                                           -> ()>,
                                Option<unsafe extern "C" fn(_: *mut byte,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>>(Some(blitVQQuad32fs));
    cinTable[currentHandle as usize].samplesPerPixel = 4i32 as libc::c_long;
    ROQ_GenYUVTables();
    RllSetupTable();
}
//-----------------------------------------------------------------------------
// RllSetupTable
//
// Allocates and initializes the square table.
//
// Parameters:	None
//
// Returns:		Nothing
//-----------------------------------------------------------------------------
unsafe extern "C" fn RllSetupTable() {
    let mut z: libc::c_int = 0;
    z = 0i32;
    while z < 128i32 {
        cin.sqrTable[z as usize] = (z * z) as libc::c_short;
        cin.sqrTable[(z + 128i32) as usize] =
            -(cin.sqrTable[z as usize] as libc::c_int) as libc::c_short;
        z += 1
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn ROQ_GenYUVTables() {
    let mut t_ub: libc::c_float = 0.;
    let mut t_vr: libc::c_float = 0.;
    let mut t_ug: libc::c_float = 0.;
    let mut t_vg: libc::c_float = 0.;
    let mut i: libc::c_long = 0;
    t_ub = 1.77200f32 / 2.0f32 * (1i32 << 6i32) as libc::c_float + 0.5f32;
    t_vr = 1.40200f32 / 2.0f32 * (1i32 << 6i32) as libc::c_float + 0.5f32;
    t_ug = 0.34414f32 / 2.0f32 * (1i32 << 6i32) as libc::c_float + 0.5f32;
    t_vg = 0.71414f32 / 2.0f32 * (1i32 << 6i32) as libc::c_float + 0.5f32;
    i = 0i32 as libc::c_long;
    while i < 256i32 as libc::c_long {
        let mut x: libc::c_float =
            (2i32 as libc::c_long * i - 255i32 as libc::c_long) as
                libc::c_float;
        ROQ_UB_tab[i as usize] =
            (t_ub * x + (1i32 << 5i32) as libc::c_float) as libc::c_long;
        ROQ_VR_tab[i as usize] =
            (t_vr * x + (1i32 << 5i32) as libc::c_float) as libc::c_long;
        ROQ_UG_tab[i as usize] = (-t_ug * x) as libc::c_long;
        ROQ_VG_tab[i as usize] =
            (-t_vg * x + (1i32 << 5i32) as libc::c_float) as libc::c_long;
        ROQ_YY_tab[i as usize] = i << 6i32 | i >> 2i32;
        i += 1
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn blitVQQuad32fs(mut status: *mut *mut byte,
                                    mut data: *mut libc::c_uchar) {
    let mut newd: libc::c_ushort = 0;
    let mut celdata: libc::c_ushort = 0;
    let mut code: libc::c_ushort = 0;
    let mut index: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut spl: libc::c_int = 0;
    newd = 0i32 as libc::c_ushort;
    celdata = 0i32 as libc::c_ushort;
    index = 0i32 as libc::c_uint;
    spl = cinTable[currentHandle as usize].samplesPerLine as libc::c_int;
    loop  {
        if 0 == newd {
            newd = 7i32 as libc::c_ushort;
            celdata =
                (*data.offset(0isize) as libc::c_int +
                     *data.offset(1isize) as libc::c_int * 256i32) as
                    libc::c_ushort;
            data = data.offset(2isize)
        } else { newd = newd.wrapping_sub(1) }
        code = (celdata as libc::c_int & 0xc000i32) as libc::c_ushort;
        celdata = ((celdata as libc::c_int) << 2i32) as libc::c_ushort;
        match code as libc::c_int {
            32768 => {
                blit8_32(&mut vq8[(*data as libc::c_int * 128i32) as usize] as
                             *mut libc::c_ushort as *mut byte,
                         *status.offset(index as isize), spl);
                data = data.offset(1isize);
                index = index.wrapping_add(5i32 as libc::c_uint)
            }
            49152 => {
                index = index.wrapping_add(1);
                i = 0i32 as libc::c_uint;
                while i < 4i32 as libc::c_uint {
                    if 0 == newd {
                        newd = 7i32 as libc::c_ushort;
                        celdata =
                            (*data.offset(0isize) as libc::c_int +
                                 *data.offset(1isize) as libc::c_int * 256i32)
                                as libc::c_ushort;
                        data = data.offset(2isize)
                    } else { newd = newd.wrapping_sub(1) }
                    code =
                        (celdata as libc::c_int & 0xc000i32) as
                            libc::c_ushort;
                    celdata =
                        ((celdata as libc::c_int) << 2i32) as libc::c_ushort;
                    match code as libc::c_int {
                        32768 => {
                            blit4_32(&mut vq4[(*data as libc::c_int * 32i32)
                                                  as usize] as
                                         *mut libc::c_ushort as *mut byte,
                                     *status.offset(index as isize), spl);
                            data = data.offset(1isize)
                        }
                        49152 => {
                            blit2_32(&mut vq2[(*data as libc::c_int * 8i32) as
                                                  usize] as
                                         *mut libc::c_ushort as *mut byte,
                                     *status.offset(index as isize), spl);
                            data = data.offset(1isize);
                            blit2_32(&mut vq2[(*data as libc::c_int * 8i32) as
                                                  usize] as
                                         *mut libc::c_ushort as *mut byte,
                                     (*status.offset(index as
                                                         isize)).offset(8isize),
                                     spl);
                            data = data.offset(1isize);
                            blit2_32(&mut vq2[(*data as libc::c_int * 8i32) as
                                                  usize] as
                                         *mut libc::c_ushort as *mut byte,
                                     (*status.offset(index as
                                                         isize)).offset((spl *
                                                                             2i32)
                                                                            as
                                                                            isize),
                                     spl);
                            data = data.offset(1isize);
                            blit2_32(&mut vq2[(*data as libc::c_int * 8i32) as
                                                  usize] as
                                         *mut libc::c_ushort as *mut byte,
                                     (*status.offset(index as
                                                         isize)).offset((spl *
                                                                             2i32)
                                                                            as
                                                                            isize).offset(8isize),
                                     spl);
                            data = data.offset(1isize)
                        }
                        16384 => {
                            move4_32((*status.offset(index as
                                                         isize)).offset(cin.mcomp[*data
                                                                                      as
                                                                                      usize]
                                                                            as
                                                                            isize),
                                     *status.offset(index as isize), spl);
                            data = data.offset(1isize)
                        }
                        _ => { }
                    }
                    index = index.wrapping_add(1);
                    i = i.wrapping_add(1)
                }
            }
            16384 => {
                move8_32((*status.offset(index as
                                             isize)).offset(cin.mcomp[*data as
                                                                          usize]
                                                                as isize),
                         *status.offset(index as isize), spl);
                data = data.offset(1isize);
                index = index.wrapping_add(5i32 as libc::c_uint)
            }
            0 => { index = index.wrapping_add(5i32 as libc::c_uint) }
            _ => { }
        }
        if (*status.offset(index as isize)).is_null() { break ; }
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn move8_32(mut src: *mut byte, mut dst: *mut byte,
                              mut spl: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8i32 {
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
               32i32 as libc::c_ulong);
        src = src.offset(spl as isize);
        dst = dst.offset(spl as isize);
        i += 1
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn move4_32(mut src: *mut byte, mut dst: *mut byte,
                              mut spl: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 4i32 {
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
               16i32 as libc::c_ulong);
        src = src.offset(spl as isize);
        dst = dst.offset(spl as isize);
        i += 1
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn blit2_32(mut src: *mut byte, mut dst: *mut byte,
                              mut spl: libc::c_int) {
    memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
           8i32 as libc::c_ulong);
    memcpy(dst.offset(spl as isize) as *mut libc::c_void,
           src.offset(8isize) as *const libc::c_void, 8i32 as libc::c_ulong);
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn blit4_32(mut src: *mut byte, mut dst: *mut byte,
                              mut spl: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 4i32 {
        memmove(dst as *mut libc::c_void, src as *const libc::c_void,
                16i32 as libc::c_ulong);
        src = src.offset(16isize);
        dst = dst.offset(spl as isize);
        i += 1
    };
}
/* *****************************************************************************
*
* Function:		
*
* Description:	
*
******************************************************************************/
unsafe extern "C" fn blit8_32(mut src: *mut byte, mut dst: *mut byte,
                              mut spl: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8i32 {
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void,
               32i32 as libc::c_ulong);
        src = src.offset(32isize);
        dst = dst.offset(spl as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CIN_SetLooping(mut handle: libc::c_int,
                                        mut loop_0: qboolean) {
    if handle < 0i32 || handle >= 16i32 ||
           cinTable[handle as usize].status as libc::c_uint ==
               FMV_EOF as libc::c_int as libc::c_uint {
        return
    }
    cinTable[handle as usize].looping = loop_0;
}
#[no_mangle]
pub unsafe extern "C" fn CIN_SetExtents(mut handle: libc::c_int,
                                        mut x: libc::c_int,
                                        mut y: libc::c_int,
                                        mut w: libc::c_int,
                                        mut h: libc::c_int) {
    if handle < 0i32 || handle >= 16i32 ||
           cinTable[handle as usize].status as libc::c_uint ==
               FMV_EOF as libc::c_int as libc::c_uint {
        return
    }
    cinTable[handle as usize].xpos = x;
    cinTable[handle as usize].ypos = y;
    cinTable[handle as usize].width = w;
    cinTable[handle as usize].height = h;
    cinTable[handle as usize].dirty = qtrue;
}
unsafe extern "C" fn CIN_HandleForVideo() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        if cinTable[i as usize].fileName[0usize] as libc::c_int == 0i32 {
            return i
        }
        i += 1
    }
    Com_Error(ERR_DROP as libc::c_int,
              b"CIN_HandleForVideo: none free\x00" as *const u8 as
                  *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SCR_StopCinematic() {
    if CL_handle >= 0i32 && CL_handle < 16i32 {
        CIN_StopCinematic(CL_handle);
        S_StopAllSounds();
        CL_handle = -1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn CIN_StopCinematic(mut handle: libc::c_int)
 -> e_status {
    if handle < 0i32 || handle >= 16i32 ||
           cinTable[handle as usize].status as libc::c_uint ==
               FMV_EOF as libc::c_int as libc::c_uint {
        return FMV_EOF
    }
    currentHandle = handle;
    Com_DPrintf(b"trFMV::stop(), closing %s\n\x00" as *const u8 as
                    *const libc::c_char,
                cinTable[currentHandle as usize].fileName.as_mut_ptr());
    if cinTable[currentHandle as usize].buf.is_null() { return FMV_EOF }
    if 0 != cinTable[currentHandle as usize].alterGameState as u64 {
        if clc.state as libc::c_uint !=
               CA_CINEMATIC as libc::c_int as libc::c_uint {
            return cinTable[currentHandle as usize].status
        }
    }
    cinTable[currentHandle as usize].status = FMV_EOF;
    RoQShutdown();
    return FMV_EOF;
}
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawCinematic() {
    if CL_handle >= 0i32 && CL_handle < 16i32 {
        CIN_DrawCinematic(CL_handle);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CIN_DrawCinematic(mut handle: libc::c_int) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut buf: *mut byte = 0 as *mut byte;
    if handle < 0i32 || handle >= 16i32 ||
           cinTable[handle as usize].status as libc::c_uint ==
               FMV_EOF as libc::c_int as libc::c_uint {
        return
    }
    if cinTable[handle as usize].buf.is_null() { return }
    x = cinTable[handle as usize].xpos as libc::c_float;
    y = cinTable[handle as usize].ypos as libc::c_float;
    w = cinTable[handle as usize].width as libc::c_float;
    h = cinTable[handle as usize].height as libc::c_float;
    buf = cinTable[handle as usize].buf;
    SCR_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    if 0 != cinTable[handle as usize].dirty as libc::c_uint &&
           (cinTable[handle as usize].CIN_WIDTH as libc::c_long !=
                cinTable[handle as usize].drawX ||
                cinTable[handle as usize].CIN_HEIGHT as libc::c_long !=
                    cinTable[handle as usize].drawY) {
        let mut buf2: *mut libc::c_int = 0 as *mut libc::c_int;
        buf2 =
            Hunk_AllocateTempMemory(256i32 * 256i32 * 4i32) as
                *mut libc::c_int;
        CIN_ResampleCinematic(handle, buf2);
        re.DrawStretchRaw.expect("non-null function pointer")(x as
                                                                  libc::c_int,
                                                              y as
                                                                  libc::c_int,
                                                              w as
                                                                  libc::c_int,
                                                              h as
                                                                  libc::c_int,
                                                              256i32, 256i32,
                                                              buf2 as
                                                                  *mut byte,
                                                              handle, qtrue);
        cinTable[handle as usize].dirty = qfalse;
        Hunk_FreeTempMemory(buf2 as *mut libc::c_void);
        return
    }
    re.DrawStretchRaw.expect("non-null function pointer")(x as libc::c_int,
                                                          y as libc::c_int,
                                                          w as libc::c_int,
                                                          h as libc::c_int,
                                                          cinTable[handle as
                                                                       usize].drawX
                                                              as libc::c_int,
                                                          cinTable[handle as
                                                                       usize].drawY
                                                              as libc::c_int,
                                                          buf, handle,
                                                          cinTable[handle as
                                                                       usize].dirty);
    cinTable[handle as usize].dirty = qfalse;
}
/*
==================
CIN_ResampleCinematic

Resample cinematic to 256x256 and store in buf2
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CIN_ResampleCinematic(mut handle: libc::c_int,
                                               mut buf2: *mut libc::c_int) {
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut buf3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xm: libc::c_int = 0;
    let mut ym: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut buf: *mut byte = 0 as *mut byte;
    buf = cinTable[handle as usize].buf;
    xm = cinTable[handle as usize].CIN_WIDTH / 256i32;
    ym = cinTable[handle as usize].CIN_HEIGHT / 256i32;
    ll = 8i32;
    if cinTable[handle as usize].CIN_WIDTH == 512i32 { ll = 9i32 }
    buf3 = buf as *mut libc::c_int;
    if xm == 2i32 && ym == 2i32 {
        let mut bc2: *mut byte = 0 as *mut byte;
        let mut bc3: *mut byte = 0 as *mut byte;
        let mut ic: libc::c_int = 0;
        let mut iiy: libc::c_int = 0;
        bc2 = buf2 as *mut byte;
        bc3 = buf3 as *mut byte;
        iy = 0i32;
        while iy < 256i32 {
            iiy = iy << 12i32;
            ix = 0i32;
            while ix < 2048i32 {
                ic = ix;
                while ic < ix + 4i32 {
                    *bc2 =
                        (*bc3.offset((iiy + ic) as isize) as libc::c_int +
                             *bc3.offset((iiy + 4i32 + ic) as isize) as
                                 libc::c_int +
                             *bc3.offset((iiy + 2048i32 + ic) as isize) as
                                 libc::c_int +
                             *bc3.offset((iiy + 2048i32 + 4i32 + ic) as isize)
                                 as libc::c_int >> 2i32) as byte;
                    bc2 = bc2.offset(1isize);
                    ic += 1
                }
                ix += 8i32
            }
            iy += 1
        }
    } else if xm == 2i32 && ym == 1i32 {
        let mut bc2_0: *mut byte = 0 as *mut byte;
        let mut bc3_0: *mut byte = 0 as *mut byte;
        let mut ic_0: libc::c_int = 0;
        let mut iiy_0: libc::c_int = 0;
        bc2_0 = buf2 as *mut byte;
        bc3_0 = buf3 as *mut byte;
        iy = 0i32;
        while iy < 256i32 {
            iiy_0 = iy << 11i32;
            ix = 0i32;
            while ix < 2048i32 {
                ic_0 = ix;
                while ic_0 < ix + 4i32 {
                    *bc2_0 =
                        (*bc3_0.offset((iiy_0 + ic_0) as isize) as libc::c_int
                             +
                             *bc3_0.offset((iiy_0 + 4i32 + ic_0) as isize) as
                                 libc::c_int >> 1i32) as byte;
                    bc2_0 = bc2_0.offset(1isize);
                    ic_0 += 1
                }
                ix += 8i32
            }
            iy += 1
        }
    } else {
        iy = 0i32;
        while iy < 256i32 {
            ix = 0i32;
            while ix < 256i32 {
                *buf2.offset(((iy << 8i32) + ix) as isize) =
                    *buf3.offset(((iy * ym << ll) + ix * xm) as isize);
                ix += 1
            }
            iy += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CIN_UploadCinematic(mut handle: libc::c_int) {
    if handle >= 0i32 && handle < 16i32 {
        if cinTable[handle as usize].buf.is_null() { return }
        if cinTable[handle as usize].playonwalls <= 0i32 &&
               0 != cinTable[handle as usize].dirty as libc::c_uint {
            if cinTable[handle as usize].playonwalls == 0i32 {
                cinTable[handle as usize].playonwalls = -1i32
            } else if cinTable[handle as usize].playonwalls == -1i32 {
                cinTable[handle as usize].playonwalls = -2i32
            } else { cinTable[handle as usize].dirty = qfalse }
        }
        if 0 != cinTable[handle as usize].dirty as libc::c_uint &&
               (cinTable[handle as usize].CIN_WIDTH as libc::c_long !=
                    cinTable[handle as usize].drawX ||
                    cinTable[handle as usize].CIN_HEIGHT as libc::c_long !=
                        cinTable[handle as usize].drawY) {
            let mut buf2: *mut libc::c_int = 0 as *mut libc::c_int;
            buf2 =
                Hunk_AllocateTempMemory(256i32 * 256i32 * 4i32) as
                    *mut libc::c_int;
            CIN_ResampleCinematic(handle, buf2);
            re.UploadCinematic.expect("non-null function pointer")(cinTable[handle
                                                                                as
                                                                                usize].CIN_WIDTH,
                                                                   cinTable[handle
                                                                                as
                                                                                usize].CIN_HEIGHT,
                                                                   256i32,
                                                                   256i32,
                                                                   buf2 as
                                                                       *mut byte,
                                                                   handle,
                                                                   qtrue);
            cinTable[handle as usize].dirty = qfalse;
            Hunk_FreeTempMemory(buf2 as *mut libc::c_void);
        } else {
            re.UploadCinematic.expect("non-null function pointer")(cinTable[handle
                                                                                as
                                                                                usize].CIN_WIDTH,
                                                                   cinTable[handle
                                                                                as
                                                                                usize].CIN_HEIGHT,
                                                                   cinTable[handle
                                                                                as
                                                                                usize].drawX
                                                                       as
                                                                       libc::c_int,
                                                                   cinTable[handle
                                                                                as
                                                                                usize].drawY
                                                                       as
                                                                       libc::c_int,
                                                                   cinTable[handle
                                                                                as
                                                                                usize].buf,
                                                                   handle,
                                                                   cinTable[handle
                                                                                as
                                                                                usize].dirty);
            cinTable[handle as usize].dirty = qfalse
        }
        if (*cl_inGameVideo).integer == 0i32 &&
               cinTable[handle as usize].playonwalls == 1i32 {
            cinTable[handle as usize].playonwalls -= 1
        } else if (*cl_inGameVideo).integer != 0i32 &&
                      cinTable[handle as usize].playonwalls != 1i32 {
            cinTable[handle as usize].playonwalls = 1i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CIN_CloseAllVideos() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        if cinTable[i as usize].fileName[0usize] as libc::c_int != 0i32 {
            CIN_StopCinematic(i);
        }
        i += 1
    };
}
//-----------------------------------------------------------------------------
// RllDecodeMonoToMono
//
// Decode mono source data into a mono buffer.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= # of shorts of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn RllDecodeMonoToMono(mut from: *mut libc::c_uchar,
                                             mut to: *mut libc::c_short,
                                             mut size: libc::c_uint,
                                             mut signedOutput: libc::c_char,
                                             mut flag: libc::c_ushort)
 -> libc::c_long {
    let mut z: libc::c_uint = 0;
    let mut prev: libc::c_int = 0;
    if 0 != signedOutput {
        prev = flag as libc::c_int - 0x8000i32
    } else { prev = flag as libc::c_int }
    z = 0i32 as libc::c_uint;
    while z < size {
        let ref mut fresh309 = *to.offset(z as isize);
        *fresh309 =
            (prev +
                 cin.sqrTable[*from.offset(z as isize) as usize] as
                     libc::c_int) as libc::c_short;
        prev = *fresh309 as libc::c_int;
        z = z.wrapping_add(1)
    }
    return size as libc::c_long;
}
//-----------------------------------------------------------------------------
// RllDecodeStereoToMono
//
// Decode stereo source data into a mono buffer.
//
// Parameters:	from -> buffer holding encoded data
//				to ->	buffer to hold decoded data
//				size =	number of bytes of input (= # of bytes of output)
//				signedOutput = 0 for unsigned output, non-zero for signed output
//				flag = flags from asset header
//
// Returns:		Number of samples placed in output buffer
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn RllDecodeStereoToMono(mut from: *mut libc::c_uchar,
                                               mut to: *mut libc::c_short,
                                               mut size: libc::c_uint,
                                               mut signedOutput: libc::c_char,
                                               mut flag: libc::c_ushort)
 -> libc::c_long {
    let mut z: libc::c_uint = 0;
    let mut prevL: libc::c_int = 0;
    let mut prevR: libc::c_int = 0;
    if 0 != signedOutput {
        prevL = (flag as libc::c_int & 0xff00i32) - 0x8000i32;
        prevR = ((flag as libc::c_int & 0xffi32) << 8i32) - 0x8000i32
    } else {
        prevL = flag as libc::c_int & 0xff00i32;
        prevR = (flag as libc::c_int & 0xffi32) << 8i32
    }
    z = 0i32 as libc::c_uint;
    while z < size {
        prevL =
            prevL +
                cin.sqrTable[*from.offset(z.wrapping_mul(2i32 as libc::c_uint)
                                              as isize) as usize] as
                    libc::c_int;
        prevR =
            prevR +
                cin.sqrTable[*from.offset(z.wrapping_mul(2i32 as
                                                             libc::c_uint).wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_uint)
                                              as isize) as usize] as
                    libc::c_int;
        *to.offset(z as isize) = ((prevL + prevR) / 2i32) as libc::c_short;
        z = z.wrapping_add(1i32 as libc::c_uint)
    }
    return size as libc::c_long;
}