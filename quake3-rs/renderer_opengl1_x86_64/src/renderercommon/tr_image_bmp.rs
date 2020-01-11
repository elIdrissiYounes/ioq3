use ::libc;

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
use crate::src::renderergl1::tr_main::ri;
use crate::stdlib::memcpy;
pub use crate::tr_public_h::refimport_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_82 {
    pub b: *mut crate::src::qcommon::q_shared::byte,
    pub v: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BMPHeader_t {
    pub id: [i8; 2],
    pub fileSize: u32,
    pub reserved0: u32,
    pub bitmapDataOffset: u32,
    pub bitmapHeaderSize: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bitsPerPixel: u16,
    pub compression: u32,
    pub bitmapDataSize: u32,
    pub hRes: u32,
    pub vRes: u32,
    pub colors: u32,
    pub importantColors: u32,
    pub palette: [[u8; 4]; 256],
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
// for color, lightmap, diffuse, and specular
// normals are swizzled, deluxe are not
// game path, including extension
// source image
// after power of two and picmip but not including clamp to MAX_TEXTURE_SIZE
// gl texture binding
// for texture usage in frame statistics
// only needed for voodoo2
// any change in the LIGHTMAP_* defines here MUST be reflected in
// R_FindShader() in tr_bsp.c
// shader is for 2D rendering
// pre-lit triangle models
// outside of TR since it shouldn't be cleared during ref re-init
// These variables should live inside glConfig but can't because of
// compatibility issues to the original ID vms.  If you release a stand-alone
// game and your mod uses tr_types.h from this build you can safely move them
// to the glconfig_t struct.
//
// cvars
//
// number of desired stencil bits
// number of desired depth bits
// number of desired color bits, only relevant for fullscreen
// number of desired texture bits
// 0 = use framebuffer depth
// 16 = use 16-bit textures
// 32 = use 32-bit textures
// all else = error
// video mode
// overrides hardware gamma capabilities
// global enable/disable of OpenGL extensions
// these control use of specific extensions
// font stuff
/*
=============================================================

IMAGE LOADERS

=============================================================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadBMP(
    mut name: *const i8,
    mut pic: *mut *mut crate::src::qcommon::q_shared::byte,
    mut width: *mut i32,
    mut height: *mut i32,
) {
    let mut columns: i32 = 0;
    let mut rows: i32 = 0;
    let mut numPixels: u32 = 0;
    let mut pixbuf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut row: i32 = 0;
    let mut column: i32 = 0;
    let mut buf_p: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut end: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut buffer: C2RustUnnamed_82 = C2RustUnnamed_82 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    let mut length: i32 = 0;
    let mut bmpHeader: BMPHeader_t = BMPHeader_t {
        id: [0; 2],
        fileSize: 0,
        reserved0: 0,
        bitmapDataOffset: 0,
        bitmapHeaderSize: 0,
        width: 0,
        height: 0,
        planes: 0,
        bitsPerPixel: 0,
        compression: 0,
        bitmapDataSize: 0,
        hRes: 0,
        vRes: 0,
        colors: 0,
        importantColors: 0,
        palette: [[0; 4]; 256],
    };
    let mut bmpRGBA: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    *pic = 0 as *mut crate::src::qcommon::q_shared::byte;
    if !width.is_null() {
        *width = 0
    }
    if !height.is_null() {
        *height = 0
    }
    //
    // load the file
    //
    length = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(name as *mut i8, &mut buffer.v) as i32;
    if buffer.b.is_null() || length < 0 {
        return;
    }
    if length < 54 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: header too short (%s)\x00" as *const u8 as *const i8,
            name,
        );
    }
    buf_p = buffer.b;
    end = buffer.b.offset(length as isize);
    let fresh0 = buf_p;
    buf_p = buf_p.offset(1);
    bmpHeader.id[0] = *fresh0 as i8;
    let fresh1 = buf_p;
    buf_p = buf_p.offset(1);
    bmpHeader.id[1] = *fresh1 as i8;
    bmpHeader.fileSize = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.reserved0 = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.bitmapDataOffset = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.bitmapHeaderSize = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.width = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.height = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.planes = *(buf_p as *mut i16) as u16;
    buf_p = buf_p.offset(2);
    bmpHeader.bitsPerPixel = *(buf_p as *mut i16) as u16;
    buf_p = buf_p.offset(2);
    bmpHeader.compression = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.bitmapDataSize = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.hRes = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.vRes = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.colors = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    bmpHeader.importantColors = *(buf_p as *mut i32) as u32;
    buf_p = buf_p.offset(4);
    if bmpHeader.bitsPerPixel as i32 == 8 {
        if buf_p.offset(::std::mem::size_of::<[[u8; 4]; 256]>() as isize) > end {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"LoadBMP: header too short (%s)\x00" as *const u8 as *const i8,
                name,
            );
        }
        crate::stdlib::memcpy(
            bmpHeader.palette.as_mut_ptr() as *mut libc::c_void,
            buf_p as *const libc::c_void,
            ::std::mem::size_of::<[[u8; 4]; 256]>(),
        );
    }
    if buffer.b.offset(bmpHeader.bitmapDataOffset as isize) > end {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: invalid offset value in header (%s)\x00" as *const u8 as *const i8,
            name,
        );
    }
    buf_p = buffer.b.offset(bmpHeader.bitmapDataOffset as isize);
    if bmpHeader.id[0] as i32 != 'B' as i32 && bmpHeader.id[1] as i32 != 'M' as i32 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: only Windows-style BMP files supported (%s)\x00" as *const u8 as *const i8,
            name,
        );
    }
    if bmpHeader.fileSize != length as u32 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: header size does not match file size (%u vs. %u) (%s)\x00" as *const u8
                as *const i8,
            bmpHeader.fileSize,
            length,
            name,
        );
    }
    if bmpHeader.compression != 0 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: only uncompressed BMP files supported (%s)\x00" as *const u8 as *const i8,
            name,
        );
    }
    if (bmpHeader.bitsPerPixel as i32) < 8 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: monochrome and 4-bit BMP files not supported (%s)\x00" as *const u8
                as *const i8,
            name,
        );
    }
    match bmpHeader.bitsPerPixel as i32 {
        8 | 16 | 24 | 32 => {}
        _ => {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"LoadBMP: illegal pixel_size \'%hu\' in file \'%s\'\x00" as *const u8 as *const i8,
                bmpHeader.bitsPerPixel as i32,
                name,
            );
        }
    }
    columns = bmpHeader.width as i32;
    rows = bmpHeader.height as i32;
    if rows < 0 {
        rows = -rows
    }
    numPixels = (columns * rows) as u32;
    if columns <= 0
        || rows == 0
        || numPixels > 0x1fffffff
        || numPixels
            .wrapping_mul(4u32)
            .wrapping_div(columns as u32)
            .wrapping_div(4u32)
            != rows as u32
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: %s has an invalid image size\x00" as *const u8 as *const i8,
            name,
        );
    }
    if buf_p.offset(
        numPixels
            .wrapping_mul(bmpHeader.bitsPerPixel as u32)
            .wrapping_div(8u32) as isize,
    ) > end
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadBMP: file truncated (%s)\x00" as *const u8 as *const i8,
            name,
        );
    }
    if !width.is_null() {
        *width = columns
    }
    if !height.is_null() {
        *height = rows
    }
    bmpRGBA = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(numPixels.wrapping_mul(4u32) as i32)
        as *mut crate::src::qcommon::q_shared::byte;
    *pic = bmpRGBA;
    row = rows - 1;
    while row >= 0 {
        pixbuf = bmpRGBA.offset((row * columns * 4) as isize);

        for column in 0..columns {
            let mut red: u8 = 0;

            let mut green: u8 = 0;

            let mut blue: u8 = 0;

            let mut alpha: u8 = 0;

            let mut palIndex: i32 = 0;

            let mut shortPixel: u16 = 0;

            match bmpHeader.bitsPerPixel as i32 {
                8 => {
                    let fresh2 = buf_p;
                    buf_p = buf_p.offset(1);
                    palIndex = *fresh2 as i32;
                    let fresh3 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh3 = bmpHeader.palette[palIndex as usize][2];
                    let fresh4 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh4 = bmpHeader.palette[palIndex as usize][1];
                    let fresh5 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh5 = bmpHeader.palette[palIndex as usize][0];
                    let fresh6 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh6 = 0xffu8
                }
                16 => {
                    shortPixel = *(pixbuf as *mut u16);
                    pixbuf = pixbuf.offset(2);
                    let fresh7 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh7 = ((shortPixel as i32 & (31) << 10) >> 7)
                        as crate::src::qcommon::q_shared::byte;
                    let fresh8 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh8 = ((shortPixel as i32 & (31) << 5) >> 2)
                        as crate::src::qcommon::q_shared::byte;
                    let fresh9 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh9 =
                        ((shortPixel as i32 & 31) << 3) as crate::src::qcommon::q_shared::byte;
                    let fresh10 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh10 = 0xffu8
                }
                24 => {
                    let fresh11 = buf_p;
                    buf_p = buf_p.offset(1);
                    blue = *fresh11;
                    let fresh12 = buf_p;
                    buf_p = buf_p.offset(1);
                    green = *fresh12;
                    let fresh13 = buf_p;
                    buf_p = buf_p.offset(1);
                    red = *fresh13;
                    let fresh14 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh14 = red;
                    let fresh15 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh15 = green;
                    let fresh16 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh16 = blue;
                    let fresh17 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh17 = 255u8
                }
                32 => {
                    let fresh18 = buf_p;
                    buf_p = buf_p.offset(1);
                    blue = *fresh18;
                    let fresh19 = buf_p;
                    buf_p = buf_p.offset(1);
                    green = *fresh19;
                    let fresh20 = buf_p;
                    buf_p = buf_p.offset(1);
                    red = *fresh20;
                    let fresh21 = buf_p;
                    buf_p = buf_p.offset(1);
                    alpha = *fresh21;
                    let fresh22 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh22 = red;
                    let fresh23 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh23 = green;
                    let fresh24 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh24 = blue;
                    let fresh25 = pixbuf;
                    pixbuf = pixbuf.offset(1);
                    *fresh25 = alpha
                }
                _ => {}
            }
        }
        row -= 1
    }
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(buffer.v);
}
