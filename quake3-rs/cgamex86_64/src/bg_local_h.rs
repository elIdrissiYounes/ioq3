#[repr(C)]
#[derive(Copy, Clone)]
pub struct pml_t {
    pub forward: crate::src::qcommon::q_shared::vec3_t,
    pub right: crate::src::qcommon::q_shared::vec3_t,
    pub up: crate::src::qcommon::q_shared::vec3_t,
    pub frametime: libc::c_float,
    pub msec: libc::c_int,
    pub walking: crate::src::qcommon::q_shared::qboolean,
    pub groundPlane: crate::src::qcommon::q_shared::qboolean,
    pub groundTrace: crate::src::qcommon::q_shared::trace_t,
    pub impactSpeed: libc::c_float,
    pub previous_origin: crate::src::qcommon::q_shared::vec3_t,
    pub previous_velocity: crate::src::qcommon::q_shared::vec3_t,
    pub previous_waterlevel: libc::c_int,
}
