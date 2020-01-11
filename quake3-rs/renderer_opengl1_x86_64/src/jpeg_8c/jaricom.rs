use ::libc;

/* JPEG_INTERNAL_OPTIONS */

/* FAST_FLOAT should be either float or double, whichever is done faster
 * by your compiler.  (Note that this type is only used in the floating point
 * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
 * Typically, float is faster in ANSI C compilers, while double is faster in
 * pre-ANSI compilers (because they insist on converting to double anyway).
 * The code below therefore chooses float if we have ANSI-style prototypes.
 */

/* type for fastest integer multiply */

/* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
 * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
 * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
 */

/* If your compiler supports inline functions, define INLINE
 * as the inline keyword; otherwise define it as empty.
 */

/* Definitions for speed-related optimizations. */

/* JSAMPLEs per RGB scanline element */

/* Offset of Blue */

/* Offset of Green */

/* Offset of Red in an RGB scanline element */

/*
 * Ordering of RGB data in scanlines passed to or from the application.
 * If your application wants to deal with data in the order B,G,R, just
 * change these macros.  You can also deal with formats such as R,G,B,X
 * (one extra byte per pixel) by changing RGB_PIXELSIZE.  Note that changing
 * the offsets will also change the order in which colormap data is organized.
 * RESTRICTIONS:
 * 1. The sample applications cjpeg,djpeg do NOT support modified RGB formats.
 * 2. These macros only affect RGB<=>YCbCr color conversion, so they are not
 *    useful if you are using JPEG color spaces other than YCbCr or grayscale.
 * 3. The color quantizer modules will not behave desirably if RGB_PIXELSIZE
 *    is not 3 (they don't understand about dummy color components!).  So you
 *    can't use color quantization if you change that value.
 */

/* more capability options later, no doubt */

/* 2-pass color quantization? */

/* 1-pass color quantization? */

/* Fast path for sloppy upsampling? */

/* Output rescaling at upsample stage? */

/* Block smoothing? (Progressive only) */

/* jpeg_save_markers() needed? */

/* Output rescaling via IDCT? */

/* Progressive JPEG? (Requires MULTISCAN)*/

/* Multiple-scan JPEG files? */

/* Arithmetic coding back end? */

/* Decoder capability options: */

/* Input image smoothing option? */

/* Note: if you selected 12-bit data precision, it is dangerous to turn off
 * ENTROPY_OPT_SUPPORTED.  The standard Huffman tables are only good for 8-bit
 * precision, so jchuff.c normally uses entropy optimization to compute
 * usable tables for higher precision.  If you don't want to do optimization,
 * you'll have to supply different default Huffman tables.
 * The exact same statements apply for progressive JPEG: the default tables
 * don't work for progressive mode.  (This may get fixed, however.)
 */

/* Optimization of entropy coding parms? */

/* Input rescaling via DCT? (Requires DCT_ISLOW)*/

/* Progressive JPEG? (Requires MULTISCAN)*/

/* Multiple-scan JPEG files? */

/* Arithmetic coding back end? */

/* Encoder capability options: */

/* floating-point: accurate, fast on fast HW */

/* faster, less accurate integer method */

/* slow but accurate integer algorithm */

/* Capability options common to encoder and decoder: */

/*
 * These defines indicate whether to include various optional functions.
 * Undefining some of these symbols will produce a smaller but less capable
 * library.  Note that you can leave certain source files out of the
 * compilation/linking process if you've #undef'd the corresponding symbols.
 * (You may HAVE to do that if your compiler doesn't like null source files.)
 */

/*
 * The remaining options affect code selection within the JPEG library,
 * but they don't need to be visible to most applications using the library.
 * To minimize application namespace pollution, the symbols won't be
 * defined unless JPEG_INTERNALS or JPEG_INTERNAL_OPTIONS has been defined.
 */

/* values of boolean */

/* in case these macros already exist */
pub use crate::jmorecfg_h::INT32;
/*
 * jaricom.c
 *
 * Developed 1997-2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains probability estimation tables for common use in
 * arithmetic entropy encoding and decoding routines.
 *
 * This data represents Table D.2 in the JPEG spec (ISO/IEC IS 10918-1
 * and CCITT Recommendation ITU-T T.81) and Table 24 in the JBIG spec
 * (ISO/IEC IS 11544 and CCITT Recommendation ITU-T T.82).
 */
/* The following #define specifies the packing of the four components
 * into the compact INT32 representation.
 * Note that this formula must match the actual arithmetic encoder
 * and decoder implementation.  The implementation has to be changed
 * if this formula is changed.
 * The current organization is leaned on Markus Kuhn's JBIG
 * implementation (jbig_tab.c).
 */
#[no_mangle]

pub static mut jpeg_aritab: [crate::jmorecfg_h::INT32; 114] = [
    (0x5a1d) << 16 | (1) << 8 | (1) << 7 | 1,
    (0x2586) << 16 | (2) << 8 | (0) << 7 | 14,
    (0x1114) << 16 | (3) << 8 | (0) << 7 | 16,
    (0x80b) << 16 | (4) << 8 | (0) << 7 | 18,
    (0x3d8) << 16 | (5) << 8 | (0) << 7 | 20,
    (0x1da) << 16 | (6) << 8 | (0) << 7 | 23,
    (0xe5) << 16 | (7) << 8 | (0) << 7 | 25,
    (0x6f) << 16 | (8) << 8 | (0) << 7 | 28,
    (0x36) << 16 | (9) << 8 | (0) << 7 | 30,
    (0x1a) << 16 | (10) << 8 | (0) << 7 | 33,
    (0xd) << 16 | (11) << 8 | (0) << 7 | 35,
    (0x6) << 16 | (12) << 8 | (0) << 7 | 9,
    (0x3) << 16 | (13) << 8 | (0) << 7 | 10,
    (0x1) << 16 | (13) << 8 | (0) << 7 | 12,
    (0x5a7f) << 16 | (15) << 8 | (1) << 7 | 15,
    (0x3f25) << 16 | (16) << 8 | (0) << 7 | 36,
    (0x2cf2) << 16 | (17) << 8 | (0) << 7 | 38,
    (0x207c) << 16 | (18) << 8 | (0) << 7 | 39,
    (0x17b9) << 16 | (19) << 8 | (0) << 7 | 40,
    (0x1182) << 16 | (20) << 8 | (0) << 7 | 42,
    (0xcef) << 16 | (21) << 8 | (0) << 7 | 43,
    (0x9a1) << 16 | (22) << 8 | (0) << 7 | 45,
    (0x72f) << 16 | (23) << 8 | (0) << 7 | 46,
    (0x55c) << 16 | (24) << 8 | (0) << 7 | 48,
    (0x406) << 16 | (25) << 8 | (0) << 7 | 49,
    (0x303) << 16 | (26) << 8 | (0) << 7 | 51,
    (0x240) << 16 | (27) << 8 | (0) << 7 | 52,
    (0x1b1) << 16 | (28) << 8 | (0) << 7 | 54,
    (0x144) << 16 | (29) << 8 | (0) << 7 | 56,
    (0xf5) << 16 | (30) << 8 | (0) << 7 | 57,
    (0xb7) << 16 | (31) << 8 | (0) << 7 | 59,
    (0x8a) << 16 | (32) << 8 | (0) << 7 | 60,
    (0x68) << 16 | (33) << 8 | (0) << 7 | 62,
    (0x4e) << 16 | (34) << 8 | (0) << 7 | 63,
    (0x3b) << 16 | (35) << 8 | (0) << 7 | 32,
    (0x2c) << 16 | (9) << 8 | (0) << 7 | 33,
    (0x5ae1) << 16 | (37) << 8 | (1) << 7 | 37,
    (0x484c) << 16 | (38) << 8 | (0) << 7 | 64,
    (0x3a0d) << 16 | (39) << 8 | (0) << 7 | 65,
    (0x2ef1) << 16 | (40) << 8 | (0) << 7 | 67,
    (0x261f) << 16 | (41) << 8 | (0) << 7 | 68,
    (0x1f33) << 16 | (42) << 8 | (0) << 7 | 69,
    (0x19a8) << 16 | (43) << 8 | (0) << 7 | 70,
    (0x1518) << 16 | (44) << 8 | (0) << 7 | 72,
    (0x1177) << 16 | (45) << 8 | (0) << 7 | 73,
    (0xe74) << 16 | (46) << 8 | (0) << 7 | 74,
    (0xbfb) << 16 | (47) << 8 | (0) << 7 | 75,
    (0x9f8) << 16 | (48) << 8 | (0) << 7 | 77,
    (0x861) << 16 | (49) << 8 | (0) << 7 | 78,
    (0x706) << 16 | (50) << 8 | (0) << 7 | 79,
    (0x5cd) << 16 | (51) << 8 | (0) << 7 | 48,
    (0x4de) << 16 | (52) << 8 | (0) << 7 | 50,
    (0x40f) << 16 | (53) << 8 | (0) << 7 | 50,
    (0x363) << 16 | (54) << 8 | (0) << 7 | 51,
    (0x2d4) << 16 | (55) << 8 | (0) << 7 | 52,
    (0x25c) << 16 | (56) << 8 | (0) << 7 | 53,
    (0x1f8) << 16 | (57) << 8 | (0) << 7 | 54,
    (0x1a4) << 16 | (58) << 8 | (0) << 7 | 55,
    (0x160) << 16 | (59) << 8 | (0) << 7 | 56,
    (0x125) << 16 | (60) << 8 | (0) << 7 | 57,
    (0xf6) << 16 | (61) << 8 | (0) << 7 | 58,
    (0xcb) << 16 | (62) << 8 | (0) << 7 | 59,
    (0xab) << 16 | (63) << 8 | (0) << 7 | 61,
    (0x8f) << 16 | (32) << 8 | (0) << 7 | 61,
    (0x5b12) << 16 | (65) << 8 | (1) << 7 | 65,
    (0x4d04) << 16 | (66) << 8 | (0) << 7 | 80,
    (0x412c) << 16 | (67) << 8 | (0) << 7 | 81,
    (0x37d8) << 16 | (68) << 8 | (0) << 7 | 82,
    (0x2fe8) << 16 | (69) << 8 | (0) << 7 | 83,
    (0x293c) << 16 | (70) << 8 | (0) << 7 | 84,
    (0x2379) << 16 | (71) << 8 | (0) << 7 | 86,
    (0x1edf) << 16 | (72) << 8 | (0) << 7 | 87,
    (0x1aa9) << 16 | (73) << 8 | (0) << 7 | 87,
    (0x174e) << 16 | (74) << 8 | (0) << 7 | 72,
    (0x1424) << 16 | (75) << 8 | (0) << 7 | 72,
    (0x119c) << 16 | (76) << 8 | (0) << 7 | 74,
    (0xf6b) << 16 | (77) << 8 | (0) << 7 | 74,
    (0xd51) << 16 | (78) << 8 | (0) << 7 | 75,
    (0xbb6) << 16 | (79) << 8 | (0) << 7 | 77,
    (0xa40) << 16 | (48) << 8 | (0) << 7 | 77,
    (0x5832) << 16 | (81) << 8 | (1) << 7 | 80,
    (0x4d1c) << 16 | (82) << 8 | (0) << 7 | 88,
    (0x438e) << 16 | (83) << 8 | (0) << 7 | 89,
    (0x3bdd) << 16 | (84) << 8 | (0) << 7 | 90,
    (0x34ee) << 16 | (85) << 8 | (0) << 7 | 91,
    (0x2eae) << 16 | (86) << 8 | (0) << 7 | 92,
    (0x299a) << 16 | (87) << 8 | (0) << 7 | 93,
    (0x2516) << 16 | (71) << 8 | (0) << 7 | 86,
    (0x5570) << 16 | (89) << 8 | (1) << 7 | 88,
    (0x4ca9) << 16 | (90) << 8 | (0) << 7 | 95,
    (0x44d9) << 16 | (91) << 8 | (0) << 7 | 96,
    (0x3e22) << 16 | (92) << 8 | (0) << 7 | 97,
    (0x3824) << 16 | (93) << 8 | (0) << 7 | 99,
    (0x32b4) << 16 | (94) << 8 | (0) << 7 | 99,
    (0x2e17) << 16 | (86) << 8 | (0) << 7 | 93,
    (0x56a8) << 16 | (96) << 8 | (1) << 7 | 95,
    (0x4f46) << 16 | (97) << 8 | (0) << 7 | 101,
    (0x47e5) << 16 | (98) << 8 | (0) << 7 | 102,
    (0x41cf) << 16 | (99) << 8 | (0) << 7 | 103,
    (0x3c3d) << 16 | (100) << 8 | (0) << 7 | 104,
    (0x375e) << 16 | (93) << 8 | (0) << 7 | 99,
    (0x5231) << 16 | (102) << 8 | (0) << 7 | 105,
    (0x4c0f) << 16 | (103) << 8 | (0) << 7 | 106,
    (0x4639) << 16 | (104) << 8 | (0) << 7 | 107,
    (0x415e) << 16 | (99) << 8 | (0) << 7 | 103,
    (0x5627) << 16 | (106) << 8 | (1) << 7 | 105,
    (0x50e7) << 16 | (107) << 8 | (0) << 7 | 108,
    (0x4b85) << 16 | (103) << 8 | (0) << 7 | 109,
    (0x5597) << 16 | (109) << 8 | (0) << 7 | 110,
    (0x504f) << 16 | (107) << 8 | (0) << 7 | 111,
    (0x5a10) << 16 | (111) << 8 | (1) << 7 | 110,
    (0x5522) << 16 | (109) << 8 | (0) << 7 | 112,
    (0x59eb) << 16 | (111) << 8 | (1) << 7 | 112,
    (0x5a1d) << 16 | (113) << 8 | (0) << 7 | 113,
];
