pub type glIndex_t = libc::c_uint;
pub type dlight_t = crate::tr_local_h::dlight_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlight_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub color: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub transformed: crate::src::qcommon::q_shared::vec3_t,
    pub additive: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trRefEntity_t {
    pub e: crate::tr_types_h::refEntity_t,
    pub axisLength: libc::c_float,
    pub needDlights: crate::src::qcommon::q_shared::qboolean,
    pub lightingCalculated: crate::src::qcommon::q_shared::qboolean,
    pub lightDir: crate::src::qcommon::q_shared::vec3_t,
    pub ambientLight: crate::src::qcommon::q_shared::vec3_t,
    pub ambientLightInt: libc::c_int,
    pub directedLight: crate::src::qcommon::q_shared::vec3_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orientationr_t {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub viewOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub modelMatrix: [libc::c_float; 16],
}
pub const SS_BAD: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const SS_PORTAL: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const SS_ENVIRONMENT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const SS_OPAQUE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const SS_DECAL: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const SS_SEE_THROUGH: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 5;
pub const SS_BANNER: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 6;
pub const SS_FOG: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 7;
pub const SS_UNDERWATER: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
pub const SS_BLEND0: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 9;
pub const SS_BLEND1: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 10;
pub const SS_BLEND2: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 11;
pub const SS_BLEND3: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 12;
pub const SS_BLEND6: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 13;
pub const SS_STENCIL_SHADOW: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 14;
pub const SS_ALMOST_NEAREST: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 15;
pub const SS_NEAREST: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 16;
pub type genFunc_t = libc::c_uint;
pub const GF_NONE: crate::tr_local_h::genFunc_t = 0;
pub const GF_SIN: crate::tr_local_h::genFunc_t = 1;
pub const GF_SQUARE: crate::tr_local_h::genFunc_t = 2;
pub const GF_TRIANGLE: crate::tr_local_h::genFunc_t = 3;
pub const GF_SAWTOOTH: crate::tr_local_h::genFunc_t = 4;
pub const GF_INVERSE_SAWTOOTH: crate::tr_local_h::genFunc_t = 5;
pub const GF_NOISE: crate::tr_local_h::genFunc_t = 6;
pub type deform_t = libc::c_uint;
pub const DEFORM_NONE: crate::tr_local_h::deform_t = 0;
pub const DEFORM_WAVE: crate::tr_local_h::deform_t = 1;
pub const DEFORM_NORMALS: crate::tr_local_h::deform_t = 2;
pub const DEFORM_BULGE: crate::tr_local_h::deform_t = 3;
pub const DEFORM_MOVE: crate::tr_local_h::deform_t = 4;
pub const DEFORM_PROJECTION_SHADOW: crate::tr_local_h::deform_t = 5;
pub const DEFORM_AUTOSPRITE: crate::tr_local_h::deform_t = 6;
pub const DEFORM_AUTOSPRITE2: crate::tr_local_h::deform_t = 7;
pub const DEFORM_TEXT0: crate::tr_local_h::deform_t = 8;
pub const DEFORM_TEXT1: crate::tr_local_h::deform_t = 9;
pub const DEFORM_TEXT2: crate::tr_local_h::deform_t = 10;
pub const DEFORM_TEXT3: crate::tr_local_h::deform_t = 11;
pub const DEFORM_TEXT4: crate::tr_local_h::deform_t = 12;
pub const DEFORM_TEXT5: crate::tr_local_h::deform_t = 13;
pub const DEFORM_TEXT6: crate::tr_local_h::deform_t = 14;
pub const DEFORM_TEXT7: crate::tr_local_h::deform_t = 15;
pub type alphaGen_t = libc::c_uint;
pub const AGEN_IDENTITY: crate::tr_local_h::alphaGen_t = 0;
pub const AGEN_SKIP: crate::tr_local_h::alphaGen_t = 1;
pub const AGEN_ENTITY: crate::tr_local_h::alphaGen_t = 2;
pub const AGEN_ONE_MINUS_ENTITY: crate::tr_local_h::alphaGen_t = 3;
pub const AGEN_VERTEX: crate::tr_local_h::alphaGen_t = 4;
pub const AGEN_ONE_MINUS_VERTEX: crate::tr_local_h::alphaGen_t = 5;
pub const AGEN_LIGHTING_SPECULAR: crate::tr_local_h::alphaGen_t = 6;
pub const AGEN_WAVEFORM: crate::tr_local_h::alphaGen_t = 7;
pub const AGEN_PORTAL: crate::tr_local_h::alphaGen_t = 8;
pub const AGEN_CONST: crate::tr_local_h::alphaGen_t = 9;
pub type colorGen_t = libc::c_uint;
pub const CGEN_BAD: crate::tr_local_h::colorGen_t = 0;
pub const CGEN_IDENTITY_LIGHTING: crate::tr_local_h::colorGen_t = 1;
pub const CGEN_IDENTITY: crate::tr_local_h::colorGen_t = 2;
pub const CGEN_ENTITY: crate::tr_local_h::colorGen_t = 3;
pub const CGEN_ONE_MINUS_ENTITY: crate::tr_local_h::colorGen_t = 4;
pub const CGEN_EXACT_VERTEX: crate::tr_local_h::colorGen_t = 5;
pub const CGEN_VERTEX: crate::tr_local_h::colorGen_t = 6;
pub const CGEN_ONE_MINUS_VERTEX: crate::tr_local_h::colorGen_t = 7;
pub const CGEN_WAVEFORM: crate::tr_local_h::colorGen_t = 8;
pub const CGEN_LIGHTING_DIFFUSE: crate::tr_local_h::colorGen_t = 9;
pub const CGEN_FOG: crate::tr_local_h::colorGen_t = 10;
pub const CGEN_CONST: crate::tr_local_h::colorGen_t = 11;
pub type texCoordGen_t = libc::c_uint;
pub const TCGEN_BAD: crate::tr_local_h::texCoordGen_t = 0;
pub const TCGEN_IDENTITY: crate::tr_local_h::texCoordGen_t = 1;
pub const TCGEN_LIGHTMAP: crate::tr_local_h::texCoordGen_t = 2;
pub const TCGEN_TEXTURE: crate::tr_local_h::texCoordGen_t = 3;
pub const TCGEN_ENVIRONMENT_MAPPED: crate::tr_local_h::texCoordGen_t = 4;
pub const TCGEN_FOG: crate::tr_local_h::texCoordGen_t = 5;
pub const TCGEN_VECTOR: crate::tr_local_h::texCoordGen_t = 6;
pub type acff_t = libc::c_uint;
pub const ACFF_NONE: crate::tr_local_h::acff_t = 0;
pub const ACFF_MODULATE_RGB: crate::tr_local_h::acff_t = 1;
pub const ACFF_MODULATE_RGBA: crate::tr_local_h::acff_t = 2;
pub const ACFF_MODULATE_ALPHA: crate::tr_local_h::acff_t = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waveForm_t {
    pub func: crate::tr_local_h::genFunc_t,
    pub base: libc::c_float,
    pub amplitude: libc::c_float,
    pub phase: libc::c_float,
    pub frequency: libc::c_float,
}
pub type texMod_t = libc::c_uint;
pub const TMOD_NONE: crate::tr_local_h::texMod_t = 0;
pub const TMOD_TRANSFORM: crate::tr_local_h::texMod_t = 1;
pub const TMOD_TURBULENT: crate::tr_local_h::texMod_t = 2;
pub const TMOD_SCROLL: crate::tr_local_h::texMod_t = 3;
pub const TMOD_SCALE: crate::tr_local_h::texMod_t = 4;
pub const TMOD_STRETCH: crate::tr_local_h::texMod_t = 5;
pub const TMOD_ROTATE: crate::tr_local_h::texMod_t = 6;
pub const TMOD_ENTITY_TRANSLATE: crate::tr_local_h::texMod_t = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct deformStage_t {
    pub deformation: crate::tr_local_h::deform_t,
    pub moveVector: crate::src::qcommon::q_shared::vec3_t,
    pub deformationWave: crate::tr_local_h::waveForm_t,
    pub deformationSpread: libc::c_float,
    pub bulgeWidth: libc::c_float,
    pub bulgeHeight: libc::c_float,
    pub bulgeSpeed: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct texModInfo_t {
    pub type_0: crate::tr_local_h::texMod_t,
    pub wave: crate::tr_local_h::waveForm_t,
    pub matrix: [[libc::c_float; 2]; 2],
    pub translate: [libc::c_float; 2],
    pub scale: [libc::c_float; 2],
    pub scroll: [libc::c_float; 2],
    pub rotateSpeed: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct textureBundle_t {
    pub image: [*mut crate::tr_common_h::image_t; 8],
    pub numImageAnimations: libc::c_int,
    pub imageAnimationSpeed: libc::c_float,
    pub tcGen: crate::tr_local_h::texCoordGen_t,
    pub tcGenVectors: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub numTexMods: libc::c_int,
    pub texMods: *mut crate::tr_local_h::texModInfo_t,
    pub videoMapHandle: libc::c_int,
    pub isLightmap: crate::src::qcommon::q_shared::qboolean,
    pub isVideoMap: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shaderStage_t {
    pub active: crate::src::qcommon::q_shared::qboolean,
    pub bundle: [crate::tr_local_h::textureBundle_t; 2],
    pub rgbWave: crate::tr_local_h::waveForm_t,
    pub rgbGen: crate::tr_local_h::colorGen_t,
    pub alphaWave: crate::tr_local_h::waveForm_t,
    pub alphaGen: crate::tr_local_h::alphaGen_t,
    pub constantColor: [crate::src::qcommon::q_shared::byte; 4],
    pub stateBits: libc::c_uint,
    pub adjustColorsForFog: crate::tr_local_h::acff_t,
    pub isDetail: crate::src::qcommon::q_shared::qboolean,
}
pub type cullType_t = libc::c_uint;
pub const CT_FRONT_SIDED: crate::tr_local_h::cullType_t = 0;
pub const CT_BACK_SIDED: crate::tr_local_h::cullType_t = 1;
pub const CT_TWO_SIDED: crate::tr_local_h::cullType_t = 2;
pub type fogPass_t = libc::c_uint;
pub const FP_NONE: crate::tr_local_h::fogPass_t = 0;
pub const FP_EQUAL: crate::tr_local_h::fogPass_t = 1;
pub const FP_LE: crate::tr_local_h::fogPass_t = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skyParms_t {
    pub cloudHeight: libc::c_float,
    pub outerbox: [*mut crate::tr_common_h::image_t; 6],
    pub innerbox: [*mut crate::tr_common_h::image_t; 6],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fogParms_t {
    pub color: crate::src::qcommon::q_shared::vec3_t,
    pub depthForOpaque: libc::c_float,
}
pub type shader_t = crate::tr_local_h::shader_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shader_s {
    pub name: [libc::c_char; 64],
    pub lightmapIndex: libc::c_int,
    pub index: libc::c_int,
    pub sortedIndex: libc::c_int,
    pub sort: libc::c_float,
    pub defaultShader: crate::src::qcommon::q_shared::qboolean,
    pub explicitlyDefined: crate::src::qcommon::q_shared::qboolean,
    pub surfaceFlags: libc::c_int,
    pub contentFlags: libc::c_int,
    pub entityMergable: crate::src::qcommon::q_shared::qboolean,
    pub isSky: crate::src::qcommon::q_shared::qboolean,
    pub sky: crate::tr_local_h::skyParms_t,
    pub fogParms: crate::tr_local_h::fogParms_t,
    pub portalRange: libc::c_float,
    pub multitextureEnv: libc::c_int,
    pub cullType: crate::tr_local_h::cullType_t,
    pub polygonOffset: crate::src::qcommon::q_shared::qboolean,
    pub noMipMaps: crate::src::qcommon::q_shared::qboolean,
    pub noPicMip: crate::src::qcommon::q_shared::qboolean,
    pub fogPass: crate::tr_local_h::fogPass_t,
    pub needsNormal: crate::src::qcommon::q_shared::qboolean,
    pub needsST1: crate::src::qcommon::q_shared::qboolean,
    pub needsST2: crate::src::qcommon::q_shared::qboolean,
    pub needsColor: crate::src::qcommon::q_shared::qboolean,
    pub numDeforms: libc::c_int,
    pub deforms: [crate::tr_local_h::deformStage_t; 3],
    pub numUnfoggedPasses: libc::c_int,
    pub stages: [*mut crate::tr_local_h::shaderStage_t; 8],
    pub optimalStageIteratorFunc: Option<unsafe extern "C" fn() -> ()>,
    pub clampTime: libc::c_double,
    pub timeOffset: libc::c_double,
    pub remappedShader: *mut crate::tr_local_h::shader_s,
    pub next: *mut crate::tr_local_h::shader_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trRefdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: crate::src::qcommon::q_shared::vec3_t,
    pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub stereoFrame: crate::tr_types_h::stereoFrame_t,
    pub time: libc::c_int,
    pub rdflags: libc::c_int,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub areamaskModified: crate::src::qcommon::q_shared::qboolean,
    pub floatTime: libc::c_double,
    pub text: [[libc::c_char; 32]; 8],
    pub num_entities: libc::c_int,
    pub entities: *mut crate::tr_local_h::trRefEntity_t,
    pub num_dlights: libc::c_int,
    pub dlights: *mut crate::tr_local_h::dlight_s,
    pub numPolys: libc::c_int,
    pub polys: *mut crate::tr_local_h::srfPoly_s,
    pub numDrawSurfs: libc::c_int,
    pub drawSurfs: *mut crate::tr_local_h::drawSurf_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skinSurface_t {
    pub name: [libc::c_char; 64],
    pub shader: *mut crate::tr_local_h::shader_t,
}
pub type skin_t = crate::tr_local_h::skin_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skin_s {
    pub name: [libc::c_char; 64],
    pub numSurfaces: libc::c_int,
    pub surfaces: *mut crate::tr_local_h::skinSurface_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fog_t {
    pub originalBrushNumber: libc::c_int,
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub colorInt: libc::c_uint,
    pub tcScale: libc::c_float,
    pub parms: crate::tr_local_h::fogParms_t,
    pub hasSurface: crate::src::qcommon::q_shared::qboolean,
    pub surface: [libc::c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viewParms_t {
    pub or: crate::tr_local_h::orientationr_t,
    pub world: crate::tr_local_h::orientationr_t,
    pub pvsOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub isPortal: crate::src::qcommon::q_shared::qboolean,
    pub isMirror: crate::src::qcommon::q_shared::qboolean,
    pub frameSceneNum: libc::c_int,
    pub frameCount: libc::c_int,
    pub portalPlane: crate::src::qcommon::q_shared::cplane_t,
    pub viewportX: libc::c_int,
    pub viewportY: libc::c_int,
    pub viewportWidth: libc::c_int,
    pub viewportHeight: libc::c_int,
    pub fovX: libc::c_float,
    pub fovY: libc::c_float,
    pub projectionMatrix: [libc::c_float; 16],
    pub frustum: [crate::src::qcommon::q_shared::cplane_t; 4],
    pub visBounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub zFar: libc::c_float,
    pub stereoFrame: crate::tr_types_h::stereoFrame_t,
}
pub type surfaceType_t = libc::c_uint;
pub const SF_BAD: crate::tr_local_h::surfaceType_t = 0;
pub const SF_SKIP: crate::tr_local_h::surfaceType_t = 1;
pub const SF_FACE: crate::tr_local_h::surfaceType_t = 2;
pub const SF_GRID: crate::tr_local_h::surfaceType_t = 3;
pub const SF_TRIANGLES: crate::tr_local_h::surfaceType_t = 4;
pub const SF_POLY: crate::tr_local_h::surfaceType_t = 5;
pub const SF_MD3: crate::tr_local_h::surfaceType_t = 6;
pub const SF_MDR: crate::tr_local_h::surfaceType_t = 7;
pub const SF_IQM: crate::tr_local_h::surfaceType_t = 8;
pub const SF_FLARE: crate::tr_local_h::surfaceType_t = 9;
pub const SF_ENTITY: crate::tr_local_h::surfaceType_t = 10;
pub const SF_NUM_SURFACE_TYPES: crate::tr_local_h::surfaceType_t = 11;
pub const SF_MAX: crate::tr_local_h::surfaceType_t = 2147483647;
pub type drawSurf_t = crate::tr_local_h::drawSurf_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawSurf_s {
    pub sort: libc::c_uint,
    pub surface: *mut crate::tr_local_h::surfaceType_t,
}
pub type srfPoly_t = crate::tr_local_h::srfPoly_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfPoly_s {
    pub surfaceType: crate::tr_local_h::surfaceType_t,
    pub hShader: crate::src::qcommon::q_shared::qhandle_t,
    pub fogIndex: libc::c_int,
    pub numVerts: libc::c_int,
    pub verts: *mut crate::tr_types_h::polyVert_t,
}
pub type srfFlare_t = crate::tr_local_h::srfFlare_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfFlare_s {
    pub surfaceType: crate::tr_local_h::surfaceType_t,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub color: crate::src::qcommon::q_shared::vec3_t,
}
pub type srfGridMesh_t = crate::tr_local_h::srfGridMesh_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfGridMesh_s {
    pub surfaceType: crate::tr_local_h::surfaceType_t,
    pub dlightBits: libc::c_int,
    pub meshBounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub meshRadius: libc::c_float,
    pub lodOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lodRadius: libc::c_float,
    pub lodFixed: libc::c_int,
    pub lodStitched: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub widthLodError: *mut libc::c_float,
    pub heightLodError: *mut libc::c_float,
    pub verts: [crate::qfiles_h::drawVert_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfSurfaceFace_t {
    pub surfaceType: crate::tr_local_h::surfaceType_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub dlightBits: libc::c_int,
    pub numPoints: libc::c_int,
    pub numIndices: libc::c_int,
    pub ofsIndices: libc::c_int,
    pub points: [[libc::c_float; 8]; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfTriangles_t {
    pub surfaceType: crate::tr_local_h::surfaceType_t,
    pub dlightBits: libc::c_int,
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub numIndexes: libc::c_int,
    pub indexes: *mut libc::c_int,
    pub numVerts: libc::c_int,
    pub verts: *mut crate::qfiles_h::drawVert_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmData_t {
    pub num_vertexes: libc::c_int,
    pub num_triangles: libc::c_int,
    pub num_frames: libc::c_int,
    pub num_surfaces: libc::c_int,
    pub num_joints: libc::c_int,
    pub num_poses: libc::c_int,
    pub surfaces: *mut crate::tr_local_h::srfIQModel_s,
    pub triangles: *mut libc::c_int,
    pub positions: *mut libc::c_float,
    pub texcoords: *mut libc::c_float,
    pub normals: *mut libc::c_float,
    pub tangents: *mut libc::c_float,
    pub colors: *mut crate::src::qcommon::q_shared::byte,
    pub influences: *mut libc::c_int,
    pub influenceBlendIndexes: *mut crate::src::qcommon::q_shared::byte,
    pub influenceBlendWeights: crate::tr_local_h::C2RustUnnamed_119,
    pub blendWeightsType: libc::c_int,
    pub jointNames: *mut libc::c_char,
    pub jointParents: *mut libc::c_int,
    pub jointMats: *mut libc::c_float,
    pub poseMats: *mut libc::c_float,
    pub bounds: *mut libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_119 {
    pub f: *mut libc::c_float,
    pub b: *mut crate::src::qcommon::q_shared::byte,
}
pub type srfIQModel_t = crate::tr_local_h::srfIQModel_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfIQModel_s {
    pub surfaceType: crate::tr_local_h::surfaceType_t,
    pub name: [libc::c_char; 64],
    pub shader: *mut crate::tr_local_h::shader_t,
    pub data: *mut crate::tr_local_h::iqmData_t,
    pub first_vertex: libc::c_int,
    pub num_vertexes: libc::c_int,
    pub first_triangle: libc::c_int,
    pub num_triangles: libc::c_int,
    pub first_influence: libc::c_int,
    pub num_influences: libc::c_int,
}
pub type msurface_t = crate::tr_local_h::msurface_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msurface_s {
    pub viewCount: libc::c_int,
    pub shader: *mut crate::tr_local_h::shader_s,
    pub fogIndex: libc::c_int,
    pub data: *mut crate::tr_local_h::surfaceType_t,
}
pub type mnode_t = crate::tr_local_h::mnode_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub parent: *mut crate::tr_local_h::mnode_s,
    pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
    pub children: [*mut crate::tr_local_h::mnode_s; 2],
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub firstmarksurface: *mut *mut crate::tr_local_h::msurface_t,
    pub nummarksurfaces: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmodel_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub firstSurface: *mut crate::tr_local_h::msurface_t,
    pub numSurfaces: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct world_t {
    pub name: [libc::c_char; 64],
    pub baseName: [libc::c_char; 64],
    pub dataSize: libc::c_int,
    pub numShaders: libc::c_int,
    pub shaders: *mut crate::qfiles_h::dshader_t,
    pub bmodels: *mut crate::tr_local_h::bmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut crate::src::qcommon::q_shared::cplane_t,
    pub numnodes: libc::c_int,
    pub numDecisionNodes: libc::c_int,
    pub nodes: *mut crate::tr_local_h::mnode_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut crate::tr_local_h::msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut crate::tr_local_h::msurface_t,
    pub numfogs: libc::c_int,
    pub fogs: *mut crate::tr_local_h::fog_t,
    pub lightGridOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridSize: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridInverseSize: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridBounds: [libc::c_int; 3],
    pub lightGridData: *mut crate::src::qcommon::q_shared::byte,
    pub numClusters: libc::c_int,
    pub clusterBytes: libc::c_int,
    pub vis: *const crate::src::qcommon::q_shared::byte,
    pub novis: *mut crate::src::qcommon::q_shared::byte,
    pub entityString: *mut libc::c_char,
    pub entityParsePoint: *mut libc::c_char,
}
pub type modtype_t = libc::c_uint;
pub const MOD_BAD: crate::tr_local_h::modtype_t = 0;
pub const MOD_BRUSH: crate::tr_local_h::modtype_t = 1;
pub const MOD_MESH: crate::tr_local_h::modtype_t = 2;
pub const MOD_MDR: crate::tr_local_h::modtype_t = 3;
pub const MOD_IQM: crate::tr_local_h::modtype_t = 4;
pub type model_t = crate::tr_local_h::model_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub type_0: crate::tr_local_h::modtype_t,
    pub index: libc::c_int,
    pub dataSize: libc::c_int,
    pub bmodel: *mut crate::tr_local_h::bmodel_t,
    pub md3: [*mut crate::qfiles_h::md3Header_t; 3],
    pub modelData: *mut libc::c_void,
    pub numLods: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frontEndCounters_t {
    pub c_sphere_cull_patch_in: libc::c_int,
    pub c_sphere_cull_patch_clip: libc::c_int,
    pub c_sphere_cull_patch_out: libc::c_int,
    pub c_box_cull_patch_in: libc::c_int,
    pub c_box_cull_patch_clip: libc::c_int,
    pub c_box_cull_patch_out: libc::c_int,
    pub c_sphere_cull_md3_in: libc::c_int,
    pub c_sphere_cull_md3_clip: libc::c_int,
    pub c_sphere_cull_md3_out: libc::c_int,
    pub c_box_cull_md3_in: libc::c_int,
    pub c_box_cull_md3_clip: libc::c_int,
    pub c_box_cull_md3_out: libc::c_int,
    pub c_leafs: libc::c_int,
    pub c_dlightSurfaces: libc::c_int,
    pub c_dlightSurfacesCulled: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glstate_t {
    pub currenttextures: [libc::c_int; 2],
    pub currenttmu: libc::c_int,
    pub finishCalled: crate::src::qcommon::q_shared::qboolean,
    pub texEnv: [libc::c_int; 2],
    pub faceCulling: libc::c_int,
    pub glStateBits: libc::c_ulong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndCounters_t {
    pub c_surfaces: libc::c_int,
    pub c_shaders: libc::c_int,
    pub c_vertexes: libc::c_int,
    pub c_indexes: libc::c_int,
    pub c_totalIndexes: libc::c_int,
    pub c_overDraw: libc::c_float,
    pub c_dlightVertexes: libc::c_int,
    pub c_dlightIndexes: libc::c_int,
    pub c_flareAdds: libc::c_int,
    pub c_flareTests: libc::c_int,
    pub c_flareRenders: libc::c_int,
    pub msec: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndState_t {
    pub refdef: crate::tr_local_h::trRefdef_t,
    pub viewParms: crate::tr_local_h::viewParms_t,
    pub or: crate::tr_local_h::orientationr_t,
    pub pc: crate::tr_local_h::backEndCounters_t,
    pub isHyperspace: crate::src::qcommon::q_shared::qboolean,
    pub currentEntity: *mut crate::tr_local_h::trRefEntity_t,
    pub skyRenderedThisView: crate::src::qcommon::q_shared::qboolean,
    pub projection2D: crate::src::qcommon::q_shared::qboolean,
    pub color2D: [crate::src::qcommon::q_shared::byte; 4],
    pub vertexes2D: crate::src::qcommon::q_shared::qboolean,
    pub entity2D: crate::tr_local_h::trRefEntity_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trGlobals_t {
    pub registered: crate::src::qcommon::q_shared::qboolean,
    pub visCount: libc::c_int,
    pub frameCount: libc::c_int,
    pub sceneCount: libc::c_int,
    pub viewCount: libc::c_int,
    pub frameSceneNum: libc::c_int,
    pub worldMapLoaded: crate::src::qcommon::q_shared::qboolean,
    pub world: *mut crate::tr_local_h::world_t,
    pub externalVisData: *const crate::src::qcommon::q_shared::byte,
    pub defaultImage: *mut crate::tr_common_h::image_t,
    pub scratchImage: [*mut crate::tr_common_h::image_t; 32],
    pub fogImage: *mut crate::tr_common_h::image_t,
    pub dlightImage: *mut crate::tr_common_h::image_t,
    pub flareImage: *mut crate::tr_common_h::image_t,
    pub whiteImage: *mut crate::tr_common_h::image_t,
    pub identityLightImage: *mut crate::tr_common_h::image_t,
    pub defaultShader: *mut crate::tr_local_h::shader_t,
    pub shadowShader: *mut crate::tr_local_h::shader_t,
    pub projectionShadowShader: *mut crate::tr_local_h::shader_t,
    pub flareShader: *mut crate::tr_local_h::shader_t,
    pub sunShader: *mut crate::tr_local_h::shader_t,
    pub numLightmaps: libc::c_int,
    pub lightmaps: *mut *mut crate::tr_common_h::image_t,
    pub currentEntity: *mut crate::tr_local_h::trRefEntity_t,
    pub worldEntity: crate::tr_local_h::trRefEntity_t,
    pub currentEntityNum: libc::c_int,
    pub shiftedEntityNum: libc::c_int,
    pub currentModel: *mut crate::tr_local_h::model_t,
    pub viewParms: crate::tr_local_h::viewParms_t,
    pub identityLight: libc::c_float,
    pub identityLightByte: libc::c_int,
    pub overbrightBits: libc::c_int,
    pub or: crate::tr_local_h::orientationr_t,
    pub refdef: crate::tr_local_h::trRefdef_t,
    pub viewCluster: libc::c_int,
    pub sunLight: crate::src::qcommon::q_shared::vec3_t,
    pub sunDirection: crate::src::qcommon::q_shared::vec3_t,
    pub pc: crate::tr_local_h::frontEndCounters_t,
    pub frontEndMsec: libc::c_int,
    pub models: [*mut crate::tr_local_h::model_t; 1024],
    pub numModels: libc::c_int,
    pub numImages: libc::c_int,
    pub images: [*mut crate::tr_common_h::image_t; 2048],
    pub numShaders: libc::c_int,
    pub shaders: [*mut crate::tr_local_h::shader_t; 16384],
    pub sortedShaders: [*mut crate::tr_local_h::shader_t; 16384],
    pub numSkins: libc::c_int,
    pub skins: [*mut crate::tr_local_h::skin_t; 1024],
    pub sinTable: [libc::c_float; 1024],
    pub squareTable: [libc::c_float; 1024],
    pub triangleTable: [libc::c_float; 1024],
    pub sawToothTable: [libc::c_float; 1024],
    pub inverseSawToothTable: [libc::c_float; 1024],
    pub fogTable: [libc::c_float; 256],
}
pub type color4ub_t = [crate::src::qcommon::q_shared::byte; 4];
pub type stageVars_t = crate::tr_local_h::stageVars;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stageVars {
    pub colors: [crate::tr_local_h::color4ub_t; 1000],
    pub texcoords: [[crate::src::qcommon::q_shared::vec2_t; 1000]; 2],
}
pub type shaderCommands_t = crate::tr_local_h::shaderCommands_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shaderCommands_s {
    pub indexes: [crate::tr_local_h::glIndex_t; 6000],
    pub xyz: [crate::src::qcommon::q_shared::vec4_t; 1000],
    pub normal: [crate::src::qcommon::q_shared::vec4_t; 1000],
    pub texCoords: [[crate::src::qcommon::q_shared::vec2_t; 2]; 1000],
    pub vertexColors: [crate::tr_local_h::color4ub_t; 1000],
    pub vertexDlightBits: [libc::c_int; 1000],
    pub svars: crate::tr_local_h::stageVars_t,
    pub constantColor255: [crate::tr_local_h::color4ub_t; 1000],
    pub shader: *mut crate::tr_local_h::shader_t,
    pub shaderTime: libc::c_double,
    pub fogNum: libc::c_int,
    pub dlightBits: libc::c_int,
    pub numIndexes: libc::c_int,
    pub numVertexes: libc::c_int,
    pub numPasses: libc::c_int,
    pub currentStageIteratorFunc: Option<unsafe extern "C" fn() -> ()>,
    pub xstages: *mut *mut crate::tr_local_h::shaderStage_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renderCommandList_t {
    pub cmds: [crate::src::qcommon::q_shared::byte; 262144],
    pub used: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setColorCommand_t {
    pub commandId: libc::c_int,
    pub color: [libc::c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawBufferCommand_t {
    pub commandId: libc::c_int,
    pub buffer: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swapBuffersCommand_t {
    pub commandId: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stretchPicCommand_t {
    pub commandId: libc::c_int,
    pub shader: *mut crate::tr_local_h::shader_t,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub w: libc::c_float,
    pub h: libc::c_float,
    pub s1: libc::c_float,
    pub t1: libc::c_float,
    pub s2: libc::c_float,
    pub t2: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawSurfsCommand_t {
    pub commandId: libc::c_int,
    pub refdef: crate::tr_local_h::trRefdef_t,
    pub viewParms: crate::tr_local_h::viewParms_t,
    pub drawSurfs: *mut crate::tr_local_h::drawSurf_t,
    pub numDrawSurfs: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct screenshotCommand_t {
    pub commandId: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fileName: *mut libc::c_char,
    pub jpeg: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct videoFrameCommand_t {
    pub commandId: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub captureBuffer: *mut crate::src::qcommon::q_shared::byte,
    pub encodeBuffer: *mut crate::src::qcommon::q_shared::byte,
    pub motionJpeg: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct colorMaskCommand_t {
    pub commandId: libc::c_int,
    pub rgba: [crate::stdlib::GLboolean; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clearDepthCommand_t {
    pub commandId: libc::c_int,
}
pub const RC_END_OF_LIST: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const RC_SET_COLOR: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const RC_STRETCH_PIC: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const RC_DRAW_SURFS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const RC_DRAW_BUFFER: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const RC_SWAP_BUFFERS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 5;
pub const RC_SCREENSHOT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 6;
pub const RC_VIDEOFRAME: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 7;
pub const RC_COLORMASK: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
pub const RC_CLEARDEPTH: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndData_t {
    pub drawSurfs: [crate::tr_local_h::drawSurf_t; 65536],
    pub dlights: [crate::tr_local_h::dlight_t; 32],
    pub entities: [crate::tr_local_h::trRefEntity_t; 1023],
    pub polys: *mut crate::tr_local_h::srfPoly_t,
    pub polyVerts: *mut crate::tr_types_h::polyVert_t,
    pub commands: crate::tr_local_h::renderCommandList_t,
}
