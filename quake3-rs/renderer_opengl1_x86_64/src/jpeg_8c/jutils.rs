use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;

/*
 * jutils.c
 *
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * Modified 2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains tables and miscellaneous utility routines needed
 * for both compression and decompression.
 * Note we prefix all global names with "j" to minimize conflicts with
 * a surrounding application.
 */
/*
 * jpeg_zigzag_order[i] is the zigzag-order position of the i'th element
 * of a DCT block read in natural order (left to right, top to bottom).
 */
/* This table is not actually needed in v6a */
/*
 * jpeg_natural_order[i] is the natural-order position of the i'th element
 * of zigzag order.
 *
 * When reading corrupted data, the Huffman decoders could attempt
 * to reference an entry beyond the end of this array (if the decoded
 * zero run length reaches past the end of the block).  To prevent
 * wild stores without adding an inner-loop test, we put some extra
 * "63"s after the real entries.  This will cause the extra coefficient
 * to be stored in location 63 of the block, not somewhere random.
 * The worst case would be a run-length of 15, which means we need 16
 * fake entries.
 */
#[no_mangle]

pub static mut jpeg_natural_order: [i32; 80] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63,
];
#[no_mangle]

pub static mut jpeg_natural_order7: [i32; 65] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 14, 21, 28, 35, 42, 49, 50, 43, 36, 29, 22, 30, 37, 44, 51, 52, 45, 38, 46, 53, 54, 63,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
];
#[no_mangle]

pub static mut jpeg_natural_order6: [i32; 52] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 41, 34, 27, 20, 13,
    21, 28, 35, 42, 43, 36, 29, 37, 44, 45, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63,
];
#[no_mangle]

pub static mut jpeg_natural_order5: [i32; 41] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 12, 19, 26, 33, 34, 27, 20, 28, 35, 36,
    63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
];
#[no_mangle]

pub static mut jpeg_natural_order4: [i32; 32] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 25, 18, 11, 19, 26, 27, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    63, 63, 63, 63, 63, 63, 63,
];
#[no_mangle]

pub static mut jpeg_natural_order3: [i32; 25] = [
    0, 1, 8, 16, 9, 2, 10, 17, 18, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
];
#[no_mangle]

pub static mut jpeg_natural_order2: [i32; 20] = [
    0, 1, 8, 9, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
];
/* Utility routines in jutils.c */
/*
 * Arithmetic utilities
 */
#[no_mangle]

pub unsafe extern "C" fn jdiv_round_up(mut a: isize, mut b: isize) -> isize
/* Compute a/b rounded up to next integer, ie, ceil(a/b) */
/* Assumes a >= 0, b > 0 */ {
    return (a + b - 1isize) / b;
}
#[no_mangle]

pub unsafe extern "C" fn jround_up(mut a: isize, mut b: isize) -> isize
/* Compute a rounded up to next multiple of b, ie, ceil(a/b)*b */
/* Assumes a >= 0, b > 0 */ {
    a += b - 1isize;
    return a - a % b;
}
/* On normal machines we can apply MEMCOPY() and MEMZERO() to sample arrays
 * and coefficient-block arrays.  This won't work on 80x86 because the arrays
 * are FAR and we're assuming a small-pointer memory model.  However, some
 * DOS compilers provide far-pointer versions of memcpy() and memset() even
 * in the small-model libraries.  These will be used if USE_FMEM is defined.
 * Otherwise, the routines below do it the hard way.  (The performance cost
 * is not all that great, because these routines aren't very heavily used.)
 */
/* normal case, same as regular macros */
/* 80x86 case, define if we can */
#[no_mangle]

pub unsafe extern "C" fn jcopy_sample_rows(
    mut input_array: crate::jpeglib_h::JSAMPARRAY,
    mut source_row: i32,
    mut output_array: crate::jpeglib_h::JSAMPARRAY,
    mut dest_row: i32,
    mut num_rows: i32,
    mut num_cols: crate::jmorecfg_h::JDIMENSION,
)
/* Copy some rows of samples from one place to another.
 * num_rows rows are copied from input_array[source_row++]
 * to output_array[dest_row++]; these areas may overlap for duplication.
 * The source and destination arrays must be at least as wide as num_cols.
 */
{
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut count: crate::stddef_h::size_t =
        (num_cols as usize).wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>());
    let mut row: i32 = 0;
    input_array = input_array.offset(source_row as isize);
    output_array = output_array.offset(dest_row as isize);
    row = num_rows;
    while row > 0 {
        let fresh0 = input_array;
        input_array = input_array.offset(1);
        inptr = *fresh0;
        let fresh1 = output_array;
        output_array = output_array.offset(1);
        outptr = *fresh1;
        crate::stdlib::memcpy(
            outptr as *mut libc::c_void,
            inptr as *const libc::c_void,
            count,
        );
        row -= 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn jcopy_block_row(
    mut input_row: crate::jpeglib_h::JBLOCKROW,
    mut output_row: crate::jpeglib_h::JBLOCKROW,
    mut num_blocks: crate::jmorecfg_h::JDIMENSION,
)
/* Copy a row of coefficient blocks from one place to another. */
{
    crate::stdlib::memcpy(
        output_row as *mut libc::c_void,
        input_row as *const libc::c_void,
        (num_blocks as usize).wrapping_mul(
            (64usize).wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JCOEF>()),
        ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn jzero_far(
    mut target: *mut libc::c_void,
    mut bytestozero: crate::stddef_h::size_t,
)
/* Zero out a chunk of FAR memory. */
/* This might be sample-array data, block-array data, or alloc_large data. */
{
    crate::stdlib::memset(target, 0, bytestozero);
}
