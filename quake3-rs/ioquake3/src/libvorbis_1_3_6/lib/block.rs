use ::libc;

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_func_residue;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::private_state;
pub use crate::codec_internal_h::vorbis_block_internal;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::codec_internal_h::vorbis_look_transform;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::vorbis_bitrate_clear;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::vorbis_bitrate_init;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_clear;
pub use crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_init;
pub use crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_mark;
pub use crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_search;
pub use crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_shift;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_band;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_init;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_ampmax_decay;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_global_free;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_global_look;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_psy_clear;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_psy_init;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;

pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_init_decode;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_init_encode;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_clear;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_init;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

#[no_mangle]

pub unsafe extern "C" fn vorbis_block_init(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vb: *mut crate::codec_h::vorbis_block,
) -> i32 {
    let mut i: i32 = 0;
    crate::stdlib::memset(
        vb as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::codec_h::vorbis_block>(),
    );
    (*vb).vd = v;
    (*vb).localalloc = 0isize;
    (*vb).localstore = 0 as *mut libc::c_void;
    if (*v).analysisp != 0 {
        (*vb).internal = crate::stdlib::calloc(
            1,
            ::std::mem::size_of::<crate::codec_internal_h::vorbis_block_internal>(),
        );
        let mut vbi: *mut crate::codec_internal_h::vorbis_block_internal =
            (*vb).internal as *mut crate::codec_internal_h::vorbis_block_internal;
        (*vbi).ampmax = -9999f32;
        i = 0;
        while i < 15 {
            if i == 15 / 2 {
                (*vbi).packetblob[i as usize] = &mut (*vb).opb
            } else {
                (*vbi).packetblob[i as usize] =
                    crate::stdlib::calloc(1, ::std::mem::size_of::<crate::ogg_h::oggpack_buffer>())
                        as *mut crate::ogg_h::oggpack_buffer
            }
            crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit(
                (*vbi).packetblob[i as usize],
            );
            i += 1
        }
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn _vorbis_block_alloc(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut bytes: isize,
) -> *mut libc::c_void {
    bytes = bytes + (8i32 - 1) as isize & !(8i32 - 1) as isize;
    if bytes + (*vb).localtop > (*vb).localalloc {
        /* can't just _ogg_realloc... there are outstanding pointers */
        if !(*vb).localstore.is_null() {
            let mut link: *mut crate::codec_h::alloc_chain =
                crate::stdlib::malloc(::std::mem::size_of::<crate::codec_h::alloc_chain>())
                    as *mut crate::codec_h::alloc_chain;
            (*vb).totaluse += (*vb).localtop;
            (*link).next = (*vb).reap;
            (*link).ptr = (*vb).localstore;
            (*vb).reap = link
        }
        /* highly conservative */
        (*vb).localalloc = bytes;
        (*vb).localstore = crate::stdlib::malloc((*vb).localalloc as usize);
        (*vb).localtop = 0isize
    }
    let mut ret: *mut libc::c_void =
        ((*vb).localstore as *mut i8).offset((*vb).localtop) as *mut libc::c_void;
    (*vb).localtop += bytes;
    return ret;
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2015             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: miscellaneous prototypes

********************************************************************/
/* reap the chain, pull the ripcord */
#[no_mangle]

pub unsafe extern "C" fn _vorbis_block_ripcord(mut vb: *mut crate::codec_h::vorbis_block) {
    /* reap the chain */
    let mut reap: *mut crate::codec_h::alloc_chain = (*vb).reap;
    while !reap.is_null() {
        let mut next: *mut crate::codec_h::alloc_chain = (*reap).next;
        crate::stdlib::free((*reap).ptr);
        crate::stdlib::memset(
            reap as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::codec_h::alloc_chain>(),
        );
        crate::stdlib::free(reap as *mut libc::c_void);
        reap = next
    }
    /* consolidate storage */
    if (*vb).totaluse != 0 {
        (*vb).localstore = crate::stdlib::realloc(
            (*vb).localstore,
            ((*vb).totaluse + (*vb).localalloc) as usize,
        );
        (*vb).localalloc += (*vb).totaluse;
        (*vb).totaluse = 0isize
    }
    /* pull the ripcord */
    (*vb).localtop = 0isize;
    (*vb).reap = 0 as *mut crate::codec_h::alloc_chain;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_block_clear(mut vb: *mut crate::codec_h::vorbis_block) -> i32 {
    let mut _i: i32 = 0;
    let mut vbi: *mut crate::codec_internal_h::vorbis_block_internal =
        (*vb).internal as *mut crate::codec_internal_h::vorbis_block_internal;
    _vorbis_block_ripcord(vb);
    if !(*vb).localstore.is_null() {
        crate::stdlib::free((*vb).localstore);
    }
    if !vbi.is_null() {
        for i in 0..15 {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear(
                (*vbi).packetblob[i as usize],
            );

            if i != 15 / 2 {
                crate::stdlib::free((*vbi).packetblob[i as usize] as *mut libc::c_void);
            }
        }
        crate::stdlib::free(vbi as *mut libc::c_void);
    }
    crate::stdlib::memset(
        vb as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::codec_h::vorbis_block>(),
    );
    return 0;
}
/* Analysis side code, but directly related to blocking.  Thus it's
here and not in analysis.c (which is for analysis transforms only).
The init is here because some of it is shared */

unsafe extern "C" fn _vds_shared_init(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vi: *mut crate::codec_h::vorbis_info,
    mut encp: i32,
) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut b: *mut crate::codec_internal_h::private_state =
        0 as *mut crate::codec_internal_h::private_state;
    let mut hs: i32 = 0;
    if ci.is_null()
        || (*ci).modes <= 0
        || (*ci).blocksizes[0] < 64
        || (*ci).blocksizes[1] < (*ci).blocksizes[0]
    {
        return 1i32;
    }
    hs = (*ci).halfrate_flag;
    crate::stdlib::memset(
        v as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::codec_h::vorbis_dsp_state>(),
    );
    (*v).backend_state = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::codec_internal_h::private_state>(),
    );
    b = (*v).backend_state as *mut crate::codec_internal_h::private_state;
    (*v).vi = vi;
    (*b).modebits = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
        ((*ci).modes - 1) as crate::config_types_h::ogg_uint32_t,
    );
    (*b).transform[0] = crate::stdlib::calloc(1, ::std::mem::size_of::<*mut libc::c_void>())
        as *mut *mut libc::c_void;
    (*b).transform[1] = crate::stdlib::calloc(1, ::std::mem::size_of::<*mut libc::c_void>())
        as *mut *mut libc::c_void;
    /* MDCT is tranform 0 */
    let ref mut fresh0 = *(*b).transform[0].offset(0);
    *fresh0 = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup>(),
    );
    let ref mut fresh1 = *(*b).transform[1].offset(0);
    *fresh1 = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup>(),
    );
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_init(
        *(*b).transform[0].offset(0) as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
        ((*ci).blocksizes[0] >> hs) as i32,
    );
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_init(
        *(*b).transform[1].offset(0) as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
        ((*ci).blocksizes[1] >> hs) as i32,
    );
    /* Vorbis I uses only window type 0 */
    /* note that the correct computation below is technically:
        b->window[0]=ov_ilog(ci->blocksizes[0]-1)-6;
        b->window[1]=ov_ilog(ci->blocksizes[1]-1)-6;
     but since blocksizes are always powers of two,
     the below is equivalent.
    */
    (*b).window[0] = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
        (*ci).blocksizes[0] as crate::config_types_h::ogg_uint32_t,
    ) - 7;
    (*b).window[1] = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
        (*ci).blocksizes[1] as crate::config_types_h::ogg_uint32_t,
    ) - 7;
    if encp != 0 {
        /* encode/decode differ here */
        /* analysis always needs an fft */
        crate::src::libvorbis_1_3_6::lib::smallft::drft_init(
            &mut *(*b).fft_look.as_mut_ptr().offset(0),
            (*ci).blocksizes[0] as i32,
        );
        crate::src::libvorbis_1_3_6::lib::smallft::drft_init(
            &mut *(*b).fft_look.as_mut_ptr().offset(1),
            (*ci).blocksizes[1] as i32,
        );
        /* finish the codebooks */
        if (*ci).fullbooks.is_null() {
            (*ci).fullbooks = crate::stdlib::calloc(
                (*ci).books as usize,
                ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>(),
            )
                as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook;
            i = 0;
            while i < (*ci).books {
                crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_init_encode(
                    (*ci).fullbooks.offset(i as isize),
                    (*ci).book_param[i as usize],
                );
                i += 1
            }
        }
        (*b).psy = crate::stdlib::calloc(
            (*ci).psys as usize,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy>(),
        ) as *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
        i = 0;
        while i < (*ci).psys {
            crate::src::libvorbis_1_3_6::lib::psy::_vp_psy_init(
                (*b).psy.offset(i as isize),
                (*ci).psy_param[i as usize],
                &mut (*ci).psy_g_param,
                ((*ci).blocksizes[(*(*ci).psy_param[i as usize]).blockflag as usize] / 2) as i32,
                (*vi).rate,
            );
            i += 1
        }
        (*v).analysisp = 1
    } else if (*ci).fullbooks.is_null() {
        (*ci).fullbooks = crate::stdlib::calloc(
            (*ci).books as usize,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>(),
        ) as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook;
        i = 0;
        loop {
            if !(i < (*ci).books) {
                current_block = 11743904203796629665;
                break;
            }
            if (*ci).book_param[i as usize].is_null() {
                current_block = 2095308920063660201;
                break;
            }
            if crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_init_decode(
                (*ci).fullbooks.offset(i as isize),
                (*ci).book_param[i as usize],
            ) != 0
            {
                current_block = 2095308920063660201;
                break;
            }
            /* finish the codebooks */
            /* decode codebooks are now standalone after init */
            crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy(
                (*ci).book_param[i as usize],
            );
            (*ci).book_param[i as usize] =
                0 as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
            i += 1
        }
        match current_block {
            11743904203796629665 => {}
            _ => {
                i = 0;
                while i < (*ci).books {
                    if !(*ci).book_param[i as usize].is_null() {
                        crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy(
                            (*ci).book_param[i as usize],
                        );
                        (*ci).book_param[i as usize] =
                            0 as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook
                    }
                    i += 1
                }
                vorbis_dsp_clear(v);
                return -(1i32);
            }
        }
    }
    /* initialize the storage vectors. blocksize[1] is small for encode,
    but the correct size for decode */
    (*v).pcm_storage = (*ci).blocksizes[1] as i32;
    (*v).pcm = crate::stdlib::malloc(
        ((*vi).channels as usize).wrapping_mul(::std::mem::size_of::<*mut f32>()),
    ) as *mut *mut f32;
    (*v).pcmret = crate::stdlib::malloc(
        ((*vi).channels as usize).wrapping_mul(::std::mem::size_of::<*mut f32>()),
    ) as *mut *mut f32;
    let mut _i_0: i32 = 0;

    for i_0 in 0..(*vi).channels {
        let ref mut fresh2 = *(*v).pcm.offset(i_0 as isize);

        *fresh2 = crate::stdlib::calloc((*v).pcm_storage as usize, ::std::mem::size_of::<f32>())
            as *mut f32;
    }
    /* all 1 (large block) or 0 (small block) */
    /* explicitly set for the sake of clarity */
    (*v).lW = 0isize; /* previous window size */
    (*v).W = 0isize; /* current window size */
    /* all vector indexes */
    (*v).centerW = (*ci).blocksizes[1] / 2;
    (*v).pcm_current = (*v).centerW as i32;
    /* initialize all the backend lookups */
    (*b).flr = crate::stdlib::calloc(
        (*ci).floors as usize,
        ::std::mem::size_of::<*mut libc::c_void>(),
    ) as *mut *mut libc::c_void;
    (*b).residue = crate::stdlib::calloc(
        (*ci).residues as usize,
        ::std::mem::size_of::<*mut libc::c_void>(),
    ) as *mut *mut libc::c_void;
    i = 0;
    while i < (*ci).floors {
        let ref mut fresh3 = *(*b).flr.offset(i as isize);
        *fresh3 = (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
            .as_ptr()
            .offset((*ci).floor_type[i as usize] as isize))
        .look
        .expect("non-null function pointer")(v, (*ci).floor_param[i as usize]);
        i += 1
    }
    i = 0;
    while i < (*ci).residues {
        let ref mut fresh4 = *(*b).residue.offset(i as isize);
        *fresh4 = (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
            .as_ptr()
            .offset((*ci).residue_type[i as usize] as isize))
        .look
        .expect("non-null function pointer")(v, (*ci).residue_param[i as usize]);
        i += 1
    }
    return 0;
}
/* Vorbis PRIMITIVES: analysis/DSP layer ****************************/
/* arbitrary settings and spec-mandated numbers get filled in here */
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_init(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> i32 {
    let mut b: *mut crate::codec_internal_h::private_state =
        0 as *mut crate::codec_internal_h::private_state;
    if _vds_shared_init(v, vi, 1) != 0 {
        return 1i32;
    }
    b = (*v).backend_state as *mut crate::codec_internal_h::private_state;
    (*b).psy_g_look = crate::src::libvorbis_1_3_6::lib::psy::_vp_global_look(vi);
    /* Initialize the envelope state storage */
    (*b).ve = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup>(),
    ) as *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
    crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_init((*b).ve, vi);
    crate::src::libvorbis_1_3_6::lib::bitrate::vorbis_bitrate_init(vi, &mut (*b).bms);
    /* compressed audio packets start after the headers
    with sequence number 3 */
    (*v).sequence = 3isize;
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_dsp_clear(mut v: *mut crate::codec_h::vorbis_dsp_state) {
    let mut i: i32 = 0;
    if !v.is_null() {
        let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
        let mut ci: *mut crate::codec_internal_h::codec_setup_info = if !vi.is_null() {
            (*vi).codec_setup
        } else {
            0 as *mut libc::c_void
        }
            as *mut crate::codec_internal_h::codec_setup_info;
        let mut b: *mut crate::codec_internal_h::private_state =
            (*v).backend_state as *mut crate::codec_internal_h::private_state;
        if !b.is_null() {
            if !(*b).ve.is_null() {
                crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_clear((*b).ve);
                crate::stdlib::free((*b).ve as *mut libc::c_void);
            }
            if !(*b).transform[0].is_null() {
                crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear(*(*b).transform[0].offset(0)
                    as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup);
                crate::stdlib::free(*(*b).transform[0].offset(0));
                crate::stdlib::free((*b).transform[0usize] as *mut libc::c_void);
            }
            if !(*b).transform[1].is_null() {
                crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear(*(*b).transform[1].offset(0)
                    as *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup);
                crate::stdlib::free(*(*b).transform[1].offset(0));
                crate::stdlib::free((*b).transform[1usize] as *mut libc::c_void);
            }
            if !(*b).flr.is_null() {
                if !ci.is_null() {
                    i = 0;
                    while i < (*ci).floors {
                        (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                            .as_ptr()
                            .offset((*ci).floor_type[i as usize] as isize))
                        .free_look
                        .expect("non-null function pointer")(
                            *(*b).flr.offset(i as isize)
                        );
                        i += 1
                    }
                }
                crate::stdlib::free((*b).flr as *mut libc::c_void);
            }
            if !(*b).residue.is_null() {
                if !ci.is_null() {
                    i = 0;
                    while i < (*ci).residues {
                        (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                            .as_ptr()
                            .offset((*ci).residue_type[i as usize] as isize))
                        .free_look
                        .expect("non-null function pointer")(
                            *(*b).residue.offset(i as isize)
                        );
                        i += 1
                    }
                }
                crate::stdlib::free((*b).residue as *mut libc::c_void);
            }
            if !(*b).psy.is_null() {
                if !ci.is_null() {
                    i = 0;
                    while i < (*ci).psys {
                        crate::src::libvorbis_1_3_6::lib::psy::_vp_psy_clear(
                            (*b).psy.offset(i as isize),
                        );
                        i += 1
                    }
                }
                crate::stdlib::free((*b).psy as *mut libc::c_void);
            }
            if !(*b).psy_g_look.is_null() {
                crate::src::libvorbis_1_3_6::lib::psy::_vp_global_free((*b).psy_g_look);
            }
            crate::src::libvorbis_1_3_6::lib::bitrate::vorbis_bitrate_clear(&mut (*b).bms);
            crate::src::libvorbis_1_3_6::lib::smallft::drft_clear(
                &mut *(*b).fft_look.as_mut_ptr().offset(0),
            );
            crate::src::libvorbis_1_3_6::lib::smallft::drft_clear(
                &mut *(*b).fft_look.as_mut_ptr().offset(1isize),
            );
        }
        if !(*v).pcm.is_null() {
            if !vi.is_null() {
                i = 0;
                while i < (*vi).channels {
                    if !(*(*v).pcm.offset(i as isize)).is_null() {
                        crate::stdlib::free(*(*v).pcm.offset(i as isize) as *mut libc::c_void);
                    }
                    i += 1
                }
            }
            crate::stdlib::free((*v).pcm as *mut libc::c_void);
            if !(*v).pcmret.is_null() {
                crate::stdlib::free((*v).pcmret as *mut libc::c_void);
            }
        }
        if !b.is_null() {
            /* free header, header1, header2 */
            if !(*b).header.is_null() {
                crate::stdlib::free((*b).header as *mut libc::c_void);
            }
            if !(*b).header1.is_null() {
                crate::stdlib::free((*b).header1 as *mut libc::c_void);
            }
            if !(*b).header2.is_null() {
                crate::stdlib::free((*b).header2 as *mut libc::c_void);
            }
            crate::stdlib::free(b as *mut libc::c_void);
        }
        crate::stdlib::memset(
            v as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::codec_h::vorbis_dsp_state>(),
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_buffer(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vals: i32,
) -> *mut *mut f32 {
    let mut i: i32 = 0;
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*v).backend_state as *mut crate::codec_internal_h::private_state;
    /* free header, header1, header2 */
    if !(*b).header.is_null() {
        crate::stdlib::free((*b).header as *mut libc::c_void);
    }
    (*b).header = 0 as *mut u8;
    if !(*b).header1.is_null() {
        crate::stdlib::free((*b).header1 as *mut libc::c_void);
    }
    (*b).header1 = 0 as *mut u8;
    if !(*b).header2.is_null() {
        crate::stdlib::free((*b).header2 as *mut libc::c_void);
    }
    (*b).header2 = 0 as *mut u8;
    /* Do we have enough storage space for the requested buffer? If not,
    expand the PCM (and envelope) storage */
    if (*v).pcm_current + vals >= (*v).pcm_storage {
        (*v).pcm_storage = (*v).pcm_current + vals * 2;
        i = 0;
        while i < (*vi).channels {
            let ref mut fresh5 = *(*v).pcm.offset(i as isize);
            *fresh5 = crate::stdlib::realloc(
                *(*v).pcm.offset(i as isize) as *mut libc::c_void,
                ((*v).pcm_storage as usize).wrapping_mul(::std::mem::size_of::<f32>()),
            ) as *mut f32;
            i += 1
        }
    }
    i = 0;
    while i < (*vi).channels {
        let ref mut fresh6 = *(*v).pcmret.offset(i as isize);
        *fresh6 = (*(*v).pcm.offset(i as isize)).offset((*v).pcm_current as isize);
        i += 1
    }
    return (*v).pcmret;
}

unsafe extern "C" fn _preextrapolate_helper(mut v: *mut crate::codec_h::vorbis_dsp_state) {
    let mut i: i32 = 0;
    let mut order: i32 = 16;
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (order as usize).wrapping_mul(::std::mem::size_of::<f32>()),
    );
    let mut lpc: *mut f32 = fresh7.as_mut_ptr() as *mut f32;
    let mut fresh8 = ::std::vec::from_elem(
        0,
        ((*v).pcm_current as usize).wrapping_mul(::std::mem::size_of::<f32>()),
    );
    let mut work: *mut f32 = fresh8.as_mut_ptr() as *mut f32;
    let mut j: isize = 0;
    (*v).preextrapolate = 1;
    if (*v).pcm_current as isize - (*v).centerW > (order * 2) as isize {
        /* safety */
        i = 0;
        while i < (*(*v).vi).channels {
            /* need to run the extrapolation in reverse! */
            j = 0;
            while j < (*v).pcm_current as isize {
                *work.offset(j) =
                    *(*(*v).pcm.offset(i as isize)).offset((*v).pcm_current as isize - j - 1);
                j += 1
            }
            /* prime as above */
            crate::src::libvorbis_1_3_6::lib::lpc::vorbis_lpc_from_data(
                work,
                lpc,
                ((*v).pcm_current as isize - (*v).centerW) as i32,
                order,
            );
            /* run the predictor filter */
            crate::src::libvorbis_1_3_6::lib::lpc::vorbis_lpc_predict(
                lpc,
                work.offset((*v).pcm_current as isize)
                    .offset(-((*v).centerW))
                    .offset(-(order as isize)),
                order,
                work.offset((*v).pcm_current as isize)
                    .offset(-((*v).centerW)),
                (*v).centerW,
            );
            j = 0;
            while j < (*v).pcm_current as isize {
                *(*(*v).pcm.offset(i as isize)).offset((*v).pcm_current as isize - j - 1) =
                    *work.offset(j);
                j += 1
            }
            i += 1
        }
    };
}
/* call with val<=0 to set eof */
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_wrote(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vals: i32,
) -> i32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    if vals <= 0 {
        let mut order: i32 = 32;
        let mut i: i32 = 0;
        let mut fresh9 = ::std::vec::from_elem(
            0,
            (order as usize).wrapping_mul(::std::mem::size_of::<f32>()),
        );
        let mut lpc: *mut f32 = fresh9.as_mut_ptr() as *mut f32;
        /* if it wasn't done earlier (very short sample) */
        if (*v).preextrapolate == 0 {
            _preextrapolate_helper(v);
        }
        /* We're encoding the end of the stream.  Just make sure we have
        [at least] a few full blocks of zeroes at the end. */
        /* actually, we don't want zeroes; that could drop a large
        amplitude off a cliff, creating spread spectrum noise that will
        suck to encode.  Extrapolate for the sake of cleanliness. */
        vorbis_analysis_buffer(v, ((*ci).blocksizes[1] * 3) as i32);
        (*v).eofflag = (*v).pcm_current;
        (*v).pcm_current = ((*v).pcm_current as isize + (*ci).blocksizes[1] * 3) as i32;
        i = 0;
        while i < (*vi).channels {
            if (*v).eofflag > order * 2 {
                /* extrapolate with LPC to fill in */
                let mut n: isize = 0;
                /* make a predictor filter */
                n = (*v).eofflag as isize;
                if n > (*ci).blocksizes[1] {
                    n = (*ci).blocksizes[1]
                }
                crate::src::libvorbis_1_3_6::lib::lpc::vorbis_lpc_from_data(
                    (*(*v).pcm.offset(i as isize))
                        .offset((*v).eofflag as isize)
                        .offset(-(n)),
                    lpc,
                    n as i32,
                    order,
                );
                /* run the predictor filter */
                crate::src::libvorbis_1_3_6::lib::lpc::vorbis_lpc_predict(
                    lpc,
                    (*(*v).pcm.offset(i as isize))
                        .offset((*v).eofflag as isize)
                        .offset(-(order as isize)),
                    order,
                    (*(*v).pcm.offset(i as isize)).offset((*v).eofflag as isize),
                    ((*v).pcm_current - (*v).eofflag) as isize,
                );
            } else {
                /* not enough data to extrapolate (unlikely to happen due to
                guarding the overlap, but bulletproof in case that
                assumtion goes away). zeroes will do. */
                crate::stdlib::memset(
                    (*(*v).pcm.offset(i as isize)).offset((*v).eofflag as isize)
                        as *mut libc::c_void,
                    0i32,
                    (((*v).pcm_current - (*v).eofflag) as usize)
                        .wrapping_mul(::std::mem::size_of::<f32>()),
                );
            }
            i += 1
        }
    } else {
        if (*v).pcm_current + vals > (*v).pcm_storage {
            return -(131i32);
        }
        (*v).pcm_current += vals;
        /* we may want to reverse extrapolate the beginning of a stream
        too... in case we're beginning on a cliff! */
        /* clumsy, but simple.  It only runs once, so simple is good. */
        if (*v).preextrapolate == 0
            && (*v).pcm_current as isize - (*v).centerW > (*ci).blocksizes[1]
        {
            _preextrapolate_helper(v);
        }
    }
    return 0;
}
/* do the deltas, envelope shaping, pre-echo and determine the size of
the next block on which to continue analysis */
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_blockout(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vb: *mut crate::codec_h::vorbis_block,
) -> i32 {
    let mut i: i32 = 0;
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*v).backend_state as *mut crate::codec_internal_h::private_state;
    let mut g: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global = (*b).psy_g_look;
    let mut beginW: isize = (*v).centerW - (*ci).blocksizes[(*v).W as usize] / 2;
    let mut centerNext: isize = 0;
    let mut vbi: *mut crate::codec_internal_h::vorbis_block_internal =
        (*vb).internal as *mut crate::codec_internal_h::vorbis_block_internal;
    /* check to see if we're started... */
    if (*v).preextrapolate == 0 {
        return 0i32;
    }
    /* check to see if we're done... */
    if (*v).eofflag == -(1) {
        return 0i32;
    }
    /* By our invariant, we have lW, W and centerW set.  Search for
    the next boundary so we can determine nW (the next window size)
    which lets us compute the shape of the current block's window */
    /* we do an envelope search even on a single blocksize; we may still
    be throwing more bits at impulses, and envelope search handles
    marking impulses too. */
    let mut bp: isize = crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_search(v); /* not enough data currently to search for a
                                                                                            full long block */
    if bp == -1 {
        if (*v).eofflag == 0 {
            return 0i32;
        }
        (*v).nW = 0isize
    } else if (*ci).blocksizes[0] == (*ci).blocksizes[1] {
        (*v).nW = 0isize
    } else {
        (*v).nW = bp
    }
    centerNext = (*v).centerW
        + (*ci).blocksizes[(*v).W as usize] / 4
        + (*ci).blocksizes[(*v).nW as usize] / 4;
    /* center of next block + next block maximum right side. */
    let mut blockbound: isize = centerNext + (*ci).blocksizes[(*v).nW as usize] / 2;
    if ((*v).pcm_current as isize) < blockbound {
        return 0i32;
    }
    /* fill in the block.  Note that for a short window, lW and nW are *short*
    regardless of actual settings in the stream */
    _vorbis_block_ripcord(vb);
    (*vb).lW = (*v).lW;
    (*vb).W = (*v).W;
    (*vb).nW = (*v).nW;
    if (*v).W != 0 {
        if (*v).lW == 0 || (*v).nW == 0 {
            (*vbi).blocktype = 0
        /*fprintf(stderr,"-");*/
        } else {
            (*vbi).blocktype = 1
            /*fprintf(stderr,"_");*/
        }
    } else if crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_mark(v) != 0 {
        (*vbi).blocktype = 0
    /*fprintf(stderr,"|");*/
    } else {
        (*vbi).blocktype = 1
        /*fprintf(stderr,".");*/
    }
    (*vb).vd = v;
    let fresh10 = (*v).sequence;
    (*v).sequence = (*v).sequence + 1;
    (*vb).sequence = fresh10;
    (*vb).granulepos = (*v).granulepos;
    (*vb).pcmend = (*ci).blocksizes[(*v).W as usize] as i32;
    /* copy the vectors; this uses the local storage in vb */
    /* this tracks 'strongest peak' for later psychoacoustics */
    /* moved to the global psy state; clean this mess up */
    if (*vbi).ampmax > (*g).ampmax {
        (*g).ampmax = (*vbi).ampmax
    }
    (*g).ampmax = crate::src::libvorbis_1_3_6::lib::psy::_vp_ampmax_decay((*g).ampmax, v);
    (*vbi).ampmax = (*g).ampmax;
    (*vb).pcm = _vorbis_block_alloc(
        vb,
        (::std::mem::size_of::<*mut f32>()).wrapping_mul((*vi).channels as usize) as isize,
    ) as *mut *mut f32;
    (*vbi).pcmdelay = _vorbis_block_alloc(
        vb,
        (::std::mem::size_of::<*mut f32>()).wrapping_mul((*vi).channels as usize) as isize,
    ) as *mut *mut f32;
    i = 0;
    while i < (*vi).channels {
        let ref mut fresh11 = *(*vbi).pcmdelay.offset(i as isize);
        *fresh11 = _vorbis_block_alloc(
            vb,
            (((*vb).pcmend as isize + beginW) as usize).wrapping_mul(::std::mem::size_of::<f32>())
                as isize,
        ) as *mut f32;
        crate::stdlib::memcpy(
            *(*vbi).pcmdelay.offset(i as isize) as *mut libc::c_void,
            *(*v).pcm.offset(i as isize) as *const libc::c_void,
            (((*vb).pcmend as isize + beginW) as usize).wrapping_mul(::std::mem::size_of::<f32>()),
        );
        let ref mut fresh12 = *(*vb).pcm.offset(i as isize);
        *fresh12 = (*(*vbi).pcmdelay.offset(i as isize)).offset(beginW);
        i += 1
        /* before we added the delay
           vb->pcm[i]=_vorbis_block_alloc(vb,vb->pcmend*sizeof(*vb->pcm[i]));
           memcpy(vb->pcm[i],v->pcm[i]+beginW,ci->blocksizes[v->W]*sizeof(*vb->pcm[i]));
        */
    }
    /* handle eof detection: eof==0 means that we've not yet received EOF
    eof>0  marks the last 'real' sample in pcm[]
    eof<0  'no more to do'; doesn't get here */
    if (*v).eofflag != 0 {
        if (*v).centerW >= (*v).eofflag as isize {
            (*v).eofflag = -(1);
            (*vb).eofflag = 1;
            return 1i32;
        }
    }
    /* advance storage vectors and clean up */
    let mut new_centerNext: i32 = ((*ci).blocksizes[1] / 2) as i32;
    let mut movementW: i32 = (centerNext - new_centerNext as isize) as i32;
    if movementW > 0 {
        crate::src::libvorbis_1_3_6::lib::envelope::_ve_envelope_shift((*b).ve, movementW as isize);
        (*v).pcm_current -= movementW;
        i = 0;
        while i < (*vi).channels {
            crate::stdlib::memmove(
                *(*v).pcm.offset(i as isize) as *mut libc::c_void,
                (*(*v).pcm.offset(i as isize)).offset(movementW as isize) as *const libc::c_void,
                ((*v).pcm_current as usize).wrapping_mul(::std::mem::size_of::<f32>()),
            );
            i += 1
        }
        (*v).lW = (*v).W;
        (*v).W = (*v).nW;
        (*v).centerW = new_centerNext as isize;
        if (*v).eofflag != 0 {
            (*v).eofflag -= movementW;
            if (*v).eofflag <= 0 {
                (*v).eofflag = -(1)
            }
            /* do not add padding to end of stream! */
            if (*v).centerW >= (*v).eofflag as isize {
                (*v).granulepos += movementW as isize - ((*v).centerW - (*v).eofflag as isize)
            } else {
                (*v).granulepos += movementW as isize
            }
        } else {
            (*v).granulepos += movementW as isize
        }
    }
    /* done */
    return 1;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_restart(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
) -> i32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        0 as *mut crate::codec_internal_h::codec_setup_info;
    let mut hs: i32 = 0;
    if (*v).backend_state.is_null() {
        return -(1i32);
    }
    if vi.is_null() {
        return -(1i32);
    }
    ci = (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    if ci.is_null() {
        return -(1i32);
    }
    hs = (*ci).halfrate_flag;
    (*v).centerW = (*ci).blocksizes[1] >> hs + 1;
    (*v).pcm_current = ((*v).centerW >> hs) as i32;
    (*v).pcm_returned = -(1);
    (*v).granulepos = -1isize;
    (*v).sequence = -1isize;
    (*v).eofflag = 0;
    (*((*v).backend_state as *mut crate::codec_internal_h::private_state)).sample_count = -1;
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_init(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> i32 {
    if _vds_shared_init(v, vi, 0) != 0 {
        vorbis_dsp_clear(v);
        return 1i32;
    }
    vorbis_synthesis_restart(v);
    return 0;
}
/* Unlike in analysis, the window is only partially applied for each
block.  The time domain envelope is not yet handled at the point of
calling (as it relies on the previous block). */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_blockin(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut vb: *mut crate::codec_h::vorbis_block,
) -> i32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi; /* out of sequence; lose count */
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*v).backend_state as *mut crate::codec_internal_h::private_state;
    let mut hs: i32 = (*ci).halfrate_flag;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    if vb.is_null() {
        return -(131i32);
    }
    if (*v).pcm_current > (*v).pcm_returned && (*v).pcm_returned != -(1) {
        return -(131i32);
    }
    (*v).lW = (*v).W;
    (*v).W = (*vb).W;
    (*v).nW = -1isize;
    if (*v).sequence == -1isize || (*v).sequence + 1isize != (*vb).sequence {
        (*v).granulepos = -1isize;
        (*b).sample_count = -1
    }
    (*v).sequence = (*vb).sequence;
    if !(*vb).pcm.is_null() {
        /* no pcm to process if vorbis_synthesis_trackonly
        was called on block */
        let mut n: i32 = ((*ci).blocksizes[(*v).W as usize] >> hs + 1) as i32;
        let mut n0: i32 = ((*ci).blocksizes[0] >> hs + 1) as i32;
        let mut n1: i32 = ((*ci).blocksizes[1] >> hs + 1) as i32;
        let mut thisCenter: i32 = 0;
        let mut prevCenter: i32 = 0;
        (*v).glue_bits += (*vb).glue_bits;
        (*v).time_bits += (*vb).time_bits;
        (*v).floor_bits += (*vb).floor_bits;
        (*v).res_bits += (*vb).res_bits;
        if (*v).centerW != 0 {
            thisCenter = n1;
            prevCenter = 0
        } else {
            thisCenter = 0;
            prevCenter = n1
        }
        /* v->pcm is now used like a two-stage double buffer.  We don't want
        to have to constantly shift *or* adjust memory usage.  Don't
        accept a new block until the old is shifted out */

        for j in 0..(*vi).channels {
            if (*v).lW != 0 {
                if (*v).W != 0 {
                    /* large/large */
                    let mut w: *const f32 =
                        crate::src::libvorbis_1_3_6::lib::window::_vorbis_window_get(
                            (*b).window[1] - hs,
                        );
                    let mut pcm: *mut f32 =
                        (*(*v).pcm.offset(j as isize)).offset(prevCenter as isize);
                    let mut p: *mut f32 = *(*vb).pcm.offset(j as isize);
                    i = 0;
                    while i < n1 {
                        *pcm.offset(i as isize) = *pcm.offset(i as isize)
                            * *w.offset((n1 - i - 1) as isize)
                            + *p.offset(i as isize) * *w.offset(i as isize);
                        i += 1
                    }
                } else {
                    /* large/small */
                    let mut w_0: *const f32 =
                        crate::src::libvorbis_1_3_6::lib::window::_vorbis_window_get(
                            (*b).window[0] - hs,
                        );
                    let mut pcm_0: *mut f32 = (*(*v).pcm.offset(j as isize))
                        .offset(prevCenter as isize)
                        .offset((n1 / 2) as isize)
                        .offset(-((n0 / 2) as isize));
                    let mut p_0: *mut f32 = *(*vb).pcm.offset(j as isize);
                    i = 0;
                    while i < n0 {
                        *pcm_0.offset(i as isize) = *pcm_0.offset(i as isize)
                            * *w_0.offset((n0 - i - 1) as isize)
                            + *p_0.offset(i as isize) * *w_0.offset(i as isize);
                        i += 1
                    }
                }
            } else if (*v).W != 0 {
                /* small/large */
                let mut w_1: *const f32 =
                    crate::src::libvorbis_1_3_6::lib::window::_vorbis_window_get(
                        (*b).window[0] - hs,
                    );
                let mut pcm_1: *mut f32 =
                    (*(*v).pcm.offset(j as isize)).offset(prevCenter as isize);
                let mut p_1: *mut f32 = (*(*vb).pcm.offset(j as isize))
                    .offset((n1 / 2) as isize)
                    .offset(-((n0 / 2) as isize));
                i = 0;
                while i < n0 {
                    *pcm_1.offset(i as isize) = *pcm_1.offset(i as isize)
                        * *w_1.offset((n0 - i - 1) as isize)
                        + *p_1.offset(i as isize) * *w_1.offset(i as isize);
                    i += 1
                }
                while i < n1 / 2 + n0 / 2 {
                    *pcm_1.offset(i as isize) = *p_1.offset(i as isize);
                    i += 1
                }
            } else {
                /* small/small */
                let mut w_2: *const f32 =
                    crate::src::libvorbis_1_3_6::lib::window::_vorbis_window_get(
                        (*b).window[0] - hs,
                    );
                let mut pcm_2: *mut f32 =
                    (*(*v).pcm.offset(j as isize)).offset(prevCenter as isize);
                let mut p_2: *mut f32 = *(*vb).pcm.offset(j as isize);
                i = 0;
                while i < n0 {
                    *pcm_2.offset(i as isize) = *pcm_2.offset(i as isize)
                        * *w_2.offset((n0 - i - 1) as isize)
                        + *p_2.offset(i as isize) * *w_2.offset(i as isize);
                    i += 1
                }
            }

            let mut pcm_3: *mut f32 = (*(*v).pcm.offset(j as isize)).offset(thisCenter as isize);

            let mut p_3: *mut f32 = (*(*vb).pcm.offset(j as isize)).offset(n as isize);

            i = 0;

            while i < n {
                *pcm_3.offset(i as isize) = *p_3.offset(i as isize);
                i += 1
            }
        }
        if (*v).centerW != 0 {
            (*v).centerW = 0isize
        } else {
            (*v).centerW = n1 as isize
        }
        /* deal with initial packet state; we do this using the explicit
        pcm_returned==-1 flag otherwise we're sensitive to first block
        being short or long */
        if (*v).pcm_returned == -(1) {
            (*v).pcm_returned = thisCenter;
            (*v).pcm_current = thisCenter
        } else {
            (*v).pcm_returned = prevCenter;
            (*v).pcm_current = (prevCenter as isize
                + ((*ci).blocksizes[(*v).lW as usize] / 4 + (*ci).blocksizes[(*v).W as usize] / 4
                    >> hs)) as i32
        }
    }
    /* track the frame number... This is for convenience, but also
    making sure our last packet doesn't end with added padding.  If
    the last packet is partial, the number of samples we'll have to
    return will be past the vb->granulepos.

    This is not foolproof!  It will be confused if we begin
    decoding at the last page after a seek or hole.  In that case,
    we don't have a starting point to judge where the last frame
    is.  For this reason, vorbisfile will always try to make sure
    it reads the last two marked pages in proper sequence */
    if (*b).sample_count == -1 {
        (*b).sample_count = 0
    } else {
        (*b).sample_count +=
            (*ci).blocksizes[(*v).lW as usize] / 4 + (*ci).blocksizes[(*v).W as usize] / 4
    }
    if (*v).granulepos == -1isize {
        if (*vb).granulepos != -1isize {
            /* only set if we have a position to set to */
            (*v).granulepos = (*vb).granulepos;
            /* is this a short page? */
            if (*b).sample_count > (*v).granulepos {
                /* corner case; if this is both the first and last audio page,
                then spec says the end is cut, not beginning */
                let mut extra: isize = (*b).sample_count - (*vb).granulepos;
                /* we use ogg_int64_t for granule positions because a
                uint64 isn't universally available.  Unfortunately,
                that means granposes can be 'negative' and result in
                extra being negative */
                if extra < 0 {
                    extra = 0
                }
                if (*vb).eofflag != 0 {
                    /* trim the end */
                    /* no preceding granulepos; assume we started at zero (we'd
                    have to in a short single-page stream) */
                    /* granulepos could be -1 due to a seek, but that would result
                    in a long count, not short count */
                    /* Guard against corrupt/malicious frames that set EOP and
                    a backdated granpos; don't rewind more samples than we
                    actually have */
                    if extra > ((*v).pcm_current - (*v).pcm_returned << hs) as isize {
                        extra = ((*v).pcm_current - (*v).pcm_returned << hs) as isize
                    }
                    (*v).pcm_current = ((*v).pcm_current as isize - (extra >> hs)) as i32
                } else {
                    /* trim the beginning */
                    (*v).pcm_returned = ((*v).pcm_returned as isize + (extra >> hs)) as i32; /* else {Shouldn't happen *unless* the bitstream is out of
                                                                                             spec.  Either way, believe the bitstream } */
                    if (*v).pcm_returned > (*v).pcm_current {
                        (*v).pcm_returned = (*v).pcm_current
                    }
                }
            }
        }
    } else {
        (*v).granulepos +=
            (*ci).blocksizes[(*v).lW as usize] / 4 + (*ci).blocksizes[(*v).W as usize] / 4;
        if (*vb).granulepos != -1isize && (*v).granulepos != (*vb).granulepos {
            if (*v).granulepos > (*vb).granulepos {
                let mut extra_0: isize = (*v).granulepos - (*vb).granulepos;
                if extra_0 != 0 {
                    if (*vb).eofflag != 0 {
                        /* else {Shouldn't happen *unless* the bitstream is out of
                        spec.  Either way, believe the bitstream } */
                        /* partial last frame.  Strip the extra samples off */
                        /* Guard against corrupt/malicious frames that set EOP and
                        a backdated granpos; don't rewind more samples than we
                        actually have */
                        if extra_0 > ((*v).pcm_current - (*v).pcm_returned << hs) as isize {
                            extra_0 = ((*v).pcm_current - (*v).pcm_returned << hs) as isize
                        }
                        /* we use ogg_int64_t for granule positions because a
                        uint64 isn't universally available.  Unfortunately,
                        that means granposes can be 'negative' and result in
                        extra being negative */
                        if extra_0 < 0 {
                            extra_0 = 0
                        }
                        (*v).pcm_current = ((*v).pcm_current as isize - (extra_0 >> hs)) as i32
                    }
                }
            }
            (*v).granulepos = (*vb).granulepos
        }
    }
    /* Update, cleanup */
    if (*vb).eofflag != 0 {
        (*v).eofflag = 1
    }
    return 0;
}
/* pcm==NULL indicates we just want the pending samples, no more */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_pcmout(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut pcm: *mut *mut *mut f32,
) -> i32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    if (*v).pcm_returned > -(1) && (*v).pcm_returned < (*v).pcm_current {
        if !pcm.is_null() {
            let mut _i: i32 = 0;

            for i in 0..(*vi).channels {
                let ref mut fresh13 = *(*v).pcmret.offset(i as isize);

                *fresh13 = (*(*v).pcm.offset(i as isize)).offset((*v).pcm_returned as isize);
            }
            *pcm = (*v).pcmret
        }
        return (*v).pcm_current - (*v).pcm_returned;
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_read(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut n: i32,
) -> i32 {
    if n != 0 && (*v).pcm_returned + n > (*v).pcm_current {
        return -(131i32);
    }
    (*v).pcm_returned += n;
    return 0;
}
/* intended for use with a specific vorbisfile feature; we want access
to the [usually synthetic/postextrapolated] buffer and lapping at
the end of a decode cycle, specifically, a half-short-block worth.
This funtion works like pcmout above, except it will also expose
this implicit buffer data not normally decoded. */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_lapout(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut pcm: *mut *mut *mut f32,
) -> i32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut hs: i32 = (*ci).halfrate_flag;
    let mut n: i32 = ((*ci).blocksizes[(*v).W as usize] >> hs + 1) as i32;
    let mut n0: i32 = ((*ci).blocksizes[0] >> hs + 1) as i32;
    let mut n1: i32 = ((*ci).blocksizes[1] >> hs + 1) as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if (*v).pcm_returned < 0 {
        return 0i32;
    }
    /* our returned data ends at pcm_returned; because the synthesis pcm
    buffer is a two-fragment ring, that means our data block may be
    fragmented by buffering, wrapping or a short block not filling
    out a buffer.  To simplify things, we unfragment if it's at all
    possibly needed. Otherwise, we'd need to call lapout more than
    once as well as hold additional dsp state.  Opt for
    simplicity. */
    /* centerW was advanced by blockin; it would be the center of the
     *next* block */
    if (*v).centerW == n1 as isize {
        /* the data buffer wraps; swap the halves */
        /* slow, sure, small */
        j = 0;
        while j < (*vi).channels {
            let mut p: *mut f32 = *(*v).pcm.offset(j as isize);
            i = 0;
            while i < n1 {
                let mut temp: f32 = *p.offset(i as isize);
                *p.offset(i as isize) = *p.offset((i + n1) as isize);
                *p.offset((i + n1) as isize) = temp;
                i += 1
            }
            j += 1
        }
        (*v).pcm_current -= n1;
        (*v).pcm_returned -= n1;
        (*v).centerW = 0isize
    }
    /* solidify buffer into contiguous space */
    if (*v).lW ^ (*v).W == 1isize {
        /* long/short or short/long */
        j = 0;
        while j < (*vi).channels {
            let mut s: *mut f32 = *(*v).pcm.offset(j as isize);
            let mut d: *mut f32 = (*(*v).pcm.offset(j as isize)).offset(((n1 - n0) / 2) as isize);
            i = (n1 + n0) / 2 - 1;
            while i >= 0 {
                *d.offset(i as isize) = *s.offset(i as isize);
                i -= 1
            }
            j += 1
        }
        (*v).pcm_returned += (n1 - n0) / 2;
        (*v).pcm_current += (n1 - n0) / 2
    } else if (*v).lW == 0isize {
        /* short/short */
        j = 0;
        while j < (*vi).channels {
            let mut s_0: *mut f32 = *(*v).pcm.offset(j as isize);
            let mut d_0: *mut f32 = (*(*v).pcm.offset(j as isize))
                .offset(n1 as isize)
                .offset(-(n0 as isize));
            i = n0 - 1;
            while i >= 0 {
                *d_0.offset(i as isize) = *s_0.offset(i as isize);
                i -= 1
            }
            j += 1
        }
        (*v).pcm_returned += n1 - n0;
        (*v).pcm_current += n1 - n0
    }
    if !pcm.is_null() {
        let mut _i_0: i32 = 0;

        for i_0 in 0..(*vi).channels {
            let ref mut fresh14 = *(*v).pcmret.offset(i_0 as isize);

            *fresh14 = (*(*v).pcm.offset(i_0 as isize)).offset((*v).pcm_returned as isize);
        }
        *pcm = (*v).pcmret
    }
    return n1 + n - (*v).pcm_returned;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_window(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
    mut W: i32,
) -> *const f32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut hs: i32 = (*ci).halfrate_flag;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*v).backend_state as *mut crate::codec_internal_h::private_state;
    if ((*b).window[W as usize] - 1) < 0 {
        return 0 as *const f32;
    }
    return crate::src::libvorbis_1_3_6::lib::window::_vorbis_window_get(
        (*b).window[W as usize] - hs,
    );
}
