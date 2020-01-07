use ::libc;

pub use crate::ogg_h::oggpack_buffer;

use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::realloc;

static mut mask: [usize; 33] = [
    0,
    0x1,
    0x3,
    0x7,
    0xf,
    0x1f,
    0x3f,
    0x7f,
    0xff,
    0x1ff,
    0x3ff,
    0x7ff,
    0xfff,
    0x1fff,
    0x3fff,
    0x7fff,
    0xffff,
    0x1ffffi32 as usize,
    0x3ffffi32 as usize,
    0x7ffffi32 as usize,
    0xfffffi32 as usize,
    0x1fffffi32 as usize,
    0x3fffffi32 as usize,
    0x7fffffi32 as usize,
    0xffffffi32 as usize,
    0x1ffffffi32 as usize,
    0x3ffffffi32 as usize,
    0x7ffffffi32 as usize,
    0xfffffffi32 as usize,
    0x1fffffffi32 as usize,
    0x3fffffffi32 as usize,
    0x7fffffffi32 as usize,
    0xffffffffu32 as usize,
];

static mut mask8B: [u32; 9] = [0, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xfe, 0xff];
/* Ogg BITSTREAM PRIMITIVES: bitstream ************************/
#[no_mangle]

pub unsafe extern "C" fn oggpack_writeinit(mut b: *mut crate::ogg_h::oggpack_buffer) {
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>(),
    );
    (*b).buffer = crate::stdlib::malloc(256) as *mut u8;
    (*b).ptr = (*b).buffer;
    *(*b).buffer.offset(0) = '\u{0}' as u8;
    (*b).storage = 256isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writeinit(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_writeinit(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writecheck(mut b: *mut crate::ogg_h::oggpack_buffer) -> i32 {
    if (*b).ptr.is_null() || (*b).storage == 0 {
        return -(1i32);
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writecheck(mut b: *mut crate::ogg_h::oggpack_buffer) -> i32 {
    return oggpack_writecheck(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writetrunc(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: isize,
) {
    let mut bytes: isize = bits >> 3;
    if !(*b).ptr.is_null() {
        bits -= bytes * 8;
        (*b).ptr = (*b).buffer.offset(bytes);
        (*b).endbit = bits as i32;
        (*b).endbyte = bytes;
        *(*b).ptr = (*(*b).ptr as usize & mask[bits as usize]) as u8
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writetrunc(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: isize,
) {
    let mut bytes: isize = bits >> 3;
    if !(*b).ptr.is_null() {
        bits -= bytes * 8;
        (*b).ptr = (*b).buffer.offset(bytes);
        (*b).endbit = bits as i32;
        (*b).endbyte = bytes;
        *(*b).ptr = (*(*b).ptr as u32 & mask8B[bits as usize]) as u8
    };
}
/* Takes only up to 32 bits. */
#[no_mangle]

pub unsafe extern "C" fn oggpack_write(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut value: usize,
    mut bits: i32,
) {
    let mut current_block: u64;
    if !(bits < 0 || bits > 32) {
        if (*b).endbyte >= (*b).storage - 4isize {
            let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
            if (*b).ptr.is_null() {
                return;
            }
            if (*b).storage > 9223372036854775807 as isize - 256 {
                current_block = 11736846078876452800;
            } else {
                ret = crate::stdlib::realloc(
                    (*b).buffer as *mut libc::c_void,
                    ((*b).storage + 256isize) as usize,
                );
                if ret.is_null() {
                    current_block = 11736846078876452800;
                } else {
                    (*b).buffer = ret as *mut u8;
                    (*b).storage += 256isize;
                    (*b).ptr = (*b).buffer.offset((*b).endbyte);
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
                let ref mut fresh0 = *(*b).ptr.offset(0);
                *fresh0 = (*fresh0 as usize | value << (*b).endbit) as u8;
                if bits >= 8 {
                    *(*b).ptr.offset(1) = (value >> 8 - (*b).endbit) as u8;
                    if bits >= 16 {
                        *(*b).ptr.offset(2) = (value >> 16 - (*b).endbit) as u8;
                        if bits >= 24 {
                            *(*b).ptr.offset(3) = (value >> 24 - (*b).endbit) as u8;
                            if bits >= 32 {
                                if (*b).endbit != 0 {
                                    *(*b).ptr.offset(4) = (value >> 32 - (*b).endbit) as u8
                                } else {
                                    *(*b).ptr.offset(4) = 0u8
                                }
                            }
                        }
                    }
                }
                (*b).endbyte += (bits / 8i32) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8i32) as isize);
                (*b).endbit = bits & 7;
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
    mut value: usize,
    mut bits: i32,
) {
    let mut current_block: u64;
    if !(bits < 0 || bits > 32) {
        if (*b).endbyte >= (*b).storage - 4isize {
            let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
            if (*b).ptr.is_null() {
                return;
            }
            if (*b).storage > 9223372036854775807 as isize - 256 {
                current_block = 2612814309992382393;
            } else {
                ret = crate::stdlib::realloc(
                    (*b).buffer as *mut libc::c_void,
                    ((*b).storage + 256isize) as usize,
                );
                if ret.is_null() {
                    current_block = 2612814309992382393;
                } else {
                    (*b).buffer = ret as *mut u8;
                    (*b).storage += 256isize;
                    (*b).ptr = (*b).buffer.offset((*b).endbyte);
                    current_block = 13109137661213826276;
                }
            }
        } else {
            current_block = 13109137661213826276;
        }
        match current_block {
            2612814309992382393 => {}
            _ => {
                value = (value & mask[bits as usize]) << 32 - bits;
                bits += (*b).endbit;
                let ref mut fresh1 = *(*b).ptr.offset(0);
                *fresh1 = (*fresh1 as usize | value >> 24 + (*b).endbit) as u8;
                if bits >= 8 {
                    *(*b).ptr.offset(1) = (value >> 16 + (*b).endbit) as u8;
                    if bits >= 16 {
                        *(*b).ptr.offset(2) = (value >> 8 + (*b).endbit) as u8;
                        if bits >= 24 {
                            *(*b).ptr.offset(3) = (value >> (*b).endbit) as u8;
                            if bits >= 32 {
                                if (*b).endbit != 0 {
                                    *(*b).ptr.offset(4) = (value << 8 - (*b).endbit) as u8
                                } else {
                                    *(*b).ptr.offset(4) = 0u8
                                }
                            }
                        }
                    }
                }
                (*b).endbyte += (bits / 8i32) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8i32) as isize);
                (*b).endbit = bits & 7;
                return;
            }
        }
    }
    oggpack_writeclear(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writealign(mut b: *mut crate::ogg_h::oggpack_buffer) {
    let mut bits: i32 = 8 - (*b).endbit;
    if bits < 8 {
        oggpack_write(b, 0usize, bits);
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writealign(mut b: *mut crate::ogg_h::oggpack_buffer) {
    let mut bits: i32 = 8 - (*b).endbit;
    if bits < 8 {
        oggpackB_write(b, 0usize, bits);
    };
}

unsafe extern "C" fn oggpack_writecopy_helper(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: isize,
    mut w: Option<
        unsafe extern "C" fn(_: *mut crate::ogg_h::oggpack_buffer, _: usize, _: i32) -> (),
    >,
    mut msb: i32,
) {
    let mut current_block: u64;
    let mut ptr: *mut u8 = source as *mut u8;
    let mut bytes: isize = bits / 8;
    let mut pbytes: isize = ((*b).endbit as isize + bits) / 8;
    bits -= bytes * 8;
    /* expand storage up-front */
    if (*b).endbyte + pbytes >= (*b).storage {
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*b).ptr.is_null() {
            current_block = 1692384543052803397;
        } else if (*b).storage > (*b).endbyte + pbytes + 256 {
            current_block = 1692384543052803397;
        } else {
            (*b).storage = (*b).endbyte + pbytes + 256;
            ret = crate::stdlib::realloc((*b).buffer as *mut libc::c_void, (*b).storage as usize);
            if ret.is_null() {
                current_block = 1692384543052803397;
            } else {
                (*b).buffer = ret as *mut u8;
                (*b).ptr = (*b).buffer.offset((*b).endbyte);
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
        let mut i: i32 = 0;
        /* unaligned copy.  Do it the hard way. */
        i = 0;
        while (i as isize) < bytes {
            w.expect("non-null function pointer")(b, *ptr.offset(i as isize) as usize, 8);
            i += 1
        }
    } else {
        /* aligned block copy */
        crate::stdlib::memmove((*b).ptr as *mut libc::c_void, source, bytes as usize);
        (*b).ptr = (*b).ptr.offset(bytes);
        (*b).endbyte += bytes;
        *(*b).ptr = 0u8
    }
    /* copy trailing bits */
    if bits != 0 {
        if msb != 0 {
            w.expect("non-null function pointer")(
                b,
                (*ptr.offset(bytes) as i32 >> 8isize - bits) as usize,
                bits as i32,
            );
        } else {
            w.expect("non-null function pointer")(b, *ptr.offset(bytes) as usize, bits as i32);
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writecopy(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: isize,
) {
    oggpack_writecopy_helper(
        b,
        source,
        bits,
        Some(
            oggpack_write
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: usize,
                    _: i32,
                ) -> (),
        ),
        0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writecopy(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: isize,
) {
    oggpack_writecopy_helper(
        b,
        source,
        bits,
        Some(
            oggpackB_write
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: usize,
                    _: i32,
                ) -> (),
        ),
        1,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_reset(mut b: *mut crate::ogg_h::oggpack_buffer) {
    if (*b).ptr.is_null() {
        return;
    }
    (*b).ptr = (*b).buffer;
    *(*b).buffer.offset(0) = 0u8;
    (*b).endbyte = 0isize;
    (*b).endbit = (*b).endbyte as i32;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_reset(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_reset(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writeclear(mut b: *mut crate::ogg_h::oggpack_buffer) {
    if !(*b).buffer.is_null() {
        crate::stdlib::free((*b).buffer as *mut libc::c_void);
    }
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writeclear(mut b: *mut crate::ogg_h::oggpack_buffer) {
    oggpack_writeclear(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_readinit(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut buf: *mut u8,
    mut bytes: i32,
) {
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>(),
    );
    (*b).ptr = buf;
    (*b).buffer = (*b).ptr;
    (*b).storage = bytes as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_readinit(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut buf: *mut u8,
    mut bytes: i32,
) {
    oggpack_readinit(b, buf, bytes);
}
/* Read in bits without advancing the bitptr; bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpack_look(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: i32,
) -> isize {
    let mut ret: usize = 0;
    let mut m: usize = 0;
    if bits < 0 || bits > 32 {
        return -1isize;
    }
    m = mask[bits as usize];
    bits += (*b).endbit;
    if (*b).endbyte >= (*b).storage - 4isize {
        /* not the main path */
        if (*b).endbyte > (*b).storage - (bits + 7i32 >> 3) as isize {
            return -1isize;
        } else {
            /* special case to avoid reading b->ptr[0], which might be past the end of
            the buffer; also skips some useless accounting */
            if bits == 0 {
                return 0isize;
            }
        }
    }
    ret = (*(*b).ptr.offset(0) as i32 >> (*b).endbit) as usize;
    if bits > 8 {
        ret |= ((*(*b).ptr.offset(1) as i32) << 8 - (*b).endbit) as usize;
        if bits > 16 {
            ret |= ((*(*b).ptr.offset(2) as i32) << 16 - (*b).endbit) as usize;
            if bits > 24 {
                ret |= ((*(*b).ptr.offset(3) as i32) << 24 - (*b).endbit) as usize;
                if bits > 32 && (*b).endbit != 0 {
                    ret |= ((*(*b).ptr.offset(4) as i32) << 32 - (*b).endbit) as usize
                }
            }
        }
    }
    return (m & ret) as isize;
}
/* Read in bits without advancing the bitptr; bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_look(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: i32,
) -> isize {
    let mut ret: usize = 0;
    let mut m: i32 = 32 - bits;
    if m < 0 || m > 32 {
        return -1isize;
    }
    bits += (*b).endbit;
    if (*b).endbyte >= (*b).storage - 4isize {
        /* not the main path */
        if (*b).endbyte > (*b).storage - (bits + 7i32 >> 3) as isize {
            return -1isize;
        } else {
            /* special case to avoid reading b->ptr[0], which might be past the end of
            the buffer; also skips some useless accounting */
            if bits == 0 {
                return 0isize;
            }
        }
    }
    ret = ((*(*b).ptr.offset(0) as i32) << 24 + (*b).endbit) as usize;
    if bits > 8 {
        ret |= ((*(*b).ptr.offset(1) as i32) << 16 + (*b).endbit) as usize;
        if bits > 16 {
            ret |= ((*(*b).ptr.offset(2) as i32) << 8 + (*b).endbit) as usize;
            if bits > 24 {
                ret |= ((*(*b).ptr.offset(3) as i32) << (*b).endbit) as usize;
                if bits > 32 && (*b).endbit != 0 {
                    ret |= (*(*b).ptr.offset(4) as i32 >> 8 - (*b).endbit) as usize
                }
            }
        }
    }
    return ((ret & 0xffffffffu32 as usize) >> (m >> 1) >> (m + 1 >> 1)) as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_look1(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    if (*b).endbyte >= (*b).storage {
        return -1isize;
    }
    return (*(*b).ptr.offset(0) as i32 >> (*b).endbit & 1) as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_look1(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    if (*b).endbyte >= (*b).storage {
        return -1isize;
    }
    return (*(*b).ptr.offset(0) as i32 >> 7 - (*b).endbit & 1) as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_adv(mut b: *mut crate::ogg_h::oggpack_buffer, mut bits: i32) {
    bits += (*b).endbit;
    if (*b).endbyte > (*b).storage - (bits + 7i32 >> 3) as isize {
        (*b).ptr = 0 as *mut u8;
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1;
        return;
    } else {
        (*b).ptr = (*b).ptr.offset((bits / 8i32) as isize);
        (*b).endbyte += (bits / 8i32) as isize;
        (*b).endbit = bits & 7;
        return;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_adv(mut b: *mut crate::ogg_h::oggpack_buffer, mut bits: i32) {
    oggpack_adv(b, bits);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_adv1(mut b: *mut crate::ogg_h::oggpack_buffer) {
    (*b).endbit += 1;
    if (*b).endbit > 7 {
        (*b).endbit = 0;
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
    mut bits: i32,
) -> isize {
    let mut current_block: u64;
    let mut ret: isize = 0;
    let mut m: usize = 0;
    if !(bits < 0 || bits > 32) {
        m = mask[bits as usize];
        bits += (*b).endbit;
        if (*b).endbyte >= (*b).storage - 4isize {
            /* not the main path */
            if (*b).endbyte > (*b).storage - (bits + 7i32 >> 3) as isize {
                current_block = 7073085723881536557;
            } else {
                /* special case to avoid reading b->ptr[0], which might be past the end of
                the buffer; also skips some useless accounting */
                if bits == 0 {
                    return 0isize;
                }
                current_block = 14523784380283086299;
            }
        } else {
            current_block = 14523784380283086299;
        }
        match current_block {
            7073085723881536557 => {}
            _ => {
                ret = (*(*b).ptr.offset(0) as i32 >> (*b).endbit) as isize;
                if bits > 8 {
                    ret |= ((*(*b).ptr.offset(1) as i32) << 8 - (*b).endbit) as isize;
                    if bits > 16 {
                        ret |= ((*(*b).ptr.offset(2) as i32) << 16 - (*b).endbit) as isize;
                        if bits > 24 {
                            ret |= ((*(*b).ptr.offset(3) as i32) << 24 - (*b).endbit) as isize;
                            if bits > 32 && (*b).endbit != 0 {
                                ret |= ((*(*b).ptr.offset(4) as i32) << 32 - (*b).endbit) as isize
                            }
                        }
                    }
                }
                ret = (ret as usize & m) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8i32) as isize);
                (*b).endbyte += (bits / 8i32) as isize;
                (*b).endbit = bits & 7;
                return ret;
            }
        }
    }
    (*b).ptr = 0 as *mut u8;
    (*b).endbyte = (*b).storage;
    (*b).endbit = 1;
    return -(1isize);
}
/* bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_read(
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut bits: i32,
) -> isize {
    let mut current_block: u64;
    let mut ret: isize = 0;
    let mut m: isize = (32i32 - bits) as isize;
    if !(m < 0 || m > 32) {
        bits += (*b).endbit;
        if (*b).endbyte + 4isize >= (*b).storage {
            /* not the main path */
            if (*b).endbyte > (*b).storage - (bits + 7i32 >> 3) as isize {
                current_block = 11946596175837608108;
            } else {
                /* special case to avoid reading b->ptr[0], which might be past the end of
                the buffer; also skips some useless accounting */
                if bits == 0 {
                    return 0isize;
                }
                current_block = 7351195479953500246;
            }
        } else {
            current_block = 7351195479953500246;
        }
        match current_block {
            11946596175837608108 => {}
            _ => {
                ret = ((*(*b).ptr.offset(0) as i32) << 24 + (*b).endbit) as isize;
                if bits > 8 {
                    ret |= ((*(*b).ptr.offset(1) as i32) << 16 + (*b).endbit) as isize;
                    if bits > 16 {
                        ret |= ((*(*b).ptr.offset(2) as i32) << 8 + (*b).endbit) as isize;
                        if bits > 24 {
                            ret |= ((*(*b).ptr.offset(3) as i32) << (*b).endbit) as isize;
                            if bits > 32 && (*b).endbit != 0 {
                                ret |= (*(*b).ptr.offset(4) as i32 >> 8 - (*b).endbit) as isize
                            }
                        }
                    }
                }
                ret = ((ret as usize & 0xffffffff as usize) >> (m >> 1) >> (m + 1 >> 1)) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8i32) as isize);
                (*b).endbyte += (bits / 8i32) as isize;
                (*b).endbit = bits & 7;
                return ret;
            }
        }
    }
    (*b).ptr = 0 as *mut u8;
    (*b).endbyte = (*b).storage;
    (*b).endbit = 1;
    return -(1isize);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_read1(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    let mut ret: isize = 0;
    if (*b).endbyte >= (*b).storage {
        (*b).ptr = 0 as *mut u8;
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1;
        return -(1isize);
    } else {
        ret = (*(*b).ptr.offset(0) as i32 >> (*b).endbit & 1) as isize;
        (*b).endbit += 1;
        if (*b).endbit > 7 {
            (*b).endbit = 0;
            (*b).ptr = (*b).ptr.offset(1);
            (*b).endbyte += 1
        }
        return ret;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_read1(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    let mut ret: isize = 0;
    if (*b).endbyte >= (*b).storage {
        (*b).ptr = 0 as *mut u8;
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1;
        return -(1isize);
    } else {
        ret = (*(*b).ptr.offset(0) as i32 >> 7 - (*b).endbit & 1) as isize;
        (*b).endbit += 1;
        if (*b).endbit > 7 {
            (*b).endbit = 0;
            (*b).ptr = (*b).ptr.offset(1);
            (*b).endbyte += 1
        }
        return ret;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_bytes(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    return (*b).endbyte + (((*b).endbit + 7i32) / 8) as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_bits(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    return (*b).endbyte * 8 + (*b).endbit as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_bytes(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    return oggpack_bytes(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_bits(mut b: *mut crate::ogg_h::oggpack_buffer) -> isize {
    return oggpack_bits(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_get_buffer(mut b: *mut crate::ogg_h::oggpack_buffer) -> *mut u8 {
    return (*b).buffer;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_get_buffer(mut b: *mut crate::ogg_h::oggpack_buffer) -> *mut u8 {
    return oggpack_get_buffer(b);
}
/* _V_SELFTEST */
/* Self test of the bitwise routines; everything else is based on
them, so they damned well better be solid. */
