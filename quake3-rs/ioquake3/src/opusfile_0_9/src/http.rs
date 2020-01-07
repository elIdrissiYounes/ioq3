use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::ptrdiff_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
use crate::src::opus_1_2_1::src::opus_multistream_decoder::OpusMSDecoder;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::internal_h::op_sample;
pub use crate::internal_h::OggOpusFile;
pub use crate::internal_h::OggOpusLink;
pub use crate::src::opusfile_0_9::src::internal::op_strncasecmp;
pub use crate::src::opusfile_0_9::src::opusfile::op_close_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_decode_cb_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_open_callbacks;
pub use crate::src::opusfile_0_9::src::opusfile::op_read_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_seek_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_tell_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_test_callbacks;
pub use crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks;
pub use crate::src::opusfile_0_9::src::opusfile::OpusHead;
pub use crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
pub use crate::src::opusfile_0_9::src::opusfile::OpusTags;
pub use crate::src::opusfile_0_9::src::stream::op_fopen;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memcpy;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
use crate::stdlib::strspn;

unsafe extern "C" fn op_string_range_dup(mut _start: *const i8, mut _end: *const i8) -> *mut i8 {
    let mut len: crate::stddef_h::size_t = 0;
    let mut ret: *mut i8 = 0 as *mut i8;
    len = _end.wrapping_offset_from(_start) as crate::stddef_h::size_t;
    /*This is to help avoid overflow elsewhere, later.*/
    if (len >= 2147483647i32 as usize) as i32 as isize != 0 {
        return 0 as *mut i8;
    }
    ret =
        crate::stdlib::malloc((::std::mem::size_of::<i8>()).wrapping_mul(len.wrapping_add(1usize)))
            as *mut i8;
    if !ret.is_null() as i32 as isize != 0 {
        ret = crate::stdlib::memcpy(
            ret as *mut libc::c_void,
            _start as *const libc::c_void,
            (::std::mem::size_of::<i8>()).wrapping_mul(len),
        ) as *mut i8;
        *ret.offset(len as isize) = '\u{0}' as i8
    }
    return ret;
}

unsafe extern "C" fn op_string_dup(mut _s: *const i8) -> *mut i8 {
    return op_string_range_dup(_s, _s.offset(crate::stdlib::strlen(_s) as isize));
}

unsafe extern "C" fn op_string_tolower(mut _s: *mut i8) -> *mut i8 {
    let mut i: i32 = 0;
    i = 0;
    while *_s.offset(i as isize) as i32 != '\u{0}' as i32 {
        let mut c: i32 = 0;
        c = *_s.offset(i as isize) as i32;
        if c >= 'A' as i32 && c <= 'Z' as i32 {
            c += 'a' as i32 - 'A' as i32
        }
        *_s.offset(i as isize) = c as i8;
        i += 1
    }
    return _s;
}
/*URI character classes (from RFC 3986).*/
/*Not a character class, but the characters allowed in <scheme>.*/
/*Not a character class, but the characters allowed in <pct-encoded>.*/
/*Not a character class or production rule, but for convenience.*/
/*Not a character class, but the characters allowed in <userinfo> and
<IP-literal>.*/
/*Not a character class, but the characters allowed in <segment-nz-nc>.*/
/*Not a character clsss, but the characters allowed in <path>.*/
/*Not a character class, but the characters allowed in <query> / <fragment>.*/
/*Check the <% HEXDIG HEXDIG> escapes of a URL for validity.
Return: 0 if valid, or a negative value on failure.*/

unsafe extern "C" fn op_validate_url_escapes(mut _s: *const i8) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while *_s.offset(i as isize) != 0 {
        if *_s.offset(i as isize) as i32 == '%' as i32 {
            if (*(*crate::stdlib::__ctype_b_loc())
                .offset(*_s.offset((i + 1) as isize) as i32 as isize) as i32
                & crate::stdlib::_ISxdigit as u16 as i32
                == 0) as i32 as isize
                != 0
                || (*(*crate::stdlib::__ctype_b_loc())
                    .offset(*_s.offset((i + 2) as isize) as i32 as isize)
                    as i32
                    & crate::stdlib::_ISxdigit as u16 as i32
                    == 0) as i32 as isize
                    != 0
                || (*_s.offset((i + 1) as isize) as i32 == '0' as i32
                    && *_s.offset((i + 2) as isize) as i32 == '0' as i32) as i32
                    as isize
                    != 0
            {
                return -(1i32);
            }
            i += 2
        }
        i += 1
    }
    return 0;
}
/*Convert a hex digit to its actual value.
_c: The hex digit to convert.
    Presumed to be valid ('0'...'9', 'A'...'F', or 'a'...'f').
Return: The value of the digit, in the range [0,15].*/

unsafe extern "C" fn op_hex_value(mut _c: i32) -> i32 {
    return if _c >= 'a' as i32 {
        (_c - 'a' as i32) + 10
    } else if _c >= 'A' as i32 {
        (_c - 'A' as i32) + 10
    } else {
        (_c) - '0' as i32
    };
}
/*Unescape all the <% HEXDIG HEXDIG> sequences in a string in-place.
This does no validity checking.*/

unsafe extern "C" fn op_unescape_url_component(mut _s: *mut i8) -> *mut i8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    j = 0;
    i = j;
    while *_s.offset(i as isize) != 0 {
        if *_s.offset(i as isize) as i32 == '%' as i32 {
            *_s.offset(i as isize) = (op_hex_value(*_s.offset((i + 1) as isize) as i32) << 4
                | op_hex_value(*_s.offset((i + 2) as isize) as i32))
                as i8;
            i += 2
        }
        i += 1;
        j += 1
    }
    return _s;
}
/*Parse a file: URL.
This code is not meant to be fast: strspn() with large sets is likely to be
 slow, but it is very convenient.
It is meant to be RFC 1738-compliant (as updated by RFC 3986).*/

unsafe extern "C" fn op_parse_file_url(mut _src: *const i8) -> *const i8 {
    let mut scheme_end: *const i8 = 0 as *const i8;
    let mut path: *const i8 = 0 as *const i8;
    let mut path_end: *const i8 = 0 as *const i8;
    scheme_end = _src.offset(crate::stdlib::strspn(
        _src,
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-.\x00" as *const u8
            as *const i8,
    ) as isize);
    if (*scheme_end as i32 != ':' as i32) as i32 as isize != 0
        || scheme_end.wrapping_offset_from(_src) != 4
        || crate::src::opusfile_0_9::src::internal::op_strncasecmp(
            _src,
            b"file\x00" as *const u8 as *const i8,
            4,
        ) != 0
    {
        /*Unsupported protocol.*/
        return 0 as *const i8;
    }
    /*Make sure all escape sequences are valid to simplify unescaping later.*/
    if (op_validate_url_escapes(scheme_end.offset(1)) < 0) as i32 as isize != 0 {
        return 0 as *const i8;
    }
    if *scheme_end.offset(1) as i32 == '/' as i32 && *scheme_end.offset(2) as i32 == '/' as i32 {
        let mut host: *const i8 = 0 as *const i8;
        /*file: URLs can have a host!
        Yeah, I was surprised, too, but that's what RFC 1738 says.
        It also says, "The file URL scheme is unusual in that it does not specify
         an Internet protocol or access method for such files; as such, its
         utility in network protocols between hosts is limited," which is a mild
         understatement.*/
        host = scheme_end.offset(3);
        /*The empty host is what we expect.*/
        if (*host as i32 == '/' as i32) as i32 as isize != 0 {
            path = host
        } else {
            let mut host_end: *const i8 = 0 as *const i8;
            let mut host_buf: [i8; 28] = [0; 28];
            /*RFC 1738 says localhost "is interpreted as `the machine from which the
            URL is being interpreted,'" so let's check for it.*/
            host_end =
                host.offset(crate::stdlib::strspn(host,
                                   b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~%!$&\'()*+,;=\x00"
                                       as *const u8 as *const i8) as
                                isize);
            /*No <port> allowed.
            This also rejects IP-Literals.*/
            if *host_end as i32 != '/' as i32 {
                return 0 as *const i8;
            }
            /*An escaped "localhost" can take at most 27 characters.*/
            if (host_end.wrapping_offset_from(host) > 27) as i32 as isize != 0 {
                return 0 as *const i8;
            }
            crate::stdlib::memcpy(
                host_buf.as_mut_ptr() as *mut libc::c_void,
                host as *const libc::c_void,
                (::std::mem::size_of::<i8>())
                    .wrapping_mul(host_end.wrapping_offset_from(host) as usize),
            );
            host_buf[host_end.wrapping_offset_from(host) as usize] = '\u{0}' as i8;
            op_unescape_url_component(host_buf.as_mut_ptr());
            op_string_tolower(host_buf.as_mut_ptr());
            /*Some other host: give up.*/
            if (crate::stdlib::strcmp(
                host_buf.as_mut_ptr(),
                b"localhost\x00" as *const u8 as *const i8,
            ) != 0) as i32 as isize
                != 0
            {
                return 0 as *const i8;
            }
            path = host_end
        }
    } else {
        path = scheme_end.offset(1)
    }
    path_end = path.offset(crate::stdlib::strspn(
        path,
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~%!$&\'()*+,;=:@/\x00"
            as *const u8 as *const i8,
    ) as isize);
    /*This will reject a <query> or <fragment> component, too.
    I don't know what to do with queries, but a temporal fragment would at
     least make sense.
    RFC 1738 pretty clearly defines a <searchpart> that's equivalent to the
     RFC 3986 <query> component for other schemes, but not the file: scheme,
     so I'm going to just reject it.*/
    if *path_end as i32 != '\u{0}' as i32 {
        return 0 as *const i8;
    }
    return path;
}
#[no_mangle]

pub unsafe extern "C" fn opus_server_info_init(
    mut _info: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo,
) {
    (*_info).name = 0 as *mut i8;
    (*_info).description = 0 as *mut i8;
    (*_info).genre = 0 as *mut i8;
    (*_info).url = 0 as *mut i8;
    (*_info).server = 0 as *mut i8;
    (*_info).content_type = 0 as *mut i8;
    (*_info).bitrate_kbps = -(1);
    (*_info).is_public = -(1);
    (*_info).is_ssl = 0;
}
#[no_mangle]

pub unsafe extern "C" fn opus_server_info_clear(
    mut _info: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo,
) {
    crate::stdlib::free((*_info).content_type as *mut libc::c_void);
    crate::stdlib::free((*_info).server as *mut libc::c_void);
    crate::stdlib::free((*_info).url as *mut libc::c_void);
    crate::stdlib::free((*_info).genre as *mut libc::c_void);
    crate::stdlib::free((*_info).description as *mut libc::c_void);
    crate::stdlib::free((*_info).name as *mut libc::c_void);
}
/*The actual URL stream creation function.
This one isn't extensible like the application-level interface, but because
 it isn't public, we're free to change it in the future.*/

unsafe extern "C" fn op_url_stream_create_impl(
    mut _cb: *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _url: *const i8,
    mut _skip_certificate_check: i32,
    mut _proxy_host: *const i8,
    mut _proxy_port: u32,
    mut _proxy_user: *const i8,
    mut _proxy_pass: *const i8,
    mut _info: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo,
) -> *mut libc::c_void {
    let mut path: *const i8 = 0 as *const i8;
    /*Check to see if this is a valid file: URL.*/
    path = op_parse_file_url(_url);
    if !path.is_null() {
        let mut unescaped_path: *mut i8 = 0 as *mut i8;
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        unescaped_path = op_string_dup(path);
        if unescaped_path.is_null() as i32 as isize != 0 {
            return 0 as *mut libc::c_void;
        }
        ret = crate::src::opusfile_0_9::src::stream::op_fopen(
            _cb,
            op_unescape_url_component(unescaped_path),
            b"rb\x00" as *const u8 as *const i8,
        );
        crate::stdlib::free(unescaped_path as *mut libc::c_void);
        return ret;
    }
    return 0 as *mut libc::c_void;
}
/*The actual implementation of op_url_stream_vcreate().
We have to do a careful dance here to avoid potential memory leaks if
 OpusServerInfo is requested, since this function is also used by
 op_vopen_url() and op_vtest_url().
Even if this function succeeds, those functions might ultimately fail.
If they do, they should return without having touched the OpusServerInfo
 passed by the application.
Therefore, if this function succeeds and OpusServerInfo is requested, the
 actual info will be stored in *_info and a pointer to the application's
 storage will be placed in *_pinfo.
If this function fails or if the application did not request OpusServerInfo,
 *_pinfo will be NULL.
Our caller is responsible for copying *_info to **_pinfo if it ultimately
 succeeds, or for clearing *_info if it ultimately fails.*/

unsafe extern "C" fn op_url_stream_vcreate_impl(
    mut _cb: *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _url: *const i8,
    mut _info: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo,
    mut _pinfo: *mut *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo,
    mut _ap: ::std::ffi::VaList,
) -> *mut libc::c_void {
    let mut skip_certificate_check: i32 = 0;
    let mut proxy_host: *const i8 = 0 as *const i8;
    let mut proxy_port: crate::opus_types_h::opus_int32 = 0;
    let mut proxy_user: *const i8 = 0 as *const i8;
    let mut proxy_pass: *const i8 = 0 as *const i8;
    let mut pinfo: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
    skip_certificate_check = 0;
    proxy_host = 0 as *const i8;
    proxy_port = 8080;
    proxy_user = 0 as *const i8;
    proxy_pass = 0 as *const i8;
    pinfo = 0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
    *_pinfo = 0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
    loop {
        let mut request: crate::stddef_h::ptrdiff_t = 0;
        request = _ap
            .as_va_list()
            .arg::<*mut i8>()
            .wrapping_offset_from(0 as *mut i8);
        /*If we hit NULL, we're done processing options.*/
        if request == 0 {
            break;
        }
        match request {
            6464 => {
                skip_certificate_check =
                    (_ap.as_va_list().arg::<crate::opus_types_h::opus_int32>() != 0) as i32
            }
            6528 => proxy_host = _ap.as_va_list().arg::<*const i8>(),
            6592 => {
                proxy_port = _ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
                if proxy_port < 0 || proxy_port > 65535 {
                    return 0 as *mut libc::c_void;
                }
            }
            6656 => proxy_user = _ap.as_va_list().arg::<*const i8>(),
            6720 => proxy_pass = _ap.as_va_list().arg::<*const i8>(),
            6784 => {
                pinfo = _ap
                    .as_va_list()
                    .arg::<*mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo>()
            }
            _ => {
                /*Some unknown option.*/
                return 0 as *mut libc::c_void;
            }
        }
    }
    /*If the caller has requested server information, proxy it to a local copy to
    simplify error handling.*/
    if !pinfo.is_null() {
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        opus_server_info_init(_info);
        ret = op_url_stream_create_impl(
            _cb,
            _url,
            skip_certificate_check,
            proxy_host,
            proxy_port as u32,
            proxy_user,
            proxy_pass,
            _info,
        );
        if !ret.is_null() {
            *_pinfo = pinfo
        } else {
            opus_server_info_clear(_info);
        }
        return ret;
    }
    return op_url_stream_create_impl(
        _cb,
        _url,
        skip_certificate_check,
        proxy_host,
        proxy_port as u32,
        proxy_user,
        proxy_pass,
        0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo,
    );
}
#[no_mangle]

pub unsafe extern "C" fn op_url_stream_vcreate(
    mut _cb: *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _url: *const i8,
    mut _ap: ::std::ffi::VaList,
) -> *mut libc::c_void {
    let mut info: crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        crate::src::opusfile_0_9::src::opusfile::OpusServerInfo {
            name: 0 as *mut i8,
            description: 0 as *mut i8,
            genre: 0 as *mut i8,
            url: 0 as *mut i8,
            server: 0 as *mut i8,
            content_type: 0 as *mut i8,
            bitrate_kbps: 0,
            is_public: 0,
            is_ssl: 0,
        };
    let mut pinfo: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = op_url_stream_vcreate_impl(_cb, _url, &mut info, &mut pinfo, _ap.as_va_list());
    if !pinfo.is_null() {
        *pinfo = info
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn op_url_stream_create(
    mut _cb: *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _url: *const i8,
    mut args: ...
) -> *mut libc::c_void {
    let mut ap: ::std::ffi::VaListImpl;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ap = args.clone();
    ret = op_url_stream_vcreate(_cb, _url, ap.as_va_list());
    return ret;
}
/*Convenience routines to open/test URLs in a single step.*/
#[no_mangle]

pub unsafe extern "C" fn op_vopen_url(
    mut _url: *const i8,
    mut _error: *mut i32,
    mut _ap: ::std::ffi::VaList,
) -> *mut crate::internal_h::OggOpusFile {
    let mut cb: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks =
        crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
            read: None,
            seek: None,
            tell: None,
            close: None,
        };
    let mut of: *mut crate::internal_h::OggOpusFile = 0 as *mut crate::internal_h::OggOpusFile;
    let mut info: crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        crate::src::opusfile_0_9::src::opusfile::OpusServerInfo {
            name: 0 as *mut i8,
            description: 0 as *mut i8,
            genre: 0 as *mut i8,
            url: 0 as *mut i8,
            server: 0 as *mut i8,
            content_type: 0 as *mut i8,
            bitrate_kbps: 0,
            is_public: 0,
            is_ssl: 0,
        };
    let mut pinfo: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
    let mut source: *mut libc::c_void = 0 as *mut libc::c_void;
    source = op_url_stream_vcreate_impl(&mut cb, _url, &mut info, &mut pinfo, _ap.as_va_list());
    if source.is_null() as i32 as isize != 0 {
        if !_error.is_null() {
            *_error = -(129)
        }
        return 0 as *mut crate::internal_h::OggOpusFile;
    }
    of = crate::src::opusfile_0_9::src::opusfile::op_open_callbacks(
        source,
        &mut cb,
        0 as *const u8,
        0,
        _error,
    );
    if of.is_null() as i32 as isize != 0 {
        if !pinfo.is_null() {
            opus_server_info_clear(&mut info);
        }
        Some(cb.close.expect("non-null function pointer")).expect("non-null function pointer")(
            source,
        );
    } else if !pinfo.is_null() {
        *pinfo = info
    }
    return of;
}
#[no_mangle]

pub unsafe extern "C" fn op_open_url(
    mut _url: *const i8,
    mut _error: *mut i32,
    mut args: ...
) -> *mut crate::internal_h::OggOpusFile {
    let mut ret: *mut crate::internal_h::OggOpusFile = 0 as *mut crate::internal_h::OggOpusFile;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    ret = op_vopen_url(_url, _error, ap.as_va_list());
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn op_vtest_url(
    mut _url: *const i8,
    mut _error: *mut i32,
    mut _ap: ::std::ffi::VaList,
) -> *mut crate::internal_h::OggOpusFile {
    let mut cb: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks =
        crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
            read: None,
            seek: None,
            tell: None,
            close: None,
        };
    let mut of: *mut crate::internal_h::OggOpusFile = 0 as *mut crate::internal_h::OggOpusFile;
    let mut info: crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        crate::src::opusfile_0_9::src::opusfile::OpusServerInfo {
            name: 0 as *mut i8,
            description: 0 as *mut i8,
            genre: 0 as *mut i8,
            url: 0 as *mut i8,
            server: 0 as *mut i8,
            content_type: 0 as *mut i8,
            bitrate_kbps: 0,
            is_public: 0,
            is_ssl: 0,
        };
    let mut pinfo: *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo =
        0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusServerInfo;
    let mut source: *mut libc::c_void = 0 as *mut libc::c_void;
    source = op_url_stream_vcreate_impl(&mut cb, _url, &mut info, &mut pinfo, _ap.as_va_list());
    if source.is_null() as i32 as isize != 0 {
        if !_error.is_null() {
            *_error = -(129)
        }
        return 0 as *mut crate::internal_h::OggOpusFile;
    }
    of = crate::src::opusfile_0_9::src::opusfile::op_test_callbacks(
        source,
        &mut cb,
        0 as *const u8,
        0,
        _error,
    );
    if of.is_null() as i32 as isize != 0 {
        if !pinfo.is_null() {
            opus_server_info_clear(&mut info);
        }
        Some(cb.close.expect("non-null function pointer")).expect("non-null function pointer")(
            source,
        );
    } else if !pinfo.is_null() {
        *pinfo = info
    }
    return of;
}
#[no_mangle]

pub unsafe extern "C" fn op_test_url(
    mut _url: *const i8,
    mut _error: *mut i32,
    mut args: ...
) -> *mut crate::internal_h::OggOpusFile {
    let mut ret: *mut crate::internal_h::OggOpusFile = 0 as *mut crate::internal_h::OggOpusFile;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    ret = op_vtest_url(_url, _error, ap.as_va_list());
    return ret;
}
