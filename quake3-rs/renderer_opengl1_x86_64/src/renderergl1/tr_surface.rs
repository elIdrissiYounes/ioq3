use ::libc;

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

    //
    // these aren't needed by any of the VMs.  put in another header?
    //
    // bit vector of area visibility
    // print levels from renderer (FIXME: set up for game / cgame?)

    // only print when "developer 1"

    // parameters to the main Error routine

    // pop up the need-cd dialog
    // client disconnected from the server

    // don't kill server

    // print to console and disconnect from game

    // exit the entire game with a popup window

    // font rendering values used by ui and cgame
    // default
    // default

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    /*
    // if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
    // or write a mail to the ioq3 mailing list.
    #else
      #define Q_ftol(v) ((long) (v))
      #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
      #define Q_SnapVector(vec) \
        do\
        {\
            vec3_t *temp = (vec);\
            \
            Q_round((*temp)[0]);\
            Q_round((*temp)[1]);\
            Q_round((*temp)[2]);\
        } while(0)
    #endif
    */
    // reciprocal square root
    // this isn't a real cheap function to call!
    // just in case you don't want to use the macros
    // fast vector normalize routine that does not check to make sure
    // that length != 0, nor does it return length, uses rsqrt approximation
    // returns vector length
    // perpendicular vector could be replaced by this
    //int	PlaneTypeForNormal (vec3_t normal);
    //=============================================
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    // data is an in/out parm, returns a parsed out token
    // mode parm for FS_FOpenFile
    //=============================================
    // portable case insensitive compare
    // buffer size safe library replacements
    // strlen that discounts Quake color sequences
    // removes color sequences from string
    // Count the number of char tocount encountered in string
    //=============================================
    // 64-bit integers for global rankings interface
    // implemented as a struct for qvm compatibility
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
    //=============================================
    //
    // key / value info strings
    //
    // this is only here so the functions in q_shared.c and bg_*.c can link
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
    // cvar_restart will reset to this value
    // for CVAR_LATCH vars
    // set each time the cvar is changed
    // incremented each time the cvar is changed
    // atof( string )
    // atoi( string )
    // the modules that run in the virtual machine can't access the cvar_t directly,
    // so they must ask for structured updates
    /*
    ==============================================================

    VoIP

    ==============================================================
    */
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
    // change this.
    /*
    ==============================================================

    COLLISION DETECTION

    ==============================================================
    */
    // plane types are used to speed some tests
    // 0-2 are axial planes
    /*
    =================
    PlaneTypeForNormal
    =================
    */
    // plane_t structure
    // !!! if this is changed, it must be changed in asm code too !!!

    // server browser sources
    // TTimo: AS_MPLAYER is no longer used
    // cinematic states

    // all other conditions, i.e. stop/EOF/abort

    // play

    #[inline]

    pub unsafe extern "C" fn VectorNormalizeFast(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        let mut ilength: f32 = 0.;
        ilength = crate::src::qcommon::q_math::Q_rsqrt(
            *v.offset(0) * *v.offset(0) + *v.offset(1) * *v.offset(1) + *v.offset(2) * *v.offset(2),
        );
        let ref mut fresh0 = *v.offset(0);
        *fresh0 *= ilength;
        let ref mut fresh1 = *v.offset(1);
        *fresh1 *= ilength;
        let ref mut fresh2 = *v.offset(2);
        *fresh2 *= ilength;
    }
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

    // __Q_SHARED_H
}

pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qfiles_h::md3Surface_t;
pub use crate::qfiles_h::mdrSurface_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::MakeNormalVectors;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::Q_rsqrt;
pub use crate::src::qcommon::q_math::RotatePointAroundVector;
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
pub use crate::src::renderergl1::tr_surface::q_shared_h::CrossProduct;
pub use crate::src::renderergl1::tr_surface::q_shared_h::VectorNormalizeFast;
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
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::LineWidthproc;
pub use crate::qgl_h::Vertex3fproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::src::renderergl1::tr_animation::RB_MDRSurfaceAnim;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_State;
pub use crate::src::renderergl1::tr_flares::RB_AddFlare;
pub use crate::src::renderergl1::tr_init::r_flares;
pub use crate::src::renderergl1::tr_init::r_lodCurveError;
pub use crate::src::renderergl1::tr_init::r_railCoreWidth;
pub use crate::src::renderergl1::tr_init::r_railSegmentLength;
pub use crate::src::renderergl1::tr_init::r_railWidth;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_model_iqm::RB_IQMSurfaceAnim;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shade::RB_BeginSurface;
pub use crate::src::renderergl1::tr_shade::RB_EndSurface;
pub use crate::src::sdl::sdl_glimp::qglBegin;
pub use crate::src::sdl::sdl_glimp::qglColor3f;
pub use crate::src::sdl::sdl_glimp::qglEnd;
pub use crate::src::sdl::sdl_glimp::qglLineWidth;
pub use crate::src::sdl::sdl_glimp::qglVertex3f;
pub use crate::src::sdl::sdl_glimp::qglVertex3fv;
use crate::stdlib::cos;
use crate::stdlib::sin;
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
pub use crate::tr_local_h::backEndCounters_t;
pub use crate::tr_local_h::backEndState_t;
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::color4ub_t;
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
pub use crate::tr_local_h::srfFlare_s;
pub use crate::tr_local_h::srfFlare_t;
pub use crate::tr_local_h::srfGridMesh_s;
pub use crate::tr_local_h::srfGridMesh_t;
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
// tr_surf.c
/*

  THIS ENTIRE FILE IS BACK END

backEnd.currentEntity will be valid.

Tess_Begin has already been called for the surface's shader.

The modelview matrix will be set.

It is safe to actually issue drawing commands here if you don't want to
use the shader system.
*/
//============================================================================
/*
==============
RB_CheckOverflow
==============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CheckOverflow(mut verts: i32, mut indexes: i32) {
    if crate::src::renderergl1::tr_shade::tess.numVertexes + verts < 1000
        && crate::src::renderergl1::tr_shade::tess.numIndexes + indexes < 6 * 1000
    {
        return;
    }
    crate::src::renderergl1::tr_shade::RB_EndSurface();
    if verts >= 1000 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"RB_CheckOverflow: verts > MAX (%d > %d)\x00" as *const u8 as *const i8,
            verts,
            1000i32,
        );
    }
    if indexes >= 6 * 1000 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"RB_CheckOverflow: indices > MAX (%d > %d)\x00" as *const u8 as *const i8,
            indexes,
            6i32 * 1000i32,
        );
    }
    crate::src::renderergl1::tr_shade::RB_BeginSurface(
        crate::src::renderergl1::tr_shade::tess.shader,
        crate::src::renderergl1::tr_shade::tess.fogNum,
    );
}
/*
==============
RB_AddQuadStampExt
==============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_AddQuadStampExt(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut left: *mut crate::src::qcommon::q_shared::vec_t,
    mut up: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: *mut crate::src::qcommon::q_shared::byte,
    mut s1: f32,
    mut t1: f32,
    mut s2: f32,
    mut t2: f32,
) {
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ndx: i32 = 0;
    if crate::src::renderergl1::tr_shade::tess.numVertexes + 4 >= 1000
        || crate::src::renderergl1::tr_shade::tess.numIndexes + 6 >= 6 * 1000
    {
        RB_CheckOverflow(4i32, 6i32);
    }
    ndx = crate::src::renderergl1::tr_shade::tess.numVertexes;
    // triangle indexes for a simple quad
    crate::src::renderergl1::tr_shade::tess.indexes
        [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
        ndx as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes
        [(crate::src::renderergl1::tr_shade::tess.numIndexes + 1) as usize] =
        (ndx + 1) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes
        [(crate::src::renderergl1::tr_shade::tess.numIndexes + 2) as usize] =
        (ndx + 3) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes
        [(crate::src::renderergl1::tr_shade::tess.numIndexes + 3) as usize] =
        (ndx + 3) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes
        [(crate::src::renderergl1::tr_shade::tess.numIndexes + 4) as usize] =
        (ndx + 1) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes
        [(crate::src::renderergl1::tr_shade::tess.numIndexes + 5) as usize] =
        (ndx + 2) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.xyz[ndx as usize][0] =
        *origin.offset(0) + *left.offset(0) + *up.offset(0);
    crate::src::renderergl1::tr_shade::tess.xyz[ndx as usize][1] =
        *origin.offset(1) + *left.offset(1) + *up.offset(1);
    crate::src::renderergl1::tr_shade::tess.xyz[ndx as usize][2] =
        *origin.offset(2) + *left.offset(2) + *up.offset(2);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 1) as usize][0] =
        *origin.offset(0) - *left.offset(0) + *up.offset(0);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 1) as usize][1] =
        *origin.offset(1) - *left.offset(1) + *up.offset(1);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 1) as usize][2] =
        *origin.offset(2) - *left.offset(2) + *up.offset(2);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 2) as usize][0] =
        *origin.offset(0) - *left.offset(0) - *up.offset(0);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 2) as usize][1] =
        *origin.offset(1) - *left.offset(1) - *up.offset(1);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 2) as usize][2] =
        *origin.offset(2) - *left.offset(2) - *up.offset(2);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 3) as usize][0] =
        *origin.offset(0) + *left.offset(0) - *up.offset(0);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 3) as usize][1] =
        *origin.offset(1) + *left.offset(1) - *up.offset(1);
    crate::src::renderergl1::tr_shade::tess.xyz[(ndx + 3) as usize][2] =
        *origin.offset(2) + *left.offset(2) - *up.offset(2);
    // constant normal all the way around
    normal[0] = crate::src::qcommon::q_math::vec3_origin[0]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0][0];
    normal[1] = crate::src::qcommon::q_math::vec3_origin[1]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0][1];
    normal[2] = crate::src::qcommon::q_math::vec3_origin[2]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0][2];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 3) as usize][0] = normal[0];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 2) as usize][0] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 3) as usize][0];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 1) as usize][0] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 2) as usize][0];
    crate::src::renderergl1::tr_shade::tess.normal[ndx as usize][0] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 1) as usize][0];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 3) as usize][1] = normal[1];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 2) as usize][1] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 3) as usize][1];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 1) as usize][1] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 2) as usize][1];
    crate::src::renderergl1::tr_shade::tess.normal[ndx as usize][1] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 1) as usize][1];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 3) as usize][2] = normal[2];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 2) as usize][2] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 3) as usize][2];
    crate::src::renderergl1::tr_shade::tess.normal[(ndx + 1) as usize][2] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 2) as usize][2];
    crate::src::renderergl1::tr_shade::tess.normal[ndx as usize][2] =
        crate::src::renderergl1::tr_shade::tess.normal[(ndx + 1) as usize][2];
    // standard square texture coordinates
    crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][1][0] = s1;
    crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][0][0] =
        crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][1][0];
    crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][1][1] = t1;
    crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][0][1] =
        crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][1][1];
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 1) as usize][1][0] = s2;
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 1) as usize][0][0] =
        crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 1) as usize][1][0];
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 1) as usize][1][1] = t1;
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 1) as usize][0][1] =
        crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 1) as usize][1][1];
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 2) as usize][1][0] = s2;
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 2) as usize][0][0] =
        crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 2) as usize][1][0];
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 2) as usize][1][1] = t2;
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 2) as usize][0][1] =
        crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 2) as usize][1][1];
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 3) as usize][1][0] = s1;
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 3) as usize][0][0] =
        crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 3) as usize][1][0];
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 3) as usize][1][1] = t2;
    crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 3) as usize][0][1] =
        crate::src::renderergl1::tr_shade::tess.texCoords[(ndx + 3) as usize][1][1];
    // constant color all the way around
    // should this be identity and let the shader specify from entity?
    let ref mut fresh3 = *(&mut *crate::src::renderergl1::tr_shade::tess
        .vertexColors
        .as_mut_ptr()
        .offset((ndx + 3) as isize) as *mut crate::tr_local_h::color4ub_t
        as *mut u32);
    *fresh3 = *(color as *mut u32);
    let ref mut fresh4 = *(&mut *crate::src::renderergl1::tr_shade::tess
        .vertexColors
        .as_mut_ptr()
        .offset((ndx + 2) as isize) as *mut crate::tr_local_h::color4ub_t
        as *mut u32);
    *fresh4 = *fresh3;
    let ref mut fresh5 = *(&mut *crate::src::renderergl1::tr_shade::tess
        .vertexColors
        .as_mut_ptr()
        .offset((ndx + 1) as isize) as *mut crate::tr_local_h::color4ub_t
        as *mut u32);
    *fresh5 = *fresh4;
    *(&mut *crate::src::renderergl1::tr_shade::tess
        .vertexColors
        .as_mut_ptr()
        .offset(ndx as isize) as *mut crate::tr_local_h::color4ub_t as *mut u32) = *fresh5;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 4;
    crate::src::renderergl1::tr_shade::tess.numIndexes += 6;
}
/*
==============
RB_AddQuadStamp
==============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_AddQuadStamp(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut left: *mut crate::src::qcommon::q_shared::vec_t,
    mut up: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: *mut crate::src::qcommon::q_shared::byte,
) {
    RB_AddQuadStampExt(origin, left, up, color, 0f32, 0f32, 1f32, 1f32);
}
/*
==============
RB_SurfaceSprite
==============
*/

unsafe extern "C" fn RB_SurfaceSprite() {
    let mut left: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut radius: f32 = 0.;
    // calculate the xyz locations for the four corners
    radius = (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
        .e
        .radius;
    if (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
        .e
        .rotation
        == 0f32
    {
        left[0] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1][0]
            * radius;
        left[1] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1][1]
            * radius;
        left[2] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1][2]
            * radius;
        up[0] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2][0]
            * radius;
        up[1] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2][1]
            * radius;
        up[2] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2][2]
            * radius
    } else {
        let mut s: f32 = 0.;
        let mut c: f32 = 0.;
        let mut ang: f32 = 0.;
        ang = (3.14159265358979323846
            * (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                .e
                .rotation as f64
            / 180f64) as f32;
        s = crate::stdlib::sin(ang as f64) as f32;
        c = crate::stdlib::cos(ang as f64) as f32;
        left[0] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1][0]
            * (c * radius);
        left[1] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1][1]
            * (c * radius);
        left[2] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1][2]
            * (c * radius);
        left[0] = left[0]
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[2][0]
                * (-s * radius);
        left[1] = left[1]
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[2][1]
                * (-s * radius);
        left[2] = left[2]
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[2][2]
                * (-s * radius);
        up[0] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2][0]
            * (c * radius);
        up[1] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2][1]
            * (c * radius);
        up[2] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2][2]
            * (c * radius);
        up[0] = up[0]
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[1][0]
                * (s * radius);
        up[1] = up[1]
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[1][1]
                * (s * radius);
        up[2] = up[2]
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[1][2]
                * (s * radius)
    }
    if crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .isMirror as u64
        != 0
    {
        left[0] = crate::src::qcommon::q_math::vec3_origin[0] - left[0];
        left[1] = crate::src::qcommon::q_math::vec3_origin[1] - left[1];
        left[2] = crate::src::qcommon::q_math::vec3_origin[2] - left[2]
    }
    RB_AddQuadStamp(
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .origin
            .as_mut_ptr(),
        left.as_mut_ptr(),
        up.as_mut_ptr(),
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA
            .as_mut_ptr(),
    );
}
/*
=============
RB_SurfacePolychain
=============
*/

unsafe extern "C" fn RB_SurfacePolychain(mut p: *mut crate::tr_local_h::srfPoly_t) {
    let mut i: i32 = 0;
    let mut numv: i32 = 0;
    if crate::src::renderergl1::tr_shade::tess.numVertexes + (*p).numVerts >= 1000
        || crate::src::renderergl1::tr_shade::tess.numIndexes + 3 * ((*p).numVerts - 2) >= 6 * 1000
    {
        RB_CheckOverflow((*p).numVerts, 3i32 * ((*p).numVerts - 2i32));
    }
    // fan triangles into the tess array
    numv = crate::src::renderergl1::tr_shade::tess.numVertexes;
    i = 0;
    while i < (*p).numVerts {
        crate::src::renderergl1::tr_shade::tess.xyz[numv as usize][0] =
            (*(*p).verts.offset(i as isize)).xyz[0];
        crate::src::renderergl1::tr_shade::tess.xyz[numv as usize][1] =
            (*(*p).verts.offset(i as isize)).xyz[1];
        crate::src::renderergl1::tr_shade::tess.xyz[numv as usize][2] =
            (*(*p).verts.offset(i as isize)).xyz[2];
        crate::src::renderergl1::tr_shade::tess.texCoords[numv as usize][0][0] =
            (*(*p).verts.offset(i as isize)).st[0];
        crate::src::renderergl1::tr_shade::tess.texCoords[numv as usize][0][1] =
            (*(*p).verts.offset(i as isize)).st[1];
        *(&mut *crate::src::renderergl1::tr_shade::tess
            .vertexColors
            .as_mut_ptr()
            .offset(numv as isize) as *mut crate::tr_local_h::color4ub_t as *mut i32) =
            *((*(*p).verts.offset(i as isize)).modulate.as_mut_ptr() as *mut i32);
        numv += 1;
        i += 1
    }
    // generate fan indexes into the tess array
    i = 0;
    while i < (*p).numVerts - 2 {
        crate::src::renderergl1::tr_shade::tess.indexes
            [(crate::src::renderergl1::tr_shade::tess.numIndexes + 0) as usize] =
            crate::src::renderergl1::tr_shade::tess.numVertexes as crate::tr_local_h::glIndex_t;
        crate::src::renderergl1::tr_shade::tess.indexes
            [(crate::src::renderergl1::tr_shade::tess.numIndexes + 1) as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes + i + 1)
                as crate::tr_local_h::glIndex_t;
        crate::src::renderergl1::tr_shade::tess.indexes
            [(crate::src::renderergl1::tr_shade::tess.numIndexes + 2) as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes + i + 2)
                as crate::tr_local_h::glIndex_t;
        crate::src::renderergl1::tr_shade::tess.numIndexes += 3;
        i += 1
    }
    crate::src::renderergl1::tr_shade::tess.numVertexes = numv;
}
/*
=============
RB_SurfaceTriangles
=============
*/

unsafe extern "C" fn RB_SurfaceTriangles(mut srf: *mut crate::tr_local_h::srfTriangles_t) {
    let mut i: i32 = 0;
    let mut dv: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut texCoords: *mut f32 = 0 as *mut f32;
    let mut color: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut dlightBits: i32 = 0;
    let mut needsNormal: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    dlightBits = (*srf).dlightBits;
    crate::src::renderergl1::tr_shade::tess.dlightBits |= dlightBits;
    if crate::src::renderergl1::tr_shade::tess.numVertexes + (*srf).numVerts >= 1000
        || crate::src::renderergl1::tr_shade::tess.numIndexes + (*srf).numIndexes >= 6 * 1000
    {
        RB_CheckOverflow((*srf).numVerts, (*srf).numIndexes);
    }
    i = 0;
    while i < (*srf).numIndexes {
        crate::src::renderergl1::tr_shade::tess.indexes
            [(crate::src::renderergl1::tr_shade::tess.numIndexes + i + 0) as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes
                + *(*srf).indexes.offset((i + 0) as isize))
                as crate::tr_local_h::glIndex_t;
        crate::src::renderergl1::tr_shade::tess.indexes
            [(crate::src::renderergl1::tr_shade::tess.numIndexes + i + 1) as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes
                + *(*srf).indexes.offset((i + 1) as isize))
                as crate::tr_local_h::glIndex_t;
        crate::src::renderergl1::tr_shade::tess.indexes
            [(crate::src::renderergl1::tr_shade::tess.numIndexes + i + 2) as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes
                + *(*srf).indexes.offset((i + 2) as isize))
                as crate::tr_local_h::glIndex_t;
        i += 3
    }
    crate::src::renderergl1::tr_shade::tess.numIndexes += (*srf).numIndexes;
    dv = (*srf).verts;
    xyz = crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
        .as_mut_ptr();
    normal = crate::src::renderergl1::tr_shade::tess.normal
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
        .as_mut_ptr();
    texCoords = crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0]
        .as_mut_ptr();
    color = crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
        .as_mut_ptr();
    needsNormal = (*crate::src::renderergl1::tr_shade::tess.shader).needsNormal;
    i = 0;
    while i < (*srf).numVerts {
        *xyz.offset(0) = (*dv).xyz[0];
        *xyz.offset(1) = (*dv).xyz[1];
        *xyz.offset(2) = (*dv).xyz[2];
        if needsNormal as u64 != 0 {
            *normal.offset(0) = (*dv).normal[0];
            *normal.offset(1) = (*dv).normal[1];
            *normal.offset(2) = (*dv).normal[2]
        }
        *texCoords.offset(0) = (*dv).st[0];
        *texCoords.offset(1) = (*dv).st[1];
        *texCoords.offset(2) = (*dv).lightmap[0];
        *texCoords.offset(3) = (*dv).lightmap[1];
        *(color as *mut i32) = *((*dv).color.as_mut_ptr() as *mut i32);
        i += 1;
        dv = dv.offset(1);
        xyz = xyz.offset(4);
        normal = normal.offset(4);
        texCoords = texCoords.offset(4);
        color = color.offset(4)
    }
    i = 0;
    while i < (*srf).numVerts {
        crate::src::renderergl1::tr_shade::tess.vertexDlightBits
            [(crate::src::renderergl1::tr_shade::tess.numVertexes + i) as usize] = dlightBits;
        i += 1
    }
    crate::src::renderergl1::tr_shade::tess.numVertexes += (*srf).numVerts;
}
/*
==============
RB_SurfaceBeam
==============
*/

unsafe extern "C" fn RB_SurfaceBeam() {
    let mut e: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut i: i32 = 0;
    let mut perpvec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut direction: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut normalized_direction: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start_points: [crate::src::qcommon::q_shared::vec3_t; 6] = [[0.; 3]; 6];
    let mut end_points: [crate::src::qcommon::q_shared::vec3_t; 6] = [[0.; 3]; 6];
    let mut oldorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    e = &mut (*crate::src::renderergl1::tr_backend::backEnd.currentEntity).e;
    oldorigin[0] = (*e).oldorigin[0];
    oldorigin[1] = (*e).oldorigin[1];
    oldorigin[2] = (*e).oldorigin[2];
    origin[0] = (*e).origin[0];
    origin[1] = (*e).origin[1];
    origin[2] = (*e).origin[2];
    direction[0] = oldorigin[0] - origin[0];
    normalized_direction[0] = direction[0];
    direction[1] = oldorigin[1] - origin[1];
    normalized_direction[1] = direction[1];
    direction[2] = oldorigin[2] - origin[2];
    normalized_direction[2] = direction[2];
    if crate::src::qcommon::q_math::VectorNormalize(normalized_direction.as_mut_ptr()) == 0f32 {
        return;
    }
    crate::src::qcommon::q_math::PerpendicularVector(
        perpvec.as_mut_ptr(),
        normalized_direction.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    perpvec[0] = perpvec[0] * 4f32;
    perpvec[1] = perpvec[1] * 4f32;
    perpvec[2] = perpvec[2] * 4f32;
    i = 0;
    while i < 6 {
        crate::src::qcommon::q_math::RotatePointAroundVector(
            start_points[i as usize].as_mut_ptr(),
            normalized_direction.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            perpvec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (360.0 / 6f64 * i as f64) as f32,
        );
        //		VectorAdd( start_points[i], origin, start_points[i] );
        end_points[i as usize][0] = start_points[i as usize][0] + direction[0];
        end_points[i as usize][1] = start_points[i as usize][1] + direction[1];
        end_points[i as usize][2] = start_points[i as usize][2] + direction[2];
        i += 1
    }
    crate::src::renderergl1::tr_backend::GL_Bind(crate::src::renderergl1::tr_main::tr.whiteImage);
    crate::src::renderergl1::tr_backend::GL_State((0x2i32 | 0x20) as usize);
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(1f32, 0f32, 0f32);
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(0x5u32);
    i = 0;
    while i <= 6 {
        crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
            start_points[(i % 6) as usize].as_mut_ptr(),
        );
        crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
            end_points[(i % 6) as usize].as_mut_ptr(),
        );
        i += 1
    }
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
}
//================================================================================

unsafe extern "C" fn DoRailCore(
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut up: *const crate::src::qcommon::q_shared::vec_t,
    mut len: f32,
    mut spanWidth: f32,
) {
    let mut spanWidth2: f32 = 0.;
    let mut vbase: i32 = 0;
    let mut t: f32 = len / 256.0;
    if crate::src::renderergl1::tr_shade::tess.numVertexes + 4 >= 1000
        || crate::src::renderergl1::tr_shade::tess.numIndexes + 6 >= 6 * 1000
    {
        RB_CheckOverflow(4i32, 6i32);
    }
    vbase = crate::src::renderergl1::tr_shade::tess.numVertexes;
    spanWidth2 = -spanWidth;
    // FIXME: use quad stamp?
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        *start.offset(0) + *up.offset(0) * spanWidth;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        *start.offset(1) + *up.offset(1) * spanWidth;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        *start.offset(2) + *up.offset(2) * spanWidth;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][0] = 0f32;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][1] = 0f32;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[0] as i32 as f64
            * 0.25) as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[1] as i32 as f64
            * 0.25) as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[2] as i32 as f64
            * 0.25) as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        *start.offset(0) + *up.offset(0) * spanWidth2;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        *start.offset(1) + *up.offset(1) * spanWidth2;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        *start.offset(2) + *up.offset(2) * spanWidth2;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][0] = 0f32;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][1] = 1f32;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[0];
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[1];
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[2];
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        *end.offset(0) + *up.offset(0) * spanWidth;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        *end.offset(1) + *up.offset(1) * spanWidth;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        *end.offset(2) + *up.offset(2) * spanWidth;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][0] = t;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][1] = 0f32;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[0];
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[1];
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[2];
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        *end.offset(0) + *up.offset(0) * spanWidth2;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        *end.offset(1) + *up.offset(1) * spanWidth2;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        *end.offset(2) + *up.offset(2) * spanWidth2;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][0] = t;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][1] = 1f32;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[0];
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[1];
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[2];
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    let fresh6 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh6 as usize] =
        vbase as crate::tr_local_h::glIndex_t;
    let fresh7 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh7 as usize] =
        (vbase + 1) as crate::tr_local_h::glIndex_t;
    let fresh8 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh8 as usize] =
        (vbase + 2) as crate::tr_local_h::glIndex_t;
    let fresh9 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh9 as usize] =
        (vbase + 2) as crate::tr_local_h::glIndex_t;
    let fresh10 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh10 as usize] =
        (vbase + 1) as crate::tr_local_h::glIndex_t;
    let fresh11 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh11 as usize] =
        (vbase + 3) as crate::tr_local_h::glIndex_t;
}

unsafe extern "C" fn DoRailDiscs(
    mut numSegs: i32,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut dir: *const crate::src::qcommon::q_shared::vec_t,
    mut right: *const crate::src::qcommon::q_shared::vec_t,
    mut up: *const crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: i32 = 0;
    let mut pos: [crate::src::qcommon::q_shared::vec3_t; 4] = [[0.; 3]; 4];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut spanWidth: i32 = (*crate::src::renderergl1::tr_init::r_railWidth).integer;
    let mut c: f32 = 0.;
    let mut s: f32 = 0.;
    let mut scale: f32 = 0.;
    if numSegs > 1 {
        numSegs -= 1
    }
    if numSegs == 0 {
        return;
    }
    scale = 0.25;
    i = 0;
    while i < 4 {
        c = crate::stdlib::cos((45 + i * 90) as f64 * 3.14159265358979323846 / 180f64) as f32;
        s = crate::stdlib::sin((45 + i * 90) as f64 * 3.14159265358979323846 / 180f64) as f32;
        v[0] = (*right.offset(0) * c + *up.offset(0) * s) * scale * spanWidth as f32;
        v[1] = (*right.offset(1) * c + *up.offset(1) * s) * scale * spanWidth as f32;
        v[2] = (*right.offset(2) * c + *up.offset(2) * s) * scale * spanWidth as f32;
        pos[i as usize][0] = *start.offset(0) + v[0];
        pos[i as usize][1] = *start.offset(1) + v[1];
        pos[i as usize][2] = *start.offset(2) + v[2];
        if numSegs > 1 {
            // offset by 1 segment if we're doing a long distance shot
            pos[i as usize][0] = pos[i as usize][0] + *dir.offset(0);
            pos[i as usize][1] = pos[i as usize][1] + *dir.offset(1);
            pos[i as usize][2] = pos[i as usize][2] + *dir.offset(2)
        }
        i += 1
    }
    i = 0;
    while i < numSegs {
        let mut j: i32 = 0;
        if crate::src::renderergl1::tr_shade::tess.numVertexes + 4 >= 1000
            || crate::src::renderergl1::tr_shade::tess.numIndexes + 6 >= 6 * 1000
        {
            RB_CheckOverflow(4i32, 6i32);
        }

        for j in 0..4 {
            crate::src::renderergl1::tr_shade::tess.xyz
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
                pos[j as usize][0];

            crate::src::renderergl1::tr_shade::tess.xyz
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
                pos[j as usize][1];

            crate::src::renderergl1::tr_shade::tess.xyz
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
                pos[j as usize][2];

            crate::src::renderergl1::tr_shade::tess.texCoords
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][0] =
                (j < 2) as i32 as crate::src::qcommon::q_shared::vec_t;

            crate::src::renderergl1::tr_shade::tess.texCoords
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0][1] =
                (j != 0 && j != 3) as i32 as crate::src::qcommon::q_shared::vec_t;

            crate::src::renderergl1::tr_shade::tess.vertexColors
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0] =
                (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                    .e
                    .shaderRGBA[0];

            crate::src::renderergl1::tr_shade::tess.vertexColors
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1] =
                (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                    .e
                    .shaderRGBA[1];

            crate::src::renderergl1::tr_shade::tess.vertexColors
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2] =
                (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                    .e
                    .shaderRGBA[2];

            crate::src::renderergl1::tr_shade::tess.numVertexes += 1;

            pos[j as usize][0] = pos[j as usize][0] + *dir.offset(0);

            pos[j as usize][1] = pos[j as usize][1] + *dir.offset(1);

            pos[j as usize][2] = pos[j as usize][2] + *dir.offset(2);
        }
        let fresh12 = crate::src::renderergl1::tr_shade::tess.numIndexes;
        crate::src::renderergl1::tr_shade::tess.numIndexes =
            crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
        crate::src::renderergl1::tr_shade::tess.indexes[fresh12 as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes - 4 + 0)
                as crate::tr_local_h::glIndex_t;
        let fresh13 = crate::src::renderergl1::tr_shade::tess.numIndexes;
        crate::src::renderergl1::tr_shade::tess.numIndexes =
            crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
        crate::src::renderergl1::tr_shade::tess.indexes[fresh13 as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes - 4 + 1)
                as crate::tr_local_h::glIndex_t;
        let fresh14 = crate::src::renderergl1::tr_shade::tess.numIndexes;
        crate::src::renderergl1::tr_shade::tess.numIndexes =
            crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
        crate::src::renderergl1::tr_shade::tess.indexes[fresh14 as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes - 4 + 3)
                as crate::tr_local_h::glIndex_t;
        let fresh15 = crate::src::renderergl1::tr_shade::tess.numIndexes;
        crate::src::renderergl1::tr_shade::tess.numIndexes =
            crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
        crate::src::renderergl1::tr_shade::tess.indexes[fresh15 as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes - 4 + 3)
                as crate::tr_local_h::glIndex_t;
        let fresh16 = crate::src::renderergl1::tr_shade::tess.numIndexes;
        crate::src::renderergl1::tr_shade::tess.numIndexes =
            crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
        crate::src::renderergl1::tr_shade::tess.indexes[fresh16 as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes - 4 + 1)
                as crate::tr_local_h::glIndex_t;
        let fresh17 = crate::src::renderergl1::tr_shade::tess.numIndexes;
        crate::src::renderergl1::tr_shade::tess.numIndexes =
            crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
        crate::src::renderergl1::tr_shade::tess.indexes[fresh17 as usize] =
            (crate::src::renderergl1::tr_shade::tess.numVertexes - 4 + 2)
                as crate::tr_local_h::glIndex_t;
        i += 1
    }
}
/*
** RB_SurfaceRailRinges
*/

unsafe extern "C" fn RB_SurfaceRailRings() {
    let mut e: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut numSegs: i32 = 0;
    let mut len: i32 = 0;
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    e = &mut (*crate::src::renderergl1::tr_backend::backEnd.currentEntity).e;
    start[0] = (*e).oldorigin[0];
    start[1] = (*e).oldorigin[1];
    start[2] = (*e).oldorigin[2];
    end[0] = (*e).origin[0];
    end[1] = (*e).origin[1];
    end[2] = (*e).origin[2];
    // compute variables
    vec[0] = end[0] - start[0];
    vec[1] = end[1] - start[1];
    vec[2] = end[2] - start[2];
    len = crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr()) as i32;
    crate::src::qcommon::q_math::MakeNormalVectors(
        vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    numSegs = (len as f32 / (*crate::src::renderergl1::tr_init::r_railSegmentLength).value) as i32;
    if numSegs <= 0 {
        numSegs = 1
    }
    vec[0] = vec[0] * (*crate::src::renderergl1::tr_init::r_railSegmentLength).value;
    vec[1] = vec[1] * (*crate::src::renderergl1::tr_init::r_railSegmentLength).value;
    vec[2] = vec[2] * (*crate::src::renderergl1::tr_init::r_railSegmentLength).value;
    DoRailDiscs(
        numSegs,
        start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
}
/*
** RB_SurfaceRailCore
*/

unsafe extern "C" fn RB_SurfaceRailCore() {
    let mut e: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut len: i32 = 0;
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    e = &mut (*crate::src::renderergl1::tr_backend::backEnd.currentEntity).e;
    start[0] = (*e).oldorigin[0];
    start[1] = (*e).oldorigin[1];
    start[2] = (*e).oldorigin[2];
    end[0] = (*e).origin[0];
    end[1] = (*e).origin[1];
    end[2] = (*e).origin[2];
    vec[0] = end[0] - start[0];
    vec[1] = end[1] - start[1];
    vec[2] = end[2] - start[2];
    len = crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr()) as i32;
    // compute side vector
    v1[0] = start[0]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0];
    v1[1] = start[1]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1];
    v1[2] = start[2]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2];
    crate::src::qcommon::q_math::VectorNormalize(v1.as_mut_ptr());
    v2[0] = end[0]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0];
    v2[1] = end[1]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1];
    v2[2] = end[2]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2];
    crate::src::qcommon::q_math::VectorNormalize(v2.as_mut_ptr());
    CrossProduct(
        v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::VectorNormalize(right.as_mut_ptr());
    DoRailCore(
        start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        len as f32,
        (*crate::src::renderergl1::tr_init::r_railCoreWidth).integer as f32,
    );
}
/*
** RB_SurfaceLightningBolt
*/

unsafe extern "C" fn RB_SurfaceLightningBolt() {
    let mut e: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut len: i32 = 0;
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut i: i32 = 0;
    e = &mut (*crate::src::renderergl1::tr_backend::backEnd.currentEntity).e;
    end[0] = (*e).oldorigin[0];
    end[1] = (*e).oldorigin[1];
    end[2] = (*e).oldorigin[2];
    start[0] = (*e).origin[0];
    start[1] = (*e).origin[1];
    start[2] = (*e).origin[2];
    // compute variables
    vec[0] = end[0] - start[0];
    vec[1] = end[1] - start[1];
    vec[2] = end[2] - start[2];
    len = crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr()) as i32;
    // compute side vector
    v1[0] = start[0]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0];
    v1[1] = start[1]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1];
    v1[2] = start[2]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2];
    crate::src::qcommon::q_math::VectorNormalize(v1.as_mut_ptr());
    v2[0] = end[0]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0];
    v2[1] = end[1]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1];
    v2[2] = end[2]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2];
    crate::src::qcommon::q_math::VectorNormalize(v2.as_mut_ptr());
    CrossProduct(
        v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::VectorNormalize(right.as_mut_ptr());
    i = 0;
    while i < 4 {
        let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        DoRailCore(
            start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            right.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            len as f32,
            8f32,
        );
        crate::src::qcommon::q_math::RotatePointAroundVector(
            temp.as_mut_ptr(),
            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            right.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            45f32,
        );
        right[0] = temp[0];
        right[1] = temp[1];
        right[2] = temp[2];
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
=============================================================

RENDERER BACK END FUNCTIONS

=============================================================
*/
/*
=============================================================

RENDERER BACK END COMMAND QUEUE

=============================================================
*/
// these are sort of arbitrary limits.
// the limits apply to the sum of all scenes in a frame --
// the main view, all the 3D icons, etc
// all of the information needed by the back end must be
// contained in a backEndData_t
//[MAX_POLYS];
//[MAX_POLYVERTS];
// the second one may not be allocated
/*
** VectorArrayNormalize
*
* The inputs to this routing seem to always be close to length = 1.0 (about 0.6 to 2.0)
* This means that we don't have to worry about zero length or enormously long vectors.
*/
#[no_mangle]

pub unsafe extern "C" fn VectorArrayNormalize(
    mut normals: *mut crate::src::qcommon::q_shared::vec4_t,
    mut count: u32,
) {
    loop
    //    assert(count);
    // No assembly version for this architecture, or C_ONLY defined
    // given the input, it's safe to call VectorNormalizeFast
    {
        let fresh18 = count;
        count = count.wrapping_sub(1);
        if !(fresh18 != 0) {
            break;
        }
        VectorNormalizeFast((*normals.offset(0)).as_mut_ptr());
        normals = normals.offset(1)
    }
}
/*
** LerpMeshVertexes
*/

unsafe extern "C" fn LerpMeshVertexes_scalar(
    mut surf: *mut crate::qfiles_h::md3Surface_t,
    mut backlerp: f32,
) {
    let mut oldXyz: *mut i16 = 0 as *mut i16;
    let mut newXyz: *mut i16 = 0 as *mut i16;
    let mut oldNormals: *mut i16 = 0 as *mut i16;
    let mut newNormals: *mut i16 = 0 as *mut i16;
    let mut outXyz: *mut f32 = 0 as *mut f32;
    let mut outNormal: *mut f32 = 0 as *mut f32;
    let mut oldXyzScale: f32 = 0.;
    let mut newXyzScale: f32 = 0.;
    let mut oldNormalScale: f32 = 0.;
    let mut newNormalScale: f32 = 0.;
    let mut vertNum: i32 = 0;
    let mut lat: u32 = 0;
    let mut lng: u32 = 0;
    let mut numVerts: i32 = 0;
    outXyz = crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
        .as_mut_ptr();
    outNormal = crate::src::renderergl1::tr_shade::tess.normal
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
        .as_mut_ptr();
    newXyz = ((surf as *mut crate::src::qcommon::q_shared::byte)
        .offset((*surf).ofsXyzNormals as isize) as *mut i16)
        .offset(
            ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                .e
                .frame
                * (*surf).numVerts
                * 4) as isize,
        );
    newNormals = newXyz.offset(3);
    newXyzScale = (1.0 / 64f64 * (1.0 - backlerp as f64)) as f32;
    newNormalScale = (1.0 - backlerp as f64) as f32;
    numVerts = (*surf).numVerts;
    if backlerp == 0f32 {
        //
        // just copy the vertexes
        //
        vertNum = 0;
        while vertNum < numVerts {
            *outXyz.offset(0) = *newXyz.offset(0) as i32 as f32 * newXyzScale;
            *outXyz.offset(1) = *newXyz.offset(1) as i32 as f32 * newXyzScale;
            *outXyz.offset(2) = *newXyz.offset(2) as i32 as f32 * newXyzScale;
            lat = (*newNormals.offset(0) as i32 >> 8 & 0xff) as u32;
            lng = (*newNormals.offset(0) as i32 & 0xff) as u32;
            lat = lat.wrapping_mul((1024i32 / 256) as u32);
            lng = lng.wrapping_mul((1024i32 / 256) as u32);
            // decode X as cos( lat ) * sin( long )
            // decode Y as sin( lat ) * sin( long )
            // decode Z as cos( long )
            *outNormal.offset(0) = crate::src::renderergl1::tr_main::tr.sinTable
                [(lat.wrapping_add((1024i32 / 4) as u32) & (1024i32 - 1) as u32) as usize]
                * crate::src::renderergl1::tr_main::tr.sinTable[lng as usize];
            *outNormal.offset(1) = crate::src::renderergl1::tr_main::tr.sinTable[lat as usize]
                * crate::src::renderergl1::tr_main::tr.sinTable[lng as usize];
            *outNormal.offset(2) = crate::src::renderergl1::tr_main::tr.sinTable
                [(lng.wrapping_add((1024i32 / 4) as u32) & (1024i32 - 1) as u32) as usize];
            vertNum += 1;
            newXyz = newXyz.offset(4);
            newNormals = newNormals.offset(4);
            outXyz = outXyz.offset(4);
            outNormal = outNormal.offset(4)
        }
    } else {
        //
        // interpolate and copy the vertex and normal
        //
        oldXyz = ((surf as *mut crate::src::qcommon::q_shared::byte)
            .offset((*surf).ofsXyzNormals as isize) as *mut i16)
            .offset(
                ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                    .e
                    .oldframe
                    * (*surf).numVerts
                    * 4) as isize,
            );
        oldNormals = oldXyz.offset(3);
        oldXyzScale = (1.0 / 64f64 * backlerp as f64) as f32;
        oldNormalScale = backlerp;
        vertNum = 0;
        while vertNum < numVerts {
            let mut uncompressedOldNormal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            let mut uncompressedNewNormal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            //			VectorNormalize (outNormal);
            *outXyz.offset(0) = *oldXyz.offset(0) as i32 as f32 * oldXyzScale
                + *newXyz.offset(0) as i32 as f32 * newXyzScale;
            *outXyz.offset(1) = *oldXyz.offset(1) as i32 as f32 * oldXyzScale
                + *newXyz.offset(1) as i32 as f32 * newXyzScale;
            *outXyz.offset(2) = *oldXyz.offset(2) as i32 as f32 * oldXyzScale
                + *newXyz.offset(2) as i32 as f32 * newXyzScale;
            lat = (*newNormals.offset(0) as i32 >> 8 & 0xff) as u32;
            lng = (*newNormals.offset(0) as i32 & 0xff) as u32;
            lat = lat.wrapping_mul(4u32);
            lng = lng.wrapping_mul(4u32);
            uncompressedNewNormal[0] = crate::src::renderergl1::tr_main::tr.sinTable
                [(lat.wrapping_add((1024i32 / 4) as u32) & (1024i32 - 1) as u32) as usize]
                * crate::src::renderergl1::tr_main::tr.sinTable[lng as usize];
            uncompressedNewNormal[1] = crate::src::renderergl1::tr_main::tr.sinTable[lat as usize]
                * crate::src::renderergl1::tr_main::tr.sinTable[lng as usize];
            uncompressedNewNormal[2] = crate::src::renderergl1::tr_main::tr.sinTable
                [(lng.wrapping_add((1024i32 / 4) as u32) & (1024i32 - 1) as u32) as usize];
            lat = (*oldNormals.offset(0) as i32 >> 8 & 0xff) as u32;
            lng = (*oldNormals.offset(0) as i32 & 0xff) as u32;
            lat = lat.wrapping_mul(4u32);
            lng = lng.wrapping_mul(4u32);
            uncompressedOldNormal[0] = crate::src::renderergl1::tr_main::tr.sinTable
                [(lat.wrapping_add((1024i32 / 4) as u32) & (1024i32 - 1) as u32) as usize]
                * crate::src::renderergl1::tr_main::tr.sinTable[lng as usize];
            uncompressedOldNormal[1] = crate::src::renderergl1::tr_main::tr.sinTable[lat as usize]
                * crate::src::renderergl1::tr_main::tr.sinTable[lng as usize];
            uncompressedOldNormal[2] = crate::src::renderergl1::tr_main::tr.sinTable
                [(lng.wrapping_add((1024i32 / 4) as u32) & (1024i32 - 1) as u32) as usize];
            *outNormal.offset(0) = uncompressedOldNormal[0] * oldNormalScale
                + uncompressedNewNormal[0] * newNormalScale;
            *outNormal.offset(1) = uncompressedOldNormal[1] * oldNormalScale
                + uncompressedNewNormal[1] * newNormalScale;
            *outNormal.offset(2) = uncompressedOldNormal[2] * oldNormalScale
                + uncompressedNewNormal[2] * newNormalScale;
            vertNum += 1;
            oldXyz = oldXyz.offset(4);
            newXyz = newXyz.offset(4);
            oldNormals = oldNormals.offset(4);
            newNormals = newNormals.offset(4);
            outXyz = outXyz.offset(4);
            outNormal = outNormal.offset(4)
        }
        VectorArrayNormalize(
            crate::src::renderergl1::tr_shade::tess.normal
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
                .as_mut_ptr() as *mut crate::src::qcommon::q_shared::vec4_t,
            numVerts as u32,
        );
    };
}

unsafe extern "C" fn LerpMeshVertexes(
    mut surf: *mut crate::qfiles_h::md3Surface_t,
    mut backlerp: f32,
) {
    // interpolate the xyz
    // FIXME: interpolate lat/long instead?
    // idppc_altivec
    LerpMeshVertexes_scalar(surf, backlerp);
}
/*
=============
RB_SurfaceMesh
=============
*/

unsafe extern "C" fn RB_SurfaceMesh(mut surface: *mut crate::qfiles_h::md3Surface_t) {
    let mut j: i32 = 0;
    let mut backlerp: f32 = 0.;
    let mut triangles: *mut i32 = 0 as *mut i32;
    let mut texCoords: *mut f32 = 0 as *mut f32;
    let mut indexes: i32 = 0;
    let mut Bob: i32 = 0;
    let mut Doug: i32 = 0;
    let mut numVerts: i32 = 0;
    if (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
        .e
        .oldframe
        == (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .frame
    {
        backlerp = 0f32
    } else {
        backlerp = (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .backlerp
    }
    if crate::src::renderergl1::tr_shade::tess.numVertexes + (*surface).numVerts >= 1000
        || crate::src::renderergl1::tr_shade::tess.numIndexes + (*surface).numTriangles * 3
            >= 6 * 1000
    {
        RB_CheckOverflow((*surface).numVerts, (*surface).numTriangles * 3i32);
    }
    LerpMeshVertexes(surface, backlerp);
    triangles = (surface as *mut crate::src::qcommon::q_shared::byte)
        .offset((*surface).ofsTriangles as isize) as *mut i32;
    indexes = (*surface).numTriangles * 3;
    Bob = crate::src::renderergl1::tr_shade::tess.numIndexes;
    Doug = crate::src::renderergl1::tr_shade::tess.numVertexes;
    j = 0;
    while j < indexes {
        crate::src::renderergl1::tr_shade::tess.indexes[(Bob + j) as usize] =
            (Doug + *triangles.offset(j as isize)) as crate::tr_local_h::glIndex_t;
        j += 1
    }
    crate::src::renderergl1::tr_shade::tess.numIndexes += indexes;
    texCoords = (surface as *mut crate::src::qcommon::q_shared::byte)
        .offset((*surface).ofsSt as isize) as *mut f32;
    numVerts = (*surface).numVerts;
    j = 0;
    while j < numVerts {
        crate::src::renderergl1::tr_shade::tess.texCoords[(Doug + j) as usize][0][0] =
            *texCoords.offset((j * 2 + 0) as isize);
        crate::src::renderergl1::tr_shade::tess.texCoords[(Doug + j) as usize][0][1] =
            *texCoords.offset((j * 2 + 1) as isize);
        j += 1
        // FIXME: fill in lightmapST for completeness?
    }
    crate::src::renderergl1::tr_shade::tess.numVertexes += (*surface).numVerts;
}
/*
==============
RB_SurfaceFace
==============
*/

unsafe extern "C" fn RB_SurfaceFace(mut surf: *mut crate::tr_local_h::srfSurfaceFace_t) {
    let mut i: i32 = 0;
    let mut indices: *mut u32 = 0 as *mut u32;
    let mut tessIndexes: *mut crate::tr_local_h::glIndex_t = 0 as *mut crate::tr_local_h::glIndex_t;
    let mut v: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut ndx: i32 = 0;
    let mut Bob: i32 = 0;
    let mut numPoints: i32 = 0;
    let mut dlightBits: i32 = 0;
    if crate::src::renderergl1::tr_shade::tess.numVertexes + (*surf).numPoints >= 1000
        || crate::src::renderergl1::tr_shade::tess.numIndexes + (*surf).numIndices >= 6 * 1000
    {
        RB_CheckOverflow((*surf).numPoints, (*surf).numIndices);
    }
    dlightBits = (*surf).dlightBits;
    crate::src::renderergl1::tr_shade::tess.dlightBits |= dlightBits;
    indices = (surf as *mut i8).offset((*surf).ofsIndices as isize) as *mut u32;
    Bob = crate::src::renderergl1::tr_shade::tess.numVertexes;
    tessIndexes = crate::src::renderergl1::tr_shade::tess
        .indexes
        .as_mut_ptr()
        .offset(crate::src::renderergl1::tr_shade::tess.numIndexes as isize);
    i = (*surf).numIndices - 1;
    while i >= 0 {
        *tessIndexes.offset(i as isize) = (*indices.offset(i as isize)).wrapping_add(Bob as u32);
        i -= 1
    }
    crate::src::renderergl1::tr_shade::tess.numIndexes += (*surf).numIndices;
    numPoints = (*surf).numPoints;
    if (*crate::src::renderergl1::tr_shade::tess.shader).needsNormal as u64 != 0 {
        normal = (*surf).plane.normal.as_mut_ptr();
        i = 0;
        ndx = crate::src::renderergl1::tr_shade::tess.numVertexes;
        while i < numPoints {
            crate::src::renderergl1::tr_shade::tess.normal[ndx as usize][0] = *normal.offset(0);
            crate::src::renderergl1::tr_shade::tess.normal[ndx as usize][1] = *normal.offset(1);
            crate::src::renderergl1::tr_shade::tess.normal[ndx as usize][2] = *normal.offset(2);
            i += 1;
            ndx += 1
        }
    }
    i = 0;
    v = (*(*surf).points.as_mut_ptr().offset(0)).as_mut_ptr();
    ndx = crate::src::renderergl1::tr_shade::tess.numVertexes;
    while i < numPoints {
        crate::src::renderergl1::tr_shade::tess.xyz[ndx as usize][0] = *v.offset(0);
        crate::src::renderergl1::tr_shade::tess.xyz[ndx as usize][1] = *v.offset(1);
        crate::src::renderergl1::tr_shade::tess.xyz[ndx as usize][2] = *v.offset(2);
        crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][0][0] = *v.offset(3);
        crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][0][1] = *v.offset(4);
        crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][1][0] = *v.offset(5);
        crate::src::renderergl1::tr_shade::tess.texCoords[ndx as usize][1][1] = *v.offset(6);
        *(&mut *crate::src::renderergl1::tr_shade::tess
            .vertexColors
            .as_mut_ptr()
            .offset(ndx as isize) as *mut crate::tr_local_h::color4ub_t as *mut u32) =
            *(&mut *v.offset(7) as *mut f32 as *mut u32);
        crate::src::renderergl1::tr_shade::tess.vertexDlightBits[ndx as usize] = dlightBits;
        i += 1;
        v = v.offset(8);
        ndx += 1
    }
    crate::src::renderergl1::tr_shade::tess.numVertexes += (*surf).numPoints;
}

unsafe extern "C" fn LodErrorForVolume(
    mut local: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
) -> f32 {
    let mut world: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: f32 = 0.;
    // never let it go negative
    if (*crate::src::renderergl1::tr_init::r_lodCurveError).value < 0f32 {
        return 0f32;
    }
    world[0] = *local.offset(0) * crate::src::renderergl1::tr_backend::backEnd.or.axis[0][0]
        + *local.offset(1) * crate::src::renderergl1::tr_backend::backEnd.or.axis[1][0]
        + *local.offset(2) * crate::src::renderergl1::tr_backend::backEnd.or.axis[2][0]
        + crate::src::renderergl1::tr_backend::backEnd.or.origin[0];
    world[1] = *local.offset(0) * crate::src::renderergl1::tr_backend::backEnd.or.axis[0][1]
        + *local.offset(1) * crate::src::renderergl1::tr_backend::backEnd.or.axis[1][1]
        + *local.offset(2) * crate::src::renderergl1::tr_backend::backEnd.or.axis[2][1]
        + crate::src::renderergl1::tr_backend::backEnd.or.origin[1];
    world[2] = *local.offset(0) * crate::src::renderergl1::tr_backend::backEnd.or.axis[0][2]
        + *local.offset(1) * crate::src::renderergl1::tr_backend::backEnd.or.axis[1][2]
        + *local.offset(2) * crate::src::renderergl1::tr_backend::backEnd.or.axis[2][2]
        + crate::src::renderergl1::tr_backend::backEnd.or.origin[2];
    world[0] = world[0]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0];
    world[1] = world[1]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1];
    world[2] = world[2]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2];
    d = world[0]
        * crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0][0]
        + world[1]
            * crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[0][1]
        + world[2]
            * crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[0][2];
    if d < 0f32 {
        d = -d
    }
    d -= radius;
    if d < 1f32 {
        d = 1f32
    }
    return (*crate::src::renderergl1::tr_init::r_lodCurveError).value / d;
}
/*
=============
RB_SurfaceGrid

Just copy the grid of points and triangulate
=============
*/

unsafe extern "C" fn RB_SurfaceGrid(mut cv: *mut crate::tr_local_h::srfGridMesh_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut texCoords: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut color: *mut u8 = 0 as *mut u8;
    let mut dv: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut rows: i32 = 0;
    let mut irows: i32 = 0;
    let mut vrows: i32 = 0;
    let mut used: i32 = 0;
    let mut widthTable: [i32; 65] = [0; 65];
    let mut heightTable: [i32; 65] = [0; 65];
    let mut lodError: f32 = 0.;
    let mut lodWidth: i32 = 0;
    let mut lodHeight: i32 = 0;
    let mut numVertexes: i32 = 0;
    let mut dlightBits: i32 = 0;
    let mut vDlightBits: *mut i32 = 0 as *mut i32;
    let mut needsNormal: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    dlightBits = (*cv).dlightBits;
    crate::src::renderergl1::tr_shade::tess.dlightBits |= dlightBits;
    // determine the allowable discrepance
    lodError = LodErrorForVolume((*cv).lodOrigin.as_mut_ptr(), (*cv).lodRadius);
    // determine which rows and columns of the subdivision
    // we are actually going to use
    widthTable[0] = 0;
    lodWidth = 1;
    i = 1;
    while i < (*cv).width - 1 {
        if *(*cv).widthLodError.offset(i as isize) <= lodError {
            widthTable[lodWidth as usize] = i;
            lodWidth += 1
        }
        i += 1
    }
    widthTable[lodWidth as usize] = (*cv).width - 1;
    lodWidth += 1;
    heightTable[0] = 0;
    lodHeight = 1;
    i = 1;
    while i < (*cv).height - 1 {
        if *(*cv).heightLodError.offset(i as isize) <= lodError {
            heightTable[lodHeight as usize] = i;
            lodHeight += 1
        }
        i += 1
    }
    heightTable[lodHeight as usize] = (*cv).height - 1;
    lodHeight += 1;
    // very large grids may have more points or indexes than can be fit
    // in the tess structure, so we may have to issue it in multiple passes
    used = 0;
    while used < lodHeight - 1 {
        loop
        // see how many rows of both verts and indexes we can add without overflowing
        {
            vrows = (1000 - crate::src::renderergl1::tr_shade::tess.numVertexes) / lodWidth;
            irows =
                (6 * 1000 - crate::src::renderergl1::tr_shade::tess.numIndexes) / (lodWidth * 6);
            // if we don't have enough space for at least one strip, flush the buffer
            if !(vrows < 2 || irows < 1) {
                break;
            }
            crate::src::renderergl1::tr_shade::RB_EndSurface();
            crate::src::renderergl1::tr_shade::RB_BeginSurface(
                crate::src::renderergl1::tr_shade::tess.shader,
                crate::src::renderergl1::tr_shade::tess.fogNum,
            );
        }
        rows = irows;
        if vrows < irows + 1 {
            rows = vrows - 1
        }
        if used + rows > lodHeight {
            rows = lodHeight - used
        }
        numVertexes = crate::src::renderergl1::tr_shade::tess.numVertexes;
        xyz = crate::src::renderergl1::tr_shade::tess.xyz[numVertexes as usize].as_mut_ptr();
        normal = crate::src::renderergl1::tr_shade::tess.normal[numVertexes as usize].as_mut_ptr();
        texCoords =
            crate::src::renderergl1::tr_shade::tess.texCoords[numVertexes as usize][0].as_mut_ptr();
        color = &mut *crate::src::renderergl1::tr_shade::tess
            .vertexColors
            .as_mut_ptr()
            .offset(numVertexes as isize) as *mut crate::tr_local_h::color4ub_t
            as *mut u8;
        vDlightBits = &mut *crate::src::renderergl1::tr_shade::tess
            .vertexDlightBits
            .as_mut_ptr()
            .offset(numVertexes as isize) as *mut i32;
        needsNormal = (*crate::src::renderergl1::tr_shade::tess.shader).needsNormal;
        i = 0;
        while i < rows {
            j = 0;
            while j < lodWidth {
                dv = (*cv)
                    .verts
                    .as_mut_ptr()
                    .offset((heightTable[(used + i) as usize] * (*cv).width) as isize)
                    .offset(widthTable[j as usize] as isize);
                *xyz.offset(0) = (*dv).xyz[0];
                *xyz.offset(1) = (*dv).xyz[1];
                *xyz.offset(2) = (*dv).xyz[2];
                *texCoords.offset(0) = (*dv).st[0];
                *texCoords.offset(1) = (*dv).st[1];
                *texCoords.offset(2) = (*dv).lightmap[0];
                *texCoords.offset(3) = (*dv).lightmap[1];
                if needsNormal as u64 != 0 {
                    *normal.offset(0) = (*dv).normal[0];
                    *normal.offset(1) = (*dv).normal[1];
                    *normal.offset(2) = (*dv).normal[2]
                }
                *(color as *mut u32) = *((*dv).color.as_mut_ptr() as *mut u32);
                let fresh19 = vDlightBits;
                vDlightBits = vDlightBits.offset(1);
                *fresh19 = dlightBits;
                xyz = xyz.offset(4);
                normal = normal.offset(4);
                texCoords = texCoords.offset(4);
                color = color.offset(4);
                j += 1
            }
            i += 1
        }
        // add the indexes
        let mut numIndexes: i32 = 0;
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        h = rows - 1;
        w = lodWidth - 1;
        numIndexes = crate::src::renderergl1::tr_shade::tess.numIndexes;
        i = 0;
        while i < h {
            j = 0;
            while j < w {
                let mut v1: i32 = 0;
                let mut v2: i32 = 0;
                let mut v3: i32 = 0;
                let mut v4: i32 = 0;
                // vertex order to be reckognized as tristrips
                v1 = numVertexes + i * lodWidth + j + 1;
                v2 = v1 - 1;
                v3 = v2 + lodWidth;
                v4 = v3 + 1;
                crate::src::renderergl1::tr_shade::tess.indexes[numIndexes as usize] =
                    v2 as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 1) as usize] =
                    v3 as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 2) as usize] =
                    v1 as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 3) as usize] =
                    v1 as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 4) as usize] =
                    v3 as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 5) as usize] =
                    v4 as crate::tr_local_h::glIndex_t;
                numIndexes += 6;
                j += 1
            }
            i += 1
        }
        crate::src::renderergl1::tr_shade::tess.numIndexes = numIndexes;
        crate::src::renderergl1::tr_shade::tess.numVertexes += rows * lodWidth;
        used += rows - 1
    }
}
/*
===========================================================================

NULL MODEL

===========================================================================
*/
/*
===================
RB_SurfaceAxis

Draws x/y/z lines from the origin for orientation debugging
===================
*/

unsafe extern "C" fn RB_SurfaceAxis() {
    crate::src::renderergl1::tr_backend::GL_Bind(crate::src::renderergl1::tr_main::tr.whiteImage);
    crate::src::renderergl1::tr_backend::GL_State(0x100);
    crate::src::sdl::sdl_glimp::qglLineWidth.expect("non-null function pointer")(3f32);
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(0x1u32);
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(1f32, 0f32, 0f32);
    crate::src::sdl::sdl_glimp::qglVertex3f.expect("non-null function pointer")(0f32, 0f32, 0f32);
    crate::src::sdl::sdl_glimp::qglVertex3f.expect("non-null function pointer")(16f32, 0f32, 0f32);
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(0f32, 1f32, 0f32);
    crate::src::sdl::sdl_glimp::qglVertex3f.expect("non-null function pointer")(0f32, 0f32, 0f32);
    crate::src::sdl::sdl_glimp::qglVertex3f.expect("non-null function pointer")(0f32, 16f32, 0f32);
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(0f32, 0f32, 1f32);
    crate::src::sdl::sdl_glimp::qglVertex3f.expect("non-null function pointer")(0f32, 0f32, 0f32);
    crate::src::sdl::sdl_glimp::qglVertex3f.expect("non-null function pointer")(0f32, 0f32, 16f32);
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglLineWidth.expect("non-null function pointer")(1f32);
}
//===========================================================================
/*
====================
RB_SurfaceEntity

Entities that have a single procedurally generated surface
====================
*/

unsafe extern "C" fn RB_SurfaceEntity(mut surfType: *mut crate::tr_local_h::surfaceType_t) {
    match (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
        .e
        .reType
    {
        2 => {
            RB_SurfaceSprite();
        }
        3 => {
            RB_SurfaceBeam();
        }
        4 => {
            RB_SurfaceRailCore();
        }
        5 => {
            RB_SurfaceRailRings();
        }
        6 => {
            RB_SurfaceLightningBolt();
        }
        _ => {
            RB_SurfaceAxis();
        }
    };
}

unsafe extern "C" fn RB_SurfaceBad(mut surfType: *mut crate::tr_local_h::surfaceType_t) {
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"Bad surface tesselated.\n\x00" as *const u8 as *const i8,
    );
}

unsafe extern "C" fn RB_SurfaceFlare(mut surf: *mut crate::tr_local_h::srfFlare_t) {
    if (*crate::src::renderergl1::tr_init::r_flares).integer != 0 {
        crate::src::renderergl1::tr_flares::RB_AddFlare(
            surf as *mut libc::c_void,
            crate::src::renderergl1::tr_shade::tess.fogNum,
            (*surf).origin.as_mut_ptr(),
            (*surf).color.as_mut_ptr(),
            (*surf).normal.as_mut_ptr(),
        );
    };
}

unsafe extern "C" fn RB_SurfaceSkip(mut surf: *mut libc::c_void) {}
#[no_mangle]

pub static mut rb_surfaceTable: [Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>; 11] = unsafe {
    [
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::surfaceType_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceBad as unsafe extern "C" fn(_: *mut crate::tr_local_h::surfaceType_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceSkip as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::srfSurfaceFace_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceFace
                as unsafe extern "C" fn(_: *mut crate::tr_local_h::srfSurfaceFace_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::srfGridMesh_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceGrid as unsafe extern "C" fn(_: *mut crate::tr_local_h::srfGridMesh_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::srfTriangles_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceTriangles
                as unsafe extern "C" fn(_: *mut crate::tr_local_h::srfTriangles_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::srfPoly_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfacePolychain as unsafe extern "C" fn(_: *mut crate::tr_local_h::srfPoly_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::qfiles_h::md3Surface_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceMesh as unsafe extern "C" fn(_: *mut crate::qfiles_h::md3Surface_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::qfiles_h::mdrSurface_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            crate::src::renderergl1::tr_animation::RB_MDRSurfaceAnim
                as unsafe extern "C" fn(_: *mut crate::qfiles_h::mdrSurface_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::surfaceType_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            crate::src::renderergl1::tr_model_iqm::RB_IQMSurfaceAnim
                as unsafe extern "C" fn(_: *mut crate::tr_local_h::surfaceType_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::srfFlare_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceFlare as unsafe extern "C" fn(_: *mut crate::tr_local_h::srfFlare_t) -> (),
        )),
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::tr_local_h::surfaceType_t) -> ()>,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(Some(
            RB_SurfaceEntity
                as unsafe extern "C" fn(_: *mut crate::tr_local_h::surfaceType_t) -> (),
        )),
    ]
};
