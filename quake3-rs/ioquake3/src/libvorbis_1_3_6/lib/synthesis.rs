use ::libc;

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_mapping;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
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
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
use crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc;
use crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_ripcord;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_band;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
use crate::src::libvorbis_1_3_6::lib::registry::_mapping_P;
use crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;
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

function: single-block PCM synthesis

********************************************************************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut vd: *mut crate::codec_h::vorbis_dsp_state = if !vb.is_null() {
        (*vb).vd
    } else {
        0 as *mut crate::codec_h::vorbis_dsp_state
    };
    let mut b: *mut crate::codec_internal_h::private_state = if !vd.is_null() {
        (*vd).backend_state
    } else {
        0 as *mut libc::c_void
    }
        as *mut crate::codec_internal_h::private_state;
    let mut vi: *mut crate::codec_h::vorbis_info = if !vd.is_null() {
        (*vd).vi
    } else {
        0 as *mut crate::codec_h::vorbis_info
    };
    let mut ci: *mut crate::codec_internal_h::codec_setup_info = if !vi.is_null() {
        (*vi).codec_setup
    } else {
        0 as *mut libc::c_void
    }
        as *mut crate::codec_internal_h::codec_setup_info;
    let mut opb: *mut crate::ogg_h::oggpack_buffer = if !vb.is_null() {
        &mut (*vb).opb
    } else {
        0 as *mut crate::ogg_h::oggpack_buffer
    };
    let mut type_0: i32 = 0;
    let mut mode: i32 = 0;
    let mut i: i32 = 0;
    if vd.is_null() || b.is_null() || vi.is_null() || ci.is_null() || opb.is_null() {
        return -(136i32);
    }
    /* first things first.  Make sure decode is ready */
    crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_ripcord(vb);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(opb, (*op).packet, (*op).bytes as i32);
    /* Check the packet type */
    if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1) != 0 {
        /* Oops.  This is not an audio data packet */
        return -(135i32);
    }
    /* read our mode and pre/post windowsize */
    mode = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, (*b).modebits) as i32;
    if mode == -(1) {
        return -(136i32);
    }
    (*vb).mode = mode;
    if (*ci).mode_param[mode as usize].is_null() {
        return -(136i32);
    }
    (*vb).W = (*(*ci).mode_param[mode as usize]).blockflag as isize;
    if (*vb).W != 0 {
        /* this doesn;t get mapped through mode selection as it's used
        only for window selection */
        (*vb).lW = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1);
        (*vb).nW = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1);
        if (*vb).nW == -1isize {
            return -(136i32);
        }
    } else {
        (*vb).lW = 0isize;
        (*vb).nW = 0isize
    }
    /* more setup */
    (*vb).granulepos = (*op).granulepos;
    (*vb).sequence = (*op).packetno;
    (*vb).eofflag = (*op).e_o_s as i32;
    /* alloc pcm passback storage */
    (*vb).pcmend = (*ci).blocksizes[(*vb).W as usize] as i32;
    (*vb).pcm = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb,
        (::std::mem::size_of::<*mut f32>()).wrapping_mul((*vi).channels as usize) as isize,
    ) as *mut *mut f32;

    for i in 0..(*vi).channels {
        let ref mut fresh0 = *(*vb).pcm.offset(i as isize);

        *fresh0 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb,
            ((*vb).pcmend as usize).wrapping_mul(::std::mem::size_of::<f32>()) as isize,
        ) as *mut f32;
    }
    /* unpack_header enforces range checking */
    type_0 = (*ci).map_type[(*(*ci).mode_param[mode as usize]).mapping as usize];
    return (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P
        .as_ptr()
        .offset(type_0 as isize))
    .inverse
    .expect("non-null function pointer")(
        vb,
        (*ci).map_param[(*(*ci).mode_param[mode as usize]).mapping as usize],
    );
}
/* used to track pcm position without actually performing decode.
Useful for sequential 'fast forward' */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_trackonly(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut vd: *mut crate::codec_h::vorbis_dsp_state = (*vb).vd;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*vd).backend_state as *mut crate::codec_internal_h::private_state;
    let mut vi: *mut crate::codec_h::vorbis_info = (*vd).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut opb: *mut crate::ogg_h::oggpack_buffer = &mut (*vb).opb;
    let mut mode: i32 = 0;
    /* first things first.  Make sure decode is ready */
    crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_ripcord(vb);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(opb, (*op).packet, (*op).bytes as i32);
    /* Check the packet type */
    if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1) != 0 {
        /* Oops.  This is not an audio data packet */
        return -(135i32);
    }
    /* read our mode and pre/post windowsize */
    mode = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, (*b).modebits) as i32;
    if mode == -(1) {
        return -(136i32);
    }
    (*vb).mode = mode;
    if (*ci).mode_param[mode as usize].is_null() {
        return -(136i32);
    }
    (*vb).W = (*(*ci).mode_param[mode as usize]).blockflag as isize;
    if (*vb).W != 0 {
        (*vb).lW = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1);
        (*vb).nW = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1);
        if (*vb).nW == -1isize {
            return -(136i32);
        }
    } else {
        (*vb).lW = 0isize;
        (*vb).nW = 0isize
    }
    /* more setup */
    (*vb).granulepos = (*op).granulepos;
    (*vb).sequence = (*op).packetno;
    (*vb).eofflag = (*op).e_o_s as i32;
    /* no pcm */
    (*vb).pcmend = 0;
    (*vb).pcm = 0 as *mut *mut f32;
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_packet_blocksize(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> isize {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut opb: crate::ogg_h::oggpack_buffer = crate::ogg_h::oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    let mut mode: i32 = 0;
    if ci.is_null() || (*ci).modes <= 0 {
        /* codec setup not properly intialized */
        return -129isize;
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit(
        &mut opb,
        (*op).packet,
        (*op).bytes as i32,
    );
    /* Check the packet type */
    if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(&mut opb, 1) != 0 {
        /* Oops.  This is not an audio data packet */
        return -135isize;
    }
    /* read our mode and pre/post windowsize */
    mode = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        &mut opb,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).modes - 1) as crate::config_types_h::ogg_uint32_t,
        ),
    ) as i32;
    if mode == -(1) || (*ci).mode_param[mode as usize].is_null() {
        return -136isize;
    }
    return (*ci).blocksizes[(*(*ci).mode_param[mode as usize]).blockflag as usize];
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_halfrate(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut flag: i32,
) -> i32 {
    /* set / clear half-sample-rate mode */
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    /* right now, our MDCT can't handle < 64 sample windows. */
    if (*ci).blocksizes[0] <= 64 && flag != 0 {
        return -(1i32);
    }
    (*ci).halfrate_flag = if flag != 0 { 1 } else { 0 };
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_halfrate_p(
    mut vi: *mut crate::codec_h::vorbis_info,
) -> i32 {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    return (*ci).halfrate_flag;
}
