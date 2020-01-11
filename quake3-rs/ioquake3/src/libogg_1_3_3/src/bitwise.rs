use ::libc;

pub use crate::ogg_h::oggpack_buffer;

use crate::stdlib::malloc;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::realloc;
use ::libc::free;

static mut mask: [libc::c_ulong; 33] = [
    0 as libc::c_int as libc::c_ulong,
    0x1 as libc::c_int as libc::c_ulong,
    0x3 as libc::c_int as libc::c_ulong,
    0x7 as libc::c_int as libc::c_ulong,
    0xf as libc::c_int as libc::c_ulong,
    0x1f as libc::c_int as libc::c_ulong,
    0x3f as libc::c_int as libc::c_ulong,
    0x7f as libc::c_int as libc::c_ulong,
    0xff as libc::c_int as libc::c_ulong,
    0x1ff as libc::c_int as libc::c_ulong,
    0x3ff as libc::c_int as libc::c_ulong,
    0x7ff as libc::c_int as libc::c_ulong,
    0xfff as libc::c_int as libc::c_ulong,
    0x1fff as libc::c_int as libc::c_ulong,
    0x3fff as libc::c_int as libc::c_ulong,
    0x7fff as libc::c_int as libc::c_ulong,
    0xffff as libc::c_int as libc::c_ulong,
    0x1ffff as libc::c_int as libc::c_ulong,
    0x3ffff as libc::c_int as libc::c_ulong,
    0x7ffff as libc::c_int as libc::c_ulong,
    0xfffff as libc::c_int as libc::c_ulong,
    0x1fffff as libc::c_int as libc::c_ulong,
    0x3fffff as libc::c_int as libc::c_ulong,
    0x7fffff as libc::c_int as libc::c_ulong,
    0xffffff as libc::c_int as libc::c_ulong,
    0x1ffffff as libc::c_int as libc::c_ulong,
    0x3ffffff as libc::c_int as libc::c_ulong,
    0x7ffffff as libc::c_int as libc::c_ulong,
    0xfffffff as libc::c_int as libc::c_ulong,
    0x1fffffff as libc::c_int as libc::c_ulong,
    0x3fffffff as libc::c_int as libc::c_ulong,
    0x7fffffff as libc::c_int as libc::c_ulong,
    0xffffffff as libc::c_uint as libc::c_ulong,
];

static mut mask8B: [libc::c_uint; 9] = [
    0 as libc::c_int as libc::c_uint,
    0x80 as libc::c_int as libc::c_uint,
    0xc0 as libc::c_int as libc::c_uint,
    0xe0 as libc::c_int as libc::c_uint,
    0xf0 as libc::c_int as libc::c_uint,
    0xf8 as libc::c_int as libc::c_uint,
    0xfc as libc::c_int as libc::c_uint,
    0xfe as libc::c_int as libc::c_uint,
    0xff as libc::c_int as libc::c_uint,
];
/* Ogg BITSTREAM PRIMITIVES: bitstream ************************/
#[no_mangle]

pub unsafe extern "C" fn oggpack_writeinit(mut b: *mut crate::ogg_h::oggpack_buffer) {
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>() as libc::c_ulong,
    );
    (*b).buffer = crate::stdlib::malloc(256 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    (*b).ptr = (*b).buffer;
    *(*b).buffer.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_uchar;
    (*b).storage = 256 as libc::c_int as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writeinit(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_writeinit(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writecheck(
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    if (*b).ptr.is_null() || (*b).storage == 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writecheck(
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    return oggpack_writecheck(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writetrunc(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_long,
) {
    let mut bytes: libc::c_long = bits >> 3 as libc::c_int;
    if !(*b).ptr.is_null() {
        bits -= bytes * 8 as libc::c_int as libc::c_long;
        (*b).ptr = (*b).buffer.offset(bytes as isize);
        (*b).endbit = bits as libc::c_int;
        (*b).endbyte = bytes;
        *(*b).ptr = (*(*b).ptr as libc::c_ulong & mask[bits as usize]) as libc::c_uchar
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writetrunc(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_long,
) {
    let mut bytes: libc::c_long = bits >> 3 as libc::c_int;
    if !(*b).ptr.is_null() {
        bits -= bytes * 8 as libc::c_int as libc::c_long;
        (*b).ptr = (*b).buffer.offset(bytes as isize);
        (*b).endbit = bits as libc::c_int;
        (*b).endbyte = bytes;
        *(*b).ptr = (*(*b).ptr as libc::c_uint & mask8B[bits as usize]) as libc::c_uchar
    };
}
/* Takes only up to 32 bits. */
#[no_mangle]

pub unsafe extern "C" fn oggpack_write(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut value: libc::c_ulong,
    mut bits: libc::c_int,
) {
    let mut current_block: u64;
    if !(bits < 0 as libc::c_int || bits > 32 as libc::c_int) {
        if (*b).endbyte >= (*b).storage - 4 as libc::c_int as libc::c_long {
            let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
            if (*b).ptr.is_null() {
                return;
            }
            if (*b).storage
                > 9223372036854775807 as libc::c_long - 256 as libc::c_int as libc::c_long
            {
                current_block = 11736846078876452800;
            } else {
                ret = crate::stdlib::realloc(
                    (*b).buffer as *mut libc::c_void,
                    ((*b).storage + 256 as libc::c_int as libc::c_long) as libc::c_ulong,
                );
                if ret.is_null() {
                    current_block = 11736846078876452800;
                } else {
                    (*b).buffer = ret as *mut libc::c_uchar;
                    (*b).storage += 256 as libc::c_int as libc::c_long;
                    (*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
                    current_block = 13109137661213826276;
                }
            }
        } else {
            current_block = 13109137661213826276;
        }
        match current_block {
            11736846078876452800 => {}
            _ => {
                value &= mask[bits as usize];
                bits += (*b).endbit;
                let ref mut fresh0 = *(*b).ptr.offset(0 as libc::c_int as isize);
                *fresh0 = (*fresh0 as libc::c_ulong | value << (*b).endbit) as libc::c_uchar;
                if bits >= 8 as libc::c_int {
                    *(*b).ptr.offset(1 as libc::c_int as isize) =
                        (value >> 8 as libc::c_int - (*b).endbit) as libc::c_uchar;
                    if bits >= 16 as libc::c_int {
                        *(*b).ptr.offset(2 as libc::c_int as isize) =
                            (value >> 16 as libc::c_int - (*b).endbit) as libc::c_uchar;
                        if bits >= 24 as libc::c_int {
                            *(*b).ptr.offset(3 as libc::c_int as isize) =
                                (value >> 24 as libc::c_int - (*b).endbit) as libc::c_uchar;
                            if bits >= 32 as libc::c_int {
                                if (*b).endbit != 0 {
                                    *(*b).ptr.offset(4 as libc::c_int as isize) =
                                        (value >> 32 as libc::c_int - (*b).endbit) as libc::c_uchar
                                } else {
                                    *(*b).ptr.offset(4 as libc::c_int as isize) =
                                        0 as libc::c_int as libc::c_uchar
                                }
                            }
                        }
                    }
                }
                (*b).endbyte += (bits / 8 as libc::c_int) as libc::c_long;
                (*b).ptr = (*b).ptr.offset((bits / 8 as libc::c_int) as isize);
                (*b).endbit = bits & 7 as libc::c_int;
                return;
            }
        }
    }
    oggpack_writeclear(b);
}
/* Takes only up to 32 bits. */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_write(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut value: libc::c_ulong,
    mut bits: libc::c_int,
) {
    let mut current_block: u64;
    if !(bits < 0 as libc::c_int || bits > 32 as libc::c_int) {
        if (*b).endbyte >= (*b).storage - 4 as libc::c_int as libc::c_long {
            let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
            if (*b).ptr.is_null() {
                return;
            }
            if (*b).storage
                > 9223372036854775807 as libc::c_long - 256 as libc::c_int as libc::c_long
            {
                current_block = 2612814309992382393;
            } else {
                ret = crate::stdlib::realloc(
                    (*b).buffer as *mut libc::c_void,
                    ((*b).storage + 256 as libc::c_int as libc::c_long) as libc::c_ulong,
                );
                if ret.is_null() {
                    current_block = 2612814309992382393;
                } else {
                    (*b).buffer = ret as *mut libc::c_uchar;
                    (*b).storage += 256 as libc::c_int as libc::c_long;
                    (*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
                    current_block = 13109137661213826276;
                }
            }
        } else {
            current_block = 13109137661213826276;
        }
        match current_block {
            2612814309992382393 => {}
            _ => {
                value = (value & mask[bits as usize]) << 32 as libc::c_int - bits;
                bits += (*b).endbit;
                let ref mut fresh1 = *(*b).ptr.offset(0 as libc::c_int as isize);
                *fresh1 = (*fresh1 as libc::c_ulong | value >> 24 as libc::c_int + (*b).endbit)
                    as libc::c_uchar;
                if bits >= 8 as libc::c_int {
                    *(*b).ptr.offset(1 as libc::c_int as isize) =
                        (value >> 16 as libc::c_int + (*b).endbit) as libc::c_uchar;
                    if bits >= 16 as libc::c_int {
                        *(*b).ptr.offset(2 as libc::c_int as isize) =
                            (value >> 8 as libc::c_int + (*b).endbit) as libc::c_uchar;
                        if bits >= 24 as libc::c_int {
                            *(*b).ptr.offset(3 as libc::c_int as isize) =
                                (value >> (*b).endbit) as libc::c_uchar;
                            if bits >= 32 as libc::c_int {
                                if (*b).endbit != 0 {
                                    *(*b).ptr.offset(4 as libc::c_int as isize) =
                                        (value << 8 as libc::c_int - (*b).endbit) as libc::c_uchar
                                } else {
                                    *(*b).ptr.offset(4 as libc::c_int as isize) =
                                        0 as libc::c_int as libc::c_uchar
                                }
                            }
                        }
                    }
                }
                (*b).endbyte += (bits / 8 as libc::c_int) as libc::c_long;
                (*b).ptr = (*b).ptr.offset((bits / 8 as libc::c_int) as isize);
                (*b).endbit = bits & 7 as libc::c_int;
                return;
            }
        }
    }
    oggpack_writeclear(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writealign(mut b: *mut crate::ogg_h::oggpack_buffer) {
    let mut bits: libc::c_int = 8 as libc::c_int - (*b).endbit;
    if bits < 8 as libc::c_int {
        oggpack_write(b, 0 as libc::c_int as libc::c_ulong, bits);
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writealign(mut b: *mut crate::ogg_h::oggpack_buffer) {
    let mut bits: libc::c_int = 8 as libc::c_int - (*b).endbit;
    if bits < 8 as libc::c_int {
        oggpackB_write(b, 0 as libc::c_int as libc::c_ulong, bits);
    };
}

unsafe extern "C" fn oggpack_writecopy_helper(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: libc::c_long,
    mut w: Option<
        unsafe extern "C" fn(
            _: *mut crate::ogg_h::oggpack_buffer,
            _: libc::c_ulong,
            _: libc::c_int,
        ) -> (),
    >,
    mut msb: libc::c_int,
) {
    let mut current_block: u64;
    let mut ptr: *mut libc::c_uchar = source as *mut libc::c_uchar;
    let mut bytes: libc::c_long = bits / 8 as libc::c_int as libc::c_long;
    let mut pbytes: libc::c_long =
        ((*b).endbit as libc::c_long + bits) / 8 as libc::c_int as libc::c_long;
    bits -= bytes * 8 as libc::c_int as libc::c_long;
    /* expand storage up-front */
    if (*b).endbyte + pbytes >= (*b).storage {
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*b).ptr.is_null() {
            current_block = 1692384543052803397;
        } else if (*b).storage > (*b).endbyte + pbytes + 256 as libc::c_int as libc::c_long {
            current_block = 1692384543052803397;
        } else {
            (*b).storage = (*b).endbyte + pbytes + 256 as libc::c_int as libc::c_long;
            ret = crate::stdlib::realloc(
                (*b).buffer as *mut libc::c_void,
                (*b).storage as libc::c_ulong,
            );
            if ret.is_null() {
                current_block = 1692384543052803397;
            } else {
                (*b).buffer = ret as *mut libc::c_uchar;
                (*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
                current_block = 7746791466490516765;
            }
        }
        match current_block {
            7746791466490516765 => {}
            _ => {
                oggpack_writeclear(b);
                return;
            }
        }
    }
    /* copy whole octets */
    if (*b).endbit != 0 {
        let mut i: libc::c_int = 0;
        /* unaligned copy.  Do it the hard way. */
        i = 0 as libc::c_int;
        while (i as libc::c_long) < bytes {
            w.expect("non-null function pointer")(
                b,
                *ptr.offset(i as isize) as libc::c_ulong,
                8 as libc::c_int,
            );
            i += 1
        }
    } else {
        /* aligned block copy */
        crate::stdlib::memmove(
            (*b).ptr as *mut libc::c_void,
            source,
            bytes as libc::c_ulong,
        );
        (*b).ptr = (*b).ptr.offset(bytes as isize);
        (*b).endbyte += bytes;
        *(*b).ptr = 0 as libc::c_int as libc::c_uchar
    }
    /* copy trailing bits */
    if bits != 0 {
        if msb != 0 {
            w.expect("non-null function pointer")(
                b,
                (*ptr.offset(bytes as isize) as libc::c_int
                    >> 8 as libc::c_int as libc::c_long - bits) as libc::c_ulong,
                bits as libc::c_int,
            );
        } else {
            w.expect("non-null function pointer")(
                b,
                *ptr.offset(bytes as isize) as libc::c_ulong,
                bits as libc::c_int,
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writecopy(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: libc::c_long,
) {
    oggpack_writecopy_helper(
        b,
        source,
        bits,
        Some(
            oggpack_write
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: libc::c_ulong,
                    _: libc::c_int,
                ) -> (),
        ),
        0 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writecopy(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: libc::c_long,
) {
    oggpack_writecopy_helper(
        b,
        source,
        bits,
        Some(
            oggpackB_write
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: libc::c_ulong,
                    _: libc::c_int,
                ) -> (),
        ),
        1 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_reset(mut b: *mut crate::ogg_h::oggpack_buffer) {
    if (*b).ptr.is_null() {
        return;
    }
    (*b).ptr = (*b).buffer;
    *(*b).buffer.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    (*b).endbyte = 0 as libc::c_int as libc::c_long;
    (*b).endbit = (*b).endbyte as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_reset(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_reset(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writeclear(mut b: *mut crate::ogg_h::oggpack_buffer) {
    if !(*b).buffer.is_null() {
        ::libc::free((*b).buffer as *mut libc::c_void);
    }
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>() as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writeclear(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_writeclear(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_readinit(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut buf: *mut libc::c_uchar,
    mut bytes: libc::c_int,
) {
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>() as libc::c_ulong,
    );
    (*b).ptr = buf;
    (*b).buffer = (*b).ptr;
    (*b).storage = bytes as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_readinit(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut buf: *mut libc::c_uchar,
    mut bytes: libc::c_int,
) {
    oggpack_readinit(b, buf, bytes);
}
/* Read in bits without advancing the bitptr; bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpack_look(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_int,
) -> libc::c_long {
    let mut ret: libc::c_ulong = 0;
    let mut m: libc::c_ulong = 0;
    if bits < 0 as libc::c_int || bits > 32 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    m = mask[bits as usize];
    bits += (*b).endbit;
    if (*b).endbyte >= (*b).storage - 4 as libc::c_int as libc::c_long {
        /* not the main path */
        if (*b).endbyte
            > (*b).storage - (bits + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        } else {
            /* special case to avoid reading b->ptr[0], which might be past the end of
            the buffer; also skips some useless accounting */
            if bits == 0 {
                return 0 as libc::c_long;
            }
        }
    }
    ret = (*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int >> (*b).endbit)
        as libc::c_ulong;
    if bits > 8 as libc::c_int {
        ret |= ((*(*b).ptr.offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int - (*b).endbit) as libc::c_ulong;
        if bits > 16 as libc::c_int {
            ret |= ((*(*b).ptr.offset(2 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int - (*b).endbit) as libc::c_ulong;
            if bits > 24 as libc::c_int {
                ret |= ((*(*b).ptr.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 24 as libc::c_int - (*b).endbit) as libc::c_ulong;
                if bits > 32 as libc::c_int && (*b).endbit != 0 {
                    ret |= ((*(*b).ptr.offset(4 as libc::c_int as isize) as libc::c_int)
                        << 32 as libc::c_int - (*b).endbit)
                        as libc::c_ulong
                }
            }
        }
    }
    return (m & ret) as libc::c_long;
}
/* Read in bits without advancing the bitptr; bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_look(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_int,
) -> libc::c_long {
    let mut ret: libc::c_ulong = 0;
    let mut m: libc::c_int = 32 as libc::c_int - bits;
    if m < 0 as libc::c_int || m > 32 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    bits += (*b).endbit;
    if (*b).endbyte >= (*b).storage - 4 as libc::c_int as libc::c_long {
        /* not the main path */
        if (*b).endbyte
            > (*b).storage - (bits + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int) as libc::c_long;
        } else {
            /* special case to avoid reading b->ptr[0], which might be past the end of
            the buffer; also skips some useless accounting */
            if bits == 0 {
                return 0 as libc::c_long;
            }
        }
    }
    ret = ((*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int)
        << 24 as libc::c_int + (*b).endbit) as libc::c_ulong;
    if bits > 8 as libc::c_int {
        ret |= ((*(*b).ptr.offset(1 as libc::c_int as isize) as libc::c_int)
            << 16 as libc::c_int + (*b).endbit) as libc::c_ulong;
        if bits > 16 as libc::c_int {
            ret |= ((*(*b).ptr.offset(2 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int + (*b).endbit) as libc::c_ulong;
            if bits > 24 as libc::c_int {
                ret |= ((*(*b).ptr.offset(3 as libc::c_int as isize) as libc::c_int) << (*b).endbit)
                    as libc::c_ulong;
                if bits > 32 as libc::c_int && (*b).endbit != 0 {
                    ret |= (*(*b).ptr.offset(4 as libc::c_int as isize) as libc::c_int
                        >> 8 as libc::c_int - (*b).endbit)
                        as libc::c_ulong
                }
            }
        }
    }
    return ((ret & 0xffffffff as libc::c_uint as libc::c_ulong)
        >> (m >> 1 as libc::c_int)
        >> (m + 1 as libc::c_int >> 1 as libc::c_int)) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_look1(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    if (*b).endbyte >= (*b).storage {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return (*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int >> (*b).endbit
        & 1 as libc::c_int) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_look1(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    if (*b).endbyte >= (*b).storage {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return (*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int
        >> 7 as libc::c_int - (*b).endbit
        & 1 as libc::c_int) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_adv(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_int,
) {
    bits += (*b).endbit;
    if (*b).endbyte > (*b).storage - (bits + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long {
        (*b).ptr = 0 as *mut libc::c_uchar;
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1 as libc::c_int;
        return;
    } else {
        (*b).ptr = (*b).ptr.offset((bits / 8 as libc::c_int) as isize);
        (*b).endbyte += (bits / 8 as libc::c_int) as libc::c_long;
        (*b).endbit = bits & 7 as libc::c_int;
        return;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_adv(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_int,
) {
    oggpack_adv(b, bits);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_adv1(mut b: *mut crate::ogg_h::oggpack_buffer) {
    (*b).endbit += 1;
    if (*b).endbit > 7 as libc::c_int {
        (*b).endbit = 0 as libc::c_int;
        (*b).ptr = (*b).ptr.offset(1);
        (*b).endbyte += 1
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_adv1(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_adv1(b);
}
/* bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpack_read(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_int,
) -> libc::c_long {
    let mut current_block: u64;
    let mut ret: libc::c_long = 0;
    let mut m: libc::c_ulong = 0;
    if !(bits < 0 as libc::c_int || bits > 32 as libc::c_int) {
        m = mask[bits as usize];
        bits += (*b).endbit;
        if (*b).endbyte >= (*b).storage - 4 as libc::c_int as libc::c_long {
            /* not the main path */
            if (*b).endbyte
                > (*b).storage - (bits + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long
            {
                current_block = 7073085723881536557;
            } else {
                /* special case to avoid reading b->ptr[0], which might be past the end of
                the buffer; also skips some useless accounting */
                if bits == 0 {
                    return 0 as libc::c_long;
                }
                current_block = 14523784380283086299;
            }
        } else {
            current_block = 14523784380283086299;
        }
        match current_block {
            7073085723881536557 => {}
            _ => {
                ret = (*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int >> (*b).endbit)
                    as libc::c_long;
                if bits > 8 as libc::c_int {
                    ret |= ((*(*b).ptr.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int - (*b).endbit)
                        as libc::c_long;
                    if bits > 16 as libc::c_int {
                        ret |= ((*(*b).ptr.offset(2 as libc::c_int as isize) as libc::c_int)
                            << 16 as libc::c_int - (*b).endbit)
                            as libc::c_long;
                        if bits > 24 as libc::c_int {
                            ret |= ((*(*b).ptr.offset(3 as libc::c_int as isize) as libc::c_int)
                                << 24 as libc::c_int - (*b).endbit)
                                as libc::c_long;
                            if bits > 32 as libc::c_int && (*b).endbit != 0 {
                                ret |= ((*(*b).ptr.offset(4 as libc::c_int as isize)
                                    as libc::c_int)
                                    << 32 as libc::c_int - (*b).endbit)
                                    as libc::c_long
                            }
                        }
                    }
                }
                ret = (ret as libc::c_ulong & m) as libc::c_long;
                (*b).ptr = (*b).ptr.offset((bits / 8 as libc::c_int) as isize);
                (*b).endbyte += (bits / 8 as libc::c_int) as libc::c_long;
                (*b).endbit = bits & 7 as libc::c_int;
                return ret;
            }
        }
    }
    (*b).ptr = 0 as *mut libc::c_uchar;
    (*b).endbyte = (*b).storage;
    (*b).endbit = 1 as libc::c_int;
    return -(1 as libc::c_long);
}
/* bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_read(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: libc::c_int,
) -> libc::c_long {
    let mut current_block: u64;
    let mut ret: libc::c_long = 0;
    let mut m: libc::c_long = (32 as libc::c_int - bits) as libc::c_long;
    if !(m < 0 as libc::c_int as libc::c_long || m > 32 as libc::c_int as libc::c_long) {
        bits += (*b).endbit;
        if (*b).endbyte + 4 as libc::c_int as libc::c_long >= (*b).storage {
            /* not the main path */
            if (*b).endbyte
                > (*b).storage - (bits + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long
            {
                current_block = 11946596175837608108;
            } else {
                /* special case to avoid reading b->ptr[0], which might be past the end of
                the buffer; also skips some useless accounting */
                if bits == 0 {
                    return 0 as libc::c_long;
                }
                current_block = 7351195479953500246;
            }
        } else {
            current_block = 7351195479953500246;
        }
        match current_block {
            11946596175837608108 => {}
            _ => {
                ret = ((*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 24 as libc::c_int + (*b).endbit) as libc::c_long;
                if bits > 8 as libc::c_int {
                    ret |= ((*(*b).ptr.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 16 as libc::c_int + (*b).endbit)
                        as libc::c_long;
                    if bits > 16 as libc::c_int {
                        ret |= ((*(*b).ptr.offset(2 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int + (*b).endbit)
                            as libc::c_long;
                        if bits > 24 as libc::c_int {
                            ret |= ((*(*b).ptr.offset(3 as libc::c_int as isize) as libc::c_int)
                                << (*b).endbit) as libc::c_long;
                            if bits > 32 as libc::c_int && (*b).endbit != 0 {
                                ret |= (*(*b).ptr.offset(4 as libc::c_int as isize) as libc::c_int
                                    >> 8 as libc::c_int - (*b).endbit)
                                    as libc::c_long
                            }
                        }
                    }
                }
                ret = ((ret as libc::c_ulong & 0xffffffff as libc::c_ulong)
                    >> (m >> 1 as libc::c_int)
                    >> (m + 1 as libc::c_int as libc::c_long >> 1 as libc::c_int))
                    as libc::c_long;
                (*b).ptr = (*b).ptr.offset((bits / 8 as libc::c_int) as isize);
                (*b).endbyte += (bits / 8 as libc::c_int) as libc::c_long;
                (*b).endbit = bits & 7 as libc::c_int;
                return ret;
            }
        }
    }
    (*b).ptr = 0 as *mut libc::c_uchar;
    (*b).endbyte = (*b).storage;
    (*b).endbit = 1 as libc::c_int;
    return -(1 as libc::c_long);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_read1(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    if (*b).endbyte >= (*b).storage {
        (*b).ptr = 0 as *mut libc::c_uchar;
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1 as libc::c_int;
        return -(1 as libc::c_long);
    } else {
        ret = (*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int >> (*b).endbit
            & 1 as libc::c_int) as libc::c_long;
        (*b).endbit += 1;
        if (*b).endbit > 7 as libc::c_int {
            (*b).endbit = 0 as libc::c_int;
            (*b).ptr = (*b).ptr.offset(1);
            (*b).endbyte += 1
        }
        return ret;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_read1(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    if (*b).endbyte >= (*b).storage {
        (*b).ptr = 0 as *mut libc::c_uchar;
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1 as libc::c_int;
        return -(1 as libc::c_long);
    } else {
        ret = (*(*b).ptr.offset(0 as libc::c_int as isize) as libc::c_int
            >> 7 as libc::c_int - (*b).endbit
            & 1 as libc::c_int) as libc::c_long;
        (*b).endbit += 1;
        if (*b).endbit > 7 as libc::c_int {
            (*b).endbit = 0 as libc::c_int;
            (*b).ptr = (*b).ptr.offset(1);
            (*b).endbyte += 1
        }
        return ret;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_bytes(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    return (*b).endbyte + (((*b).endbit + 7 as libc::c_int) / 8 as libc::c_int) as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_bits(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    return (*b).endbyte * 8 as libc::c_int as libc::c_long + (*b).endbit as libc::c_long;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_bytes(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    return oggpack_bytes(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_bits(mut b: *mut crate::ogg_h::oggpack_buffer) -> libc::c_long {
    return oggpack_bits(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_get_buffer(
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_uchar {
    return (*b).buffer;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_get_buffer(
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_uchar {
    return oggpack_get_buffer(b);
}
/* _V_SELFTEST */
/* Self test of the bitwise routines; everything else is based on
them, so they damned well better be solid. */
