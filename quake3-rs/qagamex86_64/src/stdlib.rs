extern "C" {
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const u16;

    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const crate::stdlib::__int32_t;

    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const crate::stdlib::__int32_t;
    #[no_mangle]
    pub fn cos(_: f64) -> f64;

    #[no_mangle]
    pub fn sin(_: f64) -> f64;

    #[no_mangle]
    pub fn ceil(_: f64) -> f64;

    #[no_mangle]
    pub fn atan2(_: f64, _: f64) -> f64;

    #[no_mangle]
    pub fn floor(_: f64) -> f64;

    #[no_mangle]
    pub fn acos(_: f64) -> f64;

    #[no_mangle]
    pub fn sqrt(_: f64) -> f64;

    #[no_mangle]
    pub fn fabs(_: f64) -> f64;
    #[no_mangle]
    pub fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;

    #[no_mangle]
    pub fn vsnprintf(_: *mut i8, _: usize, _: *const i8, _: ::std::ffi::VaList) -> i32;
    #[no_mangle]
    pub fn exit(_: i32) -> !;

    #[no_mangle]
    pub fn srand(__seed: u32);

    #[no_mangle]
    pub fn qsort(
        __base: *mut libc::c_void,
        __nmemb: crate::stddef_h::size_t,
        __size: crate::stddef_h::size_t,
        __compar: crate::stdlib::__compar_fn_t,
    );

    #[no_mangle]
    pub fn abs(_: i32) -> i32;

    #[no_mangle]
    pub fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> isize;

    #[no_mangle]
    pub fn rand() -> i32;

    #[no_mangle]
    pub fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    #[no_mangle]
    pub fn strrchr(_: *const i8, _: i32) -> *mut i8;

    #[no_mangle]
    pub fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;

    #[no_mangle]
    pub fn strncpy(_: *mut i8, _: *const i8, _: usize) -> *mut i8;

    #[no_mangle]
    pub fn strlen(_: *const i8) -> usize;

    #[no_mangle]
    pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: i32, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn strcat(_: *mut i8, _: *const i8) -> *mut i8;

    #[no_mangle]
    pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;

    #[no_mangle]
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;

    #[no_mangle]
    pub fn strstr(_: *const i8, _: *const i8) -> *mut i8;

    #[no_mangle]
    pub fn strchr(_: *const i8, _: i32) -> *mut i8;
}
// =============== BEGIN ctype_h ================
pub const _ISalnum: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const _ISpunct: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const _IScntrl: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const _ISblank: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const _ISgraph: crate::bg_public_h::C2RustUnnamed_0 = 32768;
pub const _ISprint: crate::bg_public_h::C2RustUnnamed_0 = 16384;
pub const _ISspace: crate::bg_public_h::C2RustUnnamed_0 = 8192;
pub const _ISxdigit: crate::bg_public_h::C2RustUnnamed_0 = 4096;
pub const _ISdigit: crate::bg_public_h::C2RustUnnamed_0 = 2048;
pub const _ISalpha: crate::bg_public_h::C2RustUnnamed_0 = 1024;
pub const _ISlower: crate::bg_public_h::C2RustUnnamed_0 = 512;
pub const _ISupper: crate::bg_public_h::C2RustUnnamed_0 = 256;
// ================ END ctype_h ================
// =============== BEGIN stdint_h ================
pub type intptr_t = isize;
// ================ END stdint_h ================
// =============== BEGIN stdlib_h ================
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;
// ================ END stdlib_h ================
// =============== BEGIN types_h ================
pub type __int32_t = i32;
