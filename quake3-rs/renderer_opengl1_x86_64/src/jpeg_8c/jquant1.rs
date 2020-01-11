use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT16;
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
pub use crate::src::jpeg_8c::jutils::jzero_far;
/* use 'int' for calculation temps */

pub type FSERRPTR = *mut FSERROR;
/* Declarations for Floyd-Steinberg dithering.
 *
 * Errors are accumulated into the array fserrors[], at a resolution of
 * 1/16th of a pixel count.  The error at a given pixel is propagated
 * to its not-yet-processed neighbors using the standard F-S fractions,
 *		...	(here)	7/16
 *		3/16	5/16	1/16
 * We work left-to-right on even rows, right-to-left on odd rows.
 *
 * We can get away with a single array (holding one row's worth of errors)
 * by using it to store the current row's errors at pixel columns not yet
 * processed, but the next row's errors at columns already processed.  We
 * need only a few extra variables to hold the errors immediately around the
 * current column.  (If we are lucky, those variables are in registers, but
 * even if not, they're probably cheaper to access than array elements are.)
 *
 * The fserrors[] array is indexed [component#][position].
 * We provide (#columns + 2) entries per component; the extra entry at each
 * end saves us from special-casing the first and last pixels.
 *
 * Note: on a wide image, we might not have enough room in a PC's near data
 * segment to hold the error array; so it is allocated with alloc_large.
 */

pub type FSERROR = crate::jmorecfg_h::INT16;

pub type my_cquantize_ptr = *mut my_cquantizer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_cquantizer {
    pub pub_0: crate::jpegint_h::jpeg_color_quantizer,
    pub sv_colormap: crate::jpeglib_h::JSAMPARRAY,
    pub sv_actual: i32,
    pub colorindex: crate::jpeglib_h::JSAMPARRAY,
    pub is_padded: crate::jmorecfg_h::boolean,
    pub Ncolors: [i32; 4],
    pub row_index: i32,
    pub odither: [ODITHER_MATRIX_PTR; 4],
    pub fserrors: [FSERRPTR; 4],
    pub on_odd_row: crate::jmorecfg_h::boolean,
}

pub type ODITHER_MATRIX_PTR = *mut [i32; 16];
/* 16 bits should be enough */

pub type LOCFSERROR = i32;
/* mask for wrapping around counters */

pub type ODITHER_MATRIX = [[i32; 16]; 16];

static mut base_dither_matrix: [[crate::jmorecfg_h::UINT8; 16]; 16] = [
    [
        0, 192, 48, 240, 12, 204, 60, 252, 3, 195, 51, 243, 15, 207, 63, 255,
    ],
    [
        128, 64, 176, 112, 140, 76, 188, 124, 131, 67, 179, 115, 143, 79, 191, 127,
    ],
    [
        32, 224, 16, 208, 44, 236, 28, 220, 35, 227, 19, 211, 47, 239, 31, 223,
    ],
    [
        160, 96, 144, 80, 172, 108, 156, 92, 163, 99, 147, 83, 175, 111, 159, 95,
    ],
    [
        8, 200, 56, 248, 4, 196, 52, 244, 11, 203, 59, 251, 7, 199, 55, 247,
    ],
    [
        136, 72, 184, 120, 132, 68, 180, 116, 139, 75, 187, 123, 135, 71, 183, 119,
    ],
    [
        40, 232, 24, 216, 36, 228, 20, 212, 43, 235, 27, 219, 39, 231, 23, 215,
    ],
    [
        168, 104, 152, 88, 164, 100, 148, 84, 171, 107, 155, 91, 167, 103, 151, 87,
    ],
    [
        2, 194, 50, 242, 14, 206, 62, 254, 1, 193, 49, 241, 13, 205, 61, 253,
    ],
    [
        130, 66, 178, 114, 142, 78, 190, 126, 129, 65, 177, 113, 141, 77, 189, 125,
    ],
    [
        34, 226, 18, 210, 46, 238, 30, 222, 33, 225, 17, 209, 45, 237, 29, 221,
    ],
    [
        162, 98, 146, 82, 174, 110, 158, 94, 161, 97, 145, 81, 173, 109, 157, 93,
    ],
    [
        10, 202, 58, 250, 6, 198, 54, 246, 9, 201, 57, 249, 5, 197, 53, 245,
    ],
    [
        138, 74, 186, 122, 134, 70, 182, 118, 137, 73, 185, 121, 133, 69, 181, 117,
    ],
    [
        42, 234, 26, 218, 38, 230, 22, 214, 41, 233, 25, 217, 37, 229, 21, 213,
    ],
    [
        170, 106, 154, 90, 166, 102, 150, 86, 169, 105, 153, 89, 165, 101, 149, 85,
    ],
];
/*
 * Policy-making subroutines for create_colormap and create_colorindex.
 * These routines determine the colormap to be used.  The rest of the module
 * only assumes that the colormap is orthogonal.
 *
 *  * select_ncolors decides how to divvy up the available colors
 *    among the components.
 *  * output_value defines the set of representative values for a component.
 *  * largest_input_value defines the mapping from input values to
 *    representative values for a component.
 * Note that the latter two routines may impose different policies for
 * different components, though this is not currently done.
 */

unsafe extern "C" fn select_ncolors(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut Ncolors: *mut i32,
) -> i32
/* Determine allocation of desired colors to components, */
/* and fill in Ncolors[] array to indicate choice. */
/* Return value is total number of colors (product of Ncolors[] values). */ {
    let mut nc: i32 = (*cinfo).out_color_components; /* number of color components */
    let mut max_colors: i32 = (*cinfo).desired_number_of_colors;
    let mut total_colors: i32 = 0;
    let mut iroot: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut changed: crate::jmorecfg_h::boolean = 0;
    let mut temp: isize = 0;
    static mut RGB_order: [i32; 3] = [1, 0, 2];
    /* We can allocate at least the nc'th root of max_colors per component. */
    /* Compute floor(nc'th root of max_colors). */
    iroot = 1; /* repeat till iroot exceeds root */
    loop {
        iroot += 1; /* set temp = iroot ** nc */
        temp = iroot as isize; /* now iroot = floor(root) */
        i = 1;
        while i < nc {
            temp *= iroot as isize;
            i += 1
        }
        if !(temp <= max_colors as isize) {
            break;
        }
    }
    iroot -= 1;
    /* Must have at least 2 color values per component */
    if iroot < 2 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_QUANT_FEW_COLORS as i32;
        (*(*cinfo).err).msg_parm.i[0] = temp as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Initialize to iroot color values for each component */
    total_colors = 1;
    i = 0;
    while i < nc {
        *Ncolors.offset(i as isize) = iroot;
        total_colors *= iroot;
        i += 1
    }
    loop
    /* We may be able to increment the count for one or more components without
     * exceeding max_colors, though we know not all can be incremented.
     * Sometimes, the first component can be incremented more than once!
     * (Example: for 16 colors, we start at 2*2*2, go to 3*2*2, then 4*2*2.)
     * In RGB colorspace, try to increment G first, then R, then B.
     */
    {
        changed = 0;
        i = 0;
        while i < nc {
            j = if (*cinfo).out_color_space == crate::jpeglib_h::JCS_RGB {
                RGB_order[i as usize]
            } else {
                i
            };
            /* calculate new total_colors if Ncolors[j] is incremented */
            temp = (total_colors / *Ncolors.offset(j as isize)) as isize; /* done in long arith to avoid oflo */
            temp *= (*Ncolors.offset(j as isize) + 1i32) as isize; /* won't fit, done with this pass */
            if temp > max_colors as isize {
                break; /* OK, apply the increment */
            }
            let ref mut fresh0 = *Ncolors.offset(j as isize);
            *fresh0 += 1;
            total_colors = temp as i32;
            changed = 1;
            i += 1
        }
        if !(changed != 0) {
            break;
        }
    }
    return total_colors;
}

unsafe extern "C" fn output_value(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut ci: i32,
    mut j: i32,
    mut maxj: i32,
) -> i32
/* Return j'th output value, where j will range from 0 to maxj */
/* The output values must fall in 0..MAXJSAMPLE in increasing order */ {
    /* We always provide values 0 and MAXJSAMPLE for each component;
     * any additional values are equally spaced between these limits.
     * (Forcing the upper and lower values to the limits ensures that
     * dithering can't produce a color outside the selected gamut.)
     */
    return ((j as crate::jmorecfg_h::INT32 * 255 + (maxj / 2i32) as isize) / maxj as isize) as i32;
}

unsafe extern "C" fn largest_input_value(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut ci: i32,
    mut j: i32,
    mut maxj: i32,
) -> i32
/* Return largest input value that should map to j'th output value */
/* Must have largest(j=0) >= 0, and largest(j=maxj) >= MAXJSAMPLE */ {
    /* Breakpoints are halfway between values returned by output_value */
    return (((2i32 * j + 1) as crate::jmorecfg_h::INT32 * 255 + maxj as isize)
        / (2i32 * maxj) as isize) as i32;
}
/*
 * Create the colormap.
 */

unsafe extern "C" fn create_colormap(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* Created colormap */
    let mut colormap: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW; /* Number of distinct output colors */
    let mut total_colors: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nci: i32 = 0;
    let mut blksize: i32 = 0;
    let mut blkdist: i32 = 0;
    let mut ptr: i32 = 0;
    let mut val: i32 = 0;
    /* Select number of colors for each component */
    total_colors = select_ncolors(cinfo, (*cquantize).Ncolors.as_mut_ptr());
    /* Report selected color counts */
    if (*cinfo).out_color_components == 3 {
        let mut _mp: *mut i32 = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = total_colors;
        *_mp.offset(1) = (*cquantize).Ncolors[0];
        *_mp.offset(2) = (*cquantize).Ncolors[1];
        *_mp.offset(3) = (*cquantize).Ncolors[2];
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_QUANT_3_NCOLORS as i32;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    } else {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_QUANT_NCOLORS as i32;
        (*(*cinfo).err).msg_parm.i[0] = total_colors;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    }
    /* Allocate and fill in the colormap. */
    /* The colors are ordered in the map in standard row-major order, */
    /* i.e. rightmost (highest-indexed) color changes most rapidly. */
    colormap = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        total_colors as crate::jmorecfg_h::JDIMENSION,
        (*cinfo).out_color_components as crate::jmorecfg_h::JDIMENSION,
    );
    /* blksize is number of adjacent repeated entries for a component */
    /* blkdist is distance between groups of identical entries for a component */
    blkdist = total_colors;

    for i in 0..(*cinfo).out_color_components {
        nci = (*cquantize).Ncolors[i as usize];

        blksize = blkdist / nci;
        for j in 0..nci {
            val = output_value(cinfo, i, j, nci - 1);
            for ptr in (j * blksize..total_colors).step_by(blkdist as usize) {
                for k in 0..blksize {
                    *(*colormap.offset(i as isize)).offset((ptr + k) as isize) =
                        val as crate::jmorecfg_h::JSAMPLE;
                }
            }
        }

        blkdist = blksize;
    }
    /* Save the colormap in private storage,
     * where it will survive color quantization mode changes.
     */
    (*cquantize).sv_colormap = colormap;
    (*cquantize).sv_actual = total_colors;
}
/*
 * Create the color index table.
 */

unsafe extern "C" fn create_colorindex(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut indexptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nci: i32 = 0;
    let mut blksize: i32 = 0;
    let mut val: i32 = 0;
    let mut pad: i32 = 0;
    /* For ordered dither, we pad the color index tables by MAXJSAMPLE in
     * each direction (input index values can be -MAXJSAMPLE .. 2*MAXJSAMPLE).
     * This is not necessary in the other dithering modes.  However, we
     * flag whether it was done in case user changes dithering mode.
     */
    if (*cinfo).dither_mode == crate::jpeglib_h::JDITHER_ORDERED {
        pad = 255 * 2;
        (*cquantize).is_padded = 1
    } else {
        pad = 0;
        (*cquantize).is_padded = 0
    }
    (*cquantize).colorindex = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        (255 + 1 + pad) as crate::jmorecfg_h::JDIMENSION,
        (*cinfo).out_color_components as crate::jmorecfg_h::JDIMENSION,
    );
    /* blksize is number of adjacent repeated entries for a component */
    blksize = (*cquantize).sv_actual;
    i = 0;
    while i < (*cinfo).out_color_components {
        /* fill in colorindex entries for i'th color component */
        nci = (*cquantize).Ncolors[i as usize]; /* # of distinct values for this color */
        blksize = blksize / nci;
        /* adjust colorindex pointers to provide padding at negative indexes. */
        if pad != 0 {
            let ref mut fresh1 = *(*cquantize).colorindex.offset(i as isize);
            *fresh1 = (*fresh1).offset(255)
        }
        /* in loop, val = index of current output value, */
        /* and k = largest j that maps to current val */
        indexptr = *(*cquantize).colorindex.offset(i as isize);
        val = 0;
        k = largest_input_value(cinfo, i, 0, nci - 1);
        j = 0;
        while j <= 255 {
            while j > k {
                /* advance val if past boundary */
                val += 1;
                k = largest_input_value(cinfo, i, val, nci - 1)
            }
            /* premultiply so that no multiplication needed in main processing */
            *indexptr.offset(j as isize) = (val * blksize) as crate::jmorecfg_h::JSAMPLE;
            j += 1
        }
        /* Pad at both ends if necessary */
        if pad != 0 {
            j = 1;
            while j <= 255 {
                *indexptr.offset(-j as isize) = *indexptr.offset(0);
                *indexptr.offset((255 + j) as isize) = *indexptr.offset(255);
                j += 1
            }
        }
        i += 1
    }
}
/*
 * Create an ordered-dither array for a component having ncolors
 * distinct output values.
 */

unsafe extern "C" fn make_odither_array(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut ncolors: i32,
) -> ODITHER_MATRIX_PTR {
    let mut odither: ODITHER_MATRIX_PTR = 0 as *mut [i32; 16];
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut num: crate::jmorecfg_h::INT32 = 0;
    let mut den: crate::jmorecfg_h::INT32 = 0;
    odither = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        ::std::mem::size_of::<ODITHER_MATRIX>(),
    ) as ODITHER_MATRIX_PTR;
    /* The inter-value distance for this color is MAXJSAMPLE/(ncolors-1).
     * Hence the dither value for the matrix cell with fill order f
     * (f=0..N-1) should be (N-1-2*f)/(2*N) * MAXJSAMPLE/(ncolors-1).
     * On 16-bit-int machine, be careful to avoid overflow.
     */
    den = (2i32 * (16 * 16)) as isize * (ncolors - 1i32) as crate::jmorecfg_h::INT32;

    for j in 0..16 {
        for k in 0..16 {
            num = (16 * 16 - 1 - 2 * base_dither_matrix[j as usize][k as usize] as i32)
                as crate::jmorecfg_h::INT32
                * 255;

            (*odither.offset(j as isize))[k as usize] =
                if num < 0 { -(-num / den) } else { (num) / den } as i32;
        }
    }
    return odither;
}
/*
 * Create the ordered-dither tables.
 * Components having the same number of representative colors may
 * share a dither table.
 */

unsafe extern "C" fn create_odither_tables(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* # of distinct values for this color */
    let mut odither: ODITHER_MATRIX_PTR = 0 as *mut [i32; 16]; /* search for matching prior component */
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nci: i32 = 0;
    i = 0;
    while i < (*cinfo).out_color_components {
        nci = (*cquantize).Ncolors[i as usize];
        odither = 0 as ODITHER_MATRIX_PTR;
        j = 0;
        while j < i {
            if nci == (*cquantize).Ncolors[j as usize] {
                odither = (*cquantize).odither[j as usize];
                break;
            } else {
                j += 1
            }
        }
        if odither.is_null() {
            /* need a new table? */
            odither = make_odither_array(cinfo, nci)
        }
        (*cquantize).odither[i as usize] = odither;
        i += 1
    }
}
/*
 * Map some rows of pixels to the output colormapped representation.
 */

unsafe extern "C" fn color_quantize(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
)
/* General case, no dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut colorindex: crate::jpeglib_h::JSAMPARRAY = (*cquantize).colorindex;
    let mut pixcode: i32 = 0;
    let mut ci: i32 = 0;
    let mut ptrin: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ptrout: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut row: i32 = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut nc: i32 = (*cinfo).out_color_components;
    row = 0;
    while row < num_rows {
        ptrin = *input_buf.offset(row as isize);
        ptrout = *output_buf.offset(row as isize);
        col = width;
        while col > 0 {
            pixcode = 0;

            for ci in 0..nc {
                let fresh2 = ptrin;

                ptrin = ptrin.offset(1);

                pixcode +=
                    *(*colorindex.offset(ci as isize)).offset(*fresh2 as i32 as isize) as i32;
            }
            let fresh3 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh3 = pixcode as crate::jmorecfg_h::JSAMPLE;
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}

unsafe extern "C" fn color_quantize3(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
)
/* Fast path for out_color_components==3, no dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut pixcode: i32 = 0;
    let mut ptrin: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ptrout: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex0: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(0);
    let mut colorindex1: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(1);
    let mut colorindex2: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(2);
    let mut row: i32 = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0;
    while row < num_rows {
        ptrin = *input_buf.offset(row as isize);
        ptrout = *output_buf.offset(row as isize);
        col = width;
        while col > 0 {
            let fresh4 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode = *colorindex0.offset(*fresh4 as i32 as isize) as i32;
            let fresh5 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex1.offset(*fresh5 as i32 as isize) as i32;
            let fresh6 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex2.offset(*fresh6 as i32 as isize) as i32;
            let fresh7 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh7 = pixcode as crate::jmorecfg_h::JSAMPLE;
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}

unsafe extern "C" fn quantize_ord_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
)
/* General case, with ordered dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* points to active row of dither matrix */
    let mut input_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE; /* current indexes into dither matrix */
    let mut output_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex_ci: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut dither: *mut i32 = 0 as *mut i32;
    let mut row_index: i32 = 0;
    let mut col_index: i32 = 0;
    let mut nc: i32 = (*cinfo).out_color_components;
    let mut ci: i32 = 0;
    let mut row: i32 = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0;
    while row < num_rows {
        /* Initialize output values to 0 so can process components separately */
        crate::src::jpeg_8c::jutils::jzero_far(
            *output_buf.offset(row as isize) as *mut libc::c_void,
            (width as usize).wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>()),
        );
        row_index = (*cquantize).row_index;

        for ci in 0..nc {
            input_ptr = (*input_buf.offset(row as isize)).offset(ci as isize);

            output_ptr = *output_buf.offset(row as isize);

            colorindex_ci = *(*cquantize).colorindex.offset(ci as isize);

            dither = (*(*cquantize).odither[ci as usize].offset(row_index as isize)).as_mut_ptr();

            col_index = 0;

            col = width;

            while col > 0 {
                /* Form pixel value + dither, range-limit to 0..MAXJSAMPLE,
                 * select output value, accumulate into output code for this pixel.
                 * Range-limiting need not be done explicitly, as we have extended
                 * the colorindex table to produce the right answers for out-of-range
                 * inputs.  The maximum dither is +- MAXJSAMPLE; this sets the
                 * required amount of padding.
                 */
                *output_ptr = (*output_ptr as i32
                    + *colorindex_ci
                        .offset((*input_ptr as i32 + *dither.offset(col_index as isize)) as isize)
                        as i32) as crate::jmorecfg_h::JSAMPLE;
                input_ptr = input_ptr.offset(nc as isize);
                output_ptr = output_ptr.offset(1);
                col_index = col_index + 1 & 16 - 1;
                col = col.wrapping_sub(1)
            }
        }
        /* Advance row index for next row */
        row_index = row_index + 1 & 16 - 1;
        (*cquantize).row_index = row_index;
        row += 1
    }
}

unsafe extern "C" fn quantize3_ord_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
)
/* Fast path for out_color_components==3, with ordered dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* points to active row of dither matrix */
    let mut pixcode: i32 = 0; /* current indexes into dither matrix */
    let mut input_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut output_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex0: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(0);
    let mut colorindex1: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(1);
    let mut colorindex2: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(2);
    let mut dither0: *mut i32 = 0 as *mut i32;
    let mut dither1: *mut i32 = 0 as *mut i32;
    let mut dither2: *mut i32 = 0 as *mut i32;
    let mut row_index: i32 = 0;
    let mut col_index: i32 = 0;
    let mut row: i32 = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0;
    while row < num_rows {
        row_index = (*cquantize).row_index;
        input_ptr = *input_buf.offset(row as isize);
        output_ptr = *output_buf.offset(row as isize);
        dither0 = (*(*cquantize).odither[0].offset(row_index as isize)).as_mut_ptr();
        dither1 = (*(*cquantize).odither[1].offset(row_index as isize)).as_mut_ptr();
        dither2 = (*(*cquantize).odither[2].offset(row_index as isize)).as_mut_ptr();
        col_index = 0;
        col = width;
        while col > 0 {
            let fresh8 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode = *colorindex0
                .offset((*fresh8 as i32 + *dither0.offset(col_index as isize)) as isize)
                as i32;
            let fresh9 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode += *colorindex1
                .offset((*fresh9 as i32 + *dither1.offset(col_index as isize)) as isize)
                as i32;
            let fresh10 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode += *colorindex2
                .offset((*fresh10 as i32 + *dither2.offset(col_index as isize)) as isize)
                as i32;
            let fresh11 = output_ptr;
            output_ptr = output_ptr.offset(1);
            *fresh11 = pixcode as crate::jmorecfg_h::JSAMPLE;
            col_index = col_index + 1 & 16 - 1;
            col = col.wrapping_sub(1)
        }
        row_index = row_index + 1 & 16 - 1;
        (*cquantize).row_index = row_index;
        row += 1
    }
}

unsafe extern "C" fn quantize_fs_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
)
/* General case, with Floyd-Steinberg dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* current error or pixel value */
    let mut cur: LOCFSERROR = 0; /* error for pixel below cur */
    let mut belowerr: LOCFSERROR = 0; /* error for below/prev col */
    let mut bpreverr: LOCFSERROR = 0; /* error for below/next col */
    let mut bnexterr: LOCFSERROR = 0; /* => fserrors[] at column before current */
    let mut delta: LOCFSERROR = 0; /* 1 for left-to-right, -1 for right-to-left */
    let mut errorptr: FSERRPTR = 0 as *mut FSERROR; /* dir * nc */
    let mut input_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut output_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex_ci: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colormap_ci: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut pixcode: i32 = 0;
    let mut nc: i32 = (*cinfo).out_color_components;
    let mut dir: i32 = 0;
    let mut dirnc: i32 = 0;
    let mut ci: i32 = 0;
    let mut row: i32 = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    row = 0;
    while row < num_rows {
        /* Initialize output values to 0 so can process components separately */
        crate::src::jpeg_8c::jutils::jzero_far(
            *output_buf.offset(row as isize) as *mut libc::c_void,
            (width as usize).wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>()),
        );

        for ci in 0..nc {
            input_ptr = (*input_buf.offset(row as isize)).offset(ci as isize);

            output_ptr = *output_buf.offset(row as isize);

            if (*cquantize).on_odd_row != 0 {
                /* unload prev err into array */
                /* work right to left in this row */
                input_ptr =
                    input_ptr.offset(width.wrapping_sub(1u32).wrapping_mul(nc as u32) as isize);
                output_ptr = output_ptr.offset(width.wrapping_sub(1u32) as isize);
                dir = -(1);
                dirnc = -nc;
                errorptr =
                    (*cquantize).fserrors[ci as usize].offset(width.wrapping_add(1u32) as isize)
            /* so point to rightmost pixel */
            /* => entry after last column */
            } else {
                /* work left to right in this row */
                dir = 1;
                dirnc = nc;
                errorptr = (*cquantize).fserrors[ci as usize]
                /* => entry before first column */
            }

            colorindex_ci = *(*cquantize).colorindex.offset(ci as isize);

            colormap_ci = *(*cquantize).sv_colormap.offset(ci as isize);

            cur = 0;

            bpreverr = 0;

            belowerr = bpreverr;

            col = width;

            while col > 0 {
                /* Preset error values: no error propagated to first pixel from left */
                /* and no error propagated to row below yet */
                /* cur holds the error propagated from the previous pixel on the
                 * current line.  Add the error propagated from the previous line
                 * to form the complete error correction term for this pixel, and
                 * round the error term (which is expressed * 16) to an integer.
                 * RIGHT_SHIFT rounds towards minus infinity, so adding 8 is correct
                 * for either sign of the error value.
                 * Note: errorptr points to *previous* column's array entry.
                 */
                cur = cur + *errorptr.offset(dir as isize) as i32 + 8 >> 4;
                /* advance errorptr to current column */
                cur += *input_ptr as i32;
                cur = *range_limit.offset(cur as isize) as i32;
                pixcode = *colorindex_ci.offset(cur as isize) as i32;
                *output_ptr = (*output_ptr as i32 + pixcode as crate::jmorecfg_h::JSAMPLE as i32)
                    as crate::jmorecfg_h::JSAMPLE;
                cur -= *colormap_ci.offset(pixcode as isize) as i32;
                bnexterr = cur;
                delta = cur * 2;
                cur += delta;
                *errorptr.offset(0) = (bpreverr + cur) as FSERROR;
                cur += delta;
                bpreverr = belowerr + cur;
                belowerr = bnexterr;
                cur += delta;
                input_ptr = input_ptr.offset(dirnc as isize);
                output_ptr = output_ptr.offset(dir as isize);
                errorptr = errorptr.offset(dir as isize);
                col = col.wrapping_sub(1)
            }

            *errorptr.offset(0) = bpreverr as FSERROR;
        }
        (*cquantize).on_odd_row = if (*cquantize).on_odd_row != 0 { 0 } else { 1 };
        row += 1
    }
}
/* Form pixel value + error, and range-limit to 0..MAXJSAMPLE.
 * The maximum error is +- MAXJSAMPLE; this sets the required size
 * of the range_limit array.
 */
/* Select output value, accumulate into output code for this pixel */
/* Compute actual representation error at this pixel */
/* Note: we can do this even though we don't have the final */
/* pixel code, because the colormap is orthogonal. */
/* Compute error fractions to be propagated to adjacent pixels.
 * Add these into the running sums, and simultaneously shift the
 * next-line error sums left by 1 column.
 */
/* form error * 3 */
/* form error * 5 */
/* form error * 7 */
/* At this point cur contains the 7/16 error value to be propagated
 * to the next pixel on the current line, and all the errors for the
 * next line have been shifted over. We are therefore ready to move on.
 */
/* advance input ptr to next column */
/* advance output ptr to next column */
/* Post-loop cleanup: we must unload the final error value into the
 * final fserrors[] entry.  Note we need not unload belowerr because
 * it is for the dummy column before or after the actual array.
 */
/*
 * Allocate workspace for Floyd-Steinberg errors.
 */

unsafe extern "C" fn alloc_fs_workspace(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut arraysize: crate::stddef_h::size_t = 0;
    let mut i: i32 = 0;
    arraysize = ((*cinfo).output_width.wrapping_add(2u32) as usize)
        .wrapping_mul(::std::mem::size_of::<FSERROR>());
    i = 0;
    while i < (*cinfo).out_color_components {
        (*cquantize).fserrors[i as usize] = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr, 1, arraysize
        ) as FSERRPTR;
        i += 1
    }
}
/*
 * Initialize for one-pass color quantization.
 */

unsafe extern "C" fn start_pass_1_quant(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut is_pre_scan: crate::jmorecfg_h::boolean,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut arraysize: crate::stddef_h::size_t = 0;
    let mut i: i32 = 0;
    /* Install my colormap. */
    (*cinfo).colormap = (*cquantize).sv_colormap;
    (*cinfo).actual_number_of_colors = (*cquantize).sv_actual;
    /* Initialize for desired dithering mode. */
    match (*cinfo).dither_mode {
        0 => {
            if (*cinfo).out_color_components == 3 {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize3
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            }
        }
        1 => {
            if (*cinfo).out_color_components == 3 {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize3_ord_dither
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize_ord_dither
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } /* initialize state for ordered dither */
            (*cquantize).row_index = 0;
            /* If user changed to ordered dither from another mode,
             * we must recreate the color index table with padding.
             * This will cost extra space, but probably isn't very likely.
             */
            if (*cquantize).is_padded == 0 {
                create_colorindex(cinfo);
            }
            /* Create ordered-dither tables if we didn't already. */
            if (*cquantize).odither[0].is_null() {
                create_odither_tables(cinfo); /* initialize state for F-S dither */
            }
        }
        2 => {
            (*cquantize).pub_0.color_quantize = Some(
                quantize_fs_dither
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: i32,
                    ) -> (),
            );
            (*cquantize).on_odd_row = 0;
            /* Allocate Floyd-Steinberg workspace if didn't already. */
            if (*cquantize).fserrors[0].is_null() {
                alloc_fs_workspace(cinfo);
            }
            /* Initialize the propagated errors to zero. */
            arraysize = ((*cinfo).output_width.wrapping_add(2u32) as usize)
                .wrapping_mul(::std::mem::size_of::<FSERROR>());
            i = 0;
            while i < (*cinfo).out_color_components {
                crate::src::jpeg_8c::jutils::jzero_far(
                    (*cquantize).fserrors[i as usize] as *mut libc::c_void,
                    arraysize,
                );
                i += 1
            }
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
    };
}
/*
 * Finish up at the end of the pass.
 */

unsafe extern "C" fn finish_pass_1_quant(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    /* no work in 1-pass case */
}
/*
 * Switch to a new external colormap between output passes.
 * Shouldn't get to this module!
 */

unsafe extern "C" fn new_color_map_1_quant(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_MODE_CHANGE as i32;
    Some(
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
}
/*
 * Module initialization routine for 1-pass color quantization.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_1pass_quantizer(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = 0 as *mut my_cquantizer; /* Flag FS workspace not allocated */
    cquantize = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        ::std::mem::size_of::<my_cquantizer>(),
    ) as my_cquantize_ptr; /* Also flag odither arrays not allocated */
    (*cinfo).cquantize = cquantize as *mut crate::jpegint_h::jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass = Some(
        start_pass_1_quant
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jmorecfg_h::boolean,
            ) -> (),
    );
    (*cquantize).pub_0.finish_pass = Some(
        finish_pass_1_quant as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_1_quant as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*cquantize).fserrors[0] = 0 as FSERRPTR;
    (*cquantize).odither[0] = 0 as ODITHER_MATRIX_PTR;
    /* Make sure my internal arrays won't overflow */
    if (*cinfo).out_color_components > 4 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_QUANT_COMPONENTS as i32;
        (*(*cinfo).err).msg_parm.i[0] = 4;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Make sure colormap indexes can be represented by JSAMPLEs */
    if (*cinfo).desired_number_of_colors > 255 + 1 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_QUANT_MANY_COLORS as i32;
        (*(*cinfo).err).msg_parm.i[0] = 255 + 1;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Create the colormap and color index table. */
    create_colormap(cinfo);
    create_colorindex(cinfo);
    /* Allocate Floyd-Steinberg workspace now if requested.
     * We do this now since it is FAR storage and may affect the memory
     * manager's space calculations.  If the user changes to FS dither
     * mode in a later pass, we will allocate the space then, and will
     * possibly overrun the max_memory_to_use setting.
     */
    if (*cinfo).dither_mode == crate::jpeglib_h::JDITHER_FS {
        alloc_fs_workspace(cinfo);
    };
}
/* QUANT_1PASS_SUPPORTED */
