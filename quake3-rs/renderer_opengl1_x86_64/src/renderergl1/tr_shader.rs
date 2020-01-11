use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::intptr_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_BeginParseSession;
pub use crate::src::qcommon::q_shared::COM_Compress;
pub use crate::src::qcommon::q_shared::COM_GetCurrentParseLine;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::SkipBracedSection;
pub use crate::src::qcommon::q_shared::SkipRestOfLine;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
pub use crate::tr_public_h::refimport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::stereoFrame_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::STEREO_CENTER;
pub use crate::tr_types_h::STEREO_LEFT;
pub use crate::tr_types_h::STEREO_RIGHT;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::src::renderergl1::tr_backend::backEndData;
pub use crate::src::renderergl1::tr_image::R_FindImageFile;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::r_detailTextures;
pub use crate::src::renderergl1::tr_init::r_greyscale;
pub use crate::src::renderergl1::tr_init::r_ignoreFastPath;
pub use crate::src::renderergl1::tr_init::r_printShaders;
pub use crate::src::renderergl1::tr_init::r_uiFullScreen;
pub use crate::src::renderergl1::tr_init::r_vertexLight;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_DecomposeSort;
pub use crate::src::renderergl1::tr_shade::RB_StageIteratorGeneric;
pub use crate::src::renderergl1::tr_shade::RB_StageIteratorLightmappedMultitexture;
pub use crate::src::renderergl1::tr_shade::RB_StageIteratorVertexLitTexture;
pub use crate::src::renderergl1::tr_shader::stdlib_float_h::atof;
pub use crate::src::renderergl1::tr_sky::RB_StageIteratorSky;
pub use crate::src::renderergl1::tr_sky::R_InitSkyTexCoords;

pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLuint;
pub use crate::tr_common_h::image_s;
pub use crate::tr_common_h::image_t;
pub use crate::tr_common_h::imgFlags_t;
pub use crate::tr_common_h::imgType_t;
pub use crate::tr_common_h::IMGFLAG_CLAMPTOEDGE;
pub use crate::tr_common_h::IMGFLAG_CUBEMAP;
pub use crate::tr_common_h::IMGFLAG_GENNORMALMAP;
pub use crate::tr_common_h::IMGFLAG_MIPMAP;
pub use crate::tr_common_h::IMGFLAG_NOLIGHTSCALE;
pub use crate::tr_common_h::IMGFLAG_NONE;
pub use crate::tr_common_h::IMGFLAG_NO_COMPRESSION;
pub use crate::tr_common_h::IMGFLAG_PICMIP;
pub use crate::tr_common_h::IMGTYPE_COLORALPHA;
pub use crate::tr_common_h::IMGTYPE_DELUXE;
pub use crate::tr_common_h::IMGTYPE_NORMAL;
pub use crate::tr_common_h::IMGTYPE_NORMALHEIGHT;
pub use crate::tr_local_h::acff_t;
pub use crate::tr_local_h::alphaGen_t;
pub use crate::tr_local_h::backEndData_t;
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::colorGen_t;
pub use crate::tr_local_h::cullType_t;
pub use crate::tr_local_h::deformStage_t;
pub use crate::tr_local_h::deform_t;
pub use crate::tr_local_h::dlight_s;
pub use crate::tr_local_h::dlight_t;
pub use crate::tr_local_h::drawBufferCommand_t;
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::drawSurf_t;
pub use crate::tr_local_h::drawSurfsCommand_t;
pub use crate::tr_local_h::fogParms_t;
pub use crate::tr_local_h::fogPass_t;
pub use crate::tr_local_h::fog_t;
pub use crate::tr_local_h::frontEndCounters_t;
pub use crate::tr_local_h::genFunc_t;
pub use crate::tr_local_h::mnode_s;
pub use crate::tr_local_h::mnode_t;
pub use crate::tr_local_h::model_s;
pub use crate::tr_local_h::model_t;
pub use crate::tr_local_h::modtype_t;
pub use crate::tr_local_h::msurface_s;
pub use crate::tr_local_h::msurface_t;
pub use crate::tr_local_h::orientationr_t;
pub use crate::tr_local_h::renderCommandList_t;
pub use crate::tr_local_h::setColorCommand_t;
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfPoly_t;
pub use crate::tr_local_h::stretchPicCommand_t;
pub use crate::tr_local_h::surfaceType_t;
pub use crate::tr_local_h::swapBuffersCommand_t;
pub use crate::tr_local_h::texCoordGen_t;
pub use crate::tr_local_h::texModInfo_t;
pub use crate::tr_local_h::texMod_t;
pub use crate::tr_local_h::textureBundle_t;
pub use crate::tr_local_h::trGlobals_t;
pub use crate::tr_local_h::trRefEntity_t;
pub use crate::tr_local_h::trRefdef_t;
pub use crate::tr_local_h::viewParms_t;
pub use crate::tr_local_h::waveForm_t;
pub use crate::tr_local_h::world_t;
pub use crate::tr_local_h::ACFF_MODULATE_ALPHA;
pub use crate::tr_local_h::ACFF_MODULATE_RGB;
pub use crate::tr_local_h::ACFF_MODULATE_RGBA;
pub use crate::tr_local_h::ACFF_NONE;
pub use crate::tr_local_h::AGEN_CONST;
pub use crate::tr_local_h::AGEN_ENTITY;
pub use crate::tr_local_h::AGEN_IDENTITY;
pub use crate::tr_local_h::AGEN_LIGHTING_SPECULAR;
pub use crate::tr_local_h::AGEN_ONE_MINUS_ENTITY;
pub use crate::tr_local_h::AGEN_ONE_MINUS_VERTEX;
pub use crate::tr_local_h::AGEN_PORTAL;
pub use crate::tr_local_h::AGEN_SKIP;
pub use crate::tr_local_h::AGEN_VERTEX;
pub use crate::tr_local_h::AGEN_WAVEFORM;
pub use crate::tr_local_h::CGEN_BAD;
pub use crate::tr_local_h::CGEN_CONST;
pub use crate::tr_local_h::CGEN_ENTITY;
pub use crate::tr_local_h::CGEN_EXACT_VERTEX;
pub use crate::tr_local_h::CGEN_FOG;
pub use crate::tr_local_h::CGEN_IDENTITY;
pub use crate::tr_local_h::CGEN_IDENTITY_LIGHTING;
pub use crate::tr_local_h::CGEN_LIGHTING_DIFFUSE;
pub use crate::tr_local_h::CGEN_ONE_MINUS_ENTITY;
pub use crate::tr_local_h::CGEN_ONE_MINUS_VERTEX;
pub use crate::tr_local_h::CGEN_VERTEX;
pub use crate::tr_local_h::CGEN_WAVEFORM;
pub use crate::tr_local_h::CT_BACK_SIDED;
pub use crate::tr_local_h::CT_FRONT_SIDED;
pub use crate::tr_local_h::CT_TWO_SIDED;
pub use crate::tr_local_h::DEFORM_AUTOSPRITE;
pub use crate::tr_local_h::DEFORM_AUTOSPRITE2;
pub use crate::tr_local_h::DEFORM_BULGE;
pub use crate::tr_local_h::DEFORM_MOVE;
pub use crate::tr_local_h::DEFORM_NONE;
pub use crate::tr_local_h::DEFORM_NORMALS;
pub use crate::tr_local_h::DEFORM_PROJECTION_SHADOW;
pub use crate::tr_local_h::DEFORM_TEXT0;
pub use crate::tr_local_h::DEFORM_TEXT1;
pub use crate::tr_local_h::DEFORM_TEXT2;
pub use crate::tr_local_h::DEFORM_TEXT3;
pub use crate::tr_local_h::DEFORM_TEXT4;
pub use crate::tr_local_h::DEFORM_TEXT5;
pub use crate::tr_local_h::DEFORM_TEXT6;
pub use crate::tr_local_h::DEFORM_TEXT7;
pub use crate::tr_local_h::DEFORM_WAVE;
pub use crate::tr_local_h::FP_EQUAL;
pub use crate::tr_local_h::FP_LE;
pub use crate::tr_local_h::FP_NONE;
pub use crate::tr_local_h::GF_INVERSE_SAWTOOTH;
pub use crate::tr_local_h::GF_NOISE;
pub use crate::tr_local_h::GF_NONE;
pub use crate::tr_local_h::GF_SAWTOOTH;
pub use crate::tr_local_h::GF_SIN;
pub use crate::tr_local_h::GF_SQUARE;
pub use crate::tr_local_h::GF_TRIANGLE;
pub use crate::tr_local_h::MOD_BAD;
pub use crate::tr_local_h::MOD_BRUSH;
pub use crate::tr_local_h::MOD_IQM;
pub use crate::tr_local_h::MOD_MDR;
pub use crate::tr_local_h::MOD_MESH;
pub use crate::tr_local_h::RC_CLEARDEPTH;
pub use crate::tr_local_h::RC_COLORMASK;
pub use crate::tr_local_h::RC_DRAW_BUFFER;
pub use crate::tr_local_h::RC_DRAW_SURFS;
pub use crate::tr_local_h::RC_END_OF_LIST;
pub use crate::tr_local_h::RC_SCREENSHOT;
pub use crate::tr_local_h::RC_SET_COLOR;
pub use crate::tr_local_h::RC_STRETCH_PIC;
pub use crate::tr_local_h::RC_SWAP_BUFFERS;
pub use crate::tr_local_h::RC_VIDEOFRAME;
pub use crate::tr_local_h::SF_BAD;
pub use crate::tr_local_h::SF_ENTITY;
pub use crate::tr_local_h::SF_FACE;
pub use crate::tr_local_h::SF_FLARE;
pub use crate::tr_local_h::SF_GRID;
pub use crate::tr_local_h::SF_IQM;
pub use crate::tr_local_h::SF_MAX;
pub use crate::tr_local_h::SF_MD3;
pub use crate::tr_local_h::SF_MDR;
pub use crate::tr_local_h::SF_NUM_SURFACE_TYPES;
pub use crate::tr_local_h::SF_POLY;
pub use crate::tr_local_h::SF_SKIP;
pub use crate::tr_local_h::SF_TRIANGLES;
pub use crate::tr_local_h::SS_ALMOST_NEAREST;
pub use crate::tr_local_h::SS_BAD;
pub use crate::tr_local_h::SS_BANNER;
pub use crate::tr_local_h::SS_BLEND0;
pub use crate::tr_local_h::SS_BLEND1;
pub use crate::tr_local_h::SS_BLEND2;
pub use crate::tr_local_h::SS_BLEND3;
pub use crate::tr_local_h::SS_BLEND6;
pub use crate::tr_local_h::SS_DECAL;
pub use crate::tr_local_h::SS_ENVIRONMENT;
pub use crate::tr_local_h::SS_FOG;
pub use crate::tr_local_h::SS_NEAREST;
pub use crate::tr_local_h::SS_OPAQUE;
pub use crate::tr_local_h::SS_PORTAL;
pub use crate::tr_local_h::SS_SEE_THROUGH;
pub use crate::tr_local_h::SS_STENCIL_SHADOW;
pub use crate::tr_local_h::SS_UNDERWATER;
pub use crate::tr_local_h::TCGEN_BAD;
pub use crate::tr_local_h::TCGEN_ENVIRONMENT_MAPPED;
pub use crate::tr_local_h::TCGEN_FOG;
pub use crate::tr_local_h::TCGEN_IDENTITY;
pub use crate::tr_local_h::TCGEN_LIGHTMAP;
pub use crate::tr_local_h::TCGEN_TEXTURE;
pub use crate::tr_local_h::TCGEN_VECTOR;
pub use crate::tr_local_h::TMOD_ENTITY_TRANSLATE;
pub use crate::tr_local_h::TMOD_NONE;
pub use crate::tr_local_h::TMOD_ROTATE;
pub use crate::tr_local_h::TMOD_SCALE;
pub use crate::tr_local_h::TMOD_SCROLL;
pub use crate::tr_local_h::TMOD_STRETCH;
pub use crate::tr_local_h::TMOD_TRANSFORM;
pub use crate::tr_local_h::TMOD_TURBULENT;

pub use crate::src::renderergl1::tr_shader::ctype_h::tolower;

pub use crate::stdlib::__ctype_tolower_loc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct collapse_t {
    pub blendA: i32,
    pub blendB: i32,
    pub multitextureEnv: i32,
    pub multitextureBlend: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct infoParm_t {
    pub name: *mut i8,
    pub clearSolid: i32,
    pub surfaceFlags: i32,
    pub contents: i32,
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
// tr_shader.c -- this file deals with the parsing and definition of shaders

static mut s_shaderText: *mut i8 = 0 as *mut i8;
// the shader is parsed into these global variables, then copied into
// dynamically allocated memory if it is valid.

static mut stages: [crate::tr_local_h::shaderStage_t; 8] = [crate::tr_local_h::shaderStage_t {
    active: crate::src::qcommon::q_shared::qfalse,
    bundle: [crate::tr_local_h::textureBundle_t {
        image: [0 as *mut crate::tr_common_h::image_t; 8],
        numImageAnimations: 0,
        imageAnimationSpeed: 0.,
        tcGen: crate::tr_local_h::TCGEN_BAD,
        tcGenVectors: [[0.; 3]; 2],
        numTexMods: 0,
        texMods: 0 as *mut crate::tr_local_h::texModInfo_t,
        videoMapHandle: 0,
        isLightmap: crate::src::qcommon::q_shared::qfalse,
        isVideoMap: crate::src::qcommon::q_shared::qfalse,
    }; 2],
    rgbWave: crate::tr_local_h::waveForm_t {
        func: crate::tr_local_h::GF_NONE,
        base: 0.,
        amplitude: 0.,
        phase: 0.,
        frequency: 0.,
    },
    rgbGen: crate::tr_local_h::CGEN_BAD,
    alphaWave: crate::tr_local_h::waveForm_t {
        func: crate::tr_local_h::GF_NONE,
        base: 0.,
        amplitude: 0.,
        phase: 0.,
        frequency: 0.,
    },
    alphaGen: crate::tr_local_h::AGEN_IDENTITY,
    constantColor: [0; 4],
    stateBits: 0,
    adjustColorsForFog: crate::tr_local_h::ACFF_NONE,
    isDetail: crate::src::qcommon::q_shared::qfalse,
}; 8];

static mut shader: crate::tr_local_h::shader_t = crate::tr_local_h::shader_t {
    name: [0; 64],
    lightmapIndex: 0,
    index: 0,
    sortedIndex: 0,
    sort: 0.,
    defaultShader: crate::src::qcommon::q_shared::qfalse,
    explicitlyDefined: crate::src::qcommon::q_shared::qfalse,
    surfaceFlags: 0,
    contentFlags: 0,
    entityMergable: crate::src::qcommon::q_shared::qfalse,
    isSky: crate::src::qcommon::q_shared::qfalse,
    sky: crate::tr_local_h::skyParms_t {
        cloudHeight: 0.,
        outerbox: [0 as *mut crate::tr_common_h::image_t; 6],
        innerbox: [0 as *mut crate::tr_common_h::image_t; 6],
    },
    fogParms: crate::tr_local_h::fogParms_t {
        color: [0.; 3],
        depthForOpaque: 0.,
    },
    portalRange: 0.,
    multitextureEnv: 0,
    cullType: crate::tr_local_h::CT_FRONT_SIDED,
    polygonOffset: crate::src::qcommon::q_shared::qfalse,
    noMipMaps: crate::src::qcommon::q_shared::qfalse,
    noPicMip: crate::src::qcommon::q_shared::qfalse,
    fogPass: crate::tr_local_h::FP_NONE,
    needsNormal: crate::src::qcommon::q_shared::qfalse,
    needsST1: crate::src::qcommon::q_shared::qfalse,
    needsST2: crate::src::qcommon::q_shared::qfalse,
    needsColor: crate::src::qcommon::q_shared::qfalse,
    numDeforms: 0,
    deforms: [crate::tr_local_h::deformStage_t {
        deformation: crate::tr_local_h::DEFORM_NONE,
        moveVector: [0.; 3],
        deformationWave: crate::tr_local_h::waveForm_t {
            func: crate::tr_local_h::GF_NONE,
            base: 0.,
            amplitude: 0.,
            phase: 0.,
            frequency: 0.,
        },
        deformationSpread: 0.,
        bulgeWidth: 0.,
        bulgeHeight: 0.,
        bulgeSpeed: 0.,
    }; 3],
    numUnfoggedPasses: 0,
    stages: [0 as *mut crate::tr_local_h::shaderStage_t; 8],
    optimalStageIteratorFunc: None,
    clampTime: 0.,
    timeOffset: 0.,
    remappedShader: 0 as *mut crate::tr_local_h::shader_s,
    next: 0 as *mut crate::tr_local_h::shader_s,
};

static mut texMods: [[crate::tr_local_h::texModInfo_t; 4]; 8] = [[crate::tr_local_h::texModInfo_t {
    type_0: crate::tr_local_h::TMOD_NONE,
    wave: crate::tr_local_h::waveForm_t {
        func: crate::tr_local_h::GF_NONE,
        base: 0.,
        amplitude: 0.,
        phase: 0.,
        frequency: 0.,
    },
    matrix: [[0.; 2]; 2],
    translate: [0.; 2],
    scale: [0.; 2],
    scroll: [0.; 2],
    rotateSpeed: 0.,
}; 4]; 8];

static mut hashTable: [*mut crate::tr_local_h::shader_t; 1024] =
    [0 as *mut crate::tr_local_h::shader_t; 1024];

static mut shaderTextHashTable: [*mut *mut i8; 2048] = [0 as *mut *mut i8; 2048];
/*
================
return a hash value for the filename
================
*/

unsafe extern "C" fn generateHashValue(mut fname: *const i8, size: i32) -> isize {
    let mut i: i32 = 0; // don't include extension
    let mut hash: isize = 0; // damn path names
    let mut letter: i8 = 0; // damn path names
    hash = 0;
    i = 0;
    while *fname.offset(i as isize) as i32 != '\u{0}' as i32 {
        letter = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *fname.offset(i as isize) as i32;
                    __res = if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*fname.offset(i as isize) as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc())
                    .offset(*fname.offset(i as isize) as i32 as isize)
            }
            __res
        }) as i8;
        if letter as i32 == '.' as i32 {
            break;
        }
        if letter as i32 == '\\' as i32 {
            letter = '/' as i8
        }
        if letter as i32 == '/' as i32 {
            letter = '/' as i8
        }
        hash += letter as isize * (i + 119) as isize;
        i += 1
    }
    hash = hash ^ hash >> 10 ^ hash >> 20;
    hash &= (size - 1i32) as isize;
    return hash;
}
#[no_mangle]

pub unsafe extern "C" fn R_RemapShader(
    mut shaderName: *const i8,
    mut newShaderName: *const i8,
    mut timeOffset: *const i8,
) {
    let mut strippedName: [i8; 64] = [0; 64];
    let mut hash: i32 = 0;
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut sh2: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut h: crate::src::qcommon::q_shared::qhandle_t = 0;
    sh = R_FindShaderByName(shaderName);
    if sh.is_null() || sh == crate::src::renderergl1::tr_main::tr.defaultShader {
        h = RE_RegisterShaderLightMap(shaderName, 0);
        sh = R_GetShaderByHandle(h)
    }
    if sh.is_null() || sh == crate::src::renderergl1::tr_main::tr.defaultShader {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: R_RemapShader: shader %s not found\n\x00" as *const u8 as *const i8,
            shaderName,
        );
        return;
    }
    sh2 = R_FindShaderByName(newShaderName);
    if sh2.is_null() || sh2 == crate::src::renderergl1::tr_main::tr.defaultShader {
        h = RE_RegisterShaderLightMap(newShaderName, 0);
        sh2 = R_GetShaderByHandle(h)
    }
    if sh2.is_null() || sh2 == crate::src::renderergl1::tr_main::tr.defaultShader {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: R_RemapShader: new shader %s not found\n\x00" as *const u8 as *const i8,
            newShaderName,
        );
        return;
    }
    // remap all the shaders with the given name
    // even tho they might have different lightmaps
    crate::src::qcommon::q_shared::COM_StripExtension(
        shaderName,
        strippedName.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    hash = generateHashValue(strippedName.as_mut_ptr(), 1024) as i32;
    sh = hashTable[hash as usize];
    while !sh.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            (*sh).name.as_mut_ptr(),
            strippedName.as_mut_ptr(),
        ) == 0
        {
            if sh != sh2 {
                (*sh).remappedShader = sh2
            } else {
                (*sh).remappedShader = 0 as *mut crate::tr_local_h::shader_s
            }
        }
        sh = (*sh).next
    }
    if !timeOffset.is_null() {
        (*sh2).timeOffset = atof(timeOffset)
    };
}
/*
===============
ParseVector
===============
*/

unsafe extern "C" fn ParseVector(
    mut text: *mut *mut i8,
    mut count: i32,
    mut v: *mut f32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut _i: i32 = 0;
    // FIXME: spaces are currently required after parens, should change parseext...
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if crate::stdlib::strcmp(token, b"(\x00" as *const u8 as *const i8) != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing parenthesis in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }

    for i in 0..count {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );

        if *token.offset(0) == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing vector element in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }

        *v.offset(i as isize) = atof(token) as f32;
    }
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if crate::stdlib::strcmp(token, b")\x00" as *const u8 as *const i8) != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing parenthesis in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
NameToAFunc
===============
*/

unsafe extern "C" fn NameToAFunc(mut funcname: *const i8) -> u32 {
    if crate::src::qcommon::q_shared::Q_stricmp(funcname, b"GT0\x00" as *const u8 as *const i8) == 0
    {
        return 0x10000000u32;
    } else {
        if crate::src::qcommon::q_shared::Q_stricmp(
            funcname,
            b"LT128\x00" as *const u8 as *const i8,
        ) == 0
        {
            return 0x20000000u32;
        } else {
            if crate::src::qcommon::q_shared::Q_stricmp(
                funcname,
                b"GE128\x00" as *const u8 as *const i8,
            ) == 0
            {
                return 0x40000000u32;
            }
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
        b"WARNING: invalid alphaFunc name \'%s\' in shader \'%s\'\n\x00" as *const u8 as *const i8,
        funcname,
        shader.name.as_mut_ptr(),
    );
    return 0u32;
}
/*
===============
NameToSrcBlendMode
===============
*/

unsafe extern "C" fn NameToSrcBlendMode(mut name: *const i8) -> i32 {
    if crate::src::qcommon::q_shared::Q_stricmp(name, b"GL_ONE\x00" as *const u8 as *const i8) == 0
    {
        return 0x2i32;
    } else {
        if crate::src::qcommon::q_shared::Q_stricmp(name, b"GL_ZERO\x00" as *const u8 as *const i8)
            == 0
        {
            return 0x1i32;
        } else {
            if crate::src::qcommon::q_shared::Q_stricmp(
                name,
                b"GL_DST_COLOR\x00" as *const u8 as *const i8,
            ) == 0
            {
                return 0x3i32;
            } else {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    name,
                    b"GL_ONE_MINUS_DST_COLOR\x00" as *const u8 as *const i8,
                ) == 0
                {
                    return 0x4i32;
                } else {
                    if crate::src::qcommon::q_shared::Q_stricmp(
                        name,
                        b"GL_SRC_ALPHA\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        return 0x5i32;
                    } else {
                        if crate::src::qcommon::q_shared::Q_stricmp(
                            name,
                            b"GL_ONE_MINUS_SRC_ALPHA\x00" as *const u8 as *const i8,
                        ) == 0
                        {
                            return 0x6i32;
                        } else {
                            if crate::src::qcommon::q_shared::Q_stricmp(
                                name,
                                b"GL_DST_ALPHA\x00" as *const u8 as *const i8,
                            ) == 0
                            {
                                return 0x7i32;
                            } else {
                                if crate::src::qcommon::q_shared::Q_stricmp(
                                    name,
                                    b"GL_ONE_MINUS_DST_ALPHA\x00" as *const u8 as *const i8,
                                ) == 0
                                {
                                    return 0x8i32;
                                } else {
                                    if crate::src::qcommon::q_shared::Q_stricmp(
                                        name,
                                        b"GL_SRC_ALPHA_SATURATE\x00" as *const u8 as *const i8,
                                    ) == 0
                                    {
                                        return 0x9i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
        b"WARNING: unknown blend mode \'%s\' in shader \'%s\', substituting GL_ONE\n\x00"
            as *const u8 as *const i8,
        name,
        shader.name.as_mut_ptr(),
    );
    return 0x2;
}
/*
===============
NameToDstBlendMode
===============
*/

unsafe extern "C" fn NameToDstBlendMode(mut name: *const i8) -> i32 {
    if crate::src::qcommon::q_shared::Q_stricmp(name, b"GL_ONE\x00" as *const u8 as *const i8) == 0
    {
        return 0x20i32;
    } else {
        if crate::src::qcommon::q_shared::Q_stricmp(name, b"GL_ZERO\x00" as *const u8 as *const i8)
            == 0
        {
            return 0x10i32;
        } else {
            if crate::src::qcommon::q_shared::Q_stricmp(
                name,
                b"GL_SRC_ALPHA\x00" as *const u8 as *const i8,
            ) == 0
            {
                return 0x50i32;
            } else {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    name,
                    b"GL_ONE_MINUS_SRC_ALPHA\x00" as *const u8 as *const i8,
                ) == 0
                {
                    return 0x60i32;
                } else {
                    if crate::src::qcommon::q_shared::Q_stricmp(
                        name,
                        b"GL_DST_ALPHA\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        return 0x70i32;
                    } else {
                        if crate::src::qcommon::q_shared::Q_stricmp(
                            name,
                            b"GL_ONE_MINUS_DST_ALPHA\x00" as *const u8 as *const i8,
                        ) == 0
                        {
                            return 0x80i32;
                        } else {
                            if crate::src::qcommon::q_shared::Q_stricmp(
                                name,
                                b"GL_SRC_COLOR\x00" as *const u8 as *const i8,
                            ) == 0
                            {
                                return 0x30i32;
                            } else {
                                if crate::src::qcommon::q_shared::Q_stricmp(
                                    name,
                                    b"GL_ONE_MINUS_SRC_COLOR\x00" as *const u8 as *const i8,
                                ) == 0
                                {
                                    return 0x40i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
        b"WARNING: unknown blend mode \'%s\' in shader \'%s\', substituting GL_ONE\n\x00"
            as *const u8 as *const i8,
        name,
        shader.name.as_mut_ptr(),
    );
    return 0x20;
}
/*
===============
NameToGenFunc
===============
*/

unsafe extern "C" fn NameToGenFunc(mut funcname: *const i8) -> crate::tr_local_h::genFunc_t {
    if crate::src::qcommon::q_shared::Q_stricmp(funcname, b"sin\x00" as *const u8 as *const i8) == 0
    {
        return crate::tr_local_h::GF_SIN;
    } else {
        if crate::src::qcommon::q_shared::Q_stricmp(
            funcname,
            b"square\x00" as *const u8 as *const i8,
        ) == 0
        {
            return crate::tr_local_h::GF_SQUARE;
        } else {
            if crate::src::qcommon::q_shared::Q_stricmp(
                funcname,
                b"triangle\x00" as *const u8 as *const i8,
            ) == 0
            {
                return crate::tr_local_h::GF_TRIANGLE;
            } else {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    funcname,
                    b"sawtooth\x00" as *const u8 as *const i8,
                ) == 0
                {
                    return crate::tr_local_h::GF_SAWTOOTH;
                } else {
                    if crate::src::qcommon::q_shared::Q_stricmp(
                        funcname,
                        b"inversesawtooth\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        return crate::tr_local_h::GF_INVERSE_SAWTOOTH;
                    } else {
                        if crate::src::qcommon::q_shared::Q_stricmp(
                            funcname,
                            b"noise\x00" as *const u8 as *const i8,
                        ) == 0
                        {
                            return crate::tr_local_h::GF_NOISE;
                        }
                    }
                }
            }
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
        b"WARNING: invalid genfunc name \'%s\' in shader \'%s\'\n\x00" as *const u8 as *const i8,
        funcname,
        shader.name.as_mut_ptr(),
    );
    return crate::tr_local_h::GF_SIN;
}
/*
===================
ParseWaveForm
===================
*/

unsafe extern "C" fn ParseWaveForm(
    mut text: *mut *mut i8,
    mut wave: *mut crate::tr_local_h::waveForm_t,
) {
    let mut token: *mut i8 = 0 as *mut i8;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing waveform parm in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    (*wave).func = NameToGenFunc(token);
    // BASE, AMP, PHASE, FREQ
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing waveform parm in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    (*wave).base = atof(token) as f32;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing waveform parm in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    (*wave).amplitude = atof(token) as f32;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing waveform parm in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    (*wave).phase = atof(token) as f32;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing waveform parm in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    (*wave).frequency = atof(token) as f32;
}
/*
===================
ParseTexMod
===================
*/

unsafe extern "C" fn ParseTexMod(
    mut _text: *mut i8,
    mut stage: *mut crate::tr_local_h::shaderStage_t,
) {
    let mut token: *const i8 = 0 as *const i8;
    let mut text: *mut *mut i8 = &mut _text;
    let mut tmi: *mut crate::tr_local_h::texModInfo_t = 0 as *mut crate::tr_local_h::texModInfo_t;
    if (*stage).bundle[0].numTexMods == 4 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ERROR: too many tcMod stages in shader \'%s\'\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    tmi = &mut *(*(*stage).bundle.as_mut_ptr().offset(0))
        .texMods
        .offset((*(*stage).bundle.as_mut_ptr().offset(0)).numTexMods as isize)
        as *mut crate::tr_local_h::texModInfo_t;
    (*stage).bundle[0].numTexMods += 1;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    //
    // turb
    //
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"turb\x00" as *const u8 as *const i8) == 0 {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing tcMod turb parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.base = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing tcMod turb in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.amplitude = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing tcMod turb in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.phase = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing tcMod turb in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.frequency = atof(token) as f32;
        (*tmi).type_0 = crate::tr_local_h::TMOD_TURBULENT
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"scale\x00" as *const u8 as *const i8,
    ) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing scale parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).scale[0] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing scale parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).scale[1] = atof(token) as f32;
        (*tmi).type_0 = crate::tr_local_h::TMOD_SCALE
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"scroll\x00" as *const u8 as *const i8,
    ) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing scale scroll parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).scroll[0] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing scale scroll parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).scroll[1] = atof(token) as f32;
        (*tmi).type_0 = crate::tr_local_h::TMOD_SCROLL
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"stretch\x00" as *const u8 as *const i8,
    ) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing stretch parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.func = NameToGenFunc(token);
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing stretch parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.base = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing stretch parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.amplitude = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing stretch parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.phase = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing stretch parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).wave.frequency = atof(token) as f32;
        (*tmi).type_0 = crate::tr_local_h::TMOD_STRETCH
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"transform\x00" as *const u8 as *const i8,
    ) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing transform parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).matrix[0][0] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing transform parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).matrix[0][1] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing transform parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).matrix[1][0] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing transform parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).matrix[1][1] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing transform parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).translate[0] = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing transform parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).translate[1] = atof(token) as f32;
        (*tmi).type_0 = crate::tr_local_h::TMOD_TRANSFORM
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"rotate\x00" as *const u8 as *const i8,
    ) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing tcMod rotate parms in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*tmi).rotateSpeed = atof(token) as f32;
        (*tmi).type_0 = crate::tr_local_h::TMOD_ROTATE
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"entityTranslate\x00" as *const u8 as *const i8,
    ) == 0
    {
        (*tmi).type_0 = crate::tr_local_h::TMOD_ENTITY_TRANSLATE
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: unknown tcMod \'%s\' in shader \'%s\'\n\x00" as *const u8 as *const i8,
            token,
            shader.name.as_mut_ptr(),
        );
    };
}
//
// scale
//
//
// scroll
//
//
// stretch
//
//
// transform
//
//
// rotate
//
//
// entityTranslate
//
/*
===================
ParseStage
===================
*/

unsafe extern "C" fn ParseStage(
    mut stage: *mut crate::tr_local_h::shaderStage_t,
    mut text: *mut *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut depthMaskBits: i32 = 0x100;
    let mut blendSrcBits: i32 = 0;
    let mut blendDstBits: i32 = 0;
    let mut atestBits: i32 = 0;
    let mut depthFuncBits: i32 = 0;
    let mut depthMaskExplicit: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    (*stage).active = crate::src::qcommon::q_shared::qtrue;
    loop {
        token =
            crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qtrue);
        if *token.offset(0) == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: no matching \'}\' found\n\x00" as *const u8 as *const i8,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        if *token.offset(0) as i32 == '}' as i32 {
            break;
        }
        //
        // map <name>
        //
        if crate::src::qcommon::q_shared::Q_stricmp(token, b"map\x00" as *const u8 as *const i8)
            == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameter for \'map\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"$whiteimage\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).bundle[0].image[0] = crate::src::renderergl1::tr_main::tr.whiteImage
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"$lightmap\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).bundle[0].isLightmap = crate::src::qcommon::q_shared::qtrue;
                if shader.lightmapIndex < 0
                    || crate::src::renderergl1::tr_main::tr.lightmaps.is_null()
                {
                    (*stage).bundle[0].image[0] = crate::src::renderergl1::tr_main::tr.whiteImage
                } else {
                    (*stage).bundle[0].image[0] = *crate::src::renderergl1::tr_main::tr
                        .lightmaps
                        .offset(shader.lightmapIndex as isize)
                }
            } else {
                let mut type_0: crate::tr_common_h::imgType_t =
                    crate::tr_common_h::IMGTYPE_COLORALPHA;
                let mut flags: crate::tr_common_h::imgFlags_t = crate::tr_common_h::IMGFLAG_NONE;
                if shader.noMipMaps as u64 == 0 {
                    flags = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
                        flags | crate::tr_common_h::IMGFLAG_MIPMAP,
                    )
                }
                if shader.noPicMip as u64 == 0 {
                    flags = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
                        flags | crate::tr_common_h::IMGFLAG_PICMIP,
                    )
                }
                (*stage).bundle[0].image[0] =
                    crate::src::renderergl1::tr_image::R_FindImageFile(token, type_0, flags);
                if (*stage).bundle[0].image[0].is_null() {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                        b"WARNING: R_FindImageFile could not find \'%s\' in shader \'%s\'\n\x00"
                            as *const u8 as *const i8,
                        token,
                        shader.name.as_mut_ptr(),
                    );
                    return crate::src::qcommon::q_shared::qfalse;
                }
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"clampmap\x00" as *const u8 as *const i8,
        ) == 0
        {
            let mut type_1: crate::tr_common_h::imgType_t = crate::tr_common_h::IMGTYPE_COLORALPHA;
            let mut flags_0: crate::tr_common_h::imgFlags_t =
                crate::tr_common_h::IMGFLAG_CLAMPTOEDGE;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameter for \'clampmap\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if shader.noMipMaps as u64 == 0 {
                flags_0 = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
                    flags_0 | crate::tr_common_h::IMGFLAG_MIPMAP,
                )
            }
            if shader.noPicMip as u64 == 0 {
                flags_0 = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
                    flags_0 | crate::tr_common_h::IMGFLAG_PICMIP,
                )
            }
            (*stage).bundle[0].image[0] =
                crate::src::renderergl1::tr_image::R_FindImageFile(token, type_1, flags_0);
            if (*stage).bundle[0].image[0].is_null() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: R_FindImageFile could not find \'%s\' in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    token,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"animMap\x00" as *const u8 as *const i8,
        ) == 0
        {
            let mut totalImages: i32 = 0;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameter for \'animMap\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            (*stage).bundle[0].imageAnimationSpeed = atof(token) as f32;
            loop
            //
            // clampmap <name>
            //
            //
            // animMap <frequency> <image1> .... <imageN>
            //
            // parse up to MAX_IMAGE_ANIMATIONS animations
            {
                let mut num: i32 = 0;
                token = crate::src::qcommon::q_shared::COM_ParseExt(
                    text,
                    crate::src::qcommon::q_shared::qfalse,
                );
                if *token.offset(0) == 0 {
                    break;
                }
                num = (*stage).bundle[0].numImageAnimations;
                if num < 8 {
                    let mut flags_1: crate::tr_common_h::imgFlags_t =
                        crate::tr_common_h::IMGFLAG_NONE;
                    if shader.noMipMaps as u64 == 0 {
                        flags_1 = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
                            flags_1 | crate::tr_common_h::IMGFLAG_MIPMAP,
                        )
                    }
                    if shader.noPicMip as u64 == 0 {
                        flags_1 = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
                            flags_1 | crate::tr_common_h::IMGFLAG_PICMIP,
                        )
                    }
                    (*stage).bundle[0].image[num as usize] =
                        crate::src::renderergl1::tr_image::R_FindImageFile(
                            token,
                            crate::tr_common_h::IMGTYPE_COLORALPHA,
                            flags_1,
                        );
                    if (*stage).bundle[0].image[num as usize].is_null() {
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                            b"WARNING: R_FindImageFile could not find \'%s\' in shader \'%s\'\n\x00"
                                as *const u8 as *const i8,
                            token,
                            shader.name.as_mut_ptr(),
                        );
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                    (*stage).bundle[0].numImageAnimations += 1
                }
                totalImages += 1
            }
            if totalImages > 8 {
                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING as
                                                                  i32,
                                                              b"WARNING: ignoring excess images for \'animMap\' (found %d, max is %d) in shader \'%s\'\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              totalImages,
                                                              8i32,
                                                              shader.name.as_mut_ptr());
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"videoMap\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameter for \'videoMap\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            (*stage).bundle[0].videoMapHandle = crate::src::renderergl1::tr_main::ri
                .CIN_PlayCinematic
                .expect("non-null function pointer")(
                token, 0, 0, 256, 256, 2 | 8 | 16
            );
            if (*stage).bundle[0].videoMapHandle != -(1) {
                (*stage).bundle[0].isVideoMap = crate::src::qcommon::q_shared::qtrue;
                (*stage).bundle[0].image[0] = crate::src::renderergl1::tr_main::tr.scratchImage
                    [(*stage).bundle[0].videoMapHandle as usize]
            } else {
                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING as
                                                                  i32,
                                                              b"WARNING: could not load \'%s\' for \'videoMap\' keyword in shader \'%s\'\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              token,
                                                              shader.name.as_mut_ptr());
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"alphaFunc\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameter for \'alphaFunc\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            atestBits = NameToAFunc(token) as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"depthfunc\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameter for \'depthfunc\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"lequal\x00" as *const u8 as *const i8,
            ) == 0
            {
                depthFuncBits = 0
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"equal\x00" as *const u8 as *const i8,
            ) == 0
            {
                depthFuncBits = 0x20000
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: unknown depthfunc \'%s\' in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    token,
                    shader.name.as_mut_ptr(),
                );
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"detail\x00" as *const u8 as *const i8,
        ) == 0
        {
            (*stage).isDetail = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"blendfunc\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) as i32 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parm for blendFunc in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                );
            } else {
                //
                // alphafunc <func>
                //
                //
                // depthFunc <func>
                //
                //
                // detail
                //
                //
                // blendfunc <srcFactor> <dstFactor>
                // or blendfunc <add|filter|blend>
                //
                // check for "simple" blends first
                if crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"add\x00" as *const u8 as *const i8,
                ) == 0
                {
                    blendSrcBits = 0x2;
                    blendDstBits = 0x20
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"filter\x00" as *const u8 as *const i8,
                ) == 0
                {
                    blendSrcBits = 0x3;
                    blendDstBits = 0x10
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"blend\x00" as *const u8 as *const i8,
                ) == 0
                {
                    blendSrcBits = 0x5;
                    blendDstBits = 0x60
                } else {
                    // complex double blends
                    blendSrcBits = NameToSrcBlendMode(token);
                    token = crate::src::qcommon::q_shared::COM_ParseExt(
                        text,
                        crate::src::qcommon::q_shared::qfalse,
                    );
                    if *token.offset(0) as i32 == 0 {
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                            b"WARNING: missing parm for blendFunc in shader \'%s\'\n\x00"
                                as *const u8 as *const i8,
                            shader.name.as_mut_ptr(),
                        );
                        continue;
                    } else {
                        blendDstBits = NameToDstBlendMode(token)
                    }
                }
                // clear depth mask for blended surfaces
                if depthMaskExplicit as u64 == 0 {
                    depthMaskBits = 0
                }
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"rgbGen\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) as i32 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameters for rgbGen in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                );
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"wave\x00" as *const u8 as *const i8,
            ) == 0
            {
                ParseWaveForm(text, &mut (*stage).rgbWave);
                (*stage).rgbGen = crate::tr_local_h::CGEN_WAVEFORM
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"const\x00" as *const u8 as *const i8,
            ) == 0
            {
                let mut color: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                color[2] = 0f32;
                color[1] = color[2];
                color[0] = color[1];
                ParseVector(text, 3, color.as_mut_ptr());
                (*stage).constantColor[0] =
                    (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
                (*stage).constantColor[1] =
                    (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
                (*stage).constantColor[2] =
                    (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
                (*stage).rgbGen = crate::tr_local_h::CGEN_CONST
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"identity\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_IDENTITY
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"identityLighting\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"entity\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_ENTITY
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"oneMinusEntity\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_ONE_MINUS_ENTITY
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"vertex\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_VERTEX;
                if (*stage).alphaGen == 0u32 {
                    (*stage).alphaGen = crate::tr_local_h::AGEN_VERTEX
                }
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"exactVertex\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_EXACT_VERTEX
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"lightingDiffuse\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_LIGHTING_DIFFUSE
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"oneMinusVertex\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).rgbGen = crate::tr_local_h::CGEN_ONE_MINUS_VERTEX
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: unknown rgbGen parameter \'%s\' in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    token,
                    shader.name.as_mut_ptr(),
                );
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"alphaGen\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) as i32 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parameters for alphaGen in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                );
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"wave\x00" as *const u8 as *const i8,
            ) == 0
            {
                ParseWaveForm(text, &mut (*stage).alphaWave);
                (*stage).alphaGen = crate::tr_local_h::AGEN_WAVEFORM
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"const\x00" as *const u8 as *const i8,
            ) == 0
            {
                token = crate::src::qcommon::q_shared::COM_ParseExt(
                    text,
                    crate::src::qcommon::q_shared::qfalse,
                );
                (*stage).constantColor[3] =
                    (255f64 * atof(token)) as crate::src::qcommon::q_shared::byte;
                (*stage).alphaGen = crate::tr_local_h::AGEN_CONST
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"identity\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_IDENTITY
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"entity\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_ENTITY
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"oneMinusEntity\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_ONE_MINUS_ENTITY
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"vertex\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_VERTEX
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"lightingSpecular\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_LIGHTING_SPECULAR
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"oneMinusVertex\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_ONE_MINUS_VERTEX
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"portal\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).alphaGen = crate::tr_local_h::AGEN_PORTAL;
                token = crate::src::qcommon::q_shared::COM_ParseExt(
                    text,
                    crate::src::qcommon::q_shared::qfalse,
                );
                if *token.offset(0) as i32 == 0 {
                    shader.portalRange = 256f32;
                    crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING
                                                                      as
                                                                      i32,
                                                                  b"WARNING: missing range parameter for alphaGen portal in shader \'%s\', defaulting to 256\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const i8,
                                                                  shader.name.as_mut_ptr());
                } else {
                    shader.portalRange = atof(token) as f32
                }
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: unknown alphaGen parameter \'%s\' in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    token,
                    shader.name.as_mut_ptr(),
                );
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"texgen\x00" as *const u8 as *const i8,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"tcGen\x00" as *const u8 as *const i8,
            ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) as i32 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing texgen parm in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                );
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"environment\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).bundle[0].tcGen = crate::tr_local_h::TCGEN_ENVIRONMENT_MAPPED
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"lightmap\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*stage).bundle[0].tcGen = crate::tr_local_h::TCGEN_LIGHTMAP
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"texture\x00" as *const u8 as *const i8,
            ) == 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"base\x00" as *const u8 as *const i8,
                ) == 0
            {
                (*stage).bundle[0].tcGen = crate::tr_local_h::TCGEN_TEXTURE
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"vector\x00" as *const u8 as *const i8,
            ) == 0
            {
                ParseVector(text, 3, (*stage).bundle[0].tcGenVectors[0].as_mut_ptr());
                ParseVector(text, 3, (*stage).bundle[0].tcGenVectors[1].as_mut_ptr());
                (*stage).bundle[0].tcGen = crate::tr_local_h::TCGEN_VECTOR
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: unknown texgen parm in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                );
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"tcMod\x00" as *const u8 as *const i8,
        ) == 0
        {
            let mut buffer: [i8; 1024] =
                *::std::mem::transmute::<&[u8; 1024],
                                         &mut [i8; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
            loop {
                token = crate::src::qcommon::q_shared::COM_ParseExt(
                    text,
                    crate::src::qcommon::q_shared::qfalse,
                );
                if *token.offset(0) as i32 == 0 {
                    break;
                }
                crate::src::qcommon::q_shared::Q_strcat(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 1024]>() as i32,
                    token,
                );
                crate::src::qcommon::q_shared::Q_strcat(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 1024]>() as i32,
                    b" \x00" as *const u8 as *const i8,
                );
            }
            ParseTexMod(buffer.as_mut_ptr(), stage);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"depthwrite\x00" as *const u8 as *const i8,
        ) == 0
        {
            depthMaskBits = 0x100;
            depthMaskExplicit = crate::src::qcommon::q_shared::qtrue
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: unknown parameter \'%s\' in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                token,
                shader.name.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    //
    // rgbGen
    //
    //
    // alphaGen
    //
    //
    // tcGen <function>
    //
    //
    // tcMod <type> <...>
    //
    //
    // depthmask
    //
    //
    // if cgen isn't explicitly specified, use either identity or identitylighting
    //
    if (*stage).rgbGen == crate::tr_local_h::CGEN_BAD {
        if blendSrcBits == 0 || blendSrcBits == 0x2 || blendSrcBits == 0x5 {
            (*stage).rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING
        } else {
            (*stage).rgbGen = crate::tr_local_h::CGEN_IDENTITY
        }
    }
    //
    // implicitly assume that a GL_ONE GL_ZERO blend mask disables blending
    //
    if blendSrcBits == 0x2 && blendDstBits == 0x10 {
        blendSrcBits = 0;
        blendDstBits = blendSrcBits;
        depthMaskBits = 0x100
    }
    // decide which agens we can skip
    if (*stage).alphaGen == crate::tr_local_h::AGEN_IDENTITY {
        if (*stage).rgbGen == crate::tr_local_h::CGEN_IDENTITY
            || (*stage).rgbGen == crate::tr_local_h::CGEN_LIGHTING_DIFFUSE
        {
            (*stage).alphaGen = crate::tr_local_h::AGEN_SKIP
        }
    }
    //
    // compute state bits
    //
    (*stage).stateBits =
        (depthMaskBits | blendSrcBits | blendDstBits | atestBits | depthFuncBits) as u32;
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
ParseDeform

deformVertexes wave <spread> <waveform> <base> <amplitude> <phase> <frequency>
deformVertexes normal <frequency> <amplitude>
deformVertexes move <vector> <waveform> <base> <amplitude> <phase> <frequency>
deformVertexes bulge <bulgeWidth> <bulgeHeight> <bulgeSpeed>
deformVertexes projectionShadow
deformVertexes autoSprite
deformVertexes autoSprite2
deformVertexes text[0-7]
===============
*/

unsafe extern "C" fn ParseDeform(mut text: *mut *mut i8) {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut ds: *mut crate::tr_local_h::deformStage_t = 0 as *mut crate::tr_local_h::deformStage_t;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing deform parm in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    if shader.numDeforms == 3 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: MAX_SHADER_DEFORMS in \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    ds = &mut *shader
        .deforms
        .as_mut_ptr()
        .offset(shader.numDeforms as isize) as *mut crate::tr_local_h::deformStage_t;
    shader.numDeforms += 1;
    if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"projectionShadow\x00" as *const u8 as *const i8,
    ) == 0
    {
        (*ds).deformation = crate::tr_local_h::DEFORM_PROJECTION_SHADOW;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"autosprite\x00" as *const u8 as *const i8)
        == 0
    {
        (*ds).deformation = crate::tr_local_h::DEFORM_AUTOSPRITE;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"autosprite2\x00" as *const u8 as *const i8)
        == 0
    {
        (*ds).deformation = crate::tr_local_h::DEFORM_AUTOSPRITE2;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmpn(token, b"text\x00" as *const u8 as *const i8, 4)
        == 0
    {
        let mut n: i32 = 0;
        n = *token.offset(4) as i32 - '0' as i32;
        if n < 0 || n > 7 {
            n = 0
        }
        (*ds).deformation =
            (crate::tr_local_h::DEFORM_TEXT0 as i32 + n) as crate::tr_local_h::deform_t;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"bulge\x00" as *const u8 as *const i8) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing deformVertexes bulge parm in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*ds).bulgeWidth = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing deformVertexes bulge parm in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*ds).bulgeHeight = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing deformVertexes bulge parm in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*ds).bulgeSpeed = atof(token) as f32;
        (*ds).deformation = crate::tr_local_h::DEFORM_BULGE;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"wave\x00" as *const u8 as *const i8) == 0 {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing deformVertexes parm in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        if atof(token) != 0f64 {
            (*ds).deformationSpread = (1f64 / atof(token)) as f32
        } else {
            (*ds).deformationSpread = 100.0;
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: illegal div value of 0 in deformVertexes command for shader \'%s\'\n\x00"
                    as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
        }
        ParseWaveForm(text, &mut (*ds).deformationWave);
        (*ds).deformation = crate::tr_local_h::DEFORM_WAVE;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"normal\x00" as *const u8 as *const i8) == 0
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing deformVertexes parm in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*ds).deformationWave.amplitude = atof(token) as f32;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            text,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) as i32 == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: missing deformVertexes parm in shader \'%s\'\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            return;
        }
        (*ds).deformationWave.frequency = atof(token) as f32;
        (*ds).deformation = crate::tr_local_h::DEFORM_NORMALS;
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"move\x00" as *const u8 as *const i8) == 0 {
        let mut _i: i32 = 0;

        for i in 0..3 {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );

            if *token.offset(0) as i32 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing deformVertexes parm in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                );
                return;
            }

            (*ds).moveVector[i as usize] = atof(token) as crate::src::qcommon::q_shared::vec_t;
        }
        ParseWaveForm(text, &mut (*ds).deformationWave);
        (*ds).deformation = crate::tr_local_h::DEFORM_MOVE;
        return;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
        b"WARNING: unknown deformVertexes subtype \'%s\' found in shader \'%s\'\n\x00" as *const u8
            as *const i8,
        token,
        shader.name.as_mut_ptr(),
    );
}
/*
===============
ParseSkyParms

skyParms <outerbox> <cloudheight> <innerbox>
===============
*/

unsafe extern "C" fn ParseSkyParms(mut text: *mut *mut i8) {
    let mut token: *mut i8 = 0 as *mut i8;
    static mut suf: [*mut i8; 6] = [
        b"rt\x00" as *const u8 as *mut i8,
        b"bk\x00" as *const u8 as *mut i8,
        b"lf\x00" as *const u8 as *mut i8,
        b"ft\x00" as *const u8 as *mut i8,
        b"up\x00" as *const u8 as *mut i8,
        b"dn\x00" as *const u8 as *mut i8,
    ];
    let mut pathname: [i8; 64] = [0; 64];
    let mut i: i32 = 0;
    let mut imgFlags: crate::tr_common_h::imgFlags_t = (crate::tr_common_h::IMGFLAG_MIPMAP as i32
        | crate::tr_common_h::IMGFLAG_PICMIP as i32)
        as crate::tr_common_h::imgFlags_t;
    // outerbox
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: \'skyParms\' missing parameter in shader \'%s\'\n\x00" as *const u8
                as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    if crate::stdlib::strcmp(token, b"-\x00" as *const u8 as *const i8) != 0 {
        i = 0;
        while i < 6 {
            crate::src::qcommon::q_shared::Com_sprintf(
                pathname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
                b"%s_%s.tga\x00" as *const u8 as *const i8,
                token,
                suf[i as usize],
            );
            shader.sky.outerbox[i as usize] = crate::src::renderergl1::tr_image::R_FindImageFile(
                pathname.as_mut_ptr(),
                crate::tr_common_h::IMGTYPE_COLORALPHA,
                imgFlags | crate::tr_common_h::IMGFLAG_CLAMPTOEDGE,
            );
            if shader.sky.outerbox[i as usize].is_null() {
                shader.sky.outerbox[i as usize] = crate::src::renderergl1::tr_main::tr.defaultImage
            }
            i += 1
        }
    }
    // cloudheight
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: \'skyParms\' missing parameter in shader \'%s\'\n\x00" as *const u8
                as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    shader.sky.cloudHeight = atof(token) as f32;
    if shader.sky.cloudHeight == 0. {
        shader.sky.cloudHeight = 512f32
    }
    crate::src::renderergl1::tr_sky::R_InitSkyTexCoords(shader.sky.cloudHeight);
    // innerbox
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: \'skyParms\' missing parameter in shader \'%s\'\n\x00" as *const u8
                as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    if crate::stdlib::strcmp(token, b"-\x00" as *const u8 as *const i8) != 0 {
        i = 0;
        while i < 6 {
            crate::src::qcommon::q_shared::Com_sprintf(
                pathname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
                b"%s_%s.tga\x00" as *const u8 as *const i8,
                token,
                suf[i as usize],
            );
            shader.sky.innerbox[i as usize] = crate::src::renderergl1::tr_image::R_FindImageFile(
                pathname.as_mut_ptr(),
                crate::tr_common_h::IMGTYPE_COLORALPHA,
                imgFlags,
            );
            if shader.sky.innerbox[i as usize].is_null() {
                shader.sky.innerbox[i as usize] = crate::src::renderergl1::tr_main::tr.defaultImage
            }
            i += 1
        }
    }
    shader.isSky = crate::src::qcommon::q_shared::qtrue;
}
/*
=================
ParseSort
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ParseSort(mut text: *mut *mut i8) {
    let mut token: *mut i8 = 0 as *mut i8;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    if *token.offset(0) as i32 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: missing sort parameter in shader \'%s\'\n\x00" as *const u8 as *const i8,
            shader.name.as_mut_ptr(),
        );
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(token, b"portal\x00" as *const u8 as *const i8) == 0
    {
        shader.sort = crate::tr_local_h::SS_PORTAL as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(token, b"sky\x00" as *const u8 as *const i8)
        == 0
    {
        shader.sort = crate::tr_local_h::SS_ENVIRONMENT as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"opaque\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_OPAQUE as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"decal\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_DECAL as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"seeThrough\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_SEE_THROUGH as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"banner\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_BANNER as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"additive\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_BLEND1 as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"nearest\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_NEAREST as i32 as f32
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        token,
        b"underwater\x00" as *const u8 as *const i8,
    ) == 0
    {
        shader.sort = crate::tr_local_h::SS_UNDERWATER as i32 as f32
    } else {
        shader.sort = atof(token) as f32
    };
}
#[no_mangle]

pub static mut infoParms: [infoParm_t; 32] = [
    {
        let mut init = infoParm_t {
            name: b"water\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 32,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"slime\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 16,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"lava\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 8,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"playerclip\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x10000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"monsterclip\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x20000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nodrop\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x80000000u32 as i32,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nonsolid\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0x4000,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"origin\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x1000000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"trans\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0,
            contents: 0x20000000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"detail\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0,
            contents: 0x8000000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"structural\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0,
            contents: 0x10000000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"areaportal\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x8000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"clusterportal\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x100000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"donotenter\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 0x200000,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"fog\x00" as *const u8 as *mut i8,
            clearSolid: 1,
            surfaceFlags: 0,
            contents: 64,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"sky\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x4,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"lightfilter\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x8000,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"alphashadow\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x10000,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"hint\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x100,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"slick\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x2,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"noimpact\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x10,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nomarks\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x20,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"ladder\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x8,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nodamage\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x1,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"metalsteps\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x1000,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"flesh\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x40,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nosteps\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x2000,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nodraw\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x80,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"pointlight\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x800,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nolightmap\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x400,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"nodlight\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x20000,
            contents: 0,
        };
        init
    },
    {
        let mut init = infoParm_t {
            name: b"dust\x00" as *const u8 as *mut i8,
            clearSolid: 0,
            surfaceFlags: 0x40000,
            contents: 0,
        };
        init
    },
];
/*
===============
ParseSurfaceParm

surfaceparm <name>
===============
*/

unsafe extern "C" fn ParseSurfaceParm(mut text: *mut *mut i8) {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut numInfoParms: i32 = (::std::mem::size_of::<[infoParm_t; 32]>())
        .wrapping_div(::std::mem::size_of::<infoParm_t>()) as i32;
    let mut i: i32 = 0;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qfalse);
    i = 0;
    while i < numInfoParms {
        if crate::src::qcommon::q_shared::Q_stricmp(token, infoParms[i as usize].name) == 0 {
            shader.surfaceFlags |= infoParms[i as usize].surfaceFlags;
            shader.contentFlags |= infoParms[i as usize].contents;
            break;
        } else {
            i += 1
        }
    }
}
/*
=================
ParseShader

The current text pointer is at the explicit text definition of the
shader.  Parse it into the global shader variable.  Later functions
will optimize it.
=================
*/

unsafe extern "C" fn ParseShader(
    mut text: *mut *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut s: i32 = 0;
    s = 0;
    token = crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qtrue);
    if *token.offset(0) as i32 != '{' as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: expecting \'{\', found \'%s\' instead in shader \'%s\'\n\x00" as *const u8
                as *const i8,
            token,
            shader.name.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    loop {
        token =
            crate::src::qcommon::q_shared::COM_ParseExt(text, crate::src::qcommon::q_shared::qtrue);
        if *token.offset(0) == 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: no concluding \'}\' in shader %s\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        // end of shader definition
        if *token.offset(0) as i32 == '}' as i32 {
            break;
        }
        // stage definition
        if *token.offset(0) as i32 == '{' as i32 {
            if s >= 8 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: too many stages in shader %s (max is %i)\n\x00" as *const u8
                        as *const i8,
                    shader.name.as_mut_ptr(),
                    8i32,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if ParseStage(&mut *stages.as_mut_ptr().offset(s as isize), text) as u64 == 0 {
                return crate::src::qcommon::q_shared::qfalse;
            }
            stages[s as usize].active = crate::src::qcommon::q_shared::qtrue;
            s += 1
        } else if crate::src::qcommon::q_shared::Q_stricmpn(
            token,
            b"qer\x00" as *const u8 as *const i8,
            3,
        ) == 0
        {
            crate::src::qcommon::q_shared::SkipRestOfLine(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"q3map_sun\x00" as *const u8 as *const i8,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"q3map_sunExt\x00" as *const u8 as *const i8,
            ) == 0
        {
            let mut a: f32 = 0.;
            let mut b: f32 = 0.;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::renderergl1::tr_main::tr.sunLight[0] =
                atof(token) as crate::src::qcommon::q_shared::vec_t;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::renderergl1::tr_main::tr.sunLight[1] =
                atof(token) as crate::src::qcommon::q_shared::vec_t;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::renderergl1::tr_main::tr.sunLight[2] =
                atof(token) as crate::src::qcommon::q_shared::vec_t;
            crate::src::qcommon::q_math::VectorNormalize(
                crate::src::renderergl1::tr_main::tr.sunLight.as_mut_ptr(),
            );
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            a = atof(token) as f32;
            crate::src::renderergl1::tr_main::tr.sunLight[0] =
                crate::src::renderergl1::tr_main::tr.sunLight[0] * a;
            crate::src::renderergl1::tr_main::tr.sunLight[1] =
                crate::src::renderergl1::tr_main::tr.sunLight[1] * a;
            crate::src::renderergl1::tr_main::tr.sunLight[2] =
                crate::src::renderergl1::tr_main::tr.sunLight[2] * a;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            a = atof(token) as f32;
            a = ((a / 180f32) as f64 * 3.14159265358979323846) as f32;
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            b = atof(token) as f32;
            b = ((b / 180f32) as f64 * 3.14159265358979323846) as f32;
            crate::src::renderergl1::tr_main::tr.sunDirection[0] = (crate::stdlib::cos(a as f64)
                * crate::stdlib::cos(b as f64))
                as crate::src::qcommon::q_shared::vec_t;
            crate::src::renderergl1::tr_main::tr.sunDirection[1] = (crate::stdlib::sin(a as f64)
                * crate::stdlib::cos(b as f64))
                as crate::src::qcommon::q_shared::vec_t;
            crate::src::renderergl1::tr_main::tr.sunDirection[2] =
                crate::stdlib::sin(b as f64) as crate::src::qcommon::q_shared::vec_t;
            crate::src::qcommon::q_shared::SkipRestOfLine(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"deformVertexes\x00" as *const u8 as *const i8,
        ) == 0
        {
            ParseDeform(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"tesssize\x00" as *const u8 as *const i8,
        ) == 0
        {
            crate::src::qcommon::q_shared::SkipRestOfLine(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"clampTime\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) != 0 {
                shader.clampTime = atof(token)
            }
        } else if crate::src::qcommon::q_shared::Q_stricmpn(
            token,
            b"q3map\x00" as *const u8 as *const i8,
            5,
        ) == 0
        {
            crate::src::qcommon::q_shared::SkipRestOfLine(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"surfaceParm\x00" as *const u8 as *const i8,
        ) == 0
        {
            ParseSurfaceParm(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"nomipmaps\x00" as *const u8 as *const i8,
        ) == 0
        {
            shader.noMipMaps = crate::src::qcommon::q_shared::qtrue;
            shader.noPicMip = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"nopicmip\x00" as *const u8 as *const i8,
        ) == 0
        {
            shader.noPicMip = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"polygonOffset\x00" as *const u8 as *const i8,
        ) == 0
        {
            shader.polygonOffset = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"entityMergable\x00" as *const u8 as *const i8,
        ) == 0
        {
            shader.entityMergable = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"fogParms\x00" as *const u8 as *const i8,
        ) == 0
        {
            if ParseVector(text, 3, shader.fogParms.color.as_mut_ptr()) as u64 == 0 {
                return crate::src::qcommon::q_shared::qfalse;
            }
            if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
                let mut luminance: f32 = 0.;
                luminance = 0.2126 * shader.fogParms.color[0]
                    + 0.7152 * shader.fogParms.color[1]
                    + 0.0722 * shader.fogParms.color[2];
                shader.fogParms.color[0] = luminance;
                shader.fogParms.color[1] = luminance;
                shader.fogParms.color[2] = luminance
            } else if (*crate::src::renderergl1::tr_init::r_greyscale).value != 0. {
                let mut luminance_0: f32 = 0.;
                luminance_0 = 0.2126 * shader.fogParms.color[0]
                    + 0.7152 * shader.fogParms.color[1]
                    + 0.0722 * shader.fogParms.color[2];
                shader.fogParms.color[0] = shader.fogParms.color[0]
                    * (1.0 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luminance_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value;
                shader.fogParms.color[1] = shader.fogParms.color[1]
                    * (1.0 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luminance_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value;
                shader.fogParms.color[2] = shader.fogParms.color[2]
                    * (1.0 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luminance_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value
            }
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing parm for \'fogParms\' keyword in shader \'%s\'\n\x00"
                        as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
            } else {
                shader.fogParms.depthForOpaque = atof(token) as f32;
                // skip stuff that only the QuakeEdRadient needs
                // sun parms
                // skip stuff that only the q3map needs
                // skip stuff that only q3map or the server needs
                // no mip maps
                // no picmip adjustment
                // polygonOffset
                // entityMergable, allowing sprite surfaces from multiple entities
                // to be merged into one batch.  This is a savings for smoke
                // puffs and blood, but can't be used for anything where the
                // shader calcs (not the surface function) reference the entity color or scroll
                // fogParms
                // skip any old gradient directions
                crate::src::qcommon::q_shared::SkipRestOfLine(text);
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"portal\x00" as *const u8 as *const i8,
        ) == 0
        {
            shader.sort = crate::tr_local_h::SS_PORTAL as i32 as f32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"skyparms\x00" as *const u8 as *const i8,
        ) == 0
        {
            ParseSkyParms(text);
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"light\x00" as *const u8 as *const i8,
        ) == 0
        {
            crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"cull\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                text,
                crate::src::qcommon::q_shared::qfalse,
            );
            if *token.offset(0) as i32 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: missing cull parms in shader \'%s\'\n\x00" as *const u8 as *const i8,
                    shader.name.as_mut_ptr(),
                );
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"none\x00" as *const u8 as *const i8,
            ) == 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"twosided\x00" as *const u8 as *const i8,
                ) == 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"disable\x00" as *const u8 as *const i8,
                ) == 0
            {
                shader.cullType = crate::tr_local_h::CT_TWO_SIDED
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"back\x00" as *const u8 as *const i8,
            ) == 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"backside\x00" as *const u8 as *const i8,
                ) == 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"backsided\x00" as *const u8 as *const i8,
                ) == 0
            {
                shader.cullType = crate::tr_local_h::CT_BACK_SIDED
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: invalid cull parm \'%s\' in shader \'%s\'\n\x00" as *const u8
                        as *const i8,
                    token,
                    shader.name.as_mut_ptr(),
                );
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"sort\x00" as *const u8 as *const i8,
        ) == 0
        {
            ParseSort(text);
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"WARNING: unknown general shader parameter \'%s\' in \'%s\'\n\x00" as *const u8
                    as *const i8,
                token,
                shader.name.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    // portal
    // skyparms <cloudheight> <outerbox> <innerbox>
    // light <value> determines flaring in q3map, not needed here
    // cull <face>
    // sort
    //
    // ignore shaders that don't have any stages, unless it is a sky or fog
    //
    if s == 0 && shader.isSky as u64 == 0 && shader.contentFlags & 64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    shader.explicitlyDefined = crate::src::qcommon::q_shared::qtrue;
    return crate::src::qcommon::q_shared::qtrue;
}
/*
========================================================================================

SHADER OPTIMIZATION AND FOGGING

========================================================================================
*/
/*
===================
ComputeStageIteratorFunc

See if we can use on of the simple fastpath stage functions,
otherwise set to the generic stage function
===================
*/

unsafe extern "C" fn ComputeStageIteratorFunc() {
    shader.optimalStageIteratorFunc = Some(
        crate::src::renderergl1::tr_shade::RB_StageIteratorGeneric as unsafe extern "C" fn() -> (),
    );
    //
    // see if this should go into the sky path
    //
    if shader.isSky as u64 != 0 {
        shader.optimalStageIteratorFunc = Some(
            crate::src::renderergl1::tr_sky::RB_StageIteratorSky as unsafe extern "C" fn() -> (),
        );
        return;
    }
    if (*crate::src::renderergl1::tr_init::r_ignoreFastPath).integer != 0 {
        return;
    }
    //
    // see if this can go into the vertex lit fast path
    //
    if shader.numUnfoggedPasses == 1 {
        if stages[0].rgbGen == crate::tr_local_h::CGEN_LIGHTING_DIFFUSE {
            if stages[0].alphaGen == crate::tr_local_h::AGEN_IDENTITY {
                if stages[0].bundle[0].tcGen == crate::tr_local_h::TCGEN_TEXTURE {
                    if shader.polygonOffset as u64 == 0 {
                        if shader.multitextureEnv == 0 {
                            if shader.numDeforms == 0 {
                                shader.optimalStageIteratorFunc =
                                    Some(crate::src::renderergl1::tr_shade::RB_StageIteratorVertexLitTexture as
                                             unsafe extern "C" fn() -> ());
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    //
    // see if this can go into an optimized LM, multitextured path
    //
    if shader.numUnfoggedPasses == 1 {
        if stages[0].rgbGen == crate::tr_local_h::CGEN_IDENTITY
            && stages[0].alphaGen == crate::tr_local_h::AGEN_IDENTITY
        {
            if stages[0].bundle[0].tcGen == crate::tr_local_h::TCGEN_TEXTURE
                && stages[0].bundle[1].tcGen == crate::tr_local_h::TCGEN_LIGHTMAP
            {
                if shader.polygonOffset as u64 == 0 {
                    if shader.numDeforms == 0 {
                        if shader.multitextureEnv != 0 {
                            shader.optimalStageIteratorFunc =
                                Some(crate::src::renderergl1::tr_shade::RB_StageIteratorLightmappedMultitexture
                                         as unsafe extern "C" fn() -> ())
                        }
                    }
                }
            }
        }
    };
}

static mut collapse: [collapse_t; 9] = [
    {
        let mut init = collapse_t {
            blendA: 0,
            blendB: 0x30 | 0x1,
            multitextureEnv: 0x2100,
            multitextureBlend: 0,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0,
            blendB: 0x10 | 0x3,
            multitextureEnv: 0x2100,
            multitextureBlend: 0,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0x10 | 0x3,
            blendB: 0x10 | 0x3,
            multitextureEnv: 0x2100,
            multitextureBlend: 0x10 | 0x3,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0x30 | 0x1,
            blendB: 0x10 | 0x3,
            multitextureEnv: 0x2100,
            multitextureBlend: 0x10 | 0x3,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0x10 | 0x3,
            blendB: 0x30 | 0x1,
            multitextureEnv: 0x2100,
            multitextureBlend: 0x10 | 0x3,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0x30 | 0x1,
            blendB: 0x30 | 0x1,
            multitextureEnv: 0x2100,
            multitextureBlend: 0x10 | 0x3,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0,
            blendB: 0x20 | 0x2,
            multitextureEnv: 0x104,
            multitextureBlend: 0,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: 0x20 | 0x2,
            blendB: 0x20 | 0x2,
            multitextureEnv: 0x104,
            multitextureBlend: 0x20 | 0x2,
        };
        init
    },
    {
        let mut init = collapse_t {
            blendA: -(1),
            blendB: 0,
            multitextureEnv: 0,
            multitextureBlend: 0,
        };
        init
    },
];
/*
================
CollapseMultitexture

Attempt to combine two stages into a single multitexture stage
FIXME: I think modulated add + modulated add collapses incorrectly
=================
*/

unsafe extern "C" fn CollapseMultitexture() -> crate::src::qcommon::q_shared::qboolean {
    let mut abits: i32 = 0;
    let mut bbits: i32 = 0;
    let mut i: i32 = 0;
    let mut tmpBundle: crate::tr_local_h::textureBundle_t = crate::tr_local_h::textureBundle_t {
        image: [0 as *mut crate::tr_common_h::image_t; 8],
        numImageAnimations: 0,
        imageAnimationSpeed: 0.,
        tcGen: crate::tr_local_h::TCGEN_BAD,
        tcGenVectors: [[0.; 3]; 2],
        numTexMods: 0,
        texMods: 0 as *mut crate::tr_local_h::texModInfo_t,
        videoMapHandle: 0,
        isLightmap: crate::src::qcommon::q_shared::qfalse,
        isVideoMap: crate::src::qcommon::q_shared::qfalse,
    };
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // make sure both stages are active
    if stages[0].active as u64 == 0 || stages[1].active as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // on voodoo2, don't combine different tmus
    if crate::src::renderergl1::tr_init::glConfig.driverType == crate::tr_types_h::GLDRV_VOODOO {
        if (*stages[0].bundle[0].image[0]).TMU == (*stages[1].bundle[0].image[0]).TMU {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    abits = stages[0].stateBits as i32;
    bbits = stages[1].stateBits as i32;
    // make sure that both stages have identical state other than blend modes
    if abits & !(0xf0 | 0xf | 0x100) != bbits & !(0xf0 | 0xf | 0x100) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    abits &= 0xf0 | 0xf;
    bbits &= 0xf0 | 0xf;
    // search for a valid multitexture blend function
    i = 0;
    while collapse[i as usize].blendA != -(1) {
        if abits == collapse[i as usize].blendA && bbits == collapse[i as usize].blendB {
            break;
        }
        i += 1
    }
    // nothing found
    if collapse[i as usize].blendA == -(1) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // GL_ADD is a separate extension
    if collapse[i as usize].multitextureEnv == 0x104
        && crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable as u64 == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // make sure waveforms have identical parameters
    if stages[0].rgbGen != stages[1].rgbGen || stages[0].alphaGen != stages[1].alphaGen {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // an add collapse can only have identity colors
    if collapse[i as usize].multitextureEnv == 0x104
        && stages[0].rgbGen != crate::tr_local_h::CGEN_IDENTITY
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if stages[0].rgbGen == crate::tr_local_h::CGEN_WAVEFORM {
        if crate::stdlib::memcmp(
            &mut (*stages.as_mut_ptr().offset(0)).rgbWave as *mut crate::tr_local_h::waveForm_t
                as *const libc::c_void,
            &mut (*stages.as_mut_ptr().offset(1)).rgbWave as *mut crate::tr_local_h::waveForm_t
                as *const libc::c_void,
            ::std::mem::size_of::<crate::tr_local_h::waveForm_t>(),
        ) != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    if stages[0].alphaGen == crate::tr_local_h::AGEN_WAVEFORM {
        if crate::stdlib::memcmp(
            &mut (*stages.as_mut_ptr().offset(0)).alphaWave as *mut crate::tr_local_h::waveForm_t
                as *const libc::c_void,
            &mut (*stages.as_mut_ptr().offset(1)).alphaWave as *mut crate::tr_local_h::waveForm_t
                as *const libc::c_void,
            ::std::mem::size_of::<crate::tr_local_h::waveForm_t>(),
        ) != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    // make sure that lightmaps are in bundle 1 for 3dfx
    if stages[0].bundle[0].isLightmap as u64 != 0 {
        tmpBundle = stages[0].bundle[0];
        stages[0].bundle[0] = stages[1].bundle[0];
        stages[0].bundle[1] = tmpBundle
    } else {
        stages[0].bundle[1] = stages[1].bundle[0]
    }
    // set the new blend state bits
    shader.multitextureEnv = collapse[i as usize].multitextureEnv;
    stages[0].stateBits &= !(0xf0i32 | 0xf) as u32;
    stages[0].stateBits |= collapse[i as usize].multitextureBlend as u32;
    //
    // move down subsequent shaders
    //
    crate::stdlib::memmove(
        &mut *stages.as_mut_ptr().offset(1) as *mut crate::tr_local_h::shaderStage_t
            as *mut libc::c_void,
        &mut *stages.as_mut_ptr().offset(2) as *mut crate::tr_local_h::shaderStage_t
            as *const libc::c_void,
        (::std::mem::size_of::<crate::tr_local_h::shaderStage_t>())
            .wrapping_mul((8i32 - 2) as usize),
    );
    crate::stdlib::memset(
        &mut *stages.as_mut_ptr().offset((8i32 - 1) as isize)
            as *mut crate::tr_local_h::shaderStage_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_local_h::shaderStage_t>(),
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=============

FixRenderCommandList
https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=493
Arnout: this is a nasty issue. Shaders can be registered after drawsurfaces are generated
but before the frame is rendered. This will, for the duration of one frame, cause drawsurfaces
to be rendered with bad shaders. To fix this, need to go through all render commands and fix
sortedIndex.
==============
*/

unsafe extern "C" fn FixRenderCommandList(mut newShader: i32) {
    let mut cmdList: *mut crate::tr_local_h::renderCommandList_t =
        &mut (*crate::src::renderergl1::tr_backend::backEndData).commands;
    if !cmdList.is_null() {
        let mut curCmd: *const libc::c_void = (*cmdList).cmds.as_mut_ptr() as *const libc::c_void;
        loop {
            curCmd = ((curCmd as usize)
                .wrapping_add(::std::mem::size_of::<*mut libc::c_void>())
                .wrapping_sub(1usize)
                & !(::std::mem::size_of::<*mut libc::c_void>()).wrapping_sub(1usize))
                as *mut libc::c_void;
            match *(curCmd as *const i32) {
                1 => {
                    let mut sc_cmd: *const crate::tr_local_h::setColorCommand_t =
                        curCmd as *const crate::tr_local_h::setColorCommand_t;
                    curCmd = sc_cmd.offset(1) as *const libc::c_void
                }
                2 => {
                    let mut sp_cmd: *const crate::tr_local_h::stretchPicCommand_t =
                        curCmd as *const crate::tr_local_h::stretchPicCommand_t;
                    curCmd = sp_cmd.offset(1) as *const libc::c_void
                }
                3 => {
                    let mut i: i32 = 0;
                    let mut drawSurf: *mut crate::tr_local_h::drawSurf_t =
                        0 as *mut crate::tr_local_h::drawSurf_t;
                    let mut shader_0: *mut crate::tr_local_h::shader_t =
                        0 as *mut crate::tr_local_h::shader_t;
                    let mut fogNum: i32 = 0;
                    let mut entityNum: i32 = 0;
                    let mut dlightMap: i32 = 0;
                    let mut sortedIndex: i32 = 0;
                    let mut ds_cmd: *const crate::tr_local_h::drawSurfsCommand_t =
                        curCmd as *const crate::tr_local_h::drawSurfsCommand_t;
                    i = 0;
                    drawSurf = (*ds_cmd).drawSurfs;
                    while i < (*ds_cmd).numDrawSurfs {
                        crate::src::renderergl1::tr_main::R_DecomposeSort(
                            (*drawSurf).sort,
                            &mut entityNum,
                            &mut shader_0,
                            &mut fogNum,
                            &mut dlightMap,
                        );
                        sortedIndex =
                            ((*drawSurf).sort >> 7 + 10 & (((1i32) << 14) - 1) as u32) as i32;
                        if sortedIndex >= newShader {
                            sortedIndex += 1;
                            (*drawSurf).sort =
                                (sortedIndex << 7 + 10 | entityNum | fogNum << 2 | dlightMap) as u32
                        }
                        i += 1;
                        drawSurf = drawSurf.offset(1)
                    }
                    curCmd = ds_cmd.offset(1) as *const libc::c_void
                }
                4 => {
                    let mut db_cmd: *const crate::tr_local_h::drawBufferCommand_t =
                        curCmd as *const crate::tr_local_h::drawBufferCommand_t;
                    curCmd = db_cmd.offset(1) as *const libc::c_void
                }
                5 => {
                    let mut sb_cmd: *const crate::tr_local_h::swapBuffersCommand_t =
                        curCmd as *const crate::tr_local_h::swapBuffersCommand_t;
                    curCmd = sb_cmd.offset(1) as *const libc::c_void
                }
                0 | _ => return,
            }
        }
    };
}
/*
==============
SortNewShader

Positions the most recently created shader in the tr.sortedShaders[]
array so that the shader->sort key is sorted relative to the other
shaders.

Sets shader->sortedIndex
==============
*/

unsafe extern "C" fn SortNewShader() {
    let mut i: i32 = 0;
    let mut sort: f32 = 0.;
    let mut newShader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    newShader = crate::src::renderergl1::tr_main::tr.shaders
        [(crate::src::renderergl1::tr_main::tr.numShaders - 1) as usize];
    sort = (*newShader).sort;
    i = crate::src::renderergl1::tr_main::tr.numShaders - 2;
    while i >= 0 {
        if (*crate::src::renderergl1::tr_main::tr.sortedShaders[i as usize]).sort <= sort {
            break;
        }
        crate::src::renderergl1::tr_main::tr.sortedShaders[(i + 1) as usize] =
            crate::src::renderergl1::tr_main::tr.sortedShaders[i as usize];
        (*crate::src::renderergl1::tr_main::tr.sortedShaders[(i + 1) as usize]).sortedIndex += 1;
        i -= 1
    }
    // Arnout: fix rendercommandlist
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=493
    FixRenderCommandList(i + 1);
    (*newShader).sortedIndex = i + 1;
    crate::src::renderergl1::tr_main::tr.sortedShaders[(i + 1) as usize] = newShader;
}
/*
====================
GeneratePermanentShader
====================
*/

unsafe extern "C" fn GeneratePermanentShader() -> *mut crate::tr_local_h::shader_t {
    let mut newShader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut _i: i32 = 0;
    let mut _b: i32 = 0;
    let mut size: i32 = 0;
    let mut hash: i32 = 0;
    if crate::src::renderergl1::tr_main::tr.numShaders == (1) << 14 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: GeneratePermanentShader - MAX_SHADERS hit\n\x00" as *const u8 as *const i8,
        );
        return crate::src::renderergl1::tr_main::tr.defaultShader;
    }
    newShader = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::shader_t>() as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::shader_t;
    *newShader = shader;
    if shader.sort <= crate::tr_local_h::SS_OPAQUE as i32 as f32 {
        (*newShader).fogPass = crate::tr_local_h::FP_EQUAL
    } else if shader.contentFlags & 64 != 0 {
        (*newShader).fogPass = crate::tr_local_h::FP_LE
    }
    crate::src::renderergl1::tr_main::tr.shaders
        [crate::src::renderergl1::tr_main::tr.numShaders as usize] = newShader;
    (*newShader).index = crate::src::renderergl1::tr_main::tr.numShaders;
    crate::src::renderergl1::tr_main::tr.sortedShaders
        [crate::src::renderergl1::tr_main::tr.numShaders as usize] = newShader;
    (*newShader).sortedIndex = crate::src::renderergl1::tr_main::tr.numShaders;
    crate::src::renderergl1::tr_main::tr.numShaders += 1;

    for i in 0..(*newShader).numUnfoggedPasses {
        if stages[i as usize].active as u64 == 0 {
            break;
        }

        (*newShader).stages[i as usize] = crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ::std::mem::size_of::<crate::tr_local_h::shaderStage_t>() as i32,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_local_h::shaderStage_t;

        *(*newShader).stages[i as usize] = stages[i as usize];
        for b in 0..2 {
            size = ((*(*newShader).stages[i as usize]).bundle[b as usize].numTexMods as usize)
                .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::texModInfo_t>())
                as i32;

            (*(*newShader).stages[i as usize]).bundle[b as usize].texMods =
                crate::src::renderergl1::tr_main::ri
                    .Hunk_Alloc
                    .expect("non-null function pointer")(
                    size, crate::src::qcommon::q_shared::h_low
                ) as *mut crate::tr_local_h::texModInfo_t;

            crate::stdlib::memcpy(
                (*(*newShader).stages[i as usize]).bundle[b as usize].texMods as *mut libc::c_void,
                stages[i as usize].bundle[b as usize].texMods as *const libc::c_void,
                size as usize,
            );
        }
    }
    SortNewShader();
    hash = generateHashValue((*newShader).name.as_mut_ptr(), 1024) as i32;
    (*newShader).next = hashTable[hash as usize];
    hashTable[hash as usize] = newShader;
    return newShader;
}
/*
=================
VertexLightingCollapse

If vertex lighting is enabled, only render a single
pass, trying to guess which is the correct one to best approximate
what it is supposed to look like.
=================
*/

unsafe extern "C" fn VertexLightingCollapse() {
    let mut stage: i32 = 0;
    let mut bestStage: *mut crate::tr_local_h::shaderStage_t =
        0 as *mut crate::tr_local_h::shaderStage_t;
    let mut bestImageRank: i32 = 0;
    let mut rank: i32 = 0;
    // if we aren't opaque, just use the first pass
    if shader.sort == crate::tr_local_h::SS_OPAQUE as i32 as f32 {
        // pick the best texture for the single pass
        bestStage = &mut *stages.as_mut_ptr().offset(0) as *mut crate::tr_local_h::shaderStage_t;
        bestImageRank = -(999999);
        stage = 0;
        while stage < 8 {
            let mut pStage: *mut crate::tr_local_h::shaderStage_t =
                &mut *stages.as_mut_ptr().offset(stage as isize)
                    as *mut crate::tr_local_h::shaderStage_t;
            if (*pStage).active as u64 == 0 {
                break;
            }
            rank = 0;
            if (*pStage).bundle[0].isLightmap as u64 != 0 {
                rank -= 100
            }
            if (*pStage).bundle[0].tcGen != crate::tr_local_h::TCGEN_TEXTURE {
                rank -= 5
            }
            if (*pStage).bundle[0].numTexMods != 0 {
                rank -= 5
            }
            if (*pStage).rgbGen != crate::tr_local_h::CGEN_IDENTITY
                && (*pStage).rgbGen != crate::tr_local_h::CGEN_IDENTITY_LIGHTING
            {
                rank -= 3
            }
            if rank > bestImageRank {
                bestImageRank = rank;
                bestStage = pStage
            }
            stage += 1
        }
        stages[0].bundle[0] = (*bestStage).bundle[0];
        stages[0].stateBits &= !(0xf0i32 | 0xf) as u32;
        stages[0].stateBits |= 0x100;
        if shader.lightmapIndex == -(1) {
            stages[0].rgbGen = crate::tr_local_h::CGEN_LIGHTING_DIFFUSE
        } else {
            stages[0].rgbGen = crate::tr_local_h::CGEN_EXACT_VERTEX
        }
        stages[0].alphaGen = crate::tr_local_h::AGEN_SKIP
    } else {
        // don't use a lightmap (tesla coils)
        if stages[0].bundle[0].isLightmap as u64 != 0 {
            stages[0] = stages[1]
        }
        // if we were in a cross-fade cgen, hack it to normal
        if stages[0].rgbGen == crate::tr_local_h::CGEN_ONE_MINUS_ENTITY
            || stages[1].rgbGen == crate::tr_local_h::CGEN_ONE_MINUS_ENTITY
        {
            stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING
        }
        if stages[0].rgbGen == crate::tr_local_h::CGEN_WAVEFORM
            && stages[0].rgbWave.func == crate::tr_local_h::GF_SAWTOOTH
            && (stages[1].rgbGen == crate::tr_local_h::CGEN_WAVEFORM
                && stages[1].rgbWave.func == crate::tr_local_h::GF_INVERSE_SAWTOOTH)
        {
            stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING
        }
        if stages[0].rgbGen == crate::tr_local_h::CGEN_WAVEFORM
            && stages[0].rgbWave.func == crate::tr_local_h::GF_INVERSE_SAWTOOTH
            && (stages[1].rgbGen == crate::tr_local_h::CGEN_WAVEFORM
                && stages[1].rgbWave.func == crate::tr_local_h::GF_SAWTOOTH)
        {
            stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING
        }
    }
    stage = 1;
    while stage < 8 {
        let mut pStage_0: *mut crate::tr_local_h::shaderStage_t =
            &mut *stages.as_mut_ptr().offset(stage as isize)
                as *mut crate::tr_local_h::shaderStage_t;
        if (*pStage_0).active as u64 == 0 {
            break;
        }
        crate::stdlib::memset(
            pStage_0 as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_local_h::shaderStage_t>(),
        );
        stage += 1
    }
}
/*
===============
InitShader
===============
*/

unsafe extern "C" fn InitShader(mut name: *const i8, mut lightmapIndex: i32) {
    let mut i: i32 = 0;
    // clear the global shader
    crate::stdlib::memset(
        &mut shader as *mut crate::tr_local_h::shader_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_local_h::shader_t>(),
    );
    crate::stdlib::memset(
        &mut stages as *mut [crate::tr_local_h::shaderStage_t; 8] as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[crate::tr_local_h::shaderStage_t; 8]>(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        shader.name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    shader.lightmapIndex = lightmapIndex;
    i = 0;
    while i < 8 {
        stages[i as usize].bundle[0].texMods = texMods[i as usize].as_mut_ptr();
        i += 1
    }
}
/*
=========================
FinishShader

Returns a freshly allocated shader with all the needed info
from the current global working shader
=========================
*/

unsafe extern "C" fn FinishShader() -> *mut crate::tr_local_h::shader_t {
    let mut stage: i32 = 0;
    let mut hasLightmapStage: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut vertexLightmap: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    hasLightmapStage = crate::src::qcommon::q_shared::qfalse;
    vertexLightmap = crate::src::qcommon::q_shared::qfalse;
    //
    // set sky stuff appropriate
    //
    if shader.isSky as u64 != 0 {
        shader.sort = crate::tr_local_h::SS_ENVIRONMENT as i32 as f32
    }
    //
    // set polygon offset
    //
    if shader.polygonOffset != 0 && shader.sort == 0. {
        shader.sort = crate::tr_local_h::SS_DECAL as i32 as f32
    }
    //
    // set appropriate stage information
    //
    stage = 0;
    while stage < 8 {
        let mut pStage: *mut crate::tr_local_h::shaderStage_t =
            &mut *stages.as_mut_ptr().offset(stage as isize)
                as *mut crate::tr_local_h::shaderStage_t;
        if (*pStage).active as u64 == 0 {
            break;
        }
        // check for a missing texture
        if (*pStage).bundle[0].image[0].is_null() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"Shader %s has a stage with no image\n\x00" as *const u8 as *const i8,
                shader.name.as_mut_ptr(),
            );
            (*pStage).active = crate::src::qcommon::q_shared::qfalse;
            stage += 1
        } else if (*pStage).isDetail != 0
            && (*crate::src::renderergl1::tr_init::r_detailTextures).integer == 0
        {
            let mut index: i32 = 0;
            index = stage + 1;
            while index < 8 {
                if stages[index as usize].active as u64 == 0 {
                    break;
                }
                index += 1
            }
            if index < 8 {
                crate::stdlib::memmove(
                    pStage as *mut libc::c_void,
                    pStage.offset(1isize) as *const libc::c_void,
                    (::std::mem::size_of::<crate::tr_local_h::shaderStage_t>())
                        .wrapping_mul((index - stage) as usize),
                );
            } else {
                if (stage + 1) < 8 {
                    crate::stdlib::memmove(
                        pStage as *mut libc::c_void,
                        pStage.offset(1isize) as *const libc::c_void,
                        (::std::mem::size_of::<crate::tr_local_h::shaderStage_t>())
                            .wrapping_mul((index - stage - 1i32) as usize),
                    );
                }
                crate::stdlib::memset(
                    &mut *stages.as_mut_ptr().offset((index - 1i32) as isize)
                        as *mut crate::tr_local_h::shaderStage_t
                        as *mut libc::c_void,
                    0i32,
                    ::std::mem::size_of::<crate::tr_local_h::shaderStage_t>(),
                );
            }
        } else {
            //
            // ditch this stage if it's detail and detail textures are disabled
            //
            //
            // default texture coordinate generation
            //
            if (*pStage).bundle[0].isLightmap as u64 != 0 {
                if (*pStage).bundle[0].tcGen == crate::tr_local_h::TCGEN_BAD {
                    (*pStage).bundle[0].tcGen = crate::tr_local_h::TCGEN_LIGHTMAP
                }
                hasLightmapStage = crate::src::qcommon::q_shared::qtrue
            } else if (*pStage).bundle[0].tcGen == crate::tr_local_h::TCGEN_BAD {
                (*pStage).bundle[0].tcGen = crate::tr_local_h::TCGEN_TEXTURE
            }
            // not a true lightmap but we want to leave existing
            // behaviour in place and not print out a warning
            //if (pStage->rgbGen == CGEN_VERTEX) {
            //  vertexLightmap = qtrue;
            //}
            //
            // determine sort order and fog color adjustment
            //
            if (*pStage).stateBits & (0xfi32 | 0xf0) as u32 != 0
                && stages[0].stateBits & (0xfi32 | 0xf0) as u32 != 0
            {
                let mut blendSrcBits: i32 = ((*pStage).stateBits & 0xf) as i32;
                let mut blendDstBits: i32 = ((*pStage).stateBits & 0xf0) as i32;
                // fog color adjustment only works for blend modes that have a contribution
                // that aproaches 0 as the modulate values aproach 0 --
                // GL_ONE, GL_ONE
                // GL_ZERO, GL_ONE_MINUS_SRC_COLOR
                // GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA
                // modulate, additive
                if blendSrcBits == 0x2 && blendDstBits == 0x20
                    || blendSrcBits == 0x1 && blendDstBits == 0x40
                {
                    (*pStage).adjustColorsForFog = crate::tr_local_h::ACFF_MODULATE_RGB
                } else if blendSrcBits == 0x5 && blendDstBits == 0x60 {
                    (*pStage).adjustColorsForFog = crate::tr_local_h::ACFF_MODULATE_ALPHA
                } else if blendSrcBits == 0x2 && blendDstBits == 0x60 {
                    (*pStage).adjustColorsForFog = crate::tr_local_h::ACFF_MODULATE_RGBA
                }
                // strict blend
                // premultiplied alpha
                // don't screw with sort order if this is a portal or environment
                if shader.sort == 0. {
                    // see through item, like a grill or grate
                    if (*pStage).stateBits & 0x100 != 0 {
                        shader.sort = crate::tr_local_h::SS_SEE_THROUGH as i32 as f32
                    } else {
                        shader.sort = crate::tr_local_h::SS_BLEND0 as i32 as f32
                    }
                }
            }
            stage += 1
        }
    }
    // there are times when you will need to manually apply a sort to
    // opaque alpha tested shaders that have later blend passes
    if shader.sort == 0. {
        shader.sort = crate::tr_local_h::SS_OPAQUE as i32 as f32
    }
    //
    // if we are in r_vertexLight mode, never use a lightmap texture
    //
    if stage > 1
        && ((*crate::src::renderergl1::tr_init::r_vertexLight).integer != 0
            && (*crate::src::renderergl1::tr_init::r_uiFullScreen).integer == 0
            || crate::src::renderergl1::tr_init::glConfig.hardwareType
                == crate::tr_types_h::GLHW_PERMEDIA2)
    {
        VertexLightingCollapse();
        stage = 1;
        hasLightmapStage = crate::src::qcommon::q_shared::qfalse
    }
    //
    // look for multitexture potential
    //
    if stage > 1 && CollapseMultitexture() != 0 {
        stage -= 1
    }
    if shader.lightmapIndex >= 0 && hasLightmapStage as u64 == 0 {
        if vertexLightmap as u64 != 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                b"WARNING: shader \'%s\' has VERTEX forced lightmap!\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                b"WARNING: shader \'%s\' has lightmap but no lightmap stage!\n\x00" as *const u8
                    as *const i8,
                shader.name.as_mut_ptr(),
            );
            shader.lightmapIndex = -(1)
        }
    }
    //
    // compute number of passes
    //
    shader.numUnfoggedPasses = stage;
    // fogonly shaders don't have any normal passes
    if stage == 0 && shader.isSky as u64 == 0 {
        shader.sort = crate::tr_local_h::SS_FOG as i32 as f32
    }
    // determine which stage iterator function is appropriate
    ComputeStageIteratorFunc();
    return GeneratePermanentShader();
}
//========================================================================================
/*
====================
FindShaderInShaderText

Scans the combined text description of all the shader files for
the given shader name.

return NULL if not found

If found, it will return a valid shader
=====================
*/

unsafe extern "C" fn FindShaderInShaderText(mut shadername: *const i8) -> *mut i8 {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut hash: i32 = 0;
    hash = generateHashValue(shadername, 2048) as i32;
    if !shaderTextHashTable[hash as usize].is_null() {
        i = 0;
        while !(*shaderTextHashTable[hash as usize].offset(i as isize)).is_null() {
            p = *shaderTextHashTable[hash as usize].offset(i as isize);
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                &mut p,
                crate::src::qcommon::q_shared::qtrue,
            );
            if crate::src::qcommon::q_shared::Q_stricmp(token, shadername) == 0 {
                return p;
            }
            i += 1
        }
    }
    p = s_shaderText;
    if p.is_null() {
        return 0 as *mut i8;
    }
    loop
    // look for label
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            &mut p,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *token.offset(0) as i32 == 0 {
            break;
        }
        if crate::src::qcommon::q_shared::Q_stricmp(token, shadername) == 0 {
            return p;
        } else {
            // skip the definition
            crate::src::qcommon::q_shared::SkipBracedSection(&mut p, 0i32);
        }
    }
    return 0 as *mut i8;
}
/*
==================
R_FindShaderByName

Will always return a valid shader, but it might be the
default shader if the real one can't be found.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_FindShaderByName(
    mut name: *const i8,
) -> *mut crate::tr_local_h::shader_t {
    let mut strippedName: [i8; 64] = [0; 64];
    let mut hash: i32 = 0;
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    if name.is_null() || *name.offset(0) as i32 == 0 {
        return crate::src::renderergl1::tr_main::tr.defaultShader;
    }
    crate::src::qcommon::q_shared::COM_StripExtension(
        name,
        strippedName.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    hash = generateHashValue(strippedName.as_mut_ptr(), 1024) as i32;
    //
    // see if the shader is already loaded
    //
    sh = hashTable[hash as usize];
    while !sh.is_null() {
        // NOTE: if there was no shader or image available with the name strippedName
        // then a default shader is created with lightmapIndex == LIGHTMAP_NONE, so we
        // have to check all default shaders otherwise for every call to R_FindShader
        // with that same strippedName a new default shader is created.
        if crate::src::qcommon::q_shared::Q_stricmp(
            (*sh).name.as_mut_ptr(),
            strippedName.as_mut_ptr(),
        ) == 0
        {
            // match found
            return sh;
        }
        sh = (*sh).next
    }
    return crate::src::renderergl1::tr_main::tr.defaultShader;
}
/*
===============
R_FindShader

Will always return a valid shader, but it might be the
default shader if the real one can't be found.

In the interest of not requiring an explicit shader text entry to
be defined for every single image used in the game, three default
shader behaviors can be auto-created for any image:

If lightmapIndex == LIGHTMAP_NONE, then the image will have
dynamic diffuse lighting applied to it, as appropriate for most
entity skin surfaces.

If lightmapIndex == LIGHTMAP_2D, then the image will be used
for 2D rendering unless an explicit shader is found

If lightmapIndex == LIGHTMAP_BY_VERTEX, then the image will use
the vertex rgba modulate values, as appropriate for misc_model
pre-lit surfaces.

Other lightmapIndex values will have a lightmap stage created
and src*dest blending applied with the texture, as appropriate for
most world construction surfaces.

===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_FindShader(
    mut name: *const i8,
    mut lightmapIndex: i32,
    mut mipRawImage: crate::src::qcommon::q_shared::qboolean,
) -> *mut crate::tr_local_h::shader_t {
    let mut strippedName: [i8; 64] = [0; 64];
    let mut hash: i32 = 0;
    let mut shaderText: *mut i8 = 0 as *mut i8;
    let mut image: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    if *name.offset(0) as i32 == 0 {
        return crate::src::renderergl1::tr_main::tr.defaultShader;
    }
    // use (fullbright) vertex lighting if the bsp file doesn't have
    // lightmaps
    if lightmapIndex >= 0 && lightmapIndex >= crate::src::renderergl1::tr_main::tr.numLightmaps {
        lightmapIndex = -(3)
    } else if lightmapIndex < -(4) {
        // negative lightmap indexes cause stray pointers (think tr.lightmaps[lightmapIndex])
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: shader \'%s\' has invalid lightmap index of %d\n\x00" as *const u8
                as *const i8,
            name,
            lightmapIndex,
        );
        lightmapIndex = -(3)
    }
    crate::src::qcommon::q_shared::COM_StripExtension(
        name,
        strippedName.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    hash = generateHashValue(strippedName.as_mut_ptr(), 1024) as i32;
    //
    // see if the shader is already loaded
    //
    sh = hashTable[hash as usize];
    while !sh.is_null() {
        // NOTE: if there was no shader or image available with the name strippedName
        // then a default shader is created with lightmapIndex == LIGHTMAP_NONE, so we
        // have to check all default shaders otherwise for every call to R_FindShader
        // with that same strippedName a new default shader is created.
        if ((*sh).lightmapIndex == lightmapIndex || (*sh).defaultShader != 0)
            && crate::src::qcommon::q_shared::Q_stricmp(
                (*sh).name.as_mut_ptr(),
                strippedName.as_mut_ptr(),
            ) == 0
        {
            // match found
            return sh;
        }
        sh = (*sh).next
    }
    InitShader(strippedName.as_mut_ptr(), lightmapIndex);
    // FIXME: set these "need" values appropriately
    shader.needsNormal = crate::src::qcommon::q_shared::qtrue;
    shader.needsST1 = crate::src::qcommon::q_shared::qtrue;
    shader.needsST2 = crate::src::qcommon::q_shared::qtrue;
    shader.needsColor = crate::src::qcommon::q_shared::qtrue;
    //
    // attempt to define shader from an explicit parameter file
    //
    shaderText = FindShaderInShaderText(strippedName.as_mut_ptr());
    if !shaderText.is_null() {
        // enable this when building a pak file to get a global list
        // of all explicit shaders
        if (*crate::src::renderergl1::tr_init::r_printShaders).integer != 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"*SHADER* %s\n\x00" as *const u8 as *const i8,
                name,
            );
        }
        if ParseShader(&mut shaderText) as u64 == 0 {
            // had errors, so use default shader
            shader.defaultShader = crate::src::qcommon::q_shared::qtrue
        }
        sh = FinishShader();
        return sh;
    }
    //
    // if not defined in the in-memory shader descriptions,
    // look for a single supported image file
    //
    let mut flags: crate::tr_common_h::imgFlags_t = crate::tr_common_h::IMGFLAG_NONE;
    flags = crate::tr_common_h::IMGFLAG_NONE;
    if mipRawImage as u64 != 0 {
        flags = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
            flags
                | (crate::tr_common_h::IMGFLAG_MIPMAP as i32
                    | crate::tr_common_h::IMGFLAG_PICMIP as i32) as u32,
        )
    } else {
        flags = ::std::mem::transmute::<u32, crate::tr_common_h::imgFlags_t>(
            flags | crate::tr_common_h::IMGFLAG_CLAMPTOEDGE,
        )
    }
    image = crate::src::renderergl1::tr_image::R_FindImageFile(
        name,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        flags,
    );
    if image.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"Couldn\'t find image file for shader %s\n\x00" as *const u8 as *const i8,
            name,
        );
        shader.defaultShader = crate::src::qcommon::q_shared::qtrue;
        return FinishShader();
    }
    //
    // create the default shading commands
    //
    if shader.lightmapIndex == -(1) {
        // dynamic colors at vertexes
        stages[0].bundle[0].image[0] = image;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_LIGHTING_DIFFUSE;
        stages[0].stateBits = 0x100
    } else if shader.lightmapIndex == -(3) {
        // explicit colors at vertexes
        stages[0].bundle[0].image[0] = image;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_EXACT_VERTEX;
        stages[0].alphaGen = crate::tr_local_h::AGEN_SKIP;
        stages[0].stateBits = 0x100
    } else if shader.lightmapIndex == -(4) {
        // GUI elements
        stages[0].bundle[0].image[0] = image;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_VERTEX;
        stages[0].alphaGen = crate::tr_local_h::AGEN_VERTEX;
        stages[0].stateBits = (0x10000i32 | 0x5 | 0x60) as u32
    } else if shader.lightmapIndex == -(2) {
        // fullbright level
        stages[0].bundle[0].image[0] = crate::src::renderergl1::tr_main::tr.whiteImage;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING;
        stages[0].stateBits = 0x100;
        stages[1].bundle[0].image[0] = image;
        stages[1].active = crate::src::qcommon::q_shared::qtrue;
        stages[1].rgbGen = crate::tr_local_h::CGEN_IDENTITY;
        stages[1].stateBits |= (0x3i32 | 0x10) as u32
    } else {
        // two pass lightmap
        stages[0].bundle[0].image[0] = *crate::src::renderergl1::tr_main::tr
            .lightmaps
            .offset(shader.lightmapIndex as isize); // lightmaps are scaled on creation
        stages[0].bundle[0].isLightmap = crate::src::qcommon::q_shared::qtrue;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY;
        // for identitylight
        stages[0].stateBits = 0x100;
        stages[1].bundle[0].image[0] = image;
        stages[1].active = crate::src::qcommon::q_shared::qtrue;
        stages[1].rgbGen = crate::tr_local_h::CGEN_IDENTITY;
        stages[1].stateBits |= (0x3i32 | 0x10) as u32
    }
    return FinishShader();
}
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterShaderFromImage(
    mut name: *const i8,
    mut lightmapIndex: i32,
    mut image: *mut crate::tr_common_h::image_t,
    mut _mipRawImage: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut hash: i32 = 0;
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    hash = generateHashValue(name, 1024) as i32;
    // probably not necessary since this function
    // only gets called from tr_font.c with lightmapIndex == LIGHTMAP_2D
    // but better safe than sorry.
    if lightmapIndex >= crate::src::renderergl1::tr_main::tr.numLightmaps {
        lightmapIndex = -(2)
    }
    //
    // see if the shader is already loaded
    //
    sh = hashTable[hash as usize];
    while !sh.is_null() {
        // NOTE: if there was no shader or image available with the name strippedName
        // then a default shader is created with lightmapIndex == LIGHTMAP_NONE, so we
        // have to check all default shaders otherwise for every call to R_FindShader
        // with that same strippedName a new default shader is created.
        if ((*sh).lightmapIndex == lightmapIndex || (*sh).defaultShader != 0)
            && crate::src::qcommon::q_shared::Q_stricmp((*sh).name.as_mut_ptr(), name) == 0
        {
            // match found
            return (*sh).index;
        }
        sh = (*sh).next
    }
    InitShader(name, lightmapIndex);
    // FIXME: set these "need" values appropriately
    shader.needsNormal = crate::src::qcommon::q_shared::qtrue;
    shader.needsST1 = crate::src::qcommon::q_shared::qtrue;
    shader.needsST2 = crate::src::qcommon::q_shared::qtrue;
    shader.needsColor = crate::src::qcommon::q_shared::qtrue;
    //
    // create the default shading commands
    //
    if shader.lightmapIndex == -(1) {
        // dynamic colors at vertexes
        stages[0].bundle[0].image[0] = image;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_LIGHTING_DIFFUSE;
        stages[0].stateBits = 0x100
    } else if shader.lightmapIndex == -(3) {
        // explicit colors at vertexes
        stages[0].bundle[0].image[0] = image;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_EXACT_VERTEX;
        stages[0].alphaGen = crate::tr_local_h::AGEN_SKIP;
        stages[0].stateBits = 0x100
    } else if shader.lightmapIndex == -(4) {
        // GUI elements
        stages[0].bundle[0].image[0] = image;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_VERTEX;
        stages[0].alphaGen = crate::tr_local_h::AGEN_VERTEX;
        stages[0].stateBits = (0x10000i32 | 0x5 | 0x60) as u32
    } else if shader.lightmapIndex == -(2) {
        // fullbright level
        stages[0].bundle[0].image[0] = crate::src::renderergl1::tr_main::tr.whiteImage;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY_LIGHTING;
        stages[0].stateBits = 0x100;
        stages[1].bundle[0].image[0] = image;
        stages[1].active = crate::src::qcommon::q_shared::qtrue;
        stages[1].rgbGen = crate::tr_local_h::CGEN_IDENTITY;
        stages[1].stateBits |= (0x3i32 | 0x10) as u32
    } else {
        // two pass lightmap
        stages[0].bundle[0].image[0] = *crate::src::renderergl1::tr_main::tr
            .lightmaps
            .offset(shader.lightmapIndex as isize); // lightmaps are scaled on creation
        stages[0].bundle[0].isLightmap = crate::src::qcommon::q_shared::qtrue;
        stages[0].active = crate::src::qcommon::q_shared::qtrue;
        stages[0].rgbGen = crate::tr_local_h::CGEN_IDENTITY;
        // for identitylight
        stages[0].stateBits = 0x100;
        stages[1].bundle[0].image[0] = image;
        stages[1].active = crate::src::qcommon::q_shared::qtrue;
        stages[1].rgbGen = crate::tr_local_h::CGEN_IDENTITY;
        stages[1].stateBits |= (0x3i32 | 0x10) as u32
    }
    sh = FinishShader();
    return (*sh).index;
}
/*
====================
RE_RegisterShader

This is the exported shader entry point for the rest of the system
It will always return an index that will be valid.

This should really only be used for explicit shaders, because there is no
way to ask for different implicit lighting modes (vertex, lightmap, etc)
====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterShaderLightMap(
    mut name: *const i8,
    mut lightmapIndex: i32,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    if crate::stdlib::strlen(name) >= 64 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Shader name exceeds MAX_QPATH\n\x00" as *const u8 as *const i8,
        );
        return 0i32;
    }
    sh = R_FindShader(name, lightmapIndex, crate::src::qcommon::q_shared::qtrue);
    // we want to return 0 if the shader failed to
    // load for some reason, but R_FindShader should
    // still keep a name allocated for it, so if
    // something calls RE_RegisterShader again with
    // the same name, we don't try looking for it again
    if (*sh).defaultShader as u64 != 0 {
        return 0i32;
    }
    return (*sh).index;
}
/*
====================
RE_RegisterShader

This is the exported shader entry point for the rest of the system
It will always return an index that will be valid.

This should really only be used for explicit shaders, because there is no
way to ask for different implicit lighting modes (vertex, lightmap, etc)
====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterShader(
    mut name: *const i8,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    if crate::stdlib::strlen(name) >= 64 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Shader name exceeds MAX_QPATH\n\x00" as *const u8 as *const i8,
        );
        return 0i32;
    }
    sh = R_FindShader(name, -(4), crate::src::qcommon::q_shared::qtrue);
    // we want to return 0 if the shader failed to
    // load for some reason, but R_FindShader should
    // still keep a name allocated for it, so if
    // something calls RE_RegisterShader again with
    // the same name, we don't try looking for it again
    if (*sh).defaultShader as u64 != 0 {
        return 0i32;
    }
    return (*sh).index;
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
// for color, lightmap, diffuse, and specular
// normals are swizzled, deluxe are not
// game path, including extension
// source image
// after power of two and picmip but not including clamp to MAX_TEXTURE_SIZE
// gl texture binding
// for texture usage in frame statistics
// only needed for voodoo2
// any change in the LIGHTMAP_* defines here MUST be reflected in
// R_FindShader() in tr_bsp.c
// shader is for 2D rendering
// pre-lit triangle models
// outside of TR since it shouldn't be cleared during ref re-init
// These variables should live inside glConfig but can't because of
// compatibility issues to the original ID vms.  If you release a stand-alone
// game and your mod uses tr_types.h from this build you can safely move them
// to the glconfig_t struct.
//
// cvars
//
// number of desired stencil bits
// number of desired depth bits
// number of desired color bits, only relevant for fullscreen
// number of desired texture bits
// 0 = use framebuffer depth
// 16 = use 16-bit textures
// 32 = use 32-bit textures
// all else = error
// video mode
// overrides hardware gamma capabilities
// global enable/disable of OpenGL extensions
// these control use of specific extensions
/*
====================
RE_RegisterShaderNoMip

For menu graphics that should never be picmiped
====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterShaderNoMip(
    mut name: *const i8,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    if crate::stdlib::strlen(name) >= 64 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Shader name exceeds MAX_QPATH\n\x00" as *const u8 as *const i8,
        );
        return 0i32;
    }
    sh = R_FindShader(name, -(4), crate::src::qcommon::q_shared::qfalse);
    // we want to return 0 if the shader failed to
    // load for some reason, but R_FindShader should
    // still keep a name allocated for it, so if
    // something calls RE_RegisterShader again with
    // the same name, we don't try looking for it again
    if (*sh).defaultShader as u64 != 0 {
        return 0i32;
    }
    return (*sh).index;
}
/*
====================
R_GetShaderByHandle

When a handle is passed in by another module, this range checks
it and returns a valid (possibly default) shader_t to be used internally.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetShaderByHandle(
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) -> *mut crate::tr_local_h::shader_t {
    if hShader < 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_GetShaderByHandle: out of range hShader \'%d\'\n\x00" as *const u8 as *const i8,
            hShader,
        );
        return crate::src::renderergl1::tr_main::tr.defaultShader;
    }
    if hShader >= crate::src::renderergl1::tr_main::tr.numShaders {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_GetShaderByHandle: out of range hShader \'%d\'\n\x00" as *const u8 as *const i8,
            hShader,
        );
        return crate::src::renderergl1::tr_main::tr.defaultShader;
    }
    return crate::src::renderergl1::tr_main::tr.shaders[hShader as usize];
}
/*
===============
R_ShaderList_f

Dump information on all valid shaders to the console
A second parameter will cause it to print in sorted order
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_ShaderList_f() {
    let mut _i: i32 = 0;
    let mut count: i32 = 0;
    let mut shader_0: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"-----------------------\n\x00" as *const u8 as *const i8,
    );
    count = 0;

    for i in 0..crate::src::renderergl1::tr_main::tr.numShaders {
        if crate::src::renderergl1::tr_main::ri
            .Cmd_Argc
            .expect("non-null function pointer")()
            > 1
        {
            shader_0 = crate::src::renderergl1::tr_main::tr.sortedShaders[i as usize]
        } else {
            shader_0 = crate::src::renderergl1::tr_main::tr.shaders[i as usize]
        }

        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%i \x00" as *const u8 as *const i8,
            (*shader_0).numUnfoggedPasses,
        );

        if (*shader_0).lightmapIndex >= 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"L \x00" as *const u8 as *const i8,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"  \x00" as *const u8 as *const i8,
            );
        }

        if (*shader_0).multitextureEnv == 0x104 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"MT(a) \x00" as *const u8 as *const i8,
            );
        } else if (*shader_0).multitextureEnv == 0x2100 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"MT(m) \x00" as *const u8 as *const i8,
            );
        } else if (*shader_0).multitextureEnv == 0x2101 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"MT(d) \x00" as *const u8 as *const i8,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"      \x00" as *const u8 as *const i8,
            );
        }

        if (*shader_0).explicitlyDefined as u64 != 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"E \x00" as *const u8 as *const i8,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"  \x00" as *const u8 as *const i8,
            );
        }

        if (*shader_0).optimalStageIteratorFunc
            == Some(
                crate::src::renderergl1::tr_shade::RB_StageIteratorGeneric
                    as unsafe extern "C" fn() -> (),
            )
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"gen \x00" as *const u8 as *const i8,
            );
        } else if (*shader_0).optimalStageIteratorFunc
            == Some(
                crate::src::renderergl1::tr_sky::RB_StageIteratorSky
                    as unsafe extern "C" fn() -> (),
            )
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"sky \x00" as *const u8 as *const i8,
            );
        } else if (*shader_0).optimalStageIteratorFunc
            == Some(
                crate::src::renderergl1::tr_shade::RB_StageIteratorLightmappedMultitexture
                    as unsafe extern "C" fn() -> (),
            )
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"lmmt\x00" as *const u8 as *const i8,
            );
        } else if (*shader_0).optimalStageIteratorFunc
            == Some(
                crate::src::renderergl1::tr_shade::RB_StageIteratorVertexLitTexture
                    as unsafe extern "C" fn() -> (),
            )
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"vlt \x00" as *const u8 as *const i8,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"    \x00" as *const u8 as *const i8,
            );
        }

        if (*shader_0).defaultShader as u64 != 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b": %s (DEFAULTED)\n\x00" as *const u8 as *const i8,
                (*shader_0).name.as_mut_ptr(),
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b": %s\n\x00" as *const u8 as *const i8,
                (*shader_0).name.as_mut_ptr(),
            );
        }

        count += 1;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"%i total shaders\n\x00" as *const u8 as *const i8,
        count,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"------------------\n\x00" as *const u8 as *const i8,
    );
}

unsafe extern "C" fn ScanAndLoadShaderFiles() {
    let mut shaderFiles: *mut *mut i8 = 0 as *mut *mut i8;
    let mut buffers: [*mut i8; 4096] = [
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
        0 as *mut i8,
    ];
    let mut p: *mut i8 = 0 as *mut i8;
    let mut numShaderFiles: i32 = 0;
    let mut i: i32 = 0;
    let mut oldp: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    let mut hashMem: *mut i8 = 0 as *mut i8;
    let mut textEnd: *mut i8 = 0 as *mut i8;
    let mut shaderTextHashTableSizes: [i32; 2048] = [0; 2048];
    let mut hash: i32 = 0;
    let mut size: i32 = 0;
    let mut shaderName: [i8; 64] = [0; 64];
    let mut shaderLine: i32 = 0;
    let mut sum: isize = 0;
    let mut summand: isize = 0;
    // scan for shader files
    shaderFiles = crate::src::renderergl1::tr_main::ri
        .FS_ListFiles
        .expect("non-null function pointer")(
        b"scripts\x00" as *const u8 as *const i8,
        b".shader\x00" as *const u8 as *const i8,
        &mut numShaderFiles,
    );
    if shaderFiles.is_null() || numShaderFiles == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: no shader files found\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    if numShaderFiles > 4096 {
        numShaderFiles = 4096
    }
    // load and parse shader files
    i = 0;
    while i < numShaderFiles {
        let mut filename: [i8; 64] = [0; 64];
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"scripts/%s\x00" as *const u8 as *const i8,
            *shaderFiles.offset(i as isize),
        );
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"...loading \'%s\'\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        summand = crate::src::renderergl1::tr_main::ri
            .FS_ReadFile
            .expect("non-null function pointer")(
            filename.as_mut_ptr(),
            &mut *buffers.as_mut_ptr().offset(i as isize) as *mut *mut i8 as *mut *mut libc::c_void,
        );
        if buffers[i as usize].is_null() {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"Couldn\'t load %s\x00" as *const u8 as *const i8,
                filename.as_mut_ptr(),
            );
        }
        // Do a simple check on the shader structure in that file to make sure one bad shader file cannot fuck up all other shaders.
        p = buffers[i as usize];
        crate::src::qcommon::q_shared::COM_BeginParseSession(filename.as_mut_ptr());
        loop {
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                &mut p,
                crate::src::qcommon::q_shared::qtrue,
            );
            if *token == 0 {
                break;
            }
            crate::src::qcommon::q_shared::Q_strncpyz(
                shaderName.as_mut_ptr(),
                token,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            shaderLine = crate::src::qcommon::q_shared::COM_GetCurrentParseLine();
            token = crate::src::qcommon::q_shared::COM_ParseExt(
                &mut p,
                crate::src::qcommon::q_shared::qtrue,
            );
            if *token.offset(0) as i32 != '{' as i32 || *token.offset(1) as i32 != '\u{0}' as i32 {
                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING as
                                                                  i32,
                                                              b"WARNING: Ignoring shader file %s. Shader \"%s\" on line %d missing opening brace\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              filename.as_mut_ptr(),
                                                              shaderName.as_mut_ptr(),
                                                              shaderLine);
                if *token.offset(0) != 0 {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                        b" (found \"%s\" on line %d)\x00" as *const u8 as *const i8,
                        token,
                        crate::src::qcommon::q_shared::COM_GetCurrentParseLine(),
                    );
                }
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b".\n\x00" as *const u8 as *const i8,
                );
                crate::src::renderergl1::tr_main::ri
                    .FS_FreeFile
                    .expect("non-null function pointer")(
                    buffers[i as usize] as *mut libc::c_void
                );
                buffers[i as usize] = 0 as *mut i8;
                break;
            } else {
                if !(crate::src::qcommon::q_shared::SkipBracedSection(&mut p, 1) as u64 == 0) {
                    continue;
                }
                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING as
                                                                  i32,
                                                              b"WARNING: Ignoring shader file %s. Shader \"%s\" on line %d missing closing brace.\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              filename.as_mut_ptr(),
                                                              shaderName.as_mut_ptr(),
                                                              shaderLine);
                crate::src::renderergl1::tr_main::ri
                    .FS_FreeFile
                    .expect("non-null function pointer")(
                    buffers[i as usize] as *mut libc::c_void
                );
                buffers[i as usize] = 0 as *mut i8;
                break;
            }
        }
        if !buffers[i as usize].is_null() {
            sum += summand
        }
        i += 1
    }
    // build single large buffer
    s_shaderText = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (sum + (numShaderFiles * 2) as isize) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut i8;
    *s_shaderText.offset(0) = '\u{0}' as i8;
    textEnd = s_shaderText;
    // free in reverse order, so the temp files are all dumped
    i = numShaderFiles - 1;
    while i >= 0 {
        if !buffers[i as usize].is_null() {
            crate::stdlib::strcat(textEnd, buffers[i as usize]);
            crate::stdlib::strcat(textEnd, b"\n\x00" as *const u8 as *const i8);
            textEnd = textEnd.offset(crate::stdlib::strlen(textEnd) as isize);
            crate::src::renderergl1::tr_main::ri
                .FS_FreeFile
                .expect("non-null function pointer")(
                buffers[i as usize] as *mut libc::c_void
            );
        }
        i -= 1
    }
    crate::src::qcommon::q_shared::COM_Compress(s_shaderText);
    // free up memory
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFileList
        .expect("non-null function pointer")(shaderFiles);
    crate::stdlib::memset(
        shaderTextHashTableSizes.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[i32; 2048]>(),
    );
    size = 0;
    p = s_shaderText;
    loop
    // look for shader names
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            &mut p,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *token.offset(0) as i32 == 0 {
            break;
        }
        hash = generateHashValue(token, 2048) as i32;
        shaderTextHashTableSizes[hash as usize] += 1;
        size += 1;
        crate::src::qcommon::q_shared::SkipBracedSection(&mut p, 0);
    }
    size += 2048;
    hashMem = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (size as usize).wrapping_mul(::std::mem::size_of::<*mut i8>()) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut i8;
    i = 0;
    while i < 2048 {
        shaderTextHashTable[i as usize] = hashMem as *mut *mut i8;
        hashMem = hashMem.offset(
            ((shaderTextHashTableSizes[i as usize] + 1) as usize)
                .wrapping_mul(::std::mem::size_of::<*mut i8>()) as isize,
        );
        i += 1
    }
    crate::stdlib::memset(
        shaderTextHashTableSizes.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[i32; 2048]>(),
    );
    p = s_shaderText;
    loop
    // look for shader names
    {
        oldp = p;
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            &mut p,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *token.offset(0) as i32 == 0 {
            break;
        }
        hash = generateHashValue(token, 2048) as i32;
        let fresh0 = shaderTextHashTableSizes[hash as usize];
        shaderTextHashTableSizes[hash as usize] = shaderTextHashTableSizes[hash as usize] + 1;
        let ref mut fresh1 = *shaderTextHashTable[hash as usize].offset(fresh0 as isize);
        *fresh1 = oldp;
        crate::src::qcommon::q_shared::SkipBracedSection(&mut p, 0);
    }
}
/*
====================
CreateInternalShaders
====================
*/

unsafe extern "C" fn CreateInternalShaders() {
    crate::src::renderergl1::tr_main::tr.numShaders = 0;
    // init the default shader
    InitShader(b"<default>\x00" as *const u8 as *const i8, -(1));
    stages[0].bundle[0].image[0] = crate::src::renderergl1::tr_main::tr.defaultImage;
    stages[0].active = crate::src::qcommon::q_shared::qtrue;
    stages[0].stateBits = 0x100;
    crate::src::renderergl1::tr_main::tr.defaultShader = FinishShader();
    // shadow shader is just a marker
    crate::src::qcommon::q_shared::Q_strncpyz(
        shader.name.as_mut_ptr(),
        b"<stencil shadow>\x00" as *const u8 as *const i8,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    shader.sort = crate::tr_local_h::SS_STENCIL_SHADOW as i32 as f32;
    crate::src::renderergl1::tr_main::tr.shadowShader = FinishShader();
}

unsafe extern "C" fn CreateExternalShaders() {
    crate::src::renderergl1::tr_main::tr.projectionShadowShader = R_FindShader(
        b"projectionShadow\x00" as *const u8 as *const i8,
        -(1),
        crate::src::qcommon::q_shared::qtrue,
    );
    crate::src::renderergl1::tr_main::tr.flareShader = R_FindShader(
        b"flareShader\x00" as *const u8 as *const i8,
        -(1),
        crate::src::qcommon::q_shared::qtrue,
    );
    // Hack to make fogging work correctly on flares. Fog colors are calculated
    // in tr_flare.c already.
    if (*crate::src::renderergl1::tr_main::tr.flareShader).defaultShader as u64 == 0 {
        let mut index: i32 = 0;
        index = 0;
        while index < (*crate::src::renderergl1::tr_main::tr.flareShader).numUnfoggedPasses {
            (*(*crate::src::renderergl1::tr_main::tr.flareShader).stages[index as usize])
                .adjustColorsForFog = crate::tr_local_h::ACFF_NONE;
            (*(*crate::src::renderergl1::tr_main::tr.flareShader).stages[index as usize])
                .stateBits |= 0x10000;
            index += 1
        }
    }
    crate::src::renderergl1::tr_main::tr.sunShader = R_FindShader(
        b"sun\x00" as *const u8 as *const i8,
        -(1),
        crate::src::qcommon::q_shared::qtrue,
    );
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
// 14 bits
// can't be increased without changing bit packing for drawsurfs
// see QSORT_SHADERNUM_SHIFT
// range from 0.0 to 1.0, should be color normalized
// origin in local coordinate system
// texture detail is lost tho when the lightmap is dark
// a trRefEntity_t has all the information passed in by
// the client game, as well as some locally derived info
// compensate for non-normalized axis
// true for bmodels that touch a dlight
// normalized direction towards light
// color normalized to 0-255
// 32 bit rgba packed
// in world coordinates
// orientation in world
// viewParms->or.origin in local coordinates
//===============================================================================
// mirrors, portals, viewscreens
// sky box
// opaque
// scorch marks, etc.
// ladders, grates, grills that may have small blended edges
// in addition to alpha test
// for items that should be drawn in front of the water plane
// regular transparency and filters
// generally only used for additive type effects
// gun smoke puffs
// blood blobs
// tr.identityLight
// always (1,1,1,1)
// grabbed from entity's modulate field
// grabbed from 1 - entity.modulate
// tess.vertexColors
// tess.vertexColors * tr.identityLight
// programmatically generated
// standard fog
// fixed color
// clear to 0,0
// S and T from world coordinates
// vertex coordinate modification type
// used for TMOD_TURBULENT and TMOD_STRETCH
// used for TMOD_TRANSFORM
// s' = s * m[0][0] + t * m[1][0] + trans[0]
// t' = s * m[0][1] + t * m[0][1] + trans[1]
// used for TMOD_SCALE
// s *= scale[0]
// t *= scale[1]
// used for TMOD_SCROLL
// s' = s + scroll[0] * time
// t' = t + scroll[1] * time
// + = clockwise
// - = counterclockwise
// for CGEN_CONST and AGEN_CONST
// GLS_xxxx mask
// surface is translucent and will just be adjusted properly
// surface is opaque but possibly alpha tested
// surface is trnaslucent, but still needs a fog pass (fog surface)
// game path, including extension
// for a shader to match, both name and lightmapIndex must match
// this shader == tr.shaders[index]
// this shader == tr.sortedShaders[sortedIndex]
// lower numbered shaders draw before higher numbered
// we want to return index 0 if the shader failed to
// load for some reason, but R_FindShader should
// still keep a name allocated for it, so if
// something calls RE_RegisterShader again with
// the same name, we don't try looking for it again
// found in a .shader file
// if explicitlyDefined, this will have SURF_* flags
// merge across entites optimizable (smoke, blood)
// distance to fog out at
// 0, GL_MODULATE, GL_ADD (FIXME: put in stage)
// CT_FRONT_SIDED, CT_BACK_SIDED, or CT_TWO_SIDED
// set for decals and other items that must be offset
// for console fonts, 2D elements, etc.
// for images that must always be full resolution
// draw a blended pass, possibly with depth test equals
// not all shaders will need all data to be gathered
// time this shader is clamped to
// current time offset for this shader
// current shader this one is remapped too
// trRefdef_t holds everything that comes in refdef_t,
// as well as the locally generated scene information
// transformation matrix
// time in milliseconds for shader effects and other time dependent rendering issues
// RDF_NOWORLDMODEL, etc
// 1 bits will prevent the associated area from rendering at all
// qtrue if areamask changed since last scene
// tr.refdef.time / 1000.0
// text messages for deform text shaders
//=================================================================================
// max surfaces per-skin
// This is an arbitry limit. Vanilla Q3 only supported 32 surfaces in skins but failed to
// enforce the maximum limit when reading skin files. It was possile to use more than 32
// surfaces which accessed out of bounds memory past end of skin->surfaces hunk block.
// skins allow models to be retextured without modifying the model file
// game path, including extension
// dynamically allocated array of surfaces
// in packed byte format
// texture coordinate vector scales
// for clipping distance in fog when outside
// may be different than or.origin for portals
// true if this view is through a portal
// the portal is a mirror, invert the face culling
// copied from tr.frameSceneNum
// copied from tr.frameCount
// clip anything behind this if mirroring
/*
==============================================================================

SURFACES

==============================================================================
*/
// any changes in surfaceType must be mirrored in rb_surfaceTable[]
// ignore
// beams, rails, lightning, etc that can be determined by entity
// ensures that sizeof( surfaceType_t ) == sizeof( int )
// bit combination for fast compares
// any of surface*_t
// max dimensions of a patch mesh in map file
// max dimensions of a grid mesh in memory
// when cgame directly specifies a polygon, it becomes a srfPoly_t
// as soon as it is called
// dynamic lighting information
// culling information
// lod information, which may be different
// than the culling information to allow for
// groups of curves that LOD as a unit
// vertexes
// variable sized
// dynamic lighting information
// triangle definitions (no normals at points)
// variable sized
// there is a variable length list of indices here also
// misc_models in maps are turned into direct geometry by q3map
// dynamic lighting information
// culling information (FIXME: use this!)
// triangle definitions
// inter-quake-model
// vertex arrays
// [num_vertexes] indexes into influenceBlendVertexes
// unique list of vertex blend indexes/weights for faster CPU vertex skinning
// [num_influences]
// [num_influences]
// depending upon the exporter, blend indices and weights might be int/float
// as opposed to the recommended byte/byte, for example Noesis exports
// int/float whereas the official IQM tool exports byte/byte
// IQM_UBYTE or IQM_FLOAT
// inter-quake-model surface
/*
==============================================================================

BRUSH MODELS

==============================================================================
*/
//
// in memory representation
//
// if == tr.viewCount, already added
// any of srf*_t
// common with leaf and node
// -1 for nodes, to differentiate from leafs
// node needs to be traversed if current
// for bounding box culling
// node specific
// leaf specific
// for culling
// ie: maps/tim_dm2.bsp
// ie: tim_dm2
// includes leafs
// may be passed in by CM_LoadMap to save space
// clusterBytes of 0xff
//======================================================================
// model = tr.models[model->index]
// just for listing purposes
// only if type == MOD_BRUSH
// only if type == MOD_MESH
// only if type == (MOD_MDR | MOD_IQM)
//====================================================
/*

the drawsurf sort data is packed into a single 32 bit value so it can be
compared quickly during the qsorting process

the bits are allocated as follows:

0 - 1	: dlightmap index
//2		: used to be clipped flag REMOVED - 03.21.00 rad
2 - 6	: fog index
11 - 20	: entity index
21 - 31	: sorted shader index

    TTimo - 1.32
0-1   : dlightmap index
2-6   : fog index
7-16  : entity index
17-30 : sorted shader index
*/
/*
** performanceCounters_t
*/
// the renderer front end should never modify glstate_t
// total msec for backend run
// all state modified by the back end is separated
// from the front end state
// flag for drawing sun
// if qtrue, drawstretchpic doesn't need to change modes
// shader needs to be finished
// currentEntity will point at this when doing 2D rendering
/*
** trGlobals_t
**
** Most renderer globals are defined here.
** backend functions should never modify any of these fields,
** but may read fields that aren't dynamically modified
** by the frontend.
*/
// cleared at shutdown, set at beginRegistration
// incremented every time a new vis cluster is entered
// incremented every frame
// incremented every scene
// incremented every view (twice a scene if portaled)
// and every R_MarkFragments call
// zeroed at RE_BeginFrame
// from RE_SetWorldVisData, shared with CM_Load
// inverse-quare highlight for projective adding
// full of 0xff
// full of tr.identityLightByte
// point currentEntity at this when rendering world
// currentEntityNum << QSORT_REFENTITYNUM_SHIFT
// 1.0 / ( 1 << overbrightBits )
// identityLight * 255
// r_overbrightBits->integer, but set to 0 if no hw gamma
// for current entity
// from the sky shader for this level
// not in pc due to clearing issue
//
// put large tables at the end, so most elements will be
// within the +/32K indexed range on risc processors
//
// shader indexes from other modules will be looked up in tr.shaders[]
// shader indexes from drawsurfs will be looked up in sortedShaders[]
// lower indexed sortedShaders must be rendered first (opaque surfaces before translucent)
// outside of TR since it shouldn't be cleared during ref re-init
//
// cvars
//
// coefficient for the flare intensity falloff function.
// used for debugging anything
// used for verbose debug spew
// allows us to ignore our Tess fast paths
// near Z clip plane
// z distance of projection plane
// separation of cameras for stereo rendering
// enables stencil buffer overdraw measurement
// push/pull LOD transitions
// "0" = based on compiled vertex array existence
// "1" = glDrawElemet tristrips
// "2" = glDrawElements triangles
// "-1" = no drawing
// controls whether in game video should be draw
// controls whether sky should be cleared or drawn
// controls drawing of sun quad
// dynamic lights enabled/disabled
// dlight non-facing surfaces for continuity
// bypasses the ref rendering
// disable/enable entity rendering
// disable/enable world rendering
// various levels of information display
// enables/disables detail texturing stages
// disable/enable usage of PVS
// enables culling of planar surfaces with back side test
// optional display refresh option
// turns off binding to appropriate textures
// make most world faces use default shader
// development aid to see texture mip usage
// controls picmip values
// avoid lightmap pass
// render lightmaps only
// vertex lighting mode for better performance
// ui is running fullscreen
// number of frames to emit GL logs
// enables wireframe rendering of the world
// forces sky in front of all surfaces
// draws wireframe normals
// force screen clear every frame
// controls shadows: 0 = none, 1 = blur, 2 = stencil, 3 = black planar projection
// light flares
//====================================================================
// completely unclipped
// clipped by one or more planes
// completely outside the clipping planes
/*
** GL wrapper/helper functions
*/
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
//
// tr_shader.c
//
/*
==================
R_InitShaders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitShaders() {
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"Initializing Shaders\n\x00" as *const u8 as *const i8,
    );
    crate::stdlib::memset(
        hashTable.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[*mut crate::tr_local_h::shader_t; 1024]>(),
    );
    CreateInternalShaders();
    ScanAndLoadShaderFiles();
    CreateExternalShaders();
}
