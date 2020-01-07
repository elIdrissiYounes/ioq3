pub type imgType_t = u32;
pub type imgFlags_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_s {
    pub imgName: [i8; 64],
    pub width: i32,
    pub height: i32,
    pub uploadWidth: i32,
    pub uploadHeight: i32,
    pub texnum: crate::stdlib::GLuint,
    pub frameUsed: i32,
    pub internalFormat: i32,
    pub TMU: i32,
    pub type_0: crate::tr_common_h::imgType_t,
    pub flags: crate::tr_common_h::imgFlags_t,
    pub next: *mut crate::tr_common_h::image_s,
}
pub type image_t = crate::tr_common_h::image_s;
pub const IMGTYPE_DELUXE: crate::tr_common_h::imgType_t = 3;
pub const IMGTYPE_NORMALHEIGHT: crate::tr_common_h::imgType_t = 2;
pub const IMGTYPE_NORMAL: crate::tr_common_h::imgType_t = 1;
pub const IMGTYPE_COLORALPHA: crate::tr_common_h::imgType_t = 0;
pub const IMGFLAG_GENNORMALMAP: crate::tr_common_h::imgFlags_t = 128;
pub const IMGFLAG_CLAMPTOEDGE: crate::tr_common_h::imgFlags_t = 64;
pub const IMGFLAG_NOLIGHTSCALE: crate::tr_common_h::imgFlags_t = 32;
pub const IMGFLAG_NO_COMPRESSION: crate::tr_common_h::imgFlags_t = 16;
pub const IMGFLAG_CUBEMAP: crate::tr_common_h::imgFlags_t = 4;
pub const IMGFLAG_PICMIP: crate::tr_common_h::imgFlags_t = 2;
pub const IMGFLAG_MIPMAP: crate::tr_common_h::imgFlags_t = 1;
pub const IMGFLAG_NONE: crate::tr_common_h::imgFlags_t = 0;
