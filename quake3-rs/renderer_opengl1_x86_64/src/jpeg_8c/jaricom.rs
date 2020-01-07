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
    (0x5a1d as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 1 as libc::c_int as libc::c_long,
    (0x2586 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (2 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 14 as libc::c_int as libc::c_long,
    (0x1114 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (3 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 16 as libc::c_int as libc::c_long,
    (0x80b as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (4 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 18 as libc::c_int as libc::c_long,
    (0x3d8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (5 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 20 as libc::c_int as libc::c_long,
    (0x1da as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (6 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 23 as libc::c_int as libc::c_long,
    (0xe5 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (7 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 25 as libc::c_int as libc::c_long,
    (0x6f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (8 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 28 as libc::c_int as libc::c_long,
    (0x36 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (9 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 30 as libc::c_int as libc::c_long,
    (0x1a as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (10 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 33 as libc::c_int as libc::c_long,
    (0xd as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (11 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 35 as libc::c_int as libc::c_long,
    (0x6 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (12 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 9 as libc::c_int as libc::c_long,
    (0x3 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (13 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 10 as libc::c_int as libc::c_long,
    (0x1 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (13 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 12 as libc::c_int as libc::c_long,
    (0x5a7f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (15 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 15 as libc::c_int as libc::c_long,
    (0x3f25 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (16 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 36 as libc::c_int as libc::c_long,
    (0x2cf2 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (17 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 38 as libc::c_int as libc::c_long,
    (0x207c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (18 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 39 as libc::c_int as libc::c_long,
    (0x17b9 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (19 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 40 as libc::c_int as libc::c_long,
    (0x1182 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (20 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 42 as libc::c_int as libc::c_long,
    (0xcef as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (21 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 43 as libc::c_int as libc::c_long,
    (0x9a1 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (22 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 45 as libc::c_int as libc::c_long,
    (0x72f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (23 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 46 as libc::c_int as libc::c_long,
    (0x55c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (24 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 48 as libc::c_int as libc::c_long,
    (0x406 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (25 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 49 as libc::c_int as libc::c_long,
    (0x303 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (26 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 51 as libc::c_int as libc::c_long,
    (0x240 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (27 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 52 as libc::c_int as libc::c_long,
    (0x1b1 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (28 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 54 as libc::c_int as libc::c_long,
    (0x144 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (29 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 56 as libc::c_int as libc::c_long,
    (0xf5 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (30 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 57 as libc::c_int as libc::c_long,
    (0xb7 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (31 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 59 as libc::c_int as libc::c_long,
    (0x8a as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (32 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 60 as libc::c_int as libc::c_long,
    (0x68 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (33 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 62 as libc::c_int as libc::c_long,
    (0x4e as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (34 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 63 as libc::c_int as libc::c_long,
    (0x3b as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (35 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 32 as libc::c_int as libc::c_long,
    (0x2c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (9 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 33 as libc::c_int as libc::c_long,
    (0x5ae1 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (37 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 37 as libc::c_int as libc::c_long,
    (0x484c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (38 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 64 as libc::c_int as libc::c_long,
    (0x3a0d as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (39 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 65 as libc::c_int as libc::c_long,
    (0x2ef1 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (40 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 67 as libc::c_int as libc::c_long,
    (0x261f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (41 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 68 as libc::c_int as libc::c_long,
    (0x1f33 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (42 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 69 as libc::c_int as libc::c_long,
    (0x19a8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (43 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 70 as libc::c_int as libc::c_long,
    (0x1518 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (44 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 72 as libc::c_int as libc::c_long,
    (0x1177 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (45 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 73 as libc::c_int as libc::c_long,
    (0xe74 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (46 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 74 as libc::c_int as libc::c_long,
    (0xbfb as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (47 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 75 as libc::c_int as libc::c_long,
    (0x9f8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (48 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 77 as libc::c_int as libc::c_long,
    (0x861 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (49 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 78 as libc::c_int as libc::c_long,
    (0x706 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (50 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 79 as libc::c_int as libc::c_long,
    (0x5cd as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (51 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 48 as libc::c_int as libc::c_long,
    (0x4de as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (52 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 50 as libc::c_int as libc::c_long,
    (0x40f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (53 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 50 as libc::c_int as libc::c_long,
    (0x363 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (54 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 51 as libc::c_int as libc::c_long,
    (0x2d4 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (55 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 52 as libc::c_int as libc::c_long,
    (0x25c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (56 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 53 as libc::c_int as libc::c_long,
    (0x1f8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (57 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 54 as libc::c_int as libc::c_long,
    (0x1a4 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (58 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 55 as libc::c_int as libc::c_long,
    (0x160 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (59 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 56 as libc::c_int as libc::c_long,
    (0x125 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (60 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 57 as libc::c_int as libc::c_long,
    (0xf6 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (61 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 58 as libc::c_int as libc::c_long,
    (0xcb as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (62 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 59 as libc::c_int as libc::c_long,
    (0xab as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (63 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 61 as libc::c_int as libc::c_long,
    (0x8f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (32 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 61 as libc::c_int as libc::c_long,
    (0x5b12 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (65 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 65 as libc::c_int as libc::c_long,
    (0x4d04 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (66 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 80 as libc::c_int as libc::c_long,
    (0x412c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (67 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 81 as libc::c_int as libc::c_long,
    (0x37d8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (68 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 82 as libc::c_int as libc::c_long,
    (0x2fe8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (69 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 83 as libc::c_int as libc::c_long,
    (0x293c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (70 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 84 as libc::c_int as libc::c_long,
    (0x2379 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (71 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 86 as libc::c_int as libc::c_long,
    (0x1edf as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (72 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 87 as libc::c_int as libc::c_long,
    (0x1aa9 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (73 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 87 as libc::c_int as libc::c_long,
    (0x174e as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (74 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 72 as libc::c_int as libc::c_long,
    (0x1424 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (75 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 72 as libc::c_int as libc::c_long,
    (0x119c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (76 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 74 as libc::c_int as libc::c_long,
    (0xf6b as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (77 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 74 as libc::c_int as libc::c_long,
    (0xd51 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (78 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 75 as libc::c_int as libc::c_long,
    (0xbb6 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (79 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 77 as libc::c_int as libc::c_long,
    (0xa40 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (48 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 77 as libc::c_int as libc::c_long,
    (0x5832 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (81 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 80 as libc::c_int as libc::c_long,
    (0x4d1c as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (82 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 88 as libc::c_int as libc::c_long,
    (0x438e as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (83 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 89 as libc::c_int as libc::c_long,
    (0x3bdd as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (84 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 90 as libc::c_int as libc::c_long,
    (0x34ee as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (85 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 91 as libc::c_int as libc::c_long,
    (0x2eae as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (86 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 92 as libc::c_int as libc::c_long,
    (0x299a as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (87 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 93 as libc::c_int as libc::c_long,
    (0x2516 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (71 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 86 as libc::c_int as libc::c_long,
    (0x5570 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (89 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 88 as libc::c_int as libc::c_long,
    (0x4ca9 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (90 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 95 as libc::c_int as libc::c_long,
    (0x44d9 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (91 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 96 as libc::c_int as libc::c_long,
    (0x3e22 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (92 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 97 as libc::c_int as libc::c_long,
    (0x3824 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (93 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 99 as libc::c_int as libc::c_long,
    (0x32b4 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (94 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 99 as libc::c_int as libc::c_long,
    (0x2e17 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (86 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 93 as libc::c_int as libc::c_long,
    (0x56a8 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (96 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 95 as libc::c_int as libc::c_long,
    (0x4f46 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (97 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 101 as libc::c_int as libc::c_long,
    (0x47e5 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (98 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 102 as libc::c_int as libc::c_long,
    (0x41cf as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (99 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 103 as libc::c_int as libc::c_long,
    (0x3c3d as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (100 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 104 as libc::c_int as libc::c_long,
    (0x375e as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (93 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 99 as libc::c_int as libc::c_long,
    (0x5231 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (102 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 105 as libc::c_int as libc::c_long,
    (0x4c0f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (103 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 106 as libc::c_int as libc::c_long,
    (0x4639 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (104 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 107 as libc::c_int as libc::c_long,
    (0x415e as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (99 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 103 as libc::c_int as libc::c_long,
    (0x5627 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (106 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 105 as libc::c_int as libc::c_long,
    (0x50e7 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (107 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 108 as libc::c_int as libc::c_long,
    (0x4b85 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (103 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 109 as libc::c_int as libc::c_long,
    (0x5597 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (109 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 110 as libc::c_int as libc::c_long,
    (0x504f as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (107 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 111 as libc::c_int as libc::c_long,
    (0x5a10 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (111 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 110 as libc::c_int as libc::c_long,
    (0x5522 as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (109 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 112 as libc::c_int as libc::c_long,
    (0x59eb as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (111 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (1 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 112 as libc::c_int as libc::c_long,
    (0x5a1d as libc::c_int as crate::jmorecfg_h::INT32) << 16 as libc::c_int
        | (113 as libc::c_int as crate::jmorecfg_h::INT32) << 8 as libc::c_int
        | (0 as libc::c_int as crate::jmorecfg_h::INT32) << 7 as libc::c_int
        | 113 as libc::c_int as libc::c_long,
];
