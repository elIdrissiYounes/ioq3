// =============== BEGIN envelope_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_lookup {
    pub ch: i32,
    pub winlength: i32,
    pub searchstep: i32,
    pub minenergy: f32,
    pub mdct: crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    pub mdct_win: *mut f32,
    pub band: [crate::src::libvorbis_1_3_6::lib::envelope::envelope_band; 7],
    pub filter: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state,
    pub stretch: i32,
    pub mark: *mut i32,
    pub storage: isize,
    pub current: isize,
    pub curmark: isize,
    pub cursor: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_filter_state {
    pub ampbuf: [f32; 17],
    pub ampptr: i32,
    pub nearDC: [f32; 15],
    pub nearDC_acc: f32,
    pub nearDC_partialacc: f32,
    pub nearptr: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct envelope_band {
    pub begin: i32,
    pub end: i32,
    pub window: *mut f32,
    pub total: f32,
}
use ::libc;

pub mod scales_h {

    /* Segher was off (too high) by ~ .3 decibel.  Center the conversion correctly. */
    #[inline]

    pub unsafe extern "C" fn todB(mut x: *const f32) -> f32 {
        let mut ix: crate::scales_h::C2RustUnnamed_58 = crate::scales_h::C2RustUnnamed_58 { i: 0 };
        ix.f = *x;
        ix.i = ix.i & 0x7fffffff;
        return ix.i as f32 * 7.17711438e-7 - 764.6161886;
    }

    /* Frequency to octave.  We arbitrarily declare 63.5 Hz to be octave
    0.0 */
    /* The bark scale equations are approximations, since the original
    table was somewhat hand rolled.  The below are chosen to have the
    best possible fit to the rolled tables, thus their somewhat odd
    appearance (these are more accurate and over a longer range than
    the oft-quoted bark equations found in the texts I have).  The
    approximations are valid from 0 - 30kHz (nyquist) or so.

    all f in Hz, z in Bark */
}

pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

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
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_forward;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_init;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

pub use crate::scales_h::C2RustUnnamed_58;
pub use crate::src::libvorbis_1_3_6::lib::envelope::scales_h::todB;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: PCM data envelope analysis

********************************************************************/
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_init(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut vi: *mut crate::codec_h::vorbis_info,
) {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info; /* not random */
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut ch: i32 = (*vi).channels;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    (*e).winlength = 128;
    let mut n: i32 = (*e).winlength;
    (*e).searchstep = 64;
    (*e).minenergy = (*gi).preecho_minenergy;
    (*e).ch = ch;
    (*e).storage = 128isize;
    (*e).cursor = (*ci).blocksizes[1] / 2;
    (*e).mdct_win = crate::stdlib::calloc(n as usize, ::std::mem::size_of::<f32>()) as *mut f32;
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_init(&mut (*e).mdct, n);
    i = 0;
    while i < n {
        *(*e).mdct_win.offset(i as isize) =
            crate::stdlib::sin(i as f64 / (n as f64 - 1.0) * 3.14159265358979323846) as f32;
        *(*e).mdct_win.offset(i as isize) *= *(*e).mdct_win.offset(i as isize);
        i += 1
    }
    /* magic follows */
    (*e).band[0].begin = 2;
    (*e).band[0].end = 4;
    (*e).band[1].begin = 4;
    (*e).band[1].end = 5;
    (*e).band[2].begin = 6;
    (*e).band[2].end = 6;
    (*e).band[3].begin = 9;
    (*e).band[3].end = 8;
    (*e).band[4].begin = 13;
    (*e).band[4].end = 8;
    (*e).band[5].begin = 17;
    (*e).band[5].end = 8;
    (*e).band[6].begin = 22;
    (*e).band[6].end = 8;

    for j in 0..7 {
        n = (*e).band[j as usize].end;

        (*e).band[j as usize].window =
            crate::stdlib::malloc((n as usize).wrapping_mul(::std::mem::size_of::<f32>()))
                as *mut f32;

        i = 0;

        while i < n {
            *(*e).band[j as usize].window.offset(i as isize) =
                crate::stdlib::sin((i as f64 + 0.5) / n as f64 * 3.14159265358979323846) as f32;
            (*e).band[j as usize].total += *(*e).band[j as usize].window.offset(i as isize);
            i += 1
        }

        (*e).band[j as usize].total = (1.0 / (*e).band[j as usize].total as f64) as f32;
    }
    (*e).filter = crate::stdlib::calloc(
        (7 * ch) as usize,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state>(),
    ) as *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
    (*e).mark =
        crate::stdlib::calloc((*e).storage as usize, ::std::mem::size_of::<i32>()) as *mut i32;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_clear(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
) {
    let mut _i: i32 = 0;
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_clear(&mut (*e).mdct);

    for i in 0..7 {
        crate::stdlib::free((*e).band[i as usize].window as *mut libc::c_void);
    }
    crate::stdlib::free((*e).mdct_win as *mut libc::c_void);
    crate::stdlib::free((*e).filter as *mut libc::c_void);
    crate::stdlib::free((*e).mark as *mut libc::c_void);
    crate::stdlib::memset(
        e as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup>(),
    );
}
/* fairly straight threshhold-by-band based until we find something
that works better and isn't patented. */

unsafe extern "C" fn _ve_amp(
    mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    mut data: *mut f32,
    mut bands: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_band,
    mut filters: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state,
) -> i32 {
    let mut n: isize = (*ve).winlength as isize;
    let mut ret: i32 = 0;
    let mut i: isize = 0;
    let mut _j: isize = 0;
    let mut decay: f32 = 0.;
    /* we want to have a 'minimum bar' for energy, else we're just
    basing blocks on quantization noise that outweighs the signal
    itself (for low power signals) */
    let mut minV: f32 = (*ve).minenergy;
    let mut fresh0 =
        ::std::vec::from_elem(0, (n as usize).wrapping_mul(::std::mem::size_of::<f32>()));
    let mut vec: *mut f32 = fresh0.as_mut_ptr() as *mut f32;
    /* stretch is used to gradually lengthen the number of windows
    considered prevoius-to-potential-trigger */
    let mut stretch: i32 = if (2) < (*ve).stretch / 2 {
        ((*ve).stretch) / 2
    } else {
        2
    };
    let mut penalty: f32 = (*gi).stretch_penalty - ((*ve).stretch / 2i32 - 2) as f32;
    if penalty < 0.0 {
        penalty = 0.0
    }
    if penalty > (*gi).stretch_penalty {
        penalty = (*gi).stretch_penalty
    }
    /*_analysis_output_always("lpcm",seq2,data,n,0,0,
    totalshift+pos*ve->searchstep);*/
    /* window and transform */
    i = 0;
    while i < n {
        *vec.offset(i) = *data.offset(i) * *(*ve).mdct_win.offset(i);
        i += 1
    }
    crate::src::libvorbis_1_3_6::lib::mdct::mdct_forward(&mut (*ve).mdct, vec, vec);
    /*_analysis_output_always("mdct",seq2,vec,n/2,0,1,0); */
    /* near-DC spreading function; this has nothing to do with
    psychoacoustics, just sidelobe leakage and window size */
    let mut temp: f32 = ((*vec.offset(0) * *vec.offset(0)) as f64
        + 0.7 * *vec.offset(1) as f64 * *vec.offset(1) as f64
        + 0.2 * *vec.offset(2) as f64 * *vec.offset(2) as f64) as f32;
    let mut ptr: i32 = (*filters).nearptr;
    /* the accumulation is regularly refreshed from scratch to avoid
    floating point creep */
    if ptr == 0 {
        (*filters).nearDC_acc = (*filters).nearDC_partialacc + temp;
        decay = (*filters).nearDC_acc;
        (*filters).nearDC_partialacc = temp
    } else {
        (*filters).nearDC_acc += temp;
        decay = (*filters).nearDC_acc;
        (*filters).nearDC_partialacc += temp
    }
    (*filters).nearDC_acc -= (*filters).nearDC[ptr as usize];
    (*filters).nearDC[ptr as usize] = temp;
    decay = (decay as f64 * (1.0 / (15i32 + 1) as f64)) as f32;
    (*filters).nearptr += 1;
    if (*filters).nearptr >= 15 {
        (*filters).nearptr = 0
    }
    decay = (todB(&mut decay) as f64 * 0.5 - 15f64) as f32;
    /* perform spreading and limiting, also smooth the spectrum.  yes,
    the MDCT results in all real coefficients, but it still *behaves*
    like real/imaginary pairs */
    i = 0;
    while i < n / 2 {
        let mut val: f32 =
            *vec.offset(i) * *vec.offset(i) + *vec.offset(i + 1) * *vec.offset(i + 1);
        val = todB(&mut val) * 0.5;
        if val < decay {
            val = decay
        }
        if val < minV {
            val = minV
        }
        *vec.offset(i >> 1) = val;
        decay = (decay as f64 - 8.0) as f32;
        i += 2
    }
    /*_analysis_output_always("spread",seq2++,vec,n/4,0,0,0);*/
    /* perform preecho/postecho triggering by band */

    for j in 0..7 {
        let mut acc: f32 = 0f32;

        let mut valmax: f32 = 0.;

        let mut valmin: f32 = 0.;

        i = 0;

        while i < (*bands.offset(j)).end as isize {
            acc += *vec.offset(i + (*bands.offset(j)).begin as isize)
                * *(*bands.offset(j)).window.offset(i);
            i += 1
        }

        acc *= (*bands.offset(j)).total;

        let mut p: i32 = 0;

        let mut this: i32 = (*filters.offset(j)).ampptr;

        let mut postmax: f32 = 0.;

        let mut postmin: f32 = 0.;

        let mut premax: f32 = -99999.0;

        let mut premin: f32 = 99999.0;

        p = this;

        p -= 1;

        if p < 0 {
            p += 16 + 2 - 1
        }

        postmax = if acc < (*filters.offset(j)).ampbuf[p as usize] {
            (*filters.offset(j)).ampbuf[p as usize]
        } else {
            acc
        };

        postmin = if acc > (*filters.offset(j)).ampbuf[p as usize] {
            (*filters.offset(j)).ampbuf[p as usize]
        } else {
            acc
        };

        i = 0;

        while i < stretch as isize {
            p -= 1;
            if p < 0 {
                p += 16 + 2 - 1
            }
            premax = if premax < (*filters.offset(j)).ampbuf[p as usize] {
                (*filters.offset(j)).ampbuf[p as usize]
            } else {
                premax
            };
            premin = if premin > (*filters.offset(j)).ampbuf[p as usize] {
                (*filters.offset(j)).ampbuf[p as usize]
            } else {
                premin
            };
            i += 1
        }

        valmin = postmin - premin;

        valmax = postmax - premax;

        (*filters.offset(j)).ampbuf[this as usize] = acc;

        let ref mut fresh1 = (*filters.offset(j)).ampptr;

        *fresh1 += 1;

        if (*filters.offset(j)).ampptr >= 16 + 2 - 1 {
            (*filters.offset(j)).ampptr = 0
        }

        if valmax > (*gi).preecho_thresh[j as usize] + penalty {
            ret |= 1;
            ret |= 4
        }

        if valmin < (*gi).postecho_thresh[j as usize] - penalty {
            ret |= 2
        }
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_search(
    mut v: *mut crate::codec_h::vorbis_dsp_state,
) -> isize {
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup =
        (*((*v).backend_state as *mut crate::codec_internal_h::private_state)).ve;
    let mut _i: isize = 0;
    let mut j: isize = 0;
    let mut first: i32 = ((*ve).current / (*ve).searchstep as isize) as i32;
    let mut last: i32 = (*v).pcm_current / (*ve).searchstep - 4;
    if first < 0 {
        first = 0
    }
    /* make sure we have enough storage to match the PCM */
    if (last + 4 + 2) as isize > (*ve).storage {
        (*ve).storage = (last + 4 + 2) as isize; /* be sure */
        (*ve).mark = crate::stdlib::realloc(
            (*ve).mark as *mut libc::c_void,
            ((*ve).storage as usize).wrapping_mul(::std::mem::size_of::<i32>()),
        ) as *mut i32
    }
    j = first as isize;
    while j < last as isize {
        let mut ret: i32 = 0;
        (*ve).stretch += 1;
        if (*ve).stretch > 12 * 2 {
            (*ve).stretch = 12 * 2
        }

        for i in 0..(*ve).ch as isize {
            let mut pcm: *mut f32 = (*(*v).pcm.offset(i)).offset((*ve).searchstep as isize * j);

            ret |= _ve_amp(
                ve,
                gi,
                pcm,
                (*ve).band.as_mut_ptr(),
                (*ve).filter.offset(i * 7),
            );
        }
        *(*ve).mark.offset(j + 2) = 0;
        if ret & 1 != 0 {
            *(*ve).mark.offset(j) = 1;
            *(*ve).mark.offset(j + 1) = 1
        }
        if ret & 2 != 0 {
            *(*ve).mark.offset(j) = 1;
            if j > 0 {
                *(*ve).mark.offset(j - 1) = 1
            }
        }
        if ret & 4 != 0 {
            (*ve).stretch = -(1)
        }
        j += 1
    }
    (*ve).current = (last * (*ve).searchstep) as isize;
    let mut centerW: isize = (*v).centerW;
    let mut testW: isize = centerW
        + (*ci).blocksizes[(*v).W as usize] / 4
        + (*ci).blocksizes[1] / 2
        + (*ci).blocksizes[0] / 4;
    j = (*ve).cursor;
    while j < (*ve).current - (*ve).searchstep as isize {
        /* account for postecho
        working back one window */
        if j >= testW {
            return 1isize;
        }
        (*ve).cursor = j;
        if *(*ve).mark.offset(j / (*ve).searchstep as isize) != 0 {
            if j > centerW {
                (*ve).curmark = j;
                if j >= testW {
                    return 1isize;
                }
                return 0isize;
            }
        }
        j += (*ve).searchstep as isize
    }
    return -1isize;
}
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_mark(mut v: *mut crate::codec_h::vorbis_dsp_state) -> i32 {
    let mut ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup =
        (*((*v).backend_state as *mut crate::codec_internal_h::private_state)).ve;
    let mut vi: *mut crate::codec_h::vorbis_info = (*v).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut centerW: isize = (*v).centerW;
    let mut beginW: isize = centerW - (*ci).blocksizes[(*v).W as usize] / 4;
    let mut endW: isize = centerW + (*ci).blocksizes[(*v).W as usize] / 4;
    if (*v).W != 0 {
        beginW -= (*ci).blocksizes[(*v).lW as usize] / 4;
        endW += (*ci).blocksizes[(*v).nW as usize] / 4
    } else {
        beginW -= (*ci).blocksizes[0] / 4;
        endW += (*ci).blocksizes[0] / 4
    }
    if (*ve).curmark >= beginW && (*ve).curmark < endW {
        return 1i32;
    }
    let mut first: isize = beginW / (*ve).searchstep as isize;
    let mut last: isize = endW / (*ve).searchstep as isize;
    let mut _i: isize = 0;

    for i in first..last {
        if *(*ve).mark.offset(i) != 0 {
            return 1i32;
        }
    }
    return 0;
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: PCM data envelope analysis and manipulation

********************************************************************/
/* a bit less than short block */
/* one-third full block */
#[no_mangle]

pub unsafe extern "C" fn _ve_envelope_shift(
    mut e: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
    mut shift: isize,
) {
    let mut smallsize: i32 = ((*e).current / (*e).searchstep as isize + 2) as i32; /* adjust for placing marks
                                                                                   ahead of ve->current */
    let mut smallshift: i32 = (shift / (*e).searchstep as isize) as i32;
    crate::stdlib::memmove(
        (*e).mark as *mut libc::c_void,
        (*e).mark.offset(smallshift as isize) as *const libc::c_void,
        ((smallsize - smallshift) as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    (*e).current -= shift;
    if (*e).curmark >= 0isize {
        (*e).curmark -= shift
    }
    (*e).cursor -= shift;
}
