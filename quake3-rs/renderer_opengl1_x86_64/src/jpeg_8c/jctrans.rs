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
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
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
pub use crate::src::jpeg_8c::jcapimin::jpeg_suppress_tables;
pub use crate::src::jpeg_8c::jcarith::jinit_arith_encoder;
pub use crate::src::jpeg_8c::jchuff::jinit_huff_encoder;
pub use crate::src::jpeg_8c::jcmarker::jinit_marker_writer;
pub use crate::src::jpeg_8c::jcmaster::jinit_c_master_control;
pub use crate::src::jpeg_8c::jcomapi::jpeg_alloc_quant_table;
pub use crate::src::jpeg_8c::jcparam::jpeg_set_colorspace;
pub use crate::src::jpeg_8c::jcparam::jpeg_set_defaults;
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

pub type my_coef_ptr = *mut my_coef_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: crate::jpegint_h::jpeg_c_coef_controller,
    pub iMCU_row_num: crate::jmorecfg_h::JDIMENSION,
    pub mcu_ctr: crate::jmorecfg_h::JDIMENSION,
    pub MCU_vert_offset: i32,
    pub MCU_rows_per_iMCU_row: i32,
    pub whole_image: *mut crate::jpeglib_h::jvirt_barray_ptr,
    pub dummy_buffer: [crate::jpeglib_h::JBLOCKROW; 10],
}
/*
 * Compression initialization for writing raw-coefficient data.
 * Before calling this, all parameters and a data destination must be set up.
 * Call jpeg_finish_compress() to actually write the data.
 *
 * The number of passed virtual arrays must match cinfo->num_components.
 * Note that the virtual arrays need not be filled or even realized at
 * the time write_coefficients is called; indeed, if the virtual arrays
 * were requested from this compression object's memory manager, they
 * typically will be realized during this routine and filled afterwards.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_write_coefficients(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
) {
    if (*cinfo).global_state != 100 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Mark all tables to be written */
    crate::src::jpeg_8c::jcapimin::jpeg_suppress_tables(cinfo, 0);
    /* (Re)initialize error mgr and destination modules */
    Some(
        (*(*cinfo).err)
            .reset_error_mgr
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    Some(
        (*(*cinfo).dest)
            .init_destination
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Perform master selection of active modules */
    transencode_master_selection(cinfo, coef_arrays);
    /* Wait for jpeg_finish_compress() call */
    (*cinfo).next_scanline = 0u32; /* so jpeg_write_marker works */
    (*cinfo).global_state = 103;
}
/*
 * Initialize the compression object with default parameters,
 * then copy from the source object all parameters needed for lossless
 * transcoding.  Parameters that can be varied without loss (such as
 * scan script and Huffman optimization) are left in their default states.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_copy_critical_parameters(
    mut srcinfo: crate::jpeglib_h::j_decompress_ptr,
    mut dstinfo: crate::jpeglib_h::j_compress_ptr,
) {
    let mut qtblptr: *mut *mut crate::jpeglib_h::JQUANT_TBL =
        0 as *mut *mut crate::jpeglib_h::JQUANT_TBL;
    let mut incomp: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut outcomp: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut c_quant: *mut crate::jpeglib_h::JQUANT_TBL = 0 as *mut crate::jpeglib_h::JQUANT_TBL;
    let mut slot_quant: *mut crate::jpeglib_h::JQUANT_TBL = 0 as *mut crate::jpeglib_h::JQUANT_TBL;
    let mut tblno: i32 = 0;
    let mut ci: i32 = 0;
    let mut coefi: i32 = 0;
    /* Safety check to ensure start_compress not called yet. */
    if (*dstinfo).global_state != 100 {
        (*(*dstinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*dstinfo).err).msg_parm.i[0] = (*dstinfo).global_state;
        Some(
            (*(*dstinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(dstinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Copy fundamental image dimensions */
    (*dstinfo).image_width = (*srcinfo).image_width;
    (*dstinfo).image_height = (*srcinfo).image_height;
    (*dstinfo).input_components = (*srcinfo).num_components;
    (*dstinfo).in_color_space = (*srcinfo).jpeg_color_space;
    (*dstinfo).jpeg_width = (*srcinfo).output_width;
    (*dstinfo).jpeg_height = (*srcinfo).output_height;
    (*dstinfo).min_DCT_h_scaled_size = (*srcinfo).min_DCT_h_scaled_size;
    (*dstinfo).min_DCT_v_scaled_size = (*srcinfo).min_DCT_v_scaled_size;
    /* Initialize all parameters to default values */
    crate::src::jpeg_8c::jcparam::jpeg_set_defaults(dstinfo);
    /* jpeg_set_defaults may choose wrong colorspace, eg YCbCr if input is RGB.
     * Fix it to get the right header markers for the image colorspace.
     */
    crate::src::jpeg_8c::jcparam::jpeg_set_colorspace(dstinfo, (*srcinfo).jpeg_color_space);
    (*dstinfo).data_precision = (*srcinfo).data_precision;
    (*dstinfo).CCIR601_sampling = (*srcinfo).CCIR601_sampling;
    /* Copy the source's quantization tables. */
    tblno = 0;
    while tblno < 4 {
        if !(*srcinfo).quant_tbl_ptrs[tblno as usize].is_null() {
            qtblptr = &mut *(*dstinfo)
                .quant_tbl_ptrs
                .as_mut_ptr()
                .offset(tblno as isize)
                as *mut *mut crate::jpeglib_h::JQUANT_TBL;
            if (*qtblptr).is_null() {
                *qtblptr = crate::src::jpeg_8c::jcomapi::jpeg_alloc_quant_table(
                    dstinfo as crate::jpeglib_h::j_common_ptr,
                )
            }
            crate::stdlib::memcpy(
                (**qtblptr).quantval.as_mut_ptr() as *mut libc::c_void,
                (*(*srcinfo).quant_tbl_ptrs[tblno as usize])
                    .quantval
                    .as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[crate::jmorecfg_h::UINT16; 64]>(),
            );
            (**qtblptr).sent_table = 0
        }
        tblno += 1
    }
    /* Copy the source's per-component info.
     * Note we assume jpeg_set_defaults has allocated the dest comp_info array.
     */
    (*dstinfo).num_components = (*srcinfo).num_components;
    if (*dstinfo).num_components < 1 || (*dstinfo).num_components > 10 {
        (*(*dstinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT as i32;
        (*(*dstinfo).err).msg_parm.i[0] = (*dstinfo).num_components;
        (*(*dstinfo).err).msg_parm.i[1] = 10;
        Some(
            (*(*dstinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(dstinfo as crate::jpeglib_h::j_common_ptr);
    }
    ci = 0;
    incomp = (*srcinfo).comp_info;
    outcomp = (*dstinfo).comp_info;
    while ci < (*dstinfo).num_components {
        (*outcomp).component_id = (*incomp).component_id;
        (*outcomp).h_samp_factor = (*incomp).h_samp_factor;
        (*outcomp).v_samp_factor = (*incomp).v_samp_factor;
        (*outcomp).quant_tbl_no = (*incomp).quant_tbl_no;
        /* Note: we do not copy the source's Huffman table assignments;
         * instead we rely on jpeg_set_colorspace to have made a suitable choice.
         */
        tblno = (*outcomp).quant_tbl_no;
        if tblno < 0 || tblno >= 4 || (*srcinfo).quant_tbl_ptrs[tblno as usize].is_null() {
            (*(*dstinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NO_QUANT_TABLE as i32;
            (*(*dstinfo).err).msg_parm.i[0] = tblno;
            Some(
                (*(*dstinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dstinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        slot_quant = (*srcinfo).quant_tbl_ptrs[tblno as usize];
        c_quant = (*incomp).quant_table;
        if !c_quant.is_null() {
            coefi = 0;
            while coefi < 64 {
                if (*c_quant).quantval[coefi as usize] as i32
                    != (*slot_quant).quantval[coefi as usize] as i32
                {
                    (*(*dstinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_MISMATCHED_QUANT_TABLE as i32;
                    (*(*dstinfo).err).msg_parm.i[0] = tblno;
                    Some(
                        (*(*dstinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        dstinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                coefi += 1
            }
        }
        ci += 1;
        incomp = incomp.offset(1);
        outcomp = outcomp.offset(1)
    }
    /* Make sure saved quantization table for component matches the qtable
     * slot.  If not, the input file re-used this qtable slot.
     * IJG encoder currently cannot duplicate this.
     */
    /* Also copy JFIF version and resolution information, if available.
     * Strictly speaking this isn't "critical" info, but it's nearly
     * always appropriate to copy it if available.  In particular,
     * if the application chooses to copy JFIF 1.02 extension markers from
     * the source file, we need to copy the version to make sure we don't
     * emit a file that has 1.02 extensions but a claimed version of 1.01.
     * We will *not*, however, copy version info from mislabeled "2.01" files.
     */
    if (*srcinfo).saw_JFIF_marker != 0 {
        if (*srcinfo).JFIF_major_version as i32 == 1 {
            (*dstinfo).JFIF_major_version = (*srcinfo).JFIF_major_version;
            (*dstinfo).JFIF_minor_version = (*srcinfo).JFIF_minor_version
        }
        (*dstinfo).density_unit = (*srcinfo).density_unit;
        (*dstinfo).X_density = (*srcinfo).X_density;
        (*dstinfo).Y_density = (*srcinfo).Y_density
    };
}
/*
 * jctrans.c
 *
 * Copyright (C) 1995-1998, Thomas G. Lane.
 * Modified 2000-2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains library routines for transcoding compression,
 * that is, writing raw DCT coefficient arrays to an output JPEG file.
 * The routines in jcapimin.c will also be needed by a transcoder.
 */
/* Forward declarations */
/*
 * Master selection of compression modules for transcoding.
 * This substitutes for jcinit.c's initialization of the full compressor.
 */

unsafe extern "C" fn transencode_master_selection(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
) {
    /* Initialize master control (includes parameter checking/processing) */
    crate::src::jpeg_8c::jcmaster::jinit_c_master_control(cinfo, 1);
    /* Entropy encoding: either Huffman or arithmetic coding. */
    if (*cinfo).arith_code != 0 {
        crate::src::jpeg_8c::jcarith::jinit_arith_encoder(cinfo);
    } else {
        crate::src::jpeg_8c::jchuff::jinit_huff_encoder(cinfo);
    }
    /* We need a special coefficient buffer controller. */
    transencode_coef_controller(cinfo, coef_arrays);
    crate::src::jpeg_8c::jcmarker::jinit_marker_writer(cinfo);
    /* We can now tell the memory manager to allocate virtual arrays. */
    Some(
        (*(*cinfo).mem)
            .realize_virt_arrays
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    /* Write the datastream header (SOI, JFIF) immediately.
     * Frame and scan headers are postponed till later.
     * This lets application insert special markers after the SOI.
     */
    Some(
        (*(*cinfo).marker)
            .write_file_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
}

unsafe extern "C" fn start_iMCU_row(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Reset within-iMCU-row counters for a new row */
{
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* In an interleaved scan, an MCU row is the same as an iMCU row.
     * In a noninterleaved scan, an iMCU row has v_samp_factor MCU rows.
     * But at the bottom of the image, process only what's left.
     */
    if (*cinfo).comps_in_scan > 1 {
        (*coef).MCU_rows_per_iMCU_row = 1
    } else if (*coef).iMCU_row_num < (*cinfo).total_iMCU_rows.wrapping_sub(1u32) {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).last_row_height
    }
    (*coef).mcu_ctr = 0;
    (*coef).MCU_vert_offset = 0;
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_coef(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut pass_mode: crate::jpegint_h::J_BUF_MODE,
) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    if pass_mode != crate::jpegint_h::JBUF_CRANK_DEST {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*coef).iMCU_row_num = 0;
    start_iMCU_row(cinfo);
}
/*
 * Process some data.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the scan.
 * The data is obtained from the virtual arrays and fed to the entropy coder.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf is ignored; it is likely to be a NULL pointer.
 */

unsafe extern "C" fn compress_output(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut _input_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> crate::jmorecfg_h::boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut last_MCU_col: crate::jmorecfg_h::JDIMENSION = (*cinfo).MCUs_per_row.wrapping_sub(1u32);
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        (*cinfo).total_iMCU_rows.wrapping_sub(1u32);
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut xindex: i32 = 0;
    let mut _yindex: i32 = 0;
    let mut _yoffset: i32 = 0;
    let mut blockcnt: i32 = 0;
    let mut start_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut buffer: [crate::jpeglib_h::JBLOCKARRAY; 4] = [0 as *mut crate::jpeglib_h::JBLOCKROW; 4];
    let mut MCU_buffer: [crate::jpeglib_h::JBLOCKROW; 10] =
        [0 as *mut crate::jpeglib_h::JBLOCK; 10];
    let mut buffer_ptr: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    /* Align the virtual buffers for the components used in this scan. */
    ci = 0;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            *(*coef)
                .whole_image
                .offset((*compptr).component_index as isize),
            (*coef)
                .iMCU_row_num
                .wrapping_mul((*compptr).v_samp_factor as u32),
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            0,
        );
        ci += 1
    }

    for yoffset in (*coef).MCU_vert_offset..(*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).mcu_ctr;

        while MCU_col_num < (*cinfo).MCUs_per_row {
            /* Construct list of pointers to DCT blocks belonging to this MCU */
            blkn = 0; /* index of current DCT block within MCU */
            ci = 0;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col = MCU_col_num.wrapping_mul((*compptr).MCU_width as u32);
                blockcnt = if MCU_col_num < last_MCU_col {
                    (*compptr).MCU_width
                } else {
                    (*compptr).last_col_width
                };

                for yindex in 0..(*compptr).MCU_height {
                    if (*coef).iMCU_row_num < last_iMCU_row
                        || yindex + yoffset < (*compptr).last_row_height
                    {
                        /* Fill in pointers to real blocks in this row */
                        buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                            .offset(start_col as isize);
                        xindex = 0;
                        while xindex < blockcnt {
                            let fresh0 = buffer_ptr;
                            buffer_ptr = buffer_ptr.offset(1);
                            let fresh1 = blkn;
                            blkn = blkn + 1;
                            MCU_buffer[fresh1 as usize] = fresh0;
                            xindex += 1
                        }
                    } else {
                        /* At bottom of image, need a whole row of dummy blocks */
                        xindex = 0
                    }

                    while xindex < (*compptr).MCU_width {
                        MCU_buffer[blkn as usize] = (*coef).dummy_buffer[blkn as usize];
                        (*MCU_buffer[blkn as usize].offset(0))[0] =
                            (*MCU_buffer[(blkn - 1) as usize].offset(0))[0];
                        blkn += 1;
                        xindex += 1
                    }
                }
                ci += 1
            }
            /* Try to write the MCU. */
            if Some(
                (*(*cinfo).entropy)
                    .encode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, MCU_buffer.as_mut_ptr())
                == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).mcu_ctr = MCU_col_num;
                return 0i32;
            }
            MCU_col_num = MCU_col_num.wrapping_add(1)
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).mcu_ctr = 0;
    }
    /* Completed the iMCU row, advance counters for next one */
    (*coef).iMCU_row_num = (*coef).iMCU_row_num.wrapping_add(1);
    start_iMCU_row(cinfo);
    return 1;
}
/*
 * Initialize coefficient buffer controller.
 *
 * Each passed coefficient array must be the right size for that
 * coefficient: width_in_blocks wide and height_in_blocks high,
 * with unitheight at least v_samp_factor.
 */

unsafe extern "C" fn transencode_coef_controller(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
) {
    let mut coef: my_coef_ptr = 0 as *mut my_coef_controller;
    let mut buffer: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut i: i32 = 0;
    coef = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        ::std::mem::size_of::<my_coef_controller>(),
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut crate::jpegint_h::jpeg_c_coef_controller;
    (*coef).pub_0.start_pass = Some(
        start_pass_coef
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jpegint_h::J_BUF_MODE,
            ) -> (),
    );
    (*coef).pub_0.compress_data = Some(
        compress_output
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jpeglib_h::JSAMPIMAGE,
            ) -> crate::jmorecfg_h::boolean,
    );
    /* Save pointer to virtual arrays */
    (*coef).whole_image = coef_arrays;
    /* Allocate and pre-zero space for dummy DCT blocks. */
    buffer = Some(
        (*(*cinfo).mem)
            .alloc_large
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1,
        (10usize).wrapping_mul(::std::mem::size_of::<crate::jpeglib_h::JBLOCK>()),
    ) as crate::jpeglib_h::JBLOCKROW;
    crate::src::jpeg_8c::jutils::jzero_far(
        buffer as *mut libc::c_void,
        (10usize).wrapping_mul(::std::mem::size_of::<crate::jpeglib_h::JBLOCK>()),
    );
    i = 0;
    while i < 10 {
        (*coef).dummy_buffer[i as usize] = buffer.offset(i as isize);
        i += 1
    }
}
