use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::forward_DCT_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table;
pub use crate::src::jpeg_8c::jcomapi::jpeg_alloc_quant_table;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DROP_SAMPLING;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PRECISION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_STATE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jpeg_8c::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jpeg_8c::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_DAC_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_DAC_VALUE;
pub use crate::src::jpeg_8c::jerror::JERR_DHT_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_DQT_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jpeg_8c::jerror::JERR_EMS_READ;
pub use crate::src::jpeg_8c::jerror::JERR_EMS_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jpeg_8c::jerror::JERR_FILE_READ;
pub use crate::src::jpeg_8c::jerror::JERR_FILE_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jpeg_8c::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jpeg_8c::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jpeg_8c::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jpeg_8c::jerror::JERR_INPUT_EOF;
pub use crate::src::jpeg_8c::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_MISSING_DATA;
pub use crate::src::jpeg_8c::jerror::JERR_MODE_CHANGE;
pub use crate::src::jpeg_8c::jerror::JERR_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_NOT_COMPILED;
pub use crate::src::jpeg_8c::jerror::JERR_NO_ARITH_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_IMAGE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_SOI;
pub use crate::src::jpeg_8c::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jpeg_8c::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jpeg_8c::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_CREATE;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_READ;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_SEEK;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jpeg_8c::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jpeg_8c::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jpeg_8c::jerror::JERR_XMS_READ;
pub use crate::src::jpeg_8c::jerror::JERR_XMS_WRITE;
pub use crate::src::jpeg_8c::jerror::JMSG_COPYRIGHT;
pub use crate::src::jpeg_8c::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jpeg_8c::jerror::JMSG_NOMESSAGE;
pub use crate::src::jpeg_8c::jerror::JMSG_VERSION;
pub use crate::src::jpeg_8c::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jpeg_8c::jerror::JTRC_ADOBE;
pub use crate::src::jpeg_8c::jerror::JTRC_APP0;
pub use crate::src::jpeg_8c::jerror::JTRC_APP14;
pub use crate::src::jpeg_8c::jerror::JTRC_DAC;
pub use crate::src::jpeg_8c::jerror::JTRC_DHT;
pub use crate::src::jpeg_8c::jerror::JTRC_DQT;
pub use crate::src::jpeg_8c::jerror::JTRC_DRI;
pub use crate::src::jpeg_8c::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_EMS_OPEN;
pub use crate::src::jpeg_8c::jerror::JTRC_EOI;
pub use crate::src::jpeg_8c::jerror::JTRC_HUFFBITS;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jpeg_8c::jerror::JTRC_MISC_MARKER;
pub use crate::src::jpeg_8c::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANTVALS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jpeg_8c::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jpeg_8c::jerror::JTRC_RST;
pub use crate::src::jpeg_8c::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JTRC_SOF;
pub use crate::src::jpeg_8c::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jpeg_8c::jerror::JTRC_SOI;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jpeg_8c::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_RGB;
pub use crate::src::jpeg_8c::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jpeg_8c::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_XMS_OPEN;
pub use crate::src::jpeg_8c::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE;
pub use crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jpeg_8c::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jpeg_8c::jerror::JWRN_HIT_MARKER;
pub use crate::src::jpeg_8c::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jpeg_8c::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jpeg_8c::jerror::JWRN_JPEG_EOF;
pub use crate::src::jpeg_8c::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jpeg_8c::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jpeg_8c::jerror::JWRN_TOO_MUCH_DATA;
use crate::stdlib::memcpy;
/*
 * jcparam.c
 *
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2003-2008 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains optional default-setting code for the JPEG compressor.
 * Applications do not have to use this file, but those that don't use it
 * must know a lot more about the innards of the JPEG code.
 */
/*
 * Quantization table setup routines
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_add_quant_table(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut which_tbl: libc::c_int,
    mut basic_table: *const libc::c_uint,
    mut scale_factor: libc::c_int,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Define a quantization table equal to the basic_table times
 * a scale factor (given as a percentage).
 * If force_baseline is TRUE, the computed quantization table entries
 * are limited to 1..255 for JPEG baseline compatibility.
 */
{
    let mut qtblptr: *mut *mut crate::jpeglib_h::JQUANT_TBL =
        0 as *mut *mut crate::jpeglib_h::JQUANT_TBL;
    let mut i: libc::c_int = 0;
    let mut temp: libc::c_long = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != 100 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if which_tbl < 0 as libc::c_int || which_tbl >= 4 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_DQT_INDEX as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = which_tbl;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    qtblptr = &mut *(*cinfo)
        .quant_tbl_ptrs
        .as_mut_ptr()
        .offset(which_tbl as isize) as *mut *mut crate::jpeglib_h::JQUANT_TBL;
    if (*qtblptr).is_null() {
        *qtblptr = crate::src::jpeg_8c::jcomapi::jpeg_alloc_quant_table(
            cinfo as crate::jpeglib_h::j_common_ptr as *mut crate::jpeglib_h::jpeg_common_struct,
        ) as *mut crate::jpeglib_h::JQUANT_TBL
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        temp = (*basic_table.offset(i as isize) as libc::c_long * scale_factor as libc::c_long
            + 50 as libc::c_long)
            / 100 as libc::c_long;
        /* limit the values to the valid range */
        if temp <= 0 as libc::c_long {
            temp = 1 as libc::c_long
        } /* max quantizer needed for 12 bits */
        if temp > 32767 as libc::c_long {
            temp = 32767 as libc::c_long
        } /* limit to baseline range if requested */
        if force_baseline != 0 && temp > 255 as libc::c_long {
            temp = 255 as libc::c_long
        }
        (**qtblptr).quantval[i as usize] = temp as crate::jmorecfg_h::UINT16;
        i += 1
    }
    /* Initialize sent_table FALSE so table will be written to JPEG file. */
    (**qtblptr).sent_table = 0 as libc::c_int;
}
/* These are the sample quantization tables given in JPEG spec section K.1.
 * The spec says that the values given produce "good" quality, and
 * when divided by 2, "very good" quality.
 */

static mut std_luminance_quant_tbl: [libc::c_uint; 64] = [
    16 as libc::c_int as libc::c_uint,
    11 as libc::c_int as libc::c_uint,
    10 as libc::c_int as libc::c_uint,
    16 as libc::c_int as libc::c_uint,
    24 as libc::c_int as libc::c_uint,
    40 as libc::c_int as libc::c_uint,
    51 as libc::c_int as libc::c_uint,
    61 as libc::c_int as libc::c_uint,
    12 as libc::c_int as libc::c_uint,
    12 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    19 as libc::c_int as libc::c_uint,
    26 as libc::c_int as libc::c_uint,
    58 as libc::c_int as libc::c_uint,
    60 as libc::c_int as libc::c_uint,
    55 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    13 as libc::c_int as libc::c_uint,
    16 as libc::c_int as libc::c_uint,
    24 as libc::c_int as libc::c_uint,
    40 as libc::c_int as libc::c_uint,
    57 as libc::c_int as libc::c_uint,
    69 as libc::c_int as libc::c_uint,
    56 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    17 as libc::c_int as libc::c_uint,
    22 as libc::c_int as libc::c_uint,
    29 as libc::c_int as libc::c_uint,
    51 as libc::c_int as libc::c_uint,
    87 as libc::c_int as libc::c_uint,
    80 as libc::c_int as libc::c_uint,
    62 as libc::c_int as libc::c_uint,
    18 as libc::c_int as libc::c_uint,
    22 as libc::c_int as libc::c_uint,
    37 as libc::c_int as libc::c_uint,
    56 as libc::c_int as libc::c_uint,
    68 as libc::c_int as libc::c_uint,
    109 as libc::c_int as libc::c_uint,
    103 as libc::c_int as libc::c_uint,
    77 as libc::c_int as libc::c_uint,
    24 as libc::c_int as libc::c_uint,
    35 as libc::c_int as libc::c_uint,
    55 as libc::c_int as libc::c_uint,
    64 as libc::c_int as libc::c_uint,
    81 as libc::c_int as libc::c_uint,
    104 as libc::c_int as libc::c_uint,
    113 as libc::c_int as libc::c_uint,
    92 as libc::c_int as libc::c_uint,
    49 as libc::c_int as libc::c_uint,
    64 as libc::c_int as libc::c_uint,
    78 as libc::c_int as libc::c_uint,
    87 as libc::c_int as libc::c_uint,
    103 as libc::c_int as libc::c_uint,
    121 as libc::c_int as libc::c_uint,
    120 as libc::c_int as libc::c_uint,
    101 as libc::c_int as libc::c_uint,
    72 as libc::c_int as libc::c_uint,
    92 as libc::c_int as libc::c_uint,
    95 as libc::c_int as libc::c_uint,
    98 as libc::c_int as libc::c_uint,
    112 as libc::c_int as libc::c_uint,
    100 as libc::c_int as libc::c_uint,
    103 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
];

static mut std_chrominance_quant_tbl: [libc::c_uint; 64] = [
    17 as libc::c_int as libc::c_uint,
    18 as libc::c_int as libc::c_uint,
    24 as libc::c_int as libc::c_uint,
    47 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    18 as libc::c_int as libc::c_uint,
    21 as libc::c_int as libc::c_uint,
    26 as libc::c_int as libc::c_uint,
    66 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    24 as libc::c_int as libc::c_uint,
    26 as libc::c_int as libc::c_uint,
    56 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    47 as libc::c_int as libc::c_uint,
    66 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
    99 as libc::c_int as libc::c_uint,
];
#[no_mangle]

pub unsafe extern "C" fn jpeg_default_qtables(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Set or change the 'quality' (quantization) setting, using default tables
 * and straight percentage-scaling quality scales.
 * This entry point allows different scalings for luminance and chrominance.
 */
{
    /* Set up two quantization tables using the specified scaling */
    jpeg_add_quant_table(
        cinfo,
        0 as libc::c_int,
        std_luminance_quant_tbl.as_ptr(),
        (*cinfo).q_scale_factor[0 as libc::c_int as usize],
        force_baseline,
    );
    jpeg_add_quant_table(
        cinfo,
        1 as libc::c_int,
        std_chrominance_quant_tbl.as_ptr(),
        (*cinfo).q_scale_factor[1 as libc::c_int as usize],
        force_baseline,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_linear_quality(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut scale_factor: libc::c_int,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Set or change the 'quality' (quantization) setting, using default tables
 * and a straight percentage-scaling quality scale.  In most cases it's better
 * to use jpeg_set_quality (below); this entry point is provided for
 * applications that insist on a linear percentage scaling.
 */
{
    /* Set up two quantization tables using the specified scaling */
    jpeg_add_quant_table(
        cinfo,
        0 as libc::c_int,
        std_luminance_quant_tbl.as_ptr(),
        scale_factor,
        force_baseline,
    );
    jpeg_add_quant_table(
        cinfo,
        1 as libc::c_int,
        std_chrominance_quant_tbl.as_ptr(),
        scale_factor,
        force_baseline,
    );
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_quality_scaling(mut quality: libc::c_int) -> libc::c_int
/* Convert a user-specified quality rating to a percentage scaling factor
 * for an underlying quantization table, using our recommended scaling curve.
 * The input 'quality' factor should be 0 (terrible) to 100 (very good).
 */ {
    /* Safety limit on quality factor.  Convert 0 to 1 to avoid zero divide. */
    if quality <= 0 as libc::c_int {
        quality = 1 as libc::c_int
    }
    if quality > 100 as libc::c_int {
        quality = 100 as libc::c_int
    }
    /* The basic table is used as-is (scaling 100) for a quality of 50.
     * Qualities 50..100 are converted to scaling percentage 200 - 2*Q;
     * note that at Q=100 the scaling is 0, which will cause jpeg_add_quant_table
     * to make all the table entries 1 (hence, minimum quantization loss).
     * Qualities 1..50 are converted to scaling percentage 5000/Q.
     */
    if quality < 50 as libc::c_int {
        quality = 5000 as libc::c_int / quality
    } else {
        quality = 200 as libc::c_int - quality * 2 as libc::c_int
    }
    return quality;
}
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_quality(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut quality: libc::c_int,
    mut force_baseline: crate::jmorecfg_h::boolean,
)
/* Set or change the 'quality' (quantization) setting, using default tables.
 * This is the standard quality-adjusting entry point for typical user
 * interfaces; only those who want detailed control over quantization tables
 * would use the preceding three routines directly.
 */
{
    /* Convert user 0-100 rating to percentage scaling */
    quality = jpeg_quality_scaling(quality);
    /* Set up standard quality tables */
    jpeg_set_linear_quality(cinfo, quality, force_baseline);
}
/*
 * Huffman table setup routines
 */

unsafe extern "C" fn add_huff_table(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut htblptr: *mut *mut crate::jpeglib_h::JHUFF_TBL,
    mut bits: *const crate::jmorecfg_h::UINT8,
    mut val: *const crate::jmorecfg_h::UINT8,
)
/* Define a Huffman table */
{
    let mut nsymbols: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if (*htblptr).is_null() {
        *htblptr = crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table(
            cinfo as crate::jpeglib_h::j_common_ptr as *mut crate::jpeglib_h::jpeg_common_struct,
        ) as *mut crate::jpeglib_h::JHUFF_TBL
    }
    /* Copy the number-of-symbols-of-each-code-length counts */
    crate::stdlib::memcpy(
        (**htblptr).bits.as_mut_ptr() as *mut libc::c_void,
        bits as *const libc::c_void,
        ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 17]>() as libc::c_ulong,
    );
    /* Validate the counts.  We do this here mainly so we can copy the right
     * number of symbols from the val[] array, without risking marching off
     * the end of memory.  jchuff.c will do a more thorough test later.
     */
    nsymbols = 0 as libc::c_int;
    len = 1 as libc::c_int;
    while len <= 16 as libc::c_int {
        nsymbols += *bits.offset(len as isize) as libc::c_int;
        len += 1
    }
    if nsymbols < 1 as libc::c_int || nsymbols > 256 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    crate::stdlib::memcpy(
        (**htblptr).huffval.as_mut_ptr() as *mut libc::c_void,
        val as *const libc::c_void,
        (nsymbols as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::UINT8>() as libc::c_ulong),
    );
    /* Initialize sent_table FALSE so table will be written to JPEG file. */
    (**htblptr).sent_table = 0 as libc::c_int;
}

unsafe extern "C" fn std_huff_tables(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Set up the standard Huffman tables (cf. JPEG standard section K.3) */
/* IMPORTANT: these are only valid for 8-bit data precision! */
{
    static mut bits_dc_luminance: [crate::jmorecfg_h::UINT8; 17] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        5 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut val_dc_luminance: [crate::jmorecfg_h::UINT8; 12] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        5 as libc::c_int as crate::jmorecfg_h::UINT8,
        6 as libc::c_int as crate::jmorecfg_h::UINT8,
        7 as libc::c_int as crate::jmorecfg_h::UINT8,
        8 as libc::c_int as crate::jmorecfg_h::UINT8,
        9 as libc::c_int as crate::jmorecfg_h::UINT8,
        10 as libc::c_int as crate::jmorecfg_h::UINT8,
        11 as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut bits_dc_chrominance: [crate::jmorecfg_h::UINT8; 17] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut val_dc_chrominance: [crate::jmorecfg_h::UINT8; 12] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        5 as libc::c_int as crate::jmorecfg_h::UINT8,
        6 as libc::c_int as crate::jmorecfg_h::UINT8,
        7 as libc::c_int as crate::jmorecfg_h::UINT8,
        8 as libc::c_int as crate::jmorecfg_h::UINT8,
        9 as libc::c_int as crate::jmorecfg_h::UINT8,
        10 as libc::c_int as crate::jmorecfg_h::UINT8,
        11 as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut bits_ac_luminance: [crate::jmorecfg_h::UINT8; 17] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        5 as libc::c_int as crate::jmorecfg_h::UINT8,
        5 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x7d as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut val_ac_luminance: [crate::jmorecfg_h::UINT8; 162] = [
        0x1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x11 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x12 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x21 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x31 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x41 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x13 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x51 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x61 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x22 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x71 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x14 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x32 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x81 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x91 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x23 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x42 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x15 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x52 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x24 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x33 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x62 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x72 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x82 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa as libc::c_int as crate::jmorecfg_h::UINT8,
        0x16 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x17 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x18 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x19 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x1a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x25 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x26 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x27 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x28 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x29 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x2a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x34 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x35 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x36 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x37 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x38 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x39 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x3a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x43 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x44 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x45 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x46 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x47 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x48 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x49 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x4a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x53 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x54 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x55 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x56 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x57 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x58 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x59 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x5a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x63 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x64 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x65 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x66 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x67 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x68 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x69 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x6a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x73 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x74 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x75 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x76 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x77 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x78 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x79 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x7a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x83 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x84 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x85 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x86 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x87 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x88 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x89 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x8a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x92 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x93 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x94 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x95 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x96 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x97 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x98 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x99 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x9a as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xaa as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xba as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xca as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xda as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xea as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xfa as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut bits_ac_chrominance: [crate::jmorecfg_h::UINT8; 17] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        3 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        7 as libc::c_int as crate::jmorecfg_h::UINT8,
        5 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        1 as libc::c_int as crate::jmorecfg_h::UINT8,
        2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x77 as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    static mut val_ac_chrominance: [crate::jmorecfg_h::UINT8; 162] = [
        0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x11 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x21 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x31 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x12 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x41 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x51 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x61 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x71 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x13 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x22 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x32 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x81 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x14 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x42 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x91 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x23 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x33 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x52 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf0 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x15 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x62 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x72 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa as libc::c_int as crate::jmorecfg_h::UINT8,
        0x16 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x24 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x34 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x25 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf1 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x17 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x18 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x19 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x1a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x26 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x27 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x28 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x29 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x2a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x35 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x36 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x37 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x38 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x39 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x3a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x43 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x44 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x45 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x46 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x47 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x48 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x49 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x4a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x53 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x54 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x55 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x56 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x57 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x58 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x59 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x5a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x63 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x64 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x65 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x66 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x67 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x68 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x69 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x6a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x73 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x74 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x75 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x76 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x77 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x78 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x79 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x7a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x82 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x83 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x84 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x85 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x86 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x87 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x88 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x89 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x8a as libc::c_int as crate::jmorecfg_h::UINT8,
        0x92 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x93 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x94 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x95 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x96 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x97 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x98 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x99 as libc::c_int as crate::jmorecfg_h::UINT8,
        0x9a as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xa9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xaa as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xb9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xba as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xc9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xca as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xd9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xda as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xe9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xea as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf2 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf3 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf4 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf5 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf6 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf7 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf8 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xf9 as libc::c_int as crate::jmorecfg_h::UINT8,
        0xfa as libc::c_int as crate::jmorecfg_h::UINT8,
    ];
    add_huff_table(
        cinfo,
        &mut *(*cinfo)
            .dc_huff_tbl_ptrs
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        bits_dc_luminance.as_ptr(),
        val_dc_luminance.as_ptr(),
    );
    add_huff_table(
        cinfo,
        &mut *(*cinfo)
            .ac_huff_tbl_ptrs
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        bits_ac_luminance.as_ptr(),
        val_ac_luminance.as_ptr(),
    );
    add_huff_table(
        cinfo,
        &mut *(*cinfo)
            .dc_huff_tbl_ptrs
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        bits_dc_chrominance.as_ptr(),
        val_dc_chrominance.as_ptr(),
    );
    add_huff_table(
        cinfo,
        &mut *(*cinfo)
            .ac_huff_tbl_ptrs
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        bits_ac_chrominance.as_ptr(),
        val_ac_chrominance.as_ptr(),
    );
}
/*
 * Default parameter setup for compression.
 *
 * Applications that don't choose to use this routine must do their
 * own setup of all these parameters.  Alternately, you can call this
 * to establish defaults and then alter parameters selectively.  This
 * is the recommended approach since, if we add any new parameters,
 * your code will still work (they'll be set to reasonable defaults).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_defaults(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut i: libc::c_int = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != 100 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate comp_info array large enough for maximum component count.
     * Array is made permanent in case application wants to compress
     * multiple images at same param settings.
     */
    if (*cinfo).comp_info.is_null() {
        (*cinfo).comp_info = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            0 as libc::c_int,
            (10 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::jpeglib_h::jpeg_component_info,
            >() as libc::c_ulong),
        ) as *mut crate::jpeglib_h::jpeg_component_info
    }
    /* Initialize everything not dependent on the color space */
    (*cinfo).scale_num = 1 as libc::c_int as libc::c_uint; /* 1:1 scaling */
    (*cinfo).scale_denom = 1 as libc::c_int as libc::c_uint;
    (*cinfo).data_precision = 8 as libc::c_int;
    /* Set up two quantization tables using default quality of 75 */
    jpeg_set_quality(cinfo, 75 as libc::c_int, 1 as libc::c_int);
    /* Set up two Huffman tables */
    std_huff_tables(cinfo);
    /* Initialize default arithmetic coding conditioning */
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*cinfo).arith_dc_L[i as usize] = 0 as libc::c_int as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1 as libc::c_int as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5 as libc::c_int as crate::jmorecfg_h::UINT8;
        i += 1
    }
    /* Default is no multiple-scan output */
    (*cinfo).scan_info = 0 as *const crate::jpeglib_h::jpeg_scan_info;
    (*cinfo).num_scans = 0 as libc::c_int;
    /* Expect normal source image, not raw downsampled data */
    (*cinfo).raw_data_in = 0 as libc::c_int;
    /* Use Huffman coding, not arithmetic coding, by default */
    (*cinfo).arith_code = 0 as libc::c_int;
    /* By default, don't do extra passes to optimize entropy coding */
    (*cinfo).optimize_coding = 0 as libc::c_int;
    /* The standard Huffman tables are only valid for 8-bit data precision.
     * If the precision is higher, force optimization on so that usable
     * tables will be computed.  This test can be removed if default tables
     * are supplied that are valid for the desired precision.
     */
    if (*cinfo).data_precision > 8 as libc::c_int {
        (*cinfo).optimize_coding = 1 as libc::c_int
    }
    /* By default, use the simpler non-cosited sampling alignment */
    (*cinfo).CCIR601_sampling = 0 as libc::c_int;
    /* By default, apply fancy downsampling */
    (*cinfo).do_fancy_downsampling = 1 as libc::c_int;
    /* No input smoothing */
    (*cinfo).smoothing_factor = 0 as libc::c_int;
    /* DCT algorithm preference */
    (*cinfo).dct_method = crate::jpeglib_h::JDCT_ISLOW;
    /* No restart markers */
    (*cinfo).restart_interval = 0 as libc::c_int as libc::c_uint;
    (*cinfo).restart_in_rows = 0 as libc::c_int;
    /* Fill in default JFIF marker parameters.  Note that whether the marker
     * will actually be written is determined by jpeg_set_colorspace.
     *
     * By default, the library emits JFIF version code 1.01.
     * An application that wants to emit JFIF 1.02 extension markers should set
     * JFIF_minor_version to 2.  We could probably get away with just defaulting
     * to 1.02, but there may still be some decoders in use that will complain
     * about that; saying 1.01 should minimize compatibility problems.
     */
    (*cinfo).JFIF_major_version = 1 as libc::c_int as crate::jmorecfg_h::UINT8; /* Default JFIF version = 1.01 */
    (*cinfo).JFIF_minor_version = 1 as libc::c_int as crate::jmorecfg_h::UINT8; /* Pixel size is unknown by default */
    (*cinfo).density_unit = 0 as libc::c_int as crate::jmorecfg_h::UINT8; /* Pixel aspect ratio is square by default */
    (*cinfo).X_density = 1 as libc::c_int as crate::jmorecfg_h::UINT16;
    (*cinfo).Y_density = 1 as libc::c_int as crate::jmorecfg_h::UINT16;
    /* Choose JPEG colorspace based on input space, set defaults accordingly */
    jpeg_default_colorspace(cinfo);
}
/*
 * Select an appropriate JPEG colorspace for in_color_space.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_default_colorspace(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    match (*cinfo).in_color_space as libc::c_uint {
        1 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_GRAYSCALE); /* By default, no translation */
        }
        2 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCbCr);
        }
        3 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCbCr);
        }
        4 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_CMYK);
        }
        5 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCCK);
        }
        0 => {
            jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_UNKNOWN);
        }
        _ => {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JERR_BAD_IN_COLORSPACE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
/*
 * Set the JPEG colorspace, and choose colorspace-dependent default values.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_colorspace(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut colorspace: crate::jpeglib_h::J_COLOR_SPACE,
) {
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut ci: libc::c_int = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != 100 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For all colorspaces, we use Q and Huff tables 0 for luminance components,
     * tables 1 for chrominance components.
     */
    (*cinfo).jpeg_color_space = colorspace; /* No marker for non-JFIF colorspaces */
    (*cinfo).write_JFIF_header = 0 as libc::c_int; /* write no Adobe marker by default */
    (*cinfo).write_Adobe_marker = 0 as libc::c_int; /* Write a JFIF marker */
    match colorspace as libc::c_uint {
        1 => {
            (*cinfo).write_JFIF_header = 1 as libc::c_int;
            (*cinfo).num_components = 1 as libc::c_int;
            /* JFIF specifies component ID 1 */
            compptr = &mut *(*cinfo).comp_info.offset(0 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info; /* write Adobe marker to flag RGB */
            (*compptr).component_id = 1 as libc::c_int; /* Write a JFIF marker */
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int
        }
        2 => {
            (*cinfo).write_Adobe_marker = 1 as libc::c_int;
            (*cinfo).num_components = 3 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(0 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x52 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(1 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x47 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(2 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x42 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int
        }
        3 => {
            (*cinfo).write_JFIF_header = 1 as libc::c_int;
            (*cinfo).num_components = 3 as libc::c_int;
            /* JFIF specifies component IDs 1,2,3 */
            /* We default to 2x2 subsamples of chrominance */
            compptr = &mut *(*cinfo).comp_info.offset(0 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info; /* write Adobe marker to flag CMYK */
            (*compptr).component_id = 1 as libc::c_int; /* write Adobe marker to flag YCCK */
            (*compptr).h_samp_factor = 2 as libc::c_int;
            (*compptr).v_samp_factor = 2 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(1 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 2 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 1 as libc::c_int;
            (*compptr).dc_tbl_no = 1 as libc::c_int;
            (*compptr).ac_tbl_no = 1 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(2 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 3 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 1 as libc::c_int;
            (*compptr).dc_tbl_no = 1 as libc::c_int;
            (*compptr).ac_tbl_no = 1 as libc::c_int
        }
        4 => {
            (*cinfo).write_Adobe_marker = 1 as libc::c_int;
            (*cinfo).num_components = 4 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(0 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x43 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(1 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x4d as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(2 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x59 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(3 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 0x4b as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int
        }
        5 => {
            (*cinfo).write_Adobe_marker = 1 as libc::c_int;
            (*cinfo).num_components = 4 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(0 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 1 as libc::c_int;
            (*compptr).h_samp_factor = 2 as libc::c_int;
            (*compptr).v_samp_factor = 2 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(1 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 2 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 1 as libc::c_int;
            (*compptr).dc_tbl_no = 1 as libc::c_int;
            (*compptr).ac_tbl_no = 1 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(2 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 3 as libc::c_int;
            (*compptr).h_samp_factor = 1 as libc::c_int;
            (*compptr).v_samp_factor = 1 as libc::c_int;
            (*compptr).quant_tbl_no = 1 as libc::c_int;
            (*compptr).dc_tbl_no = 1 as libc::c_int;
            (*compptr).ac_tbl_no = 1 as libc::c_int;
            compptr = &mut *(*cinfo).comp_info.offset(3 as libc::c_int as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            (*compptr).component_id = 4 as libc::c_int;
            (*compptr).h_samp_factor = 2 as libc::c_int;
            (*compptr).v_samp_factor = 2 as libc::c_int;
            (*compptr).quant_tbl_no = 0 as libc::c_int;
            (*compptr).dc_tbl_no = 0 as libc::c_int;
            (*compptr).ac_tbl_no = 0 as libc::c_int
        }
        0 => {
            (*cinfo).num_components = (*cinfo).input_components;
            if (*cinfo).num_components < 1 as libc::c_int
                || (*cinfo).num_components > 10 as libc::c_int
            {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).num_components;
                (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = 10 as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci = 0 as libc::c_int;
            while ci < (*cinfo).num_components {
                compptr = &mut *(*cinfo).comp_info.offset(ci as isize)
                    as *mut crate::jpeglib_h::jpeg_component_info;
                (*compptr).component_id = ci;
                (*compptr).h_samp_factor = 1 as libc::c_int;
                (*compptr).v_samp_factor = 1 as libc::c_int;
                (*compptr).quant_tbl_no = 0 as libc::c_int;
                (*compptr).dc_tbl_no = 0 as libc::c_int;
                (*compptr).ac_tbl_no = 0 as libc::c_int;
                ci += 1
            }
        }
        _ => {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}

unsafe extern "C" fn fill_a_scan(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ci: libc::c_int,
    mut Ss: libc::c_int,
    mut Se: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate one scan for specified component */ {
    (*scanptr).comps_in_scan = 1 as libc::c_int;
    (*scanptr).component_index[0 as libc::c_int as usize] = ci;
    (*scanptr).Ss = Ss;
    (*scanptr).Se = Se;
    (*scanptr).Ah = Ah;
    (*scanptr).Al = Al;
    scanptr = scanptr.offset(1);
    return scanptr;
}

unsafe extern "C" fn fill_scans(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ncomps: libc::c_int,
    mut Ss: libc::c_int,
    mut Se: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate one scan for each component */ {
    let mut ci: libc::c_int = 0;
    ci = 0 as libc::c_int;
    while ci < ncomps {
        (*scanptr).comps_in_scan = 1 as libc::c_int;
        (*scanptr).component_index[0 as libc::c_int as usize] = ci;
        (*scanptr).Ss = Ss;
        (*scanptr).Se = Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1);
        ci += 1
    }
    return scanptr;
}

unsafe extern "C" fn fill_dc_scans(
    mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info,
    mut ncomps: libc::c_int,
    mut Ah: libc::c_int,
    mut Al: libc::c_int,
) -> *mut crate::jpeglib_h::jpeg_scan_info
/* Support routine: generate interleaved DC scan if possible, else N scans */ {
    let mut ci: libc::c_int = 0;
    if ncomps <= 4 as libc::c_int {
        /* Single interleaved DC scan */
        (*scanptr).comps_in_scan = ncomps;
        ci = 0 as libc::c_int;
        while ci < ncomps {
            (*scanptr).component_index[ci as usize] = ci;
            ci += 1
        }
        (*scanptr).Se = 0 as libc::c_int;
        (*scanptr).Ss = (*scanptr).Se;
        (*scanptr).Ah = Ah;
        (*scanptr).Al = Al;
        scanptr = scanptr.offset(1)
    } else {
        /* Noninterleaved DC scan for each component */
        scanptr = fill_scans(scanptr, ncomps, 0 as libc::c_int, 0 as libc::c_int, Ah, Al)
    }
    return scanptr;
}
/*
 * Create a recommended progressive-JPEG script.
 * cinfo->num_components and cinfo->jpeg_color_space must be correct.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_simple_progression(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut ncomps: libc::c_int = (*cinfo).num_components;
    let mut nscans: libc::c_int = 0;
    let mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info =
        0 as *mut crate::jpeglib_h::jpeg_scan_info;
    /* Safety check to ensure start_compress not called yet. */
    if (*cinfo).global_state != 100 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Figure space needed for script.  Calculation must match code below! */
    if ncomps == 3 as libc::c_int
        && (*cinfo).jpeg_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
    {
        /* Custom script for YCbCr color images. */
        nscans = 10 as libc::c_int
    } else if ncomps > 4 as libc::c_int {
        /* All-purpose script for other color spaces. */
        nscans = 6 as libc::c_int * ncomps
    } else {
        nscans = 2 as libc::c_int + 4 as libc::c_int * ncomps
    } /* 2 DC + 4 AC scans per component */
    /* 2 DC scans; 4 AC scans per component */
    /* Allocate space for script.
     * We need to put it in the permanent pool in case the application performs
     * multiple compressions without changing the settings.  To avoid a memory
     * leak if jpeg_simple_progression is called repeatedly for the same JPEG
     * object, we try to re-use previously allocated space, and we allocate
     * enough space to handle YCbCr even if initially asked for grayscale.
     */
    if (*cinfo).script_space.is_null() || (*cinfo).script_space_size < nscans {
        (*cinfo).script_space_size = if nscans > 10 as libc::c_int {
            nscans
        } else {
            10 as libc::c_int
        };
        (*cinfo).script_space = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            0 as libc::c_int,
            ((*cinfo).script_space_size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::jpeglib_h::jpeg_scan_info>() as libc::c_ulong
                ),
        ) as *mut crate::jpeglib_h::jpeg_scan_info
    }
    scanptr = (*cinfo).script_space;
    (*cinfo).scan_info = scanptr;
    (*cinfo).num_scans = nscans;
    if ncomps == 3 as libc::c_int
        && (*cinfo).jpeg_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
    {
        /* Custom script for YCbCr color images. */
        /* Initial DC scan */
        scanptr = fill_dc_scans(scanptr, ncomps, 0 as libc::c_int, 1 as libc::c_int);
        /* Initial AC scan: get some luma data out in a hurry */
        scanptr = fill_a_scan(
            scanptr,
            0 as libc::c_int,
            1 as libc::c_int,
            5 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
        );
        /* Chroma data is too small to be worth expending many scans on */
        scanptr = fill_a_scan(
            scanptr,
            2 as libc::c_int,
            1 as libc::c_int,
            63 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            1 as libc::c_int,
            1 as libc::c_int,
            63 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        /* Complete spectral selection for luma AC */
        scanptr = fill_a_scan(
            scanptr,
            0 as libc::c_int,
            6 as libc::c_int,
            63 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
        );
        /* Refine next bit of luma AC */
        scanptr = fill_a_scan(
            scanptr,
            0 as libc::c_int,
            1 as libc::c_int,
            63 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
        );
        /* Finish DC successive approximation */
        scanptr = fill_dc_scans(scanptr, ncomps, 1 as libc::c_int, 0 as libc::c_int);
        /* Finish AC successive approximation */
        scanptr = fill_a_scan(
            scanptr,
            2 as libc::c_int,
            1 as libc::c_int,
            63 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        scanptr = fill_a_scan(
            scanptr,
            1 as libc::c_int,
            1 as libc::c_int,
            63 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        /* Luma bottom bit comes last since it's usually largest scan */
        scanptr = fill_a_scan(
            scanptr,
            0 as libc::c_int,
            1 as libc::c_int,
            63 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        )
    } else {
        /* All-purpose script for other color spaces. */
        /* Successive approximation first pass */
        scanptr = fill_dc_scans(scanptr, ncomps, 0 as libc::c_int, 1 as libc::c_int);
        scanptr = fill_scans(
            scanptr,
            ncomps,
            1 as libc::c_int,
            5 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
        );
        scanptr = fill_scans(
            scanptr,
            ncomps,
            6 as libc::c_int,
            63 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
        );
        /* Successive approximation second pass */
        scanptr = fill_scans(
            scanptr,
            ncomps,
            1 as libc::c_int,
            63 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
        );
        /* Successive approximation final pass */
        scanptr = fill_dc_scans(scanptr, ncomps, 1 as libc::c_int, 0 as libc::c_int);
        scanptr = fill_scans(
            scanptr,
            ncomps,
            1 as libc::c_int,
            63 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        )
    };
}
/* C_PROGRESSIVE_SUPPORTED */
