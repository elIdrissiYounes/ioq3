extern "C" {
    #[no_mangle]
    pub fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    pub fn SDL_Init(flags: crate::stdlib::Uint32) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_QuitSubSystem(flags: crate::stdlib::Uint32);

    #[no_mangle]
    pub fn SDL_WasInit(flags: crate::stdlib::Uint32) -> crate::stdlib::Uint32;
    #[no_mangle]
    pub fn SDL_calloc(
        nmemb: crate::stddef_h::size_t,
        size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn SDL_free(mem: *mut libc::c_void);
    pub type SDL_BlitMap;

    #[no_mangle]
    pub fn SDL_CreateRGBSurfaceFrom(
        pixels: *mut libc::c_void,
        width: libc::c_int,
        height: libc::c_int,
        depth: libc::c_int,
        pitch: libc::c_int,
        Rmask: crate::stdlib::Uint32,
        Gmask: crate::stdlib::Uint32,
        Bmask: crate::stdlib::Uint32,
        Amask: crate::stdlib::Uint32,
    ) -> *mut crate::stdlib::SDL_Surface;

    #[no_mangle]
    pub fn SDL_FreeSurface(surface: *mut crate::stdlib::SDL_Surface);
    pub type SDL_Window;

    #[no_mangle]
    pub fn SDL_GetCurrentVideoDriver() -> *const libc::c_char;

    #[no_mangle]
    pub fn SDL_GetNumDisplayModes(displayIndex: libc::c_int) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GetDisplayMode(
        displayIndex: libc::c_int,
        modeIndex: libc::c_int,
        mode: *mut crate::stdlib::SDL_DisplayMode,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GetDesktopDisplayMode(
        displayIndex: libc::c_int,
        mode: *mut crate::stdlib::SDL_DisplayMode,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GetWindowDisplayIndex(window: *mut crate::stdlib::SDL_Window) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_SetWindowDisplayMode(
        window: *mut crate::stdlib::SDL_Window,
        mode: *const crate::stdlib::SDL_DisplayMode,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GetWindowDisplayMode(
        window: *mut crate::stdlib::SDL_Window,
        mode: *mut crate::stdlib::SDL_DisplayMode,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_CreateWindow(
        title: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        flags: crate::stdlib::Uint32,
    ) -> *mut crate::stdlib::SDL_Window;

    #[no_mangle]
    pub fn SDL_GetWindowFlags(window: *mut crate::stdlib::SDL_Window) -> crate::stdlib::Uint32;

    #[no_mangle]
    pub fn SDL_SetWindowIcon(
        window: *mut crate::stdlib::SDL_Window,
        icon: *mut crate::stdlib::SDL_Surface,
    );

    #[no_mangle]
    pub fn SDL_GetWindowPosition(
        window: *mut crate::stdlib::SDL_Window,
        x: *mut libc::c_int,
        y: *mut libc::c_int,
    );

    #[no_mangle]
    pub fn SDL_MinimizeWindow(window: *mut crate::stdlib::SDL_Window);

    #[no_mangle]
    pub fn SDL_SetWindowFullscreen(
        window: *mut crate::stdlib::SDL_Window,
        flags: crate::stdlib::Uint32,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_SetWindowBrightness(
        window: *mut crate::stdlib::SDL_Window,
        brightness: libc::c_float,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_SetWindowGammaRamp(
        window: *mut crate::stdlib::SDL_Window,
        red: *const crate::stdlib::Uint16,
        green: *const crate::stdlib::Uint16,
        blue: *const crate::stdlib::Uint16,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_DestroyWindow(window: *mut crate::stdlib::SDL_Window);

    #[no_mangle]
    pub fn SDL_GL_GetProcAddress(proc_0: *const libc::c_char) -> *mut libc::c_void;

    #[no_mangle]
    pub fn SDL_GL_ExtensionSupported(extension: *const libc::c_char) -> crate::stdlib::SDL_bool;

    #[no_mangle]
    pub fn SDL_GL_SetAttribute(attr: crate::stdlib::SDL_GLattr, value: libc::c_int) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GL_GetAttribute(
        attr: crate::stdlib::SDL_GLattr,
        value: *mut libc::c_int,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GL_CreateContext(
        window: *mut crate::stdlib::SDL_Window,
    ) -> crate::stdlib::SDL_GLContext;

    #[no_mangle]
    pub fn SDL_GL_SetSwapInterval(interval: libc::c_int) -> libc::c_int;

    #[no_mangle]
    pub fn SDL_GL_SwapWindow(window: *mut crate::stdlib::SDL_Window);

    #[no_mangle]
    pub fn SDL_GL_DeleteContext(context: crate::stdlib::SDL_GLContext);
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;

    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const crate::stdlib::__int32_t;

    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const crate::stdlib::__int32_t;
    #[no_mangle]
    pub fn _setjmp(_: *mut crate::stdlib::__jmp_buf_tag) -> libc::c_int;

    #[no_mangle]
    pub fn longjmp(_: *mut crate::stdlib::__jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    pub fn acos(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn cos(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn sin(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn tan(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn sqrt(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn ceil(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn fabs(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub static mut stderr: *mut crate::stdlib::FILE;

    #[no_mangle]
    pub fn fflush(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

    #[no_mangle]
    pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

    #[no_mangle]
    pub fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;

    #[no_mangle]
    pub fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut crate::stdlib::FILE,
    ) -> libc::c_ulong;

    #[no_mangle]
    pub fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut crate::stdlib::FILE,
    ) -> libc::c_ulong;

    #[no_mangle]
    pub fn ferror(__stream: *mut crate::stdlib::FILE) -> libc::c_int;
    #[no_mangle]
    pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn qsort(
        __base: *mut libc::c_void,
        __nmemb: crate::stddef_h::size_t,
        __size: crate::stddef_h::size_t,
        __compar: crate::stdlib::__compar_fn_t,
    );
    #[no_mangle]
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

    #[no_mangle]
    pub fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;

    #[no_mangle]
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;

    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    pub type _IO_marker;

    pub type _IO_codecvt;

    pub type _IO_wide_data;
}
// =============== BEGIN FILE_h ================
pub type FILE = crate::stdlib::_IO_FILE;
// ================ END FILE_h ================
// =============== BEGIN SDL_opengl_glext_h ================
pub type GLsizeiptr = crate::stddef_h::ptrdiff_t;
pub type GLintptr = crate::stddef_h::ptrdiff_t;
pub type GLchar = libc::c_char;
// ================ END SDL_opengl_glext_h ================
// =============== BEGIN SDL_opengl_h ================
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLbitfield = libc::c_uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = libc::c_uchar;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
// ================ END SDL_opengl_h ================
// =============== BEGIN SDL_pixels_h ================
pub const SDL_PIXELFORMAT_UNKNOWN: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const SDL_PIXELFORMAT_INDEX1LSB: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 286261504;
pub const SDL_PIXELFORMAT_INDEX1MSB: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 287310080;
pub const SDL_PIXELFORMAT_INDEX4LSB: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 303039488;
pub const SDL_PIXELFORMAT_INDEX4MSB: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 304088064;
pub const SDL_PIXELFORMAT_INDEX8: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 318769153;
pub const SDL_PIXELFORMAT_RGB332: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 336660481;
pub const SDL_PIXELFORMAT_RGB444: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 353504258;
pub const SDL_PIXELFORMAT_RGB555: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 353570562;
pub const SDL_PIXELFORMAT_BGR555: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 357764866;
pub const SDL_PIXELFORMAT_ARGB4444: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 355602434;
pub const SDL_PIXELFORMAT_RGBA4444: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 356651010;
pub const SDL_PIXELFORMAT_ABGR4444: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 359796738;
pub const SDL_PIXELFORMAT_BGRA4444: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 360845314;
pub const SDL_PIXELFORMAT_ARGB1555: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 355667970;
pub const SDL_PIXELFORMAT_RGBA5551: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 356782082;
pub const SDL_PIXELFORMAT_ABGR1555: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 359862274;
pub const SDL_PIXELFORMAT_BGRA5551: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 360976386;
pub const SDL_PIXELFORMAT_RGB565: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 353701890;
pub const SDL_PIXELFORMAT_BGR565: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 357896194;
pub const SDL_PIXELFORMAT_RGB24: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 386930691;
pub const SDL_PIXELFORMAT_BGR24: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 390076419;
pub const SDL_PIXELFORMAT_RGB888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 370546692;
pub const SDL_PIXELFORMAT_RGBX8888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 371595268;
pub const SDL_PIXELFORMAT_BGR888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 374740996;
pub const SDL_PIXELFORMAT_BGRX8888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 375789572;
pub const SDL_PIXELFORMAT_ARGB8888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 372645892;
pub const SDL_PIXELFORMAT_RGBA8888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 373694468;
pub const SDL_PIXELFORMAT_ABGR8888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 376840196;
pub const SDL_PIXELFORMAT_BGRA8888: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 377888772;
pub const SDL_PIXELFORMAT_ARGB2101010: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 372711428;
pub const SDL_PIXELFORMAT_RGBA32: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 376840196;
pub const SDL_PIXELFORMAT_ARGB32: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 377888772;
pub const SDL_PIXELFORMAT_BGRA32: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 372645892;
pub const SDL_PIXELFORMAT_ABGR32: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 373694468;
pub const SDL_PIXELFORMAT_YV12: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 842094169;
pub const SDL_PIXELFORMAT_IYUV: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1448433993;
pub const SDL_PIXELFORMAT_YUY2: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 844715353;
pub const SDL_PIXELFORMAT_UYVY: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1498831189;
pub const SDL_PIXELFORMAT_YVYU: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1431918169;
pub const SDL_PIXELFORMAT_NV12: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 842094158;
pub const SDL_PIXELFORMAT_NV21: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 825382478;
pub const SDL_PIXELFORMAT_EXTERNAL_OES: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 542328143;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Color {
    pub r: crate::stdlib::Uint8,
    pub g: crate::stdlib::Uint8,
    pub b: crate::stdlib::Uint8,
    pub a: crate::stdlib::Uint8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Palette {
    pub ncolors: libc::c_int,
    pub colors: *mut crate::stdlib::SDL_Color,
    pub version: crate::stdlib::Uint32,
    pub refcount: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_PixelFormat {
    pub format: crate::stdlib::Uint32,
    pub palette: *mut crate::stdlib::SDL_Palette,
    pub BitsPerPixel: crate::stdlib::Uint8,
    pub BytesPerPixel: crate::stdlib::Uint8,
    pub padding: [crate::stdlib::Uint8; 2],
    pub Rmask: crate::stdlib::Uint32,
    pub Gmask: crate::stdlib::Uint32,
    pub Bmask: crate::stdlib::Uint32,
    pub Amask: crate::stdlib::Uint32,
    pub Rloss: crate::stdlib::Uint8,
    pub Gloss: crate::stdlib::Uint8,
    pub Bloss: crate::stdlib::Uint8,
    pub Aloss: crate::stdlib::Uint8,
    pub Rshift: crate::stdlib::Uint8,
    pub Gshift: crate::stdlib::Uint8,
    pub Bshift: crate::stdlib::Uint8,
    pub Ashift: crate::stdlib::Uint8,
    pub refcount: libc::c_int,
    pub next: *mut crate::stdlib::SDL_PixelFormat,
}
// ================ END SDL_pixels_h ================
// =============== BEGIN SDL_rect_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Rect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
// ================ END SDL_rect_h ================
// =============== BEGIN SDL_stdinc_h ================
pub type SDL_bool = libc::c_uint;
pub const SDL_FALSE: crate::stdlib::SDL_bool = 0;
pub const SDL_TRUE: crate::stdlib::SDL_bool = 1;
pub type Uint8 = crate::stdlib::uint8_t;
pub type Uint16 = crate::stdlib::uint16_t;
pub type Uint32 = crate::stdlib::uint32_t;
// ================ END SDL_stdinc_h ================
// =============== BEGIN SDL_surface_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Surface {
    pub flags: crate::stdlib::Uint32,
    pub format: *mut crate::stdlib::SDL_PixelFormat,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub pitch: libc::c_int,
    pub pixels: *mut libc::c_void,
    pub userdata: *mut libc::c_void,
    pub locked: libc::c_int,
    pub lock_data: *mut libc::c_void,
    pub clip_rect: crate::stdlib::SDL_Rect,
    pub map: *mut crate::stdlib::SDL_BlitMap,
    pub refcount: libc::c_int,
}
// ================ END SDL_surface_h ================
// =============== BEGIN SDL_video_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_DisplayMode {
    pub format: crate::stdlib::Uint32,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub refresh_rate: libc::c_int,
    pub driverdata: *mut libc::c_void,
}
pub const SDL_WINDOW_FULLSCREEN: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const SDL_WINDOW_OPENGL: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const SDL_WINDOW_SHOWN: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const SDL_WINDOW_HIDDEN: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
pub const SDL_WINDOW_BORDERLESS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 16;
pub const SDL_WINDOW_RESIZABLE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 32;
pub const SDL_WINDOW_MINIMIZED: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 64;
pub const SDL_WINDOW_MAXIMIZED: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 128;
pub const SDL_WINDOW_INPUT_GRABBED: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 256;
pub const SDL_WINDOW_INPUT_FOCUS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 512;
pub const SDL_WINDOW_MOUSE_FOCUS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1024;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4097;
pub const SDL_WINDOW_FOREIGN: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2048;
pub const SDL_WINDOW_ALLOW_HIGHDPI: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8192;
pub const SDL_WINDOW_MOUSE_CAPTURE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 16384;
pub const SDL_WINDOW_ALWAYS_ON_TOP: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 32768;
pub const SDL_WINDOW_SKIP_TASKBAR: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 65536;
pub const SDL_WINDOW_UTILITY: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 131072;
pub const SDL_WINDOW_TOOLTIP: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 262144;
pub const SDL_WINDOW_POPUP_MENU: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 524288;
pub const SDL_WINDOW_VULKAN: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 268435456;
pub type SDL_GLContext = *mut libc::c_void;
pub type SDL_GLattr = libc::c_uint;
pub const SDL_GL_RED_SIZE: crate::stdlib::SDL_GLattr = 0;
pub const SDL_GL_GREEN_SIZE: crate::stdlib::SDL_GLattr = 1;
pub const SDL_GL_BLUE_SIZE: crate::stdlib::SDL_GLattr = 2;
pub const SDL_GL_ALPHA_SIZE: crate::stdlib::SDL_GLattr = 3;
pub const SDL_GL_BUFFER_SIZE: crate::stdlib::SDL_GLattr = 4;
pub const SDL_GL_DOUBLEBUFFER: crate::stdlib::SDL_GLattr = 5;
pub const SDL_GL_DEPTH_SIZE: crate::stdlib::SDL_GLattr = 6;
pub const SDL_GL_STENCIL_SIZE: crate::stdlib::SDL_GLattr = 7;
pub const SDL_GL_ACCUM_RED_SIZE: crate::stdlib::SDL_GLattr = 8;
pub const SDL_GL_ACCUM_GREEN_SIZE: crate::stdlib::SDL_GLattr = 9;
pub const SDL_GL_ACCUM_BLUE_SIZE: crate::stdlib::SDL_GLattr = 10;
pub const SDL_GL_ACCUM_ALPHA_SIZE: crate::stdlib::SDL_GLattr = 11;
pub const SDL_GL_STEREO: crate::stdlib::SDL_GLattr = 12;
pub const SDL_GL_MULTISAMPLEBUFFERS: crate::stdlib::SDL_GLattr = 13;
pub const SDL_GL_MULTISAMPLESAMPLES: crate::stdlib::SDL_GLattr = 14;
pub const SDL_GL_ACCELERATED_VISUAL: crate::stdlib::SDL_GLattr = 15;
pub const SDL_GL_RETAINED_BACKING: crate::stdlib::SDL_GLattr = 16;
pub const SDL_GL_CONTEXT_MAJOR_VERSION: crate::stdlib::SDL_GLattr = 17;
pub const SDL_GL_CONTEXT_MINOR_VERSION: crate::stdlib::SDL_GLattr = 18;
pub const SDL_GL_CONTEXT_EGL: crate::stdlib::SDL_GLattr = 19;
pub const SDL_GL_CONTEXT_FLAGS: crate::stdlib::SDL_GLattr = 20;
pub const SDL_GL_CONTEXT_PROFILE_MASK: crate::stdlib::SDL_GLattr = 21;
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: crate::stdlib::SDL_GLattr = 22;
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: crate::stdlib::SDL_GLattr = 23;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: crate::stdlib::SDL_GLattr = 24;
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: crate::stdlib::SDL_GLattr = 25;
pub const SDL_GL_CONTEXT_NO_ERROR: crate::stdlib::SDL_GLattr = 26;
pub const SDL_GL_CONTEXT_PROFILE_CORE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const SDL_GL_CONTEXT_PROFILE_COMPATIBILITY: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const SDL_GL_CONTEXT_PROFILE_ES: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
// ================ END SDL_video_h ================
// =============== BEGIN __sigset_t_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
// ================ END __sigset_t_h ================
// =============== BEGIN ctype_h ================
pub const _ISupper: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 256;
pub const _ISlower: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 512;
pub const _ISalpha: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1024;
pub const _ISdigit: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2048;
pub const _ISxdigit: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4096;
pub const _ISspace: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8192;
pub const _ISprint: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 16384;
pub const _ISgraph: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 32768;
pub const _ISblank: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const _IScntrl: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const _ISpunct: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const _ISalnum: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
// ================ END ctype_h ================
// =============== BEGIN include_setjmp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: crate::stdlib::__jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: crate::stdlib::__sigset_t,
}
pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
// ================ END include_setjmp_h ================
// =============== BEGIN setjmp_h ================
pub type __jmp_buf = [libc::c_long; 8];
// ================ END setjmp_h ================
// =============== BEGIN stdint_h ================
pub type intptr_t = libc::c_long;
// ================ END stdint_h ================
// =============== BEGIN stdint_intn_h ================
pub type int16_t = crate::stdlib::__int16_t;
pub type int32_t = crate::stdlib::__int32_t;
pub type int64_t = crate::stdlib::__int64_t;
// ================ END stdint_intn_h ================
// =============== BEGIN stdint_uintn_h ================
pub type uint8_t = crate::stdlib::__uint8_t;
pub type uint16_t = crate::stdlib::__uint16_t;
pub type uint32_t = crate::stdlib::__uint32_t;
// ================ END stdint_uintn_h ================
// =============== BEGIN stdlib_h ================
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
// ================ END stdlib_h ================
// =============== BEGIN struct_FILE_h ================
pub type _IO_lock_t = ();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut crate::stdlib::_IO_marker,
    pub _chain: *mut crate::stdlib::_IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: crate::stdlib::__off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: crate::stdlib::__off64_t,
    pub _codecvt: *mut crate::stdlib::_IO_codecvt,
    pub _wide_data: *mut crate::stdlib::_IO_wide_data,
    pub _freeres_list: *mut crate::stdlib::_IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: crate::stddef_h::size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
// ================ END struct_FILE_h ================
// =============== BEGIN types_h ================
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
