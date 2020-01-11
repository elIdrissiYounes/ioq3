use ::libc;

pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::BoxOnPlaneSide;
pub use crate::src::qcommon::q_math::ClearBounds;
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
pub use crate::stdlib::GLuint;
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

pub use crate::src::renderergl1::tr_init::r_drawworld;
pub use crate::src::renderergl1::tr_init::r_facePlaneCull;
pub use crate::src::renderergl1::tr_init::r_lockpvs;
pub use crate::src::renderergl1::tr_init::r_nocull;
pub use crate::src::renderergl1::tr_init::r_nocurves;
pub use crate::src::renderergl1::tr_init::r_novis;
pub use crate::src::renderergl1::tr_init::r_showcluster;
pub use crate::src::renderergl1::tr_light::R_DlightBmodel;
pub use crate::src::renderergl1::tr_light::R_SetupEntityLighting;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_AddDrawSurf;
pub use crate::src::renderergl1::tr_main::R_CullLocalBox;
pub use crate::src::renderergl1::tr_main::R_CullLocalPointAndRadius;
pub use crate::src::renderergl1::tr_main::R_CullPointAndRadius;
pub use crate::src::renderergl1::tr_model::R_GetModelByHandle;
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
pub use crate::tr_local_h::dlight_t;
pub use crate::tr_local_h::drawSurf_s;
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
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfGridMesh_s;
pub use crate::tr_local_h::srfGridMesh_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfSurfaceFace_t;
pub use crate::tr_local_h::srfTriangles_t;
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
/*
=================
R_CullTriSurf

Returns true if the grid is completely culled away.
Also sets the clipped hint bit in tess
=================
*/

unsafe extern "C" fn R_CullTriSurf(
    mut cv: *mut crate::tr_local_h::srfTriangles_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut boxCull: i32 = 0;
    boxCull = crate::src::renderergl1::tr_main::R_CullLocalBox((*cv).bounds.as_mut_ptr());
    if boxCull == 2 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
R_CullGrid

Returns true if the grid is completely culled away.
Also sets the clipped hint bit in tess
=================
*/

unsafe extern "C" fn R_CullGrid(
    mut cv: *mut crate::tr_local_h::srfGridMesh_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut boxCull: i32 = 0;
    let mut sphereCull: i32 = 0;
    if (*crate::src::renderergl1::tr_init::r_nocurves).integer != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::renderergl1::tr_main::tr.currentEntityNum != ((1) << 10) - 1 {
        sphereCull = crate::src::renderergl1::tr_main::R_CullLocalPointAndRadius(
            (*cv).localOrigin.as_mut_ptr(),
            (*cv).meshRadius,
        )
    } else {
        sphereCull = crate::src::renderergl1::tr_main::R_CullPointAndRadius(
            (*cv).localOrigin.as_mut_ptr(),
            (*cv).meshRadius,
        )
    }
    // check for trivial reject
    if sphereCull == 2 {
        crate::src::renderergl1::tr_main::tr
            .pc
            .c_sphere_cull_patch_out += 1;
        return crate::src::qcommon::q_shared::qtrue;
    } else {
        // check bounding box if necessary
        if sphereCull == 1 {
            crate::src::renderergl1::tr_main::tr
                .pc
                .c_sphere_cull_patch_clip += 1;
            boxCull =
                crate::src::renderergl1::tr_main::R_CullLocalBox((*cv).meshBounds.as_mut_ptr());
            if boxCull == 2 {
                crate::src::renderergl1::tr_main::tr.pc.c_box_cull_patch_out += 1;
                return crate::src::qcommon::q_shared::qtrue;
            } else {
                if boxCull == 0 {
                    crate::src::renderergl1::tr_main::tr.pc.c_box_cull_patch_in += 1
                } else {
                    crate::src::renderergl1::tr_main::tr
                        .pc
                        .c_box_cull_patch_clip += 1
                }
            }
        } else {
            crate::src::renderergl1::tr_main::tr
                .pc
                .c_sphere_cull_patch_in += 1
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
================
R_CullSurface

Tries to back face cull surfaces before they are lighted or
added to the sorting list.

This will also allow mirrors on both sides of a model without recursion.
================
*/

unsafe extern "C" fn R_CullSurface(
    mut surface: *mut crate::tr_local_h::surfaceType_t,
    mut shader: *mut crate::tr_local_h::shader_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut sface: *mut crate::tr_local_h::srfSurfaceFace_t =
        0 as *mut crate::tr_local_h::srfSurfaceFace_t;
    let mut d: f32 = 0.;
    if (*crate::src::renderergl1::tr_init::r_nocull).integer != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *surface == crate::tr_local_h::SF_GRID {
        return R_CullGrid(surface as *mut crate::tr_local_h::srfGridMesh_t);
    }
    if *surface == crate::tr_local_h::SF_TRIANGLES {
        return R_CullTriSurf(surface as *mut crate::tr_local_h::srfTriangles_t);
    }
    if *surface != crate::tr_local_h::SF_FACE {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*shader).cullType == crate::tr_local_h::CT_TWO_SIDED {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // face culling
    if (*crate::src::renderergl1::tr_init::r_facePlaneCull).integer == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    sface = surface as *mut crate::tr_local_h::srfSurfaceFace_t;
    d = crate::src::renderergl1::tr_main::tr.or.viewOrigin[0] * (*sface).plane.normal[0]
        + crate::src::renderergl1::tr_main::tr.or.viewOrigin[1] * (*sface).plane.normal[1]
        + crate::src::renderergl1::tr_main::tr.or.viewOrigin[2] * (*sface).plane.normal[2];
    // don't cull exactly on the plane, because there are levels of rounding
    // through the BSP, ICD, and hardware that may cause pixel gaps if an
    // epsilon isn't allowed here
    if (*shader).cullType == crate::tr_local_h::CT_FRONT_SIDED {
        if d < (*sface).plane.dist - 8f32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    } else if d > (*sface).plane.dist + 8f32 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}

unsafe extern "C" fn R_DlightFace(
    mut face: *mut crate::tr_local_h::srfSurfaceFace_t,
    mut dlightBits: i32,
) -> i32 {
    let mut d: f32 = 0.;
    let mut i: i32 = 0;
    let mut dl: *mut crate::tr_local_h::dlight_t = 0 as *mut crate::tr_local_h::dlight_t;

    for i in 0..crate::src::renderergl1::tr_main::tr.refdef.num_dlights {
        if !(dlightBits & (1) << i == 0) {
            dl = &mut *crate::src::renderergl1::tr_main::tr
                .refdef
                .dlights
                .offset(i as isize) as *mut crate::tr_local_h::dlight_s;
            d = (*dl).origin[0] * (*face).plane.normal[0]
                + (*dl).origin[1] * (*face).plane.normal[1]
                + (*dl).origin[2] * (*face).plane.normal[2]
                - (*face).plane.dist;
            if d < -(*dl).radius || d > (*dl).radius {
                // dlight doesn't reach the plane
                dlightBits &= !((1) << i)
            }
        }
    }
    if dlightBits == 0 {
        crate::src::renderergl1::tr_main::tr
            .pc
            .c_dlightSurfacesCulled += 1
    }
    (*face).dlightBits = dlightBits;
    return dlightBits;
}

unsafe extern "C" fn R_DlightGrid(
    mut grid: *mut crate::tr_local_h::srfGridMesh_t,
    mut dlightBits: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut dl: *mut crate::tr_local_h::dlight_t = 0 as *mut crate::tr_local_h::dlight_t;

    for i in 0..crate::src::renderergl1::tr_main::tr.refdef.num_dlights {
        if !(dlightBits & (1) << i == 0) {
            dl = &mut *crate::src::renderergl1::tr_main::tr
                .refdef
                .dlights
                .offset(i as isize) as *mut crate::tr_local_h::dlight_s;
            if (*dl).origin[0] - (*dl).radius > (*grid).meshBounds[1][0]
                || (*dl).origin[0] + (*dl).radius < (*grid).meshBounds[0][0]
                || (*dl).origin[1] - (*dl).radius > (*grid).meshBounds[1][1]
                || (*dl).origin[1] + (*dl).radius < (*grid).meshBounds[0][1]
                || (*dl).origin[2] - (*dl).radius > (*grid).meshBounds[1][2]
                || (*dl).origin[2] + (*dl).radius < (*grid).meshBounds[0][2]
            {
                // dlight doesn't reach the bounds
                dlightBits &= !((1) << i)
            }
        }
    }
    if dlightBits == 0 {
        crate::src::renderergl1::tr_main::tr
            .pc
            .c_dlightSurfacesCulled += 1
    }
    (*grid).dlightBits = dlightBits;
    return dlightBits;
}

unsafe extern "C" fn R_DlightTrisurf(
    mut surf: *mut crate::tr_local_h::srfTriangles_t,
    mut dlightBits: i32,
) -> i32 {
    // FIXME: more dlight culling to trisurfs...
    (*surf).dlightBits = dlightBits;
    return dlightBits;
}
/*
====================
R_DlightSurface

The given surface is going to be drawn, and it touches a leaf
that is touched by one or more dlights, so try to throw out
more dlights if possible.
====================
*/

unsafe extern "C" fn R_DlightSurface(
    mut surf: *mut crate::tr_local_h::msurface_t,
    mut dlightBits: i32,
) -> i32 {
    if *(*surf).data == crate::tr_local_h::SF_FACE {
        dlightBits = R_DlightFace(
            (*surf).data as *mut crate::tr_local_h::srfSurfaceFace_t,
            dlightBits,
        )
    } else if *(*surf).data == crate::tr_local_h::SF_GRID {
        dlightBits = R_DlightGrid(
            (*surf).data as *mut crate::tr_local_h::srfGridMesh_t,
            dlightBits,
        )
    } else if *(*surf).data == crate::tr_local_h::SF_TRIANGLES {
        dlightBits = R_DlightTrisurf(
            (*surf).data as *mut crate::tr_local_h::srfTriangles_t,
            dlightBits,
        )
    } else {
        dlightBits = 0
    }
    if dlightBits != 0 {
        crate::src::renderergl1::tr_main::tr.pc.c_dlightSurfaces += 1
    }
    return dlightBits;
}
/*
======================
R_AddWorldSurface
======================
*/

unsafe extern "C" fn R_AddWorldSurface(
    mut surf: *mut crate::tr_local_h::msurface_t,
    mut dlightBits: i32,
) {
    if (*surf).viewCount == crate::src::renderergl1::tr_main::tr.viewCount {
        return;
        // already in this view
    }
    (*surf).viewCount = crate::src::renderergl1::tr_main::tr.viewCount;
    // FIXME: bmodel fog?
    // try to cull before dlighting or adding
    if R_CullSurface((*surf).data, (*surf).shader) as u64 != 0 {
        return;
    }
    // check for dlighting
    if dlightBits != 0 {
        dlightBits = R_DlightSurface(surf, dlightBits);
        dlightBits = (dlightBits != 0) as i32
    }
    crate::src::renderergl1::tr_main::R_AddDrawSurf(
        (*surf).data,
        (*surf).shader,
        (*surf).fogIndex,
        dlightBits,
    );
}
/*
=============================================================

    BRUSH MODELS

=============================================================
*/
/*
=================
R_AddBrushModelSurfaces
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddBrushModelSurfaces(mut ent: *mut crate::tr_local_h::trRefEntity_t) {
    let mut bmodel: *mut crate::tr_local_h::bmodel_t = 0 as *mut crate::tr_local_h::bmodel_t;
    let mut clip: i32 = 0;
    let mut pModel: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    let mut i: i32 = 0;
    pModel = crate::src::renderergl1::tr_model::R_GetModelByHandle((*ent).e.hModel);
    bmodel = (*pModel).bmodel;
    clip = crate::src::renderergl1::tr_main::R_CullLocalBox((*bmodel).bounds.as_mut_ptr());
    if clip == 2 {
        return;
    }
    crate::src::renderergl1::tr_light::R_SetupEntityLighting(
        &mut crate::src::renderergl1::tr_main::tr.refdef,
        ent,
    );
    crate::src::renderergl1::tr_light::R_DlightBmodel(bmodel);
    i = 0;
    while i < (*bmodel).numSurfaces {
        R_AddWorldSurface(
            (*bmodel).firstSurface.offset(i as isize),
            (*crate::src::renderergl1::tr_main::tr.currentEntity).needDlights as i32,
        );
        i += 1
    }
}
/*
=============================================================

    WORLD MODEL

=============================================================
*/
/*
================
R_RecursiveWorldNode
================
*/

unsafe extern "C" fn R_RecursiveWorldNode(
    mut node: *mut crate::tr_local_h::mnode_t,
    mut planeBits: u32,
    mut dlightBits: u32,
) {
    loop {
        let mut newDlights: [u32; 2] = [0; 2];
        // if the node wasn't marked as potentially visible, exit
        if (*node).visframe != crate::src::renderergl1::tr_main::tr.visCount {
            return;
        }
        // if the bounding volume is outside the frustum, nothing
        // inside can be visible OPTIMIZE: don't do this all the way to leafs?
        if (*crate::src::renderergl1::tr_init::r_nocull).integer == 0 {
            let mut r: i32 = 0;
            if planeBits & 1u32 != 0 {
                r = crate::src::qcommon::q_math::BoxOnPlaneSide(
                    (*node).mins.as_mut_ptr(),
                    (*node).maxs.as_mut_ptr(),
                    &mut *crate::src::renderergl1::tr_main::tr
                        .viewParms
                        .frustum
                        .as_mut_ptr()
                        .offset(0),
                );
                if r == 2 {
                    return;
                    // culled
                }
                if r == 1 {
                    planeBits &= !(1i32) as u32
                    // all descendants will also be in front
                }
            }
            if planeBits & 2u32 != 0 {
                r = crate::src::qcommon::q_math::BoxOnPlaneSide(
                    (*node).mins.as_mut_ptr(),
                    (*node).maxs.as_mut_ptr(),
                    &mut *crate::src::renderergl1::tr_main::tr
                        .viewParms
                        .frustum
                        .as_mut_ptr()
                        .offset(1),
                );
                if r == 2 {
                    return;
                    // culled
                }
                if r == 1 {
                    planeBits &= !(2i32) as u32
                    // all descendants will also be in front
                }
            }
            if planeBits & 4u32 != 0 {
                r = crate::src::qcommon::q_math::BoxOnPlaneSide(
                    (*node).mins.as_mut_ptr(),
                    (*node).maxs.as_mut_ptr(),
                    &mut *crate::src::renderergl1::tr_main::tr
                        .viewParms
                        .frustum
                        .as_mut_ptr()
                        .offset(2),
                );
                if r == 2 {
                    return;
                    // culled
                }
                if r == 1 {
                    planeBits &= !(4i32) as u32
                    // all descendants will also be in front
                }
            }
            if planeBits & 8u32 != 0 {
                r = crate::src::qcommon::q_math::BoxOnPlaneSide(
                    (*node).mins.as_mut_ptr(),
                    (*node).maxs.as_mut_ptr(),
                    &mut *crate::src::renderergl1::tr_main::tr
                        .viewParms
                        .frustum
                        .as_mut_ptr()
                        .offset(3),
                );
                if r == 2 {
                    return;
                    // culled
                }
                if r == 1 {
                    planeBits &= !(8i32) as u32
                    // all descendants will also be in front
                }
            }
        }
        if (*node).contents != -(1) {
            break;
        }
        // node is just a decision point, so go down both sides
        // since we don't care about sort orders, just go positive to negative
        // determine which dlights are needed
        newDlights[0] = 0;
        newDlights[1] = 0;
        if dlightBits != 0 {
            let mut i: i32 = 0;
            i = 0;
            while i < crate::src::renderergl1::tr_main::tr.refdef.num_dlights {
                let mut dl: *mut crate::tr_local_h::dlight_t =
                    0 as *mut crate::tr_local_h::dlight_t;
                let mut dist: f32 = 0.;
                if dlightBits & ((1i32) << i) as u32 != 0 {
                    dl = &mut *crate::src::renderergl1::tr_main::tr
                        .refdef
                        .dlights
                        .offset(i as isize)
                        as *mut crate::tr_local_h::dlight_s;
                    dist = (*dl).origin[0] * (*(*node).plane).normal[0]
                        + (*dl).origin[1] * (*(*node).plane).normal[1]
                        + (*dl).origin[2] * (*(*node).plane).normal[2]
                        - (*(*node).plane).dist;
                    if dist > -(*dl).radius {
                        newDlights[0] |= ((1i32) << i) as u32
                    }
                    if dist < (*dl).radius {
                        newDlights[1] |= ((1i32) << i) as u32
                    }
                }
                i += 1
            }
        }
        // recurse down the children, front side first
        R_RecursiveWorldNode((*node).children[0], planeBits, newDlights[0]);
        // tail recurse
        node = (*node).children[1];
        dlightBits = newDlights[1]
    }
    // leaf node, so add mark surfaces
    let mut c: i32 = 0;
    let mut surf: *mut crate::tr_local_h::msurface_t = 0 as *mut crate::tr_local_h::msurface_t;
    let mut mark: *mut *mut crate::tr_local_h::msurface_t =
        0 as *mut *mut crate::tr_local_h::msurface_t;
    crate::src::renderergl1::tr_main::tr.pc.c_leafs += 1;
    // add to z buffer bounds
    if (*node).mins[0] < crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0][0] {
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0][0] = (*node).mins[0]
    }
    if (*node).mins[1] < crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0][1] {
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0][1] = (*node).mins[1]
    }
    if (*node).mins[2] < crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0][2] {
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0][2] = (*node).mins[2]
    }
    if (*node).maxs[0] > crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1][0] {
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1][0] = (*node).maxs[0]
    }
    if (*node).maxs[1] > crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1][1] {
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1][1] = (*node).maxs[1]
    }
    if (*node).maxs[2] > crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1][2] {
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1][2] = (*node).maxs[2]
    }
    // add the individual surfaces
    mark = (*node).firstmarksurface;
    c = (*node).nummarksurfaces;
    loop {
        let fresh0 = c;
        c = c - 1;
        if !(fresh0 != 0) {
            break;
        }
        // the surface may have already been added if it
        // spans multiple leafs
        surf = *mark;
        R_AddWorldSurface(surf, dlightBits as i32);
        mark = mark.offset(1)
    }
}
/*
===============
R_PointInLeaf
===============
*/

unsafe extern "C" fn R_PointInLeaf(
    mut p: *const crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::tr_local_h::mnode_t {
    let mut node: *mut crate::tr_local_h::mnode_t = 0 as *mut crate::tr_local_h::mnode_t;
    let mut d: f32 = 0.;
    let mut plane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    if crate::src::renderergl1::tr_main::tr.world.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"R_PointInLeaf: bad model\x00" as *const u8 as *const i8,
        );
    }
    node = (*crate::src::renderergl1::tr_main::tr.world).nodes;
    while !((*node).contents != -(1)) {
        plane = (*node).plane;
        d = *p.offset(0) * (*plane).normal[0]
            + *p.offset(1) * (*plane).normal[1]
            + *p.offset(2) * (*plane).normal[2]
            - (*plane).dist;
        if d > 0f32 {
            node = (*node).children[0]
        } else {
            node = (*node).children[1]
        }
    }
    return node;
}
/*
==============
R_ClusterPVS
==============
*/

unsafe extern "C" fn R_ClusterPVS(mut cluster: i32) -> *const crate::src::qcommon::q_shared::byte {
    if (*crate::src::renderergl1::tr_main::tr.world).vis.is_null()
        || cluster < 0
        || cluster >= (*crate::src::renderergl1::tr_main::tr.world).numClusters
    {
        return (*crate::src::renderergl1::tr_main::tr.world).novis;
    }
    return (*crate::src::renderergl1::tr_main::tr.world)
        .vis
        .offset((cluster * (*crate::src::renderergl1::tr_main::tr.world).clusterBytes) as isize);
}
/*
=================
R_inPVS
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_inPVS(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut leaf: *mut crate::tr_local_h::mnode_t = 0 as *mut crate::tr_local_h::mnode_t; // why not R_ClusterPVS ??
    let mut vis: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    leaf = R_PointInLeaf(p1);
    vis = crate::src::renderergl1::tr_main::ri
        .CM_ClusterPVS
        .expect("non-null function pointer")((*leaf).cluster);
    leaf = R_PointInLeaf(p2);
    if *vis.offset(((*leaf).cluster >> 3) as isize) as i32 & (1) << ((*leaf).cluster & 7) == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
R_MarkLeaves

Mark the leaves and nodes that are in the PVS for the current
cluster
===============
*/

unsafe extern "C" fn R_MarkLeaves() {
    let mut vis: *const crate::src::qcommon::q_shared::byte =
        0 as *const crate::src::qcommon::q_shared::byte;
    let mut leaf: *mut crate::tr_local_h::mnode_t = 0 as *mut crate::tr_local_h::mnode_t;
    let mut parent: *mut crate::tr_local_h::mnode_t = 0 as *mut crate::tr_local_h::mnode_t;
    let mut i: i32 = 0;
    let mut cluster: i32 = 0;
    // lockpvs lets designers walk around to determine the
    // extent of the current pvs
    if (*crate::src::renderergl1::tr_init::r_lockpvs).integer != 0 {
        return;
    }
    // current viewcluster
    leaf = R_PointInLeaf(
        crate::src::renderergl1::tr_main::tr
            .viewParms
            .pvsOrigin
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    cluster = (*leaf).cluster;
    // if the cluster is the same and the area visibility matrix
    // hasn't changed, we don't need to mark everything again
    // if r_showcluster was just turned on, remark everything
    if crate::src::renderergl1::tr_main::tr.viewCluster == cluster
        && crate::src::renderergl1::tr_main::tr.refdef.areamaskModified as u64 == 0
        && (*crate::src::renderergl1::tr_init::r_showcluster).modified as u64 == 0
    {
        return;
    }
    if (*crate::src::renderergl1::tr_init::r_showcluster).modified != 0
        || (*crate::src::renderergl1::tr_init::r_showcluster).integer != 0
    {
        (*crate::src::renderergl1::tr_init::r_showcluster).modified =
            crate::src::qcommon::q_shared::qfalse;
        if (*crate::src::renderergl1::tr_init::r_showcluster).integer != 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"cluster:%i  area:%i\n\x00" as *const u8 as *const i8,
                cluster,
                (*leaf).area,
            );
        }
    }
    crate::src::renderergl1::tr_main::tr.visCount += 1;
    crate::src::renderergl1::tr_main::tr.viewCluster = cluster;
    if (*crate::src::renderergl1::tr_init::r_novis).integer != 0
        || crate::src::renderergl1::tr_main::tr.viewCluster == -(1)
    {
        i = 0;
        while i < (*crate::src::renderergl1::tr_main::tr.world).numnodes {
            if (*(*crate::src::renderergl1::tr_main::tr.world)
                .nodes
                .offset(i as isize))
            .contents
                != 1
            {
                (*(*crate::src::renderergl1::tr_main::tr.world)
                    .nodes
                    .offset(i as isize))
                .visframe = crate::src::renderergl1::tr_main::tr.visCount
            }
            i += 1
        }
        return;
    }
    vis = R_ClusterPVS(crate::src::renderergl1::tr_main::tr.viewCluster);
    i = 0;
    leaf = (*crate::src::renderergl1::tr_main::tr.world).nodes;
    while i < (*crate::src::renderergl1::tr_main::tr.world).numnodes {
        cluster = (*leaf).cluster;
        if !(cluster < 0 || cluster >= (*crate::src::renderergl1::tr_main::tr.world).numClusters) {
            // check general pvs
            if !(*vis.offset((cluster >> 3) as isize) as i32 & (1) << (cluster & 7) == 0) {
                // check for door connection
                if !(crate::src::renderergl1::tr_main::tr.refdef.areamask
                    [((*leaf).area >> 3) as usize] as i32
                    & (1) << ((*leaf).area & 7)
                    != 0)
                {
                    parent = leaf;
                    while !((*parent).visframe == crate::src::renderergl1::tr_main::tr.visCount) {
                        (*parent).visframe = crate::src::renderergl1::tr_main::tr.visCount;
                        parent = (*parent).parent;
                        if parent.is_null() {
                            break;
                        }
                    }
                }
            }
        }
        i += 1;
        leaf = leaf.offset(1)
        // not visible
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
=============
R_AddWorldSurfaces
=============
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddWorldSurfaces() {
    if (*crate::src::renderergl1::tr_init::r_drawworld).integer == 0 {
        return;
    }
    if crate::src::renderergl1::tr_main::tr.refdef.rdflags & 0x1 != 0 {
        return;
    }
    crate::src::renderergl1::tr_main::tr.currentEntityNum = ((1) << 10) - 1;
    crate::src::renderergl1::tr_main::tr.shiftedEntityNum =
        crate::src::renderergl1::tr_main::tr.currentEntityNum << 7;
    // determine which leaves are in the PVS / areamask
    R_MarkLeaves();
    // clear out the visible min/max
    crate::src::qcommon::q_math::ClearBounds(
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[0].as_mut_ptr(),
        crate::src::renderergl1::tr_main::tr.viewParms.visBounds[1].as_mut_ptr(),
    );
    // perform frustum culling and add all the potentially visible surfaces
    if crate::src::renderergl1::tr_main::tr.refdef.num_dlights > 32 {
        crate::src::renderergl1::tr_main::tr.refdef.num_dlights = 32
    }
    R_RecursiveWorldNode(
        (*crate::src::renderergl1::tr_main::tr.world).nodes,
        15,
        ((1u64) << crate::src::renderergl1::tr_main::tr.refdef.num_dlights).wrapping_sub(1u64)
            as u32,
    );
}
