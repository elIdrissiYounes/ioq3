pub type backing_store_info = crate::jmemsys_h::backing_store_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_store_struct {
    pub read_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jmemsys_h::backing_store_ptr,
            _: *mut libc::c_void,
            _: isize,
            _: isize,
        ) -> (),
    >,
    pub write_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jmemsys_h::backing_store_ptr,
            _: *mut libc::c_void,
            _: isize,
            _: isize,
        ) -> (),
    >,
    pub close_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: crate::jmemsys_h::backing_store_ptr,
        ) -> (),
    >,
    pub temp_file: *mut crate::stdlib::FILE,
    pub temp_name: [i8; 64],
}
pub type backing_store_ptr = *mut crate::jmemsys_h::backing_store_struct;
