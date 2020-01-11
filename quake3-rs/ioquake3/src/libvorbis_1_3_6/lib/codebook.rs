// =============== BEGIN codebook_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_codebook {
    pub dim: isize,
    pub entries: isize,
    pub lengthlist: *mut i8,
    pub maptype: i32,
    pub q_min: isize,
    pub q_delta: isize,
    pub q_quant: i32,
    pub q_sequencep: i32,
    pub quantlist: *mut isize,
    pub allocedp: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codebook {
    pub dim: isize,
    pub entries: isize,
    pub used_entries: isize,
    pub c: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    pub valuelist: *mut f32,
    pub codelist: *mut crate::config_types_h::ogg_uint32_t,
    pub dec_index: *mut i32,
    pub dec_codelengths: *mut i8,
    pub dec_firsttable: *mut crate::config_types_h::ogg_uint32_t,
    pub dec_firsttablen: i32,
    pub dec_maxlength: i32,
    pub quantvals: i32,
    pub minval: i32,
    pub delta: i32,
}

pub use crate::config_types_h::ogg_uint32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_adv;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_look;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals;

pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy;

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

function: basic codebook pack/unpack/code/decode operations

********************************************************************/
/* packs the given codebook into the bitstream **************************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_staticbook_pack(
    mut c: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> i32 {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut ordered: i32 = 0;
    /* first the basic parameters */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0x564342i32 as usize, 24);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*c).dim as usize, 16);
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*c).entries as usize, 24);
    /* pack the codewords.  There are two packings; length ordered and
    length random.  Decide between the two now. */
    i = 1;
    while i < (*c).entries {
        if *(*c).lengthlist.offset(i - 1) as i32 == 0
            || (*(*c).lengthlist.offset(i) as i32) < *(*c).lengthlist.offset(i - 1) as i32
        {
            break;
        }
        i += 1
    }
    if i == (*c).entries {
        ordered = 1
    }
    if ordered != 0 {
        /* length ordered.  We only need to say how many codewords of
        each length.  The actual codewords are generated
        deterministically */
        let mut count: isize = 0; /* ordered */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1); /* 1 to 32 */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            (*(*c).lengthlist.offset(0) as i32 - 1) as usize,
            5,
        );
        i = 1;
        while i < (*c).entries {
            let mut this: i8 = *(*c).lengthlist.offset(i);
            let mut last: i8 = *(*c).lengthlist.offset(i - 1);
            if this as i32 > last as i32 {
                j = last as isize;
                while j < this as isize {
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb,
                        (i - count) as usize,
                        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                            ((*c).entries - count) as crate::config_types_h::ogg_uint32_t,
                        ),
                    );
                    count = i;
                    j += 1
                }
            }
            i += 1
        }
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb,
            (i - count) as usize,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*c).entries - count) as crate::config_types_h::ogg_uint32_t,
            ),
        );
    } else {
        /* length random.  Again, we don't code the codeword itself, just
        the length.  This time, though, we have to encode each length */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0, 1); /* unordered */
        /* algortihmic mapping has use for 'unused entries', which we tag
        here.  The algorithmic mapping happens as usual, but the unused
        entry has no codeword. */
        i = 0; /* no unused entries */
        while i < (*c).entries {
            if *(*c).lengthlist.offset(i) as i32 == 0 {
                break; /* we have unused entries; thus we tag */
            }
            i += 1
        }
        if i == (*c).entries {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0, 1);
            i = 0;
            while i < (*c).entries {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb,
                    (*(*c).lengthlist.offset(i) as i32 - 1) as usize,
                    5,
                );
                i += 1
            }
        } else {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1);
            i = 0;
            while i < (*c).entries {
                if *(*c).lengthlist.offset(i) as i32 == 0 {
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 0usize, 1i32);
                } else {
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, 1, 1);
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                        opb,
                        (*(*c).lengthlist.offset(i) as i32 - 1i32) as usize,
                        5i32,
                    );
                }
                i += 1
            }
        }
    }
    /* is the entry number the desired return value, or do we have a
    mapping? If we have a mapping, what type? */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*c).maptype as usize, 4);
    match (*c).maptype {
        0 => {}
        1 | 2 => {
            /* implicitly populated value mapping */
            /* explicitly populated value mapping */
            if (*c).quantlist.is_null() {
                /* no quantlist?  error */
                return -(1i32);
            }
            /* values that define the dequantization */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*c).q_min as usize, 32);
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(opb, (*c).q_delta as usize, 32);
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb,
                ((*c).q_quant - 1i32) as usize,
                4,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb,
                (*c).q_sequencep as usize,
                1,
            );
            let mut quantvals: i32 = 0;
            match (*c).maptype {
                1 => {
                    /* a single column of (c->entries/c->dim) quantized values for
                    building a full value list algorithmically (square lattice) */
                    quantvals =
                        crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(c)
                            as i32
                }
                2 => {
                    /* every value (c->entries*c->dim total) specified explicitly */
                    quantvals = ((*c).entries * (*c).dim) as i32
                }
                _ => {
                    /* NOT_REACHABLE */
                    quantvals = -(1)
                }
            }
            /* quantized values */
            i = 0;
            while i < quantvals as isize {
                crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                    opb,
                    crate::stdlib::labs(*(*c).quantlist.offset(i)) as usize,
                    (*c).q_quant,
                );
                i += 1
            }
        }
        _ => {
            /* error case; we don't have any other map types now */
            return -(1i32);
        }
    }
    return 0;
}
/* unpacks a codebook from the packet buffer into the codebook struct,
readies the codebook auxiliary structures for decode *************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_staticbook_unpack(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook {
    let mut current_block: u64;
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut s: *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook =
        crate::stdlib::calloc(
            1,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::codebook::static_codebook>(),
        ) as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
    (*s).allocedp = 1;
    /* make sure alignment is correct */
    if !(crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 24) != 0x564342i32 as isize) {
        /* first the basic parameters */
        (*s).dim = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 16);
        (*s).entries = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 24);
        if !((*s).entries == -1) {
            if !(crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                (*s).dim as crate::config_types_h::ogg_uint32_t,
            ) + crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                (*s).entries as crate::config_types_h::ogg_uint32_t,
            ) > 24)
            {
                /* codeword ordering.... length ordered or unordered? */
                match crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1) as i32 {
                    0 => {
                        current_block = 14523784380283086299;
                        match current_block {
                            14523784380283086299 => {
                                let mut unused: isize = 0;
                                /* allocated but unused entries? */
                                unused =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1);
                                if (*s).entries * (if unused != 0 { 1i32 } else { 5 }) as isize + 7
                                    >> 3
                                    > (*opb).storage
                                        - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                                {
                                    current_block = 15187751986642917127;
                                } else {
                                    /* unordered */
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<i8>())
                                            .wrapping_mul((*s).entries as usize),
                                    )
                                        as *mut i8;
                                    /* allocated but unused entries? */
                                    if unused != 0 {
                                        /* yes, unused entries */
                                        i = 0;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                                opb, 1,
                                            ) != 0
                                            {
                                                let mut num: isize =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 5);
                                                if num == -1 {
                                                    current_block = 15187751986642917127;
                                                    break;
                                                }
                                                *(*s).lengthlist.offset(i) = (num + 1) as i8
                                            } else {
                                                *(*s).lengthlist.offset(i) = 0i8
                                            }
                                            i += 1
                                        }
                                    } else {
                                        /* all entries used; no tagging */
                                        i = 0;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            let mut num_0: isize =
                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                             5);
                                            if num_0 == -1 {
                                                current_block = 15187751986642917127;
                                                break;
                                            }
                                            *(*s).lengthlist.offset(i) = (num_0 + 1) as i8;
                                            i += 1
                                        }
                                    }
                                }
                            }
                            _ =>
                            /* ordered */
                            {
                                let mut length: isize =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 5)
                                        + 1;
                                if length == 0 {
                                    current_block = 15187751986642917127;
                                } else {
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<i8>())
                                            .wrapping_mul((*s).entries as usize),
                                    )
                                        as *mut i8;
                                    i = 0;
                                    loop {
                                        if !(i < (*s).entries) {
                                            current_block = 15004371738079956865;
                                            break;
                                        }
                                        let mut num_1: isize =
                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                         crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(((*s).entries
                                                                      - i) as
                                                                     crate::config_types_h::ogg_uint32_t));
                                        if num_1 == -1 {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32
                                            || num_1 > (*s).entries - i
                                            || num_1 > 0 && num_1 - 1 >> length - 1 > 1
                                        {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32 {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        j = 0;
                                        while j < num_1 {
                                            *(*s).lengthlist.offset(i) = length as i8;
                                            j += 1;
                                            i += 1
                                        }
                                        length += 1
                                    }
                                }
                            }
                        }
                        match current_block {
                            15187751986642917127 => {}
                            _ =>
                            /* Do we have a mapping to unpack? */
                            {
                                (*s).maptype =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4)
                                        as i32;
                                match (*s).maptype {
                                    0 => {
                                        current_block = 317151059986244064;
                                        match current_block {
                                            9333025334031379274 => {
                                                /* implicitly populated value mapping */
                                                /* explicitly populated value mapping */
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                  4)
                                                         +
                                                         1) as
                                                        i32;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 1)
                                                        as i32;
                                                if (*s).q_sequencep == -(1) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: i32 = 0;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim == 0 {
                                                                0
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s)
                                                            }
                                                                as i32
                                                        }
                                                        2 => {
                                                            quantvals =
                                                                ((*s).entries * (*s).dim) as i32
                                                        }
                                                        _ => {}
                                                    }
                                                    /* quantized values */
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7
                                                            >>
                                                            3)
                                                           as isize >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<isize>()).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        usize))
                                                                as
                                                                *mut isize;
                                                        i =
                                                            0;
                                                        while i <
                                                                  quantvals as
                                                                      isize
                                                              {
                                                            *(*s).quantlist.offset(i)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -1isize
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ =>
                                            /* no mapping */
                                            /* all set */
                                            {
                                                return s
                                            }
                                        }
                                    }
                                    1 | 2 => {
                                        current_block = 9333025334031379274;
                                        match current_block {
                                            9333025334031379274 => {
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                  4)
                                                         +
                                                         1) as
                                                        i32;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 1)
                                                        as i32;
                                                if (*s).q_sequencep == -(1) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: i32 = 0;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim == 0 {
                                                                0
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s)
                                                            }
                                                                as i32
                                                        }
                                                        2 => {
                                                            quantvals =
                                                                ((*s).entries * (*s).dim) as i32
                                                        }
                                                        _ => {}
                                                    }
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7
                                                            >>
                                                            3)
                                                           as isize >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<isize>()).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        usize))
                                                                as
                                                                *mut isize;
                                                        i =
                                                            0;
                                                        while i <
                                                                  quantvals as
                                                                      isize
                                                              {
                                                            *(*s).quantlist.offset(i)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -1isize
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ => return s,
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 3437258052017859086;
                        match current_block {
                            14523784380283086299 => {
                                let mut unused: isize = 0;
                                unused =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 1);
                                if (*s).entries * (if unused != 0 { 1i32 } else { 5 }) as isize + 7
                                    >> 3
                                    > (*opb).storage
                                        - crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                                {
                                    current_block = 15187751986642917127;
                                } else {
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<i8>())
                                            .wrapping_mul((*s).entries as usize),
                                    )
                                        as *mut i8;
                                    if unused != 0 {
                                        i = 0;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                                opb, 1,
                                            ) != 0
                                            {
                                                let mut num: isize =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 5);
                                                if num == -1 {
                                                    current_block = 15187751986642917127;
                                                    break;
                                                }
                                                *(*s).lengthlist.offset(i) = (num + 1) as i8
                                            } else {
                                                *(*s).lengthlist.offset(i) = 0i8
                                            }
                                            i += 1
                                        }
                                    } else {
                                        i = 0;
                                        loop {
                                            if !(i < (*s).entries) {
                                                current_block = 15004371738079956865;
                                                break;
                                            }
                                            let mut num_0: isize =
                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                             5);
                                            if num_0 == -1 {
                                                current_block = 15187751986642917127;
                                                break;
                                            }
                                            *(*s).lengthlist.offset(i) = (num_0 + 1) as i8;
                                            i += 1
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut length: isize =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 5)
                                        + 1;
                                if length == 0 {
                                    current_block = 15187751986642917127;
                                } else {
                                    (*s).lengthlist = crate::stdlib::malloc(
                                        (::std::mem::size_of::<i8>())
                                            .wrapping_mul((*s).entries as usize),
                                    )
                                        as *mut i8;
                                    i = 0;
                                    loop {
                                        if !(i < (*s).entries) {
                                            current_block = 15004371738079956865;
                                            break;
                                        }
                                        let mut num_1: isize =
                                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                         crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(((*s).entries
                                                                      - i) as
                                                                     crate::config_types_h::ogg_uint32_t));
                                        if num_1 == -1 {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32
                                            || num_1 > (*s).entries - i
                                            || num_1 > 0 && num_1 - 1 >> length - 1 > 1
                                        {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        if length > 32 {
                                            current_block = 15187751986642917127;
                                            break;
                                        }
                                        j = 0;
                                        while j < num_1 {
                                            *(*s).lengthlist.offset(i) = length as i8;
                                            j += 1;
                                            i += 1
                                        }
                                        length += 1
                                    }
                                }
                            }
                        }
                        match current_block {
                            15187751986642917127 => {}
                            _ => {
                                (*s).maptype =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb, 4)
                                        as i32;
                                match (*s).maptype {
                                    0 => {
                                        current_block = 317151059986244064;
                                        match current_block {
                                            9333025334031379274 => {
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                  4)
                                                         +
                                                         1) as
                                                        i32;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 1)
                                                        as i32;
                                                if (*s).q_sequencep == -(1) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: i32 = 0;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim == 0 {
                                                                0
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s)
                                                            }
                                                                as i32
                                                        }
                                                        2 => {
                                                            quantvals =
                                                                ((*s).entries * (*s).dim) as i32
                                                        }
                                                        _ => {}
                                                    }
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7
                                                            >>
                                                            3)
                                                           as isize >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<isize>()).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        usize))
                                                                as
                                                                *mut isize;
                                                        i =
                                                            0;
                                                        while i <
                                                                  quantvals as
                                                                      isize
                                                              {
                                                            *(*s).quantlist.offset(i)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -1isize
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ => return s,
                                        }
                                    }
                                    1 | 2 => {
                                        current_block = 9333025334031379274;
                                        match current_block {
                                            9333025334031379274 => {
                                                (*s).q_min =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_delta =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 32);
                                                (*s).q_quant =
                                                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                  4)
                                                         +
                                                         1) as
                                                        i32;
                                                (*s).q_sequencep =
                                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                 1)
                                                        as i32;
                                                if (*s).q_sequencep == -(1) {
                                                    current_block = 15187751986642917127;
                                                } else {
                                                    let mut quantvals: i32 = 0;
                                                    match (*s).maptype {
                                                        1 => {
                                                            quantvals = if (*s).dim == 0 {
                                                                0
                                                            } else {
                                                                crate::src::libvorbis_1_3_6::lib::sharedbook::_book_maptype1_quantvals(s)
                                                            }
                                                                as i32
                                                        }
                                                        2 => {
                                                            quantvals =
                                                                ((*s).entries * (*s).dim) as i32
                                                        }
                                                        _ => {}
                                                    }
                                                    if (quantvals *
                                                            (*s).q_quant +
                                                            7
                                                            >>
                                                            3)
                                                           as isize >
                                                           (*opb).storage -
                                                               crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(opb)
                                                       {
                                                        current_block =
                                                            15187751986642917127;
                                                    } else {
                                                        (*s).quantlist =
                                                            crate::stdlib::malloc((::std::mem::size_of::<isize>()).wrapping_mul(quantvals
                                                                                                        as
                                                                                                        usize))
                                                                as
                                                                *mut isize;
                                                        i =
                                                            0;
                                                        while i <
                                                                  quantvals as
                                                                      isize
                                                              {
                                                            *(*s).quantlist.offset(i)
                                                                =
                                                                crate::src::libogg_1_3_3::src::bitwise::oggpack_read(opb,
                                                                             (*s).q_quant);
                                                            i += 1
                                                        }
                                                        if quantvals != 0 &&
                                                               *(*s).quantlist.offset((quantvals
                                                                                           -
                                                                                           1)
                                                                                          as
                                                                                          isize)
                                                                   ==
                                                                   -1isize
                                                           {
                                                            current_block =
                                                                15187751986642917127;
                                                        } else {
                                                            current_block =
                                                                317151059986244064;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            15187751986642917127 => {}
                                            _ => return s,
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    /* EOF */
    crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy(s);
    return 0 as *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
}
/* returns the number of bits ************************************************/
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_encode(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: i32,
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> i32 {
    if a < 0 || a as isize >= (*(*book).c).entries {
        return 0i32;
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        b,
        *(*book).codelist.offset(a as isize) as usize,
        *(*(*book).c).lengthlist.offset(a as isize) as i32,
    );
    return *(*(*book).c).lengthlist.offset(a as isize) as i32;
}
/* the 'eliminate the decode tree' optimization actually requires the
codewords to be MSb first, not LSb.  This is an annoying inelegancy
(and one of the first places where carefully thought out design
turned out to be wrong; Vorbis II and future Ogg codecs should go
to an MSb bitpacker), but not actually the huge hit it appears to
be.  The first-stage decode table catches most words so that
bitreverse is not in the main execution path. */

unsafe extern "C" fn bitreverse(
    mut x: crate::config_types_h::ogg_uint32_t,
) -> crate::config_types_h::ogg_uint32_t {
    x = x >> 16 & 0xffffu32 | x << 16 & 0xffff0000;
    x = x >> 8 & 0xff00ffu32 | x << 8 & 0xff00ff00;
    x = x >> 4 & 0xf0f0f0fu32 | x << 4 & 0xf0f0f0f0;
    x = x >> 2 & 0x33333333u32 | x << 2 & 0xcccccccc;
    return x >> 1 & 0x55555555u32 | x << 1 & 0xaaaaaaaa;
}
#[inline]

unsafe extern "C" fn decode_packed_entry_number(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> isize {
    let mut read: i32 = (*book).dec_maxlength;
    let mut lo: isize = 0;
    let mut hi: isize = 0;
    let mut lok: isize =
        crate::src::libogg_1_3_3::src::bitwise::oggpack_look(b, (*book).dec_firsttablen);
    if lok >= 0 {
        let mut entry: isize = *(*book).dec_firsttable.offset(lok) as isize;
        if entry as usize & 0x80000000 as usize != 0 {
            lo = entry >> 15 & 0x7fff;
            hi = (*book).used_entries - (entry & 0x7fff)
        } else {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_adv(
                b,
                *(*book).dec_codelengths.offset(entry - 1) as i32,
            );
            return entry - 1isize;
        }
    } else {
        lo = 0;
        hi = (*book).used_entries
    }
    /* Single entry codebooks use a firsttablen of 1 and a
    dec_maxlength of 1.  If a single-entry codebook gets here (due to
    failure to read one bit above), the next look attempt will also
    fail and we'll correctly kick out instead of trying to walk the
    underformed tree */
    lok = crate::src::libogg_1_3_3::src::bitwise::oggpack_look(b, read);
    while lok < 0 && read > 1 {
        read -= 1;
        lok = crate::src::libogg_1_3_3::src::bitwise::oggpack_look(b, read)
    }
    if lok < 0 {
        return -1isize;
    }
    /* bisect search for the codeword in the ordered list */
    let mut testword: crate::config_types_h::ogg_uint32_t =
        bitreverse(lok as crate::config_types_h::ogg_uint32_t);
    while hi - lo > 1 {
        let mut p: isize = hi - lo >> 1;
        let mut test: isize = (*(*book).codelist.offset(lo + p) > testword) as i32 as isize;
        lo += p & test - 1;
        hi -= p & -test
    }
    if *(*book).dec_codelengths.offset(lo) as i32 <= read {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_adv(
            b,
            *(*book).dec_codelengths.offset(lo) as i32,
        );
        return lo;
    }
    crate::src::libogg_1_3_3::src::bitwise::oggpack_adv(b, read);
    return -1isize;
}
/* Decode side is specced and easier, because we don't need to find
matches using different criteria; we simply read and map.  There are
two things we need to do 'depending':

We may need to support interleave.  We don't really, but it's
convenient to do it here rather than rebuild the vector later.

Cascades may be additive or multiplicitive; this is not inherent in
the codebook, but set in the code using the codebook.  Like
interleaving, it's easiest to do it here.
addmul==0 -> declarative (set the value)
addmul==1 -> additive
addmul==2 -> multiplicitive */
/* returns the [original, not compacted] entry number or -1 on eof *********/
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decode(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut b: *mut crate::ogg_h::oggpack_buffer,
) -> isize {
    if (*book).used_entries > 0isize {
        let mut packed_entry: isize = decode_packed_entry_number(book, b);
        if packed_entry >= 0 {
            return *(*book).dec_index.offset(packed_entry) as isize;
        }
    }
    /* if there's no dec_index, the codebook unpacking isn't collapsed */
    return -1isize;
}
/* returns 0 on OK or -1 on eof *************************************/
/* decode vector / dim granularity gaurding is done in the upper layer */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodevs_add(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut f32,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: i32,
) -> isize {
    if (*book).used_entries > 0isize {
        let mut step: i32 = (n as isize / (*book).dim) as i32;
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<isize>()).wrapping_mul(step as usize),
        );
        let mut entry: *mut isize = fresh0.as_mut_ptr() as *mut isize;
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<*mut f32>()).wrapping_mul(step as usize),
        );
        let mut t: *mut *mut f32 = fresh1.as_mut_ptr() as *mut *mut f32;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut o: i32 = 0;
        i = 0;
        while i < step {
            *entry.offset(i as isize) = decode_packed_entry_number(book, b);
            if *entry.offset(i as isize) == -1isize {
                return -1isize;
            }
            let ref mut fresh2 = *t.offset(i as isize);
            *fresh2 = (*book)
                .valuelist
                .offset(*entry.offset(i as isize) * (*book).dim);
            i += 1
        }
        i = 0;
        o = 0;
        while (i as isize) < (*book).dim {
            j = 0;
            while o + j < n && j < step {
                *a.offset((o + j) as isize) += *(*t.offset(j as isize)).offset(i as isize);
                j += 1
            }
            i += 1;
            o += step
        }
    }
    return 0isize;
}
/* decode vector / dim granularity gaurding is done in the upper layer */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodev_add(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut f32,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: i32,
) -> isize {
    if (*book).used_entries > 0isize {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut entry: i32 = 0;
        let mut t: *mut f32 = 0 as *mut f32;
        i = 0;
        while i < n {
            entry = decode_packed_entry_number(book, b) as i32;
            if entry == -(1) {
                return -1isize;
            }
            t = (*book).valuelist.offset(entry as isize * (*book).dim);
            j = 0;
            while i < n && (j as isize) < (*book).dim {
                let fresh3 = j;
                j = j + 1;
                let fresh4 = i;
                i = i + 1;
                *a.offset(fresh4 as isize) += *t.offset(fresh3 as isize)
            }
        }
    }
    return 0isize;
}
/* unlike the others, we guard against n not being an integer number
of <dim> internally rather than in the upper layer (called only by
floor0) */
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodev_set(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut f32,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: i32,
) -> isize {
    if (*book).used_entries > 0isize {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut entry: i32 = 0;
        let mut t: *mut f32 = 0 as *mut f32;
        i = 0;
        while i < n {
            entry = decode_packed_entry_number(book, b) as i32;
            if entry == -(1) {
                return -1isize;
            }
            t = (*book).valuelist.offset(entry as isize * (*book).dim);
            j = 0;
            while i < n && (j as isize) < (*book).dim {
                let fresh5 = j;
                j = j + 1;
                let fresh6 = i;
                i = i + 1;
                *a.offset(fresh6 as isize) = *t.offset(fresh5 as isize)
            }
        }
    } else {
        let mut i_0: i32 = 0;
        i_0 = 0;
        while i_0 < n {
            let fresh7 = i_0;
            i_0 = i_0 + 1;
            *a.offset(fresh7 as isize) = 0.0f32
        }
    }
    return 0isize;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_book_decodevv_add(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut *mut f32,
    mut offset: isize,
    mut ch: i32,
    mut b: *mut crate::ogg_h::oggpack_buffer,
    mut n: i32,
) -> isize {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut entry: isize = 0;
    let mut chptr: i32 = 0;
    if (*book).used_entries > 0isize {
        let mut m: i32 = ((offset + n as isize) / ch as isize) as i32;
        i = offset / ch as isize;
        while i < m as isize {
            entry = decode_packed_entry_number(book, b);
            if entry == -1 {
                return -1isize;
            }
            let mut t: *const f32 = (*book).valuelist.offset(entry * (*book).dim);
            j = 0;
            while i < m as isize && j < (*book).dim {
                let fresh8 = chptr;
                chptr = chptr + 1;
                *(*a.offset(fresh8 as isize)).offset(i) += *t.offset(j);
                if chptr == ch {
                    chptr = 0;
                    i += 1
                }
                j += 1
            }
        }
    }
    return 0isize;
}
