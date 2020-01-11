use ::libc;

/* JERROR_H */

/* Informational/debugging messages */

/* Nonfatal errors (we can keep going, but the data is probably corrupt) */

/* Fatal errors (print message and exit) */

/* Macros to simplify using the error and trace message stuff */

/* The first parameter is either type of cinfo pointer */

/* Zap JMESSAGE macro so that future re-inclusions do nothing by default */

/* JMAKE_ENUM_LIST */
pub use crate::stddef_h::size_t;

pub use crate::jdct_h::float_DCT_method_ptr;
pub use crate::jdct_h::forward_DCT_method_ptr;
pub use crate::jdct_h::DCTELEM;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::INT32;
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
pub use crate::jpeglib_h::JCOEFPTR;
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
pub use crate::src::jpeg_8c::jfdctflt::jpeg_fdct_float;
pub use crate::src::jpeg_8c::jfdctfst::jpeg_fdct_ifast;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_10x10;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_10x5;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_11x11;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_12x12;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_12x6;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_13x13;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_14x14;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_14x7;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_15x15;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_16x16;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_16x8;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_1x1;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_1x2;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x1;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x2;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x4;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_3x3;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_3x6;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x2;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x4;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x8;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_5x10;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_5x5;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x12;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x3;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x6;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_7x14;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_7x7;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_8x16;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_8x4;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_9x9;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_islow;

pub type my_fdct_ptr = *mut my_fdct_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_fdct_controller {
    pub pub_0: crate::jpegint_h::jpeg_forward_dct,
    pub do_dct: [crate::jdct_h::forward_DCT_method_ptr; 10],
    pub divisors: [*mut crate::jdct_h::DCTELEM; 4],
    pub do_float_dct: [crate::jdct_h::float_DCT_method_ptr; 10],
    pub float_divisors: [*mut f32; 4],
}
/* The current scaled-DCT routines require ISLOW-style divisor tables,
 * so be sure to compile that code if either ISLOW or SCALING is requested.
 */
/*
 * Perform forward DCT on one or more blocks of a component.
 *
 * The input samples are taken from the sample_data[] array starting at
 * position start_row/start_col, and moving to the right for any additional
 * blocks. The quantized coefficients are returned in coef_blocks[].
 */

unsafe extern "C" fn forward_DCT(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut coef_blocks: crate::jpeglib_h::JBLOCKROW,
    mut start_row: crate::jmorecfg_h::JDIMENSION,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
    mut num_blocks: crate::jmorecfg_h::JDIMENSION,
)
/* This version is used for integer DCT implementations. */
{
    /* This routine is heavily used, so it's worth coding it tightly. */
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr; /* work area for FDCT subroutine */
    let mut do_dct: crate::jdct_h::forward_DCT_method_ptr =
        (*fdct).do_dct[(*compptr).component_index as usize]; /* fold in the vertical offset once */
    let mut divisors: *mut crate::jdct_h::DCTELEM =
        (*fdct).divisors[(*compptr).quant_tbl_no as usize];
    let mut workspace: [crate::jdct_h::DCTELEM; 64] = [0; 64];
    let mut bi: crate::jmorecfg_h::JDIMENSION = 0;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0;
    while bi < num_blocks {
        /* Perform the DCT */
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace.as_mut_ptr(),
            sample_data,
            start_col,
        );
        /* Quantize/descale the coefficients, and store into coef_blocks[] */
        let mut temp: crate::jdct_h::DCTELEM = 0;
        let mut qval: crate::jdct_h::DCTELEM = 0;
        let mut i: i32 = 0;
        let mut output_ptr: crate::jpeglib_h::JCOEFPTR =
            (*coef_blocks.offset(bi as isize)).as_mut_ptr();

        for i in 0..64 {
            qval = *divisors.offset(i as isize);

            temp = workspace[i as usize];

            if temp < 0 {
                temp = -temp; /* for rounding */
                temp += qval >> 1; /* for rounding */
                if temp >= qval {
                    temp /= qval
                } else {
                    temp = 0
                }
                temp = -temp
            } else {
                temp += qval >> 1;
                if temp >= qval {
                    temp /= qval
                } else {
                    temp = 0
                }
            }

            *output_ptr.offset(i as isize) = temp as crate::jmorecfg_h::JCOEF;
        }
        bi = bi.wrapping_add(1);
        start_col = (start_col).wrapping_add((*compptr).DCT_h_scaled_size as u32)
    }
}

unsafe extern "C" fn forward_DCT_float(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut coef_blocks: crate::jpeglib_h::JBLOCKROW,
    mut start_row: crate::jmorecfg_h::JDIMENSION,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
    mut num_blocks: crate::jmorecfg_h::JDIMENSION,
)
/* This version is used for floating-point DCT implementations. */
{
    /* This routine is heavily used, so it's worth coding it tightly. */
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr; /* work area for FDCT subroutine */
    let mut do_dct: crate::jdct_h::float_DCT_method_ptr =
        (*fdct).do_float_dct[(*compptr).component_index as usize]; /* fold in the vertical offset once */
    let mut divisors: *mut f32 = (*fdct).float_divisors[(*compptr).quant_tbl_no as usize];
    let mut workspace: [f32; 64] = [0.; 64];
    let mut bi: crate::jmorecfg_h::JDIMENSION = 0;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0;
    while bi < num_blocks {
        /* Perform the DCT */
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace.as_mut_ptr(),
            sample_data,
            start_col,
        );
        /* Quantize/descale the coefficients, and store into coef_blocks[] */
        let mut temp: f32 = 0.;
        let mut i: i32 = 0;
        let mut output_ptr: crate::jpeglib_h::JCOEFPTR =
            (*coef_blocks.offset(bi as isize)).as_mut_ptr();

        for i in 0..64 {
            /* Apply the quantization and scaling factor */
            temp = workspace[i as usize] * *divisors.offset(i as isize);

            *output_ptr.offset(i as isize) =
                ((temp + 16384.5) as i32 - 16384) as crate::jmorecfg_h::JCOEF;
        }
        bi = bi.wrapping_add(1);
        start_col = (start_col).wrapping_add((*compptr).DCT_h_scaled_size as u32)
    }
}
/* DCT_FLOAT_SUPPORTED */
/*
 * Initialize for a processing pass.
 * Verify that all referenced Q-tables are present, and set up
 * the divisor table for each one.
 * In the current implementation, DCT of all components is done during
 * the first pass, even if only some components will be output in the
 * first scan.  Hence all components should be examined here.
 */

unsafe extern "C" fn start_pass_fdctmgr(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut ci: i32 = 0;
    let mut qtblno: i32 = 0;
    let mut i: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut method: i32 = 0;
    let mut qtbl: *mut crate::jpeglib_h::JQUANT_TBL = 0 as *mut crate::jpeglib_h::JQUANT_TBL;
    let mut dtbl: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    ci = 0;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Select the proper DCT routine for this component's scaling */
        match ((*compptr).DCT_h_scaled_size << 8) + (*compptr).DCT_v_scaled_size {
            257 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_1x1
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            514 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x2
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            771 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_3x3
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1028 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x4
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1285 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_5x5
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1542 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x6
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1799 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_7x7
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2313 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_9x9
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2570 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_10x10
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2827 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_11x11
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            3084 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_12x12
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            3341 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_13x13
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            3598 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_14x14
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            3855 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_15x15
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            4112 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_16x16
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            4104 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_16x8
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            3591 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_14x7
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            3078 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_12x6
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2565 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_10x5
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2052 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_8x4
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1539 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x3
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1026 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x2
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            513 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x1
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2064 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_8x16
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1806 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_7x14
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1548 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x12
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1290 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_5x10
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            1032 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x8
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            774 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_3x6
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            516 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x4
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            258 => {
                (*fdct).do_dct[ci as usize] = Some(
                    crate::src::jpeg_8c::jfdctint::jpeg_fdct_1x2
                        as unsafe extern "C" fn(
                            _: *mut crate::jdct_h::DCTELEM,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as i32
            }
            2056 => match (*cinfo).dct_method {
                0 => {
                    (*fdct).do_dct[ci as usize] = Some(
                        crate::src::jpeg_8c::jfdctint::jpeg_fdct_islow
                            as unsafe extern "C" fn(
                                _: *mut crate::jdct_h::DCTELEM,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    );
                    method = crate::jpeglib_h::JDCT_ISLOW as i32
                }
                1 => {
                    (*fdct).do_dct[ci as usize] = Some(
                        crate::src::jpeg_8c::jfdctfst::jpeg_fdct_ifast
                            as unsafe extern "C" fn(
                                _: *mut crate::jdct_h::DCTELEM,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    );
                    method = crate::jpeglib_h::JDCT_IFAST as i32
                }
                2 => {
                    (*fdct).do_float_dct[ci as usize] = Some(
                        crate::src::jpeg_8c::jfdctflt::jpeg_fdct_float
                            as unsafe extern "C" fn(
                                _: *mut f32,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    );
                    method = crate::jpeglib_h::JDCT_FLOAT as i32
                }
                _ => {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_NOT_COMPILED as i32;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
            },
            _ => {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCTSIZE as i32;
                (*(*cinfo).err).msg_parm.i[0] = (*compptr).DCT_h_scaled_size;
                (*(*cinfo).err).msg_parm.i[1] = (*compptr).DCT_v_scaled_size;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        qtblno = (*compptr).quant_tbl_no;
        /* Make sure specified quantization table is present */
        if qtblno < 0 || qtblno >= 4 || (*cinfo).quant_tbl_ptrs[qtblno as usize].is_null() {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NO_QUANT_TABLE as i32;
            (*(*cinfo).err).msg_parm.i[0] = qtblno;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        qtbl = (*cinfo).quant_tbl_ptrs[qtblno as usize];
        /* Compute divisors for this quant table */
        /* We may do this more than once for same table, but it's not a big deal */
        match method {
            0 => {
                /* For LL&M IDCT method, divisors are equal to raw quantization
                 * coefficients multiplied by 8 (to counteract scaling).
                 */
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1,
                        (64usize).wrapping_mul(::std::mem::size_of::<crate::jdct_h::DCTELEM>()),
                    )
                        as *mut crate::jdct_h::DCTELEM
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0;
                while i < 64 {
                    *dtbl.offset(i as isize) =
                        ((*qtbl).quantval[i as usize] as crate::jdct_h::DCTELEM) << 3;
                    i += 1
                }
                (*fdct).pub_0.forward_DCT[ci as usize] = Some(
                    forward_DCT
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JBLOCKROW,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                )
            }
            1 => {
                /* For AA&N IDCT method, divisors are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * We apply a further scale factor of 8.
                 */
                static mut aanscales: [crate::jmorecfg_h::INT16; 64] = [
                    16384, 22725, 21407, 19266, 16384, 12873, 8867, 4520, 22725, 31521, 29692,
                    26722, 22725, 17855, 12299, 6270, 21407, 29692, 27969, 25172, 21407, 16819,
                    11585, 5906, 19266, 26722, 25172, 22654, 19266, 15137, 10426, 5315, 16384,
                    22725, 21407, 19266, 16384, 12873, 8867, 4520, 12873, 17855, 16819, 15137,
                    12873, 10114, 6967, 3552, 8867, 12299, 11585, 10426, 8867, 6967, 4799, 2446,
                    4520, 6270, 5906, 5315, 4520, 3552, 2446, 1247,
                ];
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1,
                        (64usize).wrapping_mul(::std::mem::size_of::<crate::jdct_h::DCTELEM>()),
                    )
                        as *mut crate::jdct_h::DCTELEM
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0;
                while i < 64 {
                    *dtbl.offset(i as isize) =
                        ((*qtbl).quantval[i as usize] as crate::jmorecfg_h::INT32
                            * aanscales[i as usize] as crate::jmorecfg_h::INT32
                            + ((1) << 14 - 3 - 1)
                            >> 14 - 3) as crate::jdct_h::DCTELEM;
                    i += 1
                }
                (*fdct).pub_0.forward_DCT[ci as usize] = Some(
                    forward_DCT
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JBLOCKROW,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                )
            }
            2 => {
                /* For float AA&N IDCT method, divisors are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * We apply a further scale factor of 8.
                 * What's actually stored is 1/divisor so that the inner loop can
                 * use a multiplication rather than a division.
                 */
                let mut fdtbl: *mut f32 = 0 as *mut f32;
                let mut row: i32 = 0;
                let mut col: i32 = 0;
                static mut aanscalefactor: [f64; 8] = [
                    1.0,
                    1.387039845,
                    1.306562965,
                    1.175875602,
                    1.0,
                    0.785694958,
                    0.541196100,
                    0.275899379,
                ];
                if (*fdct).float_divisors[qtblno as usize].is_null() {
                    (*fdct).float_divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1,
                        (64usize).wrapping_mul(::std::mem::size_of::<f32>()),
                    ) as *mut f32
                }
                fdtbl = (*fdct).float_divisors[qtblno as usize];
                i = 0;

                for row in 0..8 {
                    for col in 0..8 {
                        *fdtbl.offset(i as isize) = (1.0
                            / ((*qtbl).quantval[i as usize] as f64
                                * aanscalefactor[row as usize]
                                * aanscalefactor[col as usize]
                                * 8.0)) as f32;

                        i += 1;
                    }
                }
                (*fdct).pub_0.forward_DCT[ci as usize] = Some(
                    forward_DCT_float
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JBLOCKROW,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                )
            }
            _ => {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NOT_COMPILED as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize FDCT manager.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_forward_dct(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut fdct: my_fdct_ptr = 0 as *mut my_fdct_controller;
    let mut i: i32 = 0;
    fdct = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        ::std::mem::size_of::<my_fdct_controller>(),
    ) as my_fdct_ptr;
    (*cinfo).fdct = fdct as *mut crate::jpegint_h::jpeg_forward_dct;
    (*fdct).pub_0.start_pass =
        Some(start_pass_fdctmgr as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    /* Mark divisor tables unallocated */
    i = 0;
    while i < 4 {
        (*fdct).divisors[i as usize] = 0 as *mut crate::jdct_h::DCTELEM;
        (*fdct).float_divisors[i as usize] = 0 as *mut f32;
        i += 1
    }
}
