use ::libc;

pub mod q_shared_h {

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
    #[inline]

    pub unsafe extern "C" fn Distance(
        mut p1: *const crate::src::qcommon::q_shared::vec_t,
        mut p2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        v[0] = *p2.offset(0) - *p1.offset(0);
        v[1] = *p2.offset(1) - *p1.offset(1);
        v[2] = *p2.offset(2) - *p1.offset(2);
        return VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    }
    use crate::stdlib::sqrt;

    // __Q_SHARED_H
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
    use crate::stdlib::strtod;
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_particles::q_shared_h::Distance;
pub use crate::src::cgame::cg_particles::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
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
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
pub use crate::cg_local_h::LEMT_BLOOD;
pub use crate::cg_local_h::LEMT_BURN;
pub use crate::cg_local_h::LEMT_NONE;
pub use crate::cg_local_h::LE_EXPLOSION;
pub use crate::cg_local_h::LE_FADE_RGB;
pub use crate::cg_local_h::LE_FALL_SCALE_FADE;
pub use crate::cg_local_h::LE_FRAGMENT;
pub use crate::cg_local_h::LE_MARK;
pub use crate::cg_local_h::LE_MOVE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCOREPLUM;
pub use crate::cg_local_h::LE_SPRITE_EXPLOSION;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_particles::stdlib_float_h::atof;
pub use crate::src::cgame::cg_particles::stdlib_h::atoi;
pub use crate::src::cgame::cg_predict::CG_Trace;
pub use crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShader;
use crate::stdlib::cos;
use crate::stdlib::floor;
use crate::stdlib::memset;
pub use crate::stdlib::rand;
use crate::stdlib::sin;
use crate::stdlib::sqrt;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;

pub type cparticle_t = particle_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct particle_s {
    pub next: *mut particle_s,
    pub time: f32,
    pub endtime: f32,
    pub org: crate::src::qcommon::q_shared::vec3_t,
    pub vel: crate::src::qcommon::q_shared::vec3_t,
    pub accel: crate::src::qcommon::q_shared::vec3_t,
    pub color: i32,
    pub colorvel: f32,
    pub alpha: f32,
    pub alphavel: f32,
    pub type_0: i32,
    pub pshader: crate::src::qcommon::q_shared::qhandle_t,
    pub height: f32,
    pub width: f32,
    pub endheight: f32,
    pub endwidth: f32,
    pub start: f32,
    pub end: f32,
    pub startfade: f32,
    pub rotate: crate::src::qcommon::q_shared::qboolean,
    pub snum: i32,
    pub link: crate::src::qcommon::q_shared::qboolean,
    pub shaderAnim: i32,
    pub roll: i32,
    pub accumroll: i32,
}

pub const P_WEATHER_FLURRY: C2RustUnnamed_25 = 11;

pub const P_WEATHER_TURBULENT: C2RustUnnamed_25 = 5;

pub const P_WEATHER: C2RustUnnamed_25 = 1;

pub const P_ANIM: C2RustUnnamed_25 = 6;

pub const P_FLAT: C2RustUnnamed_25 = 2;

pub const P_FLAT_SCALEUP: C2RustUnnamed_25 = 9;

pub const P_BLEED: C2RustUnnamed_25 = 8;

pub const P_SMOKE_IMPACT: C2RustUnnamed_25 = 12;

pub const P_SMOKE: C2RustUnnamed_25 = 3;

pub const P_SPRITE: C2RustUnnamed_25 = 15;

pub const P_BUBBLE_TURBULENT: C2RustUnnamed_25 = 14;

pub const P_BUBBLE: C2RustUnnamed_25 = 13;

pub const P_BAT: C2RustUnnamed_25 = 7;

pub const P_FLAT_SCALEUP_FADE: C2RustUnnamed_25 = 10;

pub type C2RustUnnamed_25 = u32;

pub const P_ROTATE: C2RustUnnamed_25 = 4;

pub const P_NONE: C2RustUnnamed_25 = 0;

static mut shaderAnimNames: [*mut i8; 32] = [
    b"explode1\x00" as *const u8 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
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

static mut shaderAnims: [[crate::src::qcommon::q_shared::qhandle_t; 64]; 32] = [[0; 64]; 32];

static mut shaderAnimCounts: [i32; 32] = [
    23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static mut shaderAnimSTRatio: [f32; 32] = [
    1.0, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    0., 0., 0., 0., 0., 0., 0., 0., 0.,
];

static mut numShaderAnims: i32 = 0;
#[no_mangle]

pub static mut active_particles: *mut cparticle_t = 0 as *mut cparticle_t;
#[no_mangle]

pub static mut free_particles: *mut cparticle_t = 0 as *mut cparticle_t;
#[no_mangle]

pub static mut particles: [cparticle_t; 1024] = [cparticle_t {
    next: 0 as *mut particle_s,
    time: 0.,
    endtime: 0.,
    org: [0.; 3],
    vel: [0.; 3],
    accel: [0.; 3],
    color: 0,
    colorvel: 0.,
    alpha: 0.,
    alphavel: 0.,
    type_0: 0,
    pshader: 0,
    height: 0.,
    width: 0.,
    endheight: 0.,
    endwidth: 0.,
    start: 0.,
    end: 0.,
    startfade: 0.,
    rotate: crate::src::qcommon::q_shared::qfalse,
    snum: 0,
    link: crate::src::qcommon::q_shared::qfalse,
    shaderAnim: 0,
    roll: 0,
    accumroll: 0,
}; 1024];
#[no_mangle]

pub static mut cl_numparticles: i32 = 1024;
#[no_mangle]

pub static mut initparticles: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut vforward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut vright: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut vup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut rforward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut rright: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut rup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut oldtime: f32 = 0.;
// Ridah
/*
===============
CL_ClearParticles
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ClearParticles() {
    let mut i: i32 = 0;
    crate::stdlib::memset(
        particles.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[cparticle_t; 1024]>(),
    );
    free_particles = &mut *particles.as_mut_ptr().offset(0) as *mut cparticle_t;
    active_particles = 0 as *mut cparticle_t;
    i = 0;
    while i < cl_numparticles {
        particles[i as usize].next =
            &mut *particles.as_mut_ptr().offset((i + 1) as isize) as *mut cparticle_t;
        particles[i as usize].type_0 = 0;
        i += 1
    }
    particles[(cl_numparticles - 1) as usize].next = 0 as *mut particle_s;
    oldtime = crate::src::cgame::cg_main::cg.time as f32;
    // Ridah, init the shaderAnims
    i = 0;
    while !shaderAnimNames[i as usize].is_null() {
        let mut j: i32 = 0;
        j = 0;
        while j < shaderAnimCounts[i as usize] {
            shaderAnims[i as usize][j as usize] =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    crate::src::qcommon::q_shared::va(
                        b"%s%i\x00" as *const u8 as *mut i8,
                        shaderAnimNames[i as usize],
                        j + 1i32,
                    ),
                );
            j += 1
        }
        i += 1
    }
    numShaderAnims = i;
    // done.
    initparticles = crate::src::qcommon::q_shared::qtrue;
}
/*
=====================
CG_AddParticleToScene
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticleToScene(
    mut p: *mut cparticle_t,
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut alpha: f32,
) {
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut verts: [crate::tr_types_h::polyVert_t; 4] = [crate::tr_types_h::polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    let mut time: f32 = 0.;
    let mut time2: f32 = 0.;
    let mut ratio: f32 = 0.;
    let mut invratio: f32 = 0.;
    let mut color: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut TRIverts: [crate::tr_types_h::polyVert_t; 3] = [crate::tr_types_h::polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 3];
    let mut rright2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rup2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*p).type_0 == P_WEATHER as i32
        || (*p).type_0 == P_WEATHER_TURBULENT as i32
        || (*p).type_0 == P_WEATHER_FLURRY as i32
        || (*p).type_0 == P_BUBBLE as i32
        || (*p).type_0 == P_BUBBLE_TURBULENT as i32
    {
        // create a front facing polygon
        if (*p).type_0 != P_WEATHER_FLURRY as i32 {
            if (*p).type_0 == P_BUBBLE as i32 || (*p).type_0 == P_BUBBLE_TURBULENT as i32 {
                if *org.offset(2) > (*p).end {
                    (*p).time = crate::src::cgame::cg_main::cg.time as f32; // Ridah, fixes rare snow flakes that flicker on the ground
                    (*p).org[0] = *org.offset(0); // Ridah, fixes rare snow flakes that flicker on the ground
                    (*p).org[1] = *org.offset(1);
                    (*p).org[2] = *org.offset(2);
                    (*p).org[2] = ((*p).start as f64
                        + 2.0
                            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                            * 4f64)
                        as crate::src::qcommon::q_shared::vec_t;
                    if (*p).type_0 == P_BUBBLE_TURBULENT as i32 {
                        (*p).vel[0] = (2.0
                            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                            * 4f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        (*p).vel[1] = (2.0
                            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                            * 4f64)
                            as crate::src::qcommon::q_shared::vec_t
                    }
                }
            } else if *org.offset(2) < (*p).end {
                (*p).time = crate::src::cgame::cg_main::cg.time as f32;
                (*p).org[0] = *org.offset(0);
                (*p).org[1] = *org.offset(1);
                (*p).org[2] = *org.offset(2);
                while (*p).org[2] < (*p).end {
                    (*p).org[2] += (*p).start - (*p).end
                }
                if (*p).type_0 == P_WEATHER_TURBULENT as i32 {
                    (*p).vel[0] = (2.0
                        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                        * 16f64)
                        as crate::src::qcommon::q_shared::vec_t;
                    (*p).vel[1] =
                        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                            * 16f64) as crate::src::qcommon::q_shared::vec_t
                }
            }
            // Rafael snow pvs check
            if (*p).link as u64 == 0 {
                return;
            }
            (*p).alpha = 1f32
        }
        // Ridah, had to do this or MAX_POLYS is being exceeded in village1.bsp
        if Distance(
            (*crate::src::cgame::cg_main::cg.snap)
                .ps
                .origin
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            org as *const crate::src::qcommon::q_shared::vec_t,
        ) > 1024f32
        {
            return;
        }
        // done.
        if (*p).type_0 == P_BUBBLE as i32 || (*p).type_0 == P_BUBBLE_TURBULENT as i32 {
            point[0] = *org.offset(0) + vup[0] * -(*p).height;
            point[1] = *org.offset(1) + vup[1] * -(*p).height;
            point[2] = *org.offset(2) + vup[2] * -(*p).height;
            point[0] = point[0] + vright[0] * -(*p).width;
            point[1] = point[1] + vright[1] * -(*p).width;
            point[2] = point[2] + vright[2] * -(*p).width;
            verts[0].xyz[0] = point[0];
            verts[0].xyz[1] = point[1];
            verts[0].xyz[2] = point[2];
            verts[0].st[0] = 0f32;
            verts[0].st[1] = 0f32;
            verts[0].modulate[0] = 255;
            verts[0].modulate[1] = 255;
            verts[0].modulate[2] = 255;
            verts[0].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte;
            point[0] = *org.offset(0) + vup[0] * -(*p).height;
            point[1] = *org.offset(1) + vup[1] * -(*p).height;
            point[2] = *org.offset(2) + vup[2] * -(*p).height;
            point[0] = point[0] + vright[0] * (*p).width;
            point[1] = point[1] + vright[1] * (*p).width;
            point[2] = point[2] + vright[2] * (*p).width;
            verts[1].xyz[0] = point[0];
            verts[1].xyz[1] = point[1];
            verts[1].xyz[2] = point[2];
            verts[1].st[0] = 0f32;
            verts[1].st[1] = 1f32;
            verts[1].modulate[0] = 255;
            verts[1].modulate[1] = 255;
            verts[1].modulate[2] = 255;
            verts[1].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte;
            point[0] = *org.offset(0) + vup[0] * (*p).height;
            point[1] = *org.offset(1) + vup[1] * (*p).height;
            point[2] = *org.offset(2) + vup[2] * (*p).height;
            point[0] = point[0] + vright[0] * (*p).width;
            point[1] = point[1] + vright[1] * (*p).width;
            point[2] = point[2] + vright[2] * (*p).width;
            verts[2].xyz[0] = point[0];
            verts[2].xyz[1] = point[1];
            verts[2].xyz[2] = point[2];
            verts[2].st[0] = 1f32;
            verts[2].st[1] = 1f32;
            verts[2].modulate[0] = 255;
            verts[2].modulate[1] = 255;
            verts[2].modulate[2] = 255;
            verts[2].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte;
            point[0] = *org.offset(0) + vup[0] * (*p).height;
            point[1] = *org.offset(1) + vup[1] * (*p).height;
            point[2] = *org.offset(2) + vup[2] * (*p).height;
            point[0] = point[0] + vright[0] * -(*p).width;
            point[1] = point[1] + vright[1] * -(*p).width;
            point[2] = point[2] + vright[2] * -(*p).width;
            verts[3].xyz[0] = point[0];
            verts[3].xyz[1] = point[1];
            verts[3].xyz[2] = point[2];
            verts[3].st[0] = 1f32;
            verts[3].st[1] = 0f32;
            verts[3].modulate[0] = 255;
            verts[3].modulate[1] = 255;
            verts[3].modulate[2] = 255;
            verts[3].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte
        } else {
            point[0] = *org.offset(0) + vup[0] * -(*p).height;
            point[1] = *org.offset(1) + vup[1] * -(*p).height;
            point[2] = *org.offset(2) + vup[2] * -(*p).height;
            point[0] = point[0] + vright[0] * -(*p).width;
            point[1] = point[1] + vright[1] * -(*p).width;
            point[2] = point[2] + vright[2] * -(*p).width;
            TRIverts[0].xyz[0] = point[0];
            TRIverts[0].xyz[1] = point[1];
            TRIverts[0].xyz[2] = point[2];
            TRIverts[0].st[0] = 1f32;
            TRIverts[0].st[1] = 0f32;
            TRIverts[0].modulate[0] = 255;
            TRIverts[0].modulate[1] = 255;
            TRIverts[0].modulate[2] = 255;
            TRIverts[0].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte;
            point[0] = *org.offset(0) + vup[0] * (*p).height;
            point[1] = *org.offset(1) + vup[1] * (*p).height;
            point[2] = *org.offset(2) + vup[2] * (*p).height;
            point[0] = point[0] + vright[0] * -(*p).width;
            point[1] = point[1] + vright[1] * -(*p).width;
            point[2] = point[2] + vright[2] * -(*p).width;
            TRIverts[1].xyz[0] = point[0];
            TRIverts[1].xyz[1] = point[1];
            TRIverts[1].xyz[2] = point[2];
            TRIverts[1].st[0] = 0f32;
            TRIverts[1].st[1] = 0f32;
            TRIverts[1].modulate[0] = 255;
            TRIverts[1].modulate[1] = 255;
            TRIverts[1].modulate[2] = 255;
            TRIverts[1].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte;
            point[0] = *org.offset(0) + vup[0] * (*p).height;
            point[1] = *org.offset(1) + vup[1] * (*p).height;
            point[2] = *org.offset(2) + vup[2] * (*p).height;
            point[0] = point[0] + vright[0] * (*p).width;
            point[1] = point[1] + vright[1] * (*p).width;
            point[2] = point[2] + vright[2] * (*p).width;
            TRIverts[2].xyz[0] = point[0];
            TRIverts[2].xyz[1] = point[1];
            TRIverts[2].xyz[2] = point[2];
            TRIverts[2].st[0] = 0f32;
            TRIverts[2].st[1] = 1f32;
            TRIverts[2].modulate[0] = 255;
            TRIverts[2].modulate[1] = 255;
            TRIverts[2].modulate[2] = 255;
            TRIverts[2].modulate[3] = (255f32 * (*p).alpha) as crate::src::qcommon::q_shared::byte
        }
    } else if (*p).type_0 == P_SPRITE as i32 {
        let mut rr: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut ru: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut rotate_ang: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        color[0] = 1f32;
        color[1] = 1f32;
        color[2] = 0.5;
        time = crate::src::cgame::cg_main::cg.time as f32 - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).roll != 0 {
            crate::src::qcommon::q_math::vectoangles(
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0].as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                rotate_ang.as_mut_ptr(),
            );
            rotate_ang[2] += (*p).roll as f32;
            crate::src::qcommon::q_math::AngleVectors(
                rotate_ang.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rr.as_mut_ptr(),
                ru.as_mut_ptr(),
            );
        }
        if (*p).roll != 0 {
            point[0] = *org.offset(0) + ru[0] * -height;
            point[1] = *org.offset(1) + ru[1] * -height;
            point[2] = *org.offset(2) + ru[2] * -height;
            point[0] = point[0] + rr[0] * -width;
            point[1] = point[1] + rr[1] * -width;
            point[2] = point[2] + rr[2] * -width
        } else {
            point[0] = *org.offset(0) + vup[0] * -height;
            point[1] = *org.offset(1) + vup[1] * -height;
            point[2] = *org.offset(2) + vup[2] * -height;
            point[0] = point[0] + vright[0] * -width;
            point[1] = point[1] + vright[1] * -width;
            point[2] = point[2] + vright[2] * -width
        }
        verts[0].xyz[0] = point[0];
        verts[0].xyz[1] = point[1];
        verts[0].xyz[2] = point[2];
        verts[0].st[0] = 0f32;
        verts[0].st[1] = 0f32;
        verts[0].modulate[0] = 255;
        verts[0].modulate[1] = 255;
        verts[0].modulate[2] = 255;
        verts[0].modulate[3] = 255;
        if (*p).roll != 0 {
            point[0] = point[0] + ru[0] * (2f32 * height);
            point[1] = point[1] + ru[1] * (2f32 * height);
            point[2] = point[2] + ru[2] * (2f32 * height)
        } else {
            point[0] = point[0] + vup[0] * (2f32 * height);
            point[1] = point[1] + vup[1] * (2f32 * height);
            point[2] = point[2] + vup[2] * (2f32 * height)
        }
        verts[1].xyz[0] = point[0];
        verts[1].xyz[1] = point[1];
        verts[1].xyz[2] = point[2];
        verts[1].st[0] = 0f32;
        verts[1].st[1] = 1f32;
        verts[1].modulate[0] = 255;
        verts[1].modulate[1] = 255;
        verts[1].modulate[2] = 255;
        verts[1].modulate[3] = 255;
        if (*p).roll != 0 {
            point[0] = point[0] + rr[0] * (2f32 * width);
            point[1] = point[1] + rr[1] * (2f32 * width);
            point[2] = point[2] + rr[2] * (2f32 * width)
        } else {
            point[0] = point[0] + vright[0] * (2f32 * width);
            point[1] = point[1] + vright[1] * (2f32 * width);
            point[2] = point[2] + vright[2] * (2f32 * width)
        }
        verts[2].xyz[0] = point[0];
        verts[2].xyz[1] = point[1];
        verts[2].xyz[2] = point[2];
        verts[2].st[0] = 1f32;
        verts[2].st[1] = 1f32;
        verts[2].modulate[0] = 255;
        verts[2].modulate[1] = 255;
        verts[2].modulate[2] = 255;
        verts[2].modulate[3] = 255;
        if (*p).roll != 0 {
            point[0] = point[0] + ru[0] * (-2f32 * height);
            point[1] = point[1] + ru[1] * (-2f32 * height);
            point[2] = point[2] + ru[2] * (-2f32 * height)
        } else {
            point[0] = point[0] + vup[0] * (-2f32 * height);
            point[1] = point[1] + vup[1] * (-2f32 * height);
            point[2] = point[2] + vup[2] * (-2f32 * height)
        }
        verts[3].xyz[0] = point[0];
        verts[3].xyz[1] = point[1];
        verts[3].xyz[2] = point[2];
        verts[3].st[0] = 1f32;
        verts[3].st[1] = 0f32;
        verts[3].modulate[0] = 255;
        verts[3].modulate[1] = 255;
        verts[3].modulate[2] = 255;
        verts[3].modulate[3] = 255
    } else if (*p).type_0 == P_SMOKE as i32 || (*p).type_0 == P_SMOKE_IMPACT as i32 {
        // create a front rotating facing polygon
        if (*p).type_0 == P_SMOKE_IMPACT as i32
            && Distance(
                (*crate::src::cgame::cg_main::cg.snap)
                    .ps
                    .origin
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                org as *const crate::src::qcommon::q_shared::vec_t,
            ) > 1024f32
        {
            return;
        }
        if (*p).color == 2 {
            color[0] = 0.22;
            color[1] = 0.0;
            color[2] = 0.0
        } else if (*p).color == 4 {
            let mut len: f32 = 0.;
            let mut greyit: f32 = 0.;
            let mut val: f32 = 0.;
            len = Distance(
                (*crate::src::cgame::cg_main::cg.snap)
                    .ps
                    .origin
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                org as *const crate::src::qcommon::q_shared::vec_t,
            );
            if len == 0. {
                len = 1f32
            }
            val = 4096f32 / len;
            greyit = (0.25 * val as f64) as f32;
            if greyit as f64 > 0.5 {
                greyit = 0.5
            }
            color[0] = greyit;
            color[1] = greyit;
            color[2] = greyit
        } else {
            color[0] = 1f32;
            color[1] = 1f32;
            color[2] = 1f32
        }
        time = crate::src::cgame::cg_main::cg.time as f32 - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if crate::src::cgame::cg_main::cg.time as f32 > (*p).startfade {
            invratio = 1f32
                - (crate::src::cgame::cg_main::cg.time as f32 - (*p).startfade)
                    / ((*p).endtime - (*p).startfade);
            if (*p).color == 3 {
                let mut fval: f32 = 0.;
                fval = invratio * invratio;
                if fval < 0f32 {
                    fval = 0f32
                }
                color[0] = fval;
                color[1] = fval;
                color[2] = fval
            }
            invratio *= (*p).alpha
        } else {
            invratio = 1f32 * (*p).alpha
        }
        if crate::src::cgame::cg_main::cgs.glconfig.hardwareType == crate::tr_types_h::GLHW_RAGEPRO
        {
            invratio = 1f32
        }
        if invratio > 1f32 {
            invratio = 1f32
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).type_0 != P_SMOKE_IMPACT as i32 {
            let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            crate::src::qcommon::q_math::vectoangles(
                rforward.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                temp.as_mut_ptr(),
            );
            (*p).accumroll += (*p).roll;
            temp[2] = (temp[2] as f64 + (*p).accumroll as f64 * 0.1)
                as crate::src::qcommon::q_shared::vec_t;
            crate::src::qcommon::q_math::AngleVectors(
                temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rright2.as_mut_ptr(),
                rup2.as_mut_ptr(),
            );
        } else {
            rright2[0] = rright[0];
            rright2[1] = rright[1];
            rright2[2] = rright[2];
            rup2[0] = rup[0];
            rup2[1] = rup[1];
            rup2[2] = rup[2]
        }
        if (*p).rotate as u64 != 0 {
            point[0] = *org.offset(0) + rup2[0] * -height;
            point[1] = *org.offset(1) + rup2[1] * -height;
            point[2] = *org.offset(2) + rup2[2] * -height;
            point[0] = point[0] + rright2[0] * -width;
            point[1] = point[1] + rright2[1] * -width;
            point[2] = point[2] + rright2[2] * -width
        } else {
            point[0] = *org.offset(0) + vup[0] * -(*p).height;
            point[1] = *org.offset(1) + vup[1] * -(*p).height;
            point[2] = *org.offset(2) + vup[2] * -(*p).height;
            point[0] = point[0] + vright[0] * -(*p).width;
            point[1] = point[1] + vright[1] * -(*p).width;
            point[2] = point[2] + vright[2] * -(*p).width
        }
        verts[0].xyz[0] = point[0];
        verts[0].xyz[1] = point[1];
        verts[0].xyz[2] = point[2];
        verts[0].st[0] = 0f32;
        verts[0].st[1] = 0f32;
        verts[0].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[0].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[0].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[0].modulate[3] = (255f32 * invratio) as crate::src::qcommon::q_shared::byte;
        if (*p).rotate as u64 != 0 {
            point[0] = *org.offset(0) + rup2[0] * -height;
            point[1] = *org.offset(1) + rup2[1] * -height;
            point[2] = *org.offset(2) + rup2[2] * -height;
            point[0] = point[0] + rright2[0] * width;
            point[1] = point[1] + rright2[1] * width;
            point[2] = point[2] + rright2[2] * width
        } else {
            point[0] = *org.offset(0) + vup[0] * -(*p).height;
            point[1] = *org.offset(1) + vup[1] * -(*p).height;
            point[2] = *org.offset(2) + vup[2] * -(*p).height;
            point[0] = point[0] + vright[0] * (*p).width;
            point[1] = point[1] + vright[1] * (*p).width;
            point[2] = point[2] + vright[2] * (*p).width
        }
        verts[1].xyz[0] = point[0];
        verts[1].xyz[1] = point[1];
        verts[1].xyz[2] = point[2];
        verts[1].st[0] = 0f32;
        verts[1].st[1] = 1f32;
        verts[1].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[1].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[1].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[1].modulate[3] = (255f32 * invratio) as crate::src::qcommon::q_shared::byte;
        if (*p).rotate as u64 != 0 {
            point[0] = *org.offset(0) + rup2[0] * height;
            point[1] = *org.offset(1) + rup2[1] * height;
            point[2] = *org.offset(2) + rup2[2] * height;
            point[0] = point[0] + rright2[0] * width;
            point[1] = point[1] + rright2[1] * width;
            point[2] = point[2] + rright2[2] * width
        } else {
            point[0] = *org.offset(0) + vup[0] * (*p).height;
            point[1] = *org.offset(1) + vup[1] * (*p).height;
            point[2] = *org.offset(2) + vup[2] * (*p).height;
            point[0] = point[0] + vright[0] * (*p).width;
            point[1] = point[1] + vright[1] * (*p).width;
            point[2] = point[2] + vright[2] * (*p).width
        }
        verts[2].xyz[0] = point[0];
        verts[2].xyz[1] = point[1];
        verts[2].xyz[2] = point[2];
        verts[2].st[0] = 1f32;
        verts[2].st[1] = 1f32;
        verts[2].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[2].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[2].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[2].modulate[3] = (255f32 * invratio) as crate::src::qcommon::q_shared::byte;
        if (*p).rotate as u64 != 0 {
            point[0] = *org.offset(0) + rup2[0] * height;
            point[1] = *org.offset(1) + rup2[1] * height;
            point[2] = *org.offset(2) + rup2[2] * height;
            point[0] = point[0] + rright2[0] * -width;
            point[1] = point[1] + rright2[1] * -width;
            point[2] = point[2] + rright2[2] * -width
        } else {
            point[0] = *org.offset(0) + vup[0] * (*p).height;
            point[1] = *org.offset(1) + vup[1] * (*p).height;
            point[2] = *org.offset(2) + vup[2] * (*p).height;
            point[0] = point[0] + vright[0] * -(*p).width;
            point[1] = point[1] + vright[1] * -(*p).width;
            point[2] = point[2] + vright[2] * -(*p).width
        }
        verts[3].xyz[0] = point[0];
        verts[3].xyz[1] = point[1];
        verts[3].xyz[2] = point[2];
        verts[3].st[0] = 1f32;
        verts[3].st[1] = 0f32;
        verts[3].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[3].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[3].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[3].modulate[3] = (255f32 * invratio) as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_BLEED as i32 {
        let mut rr_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut ru_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut rotate_ang_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut alpha_0: f32 = 0.;
        alpha_0 = (*p).alpha;
        if crate::src::cgame::cg_main::cgs.glconfig.hardwareType == crate::tr_types_h::GLHW_RAGEPRO
        {
            alpha_0 = 1f32
        }
        if (*p).roll != 0 {
            crate::src::qcommon::q_math::vectoangles(
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0].as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                rotate_ang_0.as_mut_ptr(),
            );
            rotate_ang_0[2] += (*p).roll as f32;
            crate::src::qcommon::q_math::AngleVectors(
                rotate_ang_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rr_0.as_mut_ptr(),
                ru_0.as_mut_ptr(),
            );
        } else {
            ru_0[0] = vup[0];
            ru_0[1] = vup[1];
            ru_0[2] = vup[2];
            rr_0[0] = vright[0];
            rr_0[1] = vright[1];
            rr_0[2] = vright[2]
        }
        point[0] = *org.offset(0) + ru_0[0] * -(*p).height;
        point[1] = *org.offset(1) + ru_0[1] * -(*p).height;
        point[2] = *org.offset(2) + ru_0[2] * -(*p).height;
        point[0] = point[0] + rr_0[0] * -(*p).width;
        point[1] = point[1] + rr_0[1] * -(*p).width;
        point[2] = point[2] + rr_0[2] * -(*p).width;
        verts[0].xyz[0] = point[0];
        verts[0].xyz[1] = point[1];
        verts[0].xyz[2] = point[2];
        verts[0].st[0] = 0f32;
        verts[0].st[1] = 0f32;
        verts[0].modulate[0] = 111;
        verts[0].modulate[1] = 19;
        verts[0].modulate[2] = 9;
        verts[0].modulate[3] = (255f32 * alpha_0) as crate::src::qcommon::q_shared::byte;
        point[0] = *org.offset(0) + ru_0[0] * -(*p).height;
        point[1] = *org.offset(1) + ru_0[1] * -(*p).height;
        point[2] = *org.offset(2) + ru_0[2] * -(*p).height;
        point[0] = point[0] + rr_0[0] * (*p).width;
        point[1] = point[1] + rr_0[1] * (*p).width;
        point[2] = point[2] + rr_0[2] * (*p).width;
        verts[1].xyz[0] = point[0];
        verts[1].xyz[1] = point[1];
        verts[1].xyz[2] = point[2];
        verts[1].st[0] = 0f32;
        verts[1].st[1] = 1f32;
        verts[1].modulate[0] = 111;
        verts[1].modulate[1] = 19;
        verts[1].modulate[2] = 9;
        verts[1].modulate[3] = (255f32 * alpha_0) as crate::src::qcommon::q_shared::byte;
        point[0] = *org.offset(0) + ru_0[0] * (*p).height;
        point[1] = *org.offset(1) + ru_0[1] * (*p).height;
        point[2] = *org.offset(2) + ru_0[2] * (*p).height;
        point[0] = point[0] + rr_0[0] * (*p).width;
        point[1] = point[1] + rr_0[1] * (*p).width;
        point[2] = point[2] + rr_0[2] * (*p).width;
        verts[2].xyz[0] = point[0];
        verts[2].xyz[1] = point[1];
        verts[2].xyz[2] = point[2];
        verts[2].st[0] = 1f32;
        verts[2].st[1] = 1f32;
        verts[2].modulate[0] = 111;
        verts[2].modulate[1] = 19;
        verts[2].modulate[2] = 9;
        verts[2].modulate[3] = (255f32 * alpha_0) as crate::src::qcommon::q_shared::byte;
        point[0] = *org.offset(0) + ru_0[0] * (*p).height;
        point[1] = *org.offset(1) + ru_0[1] * (*p).height;
        point[2] = *org.offset(2) + ru_0[2] * (*p).height;
        point[0] = point[0] + rr_0[0] * -(*p).width;
        point[1] = point[1] + rr_0[1] * -(*p).width;
        point[2] = point[2] + rr_0[2] * -(*p).width;
        verts[3].xyz[0] = point[0];
        verts[3].xyz[1] = point[1];
        verts[3].xyz[2] = point[2];
        verts[3].st[0] = 1f32;
        verts[3].st[1] = 0f32;
        verts[3].modulate[0] = 111;
        verts[3].modulate[1] = 19;
        verts[3].modulate[2] = 9;
        verts[3].modulate[3] = (255f32 * alpha_0) as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_FLAT_SCALEUP as i32 {
        let mut sinR: f32 = 0.;
        let mut cosR: f32 = 0.;
        if (*p).color == 2 {
            color[0] = 1f32;
            color[1] = 1f32;
            color[2] = 1f32
        } else {
            color[0] = 0.5;
            color[1] = 0.5;
            color[2] = 0.5
        }
        time = crate::src::cgame::cg_main::cg.time as f32 - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if width > (*p).endwidth {
            width = (*p).endwidth
        }
        if height > (*p).endheight {
            height = (*p).endheight
        }
        sinR = (height as f64
            * crate::stdlib::sin((*p).roll as f64 * 3.14159265358979323846 / 180f64)
            * crate::stdlib::sqrt(2f64)) as f32;
        cosR = (width as f64
            * crate::stdlib::cos((*p).roll as f64 * 3.14159265358979323846 / 180f64)
            * crate::stdlib::sqrt(2f64)) as f32;
        verts[0].xyz[0] = *org.offset(0);
        verts[0].xyz[1] = *org.offset(1);
        verts[0].xyz[2] = *org.offset(2);
        verts[0].xyz[0] -= sinR;
        verts[0].xyz[1] -= cosR;
        verts[0].st[0] = 0f32;
        verts[0].st[1] = 0f32;
        verts[0].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[0].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[0].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[0].modulate[3] = 255;
        verts[1].xyz[0] = *org.offset(0);
        verts[1].xyz[1] = *org.offset(1);
        verts[1].xyz[2] = *org.offset(2);
        verts[1].xyz[0] -= cosR;
        verts[1].xyz[1] += sinR;
        verts[1].st[0] = 0f32;
        verts[1].st[1] = 1f32;
        verts[1].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[1].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[1].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[1].modulate[3] = 255;
        verts[2].xyz[0] = *org.offset(0);
        verts[2].xyz[1] = *org.offset(1);
        verts[2].xyz[2] = *org.offset(2);
        verts[2].xyz[0] += sinR;
        verts[2].xyz[1] += cosR;
        verts[2].st[0] = 1f32;
        verts[2].st[1] = 1f32;
        verts[2].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[2].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[2].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[2].modulate[3] = 255;
        verts[3].xyz[0] = *org.offset(0);
        verts[3].xyz[1] = *org.offset(1);
        verts[3].xyz[2] = *org.offset(2);
        verts[3].xyz[0] += cosR;
        verts[3].xyz[1] -= sinR;
        verts[3].st[0] = 1f32;
        verts[3].st[1] = 0f32;
        verts[3].modulate[0] = (255f32 * color[0]) as crate::src::qcommon::q_shared::byte;
        verts[3].modulate[1] = (255f32 * color[1]) as crate::src::qcommon::q_shared::byte;
        verts[3].modulate[2] = (255f32 * color[2]) as crate::src::qcommon::q_shared::byte;
        verts[3].modulate[3] = 255
    } else if (*p).type_0 == P_FLAT as i32 {
        verts[0].xyz[0] = *org.offset(0);
        verts[0].xyz[1] = *org.offset(1);
        verts[0].xyz[2] = *org.offset(2);
        verts[0].xyz[0] -= (*p).height;
        verts[0].xyz[1] -= (*p).width;
        verts[0].st[0] = 0f32;
        verts[0].st[1] = 0f32;
        verts[0].modulate[0] = 255;
        verts[0].modulate[1] = 255;
        verts[0].modulate[2] = 255;
        verts[0].modulate[3] = 255;
        verts[1].xyz[0] = *org.offset(0);
        verts[1].xyz[1] = *org.offset(1);
        verts[1].xyz[2] = *org.offset(2);
        verts[1].xyz[0] -= (*p).height;
        verts[1].xyz[1] += (*p).width;
        verts[1].st[0] = 0f32;
        verts[1].st[1] = 1f32;
        verts[1].modulate[0] = 255;
        verts[1].modulate[1] = 255;
        verts[1].modulate[2] = 255;
        verts[1].modulate[3] = 255;
        verts[2].xyz[0] = *org.offset(0);
        verts[2].xyz[1] = *org.offset(1);
        verts[2].xyz[2] = *org.offset(2);
        verts[2].xyz[0] += (*p).height;
        verts[2].xyz[1] += (*p).width;
        verts[2].st[0] = 1f32;
        verts[2].st[1] = 1f32;
        verts[2].modulate[0] = 255;
        verts[2].modulate[1] = 255;
        verts[2].modulate[2] = 255;
        verts[2].modulate[3] = 255;
        verts[3].xyz[0] = *org.offset(0);
        verts[3].xyz[1] = *org.offset(1);
        verts[3].xyz[2] = *org.offset(2);
        verts[3].xyz[0] += (*p).height;
        verts[3].xyz[1] -= (*p).width;
        verts[3].st[0] = 1f32;
        verts[3].st[1] = 0f32;
        verts[3].modulate[0] = 255;
        verts[3].modulate[1] = 255;
        verts[3].modulate[2] = 255;
        verts[3].modulate[3] = 255
    } else if (*p).type_0 == P_ANIM as i32 {
        let mut rr_1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut ru_1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut rotate_ang_1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        time = crate::src::cgame::cg_main::cg.time as f32 - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if ratio >= 1.0 {
            ratio = 0.9999
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        // Ridah
        // if we are "inside" this sprite, don't draw
        if (Distance(
            (*crate::src::cgame::cg_main::cg.snap)
                .ps
                .origin
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            org as *const crate::src::qcommon::q_shared::vec_t,
        ) as f64)
            < width as f64 / 1.5
        {
            return;
        }
        i = (*p).shaderAnim;
        j = crate::stdlib::floor((ratio * shaderAnimCounts[(*p).shaderAnim as usize] as f32) as f64)
            as i32;
        (*p).pshader = shaderAnims[i as usize][j as usize];
        if (*p).roll != 0 {
            crate::src::qcommon::q_math::vectoangles(
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0].as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                rotate_ang_1.as_mut_ptr(),
            );
            rotate_ang_1[2] += (*p).roll as f32;
            crate::src::qcommon::q_math::AngleVectors(
                rotate_ang_1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rr_1.as_mut_ptr(),
                ru_1.as_mut_ptr(),
            );
        }
        if (*p).roll != 0 {
            point[0] = *org.offset(0) + ru_1[0] * -height;
            point[1] = *org.offset(1) + ru_1[1] * -height;
            point[2] = *org.offset(2) + ru_1[2] * -height;
            point[0] = point[0] + rr_1[0] * -width;
            point[1] = point[1] + rr_1[1] * -width;
            point[2] = point[2] + rr_1[2] * -width
        } else {
            point[0] = *org.offset(0) + vup[0] * -height;
            point[1] = *org.offset(1) + vup[1] * -height;
            point[2] = *org.offset(2) + vup[2] * -height;
            point[0] = point[0] + vright[0] * -width;
            point[1] = point[1] + vright[1] * -width;
            point[2] = point[2] + vright[2] * -width
        }
        verts[0].xyz[0] = point[0];
        verts[0].xyz[1] = point[1];
        verts[0].xyz[2] = point[2];
        verts[0].st[0] = 0f32;
        verts[0].st[1] = 0f32;
        verts[0].modulate[0] = 255;
        verts[0].modulate[1] = 255;
        verts[0].modulate[2] = 255;
        verts[0].modulate[3] = 255;
        if (*p).roll != 0 {
            point[0] = point[0] + ru_1[0] * (2f32 * height);
            point[1] = point[1] + ru_1[1] * (2f32 * height);
            point[2] = point[2] + ru_1[2] * (2f32 * height)
        } else {
            point[0] = point[0] + vup[0] * (2f32 * height);
            point[1] = point[1] + vup[1] * (2f32 * height);
            point[2] = point[2] + vup[2] * (2f32 * height)
        }
        verts[1].xyz[0] = point[0];
        verts[1].xyz[1] = point[1];
        verts[1].xyz[2] = point[2];
        verts[1].st[0] = 0f32;
        verts[1].st[1] = 1f32;
        verts[1].modulate[0] = 255;
        verts[1].modulate[1] = 255;
        verts[1].modulate[2] = 255;
        verts[1].modulate[3] = 255;
        if (*p).roll != 0 {
            point[0] = point[0] + rr_1[0] * (2f32 * width);
            point[1] = point[1] + rr_1[1] * (2f32 * width);
            point[2] = point[2] + rr_1[2] * (2f32 * width)
        } else {
            point[0] = point[0] + vright[0] * (2f32 * width);
            point[1] = point[1] + vright[1] * (2f32 * width);
            point[2] = point[2] + vright[2] * (2f32 * width)
        }
        verts[2].xyz[0] = point[0];
        verts[2].xyz[1] = point[1];
        verts[2].xyz[2] = point[2];
        verts[2].st[0] = 1f32;
        verts[2].st[1] = 1f32;
        verts[2].modulate[0] = 255;
        verts[2].modulate[1] = 255;
        verts[2].modulate[2] = 255;
        verts[2].modulate[3] = 255;
        if (*p).roll != 0 {
            point[0] = point[0] + ru_1[0] * (-2f32 * height);
            point[1] = point[1] + ru_1[1] * (-2f32 * height);
            point[2] = point[2] + ru_1[2] * (-2f32 * height)
        } else {
            point[0] = point[0] + vup[0] * (-2f32 * height);
            point[1] = point[1] + vup[1] * (-2f32 * height);
            point[2] = point[2] + vup[2] * (-2f32 * height)
        }
        verts[3].xyz[0] = point[0];
        verts[3].xyz[1] = point[1];
        verts[3].xyz[2] = point[2];
        verts[3].st[0] = 1f32;
        verts[3].st[1] = 0f32;
        verts[3].modulate[0] = 255;
        verts[3].modulate[1] = 255;
        verts[3].modulate[2] = 255;
        verts[3].modulate[3] = 255
    }
    // done.
    if (*p).pshader == 0 {
        // (SA) temp commented out for DM
        //		CG_Printf ("CG_AddParticleToScene type %d p->pshader == ZERO\n", p->type);
        return;
    }
    if (*p).type_0 == P_WEATHER as i32
        || (*p).type_0 == P_WEATHER_TURBULENT as i32
        || (*p).type_0 == P_WEATHER_FLURRY as i32
    {
        crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene(
            (*p).pshader,
            3i32,
            TRIverts.as_mut_ptr(),
        );
    } else {
        crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene(
            (*p).pshader,
            4i32,
            verts.as_mut_ptr(),
        );
    };
}
// Ridah, made this static so it doesn't interfere with other files

static mut roll: f32 = 0f32;
/*
===============
CG_AddParticles
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticles() {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut alpha: f32 = 0.;
    let mut time: f32 = 0.;
    let mut time2: f32 = 0.;
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut active: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut tail: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut rotate_ang: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if initparticles as u64 == 0 {
        CG_ClearParticles();
    }
    vforward[0] = crate::src::cgame::cg_main::cg.refdef.viewaxis[0][0];
    vforward[1] = crate::src::cgame::cg_main::cg.refdef.viewaxis[0][1];
    vforward[2] = crate::src::cgame::cg_main::cg.refdef.viewaxis[0][2];
    vright[0] = crate::src::cgame::cg_main::cg.refdef.viewaxis[1][0];
    vright[1] = crate::src::cgame::cg_main::cg.refdef.viewaxis[1][1];
    vright[2] = crate::src::cgame::cg_main::cg.refdef.viewaxis[1][2];
    vup[0] = crate::src::cgame::cg_main::cg.refdef.viewaxis[2][0];
    vup[1] = crate::src::cgame::cg_main::cg.refdef.viewaxis[2][1];
    vup[2] = crate::src::cgame::cg_main::cg.refdef.viewaxis[2][2];
    crate::src::qcommon::q_math::vectoangles(
        crate::src::cgame::cg_main::cg.refdef.viewaxis[0].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        rotate_ang.as_mut_ptr(),
    );
    roll =
        (roll as f64 + (crate::src::cgame::cg_main::cg.time as f32 - oldtime) as f64 * 0.1) as f32;
    rotate_ang[2] =
        (rotate_ang[2] as f64 + roll as f64 * 0.9) as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AngleVectors(
        rotate_ang.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        rforward.as_mut_ptr(),
        rright.as_mut_ptr(),
        rup.as_mut_ptr(),
    );
    oldtime = crate::src::cgame::cg_main::cg.time as f32;
    active = 0 as *mut cparticle_t;
    tail = 0 as *mut cparticle_t;
    let mut current_block_54: u64;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        time = ((crate::src::cgame::cg_main::cg.time as f32 - (*p).time) as f64 * 0.001) as f32;
        alpha = (*p).alpha + time * (*p).alphavel;
        if alpha <= 0f32 {
            // faded out
            (*p).next = free_particles;
            free_particles = p;
            (*p).type_0 = 0;
            (*p).color = 0;
            (*p).alpha = 0f32
        } else {
            if (*p).type_0 == P_SMOKE as i32
                || (*p).type_0 == P_ANIM as i32
                || (*p).type_0 == P_BLEED as i32
                || (*p).type_0 == P_SMOKE_IMPACT as i32
            {
                if crate::src::cgame::cg_main::cg.time as f32 > (*p).endtime {
                    (*p).next = free_particles;
                    free_particles = p;
                    (*p).type_0 = 0;
                    (*p).color = 0;
                    (*p).alpha = 0f32;
                    current_block_54 = 12599329904712511516;
                } else {
                    current_block_54 = 11459959175219260272;
                }
            } else {
                current_block_54 = 11459959175219260272;
            }
            match current_block_54 {
                12599329904712511516 => {}
                _ => {
                    if (*p).type_0 == P_WEATHER_FLURRY as i32 {
                        if crate::src::cgame::cg_main::cg.time as f32 > (*p).endtime {
                            (*p).next = free_particles;
                            free_particles = p;
                            (*p).type_0 = 0;
                            (*p).color = 0;
                            (*p).alpha = 0f32;
                            current_block_54 = 12599329904712511516;
                        } else {
                            current_block_54 = 5529461102203738653;
                        }
                    } else {
                        current_block_54 = 5529461102203738653;
                    }
                    match current_block_54 {
                        12599329904712511516 => {}
                        _ => {
                            if (*p).type_0 == P_FLAT_SCALEUP_FADE as i32 {
                                if crate::src::cgame::cg_main::cg.time as f32 > (*p).endtime {
                                    (*p).next = free_particles;
                                    free_particles = p;
                                    (*p).type_0 = 0;
                                    (*p).color = 0;
                                    (*p).alpha = 0f32;
                                    current_block_54 = 12599329904712511516;
                                } else {
                                    current_block_54 = 6717214610478484138;
                                }
                            } else {
                                current_block_54 = 6717214610478484138;
                            }
                            match current_block_54 {
                                12599329904712511516 => {}
                                _ => {
                                    if ((*p).type_0 == P_BAT as i32
                                        || (*p).type_0 == P_SPRITE as i32)
                                        && (*p).endtime < 0f32
                                    {
                                        // temporary sprite
                                        CG_AddParticleToScene(p, (*p).org.as_mut_ptr(), alpha);
                                        (*p).next = free_particles;
                                        free_particles = p;
                                        (*p).type_0 = 0;
                                        (*p).color = 0;
                                        (*p).alpha = 0f32
                                    } else {
                                        (*p).next = 0 as *mut particle_s;
                                        if tail.is_null() {
                                            tail = p;
                                            active = tail
                                        } else {
                                            (*tail).next = p;
                                            tail = p
                                        }
                                        if alpha as f64 > 1.0 {
                                            alpha = 1f32
                                        }
                                        time2 = time * time;
                                        org[0] = (*p).org[0]
                                            + (*p).vel[0] * time
                                            + (*p).accel[0] * time2;
                                        org[1] = (*p).org[1]
                                            + (*p).vel[1] * time
                                            + (*p).accel[1] * time2;
                                        org[2] = (*p).org[2]
                                            + (*p).vel[2] * time
                                            + (*p).accel[2] * time2;
                                        CG_AddParticleToScene(p, org.as_mut_ptr(), alpha);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        p = next
    }
    active_particles = active;
}
/*
======================
CG_AddParticles
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSnowFlurry(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut turb: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qtrue;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSnowFlurry pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).color = 0;
    (*p).alpha = 0.90;
    (*p).alphavel = 0f32;
    (*p).start = (*cent).currentState.origin2[0];
    (*p).end = (*cent).currentState.origin2[1];
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time) as f32;
    (*p).startfade = (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time2) as f32;
    (*p).pshader = pshader;
    if crate::stdlib::rand() % 100 > 90 {
        (*p).height = 32f32;
        (*p).width = 32f32;
        (*p).alpha = 0.10
    } else {
        (*p).height = 1f32;
        (*p).width = 1f32
    }
    (*p).vel[2] = -20f32;
    (*p).type_0 = P_WEATHER_FLURRY as i32;
    if turb as u64 != 0 {
        (*p).vel[2] = -10f32
    }
    (*p).org[0] = (*cent).currentState.origin[0];
    (*p).org[1] = (*cent).currentState.origin[1];
    (*p).org[2] = (*cent).currentState.origin[2];
    (*p).vel[1] = 0f32;
    (*p).vel[0] = (*p).vel[1];
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).vel[0] = ((*p).vel[0] as f64
        + (((*cent).currentState.angles[0] * 32f32) as f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 16f64))
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1] = ((*p).vel[1] as f64
        + (((*cent).currentState.angles[1] * 32f32) as f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 16f64))
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2] += (*cent).currentState.angles[2];
    if turb as u64 != 0 {
        (*p).accel[0] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 16f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[1] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 16f64) as crate::src::qcommon::q_shared::vec_t
    };
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSnow(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin2: *mut crate::src::qcommon::q_shared::vec_t,
    mut turb: i32,
    mut range: f32,
    mut snum: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).color = 0;
    (*p).alpha = 0.40;
    (*p).alphavel = 0f32;
    (*p).start = *origin.offset(2);
    (*p).end = *origin2.offset(2);
    (*p).pshader = pshader;
    (*p).height = 1f32;
    (*p).width = 1f32;
    (*p).vel[2] = -50f32;
    if turb != 0 {
        (*p).type_0 = P_WEATHER_TURBULENT as i32;
        (*p).vel[2] = (-50f64 * 1.3) as crate::src::qcommon::q_shared::vec_t
    } else {
        (*p).type_0 = P_WEATHER as i32
    }
    (*p).org[0] = *origin.offset(0);
    (*p).org[1] = *origin.offset(1);
    (*p).org[2] = *origin.offset(2);
    (*p).org[0] = ((*p).org[0] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * range as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[1] = ((*p).org[1] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * range as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[2] = ((*p).org[2] as f64
        + 2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * ((*p).start - (*p).end) as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1] = 0f32;
    (*p).vel[0] = (*p).vel[1];
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    if turb != 0 {
        (*p).vel[0] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 16f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 16f64) as crate::src::qcommon::q_shared::vec_t
    }
    // Rafael snow pvs check
    (*p).snum = snum;
    (*p).link = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBubble(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin2: *mut crate::src::qcommon::q_shared::vec_t,
    mut turb: i32,
    mut range: f32,
    mut snum: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut randsize: f32 = 0.;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).color = 0;
    (*p).alpha = 0.40;
    (*p).alphavel = 0f32;
    (*p).start = *origin.offset(2);
    (*p).end = *origin2.offset(2);
    (*p).pshader = pshader;
    randsize = (1f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 0.5)
        as f32;
    (*p).height = randsize;
    (*p).width = randsize;
    (*p).vel[2] = (50f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 10f64)
        as crate::src::qcommon::q_shared::vec_t;
    if turb != 0 {
        (*p).type_0 = P_BUBBLE_TURBULENT as i32;
        (*p).vel[2] = (50f64 * 1.3) as crate::src::qcommon::q_shared::vec_t
    } else {
        (*p).type_0 = P_BUBBLE as i32
    }
    (*p).org[0] = *origin.offset(0);
    (*p).org[1] = *origin.offset(1);
    (*p).org[2] = *origin.offset(2);
    (*p).org[0] = ((*p).org[0] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * range as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[1] = ((*p).org[1] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * range as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[2] = ((*p).org[2] as f64
        + 2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * ((*p).start - (*p).end) as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1] = 0f32;
    (*p).vel[0] = (*p).vel[1];
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    if turb != 0 {
        (*p).vel[0] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 4f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 4f64) as crate::src::qcommon::q_shared::vec_t
    }
    // Rafael snow pvs check
    (*p).snum = snum;
    (*p).link = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSmoke(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    // using cent->density = enttime
    //		 cent->frame = startfade
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSmoke == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time) as f32;
    (*p).startfade = (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time2) as f32;
    (*p).color = 0;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).start = (*cent).currentState.origin[2];
    (*p).end = (*cent).currentState.origin2[2];
    (*p).pshader = pshader;
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).height = 8f32;
    (*p).width = 8f32;
    (*p).endheight = 32f32;
    (*p).endwidth = 32f32;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0] = (*cent).currentState.origin[0];
    (*p).org[1] = (*cent).currentState.origin[1];
    (*p).org[2] = (*cent).currentState.origin[2];
    (*p).vel[1] = 0f32;
    (*p).vel[0] = (*p).vel[1];
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).vel[2] = 5f32;
    if (*cent).currentState.frame == 1 {
        // reverse gravity
        (*p).vel[2] *= -1f32
    }
    (*p).roll = (8f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 4f64)
        as i32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBulletDebris(
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut duration: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as f32;
    (*p).startfade = (crate::src::cgame::cg_main::cg.time + duration / 2) as f32;
    (*p).color = 3;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).height = 0.5;
    (*p).width = 0.5;
    (*p).endheight = 0.5;
    (*p).endwidth = 0.5;
    (*p).pshader = crate::src::cgame::cg_main::cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0] = *org.offset(0);
    (*p).org[1] = *org.offset(1);
    (*p).org[2] = *org.offset(2);
    (*p).vel[0] = *vel.offset(0);
    (*p).vel[1] = *vel.offset(1);
    (*p).vel[2] = *vel.offset(2);
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).accel[2] = -60f32;
    (*p).vel[2] += -20f32;
}
/*
======================
CG_ParticleExplosion
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleExplosion(
    mut animStr: *mut i8,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut duration: i32,
    mut sizeStart: i32,
    mut sizeEnd: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut anim: i32 = 0;
    if animStr < 10i32 as *mut i8 {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ParticleExplosion: animStr is probably an index rather than a string\x00"
                as *const u8 as *const i8,
        );
    }
    // find the animation string
    anim = 0; // for sprites that are stretch in either direction
    while !shaderAnimNames[anim as usize].is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(animStr, shaderAnimNames[anim as usize]) == 0 {
            break;
        }
        anim += 1
    }
    if shaderAnimNames[anim as usize].is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ParticleExplosion: unknown animation string: %s\x00" as *const u8 as *const i8,
            animStr,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).alpha = 0.5;
    (*p).alphavel = 0f32;
    if duration < 0 {
        duration *= -(1);
        (*p).roll = 0
    } else {
        (*p).roll = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 179f64) as i32
    }
    (*p).shaderAnim = anim;
    (*p).width = sizeStart as f32;
    (*p).height = sizeStart as f32 * shaderAnimSTRatio[anim as usize];
    (*p).endheight = sizeEnd as f32;
    (*p).endwidth = sizeEnd as f32 * shaderAnimSTRatio[anim as usize];
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as f32;
    (*p).type_0 = P_ANIM as i32;
    (*p).org[0] = *origin.offset(0);
    (*p).org[1] = *origin.offset(1);
    (*p).org[2] = *origin.offset(2);
    (*p).vel[0] = *vel.offset(0);
    (*p).vel[1] = *vel.offset(1);
    (*p).vel[2] = *vel.offset(2);
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
}
// Rafael Shrapnel
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticleShrapnel(mut le: *mut crate::cg_local_h::localEntity_t) {}
// done.
#[no_mangle]

pub unsafe extern "C" fn CG_NewParticleArea(mut num: i32) -> i32 {
    // const char *str;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    let mut type_0: i32 = 0;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut i: i32 = 0;
    let mut range: f32 = 0f32;
    let mut turb: i32 = 0;
    let mut numparticles: i32 = 0;
    let mut snum: i32 = 0;
    str = crate::src::cgame::cg_main::CG_ConfigString(num) as *mut i8;
    if *str.offset(0) == 0 {
        return 0i32;
    }
    // returns type 128 64 or 32
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    type_0 = atoi(token);
    if type_0 == 1 {
        range = 128f32
    } else if type_0 == 2 {
        range = 64f32
    } else if type_0 == 3 {
        range = 32f32
    } else if type_0 == 0 {
        range = 256f32
    } else if type_0 == 4 {
        range = 8f32
    } else if type_0 == 5 {
        range = 16f32
    } else if type_0 == 6 {
        range = 32f32
    } else if type_0 == 7 {
        range = 64f32
    }
    i = 0;
    while i < 3 {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
        origin[i as usize] = atof(token) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    i = 0;
    while i < 3 {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
        origin2[i as usize] = atof(token) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    numparticles = atoi(token);
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    turb = atoi(token);
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    snum = atoi(token);
    i = 0;
    while i < numparticles {
        if type_0 >= 4 {
            CG_ParticleBubble(
                crate::src::cgame::cg_main::cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        } else {
            CG_ParticleSnow(
                crate::src::cgame::cg_main::cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        }
        i += 1
    }
    return 1;
}
#[no_mangle]

pub unsafe extern "C" fn CG_SnowLink(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut particleOn: crate::src::qcommon::q_shared::qboolean,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: i32 = 0;
    id = (*cent).currentState.frame;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_WEATHER as i32 || (*p).type_0 == P_WEATHER_TURBULENT as i32 {
            if (*p).snum == id {
                if particleOn as u64 != 0 {
                    (*p).link = crate::src::qcommon::q_shared::qtrue
                } else {
                    (*p).link = crate::src::qcommon::q_shared::qfalse
                }
            }
        }
        p = next
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleImpactSmokePuff(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleImpactSmokePuff pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).alpha = 0.25;
    (*p).alphavel = 0f32;
    (*p).roll =
        (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 179f64) as i32;
    (*p).pshader = pshader;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + 1000) as f32;
    (*p).startfade = (crate::src::cgame::cg_main::cg.time + 100) as f32;
    (*p).width = (crate::stdlib::rand() % 4 + 8) as f32;
    (*p).height = (crate::stdlib::rand() % 4 + 8) as f32;
    (*p).endheight = (*p).height * 2f32;
    (*p).endwidth = (*p).width * 2f32;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + 500) as f32;
    (*p).type_0 = P_SMOKE_IMPACT as i32;
    (*p).org[0] = *origin.offset(0);
    (*p).org[1] = *origin.offset(1);
    (*p).org[2] = *origin.offset(2);
    (*p).vel[0] = 0f32;
    (*p).vel[1] = 0f32;
    (*p).vel[2] = 20f32;
    (*p).accel[0] = 0f32;
    (*p).accel[1] = 0f32;
    (*p).accel[2] = 20f32;
    (*p).rotate = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_Bleed(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut fleshEntityNum: i32,
    mut duration: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_Particle_Bleed pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).roll = 0;
    (*p).pshader = pshader;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as f32;
    if fleshEntityNum != 0 {
        (*p).startfade = crate::src::cgame::cg_main::cg.time as f32
    } else {
        (*p).startfade = (crate::src::cgame::cg_main::cg.time + 100) as f32
    }
    (*p).width = 4f32;
    (*p).height = 4f32;
    (*p).endheight = (4 + crate::stdlib::rand() % 3) as f32;
    (*p).endwidth = (*p).endheight;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0] = *start.offset(0);
    (*p).org[1] = *start.offset(1);
    (*p).org[2] = *start.offset(2);
    (*p).vel[0] = 0f32;
    (*p).vel[1] = 0f32;
    (*p).vel[2] = -20f32;
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = crate::stdlib::rand() % 179;
    (*p).color = 2;
    (*p).alpha = 0.75;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_OilParticle(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut time: i32 = 0;
    let mut time2: i32 = 0;
    let mut ratio: f32 = 0.;
    let mut duration: f32 = 1500f32;
    time = crate::src::cgame::cg_main::cg.time;
    time2 = crate::src::cgame::cg_main::cg.time + (*cent).currentState.time;
    ratio = 1f32 - time as f32 / time2 as f32;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_Particle_OilParticle == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).roll = 0;
    (*p).pshader = pshader;
    (*p).endtime = crate::src::cgame::cg_main::cg.time as f32 + duration;
    (*p).startfade = (*p).endtime;
    (*p).width = 1f32;
    (*p).height = 3f32;
    (*p).endheight = 3f32;
    (*p).endwidth = 1f32;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0] = (*cent).currentState.origin[0];
    (*p).org[1] = (*cent).currentState.origin[1];
    (*p).org[2] = (*cent).currentState.origin[2];
    (*p).vel[0] = (*cent).currentState.origin2[0] * (16f32 * ratio);
    (*p).vel[1] = (*cent).currentState.origin2[1] * (16f32 * ratio);
    (*p).vel[2] = (*cent).currentState.origin2[2];
    (*p).snum = 1;
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).accel[2] = -20f32;
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = crate::stdlib::rand() % 179;
    (*p).alpha = 0.75;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_OilSlick(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_Particle_OilSlick == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    if (*cent).currentState.angles2[2] != 0. {
        (*p).endtime = crate::src::cgame::cg_main::cg.time as f32 + (*cent).currentState.angles2[2]
    } else {
        (*p).endtime = (crate::src::cgame::cg_main::cg.time + 60000) as f32
    }
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).roll = 0;
    (*p).pshader = pshader;
    if (*cent).currentState.angles2[0] != 0. || (*cent).currentState.angles2[1] != 0. {
        (*p).width = (*cent).currentState.angles2[0];
        (*p).height = (*cent).currentState.angles2[0];
        (*p).endheight = (*cent).currentState.angles2[1];
        (*p).endwidth = (*cent).currentState.angles2[1]
    } else {
        (*p).width = 8f32;
        (*p).height = 8f32;
        (*p).endheight = 16f32;
        (*p).endwidth = 16f32
    }
    (*p).type_0 = P_FLAT_SCALEUP as i32;
    (*p).snum = 1;
    (*p).org[0] = (*cent).currentState.origin[0];
    (*p).org[1] = (*cent).currentState.origin[1];
    (*p).org[2] = (*cent).currentState.origin[2];
    (*p).org[2] = ((*p).org[2] as f64
        + (0.55 + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 0.5))
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0] = 0f32;
    (*p).vel[1] = 0f32;
    (*p).vel[2] = 0f32;
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = crate::stdlib::rand() % 179;
    (*p).alpha = 0.75;
}
#[no_mangle]

pub unsafe extern "C" fn CG_OilSlickRemove(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: i32 = 0;
    id = 1;
    if id == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_OilSlickRevove NULL id\n\x00" as *const u8 as *const i8,
        );
    }
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_FLAT_SCALEUP as i32 {
            if (*p).snum == id {
                (*p).endtime = (crate::src::cgame::cg_main::cg.time + 100) as f32;
                (*p).startfade = (*p).endtime;
                (*p).type_0 = P_FLAT_SCALEUP_FADE as i32
            }
        }
        p = next
    }
}
#[no_mangle]

pub unsafe extern "C" fn ValidBloodPool(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut this_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut x_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut center_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut fwidth: i32 = 0;
    let mut fheight: i32 = 0;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    fwidth = 16;
    fheight = 16;
    normal[0] = 0f32;
    normal[1] = 0f32;
    normal[2] = 1f32;
    crate::src::qcommon::q_math::vectoangles(
        normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    center_pos[0] =
        (*start.offset(0) as f64 + normal[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    center_pos[1] =
        (*start.offset(1) as f64 + normal[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    center_pos[2] =
        (*start.offset(2) as f64 + normal[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    x = -fwidth / 2;
    while x < fwidth {
        x_pos[0] = center_pos[0] + right[0] * x as f32;
        x_pos[1] = center_pos[1] + right[1] * x as f32;
        x_pos[2] = center_pos[2] + right[2] * x as f32;
        y = -fheight / 2;
        while y < fheight {
            this_pos[0] = x_pos[0] + up[0] * y as f32;
            this_pos[1] = x_pos[1] + up[1] * y as f32;
            this_pos[2] = x_pos[2] + up[2] * y as f32;
            end_pos[0] = (this_pos[0] as f64 + normal[0] as f64 * (-0.5 * 2f64))
                as crate::src::qcommon::q_shared::vec_t;
            end_pos[1] = (this_pos[1] as f64 + normal[1] as f64 * (-0.5 * 2f64))
                as crate::src::qcommon::q_shared::vec_t;
            end_pos[2] = (this_pos[2] as f64 + normal[2] as f64 * (-0.5 * 2f64))
                as crate::src::qcommon::q_shared::vec_t;
            crate::src::cgame::cg_predict::CG_Trace(
                &mut trace,
                this_pos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *const crate::src::qcommon::q_shared::vec_t,
                0 as *const crate::src::qcommon::q_shared::vec_t,
                end_pos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                -(1),
                1,
            );
            if trace.entityNum < ((1) << 10) - 2 {
                // may only land on world
                return crate::src::qcommon::q_shared::qfalse;
            }
            if !(trace.startsolid as u64 == 0 && trace.fraction < 1f32) {
                return crate::src::qcommon::q_shared::qfalse;
            }
            y += fheight
        }
        x += fwidth
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_BloodPool(
    mut le: *mut crate::cg_local_h::localEntity_t,
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut tr: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut legit: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rndSize: f32 = 0.;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_BloodPool pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    start[0] = (*tr).endpos[0];
    start[1] = (*tr).endpos[1];
    start[2] = (*tr).endpos[2];
    legit = ValidBloodPool(start.as_mut_ptr());
    if legit as u64 == 0 {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + 3000) as f32;
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).roll = 0;
    (*p).pshader = pshader;
    rndSize = (0.4 + ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 * 0.6) as f32;
    (*p).width = 8f32 * rndSize;
    (*p).height = 8f32 * rndSize;
    (*p).endheight = 16f32 * rndSize;
    (*p).endwidth = 16f32 * rndSize;
    (*p).type_0 = P_FLAT_SCALEUP as i32;
    (*p).org[0] = start[0];
    (*p).org[1] = start[1];
    (*p).org[2] = start[2];
    (*p).vel[0] = 0f32;
    (*p).vel[1] = 0f32;
    (*p).vel[2] = 0f32;
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = crate::stdlib::rand() % 179;
    (*p).alpha = 0.75;
    (*p).color = 2;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBloodCloud(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut length: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut crittersize: f32 = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: i32 = 0;
    dist = 0f32;
    length = VectorLength(dir as *const crate::src::qcommon::q_shared::vec_t);
    crate::src::qcommon::q_math::vectoangles(
        dir as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    crittersize = 32f32;
    if length != 0. {
        dist = length / crittersize
    }
    if dist < 1f32 {
        dist = 1f32
    }
    point[0] = *origin.offset(0);
    point[1] = *origin.offset(1);
    point[2] = *origin.offset(2);
    i = 0;
    while (i as f32) < dist {
        point[0] = point[0] + forward[0] * crittersize;
        point[1] = point[1] + forward[1] * crittersize;
        point[2] = point[2] + forward[2] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = crate::src::cgame::cg_main::cg.time as f32;
        (*p).alpha = 1f32;
        (*p).alphavel = 0f32;
        (*p).roll = 0;
        (*p).pshader = crate::src::cgame::cg_main::cgs.media.smokePuffShader;
        (*p).endtime = ((crate::src::cgame::cg_main::cg.time + 350) as f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 100f64)
            as f32;
        (*p).startfade = crate::src::cgame::cg_main::cg.time as f32;
        (*p).width = 32f32;
        (*p).height = 32f32;
        (*p).endheight = 32f32;
        (*p).endwidth = 32f32;
        (*p).type_0 = P_SMOKE as i32;
        (*p).org[0] = *origin.offset(0);
        (*p).org[1] = *origin.offset(1);
        (*p).org[2] = *origin.offset(2);
        (*p).vel[0] = 0f32;
        (*p).vel[1] = 0f32;
        (*p).vel[2] = -1f32;
        (*p).accel[2] = 0f32;
        (*p).accel[1] = (*p).accel[2];
        (*p).accel[0] = (*p).accel[1];
        (*p).rotate = crate::src::qcommon::q_shared::qfalse;
        (*p).roll = crate::stdlib::rand() % 179;
        (*p).color = 2;
        (*p).alpha = 0.75;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSparks(
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut duration: i32,
    mut x: f32,
    mut y: f32,
    mut speed: f32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as f32;
    (*p).startfade = (crate::src::cgame::cg_main::cg.time + duration / 2) as f32;
    (*p).color = 3;
    (*p).alpha = 0.4;
    (*p).alphavel = 0f32;
    (*p).height = 0.5;
    (*p).width = 0.5;
    (*p).endheight = 0.5;
    (*p).endwidth = 0.5;
    (*p).pshader = crate::src::cgame::cg_main::cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0] = *org.offset(0);
    (*p).org[1] = *org.offset(1);
    (*p).org[2] = *org.offset(2);
    (*p).org[0] = ((*p).org[0] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * x as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[1] = ((*p).org[1] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * y as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0] = *vel.offset(0);
    (*p).vel[1] = *vel.offset(1);
    (*p).vel[2] = *vel.offset(2);
    (*p).accel[2] = 0f32;
    (*p).accel[1] = (*p).accel[2];
    (*p).accel[0] = (*p).accel[1];
    (*p).vel[0] = ((*p).vel[0] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 4f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1] = ((*p).vel[1] as f64
        + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 4f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2] = ((*p).vel[2] as f64
        + (20f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 10f64)
            * speed as f64) as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[0] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 4f64) as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1] = (2.0
        * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
        * 4f64) as crate::src::qcommon::q_shared::vec_t;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleDust(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut length: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut crittersize: f32 = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: i32 = 0;
    dist = 0f32;
    *dir.offset(0) = -*dir.offset(0);
    *dir.offset(1) = -*dir.offset(1);
    *dir.offset(2) = -*dir.offset(2);
    length = VectorLength(dir as *const crate::src::qcommon::q_shared::vec_t);
    crate::src::qcommon::q_math::vectoangles(
        dir as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    crittersize = 32f32;
    if length != 0. {
        dist = length / crittersize
    }
    if dist < 1f32 {
        dist = 1f32
    }
    point[0] = *origin.offset(0);
    point[1] = *origin.offset(1);
    point[2] = *origin.offset(2);
    i = 0;
    while (i as f32) < dist {
        point[0] = point[0] + forward[0] * crittersize;
        point[1] = point[1] + forward[1] * crittersize;
        point[2] = point[2] + forward[2] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = crate::src::cgame::cg_main::cg.time as f32;
        (*p).alpha = 5f32;
        (*p).alphavel = 0f32;
        (*p).roll = 0;
        (*p).pshader = crate::src::cgame::cg_main::cgs.media.smokePuffShader;
        // RF, stay around for long enough to expand and dissipate naturally
        if length != 0. {
            (*p).endtime = ((crate::src::cgame::cg_main::cg.time + 4500) as f64
                + 2.0
                    * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                    * 3500f64) as f32
        } else {
            (*p).endtime = ((crate::src::cgame::cg_main::cg.time + 750) as f64
                + 2.0
                    * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                    * 500f64) as f32
        }
        (*p).startfade = crate::src::cgame::cg_main::cg.time as f32;
        (*p).width = 32f32;
        (*p).height = 32f32;
        // RF, expand while falling
        (*p).endheight = (32f64 * 3.0) as f32;
        (*p).endwidth = (32f64 * 3.0) as f32;
        if length == 0. {
            (*p).width *= 0.2;
            (*p).height *= 0.2;
            (*p).endheight = 16f32;
            (*p).endwidth = 16f32
        }
        (*p).type_0 = P_SMOKE as i32;
        (*p).org[0] = point[0];
        (*p).org[1] = point[1];
        (*p).org[2] = point[2];
        (*p).vel[0] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 6f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 6f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[2] = (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 20f32;
        // RF, add some gravity/randomness
        (*p).accel[0] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 3f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[1] = (2.0
            * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
            * 3f64) as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[2] = (-40f64 * 0.4) as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[2] = 0f32;
        (*p).accel[1] = (*p).accel[2];
        (*p).accel[0] = (*p).accel[1];
        (*p).rotate = crate::src::qcommon::q_shared::qfalse;
        (*p).roll = crate::stdlib::rand() % 179;
        (*p).alpha = 0.75;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleMisc(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut size: i32,
    mut duration: i32,
    mut alpha: f32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleImpactSmokePuff pshader == ZERO!\n\x00" as *const u8 as *const i8,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as f32;
    (*p).alpha = 1f32;
    (*p).alphavel = 0f32;
    (*p).roll = crate::stdlib::rand() % 179;
    (*p).pshader = pshader;
    if duration > 0 {
        (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as f32
    } else {
        (*p).endtime = duration as f32
    }
    (*p).startfade = crate::src::cgame::cg_main::cg.time as f32;
    (*p).width = size as f32;
    (*p).height = size as f32;
    (*p).endheight = size as f32;
    (*p).endwidth = size as f32;
    (*p).type_0 = P_SPRITE as i32;
    (*p).org[0] = *origin.offset(0);
    (*p).org[1] = *origin.offset(1);
    (*p).org[2] = *origin.offset(2);
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
}
