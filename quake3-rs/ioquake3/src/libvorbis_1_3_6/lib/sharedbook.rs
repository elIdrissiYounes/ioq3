use ::libc;

pub use crate::config_types_h::ogg_uint32_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::calloc;
pub use crate::stdlib::free;
pub use crate::stdlib::malloc;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;

pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;

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

function: basic shared codebook operations

********************************************************************/
/* *** pack/unpack helpers ******************************************/
#[no_mangle]

pub unsafe extern "C" fn ov_ilog(mut v: crate::config_types_h::ogg_uint32_t) -> i32 {
    let mut ret: i32 = 0;
    ret = 0;
    while v != 0 {
        v >>= 1;
        ret += 1
    }
    return ret;
}
/* bias toward values smaller than 1. */
/* doesn't currently guard under/overflow */
#[no_mangle]

pub unsafe extern "C" fn _float32_pack(mut val: f32) -> isize {
    let mut sign: i32 = 0; //+epsilon
    let mut exp: isize = 0;
    let mut mant: isize = 0;
    if val < 0f32 {
        sign = 0x80000000u32 as i32;
        val = -val
    }
    exp = crate::stdlib::floor(crate::stdlib::log(val as f64) / crate::stdlib::log(2f64) + 0.001)
        as isize;
    mant = crate::stdlib::rint(crate::stdlib::ldexp(
        val as f64,
        ((21i32 - 1) as isize - exp) as i32,
    )) as isize;
    exp = (exp + 768) << 21;
    return sign as isize | exp | mant;
}
#[no_mangle]

pub unsafe extern "C" fn _float32_unpack(mut val: isize) -> f32 {
    let mut mant: f64 = (val & 0x1fffffi32 as isize) as f64;
    let mut sign: i32 = (val & 0x80000000u32 as isize) as i32;
    let mut exp: isize = (val & 0x7fe00000 as isize) >> 21;
    if sign != 0 {
        mant = -mant
    }
    return crate::stdlib::ldexp(mant, (exp - (21i32 - 1) as isize - 768) as i32) as f32;
}
/* given a list of word lengths, generate a list of codewords.  Works
for length ordered or unordered, always assigns the lowest valued
codewords first.  Extended to handle unused entries (length 0) */
#[no_mangle]

pub unsafe extern "C" fn _make_words(
    mut l: *mut i8,
    mut n: isize,
    mut sparsecount: isize,
) -> *mut crate::config_types_h::ogg_uint32_t {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut count: isize = 0;
    let mut marker: [crate::config_types_h::ogg_uint32_t; 33] = [0; 33];
    let mut r: *mut crate::config_types_h::ogg_uint32_t = crate::stdlib::malloc(
        ((if sparsecount != 0 { sparsecount } else { n }) as usize)
            .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>()),
    )
        as *mut crate::config_types_h::ogg_uint32_t;
    crate::stdlib::memset(
        marker.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[crate::config_types_h::ogg_uint32_t; 33]>(),
    );
    i = 0;
    while i < n {
        let mut length: isize = *l.offset(i) as isize;
        if length > 0 {
            let mut entry: crate::config_types_h::ogg_uint32_t = marker[length as usize];
            /* when we claim a node for an entry, we also claim the nodes
            below it (pruning off the imagined tree that may have dangled
            from it) as well as blocking the use of any nodes directly
            above for leaves */
            /* update ourself */
            if length < 32 && entry >> length != 0 {
                /* error condition; the lengths must specify an overpopulated tree */
                crate::stdlib::free(r as *mut libc::c_void);
                return 0 as *mut crate::config_types_h::ogg_uint32_t;
            }
            let fresh0 = count;
            count = count + 1;
            *r.offset(fresh0) = entry;
            /* Look to see if the next shorter marker points to the node
            above. if so, update it and repeat.  */
            j = length;
            while j > 0 {
                if marker[j as usize] & 1 != 0 {
                    /* have to jump branches */
                    if j == 1 {
                        marker[1] = marker[1].wrapping_add(1)
                    } else {
                        marker[j as usize] = marker[(j - 1) as usize] << 1
                    }
                    break;
                /* invariant says next upper marker would already
                have been moved if it was on the same path */
                } else {
                    marker[j as usize] = marker[j as usize].wrapping_add(1);
                    j -= 1
                }
            }
            /* prune the tree; the implicit invariant says all the longer
            markers were dangling from our just-taken node.  Dangle them
            from our *new* node. */
            j = length + 1;
            while j < 33 {
                if !(marker[j as usize] >> 1 == entry) {
                    break;
                }
                entry = marker[j as usize];
                marker[j as usize] = marker[(j - 1) as usize] << 1;
                j += 1
            }
        } else if sparsecount == 0isize {
            count += 1
        }
        i += 1
    }
    /* any underpopulated tree must be rejected. */
    /* Single-entry codebooks are a retconned extension to the spec.
    They have a single codeword '0' of length 1 that results in an
    underpopulated tree.  Shield that case from the underformed tree check. */
    if !(count == 1 && marker[2] == 2) {
        i = 1;
        while i < 33 {
            if marker[i as usize] as usize & 0xffffffff as usize >> 32 - i != 0 {
                crate::stdlib::free(r as *mut libc::c_void);
                return 0 as *mut crate::config_types_h::ogg_uint32_t;
            }
            i += 1
        }
    }
    /* bitreverse the words because our bitwise packer/unpacker is LSb
    endian */
    i = 0;
    count = 0;
    while i < n {
        let mut temp: crate::config_types_h::ogg_uint32_t = 0;
        j = 0;
        while j < *l.offset(i) as isize {
            temp <<= 1;
            temp |= *r.offset(count) >> j & 1;
            j += 1
        }
        if sparsecount != 0 {
            if *l.offset(i) != 0 {
                let fresh1 = count;
                count = count + 1;
                *r.offset(fresh1) = temp
            }
        } else {
            let fresh2 = count;
            count = count + 1;
            *r.offset(fresh2) = temp
        }
        i += 1
    }
    return r;
}
/* there might be a straightforward one-line way to do the below
that's portable and totally safe against roundoff, but I haven't
thought of it.  Therefore, we opt on the side of caution */
#[no_mangle]

pub unsafe extern "C" fn _book_maptype1_quantvals(
    mut b: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) -> isize {
    let mut vals: isize = 0;
    if (*b).entries < 1isize {
        return 0isize;
    }
    vals = crate::stdlib::floor(crate::stdlib::pow(
        (*b).entries as f32 as f64,
        (1.0 / (*b).dim as f32) as f64,
    )) as isize;
    /* the above *should* be reliable, but we'll not assume that FP is
    ever reliable when bitstream sync is at stake; verify via integer
    means that vals really is the greatest value of dim for which
    vals^b->bim <= b->entries */
    /* treat the above as an initial guess */
    if vals < 1 {
        vals = 1
    }
    loop {
        let mut acc: isize = 1;
        let mut acc1: isize = 1;
        let mut i: i32 = 0;
        i = 0;
        while (i as isize) < (*b).dim {
            if (*b).entries / vals < acc {
                break;
            }
            acc *= vals;
            if (9223372036854775807 as isize / (vals + 1)) < acc1 {
                acc1 = 9223372036854775807 as isize
            } else {
                acc1 *= vals + 1
            }
            i += 1
        }
        if i as isize >= (*b).dim && acc <= (*b).entries && acc1 > (*b).entries {
            return vals;
        } else {
            if (i as isize) < (*b).dim || acc > (*b).entries {
                vals -= 1
            } else {
                vals += 1
            }
        }
    }
}
/* unpack the quantized list of values for encode/decode ***********/
/* we need to deal with two map types: in map type 1, the values are
generated algorithmically (each column of the vector counts through
the values in the quant vector). in map type 2, all the values came
in in an explicit list.  Both value lists must be unpacked */
#[no_mangle]

pub unsafe extern "C" fn _book_unquantize(
    mut b: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    mut n: i32,
    mut sparsemap: *mut i32,
) -> *mut f32 {
    let mut j: isize = 0;
    let mut k: isize = 0;
    let mut count: isize = 0;
    if (*b).maptype == 1 || (*b).maptype == 2 {
        let mut quantvals: i32 = 0;
        let mut mindel: f32 = _float32_unpack((*b).q_min);
        let mut delta: f32 = _float32_unpack((*b).q_delta);
        let mut r: *mut f32 = crate::stdlib::calloc(
            (n as isize * (*b).dim) as usize,
            ::std::mem::size_of::<f32>(),
        ) as *mut f32;
        /* maptype 1 and 2 both use a quantized value vector, but
        different sizes */
        match (*b).maptype {
            1 => {
                /* most of the time, entries%dimensions == 0, but we need to be
                well defined.  We define that the possible vales at each
                scalar is values == entries/dim.  If entries%dim != 0, we'll
                have 'too few' values (values*dim<entries), which means that
                we'll have 'left over' entries; left over entries use zeroed
                values (and are wasted).  So don't generate codebooks like
                that */
                quantvals = _book_maptype1_quantvals(b) as i32;
                j = 0;
                while j < (*b).entries {
                    if !sparsemap.is_null() && *(*b).lengthlist.offset(j) as i32 != 0
                        || sparsemap.is_null()
                    {
                        let mut last: f32 = 0.0;
                        let mut indexdiv: i32 = 1;
                        k = 0;
                        while k < (*b).dim {
                            let mut index: i32 =
                                (j / indexdiv as isize % quantvals as isize) as i32;
                            let mut val: f32 = *(*b).quantlist.offset(index as isize) as f32;
                            val = (crate::stdlib::fabs(val as f64) * delta as f64
                                + mindel as f64
                                + last as f64) as f32;
                            if (*b).q_sequencep != 0 {
                                last = val
                            }
                            if !sparsemap.is_null() {
                                *r.offset(*sparsemap.offset(count) as isize * (*b).dim + k) = val
                            } else {
                                *r.offset(count * (*b).dim + k) = val
                            }
                            indexdiv *= quantvals;
                            k += 1
                        }
                        count += 1
                    }
                    j += 1
                }
            }
            2 => {
                j = 0;
                while j < (*b).entries {
                    if !sparsemap.is_null() && *(*b).lengthlist.offset(j) as i32 != 0
                        || sparsemap.is_null()
                    {
                        let mut last_0: f32 = 0.0;
                        k = 0;
                        while k < (*b).dim {
                            let mut val_0: f32 = *(*b).quantlist.offset(j * (*b).dim + k) as f32;
                            val_0 = (crate::stdlib::fabs(val_0 as f64) * delta as f64
                                + mindel as f64
                                + last_0 as f64) as f32;
                            if (*b).q_sequencep != 0 {
                                last_0 = val_0
                            }
                            if !sparsemap.is_null() {
                                *r.offset(*sparsemap.offset(count) as isize * (*b).dim + k) = val_0
                            } else {
                                *r.offset(count * (*b).dim + k) = val_0
                            }
                            k += 1
                        }
                        count += 1
                    }
                    j += 1
                }
            }
            _ => {}
        }
        return r;
    }
    return 0 as *mut f32;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_staticbook_destroy(
    mut b: *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) {
    if (*b).allocedp != 0 {
        if !(*b).quantlist.is_null() {
            crate::stdlib::free((*b).quantlist as *mut libc::c_void);
        }
        if !(*b).lengthlist.is_null() {
            crate::stdlib::free((*b).lengthlist as *mut libc::c_void);
        }
        crate::stdlib::memset(
            b as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::static_codebook>(),
        );
        crate::stdlib::free(b as *mut libc::c_void);
    };
    /* otherwise, it is in static memory */
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_clear(
    mut b: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
) {
    /* static book is not cleared; we're likely called on the lookup and
    the static codebook belongs to the info struct */
    if !(*b).valuelist.is_null() {
        crate::stdlib::free((*b).valuelist as *mut libc::c_void);
    }
    if !(*b).codelist.is_null() {
        crate::stdlib::free((*b).codelist as *mut libc::c_void);
    }
    if !(*b).dec_index.is_null() {
        crate::stdlib::free((*b).dec_index as *mut libc::c_void);
    }
    if !(*b).dec_codelengths.is_null() {
        crate::stdlib::free((*b).dec_codelengths as *mut libc::c_void);
    }
    if !(*b).dec_firsttable.is_null() {
        crate::stdlib::free((*b).dec_firsttable as *mut libc::c_void);
    }
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_init_encode(
    mut c: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut s: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) -> i32 {
    crate::stdlib::memset(
        c as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>(),
    );
    (*c).c = s;
    (*c).entries = (*s).entries;
    (*c).used_entries = (*s).entries;
    (*c).dim = (*s).dim;
    (*c).codelist = _make_words((*s).lengthlist, (*s).entries, 0);
    //c->valuelist=_book_unquantize(s,s->entries,NULL);
    (*c).quantvals = _book_maptype1_quantvals(s) as i32;
    (*c).minval = crate::stdlib::rint(_float32_unpack((*s).q_min) as f64) as i32;
    (*c).delta = crate::stdlib::rint(_float32_unpack((*s).q_delta) as f64) as i32;
    return 0;
}

unsafe extern "C" fn bitreverse(
    mut x: crate::config_types_h::ogg_uint32_t,
) -> crate::config_types_h::ogg_uint32_t {
    x = ((x >> 16) as usize & 0xffff | (x << 16) as usize & 0xffff0000 as usize)
        as crate::config_types_h::ogg_uint32_t;
    x = ((x >> 8) as usize & 0xff00ff as usize | (x << 8) as usize & 0xff00ff00 as usize)
        as crate::config_types_h::ogg_uint32_t;
    x = ((x >> 4) as usize & 0xf0f0f0f as usize | (x << 4) as usize & 0xf0f0f0f0 as usize)
        as crate::config_types_h::ogg_uint32_t;
    x = ((x >> 2) as usize & 0x33333333 as usize | (x << 2) as usize & 0xcccccccc as usize)
        as crate::config_types_h::ogg_uint32_t;
    return ((x >> 1) as usize & 0x55555555 as usize | (x << 1) as usize & 0xaaaaaaaa as usize)
        as crate::config_types_h::ogg_uint32_t;
}

unsafe extern "C" fn sort32a(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    return (**(a as *mut *mut crate::config_types_h::ogg_uint32_t)
        > **(b as *mut *mut crate::config_types_h::ogg_uint32_t)) as i32
        - ((**(a as *mut *mut crate::config_types_h::ogg_uint32_t))
            < **(b as *mut *mut crate::config_types_h::ogg_uint32_t)) as i32;
}
/* decode codebook arrangement is more heavily optimized than encode */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_init_decode(
    mut c: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut s: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut tabn: i32 = 0;
    let mut sortindex: *mut i32 = 0 as *mut i32;
    crate::stdlib::memset(
        c as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::codebook>(),
    );
    /* count actually used entries and find max length */
    i = 0;
    while (i as isize) < (*s).entries {
        if *(*s).lengthlist.offset(i as isize) as i32 > 0 {
            n += 1
        }
        i += 1
    }
    (*c).entries = (*s).entries;
    (*c).used_entries = n as isize;
    (*c).dim = (*s).dim;
    if n > 0 {
        /* two different remappings go on here.

        First, we collapse the likely sparse codebook down only to
        actually represented values/words.  This collapsing needs to be
        indexed as map-valueless books are used to encode original entry
        positions as integers.

        Second, we reorder all vectors, including the entry index above,
        by sorted bitreversed codeword to allow treeless decode. */
        /* perform sort */
        let mut codes: *mut crate::config_types_h::ogg_uint32_t =
            _make_words((*s).lengthlist, (*s).entries, (*c).used_entries);
        let mut fresh3 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<*mut crate::config_types_h::ogg_uint32_t>())
                .wrapping_mul(n as usize),
        );
        let mut codep: *mut *mut crate::config_types_h::ogg_uint32_t =
            fresh3.as_mut_ptr() as *mut *mut crate::config_types_h::ogg_uint32_t;
        if codes.is_null() {
            vorbis_book_clear(c);
            return -(1i32);
        } else {
            i = 0;
            while i < n {
                *codes.offset(i as isize) = bitreverse(*codes.offset(i as isize));
                let ref mut fresh4 = *codep.offset(i as isize);
                *fresh4 = codes.offset(i as isize);
                i += 1
            }
            crate::stdlib::qsort(
                codep as *mut libc::c_void,
                n as crate::stddef_h::size_t,
                ::std::mem::size_of::<*mut crate::config_types_h::ogg_uint32_t>(),
                Some(
                    sort32a
                        as unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> i32,
                ),
            );
            let mut fresh5 =
                ::std::vec::from_elem(0, (n as usize).wrapping_mul(::std::mem::size_of::<i32>()));
            sortindex = fresh5.as_mut_ptr();
            (*c).codelist = crate::stdlib::malloc(
                (n as usize)
                    .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>()),
            ) as *mut crate::config_types_h::ogg_uint32_t;
            /* the index is a reverse index */
            i = 0;
            while i < n {
                let mut position: i32 =
                    (*codep.offset(i as isize)).wrapping_offset_from(codes) as i32;
                *sortindex.offset(position as isize) = i;
                i += 1
            }
            i = 0;
            while i < n {
                *(*c).codelist.offset(*sortindex.offset(i as isize) as isize) =
                    *codes.offset(i as isize);
                i += 1
            }
            crate::stdlib::free(codes as *mut libc::c_void);
            (*c).valuelist = _book_unquantize(s, n, sortindex);
            (*c).dec_index =
                crate::stdlib::malloc((n as usize).wrapping_mul(::std::mem::size_of::<i32>()))
                    as *mut i32;
            n = 0;
            i = 0;
            while (i as isize) < (*s).entries {
                if *(*s).lengthlist.offset(i as isize) as i32 > 0 {
                    let fresh6 = n;
                    n = n + 1;
                    *(*c)
                        .dec_index
                        .offset(*sortindex.offset(fresh6 as isize) as isize) = i
                }
                i += 1
            }
            (*c).dec_codelengths =
                crate::stdlib::malloc((n as usize).wrapping_mul(::std::mem::size_of::<i8>()))
                    as *mut i8;
            (*c).dec_maxlength = 0;
            n = 0;
            i = 0;
            while (i as isize) < (*s).entries {
                if *(*s).lengthlist.offset(i as isize) as i32 > 0 {
                    let fresh7 = n;
                    n = n + 1;
                    *(*c)
                        .dec_codelengths
                        .offset(*sortindex.offset(fresh7 as isize) as isize) =
                        *(*s).lengthlist.offset(i as isize);
                    if *(*s).lengthlist.offset(i as isize) as i32 > (*c).dec_maxlength {
                        (*c).dec_maxlength = *(*s).lengthlist.offset(i as isize) as i32
                    }
                }
                i += 1
            }
            if n == 1 && (*c).dec_maxlength == 1 {
                /* special case the 'single entry codebook' with a single bit
                fastpath table (that always returns entry 0 )in order to use
                unmodified decode paths. */
                (*c).dec_firsttablen = 1; /* this is magic */
                (*c).dec_firsttable = crate::stdlib::calloc(
                    2,
                    ::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>(),
                ) as *mut crate::config_types_h::ogg_uint32_t;
                let ref mut fresh8 = *(*c).dec_firsttable.offset(1);
                *fresh8 = 1u32;
                *(*c).dec_firsttable.offset(0) = *fresh8
            } else {
                (*c).dec_firsttablen =
                    ov_ilog((*c).used_entries as crate::config_types_h::ogg_uint32_t) - 4;
                if (*c).dec_firsttablen < 5 {
                    (*c).dec_firsttablen = 5
                }
                if (*c).dec_firsttablen > 8 {
                    (*c).dec_firsttablen = 8
                }
                tabn = (1) << (*c).dec_firsttablen;
                (*c).dec_firsttable = crate::stdlib::calloc(
                    tabn as usize,
                    ::std::mem::size_of::<crate::config_types_h::ogg_uint32_t>(),
                ) as *mut crate::config_types_h::ogg_uint32_t;
                i = 0;
                while i < n {
                    if *(*c).dec_codelengths.offset(i as isize) as i32 <= (*c).dec_firsttablen {
                        let mut orig: crate::config_types_h::ogg_uint32_t =
                            bitreverse(*(*c).codelist.offset(i as isize));
                        j = 0;
                        while j
                            < (1)
                                << (*c).dec_firsttablen
                                    - *(*c).dec_codelengths.offset(i as isize) as i32
                        {
                            *(*c).dec_firsttable.offset(
                                (orig
                                    | (j << *(*c).dec_codelengths.offset(i as isize) as i32) as u32)
                                    as isize,
                            ) = (i + 1) as crate::config_types_h::ogg_uint32_t;
                            j += 1
                        }
                    }
                    i += 1
                }
                /* now fill in 'unused' entries in the firsttable with hi/lo search
                hints for the non-direct-hits */
                let mut mask: crate::config_types_h::ogg_uint32_t = ((0xfffffffe as usize)
                    << 31 - (*c).dec_firsttablen)
                    as crate::config_types_h::ogg_uint32_t;
                let mut lo: isize = 0;
                let mut hi: isize = 0;
                i = 0;
                while i < tabn {
                    let mut word: crate::config_types_h::ogg_uint32_t =
                        (i << 32 - (*c).dec_firsttablen) as crate::config_types_h::ogg_uint32_t;
                    if *(*c).dec_firsttable.offset(bitreverse(word) as isize) == 0u32 {
                        while (lo + 1) < n as isize && *(*c).codelist.offset(lo + 1) <= word {
                            lo += 1
                        }
                        while hi < n as isize && word >= *(*c).codelist.offset(hi) & mask {
                            hi += 1
                        }
                        /* we only actually have 15 bits per hint to play with here.
                        In order to overflow gracefully (nothing breaks, efficiency
                        just drops), encode as the difference from the extremes. */
                        let mut loval: usize = lo as usize;
                        let mut hival: usize = (n as isize - hi) as usize;
                        if loval > 0x7fff {
                            loval = 0x7fff
                        }
                        if hival > 0x7fff {
                            hival = 0x7fff
                        }
                        *(*c).dec_firsttable.offset(bitreverse(word) as isize) =
                            (0x80000000 as usize | loval << 15 | hival)
                                as crate::config_types_h::ogg_uint32_t
                    }
                    i += 1
                }
            }
        }
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_codeword(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut entry: i32,
) -> isize {
    if !(*book).c.is_null() {
        /* only use with encode; decode optimizations are
        allowed to break this */
        return *(*book).codelist.offset(entry as isize) as isize;
    }
    return -1isize;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_codelen(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut entry: i32,
) -> isize {
    if !(*book).c.is_null() {
        /* only use with encode; decode optimizations are
        allowed to break this */
        return *(*(*book).c).lengthlist.offset(entry as isize) as isize;
    }
    return -1isize;
}
