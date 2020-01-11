use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0) = *v1.offset(1) * *v2.offset(2) - *v1.offset(2) * *v2.offset(1);
        *cross.offset(1) = *v1.offset(2) * *v2.offset(0) - *v1.offset(0) * *v2.offset(2);
        *cross.offset(2) = *v1.offset(0) * *v2.offset(1) - *v1.offset(1) * *v2.offset(0);
    }
    #[inline]

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0) * *v.offset(0)
            + *v.offset(1) * *v.offset(1)
            + *v.offset(2) * *v.offset(2);
    }
    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }

    // __Q_SHARED_H
}

pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AxisCopy;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::PlaneFromPoints;
pub use crate::src::qcommon::q_math::RotatePointAroundVector;
pub use crate::src::qcommon::q_math::SetPlaneSignbits;
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
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
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
pub use crate::src::renderergl1::tr_main::q_shared_h::CrossProduct;
pub use crate::src::renderergl1::tr_main::q_shared_h::VectorLength;
pub use crate::src::renderergl1::tr_main::q_shared_h::VectorLengthSquared;
pub use crate::tr_public_h::refimport_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::stereoFrame_t;
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

pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::DepthRangeproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::src::renderergl1::tr_animation::R_MDRAddAnimSurfaces;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_Cull;
pub use crate::src::renderergl1::tr_backend::GL_State;
pub use crate::src::renderergl1::tr_cmds::R_AddDrawSurfCmd;
pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_init::r_debugSurface;
pub use crate::src::renderergl1::tr_init::r_drawentities;
pub use crate::src::renderergl1::tr_init::r_fastsky;
pub use crate::src::renderergl1::tr_init::r_nocull;
pub use crate::src::renderergl1::tr_init::r_noportals;
pub use crate::src::renderergl1::tr_init::r_portalOnly;
pub use crate::src::renderergl1::tr_init::r_stereoSeparation;
pub use crate::src::renderergl1::tr_init::r_znear;
pub use crate::src::renderergl1::tr_init::r_zproj;
pub use crate::src::renderergl1::tr_mesh::R_AddMD3Surfaces;
pub use crate::src::renderergl1::tr_model::R_GetModelByHandle;
pub use crate::src::renderergl1::tr_model_iqm::R_AddIQMSurfaces;
pub use crate::src::renderergl1::tr_scene::R_AddPolygonSurfaces;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shade::RB_BeginSurface;
pub use crate::src::renderergl1::tr_shader::R_GetShaderByHandle;
pub use crate::src::renderergl1::tr_surface::rb_surfaceTable;
pub use crate::src::renderergl1::tr_world::R_AddBrushModelSurfaces;
pub use crate::src::renderergl1::tr_world::R_AddWorldSurfaces;
pub use crate::src::sdl::sdl_glimp::qglBegin;
pub use crate::src::sdl::sdl_glimp::qglColor3f;
pub use crate::src::sdl::sdl_glimp::qglDepthRange;
pub use crate::src::sdl::sdl_glimp::qglEnd;
pub use crate::src::sdl::sdl_glimp::qglVertex3fv;

pub use crate::stdlib::GLclampd;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
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
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::color4ub_t;
pub use crate::tr_local_h::colorGen_t;
pub use crate::tr_local_h::cullType_t;
pub use crate::tr_local_h::deformStage_t;
pub use crate::tr_local_h::deform_t;
pub use crate::tr_local_h::dlight_s;
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::drawSurf_t;
pub use crate::tr_local_h::fogParms_t;
pub use crate::tr_local_h::fogPass_t;
pub use crate::tr_local_h::fog_t;
pub use crate::tr_local_h::frontEndCounters_t;
pub use crate::tr_local_h::genFunc_t;
pub use crate::tr_local_h::glIndex_t;
pub use crate::tr_local_h::mnode_s;
pub use crate::tr_local_h::mnode_t;
pub use crate::tr_local_h::model_s;
pub use crate::tr_local_h::model_t;
pub use crate::tr_local_h::modtype_t;
pub use crate::tr_local_h::msurface_s;
pub use crate::tr_local_h::msurface_t;
pub use crate::tr_local_h::orientationr_t;
pub use crate::tr_local_h::shaderCommands_s;
pub use crate::tr_local_h::shaderCommands_t;
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfPoly_t;
pub use crate::tr_local_h::srfSurfaceFace_t;
pub use crate::tr_local_h::srfTriangles_t;
pub use crate::tr_local_h::stageVars;
pub use crate::tr_local_h::stageVars_t;
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
// tr_main.c -- main control flow for each frame
// memcpy
#[no_mangle]

pub static mut tr: crate::tr_local_h::trGlobals_t = crate::tr_local_h::trGlobals_t {
    registered: crate::src::qcommon::q_shared::qfalse,
    visCount: 0,
    frameCount: 0,
    sceneCount: 0,
    viewCount: 0,
    frameSceneNum: 0,
    worldMapLoaded: crate::src::qcommon::q_shared::qfalse,
    world: 0 as *mut crate::tr_local_h::world_t,
    externalVisData: 0 as *const crate::src::qcommon::q_shared::byte,
    defaultImage: 0 as *mut crate::tr_common_h::image_t,
    scratchImage: [0 as *mut crate::tr_common_h::image_t; 32],
    fogImage: 0 as *mut crate::tr_common_h::image_t,
    dlightImage: 0 as *mut crate::tr_common_h::image_t,
    flareImage: 0 as *mut crate::tr_common_h::image_t,
    whiteImage: 0 as *mut crate::tr_common_h::image_t,
    identityLightImage: 0 as *mut crate::tr_common_h::image_t,
    defaultShader: 0 as *mut crate::tr_local_h::shader_t,
    shadowShader: 0 as *mut crate::tr_local_h::shader_t,
    projectionShadowShader: 0 as *mut crate::tr_local_h::shader_t,
    flareShader: 0 as *mut crate::tr_local_h::shader_t,
    sunShader: 0 as *mut crate::tr_local_h::shader_t,
    numLightmaps: 0,
    lightmaps: 0 as *mut *mut crate::tr_common_h::image_t,
    currentEntity: 0 as *mut crate::tr_local_h::trRefEntity_t,
    worldEntity: crate::tr_local_h::trRefEntity_t {
        e: crate::tr_types_h::refEntity_t {
            reType: crate::tr_types_h::RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        },
        axisLength: 0.,
        needDlights: crate::src::qcommon::q_shared::qfalse,
        lightingCalculated: crate::src::qcommon::q_shared::qfalse,
        lightDir: [0.; 3],
        ambientLight: [0.; 3],
        ambientLightInt: 0,
        directedLight: [0.; 3],
    },
    currentEntityNum: 0,
    shiftedEntityNum: 0,
    currentModel: 0 as *mut crate::tr_local_h::model_t,
    viewParms: crate::tr_local_h::viewParms_t {
        or: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        world: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        pvsOrigin: [0.; 3],
        isPortal: crate::src::qcommon::q_shared::qfalse,
        isMirror: crate::src::qcommon::q_shared::qfalse,
        frameSceneNum: 0,
        frameCount: 0,
        portalPlane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        viewportX: 0,
        viewportY: 0,
        viewportWidth: 0,
        viewportHeight: 0,
        fovX: 0.,
        fovY: 0.,
        projectionMatrix: [0.; 16],
        frustum: [crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        }; 4],
        visBounds: [[0.; 3]; 2],
        zFar: 0.,
        stereoFrame: crate::tr_types_h::STEREO_CENTER,
    },
    identityLight: 0.,
    identityLightByte: 0,
    overbrightBits: 0,
    or: crate::tr_local_h::orientationr_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
        viewOrigin: [0.; 3],
        modelMatrix: [0.; 16],
    },
    refdef: crate::tr_local_h::trRefdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        stereoFrame: crate::tr_types_h::STEREO_CENTER,
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        areamaskModified: crate::src::qcommon::q_shared::qfalse,
        floatTime: 0.,
        text: [[0; 32]; 8],
        num_entities: 0,
        entities: 0 as *mut crate::tr_local_h::trRefEntity_t,
        num_dlights: 0,
        dlights: 0 as *mut crate::tr_local_h::dlight_s,
        numPolys: 0,
        polys: 0 as *mut crate::tr_local_h::srfPoly_s,
        numDrawSurfs: 0,
        drawSurfs: 0 as *mut crate::tr_local_h::drawSurf_s,
    },
    viewCluster: 0,
    sunLight: [0.; 3],
    sunDirection: [0.; 3],
    pc: crate::tr_local_h::frontEndCounters_t {
        c_sphere_cull_patch_in: 0,
        c_sphere_cull_patch_clip: 0,
        c_sphere_cull_patch_out: 0,
        c_box_cull_patch_in: 0,
        c_box_cull_patch_clip: 0,
        c_box_cull_patch_out: 0,
        c_sphere_cull_md3_in: 0,
        c_sphere_cull_md3_clip: 0,
        c_sphere_cull_md3_out: 0,
        c_box_cull_md3_in: 0,
        c_box_cull_md3_clip: 0,
        c_box_cull_md3_out: 0,
        c_leafs: 0,
        c_dlightSurfaces: 0,
        c_dlightSurfacesCulled: 0,
    },
    frontEndMsec: 0,
    models: [0 as *mut crate::tr_local_h::model_t; 1024],
    numModels: 0,
    numImages: 0,
    images: [0 as *mut crate::tr_common_h::image_t; 2048],
    numShaders: 0,
    shaders: [0 as *mut crate::tr_local_h::shader_t; 16384],
    sortedShaders: [0 as *mut crate::tr_local_h::shader_t; 16384],
    numSkins: 0,
    skins: [0 as *mut crate::tr_local_h::skin_t; 1024],
    sinTable: [0.; 1024],
    squareTable: [0.; 1024],
    triangleTable: [0.; 1024],
    sawToothTable: [0.; 1024],
    inverseSawToothTable: [0.; 1024],
    fogTable: [0.; 256],
};

static mut s_flipMatrix: [f32; 16] = [
    0f32, 0f32, -1f32, 0f32, -1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32,
    1f32,
];
#[no_mangle]

pub static mut ri: crate::tr_public_h::refimport_t = crate::tr_public_h::refimport_t {
    Printf: None,
    Error: None,
    Milliseconds: None,
    Hunk_Alloc: None,
    Hunk_AllocateTempMemory: None,
    Hunk_FreeTempMemory: None,
    Malloc: None,
    Free: None,
    Cvar_Get: None,
    Cvar_Set: None,
    Cvar_SetValue: None,
    Cvar_CheckRange: None,
    Cvar_SetDescription: None,
    Cvar_VariableIntegerValue: None,
    Cmd_AddCommand: None,
    Cmd_RemoveCommand: None,
    Cmd_Argc: None,
    Cmd_Argv: None,
    Cmd_ExecuteText: None,
    CM_ClusterPVS: None,
    CM_DrawDebugSurface: None,
    FS_FileIsInPAK: None,
    FS_ReadFile: None,
    FS_FreeFile: None,
    FS_ListFiles: None,
    FS_FreeFileList: None,
    FS_WriteFile: None,
    FS_FileExists: None,
    CIN_UploadCinematic: None,
    CIN_PlayCinematic: None,
    CIN_RunCinematic: None,
    CL_WriteAVIVideoFrame: None,
    IN_Init: None,
    IN_Shutdown: None,
    IN_Restart: None,
    ftol: None,
    Sys_SetEnv: None,
    Sys_GLimpSafeInit: None,
    Sys_GLimpInit: None,
    Sys_LowPhysicalMemory: None,
};
// entities that will have procedurally generated surfaces will just
// point at this for their sorting surface
#[no_mangle]

pub static mut entitySurface: crate::tr_local_h::surfaceType_t = crate::tr_local_h::SF_ENTITY;
/*
=================
R_CullLocalBox

Returns CULL_IN, CULL_CLIP, or CULL_OUT
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_CullLocalBox(
    mut bounds: *mut crate::src::qcommon::q_shared::vec3_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut transformed: [crate::src::qcommon::q_shared::vec3_t; 8] = [[0.; 3]; 8];
    let mut dists: [f32; 8] = [0.; 8];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut frust: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut anyBack: i32 = 0;
    let mut front: i32 = 0;
    let mut back: i32 = 0;
    if (*crate::src::renderergl1::tr_init::r_nocull).integer != 0 {
        return 1i32;
    }
    // transform into world space
    i = 0;
    while i < 8 {
        v[0] = (*bounds.offset((i & 1) as isize))[0];
        v[1] = (*bounds.offset((i >> 1 & 1) as isize))[1];
        v[2] = (*bounds.offset((i >> 2 & 1) as isize))[2];
        transformed[i as usize][0] = tr.or.origin[0];
        transformed[i as usize][1] = tr.or.origin[1];
        transformed[i as usize][2] = tr.or.origin[2];
        transformed[i as usize][0] = transformed[i as usize][0] + tr.or.axis[0][0] * v[0];
        transformed[i as usize][1] = transformed[i as usize][1] + tr.or.axis[0][1] * v[0];
        transformed[i as usize][2] = transformed[i as usize][2] + tr.or.axis[0][2] * v[0];
        transformed[i as usize][0] = transformed[i as usize][0] + tr.or.axis[1][0] * v[1];
        transformed[i as usize][1] = transformed[i as usize][1] + tr.or.axis[1][1] * v[1];
        transformed[i as usize][2] = transformed[i as usize][2] + tr.or.axis[1][2] * v[1];
        transformed[i as usize][0] = transformed[i as usize][0] + tr.or.axis[2][0] * v[2];
        transformed[i as usize][1] = transformed[i as usize][1] + tr.or.axis[2][1] * v[2];
        transformed[i as usize][2] = transformed[i as usize][2] + tr.or.axis[2][2] * v[2];
        i += 1
    }
    // check against frustum planes
    anyBack = 0;
    i = 0;
    while i < 4 {
        frust = &mut *tr.viewParms.frustum.as_mut_ptr().offset(i as isize)
            as *mut crate::src::qcommon::q_shared::cplane_t;
        back = 0;
        front = back;

        for j in 0..8 {
            dists[j as usize] = transformed[j as usize][0] * (*frust).normal[0]
                + transformed[j as usize][1] * (*frust).normal[1]
                + transformed[j as usize][2] * (*frust).normal[2];

            if dists[j as usize] > (*frust).dist {
                front = 1;
                if back != 0 {
                    break;
                }
            } else {
                back = 1
            }
        }
        if front == 0 {
            // all points were behind one of the planes
            return 2i32;
        }
        anyBack |= back;
        i += 1
    }
    if anyBack == 0 {
        return 0i32;
        // completely inside frustum
    }
    return 1;
    // partially clipped
}
/*
** R_CullLocalPointAndRadius
*/
#[no_mangle]

pub unsafe extern "C" fn R_CullLocalPointAndRadius(
    mut pt: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
) -> i32 {
    let mut transformed: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    R_LocalPointToWorld(pt, transformed.as_mut_ptr());
    return R_CullPointAndRadius(transformed.as_mut_ptr(), radius);
}
/*
** R_CullPointAndRadius
*/
#[no_mangle]

pub unsafe extern "C" fn R_CullPointAndRadius(
    mut pt: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
) -> i32 {
    let mut _i: i32 = 0;
    let mut dist: f32 = 0.;
    let mut frust: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut mightBeClipped: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*crate::src::renderergl1::tr_init::r_nocull).integer != 0 {
        return 1i32;
    }
    // check against frustum planes

    for i in 0..4 {
        frust = &mut *tr.viewParms.frustum.as_mut_ptr().offset(i as isize)
            as *mut crate::src::qcommon::q_shared::cplane_t;

        dist = *pt.offset(0) * (*frust).normal[0]
            + *pt.offset(1) * (*frust).normal[1]
            + *pt.offset(2) * (*frust).normal[2]
            - (*frust).dist;

        if dist < -radius {
            return 2i32;
        } else {
            if dist <= radius {
                mightBeClipped = crate::src::qcommon::q_shared::qtrue
            }
        }
    }
    if mightBeClipped as u64 != 0 {
        return 1i32;
    }
    return 0;
    // completely inside frustum
}
/*
=================
R_LocalNormalToWorld

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LocalNormalToWorld(
    mut local: *mut crate::src::qcommon::q_shared::vec_t,
    mut world: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *world.offset(0) = *local.offset(0) * tr.or.axis[0][0]
        + *local.offset(1) * tr.or.axis[1][0]
        + *local.offset(2) * tr.or.axis[2][0];
    *world.offset(1) = *local.offset(0) * tr.or.axis[0][1]
        + *local.offset(1) * tr.or.axis[1][1]
        + *local.offset(2) * tr.or.axis[2][1];
    *world.offset(2) = *local.offset(0) * tr.or.axis[0][2]
        + *local.offset(1) * tr.or.axis[1][2]
        + *local.offset(2) * tr.or.axis[2][2];
}
/*
=================
R_LocalPointToWorld

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LocalPointToWorld(
    mut local: *mut crate::src::qcommon::q_shared::vec_t,
    mut world: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *world.offset(0) = *local.offset(0) * tr.or.axis[0][0]
        + *local.offset(1) * tr.or.axis[1][0]
        + *local.offset(2) * tr.or.axis[2][0]
        + tr.or.origin[0];
    *world.offset(1) = *local.offset(0) * tr.or.axis[0][1]
        + *local.offset(1) * tr.or.axis[1][1]
        + *local.offset(2) * tr.or.axis[2][1]
        + tr.or.origin[1];
    *world.offset(2) = *local.offset(0) * tr.or.axis[0][2]
        + *local.offset(1) * tr.or.axis[1][2]
        + *local.offset(2) * tr.or.axis[2][2]
        + tr.or.origin[2];
}
/*
=================
R_WorldToLocal

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_WorldToLocal(
    mut world: *mut crate::src::qcommon::q_shared::vec_t,
    mut local: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *local.offset(0) = *world.offset(0) * tr.or.axis[0][0]
        + *world.offset(1) * tr.or.axis[0][1]
        + *world.offset(2) * tr.or.axis[0][2];
    *local.offset(1) = *world.offset(0) * tr.or.axis[1][0]
        + *world.offset(1) * tr.or.axis[1][1]
        + *world.offset(2) * tr.or.axis[1][2];
    *local.offset(2) = *world.offset(0) * tr.or.axis[2][0]
        + *world.offset(1) * tr.or.axis[2][1]
        + *world.offset(2) * tr.or.axis[2][2];
}
/*
==========================
R_TransformModelToClip

==========================
*/
#[no_mangle]

pub unsafe extern "C" fn R_TransformModelToClip(
    mut src: *const crate::src::qcommon::q_shared::vec_t,
    mut modelMatrix: *const f32,
    mut projectionMatrix: *const f32,
    mut eye: *mut crate::src::qcommon::q_shared::vec_t,
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        *eye.offset(i as isize) = *src.offset(0) * *modelMatrix.offset((i + 0 * 4) as isize)
            + *src.offset(1) * *modelMatrix.offset((i + 1 * 4) as isize)
            + *src.offset(2) * *modelMatrix.offset((i + 2 * 4) as isize)
            + 1f32 * *modelMatrix.offset((i + 3 * 4) as isize);
        i += 1
    }
    i = 0;
    while i < 4 {
        *dst.offset(i as isize) = *eye.offset(0) * *projectionMatrix.offset((i + 0 * 4) as isize)
            + *eye.offset(1) * *projectionMatrix.offset((i + 1 * 4) as isize)
            + *eye.offset(2) * *projectionMatrix.offset((i + 2 * 4) as isize)
            + *eye.offset(3) * *projectionMatrix.offset((i + 3 * 4) as isize);
        i += 1
    }
}
/*
** GL wrapper/helper functions
*/
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
//
// tr_shader.c
//
/*
====================================================================

TESSELATOR/SHADER DECLARATIONS

====================================================================
*/
// or together of all vertexDlightBits
// info extracted from current shader
/*
============================================================

WORLD MAP

============================================================
*/
/*
============================================================

FLARES

============================================================
*/
/*
============================================================

LIGHTS

============================================================
*/
/*
============================================================

SHADOWS

============================================================
*/
/*
============================================================

SKIES

============================================================
*/
/*
============================================================

CURVE TESSELATION

============================================================
*/
/*
============================================================

MARKERS, POLYGON PROJECTION ON WORLD POLYGONS

============================================================
*/
/*
============================================================

SCENE GENERATION

============================================================
*/
/*
=============================================================

UNCOMPRESSING BONES

=============================================================
*/
/*
=============================================================

ANIMATED MODELS

=============================================================
*/
/*
=============================================================
=============================================================
*/
/*
==========================
R_TransformClipToWindow

==========================
*/
#[no_mangle]

pub unsafe extern "C" fn R_TransformClipToWindow(
    mut clip: *const crate::src::qcommon::q_shared::vec_t,
    mut view: *const crate::tr_local_h::viewParms_t,
    mut normalized: *mut crate::src::qcommon::q_shared::vec_t,
    mut window: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *normalized.offset(0) = *clip.offset(0) / *clip.offset(3);
    *normalized.offset(1) = *clip.offset(1) / *clip.offset(3);
    *normalized.offset(2) = (*clip.offset(2) + *clip.offset(3)) / (2f32 * *clip.offset(3));
    *window.offset(0) = 0.5 * (1.0 + *normalized.offset(0)) * (*view).viewportWidth as f32;
    *window.offset(1) = 0.5 * (1.0 + *normalized.offset(1)) * (*view).viewportHeight as f32;
    *window.offset(2) = *normalized.offset(2);
    *window.offset(0) =
        (*window.offset(0) as f64 + 0.5) as i32 as crate::src::qcommon::q_shared::vec_t;
    *window.offset(1) =
        (*window.offset(1) as f64 + 0.5) as i32 as crate::src::qcommon::q_shared::vec_t;
}
/*
==========================
myGlMultMatrix

==========================
*/
#[no_mangle]

pub unsafe extern "C" fn myGlMultMatrix(mut a: *const f32, mut b: *const f32, mut out: *mut f32) {
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    i = 0;
    while i < 4 {
        for j in 0..4 {
            *out.offset((i * 4 + j) as isize) = *a.offset((i * 4 + 0) as isize)
                * *b.offset((0 * 4 + j) as isize)
                + *a.offset((i * 4 + 1) as isize) * *b.offset((1 * 4 + j) as isize)
                + *a.offset((i * 4 + 2) as isize) * *b.offset((2 * 4 + j) as isize)
                + *a.offset((i * 4 + 3) as isize) * *b.offset((3 * 4 + j) as isize);
        }
        i += 1
    }
}
/*
=================
R_RotateForEntity

Generates an orientation for an entity and viewParms
Does NOT produce any GL calls
Called by both the front end and the back end
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_RotateForEntity(
    mut ent: *const crate::tr_local_h::trRefEntity_t,
    mut viewParms: *const crate::tr_local_h::viewParms_t,
    mut or: *mut crate::tr_local_h::orientationr_t,
) {
    let mut glMatrix: [f32; 16] = [0.; 16];
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut axisLength: f32 = 0.;
    if (*ent).e.reType != crate::tr_types_h::RT_MODEL {
        *or = (*viewParms).world;
        return;
    }
    (*or).origin[0] = (*ent).e.origin[0];
    (*or).origin[1] = (*ent).e.origin[1];
    (*or).origin[2] = (*ent).e.origin[2];
    (*or).axis[0][0] = (*ent).e.axis[0][0];
    (*or).axis[0][1] = (*ent).e.axis[0][1];
    (*or).axis[0][2] = (*ent).e.axis[0][2];
    (*or).axis[1][0] = (*ent).e.axis[1][0];
    (*or).axis[1][1] = (*ent).e.axis[1][1];
    (*or).axis[1][2] = (*ent).e.axis[1][2];
    (*or).axis[2][0] = (*ent).e.axis[2][0];
    (*or).axis[2][1] = (*ent).e.axis[2][1];
    (*or).axis[2][2] = (*ent).e.axis[2][2];
    glMatrix[0] = (*or).axis[0][0];
    glMatrix[4] = (*or).axis[1][0];
    glMatrix[8] = (*or).axis[2][0];
    glMatrix[12] = (*or).origin[0];
    glMatrix[1] = (*or).axis[0][1];
    glMatrix[5] = (*or).axis[1][1];
    glMatrix[9] = (*or).axis[2][1];
    glMatrix[13] = (*or).origin[1];
    glMatrix[2] = (*or).axis[0][2];
    glMatrix[6] = (*or).axis[1][2];
    glMatrix[10] = (*or).axis[2][2];
    glMatrix[14] = (*or).origin[2];
    glMatrix[3] = 0f32;
    glMatrix[7] = 0f32;
    glMatrix[11] = 0f32;
    glMatrix[15] = 1f32;
    myGlMultMatrix(
        glMatrix.as_mut_ptr(),
        (*viewParms).world.modelMatrix.as_ptr(),
        (*or).modelMatrix.as_mut_ptr(),
    );
    // calculate the viewer origin in the model's space
    // needed for fog, specular, and environment mapping
    delta[0] = (*viewParms).or.origin[0] - (*or).origin[0];
    delta[1] = (*viewParms).or.origin[1] - (*or).origin[1];
    delta[2] = (*viewParms).or.origin[2] - (*or).origin[2];
    // compensate for scale in the axes if necessary
    if (*ent).e.nonNormalizedAxes as u64 != 0 {
        axisLength = VectorLength((*ent).e.axis[0].as_ptr());
        if axisLength == 0. {
            axisLength = 0f32
        } else {
            axisLength = 1.0 / axisLength
        }
    } else {
        axisLength = 1.0
    }
    (*or).viewOrigin[0] =
        (delta[0] * (*or).axis[0][0] + delta[1] * (*or).axis[0][1] + delta[2] * (*or).axis[0][2])
            * axisLength;
    (*or).viewOrigin[1] =
        (delta[0] * (*or).axis[1][0] + delta[1] * (*or).axis[1][1] + delta[2] * (*or).axis[1][2])
            * axisLength;
    (*or).viewOrigin[2] =
        (delta[0] * (*or).axis[2][0] + delta[1] * (*or).axis[2][1] + delta[2] * (*or).axis[2][2])
            * axisLength;
}
/*
=================
R_RotateForViewer

Sets up the modelview matrix for a given viewParm
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_RotateForViewer() {
    let mut viewerMatrix: [f32; 16] = [0.; 16];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::stdlib::memset(
        &mut tr.or as *mut crate::tr_local_h::orientationr_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_local_h::orientationr_t>(),
    );
    tr.or.axis[0][0] = 1f32;
    tr.or.axis[1][1] = 1f32;
    tr.or.axis[2][2] = 1f32;
    tr.or.viewOrigin[0] = tr.viewParms.or.origin[0];
    tr.or.viewOrigin[1] = tr.viewParms.or.origin[1];
    tr.or.viewOrigin[2] = tr.viewParms.or.origin[2];
    // transform by the camera placement
    origin[0] = tr.viewParms.or.origin[0];
    origin[1] = tr.viewParms.or.origin[1];
    origin[2] = tr.viewParms.or.origin[2];
    viewerMatrix[0] = tr.viewParms.or.axis[0][0];
    viewerMatrix[4] = tr.viewParms.or.axis[0][1];
    viewerMatrix[8] = tr.viewParms.or.axis[0][2];
    viewerMatrix[12] =
        -origin[0] * viewerMatrix[0] + -origin[1] * viewerMatrix[4] + -origin[2] * viewerMatrix[8];
    viewerMatrix[1] = tr.viewParms.or.axis[1][0];
    viewerMatrix[5] = tr.viewParms.or.axis[1][1];
    viewerMatrix[9] = tr.viewParms.or.axis[1][2];
    viewerMatrix[13] =
        -origin[0] * viewerMatrix[1] + -origin[1] * viewerMatrix[5] + -origin[2] * viewerMatrix[9];
    viewerMatrix[2] = tr.viewParms.or.axis[2][0];
    viewerMatrix[6] = tr.viewParms.or.axis[2][1];
    viewerMatrix[10] = tr.viewParms.or.axis[2][2];
    viewerMatrix[14] =
        -origin[0] * viewerMatrix[2] + -origin[1] * viewerMatrix[6] + -origin[2] * viewerMatrix[10];
    viewerMatrix[3] = 0f32;
    viewerMatrix[7] = 0f32;
    viewerMatrix[11] = 0f32;
    viewerMatrix[15] = 1f32;
    // convert from our coordinate system (looking down X)
    // to OpenGL's coordinate system (looking down -Z)
    myGlMultMatrix(
        viewerMatrix.as_mut_ptr(),
        s_flipMatrix.as_mut_ptr(),
        tr.or.modelMatrix.as_mut_ptr(),
    );
    tr.viewParms.world = tr.or;
}
/*
** SetFarClip
*/

unsafe extern "C" fn R_SetFarClip() {
    let mut farthestCornerDistance: f32 = 0f32;
    let mut _i: i32 = 0;
    // if not rendering the world (icons, menus, etc)
    // set a 2k far clip plane
    if tr.refdef.rdflags & 0x1 != 0 {
        tr.viewParms.zFar = 2048f32;
        return;
    }
    //
    // set far clipping planes dynamically
    //
    farthestCornerDistance = 0f32;

    for i in 0..8 {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];

        let mut vecTo: crate::src::qcommon::q_shared::vec3_t = [0.; 3];

        let mut distance: f32 = 0.;

        if i & 1 != 0 {
            v[0] = tr.viewParms.visBounds[0][0]
        } else {
            v[0] = tr.viewParms.visBounds[1][0]
        }

        if i & 2 != 0 {
            v[1] = tr.viewParms.visBounds[0][1]
        } else {
            v[1] = tr.viewParms.visBounds[1][1]
        }

        if i & 4 != 0 {
            v[2] = tr.viewParms.visBounds[0][2]
        } else {
            v[2] = tr.viewParms.visBounds[1][2]
        }

        vecTo[0] = v[0] - tr.viewParms.or.origin[0];

        vecTo[1] = v[1] - tr.viewParms.or.origin[1];

        vecTo[2] = v[2] - tr.viewParms.or.origin[2];

        distance = vecTo[0] * vecTo[0] + vecTo[1] * vecTo[1] + vecTo[2] * vecTo[2];

        if distance > farthestCornerDistance {
            farthestCornerDistance = distance
        }
    }
    tr.viewParms.zFar = crate::stdlib::sqrt(farthestCornerDistance as f64) as f32;
}
/*
=================
R_SetupFrustum

Set up the culling frustum planes for the current view using the results we got from computing the first two rows of
the projection matrix.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_SetupFrustum(
    mut dest: *mut crate::tr_local_h::viewParms_t,
    mut xmin: f32,
    mut xmax: f32,
    mut ymax: f32,
    mut zProj: f32,
    mut stereoSep: f32,
) {
    let mut ofsorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut oppleg: f32 = 0.;
    let mut adjleg: f32 = 0.;
    let mut length: f32 = 0.;
    let mut i: i32 = 0;
    if stereoSep == 0f32 && xmin == -xmax {
        // symmetric case can be simplified
        ofsorigin[0] = (*dest).or.origin[0];
        ofsorigin[1] = (*dest).or.origin[1];
        ofsorigin[2] = (*dest).or.origin[2];
        length = crate::stdlib::sqrt((xmax * xmax + zProj * zProj) as f64) as f32;
        oppleg = xmax / length;
        adjleg = zProj / length;
        (*dest).frustum[0].normal[0] = (*dest).or.axis[0][0] * oppleg;
        (*dest).frustum[0].normal[1] = (*dest).or.axis[0][1] * oppleg;
        (*dest).frustum[0].normal[2] = (*dest).or.axis[0][2] * oppleg;
        (*dest).frustum[0].normal[0] =
            (*dest).frustum[0].normal[0] + (*dest).or.axis[1][0] * adjleg;
        (*dest).frustum[0].normal[1] =
            (*dest).frustum[0].normal[1] + (*dest).or.axis[1][1] * adjleg;
        (*dest).frustum[0].normal[2] =
            (*dest).frustum[0].normal[2] + (*dest).or.axis[1][2] * adjleg;
        (*dest).frustum[1].normal[0] = (*dest).or.axis[0][0] * oppleg;
        (*dest).frustum[1].normal[1] = (*dest).or.axis[0][1] * oppleg;
        (*dest).frustum[1].normal[2] = (*dest).or.axis[0][2] * oppleg;
        (*dest).frustum[1].normal[0] =
            (*dest).frustum[1].normal[0] + (*dest).or.axis[1][0] * -adjleg;
        (*dest).frustum[1].normal[1] =
            (*dest).frustum[1].normal[1] + (*dest).or.axis[1][1] * -adjleg;
        (*dest).frustum[1].normal[2] =
            (*dest).frustum[1].normal[2] + (*dest).or.axis[1][2] * -adjleg
    } else {
        // In stereo rendering, due to the modification of the projection matrix, dest->or.origin is not the
        // actual origin that we're rendering so offset the tip of the view pyramid.
        ofsorigin[0] = (*dest).or.origin[0] + (*dest).or.axis[1][0] * stereoSep;
        ofsorigin[1] = (*dest).or.origin[1] + (*dest).or.axis[1][1] * stereoSep;
        ofsorigin[2] = (*dest).or.origin[2] + (*dest).or.axis[1][2] * stereoSep;
        oppleg = xmax + stereoSep;
        length = crate::stdlib::sqrt((oppleg * oppleg + zProj * zProj) as f64) as f32;
        (*dest).frustum[0].normal[0] = (*dest).or.axis[0][0] * (oppleg / length);
        (*dest).frustum[0].normal[1] = (*dest).or.axis[0][1] * (oppleg / length);
        (*dest).frustum[0].normal[2] = (*dest).or.axis[0][2] * (oppleg / length);
        (*dest).frustum[0].normal[0] =
            (*dest).frustum[0].normal[0] + (*dest).or.axis[1][0] * (zProj / length);
        (*dest).frustum[0].normal[1] =
            (*dest).frustum[0].normal[1] + (*dest).or.axis[1][1] * (zProj / length);
        (*dest).frustum[0].normal[2] =
            (*dest).frustum[0].normal[2] + (*dest).or.axis[1][2] * (zProj / length);
        oppleg = xmin + stereoSep;
        length = crate::stdlib::sqrt((oppleg * oppleg + zProj * zProj) as f64) as f32;
        (*dest).frustum[1].normal[0] = (*dest).or.axis[0][0] * (-oppleg / length);
        (*dest).frustum[1].normal[1] = (*dest).or.axis[0][1] * (-oppleg / length);
        (*dest).frustum[1].normal[2] = (*dest).or.axis[0][2] * (-oppleg / length);
        (*dest).frustum[1].normal[0] =
            (*dest).frustum[1].normal[0] + (*dest).or.axis[1][0] * (-zProj / length);
        (*dest).frustum[1].normal[1] =
            (*dest).frustum[1].normal[1] + (*dest).or.axis[1][1] * (-zProj / length);
        (*dest).frustum[1].normal[2] =
            (*dest).frustum[1].normal[2] + (*dest).or.axis[1][2] * (-zProj / length)
    }
    length = crate::stdlib::sqrt((ymax * ymax + zProj * zProj) as f64) as f32;
    oppleg = ymax / length;
    adjleg = zProj / length;
    (*dest).frustum[2].normal[0] = (*dest).or.axis[0][0] * oppleg;
    (*dest).frustum[2].normal[1] = (*dest).or.axis[0][1] * oppleg;
    (*dest).frustum[2].normal[2] = (*dest).or.axis[0][2] * oppleg;
    (*dest).frustum[2].normal[0] = (*dest).frustum[2].normal[0] + (*dest).or.axis[2][0] * adjleg;
    (*dest).frustum[2].normal[1] = (*dest).frustum[2].normal[1] + (*dest).or.axis[2][1] * adjleg;
    (*dest).frustum[2].normal[2] = (*dest).frustum[2].normal[2] + (*dest).or.axis[2][2] * adjleg;
    (*dest).frustum[3].normal[0] = (*dest).or.axis[0][0] * oppleg;
    (*dest).frustum[3].normal[1] = (*dest).or.axis[0][1] * oppleg;
    (*dest).frustum[3].normal[2] = (*dest).or.axis[0][2] * oppleg;
    (*dest).frustum[3].normal[0] = (*dest).frustum[3].normal[0] + (*dest).or.axis[2][0] * -adjleg;
    (*dest).frustum[3].normal[1] = (*dest).frustum[3].normal[1] + (*dest).or.axis[2][1] * -adjleg;
    (*dest).frustum[3].normal[2] = (*dest).frustum[3].normal[2] + (*dest).or.axis[2][2] * -adjleg;
    i = 0;
    while i < 4 {
        (*dest).frustum[i as usize].type_0 = 3u8;
        (*dest).frustum[i as usize].dist = ofsorigin[0] * (*dest).frustum[i as usize].normal[0]
            + ofsorigin[1] * (*dest).frustum[i as usize].normal[1]
            + ofsorigin[2] * (*dest).frustum[i as usize].normal[2];
        crate::src::qcommon::q_math::SetPlaneSignbits(
            &mut *(*dest).frustum.as_mut_ptr().offset(i as isize),
        );
        i += 1
    }
}
// completely unclipped
// clipped by one or more planes
// completely outside the clipping planes
/*
===============
R_SetupProjection
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SetupProjection(
    mut dest: *mut crate::tr_local_h::viewParms_t,
    mut zProj: f32,
    mut computeFrustum: crate::src::qcommon::q_shared::qboolean,
) {
    let mut xmin: f32 = 0.;
    let mut xmax: f32 = 0.;
    let mut ymin: f32 = 0.;
    let mut ymax: f32 = 0.;
    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    let mut stereoSep: f32 = (*crate::src::renderergl1::tr_init::r_stereoSeparation).value;
    /*
     * offset the view origin of the viewer for stereo rendering
     * by setting the projection matrix appropriately.
     */
    if stereoSep != 0f32 {
        if (*dest).stereoFrame == crate::tr_types_h::STEREO_LEFT {
            stereoSep = zProj / stereoSep
        } else if (*dest).stereoFrame == crate::tr_types_h::STEREO_RIGHT {
            stereoSep = zProj / -stereoSep
        } else {
            stereoSep = 0f32
        }
    } // normally 0
    ymax = (zProj as f64
        * crate::stdlib::tan((*dest).fovY as f64 * 3.14159265358979323846 / 360f64))
        as f32;
    ymin = -ymax;
    xmax = (zProj as f64
        * crate::stdlib::tan((*dest).fovX as f64 * 3.14159265358979323846 / 360f64))
        as f32;
    xmin = -xmax;
    width = xmax - xmin;
    height = ymax - ymin;
    (*dest).projectionMatrix[0] = 2f32 * zProj / width;
    (*dest).projectionMatrix[4] = 0f32;
    (*dest).projectionMatrix[8] = (xmax + xmin + 2f32 * stereoSep) / width;
    (*dest).projectionMatrix[12] = 2f32 * zProj * stereoSep / width;
    (*dest).projectionMatrix[1] = 0f32;
    (*dest).projectionMatrix[5] = 2f32 * zProj / height;
    (*dest).projectionMatrix[9] = (ymax + ymin) / height;
    (*dest).projectionMatrix[13] = 0f32;
    (*dest).projectionMatrix[3] = 0f32;
    (*dest).projectionMatrix[7] = 0f32;
    (*dest).projectionMatrix[11] = -1f32;
    (*dest).projectionMatrix[15] = 0f32;
    // Now that we have all the data for the projection matrix we can also setup the view frustum.
    if computeFrustum as u64 != 0 {
        R_SetupFrustum(dest, xmin, xmax, ymax, zProj, stereoSep);
    };
}
/*
===============
R_SetupProjectionZ

Sets the z-component transformation part in the projection matrix
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SetupProjectionZ(mut dest: *mut crate::tr_local_h::viewParms_t) {
    let mut zNear: f32 = 0.;
    let mut zFar: f32 = 0.;
    let mut depth: f32 = 0.;
    zNear = (*crate::src::renderergl1::tr_init::r_znear).value;
    zFar = (*dest).zFar;
    depth = zFar - zNear;
    (*dest).projectionMatrix[2] = 0f32;
    (*dest).projectionMatrix[6] = 0f32;
    (*dest).projectionMatrix[10] = -(zFar + zNear) / depth;
    (*dest).projectionMatrix[14] = -2f32 * zFar * zNear / depth;
}
/*
=================
R_MirrorPoint
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_MirrorPoint(
    mut in_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut surface: *mut crate::src::qcommon::q_shared::orientation_t,
    mut camera: *mut crate::src::qcommon::q_shared::orientation_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut _i: i32 = 0;
    let mut local: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut transformed: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: f32 = 0.;
    local[0] = *in_0.offset(0) - (*surface).origin[0];
    local[1] = *in_0.offset(1) - (*surface).origin[1];
    local[2] = *in_0.offset(2) - (*surface).origin[2];
    transformed[2] = 0f32;
    transformed[1] = transformed[2];
    transformed[0] = transformed[1];

    for i in 0..3 {
        d = local[0] * (*surface).axis[i as usize][0]
            + local[1] * (*surface).axis[i as usize][1]
            + local[2] * (*surface).axis[i as usize][2];

        transformed[0] = transformed[0] + (*camera).axis[i as usize][0] * d;

        transformed[1] = transformed[1] + (*camera).axis[i as usize][1] * d;

        transformed[2] = transformed[2] + (*camera).axis[i as usize][2] * d;
    }
    *out.offset(0) = transformed[0] + (*camera).origin[0];
    *out.offset(1) = transformed[1] + (*camera).origin[1];
    *out.offset(2) = transformed[2] + (*camera).origin[2];
}
#[no_mangle]

pub unsafe extern "C" fn R_MirrorVector(
    mut in_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut surface: *mut crate::src::qcommon::q_shared::orientation_t,
    mut camera: *mut crate::src::qcommon::q_shared::orientation_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: i32 = 0;
    let mut d: f32 = 0.;
    let ref mut fresh0 = *out.offset(2);
    *fresh0 = 0f32;
    let ref mut fresh1 = *out.offset(1);
    *fresh1 = *fresh0;
    *out.offset(0) = *fresh1;
    i = 0;
    while i < 3 {
        d = *in_0.offset(0) * (*surface).axis[i as usize][0]
            + *in_0.offset(1) * (*surface).axis[i as usize][1]
            + *in_0.offset(2) * (*surface).axis[i as usize][2];
        *out.offset(0) = *out.offset(0) + (*camera).axis[i as usize][0] * d;
        *out.offset(1) = *out.offset(1) + (*camera).axis[i as usize][1] * d;
        *out.offset(2) = *out.offset(2) + (*camera).axis[i as usize][2] * d;
        i += 1
    }
}
/*
=============
R_PlaneForSurface
=============
*/
#[no_mangle]

pub unsafe extern "C" fn R_PlaneForSurface(
    mut surfType: *mut crate::tr_local_h::surfaceType_t,
    mut plane: *mut crate::src::qcommon::q_shared::cplane_t,
) {
    let mut tri: *mut crate::tr_local_h::srfTriangles_t =
        0 as *mut crate::tr_local_h::srfTriangles_t;
    let mut poly: *mut crate::tr_local_h::srfPoly_t = 0 as *mut crate::tr_local_h::srfPoly_t;
    let mut v1: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut v2: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut v3: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut plane4: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    if surfType.is_null() {
        crate::stdlib::memset(
            plane as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::cplane_t>(),
        );
        (*plane).normal[0] = 1f32;
        return;
    }
    match *surfType {
        2 => {
            *plane = (*(surfType as *mut crate::tr_local_h::srfSurfaceFace_t)).plane;
            return;
        }
        4 => {
            tri = surfType as *mut crate::tr_local_h::srfTriangles_t;
            v1 = (*tri).verts.offset(*(*tri).indexes.offset(0) as isize);
            v2 = (*tri).verts.offset(*(*tri).indexes.offset(1) as isize);
            v3 = (*tri).verts.offset(*(*tri).indexes.offset(2) as isize);
            crate::src::qcommon::q_math::PlaneFromPoints(
                plane4.as_mut_ptr(),
                (*v1).xyz.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*v2).xyz.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*v3).xyz.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            );
            (*plane).normal[0] = plane4[0];
            (*plane).normal[1] = plane4[1];
            (*plane).normal[2] = plane4[2];
            (*plane).dist = plane4[3];
            return;
        }
        5 => {
            poly = surfType as *mut crate::tr_local_h::srfPoly_t;
            crate::src::qcommon::q_math::PlaneFromPoints(
                plane4.as_mut_ptr(),
                (*(*poly).verts.offset(0)).xyz.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                (*(*poly).verts.offset(1)).xyz.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                (*(*poly).verts.offset(2)).xyz.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
            );
            (*plane).normal[0] = plane4[0];
            (*plane).normal[1] = plane4[1];
            (*plane).normal[2] = plane4[2];
            (*plane).dist = plane4[3];
            return;
        }
        _ => {
            crate::stdlib::memset(
                plane as *mut libc::c_void,
                0,
                ::std::mem::size_of::<crate::src::qcommon::q_shared::cplane_t>(),
            );
            (*plane).normal[0] = 1f32;
            return;
        }
    };
}
/*
=================
R_GetPortalOrientation

entityNum is the entity that the portal surface is a part of, which may
be moving and rotating.

Returns qtrue if it should be mirrored
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetPortalOrientations(
    mut drawSurf: *mut crate::tr_local_h::drawSurf_t,
    mut entityNum: i32,
    mut surface: *mut crate::src::qcommon::q_shared::orientation_t,
    mut camera: *mut crate::src::qcommon::q_shared::orientation_t,
    mut pvsOrigin: *mut crate::src::qcommon::q_shared::vec_t,
    mut mirror: *mut crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut _i: i32 = 0;
    let mut originalPlane: crate::src::qcommon::q_shared::cplane_t =
        crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        };
    let mut plane: crate::src::qcommon::q_shared::cplane_t =
        crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        };
    let mut e: *mut crate::tr_local_h::trRefEntity_t = 0 as *mut crate::tr_local_h::trRefEntity_t;
    let mut d: f32 = 0.;
    let mut transformed: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // create plane axis for the portal we are seeing
    R_PlaneForSurface((*drawSurf).surface, &mut originalPlane);
    // rotate the plane if necessary
    if entityNum != ((1) << 10) - 1 {
        tr.currentEntityNum = entityNum;
        tr.currentEntity = &mut *tr.refdef.entities.offset(entityNum as isize)
            as *mut crate::tr_local_h::trRefEntity_t;
        // get the orientation of the entity
        R_RotateForEntity(tr.currentEntity, &mut tr.viewParms, &mut tr.or);
        // rotate the plane, but keep the non-rotated version for matching
        // against the portalSurface entities
        R_LocalNormalToWorld(originalPlane.normal.as_mut_ptr(), plane.normal.as_mut_ptr());
        plane.dist = originalPlane.dist
            + (plane.normal[0] * tr.or.origin[0]
                + plane.normal[1] * tr.or.origin[1]
                + plane.normal[2] * tr.or.origin[2]);
        // translate the original plane
        originalPlane.dist = originalPlane.dist
            + (originalPlane.normal[0] * tr.or.origin[0]
                + originalPlane.normal[1] * tr.or.origin[1]
                + originalPlane.normal[2] * tr.or.origin[2])
    } else {
        plane = originalPlane
    }
    (*surface).axis[0][0] = plane.normal[0];
    (*surface).axis[0][1] = plane.normal[1];
    (*surface).axis[0][2] = plane.normal[2];
    crate::src::qcommon::q_math::PerpendicularVector(
        (*surface).axis[1].as_mut_ptr(),
        (*surface).axis[0].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    CrossProduct(
        (*surface).axis[0].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*surface).axis[1].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*surface).axis[2].as_mut_ptr(),
    );
    // locate the portal entity closest to this plane.
    // origin will be the origin of the portal, origin2 will be
    // the origin of the camera

    for i in 0..tr.refdef.num_entities {
        e = &mut *tr.refdef.entities.offset(i as isize) as *mut crate::tr_local_h::trRefEntity_t;

        if !((*e).e.reType != crate::tr_types_h::RT_PORTALSURFACE) {
            d = (*e).e.origin[0] * originalPlane.normal[0]
                + (*e).e.origin[1] * originalPlane.normal[1]
                + (*e).e.origin[2] * originalPlane.normal[2]
                - originalPlane.dist;
            if !(d > 64f32 || d < -64f32) {
                // get the pvsOrigin from the entity
                *pvsOrigin.offset(0) = (*e).e.oldorigin[0];
                *pvsOrigin.offset(1) = (*e).e.oldorigin[1];
                *pvsOrigin.offset(2) = (*e).e.oldorigin[2];
                // if the entity is just a mirror, don't use as a camera point
                if (*e).e.oldorigin[0] == (*e).e.origin[0]
                    && (*e).e.oldorigin[1] == (*e).e.origin[1]
                    && (*e).e.oldorigin[2] == (*e).e.origin[2]
                {
                    (*surface).origin[0] = plane.normal[0] * plane.dist;
                    (*surface).origin[1] = plane.normal[1] * plane.dist;
                    (*surface).origin[2] = plane.normal[2] * plane.dist;
                    (*camera).origin[0] = (*surface).origin[0];
                    (*camera).origin[1] = (*surface).origin[1];
                    (*camera).origin[2] = (*surface).origin[2];
                    (*camera).axis[0][0] =
                        crate::src::qcommon::q_math::vec3_origin[0] - (*surface).axis[0][0];
                    (*camera).axis[0][1] =
                        crate::src::qcommon::q_math::vec3_origin[1] - (*surface).axis[0][1];
                    (*camera).axis[0][2] =
                        crate::src::qcommon::q_math::vec3_origin[2] - (*surface).axis[0][2];
                    (*camera).axis[1][0] = (*surface).axis[1][0];
                    (*camera).axis[1][1] = (*surface).axis[1][1];
                    (*camera).axis[1][2] = (*surface).axis[1][2];
                    (*camera).axis[2][0] = (*surface).axis[2][0];
                    (*camera).axis[2][1] = (*surface).axis[2][1];
                    (*camera).axis[2][2] = (*surface).axis[2][2];
                    *mirror = crate::src::qcommon::q_shared::qtrue;
                    return crate::src::qcommon::q_shared::qtrue;
                }
                // project the origin onto the surface plane to get
                // an origin point we can rotate around
                d = (*e).e.origin[0] * plane.normal[0]
                    + (*e).e.origin[1] * plane.normal[1]
                    + (*e).e.origin[2] * plane.normal[2]
                    - plane.dist;
                (*surface).origin[0] = (*e).e.origin[0] + (*surface).axis[0][0] * -d;
                (*surface).origin[1] = (*e).e.origin[1] + (*surface).axis[0][1] * -d;
                (*surface).origin[2] = (*e).e.origin[2] + (*surface).axis[0][2] * -d;
                // now get the camera origin and orientation
                (*camera).origin[0] = (*e).e.oldorigin[0];
                (*camera).origin[1] = (*e).e.oldorigin[1];
                (*camera).origin[2] = (*e).e.oldorigin[2];
                crate::src::qcommon::q_math::AxisCopy(
                    (*e).e.axis.as_mut_ptr(),
                    (*camera).axis.as_mut_ptr(),
                );
                (*camera).axis[0][0] =
                    crate::src::qcommon::q_math::vec3_origin[0] - (*camera).axis[0][0];
                (*camera).axis[0][1] =
                    crate::src::qcommon::q_math::vec3_origin[1] - (*camera).axis[0][1];
                (*camera).axis[0][2] =
                    crate::src::qcommon::q_math::vec3_origin[2] - (*camera).axis[0][2];
                (*camera).axis[1][0] =
                    crate::src::qcommon::q_math::vec3_origin[0] - (*camera).axis[1][0];
                (*camera).axis[1][1] =
                    crate::src::qcommon::q_math::vec3_origin[1] - (*camera).axis[1][1];
                (*camera).axis[1][2] =
                    crate::src::qcommon::q_math::vec3_origin[2] - (*camera).axis[1][2];
                // optionally rotate
                if (*e).e.oldframe != 0 {
                    // if a speed is specified
                    if (*e).e.frame != 0 {
                        // continuous rotate
                        d = tr.refdef.time as f32 / 1000.0 * (*e).e.frame as f32;
                        transformed[0] = (*camera).axis[1][0];
                        transformed[1] = (*camera).axis[1][1];
                        transformed[2] = (*camera).axis[1][2];
                        crate::src::qcommon::q_math::RotatePointAroundVector(
                            (*camera).axis[1].as_mut_ptr(),
                            (*camera).axis[0].as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            transformed.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            d,
                        );
                        CrossProduct(
                            (*camera).axis[0usize].as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            (*camera).axis[1usize].as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            (*camera).axis[2usize].as_mut_ptr(),
                        );
                    } else {
                        // bobbing rotate, with skinNum being the rotation offset
                        d = crate::stdlib::sin((tr.refdef.time as f32 * 0.003) as f64) as f32;
                        d = (*e).e.skinNum as f32 + d * 4f32;
                        transformed[0] = (*camera).axis[1][0];
                        transformed[1] = (*camera).axis[1][1];
                        transformed[2] = (*camera).axis[1][2];
                        crate::src::qcommon::q_math::RotatePointAroundVector(
                            (*camera).axis[1].as_mut_ptr(),
                            (*camera).axis[0].as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            transformed.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            d,
                        );
                        CrossProduct(
                            (*camera).axis[0usize].as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            (*camera).axis[1usize].as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            (*camera).axis[2usize].as_mut_ptr(),
                        );
                    }
                } else if (*e).e.skinNum != 0 {
                    d = (*e).e.skinNum as f32;
                    transformed[0] = (*camera).axis[1][0];
                    transformed[1] = (*camera).axis[1][1];
                    transformed[2] = (*camera).axis[1][2];
                    crate::src::qcommon::q_math::RotatePointAroundVector(
                        (*camera).axis[1].as_mut_ptr(),
                        (*camera).axis[0].as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        transformed.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        d,
                    );
                    CrossProduct(
                        (*camera).axis[0usize].as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        (*camera).axis[1usize].as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        (*camera).axis[2usize].as_mut_ptr(),
                    );
                }
                *mirror = crate::src::qcommon::q_shared::qfalse;
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
    }
    // if we didn't locate a portal entity, don't render anything.
    // We don't want to just treat it as a mirror, because without a
    // portal entity the server won't have communicated a proper entity set
    // in the snapshot
    // unfortunately, with local movement prediction it is easily possible
    // to see a surface before the server has communicated the matching
    // portal surface entity, so we don't want to print anything here...
    //ri.Printf( PRINT_ALL, "Portal surface without a portal entity\n" );
    return crate::src::qcommon::q_shared::qfalse;
}

unsafe extern "C" fn IsMirror(
    mut drawSurf: *const crate::tr_local_h::drawSurf_t,
    mut entityNum: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut _i: i32 = 0;
    let mut originalPlane: crate::src::qcommon::q_shared::cplane_t =
        crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        };
    let mut plane: crate::src::qcommon::q_shared::cplane_t =
        crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        };
    let mut e: *mut crate::tr_local_h::trRefEntity_t = 0 as *mut crate::tr_local_h::trRefEntity_t;
    let mut d: f32 = 0.;
    // create plane axis for the portal we are seeing
    R_PlaneForSurface((*drawSurf).surface, &mut originalPlane);
    // rotate the plane if necessary
    if entityNum != ((1) << 10) - 1 {
        tr.currentEntityNum = entityNum;
        tr.currentEntity = &mut *tr.refdef.entities.offset(entityNum as isize)
            as *mut crate::tr_local_h::trRefEntity_t;
        // get the orientation of the entity
        R_RotateForEntity(tr.currentEntity, &mut tr.viewParms, &mut tr.or);
        // rotate the plane, but keep the non-rotated version for matching
        // against the portalSurface entities
        R_LocalNormalToWorld(originalPlane.normal.as_mut_ptr(), plane.normal.as_mut_ptr());
        plane.dist = originalPlane.dist
            + (plane.normal[0] * tr.or.origin[0]
                + plane.normal[1] * tr.or.origin[1]
                + plane.normal[2] * tr.or.origin[2]);
        // translate the original plane
        originalPlane.dist = originalPlane.dist
            + (originalPlane.normal[0] * tr.or.origin[0]
                + originalPlane.normal[1] * tr.or.origin[1]
                + originalPlane.normal[2] * tr.or.origin[2])
    }
    // locate the portal entity closest to this plane.
    // origin will be the origin of the portal, origin2 will be
    // the origin of the camera

    for i in 0..tr.refdef.num_entities {
        e = &mut *tr.refdef.entities.offset(i as isize) as *mut crate::tr_local_h::trRefEntity_t;

        if !((*e).e.reType != crate::tr_types_h::RT_PORTALSURFACE) {
            d = (*e).e.origin[0] * originalPlane.normal[0]
                + (*e).e.origin[1] * originalPlane.normal[1]
                + (*e).e.origin[2] * originalPlane.normal[2]
                - originalPlane.dist;
            if !(d > 64f32 || d < -64f32) {
                // if the entity is just a mirror, don't use as a camera point
                if (*e).e.oldorigin[0] == (*e).e.origin[0]
                    && (*e).e.oldorigin[1] == (*e).e.origin[1]
                    && (*e).e.oldorigin[2] == (*e).e.origin[2]
                {
                    return crate::src::qcommon::q_shared::qtrue;
                }
                return crate::src::qcommon::q_shared::qfalse;
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
** SurfIsOffscreen
**
** Determines if a surface is completely offscreen.
*/

unsafe extern "C" fn SurfIsOffscreen(
    mut drawSurf: *const crate::tr_local_h::drawSurf_t,
    mut _clipDest: *mut crate::src::qcommon::q_shared::vec4_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut shortest: f32 = 100000000f32;
    let mut entityNum: i32 = 0;
    let mut numTriangles: i32 = 0;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut fogNum: i32 = 0;
    let mut dlighted: i32 = 0;
    let mut clip: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut eye: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut i: i32 = 0;
    let mut pointOr: u32 = 0;
    let mut pointAnd: u32 = !(0i32) as u32;
    R_RotateForViewer();
    R_DecomposeSort(
        (*drawSurf).sort,
        &mut entityNum,
        &mut shader,
        &mut fogNum,
        &mut dlighted,
    );
    crate::src::renderergl1::tr_shade::RB_BeginSurface(shader, fogNum);
    crate::src::renderergl1::tr_surface::rb_surfaceTable[*(*drawSurf).surface as usize]
        .expect("non-null function pointer")((*drawSurf).surface as *mut libc::c_void);
    i = 0;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut _j: i32 = 0;
        let mut pointFlags: u32 = 0;
        R_TransformModelToClip(
            crate::src::renderergl1::tr_shade::tess.xyz[i as usize].as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            tr.or.modelMatrix.as_mut_ptr(),
            tr.viewParms.projectionMatrix.as_mut_ptr(),
            eye.as_mut_ptr(),
            clip.as_mut_ptr(),
        );

        for j in 0..3 {
            if clip[j as usize] >= clip[3] {
                pointFlags |= ((1i32) << j * 2) as u32
            } else if clip[j as usize] <= -clip[3] {
                pointFlags |= ((1i32) << j * 2 + 1) as u32
            }
        }
        pointAnd &= pointFlags;
        pointOr |= pointFlags;
        i += 1
    }
    // trivially reject
    if pointAnd != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    // determine if this surface is backfaced and also determine the distance
    // to the nearest vertex so we can cull based on portal range.  Culling
    // based on vertex distance isn't 100% correct (we should be checking for
    // range to the surface), but it's good enough for the types of portals
    // we have in the game right now.
    numTriangles = crate::src::renderergl1::tr_shade::tess.numIndexes / 3; // lose the sqrt
    i = 0;
    while i < crate::src::renderergl1::tr_shade::tess.numIndexes {
        let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut len: f32 = 0.;
        normal[0] = crate::src::renderergl1::tr_shade::tess.xyz
            [crate::src::renderergl1::tr_shade::tess.indexes[i as usize] as usize][0]
            - tr.viewParms.or.origin[0];
        normal[1] = crate::src::renderergl1::tr_shade::tess.xyz
            [crate::src::renderergl1::tr_shade::tess.indexes[i as usize] as usize][1]
            - tr.viewParms.or.origin[1];
        normal[2] = crate::src::renderergl1::tr_shade::tess.xyz
            [crate::src::renderergl1::tr_shade::tess.indexes[i as usize] as usize][2]
            - tr.viewParms.or.origin[2];
        len =
            VectorLengthSquared(normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if len < shortest {
            shortest = len
        }
        if normal[0]
            * crate::src::renderergl1::tr_shade::tess.normal
                [crate::src::renderergl1::tr_shade::tess.indexes[i as usize] as usize][0]
            + normal[1]
                * crate::src::renderergl1::tr_shade::tess.normal
                    [crate::src::renderergl1::tr_shade::tess.indexes[i as usize] as usize][1]
            + normal[2]
                * crate::src::renderergl1::tr_shade::tess.normal
                    [crate::src::renderergl1::tr_shade::tess.indexes[i as usize] as usize][2]
            >= 0f32
        {
            numTriangles -= 1
        }
        i += 3
    }
    if numTriangles == 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    // mirrors can early out at this point, since we don't do a fade over distance
    // with them (although we could)
    if IsMirror(drawSurf, entityNum) as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if shortest
        > (*crate::src::renderergl1::tr_shade::tess.shader).portalRange
            * (*crate::src::renderergl1::tr_shade::tess.shader).portalRange
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
========================
R_MirrorViewBySurface

Returns qtrue if another view has been rendered
========================
*/
#[no_mangle]

pub unsafe extern "C" fn R_MirrorViewBySurface(
    mut drawSurf: *mut crate::tr_local_h::drawSurf_t,
    mut entityNum: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut clipDest: [crate::src::qcommon::q_shared::vec4_t; 128] = [[0.; 4]; 128];
    let mut newParms: crate::tr_local_h::viewParms_t = crate::tr_local_h::viewParms_t {
        or: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        world: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        pvsOrigin: [0.; 3],
        isPortal: crate::src::qcommon::q_shared::qfalse,
        isMirror: crate::src::qcommon::q_shared::qfalse,
        frameSceneNum: 0,
        frameCount: 0,
        portalPlane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        viewportX: 0,
        viewportY: 0,
        viewportWidth: 0,
        viewportHeight: 0,
        fovX: 0.,
        fovY: 0.,
        projectionMatrix: [0.; 16],
        frustum: [crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        }; 4],
        visBounds: [[0.; 3]; 2],
        zFar: 0.,
        stereoFrame: crate::tr_types_h::STEREO_CENTER,
    };
    let mut oldParms: crate::tr_local_h::viewParms_t = crate::tr_local_h::viewParms_t {
        or: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        world: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        pvsOrigin: [0.; 3],
        isPortal: crate::src::qcommon::q_shared::qfalse,
        isMirror: crate::src::qcommon::q_shared::qfalse,
        frameSceneNum: 0,
        frameCount: 0,
        portalPlane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        viewportX: 0,
        viewportY: 0,
        viewportWidth: 0,
        viewportHeight: 0,
        fovX: 0.,
        fovY: 0.,
        projectionMatrix: [0.; 16],
        frustum: [crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        }; 4],
        visBounds: [[0.; 3]; 2],
        zFar: 0.,
        stereoFrame: crate::tr_types_h::STEREO_CENTER,
    };
    let mut surface: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    let mut camera: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    // don't recursively mirror
    if tr.viewParms.isPortal as u64 != 0 {
        ri.Printf.expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"WARNING: recursive mirror/portal found\n\x00" as *const u8 as *const i8,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*crate::src::renderergl1::tr_init::r_noportals).integer != 0
        || (*crate::src::renderergl1::tr_init::r_fastsky).integer == 1
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // trivially reject portal/mirror
    if SurfIsOffscreen(drawSurf, clipDest.as_mut_ptr()) as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // save old viewParms so we can return to it after the mirror view
    oldParms = tr.viewParms;
    newParms = tr.viewParms;
    newParms.isPortal = crate::src::qcommon::q_shared::qtrue;
    if R_GetPortalOrientations(
        drawSurf,
        entityNum,
        &mut surface,
        &mut camera,
        newParms.pvsOrigin.as_mut_ptr(),
        &mut newParms.isMirror,
    ) as u64
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
        // bad portal, no portalentity
    }
    R_MirrorPoint(
        oldParms.or.origin.as_mut_ptr(),
        &mut surface,
        &mut camera,
        newParms.or.origin.as_mut_ptr(),
    );
    newParms.portalPlane.normal[0] =
        crate::src::qcommon::q_math::vec3_origin[0] - camera.axis[0][0];
    newParms.portalPlane.normal[1] =
        crate::src::qcommon::q_math::vec3_origin[1] - camera.axis[0][1];
    newParms.portalPlane.normal[2] =
        crate::src::qcommon::q_math::vec3_origin[2] - camera.axis[0][2];
    newParms.portalPlane.dist = camera.origin[0] * newParms.portalPlane.normal[0]
        + camera.origin[1] * newParms.portalPlane.normal[1]
        + camera.origin[2] * newParms.portalPlane.normal[2];
    R_MirrorVector(
        oldParms.or.axis[0].as_mut_ptr(),
        &mut surface,
        &mut camera,
        newParms.or.axis[0].as_mut_ptr(),
    );
    R_MirrorVector(
        oldParms.or.axis[1].as_mut_ptr(),
        &mut surface,
        &mut camera,
        newParms.or.axis[1].as_mut_ptr(),
    );
    R_MirrorVector(
        oldParms.or.axis[2].as_mut_ptr(),
        &mut surface,
        &mut camera,
        newParms.or.axis[2].as_mut_ptr(),
    );
    // OPTIMIZE: restrict the viewport on the mirrored view
    // render the mirror view
    R_RenderView(&mut newParms);
    tr.viewParms = oldParms;
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
R_SpriteFogNum

See if a sprite is inside a fog volume
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_SpriteFogNum(mut ent: *mut crate::tr_local_h::trRefEntity_t) -> i32 {
    let mut _i: i32 = 0;
    let mut j: i32 = 0;
    let mut fog: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
    if tr.refdef.rdflags & 0x1 != 0 {
        return 0i32;
    }
    if (*ent).e.renderfx & 0x10 != 0 {
        return 0i32;
    }

    for i in 1..(*tr.world).numfogs {
        fog = &mut *(*tr.world).fogs.offset(i as isize) as *mut crate::tr_local_h::fog_t;

        j = 0;

        while j < 3 {
            if (*ent).e.origin[j as usize] - (*ent).e.radius >= (*fog).bounds[1][j as usize] {
                break;
            }
            if (*ent).e.origin[j as usize] + (*ent).e.radius <= (*fog).bounds[0][j as usize] {
                break;
            }
            j += 1
        }

        if j == 3 {
            return i;
        }
    }
    return 0;
}
/*
==========================================================================================

DRAWSURF SORTING

==========================================================================================
*/
/*
===============
R_Radix
===============
*/
#[inline]

unsafe extern "C" fn R_Radix(
    mut byte: i32,
    mut size: i32,
    mut source: *mut crate::tr_local_h::drawSurf_t,
    mut dest: *mut crate::tr_local_h::drawSurf_t,
) {
    let mut count: [i32; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut index: [i32; 256] = [0; 256];
    let mut i: i32 = 0;
    let mut sortKey: *mut u8 = 0 as *mut u8;
    let mut end: *mut u8 = 0 as *mut u8;
    sortKey = (&mut (*source.offset(0)).sort as *mut u32 as *mut u8).offset(byte as isize);
    end = sortKey.offset(
        (size as usize).wrapping_mul(::std::mem::size_of::<crate::tr_local_h::drawSurf_t>())
            as isize,
    );
    while sortKey < end {
        count[*sortKey as usize] += 1;
        sortKey = sortKey.offset(::std::mem::size_of::<crate::tr_local_h::drawSurf_t>() as isize)
    }
    index[0] = 0;
    i = 1;
    while i < 256 {
        index[i as usize] = index[(i - 1) as usize] + count[(i - 1) as usize];
        i += 1
    }
    sortKey = (&mut (*source.offset(0)).sort as *mut u32 as *mut u8).offset(byte as isize);
    i = 0;
    while i < size {
        let fresh2 = index[*sortKey as usize];
        index[*sortKey as usize] = index[*sortKey as usize] + 1;
        *dest.offset(fresh2 as isize) = *source.offset(i as isize);
        i += 1;
        sortKey = sortKey.offset(::std::mem::size_of::<crate::tr_local_h::drawSurf_t>() as isize)
    }
}
/*
===============
R_RadixSort

Radix sort with 4 byte size buckets
===============
*/

unsafe extern "C" fn R_RadixSort(mut source: *mut crate::tr_local_h::drawSurf_t, mut size: i32) {
    static mut scratch: [crate::tr_local_h::drawSurf_t; 65536] = [crate::tr_local_h::drawSurf_t {
        sort: 0,
        surface: 0 as *mut crate::tr_local_h::surfaceType_t,
    }; 65536];
    R_Radix(0, size, source, scratch.as_mut_ptr());
    R_Radix(1, size, scratch.as_mut_ptr(), source);
    R_Radix(2, size, source, scratch.as_mut_ptr());
    R_Radix(3, size, scratch.as_mut_ptr(), source);
    //Q3_LITTLE_ENDIAN
}
//==========================================================================================
/*
=================
R_AddDrawSurf
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddDrawSurf(
    mut surface: *mut crate::tr_local_h::surfaceType_t,
    mut shader: *mut crate::tr_local_h::shader_t,
    mut fogIndex: i32,
    mut dlightMap: i32,
) {
    let mut index: i32 = 0;
    // instead of checking for overflow, we just mask the index
    // so it wraps around
    index = tr.refdef.numDrawSurfs & 0x10000 - 1;
    // the sort data is packed into a single 32 bit value so it can be
    // compared quickly during the qsorting process
    (*tr.refdef.drawSurfs.offset(index as isize)).sort =
        ((*shader).sortedIndex << 7 + 10 | tr.shiftedEntityNum | fogIndex << 2 | dlightMap) as u32;
    let ref mut fresh3 = (*tr.refdef.drawSurfs.offset(index as isize)).surface;
    *fresh3 = surface;
    tr.refdef.numDrawSurfs += 1;
}
/*
=================
R_DecomposeSort
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_DecomposeSort(
    mut sort: u32,
    mut entityNum: *mut i32,
    mut shader: *mut *mut crate::tr_local_h::shader_t,
    mut fogNum: *mut i32,
    mut dlightMap: *mut i32,
) {
    *fogNum = (sort >> 2 & 31u32) as i32;
    *shader = tr.sortedShaders[(sort >> 7 + 10 & (((1i32) << 14) - 1) as u32) as usize];
    *entityNum = (sort >> 7 & (((1i32) << 10) - 1) as u32) as i32;
    *dlightMap = (sort & 3u32) as i32;
}
/*
=================
R_SortDrawSurfs
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_SortDrawSurfs(
    mut drawSurfs: *mut crate::tr_local_h::drawSurf_t,
    mut numDrawSurfs: i32,
) {
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut fogNum: i32 = 0;
    let mut entityNum: i32 = 0;
    let mut dlighted: i32 = 0;
    let mut i: i32 = 0;
    // it is possible for some views to not have any surfaces
    if numDrawSurfs < 1 {
        // we still need to add it for hyperspace cases
        crate::src::renderergl1::tr_cmds::R_AddDrawSurfCmd(drawSurfs, numDrawSurfs);
        return;
    }
    // sort the drawsurfs by sort type, then orientation, then shader
    R_RadixSort(drawSurfs, numDrawSurfs);
    // check for any pass through drawing, which
    // may cause another view to be rendered first
    i = 0;
    while i < numDrawSurfs {
        R_DecomposeSort(
            (*drawSurfs.offset(i as isize)).sort,
            &mut entityNum,
            &mut shader,
            &mut fogNum,
            &mut dlighted,
        );
        if (*shader).sort > crate::tr_local_h::SS_PORTAL as i32 as f32 {
            break;
        }
        // no shader should ever have this sort type
        if (*shader).sort == crate::tr_local_h::SS_BAD as i32 as f32 {
            ri.Error.expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"Shader \'%s\'with sort == SS_BAD\x00" as *const u8 as *const i8,
                (*shader).name.as_mut_ptr(),
            );
        }
        // if the mirror was completely clipped away, we may need to check another surface
        if R_MirrorViewBySurface(drawSurfs.offset(i as isize), entityNum) as u64 != 0 {
            // this is a debug option to see exactly what is being mirrored
            if (*crate::src::renderergl1::tr_init::r_portalOnly).integer != 0 {
                return;
            }
            break;
        // only one mirror view at a time
        } else {
            i += 1
        }
    }
    crate::src::renderergl1::tr_cmds::R_AddDrawSurfCmd(drawSurfs, numDrawSurfs);
}
/*
=============
R_AddEntitySurfaces
=============
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddEntitySurfaces() {
    let mut ent: *mut crate::tr_local_h::trRefEntity_t = 0 as *mut crate::tr_local_h::trRefEntity_t;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    if (*crate::src::renderergl1::tr_init::r_drawentities).integer == 0 {
        return;
    }
    let mut current_block_22: u64;
    tr.currentEntityNum = 0;
    while tr.currentEntityNum < tr.refdef.num_entities {
        tr.currentEntity = &mut *tr.refdef.entities.offset(tr.currentEntityNum as isize)
            as *mut crate::tr_local_h::trRefEntity_t;
        ent = tr.currentEntity;
        (*ent).needDlights = crate::src::qcommon::q_shared::qfalse;
        // preshift the value we are going to OR into the drawsurf sort
        tr.shiftedEntityNum = tr.currentEntityNum << 7;
        //
        // the weapon model must be handled special --
        // we don't want the hacked weapon position showing in
        // mirrors, because the true body position will already be drawn
        //
        if !((*ent).e.renderfx & 0x4 != 0 && tr.viewParms.isPortal != 0) {
            // simple generated models, like sprites and beams, are not culled
            match (*ent).e.reType {
                7 => {}
                2 | 3 | 6 | 4 | 5 => {
                    current_block_22 = 15325006274910139791;
                    match current_block_22 {
                        5465853293512199010 => {
                            ri.Error.expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::ERR_DROP as i32,
                                b"R_AddEntitySurfaces: Bad reType\x00" as *const u8 as *const i8,
                            );
                        }
                        12015117824625712224 => {
                            // we must set up parts of tr.or for model culling
                            R_RotateForEntity(ent, &mut tr.viewParms, &mut tr.or);
                            tr.currentModel = crate::src::renderergl1::tr_model::R_GetModelByHandle(
                                (*ent).e.hModel,
                            );
                            if tr.currentModel.is_null() {
                                R_AddDrawSurf(&mut entitySurface, tr.defaultShader, 0i32, 0i32);
                            } else {
                                match (*tr.currentModel).type_0 {
                                    2 => {
                                        crate::src::renderergl1::tr_mesh::R_AddMD3Surfaces(ent);
                                    }
                                    3 => {
                                        crate::src::renderergl1::tr_animation::R_MDRAddAnimSurfaces(
                                            ent,
                                        );
                                    }
                                    4 => {
                                        crate::src::renderergl1::tr_model_iqm::R_AddIQMSurfaces(
                                            ent,
                                        );
                                    }
                                    1 => {
                                        crate::src::renderergl1::tr_world::R_AddBrushModelSurfaces(
                                            ent,
                                        );
                                    }
                                    0 => {
                                        // null model axis
                                        if !((*ent).e.renderfx & 0x2 != 0
                                            && tr.viewParms.isPortal as u64 == 0)
                                        {
                                            R_AddDrawSurf(
                                                &mut entitySurface,
                                                tr.defaultShader,
                                                0i32,
                                                0i32,
                                            );
                                        }
                                    }
                                    _ => {
                                        ri.Error.expect("non-null function pointer")(
                                            crate::src::qcommon::q_shared::ERR_DROP as i32,
                                            b"R_AddEntitySurfaces: Bad modeltype\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                }
                            }
                        }
                        _ =>
                        // self blood sprites, talk balloons, etc should not be drawn in the primary
                        // view.  We can't just do this check for all entities, because md3
                        // entities may still want to cast shadows from them
                        {
                            if !((*ent).e.renderfx & 0x2 != 0 && tr.viewParms.isPortal as u64 == 0)
                            {
                                shader = crate::src::renderergl1::tr_shader::R_GetShaderByHandle(
                                    (*ent).e.customShader,
                                ); // don't draw anything
                                R_AddDrawSurf(
                                    &mut entitySurface,
                                    shader,
                                    R_SpriteFogNum(ent),
                                    0i32,
                                );
                            }
                        }
                    }
                }
                0 => {
                    current_block_22 = 12015117824625712224;
                    match current_block_22 {
                        5465853293512199010 => {
                            ri.Error.expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::ERR_DROP as i32,
                                b"R_AddEntitySurfaces: Bad reType\x00" as *const u8 as *const i8,
                            );
                        }
                        12015117824625712224 => {
                            R_RotateForEntity(ent, &mut tr.viewParms, &mut tr.or);
                            tr.currentModel = crate::src::renderergl1::tr_model::R_GetModelByHandle(
                                (*ent).e.hModel,
                            );
                            if tr.currentModel.is_null() {
                                R_AddDrawSurf(&mut entitySurface, tr.defaultShader, 0i32, 0i32);
                            } else {
                                match (*tr.currentModel).type_0 {
                                    2 => {
                                        crate::src::renderergl1::tr_mesh::R_AddMD3Surfaces(ent);
                                    }
                                    3 => {
                                        crate::src::renderergl1::tr_animation::R_MDRAddAnimSurfaces(
                                            ent,
                                        );
                                    }
                                    4 => {
                                        crate::src::renderergl1::tr_model_iqm::R_AddIQMSurfaces(
                                            ent,
                                        );
                                    }
                                    1 => {
                                        crate::src::renderergl1::tr_world::R_AddBrushModelSurfaces(
                                            ent,
                                        );
                                    }
                                    0 => {
                                        if !((*ent).e.renderfx & 0x2 != 0
                                            && tr.viewParms.isPortal as u64 == 0)
                                        {
                                            R_AddDrawSurf(
                                                &mut entitySurface,
                                                tr.defaultShader,
                                                0i32,
                                                0i32,
                                            );
                                        }
                                    }
                                    _ => {
                                        ri.Error.expect("non-null function pointer")(
                                            crate::src::qcommon::q_shared::ERR_DROP as i32,
                                            b"R_AddEntitySurfaces: Bad modeltype\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            if !((*ent).e.renderfx & 0x2 != 0 && tr.viewParms.isPortal as u64 == 0)
                            {
                                shader = crate::src::renderergl1::tr_shader::R_GetShaderByHandle(
                                    (*ent).e.customShader,
                                );
                                R_AddDrawSurf(
                                    &mut entitySurface,
                                    shader,
                                    R_SpriteFogNum(ent),
                                    0i32,
                                );
                            }
                        }
                    }
                }
                _ => {
                    current_block_22 = 5465853293512199010;
                    match current_block_22 {
                        5465853293512199010 => {
                            ri.Error.expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::ERR_DROP as i32,
                                b"R_AddEntitySurfaces: Bad reType\x00" as *const u8 as *const i8,
                            );
                        }
                        12015117824625712224 => {
                            R_RotateForEntity(ent, &mut tr.viewParms, &mut tr.or);
                            tr.currentModel = crate::src::renderergl1::tr_model::R_GetModelByHandle(
                                (*ent).e.hModel,
                            );
                            if tr.currentModel.is_null() {
                                R_AddDrawSurf(&mut entitySurface, tr.defaultShader, 0i32, 0i32);
                            } else {
                                match (*tr.currentModel).type_0 {
                                    2 => {
                                        crate::src::renderergl1::tr_mesh::R_AddMD3Surfaces(ent);
                                    }
                                    3 => {
                                        crate::src::renderergl1::tr_animation::R_MDRAddAnimSurfaces(
                                            ent,
                                        );
                                    }
                                    4 => {
                                        crate::src::renderergl1::tr_model_iqm::R_AddIQMSurfaces(
                                            ent,
                                        );
                                    }
                                    1 => {
                                        crate::src::renderergl1::tr_world::R_AddBrushModelSurfaces(
                                            ent,
                                        );
                                    }
                                    0 => {
                                        if !((*ent).e.renderfx & 0x2 != 0
                                            && tr.viewParms.isPortal as u64 == 0)
                                        {
                                            R_AddDrawSurf(
                                                &mut entitySurface,
                                                tr.defaultShader,
                                                0i32,
                                                0i32,
                                            );
                                        }
                                    }
                                    _ => {
                                        ri.Error.expect("non-null function pointer")(
                                            crate::src::qcommon::q_shared::ERR_DROP as i32,
                                            b"R_AddEntitySurfaces: Bad modeltype\x00" as *const u8
                                                as *const i8,
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            if !((*ent).e.renderfx & 0x2 != 0 && tr.viewParms.isPortal as u64 == 0)
                            {
                                shader = crate::src::renderergl1::tr_shader::R_GetShaderByHandle(
                                    (*ent).e.customShader,
                                );
                                R_AddDrawSurf(
                                    &mut entitySurface,
                                    shader,
                                    R_SpriteFogNum(ent),
                                    0i32,
                                );
                            }
                        }
                    }
                }
            }
        }
        tr.currentEntityNum += 1
    }
}
/*
====================
R_GenerateDrawSurfs
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_GenerateDrawSurfs() {
    crate::src::renderergl1::tr_world::R_AddWorldSurfaces();
    crate::src::renderergl1::tr_scene::R_AddPolygonSurfaces();
    // set the projection matrix with the minimum zfar
    // now that we have the world bounded
    // this needs to be done before entities are
    // added, because they use the projection
    // matrix for lod calculation
    // dynamically compute far clip plane distance
    R_SetFarClip();
    // we know the size of the clipping volume. Now set the rest of the projection matrix.
    R_SetupProjectionZ(&mut tr.viewParms);
    R_AddEntitySurfaces();
}
/*
================
R_DebugPolygon
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_DebugPolygon(mut color: i32, mut numPoints: i32, mut points: *mut f32) {
    let mut i: i32 = 0;
    crate::src::renderergl1::tr_backend::GL_State((0x100i32 | 0x2 | 0x20) as usize);
    // draw solid shade
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(
        (color & 1i32) as crate::stdlib::GLfloat,
        (color >> 1 & 1i32) as crate::stdlib::GLfloat,
        (color >> 2 & 1i32) as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(0x9u32);
    i = 0;
    while i < numPoints {
        crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
            points.offset((i * 3) as isize),
        );
        i += 1
    }
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
    // draw wireframe outline
    crate::src::renderergl1::tr_backend::GL_State((0x1000i32 | 0x100 | 0x2 | 0x20) as usize);
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(0f64, 0f64);
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(1f32, 1f32, 1f32);
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(0x9u32);
    i = 0;
    while i < numPoints {
        crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
            points.offset((i * 3) as isize),
        );
        i += 1
    }
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(0f64, 1f64);
}
/*
====================
R_DebugGraphics

Visualization aid for movement clipping debugging
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_DebugGraphics() {
    if tr.refdef.rdflags & 0x1 != 0 {
        return;
    }
    if (*crate::src::renderergl1::tr_init::r_debugSurface).integer == 0 {
        return;
    }
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    crate::src::renderergl1::tr_backend::GL_Bind(tr.whiteImage);
    crate::src::renderergl1::tr_backend::GL_Cull(crate::tr_local_h::CT_FRONT_SIDED as i32);
    ri.CM_DrawDebugSurface.expect("non-null function pointer")(Some(
        R_DebugPolygon as unsafe extern "C" fn(_: i32, _: i32, _: *mut f32) -> (),
    ));
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
/*
================
R_RenderView

A view may be either the actual camera view,
or a mirror / remote location
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_RenderView(mut parms: *mut crate::tr_local_h::viewParms_t) {
    let mut firstDrawSurf: i32 = 0;
    let mut numDrawSurfs: i32 = 0;
    if (*parms).viewportWidth <= 0 || (*parms).viewportHeight <= 0 {
        return;
    }
    tr.viewCount += 1;
    tr.viewParms = *parms;
    tr.viewParms.frameSceneNum = tr.frameSceneNum;
    tr.viewParms.frameCount = tr.frameCount;
    firstDrawSurf = tr.refdef.numDrawSurfs;
    tr.viewCount += 1;
    // set viewParms.world
    R_RotateForViewer();
    R_SetupProjection(
        &mut tr.viewParms,
        (*crate::src::renderergl1::tr_init::r_zproj).value,
        crate::src::qcommon::q_shared::qtrue,
    );
    R_GenerateDrawSurfs();
    // if we overflowed MAX_DRAWSURFS, the drawsurfs
    // wrapped around in the buffer and we will be missing
    // the first surfaces, not the last ones
    numDrawSurfs = tr.refdef.numDrawSurfs;
    if numDrawSurfs > 0x10000 {
        numDrawSurfs = 0x10000
    }
    R_SortDrawSurfs(
        tr.refdef.drawSurfs.offset(firstDrawSurf as isize),
        numDrawSurfs - firstDrawSurf,
    );
    // draw main system development information (surface outlines, etc)
    R_DebugGraphics();
}
