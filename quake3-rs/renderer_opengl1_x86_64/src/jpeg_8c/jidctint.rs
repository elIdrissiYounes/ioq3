pub use crate::stddef_h::size_t;

pub use crate::jdct_h::ISLOW_MULT_TYPE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT32;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(0.298631336) */
/* FIX(0.390180644) */
/* FIX(0.541196100) */
/* FIX(0.765366865) */
/* FIX(0.899976223) */
/* FIX(1.175875602) */
/* FIX(1.501321110) */
/* FIX(1.847759065) */
/* FIX(1.961570560) */
/* FIX(2.053119869) */
/* FIX(2.562915447) */
/* FIX(3.072711026) */
/* Multiply an INT32 variable by an INT32 constant to yield an INT32 result.
 * For 8-bit samples with the recommended scaling, all the variable
 * and constant values involved are no more than 16 bits wide, so a
 * 16x16->32 bit multiply can be used instead of a full 32x32 multiply.
 * For 12-bit samples, a full 32-bit multiplication will be needed.
 */
/* Dequantize a coefficient by multiplying it by the multiplier-table
 * entry; produce an int result.  In this module, both inputs and result
 * are 16 bits or less, so either int or short multiply will work.
 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_islow(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 64] = [0; 64];
    /* Pass 1: process columns from input, store into work array. */
    /* Note results are scaled up by sqrt(8) compared to a true IDCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 8;
    while ctr > 0 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((8i32 * 1) as isize) as i32 == 0
            && *inptr.offset((8i32 * 2) as isize) as i32 == 0
            && *inptr.offset((8i32 * 3) as isize) as i32 == 0
            && *inptr.offset((8i32 * 4) as isize) as i32 == 0
            && *inptr.offset((8i32 * 5) as isize) as i32 == 0
            && *inptr.offset((8i32 * 6) as isize) as i32 == 0
            && *inptr.offset((8i32 * 7) as isize) as i32 == 0
        {
            /* AC terms all zero */
            let mut dcval: i32 = *inptr.offset((8i32 * 0) as isize)
                as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 0) as isize)
                << 2; /* advance pointers to next column */
            *wsptr.offset((8i32 * 0) as isize) = dcval;
            *wsptr.offset((8i32 * 1) as isize) = dcval;
            *wsptr.offset((8i32 * 2) as isize) = dcval;
            *wsptr.offset((8i32 * 3) as isize) = dcval;
            *wsptr.offset((8i32 * 4) as isize) = dcval;
            *wsptr.offset((8i32 * 5) as isize) = dcval;
            *wsptr.offset((8i32 * 6) as isize) = dcval;
            *wsptr.offset((8i32 * 7) as isize) = dcval;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        } else {
            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */
            z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 2) as isize))
                as crate::jmorecfg_h::INT32;
            z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 6) as isize))
                as crate::jmorecfg_h::INT32;
            z1 = (z2 + z3) * 4433;
            tmp2 = z1 + z2 * 6270;
            tmp3 = z1 - z3 * 15137;
            z2 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 0) as isize))
                as crate::jmorecfg_h::INT32;
            z3 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 4) as isize))
                as crate::jmorecfg_h::INT32;
            z2 <<= 13isize;
            z3 <<= 13isize;
            /* Add fudge factor here for final descale. */
            z2 += (1) << 13 - 2 - 1;
            tmp0 = z2 + z3;
            tmp1 = z2 - z3;
            tmp10 = tmp0 + tmp2;
            tmp13 = tmp0 - tmp2;
            tmp11 = tmp1 + tmp3;
            tmp12 = tmp1 - tmp3;
            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */
            tmp0 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 7) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * c3 */
            tmp1 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 5) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (-c3-c5) */
            tmp2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 3) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (c5-c3) */
            tmp3 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 1) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (c7-c3) */
            z2 = tmp0 + tmp2; /* sqrt(2) * (-c1+c3+c5-c7) */
            z3 = tmp1 + tmp3; /* sqrt(2) * ( c1+c3-c5-c7) */
            z1 = (z2 + z3) * 9633; /* sqrt(2) * (-c1-c3) */
            z2 = z2 * -(16069); /* sqrt(2) * ( c1+c3-c5+c7) */
            z3 = z3 * -(3196); /* sqrt(2) * ( c1+c3+c5-c7) */
            z2 += z1;
            z3 += z1;
            z1 = (tmp0 + tmp3) * -(7373);
            tmp0 = tmp0 * 2446;
            tmp3 = tmp3 * 12299;
            tmp0 += z1 + z2;
            tmp3 += z1 + z3;
            z1 = (tmp1 + tmp2) * -(20995);
            tmp1 = tmp1 * 16819;
            tmp2 = tmp2 * 25172;
            tmp1 += z1 + z3;
            tmp2 += z1 + z2;
            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
            *wsptr.offset((8i32 * 0) as isize) = (tmp10 + tmp3 >> 13 - 2) as i32; /* advance pointers to next column */
            *wsptr.offset((8i32 * 7) as isize) = (tmp10 - tmp3 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 1) as isize) = (tmp11 + tmp2 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 6) as isize) = (tmp11 - tmp2 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 2) as isize) = (tmp12 + tmp1 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 5) as isize) = (tmp12 - tmp1 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 3) as isize) = (tmp13 + tmp0 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 4) as isize) = (tmp13 - tmp0 >> 13 - 2) as i32;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        }
        ctr -= 1
    }
    /* Pass 2: process rows from work array, store into output array. */
    /* Note that we must descale the results by a factor of 8 == 2**3, */
    /* and also undo the PASS1_BITS scaling. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * On machines with very fast multiplication, it's possible that the
         * test takes more time than it's worth.  In that case this section
         * may be commented out.
         */
        if *wsptr.offset(1) == 0
            && *wsptr.offset(2) == 0
            && *wsptr.offset(3) == 0
            && *wsptr.offset(4) == 0
            && *wsptr.offset(5) == 0
            && *wsptr.offset(6) == 0
            && *wsptr.offset(7) == 0
        {
            /* AC terms all zero */
            let mut dcval_0: crate::jmorecfg_h::JSAMPLE = *range_limit.offset(
                ((*wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 3 - 1) >> 2 + 3)
                    as i32
                    & 255 * 4 + 3) as isize,
            ); /* advance pointer to next row */
            *outptr.offset(0) = dcval_0;
            *outptr.offset(1) = dcval_0;
            *outptr.offset(2) = dcval_0;
            *outptr.offset(3) = dcval_0;
            *outptr.offset(4) = dcval_0;
            *outptr.offset(5) = dcval_0;
            *outptr.offset(6) = dcval_0;
            *outptr.offset(7) = dcval_0;
            wsptr = wsptr.offset(8)
        } else {
            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */
            z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
            z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
            z1 = (z2 + z3) * 4433;
            tmp2 = z1 + z2 * 6270;
            tmp3 = z1 - z3 * 15137;
            /* Add fudge factor here for final descale. */
            z2 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
            z3 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
            tmp0 = z2 + z3 << 13;
            tmp1 = z2 - z3 << 13;
            tmp10 = tmp0 + tmp2;
            tmp13 = tmp0 - tmp2;
            tmp11 = tmp1 + tmp3;
            tmp12 = tmp1 - tmp3;
            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */
            tmp0 = *wsptr.offset(7) as crate::jmorecfg_h::INT32; /* sqrt(2) * c3 */
            tmp1 = *wsptr.offset(5) as crate::jmorecfg_h::INT32; /* sqrt(2) * (-c3-c5) */
            tmp2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32; /* sqrt(2) * (c5-c3) */
            tmp3 = *wsptr.offset(1) as crate::jmorecfg_h::INT32; /* sqrt(2) * (c7-c3) */
            z2 = tmp0 + tmp2; /* sqrt(2) * (-c1+c3+c5-c7) */
            z3 = tmp1 + tmp3; /* sqrt(2) * ( c1+c3-c5-c7) */
            z1 = (z2 + z3) * 9633; /* sqrt(2) * (-c1-c3) */
            z2 = z2 * -(16069); /* sqrt(2) * ( c1+c3-c5+c7) */
            z3 = z3 * -(3196); /* sqrt(2) * ( c1+c3+c5-c7) */
            z2 += z1;
            z3 += z1;
            z1 = (tmp0 + tmp3) * -(7373);
            tmp0 = tmp0 * 2446;
            tmp3 = tmp3 * 12299;
            tmp0 += z1 + z2;
            tmp3 += z1 + z3;
            z1 = (tmp1 + tmp2) * -(20995);
            tmp1 = tmp1 * 16819;
            tmp2 = tmp2 * 25172;
            tmp1 += z1 + z3;
            tmp2 += z1 + z2;
            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
            *outptr.offset(0) =
                *range_limit.offset(((tmp10 + tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(7) =
                *range_limit.offset(((tmp10 - tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(1) =
                *range_limit.offset(((tmp11 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(6) =
                *range_limit.offset(((tmp11 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(2) =
                *range_limit.offset(((tmp12 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(5) =
                *range_limit.offset(((tmp12 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(3) =
                *range_limit.offset(((tmp13 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            *outptr.offset(4) =
                *range_limit.offset(((tmp13 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
            wsptr = wsptr.offset(8)
        }
        ctr += 1
    }
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 7x7 output block.
 *
 * Optimized algorithm with 12 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/14).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_7x7(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 49] = [0; 49];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 7 {
        /* Even part */
        tmp13 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp13 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp13 += (1) << 13 - 2 - 1; /* c4 */
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z2 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2+c4-c6 */
        z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        tmp10 =
            (z2 - z3) * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2-c4-c6 */
        tmp12 =
            (z1 - z2) * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2+c4+c6 */
        tmp11 = tmp10 + tmp12 + tmp13
            - z2 * (1.841218003 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c0 */
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = tmp0 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp13;
        tmp10 +=
            tmp0 - z3 * (0.077722536 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            tmp0 - z1 * (2.470602249 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 += z2 * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* (c3+c1-c5)/2 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* (c3+c5-c1)/2 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* -c1 */
        tmp1 =
            (z1 + z2) * (0.935414347 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5 */
        tmp2 =
            (z1 - z2) * (0.170262339 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c1-c5 */
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (z2 + z3)
            * -((1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp1 += tmp2;
        z2 = (z1 + z3) * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 += z2;
        tmp2 += z2 + z3 * (1.870828693 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((7i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 6) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 1) as isize) = (tmp11 + tmp1 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 5) as isize) = (tmp11 - tmp1 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 2) as isize) = (tmp12 + tmp2 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 4) as isize) = (tmp12 - tmp2 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 3) as isize) = (tmp13 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 7 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 7 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp13 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp13 <<= 13isize;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        tmp10 =
            (z2 - z3) * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 =
            (z1 - z2) * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp10 + tmp12 + tmp13
            - z2 * (1.841218003 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = z1 + z3;
        z2 -= tmp0;
        tmp0 = tmp0 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp13;
        tmp10 +=
            tmp0 - z3 * (0.077722536 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            tmp0 - z1 * (2.470602249 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 += z2 * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp1 =
            (z1 + z2) * (0.935414347 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 =
            (z1 - z2) * (0.170262339 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (z2 + z3)
            * -((1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp1 += tmp2;
        z2 = (z1 + z3) * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 += z2;
        tmp2 += z2 + z3 * (1.870828693 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp11 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(7);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c6 */
/* c2+c4-c6 */
/* c2 */
/* c2-c4-c6 */
/* c2+c4+c6 */
/* c0 */
/* Odd part */
/* (c3+c1-c5)/2 */
/* (c3+c5-c1)/2 */
/* -c1 */
/* c5 */
/* c3+c1-c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 6x6 output block.
 *
 * Optimized algorithm with 3 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/12).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_6x6(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 36] = [0; 36];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 6 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += (1) << 13 - 2 - 1; /* c4 */
        tmp2 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        tmp10 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10 >> 13 - 2;
        tmp10 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 = tmp10 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32;
        tmp1 =
            (z1 + z3) * (0.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + (z1 + z2 << 13);
        tmp2 = tmp1 + (z3 - z2 << 13);
        tmp1 = z1 - z2 - z3 << 2;
        /* Final output stage */
        *wsptr.offset((6i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 5) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 1) as isize) = (tmp11 + tmp1) as i32;
        *wsptr.offset((6i32 * 4) as isize) = (tmp11 - tmp1) as i32;
        *wsptr.offset((6i32 * 2) as isize) = (tmp12 + tmp2 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 3) as isize) = (tmp12 - tmp2 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 6 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 6 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        tmp2 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp10 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10;
        tmp10 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp0 = tmp10 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp1 =
            (z1 + z3) * (0.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + (z1 + z2 << 13);
        tmp2 = tmp1 + (z3 - z2 << 13);
        tmp1 = z1 - z2 - z3 << 13;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp11 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(6);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 5x5 output block.
 *
 * Optimized algorithm with 5 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/10).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_5x5(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 25] = [0; 25];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 5 {
        /* Even part */
        tmp12 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp12 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp12 += (1) << 13 - 2 - 1; /* (c2+c4)/2 */
        tmp0 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* (c2-c4)/2 */
        tmp1 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32;
        z1 = (tmp0 + tmp1)
            * (0.790569415 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = (tmp0 - tmp1)
            * (0.353553391 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= z2 << 2;
        /* Odd part */
        z2 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z3 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c1-c3 */
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c3 */
        tmp0 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((5i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 4) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 1) as isize) = (tmp11 + tmp1 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 3) as isize) = (tmp11 - tmp1 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 2) as isize) = (tmp12 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 5 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 5 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp12 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp12 <<= 13isize;
        tmp0 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp1 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z1 = (tmp0 + tmp1)
            * (0.790569415 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = (tmp0 - tmp1)
            * (0.353553391 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= z2 << 2;
        z2 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp11 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(5);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* (c2+c4)/2 */
/* (c2-c4)/2 */
/* Odd part */
/* c3 */
/* c1-c3 */
/* c1+c3 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 4x4 output block.
 *
 * Optimized algorithm with 3 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/16) [refers to 8-point IDCT].
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_4x4(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 16] = [0; 16];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 4 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2 << 2;
        tmp12 = tmp0 - tmp2 << 2;
        /* Odd part */
        /* Same rotation as in the even part of the 8x8 LL&M IDCT */
        z2 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z3 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * 4433;
        /* Add fudge factor here for final descale. */
        z1 += (1) << 13 - 2 - 1;
        tmp0 = z1 + z2 * 6270 >> 13 - 2;
        tmp2 = z1 - z3 * 15137 >> 13 - 2;
        /* Final output stage */
        *wsptr.offset((4i32 * 0) as isize) = (tmp10 + tmp0) as i32;
        *wsptr.offset((4i32 * 3) as isize) = (tmp10 - tmp0) as i32;
        *wsptr.offset((4i32 * 1) as isize) = (tmp12 + tmp2) as i32;
        *wsptr.offset((4i32 * 2) as isize) = (tmp12 - tmp2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 4 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 4 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2 << 13;
        tmp12 = tmp0 - tmp2 << 13;
        z2 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * 4433;
        tmp0 = z1 + z2 * 6270;
        tmp2 = z1 - z3 * 15137;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(4);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* Odd part */
/* Same rotation as in the even part of the 8x8 LL&M IDCT */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 3x3 output block.
 *
 * Optimized algorithm with 2 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/6).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_3x3(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 9] = [0; 9];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 3 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += (1) << 13 - 2 - 1; /* c2 */
        tmp2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp12 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        /* Odd part */
        tmp12 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c1 */
        tmp0 = tmp12 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((3i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((3i32 * 2) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((3i32 * 1) as isize) = (tmp2 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 3 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 3 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        tmp2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp12 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        tmp12 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        tmp0 = tmp12 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(3);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c2 */
/* Odd part */
/* c1 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 2x2 output block.
 *
 * Multiplication-less algorithm.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_2x2(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    /* Pass 1: process columns from input. */
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    /* Column 0 */
    tmp4 = (*coef_block.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
        * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
    tmp5 = (*coef_block.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
        * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32;
    /* Add fudge factor here for final descale. */
    tmp4 += (1) << 2;
    tmp0 = tmp4 + tmp5;
    tmp2 = tmp4 - tmp5;
    /* Column 1 */
    tmp4 = (*coef_block.offset((8i32 * 0 + 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
        * *quantptr.offset((8i32 * 0 + 1) as isize)) as crate::jmorecfg_h::INT32;
    tmp5 = (*coef_block.offset((8i32 * 1 + 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
        * *quantptr.offset((8i32 * 1 + 1) as isize)) as crate::jmorecfg_h::INT32;
    tmp1 = tmp4 + tmp5;
    tmp3 = tmp4 - tmp5;
    /* Pass 2: process 2 rows, store into output array. */
    /* Row 0 */
    outptr = (*output_buf.offset(0)).offset(output_col as isize);
    *outptr.offset(0) = *range_limit.offset(((tmp0 + tmp1 >> 3) as i32 & 255 * 4 + 3) as isize);
    *outptr.offset(1) = *range_limit.offset(((tmp0 - tmp1 >> 3) as i32 & 255 * 4 + 3) as isize);
    /* Row 1 */
    outptr = (*output_buf.offset(1)).offset(output_col as isize);
    *outptr.offset(0) = *range_limit.offset(((tmp2 + tmp3 >> 3) as i32 & 255 * 4 + 3) as isize);
    *outptr.offset(1) = *range_limit.offset(((tmp2 - tmp3 >> 3) as i32 & 255 * 4 + 3) as isize);
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 1x1 output block.
 *
 * We hardly need an inverse DCT routine for this: just take the
 * average pixel value, which is one-eighth of the DC coefficient.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_1x1(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut dcval: i32 = 0;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    /* 1x1 is trivial: just take the DC coefficient divided by 8. */
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    dcval = *coef_block.offset(0) as crate::jdct_h::ISLOW_MULT_TYPE * *quantptr.offset(0);
    dcval = (dcval as crate::jmorecfg_h::INT32 + ((1) << 3 - 1) >> 3) as i32;
    *(*output_buf.offset(0)).offset(output_col as isize) =
        *range_limit.offset((dcval & 255 * 4 + 3) as isize);
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 9x9 output block.
 *
 * Optimized algorithm with 10 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/18).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_9x9(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 72] = [0; 72];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += (1) << 13 - 2 - 1; /* c6 */
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z2 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32; /* c4 */
        tmp3 = z3 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c8 */
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;
        tmp0 =
            (z1 - z2) * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;
        tmp0 =
            (z1 + z2) * (1.328926049 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 = z1 * (1.083350441 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 = z2 * (0.245575608 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* -c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c7 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c1 */
        z2 = z2 * -((1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c3 */
        tmp2 =
            (z1 + z3) * (0.909038955 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 =
            (z1 + z4) * (0.483689525 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 =
            (z3 - z4) * (1.392728481 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = (z1 - z3 - z4)
            * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp11 + tmp1 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp11 - tmp1 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp12 + tmp2 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp12 - tmp2 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp13 + tmp3 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp13 - tmp3 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp14 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 9 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 9 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        tmp3 = z3 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = tmp0 + tmp3;
        tmp2 = tmp0 - tmp3 - tmp3;
        tmp0 =
            (z1 - z2) * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp2 + tmp0;
        tmp14 = tmp2 - tmp0 - tmp0;
        tmp0 =
            (z1 + z2) * (1.328926049 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 = z1 * (1.083350441 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 = z2 * (0.245575608 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp1 + tmp0 - tmp3;
        tmp12 = tmp1 - tmp0 + tmp2;
        tmp13 = tmp1 - tmp2 + tmp3;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        z2 = z2 * -((1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp2 =
            (z1 + z3) * (0.909038955 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 =
            (z1 + z4) * (0.483689525 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp2 + tmp3 - z2;
        tmp1 =
            (z3 - z4) * (1.392728481 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 += z2 - tmp1;
        tmp3 += z2 + tmp1;
        tmp1 = (z1 - z3 - z4)
            * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp11 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp13 + tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp13 - tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c6 */
/* c6 */
/* c2 */
/* c4 */
/* c8 */
/* Odd part */
/* -c3 */
/* c5 */
/* c7 */
/* c1 */
/* c3 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 10x10 output block.
 *
 * Optimized algorithm with 12 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/20).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_10x10(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut z5: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 80] = [0; 80];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        z3 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z3 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z3 += (1) << 13 - 2 - 1; /* c4 */
        z4 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c8 */
        z1 = z4 * (1.144122806 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c6 */
        z2 = z4 * (0.437016024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2-c6 */
        tmp10 = z3 + z1; /* c2+c6 */
        tmp11 = z3 - z2;
        tmp22 = z3 - (z1 - z2 << 1) >> 13 - 2;
        z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* (c3-c7)/2 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* (c3+c7)/2 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c1 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        tmp11 = z2 + z4; /* (c1-c9)/2 */
        tmp13 = z2 - z4; /* c3 */
        tmp12 = tmp13 * (0.309016994 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c7 */
        z5 = z3 << 13;
        z2 = tmp11 * (0.951056516 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z5 + tmp12;
        tmp10 = z1 * (1.396802247 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z2
            + z4;
        tmp14 = z1 * (0.221231742 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        z2 = tmp11 * (0.587785252 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z5 - tmp12 - (tmp13 << 13 - 1);
        tmp12 = z1 - tmp13 - z3 << 2;
        tmp11 = z1 * (1.260073511 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2
            - z4;
        tmp13 = z1 * (0.642039522 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp22 - tmp12) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 10 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 10 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z3 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z3 <<= 13isize;
        z4 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z1 = z4 * (1.144122806 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z4 * (0.437016024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;
        tmp22 = z3 - (z1 - z2 << 1);
        z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z3 <<= 13isize;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z2 + z4;
        tmp13 = z2 - z4;
        tmp12 = tmp13 * (0.309016994 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = tmp11 * (0.951056516 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z3 + tmp12;
        tmp10 = z1 * (1.396802247 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z2
            + z4;
        tmp14 = z1 * (0.221231742 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        z2 = tmp11 * (0.587785252 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z3 - tmp12 - (tmp13 << 13 - 1);
        tmp12 = (z1 - tmp13 << 13) - z3;
        tmp11 = z1 * (1.260073511 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2
            - z4;
        tmp13 = z1 * (0.642039522 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c8 */
/* c0 = (c4-c8)*2 */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* Odd part */
/* (c3-c7)/2 */
/* (c3+c7)/2 */
/* c1 */
/* c9 */
/* (c1-c9)/2 */
/* c3 */
/* c7 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 11x11 output block.
 *
 * Optimized algorithm with 24 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/22).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_11x11(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 88] = [0; 88];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp10 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp10 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp10 += (1) << 13 - 2 - 1; /* c2+c4 */
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* c2-c6 */
        z2 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* -(c2-c10) */
        z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        tmp20 =
            (z2 - z3) * (2.546640132 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2+c4+c10-c6 */
        tmp23 =
            (z2 - z1) * (0.430815045 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c4+c6 */
        z4 = z1 + z3; /* c6+c8 */
        tmp24 = z4 * -((1.155664402 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c8+c10 */
        z4 -= z2; /* c4+c10 */
        tmp25 =
            tmp10 + z4 * (1.356927976 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c0 */
        tmp21 = tmp20 + tmp23 + tmp25
            - z2 * (1.821790775 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 +=
            tmp25 + z3 * (2.115825087 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp23 +=
            tmp25 - z1 * (1.513598477 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp24 += tmp25;
        tmp22 =
            tmp24 - z3 * (0.788749120 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp24 += z2 * (1.944413522 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z1 * (1.390975730 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp25 =
            tmp10 - z4 * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c3-c9 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c5-c9 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c7-c9 */
        tmp11 = z1 + z2; /* c7+c5+c3-c1-2*c9 */
        tmp14 = (tmp11 + z3 + z4)
            * (0.398430003 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c7+c9 */
        tmp11 = tmp11 * (0.887983902 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c7+3*c9-c3 */
        tmp12 =
            (z1 + z3) * (0.670361295 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c5-c7-c9 */
        tmp13 = tmp14
            + (z1 + z4) * (0.366151574 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -(c1+c9) */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (0.923107866 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c5+c9-c7 */
        z1 = tmp14
            - (z2 + z3) * (1.163011579 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c9 */
        tmp11 +=
            z1 + z2 * (2.073276588 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            z1 - z3 * (1.192193623 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z4)
            * -((1.798248910 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp11 += z1;
        tmp13 +=
            z1 + z4 * (2.102458632 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += z2 * -((1.467221301 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            + z3 * (1.001388905 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.684843907 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 11 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 11 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp10 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp10 <<= 13isize;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        tmp20 =
            (z2 - z3) * (2.546640132 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp23 =
            (z2 - z1) * (0.430815045 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z1 + z3;
        tmp24 = z4 * -((1.155664402 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        z4 -= z2;
        tmp25 =
            tmp10 + z4 * (1.356927976 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp21 = tmp20 + tmp23 + tmp25
            - z2 * (1.821790775 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 +=
            tmp25 + z3 * (2.115825087 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp23 +=
            tmp25 - z1 * (1.513598477 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp24 += tmp25;
        tmp22 =
            tmp24 - z3 * (0.788749120 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp24 += z2 * (1.944413522 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z1 * (1.390975730 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp25 =
            tmp10 - z4 * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z1 + z2;
        tmp14 = (tmp11 + z3 + z4)
            * (0.398430003 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp11 * (0.887983902 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 =
            (z1 + z3) * (0.670361295 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = tmp14
            + (z1 + z4) * (0.366151574 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (0.923107866 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = tmp14
            - (z2 + z3) * (1.163011579 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 +=
            z1 + z2 * (2.073276588 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            z1 - z3 * (1.192193623 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z4)
            * -((1.798248910 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp11 += z1;
        tmp13 +=
            z1 + z4 * (2.102458632 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += z2 * -((1.467221301 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            + z3 * (1.001388905 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.684843907 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c2+c4 */
/* c2-c6 */
/* -(c2-c10) */
/* c2 */
/* c2+c4+c10-c6 */
/* c4+c6 */
/* c6+c8 */
/* c8+c10 */
/* c4+c10 */
/* c0 */
/* Odd part */
/* c9 */
/* c3-c9 */
/* c5-c9 */
/* c7-c9 */
/* c7+c5+c3-c1-2*c9 */
/* c7+c9 */
/* c1+c7+3*c9-c3 */
/* c3+c5-c7-c9 */
/* -(c1+c9) */
/* c1+c5+c9-c7 */
/* c3+c9 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 12x12 output block.
 *
 * Optimized algorithm with 15 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/24).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_12x12(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 96] = [0; 96];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        z3 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z3 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z3 += (1) << 13 - 2 - 1; /* c4 */
        z4 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        z4 = z4 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z4 = z1 * (1.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        z2 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z2 <<= 13isize;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* -c9 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c7 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c5-c7 */
        tmp11 = z2 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1-c5 */
        tmp14 = z2 * -(4433); /* -(c7+c11) */
        tmp10 = z1 + z3; /* c1+c5-c7-c11 */
        tmp15 = (tmp10 + z4)
            * (0.860918669 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c11 */
        tmp12 = tmp15
            + tmp10 * (0.261052384 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5+c7 */
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c9 */
        tmp13 = (z3 + z4)
            * -((1.045510580 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c3-c9 */
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c9 */
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 += tmp14
            - z1 * (0.676326758 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.982889723 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433;
        tmp11 = z3 + z1 * 6270;
        tmp14 = z3 - z2 * 15137;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 11) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 + tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp25 - tmp15 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 12 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 12 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z3 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z3 <<= 13isize;
        z4 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z4 = z4 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z4 = z1 * (1.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        z2 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z2 <<= 13isize;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z2 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z2 * -(4433);
        tmp10 = z1 + z3;
        tmp15 = (tmp10 + z4)
            * (0.860918669 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = tmp15
            + tmp10 * (0.261052384 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = (z3 + z4)
            * -((1.045510580 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 += tmp14
            - z1 * (0.676326758 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.982889723 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433;
        tmp11 = z3 + z1 * 6270;
        tmp14 = z3 - z2 * 15137;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp25 - tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c3 */
/* -c9 */
/* c7 */
/* c5-c7 */
/* c1-c5 */
/* -(c7+c11) */
/* c1+c5-c7-c11 */
/* c1+c11 */
/* c5+c7 */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 13x13 output block.
 *
 * Optimized algorithm with 29 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/26).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_13x13(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 104] = [0; 104];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z1 += (1) << 13 - 2 - 1; /* (c4+c6)/2 */
        z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* (c4-c6)/2 */
        z3 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        z4 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32; /* c10 */
        tmp10 = z3 + z4; /* (c8-c12)/2 */
        tmp11 = z3 - z4; /* (c8+c12)/2 */
        tmp12 = tmp10 * (1.155388986 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c6 */
        tmp13 =
            tmp11 * (0.096834934 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 + z1; /* c4 */
        tmp20 = z2 * (1.373119086 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp12
            + tmp13; /* (c2-c10)/2 */
        tmp22 = z2 * (0.501487041 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - tmp12
            + tmp13; /* (c2+c10)/2 */
        tmp12 = tmp10 * (0.316450131 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c12 */
        tmp13 =
            tmp11 * (0.486914739 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 + z1; /* c8 */
        tmp21 = z2 * (1.058554052 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - tmp12
            + tmp13; /* c0 */
        tmp25 = z2 * -((1.252223920 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            + tmp12
            + tmp13;
        tmp12 = tmp10 * (0.435816023 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 =
            tmp11 * (0.937303064 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z1;
        tmp23 = z2 * -((0.170464608 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            - tmp12
            - tmp13;
        tmp24 = z2 * -((0.803364869 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            + tmp12
            - tmp13;
        tmp26 = (tmp11 - z2)
            * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z1;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c7 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c7+c5+c3-c1 */
        tmp11 =
            (z1 + z2) * (1.322312651 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c11 */
        tmp12 =
            (z1 + z3) * (1.163874945 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5+c9+c11-c3 */
        tmp15 = z1 + z4; /* c1+c5-c9-c11 */
        tmp13 = tmp15 * (0.937797057 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c5 */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (2.020082300 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c5+c9-c7 */
        tmp14 = (z2 + z3)
            * -((0.338443458 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* -c9 */
        tmp11 +=
            tmp14 + z2 * (0.837223564 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c11 */
        tmp12 +=
            tmp14 - z3 * (1.572116027 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1-c7 */
        tmp14 = (z2 + z4)
            * -((1.163874945 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c7 */
        tmp11 += tmp14; /* c1+c11 */
        tmp13 +=
            tmp14 + z4 * (2.205608352 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = (z3 + z4)
            * -((0.657217813 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = tmp15 * (0.338443458 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = tmp15
            + z1 * (0.318774355 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2 * (0.466105296 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z3 - z2) * (0.937797057 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += z1;
        tmp15 += z1
            + z3 * (0.384515595 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.742345811 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 12) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 11) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 + tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp25 - tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp26 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 13 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 13 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z1 <<= 13isize;
        z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        tmp12 = tmp10 * (1.155388986 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 =
            tmp11 * (0.096834934 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 + z1;
        tmp20 = z2 * (1.373119086 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp12
            + tmp13;
        tmp22 = z2 * (0.501487041 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - tmp12
            + tmp13;
        tmp12 = tmp10 * (0.316450131 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 =
            tmp11 * (0.486914739 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 + z1;
        tmp21 = z2 * (1.058554052 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - tmp12
            + tmp13;
        tmp25 = z2 * -((1.252223920 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            + tmp12
            + tmp13;
        tmp12 = tmp10 * (0.435816023 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 =
            tmp11 * (0.937303064 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z1;
        tmp23 = z2 * -((0.170464608 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            - tmp12
            - tmp13;
        tmp24 = z2 * -((0.803364869 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            + tmp12
            - tmp13;
        tmp26 = (tmp11 - z2)
            * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z1;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 =
            (z1 + z2) * (1.322312651 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 =
            (z1 + z3) * (1.163874945 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 = z1 + z4;
        tmp13 = tmp15 * (0.937797057 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (2.020082300 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = (z2 + z3)
            * -((0.338443458 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp11 +=
            tmp14 + z2 * (0.837223564 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            tmp14 - z3 * (1.572116027 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = (z2 + z4)
            * -((1.163874945 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp11 += tmp14;
        tmp13 +=
            tmp14 + z4 * (2.205608352 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = (z3 + z4)
            * -((0.657217813 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp12 += tmp14;
        tmp13 += tmp14;
        tmp15 = tmp15 * (0.338443458 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = tmp15
            + z1 * (0.318774355 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2 * (0.466105296 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z3 - z2) * (0.937797057 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += z1;
        tmp15 += z1
            + z3 * (0.384515595 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.742345811 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(12) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp25 - tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp26 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* (c4+c6)/2 */
/* (c4-c6)/2 */
/* c2 */
/* c10 */
/* (c8-c12)/2 */
/* (c8+c12)/2 */
/* c6 */
/* c4 */
/* (c2-c10)/2 */
/* (c2+c10)/2 */
/* c12 */
/* c8 */
/* c0 */
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c7+c5+c3-c1 */
/* -c11 */
/* c5+c9+c11-c3 */
/* c1+c5-c9-c11 */
/* -c5 */
/* c3+c5+c9-c7 */
/* -c9 */
/* c11 */
/* c1-c7 */
/* c7 */
/* c1+c11 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 14x14 output block.
 *
 * Optimized algorithm with 20 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/28).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_14x14(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 112] = [0; 112];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z1 += (1) << 13 - 2 - 1; /* c4 */
        z4 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c12 */
        z2 = z4 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c8 */
        z3 = z4 * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c6 */
        z4 = z4 * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2-c6 */
        tmp10 = z1 + z2; /* c6+c10 */
        tmp11 = z1 + z3; /* c2 */
        tmp12 = z1 - z4;
        tmp23 = z1 - (z2 + z3 - z4 << 1) >> 13 - 2;
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z2 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (z1 + z2) * (1.105676686 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z3 + z1 * (0.273079590 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z3 - z2 * (1.719280954 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 = z1 * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2 * (1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c3+c5-c1 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        tmp13 = z4 << 13; /* c9+c11-c13 */
        tmp14 = z1 + z3; /* c11 */
        tmp11 =
            (z1 + z2) * (1.334852607 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c13 */
        tmp12 = tmp14 * (1.197448846 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3-c9-c13 */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (1.126980169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c5-c13 */
        tmp14 = tmp14 * (0.752406978 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1 */
        tmp16 =
            tmp14 - z1 * (1.061150426 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c9-c11 */
        z1 -= z2; /* c1+c11-c5 */
        tmp15 =
            z1 * (0.467085129 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - tmp13;
        tmp16 += tmp15;
        z1 += z4;
        z4 = (z2 + z3)
            * -((0.158341681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            - tmp13;
        tmp11 +=
            z4 - z2 * (0.424103948 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            z4 - z3 * (2.373959773 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = (z3 - z2) * (1.405321284 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += z4 + tmp13
            - z3 * (1.6906431334 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 +=
            z4 + z2 * (0.674957567 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 - z3 << 2;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 13) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 12) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 11) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp13) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp23 - tmp13) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 + tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp25 - tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp26 + tmp16 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp26 - tmp16 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 14 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 14 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z1 <<= 13isize;
        z4 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z2 = z4 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = z4 * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z4 * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;
        tmp23 = z1 - (z2 + z3 - z4 << 1);
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z3 = (z1 + z2) * (1.105676686 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z3 + z1 * (0.273079590 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z3 - z2 * (1.719280954 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 = z1 * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2 * (1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        z4 <<= 13isize;
        tmp14 = z1 + z3;
        tmp11 =
            (z1 + z2) * (1.334852607 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = tmp14 * (1.197448846 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + z4
            - z1 * (1.126980169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = tmp14 * (0.752406978 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp16 =
            tmp14 - z1 * (1.061150426 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 -= z2;
        tmp15 = z1 * (0.467085129 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z4;
        tmp16 += tmp15;
        tmp13 = (z2 + z3)
            * -((0.158341681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            - z4;
        tmp11 +=
            tmp13 - z2 * (0.424103948 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            tmp13 - z3 * (2.373959773 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 =
            (z3 - z2) * (1.405321284 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += tmp13 + z4
            - z3 * (1.6906431334 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 +=
            tmp13 + z2 * (0.674957567 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = (z1 - z3 << 13) + z4;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(13) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(12) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp25 - tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp26 + tmp16 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp26 - tmp16 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c12 */
/* c8 */
/* c0 = (c4+c12-c8)*2 */
/* c6 */
/* c2-c6 */
/* c6+c10 */
/* c2 */
/* Odd part */
/* c3 */
/* c5 */
/* c3+c5-c1 */
/* c9 */
/* c9+c11-c13 */
/* c11 */
/* -c13 */
/* c3-c9-c13 */
/* c3+c5-c13 */
/* c1 */
/* c1+c9-c11 */
/* c1+c11-c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 15x15 output block.
 *
 * Optimized algorithm with 22 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/30).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_15x15(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut tmp27: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 120] = [0; 120];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z1 += (1) << 13 - 2 - 1; /* c12 */
        z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z3 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c0 = (c6-c12)*2 */
        z4 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32; /* (c2+c4)/2 */
        tmp10 = z4 * (0.437016024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* (c2-c4)/2 */
        tmp11 = z4 * (1.144122806 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c4+c14 */
        tmp12 = z1 - tmp10; /* (c8+c14)/2 */
        tmp13 = z1 + tmp11; /* (c8-c14)/2 */
        z1 -= tmp11 - tmp10 << 1; /* (c6+c12)/2 */
        z4 = z2 - z3; /* (c6-c12)/2 */
        z3 += z2; /* c10 = c6-c12 */
        tmp10 = z3 * (1.337628990 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c0 = (c6-c12)*2 */
        tmp11 = z4 * (0.045680613 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z2 * (1.439773946 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;
        tmp10 = z3 * (0.547059574 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = z4 * (0.399234004 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;
        tmp10 = z3 * (0.790569415 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = z4 * (0.353553391 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11;
        tmp27 = z1 - tmp11 - tmp11;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        z4 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c3-c9 */
        z3 = z4 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c9 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* -c9 */
        tmp13 = z2 - z4; /* -c3 */
        tmp15 = (z1 + tmp13)
            * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1 */
        tmp11 =
            tmp15 + z1 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c7 */
        tmp14 = tmp15
            - tmp13 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1-c13 */
        tmp13 = z2 * -((0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c5 */
        tmp15 = z2 * -((1.344997024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c11 */
        z2 = z1 - z4; /* c7-c11 */
        tmp12 = z3 + z2 * (1.406466353 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c11+c13 */
        tmp10 = tmp12
            + z4 * (2.457431844 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - tmp15;
        tmp16 = tmp12
            - z1 * (1.112434820 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp13;
        tmp12 = z2 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z3;
        z2 = (z1 + z4) * (0.575212477 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 += z2
            + z1 * (0.475753014 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z3;
        tmp15 += z2
            - z4 * (0.869244010 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z3;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 14) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 13) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 12) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 11) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 + tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp25 - tmp15 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp26 + tmp16 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp26 - tmp16 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp27 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 15 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 15 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z1 <<= 13isize;
        z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        tmp10 = z4 * (0.437016024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = z4 * (1.144122806 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = z1 - tmp10;
        tmp13 = z1 + tmp11;
        z1 -= tmp11 - tmp10 << 1;
        z4 = z2 - z3;
        z3 += z2;
        tmp10 = z3 * (1.337628990 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = z4 * (0.045680613 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z2 * (1.439773946 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp13 + tmp10 + tmp11;
        tmp23 = tmp12 - tmp10 + tmp11 + z2;
        tmp10 = z3 * (0.547059574 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = z4 * (0.399234004 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp25 = tmp13 - tmp10 - tmp11;
        tmp26 = tmp12 + tmp10 - tmp11 - z2;
        tmp10 = z3 * (0.790569415 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = z4 * (0.353553391 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp21 = tmp12 + tmp10 + tmp11;
        tmp24 = tmp13 - tmp10 + tmp11;
        tmp11 += tmp11;
        tmp22 = z1 + tmp11;
        tmp27 = z1 - tmp11 - tmp11;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z3 = z4 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp13 = z2 - z4;
        tmp15 = (z1 + tmp13)
            * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 =
            tmp15 + z1 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = tmp15
            - tmp13 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z2 * -((0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp15 = z2 * -((1.344997024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        z2 = z1 - z4;
        tmp12 = z3 + z2 * (1.406466353 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp12
            + z4 * (2.457431844 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - tmp15;
        tmp16 = tmp12
            - z1 * (1.112434820 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp13;
        tmp12 = z2 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z3;
        z2 = (z1 + z4) * (0.575212477 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 += z2
            + z1 * (0.475753014 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z3;
        tmp15 += z2
            - z4 * (0.869244010 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z3;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(14) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(13) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(12) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp25 - tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp26 + tmp16 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp26 - tmp16 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp27 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c12 */
/* c6 */
/* c0 = (c6-c12)*2 */
/* (c2+c4)/2 */
/* (c2-c4)/2 */
/* c4+c14 */
/* (c8+c14)/2 */
/* (c8-c14)/2 */
/* (c6+c12)/2 */
/* (c6-c12)/2 */
/* c10 = c6-c12 */
/* c0 = (c6-c12)*2 */
/* Odd part */
/* c5 */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* -c9 */
/* -c3 */
/* c1 */
/* c1+c7 */
/* c1-c13 */
/* c5 */
/* c11 */
/* c7-c11 */
/* c11+c13 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 16x16 output block.
 *
 * Optimized algorithm with 28 multiplications in the 1-D kernel.
 * cK represents sqrt(2) * cos(K*pi/32).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_16x16(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut tmp27: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 128] = [0; 128];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += ((1i32) << 13 - 2 - 1) as isize; /* c4[16] = c2[8] */
        z1 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c12[16] = c6[8] */
        tmp1 = z1 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c14[16] = c7[8] */
        tmp2 = z1 * 4433; /* c2[16] = c1[8] */
        tmp10 = tmp0 + tmp1; /* (c6+c2)[16] = (c3+c1)[8] */
        tmp11 = tmp0 - tmp1; /* (c6-c14)[16] = (c3-c7)[8] */
        tmp12 = tmp0 + tmp2; /* (c2-c10)[16] = (c1-c5)[8] */
        tmp13 = tmp0 - tmp2; /* (c10-c14)[16] = (c5-c7)[8] */
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z2 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z3 = z1 - z2;
        z4 = z3 * (0.275899379 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = z3 * (1.387039845 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = z3 + z2 * 20995;
        tmp1 = z4 + z1 * 7373;
        tmp2 = z3 - z1 * (0.601344887 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 = z4 - z2 * (0.509795579 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c7 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        tmp11 = z1 + z3; /* c11 */
        tmp1 =
            (z1 + z2) * (1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c13 */
        tmp2 = tmp11 * (1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c7+c5+c3-c1 */
        tmp3 =
            (z1 + z4) * (1.093201867 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c9+c11+c13-c15 */
        tmp10 =
            (z1 - z4) * (0.897167586 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c15 */
        tmp11 = tmp11 * (0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c9+c11-c3-c15 */
        tmp12 =
            (z1 - z2) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5+c7+c15-c3 */
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1 */
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c11-c9-c13 */
        z1 = (z2 + z3) * (0.138617169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c5+c13-c7 */
        tmp1 += z1 + z2 * (0.071888074 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c11 */
        tmp2 += z1 - z3 * (1.125726048 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c11+c15-c7 */
        z1 = (z3 - z2) * (1.407403738 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c5 */
        tmp11 +=
            z1 - z3 * (0.766367282 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c5+c9-c13 */
        tmp12 +=
            z1 + z2 * (1.971951411 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c3 */
        z2 += z4; /* c13 */
        z1 = z2 * -((0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp1 += z1;
        tmp3 += z1 + z4 * (1.065388962 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z2 * -((1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp10 +=
            z2 + z4 * (3.141271809 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp2 += z2;
        tmp3 += z2;
        z2 = (z4 - z3) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 += z2;
        tmp11 += z2;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 15) as isize) = (tmp20 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp1 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 14) as isize) = (tmp21 - tmp1 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp2 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 13) as isize) = (tmp22 - tmp2 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp3 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 12) as isize) = (tmp23 - tmp3 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 11) as isize) = (tmp24 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp25 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp26 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp26 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp27 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp27 - tmp13 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 16 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 16 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        z1 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp1 = z1 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 = z1 * 4433;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z3 = z1 - z2;
        z4 = z3 * (0.275899379 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = z3 * (1.387039845 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = z3 + z2 * 20995;
        tmp1 = z4 + z1 * 7373;
        tmp2 = z3 - z1 * (0.601344887 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 = z4 - z2 * (0.509795579 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z1 + z3;
        tmp1 =
            (z1 + z2) * (1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 = tmp11 * (1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 =
            (z1 + z4) * (1.093201867 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 =
            (z1 - z4) * (0.897167586 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp11 * (0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 =
            (z1 - z2) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.138617169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 += z1 + z2 * (0.071888074 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 += z1 - z3 * (1.125726048 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z3 - z2) * (1.407403738 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 +=
            z1 - z3 * (0.766367282 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            z1 + z2 * (1.971951411 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 += z4;
        z1 = z2 * -((0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp1 += z1;
        tmp3 += z1 + z4 * (1.065388962 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z2 * -((1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp10 +=
            z2 + z4 * (3.141271809 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp2 += z2;
        tmp3 += z2;
        z2 = (z4 - z3) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 += z2;
        tmp11 += z2;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(15) =
            *range_limit.offset(((tmp20 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(14) =
            *range_limit.offset(((tmp21 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(13) =
            *range_limit.offset(((tmp22 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(12) =
            *range_limit.offset(((tmp23 - tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp24 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp25 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp26 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp26 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp27 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp27 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4[16] = c2[8] */
/* c12[16] = c6[8] */
/* c14[16] = c7[8] */
/* c2[16] = c1[8] */
/* (c6+c2)[16] = (c3+c1)[8] */
/* (c6-c14)[16] = (c3-c7)[8] */
/* (c2-c10)[16] = (c1-c5)[8] */
/* (c10-c14)[16] = (c5-c7)[8] */
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c9 */
/* c11 */
/* c13 */
/* c7+c5+c3-c1 */
/* c9+c11+c13-c15 */
/* c15 */
/* c9+c11-c3-c15 */
/* c5+c7+c15-c3 */
/* c1 */
/* c1+c11-c9-c13 */
/* c1+c5+c13-c7 */
/* -c11 */
/* c3+c11+c15-c7 */
/* -c5 */
/* c1+c5+c9-c13 */
/* -c3 */
/* c13 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 16x8 output block.
 *
 * 8-point IDCT in pass 1 (columns), 16-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_16x8(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut tmp27: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 64] = [0; 64];
    /* Pass 1: process columns from input, store into work array. */
    /* Note results are scaled up by sqrt(8) compared to a true IDCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 8;
    while ctr > 0 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((8i32 * 1) as isize) as i32 == 0
            && *inptr.offset((8i32 * 2) as isize) as i32 == 0
            && *inptr.offset((8i32 * 3) as isize) as i32 == 0
            && *inptr.offset((8i32 * 4) as isize) as i32 == 0
            && *inptr.offset((8i32 * 5) as isize) as i32 == 0
            && *inptr.offset((8i32 * 6) as isize) as i32 == 0
            && *inptr.offset((8i32 * 7) as isize) as i32 == 0
        {
            /* AC terms all zero */
            let mut dcval: i32 = *inptr.offset((8i32 * 0) as isize)
                as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 0) as isize)
                << 2; /* advance pointers to next column */
            *wsptr.offset((8i32 * 0) as isize) = dcval;
            *wsptr.offset((8i32 * 1) as isize) = dcval;
            *wsptr.offset((8i32 * 2) as isize) = dcval;
            *wsptr.offset((8i32 * 3) as isize) = dcval;
            *wsptr.offset((8i32 * 4) as isize) = dcval;
            *wsptr.offset((8i32 * 5) as isize) = dcval;
            *wsptr.offset((8i32 * 6) as isize) = dcval;
            *wsptr.offset((8i32 * 7) as isize) = dcval;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        } else {
            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */
            z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 2) as isize))
                as crate::jmorecfg_h::INT32;
            z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 6) as isize))
                as crate::jmorecfg_h::INT32;
            z1 = (z2 + z3) * 4433;
            tmp2 = z1 + z2 * 6270;
            tmp3 = z1 - z3 * 15137;
            z2 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 0) as isize))
                as crate::jmorecfg_h::INT32;
            z3 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 4) as isize))
                as crate::jmorecfg_h::INT32;
            z2 <<= 13isize;
            z3 <<= 13isize;
            /* Add fudge factor here for final descale. */
            z2 += (1) << 13 - 2 - 1;
            tmp0 = z2 + z3;
            tmp1 = z2 - z3;
            tmp10 = tmp0 + tmp2;
            tmp13 = tmp0 - tmp2;
            tmp11 = tmp1 + tmp3;
            tmp12 = tmp1 - tmp3;
            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */
            tmp0 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 7) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * c3 */
            tmp1 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 5) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (-c3-c5) */
            tmp2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 3) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (c5-c3) */
            tmp3 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 1) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (c7-c3) */
            z2 = tmp0 + tmp2; /* sqrt(2) * (-c1+c3+c5-c7) */
            z3 = tmp1 + tmp3; /* sqrt(2) * ( c1+c3-c5-c7) */
            z1 = (z2 + z3) * 9633; /* sqrt(2) * (-c1-c3) */
            z2 = z2 * -(16069); /* sqrt(2) * ( c1+c3-c5+c7) */
            z3 = z3 * -(3196); /* sqrt(2) * ( c1+c3+c5-c7) */
            z2 += z1;
            z3 += z1;
            z1 = (tmp0 + tmp3) * -(7373);
            tmp0 = tmp0 * 2446;
            tmp3 = tmp3 * 12299;
            tmp0 += z1 + z2;
            tmp3 += z1 + z3;
            z1 = (tmp1 + tmp2) * -(20995);
            tmp1 = tmp1 * 16819;
            tmp2 = tmp2 * 25172;
            tmp1 += z1 + z3;
            tmp2 += z1 + z2;
            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
            *wsptr.offset((8i32 * 0) as isize) = (tmp10 + tmp3 >> 13 - 2) as i32; /* advance pointers to next column */
            *wsptr.offset((8i32 * 7) as isize) = (tmp10 - tmp3 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 1) as isize) = (tmp11 + tmp2 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 6) as isize) = (tmp11 - tmp2 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 2) as isize) = (tmp12 + tmp1 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 5) as isize) = (tmp12 - tmp1 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 3) as isize) = (tmp13 + tmp0 >> 13 - 2) as i32;
            *wsptr.offset((8i32 * 4) as isize) = (tmp13 - tmp0 >> 13 - 2) as i32;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        }
        ctr -= 1
    }
    /* Pass 2: process 8 rows from work array, store into output array.
     * 16-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/32).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        z1 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp1 = z1 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 = z1 * 4433;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z3 = z1 - z2;
        z4 = z3 * (0.275899379 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = z3 * (1.387039845 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = z3 + z2 * 20995;
        tmp1 = z4 + z1 * 7373;
        tmp2 = z3 - z1 * (0.601344887 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 = z4 - z2 * (0.509795579 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z1 + z3;
        tmp1 =
            (z1 + z2) * (1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 = tmp11 * (1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 =
            (z1 + z4) * (1.093201867 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 =
            (z1 - z4) * (0.897167586 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp11 * (0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 =
            (z1 - z2) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.138617169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 += z1 + z2 * (0.071888074 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp2 += z1 - z3 * (1.125726048 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = (z3 - z2) * (1.407403738 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 +=
            z1 - z3 * (0.766367282 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            z1 + z2 * (1.971951411 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 += z4;
        z1 = z2 * -((0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp1 += z1;
        tmp3 += z1 + z4 * (1.065388962 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z2 * -((1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp10 +=
            z2 + z4 * (3.141271809 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp2 += z2;
        tmp3 += z2;
        z2 = (z4 - z3) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 += z2;
        tmp11 += z2;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(15) =
            *range_limit.offset(((tmp20 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(14) =
            *range_limit.offset(((tmp21 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(13) =
            *range_limit.offset(((tmp22 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(12) =
            *range_limit.offset(((tmp23 - tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp24 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp25 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp26 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp26 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp27 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp27 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4[16] = c2[8] */
/* c12[16] = c6[8] */
/* c14[16] = c7[8] */
/* c2[16] = c1[8] */
/* (c6+c2)[16] = (c3+c1)[8] */
/* (c6-c14)[16] = (c3-c7)[8] */
/* (c2-c10)[16] = (c1-c5)[8] */
/* (c10-c14)[16] = (c5-c7)[8] */
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c9 */
/* c11 */
/* c13 */
/* c7+c5+c3-c1 */
/* c9+c11+c13-c15 */
/* c15 */
/* c9+c11-c3-c15 */
/* c5+c7+c15-c3 */
/* c1 */
/* c1+c11-c9-c13 */
/* c1+c5+c13-c7 */
/* -c11 */
/* c3+c11+c15-c7 */
/* -c5 */
/* c1+c5+c9-c13 */
/* -c3 */
/* c13 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 14x7 output block.
 *
 * 7-point IDCT in pass 1 (columns), 14-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_14x7(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 56] = [0; 56];
    /* Pass 1: process columns from input, store into work array.
     * 7-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/14).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp23 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp23 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp23 += (1) << 13 - 2 - 1; /* c4 */
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z2 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2+c4-c6 */
        z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        tmp20 =
            (z2 - z3) * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2-c4-c6 */
        tmp22 =
            (z1 - z2) * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2+c4+c6 */
        tmp21 = tmp20 + tmp22 + tmp23
            - z2 * (1.841218003 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c0 */
        tmp10 = z1 + z3;
        z2 -= tmp10;
        tmp10 = tmp10 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp23;
        tmp20 +=
            tmp10 - z3 * (0.077722536 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp22 +=
            tmp10 - z1 * (2.470602249 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp23 += z2 * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* (c3+c1-c5)/2 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* (c3+c5-c1)/2 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* -c1 */
        tmp11 =
            (z1 + z2) * (0.935414347 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5 */
        tmp12 =
            (z1 - z2) * (0.170262339 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c1-c5 */
        tmp10 = tmp11 - tmp12;
        tmp11 += tmp12;
        tmp12 = (z2 + z3)
            * -((1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp11 += tmp12;
        z2 = (z1 + z3) * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 += z2;
        tmp12 +=
            z2 + z3 * (1.870828693 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 7 rows from work array, store into output array.
     * 14-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/28).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 7 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z1 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z1 <<= 13isize;
        z4 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z2 = z4 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = z4 * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z4 * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z1 + z2;
        tmp11 = z1 + z3;
        tmp12 = z1 - z4;
        tmp23 = z1 - (z2 + z3 - z4 << 1);
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z3 = (z1 + z2) * (1.105676686 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z3 + z1 * (0.273079590 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z3 - z2 * (1.719280954 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 = z1 * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2 * (1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        z4 <<= 13isize;
        tmp14 = z1 + z3;
        tmp11 =
            (z1 + z2) * (1.334852607 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = tmp14 * (1.197448846 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + z4
            - z1 * (1.126980169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = tmp14 * (0.752406978 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp16 =
            tmp14 - z1 * (1.061150426 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 -= z2;
        tmp15 = z1 * (0.467085129 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z4;
        tmp16 += tmp15;
        tmp13 = (z2 + z3)
            * -((0.158341681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            - z4;
        tmp11 +=
            tmp13 - z2 * (0.424103948 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            tmp13 - z3 * (2.373959773 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 =
            (z3 - z2) * (1.405321284 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += tmp13 + z4
            - z3 * (1.6906431334 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 +=
            tmp13 + z2 * (0.674957567 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = (z1 - z3 << 13) + z4;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(13) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(12) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp25 - tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp26 + tmp16 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp26 - tmp16 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c12 */
/* c8 */
/* c0 = (c4+c12-c8)*2 */
/* c6 */
/* c2-c6 */
/* c6+c10 */
/* c2 */
/* Odd part */
/* c3 */
/* c5 */
/* c3+c5-c1 */
/* c9 */
/* c9+c11-c13 */
/* c11 */
/* -c13 */
/* c3-c9-c13 */
/* c3+c5-c13 */
/* c1 */
/* c1+c9-c11 */
/* c1+c11-c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 12x6 output block.
 *
 * 6-point IDCT in pass 1 (columns), 12-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_12x6(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 48] = [0; 48];
    /* Pass 1: process columns from input, store into work array.
     * 6-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/12).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp10 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp10 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp10 += (1) << 13 - 2 - 1; /* c4 */
        tmp12 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        tmp20 = tmp12 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp10 + tmp20;
        tmp21 = tmp10 - tmp20 - tmp20 >> 13 - 2;
        tmp20 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp10 = tmp20 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp11 + tmp10;
        tmp22 = tmp11 - tmp10;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32;
        tmp11 =
            (z1 + z3) * (0.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + (z1 + z2 << 13);
        tmp12 = tmp11 + (z3 - z2 << 13);
        tmp11 = z1 - z2 - z3 << 2;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp11) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp21 - tmp11) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 6 rows from work array, store into output array.
     * 12-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/24).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 6 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z3 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z3 <<= 13isize;
        z4 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z4 = z4 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z4 = z1 * (1.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        z2 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z2 <<= 13isize;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z2 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z2 * -(4433);
        tmp10 = z1 + z3;
        tmp15 = (tmp10 + z4)
            * (0.860918669 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = tmp15
            + tmp10 * (0.261052384 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = (z3 + z4)
            * -((1.045510580 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 += tmp14
            - z1 * (0.676326758 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.982889723 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433;
        tmp11 = z3 + z1 * 6270;
        tmp14 = z3 - z2 * 15137;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(11) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(10) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp25 + tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp25 - tmp15 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c3 */
/* -c9 */
/* c7 */
/* c5-c7 */
/* c1-c5 */
/* -(c7+c11) */
/* c1+c5-c7-c11 */
/* c1+c11 */
/* c5+c7 */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 10x5 output block.
 *
 * 5-point IDCT in pass 1 (columns), 10-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_10x5(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 40] = [0; 40];
    /* Pass 1: process columns from input, store into work array.
     * 5-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/10).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp12 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp12 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp12 += (1) << 13 - 2 - 1; /* (c2+c4)/2 */
        tmp13 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32; /* (c2-c4)/2 */
        tmp14 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32;
        z1 = (tmp13 + tmp14)
            * (0.790569415 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = (tmp13 - tmp14)
            * (0.353553391 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= z2 << 2;
        /* Odd part */
        z2 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z3 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c1-c3 */
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c3 */
        tmp13 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp10 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp10 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp11 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp11 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp12 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 5 rows from work array, store into output array.
     * 10-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/20).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 5 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z3 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z3 <<= 13isize;
        z4 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z1 = z4 * (1.144122806 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z4 * (0.437016024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z1;
        tmp11 = z3 - z2;
        tmp22 = z3 - (z1 - z2 << 1);
        z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        z3 <<= 13isize;
        z4 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp11 = z2 + z4;
        tmp13 = z2 - z4;
        tmp12 = tmp13 * (0.309016994 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = tmp11 * (0.951056516 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z3 + tmp12;
        tmp10 = z1 * (1.396802247 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z2
            + z4;
        tmp14 = z1 * (0.221231742 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        z2 = tmp11 * (0.587785252 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z3 - tmp12 - (tmp13 << 13 - 1);
        tmp12 = (z1 - tmp13 << 13) - z3;
        tmp11 = z1 * (1.260073511 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2
            - z4;
        tmp13 = z1 * (0.642039522 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(9) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(8) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp23 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp24 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp24 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c8 */
/* c0 = (c4-c8)*2 */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* Odd part */
/* (c3-c7)/2 */
/* (c3+c7)/2 */
/* c1 */
/* c9 */
/* (c1-c9)/2 */
/* c3 */
/* c7 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 8x4 output block.
 *
 * 4-point IDCT in pass 1 (columns), 8-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_8x4(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 32] = [0; 32];
    /* Pass 1: process columns from input, store into work array.
     * 4-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2 << 2;
        tmp12 = tmp0 - tmp2 << 2;
        /* Odd part */
        /* Same rotation as in the even part of the 8x8 LL&M IDCT */
        z2 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z3 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * 4433;
        /* Add fudge factor here for final descale. */
        z1 += (1) << 13 - 2 - 1;
        tmp0 = z1 + z2 * 6270 >> 13 - 2;
        tmp2 = z1 - z3 * 15137 >> 13 - 2;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp10 + tmp0) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp10 - tmp0) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp12 + tmp2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp12 - tmp2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process rows from work array, store into output array. */
    /* Note that we must descale the results by a factor of 8 == 2**3, */
    /* and also undo the PASS1_BITS scaling. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 4 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * 4433;
        tmp2 = z1 + z2 * 6270;
        tmp3 = z1 - z3 * 15137;
        z2 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z3 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp0 = z2 + z3 << 13;
        tmp1 = z2 - z3 << 13;
        tmp10 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        tmp11 = tmp1 + tmp3;
        tmp12 = tmp1 - tmp3;
        tmp0 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp1 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        tmp3 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = tmp0 + tmp2;
        z3 = tmp1 + tmp3;
        z1 = (z2 + z3) * 9633;
        z2 = z2 * -(16069);
        z3 = z3 * -(3196);
        z2 += z1;
        z3 += z1;
        z1 = (tmp0 + tmp3) * -(7373);
        tmp0 = tmp0 * 2446;
        tmp3 = tmp3 * 12299;
        tmp0 += z1 + z2;
        tmp3 += z1 + z3;
        z1 = (tmp1 + tmp2) * -(20995);
        tmp1 = tmp1 * 16819;
        tmp2 = tmp2 * 25172;
        tmp1 += z1 + z3;
        tmp2 += z1 + z2;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp10 - tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp11 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp12 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp13 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp13 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part: reverse the even part of the forward DCT. */
/* The rotator is sqrt(2)*c(-6). */
/* Add fudge factor here for final descale. */
/* Odd part per figure 8; the matrix is unitary and hence its
 * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
 */
/* sqrt(2) * c3 */
/* sqrt(2) * (-c3-c5) */
/* sqrt(2) * (c5-c3) */
/* sqrt(2) * (c7-c3) */
/* sqrt(2) * (-c1+c3+c5-c7) */
/* sqrt(2) * ( c1+c3-c5-c7) */
/* sqrt(2) * (-c1-c3) */
/* sqrt(2) * ( c1+c3-c5+c7) */
/* sqrt(2) * ( c1+c3+c5-c7) */
/* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 6x3 output block.
 *
 * 3-point IDCT in pass 1 (columns), 6-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_6x3(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 18] = [0; 18];
    /* Pass 1: process columns from input, store into work array.
     * 3-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/6).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 6 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += (1) << 13 - 2 - 1; /* c2 */
        tmp2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp12 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        /* Odd part */
        tmp12 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c1 */
        tmp0 = tmp12 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((6i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 2) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 1) as isize) = (tmp2 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 3 rows from work array, store into output array.
     * 6-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/12).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 3 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        tmp2 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp10 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10;
        tmp10 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp0 = tmp10 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp1 =
            (z1 + z3) * (0.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + (z1 + z2 << 13);
        tmp2 = tmp1 + (z3 - z2 << 13);
        tmp1 = z1 - z2 - z3 << 13;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp11 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(6);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 4x2 output block.
 *
 * 2-point IDCT in pass 1 (columns), 4-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_4x2(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut crate::jmorecfg_h::INT32 = 0 as *mut crate::jmorecfg_h::INT32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [crate::jmorecfg_h::INT32; 8] = [0; 8];
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 4 {
        /* Even part */
        tmp10 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        /* Odd part */
        tmp0 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32;
        /* Final output stage */
        *wsptr.offset((4i32 * 0) as isize) = tmp10 + tmp0;
        *wsptr.offset((4i32 * 1) as isize) = tmp10 - tmp0;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 2 rows from work array, store into output array.
     * 4-point IDCT kernel,
     * cK represents sqrt(2) * cos(K*pi/16) [refers to 8-point IDCT].
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 2 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) + ((1) << 2);
        tmp2 = *wsptr.offset(2);
        tmp10 = tmp0 + tmp2 << 13;
        tmp12 = tmp0 - tmp2 << 13;
        z2 = *wsptr.offset(1);
        z3 = *wsptr.offset(3);
        z1 = (z2 + z3) * 4433;
        tmp0 = z1 + z2 * 6270;
        tmp2 = z1 - z3 * 15137;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(4);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* Odd part */
/* Same rotation as in the even part of the 8x8 LL&M IDCT */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 2x1 output block.
 *
 * 1-point IDCT in pass 1 (columns), 2-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_2x1(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    /* Pass 1: empty. */
    /* Pass 2: process 1 row from input, store into output array. */
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    outptr = (*output_buf.offset(0)).offset(output_col as isize);
    /* Even part */
    tmp10 = (*coef_block.offset(0) as crate::jdct_h::ISLOW_MULT_TYPE * *quantptr.offset(0))
        as crate::jmorecfg_h::INT32;
    /* Add fudge factor here for final descale. */
    tmp10 += (1) << 2;
    /* Odd part */
    tmp0 = (*coef_block.offset(1) as crate::jdct_h::ISLOW_MULT_TYPE * *quantptr.offset(1))
        as crate::jmorecfg_h::INT32;
    /* Final output stage */
    *outptr.offset(0) = *range_limit.offset(((tmp10 + tmp0 >> 3) as i32 & 255 * 4 + 3) as isize);
    *outptr.offset(1) = *range_limit.offset(((tmp10 - tmp0 >> 3) as i32 & 255 * 4 + 3) as isize);
}
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 8x16 output block.
 *
 * 16-point IDCT in pass 1 (columns), 8-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_8x16(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut tmp27: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 128] = [0; 128];
    /* Pass 1: process columns from input, store into work array.
     * 16-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/32).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += (1) << 13 - 2 - 1; /* c4[16] = c2[8] */
        z1 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c12[16] = c6[8] */
        tmp1 = z1 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c14[16] = c7[8] */
        tmp2 = z1 * 4433; /* c2[16] = c1[8] */
        tmp10 = tmp0 + tmp1; /* (c6+c2)[16] = (c3+c1)[8] */
        tmp11 = tmp0 - tmp1; /* (c6-c14)[16] = (c3-c7)[8] */
        tmp12 = tmp0 + tmp2; /* (c2-c10)[16] = (c1-c5)[8] */
        tmp13 = tmp0 - tmp2; /* (c10-c14)[16] = (c5-c7)[8] */
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z2 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z3 = z1 - z2;
        z4 = z3 * (0.275899379 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = z3 * (1.387039845 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = z3 + z2 * 20995;
        tmp1 = z4 + z1 * 7373;
        tmp2 = z3 - z1 * (0.601344887 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp3 = z4 - z2 * (0.509795579 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp0;
        tmp27 = tmp10 - tmp0;
        tmp21 = tmp12 + tmp1;
        tmp26 = tmp12 - tmp1;
        tmp22 = tmp13 + tmp2;
        tmp25 = tmp13 - tmp2;
        tmp23 = tmp11 + tmp3;
        tmp24 = tmp11 - tmp3;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c7 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        tmp11 = z1 + z3; /* c11 */
        tmp1 =
            (z1 + z2) * (1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c13 */
        tmp2 = tmp11 * (1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c7+c5+c3-c1 */
        tmp3 =
            (z1 + z4) * (1.093201867 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c9+c11+c13-c15 */
        tmp10 =
            (z1 - z4) * (0.897167586 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c15 */
        tmp11 = tmp11 * (0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c9+c11-c3-c15 */
        tmp12 =
            (z1 - z2) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5+c7+c15-c3 */
        tmp0 = tmp1 + tmp2 + tmp3
            - z1 * (2.286341144 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1 */
        tmp13 = tmp10 + tmp11 + tmp12
            - z1 * (1.835730603 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c11-c9-c13 */
        z1 = (z2 + z3) * (0.138617169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c5+c13-c7 */
        tmp1 += z1 + z2 * (0.071888074 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c11 */
        tmp2 += z1 - z3 * (1.125726048 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c11+c15-c7 */
        z1 = (z3 - z2) * (1.407403738 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c5 */
        tmp11 +=
            z1 - z3 * (0.766367282 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c5+c9-c13 */
        tmp12 +=
            z1 + z2 * (1.971951411 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c3 */
        z2 += z4; /* c13 */
        z1 = z2 * -((0.666655658 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp1 += z1;
        tmp3 += z1 + z4 * (1.065388962 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = z2 * -((1.247225013 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp10 +=
            z2 + z4 * (3.141271809 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 += z2;
        z2 = (z3 + z4)
            * -((1.353318001 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp2 += z2;
        tmp3 += z2;
        z2 = (z4 - z3) * (0.410524528 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 += z2;
        tmp11 += z2;
        /* Final output stage */
        *wsptr.offset((8i32 * 0) as isize) = (tmp20 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 15) as isize) = (tmp20 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 1) as isize) = (tmp21 + tmp1 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 14) as isize) = (tmp21 - tmp1 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 2) as isize) = (tmp22 + tmp2 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 13) as isize) = (tmp22 - tmp2 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 3) as isize) = (tmp23 + tmp3 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 12) as isize) = (tmp23 - tmp3 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 4) as isize) = (tmp24 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 11) as isize) = (tmp24 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 5) as isize) = (tmp25 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 10) as isize) = (tmp25 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 6) as isize) = (tmp26 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 9) as isize) = (tmp26 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 7) as isize) = (tmp27 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((8i32 * 8) as isize) = (tmp27 - tmp13 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process rows from work array, store into output array. */
    /* Note that we must descale the results by a factor of 8 == 2**3, */
    /* and also undo the PASS1_BITS scaling. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 16 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        z2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * 4433;
        tmp2 = z1 + z2 * 6270;
        tmp3 = z1 - z3 * 15137;
        z2 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        z3 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp0 = z2 + z3 << 13;
        tmp1 = z2 - z3 << 13;
        tmp10 = tmp0 + tmp2;
        tmp13 = tmp0 - tmp2;
        tmp11 = tmp1 + tmp3;
        tmp12 = tmp1 - tmp3;
        tmp0 = *wsptr.offset(7) as crate::jmorecfg_h::INT32;
        tmp1 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        tmp3 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = tmp0 + tmp2;
        z3 = tmp1 + tmp3;
        z1 = (z2 + z3) * 9633;
        z2 = z2 * -(16069);
        z3 = z3 * -(3196);
        z2 += z1;
        z3 += z1;
        z1 = (tmp0 + tmp3) * -(7373);
        tmp0 = tmp0 * 2446;
        tmp3 = tmp3 * 12299;
        tmp0 += z1 + z2;
        tmp3 += z1 + z3;
        z1 = (tmp1 + tmp2) * -(20995);
        tmp1 = tmp1 * 16819;
        tmp2 = tmp2 * 25172;
        tmp1 += z1 + z3;
        tmp2 += z1 + z2;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(7) =
            *range_limit.offset(((tmp10 - tmp3 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp11 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 + tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp12 - tmp1 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp13 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp13 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(8);
        ctr += 1
    }
}
/* Even part: reverse the even part of the forward DCT. */
/* The rotator is sqrt(2)*c(-6). */
/* Add fudge factor here for final descale. */
/* Odd part per figure 8; the matrix is unitary and hence its
 * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
 */
/* sqrt(2) * c3 */
/* sqrt(2) * (-c3-c5) */
/* sqrt(2) * (c5-c3) */
/* sqrt(2) * (c7-c3) */
/* sqrt(2) * (-c1+c3+c5-c7) */
/* sqrt(2) * ( c1+c3-c5-c7) */
/* sqrt(2) * (-c1-c3) */
/* sqrt(2) * ( c1+c3-c5+c7) */
/* sqrt(2) * ( c1+c3+c5-c7) */
/* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 7x14 output block.
 *
 * 14-point IDCT in pass 1 (columns), 7-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_7x14(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut tmp26: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 98] = [0; 98];
    /* Pass 1: process columns from input, store into work array.
     * 14-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/28).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 7 {
        /* Even part */
        z1 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z1 += (1) << 13 - 2 - 1; /* c4 */
        z4 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c12 */
        z2 = z4 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c8 */
        z3 = z4 * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c6 */
        z4 = z4 * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2-c6 */
        tmp10 = z1 + z2; /* c6+c10 */
        tmp11 = z1 + z3; /* c2 */
        tmp12 = z1 - z4;
        tmp23 = z1 - (z2 + z3 - z4 << 1) >> 13 - 2;
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z2 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (z1 + z2) * (1.105676686 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z3 + z1 * (0.273079590 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z3 - z2 * (1.719280954 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 = z1 * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2 * (1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp13;
        tmp26 = tmp10 - tmp13;
        tmp21 = tmp11 + tmp14;
        tmp25 = tmp11 - tmp14;
        tmp22 = tmp12 + tmp15;
        tmp24 = tmp12 - tmp15;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c3+c5-c1 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        tmp13 = z4 << 13; /* c9+c11-c13 */
        tmp14 = z1 + z3; /* c11 */
        tmp11 =
            (z1 + z2) * (1.334852607 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* -c13 */
        tmp12 = tmp14 * (1.197448846 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3-c9-c13 */
        tmp10 = tmp11 + tmp12 + tmp13
            - z1 * (1.126980169 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c5-c13 */
        tmp14 = tmp14 * (0.752406978 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1 */
        tmp16 =
            tmp14 - z1 * (1.061150426 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c9-c11 */
        z1 -= z2; /* c1+c11-c5 */
        tmp15 =
            z1 * (0.467085129 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - tmp13;
        tmp16 += tmp15;
        z1 += z4;
        z4 = (z2 + z3)
            * -((0.158341681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32)
            - tmp13;
        tmp11 +=
            z4 - z2 * (0.424103948 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 +=
            z4 - z3 * (2.373959773 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = (z3 - z2) * (1.405321284 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 += z4 + tmp13
            - z3 * (1.6906431334 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 +=
            z4 + z2 * (0.674957567 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 - z3 << 2;
        /* Final output stage */
        *wsptr.offset((7i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 13) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 12) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 11) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 3) as isize) = (tmp23 + tmp13) as i32;
        *wsptr.offset((7i32 * 10) as isize) = (tmp23 - tmp13) as i32;
        *wsptr.offset((7i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 9) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 5) as isize) = (tmp25 + tmp15 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 8) as isize) = (tmp25 - tmp15 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 6) as isize) = (tmp26 + tmp16 >> 13 - 2) as i32;
        *wsptr.offset((7i32 * 7) as isize) = (tmp26 - tmp16 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 14 rows from work array, store into output array.
     * 7-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/14).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 14 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp23 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp23 <<= 13isize;
        z1 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(6) as crate::jmorecfg_h::INT32;
        tmp20 =
            (z2 - z3) * (0.881747734 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp22 =
            (z1 - z2) * (0.314692123 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp21 = tmp20 + tmp22 + tmp23
            - z2 * (1.841218003 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z1 + z3;
        z2 -= tmp10;
        tmp10 = tmp10 * (1.274162392 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + tmp23;
        tmp20 +=
            tmp10 - z3 * (0.077722536 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp22 +=
            tmp10 - z1 * (2.470602249 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp23 += z2 * (1.414213562 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp11 =
            (z1 + z2) * (0.935414347 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 =
            (z1 - z2) * (0.170262339 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 - tmp12;
        tmp11 += tmp12;
        tmp12 = (z2 + z3)
            * -((1.378756276 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32);
        tmp11 += tmp12;
        z2 = (z1 + z3) * (0.613604268 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 += z2;
        tmp12 +=
            z2 + z3 * (1.870828693 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(6) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp23 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(7);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c6 */
/* c2+c4-c6 */
/* c2 */
/* c2-c4-c6 */
/* c2+c4+c6 */
/* c0 */
/* Odd part */
/* (c3+c1-c5)/2 */
/* (c3+c5-c1)/2 */
/* -c1 */
/* c5 */
/* c3+c1-c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 6x12 output block.
 *
 * 12-point IDCT in pass 1 (columns), 6-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_6x12(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut tmp25: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 72] = [0; 72];
    /* Pass 1: process columns from input, store into work array.
     * 12-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/24).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 6 {
        /* Even part */
        z3 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z3 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z3 += (1) << 13 - 2 - 1; /* c4 */
        z4 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        z4 = z4 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = z3 + z4;
        tmp11 = z3 - z4;
        z1 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z4 = z1 * (1.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 <<= 13isize;
        z2 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z2 <<= 13isize;
        tmp12 = z1 - z2;
        tmp21 = z3 + tmp12;
        tmp24 = z3 - tmp12;
        tmp12 = z4 + z2;
        tmp20 = tmp10 + tmp12;
        tmp25 = tmp10 - tmp12;
        tmp12 = z4 - z1 - z2;
        tmp22 = tmp11 + tmp12;
        tmp23 = tmp11 - tmp12;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c3 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* -c9 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c7 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c5-c7 */
        tmp11 = z2 * (1.306562965 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1-c5 */
        tmp14 = z2 * -(4433); /* -(c7+c11) */
        tmp10 = z1 + z3; /* c1+c5-c7-c11 */
        tmp15 = (tmp10 + z4)
            * (0.860918669 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c1+c11 */
        tmp12 = tmp15
            + tmp10 * (0.261052384 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c5+c7 */
        tmp10 = tmp12
            + tmp11
            + z1 * (0.280143716 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c9 */
        tmp13 = (z3 + z4)
            * -((1.045510580 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32); /* c3-c9 */
        tmp12 += tmp13 + tmp14
            - z3 * (1.478575242 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c3+c9 */
        tmp13 += tmp15 - tmp11
            + z4 * (1.586706681 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp15 += tmp14
            - z1 * (0.676326758 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z4 * (1.982889723 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z1 -= z4;
        z2 -= z3;
        z3 = (z1 + z2) * 4433;
        tmp11 = z3 + z1 * 6270;
        tmp14 = z3 - z2 * 15137;
        /* Final output stage */
        *wsptr.offset((6i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 11) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 10) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 2) as isize) = (tmp22 + tmp12 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 9) as isize) = (tmp22 - tmp12 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 8) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 7) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 5) as isize) = (tmp25 + tmp15 >> 13 - 2) as i32;
        *wsptr.offset((6i32 * 6) as isize) = (tmp25 - tmp15 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 12 rows from work array, store into output array.
     * 6-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/12).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 12 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp10 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp10 <<= 13isize;
        tmp12 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        tmp20 = tmp12 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp11 = tmp10 + tmp20;
        tmp21 = tmp10 - tmp20 - tmp20;
        tmp20 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp10 = tmp20 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp11 + tmp10;
        tmp22 = tmp11 - tmp10;
        z1 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z2 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(5) as crate::jmorecfg_h::INT32;
        tmp11 =
            (z1 + z3) * (0.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + (z1 + z2 << 13);
        tmp12 = tmp11 + (z3 - z2 << 13);
        tmp11 = z1 - z2 - z3 << 13;
        *outptr.offset(0) =
            *range_limit.offset(((tmp20 + tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(5) =
            *range_limit.offset(((tmp20 - tmp10 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp21 + tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp21 - tmp11 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp22 + tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp22 - tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(6);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c4 */
/* c2 */
/* Odd part */
/* c5 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 5x10 output block.
 *
 * 10-point IDCT in pass 1 (columns), 5-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_5x10(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp10: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp20: crate::jmorecfg_h::INT32 = 0;
    let mut tmp21: crate::jmorecfg_h::INT32 = 0;
    let mut tmp22: crate::jmorecfg_h::INT32 = 0;
    let mut tmp23: crate::jmorecfg_h::INT32 = 0;
    let mut tmp24: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut z4: crate::jmorecfg_h::INT32 = 0;
    let mut z5: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 50] = [0; 50];
    /* Pass 1: process columns from input, store into work array.
     * 10-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/20).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 5 {
        /* Even part */
        z3 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        z3 <<= 13isize;
        /* Add fudge factor here for final descale. */
        z3 += (1) << 13 - 2 - 1; /* c4 */
        z4 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c8 */
        z1 = z4 * (1.144122806 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c6 */
        z2 = z4 * (0.437016024 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c2-c6 */
        tmp10 = z3 + z1; /* c2+c6 */
        tmp11 = z3 - z2;
        tmp22 = z3 - (z1 - z2 << 1) >> 13 - 2;
        z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 6) as isize)) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp12 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp20 = tmp10 + tmp12;
        tmp24 = tmp10 - tmp12;
        tmp21 = tmp11 + tmp13;
        tmp23 = tmp11 - tmp13;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* (c3-c7)/2 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* (c3+c7)/2 */
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32; /* c1 */
        z4 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 7) as isize)) as crate::jmorecfg_h::INT32; /* c9 */
        tmp11 = z2 + z4; /* (c1-c9)/2 */
        tmp13 = z2 - z4; /* c3 */
        tmp12 = tmp13 * (0.309016994 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32; /* c7 */
        z5 = z3 << 13;
        z2 = tmp11 * (0.951056516 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z5 + tmp12;
        tmp10 = z1 * (1.396802247 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            + z2
            + z4;
        tmp14 = z1 * (0.221231742 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        z2 = tmp11 * (0.587785252 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z4 = z5 - tmp12 - (tmp13 << 13 - 1);
        tmp12 = z1 - tmp13 - z3 << 2;
        tmp11 = z1 * (1.260073511 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32
            - z2
            - z4;
        tmp13 = z1 * (0.642039522 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32 - z2
            + z4;
        /* Final output stage */
        *wsptr.offset((5i32 * 0) as isize) = (tmp20 + tmp10 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 9) as isize) = (tmp20 - tmp10 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 1) as isize) = (tmp21 + tmp11 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 8) as isize) = (tmp21 - tmp11 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 2) as isize) = (tmp22 + tmp12) as i32;
        *wsptr.offset((5i32 * 7) as isize) = (tmp22 - tmp12) as i32;
        *wsptr.offset((5i32 * 3) as isize) = (tmp23 + tmp13 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 6) as isize) = (tmp23 - tmp13 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 4) as isize) = (tmp24 + tmp14 >> 13 - 2) as i32;
        *wsptr.offset((5i32 * 5) as isize) = (tmp24 - tmp14 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 10 rows from work array, store into output array.
     * 5-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/10).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 10 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp12 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp12 <<= 13isize;
        tmp13 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp14 = *wsptr.offset(4) as crate::jmorecfg_h::INT32;
        z1 = (tmp13 + tmp14)
            * (0.790569415 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z2 = (tmp13 - tmp14)
            * (0.353553391 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        z3 = tmp12 + z2;
        tmp10 = z3 + z1;
        tmp11 = z3 - z1;
        tmp12 -= z2 << 2;
        z2 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * (0.831253876 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp13 = z1 + z2 * (0.513743148 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp14 = z1 - z3 * (2.176250899 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(4) =
            *range_limit.offset(((tmp10 - tmp13 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp11 + tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp11 - tmp14 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(5);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* (c2+c4)/2 */
/* (c2-c4)/2 */
/* Odd part */
/* c3 */
/* c1-c3 */
/* c1+c3 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 4x8 output block.
 *
 * 8-point IDCT in pass 1 (columns), 4-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_4x8(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 32] = [0; 32];
    /* Pass 1: process columns from input, store into work array. */
    /* Note results are scaled up by sqrt(8) compared to a true IDCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 4;
    while ctr > 0 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((8i32 * 1) as isize) as i32 == 0
            && *inptr.offset((8i32 * 2) as isize) as i32 == 0
            && *inptr.offset((8i32 * 3) as isize) as i32 == 0
            && *inptr.offset((8i32 * 4) as isize) as i32 == 0
            && *inptr.offset((8i32 * 5) as isize) as i32 == 0
            && *inptr.offset((8i32 * 6) as isize) as i32 == 0
            && *inptr.offset((8i32 * 7) as isize) as i32 == 0
        {
            /* AC terms all zero */
            let mut dcval: i32 = *inptr.offset((8i32 * 0) as isize)
                as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 0) as isize)
                << 2; /* advance pointers to next column */
            *wsptr.offset((4i32 * 0) as isize) = dcval;
            *wsptr.offset((4i32 * 1) as isize) = dcval;
            *wsptr.offset((4i32 * 2) as isize) = dcval;
            *wsptr.offset((4i32 * 3) as isize) = dcval;
            *wsptr.offset((4i32 * 4) as isize) = dcval;
            *wsptr.offset((4i32 * 5) as isize) = dcval;
            *wsptr.offset((4i32 * 6) as isize) = dcval;
            *wsptr.offset((4i32 * 7) as isize) = dcval;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        } else {
            /* Even part: reverse the even part of the forward DCT. */
            /* The rotator is sqrt(2)*c(-6). */
            z2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 2) as isize))
                as crate::jmorecfg_h::INT32;
            z3 = (*inptr.offset((8i32 * 6) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 6) as isize))
                as crate::jmorecfg_h::INT32;
            z1 = (z2 + z3) * 4433;
            tmp2 = z1 + z2 * 6270;
            tmp3 = z1 - z3 * 15137;
            z2 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 0) as isize))
                as crate::jmorecfg_h::INT32;
            z3 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 4) as isize))
                as crate::jmorecfg_h::INT32;
            z2 <<= 13isize;
            z3 <<= 13isize;
            /* Add fudge factor here for final descale. */
            z2 += (1) << 13 - 2 - 1;
            tmp0 = z2 + z3;
            tmp1 = z2 - z3;
            tmp10 = tmp0 + tmp2;
            tmp13 = tmp0 - tmp2;
            tmp11 = tmp1 + tmp3;
            tmp12 = tmp1 - tmp3;
            /* Odd part per figure 8; the matrix is unitary and hence its
             * transpose is its inverse.  i0..i3 are y7,y5,y3,y1 respectively.
             */
            tmp0 = (*inptr.offset((8i32 * 7) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 7) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * c3 */
            tmp1 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 5) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (-c3-c5) */
            tmp2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 3) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (c5-c3) */
            tmp3 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
                * *quantptr.offset((8i32 * 1) as isize))
                as crate::jmorecfg_h::INT32; /* sqrt(2) * (c7-c3) */
            z2 = tmp0 + tmp2; /* sqrt(2) * (-c1+c3+c5-c7) */
            z3 = tmp1 + tmp3; /* sqrt(2) * ( c1+c3-c5-c7) */
            z1 = (z2 + z3) * 9633; /* sqrt(2) * (-c1-c3) */
            z2 = z2 * -(16069); /* sqrt(2) * ( c1+c3-c5+c7) */
            z3 = z3 * -(3196); /* sqrt(2) * ( c1+c3+c5-c7) */
            z2 += z1;
            z3 += z1;
            z1 = (tmp0 + tmp3) * -(7373);
            tmp0 = tmp0 * 2446;
            tmp3 = tmp3 * 12299;
            tmp0 += z1 + z2;
            tmp3 += z1 + z3;
            z1 = (tmp1 + tmp2) * -(20995);
            tmp1 = tmp1 * 16819;
            tmp2 = tmp2 * 25172;
            tmp1 += z1 + z3;
            tmp2 += z1 + z2;
            /* Final output stage: inputs are tmp10..tmp13, tmp0..tmp3 */
            *wsptr.offset((4i32 * 0) as isize) = (tmp10 + tmp3 >> 13 - 2) as i32; /* advance pointers to next column */
            *wsptr.offset((4i32 * 7) as isize) = (tmp10 - tmp3 >> 13 - 2) as i32;
            *wsptr.offset((4i32 * 1) as isize) = (tmp11 + tmp2 >> 13 - 2) as i32;
            *wsptr.offset((4i32 * 6) as isize) = (tmp11 - tmp2 >> 13 - 2) as i32;
            *wsptr.offset((4i32 * 2) as isize) = (tmp12 + tmp1 >> 13 - 2) as i32;
            *wsptr.offset((4i32 * 5) as isize) = (tmp12 - tmp1 >> 13 - 2) as i32;
            *wsptr.offset((4i32 * 3) as isize) = (tmp13 + tmp0 >> 13 - 2) as i32;
            *wsptr.offset((4i32 * 4) as isize) = (tmp13 - tmp0 >> 13 - 2) as i32;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        }
        ctr -= 1
    }
    /* Pass 2: process 8 rows from work array, store into output array.
     * 4-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 8 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2 << 13;
        tmp12 = tmp0 - tmp2 << 13;
        z2 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        z3 = *wsptr.offset(3) as crate::jmorecfg_h::INT32;
        z1 = (z2 + z3) * 4433;
        tmp0 = z1 + z2 * 6270;
        tmp2 = z1 - z3 * 15137;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(3) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp12 + tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp12 - tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(4);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* Odd part */
/* Same rotation as in the even part of the 8x8 LL&M IDCT */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a reduced-size 3x6 output block.
 *
 * 6-point IDCT in pass 1 (columns), 3-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_3x6(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 18] = [0; 18];
    /* Pass 1: process columns from input, store into work array.
     * 6-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/12).
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 3 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 <<= 13isize;
        /* Add fudge factor here for final descale. */
        tmp0 += (1) << 13 - 2 - 1; /* c4 */
        tmp2 = (*inptr.offset((8i32 * 4) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 4) as isize)) as crate::jmorecfg_h::INT32; /* c2 */
        tmp10 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp1 = tmp0 + tmp10;
        tmp11 = tmp0 - tmp10 - tmp10 >> 13 - 2;
        tmp10 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp0 = tmp10 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp1 + tmp0;
        tmp12 = tmp1 - tmp0;
        /* Odd part */
        z1 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c5 */
        z2 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32;
        z3 = (*inptr.offset((8i32 * 5) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 5) as isize)) as crate::jmorecfg_h::INT32;
        tmp1 =
            (z1 + z3) * (0.366025404 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + (z1 + z2 << 13);
        tmp2 = tmp1 + (z3 - z2 << 13);
        tmp1 = z1 - z2 - z3 << 2;
        /* Final output stage */
        *wsptr.offset((3i32 * 0) as isize) = (tmp10 + tmp0 >> 13 - 2) as i32;
        *wsptr.offset((3i32 * 5) as isize) = (tmp10 - tmp0 >> 13 - 2) as i32;
        *wsptr.offset((3i32 * 1) as isize) = (tmp11 + tmp1) as i32;
        *wsptr.offset((3i32 * 4) as isize) = (tmp11 - tmp1) as i32;
        *wsptr.offset((3i32 * 2) as isize) = (tmp12 + tmp2 >> 13 - 2) as i32;
        *wsptr.offset((3i32 * 3) as isize) = (tmp12 - tmp2 >> 13 - 2) as i32;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 6 rows from work array, store into output array.
     * 3-point IDCT kernel, cK represents sqrt(2) * cos(K*pi/6).
     */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 6 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp0 = *wsptr.offset(0) as crate::jmorecfg_h::INT32 + ((1) << 2 + 2);
        tmp0 <<= 13isize;
        tmp2 = *wsptr.offset(2) as crate::jmorecfg_h::INT32;
        tmp12 = tmp2 * (0.707106781 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp12;
        tmp2 = tmp0 - tmp12 - tmp12;
        tmp12 = *wsptr.offset(1) as crate::jmorecfg_h::INT32;
        tmp0 = tmp12 * (1.224744871 * ((1isize) << 13) as f64 + 0.5) as crate::jmorecfg_h::INT32;
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(2) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp2 >> 13 + 2 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(3);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* c2 */
/* Odd part */
/* c1 */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 2x4 output block.
 *
 * 4-point IDCT in pass 1 (columns), 2-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_2x4(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0; /* buffers data between passes */
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut inptr: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut wsptr: *mut crate::jmorecfg_h::INT32 = 0 as *mut crate::jmorecfg_h::INT32;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    let mut ctr: i32 = 0;
    let mut workspace: [crate::jmorecfg_h::INT32; 8] = [0; 8];
    /* Pass 1: process columns from input, store into work array.
     * 4-point IDCT kernel,
     * cK represents sqrt(2) * cos(K*pi/16) [refers to 8-point IDCT].
     */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 2 {
        /* Even part */
        tmp0 = (*inptr.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
        tmp2 = (*inptr.offset((8i32 * 2) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 2) as isize)) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2 << 13;
        tmp12 = tmp0 - tmp2 << 13;
        /* Odd part */
        /* Same rotation as in the even part of the 8x8 LL&M IDCT */
        z2 = (*inptr.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32; /* c6 */
        z3 = (*inptr.offset((8i32 * 3) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
            * *quantptr.offset((8i32 * 3) as isize)) as crate::jmorecfg_h::INT32; /* c2-c6 */
        z1 = (z2 + z3) * 4433; /* c2+c6 */
        tmp0 = z1 + z2 * 6270;
        tmp2 = z1 - z3 * 15137;
        /* Final output stage */
        *wsptr.offset((2i32 * 0) as isize) = tmp10 + tmp0;
        *wsptr.offset((2i32 * 3) as isize) = tmp10 - tmp0;
        *wsptr.offset((2i32 * 1) as isize) = tmp12 + tmp2;
        *wsptr.offset((2i32 * 2) as isize) = tmp12 - tmp2;
        ctr += 1;
        inptr = inptr.offset(1);
        quantptr = quantptr.offset(1);
        wsptr = wsptr.offset(1)
    }
    /* Pass 2: process 4 rows from work array, store into output array. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0;
    while ctr < 4 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        tmp10 = *wsptr.offset(0) + ((1) << 13 + 2);
        tmp0 = *wsptr.offset(1);
        *outptr.offset(0) =
            *range_limit.offset(((tmp10 + tmp0 >> 13 + 3) as i32 & 255 * 4 + 3) as isize);
        *outptr.offset(1) =
            *range_limit.offset(((tmp10 - tmp0 >> 13 + 3) as i32 & 255 * 4 + 3) as isize);
        wsptr = wsptr.offset(2);
        ctr += 1
    }
}
/* Even part */
/* Add fudge factor here for final descale. */
/* Odd part */
/* Final output stage */
/*
 * Perform dequantization and inverse DCT on one block of coefficients,
 * producing a 1x2 output block.
 *
 * 2-point IDCT in pass 1 (columns), 1-point in pass 2 (rows).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_1x2(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut coef_block: crate::jpeglib_h::JCOEFPTR,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut quantptr: *mut crate::jdct_h::ISLOW_MULT_TYPE =
        0 as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit.offset(128);
    /* Process 1 column from input, store into output array. */
    quantptr = (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
    /* Even part */
    tmp10 = (*coef_block.offset((8i32 * 0) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
        * *quantptr.offset((8i32 * 0) as isize)) as crate::jmorecfg_h::INT32;
    /* Add fudge factor here for final descale. */
    tmp10 += (1) << 2;
    /* Odd part */
    tmp0 = (*coef_block.offset((8i32 * 1) as isize) as crate::jdct_h::ISLOW_MULT_TYPE
        * *quantptr.offset((8i32 * 1) as isize)) as crate::jmorecfg_h::INT32;
    /* Final output stage */
    *(*output_buf.offset(0)).offset(output_col as isize) =
        *range_limit.offset(((tmp10 + tmp0 >> 3) as i32 & 255 * 4 + 3) as isize);
    *(*output_buf.offset(1)).offset(output_col as isize) =
        *range_limit.offset(((tmp10 - tmp0 >> 3) as i32 & 255 * 4 + 3) as isize);
}
/* DCT_ISLOW_SUPPORTED */
/* IDCT_SCALING_SUPPORTED */
