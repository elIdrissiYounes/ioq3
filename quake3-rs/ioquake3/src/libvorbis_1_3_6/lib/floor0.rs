use ::libc;

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_info_floor0;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodev_set;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;

pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_floor0 {
    pub ln: i32,
    pub m: i32,
    pub linearmap: *mut *mut i32,
    pub n: [i32; 2],
    pub vi: *mut crate::backends_h::vorbis_info_floor0,
    pub bits: isize,
    pub frames: isize,
}
/* **********************************************/

unsafe extern "C" fn floor0_free_info(mut i: *mut libc::c_void) {
    let mut info: *mut crate::backends_h::vorbis_info_floor0 =
        i as *mut crate::backends_h::vorbis_info_floor0;
    if !info.is_null() {
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::backends_h::vorbis_info_floor0>(),
        );
        crate::stdlib::free(info as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor0_free_look(mut i: *mut libc::c_void) {
    let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
    if !look.is_null() {
        if !(*look).linearmap.is_null() {
            if !(*(*look).linearmap.offset(0)).is_null() {
                crate::stdlib::free(*(*look).linearmap.offset(0isize) as *mut libc::c_void);
            }
            if !(*(*look).linearmap.offset(1)).is_null() {
                crate::stdlib::free(*(*look).linearmap.offset(1isize) as *mut libc::c_void);
            }
            crate::stdlib::free((*look).linearmap as *mut libc::c_void);
        }
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0,
            ::std::mem::size_of::<vorbis_look_floor0>(),
        );
        crate::stdlib::free(look as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor0_unpack(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut j: i32 = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor0 =
        crate::stdlib::malloc(::std::mem::size_of::<crate::backends_h::vorbis_info_floor0>())
            as *mut crate::backends_h::vorbis_info_floor0;
    (*info).order = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) as i32;
    (*info).rate = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 16);
    (*info).barkmap = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 16);
    (*info).ampbits = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 6) as i32;
    (*info).ampdB = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) as i32;
    (*info).numbooks = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4) + 1) as i32;
    if !((*info).order < 1) {
        if !((*info).rate < 1) {
            if !((*info).barkmap < 1) {
                if !((*info).numbooks < 1) {
                    j = 0;
                    loop {
                        if !(j < (*info).numbooks) {
                            current_block = 5948590327928692120;
                            break;
                        }
                        (*info).books[j as usize] =
                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) as i32;
                        if (*info).books[j as usize] < 0 || (*info).books[j as usize] >= (*ci).books
                        {
                            current_block = 8027669132317143458;
                            break;
                        }
                        if (*(*ci).book_param[(*info).books[j as usize] as usize]).maptype == 0 {
                            current_block = 8027669132317143458;
                            break;
                        }
                        if (*(*ci).book_param[(*info).books[j as usize] as usize]).dim < 1 {
                            current_block = 8027669132317143458;
                            break;
                        }
                        j += 1
                    }
                    match current_block {
                        8027669132317143458 => {}
                        _ => return info as *mut libc::c_void,
                    }
                }
            }
        }
    }
    floor0_free_info(info as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
/* initialize Bark scale and normalization lookups.  We could do this
with static tables, but Vorbis allows a number of possible
combinations, so it's best to do it computationally.

The below is authoritative in terms of defining scale mapping.
Note that the scale depends on the sampling rate as well as the
linear block and mapping sizes */

unsafe extern "C" fn floor0_map_lazy_init(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut infoX: *mut libc::c_void,
    mut look: *mut vorbis_look_floor0,
) {
    if (*(*look).linearmap.offset((*vb).W)).is_null() {
        let mut vd: *mut crate::codec_h::vorbis_dsp_state = (*vb).vd;
        let mut vi: *mut crate::codec_h::vorbis_info = (*vd).vi;
        let mut ci: *mut crate::codec_internal_h::codec_setup_info =
            (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
        let mut info: *mut crate::backends_h::vorbis_info_floor0 =
            infoX as *mut crate::backends_h::vorbis_info_floor0;
        let mut W: i32 = (*vb).W as i32;
        let mut n: i32 = ((*ci).blocksizes[W as usize] / 2) as i32;
        let mut j: i32 = 0;
        /* we choose a scaling constant so that:
          floor(bark(rate/2-1)*C)=mapped-1
        floor(bark(rate/2)*C)=mapped */
        let mut scale: f32 = ((*look).ln as f64
            / (13.1f32 as f64
                * crate::stdlib::atan((0.00074 * ((*info).rate as f32 / 2.0)) as f64)
                + 2.24f32 as f64
                    * crate::stdlib::atan(
                        ((*info).rate as f32 / 2.0 * ((*info).rate as f32 / 2.0) * 1.85e-8) as f64,
                    )
                + (1e-4 * ((*info).rate as f32 / 2.0)) as f64)) as f32;
        /* the mapping from a linear scale to a smaller bark scale is
        straightforward.  We do *not* make sure that the linear mapping
        does not skip bark-scale bins; the decoder simply skips them and
        the encoder may do what it wishes in filling them.  They're
        necessary in some mapping combinations to keep the scale spacing
        accurate */
        let ref mut fresh0 = *(*look).linearmap.offset(W as isize); /* bark numbers represent band edges */
        *fresh0 =
            crate::stdlib::malloc(((n + 1) as usize).wrapping_mul(::std::mem::size_of::<i32>()))
                as *mut i32; /* guard against the approximation */
        j = 0;
        while j < n {
            let mut val: i32 = crate::stdlib::floor(
                (13.1f32 as f64
                    * crate::stdlib::atan(
                        (0.00074 * ((*info).rate as f32 / 2.0 / n as f32 * j as f32)) as f64,
                    )
                    + 2.24f32 as f64
                        * crate::stdlib::atan(
                            ((*info).rate as f32 / 2.0 / n as f32
                                * j as f32
                                * ((*info).rate as f32 / 2.0 / n as f32 * j as f32)
                                * 1.85e-8) as f64,
                        )
                    + (1e-4 * ((*info).rate as f32 / 2.0 / n as f32 * j as f32)) as f64)
                    * scale as f64,
            ) as i32;
            if val >= (*look).ln {
                val = (*look).ln - 1
            }
            *(*(*look).linearmap.offset(W as isize)).offset(j as isize) = val;
            j += 1
        }
        *(*(*look).linearmap.offset(W as isize)).offset(j as isize) = -(1);
        (*look).n[W as usize] = n
    };
}

unsafe extern "C" fn floor0_look(
    mut _vd: *mut crate::codec_h::vorbis_dsp_state,
    mut i: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut info: *mut crate::backends_h::vorbis_info_floor0 =
        i as *mut crate::backends_h::vorbis_info_floor0;
    let mut look: *mut vorbis_look_floor0 =
        crate::stdlib::calloc(1, ::std::mem::size_of::<vorbis_look_floor0>())
            as *mut vorbis_look_floor0;
    (*look).m = (*info).order;
    (*look).ln = (*info).barkmap as i32;
    (*look).vi = info;
    (*look).linearmap =
        crate::stdlib::calloc(2, ::std::mem::size_of::<*mut i32>()) as *mut *mut i32;
    return look as *mut libc::c_void;
}

unsafe extern "C" fn floor0_inverse1(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut i: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
    let mut info: *mut crate::backends_h::vorbis_info_floor0 = (*look).vi;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ampraw: i32 =
        crate::src::libogg_1_3_3::src::bitwise::oggpack_read(&mut (*vb).opb, (*info).ampbits)
            as i32;
    if ampraw > 0 {
        /* also handles the -1 out of data case */
        let mut maxval: isize = (((1i32) << (*info).ampbits) - 1) as isize;
        let mut amp: f32 = ampraw as f32 / maxval as f32 * (*info).ampdB as f32;
        let mut booknum: i32 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            &mut (*vb).opb,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                (*info).numbooks as crate::config_types_h::ogg_uint32_t,
            ),
        ) as i32;
        if booknum != -(1) && booknum < (*info).numbooks {
            /* be paranoid */
            let mut ci: *mut crate::codec_internal_h::codec_setup_info =
                (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
            let mut b: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook = (*ci)
                .fullbooks
                .offset((*info).books[booknum as usize] as isize);
            let mut last: f32 = 0.0;
            /* the additional b->dim is a guard against any possible stack
            smash; b->dim is provably more than we can overflow the
            vector */
            let mut lsp: *mut f32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                vb,
                (::std::mem::size_of::<f32>())
                    .wrapping_mul(((*look).m as isize + (*b).dim + 1) as usize)
                    as isize,
            ) as *mut f32;
            if !(crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodev_set(
                b,
                lsp,
                &mut (*vb).opb,
                (*look).m,
            ) == -1)
            {
                j = 0;
                while j < (*look).m {
                    k = 0;
                    while j < (*look).m && (k as isize) < (*b).dim {
                        *lsp.offset(j as isize) += last;
                        k += 1;
                        j += 1
                    }
                    last = *lsp.offset((j - 1) as isize)
                }
                *lsp.offset((*look).m as isize) = amp;
                return lsp as *mut libc::c_void;
            }
        }
    }
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn floor0_inverse2(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut i: *mut libc::c_void,
    mut memo: *mut libc::c_void,
    mut out: *mut f32,
) -> i32 {
    let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
    let mut info: *mut crate::backends_h::vorbis_info_floor0 = (*look).vi;
    floor0_map_lazy_init(vb, info as *mut libc::c_void, look);
    if !memo.is_null() {
        let mut lsp: *mut f32 = memo as *mut f32;
        let mut amp: f32 = *lsp.offset((*look).m as isize);
        /* take the coefficients back to a spectral envelope curve */
        crate::src::libvorbis_1_3_6::lib::lsp::vorbis_lsp_to_curve(
            out,
            *(*look).linearmap.offset((*vb).W),
            (*look).n[(*vb).W as usize],
            (*look).ln,
            lsp,
            (*look).m,
            amp,
            (*info).ampdB as f32,
        );
        return 1i32;
    }
    crate::stdlib::memset(
        out as *mut libc::c_void,
        0,
        (::std::mem::size_of::<f32>()).wrapping_mul((*look).n[(*vb).W as usize] as usize),
    );
    return 0;
}
/* export hooks */
#[no_mangle]

pub static mut floor0_exportbundle: crate::backends_h::vorbis_func_floor = {
    let mut init = crate::backends_h::vorbis_func_floor {
        pack: None,
        unpack: Some(
            floor0_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            floor0_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(floor0_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(floor0_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        inverse1: Some(
            floor0_inverse1
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        inverse2: Some(
            floor0_inverse2
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut f32,
                ) -> i32,
        ),
    };
    init
};
