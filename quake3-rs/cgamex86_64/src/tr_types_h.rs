#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyVert_t {
    pub xyz: crate::src::qcommon::q_shared::vec3_t,
    pub st: [f32; 2],
    pub modulate: [crate::src::qcommon::q_shared::byte; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly_s {
    pub hShader: crate::src::qcommon::q_shared::qhandle_t,
    pub numVerts: i32,
    pub verts: *mut crate::tr_types_h::polyVert_t,
}
pub type poly_t = crate::tr_types_h::poly_s;
pub type refEntityType_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refEntity_t {
    pub reType: crate::tr_types_h::refEntityType_t,
    pub renderfx: i32,
    pub hModel: crate::src::qcommon::q_shared::qhandle_t,
    pub lightingOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub shadowPlane: f32,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub nonNormalizedAxes: crate::src::qcommon::q_shared::qboolean,
    pub origin: [f32; 3],
    pub frame: i32,
    pub oldorigin: [f32; 3],
    pub oldframe: i32,
    pub backlerp: f32,
    pub skinNum: i32,
    pub customSkin: crate::src::qcommon::q_shared::qhandle_t,
    pub customShader: crate::src::qcommon::q_shared::qhandle_t,
    pub shaderRGBA: [crate::src::qcommon::q_shared::byte; 4],
    pub shaderTexCoord: [f32; 2],
    pub shaderTime: f32,
    pub radius: f32,
    pub rotation: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refdef_t {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fov_x: f32,
    pub fov_y: f32,
    pub vieworg: crate::src::qcommon::q_shared::vec3_t,
    pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub time: i32,
    pub rdflags: i32,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub text: [[i8; 32]; 8],
}
pub type stereoFrame_t = u32;
/*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
pub type textureCompression_t = u32;
pub type glDriverType_t = u32;
pub type glHardwareType_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glconfig_t {
    pub renderer_string: [i8; 1024],
    pub vendor_string: [i8; 1024],
    pub version_string: [i8; 1024],
    pub extensions_string: [i8; 8192],
    pub maxTextureSize: i32,
    pub numTextureUnits: i32,
    pub colorBits: i32,
    pub depthBits: i32,
    pub stencilBits: i32,
    pub driverType: crate::tr_types_h::glDriverType_t,
    pub hardwareType: crate::tr_types_h::glHardwareType_t,
    pub deviceSupportsGamma: crate::src::qcommon::q_shared::qboolean,
    pub textureCompression: crate::tr_types_h::textureCompression_t,
    pub textureEnvAddAvailable: crate::src::qcommon::q_shared::qboolean,
    pub vidWidth: i32,
    pub vidHeight: i32,
    pub windowAspect: f32,
    pub displayFrequency: i32,
    pub isFullscreen: crate::src::qcommon::q_shared::qboolean,
    pub stereoEnabled: crate::src::qcommon::q_shared::qboolean,
    pub smpActive: crate::src::qcommon::q_shared::qboolean,
}
// doesn't draw anything, just info for portals
pub const RT_MAX_REF_ENTITY_TYPE: crate::tr_types_h::refEntityType_t = 8;
pub const RT_PORTALSURFACE: crate::tr_types_h::refEntityType_t = 7;
pub const RT_LIGHTNING: crate::tr_types_h::refEntityType_t = 6;
pub const RT_RAIL_RINGS: crate::tr_types_h::refEntityType_t = 5;
pub const RT_RAIL_CORE: crate::tr_types_h::refEntityType_t = 4;
pub const RT_BEAM: crate::tr_types_h::refEntityType_t = 3;
pub const RT_SPRITE: crate::tr_types_h::refEntityType_t = 2;
pub const RT_POLY: crate::tr_types_h::refEntityType_t = 1;
pub const RT_MODEL: crate::tr_types_h::refEntityType_t = 0;
pub const STEREO_RIGHT: crate::tr_types_h::stereoFrame_t = 2;
pub const STEREO_LEFT: crate::tr_types_h::stereoFrame_t = 1;
pub const STEREO_CENTER: crate::tr_types_h::stereoFrame_t = 0;
// this is for the GL_EXT_texture_compression_s3tc extension.

// this is for the GL_S3_s3tc extension.
pub const TC_S3TC_ARB: crate::tr_types_h::textureCompression_t = 2;
pub const TC_S3TC: crate::tr_types_h::textureCompression_t = 1;
pub const TC_NONE: crate::tr_types_h::textureCompression_t = 0;
// driver is a 3Dfx standalone driver

// driver is a non-3Dfx standalone driver
pub const GLDRV_VOODOO: crate::tr_types_h::glDriverType_t = 2;
// driver is integrated with window system

// WARNING: there are tests that check for

// > GLDRV_ICD for minidriverness, so this

// should always be the lowest value in this

// enum set
pub const GLDRV_STANDALONE: crate::tr_types_h::glDriverType_t = 1;
pub const GLDRV_ICD: crate::tr_types_h::glDriverType_t = 0;
// where you don't have src*dst

// where you can't modulate alpha on alpha textures
pub const GLHW_PERMEDIA2: crate::tr_types_h::glHardwareType_t = 4;
// where you can't interpolate alpha
pub const GLHW_RAGEPRO: crate::tr_types_h::glHardwareType_t = 3;
// Voodoo Banshee or Voodoo3, relevant since if this is

// the hardware type then there can NOT exist a secondary

// display adapter
pub const GLHW_RIVA128: crate::tr_types_h::glHardwareType_t = 2;
// where everything works the way it should
pub const GLHW_3DFX_2D3D: crate::tr_types_h::glHardwareType_t = 1;
pub const GLHW_GENERIC: crate::tr_types_h::glHardwareType_t = 0;
