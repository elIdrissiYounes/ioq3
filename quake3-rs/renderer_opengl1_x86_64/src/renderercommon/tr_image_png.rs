use ::libc;

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
use crate::src::qcommon::puff::puff;
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
use crate::src::qcommon::q_shared::LongSwap;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
use crate::src::renderergl1::tr_main::ri;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::tr_public_h::refimport_t;
use ::libc::abs;
/*
 *  Some support functions for buffered files follow.
 */
/*
 *  buffered file representation
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferedFile {
    pub Buffer: *mut crate::src::qcommon::q_shared::byte,
    pub Length: libc::c_int,
    pub Ptr: *mut crate::src::qcommon::q_shared::byte,
    pub BytesLeft: libc::c_int,
}
/*
 *  Per specification the first chunk after the signature SHALL be IHDR.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PNG_Chunk_IHDR {
    pub Width: crate::stdlib::uint32_t,
    pub Height: crate::stdlib::uint32_t,
    pub BitDepth: crate::stdlib::uint8_t,
    pub ColourType: crate::stdlib::uint8_t,
    pub CompressionMethod: crate::stdlib::uint8_t,
    pub FilterMethod: crate::stdlib::uint8_t,
    pub InterlaceMethod: crate::stdlib::uint8_t,
}
/*
 *  After the signature diverse chunks follow.
 *  A chunk consists of a header and if Length
 *  is bigger than 0 a body and a CRC of the body follow.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PNG_ChunkHeader {
    pub Length: crate::stdlib::uint32_t,
    pub Type: crate::stdlib::uint32_t,
}

pub type PNG_ChunkCRC = crate::stdlib::uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_90 {
    pub b: *mut crate::src::qcommon::q_shared::byte,
    pub v: *mut libc::c_void,
}
/*
 *  Read a file into a buffer.
 */

unsafe extern "C" fn ReadBufferedFile(mut name: *const libc::c_char) -> *mut BufferedFile {
    let mut BF: *mut BufferedFile = 0 as *mut BufferedFile;
    let mut buffer: C2RustUnnamed_90 = C2RustUnnamed_90 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    /*
     *  input verification
     */
    if name.is_null() {
        return 0 as *mut BufferedFile;
    }
    /*
     *  Allocate control struct.
     */
    BF = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<BufferedFile>() as libc::c_ulong as libc::c_int
    ) as *mut BufferedFile;
    if BF.is_null() {
        return 0 as *mut BufferedFile;
    }
    /*
     *  Initialize the structs components.
     */
    (*BF).Length = 0 as libc::c_int;
    (*BF).Buffer = 0 as *mut crate::src::qcommon::q_shared::byte;
    (*BF).Ptr = 0 as *mut crate::src::qcommon::q_shared::byte;
    (*BF).BytesLeft = 0 as libc::c_int;
    /*
     *  Read the file.
     */
    (*BF).Length = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(
        name as *mut libc::c_char, &mut buffer.v
    ) as libc::c_int;
    (*BF).Buffer = buffer.b;
    /*
     *  Did we get it? Is it big enough?
     */
    if !(!(*BF).Buffer.is_null() && (*BF).Length > 0 as libc::c_int) {
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(BF as *mut libc::c_void);
        return 0 as *mut BufferedFile;
    }
    /*
     *  Set the pointers and counters.
     */
    (*BF).Ptr = (*BF).Buffer;
    (*BF).BytesLeft = (*BF).Length;
    return BF;
}
/*
 *  Close a buffered file.
 */

unsafe extern "C" fn CloseBufferedFile(mut BF: *mut BufferedFile) {
    if !BF.is_null() {
        if !(*BF).Buffer.is_null() {
            crate::src::renderergl1::tr_main::ri
                .FS_FreeFile
                .expect("non-null function pointer")((*BF).Buffer as *mut libc::c_void);
        }
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(BF as *mut libc::c_void);
    };
}
/*
 *  Get a pointer to the requested bytes.
 */

unsafe extern "C" fn BufferedFileRead(
    mut BF: *mut BufferedFile,
    mut Length: libc::c_uint,
) -> *mut libc::c_void {
    let mut RetVal: *mut libc::c_void = 0 as *mut libc::c_void;
    /*
     *  input verification
     */
    if !(!BF.is_null() && Length != 0) {
        return 0 as *mut libc::c_void;
    }
    /*
     *  not enough bytes left
     */
    if Length > (*BF).BytesLeft as libc::c_uint {
        return 0 as *mut libc::c_void;
    }
    /*
     *  the pointer to the requested data
     */
    RetVal = (*BF).Ptr as *mut libc::c_void;
    /*
     *  Raise the pointer and counter.
     */
    (*BF).Ptr = (*BF).Ptr.offset(Length as isize);
    (*BF).BytesLeft =
        ((*BF).BytesLeft as libc::c_uint).wrapping_sub(Length) as libc::c_int as libc::c_int;
    return RetVal;
}
/*
 *  Rewind the buffer.
 */

unsafe extern "C" fn BufferedFileRewind(
    mut BF: *mut BufferedFile,
    mut Offset: libc::c_uint,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut BytesRead: libc::c_uint = 0;
    /*
     *  input verification
     */
    if BF.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  special trick to rewind to the beginning of the buffer
     */
    if Offset == -(1 as libc::c_int) as libc::c_uint {
        (*BF).Ptr = (*BF).Buffer;
        (*BF).BytesLeft = (*BF).Length;
        return crate::src::qcommon::q_shared::qtrue;
    }
    /*
     *  How many bytes do we have already read?
     */
    BytesRead = (*BF).Ptr.wrapping_offset_from((*BF).Buffer) as libc::c_long as libc::c_uint;
    /*
     *  We can only rewind to the beginning of the BufferedFile.
     */
    if Offset > BytesRead {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  lower the pointer and counter.
     */
    (*BF).Ptr = (*BF).Ptr.offset(-(Offset as isize));
    (*BF).BytesLeft =
        ((*BF).BytesLeft as libc::c_uint).wrapping_add(Offset) as libc::c_int as libc::c_int;
    return crate::src::qcommon::q_shared::qtrue;
}
/*
 *  Skip some bytes.
 */

unsafe extern "C" fn BufferedFileSkip(
    mut BF: *mut BufferedFile,
    mut Offset: libc::c_uint,
) -> crate::src::qcommon::q_shared::qboolean {
    /*
     *  input verification
     */
    if BF.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  We can only skip to the end of the BufferedFile.
     */
    if Offset > (*BF).BytesLeft as libc::c_uint {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  lower the pointer and counter.
     */
    (*BF).Ptr = (*BF).Ptr.offset(Offset as isize);
    (*BF).BytesLeft =
        ((*BF).BytesLeft as libc::c_uint).wrapping_sub(Offset) as libc::c_int as libc::c_int;
    return crate::src::qcommon::q_shared::qtrue;
}
/*
 *  Find a chunk
 */

unsafe extern "C" fn FindChunk(
    mut BF: *mut BufferedFile,
    mut ChunkType: crate::stdlib::uint32_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut CH: *mut PNG_ChunkHeader = 0 as *mut PNG_ChunkHeader;
    let mut Length: crate::stdlib::uint32_t = 0;
    let mut Type: crate::stdlib::uint32_t = 0;
    /*
     *  input verification
     */
    if BF.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  cycle trough the chunks
     */
    while crate::src::qcommon::q_shared::qtrue as libc::c_int != 0 {
        /*
         *  Read the chunk-header.
         */
        CH = BufferedFileRead(BF, 8 as libc::c_int as libc::c_uint) as *mut PNG_ChunkHeader;
        if CH.is_null() {
            return crate::src::qcommon::q_shared::qfalse;
        }
        /*
         *  Do not swap the original types
         *  they might be needed later.
         */
        Length = crate::src::qcommon::q_shared::LongSwap((*CH).Length as libc::c_int)
            as crate::stdlib::uint32_t;
        Type = crate::src::qcommon::q_shared::LongSwap((*CH).Type as libc::c_int)
            as crate::stdlib::uint32_t;
        /*
         *  We found it!
         */
        if Type == ChunkType {
            /*
             *  Rewind to the start of the chunk.
             */
            BufferedFileRewind(BF, 8 as libc::c_int as libc::c_uint);
            break;
        } else if Length != 0 {
            if BufferedFileSkip(BF, Length.wrapping_add(4 as libc::c_int as libc::c_uint)) as u64
                == 0
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
        }
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
 *  Skip the rest of the chunk.
 */
/*
 *  Decompress all IDATs
 */

unsafe extern "C" fn DecompressIDATs(
    mut BF: *mut BufferedFile,
    mut Buffer: *mut *mut crate::stdlib::uint8_t,
) -> crate::stdlib::uint32_t {
    let mut DecompressedData: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut DecompressedDataLength: crate::stdlib::uint32_t = 0;
    let mut CompressedData: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut CompressedDataPtr: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut CompressedDataLength: crate::stdlib::uint32_t = 0;
    let mut CH: *mut PNG_ChunkHeader = 0 as *mut PNG_ChunkHeader;
    let mut Length: crate::stdlib::uint32_t = 0;
    let mut Type: crate::stdlib::uint32_t = 0;
    let mut BytesToRewind: libc::c_int = 0;
    let mut puffResult: crate::stdlib::int32_t = 0;
    let mut puffDest: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut puffDestLen: crate::stdlib::uint32_t = 0;
    let mut puffSrc: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut puffSrcLen: crate::stdlib::uint32_t = 0;
    /*
     *  input verification
     */
    if !(!BF.is_null() && !Buffer.is_null()) {
        return -(1 as libc::c_int) as crate::stdlib::uint32_t;
    }
    /*
     *  some zeroing
     */
    DecompressedData = 0 as *mut crate::stdlib::uint8_t;
    *Buffer = DecompressedData;
    CompressedData = 0 as *mut crate::stdlib::uint8_t;
    CompressedDataLength = 0 as libc::c_int as crate::stdlib::uint32_t;
    BytesToRewind = 0 as libc::c_int;
    /*
     *  Find the first IDAT chunk.
     */
    if FindChunk(
        BF,
        (('I' as i32) << 24 as libc::c_int
            | ('D' as i32) << 16 as libc::c_int
            | ('A' as i32) << 8 as libc::c_int
            | 'T' as i32) as crate::stdlib::uint32_t,
    ) as u64
        == 0
    {
        return -(1 as libc::c_int) as crate::stdlib::uint32_t;
    }
    /*
     *  Count the size of the uncompressed data
     */
    while crate::src::qcommon::q_shared::qtrue as libc::c_int != 0 {
        /*
         *  Read chunk header
         */
        CH = BufferedFileRead(BF, 8 as libc::c_int as libc::c_uint) as *mut PNG_ChunkHeader;
        if CH.is_null() {
            /*
             *  Rewind to the start of this adventure
             *  and return unsuccessful
             */
            BufferedFileRewind(BF, BytesToRewind as libc::c_uint);
            return -(1 as libc::c_int) as crate::stdlib::uint32_t;
        }
        /*
         *  Length and Type of chunk
         */
        Length = crate::src::qcommon::q_shared::LongSwap((*CH).Length as libc::c_int)
            as crate::stdlib::uint32_t;
        Type = crate::src::qcommon::q_shared::LongSwap((*CH).Type as libc::c_int)
            as crate::stdlib::uint32_t;
        /*
         *  We have reached the end of the IDAT chunks
         */
        if !(Type
            == (('I' as i32) << 24 as libc::c_int
                | ('D' as i32) << 16 as libc::c_int
                | ('A' as i32) << 8 as libc::c_int
                | 'T' as i32) as libc::c_uint)
        {
            BufferedFileRewind(BF, 8 as libc::c_int as libc::c_uint);
            break;
        } else {
            /*
             *  Add chunk header to count.
             */
            BytesToRewind += 8 as libc::c_int;
            /*
             *  Skip to next chunk
             */
            if Length != 0 {
                if BufferedFileSkip(BF, Length.wrapping_add(4 as libc::c_int as libc::c_uint))
                    as u64
                    == 0
                {
                    BufferedFileRewind(BF, BytesToRewind as libc::c_uint);
                    return -(1 as libc::c_int) as crate::stdlib::uint32_t;
                }
                BytesToRewind = (BytesToRewind as libc::c_uint)
                    .wrapping_add(Length.wrapping_add(4 as libc::c_int as libc::c_uint))
                    as libc::c_int as libc::c_int;
                CompressedDataLength = (CompressedDataLength as libc::c_uint).wrapping_add(Length)
                    as crate::stdlib::uint32_t
                    as crate::stdlib::uint32_t
            }
        }
    }
    BufferedFileRewind(BF, BytesToRewind as libc::c_uint);
    CompressedData = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(
        CompressedDataLength as libc::c_int
    ) as *mut crate::stdlib::uint8_t;
    if CompressedData.is_null() {
        return -(1 as libc::c_int) as crate::stdlib::uint32_t;
    }
    CompressedDataPtr = CompressedData;
    /*
     *  Collect the compressed Data
     */
    while crate::src::qcommon::q_shared::qtrue as libc::c_int != 0 {
        /*
         *  Read chunk header
         */
        CH = BufferedFileRead(BF, 8 as libc::c_int as libc::c_uint) as *mut PNG_ChunkHeader;
        if CH.is_null() {
            crate::src::renderergl1::tr_main::ri
                .Free
                .expect("non-null function pointer")(
                CompressedData as *mut libc::c_void
            );
            return -(1 as libc::c_int) as crate::stdlib::uint32_t;
        }
        /*
         *  Length and Type of chunk
         */
        Length = crate::src::qcommon::q_shared::LongSwap((*CH).Length as libc::c_int)
            as crate::stdlib::uint32_t;
        Type = crate::src::qcommon::q_shared::LongSwap((*CH).Type as libc::c_int)
            as crate::stdlib::uint32_t;
        /*
         *  We have reached the end of the IDAT chunks
         */
        if !(Type
            == (('I' as i32) << 24 as libc::c_int
                | ('D' as i32) << 16 as libc::c_int
                | ('A' as i32) << 8 as libc::c_int
                | 'T' as i32) as libc::c_uint)
        {
            BufferedFileRewind(BF, 8 as libc::c_int as libc::c_uint);
            break;
        } else if Length != 0 {
            let mut OrigCompressedData: *mut crate::stdlib::uint8_t =
                0 as *mut crate::stdlib::uint8_t;
            OrigCompressedData = BufferedFileRead(BF, Length) as *mut crate::stdlib::uint8_t;
            if OrigCompressedData.is_null() {
                crate::src::renderergl1::tr_main::ri
                    .Free
                    .expect("non-null function pointer")(
                    CompressedData as *mut libc::c_void
                );
                return -(1 as libc::c_int) as crate::stdlib::uint32_t;
            }
            if BufferedFileSkip(BF, 4 as libc::c_int as libc::c_uint) as u64 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Free
                    .expect("non-null function pointer")(
                    CompressedData as *mut libc::c_void
                );
                return -(1 as libc::c_int) as crate::stdlib::uint32_t;
            }
            crate::stdlib::memcpy(
                CompressedDataPtr as *mut libc::c_void,
                OrigCompressedData as *const libc::c_void,
                Length as libc::c_ulong,
            );
            CompressedDataPtr = CompressedDataPtr.offset(Length as isize)
        }
    }
    /*
     *  Copy the Data
     */
    /*
     *  Let puff() calculate the decompressed data length.
     */
    puffDest = 0 as *mut crate::stdlib::uint8_t;
    puffDestLen = 0 as libc::c_int as crate::stdlib::uint32_t;
    /*
     *  The zlib header and checkvalue don't belong to the compressed data.
     */
    puffSrc = CompressedData.offset(2 as libc::c_int as isize);
    puffSrcLen = CompressedDataLength
        .wrapping_sub(2 as libc::c_int as libc::c_uint)
        .wrapping_sub(4 as libc::c_int as libc::c_uint);
    /*
     *  first puff() to calculate the size of the uncompressed data
     */
    puffResult =
        crate::src::qcommon::puff::puff(puffDest, &mut puffDestLen, puffSrc, &mut puffSrcLen);
    if !(puffResult == 0 as libc::c_int && puffDestLen > 0 as libc::c_int as libc::c_uint) {
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(CompressedData as *mut libc::c_void);
        return -(1 as libc::c_int) as crate::stdlib::uint32_t;
    }
    /*
     *  Allocate the buffer for the uncompressed data.
     */
    DecompressedData = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(puffDestLen as libc::c_int)
        as *mut crate::stdlib::uint8_t;
    if DecompressedData.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(CompressedData as *mut libc::c_void);
        return -(1 as libc::c_int) as crate::stdlib::uint32_t;
    }
    /*
     *  Set the input again in case something was changed by the last puff() .
     */
    puffDest = DecompressedData;
    puffSrc = CompressedData.offset(2 as libc::c_int as isize);
    puffSrcLen = CompressedDataLength
        .wrapping_sub(2 as libc::c_int as libc::c_uint)
        .wrapping_sub(4 as libc::c_int as libc::c_uint);
    /*
     *  decompression puff()
     */
    puffResult =
        crate::src::qcommon::puff::puff(puffDest, &mut puffDestLen, puffSrc, &mut puffSrcLen);
    /*
     *  The compressed data is not needed anymore.
     */
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")(CompressedData as *mut libc::c_void);
    /*
     *  Check if the last puff() was successful.
     */
    if !(puffResult == 0 as libc::c_int && puffDestLen > 0 as libc::c_int as libc::c_uint) {
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(DecompressedData as *mut libc::c_void);
        return -(1 as libc::c_int) as crate::stdlib::uint32_t;
    }
    /*
     *  Set the output of this function.
     */
    DecompressedDataLength = puffDestLen;
    *Buffer = DecompressedData;
    return DecompressedDataLength;
}
/*
 *  the Paeth predictor
 */

unsafe extern "C" fn PredictPaeth(
    mut a: crate::stdlib::uint8_t,
    mut b: crate::stdlib::uint8_t,
    mut c: crate::stdlib::uint8_t,
) -> crate::stdlib::uint8_t {
    /*
     *  a == Left
     *  b == Up
     *  c == UpLeft
     */
    let mut Pr: crate::stdlib::uint8_t = 0;
    let mut p: libc::c_int = 0;
    let mut pa: libc::c_int = 0;
    let mut pb: libc::c_int = 0;
    let mut pc: libc::c_int = 0;
    p = a as libc::c_int + b as libc::c_int - c as libc::c_int;
    pa = ::libc::abs(p - a as libc::c_int);
    pb = ::libc::abs(p - b as libc::c_int);
    pc = ::libc::abs(p - c as libc::c_int);
    if pa <= pb && pa <= pc {
        Pr = a
    } else if pb <= pc {
        Pr = b
    } else {
        Pr = c
    }
    return Pr;
}
/*
 *  Reverse the filters.
 */

unsafe extern "C" fn UnfilterImage(
    mut DecompressedData: *mut crate::stdlib::uint8_t,
    mut ImageHeight: crate::stdlib::uint32_t,
    mut BytesPerScanline: crate::stdlib::uint32_t,
    mut BytesPerPixel: crate::stdlib::uint32_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut DecompPtr: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut FilterType: crate::stdlib::uint8_t = 0;
    let mut PixelLeft: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut PixelUp: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut PixelUpLeft: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut w: crate::stdlib::uint32_t = 0;
    let mut h: crate::stdlib::uint32_t = 0;
    let mut p: crate::stdlib::uint32_t = 0;
    /*
     *  some zeros for the filters
     */
    let mut Zeros: [crate::stdlib::uint8_t; 8] = [
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
        0 as libc::c_int as crate::stdlib::uint8_t,
    ];
    /*
     *  input verification
     */
    if !(!DecompressedData.is_null() && BytesPerPixel != 0) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  ImageHeight and BytesPerScanline can be zero in small interlaced images.
     */
    if ImageHeight == 0 || BytesPerScanline == 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    /*
     *  Set the pointer to the start of the decompressed Data.
     */
    DecompPtr = DecompressedData;
    /*
     *  Un-filtering is done in place.
     */
    /*
     *  Go trough all scanlines.
     */
    h = 0 as libc::c_int as crate::stdlib::uint32_t;
    while h < ImageHeight {
        /*
         *  Every scanline starts with a FilterType byte.
         */
        FilterType = *DecompPtr;
        DecompPtr = DecompPtr.offset(1);
        /*
         *  Left pixel of the first byte in a scanline is zero.
         */
        PixelLeft = Zeros.as_mut_ptr();
        /*
         *  Set PixelUp to previous line only if we are on the second line or above.
         *
         *  Plus one byte for the FilterType
         */
        if h > 0 as libc::c_int as libc::c_uint {
            PixelUp = DecompPtr
                .offset(-(BytesPerScanline.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
        } else {
            PixelUp = Zeros.as_mut_ptr()
        }
        /*
         * The pixel left to the first pixel of the previous scanline is zero too.
         */
        PixelUpLeft = Zeros.as_mut_ptr();
        /*
         *  Cycle trough all pixels of the scanline.
         */
        w = 0 as libc::c_int as crate::stdlib::uint32_t;
        while w < BytesPerScanline.wrapping_div(BytesPerPixel) {
            /*
             *  Cycle trough the bytes of the pixel.
             */
            p = 0 as libc::c_int as crate::stdlib::uint32_t;
            while p < BytesPerPixel {
                match FilterType as libc::c_int {
                    0 => {}
                    1 => {
                        let ref mut fresh0 = *DecompPtr.offset(p as isize);
                        *fresh0 = (*fresh0 as libc::c_int
                            + *PixelLeft.offset(p as isize) as libc::c_int)
                            as crate::stdlib::uint8_t
                    }
                    2 => {
                        let ref mut fresh1 = *DecompPtr.offset(p as isize);
                        *fresh1 = (*fresh1 as libc::c_int
                            + *PixelUp.offset(p as isize) as libc::c_int)
                            as crate::stdlib::uint8_t
                    }
                    3 => {
                        let ref mut fresh2 = *DecompPtr.offset(p as isize);
                        *fresh2 = (*fresh2 as libc::c_int
                            + ((*PixelLeft.offset(p as isize) as crate::stdlib::uint16_t
                                as libc::c_int
                                + *PixelUp.offset(p as isize) as crate::stdlib::uint16_t
                                    as libc::c_int)
                                / 2 as libc::c_int)
                                as crate::stdlib::uint8_t
                                as libc::c_int)
                            as crate::stdlib::uint8_t
                    }
                    4 => {
                        let ref mut fresh3 = *DecompPtr.offset(p as isize);
                        *fresh3 = (*fresh3 as libc::c_int
                            + PredictPaeth(
                                *PixelLeft.offset(p as isize),
                                *PixelUp.offset(p as isize),
                                *PixelUpLeft.offset(p as isize),
                            ) as libc::c_int)
                            as crate::stdlib::uint8_t
                    }
                    _ => return crate::src::qcommon::q_shared::qfalse,
                }
                p = p.wrapping_add(1)
            }
            PixelLeft = DecompPtr;
            /*
             *  We only have an upleft pixel if we are on the second line or above.
             */
            if h > 0 as libc::c_int as libc::c_uint {
                PixelUpLeft = DecompPtr.offset(
                    -(BytesPerScanline.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
                )
            }
            /*
             *  Skip to the next pixel.
             */
            DecompPtr = DecompPtr.offset(BytesPerPixel as isize);
            /*
             *  We only have a previous line if we are on the second line and above.
             */
            if h > 0 as libc::c_int as libc::c_uint {
                PixelUp = DecompPtr.offset(
                    -(BytesPerScanline.wrapping_add(1 as libc::c_int as libc::c_uint) as isize),
                )
            }
            w = w.wrapping_add(1)
        }
        h = h.wrapping_add(1)
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
 *  Convert a raw input pixel to Quake 3 RGA format.
 */

unsafe extern "C" fn ConvertPixel(
    mut IHDR: *mut PNG_Chunk_IHDR,
    mut OutPtr: *mut crate::src::qcommon::q_shared::byte,
    mut DecompPtr: *mut crate::stdlib::uint8_t,
    mut HasTransparentColour: crate::src::qcommon::q_shared::qboolean,
    mut TransparentColour: *mut crate::stdlib::uint8_t,
    mut OutPal: *mut crate::stdlib::uint8_t,
) -> crate::src::qcommon::q_shared::qboolean {
    /*
     *  input verification
     */
    if !(!IHDR.is_null()
        && !OutPtr.is_null()
        && !DecompPtr.is_null()
        && !TransparentColour.is_null()
        && !OutPal.is_null())
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    match (*IHDR).ColourType as libc::c_int {
        0 => {
            match (*IHDR).BitDepth as libc::c_int {
                1 | 2 | 4 => {
                    let mut Step: crate::stdlib::uint8_t = 0;
                    let mut GreyValue: crate::stdlib::uint8_t = 0;
                    Step = (0xff as libc::c_int
                        / (((1 as libc::c_int) << (*IHDR).BitDepth as libc::c_int)
                            - 1 as libc::c_int))
                        as crate::stdlib::uint8_t;
                    GreyValue = (*DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                        * Step as libc::c_int)
                        as crate::stdlib::uint8_t;
                    *OutPtr.offset(0 as libc::c_int as isize) = GreyValue;
                    *OutPtr.offset(1 as libc::c_int as isize) = GreyValue;
                    *OutPtr.offset(2 as libc::c_int as isize) = GreyValue;
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        0xff as libc::c_int as crate::src::qcommon::q_shared::byte;
                    /*
                     *  Grey supports full transparency for one specified colour
                     */
                    if HasTransparentColour as u64 != 0 {
                        if *TransparentColour.offset(1 as libc::c_int as isize) as libc::c_int
                            == *DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                        {
                            *OutPtr.offset(3 as libc::c_int as isize) =
                                0 as libc::c_int as crate::src::qcommon::q_shared::byte
                        }
                    }
                }
                8 | 16 => {
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        0xff as libc::c_int as crate::src::qcommon::q_shared::byte;
                    /*
                     *  Grey supports full transparency for one specified colour
                     */
                    if HasTransparentColour as u64 != 0 {
                        if (*IHDR).BitDepth as libc::c_int == 8 as libc::c_int {
                            if *TransparentColour.offset(1 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                            {
                                *OutPtr.offset(3 as libc::c_int as isize) =
                                    0 as libc::c_int as crate::src::qcommon::q_shared::byte
                            }
                        } else if *TransparentColour.offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == *DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(1 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(1 as libc::c_int as isize) as libc::c_int
                        {
                            *OutPtr.offset(3 as libc::c_int as isize) =
                                0 as libc::c_int as crate::src::qcommon::q_shared::byte
                        }
                    }
                }
                _ => return crate::src::qcommon::q_shared::qfalse,
            }
        }
        2 => {
            match (*IHDR).BitDepth as libc::c_int {
                8 => {
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(1 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(2 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        0xff as libc::c_int as crate::src::qcommon::q_shared::byte;
                    /*
                     *  True supports full transparency for one specified colour
                     */
                    if HasTransparentColour as u64 != 0 {
                        if *TransparentColour.offset(1 as libc::c_int as isize) as libc::c_int
                            == *DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(3 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(1 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(5 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(2 as libc::c_int as isize) as libc::c_int
                        {
                            *OutPtr.offset(3 as libc::c_int as isize) =
                                0 as libc::c_int as crate::src::qcommon::q_shared::byte
                        }
                    }
                }
                16 => {
                    /*
                     *  We use only the upper byte.
                     */
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(2 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(4 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        0xff as libc::c_int as crate::src::qcommon::q_shared::byte;
                    /*
                     *  True supports full transparency for one specified colour
                     */
                    if HasTransparentColour as u64 != 0 {
                        if *TransparentColour.offset(0 as libc::c_int as isize) as libc::c_int
                            == *DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(1 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(1 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(2 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(2 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(3 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(3 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(4 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(4 as libc::c_int as isize) as libc::c_int
                            && *TransparentColour.offset(5 as libc::c_int as isize) as libc::c_int
                                == *DecompPtr.offset(5 as libc::c_int as isize) as libc::c_int
                        {
                            *OutPtr.offset(3 as libc::c_int as isize) =
                                0 as libc::c_int as crate::src::qcommon::q_shared::byte
                        }
                    }
                }
                _ => return crate::src::qcommon::q_shared::qfalse,
            }
        }
        3 => {
            *OutPtr.offset(0 as libc::c_int as isize) = *OutPal.offset(
                (*DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int
                    + 0 as libc::c_int) as isize,
            );
            *OutPtr.offset(1 as libc::c_int as isize) = *OutPal.offset(
                (*DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int
                    + 1 as libc::c_int) as isize,
            );
            *OutPtr.offset(2 as libc::c_int as isize) = *OutPal.offset(
                (*DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int
                    + 2 as libc::c_int) as isize,
            );
            *OutPtr.offset(3 as libc::c_int as isize) = *OutPal.offset(
                (*DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int
                    + 3 as libc::c_int) as isize,
            )
        }
        4 => {
            match (*IHDR).BitDepth as libc::c_int {
                8 => {
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        *DecompPtr.offset(1 as libc::c_int as isize)
                }
                16 => {
                    /*
                     *  We use only the upper byte.
                     */
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        *DecompPtr.offset(2 as libc::c_int as isize)
                }
                _ => return crate::src::qcommon::q_shared::qfalse,
            }
        }
        6 => {
            match (*IHDR).BitDepth as libc::c_int {
                8 => {
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(1 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(2 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        *DecompPtr.offset(3 as libc::c_int as isize)
                }
                16 => {
                    /*
                     *  We use only the upper byte.
                     */
                    *OutPtr.offset(0 as libc::c_int as isize) =
                        *DecompPtr.offset(0 as libc::c_int as isize);
                    *OutPtr.offset(1 as libc::c_int as isize) =
                        *DecompPtr.offset(2 as libc::c_int as isize);
                    *OutPtr.offset(2 as libc::c_int as isize) =
                        *DecompPtr.offset(4 as libc::c_int as isize);
                    *OutPtr.offset(3 as libc::c_int as isize) =
                        *DecompPtr.offset(6 as libc::c_int as isize)
                }
                _ => return crate::src::qcommon::q_shared::qfalse,
            }
        }
        _ => return crate::src::qcommon::q_shared::qfalse,
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
 *  Decode a non-interlaced image.
 */

unsafe extern "C" fn DecodeImageNonInterlaced(
    mut IHDR: *mut PNG_Chunk_IHDR,
    mut OutBuffer: *mut crate::src::qcommon::q_shared::byte,
    mut DecompressedData: *mut crate::stdlib::uint8_t,
    mut DecompressedDataLength: crate::stdlib::uint32_t,
    mut HasTransparentColour: crate::src::qcommon::q_shared::qboolean,
    mut TransparentColour: *mut crate::stdlib::uint8_t,
    mut OutPal: *mut crate::stdlib::uint8_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut IHDR_Width: crate::stdlib::uint32_t = 0;
    let mut IHDR_Height: crate::stdlib::uint32_t = 0;
    let mut BytesPerScanline: crate::stdlib::uint32_t = 0;
    let mut BytesPerPixel: crate::stdlib::uint32_t = 0;
    let mut PixelsPerByte: crate::stdlib::uint32_t = 0;
    let mut w: crate::stdlib::uint32_t = 0;
    let mut h: crate::stdlib::uint32_t = 0;
    let mut p: crate::stdlib::uint32_t = 0;
    let mut OutPtr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut DecompPtr: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    /*
     *  input verification
     */
    if !(!IHDR.is_null()
        && !OutBuffer.is_null()
        && !DecompressedData.is_null()
        && DecompressedDataLength != 0
        && !TransparentColour.is_null()
        && !OutPal.is_null())
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  byte swapping
     */
    IHDR_Width = crate::src::qcommon::q_shared::LongSwap((*IHDR).Width as libc::c_int)
        as crate::stdlib::uint32_t;
    IHDR_Height = crate::src::qcommon::q_shared::LongSwap((*IHDR).Height as libc::c_int)
        as crate::stdlib::uint32_t;
    /*
     *  information for un-filtering
     */
    match (*IHDR).ColourType as libc::c_int {
        0 => match (*IHDR).BitDepth as libc::c_int {
            1 | 2 | 4 => {
                BytesPerPixel = 1 as libc::c_int as crate::stdlib::uint32_t;
                PixelsPerByte =
                    (8 as libc::c_int / (*IHDR).BitDepth as libc::c_int) as crate::stdlib::uint32_t
            }
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 1 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        2 => match (*IHDR).BitDepth as libc::c_int {
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 3 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        3 => match (*IHDR).BitDepth as libc::c_int {
            1 | 2 | 4 => {
                BytesPerPixel = 1 as libc::c_int as crate::stdlib::uint32_t;
                PixelsPerByte =
                    (8 as libc::c_int / (*IHDR).BitDepth as libc::c_int) as crate::stdlib::uint32_t
            }
            8 => {
                BytesPerPixel = 1 as libc::c_int as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        4 => match (*IHDR).BitDepth as libc::c_int {
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 2 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        6 => match (*IHDR).BitDepth as libc::c_int {
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 4 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        _ => return crate::src::qcommon::q_shared::qfalse,
    }
    /*
     *  Calculate the size of one scanline
     */
    BytesPerScanline = IHDR_Width
        .wrapping_mul(BytesPerPixel)
        .wrapping_add(PixelsPerByte.wrapping_sub(1 as libc::c_int as libc::c_uint))
        .wrapping_div(PixelsPerByte);
    /*
     *  Check if we have enough data for the whole image.
     */
    if !(DecompressedDataLength
        == BytesPerScanline
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(IHDR_Height))
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  Unfilter the image.
     */
    if UnfilterImage(
        DecompressedData,
        IHDR_Height,
        BytesPerScanline,
        BytesPerPixel,
    ) as u64
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  Set the working pointers to the beginning of the buffers.
     */
    OutPtr = OutBuffer;
    DecompPtr = DecompressedData;
    /*
     *  Create the output image.
     */
    h = 0 as libc::c_int as crate::stdlib::uint32_t;
    while h < IHDR_Height {
        /*
         *  Count the pixels on the scanline for those multipixel bytes
         */
        let mut CurrPixel: crate::stdlib::uint32_t = 0;
        /*
         *  skip FilterType
         */
        DecompPtr = DecompPtr.offset(1);
        /*
         *  Reset the pixel count.
         */
        CurrPixel = 0 as libc::c_int as crate::stdlib::uint32_t;
        w = 0 as libc::c_int as crate::stdlib::uint32_t;
        while w < BytesPerScanline.wrapping_div(BytesPerPixel) {
            if PixelsPerByte > 1 as libc::c_int as libc::c_uint {
                let mut Mask: crate::stdlib::uint8_t = 0;
                let mut Shift: crate::stdlib::uint32_t = 0;
                let mut SinglePixel: crate::stdlib::uint8_t = 0;
                p = 0 as libc::c_int as crate::stdlib::uint32_t;
                while p < PixelsPerByte {
                    if CurrPixel < IHDR_Width {
                        Mask = (((1 as libc::c_int) << (*IHDR).BitDepth as libc::c_int)
                            - 1 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        Shift = PixelsPerByte
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            .wrapping_sub(p)
                            .wrapping_mul((*IHDR).BitDepth as libc::c_uint);
                        SinglePixel =
                            ((*DecompPtr.offset(0 as libc::c_int as isize) as libc::c_int
                                & (Mask as libc::c_int) << Shift)
                                >> Shift) as crate::stdlib::uint8_t;
                        if ConvertPixel(
                            IHDR,
                            OutPtr,
                            &mut SinglePixel,
                            HasTransparentColour,
                            TransparentColour,
                            OutPal,
                        ) as u64
                            == 0
                        {
                            return crate::src::qcommon::q_shared::qfalse;
                        }
                        OutPtr = OutPtr.offset(4 as libc::c_int as isize);
                        CurrPixel = CurrPixel.wrapping_add(1)
                    }
                    p = p.wrapping_add(1)
                }
            } else {
                if ConvertPixel(
                    IHDR,
                    OutPtr,
                    DecompPtr,
                    HasTransparentColour,
                    TransparentColour,
                    OutPal,
                ) as u64
                    == 0
                {
                    return crate::src::qcommon::q_shared::qfalse;
                }
                OutPtr = OutPtr.offset(4 as libc::c_int as isize)
            }
            DecompPtr = DecompPtr.offset(BytesPerPixel as isize);
            w = w.wrapping_add(1)
        }
        h = h.wrapping_add(1)
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
 *  Decode an interlaced image.
 */

unsafe extern "C" fn DecodeImageInterlaced(
    mut IHDR: *mut PNG_Chunk_IHDR,
    mut OutBuffer: *mut crate::src::qcommon::q_shared::byte,
    mut DecompressedData: *mut crate::stdlib::uint8_t,
    mut DecompressedDataLength: crate::stdlib::uint32_t,
    mut HasTransparentColour: crate::src::qcommon::q_shared::qboolean,
    mut TransparentColour: *mut crate::stdlib::uint8_t,
    mut OutPal: *mut crate::stdlib::uint8_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut IHDR_Width: crate::stdlib::uint32_t = 0;
    let mut IHDR_Height: crate::stdlib::uint32_t = 0;
    let mut BytesPerScanline: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut BytesPerPixel: crate::stdlib::uint32_t = 0;
    let mut PixelsPerByte: crate::stdlib::uint32_t = 0;
    let mut PassWidth: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut PassHeight: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut WSkip: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut WOffset: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut HSkip: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut HOffset: [crate::stdlib::uint32_t; 7] = [0; 7];
    let mut w: crate::stdlib::uint32_t = 0;
    let mut h: crate::stdlib::uint32_t = 0;
    let mut p: crate::stdlib::uint32_t = 0;
    let mut a: crate::stdlib::uint32_t = 0;
    let mut OutPtr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut DecompPtr: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut TargetLength: crate::stdlib::uint32_t = 0;
    /*
     *  input verification
     */
    if !(!IHDR.is_null()
        && !OutBuffer.is_null()
        && !DecompressedData.is_null()
        && DecompressedDataLength != 0
        && !TransparentColour.is_null()
        && !OutPal.is_null())
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  byte swapping
     */
    IHDR_Width = crate::src::qcommon::q_shared::LongSwap((*IHDR).Width as libc::c_int)
        as crate::stdlib::uint32_t;
    IHDR_Height = crate::src::qcommon::q_shared::LongSwap((*IHDR).Height as libc::c_int)
        as crate::stdlib::uint32_t;
    /*
     *  Skip and Offset for the passes.
     */
    WSkip[0 as libc::c_int as usize] = 8 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[0 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[0 as libc::c_int as usize] = 8 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[0 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    WSkip[1 as libc::c_int as usize] = 8 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[1 as libc::c_int as usize] = 4 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[1 as libc::c_int as usize] = 8 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[1 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    WSkip[2 as libc::c_int as usize] = 4 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[2 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[2 as libc::c_int as usize] = 8 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[2 as libc::c_int as usize] = 4 as libc::c_int as crate::stdlib::uint32_t;
    WSkip[3 as libc::c_int as usize] = 4 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[3 as libc::c_int as usize] = 2 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[3 as libc::c_int as usize] = 4 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[3 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    WSkip[4 as libc::c_int as usize] = 2 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[4 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[4 as libc::c_int as usize] = 4 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[4 as libc::c_int as usize] = 2 as libc::c_int as crate::stdlib::uint32_t;
    WSkip[5 as libc::c_int as usize] = 2 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[5 as libc::c_int as usize] = 1 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[5 as libc::c_int as usize] = 2 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[5 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    WSkip[6 as libc::c_int as usize] = 1 as libc::c_int as crate::stdlib::uint32_t;
    WOffset[6 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    HSkip[6 as libc::c_int as usize] = 2 as libc::c_int as crate::stdlib::uint32_t;
    HOffset[6 as libc::c_int as usize] = 1 as libc::c_int as crate::stdlib::uint32_t;
    /*
     *  Calculate the sizes of the passes.
     */
    PassWidth[0 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    PassHeight[0 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    PassWidth[1 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(3 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    PassHeight[1 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(7 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    PassWidth[2 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(3 as libc::c_int as libc::c_uint)
        .wrapping_div(4 as libc::c_int as libc::c_uint);
    PassHeight[2 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(3 as libc::c_int as libc::c_uint)
        .wrapping_div(8 as libc::c_int as libc::c_uint);
    PassWidth[3 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(4 as libc::c_int as libc::c_uint);
    PassHeight[3 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(3 as libc::c_int as libc::c_uint)
        .wrapping_div(4 as libc::c_int as libc::c_uint);
    PassWidth[4 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    PassHeight[4 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(4 as libc::c_int as libc::c_uint);
    PassWidth[5 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    PassHeight[5 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    PassWidth[6 as libc::c_int as usize] = IHDR_Width
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_div(1 as libc::c_int as libc::c_uint);
    PassHeight[6 as libc::c_int as usize] = IHDR_Height
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    /*
     *  information for un-filtering
     */
    match (*IHDR).ColourType as libc::c_int {
        0 => match (*IHDR).BitDepth as libc::c_int {
            1 | 2 | 4 => {
                BytesPerPixel = 1 as libc::c_int as crate::stdlib::uint32_t;
                PixelsPerByte =
                    (8 as libc::c_int / (*IHDR).BitDepth as libc::c_int) as crate::stdlib::uint32_t
            }
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 1 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        2 => match (*IHDR).BitDepth as libc::c_int {
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 3 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        3 => match (*IHDR).BitDepth as libc::c_int {
            1 | 2 | 4 => {
                BytesPerPixel = 1 as libc::c_int as crate::stdlib::uint32_t;
                PixelsPerByte =
                    (8 as libc::c_int / (*IHDR).BitDepth as libc::c_int) as crate::stdlib::uint32_t
            }
            8 => {
                BytesPerPixel = 1 as libc::c_int as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        4 => match (*IHDR).BitDepth as libc::c_int {
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 2 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        6 => match (*IHDR).BitDepth as libc::c_int {
            8 | 16 => {
                BytesPerPixel = ((*IHDR).BitDepth as libc::c_int / 8 as libc::c_int
                    * 4 as libc::c_int) as crate::stdlib::uint32_t;
                PixelsPerByte = 1 as libc::c_int as crate::stdlib::uint32_t
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        },
        _ => return crate::src::qcommon::q_shared::qfalse,
    }
    /*
     *  Calculate the size of the scanlines per pass
     */
    a = 0 as libc::c_int as crate::stdlib::uint32_t;
    while a < 7 as libc::c_int as libc::c_uint {
        BytesPerScanline[a as usize] = PassWidth[a as usize]
            .wrapping_mul(BytesPerPixel)
            .wrapping_add(PixelsPerByte.wrapping_sub(1 as libc::c_int as libc::c_uint))
            .wrapping_div(PixelsPerByte);
        a = a.wrapping_add(1)
    }
    /*
     *  Calculate the size of all passes
     */
    TargetLength = 0 as libc::c_int as crate::stdlib::uint32_t;
    a = 0 as libc::c_int as crate::stdlib::uint32_t;
    while a < 7 as libc::c_int as libc::c_uint {
        TargetLength = (TargetLength as libc::c_uint).wrapping_add(
            BytesPerScanline[a as usize]
                .wrapping_add(
                    (if BytesPerScanline[a as usize] != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint,
                )
                .wrapping_mul(PassHeight[a as usize]),
        ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
        a = a.wrapping_add(1)
    }
    /*
     *  Check if we have enough data for the whole image.
     */
    if !(DecompressedDataLength == TargetLength) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    /*
     *  Unfilter the image.
     */
    DecompPtr = DecompressedData;
    a = 0 as libc::c_int as crate::stdlib::uint32_t;
    while a < 7 as libc::c_int as libc::c_uint {
        if UnfilterImage(
            DecompPtr,
            PassHeight[a as usize],
            BytesPerScanline[a as usize],
            BytesPerPixel,
        ) as u64
            == 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        DecompPtr = DecompPtr.offset(
            BytesPerScanline[a as usize]
                .wrapping_add(
                    (if BytesPerScanline[a as usize] != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint,
                )
                .wrapping_mul(PassHeight[a as usize]) as isize,
        );
        a = a.wrapping_add(1)
    }
    /*
     *  Set the working pointers to the beginning of the buffers.
     */
    DecompPtr = DecompressedData;
    /*
     *  Create the output image.
     */
    a = 0 as libc::c_int as crate::stdlib::uint32_t;
    while a < 7 as libc::c_int as libc::c_uint {
        h = 0 as libc::c_int as crate::stdlib::uint32_t;
        while h < PassHeight[a as usize] {
            /*
             *  Count the pixels on the scanline for those multipixel bytes
             */
            let mut CurrPixel: crate::stdlib::uint32_t = 0;
            /*
             *  skip FilterType
             *  but only when the pass has a width bigger than zero
             */
            if BytesPerScanline[a as usize] != 0 {
                DecompPtr = DecompPtr.offset(1)
            }
            /*
             *  Reset the pixel count.
             */
            CurrPixel = 0 as libc::c_int as crate::stdlib::uint32_t;
            w = 0 as libc::c_int as crate::stdlib::uint32_t;
            while w < BytesPerScanline[a as usize].wrapping_div(BytesPerPixel) {
                if PixelsPerByte > 1 as libc::c_int as libc::c_uint {
                    let mut Mask: crate::stdlib::uint8_t = 0;
                    let mut Shift: crate::stdlib::uint32_t = 0;
                    let mut SinglePixel: crate::stdlib::uint8_t = 0;
                    p = 0 as libc::c_int as crate::stdlib::uint32_t;
                    while p < PixelsPerByte {
                        if CurrPixel < PassWidth[a as usize] {
                            Mask = (((1 as libc::c_int) << (*IHDR).BitDepth as libc::c_int)
                                - 1 as libc::c_int)
                                as crate::stdlib::uint8_t;
                            Shift = PixelsPerByte
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                .wrapping_sub(p)
                                .wrapping_mul((*IHDR).BitDepth as libc::c_uint);
                            SinglePixel = ((*DecompPtr.offset(0 as libc::c_int as isize)
                                as libc::c_int
                                & (Mask as libc::c_int) << Shift)
                                >> Shift)
                                as crate::stdlib::uint8_t;
                            OutPtr = OutBuffer.offset(
                                h.wrapping_mul(HSkip[a as usize])
                                    .wrapping_add(HOffset[a as usize])
                                    .wrapping_mul(IHDR_Width)
                                    .wrapping_add(
                                        CurrPixel
                                            .wrapping_mul(WSkip[a as usize])
                                            .wrapping_add(WOffset[a as usize]),
                                    )
                                    .wrapping_mul(4 as libc::c_int as libc::c_uint)
                                    as isize,
                            );
                            if ConvertPixel(
                                IHDR,
                                OutPtr,
                                &mut SinglePixel,
                                HasTransparentColour,
                                TransparentColour,
                                OutPal,
                            ) as u64
                                == 0
                            {
                                return crate::src::qcommon::q_shared::qfalse;
                            }
                            CurrPixel = CurrPixel.wrapping_add(1)
                        }
                        p = p.wrapping_add(1)
                    }
                } else {
                    OutPtr = OutBuffer.offset(
                        h.wrapping_mul(HSkip[a as usize])
                            .wrapping_add(HOffset[a as usize])
                            .wrapping_mul(IHDR_Width)
                            .wrapping_add(
                                w.wrapping_mul(WSkip[a as usize])
                                    .wrapping_add(WOffset[a as usize]),
                            )
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            as isize,
                    );
                    if ConvertPixel(
                        IHDR,
                        OutPtr,
                        DecompPtr,
                        HasTransparentColour,
                        TransparentColour,
                        OutPal,
                    ) as u64
                        == 0
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                }
                DecompPtr = DecompPtr.offset(BytesPerPixel as isize);
                w = w.wrapping_add(1)
            }
            h = h.wrapping_add(1)
        }
        a = a.wrapping_add(1)
    }
    return crate::src::qcommon::q_shared::qtrue;
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
/*
 *  The PNG loader
 */
#[no_mangle]

pub unsafe extern "C" fn R_LoadPNG(
    mut name: *const libc::c_char,
    mut pic: *mut *mut crate::src::qcommon::q_shared::byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut ThePNG: *mut BufferedFile = 0 as *mut BufferedFile;
    let mut OutBuffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut Signature: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut CH: *mut PNG_ChunkHeader = 0 as *mut PNG_ChunkHeader;
    let mut ChunkHeaderLength: crate::stdlib::uint32_t = 0;
    let mut ChunkHeaderType: crate::stdlib::uint32_t = 0;
    let mut IHDR: *mut PNG_Chunk_IHDR = 0 as *mut PNG_Chunk_IHDR;
    let mut IHDR_Width: crate::stdlib::uint32_t = 0;
    let mut IHDR_Height: crate::stdlib::uint32_t = 0;
    let mut CRC: *mut PNG_ChunkCRC = 0 as *mut PNG_ChunkCRC;
    let mut InPal: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut DecompressedData: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
    let mut DecompressedDataLength: crate::stdlib::uint32_t = 0;
    let mut i: crate::stdlib::uint32_t = 0;
    /*
     *  palette with 256 RGBA entries
     */
    let mut OutPal: [crate::stdlib::uint8_t; 1024] = [0; 1024];
    /*
     *  transparent colour from the tRNS chunk
     */
    let mut HasTransparentColour: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut TransparentColour: [crate::stdlib::uint8_t; 6] = [
        0xff as libc::c_int as crate::stdlib::uint8_t,
        0xff as libc::c_int as crate::stdlib::uint8_t,
        0xff as libc::c_int as crate::stdlib::uint8_t,
        0xff as libc::c_int as crate::stdlib::uint8_t,
        0xff as libc::c_int as crate::stdlib::uint8_t,
        0xff as libc::c_int as crate::stdlib::uint8_t,
    ];
    /*
     *  input verification
     */
    if !(!name.is_null() && !pic.is_null()) {
        return;
    }
    /*
     *  Zero out return values.
     */
    *pic = 0 as *mut crate::src::qcommon::q_shared::byte;
    if !width.is_null() {
        *width = 0 as libc::c_int
    }
    if !height.is_null() {
        *height = 0 as libc::c_int
    }
    /*
     *  Read the file.
     */
    ThePNG = ReadBufferedFile(name);
    if ThePNG.is_null() {
        return;
    }
    /*
     *  Read the siganture of the file.
     */
    Signature =
        BufferedFileRead(ThePNG, 8 as libc::c_int as libc::c_uint) as *mut crate::stdlib::uint8_t;
    if Signature.is_null() {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Is it a PNG?
     */
    if crate::stdlib::memcmp(
        Signature as *const libc::c_void,
        b"\x89PNG\r\n\x1a\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Read the first chunk-header.
     */
    CH = BufferedFileRead(ThePNG, 8 as libc::c_int as libc::c_uint) as *mut PNG_ChunkHeader;
    if CH.is_null() {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  PNG multi-byte types are in Big Endian
     */
    ChunkHeaderLength = crate::src::qcommon::q_shared::LongSwap((*CH).Length as libc::c_int)
        as crate::stdlib::uint32_t;
    ChunkHeaderType = crate::src::qcommon::q_shared::LongSwap((*CH).Type as libc::c_int)
        as crate::stdlib::uint32_t;
    /*
     *  Check if the first chunk is an IHDR.
     */
    if !(ChunkHeaderType
        == (('I' as i32) << 24 as libc::c_int
            | ('H' as i32) << 16 as libc::c_int
            | ('D' as i32) << 8 as libc::c_int
            | 'R' as i32) as libc::c_uint
        && ChunkHeaderLength == 13 as libc::c_int as libc::c_uint)
    {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Read the IHDR.
     */
    IHDR = BufferedFileRead(ThePNG, 13 as libc::c_int as libc::c_uint) as *mut PNG_Chunk_IHDR;
    if IHDR.is_null() {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Read the CRC for IHDR
     */
    CRC = BufferedFileRead(ThePNG, 4 as libc::c_int as libc::c_uint) as *mut PNG_ChunkCRC;
    if CRC.is_null() {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Here we could check the CRC if we wanted to.
     */
    /*
     *  multi-byte type swapping
     */
    IHDR_Width = crate::src::qcommon::q_shared::LongSwap((*IHDR).Width as libc::c_int)
        as crate::stdlib::uint32_t;
    IHDR_Height = crate::src::qcommon::q_shared::LongSwap((*IHDR).Height as libc::c_int)
        as crate::stdlib::uint32_t;
    /*
     *  Check if Width and Height are valid.
     */
    if !(IHDR_Width > 0 as libc::c_int as libc::c_uint
        && IHDR_Height > 0 as libc::c_int as libc::c_uint)
        || IHDR_Width
            > ((2147483647 as libc::c_int / 4 as libc::c_int) as libc::c_uint)
                .wrapping_div(IHDR_Height)
    {
        CloseBufferedFile(ThePNG);
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as libc::c_int,
            b"%s: invalid image size\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        return;
    }
    /*
     *  Do we need to check if the dimensions of the image are valid for Quake3?
     */
    /*
     *  Check if CompressionMethod and FilterMethod are valid.
     */
    if !((*IHDR).CompressionMethod as libc::c_int == 0 as libc::c_int
        && (*IHDR).FilterMethod as libc::c_int == 0 as libc::c_int)
    {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Check if InterlaceMethod is valid.
     */
    if !((*IHDR).InterlaceMethod as libc::c_int == 0 as libc::c_int
        || (*IHDR).InterlaceMethod as libc::c_int == 1 as libc::c_int)
    {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Read palette for an indexed image.
     */
    if (*IHDR).ColourType as libc::c_int == 3 as libc::c_int {
        /*
         *  We need the palette first.
         */
        if FindChunk(
            ThePNG,
            (('P' as i32) << 24 as libc::c_int
                | ('L' as i32) << 16 as libc::c_int
                | ('T' as i32) << 8 as libc::c_int
                | 'E' as i32) as crate::stdlib::uint32_t,
        ) as u64
            == 0
        {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Read the chunk-header.
         */
        CH = BufferedFileRead(ThePNG, 8 as libc::c_int as libc::c_uint) as *mut PNG_ChunkHeader;
        if CH.is_null() {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  PNG multi-byte types are in Big Endian
         */
        ChunkHeaderLength = crate::src::qcommon::q_shared::LongSwap((*CH).Length as libc::c_int)
            as crate::stdlib::uint32_t;
        ChunkHeaderType = crate::src::qcommon::q_shared::LongSwap((*CH).Type as libc::c_int)
            as crate::stdlib::uint32_t;
        /*
         *  Check if the chunk is a PLTE.
         */
        if !(ChunkHeaderType
            == (('P' as i32) << 24 as libc::c_int
                | ('L' as i32) << 16 as libc::c_int
                | ('T' as i32) << 8 as libc::c_int
                | 'E' as i32) as libc::c_uint)
        {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Check if Length is divisible by 3
         */
        if ChunkHeaderLength.wrapping_rem(3 as libc::c_int as libc::c_uint) != 0 {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Read the raw palette data
         */
        InPal = BufferedFileRead(ThePNG, ChunkHeaderLength) as *mut crate::stdlib::uint8_t;
        if InPal.is_null() {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Read the CRC for the palette
         */
        CRC = BufferedFileRead(ThePNG, 4 as libc::c_int as libc::c_uint) as *mut PNG_ChunkCRC;
        if CRC.is_null() {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Set some default values.
         */
        i = 0 as libc::c_int as crate::stdlib::uint32_t;
        while i < 256 as libc::c_int as libc::c_uint {
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
                0 as libc::c_int as crate::stdlib::uint8_t;
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
                0 as libc::c_int as crate::stdlib::uint8_t;
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
                0 as libc::c_int as crate::stdlib::uint8_t;
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
                0xff as libc::c_int as crate::stdlib::uint8_t;
            i = i.wrapping_add(1)
        }
        /*
         *  Convert to the Quake3 RGBA-format.
         */
        i = 0 as libc::c_int as crate::stdlib::uint32_t;
        while i < ChunkHeaderLength.wrapping_div(3 as libc::c_int as libc::c_uint) {
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as usize] = *InPal.offset(
                i.wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
            );
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as usize] = *InPal.offset(
                i.wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            );
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as usize] = *InPal.offset(
                i.wrapping_mul(3 as libc::c_int as libc::c_uint)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            );
            OutPal[i
                .wrapping_mul(4 as libc::c_int as libc::c_uint)
                .wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
                0xff as libc::c_int as crate::stdlib::uint8_t;
            i = i.wrapping_add(1)
        }
    }
    /*
     *  transparency information is sometimes stored in a tRNS chunk
     */
    /*
     *  Let's see if there is a tRNS chunk
     */
    if FindChunk(
        ThePNG,
        (('t' as i32) << 24 as libc::c_int
            | ('R' as i32) << 16 as libc::c_int
            | ('N' as i32) << 8 as libc::c_int
            | 'S' as i32) as crate::stdlib::uint32_t,
    ) as u64
        != 0
    {
        let mut Trans: *mut crate::stdlib::uint8_t = 0 as *mut crate::stdlib::uint8_t;
        /*
         *  Read the chunk-header.
         */
        CH = BufferedFileRead(ThePNG, 8 as libc::c_int as libc::c_uint) as *mut PNG_ChunkHeader;
        if CH.is_null() {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  PNG multi-byte types are in Big Endian
         */
        ChunkHeaderLength = crate::src::qcommon::q_shared::LongSwap((*CH).Length as libc::c_int)
            as crate::stdlib::uint32_t;
        ChunkHeaderType = crate::src::qcommon::q_shared::LongSwap((*CH).Type as libc::c_int)
            as crate::stdlib::uint32_t;
        /*
         *  Check if the chunk is a tRNS.
         */
        if !(ChunkHeaderType
            == (('t' as i32) << 24 as libc::c_int
                | ('R' as i32) << 16 as libc::c_int
                | ('N' as i32) << 8 as libc::c_int
                | 'S' as i32) as libc::c_uint)
        {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Read the transparency information.
         */
        Trans = BufferedFileRead(ThePNG, ChunkHeaderLength) as *mut crate::stdlib::uint8_t;
        if Trans.is_null() {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Read the CRC.
         */
        CRC = BufferedFileRead(ThePNG, 4 as libc::c_int as libc::c_uint) as *mut PNG_ChunkCRC;
        if CRC.is_null() {
            CloseBufferedFile(ThePNG);
            return;
        }
        /*
         *  Only for Grey, True and Indexed ColourType should tRNS exist.
         */
        match (*IHDR).ColourType as libc::c_int {
            0 => {
                if ChunkHeaderLength != 2 as libc::c_int as libc::c_uint {
                    CloseBufferedFile(ThePNG);
                    return;
                }
                HasTransparentColour = crate::src::qcommon::q_shared::qtrue;
                /*
                 *  Grey can have one colour which is completely transparent.
                 *  This colour is always stored in 16 bits.
                 */
                TransparentColour[0 as libc::c_int as usize] =
                    *Trans.offset(0 as libc::c_int as isize);
                TransparentColour[1 as libc::c_int as usize] =
                    *Trans.offset(1 as libc::c_int as isize)
            }
            2 => {
                if ChunkHeaderLength != 6 as libc::c_int as libc::c_uint {
                    CloseBufferedFile(ThePNG);
                    return;
                }
                HasTransparentColour = crate::src::qcommon::q_shared::qtrue;
                /*
                 *  True can have one colour which is completely transparent.
                 *  This colour is always stored in 16 bits.
                 */
                TransparentColour[0 as libc::c_int as usize] =
                    *Trans.offset(0 as libc::c_int as isize);
                TransparentColour[1 as libc::c_int as usize] =
                    *Trans.offset(1 as libc::c_int as isize);
                TransparentColour[2 as libc::c_int as usize] =
                    *Trans.offset(2 as libc::c_int as isize);
                TransparentColour[3 as libc::c_int as usize] =
                    *Trans.offset(3 as libc::c_int as isize);
                TransparentColour[4 as libc::c_int as usize] =
                    *Trans.offset(4 as libc::c_int as isize);
                TransparentColour[5 as libc::c_int as usize] =
                    *Trans.offset(5 as libc::c_int as isize)
            }
            3 => {
                /*
                 *  Maximum of 256 one byte transparency entries.
                 */
                if ChunkHeaderLength > 256 as libc::c_int as libc::c_uint {
                    CloseBufferedFile(ThePNG);
                    return;
                }
                HasTransparentColour = crate::src::qcommon::q_shared::qtrue;
                /*
                 *  alpha values for palette entries
                 */
                i = 0 as libc::c_int as crate::stdlib::uint32_t;
                while i < ChunkHeaderLength {
                    OutPal[i
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(3 as libc::c_int as libc::c_uint)
                        as usize] = *Trans.offset(i as isize);
                    i = i.wrapping_add(1)
                }
            }
            _ => {
                /*
                 *  All other ColourTypes should not have tRNS chunks
                 */
                CloseBufferedFile(ThePNG);
                return;
            }
        }
    }
    /*
     *  Rewind to the start of the file.
     */
    if BufferedFileRewind(ThePNG, -(1 as libc::c_int) as libc::c_uint) as u64 == 0 {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Skip the signature
     */
    if BufferedFileSkip(ThePNG, 8 as libc::c_int as libc::c_uint) as u64 == 0 {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Decompress all IDAT chunks
     */
    DecompressedDataLength = DecompressIDATs(ThePNG, &mut DecompressedData);
    if !(DecompressedDataLength != 0 && !DecompressedData.is_null()) {
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Allocate output buffer.
     */
    OutBuffer = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(
        IHDR_Width
            .wrapping_mul(IHDR_Height)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::byte;
    if OutBuffer.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Free
            .expect("non-null function pointer")(DecompressedData as *mut libc::c_void);
        CloseBufferedFile(ThePNG);
        return;
    }
    /*
     *  Interlaced and Non-interlaced images need to be handled differently.
     */
    match (*IHDR).InterlaceMethod as libc::c_int {
        0 => {
            if DecodeImageNonInterlaced(
                IHDR,
                OutBuffer,
                DecompressedData,
                DecompressedDataLength,
                HasTransparentColour,
                TransparentColour.as_mut_ptr(),
                OutPal.as_mut_ptr(),
            ) as u64
                == 0
            {
                crate::src::renderergl1::tr_main::ri
                    .Free
                    .expect("non-null function pointer")(
                    OutBuffer as *mut libc::c_void
                );
                crate::src::renderergl1::tr_main::ri
                    .Free
                    .expect("non-null function pointer")(
                    DecompressedData as *mut libc::c_void
                );
                CloseBufferedFile(ThePNG);
                return;
            }
        }
        1 => {
            if DecodeImageInterlaced(
                IHDR,
                OutBuffer,
                DecompressedData,
                DecompressedDataLength,
                HasTransparentColour,
                TransparentColour.as_mut_ptr(),
                OutPal.as_mut_ptr(),
            ) as u64
                == 0
            {
                crate::src::renderergl1::tr_main::ri
                    .Free
                    .expect("non-null function pointer")(
                    OutBuffer as *mut libc::c_void
                );
                crate::src::renderergl1::tr_main::ri
                    .Free
                    .expect("non-null function pointer")(
                    DecompressedData as *mut libc::c_void
                );
                CloseBufferedFile(ThePNG);
                return;
            }
        }
        _ => {
            crate::src::renderergl1::tr_main::ri
                .Free
                .expect("non-null function pointer")(OutBuffer as *mut libc::c_void);
            crate::src::renderergl1::tr_main::ri
                .Free
                .expect("non-null function pointer")(
                DecompressedData as *mut libc::c_void
            );
            CloseBufferedFile(ThePNG);
            return;
        }
    }
    /*
     *  update the pointer to the image data
     */
    *pic = OutBuffer;
    /*
     *  Fill width and height.
     */
    if !width.is_null() {
        *width = IHDR_Width as libc::c_int
    }
    if !height.is_null() {
        *height = IHDR_Height as libc::c_int
    }
    /*
     *  DecompressedData is not needed anymore.
     */
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")(DecompressedData as *mut libc::c_void);
    /*
     *  We have all data, so close the file.
     */
    CloseBufferedFile(ThePNG);
}
