#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint32_t = libc::c_uint;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t};
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
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
    use super::{libc};
}
#[header_src =
      "ioq3/code/qcommon/md4.c"]
pub mod md4_c {
    /*
	mdfour.c

	An implementation of MD4 designed for use in the samba SMB
	authentication protocol

	Copyright (C) 1997-1998  Andrew Tridgell

	This program is free software; you can redistribute it and/or
	modify it under the terms of the GNU General Public License
	as published by the Free Software Foundation; either version 2
	of the License, or (at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

	See the GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program; if not, write to:

		Free Software Foundation, Inc.
		59 Temple Place - Suite 330
		Boston, MA  02111-1307, USA

	$Id: mdfour.c,v 1.1 2002/08/23 22:03:27 abster Exp $
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct mdfour {
        pub A: uint32_t,
        pub B: uint32_t,
        pub C: uint32_t,
        pub D: uint32_t,
        pub totalN: uint32_t,
    }
    use super::stdint_uintn_h::{uint32_t};
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
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
}
use self::types_h::{__uint32_t};
use self::stdint_uintn_h::{uint32_t};
use self::q_shared_h::{byte};
use self::md4_c::{mdfour};
use self::string_h::{memcpy, memset};
#[no_mangle]
pub unsafe extern "C" fn Com_BlockChecksum(mut buffer: *const libc::c_void,
                                           mut length: libc::c_int)
 -> libc::c_uint {
    let mut digest: [libc::c_int; 4] = [0; 4];
    let mut val: libc::c_uint = 0;
    mdfour(digest.as_mut_ptr() as *mut byte, buffer as *mut byte, length);
    val =
        (digest[0usize] ^ digest[1usize] ^ digest[2usize] ^ digest[3usize]) as
            libc::c_uint;
    return val;
}
unsafe extern "C" fn mdfour(mut out: *mut byte, mut in_0: *mut byte,
                            mut n: libc::c_int) {
    let mut md: mdfour = mdfour{A: 0, B: 0, C: 0, D: 0, totalN: 0,};
    mdfour_begin(&mut md);
    mdfour_update(&mut md, in_0, n);
    mdfour_result(&mut md, out);
}
unsafe extern "C" fn mdfour_result(mut md: *mut mdfour, mut out: *mut byte) {
    copy4(out, (*md).A);
    copy4(out.offset(4isize), (*md).B);
    copy4(out.offset(8isize), (*md).C);
    copy4(out.offset(12isize), (*md).D);
}
unsafe extern "C" fn copy4(mut out: *mut byte, mut x: uint32_t) {
    *out.offset(0isize) = (x & 0xffi32 as libc::c_uint) as byte;
    *out.offset(1isize) = (x >> 8i32 & 0xffi32 as libc::c_uint) as byte;
    *out.offset(2isize) = (x >> 16i32 & 0xffi32 as libc::c_uint) as byte;
    *out.offset(3isize) = (x >> 24i32 & 0xffi32 as libc::c_uint) as byte;
}
unsafe extern "C" fn mdfour_update(mut md: *mut mdfour, mut in_0: *mut byte,
                                   mut n: libc::c_int) {
    let mut M: [uint32_t; 16] = [0; 16];
    m = md;
    if n == 0i32 { mdfour_tail(in_0, n); }
    while n >= 64i32 {
        copy64(M.as_mut_ptr(), in_0);
        mdfour64(M.as_mut_ptr());
        in_0 = in_0.offset(64isize);
        n -= 64i32;
        (*m).totalN =
            ((*m).totalN as libc::c_uint).wrapping_add(64i32 as libc::c_uint)
                as uint32_t as uint32_t
    }
    mdfour_tail(in_0, n);
}
unsafe extern "C" fn mdfour_tail(mut in_0: *mut byte, mut n: libc::c_int) {
    let mut buf: [byte; 128] = [0; 128];
    let mut M: [uint32_t; 16] = [0; 16];
    let mut b: uint32_t = 0;
    (*m).totalN =
        ((*m).totalN as libc::c_uint).wrapping_add(n as libc::c_uint) as
            uint32_t as uint32_t;
    b = (*m).totalN.wrapping_mul(8i32 as libc::c_uint);
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0i32,
           128i32 as libc::c_ulong);
    if 0 != n {
        memcpy(buf.as_mut_ptr() as *mut libc::c_void,
               in_0 as *const libc::c_void, n as libc::c_ulong);
    }
    buf[n as usize] = 0x80i32 as byte;
    if n <= 55i32 {
        copy4(buf.as_mut_ptr().offset(56isize), b);
        copy64(M.as_mut_ptr(), buf.as_mut_ptr());
        mdfour64(M.as_mut_ptr());
    } else {
        copy4(buf.as_mut_ptr().offset(120isize), b);
        copy64(M.as_mut_ptr(), buf.as_mut_ptr());
        mdfour64(M.as_mut_ptr());
        copy64(M.as_mut_ptr(), buf.as_mut_ptr().offset(64isize));
        mdfour64(M.as_mut_ptr());
    };
}
/* this applies md4 to 64 byte chunks */
unsafe extern "C" fn mdfour64(mut M: *mut uint32_t) {
    let mut j: libc::c_int = 0;
    let mut AA: uint32_t = 0;
    let mut BB: uint32_t = 0;
    let mut CC: uint32_t = 0;
    let mut DD: uint32_t = 0;
    let mut X: [uint32_t; 16] = [0; 16];
    let mut A: uint32_t = 0;
    let mut B: uint32_t = 0;
    let mut C: uint32_t = 0;
    let mut D: uint32_t = 0;
    j = 0i32;
    while j < 16i32 { X[j as usize] = *M.offset(j as isize); j += 1 }
    A = (*m).A;
    B = (*m).B;
    C = (*m).C;
    D = (*m).D;
    AA = A;
    BB = B;
    CC = C;
    DD = D;
    A =
        A.wrapping_add(B & C | !B & D).wrapping_add(X[0usize]) << 3i32 |
            A.wrapping_add(B & C | !B & D).wrapping_add(X[0usize]) >>
                32i32 - 3i32;
    D =
        D.wrapping_add(A & B | !A & C).wrapping_add(X[1usize]) << 7i32 |
            D.wrapping_add(A & B | !A & C).wrapping_add(X[1usize]) >>
                32i32 - 7i32;
    C =
        C.wrapping_add(D & A | !D & B).wrapping_add(X[2usize]) << 11i32 |
            C.wrapping_add(D & A | !D & B).wrapping_add(X[2usize]) >>
                32i32 - 11i32;
    B =
        B.wrapping_add(C & D | !C & A).wrapping_add(X[3usize]) << 19i32 |
            B.wrapping_add(C & D | !C & A).wrapping_add(X[3usize]) >>
                32i32 - 19i32;
    A =
        A.wrapping_add(B & C | !B & D).wrapping_add(X[4usize]) << 3i32 |
            A.wrapping_add(B & C | !B & D).wrapping_add(X[4usize]) >>
                32i32 - 3i32;
    D =
        D.wrapping_add(A & B | !A & C).wrapping_add(X[5usize]) << 7i32 |
            D.wrapping_add(A & B | !A & C).wrapping_add(X[5usize]) >>
                32i32 - 7i32;
    C =
        C.wrapping_add(D & A | !D & B).wrapping_add(X[6usize]) << 11i32 |
            C.wrapping_add(D & A | !D & B).wrapping_add(X[6usize]) >>
                32i32 - 11i32;
    B =
        B.wrapping_add(C & D | !C & A).wrapping_add(X[7usize]) << 19i32 |
            B.wrapping_add(C & D | !C & A).wrapping_add(X[7usize]) >>
                32i32 - 19i32;
    A =
        A.wrapping_add(B & C | !B & D).wrapping_add(X[8usize]) << 3i32 |
            A.wrapping_add(B & C | !B & D).wrapping_add(X[8usize]) >>
                32i32 - 3i32;
    D =
        D.wrapping_add(A & B | !A & C).wrapping_add(X[9usize]) << 7i32 |
            D.wrapping_add(A & B | !A & C).wrapping_add(X[9usize]) >>
                32i32 - 7i32;
    C =
        C.wrapping_add(D & A | !D & B).wrapping_add(X[10usize]) << 11i32 |
            C.wrapping_add(D & A | !D & B).wrapping_add(X[10usize]) >>
                32i32 - 11i32;
    B =
        B.wrapping_add(C & D | !C & A).wrapping_add(X[11usize]) << 19i32 |
            B.wrapping_add(C & D | !C & A).wrapping_add(X[11usize]) >>
                32i32 - 19i32;
    A =
        A.wrapping_add(B & C | !B & D).wrapping_add(X[12usize]) << 3i32 |
            A.wrapping_add(B & C | !B & D).wrapping_add(X[12usize]) >>
                32i32 - 3i32;
    D =
        D.wrapping_add(A & B | !A & C).wrapping_add(X[13usize]) << 7i32 |
            D.wrapping_add(A & B | !A & C).wrapping_add(X[13usize]) >>
                32i32 - 7i32;
    C =
        C.wrapping_add(D & A | !D & B).wrapping_add(X[14usize]) << 11i32 |
            C.wrapping_add(D & A | !D & B).wrapping_add(X[14usize]) >>
                32i32 - 11i32;
    B =
        B.wrapping_add(C & D | !C & A).wrapping_add(X[15usize]) << 19i32 |
            B.wrapping_add(C & D | !C & A).wrapping_add(X[15usize]) >>
                32i32 - 19i32;
    A =
        A.wrapping_add(B & C | B & D |
                           C &
                               D).wrapping_add(X[0usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 3i32 |
            A.wrapping_add(B & C | B & D |
                               C &
                                   D).wrapping_add(X[0usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A & B | A & C |
                           B &
                               C).wrapping_add(X[4usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 5i32 |
            D.wrapping_add(A & B | A & C |
                               B &
                                   C).wrapping_add(X[4usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 5i32;
    C =
        C.wrapping_add(D & A | D & B |
                           A &
                               B).wrapping_add(X[8usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 9i32 |
            C.wrapping_add(D & A | D & B |
                               A &
                                   B).wrapping_add(X[8usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 9i32;
    B =
        B.wrapping_add(C & D | C & A |
                           D &
                               A).wrapping_add(X[12usize]).wrapping_add(0x5a827999i32
                                                                            as
                                                                            libc::c_uint)
            << 13i32 |
            B.wrapping_add(C & D | C & A |
                               D &
                                   A).wrapping_add(X[12usize]).wrapping_add(0x5a827999i32
                                                                                as
                                                                                libc::c_uint)
                >> 32i32 - 13i32;
    A =
        A.wrapping_add(B & C | B & D |
                           C &
                               D).wrapping_add(X[1usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 3i32 |
            A.wrapping_add(B & C | B & D |
                               C &
                                   D).wrapping_add(X[1usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A & B | A & C |
                           B &
                               C).wrapping_add(X[5usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 5i32 |
            D.wrapping_add(A & B | A & C |
                               B &
                                   C).wrapping_add(X[5usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 5i32;
    C =
        C.wrapping_add(D & A | D & B |
                           A &
                               B).wrapping_add(X[9usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 9i32 |
            C.wrapping_add(D & A | D & B |
                               A &
                                   B).wrapping_add(X[9usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 9i32;
    B =
        B.wrapping_add(C & D | C & A |
                           D &
                               A).wrapping_add(X[13usize]).wrapping_add(0x5a827999i32
                                                                            as
                                                                            libc::c_uint)
            << 13i32 |
            B.wrapping_add(C & D | C & A |
                               D &
                                   A).wrapping_add(X[13usize]).wrapping_add(0x5a827999i32
                                                                                as
                                                                                libc::c_uint)
                >> 32i32 - 13i32;
    A =
        A.wrapping_add(B & C | B & D |
                           C &
                               D).wrapping_add(X[2usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 3i32 |
            A.wrapping_add(B & C | B & D |
                               C &
                                   D).wrapping_add(X[2usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A & B | A & C |
                           B &
                               C).wrapping_add(X[6usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 5i32 |
            D.wrapping_add(A & B | A & C |
                               B &
                                   C).wrapping_add(X[6usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 5i32;
    C =
        C.wrapping_add(D & A | D & B |
                           A &
                               B).wrapping_add(X[10usize]).wrapping_add(0x5a827999i32
                                                                            as
                                                                            libc::c_uint)
            << 9i32 |
            C.wrapping_add(D & A | D & B |
                               A &
                                   B).wrapping_add(X[10usize]).wrapping_add(0x5a827999i32
                                                                                as
                                                                                libc::c_uint)
                >> 32i32 - 9i32;
    B =
        B.wrapping_add(C & D | C & A |
                           D &
                               A).wrapping_add(X[14usize]).wrapping_add(0x5a827999i32
                                                                            as
                                                                            libc::c_uint)
            << 13i32 |
            B.wrapping_add(C & D | C & A |
                               D &
                                   A).wrapping_add(X[14usize]).wrapping_add(0x5a827999i32
                                                                                as
                                                                                libc::c_uint)
                >> 32i32 - 13i32;
    A =
        A.wrapping_add(B & C | B & D |
                           C &
                               D).wrapping_add(X[3usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 3i32 |
            A.wrapping_add(B & C | B & D |
                               C &
                                   D).wrapping_add(X[3usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A & B | A & C |
                           B &
                               C).wrapping_add(X[7usize]).wrapping_add(0x5a827999i32
                                                                           as
                                                                           libc::c_uint)
            << 5i32 |
            D.wrapping_add(A & B | A & C |
                               B &
                                   C).wrapping_add(X[7usize]).wrapping_add(0x5a827999i32
                                                                               as
                                                                               libc::c_uint)
                >> 32i32 - 5i32;
    C =
        C.wrapping_add(D & A | D & B |
                           A &
                               B).wrapping_add(X[11usize]).wrapping_add(0x5a827999i32
                                                                            as
                                                                            libc::c_uint)
            << 9i32 |
            C.wrapping_add(D & A | D & B |
                               A &
                                   B).wrapping_add(X[11usize]).wrapping_add(0x5a827999i32
                                                                                as
                                                                                libc::c_uint)
                >> 32i32 - 9i32;
    B =
        B.wrapping_add(C & D | C & A |
                           D &
                               A).wrapping_add(X[15usize]).wrapping_add(0x5a827999i32
                                                                            as
                                                                            libc::c_uint)
            << 13i32 |
            B.wrapping_add(C & D | C & A |
                               D &
                                   A).wrapping_add(X[15usize]).wrapping_add(0x5a827999i32
                                                                                as
                                                                                libc::c_uint)
                >> 32i32 - 13i32;
    A =
        A.wrapping_add(B ^ C ^
                           D).wrapping_add(X[0usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 3i32 |
            A.wrapping_add(B ^ C ^
                               D).wrapping_add(X[0usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A ^ B ^
                           C).wrapping_add(X[8usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 9i32 |
            D.wrapping_add(A ^ B ^
                               C).wrapping_add(X[8usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 9i32;
    C =
        C.wrapping_add(D ^ A ^
                           B).wrapping_add(X[4usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 11i32 |
            C.wrapping_add(D ^ A ^
                               B).wrapping_add(X[4usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 11i32;
    B =
        B.wrapping_add(C ^ D ^
                           A).wrapping_add(X[12usize]).wrapping_add(0x6ed9eba1i32
                                                                        as
                                                                        libc::c_uint)
            << 15i32 |
            B.wrapping_add(C ^ D ^
                               A).wrapping_add(X[12usize]).wrapping_add(0x6ed9eba1i32
                                                                            as
                                                                            libc::c_uint)
                >> 32i32 - 15i32;
    A =
        A.wrapping_add(B ^ C ^
                           D).wrapping_add(X[2usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 3i32 |
            A.wrapping_add(B ^ C ^
                               D).wrapping_add(X[2usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A ^ B ^
                           C).wrapping_add(X[10usize]).wrapping_add(0x6ed9eba1i32
                                                                        as
                                                                        libc::c_uint)
            << 9i32 |
            D.wrapping_add(A ^ B ^
                               C).wrapping_add(X[10usize]).wrapping_add(0x6ed9eba1i32
                                                                            as
                                                                            libc::c_uint)
                >> 32i32 - 9i32;
    C =
        C.wrapping_add(D ^ A ^
                           B).wrapping_add(X[6usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 11i32 |
            C.wrapping_add(D ^ A ^
                               B).wrapping_add(X[6usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 11i32;
    B =
        B.wrapping_add(C ^ D ^
                           A).wrapping_add(X[14usize]).wrapping_add(0x6ed9eba1i32
                                                                        as
                                                                        libc::c_uint)
            << 15i32 |
            B.wrapping_add(C ^ D ^
                               A).wrapping_add(X[14usize]).wrapping_add(0x6ed9eba1i32
                                                                            as
                                                                            libc::c_uint)
                >> 32i32 - 15i32;
    A =
        A.wrapping_add(B ^ C ^
                           D).wrapping_add(X[1usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 3i32 |
            A.wrapping_add(B ^ C ^
                               D).wrapping_add(X[1usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A ^ B ^
                           C).wrapping_add(X[9usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 9i32 |
            D.wrapping_add(A ^ B ^
                               C).wrapping_add(X[9usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 9i32;
    C =
        C.wrapping_add(D ^ A ^
                           B).wrapping_add(X[5usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 11i32 |
            C.wrapping_add(D ^ A ^
                               B).wrapping_add(X[5usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 11i32;
    B =
        B.wrapping_add(C ^ D ^
                           A).wrapping_add(X[13usize]).wrapping_add(0x6ed9eba1i32
                                                                        as
                                                                        libc::c_uint)
            << 15i32 |
            B.wrapping_add(C ^ D ^
                               A).wrapping_add(X[13usize]).wrapping_add(0x6ed9eba1i32
                                                                            as
                                                                            libc::c_uint)
                >> 32i32 - 15i32;
    A =
        A.wrapping_add(B ^ C ^
                           D).wrapping_add(X[3usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 3i32 |
            A.wrapping_add(B ^ C ^
                               D).wrapping_add(X[3usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 3i32;
    D =
        D.wrapping_add(A ^ B ^
                           C).wrapping_add(X[11usize]).wrapping_add(0x6ed9eba1i32
                                                                        as
                                                                        libc::c_uint)
            << 9i32 |
            D.wrapping_add(A ^ B ^
                               C).wrapping_add(X[11usize]).wrapping_add(0x6ed9eba1i32
                                                                            as
                                                                            libc::c_uint)
                >> 32i32 - 9i32;
    C =
        C.wrapping_add(D ^ A ^
                           B).wrapping_add(X[7usize]).wrapping_add(0x6ed9eba1i32
                                                                       as
                                                                       libc::c_uint)
            << 11i32 |
            C.wrapping_add(D ^ A ^
                               B).wrapping_add(X[7usize]).wrapping_add(0x6ed9eba1i32
                                                                           as
                                                                           libc::c_uint)
                >> 32i32 - 11i32;
    B =
        B.wrapping_add(C ^ D ^
                           A).wrapping_add(X[15usize]).wrapping_add(0x6ed9eba1i32
                                                                        as
                                                                        libc::c_uint)
            << 15i32 |
            B.wrapping_add(C ^ D ^
                               A).wrapping_add(X[15usize]).wrapping_add(0x6ed9eba1i32
                                                                            as
                                                                            libc::c_uint)
                >> 32i32 - 15i32;
    A = (A as libc::c_uint).wrapping_add(AA) as uint32_t as uint32_t;
    B = (B as libc::c_uint).wrapping_add(BB) as uint32_t as uint32_t;
    C = (C as libc::c_uint).wrapping_add(CC) as uint32_t as uint32_t;
    D = (D as libc::c_uint).wrapping_add(DD) as uint32_t as uint32_t;
    j = 0i32;
    while j < 16i32 { X[j as usize] = 0i32 as uint32_t; j += 1 }
    (*m).A = A;
    (*m).B = B;
    (*m).C = C;
    (*m).D = D;
}
/* NOTE: This code makes no attempt to be fast!

   It assumes that an int is at least 32 bits long
*/
static mut m: *mut mdfour = 0 as *const mdfour as *mut mdfour;
unsafe extern "C" fn copy64(mut M: *mut uint32_t, mut in_0: *mut byte) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        *M.offset(i as isize) =
            (*in_0.offset((i * 4i32 + 3i32) as isize) as uint32_t) << 24i32 |
                (*in_0.offset((i * 4i32 + 2i32) as isize) as uint32_t) <<
                    16i32 |
                (*in_0.offset((i * 4i32 + 1i32) as isize) as uint32_t) << 8i32
                |
                (*in_0.offset((i * 4i32 + 0i32) as isize) as uint32_t) <<
                    0i32;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn mdfour_begin(mut md: *mut mdfour) {
    (*md).A = 0x67452301i32 as uint32_t;
    (*md).B = 0xefcdab89u32;
    (*md).C = 0x98badcfeu32;
    (*md).D = 0x10325476i32 as uint32_t;
    (*md).totalN = 0i32 as uint32_t;
}