use ::libc;

pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

use crate::src::qcommon::files::FS_FCloseFile;
use crate::src::qcommon::files::FS_Read;
use crate::src::qcommon::files::FS_SV_FOpenFileRead;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
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

pub type MD5_CTX = MD5Context;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MD5Context {
    pub buf: [crate::stdlib::uint32_t; 4],
    pub bits: [crate::stdlib::uint32_t; 2],
    pub in_0: [u8; 64],
}
/* Nothing */
// Q3_BIG_ENDIAN
/*
 * Start MD5 accumulation.  Set bit count to 0 and buffer to mysterious
 * initialization constants.
 */

unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context) {
    (*ctx).buf[0] = 0x67452301u32;
    (*ctx).buf[1] = 0xefcdab89u32;
    (*ctx).buf[2] = 0x98badcfeu32;
    (*ctx).buf[3] = 0x10325476u32;
    (*ctx).bits[0] = 0u32;
    (*ctx).bits[1] = 0u32;
}
/* The four core functions - F1 is optimized somewhat */
/* #define F1(x, y, z) (x & y | ~x & z) */
/* This is the central step in the MD5 algorithm. */
/*
 * The core of the MD5 algorithm, this alters an existing MD5 hash to
 * reflect the addition of 16 longwords of new data.  MD5Update blocks
 * the data and converts bytes into longwords for this routine.
 */

unsafe extern "C" fn MD5Transform(
    mut buf: *mut crate::stdlib::uint32_t,
    mut in_0: *const crate::stdlib::uint32_t,
) {
    let mut a: crate::stdlib::uint32_t = 0;
    let mut b: crate::stdlib::uint32_t = 0;
    let mut c: crate::stdlib::uint32_t = 0;
    let mut d: crate::stdlib::uint32_t = 0;
    a = *buf.offset(0);
    b = *buf.offset(1);
    c = *buf.offset(2);
    d = *buf.offset(3);
    a = (a).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(0))
            .wrapping_add(0xd76aa478u32),
    );
    a = a << 7 | a >> 32 - 7;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(1))
            .wrapping_add(0xe8c7b756u32),
    );
    d = d << 12 | d >> 32 - 12;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(2))
            .wrapping_add(0x242070dbu32),
    );
    c = c << 17 | c >> 32 - 17;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(3))
            .wrapping_add(0xc1bdceeeu32),
    );
    b = b << 22 | b >> 32 - 22;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(4))
            .wrapping_add(0xf57c0fafu32),
    );
    a = a << 7 | a >> 32 - 7;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(5))
            .wrapping_add(0x4787c62au32),
    );
    d = d << 12 | d >> 32 - 12;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(6))
            .wrapping_add(0xa8304613u32),
    );
    c = c << 17 | c >> 32 - 17;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(7))
            .wrapping_add(0xfd469501u32),
    );
    b = b << 22 | b >> 32 - 22;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(8))
            .wrapping_add(0x698098d8u32),
    );
    a = a << 7 | a >> 32 - 7;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(9))
            .wrapping_add(0x8b44f7afu32),
    );
    d = d << 12 | d >> 32 - 12;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(10))
            .wrapping_add(0xffff5bb1u32),
    );
    c = c << 17 | c >> 32 - 17;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(11))
            .wrapping_add(0x895cd7beu32),
    );
    b = b << 22 | b >> 32 - 22;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(12))
            .wrapping_add(0x6b901122u32),
    );
    a = a << 7 | a >> 32 - 7;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(13))
            .wrapping_add(0xfd987193u32),
    );
    d = d << 12 | d >> 32 - 12;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(14))
            .wrapping_add(0xa679438eu32),
    );
    c = c << 17 | c >> 32 - 17;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(15))
            .wrapping_add(0x49b40821u32),
    );
    b = b << 22 | b >> 32 - 22;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(1))
            .wrapping_add(0xf61e2562u32),
    );
    a = a << 5 | a >> 32 - 5;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(6))
            .wrapping_add(0xc040b340u32),
    );
    d = d << 9 | d >> 32 - 9;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(11))
            .wrapping_add(0x265e5a51u32),
    );
    c = c << 14 | c >> 32 - 14;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(0))
            .wrapping_add(0xe9b6c7aau32),
    );
    b = b << 20 | b >> 32 - 20;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(5))
            .wrapping_add(0xd62f105du32),
    );
    a = a << 5 | a >> 32 - 5;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(10))
            .wrapping_add(0x2441453u32),
    );
    d = d << 9 | d >> 32 - 9;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(15))
            .wrapping_add(0xd8a1e681u32),
    );
    c = c << 14 | c >> 32 - 14;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(4))
            .wrapping_add(0xe7d3fbc8u32),
    );
    b = b << 20 | b >> 32 - 20;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(9))
            .wrapping_add(0x21e1cde6u32),
    );
    a = a << 5 | a >> 32 - 5;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(14))
            .wrapping_add(0xc33707d6u32),
    );
    d = d << 9 | d >> 32 - 9;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(3))
            .wrapping_add(0xf4d50d87u32),
    );
    c = c << 14 | c >> 32 - 14;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(8))
            .wrapping_add(0x455a14edu32),
    );
    b = b << 20 | b >> 32 - 20;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(13))
            .wrapping_add(0xa9e3e905u32),
    );
    a = a << 5 | a >> 32 - 5;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(2))
            .wrapping_add(0xfcefa3f8u32),
    );
    d = d << 9 | d >> 32 - 9;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(7))
            .wrapping_add(0x676f02d9u32),
    );
    c = c << 14 | c >> 32 - 14;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(12))
            .wrapping_add(0x8d2a4c8au32),
    );
    b = b << 20 | b >> 32 - 20;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(5))
            .wrapping_add(0xfffa3942u32),
    );
    a = a << 4 | a >> 32 - 4;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(8))
            .wrapping_add(0x8771f681u32),
    );
    d = d << 11 | d >> 32 - 11;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(11))
            .wrapping_add(0x6d9d6122u32),
    );
    c = c << 16 | c >> 32 - 16;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(14))
            .wrapping_add(0xfde5380cu32),
    );
    b = b << 23 | b >> 32 - 23;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(1))
            .wrapping_add(0xa4beea44u32),
    );
    a = a << 4 | a >> 32 - 4;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(4))
            .wrapping_add(0x4bdecfa9u32),
    );
    d = d << 11 | d >> 32 - 11;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(7))
            .wrapping_add(0xf6bb4b60u32),
    );
    c = c << 16 | c >> 32 - 16;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(10))
            .wrapping_add(0xbebfbc70u32),
    );
    b = b << 23 | b >> 32 - 23;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(13))
            .wrapping_add(0x289b7ec6u32),
    );
    a = a << 4 | a >> 32 - 4;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(0))
            .wrapping_add(0xeaa127fau32),
    );
    d = d << 11 | d >> 32 - 11;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(3))
            .wrapping_add(0xd4ef3085u32),
    );
    c = c << 16 | c >> 32 - 16;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(6))
            .wrapping_add(0x4881d05u32),
    );
    b = b << 23 | b >> 32 - 23;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(9))
            .wrapping_add(0xd9d4d039u32),
    );
    a = a << 4 | a >> 32 - 4;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(12))
            .wrapping_add(0xe6db99e5u32),
    );
    d = d << 11 | d >> 32 - 11;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(15))
            .wrapping_add(0x1fa27cf8u32),
    );
    c = c << 16 | c >> 32 - 16;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(2))
            .wrapping_add(0xc4ac5665u32),
    );
    b = b << 23 | b >> 32 - 23;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(0))
            .wrapping_add(0xf4292244u32),
    );
    a = a << 6 | a >> 32 - 6;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(7))
            .wrapping_add(0x432aff97u32),
    );
    d = d << 10 | d >> 32 - 10;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(14))
            .wrapping_add(0xab9423a7u32),
    );
    c = c << 15 | c >> 32 - 15;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(5))
            .wrapping_add(0xfc93a039u32),
    );
    b = b << 21 | b >> 32 - 21;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(12))
            .wrapping_add(0x655b59c3u32),
    );
    a = a << 6 | a >> 32 - 6;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(3))
            .wrapping_add(0x8f0ccc92u32),
    );
    d = d << 10 | d >> 32 - 10;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(10))
            .wrapping_add(0xffeff47du32),
    );
    c = c << 15 | c >> 32 - 15;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(1))
            .wrapping_add(0x85845dd1u32),
    );
    b = b << 21 | b >> 32 - 21;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(8))
            .wrapping_add(0x6fa87e4fu32),
    );
    a = a << 6 | a >> 32 - 6;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(15))
            .wrapping_add(0xfe2ce6e0u32),
    );
    d = d << 10 | d >> 32 - 10;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(6))
            .wrapping_add(0xa3014314u32),
    );
    c = c << 15 | c >> 32 - 15;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(13))
            .wrapping_add(0x4e0811a1u32),
    );
    b = b << 21 | b >> 32 - 21;
    b = (b).wrapping_add(c);
    a = (a).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(4))
            .wrapping_add(0xf7537e82u32),
    );
    a = a << 6 | a >> 32 - 6;
    a = (a).wrapping_add(b);
    d = (d).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(11))
            .wrapping_add(0xbd3af235u32),
    );
    d = d << 10 | d >> 32 - 10;
    d = (d).wrapping_add(a);
    c = (c).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(2))
            .wrapping_add(0x2ad7d2bbu32),
    );
    c = c << 15 | c >> 32 - 15;
    c = (c).wrapping_add(d);
    b = (b).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(9))
            .wrapping_add(0xeb86d391u32),
    );
    b = b << 21 | b >> 32 - 21;
    b = (b).wrapping_add(c);
    let ref mut fresh0 = *buf.offset(0);
    *fresh0 = (*fresh0).wrapping_add(a);
    let ref mut fresh1 = *buf.offset(1);
    *fresh1 = (*fresh1).wrapping_add(b);
    let ref mut fresh2 = *buf.offset(2);
    *fresh2 = (*fresh2).wrapping_add(c);
    let ref mut fresh3 = *buf.offset(3);
    *fresh3 = (*fresh3).wrapping_add(d);
}
/*
 * Update context to reflect the concatenation of another buffer full
 * of bytes.
 */

unsafe extern "C" fn MD5Update(mut ctx: *mut MD5Context, mut buf: *const u8, mut len: u32) {
    let mut t: crate::stdlib::uint32_t = 0;
    /* Update bitcount */
    t = (*ctx).bits[0]; /* Carry from low to high */
    (*ctx).bits[0] = t.wrapping_add(len << 3); /* Bytes already in shsInfo->data */
    if (*ctx).bits[0] < t {
        (*ctx).bits[1] = (*ctx).bits[1].wrapping_add(1)
    }
    (*ctx).bits[1] = ((*ctx).bits[1]).wrapping_add(len >> 29);
    t = t >> 3 & 0x3f;
    /* Handle any leading odd-sized chunks */
    if t != 0 {
        let mut p: *mut u8 = (*ctx).in_0.as_mut_ptr().offset(t as isize);
        t = (64u32).wrapping_sub(t);
        if len < t {
            crate::stdlib::memcpy(
                p as *mut libc::c_void,
                buf as *const libc::c_void,
                len as usize,
            );
            return;
        }
        crate::stdlib::memcpy(
            p as *mut libc::c_void,
            buf as *const libc::c_void,
            t as usize,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *const crate::stdlib::uint32_t,
        );
        buf = buf.offset(t as isize);
        len = len.wrapping_sub(t)
    }
    /* Process data in 64-byte chunks */
    while len >= 64u32 {
        crate::stdlib::memcpy(
            (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            64,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *const crate::stdlib::uint32_t,
        );
        buf = buf.offset(64);
        len = len.wrapping_sub(64u32)
    }
    /* Handle any remaining bytes of data. */
    crate::stdlib::memcpy(
        (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
        buf as *const libc::c_void,
        len as usize,
    );
}
/*
 * Final wrapup - pad to 64-byte boundary with the bit pattern
 * 1 0* (64-bit count of bits processed, MSB-first)
 */

unsafe extern "C" fn MD5Final(mut ctx: *mut MD5Context, mut digest: *mut u8) {
    let mut count: u32 = 0;
    let mut p: *mut u8 = 0 as *mut u8;
    /* Compute number of bytes mod 64 */
    count = (*ctx).bits[0] >> 3 & 0x3f;
    /* Set the first char of padding to 0x80.  This is safe since there is
    always at least one byte free */
    p = (*ctx).in_0.as_mut_ptr().offset(count as isize);
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = 0x80u8;
    /* Bytes of padding needed to make 64 bytes */
    count = ((64i32 - 1) as u32).wrapping_sub(count);
    /* Pad out to 56 mod 64 */
    if count < 8 {
        /* Two lots of padding:  Pad the first block to 64 bytes */
        crate::stdlib::memset(p as *mut libc::c_void, 0, count as usize);
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *const crate::stdlib::uint32_t,
        );
        /* Now fill the next block with 56 bytes */
        crate::stdlib::memset((*ctx).in_0.as_mut_ptr() as *mut libc::c_void, 0i32, 56usize);
    } else {
        /* Pad block to 56 bytes */
        crate::stdlib::memset(
            p as *mut libc::c_void,
            0i32,
            count.wrapping_sub(8u32) as usize,
        );
    }
    /* Append length in bits and transform */
    *((*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t).offset(14) = (*ctx).bits[0];
    *((*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t).offset(15) = (*ctx).bits[1];
    MD5Transform(
        (*ctx).buf.as_mut_ptr(),
        (*ctx).in_0.as_mut_ptr() as *const crate::stdlib::uint32_t,
    );
    if !digest.is_null() {
        crate::stdlib::memcpy(
            digest as *mut libc::c_void,
            (*ctx).buf.as_mut_ptr() as *const libc::c_void,
            16usize,
        );
    }
    crate::stdlib::memset(
        ctx as *mut libc::c_void,
        0,
        ::std::mem::size_of::<MD5Context>(),
    );
    /* In case it's sensitive */
}
#[no_mangle]

pub unsafe extern "C" fn Com_MD5File(
    mut fn_0: *const i8,
    mut length: i32,
    mut prefix: *const i8,
    mut prefix_len: i32,
) -> *mut i8 {
    static mut final_0: [i8; 33] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    let mut digest: [u8; 16] = *::std::mem::transmute::<&[u8; 16], &mut [u8; 16]>(
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
    );
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut md5: MD5_CTX = MD5_CTX {
        buf: [0; 4],
        bits: [0; 2],
        in_0: [0; 64],
    };
    let mut buffer: [crate::src::qcommon::q_shared::byte; 2048] = [0; 2048];
    let mut i: i32 = 0;
    let mut filelen: i32 = 0;
    let mut r: i32 = 0;
    let mut total: i32 = 0;
    crate::src::qcommon::q_shared::Q_strncpyz(
        final_0.as_mut_ptr(),
        b"\x00" as *const u8 as *const i8,
        ::std::mem::size_of::<[i8; 33]>() as i32,
    );
    filelen = crate::src::qcommon::files::FS_SV_FOpenFileRead(fn_0, &mut f) as i32;
    if f == 0 {
        return final_0.as_mut_ptr();
    }
    if filelen < 1 {
        crate::src::qcommon::files::FS_FCloseFile(f);
        return final_0.as_mut_ptr();
    }
    if filelen < length || length == 0 {
        length = filelen
    }
    MD5Init(&mut md5);
    if prefix_len != 0 && *prefix as i32 != 0 {
        MD5Update(&mut md5, prefix as *mut u8, prefix_len as u32);
    }
    loop {
        r = crate::src::qcommon::files::FS_Read(
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 2048]>() as i32,
            f,
        );
        if r < 1 {
            break;
        }
        if r + total > length {
            r = length - total
        }
        total += r;
        MD5Update(&mut md5, buffer.as_mut_ptr(), r as u32);
        if (r as usize) < ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 2048]>()
            || total >= length
        {
            break;
        }
    }
    crate::src::qcommon::files::FS_FCloseFile(f);
    MD5Final(&mut md5, digest.as_mut_ptr());
    final_0[0] = '\u{0}' as i8;

    for i in 0..16 {
        crate::src::qcommon::q_shared::Q_strcat(
            final_0.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 33]>() as i32,
            crate::src::qcommon::q_shared::va(
                b"%02X\x00" as *const u8 as *mut i8,
                digest[i as usize] as i32,
            ),
        );
    }
    return final_0.as_mut_ptr();
}
