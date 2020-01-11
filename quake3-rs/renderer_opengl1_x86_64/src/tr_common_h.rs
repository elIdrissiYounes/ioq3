pub type imgType_t = libc::c_uint;
pub const IMGTYPE_COLORALPHA: crate::tr_common_h::imgType_t = 0;
pub const IMGTYPE_NORMAL: crate::tr_common_h::imgType_t = 1;
pub const IMGTYPE_NORMALHEIGHT: crate::tr_common_h::imgType_t = 2;
pub const IMGTYPE_DELUXE: crate::tr_common_h::imgType_t = 3;
pub type imgFlags_t = libc::c_uint;
pub const IMGFLAG_NONE: crate::tr_common_h::imgFlags_t = 0;
pub const IMGFLAG_MIPMAP: crate::tr_common_h::imgFlags_t = 1;
pub const IMGFLAG_PICMIP: crate::tr_common_h::imgFlags_t = 2;
pub const IMGFLAG_CUBEMAP: crate::tr_common_h::imgFlags_t = 4;
pub const IMGFLAG_NO_COMPRESSION: crate::tr_common_h::imgFlags_t = 16;
pub const IMGFLAG_NOLIGHTSCALE: crate::tr_common_h::imgFlags_t = 32;
pub const IMGFLAG_CLAMPTOEDGE: crate::tr_common_h::imgFlags_t = 64;
pub const IMGFLAG_GENNORMALMAP: crate::tr_common_h::imgFlags_t = 128;
pub type image_t = crate::tr_common_h::image_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_s {
    pub imgName: [libc::c_char; 64],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub uploadWidth: libc::c_int,
    pub uploadHeight: libc::c_int,
    pub texnum: crate::stdlib::GLuint,
    pub frameUsed: libc::c_int,
    pub internalFormat: libc::c_int,
    pub TMU: libc::c_int,
    pub type_0: crate::tr_common_h::imgType_t,
    pub flags: crate::tr_common_h::imgFlags_t,
    pub next: *mut crate::tr_common_h::image_s,
}
