use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint32_t = libc::c_uint;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
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
    pub type byte = libc::c_uchar;
    pub type fileHandle_t = libc::c_int;
    use super::{libc};
    extern "C" {
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
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
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/md5.c"]
pub mod md5_c {
    pub type MD5_CTX = MD5Context;
    /*
 * This code implements the MD5 message-digest algorithm.
 * The algorithm is due to Ron Rivest.  This code was
 * written by Colin Plumb in 1993, no copyright is claimed.
 * This code is in the public domain; do with it what you wish.
 *
 * Equivalent code is available from RSA Data Security, Inc.
 * This code has been tested against that, and is equivalent,
 * except that you don't need to include two pages of legalese
 * with every copy.
 *
 * To compute the message digest of a chunk of bytes, declare an
 * MD5Context structure, pass it to MD5Init, call MD5Update as
 * needed on buffers full of bytes, and then call MD5Final, which
 * will fill a supplied 16-byte array with the digest.
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct MD5Context {
        pub buf: [uint32_t; 4],
        pub bits: [uint32_t; 2],
        pub in_0: [libc::c_uchar; 64],
    }
    use super::stdint_uintn_h::{uint32_t};
    use super::{libc};
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
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    use super::q_shared_h::{fileHandle_t};
    extern "C" {
        #[no_mangle]
        pub fn FS_SV_FOpenFileRead(filename: *const libc::c_char,
                                   fp: *mut fileHandle_t) -> libc::c_long;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
    }
}
use self::types_h::{__uint32_t};
use self::stdint_uintn_h::{uint32_t};
use self::q_shared_h::{byte, fileHandle_t, Q_strncpyz, Q_strcat, va};
use self::md5_c::{MD5_CTX, MD5Context};
use self::string_h::{memcpy, memset};
use self::qcommon_h::{FS_SV_FOpenFileRead, FS_Read, FS_FCloseFile};
#[no_mangle]
pub unsafe extern "C" fn Com_MD5File(mut fn_0: *const libc::c_char,
                                     mut length: libc::c_int,
                                     mut prefix: *const libc::c_char,
                                     mut prefix_len: libc::c_int)
 -> *mut libc::c_char {
    static mut final_0: [libc::c_char; 33] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut digest: [libc::c_uchar; 16] =
        *::std::mem::transmute::<&[u8; 16],
                                 &mut [libc::c_uchar; 16]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut f: fileHandle_t = 0;
    let mut md5: MD5_CTX =
        MD5Context{buf: [0; 4], bits: [0; 2], in_0: [0; 64],};
    let mut buffer: [byte; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut filelen: libc::c_int = 0i32;
    let mut r: libc::c_int = 0i32;
    let mut total: libc::c_int = 0i32;
    Q_strncpyz(final_0.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char,
               ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as
                   libc::c_int);
    filelen = FS_SV_FOpenFileRead(fn_0, &mut f) as libc::c_int;
    if 0 == f { return final_0.as_mut_ptr() }
    if filelen < 1i32 { FS_FCloseFile(f); return final_0.as_mut_ptr() }
    if filelen < length || 0 == length { length = filelen }
    MD5Init(&mut md5);
    if 0 != prefix_len && 0 != *prefix as libc::c_int {
        MD5Update(&mut md5, prefix as *mut libc::c_uchar,
                  prefix_len as libc::c_uint);
    }
    loop  {
        r =
            FS_Read(buffer.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[byte; 2048]>() as libc::c_ulong as
                        libc::c_int, f);
        if r < 1i32 { break ; }
        if r + total > length { r = length - total }
        total += r;
        MD5Update(&mut md5, buffer.as_mut_ptr(), r as libc::c_uint);
        if (r as libc::c_ulong) <
               ::std::mem::size_of::<[byte; 2048]>() as libc::c_ulong ||
               total >= length {
            break ;
        }
    }
    FS_FCloseFile(f);
    MD5Final(&mut md5, digest.as_mut_ptr());
    final_0[0usize] = '\u{0}' as i32 as libc::c_char;
    i = 0i32;
    while i < 16i32 {
        Q_strcat(final_0.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                     as libc::c_int,
                 va(b"%02X\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                    digest[i as usize] as libc::c_int));
        i += 1
    }
    return final_0.as_mut_ptr();
}
/*
 * Final wrapup - pad to 64-byte boundary with the bit pattern 
 * 1 0* (64-bit count of bits processed, MSB-first)
 */
unsafe extern "C" fn MD5Final(mut ctx: *mut MD5Context,
                              mut digest: *mut libc::c_uchar) {
    let mut count: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    count = (*ctx).bits[0usize] >> 3i32 & 0x3fi32 as libc::c_uint;
    p = (*ctx).in_0.as_mut_ptr().offset(count as isize);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = 0x80i32 as libc::c_uchar;
    count = ((64i32 - 1i32) as libc::c_uint).wrapping_sub(count);
    if count < 8i32 as libc::c_uint {
        memset(p as *mut libc::c_void, 0i32, count as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *mut uint32_t as
                         *const uint32_t);
        memset((*ctx).in_0.as_mut_ptr() as *mut libc::c_void, 0i32,
               56i32 as libc::c_ulong);
    } else {
        memset(p as *mut libc::c_void, 0i32,
               count.wrapping_sub(8i32 as libc::c_uint) as libc::c_ulong);
    }
    *((*ctx).in_0.as_mut_ptr() as *mut uint32_t).offset(14isize) =
        (*ctx).bits[0usize];
    *((*ctx).in_0.as_mut_ptr() as *mut uint32_t).offset(15isize) =
        (*ctx).bits[1usize];
    MD5Transform((*ctx).buf.as_mut_ptr(),
                 (*ctx).in_0.as_mut_ptr() as *mut uint32_t as
                     *const uint32_t);
    if !digest.is_null() {
        memcpy(digest as *mut libc::c_void,
               (*ctx).buf.as_mut_ptr() as *const libc::c_void,
               16i32 as libc::c_ulong);
    }
    memset(ctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<MD5Context>() as libc::c_ulong);
}
/* The four core functions - F1 is optimized somewhat */
/* #define F1(x, y, z) (x & y | ~x & z) */
/* This is the central step in the MD5 algorithm. */
/*
 * The core of the MD5 algorithm, this alters an existing MD5 hash to
 * reflect the addition of 16 longwords of new data.  MD5Update blocks
 * the data and converts bytes into longwords for this routine.
 */
unsafe extern "C" fn MD5Transform(mut buf: *mut uint32_t,
                                  mut in_0: *const uint32_t) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    a = *buf.offset(0isize);
    b = *buf.offset(1isize);
    c = *buf.offset(2isize);
    d = *buf.offset(3isize);
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(0isize)).wrapping_add(0xd76aa478u32))
            as uint32_t as uint32_t;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(1isize)).wrapping_add(0xe8c7b756u32))
            as uint32_t as uint32_t;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(2isize)).wrapping_add(0x242070dbi32
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(3isize)).wrapping_add(0xc1bdceeeu32))
            as uint32_t as uint32_t;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(4isize)).wrapping_add(0xf57c0fafu32))
            as uint32_t as uint32_t;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(5isize)).wrapping_add(0x4787c62ai32
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(6isize)).wrapping_add(0xa8304613u32))
            as uint32_t as uint32_t;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(7isize)).wrapping_add(0xfd469501u32))
            as uint32_t as uint32_t;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(8isize)).wrapping_add(0x698098d8i32
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(9isize)).wrapping_add(0x8b44f7afu32))
            as uint32_t as uint32_t;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(10isize)).wrapping_add(0xffff5bb1u32))
            as uint32_t as uint32_t;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(11isize)).wrapping_add(0x895cd7beu32))
            as uint32_t as uint32_t;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((d ^
                                             b &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(12isize)).wrapping_add(0x6b901122i32
                                                                                                               as
                                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 7i32 | a >> 32i32 - 7i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((c ^
                                             a &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(13isize)).wrapping_add(0xfd987193u32))
            as uint32_t as uint32_t;
    d = d << 12i32 | d >> 32i32 - 12i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((b ^
                                             d &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(14isize)).wrapping_add(0xa679438eu32))
            as uint32_t as uint32_t;
    c = c << 17i32 | c >> 32i32 - 17i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((a ^
                                             c &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(15isize)).wrapping_add(0x49b40821i32
                                                                                                               as
                                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 22i32 | b >> 32i32 - 22i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(1isize)).wrapping_add(0xf61e2562u32))
            as uint32_t as uint32_t;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(6isize)).wrapping_add(0xc040b340u32))
            as uint32_t as uint32_t;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(11isize)).wrapping_add(0x265e5a51i32
                                                                                                               as
                                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(0isize)).wrapping_add(0xe9b6c7aau32))
            as uint32_t as uint32_t;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(5isize)).wrapping_add(0xd62f105du32))
            as uint32_t as uint32_t;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(10isize)).wrapping_add(0x2441453i32
                                                                                                               as
                                                                                                               libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(15isize)).wrapping_add(0xd8a1e681u32))
            as uint32_t as uint32_t;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(4isize)).wrapping_add(0xe7d3fbc8u32))
            as uint32_t as uint32_t;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(9isize)).wrapping_add(0x21e1cde6i32
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(14isize)).wrapping_add(0xc33707d6u32))
            as uint32_t as uint32_t;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(3isize)).wrapping_add(0xf4d50d87u32))
            as uint32_t as uint32_t;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(8isize)).wrapping_add(0x455a14edi32
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             d &
                                                 (b ^
                                                      c)).wrapping_add(*in_0.offset(13isize)).wrapping_add(0xa9e3e905u32))
            as uint32_t as uint32_t;
    a = a << 5i32 | a >> 32i32 - 5i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             c &
                                                 (a ^
                                                      b)).wrapping_add(*in_0.offset(2isize)).wrapping_add(0xfcefa3f8u32))
            as uint32_t as uint32_t;
    d = d << 9i32 | d >> 32i32 - 9i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             b &
                                                 (d ^
                                                      a)).wrapping_add(*in_0.offset(7isize)).wrapping_add(0x676f02d9i32
                                                                                                              as
                                                                                                              libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 14i32 | c >> 32i32 - 14i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             a &
                                                 (c ^
                                                      d)).wrapping_add(*in_0.offset(12isize)).wrapping_add(0x8d2a4c8au32))
            as uint32_t as uint32_t;
    b = b << 20i32 | b >> 32i32 - 20i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(5isize)).wrapping_add(0xfffa3942u32))
            as uint32_t as uint32_t;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(8isize)).wrapping_add(0x8771f681u32))
            as uint32_t as uint32_t;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(11isize)).wrapping_add(0x6d9d6122i32
                                                                                                     as
                                                                                                     libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(14isize)).wrapping_add(0xfde5380cu32))
            as uint32_t as uint32_t;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(1isize)).wrapping_add(0xa4beea44u32))
            as uint32_t as uint32_t;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(4isize)).wrapping_add(0x4bdecfa9i32
                                                                                                    as
                                                                                                    libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(7isize)).wrapping_add(0xf6bb4b60u32))
            as uint32_t as uint32_t;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(10isize)).wrapping_add(0xbebfbc70u32))
            as uint32_t as uint32_t;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(13isize)).wrapping_add(0x289b7ec6i32
                                                                                                     as
                                                                                                     libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(0isize)).wrapping_add(0xeaa127fau32))
            as uint32_t as uint32_t;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(3isize)).wrapping_add(0xd4ef3085u32))
            as uint32_t as uint32_t;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(6isize)).wrapping_add(0x4881d05i32
                                                                                                    as
                                                                                                    libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(9isize)).wrapping_add(0xd9d4d039u32))
            as uint32_t as uint32_t;
    a = a << 4i32 | a >> 32i32 - 4i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(12isize)).wrapping_add(0xe6db99e5u32))
            as uint32_t as uint32_t;
    d = d << 11i32 | d >> 32i32 - 11i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(15isize)).wrapping_add(0x1fa27cf8i32
                                                                                                     as
                                                                                                     libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 16i32 | c >> 32i32 - 16i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(2isize)).wrapping_add(0xc4ac5665u32))
            as uint32_t as uint32_t;
    b = b << 23i32 | b >> 32i32 - 23i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(0isize)).wrapping_add(0xf4292244u32))
            as uint32_t as uint32_t;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(7isize)).wrapping_add(0x432aff97i32
                                                                                                           as
                                                                                                           libc::c_uint))
            as uint32_t as uint32_t;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(14isize)).wrapping_add(0xab9423a7u32))
            as uint32_t as uint32_t;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(5isize)).wrapping_add(0xfc93a039u32))
            as uint32_t as uint32_t;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(12isize)).wrapping_add(0x655b59c3i32
                                                                                                            as
                                                                                                            libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(3isize)).wrapping_add(0x8f0ccc92u32))
            as uint32_t as uint32_t;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(10isize)).wrapping_add(0xffeff47du32))
            as uint32_t as uint32_t;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(1isize)).wrapping_add(0x85845dd1u32))
            as uint32_t as uint32_t;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(8isize)).wrapping_add(0x6fa87e4fi32
                                                                                                           as
                                                                                                           libc::c_uint))
            as uint32_t as uint32_t;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(15isize)).wrapping_add(0xfe2ce6e0u32))
            as uint32_t as uint32_t;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(6isize)).wrapping_add(0xa3014314u32))
            as uint32_t as uint32_t;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(13isize)).wrapping_add(0x4e0811a1i32
                                                                                                            as
                                                                                                            libc::c_uint))
            as uint32_t as uint32_t;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(4isize)).wrapping_add(0xf7537e82u32))
            as uint32_t as uint32_t;
    a = a << 6i32 | a >> 32i32 - 6i32;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(11isize)).wrapping_add(0xbd3af235u32))
            as uint32_t as uint32_t;
    d = d << 10i32 | d >> 32i32 - 10i32;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(2isize)).wrapping_add(0x2ad7d2bbi32
                                                                                                           as
                                                                                                           libc::c_uint))
            as uint32_t as uint32_t;
    c = c << 15i32 | c >> 32i32 - 15i32;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(9isize)).wrapping_add(0xeb86d391u32))
            as uint32_t as uint32_t;
    b = b << 21i32 | b >> 32i32 - 21i32;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh1 = *buf.offset(0isize);
    *fresh1 =
        (*fresh1 as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh2 = *buf.offset(1isize);
    *fresh2 =
        (*fresh2 as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh3 = *buf.offset(2isize);
    *fresh3 =
        (*fresh3 as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh4 = *buf.offset(3isize);
    *fresh4 =
        (*fresh4 as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
}
/*
 * Update context to reflect the concatenation of another buffer full
 * of bytes.
 */
unsafe extern "C" fn MD5Update(mut ctx: *mut MD5Context,
                               mut buf: *const libc::c_uchar,
                               mut len: libc::c_uint) {
    let mut t: uint32_t = 0;
    t = (*ctx).bits[0usize];
    (*ctx).bits[0usize] = t.wrapping_add(len << 3i32);
    if (*ctx).bits[0usize] < t {
        (*ctx).bits[1usize] = (*ctx).bits[1usize].wrapping_add(1)
    }
    (*ctx).bits[1usize] =
        ((*ctx).bits[1usize] as libc::c_uint).wrapping_add(len >> 29i32) as
            uint32_t as uint32_t;
    t = t >> 3i32 & 0x3fi32 as libc::c_uint;
    if 0 != t {
        let mut p: *mut libc::c_uchar =
            (*ctx).in_0.as_mut_ptr().offset(t as isize);
        t = (64i32 as libc::c_uint).wrapping_sub(t);
        if len < t {
            memcpy(p as *mut libc::c_void, buf as *const libc::c_void,
                   len as libc::c_ulong);
            return
        }
        memcpy(p as *mut libc::c_void, buf as *const libc::c_void,
               t as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *mut uint32_t as
                         *const uint32_t);
        buf = buf.offset(t as isize);
        len = len.wrapping_sub(t)
    }
    while len >= 64i32 as libc::c_uint {
        memcpy((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
               buf as *const libc::c_void, 64i32 as libc::c_ulong);
        MD5Transform((*ctx).buf.as_mut_ptr(),
                     (*ctx).in_0.as_mut_ptr() as *mut uint32_t as
                         *const uint32_t);
        buf = buf.offset(64isize);
        len = len.wrapping_sub(64i32 as libc::c_uint)
    }
    memcpy((*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
           buf as *const libc::c_void, len as libc::c_ulong);
}
/* Nothing */
// Q3_BIG_ENDIAN
/*
 * Start MD5 accumulation.  Set bit count to 0 and buffer to mysterious
 * initialization constants.
 */
unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context) {
    (*ctx).buf[0usize] = 0x67452301i32 as uint32_t;
    (*ctx).buf[1usize] = 0xefcdab89u32;
    (*ctx).buf[2usize] = 0x98badcfeu32;
    (*ctx).buf[3usize] = 0x10325476i32 as uint32_t;
    (*ctx).bits[0usize] = 0i32 as uint32_t;
    (*ctx).bits[1usize] = 0i32 as uint32_t;
}