use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::ogg_iovec_t;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::ogg_page;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;
use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memchr;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::realloc;
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE Ogg CONTAINER SOURCE CODE.              *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2010             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: code raw packets into framed OggSquish stream and
          decode Ogg streams back into raw packets
last mod: $Id$

note: The CRC code is directly derived from public domain code by
Ross Williams (ross@guest.adelaide.edu.au).  See docs/framing.html
for details.

********************************************************************/
/* A complete description of Ogg framing exists in docs/framing.html */
#[no_mangle]

pub unsafe extern "C" fn ogg_page_version(mut og: *const crate::ogg_h::ogg_page) -> i32 {
    return *(*og).header.offset(4) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_continued(mut og: *const crate::ogg_h::ogg_page) -> i32 {
    return *(*og).header.offset(5) as i32 & 0x1;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_bos(mut og: *const crate::ogg_h::ogg_page) -> i32 {
    return *(*og).header.offset(5) as i32 & 0x2;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_eos(mut og: *const crate::ogg_h::ogg_page) -> i32 {
    return *(*og).header.offset(5) as i32 & 0x4;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_granulepos(
    mut og: *const crate::ogg_h::ogg_page,
) -> crate::config_types_h::ogg_int64_t {
    let mut page: *mut u8 = (*og).header;
    let mut granulepos: crate::config_types_h::ogg_int64_t =
        (*page.offset(13) as i32 & 0xff) as crate::config_types_h::ogg_int64_t;
    granulepos = granulepos << 8 | (*page.offset(12) as i32 & 0xff) as isize;
    granulepos = granulepos << 8 | (*page.offset(11) as i32 & 0xff) as isize;
    granulepos = granulepos << 8 | (*page.offset(10) as i32 & 0xff) as isize;
    granulepos = granulepos << 8 | (*page.offset(9) as i32 & 0xff) as isize;
    granulepos = granulepos << 8 | (*page.offset(8) as i32 & 0xff) as isize;
    granulepos = granulepos << 8 | (*page.offset(7) as i32 & 0xff) as isize;
    granulepos = granulepos << 8 | (*page.offset(6) as i32 & 0xff) as isize;
    return granulepos;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_serialno(mut og: *const crate::ogg_h::ogg_page) -> i32 {
    return *(*og).header.offset(14) as i32
        | (*(*og).header.offset(15) as i32) << 8
        | (*(*og).header.offset(16) as i32) << 16
        | (*(*og).header.offset(17) as i32) << 24;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_pageno(mut og: *const crate::ogg_h::ogg_page) -> isize {
    return (*(*og).header.offset(18) as i32
        | (*(*og).header.offset(19) as i32) << 8
        | (*(*og).header.offset(20) as i32) << 16
        | (*(*og).header.offset(21) as i32) << 24) as isize;
}
/* returns the number of packets that are completed on this page (if
the leading packet is begun on a previous page, but ends on this
page, it's counted */
/* NOTE:
   If a page consists of a packet begun on a previous page, and a new
   packet begun (but not completed) on this page, the return will be:
     ogg_page_packets(page)   ==1,
     ogg_page_continued(page) !=0

   If a page happens to be a single packet that was begun on a
   previous page, and spans to the next page (in the case of a three or
   more page packet), the return will be:
     ogg_page_packets(page)   ==0,
     ogg_page_continued(page) !=0
*/
#[no_mangle]

pub unsafe extern "C" fn ogg_page_packets(mut og: *const crate::ogg_h::ogg_page) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = *(*og).header.offset(26) as i32;
    let mut count: i32 = 0;
    i = 0;
    while i < n {
        if (*(*og).header.offset((27 + i) as isize) as i32) < 255 {
            count += 1
        }
        i += 1
    }
    return count;
}

static mut crc_lookup: [crate::config_types_h::ogg_uint32_t; 256] = [
    0, 0x4c11db7, 0x9823b6e, 0xd4326d9, 0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005, 0x2608edb8,
    0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61, 0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd, 0x4c11db70,
    0x48d0c6c7, 0x4593e01e, 0x4152fda9, 0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75, 0x6a1936c8,
    0x6ed82b7f, 0x639b0da6, 0x675a1011, 0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd, 0x9823b6e0,
    0x9ce2ab57, 0x91a18d8e, 0x95609039, 0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5, 0xbe2b5b58,
    0xbaea46ef, 0xb7a96036, 0xb3687d81, 0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d, 0xd4326d90,
    0xd0f37027, 0xddb056fe, 0xd9714b49, 0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95, 0xf23a8028,
    0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1, 0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d, 0x34867077,
    0x30476dc0, 0x3d044b19, 0x39c556ae, 0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072, 0x128e9dcf,
    0x164f8078, 0x1b0ca6a1, 0x1fcdbb16, 0x18aeb13, 0x54bf6a4, 0x808d07d, 0xcc9cdca, 0x7897ab07,
    0x7c56b6b0, 0x71159069, 0x75d48dde, 0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02, 0x5e9f46bf,
    0x5a5e5b08, 0x571d7dd1, 0x53dc6066, 0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba, 0xaca5c697,
    0xa864db20, 0xa527fdf9, 0xa1e6e04e, 0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692, 0x8aad2b2f,
    0x8e6c3698, 0x832f1041, 0x87ee0df6, 0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a, 0xe0b41de7,
    0xe4750050, 0xe9362689, 0xedf73b3e, 0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2, 0xc6bcf05f,
    0xc27dede8, 0xcf3ecb31, 0xcbffd686, 0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a, 0x690ce0ee,
    0x6dcdfd59, 0x608edb80, 0x644fc637, 0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb, 0x4f040d56,
    0x4bc510e1, 0x46863638, 0x42472b8f, 0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53, 0x251d3b9e,
    0x21dc2629, 0x2c9f00f0, 0x285e1d47, 0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b, 0x315d626,
    0x7d4cb91, 0xa97ed48, 0xe56f0ff, 0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623, 0xf12f560e,
    0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7, 0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b, 0xd727bbb6,
    0xd3e6a601, 0xdea580d8, 0xda649d6f, 0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3, 0xbd3e8d7e,
    0xb9ff90c9, 0xb4bcb610, 0xb07daba7, 0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b, 0x9b3660c6,
    0x9ff77d71, 0x92b45ba8, 0x9675461f, 0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3, 0x5d8a9099,
    0x594b8d2e, 0x5408abf7, 0x50c9b640, 0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c, 0x7b827d21,
    0x7f436096, 0x7200464f, 0x76c15bf8, 0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24, 0x119b4be9,
    0x155a565e, 0x18197087, 0x1cd86d30, 0x29f3d35, 0x65e2082, 0xb1d065b, 0xfdc1bec, 0x3793a651,
    0x3352bbe6, 0x3e119d3f, 0x3ad08088, 0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654, 0xc5a92679,
    0xc1683bce, 0xcc2b1d17, 0xc8ea00a0, 0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c, 0xe3a1cbc1,
    0xe760d676, 0xea23f0af, 0xeee2ed18, 0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4, 0x89b8fd09,
    0x8d79e0be, 0x803ac667, 0x84fbdbd0, 0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c, 0xafb010b1,
    0xab710d06, 0xa6322bdf, 0xa2f33668, 0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4,
];
/* Ogg BITSTREAM PRIMITIVES: general ***************************/
/* init the encode/decode logical stream state */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_init(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut serialno: i32,
) -> i32 {
    if !os.is_null() {
        crate::stdlib::memset(
            os as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::ogg_h::ogg_stream_state>(),
        );
        (*os).body_storage = (16i32 * 1024) as isize;
        (*os).lacing_storage = 1024isize;
        (*os).body_data = crate::stdlib::malloc(
            ((*os).body_storage as usize).wrapping_mul(::std::mem::size_of::<u8>()),
        ) as *mut u8;
        (*os).lacing_vals = crate::stdlib::malloc(
            ((*os).lacing_storage as usize).wrapping_mul(::std::mem::size_of::<i32>()),
        ) as *mut i32;
        (*os).granule_vals = crate::stdlib::malloc(
            ((*os).lacing_storage as usize)
                .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_int64_t>()),
        ) as *mut crate::config_types_h::ogg_int64_t;
        if (*os).body_data.is_null() || (*os).lacing_vals.is_null() || (*os).granule_vals.is_null()
        {
            ogg_stream_clear(os);
            return -(1i32);
        }
        (*os).serialno = serialno as isize;
        return 0i32;
    }
    return -(1);
}
/* async/delayed error detection for the ogg_stream_state */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_check(mut os: *mut crate::ogg_h::ogg_stream_state) -> i32 {
    if os.is_null() || (*os).body_data.is_null() {
        return -(1i32);
    }
    return 0;
}
/* _clear does not free os, only the non-flat storage within */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_clear(mut os: *mut crate::ogg_h::ogg_stream_state) -> i32 {
    if !os.is_null() {
        if !(*os).body_data.is_null() {
            crate::stdlib::free((*os).body_data as *mut libc::c_void);
        }
        if !(*os).lacing_vals.is_null() {
            crate::stdlib::free((*os).lacing_vals as *mut libc::c_void);
        }
        if !(*os).granule_vals.is_null() {
            crate::stdlib::free((*os).granule_vals as *mut libc::c_void);
        }
        crate::stdlib::memset(
            os as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::ogg_h::ogg_stream_state>(),
        );
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_destroy(mut os: *mut crate::ogg_h::ogg_stream_state) -> i32 {
    if !os.is_null() {
        ogg_stream_clear(os);
        crate::stdlib::free(os as *mut libc::c_void);
    }
    return 0;
}
/* Helpers for ogg_stream_encode; this keeps the structure and
what's happening fairly clear */

unsafe extern "C" fn _os_body_expand(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut needed: isize,
) -> i32 {
    if (*os).body_storage - needed <= (*os).body_fill {
        let mut body_storage: isize = 0;
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*os).body_storage > 9223372036854775807 as isize - needed {
            ogg_stream_clear(os);
            return -(1i32);
        }
        body_storage = (*os).body_storage + needed;
        if body_storage < 9223372036854775807 as isize - 1024 {
            body_storage += 1024
        }
        ret = crate::stdlib::realloc(
            (*os).body_data as *mut libc::c_void,
            (body_storage as usize).wrapping_mul(::std::mem::size_of::<u8>()),
        );
        if ret.is_null() {
            ogg_stream_clear(os);
            return -(1i32);
        }
        (*os).body_storage = body_storage;
        (*os).body_data = ret as *mut u8
    }
    return 0;
}

unsafe extern "C" fn _os_lacing_expand(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut needed: isize,
) -> i32 {
    if (*os).lacing_storage - needed <= (*os).lacing_fill {
        let mut lacing_storage: isize = 0;
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*os).lacing_storage > 9223372036854775807 as isize - needed {
            ogg_stream_clear(os);
            return -(1i32);
        }
        lacing_storage = (*os).lacing_storage + needed;
        if lacing_storage < 9223372036854775807 as isize - 32 {
            lacing_storage += 32
        }
        ret = crate::stdlib::realloc(
            (*os).lacing_vals as *mut libc::c_void,
            (lacing_storage as usize).wrapping_mul(::std::mem::size_of::<i32>()),
        );
        if ret.is_null() {
            ogg_stream_clear(os);
            return -(1i32);
        }
        (*os).lacing_vals = ret as *mut i32;
        ret = crate::stdlib::realloc(
            (*os).granule_vals as *mut libc::c_void,
            (lacing_storage as usize)
                .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_int64_t>()),
        );
        if ret.is_null() {
            ogg_stream_clear(os);
            return -(1i32);
        }
        (*os).granule_vals = ret as *mut crate::config_types_h::ogg_int64_t;
        (*os).lacing_storage = lacing_storage
    }
    return 0;
}
/* checksum the page */
/* Direct table CRC; note that this will be faster in the future if we
perform the checksum simultaneously with other copies */
#[no_mangle]

pub unsafe extern "C" fn ogg_page_checksum_set(mut og: *mut crate::ogg_h::ogg_page) {
    if !og.is_null() {
        let mut crc_reg: crate::config_types_h::ogg_uint32_t = 0;
        let mut i: i32 = 0;
        /* safety; needed for API behavior, but not framing code */
        *(*og).header.offset(22) = 0u8;
        *(*og).header.offset(23) = 0u8;
        *(*og).header.offset(24) = 0u8;
        *(*og).header.offset(25) = 0u8;
        i = 0;
        while (i as isize) < (*og).header_len {
            crc_reg = crc_reg << 8
                ^ crc_lookup
                    [(crc_reg >> 24 & 0xff ^ *(*og).header.offset(i as isize) as u32) as usize];
            i += 1
        }
        i = 0;
        while (i as isize) < (*og).body_len {
            crc_reg = crc_reg << 8
                ^ crc_lookup
                    [(crc_reg >> 24 & 0xff ^ *(*og).body.offset(i as isize) as u32) as usize];
            i += 1
        }
        *(*og).header.offset(22) = (crc_reg & 0xff) as u8;
        *(*og).header.offset(23) = (crc_reg >> 8 & 0xff) as u8;
        *(*og).header.offset(24) = (crc_reg >> 16 & 0xff) as u8;
        *(*og).header.offset(25) = (crc_reg >> 24 & 0xff) as u8
    };
}
/* submit data to the internal buffer of the framing engine */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_iovecin(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut iov: *mut crate::ogg_h::ogg_iovec_t,
    mut count: i32,
    mut e_o_s: isize,
    mut granulepos: crate::config_types_h::ogg_int64_t,
) -> i32 {
    let mut bytes: isize = 0;
    let mut lacing_vals: isize = 0;
    let mut i: i32 = 0;
    if ogg_stream_check(os) != 0 {
        return -(1i32);
    }
    if iov.is_null() {
        return 0i32;
    }
    i = 0;
    while i < count {
        if (*iov.offset(i as isize)).iov_len > 9223372036854775807 as usize {
            return -(1i32);
        }
        if bytes > 9223372036854775807 as isize - (*iov.offset(i as isize)).iov_len as isize {
            return -(1i32);
        }
        bytes += (*iov.offset(i as isize)).iov_len as isize;
        i += 1
    }
    lacing_vals = bytes / 255 + 1;
    if (*os).body_returned != 0 {
        /* advance packet data according to the body_returned pointer. We
        had to keep it around to return a pointer into the buffer last
        call */
        (*os).body_fill -= (*os).body_returned;
        if (*os).body_fill != 0 {
            crate::stdlib::memmove(
                (*os).body_data as *mut libc::c_void,
                (*os).body_data.offset((*os).body_returned) as *const libc::c_void,
                (*os).body_fill as usize,
            );
        }
        (*os).body_returned = 0isize
    }
    /* make sure we have the buffer storage */
    if _os_body_expand(os, bytes) != 0 || _os_lacing_expand(os, lacing_vals) != 0 {
        return -(1i32);
    }
    /* Copy in the submitted packet.  Yes, the copy is a waste; this is
    the liability of overly clean abstraction for the time being.  It
    will actually be fairly easy to eliminate the extra copy in the
    future */
    i = 0;
    while i < count {
        crate::stdlib::memcpy(
            (*os).body_data.offset((*os).body_fill) as *mut libc::c_void,
            (*iov.offset(i as isize)).iov_base,
            (*iov.offset(i as isize)).iov_len,
        );
        (*os).body_fill += (*iov.offset(i as isize)).iov_len as i32 as isize;
        i += 1
    }
    /* Store lacing vals for this packet */
    i = 0;
    while (i as isize) < lacing_vals - 1 {
        *(*os).lacing_vals.offset((*os).lacing_fill + i as isize) = 255;
        *(*os).granule_vals.offset((*os).lacing_fill + i as isize) = (*os).granulepos;
        i += 1
    }
    *(*os).lacing_vals.offset((*os).lacing_fill + i as isize) = (bytes % 255) as i32;
    let ref mut fresh0 = *(*os).granule_vals.offset((*os).lacing_fill + i as isize);
    *fresh0 = granulepos;
    (*os).granulepos = *fresh0;
    /* flag the first segment as the beginning of the packet */
    *(*os).lacing_vals.offset((*os).lacing_fill) |= 0x100;
    (*os).lacing_fill += lacing_vals;
    /* for the sake of completeness */
    (*os).packetno += 1;
    if e_o_s != 0 {
        (*os).e_o_s = 1
    }
    return 0;
}
/* Ogg BITSTREAM PRIMITIVES: encoding **************************/
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_packetin(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut iov: crate::ogg_h::ogg_iovec_t = crate::ogg_h::ogg_iovec_t {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    iov.iov_base = (*op).packet as *mut libc::c_void;
    iov.iov_len = (*op).bytes as crate::stddef_h::size_t;
    return ogg_stream_iovecin(os, &mut iov, 1, (*op).e_o_s, (*op).granulepos);
}
/* Conditionally flush a page; force==0 will only flush nominal-size
pages, force==1 forces us to flush a page regardless of page size
so long as there's any data available at all. */

unsafe extern "C" fn ogg_stream_flush_i(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
    mut force: i32,
    mut nfill: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut vals: i32 = 0;
    let mut maxvals: i32 = if (*os).lacing_fill > 255isize {
        255isize
    } else {
        (*os).lacing_fill
    } as i32;
    let mut bytes: i32 = 0;
    let mut acc: isize = 0;
    let mut granule_pos: crate::config_types_h::ogg_int64_t = -1;
    if ogg_stream_check(os) != 0 {
        return 0i32;
    }
    if maxvals == 0 {
        return 0i32;
    }
    /* construct a page */
    /* decide how many segments to include */
    /* If this is the initial header case, the first page must only include
    the initial header packet */
    if (*os).b_o_s == 0 {
        /* 'initial header page' case */
        granule_pos = 0;
        vals = 0;
        while vals < maxvals {
            if (*(*os).lacing_vals.offset(vals as isize) & 0xff) < 255 {
                vals += 1;
                break;
            } else {
                vals += 1
            }
        }
    } else {
        /* The extra packets_done, packet_just_done logic here attempts to do two things:
        1) Don't unneccessarily span pages.
        2) Unless necessary, don't flush pages if there are less than four packets on
           them; this expands page size to reduce unneccessary overhead if incoming packets
           are large.
        These are not necessary behaviors, just 'always better than naive flushing'
        without requiring an application to explicitly request a specific optimized
        behavior. We'll want an explicit behavior setup pathway eventually as well. */
        let mut packets_done: i32 = 0;
        let mut packet_just_done: i32 = 0;
        vals = 0;
        while vals < maxvals {
            if acc > nfill as isize && packet_just_done >= 4 {
                force = 1;
                break;
            } else {
                acc += (*(*os).lacing_vals.offset(vals as isize) & 0xffi32) as isize;
                if (*(*os).lacing_vals.offset(vals as isize) & 0xff) < 255 {
                    granule_pos = *(*os).granule_vals.offset(vals as isize);
                    packets_done += 1;
                    packet_just_done = packets_done
                } else {
                    packet_just_done = 0
                }
                vals += 1
            }
        }
        if vals == 255 {
            force = 1
        }
    }
    if force == 0 {
        return 0i32;
    }
    /* construct the header in temp storage */
    crate::stdlib::memcpy(
        (*os).header.as_mut_ptr() as *mut libc::c_void,
        b"OggS\x00" as *const u8 as *const libc::c_void,
        4,
    );
    /* stream structure version */
    (*os).header[4] = 0u8;
    /* continued packet flag? */
    (*os).header[5] = 0u8;
    if *(*os).lacing_vals.offset(0) & 0x100 == 0 {
        (*os).header[5] = ((*os).header[5] as i32 | 0x1) as u8
    }
    /* first page flag? */
    if (*os).b_o_s == 0 {
        (*os).header[5] = ((*os).header[5] as i32 | 0x2) as u8
    }
    /* last page flag? */
    if (*os).e_o_s != 0 && (*os).lacing_fill == vals as isize {
        (*os).header[5] = ((*os).header[5] as i32 | 0x4) as u8
    }
    (*os).b_o_s = 1;
    /* 64 bits of PCM position */
    i = 6;
    while i < 14 {
        (*os).header[i as usize] = (granule_pos & 0xff) as u8;
        granule_pos >>= 8isize;
        i += 1
    }
    /* 32 bits of stream serial number */
    let mut serialno: isize = (*os).serialno;
    i = 14;
    while i < 18 {
        (*os).header[i as usize] = (serialno & 0xff) as u8;
        serialno >>= 8isize;
        i += 1
    }
    /* 32 bits of page counter (we have both counter and page header
    because this val can roll over) */
    if (*os).pageno == -1isize {
        (*os).pageno = 0isize
    } /* because someone called
      stream_reset; this would be a
      strange thing to do in an
      encode stream, but it has
      plausible uses */
    let fresh1 = (*os).pageno;
    (*os).pageno = (*os).pageno + 1;
    let mut pageno: isize = fresh1;
    i = 18;
    while i < 22 {
        (*os).header[i as usize] = (pageno & 0xff) as u8;
        pageno >>= 8isize;
        i += 1
    }
    /* zero for computation; filled in later */
    (*os).header[22] = 0u8;
    (*os).header[23] = 0u8;
    (*os).header[24] = 0u8;
    (*os).header[25] = 0u8;
    /* segment table */
    (*os).header[26] = (vals & 0xff) as u8;
    i = 0;
    while i < vals {
        (*os).header[(i + 27) as usize] = (*(*os).lacing_vals.offset(i as isize) & 0xffi32) as u8;
        bytes += (*os).header[(i + 27) as usize] as i32;
        i += 1
    }
    /* set pointers in the ogg_page struct */
    (*og).header = (*os).header.as_mut_ptr();
    (*os).header_fill = vals + 27;
    (*og).header_len = (*os).header_fill as isize;
    (*og).body = (*os).body_data.offset((*os).body_returned);
    (*og).body_len = bytes as isize;
    /* advance the lacing data and set the body_returned pointer */
    (*os).lacing_fill -= vals as isize;
    crate::stdlib::memmove(
        (*os).lacing_vals as *mut libc::c_void,
        (*os).lacing_vals.offset(vals as isize) as *const libc::c_void,
        ((*os).lacing_fill as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    crate::stdlib::memmove(
        (*os).granule_vals as *mut libc::c_void,
        (*os).granule_vals.offset(vals as isize) as *const libc::c_void,
        ((*os).lacing_fill as usize)
            .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_int64_t>()),
    );
    (*os).body_returned += bytes as isize;
    /* calculate the checksum */
    ogg_page_checksum_set(og);
    /* done */
    return 1;
}
/* This will flush remaining packets into a page (returning nonzero),
even if there is not enough data to trigger a flush normally
(undersized page). If there are no packets or partial packets to
flush, ogg_stream_flush returns 0.  Note that ogg_stream_flush will
try to flush a normal sized page like ogg_stream_pageout; a call to
ogg_stream_flush does not guarantee that all packets have flushed.
Only a return value of 0 from ogg_stream_flush indicates all packet
data is flushed into pages.

since ogg_stream_flush will flush the last page in a stream even if
it's undersized, you almost certainly want to use ogg_stream_pageout
(and *not* ogg_stream_flush) unless you specifically need to flush
a page regardless of size in the middle of a stream. */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_flush(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> i32 {
    return ogg_stream_flush_i(os, og, 1, 4096);
}
/* Like the above, but an argument is provided to adjust the nominal
page size for applications which are smart enough to provide their
own delay based flushing */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_flush_fill(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
    mut nfill: i32,
) -> i32 {
    return ogg_stream_flush_i(os, og, 1, nfill);
}
/* This constructs pages from buffered packet segments.  The pointers
returned are to static buffers; do not free. The returned buffers are
good only until the next call (using the same ogg_stream_state) */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_pageout(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> i32 {
    let mut force: i32 = 0;
    if ogg_stream_check(os) != 0 {
        return 0i32;
    }
    if (*os).e_o_s != 0 && (*os).lacing_fill != 0 || (*os).lacing_fill != 0 && (*os).b_o_s == 0 {
        /* 'initial header page' case */
        force = 1
    }
    return ogg_stream_flush_i(os, og, force, 4096);
}
/* Like the above, but an argument is provided to adjust the nominal
page size for applications which are smart enough to provide their
own delay based flushing */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_pageout_fill(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
    mut nfill: i32,
) -> i32 {
    let mut force: i32 = 0;
    if ogg_stream_check(os) != 0 {
        return 0i32;
    }
    if (*os).e_o_s != 0 && (*os).lacing_fill != 0 || (*os).lacing_fill != 0 && (*os).b_o_s == 0 {
        /* 'initial header page' case */
        force = 1
    }
    return ogg_stream_flush_i(os, og, force, nfill);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_eos(mut os: *mut crate::ogg_h::ogg_stream_state) -> i32 {
    if ogg_stream_check(os) != 0 {
        return 1i32;
    }
    return (*os).e_o_s;
}
/* Ogg BITSTREAM PRIMITIVES: decoding **************************/
/* DECODING PRIMITIVES: packet streaming layer **********************/
/* This has two layers to place more of the multi-serialno and paging
control in the application's hands.  First, we expose a data buffer
using ogg_sync_buffer().  The app either copies into the
buffer, or passes it directly to read(), etc.  We then call
ogg_sync_wrote() to tell how many bytes we just added.

Pages are returned (pointers into the buffer in ogg_sync_state)
by ogg_sync_pageout().  The page is then submitted to
ogg_stream_pagein() along with the appropriate
ogg_stream_state* (ie, matching serialno).  We then get raw
packets out calling ogg_stream_packetout() with a
ogg_stream_state. */
/* initialize the struct to a known state */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_init(mut oy: *mut crate::ogg_h::ogg_sync_state) -> i32 {
    if !oy.is_null() {
        (*oy).storage = -(1); /* used as a readiness flag */
        crate::stdlib::memset(
            oy as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::ogg_h::ogg_sync_state>(),
        );
    }
    return 0;
}
/* clear non-flat storage within */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_clear(mut oy: *mut crate::ogg_h::ogg_sync_state) -> i32 {
    if !oy.is_null() {
        if !(*oy).data.is_null() {
            crate::stdlib::free((*oy).data as *mut libc::c_void);
        }
        crate::stdlib::memset(
            oy as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::ogg_h::ogg_sync_state>(),
        );
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_destroy(mut oy: *mut crate::ogg_h::ogg_sync_state) -> i32 {
    if !oy.is_null() {
        ogg_sync_clear(oy);
        crate::stdlib::free(oy as *mut libc::c_void);
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_check(mut oy: *mut crate::ogg_h::ogg_sync_state) -> i32 {
    if (*oy).storage < 0 {
        return -(1i32);
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_buffer(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut size: isize,
) -> *mut i8 {
    if ogg_sync_check(oy) != 0 {
        return 0 as *mut i8;
    }
    /* first, clear out any space that has been previously returned */
    if (*oy).returned != 0 {
        (*oy).fill -= (*oy).returned;
        if (*oy).fill > 0 {
            crate::stdlib::memmove(
                (*oy).data as *mut libc::c_void,
                (*oy).data.offset((*oy).returned as isize) as *const libc::c_void,
                (*oy).fill as usize,
            );
        }
        (*oy).returned = 0
    }
    if size > ((*oy).storage - (*oy).fill) as isize {
        /* We need to extend the internal buffer */
        let mut newsize: isize = size + (*oy).fill as isize + 4096; /* an extra page to be nice */
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if !(*oy).data.is_null() {
            ret = crate::stdlib::realloc((*oy).data as *mut libc::c_void, newsize as usize)
        } else {
            ret = crate::stdlib::malloc(newsize as usize)
        }
        if ret.is_null() {
            ogg_sync_clear(oy);
            return 0 as *mut i8;
        }
        (*oy).data = ret as *mut u8;
        (*oy).storage = newsize as i32
    }
    /* expose a segment at least as large as requested at the fill mark */
    return ((*oy).data as *mut i8).offset((*oy).fill as isize);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_wrote(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut bytes: isize,
) -> i32 {
    if ogg_sync_check(oy) != 0 {
        return -(1i32);
    }
    if (*oy).fill as isize + bytes > (*oy).storage as isize {
        return -(1i32);
    }
    (*oy).fill = ((*oy).fill as isize + bytes) as i32;
    return 0;
}
/* sync the stream.  This is meant to be useful for finding page
   boundaries.

   return values for this:
  -n) skipped n bytes
   0) page not ready; more data (no bytes skipped)
   n) page synced at current location; page length n bytes

*/
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_pageseek(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> isize {
    let mut current_block: u64; /* not enough for a header */
    let mut page: *mut u8 = (*oy).data.offset((*oy).returned as isize);
    let mut next: *mut u8 = 0 as *mut u8;
    let mut bytes: isize = ((*oy).fill - (*oy).returned) as isize;
    if ogg_sync_check(oy) != 0 {
        return 0isize;
    }
    if (*oy).headerbytes == 0 {
        let mut headerbytes: i32 = 0;
        let mut i: i32 = 0;
        if bytes < 27 {
            return 0isize;
        }
        /* verify capture pattern */
        if crate::stdlib::memcmp(
            page as *const libc::c_void,
            b"OggS\x00" as *const u8 as *const libc::c_void,
            4,
        ) != 0
        {
            current_block = 8719873228262180869; /* not enough for header + seg table */
        } else {
            headerbytes = *page.offset(26) as i32 + 27;
            if bytes < headerbytes as isize {
                return 0isize;
            }
            /* count up body length in the segment table */
            i = 0;
            while i < *page.offset(26) as i32 {
                (*oy).bodybytes += *page.offset((27 + i) as isize) as i32;
                i += 1
            }
            (*oy).headerbytes = headerbytes;
            current_block = 12349973810996921269;
        }
    } else {
        current_block = 12349973810996921269;
    }
    match current_block {
        12349973810996921269 => {
            if ((*oy).bodybytes + (*oy).headerbytes) as isize > bytes {
                return 0isize;
            }
            /* The whole test page is buffered.  Verify the checksum */
            /* Grab the checksum bytes, set the header field to zero */
            let mut chksum: [i8; 4] = [0; 4];
            let mut log: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
                header: 0 as *mut u8,
                header_len: 0,
                body: 0 as *mut u8,
                body_len: 0,
            };
            crate::stdlib::memcpy(
                chksum.as_mut_ptr() as *mut libc::c_void,
                page.offset(22) as *const libc::c_void,
                4,
            );
            crate::stdlib::memset(page.offset(22) as *mut libc::c_void, 0, 4);
            /* set up a temp page struct and recompute the checksum */
            log.header = page;
            log.header_len = (*oy).headerbytes as isize;
            log.body = page.offset((*oy).headerbytes as isize);
            log.body_len = (*oy).bodybytes as isize;
            ogg_page_checksum_set(&mut log);
            /* Compare */
            if crate::stdlib::memcmp(
                chksum.as_mut_ptr() as *const libc::c_void,
                page.offset(22) as *const libc::c_void,
                4,
            ) != 0
            {
                /* D'oh.  Mismatch! Corrupt page (or miscapture and not a page
                at all) */
                /* replace the computed checksum with the one actually read in */
                crate::stdlib::memcpy(
                    page.offset(22isize) as *mut libc::c_void,
                    chksum.as_mut_ptr() as *const libc::c_void,
                    4usize,
                );
            } else {
                /* yes, have a whole page all ready to go */
                let mut page_0: *mut u8 = (*oy).data.offset((*oy).returned as isize);
                let mut bytes_0: isize = 0;
                if !og.is_null() {
                    (*og).header = page_0;
                    (*og).header_len = (*oy).headerbytes as isize;
                    (*og).body = page_0.offset((*oy).headerbytes as isize);
                    (*og).body_len = (*oy).bodybytes as isize
                }
                (*oy).unsynced = 0;
                bytes_0 = ((*oy).headerbytes + (*oy).bodybytes) as isize;
                (*oy).returned = ((*oy).returned as isize + bytes_0) as i32;
                (*oy).headerbytes = 0;
                (*oy).bodybytes = 0;
                return bytes_0;
            }
        }
        _ => {}
    }
    /* Bad checksum. Lose sync */
    (*oy).headerbytes = 0;
    (*oy).bodybytes = 0;
    /* search for possible capture */
    next = crate::stdlib::memchr(
        page.offset(1) as *const libc::c_void,
        'O' as i32,
        (bytes - 1) as usize,
    ) as *mut u8;
    if next.is_null() {
        next = (*oy).data.offset((*oy).fill as isize)
    }
    (*oy).returned = next.wrapping_offset_from((*oy).data) as i32;
    return -(next.wrapping_offset_from(page));
}
/* sync the stream and get a page.  Keep trying until we find a page.
Suppress 'sync errors' after reporting the first.

return values:
-1) recapture (hole in data)
 0) need more data
 1) page returned

Returns pointers into buffered data; invalidated by next call to
_stream, _clear, _init, or _buffer */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_pageout(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> i32 {
    if ogg_sync_check(oy) != 0 {
        return 0i32;
    }
    loop
    /* all we need to do is verify a page at the head of the stream
    buffer.  If it doesn't verify, we look for the next potential
    frame */
    {
        let mut ret: isize = ogg_sync_pageseek(oy, og);
        if ret > 0 {
            /* loop. keep looking */
            /* have a page */
            return 1i32;
        }
        if ret == 0 {
            /* need more data */
            return 0i32;
        }
        /* head did not start a synced page... skipped some bytes */
        if (*oy).unsynced == 0 {
            (*oy).unsynced = 1;
            return -(1i32);
        }
    }
}
/* add the incoming page to the stream state; we decompose the page
into packet segments here as well. */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_pagein(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> i32 {
    let mut header: *mut u8 = (*og).header;
    let mut body: *mut u8 = (*og).body;
    let mut bodysize: isize = (*og).body_len;
    let mut segptr: i32 = 0;
    let mut version: i32 = ogg_page_version(og);
    let mut continued: i32 = ogg_page_continued(og);
    let mut bos: i32 = ogg_page_bos(og);
    let mut eos: i32 = ogg_page_eos(og);
    let mut granulepos: crate::config_types_h::ogg_int64_t = ogg_page_granulepos(og);
    let mut serialno: i32 = ogg_page_serialno(og);
    let mut pageno: isize = ogg_page_pageno(og);
    let mut segments: i32 = *header.offset(26) as i32;
    if ogg_stream_check(os) != 0 {
        return -(1i32);
    }
    /* clean up 'returned data' */
    let mut lr: isize = (*os).lacing_returned;
    let mut br: isize = (*os).body_returned;
    /* body data */
    if br != 0 {
        (*os).body_fill -= br;
        if (*os).body_fill != 0 {
            crate::stdlib::memmove(
                (*os).body_data as *mut libc::c_void,
                (*os).body_data.offset(br) as *const libc::c_void,
                (*os).body_fill as usize,
            );
        }
        (*os).body_returned = 0isize
    }
    if lr != 0 {
        /* segment table */
        if (*os).lacing_fill - lr != 0 {
            crate::stdlib::memmove(
                (*os).lacing_vals as *mut libc::c_void,
                (*os).lacing_vals.offset(lr) as *const libc::c_void,
                (((*os).lacing_fill - lr) as usize).wrapping_mul(::std::mem::size_of::<i32>()),
            );
            crate::stdlib::memmove(
                (*os).granule_vals as *mut libc::c_void,
                (*os).granule_vals.offset(lr) as *const libc::c_void,
                (((*os).lacing_fill - lr) as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_int64_t>()),
            );
        }
        (*os).lacing_fill -= lr;
        (*os).lacing_packet -= lr;
        (*os).lacing_returned = 0isize
    }
    /* check the serial number */
    if serialno as isize != (*os).serialno {
        return -(1i32);
    }
    if version > 0 {
        return -(1i32);
    }
    if _os_lacing_expand(os, (segments + 1) as isize) != 0 {
        return -(1i32);
    }
    /* are we in sequence? */
    if pageno != (*os).pageno {
        let mut i: i32 = 0;
        /* unroll previous partial packet (if any) */
        i = (*os).lacing_packet as i32;
        while (i as isize) < (*os).lacing_fill {
            (*os).body_fill -= (*(*os).lacing_vals.offset(i as isize) & 0xffi32) as isize;
            i += 1
        }
        (*os).lacing_fill = (*os).lacing_packet;
        /* make a note of dropped data in segment table */
        if (*os).pageno != -1isize {
            let fresh2 = (*os).lacing_fill;
            (*os).lacing_fill = (*os).lacing_fill + 1;
            *(*os).lacing_vals.offset(fresh2) = 0x400;
            (*os).lacing_packet += 1
        }
    }
    /* are we a 'continued packet' page?  If so, we may need to skip
    some segments */
    if continued != 0 {
        if (*os).lacing_fill < 1isize
            || (*(*os).lacing_vals.offset((*os).lacing_fill - 1) & 0xff) < 255
            || *(*os).lacing_vals.offset((*os).lacing_fill - 1) == 0x400
        {
            bos = 0;
            while segptr < segments {
                let mut val: i32 = *header.offset((27 + segptr) as isize) as i32;
                body = body.offset(val as isize);
                bodysize -= val as isize;
                if val < 255 {
                    segptr += 1;
                    break;
                } else {
                    segptr += 1
                }
            }
        }
    }
    if bodysize != 0 {
        if _os_body_expand(os, bodysize) != 0 {
            return -(1i32);
        }
        crate::stdlib::memcpy(
            (*os).body_data.offset((*os).body_fill) as *mut libc::c_void,
            body as *const libc::c_void,
            bodysize as usize,
        );
        (*os).body_fill += bodysize
    }
    let mut saved: i32 = -(1);
    while segptr < segments {
        let mut val_0: i32 = *header.offset((27 + segptr) as isize) as i32;
        *(*os).lacing_vals.offset((*os).lacing_fill) = val_0;
        *(*os).granule_vals.offset((*os).lacing_fill) = -1isize;
        if bos != 0 {
            *(*os).lacing_vals.offset((*os).lacing_fill) |= 0x100;
            bos = 0
        }
        if val_0 < 255 {
            saved = (*os).lacing_fill as i32
        }
        (*os).lacing_fill += 1;
        segptr += 1;
        if val_0 < 255 {
            (*os).lacing_packet = (*os).lacing_fill
        }
    }
    /* set the granulepos on the last granuleval of the last full packet */
    if saved != -(1) {
        *(*os).granule_vals.offset(saved as isize) = granulepos
    }
    if eos != 0 {
        (*os).e_o_s = 1;
        if (*os).lacing_fill > 0isize {
            *(*os).lacing_vals.offset((*os).lacing_fill - 1) |= 0x200
        }
    }
    (*os).pageno = pageno + 1;
    return 0;
}
/* clear things to an initial state.  Good to call, eg, before seeking */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_reset(mut oy: *mut crate::ogg_h::ogg_sync_state) -> i32 {
    if ogg_sync_check(oy) != 0 {
        return -(1i32);
    }
    (*oy).fill = 0;
    (*oy).returned = 0;
    (*oy).unsynced = 0;
    (*oy).headerbytes = 0;
    (*oy).bodybytes = 0;
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_reset(mut os: *mut crate::ogg_h::ogg_stream_state) -> i32 {
    if ogg_stream_check(os) != 0 {
        return -(1i32);
    }
    (*os).body_fill = 0isize;
    (*os).body_returned = 0isize;
    (*os).lacing_fill = 0isize;
    (*os).lacing_packet = 0isize;
    (*os).lacing_returned = 0isize;
    (*os).header_fill = 0;
    (*os).e_o_s = 0;
    (*os).b_o_s = 0;
    (*os).pageno = -1isize;
    (*os).packetno = 0isize;
    (*os).granulepos = 0isize;
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_reset_serialno(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut serialno: i32,
) -> i32 {
    if ogg_stream_check(os) != 0 {
        return -(1i32);
    }
    ogg_stream_reset(os);
    (*os).serialno = serialno as isize;
    return 0;
}

unsafe extern "C" fn _packetout(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
    mut adv: i32,
) -> i32 {
    /* The last part of decode. We have the stream broken into packet
    segments.  Now we need to group them into packets (or return the
    out of sync markers) */
    let mut ptr: i32 = (*os).lacing_returned as i32;
    if (*os).lacing_packet <= ptr as isize {
        return 0i32;
    }
    if *(*os).lacing_vals.offset(ptr as isize) & 0x400 != 0 {
        /* we need to tell the codec there's a gap; it might need to
        handle previous packet dependencies. */
        (*os).lacing_returned += 1; /* just using peek as an inexpensive way
                                    to ask if there's a whole packet
                                    waiting */
        (*os).packetno += 1;
        return -(1i32);
    }
    if op.is_null() && adv == 0 {
        return 1i32;
    }
    /* Gather the whole packet. We'll have no holes or a partial packet */
    let mut size: i32 = *(*os).lacing_vals.offset(ptr as isize) & 0xff; /* last packet of the stream? */
    let mut bytes: isize = size as isize; /* first packet of the stream? */
    let mut eos: i32 = *(*os).lacing_vals.offset(ptr as isize) & 0x200;
    let mut bos: i32 = *(*os).lacing_vals.offset(ptr as isize) & 0x100;
    while size == 255 {
        ptr += 1;
        let mut val: i32 = *(*os).lacing_vals.offset(ptr as isize);
        size = val & 0xff;
        if val & 0x200 != 0 {
            eos = 0x200
        }
        bytes += size as isize
    }
    if !op.is_null() {
        (*op).e_o_s = eos as isize;
        (*op).b_o_s = bos as isize;
        (*op).packet = (*os).body_data.offset((*os).body_returned);
        (*op).packetno = (*os).packetno;
        (*op).granulepos = *(*os).granule_vals.offset(ptr as isize);
        (*op).bytes = bytes
    }
    if adv != 0 {
        (*os).body_returned += bytes;
        (*os).lacing_returned = (ptr + 1) as isize;
        (*os).packetno += 1
    }
    return 1;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_packetout(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    if ogg_stream_check(os) != 0 {
        return 0i32;
    }
    return _packetout(os, op, 1);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_packetpeek(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    if ogg_stream_check(os) != 0 {
        return 0i32;
    }
    return _packetout(os, op, 0);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_packet_clear(mut op: *mut crate::ogg_h::ogg_packet) {
    crate::stdlib::free((*op).packet as *mut libc::c_void);
    crate::stdlib::memset(
        op as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>(),
    );
}
