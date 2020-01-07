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
pub const TC_S3TC_ARB: crate::tr_types_h::textureCompression_t = 2;
pub const TC_S3TC: crate::tr_types_h::textureCompression_t = 1;
pub const TC_NONE: crate::tr_types_h::textureCompression_t = 0;
pub const GLDRV_VOODOO: crate::tr_types_h::glDriverType_t = 2;
pub const GLDRV_STANDALONE: crate::tr_types_h::glDriverType_t = 1;
pub const GLDRV_ICD: crate::tr_types_h::glDriverType_t = 0;
pub const GLHW_PERMEDIA2: crate::tr_types_h::glHardwareType_t = 4;
pub const GLHW_RAGEPRO: crate::tr_types_h::glHardwareType_t = 3;
pub const GLHW_RIVA128: crate::tr_types_h::glHardwareType_t = 2;
pub const GLHW_3DFX_2D3D: crate::tr_types_h::glHardwareType_t = 1;
pub const GLHW_GENERIC: crate::tr_types_h::glHardwareType_t = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyVert_t {
    pub xyz: crate::src::qcommon::q_shared::vec3_t,
    pub st: [f32; 2],
    pub modulate: [crate::src::qcommon::q_shared::byte; 4],
}
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
pub type stereoFrame_t = u32;
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
