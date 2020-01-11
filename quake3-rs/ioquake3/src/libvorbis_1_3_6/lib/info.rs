use ::libc;

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_func_mapping;
pub use crate::backends_h::vorbis_func_residue;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_comment;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::private_state;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::codec_internal_h::vorbis_look_transform;
pub use crate::config_types_h::ogg_int32_t;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_reset;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_pack;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_unpack;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_band;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vi_psy_free;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_clear;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

pub use crate::src::libvorbis_1_3_6::lib::info::ctype_h::toupper;
use crate::src::libvorbis_1_3_6::lib::registry::_floor_P;
use crate::src::libvorbis_1_3_6::lib::registry::_mapping_P;
use crate::src::libvorbis_1_3_6::lib::registry::_residue_P;
use crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog;
pub use crate::stdlib::__ctype_toupper_loc;
use crate::stdlib::calloc;
use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::realloc;
use crate::stdlib::strcat;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
/* helpers */

unsafe extern "C" fn _v_writestring(
    mut o: *mut crate::ogg_h::oggpack_buffer,
    mut s: *const i8,
    mut bytes: i32,
) {
    loop {
        let fresh0 = bytes;
        bytes = bytes - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(o, *fresh1 as usize, 8);
    }
}

unsafe extern "C" fn _v_readstring(
    mut o: *mut crate::ogg_h::oggpack_buffer,
    mut buf: *mut i8,
    mut bytes: i32,
) {
    loop {
        let fresh2 = bytes;
        bytes = bytes - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = buf;
        buf = buf.offset(1);
        *fresh3 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(o, 8) as i8
    }
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_init(mut vc: *mut crate::codec_h::vorbis_comment) {
    crate::stdlib::memset(
        vc as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::codec_h::vorbis_comment>(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_add(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut comment: *const i8,
) {
    (*vc).user_comments = crate::stdlib::realloc(
        (*vc).user_comments as *mut libc::c_void,
        (((*vc).comments + 2i32) as usize).wrapping_mul(::std::mem::size_of::<*mut i8>()),
    ) as *mut *mut i8;
    (*vc).comment_lengths = crate::stdlib::realloc(
        (*vc).comment_lengths as *mut libc::c_void,
        (((*vc).comments + 2i32) as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    ) as *mut i32;
    *(*vc).comment_lengths.offset((*vc).comments as isize) = crate::stdlib::strlen(comment) as i32;
    let ref mut fresh4 = *(*vc).user_comments.offset((*vc).comments as isize);
    *fresh4 = crate::stdlib::malloc(
        (*(*vc).comment_lengths.offset((*vc).comments as isize) + 1i32) as usize,
    ) as *mut i8;
    crate::stdlib::strcpy(
        *(*vc).user_comments.offset((*vc).comments as isize),
        comment,
    );
    (*vc).comments += 1;
    let ref mut fresh5 = *(*vc).user_comments.offset((*vc).comments as isize);
    *fresh5 = 0 as *mut i8;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_add_tag(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut tag: *const i8,
    mut contents: *const i8,
) {
    /* Length for key and value +2 for = and \0 */
    let mut comment: *mut i8 = crate::stdlib::malloc(
        crate::stdlib::strlen(tag)
            .wrapping_add(crate::stdlib::strlen(contents))
            .wrapping_add(2usize),
    ) as *mut i8;
    crate::stdlib::strcpy(comment, tag);
    crate::stdlib::strcat(comment, b"=\x00" as *const u8 as *const i8);
    crate::stdlib::strcat(comment, contents);
    vorbis_comment_add(vc, comment);
    crate::stdlib::free(comment as *mut libc::c_void);
}
/* This is more or less the same as strncasecmp - but that doesn't exist
 * everywhere, and this is a fairly trivial function, so we include it */

unsafe extern "C" fn tagcompare(mut s1: *const i8, mut s2: *const i8, mut n: i32) -> i32 {
    let mut c: i32 = 0; /* +1 for the = we append */
    while c < n {
        if ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *s1.offset(c as isize) as i32;
                    __res = (if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    })
                } else {
                    __res = toupper(*s1.offset(c as isize) as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc())
                    .offset(*s1.offset(c as isize) as i32 as isize)
            }
            __res
        }) != ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *s2.offset(c as isize) as i32;
                    __res = (if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    })
                } else {
                    __res = toupper(*s2.offset(c as isize) as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc())
                    .offset(*s2.offset(c as isize) as i32 as isize)
            }
            __res
        }) {
            return (0i32 == 0) as i32;
        }
        c += 1
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_query(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut tag: *const i8,
    mut count: i32,
) -> *mut i8 {
    let mut i: isize = 0;
    let mut found: i32 = 0;
    let mut taglen: i32 = crate::stdlib::strlen(tag).wrapping_add(1usize) as i32;
    let mut fulltag: *mut i8 = crate::stdlib::malloc((taglen + 1) as usize) as *mut i8;
    crate::stdlib::strcpy(fulltag, tag);
    crate::stdlib::strcat(fulltag, b"=\x00" as *const u8 as *const i8);

    for i in 0..(*vc).comments as isize {
        if tagcompare(*(*vc).user_comments.offset(i), fulltag, taglen) == 0 {
            if count == found {
                /* We return a pointer to the data, not a copy */
                crate::stdlib::free(fulltag as *mut libc::c_void);
                return (*(*vc).user_comments.offset(i)).offset(taglen as isize);
            } else {
                found += 1
            }
        }
    }
    crate::stdlib::free(fulltag as *mut libc::c_void);
    return 0 as *mut i8;
    /* didn't find anything */
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_query_count(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut tag: *const i8,
) -> i32 {
    let mut i: i32 = 0; /* +1 for the = we append */
    let mut count: i32 = 0;
    let mut taglen: i32 = crate::stdlib::strlen(tag).wrapping_add(1usize) as i32;
    let mut fulltag: *mut i8 = crate::stdlib::malloc((taglen + 1) as usize) as *mut i8;
    crate::stdlib::strcpy(fulltag, tag);
    crate::stdlib::strcat(fulltag, b"=\x00" as *const u8 as *const i8);

    for i in 0..(*vc).comments {
        if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
            count += 1
        }
    }
    crate::stdlib::free(fulltag as *mut libc::c_void);
    return count;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_clear(mut vc: *mut crate::codec_h::vorbis_comment) {
    if !vc.is_null() {
        let mut i: isize = 0;
        if !(*vc).user_comments.is_null() {
            for i in 0..(*vc).comments as isize {
                if !(*(*vc).user_comments.offset(i)).is_null() {
                    crate::stdlib::free(*(*vc).user_comments.offset(i) as *mut libc::c_void);
                }
            }
            crate::stdlib::free((*vc).user_comments as *mut libc::c_void);
        }
        if !(*vc).comment_lengths.is_null() {
            crate::stdlib::free((*vc).comment_lengths as *mut libc::c_void);
        }
        if !(*vc).vendor.is_null() {
            crate::stdlib::free((*vc).vendor as *mut libc::c_void);
        }
        crate::stdlib::memset(
            vc as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::codec_h::vorbis_comment>(),
        );
    };
}
/* blocksize 0 is guaranteed to be short, 1 is guaranteed to be long.
They may be equal, but short will never ge greater than long */
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_blocksize(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut zo: i32,
) -> i32 {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    return if !ci.is_null() {
        (*ci).blocksizes[zo as usize]
    } else {
        -1
    } as i32;
}
/* libvorbis encodes in two abstraction layers; first we perform DSP
and produce a packet (see docs/analysis.txt).  The packet is then
coded into a framed OggSquish bitstream by the second layer (see
docs/framing.txt).  Decode is the reverse process; we sync/frame
the bitstream and extract individual packets, then decode the
packet back into PCM audio.

The extra framing/packetizing is used in streaming formats, such as
files.  Over the net (such as with UDP), the framing and
packetization aren't necessary as they're provided by the transport
and the streaming layer is not used */
/* Vorbis PRIMITIVES: general ***************************************/
/* used by synthesis, which has a full, alloced vi */
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_init(mut vi: *mut crate::codec_h::vorbis_info) {
    crate::stdlib::memset(
        vi as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::codec_h::vorbis_info>(),
    );
    (*vi).codec_setup = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::codec_internal_h::codec_setup_info>(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_clear(mut vi: *mut crate::codec_h::vorbis_info) {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: i32 = 0;
    if !ci.is_null() {
        i = 0;
        while i < (*ci).modes {
            if !(*ci).mode_param[i as usize].is_null() {
                crate::stdlib::free((*ci).mode_param[i as usize] as *mut libc::c_void);
            }
            i += 1
        }
        i = 0;
        while i < (*ci).maps {
            /* unpack does the range checking */
            if !(*ci).map_param[i as usize].is_null() {
                /* this may be cleaning up an aborted
                unpack, in which case the below type
                cannot be trusted */
                (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P
                    .as_ptr()
                    .offset((*ci).map_type[i as usize] as isize))
                .free_info
                .expect("non-null function pointer")((*ci).map_param[i as usize]);
            }
            i += 1
        }
        i = 0;
        while i < (*ci).floors {
            /* unpack does the range checking */
            if !(*ci).floor_param[i as usize].is_null() {
                /* this may be cleaning up an aborted
                unpack, in which case the below type
                cannot be trusted */
                (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                    .as_ptr()
                    .offset((*ci).floor_type[i as usize] as isize))
                .free_info
                .expect("non-null function pointer")((*ci).floor_param[i as usize]);
            }
            i += 1
        }
        i = 0;
        while i < (*ci).residues {
            /* unpack does the range checking */
            if !(*ci).residue_param[i as usize].is_null() {
                /* this may be cleaning up an aborted
                unpack, in which case the below type
                cannot be trusted */
                (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                    .as_ptr()
                    .offset((*ci).residue_type[i as usize] as isize))
                .free_info
                .expect("non-null function pointer")(
                    (*ci).residue_param[i as usize]
                );
            }
            i += 1
        }
        i = 0;
        while i < (*ci).books {
            if !(*ci).book_param[i as usize].is_null() {
                /* knows if the book was not alloced */
                crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy(
                    (*ci).book_param[i as usize],
                );
            }
            if !(*ci).fullbooks.is_null() {
                crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_clear(
                    (*ci).fullbooks.offset(i as isize),
                );
            }
            i += 1
        }
        if !(*ci).fullbooks.is_null() {
            crate::stdlib::free((*ci).fullbooks as *mut libc::c_void);
        }
        i = 0;
        while i < (*ci).psys {
            crate::src::libvorbis_1_3_6::lib::psy::_vi_psy_free((*ci).psy_param[i as usize]);
            i += 1
        }
        crate::stdlib::free(ci as *mut libc::c_void);
    }
    crate::stdlib::memset(
        vi as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::codec_h::vorbis_info>(),
    );
}
/* Header packing/unpacking ********************************************/

unsafe extern "C" fn _vorbis_unpack_info(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> i32 {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info; /* EOP check */
    if ci.is_null() {
        return -(129i32);
    } /* EOP check */
    (*vi).version = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32) as i32;
    if (*vi).version != 0 {
        return -(134i32);
    }
    (*vi).channels = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) as i32;
    (*vi).rate = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32);
    (*vi).bitrate_upper = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32)
        as crate::config_types_h::ogg_int32_t as isize;
    (*vi).bitrate_nominal = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32)
        as crate::config_types_h::ogg_int32_t as isize;
    (*vi).bitrate_lower = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32)
        as crate::config_types_h::ogg_int32_t as isize;
    (*ci).blocksizes[0] =
        ((1i32) << crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4)) as isize;
    (*ci).blocksizes[1] =
        ((1i32) << crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4)) as isize;
    if !((*vi).rate < 1isize) {
        if !((*vi).channels < 1) {
            if !((*ci).blocksizes[0] < 64) {
                if !((*ci).blocksizes[1] < (*ci).blocksizes[0]) {
                    if !((*ci).blocksizes[1] > 8192) {
                        if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1) != 1) {
                            return 0i32;
                        }
                    }
                }
            }
        }
    }
    vorbis_info_clear(vi);
    return -(133);
}

unsafe extern "C" fn _vorbis_unpack_comment(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut vendorlen: i32 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32) as i32;
    if !(vendorlen < 0) {
        if !(vendorlen as isize > (*opb).storage - 8) {
            (*vc).vendor = crate::stdlib::calloc((vendorlen + 1) as usize, 1) as *mut i8;
            _v_readstring(opb, (*vc).vendor, vendorlen);
            i = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32) as i32;
            if !(i < 0) {
                if !(i as isize
                    > (*opb).storage - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                        >> 2)
                {
                    (*vc).comments = i;
                    (*vc).user_comments = crate::stdlib::calloc(
                        ((*vc).comments + 1i32) as usize,
                        ::std::mem::size_of::<*mut i8>(),
                    ) as *mut *mut i8;
                    (*vc).comment_lengths = crate::stdlib::calloc(
                        ((*vc).comments + 1i32) as usize,
                        ::std::mem::size_of::<i32>(),
                    ) as *mut i32;
                    i = 0;
                    loop {
                        if !(i < (*vc).comments) {
                            current_block = 6057473163062296781;
                            break;
                        }
                        let mut len: i32 =
                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 32) as i32;
                        if len < 0 {
                            current_block = 16389255375241467587;
                            break;
                        }
                        if len as isize
                            > (*opb).storage
                                - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                        {
                            current_block = 16389255375241467587;
                            break;
                        }
                        *(*vc).comment_lengths.offset(i as isize) = len;
                        let ref mut fresh6 = *(*vc).user_comments.offset(i as isize);
                        *fresh6 = crate::stdlib::calloc((len + 1) as usize, 1) as *mut i8;
                        _v_readstring(opb, *(*vc).user_comments.offset(i as isize), len);
                        i += 1
                    }
                    match current_block {
                        16389255375241467587 => {}
                        _ => {
                            if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1) != 1)
                            {
                                return 0i32;
                            }
                        }
                    }
                }
            }
        }
    }
    vorbis_comment_clear(vc);
    return -(133);
}
/* all of the real encoding details are here.  The modes, books,
everything */

unsafe extern "C" fn _vorbis_unpack_books(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> i32 {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: i32 = 0;
    /* codebooks */
    (*ci).books = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) + 1) as i32;
    if !((*ci).books <= 0) {
        i = 0;
        loop {
            if !(i < (*ci).books) {
                current_block = 14523784380283086299;
                break;
            }
            (*ci).book_param[i as usize] =
                crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_unpack(opb);
            if (*ci).book_param[i as usize].is_null() {
                current_block = 9493312797378346061;
                break;
            }
            i += 1
        }
        match current_block {
            9493312797378346061 => {}
            _ =>
            /* time backend settings; hooks are unused */
            {
                let mut times: i32 =
                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 6) + 1) as i32;
                if !(times <= 0) {
                    i = 0;
                    loop {
                        if !(i < times) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut test: i32 =
                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 16) as i32;
                        if test < 0 || test >= 1 {
                            current_block = 9493312797378346061;
                            break;
                        }
                        i += 1
                    }
                    match current_block {
                        9493312797378346061 => {}
                        _ => {
                            /* floor backend settings */
                            (*ci).floors =
                                (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 6) + 1)
                                    as i32;
                            if !((*ci).floors <= 0) {
                                i = 0;
                                loop {
                                    if !(i < (*ci).floors) {
                                        current_block = 10652014663920648156;
                                        break;
                                    }
                                    (*ci).floor_type[i as usize] =
                                        crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                            opb, 16,
                                        ) as i32;
                                    if (*ci).floor_type[i as usize] < 0
                                        || (*ci).floor_type[i as usize] >= 2
                                    {
                                        current_block = 9493312797378346061;
                                        break;
                                    }
                                    (*ci).floor_param[i as usize] =
                                        (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                                            .as_ptr()
                                            .offset((*ci).floor_type[i as usize] as isize))
                                        .unpack
                                        .expect("non-null function pointer")(
                                            vi, opb
                                        );
                                    if (*ci).floor_param[i as usize].is_null() {
                                        current_block = 9493312797378346061;
                                        break;
                                    }
                                    i += 1
                                }
                                match current_block {
                                    9493312797378346061 => {}
                                    _ => {
                                        /* residue backend settings */
                                        (*ci).residues =
                                            (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                                opb, 6,
                                            ) + 1)
                                                as i32;
                                        if !((*ci).residues <= 0) {
                                            i = 0;
                                            loop {
                                                if !(i < (*ci).residues) {
                                                    current_block = 18377268871191777778;
                                                    break;
                                                }
                                                (*ci).residue_type[i as usize]
                                                    =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 16)
                                                        as i32;
                                                if (*ci).residue_type[i as usize] < 0
                                                    || (*ci).residue_type[i as usize] >= 3
                                                {
                                                    current_block = 9493312797378346061;
                                                    break;
                                                }
                                                (*ci).residue_param[i as
                                                                        usize]
                                                    =
                                                    (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P.as_ptr().offset((*ci).residue_type[i
                                                                                                         as
                                                                                                         usize]
                                                                                      as
                                                                                      isize)).unpack.expect("non-null function pointer")(vi,
                                                                                                                                         opb);
                                                if (*ci).residue_param[i as usize].is_null() {
                                                    current_block = 9493312797378346061;
                                                    break;
                                                }
                                                i += 1
                                            }
                                            match current_block {
                                                9493312797378346061 => {}
                                                _ => {
                                                    /* map backend settings */
                                                    (*ci).maps =
                                                        (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                      6)
                                                             +
                                                             1)
                                                            as i32;
                                                    if !((*ci).maps <= 0) {
                                                        i = 0;
                                                        loop {
                                                            if !(i < (*ci).maps) {
                                                                current_block =
                                                                    17784502470059252271;
                                                                break;
                                                            }
                                                            (*ci).map_type[i
                                                                               as
                                                                               usize]
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                             16)
                                                                    as
                                                                    i32;
                                                            if (*ci).map_type[i as usize] < 0
                                                                || (*ci).map_type[i as usize] >= 1
                                                            {
                                                                current_block = 9493312797378346061;
                                                                break;
                                                            }
                                                            (*ci).map_param[i
                                                                                as
                                                                                usize]
                                                                =
                                                                (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P.as_ptr().offset((*ci).map_type[i
                                                                                                                 as
                                                                                                                 usize]
                                                                                                  as
                                                                                                  isize)).unpack.expect("non-null function pointer")(vi,
                                                                                                                                                     opb);
                                                            if (*ci).map_param[i as usize].is_null()
                                                            {
                                                                current_block = 9493312797378346061;
                                                                break;
                                                            }
                                                            i += 1
                                                        }
                                                        match current_block {
                                                            9493312797378346061 => {}
                                                            _ => {
                                                                /* mode settings */
                                                                (*ci).modes =
                                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                                  6)
                                                                         +
                                                                         1)
                                                                        as
                                                                        i32; /* top level EOP check */
                                                                if !((*ci).modes <= 0) {
                                                                    i = 0;
                                                                    loop {
                                                                        if !(i < (*ci).modes) {
                                                                            current_block =
                                                                                9353995356876505083;
                                                                            break;
                                                                        }
                                                                        (*ci).mode_param[i
                                                                                             as
                                                                                             usize]
                                                                            =
                                                                            crate::stdlib::calloc(1,
                                                                                   
                                                                                   ::std::mem::size_of::<crate::codec_internal_h::vorbis_info_mode>())
                                                                                as
                                                                                *mut crate::codec_internal_h::vorbis_info_mode;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).blockflag
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                                         1)
                                                                                as
                                                                                i32;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).windowtype
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                                         16)
                                                                                as
                                                                                i32;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).transformtype
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                                         16)
                                                                                as
                                                                                i32;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).mapping
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                                         8)
                                                                                as
                                                                                i32;
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .windowtype
                                                                            >= 1
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .transformtype
                                                                            >= 1
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .mapping
                                                                            >= (*ci).maps
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .mapping
                                                                            < 0
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        i += 1
                                                                    }
                                                                    match current_block
                                                                        {
                                                                        9493312797378346061
                                                                        => {
                                                                        }
                                                                        _ => {
                                                                            if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                                              1)
                                                                                     !=
                                                                                     1)
                                                                               {
                                                                                return 0
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vorbis_info_clear(vi);
    return -(133);
}
/* Vorbis PRIMITIVES: synthesis layer *******************************/
/* Is this packet a vorbis ID header? */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_idheader(mut op: *mut crate::ogg_h::ogg_packet) -> i32 {
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    }; /* Not the initial packet */
    let mut buffer: [i8; 6] = [0; 6]; /* not an ID header */
    if !op.is_null() {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(
            &mut opb,
            (*op).packet,
            (*op).bytes as i32,
        ); /* not vorbis */
        if (*op).b_o_s == 0 {
            return 0i32;
        }
        if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(&mut opb, 8) != 1 {
            return 0i32;
        }
        crate::stdlib::memset(buffer.as_mut_ptr() as *mut libc::c_void, 0, 6);
        _v_readstring(&mut opb, buffer.as_mut_ptr(), 6);
        if crate::stdlib::memcmp(
            buffer.as_mut_ptr() as *const libc::c_void,
            b"vorbis\x00" as *const u8 as *const libc::c_void,
            6,
        ) != 0
        {
            return 0i32;
        }
        return 1i32;
    }
    return 0;
}
/* The Vorbis header is in three packets; the initial small packet in
the first page that identifies basic parameters, a second packet
with bitstream comments and a third packet that holds the
codebook. */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_headerin(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    if !op.is_null() {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(
            &mut opb,
            (*op).packet,
            (*op).bytes as i32,
        );
        /* Which of the three types of header is this? */
        /* Also verify header-ness, vorbis */
        let mut buffer: [i8; 6] = [0; 6];
        let mut packtype: i32 =
            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(&mut opb, 8) as i32;
        crate::stdlib::memset(buffer.as_mut_ptr() as *mut libc::c_void, 0, 6);
        _v_readstring(&mut opb, buffer.as_mut_ptr(), 6);
        if crate::stdlib::memcmp(
            buffer.as_mut_ptr() as *const libc::c_void,
            b"vorbis\x00" as *const u8 as *const libc::c_void,
            6,
        ) != 0
        {
            /* not a vorbis header */
            return -(132i32);
        }
        match packtype {
            1 => {
                /* least significant *bit* is read first */
                if (*op).b_o_s == 0 {
                    /* Not the initial packet */
                    return -(133i32);
                }
                if (*vi).rate != 0isize {
                    /* previously initialized info header */
                    return -(133i32);
                }
                return _vorbis_unpack_info(vi, &mut opb);
            }
            3 => {
                /* least significant *bit* is read first */
                if (*vi).rate == 0isize {
                    /* um... we didn't get the initial header */
                    return -(133i32);
                }
                if !(*vc).vendor.is_null() {
                    /* previously initialized comment header */
                    return -(133i32);
                }
                return _vorbis_unpack_comment(vc, &mut opb);
            }
            5 => {
                /* least significant *bit* is read first */
                if (*vi).rate == 0isize || (*vc).vendor.is_null() {
                    /* um... we didn;t get the initial header or comments yet */
                    return -(133i32);
                }
                if (*vi).codec_setup.is_null() {
                    /* improperly initialized vorbis_info */
                    return -(129i32);
                }
                if (*((*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info)).books
                    > 0
                {
                    /* previously initialized setup header */
                    return -(133i32);
                }
                return _vorbis_unpack_books(vi, &mut opb);
            }
            _ => {
                /* Not a valid vorbis header type */
                return -(133i32);
            }
        }
    }
    return -(133);
}
/* pack side **********************************************************/

unsafe extern "C" fn _vorbis_pack_info(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> i32 {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    if ci.is_null() || (*ci).blocksizes[0] < 64 || (*ci).blocksizes[1] < (*ci).blocksizes[0] {
        return -(129i32);
    }
    /* preamble */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0x1, 8);
    _v_writestring(opb, b"vorbis\x00" as *const u8 as *const i8, 6);
    /* basic information about the stream */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0, 32);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*vi).channels as usize, 8);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*vi).rate as usize, 32);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*vi).bitrate_upper as usize, 32);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*vi).bitrate_nominal as usize, 32);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*vi).bitrate_lower as usize, 32);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).blocksizes[0] - 1) as crate::config_types_h::ogg_uint32_t,
        ) as usize,
        4,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).blocksizes[1] - 1) as crate::config_types_h::ogg_uint32_t,
        ) as usize,
        4,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1);
    return 0;
}

unsafe extern "C" fn _vorbis_pack_comment(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vc: *mut crate::codec_h::vorbis_comment,
) -> i32 {
    let mut bytes: i32 = crate::stdlib::strlen(
        b"Xiph.Org libVorbis I 20180316 (Now 100% fewer shells)\x00" as *const u8 as *const i8,
    ) as i32;
    /* preamble */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0x3, 8);
    _v_writestring(opb, b"vorbis\x00" as *const u8 as *const i8, 6);
    /* vendor */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, bytes as usize, 32);
    _v_writestring(
        opb,
        b"Xiph.Org libVorbis I 20180316 (Now 100% fewer shells)\x00" as *const u8 as *const i8,
        bytes,
    );
    /* comments */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*vc).comments as usize, 32);
    if (*vc).comments != 0 {
        let mut i: i32 = 0;
        i = 0;
        while i < (*vc).comments {
            if !(*(*vc).user_comments.offset(i as isize)).is_null() {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb,
                    *(*vc).comment_lengths.offset(i as isize) as usize,
                    32,
                );
                _v_writestring(
                    opb,
                    *(*vc).user_comments.offset(i as isize),
                    *(*vc).comment_lengths.offset(i as isize),
                );
            } else {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0usize, 32i32);
            }
            i += 1
        }
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1);
    return 0;
}

unsafe extern "C" fn _vorbis_pack_books(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> i32 {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: i32 = 0;
    if ci.is_null() {
        return -(129i32);
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0x5, 8);
    _v_writestring(opb, b"vorbis\x00" as *const u8 as *const i8, 6);
    /* books */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, ((*ci).books - 1) as usize, 8);
    i = 0;
    loop {
        if !(i < (*ci).books) {
            current_block = 14523784380283086299;
            break;
        }
        if crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_pack(
            (*ci).book_param[i as usize],
            opb,
        ) != 0
        {
            current_block = 9715915742856433172;
            break;
        }
        i += 1
    }
    match current_block {
        14523784380283086299 => {
            /* times; hook placeholders */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0, 6);
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0, 16);
            /* floors */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb,
                ((*ci).floors - 1) as usize,
                6,
            );
            i = 0;
            loop {
                if !(i < (*ci).floors) {
                    current_block = 8831408221741692167;
                    break;
                }
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb,
                    (*ci).floor_type[i as usize] as usize,
                    16,
                );
                if !(**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                    .as_ptr()
                    .offset((*ci).floor_type[i as usize] as isize))
                .pack
                .is_some()
                {
                    current_block = 9715915742856433172;
                    break;
                }
                (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                    .as_ptr()
                    .offset((*ci).floor_type[i as usize] as isize))
                .pack
                .expect("non-null function pointer")(
                    (*ci).floor_param[i as usize], opb
                );
                i += 1
            }
            match current_block {
                9715915742856433172 => {}
                _ => {
                    /* residues */
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb,
                        ((*ci).residues - 1) as usize,
                        6,
                    );
                    i = 0;
                    while i < (*ci).residues {
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb,
                            (*ci).residue_type[i as usize] as usize,
                            16,
                        );
                        (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                            .as_ptr()
                            .offset((*ci).residue_type[i as usize] as isize))
                        .pack
                        .expect("non-null function pointer")(
                            (*ci).residue_param[i as usize], opb
                        );
                        i += 1
                    }
                    /* maps */
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb,
                        ((*ci).maps - 1) as usize,
                        6,
                    );
                    i = 0;
                    while i < (*ci).maps {
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb,
                            (*ci).map_type[i as usize] as usize,
                            16,
                        );
                        (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P
                            .as_ptr()
                            .offset((*ci).map_type[i as usize] as isize))
                        .pack
                        .expect("non-null function pointer")(
                            vi, (*ci).map_param[i as usize], opb
                        );
                        i += 1
                    }
                    /* modes */
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb,
                        ((*ci).modes - 1) as usize,
                        6,
                    );
                    i = 0;
                    while i < (*ci).modes {
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb,
                            (*(*ci).mode_param[i as usize]).blockflag as usize,
                            1,
                        );
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb,
                            (*(*ci).mode_param[i as usize]).windowtype as usize,
                            16,
                        );
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb,
                            (*(*ci).mode_param[i as usize]).transformtype as usize,
                            16,
                        );
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb,
                            (*(*ci).mode_param[i as usize]).mapping as usize,
                            8,
                        );
                        i += 1
                    }
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1);
                    return 0i32;
                }
            }
        }
        _ => {}
    }
    return -(1);
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_commentheader_out(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit(&mut opb);
    if _vorbis_pack_comment(&mut opb, vc) != 0 {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(&mut opb);
        return -(130i32);
    }
    (*op).packet = crate::stdlib::malloc(crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
        &mut opb,
    ) as usize) as *mut u8;
    crate::stdlib::memcpy(
        (*op).packet as *mut libc::c_void,
        opb.buffer as *const libc::c_void,
        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
    );
    (*op).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb);
    (*op).b_o_s = 0isize;
    (*op).e_o_s = 0isize;
    (*op).granulepos = 0isize;
    (*op).packetno = 1isize;
    crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(&mut opb);
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_headerout(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut op: *mut crate::ogg_h::ogg_packet,
    mut op_comm: *mut crate::ogg_h::ogg_packet,
    mut op_code: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut ret: i32 = -(130);
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    let mut b: *mut crate::codec_internal_h::private_state =
        (*v).backend_state as *mut crate::codec_internal_h::private_state;
    if b.is_null() || (*vi).channels <= 0 || (*vi).channels > 256 {
        b = 0 as *mut crate::codec_internal_h::private_state;
        ret = -(129)
    } else {
        /* first header packet **********************************************/
        crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit(&mut opb);
        if !(_vorbis_pack_info(&mut opb, vi) != 0) {
            /* build the packet */
            if !(*b).header.is_null() {
                crate::stdlib::free((*b).header as *mut libc::c_void);
            }
            (*b).header = crate::stdlib::malloc(
                crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
            ) as *mut u8;
            crate::stdlib::memcpy(
                (*b).header as *mut libc::c_void,
                opb.buffer as *const libc::c_void,
                crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
            );
            (*op).packet = (*b).header;
            (*op).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb);
            (*op).b_o_s = 1isize;
            (*op).e_o_s = 0isize;
            (*op).granulepos = 0isize;
            (*op).packetno = 0isize;
            /* second header packet (comments) **********************************/
            crate::src::libogg_1_3_3::src::bitwise::oggpack_reset(&mut opb);
            if !(_vorbis_pack_comment(&mut opb, vc) != 0) {
                if !(*b).header1.is_null() {
                    crate::stdlib::free((*b).header1 as *mut libc::c_void);
                }
                (*b).header1 = crate::stdlib::malloc(
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
                ) as *mut u8;
                crate::stdlib::memcpy(
                    (*b).header1 as *mut libc::c_void,
                    opb.buffer as *const libc::c_void,
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
                );
                (*op_comm).packet = (*b).header1;
                (*op_comm).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb);
                (*op_comm).b_o_s = 0isize;
                (*op_comm).e_o_s = 0isize;
                (*op_comm).granulepos = 0isize;
                (*op_comm).packetno = 1isize;
                /* third header packet (modes/codebooks) ****************************/
                crate::src::libogg_1_3_3::src::bitwise::oggpack_reset(&mut opb);
                if !(_vorbis_pack_books(&mut opb, vi) != 0) {
                    if !(*b).header2.is_null() {
                        crate::stdlib::free((*b).header2 as *mut libc::c_void);
                    }
                    (*b).header2 = crate::stdlib::malloc(
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
                    ) as *mut u8;
                    crate::stdlib::memcpy(
                        (*b).header2 as *mut libc::c_void,
                        opb.buffer as *const libc::c_void,
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb) as usize,
                    );
                    (*op_code).packet = (*b).header2;
                    (*op_code).bytes =
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(&mut opb);
                    (*op_code).b_o_s = 0isize;
                    (*op_code).e_o_s = 0isize;
                    (*op_code).granulepos = 0isize;
                    (*op_code).packetno = 2isize;
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(&mut opb);
                    return 0i32;
                }
            }
        }
    }
    crate::stdlib::memset(
        op as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>(),
    );
    crate::stdlib::memset(
        op_comm as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>(),
    );
    crate::stdlib::memset(
        op_code as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>(),
    );
    if !b.is_null() {
        if (*vi).channels > 0 {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(&mut opb);
        }
        if !(*b).header.is_null() {
            crate::stdlib::free((*b).header as *mut libc::c_void);
        }
        if !(*b).header1.is_null() {
            crate::stdlib::free((*b).header1 as *mut libc::c_void);
        }
        if !(*b).header2.is_null() {
            crate::stdlib::free((*b).header2 as *mut libc::c_void);
        }
        (*b).header = 0 as *mut u8;
        (*b).header1 = 0 as *mut u8;
        (*b).header2 = 0 as *mut u8
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_granule_time(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut granulepos: crate::config_types_h::ogg_int64_t,
) -> f64 {
    if granulepos == -1isize {
        return -1f64;
    }
    /* We're not guaranteed a 64 bit unsigned type everywhere, so we
    have to put the unsigned granpo in a signed type. */
    if granulepos >= 0isize {
        return granulepos as f64 / (*(*v).vi).rate as f64;
    } else {
        let mut granuleoff: crate::config_types_h::ogg_int64_t =
            0xffffffffu32 as crate::config_types_h::ogg_int64_t;
        granuleoff <<= 31isize;
        granuleoff |= 0x7ffffffff as isize;
        return (granulepos as f64 + 2f64 + granuleoff as f64 + granuleoff as f64)
            / (*(*v).vi).rate as f64;
    };
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_version_string() -> *const i8 {
    return b"Xiph.Org libVorbis 1.3.6\x00" as *const u8 as *const i8;
}
