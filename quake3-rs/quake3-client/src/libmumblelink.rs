use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __int32_t = libc::c_int;
    pub type __uint32_t = libc::c_uint;
    pub type __uid_t = libc::c_uint;
    pub type __mode_t = libc::c_uint;
    pub type __off_t = libc::c_long;
    pub type __time_t = libc::c_long;
    pub type __suseconds_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    pub type wchar_t = libc::c_int;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/sys/mman.h"]
pub mod mman_h {
    pub type mode_t = __mode_t;
    use super::types_h::{__mode_t, __off_t};
    use super::{libc};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn mmap(__addr: *mut libc::c_void, __len: size_t,
                    __prot: libc::c_int, __flags: libc::c_int,
                    __fd: libc::c_int, __offset: __off_t)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn munmap(__addr: *mut libc::c_void, __len: size_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn shm_open(__name: *const libc::c_char, __oflag: libc::c_int,
                        __mode: mode_t) -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int32_t = __int32_t;
    use super::types_h::{__int32_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_timeval.h"]
pub mod struct_timeval_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/sys/time.h"]
pub mod time_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct timezone {
        pub tz_minuteswest: libc::c_int,
        pub tz_dsttime: libc::c_int,
    }
    pub type __timezone_ptr_t = *mut timezone;
    use super::{libc};
    use super::struct_timeval_h::{timeval};
    extern "C" {
        #[no_mangle]
        pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
         -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/client/libmumblelink.c"]
pub mod libmumblelink_c {
    /* libmumblelink.c -- mumble link interface

  Copyright (C) 2008 Ludwig Nussel <ludwig.nussel@suse.de>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct LinkedMem {
        pub uiVersion: uint32_t,
        pub uiTick: uint32_t,
        pub fAvatarPosition: [libc::c_float; 3],
        pub fAvatarFront: [libc::c_float; 3],
        pub fAvatarTop: [libc::c_float; 3],
        pub name: [wchar_t; 256],
        pub fCameraPosition: [libc::c_float; 3],
        pub fCameraFront: [libc::c_float; 3],
        pub fCameraTop: [libc::c_float; 3],
        pub identity: [wchar_t; 256],
        pub context_len: uint32_t,
        pub context: [libc::c_uchar; 256],
        pub description: [wchar_t; 2048],
    }
    use super::stdint_uintn_h::{uint32_t};
    use super::{libc};
    use super::stddef_h::{wchar_t};
}
#[header_src = "/usr/include/unistd.h"]
pub mod unistd_h {
    use super::{libc};
    use super::types_h::{__uid_t};
    extern "C" {
        #[no_mangle]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn getuid() -> __uid_t;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::stddef_h::{size_t, wchar_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char,
                        __n: size_t) -> size_t;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, ...) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/client/libmumblelink.h"]
pub mod libmumblelink_h {
    use super::{libc};
    use super::stddef_h::{size_t};
}
use self::types_h::{__int32_t, __uint32_t, __uid_t, __mode_t, __off_t,
                    __time_t, __suseconds_t};
use self::stddef_h::{size_t, wchar_t};
use self::mman_h::{mode_t, mmap, munmap, shm_open};
use self::stdint_intn_h::{int32_t};
use self::struct_timeval_h::{timeval};
use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
use self::stdint_uintn_h::{uint32_t};
use self::libmumblelink_c::{LinkedMem};
use self::unistd_h::{close, getuid};
use self::stdlib_h::{mbstowcs};
use self::string_h::{memcpy, memset, strlen};
use self::stdio_h::{snprintf};
/* libmumblelink.h -- mumble link interface

  Copyright (C) 2008 Ludwig Nussel <ludwig.nussel@suse.de>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

*/
#[no_mangle]
pub unsafe extern "C" fn mumble_link(mut name: *const libc::c_char)
 -> libc::c_int {
    let mut file: [libc::c_char; 256] = [0; 256];
    let mut shmfd: libc::c_int = 0;
    if !lm.is_null() { return 0i32 }
    snprintf(file.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
             b"/MumbleLink.%d\x00" as *const u8 as *const libc::c_char,
             getuid());
    shmfd =
        shm_open(file.as_mut_ptr(), 0o2i32, (0o400i32 | 0o200i32) as mode_t);
    if shmfd < 0i32 { return -1i32 }
    lm =
        mmap(0 as *mut libc::c_void,
             ::std::mem::size_of::<LinkedMem>() as libc::c_ulong,
             0x1i32 | 0x2i32, 0x1i32, shmfd, 0i32 as __off_t) as
            *mut LinkedMem;
    if lm == -1i32 as *mut libc::c_void as *mut LinkedMem {
        lm = 0 as *mut LinkedMem;
        close(shmfd);
        return -1i32
    }
    close(shmfd);
    memset(lm as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<LinkedMem>() as libc::c_ulong);
    mbstowcs((*lm).name.as_mut_ptr(), name,
             (::std::mem::size_of::<[wchar_t; 256]>() as
                  libc::c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                  as libc::c_ulong));
    return 0i32;
}
static mut lm: *mut LinkedMem = 0 as *const LinkedMem as *mut LinkedMem;
#[no_mangle]
pub unsafe extern "C" fn mumble_islinked() -> libc::c_int {
    return (lm != 0 as *mut libc::c_void as *mut LinkedMem) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mumble_update_coordinates(mut fPosition:
                                                       *mut libc::c_float,
                                                   mut fFront:
                                                       *mut libc::c_float,
                                                   mut fTop:
                                                       *mut libc::c_float) {
    mumble_update_coordinates2(fPosition, fFront, fTop, fPosition, fFront,
                               fTop);
}
/* new for mumble 1.2: also set camera position */
#[no_mangle]
pub unsafe extern "C" fn mumble_update_coordinates2(mut fAvatarPosition:
                                                        *mut libc::c_float,
                                                    mut fAvatarFront:
                                                        *mut libc::c_float,
                                                    mut fAvatarTop:
                                                        *mut libc::c_float,
                                                    mut fCameraPosition:
                                                        *mut libc::c_float,
                                                    mut fCameraFront:
                                                        *mut libc::c_float,
                                                    mut fCameraTop:
                                                        *mut libc::c_float) {
    if lm.is_null() { return }
    memcpy((*lm).fAvatarPosition.as_mut_ptr() as *mut libc::c_void,
           fAvatarPosition as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong);
    memcpy((*lm).fAvatarFront.as_mut_ptr() as *mut libc::c_void,
           fAvatarFront as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong);
    memcpy((*lm).fAvatarTop.as_mut_ptr() as *mut libc::c_void,
           fAvatarTop as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong);
    memcpy((*lm).fCameraPosition.as_mut_ptr() as *mut libc::c_void,
           fCameraPosition as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong);
    memcpy((*lm).fCameraFront.as_mut_ptr() as *mut libc::c_void,
           fCameraFront as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong);
    memcpy((*lm).fCameraTop.as_mut_ptr() as *mut libc::c_void,
           fCameraTop as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong);
    (*lm).uiVersion = 2i32 as uint32_t;
    (*lm).uiTick = GetTickCount() as uint32_t;
}
unsafe extern "C" fn GetTickCount() -> int32_t {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut tv, 0 as *mut timezone);
    return (tv.tv_usec / 1000i32 as libc::c_long +
                tv.tv_sec * 1000i32 as libc::c_long) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mumble_set_description(mut description:
                                                    *const libc::c_char) {
    let mut len: size_t = 0;
    if lm.is_null() { return }
    len =
        if (::std::mem::size_of::<[wchar_t; 2048]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                as libc::c_ulong) <
               strlen(description).wrapping_add(1i32 as libc::c_ulong) {
            (::std::mem::size_of::<[wchar_t; 2048]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                 as libc::c_ulong)
        } else { strlen(description).wrapping_add(1i32 as libc::c_ulong) };
    mbstowcs((*lm).description.as_mut_ptr(), description, len);
}
#[no_mangle]
pub unsafe extern "C" fn mumble_set_context(mut context: *const libc::c_uchar,
                                            mut len: size_t) {
    if lm.is_null() { return }
    len =
        if (::std::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong) <
               len {
            ::std::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong
        } else { len };
    (*lm).context_len = len as uint32_t;
    memcpy((*lm).context.as_mut_ptr() as *mut libc::c_void,
           context as *const libc::c_void, len);
}
#[no_mangle]
pub unsafe extern "C" fn mumble_set_identity(mut identity:
                                                 *const libc::c_char) {
    let mut len: size_t = 0;
    if lm.is_null() { return }
    len =
        if (::std::mem::size_of::<[wchar_t; 256]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                as libc::c_ulong) <
               strlen(identity).wrapping_add(1i32 as libc::c_ulong) {
            (::std::mem::size_of::<[wchar_t; 256]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<wchar_t>()
                                                 as libc::c_ulong)
        } else { strlen(identity).wrapping_add(1i32 as libc::c_ulong) };
    mbstowcs((*lm).identity.as_mut_ptr(), identity, len);
}
#[no_mangle]
pub unsafe extern "C" fn mumble_unlink() {
    if lm.is_null() { return }
    munmap(lm as *mut libc::c_void,
           ::std::mem::size_of::<LinkedMem>() as libc::c_ulong);
    lm = 0 as *mut LinkedMem;
}