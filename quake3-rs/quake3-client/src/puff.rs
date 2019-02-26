#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_slice_as_ptr, custom_attribute, libc)]
extern crate libc;
#[header_src = "/usr/include/bits/setjmp.h"]
pub mod setjmp_h {
    pub type __jmp_buf = [libc::c_long; 8];
    use super::{libc};
}
#[header_src = "/usr/include/bits/types/__sigset_t.h"]
pub mod __sigset_t_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
    use super::{libc};
}
#[header_src = "/usr/include/setjmp.h"]
pub mod include_setjmp_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: __jmp_buf,
        pub __mask_was_saved: libc::c_int,
        pub __saved_mask: __sigset_t,
    }
    pub type jmp_buf = [__jmp_buf_tag; 1];
    use super::setjmp_h::{__jmp_buf};
    use super::{libc};
    use super::__sigset_t_h::{__sigset_t};
    extern "C" {
        #[no_mangle]
        pub fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
        #[no_mangle]
        pub fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    }
}
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int16_t = libc::c_short;
    pub type __int32_t = libc::c_int;
    pub type __uint32_t = libc::c_uint;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int16_t = __int16_t;
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[header_src =
      "ioq3/code/qcommon/puff.c"]
pub mod puff_c {
    /*
 *  This is a modified version of Mark Adlers work,
 *  see below for the original copyright.
 *  2006 - Joerg Dietrich <dietrich_joerg@gmx.de>
 */
    /*
 * puff.c
 * Copyright (C) 2002-2004 Mark Adler
 * For conditions of distribution and use, see copyright notice in puff.h
 * version 1.8, 9 Jan 2004
 *
 * puff.c is a simple inflate written to be an unambiguous way to specify the
 * deflate format.  It is not written for speed but rather simplicity.  As a
 * side benefit, this code might actually be useful when small code is more
 * important than speed, such as bootstrap applications.  For typical deflate
 * data, zlib's inflate() is about four times as fast as puff().  zlib's
 * inflate compiles to around 20K on my machine, whereas puff.c compiles to
 * around 4K on my machine (a PowerPC using GNU cc).  If the faster decode()
 * function here is used, then puff() is only twice as slow as zlib's
 * inflate().
 *
 * All dynamically allocated memory comes from the stack.  The stack required
 * is less than 2K bytes.  This code is compatible with 16-bit int's and
 * assumes that long's are at least 32 bits.  puff.c uses the short data type,
 * assumed to be 16 bits, for arrays in order to to conserve memory.  The code
 * works whether integers are stored big endian or little endian.
 *
 * In the comments below are "Format notes" that describe the inflate process
 * and document some of the less obvious aspects of the format.  This source
 * code is meant to supplement RFC 1951, which formally describes the deflate
 * format:
 *
 *    http://www.zlib.org/rfc-deflate.html
 */
    /*
 * Change history:
 *
 * 1.0  10 Feb 2002     - First version
 * 1.1  17 Feb 2002     - Clarifications of some comments and notes
 *                      - Update puff() dest and source pointers on negative
 *                        errors to facilitate debugging deflators
 *                      - Remove longest from struct huffman -- not needed
 *                      - Simplify offs[] index in construct()
 *                      - Add input size and checking, using longjmp() to
 *                        maintain easy readability
 *                      - Use short data type for large arrays
 *                      - Use pointers instead of long to specify source and
 *                        destination sizes to avoid arbitrary 4 GB limits
 * 1.2  17 Mar 2002     - Add faster version of decode(), doubles speed (!),
 *                        but leave simple version for readabilty
 *                      - Make sure invalid distances detected if pointers
 *                        are 16 bits
 *                      - Fix fixed codes table error
 *                      - Provide a scanning mode for determining size of
 *                        uncompressed data
 * 1.3  20 Mar 2002     - Go back to lengths for puff() parameters [Jean-loup]
 *                      - Add a puff.h file for the interface
 *                      - Add braces in puff() for else do [Jean-loup]
 *                      - Use indexes instead of pointers for readability
 * 1.4  31 Mar 2002     - Simplify construct() code set check
 *                      - Fix some comments
 *                      - Add FIXLCODES #define
 * 1.5   6 Apr 2002     - Minor comment fixes
 * 1.6   7 Aug 2002     - Minor format changes
 * 1.7   3 Mar 2003     - Added test code for distribution
 *                      - Added zlib-like license
 * 1.8   9 Jan 2004     - Added some comments on no distance codes case
 */
    /* for setjmp(), longjmp(), and jmp_buf */
    /* for local function definitions */
    /*
 * Maximums for allocations and loops.  It is not useful to change these --
 * they are fixed by the deflate format.
 */
    /* maximum bits in a code */
    /* maximum number of literal/length codes */
    /* maximum number of distance codes */
    /* maximum codes lengths to read */
    /* number of fixed literal/length codes */
    /* input and output state */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct state {
        pub out: *mut uint8_t,
        pub outlen: uint32_t,
        pub outcnt: uint32_t,
        pub in_0: *mut uint8_t,
        pub inlen: uint32_t,
        pub incnt: uint32_t,
        pub bitbuf: int32_t,
        pub bitcnt: int32_t,
        pub env: jmp_buf,
    }
    /*
 * Huffman code decoding tables.  count[1..MAXBITS] is the number of symbols of
 * each length, which for a canonical code are stepped through in order.
 * symbol[] are the symbol values in canonical order, where the number of
 * entries is the sum of the counts in count[].  The decoding process can be
 * seen in the function decode() below.
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct huffman {
        pub count: *mut int16_t,
        pub symbol: *mut int16_t,
    }
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::{int32_t, int16_t};
    use super::include_setjmp_h::{jmp_buf};
}
#[header_src =
      "ioq3/code/qcommon/puff.h"]
pub mod puff_h {
    use super::stdint_intn_h::{int32_t};
    use super::stdint_uintn_h::{uint8_t, uint32_t};
}
use self::setjmp_h::{__jmp_buf};
use self::__sigset_t_h::{__sigset_t};
use self::include_setjmp_h::{__jmp_buf_tag, jmp_buf, _setjmp, longjmp};
use self::types_h::{__uint8_t, __int16_t, __int32_t, __uint32_t};
use self::stdint_intn_h::{int16_t, int32_t};
use self::stdint_uintn_h::{uint8_t, uint32_t};
use self::puff_c::{state, huffman};
/*
 *  This is a modified version of Mark Adlers work,
 *  see below for the original copyright.
 *  2006 - Joerg Dietrich <dietrich_joerg@gmx.de>
 */
/* puff.h
  Copyright (C) 2002, 2003 Mark Adler, all rights reserved
  version 1.7, 3 Mar 2002

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the author be held liable for any damages
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

  Mark Adler    madler@alumni.caltech.edu
 */
/*
 * See puff.c for purpose and usage.
 */
/* pointer to destination pointer */
#[no_mangle]
pub unsafe extern "C" fn puff(mut dest: *mut uint8_t,
                              mut destlen: *mut uint32_t,
                              mut source: *mut uint8_t,
                              mut sourcelen: *mut uint32_t) -> int32_t {
    /* input/output state */
    let mut s: state =
        state{out: 0 as *mut uint8_t,
              outlen: 0,
              outcnt: 0,
              in_0: 0 as *mut uint8_t,
              inlen: 0,
              incnt: 0,
              bitbuf: 0,
              bitcnt: 0,
              env:
                  [__jmp_buf_tag{__jmpbuf: [0; 8],
                                 __mask_was_saved: 0,
                                 __saved_mask: __sigset_t{__val: [0; 16],},};
                      1],};
    /* block information */
    let mut last: int32_t = 0;
    let mut type_0: int32_t = 0;
    /* return value */
    let mut err: int32_t = 0;
    s.out = dest;
    s.outlen = *destlen;
    s.outcnt = 0i32 as uint32_t;
    s.in_0 = source;
    s.inlen = *sourcelen;
    s.incnt = 0i32 as uint32_t;
    s.bitbuf = 0i32;
    s.bitcnt = 0i32;
    if _setjmp(s.env.as_mut_ptr()) != 0i32 {
        err = 2i32
    } else {
        loop  {
            last = bits(&mut s, 1i32);
            type_0 = bits(&mut s, 2i32);
            err =
                if type_0 == 0i32 {
                    stored(&mut s)
                } else if type_0 == 1i32 {
                    fixed(&mut s)
                } else if type_0 == 2i32 { dynamic(&mut s) } else { -1i32 };
            /* type == 3, invalid */
            if err != 0i32 {
                /* return with error */
                break ;
            } else if !(0 == last) { break ; }
        }
    }
    if err <= 0i32 { *destlen = s.outcnt; *sourcelen = s.incnt }
    return err;
}
/*
 * Process a dynamic codes block.
 *
 * Format notes:
 *
 * - A dynamic block starts with a description of the literal/length and
 *   distance codes for that block.  New dynamic blocks allow the compressor to
 *   rapidly adapt to changing data with new codes optimized for that data.
 *
 * - The codes used by the deflate format are "canonical", which means that
 *   the actual bits of the codes are generated in an unambiguous way simply
 *   from the number of bits in each code.  Therefore the code descriptions
 *   are simply a list of code lengths for each symbol.
 *
 * - The code lengths are stored in order for the symbols, so lengths are
 *   provided for each of the literal/length symbols, and for each of the
 *   distance symbols.
 *
 * - If a symbol is not used in the block, this is represented by a zero as
 *   as the code length.  This does not mean a zero-length code, but rather
 *   that no code should be created for this symbol.  There is no way in the
 *   deflate format to represent a zero-length code.
 *
 * - The maximum number of bits in a code is 15, so the possible lengths for
 *   any code are 1..15.
 *
 * - The fact that a length of zero is not permitted for a code has an
 *   interesting consequence.  Normally if only one symbol is used for a given
 *   code, then in fact that code could be represented with zero bits.  However
 *   in deflate, that code has to be at least one bit.  So for example, if
 *   only a single distance base symbol appears in a block, then it will be
 *   represented by a single code of length one, in particular one 0 bit.  This
 *   is an incomplete code, since if a 1 bit is received, it has no meaning,
 *   and should result in an error.  So incomplete distance codes of one symbol
 *   should be permitted, and the receipt of invalid codes should be handled.
 *
 * - It is also possible to have a single literal/length code, but that code
 *   must be the end-of-block code, since every dynamic block has one.  This
 *   is not the most efficient way to create an empty block (an empty fixed
 *   block is fewer bits), but it is allowed by the format.  So incomplete
 *   literal/length codes of one symbol should also be permitted.
 *
 * - If there are only literal codes and no lengths, then there are no distance
 *   codes.  This is represented by one distance code with zero bits.
 *
 * - The list of up to 286 length/literal lengths and up to 30 distance lengths
 *   are themselves compressed using Huffman codes and run-length encoding.  In
 *   the list of code lengths, a 0 symbol means no code, a 1..15 symbol means
 *   that length, and the symbols 16, 17, and 18 are run-length instructions.
 *   Each of 16, 17, and 18 are follwed by extra bits to define the length of
 *   the run.  16 copies the last length 3 to 6 times.  17 represents 3 to 10
 *   zero lengths, and 18 represents 11 to 138 zero lengths.  Unused symbols
 *   are common, hence the special coding for zero lengths.
 *
 * - The symbols for 0..18 are Huffman coded, and so that code must be
 *   described first.  This is simply a sequence of up to 19 three-bit values
 *   representing no code (0) or the code length for that symbol (1..7).
 *
 * - A dynamic block starts with three fixed-size counts from which is computed
 *   the number of literal/length code lengths, the number of distance code
 *   lengths, and the number of code length code lengths (ok, you come up with
 *   a better name!) in the code descriptions.  For the literal/length and
 *   distance codes, lengths after those provided are considered zero, i.e. no
 *   code.  The code length code lengths are received in a permuted order (see
 *   the order[] array below) to make a short code length code length list more
 *   likely.  As it turns out, very short and very long codes are less likely
 *   to be seen in a dynamic code description, hence what may appear initially
 *   to be a peculiar ordering.
 *
 * - Given the number of literal/length code lengths (nlen) and distance code
 *   lengths (ndist), then they are treated as one long list of nlen + ndist
 *   code lengths.  Therefore run-length coding can and often does cross the
 *   boundary between the two sets of lengths.
 *
 * - So to summarize, the code description at the start of a dynamic block is
 *   three counts for the number of code lengths for the literal/length codes,
 *   the distance codes, and the code length codes.  This is followed by the
 *   code length code lengths, three bits each.  This is used to construct the
 *   code length code which is used to read the remainder of the lengths.  Then
 *   the literal/length code lengths and distance lengths are read as a single
 *   set of lengths using the code length codes.  Codes are constructed from
 *   the resulting two sets of lengths, and then finally you can start
 *   decoding actual compressed data in the block.
 *
 * - For reference, a "typical" size for the code description in a dynamic
 *   block is around 80 bytes.
 */
unsafe extern "C" fn dynamic(mut s: *mut state) -> int32_t {
    /* number of lengths in descriptor */
    let mut nlen: int32_t = 0;
    let mut ndist: int32_t = 0;
    let mut ncode: int32_t = 0;
    /* index of lengths[] */
    let mut index: int32_t = 0;
    /* construct() return value */
    let mut err: int32_t = 0;
    /* descriptor code lengths */
    let mut lengths: [int16_t; 316] = [0; 316];
    /* lencode memory */
    let mut lencnt: [int16_t; 16] = [0; 16];
    let mut lensym: [int16_t; 286] = [0; 286];
    /* distcode memory */
    let mut distcnt: [int16_t; 16] = [0; 16];
    let mut distsym: [int16_t; 30] = [0; 30];
    /* length code */
    let mut lencode: huffman =
        huffman{count: lencnt.as_mut_ptr(), symbol: lensym.as_mut_ptr(),};
    /* distance code */
    let mut distcode: huffman =
        huffman{count: distcnt.as_mut_ptr(), symbol: distsym.as_mut_ptr(),};
    /* permutation of code length codes */
    static mut order: [int16_t; 19] =
        [16i32 as int16_t, 17i32 as int16_t, 18i32 as int16_t,
         0i32 as int16_t, 8i32 as int16_t, 7i32 as int16_t, 9i32 as int16_t,
         6i32 as int16_t, 10i32 as int16_t, 5i32 as int16_t, 11i32 as int16_t,
         4i32 as int16_t, 12i32 as int16_t, 3i32 as int16_t, 13i32 as int16_t,
         2i32 as int16_t, 14i32 as int16_t, 1i32 as int16_t,
         15i32 as int16_t];
    nlen = bits(s, 5i32) + 257i32;
    ndist = bits(s, 5i32) + 1i32;
    ncode = bits(s, 4i32) + 4i32;
    if nlen > 286i32 || ndist > 30i32 { return -3i32 }
    index = 0i32;
    while index < ncode {
        lengths[order[index as usize] as usize] = bits(s, 3i32) as int16_t;
        index += 1
    }
    while index < 19i32 {
        lengths[order[index as usize] as usize] = 0i32 as int16_t;
        index += 1
    }
    err = construct(&mut lencode, lengths.as_mut_ptr(), 19i32);
    if err != 0i32 { return -4i32 }
    index = 0i32;
    while index < nlen + ndist {
        let mut symbol: int32_t = 0;
        let mut len: int32_t = 0;
        symbol = decode(s, &mut lencode);
        if symbol < 16i32 {
            let fresh0 = index;
            index = index + 1;
            lengths[fresh0 as usize] = symbol as int16_t
        } else {
            len = 0i32;
            if symbol == 16i32 {
                if index == 0i32 { return -5i32 }
                len = lengths[(index - 1i32) as usize] as int32_t;
                symbol = 3i32 + bits(s, 2i32)
            } else if symbol == 17i32 {
                symbol = 3i32 + bits(s, 3i32)
            } else { symbol = 11i32 + bits(s, 7i32) }
            if index + symbol > nlen + ndist { return -6i32 }
            loop  {
                let fresh1 = symbol;
                symbol = symbol - 1;
                if !(0 != fresh1) { break ; }
                let fresh2 = index;
                index = index + 1;
                lengths[fresh2 as usize] = len as int16_t
            }
        }
    }
    err = construct(&mut lencode, lengths.as_mut_ptr(), nlen);
    if err < 0i32 ||
           err > 0i32 &&
               nlen - *lencode.count.offset(0isize) as libc::c_int != 1i32 {
        return -7i32
    }
    err =
        construct(&mut distcode, lengths.as_mut_ptr().offset(nlen as isize),
                  ndist);
    if err < 0i32 ||
           err > 0i32 &&
               ndist - *distcode.count.offset(0isize) as libc::c_int != 1i32 {
        return -8i32
    }
    return codes(s, &mut lencode, &mut distcode);
}
/*
 * Decode literal/length and distance codes until an end-of-block code.
 *
 * Format notes:
 *
 * - Compressed data that is after the block type if fixed or after the code
 *   description if dynamic is a combination of literals and length/distance
 *   pairs terminated by and end-of-block code.  Literals are simply Huffman
 *   coded bytes.  A length/distance pair is a coded length followed by a
 *   coded distance to represent a string that occurs earlier in the
 *   uncompressed data that occurs again at the current location.
 *
 * - Literals, lengths, and the end-of-block code are combined into a single
 *   code of up to 286 symbols.  They are 256 literals (0..255), 29 length
 *   symbols (257..285), and the end-of-block symbol (256).
 *
 * - There are 256 possible lengths (3..258), and so 29 symbols are not enough
 *   to represent all of those.  Lengths 3..10 and 258 are in fact represented
 *   by just a length symbol.  Lengths 11..257 are represented as a symbol and
 *   some number of extra bits that are added as an integer to the base length
 *   of the length symbol.  The number of extra bits is determined by the base
 *   length symbol.  These are in the static arrays below, lens[] for the base
 *   lengths and lext[] for the corresponding number of extra bits.
 *
 * - The reason that 258 gets its own symbol is that the longest length is used
 *   often in highly redundant files.  Note that 258 can also be coded as the
 *   base value 227 plus the maximum extra value of 31.  While a good deflate
 *   should never do this, it is not an error, and should be decoded properly.
 *
 * - If a length is decoded, including its extra bits if any, then it is
 *   followed a distance code.  There are up to 30 distance symbols.  Again
 *   there are many more possible distances (1..32768), so extra bits are added
 *   to a base value represented by the symbol.  The distances 1..4 get their
 *   own symbol, but the rest require extra bits.  The base distances and
 *   corresponding number of extra bits are below in the static arrays dist[]
 *   and dext[].
 *
 * - Literal bytes are simply written to the output.  A length/distance pair is
 *   an instruction to copy previously uncompressed bytes to the output.  The
 *   copy is from distance bytes back in the output stream, copying for length
 *   bytes.
 *
 * - Distances pointing before the beginning of the output data are not
 *   permitted.
 *
 * - Overlapped copies, where the length is greater than the distance, are
 *   allowed and common.  For example, a distance of one and a length of 258
 *   simply copies the last byte 258 times.  A distance of four and a length of
 *   twelve copies the last four bytes three times.  A simple forward copy
 *   ignoring whether the length is greater than the distance or not implements
 *   this correctly.  You should not use memcpy() since its behavior is not
 *   defined for overlapped arrays.  You should not use memmove() or bcopy()
 *   since though their behavior -is- defined for overlapping arrays, it is
 *   defined to do the wrong thing in this case.
 */
unsafe extern "C" fn codes(mut s: *mut state, mut lencode: *mut huffman,
                           mut distcode: *mut huffman) -> int32_t {
    /* decoded symbol */
    let mut symbol: int32_t = 0;
    /* length for copy */
    let mut len: int32_t = 0;
    /* distance for copy */
    let mut dist: uint32_t = 0;
    /* Size base for length codes 257..285 */
    static mut lens: [int16_t; 29] =
        [3i32 as int16_t, 4i32 as int16_t, 5i32 as int16_t, 6i32 as int16_t,
         7i32 as int16_t, 8i32 as int16_t, 9i32 as int16_t, 10i32 as int16_t,
         11i32 as int16_t, 13i32 as int16_t, 15i32 as int16_t,
         17i32 as int16_t, 19i32 as int16_t, 23i32 as int16_t,
         27i32 as int16_t, 31i32 as int16_t, 35i32 as int16_t,
         43i32 as int16_t, 51i32 as int16_t, 59i32 as int16_t,
         67i32 as int16_t, 83i32 as int16_t, 99i32 as int16_t,
         115i32 as int16_t, 131i32 as int16_t, 163i32 as int16_t,
         195i32 as int16_t, 227i32 as int16_t, 258i32 as int16_t];
    /* Extra bits for length codes 257..285 */
    static mut lext: [int16_t; 29] =
        [0i32 as int16_t, 0i32 as int16_t, 0i32 as int16_t, 0i32 as int16_t,
         0i32 as int16_t, 0i32 as int16_t, 0i32 as int16_t, 0i32 as int16_t,
         1i32 as int16_t, 1i32 as int16_t, 1i32 as int16_t, 1i32 as int16_t,
         2i32 as int16_t, 2i32 as int16_t, 2i32 as int16_t, 2i32 as int16_t,
         3i32 as int16_t, 3i32 as int16_t, 3i32 as int16_t, 3i32 as int16_t,
         4i32 as int16_t, 4i32 as int16_t, 4i32 as int16_t, 4i32 as int16_t,
         5i32 as int16_t, 5i32 as int16_t, 5i32 as int16_t, 5i32 as int16_t,
         0i32 as int16_t];
    /* Offset base for distance codes 0..29 */
    static mut dists: [int16_t; 30] =
        [1i32 as int16_t, 2i32 as int16_t, 3i32 as int16_t, 4i32 as int16_t,
         5i32 as int16_t, 7i32 as int16_t, 9i32 as int16_t, 13i32 as int16_t,
         17i32 as int16_t, 25i32 as int16_t, 33i32 as int16_t,
         49i32 as int16_t, 65i32 as int16_t, 97i32 as int16_t,
         129i32 as int16_t, 193i32 as int16_t, 257i32 as int16_t,
         385i32 as int16_t, 513i32 as int16_t, 769i32 as int16_t,
         1025i32 as int16_t, 1537i32 as int16_t, 2049i32 as int16_t,
         3073i32 as int16_t, 4097i32 as int16_t, 6145i32 as int16_t,
         8193i32 as int16_t, 12289i32 as int16_t, 16385i32 as int16_t,
         24577i32 as int16_t];
    /* Extra bits for distance codes 0..29 */
    static mut dext: [int16_t; 30] =
        [0i32 as int16_t, 0i32 as int16_t, 0i32 as int16_t, 0i32 as int16_t,
         1i32 as int16_t, 1i32 as int16_t, 2i32 as int16_t, 2i32 as int16_t,
         3i32 as int16_t, 3i32 as int16_t, 4i32 as int16_t, 4i32 as int16_t,
         5i32 as int16_t, 5i32 as int16_t, 6i32 as int16_t, 6i32 as int16_t,
         7i32 as int16_t, 7i32 as int16_t, 8i32 as int16_t, 8i32 as int16_t,
         9i32 as int16_t, 9i32 as int16_t, 10i32 as int16_t, 10i32 as int16_t,
         11i32 as int16_t, 11i32 as int16_t, 12i32 as int16_t,
         12i32 as int16_t, 13i32 as int16_t, 13i32 as int16_t];
    loop  {
        symbol = decode(s, lencode);
        if symbol < 0i32 { return symbol }
        if symbol < 256i32 {
            if !(*s).out.is_null() {
                if (*s).outcnt == (*s).outlen { return 1i32 }
                *(*s).out.offset((*s).outcnt as isize) = symbol as uint8_t
            }
            (*s).outcnt = (*s).outcnt.wrapping_add(1)
        } else if symbol > 256i32 {
            symbol -= 257i32;
            if symbol >= 29i32 { return -9i32 }
            len =
                lens[symbol as usize] as libc::c_int +
                    bits(s, lext[symbol as usize] as int32_t);
            symbol = decode(s, distcode);
            if symbol < 0i32 { return symbol }
            dist =
                (dists[symbol as usize] as libc::c_int +
                     bits(s, dext[symbol as usize] as int32_t)) as uint32_t;
            if dist > (*s).outcnt { return -10i32 }
            if !(*s).out.is_null() {
                if (*s).outcnt.wrapping_add(len as libc::c_uint) > (*s).outlen
                   {
                    return 1i32
                }
                loop  {
                    let fresh3 = len;
                    len = len - 1;
                    if !(0 != fresh3) { break ; }
                    *(*s).out.offset((*s).outcnt as isize) =
                        *(*s).out.offset((*s).outcnt.wrapping_sub(dist) as
                                             isize);
                    (*s).outcnt = (*s).outcnt.wrapping_add(1)
                }
            } else {
                (*s).outcnt =
                    ((*s).outcnt as
                         libc::c_uint).wrapping_add(len as libc::c_uint) as
                        uint32_t as uint32_t
            }
        }
        if !(symbol != 256i32) { break ; }
    }
    return 0i32;
}
/*
 * Return need bits from the input stream.  This always leaves less than
 * eight bits in the buffer.  bits() works properly for need == 0.
 *
 * Format notes:
 *
 * - Bits are stored in bytes from the least significant bit to the most
 *   significant bit.  Therefore bits are dropped from the bottom of the bit
 *   buffer, using shift right, and new bytes are appended to the top of the
 *   bit buffer, using shift left.
 */
unsafe extern "C" fn bits(mut s: *mut state, mut need: int32_t) -> int32_t {
    /* bit accumulator (can use up to 20 bits) */
    let mut val: int32_t = 0;
    val = (*s).bitbuf;
    while (*s).bitcnt < need {
        if (*s).incnt == (*s).inlen { longjmp((*s).env.as_mut_ptr(), 1i32); }
        let fresh4 = (*s).incnt;
        (*s).incnt = (*s).incnt.wrapping_add(1);
        val |= (*(*s).in_0.offset(fresh4 as isize) as int32_t) << (*s).bitcnt;
        (*s).bitcnt += 8i32
    }
    (*s).bitbuf = val >> need;
    (*s).bitcnt -= need;
    return (val as libc::c_long & (1i64 << need) - 1i32 as libc::c_long) as
               int32_t;
}
/*
 * Decode a code from the stream s using huffman table h.  Return the symbol or
 * a negative value if there is an error.  If all of the lengths are zero, i.e.
 * an empty code, or if the code is incomplete and an invalid code is received,
 * then -9 is returned after reading MAXBITS bits.
 *
 * Format notes:
 *
 * - The codes as stored in the compressed data are bit-reversed relative to
 *   a simple integer ordering of codes of the same lengths.  Hence below the
 *   bits are pulled from the compressed data one at a time and used to
 *   build the code value reversed from what is in the stream in order to
 *   permit simple integer comparisons for decoding.  A table-based decoding
 *   scheme (as used in zlib) does not need to do this reversal.
 *
 * - The first code for the shortest length is all zeros.  Subsequent codes of
 *   the same length are simply integer increments of the previous code.  When
 *   moving up a length, a zero bit is appended to the code.  For a complete
 *   code, the last code of the longest length will be all ones.
 *
 * - Incomplete codes are handled by this decoder, since they are permitted
 *   in the deflate format.  See the format notes for fixed() and dynamic().
 */
unsafe extern "C" fn decode(mut s: *mut state, mut h: *mut huffman)
 -> int32_t {
    /* current number of bits in code */
    let mut len: int32_t = 0;
    /* len bits being decoded */
    let mut code: int32_t = 0;
    /* first code of length len */
    let mut first: int32_t = 0;
    /* number of codes of length len */
    let mut count: int32_t = 0;
    /* index of first code of length len in symbol table */
    let mut index: int32_t = 0;
    /* bits from stream */
    let mut bitbuf: int32_t = 0;
    /* bits left in next or left to process */
    let mut left: int32_t = 0;
    /* next number of codes */
    let mut next: *mut int16_t = 0 as *mut int16_t;
    bitbuf = (*s).bitbuf;
    left = (*s).bitcnt;
    index = 0i32;
    first = index;
    code = first;
    len = 1i32;
    next = (*h).count.offset(1isize);
    loop  {
        loop  {
            let fresh5 = left;
            left = left - 1;
            if !(0 != fresh5) { break ; }
            code |= bitbuf & 1i32;
            bitbuf >>= 1i32;
            let fresh6 = next;
            next = next.offset(1);
            count = *fresh6 as int32_t;
            if code < first + count {
                (*s).bitbuf = bitbuf;
                (*s).bitcnt = (*s).bitcnt - len & 7i32;
                return *(*h).symbol.offset((index + (code - first)) as isize)
                           as int32_t
            }
            index += count;
            first += count;
            first <<= 1i32;
            code <<= 1i32;
            len += 1
        }
        left = 15i32 + 1i32 - len;
        if left == 0i32 { break ; }
        if (*s).incnt == (*s).inlen { longjmp((*s).env.as_mut_ptr(), 1i32); }
        let fresh7 = (*s).incnt;
        (*s).incnt = (*s).incnt.wrapping_add(1);
        bitbuf = *(*s).in_0.offset(fresh7 as isize) as int32_t;
        if left > 8i32 { left = 8i32 }
    }
    return -9i32;
}
/*
 * Given the list of code lengths length[0..n-1] representing a canonical
 * Huffman code for n symbols, construct the tables required to decode those
 * codes.  Those tables are the number of codes of each length, and the symbols
 * sorted by length, retaining their original order within each length.  The
 * return value is zero for a complete code set, negative for an over-
 * subscribed code set, and positive for an incomplete code set.  The tables
 * can be used if the return value is zero or positive, but they cannot be used
 * if the return value is negative.  If the return value is zero, it is not
 * possible for decode() using that table to return an error--any stream of
 * enough bits will resolve to a symbol.  If the return value is positive, then
 * it is possible for decode() using that table to return an error for received
 * codes past the end of the incomplete lengths.
 *
 * Not used by decode(), but used for error checking, h->count[0] is the number
 * of the n symbols not in the code.  So n - h->count[0] is the number of
 * codes.  This is useful for checking for incomplete codes that have more than
 * one symbol, which is an error in a dynamic block.
 *
 * Assumption: for all i in 0..n-1, 0 <= length[i] <= MAXBITS
 * This is assured by the construction of the length arrays in dynamic() and
 * fixed() and is not verified by construct().
 *
 * Format notes:
 *
 * - Permitted and expected examples of incomplete codes are one of the fixed
 *   codes and any code with a single symbol which in deflate is coded as one
 *   bit instead of zero bits.  See the format notes for fixed() and dynamic().
 *
 * - Within a given code length, the symbols are kept in ascending order for
 *   the code bits definition.
 */
unsafe extern "C" fn construct(mut h: *mut huffman, mut length: *mut int16_t,
                               mut n: int32_t) -> int32_t {
    /* current symbol when stepping through length[] */
    let mut symbol: int32_t = 0;
    /* current length when stepping through h->count[] */
    let mut len: int32_t = 0;
    /* number of possible codes left of current length */
    let mut left: int32_t = 0;
    /* offsets in symbol table for each length */
    let mut offs: [int16_t; 16] = [0; 16];
    len = 0i32;
    while len <= 15i32 {
        *(*h).count.offset(len as isize) = 0i32 as int16_t;
        len += 1
    }
    symbol = 0i32;
    while symbol < n {
        let ref mut fresh8 =
            *(*h).count.offset(*length.offset(symbol as isize) as isize);
        *fresh8 += 1;
        symbol += 1
    }
    if *(*h).count.offset(0isize) as libc::c_int == n { return 0i32 }
    left = 1i32;
    len = 1i32;
    while len <= 15i32 {
        left <<= 1i32;
        left -= *(*h).count.offset(len as isize) as libc::c_int;
        if left < 0i32 { return left }
        len += 1
    }
    offs[1usize] = 0i32 as int16_t;
    len = 1i32;
    while len < 15i32 {
        offs[(len + 1i32) as usize] =
            (offs[len as usize] as libc::c_int +
                 *(*h).count.offset(len as isize) as libc::c_int) as int16_t;
        len += 1
    }
    symbol = 0i32;
    while symbol < n {
        if *length.offset(symbol as isize) as libc::c_int != 0i32 {
            let fresh9 = offs[*length.offset(symbol as isize) as usize];
            offs[*length.offset(symbol as isize) as usize] =
                offs[*length.offset(symbol as isize) as usize] + 1;
            *(*h).symbol.offset(fresh9 as isize) = symbol as int16_t
        }
        symbol += 1
    }
    return left;
}
/*
 * Process a fixed codes block.
 *
 * Format notes:
 *
 * - This block type can be useful for compressing small amounts of data for
 *   which the size of the code descriptions in a dynamic block exceeds the
 *   benefit of custom codes for that block.  For fixed codes, no bits are
 *   spent on code descriptions.  Instead the code lengths for literal/length
 *   codes and distance codes are fixed.  The specific lengths for each symbol
 *   can be seen in the "for" loops below.
 *
 * - The literal/length code is complete, but has two symbols that are invalid
 *   and should result in an error if received.  This cannot be implemented
 *   simply as an incomplete code since those two symbols are in the "middle"
 *   of the code.  They are eight bits long and the longest literal/length\
 *   code is nine bits.  Therefore the code must be constructed with those
 *   symbols, and the invalid symbols must be detected after decoding.
 *
 * - The fixed distance codes also have two invalid symbols that should result
 *   in an error if received.  Since all of the distance codes are the same
 *   length, this can be implemented as an incomplete code.  Then the invalid
 *   codes are detected while decoding.
 */
unsafe extern "C" fn fixed(mut s: *mut state) -> int32_t {
    static mut virgin: int32_t = 1i32;
    static mut lencnt: [int16_t; 16] = [0; 16];
    static mut lensym: [int16_t; 288] = [0; 288];
    static mut distcnt: [int16_t; 16] = [0; 16];
    static mut distsym: [int16_t; 30] = [0; 30];
    static mut lencode: huffman =
        unsafe {
            huffman{count: lencnt.as_ptr() as *mut _,
                    symbol: lensym.as_ptr() as *mut _,}
        };
    static mut distcode: huffman =
        unsafe {
            huffman{count: distcnt.as_ptr() as *mut _,
                    symbol: distsym.as_ptr() as *mut _,}
        };
    if 0 != virgin {
        let mut symbol: int32_t = 0;
        let mut lengths: [int16_t; 288] = [0; 288];
        symbol = 0i32;
        while symbol < 144i32 {
            lengths[symbol as usize] = 8i32 as int16_t;
            symbol += 1
        }
        while symbol < 256i32 {
            lengths[symbol as usize] = 9i32 as int16_t;
            symbol += 1
        }
        while symbol < 280i32 {
            lengths[symbol as usize] = 7i32 as int16_t;
            symbol += 1
        }
        while symbol < 288i32 {
            lengths[symbol as usize] = 8i32 as int16_t;
            symbol += 1
        }
        construct(&mut lencode, lengths.as_mut_ptr(), 288i32);
        symbol = 0i32;
        while symbol < 30i32 {
            lengths[symbol as usize] = 5i32 as int16_t;
            symbol += 1
        }
        construct(&mut distcode, lengths.as_mut_ptr(), 30i32);
        virgin = 0i32
    }
    return codes(s, &mut lencode, &mut distcode);
}
/*
 * Process a stored block.
 *
 * Format notes:
 *
 * - After the two-bit stored block type (00), the stored block length and
 *   stored bytes are byte-aligned for fast copying.  Therefore any leftover
 *   bits in the byte that has the last bit of the type, as many as seven, are
 *   discarded.  The value of the discarded bits are not defined and should not
 *   be checked against any expectation.
 *
 * - The second inverted copy of the stored block length does not have to be
 *   checked, but it's probably a good idea to do so anyway.
 *
 * - A stored block can have zero length.  This is sometimes used to byte-align
 *   subsets of the compressed data for random access or partial recovery.
 */
unsafe extern "C" fn stored(mut s: *mut state) -> int32_t {
    /* length of stored block */
    let mut len: uint32_t = 0;
    (*s).bitbuf = 0i32;
    (*s).bitcnt = 0i32;
    if (*s).incnt.wrapping_add(4i32 as libc::c_uint) > (*s).inlen {
        return 2i32
    }
    let fresh10 = (*s).incnt;
    (*s).incnt = (*s).incnt.wrapping_add(1);
    len = *(*s).in_0.offset(fresh10 as isize) as uint32_t;
    let fresh11 = (*s).incnt;
    (*s).incnt = (*s).incnt.wrapping_add(1);
    len |=
        ((*(*s).in_0.offset(fresh11 as isize) as libc::c_int) << 8i32) as
            libc::c_uint;
    let fresh12 = (*s).incnt;
    (*s).incnt = (*s).incnt.wrapping_add(1);
    if *(*s).in_0.offset(fresh12 as isize) as libc::c_uint !=
           !len & 0xffi32 as libc::c_uint ||
           {
               let fresh13 = (*s).incnt;
               (*s).incnt = (*s).incnt.wrapping_add(1);
               *(*s).in_0.offset(fresh13 as isize) as libc::c_uint !=
                   !len >> 8i32 & 0xffi32 as libc::c_uint
           } {
        return -2i32
    }
    if (*s).incnt.wrapping_add(len) > (*s).inlen { return 2i32 }
    if !(*s).out.is_null() {
        if (*s).outcnt.wrapping_add(len) > (*s).outlen { return 1i32 }
        loop  {
            let fresh14 = len;
            len = len.wrapping_sub(1);
            if !(0 != fresh14) { break ; }
            let fresh16 = (*s).outcnt;
            (*s).outcnt = (*s).outcnt.wrapping_add(1);
            let fresh15 = (*s).incnt;
            (*s).incnt = (*s).incnt.wrapping_add(1);
            *(*s).out.offset(fresh16 as isize) =
                *(*s).in_0.offset(fresh15 as isize)
        }
    } else {
        (*s).outcnt =
            ((*s).outcnt as libc::c_uint).wrapping_add(len) as uint32_t as
                uint32_t;
        (*s).incnt =
            ((*s).incnt as libc::c_uint).wrapping_add(len) as uint32_t as
                uint32_t
    }
    return 0i32;
}