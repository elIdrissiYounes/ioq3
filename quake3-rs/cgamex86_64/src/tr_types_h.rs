#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyVert_t {
    pub xyz: crate::src::qcommon::q_shared::vec3_t,
    pub st: [libc::c_float; 2],
    pub modulate: [crate::src::qcommon::q_shared::byte; 4],
}
pub type poly_t = crate::tr_types_h::poly_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly_s {
    pub hShader: crate::src::qcommon::q_shared::qhandle_t,
    pub numVerts: libc::c_int,
    pub verts: *mut crate::tr_types_h::polyVert_t,
}
pub type refEntityType_t = libc::c_uint;
pub const RT_MODEL: crate::tr_types_h::refEntityType_t = 0;
pub const RT_POLY: crate::tr_types_h::refEntityType_t = 1;
pub const RT_SPRITE: crate::tr_types_h::refEntityType_t = 2;
pub const RT_BEAM: crate::tr_types_h::refEntityType_t = 3;
pub const RT_RAIL_CORE: crate::tr_types_h::refEntityType_t = 4;
pub const RT_RAIL_RINGS: crate::tr_types_h::refEntityType_t = 5;
pub const RT_LIGHTNING: crate::tr_types_h::refEntityType_t = 6;
pub const RT_PORTALSURFACE: crate::tr_types_h::refEntityType_t = 7;
// doesn't draw anything, just info for portals
pub const RT_MAX_REF_ENTITY_TYPE: crate::tr_types_h::refEntityType_t = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refEntity_t {
    pub reType: crate::tr_types_h::refEntityType_t,
    pub renderfx: libc::c_int,
    pub hModel: crate::src::qcommon::q_shared::qhandle_t,
    pub lightingOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub shadowPlane: libc::c_float,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub nonNormalizedAxes: crate::src::qcommon::q_shared::qboolean,
    pub origin: [libc::c_float; 3],
    pub frame: libc::c_int,
    pub oldorigin: [libc::c_float; 3],
    pub oldframe: libc::c_int,
    pub backlerp: libc::c_float,
    pub skinNum: libc::c_int,
    pub customSkin: crate::src::qcommon::q_shared::qhandle_t,
    pub customShader: crate::src::qcommon::q_shared::qhandle_t,
    pub shaderRGBA: [crate::src::qcommon::q_shared::byte; 4],
    pub shaderTexCoord: [libc::c_float; 2],
    pub shaderTime: libc::c_float,
    pub radius: libc::c_float,
    pub rotation: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: crate::src::qcommon::q_shared::vec3_t,
    pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub time: libc::c_int,
    pub rdflags: libc::c_int,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub text: [[libc::c_char; 32]; 8],
}
pub type stereoFrame_t = libc::c_uint;
pub const STEREO_CENTER: crate::tr_types_h::stereoFrame_t = 0;
pub const STEREO_LEFT: crate::tr_types_h::stereoFrame_t = 1;
pub const STEREO_RIGHT: crate::tr_types_h::stereoFrame_t = 2;
/*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
pub type textureCompression_t = libc::c_uint;
pub const TC_NONE: crate::tr_types_h::textureCompression_t = 0;
pub const TC_S3TC: crate::tr_types_h::textureCompression_t = 1;
// this is for the GL_EXT_texture_compression_s3tc extension.

// this is for the GL_S3_s3tc extension.
pub const TC_S3TC_ARB: crate::tr_types_h::textureCompression_t = 2;
pub type glDriverType_t = libc::c_uint;
pub const GLDRV_ICD: crate::tr_types_h::glDriverType_t = 0;
// driver is integrated with window system

// WARNING: there are tests that check for

// > GLDRV_ICD for minidriverness, so this

// should always be the lowest value in this

// enum set
pub const GLDRV_STANDALONE: crate::tr_types_h::glDriverType_t = 1;
// driver is a 3Dfx standalone driver

// driver is a non-3Dfx standalone driver
pub const GLDRV_VOODOO: crate::tr_types_h::glDriverType_t = 2;
pub type glHardwareType_t = libc::c_uint;
pub const GLHW_GENERIC: crate::tr_types_h::glHardwareType_t = 0;
// where everything works the way it should
pub const GLHW_3DFX_2D3D: crate::tr_types_h::glHardwareType_t = 1;
// Voodoo Banshee or Voodoo3, relevant since if this is

// the hardware type then there can NOT exist a secondary

// display adapter
pub const GLHW_RIVA128: crate::tr_types_h::glHardwareType_t = 2;
// where you can't interpolate alpha
pub const GLHW_RAGEPRO: crate::tr_types_h::glHardwareType_t = 3;
// where you don't have src*dst

// where you can't modulate alpha on alpha textures
pub const GLHW_PERMEDIA2: crate::tr_types_h::glHardwareType_t = 4;
#[repr(C)]
#[derive(Copy, Clone)]
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
    pub driverType: crate::tr_types_h::glDriverType_t,
    pub hardwareType: crate::tr_types_h::glHardwareType_t,
    pub deviceSupportsGamma: crate::src::qcommon::q_shared::qboolean,
    pub textureCompression: crate::tr_types_h::textureCompression_t,
    pub textureEnvAddAvailable: crate::src::qcommon::q_shared::qboolean,
    pub vidWidth: libc::c_int,
    pub vidHeight: libc::c_int,
    pub windowAspect: libc::c_float,
    pub displayFrequency: libc::c_int,
    pub isFullscreen: crate::src::qcommon::q_shared::qboolean,
    pub stereoEnabled: crate::src::qcommon::q_shared::qboolean,
    pub smpActive: crate::src::qcommon::q_shared::qboolean,
}
