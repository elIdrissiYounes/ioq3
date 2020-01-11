use ::libc;

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
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
use crate::stdlib::malloc;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::realloc;
use crate::stdlib::strlen;
use ::libc::free;
use ::libc::strcat;
use ::libc::strcpy;
/* helpers */

unsafe extern "C" fn _v_writestring(
    mut o: *mut crate::ogg_h::oggpack_buffer,
    mut s: *const libc::c_char,
    mut bytes: libc::c_int,
) {
    loop {
        let fresh0 = bytes;
        bytes = bytes - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            o as *mut crate::ogg_h::oggpack_buffer,
            *fresh1 as libc::c_ulong,
            8 as libc::c_int,
        );
    }
}

unsafe extern "C" fn _v_readstring(
    mut o: *mut crate::ogg_h::oggpack_buffer,
    mut buf: *mut libc::c_char,
    mut bytes: libc::c_int,
) {
    loop {
        let fresh2 = bytes;
        bytes = bytes - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = buf;
        buf = buf.offset(1);
        *fresh3 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            o as *mut crate::ogg_h::oggpack_buffer,
            8 as libc::c_int,
        ) as libc::c_char
    }
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_init(mut vc: *mut crate::codec_h::vorbis_comment) {
    crate::stdlib::memset(
        vc as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::codec_h::vorbis_comment>() as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_add(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut comment: *const libc::c_char,
) {
    (*vc).user_comments = crate::stdlib::realloc(
        (*vc).user_comments as *mut libc::c_void,
        (((*vc).comments + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    (*vc).comment_lengths = crate::stdlib::realloc(
        (*vc).comment_lengths as *mut libc::c_void,
        (((*vc).comments + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *(*vc).comment_lengths.offset((*vc).comments as isize) =
        crate::stdlib::strlen(comment) as libc::c_int;
    let ref mut fresh4 = *(*vc).user_comments.offset((*vc).comments as isize);
    *fresh4 = crate::stdlib::malloc(
        (*(*vc).comment_lengths.offset((*vc).comments as isize) + 1 as libc::c_int)
            as libc::c_ulong,
    ) as *mut libc::c_char;
    ::libc::strcpy(
        *(*vc).user_comments.offset((*vc).comments as isize),
        comment,
    );
    (*vc).comments += 1;
    let ref mut fresh5 = *(*vc).user_comments.offset((*vc).comments as isize);
    *fresh5 = 0 as *mut libc::c_char;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_add_tag(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut tag: *const libc::c_char,
    mut contents: *const libc::c_char,
) {
    /* Length for key and value +2 for = and \0 */
    let mut comment: *mut libc::c_char = crate::stdlib::malloc(
        crate::stdlib::strlen(tag)
            .wrapping_add(crate::stdlib::strlen(contents))
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    ::libc::strcpy(comment, tag);
    ::libc::strcat(comment, b"=\x00" as *const u8 as *const libc::c_char);
    ::libc::strcat(comment, contents);
    vorbis_comment_add(vc, comment);
    ::libc::free(comment as *mut libc::c_void);
}
/* This is more or less the same as strncasecmp - but that doesn't exist
 * everywhere, and this is a fairly trivial function, so we include it */

unsafe extern "C" fn tagcompare(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0 as libc::c_int; /* +1 for the = we append */
    while c < n {
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s1.offset(c as isize) as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    })
                } else {
                    __res = toupper(*s1.offset(c as isize) as libc::c_int)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc())
                    .offset(*s1.offset(c as isize) as libc::c_int as isize)
            }
            __res
        }) != ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s2.offset(c as isize) as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    })
                } else {
                    __res = toupper(*s2.offset(c as isize) as libc::c_int)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc())
                    .offset(*s2.offset(c as isize) as libc::c_int as isize)
            }
            __res
        }) {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        c += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_query(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut tag: *const libc::c_char,
    mut count: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_long = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut taglen: libc::c_int =
        crate::stdlib::strlen(tag).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut fulltag: *mut libc::c_char =
        crate::stdlib::malloc((taglen + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    ::libc::strcpy(fulltag, tag);
    ::libc::strcat(fulltag, b"=\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_long;
    while i < (*vc).comments as libc::c_long {
        if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
            if count == found {
                /* We return a pointer to the data, not a copy */
                ::libc::free(fulltag as *mut libc::c_void);
                return (*(*vc).user_comments.offset(i as isize)).offset(taglen as isize);
            } else {
                found += 1
            }
        }
        i += 1
    }
    ::libc::free(fulltag as *mut libc::c_void);
    return 0 as *mut libc::c_char;
    /* didn't find anything */
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_query_count(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut tag: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* +1 for the = we append */
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut taglen: libc::c_int =
        crate::stdlib::strlen(tag).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut fulltag: *mut libc::c_char =
        crate::stdlib::malloc((taglen + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    ::libc::strcpy(fulltag, tag);
    ::libc::strcat(fulltag, b"=\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*vc).comments {
        if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
            count += 1
        }
        i += 1
    }
    ::libc::free(fulltag as *mut libc::c_void);
    return count;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_clear(mut vc: *mut crate::codec_h::vorbis_comment) {
    if !vc.is_null() {
        let mut i: libc::c_long = 0;
        if !(*vc).user_comments.is_null() {
            i = 0 as libc::c_int as libc::c_long;
            while i < (*vc).comments as libc::c_long {
                if !(*(*vc).user_comments.offset(i as isize)).is_null() {
                    ::libc::free(*(*vc).user_comments.offset(i as isize) as *mut libc::c_void);
                }
                i += 1
            }
            ::libc::free((*vc).user_comments as *mut libc::c_void);
        }
        if !(*vc).comment_lengths.is_null() {
            ::libc::free((*vc).comment_lengths as *mut libc::c_void);
        }
        if !(*vc).vendor.is_null() {
            ::libc::free((*vc).vendor as *mut libc::c_void);
        }
        crate::stdlib::memset(
            vc as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::codec_h::vorbis_comment>() as libc::c_ulong,
        );
    };
}
/* blocksize 0 is guaranteed to be short, 1 is guaranteed to be long.
They may be equal, but short will never ge greater than long */
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_blocksize(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut zo: libc::c_int,
) -> libc::c_int {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    return if !ci.is_null() {
        (*ci).blocksizes[zo as usize]
    } else {
        -(1 as libc::c_int) as libc::c_long
    } as libc::c_int;
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
        0 as libc::c_int,
        ::std::mem::size_of::<crate::codec_h::vorbis_info>() as libc::c_ulong,
    );
    (*vi).codec_setup = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::codec_internal_h::codec_setup_info>() as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_clear(mut vi: *mut crate::codec_h::vorbis_info) {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: libc::c_int = 0;
    if !ci.is_null() {
        i = 0 as libc::c_int;
        while i < (*ci).modes {
            if !(*ci).mode_param[i as usize].is_null() {
                ::libc::free((*ci).mode_param[i as usize] as *mut libc::c_void);
            }
            i += 1
        }
        i = 0 as libc::c_int;
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
        i = 0 as libc::c_int;
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
        i = 0 as libc::c_int;
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
        i = 0 as libc::c_int;
        while i < (*ci).books {
            if !(*ci).book_param[i as usize].is_null() {
                /* knows if the book was not alloced */
                crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy(
                    (*ci).book_param[i as usize]
                        as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
                );
            }
            if !(*ci).fullbooks.is_null() {
                crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_clear(
                    (*ci).fullbooks.offset(i as isize)
                        as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                );
            }
            i += 1
        }
        if !(*ci).fullbooks.is_null() {
            ::libc::free((*ci).fullbooks as *mut libc::c_void);
        }
        i = 0 as libc::c_int;
        while i < (*ci).psys {
            crate::src::libvorbis_1_3_6::lib::psy::_vi_psy_free(
                (*ci).psy_param[i as usize]
                    as *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy,
            );
            i += 1
        }
        ::libc::free(ci as *mut libc::c_void);
    }
    crate::stdlib::memset(
        vi as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::codec_h::vorbis_info>() as libc::c_ulong,
    );
}
/* Header packing/unpacking ********************************************/

unsafe extern "C" fn _vorbis_unpack_info(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info; /* EOP check */
    if ci.is_null() {
        return -(129 as libc::c_int);
    } /* EOP check */
    (*vi).version = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        32 as libc::c_int,
    ) as libc::c_int;
    if (*vi).version != 0 as libc::c_int {
        return -(134 as libc::c_int);
    }
    (*vi).channels = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        8 as libc::c_int,
    ) as libc::c_int;
    (*vi).rate = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        32 as libc::c_int,
    );
    (*vi).bitrate_upper = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        32 as libc::c_int,
    ) as crate::config_types_h::ogg_int32_t as libc::c_long;
    (*vi).bitrate_nominal = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        32 as libc::c_int,
    ) as crate::config_types_h::ogg_int32_t as libc::c_long;
    (*vi).bitrate_lower = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        32 as libc::c_int,
    ) as crate::config_types_h::ogg_int32_t as libc::c_long;
    (*ci).blocksizes[0 as libc::c_int as usize] = ((1 as libc::c_int)
        << crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            opb as *mut crate::ogg_h::oggpack_buffer,
            4 as libc::c_int,
        )) as libc::c_long;
    (*ci).blocksizes[1 as libc::c_int as usize] = ((1 as libc::c_int)
        << crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            opb as *mut crate::ogg_h::oggpack_buffer,
            4 as libc::c_int,
        )) as libc::c_long;
    if !((*vi).rate < 1 as libc::c_int as libc::c_long) {
        if !((*vi).channels < 1 as libc::c_int) {
            if !((*ci).blocksizes[0 as libc::c_int as usize] < 64 as libc::c_int as libc::c_long) {
                if !((*ci).blocksizes[1 as libc::c_int as usize]
                    < (*ci).blocksizes[0 as libc::c_int as usize])
                {
                    if !((*ci).blocksizes[1 as libc::c_int as usize]
                        > 8192 as libc::c_int as libc::c_long)
                    {
                        if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            1 as libc::c_int,
                        ) != 1 as libc::c_int as libc::c_long)
                        {
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    vorbis_info_clear(vi);
    return -(133 as libc::c_int);
}

unsafe extern "C" fn _vorbis_unpack_comment(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut vendorlen: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        32 as libc::c_int,
    ) as libc::c_int;
    if !(vendorlen < 0 as libc::c_int) {
        if !(vendorlen as libc::c_long > (*opb).storage - 8 as libc::c_int as libc::c_long) {
            (*vc).vendor = crate::stdlib::calloc(
                (vendorlen + 1 as libc::c_int) as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
            ) as *mut libc::c_char;
            _v_readstring(opb, (*vc).vendor, vendorlen);
            i = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                opb as *mut crate::ogg_h::oggpack_buffer,
                32 as libc::c_int,
            ) as libc::c_int;
            if !(i < 0 as libc::c_int) {
                if !(i as libc::c_long
                    > (*opb).storage
                        - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                        )
                        >> 2 as libc::c_int)
                {
                    (*vc).comments = i;
                    (*vc).user_comments = crate::stdlib::calloc(
                        ((*vc).comments + 1 as libc::c_int) as libc::c_ulong,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ) as *mut *mut libc::c_char;
                    (*vc).comment_lengths = crate::stdlib::calloc(
                        ((*vc).comments + 1 as libc::c_int) as libc::c_ulong,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) as *mut libc::c_int;
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < (*vc).comments) {
                            current_block = 6057473163062296781;
                            break;
                        }
                        let mut len: libc::c_int =
                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                opb as *mut crate::ogg_h::oggpack_buffer,
                                32 as libc::c_int,
                            ) as libc::c_int;
                        if len < 0 as libc::c_int {
                            current_block = 16389255375241467587;
                            break;
                        }
                        if len as libc::c_long
                            > (*opb).storage
                                - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                                    opb as *mut crate::ogg_h::oggpack_buffer,
                                )
                        {
                            current_block = 16389255375241467587;
                            break;
                        }
                        *(*vc).comment_lengths.offset(i as isize) = len;
                        let ref mut fresh6 = *(*vc).user_comments.offset(i as isize);
                        *fresh6 = crate::stdlib::calloc(
                            (len + 1 as libc::c_int) as libc::c_ulong,
                            1 as libc::c_int as libc::c_ulong,
                        ) as *mut libc::c_char;
                        _v_readstring(opb, *(*vc).user_comments.offset(i as isize), len);
                        i += 1
                    }
                    match current_block {
                        16389255375241467587 => {}
                        _ => {
                            if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                opb as *mut crate::ogg_h::oggpack_buffer,
                                1 as libc::c_int,
                            ) != 1 as libc::c_int as libc::c_long)
                            {
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    vorbis_comment_clear(vc);
    return -(133 as libc::c_int);
}
/* all of the real encoding details are here.  The modes, books,
everything */

unsafe extern "C" fn _vorbis_unpack_books(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: libc::c_int = 0;
    /* codebooks */
    (*ci).books = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        8 as libc::c_int,
    ) + 1 as libc::c_int as libc::c_long) as libc::c_int;
    if !((*ci).books <= 0 as libc::c_int) {
        i = 0 as libc::c_int;
        loop {
            if !(i < (*ci).books) {
                current_block = 14523784380283086299;
                break;
            }
            (*ci).book_param[i as usize] =
                crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_unpack(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                )
                    as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
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
                let mut times: libc::c_int = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    6 as libc::c_int,
                ) + 1 as libc::c_int as libc::c_long)
                    as libc::c_int;
                if !(times <= 0 as libc::c_int) {
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < times) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut test: libc::c_int =
                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                opb as *mut crate::ogg_h::oggpack_buffer,
                                16 as libc::c_int,
                            ) as libc::c_int;
                        if test < 0 as libc::c_int || test >= 1 as libc::c_int {
                            current_block = 9493312797378346061;
                            break;
                        }
                        i += 1
                    }
                    match current_block {
                        9493312797378346061 => {}
                        _ => {
                            /* floor backend settings */
                            (*ci).floors = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                opb as *mut crate::ogg_h::oggpack_buffer,
                                6 as libc::c_int,
                            ) + 1 as libc::c_int as libc::c_long)
                                as libc::c_int;
                            if !((*ci).floors <= 0 as libc::c_int) {
                                i = 0 as libc::c_int;
                                loop {
                                    if !(i < (*ci).floors) {
                                        current_block = 10652014663920648156;
                                        break;
                                    }
                                    (*ci).floor_type[i as usize] =
                                        crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                            opb as *mut crate::ogg_h::oggpack_buffer,
                                            16 as libc::c_int,
                                        ) as libc::c_int;
                                    if (*ci).floor_type[i as usize] < 0 as libc::c_int
                                        || (*ci).floor_type[i as usize] >= 2 as libc::c_int
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
                                                opb as *mut crate::ogg_h::oggpack_buffer,
                                                6 as libc::c_int,
                                            ) + 1 as libc::c_int as libc::c_long)
                                                as libc::c_int;
                                        if !((*ci).residues <= 0 as libc::c_int) {
                                            i = 0 as libc::c_int;
                                            loop {
                                                if !(i < (*ci).residues) {
                                                    current_block = 18377268871191777778;
                                                    break;
                                                }
                                                (*ci).residue_type[i as usize]
                                                    =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                 16 as
                                                                     libc::c_int)
                                                        as libc::c_int;
                                                if (*ci).residue_type[i as usize] < 0 as libc::c_int
                                                    || (*ci).residue_type[i as usize]
                                                        >= 3 as libc::c_int
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
                                                        (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                      6 as
                                                                          libc::c_int)
                                                             +
                                                             1 as libc::c_int
                                                                 as
                                                                 libc::c_long)
                                                            as libc::c_int;
                                                    if !((*ci).maps <= 0 as libc::c_int) {
                                                        i = 0 as libc::c_int;
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
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                             16
                                                                                 as
                                                                                 libc::c_int)
                                                                    as
                                                                    libc::c_int;
                                                            if (*ci).map_type[i as usize]
                                                                < 0 as libc::c_int
                                                                || (*ci).map_type[i as usize]
                                                                    >= 1 as libc::c_int
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
                                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                                  6
                                                                                      as
                                                                                      libc::c_int)
                                                                         +
                                                                         1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_long)
                                                                        as
                                                                        libc::c_int; /* top level EOP check */
                                                                if !((*ci).modes
                                                                    <= 0 as libc::c_int)
                                                                {
                                                                    i = 0 as libc::c_int;
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
                                                                            crate::stdlib::calloc(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong,
                                                                                   ::std::mem::size_of::<crate::codec_internal_h::vorbis_info_mode>()
                                                                                       as
                                                                                       libc::c_ulong)
                                                                                as
                                                                                *mut crate::codec_internal_h::vorbis_info_mode;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).blockflag
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                                         1
                                                                                             as
                                                                                             libc::c_int)
                                                                                as
                                                                                libc::c_int;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).windowtype
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                                         16
                                                                                             as
                                                                                             libc::c_int)
                                                                                as
                                                                                libc::c_int;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).transformtype
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                                         16
                                                                                             as
                                                                                             libc::c_int)
                                                                                as
                                                                                libc::c_int;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).mapping
                                                                            =
                                                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                                         8
                                                                                             as
                                                                                             libc::c_int)
                                                                                as
                                                                                libc::c_int;
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .windowtype
                                                                            >= 1 as libc::c_int
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .transformtype
                                                                            >= 1 as libc::c_int
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
                                                                            < 0 as libc::c_int
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
                                                                            if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb as *mut crate::ogg_h::oggpack_buffer,
                                                                                              1
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                     !=
                                                                                     1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_long)
                                                                               {
                                                                                return 0
                                                                                           as
                                                                                           libc::c_int
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
    return -(133 as libc::c_int);
}
/* Vorbis PRIMITIVES: synthesis layer *******************************/
/* Is this packet a vorbis ID header? */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_idheader(
    mut op: *mut crate::ogg_h::ogg_packet,
) -> libc::c_int {
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut libc::c_uchar,
        ptr: 0 as *mut libc::c_uchar,
        storage: 0,
    }; /* Not the initial packet */
    let mut buffer: [libc::c_char; 6] = [0; 6]; /* not an ID header */
    if !op.is_null() {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            (*op).packet,
            (*op).bytes as libc::c_int,
        ); /* not vorbis */
        if (*op).b_o_s == 0 {
            return 0 as libc::c_int;
        }
        if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            8 as libc::c_int,
        ) != 1 as libc::c_int as libc::c_long
        {
            return 0 as libc::c_int;
        }
        crate::stdlib::memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            6 as libc::c_int as libc::c_ulong,
        );
        _v_readstring(&mut opb, buffer.as_mut_ptr(), 6 as libc::c_int);
        if crate::stdlib::memcmp(
            buffer.as_mut_ptr() as *const libc::c_void,
            b"vorbis\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
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
) -> libc::c_int {
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut libc::c_uchar,
        ptr: 0 as *mut libc::c_uchar,
        storage: 0,
    };
    if !op.is_null() {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            (*op).packet,
            (*op).bytes as libc::c_int,
        );
        /* Which of the three types of header is this? */
        /* Also verify header-ness, vorbis */
        let mut buffer: [libc::c_char; 6] = [0; 6];
        let mut packtype: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            8 as libc::c_int,
        ) as libc::c_int;
        crate::stdlib::memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            6 as libc::c_int as libc::c_ulong,
        );
        _v_readstring(&mut opb, buffer.as_mut_ptr(), 6 as libc::c_int);
        if crate::stdlib::memcmp(
            buffer.as_mut_ptr() as *const libc::c_void,
            b"vorbis\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            /* not a vorbis header */
            return -(132 as libc::c_int);
        }
        match packtype {
            1 => {
                /* least significant *bit* is read first */
                if (*op).b_o_s == 0 {
                    /* Not the initial packet */
                    return -(133 as libc::c_int);
                }
                if (*vi).rate != 0 as libc::c_int as libc::c_long {
                    /* previously initialized info header */
                    return -(133 as libc::c_int);
                }
                return _vorbis_unpack_info(vi, &mut opb);
            }
            3 => {
                /* least significant *bit* is read first */
                if (*vi).rate == 0 as libc::c_int as libc::c_long {
                    /* um... we didn't get the initial header */
                    return -(133 as libc::c_int);
                }
                if !(*vc).vendor.is_null() {
                    /* previously initialized comment header */
                    return -(133 as libc::c_int);
                }
                return _vorbis_unpack_comment(vc, &mut opb);
            }
            5 => {
                /* least significant *bit* is read first */
                if (*vi).rate == 0 as libc::c_int as libc::c_long || (*vc).vendor.is_null() {
                    /* um... we didn;t get the initial header or comments yet */
                    return -(133 as libc::c_int);
                }
                if (*vi).codec_setup.is_null() {
                    /* improperly initialized vorbis_info */
                    return -(129 as libc::c_int);
                }
                if (*((*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info)).books
                    > 0 as libc::c_int
                {
                    /* previously initialized setup header */
                    return -(133 as libc::c_int);
                }
                return _vorbis_unpack_books(vi, &mut opb);
            }
            _ => {
                /* Not a valid vorbis header type */
                return -(133 as libc::c_int);
            }
        }
    }
    return -(133 as libc::c_int);
}
/* pack side **********************************************************/

unsafe extern "C" fn _vorbis_pack_info(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> libc::c_int {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    if ci.is_null()
        || (*ci).blocksizes[0 as libc::c_int as usize] < 64 as libc::c_int as libc::c_long
        || (*ci).blocksizes[1 as libc::c_int as usize] < (*ci).blocksizes[0 as libc::c_int as usize]
    {
        return -(129 as libc::c_int);
    }
    /* preamble */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        0x1 as libc::c_int as libc::c_ulong,
        8 as libc::c_int,
    );
    _v_writestring(
        opb,
        b"vorbis\x00" as *const u8 as *const libc::c_char,
        6 as libc::c_int,
    );
    /* basic information about the stream */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        0 as libc::c_int as libc::c_ulong,
        32 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*vi).channels as libc::c_ulong,
        8 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*vi).rate as libc::c_ulong,
        32 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*vi).bitrate_upper as libc::c_ulong,
        32 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*vi).bitrate_nominal as libc::c_ulong,
        32 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*vi).bitrate_lower as libc::c_ulong,
        32 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).blocksizes[0 as libc::c_int as usize] - 1 as libc::c_int as libc::c_long)
                as crate::config_types_h::ogg_uint32_t,
        ) as libc::c_ulong,
        4 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).blocksizes[1 as libc::c_int as usize] - 1 as libc::c_int as libc::c_long)
                as crate::config_types_h::ogg_uint32_t,
        ) as libc::c_ulong,
        4 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int,
    );
    return 0 as libc::c_int;
}

unsafe extern "C" fn _vorbis_pack_comment(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vc: *mut crate::codec_h::vorbis_comment,
) -> libc::c_int {
    let mut bytes: libc::c_int = crate::stdlib::strlen(
        b"Xiph.Org libVorbis I 20180316 (Now 100% fewer shells)\x00" as *const u8
            as *const libc::c_char,
    ) as libc::c_int;
    /* preamble */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        0x3 as libc::c_int as libc::c_ulong,
        8 as libc::c_int,
    );
    _v_writestring(
        opb,
        b"vorbis\x00" as *const u8 as *const libc::c_char,
        6 as libc::c_int,
    );
    /* vendor */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        bytes as libc::c_ulong,
        32 as libc::c_int,
    );
    _v_writestring(
        opb,
        b"Xiph.Org libVorbis I 20180316 (Now 100% fewer shells)\x00" as *const u8
            as *const libc::c_char,
        bytes,
    );
    /* comments */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*vc).comments as libc::c_ulong,
        32 as libc::c_int,
    );
    if (*vc).comments != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*vc).comments {
            if !(*(*vc).user_comments.offset(i as isize)).is_null() {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    *(*vc).comment_lengths.offset(i as isize) as libc::c_ulong,
                    32 as libc::c_int,
                );
                _v_writestring(
                    opb,
                    *(*vc).user_comments.offset(i as isize),
                    *(*vc).comment_lengths.offset(i as isize),
                );
            } else {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    0 as libc::c_int as libc::c_ulong,
                    32 as libc::c_int,
                );
            }
            i += 1
        }
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int,
    );
    return 0 as libc::c_int;
}

unsafe extern "C" fn _vorbis_pack_books(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: libc::c_int = 0;
    if ci.is_null() {
        return -(129 as libc::c_int);
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        0x5 as libc::c_int as libc::c_ulong,
        8 as libc::c_int,
    );
    _v_writestring(
        opb,
        b"vorbis\x00" as *const u8 as *const libc::c_char,
        6 as libc::c_int,
    );
    /* books */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        ((*ci).books - 1 as libc::c_int) as libc::c_ulong,
        8 as libc::c_int,
    );
    i = 0 as libc::c_int;
    loop {
        if !(i < (*ci).books) {
            current_block = 14523784380283086299;
            break;
        }
        if crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_pack(
            (*ci).book_param[i as usize]
                as *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
            opb as *mut crate::ogg_h::oggpack_buffer,
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
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                0 as libc::c_int as libc::c_ulong,
                6 as libc::c_int,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                0 as libc::c_int as libc::c_ulong,
                16 as libc::c_int,
            );
            /* floors */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                ((*ci).floors - 1 as libc::c_int) as libc::c_ulong,
                6 as libc::c_int,
            );
            i = 0 as libc::c_int;
            loop {
                if !(i < (*ci).floors) {
                    current_block = 8831408221741692167;
                    break;
                }
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    (*ci).floor_type[i as usize] as libc::c_ulong,
                    16 as libc::c_int,
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
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        ((*ci).residues - 1 as libc::c_int) as libc::c_ulong,
                        6 as libc::c_int,
                    );
                    i = 0 as libc::c_int;
                    while i < (*ci).residues {
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            (*ci).residue_type[i as usize] as libc::c_ulong,
                            16 as libc::c_int,
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
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        ((*ci).maps - 1 as libc::c_int) as libc::c_ulong,
                        6 as libc::c_int,
                    );
                    i = 0 as libc::c_int;
                    while i < (*ci).maps {
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            (*ci).map_type[i as usize] as libc::c_ulong,
                            16 as libc::c_int,
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
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        ((*ci).modes - 1 as libc::c_int) as libc::c_ulong,
                        6 as libc::c_int,
                    );
                    i = 0 as libc::c_int;
                    while i < (*ci).modes {
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).blockflag as libc::c_ulong,
                            1 as libc::c_int,
                        );
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).windowtype as libc::c_ulong,
                            16 as libc::c_int,
                        );
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).transformtype as libc::c_ulong,
                            16 as libc::c_int,
                        );
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).mapping as libc::c_ulong,
                            8 as libc::c_int,
                        );
                        i += 1
                    }
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        1 as libc::c_int as libc::c_ulong,
                        1 as libc::c_int,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_commentheader_out(
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> libc::c_int {
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut libc::c_uchar,
        ptr: 0 as *mut libc::c_uchar,
        storage: 0,
    };
    crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit(
        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
    );
    if _vorbis_pack_comment(&mut opb, vc) != 0 {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        );
        return -(130 as libc::c_int);
    }
    (*op).packet = crate::stdlib::malloc(crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
    ) as libc::c_ulong) as *mut libc::c_uchar;
    crate::stdlib::memcpy(
        (*op).packet as *mut libc::c_void,
        opb.buffer as *const libc::c_void,
        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        ) as libc::c_ulong,
    );
    (*op).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
    );
    (*op).b_o_s = 0 as libc::c_int as libc::c_long;
    (*op).e_o_s = 0 as libc::c_int as libc::c_long;
    (*op).granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    (*op).packetno = 1 as libc::c_int as crate::config_types_h::ogg_int64_t;
    crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(
        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
    );
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_headerout(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut op: *mut crate::ogg_h::ogg_packet,
    mut op_comm: *mut crate::ogg_h::ogg_packet,
    mut op_code: *mut crate::ogg_h::ogg_packet,
) -> libc::c_int {
    let mut ret: libc::c_int = -(130 as libc::c_int);
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut libc::c_uchar,
        ptr: 0 as *mut libc::c_uchar,
        storage: 0,
    };
    let mut b: *mut crate::codec_internal_h::private_state =
        (*v).backend_state as *mut crate::codec_internal_h::private_state;
    if b.is_null() || (*vi).channels <= 0 as libc::c_int || (*vi).channels > 256 as libc::c_int {
        b = 0 as *mut crate::codec_internal_h::private_state;
        ret = -(129 as libc::c_int)
    } else {
        /* first header packet **********************************************/
        crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit(
            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        );
        if !(_vorbis_pack_info(&mut opb, vi) != 0) {
            /* build the packet */
            if !(*b).header.is_null() {
                ::libc::free((*b).header as *mut libc::c_void);
            }
            (*b).header =
                crate::stdlib::malloc(crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                ) as libc::c_ulong) as *mut libc::c_uchar;
            crate::stdlib::memcpy(
                (*b).header as *mut libc::c_void,
                opb.buffer as *const libc::c_void,
                crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                ) as libc::c_ulong,
            );
            (*op).packet = (*b).header;
            (*op).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            );
            (*op).b_o_s = 1 as libc::c_int as libc::c_long;
            (*op).e_o_s = 0 as libc::c_int as libc::c_long;
            (*op).granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
            (*op).packetno = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
            /* second header packet (comments) **********************************/
            crate::src::libogg_1_3_3::src::bitwise::oggpack_reset(
                &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            );
            if !(_vorbis_pack_comment(&mut opb, vc) != 0) {
                if !(*b).header1.is_null() {
                    ::libc::free((*b).header1 as *mut libc::c_void);
                }
                (*b).header1 =
                    crate::stdlib::malloc(crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                    ) as libc::c_ulong) as *mut libc::c_uchar;
                crate::stdlib::memcpy(
                    (*b).header1 as *mut libc::c_void,
                    opb.buffer as *const libc::c_void,
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                    ) as libc::c_ulong,
                );
                (*op_comm).packet = (*b).header1;
                (*op_comm).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                );
                (*op_comm).b_o_s = 0 as libc::c_int as libc::c_long;
                (*op_comm).e_o_s = 0 as libc::c_int as libc::c_long;
                (*op_comm).granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
                (*op_comm).packetno = 1 as libc::c_int as crate::config_types_h::ogg_int64_t;
                /* third header packet (modes/codebooks) ****************************/
                crate::src::libogg_1_3_3::src::bitwise::oggpack_reset(
                    &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                );
                if !(_vorbis_pack_books(&mut opb, vi) != 0) {
                    if !(*b).header2.is_null() {
                        ::libc::free((*b).header2 as *mut libc::c_void);
                    }
                    (*b).header2 = crate::stdlib::malloc(
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                        ) as libc::c_ulong,
                    ) as *mut libc::c_uchar;
                    crate::stdlib::memcpy(
                        (*b).header2 as *mut libc::c_void,
                        opb.buffer as *const libc::c_void,
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                            &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                        ) as libc::c_ulong,
                    );
                    (*op_code).packet = (*b).header2;
                    (*op_code).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                    );
                    (*op_code).b_o_s = 0 as libc::c_int as libc::c_long;
                    (*op_code).e_o_s = 0 as libc::c_int as libc::c_long;
                    (*op_code).granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
                    (*op_code).packetno = 2 as libc::c_int as crate::config_types_h::ogg_int64_t;
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(
                        &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
    }
    crate::stdlib::memset(
        op as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        op_comm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        op_code as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>() as libc::c_ulong,
    );
    if !b.is_null() {
        if (*vi).channels > 0 as libc::c_int {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(
                &mut opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            );
        }
        if !(*b).header.is_null() {
            ::libc::free((*b).header as *mut libc::c_void);
        }
        if !(*b).header1.is_null() {
            ::libc::free((*b).header1 as *mut libc::c_void);
        }
        if !(*b).header2.is_null() {
            ::libc::free((*b).header2 as *mut libc::c_void);
        }
        (*b).header = 0 as *mut libc::c_uchar;
        (*b).header1 = 0 as *mut libc::c_uchar;
        (*b).header2 = 0 as *mut libc::c_uchar
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_granule_time(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut granulepos: crate::config_types_h::ogg_int64_t,
) -> libc::c_double {
    if granulepos == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as libc::c_double;
    }
    /* We're not guaranteed a 64 bit unsigned type everywhere, so we
    have to put the unsigned granpo in a signed type. */
    if granulepos >= 0 as libc::c_int as libc::c_long {
        return granulepos as libc::c_double / (*(*v).vi).rate as libc::c_double;
    } else {
        let mut granuleoff: crate::config_types_h::ogg_int64_t =
            0xffffffff as libc::c_uint as crate::config_types_h::ogg_int64_t;
        granuleoff <<= 31 as libc::c_int;
        granuleoff |= 0x7ffffffff as libc::c_long;
        return (granulepos as libc::c_double
            + 2 as libc::c_int as libc::c_double
            + granuleoff as libc::c_double
            + granuleoff as libc::c_double)
            / (*(*v).vi).rate as libc::c_double;
    };
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_version_string() -> *const libc::c_char {
    return b"Xiph.Org libVorbis 1.3.6\x00" as *const u8 as *const libc::c_char;
}
