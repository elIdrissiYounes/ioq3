use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::abs;
pub use crate::stdlib::calloc;
pub use crate::stdlib::exit;
pub use crate::stdlib::free;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_info_floor1;
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
pub use crate::codec_internal_h::vorbis_look_floor1;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
use crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
use crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog;
use crate::stdlib::memset;
use crate::stdlib::rint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsfit_acc {
    pub x0: i32,
    pub x1: i32,
    pub xa: i32,
    pub ya: i32,
    pub x2a: i32,
    pub y2a: i32,
    pub xya: i32,
    pub an: i32,
    pub xb: i32,
    pub yb: i32,
    pub x2b: i32,
    pub y2b: i32,
    pub xyb: i32,
    pub bn: i32,
}
/* **********************************************/

unsafe extern "C" fn floor1_free_info(mut i: *mut libc::c_void) {
    let mut info: *mut crate::backends_h::vorbis_info_floor1 =
        i as *mut crate::backends_h::vorbis_info_floor1;
    if !info.is_null() {
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::backends_h::vorbis_info_floor1>(),
        );
        crate::stdlib::free(info as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor1_free_look(mut i: *mut libc::c_void) {
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 =
        i as *mut crate::codec_internal_h::vorbis_look_floor1;
    if !look.is_null() {
        /*fprintf(stderr,"floor 1 bit usage %f:%f (%f total)\n",
        (float)look->phrasebits/look->frames,
        (float)look->postbits/look->frames,
        (float)(look->postbits+look->phrasebits)/look->frames);*/
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::codec_internal_h::vorbis_look_floor1>(),
        );
        crate::stdlib::free(look as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor1_pack(
    mut i: *mut libc::c_void,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) {
    let mut info: *mut crate::backends_h::vorbis_info_floor1 =
        i as *mut crate::backends_h::vorbis_info_floor1;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut count: i32 = 0;
    let mut rangebits: i32 = 0;
    let mut maxposit: i32 = (*info).postlist[1];
    let mut maxclass: i32 = -(1);
    /* save out partitions */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*info).partitions as usize, 5); /* only 0 to 31 legal */
    j = 0; /* only 0 to 15 legal */
    while j < (*info).partitions {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            (*info).partitionclass[j as usize] as usize,
            4,
        );
        if maxclass < (*info).partitionclass[j as usize] {
            maxclass = (*info).partitionclass[j as usize]
        }
        j += 1
    }
    /* save out partition classes */
    j = 0; /* 1 to 8 */
    while j < maxclass + 1 {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            ((*info).class_dim[j as usize] - 1) as usize,
            3,
        ); /* 0 to 3 */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            (*info).class_subs[j as usize] as usize,
            2,
        );
        if (*info).class_subs[j as usize] != 0 {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb,
                (*info).class_book[j as usize] as usize,
                8i32,
            );
        }
        k = 0;
        while k < (1) << (*info).class_subs[j as usize] {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb,
                ((*info).class_subbook[j as usize][k as usize] + 1) as usize,
                8,
            );
            k += 1
        }
        j += 1
    }
    /* save out the post list */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, ((*info).mult - 1) as usize, 2); /* only 1,2,3,4 legal now */
    /* maxposit cannot legally be less than 1; this is encode-side, we
    can assume our setup is OK */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            (maxposit - 1) as crate::config_types_h::ogg_uint32_t,
        ) as usize,
        4,
    );
    rangebits = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
        (maxposit - 1) as crate::config_types_h::ogg_uint32_t,
    );
    j = 0;
    k = 0;
    while j < (*info).partitions {
        count += (*info).class_dim[(*info).partitionclass[j as usize] as usize];
        while k < count {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb,
                (*info).postlist[(k + 2) as usize] as usize,
                rangebits,
            );
            k += 1
        }
        j += 1
    }
}

unsafe extern "C" fn icomp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    return **(a as *mut *mut i32) - **(b as *mut *mut i32);
}

unsafe extern "C" fn floor1_unpack(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut count: i32 = 0;
    let mut maxclass: i32 = -(1);
    let mut rangebits: i32 = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::backends_h::vorbis_info_floor1>(),
    )
        as *mut crate::backends_h::vorbis_info_floor1;
    /* read partitions */
    (*info).partitions = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 5) as i32; /* only 0 to 31 legal */
    j = 0; /* only 0 to 15 legal */
    loop {
        if !(j < (*info).partitions) {
            current_block = 13183875560443969876;
            break;
        }
        (*info).partitionclass[j as usize] =
            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4) as i32;
        if (*info).partitionclass[j as usize] < 0 {
            current_block = 4682380797242156875;
            break;
        }
        if maxclass < (*info).partitionclass[j as usize] {
            maxclass = (*info).partitionclass[j as usize]
        }
        j += 1
    }
    match current_block {
        13183875560443969876 =>
        /* read partition classes */
        {
            j = 0; /* 1 to 8 */
            's_49: loop {
                if !(j < maxclass + 1) {
                    current_block = 18317007320854588510; /* 0,1,2,3 bits */
                    break;
                }
                (*info).class_dim[j as usize] =
                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 3) + 1) as i32;
                (*info).class_subs[j as usize] =
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 2) as i32;
                if (*info).class_subs[j as usize] < 0 {
                    current_block = 4682380797242156875;
                    break;
                }
                if (*info).class_subs[j as usize] != 0 {
                    (*info).class_book[j as usize] =
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) as i32
                }
                if (*info).class_book[j as usize] < 0
                    || (*info).class_book[j as usize] >= (*ci).books
                {
                    current_block = 4682380797242156875;
                    break;
                }
                k = 0;
                while k < (1) << (*info).class_subs[j as usize] {
                    (*info).class_subbook[j as usize][k as usize] =
                        (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 8) - 1) as i32;
                    if (*info).class_subbook[j as usize][k as usize] < -(1)
                        || (*info).class_subbook[j as usize][k as usize] >= (*ci).books
                    {
                        current_block = 4682380797242156875;
                        break 's_49;
                    }
                    k += 1
                }
                j += 1
            }
            match current_block {
                4682380797242156875 => {}
                _ => {
                    /* read the post list */
                    (*info).mult =
                        (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 2) + 1) as i32; /* only 1,2,3,4 legal now */
                    rangebits = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4) as i32;
                    if !(rangebits < 0) {
                        j = 0;
                        k = 0;
                        's_130: loop {
                            if !(j < (*info).partitions) {
                                current_block = 14434620278749266018;
                                break;
                            }
                            count += (*info).class_dim[(*info).partitionclass[j as usize] as usize];
                            if count > 63 {
                                current_block = 4682380797242156875;
                                break;
                            }
                            while k < count {
                                (*info).postlist[(k + 2) as usize] =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                        opb, rangebits,
                                    ) as i32;
                                let mut t: i32 = (*info).postlist[(k + 2) as usize];
                                if t < 0 || t >= (1) << rangebits {
                                    current_block = 4682380797242156875;
                                    break 's_130;
                                }
                                k += 1
                            }
                            j += 1
                        }
                        match current_block {
                            4682380797242156875 => {}
                            _ => {
                                (*info).postlist[0] = 0;
                                (*info).postlist[1] = (1) << rangebits;
                                /* don't allow repeated values in post list as they'd result in
                                zero-length segments */
                                let mut sortpointer: [*mut i32; 65] = [0 as *mut i32; 65];
                                j = 0;
                                while j < count + 2 {
                                    sortpointer[j as usize] =
                                        (*info).postlist.as_mut_ptr().offset(j as isize);
                                    j += 1
                                }
                                crate::stdlib::qsort(
                                    sortpointer.as_mut_ptr() as *mut libc::c_void,
                                    (count + 2) as crate::stddef_h::size_t,
                                    ::std::mem::size_of::<*mut i32>(),
                                    Some(
                                        icomp
                                            as unsafe extern "C" fn(
                                                _: *const libc::c_void,
                                                _: *const libc::c_void,
                                            )
                                                -> i32,
                                    ),
                                );
                                j = 1;
                                loop {
                                    if !(j < count + 2) {
                                        current_block = 1847472278776910194;
                                        break;
                                    }
                                    if *sortpointer[(j - 1) as usize] == *sortpointer[j as usize] {
                                        current_block = 4682380797242156875;
                                        break;
                                    }
                                    j += 1
                                }
                                match current_block {
                                    4682380797242156875 => {}
                                    _ => return info as *mut libc::c_void,
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    floor1_free_info(info as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn floor1_look(
    mut vd: *mut crate::codec_h::vorbis_dsp_state,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sortpointer: [*mut i32; 65] = [0 as *mut i32; 65];
    let mut info: *mut crate::backends_h::vorbis_info_floor1 =
        in_0 as *mut crate::backends_h::vorbis_info_floor1;
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 = crate::stdlib::calloc(
        1,
        ::std::mem::size_of::<crate::codec_internal_h::vorbis_look_floor1>(),
    )
        as *mut crate::codec_internal_h::vorbis_look_floor1;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    (*look).vi = info;
    (*look).n = (*info).postlist[1];
    /* we drop each position value in-between already decoded values,
    and use linear interpolation to predict each new value past the
    edges.  The positions are read in the order of the position
    list... we precompute the bounding positions in the lookup.  Of
    course, the neighbors can change (if a position is declined), but
    this is an initial mapping */
    i = 0;
    while i < (*info).partitions {
        n += (*info).class_dim[(*info).partitionclass[i as usize] as usize];
        i += 1
    }
    n += 2;
    (*look).posts = n;
    /* also store a sorted position index */
    i = 0;
    while i < n {
        sortpointer[i as usize] = (*info).postlist.as_mut_ptr().offset(i as isize);
        i += 1
    }
    crate::stdlib::qsort(
        sortpointer.as_mut_ptr() as *mut libc::c_void,
        n as crate::stddef_h::size_t,
        ::std::mem::size_of::<*mut i32>(),
        Some(icomp as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32),
    );
    /* points from sort order back to range number */
    i = 0;
    while i < n {
        (*look).forward_index[i as usize] =
            sortpointer[i as usize].wrapping_offset_from((*info).postlist.as_mut_ptr()) as i32;
        i += 1
    }
    /* points from range order to sorted position */
    i = 0;
    while i < n {
        (*look).reverse_index[(*look).forward_index[i as usize] as usize] = i;
        i += 1
    }
    /* we actually need the post values too */
    i = 0;
    while i < n {
        (*look).sorted_index[i as usize] =
            (*info).postlist[(*look).forward_index[i as usize] as usize];
        i += 1
    }
    /* quantize values to multiplier spec */
    match (*info).mult {
        1 => {
            /* 1024 -> 256 */
            (*look).quant_q = 256
        }
        2 => {
            /* 1024 -> 128 */
            (*look).quant_q = 128
        }
        3 => {
            /* 1024 -> 86 */
            (*look).quant_q = 86
        }
        4 => {
            /* 1024 -> 64 */
            (*look).quant_q = 64
        }
        _ => {}
    }
    /* discover our neighbors for decode where we don't use fit flags
    (that would push the neighbors outward) */
    i = 0; /* mask off flag */
    while i < n - 2 {
        let mut lo: i32 = 0;
        let mut hi: i32 = 1;
        let mut lx: i32 = 0;
        let mut hx: i32 = (*look).n;
        let mut currentx: i32 = (*info).postlist[(i + 2) as usize];
        j = 0;
        while j < i + 2 {
            let mut x: i32 = (*info).postlist[j as usize];
            if x > lx && x < currentx {
                lo = j;
                lx = x
            }
            if x < hx && x > currentx {
                hi = j;
                hx = x
            }
            j += 1
        }
        (*look).loneighbor[i as usize] = lo;
        (*look).hineighbor[i as usize] = hi;
        i += 1
    }
    return look as *mut libc::c_void;
}

unsafe extern "C" fn render_point(
    mut x0: i32,
    mut x1: i32,
    mut y0: i32,
    mut y1: i32,
    mut x: i32,
) -> i32 {
    y0 &= 0x7fff;
    y1 &= 0x7fff;
    let mut dy: i32 = y1 - y0;
    let mut adx: i32 = x1 - x0;
    let mut ady: i32 = crate::stdlib::abs(dy);
    let mut err: i32 = ady * (x - x0);
    let mut off: i32 = err / adx;
    if dy < 0 {
        return y0 - off;
    }
    return y0 + off;
}

unsafe extern "C" fn vorbis_dBquant(mut x: *const f32) -> i32 {
    let mut i: i32 = (*x * 7.3142857f32 + 1023.5) as i32;
    if i > 1023 {
        return 1023i32;
    }
    if i < 0 {
        return 0i32;
    }
    return i;
}

static mut FLOOR1_fromdB_LOOKUP: [f32; 256] = [
    1.0649863e-07,
    1.1341951e-07,
    1.2079015e-07,
    1.2863978e-07,
    1.3699951e-07,
    1.4590251e-07,
    1.5538408e-07,
    1.6548181e-07,
    1.7623575e-07,
    1.8768855e-07,
    1.9988561e-07,
    2.128753e-07,
    2.2670913e-07,
    2.4144197e-07,
    2.5713223e-07,
    2.7384213e-07,
    2.9163793e-07,
    3.1059021e-07,
    3.3077411e-07,
    3.5226968e-07,
    3.7516214e-07,
    3.9954229e-07,
    4.2550680e-07,
    4.5315863e-07,
    4.8260743e-07,
    5.1396998e-07,
    5.4737065e-07,
    5.8294187e-07,
    6.2082472e-07,
    6.6116941e-07,
    7.0413592e-07,
    7.4989464e-07,
    7.9862701e-07,
    8.5052630e-07,
    9.0579828e-07,
    9.6466216e-07,
    1.0273513e-06,
    1.0941144e-06,
    1.1652161e-06,
    1.2409384e-06,
    1.3215816e-06,
    1.4074654e-06,
    1.4989305e-06,
    1.5963394e-06,
    1.7000785e-06,
    1.8105592e-06,
    1.9282195e-06,
    2.0535261e-06,
    2.1869758e-06,
    2.3290978e-06,
    2.4804557e-06,
    2.6416497e-06,
    2.8133190e-06,
    2.9961443e-06,
    3.1908506e-06,
    3.3982101e-06,
    3.6190449e-06,
    3.8542308e-06,
    4.1047004e-06,
    4.3714470e-06,
    4.6555282e-06,
    4.9580707e-06,
    5.2802740e-06,
    5.6234160e-06,
    5.9888572e-06,
    6.3780469e-06,
    6.7925283e-06,
    7.2339451e-06,
    7.7040476e-06,
    8.2047000e-06,
    8.7378876e-06,
    9.3057248e-06,
    9.9104632e-06,
    1.0554501e-05,
    1.1240392e-05,
    1.1970856e-05,
    1.2748789e-05,
    1.3577278e-05,
    1.4459606e-05,
    1.5399272e-05,
    1.6400004e-05,
    1.7465768e-05,
    1.8600792e-05,
    1.9809576e-05,
    2.1096914e-05,
    2.2467911e-05,
    2.3928002e-05,
    2.5482978e-05,
    2.7139006e-05,
    2.8902651e-05,
    3.0780908e-05,
    3.2781225e-05,
    3.4911534e-05,
    3.7180282e-05,
    3.9596466e-05,
    4.2169667e-05,
    4.4910090e-05,
    4.7828601e-05,
    5.0936773e-05,
    5.4246931e-05,
    5.7772202e-05,
    6.1526565e-05,
    6.5524908e-05,
    6.9783085e-05,
    7.4317983e-05,
    7.9147585e-05,
    8.4291040e-05,
    8.9768747e-05,
    9.5602426e-05,
    0.00010181521,
    0.00010843174,
    0.00011547824,
    0.00012298267,
    0.00013097477,
    0.00013948625,
    0.00014855085,
    0.00015820453,
    0.00016848555,
    0.00017943469,
    0.00019109536,
    0.00020351382,
    0.00021673929,
    0.00023082423,
    0.00024582449,
    0.00026179955,
    0.00027881276,
    0.00029693158,
    0.00031622787,
    0.00033677814,
    0.00035866388,
    0.00038197188,
    0.00040679456,
    0.00043323036,
    0.00046138411,
    0.00049136745,
    0.00052329927,
    0.00055730621,
    0.00059352311,
    0.00063209358,
    0.00067317058,
    0.00071691700,
    0.00076350630,
    0.00081312324,
    0.00086596457,
    0.00092223983,
    0.00098217216,
    0.0010459992,
    0.0011139742,
    0.0011863665,
    0.0012634633,
    0.0013455702,
    0.0014330129,
    0.0015261382,
    0.0016253153,
    0.0017309374,
    0.0018434235,
    0.0019632195,
    0.0020908006,
    0.0022266726,
    0.0023713743,
    0.0025254795,
    0.0026895994,
    0.0028643847,
    0.0030505286,
    0.0032487691,
    0.0034598925,
    0.0036847358,
    0.0039241906,
    0.0041792066,
    0.0044507950,
    0.0047400328,
    0.0050480668,
    0.0053761186,
    0.0057254891,
    0.0060975636,
    0.0064938176,
    0.0069158225,
    0.0073652516,
    0.0078438871,
    0.0083536271,
    0.0088964928,
    0.009474637,
    0.010090352,
    0.010746080,
    0.011444421,
    0.012188144,
    0.012980198,
    0.013823725,
    0.014722068,
    0.015678791,
    0.016697687,
    0.017782797,
    0.018938423,
    0.020169149,
    0.021479854,
    0.022875735,
    0.024362330,
    0.025945531,
    0.027631618,
    0.029427276,
    0.031339626,
    0.033376252,
    0.035545228,
    0.037855157,
    0.040315199,
    0.042935108,
    0.045725273,
    0.048696758,
    0.051861348,
    0.055231591,
    0.058820850,
    0.062643361,
    0.066714279,
    0.071049749,
    0.075666962,
    0.080584227,
    0.085821044,
    0.091398179,
    0.097337747,
    0.10366330,
    0.11039993,
    0.11757434,
    0.12521498,
    0.13335215,
    0.14201813,
    0.15124727,
    0.16107617,
    0.17154380,
    0.18269168,
    0.19456402,
    0.20720788,
    0.22067342,
    0.23501402,
    0.25028656,
    0.26655159,
    0.28387361,
    0.30232132,
    0.32196786,
    0.34289114,
    0.36517414,
    0.38890521,
    0.41417847,
    0.44109412,
    0.46975890,
    0.50028648,
    0.53279791,
    0.56742212,
    0.60429640,
    0.64356699,
    0.68538959,
    0.72993007,
    0.77736504,
    0.82788260,
    0.88168307,
    0.9389798,
    1.0,
];

unsafe extern "C" fn render_line(
    mut n: i32,
    mut x0: i32,
    mut x1: i32,
    mut y0: i32,
    mut y1: i32,
    mut d: *mut f32,
) {
    let mut dy: i32 = y1 - y0;
    let mut adx: i32 = x1 - x0;
    let mut ady: i32 = crate::stdlib::abs(dy);
    let mut base: i32 = dy / adx;
    let mut sy: i32 = if dy < 0 { (base) - 1 } else { (base) + 1 };
    let mut x: i32 = x0;
    let mut y: i32 = y0;
    let mut err: i32 = 0;
    ady -= crate::stdlib::abs(base * adx);
    if n > x1 {
        n = x1
    }
    if x < n {
        *d.offset(x as isize) *= FLOOR1_fromdB_LOOKUP[y as usize]
    }
    loop {
        x += 1;
        if !(x < n) {
            break;
        }
        err = err + ady;
        if err >= adx {
            err -= adx;
            y += sy
        } else {
            y += base
        }
        *d.offset(x as isize) *= FLOOR1_fromdB_LOOKUP[y as usize]
    }
}

unsafe extern "C" fn render_line0(
    mut n: i32,
    mut x0: i32,
    mut x1: i32,
    mut y0: i32,
    mut y1: i32,
    mut d: *mut i32,
) {
    let mut dy: i32 = y1 - y0;
    let mut adx: i32 = x1 - x0;
    let mut ady: i32 = crate::stdlib::abs(dy);
    let mut base: i32 = dy / adx;
    let mut sy: i32 = if dy < 0 { (base) - 1 } else { (base) + 1 };
    let mut x: i32 = x0;
    let mut y: i32 = y0;
    let mut err: i32 = 0;
    ady -= crate::stdlib::abs(base * adx);
    if n > x1 {
        n = x1
    }
    if x < n {
        *d.offset(x as isize) = y
    }
    loop {
        x += 1;
        if !(x < n) {
            break;
        }
        err = err + ady;
        if err >= adx {
            err -= adx;
            y += sy
        } else {
            y += base
        }
        *d.offset(x as isize) = y
    }
}
/* the floor has already been filtered to only include relevant sections */

unsafe extern "C" fn accumulate_fit(
    mut flr: *const f32,
    mut mdct: *const f32,
    mut x0: i32,
    mut x1: i32,
    mut a: *mut lsfit_acc,
    mut n: i32,
    mut info: *mut crate::backends_h::vorbis_info_floor1,
) -> i32 {
    let mut i: isize = 0;
    let mut xa: i32 = 0;
    let mut ya: i32 = 0;
    let mut x2a: i32 = 0;
    let mut y2a: i32 = 0;
    let mut xya: i32 = 0;
    let mut na: i32 = 0;
    let mut xb: i32 = 0;
    let mut yb: i32 = 0;
    let mut x2b: i32 = 0;
    let mut y2b: i32 = 0;
    let mut xyb: i32 = 0;
    let mut nb: i32 = 0;
    crate::stdlib::memset(
        a as *mut libc::c_void,
        0,
        ::std::mem::size_of::<lsfit_acc>(),
    );
    (*a).x0 = x0;
    (*a).x1 = x1;
    if x1 >= n {
        x1 = n - 1
    }
    i = x0 as isize;
    while i <= x1 as isize {
        let mut quantized: i32 = vorbis_dBquant(flr.offset(i));
        if quantized != 0 {
            if *mdct.offset(i) + (*info).twofitatten >= *flr.offset(i) {
                xa = (xa as isize + i) as i32;
                ya += quantized;
                x2a = (x2a as isize + i * i) as i32;
                y2a += quantized * quantized;
                xya = (xya as isize + i * quantized as isize) as i32;
                na += 1
            } else {
                xb = (xb as isize + i) as i32;
                yb += quantized;
                x2b = (x2b as isize + i * i) as i32;
                y2b += quantized * quantized;
                xyb = (xyb as isize + i * quantized as isize) as i32;
                nb += 1
            }
        }
        i += 1
    }
    (*a).xa = xa;
    (*a).ya = ya;
    (*a).x2a = x2a;
    (*a).y2a = y2a;
    (*a).xya = xya;
    (*a).an = na;
    (*a).xb = xb;
    (*a).yb = yb;
    (*a).x2b = x2b;
    (*a).y2b = y2b;
    (*a).xyb = xyb;
    (*a).bn = nb;
    return na;
}

unsafe extern "C" fn fit_line(
    mut a: *mut lsfit_acc,
    mut fits: i32,
    mut y0: *mut i32,
    mut y1: *mut i32,
    mut info: *mut crate::backends_h::vorbis_info_floor1,
) -> i32 {
    let mut xb: f64 = 0f64;
    let mut yb: f64 = 0f64;
    let mut x2b: f64 = 0f64;
    let mut y2b: f64 = 0f64;
    let mut xyb: f64 = 0f64;
    let mut bn: f64 = 0f64;
    let mut i: i32 = 0;
    let mut x0: i32 = (*a.offset(0)).x0;
    let mut x1: i32 = (*a.offset((fits - 1i32) as isize)).x1;
    i = 0;
    while i < fits {
        let mut weight: f64 = (((*a.offset(i as isize)).bn + (*a.offset(i as isize)).an) as f32
            * (*info).twofitweight
            / ((*a.offset(i as isize)).an + 1i32) as f32) as f64
            + 1.0;
        xb += (*a.offset(i as isize)).xb as f64 + (*a.offset(i as isize)).xa as f64 * weight;
        yb += (*a.offset(i as isize)).yb as f64 + (*a.offset(i as isize)).ya as f64 * weight;
        x2b += (*a.offset(i as isize)).x2b as f64 + (*a.offset(i as isize)).x2a as f64 * weight;
        y2b += (*a.offset(i as isize)).y2b as f64 + (*a.offset(i as isize)).y2a as f64 * weight;
        xyb += (*a.offset(i as isize)).xyb as f64 + (*a.offset(i as isize)).xya as f64 * weight;
        bn += (*a.offset(i as isize)).bn as f64 + (*a.offset(i as isize)).an as f64 * weight;
        i += 1
    }
    if *y0 >= 0 {
        xb += x0 as f64;
        yb += *y0 as f64;
        x2b += (x0 * x0) as f64;
        y2b += (*y0 * *y0) as f64;
        xyb += (*y0 * x0) as f64;
        bn += 1.
    }
    if *y1 >= 0 {
        xb += x1 as f64;
        yb += *y1 as f64;
        x2b += (x1 * x1) as f64;
        y2b += (*y1 * *y1) as f64;
        xyb += (*y1 * x1) as f64;
        bn += 1.
    }
    let mut denom: f64 = bn * x2b - xb * xb;
    if denom > 0.0 {
        let mut a_0: f64 = (yb * x2b - xyb * xb) / denom;
        let mut b: f64 = (bn * xyb - xb * yb) / denom;
        *y0 = crate::stdlib::rint(a_0 + b * x0 as f64) as i32;
        *y1 = crate::stdlib::rint(a_0 + b * x1 as f64) as i32;
        /* limit to our range! */
        if *y0 > 1023 {
            *y0 = 1023
        } /* index by range list position */
        if *y1 > 1023 {
            *y1 = 1023
        } /* index by range list position */
        if *y0 < 0 {
            *y0 = 0
        } /* sorted index of range list position (+2) */
        if *y1 < 0 {
            *y1 = 0
        } /* mark all unused */
        return 0i32;
    } else {
        *y0 = 0; /* mark all unused */
        *y1 = 0; /* 0 for the implicit 0 post */
        return 1i32;
    }; /* 1 for the implicit post at n */
}

unsafe extern "C" fn inspect_error(
    mut x0: i32,
    mut x1: i32,
    mut y0: i32,
    mut y1: i32,
    mut mask: *const f32,
    mut mdct: *const f32,
    mut info: *mut crate::backends_h::vorbis_info_floor1,
) -> i32 {
    let mut dy: i32 = y1 - y0; /* no neighbor yet */
    let mut adx: i32 = x1 - x0;
    let mut ady: i32 = crate::stdlib::abs(dy);
    let mut base: i32 = dy / adx;
    let mut sy: i32 = if dy < 0 { (base) - 1 } else { (base) + 1 };
    let mut x: i32 = x0;
    let mut y: i32 = y0;
    let mut err: i32 = 0;
    let mut val: i32 = vorbis_dBquant(mask.offset(x as isize));
    let mut mse: i32 = 0;
    let mut n: i32 = 0;
    ady -= crate::stdlib::abs(base * adx);
    mse = y - val;
    mse *= mse;
    n += 1;
    if *mdct.offset(x as isize) + (*info).twofitatten >= *mask.offset(x as isize) {
        if y as f32 + (*info).maxover < val as f32 {
            return 1i32;
        }
        if y as f32 - (*info).maxunder > val as f32 {
            return 1i32;
        }
    }
    loop {
        x += 1;
        if !(x < x1) {
            break;
        }
        err = err + ady;
        if err >= adx {
            err -= adx;
            y += sy
        } else {
            y += base
        }
        val = vorbis_dBquant(mask.offset(x as isize));
        mse += (y - val) * (y - val);
        n += 1;
        if *mdct.offset(x as isize) + (*info).twofitatten >= *mask.offset(x as isize) {
            if val != 0 {
                if y as f32 + (*info).maxover < val as f32 {
                    return 1i32;
                }
                if y as f32 - (*info).maxunder > val as f32 {
                    return 1i32;
                }
            }
        }
    }
    if (*info).maxover * (*info).maxover / n as f32 > (*info).maxerr {
        return 0i32;
    }
    if (*info).maxunder * (*info).maxunder / n as f32 > (*info).maxerr {
        return 0i32;
    }
    if (mse / n) as f32 > (*info).maxerr {
        return 1i32;
    }
    return 0;
}

unsafe extern "C" fn post_Y(mut A: *mut i32, mut B: *mut i32, mut pos: i32) -> i32 {
    if *A.offset(pos as isize) < 0 {
        return *B.offset(pos as isize);
    }
    if *B.offset(pos as isize) < 0 {
        return *A.offset(pos as isize);
    }
    return *A.offset(pos as isize) + *B.offset(pos as isize) >> 1;
}
#[no_mangle]

pub unsafe extern "C" fn floor1_fit(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut look: *mut crate::codec_internal_h::vorbis_look_floor1,
    mut logmdct: *const f32,
    mut logmask: *const f32,
) -> *mut i32 {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut n: isize = (*look).n as isize;
    let mut posts: isize = (*look).posts as isize;
    let mut nonzero: isize = 0;
    let mut fits: [lsfit_acc; 64] = [lsfit_acc {
        x0: 0,
        x1: 0,
        xa: 0,
        ya: 0,
        x2a: 0,
        y2a: 0,
        xya: 0,
        an: 0,
        xb: 0,
        yb: 0,
        x2b: 0,
        y2b: 0,
        xyb: 0,
        bn: 0,
    }; 64];
    let mut fit_valueA: [i32; 65] = [0; 65];
    let mut fit_valueB: [i32; 65] = [0; 65];
    let mut loneighbor: [i32; 65] = [0; 65];
    let mut hineighbor: [i32; 65] = [0; 65];
    let mut output: *mut i32 = 0 as *mut i32;
    let mut memo: [i32; 65] = [0; 65];
    i = 0;
    while i < posts {
        fit_valueA[i as usize] = -(200);
        i += 1
    }
    i = 0;
    while i < posts {
        fit_valueB[i as usize] = -(200);
        i += 1
    }
    i = 0;
    while i < posts {
        loneighbor[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < posts {
        hineighbor[i as usize] = 1;
        i += 1
    }
    i = 0;
    while i < posts {
        memo[i as usize] = -(1);
        i += 1
    }
    /* quantize the relevant floor points and collect them into line fit
    structures (one per minimal division) at the same time */
    if posts == 0 {
        nonzero += accumulate_fit(
            logmask,
            logmdct,
            0,
            n as i32,
            fits.as_mut_ptr(),
            n as i32,
            info,
        ) as isize
    } else {
        i = 0;
        while i < posts - 1 {
            nonzero += accumulate_fit(
                logmask,
                logmdct,
                (*look).sorted_index[i as usize],
                (*look).sorted_index[(i + 1) as usize],
                fits.as_mut_ptr().offset(i),
                n as i32,
                info,
            ) as isize;
            i += 1
        }
    }
    if nonzero != 0 {
        /* start by fitting the implicit base case.... */
        let mut y0: i32 = -(200);
        let mut y1: i32 = -(200);
        fit_line(
            fits.as_mut_ptr(),
            (posts - 1) as i32,
            &mut y0,
            &mut y1,
            info,
        );
        fit_valueA[0] = y0;
        fit_valueB[0] = y0;
        fit_valueB[1] = y1;
        fit_valueA[1] = y1;
        /* Non degenerate case */
        /* start progressive splitting.  This is a greedy, non-optimal
        algorithm, but simple and close enough to the best
        answer. */
        i = 2;
        while i < posts {
            let mut sortpos: i32 = (*look).reverse_index[i as usize];
            let mut ln: i32 = loneighbor[sortpos as usize];
            let mut hn: i32 = hineighbor[sortpos as usize];
            /* eliminate repeat searches of a particular range with a memo */
            if memo[ln as usize] != hn {
                /* haven't performed this error search yet */
                let mut lsortpos: i32 = (*look).reverse_index[ln as usize];
                let mut hsortpos: i32 = (*look).reverse_index[hn as usize];
                memo[ln as usize] = hn;
                /* A note: we want to bound/minimize *local*, not global, error */
                let mut lx: i32 = (*info).postlist[ln as usize];
                let mut hx: i32 = (*info).postlist[hn as usize];
                let mut ly: i32 = post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), ln);
                let mut hy: i32 = post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), hn);
                if ly == -(1) || hy == -(1) {
                    crate::stdlib::exit(1i32);
                }
                if inspect_error(lx, hx, ly, hy, logmask, logmdct, info) != 0 {
                    /* outside error bounds/begin search area.  Split it. */
                    let mut ly0: i32 = -(200);
                    let mut ly1: i32 = -(200);
                    let mut hy0: i32 = -(200);
                    let mut hy1: i32 = -(200);
                    let mut ret0: i32 = fit_line(
                        fits.as_mut_ptr().offset(lsortpos as isize),
                        sortpos - lsortpos,
                        &mut ly0,
                        &mut ly1,
                        info,
                    );
                    let mut ret1: i32 = fit_line(
                        fits.as_mut_ptr().offset(sortpos as isize),
                        hsortpos - sortpos,
                        &mut hy0,
                        &mut hy1,
                        info,
                    );
                    if ret0 != 0 {
                        ly0 = ly;
                        ly1 = hy0
                    }
                    if ret1 != 0 {
                        hy0 = ly1;
                        hy1 = hy
                    }
                    if ret0 != 0 && ret1 != 0 {
                        fit_valueA[i as usize] = -(200);
                        fit_valueB[i as usize] = -(200)
                    } else {
                        /* store new edge values */
                        fit_valueB[ln as usize] = ly0;
                        if ln == 0 {
                            fit_valueA[ln as usize] = ly0
                        }
                        fit_valueA[i as usize] = ly1;
                        fit_valueB[i as usize] = hy0;
                        fit_valueA[hn as usize] = hy1;
                        if hn == 1 {
                            fit_valueB[hn as usize] = hy1
                        }
                        if ly1 >= 0 || hy0 >= 0 {
                            /* store new neighbor values */
                            j = (sortpos - 1) as isize;
                            while j >= 0 {
                                if !(hineighbor[j as usize] == hn) {
                                    break;
                                }
                                hineighbor[j as usize] = i as i32;
                                j -= 1
                            }
                            j = (sortpos + 1) as isize;
                            while j < posts {
                                if !(loneighbor[j as usize] == ln) {
                                    break;
                                }
                                loneighbor[j as usize] = i as i32;
                                j += 1
                            }
                        }
                    }
                } else {
                    fit_valueA[i as usize] = -(200);
                    fit_valueB[i as usize] = -(200)
                }
            }
            i += 1
        }
        output = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb,
            (::std::mem::size_of::<i32>()).wrapping_mul(posts as usize) as isize,
        ) as *mut i32;
        *output.offset(0) = post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), 0);
        *output.offset(1) = post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), 1);
        /* fill in posts marked as not using a fit; we will zero
        back out to 'unused' when encoding them so long as curve
        interpolation doesn't force them into use */
        i = 2;
        while i < posts {
            let mut ln_0: i32 = (*look).loneighbor[(i - 2) as usize];
            let mut hn_0: i32 = (*look).hineighbor[(i - 2) as usize];
            let mut x0: i32 = (*info).postlist[ln_0 as usize];
            let mut x1: i32 = (*info).postlist[hn_0 as usize];
            let mut y0_0: i32 = *output.offset(ln_0 as isize);
            let mut y1_0: i32 = *output.offset(hn_0 as isize);
            let mut predicted: i32 = render_point(x0, x1, y0_0, y1_0, (*info).postlist[i as usize]);
            let mut vx: i32 = post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), i as i32);
            if vx >= 0 && predicted != vx {
                *output.offset(i) = vx
            } else {
                *output.offset(i) = predicted | 0x8000
            }
            i += 1
        }
    }
    return output;
}
#[no_mangle]

pub unsafe extern "C" fn floor1_interpolate_fit(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut look: *mut crate::codec_internal_h::vorbis_look_floor1,
    mut A: *mut i32,
    mut B: *mut i32,
    mut del: i32,
) -> *mut i32 {
    let mut i: isize = 0;
    let mut posts: isize = (*look).posts as isize;
    let mut output: *mut i32 = 0 as *mut i32;
    if !A.is_null() && !B.is_null() {
        output = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb,
            (::std::mem::size_of::<i32>()).wrapping_mul(posts as usize) as isize,
        ) as *mut i32;
        /* overly simpleminded--- look again post 1.2 */
        i = 0;
        while i < posts {
            *output.offset(i) =
                (65536 - del) * (*A.offset(i) & 0x7fff) + del * (*B.offset(i) & 0x7fff) + 32768
                    >> 16;
            if *A.offset(i) & 0x8000 != 0 && *B.offset(i) & 0x8000 != 0 {
                *output.offset(i) |= 0x8000
            }
            i += 1
        }
    }
    return output;
}
/* local lookup storage */
/* envelope lookup */
/* block, type */
/* local storage, only used on the encoding side.  This way the
application does not need to worry about freeing some packets'
memory and not others'; packet storage is always tracked.
Cleared next call to a _dsp_ function */
/* codec_setup_info contains all the setup information specific to the
   specific compression/decompression mode in progress (eg,
   psychoacoustic settings, channel setup, options, codebook
   etc).
*********************************************************************/
/* Vorbis supports only short and long blocks, but allows the
encoder to choose the sizes */
/* modes are the primary means of supporting on-the-fly different
blocksizes, different channel mappings (LR or M/A),
different residue backends, etc.  Each mode consists of a
blocksize flag and a mapping (along with the mapping setup */
/* encode only */
/* encode only */
/* used only by vorbisenc.c.  It's a
highly redundant structure, but
improves clarity of program flow. */
/* painless downsample for decode */
/* in */
#[no_mangle]

pub unsafe extern "C" fn floor1_encode(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vb: *mut crate::codec_h::vorbis_block,
    mut look: *mut crate::codec_internal_h::vorbis_look_floor1,
    mut post: *mut i32,
    mut ilogmask: *mut i32,
) -> i32 {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut posts: isize = (*look).posts as isize;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut out: [i32; 65] = [0; 65];
    let mut sbooks: *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook =
        (*ci).book_param.as_mut_ptr();
    let mut books: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook = (*ci).fullbooks;
    /* quantize values to multiplier spec */
    if !post.is_null() {
        i = 0;
        while i < posts {
            let mut val: i32 = *post.offset(i) & 0x7fff;
            match (*info).mult {
                1 => {
                    /* 1024 -> 256 */
                    val >>= 2
                }
                2 => {
                    /* 1024 -> 128 */
                    val >>= 3
                }
                3 => {
                    /* 1024 -> 86 */
                    val /= 12
                }
                4 => {
                    /* 1024 -> 64 */
                    val >>= 4
                }
                _ => {}
            }
            *post.offset(i) = val | *post.offset(i) & 0x8000;
            i += 1
        }
        out[0] = *post.offset(0);
        out[1] = *post.offset(1);
        /* find prediction values for each post and subtract them */
        i = 2; /* in case there was roundoff jitter
               in interpolation */
        while i < posts {
            let mut ln: i32 = (*look).loneighbor[(i - 2) as usize];
            let mut hn: i32 = (*look).hineighbor[(i - 2) as usize];
            let mut x0: i32 = (*info).postlist[ln as usize];
            let mut x1: i32 = (*info).postlist[hn as usize];
            let mut y0: i32 = *post.offset(ln as isize);
            let mut y1: i32 = *post.offset(hn as isize);
            let mut predicted: i32 = render_point(x0, x1, y0, y1, (*info).postlist[i as usize]);
            if *post.offset(i) & 0x8000 != 0 || predicted == *post.offset(i) {
                *post.offset(i) = predicted | 0x8000;
                out[i as usize] = 0
            } else {
                let mut headroom: i32 = if (*look).quant_q - predicted < predicted {
                    ((*look).quant_q) - predicted
                } else {
                    predicted
                };
                let mut val_0: i32 = *post.offset(i) - predicted;
                /* at this point the 'deviation' value is in the range +/- max
                range, but the real, unique range can always be mapped to
                only [0-maxrange).  So we want to wrap the deviation into
                this limited range, but do it in the way that least screws
                an essentially gaussian probability distribution. */
                if val_0 < 0 {
                    if val_0 < -headroom {
                        val_0 = headroom - val_0 - 1
                    } else {
                        val_0 = -(1) - (val_0 << 1)
                    }
                } else if val_0 >= headroom {
                    val_0 = val_0 + headroom
                } else {
                    val_0 <<= 1
                }
                out[i as usize] = val_0;
                *post.offset(ln as isize) &= 0x7fff;
                *post.offset(hn as isize) &= 0x7fff
            }
            i += 1
        }
        /* we have everything we need. pack it out */
        /* mark nontrivial floor */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1);
        /* beginning/end post */
        (*look).frames += 1;
        (*look).postbits += (crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*look).quant_q - 1i32) as crate::config_types_h::ogg_uint32_t,
        ) * 2) as isize;
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            out[0] as usize,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*look).quant_q - 1i32) as crate::config_types_h::ogg_uint32_t,
            ),
        );
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            out[1] as usize,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*look).quant_q - 1i32) as crate::config_types_h::ogg_uint32_t,
            ),
        );
        /* partition by partition */
        i = 0;
        j = 2;
        while i < (*info).partitions as isize {
            let mut class: i32 = (*info).partitionclass[i as usize];
            let mut cdim: i32 = (*info).class_dim[class as usize];
            let mut csubbits: i32 = (*info).class_subs[class as usize];
            let mut csub: i32 = (1) << csubbits;
            let mut bookas: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            let mut cval: i32 = 0;
            let mut cshift: i32 = 0;
            let mut k: i32 = 0;
            let mut l: i32 = 0;
            /* generate the partition's first stage cascade value */
            if csubbits != 0 {
                let mut maxval: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0]; /* gcc's static analysis
                                                                     issues a warning without
                                                                     initialization */
                k = 0;
                while k < csub {
                    let mut booknum: i32 = (*info).class_subbook[class as usize][k as usize];
                    if booknum < 0 {
                        maxval[k as usize] = 1
                    } else {
                        maxval[k as usize] = (**sbooks
                            .offset((*info).class_subbook[class as usize][k as usize] as isize))
                        .entries as i32
                    }
                    k += 1
                }
                k = 0;
                while k < cdim {
                    l = 0;
                    while l < csub {
                        let mut val_1: i32 = out[(j + k as isize) as usize];
                        if val_1 < maxval[l as usize] {
                            bookas[k as usize] = l;
                            break;
                        } else {
                            l += 1
                        }
                    }
                    cval |= bookas[k as usize] << cshift;
                    cshift += csubbits;
                    k += 1
                }
                /* write it */
                (*look).phrasebits += crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode(
                    books.offset((*info).class_book[class as usize] as isize),
                    cval,
                    opb,
                ) as isize
            }
            /* write post values */
            k = 0;
            while k < cdim {
                let mut book: i32 =
                    (*info).class_subbook[class as usize][bookas[k as usize] as usize];
                if book >= 0 {
                    /* hack to allow training with 'bad' books */
                    if (out[(j + k as isize) as usize] as isize)
                        < (*books.offset(book as isize)).entries
                    {
                        (*look).postbits +=
                            crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode(
                                books.offset(book as isize),
                                out[(j + k as isize) as usize],
                                opb,
                            ) as isize
                    }
                    /*else
                    fprintf(stderr,"+!");*/
                }
                k += 1
            }
            j += cdim as isize;
            i += 1
        }
        /* generate quantized floor equivalent to what we'd unpack in decode */
        /* render the lines */
        let mut hx: i32 = 0; /* be certain */
        let mut lx: i32 = 0;
        let mut ly: i32 = *post.offset(0) * (*info).mult;
        let mut n: i32 = ((*ci).blocksizes[(*vb).W as usize] / 2) as i32;
        j = 1;
        while j < (*look).posts as isize {
            let mut current: i32 = (*look).forward_index[j as usize];
            let mut hy: i32 = *post.offset(current as isize) & 0x7fff;
            if hy == *post.offset(current as isize) {
                hy *= (*info).mult;
                hx = (*info).postlist[current as usize];
                render_line0(n, lx, hx, ly, hy, ilogmask);
                lx = hx;
                ly = hy
            }
            j += 1
        }
        j = hx as isize;
        while j < ((*vb).pcmend / 2i32) as isize {
            *ilogmask.offset(j) = ly;
            j += 1
        }
        return 1i32;
    } else {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0, 1);
        crate::stdlib::memset(
            ilogmask as *mut libc::c_void,
            0,
            (((*vb).pcmend / 2i32) as usize).wrapping_mul(::std::mem::size_of::<i32>()),
        );
        return 0i32;
    };
}

unsafe extern "C" fn floor1_inverse1(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 =
        in_0 as *mut crate::codec_internal_h::vorbis_look_floor1;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut books: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook = (*ci).fullbooks;
    /* unpack wrapped/predicted values from stream */
    if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(&mut (*vb).opb, 1) == 1 {
        let mut fit_value: *mut i32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb,
            ((*look).posts as usize).wrapping_mul(::std::mem::size_of::<i32>()) as isize,
        ) as *mut i32;
        *fit_value.offset(0) = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            &mut (*vb).opb,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*look).quant_q - 1) as crate::config_types_h::ogg_uint32_t,
            ),
        ) as i32;
        *fit_value.offset(1) = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            &mut (*vb).opb,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*look).quant_q - 1) as crate::config_types_h::ogg_uint32_t,
            ),
        ) as i32;
        /* partition by partition */
        i = 0;
        j = 2;
        's_33: loop {
            if !(i < (*info).partitions) {
                current_block = 2719512138335094285;
                break;
            }
            let mut class: i32 = (*info).partitionclass[i as usize];
            let mut cdim: i32 = (*info).class_dim[class as usize];
            let mut csubbits: i32 = (*info).class_subs[class as usize];
            let mut csub: i32 = (1) << csubbits;
            let mut cval: i32 = 0;
            /* decode the partition's first stage cascade value */
            if csubbits != 0 {
                cval = crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                    books.offset((*info).class_book[class as usize] as isize),
                    &mut (*vb).opb,
                ) as i32;
                if cval == -(1) {
                    current_block = 11812956849860977184;
                    break;
                }
            }
            k = 0;
            while k < cdim {
                let mut book: i32 =
                    (*info).class_subbook[class as usize][(cval & csub - 1) as usize];
                cval >>= csubbits;
                if book >= 0 {
                    let ref mut fresh0 = *fit_value.offset((j + k) as isize);
                    *fresh0 = crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                        books.offset(book as isize),
                        &mut (*vb).opb,
                    ) as i32;
                    if *fresh0 == -(1) {
                        current_block = 11812956849860977184;
                        break 's_33;
                    }
                } else {
                    *fit_value.offset((j + k) as isize) = 0
                }
                k += 1
            }
            j += cdim;
            i += 1
        }
        match current_block {
            11812956849860977184 => {}
            _ => {
                /* unwrap positive values and reconsitute via linear interpolation */
                i = 2;
                while i < (*look).posts {
                    let mut predicted: i32 = render_point(
                        (*info).postlist[(*look).loneighbor[(i - 2) as usize] as usize],
                        (*info).postlist[(*look).hineighbor[(i - 2) as usize] as usize],
                        *fit_value.offset((*look).loneighbor[(i - 2) as usize] as isize),
                        *fit_value.offset((*look).hineighbor[(i - 2) as usize] as isize),
                        (*info).postlist[i as usize],
                    );
                    let mut hiroom: i32 = (*look).quant_q - predicted;
                    let mut loroom: i32 = predicted;
                    let mut room: i32 = (if hiroom < loroom { hiroom } else { loroom }) << 1;
                    let mut val: i32 = *fit_value.offset(i as isize);
                    if val != 0 {
                        if val >= room {
                            if hiroom > loroom {
                                val = val - loroom
                            } else {
                                val = -(1) - (val - hiroom)
                            }
                        } else if val & 1 != 0 {
                            val = -(val + 1 >> 1)
                        } else {
                            val >>= 1
                        }
                        *fit_value.offset(i as isize) = val + predicted & 0x7fff;
                        *fit_value.offset((*look).loneighbor[(i - 2) as usize] as isize) &= 0x7fff;
                        *fit_value.offset((*look).hineighbor[(i - 2) as usize] as isize) &= 0x7fff
                    } else {
                        *fit_value.offset(i as isize) = predicted | 0x8000
                    }
                    i += 1
                }
                return fit_value as *mut libc::c_void;
            }
        }
    }
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn floor1_inverse2(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut in_0: *mut libc::c_void,
    mut memo: *mut libc::c_void,
    mut out: *mut f32,
) -> i32 {
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 =
        in_0 as *mut crate::codec_internal_h::vorbis_look_floor1;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut n: i32 = ((*ci).blocksizes[(*vb).W as usize] / 2) as i32;
    let mut j: i32 = 0;
    if !memo.is_null() {
        /* render the lines */
        let mut fit_value: *mut i32 = memo as *mut i32;
        let mut hx: i32 = 0;
        let mut lx: i32 = 0;
        let mut ly: i32 = *fit_value.offset(0) * (*info).mult;
        /* guard lookup against out-of-range values */
        ly = if ly < 0 {
            0
        } else if ly > 255 {
            255
        } else {
            ly
        };
        j = 1;
        while j < (*look).posts {
            let mut current: i32 = (*look).forward_index[j as usize];
            let mut hy: i32 = *fit_value.offset(current as isize) & 0x7fff;
            if hy == *fit_value.offset(current as isize) {
                hx = (*info).postlist[current as usize];
                hy *= (*info).mult;
                /* guard lookup against out-of-range values */
                hy = if hy < 0 {
                    0
                } else if hy > 255 {
                    255
                } else {
                    hy
                }; /* be certain */
                render_line(n, lx, hx, ly, hy, out);
                lx = hx;
                ly = hy
            }
            j += 1
        }
        j = hx;
        while j < n {
            *out.offset(j as isize) *= FLOOR1_fromdB_LOOKUP[ly as usize];
            j += 1
        }
        return 1i32;
    }
    crate::stdlib::memset(
        out as *mut libc::c_void,
        0,
        (::std::mem::size_of::<f32>()).wrapping_mul(n as usize),
    );
    return 0;
}
/* export hooks */
#[no_mangle]

pub static mut floor1_exportbundle: crate::backends_h::vorbis_func_floor = {
    let mut init = crate::backends_h::vorbis_func_floor {
        pack: Some(
            floor1_pack
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> (),
        ),
        unpack: Some(
            floor1_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            floor1_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(floor1_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(floor1_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        inverse1: Some(
            floor1_inverse1
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        inverse2: Some(
            floor1_inverse2
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
