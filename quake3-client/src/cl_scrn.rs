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
    // parameters to the main Error routine
    pub type unnamed = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed = 0;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    pub type vec4_t = [vec_t; 4];
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub static mut g_color_table: [vec4_t; 8];
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
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
    use super::q_shared_h::{byte, qboolean, cvar_t, fileHandle_t};
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
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
        #[no_mangle]
        pub fn Cvar_VariableIntegerValue(var_name: *const libc::c_char)
         -> libc::c_int;
        // doesn't work for files that are opened from a pack file
        #[no_mangle]
        pub fn FS_FTell(f: fileHandle_t) -> libc::c_int;
        #[no_mangle]
        pub static mut com_dedicated: *mut cvar_t;
        #[no_mangle]
        pub static mut com_speeds: *mut cvar_t;
        #[no_mangle]
        pub static mut time_frontend: libc::c_int;
        // renderer backend time
        #[no_mangle]
        pub static mut time_backend: libc::c_int;
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
    pub type unnamed_0 = libc::c_uint;
    pub const UIMENU_POSTGAME: unnamed_0 = 6;
    pub const UIMENU_TEAM: unnamed_0 = 5;
    pub const UIMENU_BAD_CD_KEY: unnamed_0 = 4;
    pub const UIMENU_NEED_CD: unnamed_0 = 3;
    pub const UIMENU_INGAME: unnamed_0 = 2;
    pub const UIMENU_MAIN: unnamed_0 = 1;
    pub const UIMENU_NONE: unnamed_0 = 0;
    pub type unnamed_1 = libc::c_uint;
    //	void	UI_DrawConnectScreen( qboolean overlay );
    pub const UI_HASUNIQUECDKEY: unnamed_1 = 10;
    //	qboolean UI_ConsoleCommand( int realTime );
    pub const UI_DRAW_CONNECT_SCREEN: unnamed_1 = 9;
    //	void	UI_SetActiveMenu( uiMenuCommand_t menu );
    pub const UI_CONSOLE_COMMAND: unnamed_1 = 8;
    //	qboolean UI_IsFullscreen( void );
    pub const UI_SET_ACTIVE_MENU: unnamed_1 = 7;
    //	void	UI_Refresh( int time );
    pub const UI_IS_FULLSCREEN: unnamed_1 = 6;
    //	void	UI_MouseEvent( int dx, int dy );
    pub const UI_REFRESH: unnamed_1 = 5;
    //	void	UI_KeyEvent( int key );
    pub const UI_MOUSE_EVENT: unnamed_1 = 4;
    //	void	UI_Shutdown( void );
    pub const UI_KEY_EVENT: unnamed_1 = 3;
    //	void	UI_Init( void );
    pub const UI_SHUTDOWN: unnamed_1 = 2;
    pub const UI_INIT: unnamed_1 = 1;
    // system reserved
    pub const UI_GETAPIVERSION: unnamed_1 = 0;
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
                            qhandle_t, cvar_t, vec_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t, stereoFrame_t};
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
        pub static mut cl_debugMove: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_voipSend: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_voipShowMeter: *mut cvar_t;
        #[no_mangle]
        pub static mut cl_voip: *mut cvar_t;
        #[no_mangle]
        pub fn Con_DrawConsole();
        #[no_mangle]
        pub fn Key_GetCatcher() -> libc::c_int;
        #[no_mangle]
        pub fn CL_CGameRendering(stereo: stereoFrame_t);
        #[no_mangle]
        pub fn SCR_DrawCinematic();
    }
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...)
         -> libc::c_int;
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
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/cl_scrn.c"]
pub mod cl_scrn_c {
    use super::q_shared_h::{qboolean, cvar_t};
    use super::tr_types_h::{stereoFrame_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    extern "C" {
        // stop all sounds and the background track
        #[no_mangle]
        pub fn S_StopAllSounds();
    }
}
use self::types_h::{__uint8_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, vec_t,
                       vec3_t, vec4_t, cvar_s, cvar_t, markFragment_t,
                       orientation_t, connstate_t, CA_CINEMATIC, CA_ACTIVE,
                       CA_PRIMED, CA_LOADING, CA_CONNECTED, CA_CHALLENGING,
                       CA_CONNECTING, CA_AUTHORIZING, CA_DISCONNECTED,
                       CA_UNINITIALIZED, glyphInfo_t, fontInfo_t,
                       Q_IsColorString, g_color_table, Com_Error};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t, vm_t, vm_s,
                      VM_Call, Cvar_Get, Cvar_VariableIntegerValue, FS_FTell,
                      com_dedicated, com_speeds, time_frontend, time_backend};
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
use self::ui_public_h::{unnamed_0, UIMENU_POSTGAME, UIMENU_TEAM,
                        UIMENU_BAD_CD_KEY, UIMENU_NEED_CD, UIMENU_INGAME,
                        UIMENU_MAIN, UIMENU_NONE, unnamed_1,
                        UI_HASUNIQUECDKEY, UI_DRAW_CONNECT_SCREEN,
                        UI_CONSOLE_COMMAND, UI_SET_ACTIVE_MENU,
                        UI_IS_FULLSCREEN, UI_REFRESH, UI_MOUSE_EVENT,
                        UI_KEY_EVENT, UI_SHUTDOWN, UI_INIT, UI_GETAPIVERSION};
use self::curl_h::{CURL};
use self::multi_h::{CURLM};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::client_h::{clientConnection_t, serverInfo_t, clientStatic_t, clc,
                     cls, uivm, re, cl_debugMove, cl_voipSend,
                     cl_voipShowMeter, cl_voip, Con_DrawConsole,
                     Key_GetCatcher, CL_CGameRendering, SCR_DrawCinematic};
use self::assert_h::{__assert_fail};
use self::stdio_h::{sprintf};
use self::string_h::{memcpy, strlen};
use self::snd_public_h::{S_StopAllSounds};
// call before filesystem access
// FIXME: move logging to common?
#[no_mangle]
pub unsafe extern "C" fn SCR_DebugGraph(mut value: libc::c_float) {
    values[current as usize] = value;
    current =
        ((current + 1i32) as
             libc::c_ulong).wrapping_rem((::std::mem::size_of::<[libc::c_float; 1024]>()
                                              as
                                              libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_float>()
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int;
}
static mut values: [libc::c_float; 1024] = [0.; 1024];
/*
===============================================================================

DEBUG GRAPH

===============================================================================
*/
static mut current: libc::c_int = 0;
#[no_mangle]
pub static mut cl_timegraph: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
//
// cl_scrn.c
//
#[no_mangle]
pub unsafe extern "C" fn SCR_Init() {
    cl_timegraph =
        Cvar_Get(b"timegraph\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cl_debuggraph =
        Cvar_Get(b"debuggraph\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cl_graphheight =
        Cvar_Get(b"graphheight\x00" as *const u8 as *const libc::c_char,
                 b"32\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cl_graphscale =
        Cvar_Get(b"graphscale\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cl_graphshift =
        Cvar_Get(b"graphshift\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    scr_initialized = qtrue;
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
// cl_scrn.c -- master for refresh, status bar, console, chat, notify, etc
// ready to draw
#[no_mangle]
pub static mut scr_initialized: qboolean = qfalse;
#[no_mangle]
pub static mut cl_graphshift: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_graphscale: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_graphheight: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_debuggraph: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn SCR_UpdateScreen() {
    static mut recursive: libc::c_int = 0;
    if 0 == scr_initialized as u64 { return }
    recursive += 1;
    if recursive > 2i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"SCR_UpdateScreen: recursively called\x00" as *const u8 as
                      *const libc::c_char);
    }
    recursive = 1i32;
    if !uivm.is_null() || 0 != (*com_dedicated).integer {
        let mut in_anaglyphMode: libc::c_int =
            Cvar_VariableIntegerValue(b"r_anaglyphMode\x00" as *const u8 as
                                          *const libc::c_char);
        if 0 != cls.glconfig.stereoEnabled as libc::c_uint ||
               0 != in_anaglyphMode {
            SCR_DrawScreenField(STEREO_LEFT);
            SCR_DrawScreenField(STEREO_RIGHT);
        } else { SCR_DrawScreenField(STEREO_CENTER); }
        if 0 != (*com_speeds).integer {
            re.EndFrame.expect("non-null function pointer")(&mut time_frontend,
                                                            &mut time_backend);
        } else {
            re.EndFrame.expect("non-null function pointer")(0 as
                                                                *mut libc::c_int,
                                                            0 as
                                                                *mut libc::c_int);
        }
    }
    recursive = 0i32;
}
//=======================================================
/*
==================
SCR_DrawScreenField

This will be called twice if rendering in stereo mode
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawScreenField(mut stereoFrame: stereoFrame_t) {
    let mut uiFullscreen: qboolean = qfalse;
    re.BeginFrame.expect("non-null function pointer")(stereoFrame);
    uiFullscreen =
        (!uivm.is_null() &&
             0 != VM_Call(uivm, UI_IS_FULLSCREEN as libc::c_int)) as
            libc::c_int as qboolean;
    if 0 != uiFullscreen as libc::c_uint ||
           (clc.state as libc::c_uint) <
               CA_LOADING as libc::c_int as libc::c_uint {
        if cls.glconfig.vidWidth * 480i32 > cls.glconfig.vidHeight * 640i32 {
            re.SetColor.expect("non-null function pointer")(g_color_table[0usize].as_mut_ptr());
            re.DrawStretchPic.expect("non-null function pointer")(0i32 as
                                                                      libc::c_float,
                                                                  0i32 as
                                                                      libc::c_float,
                                                                  cls.glconfig.vidWidth
                                                                      as
                                                                      libc::c_float,
                                                                  cls.glconfig.vidHeight
                                                                      as
                                                                      libc::c_float,
                                                                  0i32 as
                                                                      libc::c_float,
                                                                  0i32 as
                                                                      libc::c_float,
                                                                  0i32 as
                                                                      libc::c_float,
                                                                  0i32 as
                                                                      libc::c_float,
                                                                  cls.whiteShader);
            re.SetColor.expect("non-null function pointer")(0 as
                                                                *const libc::c_float);
        }
    }
    if !uivm.is_null() && 0 == uiFullscreen as u64 {
        match clc.state as libc::c_uint {
            9 => { SCR_DrawCinematic(); }
            1 => {
                S_StopAllSounds();
                VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                        UIMENU_MAIN as libc::c_int);
            }
            3 | 4 | 5 => {
                VM_Call(uivm, UI_REFRESH as libc::c_int, cls.realtime);
                VM_Call(uivm, UI_DRAW_CONNECT_SCREEN as libc::c_int,
                        qfalse as libc::c_int);
            }
            6 | 7 => {
                CL_CGameRendering(stereoFrame);
                VM_Call(uivm, UI_REFRESH as libc::c_int, cls.realtime);
                VM_Call(uivm, UI_DRAW_CONNECT_SCREEN as libc::c_int,
                        qtrue as libc::c_int);
            }
            8 => {
                CL_CGameRendering(stereoFrame);
                SCR_DrawDemoRecording();
                SCR_DrawVoipMeter();
            }
            _ => {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"SCR_DrawScreenField: bad clc.state\x00" as
                              *const u8 as *const libc::c_char);
            }
        }
    }
    if 0 != Key_GetCatcher() & 0x2i32 && !uivm.is_null() {
        VM_Call(uivm, UI_REFRESH as libc::c_int, cls.realtime);
    }
    Con_DrawConsole();
    if 0 != (*cl_debuggraph).integer || 0 != (*cl_timegraph).integer ||
           0 != (*cl_debugMove).integer {
        SCR_DrawDebugGraph();
    };
}
/*
==============
SCR_DrawDebugGraph
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawDebugGraph() {
    let mut a: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut v: libc::c_float = 0.;
    w = cls.glconfig.vidWidth;
    x = 0i32;
    y = cls.glconfig.vidHeight;
    re.SetColor.expect("non-null function pointer")(g_color_table[0usize].as_mut_ptr());
    re.DrawStretchPic.expect("non-null function pointer")(x as libc::c_float,
                                                          (y -
                                                               (*cl_graphheight).integer)
                                                              as
                                                              libc::c_float,
                                                          w as libc::c_float,
                                                          (*cl_graphheight).integer
                                                              as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          cls.whiteShader);
    re.SetColor.expect("non-null function pointer")(0 as
                                                        *const libc::c_float);
    a = 0i32;
    while a < w {
        i =
            (::std::mem::size_of::<[libc::c_float; 1024]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_float>()
                                                 as
                                                 libc::c_ulong).wrapping_add(current
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_sub((a
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong).wrapping_rem((::std::mem::size_of::<[libc::c_float; 1024]>()
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_float>()
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   libc::c_ulong))).wrapping_rem((::std::mem::size_of::<[libc::c_float; 1024]>()
                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_float>()
                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                      libc::c_ulong))
                as libc::c_int;
        v = values[i as usize];
        v =
            v * (*cl_graphscale).integer as libc::c_float +
                (*cl_graphshift).integer as libc::c_float;
        if v < 0i32 as libc::c_float {
            v +=
                ((*cl_graphheight).integer *
                     (1i32 +
                          (-v / (*cl_graphheight).integer as libc::c_float) as
                              libc::c_int)) as libc::c_float
        }
        h = v as libc::c_int % (*cl_graphheight).integer;
        re.DrawStretchPic.expect("non-null function pointer")((x + w - 1i32 -
                                                                   a) as
                                                                  libc::c_float,
                                                              (y - h) as
                                                                  libc::c_float,
                                                              1i32 as
                                                                  libc::c_float,
                                                              h as
                                                                  libc::c_float,
                                                              0i32 as
                                                                  libc::c_float,
                                                              0i32 as
                                                                  libc::c_float,
                                                              0i32 as
                                                                  libc::c_float,
                                                              0i32 as
                                                                  libc::c_float,
                                                              cls.whiteShader);
        a += 1
    };
}
/*
=================
SCR_DrawVoipMeter
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawVoipMeter() {
    let mut buffer: [libc::c_char; 16] = [0; 16];
    let mut string: [libc::c_char; 256] = [0; 256];
    let mut limit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if 0 == (*cl_voipShowMeter).integer {
        return
    } else {
        if 0 == (*cl_voipSend).integer {
            return
        } else {
            if clc.state as libc::c_uint !=
                   CA_ACTIVE as libc::c_int as libc::c_uint {
                return
            } else {
                if 0 == clc.voipEnabled as u64 {
                    return
                } else {
                    if 0 != clc.demoplaying as u64 {
                        return
                    } else { if 0 == (*cl_voip).integer { return } }
                }
            }
        }
    }
    limit = (clc.voipPower * 10.0f32) as libc::c_int;
    if limit > 10i32 { limit = 10i32 }
    i = 0i32;
    while i < limit {
        buffer[i as usize] = '*' as i32 as libc::c_char;
        i += 1
    }
    while i < 10i32 {
        let fresh0 = i;
        i = i + 1;
        buffer[fresh0 as usize] = ' ' as i32 as libc::c_char
    }
    buffer[i as usize] = '\u{0}' as i32 as libc::c_char;
    sprintf(string.as_mut_ptr(),
            b"VoIP: [%s]\x00" as *const u8 as *const libc::c_char,
            buffer.as_mut_ptr());
    SCR_DrawStringExt((320i32 as
                           libc::c_ulong).wrapping_sub(strlen(string.as_mut_ptr()).wrapping_mul(4i32
                                                                                                    as
                                                                                                    libc::c_ulong))
                          as libc::c_int, 10i32, 8i32 as libc::c_float,
                      string.as_mut_ptr(), g_color_table[7usize].as_mut_ptr(),
                      qtrue, qfalse);
}
/*
==================
SCR_DrawBigString[Color]

Draws a multi-colored string with a drop shadow, optionally forcing
to a fixed color.

Coordinates are at 640 by 480 virtual resolution
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawStringExt(mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut size: libc::c_float,
                                           mut string: *const libc::c_char,
                                           mut setColor: *mut libc::c_float,
                                           mut forceColor: qboolean,
                                           mut noColorEscape: qboolean) {
    let mut color: vec4_t = [0.; 4];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: libc::c_int = 0;
    color[2usize] = 0i32 as vec_t;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = *setColor.offset(3isize);
    re.SetColor.expect("non-null function pointer")(color.as_mut_ptr());
    s = string;
    xx = x;
    while 0 != *s {
        if 0 == noColorEscape as u64 &&
               0 != Q_IsColorString(s) as libc::c_uint {
            s = s.offset(2isize)
        } else {
            SCR_DrawChar(xx + 2i32, y + 2i32, size, *s as libc::c_int);
            xx = (xx as libc::c_float + size) as libc::c_int;
            s = s.offset(1isize)
        }
    }
    s = string;
    xx = x;
    re.SetColor.expect("non-null function pointer")(setColor);
    while 0 != *s {
        if 0 != Q_IsColorString(s) as u64 {
            if 0 == forceColor as u64 {
                memcpy(color.as_mut_ptr() as *mut libc::c_void,
                       g_color_table[(*s.offset(1isize) as libc::c_int -
                                          '0' as i32 & 0x7i32) as
                                         usize].as_mut_ptr() as
                           *const libc::c_void,
                       ::std::mem::size_of::<vec4_t>() as libc::c_ulong);
                color[3usize] = *setColor.offset(3isize);
                re.SetColor.expect("non-null function pointer")(color.as_mut_ptr());
            }
            if 0 == noColorEscape as u64 { s = s.offset(2isize); continue ; }
        }
        SCR_DrawChar(xx, y, size, *s as libc::c_int);
        xx = (xx as libc::c_float + size) as libc::c_int;
        s = s.offset(1isize)
    }
    re.SetColor.expect("non-null function pointer")(0 as
                                                        *const libc::c_float);
}
/*
** SCR_DrawChar
** chars are drawn at 640*480 virtual screen size
*/
unsafe extern "C" fn SCR_DrawChar(mut x: libc::c_int, mut y: libc::c_int,
                                  mut size: libc::c_float,
                                  mut ch: libc::c_int) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    ch &= 255i32;
    if ch == ' ' as i32 { return }
    if (y as libc::c_float) < -size { return }
    ax = x as libc::c_float;
    ay = y as libc::c_float;
    aw = size;
    ah = size;
    SCR_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    row = ch >> 4i32;
    col = ch & 15i32;
    frow = (row as libc::c_double * 0.0625f64) as libc::c_float;
    fcol = (col as libc::c_double * 0.0625f64) as libc::c_float;
    size = 0.0625f64 as libc::c_float;
    re.DrawStretchPic.expect("non-null function pointer")(ax, ay, aw, ah,
                                                          fcol, frow,
                                                          fcol + size,
                                                          frow + size,
                                                          cls.charSetShader);
}
#[no_mangle]
pub unsafe extern "C" fn SCR_AdjustFrom640(mut x: *mut libc::c_float,
                                           mut y: *mut libc::c_float,
                                           mut w: *mut libc::c_float,
                                           mut h: *mut libc::c_float) {
    let mut xscale: libc::c_float = 0.;
    let mut yscale: libc::c_float = 0.;
    xscale =
        (cls.glconfig.vidWidth as libc::c_double / 640.0f64) as libc::c_float;
    yscale =
        (cls.glconfig.vidHeight as libc::c_double / 480.0f64) as
            libc::c_float;
    if !x.is_null() { *x *= xscale }
    if !y.is_null() { *y *= yscale }
    if !w.is_null() { *w *= xscale }
    if !h.is_null() { *h *= yscale };
}
//===============================================================================
/*
=================
SCR_DrawDemoRecording
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawDemoRecording() {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut pos: libc::c_int = 0;
    if 0 == clc.demorecording as u64 { return }
    if 0 != clc.spDemoRecording as u64 { return }
    pos = FS_FTell(clc.demofile);
    sprintf(string.as_mut_ptr(),
            b"RECORDING %s: %ik\x00" as *const u8 as *const libc::c_char,
            clc.demoName.as_mut_ptr(), pos / 1024i32);
    SCR_DrawStringExt((320i32 as
                           libc::c_ulong).wrapping_sub(strlen(string.as_mut_ptr()).wrapping_mul(4i32
                                                                                                    as
                                                                                                    libc::c_ulong))
                          as libc::c_int, 20i32, 8i32 as libc::c_float,
                      string.as_mut_ptr(), g_color_table[7usize].as_mut_ptr(),
                      qtrue, qfalse);
}
// returns in virtual 640x480 coordinates
#[no_mangle]
pub unsafe extern "C" fn SCR_GetBigStringWidth(mut str: *const libc::c_char)
 -> libc::c_int {
    return SCR_Strlen(str) * 16i32;
}
/*
** SCR_Strlen -- skips color escape codes
*/
unsafe extern "C" fn SCR_Strlen(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = str;
    let mut count: libc::c_int = 0i32;
    while 0 != *s {
        if 0 != Q_IsColorString(s) as u64 {
            s = s.offset(2isize)
        } else { count += 1; s = s.offset(1isize) }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn SCR_FillRect(mut x: libc::c_float,
                                      mut y: libc::c_float,
                                      mut width: libc::c_float,
                                      mut height: libc::c_float,
                                      mut color: *const libc::c_float) {
    re.SetColor.expect("non-null function pointer")(color);
    SCR_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    re.DrawStretchPic.expect("non-null function pointer")(x, y, width, height,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          cls.whiteShader);
    re.SetColor.expect("non-null function pointer")(0 as
                                                        *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawPic(mut x: libc::c_float,
                                     mut y: libc::c_float,
                                     mut width: libc::c_float,
                                     mut height: libc::c_float,
                                     mut hShader: qhandle_t) {
    SCR_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    re.DrawStretchPic.expect("non-null function pointer")(x, y, width, height,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          1i32 as
                                                              libc::c_float,
                                                          1i32 as
                                                              libc::c_float,
                                                          hShader);
}
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawNamedPic(mut x: libc::c_float,
                                          mut y: libc::c_float,
                                          mut width: libc::c_float,
                                          mut height: libc::c_float,
                                          mut picname: *const libc::c_char) {
    let mut hShader: qhandle_t = 0;
    if width != 0i32 as libc::c_float {
    } else {
        __assert_fail(b"width != 0\x00" as *const u8 as *const libc::c_char,
                      b"code/client/cl_scrn.c\x00" as *const u8 as
                          *const libc::c_char, 44i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"void SCR_DrawNamedPic(float, float, float, float, const char *)\x00")).as_ptr());
    }
    hShader = re.RegisterShader.expect("non-null function pointer")(picname);
    SCR_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    re.DrawStretchPic.expect("non-null function pointer")(x, y, width, height,
                                                          0i32 as
                                                              libc::c_float,
                                                          0i32 as
                                                              libc::c_float,
                                                          1i32 as
                                                              libc::c_float,
                                                          1i32 as
                                                              libc::c_float,
                                                          hShader);
}
// draws a string with embedded color control characters with fade
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawBigString(mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut s: *const libc::c_char,
                                           mut alpha: libc::c_float,
                                           mut noColorEscape: qboolean) {
    let mut color: [libc::c_float; 4] = [0.; 4];
    color[2usize] = 1.0f64 as libc::c_float;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = alpha;
    SCR_DrawStringExt(x, y, 16i32 as libc::c_float, s, color.as_mut_ptr(),
                      qfalse, noColorEscape);
}
// ignores embedded color control characters
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawBigStringColor(mut x: libc::c_int,
                                                mut y: libc::c_int,
                                                mut s: *const libc::c_char,
                                                mut color: *mut vec_t,
                                                mut noColorEscape: qboolean) {
    SCR_DrawStringExt(x, y, 16i32 as libc::c_float, s, color, qtrue,
                      noColorEscape);
}
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawSmallStringExt(mut x: libc::c_int,
                                                mut y: libc::c_int,
                                                mut string:
                                                    *const libc::c_char,
                                                mut setColor:
                                                    *mut libc::c_float,
                                                mut forceColor: qboolean,
                                                mut noColorEscape: qboolean) {
    let mut color: vec4_t = [0.; 4];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: libc::c_int = 0;
    s = string;
    xx = x;
    re.SetColor.expect("non-null function pointer")(setColor);
    while 0 != *s {
        if 0 != Q_IsColorString(s) as u64 {
            if 0 == forceColor as u64 {
                memcpy(color.as_mut_ptr() as *mut libc::c_void,
                       g_color_table[(*s.offset(1isize) as libc::c_int -
                                          '0' as i32 & 0x7i32) as
                                         usize].as_mut_ptr() as
                           *const libc::c_void,
                       ::std::mem::size_of::<vec4_t>() as libc::c_ulong);
                color[3usize] = *setColor.offset(3isize);
                re.SetColor.expect("non-null function pointer")(color.as_mut_ptr());
            }
            if 0 == noColorEscape as u64 { s = s.offset(2isize); continue ; }
        }
        SCR_DrawSmallChar(xx, y, *s as libc::c_int);
        xx += 8i32;
        s = s.offset(1isize)
    }
    re.SetColor.expect("non-null function pointer")(0 as
                                                        *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn SCR_DrawSmallChar(mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut ch: libc::c_int) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut size: libc::c_float = 0.;
    ch &= 255i32;
    if ch == ' ' as i32 { return }
    if y < -16i32 { return }
    row = ch >> 4i32;
    col = ch & 15i32;
    frow = (row as libc::c_double * 0.0625f64) as libc::c_float;
    fcol = (col as libc::c_double * 0.0625f64) as libc::c_float;
    size = 0.0625f64 as libc::c_float;
    re.DrawStretchPic.expect("non-null function pointer")(x as libc::c_float,
                                                          y as libc::c_float,
                                                          8i32 as
                                                              libc::c_float,
                                                          16i32 as
                                                              libc::c_float,
                                                          fcol, frow,
                                                          fcol + size,
                                                          frow + size,
                                                          cls.charSetShader);
}