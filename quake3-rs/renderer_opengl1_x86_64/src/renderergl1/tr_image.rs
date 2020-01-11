use ::libc;

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

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_GetExtension;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strlwr;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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

pub use crate::qgl_h::BindTextureproc;
pub use crate::qgl_h::DeleteTexturesproc;
pub use crate::qgl_h::GenTexturesproc;
pub use crate::qgl_h::TexImage2Dproc;
pub use crate::qgl_h::TexParameterfproc;
pub use crate::qgl_h::TexParameteriproc;
pub use crate::src::renderercommon::tr_image_bmp::R_LoadBMP;
pub use crate::src::renderercommon::tr_image_jpg::R_LoadJPG;
pub use crate::src::renderercommon::tr_image_pcx::R_LoadPCX;
pub use crate::src::renderercommon::tr_image_png::R_LoadPNG;
pub use crate::src::renderercommon::tr_image_tga::R_LoadTGA;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_SelectTexture;
pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_image::ctype_h::tolower;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::glState;
pub use crate::src::renderergl1::tr_init::maxAnisotropy;
pub use crate::src::renderergl1::tr_init::r_colorMipLevels;
pub use crate::src::renderergl1::tr_init::r_ext_max_anisotropy;
pub use crate::src::renderergl1::tr_init::r_gamma;
pub use crate::src::renderergl1::tr_init::r_greyscale;
pub use crate::src::renderergl1::tr_init::r_intensity;
pub use crate::src::renderergl1::tr_init::r_overBrightBits;
pub use crate::src::renderergl1::tr_init::r_picmip;
pub use crate::src::renderergl1::tr_init::r_roundImagesDown;
pub use crate::src::renderergl1::tr_init::r_simpleMipMaps;
pub use crate::src::renderergl1::tr_init::r_texturebits;
pub use crate::src::renderergl1::tr_init::textureFilterAnisotropic;
pub use crate::src::renderergl1::tr_init::GL_CheckErrors;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_shader::R_FindShader;
pub use crate::src::sdl::sdl_gamma::GLimp_SetGamma;
pub use crate::src::sdl::sdl_glimp::qglActiveTextureARB;
pub use crate::src::sdl::sdl_glimp::qglBindTexture;
pub use crate::src::sdl::sdl_glimp::qglDeleteTextures;
pub use crate::src::sdl::sdl_glimp::qglGenTextures;
pub use crate::src::sdl::sdl_glimp::qglTexImage2D;
pub use crate::src::sdl::sdl_glimp::qglTexParameterf;
pub use crate::src::sdl::sdl_glimp::qglTexParameteri;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
pub use crate::stdlib::GLint;
pub use crate::stdlib::GLsizei;
pub use crate::stdlib::GLuint;
pub use crate::stdlib::GLvoid;
pub use crate::stdlib::__ctype_tolower_loc;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::pow;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncmp;
use crate::stdlib::strstr;
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
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::colorGen_t;
pub use crate::tr_local_h::cullType_t;
pub use crate::tr_local_h::deformStage_t;
pub use crate::tr_local_h::deform_t;
pub use crate::tr_local_h::dlight_s;
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::fogParms_t;
pub use crate::tr_local_h::fogPass_t;
pub use crate::tr_local_h::fog_t;
pub use crate::tr_local_h::frontEndCounters_t;
pub use crate::tr_local_h::genFunc_t;
pub use crate::tr_local_h::glstate_t;
pub use crate::tr_local_h::mnode_s;
pub use crate::tr_local_h::mnode_t;
pub use crate::tr_local_h::model_s;
pub use crate::tr_local_h::model_t;
pub use crate::tr_local_h::modtype_t;
pub use crate::tr_local_h::msurface_s;
pub use crate::tr_local_h::msurface_t;
pub use crate::tr_local_h::orientationr_t;
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::surfaceType_t;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imageExtToLoaderMap_t {
    pub ext: *mut i8,
    pub ImageLoader:
        Option<unsafe extern "C" fn(_: *const i8, _: *mut *mut u8, _: *mut i32, _: *mut i32) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct textureMode_t {
    pub name: *mut i8,
    pub minimize: i32,
    pub maximize: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_108 {
    pub c: *mut i8,
    pub v: *mut libc::c_void,
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
// tr_image.c

static mut s_intensitytable: [crate::src::qcommon::q_shared::byte; 256] = [0; 256];

static mut s_gammatable: [u8; 256] = [0; 256];
#[no_mangle]

pub static mut gl_filter_min: i32 = 0x2701;
#[no_mangle]

pub static mut gl_filter_max: i32 = 0x2601;

static mut hashTable: [*mut crate::tr_common_h::image_t; 1024] =
    [0 as *mut crate::tr_common_h::image_t; 1024];
/*
** R_GammaCorrect
*/
#[no_mangle]

pub unsafe extern "C" fn R_GammaCorrect(
    mut buffer: *mut crate::src::qcommon::q_shared::byte,
    mut bufSize: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < bufSize {
        *buffer.offset(i as isize) = s_gammatable[*buffer.offset(i as isize) as usize];
        i += 1
    }
}
#[no_mangle]

pub static mut modes: [textureMode_t; 6] = [
    {
        let mut init = textureMode_t {
            name: b"GL_NEAREST\x00" as *const u8 as *mut i8,
            minimize: 0x2600,
            maximize: 0x2600,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_LINEAR\x00" as *const u8 as *mut i8,
            minimize: 0x2601,
            maximize: 0x2601,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_NEAREST_MIPMAP_NEAREST\x00" as *const u8 as *mut i8,
            minimize: 0x2700,
            maximize: 0x2600,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *mut i8,
            minimize: 0x2701,
            maximize: 0x2601,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_NEAREST_MIPMAP_LINEAR\x00" as *const u8 as *mut i8,
            minimize: 0x2702,
            maximize: 0x2600,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_LINEAR_MIPMAP_LINEAR\x00" as *const u8 as *mut i8,
            minimize: 0x2703,
            maximize: 0x2601,
        };
        init
    },
];
/*
================
return a hash value for the filename
================
*/

unsafe extern "C" fn generateHashValue(mut fname: *const i8) -> isize {
    let mut i: i32 = 0; // don't include extension
    let mut hash: isize = 0; // damn path names
    let mut letter: i8 = 0;
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
        hash += letter as isize * (i + 119) as isize;
        i += 1
    }
    hash &= (1024i32 - 1) as isize;
    return hash;
}
/*
===============
GL_TextureMode
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GL_TextureMode(mut string: *const i8) {
    let mut i: i32 = 0;
    let mut glt: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    i = 0;
    while i < 6 {
        if crate::src::qcommon::q_shared::Q_stricmp(modes[i as usize].name, string) == 0 {
            break;
        }
        i += 1
    }
    // hack to prevent trilinear from being set on voodoo,
    // because their driver freaks...
    if i == 5
        && crate::src::renderergl1::tr_init::glConfig.hardwareType
            == crate::tr_types_h::GLHW_3DFX_2D3D
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Refusing to set trilinear on a voodoo.\n\x00" as *const u8 as *const i8,
        );
        i = 3
    }
    if i == 6 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"bad filter name\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    gl_filter_min = modes[i as usize].minimize;
    gl_filter_max = modes[i as usize].maximize;
    // change all the existing mipmap texture objects
    i = 0;
    while i < crate::src::renderergl1::tr_main::tr.numImages {
        glt = crate::src::renderergl1::tr_main::tr.images[i as usize];
        if (*glt).flags & crate::tr_common_h::IMGFLAG_MIPMAP != 0 {
            crate::src::renderergl1::tr_backend::GL_Bind(glt);
            crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
                0xde1u32,
                0x2801,
                gl_filter_min as crate::stdlib::GLfloat,
            );
            crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
                0xde1u32,
                0x2800u32,
                gl_filter_max as crate::stdlib::GLfloat,
            );
        }
        i += 1
    }
}
/*
===============
R_SumOfUsedImages
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SumOfUsedImages() -> i32 {
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    total = 0;

    for i in 0..crate::src::renderergl1::tr_main::tr.numImages {
        if (*crate::src::renderergl1::tr_main::tr.images[i as usize]).frameUsed
            == crate::src::renderergl1::tr_main::tr.frameCount
        {
            total += (*crate::src::renderergl1::tr_main::tr.images[i as usize]).uploadWidth
                * (*crate::src::renderergl1::tr_main::tr.images[i as usize]).uploadHeight
        }
    }
    return total;
}
/*
===============
R_ImageList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_ImageList_f() {
    let mut i: i32 = 0;
    let mut estTotalSize: i32 = 0;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"\n      -w-- -h-- type  -size- --name-------\n\x00" as *const u8 as *const i8,
    );

    for i in 0..crate::src::renderergl1::tr_main::tr.numImages {
        let mut image: *mut crate::tr_common_h::image_t =
            crate::src::renderergl1::tr_main::tr.images[i as usize];

        let mut format: *mut i8 = b"???? \x00" as *const u8 as *mut i8;

        let mut sizeSuffix: *mut i8 = 0 as *mut i8;

        let mut estSize: i32 = 0;

        let mut displaySize: i32 = 0;

        estSize = (*image).uploadHeight * (*image).uploadWidth;

        match (*image).internalFormat {
            35917 => {
                format = b"sDXT1\x00" as *const u8 as *mut i8;
                // 64 bits per 16 pixels, so 4 bits per pixel
                estSize /= 2
            }
            35919 => format = b"sDXT5\x00" as *const u8 as *mut i8,
            36493 => format = b"sBPTC\x00" as *const u8 as *mut i8,
            35954 => format = b"LATC \x00" as *const u8 as *mut i8,
            33777 => {
                format = b"DXT1 \x00" as *const u8 as *mut i8;
                // 64 bits per 16 pixels, so 4 bits per pixel
                estSize /= 2
            }
            33779 => format = b"DXT5 \x00" as *const u8 as *mut i8,
            36492 => format = b"BPTC \x00" as *const u8 as *mut i8,
            33697 => {
                format = b"S3TC \x00" as *const u8 as *mut i8;
                // same as DXT1?
                estSize /= 2
            }
            32854 | 32856 | 6408 => {
                format = b"RGBA \x00" as *const u8 as *mut i8;
                // 4 bytes per pixel
                estSize *= 4
            }
            32832 | 6409 => format = b"L    \x00" as *const u8 as *mut i8,
            32848 | 32849 | 6407 => {
                format = b"RGB  \x00" as *const u8 as *mut i8;
                // 3 bytes per pixel?
                estSize *= 3
            }
            32837 | 6410 => {
                format = b"LA   \x00" as *const u8 as *mut i8;
                // 2 bytes per pixel?
                estSize *= 2
            }
            35904 | 35905 => {
                format = b"sRGB \x00" as *const u8 as *mut i8;
                // 3 bytes per pixel?
                estSize *= 3
            }
            35906 | 35907 => {
                format = b"sRGBA\x00" as *const u8 as *mut i8;
                // 4 bytes per pixel?
                estSize *= 4
            }
            35910 | 35911 => format = b"sL   \x00" as *const u8 as *mut i8,
            35908 | 35909 => {
                format = b"sLA  \x00" as *const u8 as *mut i8;
                // 2 byte per pixel?
                estSize *= 2
            }
            _ => {}
        }

        if (*image).flags & crate::tr_common_h::IMGFLAG_MIPMAP != 0 {
            estSize += estSize / 2
        }

        sizeSuffix = b"b \x00" as *const u8 as *mut i8;

        displaySize = estSize;

        if displaySize > 1024 {
            displaySize /= 1024;
            sizeSuffix = b"kb\x00" as *const u8 as *mut i8
        }

        if displaySize > 1024 {
            displaySize /= 1024;
            sizeSuffix = b"Mb\x00" as *const u8 as *mut i8
        }

        if displaySize > 1024 {
            displaySize /= 1024;
            sizeSuffix = b"Gb\x00" as *const u8 as *mut i8
        }

        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%4i: %4ix%4i %s %4i%s %s\n\x00" as *const u8 as *const i8,
            i,
            (*image).uploadWidth,
            (*image).uploadHeight,
            format,
            displaySize,
            sizeSuffix,
            (*image).imgName.as_mut_ptr(),
        );

        estTotalSize += estSize;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b" ---------\n\x00" as *const u8 as *const i8,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b" approx %i bytes\n\x00" as *const u8 as *const i8,
        estTotalSize,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b" %i total images\n\n\x00" as *const u8 as *const i8,
        crate::src::renderergl1::tr_main::tr.numImages,
    );
}
//=======================================================================
/*
================
ResampleTexture

Used to resample images in a more general than quartering fashion.

This will only be filtered properly if the resampled size
is greater than half the original size.

If a larger shrinking is needed, use the mipmap function
before or after.
================
*/

unsafe extern "C" fn ResampleTexture(
    mut in_0: *mut u32,
    mut inwidth: i32,
    mut inheight: i32,
    mut out: *mut u32,
    mut outwidth: i32,
    mut outheight: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut inrow: *mut u32 = 0 as *mut u32;
    let mut inrow2: *mut u32 = 0 as *mut u32;
    let mut frac: u32 = 0;
    let mut fracstep: u32 = 0;
    let mut p1: [u32; 2048] = [0; 2048];
    let mut p2: [u32; 2048] = [0; 2048];
    let mut pix1: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix2: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix3: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix4: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if outwidth > 2048 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ResampleTexture: max width\x00" as *const u8 as *const i8,
        );
    }
    fracstep = (inwidth * 0x10000i32 / outwidth) as u32;
    frac = fracstep >> 2;
    i = 0;
    while i < outwidth {
        p1[i as usize] = (4u32).wrapping_mul(frac >> 16);
        frac = frac.wrapping_add(fracstep);
        i += 1
    }
    frac = (3u32).wrapping_mul(fracstep >> 2);
    i = 0;
    while i < outwidth {
        p2[i as usize] = (4u32).wrapping_mul(frac >> 16);
        frac = frac.wrapping_add(fracstep);
        i += 1
    }
    i = 0;
    while i < outheight {
        inrow = in_0.offset(
            (inwidth * ((i as f64 + 0.25) * inheight as f64 / outheight as f64) as i32) as isize,
        );
        inrow2 = in_0.offset(
            (inwidth * ((i as f64 + 0.75) * inheight as f64 / outheight as f64) as i32) as isize,
        );

        for j in 0..outwidth {
            pix1 =
                (inrow as *mut crate::src::qcommon::q_shared::byte).offset(p1[j as usize] as isize);

            pix2 =
                (inrow as *mut crate::src::qcommon::q_shared::byte).offset(p2[j as usize] as isize);

            pix3 = (inrow2 as *mut crate::src::qcommon::q_shared::byte)
                .offset(p1[j as usize] as isize);

            pix4 = (inrow2 as *mut crate::src::qcommon::q_shared::byte)
                .offset(p2[j as usize] as isize);

            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte).offset(0) =
                (*pix1.offset(0) as i32
                    + *pix2.offset(0) as i32
                    + *pix3.offset(0) as i32
                    + *pix4.offset(0) as i32
                    >> 2) as crate::src::qcommon::q_shared::byte;

            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte).offset(1) =
                (*pix1.offset(1) as i32
                    + *pix2.offset(1) as i32
                    + *pix3.offset(1) as i32
                    + *pix4.offset(1) as i32
                    >> 2) as crate::src::qcommon::q_shared::byte;

            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte).offset(2) =
                (*pix1.offset(2) as i32
                    + *pix2.offset(2) as i32
                    + *pix3.offset(2) as i32
                    + *pix4.offset(2) as i32
                    >> 2) as crate::src::qcommon::q_shared::byte;

            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte).offset(3) =
                (*pix1.offset(3) as i32
                    + *pix2.offset(3) as i32
                    + *pix3.offset(3) as i32
                    + *pix4.offset(3) as i32
                    >> 2) as crate::src::qcommon::q_shared::byte;
        }
        i += 1;
        out = out.offset(outwidth as isize)
    }
}
/*
================
R_LightScaleTexture

Scale up the pixel values in a texture to increase the
lighting range
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LightScaleTexture(
    mut in_0: *mut u32,
    mut inwidth: i32,
    mut inheight: i32,
    mut only_gamma: crate::src::qcommon::q_shared::qboolean,
) {
    if only_gamma as u64 != 0 {
        if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 == 0 {
            let mut i: i32 = 0;
            let mut c: i32 = 0;
            let mut p: *mut crate::src::qcommon::q_shared::byte =
                0 as *mut crate::src::qcommon::q_shared::byte;
            p = in_0 as *mut crate::src::qcommon::q_shared::byte;
            c = inwidth * inheight;
            i = 0;
            while i < c {
                *p.offset(0) = s_gammatable[*p.offset(0) as usize];
                *p.offset(1) = s_gammatable[*p.offset(1) as usize];
                *p.offset(2) = s_gammatable[*p.offset(2) as usize];
                i += 1;
                p = p.offset(4)
            }
        }
    } else {
        let mut i_0: i32 = 0;
        let mut c_0: i32 = 0;
        let mut p_0: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        p_0 = in_0 as *mut crate::src::qcommon::q_shared::byte;
        c_0 = inwidth * inheight;
        if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 != 0 {
            i_0 = 0;
            while i_0 < c_0 {
                *p_0.offset(0) = s_intensitytable[*p_0.offset(0) as usize];
                *p_0.offset(1) = s_intensitytable[*p_0.offset(1) as usize];
                *p_0.offset(2) = s_intensitytable[*p_0.offset(2) as usize];
                i_0 += 1;
                p_0 = p_0.offset(4)
            }
        } else {
            i_0 = 0;
            while i_0 < c_0 {
                *p_0.offset(0) = s_gammatable[s_intensitytable[*p_0.offset(0) as usize] as usize];
                *p_0.offset(1) = s_gammatable[s_intensitytable[*p_0.offset(1) as usize] as usize];
                *p_0.offset(2) = s_gammatable[s_intensitytable[*p_0.offset(2) as usize] as usize];
                i_0 += 1;
                p_0 = p_0.offset(4)
            }
        }
    };
}
/*
================
R_MipMap2

Operates in place, quartering the size of the texture
Proper linear filter
================
*/

unsafe extern "C" fn R_MipMap2(mut in_0: *mut u32, mut inWidth: i32, mut inHeight: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut outpix: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut inWidthMask: i32 = 0;
    let mut inHeightMask: i32 = 0;
    let mut total: i32 = 0;
    let mut outWidth: i32 = 0;
    let mut outHeight: i32 = 0;
    let mut temp: *mut u32 = 0 as *mut u32;
    outWidth = inWidth >> 1;
    outHeight = inHeight >> 1;
    temp = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(outWidth * outHeight * 4) as *mut u32;
    inWidthMask = inWidth - 1;
    inHeightMask = inHeight - 1;

    for i in 0..outHeight {
        for j in 0..outWidth {
            outpix = temp.offset((i * outWidth) as isize).offset(j as isize)
                as *mut crate::src::qcommon::q_shared::byte;
            for k in 0..4 {
                total = 1 * *(&mut *in_0.offset(
                    ((i * 2 - 1 & inHeightMask) * inWidth + (j * 2 - 1 & inWidthMask)) as isize,
                ) as *mut u32
                    as *mut crate::src::qcommon::q_shared::byte)
                    .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 - 1 & inHeightMask) * inWidth + (j * 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 - 1 & inHeightMask) * inWidth + (j * 2 + 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 1 * *(&mut *in_0.offset(
                        ((i * 2 - 1 & inHeightMask) * inWidth + (j * 2 + 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 & inHeightMask) * inWidth + (j * 2 - 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 4 * *(&mut *in_0
                        .offset(((i * 2 & inHeightMask) * inWidth + (j * 2 & inWidthMask)) as isize)
                        as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 4 * *(&mut *in_0.offset(
                        ((i * 2 & inHeightMask) * inWidth + (j * 2 + 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 & inHeightMask) * inWidth + (j * 2 + 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 + 1 & inHeightMask) * inWidth + (j * 2 - 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 4 * *(&mut *in_0.offset(
                        ((i * 2 + 1 & inHeightMask) * inWidth + (j * 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 4 * *(&mut *in_0.offset(
                        ((i * 2 + 1 & inHeightMask) * inWidth + (j * 2 + 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 + 1 & inHeightMask) * inWidth + (j * 2 + 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 1 * *(&mut *in_0.offset(
                        ((i * 2 + 2 & inHeightMask) * inWidth + (j * 2 - 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 + 2 & inHeightMask) * inWidth + (j * 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 2 * *(&mut *in_0.offset(
                        ((i * 2 + 2 & inHeightMask) * inWidth + (j * 2 + 1 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32
                    + 1 * *(&mut *in_0.offset(
                        ((i * 2 + 2 & inHeightMask) * inWidth + (j * 2 + 2 & inWidthMask)) as isize,
                    ) as *mut u32
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as i32;

                *outpix.offset(k as isize) = (total / 36) as crate::src::qcommon::q_shared::byte;
            }
        }
    }
    crate::stdlib::memcpy(
        in_0 as *mut libc::c_void,
        temp as *const libc::c_void,
        (outWidth * outHeight * 4) as usize,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(temp as *mut libc::c_void);
}
/*
================
R_MipMap

Operates in place, quartering the size of the texture
================
*/

unsafe extern "C" fn R_MipMap(
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut width: i32,
    mut height: i32,
) {
    let mut i: i32 = 0; // get largest
    let mut j: i32 = 0;
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut row: i32 = 0;
    if (*crate::src::renderergl1::tr_init::r_simpleMipMaps).integer == 0 {
        R_MipMap2(in_0 as *mut u32, width, height);
        return;
    }
    if width == 1 && height == 1 {
        return;
    }
    row = width * 4;
    out = in_0;
    width >>= 1;
    height >>= 1;
    if width == 0 || height == 0 {
        width += height;
        i = 0;
        while i < width {
            *out.offset(0) = (*in_0.offset(0) as i32 + *in_0.offset(4) as i32 >> 1)
                as crate::src::qcommon::q_shared::byte;
            *out.offset(1) = (*in_0.offset(1) as i32 + *in_0.offset(5) as i32 >> 1)
                as crate::src::qcommon::q_shared::byte;
            *out.offset(2) = (*in_0.offset(2) as i32 + *in_0.offset(6) as i32 >> 1)
                as crate::src::qcommon::q_shared::byte;
            *out.offset(3) = (*in_0.offset(3) as i32 + *in_0.offset(7) as i32 >> 1)
                as crate::src::qcommon::q_shared::byte;
            i += 1;
            out = out.offset(4);
            in_0 = in_0.offset(8)
        }
        return;
    }
    i = 0;
    while i < height {
        j = 0;
        while j < width {
            *out.offset(0) = (*in_0.offset(0) as i32
                + *in_0.offset(4) as i32
                + *in_0.offset((row + 0) as isize) as i32
                + *in_0.offset((row + 4) as isize) as i32
                >> 2) as crate::src::qcommon::q_shared::byte;
            *out.offset(1) = (*in_0.offset(1) as i32
                + *in_0.offset(5) as i32
                + *in_0.offset((row + 1) as isize) as i32
                + *in_0.offset((row + 5) as isize) as i32
                >> 2) as crate::src::qcommon::q_shared::byte;
            *out.offset(2) = (*in_0.offset(2) as i32
                + *in_0.offset(6) as i32
                + *in_0.offset((row + 2) as isize) as i32
                + *in_0.offset((row + 6) as isize) as i32
                >> 2) as crate::src::qcommon::q_shared::byte;
            *out.offset(3) = (*in_0.offset(3) as i32
                + *in_0.offset(7) as i32
                + *in_0.offset((row + 3) as isize) as i32
                + *in_0.offset((row + 7) as isize) as i32
                >> 2) as crate::src::qcommon::q_shared::byte;
            j += 1;
            out = out.offset(4);
            in_0 = in_0.offset(8)
        }
        i += 1;
        in_0 = in_0.offset(row as isize)
    }
}
/*
==================
R_BlendOverTexture

Apply a color blend over a set of pixels
==================
*/

unsafe extern "C" fn R_BlendOverTexture(
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut pixelCount: i32,
    mut blend: *mut crate::src::qcommon::q_shared::byte,
) {
    let mut i: i32 = 0;
    let mut inverseAlpha: i32 = 0;
    let mut premult: [i32; 3] = [0; 3];
    inverseAlpha = 255 - *blend.offset(3) as i32;
    premult[0] = *blend.offset(0) as i32 * *blend.offset(3) as i32;
    premult[1] = *blend.offset(1) as i32 * *blend.offset(3) as i32;
    premult[2] = *blend.offset(2) as i32 * *blend.offset(3) as i32;
    i = 0;
    while i < pixelCount {
        *data.offset(0) = (*data.offset(0) as i32 * inverseAlpha + premult[0] >> 9)
            as crate::src::qcommon::q_shared::byte;
        *data.offset(1) = (*data.offset(1) as i32 * inverseAlpha + premult[1] >> 9)
            as crate::src::qcommon::q_shared::byte;
        *data.offset(2) = (*data.offset(2) as i32 * inverseAlpha + premult[2] >> 9)
            as crate::src::qcommon::q_shared::byte;
        i += 1;
        data = data.offset(4)
    }
}
#[no_mangle]

pub static mut mipBlendColors: [[crate::src::qcommon::q_shared::byte; 4]; 16] = [
    [0, 0, 0, 0],
    [255, 0, 0, 128],
    [0, 255, 0, 128],
    [0, 0, 255, 128],
    [255, 0, 0, 128],
    [0, 255, 0, 128],
    [0, 0, 255, 128],
    [255, 0, 0, 128],
    [0, 255, 0, 128],
    [0, 0, 255, 128],
    [255, 0, 0, 128],
    [0, 255, 0, 128],
    [0, 0, 255, 128],
    [255, 0, 0, 128],
    [0, 255, 0, 128],
    [0, 0, 255, 128],
];
/*
===============
Upload32

===============
*/

unsafe extern "C" fn Upload32(
    mut data: *mut u32,
    mut width: i32,
    mut height: i32,
    mut mipmap: crate::src::qcommon::q_shared::qboolean,
    mut picmip: crate::src::qcommon::q_shared::qboolean,
    mut lightMap: crate::src::qcommon::q_shared::qboolean,
    mut allowCompression: crate::src::qcommon::q_shared::qboolean,
    mut format: *mut i32,
    mut pUploadWidth: *mut i32,
    mut pUploadHeight: *mut i32,
) {
    let mut current_block: u64;
    let mut samples: i32 = 0;
    let mut scaledBuffer: *mut u32 = 0 as *mut u32;
    let mut resampledBuffer: *mut u32 = 0 as *mut u32;
    let mut scaled_width: i32 = 0;
    let mut scaled_height: i32 = 0;
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut scan: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut internalFormat: crate::stdlib::GLenum = 0x1907;
    let mut rMax: f32 = 0f32;
    let mut gMax: f32 = 0f32;
    let mut bMax: f32 = 0f32;
    //
    // convert to exact power of 2 sizes
    //
    scaled_width = 1;
    while scaled_width < width {
        scaled_width <<= 1
    }
    scaled_height = 1;
    while scaled_height < height {
        scaled_height <<= 1
    }
    if (*crate::src::renderergl1::tr_init::r_roundImagesDown).integer != 0 && scaled_width > width {
        scaled_width >>= 1
    }
    if (*crate::src::renderergl1::tr_init::r_roundImagesDown).integer != 0 && scaled_height > height
    {
        scaled_height >>= 1
    }
    if scaled_width != width || scaled_height != height {
        resampledBuffer = crate::src::renderergl1::tr_main::ri
            .Hunk_AllocateTempMemory
            .expect("non-null function pointer")(
            scaled_width * scaled_height * 4
        ) as *mut u32;
        ResampleTexture(
            data,
            width,
            height,
            resampledBuffer,
            scaled_width,
            scaled_height,
        );
        data = resampledBuffer;
        width = scaled_width;
        height = scaled_height
    }
    //
    // perform optional picmip operation
    //
    if picmip as u64 != 0 {
        scaled_width >>= (*crate::src::renderergl1::tr_init::r_picmip).integer;
        scaled_height >>= (*crate::src::renderergl1::tr_init::r_picmip).integer
    }
    //
    // clamp to minimum size
    //
    if scaled_width < 1 {
        scaled_width = 1
    }
    if scaled_height < 1 {
        scaled_height = 1
    }
    //
    // clamp to the current upper OpenGL limit
    // scale both axis down equally so we don't have to
    // deal with a half mip resampling
    //
    while scaled_width > crate::src::renderergl1::tr_init::glConfig.maxTextureSize
        || scaled_height > crate::src::renderergl1::tr_init::glConfig.maxTextureSize
    {
        scaled_width >>= 1;
        scaled_height >>= 1
    }
    scaledBuffer = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(
        (::std::mem::size_of::<u32>())
            .wrapping_mul(scaled_width as usize)
            .wrapping_mul(scaled_height as usize) as i32,
    ) as *mut u32;
    //
    // scan the texture for each channel's max values
    // and verify if the alpha channel is being used or not
    //
    c = width * height;
    scan = data as *mut crate::src::qcommon::q_shared::byte;
    samples = 3;
    if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
        i = 0;
        while i < c {
            let mut luma: crate::src::qcommon::q_shared::byte = (0.2126
                * *scan.offset((i * 4) as isize) as i32 as f32
                + 0.7152 * *scan.offset((i * 4 + 1) as isize) as i32 as f32
                + 0.0722 * *scan.offset((i * 4 + 2) as isize) as i32 as f32)
                as crate::src::qcommon::q_shared::byte;
            *scan.offset((i * 4) as isize) = luma;
            *scan.offset((i * 4 + 1) as isize) = luma;
            *scan.offset((i * 4 + 2) as isize) = luma;
            i += 1
        }
    } else if (*crate::src::renderergl1::tr_init::r_greyscale).value != 0. {
        i = 0;
        while i < c {
            let mut luma_0: f32 = 0.2126 * *scan.offset((i * 4) as isize) as i32 as f32
                + 0.7152 * *scan.offset((i * 4 + 1) as isize) as i32 as f32
                + 0.0722 * *scan.offset((i * 4 + 2) as isize) as i32 as f32;
            *scan.offset((i * 4) as isize) = (*scan.offset((i * 4) as isize) as i32 as f32
                * (1.0 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                + luma_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                as crate::src::qcommon::q_shared::byte;
            *scan.offset((i * 4 + 1) as isize) = (*scan.offset((i * 4 + 1) as isize) as i32 as f32
                * (1.0 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                + luma_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                as crate::src::qcommon::q_shared::byte;
            *scan.offset((i * 4 + 2) as isize) = (*scan.offset((i * 4 + 2) as isize) as i32 as f32
                * (1.0 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                + luma_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                as crate::src::qcommon::q_shared::byte;
            i += 1
        }
    }
    if lightMap as u64 != 0 {
        if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
            internalFormat = 0x1909
        } else {
            internalFormat = 0x1907
        }
    } else {
        i = 0;
        while i < c {
            if *scan.offset((i * 4 + 0) as isize) as i32 as f32 > rMax {
                rMax = *scan.offset((i * 4 + 0) as isize) as f32
            }
            if *scan.offset((i * 4 + 1) as isize) as i32 as f32 > gMax {
                gMax = *scan.offset((i * 4 + 1) as isize) as f32
            }
            if *scan.offset((i * 4 + 2) as isize) as i32 as f32 > bMax {
                bMax = *scan.offset((i * 4 + 2) as isize) as f32
            }
            if *scan.offset((i * 4 + 3) as isize) as i32 != 255 {
                samples = 4;
                break;
            } else {
                i += 1
            }
        }
        // select proper internal format
        if samples == 3 {
            if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
                if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 16
                    || (*crate::src::renderergl1::tr_init::r_texturebits).integer == 32
                {
                    internalFormat = 0x8040
                } else {
                    internalFormat = 0x1909
                }
            } else if allowCompression != 0
                && crate::src::renderergl1::tr_init::glConfig.textureCompression
                    == crate::tr_types_h::TC_S3TC_ARB
            {
                internalFormat = 0x83f1
            } else if allowCompression != 0
                && crate::src::renderergl1::tr_init::glConfig.textureCompression
                    == crate::tr_types_h::TC_S3TC
            {
                internalFormat = 0x83a1
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 16 {
                internalFormat = 0x8050
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 32 {
                internalFormat = 0x8051
            } else {
                internalFormat = 0x1907
            }
        } else if samples == 4 {
            if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
                if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 16
                    || (*crate::src::renderergl1::tr_init::r_texturebits).integer == 32
                {
                    internalFormat = 0x8045
                } else {
                    internalFormat = 0x190a
                }
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 16 {
                internalFormat = 0x8056
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 32 {
                internalFormat = 0x8058
            } else {
                internalFormat = 0x1908
            }
        }
    }
    // copy or resample data as appropriate for first MIP level
    if scaled_width == width && scaled_height == height {
        if mipmap as u64 == 0 {
            crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
                0xde1u32,
                0,
                internalFormat as crate::stdlib::GLint,
                scaled_width,
                scaled_height,
                0,
                0x1908,
                0x1401,
                data as *const libc::c_void,
            );
            *pUploadWidth = scaled_width;
            *pUploadHeight = scaled_height;
            *format = internalFormat as i32;
            current_block = 1652156287349356005;
        } else {
            crate::stdlib::memcpy(
                scaledBuffer as *mut libc::c_void,
                data as *const libc::c_void,
                (width * height * 4i32) as usize,
            );
            current_block = 7244994750255146185;
        }
    } else {
        // use the normal mip-mapping function to go down from here
        while width > scaled_width || height > scaled_height {
            R_MipMap(
                data as *mut crate::src::qcommon::q_shared::byte,
                width,
                height,
            );
            width >>= 1;
            height >>= 1;
            if width < 1 {
                width = 1
            }
            if height < 1 {
                height = 1
            }
        }
        crate::stdlib::memcpy(
            scaledBuffer as *mut libc::c_void,
            data as *const libc::c_void,
            (width * height * 4i32) as usize,
        );
        current_block = 7244994750255146185;
    }
    match current_block {
        7244994750255146185 => {
            R_LightScaleTexture(
                scaledBuffer,
                scaled_width,
                scaled_height,
                (mipmap as u64 == 0) as crate::src::qcommon::q_shared::qboolean,
            );
            *pUploadWidth = scaled_width;
            *pUploadHeight = scaled_height;
            *format = internalFormat as i32;
            crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
                0xde1u32,
                0,
                internalFormat as crate::stdlib::GLint,
                scaled_width,
                scaled_height,
                0,
                0x1908,
                0x1401,
                scaledBuffer as *const libc::c_void,
            );
            if mipmap as u64 != 0 {
                let mut miplevel: i32 = 0;
                miplevel = 0;
                while scaled_width > 1 || scaled_height > 1 {
                    R_MipMap(
                        scaledBuffer as *mut crate::src::qcommon::q_shared::byte,
                        scaled_width,
                        scaled_height,
                    );
                    scaled_width >>= 1;
                    scaled_height >>= 1;
                    if scaled_width < 1 {
                        scaled_width = 1
                    }
                    if scaled_height < 1 {
                        scaled_height = 1
                    }
                    miplevel += 1;
                    if (*crate::src::renderergl1::tr_init::r_colorMipLevels).integer != 0 {
                        R_BlendOverTexture(
                            scaledBuffer as *mut crate::src::qcommon::q_shared::byte,
                            scaled_width * scaled_height,
                            mipBlendColors[miplevel as usize].as_mut_ptr(),
                        );
                    }
                    crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
                        0xde1u32,
                        miplevel,
                        internalFormat as crate::stdlib::GLint,
                        scaled_width,
                        scaled_height,
                        0,
                        0x1908,
                        0x1401,
                        scaledBuffer as *const libc::c_void,
                    );
                }
            }
        }
        _ => {}
    }
    if mipmap as u64 != 0 {
        if crate::src::renderergl1::tr_init::textureFilterAnisotropic as u64 != 0 {
            crate::src::sdl::sdl_glimp::qglTexParameteri.expect("non-null function pointer")(
                0xde1u32,
                0x84feu32,
                crate::src::qcommon::q_shared::Com_Clamp(
                    1f32,
                    crate::src::renderergl1::tr_init::maxAnisotropy as f32,
                    (*crate::src::renderergl1::tr_init::r_ext_max_anisotropy).integer as f32,
                ) as crate::stdlib::GLint,
            );
        }
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1u32,
            0x2801,
            gl_filter_min as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1u32,
            0x2800u32,
            gl_filter_max as crate::stdlib::GLfloat,
        );
    } else {
        if crate::src::renderergl1::tr_init::textureFilterAnisotropic as u64 != 0 {
            crate::src::sdl::sdl_glimp::qglTexParameteri.expect("non-null function pointer")(
                0xde1u32, 0x84feu32, 1i32,
            );
        }
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1u32, 0x2801, 9729f32,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1u32, 0x2800u32, 9729f32,
        );
    }
    crate::src::renderergl1::tr_init::GL_CheckErrors();
    if !scaledBuffer.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Hunk_FreeTempMemory
            .expect("non-null function pointer")(scaledBuffer as *mut libc::c_void);
    }
    if !resampledBuffer.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Hunk_FreeTempMemory
            .expect("non-null function pointer")(resampledBuffer as *mut libc::c_void);
    };
}
/*
================
R_CreateImage

This is the only way any image_t are created
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_CreateImage(
    mut name: *const i8,
    mut pic: *mut crate::src::qcommon::q_shared::byte,
    mut width: i32,
    mut height: i32,
    mut type_0: crate::tr_common_h::imgType_t,
    mut flags: crate::tr_common_h::imgFlags_t,
    mut internalFormat: i32,
) -> *mut crate::tr_common_h::image_t {
    let mut image: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    let mut isLightmap: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut hash: isize = 0;
    let mut glWrapClampMode: i32 = 0;
    if crate::stdlib::strlen(name) >= 64 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"R_CreateImage: \"%s\" is too long\x00" as *const u8 as *const i8,
            name,
        );
    }
    if crate::stdlib::strncmp(name, b"*lightmap\x00" as *const u8 as *const i8, 9) == 0 {
        isLightmap = crate::src::qcommon::q_shared::qtrue
    }
    if crate::src::renderergl1::tr_main::tr.numImages == 2048 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"R_CreateImage: MAX_DRAWIMAGES hit\x00" as *const u8 as *const i8,
        );
    }
    crate::src::renderergl1::tr_main::tr.images
        [crate::src::renderergl1::tr_main::tr.numImages as usize] =
        crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ::std::mem::size_of::<crate::tr_common_h::image_t>() as i32,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_common_h::image_t;
    image = crate::src::renderergl1::tr_main::tr.images
        [crate::src::renderergl1::tr_main::tr.numImages as usize];
    crate::src::sdl::sdl_glimp::qglGenTextures.expect("non-null function pointer")(
        1,
        &mut (*image).texnum,
    );
    crate::src::renderergl1::tr_main::tr.numImages += 1;
    (*image).type_0 = type_0;
    (*image).flags = flags;
    crate::stdlib::strcpy((*image).imgName.as_mut_ptr(), name);
    (*image).width = width;
    (*image).height = height;
    if flags & crate::tr_common_h::IMGFLAG_CLAMPTOEDGE != 0 {
        glWrapClampMode = 0x812f
    } else {
        glWrapClampMode = 0x2901
    }
    // lightmaps are always allocated on TMU 1
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() && isLightmap != 0 {
        (*image).TMU = 1
    } else {
        (*image).TMU = 0
    }
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() {
        crate::src::renderergl1::tr_backend::GL_SelectTexture((*image).TMU);
    }
    crate::src::renderergl1::tr_backend::GL_Bind(image);
    Upload32(
        pic as *mut u32,
        (*image).width,
        (*image).height,
        (*image).flags & crate::tr_common_h::IMGFLAG_MIPMAP,
        (*image).flags & crate::tr_common_h::IMGFLAG_PICMIP,
        isLightmap,
        ((*image).flags & crate::tr_common_h::IMGFLAG_NO_COMPRESSION == 0)
            as crate::src::qcommon::q_shared::qboolean,
        &mut (*image).internalFormat,
        &mut (*image).uploadWidth,
        &mut (*image).uploadHeight,
    );
    crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
        0xde1u32,
        0x2802,
        glWrapClampMode as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
        0xde1u32,
        0x2803,
        glWrapClampMode as crate::stdlib::GLfloat,
    );
    crate::src::renderergl1::tr_init::glState.currenttextures
        [crate::src::renderergl1::tr_init::glState.currenttmu as usize] = 0;
    crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(0xde1u32, 0);
    if (*image).TMU == 1 {
        crate::src::renderergl1::tr_backend::GL_SelectTexture(0i32);
    }
    hash = generateHashValue(name);
    (*image).next = hashTable[hash as usize];
    hashTable[hash as usize] = image;
    return image;
}
// Note that the ordering indicates the order of preference used
// when there are multiple images of different formats available

static mut imageLoaders: [imageExtToLoaderMap_t; 6] = unsafe {
    [
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"tga\x00" as *const u8 as *mut i8,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_tga::R_LoadTGA
                        as unsafe extern "C" fn(
                            _: *const i8,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut i32,
                            _: *mut i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"jpg\x00" as *const u8 as *mut i8,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_jpg::R_LoadJPG
                        as unsafe extern "C" fn(
                            _: *const i8,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut i32,
                            _: *mut i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"jpeg\x00" as *const u8 as *mut i8,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_jpg::R_LoadJPG
                        as unsafe extern "C" fn(
                            _: *const i8,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut i32,
                            _: *mut i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"png\x00" as *const u8 as *mut i8,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_png::R_LoadPNG
                        as unsafe extern "C" fn(
                            _: *const i8,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut i32,
                            _: *mut i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"pcx\x00" as *const u8 as *mut i8,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_pcx::R_LoadPCX
                        as unsafe extern "C" fn(
                            _: *const i8,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut i32,
                            _: *mut i32,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"bmp\x00" as *const u8 as *mut i8,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_bmp::R_LoadBMP
                        as unsafe extern "C" fn(
                            _: *const i8,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut i32,
                            _: *mut i32,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
// Initialized in run_static_initializers

static mut numImageLoaders: i32 = 0;
/*
=================
R_LoadImage

Loads any of the supported image types into a canonical
32 bit format.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadImage(
    mut name: *const i8,
    mut pic: *mut *mut crate::src::qcommon::q_shared::byte,
    mut width: *mut i32,
    mut height: *mut i32,
) {
    let mut orgNameFailed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut orgLoader: i32 = -(1);
    let mut i: i32 = 0;
    let mut localName: [i8; 64] = [0; 64];
    let mut ext: *const i8 = 0 as *const i8;
    let mut altName: *mut i8 = 0 as *mut i8;
    *pic = 0 as *mut crate::src::qcommon::q_shared::byte;
    *width = 0;
    *height = 0;
    crate::src::qcommon::q_shared::Q_strncpyz(localName.as_mut_ptr(), name, 64);
    ext = crate::src::qcommon::q_shared::COM_GetExtension(localName.as_mut_ptr());
    if *ext != 0 {
        // Look for the correct loader and use it
        i = 0;
        while i < numImageLoaders {
            if crate::src::qcommon::q_shared::Q_stricmp(ext, imageLoaders[i as usize].ext) == 0 {
                // Load
                imageLoaders[i as usize]
                    .ImageLoader
                    .expect("non-null function pointer")(
                    localName.as_mut_ptr(),
                    pic,
                    width,
                    height,
                );
                break;
            } else {
                i += 1
            }
        }
        // A loader was found
        if i < numImageLoaders {
            if (*pic).is_null() {
                // Loader failed, most likely because the file isn't there;
                // try again without the extension
                orgNameFailed = crate::src::qcommon::q_shared::qtrue;
                orgLoader = i;
                crate::src::qcommon::q_shared::COM_StripExtension(
                    name,
                    localName.as_mut_ptr(),
                    64i32,
                );
            } else {
                // Something loaded
                return;
            }
        }
    }
    // Try and find a suitable match using all
    // the image formats supported
    i = 0;
    while i < numImageLoaders {
        if !(i == orgLoader) {
            altName = crate::src::qcommon::q_shared::va(
                b"%s.%s\x00" as *const u8 as *mut i8,
                localName.as_mut_ptr(),
                imageLoaders[i as usize].ext,
            );
            // Load
            imageLoaders[i as usize]
                .ImageLoader
                .expect("non-null function pointer")(altName, pic, width, height);
            if !(*pic).is_null() {
                if orgNameFailed as u64 != 0 {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                        b"WARNING: %s not present, using %s instead\n\x00" as *const u8
                            as *const i8,
                        name,
                        altName,
                    );
                }
                break;
            }
        }
        i += 1
    }
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
===============
R_FindImageFile

Finds or loads the given image.
Returns NULL if it fails, not a default image.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn R_FindImageFile(
    mut name: *const i8,
    mut type_0: crate::tr_common_h::imgType_t,
    mut flags: crate::tr_common_h::imgFlags_t,
) -> *mut crate::tr_common_h::image_t {
    let mut image: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut pic: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut hash: isize = 0;
    if name.is_null() {
        return 0 as *mut crate::tr_common_h::image_t;
    }
    hash = generateHashValue(name);
    //
    // see if the image is already loaded
    //
    image = hashTable[hash as usize];
    while !image.is_null() {
        if crate::stdlib::strcmp(name, (*image).imgName.as_mut_ptr()) == 0 {
            // the white image can be used with any set of parms, but other mismatches are errors
            if crate::stdlib::strcmp(name, b"*white\x00" as *const u8 as *const i8) != 0 {
                if (*image).flags != flags {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                        b"WARNING: reused image %s with mixed flags (%i vs %i)\n\x00" as *const u8
                            as *const i8,
                        name,
                        (*image).flags,
                        flags,
                    );
                }
            }
            return image;
        }
        image = (*image).next
    }
    //
    // load the pic from disk
    //
    R_LoadImage(name, &mut pic, &mut width, &mut height);
    if pic.is_null() {
        return 0 as *mut crate::tr_common_h::image_t;
    }
    image = R_CreateImage(name as *mut i8, pic, width, height, type_0, flags, 0);
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")(pic as *mut libc::c_void);
    return image;
}

unsafe extern "C" fn R_CreateDlightImage() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut data: [[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16] = [[[0; 4]; 16]; 16];
    let mut b: i32 = 0;
    // make a centered inverse-square falloff blob for dynamic lighting

    for x in 0..16 {
        for y in 0..16 {
            let mut d: f32 = 0.;

            d = ((16i32 / 2) as f32 - 0.5 - x as f32) * ((16i32 / 2) as f32 - 0.5 - x as f32)
                + ((16i32 / 2) as f32 - 0.5 - y as f32) * ((16i32 / 2) as f32 - 0.5 - y as f32);

            b = (4000f32 / d) as i32;

            if b > 255 {
                b = 255
            } else if b < 75 {
                b = 0
            }

            data[y as usize][x as usize][2] = b as crate::src::qcommon::q_shared::byte;

            data[y as usize][x as usize][1] = data[y as usize][x as usize][2];

            data[y as usize][x as usize][0] = data[y as usize][x as usize][1];

            data[y as usize][x as usize][3] = 255;
        }
    }
    crate::src::renderergl1::tr_main::tr.dlightImage = R_CreateImage(
        b"*dlight\x00" as *const u8 as *const i8,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        16,
        16,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_CLAMPTOEDGE,
        0,
    );
}
/*
=================
R_InitFogTable
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitFogTable() {
    let mut i: i32 = 0;
    let mut d: f32 = 0.;
    let mut exp: f32 = 0.;
    exp = 0.5;
    i = 0;
    while i < 256 {
        d = crate::stdlib::pow((i as f32 / (256i32 - 1) as f32) as f64, exp as f64) as f32;
        crate::src::renderergl1::tr_main::tr.fogTable[i as usize] = d;
        i += 1
    }
}
/*
================
R_FogFactor

Returns a 0.0 to 1.0 fog density value
This is called for each texel of the fog texture on startup
and for each vertex of transparent shaders in fog dynamically
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_FogFactor(mut s: f32, mut t: f32) -> f32 {
    let mut d: f32 = 0.;
    s = (s as f64 - 1.0 / 512f64) as f32;
    if s < 0f32 {
        return 0f32;
    }
    if (t as f64) < 1.0 / 32f64 {
        return 0f32;
    }
    if (t as f64) < 31.0 / 32f64 {
        s *= (t - 1.0f32 / 32.0) / (30.0 / 32.0)
    }
    // we need to leave a lot of clamp range
    s *= 8f32;
    if s as f64 > 1.0 {
        s = 1f32
    }
    d = crate::src::renderergl1::tr_main::tr.fogTable[(s * (256i32 - 1) as f32) as i32 as usize];
    return d;
}

unsafe extern "C" fn R_CreateFogImage() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut data: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut d: f32 = 0.;
    data = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(256 * 32 * 4)
        as *mut crate::src::qcommon::q_shared::byte;
    // S is distance, T is depth

    for x in 0..256 {
        for y in 0..32 {
            d = R_FogFactor((x as f32 + 0.5) / 256f32, (y as f32 + 0.5) / 32f32);

            let ref mut fresh0 = *data.offset(((y * 256 + x) * 4 + 2) as isize);

            *fresh0 = 255u8;

            let ref mut fresh1 = *data.offset(((y * 256 + x) * 4 + 1) as isize);

            *fresh1 = *fresh0;

            *data.offset(((y * 256 + x) * 4 + 0) as isize) = *fresh1;

            *data.offset(((y * 256 + x) * 4 + 3) as isize) =
                (255f32 * d) as crate::src::qcommon::q_shared::byte;
        }
    }
    crate::src::renderergl1::tr_main::tr.fogImage = R_CreateImage(
        b"*fog\x00" as *const u8 as *const i8,
        data,
        256,
        32,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_CLAMPTOEDGE,
        0,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(data as *mut libc::c_void);
}

unsafe extern "C" fn R_CreateDefaultImage() {
    let mut x: i32 = 0;
    let mut data: [[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16] = [[[0; 4]; 16]; 16];
    // the default image will be a box, to allow you to see the mapping coordinates
    crate::stdlib::memset(
        data.as_mut_ptr() as *mut libc::c_void,
        32,
        ::std::mem::size_of::<[[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16]>(),
    );

    for x in 0..16 {
        data[0][x as usize][3] = 255;

        data[0][x as usize][2] = data[0][x as usize][3];

        data[0][x as usize][1] = data[0][x as usize][2];

        data[0][x as usize][0] = data[0][x as usize][1];

        data[x as usize][0][3] = 255;

        data[x as usize][0][2] = data[x as usize][0][3];

        data[x as usize][0][1] = data[x as usize][0][2];

        data[x as usize][0][0] = data[x as usize][0][1];

        data[(16i32 - 1) as usize][x as usize][3] = 255;

        data[(16i32 - 1) as usize][x as usize][2] = data[(16i32 - 1) as usize][x as usize][3];

        data[(16i32 - 1) as usize][x as usize][1] = data[(16i32 - 1) as usize][x as usize][2];

        data[(16i32 - 1) as usize][x as usize][0] = data[(16i32 - 1) as usize][x as usize][1];

        data[x as usize][(16i32 - 1) as usize][3] = 255;

        data[x as usize][(16i32 - 1) as usize][2] = data[x as usize][(16i32 - 1) as usize][3];

        data[x as usize][(16i32 - 1) as usize][1] = data[x as usize][(16i32 - 1) as usize][2];

        data[x as usize][(16i32 - 1) as usize][0] = data[x as usize][(16i32 - 1) as usize][1];
    }
    crate::src::renderergl1::tr_main::tr.defaultImage = R_CreateImage(
        b"*default\x00" as *const u8 as *const i8,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        16,
        16,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_MIPMAP,
        0,
    );
}
/*
==================
R_CreateBuiltinImages
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_CreateBuiltinImages() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut data: [[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16] = [[[0; 4]; 16]; 16];
    R_CreateDefaultImage();
    // we use a solid white image instead of disabling texturing
    crate::stdlib::memset(
        data.as_mut_ptr() as *mut libc::c_void,
        255,
        ::std::mem::size_of::<[[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16]>(),
    );
    crate::src::renderergl1::tr_main::tr.whiteImage = R_CreateImage(
        b"*white\x00" as *const u8 as *const i8,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        8,
        8,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_NONE,
        0,
    );
    // with overbright bits active, we need an image which is some fraction of full color,
    // for default lightmaps, etc
    x = 0;
    while x < 16 {
        for y in 0..16 {
            data[y as usize][x as usize][2] = crate::src::renderergl1::tr_main::tr.identityLightByte
                as crate::src::qcommon::q_shared::byte;

            data[y as usize][x as usize][1] = data[y as usize][x as usize][2];

            data[y as usize][x as usize][0] = data[y as usize][x as usize][1];

            data[y as usize][x as usize][3] = 255;
        }
        x += 1
    }
    crate::src::renderergl1::tr_main::tr.identityLightImage = R_CreateImage(
        b"*identityLight\x00" as *const u8 as *const i8,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        8,
        8,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_NONE,
        0,
    );
    x = 0;
    while x < 32 {
        // scratchimage is usually used for cinematic drawing
        crate::src::renderergl1::tr_main::tr.scratchImage[x as usize] = R_CreateImage(
            b"*scratch\x00" as *const u8 as *const i8,
            data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
            16,
            16,
            crate::tr_common_h::IMGTYPE_COLORALPHA,
            (crate::tr_common_h::IMGFLAG_PICMIP as i32
                | crate::tr_common_h::IMGFLAG_CLAMPTOEDGE as i32)
                as crate::tr_common_h::imgFlags_t,
            0,
        );
        x += 1
    }
    R_CreateDlightImage();
    R_CreateFogImage();
}
/*
===============
R_SetColorMappings
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SetColorMappings() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut g: f32 = 0.;
    let mut inf: i32 = 0;
    let mut shift: i32 = 0;
    // setup the overbright lighting
    crate::src::renderergl1::tr_main::tr.overbrightBits =
        (*crate::src::renderergl1::tr_init::r_overBrightBits).integer;
    if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 == 0 {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 0
        // need hardware gamma for overbright
    }
    // never overbright in windowed mode
    if crate::src::renderergl1::tr_init::glConfig.isFullscreen as u64 == 0 {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 0
    }
    // allow 2 overbright bits in 24 bit, but only 1 in 16 bit
    if crate::src::renderergl1::tr_init::glConfig.colorBits > 16 {
        if crate::src::renderergl1::tr_main::tr.overbrightBits > 2 {
            crate::src::renderergl1::tr_main::tr.overbrightBits = 2
        }
    } else if crate::src::renderergl1::tr_main::tr.overbrightBits > 1 {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 1
    }
    if crate::src::renderergl1::tr_main::tr.overbrightBits < 0 {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 0
    }
    crate::src::renderergl1::tr_main::tr.identityLight =
        1.0 / ((1i32) << crate::src::renderergl1::tr_main::tr.overbrightBits) as f32;
    crate::src::renderergl1::tr_main::tr.identityLightByte =
        (255f32 * crate::src::renderergl1::tr_main::tr.identityLight) as i32;
    if (*crate::src::renderergl1::tr_init::r_intensity).value <= 1f32 {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_intensity\x00" as *const u8 as *const i8,
            b"1\x00" as *const u8 as *const i8,
        );
    }
    if (*crate::src::renderergl1::tr_init::r_gamma).value < 0.5 {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_gamma\x00" as *const u8 as *const i8,
            b"0.5\x00" as *const u8 as *const i8,
        );
    } else if (*crate::src::renderergl1::tr_init::r_gamma).value > 3.0 {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_gamma\x00" as *const u8 as *const i8,
            b"3.0\x00" as *const u8 as *const i8,
        );
    }
    g = (*crate::src::renderergl1::tr_init::r_gamma).value;
    shift = crate::src::renderergl1::tr_main::tr.overbrightBits;
    i = 0;
    while i < 256 {
        if g == 1f32 {
            inf = i
        } else {
            inf = (255f64 * crate::stdlib::pow((i as f32 / 255.0) as f64, (1.0 / g) as f64) + 0.5)
                as i32
        }
        inf <<= shift;
        if inf < 0 {
            inf = 0
        }
        if inf > 255 {
            inf = 255
        }
        s_gammatable[i as usize] = inf as u8;
        i += 1
    }
    i = 0;
    while i < 256 {
        j = (i as f32 * (*crate::src::renderergl1::tr_init::r_intensity).value) as i32;
        if j > 255 {
            j = 255
        }
        s_intensitytable[i as usize] = j as crate::src::qcommon::q_shared::byte;
        i += 1
    }
    if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::sdl::sdl_gamma::GLimp_SetGamma(
            s_gammatable.as_mut_ptr(),
            s_gammatable.as_mut_ptr(),
            s_gammatable.as_mut_ptr(),
        );
    };
}
/*
===============
R_InitImages
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitImages() {
    crate::stdlib::memset(
        hashTable.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[*mut crate::tr_common_h::image_t; 1024]>(),
    );
    // build brightness translation tables
    R_SetColorMappings();
    // create default texture and white texture
    R_CreateBuiltinImages();
}
/*
===============
R_DeleteTextures
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_DeleteTextures() {
    let mut i: i32 = 0;

    for i in 0..crate::src::renderergl1::tr_main::tr.numImages {
        crate::src::sdl::sdl_glimp::qglDeleteTextures.expect("non-null function pointer")(
            1,
            &mut (**crate::src::renderergl1::tr_main::tr
                .images
                .as_mut_ptr()
                .offset(i as isize))
            .texnum,
        );
    }
    crate::stdlib::memset(
        crate::src::renderergl1::tr_main::tr.images.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[*mut crate::tr_common_h::image_t; 2048]>(),
    );
    crate::src::renderergl1::tr_main::tr.numImages = 0;
    crate::stdlib::memset(
        crate::src::renderergl1::tr_init::glState
            .currenttextures
            .as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[i32; 2]>(),
    );
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() {
        crate::src::renderergl1::tr_backend::GL_SelectTexture(1);
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(0xde1u32, 0);
        crate::src::renderergl1::tr_backend::GL_SelectTexture(0);
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1u32, 0u32,
        );
    } else {
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1u32, 0u32,
        );
    };
}
/*
============================================================================

SKINS

============================================================================
*/
/*
==================
CommaParse

This is unfortunate, but the skin files aren't
compatible with our normal parsing rules.
==================
*/

unsafe extern "C" fn CommaParse(mut data_p: *mut *mut i8) -> *mut i8 {
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    let mut data: *mut i8 = 0 as *mut i8;
    static mut com_token: [i8; 1024] = [0; 1024];
    data = *data_p;
    len = 0;
    com_token[0] = 0;
    // make sure incoming data is valid
    if data.is_null() {
        *data_p = 0 as *mut i8;
        return com_token.as_mut_ptr();
    }
    loop {
        loop
        // skip whitespace
        {
            c = *data as i32;
            if !(c <= ' ' as i32) {
                break;
            }
            if c == 0 {
                break;
            }
            data = data.offset(1)
        }
        c = *data as i32;
        // skip double slash comments
        if c == '/' as i32 && *data.offset(1) as i32 == '/' as i32 {
            data = data.offset(2);
            while *data as i32 != 0 && *data as i32 != '\n' as i32 {
                data = data.offset(1)
            }
        } else {
            // skip /* */ comments
            if !(c == '/' as i32 && *data.offset(1) as i32 == '*' as i32) {
                break;
            }
            data = data.offset(2);
            while *data as i32 != 0
                && (*data as i32 != '*' as i32 || *data.offset(1) as i32 != '/' as i32)
            {
                data = data.offset(1)
            }
            if *data != 0 {
                data = data.offset(2)
            }
        }
    }
    if c == 0 {
        return b"\x00" as *const u8 as *mut i8;
    }
    // handle quoted strings
    if c == '\"' as i32 {
        data = data.offset(1);
        loop {
            let fresh2 = data;
            data = data.offset(1);
            c = *fresh2 as i32;
            if c == '\"' as i32 || c == 0 {
                com_token[len as usize] = 0;
                *data_p = data;
                return com_token.as_mut_ptr();
            }
            if len < 1024 - 1 {
                com_token[len as usize] = c as i8;
                len += 1
            }
        }
    }
    loop
    // parse a regular word
    {
        if len < 1024 - 1 {
            com_token[len as usize] = c as i8;
            len += 1
        }
        data = data.offset(1);
        c = *data as i32;
        if !(c > 32 && c != ',' as i32) {
            break;
        }
    }
    com_token[len as usize] = 0;
    *data_p = data;
    return com_token.as_mut_ptr();
}
/*
===============
RE_RegisterSkin

===============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterSkin(
    mut name: *const i8,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut parseSurfaces: [crate::tr_local_h::skinSurface_t; 256] =
        [crate::tr_local_h::skinSurface_t {
            name: [0; 64],
            shader: 0 as *mut crate::tr_local_h::shader_t,
        }; 256];
    let mut hSkin: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    let mut surf: *mut crate::tr_local_h::skinSurface_t =
        0 as *mut crate::tr_local_h::skinSurface_t;
    let mut text: C2RustUnnamed_108 = C2RustUnnamed_108 { c: 0 as *mut i8 };
    let mut text_p: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    let mut surfName: [i8; 64] = [0; 64];
    let mut totalSurfaces: i32 = 0;
    if name.is_null() || *name.offset(0) == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"Empty name passed to RE_RegisterSkin\n\x00" as *const u8 as *const i8,
        );
        return 0i32;
    }
    if crate::stdlib::strlen(name) >= 64 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"Skin name exceeds MAX_QPATH\n\x00" as *const u8 as *const i8,
        );
        return 0i32;
    }
    // see if the skin is already loaded
    hSkin = 1;
    while hSkin < crate::src::renderergl1::tr_main::tr.numSkins {
        skin = crate::src::renderergl1::tr_main::tr.skins[hSkin as usize];
        if crate::src::qcommon::q_shared::Q_stricmp((*skin).name.as_mut_ptr(), name) == 0 {
            if (*skin).numSurfaces == 0 {
                return 0i32;
                // default skin
            }
            return hSkin;
        }
        hSkin += 1
    }
    // allocate a new skin
    if crate::src::renderergl1::tr_main::tr.numSkins == 1024 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: RE_RegisterSkin( \'%s\' ) MAX_SKINS hit\n\x00" as *const u8 as *const i8,
            name,
        );
        return 0i32;
    }
    crate::src::renderergl1::tr_main::tr.numSkins += 1;
    skin = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::skin_t>() as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::skin_t;
    crate::src::renderergl1::tr_main::tr.skins[hSkin as usize] = skin;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*skin).name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    (*skin).numSurfaces = 0;
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    // If not a .skin file, load as a single shader
    if crate::stdlib::strcmp(
        name.offset(crate::stdlib::strlen(name) as isize)
            .offset(-(5)),
        b".skin\x00" as *const u8 as *const i8,
    ) != 0
    {
        (*skin).numSurfaces = 1;
        (*skin).surfaces = crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ::std::mem::size_of::<crate::tr_local_h::skinSurface_t>() as i32,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_local_h::skinSurface_t;
        let ref mut fresh3 = (*(*skin).surfaces.offset(0)).shader;
        *fresh3 = crate::src::renderergl1::tr_shader::R_FindShader(
            name,
            -(1),
            crate::src::qcommon::q_shared::qtrue,
        );
        return hSkin;
    }
    // load and parse the skin file
    crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(name, &mut text.v);
    if text.c.is_null() {
        return 0i32;
    }
    totalSurfaces = 0;
    text_p = text.c;
    while !text_p.is_null() && *text_p as i32 != 0 {
        // get surface name
        token = CommaParse(&mut text_p);
        crate::src::qcommon::q_shared::Q_strncpyz(
            surfName.as_mut_ptr(),
            token,
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        if *token.offset(0) == 0 {
            break;
        }
        // lowercase the surface name so skin compares are faster
        crate::src::qcommon::q_shared::Q_strlwr(surfName.as_mut_ptr());
        if *text_p as i32 == ',' as i32 {
            text_p = text_p.offset(1)
        }
        if !crate::stdlib::strstr(token, b"tag_\x00" as *const u8 as *const i8).is_null() {
            continue;
        }
        // parse the shader name
        token = CommaParse(&mut text_p);
        if (*skin).numSurfaces < 256 {
            surf = &mut *parseSurfaces
                .as_mut_ptr()
                .offset((*skin).numSurfaces as isize)
                as *mut crate::tr_local_h::skinSurface_t;
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*surf).name.as_mut_ptr(),
                surfName.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            (*surf).shader = crate::src::renderergl1::tr_shader::R_FindShader(
                token,
                -(1),
                crate::src::qcommon::q_shared::qtrue,
            );
            (*skin).numSurfaces += 1
        }
        totalSurfaces += 1
    }
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(text.v);
    if totalSurfaces > 256 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: Ignoring excess surfaces (found %d, max is %d) in skin \'%s\'!\n\x00"
                as *const u8 as *const i8,
            totalSurfaces,
            256i32,
            name,
        );
    }
    // never let a skin have 0 shaders
    if (*skin).numSurfaces == 0 {
        return 0i32;
        // use default skin
    }
    // copy surfaces to skin
    (*skin).surfaces = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ((*skin).numSurfaces as usize)
            .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::skinSurface_t>()) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::skinSurface_t;
    crate::stdlib::memcpy(
        (*skin).surfaces as *mut libc::c_void,
        parseSurfaces.as_mut_ptr() as *const libc::c_void,
        ((*skin).numSurfaces as usize)
            .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::skinSurface_t>()),
    );
    return hSkin;
}
/*
===============
R_InitSkins
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitSkins() {
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    crate::src::renderergl1::tr_main::tr.numSkins = 1;
    // make the default skin have all default shaders
    crate::src::renderergl1::tr_main::tr.skins[0] = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::skin_t>() as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::skin_t;
    skin = crate::src::renderergl1::tr_main::tr.skins[0];
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*skin).name.as_mut_ptr(),
        b"<default skin>\x00" as *const u8 as *const i8,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    (*skin).numSurfaces = 1;
    (*skin).surfaces = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::skinSurface_t>() as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::skinSurface_t;
    let ref mut fresh4 = (*(*skin).surfaces.offset(0)).shader;
    *fresh4 = crate::src::renderergl1::tr_main::tr.defaultShader;
}
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
/*
===============
R_GetSkinByHandle
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetSkinByHandle(
    mut hSkin: crate::src::qcommon::q_shared::qhandle_t,
) -> *mut crate::tr_local_h::skin_t {
    if hSkin < 1 || hSkin >= crate::src::renderergl1::tr_main::tr.numSkins {
        return crate::src::renderergl1::tr_main::tr.skins[0usize];
    }
    return crate::src::renderergl1::tr_main::tr.skins[hSkin as usize];
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
/*
===============
R_SkinList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SkinList_f() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"------------------\n\x00" as *const u8 as *const i8,
    );

    for i in 0..crate::src::renderergl1::tr_main::tr.numSkins {
        skin = crate::src::renderergl1::tr_main::tr.skins[i as usize];

        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%3i:%s (%d surfaces)\n\x00" as *const u8 as *const i8,
            i,
            (*skin).name.as_mut_ptr(),
            (*skin).numSurfaces,
        );
        for j in 0..(*skin).numSurfaces {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"       %s = %s\n\x00" as *const u8 as *const i8,
                (*(*skin).surfaces.offset(j as isize)).name.as_mut_ptr(),
                (*(*(*skin).surfaces.offset(j as isize)).shader)
                    .name
                    .as_mut_ptr(),
            );
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"------------------\n\x00" as *const u8 as *const i8,
    );
}
unsafe extern "C" fn run_static_initializers() {
    numImageLoaders = (::std::mem::size_of::<[imageExtToLoaderMap_t; 6]>())
        .wrapping_div(::std::mem::size_of::<imageExtToLoaderMap_t>()) as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
