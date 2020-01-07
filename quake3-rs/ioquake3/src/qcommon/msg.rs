use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::huff_t;
pub use crate::qcommon_h::huffman_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::node_t;
pub use crate::qcommon_h::nodetype;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::huffman::Huff_Init;
pub use crate::src::qcommon::huffman::Huff_addRef;
pub use crate::src::qcommon::huffman::Huff_getBit;
pub use crate::src::qcommon::huffman::Huff_getBloc;
pub use crate::src::qcommon::huffman::Huff_offsetReceive;
pub use crate::src::qcommon::huffman::Huff_offsetTransmit;
pub use crate::src::qcommon::huffman::Huff_putBit;
pub use crate::src::qcommon::huffman::Huff_setBloc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::strlen;
extern "C" {
    #[no_mangle]
    pub static mut cl_shownet: *mut crate::src::qcommon::q_shared::cvar_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netField_t {
    pub name: *mut i8,
    pub offset: i32,
    pub bits: i32,
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/

static mut msgHuff: crate::qcommon_h::huffman_t = crate::qcommon_h::huffman_t {
    compressor: crate::qcommon_h::huff_t {
        blocNode: 0,
        blocPtrs: 0,
        tree: 0 as *mut crate::qcommon_h::node_t,
        lhead: 0 as *mut crate::qcommon_h::node_t,
        ltail: 0 as *mut crate::qcommon_h::node_t,
        loc: [0 as *mut crate::qcommon_h::node_t; 257],
        freelist: 0 as *mut *mut crate::qcommon_h::node_t,
        nodeList: [crate::qcommon_h::node_t {
            left: 0 as *mut crate::qcommon_h::nodetype,
            right: 0 as *mut crate::qcommon_h::nodetype,
            parent: 0 as *mut crate::qcommon_h::nodetype,
            next: 0 as *mut crate::qcommon_h::nodetype,
            prev: 0 as *mut crate::qcommon_h::nodetype,
            head: 0 as *mut *mut crate::qcommon_h::nodetype,
            weight: 0,
            symbol: 0,
        }; 768],
        nodePtrs: [0 as *mut crate::qcommon_h::node_t; 768],
    },
    decompressor: crate::qcommon_h::huff_t {
        blocNode: 0,
        blocPtrs: 0,
        tree: 0 as *mut crate::qcommon_h::node_t,
        lhead: 0 as *mut crate::qcommon_h::node_t,
        ltail: 0 as *mut crate::qcommon_h::node_t,
        loc: [0 as *mut crate::qcommon_h::node_t; 257],
        freelist: 0 as *mut *mut crate::qcommon_h::node_t,
        nodeList: [crate::qcommon_h::node_t {
            left: 0 as *mut crate::qcommon_h::nodetype,
            right: 0 as *mut crate::qcommon_h::nodetype,
            parent: 0 as *mut crate::qcommon_h::nodetype,
            next: 0 as *mut crate::qcommon_h::nodetype,
            prev: 0 as *mut crate::qcommon_h::nodetype,
            head: 0 as *mut *mut crate::qcommon_h::nodetype,
            weight: 0,
            symbol: 0,
        }; 768],
        nodePtrs: [0 as *mut crate::qcommon_h::node_t; 768],
    },
};

static mut msgInit: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut pcount: [i32; 256] = [0; 256];
/*
==============================================================================

            MESSAGE IO FUNCTIONS

Handles byte ordering and avoids alignment errors
==============================================================================
*/
#[no_mangle]

pub static mut oldsize: i32 = 0;
#[no_mangle]

pub unsafe extern "C" fn MSG_Init(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut length: i32,
) {
    if msgInit as u64 == 0 {
        MSG_initHuffman();
    }
    crate::stdlib::memset(
        buf as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::qcommon_h::msg_t>(),
    );
    (*buf).data = data;
    (*buf).maxsize = length;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_InitOOB(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut length: i32,
) {
    if msgInit as u64 == 0 {
        MSG_initHuffman();
    }
    crate::stdlib::memset(
        buf as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::qcommon_h::msg_t>(),
    );
    (*buf).data = data;
    (*buf).maxsize = length;
    (*buf).oob = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_Clear(mut buf: *mut crate::qcommon_h::msg_t) {
    (*buf).cursize = 0;
    (*buf).overflowed = crate::src::qcommon::q_shared::qfalse;
    (*buf).bit = 0;
    //<- in bits
}
#[no_mangle]

pub unsafe extern "C" fn MSG_Bitstream(mut buf: *mut crate::qcommon_h::msg_t) {
    (*buf).oob = crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_BeginReading(mut msg: *mut crate::qcommon_h::msg_t) {
    (*msg).readcount = 0;
    (*msg).bit = 0;
    (*msg).oob = crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_BeginReadingOOB(mut msg: *mut crate::qcommon_h::msg_t) {
    (*msg).readcount = 0;
    (*msg).bit = 0;
    (*msg).oob = crate::src::qcommon::q_shared::qtrue;
}
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
#[no_mangle]

pub unsafe extern "C" fn MSG_Copy(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut length: i32,
    mut src: *mut crate::qcommon_h::msg_t,
) {
    if length < (*src).cursize {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"MSG_Copy: can\'t copy into a smaller msg_t buffer\x00" as *const u8 as *const i8,
        );
    }
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<crate::qcommon_h::msg_t>(),
    );
    (*buf).data = data;
    crate::stdlib::memcpy(
        (*buf).data as *mut libc::c_void,
        (*src).data as *const libc::c_void,
        (*src).cursize as usize,
    );
}
/*
=============================================================================

bit functions

=============================================================================
*/
// negative bit values include signs
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteBits(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut value: i32,
    mut bits: i32,
) {
    let mut i: i32 = 0;
    oldsize += bits;
    if (*msg).overflowed as u64 != 0 {
        return;
    }
    if bits == 0 || bits < -(31) || bits > 32 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"MSG_WriteBits: bad bits %i\x00" as *const u8 as *const i8,
            bits,
        );
    }
    if bits < 0 {
        bits = -bits
    }
    if (*msg).oob as u64 != 0 {
        if (*msg).cursize + (bits >> 3) > (*msg).maxsize {
            (*msg).overflowed = crate::src::qcommon::q_shared::qtrue;
            return;
        }
        if bits == 8 {
            *(*msg).data.offset((*msg).cursize as isize) =
                value as crate::src::qcommon::q_shared::byte;
            (*msg).cursize += 1;
            (*msg).bit += 8
        } else if bits == 16 {
            let mut temp: i16 = value as i16;
            crate::stdlib::memcpy(
                &mut *(*msg).data.offset((*msg).cursize as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *mut libc::c_void,
                &mut temp as *mut i16 as *const libc::c_void,
                2,
            );
            (*msg).cursize += 2;
            (*msg).bit += 16
        } else if bits == 32 {
            crate::stdlib::memcpy(
                &mut *(*msg).data.offset((*msg).cursize as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *mut libc::c_void,
                &mut value as *mut i32 as *const libc::c_void,
                4,
            );
            (*msg).cursize += 4;
            (*msg).bit += 32
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"can\'t write %d bits\x00" as *const u8 as *const i8,
                bits,
            );
        }
    } else {
        value = (value as u32 & 0xffffffff >> 32 - bits) as i32;
        if bits & 7 != 0 {
            let mut nbits: i32 = 0;
            nbits = bits & 7;
            if (*msg).bit + nbits > (*msg).maxsize << 3 {
                (*msg).overflowed = crate::src::qcommon::q_shared::qtrue;
                return;
            }
            i = 0;
            while i < nbits {
                crate::src::qcommon::huffman::Huff_putBit(value & 1, (*msg).data, &mut (*msg).bit);
                value = value >> 1;
                i += 1
            }
            bits = bits - nbits
        }
        if bits != 0 {
            i = 0;
            while i < bits {
                crate::src::qcommon::huffman::Huff_offsetTransmit(
                    &mut msgHuff.compressor,
                    value & 0xff,
                    (*msg).data,
                    &mut (*msg).bit,
                    (*msg).maxsize << 3,
                );
                value = value >> 8;
                if (*msg).bit > (*msg).maxsize << 3 {
                    (*msg).overflowed = crate::src::qcommon::q_shared::qtrue;
                    return;
                }
                i += 8
            }
        }
        (*msg).cursize = ((*msg).bit >> 3) + 1
    };
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadBits(mut msg: *mut crate::qcommon_h::msg_t, mut bits: i32) -> i32 {
    let mut value: i32 = 0;
    let mut get: i32 = 0;
    let mut sgn: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut i: i32 = 0;
    let mut nbits: i32 = 0;
    //	FILE*	fp;
    if (*msg).readcount > (*msg).cursize {
        return 0i32;
    }
    value = 0;
    if bits < 0 {
        bits = -bits;
        sgn = crate::src::qcommon::q_shared::qtrue
    } else {
        sgn = crate::src::qcommon::q_shared::qfalse
    }
    if (*msg).oob as u64 != 0 {
        if (*msg).readcount + (bits >> 3) > (*msg).cursize {
            (*msg).readcount = (*msg).cursize + 1;
            return 0i32;
        }
        if bits == 8 {
            value = *(*msg).data.offset((*msg).readcount as isize) as i32;
            (*msg).readcount += 1;
            (*msg).bit += 8
        } else if bits == 16 {
            let mut temp: i16 = 0;
            crate::stdlib::memcpy(
                &mut temp as *mut i16 as *mut libc::c_void,
                &mut *(*msg).data.offset((*msg).readcount as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *const libc::c_void,
                2,
            );
            value = temp as i32;
            (*msg).readcount += 2;
            (*msg).bit += 16
        } else if bits == 32 {
            crate::stdlib::memcpy(
                &mut value as *mut i32 as *mut libc::c_void,
                &mut *(*msg).data.offset((*msg).readcount as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *const libc::c_void,
                4,
            );
            (*msg).readcount += 4;
            (*msg).bit += 32
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"can\'t read %d bits\x00" as *const u8 as *const i8,
                bits,
            );
        }
    } else {
        nbits = 0;
        if bits & 7 != 0 {
            nbits = bits & 7;
            if (*msg).bit + nbits > (*msg).cursize << 3 {
                (*msg).readcount = (*msg).cursize + 1;
                return 0i32;
            }
            i = 0;
            while i < nbits {
                value |=
                    crate::src::qcommon::huffman::Huff_getBit((*msg).data, &mut (*msg).bit) << i;
                i += 1
            }
            bits = bits - nbits
        }
        if bits != 0 {
            //			fp = fopen("c:\\netchan.bin", "a");
            i = 0;
            while i < bits {
                crate::src::qcommon::huffman::Huff_offsetReceive(
                    msgHuff.decompressor.tree,
                    &mut get,
                    (*msg).data,
                    &mut (*msg).bit,
                    (*msg).cursize << 3,
                );
                //				fwrite(&get, 1, 1, fp);
                value = (value as u32 | (get as u32) << i + nbits) as i32;
                if (*msg).bit > (*msg).cursize << 3 {
                    (*msg).readcount = (*msg).cursize + 1;
                    return 0i32;
                }
                i += 8
            }
            //			fclose(fp);
        }
        (*msg).readcount = ((*msg).bit >> 3) + 1
    }
    if sgn != 0 && bits > 0 && bits < 32 {
        if value & (1) << bits - 1 != 0 {
            value |= -(1) ^ ((1) << bits) - 1
        }
    }
    return value;
}
//================================================================================
//
// writing functions
//
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteChar(mut sb: *mut crate::qcommon_h::msg_t, mut c: i32) {
    MSG_WriteBits(sb, c, 8);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteByte(mut sb: *mut crate::qcommon_h::msg_t, mut c: i32) {
    MSG_WriteBits(sb, c, 8);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteData(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *const libc::c_void,
    mut length: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < length {
        MSG_WriteByte(
            buf,
            *(data as *mut crate::src::qcommon::q_shared::byte).offset(i as isize) as i32,
        );
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteShort(mut sb: *mut crate::qcommon_h::msg_t, mut c: i32) {
    MSG_WriteBits(sb, c, 16);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteLong(mut sb: *mut crate::qcommon_h::msg_t, mut c: i32) {
    MSG_WriteBits(sb, c, 32);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteFloat(mut sb: *mut crate::qcommon_h::msg_t, mut f: f32) {
    let mut dat: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    dat.f = f;
    MSG_WriteBits(sb, dat.i, 32);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteString(mut sb: *mut crate::qcommon_h::msg_t, mut s: *const i8) {
    if s.is_null() {
        MSG_WriteData(sb, b"\x00" as *const u8 as *const libc::c_void, 1i32);
    } else {
        let mut l: i32 = 0;
        let mut i: i32 = 0;
        let mut string: [i8; 1024] = [0; 1024];
        l = crate::stdlib::strlen(s) as i32;
        if l >= 1024 {
            crate::src::qcommon::common::Com_Printf(
                b"MSG_WriteString: MAX_STRING_CHARS\x00" as *const u8 as *const i8,
            );
            MSG_WriteData(sb, b"\x00" as *const u8 as *const libc::c_void, 1);
            return;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            string.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        // get rid of 0x80+ and '%' chars, because old clients don't like them
        i = 0;
        while i < l {
            if *(string.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte).offset(i as isize)
                as i32
                > 127
                || string[i as usize] as i32 == '%' as i32
            {
                string[i as usize] = '.' as i8
            }
            i += 1
        }
        MSG_WriteData(sb, string.as_mut_ptr() as *const libc::c_void, l + 1i32);
    };
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteBigString(
    mut sb: *mut crate::qcommon_h::msg_t,
    mut s: *const i8,
) {
    if s.is_null() {
        MSG_WriteData(sb, b"\x00" as *const u8 as *const libc::c_void, 1i32);
    } else {
        let mut l: i32 = 0;
        let mut i: i32 = 0;
        let mut string: [i8; 8192] = [0; 8192];
        l = crate::stdlib::strlen(s) as i32;
        if l >= 8192 {
            crate::src::qcommon::common::Com_Printf(
                b"MSG_WriteString: BIG_INFO_STRING\x00" as *const u8 as *const i8,
            );
            MSG_WriteData(sb, b"\x00" as *const u8 as *const libc::c_void, 1);
            return;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            string.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[i8; 8192]>() as i32,
        );
        // get rid of 0x80+ and '%' chars, because old clients don't like them
        i = 0;
        while i < l {
            if *(string.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte).offset(i as isize)
                as i32
                > 127
                || string[i as usize] as i32 == '%' as i32
            {
                string[i as usize] = '.' as i8
            }
            i += 1
        }
        MSG_WriteData(sb, string.as_mut_ptr() as *const libc::c_void, l + 1i32);
    };
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteAngle(mut sb: *mut crate::qcommon_h::msg_t, mut f: f32) {
    MSG_WriteByte(sb, (f * 256f32 / 360f32) as i32 & 255);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteAngle16(mut sb: *mut crate::qcommon_h::msg_t, mut f: f32) {
    MSG_WriteShort(sb, (f * 65536f32 / 360f32) as i32 & 65535);
}
//============================================================
//
// reading functions
//
// returns -1 if no more characters are available
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadChar(mut msg: *mut crate::qcommon_h::msg_t) -> i32 {
    let mut c: i32 = 0; // use ReadByte so -1 is out of bounds
    c = MSG_ReadBits(msg, 8) as i8 as i32;
    if (*msg).readcount > (*msg).cursize {
        c = -(1)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadByte(mut msg: *mut crate::qcommon_h::msg_t) -> i32 {
    let mut c: i32 = 0;
    c = MSG_ReadBits(msg, 8) as u8 as i32;
    if (*msg).readcount > (*msg).cursize {
        c = -(1)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_LookaheadByte(mut msg: *mut crate::qcommon_h::msg_t) -> i32 {
    let bloc: i32 = crate::src::qcommon::huffman::Huff_getBloc();
    let readcount: i32 = (*msg).readcount;
    let bit: i32 = (*msg).bit;
    let mut c: i32 = MSG_ReadByte(msg);
    crate::src::qcommon::huffman::Huff_setBloc(bloc);
    (*msg).readcount = readcount;
    (*msg).bit = bit;
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadShort(mut msg: *mut crate::qcommon_h::msg_t) -> i32 {
    let mut c: i32 = 0;
    c = MSG_ReadBits(msg, 16) as i16 as i32;
    if (*msg).readcount > (*msg).cursize {
        c = -(1)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadLong(mut msg: *mut crate::qcommon_h::msg_t) -> i32 {
    let mut c: i32 = 0;
    c = MSG_ReadBits(msg, 32);
    if (*msg).readcount > (*msg).cursize {
        c = -(1)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadFloat(mut msg: *mut crate::qcommon_h::msg_t) -> f32 {
    let mut dat: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    dat.i = MSG_ReadBits(msg, 32);
    if (*msg).readcount > (*msg).cursize {
        dat.f = -1f32
    }
    return dat.f;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadString(mut msg: *mut crate::qcommon_h::msg_t) -> *mut i8 {
    static mut string: [i8; 1024] = [0; 1024];
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    l = 0;
    loop {
        c = MSG_ReadByte(msg);
        if c == -(1) || c == 0 {
            break;
        }
        // translate all fmt spec to avoid crash bugs
        if c == '%' as i32 {
            c = '.' as i32
        }
        // don't allow higher ascii values
        if c > 127 {
            c = '.' as i32
        }
        // break only after reading all expected data from bitstream
        if l as usize >= (::std::mem::size_of::<[i8; 1024]>()).wrapping_sub(1usize) {
            break; // use ReadByte so -1 is out of bounds
        }
        let fresh0 = l;
        l = l + 1;
        string[fresh0 as usize] = c as i8
    }
    string[l as usize] = '\u{0}' as i8;
    return string.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadBigString(mut msg: *mut crate::qcommon_h::msg_t) -> *mut i8 {
    static mut string: [i8; 8192] = [0; 8192];
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    l = 0;
    loop {
        c = MSG_ReadByte(msg);
        if c == -(1) || c == 0 {
            break;
        }
        // translate all fmt spec to avoid crash bugs
        if c == '%' as i32 {
            c = '.' as i32
        }
        // don't allow higher ascii values
        if c > 127 {
            c = '.' as i32
        }
        // break only after reading all expected data from bitstream
        if l as usize >= (::std::mem::size_of::<[i8; 8192]>()).wrapping_sub(1usize) {
            break; // use ReadByte so -1 is out of bounds
        }
        let fresh1 = l;
        l = l + 1;
        string[fresh1 as usize] = c as i8
    }
    string[l as usize] = '\u{0}' as i8;
    return string.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadStringLine(mut msg: *mut crate::qcommon_h::msg_t) -> *mut i8 {
    static mut string: [i8; 1024] = [0; 1024];
    let mut l: i32 = 0;
    let mut c: i32 = 0;
    l = 0;
    loop {
        c = MSG_ReadByte(msg);
        if c == -(1) || c == 0 || c == '\n' as i32 {
            break;
        }
        // translate all fmt spec to avoid crash bugs
        if c == '%' as i32 {
            c = '.' as i32
        }
        // don't allow higher ascii values
        if c > 127 {
            c = '.' as i32
        }
        // break only after reading all expected data from bitstream
        if l as usize >= (::std::mem::size_of::<[i8; 1024]>()).wrapping_sub(1usize) {
            break;
        }
        let fresh2 = l;
        l = l + 1;
        string[fresh2 as usize] = c as i8
    }
    string[l as usize] = '\u{0}' as i8;
    return string.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadAngle16(mut msg: *mut crate::qcommon_h::msg_t) -> f32 {
    return (MSG_ReadShort(msg) as f64 * (360.0 / 65536f64)) as f32;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadData(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut data: *mut libc::c_void,
    mut len: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < len {
        *(data as *mut crate::src::qcommon::q_shared::byte).offset(i as isize) =
            MSG_ReadByte(msg) as crate::src::qcommon::q_shared::byte;
        i += 1
    }
}
// a string hasher which gives the same hash value even if the
// string is later modified via the legacy MSG read/write code
#[no_mangle]

pub unsafe extern "C" fn MSG_HashKey(mut string: *const i8, mut maxlen: i32) -> i32 {
    let mut hash: i32 = 0;
    let mut i: i32 = 0;
    hash = 0;
    i = 0;
    while i < maxlen && *string.offset(i as isize) as i32 != '\u{0}' as i32 {
        if *string.offset(i as isize) as i32 & 0x80 != 0
            || *string.offset(i as isize) as i32 == '%' as i32
        {
            hash += '.' as i32 * (119 + i)
        } else {
            hash += *string.offset(i as isize) as i32 * (119 + i)
        }
        i += 1
    }
    hash = hash ^ hash >> 10 ^ hash >> 20;
    return hash;
}
/*
=============================================================================

delta functions with keys

=============================================================================
*/
#[no_mangle]

pub static mut kbitmask: [i32; 32] = [
    0x1,
    0x3,
    0x7,
    0xf,
    0x1f,
    0x3f,
    0x7f,
    0xff,
    0x1ff,
    0x3ff,
    0x7ff,
    0xfff,
    0x1fff,
    0x3fff,
    0x7fff,
    0xffff,
    0x1ffff,
    0x3ffff,
    0x7ffff,
    0xfffff,
    0x1fffff,
    0x3fffff,
    0x7fffff,
    0xffffff,
    0x1ffffff,
    0x3ffffff,
    0x7ffffff,
    0xfffffff,
    0x1fffffff,
    0x3fffffff,
    0x7fffffff,
    0xffffffffu32 as i32,
];
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: i32,
    mut oldV: i32,
    mut newV: i32,
    mut bits: i32,
) {
    if oldV == newV {
        MSG_WriteBits(msg, 0, 1);
        return;
    }
    MSG_WriteBits(msg, 1, 1);
    MSG_WriteBits(msg, newV ^ key, bits);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: i32,
    mut oldV: i32,
    mut bits: i32,
) -> i32 {
    if MSG_ReadBits(msg, 1) != 0 {
        return MSG_ReadBits(msg, bits) ^ key & kbitmask[(bits - 1i32) as usize];
    }
    return oldV;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaKeyFloat(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: i32,
    mut oldV: f32,
    mut newV: f32,
) {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    if oldV == newV {
        MSG_WriteBits(msg, 0, 1);
        return;
    }
    fi.f = newV;
    MSG_WriteBits(msg, 1, 1);
    MSG_WriteBits(msg, fi.i ^ key, 32);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaKeyFloat(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: i32,
    mut oldV: f32,
) -> f32 {
    if MSG_ReadBits(msg, 1) != 0 {
        let mut fi: crate::src::qcommon::q_shared::floatint_t =
            crate::src::qcommon::q_shared::floatint_t { f: 0. };
        fi.i = MSG_ReadBits(msg, 32) ^ key;
        return fi.f;
    }
    return oldV;
}
/*
============================================================================

usercmd_t communication

============================================================================
*/
/*
=====================
MSG_WriteDeltaUsercmdKey
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaUsercmdKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: i32,
    mut from: *mut crate::src::qcommon::q_shared::usercmd_t,
    mut to: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    if (*to).serverTime - (*from).serverTime < 256 {
        MSG_WriteBits(msg, 1, 1); // no change
        MSG_WriteBits(msg, (*to).serverTime - (*from).serverTime, 8i32);
    } else {
        MSG_WriteBits(msg, 0, 1);
        MSG_WriteBits(msg, (*to).serverTime, 32i32);
    }
    if (*from).angles[0] == (*to).angles[0]
        && (*from).angles[1] == (*to).angles[1]
        && (*from).angles[2] == (*to).angles[2]
        && (*from).forwardmove as i32 == (*to).forwardmove as i32
        && (*from).rightmove as i32 == (*to).rightmove as i32
        && (*from).upmove as i32 == (*to).upmove as i32
        && (*from).buttons == (*to).buttons
        && (*from).weapon as i32 == (*to).weapon as i32
    {
        MSG_WriteBits(msg, 0, 1);
        oldsize += 7;
        return;
    }
    key ^= (*to).serverTime;
    MSG_WriteBits(msg, 1, 1);
    MSG_WriteDeltaKey(msg, key, (*from).angles[0], (*to).angles[0], 16);
    MSG_WriteDeltaKey(msg, key, (*from).angles[1], (*to).angles[1], 16);
    MSG_WriteDeltaKey(msg, key, (*from).angles[2], (*to).angles[2], 16);
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).forwardmove as i32,
        (*to).forwardmove as i32,
        8,
    );
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).rightmove as i32,
        (*to).rightmove as i32,
        8,
    );
    MSG_WriteDeltaKey(msg, key, (*from).upmove as i32, (*to).upmove as i32, 8);
    MSG_WriteDeltaKey(msg, key, (*from).buttons, (*to).buttons, 16);
    MSG_WriteDeltaKey(msg, key, (*from).weapon as i32, (*to).weapon as i32, 8);
}
/*
=====================
MSG_ReadDeltaUsercmdKey
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaUsercmdKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: i32,
    mut from: *mut crate::src::qcommon::q_shared::usercmd_t,
    mut to: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    if MSG_ReadBits(msg, 1) != 0 {
        (*to).serverTime = (*from).serverTime + MSG_ReadBits(msg, 8)
    } else {
        (*to).serverTime = MSG_ReadBits(msg, 32)
    }
    if MSG_ReadBits(msg, 1) != 0 {
        key ^= (*to).serverTime;
        (*to).angles[0] = MSG_ReadDeltaKey(msg, key, (*from).angles[0], 16);
        (*to).angles[1] = MSG_ReadDeltaKey(msg, key, (*from).angles[1], 16);
        (*to).angles[2] = MSG_ReadDeltaKey(msg, key, (*from).angles[2], 16);
        (*to).forwardmove = MSG_ReadDeltaKey(msg, key, (*from).forwardmove as i32, 8) as i8;
        if (*to).forwardmove as i32 == -(128) {
            (*to).forwardmove = -127i8
        }
        (*to).rightmove = MSG_ReadDeltaKey(msg, key, (*from).rightmove as i32, 8) as i8;
        if (*to).rightmove as i32 == -(128) {
            (*to).rightmove = -127i8
        }
        (*to).upmove = MSG_ReadDeltaKey(msg, key, (*from).upmove as i32, 8) as i8;
        if (*to).upmove as i32 == -(128) {
            (*to).upmove = -127i8
        }
        (*to).buttons = MSG_ReadDeltaKey(msg, key, (*from).buttons, 16);
        (*to).weapon = MSG_ReadDeltaKey(msg, key, (*from).weapon as i32, 8)
            as crate::src::qcommon::q_shared::byte
    } else {
        (*to).angles[0] = (*from).angles[0];
        (*to).angles[1] = (*from).angles[1];
        (*to).angles[2] = (*from).angles[2];
        (*to).forwardmove = (*from).forwardmove;
        (*to).rightmove = (*from).rightmove;
        (*to).upmove = (*from).upmove;
        (*to).buttons = (*from).buttons;
        (*to).weapon = (*from).weapon
    };
}
/*
=============================================================================

entityState_t communication

=============================================================================
*/
/*
=================
MSG_ReportChangeVectors_f

Prints out a table from the current statistics for copying to code
=================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReportChangeVectors_f() {
    let mut i: i32 = 0;
    i = 0;
    while i < 256 {
        if pcount[i as usize] != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%d used %d\n\x00" as *const u8 as *const i8,
                i,
                pcount[i as usize],
            );
        }
        i += 1
    }
}
// using the stringizing operator to save typing...
// Initialized in run_static_initializers
#[no_mangle]

pub static mut entityStateFields: [netField_t; 51] = [netField_t {
    name: 0 as *mut i8,
    offset: 0,
    bits: 0,
}; 51];
/*
==================
MSG_WriteDeltaEntity

Writes part of a packetentities message, including the entity number.
Can delta from either a baseline or a previous packet_entity
If to is NULL, a remove entity update will be sent
If force is not set, then nothing at all will be generated if the entity is
identical, under the assumption that the in-order delta code will catch it.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaEntity(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::entityState_s,
    mut to: *mut crate::src::qcommon::q_shared::entityState_s,
    mut force: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: i32 = 0;
    let mut lc: i32 = 0;
    let mut numFields: i32 = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut trunc: i32 = 0;
    let mut fullFloat: f32 = 0.;
    let mut fromF: *mut i32 = 0 as *mut i32;
    let mut toF: *mut i32 = 0 as *mut i32;
    numFields = (::std::mem::size_of::<[netField_t; 51]>())
        .wrapping_div(::std::mem::size_of::<netField_t>()) as i32;
    // all fields should be 32 bits to avoid any compiler packing issues
    // the "number" field is not part of the field list
    // if this assert fails, someone added a field to the entityState_t
    // struct without updating the message fields
    // a NULL to is a delta remove message
    if to.is_null() {
        if from.is_null() {
            return;
        }
        MSG_WriteBits(msg, (*from).number, 10);
        MSG_WriteBits(msg, 1, 1);
        return;
    }
    if (*to).number < 0 || (*to).number >= (1) << 10 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"MSG_WriteDeltaEntity: Bad entity number: %i\x00" as *const u8 as *const i8,
            (*to).number,
        );
    }
    lc = 0;
    // build the change vector as bytes so it is endien independent
    i = 0;
    field = entityStateFields.as_mut_ptr();
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        if *fromF != *toF {
            lc = i + 1
        }
        i += 1;
        field = field.offset(1)
    }
    if lc == 0 {
        // nothing at all changed
        if force as u64 == 0 {
            return;
            // nothing at all
        }
        // write two bits for no change
        MSG_WriteBits(msg, (*to).number, 10); // not removed
        MSG_WriteBits(msg, 0, 1); // no delta
        MSG_WriteBits(msg, 0, 1); // not removed
        return;
    } // we have a delta
    MSG_WriteBits(msg, (*to).number, 10); // # of changes
    MSG_WriteBits(msg, 0, 1); // no change
    MSG_WriteBits(msg, 1, 1); // changed
    MSG_WriteByte(msg, lc);
    oldsize += numFields;
    i = 0;
    field = entityStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        if *fromF == *toF {
            MSG_WriteBits(msg, 0i32, 1i32);
        } else {
            MSG_WriteBits(msg, 1, 1);
            if (*field).bits == 0 {
                // float
                fullFloat = *(toF as *mut f32);
                trunc = fullFloat as i32;
                if fullFloat == 0.0 {
                    MSG_WriteBits(msg, 0, 1);
                    oldsize += 13
                } else {
                    MSG_WriteBits(msg, 1, 1);
                    if trunc as f32 == fullFloat
                        && trunc + ((1) << 13 - 1) >= 0
                        && (trunc + ((1) << 13 - 1)) < (1) << 13
                    {
                        // send as small integer
                        MSG_WriteBits(msg, 0, 1);
                        MSG_WriteBits(msg, trunc + ((1i32) << 13i32 - 1i32), 13i32);
                    } else {
                        // send as full floating point value
                        MSG_WriteBits(msg, 1, 1);
                        MSG_WriteBits(msg, *toF, 32i32);
                    }
                }
            } else if *toF == 0 {
                MSG_WriteBits(msg, 0i32, 1i32);
            } else {
                MSG_WriteBits(msg, 1, 1);
                // integer
                MSG_WriteBits(msg, *toF, (*field).bits);
            }
        }
        i += 1;
        field = field.offset(1)
    }
}
/*
==================
MSG_ReadDeltaEntity

The entity number has already been read from the message, which
is how the from state is identified.

If the delta removes the entity, entityState_t->number will be set to MAX_GENTITIES-1

Can go from either a baseline or a previous packet_entity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaEntity(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::entityState_t,
    mut to: *mut crate::src::qcommon::q_shared::entityState_t,
    mut number: i32,
) {
    let mut i: i32 = 0;
    let mut lc: i32 = 0;
    let mut numFields: i32 = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut fromF: *mut i32 = 0 as *mut i32;
    let mut toF: *mut i32 = 0 as *mut i32;
    let mut print: i32 = 0;
    let mut trunc: i32 = 0;
    let mut startBit: i32 = 0;
    let mut endBit: i32 = 0;
    if number < 0 || number >= (1) << 10 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Bad delta entity number: %i\x00" as *const u8 as *const i8,
            number,
        );
    }
    if (*msg).bit == 0 {
        startBit = (*msg).readcount * 8 - 10
    } else {
        startBit = ((*msg).readcount - 1) * 8 + (*msg).bit - 10
    }
    // check for a remove
    if MSG_ReadBits(msg, 1) == 1 {
        crate::stdlib::memset(
            to as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>(),
        );
        (*to).number = ((1) << 10) - 1;
        if !cl_shownet.is_null() && ((*cl_shownet).integer >= 2 || (*cl_shownet).integer == -(1)) {
            crate::src::qcommon::common::Com_Printf(
                b"%3i: #%-3i remove\n\x00" as *const u8 as *const i8,
                (*msg).readcount,
                number,
            );
        }
        return;
    }
    // check for no delta
    if MSG_ReadBits(msg, 1) == 0 {
        *to = *from;
        (*to).number = number;
        return;
    }
    numFields = (::std::mem::size_of::<[netField_t; 51]>())
        .wrapping_div(::std::mem::size_of::<netField_t>()) as i32;
    lc = MSG_ReadByte(msg);
    if lc > numFields || lc < 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"invalid entityState field count\x00" as *const u8 as *const i8,
        );
    }
    // shownet 2/3 will interleave with other printed info, -1 will
    // just print the delta records`
    if !cl_shownet.is_null() && ((*cl_shownet).integer >= 2 || (*cl_shownet).integer == -(1)) {
        print = 1;
        crate::src::qcommon::common::Com_Printf(
            b"%3i: #%-3i \x00" as *const u8 as *const i8,
            (*msg).readcount,
            (*to).number,
        );
    } else {
        print = 0
    }
    (*to).number = number;
    i = 0;
    field = entityStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        if MSG_ReadBits(msg, 1) == 0 {
            // no change
            *toF = *fromF
        } else if (*field).bits == 0 {
            // float
            if MSG_ReadBits(msg, 1) == 0 {
                *(toF as *mut f32) = 0.0
            } else if MSG_ReadBits(msg, 1) == 0 {
                // integral float
                trunc = MSG_ReadBits(msg, 13);
                // bias to allow equal parts positive and negative
                trunc -= (1) << 13 - 1;
                *(toF as *mut f32) = trunc as f32;
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%i \x00" as *const u8 as *const i8,
                        (*field).name,
                        trunc,
                    );
                }
            } else {
                // full floating point value
                *toF = MSG_ReadBits(msg, 32);
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%f \x00" as *const u8 as *const i8,
                        (*field).name,
                        *(toF as *mut f32) as f64,
                    );
                }
            }
        } else if MSG_ReadBits(msg, 1) == 0 {
            *toF = 0
        } else {
            // integer
            *toF = MSG_ReadBits(msg, (*field).bits);
            if print != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:%i \x00" as *const u8 as *const i8,
                    (*field).name,
                    *toF,
                );
            }
        }
        i += 1;
        field = field.offset(1)
    }
    i = lc;
    field = &mut *entityStateFields.as_mut_ptr().offset(lc as isize) as *mut netField_t;
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        //			pcount[i]++;
        // no change
        *toF = *fromF;
        i += 1;
        field = field.offset(1)
    }
    if print != 0 {
        if (*msg).bit == 0 {
            endBit = (*msg).readcount * 8 - 10
        } else {
            endBit = ((*msg).readcount - 1) * 8 + (*msg).bit - 10
        }
        crate::src::qcommon::common::Com_Printf(
            b" (%i bits)\n\x00" as *const u8 as *const i8,
            endBit - startBit,
        );
    };
}
/*
============================================================================

plyer_state_t communication

============================================================================
*/
// using the stringizing operator to save typing...
// Initialized in run_static_initializers
#[no_mangle]

pub static mut playerStateFields: [netField_t; 48] = [netField_t {
    name: 0 as *mut i8,
    offset: 0,
    bits: 0,
}; 48];
/*
=============
MSG_WriteDeltaPlayerstate

=============
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaPlayerstate(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::playerState_s,
    mut to: *mut crate::src::qcommon::q_shared::playerState_s,
) {
    let mut i: i32 = 0; // # of changes
    let mut dummy: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        }; // no change
    let mut statsbits: i32 = 0; // changed
    let mut persistantbits: i32 = 0;
    let mut ammobits: i32 = 0;
    let mut powerupbits: i32 = 0;
    let mut numFields: i32 = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut fromF: *mut i32 = 0 as *mut i32;
    let mut toF: *mut i32 = 0 as *mut i32;
    let mut fullFloat: f32 = 0.;
    let mut trunc: i32 = 0;
    let mut lc: i32 = 0;
    if from.is_null() {
        from = &mut dummy;
        crate::stdlib::memset(
            &mut dummy as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>(),
        );
    }
    numFields = (::std::mem::size_of::<[netField_t; 48]>())
        .wrapping_div(::std::mem::size_of::<netField_t>()) as i32;
    lc = 0;
    i = 0;
    field = playerStateFields.as_mut_ptr();
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        if *fromF != *toF {
            lc = i + 1
        }
        i += 1;
        field = field.offset(1)
    }
    MSG_WriteByte(msg, lc);
    oldsize += numFields - lc;
    i = 0;
    field = playerStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        if *fromF == *toF {
            MSG_WriteBits(msg, 0i32, 1i32);
        } else {
            MSG_WriteBits(msg, 1, 1);
            //		pcount[i]++;
            if (*field).bits == 0 {
                // float
                fullFloat = *(toF as *mut f32);
                trunc = fullFloat as i32;
                if trunc as f32 == fullFloat
                    && trunc + ((1) << 13 - 1) >= 0
                    && (trunc + ((1) << 13 - 1)) < (1) << 13
                {
                    // send as small integer
                    MSG_WriteBits(msg, 0, 1);
                    MSG_WriteBits(msg, trunc + ((1i32) << 13i32 - 1i32), 13i32);
                } else {
                    // send as full floating point value
                    MSG_WriteBits(msg, 1, 1);
                    MSG_WriteBits(msg, *toF, 32i32);
                }
            } else {
                // integer
                MSG_WriteBits(msg, *toF, (*field).bits);
            }
        }
        i += 1;
        field = field.offset(1)
    }
    //
    // send the arrays
    //
    statsbits = 0; // no change
    i = 0; // changed
    while i < 16 {
        if (*to).stats[i as usize] != (*from).stats[i as usize] {
            statsbits |= (1) << i
        } // changed
        i += 1
    }
    persistantbits = 0;
    i = 0;
    while i < 16 {
        if (*to).persistant[i as usize] != (*from).persistant[i as usize] {
            persistantbits |= (1) << i
        }
        i += 1
    }
    ammobits = 0;
    i = 0;
    while i < 16 {
        if (*to).ammo[i as usize] != (*from).ammo[i as usize] {
            ammobits |= (1) << i
        }
        i += 1
    }
    powerupbits = 0;
    i = 0;
    while i < 16 {
        if (*to).powerups[i as usize] != (*from).powerups[i as usize] {
            powerupbits |= (1) << i
        }
        i += 1
    }
    if statsbits == 0 && persistantbits == 0 && ammobits == 0 && powerupbits == 0 {
        MSG_WriteBits(msg, 0, 1);
        oldsize += 4;
        return;
    }
    MSG_WriteBits(msg, 1, 1);
    if statsbits != 0 {
        MSG_WriteBits(msg, 1, 1);
        MSG_WriteBits(msg, statsbits, 16);
        i = 0;
        while i < 16 {
            if statsbits & (1) << i != 0 {
                MSG_WriteShort(msg, (*to).stats[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0i32, 1i32);
        // no change
    } // changed
    if persistantbits != 0 {
        MSG_WriteBits(msg, 1, 1);
        MSG_WriteBits(msg, persistantbits, 16);
        i = 0;
        while i < 16 {
            if persistantbits & (1) << i != 0 {
                MSG_WriteShort(msg, (*to).persistant[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0i32, 1i32);
        // no change
    } // changed
    if ammobits != 0 {
        MSG_WriteBits(msg, 1, 1);
        MSG_WriteBits(msg, ammobits, 16);
        i = 0;
        while i < 16 {
            if ammobits & (1) << i != 0 {
                MSG_WriteShort(msg, (*to).ammo[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0i32, 1i32);
        // no change
    } // changed
    if powerupbits != 0 {
        MSG_WriteBits(msg, 1, 1);
        MSG_WriteBits(msg, powerupbits, 16);
        i = 0;
        while i < 16 {
            if powerupbits & (1) << i != 0 {
                MSG_WriteLong(msg, (*to).powerups[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0i32, 1i32);
        // no change
    };
}
/*
===================
MSG_ReadDeltaPlayerstate
===================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaPlayerstate(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::playerState_t,
    mut to: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut i: i32 = 0;
    let mut lc: i32 = 0;
    let mut bits: i32 = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut numFields: i32 = 0;
    let mut startBit: i32 = 0;
    let mut endBit: i32 = 0;
    let mut print: i32 = 0;
    let mut fromF: *mut i32 = 0 as *mut i32;
    let mut toF: *mut i32 = 0 as *mut i32;
    let mut trunc: i32 = 0;
    let mut dummy: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        };
    if from.is_null() {
        from = &mut dummy;
        crate::stdlib::memset(
            &mut dummy as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>(),
        );
    }
    *to = *from;
    if (*msg).bit == 0 {
        startBit = (*msg).readcount * 8 - 10
    } else {
        startBit = ((*msg).readcount - 1) * 8 + (*msg).bit - 10
    }
    // shownet 2/3 will interleave with other printed info, -2 will
    // just print the delta records
    if !cl_shownet.is_null() && ((*cl_shownet).integer >= 2 || (*cl_shownet).integer == -(2)) {
        print = 1;
        crate::src::qcommon::common::Com_Printf(
            b"%3i: playerstate \x00" as *const u8 as *const i8,
            (*msg).readcount,
        );
    } else {
        print = 0
    }
    numFields = (::std::mem::size_of::<[netField_t; 48]>())
        .wrapping_div(::std::mem::size_of::<netField_t>()) as i32;
    lc = MSG_ReadByte(msg);
    if lc > numFields || lc < 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"invalid playerState field count\x00" as *const u8 as *const i8,
        );
    }
    i = 0;
    field = playerStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        if MSG_ReadBits(msg, 1) == 0 {
            // no change
            *toF = *fromF
        } else if (*field).bits == 0 {
            // float
            if MSG_ReadBits(msg, 1) == 0 {
                // integral float
                trunc = MSG_ReadBits(msg, 13);
                // bias to allow equal parts positive and negative
                trunc -= (1) << 13 - 1;
                *(toF as *mut f32) = trunc as f32;
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%i \x00" as *const u8 as *const i8,
                        (*field).name,
                        trunc,
                    );
                }
            } else {
                // full floating point value
                *toF = MSG_ReadBits(msg, 32);
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%f \x00" as *const u8 as *const i8,
                        (*field).name,
                        *(toF as *mut f32) as f64,
                    );
                }
            }
        } else {
            // integer
            *toF = MSG_ReadBits(msg, (*field).bits);
            if print != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:%i \x00" as *const u8 as *const i8,
                    (*field).name,
                    *toF,
                );
            }
        }
        i += 1;
        field = field.offset(1)
    }
    i = lc;
    field = &mut *playerStateFields.as_mut_ptr().offset(lc as isize) as *mut netField_t;
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut i32;
        // no change
        *toF = *fromF;
        i += 1;
        field = field.offset(1)
    }
    // read the arrays
    if MSG_ReadBits(msg, 1) != 0 {
        // parse stats
        if MSG_ReadBits(msg, 1) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const i8,
                    b"PS_STATS\x00" as *const u8 as *const i8,
                );
            }
            bits = MSG_ReadBits(msg, 16);
            i = 0;
            while i < 16 {
                if bits & (1) << i != 0 {
                    (*to).stats[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        // parse persistant stats
        if MSG_ReadBits(msg, 1) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const i8,
                    b"PS_PERSISTANT\x00" as *const u8 as *const i8,
                );
            }
            bits = MSG_ReadBits(msg, 16);
            i = 0;
            while i < 16 {
                if bits & (1) << i != 0 {
                    (*to).persistant[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        // parse ammo
        if MSG_ReadBits(msg, 1) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const i8,
                    b"PS_AMMO\x00" as *const u8 as *const i8,
                );
            }
            bits = MSG_ReadBits(msg, 16);
            i = 0;
            while i < 16 {
                if bits & (1) << i != 0 {
                    (*to).ammo[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        // parse powerups
        if MSG_ReadBits(msg, 1) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const i8,
                    b"PS_POWERUPS\x00" as *const u8 as *const i8,
                ); // Do update
            }
            bits = MSG_ReadBits(msg, 16);
            i = 0;
            while i < 16 {
                if bits & (1) << i != 0 {
                    (*to).powerups[i as usize] = MSG_ReadLong(msg)
                }
                i += 1
            }
        }
    }
    if print != 0 {
        if (*msg).bit == 0 {
            endBit = (*msg).readcount * 8 - 10
        } else {
            endBit = ((*msg).readcount - 1) * 8 + (*msg).bit - 10
        }
        crate::src::qcommon::common::Com_Printf(
            b" (%i bits)\n\x00" as *const u8 as *const i8,
            endBit - startBit,
        );
    };
}
#[no_mangle]

pub static mut msg_hData: [i32; 256] = [
    250315, 41193, 6292, 7106, 3730, 3750, 6110, 23283, 33317, 6950, 7838, 9714, 9257, 17259, 3949,
    1778, 8288, 1604, 1590, 1663, 1100, 1213, 1238, 1134, 1749, 1059, 1246, 1149, 1273, 4486, 2805,
    3472, 21819, 1159, 1670, 1066, 1043, 1012, 1053, 1070, 1726, 888, 1180, 850, 960, 780, 1752,
    3296, 10630, 4514, 5881, 2685, 4650, 3837, 2093, 1867, 2584, 1949, 1972, 940, 1134, 1788, 1670,
    1206, 5719, 6128, 7222, 6654, 3710, 3795, 1492, 1524, 2215, 1140, 1355, 971, 2180, 1248, 1328,
    1195, 1770, 1078, 1264, 1266, 1168, 965, 1155, 1186, 1347, 1228, 1529, 1600, 2617, 2048, 2546,
    3275, 2410, 3585, 2504, 2800, 2675, 6146, 3663, 2840, 14253, 3164, 2221, 1687, 3208, 2739,
    3512, 4796, 4091, 3515, 5288, 4016, 7937, 6031, 5360, 3924, 4892, 3743, 4566, 4807, 5852, 6400,
    6225, 8291, 23243, 7838, 7073, 8935, 5437, 4483, 3641, 5256, 5312, 5328, 5370, 3492, 2458,
    1694, 1821, 2121, 1916, 1149, 1516, 1367, 1236, 1029, 1258, 1104, 1245, 1006, 1149, 1025, 1241,
    952, 1287, 997, 1713, 1009, 1187, 879, 1099, 929, 1078, 951, 1656, 930, 1153, 1030, 1262, 1062,
    1214, 1060, 1621, 930, 1106, 912, 1034, 892, 1158, 990, 1175, 850, 1121, 903, 1087, 920, 1144,
    1056, 3462, 2240, 4397, 12136, 7758, 1345, 1307, 3278, 1950, 886, 1023, 1112, 1077, 1042, 1061,
    1071, 1484, 1001, 1096, 915, 1052, 995, 1070, 876, 1111, 851, 1059, 805, 1112, 923, 1103, 817,
    1899, 1872, 976, 841, 1127, 956, 1159, 950, 7791, 954, 1289, 933, 1127, 3207, 1020, 927, 1355,
    768, 1040, 745, 952, 805, 1073, 740, 1013, 805, 1008, 796, 996, 1057, 11457, 13504,
];
#[no_mangle]

pub unsafe extern "C" fn MSG_initHuffman() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    msgInit = crate::src::qcommon::q_shared::qtrue;
    crate::src::qcommon::huffman::Huff_Init(&mut msgHuff);
    i = 0;
    while i < 256 {
        j = 0;
        while j < msg_hData[i as usize] {
            crate::src::qcommon::huffman::Huff_addRef(
                &mut msgHuff.compressor,
                i as crate::src::qcommon::q_shared::byte,
            );
            crate::src::qcommon::huffman::Huff_addRef(
                &mut msgHuff.decompressor,
                i as crate::src::qcommon::q_shared::byte,
            );
            j += 1
            // Do update
        }
        i += 1
    }
}
unsafe extern "C" fn run_static_initializers() {
    entityStateFields = [
        {
            let mut init = netField_t {
                name: b"pos.trTime\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trTime as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trBase[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trBase
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trBase[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trBase
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDelta[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDelta
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDelta[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDelta
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trBase[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trBase
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trBase[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trBase
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDelta[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDelta
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trBase[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trBase
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"event\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).event
                    as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles2[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles2
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eType\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).eType
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"torsoAnim\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).torsoAnim
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventParm\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).eventParm
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"legsAnim\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).legsAnim
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"groundEntityNum\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .groundEntityNum as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trType\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trType as *mut crate::src::qcommon::q_shared::trType_t
                    as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eFlags\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).eFlags
                    as *mut i32 as i32,
                bits: 19,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"otherEntityNum\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .otherEntityNum as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weapon\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).weapon
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"clientNum\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).clientNum
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDuration\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDuration as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trType\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trType as *mut crate::src::qcommon::q_shared::trType_t
                    as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"solid\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).solid
                    as *mut i32 as i32,
                bits: 24,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"powerups\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).powerups
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"modelindex\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).modelindex
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"otherEntityNum2\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .otherEntityNum2 as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"loopSound\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).loopSound
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"generic1\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).generic1
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin2[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin2
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin2[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin2
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin2[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin2
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"modelindex2\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).modelindex2
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"time\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).time
                    as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trTime\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trTime as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDuration\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDuration as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trBase[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trBase
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDelta[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDelta
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDelta[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDelta
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDelta[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDelta
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"time2\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).time2
                    as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles2[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles2
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles2[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles2
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"constantLight\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .constantLight as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"frame\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).frame
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
    ];
    playerStateFields = [
        {
            let mut init = netField_t {
                name: b"commandTime\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).commandTime
                    as *mut i32 as i32,
                bits: 32,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"bobCycle\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).bobCycle
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"velocity[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .velocity
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"velocity[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .velocity
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewangles[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .viewangles
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewangles[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .viewangles
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weaponTime\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).weaponTime
                    as *mut i32 as i32,
                bits: -(16),
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"velocity[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .velocity
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"legsTimer\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).legsTimer
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pm_time\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).pm_time
                    as *mut i32 as i32,
                bits: -(16),
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventSequence\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .eventSequence as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"torsoAnim\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).torsoAnim
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"movementDir\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).movementDir
                    as *mut i32 as i32,
                bits: 4,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"events[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .events
                    .as_mut_ptr()
                    .offset(0) as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"legsAnim\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).legsAnim
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"events[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .events
                    .as_mut_ptr()
                    .offset(1) as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pm_flags\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).pm_flags
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"groundEntityNum\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .groundEntityNum as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weaponstate\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).weaponstate
                    as *mut i32 as i32,
                bits: 4,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eFlags\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).eFlags
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"externalEvent\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .externalEvent as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"gravity\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).gravity
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"speed\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).speed
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"delta_angles[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .delta_angles
                    .as_mut_ptr()
                    .offset(1) as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"externalEventParm\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .externalEventParm as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewheight\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).viewheight
                    as *mut i32 as i32,
                bits: -(8),
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damageEvent\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damageEvent
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damageYaw\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damageYaw
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damagePitch\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damagePitch
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damageCount\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damageCount
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"generic1\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).generic1
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pm_type\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).pm_type
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"delta_angles[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .delta_angles
                    .as_mut_ptr()
                    .offset(0) as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"delta_angles[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .delta_angles
                    .as_mut_ptr()
                    .offset(2) as *mut i32 as i32,
                bits: 16,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"torsoTimer\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).torsoTimer
                    as *mut i32 as i32,
                bits: 12,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventParms[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .eventParms
                    .as_mut_ptr()
                    .offset(0) as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventParms[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .eventParms
                    .as_mut_ptr()
                    .offset(1) as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"clientNum\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).clientNum
                    as *mut i32 as i32,
                bits: 8,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weapon\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).weapon
                    as *mut i32 as i32,
                bits: 5,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewangles[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .viewangles
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"grapplePoint[0]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .grapplePoint
                    .as_mut_ptr()
                    .offset(0) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"grapplePoint[1]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .grapplePoint
                    .as_mut_ptr()
                    .offset(1) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"grapplePoint[2]\x00" as *const u8 as *mut i8,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .grapplePoint
                    .as_mut_ptr()
                    .offset(2) as *mut crate::src::qcommon::q_shared::vec_t
                    as i32,
                bits: 0,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"jumppad_ent\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).jumppad_ent
                    as *mut i32 as i32,
                bits: 10,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"loopSound\x00" as *const u8 as *mut i8,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).loopSound
                    as *mut i32 as i32,
                bits: 16,
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
//===========================================================================
/*
void MSG_NUinitHuffman() {
    byte	*data;
    int		size, i, ch;
    int		array[256];

    msgInit = qtrue;

    Huff_Init(&msgHuff);
    // load it in
    size = FS_ReadFile( "netchan/netchan.bin", (void **)&data );

    for(i=0;i<256;i++) {
        array[i] = 0;
    }
    for(i=0;i<size;i++) {
        ch = data[i];
        Huff_addRef(&msgHuff.compressor,	ch);			// Do update
        Huff_addRef(&msgHuff.decompressor,	ch);			// Do update
        array[ch]++;
    }
    Com_Printf("msg_hData {\n");
    for(i=0;i<256;i++) {
        if (array[i] == 0) {
            Huff_addRef(&msgHuff.compressor,	i);			// Do update
            Huff_addRef(&msgHuff.decompressor,	i);			// Do update
        }
        Com_Printf("%d,			// %d\n", array[i], i);
    }
    Com_Printf("};\n");
    FS_FreeFile( data );
    Cbuf_AddText( "condump dump.txt\n" );
}
*/
