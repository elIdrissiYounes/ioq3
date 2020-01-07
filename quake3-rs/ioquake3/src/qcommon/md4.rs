use ::libc;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::stdlib::__uint32_t;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::stdlib::uint32_t;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdfour {
    pub A: crate::stdlib::uint32_t,
    pub B: crate::stdlib::uint32_t,
    pub C: crate::stdlib::uint32_t,
    pub D: crate::stdlib::uint32_t,
    pub totalN: crate::stdlib::uint32_t,
}
/* NOTE: This code makes no attempt to be fast!

   It assumes that an int is at least 32 bits long
*/

static mut m: *mut mdfour = 0 as *mut mdfour;
/* this applies md4 to 64 byte chunks */

unsafe extern "C" fn mdfour64(mut M: *mut crate::stdlib::uint32_t) {
    let mut j: i32 = 0;
    let mut AA: crate::stdlib::uint32_t = 0;
    let mut BB: crate::stdlib::uint32_t = 0;
    let mut CC: crate::stdlib::uint32_t = 0;
    let mut DD: crate::stdlib::uint32_t = 0;
    let mut X: [crate::stdlib::uint32_t; 16] = [0; 16];
    let mut A: crate::stdlib::uint32_t = 0;
    let mut B: crate::stdlib::uint32_t = 0;
    let mut C: crate::stdlib::uint32_t = 0;
    let mut D: crate::stdlib::uint32_t = 0;
    j = 0;
    while j < 16 {
        X[j as usize] = *M.offset(j as isize);
        j += 1
    }
    A = (*m).A;
    B = (*m).B;
    C = (*m).C;
    D = (*m).D;
    AA = A;
    BB = B;
    CC = C;
    DD = D;
    A = A.wrapping_add(B & C | !B & D).wrapping_add(X[0]) << 3
        | A.wrapping_add(B & C | !B & D).wrapping_add(X[0]) >> 32 - 3;
    D = D.wrapping_add(A & B | !A & C).wrapping_add(X[1]) << 7
        | D.wrapping_add(A & B | !A & C).wrapping_add(X[1]) >> 32 - 7;
    C = C.wrapping_add(D & A | !D & B).wrapping_add(X[2]) << 11
        | C.wrapping_add(D & A | !D & B).wrapping_add(X[2]) >> 32 - 11;
    B = B.wrapping_add(C & D | !C & A).wrapping_add(X[3]) << 19
        | B.wrapping_add(C & D | !C & A).wrapping_add(X[3]) >> 32 - 19;
    A = A.wrapping_add(B & C | !B & D).wrapping_add(X[4]) << 3
        | A.wrapping_add(B & C | !B & D).wrapping_add(X[4]) >> 32 - 3;
    D = D.wrapping_add(A & B | !A & C).wrapping_add(X[5]) << 7
        | D.wrapping_add(A & B | !A & C).wrapping_add(X[5]) >> 32 - 7;
    C = C.wrapping_add(D & A | !D & B).wrapping_add(X[6]) << 11
        | C.wrapping_add(D & A | !D & B).wrapping_add(X[6]) >> 32 - 11;
    B = B.wrapping_add(C & D | !C & A).wrapping_add(X[7]) << 19
        | B.wrapping_add(C & D | !C & A).wrapping_add(X[7]) >> 32 - 19;
    A = A.wrapping_add(B & C | !B & D).wrapping_add(X[8]) << 3
        | A.wrapping_add(B & C | !B & D).wrapping_add(X[8]) >> 32 - 3;
    D = D.wrapping_add(A & B | !A & C).wrapping_add(X[9]) << 7
        | D.wrapping_add(A & B | !A & C).wrapping_add(X[9]) >> 32 - 7;
    C = C.wrapping_add(D & A | !D & B).wrapping_add(X[10]) << 11
        | C.wrapping_add(D & A | !D & B).wrapping_add(X[10]) >> 32 - 11;
    B = B.wrapping_add(C & D | !C & A).wrapping_add(X[11]) << 19
        | B.wrapping_add(C & D | !C & A).wrapping_add(X[11]) >> 32 - 19;
    A = A.wrapping_add(B & C | !B & D).wrapping_add(X[12]) << 3
        | A.wrapping_add(B & C | !B & D).wrapping_add(X[12]) >> 32 - 3;
    D = D.wrapping_add(A & B | !A & C).wrapping_add(X[13]) << 7
        | D.wrapping_add(A & B | !A & C).wrapping_add(X[13]) >> 32 - 7;
    C = C.wrapping_add(D & A | !D & B).wrapping_add(X[14]) << 11
        | C.wrapping_add(D & A | !D & B).wrapping_add(X[14]) >> 32 - 11;
    B = B.wrapping_add(C & D | !C & A).wrapping_add(X[15]) << 19
        | B.wrapping_add(C & D | !C & A).wrapping_add(X[15]) >> 32 - 19;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[0])
        .wrapping_add(0x5a827999u32)
        << 3
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[0])
            .wrapping_add(0x5a827999u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[4])
        .wrapping_add(0x5a827999u32)
        << 5
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[4])
            .wrapping_add(0x5a827999u32)
            >> 32 - 5;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[8])
        .wrapping_add(0x5a827999u32)
        << 9
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[8])
            .wrapping_add(0x5a827999u32)
            >> 32 - 9;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[12])
        .wrapping_add(0x5a827999u32)
        << 13
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[12])
            .wrapping_add(0x5a827999u32)
            >> 32 - 13;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[1])
        .wrapping_add(0x5a827999u32)
        << 3
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[1])
            .wrapping_add(0x5a827999u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[5])
        .wrapping_add(0x5a827999u32)
        << 5
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[5])
            .wrapping_add(0x5a827999u32)
            >> 32 - 5;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[9])
        .wrapping_add(0x5a827999u32)
        << 9
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[9])
            .wrapping_add(0x5a827999u32)
            >> 32 - 9;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[13])
        .wrapping_add(0x5a827999u32)
        << 13
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[13])
            .wrapping_add(0x5a827999u32)
            >> 32 - 13;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[2])
        .wrapping_add(0x5a827999u32)
        << 3
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[2])
            .wrapping_add(0x5a827999u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[6])
        .wrapping_add(0x5a827999u32)
        << 5
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[6])
            .wrapping_add(0x5a827999u32)
            >> 32 - 5;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[10])
        .wrapping_add(0x5a827999u32)
        << 9
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[10])
            .wrapping_add(0x5a827999u32)
            >> 32 - 9;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[14])
        .wrapping_add(0x5a827999u32)
        << 13
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[14])
            .wrapping_add(0x5a827999u32)
            >> 32 - 13;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[3])
        .wrapping_add(0x5a827999u32)
        << 3
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[3])
            .wrapping_add(0x5a827999u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[7])
        .wrapping_add(0x5a827999u32)
        << 5
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[7])
            .wrapping_add(0x5a827999u32)
            >> 32 - 5;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[11])
        .wrapping_add(0x5a827999u32)
        << 9
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[11])
            .wrapping_add(0x5a827999u32)
            >> 32 - 9;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[15])
        .wrapping_add(0x5a827999u32)
        << 13
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[15])
            .wrapping_add(0x5a827999u32)
            >> 32 - 13;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[0])
        .wrapping_add(0x6ed9eba1u32)
        << 3
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[0])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[8])
        .wrapping_add(0x6ed9eba1u32)
        << 9
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[8])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 9;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[4])
        .wrapping_add(0x6ed9eba1u32)
        << 11
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[4])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 11;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[12])
        .wrapping_add(0x6ed9eba1u32)
        << 15
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[12])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 15;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[2])
        .wrapping_add(0x6ed9eba1u32)
        << 3
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[2])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[10])
        .wrapping_add(0x6ed9eba1u32)
        << 9
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[10])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 9;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[6])
        .wrapping_add(0x6ed9eba1u32)
        << 11
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[6])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 11;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[14])
        .wrapping_add(0x6ed9eba1u32)
        << 15
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[14])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 15;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[1])
        .wrapping_add(0x6ed9eba1u32)
        << 3
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[1])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[9])
        .wrapping_add(0x6ed9eba1u32)
        << 9
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[9])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 9;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[5])
        .wrapping_add(0x6ed9eba1u32)
        << 11
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[5])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 11;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[13])
        .wrapping_add(0x6ed9eba1u32)
        << 15
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[13])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 15;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[3])
        .wrapping_add(0x6ed9eba1u32)
        << 3
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[3])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 3;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[11])
        .wrapping_add(0x6ed9eba1u32)
        << 9
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[11])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 9;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[7])
        .wrapping_add(0x6ed9eba1u32)
        << 11
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[7])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 11;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[15])
        .wrapping_add(0x6ed9eba1u32)
        << 15
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[15])
            .wrapping_add(0x6ed9eba1u32)
            >> 32 - 15;
    A = (A).wrapping_add(AA);
    B = (B).wrapping_add(BB);
    C = (C).wrapping_add(CC);
    D = (D).wrapping_add(DD);
    j = 0;
    while j < 16 {
        X[j as usize] = 0;
        j += 1
    }
    (*m).A = A;
    (*m).B = B;
    (*m).C = C;
    (*m).D = D;
}

unsafe extern "C" fn copy64(
    mut M: *mut crate::stdlib::uint32_t,
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 16 {
        *M.offset(i as isize) = (*in_0.offset((i * 4 + 3) as isize) as crate::stdlib::uint32_t)
            << 24
            | (*in_0.offset((i * 4 + 2) as isize) as crate::stdlib::uint32_t) << 16
            | (*in_0.offset((i * 4 + 1) as isize) as crate::stdlib::uint32_t) << 8
            | (*in_0.offset((i * 4 + 0) as isize) as crate::stdlib::uint32_t) << 0;
        i += 1
    }
}

unsafe extern "C" fn copy4(
    mut out: *mut crate::src::qcommon::q_shared::byte,
    mut x: crate::stdlib::uint32_t,
) {
    *out.offset(0) = (x & 0xffu32) as crate::src::qcommon::q_shared::byte;
    *out.offset(1) = (x >> 8 & 0xffu32) as crate::src::qcommon::q_shared::byte;
    *out.offset(2) = (x >> 16 & 0xffu32) as crate::src::qcommon::q_shared::byte;
    *out.offset(3) = (x >> 24 & 0xffu32) as crate::src::qcommon::q_shared::byte;
}
#[no_mangle]

pub unsafe extern "C" fn mdfour_begin(mut md: *mut mdfour) {
    (*md).A = 0x67452301u32;
    (*md).B = 0xefcdab89u32;
    (*md).C = 0x98badcfeu32;
    (*md).D = 0x10325476u32;
    (*md).totalN = 0u32;
}

unsafe extern "C" fn mdfour_tail(mut in_0: *mut crate::src::qcommon::q_shared::byte, mut n: i32) {
    let mut buf: [crate::src::qcommon::q_shared::byte; 128] = [0; 128];
    let mut M: [crate::stdlib::uint32_t; 16] = [0; 16];
    let mut b: crate::stdlib::uint32_t = 0;
    (*m).totalN = ((*m).totalN).wrapping_add(n as u32);
    b = (*m).totalN.wrapping_mul(8u32);
    crate::stdlib::memset(buf.as_mut_ptr() as *mut libc::c_void, 0, 128);
    if n != 0 {
        crate::stdlib::memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            in_0 as *const libc::c_void,
            n as usize,
        );
    }
    buf[n as usize] = 0x80;
    if n <= 55 {
        copy4(buf.as_mut_ptr().offset(56), b);
        copy64(M.as_mut_ptr(), buf.as_mut_ptr());
        mdfour64(M.as_mut_ptr());
    } else {
        copy4(buf.as_mut_ptr().offset(120), b);
        copy64(M.as_mut_ptr(), buf.as_mut_ptr());
        mdfour64(M.as_mut_ptr());
        copy64(M.as_mut_ptr(), buf.as_mut_ptr().offset(64));
        mdfour64(M.as_mut_ptr());
    };
}

unsafe extern "C" fn mdfour_update(
    mut md: *mut mdfour,
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut n: i32,
) {
    let mut M: [crate::stdlib::uint32_t; 16] = [0; 16];
    m = md;
    if n == 0 {
        mdfour_tail(in_0, n);
    }
    while n >= 64 {
        copy64(M.as_mut_ptr(), in_0);
        mdfour64(M.as_mut_ptr());
        in_0 = in_0.offset(64);
        n -= 64;
        (*m).totalN = ((*m).totalN).wrapping_add(64u32)
    }
    mdfour_tail(in_0, n);
}

unsafe extern "C" fn mdfour_result(
    mut md: *mut mdfour,
    mut out: *mut crate::src::qcommon::q_shared::byte,
) {
    copy4(out, (*md).A);
    copy4(out.offset(4), (*md).B);
    copy4(out.offset(8), (*md).C);
    copy4(out.offset(12), (*md).D);
}

unsafe extern "C" fn mdfour(
    mut out: *mut crate::src::qcommon::q_shared::byte,
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut n: i32,
) {
    let mut md: mdfour = mdfour {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        totalN: 0,
    };
    mdfour_begin(&mut md);
    mdfour_update(&mut md, in_0, n);
    mdfour_result(&mut md, out);
}
// will be journaled properly
//===================================================================
#[no_mangle]

pub unsafe extern "C" fn Com_BlockChecksum(
    mut buffer: *const libc::c_void,
    mut length: i32,
) -> u32 {
    let mut digest: [i32; 4] = [0; 4];
    let mut val: u32 = 0;
    mdfour(
        digest.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        buffer as *mut crate::src::qcommon::q_shared::byte,
        length,
    );
    val = (digest[0] ^ digest[1] ^ digest[2] ^ digest[3]) as u32;
    return val;
}
